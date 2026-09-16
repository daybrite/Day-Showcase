use day::prelude::*;
use day_part_haptics::Haptic;
use day_part_local_notify::{Channel, Importance, Notification, Trigger};
use day_part_sound::{AssetName, Play};

use crate::widgets::page;

/// Platform services (docs/http.md, docs/clipboard.md, docs/prefs.md, docs/sound.md,
/// docs/haptics.md, docs/files.md, docs/notify.md, docs/bridge.md): the headless "do something
/// with the OS" parts, one grouped form section each: text to speech (first: it is the daybridge
/// reference), an HTTP fetch, clipboard round-trip, persisted preferences, sound effects, haptic
/// feedback, local notifications, and the native file pickers.
/// Network & HTTP: what the platform says about connectivity, then day-part-http through the
/// platform's own HTTP stack. The crate-root calls come first (fetch, PATCH, a `Resource`, a
/// free-form URL check); then a `Client` shows each capability against the test server inside the
/// app, and day-part-downloads runs a download that can pause, resume and verify.
pub(crate) fn network_page() -> AnyPiece {
    page(
        crate::res::str::nav_network_http(),
        "network-title",
        form((
            network_section(),
            http_section(),
            download_section(),
            stream_section(),
            upload_section(),
            redirect_section(),
            auth_section(),
            cookie_section(),
            cache_section(),
            websocket_section(),
            timeout_section(),
            trust_section(),
        ))
        .any(),
    )
    .any()
}

/// Notifications & badge: a local notification through the OS, its permission, and the app
/// icon's badge.
pub(crate) fn notify_page() -> AnyPiece {
    page(
        crate::res::str::nav_notify_badge(),
        "notify-page-title",
        form((notify_section(), badge_section())).any(),
    )
    .any()
}

/// Speech, sound & haptics: the parts that talk to the person, meaning the platform's voice, its
/// sound engine, and its haptic engine.
pub(crate) fn speech_page() -> AnyPiece {
    page(
        crate::res::str::nav_speech_haptics(),
        "speech-title",
        form((speech_section(), sound_section(), haptics_section())).any(),
    )
    .any()
}

/// Files & storage: the clipboard, key-value preferences, the native file pickers, and the
/// app-local file store (OPFS on the web).
pub(crate) fn files_page() -> AnyPiece {
    page(
        crate::res::str::nav_files_storage(),
        "files-title",
        form((
            clipboard_section(),
            prefs_section(),
            files_section(),
            storage_section(),
        ))
        .any(),
    )
    .any()
}

/// Text to speech (docs/bridge.md): day-part-speech is daybridge's reference part, one Rust API
/// whose implementation is Swift on Apple, Java on Android, ArkTS on HarmonyOS, JavaScript on the
/// web, C++ (SAPI) on Windows, and C on Linux, all declared in one file. This section is the
/// simplest thing that proves a bridged call ran: type a line (or take the placeholder's) and
/// the device says it.
///
/// `available()` reports what this target's arm promises, so the label is the right answer on a
/// target with no arm (Unsupported) or a partial one (HarmonyOS, whose voices are zh-CN only).
///
/// The state line is the callback tier at work (docs/bridge.md "Callbacks"): `speak_future`
/// resolves when the engine reports the end of the utterance, so "Speaking…" holds exactly as
/// long as the voice does and then says how it ended: finished, or stopped by the Stop button.
fn speech_section() -> impl Piece {
    // Empty means "say the localized sample", which is exactly what the placeholder shows.
    let phrase = Signal::new(String::new());
    // What the engine is doing, in the user's words; empty until the first tap.
    let state = Signal::new(String::new());
    let support = crate::support::speech();
    let support_text = match support {
        Support::Native => crate::res::str::speech_native(),
        Support::Emulated => crate::res::str::speech_emulated(),
        Support::Unsupported => crate::res::str::speech_unsupported(),
    };

    section((
        crate::widgets::support_note(support),
        label(crate::res::str::speech_caption()).font(Font::Footnote),
        labeled(
            crate::res::str::speech_support_label(),
            label(support_text).id("speech-support"),
        ),
        text_field(phrase)
            .placeholder(crate::res::str::speech_phrase())
            .id("speech-text"),
        row((
            button(crate::res::str::speech_speak())
                .action(move || {
                    let typed = phrase.with(|t| t.trim().to_string());
                    let text = if typed.is_empty() {
                        crate::res::str::speech_phrase().format()
                    } else {
                        typed
                    };
                    // The future starts the utterance at once and resumes on the UI thread
                    // when the engine reports its end (docs/async.md), so the state is a plain
                    // signal write on both sides of the await. An `Unsupported` here is the
                    // fallback arm answering, which the support label above already said.
                    state.set(crate::res::str::speech_state_speaking().format());
                    day::task(async move {
                        let ended = day_part_speech::speak_future(&text).await;
                        state.set(match ended {
                            Ok(day_part_speech::SpeechEnd::Finished)
                            | Ok(day_part_speech::SpeechEnd::Unobserved) => {
                                crate::res::str::speech_state_finished().format()
                            }
                            Ok(day_part_speech::SpeechEnd::Stopped) => {
                                crate::res::str::speech_state_stopped().format()
                            }
                            Err(e) => e.to_string(),
                        });
                    });
                })
                .tint(crate::widgets::primary())
                .id("speech-speak"),
            button(crate::res::str::speech_stop())
                .bordered()
                .action(day_part_speech::stop)
                .id("speech-stop"),
        ))
        .spacing(8.0),
        label(move || state.get())
            .font(Font::Footnote)
            .id("speech-state"),
    ))
    .title(crate::res::str::speech_title())
}

fn clipboard_section() -> impl Piece {
    let draft = Signal::new(String::new());
    let pasted = Signal::new(String::new());
    let status = Signal::new(crate::res::str::clipboard_idle().format());
    section((
        label(crate::res::str::clipboard_caption()).font(Font::Footnote),
        text_field(draft)
            .placeholder(crate::res::str::clipboard_placeholder())
            .id("clipboard-field"),
        row((
            button(crate::res::str::clipboard_copy())
                .action(move || {
                    let ok = draft.with(|t| day_part_clipboard::set_text(t));
                    let msg = if ok {
                        crate::res::str::clipboard_copied()
                    } else {
                        crate::res::str::clipboard_copy_failed()
                    };
                    status.set(msg.format());
                })
                .tint(crate::widgets::primary())
                .id("clipboard-copy"),
            button(crate::res::str::clipboard_paste())
                .bordered()
                .action(move || match day_part_clipboard::get_text() {
                    Some(text) => {
                        pasted.set(text);
                        status.set(crate::res::str::clipboard_pasted().format());
                    }
                    None => status.set(crate::res::str::clipboard_empty().format()),
                })
                .id("clipboard-paste"),
            label(move || status.get()).id("clipboard-status"),
        ))
        .spacing(8.0),
        label(move || pasted.get()).id("clipboard-pasted"),
    ))
    .title(crate::res::str::nav_clipboard())
}

fn prefs_section() -> impl Piece {
    const KEY: &str = "showcase.remembered";
    let field = Signal::new(String::new());
    let value = Signal::new(crate::res::str::prefs_empty().format());
    let status = Signal::new(crate::res::str::prefs_idle().format());
    section((
        label(crate::res::str::prefs_caption()).font(Font::Footnote),
        text_field(field)
            .placeholder(crate::res::str::prefs_placeholder())
            .id("prefs-field"),
        row((
            button(crate::res::str::prefs_save())
                .action(move || {
                    let ok = field.with(|t| day::prefs::set(KEY, t));
                    let msg = if ok {
                        crate::res::str::prefs_saved()
                    } else {
                        crate::res::str::prefs_save_failed()
                    };
                    status.set(msg.format());
                })
                .tint(crate::widgets::primary())
                .id("prefs-save"),
            button(crate::res::str::prefs_load())
                .bordered()
                .action(move || match day::prefs::get(KEY) {
                    Some(v) => {
                        value.set(v);
                        status.set(crate::res::str::prefs_loaded().format());
                    }
                    None => {
                        value.set(crate::res::str::prefs_empty().format());
                        status.set(crate::res::str::prefs_missing().format());
                    }
                })
                .id("prefs-load"),
            button(crate::res::str::prefs_clear())
                .action(move || {
                    day::prefs::remove(KEY);
                    value.set(crate::res::str::prefs_empty().format());
                    status.set(crate::res::str::prefs_cleared().format());
                })
                .tint(crate::widgets::danger())
                .id("prefs-clear"),
            label(move || status.get()).id("prefs-status"),
        ))
        .spacing(8.0),
        labeled(
            crate::res::str::prefs_value_label(),
            label(move || value.get()).id("prefs-value"),
        ),
    ))
    .title(crate::res::str::nav_prefs())
}

/// One button that plays a bundled clip at `pan` and shows the part's record of the play in
/// `#sound-last-played`.
fn sound_button(
    id: &'static str,
    title: LocalizedText,
    clip: AssetName,
    pan: f32,
    last: Signal<String>,
) -> impl Piece + use<> {
    button(title)
        .bordered()
        .action(move || {
            day_part_sound::play_with(
                &clip,
                Play {
                    pan,
                    ..Play::default()
                },
            );
            // What the part recorded rather than what was asked, so the readout proves the call
            // arrived (docs/sound.md: a play counts once it passes the switch and the volume).
            if let Some(played) = day_part_sound::recent().pop() {
                last.set(crate::res::str::sound_last_played(played).format());
            }
        })
        .id(id)
        .grow_w()
}

/// Sound effects: six bundled clips, one clip panned across the stereo field, and the master
/// volume every play is scaled by.
fn sound_section() -> impl Piece {
    use crate::res::assets::sounds;
    let last = Signal::new(crate::res::str::sound_none().format());
    let volume = Signal::new(f64::from(day_part_sound::volume()));
    Effect::new(move || day_part_sound::set_volume(volume.get() as f32));
    // Decoded ahead, so the first tap sounds at once.
    day_part_sound::preload(&[
        sounds::click_wav,
        sounds::confirm_wav,
        sounds::error_wav,
        sounds::glass_wav,
        sounds::card_wav,
        sounds::jingle_wav,
    ]);
    let clip = |id, title, clip| sound_button(id, title, clip, 0.0, last);
    let panned = |id, title, pan| sound_button(id, title, sounds::glass_wav, pan, last);
    section((
        crate::widgets::support_note(crate::support::sound()),
        // Equal-width cells, as in the haptics grid below.
        grid((
            grid_row((
                clip(
                    "sound-click",
                    crate::res::str::sound_click(),
                    sounds::click_wav,
                ),
                clip(
                    "sound-confirm",
                    crate::res::str::sound_confirm(),
                    sounds::confirm_wav,
                ),
                clip(
                    "sound-error",
                    crate::res::str::sound_error(),
                    sounds::error_wav,
                ),
            )),
            grid_row((
                clip(
                    "sound-glass",
                    crate::res::str::sound_glass(),
                    sounds::glass_wav,
                ),
                clip(
                    "sound-card",
                    crate::res::str::sound_card(),
                    sounds::card_wav,
                ),
                clip(
                    "sound-jingle",
                    crate::res::str::sound_jingle(),
                    sounds::jingle_wav,
                ),
            )),
        ))
        .spacing(8.0),
        label(crate::res::str::sound_pan_caption()).font(Font::Footnote),
        grid((grid_row((
            panned("sound-left", crate::res::str::sound_left(), -1.0),
            panned("sound-center", crate::res::str::sound_center(), 0.0),
            panned("sound-right", crate::res::str::sound_right(), 1.0),
        )),))
        .spacing(8.0),
        labeled(
            crate::res::str::sound_volume(),
            slider(volume).range(0.0..=1.0).id("sound-volume"),
        ),
        labeled(
            crate::res::str::sound_last(),
            label(move || last.get()).id("sound-last-played"),
        ),
    ))
    .title(crate::res::str::nav_sound())
}

