use crate::composition::Card;
use day::prelude::*;

use crate::gallery;
use crate::widgets::page_cards;

/// Layout: the containers a page is made of, and what they do as the window changes. Rows,
/// columns and layers with their alignment and spacing live; the row-fit policies and the
/// adaptive stack (docs/size-classes.md) over one row of buttons; lanes that reflow without
/// rebuilding and weighted grow; the card and environment composition tier; a plain `scroll`
/// with programmatic targets, a strip that pages or snaps, and a zooming one (docs/scroll.md);
/// and the size classes the window is in right now.
pub(crate) fn layout_page() -> AnyPiece {
    page_cards(
        crate::res::str::nav_layout(),
        day::id!(layout_title),
        gallery::balanced(
            2,
            vec![
                reactive_sizing_section().any(),
                stacks_section().any(),
                fit_section().any(),
                lanes_section().any(),
                weights_section().any(),
                cards_section().any(),
                scroll_section().any(),
                snap_section().any(),
                zoom_section().any(),
                size_section().any(),
            ],
        ),
    )
    .any()
}

/// All sliders update the existing shape. Width/height remeasure its layout; translation
/// moves its paint without changing the space reserved for the preview.
fn reactive_sizing_section() -> impl Piece {
    let width = Signal::new(120.0);
    let height = Signal::new(80.0);
    let horizontal = Signal::new(16.0);
    let vertical = Signal::new(16.0);
    crate::gallery::panel(
        crate::res::str::layout_reactive_title(),
        (
            zstack((rounded_rectangle(12.0)
                .fill(SystemColor::Accent)
                .frame(width, height)
                .translation(
                    move || {
                        if is_rtl() {
                            -horizontal.get()
                        } else {
                            horizontal.get()
                        }
                    },
                    vertical,
                )
                .a11y(move |a| {
                    a.role(Role::Image)
                        .label(crate::res::str::layout_reactive_shape(
                            crate::res::str::LayoutReactiveShape { width, height },
                        ))
                })
                .id(day::id!(layout_reactive_shape)),))
            .align(Alignment::TopLeading)
            .min_height(180.0)
            .grow(Dimension::Width)
            .background(SystemColor::Surface)
            .corner_radius(Radius::L)
            .id(day::id!(layout_reactive_preview)),
            sizing_control(
                crate::res::str::layout_reactive_width(),
                width,
                48.0..=160.0,
                day::id!(layout_reactive_width),
            ),
            sizing_control(
                crate::res::str::layout_reactive_height(),
                height,
                32.0..=120.0,
                day::id!(layout_reactive_height),
            ),
            sizing_control(
                crate::res::str::layout_reactive_horizontal(),
                horizontal,
                0.0..=40.0,
                day::id!(layout_reactive_horizontal),
            ),
            sizing_control(
                crate::res::str::layout_reactive_vertical(),
                vertical,
                0.0..=40.0,
                day::id!(layout_reactive_vertical),
            ),
            button(crate::res::str::layout_reactive_reset())
                .action(move || {
                    batch(|| {
                        width.set(120.0);
                        height.set(80.0);
                        horizontal.set(16.0);
                        vertical.set(16.0);
                    });
                })
                .id(day::id!(layout_reactive_reset)),
        ),
    )
}

fn sizing_control(
    title: LocalizedText,
    value: Signal<f64>,
    range: std::ops::RangeInclusive<f64>,
    id: ElementId,
) -> impl Piece {
    labeled(
        title,
        row((
            slider(value).range(range).step(1.0).id(id),
            label(crate::res::str::layout_reactive_points(value))
                .tabular()
                .reserving(crate::res::str::layout_reactive_points(160.0)),
        ))
        .spacing(Space::M),
    )
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
        .id(day::id!(layout_stack))
    };
    crate::gallery::panel(
        crate::res::str::layout_stacks_section(),
        (
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
                .id(day::id!(layout_align)),
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
                .id(day::id!(layout_layers)),
            ),
        ),
    )
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

    crate::gallery::panel(
        crate::res::str::layout_row_section(),
        (
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
                .id(day::id!(layout_fit)),
            ),
            labeled(
                crate::res::str::layout_count_label(),
                row((
                    button("−")
                        .bordered()
                        .action(move || count.update(|c| *c = (*c - 1).max(MIN_ITEMS)))
                        .id(day::id!(layout_remove)),
                    label(move || count.get().to_string())
                        .tabular()
                        .id(day::id!(layout_count)),
                    button("+")
                        .prominent()
                        .action(move || count.update(|c| *c = (*c + 1).min(MAX_ITEMS)))
                        .id(day::id!(layout_add)),
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
                    arm.id(day::id!(layout_demo)).any()
                },
            ),
        ),
    )
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

