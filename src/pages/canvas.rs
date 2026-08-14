use day::prelude::*;
use day_piece_rating::{Card, badge, rating};

use crate::palette::{AMBER, AZURE, CORAL, INK, RUST, SKY, SLATE, TEAL, VIOLET};
use crate::widgets::{gauge, page_wide};

/// Drawing & composition (docs/shapes.md, docs/canvas.md, DESIGN §8/§11): the unified `shape`
/// piece in every kind, live canvas transforms and gestures, the slider-driven gauge, and the
/// composition-tier widgets (rating, card, badge, button styles, ambient environment) — each
/// group in its own themed section.
pub(crate) fn canvas_page() -> AnyPiece {
    page_wide(
        crate::res::str::nav_canvas(),
        "canvas-title",
        Some(crate::res::str::canvas_caption()),
        form((
            shapes_section(),
            paths_section(),
            gradients_section(),
            gauge_section(),
            compose_section(),
        ))
        .any(),
    )
}

/// Paths, stroke styles and clipping (docs/canvas.md): the primitives beyond rectangles and
/// polygons, in three canvases so the whole vocabulary fits one screen.
///
/// Each canvas takes an equal share of the row's width (`grow_w`) and derives its height from
/// that width (`aspect_ratio`), so the group fills the page and grows with the window. The
/// drawings themselves are written in a fixed [`DESIGN_W`] x [`DESIGN_H`] space and scaled once
/// by [`in_design_box`], which is what keeps every shape's proportions instead of stretching
/// them.
fn paths_section() -> impl Piece {
    section((row((
        // Arbitrary paths: the same two contours under both fill rules (even-odd cuts the
        // hole, non-zero does not), and a Catmull-Rom spline through scattered points.
        canvas(|d, size| {
            in_design_box(d, size, |d| {
                for (i, rule) in [FillRule::EvenOdd, FillRule::NonZero].iter().enumerate() {
                    let cx = if i == 0 { 40.0 } else { 110.0 };
                    d.fill(
                        PathBuilder::new()
                            .rule(*rule)
                            .circle(Point::new(cx, 32.0), 20.0)
                            .circle(Point::new(cx, 32.0), 11.0)
                            .build(),
                        if i == 0 { TEAL } else { VIOLET },
                    );
                }
                let pts: Vec<Point> = [
                    (9.0, 90.0),
                    (42.0, 66.0),
                    (75.0, 86.0),
                    (108.0, 62.0),
                    (141.0, 80.0),
                ]
                .iter()
                .map(|(x, y)| Point::new(*x, *y))
                .collect();
                d.stroke_styled(
                    PathBuilder::new().smooth_polyline(&pts, 1.0).build(),
                    AZURE,
                    StrokeStyle::round(3.0),
                );
            });
        })
        .id("canvas-paths")
        .aspect_ratio(DESIGN_RATIO)
        .grow_w(),
        // Stroke styles and clipping: the three caps, the three joins, a dashed rule, a
        // gradient-painted stroke, and a fan of lines confined to a path.
        canvas(|d, size| {
            in_design_box(d, size, |d| {
                for (i, cap) in [LineCap::Butt, LineCap::Round, LineCap::Square]
                    .iter()
                    .enumerate()
                {
                    let y = 12.0 + 10.0 * i as f64;
                    d.stroke_styled(
                        Shape::Line(Point::new(14.0, y), Point::new(64.0, y)),
                        CORAL,
                        StrokeStyle {
                            width: 7.0,
                            cap: *cap,
                            ..Default::default()
                        },
                    );
                }
                for (i, join) in [LineJoin::Miter, LineJoin::Round, LineJoin::Bevel]
                    .iter()
                    .enumerate()
                {
                    let x = 92.0 + 21.0 * i as f64;
                    d.stroke_styled(
                        PathBuilder::new()
                            .move_to(Point::new(x - 8.0, 30.0))
                            .line_to(Point::new(x, 10.0))
                            .line_to(Point::new(x + 8.0, 30.0))
                            .build(),
                        AMBER,
                        StrokeStyle {
                            width: 6.0,
                            join: *join,
                            ..Default::default()
                        },
                    );
                }
                d.stroke_styled(
                    Shape::Line(Point::new(12.0, 46.0), Point::new(138.0, 46.0)),
                    SLATE,
                    StrokeStyle::dashed(2.0, vec![8.0, 5.0]),
                );
                d.stroke_styled(
                    Shape::Line(Point::new(12.0, 56.0), Point::new(138.0, 56.0)),
                    LinearGradient::horizontal(RUST, SKY),
                    StrokeStyle::round(6.0),
                );
                // Without the clip these lines would run the full height of the box.
                let clip = PathBuilder::new()
                    .move_to(Point::new(15.0, 98.0))
                    .line_to(Point::new(15.0, 74.0))
                    .quad_to(Point::new(75.0, 58.0), Point::new(135.0, 74.0))
                    .line_to(Point::new(135.0, 98.0))
                    .close()
                    .build();
                d.clipped(clip, |d| {
                    for i in 0..20 {
                        let x = 3.0 + 8.0 * i as f64;
                        d.stroke(
                            Shape::Line(Point::new(x, 60.0), Point::new(x - 12.0, 100.0)),
                            if i % 2 == 0 { INK } else { TEAL },
                            4.0,
                        );
                    }
                });
            });
        })
        .id("canvas-strokes")
        .aspect_ratio(DESIGN_RATIO)
        .grow_w(),
        // SVG path data, parsed at COMPILE time into PathBuilder chains (build_path!). Each
        // glyph is authored in a 24x24 box; the 3x2 grid places them inside the design box.
        canvas(|d, size| {
            in_design_box(d, size, |d| {
                let (cw, ch) = (DESIGN_W / 3.0, DESIGN_H / 2.0);
                let s = cw.min(ch) * 0.82 / 24.0;
                let cell = |col: usize, rowi: usize| {
                    Affine::scale(s, s).then(Affine::translate(
                        cw * col as f64 + (cw - 24.0 * s) / 2.0,
                        ch * rowi as f64 + (ch - 24.0 * s) / 2.0,
                    ))
                };
                // A heart, from relative cubics.
                d.transformed(cell(0, 0), |d| {
                    d.fill(
                        build_path!(
                            "M12,21 C5.5,15.5 2,12 2,8.5 C2,5.4 4.4,3 7.5,3 \
                                 C9.2,3 10.9,3.8 12,5.1 C13.1,3.8 14.8,3 16.5,3 \
                                 C19.6,3 22,5.4 22,8.5 C22,12 18.5,15.5 12,21 Z"
                        )
                        .build(),
                        CORAL,
                    );
                });
                // A PENTAGRAM — one self-intersecting contour, so the fill rules disagree:
                // even-odd hollows the middle pentagon, non-zero would fill it solid.
                d.transformed(cell(1, 0), |d| {
                    d.fill(
                        build_path!("M12,2 L19.1,21.5 2.4,9.2 21.6,9.2 4.9,21.5 Z")
                            .rule(FillRule::EvenOdd)
                            .build(),
                        AMBER,
                    );
                });
                // Arcs (A) become cubics in the macro: a crescent from two of them.
                d.transformed(cell(2, 0), |d| {
                    d.fill(
                        build_path!("M16,3 A 10,10 0 1 0 16,21 A 8,8 0 1 1 16,3 Z").build(),
                        VIOLET,
                    );
                });
                // Smooth cubics (S) reflect the previous control point: one continuous wave.
                d.transformed(cell(0, 1), |d| {
                    d.stroke_styled(
                        build_path!("M1,12 C4,4 8,4 11,12 S18,20 23,12").build(),
                        AZURE,
                        StrokeStyle::round(2.4),
                    );
                });
                // Quadratics (Q/T): a stylised cloud.
                d.transformed(cell(1, 1), |d| {
                    d.fill(
                        build_path!(
                            "M6,18 Q2,18 2,14 T6,10 Q6,4 12,4 Q18,4 18,10 Q22,10 22,14 T18,18 Z"
                        )
                        .build(),
                        SKY,
                    );
                });
                // Two arc circles, filled even-odd: the inner one reads as a hole.
                d.transformed(cell(2, 1), |d| {
                    d.fill(
                        build_path!("M12,2 A10,10 0 1 1 11.99,2 Z M12,7 A5,5 0 1 0 12.01,7 Z")
                            .rule(FillRule::EvenOdd)
                            .build(),
                        TEAL,
                    );
                });
            });
        })
        .id("canvas-svg-paths")
        .aspect_ratio(DESIGN_RATIO)
        .grow_w(),
    ))
    .spacing(16.0),))
    .title(crate::res::str::paths_title())
}

