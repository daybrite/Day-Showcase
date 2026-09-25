//! A canvas simulation driven by the window's native frame opportunities. The simulation,
//! drawing and frame ownership are deliberately separate so this is a reusable example.
use day::prelude::*;
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    ops::ControlFlow,
    rc::Rc,
};

const STEP: f64 = 1.0 / 120.0;
const FLOOR: f64 = 222.0;
const RADIUS: f64 = 23.0;

#[derive(Clone, Copy)]
struct Spark {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    age: f64,
    hue: f64,
}
struct Ball {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    time: f64,
    remainder: f64,
    trail: VecDeque<(f64, f64, f64, f64)>,
    sparks: Vec<Spark>,
    rings: Vec<(f64, f64, f64)>,
    steps: u64,
}
impl Default for Ball {
    fn default() -> Self {
        Self {
            x: 180.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            time: 0.0,
            remainder: 0.0,
            trail: VecDeque::new(),
            sparks: Vec::new(),
            rings: Vec::new(),
            steps: 0,
        }
    }
}
impl Ball {
    fn kick(&mut self, target: f64) {
        self.vy = 465.0;
        self.vx = ((target - self.x) * 1.5).clamp(-180.0, 180.0);
        self.impact();
    }
    fn moving(&self) -> bool {
        self.y > 0.0 || self.vy != 0.0
    }
    fn alive(&self) -> bool {
        self.moving() || !self.trail.is_empty() || !self.sparks.is_empty() || !self.rings.is_empty()
    }
    fn impact(&mut self) {
        self.rings.push((self.x, self.time, self.time * 100.0));
        if self.rings.len() > 8 {
            self.rings.remove(0);
        }
        for i in 0..12 {
            let angle = std::f64::consts::PI * (0.08 + i as f64 / 13.0);
            self.sparks.push(Spark {
                x: self.x,
                y: FLOOR - self.y - 3.0,
                vx: angle.cos() * 95.0,
                vy: -angle.sin() * 135.0,
                age: 0.0,
                hue: i as f64 * 30.0 + self.time * 100.0,
            });
        }
        if self.sparks.len() > 96 {
            self.sparks.drain(..self.sparks.len() - 96);
        }
    }
    fn advance(&mut self, elapsed: f64) {
        // The frame service supplies raw elapsed time. This demo explicitly chooses bounded
        // catch-up (100 ms) and fixed simulation steps, independent of display refresh rate.
        self.remainder += elapsed.clamp(0.0, 0.1);
        while self.remainder + 1e-10 >= STEP {
            self.remainder -= STEP;
            self.time += STEP;
            self.steps += 1;
            if self.moving() {
                self.vy -= 900.0 * STEP;
                self.y += self.vy * STEP;
                self.x += self.vx * STEP;
                if self.x < 35.0 || self.x > 325.0 {
                    self.x = self.x.clamp(35.0, 325.0);
                    self.vx = -self.vx * 0.8;
                }
                if self.y < 0.0 {
                    self.y = 0.0;
                    self.vy *= -0.58;
                    self.vx *= 0.72;
                    if self.vy < 38.0 {
                        self.vy = 0.0;
                        self.vx = 0.0;
                    }
                    self.impact();
                }
                if self.steps.is_multiple_of(2) {
                    self.trail.push_back((
                        self.x,
                        FLOOR - RADIUS - self.y,
                        self.time,
                        195.0 + self.time * 700.0,
                    ));
                }
            }
            self.trail.retain(|p| self.time - p.2 < 0.42);
            for s in &mut self.sparks {
                s.age += STEP;
                s.x += s.vx * STEP;
                s.y += s.vy * STEP;
                s.vy += 300.0 * STEP;
            }
            self.sparks.retain(|s| s.age < 0.65);
            self.rings.retain(|r| self.time - r.1 < 0.7);
        }
    }
}

struct Demo {
    ball: RefCell<Ball>,
    repaint: Trigger,
    status: Signal<String>,
    frames: Cell<u64>,
    readout: Signal<String>,
    width: Cell<f64>,
    paused: Cell<bool>,
    handle: RefCell<Option<day::frame::FrameHandle>>,
}
impl Demo {
    fn kick(&self, target: f64) {
        self.ball.borrow_mut().kick(target);
        self.paused.set(false);
        self.status.set(crate::res::str::frame_waiting().format());
        self.handle.borrow().as_ref().unwrap().resume();
        self.repaint.notify();
    }
    fn reset(&self) {
        self.handle.borrow().as_ref().unwrap().pause();
        *self.ball.borrow_mut() = Ball::default();
        self.paused.set(false);
        self.frames.set(0);
        self.readout
            .set(crate::res::str::frame_count(0, "—").format());
        self.status.set(crate::res::str::frame_resting().format());
        self.repaint.notify();
    }
}

