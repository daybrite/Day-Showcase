use day::prelude::*;
use day_part_biometrics::Prompt;
use day_part_keychain::{Protection, Secret};

use crate::widgets::page;

/// Security (docs/keychain.md, docs/biometrics.md): the two parts that keep something from the
/// wrong person. The keychain stores an item at a protection level in the platform's own secure
/// store and reads it back; the biometric prompt is the platform's own "prove it is you" dialog,
/// which the keychain's user-presence level uses on the platforms that need one.
pub(crate) fn security_page() -> AnyPiece {
    page(
        crate::res::str::nav_security(),
        day::id!(security_title),
        form((keychain_section(), biometrics_section())).any(),
    )
    .any()
}

/// The three-way support label every section here shows: what the part promises for this target.
fn support_text(support: Support) -> LocalizedText {
    match support {
        Support::Native => crate::res::str::speech_native(),
        Support::Emulated => crate::res::str::speech_emulated(),
        Support::Unsupported => crate::res::str::speech_unsupported(),
    }
}

/// Every key this app stored, with its level, as one line.
fn entries_text() -> String {
    match day_part_keychain::entries() {
        Ok(entries) if entries.is_empty() => crate::res::str::keychain_entries_none().format(),
        Ok(entries) => entries
            .iter()
            .map(|e| format!("{} ({:?})", e.key, e.protection))
            .collect::<Vec<_>>()
            .join(", "),
        Err(e) => e.to_string(),
    }
}

/// The keychain: a key, a secret, a protection level, and the four calls.
///
/// Save never shows a dialog, at any level. Read is the synchronous call, which answers
/// `NeedsPresence` for a user-presence item; "Read with verification" is the future, which shows
/// the device's own dialog for such an item and resolves on the UI thread with the secret
/// (docs/async.md). The stored-keys line is `entries()`, the listing that needs no dialog.
fn keychain_section() -> impl Piece {
    let key = Signal::new(String::new());
    let secret = Signal::new(String::new());
    let protection = Signal::new(Protection::WhenUnlocked);
    let result = Signal::new(crate::res::str::keychain_idle().format());
    let entries = Signal::new(entries_text());
    let support = crate::support::keychain();
    let refresh = move || entries.set(entries_text());
    // Empty means the placeholder's key, which is what the field shows.
    let key_of = move || {
        let typed = key.with(|k| k.trim().to_string());
        if typed.is_empty() {
            crate::res::str::keychain_key_placeholder().format()
        } else {
            typed
        }
    };

    section((
        crate::widgets::support_note(support),
        label(crate::res::str::keychain_caption()).font(Font::Footnote),
        labeled(
            crate::res::str::keychain_support_label(),
            label(support_text(support)).id(day::id!(keychain_support)),
        ),
        // A full line rather than a labeled row: the store's name is long enough to wrap in a
        // phone-width value column.
        label(format!(
            "{}: {}",
            crate::res::str::keychain_backend_label().format(),
            day_part_keychain::backend()
        ))
        .font(Font::Footnote)
        .id(day::id!(keychain_backend)),
        labeled(
            crate::res::str::keychain_key_label(),
            text_field(key)
                .placeholder(crate::res::str::keychain_key_placeholder())
                .id(day::id!(keychain_key)),
        ),
        labeled(
            crate::res::str::keychain_secret_label(),
            secure_field(secret)
                .placeholder(crate::res::str::keychain_secret_placeholder())
                .id(day::id!(keychain_secret)),
        ),
        labeled(
            crate::res::str::keychain_protection_label(),
            picker(
                protection,
                [
                    (
                        Protection::WhenUnlocked,
                        crate::res::str::keychain_level_unlocked(),
                    ),
                    (
                        Protection::AfterFirstUnlock,
                        crate::res::str::keychain_level_first_unlock(),
                    ),
                    (
                        Protection::UserPresence,
                        crate::res::str::keychain_level_presence(),
                    ),
                ],
            )
            .menu()
            .id(day::id!(keychain_protection)),
        ),
        // Four buttons outgrow a phone-portrait row, so they wrap in two pairs on a Compact
        // window (docs/size-classes.md) and sit in one row elsewhere.
        adaptive((
            row((
                button(crate::res::str::keychain_save())
                    .action(move || {
                        let k = key_of();
                        let value = Secret::from(secret.with(|s| s.clone()));
                        result.set(match day_part_keychain::set(&k, &value, protection.get()) {
                            Ok(()) => crate::res::str::keychain_saved(k).format(),
                            Err(e) => e.to_string(),
                        });
                        refresh();
                    })
                    .tint(crate::widgets::primary())
                    .id(day::id!(keychain_save)),
                button(crate::res::str::keychain_read())
                    .bordered()
                    .action(move || {
                        result.set(match day_part_keychain::get(&key_of()) {
                            Ok(s) => crate::res::str::keychain_read_value(
                                s.as_str().unwrap_or("<binary>").to_string(),
                            )
                            .format(),
                            Err(e) => e.to_string(),
                        });
                    })
                    .id(day::id!(keychain_read)),
            ))
            .spacing(Space::S),
            row((
                button(crate::res::str::keychain_read_presence())
                    .bordered()
                    .action(move || {
                        let k = key_of();
                        let reason = crate::res::str::keychain_presence_reason().format();
                        // The future shows the device's dialog when the item needs it and resumes
                        // here, on the UI thread, with the secret or the reason there is none.
                        day::task(async move {
                            let outcome = day_part_keychain::get_future(&k, &reason).await;
                            result.set(match outcome {
                                Ok(s) => crate::res::str::keychain_read_value(
                                    s.as_str().unwrap_or("<binary>").to_string(),
                                )
                                .format(),
                                Err(e) => e.to_string(),
                            });
                        });
                    })
                    .id(day::id!(keychain_read_presence)),
                button(crate::res::str::keychain_remove())
                    .bordered()
                    .action(move || {
                        let k = key_of();
                        result.set(match day_part_keychain::remove(&k) {
                            Ok(()) => crate::res::str::keychain_removed(k).format(),
                            Err(e) => e.to_string(),
                        });
                        refresh();
                    })
                    .id(day::id!(keychain_remove)),
            ))
            .spacing(Space::S),
        )),
        // The outcome and the listing take whole lines: an error message or a list of keys is
        // longer than a phone's value column.
        label(move || result.get())
            .font(Font::Footnote)
            .id(day::id!(keychain_result)),
        label(move || {
            format!(
                "{}: {}",
                crate::res::str::keychain_entries_label().format(),
                entries.get()
            )
        })
        .font(Font::Footnote)
        .id(day::id!(keychain_entries)),
    ))
    .title(crate::res::str::keychain_title())
}