/// The coordinate space the drawings above are written in, and the ratio the canvases hold.
///
/// 3:2 because the SVG grid IS three cells by two rows, and because a square canvas taking a
/// third of a wide window would make this one section taller than the screen.
const DESIGN_W: f64 = 150.0;
const DESIGN_H: f64 = 100.0;
const DESIGN_RATIO: f64 = DESIGN_W / DESIGN_H;

/// Run `f` in the [`DESIGN_W`] x [`DESIGN_H`] box, scaled UNIFORMLY to fit `size` and centred.
///
/// One scale factor for both axes is the whole point: the canvas is whatever size the row gives
/// it, and every shape inside keeps the proportions it was drawn with rather than stretching.
/// The canvas is asked to hold the same ratio, so in practice the fit is exact and the centring
/// terms are zero — they matter only if a backend hands the canvas a differently-shaped box.
fn in_design_box(d: &mut Draw, size: Size, f: impl FnOnce(&mut Draw)) {
    let s = (size.width / DESIGN_W)
        .min(size.height / DESIGN_H)
        .max(0.001);
    d.transformed(
        Affine::scale(s, s).then(Affine::translate(
            (size.width - DESIGN_W * s) / 2.0,
            (size.height - DESIGN_H * s) / 2.0,
        )),
        f,
    );
}

