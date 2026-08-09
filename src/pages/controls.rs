use day::prelude::*;
use day_piece_activity::activity;
use day_piece_combobox::combo_box;
use day_piece_searchfield::search_field;

use crate::widgets::{gauge, page};

/// Every core control on one page, all of them wired to ONE small piece of shared state.
///
/// The page is a tiny mixer. `level` is the number the whole page is about: a slider sets it, a
/// pair of stepper buttons nudges it, a preset picker snaps it, and a progress bar, a gauge, and a
/// readout all report it — six controls over a single `Signal<f64>`, which is the point. `preset`
/// is bound to three native picker stylings at once, so moving any one moves the other two. `on`
/// gates the lot.
///
/// The earlier version of this page was eight unrelated demos, each with a private signal: it
/// showed what the controls look like but never what binding them to the same state does. Nothing
/// here is a mock — every value round-trips through the native widget.
pub(crate) fn controls_page() -> AnyPiece {
    let mix = Mix::new();
    page(
        crate::res::str::nav_controls(),
        "controls-title",
        None,
        form((mix_section(mix), voice_section(mix))).any(),
    )
}

/// The page's shared state. Copy, so every section takes it by value and closures stay cheap.
#[derive(Clone, Copy)]
struct Mix {
    /// What the mix is called. A text field edits it; the summary line reads it.
    name: Signal<String>,
    /// The number the page is about, 0–100. Written by the slider, the steppers, and the presets;
    /// read by the progress bar, the gauge, the readout, and the summary.
    level: Signal<f64>,
    /// Which preset is selected, bound to all three picker stylings at once.
    preset: Signal<usize>,
    /// The master switch. Off dims every editor and stops the activity indicator.
    on: Signal<bool>,
    /// The combo box's text — a value that may or may not be in the list.
    voice: Signal<String>,
}

/// The level each preset snaps to. Presets reuse the existing size_* strings, so the three picker
/// stylings stay localized without inventing a parallel vocabulary.
const PRESET_LEVELS: [f64; 3] = [25.0, 60.0, 90.0];

impl Mix {
    fn new() -> Self {
        let mix = Mix {
            name: Signal::new(String::new()),
            level: Signal::new(60.0),
            preset: Signal::new(1usize),
            on: Signal::new(true),
            voice: Signal::new(String::new()),
        };
        // preset → level. `watch` rather than a binding: the arrow runs one way, so dragging the
        // slider off a preset leaves the preset alone (the readout then says "Custom") instead of
        // fighting the user for the signal.
        watch(
            move || mix.preset.get(),
            move |idx, _| mix.level.set(PRESET_LEVELS[(*idx).min(2)]),
        );
        // On the web a reload is part of normal life, so the mix survives it (docs/web.md).
        // Native launches start fresh on purpose, and the walkthrough asserts that.
        #[cfg(target_arch = "wasm32")]
        {
            day::prefs::bind("controls.name", mix.name);
            day::prefs::bind("controls.level", mix.level);
            day::prefs::bind("controls.preset", mix.preset);
            day::prefs::bind("controls.on", mix.on);
        }
        mix
    }

    /// The localized preset names, resolved once (the locale is fixed for a run).
    fn preset_names() -> std::rc::Rc<Vec<String>> {
        std::rc::Rc::new(vec![
            crate::res::str::size_small().format(),
            crate::res::str::size_medium().format(),
            crate::res::str::size_large().format(),
        ])
    }

    /// What the level currently *is*, in preset terms: the preset's name when it sits exactly on
    /// one, "Custom" the moment a slider or stepper moves it off. This is the derivation that
    /// makes the shared state visible — two signals, one sentence.
    fn preset_label(self) -> impl Fn() -> String {
        let names = Self::preset_names();
        move || {
            let level = self.level.get();
            match PRESET_LEVELS.iter().position(|p| (p - level).abs() < 0.5) {
                Some(i) => names[i].clone(),
                None => crate::res::str::mix_custom().format(),
            }
        }
    }

