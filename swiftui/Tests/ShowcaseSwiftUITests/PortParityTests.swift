//  PortParityTests.swift — the port is only useful if it is exact.
//
//  The Benchmark page hosts this package's view beside the Day-native implementation, so the two
//  are only comparable if they lay out the same thing. These are the Rust tests from
//  `src/pages/benchmark.rs` transliterated, PLUS the numbers those Rust tests pin.
//  `pack(seed: 1, count: 48)` producing 10 rows here and 10 rows there is what makes the two tabs
//  the same benchmark; if a transliteration slipped (a `&*` that should have been `*`, an
//  off-by-one in the clamp), these fail rather than silently comparing two different pictures.
//  Run with `swift test` from swiftui/.

import XCTest
import SwiftUI
@testable import ShowcaseSwiftUI

final class PortParityTests: XCTestCase {
    /// `benchmark.rs`: `the_same_seed_draws_the_same_sequence`.
    func testTheSameSeedDrawsTheSameSequence() {
        func take(_ seed: UInt32) -> [UInt32] {
            var rng = Rng(seed: seed)
            return (0..<16).map { _ in rng.range(1, 4) }
        }
        XCTAssertEqual(take(7), take(7))
        XCTAssertNotEqual(take(7), take(8), "adjacent seeds must not draw alike")
    }

    /// `benchmark.rs`: `ranges_stay_in_bounds`.
    func testRangesStayInBounds() {
        var rng = Rng(seed: 3)
        for _ in 0..<1000 {
            let v = rng.range(1, 4)
            XCTAssertTrue((1...4).contains(v), "\(v) out of range")
        }
    }

    /// `benchmark.rs`: `every_row_covers_exactly_twelve_columns` — the invariant the pane rests on.
    func testEveryRowCoversExactlyTwelveColumns() {
        for seed in [UInt32(0), 1, 42, 999] {
            for count in [1, 2, 7, 48, 120, 601, 2000] {
                for (i, row) in pack(seed: seed, count: count).enumerated() {
                    let covered = row.tiles.reduce(UInt32(0)) { $0 + $1.span }
                    XCTAssertEqual(covered, 12, "seed \(seed), count \(count), row \(i)")
                }
            }
        }
    }

    /// `benchmark.rs`: `every_tile_is_placed_once_in_order`.
    func testEveryTileIsPlacedOnceInOrder() {
        let placed = pack(seed: 7, count: 300).flatMap { $0.tiles.map(\.index) }
        XCTAssertEqual(placed, Array(0..<300))
    }

    /// `benchmark.rs`: `known_parameters_pack_to_known_row_counts`. These two numbers are the
    /// contract between the two tabs — the Rust test asserts the same literals, and the
    /// walkthrough asserts them against the live readout.
    func testKnownParametersPackToKnownRowCounts() {
        XCTAssertEqual(pack(seed: 1, count: 48).count, 10)
        XCTAssertEqual(pack(seed: 1, count: 240).count, 45)
    }

    /// The first spans of seed 1, as the Rust generator emits them. Transliteration bugs in the
    /// LCG show up here even when the row COUNTS happen to coincide.
    func testSeedOneDrawsTheRustSpanSequence() {
        var rng = Rng(seed: 1)
        let spans = (0..<12).map { _ in rng.range(1, 4) }
        XCTAssertEqual(spans, [2, 1, 1, 4, 2, 2, 1, 1, 4, 1, 2, 1])
    }
}