/// Rotate a gradient unit point about the box centre (0.5, 0.5) — the shared angle applied to
/// every swatch's base geometry.
fn spin(p: UnitPoint, deg: f64) -> UnitPoint {
    let (s, c) = deg.to_radians().sin_cos();
    let (dx, dy) = (p.x - 0.5, p.y - 0.5);
    UnitPoint::new(0.5 + dx * c - dy * s, 0.5 + dx * s + dy * c)
}

/// Linear + radial gradients (docs/shapes.md §7): `.fill_linear`/`.fill_radial` on shape pieces.
/// ONE angle slider drives the whole group — each swatch's closure re-records with its base
/// geometry rotated by the shared signal (linear lines spin about the unit-box centre; radial
/// centres orbit it).
fn gradients_section() -> impl Piece {
    let angle = Signal::new(0.0f64);
    // Base geometry + stops per swatch, spun by the shared angle at record time.
    let linear = move |start: UnitPoint, end: UnitPoint, stops: Vec<(f64, Color)>| {
        move || {
            LinearGradient::new(
                spin(start, angle.get()),
                spin(end, angle.get()),
                stops.clone(),
            )
        }
    };
    let radial = move |center: UnitPoint, radius: f64, stops: Vec<(f64, Color)>| {
        move || RadialGradient::new(spin(center, angle.get()), radius, stops.clone())
    };
    // A 3×2 grid of width-flexible swatches (like the Kinds grid) — every swatch responds to
    // the shared angle: linear lines spin about the unit-box centre, radial centres orbit it.
    const H: f64 = 72.0;
    section((
        grid((
            grid_row((
                // Dawn: the icon's amber down into its rust sun-base.
                rectangle()
                    .fill_linear(linear(
                        UnitPoint::TOP,
                        UnitPoint::BOTTOM,
                        vec![(0.0, AMBER), (1.0, RUST)],
                    ))
                    .height(H)
                    .id("gradient-vertical")
                    .grow_w(),
                rounded_rectangle(12.0)
                    .fill_linear(linear(
                        UnitPoint::LEADING,
                        UnitPoint::TRAILING,
                        vec![(0.0, VIOLET), (1.0, SKY)],
                    ))
                    .height(H)
                    .id("gradient-horizontal")
                    .grow_w(),
                // The website's sunrise gradient (--grad-day): amber through coral into blue.
                circle()
                    .fill_linear(linear(
                        UnitPoint::TOP_LEADING,
                        UnitPoint::BOTTOM_TRAILING,
                        vec![(0.0, AMBER), (0.5, CORAL), (1.0, AZURE)],
                    ))
                    .height(H)
                    .id("gradient-stops")
                    .grow_w(),
            )),
            grid_row((
                rounded_rectangle(12.0)
                    .fill_linear(linear(
                        UnitPoint::LEADING,
                        UnitPoint::TRAILING,
                        vec![(0.0, TEAL), (1.0, INK)],
                    ))
                    .height(H)
                    .id("gradient-angle")
                    .grow_w(),
                // Radial: off-center highlight, and a multi-stop "sunset" in a non-square
                // frame (the unit-space radius stretches elliptically to the bounds).
                circle()
                    .fill_radial(radial(
                        UnitPoint::new(0.35, 0.35),
                        0.75,
                        vec![(0.0, Color::hex(0xD9E6FF)), (1.0, SKY)],
                    ))
                    .height(H)
                    .id("gradient-radial-offset")
                    .grow_w(),
                rounded_rectangle(12.0)
                    .fill_radial(radial(
                        UnitPoint::BOTTOM,
                        1.0,
                        vec![(0.0, AMBER), (0.5, RUST), (1.0, INK)],
                    ))
                    .height(H)
                    .id("gradient-radial-stops")
                    .grow_w(),
            )),
        ))
        .spacing(12.0),
        labeled(
            crate::res::str::gradient_angle(),
            slider(angle).range(0.0..=360.0).id("gradient-angle-slider"),
        ),
    ))
    .title(crate::res::str::gradients_title())
}