    /// Nudge the level, clamped. Shared by both stepper buttons.
    fn nudge(self, delta: f64) {
        self.level.update(|v| *v = (*v + delta).clamp(0.0, 100.0));
    }
}

/// The mix itself: the summary, the gauge, and every editor that writes `level`.
fn mix_section(mix: Mix) -> impl Piece {
    let preset_label = mix.preset_label();
    // One line that reads all of the shared state at once, so a change anywhere is visible here.
    let summary = {
        let preset_label = mix.preset_label();
        move || {
            let name = mix.name.with(|n| {
                if n.is_empty() {
                    crate::res::str::mix_untitled().format()
                } else {
                    n.clone()
                }
            });
            // The generated accessor takes the message's variables alphabetically (level, name,
            // preset), NOT in the order they appear in the sentence.
            crate::res::str::mix_summary(format!("{:.0}", mix.level.get()), name, preset_label())
                .format()
        }
    };
    // Everything below the switch dims when the mix is off — the one visual cue that the master
    // toggle governs the rest of the page.
    let dim = move || if mix.on.get() { 1.0 } else { 0.45 };

    section((
        label(summary)
            .font(Font::Headline)
            .tabular()
            .id("mix-summary"),
        row((
            gauge(mix.level).frame(120.0, 120.0),
            column((
                labeled(
                    crate::res::str::subscribe_label(),
                    toggle(mix.on).id("subscribe-toggle"),
                ),
                labeled(
                    crate::res::str::picker_selected(),
                    label(preset_label).id("picker-value"),
                ),
                labeled(
                    crate::res::str::progress_label(),
                    progress(move || mix.level.get() / 100.0)
                        .id("volume-progress")
                        .a11y(|a| a.role(Role::Meter).label("Mix level")),
                ),
                labeled(
                    crate::res::str::activity_animating(),
                    row((
                        activity()
                            .animating(move || mix.on.get() && mix.level.get() > 0.0)
                            .id("activity-spinner"),
                        label(move || {
                            if mix.on.get() {
                                crate::res::str::activity_on()
                            } else {
                                crate::res::str::activity_off()
                            }
                            .format()
                        })
                        .id("activity-status"),
                    ))
                    .spacing(8.0),
                ),
            ))
            .spacing(8.0)
            .align(HAlign::Leading)
            .grow_w()
            .opacity(dim),
        ))
        .spacing(16.0),
        // — the editors, all writing the same `level` —
        column((
            text_field(mix.name)
                .placeholder(crate::res::str::name_placeholder())
                .id("name-field"),
            labeled(
                crate::res::str::value_label(),
                row((
                    // The two steppers are the same control pointing opposite ways, so they take
                    // the same styling: tinting only one of them made it read as the primary
                    // action of the row, which is not true — neither is.
                    button(crate::res::str::decrement())
                        .enabled(move || mix.on.get())
                        .action(move || mix.nudge(-5.0))
                        .id("decrement-button"),
                    slider(mix.level).range(0.0..=100.0).id("volume-slider"),
                    button(crate::res::str::increment())
                        .enabled(move || mix.on.get())
                        .action(move || mix.nudge(5.0))
                        .id("increment-button"),
                    // Reserves the width of "100" so the row stops reflowing as the value
                    // changes under the slider being dragged.
                    crate::widgets::numeric_readout(
                        move || format!("{:.0}", mix.level.get()),
                        "100",
                        "volume-value",
                    ),
                ))
                .spacing(8.0),
            ),
        ))
        .spacing(8.0)
        .opacity(dim),
        // The same `preset` signal in three native stylings (docs/picker.md), directly under the
        // slider that shares its state: selecting in any one moves the other two, and — through
        // the watch in `Mix::new` — snaps the level, which the slider, gauge and progress bar
        // report at once. They sat in a section of their own with a sentence explaining that
        // relationship; put under the slider they demonstrate it instead.
        picker_rows(mix).opacity(dim),
    ))
    .title(crate::res::str::controls_basics())
}

