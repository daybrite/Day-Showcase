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
    /// The exact height of the Parameters block, passed from Rust (the page's PARAMS_HEIGHT)
    /// so the patchwork below starts at the same y as the Day tab's — the pixel the two
    /// implementations are compared at.
    let paramsHeight: Double

    public init(
        parametersLabel: String,
        seedLabel: String,
        countLabel: String,
        rowsOne: String,
        rowsOther: String,
        paramsHeight: Double
    ) {
        self.parametersLabel = parametersLabel
        self.seedLabel = seedLabel
        self.countLabel = countLabel
        self.rowsOne = rowsOne
        self.rowsOther = rowsOther
        self.paramsHeight = paramsHeight
    }

    private var rows: [Row] { pack(seed: UInt32(seed), count: Int(count)) }

    /// The shared label-column width — the widest row label, measured via the preference below.
    /// The Day tab's `labeled` does the same: every label sits trailing-aligned in one column
    /// as wide as the widest, so the sliders all start at the same x.
    @State private var labelColumn: CGFloat?

    /// One parameter row, laid out exactly as the Day tab's `labeled(label, row(slider,
    /// readout))`: the trailing-aligned label column, a 12pt gap (day-pieces' LABELED_GAP),
    /// the slider, an 8pt gap, and a readout whose slot reserves the widest value so the
    /// slider's right edge never shifts as digits change. No `step:` on the slider — on macOS
    /// a stepped SwiftUI slider draws tick marks the Day slider doesn't have — so the binding
    /// snaps to integers instead.
    private func parameterRow(
        _ label: String,
        value: Binding<Double>,
        in range: ClosedRange<Double>,
        widest: String
    ) -> some View {
        HStack(spacing: 12) {
            Text(label)
                .fixedSize()
                .background(GeometryReader { g in
                    Color.clear.preference(key: LabelColumnKey.self, value: g.size.width)
                })
                .frame(width: labelColumn, alignment: .trailing)
            HStack(spacing: 8) {
                Slider(
                    value: Binding(
                        get: { value.wrappedValue },
                        set: { value.wrappedValue = $0.rounded() }
                    ),
                    in: range
                )
                // The Day readout's `reserving`: the widest value sits hidden under the live
                // one, so the slot holds its width — the value leading-aligned inside it, the
                // way Day's Label sits in its reserved box.
                ZStack(alignment: .leading) {
                    Text(widest).hidden()
                    Text("\(UInt32(value.wrappedValue))")
                }
                .monospacedDigit()
            }
        }
        // The Day row's pitch: its slider row stands 21pt tall, and the section stacks rows
        // 10pt apart — pinned so the two tabs' rows land on the same lines.
        .frame(height: 21)
    }

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
                // The Day section card's interior: rows on a 10pt pitch (its column spacing),
                // the row-count line leading-aligned below the sliders.
                VStack(alignment: .leading, spacing: 10) {
                    parameterRow(seedLabel, value: $seed, in: 0...999, widest: "999")
                    parameterRow(countLabel, value: $count, in: 0...2000, widest: "2000")
                    HStack {
                        Text(String(format: rows.count == 1 ? rowsOne : rowsOther, rows.count))
                            .font(.footnote)
                        Spacer()
                    }
                }
                .onPreferenceChange(LabelColumnKey.self) { labelColumn = $0 }
                // Measured against the Day tab's card (its interior padding is 14pt; a
                // GroupBox's own insets fall short of that by different amounts per edge).
                .padding(.top, 11)
                .padding(.leading, 13.5)
                .padding(.trailing, 5.5)
                // Greedy, so the CARD stretches to the fixed slot below rather than hugging
                // and leaving slack under it — the Day tab's form fills its slot the same way.
                .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
            }
            // The SAME fixed height the Day tab reserves (its PARAMS_HEIGHT, passed through the
            // initializer): the box hugs its content, the frame pins the SLOT, and the grid
            // below therefore begins at the identical y under either tab. Top-aligned so any
            // platform-to-platform slack in the box's natural height opens downward, never by
            // re-centring the card.
            .frame(maxWidth: .infinity, minHeight: paramsHeight, maxHeight: paramsHeight, alignment: .topLeading)

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

/// The widest parameter label, folded across the rows — how the two labels share one
/// trailing-aligned column the way the Day form's shared label column does.
private struct LabelColumnKey: PreferenceKey {
    static let defaultValue: CGFloat = 0
    static func reduce(value: inout CGFloat, nextValue: () -> CGFloat) {
        value = max(value, nextValue())
    }
}