/// The nine shape kinds in a 3×3 grid whose cells split the section width evenly (`grow_w`
/// marks every column flexible — docs/grid.md) and whose drawing scales with the cell. ONE
/// angle slider rotates every shape live. Each cell draws through [`shape_group_fn`], sizing
/// its shape to the largest box that fits the laid-out cell at EVERY angle — so the slider
/// is a pure transform (the shape spins without resizing) and rotation never clips on
/// backends that clip a canvas to its bounds (Qt, Android, the web).
fn shapes_section() -> impl Piece {
    let angle = Signal::new(0.0f64);
    const H: f64 = 96.0;
    // A Kinds cell: `make()`'s shape at `aspect` (height:width), centred, at a CONSTANT size
    // independent of the shared rotation: a w × (w·aspect) box sweeps a circumcircle of
    // diameter w·√(1+aspect²), so capping that at the cell's short side fits every angle.
    // Reads `angle` inside the recorder, so the slider re-records live.
    let cell = move |aspect: f64, make: fn() -> ShapePiece| {
        shape_group_fn(move |size| {
            let a = angle.get();
            let avail = ((size.width - 8.0).min(size.height - 8.0)).max(1.0);
            let w = avail / (1.0 + aspect * aspect).sqrt();
            let (uw, uh) = (w / size.width, w * aspect / size.height);
            vec![
                make()
                    .rotate(a)
                    .at((1.0 - uw) / 2.0, (1.0 - uh) / 2.0, uw, uh),
            ]
        })
        .height(H)
    };
    section((
        grid((
            grid_row((
                cell(0.5, || rectangle().fill(SKY))
                    .id("shape-rect")
                    .grow_w(),
                cell(0.5, || rounded_rectangle(12.0).fill(VIOLET))
                    .id("shape-rrect")
                    .grow_w(),
                cell(1.0, || circle().fill(TEAL))
                    .id("shape-circle")
                    .grow_w(),
            )),
            grid_row((
                cell(0.45, || capsule().fill(CORAL))
                    .id("shape-capsule")
                    .grow_w(),
                cell(0.55, || ellipse().stroke(AZURE, 4.0))
                    .id("shape-ellipse")
                    .grow_w(),
                cell(1.0, || arc(135.0, 270.0).stroke(TEAL, 6.0))
                    .id("shape-arc")
                    .grow_w(),
            )),
            grid_row((
                // Line + polygon resolve unit points against their box (docs/shapes.md §3.1).
                cell(1.0, || line((0.1, 0.85), (0.9, 0.15)).stroke(SLATE, 4.0))
                    .id("shape-line")
                    .grow_w(),
                cell(1.0, || {
                    polygon([
                        (0.5, 0.03),
                        (0.61, 0.38),
                        (0.98, 0.38),
                        (0.68, 0.6),
                        (0.79, 0.95),
                        (0.5, 0.73),
                        (0.21, 0.95),
                        (0.32, 0.6),
                        (0.02, 0.38),
                        (0.39, 0.38),
                    ])
                    .fill(AMBER)
                })
                .id("shape-polygon")
                .grow_w(),
                // A multi-shape group in ONE canvas leaf (docs/shapes.md §3.6): a target —
                // ring, disc, four tick lines — spun by rotating just the LINES (each line's
                // spec spans the group's box, so `.rotate` orbits its endpoints about the
                // centre); the centred ring and disc are rotation-invariant, and the figure
                // stays inside its circumcircle, so it needs no shrink-to-fit.
                shape_group_fn(move |size| {
                    let a = angle.get();
                    let side = (size.width.min(size.height) - 8.0).max(1.0);
                    let (uw, uh) = (side / size.width, side / size.height);
                    let (ux, uy) = ((1.0 - uw) / 2.0, (1.0 - uh) / 2.0);
                    // The disc's own unit rect, composed into the centred square box.
                    let (dx, dy) = (ux + 0.38 * uw, uy + 0.38 * uh);
                    vec![
                        circle().stroke(RUST, 4.0).inset(4.0).at(ux, uy, uw, uh),
                        circle().fill(RUST).at(dx, dy, 0.24 * uw, 0.24 * uh),
                        line((0.5, 0.0), (0.5, 0.14))
                            .stroke(RUST, 3.0)
                            .rotate(a)
                            .at(ux, uy, uw, uh),
                        line((0.5, 0.86), (0.5, 1.0))
                            .stroke(RUST, 3.0)
                            .rotate(a)
                            .at(ux, uy, uw, uh),
                        line((0.0, 0.5), (0.14, 0.5))
                            .stroke(RUST, 3.0)
                            .rotate(a)
                            .at(ux, uy, uw, uh),
                        line((0.86, 0.5), (1.0, 0.5))
                            .stroke(RUST, 3.0)
                            .rotate(a)
                            .at(ux, uy, uw, uh),
                    ]
                })
                .height(H)
                .id("shape-group")
                .grow_w(),
            )),
        ))
        .spacing(12.0),
        labeled(
            crate::res::str::shapes_angle(),
            slider(angle).range(0.0..=360.0).id("shapes-angle-slider"),
        ),
    ))
    .title(crate::res::str::shapes_kinds())
}

