// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The Images page: pictures of the sky in native image views.
//!
//! The cards show the three content modes, the crops an image takes (a circle, a radius, a
//! bordered surface, a surface that rounds two corners), a bundled PNG under a tint, a sprite
//! sheet rendered once and played by cell, a placeholder that fades to its picture, a picture
//! rendered down off screen, and one download with its progress, size and failure.
//!
//! A remote picture a card needs to measure or restyle goes through a small [`Loader`]: it
//! fetches the bytes with `day-part-http`, decodes them with `day::decode_image`, and publishes
//! each step as a signal. The cards that only show a picture use `day::http_image`. Frames have
//! a fixed height, so a picture arriving never moves the layout.

use std::cell::{Cell, RefCell};
use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;

use day::motion::{Animated, Animation, Playing};
use day::prelude::*;

use crate::gallery::{self, Tone, card, card_page, sky};
use crate::res::str as strings;
use crate::res::vectors;

// ---------------------------------------------------------------------------
// The loader
// ---------------------------------------------------------------------------

/// The largest download a card accepts, in bytes.
const MAX_BYTES: usize = 32 * 1024 * 1024;

thread_local! {
    /// The client every download of this page goes through.
    static CLIENT: day_part_http::Client = day_part_http::Client::new();
}

/// One task at a time, aborted when the scope that made the slot goes away.
#[derive(Clone)]
struct TaskSlot(Rc<Cell<Option<day::TaskHandle>>>);

impl TaskSlot {
    /// An empty slot, tied to the scope being built.
    fn new() -> Self {
        let slot = TaskSlot(Rc::new(Cell::new(None)));
        let end = slot.clone();
        Scope::current().on_cleanup(move || end.stop());
        slot
    }

    /// Runs `work`, aborting what ran before it.
    fn run(&self, work: impl Future<Output = ()> + 'static) {
        self.stop();
        self.0.set(Some(day::task(work)));
    }

    /// Aborts the running task, if there is one.
    fn stop(&self) {
        if let Some(task) = self.0.take() {
            task.abort();
        }
    }
}

/// Where a load stands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    /// Nothing was asked for.
    Idle,
    /// The bytes are on their way.
    Downloading,
    /// The bytes arrived and are being decoded.
    Decoding,
    /// The picture is decoded.
    Loaded,
    /// The download or the decode failed.
    Failed,
}

/// A remote picture's load, as signals: the cards are built on it.
#[derive(Clone)]
struct Loader {
    /// Where the load stands.
    phase: Signal<Phase>,
    /// How many encoded bytes arrived.
    size: Signal<usize>,
    /// The decoded picture.
    bitmap: Signal<Option<day::Bitmap>>,
    /// Why the load failed.
    error: Signal<String>,
    /// The share of the download received, 0 to 1, where the server states a length.
    progress: Signal<f64>,
    /// The running load.
    task: TaskSlot,
}

impl Loader {
    /// A loader that holds nothing.
    fn new() -> Self {
        Loader {
            phase: Signal::new(Phase::Idle),
            size: Signal::new(0),
            bitmap: Signal::new(None),
            error: Signal::new(String::new()),
            progress: Signal::new(0.0),
            task: TaskSlot::new(),
        }
    }

    /// A loader already loading `url`.
    fn loading(url: String) -> Self {
        let loader = Loader::new();
        loader.load(url);
        loader
    }

    /// Starts loading `url`, replacing what this loader held.
    fn load(&self, url: String) {
        self.clear(Phase::Downloading);
        let this = self.clone();
        self.task.run(async move {
            match this.fetch(&url).await {
                Ok(bitmap) => {
                    this.bitmap.set(Some(bitmap));
                    this.phase.set(Phase::Loaded);
                }
                Err(text) => {
                    this.error.set(text);
                    this.phase.set(Phase::Failed);
                }
            }
        });
    }

    /// Drops what this loader held and stops its load.
    fn reset(&self) {
        self.clear(Phase::Idle);
    }

