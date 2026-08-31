use day::model::Op;
use day::prelude::*;

use crate::widgets::heading;

/// The observable model (docs/model.md): a `Store<Keyed<Task>>` drives the recycling list
/// DIRECTLY — the rows bind their fields through the slot, the editor binds the selected row's
/// fields through an `Elem`, and nothing is plumbed between them: editing the title in the form
/// patches the one row label showing it, with no reload, no rebind, and nothing cloned.
///
/// The projection under the list reads exactly what the ORDER depends on (`done`, the filter),
/// so a title keystroke cannot re-run it — and the cost readout at the top makes the
/// observation tables themselves visible: scroll all you like, the counts stay flat.
///
/// The same store is BACKED (docs/persistence.md): natively it belongs to a `ModelContainer`
/// over a real SQLite file, so each edit folds to one statement at the turn's end. Undo inverts
/// that very change log, which is why undoing a delete is one `INSERT` of the row it carried
/// rather than a snapshot restore. (The scene itself is reseeded per launch — see
/// [`open_backing`] for why a demo wants that.)
///
/// `Model` where there is a database, `Observable` where there is not: the web build keeps the
/// rows in memory (rusqlite has no place in a wasm binary) and the page is otherwise identical —
/// undo included, since the stack lives in day-model and a plain store undoes the same way.
#[cfg_attr(not(target_arch = "wasm32"), derive(Model))]
#[cfg_attr(target_arch = "wasm32", derive(Observable))]
#[derive(Clone, Default, PartialEq)]
#[model(table = "tasks")]
pub(crate) struct Task {
    #[model(id)]
    pub id: u32,
    pub title: String,
    pub done: bool,
}

const SEED: u32 = 300;

/// The database the rows live in, under the app's data directory. Named in the readout: a demo
/// that claims persistence should say where it put your rows.
#[cfg(not(target_arch = "wasm32"))]
const DB_FILE: &str = "showcase-model.db";

/// How deep this page's history goes. Generous for a demo — the point is that nothing here is
/// a special case, and a hundred units cost nothing until they exist.
const UNDO_LEVELS: usize = 100;

/// The rows, the history over them, and where they are kept — opened once per process.
struct Backing {
    store: Store<Keyed<Task>>,
    stack: day::model::UndoStack,
    /// The file the rows live in, or `None` when they live in memory: the web build, and a
    /// native open that FAILED — in which case the readout says so rather than naming a file
    /// nothing is being written to.
    file: Option<String>,
}

/// The Model page's backing store — one per APP (docs/state.md): the demo's data, shared by
/// every window the way one document is.
#[derive(Clone)]
struct BackingStore(std::rc::Rc<Backing>);

impl Ambient for BackingStore {
    fn create() -> Self {
        BackingStore(std::rc::Rc::new(open_backing()))
    }
}

/// The seed scene: 300 rows, deterministic, so the walkthrough's counts are arithmetic.
/// Titles are user DATA, not chrome — seeded plain so the asserts hold across locale variants
/// (the Day-Rise seed precedent).
fn seed_rows() -> Vec<Task> {
    (1..=SEED)
        .map(|n| Task {
            id: n,
            title: format!("Task {n}"),
            done: n % 4 == 0,
        })
        .collect()
}

