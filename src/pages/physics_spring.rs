// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The spring: the second pane of the Physics page.
//!
//! A disc hangs on a spring in the middle of a stage. Drag it and let go, or press Flick, and
//! the release mode decides what the velocity does: the spring pulls the disc back, the disc
//! glides until friction stops it (rebounding off the stage's edges), or the spring takes it to
//! the dock the glide would have ended nearest. Two sliders set the spring's stiffness and
//! damping while it moves.
//!
//! The disc is a view moved by `.transform(..)`. Its offset and its grip are `day::motion`
//! values: a release hands the drag's velocity to them and a spring or a decay carries it on, so
//! nothing on the page steps time itself. The canvas under the disc draws the range rings, the
//! docks, the coil and the disc's trail from the same values. The drag is taken on the stage: a
//! view that moves under the pointer reports the pointer in coordinates that move with it, while
//! the stage's stay put, so the stage hit-tests the press against the disc.

use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    rc::{Rc, Weak},
};

use day::motion::{Animated, Animation, Decay, Playing, Spring, SpringState};
use day::prelude::*;

use crate::gallery::{self, Tone};

/// The stage's height before its first layout; it then takes what the pane has left.
const STAGE_HEIGHT: f64 = 280.0;
/// The least the stage is given: room for the disc and a short pull either side of it.
const MIN_STAGE_HEIGHT: f64 = 220.0;
/// The disc's diameter.
const DISC: f64 = 68.0;
/// How far outside the disc a press still takes hold of it.
const GRAB_SLACK: f64 = 10.0;
/// The gap between an outer dock and the stage's edge.
const DOCK_GAP: f64 = 22.0;
/// The farthest an outer dock sits from the middle one, however wide the stage is.
const DOCK_REACH: f64 = 260.0;
/// The distance between two range rings.
const RING_GAP: f64 = 56.0;
/// How many of the disc's latest positions its trail keeps.
const TRAIL_POINTS: usize = 28;
/// How long the trail takes to fade once the disc stops, in milliseconds.
const TRAIL_FADE_MS: u32 = 450;
/// The speed the Flick button releases the disc with, in points per second.
const FLICK_SPEED: f64 = 1150.0;
/// The angle between one flick and the next, in radians: the golden angle, so successive flicks
/// never line up.
const FLICK_TURN: f64 = 2.399_963;
/// How a released disc glides: the velocity keeps this share of itself every millisecond.
const GLIDE: Decay = Decay { rate: 0.996 };
/// The spring that eases the disc's grip up and down.
const GRIP: Spring = Spring::new(260.0, 20.0);
/// The stiffness the pane opens with.
const DEFAULT_STIFFNESS: f64 = 150.0;
/// The damping the pane opens with: well under critical, so the first release overshoots.
const DEFAULT_DAMPING: f64 = 10.0;

/// What a release does with the disc's velocity.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Release {
    /// The spring pulls the disc back to the middle.
    Return,
    /// The disc glides until friction stops it.
    Glide,
    /// The spring takes the disc to the dock its glide would end nearest.
    Dock,
}

/// The pane's state: the values the views read, and what the gestures and buttons change.
struct Stage {
    /// The disc's horizontal offset from the stage's center. While the disc glides this is the
    /// unbounded path of the decay, which [`Stage::shown`] folds back between the edges.
    x: Animated,
    /// The disc's vertical offset from the stage's center, folded the same way.
    y: Animated,
    /// How firmly the disc is held: 0 at rest, 1 under the pointer.
    grip: Animated,
    /// The release mode.
    mode: Signal<Release>,
    /// The spring's stiffness.
    stiffness: Signal<f64>,
    /// The spring's damping.
    damping: Signal<f64>,
    /// The last release's speed, in points per second.
    release_speed: Signal<f64>,
    /// The dock the disc belongs to in Dock mode: 0 leading, 1 middle, 2 trailing.
    dock: Signal<usize>,
    /// Where the disc has just been, oldest first.
    trail: Signal<VecDeque<(f64, f64)>>,
    /// How bright the trail is: 1 while the disc moves, fading to 0 once it stops.
    trail_fade: Animated,
    /// The stage's size, as the canvas last drew it.
    size: Cell<Size>,
    /// The disc's offset when the pointer took it, while a drag is in progress.
    grab: Cell<Option<(f64, f64)>>,
    /// Whether the disc is gliding: its offset is then the decay's unbounded path.
    gliding: Cell<bool>,
    /// Where the spring in flight is taking the disc, so a slider change can respring it.
    heading: Cell<Option<(f64, f64)>>,
    /// The motion in flight.
    motion: RefCell<Option<Playing>>,
    /// How many times Flick has been pressed, which turns the next flick's direction.
    flicks: Cell<u32>,
}

