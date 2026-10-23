use day::prelude::*;
const CARD_BG: Color = Color::rgba(0.5, 0.5, 0.5, 0.12);
const BADGE_BLUE: Color = Color::hex(0x0A_84_FF);

/// A reusable card surface, applied via [`Decorate::modifier`]: pads its content, paints a subtle
/// translucent background, and rounds the corners. Pure composition over
/// [`padding`](Decorate::padding) / [`background`](Decorate::background) /
/// [`corner_radius`](Decorate::corner_radius).
///
/// ```ignore
/// column((label("Title"), label("Body"))).modifier(Card)
/// ```
pub struct Card;

impl Modifier for Card {
    fn apply(self, content: AnyPiece) -> AnyPiece {
        content
            .padding(16.0)
            .background(CARD_BG)
            .corner_radius(12.0)
            .any()
    }
}

// ---------------------------------------------------------------------------
// badge: a numbered pill overlaid on a piece's top-trailing corner
// ---------------------------------------------------------------------------

/// Overlay a small numbered blue pill on the top-trailing corner of `over` (a notification badge),
/// via [`Decorate::overlay_aligned`]. Returns `over` unchanged when `count <= 0`, so a zero count
/// shows no badge. Pure composition: the pill is a padded, rounded, colored [`label`].
///
/// ```ignore
/// badge(3, icon.any())  // the icon with a "3" pill in its corner
/// ```
pub fn badge(count: i64, over: AnyPiece) -> AnyPiece {
    if count <= 0 {
        return over;
    }
    let pill = label(crate::res::str::badge_count(count).format())
        .font(Font::Caption2)
        .color(Color::WHITE)
        .padding(Insets::symmetric(6.0, 2.0))
        .background(BADGE_BLUE)
        .corner_radius(10.0);
    over.overlay_aligned(Alignment::TopTrailing, pill).any()
}
