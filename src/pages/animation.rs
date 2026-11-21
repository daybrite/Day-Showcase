// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The Animation page: motion the platform runs for Day, and motion Day steps itself.
//!
//! "Platform animator", "Reflow" and "Implicit animation" hand their changes to the platform's
//! animator (DESIGN.md §8.4): the first two write their signals inside `with_animation(..)`, the
//! third carries `.animation(..)` on the view that changes.
//!
//! Every other card animates with `day::motion`: `Animated` values that a view's modifiers or a
//! canvas read, moved by `Animation` steps and compositions on the display's frame clock.
//!
//! Under reduced motion every finite animation lands on its end state, and the repeating one
//! does not start.

use std::{cell::RefCell, rc::Rc};

use day::motion::{Animated, Animation, Ease, Easing, Interpolation, Outcome, Playing, Spring};
use day::prelude::*;

use crate::gallery::{self, Tone, card, card_page};
use crate::tls::ANIMATION_SEED as SEED;

/// The animation a card played last, kept so the next one can stop it first: two animations
/// of the same values resolve by ownership, but the old one's steps on other values would run
/// on.
#[derive(Clone, Default)]
struct Slot(Rc<RefCell<Option<Playing>>>);

impl Slot {
    /// Stops the animation played last and starts `animation` in its place.
    fn play(&self, animation: &Animation) -> Playing {
        self.stop();
        let playing = animation.start();
        *self.0.borrow_mut() = Some(playing.clone());
        playing
    }

    /// The animation played last, if any.
    fn last(&self) -> Option<Playing> {
        self.0.borrow().clone()
    }

    /// Stops the animation played last where it stands. Answers whether it was running.
    fn stop(&self) -> bool {
        self.last().is_some_and(|playing| playing.stop())
    }

    /// Stops the animation played last and puts its values back where it found them.
    fn reset(&self) {
        if let Some(playing) = self.last() {
            playing.reset();
        }
    }
}

// ---------------------------------------------------------------------------
// Shared drawing and layout
// ---------------------------------------------------------------------------

/// A localized name, resolved when it is shown.
type Name = fn() -> day::LocalizedText;

/// A circle of radius `radius` around `center`.
fn disc(center: Point, radius: f64) -> Shape {
    Shape::Ellipse(Rect::new(
        center.x - radius,
        center.y - radius,
        radius * 2.0,
        radius * 2.0,
    ))
}

/// Records the width a canvas was laid out at, for the views that move across it. Called from
/// the canvas's draw function, which runs again whenever its frame changes.
fn note_width(width: Signal<f64>, size: Size) {
    if (width.get_untracked() - size.width).abs() > 0.5 {
        width.set(size.width);
    }
}

/// One line of a card's form: a name in the first column, its control in the second.
fn field(name: day::LocalizedText, control: impl Piece) -> impl Piece {
    row((
        label(name)
            .font(Font::Footnote)
            .secondary()
            .single_line()
            .width(96.0),
        control.grow(Dimension::Width),
    ))
    .spacing(Space::S)
    .align(VAlign::Center)
    .grow(Dimension::Width)
}

/// A slider beside its value, which keeps the room of `widest` so the slider holds its width.
fn slider_with_readout(
    control: impl Piece,
    text: impl Fn() -> String + 'static,
    widest: &'static str,
    id: ElementId,
) -> impl Piece {
    row((
        control.grow(Dimension::Width),
        crate::widgets::numeric_readout(text, widest, id),
    ))
    .spacing(Space::S)
    .align(VAlign::Center)
}

