use day::prelude::*;
use day_piece_webview::{
    JsHandle, LinkPolicy, PageState, WebSession, eval_support, inline_support,
    script_message_support, support, web_view, web_view_inline,
};

use crate::widgets::heading;

// Page state that outlives the page. Day rebuilds a destination's whole subtree on every
// navigation, so anything declared inside `webview_page()` is minted fresh each visit: the URL bar
// would snap back, the console would clear. The fields live on the window's `Scene` (lib.rs)
// instead, so every visit finds the same signals. Same idiom as pages/scripting.rs.
//
// This is transient, not persisted: it lives as long as the process and is never written to disk.

/// `(url, script, result)`: created once, reused by every visit to this page.
fn state() -> (Signal<String>, Signal<String>, Signal<String>) {
    crate::scene().web
}

fn embed_status() -> Signal<(u8, String)> {
    crate::scene().web_embed_status
}

/// The name the bundled site posts its messages to (`window.dayMessage("showcase", text)` in
/// `resource/assets/web/minisite/js/app.js`).
const MESSAGE_CHANNEL: &str = "showcase";

thread_local! {
    /// How many messages the bundled site has posted and the text of the last one. Kept for
    /// the life of the app, as the site's retained session keeps its own count, so the two
    /// agree after a visit to another page.
    static MESSAGES: Signal<(i64, String)> = Signal::global((0, String::new()));
}

/// The two tabs. Embedded comes first, so it is what the page opens on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum WebTab {
    Embedded,
    Remote,
}

/// A native web view (day-piece-webview, an external standalone piece), in two tabs:
///
/// - **Embedded** (the default): `web_view_inline`, a complete site (pages, css, js, images)
///   bundled under `resource/assets/web/minisite/` and served from inside the app (§18.5,
///   docs/webview.md). Relative links resolve within the site; external links open in the system
///   browser by default; `day-showcase://` links are intercepted by `on_external_link` and
///   navigate this app: the custom-policy hook, demonstrated end to end. The site's Send button
///   posts a message that `on_script_message` answers, and the page shows the reply. It is the
///   default so opening the page needs no network: the walkthrough exercises this tab only, and
///   stays independent of the live site's reachability, load time and TLS chain (Android 7.0
///   does not trust daybrite.dev's root).
/// - **Remote**: WKWebView / QWebEngineView / android.webkit.WebView browsing the live web. The
///   URL bar is bound two-way, Back/Forward/Stop/Reload drive history via `Trigger`s, and the JS
///   console round-trips `eval` where the engine allows it. web-dom is the exception: an
///   `<iframe>` under the same-origin policy; the piece reports `Support::Emulated`, the history
///   buttons are disabled, and a footnote says why (docs/webview.md).
///
/// Each pane binds a [`PageState`] with `.page_state(..)`: its Back and Forward buttons are
/// enabled from the engine's history, and the page's title and load progress sit beside them.
///
/// Both tabs come back as they were left: the selection rides a hoisted signal, and each view
/// rides its own retained `WebSession`, so the engines that retain (WebKit here, docs/webview.md)
/// re-attach the same native view: page, scroll position and JS state intact.
///
/// The JS console sits below the tabs, outside both panes: each view carries its own bound
/// [`JsHandle`], and Run evaluates against whichever tab is selected.
pub(crate) fn webview_page() -> AnyPiece {
    let tab = crate::scene().web_tab;
    let js_remote = JsHandle::new();
    let js_embedded = JsHandle::new();
    column((
        heading(crate::res::str::nav_webview(), day::id!(webview_title)),
        column((picker(
            tab,
            [
                (WebTab::Embedded, crate::res::str::webview_tab_embedded()),
                (WebTab::Remote, crate::res::str::webview_tab_remote()),
            ],
        )
        .segmented()
        .id(day::id!(webview_tab)),))
        .align(HAlign::Center)
        .grow(Dimension::Width),
        switch(
            move || tab.get(),
            move |t| match t {
                WebTab::Embedded => embedded_pane(js_embedded).any(),
                WebTab::Remote => remote_pane(js_remote).any(),
            },
        ),
        js_console(tab, js_remote, js_embedded),
    ))
    .spacing(Space::M)
    .align(HAlign::Leading)
    .padding(20.0)
    .any()
}

