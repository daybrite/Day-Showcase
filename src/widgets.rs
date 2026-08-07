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

pub(crate) fn history(count: Signal<i64>) -> AnyPiece {
    let entries = Signal::new(Vec::<(u64, i64)>::new());
    let next_id = Signal::new(0u64);
    watch(
        move || count.get(),
        move |new, _old| {
            let id = next_id.get_untracked();
            next_id.set(id + 1);
            let v = *new;
            entries.update(|e| {
                e.push((id, v));
                if e.len() > 8 {
                    e.remove(0);
                }
            });
        },
    );
    // The enclosing form section's title carries the "History" heading; an empty collection
    // still shows the hint line so the card never renders blank.
    column((
        when(
            move || entries.with(|e| e.is_empty()),
            move || label(crate::res::str::history_hint()).font(Font::Footnote),
        ),
        each(
            move || entries.get(),
            |e| e.0,
            move |slot: ItemSlot<(u64, i64), u64>| {
                label(move || crate::res::str::history_entry(slot.field(|t| t.1)).format())
            },
        ),
    ))
    .spacing(4.0)
    .align(HAlign::Leading)
    .any()
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
    page_inner(title, title_id, caption, body, Some(CONTENT_MAX_WIDTH))
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
    page_inner(title, title_id, caption, body, None)
}

fn page_inner(
    title: LocalizedText,
    title_id: &'static str,
    caption: Option<LocalizedText>,
    body: AnyPiece,
    max_width: Option<f64>,
) -> AnyPiece {
    let content = column((heading(title, title_id, caption), body))
        .spacing(16.0)
        .align(HAlign::Leading);
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
