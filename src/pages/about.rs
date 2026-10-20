use day::prelude::*;

use crate::widgets::page;

/// About, the opening page (and the desktop split's default detail): the app's About as
/// day-piece-settings composes it (docs/settings.md), the vector mark, the name and version
/// from Day.toml, the blurb and the site link, declared once in `crate::install_settings`.
/// The App menu's About window and a phone's Settings header open the same content, so the
/// catalog's front page and the About can never drift apart. The facts about this build and
/// the platform it landed on moved to Settings ▸ Diagnostics.
pub(crate) fn about_page() -> AnyPiece {
    page(
        crate::res::str::nav_about(),
        day::id!(about_title),
        crate::prefs().about_page(),
    )
    .any()
}
