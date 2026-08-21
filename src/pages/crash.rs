use day::prelude::*;

use crate::widgets::page;

/// Crash Reporting — demonstrates day-break (docs/break.md). The buttons intentionally crash (or
/// trip day-core's panic containment); on the NEXT launch the saved report shows in the scrollable
/// viewer, and "Send report" opens a prefilled email to the developer. Nothing leaves the device
/// without the user's action.
///
/// The three crash flavors cover day-break's capture paths: a native `abort` (SIGABRT) and a
/// `segfault` (SIGSEGV) both die and are recorded by the signal handler; the "contained panic"
/// stays alive (day-core catches panics at its trampoline boundaries — see docs/break.md) and is
/// recorded as a NON-fatal report on the next launch.
/// One crash trigger: its explanation above, its button below and full width.
///
/// Not a `labeled` form row. These explanations are sentences, not field names, and in the label
/// column they took most of a phone's width and left the button squeezed into what remained — the
/// three buttons ended up three different sizes, each sized by how long its sentence happened to
/// be. Stacked, the sentence gets the full width to wrap into and every button is the same size as
/// every other, on a phone and on a desktop alike.
fn crash_action(explanation: String, action: impl Piece + 'static) -> impl Piece {
    column((
        label(explanation).font(Font::Footnote),
        AnyPiece::new(action).grow_w(),
    ))
    .spacing(6.0)
    .align(HAlign::Leading)
    .grow_w()
}

pub(crate) fn crash_page() -> AnyPiece {
    // The report viewer text, refreshed whenever the pending list changes (send/discard/relaunch).
    let report = Signal::new(String::new());
    let pending = day_break::pending();
    Effect::new(move || {
        pending.get(); // track
        report.set(day_break::latest_report_text().unwrap_or_default());
    });

    let crash_controls = section((
        crate::widgets::support_note(crate::support::crash_reporting()),
        // Each crash is scheduled ~150 ms out so the dayscript tap gets its reply before we die.
        crash_action(
            crate::res::str::crash_abort_label().format(),
            button(crate::res::str::crash_abort())
                .action(|| schedule(|| std::process::abort()))
                .tint(crate::widgets::danger())
                .id("crash-abort"),
        ),
        crash_action(
            crate::res::str::crash_segv_label().format(),
            button(crate::res::str::crash_segv())
                .action(|| schedule(segfault))
                .tint(crate::widgets::tinted(crate::palette::VIOLET))
                .id("crash-segv"),
        ),
        crash_action(
            crate::res::str::crash_contained_label().format(),
            // Panics in a button handler run inside day-core's event pump, which CONTAINS the
            // panic (the app survives); it becomes a non-fatal report on the next launch.
            // Pale amber, the mildest of the three — it is the one the app walks away from.
            button(crate::res::str::crash_contained())
                .action(|| panic!("intentional contained panic from the showcase crash page"))
                .tint(crate::widgets::tinted(crate::palette::AMBER))
                .id("crash-contained"),
        ),
    ))
    .title(crate::res::str::crash_trigger_section());

    // What "Send report" will do, disclosed to the user (from the configured reporter).
    let disclosure = day_break::reporter_description().unwrap_or_default();
    let has_disclosure = !disclosure.is_empty();

    let report_view = section((
        // The report shown ONCE, in a scrollable text view; an empty-state line when there is none.
        when(
            move || !report.get().is_empty(),
            move || {
                // Read-only, no spell-correction squiggles; still selectable so the report is
                // copyable.
                text_area(report)
                    .editable(false)
                    .spellcheck(false)
                    .min_lines(8)
                    .max_lines(20)
                    .id("crash-report")
                    .any()
            },
        ),
        when(
            move || report.get().is_empty(),
            move || {
                label(crate::res::str::crash_empty())
                    .id("crash-empty")
                    .any()
            },
        ),
        // Actions appear only when there is a report to act on: send it (opens an email), or clear.
        when(
            move || !report.get().is_empty(),
            move || {
                row((
                    button(crate::res::str::crash_send())
                        .action(send_newest)
                        .tint(crate::widgets::primary())
                        .id("crash-send"),
                    button(crate::res::str::crash_clear())
                        .action(clear_reports)
                        .tint(crate::widgets::secondary())
                        .id("crash-clear"),
                ))
                .spacing(8.0)
                .any()
            },
        ),
        when(
            move || has_disclosure && !report.get().is_empty(),
            move || {
                label(disclosure.clone())
                    .font(Font::Caption2)
                    .id("crash-disclosure")
                    .any()
            },
        ),
    ))
    .title(crate::res::str::crash_report_section());

    page(
        crate::res::str::nav_crash(),
        "crash-title",
        Some(crate::res::str::crash_caption()),
        form((crash_controls, report_view)).any(),
    )
    .any()
}

/// Run `crash` shortly after returning, so the caller (a button handler inside the event pump) can
/// finish and reply to the driving dayscript step before the process dies. A delayed main-loop
/// task is fine — the crash is process-wide (abort / fault) wherever it fires.
fn schedule(crash: fn()) {
    day::task(async move {
        day::sleep(150).await;
        crash();
    });
}

fn segfault() {
    // A null write the optimizer can't elide.
    unsafe {
        let p = std::hint::black_box(std::ptr::null_mut::<u8>());
        std::ptr::write_volatile(p, 1u8);
    }
}

/// Send the newest pending report through the configured reporter (here, an email compose). The
/// email app opening is the feedback; the report clears from the pending list once handed off.
fn send_newest() {
    if let Some(meta) = day_break::pending().get_untracked().into_iter().next() {
        day_break::send(&meta, |_result| {});
    }
}

fn clear_reports() {
    for meta in day_break::pending().get_untracked() {
        day_break::discard(&meta);
    }
}
