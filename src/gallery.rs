// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The gallery kit: what gives every page of the showcase one composed look.
//!
//! A page opens on a header (the destination's glyph on a tinted tile, its title, one line saying
//! what the page shows, and its group), then lays its demos out as cards: a glyph, a title and
//! a subtitle over the demo itself, dealt into as many columns as the window has room for. The
//! surfaces are the platform's own (`.card()`), the controls are Day's native ones, and the only
//! drawn parts are the specimens a demo exists to show, on a canvas or as shapes.
//!
//! The kit also carries what those drawings share: the tones a demo colors data with, a dotted
//! stage, hatching, dashes, and a color taken from a ramp.

use std::cell::RefCell;

use day::prelude::*;

use crate::palette;

// ---------------------------------------------------------------------------
// The page being built
// ---------------------------------------------------------------------------

/// What a page's header shows, taken from its row of the destination table.
#[derive(Clone)]
pub(crate) struct PageInfo {
    /// One line saying what the page shows.
    pub(crate) detail: fn() -> day::LocalizedText,
    /// The destination's glyph.
    pub(crate) icon: day::VectorName,
    /// The title of the group the destination belongs to.
    pub(crate) group: fn() -> day::LocalizedText,
    /// The group's color.
    pub(crate) tint: Color,
}

thread_local! {
    /// The destination whose page is being built. A page is built synchronously by one call of
    /// its function, so the table sets this around that call and the page's heading reads it,
    /// which is how every page gets its header without naming its own destination.
    static CURRENT: RefCell<Option<PageInfo>> = const { RefCell::new(None) };
}

/// Builds a page with `info` as the destination its heading describes.
pub(crate) fn with_page<R>(info: PageInfo, build: impl FnOnce() -> R) -> R {
    let before = CURRENT.replace(Some(info));
    let built = build();
    CURRENT.replace(before);
    built
}

/// The destination whose page is being built, if the page is built for one.
pub(crate) fn current_page() -> Option<PageInfo> {
    CURRENT.with_borrow(Clone::clone)
}

// ---------------------------------------------------------------------------
// Tones
// ---------------------------------------------------------------------------

/// The meaning a demo gives a color. A tone is a role, so a page never names a hue.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Tone {
    /// The demo's main series or its selected state.
    Accent,
    /// A second, neutral series.
    Info,
    /// Something to notice.
    Warn,
    /// Something wrong, or a value that fell.
    Danger,
    /// Something right, or a value that rose.
    Up,
}

impl Tone {
    /// The five tones, in the order a demo of several series colors them.
    pub(crate) const ALL: [Tone; 5] =
        [Tone::Accent, Tone::Info, Tone::Warn, Tone::Danger, Tone::Up];

    /// The tone's color, from the showcase's own ramp.
    pub(crate) fn color(self) -> Color {
        match self {
            Tone::Accent => palette::SKY,
            Tone::Info => palette::VIOLET,
            Tone::Warn => palette::AMBER,
            Tone::Danger => palette::CORAL,
            Tone::Up => palette::TEAL,
        }
    }

    /// The tone as a wash behind text or a specimen: a little stronger on a dark background,
    /// where the same alpha reads fainter.
    pub(crate) fn tint(self) -> Ink {
        let color = self.color();
        Ink::adaptive(color.with_alpha(0.14), color.with_alpha(0.22))
    }
}

/// The fill of a raised specimen: a tile, a knob, a chip.
pub(crate) const RAISED: Ink = Ink::adaptive(
    Color::rgba(1.0, 1.0, 1.0, 0.9),
    Color::rgba(1.0, 1.0, 1.0, 0.09),
);

/// The fill of a recessed stage inside a card.
pub(crate) const INSET: Ink = Ink::adaptive(
    Color::rgba(0.0, 0.0, 0.0, 0.045),
    Color::rgba(0.0, 0.0, 0.0, 0.26),
);

// ---------------------------------------------------------------------------
// Parts
// ---------------------------------------------------------------------------

/// A glyph on a tinted, rounded tile: the mark of a page or a card.
pub(crate) fn icon_tile(icon: day::VectorName, tint: Color, size: f64) -> impl Piece {
    zstack((
        rounded_rectangle(Corner::Fraction(0.3)).fill(tint.with_alpha(0.16)),
        vector(icon).tint(tint).frame(size * 0.5, size * 0.5),
    ))
    .frame(size, size)
}

