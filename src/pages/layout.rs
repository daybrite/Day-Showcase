use day::prelude::*;

use crate::widgets::page;

/// Row fit policies (docs/size-classes.md "Row fit policies"), live: ONE row of buttons is
/// rendered under whichever policy the picker selects, and a stepper grows or shrinks the
/// button count — the quickest way to feel what Clip, Wrap, Column and Scroll each do to the
/// same content as it outgrows the window. More layout demonstrations can join this page as
/// the vocabulary grows; the fit policies are its first residents.
pub(crate) fn layout_page() -> AnyPiece {
    page(
        crate::res::str::nav_layout(),
        "layout-title",
        Some(crate::res::str::layout_caption()),
        form((fit_section(),)).any(),
    )
    .any()
}

/// Stepper bounds: at least one component, and enough headroom that every policy visibly
/// engages (Scroll needs the line well past the window; twenty gets there on a desktop too).
const MIN_ITEMS: i64 = 1;
const MAX_ITEMS: i64 = 20;

fn fit_section() -> impl Piece {
    // Which fit policy the demo row is built with (picker index), and how many buttons it
    // holds. Both drive the `when` arms below through tracked reads.
    let fit_idx = Signal::new(0usize);
    let count = Signal::new(5i64);

    let names: Vec<String> = vec![
        crate::res::str::layout_fit_clip().format(),
        crate::res::str::layout_fit_wrap().format(),
        crate::res::str::layout_fit_wrap_columns().format(),
        crate::res::str::layout_fit_column().format(),
        crate::res::str::layout_fit_scroll().format(),
    ];

    // One arm per policy: `fit` is a build-time property, so switching policies rebuilds the
    // row — `when` is the reactive seam for exactly that. Only one arm is live at a time, so
    // the shared "layout-demo" id stays unique.
    let arm = |i: usize, fit: RowFit| {
        when(
            move || fit_idx.get() == i,
            move || demo_row(count, fit).id("layout-demo"),
        )
    };

    section((
        label(crate::res::str::layout_note()).font(Font::Footnote),
        labeled(
            crate::res::str::layout_fit_label(),
            // Menu, not segmented: four worded segments outgrow a phone-portrait row — the
            // very failure this page teaches, and a native control day's fit policies cannot
            // reach into. The menu styling is compact at every width.
            picker(names.iter().cloned(), fit_idx)
                .menu()
                .id("layout-fit"),
        ),
        labeled(
            crate::res::str::layout_count_label(),
            row((
                button("−")
                    .bordered()
                    .action(move || count.update(|c| *c = (*c - 1).max(MIN_ITEMS)))
                    .id("layout-remove"),
                label(move || count.get().to_string())
                    .tabular()
                    .id("layout-count"),
                button("+")
                    .prominent()
                    .action(move || count.update(|c| *c = (*c + 1).min(MAX_ITEMS)))
                    .id("layout-add"),
            ))
            .spacing(8.0),
        ),
        arm(0, RowFit::Clip),
        // Wrap and WrapColumns sit next to each other on purpose: the same eight buttons
        // ragged, then aligned into columns, is the comparison the page exists to make.
        arm(1, RowFit::Wrap { run_spacing: 8.0 }),
        arm(2, RowFit::WrapColumns { run_spacing: 8.0 }),
        arm(3, RowFit::ColumnAt(WidthClass::Compact)),
        arm(4, RowFit::Scroll),
    ))
    .title(crate::res::str::layout_row_section())
}

/// The demo itself: `count` numbered buttons in one `row` under `fit`. The buttons do nothing —
/// the row is the exhibit.
fn demo_row(count: Signal<i64>, fit: RowFit) -> impl Piece {
    row((each(
        items(move || (1..=count.get()).collect::<Vec<_>>(), |n: &i64| *n),
        |slot: ItemSlot<i64, i64>| {
            // Every third button carries a longer label ON PURPOSE. With labels of one width
            // Wrap and Even columns produce the same picture — correctly, but the page would
            // then demonstrate nothing. Mixed widths are also the realistic case: a chip row
            // holds words, not a keypad.
            button(slot.field(|n| {
                if n % 3 == 0 {
                    crate::res::str::layout_item_wide(*n).format()
                } else {
                    crate::res::str::layout_item(*n).format()
                }
            }))
            .bordered()
        },
    ),))
    .spacing(8.0)
    .fit(fit)
}
