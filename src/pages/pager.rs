// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The Pager page: five pictures in a `pager` (docs/pager.md), turned by a swipe where the
//! toolkit pages natively (UIKit, Android, ArkUI) and by the Previous and Next buttons
//! everywhere. The dots under it follow the pager's progress, so the highlight slides with the
//! finger where the pager reports its position continuously. Each picture is one slot, and reversing the order moves the slots without
//! changing which picture a slot holds.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use day::prelude::*;
use day::tween::Lerp;

use crate::gallery::{self, sky};
use crate::res::str as strings;

/// One page of the pager: a picture, its name, and one fact about it.
struct Sight {
    /// The picture's id in NASA's library.
    picture: &'static str,
    /// The name of what the picture shows.
    name: fn() -> LocalizedText,
    /// One fact about it.
    fact: fn() -> LocalizedText,
}

/// The pages, in slot order.
const SIGHTS: [Sight; 5] = [
    Sight {
        picture: sky::SATURN,
        name: strings::pager_saturn,
        fact: strings::pager_saturn_fact,
    },
    Sight {
        picture: sky::MARS,
        name: strings::pager_mars,
        fact: strings::pager_mars_fact,
    },
    Sight {
        picture: sky::EARTH,
        name: strings::pager_earth,
        fact: strings::pager_earth_fact,
    },
    Sight {
        picture: sky::CARINA,
        name: strings::pager_carina,
        fact: strings::pager_carina_fact,
    },
    Sight {
        picture: sky::PILLARS,
        name: strings::pager_pillars,
        fact: strings::pager_pillars_fact,
    },
];

/// The height of the pager: room for a picture and two lines under it on a phone.
const PAGER_HEIGHT: f64 = 360.0;

/// The Pager page.
pub(crate) fn pager_page() -> AnyPiece {
    crate::widgets::page_cards(strings::nav_pager(), day::id!(pager_title), tour_card()).any()
}

/// The position of the selected slot in the reading order.
fn position(state: &PagerState) -> usize {
    state
        .order
        .iter()
        .position(|slot| *slot == state.selected)
        .unwrap_or(0)
}

/// Selects the page `step` positions away in the reading order, if there is one. The epoch
/// moves with every change the app makes, so a swipe that settles late cannot undo it.
fn turn(state: Signal<PagerState>, step: isize) {
    state.update(|state| {
        let Some(to) = position(state).checked_add_signed(step) else {
            return;
        };
        if let Some(slot) = state.order.get(to).copied() {
            state.selected = slot;
            state.epoch += 1;
        }
    });
}

