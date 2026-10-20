use day::prelude::*;
use day_piece_rating::Card;

use crate::widgets::page;

/// Layout: the containers a page is made of, and what they do as the window changes. Rows,
/// columns and layers with their alignment and spacing live; the row-fit policies and the
/// adaptive stack (docs/size-classes.md) over one row of buttons; the card and environment
/// composition tier;
/// a plain `scroll` with programmatic targets (docs/scroll.md); and the size classes the
/// window is in right now.
pub(crate) fn layout_page() -> AnyPiece {
    page(
        crate::res::str::nav_layout(),
        "layout-title",
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
/// each other. `align` is a build-time property, so `switch` rebuilds the same swatches per
/// alignment; the layered example is a `zstack`.
fn stacks_section() -> impl Piece {
    let align = Signal::new(VAlign::Center);
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
        .spacing(Space::M)
        .id("layout-stack")
    };
    section((
        labeled(
            crate::res::str::layout_align_label(),
            picker(
                align,
                [
                    (VAlign::Top, crate::res::str::layout_align_top()),
                    (VAlign::Center, crate::res::str::layout_align_center()),
                    (VAlign::Bottom, crate::res::str::layout_align_bottom()),
                ],
            )
            .segmented()
            .id("layout-align"),
        ),
        switch(move || align.get(), move |a| swatches(*a)),
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

/// The arrangements the demo row can take: the four row-fit policies (docs/size-classes.md
/// "Row fit policies") and the adaptive stack, which is what Column is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Fit {
    Clip,
    Wrap,
    WrapColumns,
    Column,
    Scroll,
}

/// Row fit policies (docs/size-classes.md "Row fit policies"), live: one row of buttons is
/// rendered under whichever arrangement the picker selects, and a stepper grows or shrinks the
/// button count, the quickest way to feel what Clip, Wrap, Column and Scroll each do to the
/// same content as it outgrows the window.
fn fit_section() -> impl Piece {
    // Which arrangement the demo row is built with, and how many buttons it holds. Both drive
    // the `switch` below through tracked reads.
    let fit = Signal::new(Fit::Clip);
    let count = Signal::new(5i64);

    section((
        label(crate::res::str::layout_note()).font(Font::Footnote),
        labeled(
            crate::res::str::layout_fit_label(),
            // Menu, not segmented: four worded segments outgrow a phone-portrait row, the
            // very failure this page teaches, and a native control day's fit policies cannot
            // reach into. The menu styling is compact at every width.
            picker(
                fit,
                [
                    (Fit::Clip, crate::res::str::layout_fit_clip()),
                    (Fit::Wrap, crate::res::str::layout_fit_wrap()),
                    (Fit::WrapColumns, crate::res::str::layout_fit_wrap_columns()),
                    (Fit::Column, crate::res::str::layout_fit_column()),
                    (Fit::Scroll, crate::res::str::layout_fit_scroll()),
                ],
            )
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
            .spacing(Space::S),
        ),
        // One subtree per arrangement: `fit` is a build-time property, so switching rebuilds
        // the row, and `switch` is the reactive one-of-N for exactly that. Only one arm is live
        // at a time, so the "layout-demo" id, applied once to whichever arm is live, stays
        // unique. Wrap and WrapColumns sit next to each other: the same eight buttons ragged,
        // then aligned into columns, is the comparison the page exists to make.
        switch(
            move || fit.get(),
            move |f| {
                let arm = match f {
                    Fit::Clip => demo_row(count, RowFit::Clip).any(),
                    Fit::Wrap => demo_row(count, RowFit::Wrap { run_spacing: 8.0 }).any(),
                    Fit::WrapColumns => {
                        demo_row(count, RowFit::WrapColumns { run_spacing: 8.0 }).any()
                    }
                    Fit::Column => adaptive_row(count).any(),
                    Fit::Scroll => demo_row(count, RowFit::Scroll).any(),
                };
                arm.id("layout-demo").any()
            },
        ),
    ))
    .title(crate::res::str::layout_row_section())
}

/// The demo itself: `count` numbered buttons in one `row` under `fit`. The buttons do nothing;
/// the row is the exhibit.
fn demo_row(count: Signal<i64>, fit: RowFit) -> impl Piece {
    row((demo_buttons(count),)).spacing(Space::S).fit(fit)
}

/// The same buttons as an `adaptive` stack (docs/size-classes.md): a row while the window is
/// wider than Compact, a leading-aligned column at or below it, the same nodes re-laid when
/// the window crosses the breakpoint.
fn adaptive_row(count: Signal<i64>) -> impl Piece {
    adaptive((demo_buttons(count),)).spacing(Space::S)
}

/// The buttons every arrangement holds.
fn demo_buttons(count: Signal<i64>) -> impl Piece {
    each(
        items(move || (1..=count.get()).collect::<Vec<_>>(), |n: &i64| *n),
        |slot: ItemSlot<i64, i64>| {
            // Every third button carries a longer label. With labels of one width
            // Wrap and Even columns produce the same picture, correctly, but the page would
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
    )
}

/// The composition tier: the `Card` modifier (padding, background and rounded corners as one
/// reusable surface) and an ambient value flowed through `with_environment` and read back by
/// a descendant. Pure composition: no native code or cargo features, and it works on every
/// backend.
fn cards_section() -> impl Piece {
    #[derive(Clone, Copy)]
    struct Accent(Color);
    section((
        label(crate::res::str::compose_caption()).font(Font::Footnote),
        column((
            label(crate::res::str::compose_card_title()).font(Font::Headline),
            label(crate::res::str::compose_card_body()),
        ))
        .spacing(Space::XS)
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
/// `ScrollTarget`, and the dayscript `scroll_to` step drives the same target by id. A fixed
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
        .spacing(Space::S)
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
        .id("layout-scroll")
        .height(200.0),
    ))
    .title(crate::res::str::layout_scroll_section())
}

/// The size classes the window is in right now (docs/size-classes.md). `size_class()` is a
/// tracked read, so the rows re-run when the window crosses a breakpoint: a rotation, a
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
