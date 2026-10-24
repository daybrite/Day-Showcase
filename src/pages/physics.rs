//! Physics: marbles under adjustable gravity, driven by one native frame subscription, filling the
//! whole page.
//!
//! Tap or click the canvas and every marble leaps along a ballistic arc toward that point, each
//! turned a few degrees off the line so a crowd scatters around it, then gravity takes over. The
//! Gravity slider runs from zero-G (the default) to Jupiter, in real units (m/s²). The marbles
//! collide with the walls and with each other: no two overlap, a crowd thrown at one spot piles
//! up and spills, and under gravity they settle into a heap. Pairs are found on a uniform grid a
//! marble's diameter wide, so a thousand of them cost a thousand cell lookups a step, not half a
//! million pair tests; the fixed-step physics stays O(n).
//!
//! The Elasticity slider is one number for two things a real ball's material decides at once.
//! It is the coefficient of restitution, the fraction of approach speed that survives an impact:
//! at zero every collision is perfectly inelastic (a dropped marble lands with a thud and stays
//! down, two that meet move on together), at one it is perfectly elastic. And it is how far the
//! ball deforms on impact: a hard marble keeps its shape, a squishy one flattens along the line
//! of the hit and bulges across it, keeping its volume, then wobbles back to round like jelly.
//! Each marble is drawn as vector geometry (no bitmap cache), so the count exercises the native
//! canvas renderer; a deformed marble is four cubics instead of one ellipse.
//!
//! Full screen presents the same simulation as a cover over the whole window, and takes the
//! window itself full screen where the platform can.
use day::prelude::*;
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    ops::ControlFlow,
    rc::Rc,
};

const STEP: f64 = 1.0 / 120.0;
const RADIUS: f64 = 12.0;
const MAX_BALLS: usize = 1000;
const FPS_WINDOW: f64 = 2.0;

/// How many points on screen stand for a metre, the scale that turns gravity in m/s² into the
/// simulation's points per second squared. Chosen so Earth gravity feels as it always did here
/// (1100 pt/s²), a marble the size of a large one dropped from a tabletop.
const POINTS_PER_METRE: f64 = 1100.0 / EARTH;

/// Standard gravity at the surface of each body, in m/s², the marks the slider's readout names.
pub(crate) const EARTH: f64 = 9.80665;
const MOON: f64 = 1.62;
const MARS: f64 = 3.72;
pub(crate) const JUPITER: f64 = 24.79;

/// How a launch toward a tapped point chooses its flight time: proportional to the distance, so a
/// near point is a flick and a far one a proper throw, within bounds that keep both visible.
const FLIGHT_SECS_PER_POINT: f64 = 1.0 / 900.0;
const FLIGHT_MIN_SECS: f64 = 0.25;
const FLIGHT_MAX_SECS: f64 = 0.9;
/// Newton's constant, m³/(kg·s²). Mutual attraction uses the real one; what makes it visible at
/// the scale of a screen is the marbles' mass, which the Ball mass slider sets in megatonnes.
const NEWTON_G: f64 = 6.674e-11;
/// The heaviest a marble can be made, in megatonnes (10⁹ kg). How strong the pull is depends on
/// the count as much as on the mass (a thousand marbles weigh a thousand times one): at this
/// much a crowd bends visibly toward itself as it flies, without every throw ending in one
/// clump, and a handful of marbles barely notice each other.
pub(crate) const MAX_MASS_MT: f64 = 0.5;
/// No marble moves faster than this, points per second. Only the heaviest settings reach it:
/// a marble falling through a dense core there would otherwise cross the box in one step.
const MAX_SPEED: f64 = 3000.0;
/// The mutual-gravity mesh: its spacing is the canvas's longer side over this many cells, never
/// finer than a marble.
const MESH_CELLS: f64 = 20.0;
/// The simulation settles, and the frame clock stops, once every marble has stayed slower than
/// this (points per second) for [`CALM_STEPS`] steps in a row. A heap at rest is not perfectly
/// still: the sequential solver leaves a marble high in a stack a few points per second of
/// residual, so the threshold scales with gravity's per-step kick as well.
const CALM_SPEED: f64 = 5.0;
const CALM_STEPS: u32 = 60;

/// How far off the line to the tapped point a marble may be thrown, in radians either way (about
/// 7°): enough that a crowd keeps some entropy and scatters around the point, little enough that
/// every one of them plainly leaps toward it.
const AIM_JITTER: f64 = 0.12;
/// Air drag, as the fraction of its speed a marble loses per second. Too slight to notice under
/// gravity; at zero-G it is what lets a drifting marble ease to a stop instead of creeping across
/// the box for minutes between wall hits (keeping the frame clock awake all the while).
const DRAG_PER_SEC: f64 = 0.3;

/// The slider's starting point: a marble about as lively as a rubber ball, bouncing to a quarter
/// of its drop height and squashing visibly on a hard landing.
pub(crate) const DEFAULT_ELASTICITY: f64 = 0.5;
/// What fraction of the box's area the marbles may cover. The radius shrinks from [`RADIUS`] as
/// the count rises so that even a thousand marbles have room to move and to pile up: at this
/// density a heap under gravity fills about half the height.
const PACKING: f64 = 0.45;
/// Impacts slower than this (points per second) do not bounce whatever the elasticity. It is how
/// a resting contact stays at rest: gravity adds `g·STEP` to a marble on the floor every step,
/// and reflecting that would be a permanent one-point jitter. Twice the per-step kick, with a
/// floor for zero-G where there is no kick.
const REST_SPEED_MIN: f64 = 4.0;
/// Rolling resistance: the fraction of its sideways speed a marble keeps each step it spends
/// resting on the floor, which brings a heap to a stop instead of letting it creep.
const FLOOR_ROLL_KEEP: f64 = 0.9;
/// Sliding friction on a bounce: the fraction of tangential speed that survives a wall hit.
const WALL_FRICTION_KEEP: f64 = 0.85;
/// How many times a step the solver visits every contact. One pass resolves a pair; a stack is
/// a chain of pairs, and each extra pass carries the floor's support one marble higher.
const SOLVER_PASSES: usize = 4;
/// Overlap below this (points) is left alone, so touching marbles do not vibrate apart.
const SLOP: f64 = 0.05;
/// How much of an overlap one pass removes: less than all of it, so a crowd that spawns or is
/// thrown into itself eases apart over a few steps rather than exploding.
const SEPARATION: f64 = 0.8;

/// Deformation. A hit adds `elasticity × Δv × SQUASH_PER_SPEED` to the rate the marble is
/// flattening, so a squishy marble dropped from the top of the box (about 800 pt/s) flattens to
/// around 40% of its radius, and a tap-speed graze barely dents it.
const SQUASH_PER_SPEED: f64 = 0.012;
/// The most a marble flattens, as a fraction of its radius along the hit; it bulges across the
/// hit to keep its volume (a 3D ball drawn in 2D: `a·b² = r³`).
const SQUASH_MAX: f64 = 0.45;
/// The deformation is a damped spring: this is its natural angular frequency (radians per
/// second; a wobble about every 0.3 s) and damping ratio, under one so a soft marble visibly
/// rings for a cycle or two after a hit before it is round again.
const SQUASH_OMEGA: f64 = 22.0;
const SQUASH_DAMPING: f64 = 0.35;
/// A soft marble resting under its own weight flattens a little: this much of its radius at
/// Earth gravity and full elasticity, in proportion to the gravity otherwise.
const REST_SQUASH: f64 = 0.08;