/// One button that plays a haptic and records the style name into `#haptics-last-played`.
fn haptic_button(
    id: &'static str,
    title: LocalizedText,
    h: Haptic,
    playing: Signal<bool>,
    last: Signal<String>,
) -> impl Piece + use<> {
    button(title)
        .bordered()
        // A single tap fired mid-song would land inside the rhythm and read as part of it, so every
        // haptic control greys out for the duration. The native control does the greying.
        .enabled(move || !playing.get())
        .action(move || {
            day_part_haptics::play(h);
            last.set(crate::res::str::haptics_last_played(format!("{h:?}")).format());
        })
        .id(id)
        // `.grow_w()` is what makes the grid column flexible. With every cell flexible the layout
        // splits the leftover width evenly between the columns (docs/grid.md §3), so the buttons
        // come out identical whatever the label length or screen width. That is the reason a
        // `row` was wrong here: it sized each button to its own text and ran off the edge.
        .grow_w()
}

/// One step of a haptic "song": wait `delay_ms`, then fire `haptic`.
type Beat = (u32, Haptic);

/// Play a timed sequence on `day::task` (docs/async.md), which polls on the UI thread, where
/// `day_part_haptics::play` has to be called anyway, so no thread hop is needed. `playing` guards
/// against a second tap overlapping the first, which would garble the rhythm into noise.
fn play_song(
    name: &'static str,
    beats: &'static [Beat],
    playing: Signal<bool>,
    last: Signal<String>,
) {
    if playing.get() {
        return;
    }
    playing.set(true);
    last.set(crate::res::str::haptics_last_played(name.to_string()).format());
    day::task(async move {
        for (delay, h) in beats {
            day::sleep(*delay).await;
            day_part_haptics::play(*h);
        }
        playing.set(false);
    });
}

// The songs.
//
// Tempo grid. The beats are written against 120 BPM rather than in ad-hoc milliseconds, because
// that is what separates a rhythm from a list of buzzes: repetition on a grid is what the ear
// (and the hand) hears as musical. Accelerandos leave the grid, which is why those runs carry
// explicit millisecond gaps.
//
// Dynamic range, and what varies per platform. iOS maps the seven styles onto three
// impact intensities plus three multi-tap notification patterns, so all seven feel distinct.
// Android collapses them: Light/Selection are both EFFECT_TICK, Heavy and Warning are both
// EFFECT_HEAVY_CLICK, Success and Error are both EFFECT_DOUBLE_CLICK. So the palette these
// songs compose against is four sensations (tick (quietest), click, heavy click (loudest single
// hit), and double click (the accent)), and the contrast is built from Selection/Light against
// Heavy, with Error reserved for phrase-ending crashes. Leaning on Warning-vs-Heavy would have
// sounded like a difference on iPhone and like nothing at all on a Pixel.
//
// A notification-style haptic (Success/Error) plays its own multi-tap pattern over ~150-300 ms, so
// nothing is scheduled tight behind one; it would collide rather than syncopate.

/// Quarter note at 120 BPM.
const Q: u32 = 500;
/// Eighth.
const E: u32 = 250;
/// Sixteenth.
const S: u32 = 125;
/// Thirty-second, around the floor where the engine still resolves separate taps rather than
/// smearing them into one buzz.
const T: u32 = 63;

/// 5.6 s. The Duolingo shape: a pickup that rises into a downbeat, a two-bar phrase answered by a
/// denser repeat, a crash, and a resolve. Maximum contrast: near-silent Selection ticks a beat
/// away from full Heavy hits.
const CELEBRATION: &[Beat] = &[
    // Pickup: three rising sixteenths into the bar line.
    (0, Haptic::Selection),
    (S, Haptic::Light),
    (S, Haptic::Medium),
    (S, Haptic::Heavy), // downbeat, full force
    (E, Haptic::Light),
    (S, Haptic::Selection),
    (S, Haptic::Light),
    (E, Haptic::Heavy), // beat 2
    (E, Haptic::Selection),
    (T, Haptic::Selection),
    (T, Haptic::Selection),
    (E, Haptic::Heavy),
    (S, Haptic::Medium),
    (S, Haptic::Heavy),
    (E, Haptic::Success), // phrase accent
    // Second bar: same skeleton, twice the density.
    (Q, Haptic::Heavy),
    (T, Haptic::Light),
    (T, Haptic::Light),
    (T, Haptic::Light),
    (E, Haptic::Heavy),
    (T, Haptic::Light),
    (T, Haptic::Light),
    (T, Haptic::Light),
    (E, Haptic::Heavy),
    (E, Haptic::Heavy),
    (E, Haptic::Error), // crash
    (Q, Haptic::Success),
    (E, Haptic::Selection),
    (E, Haptic::Selection),
    (Q, Haptic::Success),
];

/// 7.5 s. A riser and a drop. The pulse accelerates from a quarter note to a near-continuous buzz
/// while the intensity climbs tick → click → heavy, then everything stops dead for most of a second
/// before the hit lands. The silence is the loudest part.
const LEVEL_UP: &[Beat] = &[
    (0, Haptic::Selection),
    (Q, Haptic::Selection),
    (450, Haptic::Light),
    (400, Haptic::Light),
    (350, Haptic::Medium),
    (300, Haptic::Medium),
    (260, Haptic::Medium),
    (220, Haptic::Heavy),
    (190, Haptic::Heavy),
    (160, Haptic::Heavy),
    (135, Haptic::Heavy),
    (115, Haptic::Heavy),
    (100, Haptic::Heavy),
    (85, Haptic::Heavy),
    (75, Haptic::Heavy),
    (65, Haptic::Heavy),
    (58, Haptic::Heavy),
    (52, Haptic::Heavy),
    (48, Haptic::Heavy),
    (45, Haptic::Heavy),
    (45, Haptic::Heavy),
    (45, Haptic::Heavy),
    // The drop: dead air, then the biggest thing the platform has.
    (750, Haptic::Error),
    (Q, Haptic::Heavy),
    (E, Haptic::Heavy),
    (E, Haptic::Heavy),
    // Fanfare, back on the grid.
    (Q, Haptic::Success),
    (E, Haptic::Medium),
    (E, Haptic::Heavy),
    (Q, Haptic::Success),
    (Q, Haptic::Error),
];

/// 8.0 s. Resting pulse, exertion, panic, flatline, one last beat. The widest dynamic swing of the
/// four: a 12-tick flatline buzz at the noise floor sits directly between full-force Heavy pairs.
const HEARTBEAT: &[Beat] = &[
    // At rest: lub-dub, slow.
    (0, Haptic::Heavy),
    (180, Haptic::Medium),
    (900, Haptic::Heavy),
    (180, Haptic::Medium),
    (820, Haptic::Heavy),
    (170, Haptic::Medium),
    // Quickening.
    (680, Haptic::Heavy),
    (160, Haptic::Medium),
    (560, Haptic::Heavy),
    (150, Haptic::Medium),
    (460, Haptic::Heavy),
    (140, Haptic::Medium),
    (380, Haptic::Heavy),
    (130, Haptic::Medium),
    // Panic: both halves of the beat at full force.
    (300, Haptic::Heavy),
    (120, Haptic::Heavy),
    (240, Haptic::Heavy),
    (110, Haptic::Heavy),
    (200, Haptic::Heavy),
    (100, Haptic::Heavy),
    // Flatline: the quietest sensation the platform has, held.
    (280, Haptic::Selection),
    (55, Haptic::Selection),
    (55, Haptic::Selection),
    (55, Haptic::Selection),
    (55, Haptic::Selection),
    (55, Haptic::Selection),
    (55, Haptic::Selection),
    (55, Haptic::Selection),
    (55, Haptic::Selection),
    // One last beat, then release.
    (620, Haptic::Heavy),
    (700, Haptic::Success),
];

/// 5.4 s. A fall and a climb, mirrored: heavy hits tumble away into a near-continuous tick, hold
/// at the bottom, then rebuild (decelerating as they intensify) into a crash.
const CASCADE: &[Beat] = &[
    // Fall: loud and slow to quiet and fast.
    (0, Haptic::Heavy),
    (E, Haptic::Heavy),
    (S + T, Haptic::Medium),
    (S, Haptic::Medium),
    (100, Haptic::Light),
    (85, Haptic::Light),
    (70, Haptic::Selection),
    (60, Haptic::Selection),
    (52, Haptic::Selection),
    (46, Haptic::Selection),
    (42, Haptic::Selection),
    (40, Haptic::Selection),
    (40, Haptic::Selection),
    (40, Haptic::Selection),
    // Bottom of the arc.
    (Q, Haptic::Selection),
    // Climb: the fall run backwards, slowing down as it gets heavier.
    (42, Haptic::Selection),
    (48, Haptic::Selection),
    (58, Haptic::Light),
    (72, Haptic::Light),
    (92, Haptic::Medium),
    (118, Haptic::Medium),
    (150, Haptic::Heavy),
    (195, Haptic::Heavy),
    (250, Haptic::Heavy),
    (E, Haptic::Error),
    // Second fall, heavier and shorter: the pattern the ear now expects, delivered harder.
    (Q, Haptic::Heavy),
    (S, Haptic::Heavy),
    (S, Haptic::Heavy),
    (100, Haptic::Medium),
    (80, Haptic::Medium),
    (65, Haptic::Light),
    (52, Haptic::Selection),
    (44, Haptic::Selection),
    (40, Haptic::Selection),
    (40, Haptic::Selection),
    (40, Haptic::Selection),
    (E, Haptic::Heavy),
    (S, Haptic::Heavy),
    (S, Haptic::Heavy),
    (E, Haptic::Error),
    (Q, Haptic::Success),
];

