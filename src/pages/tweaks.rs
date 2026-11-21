use day::prelude::*;
use day_tweak_button_bezel::{Bezel, ButtonBezelTweak};
use day_tweak_slider_tick_marks::{SliderTickmarksTweak, TickPosition, Tickmarks};
use day_tweak_tooltip::TooltipTweak;

// Tweaks (docs/tweaks.md): packaged per-toolkit configuration of built-in pieces. Each card shows
// a stock piece beside its Tweaked Piece; the note names the toolkits the tweak affects; on
// every other toolkit the tweak is a documented no-op and the two sides look identical.
pub(crate) fn tweaks_page() -> AnyPiece {
    let free = Signal::new(50.0f64);
    let snapped = Signal::new(50.0f64);
    let ref_mounted = Signal::new(true);
    let slider_ref = NativeRef::new();

    // day-tweak-button-bezel: the trivial single-toolkit tweak (AppKit bezel constants).
    let bezel_card = crate::gallery::group(
        crate::res::str::tweaks_bezel_title(),
        column((
            crate::gallery::note(crate::res::str::tweaks_bezel_caption()),
            row((
                button(crate::res::str::tweaks_stock()).id(day::id!(tweak_bezel_stock)),
                button(crate::res::str::tweaks_tweaked())
                    .bezel(Bezel::Toolbar)
                    .id(day::id!(tweak_bezel_toolbar)),
                button(crate::res::str::tweaks_tweaked())
                    .bezel(Bezel::Badge)
                    .id(day::id!(tweak_bezel_badge)),
            ))
            .spacing(Space::S)
            .align(VAlign::Center)
            .fit(RowFit::Wrap { run_spacing: 8.0 }),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    );

    // day-tweak-tooltip: one modifier across three access tiers (objc2 / gtk4-rs / JNI). Hover the
    // button (macOS/GTK) or long-press it (Android) to see the native tooltip.
    let tooltip_card = crate::gallery::group(
        crate::res::str::tweaks_tooltip_title(),
        column((
            crate::gallery::note(crate::res::str::tweaks_tooltip_caption()),
            button(crate::res::str::tweaks_tooltip_label())
                .tooltip(crate::res::str::tweaks_tooltip_hint().format())
                .id(day::id!(tweak_tooltip_button)),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    );

    // day-tweak-slider-tick-marks: the full-range tweak: six toolkits, incl. its own Qt/XAML/
    // ArkUI native code. The tweaked slider snaps to its marks where the platform supports it.
    let ticks_card = crate::gallery::group(
        crate::res::str::tweaks_ticks_title(),
        column((
            crate::gallery::note(crate::res::str::tweaks_ticks_caption()),
            row((
                label(crate::res::str::tweaks_stock()).font(Font::Caption),
                slider(free)
                    .range(0.0..=100.0)
                    .id(day::id!(tweak_ticks_stock)),
                label(move || format!("{:.0}", free.get()))
                    .tabular()
                    .id(day::id!(tweak_ticks_stock_value)),
            ))
            .spacing(Space::S),
            row((
                label(crate::res::str::tweaks_tweaked()).font(Font::Caption),
                slider(snapped)
                    .range(0.0..=100.0)
                    .tickmarks(
                        Tickmarks::count(11)
                            .snap(true)
                            .position(TickPosition::Below),
                    )
                    .id(day::id!(tweak_ticks_slider)),
                label(move || format!("{:.0}", snapped.get()))
                    .tabular()
                    .id(day::id!(tweak_ticks_value)),
            ))
            .spacing(Space::S),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    );

    // NativeRef: imperative access with liveness; unmount the Tweaked Piece and the ref clears.
    let ref_card = crate::gallery::group(
        crate::res::str::tweaks_ref_title(),
        column((
            row((
                label(crate::res::str::tweaks_ref_caption()).font(Font::Caption),
                toggle(ref_mounted)
                    .id(day::id!(tweak_ref_toggle))
                    .a11y(|a| a.label(crate::res::str::tweaks_ref_caption())),
            ))
            .spacing(Space::S),
            when(move || ref_mounted.get(), {
                let r = slider_ref.clone();
                // On the tick grid (count 5 over 0..=100 → step 25): a stepped Material slider
                // requires values on the grid, and programmatic snapping doesn't write back.
                let v = Signal::new(25.0f64);
                move || {
                    slider(v)
                        .range(0.0..=100.0)
                        .tickmarks(Tickmarks::count(5))
                        .native_ref(&r)
                        .id(day::id!(tweak_ref_slider))
                }
            }),
            label({
                let r = slider_ref.clone();
                // NativeRef reads are tracked: this re-runs on the ref's mount/clear transitions.
                move || {
                    if r.node().is_some() {
                        crate::res::str::tweaks_ref_live().format()
                    } else {
                        crate::res::str::tweaks_ref_cleared().format()
                    }
                }
            })
            .id(day::id!(tweak_ref_status)),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    );

    // Inline `.tweak(appkit(…))`/`.tweak(uikit(…))` on a text label with a conditional backing
    // (docs/tweaks.md "Conditional backings"): with Selectable on, iOS realizes the label as a
    // read-only UITextView instead of a UILabel (docs/text.md). The tweak stays correct because
    // it sits after `.selectable()` in the chain and pokes view-level surface (alpha), which
    // lands on either class; the caption line reports the concrete class the tweak saw. Flip
    // the toggle on iOS and watch it change. AppKit keeps NSTextField for both states; every
    // other toolkit shows the stock label and an em-dash class.
    let label_sel = Signal::new(false);
    let seen_class = Signal::new(String::from("\u{2014}"));
    let label_card = crate::gallery::group(
        crate::res::str::tweaks_label_title(),
        column((
            crate::gallery::note(crate::res::str::tweaks_label_caption()),
            row((
                label(crate::res::str::text_selectable_toggle()).font(Font::Caption),
                toggle(label_sel).id(day::id!(tweak_label_selectable)),
            ))
            .spacing(Space::S),
            when(
                move || label_sel.get(),
                move || tweaked_label(true, seen_class),
            ),
            when(
                move || !label_sel.get(),
                move || tweaked_label(false, seen_class),
            ),
            label(move || crate::res::str::tweaks_label_class(seen_class.get()).format())
                .font(Font::Caption)
                .id(day::id!(tweak_label_class)),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    );

    crate::widgets::page_cards(
        crate::res::str::nav_tweaks(),
        day::id!(tweaks_title),
        crate::gallery::balanced(
            2,
            vec![bezel_card, tooltip_card, ticks_card, ref_card, label_card],
        ),
    )
    .any()
}

/// The label-card sample for one Selectable state: `.selectable()` first, since it may rebuild
/// the backing (UIKit), then the tweak, so it runs against the widget that ships and reports
/// that widget's class (docs/tweaks.md "Conditional backings").
fn tweaked_label(selectable: bool, seen: Signal<String>) -> impl Piece {
    let l = label(crate::res::str::tweaks_label_sample()).id(day::id!(tweak_label_sample));
    let l = if selectable { l.selectable() } else { l };
    #[cfg(feature = "appkit")]
    let l = {
        use day_toolkit_appkit::appkit;
        l.tweak(appkit(move |view, class, _mtm| {
            seen.set(class.to_string());
            // View-level surface, because alpha lands identically on any backing class.
            view.setAlphaValue(0.55);
        }))
    };
    #[cfg(feature = "uikit")]
    let l = {
        use day_toolkit_uikit::uikit;
        l.tweak(uikit(move |view, class, _mtm| {
            seen.set(class.to_string());
            view.setAlpha(0.55);
        }))
    };
    // Toolkits without the inline arm keep the stock label; the class line stays an em dash.
    #[cfg(not(any(feature = "appkit", feature = "uikit")))]
    let _ = seen;
    l
}
