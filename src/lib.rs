//! The Day showcase (DESIGN.md Appendix A): every implemented piece behind a native navigation
//! host (docs/navigation.md) — stack presentation on mobile, sidebar + detail split on desktop.
//!
//! This crate root wires the navigation together in [`root`] and owns the app-wide lifecycle
//! plumbing; each navigation destination lives in its own module under [`pages`], and reusable
//! pieces shared by several pages live in [`widgets`].

use day::prelude::*;
use std::cell::OnceCell;

mod commands;
mod pages;
mod palette;
pub(crate) mod support;
mod widgets;

use crate::pages::*;

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

thread_local! {
    /// The most recent app-lifecycle phase, shown live on the About page (docs/lifecycle.md).
    static LIFECYCLE_LOG: OnceCell<Signal<String>> = const { OnceCell::new() };
}
pub(crate) fn lifecycle_log() -> Signal<String> {
    // `global`, NOT `new`: the first read can come from inside a page scope (on desktop-split
    // web the About page is the default detail), and a scope-owned signal would die with that
    // page — the second About visit would read a disposed signal.
    LIFECYCLE_LOG.with(|c| *c.get_or_init(|| Signal::global("—".into())))
}

/// Register app-lifecycle handlers (docs/lifecycle.md). Call this from `main` BEFORE `day::launch`
/// so the launch phases are captured. Each handler logs to the console and to a live UI readout.
///
/// The mobile-only phases are registered only where the compiled-in backend actually delivers them,
/// using the compile-time-accurate guard `day::lifecycle::supported(..)` — on desktop those `if`s are
/// `false` and the handlers are never registered, so no "unsupported phase" warning is produced.
pub fn install_lifecycle_handlers() {
    use day::Lifecycle::*;

    // Idempotent: desktop calls this from `main` (to catch WillLaunch), mobile from `root`.
    thread_local! { static INSTALLED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
    if INSTALLED.with(|c| c.replace(true)) {
        return;
    }

    let note = |phase: day::Lifecycle| {
        move || {
            // stdout, not stderr: this is a trace of normal operation, and day-android routes
            // fd 1 to logcat at INFO and fd 2 at ERROR — on stderr every phase would surface as
            // `E Day` and drown the log level out as a filter.
            println!("day lifecycle: {}", phase.name());
            lifecycle_log().set(phase.name().into());
        }
    };

    // Universal phases — every backend delivers these.
    for phase in [
        WillLaunch,
        DidLaunch,
        DidBecomeActive,
        WillResignActive,
        WillTerminate,
    ] {
        day::on_lifecycle(phase, note(phase));
    }
    // Mobile-only phases — guard so we register only where they're delivered (iOS / Android).
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
        Model => "model",
        Refresh => "refresh",
        Tabs => "tabs",
        Stack => "stack",
        Media => "media",
        WebView => "webview",
        Menus => "menus",
        System => "system",
        Services => "services",
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
/// The git ref "Show Source" links against — `vX.Y.Z` for a tagged release, else `main` for a
/// development build. Baked by `build.rs` from the release pipeline's `GITHUB_REF` (see there).
const SOURCE_REF: &str = env!("DAY_SHOWCASE_SOURCE_REF");

impl Section {
    /// The repo-relative source file whose page this section shows — the "Show Source" target.
    /// Kept exhaustive by the compiler, so a new section must name its file here.
    fn source_file(self) -> &'static str {
        match self {
            Section::About => "src/pages/about.rs",
            Section::Animation => "src/pages/animation.rs",
            Section::Benchmark => "src/pages/benchmark.rs",
            Section::Canvas => "src/pages/canvas.rs",
            Section::Controls => "src/pages/controls.rs",
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
            Section::Menus => "src/pages/menus.rs",
            Section::Refresh => "src/pages/refresh.rs",
            Section::Resources => "src/pages/resources.rs",
            Section::Scripting => "src/pages/scripting.rs",
            Section::Services => "src/pages/services.rs",
            Section::Stack => "src/pages/stack.rs",
            Section::System => "src/pages/system.rs",
            Section::Tabs => "src/pages/tabs.rs",
            Section::Text => "src/pages/text.rs",
            Section::TextAreas => "src/pages/text_areas.rs",
            Section::Toolbars => "src/pages/toolbars.rs",
            Section::Tweaks => "src/pages/tweaks.rs",
            Section::WebView => "src/pages/webview.rs",
        }
    }
}

/// Open the source of the page currently showing on GitHub, pinned to this build's ref (a release
/// tag, else `main`). Both the desktop toolbar button and the mobile nav-bar button call this —
/// it reads the live route, so one handler serves every page (docs/navigation.md). With nothing
/// selected (the desktop split's default) it falls back to About, which is that default detail.
pub(crate) fn show_source() {
    let section = current_route()
        .as_deref()
        .and_then(|r| r.split(['/', '?']).next())
        .filter(|s| !s.is_empty())
        .and_then(Section::from_key)
        .unwrap_or(Section::About);
    open_source_of(section);
}

/// Open one section's source on GitHub — the sidebar rows' context-menu "Show Source"
/// (docs/menus.md) names its own section, so a right-click / long-press on ANY row works
/// without navigating there first; the toolbar/nav-bar button resolves the live route
/// through [`show_source`] instead.
pub(crate) fn open_source_of(section: Section) {
    open_url(&format!(
        "{SOURCE_REPO}/blob/{SOURCE_REF}/{}",
        section.source_file()
    ));
}

/// Arm crash reporting (docs/break.md) — the Crash Reporting page demonstrates it. Idempotent
/// (day-break's `init` is single-shot); safe to call from every entry point.
pub fn install_crash_reporting() {
    let _ = day_break::Config::new()
        // "Send report" opens a prefilled email to the developer (no server needed).
        .reporter(day_break::EmailReporter::new("crashdemo@daybrite.dev"))
        .init();
}

pub fn root() -> AnyPiece {
    // Arm crash capture before the UI mounts so the Crash Reporting page's crashes are recorded.
    install_crash_reporting();
    // Narrate every action to the console for the app's whole life (§14.6) — the same lines the
    // Scripting page's recorder echoes, in the same dayscript vocabulary:
    //
    //     dayscript ▸ navigate → dates  "Date & time"
    //     dayscript ▸ tap list-shuffle  "Shuffle"
    //
    // Nothing is retained, so it costs the same as the recorder's observer and never grows. The
    // showcase leaves it on because it is a demonstration: the log is a live reading of what a
    // recording WOULD capture, which makes the Scripting page's output legible before you record
    // anything. Set DAY_LOG_ACTIONS=0 to silence it — the env is read first, so a launch can
    // override the app.
    if std::env::var("DAY_LOG_ACTIONS").as_deref() != Ok("0") {
        // The Scripting page's own controls stay out, exactly as they stay out of a recording.
        day::record::exclude_prefix("scripting-");
        day::record::log_actions(true);
    }
    // Every locale under `resource/locales/` (en, fr, ar, zh-CN), embedded and registered by the
    // generated catalog (§18.5) — adding a language is a new directory, nothing to edit here.
    res::locales::install();
    // Persisted theme/language overrides (docs/windows.md; the launch env wins — CI variant
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
    day::register_new_window(|| {
        // Each window gets its own toolbar; the install targets the window being built.
        pages::toolbars::install();
        window_root(false)
    });
    install_app_menu();
    // The main window's own toolbar (docs/toolbars.md) — the Toolbars page drives it.
    pages::toolbars::install();
    // Lifecycle handlers (docs/lifecycle.md). On mobile this is the registration point; on desktop
    // `main` already registered them before launch (to also catch WillLaunch) — the call is idempotent.
    install_lifecycle_handlers();
    window_root(true)
}

/// One sidebar destination: the row's title and icon, and the page it opens.
///
/// `Clone` because `.items(…)` re-derives the list on every query keystroke; the fields are a
/// key, two fn pointers and a name, so a clone is cheap.
///
/// A TABLE, not a chain of `.item_icon(…)` calls, because the sidebar is filterable — its rows
/// are derived from the search query, and `.items(…)` wants a list it can re-derive. The table
/// is also what `.destination` looks a key up in, so a row and its page can never drift apart.
#[derive(Clone)]
struct Dest {
    section: Section,
    /// The generated `res::str` accessor, not a resolved `String`: the title has to be
    /// re-resolved on every derive so the rows re-title (and re-filter) on a locale switch.
    title: fn() -> day::LocalizedText,
    /// A `resource/vectors/` glyph (docs/vectors.md): resolution-independent, staged per backend
    /// as whatever its nav rows load natively (VectorDrawable / catalog entry / raster cache).
    icon: day::VectorName,
    /// This destination's icon tint — a vivid categorical cycle, anchored on the identity
    /// palette (palette.rs) where a brand color fits: high-chroma mid-value hues, with
    /// neighbouring rows never in the same hue family. The chroma matters — pastel or
    /// near-neutral tints read as washed out on a glyph this small (docs/vectors.md).
    tint: Color,
    page: fn() -> AnyPiece,
}

/// Every destination, in the order the sidebar shows them — ALPHABETICAL by the US-English
/// display title. Keep it that way when adding a page. About is both alphabetically first and
/// the desktop split's default detail (the split selects the first row when nothing is chosen).
fn destinations() -> Vec<Dest> {
    vec![
        Dest {
            section: Section::About,
            title: crate::res::str::nav_about,
            icon: res::vectors::nav_about,
            tint: crate::palette::SKY,
            page: about_page,
        },
        Dest {
            section: Section::Animation,
            title: crate::res::str::nav_animation,
            icon: res::vectors::nav_animation,
            tint: Color::hex(0x06B6D4),
            page: animation_page,
        },
        Dest {
            section: Section::Benchmark,
            title: crate::res::str::nav_benchmark,
            icon: res::vectors::nav_benchmark,
            tint: Color::hex(0xF97316),
            page: benchmark_page,
        },
        Dest {
            section: Section::Canvas,
            title: crate::res::str::nav_canvas,
            icon: res::vectors::nav_canvas,
            tint: crate::palette::AMBER,
            page: canvas_page,
        },
        Dest {
            section: Section::Controls,
            title: crate::res::str::nav_controls,
            icon: res::vectors::nav_controls,
            tint: Color::hex(0x16A34A),
            page: controls_page,
        },
        Dest {
            section: Section::CrashReporting,
            title: crate::res::str::nav_crash,
            icon: res::vectors::nav_crash,
            tint: Color::hex(0x84CC16),
            page: crash_page,
        },
        Dest {
            section: Section::Dates,
            title: crate::res::str::nav_dates,
            icon: res::vectors::nav_dates,
            tint: Color::hex(0xEAB308),
            page: dates_page,
        },
        Dest {
            section: Section::System,
            title: crate::res::str::nav_system,
            icon: res::vectors::nav_system,
            tint: Color::hex(0x6366F1),
            page: system_page,
        },
        Dest {
            section: Section::Focus,
            title: crate::res::str::nav_focus,
            icon: res::vectors::nav_focus,
            tint: Color::hex(0x14B8A6),
            page: focus_page,
        },
        Dest {
            section: Section::Grid,
            title: crate::res::str::nav_grid,
            icon: res::vectors::nav_grid,
            tint: Color::hex(0xA855F7),
            page: grid_page,
        },
        Dest {
            section: Section::Layout,
            title: crate::res::str::nav_layout,
            icon: res::vectors::nav_layout,
            tint: Color::hex(0x2563EB),
            page: layout_page,
        },
        Dest {
            section: Section::List,
            title: crate::res::str::nav_list,
            icon: res::vectors::nav_list,
            tint: Color::hex(0xEF4444),
            page: list_page,
        },
        Dest {
            section: Section::Model,
            title: crate::res::str::nav_model,
            icon: res::vectors::nav_model,
            tint: Color::hex(0x7C3AED),
            page: model_page,
        },
        Dest {
            section: Section::Localization,
            title: crate::res::str::nav_localization,
            icon: res::vectors::nav_localization,
            tint: Color::hex(0xEC4899),
            page: localization_page,
        },
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        Dest {
            section: Section::Map,
            title: crate::res::str::nav_map,
            icon: res::vectors::nav_map,
            tint: Color::hex(0x0EA5E9),
            page: map_page,
        },
        Dest {
            section: Section::Media,
            title: crate::res::str::nav_media,
            icon: res::vectors::nav_media,
            tint: Color::hex(0xD946EF),
            page: media_page,
        },
        Dest {
            section: Section::Menus,
            title: crate::res::str::nav_menus,
            icon: res::vectors::nav_menus,
            tint: Color::hex(0xF43F5E),
            page: menus_page,
        },
        Dest {
            section: Section::Services,
            title: crate::res::str::nav_services,
            icon: res::vectors::nav_services,
            tint: Color::hex(0x10B981),
            page: services_page,
        },
        Dest {
            section: Section::Refresh,
            title: crate::res::str::nav_refresh,
            icon: res::vectors::nav_refresh,
            tint: crate::palette::VIOLET,
            page: refresh_page,
        },
        Dest {
            section: Section::Resources,
            title: crate::res::str::nav_resources,
            icon: res::vectors::nav_resources,
            tint: crate::palette::CORAL,
            page: resources_page,
        },
        Dest {
            section: Section::Scripting,
            title: crate::res::str::nav_scripting,
            icon: res::vectors::nav_scripting,
            tint: Color::hex(0x0D9488),
            page: scripting_page,
        },
        Dest {
            section: Section::Stack,
            title: crate::res::str::nav_stack,
            icon: res::vectors::nav_stack,
            tint: crate::palette::RUST,
            page: stack_page,
        },
        Dest {
            section: Section::Tabs,
            title: crate::res::str::nav_tabs,
            icon: res::vectors::nav_tabs,
            tint: crate::palette::SKY,
            page: tabs_page,
        },
        Dest {
            section: Section::Text,
            title: crate::res::str::nav_text,
            icon: res::vectors::nav_text,
            tint: Color::hex(0x06B6D4),
            page: text_page,
        },
        Dest {
            section: Section::TextAreas,
            title: crate::res::str::nav_textareas,
            icon: res::vectors::nav_textareas,
            tint: Color::hex(0xF97316),
            page: text_areas_page,
        },
        Dest {
            section: Section::Toolbars,
            title: crate::res::str::nav_toolbars,
            icon: res::vectors::nav_toolbars,
            tint: crate::palette::AMBER,
            page: toolbars_page,
        },
        Dest {
            section: Section::Tweaks,
            title: crate::res::str::nav_tweaks,
            icon: res::vectors::nav_tweaks,
            tint: Color::hex(0x16A34A),
            page: tweaks_page,
        },
        Dest {
            section: Section::WebView,
            title: crate::res::str::nav_webview,
            icon: res::vectors::nav_webview,
            tint: Color::hex(0x84CC16),
            page: webview_page,
        },
    ]
}

/// One showcase shell — the primary window's content, and (via `register_new_window`) each
/// File ▸ New Window's. Every call creates its own section signal, so windows navigate
/// independently; app-global state (menu log, lifecycle log, controls prefs) is shared.
/// Only the PRIMARY shell joins the route namespace — secondary windows are `.local()`
/// (docs/navigation.md), so `navigate()`/dayscript keep driving the primary unambiguously.
fn window_root(primary: bool) -> AnyPiece {
    // Remember the last-opened section across launches (docs/navigation.md). Web only, matching
    // this app's prefs policy (controls.rs): a browser reload is normal life on the web, so the
    // store is installed there and the top-level selector's `.restore` persists the section;
    // native launches install no store, so `.restore` is a silent no-op and every run starts
    // fresh — which is what the walkthrough asserts.
    #[cfg(target_arch = "wasm32")]
    day::prefs::install_nav_store();
    // Deep-link: open directly on a section when `DAY_DEMO_ROUTE` is set (`day launch --env
    // DAY_DEMO_ROUTE=canvas`), else start at the root menu. Handy for driving the emulator when
    // synthetic input is unreliable.
    // The selection lives in commands.rs (hoisted there so the toolbar and the app menu can read
    // it reactively — see `commands::section`); it still seeds from DAY_DEMO_ROUTE.
    let section = crate::commands::section();
    // Each destination carries a bundled vector glyph (resource/vectors/nav_*.svg) shown in the
    // native nav where the backend supports it (e.g. the Windows NavigationView pane).
    // The sidebar filters live on what its own search field holds (docs/localization.md
    // "Searching"): a row survives when the query is a case-insensitive prefix of one of its
    // title's words, with the words found by the current locale's own segmentation.
    let query = pages::toolbars::search_query();
    let nav = selector(section)
        .style(SelectorStyle::Sidebar)
        .title(crate::res::str::app_title())
        // Search belongs to the surface it filters, not to the toolbar (docs/search.md). Day
        // resolves where to draw it: today the window's toolbar on every desktop, and — once the
        // size-class work lands — the navigation list itself on a window too narrow for a
        // sidebar, with nothing here changing.
        .searchable(query)
        .search_prompt(crate::res::str::toolbar_search_placeholder())
        // Completions, drawn by whatever the platform's search field already has for them (a
        // QCompleter popup, a <datalist>, an AutoSuggestBox — docs/search.md). Every section
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
        // Reopen on the last-viewed section (web only — see the install_nav_store note above).
        .restore("nav.section")
        .items(
            move || {
                // TRACKED: reads the query AND (through `matches_search`) the locale, so the
                // rows re-filter on a keystroke and re-title on a language switch.
                let q = query.get();
                let mut rows = destinations()
                    .into_iter()
                    .filter(|d| matches_search(&(d.title)().format(), &q))
                    .collect::<Vec<_>>();
                // Starred pages rise to the top, keeping their relative (alphabetical) order —
                // a STABLE partition, so unstarring a page drops it back exactly where it was
                // rather than shuffling the list. Reading `is_starred` here is what subscribes
                // this derive to the starred set: a star from any surface re-orders the rows.
                rows.sort_by_key(|d| !crate::commands::is_starred(d.section));
                rows
            },
            |d: &Dest| {
                // Each row's context menu (docs/menus.md): "Show Source" opens THIS row's
                // page source on GitHub — the same handler surface as the toolbar button,
                // but per destination, so no navigation is needed first. The label re-lowers
                // localized on every derive (locale switches re-run this mapper).
                let section = d.section;
                let starred = crate::commands::is_starred(section);
                let row = item(d.section, (d.title)())
                    .icon(d.icon.clone())
                    .icon_tint(d.tint)
                    // Star/Unstar per ROW, so any page can be starred without navigating to it
                    // first — the same handler surface "Show Source" already uses here.
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
                // The marker itself: the app's own star, tinted the colour a star IS rather
                // than a theme accent (palette.rs AMBER).
                if starred {
                    row.badge_icon(res::vectors::star.clone())
                        .badge_tint(crate::palette::AMBER)
                } else {
                    row
                }
            },
        )
        // "Show Source" as an upper-right nav-bar button on the toolkits with no window toolbar
        // (the phones, HarmonyOS — docs/navigation.md); desktop shows it in the toolbar instead
        // (pages/toolbars.rs). One handler for every page: it reads the live route.
        .bar_action(
            res::images::show_source,
            crate::res::str::show_source(),
            show_source,
        )
        // Dynamic rows carry no page builder of their own — the key is looked up here.
        .destination(|key: &Option<Section>| match key {
            Some(sec) => destinations()
                .into_iter()
                .find(|d| d.section == *sec)
                .map(|d| (d.page)())
                .unwrap_or_else(|| column(()).any()),
            None => column(()).any(),
        });
    let nav = if primary { nav } else { nav.local() };
    nav.id("nav")
}

// The mobile / embedded entry point (DESIGN.md §17.4): the iOS and macOS Runners bind `day_main`,
// DayBridge binds the `Java_…` natives, the HarmonyOS ArkTS host binds `day_arkui_start`, and the
// web host page binds `day_dom_main`. One macro expands to all of them, and to nothing at all on a
// plain cargo desktop build, where src/main.rs is the entry instead.
day::day_main!("Day Showcase", root);
