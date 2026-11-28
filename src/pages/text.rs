use day::datetime::{DayDate, date_picker};
use day::prelude::*;

use crate::gallery;
use crate::palette::{AMBER, CORAL, SKY, TEAL, VIOLET};
use crate::widgets::page_cards_trailing;

/// Typography playground (docs/text.md): every semantic text style, mapped to the platform's
/// native styles and its accessibility text size; weights, bold and italic, color, bundled
/// fonts, relative sizes, the reader's text scale and presets; styled runs, decorations, small
/// capitals and oldstyle figures; Markdown parsed as it is typed and a Markdown view streamed in;
/// line limits, truncation, spacing and alignment of a wrapped paragraph; text in several
/// scripts and both directions; links, selection and baseline alignment.
///
/// The Selectable toggle in the heading's corner opts every text piece on the page in and out of
/// `.selectable()` (docs/text.md): regular labels, custom-font specimens, and links alike. The
/// body rebuilds through the `when` arms on each flip; the modifier itself is one-shot.
pub(crate) fn text_page() -> AnyPiece {
    let sel_on = Signal::new(true);
    // The markdown editor's buffer lives here, not in `sections`: flipping the Selectable toggle
    // rebuilds the body, and a signal created down there would reset whatever the user typed.
    let md = Signal::new(crate::res::str::text_markdown_sample().format());
    page_cards_trailing(
        crate::res::str::nav_text(),
        day::id!(text_title),
        row((
            label(crate::res::str::text_selectable_toggle()).font(Font::Subheadline),
            toggle(sel_on).id(day::id!(text_selectable_toggle)),
        ))
        .spacing(Space::S)
        .any(),
        column((
            when(move || sel_on.get(), move || sections(true, md)),
            when(move || !sel_on.get(), move || sections(false, md)),
        ))
        .grow(Dimension::Width)
        .any(),
    )
    .any()
}

/// Apply the page's Selectable state to one text piece. Per piece, not on a containing
/// section: `.selectable()` on a container only reaches the text within on backends whose
/// selection affordance cascades (docs/text.md); per-label is what works everywhere.
fn sel(on: bool, p: impl Piece) -> impl Piece {
    p.apply_if(on, |p| p.selectable())
}

