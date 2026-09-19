//! The Day showcase (DESIGN.md Appendix A): every implemented piece behind a native navigation
//! host (docs/navigation.md): stack presentation on mobile, sidebar + detail split on desktop.
//!
//! This crate root wires the navigation together in [`root`] and owns the app-wide lifecycle
//! plumbing; each navigation destination lives in its own module under [`pages`], and reusable
//! pieces shared by several pages live in [`widgets`].

use day::prelude::*;

mod commands;
mod pages;
mod palette;
pub(crate) mod support;
mod widgets;

use crate::pages::*;

// The mobile / embedded entry point (DESIGN.md §17.4): the iOS and macOS Runners bind `day_main`,
// DayBridge binds the `Java_…` natives, the HarmonyOS ArkTS host binds `day_arkui_start`, and the
// web host page binds `day_dom_main`. One macro expands to all of them, and to nothing at all on a
// plain cargo desktop build, where src/main.rs is the entry instead.
day::day_start!("Day Showcase", root);

/// Typed constants for the files under `resource/`, generated at build time by `day-build` (§18.5):
/// `res::images::<stem>`, `res::assets::<file>`, `res::fonts::<family>`. The showcase references its
/// bundled resources through these, so a renamed/removed file is a compile error, not a runtime miss.
pub mod res {
    include!(concat!(env!("OUT_DIR"), "/day_resources.rs"));
}

/// Typed constructors for the SwiftUI views exported by the `swiftui/` package (docs/swiftui.md),
/// generated at build time by `day-build` from the `[package.metadata.day.ios/macos]` declaration:
/// `crate::swiftui::BenchGridsView(…)` mirrors `public struct BenchGridsView`'s init exactly, so a
/// renamed view or a changed parameter is a compile error here, not a runtime miss there.
pub mod swiftui {
    include!(concat!(env!("OUT_DIR"), "/day_swiftui.rs"));
}

pub(crate) fn lifecycle_log() -> Signal<String> {
    LifecycleLog::app().0
}

/// Everything a single window owns (docs/state.md): which page it is on, and the per-page
/// editor state that belongs to a view rather than to the app.
///
/// The showcase keeps two tiers. App-wide (`Ambient::app`) are the things one process has one
/// of: the lifecycle and menu logs, the SQLite container behind the Query page, the model
/// store, and the persisted preferences (starred pages, appearance). Per-window are
/// the ones a second window should get its own of: the sidebar selection, the scripting buffer,
/// the toolbar demo's controls, the benchmark's parameters and the webview's fields.
#[derive(Clone, Copy)]
pub(crate) struct Scene {
    /// The sidebar's selection: the app's routing signal, hoisted so every surface can read
    /// it reactively (see `commands::section`).
    pub(crate) section: Signal<Option<crate::Section>>,
    /// The Scripting page's working buffer, its saved baseline, and the file it came from.
    pub(crate) script_buf: Signal<String>,
    pub(crate) script_baseline: Signal<String>,
    pub(crate) script_file: Signal<Option<String>>,
    /// The Benchmark page's `(scale, count)` parameters.
    pub(crate) bench: (Signal<f64>, Signal<f64>),
    /// The WebView page's `(url, script, result)`, its selected tab, and the embedded tab's
    /// status.
    pub(crate) web: (Signal<String>, Signal<String>, Signal<String>),
    pub(crate) web_tab: Signal<usize>,
    pub(crate) web_embed_status: Signal<(u8, String)>,
}

impl Ambient for Scene {
    fn create() -> Self {
        Scene {
            section: Signal::new(
                std::env::var("DAY_DEMO_ROUTE").ok().and_then(|r| {
                    crate::Section::from_key(r.split(['/', '?']).next().unwrap_or(""))
                }),
            ),
            script_buf: Signal::new(day::record::script()),
            // Seeded to the initial buffer, so a page with nothing new is not dirty.
            script_baseline: Signal::new(day::record::script()),
            script_file: Signal::new(None),
            bench: (
                Signal::new(1.0),
                Signal::new(crate::pages::benchmark::DEFAULT_COUNT),
            ),
            web: (
                Signal::new("https://daybrite.dev".to_string()),
                Signal::new("document.title".to_string()),
                Signal::new(String::new()),
            ),
            web_tab: Signal::new(0usize),
            web_embed_status: Signal::new((0, String::new())),
        }
    }
}

