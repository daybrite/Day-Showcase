use day::prelude::*;
use day_piece_rating::Card;

use crate::widgets::page;

/// Layout: the containers a page is made of, and what they do as the window changes. Rows,
/// columns and layers with their alignment and spacing live; the five row-fit policies
/// (docs/size-classes.md) over one row of buttons; the card and environment composition tier;
/// a plain `scroll` with programmatic targets (docs/scroll.md); and the size classes the
/// window is in right now.
pub(crate) fn layout_page() -> AnyPiece {
    page(
        crate::res::str::nav_layout(),
        "layout-title",
        Some(crate::res::str::layout_caption()),
        form((
            stacks_section(),
            fit_section(),
            cards_section(),
            scroll_section(),
            size_section(),
        ))
        .any(),
    )
    .any()
}

/// Three swatches of different heights in one row: the alignment picker moves them against
/// each other. `align` is a build-time property, so each alignment is its own `when` arm over
/// the same swatches; the layered example is a `zstack`.
fn stacks_section() -> impl Piece {
    let align_idx = Signal::new(1usize);
    let names: Vec<String> = vec![
        crate::res::str::layout_align_top().format(),
        crate::res::str::layout_align_center().format(),
        crate::res::str::layout_align_bottom().format(),
    ];
    let swatches = move |align: VAlign| {
        row((
            rounded_rectangle(6.0)
                .fill(crate::palette::SKY)
                .frame(56.0, 24.0),
            rounded_rectangle(6.0)
                .fill(crate::palette::TEAL)
                .frame(56.0, 44.0),
            rounded_rectangle(6.0)
                .fill(crate::palette::CORAL)
                .frame(56.0, 64.0),
        ))
        .align(align)
        .spacing(12.0)
        .id("layout-stack")
    };
    let arm =
        move |i: usize, align: VAlign| when(move || align_idx.get() == i, move || swatches(align));
    section((
        labeled(
            crate::res::str::layout_align_label(),
            picker(names.iter().cloned(), align_idx)
                .segmented()
                .id("layout-align"),
        ),
        arm(0, VAlign::Top),
        arm(1, VAlign::Center),
        arm(2, VAlign::Bottom),
        labeled(
            crate::res::str::layout_layered(),
            zstack((
                rounded_rectangle(12.0)
                    .fill(crate::palette::SLATE)
                    .frame(140.0, 80.0),
                circle().fill(crate::palette::AMBER).frame(40.0, 40.0),
            ))
            .id("layout-layers"),
        ),
    ))
    .title(crate::res::str::layout_stacks_section())
}

/// Stepper bounds: at least one component, and enough headroom that every policy visibly
/// engages (Scroll needs the line well past the window; twenty gets there on a desktop too).
const MIN_ITEMS: i64 = 1;
const MAX_ITEMS: i64 = 20;

/// Row fit policies (docs/size-classes.md "Row fit policies"), live: ONE row of buttons is
/// rendered under whichever policy the picker selects, and a stepper grows or shrinks the
/// button count — the quickest way to feel what Clip, Wrap, Column and Scroll each do to the
/// same content as it outgrows the window.
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

/// The composition tier: the `Card` modifier — padding, background and rounded corners as one
/// reusable surface — and an ambient value flowed through `with_environment` and read back by
/// a descendant. Pure composition: no native code, no cargo features, every backend for free.
fn cards_section() -> impl Piece {
    #[derive(Clone, Copy)]
    struct Accent(Color);
    section((
        label(crate::res::str::compose_caption()).font(Font::Footnote),
        column((
            label(crate::res::str::compose_card_title()).font(Font::Headline),
            label(crate::res::str::compose_card_body()),
        ))
        .spacing(4.0)
        .align(HAlign::Leading)
        .modifier(Card)
        .id("layout-card"),
        with_environment(Accent(crate::palette::TEAL), || {
            let tint = environment::<Accent>().map(|a| a.0).unwrap_or(Color::BLACK);
            label(crate::res::str::compose_env_value())
                .font(Font::Headline)
                .color(tint)
                .id("compose-env-value")
                .any()
        }),
    ))
    .title(crate::res::str::nav_compose())
}

/// A plain `scroll` (docs/scroll.md) with programmatic targets: the two buttons write a
/// `ScrollTarget`, and the dayscript `scroll_to` step drives the same seam by id. A fixed
/// height, so the strip scrolls inside the page rather than growing with it.
fn scroll_section() -> impl Piece {
    let target: Signal<Option<ScrollTarget>> = Signal::new(None);
    section((
        row((
            button(crate::res::str::layout_scroll_top())
                .bordered()
                .action(move || target.set(Some(ScrollTarget::Top)))
                .id("layout-scroll-top"),
            button(crate::res::str::layout_scroll_end())
                .bordered()
                .action(move || target.set(Some(ScrollTarget::Bottom)))
                .id("layout-scroll-end"),
        ))
        .spacing(8.0)
        .fit(RowFit::Wrap { run_spacing: 8.0 }),
        scroll(
            column((each(
                items(|| (1..=30).collect::<Vec<i64>>(), |i: &i64| *i),
                |slot: ItemSlot<i64, i64>| {
                    label(move || crate::res::str::layout_row(slot.get()).format())
                        .padding(Insets::symmetric(12.0, 8.0))
                        .id_keyed("layout-row", slot.key())
                },
            ),))
            .align(HAlign::Leading),
        )
        .scroll_target(target)
        // `.id` BEFORE `.height`: the id must tag the scroll piece itself, not the sizing
        // wrapper, for the dayscript `scroll_to` step to find a realized scroll.
        .id("layout-scroll")
        .height(200.0),
    ))
    .title(crate::res::str::layout_scroll_section())
}

/// The size classes the window is in right now (docs/size-classes.md). `size_class()` is a
/// tracked read, so the rows re-run when the window crosses a breakpoint — a rotation, a
/// foldable opening, a desktop window dragged narrow.
fn size_section() -> impl Piece {
    let class = |pick: fn(SizeClass) -> String| {
        move || {
            day::size_class()
                .map(pick)
                .unwrap_or_else(|| "\u{2014}".to_string())
        }
    };
    section((
        labeled(
            crate::res::str::layout_size_width(),
            label(class(|c| format!("{:?}", c.width))).id("layout-size-width"),
        ),
        labeled(
            crate::res::str::layout_size_height(),
            label(class(|c| format!("{:?}", c.height))).id("layout-size-height"),
        ),
    ))
    .title(crate::res::str::layout_size_section())
}
