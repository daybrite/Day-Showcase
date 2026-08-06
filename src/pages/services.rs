use day::prelude::*;
use day_part_haptics::Haptic;
use day_part_local_notify::{Channel, Importance, Notification, Trigger};

use crate::widgets::page;

/// Platform services (docs/http.md, docs/clipboard.md, docs/prefs.md, docs/haptics.md,
/// docs/files.md, docs/notify.md): the headless "do something with the OS" parts, one grouped
/// form section each — an HTTP fetch (first: it works on every target, including web-dom),
/// clipboard round-trip, persisted preferences, haptic feedback, local notifications, and the
/// native file pickers.
pub(crate) fn services_page() -> AnyPiece {
    page(
        crate::res::str::nav_services(),
        "services-title",
        Some(crate::res::str::services_caption()),
        form((
            http_section(),
            clipboard_section(),
            prefs_section(),
            haptics_section(),
            notify_section(),
            files_section(),
            storage_section(),
        ))
        .any(),
    )
}

fn clipboard_section() -> impl Piece {
    let draft = Signal::new(String::new());
    let pasted = Signal::new(String::new());
    let status = Signal::new(crate::res::str::clipboard_idle().format());
    section((
        label(crate::res::str::clipboard_caption()).font(Font::Footnote),
        text_field(draft)
            .placeholder(crate::res::str::clipboard_placeholder())
            .id("clipboard-field"),
        row((
            button(crate::res::str::clipboard_copy())
                .bordered()
                .action(move || {
                    let ok = draft.with(|t| day_part_clipboard::set_text(t));
                    let msg = if ok {
                        crate::res::str::clipboard_copied()
                    } else {
                        crate::res::str::clipboard_copy_failed()
                    };
                    status.set(msg.format());
                })
                .id("clipboard-copy"),
            button(crate::res::str::clipboard_paste())
                .bordered()
                .action(move || match day_part_clipboard::get_text() {
                    Some(text) => {
                        pasted.set(text);
                        status.set(crate::res::str::clipboard_pasted().format());
                    }
                    None => status.set(crate::res::str::clipboard_empty().format()),
                })
                .id("clipboard-paste"),
            label(move || status.get()).id("clipboard-status"),
        ))
        .spacing(8.0),
        label(move || pasted.get()).id("clipboard-pasted"),
    ))
    .title(crate::res::str::nav_clipboard())
}

fn prefs_section() -> impl Piece {
    const KEY: &str = "showcase.remembered";
    let field = Signal::new(String::new());
    let value = Signal::new(crate::res::str::prefs_empty().format());
    let status = Signal::new(crate::res::str::prefs_idle().format());
    section((
        label(crate::res::str::prefs_caption()).font(Font::Footnote),
        text_field(field)
            .placeholder(crate::res::str::prefs_placeholder())
            .id("prefs-field"),
        row((
            button(crate::res::str::prefs_save())
                .bordered()
                .action(move || {
                    let ok = field.with(|t| day::prefs::set(KEY, t));
                    let msg = if ok {
                        crate::res::str::prefs_saved()
                    } else {
                        crate::res::str::prefs_save_failed()
                    };
                    status.set(msg.format());
                })
                .id("prefs-save"),
            button(crate::res::str::prefs_load())
                .bordered()
                .action(move || match day::prefs::get(KEY) {
                    Some(v) => {
                        value.set(v);
                        status.set(crate::res::str::prefs_loaded().format());
                    }
                    None => {
                        value.set(crate::res::str::prefs_empty().format());
                        status.set(crate::res::str::prefs_missing().format());
                    }
                })
                .id("prefs-load"),
            button(crate::res::str::prefs_clear())
                .bordered()
                .action(move || {
                    day::prefs::remove(KEY);
                    value.set(crate::res::str::prefs_empty().format());
                    status.set(crate::res::str::prefs_cleared().format());
                })
                .id("prefs-clear"),
            label(move || status.get()).id("prefs-status"),
        ))
        .spacing(8.0),
        labeled(
            crate::res::str::prefs_value_label(),
            label(move || value.get()).id("prefs-value"),
        ),
    ))
    .title(crate::res::str::nav_prefs())
}