/// The page body, built for one Selectable state (the `when` arms above rebuild it on flip).
fn sections(on: bool, md: Signal<String>) -> impl Piece {
    // A style name (localized) rendered in its own style, a self-documenting type specimen.
    // The dayscript id keeps the stable English style id regardless of locale.
    fn specimen(on: bool, id: &'static str, name: LocalizedText, f: Font) -> impl Piece {
        sel(on, label(name).font(f).id(day::id!(text_style, id)))
    }
    // Every semantic style (largest → smallest), each rendered in its own style.
    let styles = crate::gallery::panel(
        crate::res::str::text_styles_header(),
        (
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
        ),
    );
    // Font weights on a body-size line.
    let weights = crate::gallery::panel(
        crate::res::str::text_weights_header(),
        (
            sel(
                on,
                label(crate::res::str::text_weight_ultralight())
                    .weight(FontWeight::UltraLight)
                    .id(day::id!(text_w_ultralight)),
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
                    .id(day::id!(text_w_bold)),
            ),
            sel(
                on,
                label(crate::res::str::text_weight_heavy()).weight(FontWeight::Heavy),
            ),
            sel(
                on,
                label(crate::res::str::text_weight_black()).weight(FontWeight::Black),
            ),
        ),
    );
    // Bold / italic / both, and everything-at-once.
    let styling = crate::gallery::panel(
        crate::res::str::text_styling_header(),
        (
            sel(
                on,
                label(crate::res::str::text_bold())
                    .bold()
                    .id(day::id!(text_bold)),
            ),
            sel(
                on,
                label(crate::res::str::text_italic())
                    .italic()
                    .id(day::id!(text_italic)),
            ),
            sel(
                on,
                label(crate::res::str::text_bolditalic())
                    .bold()
                    .italic()
                    .id(day::id!(text_bolditalic)),
            ),
            sel(
                on,
                label(crate::res::str::text_emphasis_label())
                    .font(Font::Title2)
                    .weight(FontWeight::Heavy)
                    .italic()
                    .color(VIOLET)
                    .id(day::id!(text_emphasis)),
            ),
            row((
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
            .spacing(Space::M)
            .fit(RowFit::Wrap { run_spacing: 4.0 }),
        ),
    );
    // Custom sizes: `Font::System(pt)`, the one specimen of a point size on the page. The
    // platform's accessibility text size still scales it.
    let custom = crate::gallery::panel(
        crate::res::str::text_custom_header(),
        (
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
                    .id(day::id!(text_custom_28)),
            ),
            sel(
                on,
                label(crate::res::str::text_size_pt(40))
                    .font(Font::System(40.0))
                    .weight(FontWeight::Bold),
            ),
        ),
    );
    // Bundled custom fonts (docs/resources.md): the three families ship in the app's fonts/
    // directory; `Font::Custom` references them by family name (what the font file reports),
    // and `day build` + the backend make that name resolve on every platform.
    let fonts = crate::gallery::panel(
        crate::res::str::text_fonts_header(),
        (
            // The family names stay Latin (proper nouns, and the sample must exercise the font);
            // the descriptions localize; non-Latin glyphs fall back to the system font mid-line.
            sel(
                on,
                label(crate::res::str::text_font_pacifico())
                    .font(Font::custom(crate::res::fonts::pacifico, 24.0))
                    .id(day::id!(text_font_pacifico)),
            ),
            sel(
                on,
                label(crate::res::str::text_font_bungee())
                    .font(Font::custom(crate::res::fonts::bungee, 20.0))
                    .id(day::id!(text_font_bungee)),
            ),
            sel(
                on,
                label(crate::res::str::text_font_specialelite())
                    .font(Font::custom(crate::res::fonts::special_elite, 20.0))
                    .id(day::id!(text_font_specialelite)),
            ),
            sel(
                on,
                label(crate::res::str::text_font_pacifico_lg())
                    .font(Font::custom(crate::res::fonts::pacifico, 36.0))
                    .color(SKY)
                    .id(day::id!(text_font_pacifico_lg)),
            ),
        ),
    );

    // Bundled fonts lead the page: the most visually distinctive section, and the one the
    // walkthrough screenshot must show above the fold.
    gallery::balanced(
        2,
        vec![
            fonts.any(),
            styles.any(),
            weights.any(),
            styling.any(),
            rich(on).any(),
            decorations(on).any(),
            markdown_view().any(),
            markdown_live(on, md).any(),
            custom.any(),
            scales(on).any(),
            presets(on).any(),
            line_limits(on).any(),
            spacing(on).any(),
            alignment(on).any(),
            scripts(on).any(),
            links(on).any(),
            selection(on).any(),
            baseline().any(),
        ],
    )
}

/// Links (docs/text.md): `link(..)` is a label whose whole text is a link, so every label builder
/// applies to it. A target that is a web address opens in the browser (or the mail client for
/// `mailto:`); `.on_link` hands the tap to the app instead.
fn links(on: bool) -> impl Piece {
    let opened = Signal::new(String::new());
    crate::gallery::panel(
        crate::res::str::text_links_section(),
        (
            // "daybrite.dev" is the URL itself (a value), so it stays raw.
            sel(
                on,
                link("daybrite.dev", "https://daybrite.dev").id(day::id!(text_link_web)),
            ),
            sel(
                on,
                link(
                    crate::res::str::text_link_icons_label().format(),
                    "https://fonts.google.com/icons",
                )
                .font(Font::Footnote)
                .id(day::id!(text_link_icons)),
            ),
            sel(
                on,
                link(
                    crate::res::str::text_link_mail_label().format(),
                    "mailto:hello@daybrite.dev",
                )
                .color(TEAL)
                .bold()
                .id(day::id!(text_link_mail)),
            ),
            // An address too long for the card, kept to one line with its two ends showing.
            sel(
                on,
                link(LONG_ADDRESS, LONG_ADDRESS)
                    .font(Font::Footnote)
                    .single_line()
                    .truncation(Truncation::Middle)
                    .id(day::id!(text_link_long))
                    .grow(Dimension::Width),
            ),
            sel(
                on,
                link(
                    crate::res::str::text_link_handled_label().format(),
                    "https://daybrite.dev/docs/reference/text",
                )
                .on_link(move |url| opened.set(url.to_string()))
                .id(day::id!(text_link_handled)),
            ),
            when(
                move || !opened.get().is_empty(),
                move || {
                    label(crate::res::str::text_markdown_opened(opened))
                        .font(Font::Footnote)
                        .secondary()
                        .single_line()
                        .truncation(Truncation::Middle)
                        .id(day::id!(text_link_opened))
                },
            ),
        ),
    )
}

/// A web address long enough to need truncating. A value, so it is the same in every language.
const LONG_ADDRESS: &str =
    "https://images.nasa.gov/details/GSFC_20171208_Archive_e001386/the-blue-marble";

/// Selection (docs/text.md): `.selectable()` lets the reader select and copy, and `.on_selection`
/// also tells the app what is selected. Both ride the page's Selectable toggle.
fn selection(on: bool) -> impl Piece {
    let picked = Signal::new(String::new());
    let text = label(crate::res::str::tweaks_selectable_text());
    let text = if on {
        text.on_selection(move |selected| picked.set(selected.text.clone()))
    } else {
        text
    };
    crate::gallery::panel(
        crate::res::str::tweaks_selectable_title(),
        (
            crate::widgets::support_banner(capability(Cap::LabelSelection)),
            sel(on, text.id(day::id!(text_selectable))),
            label(move || {
                if picked.with(String::is_empty) {
                    crate::res::str::text_selection_none().format()
                } else {
                    crate::res::str::text_selection_value(picked.get()).format()
                }
            })
            .font(Font::Footnote)
            .secondary()
            .max_lines(2)
            .id(day::id!(text_selection_readout)),
        ),
    )
}

/// Styled runs inside one wrapping paragraph (docs/text-runs.md): emphasis, code, color, a
/// strike, a highlight, a relative size, a wavy colored underline and a link in a single label.
/// Several labels in a row look similar on one line, then wrap wrongly,
/// break selection, and read as separate items to a screen reader.
///
/// Every phrase comes from the catalog and the builder computes the byte ranges as it appends
/// them, so the runs land on the right words in every language.
///
/// A banner marks the backends that render the text plain; `Cap::TextRuns` answers for it.
fn rich(on: bool) -> impl Piece {
    let gap = " \u{00B7} ";
    let word = |text: LocalizedText| text.format();
    let (text, runs) = TextBuilder::new()
        .base(Font::Body)
        .strong(&word(crate::res::str::textareas_mark_bold()))
        .text(gap)
        .emphasis(&word(crate::res::str::textareas_mark_italic()))
        .text(gap)
        .code(&word(crate::res::str::text_run_code()))
        .text(gap)
        .strikethrough(&word(crate::res::str::textareas_mark_strike()))
        .text(gap)
        .colored(&word(crate::res::str::text_run_color()), TEAL)
        .text(gap)
        .highlight(&word(crate::res::str::text_run_highlight()), AMBER)
        .text(gap)
        .sized(&word(crate::res::str::text_run_larger()), 1.4)
        .text(gap)
        .underline_styled(
            &word(crate::res::str::text_underline_wavy()),
            Decoration::wavy().color(CORAL),
        )
        .text(gap)
        // The link's text is its address (a value), so it stays as written.
        .link("daybrite.dev", "https://daybrite.dev")
        .build();
    crate::gallery::panel(
        crate::res::str::text_runs_section(),
        (
            crate::widgets::support_banner(crate::support::cap(Cap::TextRuns)),
            sel(on, label(text).runs(runs).id(day::id!(text_runs))),
        ),
    )
}

/// Decorations and font features (docs/text.md): a line under or through a whole label in each
/// style, in the text's color or one of its own, and the small capitals and oldstyle figures a
/// font carries. A toolkit that cannot draw a style draws a single line.
fn decorations(on: bool) -> impl Piece {
    let styled = |name: LocalizedText, decoration: Decoration, id: &'static str| {
        sel(
            on,
            label(name)
                .underline(decoration)
                .id(day::id!(text_decoration, id)),
        )
    };
    crate::gallery::panel(
        crate::res::str::text_decorations_header(),
        (
            row((
                styled(
                    crate::res::str::text_underline_single(),
                    Decoration::SINGLE,
                    "single",
                ),
                styled(
                    crate::res::str::text_underline_double(),
                    Decoration::double().color(SKY),
                    "double",
                ),
                styled(
                    crate::res::str::text_underline_dotted(),
                    Decoration::dotted().color(TEAL),
                    "dotted",
                ),
                styled(
                    crate::res::str::text_underline_wavy(),
                    Decoration::wavy().color(CORAL),
                    "wavy",
                ),
            ))
            .spacing(Space::M)
            .fit(RowFit::Wrap { run_spacing: 6.0 }),
            row((
                sel(
                    on,
                    label(crate::res::str::text_preset_moon_value())
                        .strikethrough(Decoration::SINGLE)
                        .secondary()
                        .id(day::id!(text_strike)),
                ),
                sel(
                    on,
                    label(crate::res::str::text_preset_moon_value())
                        .strikethrough(Decoration::double().color(CORAL))
                        .id(day::id!(text_strike_double)),
                ),
            ))
            .spacing(Space::M),
            labeled(
                crate::res::str::text_small_caps(),
                sel(
                    on,
                    label(crate::res::str::text_small_caps_sample())
                        .small_caps()
                        .letter_spacing(0.06)
                        .id(day::id!(text_small_caps)),
                ),
            ),
            labeled(
                crate::res::str::text_figures(),
                column((
                    sel(on, label(FIGURES_SAMPLE).id(day::id!(text_figures_lining))),
                    sel(
                        on,
                        label(FIGURES_SAMPLE)
                            .oldstyle_figures()
                            .id(day::id!(text_figures_oldstyle)),
                    ),
                ))
                .spacing(2.0)
                .align(HAlign::Leading),
            ),
        ),
    )
}

