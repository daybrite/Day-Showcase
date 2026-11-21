//! The Media page: a native video player (day-piece-media, an external piece) with every part
//! of its transport on Day's controls.
//!
//! The player is AVPlayerView, AVPlayerViewController, QMediaPlayer with QVideoWidget,
//! android.widget.VideoView, GtkVideo, MediaPlayerElement, `<video>` or an ArkTS Video. Commands
//! go in through `Trigger`s and bindings (`.play`, `.pause`, `.load`, `.seek`, `.rate`,
//! `.volume`); what the player does comes back through `.state`, `.timeline`, `.error` and
//! `.metadata`, and the page draws its scrubber, its buffered bar and its readouts from those.
//!
//! The mute switch is `.muted_while`, which silences the track and leaves the volume where it
//! is. While the Now playing switch is on, `.now_playing` tells the system what is playing (the
//! lock screen, Control Center and the media keys' overlay, MPRIS, a browser's media controls),
//! and `.on_remote` runs the commands those surfaces send back through the same triggers as the
//! page's buttons.

use day::prelude::*;
use day_piece_media::{
    MediaError, MediaErrorKind, MediaTimeline, NowPlayingInfo, PlaybackState, RemoteCommand,
    StreamMetadata, media,
};

use crate::gallery::{self, Tone, sky};
use crate::res::str as tr;
use crate::widgets::page_cards;

/// The sample: Hurricane Milton over the Gulf of Mexico on 9 October 2024, filmed by the
/// external cameras of the International Space Station, with the station's solar array and
/// robotic arm in front. It is five and a half minutes of 480×270 H.264 with a near-silent AAC
/// track, from NASA's Image and Video Library (images.nasa.gov), whose media is in the public
/// domain; resource/assets/nasa/README.md records it.
const CLIP: &str = "jsc2024m000174_Cameras_On_The_International_Space_Station_Capture_New_Views_Of_Hurricane_Milton-241009";

/// The address of the sample in the library's 480-pixel `~preview` size: a 26 MB stream that
/// decodes in a fraction of the memory the 640-pixel `~small` one takes, which a phone-sized
/// player cannot tell apart.
fn clip_url() -> String {
    format!("https://images-assets.nasa.gov/video/{CLIP}/{CLIP}~preview.mp4")
}

/// The library's still of the sample, for a now-playing surface that fetches its artwork from
/// an address.
fn still_url() -> String {
    format!("https://images-assets.nasa.gov/video/{CLIP}/{CLIP}~small.jpg")
}

/// What the player opens: the stream, or in fixture mode six seconds of it from the bundle.
fn clip_source() -> String {
    if sky::fixtures()
        && let Some(path) = bundled(crate::res::assets::nasa::milton_mp4)
    {
        return path;
    }
    clip_url()
}

/// A bundled file where a player or the system can open it: on the web, its address beside the
/// page; elsewhere a copy in the app's cache directory, written when it is missing, because an
/// Android or HarmonyOS bundle and a GTK or Qt resource are not files a player can be handed.
fn bundled(asset: day::AssetName) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        Some(format!("assets/data/{}", asset.as_str()))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = day::storage::cache_dir()?.join(asset.as_str());
        let bytes = day::resource(asset)?;
        let current = std::fs::metadata(&path).is_ok_and(|file| file.len() == bytes.len() as u64);
        if !current {
            std::fs::create_dir_all(path.parent()?).ok()?;
            std::fs::write(&path, bytes.as_slice()).ok()?;
        }
        Some(path.to_string_lossy().into_owned())
    }
}

/// The artwork the now-playing surface shows: a still of the sample. The Apple platforms take a
/// file, the web and HarmonyOS an address (docs/media.md of the piece).
fn artwork() -> (Option<String>, Option<String>) {
    let file = bundled(crate::res::assets::nasa::artwork_jpg);
    if cfg!(target_arch = "wasm32") {
        (None, file)
    } else {
        (file, Some(still_url()))
    }
}

