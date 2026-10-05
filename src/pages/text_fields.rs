use day::prelude::*;

use crate::widgets::page;

/// The PIN's length, and the limited field's: named because the counters beside them read the
/// same number the fields are held to.
const PIN_LENGTH: u32 = 4;
const NOTE_LENGTH: u32 = 12;

/// Text fields (docs/textfield.md): how a `text_field` takes its text. Every section is the
/// platform's own single-line field; what changes is what it is told about its contents.
pub(crate) fn text_fields_page() -> AnyPiece {
    page(crate::res::str::nav_textfields(), "textfields-title", {
        form((
            password_section(),
            purpose_section(),
            pin_section(),
            attributes_section(),
        ))
        .any()
    })
    .any()
}

/// A password field and its Show Password switch. The switch drives `.secure(_)`, so the same
/// field shows and hides its characters, keeping its text and its focus on every backend,
/// including the two (AppKit, WinUI) whose secure field is a different native class.
fn password_section() -> impl Piece {
    let password = Signal::new(String::new());
    let shown = Signal::new(false);
    section((
        labeled(
            crate::res::str::textfields_password(),
            secure_field(password)
                .secure(move || !shown.get())
                .placeholder(crate::res::str::textfields_password_placeholder())
                .id("tf-password"),
        ),
        labeled(
            crate::res::str::textfields_show_password(),
            toggle(shown).id("tf-show-password"),
        ),
        // The length, never the text: a readout of a password would defeat the field above it.
        labeled(
            crate::res::str::textfields_length(),
            label(move || password.get().chars().count().to_string()).id("tf-password-length"),
        ),
    ))
    .title(crate::res::str::textfields_password_section())
}

/// One field per purpose. A phone or tablet changes its keyboard for each; every platform
/// changes what it offers to fill in, and whether it capitalizes and corrects.
fn purpose_section() -> impl Piece {
    let field = |title: day::LocalizedText,
                 purpose: InputPurpose,
                 placeholder: &'static str,
                 id: &'static str| {
        labeled(
            title,
            text_field(Signal::new(String::new()))
                .input_purpose(purpose)
                .submit_label(SubmitLabel::Next)
                .placeholder(placeholder)
                .id(id),
        )
    };
    section((
        field(
            crate::res::str::textfields_email(),
            InputPurpose::Email,
            "ada@example.com",
            "tf-email",
        ),
        field(
            crate::res::str::textfields_phone(),
            InputPurpose::Phone,
            "+1 555 0100",
            "tf-phone",
        ),
        field(
            crate::res::str::textfields_url(),
            InputPurpose::Url,
            "https://daybrite.dev",
            "tf-url",
        ),
        field(
            crate::res::str::textfields_number(),
            InputPurpose::Number,
            "42",
            "tf-number",
        ),
        field(
            crate::res::str::textfields_decimal(),
            InputPurpose::Decimal,
            "19.99",
            "tf-decimal",
        ),
        field(
            crate::res::str::textfields_code(),
            InputPurpose::OneTimeCode,
            "123456",
            "tf-code",
        ),
    ))
    .title(crate::res::str::textfields_purpose_section())
}

/// Secure and numeric together, held to four characters: the traits compose.
fn pin_section() -> impl Piece {
    let pin = Signal::new(String::new());
    section((
        labeled(
            crate::res::str::textfields_pin(),
            secure_field(pin)
                .input_purpose(InputPurpose::Number)
                .max_length(PIN_LENGTH)
                .submit_label(SubmitLabel::Done)
                .id("tf-pin"),
        ),
        labeled(
            crate::res::str::textfields_length(),
            label(move || format!("{} / {PIN_LENGTH}", pin.get().chars().count()))
                .id("tf-pin-count"),
        ),
    ))
    .title(crate::res::str::textfields_pin_section())
}

/// Read-only, a length limit, and the keyboard's action key.
fn attributes_section() -> impl Piece {
    let reference = Signal::new("DAY-2026-0042".to_string());
    let read_only = Signal::new(true);
    let note = Signal::new(String::new());
    let message = Signal::new(String::new());
    let sent = Signal::new(0u32);
    section((
        labeled(
            crate::res::str::textfields_reference(),
            text_field(reference).read_only(read_only).id("tf-readonly"),
        ),
        labeled(
            crate::res::str::textfields_read_only(),
            toggle(read_only).id("tf-readonly-toggle"),
        ),
        labeled(
            crate::res::str::textfields_limited(),
            text_field(note).max_length(NOTE_LENGTH).id("tf-limited"),
        ),
        labeled(
            crate::res::str::textfields_length(),
            label(move || format!("{} / {NOTE_LENGTH}", note.get().chars().count()))
                .id("tf-limited-count"),
        ),
        labeled(
            crate::res::str::textfields_message(),
            text_field(message)
                .submit_label(SubmitLabel::Send)
                .on_submit(move || {
                    if !message.get_untracked().is_empty() {
                        sent.update(|n| *n += 1);
                        message.set(String::new());
                    }
                })
                .placeholder(crate::res::str::textfields_message_placeholder())
                .id("tf-message"),
        ),
        labeled(
            crate::res::str::textfields_sent(),
            label(move || sent.get().to_string()).id("tf-sent-count"),
        ),
    ))
    .title(crate::res::str::textfields_attrs_section())
}
