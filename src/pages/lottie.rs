//! Lottie (day-piece-lottie): every example page of the Lottie piece, with a picker between them.
//! The pages are day-piece-lottie's own, from its `day-piece-lottie-gallery` crate, the same code
//! its Lottie Demo app is a sidebar of: a playground that swaps the running animation and reads
//! the file with the piece's headless `LottieModel`, and one page per bundled animation.
//!
//! Nothing here is Lottie code, and this app bundles no animation: the gallery's files arrive in
//! the bundle as its data assets, its strings come from its private catalog (following this app's
//! language), and it switches the player's backends on itself. Two renderers sit behind the one
//! API: iOS and Android get Airbnb's own players, and every other backend plays the same file with
//! lottie-web in a web view through day-piece-webview.
//!
//! The page exists only where the piece has an arm: the crate carries no `support()` and
//! `Cap::Lottie` goes unanswered on every backend, so the target cfg is the gate, here and in
//! lib.rs `destinations`. The one gap is `macos-gtk` and `windows-gtk`, which ship no WebKitGTK
//! to host the player, so the piece would realize Day's placeholder there.

use day::prelude::*;
use day_piece_lottie_gallery::{LottiePage, gallery};

use crate::widgets::heading;

thread_local! {
    /// The open page, kept for the life of the app, so leaving the section and coming back
    /// returns to the same animation.
    static OPEN: Signal<LottiePage> = Signal::global(LottiePage::Playground);
}

#[cfg(not(all(feature = "gtk", any(target_os = "macos", target_os = "windows"))))]
pub(crate) fn lottie_page() -> AnyPiece {
    let open = OPEN.with(|s| *s);
    // Not `widgets::page`: that scrolls, and an animation sizes itself by growing into the room
    // it is given, which a scroll view never offers.
    column((
        heading(crate::res::str::nav_lottie(), "lottie-title"),
        gallery(open),
    ))
    .spacing(8.0)
    .padding(20.0)
    .grow()
    .any()
}