/// What the system's now-playing surface shows for the sample.
fn now_playing_info(
    playing: bool,
    timeline: &MediaTimeline,
    artwork: &(Option<String>, Option<String>),
) -> NowPlayingInfo {
    NowPlayingInfo {
        title: tr::media_clip_title().format(),
        artist: tr::media_clip_credit().format(),
        album: String::new(),
        artwork_path: artwork.0.clone(),
        artwork_url: artwork.1.clone(),
        duration: timeline.duration,
        position: timeline.position,
        rate: timeline.rate,
        playing,
        live: false,
        has_next: false,
        has_previous: false,
    }
}

/// How far the position may move between two timeline reports before it counts as a jump (a
/// seek, or the loop starting over), which the now-playing surface is told about. Reports come
/// four times a second, so playback at 2× moves half a second between them.
const JUMP_SECONDS: f64 = 2.0;

/// The `media_remote` selector for a remote command.
fn remote_key(command: Option<RemoteCommand>) -> &'static str {
    match command {
        None => "none",
        Some(RemoteCommand::Play) => "play",
        Some(RemoteCommand::Pause) => "pause",
        Some(RemoteCommand::Toggle) => "toggle",
        Some(RemoteCommand::Stop) => "stop",
        Some(RemoteCommand::Next) => "next",
        Some(RemoteCommand::Previous) => "previous",
        Some(RemoteCommand::Seek(_)) => "seek",
        Some(RemoteCommand::SkipForward(_)) => "forward",
        Some(RemoteCommand::SkipBackward(_)) => "backward",
    }
}

/// The speeds the picker offers. Every engine with a rate accepts these four (ArkUI takes
/// 0.75, 1, 1.25, 1.75 and 2 only).
const RATES: [f64; 4] = [0.75, 1.0, 1.25, 2.0];

/// How many timeline reports the scrubber ignores after the user moves it, about two seconds at
/// the piece's four reports a second. A seek lands asynchronously, and the reports in between
/// still carry the old position, which would pull the thumb back under the pointer.
const SCRUB_HOLD: u32 = 8;

/// The volume the page opens at. The page opens muted, which is also what lets a browser start
/// the video on its own, so the walkthrough's captures on CI runners stay silent.
const START_VOLUME: f64 = 0.6;

/// Whether the player has mute only: ArkUI's video has no volume, so its slider is inert.
fn mute_only() -> bool {
    cfg!(feature = "arkui")
}

/// Whether this toolkit's player reports buffered ranges (docs/media.md of the piece): GTK, Qt,
/// WinUI and ArkUI have no observation to report.
fn reports_buffered() -> bool {
    !cfg!(any(
        feature = "gtk",
        feature = "qt",
        feature = "xaml",
        feature = "winui",
        feature = "arkui"
    ))
}

/// Seconds as `m:ss`.
fn clock(seconds: f64) -> String {
    let whole = seconds.max(0.0).round() as u64;
    format!("{}:{:02}", whole / 60, whole % 60)
}

/// A speed as a readout: `1×`, `1.25×`.
fn times(rate: f64) -> String {
    format!("{rate}×")
}

/// The `media_state` selector for a state.
fn state_key(state: &PlaybackState) -> &'static str {
    match state {
        PlaybackState::Idle => "idle",
        PlaybackState::Loading => "loading",
        PlaybackState::Playing => "playing",
        PlaybackState::Paused => "paused",
        PlaybackState::Ended => "ended",
        PlaybackState::Error(_) => "error",
    }
}

/// The tone a state's badge takes.
fn state_tone(state: &PlaybackState) -> Option<Tone> {
    match state {
        PlaybackState::Playing => Some(Tone::Up),
        PlaybackState::Loading => Some(Tone::Warn),
        PlaybackState::Error(_) => Some(Tone::Danger),
        PlaybackState::Idle | PlaybackState::Paused | PlaybackState::Ended => None,
    }
}

/// An error as a line of the app's own words. The engine's diagnostic text stays out of the
/// page; `operation` is the piece's identifier for the command (`seek`, `rate`).
fn error_text(error: &MediaError) -> String {
    let kind = match error.kind {
        MediaErrorKind::InvalidInput => "invalid",
        MediaErrorKind::Unsupported => "unsupported",
        MediaErrorKind::Playback => "playback",
        MediaErrorKind::Command => "command",
    };
    tr::media_error(tr::MediaError {
        kind,
        operation: error.operation.clone(),
    })
    .format()
}

