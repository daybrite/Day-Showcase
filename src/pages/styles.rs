// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The Styles page: what a native view wears.
//!
//! A `Surface` puts a solid or gradient fill, one radius on the corners it names, a border and
//! a shadow on one view, and the page opens on a card that composes one from native controls.
//! The cards after it take each part in turn, then clipping, stacking order, opacity, the 2D
//! transform, tabular digits and layout direction. A specimen is a real view wherever a view
//! can be one. The few that only a canvas draws (a radius per corner, dashes, a shear) are
//! marked "Canvas".
//!
//! Every color is an ink (a light/dark pair, a role or a tone), so no specimen is rebuilt when
//! the appearance changes: the toolkit recolors the views it already has.

use day::motion::{Animated, Animation};
use day::prelude::*;

use crate::gallery::{self, Tone, card, card_page};
use crate::res::str as text;
use crate::res::vectors;

// ---------------------------------------------------------------------------
// Stages and tiles
// ---------------------------------------------------------------------------

/// The height of a tile's stage.
const STAGE_HEIGHT: f64 = 92.0;
/// The size of the box most specimens are.
const BOX: (f64, f64) = (100.0, 58.0);
/// The side of a specimen that shares its stage with two or three others.
const SMALL: f64 = 30.0;

/// The color of a line or a word on a surface: the window's text color at `alpha`.
const fn line(alpha: f64) -> Ink {
    SystemColor::Text.with_alpha(alpha)
}

/// The floor of a stage: a recessed, rounded view with a hairline border. It is a layer
/// under the specimen and not a container around it, since a rounded surface clips its content
/// and a specimen's shadow must not be cut.
fn floor() -> impl Piece {
    label("").grow(Dimension::Both).surface(
        Surface::new()
            .fill(gallery::INSET)
            .corner_radius(Radius::M)
            .border(SystemColor::Separator, 1.0),
    )
}

/// A stage `height` points high: the floor with `specimen` centered over it.
fn stage_box(height: f64, specimen: impl Piece) -> impl Piece {
    zstack((floor(), specimen))
        .height(height)
        .grow(Dimension::Width)
}

/// A stage whose specimen is a real view.
fn stage<P: Piece>(specimen: impl FnOnce() -> P) -> AnyPiece {
    stage_box(STAGE_HEIGHT, specimen()).any()
}

/// A stage whose specimen is drawn: a canvas over the floor.
fn drawn_stage(draw: impl Fn(&mut Draw, Size) + 'static) -> AnyPiece {
    stage(move || canvas(draw).grow(Dimension::Both))
}

/// A stage over its name. A drawn specimen says so beside the name.
fn tile<M>(stage: AnyPiece, name: impl IntoText<M>, drawn: bool) -> AnyPiece {
    let name = label(name)
        .font(Font::Footnote)
        .weight(FontWeight::Medium)
        .single_line()
        .grow(Dimension::Width);
    let mark = drawn.then(|| {
        label(text::styles_canvas())
            .font(Font::Caption)
            .secondary()
            .single_line()
    });
    column((
        stage,
        row((name, mark))
            .spacing(Space::S)
            .align(VAlign::Center)
            .padding(Insets::symmetric(2.0, 0.0))
            .grow(Dimension::Width),
    ))
    .spacing(6.0)
    .align(HAlign::Leading)
    .min_width(132.0)
    .grow(Dimension::Width)
    .any()
}

/// Tiles laid two to a row where the card has room for two, one where it does not.
fn tiles(tiles: Vec<AnyPiece>) -> AnyPiece {
    row(tiles)
        .spacing(12.0)
        .fit(RowFit::WrapColumns { run_spacing: 14.0 })
        .grow(Dimension::Width)
        .any()
}

/// A `width` by `height` rectangle in the middle of `size`.
fn centered(size: Size, width: f64, height: f64) -> Rect {
    Rect::new(
        (size.width - width) / 2.0,
        (size.height - height) / 2.0,
        width,
        height,
    )
}

/// `tone` as an opaque wash: its color mixed into a light window's and into a dark one's, so a
/// shadow under it stays hidden.
fn solid_tint(tone: Tone) -> Ink {
    let color = tone.color();
    let mix = |base: f64, share: f64| {
        Color::rgb(
            base + (color.r - base) * share,
            base + (color.g - base) * share,
            base + (color.b - base) * share,
        )
    };
    Ink::adaptive(mix(1.0, 0.18), mix(0.13, 0.28))
}

/// An opaque neutral fill for a box that casts a shadow.
const NEUTRAL: Ink = Ink::adaptive(Color::rgb(0.99, 0.99, 0.98), Color::rgb(0.2, 0.21, 0.23));

/// The color of a neutral shadow: light where the window is light, deep where it is dark.
const fn shade(light: f64, dark: f64) -> Ink {
    Ink::adaptive(
        Color::rgba(0.0, 0.0, 0.0, light),
        Color::rgba(0.0, 0.0, 0.0, dark),
    )
}

/// A plain colored view of this size and radius.
fn swatch<M>(
    width: f64,
    height: f64,
    radius: impl Into<Corner>,
    color: impl IntoReactive<Paint, M>,
) -> impl Piece {
    label("")
        .frame(width, height)
        .background(color)
        .corner_radius(radius.into())
}