/// Figures to set in lining and oldstyle forms. A value, the same in every language.
const FIGURES_SAMPLE: &str = "1969 · 384,400 · 0.0123";

/// `.font_scale(k)`: a semantic style made larger or smaller by a factor, so the label still
/// follows the reader's text-size setting, which a point size would only do by accident.
fn scales(on: bool) -> impl Piece {
    let specimen = |style: LocalizedText, font: Font, scale: f64, id: &'static str| {
        sel(
            on,
            label(crate::res::str::text_scale_of(
                crate::res::str::TextScaleOf {
                    style: style.format(),
                    scale: format!("{scale}"),
                },
            ))
            .font(font)
            .font_scale(scale)
            .id(day::id!(text_scale, id)),
        )
    };
    crate::gallery::panel(
        crate::res::str::text_scale_header(),
        (
            specimen(crate::res::str::text_style_body(), Font::Body, 0.8, "small"),
            specimen(crate::res::str::text_style_body(), Font::Body, 1.0, "one"),
            specimen(crate::res::str::text_style_body(), Font::Body, 1.5, "large"),
            specimen(
                crate::res::str::text_style_caption(),
                Font::Caption,
                2.0,
                "caption",
            ),
            // The reader's text-size setting, read as it changes, and a badge that does not
            // grow with it because it has a fixed place to fit.
            row((
                label(move || {
                    crate::res::str::text_scale_reading(format!("{:.2}", day::text_scale()))
                        .format()
                })
                .font(Font::Footnote)
                .secondary()
                .tabular()
                .id(day::id!(text_scale_reading))
                .grow(Dimension::Width),
                label(crate::res::str::text_scale_fixed())
                    .font(Font::Caption)
                    .max_text_scale(1.0)
                    .single_line()
                    .id(day::id!(text_scale_fixed))
                    .padding(Insets::symmetric(10.0, 4.0))
                    .background(gallery::Tone::Info.tint())
                    .corner_radius(Corner::CAPSULE),
            ))
            .spacing(Space::S)
            .align(VAlign::Center),
        ),
    )
}

