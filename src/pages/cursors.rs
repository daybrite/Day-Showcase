use day::prelude::*;

use crate::widgets::{page, support_note};

/// The `.cursor()` decorator (day/docs/cursor.md): every shared shape as a swatch to hover, one
/// box whose shape follows a picker, a nested pair that proves the nearest ancestor wins, and
/// the shapes only this toolkit names. `Cap::Cursor` says up front whether this host draws the
/// requested shapes, a nearest neighbor, or nothing at all — a touch screen never shows one.
pub(crate) fn cursors_page() -> AnyPiece {
    page(
        crate::res::str::nav_cursors(),
        "cursors-title",
        form((
            support_section(),
            swatches_section(
                crate::res::str::cursors_shapes_title(),
                "shapes",
                &[
                    Cursor::Default,
                    Cursor::Pointer,
                    Cursor::Text,
                    Cursor::VerticalText,
                    Cursor::Crosshair,
                    Cursor::Cell,
                    Cursor::ContextMenu,
                    Cursor::Help,
                    Cursor::None,
                ],
            ),
            swatches_section(
                crate::res::str::cursors_drag_title(),
                "drag",
                &[
                    Cursor::Move,
                    Cursor::Grab,
                    Cursor::Grabbing,
                    Cursor::Copy,
                    Cursor::Alias,
                    Cursor::NotAllowed,
                ],
            ),
            swatches_section(
                crate::res::str::cursors_resize_title(),
                "resize",
                &[
                    Cursor::NsResize,
                    Cursor::EwResize,
                    Cursor::NeswResize,
                    Cursor::NwseResize,
                    Cursor::ColResize,
                    Cursor::RowResize,
                    Cursor::ZoomIn,
                    Cursor::ZoomOut,
                ],
            ),
            swatches_section(
                crate::res::str::cursors_busy_title(),
                "busy",
                &[Cursor::Wait, Cursor::Progress],
            ),
            reactive_section(),
            nested_section(),
            native_section(),
        ))
        .any(),
    )
    .any()
}

/// What this toolkit answers for `Cap::Cursor`, as a banner and as a row a script can read.
fn support_section() -> impl Piece {
    let support = capability(Cap::Cursor);
    let text = match support {
        Support::Native => crate::res::str::cursors_native(),
        Support::Emulated => crate::res::str::cursors_emulated(),
        Support::Unsupported => crate::res::str::cursors_unsupported(),
    };
    section((
        support_note(support),
        label(crate::res::str::cursors_hint()).font(Font::Footnote),
        labeled(
            crate::res::str::cursors_support_label(),
            label(text).id("cursors-support"),
        ),
    ))
    .title(crate::res::str::cursors_support_title())
}

/// A tile per shape: its API name, with that shape over it. The name is the `Cursor` variant's
/// CSS keyword, which is what every backend maps from, so it stays as it is in every language.
fn swatch(cursor: Cursor, id_prefix: &str) -> impl Piece {
    let id: &'static str =
        Box::leak(format!("cursor-{id_prefix}-{}", cursor.css_name()).into_boxed_str());
    // `.id` on the label itself, before the wrappers: `.background()` adds a container node,
    // and an id after it would name that instead (a script then reads no text).
    label(cursor.css_name().to_string())
        .id(id)
        .font(Font::Callout)
        .padding(Insets::symmetric(14.0, 10.0))
        .background(Color::rgba(0.5, 0.5, 0.5, 0.12))
        .corner_radius(8.0)
        .cursor(cursor)
}

/// Tiles in rows of four: Day has no flow container, and four fits the page width on every
/// desktop while a phone, which shows no pointer anyway, just scrolls.
fn tile_rows(tiles: Vec<AnyPiece>) -> impl Piece {
    let mut rows: Vec<AnyPiece> = Vec::new();
    let mut it = tiles.into_iter().peekable();
    while it.peek().is_some() {
        let chunk: Vec<AnyPiece> = it.by_ref().take(4).collect();
        rows.push(row(PieceVec(chunk)).spacing(8.0).any());
    }
    column(PieceVec(rows)).spacing(8.0)
}

fn swatches_section(
    title: LocalizedText,
    id_prefix: &'static str,
    cursors: &[Cursor],
) -> impl Piece {
    let tiles: Vec<AnyPiece> = cursors
        .iter()
        .map(|c| swatch(c.clone(), id_prefix).any())
        .collect();
    section((tile_rows(tiles),)).title(title)
}

