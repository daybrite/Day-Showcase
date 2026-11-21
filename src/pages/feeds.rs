// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The Feeds page: two lists that take their rows over time (docs/list.md). The timeline is a
//! descent log that follows its newest entry as a button or a timer appends one, while the reader
//! is at its end. The pictures list loads its rows a page at a time as the reader nears its end,
//! downloads a thumbnail per row, and keeps each download tied to the row that asked for it, so a
//! recycled cell never shows another row's picture.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use day::prelude::*;

use crate::gallery::{self, sky};
use crate::res::str as strings;
use crate::widgets::heading;

/// The Feeds page: a native segmented control over the two lists, each filling the page.
pub(crate) fn feeds_page() -> AnyPiece {
    let feed = Signal::new(Feed::Timeline);
    let log = Log::new();
    column((
        heading(strings::nav_feeds(), day::id!(feeds_title)),
        picker(
            feed,
            [
                (Feed::Timeline, strings::feeds_tab_timeline()),
                (Feed::Pictures, strings::feeds_tab_pictures()),
            ],
        )
        .segmented()
        .id(day::id!(feeds_picker)),
        // A feed reads as one column, so on a wide window the list keeps a reading measure.
        column((
            when(move || feed.get() == Feed::Timeline, move || timeline(log))
                .otherwise(pictures)
                .max_width(FEED_MAX_WIDTH)
                .grow(Dimension::Both),
        ))
        .align(HAlign::Center)
        .grow(Dimension::Both),
    ))
    .spacing(Space::M)
    .align(HAlign::Leading)
    .padding(20.0)
    .any()
}

/// The widest a feed grows.
const FEED_MAX_WIDTH: f64 = 720.0;

/// The list the page shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Feed {
    /// The descent log.
    Timeline,
    /// The rows with downloaded thumbnails.
    Pictures,
}

// ---------------------------------------------------------------------------
// Timeline
// ---------------------------------------------------------------------------

/// The lines of the descent log, in order, with whether the lander says the line (Flight says
/// the others). The log repeats from the top once it runs out.
const LINES: [(bool, fn() -> LocalizedText); 12] = [
    (false, strings::feeds_line_go_descent),
    (true, strings::feeds_line_engine),
    (true, strings::feeds_line_throttle),
    (false, strings::feeds_line_guidance),
    (true, strings::feeds_line_radar),
    (false, strings::feeds_line_go_landing),
    (true, strings::feeds_line_pitch),
    (true, strings::feeds_line_boulders),
    (false, strings::feeds_line_fuel),
    (true, strings::feeds_line_dust),
    (true, strings::feeds_line_contact),
    (false, strings::feeds_line_down),
];

/// The entries the timeline opens with.
const SEED_ENTRIES: u64 = 5;

/// The most entries the timeline keeps. The timer can run for as long as the page is open, so
/// the oldest entries leave as new ones arrive.
const KEPT_ENTRIES: usize = 200;

/// Milliseconds between two entries while the timer runs.
const LIVE_INTERVAL_MS: u32 = 1500;

/// Mission seconds between two entries, for the time each entry shows.
const SECONDS_PER_ENTRY: u64 = 7;

/// The timeline's state. It belongs to the page, so the log keeps growing while the other list
/// is on show.
#[derive(Clone, Copy)]
struct Log {
    /// The entries on show, oldest first. An entry is its number, and its speaker, text and
    /// time all follow from it.
    entries: Signal<Vec<u64>>,
    /// How many entries were ever logged, which is also the next entry's number.
    logged: Signal<u64>,
    /// Whether the timer appends entries.
    live: Signal<bool>,
}

impl Log {
    /// A log holding the seed entries, with its timer stopped.
    fn new() -> Self {
        let log = Log {
            entries: Signal::new((0..SEED_ENTRIES).collect()),
            logged: Signal::new(SEED_ENTRIES),
            live: Signal::new(false),
        };
        // The timer: one task per time the switch goes on, stopped when it goes off or the
        // page goes away.
        Effect::new(move || {
            if !log.live.get() {
                return;
            }
            let task = day::task(async move {
                loop {
                    day::sleep(LIVE_INTERVAL_MS).await;
                    log.append();
                }
            });
            day::reactive::on_run_retrack(move || task.abort());
        });
        log
    }

