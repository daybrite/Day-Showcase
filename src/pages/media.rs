use day::prelude::*;
use day_piece_media::media;

use crate::widgets::page_wide;

/// A native media player (day-piece-media, an external standalone piece): AVPlayerView /
/// AVPlayerViewController / QMediaPlayer+QVideoWidget / android.widget.VideoView / GtkVideo.
/// Transport is imperative via `Trigger`s the piece watches; native chrome (where the toolkit
/// has one) offers its own controls too. Lottie animations have a page of their own
/// (`pages::lottie`) on the targets that can draw one.
pub(crate) fn media_page() -> AnyPiece {
    let url = Signal::new(
        "https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4".to_string(),
    );
    let play = Trigger::new();
    let pause = Trigger::new();
    let load = Trigger::new();
    let video = section((
        // muted: CI walkthroughs screenshot this page, so don't blast audio on runners. The
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
        form((video,)).any(),
    )
    .any()
}
