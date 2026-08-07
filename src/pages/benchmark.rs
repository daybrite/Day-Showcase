//! The Benchmark page: Day-Bench's Grids benchmark — a pseudo-random patchwork of grid cells that
//! must tile the pane exactly — with, on the Apple-native backends, a hand-written SwiftUI twin of
//! the same benchmark hosted beside it under a segmented picker (day-piece-swiftui,
//! docs/swiftui.md). The twin lives in this repo's `swiftui/` package and reaches Rust as the
//! generated `crate::swiftui::BenchGridsView` constructor.
//!
//! What the benchmark measures. Every cell is flexible on both axes and every row is packed to
//! exactly [`COLUMNS`] columns, so the grid can only resolve by negotiating all of it at once:
//! column widths come from the flexible-share rule, spans redistribute their deficit across the
//! columns they cover, and row heights stretch to consume the leftover height. Nothing here can be
//! solved per-cell — changing "Total Count" repacks the rows, which changes the spans, which
//! changes the column split. That is the point: it is the layout engine under load, not the
//! renderer. Drag Total Count and watch; Random Seed does the same work at a fixed cell count,
//! which separates layout cost from the cost of creating and destroying views.
//!
//! Why it stays honest. The patchwork is a pure function of (seed, count) through [`Rng`], so the
//! same parameters draw the same picture on every target — and in BOTH tabs, whose generators pin
//! the same literal fixtures (`swiftui/Tests`, the `parity` tests below) — so two screenshots can
//! be diffed. The colors step by the golden angle, so adjacent tiles never blur together and a
//! mis-packed row is visible rather than merely slow.

use day::prelude::*;

/// The height reserved for the Parameters block, so the patchwork below it starts at the same y in
/// the Day-native tab and the SwiftUI tab (whose own controls are laid out by SwiftUI). Two slider
/// rows plus the row-count line at the default text size.
const PARAMS_HEIGHT: f64 = 132.0;

use crate::widgets::heading;

// --- The deterministic generator (ported from Day-Bench src/bench.rs) ---

/// A 32-bit linear congruential generator. Not statistically strong — it does not need to be.
/// What it needs is to be cheap, reproducible, and identical everywhere: integer wrapping
/// arithmetic is bit-identical on every architecture Day targets, wasm32 included.
struct Rng(u32);

impl Rng {
    /// Seed the generator. The multiply spreads small, adjacent seeds (0, 1, 2 — what a slider
    /// produces) across the state space, so consecutive seeds look unrelated instead of drawing
    /// near-identical layouts.
    fn new(seed: u32) -> Self {
        Rng(seed.wrapping_mul(2_654_435_769).wrapping_add(1))
    }

    /// The next raw value. The high bits are the well-mixed ones in an LCG, so the low byte is
    /// discarded rather than returned — taking `% n` of the raw state would expose its short
    /// low-bit cycles as visible banding in the patchwork.
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        self.0 >> 8
    }

    /// A value in `lo..=hi`. `lo > hi` is not reachable from the call sites (both bounds are
    /// constants or clamped counts), so the range is asserted rather than silently repaired.
    fn range(&mut self, lo: u32, hi: u32) -> u32 {
        debug_assert!(lo <= hi, "empty range {lo}..={hi}");
        lo + self.next() % (hi - lo + 1)
    }
}

/// The color for cell `i`: a predictable sequence in which no two nearby cells collide.
///
/// Stepping the hue by the golden angle (137°) is the standard trick for a sequence whose
/// consecutive entries are as far apart on the wheel as possible — 0°, 137°, 274°, 51°, 188° —
/// so neighbours stay distinct however the patchwork packs them, and cell `i` is always the same
/// color for a given `i`. Saturation and lightness are fixed, which keeps every cell equally
/// readable against both the light and dark app grounds.
fn cell_color(i: usize) -> Color {
    let hue = (i as u32).wrapping_mul(137) % 360;
    hsl(hue as f64, 0.62, 0.56)
}

