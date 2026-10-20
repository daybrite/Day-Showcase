//! Application command definitions shared across native menus, toolbars, and content buttons.
//! Day owns presentation adapters and invocation guards; this module owns application behavior.
//! Factories targeting a page capture that page, while app-menu factories resolve the active scene.

use crate::Section;
use day::prelude::*;

// ── Starred pages ───────────────────────────────────────────────────────────────────────────
//
// Persisted as one comma-separated string of route keys rather than a set: `prefs::bind` stores
// `FromStr + ToString`, and the route key is already the stable identity every other subsystem
// (deep links, dayscript, `current_route`) addresses a page by. A page that is renamed or dropped
// stops matching, which is the right way for stale state to expire.

/// The persisted starred set, app-wide (docs/state.md): a preference, the same in every window.
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

/// The sidebar's selection: the app's routing signal, hoisted so every surface can read it
/// reactively.
///
/// `current_route()` cannot serve here: a route surface reports its segments `get_untracked`
/// (reading the route inside a builder must not subscribe that builder to navigation), so
/// a command deriving its state from `current_route` never re-ran when the selection moved; the
/// toolbar's star kept the previous page's on/off state until something else touched the starred
/// set. Reading this signal is a tracked read, so navigating re-lowers the toolbar and the menu.
pub(crate) fn section() -> Signal<Option<Section>> {
    crate::scene().section
}

/// The starred set, created once and reused by every visit and every surface: an app-wide
/// `Ambient`, so it outlives the page subtrees that read it and every rebuild sees the same
/// signal.
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

/// How many pages are starred. A tracked read, for the Settings window's Catalog group.
pub(crate) fn starred_count() -> usize {
    starred().with(|s| s.split(',').filter(|k| !k.is_empty()).count())
}

/// Unstar every page: the Catalog group's one destructive command, confirmed first there.
pub(crate) fn clear_starred() {
    starred().set(String::new());
}

/// The route the star commands act on: whichever page is showing.
///
/// Falls back to About when the route is empty, exactly as `show_source` does: the desktop
/// split selects the first row as its default detail without setting a route, so a command that
/// insisted on a route would sit disabled on the page the user is actually looking at.
fn active_section() -> Option<Section> {
    // A tracked read of the selection signal (see `section`). Falls back to About when nothing is
    // selected, exactly as `show_source` does: the desktop split shows the first row as its
    // default detail without selecting it, and a command that sat disabled on the page the user is
    // looking at would be wrong.
    //
    // `try_scene`, not `scene`: the menu bar is lowered from surfaces that can run before any
    // window has built (the Android app-bar menu is applied from a posted task), and a command
    // with no front window has nothing to act on rather than a reason to panic.
    Some(crate::try_scene()?.section.get().unwrap_or(Section::About))
}

/// The Star command for the active page: "Star" when it is not starred, "Unstar" when it is.
///
/// Disabled when there is no active page to star, which on mobile is the root list itself.
pub(crate) fn star() -> Command {
    Command::new(
        day::id!(cmd_star),
        move || {
            match active_section() {
                Some(s) if is_starred(s) => crate::res::str::cmd_unstar(),
                _ => crate::res::str::cmd_star(),
            }
            .format()
        },
        || {
            if let Some(s) = active_section() {
                toggle_star(s);
            }
        },
    )
    .enabled(|| active_section().is_some())
    .checked(|| active_section().is_some_and(is_starred))
    .shortcut(Shortcut::primary("d"))
    .icon(Symbol::Star)
}

// ── Pseudo-locale ───────────────────────────────────────────────────────────────────────────

/// Whether the locale in force is a pseudo-locale (docs/localization.md): the `-XA` variant of
/// a real locale, accented and expanded for layout testing. A tracked read.
pub(crate) fn pseudo_locale() -> bool {
    day::locale().with(|l| l.ends_with("-XA"))
}

