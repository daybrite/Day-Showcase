//! Window toolbars (docs/toolbars.md). The demonstration is the MAIN WINDOW'S OWN toolbar —
//! a toolbar is window chrome, so there is nowhere on a page to put one. This page installs
//! that bar, shows what each item is doing live, and drives the whole API from the content:
//! add and remove an item, enable and disable one, and read the two-way bindings.
//!
//! Where the toolkit has no toolbar (`Cap::Toolbar` is `Unsupported` — the phones, the web)
//! nothing installs, and the page says so rather than drawing an imitation.

use day::prelude::*;

use crate::widgets::page;

/// The signals the main window's toolbar is bound to.
#[derive(Clone, Copy)]
pub(crate) struct ToolbarDemo {
    query: Signal<String>,
    /// The Toolbars page's own star switch, mirroring the command's state so the control reads
    /// right whichever surface last toggled it.
    star_switch: Signal<bool>,
    /// How many times a plain toolbar button has been pressed.
    presses: Signal<i64>,
    /// Whether the optional item is in the bar — the add/remove demonstration.
    extra: Signal<bool>,
    /// Whether the disable-able item (Show Source) is enabled — the targeted-patch demo.
    source_enabled: Signal<bool>,
    /// The last thing the toolbar did, in words.
    last: Signal<String>,
    /// The appearance segmented control's chosen index. The truth is the persisted setting
    /// (commands.rs); this MIRRORS it, the way `starred` mirrors the starred set.
    theme: Signal<usize>,
}

/// The appearance chooser: ONE native segmented control, not three toggles (docs/toolbars.md).
///
/// Exactly one mode is in force, so this is what a segmented control is for — the platform draws
/// the three as one grouped control, announces them as one radio group, and keeps the exclusivity
/// itself. The sun/auto/moon glyphs are the standard symbols, so each desktop draws its own.
fn appearance_item(mode: Signal<usize>) -> ToolbarEntry {
    toolbar_segmented(
        "tb-theme",
        vec![
            segment(crate::res::str::cmd_appearance_light()).icon(Symbol::Light),
            segment(crate::res::str::cmd_appearance_system()).icon(Symbol::Auto),
            segment(crate::res::str::cmd_appearance_dark()).icon(Symbol::Dark),
        ],
        mode,
    )
    .enabled_when(crate::commands::appearance_supported)
    .action(move || {
        // The control has already written the chosen index into `mode`; turn it into the app's
        // setting. The mirroring effect in `install` writes the index back, which is a no-op when
        // it agrees — and the backends suppress their own programmatic echo, so it stays one hop.
        crate::commands::set_appearance(crate::commands::Appearance::from_index(
            mode.get_untracked(),
        ));
    })
}

/// The toolbar's search text, which also filters the sidebar (`crate::destinations`). Public to
/// the crate because the shell reads it while building the nav, before this page ever opens.
pub(crate) fn search_query() -> Signal<String> {
    state().query
}

impl Ambient for ToolbarDemo {
    fn create() -> Self {
        ToolbarDemo {
            query: Signal::new(String::new()),
            star_switch: Signal::new(false),
            presses: Signal::new(0),
            extra: Signal::new(false),
            source_enabled: Signal::new(true),
            last: Signal::new(String::new()),
            theme: Signal::new(crate::commands::Appearance::System.index()),
        }
    }
}

/// The toolbar demo's own controls — PER WINDOW (docs/state.md), like the toolbar they drive.
fn state() -> ToolbarDemo {
    ToolbarDemo::try_ambient()
        .or_else(ToolbarDemo::focused)
        .expect("no window is open")
}

/// Does this toolkit have a real toolbar?
fn available() -> bool {
    // Emulated counts: web-dom docks a real strip above the app root rather than hanging chrome
    // off a title bar it does not have (docs/toolbars.md). What matters to the app is whether
    // the commands belong in a bar at all, not who draws it.
    capability(Cap::Toolbar) != Support::Unsupported
}

