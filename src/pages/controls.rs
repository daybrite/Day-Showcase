use day::prelude::*;
use day_piece_activity::activity;
use day_piece_colorpicker::color_picker;
use day_piece_combobox::combo_box;
use day_piece_datetime::{DayDate, DayTime, date_picker, time_picker};
use day_piece_rating::{badge, rating};
use day_piece_searchfield::search_field;
use day_piece_stepper::stepper;

use crate::widgets::{gauge, page};

/// Every control Day ships, on one page, family by family: the reference a reader scrolls
/// when they ask "does Day have a…". One `labeled` row per control, named after the control,
/// and every control bound to live state — the State section at the foot reads it all back,
/// which is what proves the bindings. The composition-tier pieces and the external control
/// crates (stepper, combo box, search field, date and time pickers, color picker, rating,
/// activity) sit beside the built-ins and are named at the foot of the page.
///
/// Where one value can feed several controls it does: the slider, the stepped slider, the
/// progress bar and the gauge share `level`, and a preset snaps it, so moving one moves the
/// rest — the one idea kept from the page's earlier life as a six-control mixer.
pub(crate) fn controls_page() -> AnyPiece {
    let st = Catalog::new();
    page(
        crate::res::str::nav_controls(),
        "controls-title",
        Some(crate::res::str::controls_caption()),
        form((
            buttons_section(st),
            switches_section(st),
            text_section(st),
            pickers_section(st),
            indicators_section(st),
            state_section(st),
        ))
        .any(),
    )
    .any()
}

/// The page's state. Copy, so every section takes it by value and closures stay cheap.
#[derive(Clone, Copy)]
struct Catalog {
    /// The master switch: dims and disables the controls that take a value.
    on: Signal<bool>,
    /// The number the slider, the progress bar and the gauge share, 0–100.
    level: Signal<f64>,
    /// The stepped slider's own value. Not `level`: a native scale with a step snaps any value
    /// written to it and writes the snapped one back, so sharing the signal would let this
    /// control round the plain slider's 42 down to 40 (GTK does exactly that).
    steps: Signal<f64>,
    /// The stepper's count.
    count: Signal<f64>,
    /// How many times any button was pressed.
    presses: Signal<u32>,
    /// The text field.
    name: Signal<String>,
    /// The search field's query, over the combo box's list.
    query: Signal<String>,
    /// The combo box's text — a value that may or may not be in the list.
    voice: Signal<String>,
    /// The text area.
    notes: Signal<String>,
    /// Which preset is selected, bound to all three picker stylings at once.
    preset: Signal<usize>,
    date: Signal<DayDate>,
    time: Signal<DayTime>,
    color: Signal<Color>,
    stars: Signal<usize>,
}

/// The level each preset snaps to. Presets reuse the existing size_* strings, so the three picker
/// stylings stay localized without inventing a parallel vocabulary.
const PRESET_LEVELS: [f64; 3] = [25.0, 60.0, 90.0];