/// A view of this size wearing `surface`, with `word` centered in it in `ink`. An empty word
/// leaves the surface bare. The surface is a constant or a closure: a closure patches the view
/// in place whenever what it reads changes.
fn surface_box<Ms, Mw>(
    (width, height): (f64, f64),
    surface: impl IntoReactive<Surface, Ms>,
    word: impl IntoText<Mw>,
    ink: impl Into<Ink>,
) -> impl Piece {
    zstack((label(word)
        .font(Font::Footnote)
        .weight(FontWeight::Semibold)
        .tabular()
        .color(ink.into())
        .single_line(),))
    .frame(width, height)
    .surface(surface)
}

/// Two tiles side by side, sharing the card's width.
fn pair(first: AnyPiece, second: AnyPiece) -> impl Piece {
    row((first, second))
        .spacing(12.0)
        .align(VAlign::Top)
        .grow(Dimension::Width)
}

/// A slider with its value beside it.
fn with_readout(slider: impl Piece, value: impl Fn() -> String + 'static) -> impl Piece {
    row((
        slider.grow(Dimension::Width),
        label(value)
            .font(Font::Footnote)
            .secondary()
            .tabular()
            .single_line()
            .align(TextAlign::Trailing)
            .width(48.0),
    ))
    .spacing(Space::S)
    .align(VAlign::Center)
    .grow(Dimension::Width)
}

/// A row of small specimens sharing one stage.
fn lineup(specimens: Vec<AnyPiece>) -> impl Piece {
    row(specimens).spacing(10.0).align(VAlign::Center)
}

// ---------------------------------------------------------------------------
// Surface
// ---------------------------------------------------------------------------

/// The size of the box the Surface card composes.
const RECIPE_BOX: (f64, f64) = (176.0, 84.0);

/// What the Surface card's controls ask for.
#[derive(Clone, Copy, PartialEq)]
struct Recipe {
    /// Which of [`recipe_fill`]'s fills: solid, linear, radial or conic.
    fill: usize,
    /// Which of [`corner_set`]'s sets the radius rounds.
    corners: usize,
    /// The radius, in points.
    radius: f64,
    /// Whether the surface has a border.
    border: bool,
    /// Whether the surface casts a shadow.
    shadow: bool,
}

/// The fills the Surface card's picker chooses between: a solid tint, then a linear, a radial
/// and a conic gradient between the same tones.
fn recipe_fill(index: usize) -> Paint {
    let (from, to) = (Tone::Info.color(), Tone::Accent.color());
    match index {
        1 => LinearGradient::new(
            UnitPoint::TOP_LEADING,
            UnitPoint::BOTTOM_TRAILING,
            [(0.0, from), (1.0, to)],
        )
        .into(),
        2 => RadialGradient::new(UnitPoint::new(0.3, 0.25), 0.9, [(0.0, to), (1.0, from)]).into(),
        3 => {
            ConicGradient::centered(Angle::deg(-90.0), [(0.0, from), (0.5, to), (1.0, from)]).into()
        }
        _ => solid_tint(Tone::Accent).into(),
    }
}

impl Recipe {
    /// The surface this recipe describes.
    fn surface(self) -> Surface {
        let surface = Surface::new()
            .fill(recipe_fill(self.fill))
            .corner_radius(self.radius)
            .corners(corner_set(self.corners));
        let surface = if self.border {
            surface.border(self.ink().with_alpha(0.7), 2.0)
        } else {
            surface
        };
        if self.shadow {
            surface.shadow(Shadow::new(shade(0.3, 0.7), 14.0).offset(0.0, 6.0))
        } else {
            surface
        }
    }

    /// The color of what lies on the fill: the border and the readout.
    fn ink(self) -> Ink {
        if self.fill == 0 {
            Tone::Accent.color().into()
        } else {
            Color::WHITE.into()
        }
    }
}

/// The corners the Surface card's picker chooses between: all, the top pair, the bottom pair,
/// and a diagonal.
fn corner_set(index: usize) -> Corners {
    match index {
        1 => Corners::TOP,
        2 => Corners::BOTTOM,
        3 => Corners::TOP_RIGHT | Corners::BOTTOM_LEFT,
        _ => Corners::ALL,
    }
}

