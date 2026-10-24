use std::collections::BTreeSet;

use day::prelude::*;
use day::pullrefresh::pull_to_refresh;

use crate::widgets::heading;

/// A native recycling list (docs/list.md): 500 rows, but only the visible cells are ever built;
/// the platform's NSTableView / RecyclerView / GtkListView / QListView / ItemsRepeater owns reuse.
/// The list is wrapped in `pull_to_refresh` (day::pullrefresh), the recycling-list example:
/// a pull (or dayscript `toggle: {id: list-refresh}`) adds 100 rows, same as the button.
///
/// Selection (docs/list.md): the rows are multi-selectable where the toolkit supports it, the
/// full selection lives in an app signal summarized above the list (with ranges compressed,
/// "4-10"), and Clear Selection syncs an empty selection back into the native list.
///
/// The Options sheet rebuilds the list with section headers, self-sizing rows, separators,
/// native list items, and each gesture on or off, so a list's behavior can be compared across
/// toolkits with the same settings (docs/list.md).
///
/// The row "Scroll to item 100" targets: a row number, which is also its key. The button
/// resolves it to a position at press time, so the jump follows the row through a shuffle.
const TARGET_ROW: i64 = 100;

/// How the demo list is built (docs/list.md): every option the Options sheet flips. A change
/// rebuilds the list (most of these are native-list configuration fixed at realize); the rows,
/// the selection and the jump targets live outside it, so a rebuild keeps them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ListOptions {
    sections: bool,
    automatic: bool,
    separators: Option<bool>,
    native_items: bool,
    multi_select: bool,
    pull_refresh: bool,
    reorder: bool,
    swipe: bool,
    delete: bool,
}

/// The page's state, shared by the controls above the list and every rebuild of the list.
#[derive(Clone, Copy)]
struct ListState {
    rows: Signal<Vec<i64>>,
    refreshing: Signal<bool>,
    jump_row: Signal<Option<usize>>,
    jump_end: Trigger,
    activated: Signal<i64>,
    selected: Signal<BTreeSet<i64>>,
    editing: Signal<bool>,
}

/// Rows per section when the list shows section headers.
const SECTION_ROWS: i64 = 25;

