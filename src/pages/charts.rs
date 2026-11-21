// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The Charts page (day-piece-charts): four charts from an observatory's night log, then the
//! charting piece's own example pages.
//!
//! Each of the four cards is one `chart(..)` of composed marks, with no drawing code of its
//! own: an area under a line with a hatched band and a labelled rule, a ridgeline, a calendar
//! heat map, and a dumbbell chart. Their entrances, replays, transitions and inspection are the
//! chart's (`Entrance`, `Replay`, `Inspect`).
//!
//! The last card mounts the pages of `day-piece-charts-gallery`, the same code the piece's
//! Charts Demo app is a sidebar of, behind a picker. Those pages bring their own strings (a
//! private catalog that follows this app's language).

use std::cell::Cell;
use std::rc::Rc;

use day::prelude::*;
use day_piece_charts::{
    self as ch, AnnotationPosition, BandPadding, Datum, Entrance, Guides, Inspect, Interpolation,
    Replay, Selection, Snap, chart, value,
};
use day_piece_charts_gallery::{ChartPage, gallery};

use crate::gallery::{self as kit, Tone, card};

thread_local! {
    /// The open example, kept for the life of the app, so leaving the section and coming back
    /// returns to the same chart. Global rather than page-scoped for exactly that reason.
    static OPEN: Signal<ChartPage> = Signal::global(ChartPage::Pipeline);
}

/// The Charts page: the header, the four observatory cards in as many columns as the window
/// has room for (two at most), and the example gallery across the full width under them.
pub(crate) fn charts_page() -> AnyPiece {
    let header = kit::page_header(crate::res::str::nav_charts(), day::id!(charts_title));
    let grid = kit::deck(
        2,
        vec![flux_card(), spectra_card(), nights_card(), mirror_card()],
    );
    let content = column((header, grid, examples_card()))
        .spacing(Space::L)
        .align(HAlign::Leading)
        .max_width(2.0 * 560.0);
    scroll(
        column((content,))
            .align(HAlign::Center)
            .grow(Dimension::Width)
            .padding(20.0),
    )
    .any()
}

// ---------------------------------------------------------------------------
// Shared parts
// ---------------------------------------------------------------------------

/// A seeded generator of numbers in `0..1` (mulberry32), so a chart's data is the same on
/// every run and every platform.
struct Seeded(u32);

impl Seeded {
    /// The next number.
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x6D2B_79F5);
        let mut t = self.0;
        t = (t ^ (t >> 15)).wrapping_mul(t | 1);
        t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
        f64::from(t ^ (t >> 14)) / 4_294_967_296.0
    }
}

/// A large figure with its unit and a toned note, on one baseline.
fn figure<M1, M2>(
    value: impl IntoText<M1>,
    unit: day::LocalizedText,
    note: impl IntoText<M2>,
    tone: impl Fn() -> Color + 'static,
) -> impl Piece {
    row((
        label(value)
            .font(Font::Title)
            .weight(FontWeight::Semibold)
            .tabular()
            .single_line(),
        label(unit).font(Font::Body).secondary().single_line(),
        label(note)
            .font(Font::Callout)
            .weight(FontWeight::Medium)
            .color(tone)
            .single_line(),
    ))
    .spacing(6.0)
    .align(VAlign::FirstBaseline)
    .grow(Dimension::Width)
}

/// The body of a card: its pieces in a leading-aligned column.
fn body(pieces: impl PieceSeq) -> impl Piece {
    column(pieces).spacing(Space::M).align(HAlign::Leading)
}

/// The entries of a chart's key, in one row.
fn key(entries: impl PieceSeq) -> impl Piece {
    row(entries).spacing(16.0).align(VAlign::Center)
}

// ---------------------------------------------------------------------------
// 1. Guide star flux: an area, its average, and a hatched window
// ---------------------------------------------------------------------------

/// The minutes, counted back from now, that the cloud took to cross the guide star.
const CLOUD: (f64, f64) = (-6.5, -3.5);

