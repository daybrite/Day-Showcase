// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

use crate::{gallery, res::str, widgets};
use day::prelude::*;

pub(crate) fn feedback_page() -> AnyPiece {
    widgets::page_cards(
        str::nav_feedback(),
        day::id!(feedback_title),
        gallery::deck(3, vec![tooltip_card(), popover_card(), toast_card()]),
    )
    .any()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Length {
    Short,
    Medium,
    Long,
}

fn length_picker(length: Signal<Length>, id: ElementId) -> impl Piece {
    labeled(
        str::feedback_length(),
        picker(
            length,
            [
                (Length::Short, str::feedback_short()),
                (Length::Medium, str::feedback_medium()),
                (Length::Long, str::feedback_long()),
            ],
        )
        .segmented()
        .id(id)
        .a11y(|a| a.label(str::feedback_length())),
    )
}

fn tooltip_card() -> AnyPiece {
    let length = Signal::new(Length::Short);
    gallery::panel(
        str::feedback_tooltip(),
        (
            gallery::note(str::feedback_tooltip_help()),
            length_picker(length, day::id!(tooltip_length)),
            label(str::feedback_draft())
                .padding(Space::M)
                .background(gallery::INSET)
                .corner_radius(8.0)
                .id(day::id!(tooltip_target))
                .tooltip(move || {
                    match length.get() {
                        Length::Short => str::feedback_tooltip_short(),
                        Length::Medium => str::feedback_tooltip_medium(),
                        Length::Long => str::feedback_tooltip_long(),
                    }
                    .format()
                }),
        ),
    )
}

fn popover_card() -> AnyPiece {
    let length = Signal::new(Length::Short);
    let narrow = Signal::new(false);
    let open = Signal::new(false);
    gallery::panel(
        str::feedback_popover(),
        (
            gallery::note(str::feedback_popover_help()),
            length_picker(length, day::id!(popover_length)),
            labeled(
                str::feedback_narrow(),
                toggle(narrow)
                    .id(day::id!(popover_narrow))
                    .a11y(|a| a.label(str::feedback_narrow())),
            ),
            button(str::feedback_open())
                .id(day::id!(popover_open))
                .action(move || open.set(true))
                .popover(open, move || {
                    let message = match length.get_untracked() {
                        Length::Short => str::feedback_popover_short(),
                        Length::Medium => str::feedback_popover_medium(),
                        Length::Long => str::feedback_popover_long(),
                    };
                    column((
                        label(message).id(day::id!(popover_body)),
                        button(str::feedback_close())
                            .id(day::id!(popover_close))
                            .action(move || open.set(false)),
                    ))
                    .align(HAlign::Leading)
                    .spacing(Space::M)
                    .padding(Space::M)
                    .max_width(if narrow.get_untracked() {
                        200.0
                    } else {
                        320.0
                    })
                }),
        ),
    )
}

fn toast_card() -> AnyPiece {
    let length = Signal::new(Length::Short);
    gallery::panel(
        str::feedback_toast(),
        (
            gallery::note(str::feedback_toast_help()),
            length_picker(length, day::id!(toast_length)),
            button(str::feedback_show())
                .id(day::id!(toast_show))
                .action(move || {
                    toast(match length.get_untracked() {
                        Length::Short => str::feedback_toast_short(),
                        Length::Medium => str::feedback_toast_medium(),
                        Length::Long => str::feedback_toast_long(),
                    });
                }),
        ),
    )
}