/// A small capsule naming a state or a category, in a tone or neutral.
pub(crate) fn badge<M>(text: impl IntoText<M>, tone: Option<Tone>) -> impl Piece {
    capsule_label(badge_text(text, tone), tone)
}

/// A [`badge`] a script can read: `id` goes on the text itself, so an assertion on it sees the
/// words and not the capsule drawn around them.
pub(crate) fn badge_with_id<M>(
    id: ElementId,
    text: impl IntoText<M>,
    tone: Option<Tone>,
) -> impl Piece {
    capsule_label(badge_text(text, tone).id(id), tone)
}

/// A badge's text: small, medium weight, one line, in its tone.
fn badge_text<M>(text: impl IntoText<M>, tone: Option<Tone>) -> impl Piece {
    let text = label(text)
        .font(Font::Caption)
        .weight(FontWeight::Medium)
        .single_line();
    match tone {
        Some(tone) => text.color(tone.color()),
        None => text.secondary(),
    }
}

/// The capsule a badge draws around its text.
fn capsule_label(text: impl Piece, tone: Option<Tone>) -> impl Piece {
    text.padding(Insets::symmetric(10.0, 4.0))
        .background(match tone {
            Some(tone) => tone.tint(),
            None => INSET,
        })
        .corner_radius(Corner::CAPSULE)
}

/// The header every page opens on. Where the navigation already titles the page in its own bar
/// (the phones, the Windows navigation view) it is the one line of detail; elsewhere it is the
/// tile, the title over the detail, and the group.
pub(crate) fn page_header(title: day::LocalizedText, title_id: ElementId) -> AnyPiece {
    let info = current_page();
    let native_title = capability(Cap::NavHeader) == Support::Native;
    match (info, native_title) {
        (Some(info), true) => label((info.detail)())
            .font(Font::Subheadline)
            .secondary()
            .grow(Dimension::Width)
            .any(),
        (Some(info), false) => row((
            icon_tile(info.icon, info.tint, 44.0),
            column((
                label(title).font(Font::Title2).bold().id(title_id),
                label((info.detail)()).font(Font::Subheadline).secondary(),
            ))
            .spacing(2.0)
            .align(HAlign::Leading)
            .grow(Dimension::Width),
            badge((info.group)(), None),
        ))
        .spacing(Space::M)
        .align(VAlign::Center)
        .grow(Dimension::Width)
        .any(),
        (None, true) => crate::widgets::nothing().any(),
        (None, false) => label(title).font(Font::Title).id(title_id).any(),
    }
}

/// A card of a gallery page: a glyph, a title and a subtitle over a demo.
pub(crate) struct Card {
    icon: day::VectorName,
    title: day::LocalizedText,
    subtitle: day::LocalizedText,
    trailing: Option<AnyPiece>,
    body: AnyPiece,
    id: Option<ElementId>,
}

/// A card titled `title`, saying `subtitle`, around `body`.
pub(crate) fn card(
    icon: day::VectorName,
    title: day::LocalizedText,
    subtitle: day::LocalizedText,
    body: impl Piece,
) -> Card {
    Card {
        icon,
        title,
        subtitle,
        trailing: None,
        body: body.any(),
        id: None,
    }
}

impl Card {
    /// A control at the trailing end of the card's header: a button, a toggle, a badge.
    pub(crate) fn trailing(mut self, piece: impl Piece) -> Self {
        self.trailing = Some(piece.any());
        self
    }

    /// The card's element id, for scripts.
    pub(crate) fn id(mut self, id: ElementId) -> Self {
        self.id = Some(id);
        self
    }