/// The name over a figure: a small, heavy, secondary line in small capitals, spaced out a little.
const EYEBROW: TextPreset = TextPreset::new(Font::Caption)
    .weight(FontWeight::Semibold)
    .small_caps()
    .letter_spacing(0.04)
    .secondary()
    .single_line();
/// A figure: large, bold, with digits of one width.
const FIGURE: TextPreset = TextPreset::new(Font::Title).bold().tabular();
/// The unit after a figure.
const UNIT: TextPreset = TextPreset::new(Font::Callout).color(Ink::Fixed(CORAL));

/// `TextPreset`: three named styles, each a constant, applied to two readings with
/// `.preset(&NAME)`. The two readings share every style without repeating a builder chain.
fn presets(on: bool) -> impl Piece {
    let figure = |name: LocalizedText, value: LocalizedText, unit: LocalizedText, id| {
        column((
            sel(on, label(name).preset(&EYEBROW)),
            row((
                sel(on, label(value).preset(&FIGURE).id(id)),
                sel(on, label(unit).preset(&UNIT)),
            ))
            .spacing(Space::XS)
            .align(VAlign::FirstBaseline),
        ))
        .spacing(2.0)
        .align(HAlign::Leading)
        .grow(Dimension::Width)
    };
    crate::gallery::panel(
        crate::res::str::text_preset_header(),
        (row((
            figure(
                crate::res::str::text_preset_moon(),
                crate::res::str::text_preset_moon_value(),
                crate::res::str::text_preset_km(),
                day::id!(text_preset_moon),
            ),
            figure(
                crate::res::str::text_preset_sun(),
                crate::res::str::text_preset_sun_value(),
                crate::res::str::text_preset_million_km(),
                day::id!(text_preset_sun),
            ),
        ))
        .spacing(Space::L)
        .fit(RowFit::Wrap { run_spacing: 12.0 })
        .grow(Dimension::Width),),
    )
}