/// Callback cadence, not a claim about completed GPU presentations. Keep real elapsed time
/// (including slow frames), independently of the physics catch-up cap. Day supplies zero
/// delta on the first callback and after pause/resume; those are not FPS samples. Retain
/// intervals ending in the last two seconds of active animation; a new bounce starts fresh.
#[derive(Default)]
struct FrameStats {
    samples: VecDeque<(f64, f64)>, // (interval end, duration)
    elapsed: f64,
    since_readout: f64,
}
impl FrameStats {
    fn record(&mut self, delta: f64) -> bool {
        if !delta.is_finite() || delta <= 0.0 {
            return false;
        }
        self.elapsed += delta;
        self.samples.push_back((self.elapsed, delta));
        let cutoff = self.elapsed - FPS_WINDOW;
        while self.samples.len() > 1 && self.samples.front().is_some_and(|s| s.0 <= cutoff + 1e-9) {
            self.samples.pop_front();
        }
        self.since_readout += delta;
        if self.samples.len() == 1 || self.since_readout >= 0.25 {
            self.since_readout = 0.0;
            true
        } else {
            false
        }
    }
    fn rates(&self) -> Option<(f64, f64, f64)> {
        (!self.samples.is_empty()).then(|| {
            let (mut shortest, mut longest, mut duration) = (f64::INFINITY, 0.0_f64, 0.0);
            for &(_, dt) in &self.samples {
                shortest = shortest.min(dt);
                longest = longest.max(dt);
                duration += dt;
            }
            let (min, max) = (1.0 / longest, 1.0 / shortest);
            // All three rates use exactly the same window. Clamp only rounding error at
            // the endpoints (e.g. a constant 60 Hz stream's accumulated f64 durations).
            let avg = (self.samples.len() as f64 / duration).clamp(min, max);
            (min, max, avg)
        })
    }
    fn text(&self) -> String {
        let (min, max, avg) = match self.rates() {
            Some((min, max, avg)) => (
                // Figure spaces have a digit's advance, keeping each field steady as
                // values cross 10 or 100 FPS. Allow larger readings without truncating.
                format!("{min:\u{2007}>5.1}"),
                format!("{max:\u{2007}>5.1}"),
                format!("{avg:\u{2007}>5.1}"),
            ),
            None => ("—".into(), "—".into(), "—".into()),
        };
        crate::res::str::frame_fps(crate::res::str::FrameFps { min, max, avg }).format()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Ball {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    /// Where the marble was when the step began: the velocity after the collision passes is
    /// the distance from here, over the step.
    px: f64,
    py: f64,
    /// How flattened the marble is along `(nx, ny)`, as a fraction of its radius: positive is
    /// squashed, negative stretched (the overshoot of a wobble), zero round.
    squash: f64,
    /// The rate `squash` is changing, per second: the spring's velocity.
    squash_vel: f64,
    /// The unit normal of the last hit, the axis the marble flattens along.
    nx: f64,
    ny: f64,
    /// The largest velocity change a collision gave this marble this step (points per second)
    /// and the normal it came along; zero when nothing touched it. Cleared every step.
    hit: f64,
    hit_nx: f64,
    hit_ny: f64,
    /// Whether the marble is pressed against something this step (a wall or another marble),
    /// which is when its weight flattens it a little.
    contact: bool,
}
impl Ball {
    fn at(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            px: x,
            py: y,
            squash: 0.0,
            squash_vel: 0.0,
            nx: 0.0,
            ny: 1.0,
            hit: 0.0,
            hit_nx: 0.0,
            hit_ny: 1.0,
            contact: false,
        }
    }
    fn moving(&self) -> bool {
        self.vx != 0.0 || self.vy != 0.0
    }
    fn speed(&self) -> f64 {
        self.vx.hypot(self.vy)
    }
    /// A collision changed this marble's velocity by `dv` along the unit normal `(nx, ny)`
    /// pointing into the marble (away from what it hit). The step keeps the largest.
    fn struck(&mut self, dv: f64, nx: f64, ny: f64) {
        self.contact = true;
        if dv > self.hit {
            self.hit = dv;
            self.hit_nx = nx;
            self.hit_ny = ny;
        }
    }
}

/// Uniform grid over the box, one cell a marble's diameter wide, holding the index of every
/// marble in each cell (a counting sort, so no allocation after the first step and a fixed
/// order of visits, which keeps the simulation deterministic). A marble can only touch marbles
/// in its own cell and the eight around it.
#[derive(Default)]
struct Grid {
    cols: usize,
    rows: usize,
    cell: f64,
    /// `starts[c]..starts[c + 1]` indexes `items` for cell `c`.
    starts: Vec<usize>,
    items: Vec<usize>,
}
impl Grid {
    /// Never more cells than this: at a thousand marbles on a large window the diameter is still
    /// a few points, and a finer grid than the marbles need only costs the pass that clears it.
    const MAX_CELLS: usize = 40_000;
    fn build(&mut self, balls: &[Ball], size: Size, radius: f64) {
        let mut cell = (radius * 2.0).max(1.0);
        let mut cols = (size.width / cell).ceil().max(1.0) as usize;
        let mut rows = (size.height / cell).ceil().max(1.0) as usize;
        while cols * rows > Self::MAX_CELLS {
            cell *= 1.5;
            cols = (size.width / cell).ceil().max(1.0) as usize;
            rows = (size.height / cell).ceil().max(1.0) as usize;
        }
        (self.cols, self.rows, self.cell) = (cols, rows, cell);
        self.starts.clear();
        self.starts.resize(cols * rows + 1, 0);
        for b in balls {
            let c = self.index(b.x, b.y);
            self.starts[c + 1] += 1;
        }
        for c in 0..cols * rows {
            self.starts[c + 1] += self.starts[c];
        }
        self.items.clear();
        self.items.resize(balls.len(), 0);
        let mut fill = self.starts.clone();
        for (i, b) in balls.iter().enumerate() {
            let c = self.index(b.x, b.y);
            self.items[fill[c]] = i;
            fill[c] += 1;
        }
    }
    fn index(&self, x: f64, y: f64) -> usize {
        let cx = ((x / self.cell).floor().max(0.0) as usize).min(self.cols - 1);
        let cy = ((y / self.cell).floor().max(0.0) as usize).min(self.rows - 1);
        cy * self.cols + cx
    }
    /// Every marble index that can touch marble `i` and comes after it, so each pair is seen
    /// once.
    fn later_neighbors(&self, i: usize, balls: &[Ball], out: &mut Vec<usize>) {
        out.clear();
        let (x, y) = (balls[i].x, balls[i].y);
        let cx = ((x / self.cell).floor().max(0.0) as usize).min(self.cols - 1);
        let cy = ((y / self.cell).floor().max(0.0) as usize).min(self.rows - 1);
        for dy in -1_isize..=1 {
            let Some(ny) = cy.checked_add_signed(dy).filter(|&r| r < self.rows) else {
                continue;
            };
            for dx in -1_isize..=1 {
                let Some(nx) = cx.checked_add_signed(dx).filter(|&c| c < self.cols) else {
                    continue;
                };
                let c = ny * self.cols + nx;
                out.extend(
                    self.items[self.starts[c]..self.starts[c + 1]]
                        .iter()
                        .copied()
                        .filter(|&j| j > i),
                );
            }
        }
    }
}

struct Simulation {
    balls: Vec<Ball>,
    size: Size,
    remainder: f64,
    steps: u64,
    rng: u64,
    /// Downward acceleration in points per second squared ([`POINTS_PER_METRE`] × m/s²).
    gravity: f64,
    /// Each marble's mass in kilograms, which is what pulls it toward every other marble.
    mass: f64,
    /// Coefficient of restitution and softness in one, 0 (hard, dead) to 1 (bouncy, squishy).
    elasticity: f64,
    /// The field those masses make, rebuilt every step while there is any mass.
    mesh: Mesh,
    /// Each marble's acceleration toward the others this step, points per second squared.
    pull: Vec<(f64, f64)>,
    grid: Grid,
    neighbors: Vec<usize>,
    /// The contacts the first projection pass found, with the speed each was closing at before
    /// anything moved: `(a, b, nx, ny, closing)` for a pair (the normal from `a` to `b`) and
    /// `(i, nx, ny, approach)` for a wall (the normal into the box). What the restitution pass
    /// bounces.
    pair_contacts: Vec<(usize, usize, f64, f64, f64)>,
    wall_contacts: Vec<(usize, f64, f64, f64)>,
    /// The marble under the finger, if one is: pinned to `hold` and moving at `hold_vel`, which
    /// is what it fires at when let go, and what it hits the others with meanwhile.
    held: Option<usize>,
    hold: (f64, f64),
    hold_vel: (f64, f64),
    /// Consecutive steps every marble has been slower than the calm threshold, and whether that
    /// has lasted long enough to call the simulation settled (nothing moves; the clock stops).
    calm: u32,
    settled: bool,
}
impl Simulation {
    fn new(count: usize) -> Self {
        let mut sim = Self {
            balls: Vec::with_capacity(MAX_BALLS),
            size: Size::new(360.0, 290.0),
            remainder: 0.0,
            steps: 0,
            rng: 0x9E37_79B9_7F4A_7C15,
            gravity: EARTH * POINTS_PER_METRE,
            mass: 0.0,
            elasticity: DEFAULT_ELASTICITY,
            mesh: Mesh::default(),
            pull: Vec::with_capacity(MAX_BALLS),
            grid: Grid::default(),
            neighbors: Vec::with_capacity(64),
            pair_contacts: Vec::with_capacity(MAX_BALLS * 3),
            wall_contacts: Vec::with_capacity(MAX_BALLS),
            held: None,
            hold: (0.0, 0.0),
            hold_vel: (0.0, 0.0),
            calm: 0,
            settled: false,
        };
        sim.set_count(count);
        // Freshly packed on the floor, nothing is moving: at rest until something changes.
        sim.settled = true;
        sim
    }
    /// The marbles' radius: [`RADIUS`], or less when that many would not fit the box at
    /// [`PACKING`] density, and never more than half the box.
    fn radius(&self) -> f64 {
        let n = self.balls.len().max(1) as f64;
        let fit =
            (self.size.width * self.size.height * PACKING / (std::f64::consts::PI * n)).sqrt();
        RADIUS
            .min(fit)
            .min(self.size.width / 2.0)
            .min(self.size.height / 2.0)
            .max(1.0)
    }
    fn random(&mut self, lo: f64, hi: f64) -> f64 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        lo + (hi - lo) * ((self.rng >> 11) as f64 / (1u64 << 53) as f64)
    }
    /// Where marble `i` sits when the marbles are packed on the floor: hexagonal rows from the
    /// bottom up, each row offset half a diameter from the one below and one marble shorter, so
    /// every marble above the floor sits in the pocket between two below it, the way marbles in
    /// a box actually stack. No two overlap and none overhangs, so a packed layout is at rest.
    fn packed(&self, i: usize, r: f64) -> (f64, f64) {
        let full = ((self.size.width - 2.0 * r) / (2.0 * r)).floor().max(2.0) as usize;
        // Two rows hold `full + (full − 1)` marbles.
        let (pair, rem) = (i / (2 * full - 1), i % (2 * full - 1));
        let (row, col) = if rem < full {
            (2 * pair, rem)
        } else {
            (2 * pair + 1, rem - full)
        };
        let offset = if row % 2 == 1 { r } else { 0.0 };
        let x = (r + offset + col as f64 * 2.0 * r).min(self.size.width - r);
        let y = (self.size.height - r - row as f64 * 2.0 * r * 0.866_025_4).max(r);
        (x, y)
    }
    fn set_count(&mut self, count: usize) {
        let count = count.clamp(1, MAX_BALLS);
        self.unsettle();
        if self.held.is_some_and(|i| i >= count) {
            self.held = None;
        }
        self.balls.truncate(count);
        let mut i = self.balls.len();
        while self.balls.len() < count {
            // The radius with the full count, so new marbles are packed at the size they will be.
            let mut r = RADIUS;
            {
                let fit = (self.size.width * self.size.height * PACKING
                    / (std::f64::consts::PI * count as f64))
                    .sqrt();
                r = r
                    .min(fit)
                    .min(self.size.width / 2.0)
                    .min(self.size.height / 2.0)
                    .max(1.0);
            }
            let (x, y) = self.packed(i, r);
            self.balls.push(Ball::at(x, y));
            i += 1;
        }
    }
    fn reset(&mut self) {
        let r = self.radius();
        for i in 0..self.balls.len() {
            let (x, y) = self.packed(i, r);
            self.balls[i] = Ball::at(x, y);
        }
        self.remainder = 0.0;
        self.steps = 0;
        self.held = None;
        // Packed and still: at rest until a tap or a slider says otherwise.
        self.calm = 0;
        self.settled = true;
    }
    /// Set gravity in m/s². A marble left floating by zero-G starts to fall again as soon as there
    /// is gravity to pull it; the caller wakes the frame clock ([`Simulation::alive`] says so).
    fn set_gravity(&mut self, metres_per_sec2: f64) {
        let g = metres_per_sec2.max(0.0) * POINTS_PER_METRE;
        if g != self.gravity {
            self.gravity = g;
            self.unsettle();
        }
    }
    /// Set each marble's mass in megatonnes. Any mass at all makes the marbles pull on each
    /// other; the caller wakes the frame clock, as for gravity.
    fn set_mass(&mut self, megatonnes: f64) {
        let kg = megatonnes.max(0.0) * 1e9;
        if kg != self.mass {
            self.mass = kg;
            self.unsettle();
        }
    }
    /// Set the elasticity, 0 to 1. A marble resting under its weight flattens by a different
    /// amount, so a settled simulation is woken to show it.
    fn set_elasticity(&mut self, elasticity: f64) {
        let e = elasticity.clamp(0.0, 1.0);
        if e != self.elasticity {
            self.elasticity = e;
            self.unsettle();
        }
    }
    /// Whether the marbles pull on each other: they have mass, and there is more than one.
    fn mutual(&self) -> bool {
        self.mass > 0.0 && self.balls.len() > 1
    }
    /// Something changed that can set marbles moving again: start the settling count over.
    fn unsettle(&mut self) {
        self.calm = 0;
        self.settled = false;
    }
    /// Whether a marble is above the floor rather than resting on it.
    fn floating(&self, ball: &Ball) -> bool {
        ball.y < self.size.height - self.radius() - 1e-9
    }
    /// Impacts slower than this do not bounce ([`REST_SPEED_MIN`]).
    fn rest_speed(&self) -> f64 {
        (2.0 * self.gravity * STEP).max(REST_SPEED_MIN)
    }
    /// Slower than this counts as still ([`CALM_SPEED`]).
    fn calm_speed(&self) -> f64 {
        CALM_SPEED.max(self.gravity * STEP * 0.75)
    }
    fn resize(&mut self, size: Size) {
        if size.width <= 0.0 || size.height <= 0.0 || self.size == size {
            return;
        }
        let old = self.size;
        let floating: Vec<bool> = self.balls.iter().map(|b| self.floating(b)).collect();
        self.size = size;
        let r = self.radius();
        for (ball, floating) in self.balls.iter_mut().zip(floating) {
            ball.x = (ball.x * size.width / old.width).clamp(r, size.width - r);
            ball.y = if ball.moving() || floating {
                (ball.y * size.height / old.height).clamp(r, size.height - r)
            } else {
                size.height - r
            };
        }
        // Scaled positions can overlap or float; let the solver and gravity sort them out.
        self.unsettle();
    }
    fn kick(&mut self) {
        self.unsettle();
        for i in 0..self.balls.len() {
            let vx = self.random(-220.0, 220.0);
            let vy = self.random(-650.0, -420.0);
            self.balls[i].vx = vx;
            self.balls[i].vy = vy;
        }
        self.remainder = 0.0;
    }
    /// Launch every marble toward `target`: along the arc [`launch`] computes through a point near
    /// it, turned off the line by up to [`AIM_JITTER`] and thrown a little harder or softer than
    /// exact, so a crowd fans out around the point with some entropy instead of converging on one
    /// spot. With zero gravity each path is a straight line.
    fn kick_toward(&mut self, target: Point) {
        self.unsettle();
        let g = self.gravity;
        for i in 0..self.balls.len() {
            let (x, y) = (self.balls[i].x, self.balls[i].y);
            let turn = self.random(-AIM_JITTER, AIM_JITTER);
            let spread = self.random(0.9, 1.1);
            // Aim at the point rotated about the marble by `turn`.
            let (dx, dy) = (target.x - x, target.y - y);
            let (sin, cos) = turn.sin_cos();
            let aim = Point::new(x + dx * cos - dy * sin, y + dx * sin + dy * cos);
            let (vx, vy) = launch(Point::new(x, y), aim, g);
            self.balls[i].vx = vx * spread;
            self.balls[i].vy = vy * spread;
        }
        self.remainder = 0.0;
    }
    /// Whether anything can still move: the simulation has not settled.
    fn alive(&self) -> bool {
        !self.settled
    }
    /// Random values for every control, for the Shuffle button: a ball count, planetary gravity
    /// in m/s², a ball mass in megatonnes and an elasticity, each rounded to its slider's step.
    fn shuffled(&mut self) -> (f64, f64, f64, f64) {
        let count = self.random(1.0, MAX_BALLS as f64).round();
        let gravity = (self.random(0.0, JUPITER) * 100.0).round() / 100.0;
        let mass = (self.random(0.0, MAX_MASS_MT) * 1000.0).round() / 1000.0;
        let elasticity = (self.random(0.0, 1.0) * 100.0).round() / 100.0;
        (count, gravity, mass, elasticity)
    }
    /// A random point inside the canvas, for Shuffle's throw.
    fn random_point(&mut self) -> Point {
        let r = self.radius();
        let x = self.random(r, self.size.width - r);
        let y = self.random(r, self.size.height - r);
        Point::new(x, y)
    }
    /// The marble under `at`, if one is: the nearest whose centre is within a grab's reach,
    /// which is the radius with a finger's worth of slack so a small marble can be caught.
    fn ball_at(&self, at: Point) -> Option<usize> {
        let reach = (self.radius() * 1.5).max(14.0);
        self.balls
            .iter()
            .enumerate()
            .map(|(i, b)| (i, (b.x - at.x).hypot(b.y - at.y)))
            .filter(|&(_, d)| d <= reach)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(i, _)| i)
    }
    /// Take hold of the marble under `at`, if there is one. Held, it goes where the finger goes
    /// and nothing moves it: the others bounce off it as off a wall that travels.
    fn grab(&mut self, at: Point) -> bool {
        let Some(i) = self.ball_at(at) else {
            return false;
        };
        self.held = Some(i);
        let b = &self.balls[i];
        self.hold = (b.x, b.y);
        self.hold_vel = (0.0, 0.0);
        self.unsettle();
        true
    }
    /// Move the held marble to `at` (kept inside the box), travelling at `velocity`: the finger's
    /// speed, which is the speed it strikes the others with and the speed it keeps when let go.
    fn drag_to(&mut self, at: Point, velocity: (f64, f64)) {
        if self.held.is_none() {
            return;
        }
        let r = self.radius();
        self.hold = (
            at.x.clamp(r, self.size.width - r),
            at.y.clamp(r, self.size.height - r),
        );
        self.hold_vel = velocity;
        self.unsettle();
    }
    /// Let the held marble go: it flies on at the speed it was being moved, a fling, or simply
    /// drops when the finger had stopped.
    fn release(&mut self) {
        if let Some(i) = self.held.take() {
            let (vx, vy) = self.hold_vel;
            let speed = vx.hypot(vy);
            let scale = if speed > MAX_SPEED {
                MAX_SPEED / speed
            } else {
                1.0
            };
            self.balls[i].vx = vx * scale;
            self.balls[i].vy = vy * scale;
            self.unsettle();
        }
    }
    /// A tap on a marble gives it a flick upward and a little to one side, like a finger
    /// snapping it, rather than throwing the whole crowd. `false` when nothing was there.
    fn poke(&mut self, at: Point) -> bool {
        let Some(i) = self.ball_at(at) else {
            return false;
        };
        let side = self.random(-120.0, 120.0);
        let b = &mut self.balls[i];
        b.vx += side;
        b.vy -= 420.0;
        self.unsettle();
        true
    }
    /// Pin the held marble to the finger: where it is, and how fast it is going.
    fn pin_held(&mut self) {
        if let Some(i) = self.held {
            let b = &mut self.balls[i];
            (b.x, b.y) = self.hold;
            (b.vx, b.vy) = self.hold_vel;
        }
    }
    fn advance(&mut self, elapsed: f64) {
        // Bound catch-up after stalls; neither physics speed nor work depends on refresh rate.
        self.remainder += elapsed.clamp(0.0, 0.1);
        if self.settled {
            self.remainder = 0.0;
            return;
        }
        let g = self.gravity;
        let mutual = self.mutual();
        while self.remainder + 1e-10 >= STEP {
            self.remainder = (self.remainder - STEP).max(0.0);
            self.steps += 1;
            if mutual {
                self.mesh
                    .pull(&self.balls, self.size, self.mass, &mut self.pull);
            }
            // Forces, then free flight for one step. Collisions are resolved afterwards from the
            // overlaps the flight produced. The held marble flies nowhere: it is wherever the
            // finger put it.
            let held = self.held;
            for (i, ball) in self.balls.iter_mut().enumerate() {
                ball.hit = 0.0;
                ball.contact = false;
                ball.px = ball.x;
                ball.py = ball.y;
                if held == Some(i) {
                    continue;
                }
                if mutual {
                    let (ax, ay) = self.pull[i];
                    ball.vx += ax * STEP;
                    ball.vy += ay * STEP;
                }
                ball.vy += g * STEP;
                let speed = ball.speed();
                if speed > MAX_SPEED {
                    ball.vx *= MAX_SPEED / speed;
                    ball.vy *= MAX_SPEED / speed;
                }
                ball.vx *= 1.0 - DRAG_PER_SEC * STEP;
                ball.vy *= 1.0 - DRAG_PER_SEC * STEP;
                ball.x += ball.vx * STEP;
                ball.y += ball.vy * STEP;
            }
            // Collisions, position-based (Müller's PBD, as XPBD and most game engines do it
            // today). The flight above predicted where every marble would be; the passes below
            // move overlapping marbles apart and back inside the box, positions only, and
            // remember each contact with the speed it was closing at. Velocities are then read
            // back from where each marble actually ended up, so a marble the heap held still has
            // no velocity to speak of, however deep the heap: this is what lets a pile of
            // hundreds stand still, where exchanging impulses pair by pair leaves the lower
            // layers forever absorbing the layers above. Restitution comes last, from the
            // remembered closing speeds, so a bounce is still a bounce.
            self.pin_held();
            for pass in 0..SOLVER_PASSES {
                self.project_pairs(pass == 0);
                // Walls last: the box is a hard bound whatever a pair's separation did.
                self.project_walls(pass == 0);
                self.pin_held();
            }
            let held = self.held;
            for (i, ball) in self.balls.iter_mut().enumerate() {
                if held != Some(i) {
                    ball.vx = (ball.x - ball.px) / STEP;
                    ball.vy = (ball.y - ball.py) / STEP;
                }
            }
            self.restitute();
            // Resting contact. A marble pressed against something and barely moving is at rest:
            // the residue of a separation spread over several passes is what static friction
            // would have stopped, and without this a heap never quite stands still.
            let calm_speed = self.calm_speed();
            for ball in &mut self.balls {
                if ball.contact && ball.speed() < calm_speed {
                    ball.vx = 0.0;
                    ball.vy = 0.0;
                }
            }
            self.pin_held();
            self.deform();
            // A held marble is never still: the finger may move it at any moment.
            let calm = self.held.is_none() && self.balls.iter().all(|b| b.speed() < calm_speed);
            self.calm = if calm { self.calm + 1 } else { 0 };
            if self.calm >= CALM_STEPS {
                let el = self.elasticity;
                let rest = self.rest_squash();
                for ball in &mut self.balls {
                    ball.vx = 0.0;
                    ball.vy = 0.0;
                    // Hold the shape a resting marble has at this softness, no wobble left.
                    ball.squash = if ball.contact && el > 0.0 { rest } else { 0.0 };
                    ball.squash_vel = 0.0;
                }
                self.settled = true;
                self.remainder = 0.0;
                return;
            }
        }
    }
    /// How flat a resting marble sits under its weight at this elasticity and gravity.
    fn rest_squash(&self) -> f64 {
        self.elasticity * REST_SQUASH * (self.gravity / (EARTH * POINTS_PER_METRE)).min(2.0)
    }
    /// Keep every marble inside the box, positions only. With `record`, remember each marble
    /// that was pressing on a wall and how fast it was approaching, for [`Simulation::restitute`].
    fn project_walls(&mut self, record: bool) {
        let r = self.radius();
        let (right, bottom) = (self.size.width - r, self.size.height - r);
        if record {
            self.wall_contacts.clear();
        }
        for (i, ball) in self.balls.iter_mut().enumerate() {
            let mut touched: Option<(f64, f64, f64)> = None;
            if ball.x < r {
                ball.x = r;
                touched = Some((1.0, 0.0, -ball.vx));
            } else if ball.x > right {
                ball.x = right;
                touched = Some((-1.0, 0.0, ball.vx));
            }
            if ball.y < r {
                ball.y = r;
                touched = Some((0.0, 1.0, -ball.vy));
            } else if ball.y > bottom {
                ball.y = bottom;
                touched = Some((0.0, -1.0, ball.vy));
            }
            if let Some((nx, ny, approach)) = touched {
                ball.contact = true;
                if record {
                    self.wall_contacts.push((i, nx, ny, approach));
                }
            }
        }
    }
    /// Move every overlapping pair apart along the line between their centres, positions only:
    /// half each, since they weigh the same, or all of it to the marble that is not in the
    /// finger's grip. With `record`, remember each pair and its closing speed for
    /// [`Simulation::restitute`].
    fn project_pairs(&mut self, record: bool) {
        if record {
            self.pair_contacts.clear();
        }
        if self.balls.len() < 2 {
            return;
        }
        let r = self.radius();
        let diameter = 2.0 * r;
        self.grid.build(&self.balls, self.size, r);
        for i in 0..self.balls.len() {
            self.grid
                .later_neighbors(i, &self.balls, &mut self.neighbors);
            for k in 0..self.neighbors.len() {
                let j = self.neighbors[k];
                let (a, b) = (self.balls[i], self.balls[j]);
                let (dx, dy) = (b.x - a.x, b.y - a.y);
                let d2 = dx * dx + dy * dy;
                if d2 >= diameter * diameter {
                    continue;
                }
                // Two marbles on the same spot have no line between them; pick one, the same
                // one every time.
                let (d, nx, ny) = if d2 > 1e-12 {
                    let d = d2.sqrt();
                    (d, dx / d, dy / d)
                } else {
                    (0.0, 1.0, 0.0)
                };
                let (share_a, share_b) = self.shares(i, j);
                let overlap = (diameter - d - SLOP).max(0.0) * SEPARATION;
                if record {
                    let closing = (a.vx - b.vx) * nx + (a.vy - b.vy) * ny;
                    self.pair_contacts.push((i, j, nx, ny, closing));
                }
                // `j > i` by construction of the neighbour list, so the two halves are disjoint.
                let (head, tail) = self.balls.split_at_mut(j);
                let (a, b) = (&mut head[i], &mut tail[0]);
                a.x -= overlap * share_a * nx;
                a.y -= overlap * share_a * ny;
                b.x += overlap * share_b * nx;
                b.y += overlap * share_b * ny;
                a.contact = true;
                b.contact = true;
            }
        }
    }
    /// How a pair splits a separation or an impulse: equal masses take half each; a held marble
    /// is the finger's and takes none, so the other takes all of it, as against a wall.
    fn shares(&self, i: usize, j: usize) -> (f64, f64) {
        match self.held {
            Some(h) if h == i => (0.0, 1.0),
            Some(h) if h == j => (1.0, 0.0),
            _ => (0.5, 0.5),
        }
    }
    /// Bounce. The velocities read back from the projected positions have every contact at
    /// rest; this gives each contact that was closing fast enough to bounce the separating speed
    /// the elasticity says it should have: `−e` times the closing speed, which for two equal
    /// marbles at `e = 1` swaps their velocities and at `e = 0` leaves them moving together.
    /// Momentum is conserved exactly in a pair; a wall absorbs whatever it is given. Friction
    /// takes a little of the sliding speed at every wall hit, and a marble lying on the floor
    /// rolls to a stop.
    fn restitute(&mut self) {
        let e = self.elasticity;
        let rest = self.rest_speed();
        for k in 0..self.pair_contacts.len() {
            let (i, j, nx, ny, closing) = self.pair_contacts[k];
            if closing <= 0.0 {
                continue;
            }
            let restitution = if closing < rest { 0.0 } else { e };
            let (share_a, share_b) = self.shares(i, j);
            let (a, b) = (self.balls[i], self.balls[j]);
            let now = (a.vx - b.vx) * nx + (a.vy - b.vy) * ny;
            // From the closing speed they have to the separating speed they should have.
            let delta = -restitution * closing - now;
            if delta >= 0.0 {
                continue; // already separating faster than the bounce would make them
            }
            let (head, tail) = self.balls.split_at_mut(j);
            let (a, b) = (&mut head[i], &mut tail[0]);
            a.vx += delta * share_a * nx;
            a.vy += delta * share_a * ny;
            b.vx -= delta * share_b * nx;
            b.vy -= delta * share_b * ny;
            if restitution > 0.0 {
                a.struck(-delta * share_a.max(0.5), -nx, -ny);
                b.struck(-delta * share_b.max(0.5), nx, ny);
            }
        }
        for k in 0..self.wall_contacts.len() {
            let (i, nx, ny, approach) = self.wall_contacts[k];
            if self.held == Some(i) || approach <= 0.0 {
                continue;
            }
            let restitution = if approach < rest { 0.0 } else { e };
            let ball = &mut self.balls[i];
            // Normal and tangential parts of what it has now; the normal part becomes the bounce.
            let normal = ball.vx * nx + ball.vy * ny;
            let (tx, ty) = (-ny, nx);
            let tangent = ball.vx * tx + ball.vy * ty;
            let outward = (restitution * approach).max(normal);
            let keep = if restitution == 0.0 && ny < 0.0 {
                FLOOR_ROLL_KEEP
            } else {
                WALL_FRICTION_KEEP
            };
            ball.vx = outward * nx + tangent * keep * tx;
            ball.vy = outward * ny + tangent * keep * ty;
            ball.struck(approach * (1.0 + restitution), nx, ny);
        }
    }
    /// Advance every marble's shape one step: the hits of this step drive the deformation
    /// spring, a resting contact sets where it comes to rest, and the spring relaxes toward
    /// round. A hard marble (elasticity zero) is never driven, so it stays a circle.
    fn deform(&mut self) {
        let el = self.elasticity;
        let rest = self.rest_squash();
        for ball in &mut self.balls {
            if el <= 0.0 {
                ball.squash = 0.0;
                ball.squash_vel = 0.0;
                continue;
            }
            if ball.hit > 0.0 {
                let kick = el * ball.hit * SQUASH_PER_SPEED;
                // A hit at least as hard as whatever the marble is still wobbling from turns
                // the flattening onto the new line; a lighter one keeps the current axis.
                if kick >= ball.squash_vel.abs() * 0.5 {
                    ball.nx = ball.hit_nx;
                    ball.ny = ball.hit_ny;
                }
                ball.squash_vel += kick;
            }
            let target = if ball.contact { rest } else { 0.0 };
            let accel = -SQUASH_OMEGA * SQUASH_OMEGA * (ball.squash - target)
                - 2.0 * SQUASH_DAMPING * SQUASH_OMEGA * ball.squash_vel;
            ball.squash_vel += accel * STEP;
            ball.squash =
                (ball.squash + ball.squash_vel * STEP).clamp(-SQUASH_MAX * 0.6, SQUASH_MAX);
            if ball.squash.abs() < 1e-4 && ball.squash_vel.abs() < 1e-3 {
                ball.squash = 0.0;
                ball.squash_vel = 0.0;
            }
        }
    }
}