/// View ▸ Toggle Pseudo-Locale: an on/off setting, where the appearance trio below is a
/// one-of-three choice. Both are drawn with `MenuEntry::checked`, which is why they sit
/// side by side: the same check mark serves a switch and a radio group, exactly as it does
/// natively.
///
/// On, the current locale takes the `-XA` suffix (`fr` → `fr-XA`) and every string re-renders
/// accented and expanded; off strips it again. Toggled while a French run is in force it
/// stresses the French strings, not the English ones, which is what makes it a menu item rather
/// than the Localization page's fixed `en-XA` button.
pub(crate) fn pseudo_locale_command() -> Command {
    Command::new(
        day::id!(cmd_pseudo_locale),
        crate::res::str::cmd_toggle_pseudo_locale(),
        || {
            let current = day::locale().get_untracked();
            match current.strip_suffix("-XA") {
                Some(base) => set_locale(base),
                None => set_locale(&format!("{current}-XA")),
            }
        },
    )
    .checked(pseudo_locale)
    .shortcut(Shortcut::primary("x").shift())
}

/// Save a picture of this window (docs/window-image.md).
///
/// The capture is deferred a turn rather than taken inline: this command is reached from a menu
/// and from a toolbar button, and on some backends the menu is still on screen (or the button
/// still drawn pressed) at the moment the action runs, so the picture would show the affordance
/// that took it. One hop lets the chrome settle first.
///
/// Disabled where the toolkit cannot rasterize itself, so the affordance is absent rather than
/// present-and-failing (`Cap::Snapshot`; today that is web-dom).
pub(crate) fn screenshot() -> Command {
    Command::new(
        day::id!(cmd_screenshot),
        crate::res::str::cmd_screenshot(),
        || {
            day::task(async move {
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
    )
    .enabled(|| day::window_image_support() == Support::Native)
    .shortcut(Shortcut::primary("s").alt())
}

// ── Appearance ──────────────────────────────────────────────────────────────────────────────
//
// Three commands over one persisted setting, Day's own `APPEARANCE` pref (docs/settings.md), so
// the toolbar's button group, the View ▸ Appearance menu and the Settings window's Appearance
// row can never disagree about which mode is on: every surface reads and writes the one key,
// and day-piece-settings applies each change to the running app (`wire_general`), honoring the
// launch env (`DAY_THEME`) at boot.

/// Which appearance the app is asking for. `System` is the absence of an override.
pub(crate) use day_piece_settings::Appearance;

/// A mode's position in the toolbar's segmented control: light, system, dark, left to right.
pub(crate) fn appearance_index(mode: Appearance) -> usize {
    match mode {
        Appearance::Light => 0,
        Appearance::System => 1,
        Appearance::Dark => 2,
    }
}

/// The mode a segment index chose. Out of range is `System`, the neutral one.
pub(crate) fn appearance_from_index(i: usize) -> Appearance {
    match i {
        0 => Appearance::Light,
        2 => Appearance::Dark,
        _ => Appearance::System,
    }
}

fn appearance_title(mode: Appearance) -> day::LocalizedText {
    match mode {
        Appearance::Light => crate::res::str::cmd_appearance_light(),
        Appearance::System => crate::res::str::cmd_appearance_system(),
        Appearance::Dark => crate::res::str::cmd_appearance_dark(),
    }
}

fn appearance_id(mode: Appearance) -> day::ElementId {
    match mode {
        Appearance::Light => day::id!(cmd_appearance_light),
        Appearance::System => day::id!(cmd_appearance_system),
        Appearance::Dark => day::id!(cmd_appearance_dark),
    }
}

/// The mode in force. A tracked read, so a surface rendering the group re-lowers when it changes.
pub(crate) fn appearance() -> Appearance {
    day_piece_settings::APPEARANCE.get()
}

/// Ask for `mode`. Idempotent: choosing the mode already in force changes nothing.
pub(crate) fn set_appearance(mode: Appearance) {
    day_piece_settings::APPEARANCE.set(mode);
}

/// One appearance mode as a command; `checked` is "this is the mode in force", which is what
/// makes the three read as a radio group in a toolbar and a menu alike.
///
/// Disabled where the toolkit cannot restyle itself (`Cap::Appearance`; today Qt and ArkUI, and
/// Android below API 31), so the affordance is visibly inert rather than silently doing nothing.
/// Android answers that capability from the device rather than for the backend, which is why this
/// asks rather than testing the target.
pub(crate) fn appearance_command(mode: Appearance) -> Command {
    Command::new(
        appearance_id(mode),
        move || appearance_title(mode).format(),
        move || set_appearance(mode),
    )
    .enabled(appearance_supported)
    .checked(move || appearance() == mode)
    .shortcut(Shortcut::primary((appearance_index(mode) + 1).to_string()).alt())
}

/// Whether this backend honours an appearance override at all.
pub(crate) fn appearance_supported() -> bool {
    capability(Cap::Appearance) != Support::Unsupported
}

// ── The recorder ────────────────────────────────────────────────────────────────────────────
//
// The toolbar's transport and the Scripting page drive one recording, into the page's buffer: a
// recording started from the toolbar is the script the page shows, and one started on the page is
// what the toolbar's Play plays. Anything else would be two recorders with one Record button.

/// Record ↔ Stop. The title and optional check both follow the recorder state.
pub(crate) fn record() -> Command {
    Command::new(
        day::id!(cmd_record),
        move || {
            match day::record::recording_signal().get() {
                true => crate::res::str::cmd_stop_recording(),
                false => crate::res::str::cmd_record(),
            }
            .format()
        },
        || {
            if day::record::is_recording() {
                day::record::stop();
                navigate_to(&Section::Scripting);
            } else {
                crate::pages::scripting::record_into_buffer();
            }
        },
    )
    .enabled(|| !day::record::playing_signal().get())
    .checked(|| day::record::recording_signal().get())
    .shortcut(Shortcut::primary("r").shift())
}

/// Play ↔ Pause over the recorded script: Play when idle, Pause while it runs, Play again to
/// resume. One button, because that is what a transport control is; Stop is the recorder's.
pub(crate) fn play_pause() -> Command {
    play_pause_with_delay(crate::pages::scripting::configured_delay_secs)
}

/// The content field may hold an edit before its preferences write has flushed.
pub(crate) fn play_pause_with_delay(delay: impl Fn() -> f64 + 'static) -> Command {
    Command::new(
        day::id!(cmd_play),
        move || {
            match (
                day::record::playing_signal().get(),
                day::record::paused_signal().get(),
            ) {
                (true, false) => crate::res::str::cmd_pause(),
                (true, true) => crate::res::str::cmd_resume(),
                _ => crate::res::str::cmd_play(),
            }
            .format()
        },
        move || match (day::record::is_playing(), day::record::is_paused()) {
            (true, false) => day::record::pause_playback(),
            (true, true) => day::record::resume_playback(),
            _ => crate::pages::scripting::play_buffer_with_delay(delay()),
        },
    )
    .enabled(|| {
        let playing = day::record::playing_signal().get();
        let recording = day::record::recording_signal().get();
        let has = crate::pages::scripting::has_script();
        day::record::playback_supported() && (playing || (!recording && has))
    })
    .checked(|| day::record::playing_signal().get() && !day::record::paused_signal().get())
    .shortcut(Shortcut::primary("p").shift())
}

/// Throw the recording away: the transport's reset, and the one destructive command here.
pub(crate) fn clear_recording() -> Command {
    Command::new(
        day::id!(cmd_clear_recording),
        crate::res::str::cmd_clear_recording(),
        crate::pages::scripting::clear_buffer,
    )
    .enabled(|| {
        let recording = day::record::recording_signal().get();
        let playing = day::record::playing_signal().get();
        let has = crate::pages::scripting::has_script();
        !recording && !playing && has
    })
    .shortcut(Shortcut::primary("k").shift())
}

/// `Day-Showcase-YYYY-MM-DD-HH-MM-SS.png`, a sortable name the user can still change in the
/// save sheet. UTC, because that is the clock day-piece-datetime offers (see `DayTime::now`).
fn default_shot_name() -> String {
    let d = day_piece_datetime::DayDate::today();
    let t = day_piece_datetime::DayTime::now();
    format!(
        "Day-Showcase-{:04}-{:02}-{:02}-{:02}-{:02}-{:02}.png",
        d.year, d.month, d.day, t.hour, t.minute, t.second
    )
}

/// Star a particular page, independent of whichever window is frontmost at invocation.
pub(crate) fn star_page(section: Section) -> Command {
    Command::new(
        day::id!(cmd_star),
        move || {
            if is_starred(section) {
                crate::res::str::cmd_unstar()
            } else {
                crate::res::str::cmd_star()
            }
            .format()
        },
        move || toggle_star(section),
    )
    .checked(move || is_starred(section))
    .image(crate::res::vectors::star.clone())
}
