# Day Showcase

The demonstration app for [Day](https://daybrite.dev): every implemented piece behind a native
navigation host — stack presentation on mobile, sidebar + detail split on desktop. What you see is
real UIKit on iPhone, real Material Components on Android, real AppKit on a Mac, drawn from one
Rust codebase with no web view and no custom rendering.

Twenty-five screens cover controls, text and text areas, canvas and shapes, grid, list, tabs, stack
navigation, menus and dialogs, toolbars, preferences, date and time, localization, resources,
device and sensors, platform services, focus, animation, web view, map, refresh, and crash
reporting. The whole app is translated into English, French, Arabic and Simplified Chinese, and
switching language at runtime relays every screen — including the right-to-left layout Arabic
requires.

This app is also Day's own integration test. The `dayscript/walkthrough.yaml` flow drives all 407
steps of it on every platform, in every theme and locale, and the screenshots it captures are what
the [gallery](https://daybrite.dev/gallery) shows.

## Run the latest release

You don't need a Rust toolchain to see the app. One command downloads the newest release for your
system and runs it:

**macOS and Linux**

```sh
curl -fsSL https://github.com/daybrite/Day-Showcase/releases/latest/download/launch.sh | bash
```

**Windows** (PowerShell)

```powershell
irm https://github.com/daybrite/Day-Showcase/releases/latest/download/launch.ps1 | iex
```

Each script picks the build for the machine it's on: the signed, notarized `.dmg` on macOS, the
AppImage on Linux (the Qt build under KDE, Plasma or LXQt, the GTK build otherwise), the per-user
installer on Windows. All of them print what they're about to download and where, and ask before
doing it. macOS and Linux run the app straight out of a temporary directory and leave nothing
behind; Windows installs into a temporary folder without an admin prompt and prints the uninstall
line when it's done. Pass `--yes` to skip the confirmation (`bash -s -- --yes` when piping) or
`--target linux-qt` to override the choice; on Windows the flag is `-Yes`, or set
`DAY_LAUNCH_YES=1`.

The [releases page](https://github.com/daybrite/Day-Showcase/releases) has everything else: the
iOS `.ipa`, the Android `.apk` and `.aab`, the HarmonyOS `.hap`, `SHA256SUMS`, and an SBOM and
build-provenance record beside every package.

## Platforms

Declared shipping targets (`Day.toml`): `macos-appkit`, `macos-gtk`, `macos-qt`, `ios-uikit`,
`android-mdc`, `harmony-arkui`. CI additionally builds `windows-xaml`, `linux-gtk`, `linux-qt` and
`web-dom`.

## Day dependency

`Cargo.toml` resolves every day crate from git (`https://github.com/daybrite/day.git`), so the
project builds on CI and on machines without a day checkout. To develop against a local checkout
instead, let the CLI write the `[patch]` table for you:

```sh
day patch --local /path/to/day
```

That writes the gitignored `.cargo/config.toml` and then verifies the result — a hand-written table
that misses one direct dependency does not fail, it silently resolves that crate from the git cache
and builds a mixture of your checkout and a published release. `day patch --check` alone re-runs
just the verification. [Developing Day and an app together](https://daybrite.dev/docs/local-development)
covers the full workflow: changing a day crate and its showcase demonstration in the same sitting.

## Run it

Day compiles one backend per binary, so pick a target when you build or launch:

```sh
day doctor                                   # check toolchains
day launch -p macos-appkit                   # build + run
day launch -p ios-uikit                      # needs a booted Simulator
JAVA_HOME=$(brew --prefix openjdk@21)/libexec/openjdk.jdk/Contents/Home \
  day launch -p android-mdc                  # needs JDK 21 + a running emulator/device
```

## The walkthrough

```sh
day launch -p macos-appkit --script dayscript/walkthrough.yaml
day launch -p macos-appkit --script dayscript/walkthrough.yaml \
  --themes "light dark" --locales "en fr ar zh-CN"     # the full capture matrix
```

Screenshots land under `build/day/screenshots/<target>/<variant>/`. The other flows in
`dayscript/` exercise window management, the clipboard round-trip, and crash reporting.

## Store listing

`store/<locale>/` holds the App Store and Google Play copy, one directory per locale, keyed the
same way `resource/locales/` is. `day store stage` generates the fastlane trees a release uploads;
`day lint` holds the text to each store's length limits and checks that the listing's locales match
the app's own translations.
