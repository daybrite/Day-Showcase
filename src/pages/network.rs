//! One session, one set of operations: native/reqwest and intercepted responses use the same UI.
use crate::res::str as tr;
use day::prelude::*;
use day_part_http::{
    Client, Cookies, Method, Provider, Request, Session, Statistics,
    simulation::{Conditions, Reply, Simulation},
};
use day_piece_charts::{LegendPosition, chart, line, value};
use std::{cell::RefCell, rc::Rc, time::Duration};

fn base_url() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        static SERVER: std::sync::OnceLock<Result<day_part_http::testing::Server, String>> =
            std::sync::OnceLock::new();
        SERVER
            .get_or_init(|| day_part_http::testing::Server::start().map_err(|e| e.to_string()))
            .as_ref()
            .map(|s| s.url("").trim_end_matches('/').to_owned())
            .unwrap_or_default()
    }
    #[cfg(target_arch = "wasm32")]
    {
        String::new()
    }
}
fn demo_simulation(base: String) -> Simulation {
    let sim = Simulation::new();
    let ws_base = base.replacen("http:", "ws:", 1);
    sim.handle(
        move |_, url| url.starts_with(&base) || url == "day-http-ok",
        |req| {
            let path = req
                .url
                .split_once("://")
                .and_then(|(_, s)| s.find('/').map(|i| &s[i..]))
                .unwrap_or("/");
            if path == "/redirect/2" {
                return Ok(Reply::new(302, Vec::new()).header("Location", "/"));
            }
            let bytes = if path.starts_with("/bytes/") {
                vec![b'x'; 512 * 1024]
            } else if path == "/echo" {
                match req.body {
                    day_part_http::transport::PreparedBody::Bytes(b) => (*b).clone(),
                    _ => Vec::new(),
                }
            } else if req.method == "GET" {
                b"day-http-ok".to_vec()
            } else {
                format!("day-http-ok:{}", req.method).into_bytes()
            };
            let mut reply = Reply::new(200, bytes).header("Content-Type", "text/plain");
            if path.starts_with("/delay/") {
                reply.delay = Duration::from_secs(3);
            }
            Ok(reply)
        },
    );
    sim.websocket(
        move |_, u| u.starts_with(&ws_base) || u == "wss://fixture.example/echo",
        Ok,
    );
    sim
}
#[derive(Clone)]
struct Sample {
    time: f64,
    real_down: f64,
    real_up: f64,
    sim_down: f64,
    sim_up: f64,
}
#[derive(Clone, Copy)]
struct Model {
    session: Signal<Session>,
    simulation: Signal<Simulation>,
    client: Signal<Client>,
    enabled: Signal<bool>,
    speed: Signal<f64>,
    failure: Signal<f64>,
    paused: Signal<bool>,
    offline: Signal<bool>,
    provider: Signal<usize>,
    url: Signal<String>,
    method: Signal<usize>,
    body: Signal<String>,
    headers: Signal<String>,
    response_headers: Signal<String>,
    timeout: Signal<f64>,
    status: Signal<String>,
    received: Signal<u64>,
    statistics: Signal<Statistics>,
    samples: Signal<Vec<Sample>>,
}
pub(crate) fn network_page() -> AnyPiece {
    let base = base_url();
    let session = Session::new();
    let simulation = demo_simulation(base.clone());
    let client = Client::builder()
        .session(session.clone())
        .cookies(Cookies::jar())
        .timeout_total(Duration::from_secs(30))
        .build();
    let st = Model {
        session: Signal::new(session),
        simulation: Signal::new(simulation),
        client: Signal::new(client),
        enabled: Signal::new(false),
        speed: Signal::new(512.0),
        failure: Signal::new(0.0),
        paused: Signal::new(false),
        offline: Signal::new(false),
        provider: Signal::new(0),
        url: Signal::new(if base.is_empty() {
            "day-http-ok".into()
        } else {
            format!("{base}/")
        }),
        method: Signal::new(0),
        body: Signal::new(String::new()),
        headers: Signal::new("{}".into()),
        response_headers: Signal::new(String::new()),
        timeout: Signal::new(10.0),
        status: Signal::new(tr::http_idle().format()),
        received: Signal::new(0),
        statistics: Signal::new(Statistics::default()),
        samples: Signal::new(vec![]),
    };
    Effect::new(move || {
        let sim = st.simulation.get();
        sim.set_conditions(Conditions {
            bytes_per_second: Some((st.speed.get() * 1024.0) as u64),
            failure_rate: st.failure.get() / 100.0,
            paused: st.paused.get(),
            offline: st.offline.get(),
            seed: 42,
            latency: Duration::from_millis(50),
        });
        st.session
            .get()
            .set_simulation(st.enabled.get().then_some(sim));
    });
    Effect::new(move || {
        let selected = st.provider.get();
        #[cfg(all(not(target_arch = "wasm32"), not(target_env = "ohos")))]
        st.session.get().set_provider(if selected == 1 {
            Provider::Reqwest
        } else {
            Provider::Native
        });
        #[cfg(any(target_arch = "wasm32", target_env = "ohos"))]
        {
            let _ = selected;
            st.session.get().set_provider(Provider::Native);
        }
    });
    day::task(async move {
        let mut previous = st.session.get_untracked().statistics();
        let mut time = 0.0;
        loop {
            day::sleep(250).await;
            let next = st.session.get_untracked().statistics();
            let dt = (next.elapsed.saturating_sub(previous.elapsed))
                .as_secs_f64()
                .max(0.001);
            time += dt;
            let down = next
                .downloaded_bytes
                .saturating_sub(previous.downloaded_bytes) as f64
                / dt
                / 1024.0;
            let up =
                next.uploaded_bytes.saturating_sub(previous.uploaded_bytes) as f64 / dt / 1024.0;
            let sim_down =
                next.simulated_downloaded_bytes
                    .saturating_sub(previous.simulated_downloaded_bytes) as f64
                    / dt
                    / 1024.0;
            let sim_up = next
                .simulated_uploaded_bytes
                .saturating_sub(previous.simulated_uploaded_bytes) as f64
                / dt
                / 1024.0;
            let mut samples = st.samples.get_untracked();
            samples.push(Sample {
                time,
                real_down: (down - sim_down).max(0.0),
                real_up: (up - sim_up).max(0.0),
                sim_down,
                sim_up,
            });
            if samples.len() > 120 {
                samples.remove(0);
            }
            st.samples.set(samples);
            st.statistics.set(next.clone());
            previous = next;
        }
    });
    let operation = Signal::new(0usize);
    let ws_base = base.clone();
    crate::widgets::page_wide(
        tr::nav_network_http(),
        "network-title",
        row((
            form((
                section((
                    provider_picker(st),
                    label(tr::net_provider_note()).font(Font::Footnote),
                ))
                .title(tr::net_transport()),
                simulation_section(st),
                super::services::network_section(),
            ))
            .grow_w(),
            form((
                picker(
                    [tr::net_request().format(), tr::ws_title().format()],
                    operation,
                )
                .segmented()
                .id("network-operation"),
                when(
                    move || operation.get() == 0,
                    move || request_section(st, base.clone()),
                ),
                when(
                    move || operation.get() == 1,
                    move || websocket_section(st, ws_base.clone()),
                ),
                statistics_section(st),
            ))
            .grow_w(),
        ))
        .spacing(16.0)
        .align(VAlign::Top)
        .fit(RowFit::ColumnAt(WidthClass::Medium)),
    )
    .any()
}