pub(crate) fn list_page() -> AnyPiece {
    // A real Vec of row numbers (not a derived range): drag-to-reorder rotates it, shuffle
    // permutes it, and the refresh paths append to it; the order is app-owned state.
    let rows: Signal<Vec<i64>> = Signal::new((1..=500).collect());
    let refreshing = Signal::new(false);
    // The options the sheet flips, one signal per control. The defaults are the list this
    // page has always shown: plain uniform rows, multi-select, pull to refresh and every
    // gesture.
    let sections = Signal::new(false);
    let automatic = Signal::new(false);
    let separators: Signal<Option<bool>> = Signal::new(None);
    let native_items = Signal::new(false);
    let multi_select = Signal::new(true);
    let pull_refresh = Signal::new(true);
    let reorder = Signal::new(true);
    let swipe = Signal::new(true);
    let delete = Signal::new(true);
    let options_open: Signal<Option<OptionsKey>> = Signal::new(None);
    let st = ListState {
        rows,
        refreshing,
        // Programmatic scrolling (docs/list.md): a row-jump signal + an end trigger drive the
        // native list, the row rail's counterpart to `scroll(...).scroll_target(...)`.
        jump_row: Signal::new(None),
        jump_end: Trigger::new(),
        // The selected row numbers (1-based, matching the row labels), fed from the native
        // list's selection reports; single-selection toolkits contribute one-element sets.
        activated: Signal::new(0i64),
        selected: Signal::new(BTreeSet::new()),
        // Reactive, unlike the rest: the platform animates into and out of editing mode.
        editing: Signal::new(false),
    };
    let (jump_row, jump_end, selected, activated) =
        (st.jump_row, st.jump_end, st.selected, st.activated);
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
    let options = move || ListOptions {
        sections: sections.get(),
        automatic: automatic.get(),
        separators: separators.get(),
        native_items: native_items.get(),
        multi_select: multi_select.get(),
        pull_refresh: pull_refresh.get(),
        reorder: reorder.get(),
        swipe: swipe.get(),
        delete: delete.get(),
    };
    column((
        row((
            heading(crate::res::str::nav_list(), day::id!(list_title)),
            spacer(),
            button(crate::res::str::list_options())
                .bordered()
                .action(move || options_open.set(Some(OptionsKey)))
                .id(day::id!(list_options)),
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
        .spacing(Space::S)
        .fit(RowFit::Wrap { run_spacing: 8.0 }),
        label(move || crate::res::str::list_caption(rows.get().len() as i64).format())
            .tabular()
            .id(day::id!(list_caption)),
        // The tier this backend realizes for the pull (docs/pullrefresh.md): native on the
        // phones and HarmonyOS, an elastic overshoot on the desktops, visible in the CI captures.
        label(match day::pullrefresh::support() {
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
        keyed(
            move || Some(options()),
            move |o: &ListOptions| demo_list(*o, st),
        ),
        options_sheet(
            options_open,
            OptionSignals {
                sections,
                automatic,
                separators,
                native_items,
                multi_select,
                pull_refresh,
                editing: st.editing,
                reorder,
                swipe,
                delete,
            },
        ),
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    .padding(Space::L)
    .any()
}

/// The demo list as `o` configures it.
fn demo_list(o: ListOptions, st: ListState) -> AnyPiece {
    let ListState {
        rows,
        refreshing,
        jump_row,
        jump_end,
        activated,
        selected,
        editing,
    } = st;
    let list = list(
        items(move || rows.get(), |n: &i64| *n),
        move |row: ItemSlot<i64, i64>| demo_row(row, o, selected),
    )
    .row_height(if o.automatic {
        RowHeight::Automatic
    } else {
        RowHeight::Uniform(36.0)
    })
    // Keys are the row numbers, so the selection set is the report itself.
    .on_activate(move |key| activated.set(key))
    .on_selection(move |keys: Vec<i64>| selected.set(keys.into_iter().collect()))
    // Two-way: app-state changes (Clear Selection) sync into the native list; indices are the
    // rows' current positions (reorder can move them).
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
    .editing(editing);
    let list = if o.multi_select {
        list.multi_select()
    } else {
        list
    };
    let list = match o.separators {
        Some(true) => list.separators(),
        Some(false) => list.no_separators(),
        None => list,
    };
    // Section headers (docs/list.md "Sections"): rows grouped by twenty-fives, by number, so a
    // shuffle splits them into many small sections, each with its own header.
    let list = if o.sections {
        list.sections(
            move |i| rows.with(|v| (v[i] - 1) / SECTION_ROWS),
            |row: ItemSlot<i64, i64>| {
                let first = move || (row.get() - 1) / SECTION_ROWS * SECTION_ROWS + 1;
                label(move || {
                    crate::res::str::list_section(crate::res::str::ListSection {
                        first: first(),
                        last: first() + SECTION_ROWS - 1,
                    })
                    .format()
                })
                .font(Font::Subheadline)
                .weight(FontWeight::Semibold)
                .secondary()
                .padding(Insets::symmetric(12.0, 6.0))
                .id_of(move || day::id!(list_section, first()))
                .grow_w()
            },
        )
    } else {
        list
    };
    // Native drag-to-reorder, with a guard demo: the first row is pinned: it cannot be dragged,
    // and a drop aimed at its slot lands just below it instead (Retarget). With section headers
    // a row drags within its own section; the guard still sees row indices.
    let list = if o.reorder {
        list.reorderable(
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
    } else {
        list
    };
    let list = if o.swipe {
        // Leading swipe opens the same row as activation; trailing selects or deselects it.
        // WinUI also offers these actions in a native context menu for mouse and keyboard users.
        list.swipe_leading(move |index| {
            let key = rows.get_untracked()[index];
            vec![
                swipe_action(crate::res::str::menu_open())
                    .symbol(Symbol::Open)
                    .action(move || activated.set(key)),
            ]
        })
        .swipe_trailing(move |index| {
            let key = rows.get_untracked()[index];
            let on = selected.with_untracked(|s| s.contains(&key));
            let text = if on {
                crate::res::str::list_swipe_deselect()
            } else {
                crate::res::str::list_swipe_select()
            };
            vec![
                swipe_action(text)
                    .symbol(Symbol::Check)
                    .tint(crate::widgets::primary())
                    .action(move || {
                        selected.update(|s| {
                            if !s.remove(&key) {
                                s.insert(key);
                            }
                        })
                    }),
            ]
        })
    } else {
        list
    };
    // Native swipe-to-delete where supported (including WinUI). The same first row is pinned
    // as for reorder, so one demo shows both guards agreeing about it.
    let list = if o.delete {
        list.deletable(
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
    } else {
        list
    };
    let list = list.id(day::id!(demo_list)).grow();
    // The list is wrapped in `pull_to_refresh` (day::pullrefresh), the recycling-list example: a
    // pull (or dayscript `toggle: {id: list-refresh}`) adds 100 rows, same as the button.
    if o.pull_refresh {
        pull_to_refresh(refreshing, list)
            .id(day::id!(list_refresh))
            .grow()
            .any()
    } else {
        list.any()
    }
}

/// One demo row: a plain label or the platform's own list item, with extra lines of detail on
/// some rows when the list sizes rows by their content.
fn demo_row(row: ItemSlot<i64, i64>, o: ListOptions, selected: Signal<BTreeSet<i64>>) -> AnyPiece {
    let n = move || row.get();
    // Uniform rows have one line's room; only self-sizing rows vary.
    let note = move || match n() {
        n if o.automatic && n % 5 == 0 => crate::res::str::list_row_note_long().format(),
        n if o.automatic && n % 3 == 0 => crate::res::str::list_row_note().format(),
        _ => String::new(),
    };
    // The target row wears the warm accent so "Scroll to item 100" visibly lands on it wherever
    // a shuffle has moved it; the rest keep the platform's own label color, like any list
    // (reactive, so recycled cells restyle as they rebind). The list item's icon takes the
    // theme-neutral slate.
    let target = move || n() == TARGET_ROW;
    let tint = move || {
        if target() {
            crate::palette::CORAL
        } else {
            crate::palette::SLATE
        }
    };
    let title = move || crate::res::str::list_row(n()).format();
    if o.native_items {
        let item = list_item(title)
            .symbol(Symbol::Document)
            .icon_tint(tint)
            .detail(move || format!("#{}", n()))
            .checked(move || selected.with(|s| s.contains(&n())));
        let item = if o.automatic {
            item.subtitle(note)
        } else {
            item
        };
        // A physical row keeps its subtree across recycling; its test/a11y identity must follow
        // the slot just as its text and color do.
        return item.id_of(move || day::id!(list_row, row.key())).any();
    }
    let id = move || day::id!(list_row, row.key());
    column((
        when(target, move || label(title).color(crate::palette::CORAL).id_of(id))
            .otherwise(move || label(title).id_of(id)),
        when(
            move || !note().is_empty(),
            move || label(note).font(Font::Footnote).secondary(),
        ),
    ))
    .spacing(2.0)
    .align(HAlign::Leading)
    .padding(if o.automatic {
        Insets::symmetric(12.0, 8.0)
    } else {
        Insets::symmetric(12.0, 0.0)
    })
    .grow_w()
    .any()
}

// The options sheet's route key: a unit route matching only "options", so a String key never
// claims every segment routed while this page is current (docs/cover.md).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct OptionsKey;
impl Route for OptionsKey {
    fn key(&self) -> String {
        "options".into()
    }
    fn from_key(key: &str) -> Option<Self> {
        (key == "options").then_some(OptionsKey)
    }
}

/// The signals the options sheet edits.
#[derive(Clone, Copy)]
struct OptionSignals {
    sections: Signal<bool>,
    automatic: Signal<bool>,
    separators: Signal<Option<bool>>,
    native_items: Signal<bool>,
    multi_select: Signal<bool>,
    pull_refresh: Signal<bool>,
    editing: Signal<bool>,
    reorder: Signal<bool>,
    swipe: Signal<bool>,
    delete: Signal<bool>,
}

/// The List Options sheet: each control rebuilds the list behind it as it changes, so two
/// windows on two toolkits can be set alike and compared (docs/list.md).
fn options_sheet(open: Signal<Option<OptionsKey>>, s: OptionSignals) -> impl Piece {
    use crate::res::str as t;
    cover(open, move |_| {
        scroll(
            column((
                label(t::list_options_title())
                    .font(Font::Title)
                    .id(day::id!(list_options_title)),
                form((
                    section((
                        labeled(
                            t::list_opt_sections(),
                            toggle(s.sections).id(day::id!(list_opt_sections)),
                        ),
                        labeled(
                            t::list_opt_row_height(),
                            picker(
                                s.automatic,
                                [
                                    (false, t::list_opt_uniform()),
                                    (true, t::list_opt_automatic()),
                                ],
                            )
                            .segmented()
                            .id(day::id!(list_opt_row_height)),
                        ),
                        labeled(
                            t::list_opt_separators(),
                            picker(
                                s.separators,
                                [
                                    (None, t::list_opt_default()),
                                    (Some(true), t::list_opt_on()),
                                    (Some(false), t::list_opt_off()),
                                ],
                            )
                            .segmented()
                            .id(day::id!(list_opt_separators)),
                        ),
                        labeled(
                            t::list_opt_content(),
                            picker(
                                s.native_items,
                                [(false, t::list_opt_plain()), (true, t::list_opt_item())],
                            )
                            .segmented()
                            .id(day::id!(list_opt_content)),
                        ),
                    ))
                    .title(t::list_options_layout()),
                    section((
                        labeled(
                            t::list_opt_multi(),
                            toggle(s.multi_select).id(day::id!(list_opt_multi)),
                        ),
                        labeled(
                            t::list_opt_pull(),
                            toggle(s.pull_refresh).id(day::id!(list_opt_pull)),
                        ),
                        labeled(
                            t::list_opt_editing(),
                            toggle(s.editing).id(day::id!(list_opt_editing)),
                        ),
                    ))
                    .title(t::list_options_behavior()),
                    section((
                        labeled(
                            t::list_opt_reorder(),
                            toggle(s.reorder).id(day::id!(list_opt_reorder)),
                        ),
                        labeled(
                            t::list_opt_swipe(),
                            toggle(s.swipe).id(day::id!(list_opt_swipe)),
                        ),
                        // A sectioned list offers no swipe-to-delete (docs/list.md "Sections").
                        labeled(
                            t::list_opt_delete(),
                            toggle(s.delete)
                                .enabled(move || !s.sections.get())
                                .id(day::id!(list_opt_delete)),
                        ),
                    ))
                    .title(t::list_options_gestures())
                    .footer(t::list_opt_gestures_footer()),
                )),
                button(t::cover_dismiss())
                    .prominent()
                    .action(move || open.set(None))
                    .id(day::id!(list_options_dismiss)),
            ))
            .spacing(Space::M)
            .padding(Space::XL),
        )
        .any()
    })
    .sheet()
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
