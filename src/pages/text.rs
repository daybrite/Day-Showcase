use day::prelude::*;
use day_piece_datetime::{DayDate, date_picker};

use crate::palette::{AMBER, SKY, TEAL, VIOLET};
use crate::widgets::page_trailing;

/// Typography playground: every semantic text style (mapped to the platform's native styles + Dynamic
/// Type / font-scale accessibility sizing), font weights, bold/italic, color, and accessibility-scaled
/// custom sizes. See docs/text.md.
///
/// The Selectable toggle in the heading's corner opts EVERY text piece on the page in and out of
/// `.selectable()` (docs/text.md) — regular labels, custom-font specimens, and links alike. The
/// body rebuilds through the `when` arms on each flip; the modifier itself is one-shot.
pub(crate) fn text_page() -> AnyPiece {
    let sel_on = Signal::new(true);
    // The markdown editor's buffer lives HERE, not in `sections`: flipping the Selectable toggle
    // rebuilds the body, and a signal created down there would reset whatever the user typed.
    let md = Signal::new(crate::res::str::text_markdown_sample().format());
    page_trailing(
        crate::res::str::nav_text(),
        "text-title",
        Some(crate::res::str::text_caption()),
        row((
            label(crate::res::str::text_selectable_toggle()).font(Font::Subheadline),
            toggle(sel_on).id("text-selectable-toggle"),
        ))
        .spacing(8.0)
        .any(),
        column((
            when(move || sel_on.get(), move || sections(true, md)),
            when(move || !sel_on.get(), move || sections(false, md)),
        ))
        .any(),
    )
}

/// Apply the page's Selectable state to one text piece. Per piece, not on a containing
/// section: `.selectable()` on a container only reaches the text within on backends whose
/// selection affordance cascades (docs/text.md) — per-label is what works everywhere.
fn sel(on: bool, p: impl Piece) -> AnyPiece {
    if on { p.selectable().any() } else { p.any() }
}