/// Fifteen minutes of guide star flux, a sample a minute from then to now: a slow drift
/// plus noise from `rand`, halved while the cloud passes.
fn flux_data(mut rand: impl FnMut() -> f64) -> Vec<f64> {
    (0..16)
        .map(|sample| {
            let minute = f64::from(sample) - 15.0;
            let drift = 60.0 + 16.0 * (f64::from(sample) / 2.6).sin();
            let veil = if (CLOUD.0..=CLOUD.1).contains(&minute) {
                0.5
            } else {
                1.0
            };
            ((drift + (rand() - 0.5) * 20.0) * veil)
                .round()
                .clamp(4.0, 96.0)
        })
        .collect()
}

/// The latest value of a series and its rounded average.
fn flux_figures(data: &[f64]) -> (i64, i64) {
    let last = data.last().copied().unwrap_or(0.0);
    let average = if data.is_empty() {
        0.0
    } else {
        (data.iter().sum::<f64>() / data.len() as f64).round()
    };
    (last as i64, average as i64)
}

/// "Guide star flux": a gradient area under a line, swept in from the left, with the average
/// as a dashed rule that carries its value in a pill and the cloud's passage as a hatched band.
fn flux_card() -> AnyPiece {
    let mut seeded = Seeded(11);
    let data = Signal::new(flux_data(|| seeded.next()));
    let replay = Replay::new();
    let inspected = Signal::new(None::<Selection>);
    let rolls = Cell::new(0_u32);
    let figures = move || data.with(|data| flux_figures(data));
    let flux = chart(move || {
        let (accent, warn) = (Tone::Accent.color(), Tone::Warn.color());
        let samples = data.get();
        let (_, average) = flux_figures(&samples);
        let minute = |index: usize| value("Minute", index as f64 - 15.0);
        let mut marks = vec![
            ch::band_x(value("Minute", CLOUD.0), value("Minute", CLOUD.1))
                .foreground(warn)
                .opacity(0.5)
                .hatched(5.0)
                .annotation(
                    AnnotationPosition::Top,
                    crate::res::str::charts_flux_cloud().format(),
                )
                .annotation_color(warn)
                .annotation_pill(),
        ];
        for (index, flux) in samples.iter().enumerate() {
            marks.push(
                ch::area(minute(index), value("Flux", *flux))
                    .foreground(accent)
                    .gradient(accent.with_alpha(0.34), accent.with_alpha(0.0))
                    .interpolation(Interpolation::Monotone),
            );
            marks.push(
                ch::line(minute(index), value("Flux", *flux))
                    .foreground(accent)
                    .rounded()
                    .interpolation(Interpolation::Monotone),
            );
        }
        if let Some(latest) = samples.last() {
            marks.push(
                ch::point(minute(samples.len() - 1), value("Flux", *latest))
                    .foreground(accent)
                    .symbol_size(70.0),
            );
        }
        marks.push(
            ch::rule_y(value("Flux", average as f64))
                .foreground(day::resolve(SystemColor::SecondaryText))
                .line_width(1.0)
                .dash([3.0, 4.0])
                .annotation(
                    AnnotationPosition::Leading,
                    crate::res::str::charts_flux_average(average).format(),
                )
                .annotation_pill(),
        );
        marks
    })
    .y_domain(0.0, 100.0)
    .y_tick_count(3)
    .x_tick_count(4)
    .x_format(|at| match at {
        Datum::Number(minute) if *minute >= -0.5 => crate::res::str::charts_flux_now().format(),
        Datum::Number(minute) => crate::res::str::charts_flux_ago(-*minute as i64).format(),
        _ => String::new(),
    })
    .entrance(Entrance::Sweep)
    .animation(AnimSpec::ease_out(1100))
    .replay(replay)
    .interact(Inspect::new(inspected));
    card(
        crate::res::vectors::nav_charts,
        crate::res::str::charts_flux_title(),
        crate::res::str::charts_flux_subtitle(),
        body((
            figure(
                move || {
                    let (last, _) = figures();
                    day::format_decimal(last as f64 * 1.3, 1)
                },
                crate::res::str::charts_flux_unit(),
                move || {
                    let (last, average) = figures();
                    let delta = (last - average).abs();
                    if last >= average {
                        crate::res::str::charts_flux_above(delta)
                    } else {
                        crate::res::str::charts_flux_below(delta)
                    }
                    .format()
                },
                move || {
                    let (last, average) = figures();
                    if last >= average {
                        Tone::Up.color()
                    } else {
                        Tone::Danger.color()
                    }
                },
            ),
            flux.id(day::id!(canvas_area))
                .height(210.0)
                .grow(Dimension::Width)
                .a11y(|a| a.label(crate::res::str::charts_flux_title())),
            key((
                kit::legend(
                    Tone::Accent.color(),
                    crate::res::str::charts_flux_key_flux(),
                ),
                kit::legend(
                    SystemColor::SecondaryText,
                    crate::res::str::charts_flux_key_average(),
                ),
                kit::legend(Tone::Warn.color(), crate::res::str::charts_flux_key_cloud()),
            )),
        )),
    )
    .trailing(
        button(crate::res::str::charts_flux_new())
            .action(move || {
                // A fresh series each press: the seed mixes the clock with a press counter.
                rolls.set(rolls.get().wrapping_add(1));
                let clock = (day::frame::now_secs() * 1000.0) as u32;
                let mut random = Seeded(clock ^ rolls.get().wrapping_mul(0x9E37_79B9));
                data.set(flux_data(|| random.next()));
                replay.play();
            })
            .bordered()
            .id(day::id!(canvas_area_new)),
    )
    .id(day::id!(canvas_card_area))
    .build()
}

