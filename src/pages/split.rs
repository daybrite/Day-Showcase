use day::prelude::*;
use day_piece_stepper::stepper;

use crate::widgets::page_wide;

/// The split (day/docs/split.md): two panes and a divider the user drags, side by side or
/// stacked, the first pane's share bound to a signal. The first pane is a form; the second
/// draws what the form says, from the same signals, so a change on either side of the divider
/// shows on the other. `Cap::Split` says whether this host's own splitter draws the divider
/// (an `NSSplitView`, a `GtkPaned`, a `QSplitter`) or Day does.
pub(crate) fn split_page() -> AnyPiece {
    let st = Scene::new();
    page_wide(
        crate::res::str::nav_split(),
        "split-title",
        column((support_section(), arrangement_section(st), demo(st))).spacing(16.0),
    )
    .any()
}

/// Everything both panes read. The form writes it; the canvas draws it.
#[derive(Clone, Copy)]
struct Scene {
    /// The response under the request (`true`) or beside it: the split's axis.
    stacked: Signal<bool>,
    /// The form pane's share of the split, two-way with the divider.
    share: Signal<f64>,
    caption: Signal<String>,
    count: Signal<f64>,
    radius: Signal<f64>,
    hue: Signal<f64>,
    filled: Signal<bool>,
    /// Index into the shape picker: circle, square, diamond.
    shape: Signal<usize>,
}

impl Scene {
    fn new() -> Scene {
        Scene {
            stacked: Signal::new(false),
            share: Signal::new(0.5),
            caption: Signal::new(String::new()),
            count: Signal::new(5.0),
            radius: Signal::new(28.0),
            hue: Signal::new(210.0),
            filled: Signal::new(false),
            shape: Signal::new(0),
        }
    }
}

/// What this toolkit answers for `Cap::Split`, as a banner and as a row a script can read.
fn support_section() -> impl Piece {
    let support = capability(Cap::Split);
    let text = match support {
        Support::Native => crate::res::str::split_native(),
        _ => crate::res::str::split_composed(),
    };
    section((
        label(crate::res::str::split_hint()).font(Font::Footnote),
        labeled(
            crate::res::str::split_support_label(),
            label(text).id("split-support"),
        ),
    ))
    .title(crate::res::str::split_support_title())
}

/// The split's own two properties, driven from outside it: the axis, and the share the
/// divider also writes, so the slider follows a drag and a drag follows the slider.
fn arrangement_section(st: Scene) -> impl Piece {
    section((
        labeled(
            crate::res::str::split_stacked(),
            toggle(st.stacked).id("split-stacked"),
        ),
        labeled(
            crate::res::str::split_share(),
            row((
                slider(st.share).range(0.2..=0.8).id("split-share"),
                label(move || format!("{:.0}%", st.share.get() * 100.0))
                    .id("split-share-value")
                    .font(Font::Callout),
            ))
            .spacing(12.0),
        ),
    ))
    .title(crate::res::str::split_arrangement_title())
}

/// The split itself: the form first, the drawing second, at a height that leaves both panes
/// usable whichever way they are arranged.
fn demo(st: Scene) -> impl Piece {
    split(form_pane(st), canvas_pane(st))
        .axis(move || {
            if st.stacked.get() {
                SplitAxis::Vertical
            } else {
                SplitAxis::Horizontal
            }
        })
        .fraction(st.share)
        .min_pane(160.0)
        .id("split-demo")
        .height(460.0)
}

/// The stateful side: every control writes one of the scene's signals. It scrolls, because
/// stacked under a dragged divider the pane can be shorter than the form.
fn form_pane(st: Scene) -> impl Piece {
    let shapes = vec![
        crate::res::str::split_shape_circle().format(),
        crate::res::str::split_shape_square().format(),
        crate::res::str::split_shape_diamond().format(),
    ];
    scroll(form((section((
        labeled(
            crate::res::str::split_caption(),
            text_field(st.caption)
                .placeholder(crate::res::str::split_caption_placeholder())
                .id("split-caption"),
        ),
        labeled(
            crate::res::str::split_count(),
            stepper(st.count)
                .range(1.0..=12.0)
                .step(1.0)
                .decimals(0)
                .id("split-count"),
        ),
        labeled(
            crate::res::str::split_radius(),
            slider(st.radius).range(8.0..=80.0).id("split-radius"),
        ),
        labeled(
            crate::res::str::split_hue(),
            slider(st.hue).range(0.0..=360.0).id("split-hue"),
        ),
        labeled(
            crate::res::str::split_filled(),
            toggle(st.filled).id("split-filled"),
        ),
        labeled(
            crate::res::str::split_shape(),
            picker(shapes, st.shape).id("split-shape"),
        ),
    ))
    .title(crate::res::str::split_form_title()),)))
    .id("split-form")
}

/// The derived side: a ring of the form's shapes, redrawn as any of its signals change, with
/// a readout underneath a script can check.
fn canvas_pane(st: Scene) -> impl Piece {
    column((
        canvas(move |d, size| {
            let n = st.count.get().round().max(1.0) as usize;
            let r = st.radius.get();
            let c = Point::new(size.width / 2.0, size.height / 2.0);
            // One shape alone sits at the center; more share a ring that keeps them inside
            // the pane at whatever size the divider leaves it, each no larger than the ring
            // has room for, so a short pane shrinks them rather than piling them up.
            let ring = if n == 1 {
                0.0
            } else {
                (size.width.min(size.height) / 2.0 - r - 12.0).max(0.0)
            };
            let r = if n > 1 {
                r.min(ring * (std::f64::consts::PI / n as f64).sin())
                    .max(3.0)
            } else {
                r
            };
            let color = hue_color(st.hue.get());
            let filled = st.filled.get();
            let kind = st.shape.get();
            for i in 0..n {
                let a = std::f64::consts::TAU * i as f64 / n as f64 - std::f64::consts::FRAC_PI_2;
                let p = Point::new(c.x + ring * a.cos(), c.y + ring * a.sin());
                let shape = match kind {
                    1 => Shape::Rect(Rect::new(p.x - r, p.y - r, 2.0 * r, 2.0 * r)),
                    2 => Shape::Polygon(vec![
                        Point::new(p.x, p.y - r),
                        Point::new(p.x + r, p.y),
                        Point::new(p.x, p.y + r),
                        Point::new(p.x - r, p.y),
                    ]),
                    _ => Shape::Ellipse(Rect::new(p.x - r, p.y - r, 2.0 * r, 2.0 * r)),
                };
                if filled {
                    d.fill(shape, color);
                } else {
                    d.stroke(shape, color, 3.0);
                }
            }
            let caption = st.caption.get();
            if !caption.is_empty() {
                d.text(
                    &caption,
                    Point::new(12.0, 10.0),
                    TextStyle {
                        size: 17.0,
                        color,
                        ..Default::default()
                    },
                );
            }
        })
        .id("split-canvas")
        .grow(),
        label(move || {
            crate::res::str::split_readout(
                st.count.get().round() as i64,
                st.radius.get().round() as i64,
            )
            .format()
        })
        .id("split-readout")
        .font(Font::Footnote),
    ))
    .spacing(8.0)
    .padding(12.0)
}

/// A saturated color at `hue` degrees, the same lightness all the way round the wheel, so the
/// hue slider reads as one continuous sweep.
fn hue_color(hue: f64) -> Color {
    let h = hue.rem_euclid(360.0) / 60.0;
    let (s, v) = (0.72, 0.92);
    let c = v * s;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match h as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    Color::rgba(r + m, g + m, b + m, 1.0)
}