/// One button that plays a haptic and records the style name into `#haptics-last-played`.
fn haptic_button(
    id: &'static str,
    title: LocalizedText,
    h: Haptic,
    last: Signal<String>,
) -> AnyPiece {
    button(title)
        .bordered()
        .action(move || {
            day_part_haptics::play(h);
            last.set(crate::res::str::haptics_last_played(format!("{h:?}")).format());
        })
        .id(id)
        .any()
}

fn haptics_section() -> impl Piece {
    let last = Signal::new(crate::res::str::haptics_none().format());
    // Report whether this platform has a haptic engine (each branch a full `tr(...)` for `day lint`).
    let supported = if day_part_haptics::is_supported() {
        crate::res::str::haptics_supported_yes()
    } else {
        crate::res::str::haptics_supported_no()
    };
    section((
        label(supported)
            .font(Font::Footnote)
            .id("haptics-supported"),
        row((
            haptic_button(
                "haptics-light",
                crate::res::str::haptics_light(),
                Haptic::Light,
                last,
            ),
            haptic_button(
                "haptics-medium",
                crate::res::str::haptics_medium(),
                Haptic::Medium,
                last,
            ),
            haptic_button(
                "haptics-heavy",
                crate::res::str::haptics_heavy(),
                Haptic::Heavy,
                last,
            ),
        ))
        .spacing(8.0),
        row((
            haptic_button(
                "haptics-success",
                crate::res::str::haptics_success(),
                Haptic::Success,
                last,
            ),
            haptic_button(
                "haptics-warning",
                crate::res::str::haptics_warning(),
                Haptic::Warning,
                last,
            ),
            haptic_button(
                "haptics-error",
                crate::res::str::haptics_error(),
                Haptic::Error,
                last,
            ),
            haptic_button(
                "haptics-selection",
                crate::res::str::haptics_selection(),
                Haptic::Selection,
                last,
            ),
        ))
        .spacing(8.0),
        labeled(
            crate::res::str::haptics_last(),
            label(move || last.get()).id("haptics-last-played"),
        ),
    ))
    .title(crate::res::str::nav_haptics())
}