/// HSL → RGB, the standard piecewise conversion. Arithmetic only (no transcendentals), so it
/// gives bit-identical colors on every target — and to the SwiftUI twin, whose `hsl` is this
/// function transliterated.
fn hsl(h_deg: f64, s: f64, l: f64) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h = h_deg / 60.0;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());
    let (r, g, b) = match h as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    Color::rgb(r + m, g + m, b + m)
}

// --- The packing (ported from Day-Bench src/pages/grids.rs) ---

/// Columns every row must fill exactly. Twelve divides by 2, 3, 4, and 6, so spans of 1–4 can
/// close a row on many different boundaries instead of forcing one repeating rhythm.
const COLUMNS: u32 = 12;
/// The widest a single tile may span. Wider spans make the deficit-distribution path do more work.
const MAX_SPAN: u32 = 4;

/// One tile: its index (which fixes its color) and how many columns it covers.
#[derive(Clone, PartialEq)]
struct Tile {
    index: usize,
    span: u32,
}

/// One packed row, keyed for `each` so a parameter change rebuilds exactly the rows that moved.
#[derive(Clone, PartialEq)]
struct Row {
    key: (u32, usize),
    tiles: Vec<Tile>,
}

/// Pack `count` tiles into rows of exactly [`COLUMNS`] columns.
///
/// The span is drawn from the generator and then clamped to what is left in the row, so a row
/// always closes on the column boundary rather than overflowing into the next. The final row is
/// short by construction — its last tile absorbs the remainder, which keeps the "every row fills
/// the width" invariant true for the whole grid rather than all-but-one of it.
fn pack(seed: u32, count: usize) -> Vec<Row> {
    let mut rng = Rng::new(seed);
    let mut rows: Vec<Row> = Vec::new();
    let mut tiles: Vec<Tile> = Vec::new();
    let mut used = 0u32;
    for index in 0..count {
        let remaining = COLUMNS - used;
        let span = rng.range(1, MAX_SPAN).min(remaining);
        tiles.push(Tile { index, span });
        used += span;
        if used == COLUMNS {
            rows.push(Row {
                key: (seed, rows.len()),
                tiles: std::mem::take(&mut tiles),
            });
            used = 0;
        }
    }
    if let Some(last) = tiles.last_mut() {
        last.span += COLUMNS - used;
        rows.push(Row {
            key: (seed, rows.len()),
            tiles,
        });
    }
    rows
}

/// Tiles the page opens with. Deliberately modest: the slider reaches 2000, and a benchmark that
/// cannot cold-start on its slowest target is not a benchmark. Sweep up from here.
const DEFAULT_COUNT: f64 = 48.0;

// --- The page ---

/// The Day tab's parameter signals, GLOBAL so they outlive the page scope: both tabs keep their
/// slider settings across tab switches and page revisits — the Day side through these, the
/// SwiftUI side through its `.state_key` (the retained hosting view keeps the `@State`). Guarded
/// like `lifecycle_log` in lib.rs: `Signal::global` on every build would mint a fresh pair.
fn bench_signals() -> (Signal<f64>, Signal<f64>) {
    use std::cell::OnceCell;
    thread_local! {
        static PARAMS: OnceCell<(Signal<f64>, Signal<f64>)> = const { OnceCell::new() };
    }
    PARAMS.with(|c| *c.get_or_init(|| (Signal::global(1.0), Signal::global(DEFAULT_COUNT))))
}

