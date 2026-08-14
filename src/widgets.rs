//! Reusable pieces shared by more than one page (see the `pages` modules).

use day::prelude::*;

/// The current battery reading as a localized line (Fluent; the state name stays the API's
/// enum debug form — it is a value, not prose). Shared by the Battery and About pages.
pub(crate) fn battery_line() -> LocalizedText {
    match day_part_battery::status() {
        Some(b) => crate::res::str::battery_reading(
            b.percent()
                .map(|p| format!("{p}%"))
                .unwrap_or_else(|| "?".into()),
            format!("{:?}", b.state),
        ),
        None => crate::res::str::battery_reading_none(),
    }
}

/// The arc dial: a 270° track sweep with the value centred — RESPONSIVE, drawing a centred
/// square dial scaled to whatever size the caller lays it out at (the canvas re-records on
/// `FrameChanged`). Size it with `.height`/`.grow_w` (or `.frame`) at the call site.
pub(crate) fn gauge(value: Signal<f64>) -> AnyPiece {
    canvas(move |d, size| {
        let side = size.width.min(size.height);
        if side <= 20.0 {
            return;
        }
        let r = Rect::new(
            (size.width - side) / 2.0,
            (size.height - side) / 2.0,
            side,
            side,
        )
        .inset(8.0);
        let track = Color::rgba(0.5, 0.5, 0.55, 0.35);
        let accent = crate::palette::SKY;
        let stroke_w = (side * 0.055).clamp(4.0, 9.0);
        d.stroke(
            Shape::Arc {
                rect: r,
                start_deg: 135.0,
                sweep_deg: 270.0,
            },
            track,
            stroke_w,
        );
        let frac = (value.get() / 100.0).clamp(0.0, 1.0);
        if frac > 0.0 {
            d.stroke(
                Shape::Arc {
                    rect: r,
                    start_deg: 135.0,
                    sweep_deg: 270.0 * frac,
                },
                accent,
                stroke_w,
            );
        }
        d.text(
            &format!("{:.0}", value.get()),
            Point::new(size.width / 2.0, size.height / 2.0),
            TextStyle {
                size: (side * 0.2).clamp(14.0, 30.0),
                color: accent,
                anchor: TextAnchor::Centered,
            },
        );
    })
    // Accessibility (§13): a canvas has no inherent role, so Day applies `Meter` + a spoken value
    // and label. `.id`/`.a11y` go on the canvas leaf (before any frame wrapper, a handle-less
    // layout node), so they reach the native widget. Value is a build-time snapshot (reactive
    // a11y is a follow-up).
    .a11y(move |a| {
        a.role(Role::Meter)
            .label(crate::res::str::gauge_value_label().format())
            .value(format!("{:.0}", value.get_untracked()))
    })
    .id("gauge")
}

/// Standard page scaffold (the showcase design pass): a title + optional caption header over a
/// scrollable, consistently padded content column. Every page uses it, so typography, spacing,
/// and scrolling behave identically across the app.
/// A page's title heading. When the native nav shows the destination title in its own header
/// (`Cap::NavHeader` — e.g. the Windows NavigationView), the big in-content title is redundant, so
/// it is dropped: the caption (or, lacking one, a de-emphasized title) carries the `title_id` so
/// scripts/tests still find the anchor. Elsewhere it renders the usual `Font::Title` + caption.
pub(crate) fn heading(
    title: LocalizedText,
    title_id: &'static str,
    caption: Option<LocalizedText>,
) -> AnyPiece {
    let native_header = capability(Cap::NavHeader) == Support::Native;
    match (native_header, caption) {
        (true, Some(c)) => label(c).font(Font::Subheadline).id(title_id).any(),
        (true, None) => label(title).font(Font::Subheadline).id(title_id).any(),
        (false, Some(c)) => column((
            label(title).font(Font::Title).id(title_id),
            label(c).font(Font::Footnote),
        ))
        .spacing(4.0)
        .align(HAlign::Leading)
        .any(),
        (false, None) => label(title).font(Font::Title).id(title_id).any(),
    }
}

/// The widest a page's content column grows before it stops and centres instead.
///
/// A desktop window is far wider than a comfortable reading measure, and a form stretched to
/// 1900px reads as a spreadsheet: labels drift a screen away from the controls they name, and a
/// row of buttons scatters. Capping the column and centring the remainder gives every page the
/// same spine, on every window size and every platform — and it is what makes the screenshots
/// look composed rather than merely wide.
///
/// The value is tuned against the window the screenshots are captured at (Day.toml `[window]`,
/// 1000×720pt — about 760pt of content beside the sidebar), so the cap engages there and leaves a
/// visible margin. A phone is far narrower, so it never engages on mobile and that layout is
/// unchanged; [`page_wide`] opts out for the pages whose content wants the whole canvas.
pub(crate) const CONTENT_MAX_WIDTH: f64 = 680.0;