/// Local notifications (docs/notify.md). The controls cover what the API actually varies —
/// message, delay, importance, sound, badge, and the route a tap opens — and the capability lines
/// are first, because the honest answer differs per platform: Apple and Android hand a scheduled
/// notification to the OS, while Linux and the web run an in-process timer that dies with the app.
///
/// Importance is fixed when a channel is registered (Android's `NotificationChannel` is immutable
/// after first use), so the page registers ONE CHANNEL PER LEVEL up front and the picker chooses
/// between them rather than mutating one.
fn notify_section() -> impl Piece {
    let caps = day_part_local_notify::capabilities();
    let levels = [
        Importance::Low,
        Importance::Default,
        Importance::High,
        Importance::Urgent,
    ];

    let title = Signal::new(crate::res::str::notify_title_default().format());
    let body = Signal::new(crate::res::str::notify_body_default().format());
    let delay_idx = Signal::new(0usize);
    // High, not Default: Android only shows a heads-up banner from IMPORTANCE_HIGH up, and a
    // notification that lands silently in the shade reads as "the button did nothing".
    let level_idx = Signal::new(2usize);
    let sound = Signal::new(true);
    let badge = Signal::new(0.0f64);
    let status = Signal::new(crate::res::str::notify_status_idle().format());
    let granted = Signal::new(
        day_part_permissions::status(day_part_permissions::Permission::Notifications)
            == day_part_permissions::Status::Granted,
    );

    let supported = if caps.post {
        crate::res::str::notify_caps_post()
    } else {
        crate::res::str::notify_caps_unsupported()
    };
    let scheduling = if caps.schedule_while_dead {
        crate::res::str::notify_caps_schedule_os()
    } else {
        crate::res::str::notify_caps_schedule_process()
    };

    section((
        label(supported).font(Font::Footnote).id("notify-supported"),
        label(scheduling).font(Font::Footnote).id("notify-scheduling"),
        labeled(
            crate::res::str::notify_title_label(),
            text_field(title)
                .placeholder(crate::res::str::notify_title_placeholder())
                .id("notify-title"),
        ),
        labeled(
            crate::res::str::notify_body_label(),
            text_field(body)
                .placeholder(crate::res::str::notify_body_placeholder())
                .id("notify-body"),
        ),
        labeled(
            crate::res::str::notify_delay(),
            picker(
                vec![
                    crate::res::str::notify_delay_now().format(),
                    crate::res::str::notify_delay_5s().format(),
                    crate::res::str::notify_delay_15s().format(),
                    crate::res::str::notify_delay_60s().format(),
                ],
                delay_idx,
            )
            .id("notify-delay"),
        ),
        labeled(
            crate::res::str::notify_importance(),
            picker(
                vec![
                    crate::res::str::notify_importance_low().format(),
                    crate::res::str::notify_importance_default().format(),
                    crate::res::str::notify_importance_high().format(),
                    crate::res::str::notify_importance_urgent().format(),
                ],
                level_idx,
            )
            .id("notify-importance"),
        ),
        labeled(
            crate::res::str::notify_sound(),
            toggle(sound).id("notify-sound"),
        ),
        // Badge is Apple-only among the wired backends. Day's Decorate trait has no `disabled`,
        // so the control is omitted where it would do nothing rather than shown doing nothing.
        when(
            move || caps.badge,
            move || {
                labeled(
                    crate::res::str::notify_badge(),
                    slider(badge).range(0.0..=9.0).step(1.0).id("notify-badge"),
                )
            },
        ),
        // The consent line, ahead of the controls: on Apple an unauthorized post is accepted and
        // then dropped by the system with no error, so without this the page would look broken.
        row((
            label(move || {
                if granted.get() {
                    crate::res::str::notify_perm_granted().format()
                } else {
                    crate::res::str::notify_perm_missing().format()
                }
            })
            .font(Font::Footnote)
            .id("notify-perm"),
            button(crate::res::str::notify_perm_request())
                .bordered()
                .action(move || {
                    // The callback can land on another thread, and Signal is !Send — a Setter is
                    // the sanctioned cross-thread door (DESIGN §3.3).
                    let set = granted.setter();
                    day_part_permissions::request(
                        day_part_permissions::Permission::Notifications,
                        move |s| set.set(s == day_part_permissions::Status::Granted),
                    );
                })
                .id("notify-perm-request"),
        ))
        .spacing(8.0),
        row((
            button(crate::res::str::notify_post())
                .prominent()
                .action(move || {
                    let level = levels[level_idx.get().min(levels.len() - 1)];
                    // Re-register on every post so the sound toggle takes effect; registration is
                    // idempotent, and each level keeps its own channel id.
                    let chan = format!("showcase-{}", level.as_str());
                    Channel::new(chan.clone(), level)
                        .sound(sound.get())
                        .register();
                    let secs = [0u64, 5, 15, 60][delay_idx.get().min(3)];
                    let trigger = if secs == 0 {
                        Trigger::Now
                    } else {
                        Trigger::In(std::time::Duration::from_secs(secs))
                    };
                    let mut n = Notification::new(title.get())
                        .body(body.get())
                        .channel(chan)
                        .route("services")
                        .trigger(trigger);
                    let count = badge.get() as u32;
                    if count > 0 {
                        n = n.badge(count);
                    }
                    let msg = match n.post() {
                        Ok(id) => {
                            let head = if secs == 0 {
                                crate::res::str::notify_status_posted()
                            } else {
                                crate::res::str::notify_status_scheduled()
                            };
                            format!("{} (#{})", head.format(), id.0)
                        }
                        Err(e) => format!(
                            "{}: {e}",
                            crate::res::str::notify_status_failed().format()
                        ),
                    };
                    status.set(msg);
                })
                .id("notify-post"),
            button(crate::res::str::notify_cancel())
                .bordered()
                .action(move || {
                    day_part_local_notify::cancel_all();
                    status.set(crate::res::str::notify_status_cancelled().format());
                })
                .id("notify-cancel"),
        ))
        .spacing(8.0),
        labeled(
            crate::res::str::notify_last(),
            label(move || status.get()).id("notify-status"),
        ),
    ))
    .title(crate::res::str::nav_notify())
}

