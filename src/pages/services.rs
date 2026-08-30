use day::prelude::*;
use day_part_haptics::Haptic;
use day_part_local_notify::{Channel, Importance, Notification, Trigger};

use crate::widgets::page;

/// Platform services (docs/http.md, docs/clipboard.md, docs/prefs.md, docs/haptics.md,
/// docs/files.md, docs/notify.md, docs/bridge.md): the headless "do something with the OS" parts,
/// one grouped form section each — text to speech (first: it is the daybridge reference, and the
/// one demo you can hear), an HTTP fetch, clipboard round-trip, persisted preferences, haptic
/// feedback, local notifications, and the native file pickers.
pub(crate) fn services_page() -> AnyPiece {
    page(
        crate::res::str::nav_services(),
        "services-title",
        Some(crate::res::str::services_caption()),
        form((
            speech_section(),
            http_section(),
            clipboard_section(),
            prefs_section(),
            haptics_section(),
            notify_section(),
            badge_section(),
            files_section(),
            storage_section(),
        ))
        .any(),
    )
    .any()
}

/// Text to speech (docs/bridge.md): day-part-speech is daybridge's reference part — one Rust API
/// whose implementation is Swift on Apple, Java on Android, ArkTS on HarmonyOS, JavaScript on the
/// web, C++ (SAPI) on Windows, and C on Linux, all declared in one file. This section is deliberately the simplest thing
/// that proves a bridged call ran: type a line (or take the placeholder's) and the device says it.
///
/// `available()` reports what THIS target's arm promises, so the label is the honest answer on a
/// target with no arm (Unsupported) or a partial one (HarmonyOS, whose voices are zh-CN only).
///
/// There is no progress readout. A v1 bridge call is synchronous and one-shot, so nothing tells the
/// app when the engine stopped talking (docs/bridge.md "After v1") — a "Speaking…" label would sit
/// there forever and read as a hang. The voice is the feedback.
fn speech_section() -> impl Piece {
    // Empty means "say the localized sample", which is exactly what the placeholder shows.
    let phrase = Signal::new(String::new());
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
                    // An `Unsupported` here is the fallback arm answering, which the support label
                    // above already said; nothing to report that the user was not told.
                    let _ = day_part_speech::speak(&text);
                })
                .tint(crate::widgets::primary())
                .id("speech-speak"),
            button(crate::res::str::speech_stop())
                .bordered()
                .action(day_part_speech::stop)
                .id("speech-stop"),
        ))
        .spacing(8.0),
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
        // `.grow_w()` is what makes the grid column FLEXIBLE. With every cell flexible the layout
        // splits the leftover width evenly between the columns (docs/grid.md §3), so the buttons
        // come out identical whatever the label length or screen width — the reason a `row` was
        // wrong here: it sized each button to its own text and ran off the edge.
        .grow_w()
}

/// One step of a haptic "song": wait `delay_ms`, then fire `haptic`.
type Beat = (u32, Haptic);

/// Play a timed sequence on `day::task` (docs/async.md), which polls on the UI thread — where
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
// TEMPO GRID. The beats are written against 120 BPM rather than in ad-hoc milliseconds, because
// that is what separates a rhythm from a list of buzzes: repetition on a grid is what the ear —
// and the hand — hears as musical. Accelerandos deliberately leave the grid, which is why those
// runs carry explicit millisecond gaps.
//
// DYNAMIC RANGE, and what actually varies per platform. iOS maps the seven styles onto three
// impact intensities plus three multi-tap notification patterns, so all seven feel distinct.
// Android collapses them: Light/Selection are both EFFECT_TICK, Heavy AND Warning are both
// EFFECT_HEAVY_CLICK, Success AND Error are both EFFECT_DOUBLE_CLICK. So the honest palette these
// songs compose against is four sensations — tick (quietest), click, heavy click (loudest single
// hit), and double click (the accent) — and the contrast is built from Selection/Light against
// Heavy, with Error reserved for phrase-ending crashes. Leaning on Warning-vs-Heavy would have
// sounded like a difference on iPhone and like nothing at all on a Pixel.
//
// A notification-style haptic (Success/Error) plays its own multi-tap pattern over ~150-300 ms, so
// nothing is scheduled tight behind one — it would collide rather than syncopate.