/// How far an outer dock sits from the middle one on a stage of `size`.
fn dock_reach(size: Size) -> f64 {
    (size.width / 2.0 - DISC / 2.0 - DOCK_GAP).clamp(0.0, DOCK_REACH)
}

/// The horizontal offset of dock `index` on a stage of `size`.
fn dock_offset(index: usize, size: Size) -> f64 {
    (index as f64 - 1.0) * dock_reach(size)
}

/// The dock nearest the horizontal offset `x` on a stage of `size`.
fn nearest_dock(x: f64, size: Size) -> usize {
    let reach = dock_reach(size);
    if reach <= 0.0 || x.abs() < reach / 2.0 {
        1
    } else if x < 0.0 {
        0
    } else {
        2
    }
}

/// The damping ratio of a spring of mass 1: under 1 it overshoots, at 1 it is critically damped.
fn damping_ratio(stiffness: f64, damping: f64) -> f64 {
    damping / (2.0 * stiffness.max(f64::EPSILON).sqrt())
}

/// `path` folded back between `-limit` and `limit`, as a disc moving along it would be if it
/// rebounded off both edges: a free path reflected at every edge it crosses.
fn fold(path: f64, limit: f64) -> f64 {
    if limit <= 0.0 {
        return 0.0;
    }
    let along = (path + limit).rem_euclid(4.0 * limit);
    if along <= 2.0 * limit {
        along - limit
    } else {
        3.0 * limit - along
    }
}

impl Stage {
    /// How far the disc's center may be from the stage's on each axis with the disc inside it.
    fn limits(&self) -> (f64, f64) {
        let size = self.size.get();
        (
            ((size.width - DISC) / 2.0).max(0.0),
            ((size.height - DISC) / 2.0).max(0.0),
        )
    }

    /// Where the disc is drawn (tracked): its offset, folded between the edges while it glides.
    fn shown(&self) -> (f64, f64) {
        let (x, y) = (self.x.get(), self.y.get());
        if self.gliding.get() {
            let (limit_x, limit_y) = self.limits();
            (fold(x, limit_x), fold(y, limit_y))
        } else {
            (x, y)
        }
    }

    /// Where the disc is now, without subscribing.
    fn position(&self) -> (f64, f64) {
        day::reactive::untrack(|| self.shown())
    }

    /// Ends a glide where the disc is drawn, so the offset is a place on the stage again.
    fn settle_glide(&self) {
        if !self.gliding.replace(false) {
            return;
        }
        if let Some(motion) = self.motion.borrow_mut().take() {
            motion.stop();
        }
        let (limit_x, limit_y) = self.limits();
        self.x.set(fold(self.x.get_untracked(), limit_x));
        self.y.set(fold(self.y.get_untracked(), limit_y));
    }

    /// Moves the disc, keeping it on the stage, at rest.
    fn place(&self, x: f64, y: f64) {
        let (limit_x, limit_y) = self.limits();
        self.x.set(x.clamp(-limit_x, limit_x));
        self.y.set(y.clamp(-limit_y, limit_y));
    }

    /// Whether `at`, a point of the stage, is on the disc.
    fn on_disc(&self, at: Point) -> bool {
        let size = self.size.get();
        let (x, y) = self.position();
        let (cx, cy) = (size.width / 2.0 + x, size.height / 2.0 + y);
        (at.x - cx).hypot(at.y - cy) <= DISC / 2.0 + GRAB_SLACK
    }

    /// The spring as the sliders have it now.
    fn spring(&self) -> Spring {
        Spring::new(self.stiffness.get_untracked(), self.damping.get_untracked())
    }

