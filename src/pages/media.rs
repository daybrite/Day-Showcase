use day::prelude::*;
use day_piece_media::media;

use crate::widgets::page_wide;

/// A native media player (day-piece-media, an EXTERNAL standalone piece): AVPlayerView /
/// AVPlayerViewController / QMediaPlayer+QVideoWidget / android.widget.VideoView / GtkVideo.
/// Transport is imperative via `Trigger`s the piece watches; native chrome (where the toolkit
/// has one) offers its own controls too. The bundled Lottie animation has a page of its own
/// ([`lottie_page`]) on the targets that can draw one.
pub(crate) fn media_page() -> AnyPiece {
    let url = Signal::new(
        "https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4".to_string(),
    );
    let play = Trigger::new();
    let pause = Trigger::new();
    let load = Trigger::new();
    let video = section((
        // muted: CI walkthroughs screenshot this page — don't blast audio on runners. The
        // fixed height keeps the 16:9 sample balanced against the transport row instead of
        // flooding the page with letterboxing.
        media(url)
            .looping(true)
            .muted(true)
            .play(play)
            .pause(pause)
            .load(load)
            .id("media")
            .height(300.0),
        row((
            button(crate::res::str::media_play())
                .prominent()
                .action(move || play.notify())
                .id("media-play"),
            button(crate::res::str::media_pause())
                .bordered()
                .action(move || pause.notify())
                .id("media-pause"),
            button(crate::res::str::media_load())
                .bordered()
                .action(move || load.notify())
                .id("media-load"),
        ))
        .spacing(8.0),
    ))
    .title(crate::res::str::media_player_section());
    page_wide(
        crate::res::str::nav_media(),
        "media-title",
        Some(crate::res::str::media_caption()),
        form((video,)).any(),
    )
    .any()
}

/// The bundled Lottie animation on its own page (day-piece-lottie, an EXTERNAL standalone
/// piece): a LottieAnimationView driven by airbnb's lottie-ios (SwiftPM) / lottie-android
/// (Gradle), rendering `resource/assets/hello.json` in a loop. The page exists only where the
/// piece has an arm: the crate carries no `support()` and `Cap::Lottie` goes unanswered on
/// every backend, so the target cfg is the gate, here and in lib.rs `destinations`.
#[cfg(any(target_os = "ios", target_os = "android"))]
pub(crate) fn lottie_page() -> AnyPiece {
    // Which bundled animation plays: the picker writes it, `lottie(closure)` reads it and swaps
    // the native view's file live (a `Name` patch), and the facts panel reads it too. Pin jump
    // opens the page (LOTTIE_DEFAULT): the liveliest of the set at a glance.
    let selected = Signal::new(LOTTIE_DEFAULT);
    let name = move || {
        LOTTIE_ANIMATIONS[selected.get().min(LOTTIE_ANIMATIONS.len() - 1)]
            .0
            .to_string()
    };
    // Playback rate, bound two ways: the slider (or a preset button) drives it and
    // `.speed(speed)` pushes it to the native LottieAnimationView live (a `Speed` patch per
    // change). Looping and autoplay are build-time properties of the view.
    let speed = Signal::new(1.0);
    let preset = |label: &'static str, value: f64, id: &'static str| {
        button(label)
            .bordered()
            .action(move || speed.set(value))
            .id(id)
    };
    let stage = section((
        labeled(
            crate::res::str::lottie_animation(),
            picker(
                [
                    crate::res::str::lottie_anim_hello().format(),
                    crate::res::str::lottie_anim_hamburger().format(),
                    crate::res::str::lottie_anim_heart().format(),
                    crate::res::str::lottie_anim_watermelon().format(),
                    crate::res::str::lottie_anim_pin().format(),
                    crate::res::str::lottie_anim_logo().format(),
                ],
                selected,
            )
            .menu()
            .id("lottie-animation"),
        ),
        column((lottie(name)
            .looping(true)
            .autoplay(true)
            .speed(speed)
            .frame(280.0, 280.0)
            .id("lottie-view"),))
        .align(HAlign::Center)
        .grow_w(),
        labeled(
            crate::res::str::lottie_speed(),
            row((
                slider(speed)
                    .range(0.25..=3.0)
                    .step(0.25)
                    .id("lottie-speed-slider"),
                crate::widgets::numeric_readout(
                    move || format!("{:.2}\u{d7}", speed.get()),
                    "3.00\u{d7}",
                    "lottie-speed-value",
                ),
            ))
            .spacing(8.0),
        ),
        // Presets: the same signal the slider writes, so a tap moves the slider and the
        // readout together — and gives a script a deterministic value to assert.
        row((
            preset("\u{bd}\u{d7}", 0.5, "lottie-speed-half"),
            preset("1\u{d7}", 1.0, "lottie-speed-one"),
            preset("2\u{d7}", 2.0, "lottie-speed-double"),
        ))
        .spacing(8.0),
    ))
    .title(crate::res::str::lottie_playback_section());
    crate::widgets::page(
        crate::res::str::nav_lottie(),
        "lottie-title",
        Some(crate::res::str::lottie_caption()),
        form((stage, lottie_facts(selected))).any(),
    )
    .any()
}

