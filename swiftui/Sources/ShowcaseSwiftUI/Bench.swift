//  Bench.swift — the deterministic generator behind the Benchmark page's SwiftUI twin, ported
//  from Day-Bench (`versus/swiftui/DayBenchSwiftUI/Bench.swift`, itself a line-for-line port of
//  the Rust `bench.rs` that src/pages/benchmark.rs carries).
//
//  The generator is transliterated rather than rewritten idiomatically: the same LCG constants,
//  the same discarded low byte, the same golden-angle color sequence. Swift's `&*`/`&+` are Rust's
//  `wrapping_mul`/`wrapping_add`, so both tabs draw byte-identical patchworks from the same seed —
//  which is what makes the two implementations comparable at all. The parity tests pin the same
//  literal fixtures the Rust tests do.

import SwiftUI

/// A 32-bit linear congruential generator. Not statistically strong — it does not need to be.
/// What it needs is to be cheap, reproducible, and identical everywhere (`benchmark.rs`: `Rng`).
struct Rng {
    private var state: UInt32

    /// Seed the generator. The multiply spreads small, adjacent seeds (0, 1, 2 — what a slider
    /// produces) across the state space, so consecutive seeds look unrelated instead of drawing
    /// near-identical layouts.
    init(seed: UInt32) {
        state = seed &* 2_654_435_769 &+ 1
    }

    /// The next raw value. The high bits are the well-mixed ones in an LCG, so the low byte is
    /// discarded rather than returned — taking `% n` of the raw state would expose its short
    /// low-bit cycles as visible banding in the patchwork.
    private mutating func next() -> UInt32 {
        state = state &* 1_664_525 &+ 1_013_904_223
        return state >> 8
    }

    /// A value in `lo...hi`.
    mutating func range(_ lo: UInt32, _ hi: UInt32) -> UInt32 {
        precondition(lo <= hi, "empty range \(lo)...\(hi)")
        return lo + next() % (hi - lo + 1)
    }
}

/// The color for cell `i`: a predictable sequence in which no two nearby cells collide.
///
/// Stepping the hue by the golden angle (137°) is the standard trick for a sequence whose
/// consecutive entries are as far apart on the wheel as possible — 0°, 137°, 274°, 51°, 188° —
/// so neighbours stay distinct however the patchwork packs them, and cell `i` is always the same
/// color for a given `i`.
func cellColor(_ i: Int) -> Color {
    let hue = (UInt32(truncatingIfNeeded: i) &* 137) % 360
    return hsl(Double(hue), 0.62, 0.56)
}

/// HSL → RGB, the standard piecewise conversion. Arithmetic only (no transcendentals), so it
/// gives bit-identical colors to the Day tab. Built in the sRGB space explicitly: Day hands its
/// backends device sRGB components, and SwiftUI's default `Color(red:green:blue:)` is sRGB too,
/// but naming it keeps the two from drifting if a default ever changes.
private func hsl(_ hDeg: Double, _ s: Double, _ l: Double) -> Color {
    let c = (1.0 - abs(2.0 * l - 1.0)) * s
    let h = hDeg / 60.0
    let x = c * (1.0 - abs(h.truncatingRemainder(dividingBy: 2.0) - 1.0))
    let (r, g, b): (Double, Double, Double)
    switch UInt32(h) {
    case 0: (r, g, b) = (c, x, 0.0)
    case 1: (r, g, b) = (x, c, 0.0)
    case 2: (r, g, b) = (0.0, c, x)
    case 3: (r, g, b) = (0.0, x, c)
    case 4: (r, g, b) = (x, 0.0, c)
    default: (r, g, b) = (c, 0.0, x)
    }
    let m = l - c / 2.0
    return Color(.sRGB, red: r + m, green: g + m, blue: b + m, opacity: 1.0)
}