/// This window's `Scene`: the ambient one while a piece builds, the focused window's when a
/// command runs later from a handler that belongs to no scope (docs/state.md).
pub(crate) fn scene() -> Scene {
    try_scene().expect("no window is open, so there is no Scene to act on")
}

/// [`scene`] without the panic, for the app-wide surfaces (the menu bar) whose builders can be
/// evaluated before any window has built, which is where the Android app-bar menu is lowered
/// from. A command with no front window has nothing to act on and reports so.
pub(crate) fn try_scene() -> Option<Scene> {
    Scene::try_ambient().or_else(Scene::focused)
}

/// The most recent app-lifecycle phase, shown live on the About page (docs/lifecycle.md).
/// App-wide: it records the APP's phases, not a window's.
#[derive(Clone, Copy)]
struct LifecycleLog(Signal<String>);

impl Ambient for LifecycleLog {
    fn create() -> Self {
        LifecycleLog(Signal::new("—".into()))
    }
}

/// Register app-lifecycle handlers (docs/lifecycle.md). Call this from `main` before `day::launch`
/// so the launch phases are captured. Each handler logs to the console and to a live UI readout.
///
/// The mobile-only phases are registered only where the compiled-in backend actually delivers them,
/// using the compile-time-accurate guard `day::lifecycle::supported(..)`. On desktop those `if`s
/// are `false` and the handlers are never registered, so no "unsupported phase" warning is
/// produced.
pub fn install_lifecycle_handlers() {
    use day::Lifecycle::*;

    // Idempotent: desktop calls this from `main` (to catch WillLaunch), mobile from `root`.
    // A run-once latch, not app state; `Once` says that directly.
    static INSTALLED: std::sync::Once = std::sync::Once::new();
    let mut first = false;
    INSTALLED.call_once(|| first = true);
    if !first {
        return;
    }

    let note = |phase: day::Lifecycle| {
        move || {
            // `info!`, because a lifecycle phase is normal operation (docs/logging.md). This was
            // a `println!` for a reason worth recording: day-android maps fd 1 to logcat INFO and
            // fd 2 to `ERROR`, so `eprintln!` made every phase surface as `E Day` and drowned
            // the level out as a filter. Choosing stdout to mean "info" is exactly the
            // workaround a logging level replaces, and the line now reaches the browser console
            // too, where `println!` was silently dropped.
            info!("lifecycle: {}", phase.name());
            lifecycle_log().set(phase.name().into());
        }
    };

    // Universal phases; every backend delivers these.
    for phase in [
        WillLaunch,
        DidLaunch,
        DidBecomeActive,
        WillResignActive,
        WillTerminate,
    ] {
        day::on_lifecycle(phase, note(phase));
    }
    // Mobile-only phases; guard so we register only where they're delivered (iOS / Android).
    for phase in [
        WillEnterForeground,
        DidEnterBackground,
        DidReceiveMemoryWarning,
    ] {
        if day::lifecycle::supported(phase) {
            day::on_lifecycle(phase, note(phase));
        }
    }
}

day::routes! {
    /// The top-level sections, typed (docs/navigation.md): each variant's key is what deep
    /// links, dayscript, and `current_route()` speak; the `.item(Section::…)` declarations
    /// and any `navigate_to`/`route` call sites are compile-checked against this enum.
    pub(crate) enum Section {
        Controls => "controls",
        Dates => "dates",
        Focus => "focus",
        Text => "text",
        TextAreas => "textareas",
        Toolbars => "toolbars",
        Localization => "localization",
        Canvas => "canvas",
        Animation => "animation",
        Benchmark => "benchmark",
        Grid => "grid",
        Layout => "layout",
        List => "list",
        Tree => "tree",
        DragDrop => "drag-drop",
        Model => "model",
        Query => "query",
        Tabs => "tabs",
        Stack => "stack",
        ContentList => "content-list",
        Media => "media",
        Lottie => "lottie",
        WebView => "webview",
        Menus => "menus",
        System => "system",
        Network => "network",
        Notify => "notify",
        Speech => "speech",
        Camera => "camera",
        Cursors => "cursors",
        Files => "files",
        Scripting => "scripting",
        Resources => "resources",
        Tweaks => "tweaks",
        CrashReporting => "crash",
        Map => "map",
        About => "about",
    }
}