fn haptics_section() -> impl Piece {
    let last = Signal::new(crate::res::str::haptics_none().format());
    let playing = Signal::new(false);
    // Report whether this platform has a haptic engine (each branch a full `tr(...)` for `day lint`).
    let supported = if day_part_haptics::is_supported() {
        crate::res::str::haptics_supported_yes()
    } else {
        crate::res::str::haptics_supported_no()
    };
    section((
        crate::widgets::support_note(crate::support::haptics()),
        label(supported)
            .font(Font::Footnote)
            .id("haptics-supported"),
        // A grid, not rows: every cell is `grow_w`, so all three columns are flexible and the
        // layout divides the width evenly between them. Buttons stay the same size and none can
        // overflow, whatever the label or the screen. The trailing `spacer()`s are inert cells that
        // hold the last row's columns open so its buttons match the rows above.
        grid((
            grid_row((
                haptic_button(
                    "haptics-light",
                    crate::res::str::haptics_light(),
                    Haptic::Light,
                    playing,
                    last,
                ),
                haptic_button(
                    "haptics-medium",
                    crate::res::str::haptics_medium(),
                    Haptic::Medium,
                    playing,
                    last,
                ),
                haptic_button(
                    "haptics-heavy",
                    crate::res::str::haptics_heavy(),
                    Haptic::Heavy,
                    playing,
                    last,
                ),
            )),
            grid_row((
                haptic_button(
                    "haptics-success",
                    crate::res::str::haptics_success(),
                    Haptic::Success,
                    playing,
                    last,
                ),
                haptic_button(
                    "haptics-warning",
                    crate::res::str::haptics_warning(),
                    Haptic::Warning,
                    playing,
                    last,
                ),
                haptic_button(
                    "haptics-error",
                    crate::res::str::haptics_error(),
                    Haptic::Error,
                    playing,
                    last,
                ),
            )),
            grid_row((
                haptic_button(
                    "haptics-selection",
                    crate::res::str::haptics_selection(),
                    Haptic::Selection,
                    playing,
                    last,
                ),
                spacer(),
                spacer(),
            )),
        ))
        .spacing(8.0),
        label(crate::res::str::haptics_songs_caption()).font(Font::Footnote),
        // The songs get filled colors so they read as a different kind of control from the single
        // taps above. Each fill picks its own label color: the three saturated ones take white,
        // and `AMBER` takes `tinted_pale`, which swaps in `INK` text; white on a pale fill is the
        // contrast case that variant exists for.
        grid((
            grid_row((
                song_button(
                    "haptics-song-celebration",
                    crate::res::str::haptics_song_celebration(),
                    "Celebration",
                    CELEBRATION,
                    crate::widgets::tinted(crate::palette::TEAL),
                    playing,
                    last,
                ),
                song_button(
                    "haptics-song-levelup",
                    crate::res::str::haptics_song_levelup(),
                    "Level up",
                    LEVEL_UP,
                    crate::widgets::tinted(crate::palette::VIOLET),
                    playing,
                    last,
                ),
            )),
            grid_row((
                song_button(
                    "haptics-song-heartbeat",
                    crate::res::str::haptics_song_heartbeat(),
                    "Heartbeat",
                    HEARTBEAT,
                    crate::widgets::tinted(crate::palette::CORAL),
                    playing,
                    last,
                ),
                song_button(
                    "haptics-song-cascade",
                    crate::res::str::haptics_song_cascade(),
                    "Cascade",
                    CASCADE,
                    crate::widgets::tinted(crate::palette::AMBER),
                    playing,
                    last,
                ),
            )),
        ))
        .spacing(8.0),
        labeled(
            crate::res::str::haptics_last(),
            label(move || last.get()).id("haptics-last-played"),
        ),
    ))
    .title(crate::res::str::nav_haptics())
}

/// A colored button that plays one haptic song.
#[allow(clippy::too_many_arguments)]
fn song_button(
    id: &'static str,
    title: LocalizedText,
    name: &'static str,
    beats: &'static [Beat],
    tint: Color,
    playing: Signal<bool>,
    last: Signal<String>,
) -> impl Piece + use<> {
    // `.tint` is a Button method, so it comes before the Decorate modifiers; the grid modifier
    // goes last, per docs/grid.md's ordering rule.
    button(title)
        .enabled(move || !playing.get())
        .action(move || play_song(name, beats, playing, last))
        .tint(tint)
        .id(id)
        .grow_w()
}

fn notify_section() -> impl Piece {
    let caps = day_part_local_notify::capabilities();
    let levels = [
        Importance::Low,
        Importance::Default,
        Importance::High,
        Importance::Urgent,
    ];

    let title = Signal::new(crate::res::str::notify_title_default().format());
    let body = Signal::new(crate::res::str::notify_body_default().format());
    let delay_idx = Signal::new(0usize);
    // High, not Default: Android only shows a heads-up banner from IMPORTANCE_HIGH up, and a
    // notification that lands silently in the shade reads as "the button did nothing".
    let level_idx = Signal::new(2usize);
    let sound = Signal::new(true);
    let badge = Signal::new(0.0f64);
    let status = Signal::new(crate::res::str::notify_status_idle().format());
    // Whether this platform keeps a consent record at all. Static: a target does not grow a
    // permissions database at runtime.
    let prompts = day_part_permissions::gate(day_part_permissions::Permission::Notifications)
        == day_part_permissions::Gate::Prompts;
    let granted = Signal::new(
        day_part_permissions::status(day_part_permissions::Permission::Notifications)
            == day_part_permissions::Status::Granted,
    );
    // Reactive, because a denial flips it: `request` stops showing a dialog once the answer is
    // final and the affordance must switch to Open Settings.
    let can_prompt = Signal::new(day_part_permissions::can_prompt(
        day_part_permissions::Permission::Notifications,
    ));
    // Prime both from the authoritative status. Notifications are the one Apple permission with no
    // synchronous accessor: the first `status()` answers `Unknown` while it fills its cache in the
    // background, so `can_prompt()` (which is `status == Prompt` there) reads false on a fresh
    // install and the button would offer Open Settings when a real prompt was still available.
    {
        let g = granted.setter();
        let c = can_prompt.setter();
        day_part_permissions::status_async(
            day_part_permissions::Permission::Notifications,
            move |s| {
                g.set(s == day_part_permissions::Status::Granted);
                // Re-read rather than derive: the cache is primed now, and Android computes
                // can_prompt from its own rationale rules rather than from the status alone.
                c.set(day_part_permissions::can_prompt(
                    day_part_permissions::Permission::Notifications,
                ));
            },
        );
    }

    let supported = if caps.post {
        crate::res::str::notify_caps_post()
    } else {
        crate::res::str::notify_caps_unsupported()
    };
    let scheduling = if caps.schedule_while_dead {
        crate::res::str::notify_caps_schedule_os()
    } else {
        crate::res::str::notify_caps_schedule_process()
    };

    section((
        crate::widgets::support_note(crate::support::notifications()),
        label(supported).font(Font::Footnote).id("notify-supported"),
        label(scheduling)
            .font(Font::Footnote)
            .id("notify-scheduling"),
        labeled(
            crate::res::str::notify_title_label(),
            text_field(title)
                .placeholder(crate::res::str::notify_title_placeholder())
                .id("notify-title"),
        ),
        labeled(
            crate::res::str::notify_body_label(),
            text_field(body)
                .placeholder(crate::res::str::notify_body_placeholder())
                .id("notify-body"),
        ),
        labeled(
            crate::res::str::notify_delay(),
            picker(
                vec![
                    crate::res::str::notify_delay_now().format(),
                    crate::res::str::notify_delay_5s().format(),
                    crate::res::str::notify_delay_15s().format(),
                    crate::res::str::notify_delay_60s().format(),
                ],
                delay_idx,
            )
            .id("notify-delay"),
        ),
        labeled(
            crate::res::str::notify_importance(),
            picker(
                vec![
                    crate::res::str::notify_importance_low().format(),
                    crate::res::str::notify_importance_default().format(),
                    crate::res::str::notify_importance_high().format(),
                    crate::res::str::notify_importance_urgent().format(),
                ],
                level_idx,
            )
            .id("notify-importance"),
        ),
        labeled(
            crate::res::str::notify_sound(),
            toggle(sound).id("notify-sound"),
        ),
        // Badge is Apple-only among the wired backends. Day's Decorate trait has no `disabled`,
        // so the control is omitted where it would do nothing rather than shown doing nothing.
        when(
            move || caps.badge,
            move || {
                labeled(
                    crate::res::str::notify_badge(),
                    slider(badge).range(0.0..=9.0).step(1.0).id("notify-badge"),
                )
            },
        ),
        // The consent line, ahead of the controls: on Apple an unauthorized post is accepted and
        // then dropped by the system with no error, so without this the page would look broken.
        //
        // What is offered depends on what the platform actually does about this permission
        // (docs/permissions.md). `Gate::Absent`/`Ungated` mean no consent record exists (desktop
        // Linux and Windows have no database to ask), so a Request button there would be a
        // control that provably does nothing, and none is shown. Where the OS does prompt, the
        // affordance still changes: once the answer is final, `request` no longer puts a dialog
        // on screen and Settings is the only remedy, which is why `can_prompt` picks the label
        // and the action.
        when(
            move || prompts,
            move || {
                // A column, not a row: the status sentence is long enough that sharing a line
                // squeezed the button off the edge of a phone screen.
                column((
                    label(move || {
                        if granted.get() {
                            crate::res::str::notify_perm_granted().format()
                        } else {
                            crate::res::str::notify_perm_missing().format()
                        }
                    })
                    .font(Font::Footnote)
                    .id("notify-perm"),
                    when(
                        move || !granted.get(),
                        move || {
                            button(move || {
                                if can_prompt.get() {
                                    crate::res::str::notify_perm_request().format()
                                } else {
                                    crate::res::str::perm_open_settings().format()
                                }
                            })
                            .bordered()
                            .action(move || {
                                if can_prompt.get() {
                                    // The callback can land on another thread, and Signal is
                                    // !Send; a Setter is the sanctioned cross-thread door
                                    // (DESIGN.md §3.3).
                                    let set = granted.setter();
                                    let still = can_prompt.setter();
                                    day_part_permissions::request(
                                        day_part_permissions::Permission::Notifications,
                                        move |s| {
                                            set.set(s == day_part_permissions::Status::Granted);
                                            // A denial usually makes the answer final, so the
                                            // button has to become Open Settings.
                                            still.set(day_part_permissions::can_prompt(
                                                day_part_permissions::Permission::Notifications,
                                            ));
                                        },
                                    );
                                } else {
                                    day_part_permissions::open_settings(
                                        day_part_permissions::Permission::Notifications,
                                    );
                                }
                            })
                            .id("notify-perm-request")
                        },
                    ),
                ))
                .spacing(6.0)
                .align(HAlign::Leading)
            },
        ),
        row((
            button(crate::res::str::notify_post())
                .prominent()
                .action(move || {
                    let level = levels[level_idx.get().min(levels.len() - 1)];
                    // Re-register on every post so the sound toggle takes effect; registration is
                    // idempotent, and each level keeps its own channel id.
                    let chan = format!("showcase-{}", level.as_str());
                    Channel::new(chan.clone(), level)
                        .sound(sound.get())
                        .register();
                    let secs = [0u64, 5, 15, 60][delay_idx.get().min(3)];
                    let trigger = if secs == 0 {
                        Trigger::Now
                    } else {
                        Trigger::In(std::time::Duration::from_secs(secs))
                    };
                    let mut n = Notification::new(title.get())
                        .body(body.get())
                        .channel(chan)
                        .route("services")
                        .trigger(trigger);
                    let count = badge.get() as u32;
                    if count > 0 {
                        n = n.badge(count);
                    }
                    let msg = match n.post() {
                        Ok(id) => {
                            let head = if secs == 0 {
                                crate::res::str::notify_status_posted()
                            } else {
                                crate::res::str::notify_status_scheduled()
                            };
                            format!("{} (#{})", head.format(), id.0)
                        }
                        Err(e) => {
                            format!("{}: {e}", crate::res::str::notify_status_failed().format())
                        }
                    };
                    status.set(msg);
                })
                .tint(crate::widgets::primary())
                .id("notify-post"),
            button(crate::res::str::notify_cancel())
                .bordered()
                .action(move || {
                    day_part_local_notify::cancel_all();
                    status.set(crate::res::str::notify_status_cancelled().format());
                })
                .id("notify-cancel"),
        ))
        .spacing(8.0),
        labeled(
            crate::res::str::notify_last(),
            label(move || status.get()).id("notify-status"),
        ),
    ))
    .title(crate::res::str::nav_notify())
}

