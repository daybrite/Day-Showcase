use day::prelude::*;

use crate::palette::{SKY, TEAL, VIOLET};
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
            when(move || sel_on.get(), move || sections(true)),
            when(move || !sel_on.get(), move || sections(false)),
        ))
        .any(),
    )
}

/// Apply the page's Selectable state to one text piece. Per piece, not on a containing
/// section: `.selectable()` on a container only reaches the text within on backends whose
/// selection affordance cascades (docs/text.md) — per-label is what works everywhere.
fn sel(on: bool, p: impl Piece) -> AnyPiece {
    if on { p.selectable() } else { p.any() }
}

/// The page body, built for one Selectable state (the `when` arms above rebuild it on flip).
fn sections(on: bool) -> AnyPiece {
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
        fonts, styles, weights, styling, colors, custom, links, selectable,
    ))
    .any()
}
