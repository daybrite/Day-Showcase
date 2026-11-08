// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! Map (day-piece-map): a vector map drawn on Day's canvas from a MapLibre style, with a picker
//! for where the style comes from, the piece's camera calls on buttons, and a readout of what it
//! reports.
//!
//! The default source is the piece's built-in sample, a small island whose style and tiles are
//! answered inside the app through `day-part-http`'s interception, so the page draws the same
//! map on every target without a network (the CI captures have none). The other entries are
//! OpenFreeMap's five styles and VersaTiles' two over London, all OpenStreetMap data served
//! without a key, which go out to the network: the interception is not strict, so only the
//! sample's own URLs are answered in-process.
//!
//! The Current Location button beside the picker asks for location access the first time, then
//! for one fix, and flies there and marks it. The style stays as chosen, so on the island the
//! flight lands on open sea.
//!
//! Nothing here is map code: the piece draws, pans, pinches, turns and places labels on every
//! backend, and the page only shares a [`MapView`] with it.

use day::prelude::*;
use day_part_http::{Session, simulation::Simulation};
use day_part_location::{Accuracy, LocationError};
use day_part_permissions::{self as perms, Permission};
use day_piece_map::{LngLat, MapStatus, MapView, Marker, MarkerId, map, sample};

use crate::res::str as tr;
use crate::widgets::heading;

/// Trafalgar Square and Tower Bridge: the two London sights the live styles mark.
const TRAFALGAR: LngLat = LngLat::new(-0.1281, 51.5080);
const TOWER_BRIDGE: LngLat = LngLat::new(-0.0754, 51.5055);

/// The zoom a flight lands at: street level on the live styles, the island's lanes on the sample.
const FLY_ZOOM: f64 = 16.0;
/// Points kept clear around the two sights when Fit frames them.
const FIT_PADDING: f64 = 60.0;
/// The tag on the marker at the user's own position, which a tap on it is told apart by.
const HERE: &str = "here";

/// Where the map's style comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Source {
    /// The hand-drawn island, served in-process.
    Sample,
    Liberty,
    Bright,
    Positron,
    Fiord,
    Dark,
    /// VersaTiles Colorful: the Shortbread schema, so a different set of layers and sources.
    Colorful,
    ColorfulDark,
}

impl Source {
    fn style_url(self) -> &'static str {
        match self {
            Source::Sample => sample::STYLE_URL,
            Source::Liberty => "https://tiles.openfreemap.org/styles/liberty",
            Source::Bright => "https://tiles.openfreemap.org/styles/bright",
            Source::Positron => "https://tiles.openfreemap.org/styles/positron",
            Source::Fiord => "https://tiles.openfreemap.org/styles/fiord",
            Source::Dark => "https://tiles.openfreemap.org/styles/dark",
            Source::Colorful => "https://tiles.versatiles.org/assets/styles/colorful/style.json",
            Source::ColorfulDark => {
                "https://tiles.versatiles.org/assets/styles/colorful-dark/style.json"
            }
        }
    }

    /// Where a source opens: the whole island, or central London close enough to read streets.
    fn home(self) -> (LngLat, f64) {
        match self {
            Source::Sample => (sample::HOME, 9.0),
            _ => (TRAFALGAR, 14.0),
        }
    }

    /// The two places this source marks, flies between and fits.
    fn sights(self) -> [Sight; 2] {
        match self {
            Source::Sample => [Sight::Center, Sight::Coast],
            _ => [Sight::Trafalgar, Sight::Bridge],
        }
    }
}

/// A marked place. Its name is a catalog message selected by [`Sight::key`], so a script can
/// assert a tap on its marker in every locale.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Sight {
    /// The middle of the island, where the sample view opens.
    Center,
    Coast,
    Trafalgar,
    Bridge,
}

impl Sight {
    fn key(self) -> &'static str {
        match self {
            Sight::Center => "center",
            Sight::Coast => "coast",
            Sight::Trafalgar => "trafalgar",
            Sight::Bridge => "bridge",
        }
    }

    fn at(self) -> LngLat {
        match self {
            Sight::Center => sample::HOME,
            Sight::Coast => sample::EAST_COAST,
            Sight::Trafalgar => TRAFALGAR,
            Sight::Bridge => TOWER_BRIDGE,
        }
    }
}

