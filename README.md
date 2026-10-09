# Day Showcase

The catalog app for [Day](https://daybrite.dev): every implemented piece behind a native
navigation host, from one Rust codebase, rendered with the platform's own widgets on Mac, iPhone,
Android, Windows, Linux, HarmonyOS, and the web. The switch on the Controls page is a `UISwitch`
on iPhone, a `MaterialSwitch` on Android, and an `NSSwitch` on a Mac.

<p align="center">
  <kbd><img src="https://showcase.daybrite.dev/gallery/macos-appkit/light/controls.png" width="760" alt="The Controls page on macOS"></kbd>
</p>

## Run it in one command

Install the `day` CLI, then let it clone, build, and launch the app for your desktop:

```sh
cargo install day-cli
day launch --git https://github.com/daybrite/Day-Showcase.git
```

`day doctor` lists what your platform's toolkit needs and prints the install command for anything
missing. The launch prints where it put the checkout, so you can open the code and change it.

No Rust toolchain handy? The latest release downloads and runs itself. On macOS and Linux:

```sh
curl -fsSL https://github.com/daybrite/Day-Showcase/releases/latest/download/launch.sh | bash
```

On Windows, in PowerShell:

```powershell
irm https://github.com/daybrite/Day-Showcase/releases/latest/download/launch.ps1 | iex
```

Each script picks the build for the machine it is on, prints what it is about to download, and
asks before doing it. The [releases page](https://github.com/daybrite/Day-Showcase/releases) has
the rest: the iOS `.ipa`, the Android `.apk` and `.aab`, the HarmonyOS `.hap`, checksums, and an
SBOM beside every package. The web build runs at
[showcase.daybrite.dev/webapp](https://showcase.daybrite.dev/webapp/).

## What is in it

Thirty-three screens in eight groups, each a working demo of the pieces it names: an overview;
the controls (a catalogue of every control Day ships, text, text editing, date and time, focus);
layout and grid; navigation and window chrome; data (list, tree, model, query); graphics and
media (canvas, charts, animation, resources, video, Lottie, map, web view); the platform (device and
sensors, network and HTTP, notifications and badge, speech, sound and haptics, files and storage); and
the app's own tooling (localization, scripting, tweaks, benchmark, crash reporting). A page whose
central feature a platform cannot run is left out of that platform's sidebar; a section a
platform cannot run stays, with a note.

<p align="center">
  <kbd><img src="https://showcase.daybrite.dev/gallery/ios-uikit/iphone/light/home.png" width="200" alt="The home list on iPhone"></kbd>
  <kbd><img src="https://showcase.daybrite.dev/gallery/ios-uikit/iphone/light/controls.png" width="200" alt="Controls on iPhone"></kbd>
  <kbd><img src="https://showcase.daybrite.dev/gallery/ios-uikit/iphone/light/canvas.png" width="200" alt="Canvas and shapes on iPhone"></kbd>
  <kbd><img src="https://showcase.daybrite.dev/gallery/ios-uikit/iphone/light/tree.png" width="200" alt="The tree piece on iPhone"></kbd>
</p>

- **Controls, Text, Text areas, Focus.** Buttons, toggles, sliders, pickers, and fields, each bound
  two ways to a signal; styled text, selection, editing, and live syntax highlighting.
- **Canvas & shapes, Animation, Grid, Layout.** Paths, gradients, transforms, and gestures on a
  native drawing surface, next to the layout containers. Animation includes 1–250 glass marbles
  with randomized launches and collisions against all four canvas edges. The native frame clock
  drives fixed-step physics; minimum/maximum/average callback FPS uses a rolling two-second window,
  excludes paused intervals, and starts fresh on each bounce, count change, or Reset. Marbles move
  independently (no inter-ball collisions), and frame requests stop when they settle.
- **List, Tree, Tabs, Stack, Refresh.** The native recycling list and outline, tab and stack
  navigation, a fullscreen cover, and pull to refresh.
- **Menus & dialogs, Toolbars, Preferences.** The platform's own chrome, including a preferences
  window on the desktop.
- **Model, Query, Localization, Resources, Date & time.** Observable models over SQLite, live
  queries, four languages switched at runtime, typed resources, and native date pickers.
- **Device & sensors, Platform services, Media, Web view, Tweaks, Crash reporting.** Battery,
  connectivity, live accelerometer traces, notifications and permissions, a media player, the
  system web view, per-platform widget tweaks, and a crash report you can trigger on purpose.

Every screen is translated into English, French, Arabic, and Simplified Chinese. Switching the
language at runtime lays every screen out again, mirrored for Arabic.

<p align="center">
  <kbd><img src="https://showcase.daybrite.dev/gallery/macos-appkit/light-ar/localization.png" width="360" alt="The Localization page in Arabic on macOS"></kbd>
  <kbd><img src="https://showcase.daybrite.dev/gallery/macos-appkit/dark/home.png" width="360" alt="The home page in dark mode on macOS"></kbd>
</p>

## The same code on every platform

The app is also Day's own integration test. `dayscript/walkthrough.yaml` drives every screen on
every target, in both themes and all four locales, and the captures it takes are the
[gallery](https://daybrite.dev/gallery/Day-Showcase/) on daybrite.dev.

| Windows · XAML | Linux · GTK | Linux · Qt |
|:---:|:---:|:---:|
| <kbd><img src="https://showcase.daybrite.dev/gallery/windows-xaml/light/list.png" width="300" alt="List on Windows"></kbd> | <kbd><img src="https://showcase.daybrite.dev/gallery/linux-gtk/light/list.png" width="300" alt="List on GTK"></kbd> | <kbd><img src="https://showcase.daybrite.dev/gallery/linux-qt/light/list.png" width="300" alt="List on Qt"></kbd> |

| Web · DOM | Android · Material | HarmonyOS · ArkUI |
|:---:|:---:|:---:|
| <kbd><img src="https://showcase.daybrite.dev/gallery/web-dom/light/menus.png" width="300" alt="Menus and dialogs in the browser"></kbd> | <kbd><img src="https://showcase.daybrite.dev/gallery/android-mdc/phone/light/menus.png" width="150" alt="Menus and dialogs on Android"></kbd> | <kbd><img src="https://showcase.daybrite.dev/gallery/harmony-arkui/phone/light/menus.png" width="150" alt="Menus and dialogs on HarmonyOS"></kbd> |

## Build from a clone

Day compiles one toolkit backend per binary, so name a target when you build or launch. Every
target the app ships is listed in `Day.toml`.

```sh
day doctor                       # toolchains present and missing, with fixes
day launch -p macos-appkit       # build + run
day launch -p ios-uikit          # needs a booted Simulator
day launch -p android-mdc        # needs a JDK and a running emulator or device
day launch -p web-dom            # serves the WebAssembly build locally
```

Run the walkthrough on one target, or the full capture matrix:

```sh
day launch -p macos-appkit --script dayscript/walkthrough.yaml
day launch -p macos-appkit --script dayscript/walkthrough.yaml \
  --themes "light dark" --locales "en fr ar zh-CN"
```

Captures land under `build/day/screenshots/<target>/<variant>/`. The other flows in `dayscript/`
exercise window management, the clipboard round trip, and crash reporting.

To build against a local `day` checkout instead of the pinned git revision, let the CLI write and
verify the patch table. [Developing Day and an app together](https://daybrite.dev/docs/local-development)
covers the workflow of changing a `day` crate and its showcase demonstration in one sitting:

```sh
day patch --local /path/to/day
```

### CI and framework updates

This app's CI uses `update-day-deps: true` in `daybrite/actions`. Preflight resolves the latest
commit on the Day branch the app tracks, then pins every build in that run to that same commit.
Local `day patch` overrides are gitignored and do not affect CI.

When changing the framework and Showcase together, publish the framework fix before starting
Showcase CI. A run already in progress keeps its original Day revision, even if the framework
branch advances while its builds are running. Check preflight's summary for the chosen revision
and each build's **Track the day revision preflight resolved** step to confirm what was tested.
To pick up a newer revision, start a new **ci** run from GitHub Actions, or choose **Re-run all
jobs** on a completed run. **Re-run failed jobs** reuses the successful preflight's old revision.

## Inside the code

- `src/lib.rs` is `root()`: the typed-route navigation host and the `Section` list every screen
  hangs off.
- `src/pages/` has one module per screen, named after the subsystem it demonstrates.
- `swiftui/` is the Swift package whose views the SwiftUI page embeds on Apple platforms, with
  typed Rust constructors generated at build time.
- `resource/locales/` carries the Fluent strings for `en`, `fr`, `ar`, and `zh-CN`.
- `store/<locale>/` holds the App Store and Google Play copy; `day store stage` generates the
  fastlane trees a release uploads, and `day lint` holds the text to each store's limits.
- `platform/` holds the thin native host projects the mobile targets build through.

Day Showcase is open source under the Apache-2.0 license.


The native Query page opens and seeds its database on `DatabaseWorker`, evaluates search,
FTS and geographic filters on that queue, and publishes owned results to the UI. Star edits
perform their read-modify-write on the same queue. `dayscript/query-worker.yaml` checks the
asynchronous query lifecycle and selection; it waits for row identity when successive searches
have the same result count. The web Query demo retains its in-memory model.

### Network lab

The Network page has one isolated `day-part-http::Session`. Select Native or the optional
reqwest provider (enabled in this app on supported native targets), then use the same HTTP or
WebSocket controls with interception on or off. HTTP supports editable methods, URLs, bodies,
JSON request headers, idle timeouts, streaming download/upload presets and cancellation.
The simulation section controls per-transfer speed, seeded failure rate, pause and offline state.
Desktop uses two columns; narrower windows stack the same controls. An animated chart shows
real/simulated upload/download payload rates over the last 30 seconds.

Native defaults talk to the in-app loopback server; web HTTP uses the dev server's same-origin
endpoint. Browser WebSockets require a user-supplied reachable server or the simulated echo.
The reqwest selector is omitted on browser and HarmonyOS builds. These counters cover only
this page's HTTP session, not other frameworks or all device traffic. `dayscript/network.yaml`
exercises native/reqwest HTTP, real and simulated WebSockets, failures and paused cancellation;
the app CI selects it explicitly.
