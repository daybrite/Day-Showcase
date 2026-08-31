use day::prelude::*;

use crate::widgets::page;

const DELAY_KEY: &str = "scripting.delay";
const DEFAULT_DELAY: &str = "0.25";

// Global page state (Signal::global outlives the page's rebuilds), so navigating away and back
// preserves the working script, the baseline it was last saved/loaded at (for the dirty check that
// gates Save), and which file on disk it maps to (an in-place Save vs a prompt for a new name).
/// The working script. `pub(crate)` because the toolbar's transport and the App menu record into
/// and play from THIS buffer (commands.rs): one recording, whichever surface starts it.
pub(crate) fn buf_signal() -> Signal<String> {
    crate::scene().script_buf
}

/// The per-step playback delay, in seconds — persisted by the page's own field, and read here so
/// a Play from the toolbar runs at the speed the page is set to.
fn configured_delay_secs() -> f64 {
    day::prefs::get(DELAY_KEY)
        .unwrap_or_else(|| DEFAULT_DELAY.into())
        .trim()
        .parse::<f64>()
        .unwrap_or(0.0)
        .max(0.0)
}

/// Start recording into the shared buffer, excluding the page's own controls. The one place that
/// knows how this app records, so the page button and the toolbar cannot start it differently.
/// The working buffer of the FRONT window, or `None` before any window has built — the app
/// menu's transport items are lowered from surfaces that can run that early (docs/state.md).
fn try_buf() -> Option<Signal<String>> {
    crate::try_scene().map(|s| s.script_buf)
}

pub(crate) fn record_into_buffer() {
    let Some(buf) = try_buf() else { return };
    day::record::exclude_prefix("scripting-");
    day::record::start_into(buf);
}

/// Is there a script to play — recorded, loaded or hand-typed?
pub(crate) fn has_script() -> bool {
    try_buf().is_some_and(|b| b.with(|t| day::record::is_playable(t)))
}

/// Play the shared buffer at the page's configured delay.
pub(crate) fn play_buffer() {
    let Some(buf) = try_buf() else { return };
    let _ = day::record::play_with_delay(&buf.get_untracked(), configured_delay_secs());
}

/// Throw the recording away: the recorder's own steps AND the buffer the surfaces read.
pub(crate) fn clear_buffer() {
    day::record::clear();
    if let Some(buf) = try_buf() {
        buf.set(String::new());
    }
}
fn baseline_signal() -> Signal<String> {
    // Seeded to the initial buffer, so a page with nothing new is NOT dirty (Save disabled); a
    // recording or an edit then diverges from it and enables Save.
    crate::scene().script_baseline
}
fn current_file_signal() -> Signal<Option<String>> {
    crate::scene().script_file
}