/// Mutual gravity on a coarse mesh: every marble deposits its mass on the four mesh nodes around
/// it (cloud-in-cell), the pull at each occupied node is summed directly over every other
/// occupied node, and each marble reads its pull back from the same four nodes with the same
/// weights.
///
/// Direct summation over marbles is n² pairs, 500,000 of them at a thousand marbles, 120 times a
/// second; this costs the square of the occupied nodes instead, a few hundred at most, however many
/// marbles there are. The price is resolution: closer than about one mesh cell, the pull softens
/// to nothing rather than growing without bound, which is also what keeps a close pass from
/// flinging a marble across the box. Because the pair term is antisymmetric and deposit and
/// read-back use the same weights, a marble exerts no net pull on itself and momentum is
/// conserved.
#[derive(Default)]
struct Mesh {
    cols: usize,
    rows: usize,
    cell: f64,
    /// The pair term by node offset, `(dx, dy) / (r² + ε²)^(3/2)` in mesh units, indexed by
    /// [`Mesh::offset`]; rebuilt when the mesh's shape changes.
    kernel: Vec<(f64, f64)>,
    mass: Vec<f64>,
    field: Vec<(f64, f64)>,
    occupied: Vec<usize>,
}

impl Mesh {
    fn shape(&mut self, size: Size) {
        let cell = (size.width.max(size.height) / MESH_CELLS).max(RADIUS * 2.0);
        let cols = (size.width / cell).ceil() as usize + 2;
        let rows = (size.height / cell).ceil() as usize + 2;
        if (cols, rows, cell) == (self.cols, self.rows, self.cell) {
            return;
        }
        (self.cols, self.rows, self.cell) = (cols, rows, cell);
        let (w, h) = (2 * cols - 1, 2 * rows - 1);
        self.kernel = (0..w * h)
            .map(|k| {
                let dx = (k % w) as f64 - (cols - 1) as f64;
                let dy = (k / w) as f64 - (rows - 1) as f64;
                // Softened by one cell: the pull peaks about a cell out and fades inside it.
                let d = (dx * dx + dy * dy + 1.0).powf(1.5);
                (dx / d, dy / d)
            })
            .collect();
        self.mass = vec![0.0; cols * rows];
        self.field = vec![(0.0, 0.0); cols * rows];
    }
    /// The kernel index for the offset from node `a` to node `b`.
    fn offset(&self, a: usize, b: usize) -> usize {
        let (ax, ay) = ((a % self.cols) as isize, (a / self.cols) as isize);
        let (bx, by) = ((b % self.cols) as isize, (b / self.cols) as isize);
        let w = 2 * self.cols - 1;
        let dx = (bx - ax + self.cols as isize - 1) as usize;
        let dy = (by - ay + self.rows as isize - 1) as usize;
        dy * w + dx
    }
    /// The four nodes around `(x, y)` and their cloud-in-cell weights.
    fn corners(&self, x: f64, y: f64) -> [(usize, f64); 4] {
        let (fx, fy) = (x / self.cell, y / self.cell);
        let i = (fx.floor().max(0.0) as usize).min(self.cols - 2);
        let j = (fy.floor().max(0.0) as usize).min(self.rows - 2);
        let (tx, ty) = (
            (fx - i as f64).clamp(0.0, 1.0),
            (fy - j as f64).clamp(0.0, 1.0),
        );
        let n = j * self.cols + i;
        [
            (n, (1.0 - tx) * (1.0 - ty)),
            (n + 1, tx * (1.0 - ty)),
            (n + self.cols, (1.0 - tx) * ty),
            (n + self.cols + 1, tx * ty),
        ]
    }
    /// Every marble's acceleration toward the others, in points per second squared, into `out`.
    fn pull(&mut self, balls: &[Ball], size: Size, mass: f64, out: &mut Vec<(f64, f64)>) {
        self.shape(size);
        self.mass.iter_mut().for_each(|m| *m = 0.0);
        self.occupied.clear();
        for b in balls {
            for (n, w) in self.corners(b.x, b.y) {
                if w > 0.0 {
                    if self.mass[n] == 0.0 {
                        self.occupied.push(n);
                    }
                    self.mass[n] += w;
                }
            }
        }
        // G·m in points³/s² for one marble, with the kernel's cell units turned into points.
        let points_g = NEWTON_G * POINTS_PER_METRE.powi(3) * mass / (self.cell * self.cell);
        for &a in &self.occupied {
            let (mut fx, mut fy) = (0.0, 0.0);
            for &b in &self.occupied {
                let (kx, ky) = self.kernel[self.offset(a, b)];
                fx += kx * self.mass[b];
                fy += ky * self.mass[b];
            }
            self.field[a] = (fx * points_g, fy * points_g);
        }
        out.clear();
        out.extend(balls.iter().map(|b| {
            self.corners(b.x, b.y)
                .iter()
                .fold((0.0, 0.0), |(ax, ay), &(n, w)| {
                    (ax + self.field[n].0 * w, ay + self.field[n].1 * w)
                })
        }));
    }
}

