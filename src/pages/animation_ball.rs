//! Independent marbles driven by one native frame subscription. Fixed-step physics is
//! O(n); marbles collide with the canvas walls, not each other. Each marble is drawn as
//! vector geometry (no bitmap cache), so the count exercises the native canvas renderer.
use day::prelude::*;
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    ops::ControlFlow,
    rc::Rc,
};

const STEP: f64 = 1.0 / 120.0;
const RADIUS: f64 = 12.0;
const MAX_BALLS: usize = 250;
const FPS_WINDOW: f64 = 2.0;

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
}
impl Simulation {
    fn new(count: usize) -> Self {
        let mut sim = Self {
            balls: Vec::with_capacity(MAX_BALLS),
            size: Size::new(360.0, 290.0),
            remainder: 0.0,
            steps: 0,
            rng: 0x9E37_79B9_7F4A_7C15,
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
    }
    fn resize(&mut self, size: Size) {
        if size.width <= 0.0 || size.height <= 0.0 || self.size == size {
            return;
        }
        let old = self.size;
        self.size = size;
        let r = self.radius();
        for ball in &mut self.balls {
            ball.x = (ball.x * size.width / old.width).clamp(r, size.width - r);
            ball.y = if ball.moving() {
                (ball.y * size.height / old.height).clamp(r, size.height - r)
            } else {
                size.height - r
            };
        }
    }
    fn kick(&mut self) {
        for i in 0..self.balls.len() {
            let vx = self.random(-220.0, 220.0);
            let vy = self.random(-650.0, -420.0);
            self.balls[i].vx = vx;
            self.balls[i].vy = vy;
        }
        self.remainder = 0.0;
    }
    fn alive(&self) -> bool {
        self.balls.iter().any(Ball::moving)
    }
    fn advance(&mut self, elapsed: f64) {
        // Bound catch-up after stalls; neither physics speed nor work depends on refresh rate.
        self.remainder += elapsed.clamp(0.0, 0.1);
        let r = self.radius();
        let (right, bottom) = (self.size.width - r, self.size.height - r);
        while self.remainder + 1e-10 >= STEP {
            self.remainder = (self.remainder - STEP).max(0.0);
            self.steps += 1;
            for ball in &mut self.balls {
                if !ball.moving() {
                    continue;
                }
                ball.vy += 1100.0 * STEP;
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
                    if ball.vy.abs() < 32.0 {
                        ball.vy = 0.0;
                        ball.vx = 0.0;
                    }
                }
            }
        }
    }
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

pub(super) fn demo() -> impl Piece {
    let count = Signal::new(1.0_f64);
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
    let (draw, tap, bounce, pause, reset) =
        (ui.clone(), ui.clone(), ui.clone(), ui.clone(), ui.clone());
    let (status, fps) = (ui.status, ui.fps);
    column((
        label(crate::res::str::frame_title()).font(Font::Headline),
        label(crate::res::str::frame_hint()).font(Font::Callout),
        label(move || crate::res::str::frame_balls(count.get().round() as i64).format())
            .id("frame-ball-count"),
        slider(count)
            .range(1.0..=250.0)
            .step(1.0)
            .a11y(|a| a.label(crate::res::str::frame_balls_label().format()))
            .id("frame-ball-slider")
            .grow_w(),
        canvas(move |d, size| {
            draw.repaint.track();
            let mut sim = draw.simulation.borrow_mut();
            sim.resize(size);
            paint(d, size, &sim);
        })
        .on_tap(move || tap.kick())
        .a11y(|a| {
            a.role(Role::Button)
                .label(crate::res::str::frame_hint().format())
        })
        .id("frame-ball")
        .height(290.0)
        .grow_w(),
        row((
            button(crate::res::str::frame_bounce())
                .action(move || bounce.kick())
                .id("frame-bounce"),
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
    .id("frame-demo")
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
        sim.set_count(999);
        assert_eq!(sim.balls.len(), 250);
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
