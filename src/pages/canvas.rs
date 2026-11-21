// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The Canvas page (docs/canvas.md): one card per drawing, each on a `canvas` that reads the
//! gallery's colors, so it follows light and dark.
//!
//! The first five cards are a dial on a spring, a clock, parsed SVG paths, compositing (clips,
//! shadows, erased cut-outs and blend modes), and a sketch pad. Their motion is stepped from
//! `day::frame::FrameClock` with `day::motion` and written into signals the draw closures
//! read. The last four cards are the reference demos for canvas text, text anchors, pointer
//! input and decoded images. Charts are on the Charts page (`charts.rs`), built with
//! day-piece-charts.

use std::cell::{Cell, RefCell};
use std::ops::ControlFlow;
use std::rc::Rc;

use day::frame::{FrameClock, FrameHandle};
use day::motion::{Spring, SpringState};
use day::prelude::*;

use crate::gallery::{self, Tone, card, card_page};
use crate::palette::{AZURE, CORAL, TEAL};

/// The Canvas page: the five drawings, then the text, anchor, input and image demos.
pub(crate) fn canvas_page() -> AnyPiece {
    card_page(
        crate::res::str::nav_canvas(),
        day::id!(canvas_title),
        3,
        vec![
            dial_card,
            clock_card,
            svg_card,
            compositing_card,
            sketch_card,
            text_card,
            anchors_card,
            interaction_card,
            image_card,
        ],
    )
}

// ---------------------------------------------------------------------------
// Shared drawing helpers
// ---------------------------------------------------------------------------

/// The margin between a stage's edge and the drawing on it.
const PAD: f64 = 12.0;

/// The size of a chart's small labels, in canvas points.
const SMALL: f64 = 11.0;

/// The opaque color of a stage: the kit's recessed wash over the platform's surface color.
///
/// Opaque, so a shape filled with it hides what is under it: a ridge that covers the one
/// behind it, a dot with a hollow middle, the base of a disc that casts a shadow.
///
/// Mixed from two inks, so it reads them with `day::resolve`: a canvas that calls it records
/// again when the appearance changes.
fn stage_color() -> Color {
    let base = day::resolve(SystemColor::Surface);
    let wash = day::resolve(gallery::INSET);
    let over = |under: f64, top: f64| under * (1.0 - wash.a) + top * wash.a;
    Color::rgb(
        over(base.r, wash.r),
        over(base.g, wash.g),
        over(base.b, wash.b),
    )
}

/// Fills `size` with the stage and runs `content` inside its margin, with the size left.
fn staged(draw: &mut Draw, size: Size, content: impl FnOnce(&mut Draw, Size)) {
    draw.fill(
        Shape::RoundedRect(Rect::new(0.0, 0.0, size.width, size.height), 12.0),
        stage_color(),
    );
    margined(draw, size, content);
}

/// Runs `content` inside a stage's margin, with the size left, and paints no stage: for a
/// canvas whose stage is the view behind it, so what the drawing erases shows the stage.
fn margined(draw: &mut Draw, size: Size, content: impl FnOnce(&mut Draw, Size)) {
    let inner = Size::new(
        (size.width - 2.0 * PAD).max(1.0),
        (size.height - 2.0 * PAD).max(1.0),
    );
    draw.transformed(Affine::translate(PAD, PAD), |draw| content(draw, inner));
}

/// The color of a shadow: black, stronger in dark where a shadow has less to darken.
fn shadow_color(light: f64, dark: f64) -> Ink {
    Ink::adaptive(
        Color::rgba(0.08, 0.08, 0.04, light),
        Color::rgba(0.0, 0.0, 0.0, dark),
    )
}

/// One line of text in the platform's face, hung on `at` by the anchor (`h`, `v`).
fn text(
    draw: &mut Draw,
    text: &str,
    at: Point,
    (size, color): (f64, impl Into<Ink>),
    (h, v): (TextAlign, TextVAlign),
) {
    draw.text(
        text,
        at,
        TextStyle {
            size,
            color: color.into(),
            anchor: TextAnchor { h, v },
            font: CanvasFont::default(),
        },
    );
}

/// A circle of radius `r` around `center`.
fn disc(center: Point, r: f64) -> Shape {
    Shape::Ellipse(Rect::new(center.x - r, center.y - r, r * 2.0, r * 2.0))
}

/// The point `r` from `center` at `deg` (0 points right, 90 points down).
fn polar(center: Point, r: f64, deg: f64) -> Point {
    let (sin, cos) = deg.to_radians().sin_cos();
    Point::new(center.x + r * cos, center.y + r * sin)
}

/// An open path through `points`.
fn open_path(points: &[Point]) -> Shape {
    let mut path = PathBuilder::new();
    for (index, point) in points.iter().enumerate() {
        path = if index == 0 {
            path.move_to(*point)
        } else {
            path.line_to(*point)
        };
    }
    path.build()
}