/// Quarter note at 120 BPM.
const Q: u32 = 500;
/// Eighth.
const E: u32 = 250;
/// Sixteenth.
const S: u32 = 125;
/// Thirty-second — around the floor where the engine still resolves separate taps rather than
/// smearing them into one buzz.
const T: u32 = 63;

/// 5.6 s. The Duolingo shape: a pickup that rises into a downbeat, a two-bar phrase answered by a
/// denser repeat, a crash, and a resolve. Maximum contrast — near-silent Selection ticks a beat
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
    // Panic — both halves of the beat at full force.
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
/// at the bottom, then rebuild — decelerating as they intensify — into a crash.
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
    // Climb: the fall run backwards — slowing down as it gets heavier.
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
    // Second fall, heavier and shorter — the pattern the ear now expects, delivered harder.
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
        // The songs get filled colors so they read as a different KIND of control from the single
        // taps above. Each fill picks its own label color: the three saturated ones take white,
        // and AMBER takes `tinted_pale`, which swaps in INK text — white on a pale fill is the
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
    // Prime both from the AUTHORITATIVE status. Notifications are the one Apple permission with no
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
        // (docs/permissions.md). `Gate::Absent`/`Ungated` mean no consent record exists — desktop
        // Linux and Windows have no database to ask — so a Request button there would be a control
        // that provably does nothing, and none is shown. Where the OS does prompt, the affordance
        // still changes: once the answer is final, `request` no longer puts a dialog on screen and
        // Settings is the only remedy, which is why `can_prompt` picks the label and the action.
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
                                    // !Send — a Setter is the sanctioned cross-thread door
                                    // (DESIGN §3.3).
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

    // Three states, not two: `Emulated` means the call is made and the shell may ignore it — the
    // web unless installed, and desktop Linux under a shell that skips the Unity protocol.
    let caps_line = match can_count {
        Support::Native => crate::res::str::badge_caps_native(),
        Support::Emulated => crate::res::str::badge_caps_emulated(),
        Support::Unsupported => crate::res::str::badge_caps_none(),
    };

    section((
        crate::widgets::support_note(crate::support::cap(Cap::AppBadgeCount)),
        label(caps_line).font(Font::Footnote).id("badge-supported"),
        // Named so a user on Android reads WHY rather than assuming it is broken.
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
            // Text is macOS-only, so the control simply is not offered elsewhere rather than
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

/// The demo's target URL. Native targets spin the one-shot loopback server below; the web
/// (web-dom) instead fetches the same-origin `day-http-ok` path — a browser tab can host no
/// TCP listener, and `day launch`'s dev server answers that path with the identical bodies
/// (crates/day-cli/src/web.rs), so the walkthrough asserts the same bytes everywhere. On a
/// static host without the endpoint the buttons report the server's honest error instead.
fn demo_url() -> Result<String, String> {
    #[cfg(target_arch = "wasm32")]
    {
        // Relative on purpose: resolves against the page origin (and subpath, e.g. the
        // project-Pages /Day-Showcase/), keeping the request same-origin — no CORS.
        Ok("day-http-ok".into())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        serve_once().map_err(|e| e.to_string())
    }
}

/// One-shot loopback server answering `200` — the demo needs no external network, so it behaves
/// the same in airplane mode, on CI, and behind a proxy. GET keeps the historic `day-http-ok`
/// body (walkthrough-asserted, byte-identical); any other method echoes it as
/// `day-http-ok:<METHOD>` — the deterministic proof that e.g. PATCH crossed the platform engine.
#[cfg(not(target_arch = "wasm32"))]
fn serve_once() -> std::io::Result<String> {
    use std::io::{Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    std::thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0u8; 2048];
            let n = stream.read(&mut buf).unwrap_or(0);
            let head = String::from_utf8_lossy(&buf[..n]);
            let method = head.split_whitespace().next().unwrap_or("GET").to_string();
            let body = if method == "GET" {
                "day-http-ok".to_string()
            } else {
                format!("day-http-ok:{method}")
            };
            let _ = stream.write_all(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                )
                .as_bytes(),
            );
        }
    });
    Ok(format!("http://127.0.0.1:{port}/"))
}