/// The launch velocity that carries a marble from `from` through `to` under gravity `g` (points
/// per second squared, y down) and the simulation's drag. Flight time `t` grows with the distance,
/// so a near point is a flick and a far one a proper throw. Under linear drag `k`,
/// `dv/dt = g − k·v`, so a marble travels `g·t/k + (v₀ − g/k)·(1 − e^(−kt))/k` in time `t`,
/// which solves to `vx = dx·k / (1 − e^(−kt))` and `vy = (dy − g·t/k)·k / (1 − e^(−kt)) + g/k`.
fn launch(from: Point, to: Point, g: f64) -> (f64, f64) {
    let k = DRAG_PER_SEC;
    let (dx, dy) = (to.x - from.x, to.y - from.y);
    let distance = (dx * dx + dy * dy).sqrt();
    let t = (distance * FLIGHT_SECS_PER_POINT).clamp(FLIGHT_MIN_SECS, FLIGHT_MAX_SECS);
    let reach = (1.0 - (-k * t).exp()) / k; // the distance one point per second covers in `t`
    (dx / reach, (dy - g * t / k) / reach + g / k)
}

struct Demo {
    simulation: RefCell<Simulation>,
    repaint: Trigger,
    status: Signal<String>,
    stats: RefCell<FrameStats>,
    fps: Signal<String>,
    paused: Cell<bool>,
    handle: RefCell<Option<day::frame::FrameHandle>>,
    /// Whether the full-screen cover is up: then its canvas owns the simulation's size and the
    /// page's canvas, hidden under it, paints nothing.
    fullscreen: Cell<bool>,
    /// The window state to put back when full screen ends, when entering it changed the window.
    window_before: Cell<Option<day::WindowState>>,
    /// The drag in progress on a marble: where the finger last was and when, and its speed
    /// smoothed over the last few events, which is the fling the marble gets when let go.
    drag: RefCell<Option<Pointer>>,
}
/// A finger on a marble.
struct Pointer {
    at: Point,
    when: std::time::Instant,
    velocity: (f64, f64),
}
impl Demo {
    /// A tap: on a marble, flick it; on empty canvas, throw every marble toward the point.
    fn tap(&self, at: Point) {
        let poked = self.simulation.borrow_mut().poke(at);
        if poked {
            self.wake();
            self.repaint.notify();
        } else {
            self.kick_toward(at);
        }
    }
    /// A drag: pick up the marble under the finger, carry it, and fling it on release.
    fn drag(&self, drag: Drag) {
        let now = std::time::Instant::now();
        match drag.phase {
            DragPhase::Began => {
                let grabbed = self.simulation.borrow_mut().grab(drag.location);
                *self.drag.borrow_mut() = grabbed.then_some(Pointer {
                    at: drag.location,
                    when: now,
                    velocity: (0.0, 0.0),
                });
                if grabbed {
                    self.wake();
                    self.repaint.notify();
                }
            }
            DragPhase::Changed => {
                let mut pointer = self.drag.borrow_mut();
                let Some(p) = pointer.as_mut() else {
                    return;
                };
                // The finger's speed, smoothed: one event's jitter should not decide the fling.
                let dt = now.duration_since(p.when).as_secs_f64().max(1e-3);
                let instant = (
                    (drag.location.x - p.at.x) / dt,
                    (drag.location.y - p.at.y) / dt,
                );
                p.velocity = (
                    p.velocity.0 * 0.5 + instant.0 * 0.5,
                    p.velocity.1 * 0.5 + instant.1 * 0.5,
                );
                p.at = drag.location;
                p.when = now;
                self.simulation
                    .borrow_mut()
                    .drag_to(drag.location, p.velocity);
                self.wake();
            }
            DragPhase::Ended => {
                let Some(p) = self.drag.borrow_mut().take() else {
                    return;
                };
                // A finger that paused before lifting drops the marble rather than flinging it.
                let idle = now.duration_since(p.when).as_secs_f64();
                let keep = (1.0 - idle / 0.15).clamp(0.0, 1.0);
                let mut sim = self.simulation.borrow_mut();
                sim.drag_to(drag.location, (p.velocity.0 * keep, p.velocity.1 * keep));
                sim.release();
                drop(sim);
                self.wake();
                self.repaint.notify();
            }
        }
    }
    fn reset_stats(&self) {
        *self.stats.borrow_mut() = FrameStats::default();
        self.fps.set(self.stats.borrow().text());
    }
    fn kick(&self) {
        self.simulation.borrow_mut().kick();
        self.start();
    }
    fn kick_toward(&self, target: Point) {
        self.simulation.borrow_mut().kick_toward(target);
        self.start();
    }
    /// Set every slider to a random value, then throw the marbles toward a random point, as if
    /// it had been tapped. The sliders' effects run first (a new count re-seeds the marbles, a
    /// new gravity or mass reaches the simulation), so the throw is the last word.
    fn shuffle(
        &self,
        count: Signal<f64>,
        gravity: Signal<f64>,
        mass: Signal<f64>,
        elasticity: Signal<f64>,
    ) {
        let (n, g, m, e) = self.simulation.borrow_mut().shuffled();
        count.set(n);
        gravity.set(g);
        mass.set(m);
        elasticity.set(e);
        day::reactive::flush_now();
        let at = self.simulation.borrow_mut().random_point();
        self.kick_toward(at);
    }
    /// Run the frame clock for a fresh burst of motion.
    fn start(&self) {
        self.reset_stats();
        self.paused.set(false);
        self.status.set(crate::res::str::frame_waiting().format());
        let handle = self.handle.borrow();
        let handle = handle.as_ref().unwrap();
        handle.pause(); // drop an interval partly measured before this bounce
        handle.resume();
        self.repaint.notify();
    }
    fn reset(&self) {
        self.handle.borrow().as_ref().unwrap().pause();
        self.simulation.borrow_mut().reset();
        self.paused.set(false);
        self.reset_stats();
        self.status.set(crate::res::str::frame_resting().format());
        self.repaint.notify();
    }
    /// Wake the clock for a change that can set marbles moving (unless the demo is paused).
    fn wake(&self) {
        let handle = self.handle.borrow();
        if let Some(handle) = handle.as_ref()
            && self.simulation.borrow().alive()
            && !self.paused.get()
            && !handle.is_active()
        {
            self.status.set(crate::res::str::frame_waiting().format());
            handle.resume();
        }
    }
    /// Enter full screen: the cover goes up, and on a platform that can, the window follows it
    /// into the system's full screen, remembering what to put back.
    fn enter_fullscreen(&self) {
        self.fullscreen.set(true);
        if capability(Cap::WindowFullscreen) == Support::Native
            && let Some(window) = day::current_window()
        {
            let before = window.state().get();
            if before != day::WindowState::Fullscreen {
                self.window_before.set(Some(before));
                window.set_state(day::WindowState::Fullscreen);
            }
        }
        self.repaint.notify();
    }
    fn exit_fullscreen(&self) {
        self.fullscreen.set(false);
        if let Some(before) = self.window_before.take()
            && let Some(window) = day::current_window()
        {
            window.set_state(before);
        }
        self.repaint.notify();
    }
}