/// Three custom-drawn readings of ONE value signal — the arc dial, a VU-style segment
/// meter, and a sunrise (the sun climbs from the left horizon to the zenith and sets to the
/// right as the value runs 0→100, under a sky whose light follows it). Laid out like the
/// grids above: three width-flexible cells splitting the row evenly, each canvas
/// re-recording at its laid-out size, with the shared slider underneath.
fn gauge_section() -> impl Piece {
    let level = Signal::new(40.0f64);
    const H: f64 = 120.0;
    section((
        grid((grid_row((
            gauge(level).height(H).grow_w(),
            led_meter(level).height(H).grow_w(),
            sunrise_meter(level).height(H).grow_w(),
        )),))
        .spacing(12.0),
        labeled(
            crate::res::str::gauge_value_label(),
            slider(level).range(0.0..=100.0).id("gauge-slider"),
        ),
    ))
    .title(crate::res::str::canvas_gauge())
}

/// A VU-style segment meter: twelve bottom-anchored bars in a rising ramp, lit up to the
/// level — teal through amber into coral at the top of the scale, the unlit tail dimmed.
fn led_meter(level: Signal<f64>) -> AnyPiece {
    canvas(move |d, size| {
        const N: usize = 12;
        let gap = (size.width * 0.012).clamp(3.0, 8.0);
        let w = (size.width - 16.0 - gap * (N as f64 - 1.0)) / N as f64;
        let max_h = size.height - 16.0;
        if w <= 1.0 || max_h <= 8.0 {
            return;
        }
        let frac = (level.get() / 100.0).clamp(0.0, 1.0);
        let lit = (frac * N as f64).round() as usize;
        let track = Color::rgba(0.5, 0.5, 0.55, 0.25);
        for i in 0..N {
            let t = (i as f64 + 1.0) / N as f64;
            let h = max_h * (0.35 + 0.65 * t);
            let color = if i < lit {
                if t > 0.85 {
                    CORAL
                } else if t > 0.6 {
                    AMBER
                } else {
                    TEAL
                }
            } else {
                track
            };
            d.fill(
                Shape::RoundedRect(
                    Rect::new(8.0 + i as f64 * (w + gap), 8.0 + (max_h - h), w, h),
                    (w / 2.0).min(4.0),
                ),
                color,
            );
        }
    })
    .a11y(move |a| {
        a.role(Role::Meter)
            .label(crate::res::str::gauge_value_label().format())
            .value(format!("{:.0}", level.get_untracked()))
    })
    .id("gauge-led")
}

