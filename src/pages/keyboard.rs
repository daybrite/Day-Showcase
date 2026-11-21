// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The Keyboard page: where key presses go, one card per route.
//!
//! Keys follow focus (docs/menus.md "Keys follow focus"), so each card owns a piece that can
//! hold it. A canvas with `.on_key` hears the arrows and the digits (the short route). A canvas
//! that also declares `.keyboard()` hears every key, with the text a press typed kept apart
//! from the key's name (the full route). A composed row becomes a key target with
//! `.focusable()`. A text field with `.on_key` hands over Up, Down and Escape, which is what a
//! suggestion list needs, and `.select(trigger, range)` selects the part of a suggestion the
//! reader did not type (docs/textfield.md "Keys" and "Selection").
//!
//! Every demo has native buttons that do what its keys do, so it runs on a phone with no
//! hardware keyboard, and a script can drive it on every toolkit.

use day::prelude::*;
use day::{Key, KeyEvent, KeyPhase, Modifiers};

use crate::gallery::{self, Tone, card, card_page};

// ---------------------------------------------------------------------------
// Shared parts
// ---------------------------------------------------------------------------

/// The wording for the pointer in use when the page is built: `touch` for a finger, `pointer`
/// for a mouse or a trackpad (`day::primary_pointer()`, the last input). Read once, untracked,
/// so a card's subtitle never rebuilds the page.
fn worded(
    touch: fn() -> day::LocalizedText,
    pointer: fn() -> day::LocalizedText,
) -> day::LocalizedText {
    if day::reactive::untrack(day::primary_pointer).hovers() {
        pointer()
    } else {
        touch()
    }
}

/// Whether a focused text field hands Up, Down and Escape to its `.on_key` handler on this
/// toolkit.
///
/// Declared: no `Cap` answers for it. The desktop toolkits and the web deliver the three
/// keys; UIKit, Android and ArkUI do not deliver them from a text field (docs/textfield.md
/// "Keys").
fn field_keys() -> Support {
    if cfg!(any(
        feature = "uikit",
        feature = "material",
        feature = "arkui"
    )) {
        Support::Unsupported
    } else {
        Support::Native
    }
}

/// A badge saying whether the card's key target holds focus, which is when its keys arrive.
fn focus_badge(id: ElementId, focused: Signal<bool>) -> impl Piece {
    label(move || {
        if focused.get() {
            crate::res::str::keyboard_focused().format()
        } else {
            crate::res::str::keyboard_unfocused().format()
        }
    })
    .font(Font::Caption)
    .weight(FontWeight::Medium)
    .color(move || {
        if focused.get() {
            Ink::Fixed(Tone::Accent.color())
        } else {
            Ink::system(SystemColor::SecondaryText)
        }
    })
    .single_line()
    .id(id)
    .padding(Insets::symmetric(10.0, 4.0))
    .background(move || {
        if focused.get() {
            Tone::Accent.tint()
        } else {
            gallery::INSET
        }
    })
    .corner_radius(Corner::CAPSULE)
}

/// A reading with its name above it, like the kit's `stat`, with `id` on the value so a script
/// can read it.
fn reading(
    name: day::LocalizedText,
    id: ElementId,
    value: impl Fn() -> String + 'static,
) -> impl Piece {
    column((
        label(name).font(Font::Caption).secondary(),
        label(value)
            .font(Font::Callout)
            .tabular()
            .single_line()
            .id(id),
    ))
    .spacing(2.0)
    .align(HAlign::Leading)
    .grow(Dimension::Width)
}

/// A bordered button showing only `symbol`, named `title` for a screen reader.
fn icon_button(
    symbol: Symbol,
    title: day::LocalizedText,
    id: ElementId,
    action: impl Fn() + 'static,
) -> impl Piece {
    button(title)
        .bordered()
        .icon(symbol)
        .icon_only()
        .action(action)
        .id(id)
}

/// The stroke that marks a stage whose canvas holds focus.
fn focus_ring(draw: &mut Draw, size: Size, radius: f64) {
    draw.stroke(
        Shape::RoundedRect(
            Rect::new(1.0, 1.0, size.width - 2.0, size.height - 2.0),
            radius,
        ),
        Tone::Accent.color(),
        2.0,
    );
}

