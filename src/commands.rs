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

/// The persisted starred set — APP-wide (docs/state.md): a preference, the same in every window.
#[derive(Clone, Copy)]
struct Starred(Signal<String>);

impl Ambient for Starred {
    fn create() -> Self {
        let s = Signal::new(String::new());
        // Survives relaunch (docs/prefs.md). Registered once, with the signal, so every toggle
        // from any surface is written through without the surfaces knowing.
        day::prefs::bind("showcase.starred", s);
        Starred(s)
    }
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
    crate::scene().section
}

/// The starred set, created once and reused by every visit and every surface.
///
/// `Signal::global` allocates in the root scope so it outlives the page subtrees that read it,
/// and the `OnceCell` keeps the SAME signal across rebuilds — calling `global` per build would
/// mint a fresh one each time and the stars would vanish on the next navigation.
pub(crate) fn starred() -> Signal<String> {
    Starred::app().0
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
    //
    // `try_scene`, not `scene`: the menu bar is lowered from surfaces that can run before any
    // window has built (the Android app-bar menu is applied from a posted task), and a command
    // with no front window has nothing to act on rather than a reason to panic.
    Some(crate::try_scene()?.section.get().unwrap_or(Section::About))
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

/// Save a picture of this window (docs/window-image.md).
///
/// The capture is deferred a turn rather than taken inline: this command is reached from a MENU
/// and from a toolbar button, and on some backends the menu is still on screen (or the button
/// still drawn pressed) at the moment the action runs — the picture would show the affordance
/// that took it. One hop lets the chrome settle first.
///
/// Disabled where the toolkit cannot rasterize itself, so the affordance is absent rather than
/// present-and-failing (`Cap::Snapshot`; today that is web-dom).
pub(crate) fn screenshot() -> Command {
    Command {
        id: "cmd-screenshot",
        title: || crate::res::str::cmd_screenshot(),
        enabled: || day::window_image_support() == Support::Native,
        checked: || false,
        run: || {
            day::task(async move {
                // Let the menu dismiss (and the toolbar button un-press) before the shutter.
                day::sleep(150).await;
                let png = match day::window_image().capture() {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        warn!("window image capture failed: {e}");
                        return;
                    }
                };
                let _ = save_file(png)
                    .title(crate::res::str::cmd_screenshot())
                    .suggested_name(default_shot_name())
                    .filter("PNG", &["png"])
                    .await;
            });
        },
    }
}

// ── Appearance ──────────────────────────────────────────────────────────────────────────────
//
// Three commands over one persisted setting, so the toolbar's button group, the View ▸ Appearance
// menu and the Preferences window can never disagree about which mode is on.

/// Which appearance the app is asking for. `System` is the absence of an override.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Appearance {
    Light,
    System,
    Dark,
}

impl Appearance {
    /// The persisted form. A stable key, not a display string — it outlives translations.
    fn key(self) -> &'static str {
        match self {
            Appearance::Light => "light",
            Appearance::System => "system",
            Appearance::Dark => "dark",
        }
    }
    /// Its position in the toolbar's segmented control — light, system, dark, left to right.
    pub(crate) fn index(self) -> usize {
        match self {
            Appearance::Light => 0,
            Appearance::System => 1,
            Appearance::Dark => 2,
        }
    }
    /// The mode a segment index chose. Out of range is `System`, the neutral one.
    pub(crate) fn from_index(i: usize) -> Appearance {
        match i {
            0 => Appearance::Light,
            2 => Appearance::Dark,
            _ => Appearance::System,
        }
    }
    fn from_key(s: &str) -> Appearance {
        match s {
            "light" => Appearance::Light,
            "dark" => Appearance::Dark,
            _ => Appearance::System,
        }
    }
    /// What `day::set_appearance` takes: an override, or `None` to follow the system.
    fn override_dark(self) -> Option<bool> {
        match self {
            Appearance::Light => Some(false),
            Appearance::Dark => Some(true),
            Appearance::System => None,
        }
    }
    fn title(self) -> day::LocalizedText {
        match self {
            Appearance::Light => crate::res::str::cmd_appearance_light(),
            Appearance::System => crate::res::str::cmd_appearance_system(),
            Appearance::Dark => crate::res::str::cmd_appearance_dark(),
        }
    }
    fn id(self) -> &'static str {
        match self {
            Appearance::Light => "cmd-appearance-light",
            Appearance::System => "cmd-appearance-system",
            Appearance::Dark => "cmd-appearance-dark",
        }
    }
}

