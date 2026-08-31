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
    starred: Signal<bool>,
    /// How many times a plain toolbar button has been pressed.
    presses: Signal<i64>,
    /// Whether the optional item is in the bar — the add/remove demonstration.
    extra: Signal<bool>,
    /// Whether the disable-able item (Clear recording) is enabled.
    refresh_enabled: Signal<bool>,
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
            starred: Signal::new(false),
            presses: Signal::new(0),
            extra: Signal::new(false),
            refresh_enabled: Signal::new(true),
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

/// Install the main window's toolbar. Called from `root` for the primary window and from the
/// New Window builder for each secondary, so every window gets its own bar.
pub(crate) fn install() {
    if !available() {
        return;
    }
    let s = state();
    // The toolbar's star shows the STARRED state of the page that is showing. `toolbar_toggle`
    // is bound to a signal, and the truth lives in the command's persisted set, so mirror it
    // here: the effect re-runs whenever the set (or the route) changes, from whichever surface
    // did it, and the button follows.
    Effect::new(move || s.starred.set((crate::commands::star().checked)()));
    // The appearance group, same shape: the SETTING is the truth (commands.rs), and these three
    // follow it. So a mode chosen from the App menu presses the right button here, and pressing
    // the mode already on writes the same value back rather than turning the group off.
    Effect::new(move || s.theme.set(crate::commands::appearance().index()));

    // Reactive: the builder reads `extra`, so ticking that switch adds or removes the item —
    // the add/remove API is just a different list. It also re-lowers on a language change,
    // which is why the labels are `res::str` calls rather than captured Strings.
    toolbar_reactive(move || {
        let mut items = vec![
            // Show/hide the sidebar. Declared FIRST, which is where every desktop platform
            // expects it — beside the split's divider on macOS, at the head of the header bar
            // on GNOME. It takes no `.action`: the toolkit binds it to this window's
            // `selector(Sidebar)` and drives that host's own collapse (docs/toolbars.md).
            toolbar_sidebar_toggle("tb-sidebar", crate::res::str::toolbar_sidebar()),
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
            toolbar_separator(),
            // ── The recorder's transport (commands.rs, docs/agent.md) ────────────────────
            //
            // Record ↔ Stop, then Play ↔ Pause. Both drive the Scripting page's buffer, so a
            // recording started here is the script that page shows, and Play replays it.
            {
                let rec = crate::commands::record();
                // Stop is a standard Symbol; there is no standard "record", so the dot is a
                // bundled vector (§18.4) — the one glyph in this bar the platform has no idea of.
                let mut item = toolbar_button(rec.id, (rec.title)());
                item = if day::record::recording_signal().get() {
                    item.icon(Symbol::Stop)
                } else {
                    item.image(crate::res::vectors::record_dot.clone())
                };
                item.tooltip((rec.title)())
                    .enabled_when(rec.enabled)
                    .action(move || {
                        // The title BEFORE running: pressing Record leaves the item reading
                        // "Stop", and the readout is supposed to name what was invoked.
                        let what = (rec.title)();
                        (rec.run)();
                        note(s, what);
                    })
            },
            {
                let play = crate::commands::play_pause();
                toolbar_button(play.id, (play.title)())
                    .icon(
                        if day::record::playing_signal().get()
                            && !day::record::paused_signal().get()
                        {
                            Symbol::Pause
                        } else {
                            Symbol::Play
                        },
                    )
                    .tooltip((play.title)())
                    .enabled_when(play.enabled)
                    .action(move || {
                        // The title BEFORE running: pressing Record leaves the item reading
                        // "Stop", and the readout is supposed to name what was invoked.
                        let what = (play.title)();
                        (play.run)();
                        note(s, what);
                    })
            },
            // The one item the page can disable, so the targeted-patch demo still has a subject:
            // only this item changes, and a search in progress is undisturbed.
            {
                let clear = crate::commands::clear_recording();
                toolbar_button(clear.id, (clear.title)())
                    .icon(Symbol::Delete)
                    .enabled_when(move || s.refresh_enabled.get() && (clear.enabled)())
                    .action(move || {
                        // The title BEFORE running: pressing Record leaves the item reading
                        // "Stop", and the readout is supposed to name what was invoked.
                        let what = (clear.title)();
                        (clear.run)();
                        note(s, what);
                    })
            },
            toolbar_separator(),
            // "Show Source" (docs/toolbars.md): open the current page's source on GitHub — the
            // desktop counterpart to the mobile nav-bar button (lib.rs `show_source`). It leads
            // the page-command group (source, star, screenshot): all three act on the page that
            // is showing, and it used to sit apart from them wearing an oversized bundled PNG.
            toolbar_button("tb-source", crate::res::str::show_source())
                .icon(Symbol::Code)
                .tooltip(crate::res::str::show_source())
                .action(crate::show_source),
            // The Star command (commands.rs), not a demo toggle: it stars the page that is
            // showing. The label, the pressed state and the enablement all come from the one
            // `Command`, so this button, the App menu's item and the row's context menu can
            // never disagree — and because they are read HERE, inside `toolbar_reactive`, a
            // star from any of them re-lowers this bar with no wiring between them.
            {
                let star = crate::commands::star();
                toolbar_toggle(star.id, (star.title)(), s.starred)
                    .image(crate::res::vectors::star.clone())
                    .enabled_when(star.enabled)
                    .action(move || {
                        // Honour the state the toggle was moved TO, rather than flipping blindly.
                        // `toolbar_toggle` writes the requested value into the bound signal before
                        // running this, so that signal IS the intent — and a toggle asked to turn
                        // ON while the page is already starred must be a no-op, not an unstar.
                        // Toggling regardless made the action depend on what happened to be
                        // persisted from the last run: the walkthrough starred this page, and the
                        // NEXT run's `on: true` silently unstarred it.
                        if s.starred.get_untracked() != (star.checked)() {
                            (star.run)();
                        }
                        note(s, crate::res::str::toolbar_last_star());
                    })
            },
            // The Screenshot command (commands.rs) — the second real command on this bar, and
            // the reason `Command` exists: declared once, rendered here and in the App menu.
            {
                let shot = crate::commands::screenshot();
                toolbar_button(shot.id, (shot.title)())
                    .icon(Symbol::Camera)
                    .enabled_when(shot.enabled)
                    .action(move || (shot.run)())
            },
            // A pull-down, built from the same entries the menu bar takes — the recorder's
            // less-used commands, which do not each deserve a button.
            toolbar_menu(
                "tb-menu",
                crate::res::str::toolbar_menu(),
                vec![
                    menu_item(crate::res::str::toolbar_menu_open_scripting().format()).action(
                        move || {
                            navigate_to(&crate::Section::Scripting);
                            note(s, crate::res::str::toolbar_menu_open_scripting());
                        },
                    ),
                    menu_item(crate::res::str::toolbar_menu_copy_script().format())
                        .enabled(crate::pages::scripting::has_script())
                        .action(move || {
                            let ok = crate::pages::scripting::buf_signal()
                                .with(|t| day_part_clipboard::set_text(t));
                            if ok {
                                note(s, crate::res::str::toolbar_menu_copy_script());
                            }
                        }),
                    menu_separator(),
                    menu_role(MenuRole::Copy),
                ],
            )
            .icon(Symbol::More),
        ];
        if s.extra.get() {
            // The add/remove demonstration, and a real command: save a picture of the window
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
        items.push(toolbar_flexible_space());
        // The search field is NOT declared here. It belongs to the sidebar it filters
        // (`crate::showcase_nav`'s `.searchable(query)`, docs/search.md), and day drops it into
        // this bar itself — trailing, after everything above. That is what will let it move into
        // the navigation list when the sidebar collapses on a narrow window, without this page
        // changing.
        items
    });
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
        Some(crate::res::str::toolbars_caption()),
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
            label(move || {
                if s.starred.get() {
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
            toggle(s.refresh_enabled).id("toolbar-enabled-switch"),
        ),
        labeled(
            crate::res::str::toolbar_star_label(),
            toggle(s.starred).id("toolbar-star-switch"),
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