    /// Appends the next entry.
    fn append(self) {
        let next = self.logged.get_untracked();
        self.logged.set(next + 1);
        self.entries.update(|entries| {
            entries.push(next);
            if entries.len() > KEPT_ENTRIES {
                let extra = entries.len() - KEPT_ENTRIES;
                entries.drain(..extra);
            }
        });
    }
}

/// The line entry `entry` carries.
fn line(entry: u64) -> (bool, fn() -> LocalizedText) {
    LINES[(entry % LINES.len() as u64) as usize]
}

/// The mission time of entry `entry`, as minutes and seconds since the descent began. A value,
/// so it reads the same in every locale.
fn mission_time(entry: u64) -> String {
    let seconds = entry * SECONDS_PER_ENTRY;
    format!("T+{:02}:{:02}", seconds / 60, seconds % 60)
}

/// How near the end of the timeline, in points, the reader counts as reading the newest entry:
/// about one entry.
const FOLLOW_THRESHOLD: f64 = 60.0;

/// The timeline: the controls that append entries over the list that follows them.
fn timeline(log: Log) -> AnyPiece {
    let latest = Trigger::new();
    let controls = row((
        button(strings::feeds_add())
            .prominent()
            .icon(Symbol::Add)
            .tint(crate::widgets::primary())
            .action(move || log.append())
            .id(day::id!(feeds_add)),
        labeled(
            strings::feeds_live(),
            toggle(log.live).id(day::id!(feeds_live)),
        ),
        button(strings::feeds_latest())
            .bordered()
            .action(move || latest.notify())
            .id(day::id!(feeds_latest)),
        button(strings::feeds_clear())
            .bordered()
            .action(move || log.entries.set(Vec::new()))
            .id(day::id!(feeds_clear)),
        spacer(),
        label(move || strings::feeds_count(log.logged.get() as i64).format())
            .font(Font::Footnote)
            .secondary()
            .tabular()
            .id(day::id!(feeds_count)),
    ))
    .spacing(Space::M)
    .align(VAlign::Center)
    .fit(RowFit::Wrap { run_spacing: 8.0 })
    .grow(Dimension::Width);
    let entries = log.entries;
    let list = list(
        items(move || entries.get(), |entry: &u64| *entry),
        entry_row,
    )
    .row_height(RowHeight::Automatic)
    .no_separators()
    // A new entry scrolls into view only while the reader is at the end; one who scrolled up
    // to read stays put, and the Latest button brings them back.
    .follow_end(FOLLOW_THRESHOLD)
    .scroll_to_end(latest)
    .header(|| {
        label(strings::feeds_timeline_header())
            .font(Font::Footnote)
            .secondary()
            .id(day::id!(feeds_timeline_header))
            .padding(Insets::symmetric(12.0, 8.0))
    })
    .empty(|| {
        label(strings::feeds_timeline_empty())
            .secondary()
            .align(TextAlign::Center)
            .id(day::id!(feeds_timeline_empty))
            .padding(Space::L)
    })
    .content_padding(Insets {
        top: 0.0,
        leading: 0.0,
        bottom: 12.0,
        trailing: 0.0,
    })
    .id(day::id!(feeds_timeline))
    .grow(Dimension::Both);
    column((controls, list))
        .spacing(Space::M)
        .align(HAlign::Leading)
        .grow(Dimension::Both)
        .any()
}