/// Blend two colors component-wise, `t` = 0 → `a`, 1 → `b`.
fn mix(a: Color, b: Color, t: f64) -> Color {
    let l = |x: f64, y: f64| x + (y - x) * t;
    Color::rgba(l(a.r, b.r), l(a.g, b.g), l(a.b, b.b), l(a.a, b.a))
}

/// A sunrise meter: the sun travels a half-circle above the horizon — rising from the left
/// at 0, zenith at 50, setting to the right at 100 — with rays, a faint path track, a
/// ground line, and a sky gradient whose light follows the sun: night indigo over an amber
/// glow at dawn, blue over haze at noon, dusk purple over coral at sunset. All geometry
/// derives from the laid-out size.
fn sunrise_meter(level: Signal<f64>) -> AnyPiece {
    canvas(move |d, size| {
        let frac = (level.get() / 100.0).clamp(0.0, 1.0);
        let horizon_y = size.height - 24.0;
        let cx = size.width / 2.0;
        let r = (size.width / 2.0 - 26.0).min(horizon_y - 26.0);
        if r <= 10.0 {
            return;
        }
        // Sky first, behind everything: top and horizon colors each lerp dawn → noon →
        // sunset with the slider.
        let (top, glow) = if frac < 0.5 {
            let t = frac * 2.0;
            (
                mix(Color::hex(0x232A5C), Color::hex(0x6FBFF2), t),
                mix(Color::hex(0xFFAC5F), Color::hex(0xEAF6FF), t),
            )
        } else {
            let t = frac * 2.0 - 1.0;
            (
                mix(Color::hex(0x6FBFF2), Color::hex(0x46265E), t),
                mix(Color::hex(0xEAF6FF), Color::hex(0xFF6B52), t),
            )
        };
        d.fill(
            Shape::Rect(Rect::new(8.0, 8.0, size.width - 16.0, horizon_y - 8.0)),
            LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                vec![(0.0, top), (1.0, glow)],
            ),
        );
        let track = Color::rgba(0.5, 0.5, 0.55, 0.3);
        // The sun's path, then the ground.
        d.stroke(
            Shape::Arc {
                rect: Rect::new(cx - r, horizon_y - r, r * 2.0, r * 2.0),
                start_deg: 180.0,
                sweep_deg: 180.0,
            },
            track,
            2.0,
        );
        d.fill(
            Shape::Rect(Rect::new(
                8.0,
                horizon_y,
                size.width - 16.0,
                size.height - horizon_y - 8.0,
            )),
            Color::rgba(0.5, 0.5, 0.55, 0.15),
        );
        d.stroke(
            Shape::Line(
                Point::new(8.0, horizon_y),
                Point::new(size.width - 8.0, horizon_y),
            ),
            SLATE,
            2.5,
        );
        // Sun position along the half-circle (y-down coords: subtract the sine).
        let ang = std::f64::consts::PI * (1.0 - frac);
        let (sx, sy) = (cx + r * ang.cos(), horizon_y - r * ang.sin());
        let sun_r = (r * 0.16).clamp(6.0, 15.0);
        for i in 0..8 {
            let ra = f64::from(i) * std::f64::consts::FRAC_PI_4;
            let (rc, rs) = (ra.cos(), ra.sin());
            d.stroke(
                Shape::Line(
                    Point::new(sx + rc * (sun_r + 4.0), sy + rs * (sun_r + 4.0)),
                    Point::new(sx + rc * (sun_r + 9.0), sy + rs * (sun_r + 9.0)),
                ),
                RUST,
                2.5,
            );
        }
        // The sun itself warms from deep amber at the horizons to a pale noon glare.
        d.fill(
            Shape::Ellipse(Rect::new(sx - sun_r, sy - sun_r, sun_r * 2.0, sun_r * 2.0)),
            mix(
                Color::hex(0xFF9E3B),
                Color::hex(0xFFEDAD),
                (std::f64::consts::PI * frac).sin(),
            ),
        );
    })
    .a11y(move |a| {
        a.role(Role::Meter)
            .label(crate::res::str::gauge_value_label().format())
            .value(format!("{:.0}", level.get_untracked()))
    })
    .id("gauge-sunrise")
}