/// The GitHub repository the "Show Source" action opens.
const SOURCE_REPO: &str = "https://github.com/daybrite/Day-Showcase";
/// The git ref "Show Source" links against: `vX.Y.Z` for a tagged release, else `main` for a
/// development build. Baked by `build.rs` from the release pipeline's `GITHUB_REF` (see there).
const SOURCE_REF: &str = env!("DAY_SHOWCASE_SOURCE_REF");

impl Section {
    /// The repo-relative source file whose page this section shows, the "Show Source" target.
    /// Kept exhaustive by the compiler, so a new section must name its file here.
    fn source_file(self) -> &'static str {
        match self {
            Section::About => "src/pages/about.rs",
            Section::Animation => "src/pages/animation.rs",
            Section::Benchmark => "src/pages/benchmark.rs",
            Section::Canvas => "src/pages/canvas.rs",
            Section::ContentList => "src/pages/content_list/mod.rs",
            Section::Controls => "src/pages/controls.rs",
            Section::Cursors => "src/pages/cursors.rs",
            Section::CrashReporting => "src/pages/crash.rs",
            Section::Dates => "src/pages/dates.rs",
            Section::Focus => "src/pages/focus.rs",
            Section::Grid => "src/pages/grid.rs",
            Section::Layout => "src/pages/layout.rs",
            Section::List => "src/pages/list.rs",
            Section::Model => "src/pages/model.rs",
            Section::Localization => "src/pages/localization.rs",
            Section::Map => "src/pages/map.rs",
            Section::Media => "src/pages/media.rs",
            Section::Lottie => "src/pages/media.rs",
            Section::Menus => "src/pages/menus.rs",
            Section::Query => "src/pages/query.rs",
            Section::Resources => "src/pages/resources.rs",
            Section::Scripting => "src/pages/scripting.rs",
            Section::Network => "src/pages/services.rs",
            Section::Notify => "src/pages/services.rs",
            Section::Speech => "src/pages/services.rs",
            Section::Camera => "src/pages/camera.rs",
            Section::Files => "src/pages/services.rs",
            Section::Stack => "src/pages/stack.rs",
            Section::System => "src/pages/system.rs",
            Section::Tabs => "src/pages/tabs.rs",
            Section::Text => "src/pages/text.rs",
            Section::TextAreas => "src/pages/text_areas.rs",
            Section::Toolbars => "src/pages/toolbars.rs",
            Section::Tree => "src/pages/tree.rs",
            Section::DragDrop => "src/pages/drag_drop.rs",
            Section::Tweaks => "src/pages/tweaks.rs",
            Section::WebView => "src/pages/webview.rs",
        }
    }
}

/// Open one section's source on GitHub, pinned to this build's ref (a release tag, else `main`).
///
/// Every caller names its own section: the sidebar rows' context-menu "Show Source"
/// (docs/menus.md) so a right-click on any row works without navigating there first, and the
/// page's own toolbar button (pages/toolbars.rs) because a page-declared command knows which page
/// it is on. Nothing here reads the live route.
pub(crate) fn open_source_of(section: Section) {
    open_url(&format!(
        "{SOURCE_REPO}/blob/{SOURCE_REF}/{}",
        section.source_file()
    ));
}

/// Arm crash reporting (docs/break.md); the Crash Reporting page demonstrates it. Idempotent
/// (day-break's `init` is single-shot); safe to call from every entry point.
pub fn install_crash_reporting() {
    let _ = day_break::Config::new()
        // "Send report" opens a prefilled email to the developer (no server needed).
        .reporter(day_break::EmailReporter::new("crashdemo@daybrite.dev"))
        .init();
}

