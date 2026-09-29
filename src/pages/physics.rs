//! Physics: marbles under adjustable gravity, driven by one native frame subscription, filling the
//! whole page.
//!
//! Tap or click the canvas and every marble leaps along a ballistic arc toward that point, each
//! turned a few degrees off the line so a crowd scatters around it, then gravity takes over. The
//! Gravity slider runs from zero-G (the default) to Jupiter, in real units (m/s²). Fixed-step physics is O(n); marbles
//! collide with the canvas walls, not each other. Each marble is drawn as vector geometry (no
//! bitmap cache), so the count exercises the native canvas renderer.
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
/// While marbles pull on each other nothing comes to rest on its own, so the simulation settles
/// once every marble has stayed slower than this for [`CALM_STEPS`] steps in a row.
const CALM_SPEED: f64 = 5.0;
const CALM_STEPS: u32 = 60;

/// How far off the line to the tapped point a marble may be thrown, in radians either way (about
/// 7°): enough that a crowd keeps some entropy and scatters around the point, little enough that
/// every one of them plainly leaps toward it.
const AIM_JITTER: f64 = 0.12;
/// With no gravity nothing brings a marble to rest on the floor, so one slower than this simply
/// stops where it is, floating.
const FLOAT_REST_SPEED: f64 = 6.0;
/// Air drag, as the fraction of its speed a marble loses per second. Too slight to notice under
/// gravity; at zero-G it is what lets a drifting marble ease to a stop instead of creeping across
/// the box for minutes between wall hits (keeping the frame clock awake all the while).
const DRAG_PER_SEC: f64 = 0.3;

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
        crate::res::str::frame_fps(avg, max, min).format()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Ball {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}
