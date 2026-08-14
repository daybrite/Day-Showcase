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
    // Edge length of the zoomable art, in points.
    let zoom = Signal::new(96.0_f64);
    /// One glyph cell at the current size.
    fn v(g: day::VectorName, px: f64) -> AnyPiece {
        vector(g).tint(crate::palette::SLATE).frame(px, px).any()
    }
    /// The glyph edge for a given zoom, scaled from the 28 pt the grid used when it was three
    /// hand-written rows — so the default slider position still draws the size it always did.
    fn glyph_px(zoom: f64) -> f64 {
        (28.0 * zoom / 96.0).clamp(14.0, 72.0)
    }
    /// Columns for the glyph grid.
    ///
    /// Day reports the window's width CLASS, not a measured width (docs/size-classes.md), so the
    /// base count is per-class and the glyph size scales it: bigger art means fewer columns,
    /// which is what stops the grid running off the edge of a narrow window as the slider grows.
    fn glyph_columns(class: Option<SizeClass>, px: f64) -> usize {
        let base: f64 = match class.map(|c| c.width) {
            None | Some(WidthClass::Compact) => 4.0,
            Some(WidthClass::Medium) => 6.0,
            Some(WidthClass::Expanded) => 8.0,
            Some(WidthClass::Large) => 10.0,
            Some(WidthClass::ExtraLarge) => 12.0,
        };
        ((base * 28.0 / px).round() as i64).clamp(2, base as i64) as usize
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
        // The zoom governs BOTH the glyph grid below and the full-colour art further down, so it
        // sits above the first thing it controls.
        labeled(
            crate::res::str::vectors_zoom(),
            slider(zoom).range(48.0..=240.0).id("vector-zoom"),
        ),
        // The glyph grid is DERIVED, not hand-written: one list of glyphs, chunked into rows by a
        // column count that falls out of the window's width class and the current glyph size. It
        // was three fixed rows of eight, which meant a narrow window clipped them and adding a
        // page's icon meant editing the layout.
        //
        // Rebuilt through `each` (rather than a reactive size, which pieces have no decorator for)
        // keyed on BOTH inputs — the zoom step and the column count — so crossing a breakpoint
        // re-chunks the rows and a zoom step re-renders every glyph at its new size. Keying on the
        // derived column count rather than the raw class means a resize that does not change the
        // layout costs nothing.
        each(
            move || {
                let px = glyph_px(zoom.get());
                // Both reads are tracked: the grid rebuilds on a zoom step and on a breakpoint.
                vec![((px * 4.0).round() as i64, glyph_columns(size_class(), px))]
            },
            |k| *k,
            move |slot| {
                let (px_q, cols) = slot.get();
                let px = px_q as f64 / 4.0;
                let glyphs = [
                    gv::nav_about,
                    gv::nav_animation,
                    gv::nav_canvas,
                    gv::nav_controls,
                    gv::nav_crash,
                    gv::nav_dates,
                    gv::nav_focus,
                    gv::nav_grid,
                    gv::nav_list,
                    gv::nav_localization,
                    gv::nav_media,
                    gv::nav_menus,
                    gv::nav_refresh,
                    gv::nav_resources,
                    gv::nav_scripting,
                    gv::nav_services,
                    gv::nav_stack,
                    gv::nav_tabs,
                    gv::nav_text,
                    gv::nav_textareas,
                    gv::nav_toolbars,
                    gv::nav_tweaks,
                    gv::nav_webview,
                ];
                let rows: Vec<AnyPiece> = glyphs
                    .chunks(cols)
                    .map(|chunk| {
                        grid_row(PieceVec(chunk.iter().cloned().map(|g| v(g, px)).collect())).any()
                    })
                    .collect();
                grid(PieceVec(rows))
                    .spacing(12.0)
                    .id("resources-vectors-grid")
            },
        ),
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
        // Full-colour art, NOT a tintable glyph: everything above is a monochrome symbol that
        // takes a `.tint`, so the pipeline's other half — many-path, authored-colour art — had
        // no example here. The tiger is 240 paths, and it stays a real vector on every backend
        // (on android-mdc a VectorDrawable, not a raster — docs/vectors.md). It follows the same
        // zoom slider as the grid above.
        //
        // The glyph is REBUILT at each zoom step, not transform-scaled. A `.scale()` would
        // magnify whatever the backend last rasterized — which is the very thing this demo
        // exists to disprove — so the size change goes through `each`, whose keyed diff disposes
        // the old view and asks the backend for the art at the NEW size. Every step is a fresh
        // render from the SVG, which is what keeps the edges crisp as it grows.
        //
        // Keyed on 8 px steps rather than the raw f64: a slider drag would otherwise rebuild a
        // 240-path drawing on every pixel of travel.
        each(
            move || vec![(zoom.get() / 8.0).round() as i64],
            |step| *step,
            move |slot| {
                let px = slot.get() as f64 * 8.0;
                vector(gv::tiger).frame(px, px).id("vector-tiger")
            },
        ),
        label(crate::res::str::vectors_scene_note()).font(Font::Footnote),
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
                // A PLAIN SVG at a weight: it has no weight axis, so it stages once and this
                // resolves back to the base glyph (docs/vectors.md). It looks identical to
                // Regular by design — what it proves is that the alias resolves instead of
                // drawing nothing, which is the failure this arrangement risks on every backend.
                vector(gv::nav_about)
                    .weight(VectorWeight::Bold)
                    .tint(crate::palette::SLATE)
                    .frame(36.0, 36.0)
                    .id("vector-aliased-weight"),
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
