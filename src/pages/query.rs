//! Worker-backed queries: SQLite filtering, FTS, seeding and materialization run off the UI
//! thread. The UI receives owned rows and preserves stable list identities. The full result
//! remains scrollable; the web build uses the same page with an in-memory projection.

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

// Native: a real container (in-memory engine; the Model page shows the file) and a live
// query. Web: the same store shape, filtered by a plain projection.
#[cfg(not(target_arch = "wasm32"))]
mod engine {
    use super::{Track, TrackFields, seed};
    use day::persistence::{DatabaseWorker, DbError, Fetch};
    use day::prelude::*;
    use std::{cell::RefCell, rc::Rc};

    #[derive(Clone)]
    struct Db {
        worker: Rc<RefCell<Option<DatabaseWorker>>>,
        ready: Signal<bool>,
    }
    impl Ambient for Db {
        fn create() -> Self {
            let db = Self {
                worker: Rc::new(RefCell::new(None)),
                ready: Signal::new(false),
            };
            let target = db.clone();
            day::task(async move {
                let result = DatabaseWorker::open(|| {
                    let db = ModelContainer::open(Sqlite::memory(), schema![Track])?;
                    db.cache::<Track>().update("seed", |rows| *rows = seed());
                    db.save()?;
                    db.set_cache_limit(2_048);
                    Ok(db)
                })
                .await;
                match result {
                    Ok(worker) => {
                        *target.worker.borrow_mut() = Some(worker);
                        target.ready.set(true);
                    }
                    Err(error) => day::log::error!("query worker open: {error}"),
                }
            });
            db
        }
    }
    #[derive(Clone, Copy)]
    pub(super) struct QueryView {
        pub store: Store<Keyed<Track>>,
        ids: Signal<Vec<u64>>,
        count: Signal<usize>,
        pub resident: Signal<usize>,
    }
    impl QueryView {
        pub fn count(self) -> usize {
            self.count.get()
        }
        pub fn ids(self) -> Vec<u64> {
            self.ids.get()
        }
    }
    pub(super) fn query(
        term: Signal<String>,
        starred: Signal<bool>,
        fts: Signal<bool>,
        viewport: Signal<bool>,
        lat_min: Signal<f64>,
    ) -> QueryView {
        let db = Db::app();
        let view = QueryView {
            store: Store::new(Keyed::new(vec![])),
            ids: Signal::new(vec![]),
            count: Signal::new(0),
            resident: Signal::new(0),
        };
        Effect::new(move || {
            if !db.ready.get() {
                return;
            }
            let Some(worker) = db.worker.borrow().clone() else {
                return;
            };
            let mut fetch = Fetch::new().sort(Track::id().asc());
            let text = term.get();
            if !text.is_empty() {
                fetch = fetch.filter(if fts.get() {
                    Track::fts().matches(text)
                } else {
                    Track::title().contains_ci(text)
                });
            }
            if starred.get() {
                fetch = fetch.filter(Track::starred().eq(true));
            }
            if viewport.get() {
                let min = lat_min.get();
                fetch = fetch.filter(Track::geo().within(day::persistence::GeoRect {
                    min_lat: min,
                    max_lat: min + 15.0,
                    min_lon: 0.0,
                    max_lon: 100.0,
                }));
            }
            let task = day::task(async move {
                let result = worker
                    .observe(move |db| {
                        let count = db
                            .query::<Track>()
                            .filter(fetch.pred.clone())
                            .live_count()
                            .try_get()?;
                        let rows = db
                            .query::<Track>()
                            .filter(fetch.pred.clone())
                            .sort(Track::id().asc())
                            .live()
                            .try_collect()?;
                        let resident = db.cache::<Track>().with_untracked(|rows| rows.len());
                        Ok((rows, count, resident))
                    })
                    .await;
                let mut sub = match result {
                    Ok(sub) => sub,
                    Err(e) => {
                        report(e);
                        return;
                    }
                };
                while let Some(result) = sub.next().await {
                    match result {
                        Ok(result) => {
                            let (rows, count, resident) = result.value;
                            day::reactive::batch(|| {
                                view.ids.set(rows.iter().map(|r| u64::from(r.id)).collect());
                                view.store
                                    .update("snapshot", |store| *store = Keyed::new(rows));
                                view.count.set(count);
                                view.resident.set(resident);
                            });
                        }
                        Err(e) => report(e),
                    }
                }
            });
            day::reactive::on_run_retrack(move || task.abort());
        });
        view
    }
    fn report(error: DbError) {
        day::log::error!("query worker: {error}");
    }
    pub(super) fn toggle(id: u64) {
        let Some(worker) = Db::app().worker.borrow().clone() else {
            return;
        };
        let request = worker.write(move |db| {
            if let Some(row) = db.try_get::<Track>(id as u32)? {
                row.starred().write(!row.starred().peek());
            }
            Ok(())
        });
        day::task(async move {
            if let Err(e) = request.await {
                report(e);
            }
        });
    }
}

#[cfg(target_arch = "wasm32")]
mod engine {
    use super::{Track, seed};
    use day::prelude::*;

    /// The demo's track store, one per app (docs/state.md), like the container above.
    #[derive(Clone, Copy)]
    struct Tracks(Store<Keyed<Track>>);

    impl Ambient for Tracks {
        fn create() -> Self {
            Tracks(Store::new(seed()))
        }
    }

    pub(super) fn store() -> Store<Keyed<Track>> {
        Tracks::app().0
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
    #[cfg(target_arch = "wasm32")]
    let store = engine::store();

    #[cfg(not(target_arch = "wasm32"))]
    let q = engine::query(term, starred, fts, viewport, lat_min);
    #[cfg(not(target_arch = "wasm32"))]
    let store = q.store;
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
    let track_list = list(store.rows(move || q.ids()), row_view)
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
        heading(crate::res::str::nav_query(), "query-title"),
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
                    crate::res::str::query_resident(q.resident.get() as i64, TOTAL as i64).format()
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
            Some(id) => store
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
                    #[cfg(not(target_arch = "wasm32"))]
                    engine::toggle(id);
                    #[cfg(target_arch = "wasm32")]
                    {
                        let s = store.elem(id).starred();
                        s.write(!s.peek());
                    }
                }
            })
            .id("query-star"),
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    .padding(16.0)
    .any()
}

// Reference `Op` under both cfgs, so the wasm build also compiles the one API difference.
#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
fn _unused(_: Op) {}
#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
fn _unused(_: Op) {}