impl Ball {
    fn moving(&self) -> bool {
        self.vx != 0.0 || self.vy != 0.0
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
    /// The field those masses make, rebuilt every step while there is any mass.
    mesh: Mesh,
    /// Each marble's acceleration toward the others this step, points per second squared.
    pull: Vec<(f64, f64)>,
    /// Consecutive steps every marble has been slower than [`CALM_SPEED`], and whether that has
    /// lasted long enough to call the simulation settled.
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
            mesh: Mesh::default(),
            pull: Vec::with_capacity(MAX_BALLS),
            calm: 0,
            settled: false,
        };
        sim.set_count(count);
        sim
    }
    fn radius(&self) -> f64 {
        RADIUS
            .min(self.size.width / 2.0)
            .min(self.size.height / 2.0)
    }
    fn random(&mut self, lo: f64, hi: f64) -> f64 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        lo + (hi - lo) * ((self.rng >> 11) as f64 / (1u64 << 53) as f64)
    }
    fn set_count(&mut self, count: usize) {
        let count = count.clamp(1, MAX_BALLS);
        self.unsettle();
        self.balls.truncate(count);
        let r = self.radius();
        while self.balls.len() < count {
            let x = self.random(r, self.size.width - r);
            self.balls.push(Ball {
                x,
                y: self.size.height - r,
                vx: 0.0,
                vy: 0.0,
            });
        }
    }
    fn reset(&mut self) {
        let count = self.balls.len();
        let r = self.radius();
        for (i, ball) in self.balls.iter_mut().enumerate() {
            *ball = Ball {
                x: r + (self.size.width - 2.0 * r) * (i as f64 + 0.5) / count as f64,
                y: self.size.height - r,
                vx: 0.0,
                vy: 0.0,
            };
        }
        self.remainder = 0.0;
        self.steps = 0;
        self.unsettle();
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
    fn alive(&self) -> bool {
        if self.mutual() {
            return !self.settled;
        }
        self.balls
            .iter()
            .any(|b| b.moving() || (self.gravity > 0.0 && self.floating(b)))
    }
    /// Random values for every control, for the Shuffle button: a ball count, planetary gravity
    /// in m/s² and a ball mass in megatonnes, each rounded to its slider's step.
    fn shuffled(&mut self) -> (f64, f64, f64) {
        let count = self.random(1.0, MAX_BALLS as f64).round();
        let gravity = (self.random(0.0, JUPITER) * 100.0).round() / 100.0;
        let mass = (self.random(0.0, MAX_MASS_MT) * 1000.0).round() / 1000.0;
        (count, gravity, mass)
    }
    /// A random point inside the canvas, for Shuffle's throw.
    fn random_point(&mut self) -> Point {
        let r = self.radius();
        let x = self.random(r, self.size.width - r);
        let y = self.random(r, self.size.height - r);
        Point::new(x, y)
    }
    fn advance(&mut self, elapsed: f64) {
        // Bound catch-up after stalls; neither physics speed nor work depends on refresh rate.
        self.remainder += elapsed.clamp(0.0, 0.1);
        let r = self.radius();
        let (right, bottom) = (self.size.width - r, self.size.height - r);
        let g = self.gravity;
        let mutual = self.mutual();
        if mutual && self.settled {
            self.remainder = 0.0;
            return;
        }
        while self.remainder + 1e-10 >= STEP {
            self.remainder = (self.remainder - STEP).max(0.0);
            self.steps += 1;
            if mutual {
                self.mesh
                    .pull(&self.balls, self.size, self.mass, &mut self.pull);
            }
            for (i, ball) in self.balls.iter_mut().enumerate() {
                // At rest on the floor, or floating at rest with nothing to pull it down. With
                // mutual attraction every marble is pulled by the others, so none is skipped.
                if !mutual && !ball.moving() && (ball.y >= bottom - 1e-9 || g == 0.0) {
                    continue;
                }
                if mutual {
                    let (ax, ay) = self.pull[i];
                    ball.vx += ax * STEP;
                    ball.vy += ay * STEP;
                }
                ball.vy += g * STEP;
                let speed = ball.vx.hypot(ball.vy);
                if speed > MAX_SPEED {
                    ball.vx *= MAX_SPEED / speed;
                    ball.vy *= MAX_SPEED / speed;
                }
                ball.vx *= 1.0 - DRAG_PER_SEC * STEP;
                ball.vy *= 1.0 - DRAG_PER_SEC * STEP;
                ball.x += ball.vx * STEP;
                ball.y += ball.vy * STEP;
                if ball.x < r {
                    ball.x = r;
                    ball.vx = ball.vx.abs() * 0.82;
                } else if ball.x > right {
                    ball.x = right;
                    ball.vx = -ball.vx.abs() * 0.82;
                }
                if ball.y < r {
                    ball.y = r;
                    ball.vy = ball.vy.abs() * 0.82;
                } else if ball.y > bottom {
                    ball.y = bottom;
                    ball.vy = -ball.vy.abs() * 0.62;
                    ball.vx *= 0.72;
                    // Too little bounce left to leave the floor: rest. Under a weak pull the
                    // threshold shrinks with it, so a marble on the Moon keeps its low hops.
                    if ball.vy.abs() < 32.0 * (g / (EARTH * POINTS_PER_METRE)).min(1.0) {
                        ball.vy = 0.0;
                        ball.vx = 0.0;
                    }
                }
                // Floating marbles come to rest one by one only without mutual attraction:
                // with it, a slow marble is one the others are just starting to pull.
                if !mutual && g == 0.0 && ball.vx.hypot(ball.vy) < FLOAT_REST_SPEED {
                    ball.vx = 0.0;
                    ball.vy = 0.0;
                }
            }
            if mutual {
                let calm = self.balls.iter().all(|b| b.vx.hypot(b.vy) < CALM_SPEED);
                self.calm = if calm { self.calm + 1 } else { 0 };
                if self.calm >= CALM_STEPS {
                    for ball in &mut self.balls {
                        ball.vx = 0.0;
                        ball.vy = 0.0;
                    }
                    self.settled = true;
                    self.remainder = 0.0;
                    return;
                }
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
}
impl Demo {
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
    fn shuffle(&self, count: Signal<f64>, gravity: Signal<f64>, mass: Signal<f64>) {
        let (n, g, m) = self.simulation.borrow_mut().shuffled();
        count.set(n);
        gravity.set(g);
        mass.set(m);
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
        Some(body) => crate::res::str::physics_gravity_named(body, value).format(),
        None => crate::res::str::physics_gravity(value).format(),
    }
}

/// Readout for the ball mass slider.
fn mass_text(megatonnes: f64) -> String {
    crate::res::str::physics_mass(day::format_decimal(megatonnes, 3)).format()
}

/// One row of the controls form: its title in a column as wide as the widest title, the slider,
/// and its value read out at the trailing end in a box as wide as `widest`. Both reservations
/// keep the three sliders aligned, and keep each one still as its digits change.
fn control(
    title: day::LocalizedText,
    widest_title: String,
    slider: impl Piece,
    readout: impl Fn() -> String + 'static,
    widest: String,
    readout_id: &'static str,
) -> impl Piece {
    row((
        label(title).reserving(widest_title),
        column((slider,)).grow_w(),
        // `widgets::numeric_readout`, trailing-aligned inside its reserved box.
        label(readout)
            .tabular()
            .align(TextAlign::Trailing)
            .id(readout_id)
            .reserving(widest),
    ))
    .spacing(12.0)
    .align(VAlign::Center)
}

pub(crate) fn physics_page() -> AnyPiece {
    let count = Signal::new(1.0_f64);
    // Each marble's mass in megatonnes. Massless to start: the marbles ignore each other until
    // the slider gives them weight.
    let mass = Signal::new(0.0_f64);
    // Zero-G to start: a tap sends the marbles drifting toward the point, and the slider adds
    // whatever pull the reader wants.
    let gravity = Signal::new(0.0);
    let ui = Rc::new(Demo {
        simulation: RefCell::new(Simulation::new(1)),
        repaint: Trigger::new(),
        status: Signal::new(crate::res::str::frame_resting().format()),
        stats: RefCell::new(FrameStats::default()),
        fps: Signal::new(FrameStats::default().text()),
        paused: Cell::new(false),
        handle: RefCell::new(None),
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
    // Gravity and mass reach the simulation live. A marble left floating by zero-G falls as soon
    // as there is gravity again, and marbles given mass start pulling on each other, so a
    // sleeping clock is woken for either (unless the demo is paused).
    let pull = ui.clone();
    Effect::new(move || {
        let (g, m) = (gravity.get(), mass.get());
        let wake = {
            let mut sim = pull.simulation.borrow_mut();
            sim.set_gravity(g);
            sim.set_mass(m);
            sim.alive()
        };
        let handle = pull.handle.borrow();
        if let Some(handle) = handle.as_ref()
            && wake
            && !pull.paused.get()
            && !handle.is_active()
        {
            pull.status.set(crate::res::str::frame_waiting().format());
            handle.resume();
        }
    });
    let (draw, tap, bounce, shuffle, pause, reset) = (
        ui.clone(),
        ui.clone(),
        ui.clone(),
        ui.clone(),
        ui.clone(),
        ui.clone(),
    );
    // The widest each readout can be in this locale, so the three sliders line up and hold
    // still: every named gravity reading, and the extremes of the other two.
    let widest_gravity = [0.0, MOON, MARS, EARTH, JUPITER, 88.88]
        .into_iter()
        .map(gravity_text)
        .max_by_key(|t| t.chars().count())
        .unwrap_or_default();
    // One width for all three readouts, so the sliders end together as well as start together.
    let widest_value = [
        day::format_decimal(8888.0, 0),
        mass_text(8.888),
        widest_gravity,
    ]
    .into_iter()
    .max_by_key(|t| t.chars().count())
    .unwrap_or_default();
    let widest_title = [
        crate::res::str::frame_balls_label(),
        crate::res::str::physics_mass_label(),
        crate::res::str::physics_gravity_label(),
    ]
    .into_iter()
    .map(|t| t.format())
    .max_by_key(|t| t.chars().count())
    .unwrap_or_default();
    let (status, fps) = (ui.status, ui.fps);
    // Not `widgets::page`: that scrolls, and the marbles' box takes every point the page has
    // left, which a scroll view never offers.
    column((
        crate::widgets::heading(crate::res::str::nav_physics(), "physics-title"),
        label(crate::res::str::frame_hint()).font(Font::Callout),
        canvas(move |d, size| {
            draw.repaint.track();
            let mut sim = draw.simulation.borrow_mut();
            sim.resize(size);
            paint(d, size, &sim);
        })
        // Every marble leaps toward the point tapped, then falls under the slider's gravity.
        .on_tap_at(move |at| tap.kick_toward(at))
        .a11y(|a| {
            a.role(Role::Button)
                .label(crate::res::str::frame_hint().format())
        })
        .id("frame-ball")
        .grow(),
        // The three controls as one form under the marbles, each slider with its value beside it.
        section((column((
            control(
                crate::res::str::frame_balls_label(),
                widest_title.clone(),
                slider(count)
                    .range(1.0..=MAX_BALLS as f64)
                    .step(1.0)
                    .a11y(|a| a.label(crate::res::str::frame_balls_label().format()))
                    .id("frame-ball-slider"),
                move || day::format_decimal(count.get().round(), 0),
                widest_value.clone(),
                "frame-ball-count",
            ),
            control(
                crate::res::str::physics_mass_label(),
                widest_title.clone(),
                slider(mass)
                    .range(0.0..=MAX_MASS_MT)
                    .step(0.001)
                    .a11y(|a| a.label(crate::res::str::physics_mass_label().format()))
                    .id("physics-mass-slider"),
                move || mass_text(mass.get()),
                widest_value.clone(),
                "physics-mass-value",
            ),
            control(
                crate::res::str::physics_gravity_label(),
                widest_title.clone(),
                slider(gravity)
                    .range(0.0..=JUPITER)
                    .step(0.01)
                    .a11y(|a| a.label(crate::res::str::physics_gravity_label().format()))
                    .id("physics-gravity-slider"),
                move || gravity_text(gravity.get()),
                widest_value,
                "physics-gravity-value",
            ),
        ))
        .spacing(8.0),)),
        row((
            button(crate::res::str::frame_bounce())
                .action(move || bounce.kick())
                .id("frame-bounce"),
            button(crate::res::str::frame_shuffle())
                .action(move || shuffle.shuffle(count, gravity, mass))
                .id("physics-shuffle"),
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
                .id("frame-pause"),
            button(crate::res::str::anim_reset_label())
                .action(move || reset.reset())
                .id("frame-reset"),
        ))
        .spacing(10.0),
        row((
            label(status).id("frame-status"),
            spacer(),
            label(fps)
                .tabular()
                .align(TextAlign::Trailing)
                .id("frame-fps")
                .reserving(crate::res::str::frame_fps("999.9", "999.9", "999.9").format())
                .font(Font::Caption),
        )),
        label(crate::res::str::frame_explanation()).font(Font::Caption),
    ))
    .spacing(10.0)
    .padding(20.0)
    .grow()
    .id("frame-demo")
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
        let orb = circle(ball.x, ball.y, r);
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
        d.stroke(orb, Color::hsl(hue, 0.5, 0.8).with_alpha(0.7), 0.7);
        d.fill(
            Shape::Ellipse(Rect::new(
                ball.x - r * 0.52,
                ball.y - r * 0.65,
                r * 0.62,
                r * 0.28,
            )),
            Color::WHITE.with_alpha(0.85),
        );
        d.fill(
            Shape::Ellipse(Rect::new(
                ball.x - r * 0.2,
                ball.y + r * 0.45,
                r * 0.65,
                r * 0.2,
            )),
            Color::hsl(hue, 0.8, 0.8).with_alpha(0.6),
        );
    }
    d.stroke(
        Shape::Rect(Rect::new(0.5, 0.5, size.width - 1.0, size.height - 1.0)),
        Color::hex(0x8293A0).with_alpha(0.4),
        1.0,
    );
}
fn circle(x: f64, y: f64, r: f64) -> Shape {
    Shape::Ellipse(Rect::new(x - r, y - r, r * 2.0, r * 2.0))
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
    #[test]
    fn all_250_settle_and_can_bounce_again() {
        let mut sim = Simulation::new(250);
        sim.kick();
        for _ in 0..1200 {
            sim.advance(STEP);
            in_bounds(&sim);
        }
        assert!(!sim.alive());
        assert!(
            sim.balls
                .iter()
                .all(|b| b.y == sim.size.height - sim.radius())
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
            sim.balls[0] = Ball { x, y, vx, vy };
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
                x: from.x,
                y: from.y,
                vx,
                vy,
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
        for _ in 0..20_000 {
            sim.advance(STEP);
            if !sim.alive() {
                break;
            }
        }
        assert!(
            !sim.alive(),
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
        for _ in 0..2400 {
            sim.advance(STEP);
        }
        assert!(!sim.alive());
        assert_eq!(sim.balls[0].y, sim.size.height - sim.radius());
    }

    #[test]
    fn stronger_gravity_brings_a_dropped_marble_down_sooner() {
        let fall_steps = |g: f64| {
            let mut sim = Simulation::new(1);
            sim.set_gravity(g);
            sim.balls[0] = Ball {
                x: 100.0,
                y: sim.radius(),
                vx: 0.0,
                vy: 0.0,
            };
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
        sim.balls[0] = Ball {
            x: 250.0,
            y: 300.0,
            vx: 0.0,
            vy: 0.0,
        };
        sim.balls[1] = Ball {
            x: 550.0,
            y: 300.0,
            vx: 0.0,
            vy: 0.0,
        };
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
        assert!(!still.alive());
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
        let alone = [Ball {
            x: 333.3,
            y: 123.4,
            vx: 0.0,
            vy: 0.0,
        }];
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
        let settled = (0..120 * 180).find(|_| {
            sim.advance(STEP);
            !sim.alive()
        });
        assert!(
            settled.is_some(),
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
            assert!(
                sim.balls
                    .iter()
                    .all(|b| b.vx.hypot(b.vy) <= MAX_SPEED + 1e-6)
            );
        }
    }

    #[test]
    fn shuffle_picks_values_inside_every_slider() {
        let mut sim = Simulation::new(1);
        for _ in 0..500 {
            let (count, gravity, mass) = sim.shuffled();
            assert!((1.0..=MAX_BALLS as f64).contains(&count) && count.fract() == 0.0);
            assert!((0.0..=JUPITER).contains(&gravity));
            assert!((0.0..=MAX_MASS_MT).contains(&mass));
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
}