impl Catalog {
    fn new() -> Self {
        let st = Catalog {
            on: Signal::new(true),
            level: Signal::new(60.0),
            steps: Signal::new(60.0),
            count: Signal::new(3.0),
            presses: Signal::new(0),
            name: Signal::new(String::new()),
            query: Signal::new(String::new()),
            voice: Signal::new(String::new()),
            notes: Signal::new(String::new()),
            preset: Signal::new(1usize),
            // Fixed seeds, so the walkthrough's screenshots reproduce.
            date: Signal::new(DayDate::new(2026, 9, 1).unwrap_or_else(DayDate::today)),
            time: Signal::new(DayTime::new(9, 30, 0).unwrap_or_else(DayTime::now)),
            color: Signal::new(crate::palette::SKY),
            stars: Signal::new(3usize),
        };
        // preset → level. `watch` rather than a binding: the arrow runs one way, so dragging the
        // slider off a preset leaves the preset alone (the readout then says "Custom") instead of
        // fighting the user for the signal.
        watch(
            move || st.preset.get(),
            move |idx, _| st.level.set(PRESET_LEVELS[(*idx).min(2)]),
        );
        // On the web a reload is part of normal life, so the switch and the level survive it
        // (docs/web.md). Native launches start fresh on purpose, and the walkthrough asserts that.
        #[cfg(target_arch = "wasm32")]
        {
            day::prefs::bind("controls.level", st.level);
            day::prefs::bind("controls.preset", st.preset);
            day::prefs::bind("controls.on", st.on);
        }
        st
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
    /// one, "Custom" the moment a slider or stepper moves it off.
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

    fn press(self) {
        self.presses.update(|p| *p += 1);
    }

    /// Everything below the master switch dims when it is off — the one visual cue that the
    /// toggle governs the controls that take a value.
    fn dim(self) -> impl Fn() -> f64 {
        move || if self.on.get() { 1.0 } else { 0.45 }
    }
}

/// The button in each of its styles. Every one counts a press, so the readout beside them
/// proves each fired — the styles are presentation, the action is one.
fn buttons_section(st: Catalog) -> impl Piece {
    let press = crate::res::str::ctl_press;
    section((
        labeled(
            crate::res::str::ctl_plain(),
            button(press()).action(move || st.press()).id("btn-plain"),
        ),
        labeled(
            crate::res::str::ctl_bordered(),
            button(press())
                .bordered()
                .action(move || st.press())
                .id("btn-bordered"),
        ),
        labeled(
            crate::res::str::ctl_prominent(),
            button(press())
                .prominent()
                .action(move || st.press())
                .id("btn-prominent"),
        ),
        labeled(
            crate::res::str::ctl_tinted(),
            button(press())
                .tint(crate::widgets::primary())
                .action(move || st.press())
                .id("btn-tinted"),
        ),
        labeled(
            crate::res::str::ctl_destructive(),
            button(press())
                .tint(crate::widgets::danger())
                .action(move || st.press())
                .id("btn-destructive"),
        ),
        labeled(
            crate::res::str::ctl_disabled(),
            button(press())
                .enabled(false)
                .action(move || st.press())
                .id("btn-disabled"),
        ),
        labeled(
            crate::res::str::ctl_link(),
            link(crate::res::str::ctl_link_text(), "https://daybrite.dev").id("ctl-link"),
        ),
        labeled(
            crate::res::str::ctl_presses(),
            crate::widgets::numeric_readout(
                move || st.presses.get().to_string(),
                "888",
                "btn-presses",
            ),
        ),
    ))
    .title(crate::res::str::ctl_buttons())
}

/// The value controls, most of them over ONE number: the slider writes `level`, the progress
/// bar reports it, and the switch above gates the lot; the stepped slider keeps a value of its
/// own (see `Catalog::steps`).
/// The stepper (day-piece-stepper: an NSStepper field, a GtkSpinButton, a QDoubleSpinBox, and
/// a composed field elsewhere) keeps its own count, shown twice — once native where the
/// toolkit has one, once composed — so the two idioms sit side by side.
fn switches_section(st: Catalog) -> impl Piece {
    let dim = st.dim();
    section((
        labeled(
            crate::res::str::ctl_toggle(),
            toggle(st.on).id("subscribe-toggle"),
        ),
        column((
            labeled(
                crate::res::str::ctl_slider(),
                row((
                    slider(st.level).range(0.0..=100.0).id("volume-slider"),
                    // Reserves the width of "100" so the row stops reflowing as the value
                    // changes under the slider being dragged.
                    crate::widgets::numeric_readout(
                        move || format!("{:.0}", st.level.get()),
                        "100",
                        "volume-value",
                    ),
                ))
                .spacing(8.0),
            ),
            labeled(
                crate::res::str::ctl_stepped(),
                row((
                    slider(st.steps)
                        .range(0.0..=100.0)
                        .step(10.0)
                        .id("stepped-slider"),
                    crate::widgets::numeric_readout(
                        move || format!("{:.0}", st.steps.get()),
                        "100",
                        "stepped-value",
                    ),
                ))
                .spacing(8.0),
            ),
            labeled(
                crate::res::str::ctl_stepper(),
                row((
                    stepper(st.count)
                        .range(0.0..=10.0)
                        .step(1.0)
                        .decimals(0)
                        .id("stepper-field"),
                    crate::widgets::numeric_readout(
                        move || format!("{:.0}", st.count.get()),
                        "88",
                        "stepper-value",
                    ),
                ))
                .spacing(8.0),
            ),
            labeled(
                crate::res::str::ctl_composed_stepper(),
                stepper(st.count)
                    .range(0.0..=10.0)
                    .step(1.0)
                    .decimals(0)
                    .composed()
                    .key("stepper-composed"),
            ),
            labeled(
                crate::res::str::ctl_progress(),
                progress(move || st.level.get() / 100.0)
                    .id("volume-progress")
                    .a11y(|a| a.role(Role::Meter).label("Level")),
            ),
            labeled(
                crate::res::str::ctl_activity(),
                row((
                    activity()
                        .animating(move || st.on.get() && st.level.get() > 0.0)
                        .id("activity-spinner"),
                    label(move || {
                        if st.on.get() {
                            crate::res::str::activity_on()
                        } else {
                            crate::res::str::activity_off()
                        }
                        .format()
                    })
                    .id("activity-status"),
                ))
                .spacing(8.0)
                .fit(RowFit::Wrap { run_spacing: 8.0 }),
            ),
            labeled(crate::res::str::ctl_spinner(), spinner().id("ctl-spinner")),
        ))
        .spacing(8.0)
        .opacity(dim),
    ))
    .title(crate::res::str::ctl_switches())
}

/// Text entry: the field, the search field over the combo box's list, the combo box itself
/// (day-piece-combobox; a placeholder where the toolkit has no such control), and the plain
/// text area.
fn text_section(st: Catalog) -> impl Piece {
    let voices = Signal::new(vec![
        crate::res::str::vanilla().format(),
        crate::res::str::chocolate().format(),
        crate::res::str::pistachio().format(),
    ]);
    section((
        labeled(
            crate::res::str::ctl_text_field(),
            text_field(st.name)
                .placeholder(crate::res::str::name_placeholder())
                .id("name-field"),
        ),
        labeled(
            crate::res::str::ctl_search_field(),
            column((
                row((
                    search_field(st.query)
                        .placeholder(crate::res::str::voice_search_placeholder())
                        .id("search-input"),
                    button(crate::res::str::search_clear())
                        .bordered()
                        .action(move || st.query.set(String::new()))
                        .id("search-clear"),
                ))
                .spacing(8.0),
                // The first match in the SAME list the combo offers — a value, or an em-dash.
                label(move || first_match(&st.query.get(), &voices.get()))
                    .font(Font::Footnote)
                    .id("search-result"),
            ))
            .spacing(4.0)
            .align(HAlign::Leading),
        ),
        labeled(
            crate::res::str::ctl_combo_box(),
            // The bound value reads BELOW the field, not at the end of the row: on the same row
            // it grew with every keystroke and pushed Add sideways as you typed.
            column((
                row((
                    combo_box(voices, st.voice)
                        .placeholder(crate::res::str::flavor_placeholder())
                        .id("flavor-combo"),
                    button(crate::res::str::flavor_add())
                        .action(move || {
                            let typed = st.voice.get_untracked();
                            if !typed.is_empty() && !voices.get_untracked().contains(&typed) {
                                voices.update(|v| v.push(typed));
                            }
                        })
                        .tint(crate::widgets::primary())
                        .id("flavor-add"),
                ))
                .spacing(8.0),
                // No combo-box arm on iOS, HarmonyOS or the web (docs/combobox.md): day renders
                // its placeholder leaf in the row above, and this note sits right beside it.
                // `when` rather than `#[cfg]` because an attribute cannot gate one tuple element.
                when(
                    || {
                        cfg!(any(
                            target_os = "ios",
                            target_env = "ohos",
                            target_arch = "wasm32"
                        ))
                    },
                    || label(crate::res::str::ctl_combo_note()).font(Font::Footnote),
                ),
                label(move || st.voice.get())
                    .font(Font::Footnote)
                    .id("flavor-value"),
            ))
            .spacing(4.0)
            .align(HAlign::Leading),
        ),
        labeled(
            crate::res::str::ctl_text_area(),
            text_area(st.notes)
                .placeholder(crate::res::str::ctl_notes_placeholder())
                .min_lines(3)
                .max_lines(3)
                .id("notes-area"),
        ),
    ))
    .title(crate::res::str::ctl_text_entry())
}

/// Pickers: the built-in picker in its three stylings over ONE selection (docs/picker.md),
/// then the date, time and color pickers from their crates. The date and time pickers appear
/// here compact; the inline calendar has the Date & time page to itself.
fn pickers_section(st: Catalog) -> impl Piece {
    let names = Catalog::preset_names();
    let preset_label = st.preset_label();
    let dim = st.dim();
    section((
        column((
            labeled(
                crate::res::str::picker_segmented(),
                picker(names.iter().cloned(), st.preset)
                    .segmented()
                    .id("picker-segmented"),
            ),
            labeled(
                crate::res::str::picker_menu(),
                picker(names.iter().cloned(), st.preset)
                    .menu()
                    .id("picker-menu"),
            ),
            labeled(
                crate::res::str::picker_inline(),
                picker(names.iter().cloned(), st.preset)
                    .inline()
                    .id("picker-inline"),
            ),
            labeled(
                crate::res::str::picker_selected(),
                label(preset_label).id("picker-value"),
            ),
        ))
        .spacing(8.0)
        .opacity(dim),
        labeled(
            crate::res::str::ctl_date(),
            date_picker(st.date).compact().id("ctl-date"),
        ),
        labeled(
            crate::res::str::ctl_time(),
            time_picker(st.time).compact().id("ctl-time"),
        ),
        // Native chooser or a composed one per toolkit (docs/colorpicker.md) — never a banner:
        // there is no target where it does not work, only which picker differs.
        labeled(
            crate::res::str::ctl_color(),
            color_picker(st.color).id("ctl-color"),
        ),
    ))
    .title(crate::res::str::ctl_pickers())
}

/// Indicators and the composition tier: the star rating (canvas polygons), a badge overlay, the
/// arc gauge over the shared level, and the divider.
fn indicators_section(st: Catalog) -> impl Piece {
    section((
        labeled(
            crate::res::str::ctl_rating(),
            row((
                rating(st.stars).id("compose-rating"),
                crate::widgets::numeric_readout(
                    move || st.stars.get().to_string(),
                    "5",
                    "rating-value",
                ),
            ))
            .spacing(8.0),
        ),
        labeled(
            crate::res::str::ctl_badge(),
            badge(
                3,
                rounded_rectangle(10.0)
                    .fill(crate::palette::SLATE)
                    .frame(48.0, 48.0)
                    .any(),
            )
            .id("ctl-badge"),
        ),
        labeled(
            crate::res::str::ctl_gauge(),
            gauge(st.level).frame(120.0, 120.0),
        ),
        labeled(crate::res::str::ctl_divider(), divider().id("ctl-divider")),
    ))
    .title(crate::res::str::ctl_indicators())
}

/// Every bound value, read back. Scrolling here is the walkthrough's second screenshot, and the
/// row values are what its assertions check after driving the controls above.
fn state_section(st: Catalog) -> impl Piece {
    let readout =
        |text: fn() -> LocalizedText, value: Box<dyn Fn() -> String>, id: &'static str| {
            labeled(text(), label(value).tabular().id(id))
        };
    section((
        readout(
            crate::res::str::ctl_toggle,
            Box::new(move || {
                if st.on.get() {
                    crate::res::str::ctl_on()
                } else {
                    crate::res::str::ctl_off()
                }
                .format()
            }),
            "state-toggle",
        ),
        readout(
            crate::res::str::ctl_slider,
            Box::new(move || format!("{:.0}", st.level.get())),
            "state-level",
        ),
        readout(
            crate::res::str::ctl_stepper,
            Box::new(move || format!("{:.0}", st.count.get())),
            "state-count",
        ),
        readout(
            crate::res::str::ctl_text_field,
            Box::new(move || st.name.get()),
            "state-name",
        ),
        readout(
            crate::res::str::ctl_date,
            Box::new(move || st.date.get().to_string()),
            "state-date",
        ),
        readout(
            crate::res::str::ctl_time,
            Box::new(move || st.time.get().to_string()),
            "state-time",
        ),
        readout(
            crate::res::str::ctl_color,
            Box::new(move || st.color.get().to_hex_string()),
            "state-color",
        ),
        readout(
            crate::res::str::ctl_rating,
            Box::new(move || st.stars.get().to_string()),
            "state-rating",
        ),
        readout(
            crate::res::str::ctl_presses,
            Box::new(move || st.presses.get().to_string()),
            "state-presses",
        ),
        label(crate::res::str::ctl_crates_note()).font(Font::Footnote),
    ))
    .title(crate::res::str::ctl_state())
}

/// Case-insensitive substring match; an empty query matches everything.
fn matches(query: &str, item: &str) -> bool {
    query.is_empty() || item.to_lowercase().contains(&query.to_lowercase())
}

/// The first item matching `query` (a data value), or an em-dash when none match.
fn first_match(query: &str, items: &[String]) -> String {
    items
        .iter()
        .find(|i| matches(query, i))
        .cloned()
        .unwrap_or_else(|| "\u{2014}".to_string())
}
