// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! A deterministic async-data example: refresh, fail, cancel and change identity.
use crate::res::str as tr;
use day::prelude::*;

pub(super) fn section() -> impl Piece {
    let source = Signal::new(1i64);
    let fail = Signal::new(false);
    let requests = std::cell::Cell::new(0i64);
    let data = AsyncData::new(
        move || source.get(),
        move |source| {
            let fail = fail.get_untracked();
            let request = requests.get() + 1;
            requests.set(request);
            async move {
                day::sleep(600).await;
                if fail { Err(()) } else { Ok((source, request)) }
            }
        },
    );
    crate::gallery::panel(
        tr::async_data_title(),
        (
            crate::gallery::note(tr::async_data_note()),
            row((
                button(tr::async_data_refresh())
                    .action(move || data.refetch())
                    .id(day::id!(async_data_refresh)),
                button(tr::async_data_cancel())
                    .enabled(move || data.loading())
                    .action(move || data.cancel())
                    .id(day::id!(async_data_cancel)),
                button(tr::async_data_source())
                    .action(move || source.update(|s| *s += 1))
                    .id(day::id!(async_data_source)),
            ))
            .spacing(Space::S),
            labeled(
                tr::async_data_fail(),
                toggle(fail).id(day::id!(async_data_fail)),
            ),
            load(data, |value| {
                // This field belongs to the content's scope: refetching must not reset it.
                let note = Signal::new(String::new());
                column((
                    label(move || {
                        value.with(|(source, request)| {
                            tr::async_data_value(tr::AsyncDataValue {
                                source: *source,
                                request: *request,
                            })
                            .format()
                        })
                    })
                    .id(day::id!(async_data_value)),
                    text_field(note)
                        .placeholder(tr::async_data_edit())
                        .id(day::id!(async_data_edit)),
                ))
                .spacing(Space::S)
            })
            .loading(|| label(tr::async_data_loading()).id(day::id!(async_data_loading)))
            .failed(|()| label(tr::async_data_failed()).id(day::id!(async_data_failed))),
            when(
                move || data.refreshing(),
                || label(tr::async_data_refreshing()).id(day::id!(async_data_refreshing)),
            ),
            when(
                move || data.has_value() && data.with_error(|e| e.is_some()),
                || label(tr::async_data_retained()).id(day::id!(async_data_retained)),
            ),
        ),
    )
}
