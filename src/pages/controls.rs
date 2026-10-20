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
/// and every control bound to live state; the State section at the foot reads it all back,
/// which is what proves the bindings. The composition-tier pieces and the external control
/// crates (stepper, combo box, search field, date and time pickers, color picker, rating,
/// activity) sit beside the built-ins and are named at the foot of the page.
///
/// Where one value can feed several controls it does: the slider, the stepped slider, the
/// progress bar and the gauge share `level`, and a preset snaps it, so moving one moves the
/// rest, the one idea kept from the page's earlier life as a six-control mixer.
pub(crate) fn controls_page() -> AnyPiece {
    let st = Catalog::new();
    page(
        crate::res::str::nav_controls(),
        day::id!(controls_title),
        form((
            buttons_section(st),
            switches_section(st),
            text_section(st),
            pickers_section(st),
            indicators_section(st),
            accessibility_section(),
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
    /// The combo box's text: a value that may or may not be in the list.
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
        // (docs/web.md). Native launches start fresh, and the walkthrough asserts that.
        #[cfg(target_arch = "wasm32")]
        {
            day::prefs::bind("controls.level", st.level);
            day::prefs::bind("controls.preset", st.preset);
            day::prefs::bind("controls.on", st.on);
        }
        st
    }

    /// The presets as the three picker stylings list them (docs/picker.md): the index into
    /// `PRESET_LEVELS` and its localized name, live, so the titles follow a language switch.
    fn preset_options() -> [(usize, LocalizedText); 3] {
        [
            (0, crate::res::str::size_small()),
            (1, crate::res::str::size_medium()),
            (2, crate::res::str::size_large()),
        ]
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

    /// Everything below the master switch dims when it is off, the one visual cue that the
    /// toggle governs the controls that take a value.
    fn dim(self) -> impl Fn() -> f64 {
        move || if self.on.get() { 1.0 } else { 0.45 }
    }
}

/// The button in each of its styles. Every one counts a press, so the readout beside them
/// proves each fired: the styles are presentation, the action is one.
fn buttons_section(st: Catalog) -> impl Piece {
    let press = crate::res::str::ctl_press;
    let playing = Signal::new(false);
    section((
        labeled(
            crate::res::str::ctl_plain(),
            button(press())
                .action(move || st.press())
                .id(day::id!(btn_plain)),
        ),
        labeled(
            crate::res::str::ctl_bordered(),
            button(press())
                .bordered()
                .action(move || st.press())
                .id(day::id!(btn_bordered)),
        ),
        labeled(
            crate::res::str::ctl_prominent(),
            button(press())
                .prominent()
                .action(move || st.press())
                .id(day::id!(btn_prominent)),
        ),
        labeled(
            crate::res::str::ctl_tinted(),
            button(press())
                .tint(crate::widgets::primary())
                .action(move || st.press())
                .id(day::id!(btn_tinted)),
        ),
        labeled(
            crate::res::str::ctl_destructive(),
            button(press())
                .tint(crate::widgets::danger())
                .action(move || st.press())
                .id(day::id!(btn_destructive)),
        ),
        labeled(
            crate::res::str::ctl_disabled(),
            button(press())
                .icon(Symbol::Close)
                .enabled(false)
                .action(move || st.press())
                .id(day::id!(btn_disabled)),
        ),
        labeled(
            crate::res::str::ctl_icon_label(),
            button(press())
                .icon(Symbol::Add)
                .action(move || st.press())
                .id(day::id!(btn_icon_label)),
        ),
        labeled(
            crate::res::str::ctl_icon_only(),
            button(move || {
                if playing.get() {
                    crate::res::str::media_pause().format()
                } else {
                    crate::res::str::media_play().format()
                }
            })
            .bordered()
            .icon(move || {
                if playing.get() {
                    Symbol::Pause
                } else {
                    Symbol::Play
                }
            })
            .icon_only()
            .action(move || playing.set(!playing.get()))
            .id(day::id!(btn_icon_only)),
        ),
        labeled(
            crate::res::str::ctl_image(),
            button(press())
                .image(crate::res::vectors::nav_controls)
                .action(move || st.press())
                .id(day::id!(btn_image)),
        ),
        labeled(
            crate::res::str::ctl_link(),
            link(crate::res::str::ctl_link_text(), "https://daybrite.dev").id(day::id!(ctl_link)),
        ),
        labeled(
            crate::res::str::ctl_presses(),
            crate::widgets::numeric_readout(
                move || st.presses.get().to_string(),
                "888",
                day::id!(btn_presses),
            ),
        ),
    ))
    .title(crate::res::str::ctl_buttons())
}

/// The value controls, most of them over one number: the slider writes `level`, the progress
/// bar reports it, and the switch above gates the lot; the stepped slider keeps a value of its
/// own (see `Catalog::steps`).
/// The stepper (day-piece-stepper: an NSStepper field, a GtkSpinButton, a QDoubleSpinBox, and
/// a composed field elsewhere) keeps its own count, shown twice (once native where the
/// toolkit has one, once composed) so the two idioms sit side by side.
fn switches_section(st: Catalog) -> impl Piece {
    let dim = st.dim();
    section((
        labeled(
            crate::res::str::ctl_toggle(),
            toggle(st.on).id(day::id!(subscribe_toggle)),
        ),
        column((
            labeled(
                crate::res::str::ctl_slider(),
                row((
                    slider(st.level)
                        .range(0.0..=100.0)
                        .id(day::id!(volume_slider)),
                    // Reserves the width of "100" so the row stops reflowing as the value
                    // changes under the slider being dragged.
                    crate::widgets::numeric_readout(
                        move || format!("{:.0}", st.level.get()),
                        "100",
                        day::id!(volume_value),
                    ),
                ))
                .spacing(Space::S),
            ),
            labeled(
                crate::res::str::ctl_stepped(),
                row((
                    slider(st.steps)
                        .range(0.0..=100.0)
                        .step(10.0)
                        .id(day::id!(stepped_slider)),
                    crate::widgets::numeric_readout(
                        move || format!("{:.0}", st.steps.get()),
                        "100",
                        day::id!(stepped_value),
                    ),
                ))
                .spacing(Space::S),
            ),
            labeled(
                crate::res::str::ctl_stepper(),
                row((
                    stepper(st.count)
                        .range(0.0..=10.0)
                        .step(1.0)
                        .decimals(0)
                        .id(day::id!(stepper_field)),
                    crate::widgets::numeric_readout(
                        move || format!("{:.0}", st.count.get()),
                        "88",
                        day::id!(stepper_value),
                    ),
                ))
                .spacing(Space::S),
            ),
            labeled(
                crate::res::str::ctl_composed_stepper(),
                stepper(st.count)
                    .range(0.0..=10.0)
                    .step(1.0)
                    .decimals(0)
                    .composed()
                    .id(day::id!(stepper_composed)),
            ),
            labeled(
                crate::res::str::ctl_progress(),
                progress(move || st.level.get() / 100.0)
                    .id(day::id!(volume_progress))
                    .a11y(|a| a.role(Role::Meter).label("Level")),
            ),
            labeled(
                crate::res::str::ctl_activity(),
                row((
                    activity()
                        .animating(move || st.on.get() && st.level.get() > 0.0)
                        .id(day::id!(activity_spinner)),
                    label(move || {
                        if st.on.get() {
                            crate::res::str::activity_on()
                        } else {
                            crate::res::str::activity_off()
                        }
                        .format()
                    })
                    .id(day::id!(activity_status)),
                ))
                .spacing(Space::S)
                .fit(RowFit::Wrap { run_spacing: 8.0 }),
            ),
            labeled(
                crate::res::str::ctl_spinner(),
                spinner().id(day::id!(ctl_spinner)),
            ),
        ))
        .spacing(Space::S)
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
                .id(day::id!(name_field)),
        ),
        labeled(
            crate::res::str::ctl_search_field(),
            column((
                row((
                    search_field(st.query)
                        .placeholder(crate::res::str::voice_search_placeholder())
                        .id(day::id!(search_input)),
                    button(crate::res::str::search_clear())
                        .bordered()
                        .action(move || st.query.set(String::new()))
                        .id(day::id!(search_clear)),
                ))
                .spacing(Space::S),
                // The first match in the same list the combo offers: a value, or an em-dash.
                label(move || first_match(&st.query.get(), &voices.get()))
                    .font(Font::Footnote)
                    .id(day::id!(search_result)),
            ))
            .spacing(Space::XS)
            .align(HAlign::Leading),
        ),
        labeled(
            crate::res::str::ctl_combo_box(),
            // The bound value reads below the field, not at the end of the row: on the same row
            // it grew with every keystroke and pushed Add sideways as you typed.
            column((
                row((
                    combo_box(voices, st.voice)
                        .placeholder(crate::res::str::flavor_placeholder())
                        .id(day::id!(flavor_combo)),
                    button(crate::res::str::flavor_add())
                        .action(move || {
                            let typed = st.voice.get_untracked();
                            if !typed.is_empty() && !voices.get_untracked().contains(&typed) {
                                voices.update(|v| v.push(typed));
                            }
                        })
                        .tint(crate::widgets::primary())
                        .id(day::id!(flavor_add)),
                ))
                .spacing(Space::S),
                // No combo-box arm on iOS or the web (docs/combobox.md): day renders its
                // placeholder leaf in the row above, and this note sits right beside it.
                cfg!(any(target_os = "ios", target_arch = "wasm32"))
                    .then(|| label(crate::res::str::ctl_combo_note()).font(Font::Footnote)),
                label(move || st.voice.get())
                    .font(Font::Footnote)
                    .id(day::id!(flavor_value)),
            ))
            .spacing(Space::XS)
            .align(HAlign::Leading),
        ),
        labeled(
            crate::res::str::ctl_text_area(),
            text_area(st.notes)
                .placeholder(crate::res::str::ctl_notes_placeholder())
                .min_lines(3)
                .max_lines(3)
                .id(day::id!(notes_area)),
        ),
    ))
    .title(crate::res::str::ctl_text_entry())
}

/// Pickers: the built-in picker in its three stylings over one selection (docs/picker.md),
/// then the date, time and color pickers from their crates. The date and time pickers appear
/// here compact; the inline calendar has the Date & time page to itself.
fn pickers_section(st: Catalog) -> impl Piece {
    let preset_label = st.preset_label();
    let dim = st.dim();
    section((
        column((
            labeled(
                crate::res::str::picker_segmented(),
                picker(st.preset, Catalog::preset_options())
                    .segmented()
                    .id(day::id!(picker_segmented)),
            ),
            labeled(
                crate::res::str::picker_menu(),
                picker(st.preset, Catalog::preset_options())
                    .menu()
                    .id(day::id!(picker_menu)),
            ),
            labeled(
                crate::res::str::picker_inline(),
                picker(st.preset, Catalog::preset_options())
                    .inline()
                    .id(day::id!(picker_inline)),
            ),
            labeled(
                crate::res::str::picker_selected(),
                label(preset_label).id(day::id!(picker_value)),
            ),
        ))
        .spacing(Space::S)
        .opacity(dim),
        labeled(
            crate::res::str::ctl_date(),
            date_picker(st.date).compact().id(day::id!(ctl_date)),
        ),
        labeled(
            crate::res::str::ctl_time(),
            time_picker(st.time).compact().id(day::id!(ctl_time)),
        ),
        // Native chooser or a composed one per toolkit (docs/colorpicker.md), never a banner:
        // there is no target where it does not work, only which picker differs.
        labeled(
            crate::res::str::ctl_color(),
            color_picker(st.color).id(day::id!(ctl_color)),
        ),
    ))
    .title(crate::res::str::ctl_pickers())
}

/// Indicators and the composition tier: the star rating (canvas polygons), a badge overlay, the
/// arc gauge over the shared level, and the divider.
/// Accessibility (docs/accessibility.md): what a screen reader is told beyond what the native
/// controls say on their own. A heading role on a label, a hint on a button, and an announcement
/// that reaches the screen reader without moving its focus.
fn accessibility_section() -> impl Piece {
    let announced = Signal::new(0i64);
    let support = match capability(Cap::Announce) {
        Support::Native => crate::res::str::cursors_native(),
        Support::Emulated => crate::res::str::ctl_announce_emulated(),
        Support::Unsupported => crate::res::str::cursors_unsupported(),
    };
    section((
        label(crate::res::str::ctl_a11y_heading())
            .font(Font::Headline)
            .a11y(|a| a.role(Role::Heading(2)))
            .id(day::id!(ctl_a11y_heading)),
        labeled(
            crate::res::str::ctl_announce(),
            row((
                button(crate::res::str::ctl_announce())
                    .bordered()
                    .a11y(|a| a.hint(crate::res::str::ctl_announce_hint().format()))
                    .action(move || {
                        announced.update(|n| *n += 1);
                        announce(
                            &crate::res::str::ctl_announced_times(announced.get_untracked())
                                .format(),
                        );
                    })
                    .id(day::id!(ctl_announce)),
                label(move || crate::res::str::ctl_announced_times(announced.get()).format())
                    .id(day::id!(ctl_announce_count)),
            ))
            .spacing(Space::S)
            .fit(RowFit::Wrap { run_spacing: 8.0 }),
        ),
        labeled(
            crate::res::str::cursors_support_label(),
            label(support).id(day::id!(ctl_announce_support)),
        ),
    ))
    .title(crate::res::str::ctl_accessibility())
}

fn indicators_section(st: Catalog) -> impl Piece {
    section((
        labeled(
            crate::res::str::ctl_rating(),
            row((
                rating(st.stars)
                    .star_id(|n| day::id!(compose_rating_star, n))
                    .id(day::id!(compose_rating)),
                crate::widgets::numeric_readout(
                    move || st.stars.get().to_string(),
                    "5",
                    day::id!(rating_value),
                ),
            ))
            .spacing(Space::S),
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
            .id(day::id!(ctl_badge)),
        ),
        labeled(
            crate::res::str::ctl_gauge(),
            gauge(st.level).frame(120.0, 120.0),
        ),
        labeled(
            crate::res::str::ctl_divider(),
            divider().id(day::id!(ctl_divider)),
        ),
    ))
    .title(crate::res::str::ctl_indicators())
}

/// Every bound value, read back. Scrolling here is the walkthrough's second screenshot, and the
/// row values are what its assertions check after driving the controls above.
fn state_section(st: Catalog) -> impl Piece {
    let readout =
        |text: fn() -> LocalizedText, value: Box<dyn Fn() -> String>, id: day::ElementId| {
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
            day::id!(state_toggle),
        ),
        readout(
            crate::res::str::ctl_slider,
            Box::new(move || format!("{:.0}", st.level.get())),
            day::id!(state_level),
        ),
        readout(
            crate::res::str::ctl_stepper,
            Box::new(move || format!("{:.0}", st.count.get())),
            day::id!(state_count),
        ),
        readout(
            crate::res::str::ctl_text_field,
            Box::new(move || st.name.get()),
            day::id!(state_name),
        ),
        readout(
            crate::res::str::ctl_date,
            Box::new(move || st.date.get().to_string()),
            day::id!(state_date),
        ),
        readout(
            crate::res::str::ctl_time,
            Box::new(move || st.time.get().to_string()),
            day::id!(state_time),
        ),
        readout(
            crate::res::str::ctl_color,
            Box::new(move || st.color.get().to_hex_string()),
            day::id!(state_color),
        ),
        readout(
            crate::res::str::ctl_rating,
            Box::new(move || st.stars.get().to_string()),
            day::id!(state_rating),
        ),
        readout(
            crate::res::str::ctl_presses,
            Box::new(move || st.presses.get().to_string()),
            day::id!(state_presses),
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