fn provider_picker(st: Model) -> AnyPiece {
    #[cfg(all(not(target_arch = "wasm32"), not(target_env = "ohos")))]
    {
        labeled(
            tr::net_provider(),
            picker(
                [tr::net_native().format(), tr::net_reqwest().format()],
                st.provider,
            )
            .segmented()
            .id("network-provider"),
        )
        .any()
    }
    #[cfg(any(target_arch = "wasm32", target_env = "ohos"))]
    {
        let _ = st;
        label(tr::net_native_only())
            .id("network-provider-unavailable")
            .any()
    }
}
fn simulation_section(st: Model) -> impl Piece {
    section((
        labeled(
            tr::net_intercept(),
            toggle(st.enabled).id("network-intercept"),
        ),
        label(tr::net_sim_note()).font(Font::Footnote),
        when(
            move || st.enabled.get(),
            move || {
                column((
                    labeled(
                        tr::net_speed(),
                        slider(st.speed).range(16.0..=4096.0).id("network-speed"),
                    ),
                    label(move || tr::net_speed_value(format!("{:.0}", st.speed.get())).format())
                        .id("network-speed-value"),
                    labeled(
                        tr::net_failures(),
                        slider(st.failure).range(0.0..=100.0).id("network-failure"),
                    ),
                    label(move || {
                        tr::net_failure_value(format!("{:.0}", st.failure.get())).format()
                    })
                    .id("network-failure-value"),
                    row((
                        labeled(tr::net_pause(), toggle(st.paused).id("network-pause")),
                        labeled(tr::net_offline(), toggle(st.offline).id("network-offline")),
                    ))
                    .spacing(16.0),
                ))
                .spacing(6.0)
            },
        ),
    ))
    .title(tr::net_simulation())
}
fn request_section(st: Model, base: String) -> impl Piece {
    let options = Signal::new(false);
    let task: Rc<RefCell<Option<day::TaskHandle>>> = Rc::default();
    let run = {
        let task = task.clone();
        move || {
            if let Some(old) = task.borrow_mut().take() {
                old.abort();
            }
            st.status.set(tr::http_checking().format());
            st.received.set(0);
            let handle = day::task(async move {
                let method = [
                    Method::Get,
                    Method::Post,
                    Method::Patch,
                    Method::Head,
                    Method::Put,
                    Method::Delete,
                ][st.method.get_untracked().min(5)];
                let mut req = Request::new(method, st.url.get_untracked())
                    .timeout(Duration::from_secs_f64(st.timeout.get_untracked()));
                if !matches!(method, Method::Get | Method::Head) {
                    req = req.body(st.body.get_untracked().into_bytes());
                }
                let headers: std::collections::BTreeMap<String, String> =
                    match serde_json::from_str(&st.headers.get_untracked()) {
                        Ok(h) => h,
                        Err(e) => {
                            st.status.set(tr::net_error(e.to_string()).format());
                            return;
                        }
                    };
                for (name, value) in headers {
                    req = req.header(&name, &value);
                }
                st.response_headers.set(String::new());
                match st.client.get_untracked().send_future(req).await {
                    Err(e) => st.status.set(tr::net_error(e.to_string()).format()),
                    Ok(response) => {
                        let status = response.status();
                        st.response_headers.set(
                            response
                                .headers()
                                .iter()
                                .map(|(k, v)| format!("{k}: {v}"))
                                .collect::<Vec<_>>()
                                .join("\n"),
                        );
                        let mut body = response.into_body();
                        let mut preview = Vec::new();
                        while let Some(chunk) = body.next().await {
                            match chunk {
                                Ok(bytes) => {
                                    st.received
                                        .set(st.received.get_untracked() + bytes.len() as u64);
                                    let n = (512 - preview.len()).min(bytes.len());
                                    preview.extend_from_slice(&bytes[..n]);
                                }
                                Err(e) => {
                                    st.status.set(tr::net_error(e.to_string()).format());
                                    return;
                                }
                            }
                        }
                        st.status
                            .set(format!("{status} {}", String::from_utf8_lossy(&preview)));
                    }
                }
            });
            *task.borrow_mut() = Some(handle);
        }
    };
    let stream_base = base.clone();
    let upload_base = base;
    section((
        labeled(
            tr::net_method(),
            picker(["GET", "POST", "PATCH", "HEAD", "PUT", "DELETE"], st.method)
                .segmented()
                .id("http-method"),
        ),
        labeled(tr::net_url(), text_field(st.url).id("http-url")),
        labeled(tr::net_body(), text_field(st.body).id("http-body")),
        labeled(tr::net_options(), toggle(options).id("http-options")),
        when(
            move || options.get(),
            move || {
                column((
                    labeled(tr::net_headers(), text_field(st.headers).id("http-headers")),
                    labeled(
                        tr::net_timeout(),
                        slider(st.timeout).range(1.0..=30.0).id("http-timeout"),
                    ),
                    label(move || {
                        tr::net_timeout_value(format!("{:.0}", st.timeout.get())).format()
                    }),
                    label(move || st.response_headers.get())
                        .max_lines(5)
                        .id("http-response-headers"),
                ))
                .spacing(6.0)
            },
        ),
        crate::widgets::action_result(
            row((
                button(tr::net_send_request())
                    .prominent()
                    .action(run)
                    .id("http-fetch"),
                button(tr::net_cancel())
                    .bordered()
                    .action(move || {
                        if let Some(old) = task.borrow_mut().take() {
                            old.abort();
                        }
                        st.status.set(tr::net_cancelled().format());
                    })
                    .id("http-cancel"),
            ))
            .spacing(8.0),
            label(move || st.status.get())
                .max_lines(4)
                .id("http-status"),
        ),
        row((
            button(tr::net_download_preset())
                .bordered()
                .action(move || {
                    st.method.set(0);
                    st.url.set(if stream_base.is_empty() {
                        "day-http-ok".into()
                    } else {
                        format!("{stream_base}/bytes/524288")
                    });
                })
                .id("http-download-preset"),
            button(tr::net_upload_preset())
                .bordered()
                .action(move || {
                    st.method.set(1);
                    st.url.set(if upload_base.is_empty() {
                        "day-http-ok".into()
                    } else {
                        format!("{upload_base}/echo")
                    });
                    st.body.set("x".repeat(64 * 1024));
                })
                .id("http-upload-preset"),
        ))
        .spacing(8.0),
        label(move || tr::net_received(st.received.get().to_string()).format()).id("http-received"),
    ))
    .title(tr::net_request())
}
fn websocket_section(st: Model, base: String) -> impl Piece {
    let url = Signal::new(if base.is_empty() {
        "wss://fixture.example/echo".into()
    } else {
        format!("{}/ws/echo", base.replacen("http:", "ws:", 1))
    });
    let text = Signal::new(String::new());
    let state = Signal::new(tr::http_idle().format());
    let last = Signal::new(String::new());
    let sender: Rc<RefCell<Option<day_part_http::WsSender>>> = Rc::default();
    let task: Rc<RefCell<Option<day::TaskHandle>>> = Rc::default();
    let connect = {
        let sender = sender.clone();
        move || {
            if let Some(old) = task.borrow_mut().take() {
                old.abort();
            }
            sender.borrow_mut().take();
            state.set(tr::http_checking().format());
            let slot = sender.clone();
            let t = day::task(async move {
                match st
                    .client
                    .get_untracked()
                    .websocket_future(Request::get(url.get_untracked()).protocols(["day"]))
                    .await
                {
                    Err(e) => state.set(tr::net_error(e.to_string()).format()),
                    Ok(mut socket) => {
                        state.set(tr::net_connected().format());
                        *slot.borrow_mut() = Some(socket.sender());
                        while let Some(message) = socket.next().await {
                            match message {
                                Ok(day_part_http::Message::Text(t)) => last.set(t),
                                Ok(day_part_http::Message::Binary(b)) => {
                                    last.set(tr::net_received(b.len().to_string()).format())
                                }
                                Ok(day_part_http::Message::Close { .. }) => {
                                    state.set(tr::net_closed().format())
                                }
                                Err(e) => state.set(tr::net_error(e.to_string()).format()),
                            }
                        }
                        slot.borrow_mut().take();
                    }
                }
            });
            *task.borrow_mut() = Some(t);
        }
    };
    let send = {
        let sender = sender.clone();
        move || {
            if let Some(s) = sender.borrow().as_ref() {
                let socket = s.clone();
                day::task(async move {
                    match socket
                        .send_future(day_part_http::Message::Text(text.get_untracked()))
                        .await
                    {
                        Ok(Ok(())) => {}
                        Ok(Err(e)) => state.set(tr::net_error(e.to_string()).format()),
                        Err(e) => state.set(tr::net_error(e.to_string()).format()),
                    }
                });
            }
        }
    };
    let close = move || {
        if let Some(s) = sender.borrow_mut().take() {
            s.close(1000, "");
        }
    };
    section((
        label(tr::net_ws_note()).font(Font::Footnote),
        labeled(tr::net_url(), text_field(url).id("ws-url")),
        row((
            button(tr::ws_connect())
                .bordered()
                .action(connect)
                .id("ws-connect"),
            button(tr::ws_close())
                .bordered()
                .action(close)
                .id("ws-close"),
        ))
        .spacing(8.0),
        labeled(tr::net_message(), text_field(text).id("ws-text")),
        button(tr::ws_send()).bordered().action(send).id("ws-send"),
        label(move || state.get()).id("ws-state"),
        label(move || last.get()).id("ws-last"),
    ))
    .title(tr::ws_title())
}
fn statistics_section(st: Model) -> impl Piece {
    let plot = chart(move || {
        let labels = [
            tr::net_real_down().format(),
            tr::net_real_up().format(),
            tr::net_sim_down().format(),
            tr::net_sim_up().format(),
        ];
        let mut marks = Vec::new();
        for sample in st.samples.get() {
            for (rate, label) in [
                sample.real_down,
                sample.real_up,
                sample.sim_down,
                sample.sim_up,
            ]
            .into_iter()
            .zip(labels.iter())
            {
                marks.push(
                    line(
                        value(tr::net_seconds().format(), sample.time),
                        value(tr::net_rate().format(), rate),
                    )
                    .by_series(value(tr::net_series().format(), label.clone())),
                );
            }
        }
        marks
    })
    .animated()
    .legend(LegendPosition::Leading)
    .y_label(tr::net_rate().format())
    .height(190.0)
    .id("network-chart");
    section((
        plot,
        label(move || {
            let s = st.statistics.get();
            tr::net_totals(
                s.active.to_string(),
                s.failed.to_string(),
                s.downloaded_bytes.to_string(),
                s.uploaded_bytes.to_string(),
            )
            .format()
        })
        .id("network-totals"),
        label(move || {
            let s = st.statistics.get();
            tr::net_average(
                format!("{:.1}", s.download_bytes_per_second() / 1024.0),
                format!("{:.1}", s.upload_bytes_per_second() / 1024.0),
            )
            .format()
        }),
        label(tr::net_stats_note()).font(Font::Footnote),
    ))
    .title(tr::net_statistics())
}