pub(crate) fn page(
    title: LocalizedText,
    title_id: &'static str,
    caption: Option<LocalizedText>,
    body: AnyPiece,
) -> AnyPiece {
    page_inner(
        title,
        title_id,
        caption,
        None,
        body,
        Some(CONTENT_MAX_WIDTH),
    )
}

/// [`page`] with a control in the heading's corner slot — pushed to the trailing edge of the
/// heading row (upper-right in LTR, mirrored under RTL), for a page-wide switch like the Text
/// page's Selectable toggle.
pub(crate) fn page_trailing(
    title: LocalizedText,
    title_id: &'static str,
    caption: Option<LocalizedText>,
    trailing: AnyPiece,
    body: AnyPiece,
) -> AnyPiece {
    page_inner(
        title,
        title_id,
        caption,
        Some(trailing),
        body,
        Some(CONTENT_MAX_WIDTH),
    )
}

/// [`page`] without the width cap — for pages whose content is the canvas itself (a benchmark
/// patchwork, a map, a web view, a wide grid), where narrowing it would throw away the thing the
/// page exists to show.
pub(crate) fn page_wide(
    title: LocalizedText,
    title_id: &'static str,
    caption: Option<LocalizedText>,
    body: AnyPiece,
) -> AnyPiece {
    page_inner(title, title_id, caption, None, body, None)
}

fn page_inner(
    title: LocalizedText,
    title_id: &'static str,
    caption: Option<LocalizedText>,
    trailing: Option<AnyPiece>,
    body: AnyPiece,
    max_width: Option<f64>,
) -> AnyPiece {
    let head = heading(title, title_id, caption);
    let head = match trailing {
        // The corner slot: beside the heading where the big in-content title renders. On
        // native-header targets (the phones — the nav bar owns the title, so the heading is
        // the caption sentence) that sentence needs the full measure; the control rides its
        // own trailing-aligned row above it instead of squeezing the caption sideways.
        Some(t) => {
            if capability(Cap::NavHeader) == Support::Native {
                column((row((spacer(), t)).grow_w(), head))
                    .spacing(8.0)
                    .grow_w()
                    .any()
            } else {
                row((head, spacer(), t)).grow_w().any()
            }
        }
        None => head,
    };
    let content = column((head, body)).spacing(16.0).align(HAlign::Leading);
    let content = match max_width {
        Some(w) => content.max_width(w).any(),
        None => content.any(),
    };
    // The outer column is what centres: it grows to the scroll's width and aligns the capped
    // content column in the middle of it.
    scroll(
        column((content,))
            .align(HAlign::Center)
            .grow_w()
            .padding(20.0),
    )
    .any()
}

/// A numeric readout that does not resize as its value changes.
///
/// A bare `label(move || format!("{v:.0}"))` beside a slider reflows the row on every drag: `1` is
/// narrower than `8`, and `9` → `10` adds a glyph, so the slider shifts under the pointer that is
/// dragging it. `reserving` measures `widest` in this label's own font and holds that much room,
/// which is why this scales with the reader's accessibility text size where a fixed `.width()`
/// would clip.
///
/// Pass the widest string the field can ever show — `"100"` for a percentage, `"8888"` for a
/// count.
/// The `id` goes on the LABEL, not on the wrapper `reserving` returns: a script asserting the
/// readout's text has to resolve to the piece that has text, and the reservation wrapper has none.
pub(crate) fn numeric_readout(
    text: impl Fn() -> String + 'static,
    widest: &'static str,
    id: &'static str,
) -> AnyPiece {
    // Both halves of the problem: `tabular` stops the digits shifting inside the box (`1` is
    // narrower than `8`), `reserving` stops the box itself resizing when the digit count changes.
    // `.tabular()` is a Label builder method, so it comes before the Decorate modifiers.
    label(text).tabular().id(id).reserving(widest)
}

// ---------------------------------------------------------------------------
// Button styling (§5.2 Decorate + ButtonStyle)
// ---------------------------------------------------------------------------

/// A filled, centred button in one of the palette's colors.
///
/// `FilledButtonStyle` leaves the label at its natural position, which reads as off-centre the
/// moment `grow_w` stretches a button to share a grid column — so this centres it. Everything
/// here is plain composition (`padding`/`background`/`corner_radius`), so it needs no per-backend
/// code and looks the same on all nine.
pub(crate) struct Tinted {
    color: Color,
    /// Pale fills (AMBER) need dark text; the palette's own note.
    ink: bool,
}

