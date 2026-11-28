use day::datetime::{DayDate, DayTime, date_picker, time_picker};
use day::prelude::*;

use crate::gallery;
use crate::widgets::page_cards;

/// Native date & time pickers (day::datetime, core controls with
/// renderers on all seven toolkits; docs/datepicker.md), with stable ids for the walkthrough
/// (§14): a full-width embedded calendar above a form of the compact/bounded date stylings, the
/// time stylings (compact + seconds), and combined date+time as composition.
pub(crate) fn dates_page() -> AnyPiece {
    // One civil date signal behind the compact field, the ISO readout, and the full-width
    // calendar (docs/datepicker.md): each is a two-way projection of the same state, so driving
    // `date-inline` moves the same `date-value` readout (the walkthrough asserts this). Kept at
    // page scope.
    let date = Signal::new(seed_date());
    page_cards(
        crate::res::str::nav_dates(),
        day::id!(dates_title),
        gallery::balanced(
            2,
            vec![
                date_section(date).any(),
                time_section().any(),
                composed_section().any(),
            ],
        ),
    )
    .any()
}

/// The seed civil date. Seeded deterministically so the per-locale screenshot grid is
/// reproducible (the walkthrough drives changes via `input:` with ISO values).
fn seed_date() -> DayDate {
    DayDate::new(2026, 7, 18).unwrap_or_else(DayDate::today_local)
}

fn date_section(date: Signal<DayDate>) -> impl Piece {
    // A separately-bounded picker: picks outside 2026 clamp (natively where the control supports
    // min/max, and always in the piece).
    let bounded = Signal::new(seed_date());
    crate::gallery::panel(
        crate::res::str::dates_date_section(),
        (
            // The embedded calendar, unlabeled and as wide as the card: the native month grid needs
            // the whole width. Inside a labeled row the control column was too narrow and clipped
            // the grid's trailing edge on iOS/Android, so it sits above the rows.
            date_picker(date).inline().id(day::id!(date_inline)),
            // Compact: a field/button that summons the platform's transient chooser.
            labeled(
                crate::res::str::date_compact(),
                date_picker(date).id(day::id!(date_compact)),
            ),
            // Bounded: min/max clamp every pick into 2026 (readout beside it, the volume pattern).
            labeled(
                crate::res::str::date_bounded(),
                row((
                    within_2026(date_picker(bounded)).id(day::id!(date_bounded)),
                    label(move || bounded.get().to_string())
                        .tabular()
                        .id(day::id!(date_bounded_value)),
                ))
                .spacing(Space::S)
                // The readout is half the picker's height, so centering it would leave its digits
                // riding above the picker's; a control and the value it produces belong on one
                // line (docs/baseline.md).
                .align(VAlign::FirstBaseline),
            ),
            // Locale-independent ISO readout: what the walkthrough asserts on every backend.
            labeled(
                crate::res::str::date_picked(),
                label(move || date.get().to_string())
                    .tabular()
                    .id(day::id!(date_value)),
            ),
        ),
    )
}

/// `picker` held to the days of 2026.
fn within_2026(
    picker: day::datetime::DatePicker<Signal<DayDate>>,
) -> day::datetime::DatePicker<Signal<DayDate>> {
    match (DayDate::new(2026, 1, 1), DayDate::new(2026, 12, 31)) {
        (Some(first), Some(last)) => picker.min(first).max(last),
        _ => picker,
    }
}

fn time_section() -> impl Piece {
    // One time signal behind both time pickers: the seconds variant (honored natively on
    // AppKit/Qt; docs/datepicker.md) mirrors the compact one.
    let time = Signal::new(DayTime::new(9, 30, 0).unwrap_or_else(DayTime::now_local));
    crate::gallery::panel(
        crate::res::str::dates_time_section(),
        (
            labeled(
                crate::res::str::time_compact(),
                time_picker(time).id(day::id!(time_compact)),
            ),
            labeled(
                crate::res::str::time_seconds(),
                time_picker(time).seconds().id(day::id!(time_seconds)),
            ),
            labeled(
                crate::res::str::time_picked(),
                label(move || time.get().to_string())
                    .tabular()
                    .id(day::id!(time_value)),
            ),
        ),
    )
}

fn composed_section() -> impl Piece {
    // Combined date+time is composition (a single combined control exists on only 3 of the 7
    // toolkits, so Day doesn't paper over it; docs/datepicker.md).
    let date = Signal::new(seed_date());
    let time = Signal::new(DayTime::new(18, 0, 0).unwrap_or_else(DayTime::now_local));
    crate::gallery::panel(
        crate::res::str::dates_composed_section(),
        (labeled(
            crate::res::str::dates_composed(),
            row((date_picker(date), time_picker(time))).spacing(Space::S),
        ),),
    )
}