/// App-icon badge (docs/badge.md). The capability line comes first because this is the feature
/// whose support varies most: macOS renders arbitrary text, iOS and the web take a number,
/// Android has no API for it at all and says so.
fn badge_section() -> impl Piece {
    let count = Signal::new(0.0f64);
    let status = Signal::new(crate::res::str::badge_status_idle().format());

    let can_count = capability(Cap::AppBadgeCount);
    let can_text = capability(Cap::AppBadgeText) == Support::Native;
    let supported = can_count != Support::Unsupported;

    // Three states, not two: `Emulated` means the call is made and the shell may ignore it (the
    // web unless installed, and desktop Linux under a shell that skips the Unity protocol).
    let caps_line = match can_count {
        Support::Native => crate::res::str::badge_caps_native(),
        Support::Emulated => crate::res::str::badge_caps_emulated(),
        Support::Unsupported => crate::res::str::badge_caps_none(),
    };

    section((
        crate::widgets::support_note(crate::support::cap(Cap::AppBadgeCount)),
        label(caps_line).font(Font::Footnote).id("badge-supported"),
        // Named so a user on Android reads why rather than assuming it is broken.
        when(
            move || !supported,
            move || {
                label(crate::res::str::badge_android_note())
                    .font(Font::Footnote)
                    .id("badge-unsupported-why")
            },
        ),
        labeled(
            crate::res::str::badge_count_label(),
            row((
                button(crate::res::str::badge_minus())
                    .bordered()
                    .enabled(move || supported && count.get() > 0.0)
                    .action(move || count.set((count.get() - 1.0).max(0.0)))
                    .id("badge-minus"),
                label(move || format!("{}", count.get() as u32))
                    .tabular()
                    .id("badge-value"),
                button(crate::res::str::badge_plus())
                    .bordered()
                    .enabled(move || supported && count.get() < 99.0)
                    .action(move || count.set((count.get() + 1.0).min(99.0)))
                    .id("badge-plus"),
            ))
            .spacing(8.0),
        ),
        row((
            button(crate::res::str::badge_set())
                .prominent()
                .enabled(supported)
                .action(move || {
                    let n = count.get() as u32;
                    day::set_app_badge(&day::AppBadge::Count(n));
                    status.set(crate::res::str::badge_status_set(n.to_string()).format());
                })
                .id("badge-set"),
            button(crate::res::str::badge_clear())
                .bordered()
                .enabled(supported)
                .action(move || {
                    day::set_app_badge(&day::AppBadge::None);
                    status.set(crate::res::str::badge_status_cleared().format());
                })
                .id("badge-clear"),
            // Text is macOS-only, so the control is not offered elsewhere rather than
            // sitting there doing nothing.
            when(
                move || can_text,
                move || {
                    button(crate::res::str::badge_set_text())
                        .bordered()
                        .action(move || {
                            day::set_app_badge(&day::AppBadge::Text("beta".into()));
                            status.set(crate::res::str::badge_status_text().format());
                        })
                        .id("badge-set-text")
                },
            ),
        ))
        .spacing(8.0),
        labeled(
            crate::res::str::badge_last(),
            label(move || status.get()).id("badge-status"),
        ),
    ))
    .title(crate::res::str::nav_badge())
}

fn files_section() -> impl Piece {
    // The editor text: what "Save" writes and what "Open" loads into.
    let content = Signal::new(crate::res::str::files_initial_content().format());
    let status = Signal::new(String::new());
    let opened = Signal::new(String::new());
    section((
        crate::widgets::support_note(crate::support::cap(Cap::FileDialogs)),
        label(crate::res::str::files_caption()).font(Font::Footnote),
        text_field(content)
            .placeholder(crate::res::str::files_placeholder())
            .id("files-content"),
        row((
            button(crate::res::str::files_open())
                .action(move || {
                    day::task(async move {
                        match open_file()
                            .title(crate::res::str::files_open())
                            .filter("Text", &["txt", "md"])
                            .await
                        {
                            Some(file) => match file.read_to_string() {
                                Ok(text) => {
                                    content.set(text);
                                    opened.set(file.file_name().unwrap_or_default());
                                    status.set("opened".into());
                                }
                                Err(_) => status.set("open-error".into()),
                            },
                            None => status.set("open-cancel".into()),
                        }
                    });
                })
                .tint(crate::widgets::primary())
                .id("btn-open-file"),
            button(crate::res::str::files_save())
                .bordered()
                .action(move || {
                    day::task(async move {
                        let data = content.get_untracked().into_bytes();
                        match save_file(data)
                            .title(crate::res::str::files_save())
                            .suggested_name("day-notes.txt")
                            .filter("Text", &["txt"])
                            .await
                        {
                            Some(dest) => status
                                .set(format!("saved:{}", dest.file_name().unwrap_or_default())),
                            None => status.set("save-cancel".into()),
                        }
                    });
                })
                .id("btn-save-file"),
            label(move || status.get()).id("files-status"),
        ))
        .spacing(8.0),
        when(
            move || !opened.with(|s| s.is_empty()),
            move || label(crate::res::str::files_opened(opened)).id("files-opened-name"),
        ),
    ))
    .title(crate::res::str::nav_files())
}

/// The loopback test server every demonstration on the Network & HTTP page talks to, started on
/// first use (docs/http.md "Testing"). It needs no network, so the page behaves the same in
/// airplane mode, on CI, and behind a proxy.
#[cfg(not(target_arch = "wasm32"))]
fn test_server() -> Result<&'static day_part_http::testing::Server, String> {
    static SERVER: std::sync::OnceLock<Result<day_part_http::testing::Server, String>> =
        std::sync::OnceLock::new();
    SERVER
        .get_or_init(|| day_part_http::testing::Server::start().map_err(|e| e.to_string()))
        .as_ref()
        .map_err(Clone::clone)
}

/// A URL on the test server. A browser tab can host no listener: on the web the root path goes
/// to `day launch`'s dev server, whose same-origin `day-http-ok` endpoint answers with the
/// identical bodies (crates/day-cli/src/web.rs), and every other path reports that it needs the
/// server inside the app.
fn local_url(path: &str) -> Result<String, String> {
    #[cfg(target_arch = "wasm32")]
    {
        // Relative, so it resolves against the page origin (and subpath, e.g. the
        // project-Pages /Day-Showcase/), keeping the request same-origin with no CORS.
        if path == "/" {
            return Ok("day-http-ok".into());
        }
        Err(crate::res::str::network_needs_server().format())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        test_server().map(|server| server.url(path))
    }
}

/// A `ws://` URL on the test server.
fn local_ws_url(path: &str) -> Result<String, String> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = path;
        Err(crate::res::str::network_needs_server().format())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        test_server().map(|server| server.ws_url(path))
    }
}

/// `"<status> <body>"` or `"error: …"`. Raw, so it is identical in every locale and the
/// scripts can assert it exactly.
fn status_line(result: Result<day_part_http::Response, day_part_http::HttpError>) -> String {
    match result {
        Ok(resp) => format!("{} {}", resp.status, resp.text()),
        Err(e) => format!("error: {e}"),
    }
}

/// `"<status>"` or `"error: …"`, for responses whose body is a whole web page.
fn status_only(result: Result<day_part_http::Response, day_part_http::HttpError>) -> String {
    match result {
        Ok(resp) => resp.status.to_string(),
        Err(e) => format!("error: {e}"),
    }
}

/// The banner for a demonstration that needs a capability this platform may lack.
fn needs(available: bool) -> impl Piece {
    crate::widgets::support_note(if available {
        Support::Native
    } else {
        Support::Unsupported
    })
}

/// Bytes as MiB with one decimal.
fn mib(bytes: u64) -> String {
    format!("{:.1} MiB", bytes as f64 / f64::from(1u32 << 20))
}

/// Every capability the platform's client reports, by its API name.
fn capabilities_line() -> String {
    let c = day_part_http::capabilities();
    let names = [
        ("streaming", c.streaming),
        ("upload_streaming", c.upload_streaming),
        ("upload_progress", c.upload_progress),
        ("manual_redirects", c.manual_redirects),
        ("auth_questions", c.auth_questions),
        ("native_auth_schemes", c.native_auth_schemes),
        ("server_trust", c.server_trust),
        ("client_identity", c.client_identity),
        ("platform_cookies", c.platform_cookies),
        ("platform_cache", c.platform_cache),
        ("metrics", c.metrics),
        ("websockets", c.websockets),
        ("websocket_ping", c.websocket_ping),
        ("websocket_headers", c.websocket_headers),
        ("wait_for_connectivity", c.wait_for_connectivity),
    ];
    let on: Vec<&str> = names
        .iter()
        .filter(|(_, on)| *on)
        .map(|(name, _)| *name)
        .collect();
    if on.is_empty() {
        "\u{2014}".into()
    } else {
        on.join(", ")
    }
}