/// The slider's side of the timeline: a binding whose value is the position as a fraction of the
/// duration.
///
/// Reading it gives where the thumb sits. The player's reports move it while the user leaves it
/// alone; a drag moves it at once and asks the player to follow. A settled value seeks
/// immediately. A value still being dragged seeks on the next timeline report, so a drag sends
/// four seeks a second at most, and a toolkit whose slider never reports the end of a drag
/// still seeks.
#[derive(Clone, Copy)]
struct Scrubber {
    /// Where the thumb sits, `0.0..=1.0`.
    fraction: Signal<f64>,
    /// Timeline reports still to ignore after the user's last move.
    hold: Signal<u32>,
    /// Whether a dragged value waits for the next report to be sought.
    pending: Signal<bool>,
    /// What the player reports.
    timeline: Signal<MediaTimeline>,
    /// The seconds `.seek` reads when `seek` fires.
    target: Signal<f64>,
    seek: Trigger,
}

impl Scrubber {
    fn new(timeline: Signal<MediaTimeline>, target: Signal<f64>, seek: Trigger) -> Self {
        let scrubber = Scrubber {
            fraction: Signal::new(0.0),
            hold: Signal::new(0),
            pending: Signal::new(false),
            timeline,
            target,
            seek,
        };
        // Each report: send the drag's waiting seek, then either count the hold down or move
        // the thumb to the reported position.
        Effect::new(move || {
            let report = scrubber.timeline.get();
            if scrubber.pending.get_untracked() {
                scrubber.pending.set(false);
                scrubber.seek_to_thumb();
            }
            let reported = match report.duration {
                Some(duration) if duration > 0.0 => (report.position / duration).clamp(0.0, 1.0),
                _ => 0.0,
            };
            let hold = scrubber.hold.get_untracked();
            // The seek has landed once the player reports the thumb's own position.
            let landed = (reported - scrubber.fraction.get_untracked()).abs() < 0.02;
            if hold > 0 && !landed {
                scrubber.hold.set(hold - 1);
            } else {
                scrubber.hold.set_if_changed(0);
                scrubber.fraction.set_if_changed(reported);
            }
        });
        scrubber
    }

    /// Asks the player for the thumb's position, clamped into what the source can seek to.
    fn seek_to_thumb(self) {
        let timeline = self.timeline.get_untracked();
        let Some(duration) = timeline.duration else {
            return;
        };
        let wanted = self.fraction.get_untracked() * duration;
        if let Some(seconds) = timeline.seek_target(wanted) {
            self.target.set(seconds);
            self.seek.notify();
        }
    }

    /// The thumb moved under the user's hand.
    fn moved(self, fraction: f64) {
        self.fraction.set(fraction.clamp(0.0, 1.0));
        self.hold.set(SCRUB_HOLD);
    }

    /// Moves the thumb to `seconds` and asks the player to follow, as a remote command does.
    fn seek_to(self, seconds: f64) {
        let timeline = self.timeline.get_untracked();
        let Some(target) = timeline.seek_target(seconds) else {
            return;
        };
        if let Some(duration) = timeline.duration.filter(|duration| *duration > 0.0) {
            self.moved(target / duration);
        }
        self.pending.set(false);
        self.target.set(target);
        self.seek.notify();
    }

    /// Moves `seconds` ahead of the player's position, or back where it is negative.
    fn seek_by(self, seconds: f64) {
        self.seek_to(self.timeline.get_untracked().position + seconds);
    }
}

impl Binding<f64> for Scrubber {
    fn read(&self) -> f64 {
        self.fraction.get()
    }
    fn peek(&self) -> f64 {
        self.fraction.get_untracked()
    }
    fn write(&self, fraction: f64) {
        self.moved(fraction);
        self.pending.set(false);
        self.seek_to_thumb();
    }
    fn write_preview(&self, fraction: f64) {
        self.moved(fraction);
        self.pending.set(true);
    }
    // The signals live in the page's scope, which is gone by the time the slider is disposed.
    fn write_teardown(&self, _fraction: f64) {}
}

