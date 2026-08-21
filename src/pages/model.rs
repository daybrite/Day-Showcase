use std::cell::OnceCell;

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
#[derive(Observable, Clone, Default, PartialEq)]
pub(crate) struct Task {
    #[obs(key)]
    pub id: u32,
    pub title: String,
    pub done: bool,
}

const SEED: u32 = 300;

thread_local! {
    static TASKS: OnceCell<Store<Keyed<Task>>> = const { OnceCell::new() };
}

fn tasks() -> Store<Keyed<Task>> {
    TASKS.with(|c| {
        *c.get_or_init(|| {
            Store::new(Keyed::new(
                (1..=SEED)
                    .map(|n| Task {
                        id: n,
                        // Titles are user DATA, not chrome — seeded plain so walkthrough
                        // asserts hold across locale variants (the Day-Rise seed precedent).
                        title: format!("Task {n}"),
                        done: n % 4 == 0,
                    })
                    .collect(),
            ))
        })
    })
}

pub(crate) fn model_page() -> AnyPiece {
    let store = tasks();
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
        // The selected row's editor: two controls bound STRAIGHT to the store through the
        // element's field accessors — the mirror of what the row labels read, with no plumbing.
        when(
            move || selected.get().is_some(),
            move || {
                let id = selected.get_untracked().unwrap_or(0);
                let it = tasks().elem(id);
                row((
                    text_field(it.title()).id("model-name"),
                    toggle(it.done()).id("model-done"),
                ))
                .spacing(8.0)
                .any()
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
        ))
        .spacing(8.0),
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    .padding(16.0)
    .any()
}
