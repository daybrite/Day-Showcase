use std::collections::BTreeSet;

use day::prelude::*;
use day_piece_pullrefresh::pull_to_refresh;

use crate::widgets::heading;

/// A native recycling list (docs/list.md): 500 rows, but only the visible cells are ever built;
/// the platform's NSTableView / RecyclerView / GtkListView / QListView / ItemsRepeater owns reuse.
/// The list is wrapped in `pull_to_refresh` (day-piece-pullrefresh), the recycling-list example:
/// a pull (or dayscript `toggle: {id: list-refresh}`) adds 100 rows, same as the button.
///
/// Selection (docs/list.md): the rows are multi-selectable where the toolkit supports it, the
/// full selection lives in an app signal summarized above the list (with ranges compressed,
/// "4-10"), and Clear Selection syncs an empty selection back into the native list.
/// The row "Scroll to item 100" targets: a row number, which is also its key. The button
/// resolves it to a position at press time, so the jump follows the row through a shuffle.
const TARGET_ROW: i64 = 100;

pub(crate) fn list_page() -> AnyPiece {
    // A real Vec of row numbers (not a derived range): drag-to-reorder rotates it, shuffle
    // permutes it, and the refresh paths append to it; the order is app-owned state.
    let rows: Signal<Vec<i64>> = Signal::new((1..=500).collect());
    let refreshing = Signal::new(false);
    // Programmatic scrolling (docs/list.md): a row-jump signal + an end trigger drive the
    // native list, the row rail's counterpart to `scroll(...).scroll_target(...)`.
    let jump_row: Signal<Option<usize>> = Signal::new(None);
    let jump_end = Trigger::new();
    // The selected row numbers (1-based, matching the row labels), fed from the native list's
    // selection reports; single-selection toolkits contribute one-element sets.
    let activated = Signal::new(0i64);
    let selected: Signal<BTreeSet<i64>> = Signal::new(BTreeSet::new());
    // The one reload path for every begin (pull, toggle, programmatic): a timed task on the
    // main-loop executor (`day::sleep`, docs/async.md) stands in for the network: the same
    // code on every platform, including the single-threaded web backend.
    // The target row total. The async refresh below bumps it through a `Setter` (Send), and this
    // watch appends the new rows, reorder-safe because appending never disturbs the order.
    let total = Signal::new(500i64);
    watch(
        move || total.get(),
        move |t, _| {
            rows.update(|v| {
                let next = v.iter().copied().max().unwrap_or(0);
                if *t > next {
                    v.extend(next + 1..=*t);
                }
            });
        },
    );
    watch(
        move || refreshing.get(),
        move |now, _| {
            if *now {
                let next = total.get_untracked() + 100;
                let grow = total.setter();
                let done = refreshing.setter();
                day::task(async move {
                    day::sleep(900).await;
                    grow.set(next);
                    done.set(false);
                });
            }
        },
    );
    column((
        row((
            heading(crate::res::str::nav_list(), day::id!(list_title)),
            spacer(),
            button(crate::res::str::list_clear_selection())
                .bordered()
                .action(move || selected.set(BTreeSet::new()))
                .id(day::id!(list_clear)),
            button(crate::res::str::list_add())
                .prominent()
                .action(move || total.update(|c| *c += 100))
                .tint(crate::widgets::primary())
                .id(day::id!(list_add)),
        ))
        .spacing(Space::S),
        label(move || crate::res::str::list_caption(rows.get().len() as i64).format())
            .tabular()
            .id(day::id!(list_caption)),
        // The tier this backend realizes for the pull (docs/pullrefresh.md): native on the
        // phones and HarmonyOS, an elastic overshoot on the desktops, visible in the CI captures.
        label(match day_piece_pullrefresh::support() {
            Support::Native => crate::res::str::refresh_tier_native(),
            _ => crate::res::str::refresh_tier_emulated(),
        })
        .font(Font::Footnote)
        .id(day::id!(list_tier)),
        // Programmatic scrolling + order controls, merged from the old Scrolling page: the
        // buttons drive the recycling list (scroll-to-row realizes virtualized rows), and
        // Shuffle/Reset permute the backing Vec, animated as native row moves where the
        // toolkit supports it (docs/list.md). Wrapped, because five buttons overflow a
        // phone-portrait window: a plain row left Shuffle and Reset offscreen and untappable
        // (docs/size-classes.md "Row fit policies").
        row((
            button(crate::res::str::scroll_to_top())
                .bordered()
                .action(move || jump_row.set(Some(0)))
                .id(day::id!(list_scroll_top)),
            button(crate::res::str::scroll_to_item())
                .bordered()
                // Find the row by identity, not by position. The list's keys are the row
                // numbers, but `scroll_to_row` addresses a position, and shuffle and
                // drag-to-reorder move row 100 elsewhere, so a hardcoded index 99 scrolls to
                // whatever row happens to have landed there. Selecting it too makes the landing
                // unambiguous once the row is no longer where its number suggests.
                .action(move || {
                    if let Some(at) = rows.get_untracked().iter().position(|n| *n == TARGET_ROW) {
                        jump_row.set(Some(at));
                        selected.set(BTreeSet::from([TARGET_ROW]));
                    }
                })
                .id(day::id!(list_scroll_item)),
            button(crate::res::str::scroll_to_bottom())
                .bordered()
                .action(move || jump_end.notify())
                .id(day::id!(list_scroll_bottom)),
            button(crate::res::str::list_shuffle())
                .action(move || rows.update(|v| shuffle(v)))
                .tint(crate::widgets::secondary())
                .id(day::id!(list_shuffle)),
            // Reset rebuilds the full set rather than sorting in place: rows can now be
            // swiped away, and a sort would faithfully restore the order of whatever survived
            // while leaving the deleted ones gone.
            button(crate::res::str::list_reset())
                .bordered()
                .action(move || rows.set((1..=total.get_untracked()).collect()))
                .id(day::id!(list_reset)),
        ))
        .spacing(Space::S)
        .fit(RowFit::Wrap { run_spacing: 8.0 }),
        // Drag-to-reorder (docs/list.md): the hint names the pinned-row guard, and the order
        // readout makes the committed order assertable (dayscript `reorder` steps check it).
        label(crate::res::str::list_reorder_hint())
            .font(Font::Footnote)
            .id(day::id!(list_reorder_hint)),
        label(move || {
            let first: Vec<String> = rows.get().iter().take(5).map(|n| n.to_string()).collect();
            crate::res::str::list_order(first.join(",")).format()
        })
        .font(Font::Footnote)
        .id(day::id!(list_order)),
        // The live selection summary: pluralized per locale, runs compressed ("4-10").
        label(move || {
            let sel = selected.get();
            if sel.is_empty() {
                crate::res::str::list_selection_none().format()
            } else {
                crate::res::str::list_selection(crate::res::str::ListSelection {
                    count: sel.len() as i64,
                    rows: format_rows(&sel),
                })
                .format()
            }
        })
        .font(Font::Footnote)
        .id(day::id!(list_selection)),
        label(move || crate::res::str::list_activated(activated.get()).format())
            .font(Font::Footnote)
            .id(day::id!(list_activated)),
        pull_to_refresh(
            refreshing,
            list(
                items(move || rows.get(), |n: &i64| *n),
                |row: ItemSlot<i64, i64>| {
                    // The target row wears the warm accent so "Scroll to item 100" visibly
                    // lands on it wherever a shuffle has moved it; the rest keep the
                    // theme-neutral slate (reactive, so recycled cells restyle as they rebind).
                    label(move || crate::res::str::list_row(row.get()).format())
                        .color(move || {
                            if row.get() == TARGET_ROW {
                                crate::palette::CORAL
                            } else {
                                crate::palette::SLATE
                            }
                        })
                        // A physical row keeps its subtree across recycling; its test/a11y
                        // identity must follow the slot just as its text and color do.
                        .id_of(move || day::id!(list_row, row.key()))
                        .padding(Insets::symmetric(12.0, 8.0))
                },
            )
            .row_height(RowHeight::Uniform(36.0))
            .multi_select()
            // Keys are the row numbers, so the selection set is the report itself.
            .on_activate(move |key| activated.set(key))
            .on_selection(move |keys: Vec<i64>| selected.set(keys.into_iter().collect()))
            // Two-way: app-state changes (Clear Selection) sync into the native list;
            // indices are the rows' current positions (reorder can move them).
            .selected_rows(move || {
                let v = rows.get();
                selected
                    .get()
                    .iter()
                    .filter_map(|n| v.iter().position(|r| r == n))
                    .collect()
            })
            .scroll_to_row(jump_row)
            .scroll_to_end(jump_end)
            // Native drag-to-reorder, with a guard demo: the first row is pinned: it cannot be
            // dragged, and a drop aimed at its slot lands just below it instead (Retarget).
            .reorderable(
                Reorder::new(move |from, to| {
                    rows.update(|v| {
                        let it = v.remove(from);
                        v.insert(to, it);
                    });
                })
                .guard(|from, to| {
                    if from == 0 {
                        ReorderVerdict::Deny
                    } else if to == 0 {
                        ReorderVerdict::Retarget(1)
                    } else {
                        ReorderVerdict::Allow
                    }
                }),
            )
            // Leading swipe opens the same row as activation. WinUI also offers these actions
            // in a native context menu for mouse and keyboard users.
            .swipe_leading(move |index| {
                let key = rows.get_untracked()[index];
                vec![
                    swipe_action(crate::res::str::menu_open())
                        .symbol(Symbol::Open)
                        .action(move || activated.set(key)),
                ]
            })
            // Native swipe-to-delete where supported (including WinUI). The same first row is
            // pinned as for reorder, so one demo shows both guards agreeing about it.
            .deletable(
                Delete::new(move |index| {
                    rows.update(|v| {
                        if index < v.len() {
                            v.remove(index);
                        }
                    });
                })
                .label(crate::res::str::list_delete())
                .guard(|index| index != 0),
            )
            .id(day::id!(demo_list)),
        )
        .id(day::id!(list_refresh)),
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    .padding(Space::L)
    .any()
}

/// Fisher–Yates over a hand-rolled xorshift64 (no rand dependency; the seed is fixed and
/// advances per call, so the sequence is deterministic per launch, which suits CI, while
/// successive shuffles differ).
fn shuffle(v: &mut [i64]) {
    thread_local! {
        static SEED: std::cell::Cell<u64> = const { std::cell::Cell::new(0x9E37_79B9_7F4A_7C15) };
    }
    let mut s = SEED.with(|c| c.get());
    for i in (1..v.len()).rev() {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        v.swap(i, (s % (i as u64 + 1)) as usize);
    }
    SEED.with(|c| c.set(s));
}

/// Compress sorted row numbers into the display value: runs of three or more become a range
/// ("4-10"), everything else lists out ("1,2,8"). A value, not prose: identical per locale.
fn format_rows(rows: &BTreeSet<i64>) -> String {
    let mut out = String::new();
    let mut iter = rows.iter().copied().peekable();
    while let Some(start) = iter.next() {
        let mut end = start;
        while iter.peek() == Some(&(end + 1)) {
            end = iter.next().unwrap_or(end);
        }
        if !out.is_empty() {
            out.push(',');
        }
        match end - start {
            0 => out.push_str(&start.to_string()),
            1 => out.push_str(&format!("{start},{end}")),
            _ => out.push_str(&format!("{start}-{end}")),
        }
    }
    out
}