fn http_section() -> impl Piece {
    let status = Signal::new(crate::res::str::http_idle().format());
    // The callback idiom (docs/http.md): fetch_async completes on a BACKGROUND thread (the
    // sole browser thread on web); the
    // captured Setter hops to the UI thread itself and no-ops if the page is gone. Kept as the
    // living Setter example — the rows below use the newer await/Resource rails (docs/async.md).
    let done = status.setter();
    let patch_status = Signal::new(crate::res::str::http_idle().format());
    section((
        label(crate::res::str::http_caption()).font(Font::Footnote),
        crate::widgets::action_result(
            button(crate::res::str::http_fetch())
                .action(move || match demo_url() {
                    Ok(url) => day_part_http::fetch_async(
                        day_part_http::Request::get(url)
                            .timeout(std::time::Duration::from_secs(10)),
                        move |result| {
                            // Raw "<status> <body>" on purpose: identical in every locale, so
                            // the walkthrough can assert it exactly.
                            let text = match result {
                                Ok(resp) => format!("{} {}", resp.status, resp.text()),
                                Err(e) => format!("error: {e}"),
                            };
                            done.set(text);
                        },
                    ),
                    Err(e) => status.set(format!("error: {e}")),
                })
                .tint(crate::widgets::primary())
                .id("http-fetch")
                .any(),
            label(move || status.get()).id("http-status").any(),
        ),
        // PATCH through the same engine, await-style (docs/async.md): the echo body proves the
        // method crossed the platform stack — the historic Android HttpURLConnection gap.
        crate::widgets::action_result(
            button(crate::res::str::http_patch())
                .bordered()
                .action(move || match demo_url() {
                    Ok(url) => {
                        day::task(async move {
                            let req = day_part_http::Request::patch(url, Vec::new())
                                .timeout(std::time::Duration::from_secs(10));
                            let text = match day_part_http::fetch_future(req).await {
                                Ok(resp) => format!("{} {}", resp.status, resp.text()),
                                Err(e) => format!("error: {e}"),
                            };
                            patch_status.set(text);
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
                let url = demo_url().map_err(day_part_http::HttpError::Io)?;
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
/// response headers plus the body size — a live view of what the platform stack returns
/// (and of platform policy: iOS ATS rejecting a cleartext host shows up here as the error).
fn url_check_field() -> impl Piece {
    // Pre-filled with a host that answers cross-origin requests (httpbin echoes with
    // `Access-Control-Allow-Origin: *`), so Check works out of the box on web-dom too —
    // an arbitrary site would be blocked by CORS in a browser (docs/web.md).
    let url = Signal::new("https://httpbin.org/get".to_string());
    let out = Signal::new(String::new());
    // The in-flight check, if any: re-tapping Check aborts the previous task, which drops its
    // FetchFuture and CANCELS the platform request (docs/async.md's drop-cancel rail) — type a
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
                    // readout is a plain Signal write — no Setter needed.
                    let text = match day_part_http::fetch_future(req).await {
                        // Raw readout on purpose (headers and sizes aren't locale material).
                        Ok(resp) => {
                            let mut s = format!("HTTP {} · {} bytes", resp.status, resp.body.len());
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
    // Leading, like the section's other rows — the default centered alignment floated the
    // Check button and readout as islands mid-card on every platform.
    .align(HAlign::Leading)
}

/// App-local file storage (docs/fs.md): day-part-fs write/read/list/remove through the async
/// futures, so the SAME code runs on every target — real files natively, OPFS in the browser.
/// Statuses are raw on purpose (walkthrough-asserted, identical across locales).
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
