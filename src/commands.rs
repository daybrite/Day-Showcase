//! The window's shared commands — one declaration each, consumed by every surface.
//!
//! A command in this app is reachable from four places at once: the window toolbar, the
//! application menu, a navigation row's context menu, and (for state-bearing ones) the row's own
//! decoration. Writing it four times is how those drift — the toolbar keeps saying "Star" after
//! the menu learned to say "Unstar", or one of them stops being disabled when the others are.
//!
//! So a command is declared ONCE here as a [`Command`]: its id, the title for its current state,
//! whether it is available, whether it is on, and what it does. Every surface renders the same
//! struct. Because the title/enabled/checked members are read inside the surfaces' own reactive
//! builders (`toolbar_reactive`, `app_menu_reactive`, the selector's `.items` mapper), touching
//! the state behind them re-lowers all four with no coordination code between them: the signal
//! IS the coordination.
//!
//! Star is the first of these. The rest of the Showcase's menu items are still decorative, and
//! this is the shape they move into as they become real. There is deliberately no `all()` list
//! yet: with one command it would be a speculative API with no caller, and the surfaces differ in
//! what they need around a command (a toolbar toggle wants a bound signal, a menu item wants a
//! key), so what the list should carry is better decided against a second real command than
//! guessed at now.

use day::prelude::*;

use crate::Section;

/// One command, in the form every surface can render.
///
/// The three state members are plain `fn()` rather than captured closures so a `Command` stays
/// `Copy` and can be handed to a `'static` toolbar/menu builder without cloning ceremony. They
/// are called INSIDE those builders, which is what subscribes each surface to the state.
#[derive(Clone, Copy)]
pub(crate) struct Command {
    /// Stable id — the toolbar item id and the menu action key. Also what a dayscript step names.
    pub id: &'static str,
    /// The label for the CURRENT state ("Star" vs "Unstar"), localized on every read.
    pub title: fn() -> day::LocalizedText,
    /// Whether the command applies right now.
    pub enabled: fn() -> bool,
    /// Whether it reads as "on" — a toolbar toggle's pressed state, a menu item's check mark.
    pub checked: fn() -> bool,
    /// Perform it.
    pub run: fn(),
}

// ── Starred pages ───────────────────────────────────────────────────────────────────────────
//
// Persisted as one comma-separated string of route keys rather than a set: `prefs::bind` stores
// `FromStr + ToString`, and the route key is already the stable identity every other subsystem
// (deep links, dayscript, `current_route`) addresses a page by. A page that is renamed or dropped
// simply stops matching, which is the right way for stale state to expire.

thread_local! {
    static STARRED: std::cell::OnceCell<Signal<String>> = const { std::cell::OnceCell::new() };
    static SECTION: std::cell::OnceCell<Signal<Option<Section>>> = const { std::cell::OnceCell::new() };
}

/// The sidebar's selection — the app's own routing signal, hoisted so every surface can READ it
/// reactively.
///
/// `current_route()` cannot serve here: a route surface reports its segments `get_untracked`, on
/// purpose (reading the route inside a builder must not subscribe that builder to navigation), so
/// a command deriving its state from `current_route` never re-ran when the selection moved — the
/// toolbar's star kept the previous page's on/off state until something else touched the starred
/// set. Reading THIS signal is a tracked read, so navigating re-lowers the toolbar and the menu.
pub(crate) fn section() -> Signal<Option<Section>> {
    SECTION.with(|c| {
        *c.get_or_init(|| {
            Signal::global(
                std::env::var("DAY_DEMO_ROUTE")
                    .ok()
                    .and_then(|r| Section::from_key(r.split(['/', '?']).next().unwrap_or(""))),
            )
        })
    })
}

/// The starred set, created once and reused by every visit and every surface.
///
/// `Signal::global` allocates in the root scope so it outlives the page subtrees that read it,
/// and the `OnceCell` keeps the SAME signal across rebuilds — calling `global` per build would
/// mint a fresh one each time and the stars would vanish on the next navigation.
pub(crate) fn starred() -> Signal<String> {
    STARRED.with(|c| {
        *c.get_or_init(|| {
            let s = Signal::global(String::new());
            // Survives relaunch (docs/prefs.md). Registered once, with the signal, so every
            // toggle from any surface is written through without the surfaces knowing.
            day::prefs::bind("showcase.starred", s);
            s
        })
    })
}

/// Whether `section` is starred. A tracked read, so any surface calling it re-renders on change.
pub(crate) fn is_starred(section: Section) -> bool {
    let key = section.key();
    starred().with(|s| s.split(',').any(|k| k == key))
}

/// Star or unstar `section`. Every surface goes through here, so there is one definition of what
/// toggling means and one place the persisted string is shaped.
pub(crate) fn toggle_star(section: Section) {
    let key = section.key();
    starred().update(|s| {
        let mut keys: Vec<String> = s
            .split(',')
            .filter(|k| !k.is_empty())
            .map(str::to_string)
            .collect();
        match keys.iter().position(|k| *k == key) {
            Some(i) => {
                keys.remove(i);
            }
            None => keys.push(key.clone()),
        }
        *s = keys.join(",");
    });
}

/// The route the star commands act on: whichever page is showing.
///
/// Falls back to About when the route is empty, exactly as `show_source` does — the desktop
/// split selects the first row as its default detail WITHOUT setting a route, so a command that
/// insisted on a route would sit disabled on the page the user is actually looking at.
fn active_section() -> Option<Section> {
    // A TRACKED read of the selection signal (see `section`). Falls back to About when nothing is
    // selected, exactly as `show_source` does — the desktop split shows the first row as its
    // default detail without selecting it, and a command that sat disabled on the page the user is
    // looking at would be wrong.
    Some(section().get().unwrap_or(Section::About))
}

/// The Star command for the active page — "Star" when it is not starred, "Unstar" when it is.
///
/// Disabled when there is no active page to star, which on mobile is the root list itself.
pub(crate) fn star() -> Command {
    Command {
        id: "cmd-star",
        title: || match active_section() {
            Some(s) if is_starred(s) => crate::res::str::cmd_unstar(),
            _ => crate::res::str::cmd_star(),
        },
        enabled: || active_section().is_some(),
        checked: || active_section().is_some_and(is_starred),
        run: || {
            if let Some(s) = active_section() {
                toggle_star(s);
            }
        },
    }
}
