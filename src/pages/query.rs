//! Live queries (docs/persistence.md): ten thousand rows behind a typed query the ENGINE
//! answers — the query holds ids, and the list faults in only the rows it shows. The search
//! term and the star filter drive `query_fn`; edits to rows flow through the change log, a
//! change no predicate reads costs nothing, and the list receives row deltas it can animate
//! instead of reloads. The residency readout shows the working set staying small under the
//! ten-thousand-row table. The web build keeps the same page over an in-memory projection.

use day::model::Op;
use day::prelude::*;
use day_piece_searchfield::search_field;

use crate::widgets::heading;

#[derive(Clone, Default, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Model))]
#[cfg_attr(target_arch = "wasm32", derive(Observable))]
#[model(table = "tracks", fts("title"), spatial(lat = "lat", lon = "lon"))]
pub(crate) struct Track {
    #[model(id)]
    pub id: u32,
    pub title: String,
    pub plays: i64,
    pub starred: bool,
    pub lat: f64,
    pub lon: f64,
}

pub(crate) const TOTAL: u32 = 10_000;

const ADJ: [&str; 8] = [
    "Silent",
    "Golden",
    "Electric",
    "Wandering",
    "Crimson",
    "Hollow",
    "Northern",
    "Paper",
];
const NOUN: [&str; 8] = [
    "River",
    "Harbor",
    "Skyline",
    "Meadow",
    "Signal",
    "Harbor Light",
    "Orchard",
    "Milestone",
];

fn seed() -> Keyed<Track> {
    Keyed::new(
        (1..=TOTAL)
            .map(|i| Track {
                id: i,
                // Deterministic titles, so walkthrough counts are arithmetic: "0042" appears
                // in exactly one title.
                title: format!(
                    "{} {} {:04}",
                    ADJ[(i % 8) as usize],
                    NOUN[((i / 8) % 8) as usize],
                    i
                ),
                plays: ((i as i64) * 37) % 1000,
                starred: i % 7 == 0,
                // A deterministic grid of pins, so viewport counts are arithmetic.
                lat: (i % 100) as f64,
                lon: ((i / 100) % 100) as f64,
            })
            .collect(),
    )
}

// Native: a real container (in-memory engine — the Model page shows the file) and a live
// query. Web: the same store shape, filtered by a plain projection.
#[cfg(not(target_arch = "wasm32"))]
mod engine {
    use super::{Track, seed};
    use day::prelude::*;
    use std::cell::OnceCell;

    thread_local! {
        static CONTAINER: OnceCell<ModelContainer> = const { OnceCell::new() };
    }

    pub(super) fn container() -> ModelContainer {
        CONTAINER.with(|c| {
            c.get_or_init(|| {
                let container = ModelContainer::open(Sqlite::memory(), schema![Track])
                    .unwrap_or_else(|e| {
                        // A memory database failing to open leaves nothing to demo; surface
                        // the reason in the one place a demo can.
                        panic!("query page container: {e}")
                    });
                container.cache::<Track>().update("seed", |k| *k = seed());
                let _ = container.save();
                // The page's point: the table stays in SQLite and memory holds a working
                // set. Cap the cache well under the row count so faulting is visible.
                container.set_cache_limit(2_048);
                container
            })
            .clone()
        })
    }

    pub(super) fn store() -> Store<Keyed<Track>> {
        container().cache::<Track>()
    }

    pub(super) fn query(
        term: Signal<String>,
        starred: Signal<bool>,
        fts: Signal<bool>,
        viewport: Signal<bool>,
        lat_min: Signal<f64>,
    ) -> day::persistence::Query<Track> {
        container().query_fn::<Track>(move || {
            let mut f = day::persistence::Fetch::new().sort(Track::id().asc());
            let t = term.get();
            if !t.is_empty() {
                // The same term through either engine: substring in memory, or FTS5 MATCH
                // through the generated shadow index.
                if fts.get() {
                    f = f.filter(Track::fts().matches(t));
                } else {
                    f = f.filter(Track::title().contains_ci(t));
                }
            }
            if starred.get() {
                f = f.filter(Track::starred().eq(true));
            }
            if viewport.get() {
                let m = lat_min.get();
                f = f.filter(Track::geo().within(day::persistence::GeoRect {
                    min_lat: m,
                    max_lat: m + 15.0,
                    min_lon: 0.0,
                    max_lon: 100.0,
                }));
            }
            f
        })
    }

    /// How many rows are resident right now — the working set behind the readout.
    pub(super) fn resident() -> usize {
        container().cache::<Track>().with_untracked(|k| k.len())
    }
}

#[cfg(target_arch = "wasm32")]
mod engine {
    use super::{Track, seed};
    use day::prelude::*;
    use std::cell::OnceCell;

    thread_local! {
        static TRACKS: OnceCell<Store<Keyed<Track>>> = const { OnceCell::new() };
    }

    pub(super) fn store() -> Store<Keyed<Track>> {
        TRACKS.with(|c| *c.get_or_init(|| Store::new(seed())))
    }
}