// ---------------------------------------------------------------------------
// 2. Starlight by class: a ridgeline
// ---------------------------------------------------------------------------

/// A spectral class: its letter, its surface temperature in kelvin, and the hue of its ridge.
type StarClass = (&'static str, f64, f64);

/// The seven classes of the Harvard sequence, hottest first.
const CLASSES: [StarClass; 7] = [
    ("O", 30_000.0, 232.0),
    ("B", 15_000.0, 214.0),
    ("A", 9_000.0, 196.0),
    ("F", 6_800.0, 62.0),
    ("G", 5_800.0, 44.0),
    ("K", 4_400.0, 24.0),
    ("M", 3_200.0, 6.0),
];

/// The ends of visible light, in nanometers.
const VISIBLE: (f64, f64) = (380.0, 750.0);

/// The color of the ridge with `hue`: deeper on light, brighter on dark. The chart takes
/// colors, so this resolves the pair for the appearance in force; the chart's closure reads it,
/// and so draws again when the appearance changes.
fn ridge_color(hue: f64) -> Color {
    day::resolve(Ink::adaptive(
        Color::hsl(hue, 0.66, 0.42),
        Color::hsl(hue, 0.78, 0.68),
    ))
}

/// What a black body at `kelvin` radiates at each sampled wavelength (nanometers, 100 to
/// 1,150), by Planck's law, scaled so its strongest sample is 1.
fn spectrum(kelvin: f64) -> Vec<(f64, f64)> {
    // hc/k in nanometer kelvins: the exponent of Planck's law is this over (wavelength × T).
    const SECOND_RADIATION: f64 = 1.438_777e7;
    let raw: Vec<(f64, f64)> = (0..=42)
        .map(|step| {
            let nm = 100.0 + 25.0 * f64::from(step);
            let radiance = nm.powi(-5) / ((SECOND_RADIATION / (nm * kelvin)).exp() - 1.0);
            (nm, radiance)
        })
        .collect();
    let peak = raw
        .iter()
        .map(|(_, r)| *r)
        .fold(f64::MIN_POSITIVE, f64::max);
    raw.into_iter().map(|(nm, r)| (nm, r / peak)).collect()
}

/// "Starlight by class": seven blackbody spectra as ridges, each hiding the one behind it,
/// rising from their rows on entrance and on Replay.
fn spectra_card() -> AnyPiece {
    let spectra: Rc<Vec<Vec<(f64, f64)>>> =
        Rc::new(CLASSES.iter().map(|class| spectrum(class.1)).collect());
    let replay = Replay::new();
    let inspected = Signal::new(None::<Selection>);
    let ridges = chart(move || {
        let mut marks = Vec::new();
        for ((class, _, hue), samples) in CLASSES.iter().zip(spectra.iter()) {
            let color = ridge_color(*hue);
            for (nm, flux) in samples {
                let (at, row) = (value("Wavelength", *nm), value("Class", *class));
                marks.push(
                    ch::area(at.clone(), row.clone())
                        .ridge(value("Flux", *flux))
                        .foreground(color)
                        .gradient(color.with_alpha(0.42), color.with_alpha(0.04))
                        .interpolation(Interpolation::Monotone),
                );
                marks.push(
                    ch::line(at, row)
                        .ridge(value("Flux", *flux))
                        .foreground(color)
                        .line_width(1.5)
                        .rounded()
                        .interpolation(Interpolation::Monotone),
                );
            }
        }
        for edge in [VISIBLE.0, VISIBLE.1] {
            marks.push(
                ch::rule_x(value("Wavelength", edge))
                    .foreground(day::resolve(SystemColor::SecondaryText.with_alpha(0.5)))
                    .line_width(1.0)
                    .dash([3.0, 4.0]),
            );
        }
        marks
    })
    .ridge_overlap(1.7)
    .x_domain(100.0, 1150.0)
    .x_tick_count(5)
    .x_format(|at| match at {
        Datum::Number(nm) => day::format_decimal(*nm, 0),
        _ => String::new(),
    })
    .no_grid()
    .replay(replay)
    .interact(
        Inspect::new(inspected)
            .snap(Snap::NearestMark)
            .guides(Guides {
                points: true,
                label: true,
                ..Guides::NONE
            }),
    );
    card(
        crate::res::vectors::star,
        crate::res::str::charts_spectra_title(),
        crate::res::str::charts_spectra_subtitle(),
        body((
            kit::caption(
                crate::res::str::charts_spectra_caption(),
                crate::res::str::charts_spectra_visible(),
            ),
            ridges
                .id(day::id!(canvas_ridge))
                .height(250.0)
                .grow(Dimension::Width)
                .a11y(|a| a.label(crate::res::str::charts_spectra_title())),
        )),
    )
    .trailing(
        button(crate::res::str::charts_replay())
            .action(move || replay.play())
            .bordered()
            .id(day::id!(canvas_ridge_replay)),
    )
    .id(day::id!(canvas_card_ridge))
    .build()
}

// ---------------------------------------------------------------------------
// 3. Clear nights: a calendar heat map
// ---------------------------------------------------------------------------

/// The first week of the season, and how many weeks it ran.
const SEASON: (u32, u32) = (14, 26);

/// The nights the dome stayed shut for wind, as (week of the season, day of the week).
const CLOSED: [(u32, u32); 3] = [(4, 2), (13, 5), (21, 1)];

/// The hours of clear sky each night of the season, a week a row, Monday first: seeded, with
/// the better weather of midsummer in the middle weeks. A closed night has none.
fn night_hours() -> Vec<[f64; 7]> {
    let mut rand = Seeded(29);
    (0..SEASON.1)
        .map(|week| {
            let summer = (f64::from(week) / f64::from(SEASON.1 - 1) * std::f64::consts::PI).sin();
            let mut hours = [0.0; 7];
            for (day, slot) in (0_u32..).zip(&mut hours) {
                let roll = rand.next();
                *slot = if CLOSED.contains(&(week, day)) {
                    0.0
                } else {
                    ((roll * roll * 6.0 + summer * 4.5).min(9.0) * 2.0).round() / 2.0
                };
            }
            hours
        })
        .collect()
}

/// The five colors of a night, from overcast to clear all night.
fn night_levels() -> [Ink; 5] {
    let accent = Tone::Accent.color();
    [
        kit::INSET,
        accent.with_alpha(0.24).into(),
        accent.with_alpha(0.48).into(),
        accent.with_alpha(0.72).into(),
        accent.into(),
    ]
}

/// The names of the week's days, Monday first.
fn day_names() -> [String; 7] {
    [
        crate::res::str::charts_day_mon().format(),
        crate::res::str::charts_day_tue().format(),
        crate::res::str::charts_day_wed().format(),
        crate::res::str::charts_day_thu().format(),
        crate::res::str::charts_day_fri().format(),
        crate::res::str::charts_day_sat().format(),
        crate::res::str::charts_day_sun().format(),
    ]
}

/// "Clear nights": one cell a night, a column a week, colored by the hours of clear sky, with
/// the night under the pointer named in a label box.
fn nights_card() -> AnyPiece {
    let hours = Rc::new(night_hours());
    let nights = hours.len() * 7;
    let usable = hours.iter().flatten().filter(|h| **h >= 4.0).count();
    let share = usable as f64 / nights.max(1) as f64 * 100.0;
    let inspected = Signal::new(None::<Selection>);
    let levels = night_levels();
    let swatch = |color: Ink| rounded_rectangle(2.5).fill(color).frame(9.0, 9.0);
    let grid = chart(move || {
        let (levels, names) = (night_levels(), day_names());
        let mut marks = Vec::new();
        for (week, row) in (0_u32..).zip(hours.iter()) {
            let number = SEASON.0 + week;
            for (day, (clear, name)) in (0_u32..).zip(row.iter().zip(&names)) {
                let closed = CLOSED.contains(&(week, day));
                let fill = if closed {
                    Tone::Danger.color()
                } else {
                    day::resolve(levels[((clear / 9.0 * 4.0).ceil() as usize).min(4)])
                };
                let told = if closed {
                    crate::res::str::charts_nights_cell_closed(name.as_str())
                } else {
                    crate::res::str::charts_nights_cell(crate::res::str::ChartsNightsCell {
                        day: name.as_str(),
                        hours: day::format_decimal(*clear, 1),
                    })
                };
                marks.push(
                    ch::rect(
                        value("Week", number.to_string()),
                        value("Day", name.as_str()),
                    )
                    .foreground(fill)
                    .corner_radius(3.5)
                    .selection_value(value(told.format(), *clear)),
                );
            }
        }
        marks
    })
    .x_band_padding(BandPadding {
        inner: 0.16,
        outer: 0.0,
        align: 0.5,
    })
    .y_band_padding(BandPadding {
        inner: 0.16,
        outer: 0.0,
        align: 0.5,
    })
    .x_format(|at| match at {
        // Every fourth week is numbered; the rest are counted from those.
        Datum::Category(week) if week.parse::<u32>().is_ok_and(|n| n % 4 == 0) => week.clone(),
        _ => String::new(),
    })
    .x_label(crate::res::str::charts_nights_week())
    .no_grid()
    .animate_appearance()
    .interact(Inspect::new(inspected).snap(Snap::Hit).guides(Guides {
        label: true,
        ..Guides::NONE
    }));
    card(
        crate::res::vectors::nav_dates,
        crate::res::str::charts_nights_title(),
        crate::res::str::charts_nights_subtitle(),
        body((
            figure(
                day::format_decimal(share, 0),
                crate::res::str::charts_nights_unit(),
                crate::res::str::charts_nights_closed(CLOSED.len() as i64),
                || Tone::Danger.color(),
            ),
            grid.id(day::id!(canvas_matrix))
                .height(186.0)
                .grow(Dimension::Width)
                .a11y(|a| a.label(crate::res::str::charts_nights_title())),
            row((
                label(crate::res::str::charts_nights_less())
                    .font(Font::Caption)
                    .secondary(),
                row((
                    swatch(levels[0]),
                    swatch(levels[1]),
                    swatch(levels[2]),
                    swatch(levels[3]),
                    swatch(levels[4]),
                ))
                .spacing(6.0)
                .align(VAlign::Center),
                label(crate::res::str::charts_nights_more())
                    .font(Font::Caption)
                    .secondary()
                    .grow(Dimension::Width),
                kit::legend(
                    Tone::Danger.color(),
                    crate::res::str::charts_nights_key_closed(),
                ),
            ))
            .spacing(6.0)
            .align(VAlign::Center)
            .grow(Dimension::Width),
        )),
    )
    .id(day::id!(canvas_card_matrix))
    .build()
}

// ---------------------------------------------------------------------------
// 4. Mirror recoating: a dumbbell chart
// ---------------------------------------------------------------------------

/// A filter band with the telescope's throughput in it before and after the mirror was
/// recoated, in percent.
type Band = (&'static str, f64, f64);

/// The six bands of the survey's filter wheel, bluest first. Protected silver reflects red
/// and infrared light better than the bare aluminum it replaced, and ultraviolet worse.
const BANDS: [Band; 6] = [
    ("u", 61.0, 54.0),
    ("g", 72.0, 86.0),
    ("r", 78.0, 93.0),
    ("i", 80.0, 96.0),
    ("z", 74.0, 94.0),
    ("y", 63.0, 90.0),
];

/// "Mirror recoating": each band's throughput drawn out from its old value to its new one,
/// the new value's dot riding the end of the range.
fn mirror_card() -> AnyPiece {
    let replay = Replay::new();
    let ranges = chart(|| {
        let muted = day::resolve(SystemColor::SecondaryText);
        let mut marks = Vec::new();
        for (band, before, after) in BANDS {
            let lost = after < before;
            let tone = if lost {
                Tone::Danger.color()
            } else {
                Tone::Accent.color()
            };
            let row = || value("Band", band);
            marks.push(
                ch::rule_y(row())
                    .x_range(value("Before", before), value("After", after))
                    .foreground(tone.with_alpha(0.45))
                    .line_width(3.0)
                    .rounded(),
            );
            marks.push(
                ch::point(value("Before", before), row())
                    .foreground(muted)
                    .symbol_size(40.0),
            );
            marks.push(
                ch::point(value("After", after), row())
                    .foreground(tone)
                    .symbol_size(105.0)
                    .annotation(
                        if lost {
                            AnnotationPosition::Leading
                        } else {
                            AnnotationPosition::Trailing
                        },
                        // A true minus sign, the width of the plus.
                        format!("{:+.0}", after - before).replace('-', "−"),
                    )
                    .annotation_color(tone),
            );
        }
        marks
    })
    .x_domain(40.0, 105.0)
    .x_tick_count(4)
    .x_format(|at| match at {
        Datum::Number(percent) => format!("{percent:.0}%"),
        _ => String::new(),
    })
    .replay(replay)
    .animation(AnimSpec::ease_out(900));
    card(
        crate::res::vectors::nav_tweaks,
        crate::res::str::charts_mirror_title(),
        crate::res::str::charts_mirror_subtitle(),
        body((
            ranges
                .id(day::id!(canvas_dumbbell))
                .height(30.0 * BANDS.len() as f64 + 28.0)
                .grow(Dimension::Width)
                .a11y(|a| a.label(crate::res::str::charts_mirror_title())),
            key((
                kit::legend(
                    SystemColor::SecondaryText,
                    crate::res::str::charts_mirror_before(),
                ),
                kit::legend(Tone::Accent.color(), crate::res::str::charts_mirror_after()),
                kit::legend(Tone::Danger.color(), crate::res::str::charts_mirror_lost()),
            )),
        )),
    )
    .trailing(
        button(crate::res::str::charts_replay())
            .action(move || replay.play())
            .bordered()
            .id(day::id!(canvas_dumbbell_replay)),
    )
    .id(day::id!(canvas_card_dumbbell))
    .build()
}

// ---------------------------------------------------------------------------
// 5. The example gallery
// ---------------------------------------------------------------------------

/// The height the open example is given. A chart grows into the room it has, and a scrolling
/// page offers none, so the card sets it.
const EXAMPLE_HEIGHT: f64 = 780.0;

/// The pages of `day-piece-charts-gallery` behind their picker, with the piece's export
/// buttons. Nothing in it is this app's chart code.
fn examples_card() -> AnyPiece {
    let open = OPEN.with(|s| *s);
    card(
        crate::res::vectors::nav_grid,
        crate::res::str::charts_examples_title(),
        crate::res::str::charts_examples_subtitle(),
        gallery(open).height(EXAMPLE_HEIGHT).grow(Dimension::Width),
    )
    .id(day::id!(charts_card_examples))
    .build()
}
