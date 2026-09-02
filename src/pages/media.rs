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
        column((lottie("hello")
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
        form((stage,)).any(),
    )
    .any()
}

#[cfg(any(target_os = "ios", target_os = "android"))]
use day_piece_lottie::lottie;