fn files_section() -> impl Piece {
    // The editor text: what "Save" writes and what "Open" loads into.
    let content = Signal::new(crate::res::str::files_initial_content().format());
    let status = Signal::new(String::new());
    let opened = Signal::new(String::new());
    section((
        label(crate::res::str::files_caption()).font(Font::Footnote),
        text_field(content)
            .placeholder(crate::res::str::files_placeholder())
            .id("files-content"),
        row((
            button(crate::res::str::files_open())
                .bordered()
                .action(move || {
                    day::task(async move {
                        match open_file()
                            .title(crate::res::str::files_open())
                            .filter("Text", &["txt", "md"])
                            .await
                        {
                            Some(file) => match file.read_to_string() {
                                Ok(text) => {
                                    content.set(text);
                                    opened.set(file.file_name().unwrap_or_default());
                                    status.set("opened".into());
                                }
                                Err(_) => status.set("open-error".into()),
                            },
                            None => status.set("open-cancel".into()),
                        }
                    });
                })
                .id("btn-open-file"),
            button(crate::res::str::files_save())
                .bordered()
                .action(move || {
                    day::task(async move {
                        let data = content.get_untracked().into_bytes();
                        match save_file(data)
                            .title(crate::res::str::files_save())
                            .suggested_name("day-notes.txt")
                            .filter("Text", &["txt"])
                            .await
                        {
                            Some(dest) => status
                                .set(format!("saved:{}", dest.file_name().unwrap_or_default())),
                            None => status.set("save-cancel".into()),
                        }
                    });
                })
                .id("btn-save-file"),
            label(move || status.get()).id("files-status"),
        ))
        .spacing(8.0),
        when(
            move || !opened.with(|s| s.is_empty()),
            move || label(crate::res::str::files_opened(opened)).id("files-opened-name"),
        ),
    ))
    .title(crate::res::str::nav_files())
}

/// The demo's target URL. Native targets spin the one-shot loopback server below; the web
/// (web-dom) instead fetches the same-origin `day-http-ok` path — a browser tab can host no
/// TCP listener, and `day launch`'s dev server answers that path with the identical bodies
/// (crates/day-cli/src/web.rs), so the walkthrough asserts the same bytes everywhere. On a
/// static host without the endpoint the buttons report the server's honest error instead.
fn demo_url() -> Result<String, String> {
    #[cfg(target_arch = "wasm32")]
    {
        // Relative on purpose: resolves against the page origin (and subpath, e.g. the
        // project-Pages /Day-Showcase/), keeping the request same-origin — no CORS.
        Ok("day-http-ok".into())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        serve_once().map_err(|e| e.to_string())
    }
}

/// One-shot loopback server answering `200` — the demo needs no external network, so it behaves
/// the same in airplane mode, on CI, and behind a proxy. GET keeps the historic `day-http-ok`
/// body (walkthrough-asserted, byte-identical); any other method echoes it as
/// `day-http-ok:<METHOD>` — the deterministic proof that e.g. PATCH crossed the platform engine.
#[cfg(not(target_arch = "wasm32"))]
fn serve_once() -> std::io::Result<String> {
    use std::io::{Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    std::thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0u8; 2048];
            let n = stream.read(&mut buf).unwrap_or(0);
            let head = String::from_utf8_lossy(&buf[..n]);
            let method = head.split_whitespace().next().unwrap_or("GET").to_string();
            let body = if method == "GET" {
                "day-http-ok".to_string()
            } else {
                format!("day-http-ok:{method}")
            };
            let _ = stream.write_all(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                )
                .as_bytes(),
            );
        }
    });
    Ok(format!("http://127.0.0.1:{port}/"))
}