// ---------------------------------------------------------------------------
// 1. Arrow keys on a canvas
// ---------------------------------------------------------------------------

/// The sky's grid: the probe moves from cell to cell.
const SKY_COLUMNS: i32 = 9;
const SKY_ROWS: i32 = 5;
/// Where the probe starts, and where the Center button sends it.
const SKY_HOME: (i32, i32) = (SKY_COLUMNS / 2, SKY_ROWS / 2);
/// How many cells Shift moves the probe by.
const SKY_LEAP: i32 = 3;
/// How many past cells the probe's trail keeps.
const SKY_TRAIL: usize = 6;
/// The night sky, the same in both appearances: it is the picture, not a surface.
const SKY_COLOR: Color = Color::rgba(0.043, 0.063, 0.149, 1.0);

/// The middle of cell `cell` on a sky of `size`.
fn sky_point(cell: (i32, i32), size: Size) -> Point {
    Point::new(
        (f64::from(cell.0) + 0.5) * size.width / f64::from(SKY_COLUMNS),
        (f64::from(cell.1) + 0.5) * size.height / f64::from(SKY_ROWS),
    )
}

/// Draws the star field: the same scatter on every run, from a small generator, so the
/// screenshots reproduce.
fn draw_stars(draw: &mut Draw, size: Size) {
    let mut seed: u32 = 0x2F6F_DE01;
    let mut next = || {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        f64::from(seed >> 8) / f64::from(1u32 << 24)
    };
    for _ in 0..70 {
        let (x, y) = (next() * size.width, next() * size.height);
        let radius = 0.5 + next() * 1.1;
        let star = Color::rgba(1.0, 1.0, 1.0, 0.25 + next() * 0.6);
        draw.fill(
            Shape::Ellipse(Rect::new(
                x - radius,
                y - radius,
                radius * 2.0,
                radius * 2.0,
            )),
            star,
        );
    }
}

/// Draws the probe at `at`: a gold body, two solar panels and a dish.
fn draw_probe(draw: &mut Draw, at: Point) {
    let panel = Color::rgba(0.42, 0.66, 0.98, 1.0);
    let body = Color::rgba(0.96, 0.78, 0.33, 1.0);
    draw.stroke(
        Shape::Line(Point::new(at.x - 17.0, at.y), Point::new(at.x + 17.0, at.y)),
        Color::rgba(1.0, 1.0, 1.0, 0.8),
        1.5,
    );
    for side in [-1.0, 1.0] {
        draw.fill(
            Shape::RoundedRect(
                Rect::new(at.x + side * 13.0 - 5.0, at.y - 6.0, 10.0, 12.0),
                2.0,
            ),
            panel,
        );
    }
    draw.fill(
        Shape::RoundedRect(Rect::new(at.x - 6.0, at.y - 6.0, 12.0, 12.0), 3.0),
        body,
    );
    draw.fill(
        Shape::Ellipse(Rect::new(at.x - 3.0, at.y - 12.0, 6.0, 5.0)),
        Color::rgba(1.0, 1.0, 1.0, 0.95),
    );
}