    /// Plays `animation` as the disc's motion, in place of the one before.
    fn play(&self, animation: &Animation) -> Playing {
        if let Some(motion) = self.motion.borrow_mut().take() {
            motion.stop();
        }
        let playing = animation.start();
        *self.motion.borrow_mut() = Some(playing.clone());
        playing
    }

    /// Springs the disc to an offset from the velocity it has. Under reduced motion the spring
    /// lands at once.
    fn spring_to(&self, x: f64, y: f64) {
        self.heading.set(Some((x, y)));
        let spring = self.spring();
        self.play(&Animation::parallel([
            Animation::spring(self.x, x, spring),
            Animation::spring(self.y, y, spring),
        ]));
    }

    /// The sliders moved: a spring in flight carries on toward its target on the new spring,
    /// from the velocity it has.
    fn respring(&self) {
        let moving = day::reactive::untrack(|| self.x.is_animating() || self.y.is_animating());
        if let (Some((x, y)), true, false) = (self.heading.get(), moving, self.gliding.get()) {
            self.spring_to(x, y);
        }
    }

    /// Tightens or loosens the grip.
    fn grip_to(&self, target: f64) {
        Animation::spring(self.grip, target, GRIP).start();
    }

    /// One event of a drag on the stage.
    fn drag(self: &Rc<Self>, drag: Drag) {
        match drag.phase {
            DragPhase::Began => {
                self.settle_glide();
                // The press point, which `Drag::start` gives on every toolkit: `Began` can arrive
                // after the recognizer has already watched the pointer move.
                if !self.on_disc(drag.start()) {
                    self.grab.set(None);
                    return;
                }
                self.heading.set(None);
                self.grab.set(Some(self.position()));
                self.follow(drag.translation);
                self.grip_to(1.0);
            }
            DragPhase::Changed => self.follow(drag.translation),
            DragPhase::Ended => {
                self.follow(drag.translation);
                if self.grab.take().is_some() {
                    self.grip_to(0.0);
                    self.launch(drag.velocity.x, drag.velocity.y);
                }
            }
        }
    }

    /// Puts the dragged disc where the pointer has carried it.
    fn follow(&self, translation: Point) {
        if let Some((x, y)) = self.grab.get() {
            self.place(x + translation.x, y + translation.y);
        }
    }

    /// Lets the disc go at a velocity: reads its speed and starts the mode's motion with it.
    fn launch(self: &Rc<Self>, vx: f64, vy: f64) {
        self.settle_glide();
        self.release_speed.set(vx.hypot(vy).round());
        self.x.set_velocity(vx);
        self.y.set_velocity(vy);
        let (x, _) = self.position();
        match self.mode.get_untracked() {
            Release::Return => self.spring_to(0.0, 0.0),
            Release::Glide => {
                self.heading.set(None);
                self.gliding.set(true);
                let glide = self.play(&Animation::parallel([
                    Animation::decay(self.x, GLIDE),
                    Animation::decay(self.y, GLIDE),
                ]));
                let stage: Weak<Stage> = Rc::downgrade(self);
                glide.on_end(move |_| {
                    if let Some(stage) = stage.upgrade() {
                        stage.settle_glide();
                    }
                });
            }
            Release::Dock => {
                let size = self.size.get();
                let rest = GLIDE.rest(SpringState {
                    value: x,
                    velocity: vx,
                });
                let dock = nearest_dock(rest, size);
                self.dock.set(dock);
                self.spring_to(dock_offset(dock, size), 0.0);
            }
        }
    }

    /// The Flick button: releases the disc from where it is at [`FLICK_SPEED`], each press
    /// turned [`FLICK_TURN`] on from the last.
    fn flick(self: &Rc<Self>) {
        if self.grab.get().is_some() {
            return;
        }
        let turn = f64::from(self.flicks.get()) * FLICK_TURN;
        self.flicks.set(self.flicks.get().wrapping_add(1));
        let (sin, cos) = turn.sin_cos();
        let (x, y) = self.position();
        // From rest, so the flick's speed is the whole of the release.
        self.place(x, y);
        self.launch(FLICK_SPEED * cos, FLICK_SPEED * sin);
    }