/// The Scripting page (DESIGN.md §14.6): record your own taps and navigation into a replayable
/// dayscript, edit it, play it back, and save it to the app's `scripts/` folder for later. The
/// controls carry ids under the `scripting-` prefix, which is what the recorder excludes, so a
/// Record/Stop/Play/Save tap never records itself.
pub(crate) fn scripting_page() -> AnyPiece {
    let buf = buf_signal();
    let baseline = baseline_signal();
    let current_file = current_file_signal();
    // Returning to a rebuilt page mid-recording: keep the live stream flowing into the buffer.
    if day::record::is_recording() {
        day::record::exclude_prefix("scripting-");
        day::record::start_into(buf);
    }
    let recording = day::record::recording_signal();
    let status = Signal::new(String::new());

    // "Delay between steps" (seconds): default 0.25, persisted across navigation AND restarts via
    // prefs. Held as text so the field edits freely; the steppers nudge by a quarter second.
    let delay_text =
        Signal::new(day::prefs::get(DELAY_KEY).unwrap_or_else(|| DEFAULT_DELAY.into()));
    watch(
        move || delay_text.get(),
        move |v, _| {
            day::prefs::set(DELAY_KEY, v);
        },
    );
    let delay_secs = move || delay_text.with(|t| t.trim().parse::<f64>().unwrap_or(0.0).max(0.0));
    let bump = move |by: f64| {
        let next = (delay_secs() + by).max(0.0);
        delay_text.set(format!("{next:.2}"));
    };

    // Saved-script dropdown: the app-container `scripts/` folder, listed at build (so it is
    // populated on launch and refreshed whenever the page is re-entered — e.g. after a new Save).
    // Index 0 is a placeholder; selecting a name loads that file's raw yaml into the buffer.
    let names = saved_script_names();
    let mut options = vec![crate::res::str::scripting_pick().format()];
    options.extend(names.iter().cloned());
    let sel = Signal::new(0usize);
    {
        let names = names.clone();
        watch(
            move || sel.get(),
            move |i, _| {
                if *i == 0 {
                    return;
                }
                let Some(name) = names.get(*i - 1) else {
                    return;
                };
                if let Ok(bytes) = day_part_fs::read(&script_path(name))
                    && let Ok(text) = String::from_utf8(bytes)
                {
                    // Loaded from a file: the buffer IS the file, so it is clean (Save disabled).
                    buf.set(text.clone());
                    baseline.set(text);
                    current_file.set(Some(name.clone()));
                }
            },
        );
    }

    page(
        crate::res::str::nav_scripting(),
        "scripting-title",
        Some(crate::res::str::scripting_caption()),
        form((
            // The editable script — streamed into while recording, hand-editable any time.
            section((text_area(buf)
                .min_lines(12)
                .max_lines(24)
                .id("scripting-buffer"),)),
            // Load a saved script from the app container.
            section((row((
                label(crate::res::str::scripting_saved_label()).font(Font::Footnote),
                picker(options, sel).menu().id("scripting-menu"),
            ))
            .spacing(8.0),)),
            // The controls. Every id starts with `scripting-` so the recorder skips them.
            section((
                row((
                    // Record ↔ Stop: label and fill both track the recording flag reactively.
                    button(move || {
                        if recording.get() {
                            crate::res::str::scripting_stop().format()
                        } else {
                            crate::res::str::scripting_record().format()
                        }
                    })
                    // Through the shared helper, so this button and the toolbar's transport
                    // start the SAME recording into the SAME buffer (commands.rs).
                    .action(move || {
                        if day::record::is_recording() {
                            day::record::stop();
                        } else {
                            record_into_buffer();
                        }
                    })
                    // A REACTIVE tint: the native button recolors in place while recording,
                    // keeping its ripple/press rendering and its role (docs/buttons.md).
                    .tint(move || {
                        if recording.get() {
                            crate::palette::RUST
                        } else {
                            crate::palette::SLATE
                        }
                    })
                    .id("scripting-record"),
                    // Play ↔ Pause ↔ Resume, the same transport the toolbar carries: the title
                    // follows the run so one button covers all three, and it is disabled while
                    // recording and when the script is empty or does not parse.
                    button(move || (crate::commands::play_pause().title)().format())
                        .bordered()
                        .enabled(move || {
                            // Read the flags REACTIVELY (the signals, not the atomics) so the
                            // title and enablement follow a run that ends on its own. On web there
                            // is no in-process playback at all, so the button says so by being
                            // disabled (docs/web.md).
                            let playing = day::record::playing_signal().get();
                            day::record::playback_supported()
                                && (playing
                                    || (!recording.get()
                                        && buf.with(|t| day::record::is_playable(t))))
                        })
                        .action(move || {
                            // The delay the FIELD shows, which may be ahead of what prefs hold.
                            match (day::record::is_playing(), day::record::is_paused()) {
                                (true, false) => day::record::pause_playback(),
                                (true, true) => day::record::resume_playback(),
                                _ => {
                                    let _ = day::record::play_with_delay(
                                        &buf.get_untracked(),
                                        delay_secs(),
                                    );
                                }
                            }
                        })
                        .id("scripting-play"),
                    // Save to the app's scripts folder. Enabled only when the buffer differs from
                    // the last saved/loaded content (dirty). A script already mapped to a file
                    // updates that file IN PLACE; an unsaved one prompts for a name.
                    button(crate::res::str::scripting_save())
                        .bordered()
                        .enabled(move || buf.get() != baseline.get())
                        .action(move || {
                            let content = buf.get_untracked();
                            match current_file.get_untracked() {
                                Some(name) => {
                                    let _ =
                                        day_part_fs::write(&script_path(&name), content.as_bytes());
                                    baseline.set(content); // now clean
                                    status.set(crate::res::str::scripting_saved().format());
                                }
                                None => {
                                    let status = status;
                                    day::task(async move {
                                        let name = prompt(crate::res::str::scripting_save())
                                            .placeholder(crate::res::str::scripting_name_hint())
                                            .present()
                                            .await;
                                        let Some(name) = name else { return };
                                        let name = name.trim().to_string();
                                        if name.is_empty() {
                                            return;
                                        }
                                        let _ = day_part_fs::write(
                                            &script_path(&name),
                                            content.as_bytes(),
                                        );
                                        current_file.set(Some(name));
                                        baseline.set(content); // now clean
                                        status.set(crate::res::str::scripting_saved().format());
                                    });
                                }
                            }
                        })
                        .id("scripting-save"),
                    // Copy the script to the clipboard (docs/clipboard.md), with a transient note.
                    button(crate::res::str::scripting_copy())
                        .bordered()
                        .action(move || {
                            let ok = buf.with(|t| day_part_clipboard::set_text(t));
                            if ok {
                                status.set(crate::res::str::scripting_copied().format());
                            }
                        })
                        .id("scripting-copy"),
                    // Export to a .yaml via the native save-file picker (docs/files.md).
                    button(crate::res::str::scripting_export())
                        .bordered()
                        .action(move || {
                            day::task(async move {
                                let data = buf.get_untracked().into_bytes();
                                let _ = save_file(data)
                                    .title(crate::res::str::scripting_export())
                                    .suggested_name("recording.yaml")
                                    .filter("YAML", &["yaml", "yml"])
                                    .await;
                            });
                        })
                        .id("scripting-export"),
                ))
                .spacing(8.0),
                // "Delay between steps" — an editable field with - / + steppers (Day has no native
                // stepper piece), feeding the per-step Playback pause above.
                row((
                    label(crate::res::str::scripting_delay_label()).font(Font::Footnote),
                    button("\u{2212}")
                        .bordered()
                        .action(move || bump(-0.25))
                        .id("scripting-delay-dec"),
                    text_field(delay_text)
                        .placeholder(DEFAULT_DELAY)
                        .id("scripting-delay"),
                    button("+")
                        .bordered()
                        .action(move || bump(0.25))
                        .id("scripting-delay-inc"),
                    label(crate::res::str::scripting_delay_unit()).font(Font::Footnote),
                ))
                .spacing(8.0),
                label(move || status.get())
                    .font(Font::Footnote)
                    .id("scripting-status"),
            )),
        ))
        .any(),
    )
    .any()
}

