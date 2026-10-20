use day::prelude::*;
use day_piece_map::map;

#[cfg(any(target_os = "macos", target_os = "ios"))]
/// A native map view (day-piece-map, an external standalone piece), Apple platforms only. Preset
/// buttons recenter the map live via a bound coordinate `Signal` (a `Center` patch to the native
/// `MKMapView`). The map fills its `.frame`, and a marker pins the initial Boston center.
pub(crate) fn map_page() -> AnyPiece {
    const BOSTON: (f64, f64) = (42.3601, -71.0589);
    const PARIS: (f64, f64) = (48.8566, 2.3522);
    let center = Signal::new(BOSTON);
    column((
        label(crate::res::str::nav_map())
            .font(Font::Title)
            .id("map-title"),
        label(crate::res::str::map_caption()).id("map-caption"),
        row((
            button(crate::res::str::map_boston())
                .bordered()
                .action(move || center.set(BOSTON))
                .id("map-boston"),
            button(crate::res::str::map_paris())
                .bordered()
                .action(move || center.set(PARIS))
                .id("map-paris"),
        ))
        .spacing(Space::S),
        label(move || {
            let (lat, lon) = center.get();
            format!("{lat:.4}, {lon:.4}")
        })
        .id("map-coords"),
        map()
            .center(center)
            .span(0.05)
            .marker(BOSTON.0, BOSTON.1)
            .id("map")
            .grow(),
    ))
    .spacing(Space::M)
    .align(HAlign::Leading)
    .padding(Space::L)
    .grow()
    .any()
}
