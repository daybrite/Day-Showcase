use std::time::Duration;

use crate::res::str as tr;
use crate::widgets::page;
use day::prelude::*;
use day_part_keychain::{Protection, Secret};
use day_part_push::{
    self as push,
    relay::{Client, Environment, Registration},
};

const RELAY: &str = "https://push.daybrite.cloud";
const REGISTRATION: &str = "day.showcase.push.registration";
/// How long the delayed send waits at the relay: enough to leave the app or lock the device.
const DELAY_SECONDS: u64 = 10;
/// How long a first send waits for the platform to hand over a device token.
const TOKEN_WAIT_MS: u32 = 20_000;

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
fn failed(error: push::Error) -> String {
    match error {
        push::Error::PermissionDenied => tr::push_denied().format(),
        push::Error::Unsupported => tr::push_unavailable().format(),
        push::Error::RelayStatus(429) => tr::push_too_soon().format(),
        error => tr::push_error(error.to_string()).format(),
    }
}

/// The page's shared state: both sections read and drive the same registration.
#[derive(Clone, Copy)]
struct Push {
    token: Signal<Option<push::Token>>,
    event: Signal<Option<push::Event>>,
    registration: Signal<Option<Registration>>,
    code: Signal<String>,
    sandbox: Signal<bool>,
    status: Signal<String>,
    busy: Signal<bool>,
}

impl Push {
    fn environment(self, tracked: bool) -> Environment {
        let sandbox = if tracked {
            self.sandbox.get()
        } else {
            self.sandbox.get_untracked()
        };
        if sandbox {
            Environment::Sandbox
        } else {
            Environment::Production
        }
    }

    /// Everything one tap needs: notification permission, a device token, a relay registration
    /// (first time only, from the enrollment code), then the send itself.
    async fn send(self, delay: u64) -> Result<String, String> {
        let token = match self.token.get_untracked() {
            Some(token) => token,
            None => {
                push::request_permission().await.map_err(failed)?;
                self.status.set(tr::push_requested().format());
                push::register().await.map_err(failed)?;
                self.wait_for_token()
                    .await
                    .ok_or_else(|| tr::push_no_token().format())?
            }
        };
        let enrolled = match self.registration.get_untracked() {
            Some(enrolled) => enrolled,
            None => {
                let code = self.code.get_untracked();
                if code.trim().is_empty() {
                    return Err(tr::push_need_code().format());
                }
                let enrolled = client()
                    .enroll(code.trim(), &token, self.environment(false))
                    .await
                    .map_err(failed)?;
                if let Err(error) = remember(&enrolled) {
                    let _ = client().remove(&enrolled).await;
                    return Err(tr::push_error(error).format());
                }
                self.registration.set(Some(enrolled.clone()));
                self.code.set(String::new());
                enrolled
            }
        };
        let title = tr::push_sample_title().format();
        let body = tr::push_sample_body().format();
        client()
            .send_test_after(&enrolled, &title, &body, Duration::from_secs(delay))
            .await
            .map_err(failed)?;
        Ok(if delay == 0 {
            tr::push_accepted().format()
        } else {
            tr::push_scheduled(delay as i64).format()
        })
    }

    /// The token arrives as an event after `register` returns; poll the signal the watch fills.
    async fn wait_for_token(self) -> Option<push::Token> {
        for _ in 0..TOKEN_WAIT_MS / 250 {
            if let Some(token) = self.token.get_untracked() {
                return Some(token);
            }
            day::sleep(250).await;
        }
        None
    }

    fn start_send(self, delay: u64) {
        self.busy.set(true);
        self.status.set(tr::push_sending().format());
        page_task(async move {
            let outcome = self.send(delay).await;
            self.status.set(outcome.unwrap_or_else(|error| error));
            self.busy.set(false);
        });
    }
}

/// Push notifications (day-part-push, docs/push.md in that repository): a notification sent
/// from a server through APNs or FCM rather than posted by the app. One tap sends one through
/// the Day demo relay; the delayed send leaves time to background the app and see how the
/// notification arrives when the app is not in front. The first send also asks for permission,
/// waits for the device token, and registers this device with the relay using a one-use
/// enrollment code from the relay's administrator. Advanced holds the token, the APNs
/// environment, the raw payload of the last notification, and removal from the relay.
pub(crate) fn push_page() -> AnyPiece {
    let state = Push {
        token: Signal::new(None),
        event: Signal::new(None),
        registration: Signal::new(saved()),
        code: Signal::new(String::new()),
        sandbox: Signal::new(cfg!(debug_assertions)),
        status: Signal::new(String::new()),
        busy: Signal::new(false),
    };
    let token_setter = state.token.setter();
    let event_setter = state.event.setter();
    if let Ok(watch) = push::watch(move |value| {
        if let push::Event::Token { token } = &value {
            token_setter.set(Some(token.clone()));
        }
        event_setter.set(Some(value));
    }) {
        Scope::current().on_cleanup(move || drop(watch));
    }
    keep_token_current(state);

    page(
        tr::nav_push(),
        day::id!(push_title),
        form((send_section(state), advanced_section(state))).any(),
    )
    .any()
}