fn http_section() -> impl Piece {
    let status = Signal::new(crate::res::str::http_idle().format());
    // The callback idiom (docs/http.md): fetch_async completes on a BACKGROUND thread (the
    // sole browser thread on web); the
    // captured Setter hops to the UI thread itself and no-ops if the page is gone. Kept as the
    // living Setter example — the rows below use the newer await/Resource rails (docs/async.md).
    let done = status.setter();
    let patch_status = Signal::new(crate::res::str::http_idle().format());
    section((
        label(crate::res::str::http_caption()).font(Font::Footnote),
        row((
            button(crate::res::str::http_fetch())
                .bordered()
                .action(move || match demo_url() {
                    Ok(url) => day_part_http::fetch_async(
                        day_part_http::Request::get(url)
                            .timeout(std::time::Duration::from_secs(10)),
                        move |result| {
                            // Raw "<status> <body>" on purpose: identical in every locale, so
                            // the walkthrough can assert it exactly.
                            let text = match result {
                                Ok(resp) => format!("{} {}", resp.status, resp.text()),
                                Err(e) => format!("error: {e}"),
                            };
                            done.set(text);
                        },
                    ),
                    Err(e) => status.set(format!("error: {e}")),
                })
                .id("http-fetch"),
            label(move || status.get()).id("http-status"),
        ))
        .spacing(8.0),
        // PATCH through the same engine, await-style (docs/async.md): the echo body proves the
        // method crossed the platform stack — the historic Android HttpURLConnection gap.
        row((
            button(crate::res::str::http_patch())
                .bordered()
                .action(move || match demo_url() {
                    Ok(url) => {
                        day::task(async move {
                            let req = day_part_http::Request::patch(url, Vec::new())
                                .timeout(std::time::Duration::from_secs(10));
                            let text = match day_part_http::fetch_future(req).await {
                                Ok(resp) => format!("{} {}", resp.status, resp.text()),
                                Err(e) => format!("error: {e}"),
                            };
                            patch_status.set(text);
                        });
                    }
                    Err(e) => patch_status.set(format!("error: {e}")),
                })
                .id("http-patch"),
            label(move || patch_status.get()).id("http-patch-status"),
        ))
        .spacing(8.0),
        http_resource_row(),
        labeled(
            crate::res::str::http_tier(),
            label(day_part_http::tier().label()).id("http-tier"),
        ),
        url_check_field(),
    ))
    .title(crate::res::str::http_title())
}

/// Declarative loading (docs/async.md): a `Resource` wraps "fetch the loopback URL". The
/// attempt counter makes Refetch observable (`ok 1:` → `ok 2:`), and the readout mirrors
/// `Load`'s three states.
fn http_resource_row() -> impl Piece {
    use day::reactive::{Load, Resource};
    let attempts = std::rc::Rc::new(std::cell::Cell::new(0u32));
    let res: Resource<String> = Resource::new(
        || (),
        move |_| {
            attempts.set(attempts.get() + 1);
            let n = attempts.get();
            async move {
                let url = demo_url().map_err(day_part_http::HttpError::Io)?;
                let resp = day_part_http::fetch_future(
                    day_part_http::Request::get(url).timeout(std::time::Duration::from_secs(10)),
                )
                .await?;
                Ok::<_, day_part_http::HttpError>(format!(
                    "ok {n}: {} {}",
                    resp.status,
                    resp.text()
                ))
            }
        },
    );
    labeled(
        crate::res::str::http_res_label(),
        row((
            button(crate::res::str::http_res_refetch())
                .bordered()
                .action(move || res.refetch())
                .id("http-res-refetch"),
            label(move || {
                res.with(|l| match l {
                    Load::Loading => crate::res::str::http_checking().format(),
                    Load::Ready(s) => s.clone(),
                    Load::Failed(e) => format!("error: {e}"),
                })
            })
            .font(Font::Footnote)
            .id("http-res-status"),
        ))
        .spacing(8.0),
    )
}