/// One entry of the timeline: who spoke and when, over what was said. The lander's entries
/// sit at the trailing edge and Flight's at the leading edge, as the two sides of a
/// conversation do. Everything is read through the slot, so a recycled cell follows its entry.
fn entry_row(slot: ItemSlot<u64, u64>) -> impl Piece {
    let from_lander = move || line(slot.get()).0;
    let bubble = move |lander: bool| {
        let speaker = if lander {
            strings::feeds_speaker_lander()
        } else {
            strings::feeds_speaker_flight()
        };
        column((
            label(move || format!("{} · {}", speaker.format(), mission_time(slot.get())))
                .font(Font::Caption)
                .secondary()
                .tabular(),
            label(move || (line(slot.get()).1)().format())
                .id_of(move || day::id!(feeds_entry, slot.key()))
                .padding(Insets::symmetric(12.0, 8.0))
                .background(if lander {
                    gallery::Tone::Accent.tint()
                } else {
                    gallery::INSET
                })
                .corner_radius(Radius::L),
        ))
        .spacing(3.0)
        .align(if lander {
            HAlign::Trailing
        } else {
            HAlign::Leading
        })
        .max_width(BUBBLE_MAX_WIDTH)
    };
    when(from_lander, move || {
        row((spacer(), bubble(true))).grow(Dimension::Width)
    })
    .otherwise(move || row((bubble(false), spacer())).grow(Dimension::Width))
    .padding(Insets::symmetric(12.0, 6.0))
    .grow(Dimension::Width)
}

/// The widest an entry's text runs before it wraps.
const BUBBLE_MAX_WIDTH: f64 = 440.0;

// ---------------------------------------------------------------------------
// Pictures
// ---------------------------------------------------------------------------

/// A row of the pictures list.
struct Picture {
    /// The picture's id in NASA's library.
    id: &'static str,
    /// What the picture shows.
    name: fn() -> LocalizedText,
    /// The spacecraft or mission that took it.
    source: fn() -> LocalizedText,
}

/// The rows: the gallery's pictures, then seven more from the same library. Each address
/// answered 200 when it was added.
const PICTURES: [Picture; 17] = [
    Picture {
        id: sky::SUN,
        name: strings::feeds_sun,
        source: strings::feeds_source_sdo,
    },
    Picture {
        id: "PIA11245",
        name: strings::feeds_mercury,
        source: strings::feeds_source_messenger,
    },
    Picture {
        id: "PIA00104",
        name: strings::feeds_venus,
        source: strings::feeds_source_magellan,
    },
    Picture {
        id: sky::EARTH,
        name: strings::feeds_earth,
        source: strings::feeds_source_suomi,
    },
    Picture {
        id: "as17-148-22727",
        name: strings::feeds_earth_apollo,
        source: strings::feeds_source_apollo17,
    },
    Picture {
        id: sky::MOON,
        name: strings::feeds_moon,
        source: strings::feeds_source_lro,
    },
    Picture {
        id: sky::MARS,
        name: strings::feeds_mars,
        source: strings::feeds_source_viking,
    },
    Picture {
        id: "PIA22946",
        name: strings::feeds_jupiter,
        source: strings::feeds_source_juno,
    },
    Picture {
        id: "PIA02879",
        name: strings::feeds_jupiter_io,
        source: strings::feeds_source_cassini,
    },
    Picture {
        id: sky::SATURN,
        name: strings::feeds_saturn,
        source: strings::feeds_source_cassini,
    },
    Picture {
        id: "PIA00032",
        name: strings::feeds_uranus,
        source: strings::feeds_source_voyager2,
    },
    Picture {
        id: sky::NEPTUNE,
        name: strings::feeds_neptune,
        source: strings::feeds_source_voyager2,
    },
    Picture {
        id: "PIA19952",
        name: strings::feeds_pluto,
        source: strings::feeds_source_new_horizons,
    },
    Picture {
        id: sky::CARINA,
        name: strings::feeds_carina,
        source: strings::feeds_source_webb,
    },
    Picture {
        id: sky::PILLARS,
        name: strings::feeds_pillars,
        source: strings::feeds_source_hubble,
    },
    Picture {
        id: sky::ANDROMEDA,
        name: strings::feeds_andromeda,
        source: strings::feeds_source_galex,
    },
    Picture {
        id: sky::WHIRLPOOL,
        name: strings::feeds_whirlpool,
        source: strings::feeds_source_hubble,
    },
];

/// The side of a row's thumbnail, in points.
const THUMB_SIDE: f64 = 56.0;