/// The tiles of the lanes demo: their heights, in points, so a balanced deal has something to
/// even out.
const TILE_HEIGHTS: [f64; 8] = [64.0, 40.0, 88.0, 52.0, 72.0, 44.0, 96.0, 60.0];

/// `lanes` (docs/size-classes.md "Lanes"): tiles dealt into as many lanes as the card's width
/// has room for. The lane count is decided during layout, so a window dragged narrow moves the
/// same tiles to new lanes; a tile tapped on keeps its color through the move. The Balanced
/// switch is a build-time property, so it rebuilds the lanes over the same per-tile state.
fn lanes_section() -> impl Piece {
    let balanced = Signal::new(false);
    let picked: Vec<Signal<bool>> = TILE_HEIGHTS.iter().map(|_| Signal::new(false)).collect();
    let tiles = move |balanced: bool| {
        let tiles: Vec<AnyPiece> = TILE_HEIGHTS
            .iter()
            .zip(picked.iter().copied())
            .enumerate()
            .map(|(index, (height, on))| {
                zstack((
                    rounded_rectangle(Radius::M).fill(move || {
                        if on.get() {
                            gallery::Tone::Accent.tint()
                        } else {
                            gallery::INSET
                        }
                    }),
                    label((index + 1).to_string())
                        .font(Font::Caption)
                        .secondary(),
                ))
                .height(*height)
                .grow(Dimension::Width)
                .on_tap(move || on.update(|on| *on = !*on))
                .id(day::id!(layout_lane_tile, index as u64))
                .any()
            })
            .collect();
        let lanes = lanes(tiles).lane_width(84.0).max_lanes(4).spacing(Space::S);
        if balanced {
            lanes.balanced().any()
        } else {
            lanes.any()
        }
    };
    crate::gallery::panel(
        crate::res::str::layout_lanes_title(),
        (
            crate::gallery::note(crate::res::str::layout_lanes_note()),
            labeled(
                crate::res::str::layout_lanes_balanced(),
                toggle(balanced).id(day::id!(layout_lanes_balanced)),
            ),
            switch(move || balanced.get(), move |b| tiles(*b))
                .grow(Dimension::Width)
                .id(day::id!(layout_lanes)),
        ),
    )
}

/// Weighted grow (docs/size-classes.md): three bars share a row's spare width by weight, the
/// middle one's picked here. Under them, chips in a wrapping row grow to share each line.
fn weights_section() -> impl Piece {
    let middle = Signal::new(2u32);
    let bar = |color: Color, weight: f64| {
        rounded_rectangle(Radius::S)
            .fill(color)
            .height(28.0)
            .grow(Dimension::Width.weight(weight))
    };
    crate::gallery::panel(
        crate::res::str::layout_weights_title(),
        (
            labeled(
                crate::res::str::layout_weights_middle(),
                picker(
                    middle,
                    [1u32, 2, 3, 4].map(|w| (w, crate::res::str::layout_weight(i64::from(w)))),
                )
                .segmented()
                .id(day::id!(layout_weight)),
            ),
            switch(
                move || middle.get(),
                move |w| {
                    row((
                        bar(crate::palette::SKY, 1.0),
                        bar(crate::palette::CORAL, f64::from(*w)),
                        bar(crate::palette::TEAL, 1.0),
                    ))
                    .spacing(Space::S)
                    .grow(Dimension::Width)
                },
            )
            .grow(Dimension::Width)
            .id(day::id!(layout_weights)),
            crate::gallery::note(crate::res::str::layout_weights_wrap()),
            row((1..=7)
                .map(|n: i64| {
                    let text = if n % 3 == 0 {
                        crate::res::str::layout_item_wide(n)
                    } else {
                        crate::res::str::layout_item(n)
                    };
                    label(text)
                        .font(Font::Footnote)
                        .single_line()
                        .padding(Insets::symmetric(10.0, 6.0))
                        .min_width(64.0)
                        .grow(Dimension::Width)
                        .background(gallery::INSET)
                        .corner_radius(Corner::CAPSULE)
                })
                .collect::<Vec<_>>())
            .spacing(Space::S)
            .fit(RowFit::Wrap { run_spacing: 8.0 })
            .grow(Dimension::Width)
            .id(day::id!(layout_wrap_grow)),
        ),
    )
}