fn compose_section() -> impl Piece {
    // A shared rating signal, driven by tapping stars. Its count is mirrored into a text field:
    // `bind` pushes each newly-tapped value into `rating_text`, so tapping a star updates the field.
    let stars = Signal::new(3usize);
    let rating_text = Signal::new(stars.get().to_string());
    bind(
        move || stars.get(),
        move |n: &usize| rating_text.set(n.to_string()),
    );
    // A custom ambient value flowed via `with_environment` and read back by a descendant.
    #[derive(Clone, Copy)]
    struct Accent(Color);
    let accent = TEAL;

    section((
        label(crate::res::str::compose_caption()).font(Font::Footnote),
        // 1) Interactive star rating (canvas-polygon compose piece): tap a star, and the text
        //    field beside it updates with the count (the `bind` above drives it).
        labeled(
            crate::res::str::compose_rating_label(),
            rating(stars).id("compose-rating"),
        ),
        labeled(
            crate::res::str::compose_rating_count(),
            text_field(rating_text)
                .placeholder(crate::res::str::compose_rating_placeholder())
                .id("compose-rating-value"),
        ),
        // 2) Card modifier — a reusable surface wrapping arbitrary content — plus the badge
        //    overlay (a numbered pill on an icon's top-trailing corner).
        row((
            column((
                label(crate::res::str::compose_card_title()).font(Font::Headline),
                label(crate::res::str::compose_card_body()),
            ))
            .spacing(4.0)
            .align(HAlign::Leading)
            .modifier(Card),
            badge(3, rounded_rectangle(10.0).fill(SLATE).frame(48.0, 48.0)),
        ))
        .spacing(20.0),
        // 3) ButtonStyle — a FilledButtonStyle button next to a plain one for contrast.
        row((
            button(crate::res::str::compose_plain_btn()).id("compose-plain-btn"),
            button(crate::res::str::compose_styled_btn())
                .tint(SKY)
                .id("compose-styled-btn"),
        ))
        .spacing(12.0),
        // 4) Ambient environment flow — a descendant tints itself from the provided Accent.
        with_environment(Accent(accent), || {
            let tint = environment::<Accent>().map(|a| a.0).unwrap_or(Color::BLACK);
            label(crate::res::str::compose_env_value())
                .font(Font::Headline)
                .color(tint)
                .id("compose-env-value")
        }),
    ))
    .title(crate::res::str::nav_compose())
}
