use day::prelude::*;

use crate::widgets::page;

/// Bundled resources (§18.3): an image loaded *by name* from the `images/` resource (the native
/// image pipeline) shown in each content mode, plus efficient random-access reads of arbitrary
/// embedded data via `resource()`.
pub(crate) fn resources_page() -> AnyPiece {
    page(
        crate::res::str::nav_resources(),
        "resources-title",
        Some(crate::res::str::resources_caption()),
        form((image_section(), vectors_section(), data_section())).any(),
    )
}

/// `image(res::images::…)` resolves by name through the backend's native image path (bundle
/// file / Assets.car / R.drawable / …). One asset rendered under each content mode shows what
/// Fit (default), Fill, and Stretch each do to a non-square frame.
fn image_section() -> impl Piece {
    fn mode(label_text: LocalizedText, img: AnyPiece) -> AnyPiece {
        column((img, label(label_text).font(Font::Caption)))
            .spacing(6.0)
            .align(HAlign::Center)
            .any()
    }
    section((
        image(crate::res::images::day_logo).frame(96.0, 96.0),
        label(crate::res::str::resources_modes_note()).font(Font::Footnote),
        row((
            mode(
                crate::res::str::image_mode_fit(),
                image(crate::res::images::day_logo).frame(120.0, 72.0).any(),
            ),
            mode(
                crate::res::str::image_mode_fill(),
                image(crate::res::images::day_logo)
                    .fill()
                    .frame(120.0, 72.0)
                    .any(),
            ),
            mode(
                crate::res::str::image_mode_stretch(),
                image(crate::res::images::day_logo)
                    .stretch()
                    .frame(120.0, 72.0)
                    .any(),
            ),
        ))
        .spacing(16.0),
    ))
    .title(crate::res::str::resources_image_section())
}

/// Random-access reads of two bundled data resources, via the zero-copy `resource()` view.
fn data_section() -> impl Piece {
    let (numbers_line, greeting_line) = resource_lines();
    section((
        label(move || numbers_line.clone()).id("resources-numbers"),
        label(move || greeting_line.clone()).id("resources-greeting"),
    ))
    .title(crate::res::str::resources_data_section())
}

/// Open two bundled data resources and format one random-access read from each. `numbers.bin` holds
/// the bytes `0..=255`, so `byte[100]` must be `100`; `greeting.txt` is a short UTF-8 string.
fn resource_lines() -> (String, String) {
    let numbers = match resource(crate::res::assets::numbers_bin) {
        Some(r) => {
            let mut b = [0u8; 1];
            r.read_at(100, &mut b);
            crate::res::str::resources_numbers(b[0] as f64, r.len() as f64).format()
        }
        None => "numbers.bin: (not bundled)".to_string(),
    };
    let greeting = match resource(crate::res::assets::greeting_txt) {
        Some(r) => {
            crate::res::str::resources_greeting(String::from_utf8_lossy(r.as_slice()).into_owned())
                .format()
        }
        None => "greeting.txt: (not bundled)".to_string(),
    };
    (numbers, greeting)
}

/// `vector(res::vectors::…)` (docs/vectors.md): the same glyphs the sidebar rows use, drawn
/// in-page — resolution-independent, and tinted through the piece's `.tint(…)` where the
/// backend supports recoloring (Apple template rendering, Android drawable tint, GTK pixel
/// recolor; other backends draw the authored color).
fn vectors_section() -> impl Piece {
    // Which stop of the palette ramp the live-tint glyph is showing.
    let live = Signal::new(0usize);
    fn v(g: day::VectorName) -> AnyPiece {
        vector(g)
            .tint(crate::palette::SLATE)
            .frame(28.0, 28.0)
            .any()
    }
    fn t(g: day::VectorName, c: Color) -> AnyPiece {
        vector(g).tint(c).frame(28.0, 28.0).any()
    }
    fn z(px: f64) -> AnyPiece {
        vector(crate::res::vectors::nav_webview)
            .tint(crate::palette::SLATE)
            .frame(px, px)
            .any()
    }
    use crate::res::vectors as gv;
    section((
        label(crate::res::str::vectors_note()).font(Font::Footnote),
        column((
            row((
                v(gv::nav_about),
                v(gv::nav_animation),
                v(gv::nav_canvas),
                v(gv::nav_controls),
                v(gv::nav_crash),
                v(gv::nav_dates),
                v(gv::nav_focus),
                v(gv::nav_grid),
            ))
            .spacing(12.0),
            row((
                v(gv::nav_list),
                v(gv::nav_localization),
                v(gv::nav_media),
                v(gv::nav_menus),
                v(gv::nav_refresh),
                v(gv::nav_resources),
                v(gv::nav_scripting),
                v(gv::nav_services),
            ))
            .spacing(12.0),
            row((
                v(gv::nav_stack),
                v(gv::nav_tabs),
                v(gv::nav_text),
                v(gv::nav_textareas),
                v(gv::nav_toolbars),
                v(gv::nav_tweaks),
                v(gv::nav_webview),
            ))
            .spacing(12.0),
        ))
        .spacing(10.0)
        .align(HAlign::Leading)
        .id("resources-vectors-grid"),
        // Tints: one glyph through the tint ladder.
        labeled(
            crate::res::str::vectors_tints(),
            row((
                t(gv::nav_scripting, crate::palette::SLATE),
                t(gv::nav_scripting, crate::palette::RUST),
                t(gv::nav_scripting, Color::rgba(0.18, 0.50, 0.94, 1.0)),
                t(gv::nav_scripting, Color::rgba(0.16, 0.65, 0.37, 1.0)),
            ))
            .spacing(12.0)
            .id("resources-vectors-tints"),
        ),
        // A LIVE tint: the same glyph bound to a signal, so pressing the button repaints the
        // realized view through `ImagePatch::Tint` instead of rebuilding it (docs/vectors.md).
        labeled(
            crate::res::str::vectors_live_tint(),
            row((
                vector(gv::nav_animation)
                    .tint(move || crate::palette::RAMP[live.get() % crate::palette::RAMP.len()])
                    .frame(28.0, 28.0)
                    .id("vector-live-tint"),
                button(crate::res::str::vectors_cycle_tint())
                    .action(move || live.update(|i| *i += 1))
                    .id("vector-cycle-tint"),
            ))
            .spacing(12.0)
            .align(VAlign::Center),
        ),
        // Weights: ONE SF-template master (home_symbol.svg), three true weight variants
        // selected through the piece API (docs/vectors.md).
        labeled(
            crate::res::str::vectors_weights(),
            row((
                vector(gv::home_symbol)
                    .weight(VectorWeight::Light)
                    .tint(crate::palette::SLATE)
                    .frame(36.0, 36.0),
                vector(gv::home_symbol)
                    .tint(crate::palette::SLATE)
                    .frame(36.0, 36.0),
                vector(gv::home_symbol)
                    .weight(VectorWeight::Bold)
                    .tint(crate::palette::SLATE)
                    .frame(36.0, 36.0),
            ))
            .spacing(12.0)
            .id("resources-vectors-weights"),
        ),
        // Sizes: one vector, sharp at every scale — the point of vectors.
        labeled(
            crate::res::str::vectors_sizes(),
            row((z(16.0), z(24.0), z(48.0), z(96.0)))
                .spacing(12.0)
                .id("resources-vectors-sizes"),
        ),
    ))
    .title(crate::res::str::vectors_title())
}