/// Surface: one view whose fill, corners, radius, border and shadow follow five controls. The
/// surface is a closure of them, so each change patches the one view in place.
fn surface_card() -> AnyPiece {
    let fill = Signal::new(1_usize);
    let corners = Signal::new(0_usize);
    let radius = Signal::new(22.0_f64);
    let border = Signal::new(true);
    let shadow = Signal::new(true);
    let recipe = move || Recipe {
        fill: fill.get(),
        corners: corners.get(),
        radius: radius.get().round(),
        border: border.get(),
        shadow: shadow.get(),
    };

    let specimen = stage_box(
        148.0,
        zstack((
            label(move || text::styles_points(radius.get().round()).format())
                .font(Font::Footnote)
                .weight(FontWeight::Semibold)
                .tabular()
                .color(move || recipe().ink())
                .single_line(),
        ))
        .frame(RECIPE_BOX.0, RECIPE_BOX.1)
        .surface(move || recipe().surface())
        .id(day::id!(styles_surface_box)),
    );

    card(
        vectors::nav_styles,
        text::styles_surface_title(),
        text::styles_surface_subtitle(),
        column((
            specimen,
            gallery::fields((
                labeled(
                    text::styles_surface_fill(),
                    picker(
                        fill,
                        [
                            (0_usize, text::styles_surface_solid()),
                            (1, text::styles_surface_linear()),
                            (2, text::styles_surface_radial()),
                            (3, text::styles_surface_conic()),
                        ],
                    )
                    .segmented()
                    .id(day::id!(styles_surface_fill)),
                ),
                labeled(
                    text::styles_surface_corners(),
                    picker(
                        corners,
                        [
                            (0_usize, text::styles_surface_all()),
                            (1, text::styles_corners_top()),
                            (2, text::styles_surface_bottom()),
                            (3, text::styles_corners_diagonal()),
                        ],
                    )
                    .segmented()
                    .id(day::id!(styles_surface_corners)),
                ),
                labeled(
                    text::styles_surface_radius(),
                    slider(radius)
                        .range(0.0..=40.0)
                        .step(1.0)
                        .grow(Dimension::Width)
                        .id(day::id!(styles_surface_radius)),
                ),
                labeled(
                    text::styles_surface_border(),
                    toggle(border).id(day::id!(styles_surface_border)),
                ),
                labeled(
                    text::styles_surface_shadow(),
                    toggle(shadow).id(day::id!(styles_surface_shadow)),
                ),
            )),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .id(day::id!(styles_surface))
    .build()
}

// ---------------------------------------------------------------------------
// Corners
// ---------------------------------------------------------------------------

/// A tinted, outlined box whose radius rounds `corners`, on its stage.
fn corner_stage(tone: Tone, corners: fn() -> Corners) -> AnyPiece {
    stage(move || {
        surface_box(
            BOX,
            Surface::new()
                .fill(tone.tint())
                .corner_radius(Radius::XL)
                .corners(corners())
                .border(tone.color(), 1.5),
            "",
            tone.color(),
        )
    })
}

/// The radius tokens, smallest first, with the names the code gives them.
const RADII: [(Radius, &str); 5] = [
    (Radius::XS, "XS"),
    (Radius::S, "S"),
    (Radius::M, "M"),
    (Radius::L, "L"),
    (Radius::XL, "XL"),
];

/// Corners: the radius tokens on plain views, the corners a surface can name, a capsule that
/// rounds its ends whatever its length, and a radius per corner, which is a canvas shape.
fn corners_card() -> AnyPiece {
    let tokens: Vec<AnyPiece> = RADII
        .into_iter()
        .map(|(radius, name)| {
            zstack((label(name)
                .font(Font::Caption)
                .weight(FontWeight::Semibold)
                .color(Tone::Accent.color())
                .single_line(),))
            .frame(48.0, 40.0)
            .background(Tone::Accent.tint())
            .corner_radius(radius)
            .any()
        })
        .collect();
    let tokens = column((
        row(tokens)
            .spacing(8.0)
            .fit(RowFit::Wrap { run_spacing: 8.0 })
            .grow(Dimension::Width)
            .id(day::id!(styles_radius_tokens)),
        label(text::styles_corners_radius())
            .font(Font::Footnote)
            .weight(FontWeight::Medium)
            .single_line(),
    ))
    .spacing(6.0)
    .align(HAlign::Leading)
    .grow(Dimension::Width);
    let capsule = stage(|| {
        surface_box(
            (BOX.0, 36.0),
            Surface::new()
                .fill(Tone::Up.tint())
                .corner_radius(Corner::CAPSULE)
                .border(Tone::Up.color(), 1.5),
            "",
            Tone::Up.color(),
        )
        .id(day::id!(styles_capsule))
    });
    let four = drawn_stage(|draw, size| {
        let shape = Shape::rounded_rect(
            centered(size, BOX.0, BOX.1),
            CornerRadii {
                top_left: 6.0,
                top_right: 26.0,
                bottom_right: 12.0,
                bottom_left: 34.0,
            },
        );
        draw.fill(shape.clone(), Tone::Danger.tint());
        draw.stroke(shape, Tone::Danger.color(), 1.5);
    });

    card(
        vectors::grid_sizing,
        text::styles_corners_title(),
        text::styles_corners_subtitle(),
        column((
            tokens,
            tiles(vec![
                tile(
                    corner_stage(Tone::Accent, || Corners::TOP),
                    text::styles_corners_top(),
                    false,
                ),
                tile(
                    corner_stage(Tone::Info, || Corners::RIGHT),
                    text::styles_corners_right(),
                    false,
                ),
                tile(
                    corner_stage(Tone::Up, || Corners::TOP_RIGHT | Corners::BOTTOM_LEFT),
                    text::styles_corners_diagonal(),
                    false,
                ),
                tile(
                    corner_stage(Tone::Warn, || Corners::BOTTOM_RIGHT),
                    text::styles_corners_one(),
                    false,
                ),
                tile(capsule, text::styles_corners_capsule(), false),
                tile(four, text::styles_corners_four(), true),
            ]),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .id(day::id!(styles_corners))
    .build()
}

// ---------------------------------------------------------------------------
// Borders
// ---------------------------------------------------------------------------

/// Borders: three weights, a line that follows a surface's corners, the square line the
/// `.border` modifier puts round any piece, and the dashes and dots of a canvas stroke.
fn borders_card() -> AnyPiece {
    let weights = stage(|| {
        lineup(
            [1.0, 2.0, 4.0]
                .into_iter()
                .map(|width| {
                    surface_box(
                        (SMALL + 6.0, SMALL + 6.0),
                        Surface::new()
                            .fill(gallery::RAISED)
                            .corner_radius(Radius::S)
                            .border(line(0.8), width),
                        format!("{width:.0}"),
                        line(0.8),
                    )
                    .any()
                })
                .collect(),
        )
    });
    let rounded = stage(|| {
        surface_box(
            BOX,
            Surface::new()
                .fill(Tone::Info.tint())
                .corner_radius(20.0)
                .border(Tone::Info.color(), 2.0),
            "",
            Tone::Info.color(),
        )
    });
    // The modifier with no surface: a square line round whatever it is given.
    let piece = stage(|| {
        label(text::styles_borders_sample())
            .font(Font::Headline)
            .color(Tone::Up.color())
            .single_line()
            .padding(Insets::symmetric(16.0, 10.0))
            .border(Tone::Up.color(), 1.5)
    });
    let ring = stage(|| {
        surface_box(
            (BOX.1, BOX.1),
            Surface::new()
                .corner_radius(Corner::CAPSULE)
                .border(Tone::Warn.color(), 5.0),
            "",
            Tone::Warn.color(),
        )
    });
    let overlay = stage(|| {
        surface_box(
            BOX,
            Surface::new()
                .fill(LinearGradient::horizontal(
                    Tone::Accent.color(),
                    Tone::Info.color(),
                ))
                .corner_radius(14.0)
                .border(Color::WHITE.with_alpha(0.55), 3.0),
            "",
            Color::WHITE,
        )
    });
    // Two orbits round a planet: one dashed, one of dots, which are dashes too short to see
    // with a round cap on each end.
    let orbits = drawn_stage(|draw, size| {
        let middle = Point::new(size.width / 2.0, size.height / 2.0);
        let orbit = |rx: f64, ry: f64| {
            Shape::Ellipse(Rect::new(middle.x - rx, middle.y - ry, rx * 2.0, ry * 2.0))
        };
        draw.fill(orbit(7.0, 7.0), Tone::Warn.color());
        draw.stroke_styled(
            orbit(30.0, 18.0),
            Tone::Accent.color(),
            StrokeStyle::dashed(1.5, vec![7.0, 4.0]),
        );
        draw.stroke_styled(
            orbit(54.0, 32.0),
            Tone::Danger.color(),
            StrokeStyle {
                dash: vec![0.1, 6.5],
                ..StrokeStyle::round(2.5)
            },
        );
    });

    card(
        vectors::nav_grid,
        text::styles_borders_title(),
        text::styles_borders_subtitle(),
        tiles(vec![
            tile(weights, text::styles_borders_weights(), false),
            tile(rounded, text::styles_borders_rounded(), false),
            tile(piece, text::styles_borders_piece(), false),
            tile(ring, text::styles_borders_ring(), false),
            tile(overlay, text::styles_borders_overlay(), false),
            tile(orbits, text::styles_borders_dashes(), true),
        ]),
    )
    .id(day::id!(styles_borders))
    .build()
}

// ---------------------------------------------------------------------------
// Gradients
// ---------------------------------------------------------------------------

/// A bare view filled with `paint`, on its stage.
fn gradient_stage(paint: impl Into<Paint>) -> AnyPiece {
    let paint = paint.into();
    stage(move || {
        surface_box(
            BOX,
            Surface::new().fill(paint).corner_radius(Radius::M),
            "",
            Color::WHITE,
        )
    })
}

/// Gradients: linear fills on a view along either axis, a diagonal and out to clear, then a
/// radial fill lit from one side and a conic wheel of the five tones, all on views.
fn gradients_card() -> AnyPiece {
    let vertical = gradient_stage(LinearGradient::vertical(
        Tone::Info.color(),
        Tone::Accent.color(),
    ));
    let horizontal = gradient_stage(LinearGradient::horizontal(
        Tone::Warn.color(),
        Tone::Danger.color(),
    ));
    let diagonal = gradient_stage(LinearGradient::new(
        UnitPoint::BOTTOM_LEADING,
        UnitPoint::TOP_TRAILING,
        [(0.0, Tone::Accent.color()), (1.0, Tone::Up.color())],
    ));
    let clear = {
        let color = Tone::Danger.color();
        gradient_stage(LinearGradient::vertical(color, color.with_alpha(0.0)))
    };
    // A sun lit from its upper leading side, darkening to its rim.
    let warn = Tone::Warn.color();
    let radial = gradient_stage(RadialGradient::new(
        UnitPoint::new(0.3, 0.3),
        0.9,
        [
            (0.0, Color::WHITE),
            (0.3, warn),
            (1.0, Color::rgb(warn.r * 0.5, warn.g * 0.32, warn.b * 0.24)),
        ],
    ));
    // The five tones round a wheel that starts at the top, closed on its first color.
    let last = Tone::ALL.len() as f64;
    let conic = gradient_stage(ConicGradient::centered(
        Angle::deg(-90.0),
        Tone::ALL
            .iter()
            .enumerate()
            .map(|(index, tone)| (index as f64 / last, tone.color()))
            .chain([(1.0, Tone::ALL[0].color())])
            .collect::<Vec<_>>(),
    ));

    card(
        vectors::nav_canvas,
        text::styles_gradients_title(),
        text::styles_gradients_subtitle(),
        tiles(vec![
            tile(vertical, text::styles_gradients_vertical(), false),
            tile(horizontal, text::styles_gradients_horizontal(), false),
            tile(diagonal, text::styles_corners_diagonal(), false),
            tile(clear, text::styles_gradients_clear(), false),
            tile(radial, text::styles_gradients_radial(), false),
            tile(conic, text::styles_gradients_conic(), false),
        ]),
    )
    .id(day::id!(styles_gradients))
    .build()
}

// ---------------------------------------------------------------------------
// Shadows
// ---------------------------------------------------------------------------

/// Shadows: three blurs, a glow, a shadow moved to one side and one under a photograph, each
/// the shadow of a real view's rounded rectangle. Lift raises the modifier specimen: its
/// shadow, corners and border are closures of the switch, patched in place.
fn shadows_card() -> AnyPiece {
    let lifted = Signal::new(false);
    let blurs = stage(|| {
        lineup(
            [2.0, 8.0, 20.0]
                .into_iter()
                .map(|blur: f64| {
                    surface_box(
                        (SMALL + 2.0, SMALL + 2.0),
                        Surface::new()
                            .fill(NEUTRAL)
                            .corner_radius(9.0)
                            .shadow(Shadow::new(shade(0.26, 0.7), blur).offset(0.0, blur * 0.4)),
                        format!("{blur:.0}"),
                        line(0.7),
                    )
                    .padding(Insets::symmetric(4.0, 0.0))
                    .any()
                })
                .collect(),
        )
    });
    // A star: a round surface with a shadow of its own color and no offset.
    let glow = stage(|| {
        let warm = Tone::Warn.color();
        surface_box(
            (40.0, 40.0),
            Surface::new()
                .fill(warm)
                .corner_radius(Corner::CAPSULE)
                .shadow(Shadow::new(warm.with_alpha(0.75), 22.0)),
            "",
            Color::WHITE,
        )
    });
    let offset = stage(|| {
        let info = Tone::Info.color();
        surface_box(
            (84.0, 44.0),
            Surface::new()
                .fill(solid_tint(Tone::Info))
                .corner_radius(12.0)
                .border(info, 1.0)
                .shadow(Shadow::new(info.with_alpha(0.5), 5.0).offset(-7.0, 7.0)),
            "",
            info,
        )
    });
    // The shadow takes the shape of the corners the surface names.
    let shaped = stage(|| {
        surface_box(
            (84.0, 44.0),
            Surface::new()
                .fill(NEUTRAL)
                .corner_radius(Corner::CAPSULE)
                .corners(Corners::LEFT)
                .shadow(Shadow::new(shade(0.3, 0.7), 10.0).offset(0.0, 4.0)),
            "",
            line(0.7),
        )
    });
    // The modifiers with no surface: a piece's corners, border and shadow, each following the
    // Lift switch.
    let piece = stage(move || {
        label(text::styles_shadows_sample())
            .font(Font::Headline)
            .color(Tone::Up.color())
            .single_line()
            .padding(Insets::symmetric(16.0, 10.0))
            .background(NEUTRAL)
            .corner_radius(move || if lifted.get() { Radius::L } else { Radius::XS })
            .border(
                Tone::Up.color(),
                move || if lifted.get() { 0.0 } else { 1.5 },
            )
            .shadow(move || {
                if lifted.get() {
                    Shadow::new(Tone::Up.color().with_alpha(0.6), 18.0).offset(0.0, 8.0)
                } else {
                    Shadow::new(Tone::Up.color().with_alpha(0.0), 0.0)
                }
            })
            .id(day::id!(styles_shadows_piece))
    });
    // The surface is filled so the shadow has a shape while the photograph loads. A shadowed
    // surface does not clip, so the photograph rounds its corners to match.
    let picture = stage(|| {
        day::http_image(gallery::sky::url(
            gallery::sky::JUPITER,
            gallery::sky::Size::Small,
        ))
        .rounded(14.0)
        .frame(64.0, 64.0)
        .id(day::id!(styles_image))
        .surface(
            Surface::new()
                .fill(NEUTRAL)
                .corner_radius(14.0)
                .shadow(Shadow::new(shade(0.3, 0.7), 12.0).offset(0.0, 5.0)),
        )
    });

    card(
        vectors::nav_images,
        text::styles_shadows_title(),
        text::styles_shadows_subtitle(),
        tiles(vec![
            tile(blurs, text::styles_shadows_blur(), false),
            tile(glow, text::styles_shadows_glow(), false),
            tile(offset, text::styles_shadows_offset(), false),
            tile(shaped, text::styles_shadows_shaped(), false),
            tile(piece, text::styles_borders_piece(), false),
            tile(picture, text::styles_shadows_image(), false),
        ]),
    )
    .trailing(
        row((
            label(text::styles_shadows_lift())
                .font(Font::Footnote)
                .secondary(),
            toggle(lifted).id(day::id!(styles_shadows_lift)),
        ))
        .spacing(Space::S)
        .align(VAlign::Center),
    )
    .id(day::id!(styles_shadows))
    .build()
}

// ---------------------------------------------------------------------------
// Layers
// ---------------------------------------------------------------------------

/// The diameter of the disc that hangs out of the clip specimen's corner.
const CLIP_DISC: f64 = 56.0;
/// How far the disc's center sits from the box's, toward the bottom trailing corner.
const CLIP_REACH: (f64, f64) = (26.0, 20.0);
/// The size of a card of the stacking specimen.
const STACK_CARD: (f64, f64) = (58.0, 42.0);
/// How far each card of the stack sits from the one before it.
const STACK_STEP: (f64, f64) = (24.0, 12.0);

/// 1 where the layout runs left to right and −1 where it runs right to left: the sign of a
/// translation that moves a piece toward the trailing side.
fn toward_trailing() -> f64 {
    if is_rtl() { -1.0 } else { 1.0 }
}

/// The clip specimen: a rounded box and a disc that hangs out of its bottom trailing corner.
/// Clipped, the disc is the box's content, and a rounded surface clips its content. Unclipped,
/// the disc is a sibling over the same surface.
fn clip_specimen(clipped: bool) -> AnyPiece {
    let surface = || {
        Surface::new()
            .fill(Tone::Info.tint())
            .corner_radius(20.0)
            .border(Tone::Info.color(), 1.5)
    };
    let disc = || {
        swatch(CLIP_DISC, CLIP_DISC, Corner::CAPSULE, Tone::Warn.color())
            .translation(CLIP_REACH.0 * toward_trailing(), CLIP_REACH.1)
    };
    if clipped {
        zstack((disc(),))
            .frame(BOX.0, BOX.1)
            .surface(surface())
            .any()
    } else {
        zstack((label("").grow(Dimension::Both).surface(surface()), disc()))
            .frame(BOX.0, BOX.1)
            .any()
    }
}

/// One card of the stacking specimen. The front one wears an outline and takes `opacity`.
fn stack_card(index: usize, front: bool, opacity: Signal<f64>) -> AnyPiece {
    let tone = [Tone::Danger, Tone::Up, Tone::Accent][index % 3];
    let surface = Surface::new().fill(tone.color()).corner_radius(Radius::M);
    let number = (index + 1).to_string();
    let step = index as f64;
    let (dx, dy) = (STACK_STEP.0 * step * toward_trailing(), STACK_STEP.1 * step);
    if front {
        surface_box(
            STACK_CARD,
            surface.border(line(0.9), 2.0),
            number,
            Color::WHITE,
        )
        .opacity(opacity)
        .id(day::id!(styles_opacity_box))
        .aligned(Alignment::TopLeading)
        .translation(dx, dy)
        .any()
    } else {
        surface_box(STACK_CARD, surface, number, Color::WHITE)
            .aligned(Alignment::TopLeading)
            .translation(dx, dy)
            .any()
    }
}

/// Layers: a clip switch, three overlapping cards whose front one a picker chooses, and a
/// slider for the front card's opacity.
fn layers_card() -> AnyPiece {
    let clip = Signal::new(true);
    let front = Signal::new(2_usize);
    let opacity = Signal::new(1.0_f64);

    let clip_stage = stage_box(
        STAGE_HEIGHT,
        switch(move || clip.get(), |clip| clip_specimen(*clip)),
    )
    .id(day::id!(styles_clip_stage))
    .any();

    // A zstack paints its children in order, the last one on top, so bringing a card to the
    // front is building the stack again with that card last.
    let stack_stage = stage_box(
        STAGE_HEIGHT,
        switch(
            move || front.get(),
            move |front| {
                let front = *front;
                let cards: Vec<AnyPiece> = (1..=3)
                    .map(|after| (front + after) % 3)
                    .map(|index| stack_card(index, index == front, opacity))
                    .collect();
                zstack(cards).align(Alignment::TopLeading).frame(
                    STACK_CARD.0 + STACK_STEP.0 * 2.0,
                    STACK_CARD.1 + STACK_STEP.1 * 2.0,
                )
            },
        ),
    )
    .id(day::id!(styles_stack_stage))
    .any();

    card(
        vectors::nav_stack,
        text::styles_layers_title(),
        text::styles_layers_subtitle(),
        column((
            pair(
                tile(
                    clip_stage,
                    move || {
                        if clip.get() {
                            text::styles_layers_clipped().format()
                        } else {
                            text::styles_layers_visible().format()
                        }
                    },
                    false,
                ),
                tile(stack_stage, text::styles_layers_stack(), false),
            ),
            gallery::fields((
                labeled(
                    text::styles_layers_front(),
                    picker(front, [(0_usize, "1"), (1, "2"), (2, "3")])
                        .segmented()
                        .id(day::id!(styles_front_picker)),
                ),
                labeled(
                    text::styles_layers_opacity(),
                    with_readout(
                        slider(opacity)
                            .range(0.1..=1.0)
                            .id(day::id!(styles_opacity_slider)),
                        move || text::styles_percent(opacity.get()).format(),
                    ),
                ),
            )),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .trailing(
        row((
            label(text::styles_layers_clip())
                .font(Font::Footnote)
                .secondary(),
            toggle(clip).id(day::id!(styles_clip_toggle)),
        ))
        .spacing(Space::S)
        .align(VAlign::Center),
    )
    .id(day::id!(styles_layers))
    .build()
}

// ---------------------------------------------------------------------------
// Transforms
// ---------------------------------------------------------------------------

/// The size of the box the Transforms card turns.
const TURN_BOX: (f64, f64) = (112.0, 64.0);
/// The side of the square a moon is carried round in.
const ORBIT_SIDE: f64 = 72.0;
/// How long the moon takes to go round, in milliseconds.
const ORBIT_MS: u32 = 8000;

/// The dashed outline of where the turned box sits untransformed.
fn draw_home(draw: &mut Draw, size: Size) {
    draw.stroke_styled(
        Shape::RoundedRect(centered(size, TURN_BOX.0, TURN_BOX.1), 12.0),
        SystemColor::TertiaryText,
        StrokeStyle::dashed(1.0, vec![3.0, 4.0]),
    );
}

/// Transforms: a view turned and scaled by two sliders about its center or its corner, a moon
/// carried round a planet by a repeating `Animated<Angle>`, and a shear, which only a canvas
/// has.
fn transforms_card() -> AnyPiece {
    let turn = Signal::new(-18.0_f64);
    let scale = Signal::new(1.0_f64);
    let corner = Signal::new(false);
    let anchor = move || if corner.get() { 0.0 } else { 0.5 };

    let specimen = zstack((
        floor(),
        canvas(draw_home).grow(Dimension::Both),
        label("")
            .frame(TURN_BOX.0, TURN_BOX.1)
            .surface(
                Surface::new()
                    .fill(solid_tint(Tone::Accent))
                    .corner_radius(12.0)
                    .border(Tone::Accent.color(), 1.5),
            )
            .transform(move || Transform {
                rotation: Angle::deg(turn.get()),
                sx: scale.get(),
                sy: scale.get(),
                anchor_x: anchor(),
                anchor_y: anchor(),
                ..Transform::IDENTITY
            })
            .id(day::id!(styles_origin_box)),
        // The pivot: the point the box turns and scales about.
        swatch(8.0, 8.0, Corner::CAPSULE, SystemColor::Text).translation(
            move || (anchor() - 0.5) * TURN_BOX.0,
            move || (anchor() - 0.5) * TURN_BOX.1,
        ),
    ))
    .height(156.0)
    .grow(Dimension::Width);

    // The moon is moved to the top of a square that turns about its center, so it goes round the
    // planet in the middle. One turn, repeated until the page goes: a decoration, so it does
    // not start under reduced motion.
    let angle = Animated::new(Angle::ZERO);
    if !day::reactive::untrack(day::reduce_motion) {
        Animation::tween(
            angle,
            Angle::FULL,
            AnimSpec::linear(ORBIT_MS).repeat_forever(false),
        )
        .start();
    }
    let orbit = stage(move || {
        zstack((
            swatch(26.0, 26.0, Corner::CAPSULE, Tone::Warn.color()),
            zstack((swatch(10.0, 10.0, Corner::CAPSULE, SystemColor::Text)
                .translation(0.0, 5.0 - ORBIT_SIDE / 2.0),))
            .frame(ORBIT_SIDE, ORBIT_SIDE)
            .rotation(angle)
            .id(day::id!(styles_orbit)),
        ))
    });
    let shear = drawn_stage(|draw, size| {
        // x moves by tan(18°) of y, the one part of an affine map a view's transform lacks.
        let shear = Affine {
            c: 18.0_f64.to_radians().tan(),
            ..Affine::IDENTITY
        };
        let to_middle = Affine::translate(size.width / 2.0, size.height / 2.0);
        draw.transformed(shear.then(to_middle), |draw| {
            let shape =
                Shape::RoundedRect(Rect::new(-BOX.0 / 2.0, -BOX.1 / 2.0, BOX.0, BOX.1), 10.0);
            draw.fill(shape.clone(), Tone::Up.tint());
            draw.stroke(shape, Tone::Up.color(), 1.5);
        });
    });

    card(
        vectors::nav_physics,
        text::styles_transforms_title(),
        text::styles_transforms_subtitle(),
        column((
            specimen,
            gallery::fields((
                labeled(
                    text::styles_transforms_rotate(),
                    with_readout(
                        slider(turn)
                            .range(-180.0..=180.0)
                            .step(1.0)
                            .id(day::id!(styles_rotate_slider)),
                        move || text::styles_degrees(turn.get()).format(),
                    ),
                ),
                labeled(
                    text::styles_transforms_scale(),
                    with_readout(
                        slider(scale)
                            .range(0.5..=1.5)
                            .id(day::id!(styles_scale_slider)),
                        move || text::styles_percent(scale.get()).format(),
                    ),
                ),
                labeled(
                    text::styles_transforms_anchor(),
                    picker(
                        corner,
                        [
                            (false, text::styles_transforms_center()),
                            (true, text::styles_transforms_corner()),
                        ],
                    )
                    .segmented()
                    .id(day::id!(styles_anchor_picker)),
                ),
            )),
            pair(
                tile(orbit, text::styles_transforms_orbit(), false),
                tile(shear, text::styles_transforms_shear(), true),
            ),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .trailing(
        button(text::styles_transforms_reset())
            .bordered()
            .compact()
            .action(move || {
                with_animation(AnimSpec::spring(0.5, 0.6), || {
                    turn.set(0.0);
                    scale.set(1.0);
                });
            })
            .id(day::id!(styles_transforms_reset)),
    )
    .id(day::id!(styles_transforms))
    .build()
}

// ---------------------------------------------------------------------------
// Tabular digits
// ---------------------------------------------------------------------------

/// A planet of the page's sample data: its name and its mean distance from the Sun, in
/// millions of kilometers.
type Planet = (fn() -> day::LocalizedText, f64);

/// The planets, outward from the Sun.
const PLANETS: [Planet; 8] = [
    (text::styles_planet_mercury, 57.91),
    (text::styles_planet_venus, 108.21),
    (text::styles_planet_earth, 149.6),
    (text::styles_planet_mars, 227.94),
    (text::styles_planet_jupiter, 778.41),
    (text::styles_planet_saturn, 1426.67),
    (text::styles_planet_uranus, 2870.66),
    (text::styles_planet_neptune, 4498.4),
];

/// The width of a column of figures.
const FIGURE_WIDTH: f64 = 92.0;

/// One cell of a figure column: `text` trailing-aligned on one line, in the font's digits or
/// in tabular ones.
fn figure_cell<M>(text: impl IntoText<M>, font: Font, tabular: bool) -> AnyPiece {
    let cell = label(text).font(font).single_line();
    let cell = if tabular { cell.tabular() } else { cell };
    cell.align(TextAlign::Trailing).width(FIGURE_WIDTH).any()
}

/// Tabular digits: the same distances set with the font's proportional digits and with
/// tabular ones, which share one width so the columns of a table line up.
fn numerals_card() -> AnyPiece {
    let mut rows = vec![
        row((
            spacer(),
            figure_cell(text::styles_numerals_proportional(), Font::Caption, false),
            figure_cell(text::styles_numerals_tabular(), Font::Caption, false),
        ))
        .spacing(Space::S)
        .grow(Dimension::Width)
        .any(),
    ];
    for (name, distance) in PLANETS.iter().take(6) {
        let figure = text::styles_numerals_figure(*distance);
        rows.push(
            row((
                label(name())
                    .font(Font::Callout)
                    .secondary()
                    .single_line()
                    .grow(Dimension::Width),
                figure_cell(figure.clone(), Font::Title3, false),
                figure_cell(figure, Font::Title3, true),
            ))
            .spacing(Space::S)
            .align(VAlign::Center)
            .grow(Dimension::Width)
            .any(),
        );
    }

    card(
        vectors::nav_text,
        text::styles_numerals_title(),
        text::styles_numerals_subtitle(),
        column(rows).spacing(6.0).align(HAlign::Leading),
    )
    .id(day::id!(styles_numerals))
    .build()
}

// ---------------------------------------------------------------------------
// Direction
// ---------------------------------------------------------------------------

/// Direction: Day lays an app out in its language's direction, so the strip of planets starts
/// from the leading side, whichever side that is, and a trailing label ends on the other.
fn direction_card() -> AnyPiece {
    let chips: Vec<AnyPiece> = PLANETS
        .iter()
        .map(|(name, _)| {
            label(name())
                .font(Font::Footnote)
                .weight(FontWeight::Medium)
                .color(Tone::Info.color())
                .single_line()
                .padding(Insets::symmetric(12.0, 7.0))
                .background(Tone::Info.tint())
                .corner_radius(Corner::CAPSULE)
                .any()
        })
        .collect();

    let subtitle = if is_rtl() {
        text::styles_rtl_subtitle_rtl()
    } else {
        text::styles_rtl_subtitle_ltr()
    };
    card(
        vectors::nav_localization,
        text::styles_rtl_title(),
        subtitle,
        column((
            scroll(row(chips).spacing(6.0))
                .horizontal()
                .height(32.0)
                .grow(Dimension::Width)
                .id(day::id!(styles_rtl_scroll)),
            row((
                label(text::styles_rtl_leading())
                    .font(Font::Footnote)
                    .secondary()
                    .grow(Dimension::Width),
                label(text::styles_rtl_trailing())
                    .font(Font::Footnote)
                    .secondary()
                    .align(TextAlign::Trailing)
                    .grow(Dimension::Width),
            ))
            .spacing(Space::S)
            .grow(Dimension::Width),
            divider(),
            label(text::styles_rtl_mixed())
                .font(Font::Callout)
                .grow(Dimension::Width),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .id(day::id!(styles_rtl))
    .build()
}

/// The Styles page: nine cards, in at most two columns.
pub(crate) fn styles_page() -> AnyPiece {
    card_page(
        text::nav_styles(),
        day::id!(styles_title),
        2,
        vec![
            surface_card,
            gradients_card,
            corners_card,
            borders_card,
            layers_card,
            shadows_card,
            numerals_card,
            transforms_card,
            direction_card,
        ],
    )
}
