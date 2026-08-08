use day::prelude::*;
use day_piece_webview::{JsHandle, WebSession, eval_support, support, web_view};

use crate::widgets::heading;

// Page state that outlives the page. Day rebuilds a destination's whole subtree on every
// navigation, so anything declared inside `webview_page()` is minted fresh each visit — the URL bar
// would snap back, the console would clear. `Signal::global` allocates in the root scope instead,
// and the `OnceCell` keeps the SAME signal across rebuilds (calling `global` per build would mint a
// new one every time). Same idiom as pages/scripting.rs.
//
// This is transient, not persisted: it lives as long as the process and is never written to disk.
thread_local! {
    static STATE: std::cell::OnceCell<(Signal<String>, Signal<String>, Signal<String>)> =
        const { std::cell::OnceCell::new() };
}

/// `(url, script, result)` — created once, reused by every visit to this page.
fn state() -> (Signal<String>, Signal<String>, Signal<String>) {
    STATE.with(|c| {
        *c.get_or_init(|| {
            (
                Signal::global("https://daybrite.dev".to_string()),
                Signal::global("document.title".to_string()),
                Signal::global(String::new()),
            )
        })
    })
}

/// A native web view (day-piece-webview, an EXTERNAL standalone piece): WKWebView / QWebEngineView /
/// android.webkit.WebView. The URL bar is bound two-way to the view — type + Go loads it, and
/// navigation reports the URL back so the field follows. Back/Forward/Stop/Reload drive history via
/// `Trigger`s the piece watches. The view fills the remaining space (a growing leaf).
///
/// web-dom is the exception: it embeds an `<iframe>`, where the same-origin policy blocks history
/// and URL readback. There the piece reports `Support::Emulated`, the history buttons are disabled
/// rather than left to do nothing, and a footnote says why (docs/webview.md).
pub(crate) fn webview_page() -> AnyPiece {
    let (url, script, result) = state();
    let go = Trigger::new();
    let back = Trigger::new();
    let forward = Trigger::new();
    let stop = Trigger::new();
    let reload = Trigger::new();
    // Only a real embedded engine can drive session history. `Go` and `Reload` work everywhere the
    // piece has a renderer at all, so only Back/Forward/Stop are gated.
    let history = support() == Support::Native;
    let iframe = support() == Support::Emulated;
    // The JS console below. `script` is what the user types, `result` the JSON it evaluated to (or
    // the error it threw) — both bound to text areas, so the round trip is visible in one screen.
    let js = JsHandle::new();
    let can_eval = eval_support() == Support::Native;
    column((
        heading(crate::res::str::nav_webview(), "webview-title", None),
        // URL bar: the field is bound to the view's URL; Go loads whatever it holds.
        row((
            text_field(url)
                .placeholder(crate::res::str::webview_url_hint())
                .id("webview-url"),
            button(crate::res::str::webview_go())
                .prominent()
                .action(move || go.notify())
                .style(crate::widgets::primary())
                .id("webview-go"),
        ))
        .spacing(8.0),
        // Why the three buttons below are dead on this backend.
        when(
            move || iframe,
            move || {
                label(crate::res::str::webview_note_iframe())
                    .font(Font::Footnote)
                    .id("webview-note")
            },
        ),
        // History controls. "Stop" is the demo's cancel.
        row((
            button(crate::res::str::webview_back())
                .bordered()
                .enabled(move || history)
                .action(move || back.notify())
                .id("webview-back"),
            button(crate::res::str::webview_forward())
                .bordered()
                .enabled(move || history)
                .action(move || forward.notify())
                .id("webview-forward"),
            button(crate::res::str::webview_stop())
                .bordered()
                .enabled(move || history)
                .action(move || stop.notify())
                .id("webview-stop"),
            button(crate::res::str::webview_reload())
                .bordered()
                .action(move || reload.notify())
                .id("webview-reload"),
        ))
        .spacing(8.0),
        // JS console: script in, JSON out. `eval` returns a future, so the click spawns a task and
        // the result lands in the bound signal whenever the engine answers.
        row((
            text_area(script)
                .placeholder(crate::res::str::webview_js_hint())
                .min_lines(3)
                .max_lines(3)
                .spellcheck(false)
                .editable(move || can_eval)
                .id("webview-js"),
            button(crate::res::str::webview_js_run())
                .prominent()
                .enabled(move || can_eval)
                .action(move || {
                    day::task(async move {
                        let text = match js.eval(script.get_untracked()).await {
                            Ok(json) => json,
                            Err(e) => e.to_string(),
                        };
                        result.set(text);
                    });
                })
                .style(crate::widgets::primary())
                .id("webview-js-run"),
            text_area(result)
                .placeholder(crate::res::str::webview_js_result_hint())
                .min_lines(3)
                .max_lines(3)
                .editable(false)
                .id("webview-js-result"),
        ))
        .spacing(8.0),
        web_view(url)
            // A retained session: the engine outlives this page's subtree, so navigating away and
            // back returns to the page as it was left rather than reloading it.
            .session(WebSession::global("showcase.webview"))
            .js(js)
            .go(go)
            .back(back)
            .forward(forward)
            .stop(stop)
            .reload(reload)
            .id("webview"),
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    .padding(16.0)
    .any()
}