/// Answer the sample's URLs in-process for the whole app. The HTTP session holds one simulation,
/// and nothing else in the app installs one on the global session (the Network page runs its
/// own session), so this one is the app's. Not strict: every other URL, the live styles' among
/// them, still reaches the network.
fn install_sample() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let sim = Simulation::new();
        sim.set_strict(false);
        sample::routes(&sim);
        Session::global().set_simulation(Some(sim));
    });
}

/// A coordinate to four places, without the `-0.0000` a value a hair below zero would print.
fn coord(v: f64) -> String {
    let text = format!("{v:.4}");
    if text == "-0.0000" {
        "0.0000".to_string()
    } else {
        text
    }
}

/// What the Current Location button is doing, while it is doing something other than nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Locating {
    /// Waiting on the permission prompt or the fix.
    Busy,
    /// Why there is no fix: a `map_locate_failed` selector.
    Failed(&'static str),
}

/// The `map_locate_failed` selector for a location error.
fn failure(error: &LocationError) -> &'static str {
    match error {
        LocationError::PermissionDenied => "denied",
        LocationError::Disabled => "off",
        LocationError::Timeout => "timeout",
        LocationError::Unavailable => "unavailable",
        _ => "other",
    }
}

/// The bearing in whole degrees, `0..360`.
fn degrees(bearing: f64) -> i64 {
    (bearing.rem_euclid(360.0).round() as i64) % 360
}