/// Readout for the gravity slider: the value, and the body it matches when it matches one.
fn gravity_text(g: f64) -> String {
    let value = day::format_decimal(g, 2);
    let body = [
        (
            0.0,
            crate::res::str::physics_zero_g as fn() -> day::LocalizedText,
        ),
        (MOON, crate::res::str::physics_moon),
        (MARS, crate::res::str::physics_mars),
        (EARTH, crate::res::str::physics_earth),
        (JUPITER, crate::res::str::physics_jupiter),
    ]
    .into_iter()
    .find(|(at, _)| (g - at).abs() < 0.05)
    .map(|(_, name)| name().format());
    match body {
        Some(body) => {
            crate::res::str::physics_gravity_named(crate::res::str::PhysicsGravityNamed {
                value,
                body,
            })
            .format()
        }
        None => crate::res::str::physics_gravity(value).format(),
    }
}

/// Readout for the ball mass slider.
fn mass_text(megatonnes: f64) -> String {
    crate::res::str::physics_mass(day::format_decimal(megatonnes, 3)).format()
}

/// Readout for the elasticity slider, as a percentage.
fn elasticity_text(elasticity: f64) -> String {
    crate::res::str::physics_elasticity(day::format_decimal((elasticity * 100.0).round(), 0))
        .format()
}

/// One row of the controls form: its title in a column as wide as the widest title, the slider,
/// and its value read out at the trailing end in a box as wide as `widest`. Both reservations
/// keep the sliders aligned, and keep each one still as its digits change.
///
/// At Compact width (a phone) the title and the widest readout together leave a slider no room
/// to move in, so the control stacks: the title with its readout at the trailing edge, and the
/// slider under them with the whole row to itself. `size_class()` is a tracked read
/// (docs/size-classes.md), so a rotation or a window dragged narrow re-lays the form.
fn control(
    title: day::LocalizedText,
    widest_title: String,
    slider: impl Piece,
    readout: impl Fn() -> String + 'static,
    widest: String,
    readout_id: day::ElementId,
) -> AnyPiece {
    let compact = day::size_class()
        .map(|c| c.width == WidthClass::Compact)
        .unwrap_or(false);
    // `widgets::numeric_readout`, trailing-aligned inside its reserved box.
    let readout = label(readout)
        .tabular()
        .align(TextAlign::Trailing)
        .id(readout_id)
        .reserving(widest);
    if compact {
        column((
            row((label(title).grow_w(), readout))
                .spacing(Space::M)
                .align(VAlign::Center)
                .grow_w(),
            column((slider,)).grow_w(),
        ))
        .spacing(Space::XS)
        .grow_w()
        .any()
    } else {
        row((
            label(title).reserving(widest_title),
            column((slider,)).grow_w(),
            readout,
        ))
        .spacing(Space::M)
        .align(VAlign::Center)
        .any()
    }
}

/// The marbles' canvas: paints the simulation and throws the marbles at a tap. `owns_size` is
/// whether this canvas's size is the simulation's box (the page's canvas gives that up while
/// the full-screen cover is showing, and paints nothing under it).
fn marbles(ui: Rc<Demo>, owns_size: bool, id: day::ElementId) -> AnyPiece {
    let (draw, tap, drag) = (ui.clone(), ui.clone(), ui);
    canvas(move |d, size| {
        draw.repaint.track();
        let fullscreen = draw.fullscreen.get();
        if fullscreen != owns_size {
            return;
        }
        let mut sim = draw.simulation.borrow_mut();
        sim.resize(size);
        paint(d, size, &sim);
    })
    // A tap on empty canvas throws every marble toward the point; a tap on a marble flicks it,
    // and a drag picks it up and flings it.
    .on_tap_at(move |at| tap.tap(at))
    .on_drag(move |d| drag.drag(d))
    .a11y(|a| a.role(Role::Button).label(crate::res::str::frame_hint()))
    .id(id)
    .grow()
    .any()
}