pub(crate) fn query_page() -> AnyPiece {
    let term = Signal::new(String::new());
    let starred = Signal::new(false);
    // The FTS/viewport controls exist only where the SQL engine drives the page.
    #[cfg(not(target_arch = "wasm32"))]
    let fts = Signal::new(false);
    #[cfg(not(target_arch = "wasm32"))]
    let viewport = Signal::new(false);
    #[cfg(not(target_arch = "wasm32"))]
    let lat_min = Signal::new(20.0f64);
    let selected: Signal<Option<u64>> = Signal::new(None);
    let store = engine::store();

    #[cfg(not(target_arch = "wasm32"))]
    let q = engine::query(term, starred, fts, viewport, lat_min);
    #[cfg(not(target_arch = "wasm32"))]
    let count = {
        let q = q.clone();
        move || q.count()
    };
    #[cfg(target_arch = "wasm32")]
    let ids = move || {
        let t = term.get().to_lowercase();
        let only = starred.get();
        let mut ids: Vec<u64> = store
            .keys()
            .into_iter()
            .filter(|k| {
                store.elem(*k).with(|track| {
                    track.is_some_and(|track| {
                        (t.is_empty() || track.title.to_lowercase().contains(&t))
                            && (!only || track.starred)
                    })
                })
            })
            .collect();
        ids.sort_unstable();
        ids
    };
    #[cfg(target_arch = "wasm32")]
    let count = move || ids().len();

    let row_view = |slot: ModelSlot<Track>| {
        row((
            label(move || slot.title().read())
                .padding(Insets::symmetric(12.0, 6.0))
                .grow(),
            when(
                move || slot.starred().read(),
                || label("★").padding(Insets::symmetric(12.0, 6.0)),
            ),
        ))
        .id_of(move || format!("query-row:{}", slot.item().key()))
    };
    #[cfg(not(target_arch = "wasm32"))]
    let track_list = list(q.clone(), row_view)
        .row_height(RowHeight::Uniform(32.0))
        .on_select(move |it: Elem<Track>| selected.set(Some(it.key())))
        .any();
    #[cfg(target_arch = "wasm32")]
    let track_list = list(store.rows(ids), row_view)
        .row_height(RowHeight::Uniform(32.0))
        .on_select(move |it: Elem<Track>| selected.set(Some(it.key())))
        .any();
    // One textual id for both engines (day lint counts the literal).
    let track_list = track_list.id("query-list");

    column((
        heading(crate::res::str::nav_query(), "query-title", None),
        search_field(term).id("query-search"),
        // Stacked, not a three-up row: phone widths clip it.
        labeled(
            crate::res::str::query_starred(),
            toggle(starred).id("query-filter"),
        ),
        {
            #[cfg(not(target_arch = "wasm32"))]
            {
                labeled(crate::res::str::query_fts(), toggle(fts).id("query-fts"))
            }
            #[cfg(target_arch = "wasm32")]
            {
                spacer().any()
            }
        },
        {
            #[cfg(not(target_arch = "wasm32"))]
            {
                column((
                    labeled(
                        crate::res::str::query_viewport(),
                        toggle(viewport).id("query-viewport"),
                    ),
                    slider(lat_min).range(0.0..=85.0).id("query-lat"),
                    label(move || {
                        crate::res::str::query_viewport_box(
                            lat_min.get() as i64,
                            (lat_min.get() + 15.0) as i64,
                        )
                        .format()
                    })
                    .tabular()
                    .font(Font::Footnote)
                    .id("query-viewport-box"),
                ))
                .spacing(8.0)
                .any()
            }
            #[cfg(target_arch = "wasm32")]
            {
                spacer().any()
            }
        },
        label(move || crate::res::str::query_caption(count() as i64, TOTAL as i64).format())
            .tabular()
            .id("query-caption"),
        {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let q = q.clone();
                label(move || {
                    let _ = q.count(); // re-render alongside the set
                    crate::res::str::query_resident(engine::resident() as i64, TOTAL as i64)
                        .format()
                })
                .font(Font::Footnote)
                .id("query-evals")
                .any()
            }
            #[cfg(target_arch = "wasm32")]
            {
                spacer().any()
            }
        },
        label(move || match selected.get() {
            Some(id) => engine::store()
                .elem(id)
                .title()
                .with(|t| t.cloned())
                .map(|t| crate::res::str::query_selected(t).format())
                .unwrap_or_else(|| crate::res::str::query_selected_none().format()),
            None => crate::res::str::query_selected_none().format(),
        })
        .font(Font::Footnote)
        .id("query-selected"),
        track_list,
        button(crate::res::str::query_star())
            .bordered()
            .action(move || {
                if let Some(id) = selected.get_untracked() {
                    let s = store.elem(id).starred();
                    s.write(!s.peek());
                }
            })
            .id("query-star"),
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    .padding(16.0)
    .any()
}

// Keep the wasm build honest about the one API difference.
#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
fn _unused(_: Op) {}
#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
fn _unused(_: Op) {}