    /// The finished card.
    pub(crate) fn build(self) -> AnyPiece {
        let tint = current_page().map_or(palette::SKY, |page| page.tint);
        let titles = column((
            label(self.title).font(Font::Headline),
            label(self.subtitle).font(Font::Footnote).secondary(),
        ))
        .spacing(1.0)
        .align(HAlign::Leading)
        .grow(Dimension::Width);
        // An absent trailing control is an absent piece: `Option<P>` mounts nothing for `None`.
        let header = row((icon_tile(self.icon, tint, 36.0), titles, self.trailing))
            .spacing(Space::M)
            .align(VAlign::Center)
            .grow(Dimension::Width);
        let card = column((header, self.body.grow(Dimension::Width)))
            .spacing(Space::M)
            .align(HAlign::Leading)
            .grow(Dimension::Width)
            .card();
        match self.id {
            Some(id) => card.id(id).any(),
            None => card.any(),
        }
    }
}

/// The narrowest a column of cards gets: a window has one column for every lane this wide (plus
/// the gap) that fits, so a phone has one and a wide window two or three.
const LANE_WIDTH: f64 = 340.0;

/// A gallery page: the header, then `cards` in at most `max_columns` columns.
pub(crate) fn card_page(
    title: day::LocalizedText,
    title_id: ElementId,
    max_columns: usize,
    cards: Vec<fn() -> AnyPiece>,
) -> AnyPiece {
    let header = page_header(title, title_id);
    let grid = deck(max_columns, cards.into_iter().map(|card| card()).collect());
    let content = column((header, grid))
        .spacing(Space::L)
        .align(HAlign::Leading)
        .max_width(max_columns.max(1) as f64 * 560.0);
    scroll(
        column((content,))
            .align(HAlign::Center)
            .grow(Dimension::Width)
            .padding(20.0),
    )
    .any()
}

/// A card with a title alone over its content: a group of controls that needs a name and no
/// explanation, as a settings group has. The title sits in the page's tint.
pub(crate) fn group(title: day::LocalizedText, body: impl Piece) -> AnyPiece {
    column((
        label(title).font(Font::Headline),
        body.grow(Dimension::Width),
    ))
    .spacing(Space::M)
    .align(HAlign::Leading)
    .grow(Dimension::Width)
    .card()
    .any()
}

/// Cards in as many columns as the window has room for, up to `max_columns`, dealt in turn. The
/// columns are decided during layout, so a window dragged across a breakpoint moves the same
/// cards (and whatever state is inside them) to their new columns.
pub(crate) fn deck(max_columns: usize, cards: Vec<AnyPiece>) -> AnyPiece {
    lanes(cards)
        .lane_width(LANE_WIDTH)
        .max_lanes(max_columns.max(1))
        .spacing(Space::L)
        .grow(Dimension::Width)
        .any()
}

/// Rows of a card whose [`labeled`] ones share a label column of their own, so the labels of
/// one card line up with each other and the controls start on one edge. A card is its own
/// form: the column is as wide as that card's longest label, not the page's.
pub(crate) fn fields<C: PieceSeq + 'static>(rows: C) -> AnyPiece {
    form((column(rows)
        .spacing(Space::M)
        .align(HAlign::Leading)
        .grow(Dimension::Width),))
    .grow(Dimension::Width)
    .any()
}

/// A [`group`] of [`fields`]: a titled card of form rows, which is what a section of a settings
/// form becomes on a gallery page.
pub(crate) fn panel<C: PieceSeq + 'static>(title: day::LocalizedText, rows: C) -> AnyPiece {
    group(title, fields(rows))
}

/// Buttons that belong together: one row, wrapping onto a second where the card is narrow.
pub(crate) fn actions<C: PieceSeq + 'static>(buttons: C) -> AnyPiece {
    row(buttons)
        .spacing(Space::S)
        .align(VAlign::Center)
        .fit(RowFit::Wrap { run_spacing: 8.0 })
        .any()
}

/// A sentence of explanation inside a card, in the secondary footnote style.
pub(crate) fn note<M>(text: impl IntoText<M>) -> impl Piece {
    label(text)
        .font(Font::Footnote)
        .secondary()
        .grow(Dimension::Width)
}

/// [`fields`] on a card with no title, for a group that the page's own header already names.
pub(crate) fn plain<C: PieceSeq + 'static>(rows: C) -> AnyPiece {
    fields(rows).card().any()
}