/// One box whose shape follows the picker: the reactive form, `cursor(closure)`.
fn reactive_section() -> impl Piece {
    let names: Vec<String> = Cursor::NAMED
        .iter()
        .map(|c| c.css_name().to_string())
        .collect();
    let selected = Signal::new(1usize);
    let shape = move || Cursor::NAMED[selected.get().min(Cursor::NAMED.len() - 1)].clone();
    section((
        label(crate::res::str::cursors_reactive_caption()).font(Font::Footnote),
        labeled(
            crate::res::str::cursors_reactive_pick(),
            picker(names, selected).id("cursors-pick"),
        ),
        label(move || shape().css_name().to_string())
            .id("cursors-reactive-box")
            .font(Font::Title2)
            .frame(320.0, 120.0)
            .background(Color::rgba(0.2, 0.5, 0.9, 0.15))
            .corner_radius(12.0)
            .cursor(shape),
    ))
    .title(crate::res::str::cursors_reactive_title())
}

/// Two shapes, nested: the pointer is a crosshair over the outer box and a hand over the inner
/// label, then a crosshair again on the way out.
fn nested_section() -> impl Piece {
    section((
        label(crate::res::str::cursors_nested_caption()).font(Font::Footnote),
        column((
            label(crate::res::str::cursors_nested_outer()).font(Font::Footnote),
            label(crate::res::str::cursors_nested_inner())
                .id("cursors-nested-inner")
                .padding(Insets::symmetric(20.0, 12.0))
                .background(Color::rgba(0.9, 0.5, 0.2, 0.2))
                .corner_radius(8.0)
                .cursor(Cursor::Pointer),
        ))
        .id("cursors-nested-outer")
        .spacing(12.0)
        .padding(24.0)
        .background(Color::rgba(0.5, 0.5, 0.5, 0.1))
        .corner_radius(12.0)
        .cursor(Cursor::Crosshair),
    ))
    .title(crate::res::str::cursors_nested_title())
}

/// The shapes only this toolkit names, from `day::cursor::<toolkit>`: each constant is gated
/// on the toolkit's feature, so this section is compiled per backend and empty where there
/// are none.
fn native_section() -> impl Piece {
    let extras: Vec<(&'static str, Cursor)> = native_extras();
    let tiles: Vec<AnyPiece> = extras
        .into_iter()
        .map(|(name, c)| {
            let id: &'static str = Box::leak(format!("cursor-native-{name}").into_boxed_str());
            label(name)
                .id(id)
                .font(Font::Callout)
                .padding(Insets::symmetric(14.0, 10.0))
                .background(Color::rgba(0.3, 0.7, 0.4, 0.15))
                .corner_radius(8.0)
                .cursor(c)
                .any()
        })
        .collect();
    let body: AnyPiece = if tiles.is_empty() {
        label(crate::res::str::cursors_native_none())
            .font(Font::Footnote)
            .any()
    } else {
        tile_rows(tiles).any()
    };
    section((body,)).title(crate::res::str::cursors_native_title())
}

#[cfg(feature = "appkit")]
fn native_extras() -> Vec<(&'static str, Cursor)> {
    use day::cursor::appkit as c;
    vec![
        ("DISAPPEARING_ITEM", c::DISAPPEARING_ITEM),
        ("DRAG_COPY", c::DRAG_COPY),
        ("DRAG_LINK", c::DRAG_LINK),
        ("CONTEXTUAL_MENU", c::CONTEXTUAL_MENU),
        ("I_BEAM_VERTICAL", c::I_BEAM_VERTICAL),
    ]
}
#[cfg(feature = "qt")]
fn native_extras() -> Vec<(&'static str, Cursor)> {
    use day::cursor::qt as c;
    vec![
        ("WHATS_THIS", c::WHATS_THIS),
        ("BUSY", c::BUSY),
        ("UP_ARROW", c::UP_ARROW),
        ("SPLIT_H", c::SPLIT_H),
        ("SPLIT_V", c::SPLIT_V),
    ]
}
#[cfg(feature = "xaml")]
fn native_extras() -> Vec<(&'static str, Cursor)> {
    use day::cursor::xaml as c;
    vec![
        ("PERSON", c::PERSON),
        ("PIN", c::PIN),
        ("UP_ARROW", c::UP_ARROW),
        ("APP_STARTING", c::APP_STARTING),
    ]
}
#[cfg(feature = "mdc")]
fn native_extras() -> Vec<(&'static str, Cursor)> {
    use day::cursor::android as c;
    vec![
        ("ALL_SCROLL", c::ALL_SCROLL),
        ("NO_DROP", c::NO_DROP),
        ("TOP_RIGHT_DIAGONAL", c::TOP_RIGHT_DIAGONAL),
        ("TOP_LEFT_DIAGONAL", c::TOP_LEFT_DIAGONAL),
    ]
}
#[cfg(not(any(feature = "appkit", feature = "qt", feature = "xaml", feature = "mdc")))]
fn native_extras() -> Vec<(&'static str, Cursor)> {
    Vec::new()
}