/// The page body, built for one Selectable state (the `when` arms above rebuild it on flip).
fn sections(on: bool, md: Signal<String>) -> impl Piece{
    // A style name (localized) rendered IN its own style — a self-documenting type specimen.
    // The dayscript id keeps the stable English style id regardless of locale.
    fn specimen(on: bool, id: &'static str, name: LocalizedText, f: Font) -> AnyPiece {
        sel(on, label(name).font(f).id_keyed("text-style", id))
    }
    // Every semantic style (largest → smallest), each rendered in its own style.
    let styles = section((
        specimen(
            on,
            "Large Title",
            crate::res::str::text_style_large_title(),
            Font::LargeTitle,
        ),
        specimen(
            on,
            "Title",
            crate::res::str::text_style_title(),
            Font::Title,
        ),
        specimen(
            on,
            "Title 2",
            crate::res::str::text_style_title2(),
            Font::Title2,
        ),
        specimen(
            on,
            "Title 3",
            crate::res::str::text_style_title3(),
            Font::Title3,
        ),
        specimen(
            on,
            "Headline",
            crate::res::str::text_style_headline(),
            Font::Headline,
        ),
        specimen(
            on,
            "Subheadline",
            crate::res::str::text_style_subheadline(),
            Font::Subheadline,
        ),
        specimen(on, "Body", crate::res::str::text_style_body(), Font::Body),
        specimen(
            on,
            "Callout",
            crate::res::str::text_style_callout(),
            Font::Callout,
        ),
        specimen(
            on,
            "Footnote",
            crate::res::str::text_style_footnote(),
            Font::Footnote,
        ),
        specimen(
            on,
            "Caption",
            crate::res::str::text_style_caption(),
            Font::Caption,
        ),
        specimen(
            on,
            "Caption 2",
            crate::res::str::text_style_caption2(),
            Font::Caption2,
        ),
    ))
    .title(crate::res::str::text_styles_header());
    // Font weights on a body-size line.
    let weights = section((
        sel(
            on,
            label(crate::res::str::text_weight_ultralight())
                .weight(FontWeight::UltraLight)
                .id("text-w-ultralight"),
        ),
        sel(
            on,
            label(crate::res::str::text_weight_light()).weight(FontWeight::Light),
        ),
        sel(
            on,
            label(crate::res::str::text_weight_regular()).weight(FontWeight::Regular),
        ),
        sel(
            on,
            label(crate::res::str::text_weight_medium()).weight(FontWeight::Medium),
        ),
        sel(
            on,
            label(crate::res::str::text_weight_semibold()).weight(FontWeight::Semibold),
        ),
        sel(
            on,
            label(crate::res::str::text_weight_bold())
                .weight(FontWeight::Bold)
                .id("text-w-bold"),
        ),
        sel(
            on,
            label(crate::res::str::text_weight_heavy()).weight(FontWeight::Heavy),
        ),
        sel(
            on,
            label(crate::res::str::text_weight_black()).weight(FontWeight::Black),
        ),
    ))
    .title(crate::res::str::text_weights_header());
    // Bold / italic / both, and everything-at-once.
    let styling = section((
        sel(
            on,
            label(crate::res::str::text_bold()).bold().id("text-bold"),
        ),
        sel(
            on,
            label(crate::res::str::text_italic())
                .italic()
                .id("text-italic"),
        ),
        sel(
            on,
            label(crate::res::str::text_bolditalic())
                .bold()
                .italic()
                .id("text-bolditalic"),
        ),
        sel(
            on,
            label(crate::res::str::text_emphasis_label())
                .font(Font::Title2)
                .weight(FontWeight::Heavy)
                .italic()
                .color(VIOLET)
                .id("text-emphasis"),
        ),
    ))
    .title(crate::res::str::text_styling_header());
    // Color.
    let colors = section((row((
        sel(
            on,
            label(crate::res::str::color_red()).color(Color::hex(0xE74C3C)),
        ),
        sel(
            on,
            label(crate::res::str::color_green()).color(Color::hex(0x27AE60)),
        ),
        sel(
            on,
            label(crate::res::str::color_blue()).color(Color::hex(0x2F6FDE)),
        ),
        sel(
            on,
            label(crate::res::str::color_orange()).color(Color::hex(0xE67E22)),
        ),
    ))
    .spacing(12.0),))
    .title(crate::res::str::text_colors_header());
    // Custom sizes — Font::System(pt), still scaled by the platform accessibility text size.
    let custom = section((
        sel(
            on,
            label(crate::res::str::text_custom_note()).font(Font::Footnote),
        ),
        sel(
            on,
            label(crate::res::str::text_size_pt(13)).font(Font::System(13.0)),
        ),
        sel(
            on,
            label(crate::res::str::text_size_pt(20)).font(Font::System(20.0)),
        ),
        sel(
            on,
            label(crate::res::str::text_size_pt(28))
                .font(Font::System(28.0))
                .id("text-custom-28"),
        ),
        sel(
            on,
            label(crate::res::str::text_size_pt(40))
                .font(Font::System(40.0))
                .weight(FontWeight::Bold),
        ),
    ))
    .title(crate::res::str::text_custom_header());
    // Bundled custom fonts (docs/resources.md): the three families ship in the app's fonts/
    // directory; `Font::Custom` references them by FAMILY name (what the font file reports),
    // and `day build` + the backend make that name resolve on every platform.
    let fonts = section((
        sel(
            on,
            label(crate::res::str::text_fonts_note()).font(Font::Footnote),
        ),
        // The family NAMES stay Latin (proper nouns, and the sample must exercise the font);
        // the descriptions localize — non-Latin glyphs fall back to the system font mid-line.
        sel(
            on,
            label(crate::res::str::text_font_pacifico())
                .font(Font::custom(crate::res::fonts::pacifico, 24.0))
                .id("text-font-pacifico"),
        ),
        sel(
            on,
            label(crate::res::str::text_font_bungee())
                .font(Font::custom(crate::res::fonts::bungee, 20.0))
                .id("text-font-bungee"),
        ),
        sel(
            on,
            label(crate::res::str::text_font_specialelite())
                .font(Font::custom(crate::res::fonts::special_elite, 20.0))
                .id("text-font-specialelite"),
        ),
        sel(
            on,
            label(crate::res::str::text_font_pacifico_lg())
                .font(Font::custom(crate::res::fonts::pacifico, 36.0))
                .color(SKY)
                .id("text-font-pacifico-lg"),
        ),
    ))
    .title(crate::res::str::text_fonts_header());

    // Links (docs/text.md): tappable accent-coloured text that opens a URL in the system browser
    // (or the mail client for `mailto:`) via the backend's `open_url`. `.color()` overrides the
    // default tint; `.font()` and `.bold()` style the run like a label. `.selectable()` on a
    // link is honored where the backend's link widget has a selection affordance, and is a
    // silent no-op elsewhere — the tap keeps working either way.
    let links = section((
        sel(
            on,
            label(crate::res::str::text_links_caption()).font(Font::Footnote),
        ),
        // "daybrite.dev" is the URL itself (a value), so it stays raw.
        sel(
            on,
            link("daybrite.dev", "https://daybrite.dev").id("text-link-web"),
        ),
        sel(
            on,
            link(
                crate::res::str::text_link_icons_label().format(),
                "https://fonts.google.com/icons",
            )
            .font(Font::Footnote)
            .id("text-link-icons"),
        ),
        sel(
            on,
            link(
                crate::res::str::text_link_mail_label().format(),
                "mailto:hello@daybrite.dev",
            )
            .color(TEAL)
            .id("text-link-mail"),
        ),
    ))
    .title(crate::res::str::text_links_section());

    // The `.selectable()` core modifier itself (docs/text.md) — supported on every backend
    // (on iOS the label is rebuilt as a read-only text view, since a UILabel has no selection
    // affordance). This section rides the page toggle like everything else; its text explains
    // what the toggle is exercising.
    let selectable = section((
        sel(
            on,
            label(crate::res::str::tweaks_selectable_caption()).font(Font::Footnote),
        ),
        sel(
            on,
            label(crate::res::str::tweaks_selectable_text()).id("text-selectable"),
        ),
    ))
    .title(crate::res::str::tweaks_selectable_title());

    // Bundled fonts lead the page: the most visually distinctive section, and the one the
    // walkthrough screenshot must show above the fold.
    form((
        fonts,
        styles,
        weights,
        styling,
        rich(on),
        markdown_live(on, md),
        colors,
        custom,
        links,
        selectable,
        baseline(),
    ))
    
}

