use crate::res::str as tr;
use day::prelude::*;
use day_part_keychain::{Protection, Secret};
use day_part_push::{
    self as push,
    relay::{Client, Environment, Registration},
};

const RELAY: &str = "https://push.daybrite.cloud";
const REGISTRATION: &str = "day.showcase.push.registration";

fn saved() -> Option<Registration> {
    let secret = day_part_keychain::get(REGISTRATION).ok()?;
    serde_json::from_slice(secret.as_bytes()).ok()
}
fn remember(registration: &Registration) -> Result<(), String> {
    let value = serde_json::to_string(registration).map_err(|e| e.to_string())?;
    day_part_keychain::set(REGISTRATION, &Secret::from(value), Protection::WhenUnlocked)
        .map_err(|e| e.to_string())
}
fn client() -> Client {
    Client::new(RELAY).expect("configured HTTPS relay origin")
}
fn page_task(future: impl std::future::Future<Output = ()> + 'static) {
    let task = day::task(future);
    Scope::current().on_cleanup(move || task.abort());
}

pub(crate) fn push_section() -> impl Piece {
    let token = Signal::new(None::<push::Token>);
    let event = Signal::new(None::<push::Event>);
    let registration = Signal::new(saved());
    let code = Signal::new(String::new());
    let sandbox = Signal::new(cfg!(debug_assertions));
    let status = Signal::new(String::new());
    let busy = Signal::new(false);
    let last_sync = std::cell::RefCell::new(String::new());
    let token_setter = token.setter();
    let event_setter = event.setter();
    if let Ok(watch) = push::watch(move |value| {
        if let push::Event::Token { token } = &value {
            token_setter.set(Some(token.clone()));
        }
        event_setter.set(Some(value));
    }) {
        Scope::current().on_cleanup(move || drop(watch));
    }

    // Keep an enrolled device's server token current after provider rotation or app restart.
    Effect::new(move || {
        if busy.get() {
            return;
        }
        let current = token.get();
        let enrolled = registration.get();
        let environment = if sandbox.get() {
            Environment::Sandbox
        } else {
            Environment::Production
        };
        if let (Some(current), Some(enrolled)) = (current, enrolled) {
            let snapshot =
                serde_json::to_string(&(&current, enrolled.id(), environment)).unwrap_or_default();
            if *last_sync.borrow() == snapshot {
                return;
            }
            *last_sync.borrow_mut() = snapshot;
            busy.set(true);
            page_task(async move {
                if let Err(error) = client().update(&enrolled, &current, environment).await {
                    status.set(tr::push_error(error.to_string()).format());
                }
                busy.set(false);
            });
        }
    });

    section((
        label(tr::push_caption()).font(Font::Footnote),
        label(if push::supported() {
            tr::push_supported()
        } else {
            tr::push_unavailable()
        })
        .id(day::id!(push_support)),
        button(tr::push_enable())
            .id(day::id!(push_enable))
            .enabled(move || push::supported() && !busy.get())
            .action(move || {
                busy.set(true);
                page_task(async move {
                    let result = async {
                        push::request_permission().await?;
                        push::register().await
                    }
                    .await;
                    status.set(match result {
                        Ok(()) => tr::push_requested().format(),
                        Err(error) => tr::push_error(error.to_string()).format(),
                    });
                    busy.set(false);
                });
            }),
        label(move || {
            if token.get().is_some() {
                tr::push_token_ready()
            } else {
                tr::push_no_token()
            }
        })
        .id(day::id!(push_token_state)),
        text_field(code)
            .placeholder(tr::push_code())
            .a11y(|a| a.label(tr::push_code()))
            .id(day::id!(push_code)),
        labeled(
            tr::push_sandbox(),
            toggle(sandbox)
                .enabled(move || cfg!(target_os = "ios") && !busy.get())
                .id(day::id!(push_sandbox)),
        ),
        button(tr::push_enroll())
            .id(day::id!(push_enroll))
            .enabled(move || !busy.get() && token.get().is_some() && registration.get().is_none())
            .action(move || {
                let Some(current) = token.get_untracked() else {
                    status.set(tr::push_no_token().format());
                    return;
                };
                let enrollment = code.get_untracked();
                let environment = if sandbox.get_untracked() {
                    Environment::Sandbox
                } else {
                    Environment::Production
                };
                busy.set(true);
                page_task(async move {
                    match client()
                        .enroll(enrollment.trim(), &current, environment)
                        .await
                    {
                        Ok(enrolled) => match remember(&enrolled) {
                            Ok(()) => {
                                registration.set(Some(enrolled));
                                code.set(String::new());
                                status.set(tr::push_enrolled().format());
                            }
                            Err(error) => {
                                let _ = client().remove(&enrolled).await;
                                status.set(tr::push_error(error).format());
                            }
                        },
                        Err(error) => status.set(tr::push_error(error.to_string()).format()),
                    }
                    busy.set(false);
                });
            }),
        button(tr::push_send())
            .id(day::id!(push_send))
            .enabled(move || !busy.get() && registration.get().is_some())
            .action(move || {
                let Some(enrolled) = registration.get_untracked() else {
                    status.set(tr::push_need_enrollment().format());
                    return;
                };
                let title = tr::push_sample_title().format();
                let body = tr::push_sample_body().format();
                busy.set(true);
                page_task(async move {
                    status.set(match client().send_test(&enrolled, &title, &body).await {
                        Ok(()) => tr::push_accepted().format(),
                        Err(error) => tr::push_error(error.to_string()).format(),
                    });
                    busy.set(false);
                });
            }),
        button(tr::push_remove())
            .id(day::id!(push_remove))
            .enabled(move || !busy.get() && registration.get().is_some())
            .action(move || {
                let enrolled = registration.get_untracked();
                busy.set(true);
                page_task(async move {
                    if let Some(enrolled) = enrolled {
                        match client().remove(&enrolled).await {
                            // The provider may already have invalidated and removed this registration.
                            Ok(()) | Err(push::Error::RelayStatus(401)) => {}
                            Err(error) => {
                                status.set(tr::push_error(error.to_string()).format());
                                busy.set(false);
                                return;
                            }
                        }
                    }
                    if let Err(error) = day_part_keychain::remove(REGISTRATION) {
                        status.set(tr::push_error(error.to_string()).format());
                        busy.set(false);
                        return;
                    }
                    registration.set(None);
                    status.set(tr::push_removed().format());
                    busy.set(false);
                });
            }),
        label(status).id(day::id!(push_status)),
        label(move || match event.get() {
            Some(push::Event::Message { payload, opened }) => {
                let value = serde_json::to_string_pretty(&payload).unwrap_or_default();
                if opened {
                    tr::push_opened(value).format()
                } else {
                    tr::push_received(value).format()
                }
            }
            Some(push::Event::Error { message }) => tr::push_error(message).format(),
            _ => tr::push_waiting().format(),
        })
        .id(day::id!(push_message)),
    ))
    .title(tr::push_heading())
}