    /// Stops the running load and empties every signal, leaving the loader in `phase`.
    fn clear(&self, phase: Phase) {
        self.task.stop();
        self.phase.set(phase);
        self.size.set(0);
        self.bitmap.set(None);
        self.error.set(String::new());
        self.progress.set(0.0);
    }

    /// Downloads and decodes `url`, or says why it could not.
    async fn fetch(&self, url: &str) -> Result<day::Bitmap, String> {
        let progress = self.progress;
        let client = CLIENT.with(Clone::clone);
        let response = client
            .fetch_limited_with_progress_future(
                day_part_http::Request::get(url),
                MAX_BYTES,
                move |received, total| {
                    let Some(total) = total.filter(|total| *total > 0) else {
                        return;
                    };
                    let share = (received as f64 / total as f64).min(1.0);
                    // A chunk arrives far more often than a bar needs to move.
                    if share >= 1.0 || share - progress.get_untracked() >= 0.01 {
                        progress.set(share);
                    }
                },
            )
            .await
            .map_err(|error| error.to_string())?;
        if !(200..300).contains(&response.status) {
            return Err(strings::images_error_status(response.status.to_string()).format());
        }
        self.size.set(response.body.len());
        self.progress.set(1.0);
        self.phase.set(Phase::Decoding);
        day::decode_image(Arc::new(response.body))
            .await
            .map_err(|error| error.to_string())
    }
}

/// The pixel size of `bitmap` as text, or the loading text while there is none.
fn pixel_text(bitmap: Option<day::Bitmap>) -> String {
    match bitmap {
        Some(bitmap) => {
            let pixels = bitmap.info().pixels;
            strings::images_pixels(strings::ImagesPixels {
                width: format!("{:.0}", pixels.width),
                height: format!("{:.0}", pixels.height),
            })
            .format()
        }
        None => strings::images_loading().format(),
    }
}

// ---------------------------------------------------------------------------
// Frames and tiles
// ---------------------------------------------------------------------------

/// The corner radius of a picture's frame.
const FRAME_RADIUS: Radius = Radius::M;

/// How the native image view scales the picture in its frame.
#[derive(Clone, Copy)]
enum Mode {
    /// `.fit()`: the whole picture, letterboxed.
    Fit,
    /// `.fill()`: covers the frame, cropping the overflow.
    Fill,
    /// `.stretch()`: the frame's proportions, not the picture's.
    Stretch,
}

/// A decoded picture in a native image view, scaled as `mode` says. The loader's signal keeps
/// the decode alive, and the view names it by id.
fn shot(bitmap: Signal<Option<day::Bitmap>>, mode: Mode) -> impl Piece {
    let view = image(move || match bitmap.get() {
        Some(bitmap) => day::ImageSource::from(bitmap.id()),
        None => day::ImageSource::Named(String::new()),
    })
    .decorative();
    let view = match mode {
        Mode::Fit => view.fit(),
        Mode::Fill => view.fill(),
        Mode::Stretch => view.stretch(),
    };
    // An empty source clears the view only on some toolkits; elsewhere the last picture stays
    // (docs/images.md), so a cleared card hides it.
    view.opacity(move || if bitmap.get().is_some() { 1.0 } else { 0.0 })
}

/// A rounded field `height` points high and as wide as its tile, in the raised fill a picture
/// replaces, around `content`.
fn frame(height: f64, content: impl Piece) -> impl Piece {
    zstack((content,))
        .height(height)
        .grow(Dimension::Width)
        .background(gallery::RAISED)
        .corner_radius(FRAME_RADIUS)
}

/// A specimen over its name.
fn tile<M>(specimen: impl Piece, name: impl IntoText<M>) -> AnyPiece {
    column((
        specimen,
        label(name)
            .font(Font::Caption)
            .weight(FontWeight::Medium)
            .single_line(),
    ))
    .spacing(5.0)
    .align(HAlign::Center)
    .grow(Dimension::Width)
    .any()
}