pub(crate) fn map_page() -> AnyPiece {
    install_sample();
    // Liberty for people, the island for the script: the walkthrough switches to it first.
    let source = Signal::new(Source::Liberty);
    let (center, zoom) = Source::Liberty.home();
    let view = MapView::new(center, zoom);
    // Which sight each marker on the map stands for, so the tap readout names it.
    let marks: Signal<Vec<(MarkerId, Sight)>> = Signal::new(Vec::new());
    let flights = Signal::new(0usize);
    // The user's position once the button has found it, marked with the source's sights.
    let here: Signal<Option<LngLat>> = Signal::new(None);
    let locating: Signal<Option<Locating>> = Signal::new(None);

    // Choosing a source goes to its home, north up, and forgets the last tap.
    Effect::new(move || {
        let (center, zoom) = source.get().home();
        view.jump_to(center, zoom);
        view.fly_to_with(center, zoom, Some(0.0), 0.0);
        view.tap.set(None);
    });
    // Labels follow the app's language where the data has names in it (the island is named in
    // French too, OpenFreeMap in dozens of languages); a feature without one keeps its own name.
    Effect::new(move || {
        let locale = day::locale().get();
        let language = locale.split(['-', '_']).next().unwrap_or_default();
        view.set_language((!language.is_empty()).then_some(language));
    });
    // The source's two markers and the user's own, titled in the current language: this effect
    // follows the language too, so a switch at runtime renames them without moving the map.
    Effect::new(move || {
        view.clear_markers();
        if let Some(at) = here.get() {
            view.add_marker(
                Marker::new(at)
                    .title(tr::map_sight(HERE).format())
                    .tag(HERE)
                    .color(Color::rgb(0.0, 0.48, 1.0)),
            );
        }
        let placed = source
            .get()
            .sights()
            .into_iter()
            .map(|sight| {
                let title = tr::map_sight(sight.key()).format();
                (view.add_marker(Marker::new(sight.at()).title(title)), sight)
            })
            .collect();
        marks.set(placed);
    });

    let picker_row = picker(
        source,
        [
            (Source::Liberty, tr::map_source_liberty()),
            (Source::Bright, tr::map_source_bright()),
            (Source::Positron, tr::map_source_positron()),
            (Source::Fiord, tr::map_source_fiord()),
            (Source::Dark, tr::map_source_dark()),
            (Source::Colorful, tr::map_source_colorful()),
            (Source::ColorfulDark, tr::map_source_colorful_dark()),
            (Source::Sample, tr::map_source_sample()),
        ],
    )
    .menu()
    .id(day::id!(map_source));

    // Permission first: the request answers at once when access is already settled and prompts
    // only when nobody has decided yet, so one call covers the first tap and every later one.
    // Both awaits resume on the main thread, where CoreLocation wants its manager made.
    let locate = button(tr::map_locate())
        .icon(Symbol::Location)
        .icon_only()
        .enabled(move || locating.get() != Some(Locating::Busy))
        .action(move || {
            locating.set(Some(Locating::Busy));
            day::task(async move {
                if !perms::request_future(Permission::Location)
                    .await
                    .is_granted()
                {
                    locating.set(Some(Locating::Failed("denied")));
                    return;
                }
                match day_part_location::current_future(Accuracy::Balanced).await {
                    Ok(fix) => {
                        let at = LngLat::new(fix.longitude, fix.latitude);
                        here.set(Some(at));
                        view.fly_to(at, FLY_ZOOM);
                        locating.set(None);
                    }
                    Err(error) => locating.set(Some(Locating::Failed(failure(&error)))),
                }
            });
        })
        .id(day::id!(map_locate));

    // The camera calls: fly between the two sights, frame both, turn by 45°, back to north up.
    let buttons = row((
        button(tr::map_fly())
            .bordered()
            .action(move || {
                let n = flights.get_untracked() + 1;
                flights.set(n);
                view.fly_to(source.get_untracked().sights()[n % 2].at(), FLY_ZOOM);
            })
            .id(day::id!(map_fly)),
        button(tr::map_fit())
            .bordered()
            .action(move || {
                let [a, b] = source.get_untracked().sights().map(Sight::at);
                let sw = LngLat::new(a.lng.min(b.lng), a.lat.min(b.lat));
                let ne = LngLat::new(a.lng.max(b.lng), a.lat.max(b.lat));
                view.fit_bounds(sw, ne, FIT_PADDING);
            })
            .id(day::id!(map_fit)),
        button(tr::map_rotate())
            .bordered()
            .action(move || view.rotate_to(view.bearing.get_untracked() + 45.0))
            .id(day::id!(map_rotate)),
        button(tr::map_north())
            .bordered()
            .action(move || view.reset_north())
            .id(day::id!(map_north)),
    ))
    .spacing(Space::S);

    let readout = label(move || {
        let (status, reason) = match view.status.get() {
            MapStatus::Loading => ("loading", String::new()),
            MapStatus::Ready => ("ready", String::new()),
            MapStatus::Failed(why) => ("failed", why),
        };
        let c = view.center.get();
        tr::map_readout(tr::MapReadout {
            status,
            reason,
            lat: coord(c.lat),
            lng: coord(c.lng),
            zoom: format!("{:.2}", view.zoom.get()),
            bearing: degrees(view.bearing.get()),
        })
        .format()
    })
    .font(Font::Footnote)
    .id(day::id!(map_readout));

    // What the last tap found: a marker, else the topmost feature the style drew there.
    let tap = label(move || {
        let Some(tap) = view.tap.get() else {
            return tr::map_tap_prompt().format();
        };
        let sight = tap.marker.and_then(|id| {
            marks
                .get()
                .into_iter()
                .find(|(m, _)| *m == id)
                .map(|(_, s)| s)
        });
        if let Some(sight) = sight {
            return tr::map_tap_marker(sight.key()).format();
        }
        if tap
            .marker
            .and_then(|id| view.marker(id))
            .is_some_and(|m| m.tag == HERE)
        {
            return tr::map_tap_marker(HERE).format();
        }
        match tap.features.first() {
            None => tr::map_tap_nothing().format(),
            Some(hit) => match hit.properties.get("name").and_then(|v| v.as_str()) {
                Some(name) => tr::map_tap_feature(tr::MapTapFeature {
                    layer: hit.layer.clone(),
                    name: name.to_string(),
                })
                .format(),
                None => tr::map_tap_layer(hit.layer.clone()).format(),
            },
        }
    })
    .font(Font::Footnote)
    .id(day::id!(map_tap));

    // Only while the button is busy or has something to say: an empty line otherwise.
    let note = when(
        move || locating.get().is_some(),
        move || {
            label(move || match locating.get() {
                Some(Locating::Failed(why)) => tr::map_locate_failed(why).format(),
                _ => tr::map_locating().format(),
            })
            .font(Font::Footnote)
            .id(day::id!(map_locate_note))
        },
    );

    // Not `widgets::page`: that scrolls, and the map sizes itself by growing into the room it
    // is given, which a scroll view never offers.
    column((
        heading(tr::nav_map(), day::id!(map_title)),
        row((picker_row, spacer(), locate)).spacing(Space::S),
        map(move || source.get().style_url().to_string(), view)
            .id(day::id!(map_canvas))
            .grow(Dimension::Both),
        buttons,
        readout,
        tap,
        note,
    ))
    .spacing(8.0)
    .padding(20.0)
    .grow(Dimension::Both)
    .any()
}