pub(crate) fn physics_page() -> AnyPiece {
    // Twenty marbles to start: enough to see them scatter, collide and pull on each other,
    // light enough for any device; the slider runs the count up to `MAX_BALLS` for a load test.
    let count = Signal::new(20.0_f64);
    // Each marble's mass in megatonnes. Massless to start: the marbles ignore each other until
    // the slider gives them weight.
    let mass = Signal::new(0.0_f64);
    // Zero-G to start: a tap sends the marbles drifting toward the point, and the slider adds
    // whatever pull the reader wants.
    let gravity = Signal::new(0.0);
    let elasticity = Signal::new(DEFAULT_ELASTICITY);
    // The full-screen cover's own key: a control's panel, not a place (docs/cover.md), so it
    // stays out of the route space.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    struct FullscreenKey;
    impl Route for FullscreenKey {
        fn key(&self) -> String {
            "fullscreen".into()
        }
        fn from_key(key: &str) -> Option<Self> {
            (key == "fullscreen").then_some(FullscreenKey)
        }
    }
    let fullscreen_open = Signal::new(Option::<FullscreenKey>::None);
    let ui = Rc::new(Demo {
        simulation: RefCell::new(Simulation::new(20)),
        repaint: Trigger::new(),
        status: Signal::new(crate::res::str::frame_resting().format()),
        stats: RefCell::new(FrameStats::default()),
        fps: Signal::new(FrameStats::default().text()),
        paused: Cell::new(false),
        handle: RefCell::new(None),
        fullscreen: Cell::new(false),
        window_before: Cell::new(None),
        drag: RefCell::new(None),
    });
    let weak = Rc::downgrade(&ui);
    let handle = day::frame::FrameClock::current().subscribe(move |frame| {
        let Some(ui) = weak.upgrade() else {
            return ControlFlow::Break(());
        };
        let alive = {
            let mut sim = ui.simulation.borrow_mut();
            sim.advance(frame.delta.as_secs_f64());
            sim.alive()
        };
        let publish_stats = ui.stats.borrow_mut().record(frame.delta.as_secs_f64());
        if publish_stats || !alive {
            ui.fps.set(ui.stats.borrow().text());
        }
        // Diagnostics update occasionally; only the canvas binding invalidates every frame.
        if frame.delta.is_zero() || publish_stats || !alive {
            ui.status.set(
                if alive {
                    crate::res::str::frame_running()
                } else {
                    crate::res::str::frame_resting()
                }
                .format(),
            );
        }
        ui.repaint.notify();
        if alive {
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(())
        }
    });
    handle.pause();
    *ui.handle.borrow_mut() = Some(handle);
    let cleanup = ui.clone();
    Scope::current().on_cleanup(move || {
        cleanup.handle.borrow_mut().take();
    });
    let resize = ui.clone();
    Effect::new(move || {
        let n = count.get().round().clamp(1.0, MAX_BALLS as f64) as usize;
        if resize.simulation.borrow().balls.len() != n {
            resize.simulation.borrow_mut().set_count(n);
            resize.reset_stats();
            // Discard an interval partly measured at the old load. Resume's first delta is zero.
            resize.handle.borrow().as_ref().unwrap().pause();
            // Changing load starts a fresh burst so even new marbles immediately animate.
            // A paused demo stays paused, making comparison screenshots easy.
            if resize.paused.get() {
                resize.simulation.borrow_mut().kick();
                resize.repaint.notify();
            } else {
                resize.kick();
            }
        }
    });
    // Gravity, mass and elasticity reach the simulation live. A marble left floating by zero-G
    // falls as soon as there is gravity again, marbles given mass start pulling on each other,
    // and a softer marble sits flatter, so a sleeping clock is woken for any of them (unless
    // the demo is paused).
    let pull = ui.clone();
    Effect::new(move || {
        let (g, m, e) = (gravity.get(), mass.get(), elasticity.get());
        {
            let mut sim = pull.simulation.borrow_mut();
            sim.set_gravity(g);
            sim.set_mass(m);
            sim.set_elasticity(e);
        }
        pull.wake();
    });
    // The cover follows the signal both ways: a tap on Full screen opens it, and whatever closes
    // it (the button, the platform's back) puts the window and the page's canvas back.
    let follow = ui.clone();
    Effect::new(move || {
        let open = fullscreen_open.get().is_some();
        if open != follow.fullscreen.get() {
            if open {
                follow.enter_fullscreen();
            } else {
                follow.exit_fullscreen();
            }
        }
    });
    let (bounce, shuffle, pause, reset) = (ui.clone(), ui.clone(), ui.clone(), ui.clone());
    // The widest each readout can be in this locale, so the sliders line up and hold still:
    // every named gravity reading, and the extremes of the others.
    let widest_gravity = [0.0, MOON, MARS, EARTH, JUPITER, 88.88]
        .into_iter()
        .map(gravity_text)
        .max_by_key(|t| t.chars().count())
        .unwrap_or_default();
    // One width for all the readouts, so the sliders end together as well as start together.
    let widest_value = [
        day::format_decimal(8888.0, 0),
        mass_text(8.888),
        elasticity_text(1.0),
        widest_gravity,
    ]
    .into_iter()
    .max_by_key(|t| t.chars().count())
    .unwrap_or_default();
    let widest_title = [
        crate::res::str::frame_balls_label(),
        crate::res::str::physics_mass_label(),
        crate::res::str::physics_gravity_label(),
        crate::res::str::physics_elasticity_label(),
    ]
    .into_iter()
    .map(|t| t.format())
    .max_by_key(|t| t.chars().count())
    .unwrap_or_default();
    let (status, fps) = (ui.status, ui.fps);
    let cover_ui = ui.clone();
    // Not `widgets::page`: that scrolls, and the marbles' box takes every point the page has
    // left, which a scroll view never offers.
    column((
        crate::widgets::heading(crate::res::str::nav_physics(), day::id!(physics_title)),
        label(crate::res::str::frame_hint()).font(Font::Callout),
        marbles(ui.clone(), false, day::id!(frame_ball)),
        // The four controls as one form under the marbles, each slider with its value beside it.
        section((column((
            control(
                crate::res::str::frame_balls_label(),
                widest_title.clone(),
                slider(count)
                    .range(1.0..=MAX_BALLS as f64)
                    .step(1.0)
                    .a11y(|a| a.label(crate::res::str::frame_balls_label()))
                    .id(day::id!(frame_ball_slider)),
                move || day::format_decimal(count.get().round(), 0),
                widest_value.clone(),
                day::id!(frame_ball_count),
            ),
            control(
                crate::res::str::physics_elasticity_label(),
                widest_title.clone(),
                slider(elasticity)
                    .range(0.0..=1.0)
                    .step(0.01)
                    .a11y(|a| a.label(crate::res::str::physics_elasticity_label()))
                    .id(day::id!(physics_elasticity_slider)),
                move || elasticity_text(elasticity.get()),
                widest_value.clone(),
                day::id!(physics_elasticity_value),
            ),
            control(
                crate::res::str::physics_mass_label(),
                widest_title.clone(),
                slider(mass)
                    .range(0.0..=MAX_MASS_MT)
                    .step(0.001)
                    .a11y(|a| a.label(crate::res::str::physics_mass_label()))
                    .id(day::id!(physics_mass_slider)),
                move || mass_text(mass.get()),
                widest_value.clone(),
                day::id!(physics_mass_value),
            ),
            control(
                crate::res::str::physics_gravity_label(),
                widest_title.clone(),
                slider(gravity)
                    .range(0.0..=JUPITER)
                    .step(0.01)
                    .a11y(|a| a.label(crate::res::str::physics_gravity_label()))
                    .id(day::id!(physics_gravity_slider)),
                move || gravity_text(gravity.get()),
                widest_value,
                day::id!(physics_gravity_value),
            ),
        ))
        .spacing(Space::S),)),
        row((
            button(crate::res::str::frame_bounce())
                .action(move || bounce.kick())
                .id(day::id!(frame_bounce)),
            button(crate::res::str::frame_shuffle())
                .action(move || shuffle.shuffle(count, gravity, mass, elasticity))
                .id(day::id!(physics_shuffle)),
            button(crate::res::str::frame_pause())
                .action(move || {
                    let handle = pause.handle.borrow();
                    let handle = handle.as_ref().unwrap();
                    if pause.paused.get() {
                        pause.paused.set(false);
                        pause.status.set(crate::res::str::frame_waiting().format());
                        handle.resume();
                    } else if handle.is_active() {
                        pause.paused.set(true);
                        handle.pause();
                        pause.fps.set(pause.stats.borrow().text());
                        pause.status.set(crate::res::str::frame_paused().format());
                    }
                })
                .id(day::id!(frame_pause)),
            button(crate::res::str::anim_reset_label())
                .action(move || reset.reset())
                .id(day::id!(frame_reset)),
            button(crate::res::str::physics_fullscreen())
                .action(move || fullscreen_open.set(Some(FullscreenKey)))
                .id(day::id!(physics_fullscreen)),
        ))
        .spacing(10.0),
        row((
            label(status).id(day::id!(frame_status)),
            spacer(),
            label(fps)
                .tabular()
                .align(TextAlign::Trailing)
                .id(day::id!(frame_fps))
                .reserving(
                    crate::res::str::frame_fps(crate::res::str::FrameFps {
                        min: "999.9",
                        max: "999.9",
                        avg: "999.9",
                    })
                    .format(),
                )
                .font(Font::Caption),
        )),
        label(crate::res::str::frame_explanation()).font(Font::Caption),
        // Full screen: the same marbles over the whole window, with one way out in the corner.
        cover(fullscreen_open, move |_| {
            marbles(cover_ui.clone(), true, day::id!(physics_fullscreen_canvas))
                .overlay_aligned(
                    Alignment::TopTrailing,
                    button(crate::res::str::physics_exit_fullscreen())
                        .prominent()
                        .action(move || fullscreen_open.set(None))
                        .id(day::id!(physics_exit_fullscreen))
                        .padding(Space::L),
                )
                .any()
        })
        .unrouted()
        .background(|_| Color::hex(0x111820)),
    ))
    .spacing(10.0)
    .padding(20.0)
    .grow()
    .id(day::id!(frame_demo))
    .any()
}

fn paint(d: &mut Draw, size: Size, sim: &Simulation) {
    if size.width <= 0.0 || size.height <= 0.0 {
        return;
    }
    let bounds = Rect::new(0.0, 0.0, size.width, size.height);
    d.fill(
        Shape::Rect(bounds),
        LinearGradient::new(
            UnitPoint::TOP,
            UnitPoint::BOTTOM,
            vec![(0.0, Color::hex(0x111820)), (1.0, Color::hex(0x303A44))],
        ),
    );
    let r = sim.radius();
    // Fixed colors follow ball identity; only positions change. Keep each marble's four
    // layers together so overlaps occlude correctly, without trails or particle effects.
    const HUES: [f64; 10] = [
        210.0, 5.0, 155.0, 280.0, 40.0, 185.0, 330.0, 95.0, 245.0, 25.0,
    ];
    for (i, ball) in sim.balls.iter().enumerate() {
        let hue = HUES[i % HUES.len()];
        let shape = Squash::of(ball);
        let orb = shape.ellipse(ball.x, ball.y, 0.0, 0.0, r, r);
        d.fill(
            orb.clone(),
            RadialGradient::new(
                UnitPoint::new(0.3, 0.22),
                0.9,
                vec![
                    (0.0, Color::hsl(hue, 0.35, 0.97)),
                    (0.28, Color::hsl(hue, 0.7, 0.72)),
                    (0.55, Color::hsl(hue, 0.75, 0.43)),
                    (0.8, Color::hsl(hue, 0.7, 0.23)),
                    (1.0, Color::hsl(hue, 0.55, 0.12)),
                ],
            ),
        );
        // The rim brightens with the deformation: a squashed marble is a marble being hit.
        let glow = 0.7 + 0.3 * (ball.squash.abs() * 3.0).min(1.0);
        d.stroke(orb, Color::hsl(hue, 0.5, 0.8).with_alpha(glow), 0.7);
        // The highlight and the underglow deform with the marble, so a squashed one reads as a
        // squashed sphere rather than a sphere behind a stretched outline.
        d.fill(
            shape.ellipse(ball.x, ball.y, -r * 0.21, -r * 0.51, r * 0.31, r * 0.14),
            Color::WHITE.with_alpha(0.85),
        );
        d.fill(
            shape.ellipse(ball.x, ball.y, r * 0.125, r * 0.55, r * 0.325, r * 0.1),
            Color::hsl(hue, 0.8, 0.8).with_alpha(0.6),
        );
    }
    // The marble in hand wears a ring, so the eye knows which one the finger owns.
    if let Some(ball) = sim.held.and_then(|i| sim.balls.get(i)) {
        d.stroke(
            Shape::Ellipse(Rect::new(
                ball.x - r - 4.0,
                ball.y - r - 4.0,
                (r + 4.0) * 2.0,
                (r + 4.0) * 2.0,
            )),
            Color::WHITE.with_alpha(0.75),
            1.5,
        );
    }
    d.stroke(
        Shape::Rect(Rect::new(0.5, 0.5, size.width - 1.0, size.height - 1.0)),
        Color::hex(0x8293A0).with_alpha(0.4),
        1.0,
    );
}