/// This window's own toolbar items, for the sidebar host to carry (`crate::showcase_nav`).
///
/// Called once per window while its shell builds, so every window gets its own bar bound to its
/// own state.
pub(crate) fn window_items() -> impl Fn() -> Vec<ToolbarEntry> + 'static {
    let s = state();
    // Two-way, both hops guarded on disagreement: the effect follows the command's state from
    // whichever surface changed it, and the watch runs the command only when the switch was
    // moved to something the set does not already say.
    Effect::new(move || s.star_switch.set((crate::commands::star().checked)()));
    watch(
        move || s.star_switch.get(),
        move |on, _| {
            if *on != (crate::commands::star().checked)() {
                (crate::commands::star().run)();
            }
        },
    );
    // The appearance group: the SETTING is the truth (commands.rs), and the control follows it.
    // So a mode chosen from the App menu presses the right button here, and pressing the mode
    // already on writes the same value back rather than turning the group off.
    Effect::new(move || s.theme.set(crate::commands::appearance().index()));

    // The window's own commands: New Window, and the appearance picker. Both act on the app
    // rather than on any one page, so they are declared where the window is
    // (docs/toolbars.md) — the sidebar host's chrome, which is the sidebar column on a desktop
    // and the root list's bar when that collapses.
    //
    // Reactive, because the builder reads `extra`: ticking that switch adds or removes the item,
    // and the add/remove API is just a different list. It also re-lowers on a language change,
    // which is why the labels are `res::str` calls rather than captured Strings.
    //
    // The sidebar toggle is NOT here. A `nav(Sidebar)` draws the platform's own, so the app
    // declares nothing for it.
    move || {
        let mut items = vec![
            // A plain command: another window on the same app state (docs/windows.md), the same
            // thing File ▸ New Window does.
            toolbar_button("tb-new", crate::res::str::toolbar_new())
                .icon(Symbol::New)
                .enabled_when(|| capability(Cap::MultiWindow) != Support::Unsupported)
                .action(move || {
                    day::open_new_window();
                    note(s, crate::res::str::toolbar_last_new());
                }),
            toolbar_separator(),
            // ── Appearance: one segmented control over one setting (commands.rs) ─────────
            appearance_item(s.theme),
        ];
        if s.extra.get() {
            // The add/remove demonstration, and a real command: saving a picture of the window
            // straight to the app's scripts-adjacent container is overkill, so this one copies
            // the running toolkit's name — the thing a bug report always wants and nothing else
            // in the app puts on the clipboard.
            items.push(
                toolbar_button("tb-extra", crate::res::str::toolbar_extra())
                    .icon(Symbol::Copy)
                    .tooltip(crate::res::str::toolbar_extra_tooltip())
                    .action(move || {
                        let info = format!(
                            "Day-Showcase {} · {}",
                            env!("CARGO_PKG_VERSION"),
                            day::toolkit_name()
                        );
                        if day_part_clipboard::set_text(&info) {
                            note(s, crate::res::str::toolbar_last_extra());
                        }
                    }),
            );
        }
        // The search field is NOT declared here. It belongs to the sidebar it filters
        // (`crate::showcase_nav`'s `.searchable(query)`, docs/search.md), and day places it
        // itself — trailing, after everything above.
        items
    }
}

/// The three commands that act on THE PAGE THAT IS SHOWING, declared on the page itself so they
/// arrive and leave with it (docs/toolbars.md).
///
/// Each takes the section directly, which is what makes this the whole of their wiring: before
/// the toolbar knew which page it was over, all three resolved `current_route()` at press time
/// and the star had to mirror its state into a signal the bar could read.
pub(crate) fn page_commands(sec: crate::Section) -> Vec<ToolbarEntry> {
    let starred = Signal::new(crate::commands::is_starred(sec));
    // The button follows the starred SET, which the row context menu and the App menu also
    // write — one truth, three surfaces.
    Effect::new(move || starred.set(crate::commands::is_starred(sec)));
    // The one item the Toolbars page can disable, so the targeted-patch demo has a subject:
    // only this item changes, and a search in progress is undisturbed.
    let demo = state();
    vec![
        // "Show Source": open this page's source on GitHub. The standard symbol, which every
        // toolkit draws from its own set — SF Symbols on Apple, Day's Material glyphs on Android.
        toolbar_button("tb-source", crate::res::str::show_source())
            .icon(Symbol::Code)
            .tooltip(crate::res::str::show_source())
        .enabled_when(move || demo.source_enabled.get())
        .action(move || crate::open_source_of(sec)),
        // The Star command (commands.rs), not a demo toggle. Its label comes from the one
        // `Command`, so this button, the App menu's item and the row's context menu can never
        // disagree.
        toolbar_toggle("tb-star", (crate::commands::star().title)(), starred)
            .image(crate::res::vectors::star.clone())
            .action(move || {
                // Honour the state the toggle was moved TO, rather than flipping blindly.
                // `toolbar_toggle` writes the requested value into the bound signal before
                // running this, so that signal IS the intent — and a toggle asked to turn ON
                // while the page is already starred must be a no-op, not an unstar.
                if starred.get_untracked() != crate::commands::is_starred(sec) {
                    crate::commands::toggle_star(sec);
                }
            }),
    ]
}

/// Record what the toolbar just did, and count the presses.
fn note(s: ToolbarDemo, what: day::LocalizedText) {
    s.presses.set(s.presses.get_untracked() + 1);
    s.last.set(what.format());
}

pub(crate) fn toolbars_page() -> AnyPiece {
    page(
        crate::res::str::nav_toolbars(),
        "toolbars-title",
        form((readout_section(), controls_section(), vocabulary_section())).any(),
    )
    .any()
}