thread_local! {
    /// The thumbnails downloaded so far, by row, so a row scrolled back into view shows its
    /// picture without another request.
    static THUMBS: RefCell<HashMap<usize, Arc<Vec<u8>>>> = RefCell::new(HashMap::new());
}

/// How many pictures the list opens with, and how many each later page adds.
const PAGE_SIZE: usize = 6;

/// How long a page takes to arrive, in milliseconds: a stand-in for a server's answer.
const PAGE_DELAY_MS: u32 = 600;

/// The pictures list: a recycling list whose rows each download a thumbnail, and which asks for
/// the next page of rows when the reader comes within a screen of its end.
fn pictures() -> AnyPiece {
    let shown = Signal::new(PAGE_SIZE.min(PICTURES.len()));
    let loading = Signal::new(false);
    // One request per time `loading` goes on, stopped if the page goes away first.
    Effect::new(move || {
        if !loading.get() {
            return;
        }
        let task = day::task(async move {
            day::sleep(PAGE_DELAY_MS).await;
            shown.update(|n| *n = (*n + PAGE_SIZE).min(PICTURES.len()));
            loading.set(false);
        });
        day::reactive::on_run_retrack(move || task.abort());
    });
    let load_more = move || {
        if !loading.get_untracked() && shown.get_untracked() < PICTURES.len() {
            loading.set(true);
        }
    };
    list(
        items(move || (0..shown.get()).collect::<Vec<_>>(), |index| *index),
        picture_row,
    )
    .row_height(RowHeight::Uniform(THUMB_SIDE + 16.0))
    .on_end_reached(400.0, load_more)
    .footer(move || {
        when(
            move || shown.get() < PICTURES.len(),
            || spinner().id(day::id!(feeds_loading)),
        )
        .otherwise(|| {
            label(strings::feeds_pictures_end())
                .font(Font::Footnote)
                .secondary()
                .id(day::id!(feeds_pictures_end))
        })
        .padding(Space::L)
    })
    .id(day::id!(feeds_pictures))
    .grow(Dimension::Both)
    .any()
}

/// One row of the pictures list, as docs/list.md "Asynchronous row images" asks. The image
/// view stays mounted and shows what `bytes` holds. Each time the cell is bound to a row, the
/// effect clears the previous row's pixels and starts that row's request; the rebind and the
/// row's disposal both stop the request before it. A finished request checks that the cell
/// still shows the row it was started for before it hands over the picture.
fn picture_row(slot: ItemSlot<usize, usize>) -> impl Piece {
    let bytes: Signal<Option<Arc<Vec<u8>>>> = Signal::new(None);
    Effect::new(move || {
        let index = slot.key();
        let kept = THUMBS.with_borrow(|thumbs| thumbs.get(&index).cloned());
        bytes.set(kept.clone());
        if kept.is_some() {
            return;
        }
        let url = sky::url(PICTURES[index].id, sky::Size::Thumb);
        let task = day::task(async move {
            let Ok(response) = day_part_http::fetch_future(day_part_http::Request::get(url)).await
            else {
                return;
            };
            if !(200..300).contains(&response.status) {
                return;
            }
            let picture = Arc::new(response.body);
            THUMBS.with_borrow_mut(|thumbs| thumbs.insert(index, picture.clone()));
            if day::reactive::untrack(|| slot.key()) == index {
                bytes.set(Some(picture));
            }
        });
        day::reactive::on_run_retrack(move || task.abort());
    });
    row((
        // Both axes are fixed, so the row keeps its size while the picture loads.
        async_image(bytes)
            .rounded(8.0)
            .placeholder_color(gallery::INSET)
            .frame(THUMB_SIDE, THUMB_SIDE),
        column((
            label(move || (PICTURES[slot.key()].name)().format())
                .id_of(move || day::id!(feeds_picture, slot.key())),
            label(move || (PICTURES[slot.key()].source)().format())
                .font(Font::Footnote)
                .secondary(),
        ))
        .spacing(2.0)
        .align(HAlign::Leading)
        .grow(Dimension::Width),
    ))
    .spacing(Space::M)
    .align(VAlign::Center)
    .padding(Insets::symmetric(12.0, 0.0))
    .grow(Dimension::Width)
}