/// Styled RUNS inside one wrapping paragraph (docs/text-runs.md): emphasis, code, colour and a
/// strike, all in a single label. Composing several labels in a row looks similar on one line and
/// then wraps wrongly, breaks selection, and reads as separate items to a screen reader, which is
/// the reason this exists.
///
/// The text is built in code rather than from `res::str`, since the runs index the string by byte
/// range: a translated string needs its own ranges, which is a per-locale build the localization
/// pipeline does not do yet.
///
/// A banner marks the backends that render the text plain; `Cap::TextRuns` answers for it.
fn rich(on: bool) -> impl Piece{
    // The sample names each style with its markdown spelling and renders it that way, so the
    // supported set reads off the label itself.
    let (text, runs) = TextBuilder::new()
        .base(Font::Body)
        .text("Inline styles in one label: ")
        .strong("**bold**")
        .text(", ")
        .emphasis("*italic*")
        .text(", ")
        .code("`code`")
        .text(", ")
        .strikethrough("~~strikethrough~~")
        .text(", and ")
        .colored("colour", TEAL)
        .text(", ")
        .underline("underline")
        .text(", ")
        .highlight("highlight", AMBER)
        .text(" and a ")
        .sized("relative size", 1.4)
        .text(
            ", which markdown has no syntax for. A relative size scales the semantic style, so it              still follows the reader's text-size setting. Links are runs as well: they render,              but ",
        )
        .text("nothing opens them yet.")
        .build();
    section((
        crate::widgets::support_note(crate::support::cap(Cap::TextRuns)),
        sel(on, label(text).runs(runs).id("text-runs")),
    ))
    .title(crate::res::str::text_runs_section())
    
}