/// What the bar is doing right now — the two-way bindings, read from the page.
fn readout_section() -> impl Piece {
    let s = state();
    section((
        // Where there is no toolbar the rest of the page has nothing to report on.
        when(
            || !available(),
            || {
                label(crate::res::str::toolbar_unsupported())
                    .color(crate::palette::SLATE)
                    .id("toolbar-unsupported")
            },
        ),
        labeled(
            crate::res::str::toolbar_query_label(),
            label(move || {
                let q = s.query.get();
                if q.is_empty() {
                    crate::res::str::toolbar_query_empty().format()
                } else {
                    q
                }
            })
            .id("toolbar-query"),
        ),
        labeled(
            crate::res::str::toolbar_star_label(),
            // Read from the COMMAND, not from a mirror signal: the star button now belongs to
            // the page it acts on, so there is no window-level copy of its state to read.
            label(move || {
                if (crate::commands::star().checked)() {
                    crate::res::str::toolbar_on().format()
                } else {
                    crate::res::str::toolbar_off().format()
                }
            })
            .id("toolbar-star-state"),
        ),
        // The appearance group's setting, and whether this toolkit acts on it — the three buttons
        // lower disabled where it does not, and this says why.
        labeled(
            crate::res::str::toolbar_appearance_label(),
            label(move || {
                let mode = (crate::commands::appearance_command(crate::commands::appearance())
                    .title)()
                .format();
                if crate::commands::appearance_supported() {
                    mode
                } else {
                    crate::res::str::toolbar_appearance_ignored(mode).format()
                }
            })
            .id("toolbar-appearance"),
        ),
        // The recorder's transport, in one word: what the bar's Record and Play items are doing.
        labeled(
            crate::res::str::toolbar_transport_label(),
            label(|| {
                let s = if day::record::recording_signal().get() {
                    crate::res::str::toolbar_transport_recording()
                } else if day::record::playing_signal().get() {
                    if day::record::paused_signal().get() {
                        crate::res::str::toolbar_transport_paused()
                    } else {
                        crate::res::str::toolbar_transport_playing()
                    }
                } else {
                    crate::res::str::toolbar_transport_idle()
                };
                s.format()
            })
            .id("toolbar-transport"),
        ),
        labeled(
            crate::res::str::toolbar_presses_label(),
            label(move || crate::res::str::toolbar_presses(s.presses.get() as f64).format())
                .tabular()
                .id("toolbar-presses"),
        ),
        labeled(
            crate::res::str::toolbar_last_label(),
            label(move || {
                let l = s.last.get();
                if l.is_empty() {
                    crate::res::str::toolbar_last_none().format()
                } else {
                    l
                }
            })
            .id("toolbar-last"),
        ),
    ))
    .title(crate::res::str::toolbar_readout_title())
}

/// Driving the bar from the content: add/remove an item, disable one, write a bound signal.
fn controls_section() -> impl Piece {
    let s = state();
    section((
        labeled(
            crate::res::str::toolbar_extra_label(),
            toggle(s.extra).id("toolbar-extra-switch"),
        ),
        labeled(
            crate::res::str::toolbar_enabled_label(),
            toggle(s.source_enabled).id("toolbar-enabled-switch"),
        ),
        // Mirrors the command's own state, not a window-level copy: the star button belongs to
        // whichever page is showing now, and this switch drives the same command it does.
        labeled(
            crate::res::str::toolbar_star_label(),
            toggle(s.star_switch).id("toolbar-star-switch"),
        ),
        row((
            button(crate::res::str::toolbar_clear_search())
                .action(move || s.query.set(String::new()))
                .id("toolbar-clear-search"),
            button(crate::res::str::toolbar_seed_search())
                .action(move || s.query.set(crate::res::str::toolbar_seed_text().format()))
                .id("toolbar-seed-search"),
        ))
        .spacing(8.0),
    ))
    .title(crate::res::str::toolbar_controls_title())
}

/// What is in the bar, so the page names each kind the vocabulary offers.
fn vocabulary_section() -> impl Piece {
    section((
        labeled(
            crate::res::str::toolbar_kind_button(),
            label(crate::res::str::toolbar_kind_button_note()),
        ),
        labeled(
            crate::res::str::toolbar_kind_toggle(),
            label(crate::res::str::toolbar_kind_toggle_note()),
        ),
        labeled(
            crate::res::str::toolbar_kind_menu(),
            label(crate::res::str::toolbar_kind_menu_note()),
        ),
        labeled(
            crate::res::str::toolbar_kind_search(),
            label(crate::res::str::toolbar_kind_search_note()),
        ),
        labeled(
            crate::res::str::toolbar_kind_space(),
            label(crate::res::str::toolbar_kind_space_note()),
        ),
    ))
    .title(crate::res::str::toolbar_vocabulary_title())
}