/// A marble's deformation as the linear map that turns its round shape into its squashed one:
/// scaled by `1 − s` along the hit normal and by `1/√(1 − s)` across it, so the drawn area is
/// what a ball of constant volume shows when flattened by `s` (`a·b² = r³`). Every feature of
/// the marble (body, highlight, underglow) is an ellipse in the round marble's own coordinates,
/// and an affine map takes an ellipse to an ellipse, so each is drawn by mapping the control
/// points of the standard four-cubic ellipse. A round marble skips all that for the plain
/// ellipse the renderer knows.
#[derive(Clone, Copy)]
struct Squash {
    // The 2×2 matrix, row-major, or `None` for the identity.
    m: Option<[f64; 4]>,
}
impl Squash {
    fn of(ball: &Ball) -> Self {
        let s = ball.squash;
        if s.abs() < 0.004 {
            return Self { m: None };
        }
        let along = 1.0 - s;
        let across = 1.0 / along.sqrt();
        let (nx, ny) = (ball.nx, ball.ny);
        // along·n·nᵀ + across·t·tᵀ with t = (−ny, nx).
        Self {
            m: Some([
                along * nx * nx + across * ny * ny,
                along * nx * ny - across * nx * ny,
                along * nx * ny - across * nx * ny,
                along * ny * ny + across * nx * nx,
            ]),
        }
    }
    /// The ellipse of radii `(rx, ry)` centred `(ox, oy)` from the marble's centre `(cx, cy)`,
    /// in the marble's round coordinates, drawn deformed.
    fn ellipse(&self, cx: f64, cy: f64, ox: f64, oy: f64, rx: f64, ry: f64) -> Shape {
        let Some(m) = self.m else {
            return Shape::Ellipse(Rect::new(cx + ox - rx, cy + oy - ry, rx * 2.0, ry * 2.0));
        };
        const K: f64 = 0.552_284_749_8;
        let map = |x: f64, y: f64| {
            let (x, y) = (ox + x * rx, oy + y * ry);
            Point::new(cx + m[0] * x + m[1] * y, cy + m[2] * x + m[3] * y)
        };
        PathBuilder::new()
            .move_to(map(1.0, 0.0))
            .cubic_to(map(1.0, K), map(K, 1.0), map(0.0, 1.0))
            .cubic_to(map(-K, 1.0), map(-1.0, K), map(-1.0, 0.0))
            .cubic_to(map(-1.0, -K), map(-K, -1.0), map(0.0, -1.0))
            .cubic_to(map(K, -1.0), map(1.0, -K), map(1.0, 0.0))
            .close()
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fps_extrema_and_average_use_real_elapsed_time() {
        let mut stats = FrameStats::default();
        assert_eq!(stats.rates(), None);
        for dt in [0.01, 0.02, 0.05] {
            stats.record(dt);
        }
        let (min, max, avg) = stats.rates().unwrap();
        assert_eq!(min, 20.0);
        assert_eq!(max, 100.0);
        assert!((avg - 37.5).abs() < 1e-9);
        // A real stall affects FPS even though physics catches up by at most 100 ms.
        stats.record(2.0);
        assert_eq!(stats.rates().unwrap().0, 0.5);
        assert_eq!(stats.rates(), Some((0.5, 0.5, 0.5)));
    }
    #[test]
    fn fps_window_forgets_old_extrema_and_keeps_average_between_them() {
        let mut stats = FrameStats::default();
        stats.record(0.2); // 5 FPS
        stats.record(0.005); // 200 FPS
        for _ in 0..180 {
            stats.record(1.0 / 60.0);
            let (min, max, avg) = stats.rates().unwrap();
            assert!(min <= avg && avg <= max);
        }
        assert_eq!(stats.rates(), Some((60.0, 60.0, 60.0)));
        assert_eq!(stats.samples.len(), 120);
        for _ in 0..90 {
            stats.record(1.0 / 30.0);
        }
        assert_eq!(stats.rates(), Some((30.0, 30.0, 30.0)));
        assert_eq!(stats.samples.len(), 60);
    }
    #[test]
    fn fps_labels_match_their_numeric_values() {
        crate::res::locales::install();
        set_locale("en");
        let mut stats = FrameStats::default();
        for dt in [0.01, 0.02, 0.05] {
            stats.record(dt);
        }
        let text = stats.text().replace(['\u{2068}', '\u{2069}'], "");
        assert_eq!(
            text,
            "FPS · min \u{2007}20.0 · max 100.0 · avg \u{2007}37.5"
        );
    }
    #[test]
    fn fps_ignores_first_resume_and_invalid_intervals() {
        let mut stats = FrameStats::default();
        for dt in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(!stats.record(dt));
        }
        assert_eq!(stats.rates(), None);
        stats.record(1.0 / 60.0);
        let before_pause = stats.rates();
        stats.record(0.0); // native clock's first callback after any length of pause
        assert_eq!(stats.rates(), before_pause);
        stats.record(1.0 / 60.0);
        assert_eq!(stats.rates(), Some((60.0, 60.0, 60.0)));
    }
    #[test]
    fn fps_readout_is_throttled_and_reset_starts_fresh() {
        let mut stats = FrameStats::default();
        let updates = (0..120).filter(|_| stats.record(1.0 / 120.0)).count();
        assert!((4..=5).contains(&updates));
        stats = FrameStats::default();
        assert_eq!(stats.rates(), None);
        stats.record(1.0 / 30.0);
        assert_eq!(stats.rates(), Some((30.0, 30.0, 30.0)));
    }
    fn in_bounds(sim: &Simulation) {
        let r = sim.radius();
        for b in &sim.balls {
            assert!((r..=sim.size.width - r).contains(&b.x), "{b:?}");
            assert!((r..=sim.size.height - r).contains(&b.y), "{b:?}");
        }
    }
    /// The deepest overlap between any two marbles, in points, zero when none overlap.
    fn deepest_overlap(sim: &Simulation) -> f64 {
        let d = 2.0 * sim.radius();
        let mut worst = 0.0_f64;
        for (i, a) in sim.balls.iter().enumerate() {
            for b in &sim.balls[i + 1..] {
                worst = worst.max(d - (a.x - b.x).hypot(a.y - b.y));
            }
        }
        worst
    }
    fn settle(sim: &mut Simulation, max_steps: usize) -> bool {
        (0..max_steps).any(|_| {
            sim.advance(STEP);
            !sim.alive()
        })
    }
    #[test]
    fn all_250_settle_into_a_heap_and_can_bounce_again() {
        let mut sim = Simulation::new(250);
        assert!(deepest_overlap(&sim) < 1e-6, "packed without overlap");
        sim.kick();
        for _ in 0..1200 {
            sim.advance(STEP);
            in_bounds(&sim);
        }
        assert!(
            settle(&mut sim, 1200),
            "a kicked crowd comes to rest within twenty seconds"
        );
        in_bounds(&sim);
        // A heap: some marbles rest on others, none sinks into another.
        assert!(sim.balls.iter().any(|b| sim.floating(b)));
        assert!(
            deepest_overlap(&sim) < sim.radius() * 0.25,
            "overlap {}",
            deepest_overlap(&sim)
        );
        sim.kick();
        assert!(sim.balls.iter().all(Ball::moving));
    }
    #[test]
    fn same_simulation_at_30_60_and_120_hz() {
        let simulate = |hz| {
            let mut sim = Simulation::new(250);
            sim.kick();
            for _ in 0..hz {
                sim.advance(1.0 / hz as f64);
            }
            (sim.balls, sim.steps)
        };
        assert_eq!(simulate(30), simulate(120));
        assert_eq!(simulate(60), simulate(120));
    }
    #[test]
    fn long_stall_has_bounded_work() {
        let mut sim = Simulation::new(250);
        sim.kick();
        sim.advance(120.0);
        assert_eq!(sim.steps, 12);
        in_bounds(&sim);
    }
    #[test]
    fn each_wall_reflects_velocity_inward() {
        for (x, y, vx, vy) in [
            (12.0, 80.0, -200.0, 0.0),
            (348.0, 80.0, 200.0, 0.0),
            (80.0, 12.0, 0.0, -500.0),
            (80.0, 278.0, 0.0, 500.0),
        ] {
            let mut sim = Simulation::new(1);
            sim.balls[0] = Ball {
                vx,
                vy,
                ..Ball::at(x, y)
            };
            sim.unsettle();
            sim.advance(STEP);
            in_bounds(&sim);
            let b = sim.balls[0];
            if vx != 0.0 {
                assert!(b.vx * vx < 0.0);
            }
            if vy != 0.0 {
                assert!(b.vy * vy < 0.0);
            }
        }
    }
    #[test]
    fn count_changes_and_resize_keep_storage_and_positions_bounded() {
        let mut sim = Simulation::new(0);
        assert_eq!(sim.balls.len(), 1);
        sim.set_count(4999);
        assert_eq!(sim.balls.len(), MAX_BALLS);
        assert!(sim.radius() < RADIUS, "a thousand marbles shrink to fit");
        sim.kick();
        for size in [
            Size::new(120.0, 80.0),
            Size::new(10.0, 10.0),
            Size::new(640.0, 290.0),
        ] {
            sim.resize(size);
            in_bounds(&sim);
            for _ in 0..120 {
                sim.advance(STEP);
                in_bounds(&sim);
            }
        }
        sim.set_count(1);
        assert_eq!(sim.radius(), RADIUS);
        sim.reset();
        assert_eq!(sim.balls.len(), 1);
        assert!(!sim.alive());
        in_bounds(&sim);
    }
    #[test]
    fn kicks_vary_between_bearings_and_between_taps() {
        let mut sim = Simulation::new(250);
        sim.kick();
        assert!(sim.balls.iter().any(|b| b.vx < 0.0));
        assert!(sim.balls.iter().any(|b| b.vx > 0.0));
        assert!(
            sim.balls
                .windows(2)
                .all(|p| p[0].vx != p[1].vx && p[0].vy != p[1].vy)
        );
        let first = sim.balls.clone();
        sim.kick();
        assert_ne!(first, sim.balls);
    }
    /// The launch is aimed: flying freely (no walls in the way), a marble thrown with the exact
    /// launch velocity reaches the point, under any gravity, give or take the fixed step.
    #[test]
    fn the_launch_arc_passes_through_the_point() {
        for g in [EARTH, 0.0, JUPITER] {
            let mut sim = Simulation::new(1);
            sim.resize(Size::new(800.0, 600.0));
            sim.set_gravity(g);
            let (from, target) = (Point::new(100.0, 588.0), Point::new(500.0, 250.0));
            let (vx, vy) = launch(from, target, sim.gravity);
            sim.balls[0] = Ball {
                vx,
                vy,
                ..Ball::at(from.x, from.y)
            };
            let mut closest = f64::INFINITY;
            for _ in 0..240 {
                sim.advance(STEP);
                let b = sim.balls[0];
                closest = closest.min((b.x - target.x).hypot(b.y - target.y));
            }
            assert!(
                closest < 12.0,
                "g = {g}: passed {closest:.1} pt from the point"
            );
        }
    }

    /// A tap keeps some entropy: the marbles all head for the point, but not along exactly the
    /// same bearing, and none strays further off it than the jitter allows.
    #[test]
    fn a_tap_scatters_the_throws_around_the_point() {
        let mut sim = Simulation::new(200);
        sim.resize(Size::new(800.0, 600.0));
        sim.set_gravity(0.0);
        let target = Point::new(400.0, 100.0);
        sim.kick_toward(target);
        let offsets: Vec<f64> = sim
            .balls
            .iter()
            .map(|b| {
                let toward = (target.y - b.y).atan2(target.x - b.x);
                let heading = b.vy.atan2(b.vx);
                (heading - toward + std::f64::consts::PI).rem_euclid(std::f64::consts::TAU)
                    - std::f64::consts::PI
            })
            .collect();
        assert!(offsets.iter().all(|o| o.abs() <= AIM_JITTER + 1e-9));
        assert!(offsets.iter().any(|o| *o > AIM_JITTER / 2.0));
        assert!(offsets.iter().any(|o| *o < -AIM_JITTER / 2.0));
    }

    #[test]
    fn zero_g_floats_and_gravity_brings_it_back_down() {
        let mut sim = Simulation::new(1);
        sim.set_gravity(0.0);
        sim.kick_toward(Point::new(180.0, 60.0));
        assert!(
            settle(&mut sim, 20_000),
            "a zero-G marble comes to rest: {:?}",
            sim.balls[0]
        );
        let b = sim.balls[0];
        assert!(
            sim.floating(&b),
            "and rests where it is, above the floor: {b:?}"
        );
        // Gravity again: the floating marble is alive and falls to the floor.
        sim.set_gravity(EARTH);
        assert!(sim.alive());
        assert!(settle(&mut sim, 2400));
        assert_eq!(sim.balls[0].y, sim.size.height - sim.radius());
    }

    #[test]
    fn stronger_gravity_brings_a_dropped_marble_down_sooner() {
        let fall_steps = |g: f64| {
            let mut sim = Simulation::new(1);
            sim.set_gravity(g);
            sim.balls[0] = Ball::at(100.0, sim.radius());
            sim.unsettle();
            (0..10_000)
                .find(|_| {
                    sim.advance(STEP);
                    !sim.floating(&sim.balls[0])
                })
                .unwrap()
        };
        let (moon, earth, jupiter) = (fall_steps(MOON), fall_steps(EARTH), fall_steps(JUPITER));
        assert!(moon > earth && earth > jupiter, "{moon} {earth} {jupiter}");
    }

    fn floating_pair(mass_mt: f64) -> Simulation {
        let mut sim = Simulation::new(2);
        sim.resize(Size::new(800.0, 600.0));
        sim.set_gravity(0.0);
        sim.set_mass(mass_mt);
        sim.balls[0] = Ball::at(250.0, 300.0);
        sim.balls[1] = Ball::at(550.0, 300.0);
        sim
    }

    #[test]
    fn massive_marbles_pull_each_other_together_and_massless_ones_do_not() {
        let mut sim = floating_pair(300.0);
        assert!(
            sim.alive(),
            "marbles with mass pull on each other from rest"
        );
        for _ in 0..120 {
            sim.advance(STEP);
        }
        let gap = sim.balls[1].x - sim.balls[0].x;
        assert!(gap < 300.0 - 10.0, "the pair closed in: {gap}");
        // Symmetric: each moved the same distance toward the other.
        assert!(((sim.balls[0].x - 250.0) - (550.0 - sim.balls[1].x)).abs() < 1e-6);
        assert!((sim.balls[0].y - 300.0).abs() < 1e-6);

        let mut still = floating_pair(0.0);
        assert!(settle(&mut still, CALM_STEPS as usize + 1));
        still.advance(1.0);
        assert_eq!((still.balls[0].x, still.balls[1].x), (250.0, 550.0));
    }

    /// Deposit and read-back share their weights and the pair term is antisymmetric, so a marble
    /// does not pull on itself, and a crowd's pulls cancel overall (momentum is conserved).
    #[test]
    fn the_mesh_exerts_no_self_force_and_conserves_momentum() {
        let mut mesh = Mesh::default();
        let size = Size::new(900.0, 500.0);
        let mut out = Vec::new();
        let alone = [Ball::at(333.3, 123.4)];
        mesh.pull(&alone, size, 1e10, &mut out);
        assert!(
            out[0].0.abs() < 1e-9 && out[0].1.abs() < 1e-9,
            "{:?}",
            out[0]
        );

        let mut sim = Simulation::new(300);
        sim.resize(size);
        sim.kick();
        for _ in 0..30 {
            sim.advance(STEP);
        }
        mesh.pull(&sim.balls, size, 1e10, &mut out);
        let (sx, sy) = out.iter().fold((0.0, 0.0), |(x, y), p| (x + p.0, y + p.1));
        let typical = out.iter().map(|p| p.0.hypot(p.1)).sum::<f64>() / out.len() as f64;
        assert!(typical > 0.0);
        assert!(
            sx.hypot(sy) < typical * 1e-6,
            "net pull {sx}, {sy} vs {typical}"
        );
    }

