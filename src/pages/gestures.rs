// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The Gestures page: what Day's gesture decorators report, and what a page builds from them.
//!
//! The stages take their gestures on a `canvas` that is the top layer of the stage, so nothing
//! drawn on it sits between the pointer and the handler (docs/canvas.md "Interaction" and "Zoom
//! and pan"). Tap, double tap, long press and press come from Day's recognizers, a drag carries
//! its release velocity, and the wording follows the kind of pointer in use
//! (`day::primary_pointer()`). Motion that outlives a gesture (the picture's way home, the
//! dot's dip, a swiped row's return) is `day::motion`.

use std::cell::Cell;
use std::collections::VecDeque;
use std::rc::Rc;

use day::motion::{Animated, Animation, Spring};
use day::prelude::*;

use crate::gallery::{self, Tone, card, card_page, sky};

// ---------------------------------------------------------------------------
// Shared parts
// ---------------------------------------------------------------------------

/// The wording for the pointer in use (tracked): `touch` for a finger, `pointer` for a mouse or
/// a trackpad. It follows the last input, so an iPad with a trackpad switches as it is used.
fn worded(
    touch: fn() -> day::LocalizedText,
    pointer: fn() -> day::LocalizedText,
) -> impl Fn() -> day::LocalizedText + Copy + 'static {
    move || {
        if day::primary_pointer().hovers() {
            pointer()
        } else {
            touch()
        }
    }
}

/// A value rounded to a whole number, for a readout.
fn whole(value: f64) -> i64 {
    value.round() as i64
}

/// A point as a readout: both coordinates rounded.
fn coordinates(at: Point) -> String {
    format!("{}, {}", whole(at.x), whole(at.y))
}

/// A circle of radius `radius` around `center`.
fn disc(center: Point, radius: f64) -> Shape {
    Shape::Ellipse(Rect::new(
        center.x - radius,
        center.y - radius,
        radius * 2.0,
        radius * 2.0,
    ))
}

/// A line of secondary text under a demo.
fn note<M>(text: impl IntoText<M>) -> impl Piece {
    label(text)
        .font(Font::Footnote)
        .secondary()
        .grow(Dimension::Width)
}