pub fn root() -> impl Piece {
    // Arm crash capture before the UI mounts so the Crash Reporting page's crashes are recorded.
    install_crash_reporting();
    // Narrate every action to the console for the app's whole life (§14.6): the same lines the
    // Scripting page's recorder echoes, in the same dayscript vocabulary:
    //
    //     dayscript ▸ navigate → dates  "Date & time"
    //     dayscript ▸ tap list-shuffle  "Shuffle"
    //
    // Nothing is retained, so it costs the same as the recorder's observer and never grows. The
    // showcase leaves it on because it is a demonstration: the log is a live reading of what a
    // recording would capture, which makes the Scripting page's output legible before you record
    // anything. Set DAY_LOG_ACTIONS=0 to silence it; the env is read first, so a launch can
    // override the app.
    if std::env::var("DAY_LOG_ACTIONS").as_deref() != Ok("0") {
        // The Scripting page's own controls stay out, exactly as they stay out of a recording.
        day::record::exclude_prefix("scripting-");
        day::record::log_actions(true);
    }
    // Every locale under `resource/locales/` (en, fr, ar, zh-CN), embedded and registered by the
    // generated catalog (§18.5), so adding a language is a new directory, nothing to edit here.
    res::locales::install();
    // Persisted theme/language overrides (docs/windows.md; the launch env wins, so CI variant
    // loops with DAY_THEME/--locale stay deterministic).
    day_piece_settings::apply_startup("showcase.theme", "showcase.locale");
    // Write the bundled sample dayscripts into the app container on launch (absent-only), so the
    // Scripting page's dropdown demonstrates record/playback out of the box (pages/scripting.rs).
    crate::pages::scripting::seed_sample_scripts();
    // The Preferences window (Settings…/⌘, on macOS; primary+`,` elsewhere; a fullscreen
    // cover on backends without windows) and File ▸ New Window / the macOS tab-bar "+"
    // (docs/windows.md). Registered before the menu so its items lower live.
    day::register_preferences_with(
        day::WindowOptions {
            title: crate::res::str::prefs_window_title().format(),
            size: Size::new(520.0, 420.0),
            min_size: None,
            app_name: None,
            ..Default::default()
        },
        pages::preferences_window,
    );
    day::register_new_window(|| window_root(false));
    // Lifecycle handlers (docs/lifecycle.md). On mobile this is the registration point; on desktop
    // `main` already registered them before launch (to also catch WillLaunch); the call is
    // idempotent.
    install_lifecycle_handlers();
    window_root(true)
}

/// One sidebar destination: the row's title and icon, and the page it opens.
///
/// The sidebar's groups, in the order the sidebar shows them: what a page is made of, how
/// pages are arranged, how the app moves between them, what it holds, what it draws, what the
/// OS gives it, and the tooling around the app. One tint per group (palette.rs): on the
/// toolkits whose nav rows ignore section headers and show one flat list, the tint is the only
/// grouping signal a row carries, so neighbouring groups never share a hue family.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Group {
    Overview,
    Controls,
    Layout,
    Navigation,
    Data,
    Graphics,
    Platform,
    App,
}

impl Group {
    /// The header the sidebar shows over the group's first row: a `res::str` accessor, not a
    /// resolved `String`, so it re-resolves on every derive and follows a locale switch.
    fn title(self) -> fn() -> day::LocalizedText {
        match self {
            Group::Overview => crate::res::str::group_overview,
            Group::Controls => crate::res::str::group_controls,
            Group::Layout => crate::res::str::group_layout,
            Group::Navigation => crate::res::str::group_navigation,
            Group::Data => crate::res::str::group_data,
            Group::Graphics => crate::res::str::group_graphics,
            Group::Platform => crate::res::str::group_platform,
            Group::App => crate::res::str::group_app,
        }
    }

    /// The group's icon tint: high-chroma mid-value hues, since a pastel reads as washed out
    /// on a glyph this small (docs/vectors.md).
    fn tint(self) -> Color {
        match self {
            Group::Overview => crate::palette::SKY,
            Group::Controls => crate::palette::TEAL,
            Group::Layout => crate::palette::SKY,
            Group::Navigation => crate::palette::VIOLET,
            Group::Data => crate::palette::AMBER,
            Group::Graphics => crate::palette::CORAL,
            Group::Platform => crate::palette::RUST,
            Group::App => crate::palette::SLATE,
        }
    }
}