/// Transfer metrics on one line: protocol, timings, connection reuse, address, TLS version.
fn metrics_line(m: &day_part_http::Metrics) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(protocol) = &m.protocol {
        parts.push(protocol.clone());
    }
    if let Some(total) = m.total {
        parts.push(format!("{} ms", total.as_millis()));
    }
    if let Some(first) = m.first_byte {
        parts.push(format!("ttfb {} ms", first.as_millis()));
    }
    if let Some(reused) = m.reused_connection {
        parts.push(if reused { "reused" } else { "new connection" }.into());
    }
    if let Some(address) = &m.remote_address {
        parts.push(address.clone());
    }
    if let Some(tls) = &m.tls_version {
        parts.push(tls.clone());
    }
    if m.from_cache {
        parts.push("cache".into());
    }
    parts.join(" · ")
}

fn http_section() -> impl Piece {
    let status = Signal::new(crate::res::str::http_idle().format());
    // The callback idiom (docs/http.md): fetch_async completes on a background thread (the
    // sole browser thread on web); the captured Setter hops to the UI thread itself and no-ops
    // if the page is gone. Kept as the living Setter example; the rows below use the newer
    // await/Resource rails (docs/async.md).
    let done = status.setter();
    let patch_status = Signal::new(crate::res::str::http_idle().format());
    section((
        label(crate::res::str::http_caption()).font(Font::Footnote),
        crate::widgets::action_result(
            button(crate::res::str::http_fetch())
                .action(move || match local_url("/") {
                    Ok(url) => day_part_http::fetch_async(
                        day_part_http::Request::get(url)
                            .timeout(std::time::Duration::from_secs(10)),
                        move |result| done.set(status_line(result)),
                    ),
                    Err(e) => status.set(format!("error: {e}")),
                })
                .tint(crate::widgets::primary())
                .id("http-fetch")
                .any(),
            label(move || status.get()).id("http-status").any(),
        ),
        // PATCH through the same engine, await-style (docs/async.md): the echo body proves the
        // method crossed the platform stack (the historic Android HttpURLConnection gap).
        crate::widgets::action_result(
            button(crate::res::str::http_patch())
                .bordered()
                .action(move || match local_url("/") {
                    Ok(url) => {
                        day::task(async move {
                            let req = day_part_http::Request::patch(url, Vec::new())
                                .timeout(std::time::Duration::from_secs(10));
                            patch_status.set(status_line(day_part_http::fetch_future(req).await));
                        });
                    }
                    Err(e) => patch_status.set(format!("error: {e}")),
                })
                .id("http-patch")
                .any(),
            label(move || patch_status.get())
                .id("http-patch-status")
                .any(),
        ),
        http_resource_row(),
        labeled(
            crate::res::str::http_tier(),
            label(day_part_http::tier().label()).id("http-tier"),
        ),
        labeled(
            crate::res::str::http_caps(),
            label(capabilities_line())
                .font(Font::Footnote)
                .id("http-caps"),
        ),
        url_check_field(),
    ))
    .title(crate::res::str::http_title())
}

/// Declarative loading (docs/async.md): a `Resource` wraps "fetch the loopback URL". The
/// attempt counter makes Refetch observable (`ok 1:` → `ok 2:`), and the readout mirrors
/// `Load`'s three states.
fn http_resource_row() -> impl Piece {
    use day::reactive::{Load, Resource};
    let attempts = std::rc::Rc::new(std::cell::Cell::new(0u32));
    let res: Resource<String> = Resource::new(
        || (),
        move |_| {
            attempts.set(attempts.get() + 1);
            let n = attempts.get();
            async move {
                let url = local_url("/").map_err(day_part_http::HttpError::Io)?;
                let resp = day_part_http::fetch_future(
                    day_part_http::Request::get(url).timeout(std::time::Duration::from_secs(10)),
                )
                .await?;
                Ok::<_, day_part_http::HttpError>(format!(
                    "ok {n}: {} {}",
                    resp.status,
                    resp.text()
                ))
            }
        },
    );
    labeled(
        crate::res::str::http_res_label(),
        crate::widgets::action_result(
            button(crate::res::str::http_res_refetch())
                .bordered()
                .action(move || res.refetch())
                .id("http-res-refetch")
                .any(),
            label(move || {
                res.with(|l| match l {
                    Load::Loading => crate::res::str::http_checking().format(),
                    Load::Ready(s) => s.clone(),
                    Load::Failed(e) => format!("error: {e}"),
                })
            })
            .font(Font::Footnote)
            .id("http-res-status")
            .any(),
        ),
    )
}

/// The second half of the HTTP section: type any http(s) URL, tap Check, and read back the
/// response headers, the body size and the transfer metrics: a live view of what the platform
/// stack returns (and of platform policy: iOS ATS rejecting a cleartext host shows up here as
/// the error).
fn url_check_field() -> impl Piece {
    // Pre-filled with a host that answers cross-origin requests (httpbin echoes with
    // `Access-Control-Allow-Origin: *`), so Check works out of the box on web-dom too;
    // an arbitrary site would be blocked by CORS in a browser (docs/web.md).
    let url = Signal::new("https://httpbin.org/get".to_string());
    let out = Signal::new(String::new());
    // The in-flight check, if any: re-tapping Check aborts the previous task, which drops its
    // future and cancels the platform request (docs/async.md's drop-cancel rail): type a
    // slow URL, tap Check twice, and only the second answer ever lands.
    let inflight: std::rc::Rc<std::cell::Cell<Option<day::TaskHandle>>> =
        std::rc::Rc::new(std::cell::Cell::new(None));
    column((
        text_field(url)
            .placeholder(crate::res::str::http_url_placeholder())
            .id("http-url"),
        button(crate::res::str::http_check())
            .bordered()
            .action(move || {
                let target = url.get_untracked();
                if target.trim().is_empty() {
                    return;
                }
                let req = day_part_http::Request::get(target.trim())
                    .timeout(std::time::Duration::from_secs(15));
                out.set(crate::res::str::http_checking().format());
                if let Some(prev) = inflight.take() {
                    prev.abort();
                }
                let slot = inflight.clone();
                let handle = day::task(async move {
                    // Await-style (docs/async.md): the future resumes on the UI thread, so the
                    // readout is a plain Signal write with no Setter needed. A `Client` carries the
                    // transfer metrics the crate-root calls leave out.
                    let text = match day_part_http::Client::new().fetch_future(req).await {
                        // Raw readout (headers and sizes aren't locale material).
                        Ok(resp) => {
                            let mut s = format!("HTTP {} · {} bytes", resp.status, resp.body.len());
                            if let Some(line) = resp.metrics.as_ref().map(metrics_line)
                                && !line.is_empty()
                            {
                                s.push_str(&format!("\n{line}"));
                            }
                            for (k, v) in &resp.headers {
                                s.push_str(&format!("\n{k}: {v}"));
                            }
                            s
                        }
                        Err(e) => format!("error: {e}"),
                    };
                    out.set(text);
                    slot.set(None);
                });
                // A synchronously-failed fetch already finished (and cleared the slot) inside
                // task(); storing its handle then is a harmless stale-id miss.
                inflight.set(Some(handle));
            })
            .id("http-check"),
        label(move || out.get())
            .font(Font::Footnote)
            .id("http-headers"),
    ))
    .spacing(8.0)
    // Leading, like the section's other rows; the default centered alignment floated the
    // Check button and readout as islands mid-card on every platform.
    .align(HAlign::Leading)
}

/// 32 MiB, fed at 6 MiB a second so the bar has time to move and the buttons time to act.
const DOWNLOAD_BYTES: u64 = 32 << 20;
const DOWNLOAD_RATE: u64 = 6 << 20;

/// The download the Download manager section shows, for as long as the app runs.
static DOWNLOAD: std::sync::Mutex<Option<day_part_downloads::DownloadId>> =
    std::sync::Mutex::new(None);
/// The SHA-256 the finished file must have, as the test server computed it.
static DOWNLOAD_SHA256: std::sync::OnceLock<String> = std::sync::OnceLock::new();

fn current_download() -> Option<day_part_downloads::DownloadId> {
    DOWNLOAD.lock().ok().and_then(|id| *id)
}

/// The download manager behind the section, opened once beside the app's files
/// (docs/downloads.md). Its journal outlives the app, but the test server's port does not, so
/// whatever a previous run left behind is cleared on first use.
fn downloads() -> Result<&'static day_part_downloads::Downloads, String> {
    static DOWNLOADS: std::sync::OnceLock<Result<day_part_downloads::Downloads, String>> =
        std::sync::OnceLock::new();
    DOWNLOADS
        .get_or_init(|| {
            let manager = day_part_downloads::Downloads::open(download_dir()?.join("manager"))
                .map_err(|e| e.to_string())?;
            for stale in manager.list() {
                let _ = manager.remove(stale.id);
            }
            Ok(manager)
        })
        .as_ref()
        .map_err(Clone::clone)
}

/// Where the section's download and its manager live.
fn download_dir() -> Result<std::path::PathBuf, String> {
    #[cfg(target_arch = "wasm32")]
    {
        Err(crate::res::str::network_needs_server().format())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        day_part_fs::data_dir()
            .map(|dir| dir.join("showcase-downloads"))
            .map_err(|e| e.to_string())
    }
}