pub(super) fn demo() -> impl Piece {
    let ui = Rc::new(Demo {
        ball: RefCell::new(Ball::default()),
        repaint: Trigger::new(),
        status: Signal::new(crate::res::str::frame_resting().format()),
        frames: Cell::new(0),
        readout: Signal::new(crate::res::str::frame_count(0, "—").format()),
        width: Cell::new(360.0),
        paused: Cell::new(false),
        handle: RefCell::new(None),
    });
    let weak = Rc::downgrade(&ui);
    let handle = day::frame::FrameClock::current().subscribe(move |frame| {
        let Some(ui) = weak.upgrade() else {
            return ControlFlow::Break(());
        };
        let alive = {
            let mut ball = ui.ball.borrow_mut();
            ball.advance(frame.delta.as_secs_f64());
            ball.alive()
        };
        let count = ui.frames.get() + 1;
        ui.frames.set(count);
        // Keep diagnostics inexpensive: the artwork updates every frame; the readout periodically.
        if count == 1 || count.is_multiple_of(15) || !alive {
            ui.readout.set(
                crate::res::str::frame_count(
                    count as i64,
                    format!("{:.1}", frame.delta.as_secs_f64() * 1000.0),
                )
                .format(),
            );
        }
        ui.status.set(
            if alive {
                crate::res::str::frame_running()
            } else {
                crate::res::str::frame_resting()
            }
            .format(),
        );
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
    let (draw, tap, bounce, pause, reset) =
        (ui.clone(), ui.clone(), ui.clone(), ui.clone(), ui.clone());
    let (status, readout) = (ui.status, ui.readout);
    column((
        label(crate::res::str::frame_title()).font(Font::Headline),
        label(crate::res::str::frame_hint()).font(Font::Callout),
        canvas(move |d, size| {
            draw.repaint.track();
            draw.width.set(size.width);
            paint(d, size, &draw.ball.borrow());
        })
        .on_tap_at(move |p| {
            let scale = (tap.width.get() / 360.0).min(1.15);
            let x = (p.x - (tap.width.get() - 360.0 * scale) / 2.0) / scale;
            tap.kick(x.clamp(35.0, 325.0));
        })
        .a11y(|a| {
            a.role(Role::Button)
                .label(crate::res::str::frame_hint().format())
        })
        .id("frame-ball")
        .height(290.0)
        .grow_w(),
        row((
            button(crate::res::str::frame_bounce())
                .action(move || bounce.kick(270.0))
                .id("frame-bounce"),
            button(crate::res::str::frame_pause())
                .action(move || {
                    let handle = pause.handle.borrow();
                    let handle = handle.as_ref().unwrap();
                    if pause.paused.get() {
                        pause.paused.set(false);
                        handle.resume();
                    } else if handle.is_active() {
                        pause.paused.set(true);
                        handle.pause();
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
            label(readout).font(Font::Caption).id("frame-readout"),
        )),
        label(crate::res::str::frame_explanation()).font(Font::Caption),
    ))
    .spacing(10.0)
}

fn paint(d: &mut Draw, size: Size, ball: &Ball) {
    d.fill(
        Shape::RoundedRect(Rect::new(0.0, 0.0, size.width, size.height), 22.0),
        LinearGradient::new(
            UnitPoint::TOP,
            UnitPoint::BOTTOM,
            vec![(0.0, Color::hex(0x101C39)), (1.0, Color::hex(0x242344))],
        ),
    );
    let scale = (size.width / 360.0).min(1.15);
    if scale <= 0.0 {
        return;
    }
    d.save();
    d.concat(Affine {
        a: scale,
        b: 0.0,
        c: 0.0,
        d: scale,
        tx: (size.width - 360.0 * scale) / 2.0,
        ty: (size.height - 260.0 * scale) / 2.0,
    });
    for i in 0..24 {
        let x = 20.0 + ((i * 79) % 320) as f64;
        let y = 16.0 + ((i * 47) % 166) as f64;
        d.fill(
            circle(x, y, if i % 3 == 0 { 1.4 } else { 0.8 }),
            Color::WHITE.with_alpha(0.2),
        );
    }
    for i in 0..5 {
        let r = Rect::new(
            35.0 + i as f64 * 4.0,
            FLOOR - 5.0 + i as f64 * 3.0,
            290.0 - i as f64 * 8.0,
            24.0,
        );
        d.stroke(
            Shape::Ellipse(r),
            Color::hsl(185.0 + i as f64 * 40.0, 0.8, 0.65).with_alpha(0.14),
            1.0,
        );
    }
    let shadow = 24.0 + ball.y * 0.12;
    d.fill(
        Shape::Ellipse(Rect::new(ball.x - shadow, FLOOR - 4.0, shadow * 2.0, 12.0)),
        Color::BLACK.with_alpha(0.27 - ball.y * 0.001),
    );
    for &(x, born, hue) in &ball.rings {
        let life = (ball.time - born) / 0.7;
        d.stroke(
            Shape::Ellipse(Rect::new(
                x - 15.0 - life * 62.0,
                FLOOR - 3.0 - life * 10.0,
                30.0 + life * 124.0,
                6.0 + life * 20.0,
            )),
            Color::hsl(hue, 0.85, 0.7).with_alpha((1.0 - life) * 0.65),
            2.0 * (1.0 - life) + 0.5,
        );
    }
    for &(x, y, born, hue) in &ball.trail {
        let life = 1.0 - (ball.time - born) / 0.42;
        d.fill(
            circle(x, y, RADIUS * life * 0.8),
            Color::hsl(hue, 0.85, 0.65).with_alpha(life * 0.24),
        );
    }
    for s in &ball.sparks {
        d.fill(
            circle(s.x, s.y, 2.5 * (1.0 - s.age / 0.65)),
            Color::hsl(s.hue, 0.85, 0.7).with_alpha(1.0 - s.age / 0.65),
        );
    }
    let hue = (195.0 + ball.time * 700.0) % 360.0;
    let cy = FLOOR - RADIUS - ball.y;
    let squash = (1.0 - ball.y / 12.0).clamp(0.0, 1.0) * (ball.vy.abs() / 300.0).min(1.0) * 0.16;
    let (rx, ry) = (RADIUS * (1.0 + squash), RADIUS * (1.0 - squash));
    let orb = Shape::Ellipse(Rect::new(ball.x - rx, cy - ry, rx * 2.0, ry * 2.0));
    d.fill(
        circle(ball.x, cy, RADIUS + 7.0),
        Color::hsl(hue, 0.9, 0.65).with_alpha(0.07),
    );
    d.fill(
        orb.clone(),
        RadialGradient::new(
            UnitPoint::new(0.3, 0.25),
            0.9,
            vec![
                (0.0, Color::hsl(hue - 25.0, 0.8, 0.9)),
                (0.32, Color::hsl(hue, 0.8, 0.64)),
                (0.72, Color::hsl(hue + 30.0, 0.8, 0.42)),
                (1.0, Color::hsl(hue + 45.0, 0.7, 0.22)),
            ],
        ),
    );
    d.stroke(orb, Color::WHITE.with_alpha(0.35), 0.8);
    d.fill(
        Shape::Ellipse(Rect::new(ball.x - 11.0, cy - 16.0, 12.0, 7.0)),
        Color::WHITE.with_alpha(0.48),
    );
    d.restore();
}
fn circle(x: f64, y: f64, r: f64) -> Shape {
    Shape::Ellipse(Rect::new(x - r, y - r, r * 2.0, r * 2.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn settles_and_releases_all_effects() {
        let mut ball = Ball::default();
        ball.kick(280.0);
        for _ in 0..720 {
            ball.advance(STEP);
        }
        assert!(!ball.alive());
        assert_eq!(ball.y, 0.0);
    }
    #[test]
    fn same_simulation_at_30_60_and_120_hz() {
        let simulate = |hz| {
            let mut ball = Ball::default();
            ball.kick(300.0);
            for _ in 0..hz {
                ball.advance(1.0 / hz as f64);
            }
            (ball.x, ball.y, ball.vy, ball.steps)
        };
        assert_eq!(simulate(30), simulate(120));
        assert_eq!(simulate(60), simulate(120));
    }
    #[test]
    fn long_stall_has_bounded_work() {
        let mut ball = Ball::default();
        ball.kick(100.0);
        ball.advance(120.0);
        assert_eq!(ball.steps, 12);
        assert!(ball.y > 0.0);
    }
    #[test]
    fn repeated_input_keeps_effect_storage_bounded() {
        let mut ball = Ball::default();
        for _ in 0..1000 {
            ball.kick(500.0);
            ball.advance(STEP);
        }
        assert!(ball.sparks.len() <= 96 && ball.rings.len() <= 8 && ball.trail.len() <= 26);
        assert!((35.0..=325.0).contains(&ball.x));
    }
}