    /// The Center button: clears the reading and sends the disc to the middle from rest. It
    /// ends a drag in progress.
    fn center(&self) {
        self.grab.set(None);
        self.settle_glide();
        self.release_speed.set(0.0);
        self.dock.set(1);
        let (x, y) = self.position();
        self.place(x, y);
        self.grip_to(0.0);
        self.spring_to(0.0, 0.0);
    }

    /// The release mode changed. A disc that is not in hand goes where the new mode keeps it:
    /// the middle, or the dock it is nearest. A glide leaves it where it is.
    fn mode_changed(&self, mode: Release) {
        if self.grab.get().is_some() {
            return;
        }
        match mode {
            Release::Return => {
                self.settle_glide();
                self.spring_to(0.0, 0.0);
            }
            Release::Glide => {}
            Release::Dock => {
                self.settle_glide();
                let size = self.size.get();
                let dock = nearest_dock(self.position().0, size);
                self.dock.set(dock);
                self.spring_to(dock_offset(dock, size), 0.0);
            }
        }
    }

    /// Adds where the disc is drawn now to its trail, which then shows at full strength and
    /// fades out after the last move.
    fn mark_trail(&self, at: (f64, f64)) {
        let stale = self.trail_fade.get_untracked() < 0.01;
        self.trail.update(|trail| {
            if stale {
                trail.clear();
            }
            trail.push_back(at);
            while trail.len() > TRAIL_POINTS {
                trail.pop_front();
            }
        });
        self.trail_fade.set(1.0);
        self.trail_fade
            .animate_to(0.0, AnimSpec::ease_in(TRAIL_FADE_MS));
    }
}

/// A coil from `from` to `to`: a straight lead at each end and a zigzag between them.
fn coil(from: Point, to: Point) -> Option<Shape> {
    /// How many times the coil crosses its own axis.
    const CROSSINGS: usize = 14;
    /// How far the coil swings either side of its axis.
    const SWING: f64 = 6.0;
    let (dx, dy) = (to.x - from.x, to.y - from.y);
    let length = dx.hypot(dy);
    if length < 8.0 {
        return None;
    }
    let (ux, uy) = (dx / length, dy / length);
    let lead = (length * 0.12).min(14.0);
    let run = length - 2.0 * lead;
    let mut path = PathBuilder::new()
        .move_to(from)
        .line_to(Point::new(from.x + ux * lead, from.y + uy * lead));
    for crossing in 0..CROSSINGS {
        let along = lead + run * (crossing as f64 + 0.5) / CROSSINGS as f64;
        let side = if crossing % 2 == 0 { SWING } else { -SWING };
        path = path.line_to(Point::new(
            from.x + ux * along - uy * side,
            from.y + uy * along + ux * side,
        ));
    }
    Some(
        path.line_to(Point::new(to.x - ux * lead, to.y - uy * lead))
            .line_to(to)
            .build(),
    )
}