/// `Clone` because `.items(…)` re-derives the list on every query keystroke; the fields are a
/// key, two fn pointers and a name, so a clone is cheap.
///
/// A table, not a chain of `.item_icon(…)` calls, because the sidebar is filterable: its rows
/// are derived from the search query, and `.items(…)` wants a list it can re-derive. The table
/// is also what `.destination` looks a key up in, so a row and its page can never drift apart.
#[derive(Clone)]
struct Dest {
    section: Section,
    group: Group,
    /// The generated `res::str` accessor, not a resolved `String`: the title has to be
    /// re-resolved on every derive so the rows re-title (and re-filter) on a locale switch.
    title: fn() -> day::LocalizedText,
    /// A `resource/vectors/` glyph (docs/vectors.md): resolution-independent, staged per backend
    /// as whatever its nav rows load natively (VectorDrawable / catalog entry / raster cache).
    icon: day::VectorName,
    page: fn() -> AnyPiece,
}

/// One derived sidebar row: a destination and, on the first row of each group, the header
/// the nav opens above it.
#[derive(Clone)]
struct Row {
    dest: Dest,
    header: Option<fn() -> day::LocalizedText>,
}

/// Every destination, in the order the sidebar shows them: by group (see [`Group`]), and
/// within a group from the general to the specialised. About is both first and the desktop
/// split's default detail (the split selects the first row when nothing is chosen).
///
/// A page whose central feature this target cannot run is not listed at all (a target with
/// no toolbar has nothing to show on a Toolbars page), while a section inside a page that the
/// target cannot run keeps its banner (support.rs). Two of the four such pages are decided at
/// compile time because their crates carry no runtime probe (Map, Lottie), three at runtime.
fn destinations() -> Vec<Dest> {
    use crate::res::vectors;
    use Group::*;
    let d = |group: Group,
             section: Section,
             title: fn() -> day::LocalizedText,
             icon: day::VectorName,
             page: fn() -> AnyPiece| Dest {
        section,
        group,
        title,
        icon,
        page,
    };
    let mut all = vec![
        d(
            Overview,
            Section::About,
            crate::res::str::nav_about,
            vectors::nav_about,
            about_page,
        ),
        d(
            Controls,
            Section::Controls,
            crate::res::str::nav_controls,
            vectors::nav_controls,
            controls_page,
        ),
        d(
            Controls,
            Section::Text,
            crate::res::str::nav_text,
            vectors::nav_text,
            text_page,
        ),
        d(
            Controls,
            Section::TextAreas,
            crate::res::str::nav_textareas,
            vectors::nav_textareas,
            text_areas_page,
        ),
        d(
            Controls,
            Section::Dates,
            crate::res::str::nav_dates,
            vectors::nav_dates,
            dates_page,
        ),
        d(
            Controls,
            Section::Focus,
            crate::res::str::nav_focus,
            vectors::nav_focus,
            focus_page,
        ),
        d(
            Controls,
            Section::Cursors,
            crate::res::str::nav_cursors,
            vectors::nav_cursor,
            cursors_page,
        ),
        d(
            Layout,
            Section::Layout,
            crate::res::str::nav_layout,
            vectors::nav_layout,
            layout_page,
        ),
        d(
            Layout,
            Section::Grid,
            crate::res::str::nav_grid,
            vectors::nav_grid,
            grid_page,
        ),
        d(
            Navigation,
            Section::Stack,
            crate::res::str::nav_stack,
            vectors::nav_stack,
            stack_page,
        ),
        d(
            Navigation,
            Section::Tabs,
            crate::res::str::nav_tabs,
            vectors::nav_tabs,
            tabs_page,
        ),
        d(
            Navigation,
            Section::Menus,
            crate::res::str::nav_menus,
            vectors::nav_menus,
            menus_page,
        ),
        d(
            Navigation,
            Section::Toolbars,
            crate::res::str::nav_toolbars,
            vectors::nav_toolbars,
            toolbars_page,
        ),
        d(
            Navigation,
            Section::ContentList,
            crate::res::str::nav_content_list,
            vectors::nav_content_list,
            content_list_page,
        ),
        d(
            Data,
            Section::List,
            crate::res::str::nav_list,
            vectors::nav_list,
            list_page,
        ),
        d(
            Data,
            Section::DragDrop,
            crate::res::str::nav_drag_drop,
            vectors::nav_drag_drop,
            drag_drop_page,
        ),
        d(
            Data,
            Section::Tree,
            crate::res::str::nav_tree,
            vectors::nav_tree,
            tree_page,
        ),
        d(
            Data,
            Section::Model,
            crate::res::str::nav_model,
            vectors::nav_model,
            model_page,
        ),
        d(
            Data,
            Section::Query,
            crate::res::str::nav_query,
            vectors::nav_query,
            query_page,
        ),
        d(
            Graphics,
            Section::Canvas,
            crate::res::str::nav_canvas,
            vectors::nav_canvas,
            canvas_page,
        ),
        d(
            Graphics,
            Section::Animation,
            crate::res::str::nav_animation,
            vectors::nav_animation,
            animation_page,
        ),
        d(
            Graphics,
            Section::Resources,
            crate::res::str::nav_resources,
            vectors::nav_resources,
            resources_page,
        ),
        d(
            Graphics,
            Section::Media,
            crate::res::str::nav_media,
            vectors::nav_media,
            media_page,
        ),
        #[cfg(not(all(feature = "gtk", any(target_os = "macos", target_os = "windows"))))]
        d(
            Graphics,
            Section::Lottie,
            crate::res::str::nav_lottie,
            vectors::nav_lottie,
            lottie_page,
        ),
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        d(
            Graphics,
            Section::Map,
            crate::res::str::nav_map,
            vectors::nav_map,
            map_page,
        ),
        d(
            Graphics,
            Section::WebView,
            crate::res::str::nav_webview,
            vectors::nav_webview,
            webview_page,
        ),
        d(
            Platform,
            Section::System,
            crate::res::str::nav_system,
            vectors::nav_system,
            system_page,
        ),
        d(
            Platform,
            Section::Network,
            crate::res::str::nav_network_http,
            vectors::nav_network,
            network_page,
        ),
        d(
            Platform,
            Section::Notify,
            crate::res::str::nav_notify_badge,
            vectors::nav_notify,
            notify_page,
        ),
        d(
            Platform,
            Section::Speech,
            crate::res::str::nav_speech_haptics,
            vectors::nav_speech,
            speech_page,
        ),
        d(
            Platform,
            Section::Files,
            crate::res::str::nav_files_storage,
            vectors::nav_files,
            files_page,
        ),
        d(
            Platform,
            Section::Camera,
            crate::res::str::nav_camera,
            vectors::nav_camera,
            camera_page,
        ),
        d(
            App,
            Section::Localization,
            crate::res::str::nav_localization,
            vectors::nav_localization,
            localization_page,
        ),
        d(
            App,
            Section::Scripting,
            crate::res::str::nav_scripting,
            vectors::nav_scripting,
            scripting_page,
        ),
        d(
            App,
            Section::Tweaks,
            crate::res::str::nav_tweaks,
            vectors::nav_tweaks,
            tweaks_page,
        ),
        d(
            App,
            Section::Benchmark,
            crate::res::str::nav_benchmark,
            vectors::nav_benchmark,
            benchmark_page,
        ),
        d(
            App,
            Section::CrashReporting,
            crate::res::str::nav_crash,
            vectors::nav_crash,
            crash_page,
        ),
    ];
    // The runtime-decided omissions. Each answer is constant for a binary, so the tracked
    // derive that calls this pays nothing for asking every time.
    all.retain(|d| match d.section {
        Section::Toolbars => capability(Cap::Toolbar) != Support::Unsupported,
        Section::CrashReporting => crate::support::crash_reporting() != Support::Unsupported,
        // The piece's own probe: it renders on the mobile toolkits and nowhere else.
        Section::Camera => day_piece_camera::support() != Support::Unsupported,
        _ => true,
    });
    all
}