/// `.max_lines(n)` and `.truncation(mode)`, both reactive: one paragraph held to the chosen
/// number of lines with the ellipsis where the picker puts it, and a file path on one line.
fn line_limits(on: bool) -> impl Piece {
    let lines = Signal::new(2_u32);
    let mode = Signal::new(Truncation::Tail);
    crate::gallery::panel(
        crate::res::str::text_lines_header(),
        (
            labeled(
                crate::res::str::text_lines_count(),
                picker(
                    lines,
                    [
                        (1_u32, crate::res::str::text_lines_one()),
                        (2, crate::res::str::text_lines_two()),
                        (0, crate::res::str::text_lines_all()),
                    ],
                )
                .segmented()
                .id(day::id!(text_lines_pick)),
            ),
            labeled(
                crate::res::str::text_lines_truncate(),
                picker(
                    mode,
                    [
                        (Truncation::Head, crate::res::str::text_trunc_head()),
                        (Truncation::Middle, crate::res::str::text_trunc_middle()),
                        (Truncation::Tail, crate::res::str::text_trunc_tail()),
                    ],
                )
                .segmented()
                .id(day::id!(text_trunc_pick)),
            ),
            sel(
                on,
                label(crate::res::str::text_sample_paragraph())
                    .max_lines(lines)
                    .truncation(mode)
                    .id(day::id!(text_lines_sample))
                    .grow(Dimension::Width),
            ),
            sel(
                on,
                label(SAMPLE_PATH)
                    .font(Font::Footnote)
                    .monospace()
                    .single_line()
                    .truncation(mode)
                    .id(day::id!(text_lines_path))
                    .grow(Dimension::Width),
            ),
        ),
    )
}