/// The persisted appearance setting — APP-wide (docs/state.md): it drives
/// `day::set_appearance`, a process-level override, so it is the app's choice not a window's.
#[derive(Clone, Copy)]
struct AppearanceSetting(Signal<String>);

impl Ambient for AppearanceSetting {
    fn create() -> Self {
        let s = Signal::new(Appearance::System.key().to_string());
        day::prefs::bind("showcase.appearance", s);
        // The boot run (the Effect fires once at creation) applies only a RESTORED
        // user choice. With no stored pref there is nothing to apply — the default is
        // System, and applying `None` anyway would CLEAR whatever the launch already
        // established: a forced `DAY_THEME` (day-appkit applies it as the NSApp
        // override at startup; the env wins over persistence, day-piece-settings'
        // `apply_startup` rule) or the Preferences window's own `showcase.theme`
        // setting, which `apply_startup` applied just before this menu builds.
        // `prefs::bind` restores synchronously above, so the first run is the only
        // non-user one; a pick after boot always applies — user intent beats the
        // environment once the app runs.
        let forced = std::env::var("DAY_THEME").is_ok();
        let stored = day::prefs::get("showcase.appearance").is_some();
        let booted = std::cell::Cell::new(false);
        Effect::new(move || {
            let dark = Appearance::from_key(&s.get()).override_dark();
            if !booted.replace(true) && (forced || !stored) {
                return;
            }
            day::set_appearance(dark);
        });
        AppearanceSetting(s)
    }
}

/// The persisted appearance setting, applied to the running app whenever it changes.
///
/// The `Effect` is what makes this one setting rather than three buttons that each call
/// `set_appearance`: whoever writes the signal — a toolbar toggle, a menu item, a restored
/// preference — the override lands the same way.
fn appearance_signal() -> Signal<String> {
    AppearanceSetting::app().0
}

/// The mode in force. A tracked read, so a surface rendering the group re-lowers when it changes.
pub(crate) fn appearance() -> Appearance {
    appearance_signal().with(|s| Appearance::from_key(s))
}

/// Ask for `mode`. Idempotent: choosing the mode already in force changes nothing.
pub(crate) fn set_appearance(mode: Appearance) {
    appearance_signal().set(mode.key().to_string());
}

/// One appearance mode as a command — `checked` is "this is the mode in force", which is what
/// makes the three read as a radio group in a toolbar and a menu alike.
///
/// Disabled where the toolkit cannot restyle itself (`Cap::Appearance`; today Qt and ArkUI, and
/// Android below API 31), so the affordance is visibly inert rather than silently doing nothing.
/// Android answers that capability from the DEVICE rather than for the backend, which is why this
/// asks rather than testing the target.
pub(crate) fn appearance_command(mode: Appearance) -> Command {
    match mode {
        Appearance::Light => Command {
            id: Appearance::Light.id(),
            title: || Appearance::Light.title(),
            enabled: appearance_supported,
            checked: || appearance() == Appearance::Light,
            run: || set_appearance(Appearance::Light),
        },
        Appearance::System => Command {
            id: Appearance::System.id(),
            title: || Appearance::System.title(),
            enabled: appearance_supported,
            checked: || appearance() == Appearance::System,
            run: || set_appearance(Appearance::System),
        },
        Appearance::Dark => Command {
            id: Appearance::Dark.id(),
            title: || Appearance::Dark.title(),
            enabled: appearance_supported,
            checked: || appearance() == Appearance::Dark,
            run: || set_appearance(Appearance::Dark),
        },
    }
}

/// Whether this backend honours an appearance override at all.
pub(crate) fn appearance_supported() -> bool {
    capability(Cap::Appearance) != Support::Unsupported
}

// ── The recorder ────────────────────────────────────────────────────────────────────────────
//
// The toolbar's transport and the Scripting page drive ONE recording, into the page's buffer: a
// recording started from the toolbar is the script the page shows, and one started on the page is
// what the toolbar's Play plays. Anything else would be two recorders with one Record button.