/// The composition tier: the `Card` modifier (padding, background and rounded corners as one
/// reusable surface) and an ambient value flowed through `with_environment` and read back by
/// a descendant. Pure composition: no native code or cargo features, and it works on every
/// backend.
fn cards_section() -> impl Piece {
    #[derive(Clone, Copy)]
    struct Accent(Color);
    crate::gallery::panel(
        crate::res::str::nav_compose(),
        (
            column((
                label(crate::res::str::compose_card_title()).font(Font::Headline),
                label(crate::res::str::compose_card_body()),
            ))
            .spacing(Space::XS)
            .align(HAlign::Leading)
            .modifier(Card)
            .id(day::id!(layout_card)),
            with_environment(Accent(crate::palette::TEAL), || {
                let tint = environment::<Accent>().map(|a| a.0).unwrap_or(Color::BLACK);
                label(crate::res::str::compose_env_value())
                    .font(Font::Headline)
                    .color(tint)
                    .id(day::id!(compose_env_value))
                    .any()
            }),
        ),
    )
}

/// A plain `scroll` (docs/scroll.md) with programmatic targets: the two buttons write a
/// `ScrollTarget`, and the dayscript `scroll_to` step drives the same target by id. A fixed
/// height, so the strip scrolls inside the page rather than growing with it.
fn scroll_section() -> impl Piece {
    let target: Signal<Option<ScrollTarget>> = Signal::new(None);
    crate::gallery::panel(
        crate::res::str::layout_scroll_section(),
        (
            row((
                button(crate::res::str::layout_scroll_top())
                    .bordered()
                    .action(move || target.set(Some(ScrollTarget::Top)))
                    .id(day::id!(layout_scroll_top)),
                button(crate::res::str::layout_scroll_end())
                    .bordered()
                    .action(move || target.set(Some(ScrollTarget::Bottom)))
                    .id(day::id!(layout_scroll_end)),
            ))
            .spacing(Space::S)
            .fit(RowFit::Wrap { run_spacing: 8.0 }),
            scroll(
                column((each(
                    items(|| (1..=30).collect::<Vec<i64>>(), |i: &i64| *i),
                    |slot: ItemSlot<i64, i64>| {
                        label(move || crate::res::str::layout_row(slot.get()).format())
                            .padding(Insets::symmetric(12.0, 8.0))
                            .id(day::id!(layout_row, slot.key()))
                    },
                ),))
                .align(HAlign::Leading),
            )
            .scroll_target(target)
            .id(day::id!(layout_scroll))
            .height(200.0),
        ),
    )
}

/// Where the strip comes to rest after a swipe.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rest {
    Anywhere,
    Pages,
    Cards,
}

/// The strip's cards: their width and the gap between them, so a snap interval of one card
/// plus one gap always rests a card on the leading edge.
const STRIP_CARD: f64 = 140.0;
const STRIP_GAP: f64 = 12.0;

