// swift-tools-version:6.0
// The showcase's own SwiftUI code (docs/swiftui.md): an ordinary SwiftPM package, declared in
// Cargo.toml's [package.metadata.day.ios/macos] swift-packages. `day build` compiles it into the
// generated DayPieces module on the Apple legs and wraps its public views in hosting views;
// day-build generates the matching typed Rust constructors (`crate::swiftui::BenchGridsView`).
// `swift test` runs the parity fixtures against the Rust originals.
import PackageDescription

let package = Package(
    name: "ShowcaseSwiftUI",
    // macOS 13 for SwiftUI `Grid`/`GridRow`, the floor the Cargo.toml metadata raises the Mac app
    // to. iOS stays at the app's own 15.0: GridsView falls back to stacks where `Grid` is missing.
    platforms: [.macOS("13.0"), .iOS("15.0")],
    products: [.library(name: "ShowcaseSwiftUI", targets: ["ShowcaseSwiftUI"])],
    targets: [
        .target(name: "ShowcaseSwiftUI"),
        .testTarget(name: "ShowcaseSwiftUITests", dependencies: ["ShowcaseSwiftUI"]),
    ]
)