/// The canvas under the disc: range rings around the middle, the docks in Dock mode, the
/// disc's trail, and the coil from where the spring is anchored out to the disc.
fn backdrop(draw: &mut Draw, size: Size, stage: &Stage) {
    stage.size.set(size);
    if size.width <= 0.0 || size.height <= 0.0 {
        return;
    }
    let bounds = Rect::new(0.0, 0.0, size.width, size.height);
    draw.fill(Shape::RoundedRect(bounds, 12.0), gallery::INSET);
    let center = Point::new(size.width / 2.0, size.height / 2.0);
    let accent = Tone::Accent.color();
    let mode = stage.mode.get();
    let (x, y) = stage.shown();
    let disc = Point::new(center.x + x, center.y + y);

    let mut radius = RING_GAP;
    // Whole rings only: the last one is the largest the stage holds.
    while radius < center.x.min(center.y) {
        draw.stroke(
            Shape::Ellipse(Rect::new(
                center.x - radius,
                center.y - radius,
                radius * 2.0,
                radius * 2.0,
            )),
            SystemColor::Separator,
            1.0,
        );
        radius += RING_GAP;
    }

    let active = stage.dock.get();
    if mode == Release::Dock {
        let side = DISC + 12.0;
        for index in 0..3 {
            let at = center.x + dock_offset(index, size);
            draw.stroke_styled(
                Shape::Ellipse(Rect::new(
                    at - side / 2.0,
                    center.y - side / 2.0,
                    side,
                    side,
                )),
                if index == active {
                    Ink::Fixed(accent.with_alpha(0.8))
                } else {
                    Ink::system(SystemColor::TertiaryText)
                },
                StrokeStyle::dashed(1.5, vec![5.0, 4.0]),
            );
        }
    }

    let fade = stage.trail_fade.get();
    if fade > 0.0 {
        stage.trail.with(|trail| {
            let count = trail.len().max(1) as f64;
            for (index, (older, newer)) in trail.iter().zip(trail.iter().skip(1)).enumerate() {
                let fresh = (index + 1) as f64 / count;
                draw.stroke(
                    Shape::Line(
                        Point::new(center.x + older.0, center.y + older.1),
                        Point::new(center.x + newer.0, center.y + newer.1),
                    ),
                    accent.with_alpha(0.5 * fresh * fade),
                    3.0,
                );
            }
        });
    }

    // A glide has no spring on it, so nothing is drawn between the middle and the disc.
    if mode != Release::Glide {
        let anchor = match mode {
            Release::Dock => Point::new(center.x + dock_offset(active, size), center.y),
            Release::Return | Release::Glide => center,
        };
        if let Some(coil) = coil(anchor, disc) {
            draw.stroke(coil, accent.with_alpha(0.7), 1.5);
        }
        draw.fill(
            Shape::Ellipse(Rect::new(anchor.x - 3.5, anchor.y - 3.5, 7.0, 7.0)),
            accent,
        );
    }
}

/// The stage: the canvas, the hint and the disc, with the drag on the stage itself.
fn stage_piece(stage: &Rc<Stage>) -> impl Piece + use<> {
    let grip = stage.grip;
    let paint = stage.clone();
    let moved = stage.clone();
    let floor = canvas(move |draw, size| backdrop(draw, size, &paint))
        .min_height(MIN_STAGE_HEIGHT)
        .grow(Dimension::Both)
        // The hint fades as the disc is taken, and returns when it is let go.
        .overlay_aligned(
            Alignment::Top,
            label(crate::res::str::physics_spring_hint())
                .font(Font::Footnote)
                .secondary()
                .single_line()
                .opacity(move || 1.0 - grip.get().clamp(0.0, 1.0))
                .padding(Space::M),
        );
    let disc = zstack((vector(crate::res::vectors::nav_physics)
        .tint(Color::WHITE)
        .frame(32.0, 32.0),))
    .frame(DISC, DISC)
    .background(Tone::Accent.color())
    .corner_radius(Corner::CAPSULE)
    .transform(move || {
        // The disc rolls as it travels sideways, and swells a little in the hand.
        let grown = 1.0 + 0.14 * grip.get().clamp(0.0, 1.5);
        let (x, y) = moved.shown();
        Transform {
            tx: x,
            ty: y,
            sx: grown,
            sy: grown,
            rotation: Angle::turns(x / (std::f64::consts::PI * DISC)),
            ..Transform::IDENTITY
        }
    })
    .a11y(|a| a.label(crate::res::str::physics_spring_puck_label()))
    .id(day::id!(physics_spring_puck));
    let drag = stage.clone();
    // The rounded container clips a spring's overshoot to the stage, and is the one view whose
    // coordinates the drag is in.
    zstack((floor, disc))
        .corner_radius(12.0)
        .on_drag(move |event| drag.drag(event))
        .a11y(|a| a.label(crate::res::str::physics_spring_stage_label()))
        .id(day::id!(physics_spring_stage))
        .grow(Dimension::Both)
}

/// A figure with its name above it, with `id` on the figure so a script can read it.
fn reading(
    name: day::LocalizedText,
    value: impl Fn() -> String + 'static,
    id: day::ElementId,
) -> impl Piece {
    column((
        label(name).font(Font::Caption).secondary().single_line(),
        label(value)
            .font(Font::Callout)
            .tabular()
            .single_line()
            .id(id),
    ))
    .spacing(2.0)
    .align(HAlign::Leading)
    .grow(Dimension::Width)
}

