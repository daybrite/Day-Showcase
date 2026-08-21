use day::prelude::*;
use day_piece_colorpicker::color_picker;

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
    .any()
}

/// `image(res::images::…)` resolves by name through the backend's native image path (bundle
/// file / Assets.car / R.drawable / …). One asset rendered under each content mode shows what
/// Fit (default), Fill, and Stretch each do to a non-square frame.
fn image_section() -> impl Piece {
    fn mode(label_text: LocalizedText, img: impl Piece) -> impl Piece {
        column((img, label(label_text).font(Font::Caption)))
            .spacing(6.0)
            .align(HAlign::Center)
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

/// The glyph sizes the Size picker offers, in points.
const GLYPH_SIZES: [f64; 4] = [16.0, 24.0, 36.0, 48.0];

/// `vector(res::vectors::…)` (docs/vectors.md): the same glyphs the sidebar rows use, drawn
/// in-page — resolution-independent, and tinted through the piece's `.tint(…)` where the
/// backend supports recoloring (Apple template rendering, Android drawable tint, GTK pixel
/// recolor; other backends draw the authored color).
fn vectors_section() -> impl Piece {
    // Which stop of the palette ramp the live-tint glyph is showing, and the color the two
    // pickers write. The ramp button and both pickers drive the SAME signal, which is what makes
    // "cycle, then open a picker" pick up where the ramp left off.
    let live = Signal::new(0usize);
    let tint = Signal::new(crate::palette::RAMP[0]);
    // The Weight and Size pickers apply to every glyph in the wrapping row below.
    let weight = Signal::new(1usize); // Light / Regular / Bold — Regular by default
    let size = Signal::new(1usize); // 16 / 24 / 36 / 48 pt
    // Edge length of the zoomable art, in points. The tiger's alone now: the glyph row takes its
    // size from the Size picker, so the two no longer fight over one slider.
    let zoom = Signal::new(96.0_f64);

    fn weight_of(i: usize) -> VectorWeight {
        match i {
            0 => VectorWeight::Light,
            2 => VectorWeight::Bold,
            _ => VectorWeight::Regular,
        }
    }

    use crate::res::vectors as gv;
    section((
        label(crate::res::str::vectors_note()).font(Font::Footnote),
        // The two pickers that govern EVERY glyph below. Neither is reactive on the piece:
        // `.weight(…)` resolves to a different staged source name and `.frame(…)` is a layout
        // constant, so both are build-time facts — which is why the row is rebuilt through
        // `each` keyed on the pair rather than patched.
        labeled(
            crate::res::str::vectors_weights(),
            picker(
                [
                    crate::res::str::text_weight_light().format(),
                    crate::res::str::text_weight_regular().format(),
                    crate::res::str::text_weight_bold().format(),
                ],
                weight,
            )
            .segmented()
            .id("vector-weight"),
        ),
        labeled(
            crate::res::str::vectors_sizes(),
            picker(GLYPH_SIZES.map(|s| format!("{s:.0}")), size)
                .segmented()
                .id("vector-size"),
        ),
        // The glyph row is DERIVED, not hand-written: one list of glyphs in one `row`, wrapped by
        // the row itself. It was a `grid` whose column count this page computed from the window's
        // width class and the glyph size — arithmetic that existed only because a row could not
        // wrap. `RowFit::Wrap` (docs/size-classes.md) moved that into layout, where the run break
        // happens against the measured width rather than against a guess.
        //
        // Still rebuilt through `each`, keyed on the weight and size the pickers hold, because
        // both change what each glyph IS rather than how it is laid out.
        each(
            items(move || vec![(weight.get(), size.get())], |k| *k),
            move |slot| {
                let (w, s) = slot.get();
                let px = GLYPH_SIZES[s.min(GLYPH_SIZES.len() - 1)];
                // `home_symbol` leads: it is the one SF-template source here with true
                // per-weight art. Every glyph after it is a plain SVG, so moving the Weight
                // picker off Regular exercises the ALIASING path on all 23 at once — a weight a
                // source has no art for must resolve back to its base glyph rather than draw
                // nothing (docs/vectors.md). Doing it in bulk is what caught android-mdc
                // resolving the image piece's name WITHOUT that fallback: every aliased glyph was
                // blank there, and `assert_visible` could not see it because the ImageView had a
                // frame either way.
                let glyphs = [
                    gv::home_symbol,
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
                let cells: Vec<AnyPiece> = glyphs
                    .into_iter()
                    .map(|g| {
                        vector(g)
                            .weight(weight_of(w))
                            .tint(crate::palette::SLATE)
                            .frame(px, px)
                            .any()
                    })
                    .collect();
                row(PieceVec(cells))
                    .spacing(12.0)
                    .fit(RowFit::Wrap { run_spacing: 12.0 })
                    .grow_w()
                    .id("resources-vectors-row")
            },
        ),
        // The aliasing case again, called out on its own so a script can assert it directly: a
        // plain SVG asked for Bold, which it has no art for. It must look exactly like Regular —
        // what that proves is that the alias RESOLVES instead of drawing nothing.
        row((
            vector(gv::nav_about)
                .weight(VectorWeight::Bold)
                .tint(crate::palette::SLATE)
                .frame(28.0, 28.0)
                .id("vector-aliased-weight"),
            label(crate::res::str::vectors_alias_note()).font(Font::Footnote),
        ))
        .spacing(8.0)
        .align(VAlign::Top),
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
        // A LIVE tint: the same glyph bound to a signal, so any of the three controls beside it
        // repaints the realized view through `ImagePatch::Tint` instead of rebuilding it
        // (docs/vectors.md). Cycle walks the palette ramp; the two wells open a real color
        // chooser — one the platform's, one Day's own (docs/colorpicker.md), which is the whole
        // point of having both here.
        labeled(
            crate::res::str::vectors_live_tint(),
            row((
                vector(gv::nav_animation)
                    .tint(move || tint.get())
                    .frame(28.0, 28.0)
                    .id("vector-live-tint"),
                button(crate::res::str::vectors_cycle_tint())
                    .action(move || {
                        live.update(|i| *i += 1);
                        tint.set(
                            crate::palette::RAMP[live.get_untracked() % crate::palette::RAMP.len()],
                        );
                    })
                    .id("vector-cycle-tint"),
                // The platform's own chooser. `.native()` is literal — on a toolkit with no
                // color picker (android-mdc, harmony-arkui) this draws Day's placeholder rather
                // than quietly substituting the composed panel, which is exactly what the page
                // is here to show. The banner below says so in words.
                color_picker(tint)
                    .native()
                    .title(crate::res::str::vectors_pick_tint())
                    .id("vector-tint-native"),
                // Day's own panel, built from pieces and a canvas — identical on all nine
                // targets. Its `key` is also the well's dayscript id (the piece tags the button
                // itself, since an id set from out here would land on the wrapper).
                color_picker(tint)
                    .composed()
                    .title(crate::res::str::vectors_pick_tint())
                    .key("vector-tint-composed"),
            ))
            .spacing(12.0)
            .fit(RowFit::Wrap { run_spacing: 8.0 })
            .align(VAlign::Center),
        ),
        label(crate::res::str::vectors_tint_idioms()).font(Font::Footnote),
        // On android-mdc and harmony-arkui the FIRST well has nothing to open — neither platform
        // ships a color chooser anywhere — so it draws Day's `⟨day.piece.colorpicker⟩`
        // placeholder and this banner says why. The second well works there exactly as it does
        // here, which is the argument for having built it. (Same shape as the Controls page's
        // note beside the combo box on iOS.)
        crate::widgets::support_note(day_piece_colorpicker::support()),
        // Full-colour art, NOT a tintable glyph: everything above is a monochrome symbol that
        // takes a `.tint`, so the pipeline's other half — many-path, authored-colour art — had
        // no example here. The tiger is 240 paths, and it stays a real vector on every backend
        // (on android-mdc a VectorDrawable, not a raster — docs/vectors.md).
        //
        // The glyph is REBUILT at each zoom step, not transform-scaled. A `.scale()` would
        // magnify whatever the backend last rasterized — which is the very thing this demo
        // exists to disprove — so the size change goes through `each`, whose keyed diff disposes
        // the old view and asks the backend for the art at the NEW size. Every step is a fresh
        // render from the SVG, which is what keeps the edges crisp as it grows.
        //
        // Keyed on 8 px steps rather than the raw f64: a slider drag would otherwise rebuild a
        // 240-path drawing on every pixel of travel.
        //
        // Centred while it fits, pannable once it does not. The top of the zoom range is five
        // times what it was, which is wider than the page's content column on every desktop
        // window and several times a phone's. Clipping the art at the column edge would hide
        // exactly the detail the raised range exists to show, so past that width the art keeps
        // its full size inside a horizontal scroll strip and the reader pans across it.
        //
        // The `each` key therefore carries the width CLASS as well as the zoom step: crossing a
        // breakpoint changes which of the two arrangements applies, and a key that ignored it
        // would leave a resized window showing the wrong one.
        each(
            items(
                move || vec![((zoom.get() / 8.0).round() as i64, tiger_room() as i64)],
                |k| *k,
            ),
            move |slot| {
                let (step, room) = slot.get();
                let px = step as f64 * 8.0;
                let art = vector(gv::tiger).frame(px, px).id("vector-tiger");
                // Two different piece types, one branch each: `Either` keeps both concrete
                // instead of boxing whichever arm this row takes.
                if px <= room as f64 {
                    Either::Left(column((art,)).align(HAlign::Center).grow_w())
                } else {
                    Either::Right(scroll(art).horizontal().grow_w())
                }
            },
        ),
        // The zoom sits BELOW the art it drives now that it drives only that art — the glyph row
        // above takes its size from the Size picker instead.
        labeled(
            crate::res::str::vectors_zoom(),
            slider(zoom).range(48.0..=1200.0).id("vector-zoom"),
        ),
        label(crate::res::str::vectors_scene_note()).font(Font::Footnote),
    ))
    .title(crate::res::str::vectors_title())
}

/// One glyph of the tint ladder, at the fixed size that row has always used.
fn t(g: day::VectorName, c: Color) -> impl Piece {
    vector(g).tint(c).frame(28.0, 28.0)
}

/// Roughly the width the section card leaves the tiger at the window's current width class —
/// the point past which the art stops being centred and becomes a pannable strip instead.
///
/// Day reports the window's width CLASS, not a measured width (docs/size-classes.md), so this is
/// a per-class constant rather than a measurement. The read is tracked, so a window dragged
/// across a breakpoint re-keys the `each` and swaps the arrangement.
fn tiger_room() -> f64 {
    match size_class().map(|c| c.width) {
        None | Some(WidthClass::Compact) => 300.0,
        Some(WidthClass::Medium) => 480.0,
        _ => 620.0,
    }
}