    #[test]
    fn a_massive_crowd_settles_and_stays_settled() {
        let mut sim = Simulation::new(60);
        sim.resize(Size::new(800.0, 600.0));
        sim.set_gravity(0.0);
        sim.set_mass(MAX_MASS_MT);
        sim.kick_toward(Point::new(400.0, 300.0));
        assert!(
            settle(&mut sim, 120 * 180),
            "settles within three minutes of simulated time"
        );
        let before = sim.balls.clone();
        sim.advance(1.0);
        assert_eq!(sim.balls, before, "a settled crowd stays put");
        in_bounds(&sim);
        // Anything that can set it moving again wakes it.
        sim.set_mass(MAX_MASS_MT / 2.0);
        assert!(sim.alive());
    }

    /// The heaviest setting on the largest crowd is violent but bounded: no marble outruns the
    /// speed cap or leaves the box.
    #[test]
    fn the_heaviest_crowd_stays_in_bounds_and_under_the_speed_cap() {
        let mut sim = Simulation::new(MAX_BALLS);
        sim.resize(Size::new(800.0, 600.0));
        sim.set_gravity(0.0);
        sim.set_mass(MAX_MASS_MT);
        sim.kick();
        for _ in 0..240 {
            sim.advance(STEP);
            in_bounds(&sim);
            assert!(sim.balls.iter().all(|b| b.speed() <= MAX_SPEED + 1e-6));
        }
    }

    #[test]
    fn shuffle_picks_values_inside_every_slider() {
        let mut sim = Simulation::new(1);
        for _ in 0..500 {
            let (count, gravity, mass, elasticity) = sim.shuffled();
            assert!((1.0..=MAX_BALLS as f64).contains(&count) && count.fract() == 0.0);
            assert!((0.0..=JUPITER).contains(&gravity));
            assert!((0.0..=MAX_MASS_MT).contains(&mass));
            assert!((0.0..=1.0).contains(&elasticity));
            let p = sim.random_point();
            assert!((0.0..=sim.size.width).contains(&p.x));
            assert!((0.0..=sim.size.height).contains(&p.y));
        }
    }

    #[test]
    fn draw_work_scales_with_ball_count_without_effects() {
        let mut sim = Simulation::new(1);
        let mut one = Draw::new();
        paint(&mut one, sim.size, &sim);
        sim.set_count(250);
        let mut many = Draw::new();
        paint(&mut many, sim.size, &sim);
        assert_eq!(many.ops().len() - one.ops().len(), 249 * 4);
    }

    // --- Collisions and elasticity ---------------------------------------------------------

    /// Two marbles facing each other at zero-G, no drag to speak of in one step.
    fn head_on(elasticity: f64) -> Simulation {
        let mut sim = Simulation::new(2);
        sim.resize(Size::new(800.0, 600.0));
        sim.set_gravity(0.0);
        sim.set_elasticity(elasticity);
        sim.balls[0] = Ball {
            vx: 300.0,
            ..Ball::at(380.0, 300.0)
        };
        sim.balls[1] = Ball {
            vx: -100.0,
            ..Ball::at(420.0, 300.0)
        };
        sim
    }

    #[test]
    fn marbles_never_overlap_once_the_solver_has_had_a_step() {
        let mut sim = Simulation::new(400);
        sim.resize(Size::new(800.0, 600.0));
        sim.set_gravity(EARTH);
        sim.kick_toward(Point::new(400.0, 300.0)); // everything thrown at one point
        for _ in 0..600 {
            sim.advance(STEP);
        }
        for _ in 0..600 {
            sim.advance(STEP);
            assert!(
                deepest_overlap(&sim) < sim.radius() * 0.5,
                "overlap {} of radius {}",
                deepest_overlap(&sim),
                sim.radius()
            );
        }
        assert!(settle(&mut sim, 120 * 30), "the crowd settles");
        assert!(deepest_overlap(&sim) < sim.radius() * 0.25);
        in_bounds(&sim);
    }

    #[test]
    fn an_elastic_head_on_collision_swaps_velocities_and_an_inelastic_one_merges_them() {
        let mut elastic = head_on(1.0);
        for _ in 0..12 {
            elastic.advance(STEP);
        }
        let (a, b) = (elastic.balls[0], elastic.balls[1]);
        assert!(a.vx < 0.0 && b.vx > 0.0, "they rebound: {a:?} {b:?}");
        // Momentum is conserved exactly; the drag of twelve steps is the only energy lost.
        let momentum = a.vx + b.vx;
        let expected = 200.0 * (1.0 - DRAG_PER_SEC * STEP).powi(12);
        assert!(
            (momentum - expected).abs() < 1e-6,
            "{momentum} vs {expected}"
        );
        // Equal masses at e = 1 swap speeds: the fast one leaves at the slow one's speed.
        assert!(
            (a.vx + 100.0 * (1.0 - DRAG_PER_SEC * STEP).powi(12)).abs() < 2.0,
            "{a:?}"
        );

        let mut dead = head_on(0.0);
        for _ in 0..12 {
            dead.advance(STEP);
        }
        let (a, b) = (dead.balls[0], dead.balls[1]);
        assert!(
            (a.vx - b.vx).abs() < 0.5,
            "a perfectly inelastic pair moves on together: {a:?} {b:?}"
        );
        assert!(a.vx > 0.0, "with the momentum they had between them");
    }

    #[test]
    fn restitution_follows_the_elasticity_slider() {
        let rebound = |e: f64| {
            let mut sim = Simulation::new(1);
            sim.resize(Size::new(400.0, 400.0));
            sim.set_gravity(0.0);
            sim.set_elasticity(e);
            sim.balls[0] = Ball {
                vy: 600.0,
                ..Ball::at(200.0, 380.0)
            };
            sim.unsettle();
            for _ in 0..6 {
                sim.advance(STEP);
            }
            -sim.balls[0].vy
        };
        let (dead, half, full) = (rebound(0.0), rebound(0.5), rebound(1.0));
        assert_eq!(dead, 0.0, "an inelastic marble does not leave the floor");
        assert!(half > 250.0 && half < 310.0, "half the speed back: {half}");
        assert!(full > 550.0, "nearly all of it back: {full}");
    }

    #[test]
    fn a_hard_marble_keeps_its_shape_and_a_soft_one_squashes_then_recovers() {
        let drop = |e: f64| {
            let mut sim = Simulation::new(1);
            sim.resize(Size::new(400.0, 400.0));
            sim.set_gravity(EARTH);
            sim.set_elasticity(e);
            sim.balls[0] = Ball::at(200.0, 40.0);
            sim.unsettle();
            let mut deepest = 0.0_f64;
            for _ in 0..240 {
                sim.advance(STEP);
                deepest = deepest.max(sim.balls[0].squash);
            }
            (deepest, sim)
        };
        let (hard, _) = drop(0.0);
        assert_eq!(hard, 0.0, "elasticity zero never deforms");
        let (medium, _) = drop(0.5);
        let (soft, soft_sim) = drop(1.0);
        assert!(medium > 0.05, "a landing shows: {medium}");
        assert!(soft > medium, "softer squashes further: {soft} vs {medium}");
        assert!(soft <= SQUASH_MAX);
        // The flattening is along the floor's normal, so the marble spreads sideways.
        let b = soft_sim.balls[0];
        assert!(b.nx.abs() < 1e-9 && b.ny.abs() == 1.0, "{b:?}");
        // Left alone it comes to rest with no more than its resting flatten.
        let mut sim = soft_sim;
        assert!(settle(&mut sim, 120 * 60));
        let b = sim.balls[0];
        assert!(
            b.squash <= REST_SQUASH + 1e-9 && b.squash_vel == 0.0,
            "{b:?}"
        );
    }

    #[test]
    fn a_squashed_marble_keeps_its_volume_and_a_round_one_draws_as_an_ellipse() {
        let mut ball = Ball::at(100.0, 100.0);
        assert!(matches!(
            Squash::of(&ball).ellipse(ball.x, ball.y, 0.0, 0.0, 12.0, 12.0),
            Shape::Ellipse(_)
        ));
        ball.squash = 0.36;
        ball.nx = 0.0;
        ball.ny = 1.0;
        let m = Squash::of(&ball).m.unwrap();
        // Along y (the hit) it is 0.64 as tall; across it 1/√0.64 = 1.25 as wide.
        assert!(
            (m[3] - 0.64).abs() < 1e-9 && (m[0] - 1.25).abs() < 1e-9,
            "{m:?}"
        );
        assert!(m[1].abs() < 1e-9 && m[2].abs() < 1e-9);
        assert!(matches!(
            Squash::of(&ball).ellipse(ball.x, ball.y, 0.0, 0.0, 12.0, 12.0),
            Shape::Path(_)
        ));
    }

    #[test]
    fn a_held_marble_follows_the_finger_pushes_the_others_and_flies_when_flung() {
        let mut sim = Simulation::new(2);
        sim.resize(Size::new(800.0, 600.0));
        sim.set_gravity(0.0);
        sim.balls[0] = Ball::at(200.0, 300.0);
        sim.balls[1] = Ball::at(400.0, 300.0);
        assert!(!sim.grab(Point::new(300.0, 100.0)), "nothing to grab there");
        assert!(
            sim.grab(Point::new(205.0, 304.0)),
            "within reach of the first"
        );
        assert_eq!(sim.held, Some(0));
        assert!(sim.alive(), "a held marble keeps the clock running");
        // Carry it into the other one at speed: the other is knocked away, the held one is not
        // moved off the finger.
        for step in 1..=20 {
            let x = 200.0 + step as f64 * 10.0;
            sim.drag_to(Point::new(x, 300.0), (1200.0, 0.0));
            sim.advance(STEP);
            assert_eq!(
                (sim.balls[0].x, sim.balls[0].y),
                (x, 300.0),
                "pinned to the finger"
            );
        }
        let other = sim.balls[1];
        assert!(
            other.vx > 1000.0,
            "struck by a marble doing 1200: {other:?}"
        );
        assert!(
            other.x > 400.0 + 2.0 * sim.radius(),
            "and pushed clear: {other:?}"
        );
        // Fling: let go moving up and to the left.
        sim.drag_to(Point::new(400.0, 300.0), (-500.0, -300.0));
        sim.release();
        assert_eq!(sim.held, None);
        assert_eq!((sim.balls[0].vx, sim.balls[0].vy), (-500.0, -300.0));
        sim.advance(STEP);
        assert!(
            sim.balls[0].x < 400.0 && sim.balls[0].y < 300.0,
            "{:?}",
            sim.balls[0]
        );
        // A flick: the tapped marble jumps, the other does not.
        sim.balls[0] = Ball::at(200.0, 588.0);
        sim.balls[1] = Ball::at(600.0, 588.0);
        assert!(sim.poke(Point::new(200.0, 588.0)));
        assert!(sim.balls[0].vy < -300.0 && sim.balls[1].vy == 0.0);
        assert!(!sim.poke(Point::new(400.0, 100.0)));
    }

    #[test]
    fn a_crowd_at_rest_on_the_floor_stays_at_rest() {
        let mut sim = Simulation::new(40);
        sim.set_gravity(EARTH);
        // Packed and still, but told something changed: it should find itself calm again
        // without any marble bouncing or creeping.
        let before = sim.balls.clone();
        sim.unsettle();
        assert!(settle(&mut sim, CALM_STEPS as usize + 2));
        for (a, b) in sim.balls.iter().zip(&before) {
            // A heap relaxes a little as the rows above press down and wedge the row below
            // apart, the way frictionless marbles do; nothing rolls anywhere.
            let r = sim.radius();
            assert!(
                (a.x - b.x).abs() < r / 4.0 && (a.y - b.y).abs() < r / 4.0,
                "{a:?} vs {b:?}"
            );
        }
    }
}