/// A store with the seed already in it and a history that starts empty — the memory arrangement,
/// which the web always takes and a native build falls back to.
fn memory_backing() -> Backing {
    let store = Store::new(Keyed::new(seed_rows()));
    let stack = day::model::UndoStack::new(UNDO_LEVELS);
    stack.watch(store);
    Backing {
        store,
        stack,
        file: None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn open_backing() -> Backing {
    // Seeded on every open, not only an empty one: this is a DEMO, and the page it opens has to
    // be the same page each time — the walkthrough asserts exact titles and counts, and its eight
    // themed/localized variants run one after another against this very file. What the container
    // is here to show is the machinery (one statement per edit, the change log driving undo, the
    // real file behind it), which is unaffected by starting from a known scene. An app keeping
    // the user's work would drop this line and seed only when the table comes up empty.
    let opened = Sqlite::app_data(DB_FILE)
        .and_then(|driver| ModelContainer::open(driver, schema![Task]))
        .map(|container| {
            // Reset the demo file before seeding: the lazy cache upserts what it holds and
            // never infers deletions, so rows a previous run added are cleared explicitly.
            container.with_connection(|conn| {
                let _ = conn.execute("DELETE FROM tasks", &[]);
            });
            let store = container.cache::<Task>();
            store.update("seed", |k| *k = Keyed::new(seed_rows()));
            // AFTER the seed: opening a file for the first time is not an edit the user can undo.
            let stack = container.undo(UNDO_LEVELS);
            Backing {
                store,
                stack,
                file: Some(DB_FILE.to_string()),
            }
        });
    match opened {
        Ok(backing) => backing,
        Err(e) => {
            // A demo with no rows teaches nothing, so this degrades to memory — loudly, and with
            // the readout telling the truth about where the rows now are.
            warn!("model page: {DB_FILE} unavailable ({e}) — keeping rows in memory");
            memory_backing()
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn open_backing() -> Backing {
    memory_backing()
}

fn with_backing<T>(f: impl FnOnce(&Backing) -> T) -> T {
    f(&BackingStore::app().0)
}

fn tasks() -> Store<Keyed<Task>> {
    with_backing(|b| b.store)
}

/// The history over [`tasks`] — the page's buttons and the platform's own Edit menu drive the
/// same one.
fn history() -> day::model::UndoStack {
    with_backing(|b| b.stack.clone())
}

pub(crate) fn model_page() -> AnyPiece {
    let store = tasks();
    // The Edit menu declares MenuRole::Undo/Redo (menus.rs); this is what puts something behind
    // them — the stock item retitles itself "Undo Remove" and ⌘Z lands here (docs/persistence.md).
    day::install_undo(&history());
    let hide_done = Signal::new(false);
    let selected: Signal<Option<u64>> = Signal::new(None);

    // The display projection: KEYS only, undone first, stable by id. Its tracked reads are the
    // collection's shape, each row's `done`, and the filter — a title edit re-runs nothing here.
    let rows = move || {
        let hide = hide_done.get();
        let mut keys: Vec<(u64, bool)> = store
            .keys()
            .into_iter()
            .filter_map(|k| {
                let done = store.elem(k).done().with(|d| d.copied().unwrap_or(false));
                (!hide || !done).then_some((k, done))
            })
            .collect();
        keys.sort_by_key(|(id, done)| (*done, *id));
        keys.into_iter().map(|(k, _)| k).collect::<Vec<u64>>()
    };

    column((
        row((
            heading(crate::res::str::nav_model(), "model-title", None),
            spacer(),
            labeled(
                crate::res::str::model_hide_done(),
                toggle(hide_done).id("model-filter"),
            ),
        ))
        .spacing(8.0),
        // How many rows the projection currently shows.
        label(move || crate::res::str::model_caption(rows().len() as i64).format())
            .tabular()
            .id("model-caption"),
        // The observation tables, live: triggers and interner slots currently held. A coarse
        // read of the store re-renders this on every change; the point of the number is that
        // scrolling a 300-row list does not move it.
        label(move || {
            store.with(|_| {});
            crate::res::str::model_cost(
                day::model::observed_paths() as i64,
                day::model::interned_nodes() as i64,
            )
            .format()
        })
        .font(Font::Footnote)
        .id("model-cost"),
        // Where the rows actually are. The file name is the honest part of a persistence demo:
        // it is openable with any SQLite tool, and it is still there next launch.
        label(move || match with_backing(|b| b.file.clone()) {
            Some(file) => crate::res::str::model_storage_file(file).format(),
            None => crate::res::str::model_storage_memory().format(),
        })
        .font(Font::Footnote)
        .id("model-storage"),
        // The selected row's editor: two controls bound STRAIGHT to the store through the
        // element's field accessors — the mirror of what the row labels read, with no plumbing.
        //
        // Keyed on the row, as a collection of nought-or-one, because the editor IS its row: an
        // `Elem`'s field accessors name one key for the life of the subtree, so the selection
        // moving from one row to the next has to build a new one. A `when` on "something is
        // selected" would not — the condition is still true, so the arm it built for the FIRST
        // selection stays mounted, and every later edit writes to whatever row that was.
        each(
            items(
                move || selected.get().into_iter().collect::<Vec<u64>>(),
                |id: &u64| *id,
            ),
            move |slot: ItemSlot<u64, u64>| {
                let it = tasks().elem(slot.key());
                row((
                    text_field(it.title()).id("model-name"),
                    toggle(it.done()).id("model-done"),
                ))
                .spacing(8.0)
            },
        ),
        // What is selected, read back OUT of the store — asserting this after typing into the
        // field above proves the round trip without reaching into a recycled row.
        label(move || match selected.get() {
            Some(id) => tasks()
                .elem(id)
                .title()
                .with(|t| t.cloned())
                .map(|t| crate::res::str::model_selected(t).format())
                .unwrap_or_else(|| crate::res::str::model_selected_none().format()),
            None => crate::res::str::model_selected_none().format(),
        })
        .font(Font::Footnote)
        .id("model-selected"),
        list(store.rows(rows), |slot: ModelSlot<Task>| {
            row((
                label(move || slot.title().read())
                    .padding(Insets::symmetric(12.0, 8.0))
                    .grow(),
                when(
                    move || slot.done().read(),
                    || label("✓").padding(Insets::symmetric(12.0, 8.0)),
                ),
            ))
            .id_keyed("model-row", slot.item().key())
        })
        .row_height(RowHeight::Uniform(36.0))
        .on_select(move |it: Elem<Task>| selected.set(Some(it.key())))
        // Two-way selection: app-state changes sync back into the native highlight, indices
        // resolved through the same projection the list shows.
        .selected_rows(move || {
            selected
                .get()
                .and_then(|id| rows().iter().position(|k| *k == id))
                .into_iter()
                .collect()
        })
        .id("model-list"),
        row((
            button(crate::res::str::model_add())
                .prominent()
                .tint(crate::widgets::primary())
                .action(move || {
                    let id = store
                        .with_untracked(|k| k.items().iter().map(|t| t.id).max().unwrap_or(0))
                        + 1;
                    store.restructure("add", Op::Insert, id as u64, |v| {
                        v.push(Task {
                            id,
                            title: format!("Task {id}"),
                            done: false,
                        });
                    });
                    selected.set(Some(id as u64));
                })
                .id("model-add"),
            button(crate::res::str::model_delete())
                .bordered()
                .action(move || {
                    if let Some(id) = selected.get_untracked() {
                        store.restructure("remove", Op::Delete, id, |v| {
                            v.remove(id);
                        });
                        selected.set(None);
                    }
                })
                .id("model-delete"),
            // One unit per turn, inverted out of the change log: the deleted row comes back
            // WHOLE — data included — because the log's `Delete` carries it.
            button(crate::res::str::model_undo())
                .bordered()
                .action(move || {
                    history().undo();
                })
                .enabled(move || history().can_undo().get())
                .id("model-undo"),
            button(crate::res::str::model_redo())
                .bordered()
                .action(move || {
                    history().redo();
                })
                .enabled(move || history().can_redo().get())
                .id("model-redo"),
        ))
        .spacing(8.0),
        // What one ⌘Z would take back, in the same words the native Edit menu interpolates into
        // its own item — both read this signal, so they can never disagree.
        label(move || history().undo_label().get())
            .font(Font::Footnote)
            .id("model-undo-label"),
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    .padding(16.0)
    .any()
}