/// Keep an enrolled device's server token current after provider rotation or app restart.
fn keep_token_current(state: Push) {
    let last_sync = std::cell::RefCell::new(String::new());
    Effect::new(move || {
        if state.busy.get() {
            return;
        }
        let environment = state.environment(true);
        if let (Some(current), Some(enrolled)) = (state.token.get(), state.registration.get()) {
            let snapshot =
                serde_json::to_string(&(&current, enrolled.id(), environment)).unwrap_or_default();
            if *last_sync.borrow() == snapshot {
                return;
            }
            *last_sync.borrow_mut() = snapshot;
            state.busy.set(true);
            page_task(async move {
                if let Err(error) = client().update(&enrolled, &current, environment).await {
                    state.status.set(failed(error));
                }
                state.busy.set(false);
            });
        }
    });
}

fn send_section(state: Push) -> impl Piece {
    let ready = move || push::supported() && !state.busy.get();
    section((
        label(if push::supported() {
            tr::push_caption()
        } else {
            tr::push_unavailable()
        })
        .font(Font::Footnote)
        .id(day::id!(push_support)),
        // Needed once per device: the relay registers only devices its administrator invited.
        when(
            move || state.registration.get().is_none(),
            move || {
                text_field(state.code)
                    .placeholder(tr::push_code())
                    .a11y(|a| a.label(tr::push_code()))
                    .enabled(ready)
                    .id(day::id!(push_code))
                    .any()
            },
        ),
        row((
            button(tr::push_send_now())
                .tint(crate::widgets::primary())
                .enabled(ready)
                .action(move || state.start_send(0))
                .id(day::id!(push_send)),
            button(tr::push_send_later(DELAY_SECONDS as i64))
                .bordered()
                .enabled(ready)
                .action(move || state.start_send(DELAY_SECONDS))
                .id(day::id!(push_send_later)),
        ))
        .spacing(Space::S),
        label(move || {
            let status = state.status.get();
            if status.is_empty() {
                tr::push_hint().format()
            } else {
                status
            }
        })
        .id(day::id!(push_status)),
        // A notification that arrives while the app is in front shows no system banner, so the
        // page says it came; one tapped from the background says it opened the app.
        when(
            move || matches!(state.event.get(), Some(push::Event::Message { .. })),
            move || {
                label(move || match state.event.get() {
                    Some(push::Event::Message { payload, opened }) => {
                        let title = ["/notification/title", "/aps/alert/title"]
                            .iter()
                            .find_map(|path| payload.pointer(path)?.as_str())
                            .unwrap_or_default()
                            .to_string();
                        if opened {
                            tr::push_opened(title).format()
                        } else {
                            tr::push_received(title).format()
                        }
                    }
                    _ => String::new(),
                })
                .id(day::id!(push_last))
                .any()
            },
        ),
    ))
}

fn advanced_section(state: Push) -> impl Piece {
    section((
        labeled(
            tr::push_token(),
            label(move || match state.token.get() {
                Some(token) => match token.provider {
                    push::Provider::Apns => tr::push_token_apns().format(),
                    push::Provider::Fcm => tr::push_token_fcm().format(),
                },
                None => tr::push_token_none().format(),
            })
            .id(day::id!(push_token_state)),
        ),
        labeled(
            tr::push_relay(),
            label(move || match state.registration.get() {
                Some(enrolled) => enrolled.id().to_string(),
                None => tr::push_relay_none().format(),
            })
            .id(day::id!(push_registration)),
        ),
        labeled(
            tr::push_sandbox(),
            toggle(state.sandbox)
                .enabled(move || cfg!(target_os = "ios") && !state.busy.get())
                .id(day::id!(push_sandbox)),
        ),
        label(move || match state.event.get() {
            Some(push::Event::Message { payload, .. }) => {
                serde_json::to_string_pretty(&payload).unwrap_or_default()
            }
            Some(push::Event::Error { message }) => tr::push_error(message).format(),
            _ => tr::push_waiting().format(),
        })
        .font(Font::Footnote)
        .id(day::id!(push_message)),
        button(tr::push_remove())
            .id(day::id!(push_remove))
            .enabled(move || !state.busy.get() && state.registration.get().is_some())
            .action(move || {
                let enrolled = state.registration.get_untracked();
                state.busy.set(true);
                page_task(async move {
                    if let Some(enrolled) = enrolled {
                        match client().remove(&enrolled).await {
                            // The provider may already have invalidated and removed this
                            // registration.
                            Ok(()) | Err(push::Error::RelayStatus(401)) => {}
                            Err(error) => {
                                state.status.set(failed(error));
                                state.busy.set(false);
                                return;
                            }
                        }
                    }
                    if let Err(error) = day_part_keychain::remove(REGISTRATION) {
                        state.status.set(tr::push_error(error.to_string()).format());
                        state.busy.set(false);
                        return;
                    }
                    state.registration.set(None);
                    state.status.set(tr::push_removed().format());
                    state.busy.set(false);
                });
            }),
    ))
    .title(tr::push_advanced())
}