/// The JS console, below both tabs: script in, JSON out, evaluated against whichever web view
/// the selected tab shows: each view binds its own [`JsHandle`], and Run picks by the tab
/// signal. `eval` returns a future, so the click spawns a task and the result lands in the
/// bound signal whenever the engine answers. Sized in lines, not points, so the editors track
/// the platform accessibility text scale (day-toolkit-dom used to ignore the hints; it measures them
/// now, docs/textarea.md).
fn js_console(tab: Signal<WebTab>, js_remote: JsHandle, js_embedded: JsHandle) -> impl Piece {
    let (_, script, result) = state();
    let can_eval = eval_support() == Support::Native;
    row((
        text_area(script)
            .placeholder(crate::res::str::webview_js_hint())
            .min_lines(3)
            .max_lines(3)
            .spellcheck(false)
            .editable(move || can_eval)
            .id(day::id!(webview_js)),
        button(crate::res::str::webview_js_run())
            .prominent()
            .enabled(move || can_eval)
            .action(move || {
                let js = if tab.get_untracked() == WebTab::Remote {
                    js_remote
                } else {
                    js_embedded
                };
                // A new evaluation invalidates the previous result immediately. In particular,
                // repeated expressions must not satisfy a walkthrough assertion with stale text.
                result.set(String::new());
                let script = script.get_untracked();
                day::task(async move {
                    let text = match js.eval(script).await {
                        Ok(json) => json,
                        Err(e) => e.to_string(),
                    };
                    result.set(text);
                });
            })
            .tint(crate::widgets::primary())
            .id(day::id!(webview_js_run)),
        text_area(result)
            .placeholder(crate::res::str::webview_js_result_hint())
            .min_lines(3)
            .max_lines(3)
            .editable(false)
            .id(day::id!(webview_js_result)),
    ))
    .spacing(Space::S)
    .align(VAlign::Center)
}

/// The page's title, as the engine reports it, on one line.
fn page_title(page: Signal<PageState>, id: ElementId) -> impl Piece {
    label(move || page.get().title)
        .font(Font::Headline)
        .single_line()
        .id(id)
        .grow(Dimension::Width)
}

/// The load progress the engine reports. It rests full once a page has loaded.
fn load_progress(page: Signal<PageState>, id: ElementId) -> impl Piece {
    progress(move || page.get().progress)
        .id(id)
        .grow(Dimension::Width)
}

/// The Remote pane: URL bar + history controls over a session-retained view. The JS console
/// lives below the tabs (`js_console`), bound here through `js`.
fn remote_pane(js: JsHandle) -> impl Piece {
    let (url, _, _) = state();
    let page = Signal::new(PageState::default());
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
                .id(day::id!(webview_url)),
            button(crate::res::str::webview_go())
                .prominent()
                .action(move || go.notify())
                .tint(crate::widgets::primary())
                .id(day::id!(webview_go)),
        ))
        .spacing(Space::S),
        // Why the three buttons below are dead on this backend.
        when(
            move || iframe,
            move || {
                label(crate::res::str::webview_note_iframe())
                    .font(Font::Footnote)
                    .secondary()
                    .id(day::id!(webview_note))
            },
        ),
        // History controls, enabled from what the engine reports. "Stop" is the demo's cancel.
        row((
            button(crate::res::str::webview_back())
                .bordered()
                .enabled(move || history && page.get().can_go_back)
                .action(move || back.notify())
                .id(day::id!(webview_back)),
            button(crate::res::str::webview_forward())
                .bordered()
                .enabled(move || history && page.get().can_go_forward)
                .action(move || forward.notify())
                .id(day::id!(webview_forward)),
            button(crate::res::str::webview_stop())
                .bordered()
                .enabled(move || history && page.get().loading)
                .action(move || stop.notify())
                .id(day::id!(webview_stop)),
            button(crate::res::str::webview_reload())
                .bordered()
                .action(move || reload.notify())
                .id(day::id!(webview_reload)),
        ))
        .spacing(Space::S)
        .fit(RowFit::Wrap { run_spacing: 8.0 }),
        page_title(page, day::id!(webview_page_title)),
        load_progress(page, day::id!(webview_progress)),
        web_view(url)
            // A retained session: the engine outlives this page's subtree, so navigating away and
            // back returns to the page as it was left rather than reloading it.
            .session(WebSession::global("showcase.webview"))
            .page_state(page)
            .js(js)
            .go(go)
            .back(back)
            .forward(forward)
            .stop(stop)
            .reload(reload)
            .id(day::id!(webview)),
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    .grow(Dimension::Both)
}