/// The Benchmark page: heading, then — where `day_piece_swiftui::support()` is Native (the
/// macos-appkit and ios-uikit builds) — a segmented picker hosting the Day-native benchmark and
/// its SwiftUI twin as tabs. Everywhere else, just the Day-native benchmark: the picker never
/// exists, so the swiftui piece is never built and no placeholder can realize.
pub(crate) fn benchmark_page() -> AnyPiece {
    let (seed, count) = bench_signals();

    let body: AnyPiece = if day_piece_swiftui::support() == Support::Native {
        let tab = Signal::new(0usize);
        // The picker centres over the pane rather than hugging the leading edge: it names the two
        // implementations being compared, so it reads as a title for the comparison below it.
        let picker_row = column((picker(
            [
                crate::res::str::bench_tab_day().format(),
                crate::res::str::bench_tab_swiftui().format(),
            ],
            tab,
        )
        .segmented()
        .id("bench-impl"),))
        .align(HAlign::Center)
        .grow_w()
        .any();
        let body = column((
            picker_row,
            when(move || tab.get() == 0, move || day_native(seed, count)),
            when(move || tab.get() == 1, swiftui_pane),
        ))
        .spacing(10.0)
        .align(HAlign::Leading)
        .grow()
        .any();
        body
    } else {
        day_native(seed, count)
    };

    // NOT widgets::page(): that scroll-wraps its body, and the patchwork must FILL the remaining
    // pane exactly (the benchmark's whole invariant) — the tabs page sets the same precedent.
    column((
        heading(
            crate::res::str::nav_benchmark(),
            "benchmark-title",
            Some(crate::res::str::bench_caption()),
        ),
        body,
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    .padding(16.0)
    .grow()
    .any()
}

/// The Day-native benchmark: the parameter controls above, the patchwork filling everything below
/// — the Day-Bench Grids page, with the heading hoisted into the page shell. The signals come
/// from [`bench_signals`], so the parameters survive tab switches and page revisits.
fn day_native(seed: Signal<f64>, count: Signal<f64>) -> AnyPiece {
    let rows = move || pack(seed.get() as u32, count.get() as usize);

    column((
        // Parameters. Both are sliders because a benchmark is driven by sweeping a value, not by
        // typing one — and because a slider is the one control every backend renders natively.
        form((section((
            labeled(
                crate::res::str::bench_seed(),
                row((
                    slider(seed).range(0.0..=999.0).step(1.0).id("bench-seed"),
                    crate::widgets::numeric_readout(
                        move || (seed.get() as u32).to_string(),
                        "999",
                        "bench-seed-value",
                    ),
                ))
                .spacing(8.0),
            ),
            labeled(
                crate::res::str::bench_count(),
                row((
                    slider(count)
                        .range(0.0..=2000.0)
                        .step(1.0)
                        .id("bench-count"),
                    crate::widgets::numeric_readout(
                        move || (count.get() as u32).to_string(),
                        "2000",
                        "bench-count-value",
                    ),
                ))
                .spacing(8.0),
            ),
            label(move || {
                crate::res::str::bench_rows(
                    pack(seed.get() as u32, count.get() as usize).len() as i64
                )
                .format()
            })
            .tabular()
            .font(Font::Footnote)
            .id("bench-rows"),
        ))
        .title(crate::res::str::bench_parameters()),))
        // A FIXED height, because the whole point of this page is that the two tabs draw the same
        // scene at the same size: if the Day-native parameter block is a different height from the
        // SwiftUI pane's own, the grids below them get different areas and stop being comparable.
        // Sized for two slider rows plus the row-count line at the default text size.
        .height(PARAMS_HEIGHT),
        // The patchwork. Every tile grows on both axes, so the grid resolves columns by the
        // flexible share and stretches rows into the leftover height — it fills the pane exactly.
        grid((each(
            rows,
            |r: &Row| r.key,
            |slot| {
                let tiles = slot.get().tiles;
                grid_row(PieceVec(
                    tiles
                        .into_iter()
                        .map(|t| {
                            rounded_rectangle(3.0)
                                .fill(cell_color(t.index))
                                .grow()
                                .grid_span(t.span as usize)
                        })
                        .collect(),
                ))
                .any()
            },
        ),))
        .spacing(2.0)
        .grow()
        .id("bench-grid"),
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    .grow()
    .any()
}

/// The SwiftUI twin, hosted natively (docs/swiftui.md). Its sliders and row readout live in
/// SwiftUI `@State`; the labels arrive from the same `res::str` keys the Day tab shows, passed as
/// closures so a locale switch re-forms them live (the row templates are `%d`-style because the
/// count is on the Swift side). `.state_key` keeps the hosting view across tab switches and page
/// revisits, so the sliders hold their values like the Day tab's global signals do.
fn swiftui_pane() -> AnyPiece {
    crate::swiftui::BenchGridsView(
        || crate::res::str::bench_parameters().format(),
        || crate::res::str::bench_seed().format(),
        || crate::res::str::bench_count().format(),
        || crate::res::str::bench_rows_one().format(),
        || crate::res::str::bench_rows_other().format(),
    )
    .state_key("bench-grids")
    .grow()
    .id("bench-swiftui")
    .any()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_same_seed_draws_the_same_sequence() {
        let take = |seed| {
            let mut rng = Rng::new(seed);
            (0..16).map(|_| rng.range(1, 4)).collect::<Vec<_>>()
        };
        assert_eq!(take(7), take(7));
        assert_ne!(take(7), take(8), "adjacent seeds must not draw alike");
    }

    #[test]
    fn ranges_stay_in_bounds() {
        let mut rng = Rng::new(3);
        for _ in 0..1000 {
            let v = rng.range(1, 4);
            assert!((1..=4).contains(&v), "{v} out of range");
        }
    }

    #[test]
    fn nearby_cells_get_distinct_colors() {
        // The golden-angle step only earns its keep if neighbours differ visibly; compare each of
        // the first 32 cells against its 3 predecessors.
        for i in 3..32usize {
            for back in 1..=3 {
                let (a, b) = (cell_color(i), cell_color(i - back));
                let delta = (a.r - b.r).abs() + (a.g - b.g).abs() + (a.b - b.b).abs();
                assert!(delta > 0.15, "cells {i} and {} look alike", i - back);
            }
        }
    }

    /// The fixture the SwiftUI twin asserts too (`swiftui/Tests`, `PortParityTests`). Both sides
    /// pin the SAME literal sequence, so a transliteration slip in either implementation — a
    /// wrapping multiply that isn't wrapping, an off-by-one in the range — fails a test instead of
    /// quietly making the two tabs draw different pictures and the comparison meaningless.
    #[test]
    fn seed_one_draws_the_pinned_span_sequence() {
        let mut rng = Rng::new(1);
        let spans: Vec<u32> = (0..12).map(|_| rng.range(1, 4)).collect();
        assert_eq!(spans, vec![2, 1, 1, 4, 2, 2, 1, 1, 4, 1, 2, 1]);
    }

    /// The invariant the whole page rests on: every row covers exactly [`COLUMNS`] columns, so
    /// the grid tiles the pane's width with no ragged edge — including the final row, whose last
    /// tile absorbs the remainder. A screenshot shows this at one size; this shows it for a sweep
    /// of counts and seeds at once.
    #[test]
    fn every_row_covers_exactly_twelve_columns() {
        for seed in [0, 1, 42, 999] {
            for count in [1, 2, 7, 48, 120, 601, 2000] {
                for (i, row) in pack(seed, count).iter().enumerate() {
                    let covered: u32 = row.tiles.iter().map(|t| t.span).sum();
                    assert_eq!(covered, COLUMNS, "seed {seed}, count {count}, row {i}");
                }
            }
        }
    }

    /// Every tile is placed exactly once, in order — the patchwork shows the whole count, and a
    /// tile's index (hence its color) is stable for a given seed.
    #[test]
    fn every_tile_is_placed_once_in_order() {
        let placed: Vec<usize> = pack(7, 300)
            .iter()
            .flat_map(|r| r.tiles.iter().map(|t| t.index))
            .collect();
        assert_eq!(placed, (0..300).collect::<Vec<_>>());
    }

    /// The row counts the walkthrough asserts, pinned here so a generator change fails in `cargo
    /// test` rather than as a puzzling dayscript diff on some target.
    #[test]
    fn known_parameters_pack_to_known_row_counts() {
        assert_eq!(pack(1, 48).len(), 10);
        assert_eq!(pack(1, 240).len(), 45);
    }
}