/// A badge whose text and tone follow the player's state.
fn state_badge(state: Signal<PlaybackState>) -> impl Piece {
    label(move || tr::media_state(state_key(&state.get())).format())
        .font(Font::Caption)
        .weight(FontWeight::Medium)
        .color(move || {
            state_tone(&state.get()).map_or(Ink::system(SystemColor::SecondaryText), |tone| {
                tone.color().into()
            })
        })
        .single_line()
        .id(day::id!(media_state))
        .padding(Insets::symmetric(10.0, 4.0))
        .background(move || state_tone(&state.get()).map_or(gallery::INSET, Tone::tint))
        .corner_radius(Corner::CAPSULE)
}

/// The bar under the video: the buffered ranges, and over them how far playback has come.
fn buffered_bar(timeline: Signal<MediaTimeline>) -> impl Piece {
    canvas(move |draw, size| {
        let radius = size.height / 2.0;
        let span = |from: f64, to: f64| {
            Shape::RoundedRect(
                Rect::new(
                    from * size.width,
                    0.0,
                    (to - from) * size.width,
                    size.height,
                ),
                radius,
            )
        };
        draw.fill(span(0.0, 1.0), gallery::INSET);
        let timeline = timeline.get();
        let Some(duration) = timeline.duration.filter(|duration| *duration > 0.0) else {
            return;
        };
        let accent = Tone::Accent.color();
        for range in &timeline.buffered {
            let from = (range.start / duration).clamp(0.0, 1.0);
            let to = (range.end / duration).clamp(0.0, 1.0);
            draw.fill(span(from, to), accent.with_alpha(0.35));
        }
        let played = (timeline.position / duration).clamp(0.0, 1.0);
        draw.fill(span(0.0, played), accent);
    })
    .a11y(move |a| {
        a.role(Role::Meter)
            .label(tr::media_buffered())
            .value(move || clock(buffered_end(&timeline.get())))
    })
    .id(day::id!(media_buffered))
    .height(6.0)
    .grow(Dimension::Width)
}

/// The end of the furthest buffered range, in seconds.
fn buffered_end(timeline: &MediaTimeline) -> f64 {
    timeline
        .buffered
        .iter()
        .map(|range| range.end)
        .fold(0.0, f64::max)
}

/// What the page binds to the player, shared by its four cards.
#[derive(Clone, Copy)]
struct Deck {
    state: Signal<PlaybackState>,
    timeline: Signal<MediaTimeline>,
    error: Signal<Option<MediaError>>,
    metadata: Signal<Option<StreamMetadata>>,
    rate: Signal<f64>,
    volume: Signal<f64>,
    muted: Signal<bool>,
    /// Whether the page tells the system's now-playing surface about the sample.
    publish: Signal<bool>,
    /// The last command that surface sent.
    remote: Signal<Option<RemoteCommand>>,
}

pub(crate) fn media_page() -> AnyPiece {
    let deck = Deck {
        state: Signal::new(PlaybackState::Idle),
        timeline: Signal::new(MediaTimeline::default()),
        error: Signal::new(None),
        metadata: Signal::new(None),
        rate: Signal::new(1.0),
        volume: Signal::new(START_VOLUME),
        muted: Signal::new(true),
        publish: Signal::new(false),
        remote: Signal::new(None),
    };
    page_cards(
        tr::nav_media(),
        day::id!(media_title),
        gallery::balanced(
            2,
            vec![
                player_card(deck),
                playback_card(deck),
                system_card(deck),
                readout_card(deck),
            ],
        ),
    )
    .any()
}