/// One showcase shell: the primary window's content, and (via `register_new_window`) each
/// File ▸ New Window's. Every call creates its own section signal, so windows navigate
/// independently; app-global state (menu log, lifecycle log, controls prefs) is shared.
/// Only the primary shell joins the route namespace; secondary windows are `.local()`
/// (docs/navigation.md), so `navigate()`/dayscript keep driving the primary unambiguously.
fn window_root(primary: bool) -> impl Piece {
    // One `Scene` per window (docs/state.md): the sidebar selection, the scripting buffer, the
    // toolbar demo's controls, the benchmark parameters and the webview's fields all belong to
    // this window. What one process has one of (the logs, the SQLite container, the model
    // store, the persisted preferences) stays app-wide behind `Ambient::app`.
    Scene::scoped(move |_scene| {
        // The Content List page's document, filter and selection: the scaffold's own `Scene`,
        // one per window like the showcase's, saved by the primary window only
        // (src/pages/content_list/model.rs).
        pages::content_list::Scene::scoped(move |items| {
            if primary {
                items.persist();
            }
            crate::pages::toolbars::ToolbarDemo::scoped(move |_bar| window_body(primary))
        })
    })
}

fn window_body(primary: bool) -> impl Piece {
    // The app menu is one bar for the app, but its titles and enabled states read the front
    // page, so it installs from inside a window's scope, once. Before any window exists there
    // is no page to describe.
    static MENU_ONCE: std::sync::Once = std::sync::Once::new();
    MENU_ONCE.call_once(install_app_menu);
    // Remember the last-opened section across launches (docs/navigation.md). Web only, matching
    // this app's prefs policy (controls.rs): a browser reload is normal life on the web, so the
    // store is installed there and the top-level nav's `.restore` persists the section;
    // native launches install no store, so `.restore` is a silent no-op and every run starts
    // fresh, which is what the walkthrough asserts.
    #[cfg(target_arch = "wasm32")]
    day::prefs::install_nav_store();
    // Deep-link: open directly on a section when `DAY_DEMO_ROUTE` is set (`day launch --env
    // DAY_DEMO_ROUTE=canvas`), else start at the root menu. Handy for driving the emulator when
    // synthetic input is unreliable.
    // The selection lives in commands.rs (hoisted there so the toolbar and the app menu can read
    // it reactively; see `commands::section`); it still seeds from DAY_DEMO_ROUTE.
    let section = crate::commands::section();
    // Each destination carries a bundled vector glyph (resource/vectors/nav_*.svg) shown in the
    // native nav where the backend supports it (e.g. the Windows NavigationView pane).
    // The sidebar filters live on what its own search field holds (docs/localization.md
    // "Searching"): a row survives when the query is a case-insensitive prefix of one of its
    // title's words, with the words found by the current locale's own segmentation.
    let query = pages::toolbars::search_query();
    // The Content List page's list is this host's own content-list pane (docs/navigation.md):
    // a third column on a desktop, the pushed middle layer on a phone, collapsed on every other
    // page. Its commands ride the pane, so they come and go with it (docs/toolbars.md).
    let items = pages::content_list::Scene::ambient();
    let nav = nav(section)
        .style(NavStyle::Sidebar)
        .title(crate::res::str::app_title())
        .content_list(pages::content_list::item_list_pane)
        .content_list_width(320.0)
        .content_list_for(|k: &Option<Section>| matches!(k, Some(Section::ContentList)))
        .detail_visible(items.detail_open)
        // The pushed detail's bar title on a phone: the open item's name on the Content List
        // page (live, as the scaffold titles its editor), the section's own title elsewhere.
        .detail_title(move || match section.get() {
            // The open item's name, live; the section's own title while nothing is open (the
            // scaffold's fallback names its section, which is not this app's).
            Some(Section::ContentList) => {
                match items.selected.get().and_then(|id| items.find(id)) {
                    Some(item) if !item.name.is_empty() => item.name,
                    _ => crate::res::str::nav_content_list().format(),
                }
            }
            Some(sec) => destinations()
                .into_iter()
                .find(|d| d.section == sec)
                .map(|d| (d.title)().format())
                .unwrap_or_default(),
            None => crate::res::str::app_title().format(),
        })
        // Search belongs to the surface it filters, not to the toolbar (docs/search.md). Day
        // resolves where to draw it: today the window's toolbar on every desktop, and, once the
        // size-class work lands, the navigation list itself on a window too narrow for a
        // sidebar, with nothing here changing.
        .searchable(query)
        .search_prompt(crate::res::str::toolbar_search_placeholder())
        // Completions, drawn by whatever the platform's search field already has for them (a
        // QCompleter popup, a <datalist>, an AutoSuggestBox; docs/search.md). Every section
        // title that starts with what has been typed, localized, so they follow a language
        // switch like the rows do.
        .search_suggestions(|q: &str| {
            let q = q.to_lowercase();
            if q.is_empty() {
                return Vec::new();
            }
            destinations()
                .into_iter()
                .map(|d| (d.title)().format())
                .filter(|t| t.to_lowercase().starts_with(&q))
                .take(8)
                .collect()
        })
        // Reopen on the last-viewed section (web only; see the install_nav_store note above).
        .restore("nav.section")
        .items(
            move || {
                // Tracked: reads the query and (through `matches_search`) the locale, so the
                // rows re-filter on a keystroke and re-title on a language switch.
                let q = query.get();
                let rows: Vec<Dest> = destinations()
                    .into_iter()
                    .filter(|d| matches_search(&(d.title)().format(), &q))
                    .collect();
                // Starred pages leave their groups for a Starred section at the top (the
                // way a feed reader keeps its smart feeds above the subscriptions) and keep
                // their table order there. The rest keep table order under their group's
                // header, which the first surviving row of each group carries, so a group
                // whose rows the search filtered out disappears with them. Reading
                // `is_starred` here is what subscribes this derive to the starred set.
                let (starred, rest): (Vec<Dest>, Vec<Dest>) = rows
                    .into_iter()
                    .partition(|d| crate::commands::is_starred(d.section));
                let mut out: Vec<Row> = Vec::with_capacity(starred.len() + rest.len());
                let mut header: Option<fn() -> day::LocalizedText> =
                    Some(crate::res::str::group_starred);
                for dest in starred {
                    out.push(Row {
                        dest,
                        header: header.take(),
                    });
                }
                let mut last: Option<Group> = None;
                for dest in rest {
                    let header = (last != Some(dest.group)).then(|| dest.group.title());
                    last = Some(dest.group);
                    out.push(Row { dest, header });
                }
                out
            },
            |r: &Row| {
                // Each row's context menu (docs/menus.md): "Show Source" opens this row's
                // page source on GitHub, the same handler surface as the toolbar button,
                // but per destination, so no navigation is needed first. The label re-lowers
                // localized on every derive (locale switches re-run this mapper).
                let d = &r.dest;
                let section = d.section;
                let starred = crate::commands::is_starred(section);
                let row = item(d.section, (d.title)())
                    .icon(d.icon.clone())
                    .icon_tint(d.group.tint())
                    // Star/Unstar per row, so any page can be starred without navigating to it
                    // first, through the same handler surface "Show Source" already uses here.
                    .context_menu(vec![
                        menu_item(
                            if starred {
                                crate::res::str::cmd_unstar()
                            } else {
                                crate::res::str::cmd_star()
                            }
                            .format(),
                        )
                        .action(move || crate::commands::toggle_star(section)),
                        menu_item(crate::res::str::show_source().format())
                            .action(move || open_source_of(section)),
                    ]);
                // The group header rides the group's first row (docs/navigation.md): the
                // nav opens a section there, and the flat-list toolkits ignore it.
                let row = match r.header {
                    Some(h) => row.section(h()),
                    None => row,
                };
                // The marker itself: the app's star, tinted the color a star is rather
                // than a theme accent (palette.rs `AMBER`).
                if starred {
                    row.badge_icon(res::vectors::star.clone())
                        .badge_tint(crate::palette::AMBER)
                } else {
                    row
                }
            },
        )
        // Dynamic rows carry no page builder of their own; the key is looked up here. Each
        // page carries the three commands that act on it (docs/toolbars.md), so they arrive and
        // leave with the page and every one of them knows its own section without asking the
        // route.
        .destination(|key: &Option<Section>| match key {
            Some(sec) => {
                let sec = *sec;
                destinations()
                    .into_iter()
                    .find(|d| d.section == sec)
                    .map(|d| {
                        (d.page)()
                            .toolbar(move || pages::toolbars::page_commands(sec))
                            .any()
                    })
                    .unwrap_or_else(|| column(()).any())
            }
            None => column(()).any(),
        });
    // The window's commands ride this host's chrome (docs/toolbars.md): the sidebar column
    // where the presentation has one, the root list's bar when it collapses. No capability
    // probe: a contribution always has a chrome to land on, so the fallback branch this used to
    // carry for HarmonyOS and the web is gone.
    let nav = nav.toolbar(pages::toolbars::window_items());
    let nav = if primary { nav } else { nav.local() };
    nav.id("nav")
}