/// [`deck`] for cards of unequal height: each card goes to the column that is shortest so far,
/// so the columns end as near each other as the cards allow. A phone's single column keeps the
/// cards in the order they were given.
pub(crate) fn balanced(max_columns: usize, cards: Vec<AnyPiece>) -> AnyPiece {
    lanes(cards)
        .lane_width(LANE_WIDTH)
        .max_lanes(max_columns.max(1))
        .spacing(Space::L)
        .balanced()
        .grow(Dimension::Width)
        .any()
}

/// A figure with its name above it: one cell of a row of readings.
pub(crate) fn stat<M>(
    name: day::LocalizedText,
    value: impl IntoText<M>,
    tone: Option<Tone>,
) -> impl Piece {
    let value = label(value).font(Font::Callout).tabular().single_line();
    let value = match tone {
        Some(tone) => value.color(tone.color()),
        None => value,
    };
    column((label(name).font(Font::Caption).secondary(), value))
        .spacing(2.0)
        .align(HAlign::Leading)
        .grow(Dimension::Width)
}

/// A line under a demo: what it shows on the leading side, a value or a name on the trailing.
pub(crate) fn caption<M1, M2>(text: impl IntoText<M1>, note: impl IntoText<M2>) -> impl Piece {
    row((
        label(text)
            .font(Font::Footnote)
            .secondary()
            .grow(Dimension::Width),
        label(note)
            .font(Font::Caption)
            .secondary()
            .tabular()
            .single_line(),
    ))
    .spacing(Space::S)
    .align(VAlign::Center)
    .grow(Dimension::Width)
}

/// A swatch and a name: one entry of a chart's key.
pub(crate) fn legend<M>(color: impl Into<Ink>, name: impl IntoText<M>) -> impl Piece {
    row((
        rounded_rectangle(2.5).fill(color.into()).frame(9.0, 9.0),
        label(name).font(Font::Caption).secondary().single_line(),
    ))
    .spacing(6.0)
    .align(VAlign::Center)
}

// ---------------------------------------------------------------------------
// Drawing
// ---------------------------------------------------------------------------

/// Fills `size` with the recessed stage a demo draws on: a rounded field with a grid of dots.
pub(crate) fn dotted_stage(draw: &mut Draw, size: Size, pitch: f64) {
    let bounds = Rect::new(0.0, 0.0, size.width, size.height);
    draw.fill(Shape::RoundedRect(bounds, 12.0), INSET);
    let dot = SystemColor::Text.with_alpha(0.12);
    let mut y = pitch / 2.0;
    while y < size.height {
        let mut x = pitch / 2.0;
        while x < size.width {
            draw.fill(Shape::Rect(Rect::new(x - 0.75, y - 0.75, 1.5, 1.5)), dot);
            x += pitch;
        }
        y += pitch;
    }
}

/// Hatches `area` with 45° lines `gap` apart. Clip to the shape being hatched first.
pub(crate) fn hatch(draw: &mut Draw, area: Rect, gap: f64, color: impl Into<Ink> + Copy) {
    let mut offset = -area.size.height;
    while offset < area.size.width {
        draw.stroke(
            Shape::Line(
                Point::new(area.origin.x + offset, area.origin.y + area.size.height),
                Point::new(area.origin.x + offset + area.size.height, area.origin.y),
            ),
            color,
            1.0,
        );
        offset += gap;
    }
}

/// A dashed hairline from `from` to `to`.
pub(crate) fn dashed(draw: &mut Draw, from: Point, to: Point, color: impl Into<Ink>) {
    draw.stroke_styled(
        Shape::Line(from, to),
        color.into(),
        StrokeStyle::dashed(1.0, vec![3.0, 4.0]),
    );
}

// ---------------------------------------------------------------------------
// Pictures
// ---------------------------------------------------------------------------