/// The second half of the HTTP section: type any http(s) URL, tap Check, and read back the
/// response headers plus the body size — a live view of what the platform stack returns
/// (and of platform policy: iOS ATS rejecting a cleartext host shows up here as the error).
fn url_check_field() -> impl Piece {
    // Pre-filled with a host that answers cross-origin requests (httpbin echoes with
    // `Access-Control-Allow-Origin: *`), so Check works out of the box on web-dom too —
    // an arbitrary site would be blocked by CORS in a browser (docs/web.md).
    let url = Signal::new("https://httpbin.org/get".to_string());
    let out = Signal::new(String::new());
    // The in-flight check, if any: re-tapping Check aborts the previous task, which drops its
    // FetchFuture and CANCELS the platform request (docs/async.md's drop-cancel rail) — type a
    // slow URL, tap Check twice, and only the second answer ever lands.
    let inflight: std::rc::Rc<std::cell::Cell<Option<day::TaskHandle>>> =
        std::rc::Rc::new(std::cell::Cell::new(None));
    column((
        text_field(url)
            .placeholder(crate::res::str::http_url_placeholder())
            .id("http-url"),
        button(crate::res::str::http_check())
            .bordered()
            .action(move || {
                let target = url.get_untracked();
                if target.trim().is_empty() {
                    return;
                }
                let req = day_part_http::Request::get(target.trim())
                    .timeout(std::time::Duration::from_secs(15));
                out.set(crate::res::str::http_checking().format());
                if let Some(prev) = inflight.take() {
                    prev.abort();
                }
                let slot = inflight.clone();
                let handle = day::task(async move {
                    // Await-style (docs/async.md): the future resumes on the UI thread, so the
                    // readout is a plain Signal write — no Setter needed.
                    let text = match day_part_http::fetch_future(req).await {
                        // Raw readout on purpose (headers and sizes aren't locale material).
                        Ok(resp) => {
                            let mut s = format!("HTTP {} · {} bytes", resp.status, resp.body.len());
                            for (k, v) in &resp.headers {
                                s.push_str(&format!("\n{k}: {v}"));
                            }
                            s
                        }
                        Err(e) => format!("error: {e}"),
                    };
                    out.set(text);
                    slot.set(None);
                });
                // A synchronously-failed fetch already finished (and cleared the slot) inside
                // task(); storing its handle then is a harmless stale-id miss.
                inflight.set(Some(handle));
            })
            .id("http-check"),
        label(move || out.get())
            .font(Font::Footnote)
            .id("http-headers"),
    ))
    .spacing(8.0)
    // Leading, like the section's other rows — the default centered alignment floated the
    // Check button and readout as islands mid-card on every platform.
    .align(HAlign::Leading)
}

/// App-local file storage (docs/fs.md): day-part-fs write/read/list/remove through the async
/// futures, so the SAME code runs on every target — real files natively, OPFS in the browser.
/// Statuses are raw on purpose (walkthrough-asserted, identical across locales).
fn storage_section() -> impl Piece {
    const FILE: &str = "demo/showcase-note.txt";
    let note = Signal::new(String::new());
    let status = Signal::new(crate::res::str::storage_idle().format());
    let files = Signal::new("\u{2014}".to_string());

    // Re-list the demo directory after every operation ("\u{2014}" = nothing stored).
    let refresh = move || {
        day::task(async move {
            match day_part_fs::list_future("demo").await {
                Ok(names) if names.is_empty() => files.set("\u{2014}".into()),
                Ok(names) => files.set(names.join(", ")),
                Err(e) => files.set(format!("error: {e}")),
            }
        });
    };
    refresh();

    section((
        label(crate::res::str::storage_caption()).font(Font::Footnote),
        text_field(note)
            .placeholder(crate::res::str::storage_placeholder())
            .id("fs-note"),
        row((
            button(crate::res::str::storage_save())
                .bordered()
                .action(move || {
                    let data = note.get_untracked().into_bytes();
                    day::task(async move {
                        let text = match day_part_fs::write_future(FILE, data).await {
                            Ok(()) => "saved".to_string(),
                            Err(e) => format!("error: {e}"),
                        };
                        status.set(text);
                        refresh();
                    });
                })
                .id("fs-save"),
            button(crate::res::str::storage_load())
                .bordered()
                .action(move || {
                    day::task(async move {
                        let text = match day_part_fs::read_future(FILE).await {
                            Ok(bytes) => {
                                format!("loaded:{}", String::from_utf8_lossy(&bytes))
                            }
                            Err(e) => format!("error: {e}"),
                        };
                        status.set(text);
                    });
                })
                .id("fs-load"),
            button(crate::res::str::storage_delete())
                .bordered()
                .action(move || {
                    day::task(async move {
                        let text = match day_part_fs::remove_future(FILE).await {
                            Ok(()) => "deleted".to_string(),
                            Err(e) => format!("error: {e}"),
                        };
                        status.set(text);
                        refresh();
                    });
                })
                .id("fs-delete"),
            label(move || status.get()).id("fs-status"),
        ))
        .spacing(8.0),
        labeled(
            crate::res::str::storage_files_label(),
            label(move || files.get())
                .font(Font::Footnote)
                .id("fs-list"),
        ),
    ))
    .title(crate::res::str::storage_title())
}