/// A row of tiles sharing the card's width.
fn tiles(tiles: Vec<AnyPiece>) -> impl Piece {
    row(tiles)
        .spacing(Space::M)
        .align(VAlign::Top)
        .grow(Dimension::Width)
}

// ---------------------------------------------------------------------------
// Content modes
// ---------------------------------------------------------------------------

/// Content modes: Saturn, twice as wide as it is tall, in three frames of one size.
fn modes_card() -> AnyPiece {
    let bitmap = Loader::loading(sky::url(sky::SATURN, sky::Size::Small)).bitmap;
    let mode = move |mode: Mode, name: day::LocalizedText| {
        tile(frame(92.0, shot(bitmap, mode).grow(Dimension::Both)), name)
    };
    card(
        vectors::grid_sizing,
        strings::images_modes_title(),
        strings::images_modes_subtitle(),
        column((
            tiles(vec![
                mode(Mode::Fit, strings::images_mode_fit()),
                mode(Mode::Fill, strings::images_mode_fill()),
                mode(Mode::Stretch, strings::images_mode_stretch()),
            ]),
            gallery::caption(strings::images_name_saturn(), move || {
                pixel_text(bitmap.get())
            }),
            label(strings::images_credit())
                .font(Font::Caption)
                .secondary(),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .id(day::id!(images_modes))
    .build()
}

// ---------------------------------------------------------------------------
// Crops
// ---------------------------------------------------------------------------

/// The side of a crop specimen.
const CROP_SIDE: f64 = 68.0;
/// The width of the bordered crop's line.
const CROP_BORDER: f64 = 2.5;

/// A decoded picture in a native image view that covers a square of `side` points.
fn covering(bitmap: Signal<Option<day::Bitmap>>, side: f64) -> impl Piece {
    shot(bitmap, Mode::Fill).frame(side, side)
}

/// Crops: the circle and the radius `http_image` clips to, then two image views on a
/// `Surface`, one inside a border that follows the corners and one under a surface that rounds
/// two corners of its four.
fn crops_card() -> AnyPiece {
    let sun = Loader::loading(sky::url(sky::SUN, sky::Size::Small)).bitmap;
    let earth = Loader::loading(sky::url(sky::EARTH, sky::Size::Small)).bitmap;
    let remote = |picture: &str| {
        day::http_image(sky::url(picture, sky::Size::Small)).placeholder_color(gallery::RAISED)
    };
    // The picture sits inside the line and rounds its corners to the line's inner edge, so
    // the two never overlap.
    let bordered = covering(sun, CROP_SIDE - 2.0 * CROP_BORDER)
        .corner_radius(16.0 - CROP_BORDER)
        .padding(CROP_BORDER)
        .surface(
            Surface::new()
                .fill(gallery::RAISED)
                .corner_radius(16.0)
                .border(Tone::Warn.color(), CROP_BORDER),
        );
    let cornered = covering(earth, CROP_SIDE).surface(
        Surface::new()
            .fill(gallery::RAISED)
            .corner_radius(30.0)
            .corners(Corners::TOP_RIGHT | Corners::BOTTOM_LEFT),
    );
    card(
        vectors::nav_styles,
        strings::images_corners_title(),
        strings::images_corners_subtitle(),
        tiles(vec![
            tile(
                remote(sky::MOON).circle().frame(CROP_SIDE, CROP_SIDE),
                strings::images_name_moon(),
            ),
            tile(
                remote(sky::MARS).rounded(18.0).frame(CROP_SIDE, CROP_SIDE),
                strings::images_name_mars(),
            ),
            tile(bordered, strings::images_name_sun()),
            tile(cornered, strings::images_name_earth()),
        ]),
    )
    .id(day::id!(images_corners))
    .build()
}

// ---------------------------------------------------------------------------
// Tint
// ---------------------------------------------------------------------------

/// The field under the untinted glyph: light in both appearances, since the file is black on
/// clear and would not show on a dark one.
const PAPER: Ink = Ink::adaptive(
    Color::rgba(0.0, 0.0, 0.0, 0.045),
    Color::rgba(1.0, 1.0, 1.0, 0.82),
);

/// A glyph tile: `mark` centered on a field in `fill`.
fn glyph_tile(mark: impl Piece, fill: Ink, name: day::LocalizedText) -> AnyPiece {
    tile(
        zstack((mark.frame(30.0, 30.0),))
            .height(58.0)
            .grow(Dimension::Width)
            .background(fill)
            .corner_radius(FRAME_RADIUS),
        name,
    )
}

/// Tint: a bundled raster glyph (a 96-pixel PNG, black on clear) as it was drawn, and in four
/// colors of the showcase's palette. A tint keeps a picture's alpha and replaces its color.
fn tint_card() -> AnyPiece {
    let glyph = || image(crate::res::images::nav_scrolling).decorative();
    let tinted = |tone: Tone, name: day::LocalizedText| {
        glyph_tile(glyph().tint(tone.color()), gallery::INSET, name)
    };
    card(
        vectors::nav_resources,
        strings::images_assets_title(),
        strings::images_assets_subtitle(),
        tiles(vec![
            glyph_tile(glyph(), PAPER, strings::images_asset_original()),
            tinted(Tone::Accent, strings::images_asset_sky()),
            tinted(Tone::Info, strings::images_asset_violet()),
            tinted(Tone::Warn, strings::images_asset_amber()),
            tinted(Tone::Danger, strings::images_asset_coral()),
        ]),
    )
    .id(day::id!(images_assets))
    .build()
}

// ---------------------------------------------------------------------------
// Sprite sheet
// ---------------------------------------------------------------------------

/// The phases of the Moon the sheet holds, from new to new.
const PHASES: u64 = 16;
/// The cells in a row of the sheet.
const SHEET_COLUMNS: u64 = 8;
/// The side of a sheet cell, in points.
const CELL_SIDE: f64 = 48.0;
/// How long the sheet takes to play through once, in milliseconds: five phases a second.
const SHEET_MS: u32 = PHASES as u32 * 200;

/// The phase `position` shows: a position counts phases, and goes on past the last one.
fn phase_index(position: f64) -> u64 {
    position.max(0.0).floor() as u64 % PHASES
}

/// Draws phase `index` of the Moon into `cell`: a dark disc, its sunlit half, and the
/// terminator's ellipse, which takes light from a crescent or adds it to a gibbous moon. The
/// colors are fixed, so the phases can be rendered once into a sheet.
fn moon_phase(draw: &mut Draw, cell: Rect, index: u64) {
    let lit = Color::rgb(0.95, 0.93, 0.86);
    let dark = Color::rgb(0.2, 0.22, 0.29);
    let radius = cell.size.width.min(cell.size.height) * 0.4;
    let (cx, cy) = (
        cell.origin.x + cell.size.width / 2.0,
        cell.origin.y + cell.size.height / 2.0,
    );
    let disc = Shape::Ellipse(Rect::new(
        cx - radius,
        cy - radius,
        radius * 2.0,
        radius * 2.0,
    ));
    let phase = (index % PHASES) as f64 / PHASES as f64;
    let waxing = phase < 0.5;
    // The terminator is half an ellipse whose width follows the cosine of the phase angle.
    let width = radius * (phase * std::f64::consts::TAU).cos().abs();
    let crescent = (phase < 0.25) || (phase >= 0.75);
    draw.fill(disc.clone(), dark);
    draw.clipped(disc, |draw| {
        let half = if waxing { cx } else { cx - radius };
        draw.fill(
            Shape::Rect(Rect::new(half, cy - radius, radius, radius * 2.0)),
            lit,
        );
        draw.fill(
            Shape::Ellipse(Rect::new(
                cx - width,
                cy - radius,
                width * 2.0,
                radius * 2.0,
            )),
            if crescent { dark } else { lit },
        );
    });
}

/// The rectangle of cell `index` in a sheet of `size`.
fn sheet_cell(size: Size, index: u64) -> Rect {
    let rows = PHASES.div_ceil(SHEET_COLUMNS);
    let (width, height) = (size.width / SHEET_COLUMNS as f64, size.height / rows as f64);
    Rect::new(
        (index % SHEET_COLUMNS) as f64 * width,
        (index / SHEET_COLUMNS) as f64 * height,
        width,
        height,
    )
}

/// The cell of `sheet` for the phase `position` shows, drawn to fill the canvas with
/// `Draw::image_region` (or the phase drawn directly until the sheet is rendered).
fn phase_player(sheet: Signal<Option<day::Bitmap>>, position: Animated) -> impl Piece {
    canvas(move |draw: &mut Draw, size: Size| {
        let cell = Rect::from_size(size);
        let index = phase_index(position.get());
        match sheet.get() {
            Some(sheet) => {
                let source = sheet_cell(sheet.info().pixels, index);
                draw.image_region(&sheet, source, cell);
            }
            None => moon_phase(draw, cell, index),
        }
    })
}

/// The whole sheet, fitted into the canvas, with an outline round the cell being played.
fn sheet_view(sheet: Signal<Option<day::Bitmap>>, position: Animated) -> impl Piece {
    canvas(move |draw: &mut Draw, size: Size| {
        let Some(sheet) = sheet.get() else {
            return;
        };
        let pixels = sheet.info().pixels;
        if pixels.width <= 0.0 || pixels.height <= 0.0 {
            return;
        }
        let factor = (size.width / pixels.width).min(size.height / pixels.height);
        let fitted = Size::new(pixels.width * factor, pixels.height * factor);
        let origin = Point::new(
            (size.width - fitted.width) / 2.0,
            (size.height - fitted.height) / 2.0,
        );
        draw.image(
            &sheet,
            Rect::new(origin.x, origin.y, fitted.width, fitted.height),
        );
        let cell = sheet_cell(fitted, phase_index(position.get()));
        draw.stroke(
            Shape::RoundedRect(
                Rect::new(
                    origin.x + cell.origin.x + 1.0,
                    origin.y + cell.origin.y + 1.0,
                    cell.size.width - 2.0,
                    cell.size.height - 2.0,
                ),
                5.0,
            ),
            Tone::Accent.color(),
            1.5,
        );
    })
}

/// Sprite sheet: sixteen phases of the Moon recorded as one drawing, rendered once with
/// `Drawing::to_bitmap`, and played a cell at a time by a repeating `day::motion` value.
fn sprite_card() -> AnyPiece {
    let playing = Signal::new(!day::reactive::untrack(day::reduce_motion));
    // The phase as a position that counts on: each pass runs it sixteen phases further from
    // where it was started, so a pause and a resume carry on from the phase shown.
    let position = Animated::new(3.0);
    let run: Rc<RefCell<Option<Playing>>> = Rc::default();
    bind(
        move || playing.get(),
        move |on: &bool| {
            if let Some(old) = run.borrow_mut().take() {
                old.stop();
            }
            if *on {
                let from = position.get_untracked();
                let spec = AnimSpec::linear(SHEET_MS).repeat_forever(false);
                *run.borrow_mut() =
                    Some(Animation::tween(position, from + PHASES as f64, spec).start());
            }
        },
    );

    let sheet: Signal<Option<day::Bitmap>> = Signal::new(None);
    let rows = PHASES.div_ceil(SHEET_COLUMNS);
    let strip = day::Drawing::record(
        Size::new(CELL_SIDE * SHEET_COLUMNS as f64, CELL_SIDE * rows as f64),
        |draw, size| {
            for phase in 0..PHASES {
                moon_phase(draw, sheet_cell(size, phase), phase);
            }
        },
    );
    TaskSlot::new().run(async move {
        sheet.set(strip.to_bitmap(2.0).await.ok());
    });

    card(
        vectors::nav_animation,
        strings::images_frames_title(),
        strings::images_frames_subtitle(),
        column((
            row((
                phase_player(sheet, position)
                    .frame(84.0, 84.0)
                    .background(gallery::INSET)
                    .corner_radius(FRAME_RADIUS)
                    .id(day::id!(images_frames_player)),
                sheet_view(sheet, position)
                    .height(84.0)
                    .grow(Dimension::Width),
            ))
            .spacing(Space::M)
            .align(VAlign::Center)
            .grow(Dimension::Width),
            gallery::caption(strings::images_frames_label(), move || {
                strings::images_frames_count(strings::ImagesFramesCount {
                    n: (phase_index(position.get()) + 1) as i64,
                    total: PHASES as i64,
                })
                .format()
            }),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .trailing(
        row((
            label(strings::images_frames_play())
                .font(Font::Footnote)
                .secondary(),
            toggle(playing).id(day::id!(images_frames_play)),
        ))
        .spacing(Space::S)
        .align(VAlign::Center),
    )
    .id(day::id!(images_frames))
    .build()
}

// ---------------------------------------------------------------------------
// Placeholder
// ---------------------------------------------------------------------------

/// A picture of the Placeholder card: its id in the library, and its name.
type Named = (&'static str, fn() -> day::LocalizedText);

/// The pictures that fade in.
const FADES: [Named; 4] = [
    (sky::WHIRLPOOL, strings::images_name_whirlpool),
    (sky::NEPTUNE, strings::images_name_neptune),
    (sky::PILLARS, strings::images_name_pillars),
    (sky::ANDROMEDA, strings::images_name_andromeda),
];

/// The size each load asks for, in turn: a different address every time, so a load is a
/// download and not an answer from a cache.
const LOAD_SIZES: [sky::Size; 4] = [
    sky::Size::Small,
    sky::Size::Medium,
    sky::Size::Thumb,
    sky::Size::Large,
];

/// The fades the picker offers, in milliseconds.
const FADE_TIMES: [u32; 3] = [0, 400, 1200];

/// A frame showing a picture glyph until `loader` has its picture, which then fades in over
/// the time `fade` holds.
fn fade_tile(loader: &Loader, fade: Signal<u32>, name: day::LocalizedText) -> AnyPiece {
    let opacity = Signal::new(0.0_f64);
    let bitmap = loader.bitmap;
    watch(
        move || bitmap.with(Option::is_some),
        move |ready, _| {
            let ms = fade.get_untracked();
            if !*ready {
                // A reload starts from the placeholder again.
                opacity.set(0.0);
            } else if ms == 0 {
                opacity.set(1.0);
            } else {
                with_animation(AnimSpec::linear(ms), move || opacity.set(1.0));
            }
        },
    );
    tile(
        frame(
            92.0,
            zstack((
                vector(vectors::nav_images)
                    .tint(SystemColor::TertiaryText)
                    .frame(26.0, 26.0),
                shot(bitmap, Mode::Fill)
                    .grow(Dimension::Both)
                    .opacity(opacity),
            ))
            .grow(Dimension::Both),
        ),
        name,
    )
}

/// Placeholder: four remote pictures behind a glyph, fading in over the time the picker
/// chooses, with a button that downloads them again.
fn fade_card() -> AnyPiece {
    let fade = Signal::new(FADE_TIMES[1]);
    let loads = Signal::new(0_usize);
    let loaders: Vec<Loader> = FADES.iter().map(|_| Loader::new()).collect();
    let reload = {
        let loaders = loaders.clone();
        move || {
            let size = LOAD_SIZES[loads.get_untracked() % LOAD_SIZES.len()];
            loads.update(|loads| *loads += 1);
            for (loader, (picture, _)) in loaders.iter().zip(FADES) {
                loader.load(sky::url(picture, size));
            }
        }
    };
    reload();
    let frames: Vec<AnyPiece> = loaders
        .iter()
        .zip(FADES)
        .map(|(loader, (_, name))| fade_tile(loader, fade, name()))
        .collect();
    let seconds = |ms: u32| strings::images_fade_seconds(f64::from(ms) / 1000.0);
    card(
        vectors::nav_images,
        strings::images_fade_title(),
        strings::images_fade_subtitle(),
        column((
            tiles(frames),
            gallery::fields((labeled(
                strings::images_fade_label(),
                picker(
                    fade,
                    [
                        (FADE_TIMES[0], strings::images_fade_off().format()),
                        (FADE_TIMES[1], seconds(FADE_TIMES[1]).format()),
                        (FADE_TIMES[2], seconds(FADE_TIMES[2]).format()),
                    ],
                )
                .segmented()
                .id(day::id!(images_fade_picker)),
            ),)),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .trailing(
        button(strings::images_fade_reload())
            .bordered()
            .compact()
            .action(reload)
            .id(day::id!(images_fade_reload)),
    )
    .id(day::id!(images_fade))
    .build()
}

// ---------------------------------------------------------------------------
// Resample
// ---------------------------------------------------------------------------

/// `bitmap` rendered down to `long_side` pixels on its long side, through an offscreen
/// drawing.
async fn resample(bitmap: &day::Bitmap, long_side: f64) -> Option<day::Bitmap> {
    let pixels = bitmap.info().pixels;
    let longest = pixels.width.max(pixels.height);
    if longest <= 0.0 {
        return None;
    }
    let factor = long_side / longest;
    let size = Size::new(
        (pixels.width * factor).round().max(1.0),
        (pixels.height * factor).round().max(1.0),
    );
    let source = bitmap.clone();
    day::Drawing::record(size, move |draw, size| {
        draw.image(&source, Rect::from_size(size));
    })
    .to_bitmap(1.0)
    .await
    .ok()
}

/// Resample: a picture beside itself rendered down to the slider's count of pixels with
/// `Drawing::to_bitmap`, both shown at the same size.
fn resample_card() -> AnyPiece {
    let bitmap = Loader::loading(sky::url(sky::CARINA, sky::Size::Small)).bitmap;
    let side = Signal::new(20.0_f64);
    let small: Signal<Option<day::Bitmap>> = Signal::new(None);
    let work = TaskSlot::new();
    watch(
        move || (bitmap.get(), side.get().round()),
        move |(source, long_side), _| {
            let Some(source) = source.clone() else {
                small.set(None);
                return;
            };
            let long_side = *long_side;
            work.run(async move {
                small.set(resample(&source, long_side).await);
            });
        },
    );
    let view = |bitmap: Signal<Option<day::Bitmap>>| {
        frame(96.0, shot(bitmap, Mode::Fill).grow(Dimension::Both))
    };
    card(
        vectors::nav_canvas,
        strings::images_blur_title(),
        strings::images_blur_subtitle(),
        column((
            tiles(vec![
                tile(view(bitmap), move || pixel_text(bitmap.get())),
                tile(view(small), move || pixel_text(small.get())),
            ]),
            gallery::fields((labeled(
                strings::images_blur_side(),
                slider(side)
                    .range(4.0..=64.0)
                    .step(1.0)
                    .grow(Dimension::Width)
                    .id(day::id!(images_blur_slider)),
            ),)),
            label(strings::images_name_carina())
                .font(Font::Caption)
                .secondary(),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .id(day::id!(images_blur))
    .build()
}

// ---------------------------------------------------------------------------
// Download
// ---------------------------------------------------------------------------

/// An address nothing answers at: `.invalid` never resolves.
const MISSING_URL: &str = "https://example.invalid/missing.png";

/// Download: one request for a picture of about two megabytes, or for an address that cannot
/// answer, with its progress on a native bar and what it received, decoded or reported.
fn download_card() -> AnyPiece {
    let loader = Loader::new();
    let broken = Signal::new(false);
    let (phase, received, size) = (loader.phase, loader.progress, loader.size);
    let (bitmap, error) = (loader.bitmap, loader.error);
    {
        // The other address starts from an empty card.
        let loader = loader.clone();
        watch(move || broken.get(), move |_, _| loader.reset());
    }

    let toggle = {
        let loader = loader.clone();
        button(move || {
            if phase.get() == Phase::Idle {
                strings::images_fetch_show().format()
            } else {
                strings::images_fetch_hide().format()
            }
        })
        .prominent()
        .action(move || {
            if phase.get_untracked() != Phase::Idle {
                loader.reset();
            } else if broken.get_untracked() {
                loader.load(MISSING_URL.to_string());
            } else {
                loader.load(sky::url(sky::CRAB, sky::Size::Original));
            }
        })
        .id(day::id!(images_fetch_toggle))
    };
    let status = move || match phase.get() {
        Phase::Idle => String::new(),
        Phase::Downloading => strings::images_fetch_downloading().format(),
        Phase::Decoding => strings::images_fetch_decoding().format(),
        Phase::Loaded => strings::images_fetch_ready().format(),
        Phase::Failed => error.get(),
    };
    let status_color = move || match phase.get() {
        Phase::Failed => Ink::from(Tone::Danger.color()),
        Phase::Loaded => Tone::Up.color().into(),
        _ => SystemColor::SecondaryText.into(),
    };
    let none = || strings::images_none().format();

    card(
        vectors::nav_network,
        strings::images_fetch_title(),
        strings::images_fetch_subtitle(),
        column((
            row((
                picker(
                    broken,
                    [
                        (false, strings::images_name_crab()),
                        (true, strings::images_fetch_broken()),
                    ],
                )
                .segmented()
                .grow(Dimension::Width)
                .id(day::id!(images_fetch_source)),
                toggle,
            ))
            .spacing(Space::S)
            .align(VAlign::Center)
            .grow(Dimension::Width),
            frame(
                172.0,
                zstack((
                    vector(vectors::nav_images)
                        .tint(SystemColor::TertiaryText)
                        .frame(30.0, 30.0),
                    shot(bitmap, Mode::Fill).grow(Dimension::Both),
                ))
                .grow(Dimension::Both),
            ),
            progress(move || received.get())
                .grow(Dimension::Width)
                .id(day::id!(images_fetch_progress)),
            row((
                gallery::stat(
                    strings::images_fetch_received(),
                    move || strings::images_percent(received.get()).format(),
                    None,
                ),
                gallery::stat(
                    strings::images_fetch_bytes(),
                    move || match size.get() {
                        0 => none(),
                        bytes => strings::images_kilobytes(bytes as f64 / 1024.0).format(),
                    },
                    None,
                ),
                gallery::stat(
                    strings::images_fetch_pixels(),
                    move || match bitmap.get() {
                        Some(bitmap) => pixel_text(Some(bitmap)),
                        None => none(),
                    },
                    None,
                ),
            ))
            .spacing(Space::M)
            .align(VAlign::Top)
            .grow(Dimension::Width),
            label(status)
                .font(Font::Footnote)
                .color(status_color)
                .min_height(18.0)
                .grow(Dimension::Width)
                .id(day::id!(images_fetch_status)),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .id(day::id!(images_fetch))
    .build()
}

/// The Images page: seven cards on remote and bundled pictures.
pub(crate) fn images_page() -> AnyPiece {
    card_page(
        crate::res::str::nav_images(),
        day::id!(images_title),
        3,
        vec![
            modes_card,
            crops_card,
            fade_card,
            tint_card,
            download_card,
            resample_card,
            sprite_card,
        ],
    )
}
