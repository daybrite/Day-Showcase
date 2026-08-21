use day::prelude::*;
use day_piece_webview::{
    JsHandle, LinkPolicy, WebSession, eval_support, inline_support, support, web_view,
    web_view_inline,
};

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
    // The Embedded tab's external-link readout: (kind, payload) where kind 0 = none yet,
    // 1 = opened outside, 2 = intercepted in-app. Stored as data and FORMATTED in the label so a
    // locale switch re-resolves the text.
    static EMBED_STATUS: std::cell::OnceCell<Signal<(u8, String)>> =
        const { std::cell::OnceCell::new() };
    // The selected tab (0 = Remote, 1 = Embedded), hoisted so a revisit returns to the tab the
    // user left — the pane content itself survives through each view's WebSession either way.
    static TAB: std::cell::OnceCell<Signal<usize>> = const { std::cell::OnceCell::new() };
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

fn embed_status() -> Signal<(u8, String)> {
    EMBED_STATUS.with(|c| *c.get_or_init(|| Signal::global((0, String::new()))))
}

/// A native web view (day-piece-webview, an EXTERNAL standalone piece), in two tabs:
///
/// - **Remote** — WKWebView / QWebEngineView / android.webkit.WebView browsing the live web. The
///   URL bar is bound two-way, Back/Forward/Stop/Reload drive history via `Trigger`s, and the JS
///   console round-trips `eval` where the engine allows it. web-dom is the exception: an
///   `<iframe>` under the same-origin policy — the piece reports `Support::Emulated`, the history
///   buttons are disabled, and a footnote says why (docs/webview.md).
/// - **Embedded** — `web_view_inline`: a complete site (pages, css, js, images) bundled under
///   `resource/assets/web/minisite/` and served from inside the app (§18.5, docs/webview.md).
///   Relative links resolve within the site; external links open in the system browser by
///   default; `day-showcase://` links are intercepted by `on_external_link` and navigate THIS
///   app — the custom-policy hook, demonstrated end to end.
///
/// Both tabs come back as they were left: the selection rides a hoisted signal, and each view
/// rides its own retained `WebSession`, so the engines that retain (WebKit here, docs/webview.md)
/// re-attach the SAME native view — page, scroll position and JS state intact.
///
/// The JS console sits BELOW the tabs, outside both panes: each view carries its own bound
/// [`JsHandle`], and Run evaluates against whichever tab is selected.
pub(crate) fn webview_page() -> impl Piece{
    let tab = TAB.with(|c| *c.get_or_init(|| Signal::global(0usize)));
    let js_remote = JsHandle::new();
    let js_embedded = JsHandle::new();
    column((
        heading(crate::res::str::nav_webview(), "webview-title", None),
        column((picker(
            [
                crate::res::str::webview_tab_remote().format(),
                crate::res::str::webview_tab_embedded().format(),
            ],
            tab,
        )
        .segmented()
        .id("webview-tab"),))
        .align(HAlign::Center)
        .grow_w(),
        when(move || tab.get() == 0, move || remote_pane(js_remote)),
        when(move || tab.get() == 1, move || embedded_pane(js_embedded)),
        js_console(tab, js_remote, js_embedded),
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    .padding(16.0)
    
}

/// The JS console, below both tabs: script in, JSON out, evaluated against WHICHEVER web view
/// the selected tab shows — each view binds its own [`JsHandle`], and Run picks by the tab
/// signal. `eval` returns a future, so the click spawns a task and the result lands in the
/// bound signal whenever the engine answers. Sized in LINES, not points, so the editors track
/// the platform accessibility text scale (day-dom used to ignore the hints; it measures them
/// now, docs/textarea.md).
fn js_console(tab: Signal<usize>, js_remote: JsHandle, js_embedded: JsHandle) -> impl Piece{
    let (_, script, result) = state();
    let can_eval = eval_support() == Support::Native;
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
                let js = if tab.get_untracked() == 1 {
                    js_embedded
                } else {
                    js_remote
                };
                day::task(async move {
                    let text = match js.eval(script.get_untracked()).await {
                        Ok(json) => json,
                        Err(e) => e.to_string(),
                    };
                    result.set(text);
                });
            })
            .tint(crate::widgets::primary())
            .id("webview-js-run"),
        text_area(result)
            .placeholder(crate::res::str::webview_js_result_hint())
            .min_lines(3)
            .max_lines(3)
            .editable(false)
            .id("webview-js-result"),
    ))
    .spacing(8.0)
    
}

/// The Remote pane: URL bar + history controls over a session-retained view. The JS console
/// lives below the tabs (`js_console`), bound here through `js`.
fn remote_pane(js: JsHandle) -> impl Piece{
    let (url, _, _) = state();
    let go = Trigger::new();
    let back = Trigger::new();
    let forward = Trigger::new();
    let stop = Trigger::new();
    let reload = Trigger::new();
    // Only a real embedded engine can drive session history. `Go` and `Reload` work everywhere the
    // piece has a renderer at all, so only Back/Forward/Stop are gated.
    let history = support() == Support::Native;
    let iframe = support() == Support::Emulated;
    column((
        // URL bar: the field is bound to the view's URL; Go loads whatever it holds.
        row((
            text_field(url)
                .placeholder(crate::res::str::webview_url_hint())
                .id("webview-url"),
            button(crate::res::str::webview_go())
                .prominent()
                .action(move || go.notify())
                .tint(crate::widgets::primary())
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
    .grow()
    
}

/// The bundled mini site (docs/webview.md): `resource/assets/web/minisite/**` ships with the app,
/// `web_view_inline` serves it through each backend's local-content channel, and the
/// `on_external_link` hook shows both dispositions — system browser for real URLs, in-app
/// navigation for `day-showcase://` ones. The site's own text is sample CONTENT (like the bundled
/// font specimens), so it ships in English only; the chrome around it localizes as usual. The
/// shared JS console below the tabs evaluates in THIS view while the tab is selected, through
/// the bound `js`.
fn embedded_pane(js: JsHandle) -> impl Piece{
    let status = embed_status();
    let arm = inline_support();
    let body: AnyPiece = if arm == Support::Unsupported {
        // No web engine in this toolkit build (docs/webview.md) — say so instead of showing a
        // blank frame.
        label(crate::res::str::webview_embedded_unsupported())
            .font(Font::Footnote)
            .id("webview-embedded-unsupported")
            .any()
    } else {
        web_view_inline(crate::res::assets::web::minisite)
            .session(WebSession::global("showcase.webview.embedded"))
            .js(js)
            .on_external_link(move |url| {
                if let Some(route) = url.strip_prefix("day-showcase://") {
                    // The custom hook: a link the SITE authors as day-showcase://<route>
                    // navigates this app on the deep-link rail instead of leaving it.
                    let route = route.trim_matches('/').to_string();
                    status.set((2, route.clone()));
                    day::request_route(&route);
                    LinkPolicy::Ignore
                } else {
                    status.set((1, url.to_string()));
                    LinkPolicy::OpenSystem
                }
            })
            .id("webview-embedded")
            .any()
    };
    column((
        label(crate::res::str::webview_embedded_caption()).font(Font::Footnote),
        label(move || {
            let (kind, payload) = status.get();
            match kind {
                1 => crate::res::str::webview_embedded_opened(payload).format(),
                2 => crate::res::str::webview_embedded_intercepted(payload).format(),
                _ => crate::res::str::webview_embedded_status_none().format(),
            }
        })
        .font(Font::Footnote)
        .id("webview-embedded-status"),
        body,
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    .grow()
    
}