/// A whole number with thousands separators.
fn grouped(value: i64) -> String {
    let digits = value.abs().to_string();
    let mut out = String::new();
    if value < 0 {
        out.push('-');
    }
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

// ---------------------------------------------------------------------------
// Shared parts
// ---------------------------------------------------------------------------

/// A fact about a drawing with the mechanism behind it underneath.
fn fact(name: day::LocalizedText, note: day::LocalizedText) -> impl Piece {
    column((
        label(name).font(Font::Footnote),
        label(note).font(Font::Caption).secondary(),
    ))
    .spacing(1.0)
    .align(HAlign::Leading)
}

/// The body of a card: its pieces in a leading-aligned column.
fn body(pieces: impl PieceSeq) -> impl Piece {
    column(pieces).spacing(Space::M).align(HAlign::Leading)
}

/// Decodes the Day logo's bytes and puts the bitmap in `shown`. `readout`, when given, is told
/// what the decoder said the bytes were, or that it could not read them.
fn decode_logo(shown: Signal<Option<day::Bitmap>>, readout: Option<Signal<String>>) {
    // Started while the card builds: a backend that decodes inline has its completion drained
    // when the tree borrow ends rather than being lost.
    day::task(async move {
        match day::decode_image(std::sync::Arc::new(
            crate::res::image_bytes::day_logo.to_vec(),
        ))
        .await
        {
            Ok(bitmap) => {
                if let Some(readout) = readout {
                    let info = bitmap.info();
                    readout.set(
                        crate::res::str::canvas_image_readout(
                            crate::res::str::CanvasImageReadout {
                                format: info.format.map(|f| f.mime()).unwrap_or("—"),
                                width: info.pixels.width,
                                height: info.pixels.height,
                            },
                        )
                        .format(),
                    );
                }
                shown.set(Some(bitmap));
            }
            Err(_) => {
                if let Some(readout) = readout {
                    readout.set(crate::res::str::canvas_image_failed().format());
                }
            }
        }
    });
}

/// Draws `bitmap` as large as fits `into`, centered, in the proportions its decoder reported.
fn draw_fitted(draw: &mut Draw, bitmap: &day::Bitmap, into: Rect) {
    let pixels = bitmap.info().pixels;
    if pixels.width <= 0.0 || pixels.height <= 0.0 {
        return;
    }
    let scale = (into.size.width / pixels.width).min(into.size.height / pixels.height);
    let (w, h) = (pixels.width * scale, pixels.height * scale);
    draw.image(
        bitmap,
        Rect::new(
            into.origin.x + (into.size.width - w) / 2.0,
            into.origin.y + (into.size.height - h) / 2.0,
            w,
            h,
        ),
    );
}

// ---------------------------------------------------------------------------
// 1. The dial
// ---------------------------------------------------------------------------

/// Where the dial's scale starts, in degrees (lower left).
const DIAL_START: f64 = 135.0;
/// How far the dial's scale runs, in degrees (to lower right).
const DIAL_SWEEP: f64 = 270.0;
/// The load above which the turbine is past its limit.
const DIAL_LIMIT: f64 = 85.0;
/// The loads the three presets spring to: idle, cruise, peak.
const DIAL_PRESETS: [f64; 3] = [18.0, 57.0, 91.0];

/// "Turbine 3 load": a needle and a color sweep that follow one value on a damped spring.
fn dial_card() -> AnyPiece {
    let load = Signal::new(0.0_f64);
    let preset = Signal::new(1_usize);
    let state = Rc::new(Cell::new(SpringState::default()));
    let handle: Rc<RefCell<Option<FrameHandle>>> = Rc::default();
    let clock = FrameClock::current();
    // Springs the load to `target`. The spring's state outlives a retarget, so a preset chosen
    // mid-swing bends the motion instead of restarting it; the subscription pauses itself once
    // the spring has settled.
    let spring_to = {
        let (state, handle) = (state.clone(), handle.clone());
        move |spring: Spring, target: f64| {
            if untrack(day::reduce_motion) {
                handle.borrow_mut().take();
                state.set(SpringState::at(target));
                load.set(target);
                return;
            }
            let state = state.clone();
            let subscription = clock.subscribe(move |frame| {
                let mut now = state.get();
                spring.step(&mut now, target, frame.delta.as_secs_f64());
                let settled = spring.settled(now, target);
                if settled {
                    now = SpringState::at(target);
                }
                state.set(now);
                load.set(now.value);
                if settled {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            });
            *handle.borrow_mut() = Some(subscription);
        }
    };
    Scope::current().on_cleanup(move || {
        handle.borrow_mut().take();
    });
    spring_to(Spring::new(90.0, 9.0), DIAL_PRESETS[1]);
    watch(
        move || preset.get(),
        move |chosen: &usize, _| {
            let target = DIAL_PRESETS
                .get(*chosen)
                .copied()
                .unwrap_or(DIAL_PRESETS[1]);
            spring_to(Spring::new(140.0, 11.0), target);
        },
    );

    // The readings follow the number the dial shows, which is the sprung value on its scale.
    let shown = move || load.get().clamp(0.0, 100.0).round();
    let zone = move || {
        let load = shown();
        if load >= DIAL_LIMIT {
            (crate::res::str::canvas_dial_zone_redline(), Tone::Danger)
        } else if load >= 60.0 {
            (crate::res::str::canvas_dial_zone_high(), Tone::Warn)
        } else {
            (crate::res::str::canvas_dial_zone_nominal(), Tone::Up)
        }
    };
    card(
        crate::res::vectors::nav_benchmark,
        crate::res::str::canvas_dial_title(),
        crate::res::str::canvas_dial_subtitle(),
        body((
            canvas(move |draw, size| {
                staged(draw, size, |draw, size| draw_dial(draw, size, load.get()));
            })
            .height(230.0 + 2.0 * PAD)
            .grow(Dimension::Width)
            .a11y(move |a| {
                a.role(Role::Meter)
                    .label(crate::res::str::canvas_dial_title())
                    .value(move || format!("{:.0}", shown()))
            })
            .id(day::id!(canvas_dial)),
            picker(
                preset,
                [
                    (0_usize, crate::res::str::canvas_dial_idle()),
                    (1, crate::res::str::canvas_dial_cruise()),
                    (2, crate::res::str::canvas_dial_peak()),
                ],
            )
            .segmented()
            .id(day::id!(canvas_dial_preset)),
            row((
                // The zone's tone changes with the load, which the kit's `stat` fixes when it
                // is built, so this cell is laid out the same way with a reactive color.
                column((
                    label(crate::res::str::canvas_dial_zone())
                        .font(Font::Caption)
                        .secondary(),
                    label(move || zone().0.format())
                        .font(Font::Callout)
                        .single_line()
                        .color(move || zone().1.color())
                        .id(day::id!(canvas_dial_zone)),
                ))
                .spacing(2.0)
                .align(HAlign::Leading)
                .grow(Dimension::Width),
                gallery::stat(
                    crate::res::str::canvas_dial_spool(),
                    move || {
                        crate::res::str::canvas_dial_spool_value(grouped(shown() as i64 * 128))
                            .format()
                    },
                    None,
                ),
                gallery::stat(
                    crate::res::str::canvas_dial_limit(),
                    format!("{DIAL_LIMIT:.0} %"),
                    Some(Tone::Danger),
                ),
            ))
            .grow(Dimension::Width),
        )),
    )
    .id(day::id!(canvas_card_dial))
    .build()
}

/// The dial at `value` (0 to 100): the hatched red zone, the ticks and their figures, the
/// sweep, the readout, and the needle over its hub.
fn draw_dial(draw: &mut Draw, size: Size, value: f64) {
    // A loose spring overshoots; the needle stops at the ends of its scale.
    let value = value.clamp(0.0, 100.0);
    let center = Point::new(size.width / 2.0, 0.54 * size.height);
    let r = (size.width / 2.0 - 8.0).min(0.5 * size.height) - 6.0;
    if r < 36.0 {
        return;
    }
    let at = |v: f64| DIAL_START + v / 100.0 * DIAL_SWEEP;
    let (ink, muted, faint) = (
        SystemColor::Text,
        SystemColor::SecondaryText,
        SystemColor::TertiaryText,
    );
    let danger = Tone::Danger.color();

    // The red zone: hatching confined to the ring between the limit and the end of the scale.
    let zone = PathBuilder::new()
        .arc_to(
            center,
            r + 2.0,
            Angle::deg(at(DIAL_LIMIT)),
            Angle::deg(at(100.0) - at(DIAL_LIMIT)),
        )
        .arc_to(
            center,
            r - 20.0,
            Angle::deg(at(100.0)),
            Angle::deg(at(DIAL_LIMIT) - at(100.0)),
        )
        .close()
        .build();
    draw.clipped(zone, |draw| {
        let area = Rect::new(center.x + 0.4 * r, center.y - 0.2 * r, 0.6 * r + 4.0, r);
        draw.fill(Shape::Rect(area), danger.with_alpha(0.08));
        gallery::hatch(draw, area, 5.0, danger.with_alpha(0.55));
    });

    // A tick every 2, a longer one with its figure every 20.
    for step in 0..=50 {
        let v = f64::from(step) * 2.0;
        let major = step % 10 == 0;
        let color = if v >= DIAL_LIMIT {
            danger.into()
        } else if major {
            Ink::from(muted)
        } else {
            faint.into()
        };
        let (inner, width) = if major {
            (r - 14.0, 2.0)
        } else {
            (r - 8.0, 1.0)
        };
        draw.stroke_styled(
            Shape::Line(polar(center, r, at(v)), polar(center, inner, at(v))),
            color,
            StrokeStyle::round(width),
        );
        if major {
            text(
                draw,
                &format!("{v:.0}"),
                polar(center, r - 28.0, at(v)),
                (SMALL, faint),
                (TextAlign::Center, TextVAlign::Middle),
            );
        }
    }

    // The sweep: a track, then the arc up to the needle, one stroke in a conic gradient. The
    // gradient is fixed to the dial, so a color stays at its angle as the arc grows.
    let ring = Rect::new(
        center.x - (r + 12.0),
        center.y - (r + 12.0),
        2.0 * (r + 12.0),
        2.0 * (r + 12.0),
    );
    draw.stroke_styled(
        Shape::Arc {
            rect: ring,
            start: Angle::deg(DIAL_START),
            sweep: Angle::deg(DIAL_SWEEP),
        },
        gallery::RAISED,
        StrokeStyle::round(5.0),
    );
    let swept = value / 100.0 * DIAL_SWEEP;
    if swept > 0.01 {
        // The colors meet in the middle of the gap at the foot of the dial, where nothing is
        // drawn. The ramp starts 0.2 rad before the scale does, and the first and last colors
        // hold flat on either side of it, so the round caps take the color of their own end.
        let seam = DIAL_START + DIAL_SWEEP + (360.0 - DIAL_SWEEP) / 2.0 - 360.0;
        let lead = (DIAL_START - 0.2_f64.to_degrees() - seam) / 360.0;
        let info = Tone::Info.color();
        draw.stroke_styled(
            Shape::Arc {
                rect: ring,
                start: Angle::deg(DIAL_START),
                sweep: Angle::deg(swept),
            },
            ConicGradient::centered(
                Angle::deg(seam),
                vec![
                    (0.0, info),
                    (lead, info),
                    (lead + 0.45, Tone::Accent.color()),
                    (lead + 0.75, Tone::Warn.color()),
                    (lead + 0.8, danger),
                    (1.0, danger),
                ],
            ),
            StrokeStyle::round(5.0),
        );
    }

    // The readout.
    draw.text(
        &format!("{value:.0}"),
        Point::new(center.x, center.y + 0.62 * r),
        TextStyle {
            size: 40.0,
            color: ink.into(),
            anchor: TextAnchor {
                h: TextAlign::Center,
                v: TextVAlign::Baseline,
            },
            font: CanvasFont {
                weight: Some(FontWeight::Semibold),
                ..CanvasFont::default()
            },
        },
    );
    text(
        draw,
        &crate::res::str::canvas_dial_unit().format(),
        Point::new(center.x, center.y + 0.62 * r + 18.0),
        (SMALL, faint),
        (TextAlign::Center, TextVAlign::Baseline),
    );

    // The needle, drawn pointing right and turned to its angle. It casts a shadow straight
    // down whichever way it points: a shadow's offset does not turn with the transform.
    draw.transformed(
        Affine::rotate(Angle::deg(at(value))).then(Affine::translate(center.x, center.y)),
        |draw| {
            draw.shadowed(
                Shadow::new(shadow_color(0.26, 0.55), 8.0).offset(0.0, 3.0),
                |draw| {
                    draw.fill(
                        Shape::Polygon(vec![
                            Point::new(-14.0, -3.0),
                            Point::new(r - 18.0, 0.0),
                            Point::new(-14.0, 3.0),
                        ]),
                        ink,
                    );
                },
            );
        },
    );
    // The hub: only its opaque base casts a shadow, so the wash and the rim over it add none.
    draw.shadowed(
        Shadow::new(shadow_color(0.24, 0.5), 6.0).offset(0.0, 2.0),
        |draw| draw.fill(disc(center, 7.0), stage_color()),
    );
    draw.fill(disc(center, 7.0), gallery::RAISED);
    draw.stroke(disc(center, 7.0), ink, 2.0);
}

// ---------------------------------------------------------------------------
// 2. The clock
// ---------------------------------------------------------------------------

/// The side of the clock's canvas.
const CLOCK_SIDE: f64 = 150.0;

/// "Transforms": a clock drawn by rotating the canvas, its second hand turning on the frame
/// clock.
fn clock_card() -> AnyPiece {
    // The seconds the hand shows: one turn a minute. With reduced motion it rests at twelve.
    let seconds = Signal::new(0.0_f64);
    if !untrack(day::reduce_motion) {
        let mut elapsed = 0.0;
        FrameClock::current()
            .subscribe(move |frame| {
                elapsed = (elapsed + frame.delta.as_secs_f64()) % 60.0;
                seconds.set(elapsed);
                ControlFlow::Continue(())
            })
            .in_scope();
    }
    card(
        crate::res::vectors::nav_dates,
        crate::res::str::canvas_clock_title(),
        crate::res::str::canvas_clock_subtitle(),
        row((
            canvas(move |draw, size| draw_clock(draw, size, seconds.get()))
                .frame(CLOCK_SIDE, CLOCK_SIDE)
                .a11y(|a| a.label(crate::res::str::canvas_clock_title()))
                .id(day::id!(canvas_clock)),
            column((
                fact(
                    crate::res::str::canvas_clock_ticks(),
                    crate::res::str::canvas_clock_ticks_note(),
                ),
                fact(
                    crate::res::str::canvas_clock_hands(),
                    crate::res::str::canvas_clock_hands_note(),
                ),
                fact(
                    crate::res::str::canvas_clock_second(),
                    crate::res::str::canvas_clock_second_note(),
                ),
            ))
            .spacing(Space::S)
            .align(HAlign::Leading)
            .grow(Dimension::Width),
        ))
        .spacing(16.0)
        .align(VAlign::Center),
    )
    .id(day::id!(canvas_card_clock))
    .build()
}

/// The clock at ten past ten, with its second hand at `seconds`.
fn draw_clock(draw: &mut Draw, size: Size, seconds: f64) {
    let r = size.width.min(size.height) / 2.0;
    if r < 20.0 {
        return;
    }
    let center = Affine::translate(size.width / 2.0, size.height / 2.0);
    let (ink, accent) = (SystemColor::Text, Tone::Accent.color());
    let face = disc(Point::new(0.0, 0.0), r - 2.0);
    draw.transformed(center, |draw| {
        draw.fill(face.clone(), gallery::RAISED);
        draw.stroke(face, SystemColor::Separator, 1.5);
    });
    // Sixty ticks: one line, drawn sixty times under sixty rotations.
    for tick in 0..60 {
        let turn = Affine::rotate(Angle::turns(f64::from(tick) / 60.0)).then(center);
        let (inner, width, color) = if tick % 5 == 0 {
            (r - 17.0, 2.0, SystemColor::SecondaryText)
        } else {
            (r - 12.0, 1.0, SystemColor::TertiaryText)
        };
        draw.transformed(turn, |draw| {
            draw.stroke(
                Shape::Line(Point::new(r - 8.0, 0.0), Point::new(inner, 0.0)),
                color,
                width,
            );
        });
    }
    // A hand is a line along x, turned to its place; twelve o'clock is a quarter turn back.
    let mut hand = |turns: f64, (tail, length): (f64, f64), width: f64, color: Ink| {
        draw.transformed(
            (Affine::rotate(Angle::turns(turns) - Angle::QUARTER)).then(center),
            |draw| {
                draw.stroke_styled(
                    Shape::Line(Point::new(-tail, 0.0), Point::new(length, 0.0)),
                    color,
                    StrokeStyle::round(width),
                );
            },
        );
    };
    hand(10.0 / 12.0, (8.0, 0.48 * r), 4.5, ink.into());
    hand(10.0 / 60.0, (8.0, 0.7 * r), 3.0, ink.into());
    hand(seconds / 60.0, (14.0, 0.8 * r), 1.5, accent.into());
    draw.transformed(center, |draw| {
        draw.fill(disc(Point::new(0.0, 0.0), 4.0), accent);
    });
}

// ---------------------------------------------------------------------------
// 3. SVG paths
// ---------------------------------------------------------------------------

/// The side of a glyph's tile.
const GLYPH_TILE: f64 = 60.0;

/// "SVG paths": four glyphs whose path data `build_path!` parsed at compile time, relative and
/// smooth commands included.
fn svg_card() -> AnyPiece {
    card(
        crate::res::vectors::star,
        crate::res::str::canvas_svg_title(),
        crate::res::str::canvas_svg_subtitle(),
        row((
            glyph(
                crate::res::str::canvas_svg_heart(),
                day::id!(canvas_svg_heart),
                |draw| {
                    draw.fill(
                        build_path!(
                            "M12 21s-7-4.35-9.5-8.5C.5 8.5 3 4 7 4c2 0 3.5 1 5 3 1.5-2 3-3 5-3 \
                         4 0 6.5 4.5 4.5 8.5C19 16.65 12 21 12 21z"
                        )
                        .build(),
                        Tone::Danger.color(),
                    );
                },
            ),
            glyph(
                crate::res::str::canvas_svg_star(),
                day::id!(canvas_svg_star),
                |draw| {
                    draw.fill(
                    build_path!(
                        "M12 2l3.09 6.26L22 9.27l-5 4.87L18.18 21 12 17.27 5.82 21 7 14.14l-5-4.87 \
                         6.91-1.01z"
                    )
                    .build(),
                    Tone::Warn.color(),
                );
                },
            ),
            glyph(
                crate::res::str::canvas_svg_bolt(),
                day::id!(canvas_svg_bolt),
                |draw| {
                    draw.fill(
                        build_path!("M13 2L3 14h7l-1 8 10-12h-7z").build(),
                        Tone::Info.color(),
                    );
                },
            ),
            glyph(
                crate::res::str::canvas_svg_check(),
                day::id!(canvas_svg_check),
                |draw| {
                    draw.stroke_styled(
                        build_path!("M20 6L9 17l-5-5").build(),
                        Tone::Accent.color(),
                        StrokeStyle::round(2.5),
                    );
                },
            ),
        ))
        .align(VAlign::Top)
        .grow(Dimension::Width),
    )
    .id(day::id!(canvas_card_svg))
    .build()
}

/// A glyph on a raised tile with its name underneath. `paint` draws in the path's own
/// 24-unit box.
fn glyph(name: day::LocalizedText, id: ElementId, paint: fn(&mut Draw)) -> impl Piece {
    let spoken = name.clone();
    column((
        canvas(move |draw, size| {
            let tile = Rect::new(
                (size.width - GLYPH_TILE) / 2.0,
                (size.height - GLYPH_TILE) / 2.0,
                GLYPH_TILE,
                GLYPH_TILE,
            );
            let inner = Rect::new(
                tile.origin.x + 0.5,
                tile.origin.y + 0.5,
                GLYPH_TILE - 1.0,
                GLYPH_TILE - 1.0,
            );
            draw.fill(Shape::RoundedRect(inner, 18.0), gallery::RAISED);
            draw.stroke(Shape::RoundedRect(inner, 18.0), SystemColor::Separator, 1.0);
            // The 24-unit path at 30 points, centered on the tile.
            let scale = 30.0 / 24.0;
            draw.transformed(
                Affine::scale(scale, scale).then(Affine::translate(
                    tile.origin.x + (GLYPH_TILE - 30.0) / 2.0,
                    tile.origin.y + (GLYPH_TILE - 30.0) / 2.0,
                )),
                paint,
            );
        })
        .frame(GLYPH_TILE, GLYPH_TILE)
        .a11y(move |a| a.label(spoken.clone()))
        .id(id),
        label(name).font(Font::Footnote).secondary(),
    ))
    .spacing(Space::S)
    .align(HAlign::Center)
    .grow(Dimension::Width)
}

// ---------------------------------------------------------------------------
// 4. Compositing
// ---------------------------------------------------------------------------

/// "Compositing": an image clipped to a disc, a ticket with its holes erased, a sphere lit by
/// radial gradients, and three discs in each of four blend modes.
///
/// The stage of both canvases is a native view behind them and the canvases are clear, so the
/// holes erased from the ticket show the stage itself.
fn compositing_card() -> AnyPiece {
    let logo: Signal<Option<day::Bitmap>> = Signal::new(None);
    decode_logo(logo, None);
    let name = |text: day::LocalizedText| {
        column((label(text).font(Font::Footnote).secondary(),))
            .align(HAlign::Center)
            .grow(Dimension::Width)
    };
    card(
        crate::res::vectors::nav_images,
        crate::res::str::canvas_compositing_title(),
        crate::res::str::canvas_compositing_subtitle(),
        body((
            canvas(move |draw, size| {
                let logo = logo.get();
                margined(draw, size, |draw, size| {
                    draw_compositing(draw, size, logo.as_ref());
                });
            })
            .height(132.0 + 2.0 * PAD)
            .grow(Dimension::Width)
            .a11y(|a| a.label(crate::res::str::canvas_compositing_title()))
            .id(day::id!(canvas_compositing))
            .background(stage_color)
            .corner_radius(12.0),
            row((
                name(crate::res::str::canvas_compositing_image()),
                name(crate::res::str::canvas_compositing_cutout()),
                name(crate::res::str::canvas_compositing_radial()),
            ))
            .align(VAlign::Top)
            .grow(Dimension::Width),
            gallery::caption(
                crate::res::str::canvas_compositing_blend(),
                crate::res::str::canvas_compositing_blend_code(),
            ),
            canvas(draw_blends)
                .height(BLEND_HEIGHT)
                .grow(Dimension::Width)
                .a11y(|a| a.label(crate::res::str::canvas_compositing_blend()))
                .id(day::id!(canvas_blends))
                .background(stage_color)
                .corner_radius(12.0),
            row((
                name(crate::res::str::canvas_blend_normal()),
                name(crate::res::str::canvas_blend_multiply()),
                name(crate::res::str::canvas_blend_screen()),
                name(crate::res::str::canvas_blend_add()),
            ))
            .align(VAlign::Top)
            .grow(Dimension::Width),
        )),
    )
    .id(day::id!(canvas_card_compositing))
    .build()
}

/// The three motifs, each centered on a third of the width and scaled down together where
/// the stage is too narrow for them side by side.
fn draw_compositing(draw: &mut Draw, size: Size, logo: Option<&day::Bitmap>) {
    let scale = (size.width / 330.0).min(1.0);
    let y = size.height / 2.0;
    // A shadow's blur and offset are in points whatever the scale, so they are not scaled here.
    let shadow = Shadow::new(shadow_color(0.22, 0.5), 14.0).offset(0.0, 5.0);
    let accent = Tone::Accent.color();
    let origin = Point::new(0.0, 0.0);
    let place = |sixths: f64| {
        Affine::scale(scale, scale).then(Affine::translate(size.width * sixths / 6.0, y))
    };

    // An image clipped to a disc. The opaque base is the one shape that casts the shadow; the
    // wash and the picture clipped over it cast none.
    draw.transformed(place(1.0), |draw| {
        let bounds = Rect::new(-40.0, -40.0, 80.0, 80.0);
        draw.shadowed(shadow, |draw| {
            draw.fill(disc(origin, 40.0), stage_color());
        });
        draw.clipped(disc(origin, 40.0), |draw| {
            draw.fill(
                Shape::Rect(bounds),
                LinearGradient::new(
                    UnitPoint::TOP_LEADING,
                    UnitPoint::BOTTOM_TRAILING,
                    vec![(0.0, gallery::RAISED), (1.0, gallery::INSET)],
                ),
            );
            if let Some(logo) = logo {
                draw_fitted(draw, logo, Rect::new(-46.0, -46.0, 92.0, 92.0));
            }
        });
        draw.stroke(disc(origin, 44.0), accent.with_alpha(0.55), 1.5);
    });

    // A ticket with two notches, a hole and a tear line, all erased from it: the canvas is
    // left clear there, shadow included, and the stage behind the canvas shows through.
    draw.transformed(place(3.0), |draw| {
        let ticket = Rect::new(-52.0, -36.0, 104.0, 72.0);
        let deep = Color::rgb(accent.r * 0.62, accent.g * 0.62, accent.b * 0.62);
        draw.shadowed(shadow, |draw| {
            draw.fill(
                Shape::RoundedRect(ticket, 14.0),
                LinearGradient::new(
                    UnitPoint::TOP_LEADING,
                    UnitPoint::BOTTOM_TRAILING,
                    vec![(0.0, accent), (1.0, deep)],
                ),
            );
        });
        draw.blended(BlendMode::Erase, |draw| {
            // An erasing shape's color is ignored; only its alpha counts.
            let cut = Color::BLACK;
            draw.fill(disc(Point::new(20.0, -36.0), 8.0), cut);
            draw.fill(disc(Point::new(20.0, 36.0), 8.0), cut);
            draw.fill(disc(Point::new(-16.0, 0.0), 13.0), cut);
            draw.stroke_styled(
                Shape::Line(Point::new(20.0, -26.0), Point::new(20.0, 26.0)),
                cut,
                StrokeStyle::dashed(2.0, vec![4.0, 4.0]),
            );
        });
    });

    // A sphere: a glow, an off-center highlight, a dashed orbit and a moon.
    draw.transformed(place(5.0), |draw| {
        let warn = Tone::Warn.color();
        draw.fill(
            disc(origin, 52.0),
            RadialGradient::new(
                UnitPoint::CENTER,
                0.5,
                vec![
                    (0.0, warn.with_alpha(0.32)),
                    (22.0 / 52.0, warn.with_alpha(0.32)),
                    (1.0, warn.with_alpha(0.0)),
                ],
            ),
        );
        // The gradient is centered on the highlight and reaches the far limb.
        draw.fill(
            disc(origin, 30.0),
            RadialGradient::new(
                UnitPoint::new(0.5 - 10.0 / 60.0, 0.5 - 12.0 / 60.0),
                0.76,
                vec![
                    (0.0, Color::hex(0x00FF_F3D6)),
                    (0.35, Color::hex(0x00FF_C05C)),
                    (0.75, Color::hex(0x00E8_743B)),
                    (1.0, Color::hex(0x00A9_3F2C)),
                ],
            ),
        );
        draw.stroke_styled(
            disc(origin, 42.0),
            warn.with_alpha(0.6),
            StrokeStyle::dashed(1.25, vec![3.0, 5.0]),
        );
        draw.fill(
            disc(polar(origin, 42.0, (-0.7_f64).to_degrees()), 4.0),
            Color::hex(0x00FF_E2A8),
        );
    });
}

/// The height of the blend swatches' stage.
const BLEND_HEIGHT: f64 = 76.0;

/// The blend modes the swatches show, in the order of the names under them.
const BLEND_MODES: [BlendMode; 4] = [
    BlendMode::Normal,
    BlendMode::Multiply,
    BlendMode::Screen,
    BlendMode::Add,
];

/// Three overlapping discs per blend mode, each group centered on a quarter of the width. The
/// first disc is drawn plainly and the other two in the mode, so each overlap shows the mode
/// at work on the colors under it. A canvas blends with its own drawing only, which is why the
/// result is the same over a light stage and a dark one.
fn draw_blends(draw: &mut Draw, size: Size) {
    let cell = size.width / BLEND_MODES.len() as f64;
    // A group is 52 points wide and 50 tall.
    let scale = (cell / 60.0).min(size.height / 58.0).min(1.0);
    for (index, mode) in BLEND_MODES.into_iter().enumerate() {
        let center = Point::new(cell * (index as f64 + 0.5), size.height / 2.0);
        draw.transformed(
            Affine::scale(scale, scale).then(Affine::translate(center.x, center.y)),
            |draw| {
                draw.fill(disc(Point::new(0.0, -9.0), 16.0), Tone::Danger.color());
                draw.blended(mode, |draw| {
                    draw.fill(disc(Point::new(-10.0, 9.0), 16.0), Tone::Up.color());
                    draw.fill(disc(Point::new(10.0, 9.0), 16.0), Tone::Info.color());
                });
            },
        );
    }
}

// ---------------------------------------------------------------------------
// 5. The sketch pad
// ---------------------------------------------------------------------------

/// The strokes the pad opens with: the letters D, A and Y, centered in `size`.
fn sketch_sample(size: Size) -> Vec<Vec<Point>> {
    const LETTER: (f64, f64) = (50.0, 72.0);
    const GAP: f64 = 24.0;
    let left = (size.width - (3.0 * LETTER.0 + 2.0 * GAP)) / 2.0;
    let top = (size.height - LETTER.1) / 2.0;
    let at = |letter: usize, (x, y): (f64, f64)| {
        Point::new(
            left + letter as f64 * (LETTER.0 + GAP) + x * LETTER.0,
            top + y * LETTER.1,
        )
    };
    // The D: its stem, then its bowl as a half ellipse back to the foot of the stem.
    let mut d = vec![at(0, (0.0, 1.0)), at(0, (0.0, 0.0))];
    d.extend((1..=8).map(|step| {
        let turn = (-90.0 + 180.0 * f64::from(step) / 8.0_f64).to_radians();
        at(0, (turn.cos(), 0.5 + 0.5 * turn.sin()))
    }));
    vec![
        d,
        vec![at(1, (0.0, 1.0)), at(1, (0.5, 0.0)), at(1, (1.0, 1.0))],
        vec![at(1, (0.2, 0.6)), at(1, (0.8, 0.6))],
        vec![at(2, (0.0, 0.0)), at(2, (0.5, 0.5)), at(2, (1.0, 0.0))],
        vec![at(2, (0.5, 0.5)), at(2, (0.5, 1.0))],
    ]
}

/// "Sketch": a pad that turns each drag into a stroke.
fn sketch_card() -> AnyPiece {
    let sample = Signal::new(true);
    let strokes = Signal::new(Vec::<Vec<Point>>::new());
    // How many strokes and points the pad holds, the opening letters included.
    let counts = move || {
        let (mut count, mut points) =
            strokes.with(|all| (all.len(), all.iter().map(Vec::len).sum::<usize>()));
        if sample.get() {
            let letters = sketch_sample(Size::new(0.0, 0.0));
            count += letters.len();
            points += letters.iter().map(Vec::len).sum::<usize>();
        }
        (count, points)
    };
    card(
        crate::res::vectors::nav_gestures,
        crate::res::str::canvas_sketch_title(),
        crate::res::str::canvas_sketch_subtitle(),
        body((
            canvas(move |draw, size| {
                let letters = if sample.get() {
                    sketch_sample(size)
                } else {
                    Vec::new()
                };
                strokes.with(|drawn| draw_sketch(draw, size, &letters, drawn));
            })
            .height(200.0)
            .grow(Dimension::Width)
            .on_drag(move |drag| match drag.phase {
                DragPhase::Began => strokes.update(|all| all.push(vec![drag.location])),
                _ => strokes.update(|all| {
                    if let Some(stroke) = all.last_mut() {
                        stroke.push(drag.location);
                    }
                }),
            })
            .a11y(|a| a.label(crate::res::str::canvas_sketch_title()))
            .id(day::id!(canvas_sketch)),
            // The kit's caption, laid out here so the count itself carries the id.
            row((
                label(crate::res::str::canvas_sketch_strokes())
                    .font(Font::Footnote)
                    .secondary()
                    .grow(Dimension::Width),
                label(move || {
                    let (count, points) = counts();
                    crate::res::str::canvas_sketch_count(crate::res::str::CanvasSketchCount {
                        points: points as i64,
                        strokes: count as i64,
                    })
                    .format()
                })
                .font(Font::Caption)
                .secondary()
                .tabular()
                .single_line()
                .id(day::id!(canvas_sketch_count)),
            ))
            .spacing(Space::S)
            .align(VAlign::Center)
            .grow(Dimension::Width),
        )),
    )
    .trailing(
        button(crate::res::str::canvas_sketch_clear())
            .action(move || {
                sample.set(false);
                strokes.set(Vec::new());
            })
            .enabled(move || sample.get() || strokes.with(|all| !all.is_empty()))
            .bordered()
            .id(day::id!(canvas_sketch_clear)),
    )
    .id(day::id!(canvas_card_sketch))
    .build()
}

/// The pad: a dotted stage, then the opening `letters` and the `drawn` strokes as round lines.
fn draw_sketch(draw: &mut Draw, size: Size, letters: &[Vec<Point>], drawn: &[Vec<Point>]) {
    draw.fill(
        Shape::RoundedRect(Rect::new(0.0, 0.0, size.width, size.height), 12.0),
        stage_color(),
    );
    let mut dots = Vec::new();
    let mut y = 8.0;
    while y < size.height {
        let mut x = 8.0;
        while x < size.width {
            dots.push(Point::new(x, y));
            x += 16.0;
        }
        y += 16.0;
    }
    draw.stamp(
        Shape::Rect(Rect::new(-0.75, -0.75, 1.5, 1.5)),
        dots,
        SystemColor::Separator,
    );
    if letters.is_empty() && drawn.is_empty() {
        text(
            draw,
            &crate::res::str::canvas_sketch_empty().format(),
            Point::new(size.width / 2.0, size.height / 2.0),
            (14.0, SystemColor::TertiaryText),
            (TextAlign::Center, TextVAlign::Middle),
        );
        return;
    }
    let accent = Tone::Accent.color();
    for stroke in letters.iter().chain(drawn) {
        match stroke.as_slice() {
            [] => {}
            // A press that has not moved yet: a dot the width of the line.
            [only] => draw.fill(disc(*only, 1.5), accent),
            points => draw.stroke_styled(open_path(points), accent, StrokeStyle::round(3.0)),
        }
    }
}

// ---------------------------------------------------------------------------
// Reference demos: text, anchors, pointer input, a decoded image
// ---------------------------------------------------------------------------

/// Canvas text in a chosen font (docs/fonts.md): a specimen line, the bold / italic / bold-italic
/// faces, and a centered glyph, each framed by the box `measure_text` reports, so the anchor
/// rule (a `Leading` anchor is the line box's top-leading corner) is visible. The menu lists
/// every family `font_families()` knows (the platform's plus the bundled ones) and
/// starts on the bundled Pacifico, which is exactly the family no platform ships.
fn text_card() -> AnyPiece {
    let mut families: Vec<String> = day::font_families()
        .iter()
        .map(|f| f.family.clone())
        .collect();
    if families.is_empty() {
        // `Cap::FontList` = Unsupported: one entry, the platform's default face.
        families.push(crate::res::str::canvas_font_default().format());
    }
    let default_ix = families.iter().position(|f| f == "Pacifico").unwrap_or(0);
    let chosen = Signal::new(default_ix);
    let names = families.clone();
    let listed = !day::font_families().is_empty();
    const H: f64 = 168.0;
    card(
        crate::res::vectors::nav_text,
        crate::res::str::canvas_text_title(),
        crate::res::str::canvas_text_subtitle(),
        body((
            canvas(move |d, size| {
                let family = names.get(chosen.get()).cloned().filter(|_| listed);
                let font = |weight: Option<FontWeight>, italic: bool| CanvasFont {
                    family: family.clone(),
                    weight,
                    italic,
                };
                let frame = Color::rgba(0.5, 0.5, 0.55, 0.45);
                let baseline_c = Color::rgba(0.9, 0.3, 0.3, 0.55);
                let cap_c = Color::rgba(0.95, 0.65, 0.15, 0.75);
                let ink_c = Color::rgba(0.35, 0.8, 0.55, 0.8);
                // A line in the family under everything `measure_text` reports about it: the
                // line box (grey; the typographic slot, identical for every string in this face
                // at this size), the ink box (green; where the glyphs put marks), the baseline
                // (red) and the cap line (amber). The three boxes only coincide for text that
                // happens to fill its slot; a line of digits leaves the descender space empty,
                // and the gap between grey and green is exactly why a chart's y labels need the
                // cap line to look centered (docs/fonts.md).
                let framed = |d: &mut Draw, text: &str, at: Point, size: f64, font: CanvasFont| {
                    let m = day::measure_text(text, size, &font);
                    d.stroke(
                        Shape::Rect(Rect::new(at.x, at.y, m.width, m.height)),
                        frame,
                        1.0,
                    );
                    d.stroke(
                        Shape::Rect(Rect::new(
                            at.x + m.ink.origin.x,
                            at.y + m.ink.origin.y,
                            m.ink.size.width,
                            m.ink.size.height,
                        )),
                        ink_c,
                        1.0,
                    );
                    d.stroke(
                        Shape::Line(
                            Point::new(at.x, at.y + m.ascent),
                            Point::new(at.x + m.width, at.y + m.ascent),
                        ),
                        baseline_c,
                        1.0,
                    );
                    d.stroke(
                        Shape::Line(
                            Point::new(at.x, at.y + m.ascent - m.cap_height),
                            Point::new(at.x + m.width, at.y + m.ascent - m.cap_height),
                        ),
                        cap_c,
                        1.0,
                    );
                    d.text(
                        text,
                        at,
                        TextStyle {
                            size,
                            color: AZURE.into(),
                            anchor: TextAnchor::LEADING,
                            font,
                        },
                    );
                    m
                };
                let specimen = crate::res::str::canvas_text_specimen().format();
                // As large as the card is wide, up to 28 points: a wide face in a narrow column
                // would otherwise run out of the card.
                let natural = day::measure_text(&specimen, 28.0, &font(None, false)).width;
                let room = (size.width - 16.0).max(1.0);
                let big = if natural > room {
                    (28.0 * room / natural).max(12.0)
                } else {
                    28.0
                };
                let m = framed(d, &specimen, Point::new(8.0, 8.0), big, font(None, false));
                let y = 8.0 + m.height + 12.0;
                let mut x = 8.0;
                for (label, weight, italic) in [
                    (
                        crate::res::str::canvas_text_bold().format(),
                        Some(FontWeight::Bold),
                        false,
                    ),
                    (crate::res::str::canvas_text_italic().format(), None, true),
                    (
                        crate::res::str::canvas_text_bold_italic().format(),
                        Some(FontWeight::Bold),
                        true,
                    ),
                ] {
                    let m = framed(d, &label, Point::new(x, y), 20.0, font(weight, italic));
                    x += m.width + 18.0;
                }
                // A centered glyph pair in a circle: the `Centered` anchor is the line box's
                // center.
                let r = 30.0;
                let c = Point::new(size.width - r - 12.0, size.height - r - 10.0);
                d.stroke(disc(c, r), CORAL, 2.0);
                d.text(
                    "Aa",
                    c,
                    TextStyle {
                        size: 26.0,
                        color: CORAL.into(),
                        anchor: TextAnchor::CENTERED,
                        font: font(None, false),
                    },
                );
                // What the measuring above cost, read last so it counts this recording's own
                // calls (docs/fonts.md "It is cached"). Drawn inside the canvas rather than
                // beside it because that needs no reactivity at all: the numbers change when
                // this closure runs, and this closure is what draws them.
                //
                // Watch it move. Resizing the window re-records with the same strings, so
                // `hits` climbs and `misses` does not; the measurements never reach the toolkit
                // twice. Choosing another font changes the key of every specimen, so `misses`
                // jumps by the number of lines here and then stops again.
                let st = day::text_metrics_cache_stats();
                d.text(
                    // Named arguments: three `i64`s in the wrong order would compile and render
                    // a plausible-looking lie, so the generated accessor takes a struct.
                    &crate::res::str::canvas_metrics_cache(crate::res::str::CanvasMetricsCache {
                        hits: st.hits as i64,
                        misses: st.misses as i64,
                        entries: st.entries as i64,
                    })
                    .format(),
                    Point::new(8.0, size.height - 6.0),
                    TextStyle {
                        size: 11.0,
                        color: SystemColor::SecondaryText.into(),
                        anchor: TextAnchor {
                            h: TextAlign::Leading,
                            v: TextVAlign::Bottom,
                        },
                        font: CanvasFont::default(),
                    },
                );
            })
            .height(H)
            .grow(Dimension::Width)
            .id(day::id!(canvas_text)),
            row((
                label(crate::res::str::canvas_font_label())
                    .font(Font::Footnote)
                    .secondary()
                    .grow(Dimension::Width),
                picker(
                    chosen,
                    families.iter().cloned().enumerate().collect::<Vec<_>>(),
                )
                .menu()
                .id(day::id!(canvas_font)),
            ))
            .spacing(Space::S)
            .align(VAlign::Center)
            .grow(Dimension::Width),
        )),
    )
    .id(day::id!(canvas_card_text))
    .build()
}

/// Every `TextAnchor` placement, on a crosshair (docs/canvas.md "Anchors").
///
/// Twelve cells: the three `TextAlign` values across, the four `TextVAlign` values down. Each
/// cell draws the anchor `at` as a crosshair and then hangs the same word on it, so what an
/// anchor means is the picture rather than a sentence, and a backend that gets one wrong shows it
/// here immediately, which is the reason this is a card and not a doc example. The sample has
/// both an ascender and a descender so `Baseline` and `Bottom` are told apart.
///
/// The placement names are API identifiers, shown verbatim like the Cursors page's `css_name`s;
/// only the card's title and subtitle are localized.
fn anchors_card() -> AnyPiece {
    const H: f64 = 232.0;
    const SAMPLE: &str = "Agy";
    card(
        crate::res::vectors::nav_textfields,
        crate::res::str::canvas_anchors_title(),
        crate::res::str::canvas_anchors_subtitle(),
        canvas(|d, size| {
            let muted = Color::rgba(0.5, 0.5, 0.55, 0.85);
            let rule = Color::rgba(0.9, 0.3, 0.3, 0.5);
            let caption = |d: &mut Draw, text: &str, at: Point, anchor: TextAnchor| {
                d.text(
                    text,
                    at,
                    TextStyle {
                        size: 11.0,
                        color: muted.into(),
                        anchor,
                        font: CanvasFont::default(),
                    },
                );
            };
            let gutter = 62.0;
            let head = 18.0;
            let cw = ((size.width - gutter - 8.0) / 3.0).max(1.0);
            let ch = ((size.height - head - 8.0) / 4.0).max(1.0);
            for (i, h) in [TextAlign::Leading, TextAlign::Center, TextAlign::Trailing]
                .into_iter()
                .enumerate()
            {
                let cx = gutter + cw * (i as f64 + 0.5);
                caption(
                    d,
                    &format!("{h:?}"),
                    Point::new(cx, 2.0),
                    TextAnchor {
                        h: TextAlign::Center,
                        v: TextVAlign::Top,
                    },
                );
                for (j, v) in [
                    TextVAlign::Top,
                    TextVAlign::Middle,
                    TextVAlign::Baseline,
                    TextVAlign::Bottom,
                ]
                .into_iter()
                .enumerate()
                {
                    let cy = head + ch * (j as f64 + 0.5);
                    if i == 0 {
                        // The row's name, itself right-aligned against the grid: this card's
                        // own small use of the anchor it is documenting.
                        caption(
                            d,
                            &format!("{v:?}"),
                            Point::new(gutter - 10.0, cy),
                            TextAnchor {
                                h: TextAlign::Trailing,
                                v: TextVAlign::Middle,
                            },
                        );
                    }
                    // The crosshair marks the anchor position itself.
                    d.stroke(
                        Shape::Line(
                            Point::new(cx - cw * 0.45, cy),
                            Point::new(cx + cw * 0.45, cy),
                        ),
                        rule,
                        1.0,
                    );
                    d.stroke(
                        Shape::Line(Point::new(cx, cy - 13.0), Point::new(cx, cy + 13.0)),
                        rule,
                        1.0,
                    );
                    d.text(
                        SAMPLE,
                        Point::new(cx, cy),
                        TextStyle {
                            size: 19.0,
                            color: AZURE.into(),
                            anchor: TextAnchor { h, v },
                            font: CanvasFont::default(),
                        },
                    );
                }
            }
        })
        .height(H)
        .grow(Dimension::Width)
        .id(day::id!(canvas_anchors)),
    )
    .id(day::id!(canvas_card_anchors))
    .build()
}

/// Tap, drag and hover on one canvas (docs/canvas.md "Interaction").
///
/// Three gestures, three marks, put together because they are not interchangeable. Hover
/// follows a pointer and is the only one a touch-only phone never reports; a tap is the one every
/// device has; a drag is what a press that travels becomes, and on some backends what a press
/// that barely travels becomes too. Anything reachable by hover has to be
/// reachable by a tap, which is why a chart's selection wires all three to the same signal.
///
/// The readout names what each gesture last reported, so a run on any backend says plainly which
/// of the three that backend delivers.
fn interaction_card() -> AnyPiece {
    const H: f64 = 200.0;
    let hover = Signal::new(None::<Point>);
    let tap = Signal::new(None::<Point>);
    let trail = Signal::new(Vec::<Point>::new());
    let dragging = Signal::new(false);
    card(
        crate::res::vectors::nav_cursor,
        crate::res::str::canvas_interaction_title(),
        crate::res::str::canvas_interaction_subtitle(),
        body((
            canvas(move |d, size| {
                let muted = Color::rgba(0.5, 0.5, 0.55, 0.7);
                // A frame, so the canvas's bounds are visible; a hover that reports nothing and
                // a canvas that is not there look the same otherwise.
                d.stroke(
                    Shape::Rect(Rect::new(0.5, 0.5, size.width - 1.0, size.height - 1.0)),
                    muted,
                    1.0,
                );
                // Hover: a crosshair, drawn only while a pointer is inside.
                if let Some(p) = hover.get() {
                    d.stroke(
                        Shape::Line(Point::new(0.0, p.y), Point::new(size.width, p.y)),
                        muted,
                        1.0,
                    );
                    d.stroke(
                        Shape::Line(Point::new(p.x, 0.0), Point::new(p.x, size.height)),
                        muted,
                        1.0,
                    );
                }
                // Drag: the path travelled, as one stamped run of dots plus a line, the batched
                // op (docs/canvas.md "Stamping"), because a long drag is a lot of marks.
                let pts = trail.get();
                if pts.len() > 1 {
                    d.stroke(Shape::Polygon(pts.clone()), TEAL.with_alpha(0.45), 1.5);
                    d.stamp(Shape::Ellipse(Rect::new(-2.0, -2.0, 4.0, 4.0)), pts, TEAL);
                }
                // Tap: a ring where the last one landed.
                if let Some(p) = tap.get() {
                    d.stroke(disc(p, 9.0), CORAL, 2.0);
                    d.fill(disc(p, 2.5), CORAL);
                }
            })
            .height(H)
            .grow(Dimension::Width)
            .on_hover(move |at| hover.set(at))
            .on_tap_at(move |p| tap.set(Some(p)))
            .on_drag(move |drag| {
                match drag.phase {
                    DragPhase::Began => {
                        dragging.set(true);
                        trail.set(vec![drag.location]);
                    }
                    _ => trail.update(|t| t.push(drag.location)),
                }
                if drag.phase == DragPhase::Ended {
                    dragging.set(false);
                }
            })
            .id(day::id!(canvas_interaction)),
            column((
                label(crate::res::str::canvas_interaction_label())
                    .font(Font::Caption)
                    .secondary(),
                label(move || {
                    let fmt = |p: Option<Point>| match p {
                        Some(p) => format!("{:.0}, {:.0}", p.x, p.y),
                        None => crate::res::str::canvas_interaction_none().format(),
                    };
                    // A generated `res::str` takes its arguments alphabetically, not in the
                    // order the message mentions them; named locals so the call reads as a
                    // check.
                    let (hover_s, tap_s, trail_n) = (
                        fmt(hover.get()),
                        fmt(tap.get()),
                        trail.with(|t| t.len() as i64),
                    );
                    crate::res::str::canvas_interaction_readout(
                        crate::res::str::CanvasInteractionReadout {
                            hover: hover_s,
                            tap: tap_s,
                            trail: trail_n,
                        },
                    )
                    .format()
                })
                .font(Font::Callout)
                .id(day::id!(canvas_interaction_readout)),
            ))
            .spacing(2.0)
            .align(HAlign::Leading)
            .grow(Dimension::Width),
        )),
    )
    .id(day::id!(canvas_card_interaction))
    .build()
}

/// A raster image decoded from BYTES and drawn on a canvas (docs/images.md).
///
/// The asset is read as a blob, handed to `day::decode_image`, and the returned handle is drawn
/// with `Draw::image`, the path a downloaded, pasted, or user-picked image takes, with no staged
/// image resource behind it. That these particular bytes happen to be bundled is incidental:
/// nothing on this path knows where they came from.
///
/// The readout names what the platform's decoder said the bytes were, so a backend that reads
/// them differently says so here rather than in a doc, which is the reason this runs on every
/// toolkit.
fn image_card() -> AnyPiece {
    let shown: Signal<Option<day::Bitmap>> = Signal::new(None);
    let readout = Signal::new(crate::res::str::canvas_image_loading().format());
    decode_logo(shown, Some(readout));
    card(
        crate::res::vectors::nav_resources,
        crate::res::str::canvas_image_title(),
        crate::res::str::canvas_image_subtitle(),
        body((
            canvas(move |d, size| {
                let Some(bitmap) = shown.get() else { return };
                // Fit inside the canvas and center, preserving the aspect the decoder reported:
                // a stretched logo would misrepresent the very thing this card is showing.
                draw_fitted(d, &bitmap, Rect::new(0.0, 0.0, size.width, size.height));
            })
            .height(180.0)
            .grow(Dimension::Width)
            .a11y(|a| a.label(crate::res::str::canvas_image_title()))
            .id(day::id!(canvas_image)),
            label(move || readout.get()).id(day::id!(canvas_image_readout)),
        )),
    )
    .id(day::id!(canvas_card_image))
    .build()
}