// ---------------------------------------------------------------------------
// Saved scripts: the app-container `scripts/` folder (day-part-fs), the dropdown's source, and
// three bundled samples the app writes on launch as a live demo of the scripting feature.
// ---------------------------------------------------------------------------

/// The sandboxed sub-folder under the app's private container where saved scripts live.
pub(crate) const SCRIPTS_DIR: &str = "scripts";

/// Bundled sample scripts (name, yaml) written on launch if absent — a ready demonstration of
/// record/playback that the dropdown shows the first time the app runs. Authored in the compact
/// flow style; `seed_sample_scripts` normalizes them to the recorder's own on-disk shape so a
/// loaded sample reads identically to a freshly recorded one.
pub(crate) const SAMPLES: &[(&str, &str)] = &[
    (
        "Counter",
        "flow:\n\
         - navigate: { route: controls }\n\
         - tap: { id: increment-button }\n\
         - tap: { id: increment-button }\n\
         - tap: { id: increment-button }\n",
    ),
    (
        "Page Tour",
        "flow:\n\
         - navigate: { route: about }\n\
         - navigate: { route: controls }\n\
         - navigate: { route: focus }\n\
         - navigate: { route: scripting }\n",
    ),
    (
        "Focus Demo",
        "flow:\n\
         - navigate: { route: focus }\n\
         - tap: { id: focus-next-button }\n\
         - tap: { id: focus-next-button }\n",
    ),
];

/// `scripts/<name>.yaml` for a script name (the on-disk path the dropdown and Save use).
fn script_path(name: &str) -> String {
    format!("{SCRIPTS_DIR}/{name}.yaml")
}

/// Write each bundled sample that is not already on disk — called once at launch (`lib::root`).
/// Absent-only, so a user's edits to a sample survive a relaunch. Each is round-tripped through the
/// recorder (`steps_from_yaml` → `steps_to_yaml`) so the saved file is byte-for-byte the shape a
/// recording produces; a malformed sample (should never happen) falls back to its literal text.
pub(crate) fn seed_sample_scripts() {
    for (name, yaml) in SAMPLES {
        let path = script_path(name);
        if day_part_fs::read(&path).is_err() {
            let normalized = day::record::steps_from_yaml(yaml)
                .map(|steps| day::record::steps_to_yaml(&steps))
                .unwrap_or_else(|_| (*yaml).to_string());
            let _ = day_part_fs::write(&path, normalized.as_bytes());
        }
    }
}

/// The saved script names (file stems), sorted — the dropdown's items. Best-effort: an unreadable
/// folder yields an empty list, and the page still records/plays.
fn saved_script_names() -> Vec<String> {
    let mut names: Vec<String> = day_part_fs::list(SCRIPTS_DIR)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|f| f.strip_suffix(".yaml").map(str::to_string))
        .collect();
    names.sort();
    names
}