/// What the now-playing surface shows, `None` while the page does not publish. It changes when
/// the switch, the playing state, the duration or the speed change and when the position jumps;
/// between those the system advances its own clock at the published speed.
fn now_playing(deck: Deck) -> Signal<Option<NowPlayingInfo>> {
    let Deck {
        state,
        timeline,
        publish,
        ..
    } = deck;
    let jumps = Signal::new(0_u32);
    let last = std::cell::Cell::new(None::<f64>);
    Effect::new(move || {
        let position = timeline.with(|timeline| timeline.position);
        if let Some(before) = last.replace(Some(position))
            && (position - before).abs() > JUMP_SECONDS
        {
            jumps.update(|jumps| *jumps += 1);
        }
    });
    let moments = Memo::new(move || {
        publish.get().then(|| {
            (
                state.get() == PlaybackState::Playing,
                timeline.with(|timeline| {
                    (timeline.duration.map(f64::to_bits), timeline.rate.to_bits())
                }),
                jumps.get(),
            )
        })
    });
    let artwork = artwork();
    let info = Signal::new(None);
    Effect::new(move || {
        let published = moments.get().map(|(playing, _, _)| {
            untrack(|| timeline.with(|timeline| now_playing_info(playing, timeline, &artwork)))
        });
        info.set(published);
    });
    info
}