/// Inline markdown parsed at RUN TIME (docs/markdown.md): a text area the user edits, and a
/// `.markdown()` label under it that re-parses on every keystroke.
///
/// This is the case a compile-time macro cannot serve — the string is not a literal — and it is
/// the same path a translated string or a value off the network takes. The sample seeds a link,
/// which makes the backing on iOS a text view from the start (docs/text-runs.md); tapping it
/// reports through `.on_link()`, which here shows the target rather than opening it.
fn markdown_live(on: bool, md: Signal<String>) -> impl Piece{
    let opened = Signal::new(String::new());
    section((
        sel(
            on,
            label(crate::res::str::text_markdown_caption()).font(Font::Footnote),
        ),
        text_area(md).min_lines(3).max_lines(5).id("text-md-input"),
        divider(),
        sel(
            on,
            label(move || md.get())
                .markdown()
                .on_link(move |url| opened.set(url.to_string()))
                .id("text-md-output"),
        ),
        when(
            move || !opened.get().is_empty(),
            move || {
                label(crate::res::str::text_markdown_opened(opened))
                    .font(Font::Footnote)
                    .color(TEAL)
                    .id("text-md-opened")
            },
        ),
    ))
    .title(crate::res::str::text_markdown_section())
    
}

/// Baseline alignment (docs/baseline.md), with a toggle that turns it off so the difference is
/// visible rather than asserted.
///
/// The rows are built to make the two states disagree as loudly as possible: each mixes a
/// Body-size label, a control whose text is inset by its own border, and a trailing unit in
/// Caption size. Aligned, all three sit on one line. Centered — what every row did before Day
/// had a baseline concept — each text sits in the middle of its own box, so the three drift
/// apart by the difference in their heights.
///
/// These use `row(..).align(..)` rather than `labeled`, which is baseline-aligned with no way to
/// opt out: the point here is to show both states side by side in time.
fn baseline() -> impl Piece{
    let aligned = Signal::new(true);
    let qty = Signal::new("12".to_string());
    let due = Signal::new(DayDate::new(2026, 3, 9).expect("valid demo date"));
    let support = capability(Cap::BaselineAlignment);
    section((
        crate::widgets::support_note(support),
        label(crate::res::str::text_baseline_caption()).font(Font::Footnote),
        row((
            label(crate::res::str::text_baseline_toggle()).font(Font::Subheadline),
            toggle(aligned).id("text-baseline-toggle"),
        ))
        .spacing(8.0),
        when(
            move || aligned.get(),
            move || baseline_rows(VAlign::FirstBaseline, qty, due),
        ),
        when(
            move || !aligned.get(),
            move || baseline_rows(VAlign::Center, qty, due),
        ),
    ))
    .title(crate::res::str::text_baseline_section())
    
}

/// The three demo rows at one alignment (the `when` arms above rebuild them on flip). The ids
/// carry the alignment so a dayscript can tell the two states apart.
fn baseline_rows(align: VAlign, qty: Signal<String>, due: Signal<DayDate>) -> impl Piece{
    let tag = match align {
        VAlign::FirstBaseline => "baseline",
        _ => "centered",
    };
    let lead = |t: LocalizedText| label(t).font(Font::Body).width(90.0);
    column((
        // A bordered field between two labels: the field insets its text, the labels do not.
        row((
            lead(crate::res::str::text_baseline_quantity()),
            text_field(qty)
                .width(70.0)
                .id_keyed("text-baseline-qty", tag),
            label(crate::res::str::text_baseline_unit()).font(Font::Caption),
        ))
        .spacing(8.0)
        .align(align),
        // Type sizes alone, no control: a Title-size number beside Body and Caption text.
        row((
            lead(crate::res::str::text_baseline_total()),
            label("$1,240.00")
                .font(Font::Title2)
                .id_keyed("text-baseline-total", tag),
            label(crate::res::str::text_baseline_currency()).font(Font::Caption),
        ))
        .spacing(8.0)
        .align(align),
        // The tallest control on the page: a date picker's stepper makes its box far taller than
        // its text, which is where centering goes most obviously wrong.
        row((
            lead(crate::res::str::text_baseline_due()),
            date_picker(due).id_keyed("text-baseline-due", tag),
        ))
        .spacing(8.0)
        .align(align),
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    
}