/// A download manager at work: a large file with a progress bar, paused, resumed from its
/// partial file with a validated range request, and verified against its SHA-256 at the end.
fn download_section() -> impl Piece {
    use day_part_downloads::{
        Download, DownloadError, DownloadId, Downloads, Progress, State, Tier,
    };

    let snapshot: Signal<Option<Progress>> = Signal::new(
        downloads()
            .ok()
            .zip(current_download())
            .and_then(|(manager, id)| manager.progress(id)),
    );
    let message = Signal::new(String::new());
    // Hand the next download to the OS (a background URLSession, DownloadManager) where it has a
    // service for that.
    let system = Signal::new(false);
    if let Ok(manager) = downloads() {
        // Progress arrives on the transport's thread; the setter carries it to the UI thread.
        let deliver = snapshot.setter();
        let watch = manager.watch(move |progress| {
            if current_download() == Some(progress.id) {
                deliver.set(Some(progress.clone()));
            }
        });
        // The watch stops when the page's scope drops it.
        day::reactive::Scope::current().on_cleanup(move || {
            let _stop = watch;
        });
        // The test server takes a moment to hash the file, so ask for the digest up front.
        static ASKED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if !ASKED.swap(true, std::sync::atomic::Ordering::Relaxed)
            && let Ok(url) = local_url(&format!("/bytes/{DOWNLOAD_BYTES}/sha256"))
        {
            let request =
                day_part_http::Request::get(url).timeout(std::time::Duration::from_secs(120));
            day_part_http::fetch_async(request, |result| {
                if let Ok(resp) = result
                    && resp.status == 200
                {
                    let _ = DOWNLOAD_SHA256.set(resp.text().trim().to_string());
                }
            });
        }
    }

    let start = move || {
        let manager = match downloads() {
            Ok(manager) => manager,
            Err(e) => return message.set(e),
        };
        if let Some(id) = current_download() {
            match manager.progress(id) {
                Some(progress) if !progress.state.is_settled() => return,
                Some(_) => {
                    let _ = manager.remove(id);
                }
                None => {}
            }
        }
        let (url, digest_url, dest) = match (
            local_url(&format!("/bytes/{DOWNLOAD_BYTES}?rate={DOWNLOAD_RATE}")),
            local_url(&format!("/bytes/{DOWNLOAD_BYTES}/sha256")),
            download_dir(),
        ) {
            (Ok(url), Ok(digest_url), Ok(dir)) => (url, digest_url, dir.join("sample.bin")),
            (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => return message.set(e),
        };
        message.set(crate::res::str::http_checking().format());
        day::task(async move {
            let expected = match DOWNLOAD_SHA256.get() {
                Some(digest) => digest.clone(),
                None => {
                    let request = day_part_http::Request::get(digest_url)
                        .timeout(std::time::Duration::from_secs(120));
                    match day_part_http::fetch_future(request).await {
                        Ok(resp) => resp.text().trim().to_string(),
                        Err(e) => return message.set(format!("error: {e}")),
                    }
                }
            };
            let download = Download::new(day_part_http::Request::get(url), dest)
                .expect_len(DOWNLOAD_BYTES)
                .expect_sha256(&expected)
                .in_background(system.get_untracked());
            match manager.enqueue(download) {
                Ok(id) => {
                    if let Ok(mut current) = DOWNLOAD.lock() {
                        *current = Some(id);
                    }
                    message.set(String::new());
                    snapshot.set(manager.progress(id));
                }
                Err(e) => message.set(format!("error: {e}")),
            }
        });
    };
    let act = move |op: fn(&Downloads, DownloadId) -> Result<(), DownloadError>| {
        if let (Ok(manager), Some(id)) = (downloads(), current_download())
            && let Err(e) = op(manager, id)
        {
            message.set(format!("error: {e}"));
        }
    };

    section((
        needs(!cfg!(target_arch = "wasm32")),
        progress(move || snapshot.with(|p| p.as_ref().and_then(|p| p.fraction()).unwrap_or(0.0)))
            .id("download-progress"),
        when(Downloads::system_tier, move || {
            labeled(
                crate::res::str::download_system(),
                toggle(system).id("download-system"),
            )
        }),
        row((
            button(crate::res::str::download_start())
                .tint(crate::widgets::primary())
                .action(start)
                .id("download-start"),
            button(crate::res::str::download_cancel())
                .tint(crate::widgets::danger())
                .action(move || act(Downloads::cancel))
                .id("download-cancel"),
        ))
        .spacing(8.0),
        row((
            button(crate::res::str::download_pause())
                .bordered()
                .action(move || act(Downloads::pause))
                .id("download-pause"),
            button(crate::res::str::download_resume())
                .bordered()
                .action(move || act(Downloads::resume))
                .id("download-resume"),
        ))
        .spacing(8.0),
        // The state is the manager's own word for it (`running`, `paused`, `done`): a value, not
        // prose, and what the scripts assert.
        label(move || {
            snapshot.with(|p| match p {
                Some(p) => p.state.label().to_string(),
                None => crate::res::str::http_idle().format(),
            })
        })
        .id("download-state"),
        label(move || {
            snapshot.with(|p| match p {
                Some(p) if p.tier == Tier::System => {
                    crate::res::str::download_tier_system().format()
                }
                Some(_) => crate::res::str::download_tier_app().format(),
                None => String::new(),
            })
        })
        .font(Font::Footnote)
        .id("download-tier"),
        label(move || {
            snapshot.with(|p| match p {
                Some(p) => crate::res::str::download_detail(
                    format!("{}/s", mib(p.bytes_per_second as u64)),
                    mib(p.received),
                    p.remaining
                        .map_or_else(|| "\u{2014}".to_string(), |r| format!("{} s", r.as_secs())),
                    mib(p.total.unwrap_or(DOWNLOAD_BYTES)),
                )
                .format(),
                None => String::new(),
            })
        })
        .font(Font::Footnote)
        .id("download-detail"),
        label(move || {
            snapshot.with(|p| match p {
                Some(p) if p.resumed_from > 0 => {
                    crate::res::str::download_resumed(mib(p.resumed_from)).format()
                }
                _ => String::new(),
            })
        })
        .font(Font::Footnote)
        .id("download-resumed"),
        label(move || {
            snapshot.with(|p| match p {
                Some(p) if p.state == State::Done && p.sha256.is_some() => {
                    crate::res::str::download_verified().format()
                }
                Some(p) => p
                    .error
                    .as_ref()
                    .map(|e| format!("error: {e}"))
                    .unwrap_or_default(),
                None => String::new(),
            })
        })
        .id("download-verified"),
        label(move || message.get())
            .font(Font::Footnote)
            .id("download-message"),
    ))
    .title(crate::res::str::download_title())
}

/// A slow response read chunk by chunk as the platform delivers it (docs/http.md "Streaming"):
/// under `day::task` each chunk lands on the UI thread, and the body pulls the next one only
/// when the loop asks.
fn stream_section() -> impl Piece {
    let bytes = Signal::new(crate::res::str::http_idle().format());
    let chunks = Signal::new(String::new());
    let inflight: std::rc::Rc<std::cell::Cell<Option<day::TaskHandle>>> = std::rc::Rc::default();
    section((
        needs(day_part_http::capabilities().streaming),
        crate::widgets::action_result(
            button(crate::res::str::stream_start())
                .bordered()
                .action(move || {
                    let url = match local_url("/drip?chunks=12&size=4096&delay_ms=150") {
                        Ok(url) => url,
                        Err(e) => return bytes.set(e),
                    };
                    if let Some(previous) = inflight.take() {
                        previous.abort();
                    }
                    bytes.set(crate::res::str::http_checking().format());
                    chunks.set(String::new());
                    let slot = inflight.clone();
                    let handle = day::task(async move {
                        let request = day_part_http::Request::get(url);
                        match day_part_http::Client::new().send_future(request).await {
                            Ok(streaming) => {
                                let mut body = streaming.into_body();
                                let (mut count, mut total) = (0u32, 0u64);
                                loop {
                                    match body.next().await {
                                        Some(Ok(chunk)) => {
                                            count += 1;
                                            total += chunk.len() as u64;
                                            chunks.set(format!("{count} chunks"));
                                            bytes.set(format!("{total} bytes…"));
                                        }
                                        Some(Err(e)) => break bytes.set(format!("error: {e}")),
                                        None => break bytes.set(format!("{total} bytes")),
                                    }
                                }
                            }
                            Err(e) => bytes.set(format!("error: {e}")),
                        }
                        slot.set(None);
                    });
                    inflight.set(Some(handle));
                })
                .id("stream-start")
                .any(),
            column((
                label(move || bytes.get()).id("stream-bytes"),
                label(move || chunks.get())
                    .font(Font::Footnote)
                    .id("stream-chunks"),
            ))
            .spacing(2.0)
            .align(HAlign::Leading)
            .any(),
        ),
    ))
    .title(crate::res::str::stream_title())
}

/// 8 MiB of the test server's pattern bytes.
#[cfg(not(target_arch = "wasm32"))]
const UPLOAD_BYTES: u64 = 8 << 20;
/// Their SHA-256, hashed once off the UI thread.
#[cfg(not(target_arch = "wasm32"))]
static UPLOAD_SHA256: std::sync::OnceLock<String> = std::sync::OnceLock::new();

/// The pattern bytes, generated while the upload reads them.
#[cfg(not(target_arch = "wasm32"))]
struct PatternReader {
    offset: u64,
    len: u64,
}

#[cfg(not(target_arch = "wasm32"))]
impl std::io::Read for PatternReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = (self.len - self.offset).min(buf.len() as u64) as usize;
        for (i, byte) in buf[..n].iter_mut().enumerate() {
            *byte = day_part_http::testing::pattern_byte(self.offset + i as u64);
        }
        self.offset += n as u64;
        Ok(n)
    }
}

/// Uploads (docs/http.md "Uploads"): a body streamed from a reader with a progress bar, checked by
/// the SHA-256 the server computes, and a multipart form.
fn upload_section() -> impl Piece {
    let fraction = Signal::new(0.0f64);
    let status = Signal::new(crate::res::str::http_idle().format());
    let digest = Signal::new(String::new());
    let form_status = Signal::new(String::new());

    #[cfg(not(target_arch = "wasm32"))]
    {
        static HASHING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if !HASHING.swap(true, std::sync::atomic::Ordering::Relaxed) {
            std::thread::spawn(|| {
                let _ = UPLOAD_SHA256.set(day_part_http::testing::pattern_sha256(UPLOAD_BYTES));
            });
        }
    }

    let upload = move || {
        let url = match local_url("/upload") {
            Ok(url) => url,
            Err(e) => return status.set(e),
        };
        fraction.set(0.0);
        digest.set(String::new());
        status.set(crate::res::str::http_checking().format());
        // The web has no test server to upload to, so `local_url` answered before this.
        #[cfg(target_arch = "wasm32")]
        let _ = url;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let sent = fraction.setter();
            let request = day_part_http::Request::post(url, Vec::new())
                .body_stream(
                    PatternReader {
                        offset: 0,
                        len: UPLOAD_BYTES,
                    },
                    Some(UPLOAD_BYTES),
                )
                .upload_progress(move |done, total| {
                    sent.set(done as f64 / total.unwrap_or(UPLOAD_BYTES).max(1) as f64);
                });
            day::task(async move {
                match day_part_http::Client::new().fetch_future(request).await {
                    Ok(resp) => {
                        let reply = resp.text().into_owned();
                        let (received, hash) = reply.split_once(' ').unwrap_or((&reply, ""));
                        status.set(format!("{received} bytes received"));
                        fraction.set(1.0);
                        digest.set(match UPLOAD_SHA256.get() {
                            Some(expected) if expected == hash => {
                                crate::res::str::upload_digest_match().format()
                            }
                            Some(_) => crate::res::str::upload_digest_mismatch().format(),
                            None => String::new(),
                        });
                    }
                    Err(e) => status.set(format!("error: {e}")),
                }
            });
        }
    };
    let upload_form = move || {
        let url = match local_url("/upload") {
            Ok(url) => url,
            Err(e) => return form_status.set(e),
        };
        form_status.set(crate::res::str::http_checking().format());
        let form = day_part_http::Form::new()
            .text("title", "Day Showcase")
            .bytes(
                "sample",
                "sample.bin",
                "application/octet-stream",
                vec![0x5A; 64 << 10],
            );
        let request = day_part_http::Request::post(url, Vec::new()).form(form);
        day::task(async move {
            let text = match day_part_http::Client::new().fetch_future(request).await {
                Ok(resp) => format!(
                    "{} bytes received",
                    resp.text().split(' ').next().unwrap_or("0")
                ),
                Err(e) => format!("error: {e}"),
            };
            form_status.set(text);
        });
    };

    section((
        needs(day_part_http::capabilities().upload_streaming),
        progress(fraction).id("upload-progress"),
        crate::widgets::action_result(
            button(crate::res::str::upload_stream())
                .bordered()
                .action(upload)
                .id("upload-stream")
                .any(),
            column((
                label(move || status.get()).id("upload-status"),
                label(move || digest.get())
                    .font(Font::Footnote)
                    .id("upload-digest"),
            ))
            .spacing(2.0)
            .align(HAlign::Leading)
            .any(),
        ),
        crate::widgets::action_result(
            button(crate::res::str::upload_form())
                .bordered()
                .action(upload_form)
                .id("upload-form")
                .any(),
            label(move || form_status.get())
                .id("upload-form-status")
                .any(),
        ),
    ))
    .title(crate::res::str::upload_title())
}

