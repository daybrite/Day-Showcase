use day::prelude::*;

use crate::gallery;
use crate::widgets::page_cards;
use crate::{Dest, Group};

/// About, the opening page (and the desktop split's default detail): the app's About as
/// day-piece-settings composes it (docs/settings.md), the vector mark, the name and version
/// from Day.toml, the blurb and the site link, declared once in `crate::install_settings`.
/// The App menu's About window and a phone's Settings header open the same content, so the
/// catalog's front page and the About can never drift apart. The facts about this build and
/// the platform it landed on moved to Settings ▸ Diagnostics.
///
/// Under that identity the page is the gallery's front door: what this build holds in three
/// figures, then one card per sidebar group saying what it shows, with a button for each of its
/// pages, and the places to read more. The cards are derived from the destination table, so a
/// page added to the sidebar appears here too.
pub(crate) fn about_page() -> AnyPiece {
    let destinations = crate::destinations();
    let groups = [
        (Group::Controls, crate::res::str::about_group_controls()),
        (Group::Layout, crate::res::str::about_group_layout()),
        (Group::Navigation, crate::res::str::about_group_navigation()),
        (Group::Data, crate::res::str::about_group_data()),
        (Group::Graphics, crate::res::str::about_group_graphics()),
        (Group::Platform, crate::res::str::about_group_platform()),
        (Group::App, crate::res::str::about_group_app()),
    ];
    let mut cards: Vec<AnyPiece> = Vec::new();
    for (group, says) in groups {
        let pages: Vec<&Dest> = destinations
            .iter()
            .filter(|dest| dest.group == group)
            .collect();
        if pages.is_empty() {
            continue;
        }
        cards.push(group_card(group, says, &pages));
    }
    cards.push(links_card());
    page_cards(
        crate::res::str::nav_about(),
        day::id!(about_title),
        column((
            crate::prefs().about_page(),
            figures(&destinations),
            gallery::balanced(2, cards),
        ))
        .spacing(Space::L)
        .align(HAlign::Leading)
        .grow(Dimension::Width),
    )
    .any()
}

/// What this build holds: its pages, the groups they fall in, the languages it speaks and the
/// toolkit drawing it.
fn figures(destinations: &[Dest]) -> impl Piece {
    let mut groups: Vec<Group> = Vec::new();
    for dest in destinations {
        if !groups.contains(&dest.group) {
            groups.push(dest.group);
        }
    }
    row((
        gallery::stat(
            crate::res::str::about_stat_pages(),
            destinations.len().to_string(),
            Some(gallery::Tone::Accent),
        ),
        gallery::stat(
            crate::res::str::about_stat_groups(),
            groups.len().to_string(),
            None,
        ),
        gallery::stat(
            crate::res::str::about_stat_languages(),
            crate::res::locales::ALL.len().to_string(),
            None,
        ),
        gallery::stat(
            crate::res::str::about_toolkit(),
            day::toolkit_name().to_string(),
            None,
        ),
    ))
    .spacing(Space::M)
    .align(VAlign::Top)
    .grow(Dimension::Width)
    .card()
    .id(day::id!(about_figures))
}

/// One sidebar group: its glyph in the group's color, its name, what it shows, and a button
/// that opens each of its pages.
fn group_card(group: Group, says: LocalizedText, pages: &[&Dest]) -> AnyPiece {
    let buttons: Vec<AnyPiece> = pages
        .iter()
        .map(|dest| {
            let section = dest.section;
            button((dest.title)())
                .bordered()
                .action(move || {
                    navigate_to(&section);
                })
                .id(day::id!(about_open, section.key()))
                .any()
        })
        .collect();
    let icon = pages
        .first()
        .map_or(crate::res::vectors::nav_about, |dest| dest.icon.clone());
    let build = || {
        gallery::card(icon, (group.title())(), says, gallery::actions(buttons))
            .id(day::id!(about_group, group.id()))
            .build()
    };
    // The card's tile takes the color of the page being built, so the group's own color is
    // set around the build.
    match gallery::current_page() {
        Some(mut info) => {
            info.tint = group.tint();
            gallery::with_page(info, build)
        }
        None => build(),
    }
}

/// Where to read more: the documentation, this app's source and the framework's.
fn links_card() -> AnyPiece {
    gallery::card(
        crate::res::vectors::nav_about,
        crate::res::str::about_links_title(),
        crate::res::str::about_links_subtitle(),
        gallery::actions((
            link(
                crate::res::str::about_link_docs(),
                "https://daybrite.dev/docs",
            )
            .id(day::id!(about_docs)),
            link(crate::res::str::about_link_source(), crate::SOURCE_REPO)
                .id(day::id!(about_source)),
            link(
                crate::res::str::about_link_day(),
                "https://github.com/daybrite/day",
            )
            .id(day::id!(about_day)),
        )),
    )
    .id(day::id!(about_links))
    .build()
}