/// A horizontal strip (docs/scroll.md "Paging and snapping"): it rests anywhere, on whole
/// pages, or on each card, with its scroll bars and bounce switched off or on. All four are
/// realize-time preferences, so the strip is rebuilt when one changes. The offset and the
/// phase are tracked bindings, read back where the toolkit reports them.
fn snap_section() -> impl Piece {
    let rest = Signal::new(Rest::Cards);
    let indicators = Signal::new(true);
    let bounce = Signal::new(true);
    let offset = Signal::new(Point::ZERO);
    let phase = Signal::new(ScrollPhase::Idle);
    let snaps = capability(Cap::ScrollSnap) != Support::Unsupported;
    let tracked = capability(Cap::ScrollOffset) != Support::Unsupported;
    let strip = move |(rest, indicators, bounce): (Rest, bool, bool)| {
        let cards: Vec<AnyPiece> = (0..8)
            .map(|n| {
                zstack((
                    rounded_rectangle(Radius::L)
                        .fill(crate::palette::RAMP[n % crate::palette::RAMP.len()]),
                    label((n + 1).to_string())
                        .font(Font::Title2)
                        .bold()
                        .color(Color::WHITE),
                ))
                .frame(STRIP_CARD, 88.0)
                .any()
            })
            .collect();
        let mut strip = scroll(row(cards).spacing(STRIP_GAP))
            .horizontal()
            .scroll_offset(offset)
            .scroll_phase(phase);
        strip = match rest {
            Rest::Anywhere => strip,
            Rest::Pages => strip.paging(),
            Rest::Cards => strip.snap_interval(STRIP_CARD + STRIP_GAP),
        };
        if !indicators {
            strip = strip.hide_indicators();
        }
        if !bounce {
            strip = strip.no_bounce();
        }
        strip
            .height(104.0)
            .grow(Dimension::Width)
            .id(day::id!(layout_strip))
    };
    crate::gallery::panel(
        crate::res::str::layout_snap_title(),
        (
            labeled(
                crate::res::str::layout_snap_rest(),
                picker(
                    rest,
                    [
                        (Rest::Anywhere, crate::res::str::layout_snap_anywhere()),
                        (Rest::Pages, crate::res::str::layout_snap_pages()),
                        (Rest::Cards, crate::res::str::layout_snap_cards()),
                    ],
                )
                .segmented()
                .id(day::id!(layout_snap_rest)),
            ),
            (!snaps).then(|| crate::gallery::note(crate::res::str::layout_snap_unsupported())),
            labeled(
                crate::res::str::layout_snap_indicators(),
                toggle(indicators).id(day::id!(layout_snap_indicators)),
            ),
            labeled(
                crate::res::str::layout_snap_bounce(),
                toggle(bounce).id(day::id!(layout_snap_bounce)),
            ),
            switch(
                move || (rest.get(), indicators.get(), bounce.get()),
                move |key| strip(*key),
            )
            .grow(Dimension::Width),
            if tracked {
                labeled(
                    crate::res::str::layout_scroll_position(),
                    row((
                        label(move || crate::res::str::layout_scroll_offset(offset.get().x))
                            .tabular()
                            .reserving(crate::res::str::layout_scroll_offset(8888.0))
                            .id(day::id!(layout_strip_offset)),
                        label(move || match phase.get() {
                            ScrollPhase::Idle => crate::res::str::layout_phase_idle(),
                            ScrollPhase::Dragging => crate::res::str::layout_phase_dragging(),
                            ScrollPhase::Coasting => crate::res::str::layout_phase_coasting(),
                        })
                        .secondary()
                        .id(day::id!(layout_strip_phase)),
                    ))
                    .spacing(Space::S),
                )
                .any()
            } else {
                crate::gallery::note(crate::res::str::layout_scroll_untracked()).any()
            },
        ),
    )
}

/// Pinch zoom (docs/scroll.md "Zoom"): a scroll view that scales its content between one and
/// four times, with the scale bound both ways. Only some scroll views zoom
/// (`Cap::ScrollZoom`), so elsewhere the card says so instead of offering a control that does
/// nothing.
fn zoom_section() -> impl Piece {
    let scale = Signal::new(1.0_f64);
    let zooms = capability(Cap::ScrollZoom) != Support::Unsupported;
    let content = canvas(|draw, size| {
        gallery::dotted_stage(draw, size, 14.0);
        let center = Point::new(size.width / 2.0, size.height / 2.0);
        for (ring, color) in crate::palette::RAMP.iter().enumerate() {
            let r = 14.0 + ring as f64 * 14.0;
            draw.stroke(
                Shape::Ellipse(Rect::new(center.x - r, center.y - r, r * 2.0, r * 2.0)),
                *color,
                3.0,
            );
        }
    })
    .height(180.0)
    .grow(Dimension::Width);
    crate::gallery::panel(
        crate::res::str::layout_zoom_title(),
        (
            scroll(content)
                .zoom(1.0..=4.0)
                .zoom_scale(scale)
                .height(180.0)
                .grow(Dimension::Width)
                .id(day::id!(layout_zoom)),
            if zooms {
                row((
                    label(move || crate::res::str::layout_zoom_scale(scale.get()).format())
                        .tabular()
                        .grow(Dimension::Width)
                        .id(day::id!(layout_zoom_value)),
                    button(crate::res::str::layout_zoom_in())
                        .bordered()
                        .action(move || scale.update(|s| *s = (*s * 2.0).min(4.0)))
                        .id(day::id!(layout_zoom_in)),
                    button(crate::res::str::layout_zoom_reset())
                        .bordered()
                        .action(move || scale.set(1.0))
                        .id(day::id!(layout_zoom_reset)),
                ))
                .spacing(Space::S)
                .align(VAlign::Center)
                .any()
            } else {
                crate::gallery::note(crate::res::str::layout_zoom_unsupported()).any()
            },
        ),
    )
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
    crate::gallery::panel(
        crate::res::str::layout_size_section(),
        (
            labeled(
                crate::res::str::layout_size_width(),
                label(class(|c| format!("{:?}", c.width))).id(day::id!(layout_size_width)),
            ),
            labeled(
                crate::res::str::layout_size_height(),
                label(class(|c| format!("{:?}", c.height))).id(day::id!(layout_size_height)),
            ),
        ),
    )
}