/// A path long enough to truncate on a phone. A value, the same in every language.
const SAMPLE_PATH: &str = "~/Projects/Day-Showcase/resource/locales/zh-CN/text.ftl";

/// `.letter_spacing(ems)` and `.line_height(multiple)`, both reactive: two sliders re-measure
/// one paragraph as they move.
fn spacing(on: bool) -> impl Piece {
    let tracking = Signal::new(0.0);
    let leading = Signal::new(1.2);
    crate::gallery::panel(
        crate::res::str::text_spacing_header(),
        (
            labeled(
                crate::res::str::text_spacing_letters(),
                slider(tracking)
                    .range(-0.05..=0.25)
                    .id(day::id!(text_spacing_letters)),
            ),
            labeled(
                crate::res::str::text_spacing_lines(),
                slider(leading)
                    .range(0.9..=2.0)
                    .id(day::id!(text_spacing_lines)),
            ),
            sel(
                on,
                label(crate::res::str::text_sample_paragraph())
                    .letter_spacing(tracking)
                    .line_height(leading)
                    .id(day::id!(text_spacing_sample))
                    .grow(Dimension::Width),
            ),
        ),
    )
}

/// `.align(TextAlign)`: where a wrapped label puts its lines within its own width, `Justified`
/// stretching all but the last to both edges. The picker rebuilds the label, since the alignment
/// is fixed when a label is built.
fn alignment(on: bool) -> impl Piece {
    let align = Signal::new(TextAlign::Leading);
    crate::gallery::panel(
        crate::res::str::text_align_header(),
        (
            picker(
                align,
                [
                    (TextAlign::Leading, crate::res::str::text_align_leading()),
                    (TextAlign::Center, crate::res::str::textareas_align_center()),
                    (
                        TextAlign::Trailing,
                        crate::res::str::textareas_align_trailing(),
                    ),
                    (
                        TextAlign::Justified,
                        crate::res::str::text_align_justified(),
                    ),
                ],
            )
            .segmented()
            .id(day::id!(text_align_pick)),
            switch(
                move || align.get(),
                move |pick| {
                    sel(
                        on,
                        label(crate::res::str::text_sample_paragraph())
                            .align(*pick)
                            .id(day::id!(text_align_sample))
                            .grow(Dimension::Width),
                    )
                },
            ),
        ),
    )
}

// The script specimens. They show how a label shapes and orders text in several scripts at
// once, so they read the same in every language of the interface, the way a document does.
/// One sentence each in Chinese, Japanese and Korean.
const CJK_SAMPLE: &str =
    "月球是地球唯一的天然卫星。月は地球の唯一の自然衛星です。달은 지구의 유일한 자연 위성이다.";
/// A left-to-right sentence carrying an Arabic and a Hebrew word.
const RTL_IN_LTR_SAMPLE: &str = "The star Aldebaran is named from the Arabic الدبران (al-dabarān), \
                                 and the Hebrew for the Moon is ירח (yareach).";
/// A right-to-left sentence carrying Latin names and a number.
const LTR_IN_RTL_SAMPLE: &str =
    "يدور التلسكوب James Webb حول النقطة L2 على بُعد 1.5 مليون كم من الأرض.";

/// Text in several scripts and both directions: each paragraph is one label, shaped and ordered
/// by the platform's text engine.
fn scripts(on: bool) -> impl Piece {
    let specimen = |name: LocalizedText, text: &'static str, id: &'static str| {
        column((
            label(name).font(Font::Caption).secondary(),
            sel(
                on,
                label(text)
                    .id(day::id!(text_script, id))
                    .grow(Dimension::Width),
            ),
        ))
        .spacing(2.0)
        .align(HAlign::Leading)
        .grow(Dimension::Width)
    };
    crate::gallery::panel(
        crate::res::str::text_scripts_header(),
        (
            specimen(crate::res::str::text_scripts_cjk(), CJK_SAMPLE, "cjk"),
            specimen(
                crate::res::str::text_scripts_rtl_in_ltr(),
                RTL_IN_LTR_SAMPLE,
                "rtl-in-ltr",
            ),
            specimen(
                crate::res::str::text_scripts_ltr_in_rtl(),
                LTR_IN_RTL_SAMPLE,
                "ltr-in-rtl",
            ),
            direction(on),
        ),
    )
}