/// A probe on a star field, moved by the arrow keys while the canvas has focus (the short
/// route: `.on_key` with no `.keyboard()`, so the canvas claims the arrows and the digits and
/// Tab still walks). The four buttons make the same moves.
fn probe_card() -> AnyPiece {
    let at = Signal::new(SKY_HOME);
    // The cells the probe left, oldest first.
    let trail: Signal<Vec<(i32, i32)>> = Signal::new(Vec::new());
    let focused = Signal::new(false);

    let nudge = move |dx: i32, dy: i32| {
        let from = at.get_untracked();
        let to = (
            (from.0 + dx).clamp(0, SKY_COLUMNS - 1),
            (from.1 + dy).clamp(0, SKY_ROWS - 1),
        );
        if to != from {
            trail.update(|cells| {
                cells.push(from);
                if cells.len() > SKY_TRAIL {
                    cells.remove(0);
                }
            });
            at.set(to);
        }
    };
    let home = move || {
        trail.set(Vec::new());
        at.set(SKY_HOME);
    };

    let draw = move |draw: &mut Draw, size: Size| {
        let bounds = Rect::new(0.0, 0.0, size.width, size.height);
        draw.fill(Shape::RoundedRect(bounds, 12.0), SKY_COLOR);
        draw.clipped(Shape::RoundedRect(bounds, 12.0), |draw| {
            draw_stars(draw, size);
        });
        // The trail: a line through the cells the probe left, fading toward the oldest.
        let mut cells = trail.get();
        cells.push(at.get());
        let count = cells.len();
        for (index, pair) in cells.windows(2).enumerate() {
            let alpha = 0.15 + 0.5 * (index + 1) as f64 / count as f64;
            draw.stroke(
                Shape::Line(sky_point(pair[0], size), sky_point(pair[1], size)),
                Tone::Accent.color().with_alpha(alpha),
                2.0,
            );
        }
        draw_probe(draw, sky_point(at.get(), size));
        if focused.get() {
            focus_ring(draw, size, 12.0);
        }
    };
    // A held arrow keeps moving the probe: its repeats arrive here too, as `KeyPhase::Repeat`.
    let on_key = move |key: &KeyEvent| {
        let step = if key.shift() { SKY_LEAP } else { 1 };
        match key.key {
            Key::ArrowLeft => nudge(-step, 0),
            Key::ArrowRight => nudge(step, 0),
            Key::ArrowUp => nudge(0, -step),
            Key::ArrowDown => nudge(0, step),
            _ => {}
        }
    };

    // The row of buttons mirrors under a right-to-left layout and so do the two chevrons, so
    // the button at the row's start moves the probe toward the start of the line: left in
    // English, right in Arabic.
    let rtl = is_rtl();
    let (start_dx, start_title, start_id, end_title, end_id) = if rtl {
        (
            1,
            crate::res::str::keyboard_probe_right(),
            day::id!(kb_probe_right),
            crate::res::str::keyboard_probe_left(),
            day::id!(kb_probe_left),
        )
    } else {
        (
            -1,
            crate::res::str::keyboard_probe_left(),
            day::id!(kb_probe_left),
            crate::res::str::keyboard_probe_right(),
            day::id!(kb_probe_right),
        )
    };

    card(
        crate::res::vectors::nav_canvas,
        crate::res::str::keyboard_probe_title(),
        worded(
            crate::res::str::keyboard_probe_subtitle_touch,
            crate::res::str::keyboard_probe_subtitle_pointer,
        ),
        column((
            canvas(draw)
                .on_key(on_key)
                .focused(focused)
                .a11y(|a| a.label(crate::res::str::keyboard_probe_stage_label()))
                .id(day::id!(kb_probe_sky))
                .height(190.0)
                .grow(Dimension::Width),
            row((
                gallery::actions((
                    icon_button(Symbol::Back, start_title, start_id, move || {
                        nudge(start_dx, 0);
                    }),
                    icon_button(
                        Symbol::Up,
                        crate::res::str::keyboard_probe_up(),
                        day::id!(kb_probe_up),
                        move || nudge(0, -1),
                    ),
                    icon_button(
                        Symbol::Down,
                        crate::res::str::keyboard_probe_down(),
                        day::id!(kb_probe_down),
                        move || nudge(0, 1),
                    ),
                    icon_button(Symbol::Forward, end_title, end_id, move || {
                        nudge(-start_dx, 0);
                    }),
                    button(crate::res::str::keyboard_probe_center())
                        .action(home)
                        .id(day::id!(kb_probe_center)),
                ))
                .grow(Dimension::Width),
                reading(
                    crate::res::str::keyboard_probe_position(),
                    day::id!(kb_probe_position),
                    move || {
                        let (column, row) = at.get();
                        format!("{}, {}", column + 1, row + 1)
                    },
                )
                .reserving("88, 88"),
            ))
            .spacing(Space::M)
            .align(VAlign::Center)
            .grow(Dimension::Width),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .trailing(focus_badge(day::id!(kb_probe_focus), focused))
    .id(day::id!(kb_probe))
    .build()
}

// ---------------------------------------------------------------------------
// 2. Every key, with its modifiers
// ---------------------------------------------------------------------------

/// The last key event the key card heard.
#[derive(Clone, Default, PartialEq)]
struct Press {
    /// The key, `Unidentified` for text an input method committed with no single key behind it.
    key: Option<Key>,
    /// The modifiers held with it.
    modifiers: Modifiers,
    /// What the press typed, if it typed.
    text: Option<String>,
    /// Down, repeated, or up.
    phase: KeyPhase,
}

impl Press {
    /// What the keycap shows: the key's name, a word for the space bar, or the committed text.
    fn cap(&self) -> String {
        match (self.key, &self.text) {
            (Some(Key::Character(' ')), _) => crate::res::str::keyboard_key_space().format(),
            (Some(Key::Unidentified) | None, Some(text)) => text.clone(),
            (Some(key), _) => key.name(),
            (None, None) => String::new(),
        }
    }

    /// Whether `modifier` was held.
    fn held(&self, modifier: Modifiers) -> bool {
        self.modifiers.contains(modifier)
    }

    /// The phase's name in the readout.
    fn phase_name(&self) -> day::LocalizedText {
        match self.phase {
            KeyPhase::Down => crate::res::str::keyboard_phase_down(),
            KeyPhase::Repeat => crate::res::str::keyboard_phase_repeat(),
            KeyPhase::Up => crate::res::str::keyboard_phase_up(),
        }
    }
}

/// The modifiers the keycaps show, in the order they are drawn.
///
/// Three, not four: the primary key is Control everywhere but on Apple platforms, where a ⌘
/// chord stays with the menu and never reaches a piece (docs/menus.md "The full route").
fn modifier_names() -> [(Modifiers, String); 3] {
    let alt = if cfg!(target_vendor = "apple") {
        crate::res::str::keyboard_mod_option()
    } else {
        crate::res::str::keyboard_mod_alt()
    };
    [
        (
            Modifiers::SHIFT,
            crate::res::str::keyboard_mod_shift().format(),
        ),
        (
            Modifiers::CONTROL,
            crate::res::str::keyboard_mod_control().format(),
        ),
        (Modifiers::ALT, alt.format()),
    ]
}

/// Draws one keycap in `rect`: lit in the accent while `lit`, raised otherwise.
fn draw_keycap(draw: &mut Draw, rect: Rect, name: &str, text_size: f64, lit: bool) {
    let (fill, ink) = if lit {
        (Ink::Fixed(Tone::Accent.color()), Ink::Fixed(Color::WHITE))
    } else {
        (gallery::RAISED, Ink::system(SystemColor::Text))
    };
    // The edge under the cap, which is what makes it read as a key.
    let mut edge = rect;
    edge.origin.y += 3.0;
    draw.fill(Shape::RoundedRect(edge, 8.0), SystemColor::Separator);
    draw.fill(Shape::RoundedRect(rect, 8.0), fill);
    draw.stroke(Shape::RoundedRect(rect, 8.0), SystemColor::Separator, 1.0);
    draw.text(
        name,
        Point::new(
            rect.origin.x + rect.size.width / 2.0,
            rect.origin.y + rect.size.height / 2.0,
        ),
        TextStyle {
            size: text_size,
            color: ink,
            anchor: TextAnchor::CENTERED,
            ..Default::default()
        },
    );
}

/// A canvas that takes the whole keyboard (`.keyboard()`, the full route): the last key as a
/// keycap under the three modifiers, lit while they are held, and the same event as readings.
/// `.on_key` hears the press and its repeats and `.on_key_up` the release, so the key's cap is
/// lit from down to up where the toolkit reports releases (`Cap::KeyPhases`).
fn keys_card() -> AnyPiece {
    let last: Signal<Option<Press>> = Signal::new(None);
    let presses = Signal::new(0u32);
    let focused = Signal::new(false);
    let modifiers = modifier_names();
    let phases = capability(Cap::KeyPhases) == Support::Native;

    let draw = {
        let modifiers = modifiers.clone();
        move |draw: &mut Draw, size: Size| {
            gallery::dotted_stage(draw, size, 14.0);
            let press = last.get().unwrap_or_default();
            // The modifiers: three small caps in a row, centered.
            let (cap_width, cap_height, gap) = (78.0, 28.0, 8.0);
            let total = cap_width * 3.0 + gap * 2.0;
            let mut x = (size.width - total) / 2.0;
            for (bit, name) in &modifiers {
                draw_keycap(
                    draw,
                    Rect::new(x, 16.0, cap_width, cap_height),
                    name,
                    12.0,
                    press.held(*bit),
                );
                x += cap_width + gap;
            }
            // The key itself, as wide as its name needs.
            let name = press.cap();
            let shown: String = name.chars().take(14).collect();
            let width = (shown.chars().count() as f64 * 15.0 + 30.0)
                .clamp(64.0, (size.width - 32.0).max(64.0));
            draw_keycap(
                draw,
                Rect::new((size.width - width) / 2.0, 62.0, width, 56.0),
                &shown,
                24.0,
                phases && press.key.is_some() && press.phase != KeyPhase::Up,
            );
            if focused.get() {
                focus_ring(draw, size, 12.0);
            }
        }
    };
    let record = move |key: &KeyEvent| {
        if key.phase == KeyPhase::Down {
            presses.update(|count| *count += 1);
        }
        last.set(Some(Press {
            key: Some(key.key),
            modifiers: key.modifiers,
            // A release types nothing: keep what its press typed.
            text: if key.phase == KeyPhase::Up {
                last.get_untracked().and_then(|press| press.text)
            } else {
                key.text.clone()
            },
            phase: key.phase,
        }));
    };
    let held = move || {
        let press = last.get().unwrap_or_default();
        let names: Vec<&str> = modifiers
            .iter()
            .filter(|(bit, _)| press.held(*bit))
            .map(|(_, name)| name.as_str())
            .collect();
        if names.is_empty() {
            "\u{2014}".to_string()
        } else {
            names.join(" + ")
        }
    };

    card(
        crate::res::vectors::nav_keyboard,
        crate::res::str::keyboard_keys_title(),
        worded(
            crate::res::str::keyboard_keys_subtitle_touch,
            crate::res::str::keyboard_keys_subtitle_pointer,
        ),
        column((
            // ArkUI delivers a canvas's keys from a hardware keyboard only: a canvas is not an
            // input-method client there, so no soft keyboard rises for it (docs/menus.md).
            cfg!(feature = "arkui")
                .then(|| gallery::note(crate::res::str::keyboard_keys_hardware_note())),
            canvas(draw)
                .keyboard()
                .on_key(record)
                .on_key_up(record)
                .focused(focused)
                .a11y(|a| a.label(crate::res::str::keyboard_keys_stage_label()))
                .id(day::id!(kb_keys_stage))
                .height(136.0)
                .grow(Dimension::Width),
            row((
                reading(
                    crate::res::str::keyboard_keys_key(),
                    day::id!(kb_keys_key),
                    move || {
                        last.get()
                            .map_or_else(|| "\u{2014}".to_string(), |press| press.cap())
                    },
                ),
                reading(
                    crate::res::str::keyboard_keys_phase(),
                    day::id!(kb_keys_phase),
                    move || {
                        last.get().map_or_else(
                            || "\u{2014}".to_string(),
                            |press| press.phase_name().format(),
                        )
                    },
                ),
                reading(
                    crate::res::str::keyboard_keys_typed(),
                    day::id!(kb_keys_typed),
                    move || {
                        last.get()
                            .and_then(|press| press.text)
                            .filter(|text| !text.trim().is_empty())
                            .unwrap_or_else(|| "\u{2014}".to_string())
                    },
                ),
            ))
            .spacing(Space::M)
            .grow(Dimension::Width),
            row((
                reading(
                    crate::res::str::keyboard_keys_modifiers(),
                    day::id!(kb_keys_modifiers),
                    held,
                ),
                reading(
                    crate::res::str::keyboard_keys_presses(),
                    day::id!(kb_keys_presses),
                    move || presses.get().to_string(),
                ),
            ))
            .spacing(Space::M)
            .grow(Dimension::Width),
            (!phases).then(|| gallery::note(crate::res::str::keyboard_keys_phases_note())),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .trailing(focus_badge(day::id!(kb_keys_focus), focused))
    .id(day::id!(kb_keys))
    .build()
}

// ---------------------------------------------------------------------------
// 3. A focusable row
// ---------------------------------------------------------------------------

/// A planet of the row: its name, its color, and the diameter of its disc in points.
struct Planet {
    name: fn() -> day::LocalizedText,
    color: u32,
    size: f64,
}

/// The eight planets, in order from the Sun, which is also the digit that picks each.
const PLANETS: [Planet; 8] = [
    Planet {
        name: crate::res::str::keyboard_planet_mercury,
        color: 0x9A_9A_A0,
        size: 10.0,
    },
    Planet {
        name: crate::res::str::keyboard_planet_venus,
        color: 0xE3_C2_7A,
        size: 15.0,
    },
    Planet {
        name: crate::res::str::keyboard_planet_earth,
        color: 0x4B_8F_E2,
        size: 16.0,
    },
    Planet {
        name: crate::res::str::keyboard_planet_mars,
        color: 0xC8_55_3D,
        size: 12.0,
    },
    Planet {
        name: crate::res::str::keyboard_planet_jupiter,
        color: 0xD9_A0_66,
        size: 28.0,
    },
    Planet {
        name: crate::res::str::keyboard_planet_saturn,
        color: 0xE4_CD_8F,
        size: 24.0,
    },
    Planet {
        name: crate::res::str::keyboard_planet_uranus,
        color: 0x7F_CF_D6,
        size: 20.0,
    },
    Planet {
        name: crate::res::str::keyboard_planet_neptune,
        color: 0x46_67_D6,
        size: 19.0,
    },
];

/// The side of a planet's cell in the row.
const PLANET_CELL: f64 = 34.0;

/// A composed row made a key target with `.focusable()`: while it has focus the arrows step
/// through the planets and a digit picks one. The row is a container, with no native control
/// under it, which is the case the decorator exists for; the Previous and Next buttons write
/// the same selection.
fn planets_card() -> AnyPiece {
    let selected = Signal::new(2usize);
    let focused = Signal::new(false);
    let last = PLANETS.len() - 1;
    let step = move |forward: bool| {
        selected.update(|index| {
            *index = if forward {
                (*index + 1).min(last)
            } else {
                index.saturating_sub(1)
            };
        });
    };
    // The row mirrors under a right-to-left layout, so the arrow that points at the next
    // planet does too.
    let toward_end = if is_rtl() {
        Key::ArrowLeft
    } else {
        Key::ArrowRight
    };
    let on_key = move |key: &KeyEvent| match key.key {
        Key::ArrowLeft | Key::ArrowRight => step(key.key == toward_end),
        _ => {
            if let Some(digit) = key.digit().map(usize::from)
                && (1..=PLANETS.len()).contains(&digit)
            {
                selected.set(digit - 1);
            }
        }
    };

    let cells: Vec<AnyPiece> = PLANETS
        .iter()
        .enumerate()
        .map(|(index, planet)| {
            zstack((
                circle()
                    .stroke(
                        move || {
                            if selected.get() == index {
                                Ink::Fixed(Tone::Accent.color())
                            } else {
                                Ink::CLEAR
                            }
                        },
                        2.0,
                    )
                    .frame(PLANET_CELL, PLANET_CELL),
                circle()
                    .fill(Color::hex(planet.color))
                    .frame(planet.size, planet.size),
            ))
            .height(PLANET_CELL)
            .grow(Dimension::Width)
            .any()
        })
        .collect();

    card(
        crate::res::vectors::nav_focus,
        crate::res::str::keyboard_planets_title(),
        worded(
            crate::res::str::keyboard_planets_subtitle_touch,
            crate::res::str::keyboard_planets_subtitle_pointer,
        ),
        column((
            crate::widgets::support_banner(capability(Cap::Focusable)),
            row(cells)
                .spacing(2.0)
                .align(VAlign::Center)
                .padding(Insets::symmetric(8.0, 10.0))
                .background(move || {
                    if focused.get() {
                        Tone::Accent.tint()
                    } else {
                        gallery::INSET
                    }
                })
                .corner_radius(12.0)
                .a11y(|a| a.label(crate::res::str::keyboard_planets_row_label()))
                // The id names the rounded container, the view that takes focus, where a
                // plain `.id` would name the row inside it.
                .wrapper_id(day::id!(kb_planets_row))
                .focusable()
                .on_key(on_key)
                .focused(focused)
                .grow(Dimension::Width),
            row((
                column((
                    label(move || (PLANETS[selected.get().min(last)].name)().format())
                        .font(Font::Title3)
                        .single_line()
                        .id(day::id!(kb_planets_name)),
                    label(move || format!("{} / {}", selected.get() + 1, PLANETS.len()))
                        .font(Font::Caption)
                        .secondary()
                        .tabular()
                        .id(day::id!(kb_planets_index)),
                ))
                .spacing(2.0)
                .align(HAlign::Leading)
                .grow(Dimension::Width),
                gallery::actions((
                    button(crate::res::str::keyboard_planets_previous())
                        .bordered()
                        .enabled(move || selected.get() > 0)
                        .action(move || step(false))
                        .id(day::id!(kb_planets_previous)),
                    button(crate::res::str::keyboard_planets_next())
                        .bordered()
                        .enabled(move || selected.get() < last)
                        .action(move || step(true))
                        .id(day::id!(kb_planets_next)),
                )),
            ))
            .spacing(Space::M)
            .align(VAlign::Center)
            .grow(Dimension::Width),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .trailing(focus_badge(day::id!(kb_planets_focus), focused))
    .id(day::id!(kb_planets))
    .build()
}

// ---------------------------------------------------------------------------
// 4. A field that walks its suggestions
// ---------------------------------------------------------------------------

/// The catalog the field suggests from: the proper names of bright stars, which are the same
/// in every language the showcase ships.
const STARS: [&str; 21] = [
    "Achernar",
    "Aldebaran",
    "Altair",
    "Antares",
    "Arcturus",
    "Bellatrix",
    "Betelgeuse",
    "Canopus",
    "Capella",
    "Castor",
    "Deneb",
    "Fomalhaut",
    "Mimosa",
    "Polaris",
    "Pollux",
    "Procyon",
    "Regulus",
    "Rigel",
    "Sirius",
    "Spica",
    "Vega",
];

/// How many suggestions the list shows.
const SUGGESTIONS: usize = 4;

/// The first stars whose names begin with `typed`, ignoring case. An empty field suggests the
/// head of the catalog.
fn suggestions(typed: &str) -> Vec<&'static str> {
    let typed = typed.trim().to_lowercase();
    STARS
        .iter()
        .filter(|star| star.to_lowercase().starts_with(&typed))
        .take(SUGGESTIONS)
        .copied()
        .collect()
}

/// A text field over a list of suggestions. With `.on_key` the field hands Up, Down and Escape
/// to the page: Up and Down move the highlight and put the highlighted name in the field,
/// where `.select(trigger, range)` selects the part the reader did not type, so the next
/// keystroke carries on from what they typed. Return picks; Escape puts the typed text back.
fn suggest_card() -> AnyPiece {
    let text = Signal::new(String::new());
    // What the reader typed, which is what the list filters by while the field shows a
    // suggestion.
    let typed = Signal::new(String::new());
    let highlight: Signal<Option<usize>> = Signal::new(None);
    let picked = Signal::new(String::new());
    // The text the page last put in the field. The watch below takes any other text for
    // typing.
    let written: Signal<Option<String>> = Signal::new(None);
    // The selection the next firing of `select` applies, in characters.
    let range = Signal::new(0..0usize);
    let select = Trigger::new();

    // Writes `value` into the field as the page's text, with the caret or selection at `to`.
    let write = move |value: String, to: std::ops::Range<usize>| {
        written.set(Some(value.clone()));
        text.set(value);
        range.set(to);
        select.notify();
    };
    watch(
        move || text.get(),
        move |now: &String, _| {
            if written.get_untracked().as_ref() == Some(now) {
                return;
            }
            written.set(None);
            typed.set(now.clone());
            highlight.set(None);
        },
    );
    // Moves the highlight one suggestion down or up, around the ends, and shows that name in
    // the field with everything past the typed text selected.
    let walk = move |down: bool| {
        let prefix = typed.get_untracked();
        let found = suggestions(&prefix);
        if found.is_empty() {
            return;
        }
        let next = match (highlight.get_untracked(), down) {
            (None, true) => 0,
            (None, false) => found.len() - 1,
            (Some(index), true) => (index + 1) % found.len(),
            (Some(index), false) => (index + found.len() - 1) % found.len(),
        };
        highlight.set(Some(next));
        write(found[next].to_string(), prefix.chars().count()..usize::MAX);
    };
    let pick = move || {
        let name = text.get_untracked();
        if name.trim().is_empty() {
            return;
        }
        let end = name.chars().count();
        picked.set(name.clone());
        typed.set(name.clone());
        highlight.set(None);
        write(name, end..end);
    };
    let restore = move || {
        let prefix = typed.get_untracked();
        let end = prefix.chars().count();
        highlight.set(None);
        write(prefix, end..end);
    };

    let rows: Vec<AnyPiece> = (0..SUGGESTIONS)
        .map(|slot| {
            when(
                move || suggestions(&typed.get()).len() > slot,
                move || {
                    label(move || {
                        suggestions(&typed.get())
                            .get(slot)
                            .map_or_else(String::new, |star| (*star).to_string())
                    })
                    .single_line()
                    .id(day::id!(kb_suggest_row, slot))
                    .grow(Dimension::Width)
                    .padding(Insets::symmetric(10.0, 6.0))
                    .background(move || {
                        if highlight.get() == Some(slot) {
                            Tone::Accent.tint()
                        } else {
                            Ink::CLEAR
                        }
                    })
                    .corner_radius(8.0)
                },
            )
            .any()
        })
        .collect();

    card(
        crate::res::vectors::nav_textfields,
        crate::res::str::keyboard_suggest_title(),
        crate::res::str::keyboard_suggest_subtitle(),
        column((
            crate::widgets::support_banner(field_keys()),
            row((
                text_field(text)
                    .placeholder(crate::res::str::keyboard_suggest_placeholder())
                    .on_submit(pick)
                    .select(select, move || range.get_untracked())
                    .on_key(move |key: &KeyEvent| match key.key {
                        Key::ArrowDown => walk(true),
                        Key::ArrowUp => walk(false),
                        Key::Escape => restore(),
                        _ => {}
                    })
                    .id(day::id!(kb_suggest_field))
                    .grow(Dimension::Width),
                icon_button(
                    Symbol::Up,
                    crate::res::str::keyboard_suggest_previous(),
                    day::id!(kb_suggest_up),
                    move || walk(false),
                ),
                icon_button(
                    Symbol::Down,
                    crate::res::str::keyboard_suggest_next(),
                    day::id!(kb_suggest_down),
                    move || walk(true),
                ),
            ))
            .spacing(Space::S)
            .align(VAlign::Center)
            .grow(Dimension::Width),
            column(rows)
                .spacing(2.0)
                .align(HAlign::Leading)
                .padding(4.0)
                .background(gallery::INSET)
                .corner_radius(10.0)
                .grow(Dimension::Width),
            row((
                label(crate::res::str::keyboard_suggest_picked())
                    .font(Font::Footnote)
                    .secondary()
                    .grow(Dimension::Width),
                label(move || {
                    let name = picked.get();
                    if name.is_empty() {
                        "\u{2014}".to_string()
                    } else {
                        name
                    }
                })
                .font(Font::Callout)
                .single_line()
                .id(day::id!(kb_suggest_picked)),
            ))
            .spacing(Space::S)
            .align(VAlign::Center)
            .grow(Dimension::Width),
        ))
        .spacing(Space::M)
        .align(HAlign::Leading),
    )
    .id(day::id!(kb_suggest))
    .build()
}

// ---------------------------------------------------------------------------
// The page
// ---------------------------------------------------------------------------

/// The Keyboard page: four cards, one per route a key takes to a piece.
pub(crate) fn keyboard_page() -> AnyPiece {
    card_page(
        crate::res::str::nav_keyboard(),
        day::id!(keyboard_title),
        2,
        vec![probe_card, keys_card, planets_card, suggest_card],
    )
}