/// The video over its buffered bar, its scrubber and the three transport buttons.
fn player_card(deck: Deck) -> AnyPiece {
    let Deck {
        state,
        timeline,
        error,
        metadata,
        rate,
        volume,
        muted,
        remote,
        ..
    } = deck;
    let play = Trigger::new();
    let pause = Trigger::new();
    let load = Trigger::new();
    let seek = Trigger::new();
    let target = Signal::new(0.0);
    let scrubber = Scrubber::new(timeline, target, seek);

    // A command from the lock screen, the media keys or another system surface goes through the
    // page's own triggers, so the buttons, the scrubber and the badge follow it.
    let on_remote = move |command: RemoteCommand| {
        remote.set(Some(command));
        match command {
            RemoteCommand::Play => play.notify(),
            RemoteCommand::Pause | RemoteCommand::Stop => pause.notify(),
            RemoteCommand::Toggle => {
                if state.get_untracked().is_active() {
                    pause.notify();
                } else {
                    play.notify();
                }
            }
            RemoteCommand::Seek(seconds) => scrubber.seek_to(seconds),
            RemoteCommand::SkipForward(seconds) => scrubber.seek_by(seconds),
            RemoteCommand::SkipBackward(seconds) => scrubber.seek_by(-seconds),
            // The sample is the only item, and the info says there is no other.
            RemoteCommand::Next | RemoteCommand::Previous => {}
        }
    };

    let player = media(clip_source())
        .looping()
        .volume(volume)
        .muted_while(muted)
        .play(play)
        .pause(pause)
        .load(load)
        .seek(target, seek)
        .rate(rate)
        .state(state)
        .timeline(timeline)
        .error(error)
        .metadata(metadata)
        .now_playing(now_playing(deck))
        .on_remote(on_remote);

    let position = label(move || clock(timeline.get().position))
        .font(Font::Caption)
        .secondary()
        .tabular()
        .id(day::id!(media_position))
        .reserving("00:00");
    let duration = label(move || match timeline.get().duration {
        Some(duration) => clock(duration),
        None => tr::media_none().format(),
    })
    .font(Font::Caption)
    .secondary()
    .tabular()
    .id(day::id!(media_duration))
    .reserving("00:00");

    let key: AnyPiece = if reports_buffered() {
        row((
            gallery::legend(Tone::Accent.color(), tr::media_played()),
            gallery::legend(Tone::Accent.color().with_alpha(0.35), tr::media_buffered()),
        ))
        .spacing(Space::M)
        .any()
    } else {
        gallery::note(tr::media_buffered_unavailable()).any()
    };

    gallery::card(
        crate::res::vectors::nav_media,
        tr::media_player_section(),
        tr::media_clip(),
        column((
            // The sample's own shape, so the player shows no letterboxing at any card width.
            player
                .id(day::id!(media))
                .grow(Dimension::Width)
                .aspect_ratio(16.0 / 9.0),
            buffered_bar(timeline),
            row((
                position,
                slider(scrubber)
                    .enabled(move || timeline.get().capabilities.seek)
                    .id(day::id!(media_scrubber)),
                duration,
            ))
            .spacing(Space::S)
            .align(VAlign::Center),
            gallery::actions((
                button(tr::media_play())
                    .prominent()
                    .action(move || play.notify())
                    .id(day::id!(media_play)),
                button(tr::media_pause())
                    .bordered()
                    .action(move || pause.notify())
                    .id(day::id!(media_pause)),
                button(tr::media_load())
                    .bordered()
                    .action(move || load.notify())
                    .id(day::id!(media_load)),
                spacer(),
                key,
            )),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .trailing(state_badge(state))
    .id(day::id!(media_player_card))
    .build()
}

/// Speed, volume and mute: the bindings the player follows as they change. Muting leaves the
/// volume where it is, so the slider keeps its place and unmuting returns to it.
fn playback_card(deck: Deck) -> AnyPiece {
    let Deck {
        rate,
        volume,
        muted,
        ..
    } = deck;
    let fixed_volume = mute_only();
    gallery::panel(
        tr::media_playback_section(),
        (
            labeled(
                tr::media_rate(),
                picker(rate, RATES.map(|rate| (rate, times(rate))))
                    .segmented()
                    .id(day::id!(media_rate)),
            ),
            labeled(
                tr::media_volume(),
                row((
                    slider(volume)
                        .enabled(move || !fixed_volume)
                        .id(day::id!(media_volume)),
                    crate::widgets::numeric_readout(
                        move || format!("{:.0}%", volume.get() * 100.0),
                        "100%",
                        day::id!(media_volume_value),
                    ),
                ))
                .spacing(Space::S)
                .align(VAlign::Center),
            ),
            labeled(tr::media_mute(), toggle(muted).id(day::id!(media_mute))),
            fixed_volume.then(|| gallery::note(tr::media_volume_fixed())),
        ),
    )
}

/// The system's now-playing surface: whether the page publishes to it, and the last command it
/// sent back.
fn system_card(deck: Deck) -> AnyPiece {
    let Deck {
        publish, remote, ..
    } = deck;
    gallery::panel(
        tr::media_system_section(),
        (
            crate::widgets::support_note(crate::support::now_playing()),
            labeled(
                tr::media_now_playing(),
                toggle(publish).id(day::id!(media_now_playing)),
            ),
            labeled(
                tr::media_remote_title(),
                label(move || tr::media_remote(remote_key(remote.get())).format())
                    .id(day::id!(media_remote)),
            ),
            gallery::note(tr::media_system_note()),
        ),
    )
}

/// A figure with its name above it, as `gallery::stat` draws one, with `id` on the figure so a
/// script reads its text. `alert` colors the figure while it reports something wrong.
fn reading(
    name: day::LocalizedText,
    value: impl Fn() -> String + 'static,
    id: ElementId,
    alert: impl Fn() -> bool + 'static,
) -> impl Piece {
    column((
        label(name).font(Font::Caption).secondary(),
        label(value)
            .font(Font::Callout)
            .tabular()
            .color(move || {
                if alert() {
                    Ink::Fixed(Tone::Danger.color())
                } else {
                    SystemColor::Text.into()
                }
            })
            .id(id),
    ))
    .spacing(2.0)
    .align(HAlign::Leading)
    .grow(Dimension::Width)
}

/// What the player reports: the timeline's figures, the stream's metadata and the last error.
fn readout_card(deck: Deck) -> AnyPiece {
    let Deck {
        timeline,
        error,
        metadata,
        ..
    } = deck;
    gallery::group(
        tr::media_readout_section(),
        column((
            row((
                reading(
                    tr::media_buffered(),
                    move || {
                        let timeline = timeline.get();
                        if timeline.buffered.is_empty() {
                            tr::media_none().format()
                        } else {
                            clock(buffered_end(&timeline))
                        }
                    },
                    day::id!(media_buffered_value),
                    || false,
                ),
                reading(
                    tr::media_rate(),
                    move || times(timeline.get().rate),
                    day::id!(media_rate_value),
                    || false,
                ),
            ))
            .spacing(Space::M),
            row((
                reading(
                    tr::media_stream_title(),
                    move || match metadata.get() {
                        Some(metadata) => metadata.raw,
                        None => tr::media_none().format(),
                    },
                    day::id!(media_metadata),
                    || false,
                ),
                reading(
                    tr::media_error_title(),
                    move || match error.get() {
                        Some(error) => error_text(&error),
                        None => tr::media_none().format(),
                    },
                    day::id!(media_error),
                    move || error.get().is_some(),
                ),
            ))
            .spacing(Space::M),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
}