/// The bundled animations the Lottie page offers: the name `lottie()` loads by (a `/` path under
/// `resource/assets/`), and the file's text for the facts panel. `hello.json` is hand-authored;
/// the rest are Airbnb's samples (resource/assets/lottie/README.md). The picker's order.
#[cfg(any(target_os = "ios", target_os = "android"))]
const LOTTIE_DEFAULT: usize = 4; // pin jump

#[cfg(any(target_os = "ios", target_os = "android"))]
const LOTTIE_ANIMATIONS: [(&str, &str); 6] = [
    ("hello", include_str!("../../resource/assets/hello.json")),
    (
        "lottie/hamburger-arrow",
        include_str!("../../resource/assets/lottie/hamburger-arrow.json"),
    ),
    (
        "lottie/heart",
        include_str!("../../resource/assets/lottie/heart.json"),
    ),
    (
        "lottie/watermelon",
        include_str!("../../resource/assets/lottie/watermelon.json"),
    ),
    (
        "lottie/pin-jump",
        include_str!("../../resource/assets/lottie/pin-jump.json"),
    ),
    (
        "lottie/lottie-logo",
        include_str!("../../resource/assets/lottie/lottie-logo.json"),
    ),
];

/// The selected file read by the piece's headless `model` module: the same bytes the native view
/// plays, so the page states each animation's length beside it and the walkthrough asserts the
/// reader's answers inside the iOS and Android builds. Every row follows the picker.
#[cfg(any(target_os = "ios", target_os = "android"))]
fn lottie_facts(selected: Signal<usize>) -> AnyPiece {
    use day_piece_lottie::LottieModel;
    let fact = move |pick: fn(&LottieModel) -> String| {
        move || {
            let text = LOTTIE_ANIMATIONS[selected.get().min(LOTTIE_ANIMATIONS.len() - 1)].1;
            match LottieModel::parse(text) {
                Ok(model) => pick(&model),
                Err(e) => e.to_string(),
            }
        }
    };
    section((
        labeled(
            crate::res::str::lottie_model_frames(),
            label(fact(|m| format!("{} @ {} fps", m.frames(), m.frame_rate)))
                .id("lottie-model-frames"),
        ),
        labeled(
            crate::res::str::lottie_model_duration(),
            label(fact(|m| format!("{:.1} s", m.duration_secs()))).id("lottie-model-duration"),
        ),
        labeled(
            crate::res::str::lottie_model_layers(),
            label(fact(|m| m.layers.len().to_string())).id("lottie-model-layers"),
        ),
        labeled(
            crate::res::str::lottie_model_issues(),
            label(fact(|m| m.verify().len().to_string())).id("lottie-model-issues"),
        ),
    ))
    .title(crate::res::str::lottie_model_section())
    .any()
}

#[cfg(any(target_os = "ios", target_os = "android"))]
use day_piece_lottie::lottie;