/// Redirects (docs/http.md "Redirects"): a handler sees each hop and follows it, or stops at
/// the first so the redirect response is the response.
fn redirect_section() -> impl Piece {
    let status = Signal::new(crate::res::str::http_idle().format());
    let hops = Signal::new(String::new());
    let run = move |stop_at_first: bool| {
        let url = match local_url("/redirect/3") {
            Ok(url) => url,
            Err(e) => return status.set(e),
        };
        status.set(crate::res::str::http_checking().format());
        hops.set(String::new());
        let record = hops.setter();
        let trail = std::sync::Mutex::new(Vec::<String>::new());
        let client = day_part_http::Client::builder()
            .on_redirect(move |hop, reply| {
                // The path alone: the test server's port changes with every run.
                let path = hop
                    .to
                    .splitn(4, '/')
                    .nth(3)
                    .map_or_else(|| hop.to.clone(), |p| format!("/{p}"));
                if let Ok(mut trail) = trail.lock() {
                    trail.push(path);
                    record.set(trail.join(" → "));
                }
                if stop_at_first {
                    reply.stop();
                } else {
                    reply.follow();
                }
            })
            .build();
        day::task(async move {
            let text = match client.fetch_future(day_part_http::Request::get(url)).await {
                Ok(resp) if (300..400).contains(&resp.status) => {
                    format!(
                        "{} {}",
                        resp.status,
                        resp.header("location").unwrap_or_default()
                    )
                }
                other => status_line(other),
            };
            status.set(text);
        });
    };
    section((
        needs(day_part_http::capabilities().manual_redirects),
        column((
            button(crate::res::str::redirect_follow())
                .bordered()
                .action(move || run(false))
                .id("redirect-follow"),
            button(crate::res::str::redirect_stop())
                .bordered()
                .action(move || run(true))
                .id("redirect-stop"),
        ))
        .spacing(8.0)
        .align(HAlign::Leading),
        label(move || status.get()).id("redirect-status"),
        label(move || hops.get())
            .font(Font::Footnote)
            .id("redirect-hops"),
    ))
    .title(crate::res::str::redirect_title())
}

/// Authentication (docs/http.md "Challenges"): the server challenges with Basic, Digest or
/// Bearer, and the page's handler answers with the fields' values.
fn auth_section() -> impl Piece {
    let user = Signal::new("day".to_string());
    let password = Signal::new("sunrise".to_string());
    let status = Signal::new(crate::res::str::http_idle().format());
    let run = move |path: &'static str| {
        let url = match local_url(path) {
            Ok(url) => url,
            Err(e) => return status.set(e),
        };
        let (name, secret) = (user.get_untracked(), password.get_untracked());
        status.set(crate::res::str::http_checking().format());
        let client = day_part_http::Client::builder()
            .cookies(day_part_http::Cookies::Off)
            .on_challenge(move |challenge, reply| match challenge.scheme {
                day_part_http::Scheme::Bearer => reply.bearer(secret.clone()),
                _ => reply.credential(name.clone(), secret.clone()),
            })
            .build();
        day::task(async move {
            status.set(status_line(
                client.fetch_future(day_part_http::Request::get(url)).await,
            ));
        });
    };
    section((
        labeled(
            crate::res::str::auth_user(),
            text_field(user).id("auth-user"),
        ),
        labeled(
            crate::res::str::auth_password(),
            text_field(password).id("auth-password"),
        ),
        row((
            button(crate::res::str::auth_basic())
                .bordered()
                .action(move || run("/basic-auth/day/sunrise"))
                .id("auth-basic"),
            button(crate::res::str::auth_digest())
                .bordered()
                .action(move || run("/digest-auth/day/sunrise"))
                .id("auth-digest"),
            button(crate::res::str::auth_bearer())
                .bordered()
                .action(move || run("/bearer/sunrise"))
                .id("auth-bearer"),
        ))
        .spacing(8.0),
        label(move || status.get()).id("auth-status"),
    ))
    .title(crate::res::str::auth_title())
}

/// The client the Cookies section shares, so a cookie one tap sets is there for the next.
fn cookie_client() -> &'static day_part_http::Client {
    static CLIENT: std::sync::OnceLock<day_part_http::Client> = std::sync::OnceLock::new();
    CLIENT.get_or_init(|| {
        day_part_http::Client::builder()
            .cache(day_part_http::Cache::Off)
            .build()
    })
}

/// Cookies (docs/http.md "Cookies"): a cookie set by a redirect response, sent on the next hop
/// and on later requests, then cleared.
fn cookie_section() -> impl Piece {
    let status = Signal::new(crate::res::str::http_idle().format());
    let send = move |path: &'static str| {
        let url = match local_url(path) {
            Ok(url) => url,
            Err(e) => return status.set(e),
        };
        day::task(async move {
            let text = match cookie_client()
                .fetch_future(day_part_http::Request::get(url))
                .await
            {
                Ok(resp) => resp.text().into_owned(),
                Err(e) => format!("error: {e}"),
            };
            status.set(text);
        });
    };
    let platform_store = cfg!(any(
        target_os = "macos",
        target_os = "ios",
        target_arch = "wasm32"
    )) && day_part_http::capabilities().platform_cookies;
    section((
        needs(!cfg!(target_arch = "wasm32")),
        label(if platform_store {
            crate::res::str::cookie_store_platform()
        } else {
            crate::res::str::cookie_store_jar()
        })
        .font(Font::Footnote),
        row((
            button(crate::res::str::cookie_set())
                .bordered()
                .action(move || send("/cookies/set?flavor=oat"))
                .id("cookie-set"),
            button(crate::res::str::cookie_send())
                .bordered()
                .action(move || send("/cookies"))
                .id("cookie-send"),
            button(crate::res::str::cookie_clear())
                .bordered()
                .action(move || {
                    cookie_client().clear_cookies();
                    send("/cookies");
                })
                .id("cookie-clear"),
        ))
        .spacing(8.0),
        label(move || status.get()).id("cookie-status"),
    ))
    .title(crate::res::str::cookie_title())
}

/// The client the Cache section shares, with the platform's cache.
fn cache_client() -> &'static day_part_http::Client {
    static CLIENT: std::sync::OnceLock<day_part_http::Client> = std::sync::OnceLock::new();
    CLIENT.get_or_init(|| {
        day_part_http::Client::builder()
            .cookies(day_part_http::Cookies::Off)
            .cache(day_part_http::Cache::platform())
            .build()
    })
}

/// The cache (docs/http.md "Caching"): the server says a response keeps for a minute and counts
/// the times it really answers, so a response from the cache repeats the count.
fn cache_section() -> impl Piece {
    let status = Signal::new(crate::res::str::http_idle().format());
    let fetch = move |policy: day_part_http::CachePolicy| {
        let url = match local_url("/cache/60") {
            Ok(url) => url,
            Err(e) => return status.set(e),
        };
        day::task(async move {
            let request = day_part_http::Request::get(url).cache(policy);
            let text = match cache_client().fetch_future(request).await {
                Ok(resp) => {
                    let from_cache = resp.metrics.as_ref().is_some_and(|m| m.from_cache);
                    format!(
                        "{} · {}",
                        resp.text(),
                        if from_cache { "cache" } else { "network" }
                    )
                }
                Err(e) => format!("error: {e}"),
            };
            status.set(text);
        });
    };
    section((
        needs(day_part_http::capabilities().platform_cache),
        row((
            button(crate::res::str::cache_fetch())
                .bordered()
                .action(move || fetch(day_part_http::CachePolicy::Default))
                .id("cache-fetch"),
            button(crate::res::str::cache_reload())
                .bordered()
                .action(move || fetch(day_part_http::CachePolicy::Reload))
                .id("cache-reload"),
            button(crate::res::str::cache_clear())
                .bordered()
                .action(move || {
                    cache_client().clear_cache();
                    status.set("\u{2014}".into());
                })
                .id("cache-clear"),
        ))
        .spacing(8.0),
        label(move || status.get()).id("cache-status"),
    ))
    .title(crate::res::str::cache_title())
}