/// The pager, its indicator and the controls that turn it.
fn tour_card() -> AnyPiece {
    let state = Signal::new(PagerState {
        order: (0..SIGHTS.len()).collect(),
        selected: 0,
        epoch: 1,
    });
    // True while a swipe is in flight. The buttons wait for it, since the pager applies a
    // change of page only at rest.
    let busy = Signal::new(false);
    let reversed = Signal::new(false);
    watch(
        move || reversed.get(),
        move |_, _| {
            state.update(|state| {
                state.order.reverse();
                state.epoch += 1;
            });
        },
    );
    let at = move || state.with(position);
    let pages = SIGHTS
        .iter()
        .enumerate()
        .map(|(slot, sight)| sight_page(slot, sight, state))
        .collect();
    // The pager's position in reading order, in pages: 1.5 is halfway from the second page to
    // the third. Where the pager pages natively it follows the finger; elsewhere it steps to the
    // selected page.
    let progress = Signal::new(0.0);
    // Each dot is lit by how near the position is to it, so the highlight slides from one dot to
    // the next as the page moves and the current dot stretches into a pill.
    let dots: Vec<AnyPiece> = (0..SIGHTS.len())
        .map(|index| {
            let near = move || (1.0 - (progress.get() - index as f64).abs()).clamp(0.0, 1.0);
            capsule()
                .fill(move || {
                    Ink::system(SystemColor::TertiaryText)
                        .lerp(&gallery::Tone::Accent.color().into(), near())
                })
                .frame(move || 8.0 + 10.0 * near(), 8.0)
                .id(day::id!(pager_dot, index))
                .any()
        })
        .collect();
    let indicator = column((
        row(dots).spacing(6.0).align(VAlign::Center),
        label(move || {
            strings::pager_position(strings::PagerPosition {
                page: at() as i64 + 1,
                count: SIGHTS.len() as i64,
            })
            .format()
        })
        .font(Font::Caption)
        .secondary()
        .tabular()
        .single_line()
        .id(day::id!(pager_position)),
    ))
    .spacing(4.0)
    .align(HAlign::Center)
    .grow(Dimension::Width);
    let controls = row((
        button(strings::pager_previous())
            .bordered()
            .icon(Symbol::Back)
            .enabled(move || !busy.get() && at() > 0)
            .action(move || turn(state, -1))
            .id(day::id!(pager_previous)),
        indicator,
        button(strings::pager_next())
            .bordered()
            .icon(Symbol::Forward)
            .enabled(move || !busy.get() && at() + 1 < SIGHTS.len())
            .action(move || turn(state, 1))
            .id(day::id!(pager_next)),
    ))
    .spacing(Space::S)
    .align(VAlign::Center)
    .grow(Dimension::Width);
    let body = column((
        pager(state, busy, pages, move |slot| {
            state.update(|state| state.selected = slot);
        })
        .progress(progress)
        .id(day::id!(demo_pager))
        .height(PAGER_HEIGHT),
        controls,
        labeled(
            strings::pager_reverse(),
            toggle(reversed).id(day::id!(pager_reverse)),
        ),
        // The composed pager shows the selected page and has no gesture of its own.
        (day::pager::support() != Support::Native)
            .then(|| gallery::note(strings::pager_buttons_note()).id(day::id!(pager_note))),
    ))
    .spacing(Space::M)
    .align(HAlign::Leading)
    .grow(Dimension::Width);
    gallery::card(
        crate::res::vectors::nav_pager.clone(),
        strings::pager_tour_title(),
        strings::pager_tour_subtitle(),
        body,
    )
    .id(day::id!(pager_card))
    .build()
}

/// The page of slot `slot`. Where the toolkit pages natively the slot holds its page for good.
/// The composed pager hides an unselected slot by hiding the slot's wrapper, and on AppKit that
/// wrapper is a layout node with no view to hide, so every page stays drawn over the others.
/// There the slot mounts its page only while it is selected.
fn sight_page(slot: usize, sight: &'static Sight, state: Signal<PagerState>) -> AnyPiece {
    if day::pager::support() == Support::Native {
        return sight_content(slot, sight).any();
    }
    when(
        move || state.with(|state| state.selected) == slot,
        move || sight_content(slot, sight),
    )
    .grow(Dimension::Both)
    .any()
}

/// What a page shows: the picture over its name and the fact.
fn sight_content(slot: usize, sight: &'static Sight) -> impl Piece {
    column((
        picture(sky::url(sight.picture, sky::Size::Medium))
            .rounded(10.0)
            .placeholder_color(gallery::INSET)
            .grow(Dimension::Both),
        label((sight.name)())
            .font(Font::Headline)
            .id(day::id!(pager_name, slot)),
        label((sight.fact)()).font(Font::Footnote).secondary(),
    ))
    .spacing(Space::XS)
    .align(HAlign::Leading)
    .grow(Dimension::Both)
}

thread_local! {
    /// The pictures downloaded so far, by address, so a page mounted again shows its picture
    /// without another request.
    static PICTURES: RefCell<HashMap<String, Arc<Vec<u8>>>> = RefCell::new(HashMap::new());
}

/// The picture at `url`: the kept bytes when it was downloaded before, otherwise a download that
/// stops when the page it belongs to goes away.
fn picture(url: String) -> AsyncImage {
    let kept = PICTURES.with_borrow(|pictures| pictures.get(&url).cloned());
    let bytes = Signal::new(kept.clone());
    if kept.is_none() {
        let task = day::task(async move {
            let request = day_part_http::Request::get(url.clone());
            if let Ok(response) = day_part_http::fetch_future(request).await
                && (200..300).contains(&response.status)
            {
                let picture = Arc::new(response.body);
                PICTURES.with_borrow_mut(|pictures| pictures.insert(url, picture.clone()));
                bytes.set(Some(picture));
            }
        });
        Scope::current().on_cleanup(move || task.abort());
    }
    async_image(bytes)
}