/// `.layout_direction(..)` on a subtree: three steps that read in the direction the picker
/// names, whatever the direction of the app around them.
fn direction(on: bool) -> impl Piece {
    let dir = Signal::new(LayoutDirection::Ltr);
    let step = |text: LocalizedText, id: &'static str| {
        sel(
            on,
            label(text)
                .font(Font::Footnote)
                .single_line()
                .id(day::id!(text_direction_step, id))
                .padding(Insets::symmetric(10.0, 4.0))
                .background(gallery::INSET)
                .corner_radius(Corner::CAPSULE),
        )
    };
    column((
        labeled(
            crate::res::str::text_direction(),
            picker(
                dir,
                [
                    (LayoutDirection::Ltr, crate::res::str::text_direction_ltr()),
                    (LayoutDirection::Rtl, crate::res::str::text_direction_rtl()),
                ],
            )
            .segmented()
            .id(day::id!(text_direction_pick)),
        ),
        row((
            step(crate::res::str::text_direction_first(), "first"),
            step(crate::res::str::text_direction_second(), "second"),
            step(crate::res::str::text_direction_third(), "third"),
        ))
        .spacing(Space::S)
        .layout_direction(dir)
        .id(day::id!(text_direction_row)),
    ))
    .spacing(Space::S)
    .align(HAlign::Leading)
    .grow(Dimension::Width)
}

/// How many characters the streamed reply grows by at a time.
const STREAM_CHUNK: usize = 3;

/// How long the streamed reply waits between two arrivals, in milliseconds.
const STREAM_STEP_MS: u32 = 25;

/// The Markdown view (docs/markdown-view.md): a reply that arrives a few characters at a time, the way a
/// language model's does, with a heading, a list and a fenced code block whose header carries a
/// Copy button. Each arrival re-parses only the block being written.
fn markdown_view() -> impl Piece {
    let reply = Signal::new(String::new());
    let streaming = Signal::new(false);
    let copied = Signal::new(false);
    // One stream per time `streaming` goes on, stopped if the page goes away first.
    Effect::new(move || {
        if !streaming.get() {
            return;
        }
        let text = day::reactive::untrack(|| crate::res::str::text_md_reply().format());
        let task = day::task(async move {
            // A few characters at a time, which is how a model's tokens arrive and splits every
            // script alike, Markdown markers included.
            let chars: Vec<char> = text.chars().collect();
            for chunk in chars.chunks(STREAM_CHUNK) {
                reply.update(|r| r.extend(chunk));
                day::sleep(STREAM_STEP_MS).await;
            }
            streaming.set(false);
        });
        day::reactive::on_run_retrack(move || task.abort());
    });
    let start = move || {
        if !streaming.get_untracked() {
            reply.set(String::new());
            copied.set(false);
            streaming.set(true);
        }
    };
    crate::gallery::panel(
        crate::res::str::text_md_view_header(),
        (
            row((
                button(crate::res::str::text_md_stream())
                    .bordered()
                    .icon(Symbol::Play)
                    .enabled(move || !streaming.get())
                    .action(start)
                    .id(day::id!(text_md_stream)),
                when(
                    move || copied.get(),
                    || {
                        label(crate::res::str::text_md_copied())
                            .font(Font::Footnote)
                            .secondary()
                            .id(day::id!(text_md_copied))
                    },
                ),
            ))
            .spacing(Space::M)
            .align(VAlign::Center),
            markdown(reply)
                .selectable_text()
                .code_accessory(move |code| {
                    button(crate::res::str::text_md_copy())
                        .icon(Symbol::Copy)
                        .icon_only()
                        .plain()
                        .action(move || {
                            copied.set(day_part_clipboard::set_text(&code.text()));
                        })
                        .id(day::id!(text_md_copy))
                })
                .block_ids(|i| day::id!(text_md_block, i))
                .id(day::id!(text_md_view))
                .grow(Dimension::Width),
        ),
    )
}