/// The bundled mini site (docs/webview.md): `resource/assets/web/minisite/**` ships with the app,
/// `web_view_inline` serves it through each backend's local-content channel, and the
/// `on_external_link` hook shows both dispositions: system browser for real URLs, in-app
/// navigation for `day-showcase://` ones. The site's Send button calls
/// `window.dayMessage("showcase", text)`; `on_script_message` counts the message, shows it in
/// the line under the title, and returns the reply the page's promise resolves with. The site's
/// text is sample content (like the bundled font specimens), so it ships in English only; the
/// chrome around it localizes as usual. The shared JS console below the tabs evaluates in this
/// view while the tab is selected, through the bound `js`.
fn embedded_pane(js: JsHandle) -> impl Piece {
    let status = embed_status();
    let messages = MESSAGES.with(|messages| *messages);
    let page = Signal::new(PageState::default());
    let back = Trigger::new();
    let forward = Trigger::new();
    let arm = inline_support();
    // web-dom's iframe and Servo have no channel from the page to the app (docs/webview.md).
    let can_message = script_message_support() != Support::Unsupported;
    let body: AnyPiece = if arm == Support::Unsupported {
        // No web engine in this toolkit build (docs/webview.md): say so instead of showing a
        // blank frame.
        label(crate::res::str::webview_embedded_unsupported())
            .font(Font::Footnote)
            .id(day::id!(webview_embedded_unsupported))
            .any()
    } else {
        web_view_inline(crate::res::assets::web::minisite)
            .session(WebSession::global("showcase.webview.embedded"))
            .page_state(page)
            .js(js)
            .back(back)
            .forward(forward)
            .on_script_message(MESSAGE_CHANNEL, move |message| {
                let count = messages.get_untracked().0 + 1;
                messages.set((count, message.payload));
                Some(crate::res::str::webview_message_reply(count).format())
            })
            .on_external_link(move |url| {
                if let Some(route) = url.strip_prefix("day-showcase://") {
                    // The custom hook: a link the site authors as day-showcase://<route>
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
            .id(day::id!(webview_embedded))
            .any()
    };
    column((
        row((
            button(crate::res::str::webview_back())
                .bordered()
                .enabled(move || page.get().can_go_back)
                .action(move || back.notify())
                .id(day::id!(webview_embedded_back)),
            button(crate::res::str::webview_forward())
                .bordered()
                .enabled(move || page.get().can_go_forward)
                .action(move || forward.notify())
                .id(day::id!(webview_embedded_forward)),
            page_title(page, day::id!(webview_embedded_title)),
        ))
        .spacing(Space::S)
        .align(VAlign::Center),
        load_progress(page, day::id!(webview_embedded_progress)),
        label(move || {
            let (kind, payload) = status.get();
            match kind {
                1 => crate::res::str::webview_embedded_opened(payload).format(),
                2 => crate::res::str::webview_embedded_intercepted(payload).format(),
                _ => crate::res::str::webview_embedded_status_none().format(),
            }
        })
        .font(Font::Footnote)
        .secondary()
        .id(day::id!(webview_embedded_status)),
        label(move || {
            let (count, text) = messages.get();
            if !can_message {
                crate::res::str::webview_message_unsupported().format()
            } else if count == 0 {
                crate::res::str::webview_message_none().format()
            } else {
                crate::res::str::webview_message_received(crate::res::str::WebviewMessageReceived {
                    count,
                    text,
                })
                .format()
            }
        })
        .font(Font::Footnote)
        .secondary()
        .id(day::id!(webview_message)),
        body,
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    .grow(Dimension::Both)
}