/// A WebSocket (docs/http.md "WebSockets"): connect to the test server's echo, send a message,
/// ping, and close with a code, while a task reads what comes back.
fn websocket_section() -> impl Piece {
    use day_part_http::Message;

    let state = Signal::new(crate::res::str::http_idle().format());
    let last = Signal::new(String::new());
    let ping = Signal::new(String::new());
    let text = Signal::new("hello".to_string());
    let sender: std::rc::Rc<std::cell::RefCell<Option<day_part_http::WsSender>>> =
        std::rc::Rc::default();
    let reader: std::rc::Rc<std::cell::Cell<Option<day::TaskHandle>>> = std::rc::Rc::default();

    let connect = {
        let sender = sender.clone();
        move || {
            let url = match local_ws_url("/ws/echo") {
                Ok(url) => url,
                Err(e) => return state.set(e),
            };
            // A new connection replaces the old: aborting its reader drops the socket, which
            // closes it.
            if let Some(previous) = reader.take() {
                previous.abort();
            }
            sender.borrow_mut().take();
            state.set(crate::res::str::http_checking().format());
            last.set(String::new());
            let slot = sender.clone();
            let handle = day::task(async move {
                let request = day_part_http::Request::get(url).protocols(["day"]);
                match day_part_http::Client::new().websocket_future(request).await {
                    Ok(mut socket) => {
                        state.set(format!("open · {}", socket.protocol().unwrap_or_default()));
                        *slot.borrow_mut() = Some(socket.sender());
                        while let Some(item) = socket.next().await {
                            match item {
                                Ok(Message::Text(text)) => last.set(text),
                                Ok(Message::Binary(bytes)) => {
                                    last.set(format!("{} bytes", bytes.len()))
                                }
                                Ok(Message::Close { code, reason }) => {
                                    state.set(format!("closed · {code} {reason}"))
                                }
                                Err(e) => state.set(format!("error: {e}")),
                            }
                        }
                        slot.borrow_mut().take();
                    }
                    Err(e) => state.set(format!("error: {e}")),
                }
            });
            reader.set(Some(handle));
        }
    };
    let send = {
        let sender = sender.clone();
        move || {
            if let Some(socket) = sender.borrow().as_ref() {
                socket.send_async(Message::Text(text.get_untracked()), |_| {});
            }
        }
    };
    let send_ping = {
        let sender = sender.clone();
        move || {
            if let Some(socket) = sender.borrow().as_ref() {
                let done = ping.setter();
                socket.ping_async(move |result| {
                    done.set(match result {
                        Ok(()) => "pong".into(),
                        Err(e) => format!("error: {e}"),
                    })
                });
            }
        }
    };
    let close = move || {
        if let Some(socket) = sender.borrow().as_ref() {
            socket.close(4000, "done");
        }
    };

    section((
        needs(day_part_http::capabilities().websockets),
        row((
            button(crate::res::str::ws_connect())
                .bordered()
                .action(connect)
                .id("ws-connect"),
            button(crate::res::str::ws_close())
                .bordered()
                .action(close)
                .id("ws-close"),
        ))
        .spacing(8.0),
        text_field(text).id("ws-text"),
        row((
            button(crate::res::str::ws_send())
                .bordered()
                .action(send)
                .id("ws-send"),
            button(crate::res::str::ws_ping())
                .bordered()
                .action(send_ping)
                .id("ws-ping"),
        ))
        .spacing(8.0),
        label(move || state.get()).id("ws-state"),
        label(move || last.get()).id("ws-last"),
        label(move || ping.get())
            .font(Font::Footnote)
            .id("ws-ping-status"),
    ))
    .title(crate::res::str::ws_title())
}

/// A total-time limit (docs/http.md "Time limits"): a response the server holds for three seconds
/// against a client that allows one.
fn timeout_section() -> impl Piece {
    let status = Signal::new(crate::res::str::http_idle().format());
    section((crate::widgets::action_result(
        button(crate::res::str::timeout_run())
            .bordered()
            .action(move || {
                let url = match local_url("/delay/3000") {
                    Ok(url) => url,
                    Err(e) => return status.set(e),
                };
                status.set(crate::res::str::http_checking().format());
                day::task(async move {
                    let client = day_part_http::Client::builder()
                        .timeout_total(std::time::Duration::from_secs(1))
                        .build();
                    status.set(status_line(
                        client.fetch_future(day_part_http::Request::get(url)).await,
                    ));
                });
            })
            .id("timeout-run")
            .any(),
        label(move || status.get()).id("timeout-status").any(),
    ),))
    .title(crate::res::str::timeout_title())
}

/// Server trust and client certificates (docs/http.md "Trust"): a self-signed server refused and
/// then trusted by a handler, public-key pins learned, honored and violated, and a server that
/// asks for a client certificate. badssl.com publishes these servers, and the certificate, for
/// exactly this kind of test.
fn trust_section() -> impl Piece {
    use day_part_http::{Client, Identity, Request, Trust};

    let caps = day_part_http::capabilities();
    let visit = Signal::new(crate::res::str::http_idle().format());
    let pins = Signal::new(String::new());
    let identity = Signal::new(crate::res::str::http_idle().format());

    let self_signed = move |trust_anyway: bool| {
        visit.set(crate::res::str::http_checking().format());
        let mut builder = Client::builder();
        if trust_anyway {
            builder = builder.on_server_trust(|trust, reply| {
                if trust.host == "self-signed.badssl.com" {
                    reply.accept();
                } else {
                    reply.default_handling();
                }
            });
        }
        let client = builder.build();
        day::task(async move {
            let request = Request::get("https://self-signed.badssl.com/");
            visit.set(status_only(client.fetch_future(request).await));
        });
    };
    let check_pins = move || {
        pins.set(crate::res::str::http_checking().format());
        day::task(async move {
            // Learn the key example.com presents, pin it, then pin a key it does not present.
            let leaf = std::sync::Arc::new(std::sync::Mutex::new(None::<String>));
            let seen = leaf.clone();
            let learner = Client::builder()
                .on_server_trust(move |trust, reply| {
                    if let Ok(mut leaf) = seen.lock() {
                        *leaf = trust.pins().into_iter().next();
                    }
                    reply.default_handling();
                })
                .build();
            if let Err(e) = learner
                .fetch_future(Request::get("https://example.com/"))
                .await
            {
                return pins.set(format!("error: {e}"));
            }
            let Some(pin) = leaf.lock().ok().and_then(|leaf| leaf.clone()) else {
                return pins.set("error: no certificate chain".into());
            };
            let pinned = Client::builder()
                .trust(Trust::system().pin("example.com", &pin))
                .build();
            let good = status_only(
                pinned
                    .fetch_future(Request::get("https://example.com/"))
                    .await,
            );
            let wrong = Client::builder()
                .trust(Trust::system().pin(
                    "example.com",
                    "sha256/AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
                ))
                .build();
            let bad = status_only(
                wrong
                    .fetch_future(Request::get("https://example.com/"))
                    .await,
            );
            pins.set(format!("{pin}\npinned: {good}\nwrong pin: {bad}"));
        });
    };
    let connect_identity = move |with_certificate: bool| {
        identity.set(crate::res::str::http_checking().format());
        day::task(async move {
            let mut builder = Client::builder();
            if with_certificate {
                let archive = Request::get("https://badssl.com/certs/badssl.com-client.p12");
                match day_part_http::fetch_future(archive).await {
                    Ok(resp) if resp.status == 200 => {
                        builder = builder.identity(Identity::pkcs12(resp.body, "badssl.com"));
                    }
                    other => return identity.set(status_only(other)),
                }
            }
            let request = Request::get("https://client.badssl.com/");
            identity.set(status_only(builder.build().fetch_future(request).await));
        });
    };

    section((
        needs(caps.server_trust),
        label(crate::res::str::network_needs_internet()).font(Font::Footnote),
        column((
            button(crate::res::str::trust_visit())
                .bordered()
                .action(move || self_signed(false))
                .id("trust-visit"),
            button(crate::res::str::trust_accept())
                .bordered()
                .action(move || self_signed(true))
                .id("trust-accept"),
        ))
        .spacing(8.0)
        .align(HAlign::Leading),
        label(move || visit.get()).id("trust-status"),
        button(crate::res::str::trust_pins())
            .bordered()
            .action(check_pins)
            .id("trust-pins"),
        label(move || pins.get())
            .font(Font::Footnote)
            .id("trust-pin-status"),
        column((
            button(crate::res::str::identity_without())
                .bordered()
                .action(move || connect_identity(false))
                .id("identity-without"),
            button(crate::res::str::identity_with())
                .bordered()
                .action(move || connect_identity(true))
                .id("identity-with"),
        ))
        .spacing(8.0)
        .align(HAlign::Leading),
        label(move || identity.get()).id("identity-status"),
    ))
    .title(crate::res::str::trust_title())
}

/// App-local file storage (docs/fs.md): day-part-fs write/read/list/remove through the async
/// futures, so the same code runs on every target: real files natively, OPFS in the browser.
/// Statuses are raw (walkthrough-asserted, identical across locales).
fn storage_section() -> impl Piece {
    const FILE: &str = "demo/showcase-note.txt";
    let note = Signal::new(String::new());
    let status = Signal::new(crate::res::str::storage_idle().format());
    let files = Signal::new("\u{2014}".to_string());

    // Re-list the demo directory after every operation ("\u{2014}" = nothing stored).
    let refresh = move || {
        day::task(async move {
            match day_part_fs::list_future("demo").await {
                Ok(names) if names.is_empty() => files.set("\u{2014}".into()),
                Ok(names) => files.set(names.join(", ")),
                Err(e) => files.set(format!("error: {e}")),
            }
        });
    };
    refresh();

    section((
        label(crate::res::str::storage_caption()).font(Font::Footnote),
        text_field(note)
            .placeholder(crate::res::str::storage_placeholder())
            .id("fs-note"),
        row((
            button(crate::res::str::storage_save())
                .action(move || {
                    let data = note.get_untracked().into_bytes();
                    day::task(async move {
                        let text = match day_part_fs::write_future(FILE, data).await {
                            Ok(()) => "saved".to_string(),
                            Err(e) => format!("error: {e}"),
                        };
                        status.set(text);
                        refresh();
                    });
                })
                .tint(crate::widgets::primary())
                .id("fs-save"),
            button(crate::res::str::storage_load())
                .bordered()
                .action(move || {
                    day::task(async move {
                        let text = match day_part_fs::read_future(FILE).await {
                            Ok(bytes) => {
                                format!("loaded:{}", String::from_utf8_lossy(&bytes))
                            }
                            Err(e) => format!("error: {e}"),
                        };
                        status.set(text);
                    });
                })
                .id("fs-load"),
            button(crate::res::str::storage_delete())
                .action(move || {
                    day::task(async move {
                        let text = match day_part_fs::remove_future(FILE).await {
                            Ok(()) => "deleted".to_string(),
                            Err(e) => format!("error: {e}"),
                        };
                        status.set(text);
                        refresh();
                    });
                })
                .tint(crate::widgets::danger())
                .id("fs-delete"),
            label(move || status.get()).id("fs-status"),
        ))
        .spacing(8.0),
        labeled(
            crate::res::str::storage_files_label(),
            label(move || files.get())
                .font(Font::Footnote)
                .id("fs-list"),
        ),
    ))
    .title(crate::res::str::storage_title())
}

/// Connectivity (docs/network.md): online, metered, and the interface kind, read on demand.
fn network_section() -> impl Piece {
    let reading = Signal::new(network_line().format());
    section((row((
        button(crate::res::str::network_refresh())
            .bordered()
            .action(move || reading.set(network_line().format()))
            .id("network-refresh"),
        label(move || reading.get()).id("network-reading"),
    ))
    .spacing(8.0),))
    .title(crate::res::str::network_status_section())
}

/// The current connectivity snapshot as a localized line (Fluent; kind stays the API's enum
/// debug form; it is a value, not prose).
fn network_line() -> LocalizedText {
    match day_part_network::status() {
        Some(n) => {
            if n.online {
                crate::res::str::network_reading_online(
                    match n.expensive {
                        Some(true) => "yes",
                        Some(false) => "no",
                        None => "?",
                    },
                    format!("{:?}", n.kind),
                )
            } else {
                crate::res::str::network_reading_offline()
            }
        }
        None => crate::res::str::network_reading_none(),
    }
}