/// Inline markdown parsed at run time (docs/markdown.md): a text area the user edits, and a
/// `.markdown()` label under it that re-parses on every keystroke.
///
/// This is the case a compile-time macro cannot serve (the string is not a literal), and it is
/// the same path a translated string or a value off the network takes. The sample seeds a link,
/// which makes the backing on iOS a text view from the start (docs/text-runs.md); tapping it
/// reports through `.on_link()`, which here shows the target rather than opening it.
fn markdown_live(on: bool, md: Signal<String>) -> impl Piece {
    let opened = Signal::new(String::new());
    crate::gallery::panel(
        crate::res::str::text_markdown_section(),
        (
            text_area(md)
                .min_lines(3)
                .max_lines(5)
                .id(day::id!(text_md_input)),
            divider(),
            sel(
                on,
                label(move || md.get())
                    .markdown()
                    .on_link(move |url| opened.set(url.to_string()))
                    .id(day::id!(text_md_output)),
            ),
            when(
                move || !opened.get().is_empty(),
                move || {
                    label(crate::res::str::text_markdown_opened(opened))
                        .font(Font::Footnote)
                        .color(TEAL)
                        .id(day::id!(text_md_opened))
                },
            ),
        ),
    )
}

/// Baseline alignment (docs/baseline.md), with a toggle that turns it off so the difference is
/// visible rather than asserted.
///
/// The rows are built to make the two states disagree as loudly as possible: each mixes a
/// Body-size label, a control whose text is inset by its own border, and a trailing unit in
/// Caption size. Aligned, all three sit on one line. Centered (what every row did before Day
/// had a baseline concept), each text sits in the middle of its own box, so the three drift
/// apart by the difference in their heights.
///
/// These use `row(..).align(..)` rather than `labeled`, which is baseline-aligned with no way to
/// opt out: this section exists to show both states side by side in time.
fn baseline() -> impl Piece {
    let aligned = Signal::new(true);
    let qty = Signal::new("12".to_string());
    let due = Signal::new(DayDate::new(2026, 3, 9).unwrap_or_else(DayDate::today_local));
    let support = capability(Cap::BaselineAlignment);
    crate::gallery::panel(
        crate::res::str::text_baseline_section(),
        (
            crate::widgets::support_banner(support),
            row((
                label(crate::res::str::text_baseline_toggle()).font(Font::Subheadline),
                toggle(aligned).id(day::id!(text_baseline_toggle)),
            ))
            .spacing(Space::S),
            when(
                move || aligned.get(),
                move || baseline_rows(VAlign::FirstBaseline, qty, due),
            ),
            when(
                move || !aligned.get(),
                move || baseline_rows(VAlign::Center, qty, due),
            ),
        ),
    )
}

/// The three demo rows at one alignment (the `when` arms above rebuild them on flip). The ids
/// carry the alignment so a dayscript can tell the two states apart.
fn baseline_rows(align: VAlign, qty: Signal<String>, due: Signal<DayDate>) -> impl Piece {
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
                .id(day::id!(text_baseline_qty, tag)),
            label(crate::res::str::text_baseline_unit()).font(Font::Caption),
        ))
        .spacing(Space::S)
        .align(align),
        // Type sizes alone, no control: a Title-size number beside Body and Caption text.
        row((
            lead(crate::res::str::text_baseline_total()),
            label("$1,240.00")
                .font(Font::Title2)
                .id(day::id!(text_baseline_total, tag)),
            label(crate::res::str::text_baseline_currency()).font(Font::Caption),
        ))
        .spacing(Space::S)
        .align(align),
        // The tallest control on the page: a date picker's stepper makes its box far taller than
        // its text, which is where centering goes most obviously wrong.
        row((
            lead(crate::res::str::text_baseline_due()),
            date_picker(due).id(day::id!(text_baseline_due, tag)),
        ))
        .spacing(Space::S)
        .align(align),
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
}