/// A uniform random `f64` in `lo..=hi`: a tiny xorshift seeded once from the clock.
fn rand_range(lo: f64, hi: f64) -> f64 {
    let x = SEED.with(|s| {
        let mut x = s.get();
        if x == 0 {
            // A fixed odd seed is fine when the clock has nothing to offer: the demos need
            // variety within a session, not across runs.
            x = (day::now_epoch_millis() as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        }
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        s.set(x);
        x
    });
    let unit = (x >> 11) as f64 / (1u64 << 53) as f64;
    lo + (hi - lo) * unit
}

// ---------------------------------------------------------------------------
// Easing curves
// ---------------------------------------------------------------------------

/// How long one run across the curves takes, in milliseconds.
const CURVES_MS: u32 = 1500;
/// The progress the curves hold before their first run: far enough in to tell them apart.
const CURVES_REST: f64 = 0.62;

/// One curve of each [`Easing`] variant, in the order the variants are declared.
fn curves() -> [(Name, Easing); 9] {
    [
        (crate::res::str::animation_curve_linear, Easing::Linear),
        (
            crate::res::str::animation_curve_bezier,
            Easing::Bezier(0.7, 0.0, 0.3, 1.0),
        ),
        (
            crate::res::str::animation_curve_power,
            Easing::Power(Ease::Out, 4.0),
        ),
        (
            crate::res::str::animation_curve_sine,
            Easing::Sine(Ease::InOut),
        ),
        (
            crate::res::str::animation_curve_circle,
            Easing::Circle(Ease::In),
        ),
        (
            crate::res::str::animation_curve_exponential,
            Easing::Exponential(Ease::Out),
        ),
        (
            crate::res::str::animation_curve_back,
            Easing::Back(Ease::InOut, 2.2),
        ),
        (
            crate::res::str::animation_curve_elastic,
            Easing::ElasticOut(0.7),
        ),
        (crate::res::str::animation_curve_bounce, Easing::BounceOut),
    ]
}

/// Plots `easing` with a dot at progress `t`, and the eased value alone on a rail beside it.
fn draw_curve(draw: &mut Draw, size: Size, easing: Easing, t: f64) {
    /// The values the plot spans, with room for a curve that leaves 0 to 1.
    const SPAN: (f64, f64) = (-0.34, 1.34);
    let accent = Tone::Accent.color();
    draw.fill(
        Shape::RoundedRect(Rect::new(0.0, 0.0, size.width, size.height), 10.0),
        gallery::INSET,
    );
    let rail = (size.width - 11.0).max(12.0);
    let (left, right) = (9.0, (rail - 12.0).max(10.0));
    let (top, bottom) = (5.0, (size.height - 5.0).max(6.0));
    let at_x = |t: f64| left + (right - left) * t;
    let at_y = |value: f64| bottom - (value - SPAN.0) / (SPAN.1 - SPAN.0) * (bottom - top);

    for value in [0.0, 1.0] {
        draw.stroke(
            Shape::Line(
                Point::new(left, at_y(value)),
                Point::new(right, at_y(value)),
            ),
            SystemColor::Separator,
            1.0,
        );
    }
    let samples = easing.samples(97);
    let last = samples.len().saturating_sub(1).max(1) as f64;
    let mut line = PathBuilder::new();
    for (index, value) in samples.iter().enumerate() {
        let point = Point::new(at_x(index as f64 / last), at_y(*value));
        line = if index == 0 {
            line.move_to(point)
        } else {
            line.line_to(point)
        };
    }
    draw.stroke_styled(line.build(), accent, StrokeStyle::round(1.75));

    let value = easing.value(t);
    draw.fill(
        disc(Point::new(at_x(t), at_y(value)), 4.5),
        SystemColor::Text,
    );
    draw.stroke(
        Shape::Line(Point::new(rail, top), Point::new(rail, bottom)),
        SystemColor::Separator,
        1.0,
    );
    draw.fill(disc(Point::new(rail, at_y(value)), 3.5), accent);
}

/// "Easing curves": one curve of every `Easing` variant, all run by one `Animated` progress.
fn easing_card() -> AnyPiece {
    let progress = Animated::new(CURVES_REST);
    let cells: Vec<AnyPiece> = curves()
        .into_iter()
        .enumerate()
        .map(|(index, (name, easing))| {
            column((
                canvas(move |draw: &mut Draw, size: Size| {
                    draw_curve(draw, size, easing, progress.get());
                })
                .a11y(|a| a.decorative())
                .grow(Dimension::Width)
                .height(74.0)
                .id(day::id!(animation_easing_plot, index)),
                label(name())
                    .font(Font::Caption)
                    .secondary()
                    .single_line()
                    .grow(Dimension::Width),
            ))
            .spacing(4.0)
            .align(HAlign::Leading)
            .grow(Dimension::Width)
            .any()
        })
        .collect();
    let mut cells = cells.into_iter();
    let mut lines: Vec<AnyPiece> = Vec::new();
    while let (Some(a), Some(b), Some(c)) = (cells.next(), cells.next(), cells.next()) {
        lines.push(
            row((a, b, c))
                .spacing(Space::S)
                .align(VAlign::Top)
                .grow(Dimension::Width)
                .any(),
        );
    }

    card(
        crate::res::vectors::nav_charts,
        crate::res::str::animation_easing_title(),
        crate::res::str::animation_easing_subtitle(),
        column(lines).spacing(Space::S).grow(Dimension::Width),
    )
    .trailing(
        button(crate::res::str::animation_easing_play())
            .icon(Symbol::Play)
            .bordered()
            .action(move || {
                progress.set(0.0);
                progress.animate_to(1.0, AnimSpec::linear(CURVES_MS));
            })
            .id(day::id!(animation_easing_play)),
    )
    .id(day::id!(animation_easing_card))
    .build()
}

// ---------------------------------------------------------------------------
// Platform animator
// ---------------------------------------------------------------------------

/// Every signal the playground threads together: the queued (`p_`) targets the controls write,
/// the applied (`a_`) state the box renders, and the animation settings. All fields are
/// `Signal` (`Copy`), so the whole struct is `Copy` and drops straight into each handler.
#[derive(Clone, Copy)]
struct Anim {
    /// The queued scale.
    p_scale: Signal<f64>,
    /// The queued rotation, in degrees.
    p_rot: Signal<f64>,
    /// The queued opacity.
    p_op: Signal<f64>,
    /// The queued horizontal offset, in points.
    p_offx: Signal<f64>,
    /// The queued vertical offset, in points.
    p_offy: Signal<f64>,
    /// The queued hue, in degrees.
    p_hue: Signal<f64>,
    /// The applied scale.
    a_scale: Signal<f64>,
    /// The applied rotation.
    a_rot: Signal<f64>,
    /// The applied opacity.
    a_op: Signal<f64>,
    /// The applied horizontal offset.
    a_offx: Signal<f64>,
    /// The applied vertical offset.
    a_offy: Signal<f64>,
    /// The applied hue.
    a_hue: Signal<f64>,
    /// The curve picked: spring, ease-in-out, ease-out or linear.
    curve: Signal<usize>,
    /// The duration, in milliseconds.
    dur: Signal<f64>,
}

/// The hue the playground's box starts on, in degrees.
const HOME_HUE: f64 = 24.0;

impl Anim {
    /// The playground at rest.
    fn new() -> Self {
        Anim {
            p_scale: Signal::new(1.0),
            p_rot: Signal::new(0.0),
            p_op: Signal::new(1.0),
            p_offx: Signal::new(0.0),
            p_offy: Signal::new(0.0),
            p_hue: Signal::new(HOME_HUE),
            a_scale: Signal::new(1.0),
            a_rot: Signal::new(0.0),
            a_op: Signal::new(1.0),
            a_offx: Signal::new(0.0),
            a_offy: Signal::new(0.0),
            a_hue: Signal::new(HOME_HUE),
            curve: Signal::new(0),
            dur: Signal::new(500.0),
        }
    }

    /// Commits every queued value at once, under one animation, so all channels move together.
    fn commit(self) {
        let spec = spec_for(
            self.curve.get_untracked(),
            self.dur.get_untracked().clamp(100.0, 3000.0) as u32,
        );
        with_animation(spec, move || {
            self.a_scale.set(self.p_scale.get_untracked());
            self.a_rot.set(self.p_rot.get_untracked());
            self.a_op.set(self.p_op.get_untracked());
            self.a_offx.set(self.p_offx.get_untracked());
            self.a_offy.set(self.p_offy.get_untracked());
            self.a_hue.set(self.p_hue.get_untracked());
        });
    }

    /// Queues random targets (updating the sliders), then animates to them.
    fn randomize(self) {
        self.p_scale.set(rand_range(0.5, 1.5));
        self.p_rot.set(rand_range(0.0, 360.0));
        self.p_op.set(rand_range(0.2, 1.0));
        self.p_offx.set(rand_range(-72.0, 72.0));
        self.p_offy.set(rand_range(-40.0, 40.0));
        self.p_hue.set(rand_range(0.0, 360.0));
        self.commit();
    }

    /// Queues the defaults (resetting the sliders), then animates back to them.
    fn reset(self) {
        self.p_scale.set(1.0);
        self.p_rot.set(0.0);
        self.p_op.set(1.0);
        self.p_offx.set(0.0);
        self.p_offy.set(0.0);
        self.p_hue.set(HOME_HUE);
        self.commit();
    }

    /// Whether any queued target differs from the applied state, which is whether Animate!
    /// has anything to do. Tracked reads, so bindings on this re-run as the sliders move and
    /// as a commit applies the queue.
    fn dirty(self) -> bool {
        self.p_scale.get() != self.a_scale.get()
            || self.p_rot.get() != self.a_rot.get()
            || self.p_op.get() != self.a_op.get()
            || self.p_offx.get() != self.a_offx.get()
            || self.p_offy.get() != self.a_offy.get()
            || self.p_hue.get() != self.a_hue.get()
    }
}

/// Curve index (from the segmented picker) and duration as an [`AnimSpec`]. For the spring the
/// duration is its response, and the run lasts until it settles, about twice as long.
fn spec_for(curve: usize, dur_ms: u32) -> AnimSpec {
    match curve {
        0 => AnimSpec::spring((f64::from(dur_ms) / 1000.0).max(0.1), 0.6),
        1 => AnimSpec::ease_in_out(dur_ms),
        2 => AnimSpec::ease_out(dur_ms),
        _ => AnimSpec::linear(dur_ms),
    }
}

/// The box on its stage. The box is a direct child of the stage (no box-sized wrapper between
/// them) and its `.transform` is the outermost modifier, so the transform moves it within the
/// stage's bounds, the only clipping container above it. The slider ranges keep it inside the
/// stage on any screen.
fn playground_stage(s: Anim) -> impl Piece {
    let (a_hue, a_offx, a_offy, a_scale, a_rot, a_op) =
        (s.a_hue, s.a_offx, s.a_offy, s.a_scale, s.a_rot, s.a_op);
    zstack((
        canvas(|draw: &mut Draw, size: Size| gallery::dotted_stage(draw, size, 16.0))
            .grow(Dimension::Width)
            .height(190.0),
        label("")
            .frame(68.0, 68.0)
            .background(move || Color::hsl(a_hue.get(), 0.7, 0.56))
            .corner_radius(16.0)
            .opacity(a_op)
            .transform(move || Transform {
                tx: a_offx.get(),
                ty: a_offy.get(),
                sx: a_scale.get(),
                sy: a_scale.get(),
                rotation: Angle::deg(a_rot.get()),
                anchor_x: 0.5,
                anchor_y: 0.5,
            })
            .id(day::id!(anim_box)),
    ))
    .grow(Dimension::Width)
}

/// "Platform animator": one box driven by every animatable channel at once. The sliders only
/// queue target values; the buttons commit them inside a single `with_animation`, so scale,
/// rotation, opacity, offset and color animate together with the chosen curve and duration.
fn playground_card() -> AnyPiece {
    let s = Anim::new();
    let actions = gallery::actions((
        button(crate::res::str::anim_go_label())
            .prominent()
            .enabled(move || s.dirty())
            .action(move || s.commit())
            .id(day::id!(anim_go)),
        button(crate::res::str::anim_randomize())
            .bordered()
            .action(move || s.randomize())
            .id(day::id!(anim_randomize)),
        button(crate::res::str::anim_reset_label())
            .bordered()
            .action(move || s.reset())
            .id(day::id!(anim_reset)),
    ));
    let form = column((
        field(
            crate::res::str::anim_scale(),
            slider(s.p_scale).range(0.5..=1.5).id(day::id!(anim_scale)),
        ),
        field(
            crate::res::str::anim_rotation(),
            slider(s.p_rot)
                .range(0.0..=360.0)
                .id(day::id!(anim_rotation)),
        ),
        field(
            crate::res::str::anim_opacity(),
            slider(s.p_op).range(0.15..=1.0).id(day::id!(anim_opacity)),
        ),
        field(
            crate::res::str::anim_offset_x(),
            slider(s.p_offx).range(-72.0..=72.0).id(day::id!(anim_offx)),
        ),
        field(
            crate::res::str::anim_offset_y(),
            slider(s.p_offy).range(-40.0..=40.0).id(day::id!(anim_offy)),
        ),
        field(
            crate::res::str::anim_hue(),
            slider(s.p_hue).range(0.0..=360.0).id(day::id!(anim_hue)),
        ),
        // The curve's names say what the row is, and the full width keeps all four on a phone.
        picker(
            s.curve,
            [
                (0, crate::res::str::anim_curve_spring()),
                (1, crate::res::str::anim_curve_ease_in_out()),
                (2, crate::res::str::anim_curve_ease_out()),
                (3, crate::res::str::anim_curve_linear()),
            ],
        )
        .segmented()
        .a11y(|a| a.label(crate::res::str::anim_curve()))
        .id(day::id!(anim_curve)),
        field(
            crate::res::str::anim_duration(),
            slider_with_readout(
                slider(s.dur)
                    .range(100.0..=3000.0)
                    .step(100.0)
                    .id(day::id!(anim_duration)),
                // The length of the run the curve makes of it: a spring settles after its
                // response.
                move || {
                    let spec = spec_for(s.curve.get(), s.dur.get().clamp(100.0, 3000.0) as u32);
                    crate::res::str::anim_duration_ms(i64::from(spec.duration_ms)).format()
                },
                "8888 ms",
                day::id!(anim_dur_value),
            ),
        ),
        // The user's reduce-motion setting (docs/accessibility.md), live: under it every
        // transition on this page lands at once, and a scripted run's fast mode reads the
        // same way, since fast mode is this setting forced on.
        field(
            crate::res::str::anim_reduced_motion(),
            label(move || {
                if day::reduce_motion() {
                    crate::res::str::ctl_on()
                } else {
                    crate::res::str::ctl_off()
                }
                .format()
            })
            .id(day::id!(anim_reduced_motion)),
        ),
    ))
    .spacing(Space::S)
    .align(HAlign::Leading)
    .grow(Dimension::Width);

    card(
        crate::res::vectors::nav_controls,
        crate::res::str::animation_play_title(),
        crate::res::str::animation_play_subtitle(),
        column((playground_stage(s), actions, form))
            .spacing(Space::M)
            .align(HAlign::Leading),
    )
    .id(day::id!(animation_play_card))
    .build()
}

// ---------------------------------------------------------------------------
// Animated values
// ---------------------------------------------------------------------------

/// What the card glides between: a diameter in kilometers, a distance from the Sun in
/// astronomical units, and a color. `Lerp` covers the tuple, and blends the color in OKLab.
type Look = (f64, f64, Color);

/// The four planets the card visits, nearest the Sun first, with their mean figures.
fn planets() -> [(Name, Look); 4] {
    [
        (
            crate::res::str::animation_tween_mercury,
            (4_879.0, 0.39, crate::palette::SLATE),
        ),
        (
            crate::res::str::animation_tween_earth,
            (12_742.0, 1.0, crate::palette::SKY),
        ),
        (
            crate::res::str::animation_tween_jupiter,
            (139_820.0, 5.2, crate::palette::AMBER),
        ),
        (
            crate::res::str::animation_tween_neptune,
            (49_244.0, 30.07, crate::palette::VIOLET),
        ),
    ]
}

/// Draws the Sun at the leading edge, the four orbits, and a planet that looks like `look`.
/// Distances and sizes are on logarithmic scales, which is what lets Mercury and Neptune share
/// a stage.
fn draw_planet(draw: &mut Draw, size: Size, look: Look) {
    let (km, au, color) = look;
    let all = planets();
    let (near, far) = (all[0].1.1.log10(), all[3].1.1.log10());
    let middle = size.height / 2.0;
    let sun = Point::new(-size.height * 0.42, middle);
    let (first, last) = (56.0, (size.width - 38.0).max(57.0));
    let at_x = |au: f64| first + (last - first) * (au.max(0.01).log10() - near) / (far - near);

    draw.clipped(
        Shape::RoundedRect(Rect::new(0.0, 0.0, size.width, size.height), 12.0),
        |draw| {
            gallery::dotted_stage(draw, size, 16.0);
            for (_, (_, orbit, _)) in &all {
                draw.stroke(disc(sun, at_x(*orbit) - sun.x), SystemColor::Separator, 1.0);
            }
            draw.fill(
                disc(sun, size.height * 0.42 + 26.0),
                crate::palette::AMBER.with_alpha(0.35),
            );
            draw.fill(disc(sun, size.height * 0.42 + 16.0), crate::palette::AMBER);
            let radius = (6.0 + 13.0 * (km.max(1.0) / all[0].1.0).log10()).max(3.0);
            draw.fill(disc(Point::new(at_x(au), middle), radius), color);
        },
    );
}

/// "Animated values": one `Animated` tuple that glides to the planet picked, read by the canvas
/// and by the two figures under it.
fn tween_card() -> AnyPiece {
    let picked = Signal::new(1_usize);
    let look = Animated::new(planets()[1].1);
    watch(
        move || picked.get(),
        move |picked: &usize, _| {
            if let Some((_, target)) = planets().get(*picked) {
                look.animate_to(*target, AnimSpec::spring(0.8, 0.68));
            }
        },
    );

    let options: Vec<(usize, day::LocalizedText)> = planets()
        .iter()
        .enumerate()
        .map(|(index, (name, _))| (index, name()))
        .collect();
    let (drawn, diameter, distance) = (look, look, look);
    card(
        crate::res::vectors::nav_canvas,
        crate::res::str::animation_tween_title(),
        crate::res::str::animation_tween_subtitle(),
        column((
            canvas(move |draw: &mut Draw, size: Size| draw_planet(draw, size, drawn.get()))
                .a11y(|a| a.decorative())
                .grow(Dimension::Width)
                .height(132.0)
                .id(day::id!(animation_tween_stage)),
            picker(picked, options)
                .segmented()
                .id(day::id!(animation_tween_target)),
            row((
                gallery::stat(
                    crate::res::str::animation_tween_diameter(),
                    move || {
                        crate::res::str::animation_tween_km(diameter.get().0.round() as i64)
                            .format()
                    },
                    None,
                ),
                gallery::stat(
                    crate::res::str::animation_tween_distance(),
                    move || {
                        crate::res::str::animation_tween_au(format!("{:.2}", distance.get().1))
                            .format()
                    },
                    None,
                ),
            ))
            .grow(Dimension::Width),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .id(day::id!(animation_tween_card))
    .build()
}

// ---------------------------------------------------------------------------
// Interpolation
// ---------------------------------------------------------------------------

/// How far the tile's center keeps from each end of the stage, in points.
const INTERP_MARGIN: f64 = 44.0;
/// The tile's side, in points.
const INTERP_TILE: f64 = 52.0;

/// "Interpolation": one `Animated` number between 0 and 1 that the tile's place, opacity, scale,
/// turn and color are all mapped from. Dragged and let go, the number springs to the nearer
/// end from the speed it was released at.
fn interpolation_card() -> AnyPiece {
    let t = Animated::new(0.0);
    let width = Signal::new(0.0_f64);
    let released = Signal::new(0.0_f64);
    let slot = Slot::default();
    let spring = Spring::new(170.0, 15.0);
    let travel = move || (width.get() - 2.0 * INTERP_MARGIN).max(1.0);

    let drag = {
        let slot = slot.clone();
        move |d: Drag| {
            let room = day::reactive::untrack(travel);
            let at = ((d.location.x - INTERP_MARGIN) / room).clamp(-0.15, 1.15);
            if d.phase == DragPhase::Ended {
                let velocity = d.velocity.x / room;
                released.set(d.velocity.x);
                t.set(at);
                t.set_velocity(velocity);
                let to = if at + velocity * 0.15 > 0.5 { 1.0 } else { 0.0 };
                slot.play(&Animation::spring(t, to, spring));
            } else {
                slot.stop();
                t.set(at);
            }
        }
    };
    let flip = move || {
        let to = if t.target() > 0.5 { 0.0 } else { 1.0 };
        slot.play(&Animation::spring(t, to, spring));
    };

    let tile = label("")
        .frame(INTERP_TILE, INTERP_TILE)
        .background(t.interpolate([0.0, 1.0], [Tone::Accent.color(), Tone::Danger.color()]))
        .corner_radius(Radius::L)
        .opacity(t.interpolate([0.0, 0.5, 1.0], [1.0, 0.6, 1.0]))
        .scale(t.interpolate([0.0, 0.5, 1.0], [1.0, 1.3, 1.0]))
        .rotation(t.interpolate([0.0, 1.0], [Angle::ZERO, Angle::HALF]))
        .translation(move || (t.get() - 0.5) * travel(), 0.0)
        .id(day::id!(animation_interp_tile));
    let stage = zstack((
        canvas(move |draw: &mut Draw, size: Size| {
            note_width(width, size);
            gallery::dotted_stage(draw, size, 16.0);
        })
        .a11y(|a| a.decorative())
        .grow(Dimension::Width)
        .height(112.0),
        tile,
    ))
    .align(Alignment::Center)
    .grow(Dimension::Width)
    .height(112.0)
    .on_drag_along(DragAxis::Horizontal, drag)
    .id(day::id!(animation_interp_stage));

    card(
        crate::res::vectors::nav_styles,
        crate::res::str::animation_interp_title(),
        crate::res::str::animation_interp_subtitle(),
        column((
            stage,
            row((
                // A stat whose figure a script reads, so the id is on the figure itself.
                column((
                    label(crate::res::str::animation_interp_value())
                        .font(Font::Caption)
                        .secondary(),
                    label(move || format!("{:.2}", t.get()))
                        .font(Font::Callout)
                        .tabular()
                        .single_line()
                        .id(day::id!(animation_interp_value)),
                ))
                .spacing(2.0)
                .align(HAlign::Leading)
                .grow(Dimension::Width),
                gallery::stat(
                    crate::res::str::animation_interp_release(),
                    move || {
                        crate::res::str::animation_interp_speed(released.get().round() as i64)
                            .format()
                    },
                    None,
                ),
            ))
            .grow(Dimension::Width),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .trailing(
        button(crate::res::str::animation_interp_flip())
            .bordered()
            .action(flip)
            .id(day::id!(animation_interp_flip)),
    )
    .id(day::id!(animation_interp_card))
    .build()
}

// ---------------------------------------------------------------------------
// Spring
// ---------------------------------------------------------------------------

/// How long the spring is plotted and timed for, in seconds.
const SPRING_SECS: f64 = 2.0;
/// The share of the track the knob travels. The rest is room for a loose spring's overshoot.
const KNOB_TRAVEL: f64 = 0.64;
/// The knob's diameter, in points.
const KNOB: f64 = 22.0;
/// The largest value the plot shows; a step response rests on 1.
const PLOT_TOP: f64 = 1.75;

/// Plots a spring's answer to a step, with a dot at where the knob is now: `clock` across, its
/// live value up. `returning` flips the plot for the way back to 0.
fn draw_spring(
    draw: &mut Draw,
    size: Size,
    response: &[f64],
    live: f64,
    clock: f64,
    returning: bool,
) {
    gallery::dotted_stage(draw, size, 16.0);
    let inset = 10.0;
    let (left, right) = (inset, (size.width - inset).max(inset + 1.0));
    let (top, bottom) = (inset, (size.height - inset).max(inset + 1.0));
    let at_y = |value: f64| bottom - value.clamp(0.0, PLOT_TOP) / PLOT_TOP * (bottom - top);
    let shown = |value: f64| if returning { 1.0 - value } else { value };
    gallery::dashed(
        draw,
        Point::new(left, at_y(shown(1.0))),
        Point::new(right, at_y(shown(1.0))),
        SystemColor::TertiaryText,
    );
    let last = response.len().saturating_sub(1).max(1) as f64;
    let mut line = PathBuilder::new();
    for (index, sample) in response.iter().enumerate() {
        let point = Point::new(
            left + (right - left) * index as f64 / last,
            at_y(shown(*sample)),
        );
        line = if index == 0 {
            line.move_to(point)
        } else {
            line.line_to(point)
        };
    }
    let accent = Tone::Accent.color();
    draw.stroke_styled(line.build(), accent, StrokeStyle::round(2.0));
    let dot = Point::new(left + (right - left) * clock, at_y(live));
    draw.fill(disc(dot, 7.0), accent.with_alpha(0.3));
    draw.fill(disc(dot, 4.0), SystemColor::Text);
}

/// The knob on its track. `value` is the knob's place between its two rests, 0 to 1.
fn spring_track(value: Animated, width: Signal<f64>) -> impl Piece {
    let room = move || (width.get() - KNOB - 4.0).max(0.0);
    zstack((
        canvas(move |draw: &mut Draw, size: Size| {
            note_width(width, size);
            draw.fill(
                Shape::RoundedRect(
                    Rect::new(0.0, 0.0, size.width, size.height),
                    size.height / 2.0,
                ),
                gallery::INSET,
            );
            // Where the knob comes to rest when sent across.
            let rest = 2.0 + KNOB / 2.0 + KNOB_TRAVEL * (size.width - KNOB - 4.0).max(0.0);
            draw.stroke(
                Shape::Line(Point::new(rest, 5.0), Point::new(rest, size.height - 5.0)),
                SystemColor::TertiaryText,
                1.0,
            );
        })
        .a11y(|a| a.decorative())
        .grow(Dimension::Width)
        .height(KNOB + 4.0),
        circle()
            .fill(Tone::Accent.color())
            .frame(KNOB, KNOB)
            .transform(move || {
                // A very loose spring swings past the track's ends; the knob stops at them.
                let x = (value.get() * KNOB_TRAVEL * room()).clamp(0.0, room());
                Transform::translate(2.0 + x, 0.0)
            })
            .id(day::id!(animation_springs_knob, 0_usize)),
    ))
    .align(Alignment::Leading)
    .grow(Dimension::Width)
}

/// "Spring": one `Spring` whose stiffness, damping and mass the sliders set. The plot is its
/// step response. Launch sends the knob across on it; a knob dragged and let go springs to the
/// nearer rest from the speed it was released at.
fn springs_card() -> AnyPiece {
    let stiffness = Signal::new(210.0_f64);
    let damping = Signal::new(11.0_f64);
    let mass = Signal::new(1.0_f64);
    let spring = move || Spring::new(stiffness.get(), damping.get()).mass(mass.get());
    let knob = Animated::new(0.0);
    let clock = Animated::new(0.0);
    let heading = Signal::new(0.0_f64);
    let width = Signal::new(0.0_f64);
    let slot = Slot::default();

    // Springs the knob to `target` from the velocity it has, and runs the plot's clock beside it.
    let send = {
        let slot = slot.clone();
        move |target: f64| {
            heading.set(target);
            clock.set(0.0);
            slot.play(&Animation::parallel([
                Animation::spring(knob, target, day::reactive::untrack(spring)),
                Animation::timing(clock, 1.0, (SPRING_SECS * 1000.0) as u32, Easing::Linear),
            ]));
        }
    };
    // The target is the rest the knob is not heading for.
    let launch = {
        let send = send.clone();
        move || send(1.0 - heading.get_untracked())
    };
    // The knob's travel in points, and the place of a point on the track between the rests.
    let travel = move || (KNOB_TRAVEL * (width.get_untracked() - KNOB - 4.0)).max(1.0);
    let drag = move |d: Drag| {
        let at = ((d.location.x - 2.0 - KNOB / 2.0) / travel()).clamp(-0.2, 1.2);
        if d.phase == DragPhase::Ended {
            // The release velocity, in track lengths per second, is where the spring starts.
            let velocity = d.velocity.x / travel();
            knob.set(at);
            knob.set_velocity(velocity);
            send(if at + velocity * 0.15 > 0.5 { 1.0 } else { 0.0 });
        } else {
            slot.stop();
            knob.set(at);
        }
    };

    card(
        crate::res::vectors::nav_physics,
        crate::res::str::animation_springs_title(),
        crate::res::str::animation_springs_subtitle(),
        column((
            canvas(move |draw: &mut Draw, size: Size| {
                draw_spring(
                    draw,
                    size,
                    &spring().response(SPRING_SECS, 97),
                    knob.get(),
                    clock.get(),
                    heading.get() == 0.0 && clock.get() > 0.0,
                );
            })
            .a11y(|a| a.decorative())
            .grow(Dimension::Width)
            .height(132.0)
            .id(day::id!(animation_springs_plot)),
            row((
                spring_track(knob, width).on_drag_along(DragAxis::Horizontal, drag),
                button(crate::res::str::animation_springs_release())
                    .prominent()
                    .action(launch)
                    .id(day::id!(animation_springs_release)),
            ))
            .spacing(Space::M)
            .align(VAlign::Center)
            .grow(Dimension::Width),
            column((
                field(
                    crate::res::str::animation_springs_stiffness(),
                    slider_with_readout(
                        slider(stiffness)
                            .range(40.0..=600.0)
                            .id(day::id!(animation_springs_stiffness)),
                        move || format!("{:.0}", stiffness.get()),
                        "600",
                        day::id!(animation_springs_stiffness_value),
                    ),
                ),
                field(
                    crate::res::str::animation_springs_damping(),
                    slider_with_readout(
                        slider(damping)
                            .range(2.0..=44.0)
                            .id(day::id!(animation_springs_damping)),
                        move || format!("{:.0}", damping.get()),
                        "600",
                        day::id!(animation_springs_damping_value),
                    ),
                ),
                field(
                    crate::res::str::animation_springs_mass(),
                    slider_with_readout(
                        slider(mass)
                            .range(0.4..=4.0)
                            .id(day::id!(animation_springs_mass)),
                        move || format!("{:.1}", mass.get()),
                        "600",
                        day::id!(animation_springs_mass_value),
                    ),
                ),
            ))
            .spacing(Space::S)
            .grow(Dimension::Width),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .trailing(gallery::badge_with_id(
        day::id!(animation_springs_ratio),
        move || {
            // The damping ratio: below 1 the spring overshoots, at 1 and above it does not.
            let ratio = damping.get() / (2.0 * (stiffness.get() * mass.get()).sqrt());
            crate::res::str::animation_springs_ratio(format!("{ratio:.2}")).format()
        },
        Some(Tone::Accent),
    ))
    .id(day::id!(animation_springs_card))
    .build()
}

// ---------------------------------------------------------------------------
// Choreography
// ---------------------------------------------------------------------------

/// How many bars the choreography moves.
const BARS: usize = 7;
/// A bar's width and its full height, in points.
const BAR: (f64, f64) = (16.0, 72.0);
/// The share of its height a bar keeps at rest.
const BAR_REST: f64 = 0.3;
/// The height of the stage, in points.
const BARS_STAGE: f64 = 120.0;
/// How far above the stage's bottom edge the bars stand, in points.
const BARS_BASELINE: f64 = 16.0;

/// The animation for composition `kind` over the bars' `levels`, each 0 at rest and 1 at full
/// height.
fn choreography(kind: usize, levels: &[Animated]) -> Animation {
    let count = levels.len().max(1) as f64;
    let fall = |ms: u32| {
        Animation::parallel(
            levels
                .iter()
                .map(|&level| Animation::timing(level, 0.0, ms, Easing::EASE_IN_OUT)),
        )
    };
    match kind {
        // In order: each bar rises once the one before it has, then all fall.
        0 => Animation::sequence(
            levels
                .iter()
                .map(|&level| Animation::timing(level, 1.0, 130, Easing::Power(Ease::Out, 2.0)))
                .chain([Animation::delay(260), fall(380)]),
        ),
        // Together: every bar springs to its place on an arch, then all fall.
        1 => Animation::sequence([
            Animation::parallel(levels.iter().enumerate().map(|(index, &level)| {
                let across = (index as f64 + 0.5) / count;
                let arch = 0.3 + 0.7 * (across * std::f64::consts::PI).sin();
                Animation::spring(level, arch, Spring::new(230.0, 9.0))
            })),
            Animation::delay(320),
            fall(420),
        ]),
        // Offset: each bar rises and springs back, 65 ms after the one before it.
        2 => Animation::stagger(
            65,
            levels.iter().map(|&level| {
                Animation::sequence([
                    Animation::timing(level, 1.0, 190, Easing::Circle(Ease::Out)),
                    Animation::spring(level, 0.0, Spring::new(280.0, 12.0)),
                ])
            }),
        ),
        // Repeating: a wave that crosses the bars until it is stopped.
        _ => Animation::stagger(
            90,
            levels.iter().map(|&level| {
                Animation::sequence([
                    Animation::timing(level, 1.0, 340, Easing::Sine(Ease::InOut)),
                    Animation::timing(level, 0.0, 340, Easing::Sine(Ease::InOut)),
                ])
            }),
        )
        .looped(),
    }
}

/// "Choreography": seven bars moved by a sequence, a group, a stagger or a loop. Stop holds
/// them where they are; Reset puts them back where the run found them.
fn composition_card() -> AnyPiece {
    let levels: Vec<Animated> = (0..BARS).map(|_| Animated::new(0.0)).collect();
    let kind = Signal::new(2_usize);
    let slot = Slot::default();
    let running = Signal::new(false);

    // Choosing another composition stops the one playing and puts the bars back.
    watch(move || kind.get(), {
        let (slot, levels) = (slot.clone(), levels.clone());
        move |_, _| {
            slot.stop();
            running.set(false);
            levels.iter().for_each(|level| level.set(0.0));
        }
    });
    let toggle = {
        let (slot, levels) = (slot.clone(), levels.clone());
        move || {
            if running.get_untracked() {
                slot.stop();
                running.set(false);
                return;
            }
            let animation = choreography(kind.get_untracked(), &levels);
            // The loop is decoration: under reduced motion it does not start. A finite
            // composition lands on its end at once there, and reports that it finished.
            if animation.is_endless() && day::reactive::untrack(day::reduce_motion) {
                return;
            }
            running.set(true);
            slot.play(&animation).on_end(move |outcome| {
                if outcome == Outcome::Finished {
                    running.set(false);
                }
            });
        }
    };
    let reset = move || {
        slot.reset();
        running.set(false);
    };

    // Each bar stands at the bottom of a lane as tall as the stage. The lane is the bar's
    // parent, so a spring's overshoot stays inside it on the toolkits that clip children. The
    // scale is about the bar's center, and the offset keeps its foot on the baseline.
    let lanes: Vec<AnyPiece> = levels
        .iter()
        .copied()
        .enumerate()
        .map(|(index, level)| {
            let scale = move || BAR_REST + (1.0 - BAR_REST) * level.get();
            let bar = label("")
                .frame(BAR.0, BAR.1)
                .background(Tone::Accent.color())
                .corner_radius(Corner::CAPSULE)
                .opacity(level.interpolate([0.0, 1.0], [0.55, 1.0]))
                .transform(move || Transform {
                    ty: (1.0 - scale()) * BAR.1 / 2.0,
                    sy: scale(),
                    ..Transform::IDENTITY
                })
                .id(day::id!(animation_compose_bar, index));
            column((spacer(), bar, label("").frame(BAR.0, BARS_BASELINE)))
                .spacing(0.0)
                .align(HAlign::Center)
                .frame(BAR.0 + 14.0, BARS_STAGE)
                .any()
        })
        .collect();
    let stage = zstack((
        canvas(|draw: &mut Draw, size: Size| {
            gallery::dotted_stage(draw, size, 16.0);
            let y = size.height - BARS_BASELINE;
            draw.stroke(
                Shape::Line(Point::new(14.0, y), Point::new(size.width - 14.0, y)),
                SystemColor::TertiaryText,
                1.0,
            );
        })
        .a11y(|a| a.decorative())
        .grow(Dimension::Width)
        .height(BARS_STAGE),
        row(lanes).spacing(0.0).align(VAlign::Top),
    ))
    .align(Alignment::Center)
    .grow(Dimension::Width)
    .height(BARS_STAGE);

    card(
        crate::res::vectors::nav_stack,
        crate::res::str::animation_compose_title(),
        crate::res::str::animation_compose_subtitle(),
        column((
            stage,
            picker(
                kind,
                [
                    (0_usize, crate::res::str::animation_compose_sequence()),
                    (1, crate::res::str::animation_compose_parallel()),
                    (2, crate::res::str::animation_compose_stagger()),
                    (3, crate::res::str::animation_compose_repeat()),
                ],
            )
            .segmented()
            .id(day::id!(animation_compose_kind)),
            gallery::actions((
                button(move || {
                    if running.get() {
                        crate::res::str::animation_compose_stop().format()
                    } else {
                        crate::res::str::animation_compose_play().format()
                    }
                })
                .prominent()
                .action(toggle)
                .id(day::id!(animation_compose_play)),
                button(crate::res::str::animation_compose_reset())
                    .bordered()
                    .action(reset)
                    .id(day::id!(animation_compose_reset)),
            )),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .id(day::id!(animation_compose_card))
    .build()
}

// ---------------------------------------------------------------------------
// Implicit animation
// ---------------------------------------------------------------------------

/// How many rows the log under the floating button has.
const SOLS: i64 = 60;
/// How far the log scrolls, in points, before the button shows.
const TOP_AFTER: f64 = 160.0;
/// How far the log scrolls, in points, while its heading fades out.
const HEADING_FADE: f64 = 56.0;

/// "Implicit animation": a log whose heading fades as it scrolls, and a button that arrives
/// once it has scrolled on. The heading's opacity is a function of the scroll offset. The
/// button waits for the scrolling to stop, and `.animation(..)` on it is all that animates it.
fn header_card() -> AnyPiece {
    let offset = Signal::new(Point::ZERO);
    let phase = Signal::new(ScrollPhase::Idle);
    let jump: Signal<Option<ScrollTarget>> = Signal::new(None);
    // Where the toolkit reports no offset the binding keeps its value, so the button stays.
    let tracked = capability(Cap::ScrollOffset) != Support::Unsupported;
    let shown =
        move || !tracked || (offset.get().y > TOP_AFTER && phase.get() == ScrollPhase::Idle);
    let fade = Rc::new(Interpolation::new([0.0, HEADING_FADE], [1.0, 0.0]));

    let rows: Vec<AnyPiece> = (1..=SOLS)
        .map(|n| {
            label(crate::res::str::animation_header_row(n))
                .single_line()
                .grow(Dimension::Width)
                .padding(Insets::symmetric(14.0, 10.0))
                .any()
        })
        .collect();
    let log = scroll(
        column((
            label(crate::res::str::animation_header_heading())
                .font(Font::Title3)
                .bold()
                .padding(Insets::symmetric(14.0, 12.0))
                .opacity(move || fade.at(offset.get().y))
                .id(day::id!(animation_header_heading)),
            column(rows).spacing(0.0).grow(Dimension::Width),
        ))
        .spacing(0.0)
        .align(HAlign::Leading)
        .grow(Dimension::Width),
    )
    .scroll_offset(offset)
    .scroll_phase(phase)
    .scroll_target(jump)
    .id(day::id!(animation_header_list))
    .grow(Dimension::Width)
    .height(248.0);

    let top = button(crate::res::str::animation_header_top())
        .prominent()
        .action(move || jump.set(Some(ScrollTarget::Top)))
        .id(day::id!(animation_header_top))
        .opacity(move || if shown() { 1.0 } else { 0.0 })
        .translation(0.0, move || if shown() { 0.0 } else { 18.0 })
        .animation(AnimSpec::spring(0.36, 0.74))
        .padding(12.0);

    card(
        crate::res::vectors::nav_list,
        crate::res::str::animation_header_title(),
        crate::res::str::animation_header_subtitle(),
        zstack((log, top))
            .align(Alignment::BottomTrailing)
            .grow(Dimension::Width)
            .height(248.0)
            .corner_radius(Radius::M),
    )
    .trailing(gallery::badge_with_id(
        day::id!(animation_header_offset),
        move || {
            if tracked {
                crate::res::str::animation_header_offset(offset.get().y.max(0.0).round() as i64)
                    .format()
            } else {
                crate::res::str::animation_header_untracked().format()
            }
        },
        None,
    ))
    .id(day::id!(animation_header_card))
    .build()
}

// ---------------------------------------------------------------------------
// Reflow
// ---------------------------------------------------------------------------

/// The most tiles the reflow card holds.
const MAX_TILES: usize = 14;
/// The sides a tile can have, in points: small, medium and large.
const TILE_SIDES: [f64; 3] = [34.0, 50.0, 70.0];

/// "Reflow": tiles that arrive, leave, resize and trade places, their frames animated by the
/// platform because each change is made inside `with_animation`.
fn layout_card() -> AnyPiece {
    let tiles = Signal::new((1..=6_i64).collect::<Vec<_>>());
    let next = Signal::new(7_i64);
    let count = Signal::new(6.0_f64);
    // The size the picker shows, and the size the tiles have: the second follows the first
    // inside `with_animation`.
    let (picked, side) = (Signal::new(1_usize), Signal::new(TILE_SIDES[1]));
    let animated = |change: &dyn Fn()| with_animation(AnimSpec::spring(0.5, 0.76), change);

    // The slider sets how many tiles there are. New tiles join at the end, and the oldest
    // leave from the front, so every other tile moves up.
    watch(
        move || count.get().round() as usize,
        move |wanted: &usize, _| {
            let wanted = (*wanted).clamp(1, MAX_TILES);
            let mut now = tiles.get_untracked();
            let mut id = next.get_untracked();
            while now.len() < wanted {
                now.push(id);
                id += 1;
            }
            if now.len() > wanted {
                now.drain(..now.len() - wanted);
            }
            next.set(id);
            animated(&|| tiles.set(now.clone()));
        },
    );
    watch(
        move || picked.get(),
        move |picked: &usize, _| {
            let to = TILE_SIDES[(*picked).min(TILE_SIDES.len() - 1)];
            animated(&|| side.set(to));
        },
    );
    let shuffle = move || {
        animated(&|| {
            tiles.update(|tiles| {
                // Fisher–Yates.
                for index in (1..tiles.len()).rev() {
                    let other = (rand_range(0.0, (index + 1) as f64) as usize).min(index);
                    tiles.swap(index, other);
                }
            });
        });
    };

    let stage = row((each(
        items(move || tiles.get(), |id: &i64| *id),
        move |slot: ItemSlot<i64, i64>| {
            let id = slot.key();
            let ramp = crate::palette::RAMP;
            zstack((label(id.to_string())
                .font(Font::Footnote)
                .weight(FontWeight::Semibold)
                .color(Color::WHITE),))
            .width(move || side.get())
            .height(move || side.get())
            .background(ramp[id.unsigned_abs() as usize % ramp.len()])
            .corner_radius(Radius::M)
            .id(day::id!(animation_layout_tile, id))
        },
    ),))
    .spacing(8.0)
    .align(VAlign::Top)
    .fit(RowFit::Wrap { run_spacing: 8.0 })
    .grow(Dimension::Width);

    card(
        crate::res::vectors::nav_grid,
        crate::res::str::animation_layout_title(),
        crate::res::str::animation_layout_subtitle(),
        column((
            stage,
            column((
                field(
                    crate::res::str::animation_layout_tiles(),
                    slider_with_readout(
                        slider(count)
                            .range(1.0..=MAX_TILES as f64)
                            .step(1.0)
                            .id(day::id!(animation_layout_tiles)),
                        move || tiles.with(Vec::len).to_string(),
                        "88",
                        day::id!(animation_layout_count),
                    ),
                ),
                field(
                    crate::res::str::animation_layout_size(),
                    picker(
                        picked,
                        [
                            (0_usize, crate::res::str::animation_layout_small()),
                            (1, crate::res::str::animation_layout_medium()),
                            (2, crate::res::str::animation_layout_large()),
                        ],
                    )
                    .segmented()
                    .id(day::id!(animation_layout_size)),
                ),
            ))
            .spacing(Space::S)
            .grow(Dimension::Width),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .trailing(
        button(crate::res::str::animation_layout_shuffle())
            .bordered()
            .action(shuffle)
            .id(day::id!(animation_layout_shuffle)),
    )
    .id(day::id!(animation_layout_card))
    .build()
}

/// The Animation page. The cards are dealt left and right in this order, which is also the
/// order of a phone's single column.
pub(crate) fn animation_page() -> AnyPiece {
    card_page(
        crate::res::str::nav_animation(),
        day::id!(animation_title),
        2,
        vec![
            easing_card,
            playground_card,
            tween_card,
            interpolation_card,
            springs_card,
            composition_card,
            header_card,
            layout_card,
        ],
    )
}
