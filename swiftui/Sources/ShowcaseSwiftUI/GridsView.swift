//  GridsView.swift — the SwiftUI twin of the Benchmark page's Day-native patchwork, ported from
//  Day-Bench (`versus/swiftui/DayBenchSwiftUI/GridsView.swift`).
//
//  What this measures. Every cell is flexible on both axes and every row is packed to exactly
//  `columns` columns, so the grid can only resolve by negotiating all of it at once: column widths
//  come from the flexible cells, spans redistribute across the columns they cover, and row heights
//  stretch to consume the leftover height. Nothing here can be solved per-cell — changing "Total
//  Count" repacks the rows, which changes the spans, which changes the column split.
//
//  The Day original builds this from `grid`/`grid_row`/`.grid_span(n)`/`.grow()`; the SwiftUI
//  counterparts are `Grid`/`GridRow`/`.gridCellColumns(n)` and a maximal frame. The packing
//  function below is the same algorithm, transliterated, so both tabs lay out the same patchwork.
//
//  Unlike the standalone Day-Bench app this renders as a SUBVIEW (a tab of the Benchmark page), so
//  it carries no title, caption, or navigation chrome — the page's Rust heading owns those — and
//  its labels arrive localized from Rust through the view's initializer (the page passes the same
//  res::str values its own tab shows).

import SwiftUI

/// Columns every row must fill exactly. Twelve divides by 2, 3, 4, and 6, so spans of 1–4 can
/// close a row on many different boundaries instead of forcing one repeating rhythm.
private let columns: UInt32 = 12
/// The widest a single tile may span. Wider spans make the span-distribution path do more work.
private let maxSpan: UInt32 = 4
/// Tiles the tab opens with — matches the Day tab so the two cold-start identically.
private let defaultCount: Double = 48

/// One tile: its index (which fixes its color) and how many columns it covers.
struct Tile: Identifiable, Equatable {
    let index: Int
    let span: UInt32
    var id: Int { index }
}

/// One packed row, keyed so `ForEach` rebuilds exactly the rows that moved.
struct Row: Identifiable, Equatable {
    let seed: UInt32
    let position: Int
    let tiles: [Tile]
    var id: some Hashable { [Int(seed), position] }
}

/// Pack `count` tiles into rows of exactly `columns` columns.
///
/// The span is drawn from the generator and then clamped to what is left in the row, so a row
/// always closes on the column boundary rather than overflowing into the next. The final row is
/// short by construction — its last tile absorbs the remainder, which keeps the "every row fills
/// the width" invariant true for the whole grid rather than all-but-one of it.
func pack(seed: UInt32, count: Int) -> [Row] {
    var rng = Rng(seed: seed)
    var rows: [Row] = []
    var tiles: [Tile] = []
    var used: UInt32 = 0
    for index in 0..<count {
        let remaining = columns - used
        let span = min(rng.range(1, maxSpan), remaining)
        tiles.append(Tile(index: index, span: span))
        used += span
        if used == columns {
            rows.append(Row(seed: seed, position: rows.count, tiles: tiles))
            tiles.removeAll()
            used = 0
        }
    }
    if let last = tiles.last {
        tiles[tiles.count - 1] = Tile(index: last.index, span: last.span + (columns - used))
        rows.append(Row(seed: seed, position: rows.count, tiles: tiles))
    }
    return rows
}

/// The Grids benchmark as an embeddable pane: the parameter controls above, the patchwork filling
/// everything below. Day exports it via the generated `crate::swiftui::BenchGridsView(…)` binding
/// (docs/swiftui.md); the labels are `%d`-style templates for the two row-count forms so the
/// readout localizes even though the count lives in SwiftUI `@State`.
public struct BenchGridsView: View {
    @State private var seed: Double = 1
    @State private var count: Double = defaultCount

    let parametersLabel: String
    let seedLabel: String
    let countLabel: String
    let rowsOne: String
    let rowsOther: String

    public init(
        parametersLabel: String,
        seedLabel: String,
        countLabel: String,
        rowsOne: String,
        rowsOther: String
    ) {
        self.parametersLabel = parametersLabel
        self.seedLabel = seedLabel
        self.countLabel = countLabel
        self.rowsOne = rowsOne
        self.rowsOther = rowsOther
    }

    private var rows: [Row] { pack(seed: UInt32(seed), count: Int(count)) }

    public var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            // Parameters. Both are sliders because a benchmark is driven by sweeping a value, not
            // by typing one.
            //
            // `GroupBox`, not `Form`: Day's `form`/`section` is a card that hugs its content, and
            // so is a GroupBox. A `Form` is backed by a scroll view that claims all the height it
            // is offered, which would starve the grid below — capping it with a fixed frame just
            // trades that for dead space. The card must hug so the grid gets exactly the rest,
            // which is the geometry the Day tab lays out.
            GroupBox(parametersLabel) {
                LabeledContent(seedLabel) {
                    HStack(spacing: 8) {
                        Slider(value: $seed, in: 0...999, step: 1)
                        Text("\(UInt32(seed))")
                    }
                }
                LabeledContent(countLabel) {
                    HStack(spacing: 8) {
                        Slider(value: $count, in: 0...2000, step: 1)
                        Text("\(UInt32(count))")
                    }
                }
                HStack {
                    Text(String(format: rows.count == 1 ? rowsOne : rowsOther, rows.count))
                        .font(.footnote)
                    Spacer()
                }
            }

            // The patchwork. Every tile grows on both axes, so the grid resolves columns by the
            // flexible share and stretches rows into the leftover height — it fills the pane
            // exactly, the same invariant the Day tab holds.
            Grid(horizontalSpacing: 2, verticalSpacing: 2) {
                ForEach(rows) { row in
                    GridRow {
                        ForEach(row.tiles) { tile in
                            RoundedRectangle(cornerRadius: 3)
                                .fill(cellColor(tile.index))
                                .frame(maxWidth: .infinity, maxHeight: .infinity)
                                .gridCellColumns(Int(tile.span))
                        }
                    }
                }
            }
            .frame(maxWidth: .infinity, maxHeight: .infinity)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .leading)
    }
}