/// The three picker stylings, all bound to `preset`.
fn picker_rows(mix: Mix) -> impl Piece {
    let names = Mix::preset_names();
    column((
        labeled(
            crate::res::str::picker_segmented(),
            picker(names.iter().cloned(), mix.preset)
                .segmented()
                .id("picker-segmented"),
        ),
        labeled(
            crate::res::str::picker_menu(),
            picker(names.iter().cloned(), mix.preset)
                .menu()
                .id("picker-menu"),
        ),
        labeled(
            crate::res::str::picker_inline(),
            picker(names.iter().cloned(), mix.preset)
                .inline()
                .id("picker-inline"),
        ),
    ))
    .spacing(8.0)
}

/// The combo box and the search field over ONE list: search filters what the rows show, the combo
/// selects from the same collection, and Add grows it for both.
fn voice_section(mix: Mix) -> impl Piece {
    let voices = Signal::new(vec![
        crate::res::str::vanilla().format(),
        crate::res::str::chocolate().format(),
        crate::res::str::pistachio().format(),
    ]);
    let query = Signal::new(String::new());
    let block = section((
        labeled(
            crate::res::str::flavor_label(),
            // The bound value reads BELOW the field, not at the end of the row. On the same row it
            // grew with every keystroke and pushed Add sideways as you typed — the control you are
            // aiming at moving while you type is a bad enough jolt to be worth a whole row.
            column((
                row((
                    combo_box(voices, mix.voice)
                        .placeholder(crate::res::str::flavor_placeholder())
                        .id("flavor-combo"),
                    button(crate::res::str::flavor_add())
                        .action(move || {
                            let typed = mix.voice.get_untracked();
                            if !typed.is_empty() && !voices.get_untracked().contains(&typed) {
                                voices.update(|v| v.push(typed));
                            }
                        })
                        .style(crate::widgets::primary())
                        .id("flavor-add"),
                ))
                .spacing(8.0),
                label(move || mix.voice.get())
                    .font(Font::Footnote)
                    .id("flavor-value"),
            ))
            .spacing(4.0)
            .align(HAlign::Leading),
        ),
        row((
            search_field(query)
                .placeholder(crate::res::str::voice_search_placeholder())
                .id("search-input"),
            button(crate::res::str::search_clear())
                .bordered()
                .action(move || query.set(String::new()))
                .id("search-clear"),
        ))
        .spacing(8.0),
        // The first match in the SAME list the combo offers — a value, or an em-dash.
        label(move || first_match(&query.get(), &voices.get())).id("search-result"),
        each(
            move || filtered(&query.get(), &voices.get()),
            |v: &String| v.clone(),
            move |slot: ItemSlot<String, String>| label(slot.field(|v| v.clone())),
        ),
    ))
    .title(crate::res::str::nav_search());
    // iOS has no native combo-box control and the piece carries no uikit renderer
    // (docs/combobox.md) — day renders its placeholder leaf, and this note says why.
    #[cfg(target_os = "ios")]
    let block = column((
        block.any(),
        label(crate::res::str::flavor_ios_note()).font(Font::Footnote),
    ))
    .spacing(4.0);
    block
}

/// Case-insensitive substring match; an empty query matches everything.
fn matches(query: &str, item: &str) -> bool {
    query.is_empty() || item.to_lowercase().contains(&query.to_lowercase())
}

fn filtered(query: &str, items: &[String]) -> Vec<String> {
    items
        .iter()
        .filter(|i| matches(query, i))
        .cloned()
        .collect()
}

/// The first item matching `query` (a data value), or an em-dash when none match.
fn first_match(query: &str, items: &[String]) -> String {
    filtered(query, items)
        .into_iter()
        .next()
        .unwrap_or_else(|| "\u{2014}".to_string())
}