/// The biometric prompt: the two probes, a reason, the passcode switch, and the dialog itself.
///
/// The probes are the part's answers for this device at this moment (`availability`, `biometry`);
/// the result line is the future resolving on the UI thread with how the person passed, or why
/// they did not. Cancel dismisses a dialog on screen, whose caller then hears `SystemCancelled`.
fn biometrics_section() -> impl Piece {
    let reason = Signal::new(String::new());
    let credential = Signal::new(true);
    let result = Signal::new(crate::res::str::biometrics_idle().format());
    let support = crate::support::biometrics();

    section((
        crate::widgets::support_note(support),
        label(crate::res::str::biometrics_caption()).font(Font::Footnote),
        labeled(
            crate::res::str::biometrics_support_label(),
            label(support_text(support)).id(day::id!(biometrics_support)),
        ),
        labeled(
            crate::res::str::biometrics_availability_label(),
            label(format!("{:?}", day_part_biometrics::availability()))
                .id(day::id!(biometrics_availability)),
        ),
        labeled(
            crate::res::str::biometrics_biometry_label(),
            label(format!("{:?}", day_part_biometrics::biometry()))
                .id(day::id!(biometrics_biometry)),
        ),
        text_field(reason)
            .placeholder(crate::res::str::biometrics_reason_placeholder())
            .id(day::id!(biometrics_reason)),
        labeled(
            crate::res::str::biometrics_credential_label(),
            toggle(credential).id(day::id!(biometrics_credential)),
        ),
        row((
            button(crate::res::str::biometrics_authenticate())
                .action(move || {
                    let typed = reason.with(|r| r.trim().to_string());
                    let text = if typed.is_empty() {
                        crate::res::str::biometrics_reason_placeholder().format()
                    } else {
                        typed
                    };
                    let prompt = Prompt::new(text).device_credential(credential.get());
                    result.set(crate::res::str::biometrics_prompting().format());
                    day::task(async move {
                        let outcome = day_part_biometrics::authenticate_future(prompt).await;
                        result.set(match outcome {
                            Ok(v) => {
                                crate::res::str::biometrics_verified(format!("{:?}", v.method))
                                    .format()
                            }
                            Err(e) => e.to_string(),
                        });
                    });
                })
                .tint(crate::widgets::primary())
                .id(day::id!(biometrics_authenticate)),
            button(crate::res::str::biometrics_cancel())
                .bordered()
                .action(day_part_biometrics::cancel)
                .id(day::id!(biometrics_cancel)),
        ))
        .spacing(Space::S),
        label(move || result.get())
            .font(Font::Footnote)
            .id(day::id!(biometrics_result)),
    ))
    .title(crate::res::str::biometrics_title())
}