/// A live figure under its name, with `id` on the figure so a script can read it.
fn reading(
    name: day::LocalizedText,
    id: ElementId,
    value: impl Fn() -> String + 'static,
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

/// A count in a capsule, in the accent once it is above zero.
fn count_badge(
    id: ElementId,
    text: impl Fn() -> String + 'static,
    count: Signal<i64>,
) -> impl Piece {
    let lit = move || count.get() > 0;
    label(text)
        .font(Font::Caption)
        .weight(FontWeight::Medium)
        .color(move || {
            if lit() {
                Ink::Fixed(Tone::Accent.color())
            } else {
                Ink::system(SystemColor::SecondaryText)
            }
        })
        .single_line()
        .id(id)
        .padding(Insets::symmetric(10.0, 4.0))
        .background(move || {
            if lit() {
                Tone::Accent.tint()
            } else {
                gallery::INSET
            }
        })
        .corner_radius(Corner::CAPSULE)
}

// ---------------------------------------------------------------------------
// Drag, pinch and rotate
// ---------------------------------------------------------------------------

/// The picture's width and height on the stage, in points.
const PICTURE: (f64, f64) = (184.0, 138.0);
/// The corner radius of the picture and of its home outline.
const PICTURE_RADIUS: f64 = 14.0;
/// The stage's height, in points.
const TRANSFORM_STAGE_HEIGHT: f64 = 244.0;
/// How small and how large a pinch may make the picture.
const SCALE_RANGE: (f64, f64) = (0.5, 2.5);
/// The spring that takes the picture home.
const HOMING: Spring = Spring::new(190.0, 21.0);

/// The picture's place on the stage: an offset, a scale and an angle in degrees, each a value
/// a gesture sets and a spring takes home.
#[derive(Clone, Copy)]
struct Placement {
    x: Animated,
    y: Animated,
    scale: Animated,
    angle: Animated,
}

impl Placement {
    /// The picture at home.
    fn new() -> Self {
        Placement {
            x: Animated::new(0.0),
            y: Animated::new(0.0),
            scale: Animated::new(1.0),
            angle: Animated::new(0.0),
        }
    }

    /// Stops every value where it is: what a gesture does before it takes over.
    fn hold(self) {
        for value in [self.x, self.y, self.scale, self.angle] {
            value.set(value.get_untracked());
        }
    }

    /// Springs every value home together. Under reduced motion they land at once.
    fn home(self) {
        Animation::parallel([
            Animation::spring(self.x, 0.0, HOMING),
            Animation::spring(self.y, 0.0, HOMING),
            Animation::spring(self.scale, 1.0, HOMING),
            Animation::spring(self.angle, 0.0, HOMING),
        ])
        .start();
    }
}

/// A picture of Saturn that follows a drag, a pinch and a rotation at once, and springs home on
/// Reset.
///
/// Each gesture reports a cumulative value (the drag's translation, the pinch's scale, the
/// rotation's angle), so each handler applies it to the value it captured when the gesture
/// began; a second gesture then continues from where the first left the picture. A pinch or a
/// rotation that arrives as lone `Changed` ticks (a ctrl+wheel, a trackpad without a gesture
/// session) carries each tick's own amount and is applied to the current value instead.
fn transform_card() -> AnyPiece {
    let picture = Placement::new();
    let stage = Rc::new(Cell::new(Size::new(0.0, 0.0)));
    // Each handler owns its starting value for the duration of one gesture.
    let drag_origin = Cell::new(Point::new(0.0, 0.0));
    let pinch_origin = Cell::new(None);
    let rotation_origin = Cell::new(None);

    let sized = stage.clone();
    let backdrop = canvas(move |draw: &mut Draw, size: Size| {
        sized.set(size);
        gallery::dotted_stage(draw, size, 16.0);
        let home = Rect::new(
            (size.width - PICTURE.0) / 2.0,
            (size.height - PICTURE.1) / 2.0,
            PICTURE.0,
            PICTURE.1,
        );
        draw.stroke_styled(
            Shape::RoundedRect(home, PICTURE_RADIUS),
            SystemColor::TertiaryText,
            StrokeStyle::dashed(1.0, vec![3.0, 4.0]),
        );
    })
    .a11y(|a| a.decorative())
    .height(TRANSFORM_STAGE_HEIGHT)
    .grow(Dimension::Width);

    let shown = day::http_image(sky::url(sky::SATURN, sky::Size::Small))
        .rounded(PICTURE_RADIUS)
        .a11y(|a| a.decorative())
        .frame(PICTURE.0, PICTURE.1)
        .transform(move || Transform {
            tx: picture.x.get(),
            ty: picture.y.get(),
            sx: picture.scale.get(),
            sy: picture.scale.get(),
            rotation: Angle::deg(picture.angle.get()),
            anchor_x: 0.5,
            anchor_y: 0.5,
        });

    let on_drag = move |drag: Drag| {
        if drag.phase == DragPhase::Began {
            picture.hold();
            drag_origin.set(Point::new(
                picture.x.get_untracked(),
                picture.y.get_untracked(),
            ));
        }
        // The picture's center stays on the stage, so it can always be picked up again.
        let size = stage.get();
        let reach_x = (size.width / 2.0).max(PICTURE.0 / 4.0);
        let reach_y = (size.height / 2.0).max(PICTURE.1 / 4.0);
        let origin = drag_origin.get();
        picture
            .x
            .set((origin.x + drag.translation.x).clamp(-reach_x, reach_x));
        picture
            .y
            .set((origin.y + drag.translation.y).clamp(-reach_y, reach_y));
    };
    let on_pinch = move |pinch: Pinch| {
        let from = match pinch.phase {
            DragPhase::Began => {
                picture.hold();
                let now = picture.scale.get_untracked();
                pinch_origin.set(Some(now));
                now
            }
            DragPhase::Changed => pinch_origin
                .get()
                .unwrap_or_else(|| picture.scale.get_untracked()),
            DragPhase::Ended => {
                pinch_origin.set(None);
                return;
            }
        };
        picture
            .scale
            .set((from * pinch.scale).clamp(SCALE_RANGE.0, SCALE_RANGE.1));
    };
    let on_rotate = move |turn: Rotate| {
        let from = match turn.phase {
            DragPhase::Began => {
                picture.hold();
                let now = picture.angle.get_untracked();
                rotation_origin.set(Some(now));
                now
            }
            DragPhase::Changed => rotation_origin
                .get()
                .unwrap_or_else(|| picture.angle.get_untracked()),
            DragPhase::Ended => {
                rotation_origin.set(None);
                return;
            }
        };
        picture.angle.set(from + turn.rotation.degrees());
    };
    // The top layer: a canvas that draws nothing and takes the three gestures.
    let surface = canvas(|_: &mut Draw, _: Size| {})
        .on_drag(on_drag)
        .on_pinch(on_pinch)
        .on_rotate(on_rotate)
        .a11y(|a| a.label(crate::res::str::gestures_transform_stage_label()))
        .id(day::id!(gestures_transform_stage))
        .height(TRANSFORM_STAGE_HEIGHT)
        .grow(Dimension::Width);

    card(
        crate::res::vectors::nav_gestures,
        crate::res::str::gestures_transform_title(),
        crate::res::str::gestures_transform_subtitle(),
        column((
            zstack((backdrop, shown, surface))
                .corner_radius(12.0)
                .grow(Dimension::Width),
            row((
                reading(
                    crate::res::str::gestures_transform_offset(),
                    day::id!(gestures_transform_offset),
                    move || coordinates(Point::new(picture.x.get(), picture.y.get())),
                ),
                reading(
                    crate::res::str::gestures_transform_scale(),
                    day::id!(gestures_transform_scale),
                    move || format!("{:.2}×", picture.scale.get()),
                ),
                reading(
                    crate::res::str::gestures_transform_angle(),
                    day::id!(gestures_transform_angle),
                    move || format!("{}°", whole(picture.angle.get())),
                ),
            ))
            .grow(Dimension::Width),
            note(worded(
                crate::res::str::gestures_transform_hint_touch,
                crate::res::str::gestures_transform_hint_pointer,
            )),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .trailing(
        button(crate::res::str::gestures_reset())
            .bordered()
            .action(move || picture.home())
            .id(day::id!(gestures_reset)),
    )
    .id(day::id!(gestures_transform))
    .build()
}

// ---------------------------------------------------------------------------
// Pointer
// ---------------------------------------------------------------------------

/// How many drag samples the trail keeps.
const TRAIL: usize = 36;
/// How long a release's arrow is for each point per second of speed, in seconds, and the
/// longest it is drawn, in points.
const ARROW: (f64, f64) = (0.14, 120.0);

/// What the pointer card last heard.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PointerPhase {
    /// A drag began.
    Began,
    /// The drag moved.
    Changed,
    /// The drag ended.
    Ended,
    /// The pointer moved over the stage with nothing pressed.
    Hover,
    /// A press and release that Day reported as a tap.
    Tap,
}

impl PointerPhase {
    /// The phase's name in the readout.
    fn name(self) -> day::LocalizedText {
        match self {
            PointerPhase::Began => crate::res::str::gestures_pointer_began(),
            PointerPhase::Changed => crate::res::str::gestures_pointer_changed(),
            PointerPhase::Ended => crate::res::str::gestures_pointer_ended(),
            PointerPhase::Hover => crate::res::str::gestures_pointer_hover(),
            PointerPhase::Tap => crate::res::str::gestures_pointer_tap(),
        }
    }
}

/// Draws an arrow from `from` along `velocity`, as long as the speed says.
fn draw_arrow(draw: &mut Draw, from: Point, velocity: Point, color: Color) {
    let speed = velocity.x.hypot(velocity.y);
    if speed < 1.0 {
        return;
    }
    let length = (speed * ARROW.0).min(ARROW.1);
    let (ux, uy) = (velocity.x / speed, velocity.y / speed);
    let tip = Point::new(from.x + ux * length, from.y + uy * length);
    draw.stroke_styled(Shape::Line(from, tip), color, StrokeStyle::round(2.5));
    // The head: two short strokes back from the tip, 28° either side of the shaft.
    for side in [-1.0_f64, 1.0] {
        let (sin, cos) = (side * 28.0_f64.to_radians()).sin_cos();
        let back = Point::new(
            tip.x - 10.0 * (ux * cos - uy * sin),
            tip.y - 10.0 * (ux * sin + uy * cos),
        );
        draw.stroke_styled(Shape::Line(tip, back), color, StrokeStyle::round(2.5));
    }
}

/// A stage that shows what `on_drag`, `on_touches`, `on_tap_at` and `on_hover` report: a trail
/// of the drag's samples, an arrow for the velocity it was released with, a ring under every
/// contact, a mark where a tap landed and a ring under a hovering pointer, with the newest
/// report in figures underneath.
fn pointer_card() -> AnyPiece {
    let phase: Signal<Option<PointerPhase>> = Signal::new(None);
    let location = Signal::new(Point::new(0.0, 0.0));
    let translation = Signal::new(Point::new(0.0, 0.0));
    let speed = Signal::new(0.0_f64);
    let pressed = Signal::new(false);
    let hover: Signal<Option<Point>> = Signal::new(None);
    let tapped: Signal<Option<Point>> = Signal::new(None);
    // The drag's newest samples, oldest first.
    let trail: Signal<VecDeque<Point>> = Signal::new(VecDeque::new());
    // The velocity the last drag was released with.
    let thrown: Signal<Option<Point>> = Signal::new(None);
    // The contacts down on the stage: id, where, how hard.
    let contacts: Signal<Vec<(u64, Point, f64)>> = Signal::new(Vec::new());

    let backdrop = canvas(|draw: &mut Draw, size: Size| gallery::dotted_stage(draw, size, 16.0))
        .a11y(|a| a.decorative())
        .height(176.0)
        .grow(Dimension::Width);
    let hint = when(
        move || phase.get().is_none(),
        || {
            label(worded(
                crate::res::str::gestures_pointer_hint_touch,
                crate::res::str::gestures_pointer_hint_pointer,
            ))
            .font(Font::Subheadline)
            .secondary()
        },
    );
    // The top layer draws the marks and takes the gestures.
    let surface = canvas(move |draw: &mut Draw, _: Size| {
        let accent = Tone::Accent.color();
        trail.with(|trail| {
            let count = trail.len().max(1) as f64;
            for (index, at) in trail.iter().enumerate() {
                let newness = (index + 1) as f64 / count;
                draw.fill(
                    disc(*at, 1.5 + 3.5 * newness),
                    accent.with_alpha(0.12 + 0.6 * newness),
                );
            }
            if let (Some(from), Some(velocity)) = (trail.back(), thrown.get()) {
                draw_arrow(draw, *from, velocity, Tone::Info.color());
            }
        });
        contacts.with(|contacts| {
            for (_, at, pressure) in contacts {
                let radius = 18.0 + 10.0 * pressure.clamp(0.0, 1.0);
                draw.fill(disc(*at, radius), Tone::Info.tint());
                draw.stroke(disc(*at, radius), Tone::Info.color(), 2.0);
            }
        });
        if pressed.get() {
            let at = location.get();
            draw.fill(disc(at, 22.0), Tone::Accent.tint());
            draw.stroke(disc(at, 22.0), accent, 2.0);
        } else if let Some(at) = hover.get() {
            draw.stroke(disc(at, 22.0), SystemColor::TertiaryText, 1.5);
        }
        if let Some(at) = tapped.get() {
            draw.stroke(disc(at, 9.0), Tone::Up.color(), 2.0);
            draw.fill(disc(at, 3.0), Tone::Up.color());
        }
    })
    .on_drag(move |drag: Drag| {
        location.set(drag.location);
        translation.set(drag.translation);
        speed.set(drag.speed());
        match drag.phase {
            DragPhase::Began => {
                trail.set(VecDeque::from([drag.start()]));
                thrown.set(None);
                tapped.set(None);
                pressed.set(true);
            }
            DragPhase::Changed => {}
            DragPhase::Ended => {
                pressed.set(false);
                thrown.set(Some(drag.velocity));
            }
        }
        if drag.phase != DragPhase::Ended {
            trail.update(|trail| {
                trail.push_back(drag.location);
                while trail.len() > TRAIL {
                    trail.pop_front();
                }
            });
        }
        phase.set_if_changed(Some(match drag.phase {
            DragPhase::Began => PointerPhase::Began,
            DragPhase::Changed => PointerPhase::Changed,
            DragPhase::Ended => PointerPhase::Ended,
        }));
    })
    .on_touches(move |touches: &[Touch]| {
        contacts.set(
            touches
                .iter()
                .filter(|touch| !matches!(touch.phase, TouchPhase::Ended | TouchPhase::Cancelled))
                .map(|touch| (touch.id, touch.location, touch.pressure))
                .collect(),
        );
    })
    .on_tap_at(move |at: Point| {
        tapped.set(Some(at));
        location.set(at);
        phase.set(Some(PointerPhase::Tap));
    })
    .on_hover(move |at: Option<Point>| {
        hover.set(at);
        if let (Some(at), false) = (at, pressed.get_untracked()) {
            location.set(at);
            phase.set_if_changed(Some(PointerPhase::Hover));
        }
    })
    .a11y(|a| a.label(crate::res::str::gestures_pointer_stage_label()))
    .id(day::id!(gestures_pointer_stage))
    .height(176.0)
    .grow(Dimension::Width);

    // Every finger, where the platform reports them; one contact where Day derives the list from
    // the drag (docs/gestures.md "Touches").
    let fingers = day::gesture_support(GestureKind::Touches) == Support::Native;
    card(
        crate::res::vectors::nav_cursor,
        crate::res::str::gestures_pointer_title(),
        crate::res::str::gestures_pointer_subtitle(),
        column((
            zstack((backdrop, hint, surface))
                .corner_radius(12.0)
                .grow(Dimension::Width),
            row((
                reading(
                    crate::res::str::gestures_pointer_phase(),
                    day::id!(gestures_pointer_phase),
                    move || {
                        phase
                            .get()
                            .map_or_else(crate::res::str::gestures_pointer_waiting, |phase| {
                                phase.name()
                            })
                            .format()
                    },
                ),
                reading(
                    crate::res::str::gestures_pointer_location(),
                    day::id!(gestures_pointer_location),
                    move || coordinates(location.get()),
                ),
                reading(
                    crate::res::str::gestures_pointer_contacts(),
                    day::id!(gestures_pointer_contacts),
                    move || contacts.with(Vec::len).to_string(),
                ),
            ))
            .grow(Dimension::Width),
            row((
                reading(
                    crate::res::str::gestures_pointer_translation(),
                    day::id!(gestures_pointer_translation),
                    move || coordinates(translation.get()),
                ),
                reading(
                    crate::res::str::gestures_pointer_speed(),
                    day::id!(gestures_pointer_speed),
                    move || {
                        crate::res::str::gestures_pointer_speed_value(whole(speed.get())).format()
                    },
                ),
            ))
            .grow(Dimension::Width),
            (!fingers).then(|| note(crate::res::str::gestures_pointer_one_contact())),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .id(day::id!(gestures_pointer))
    .build()
}

// ---------------------------------------------------------------------------
// Tap, double tap and hold
// ---------------------------------------------------------------------------

/// How long the ring takes to close, in milliseconds: the long-press delay where Day derives the
/// gesture. A platform recognizer may fire a little sooner or later; the ring closes then.
const HOLD_MS: u32 = 500;

/// A pad with Day's press gestures on it: `on_tap`, `on_double_tap` and `on_long_press` count,
/// `on_pressed` fills a ring toward a hold, and an `InteractionState` colors the pad while it
/// is hovered or pressed. A single tap waits out the double-tap interval, so the two never
/// both count, and a long press's release is not a tap.
fn press_card() -> AnyPiece {
    let taps = Signal::new(0_i64);
    let doubles = Signal::new(0_i64);
    let holds = Signal::new(0_i64);
    let state = InteractionState::new();
    // How far the press under way is toward a hold, 0 to 1.
    let fill = Animated::new(0.0_f64);

    let ring = canvas(move |draw: &mut Draw, size: Size| {
        let center = Point::new(size.width / 2.0, size.height / 2.0);
        let ring = Rect::new(center.x - 26.0, center.y - 26.0, 52.0, 52.0);
        let filled = fill.get().clamp(0.0, 1.0);
        let color = if filled >= 1.0 {
            Tone::Up.color()
        } else {
            Tone::Accent.color()
        };
        draw.stroke(Shape::Ellipse(ring), SystemColor::Separator, 5.0);
        if filled > 0.0 {
            draw.stroke_styled(
                Shape::Arc {
                    rect: ring,
                    start: Angle::deg(-90.0),
                    sweep: Angle::FULL * filled,
                },
                color,
                StrokeStyle::round(5.0),
            );
        }
        draw.fill(disc(center, 6.0), color.with_alpha(0.25 + 0.75 * filled));
    })
    .a11y(|a| a.decorative())
    .height(112.0)
    .grow(Dimension::Width);

    let pad = zstack((ring,))
        .surface(move || {
            let (fill, rim) = if state.pressed() {
                (Tone::Accent.tint(), Ink::Fixed(Tone::Accent.color()))
            } else if state.hovered() {
                (
                    Ink::system(SystemColor::ControlFill),
                    Ink::system(SystemColor::Separator),
                )
            } else {
                (gallery::RAISED, Ink::system(SystemColor::Separator))
            };
            Surface::new()
                .fill(fill)
                .corner_radius(12.0)
                .border(rim, 1.0)
        })
        .interaction(state)
        .on_pressed(move |down| {
            if down {
                fill.set(0.0);
                // Under reduced motion the ring closes when the hold fires, not before.
                if !day::reduce_motion() {
                    fill.animate_to(1.0, AnimSpec::linear(HOLD_MS));
                }
            } else {
                fill.animate_to(0.0, AnimSpec::ease_out(260));
            }
        })
        .on_tap(move || taps.update(|taps| *taps += 1))
        .on_double_tap(move || doubles.update(|doubles| *doubles += 1))
        .on_long_press(move |_at| {
            holds.update(|holds| *holds += 1);
            fill.set(1.0);
        })
        .a11y(move |a| {
            a.role(Role::Button)
                .label(crate::res::str::gestures_press_pad_label())
                .hint(worded(
                    crate::res::str::gestures_press_pad_touch,
                    crate::res::str::gestures_press_pad_pointer,
                )())
        })
        .id(day::id!(gestures_press_pad))
        .grow(Dimension::Width);

    // Where Day derives the press from drag events and the platform's drag begins only once
    // the pointer moves, a press held perfectly still never starts; the pad says so there.
    let derived_note = (day::gesture_support(GestureKind::Press) != Support::Native)
        .then(|| note(crate::res::str::gestures_press_derived_note()));

    card(
        crate::res::vectors::nav_controls,
        crate::res::str::gestures_press_title(),
        crate::res::str::gestures_press_subtitle(),
        column((
            pad,
            row((
                reading(
                    crate::res::str::gestures_press_taps(),
                    day::id!(gestures_press_count),
                    move || taps.get().to_string(),
                ),
                reading(
                    crate::res::str::gestures_press_doubles(),
                    day::id!(gestures_press_doubles),
                    move || doubles.get().to_string(),
                ),
                reading(
                    crate::res::str::gestures_press_holds(),
                    day::id!(gestures_press_holds),
                    move || holds.get().to_string(),
                ),
            ))
            .grow(Dimension::Width),
            note(worded(
                crate::res::str::gestures_press_pad_touch,
                crate::res::str::gestures_press_pad_pointer,
            )),
            derived_note,
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .id(day::id!(gestures_press))
    .build()
}

// ---------------------------------------------------------------------------
// Swipe
// ---------------------------------------------------------------------------

/// How far a row must be swiped, in points, or how fast it must be let go, in points per
/// second, to count as a swipe.
const SWIPE: (f64, f64) = (96.0, 700.0);
/// The spring a swiped row comes back on, from the speed it was let go at.
const SWIPE_BACK: Spring = Spring::new(320.0, 24.0);

/// A row that swipes sideways inside the page's vertical scroll (`on_drag_along`): far enough or
/// fast enough counts, and the row springs back from the velocity it was let go with.
fn swipe_card() -> AnyPiece {
    let offset = Animated::new(0.0_f64);
    let swipes = Signal::new(0_i64);
    let release = Signal::new(0.0_f64);
    let armed = move || offset.get().abs() >= SWIPE.0;

    let under = row((
        vector(crate::res::vectors::nav_feeds)
            .tint(Tone::Up.color())
            .frame(20.0, 20.0),
        spacer(),
        vector(crate::res::vectors::nav_feeds)
            .tint(Tone::Up.color())
            .frame(20.0, 20.0),
    ))
    .align(VAlign::Center)
    .padding(Insets::symmetric(Space::L.into(), 0.0))
    .grow(Dimension::Both)
    .background(move || {
        if armed() {
            Tone::Up.tint()
        } else {
            gallery::INSET
        }
    });
    let front = row((
        icon_mark(),
        column((
            label(crate::res::str::gestures_swipe_row_title()).font(Font::Body),
            label(worded(
                crate::res::str::gestures_swipe_row_touch,
                crate::res::str::gestures_swipe_row_pointer,
            ))
            .font(Font::Footnote)
            .secondary(),
        ))
        .spacing(1.0)
        .align(HAlign::Leading)
        .grow(Dimension::Width),
    ))
    .spacing(Space::M)
    .align(VAlign::Center)
    .padding(Space::M)
    .grow(Dimension::Width)
    .background(SystemColor::Background)
    .translation(offset, 0.0);
    let surface = zstack((under, front))
        .corner_radius(12.0)
        .border(SystemColor::Separator, 1.0)
        .on_drag_along(DragAxis::Horizontal, move |drag: Drag| match drag.phase {
            DragPhase::Began | DragPhase::Changed => offset.set(drag.translation.x),
            DragPhase::Ended => {
                release.set(drag.velocity.x.abs().round());
                if drag.translation.x.abs() >= SWIPE.0 || drag.velocity.x.abs() >= SWIPE.1 {
                    swipes.update(|swipes| *swipes += 1);
                }
                offset.set(drag.translation.x);
                offset.set_velocity(drag.velocity.x);
                Animation::spring(offset, 0.0, SWIPE_BACK).start();
            }
        })
        .a11y(|a| a.label(crate::res::str::gestures_swipe_row_title()))
        .id(day::id!(gestures_swipe_row))
        .grow(Dimension::Width);

    card(
        crate::res::vectors::nav_feeds,
        crate::res::str::gestures_swipe_title(),
        crate::res::str::gestures_swipe_subtitle(),
        column((
            surface,
            row((
                reading(
                    crate::res::str::gestures_swipe_count(),
                    day::id!(gestures_swipe_count),
                    move || swipes.get().to_string(),
                ),
                reading(
                    crate::res::str::gestures_swipe_release(),
                    day::id!(gestures_swipe_release),
                    move || {
                        crate::res::str::gestures_pointer_speed_value(whole(release.get())).format()
                    },
                ),
            ))
            .grow(Dimension::Width),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .id(day::id!(gestures_swipe))
    .build()
}

/// The swiped row's leading mark.
fn icon_mark() -> impl Piece {
    zstack((vector(crate::res::vectors::nav_gestures)
        .tint(Tone::Accent.color())
        .frame(18.0, 18.0),))
    .frame(34.0, 34.0)
    .background(Tone::Accent.tint())
    .corner_radius(Corner::CAPSULE)
}

// ---------------------------------------------------------------------------
// Hit testing
// ---------------------------------------------------------------------------

/// The dot's diameter, in points.
const HIT_DOT: f64 = 14.0;
/// How far past its own edge the dot takes a tap, in points.
const HIT_SLOP: f64 = 25.0;

/// Two specimens of where a tap lands: a small dot whose `.hit_slop` takes taps well outside
/// what it draws, and a native button that takes them through a canvas drawn over it.
///
/// docs/gestures.md "Pass-through": a piece with no handler takes no pointer input, so a tap on
/// the canvas over the button reaches the button.
fn hit_card() -> AnyPiece {
    let taps = Signal::new(0_i64);
    let presses = Signal::new(0_i64);
    // The dot's scale: it dips on a tap and springs back.
    let dip = Animated::new(1.0_f64);
    let slop = capability(Cap::HitSlop) != Support::Unsupported;
    let area = HIT_DOT + 2.0 * HIT_SLOP;

    let outline = canvas(move |draw: &mut Draw, size: Size| {
        gallery::dotted_stage(draw, size, 16.0);
        if !slop {
            return;
        }
        let ring = disc(Point::new(size.width / 2.0, size.height / 2.0), area / 2.0);
        draw.fill(ring.clone(), Tone::Warn.tint());
        draw.stroke_styled(
            ring,
            Tone::Warn.color(),
            StrokeStyle::dashed(1.5, vec![3.0, 4.0]),
        );
    })
    .a11y(|a| a.decorative())
    .height(104.0)
    .grow(Dimension::Width);
    // The dot is its own 14 points in layout; the slop grows only where it takes a tap.
    let target = circle()
        .fill(Tone::Danger.color())
        .frame(HIT_DOT, HIT_DOT)
        .scale(dip)
        .hit_slop(HIT_SLOP)
        .on_tap(move || {
            taps.update(|taps| *taps += 1);
            // The dip is decoration: under reduced motion the dot stays still.
            if !day::reduce_motion() {
                dip.set(0.7);
                Animation::spring(dip, 1.0, Spring::new(460.0, 18.0)).start();
            }
        })
        .a11y(|a| {
            a.role(Role::Button)
                .label(crate::res::str::gestures_slop_target_label())
        })
        .id(day::id!(gestures_slop_target));

    let overlay = canvas(|draw: &mut Draw, size: Size| {
        let band = Rect::new(
            size.width * 0.2,
            2.0,
            size.width * 0.6,
            (size.height - 4.0).max(0.0),
        );
        let color = Tone::Info.color();
        draw.clipped(Shape::RoundedRect(band, 8.0), |draw| {
            gallery::hatch(draw, band, 9.0, color.with_alpha(0.3));
        });
        draw.stroke_styled(
            Shape::RoundedRect(band, 8.0),
            color,
            StrokeStyle::dashed(1.0, vec![3.0, 4.0]),
        );
    })
    .a11y(|a| a.decorative())
    .height(60.0)
    .grow(Dimension::Width);

    let labeled_count = |text: day::LocalizedText, badge: AnyPiece| {
        row((note(text), badge))
            .spacing(Space::S)
            .align(VAlign::Center)
            .grow(Dimension::Width)
    };
    let slop_caption = if slop {
        crate::res::str::gestures_slop_caption(crate::res::str::GesturesSlopCaption {
            dot: whole(HIT_DOT),
            area: whole(area),
        })
    } else {
        crate::res::str::gestures_slop_unsupported(whole(HIT_DOT))
    };
    card(
        crate::res::vectors::record_dot,
        crate::res::str::gestures_hit_title(),
        crate::res::str::gestures_hit_subtitle(),
        column((
            zstack((outline, target))
                .corner_radius(12.0)
                .grow(Dimension::Width),
            labeled_count(
                slop_caption,
                count_badge(
                    day::id!(gestures_slop_count),
                    move || crate::res::str::gestures_slop_count(taps.get()).format(),
                    taps,
                )
                .any(),
            ),
            divider(),
            zstack((
                button(crate::res::str::gestures_through_button())
                    .bordered()
                    .action(move || presses.update(|presses| *presses += 1))
                    .id(day::id!(gestures_through_button))
                    .grow(Dimension::Width),
                overlay,
            ))
            .grow(Dimension::Width),
            labeled_count(
                crate::res::str::gestures_through_caption(),
                count_badge(
                    day::id!(gestures_through_count),
                    move || presses.get().to_string(),
                    presses,
                )
                .any(),
            ),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .id(day::id!(gestures_hit))
    .build()
}

// ---------------------------------------------------------------------------
// The page
// ---------------------------------------------------------------------------

/// The Gestures page: five cards, dealt left and right in this order.
pub(crate) fn gestures_page() -> AnyPiece {
    card_page(
        crate::res::str::nav_gestures(),
        day::id!(gestures_title),
        2,
        vec![
            transform_card,
            pointer_card,
            press_card,
            swipe_card,
            hit_card,
        ],
    )
}