/// The readings: the last release's speed, how far the disc is from the middle, and the
/// spring's damping ratio.
fn readings(stage: &Rc<Stage>) -> impl Piece + use<> {
    let (shown, speed, stiffness, damping) = (
        stage.clone(),
        stage.release_speed,
        stage.stiffness,
        stage.damping,
    );
    row((
        reading(
            crate::res::str::physics_spring_stat_speed(),
            move || {
                crate::res::str::physics_spring_speed(day::format_decimal(speed.get(), 0)).format()
            },
            day::id!(physics_spring_speed),
        ),
        reading(
            crate::res::str::physics_spring_stat_offset(),
            move || {
                let (x, y) = shown.shown();
                crate::res::str::physics_spring_points(day::format_decimal(x.hypot(y).round(), 0))
                    .format()
            },
            day::id!(physics_spring_offset),
        ),
        reading(
            crate::res::str::physics_spring_stat_ratio(),
            move || day::format_decimal(damping_ratio(stiffness.get(), damping.get()), 2),
            day::id!(physics_spring_ratio),
        ),
    ))
    .spacing(Space::M)
    .grow(Dimension::Width)
}

/// The spring pane: the stage with the release picker, the spring's two sliders, the buttons
/// and the readings. With room the controls stand beside the stage; otherwise under it. The
/// stage takes every point the controls leave.
pub(crate) fn spring_pane() -> AnyPiece {
    let stage = Rc::new(Stage {
        x: Animated::new(0.0),
        y: Animated::new(0.0),
        grip: Animated::new(0.0),
        mode: Signal::new(Release::Return),
        stiffness: Signal::new(DEFAULT_STIFFNESS),
        damping: Signal::new(DEFAULT_DAMPING),
        release_speed: Signal::new(0.0),
        dock: Signal::new(1),
        trail: Signal::new(VecDeque::new()),
        trail_fade: Animated::new(0.0),
        size: Cell::new(Size::new(0.0, STAGE_HEIGHT)),
        grab: Cell::new(None),
        gliding: Cell::new(false),
        heading: Cell::new(None),
        motion: RefCell::new(None),
        flicks: Cell::new(0),
    });

    let mode = stage.mode;
    let follow = stage.clone();
    Effect::new(move || follow.mode_changed(mode.get()));
    let (stiffness, damping) = (stage.stiffness, stage.damping);
    let retune = stage.clone();
    Effect::new(move || {
        // Subscribe to both sliders; the values are read again when the spring is built.
        let _ = (stiffness.get(), damping.get());
        retune.respring();
    });
    let trace = stage.clone();
    Effect::new(move || {
        let at = trace.shown();
        day::reactive::untrack(|| trace.mark_trail(at));
    });

    // A glide has no spring on it, so the spring's sliders have nothing to change.
    let sprung = move || mode.get() != Release::Glide;
    let widest_title = [
        crate::res::str::physics_spring_stiffness(),
        crate::res::str::physics_spring_damping(),
    ]
    .into_iter()
    .map(|title| title.format())
    .max_by_key(|title| title.chars().count())
    .unwrap_or_default();
    let widest_value = day::format_decimal(888.8, 1);
    let arrangement = super::physics::Arrangement::current();
    let beside = arrangement == super::physics::Arrangement::Beside;
    // Only a narrow window's full-width column has room for a title, a slider and a value on
    // one line.
    let wide = arrangement == super::physics::Arrangement::Over;
    let (flick, center) = (stage.clone(), stage.clone());
    let controls = column((
        picker(
            stage.mode,
            [
                (
                    Release::Return,
                    crate::res::str::physics_spring_mode_return(),
                ),
                (Release::Glide, crate::res::str::physics_spring_mode_glide()),
                (Release::Dock, crate::res::str::physics_spring_mode_dock()),
            ],
        )
        .segmented()
        .a11y(|a| a.label(crate::res::str::physics_spring_mode_label()))
        .id(day::id!(physics_spring_mode)),
        section((column((
            super::physics::control(
                wide,
                crate::res::str::physics_spring_stiffness(),
                widest_title.clone(),
                slider(stiffness)
                    .range(30.0..=500.0)
                    .step(5.0)
                    .enabled(sprung)
                    .a11y(|a| a.label(crate::res::str::physics_spring_stiffness()))
                    .id(day::id!(physics_spring_stiffness)),
                move || day::format_decimal(stiffness.get(), 0),
                widest_value.clone(),
                day::id!(physics_spring_stiffness_value),
            ),
            super::physics::control(
                wide,
                crate::res::str::physics_spring_damping(),
                widest_title,
                slider(damping)
                    .range(2.0..=40.0)
                    .step(0.5)
                    .enabled(sprung)
                    .a11y(|a| a.label(crate::res::str::physics_spring_damping()))
                    .id(day::id!(physics_spring_damping)),
                move || day::format_decimal(damping.get(), 1),
                widest_value,
                day::id!(physics_spring_damping_value),
            ),
        ))
        .spacing(Space::S),)),
        row((
            // A drag releases the disc too, but a gesture is never the only way.
            button(crate::res::str::physics_spring_flick())
                .action(move || flick.flick())
                .id(day::id!(physics_spring_flick)),
            button(crate::res::str::physics_spring_reset())
                .action(move || center.center())
                .id(day::id!(physics_spring_reset)),
        ))
        .spacing(10.0),
        readings(&stage),
    ))
    .spacing(Space::M)
    .align(HAlign::Leading);
    let pane = if beside {
        row((
            stage_piece(&stage),
            controls.width(super::physics::SIDE_CONTROLS_WIDTH),
        ))
        .spacing(Space::L)
        .align(VAlign::Top)
        .grow(Dimension::Both)
        .any()
    } else {
        column((stage_piece(&stage), controls.grow(Dimension::Width)))
            .spacing(Space::M)
            .align(HAlign::Leading)
            .grow(Dimension::Both)
            .any()
    };
    pane.card().id(day::id!(physics_spring_card)).any()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn docks_split_the_stage_into_thirds_around_the_middle() {
        let size = Size::new(600.0, 300.0);
        let reach = dock_reach(size);
        assert_eq!(reach, 600.0 / 2.0 - DISC / 2.0 - DOCK_GAP);
        assert_eq!(dock_offset(0, size), -reach);
        assert_eq!(dock_offset(1, size), 0.0);
        assert_eq!(dock_offset(2, size), reach);
        assert_eq!(nearest_dock(0.0, size), 1);
        assert_eq!(nearest_dock(reach / 2.0 - 1.0, size), 1);
        assert_eq!(nearest_dock(reach / 2.0 + 1.0, size), 2);
        assert_eq!(nearest_dock(-reach * 3.0, size), 0);
        // A stage too narrow for outer docks has the middle one only.
        assert_eq!(nearest_dock(40.0, Size::new(DISC, 300.0)), 1);
        // A wide stage keeps the docks within reach.
        assert_eq!(dock_reach(Size::new(4000.0, 300.0)), DOCK_REACH);
    }

    #[test]
    fn a_glide_folds_back_off_both_edges() {
        assert_eq!(fold(30.0, 100.0), 30.0);
        assert_eq!(fold(-30.0, 100.0), -30.0);
        // 20 past the trailing edge is 20 back from it.
        assert_eq!(fold(120.0, 100.0), 80.0);
        // Across the stage and off the leading edge too.
        assert_eq!(fold(320.0, 100.0), -80.0);
        assert_eq!(fold(-120.0, 100.0), -80.0);
        assert_eq!(fold(400.0, 100.0), 0.0);
        assert_eq!(fold(50.0, 0.0), 0.0);
    }

    #[test]
    fn damping_ratio_is_one_at_critical_damping() {
        assert!((damping_ratio(100.0, 20.0) - 1.0).abs() < 1e-12);
        assert!(damping_ratio(DEFAULT_STIFFNESS, DEFAULT_DAMPING) < 0.5);
    }

    #[test]
    fn a_coil_joins_its_ends_and_needs_some_length() {
        assert!(coil(Point::new(10.0, 10.0), Point::new(12.0, 11.0)).is_none());
        assert!(coil(Point::new(10.0, 10.0), Point::new(120.0, 60.0)).is_some());
    }
}