/// Record ↔ Stop. The title carries the state, as Star does — the item does not need a check mark
/// to say which half it is on.
pub(crate) fn record() -> Command {
    Command {
        id: "cmd-record",
        // Through the SIGNALS, not `is_recording()` / `is_playing()`: those read a RefCell and an
        // atomic, so a surface that consulted them would never learn the state had changed. The
        // signal read is what subscribes the toolbar's builder and each item's `enabled_when`.
        title: || match day::record::recording_signal().get() {
            true => crate::res::str::cmd_stop_recording(),
            false => crate::res::str::cmd_record(),
        },
        // Recording during a replay would capture the replay's own synthesized actions.
        enabled: || !day::record::playing_signal().get(),
        checked: || day::record::recording_signal().get(),
        run: || {
            if day::record::is_recording() {
                day::record::stop();
                // Stopping lands the user ON the script they just recorded: it is the only
                // surface that shows the buffer, and staying wherever the recording happened to
                // end reads as the whole thing having gone nowhere. Ordered after `stop()`,
                // which unhooks the nav observer — otherwise this jump would be the recording's
                // last step, and replaying it would navigate away before the rest could run.
                navigate_to(&Section::Scripting);
            } else {
                crate::pages::scripting::record_into_buffer();
            }
        },
    }
}

/// Play ↔ Pause over the recorded script: Play when idle, Pause while it runs, Play again to
/// resume. One button, because that is what a transport control is; Stop is the recorder's.
pub(crate) fn play_pause() -> Command {
    Command {
        id: "cmd-play",
        title: || match (
            day::record::playing_signal().get(),
            day::record::paused_signal().get(),
        ) {
            (true, false) => crate::res::str::cmd_pause(),
            (true, true) => crate::res::str::cmd_resume(),
            _ => crate::res::str::cmd_play(),
        },
        // Nothing to play until something is recorded (or typed on the Scripting page), and
        // never while recording — a replay must not record itself.
        //
        // All three reads happen EVERY time, before the logic: `||`/`&&` would short-circuit
        // past one of them, and a read that does not happen is a dependency not subscribed —
        // which is how Play stayed disabled after a recording ended (nothing had subscribed to
        // the buffer while recording was live).
        enabled: || {
            let playing = day::record::playing_signal().get();
            let recording = day::record::recording_signal().get();
            let has = crate::pages::scripting::has_script();
            // In-process playback needs a background thread, which wasm has not got: on web the
            // control lowers DISABLED rather than sitting there doing nothing when pressed
            // (docs/web.md — drive the page over the dayscript socket instead). Recording itself
            // works on every target.
            day::record::playback_supported() && (playing || (!recording && has))
        },
        checked: || day::record::playing_signal().get() && !day::record::paused_signal().get(),
        run: || match (day::record::is_playing(), day::record::is_paused()) {
            (true, false) => day::record::pause_playback(),
            (true, true) => day::record::resume_playback(),
            _ => crate::pages::scripting::play_buffer(),
        },
    }
}

/// Throw the recording away — the transport's reset, and the one destructive command here.
pub(crate) fn clear_recording() -> Command {
    Command {
        id: "cmd-clear-recording",
        title: || crate::res::str::cmd_clear_recording(),
        enabled: || {
            let recording = day::record::recording_signal().get();
            let playing = day::record::playing_signal().get();
            let has = crate::pages::scripting::has_script();
            !recording && !playing && has
        },
        checked: || false,
        run: crate::pages::scripting::clear_buffer,
    }
}

/// `Day-Showcase-YYYY-MM-DD-HH-MM-SS.png` — a sortable name the user can still change in the
/// save sheet. UTC, because that is the clock day-piece-datetime offers (see `DayTime::now`).
fn default_shot_name() -> String {
    let d = day_piece_datetime::DayDate::today();
    let t = day_piece_datetime::DayTime::now();
    format!(
        "Day-Showcase-{:04}-{:02}-{:02}-{:02}-{:02}-{:02}.png",
        d.year, d.month, d.day, t.hour, t.minute, t.second
    )
}