/// The gallery's photographs: the Sun, the planets and deep sky, from NASA's Image and Video
/// Library (images.nasa.gov). NASA's imagery is in the public domain, and the library serves
/// each picture in several sizes, which is what the image demos load. Fixture mode answers those
/// addresses with the copies under `resource/assets/nasa/` ([`sky::install_fixtures`]).
pub(crate) mod sky {
    /// The Sun in extreme ultraviolet, from the Solar Dynamics Observatory.
    pub(crate) const SUN: &str = "PIA26681";
    /// Earth, the 2012 Blue Marble.
    pub(crate) const EARTH: &str = "GSFC_20171208_Archive_e001386";
    /// The far side of the Moon, from the Lunar Reconnaissance Orbiter.
    pub(crate) const MOON: &str = "GSFC_20171208_Archive_e001939";
    /// Mars, the Valles Marineris hemisphere.
    pub(crate) const MARS: &str = "PIA00003";
    /// Saturn and its rings, from Cassini.
    pub(crate) const SATURN: &str = "PIA06193";
    /// Neptune, from Voyager 2.
    pub(crate) const NEPTUNE: &str = "PIA00046";
    /// The Cosmic Cliffs of the Carina Nebula, from the Webb telescope.
    pub(crate) const CARINA: &str = "carina_nebula";
    /// The Andromeda galaxy in ultraviolet.
    pub(crate) const ANDROMEDA: &str = "PIA04921";
    /// The Pillars of Creation in the Eagle Nebula, from Hubble.
    pub(crate) const PILLARS: &str = "GSFC_20171208_Archive_e000842";
    /// The Whirlpool galaxy.
    pub(crate) const WHIRLPOOL: &str = "PIA04230";
    /// Jupiter, from Juno.
    pub(crate) const JUPITER: &str = "PIA22946";
    /// The Crab Nebula, from Hubble.
    pub(crate) const CRAB: &str = "PIA03606";

    /// A size the library serves a picture in.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub(crate) enum Size {
        /// About 160 pixels on the long side.
        Thumb,
        /// About 640 pixels.
        Small,
        /// About 1,280 pixels.
        Medium,
        /// About 1,920 pixels.
        Large,
        /// The picture as it was published, which is the largest the library serves.
        Original,
    }

    /// Where the library serves its pictures.
    const HOST: &str = "https://images-assets.nasa.gov/image/";

    /// The address of picture `id` at `size`.
    pub(crate) fn url(id: &str, size: Size) -> String {
        let size = match size {
            Size::Thumb => "thumb",
            Size::Small => "small",
            Size::Medium => "medium",
            Size::Large => "large",
            Size::Original => "orig",
        };
        format!("{HOST}{id}/{id}~{size}.jpg")
    }

    /// The environment variable that turns fixture mode on: `day launch --env
    /// SHOWCASE_FIXTURES=1`. The pictures then come from the copies under
    /// `resource/assets/nasa/` and the Media page plays a bundled clip, so a scripted run shows
    /// and asserts the same pixels with or without a network.
    pub(crate) const FIXTURES_ENV: &str = "SHOWCASE_FIXTURES";

    /// Whether fixture mode is on: [`FIXTURES_ENV`] is set to anything but empty, `0`,
    /// `false`, `no` or `off`, the values a dayscript's `env:` gate reads as unset too.
    pub(crate) fn fixtures() -> bool {
        day::env(FIXTURES_ENV).is_some_and(|value| {
            !matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "" | "0" | "false" | "no" | "off"
            )
        })
    }

    /// The picture id in a library address, `PIA00003` in `…/image/PIA00003/PIA00003~small.jpg`.
    fn picture_id(url: &str) -> Option<&str> {
        url.strip_prefix(HOST)?
            .split('/')
            .next()
            .filter(|id| !id.is_empty())
    }

    /// In fixture mode, answers every request for a library picture with its bundled copy, at
    /// whatever size was asked for, and a picture with no copy with 404. Every other address
    /// still goes out.
    pub(crate) fn install_fixtures() {
        if !fixtures() {
            return;
        }
        let dir = crate::res::assets::nasa::pictures.as_str().to_string();
        crate::http_simulation().handle(
            |method, url| method == "GET" && url.starts_with(HOST),
            move |request| {
                let copy = picture_id(&request.url).and_then(|id| {
                    day::resource(day::AssetName::dynamic(format!("{dir}/{id}.jpg")))
                });
                Ok(match copy {
                    Some(copy) => {
                        let bytes = copy.to_vec();
                        let length = bytes.len().to_string();
                        day_part_http::simulation::Reply::new(200, bytes)
                            .header("Content-Type", "image/jpeg")
                            .header("Content-Length", length)
                    }
                    None => day_part_http::simulation::Reply::new(404, Vec::new()),
                })
            },
        );
    }
}