impl ButtonStyle for Tinted {
    fn body(&self, label: AnyPiece) -> AnyPiece {
        column((label,))
            .align(HAlign::Center)
            .grow_w()
            .padding(Insets::symmetric(16.0, 10.0))
            .background(self.color)
            .corner_radius(10.0)
    }
    fn label_color(&self) -> Option<Color> {
        Some(if self.ink {
            crate::palette::INK
        } else {
            Color::WHITE
        })
    }
}

/// A tinted button in an arbitrary palette color (white label).
pub(crate) fn tinted(color: Color) -> Tinted {
    Tinted { color, ink: false }
}

/// A tinted button on a PALE fill, which takes [`crate::palette::INK`] text instead of white.
pub(crate) fn tinted_pale(color: Color) -> Tinted {
    Tinted { color, ink: true }
}

/// The page's headline action — the one thing a visitor should press first.
pub(crate) fn primary() -> Tinted {
    tinted(crate::palette::SKY)
}

/// A supporting action that still deserves color: a second, cooler voice next to [`primary`].
pub(crate) fn secondary() -> Tinted {
    tinted(crate::palette::TEAL)
}

/// Destructive or irreversible — deleting, clearing, crashing on purpose.
pub(crate) fn danger() -> Tinted {
    tinted(crate::palette::RUST)
}

// ---------------------------------------------------------------------------
// "Not supported here" banners (docs/coverage-matrix.md)
// ---------------------------------------------------------------------------

/// A banner marking a demo the current target cannot actually run.
///
/// The demo stays on screen: a visitor comparing two platforms should see the SAME pages, and a
/// missing section reads as a bug in the showcase rather than as a fact about the platform. The
/// banner says which it is, right where the disappointment would otherwise happen.
///
/// `Native` gets no banner at all — the overwhelmingly common case, and a banner on a working
/// feature is noise. `Emulated` gets the amber one, because the demo does something but not the
/// native thing. `Unsupported` gets the coral one.
pub(crate) fn support_banner(support: Support) -> Option<AnyPiece> {
    let (color, text) = match support {
        Support::Native => return None,
        Support::Emulated => (
            crate::palette::AMBER,
            crate::res::str::support_emulated_here(),
        ),
        Support::Unsupported => (
            crate::palette::CORAL,
            crate::res::str::support_missing_here(),
        ),
    };
    Some(
        row((
            // Untinted: the caution gold (#F2C94C) is authored into the glyph itself. Tinting
            // it here would leave it grey on the backends whose `vector` piece has no tint arm
            // (Qt, web — docs/vectors.md), and the icon has to read as a caution mark rather
            // than as the first letter of the sentence beside it.
            vector(crate::res::vectors::support_warning.clone()).frame(18.0, 18.0),
            // `grow_w`, not `grow`: the label takes the width but keeps the height of its text,
            // so the row is exactly as tall as the wrapped sentence.
            label(text).font(Font::Footnote).color(color).grow_w(),
        ))
        .spacing(8.0)
        // TOP, not centre. This sentence wraps to two or three lines on a narrow window, and a
        // centred icon then sits BETWEEN them — the mark belongs beside the line the reader
        // starts on. `VAlign::FirstBaseline` would not help: a vector has no text baseline, so a
        // baseline row falls back to centring it (docs/baseline.md).
        .align(VAlign::Top)
        .padding(10.0)
        .background(Color::rgba(color.r, color.g, color.b, 0.14))
        .corner_radius(8.0)
        .any(),
    )
}

/// The banner as a form child: renders nothing at all where the feature works, so a section can
/// carry it unconditionally as its first row.
pub(crate) fn support_note(support: Support) -> AnyPiece {
    match support_banner(support) {
        Some(banner) => banner,
        None => column(()).any(),
    }
}

/// A button and the result it produces: side by side where they fit, stacked where they do not.
///
/// Side by side, the button takes its natural width and the result takes what is left — which on a
/// phone is a few characters once the button carries a translated label ("Récupérer depuis
/// localhost" is most of a 411dp row). The result then wraps mid-word, because at that width no
/// break point helps: the remaining column is narrower than a single token like `day-http-ok`.
/// Stacking at Compact width gives the result the whole row instead.
///
/// `size_class()` is a tracked read (docs/size-classes.md), so this re-lays out when the window
/// crosses a breakpoint — a rotation, a foldable opening, a desktop window dragged narrow.
pub(crate) fn action_result(action: AnyPiece, result: AnyPiece) -> AnyPiece {
    let compact = day::size_class()
        .map(|c| c.width == WidthClass::Compact)
        .unwrap_or(false);
    if compact {
        // Leading, not the container default of centered: stacked, these read as a control and
        // the line it produced, and a centered result floats away from the button it belongs to.
        column((action, result))
            .spacing(6.0)
            .align(HAlign::Leading)
            .any()
    } else {
        row((action, result)).spacing(8.0).any()
    }
}
