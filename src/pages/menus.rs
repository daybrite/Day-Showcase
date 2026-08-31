use day::prelude::*;

use crate::widgets::page;

/// The last menu action fired — shared between the app menu and this page, so both demonstrate
/// action dispatch. App-wide (docs/state.md): it records what the app's ONE menu bar did.
#[derive(Clone, Copy)]
struct MenuLog(Signal<String>);

impl Ambient for MenuLog {
    fn create() -> Self {
        // An em dash, not an empty string: the readout is a label, and an empty one has no
        // frame at all — which is a walkthrough failure rather than a blank line.
        MenuLog(Signal::new("—".into()))
    }
}

fn menu_log() -> Signal<String> {
    // App-scoped, not page-scoped: the readout and the app menu both reach it, and a
    // scope-owned signal would die with whichever page created it — which is what wedged the
    // walkthrough's second and subsequent variants.
    MenuLog::app().0
}

/// The application menu bar (native NSMenu / GtkPopoverMenuBar / QMenuBar; app-bar overflow on Android;
/// UIMenuBuilder on iPadOS). Custom items carry keyboard shortcuts and update the shared `menu_log`;
/// the Edit menu uses standard roles so Cut/Copy/Paste target the focused control natively.
/// Installed REACTIVELY (docs/menus.md): the builder's localized reads re-run on a runtime
/// language change (the Preferences window's language picker), rebuilding the whole bar in
/// the new language.
pub(crate) fn install_app_menu() {
    app_menu_reactive(build_app_menu);
}

fn build_app_menu() -> Vec<MenuEntry> {
    let log = |what: String| move || menu_log().set(what.clone());
    // Localized menu names, resolved per install (the reactive builder re-resolves them on a
    // locale change). The MENU_LOG readout composes from the SAME strings so it always
    // matches the visible menus; `menu_role` items are localized by the OS itself.
    // `report.pdf`/`budget.xlsx` are fixture FILENAMES (data, not prose) and stay raw.
    let file = crate::res::str::menu_file().format();
    let open = crate::res::str::menu_open().format();
    let recent = crate::res::str::menu_open_recent().format();
    let clear = crate::res::str::menu_clear_menu().format();
    let save = crate::res::str::menu_save().format();
    let save_as = crate::res::str::menu_save_as().format();
    let view = crate::res::str::menu_view().format();
    let reload = crate::res::str::menu_reload().format();
    let actual_size = crate::res::str::menu_actual_size().format();
    vec![
        // `.bar_role(...)` claims the platform's standard slot for this menu, which is what puts
        // File/Edit/View in the platform's own order and stops the backend adding its stock copy
        // beside them. The TAG, not the title, identifies the slot: day's catalog and this app's
        // may translate the same menu differently (day's `day-view` is "Présentation", this app's
        // is "Affichage"), and a bar showing both is exactly the bug that taught us so.
        sub_menu(
            file.clone(),
            vec![
                // File ▸ New Window (⌘N; also the macOS tab-bar "+"): opens another
                // showcase shell through the `register_new_window` builder
                // (docs/windows.md); lowers disabled where no builder is registered.
                menu_role(MenuRole::NewWindow),
                menu_item(open.clone())
                    .key("o")
                    .action(log(format!("{file} ▸ {open}"))),
                // A nested submenu with keyboard shortcuts.
                sub_menu(
                    recent.clone(),
                    vec![
                        menu_item("report.pdf").action(log(format!("{recent} ▸ report.pdf"))),
                        menu_item("budget.xlsx").action(log(format!("{recent} ▸ budget.xlsx"))),
                        menu_separator(),
                        menu_item(clear.clone()).action(log(format!("{recent} ▸ {clear}"))),
                    ],
                ),
                menu_separator(),
                menu_item(save.clone())
                    .key("s")
                    .action(log(format!("{file} ▸ {save}"))),
                menu_item(save_as.clone())
                    .shortcut(Shortcut::new("s").shift())
                    .action(log(format!("{file} ▸ {save_as}"))),
                menu_separator(),
                menu_role(MenuRole::CloseWindow),
                // Quit is a standard role: ⌘Q / Ctrl+Q, it exits the app and fires the
                // `WillTerminate` lifecycle phase (docs/lifecycle.md). macOS also keeps the
                // conventional Quit in the App menu.
                menu_role(MenuRole::Quit),
            ],
        )
        .bar_role(MenuBarRole::File),
        // Standard edit commands — native items that target the focused control (default shortcuts).
        sub_menu(
            crate::res::str::menu_edit().format(),
            vec![
                menu_role(MenuRole::Undo),
                menu_role(MenuRole::Redo),
                menu_separator(),
                menu_role(MenuRole::Cut),
                menu_role(MenuRole::Copy),
                menu_role(MenuRole::Paste),
                menu_role(MenuRole::SelectAll),
            ],
        )
        .bar_role(MenuBarRole::Edit),
        sub_menu(
            view.clone(),
            vec![
                // The Star command (commands.rs). Its TITLE is the command's, so this item reads
                // "Star" or "Unstar" for the page that is showing — `app_menu_reactive` re-lowers
                // the bar when the starred set changes, which is what keeps it in step with the
                // toolbar button and the sidebar rows without any of them knowing about the
                // others. ⌘D / Ctrl+D, the platform's usual "bookmark this" key.
                //
                // The TITLE carries the state, not a check mark: "Star" / "Unstar" is the
                // platform idiom for a command whose two directions are one item, and day's
                // menu model has no checked state to set anyway.
                {
                    let star = crate::commands::star();
                    menu_item((star.title)().format())
                        .key("d")
                        .enabled((star.enabled)())
                        .action(move || (star.run)())
                },
                {
                    let shot = crate::commands::screenshot();
                    menu_item((shot.title)().format())
                        .key("s")
                        .enabled((shot.enabled)())
                        .action(move || (shot.run)())
                },
                menu_separator(),
                menu_separator(),
                // Appearance (commands.rs): the same three commands the toolbar's segmented
                // control carries. ⌘⌥1/2/3 — the digits are the group's order, and ⌥ keeps them
                // clear of the tab-switching ⌘1..9 every desktop browser and editor already owns.
                sub_menu(
                    crate::res::str::menu_appearance().format(),
                    vec![
                        appearance_item(crate::commands::Appearance::Light, "1"),
                        appearance_item(crate::commands::Appearance::System, "2"),
                        appearance_item(crate::commands::Appearance::Dark, "3"),
                    ],
                ),
                menu_separator(),
                menu_item(reload.clone())
                    .key("r")
                    .action(log(format!("{view} ▸ {reload}"))),
                menu_item(actual_size.clone())
                    .key("0")
                    .action(log(format!("{view} ▸ {actual_size}"))),
                menu_separator(),
                menu_role(MenuRole::Fullscreen),
            ],
        )
        .bar_role(MenuBarRole::View),
        // A menu of its own for the recorder (docs/agent.md): it is neither a File nor a View
        // command, and burying a transport in another menu is how it stops being found. No
        // `bar_role` — there is no standard slot for it, so it takes an ordinary custom menu.
        sub_menu(
            crate::res::str::menu_record().format(),
            vec![
                // ⌘⇧R / ⌘⇧P: the recording pair, shifted clear of View ▸ Reload (⌘R) and the
                // platform's Print (⌘P). Both TITLES carry their state, so one item is
                // Record ▸ Stop and the other Play ▸ Pause ▸ Resume.
                {
                    let rec = crate::commands::record();
                    menu_item((rec.title)().format())
                        .shortcut(Shortcut::new("r").shift())
                        .enabled((rec.enabled)())
                        .action(move || (rec.run)())
                },
                {
                    let play = crate::commands::play_pause();
                    menu_item((play.title)().format())
                        .shortcut(Shortcut::new("p").shift())
                        .enabled((play.enabled)())
                        .action(move || (play.run)())
                },
                menu_separator(),
                {
                    let clear = crate::commands::clear_recording();
                    menu_item((clear.title)().format())
                        .shortcut(Shortcut::new("k").shift())
                        .enabled((clear.enabled)())
                        .action(move || (clear.run)())
                },
                menu_separator(),
                menu_item(crate::res::str::toolbar_menu_open_scripting().format()).action(|| {
                    navigate_to(&crate::Section::Scripting);
                }),
            ],
        ),
    ]
}

/// One appearance mode as a menu item: ⌘⌥`key`, the command's own title, and a check mark on the
/// mode in force. Reading `checked` HERE is what re-lowers the bar when the setting changes.
fn appearance_item(mode: crate::commands::Appearance, key: &str) -> MenuEntry {
    let cmd = crate::commands::appearance_command(mode);
    let title = (cmd.title)().format();
    // The mode in force is marked in the title, since day's menu model carries no checked state
    // (the same reason Star spells its two directions into the title).
    let title = if (cmd.checked)() {
        format!("✓ {title}")
    } else {
        title
    };
    menu_item(title)
        .shortcut(Shortcut::new(key).alt())
        .enabled((cmd.enabled)())
        .action(move || (cmd.run)())
}

/// Menus & dialogs — the app's transient native surfaces in one place: the menu bar and
/// context menus (docs/menus.md), and the imperative dialogs (docs/dialogs.md), each in its own
/// themed section with a live result readout.
pub(crate) fn menus_page() -> AnyPiece {
    page(
        crate::res::str::nav_menus(),
        "menus-title",
        Some(crate::res::str::menus_caption()),
        form((
            app_menu_section(),
            context_section(),
            messages_section(),
            photo_section(),
            dialogs_section(),
        ))
        .any(),
    )
    .any()
}

/// The app-menu section: a live readout of the last action fired from the menu bar (or the
/// context menu below), plus the keyboard-shortcut hint.
fn app_menu_section() -> impl Piece {
    section((
        labeled(
            crate::res::str::menus_last(),
            label(move || menu_log().get()).id("menus-last"),
        ),
        label(crate::res::str::menus_shortcut_hint()).font(Font::Footnote),
    ))
    .title(crate::res::str::menus_appmenu_section())
}

/// The context-menu section: a visually delineated target the user secondary-clicks
/// (long-presses on mobile) — nested submenu, separator, and a standard role.
fn context_section() -> impl Piece {
    // Localized like the app menu above; the log readout composes from the same strings.
    let log = |what: String| move || menu_log().set(what.clone());
    let context = crate::res::str::menu_context().format();
    let rename = crate::res::str::menu_rename().format();
    let duplicate = crate::res::str::menu_duplicate().format();
    let move_to = crate::res::str::menu_move_to().format();
    let inbox = crate::res::str::menu_inbox().format();
    let archive = crate::res::str::menu_archive().format();
    let delete = crate::res::str::delete().format();
    // Order matters: `.background`/`.corner_radius` build the pill CONTAINER (a native
    // view), and `.context_menu` after them attaches to that container — the whole padded
    // pill is the right-click / long-press surface, not just the label's text run.
    section((label(crate::res::str::menus_target())
        .padding(Insets::symmetric(24.0, 24.0))
        // A translucent brand-blue wash: tinted enough to read as "this spot is interactive"
        // over both the light and dark grounds.
        .background(Color::rgba(0.184, 0.435, 0.871, 0.13))
        .corner_radius(10.0)
        .id("menus-context-target")
        .context_menu(vec![
            menu_item(rename.clone()).action(log(format!("{context} ▸ {rename}"))),
            menu_item(duplicate.clone())
                .key("d")
                .action(log(format!("{context} ▸ {duplicate}"))),
            menu_separator(),
            sub_menu(
                move_to.clone(),
                vec![
                    menu_item(inbox.clone())
                        .action(log(format!("{context} ▸ {move_to} ▸ {inbox}"))),
                    menu_item(archive.clone())
                        .action(log(format!("{context} ▸ {move_to} ▸ {archive}"))),
                ],
            ),
            menu_separator(),
            menu_role(MenuRole::Copy),
            menu_item(delete.clone())
                .shortcut(Shortcut::plain("Delete"))
                .action(log(format!("{context} ▸ {delete}"))),
        ]),))
    .title(crate::res::str::menus_context_section())
}

/// Real-world per-ROW menus (docs/menus.md): every message row carries its own context menu,
/// so the action names the row it came from — the mail-list idiom. The senders are fixture
/// DATA (like the file names above) and stay raw; everything the user reads as UI is
/// localized.
fn messages_section() -> impl Piece {
    let log = |what: String| move || menu_log().set(what.clone());
    let reply = crate::res::str::menu_reply().format();
    let forward = crate::res::str::menu_forward().format();
    let archive = crate::res::str::menu_archive().format();
    let delete = crate::res::str::delete().format();
    let message_row = move |i: usize, sender: &'static str, subject: String| {
        let menu = vec![
            menu_item(reply.clone()).action(log(format!("{reply} ▸ {sender}"))),
            menu_item(forward.clone()).action(log(format!("{forward} ▸ {sender}"))),
            menu_separator(),
            menu_item(archive.clone()).action(log(format!("{archive} ▸ {sender}"))),
            menu_item(delete.clone()).action(log(format!("{delete} ▸ {sender}"))),
        ];
        column((label(sender), label(subject).font(Font::Footnote)))
            .spacing(2.0)
            .padding(Insets::symmetric(14.0, 10.0))
            .background(Color::rgba(0.5, 0.5, 0.5, 0.10))
            .corner_radius(8.0)
            .id(format!("menus-message-{i}"))
            .context_menu(menu)
    };
    section((
        column((
            message_row(0, "Maya Chen", crate::res::str::msg_subject_one().format()),
            message_row(
                1,
                "Tomás Rivera",
                crate::res::str::msg_subject_two().format(),
            ),
            message_row(
                2,
                "Aiko Tanaka",
                crate::res::str::msg_subject_three().format(),
            ),
        ))
        .spacing(8.0),
        label(crate::res::str::menus_messages_hint()).font(Font::Footnote),
    ))
    .title(crate::res::str::menus_messages_section())
}

/// A media card with the sharing-flavored menu every photo grid grows eventually — the
/// context target is the IMAGE itself (the decorator attaches to whatever piece it follows).
fn photo_section() -> impl Piece {
    let log = |what: String| move || menu_log().set(what.clone());
    let share = crate::res::str::menu_share().format();
    let copy_image = crate::res::str::menu_copy_image().format();
    let save_image = crate::res::str::menu_save_image().format();
    let info = crate::res::str::menu_get_info().format();
    section((image(crate::res::images::day_logo)
        .frame(96.0, 96.0)
        .corner_radius(12.0)
        .id("menus-photo-target")
        .context_menu(vec![
            menu_item(share.clone()).action(log(share.clone())),
            menu_item(copy_image.clone()).action(log(copy_image.clone())),
            menu_item(save_image.clone()).action(log(save_image.clone())),
            menu_separator(),
            menu_item(info.clone()).action(log(info.clone())),
        ]),))
    .title(crate::res::str::menus_photo_section())
}

/// Imperative dialogs (docs/dialogs.md): each button opens a native dialog from within an
/// async task and writes a fixed result token to `modal-result` (locale-independent so the
/// walkthrough can assert it).
fn dialogs_section() -> impl Piece {
    let last = Signal::new(String::new());
    section((
        row((
            button(crate::res::str::modal_alert())
                .action(move || {
                    day::task(async move {
                        alert(crate::res::str::alert_title())
                            .message(crate::res::str::alert_body())
                            .button(crate::res::str::ok(), ())
                            .present()
                            .await;
                        last.set("alert-ok".into());
                    });
                })
                .tint(crate::widgets::primary())
                .id("btn-alert"),
            button(crate::res::str::modal_confirm())
                .bordered()
                .action(move || {
                    day::task(async move {
                        let ok = confirm(crate::res::str::confirm_title())
                            .message(crate::res::str::confirm_body())
                            .await;
                        last.set(if ok { "confirm-yes" } else { "confirm-no" }.into());
                    });
                })
                .id("btn-confirm"),
            button(crate::res::str::modal_delete())
                .action(move || {
                    day::task(async move {
                        let ok = confirm(crate::res::str::delete_title())
                            .message(crate::res::str::delete_body())
                            .confirm_label(crate::res::str::delete())
                            .destructive()
                            .await;
                        last.set(if ok { "delete-yes" } else { "delete-no" }.into());
                    });
                })
                .tint(crate::widgets::danger())
                .id("btn-delete"),
        ))
        .spacing(8.0),
        row((
            button(crate::res::str::modal_sheet())
                .bordered()
                .action(move || {
                    day::task(async move {
                        let choice = Alert::new(crate::res::str::flavor_title())
                            .sheet()
                            .button(crate::res::str::vanilla(), 0i64)
                            .button(crate::res::str::pistachio(), 1i64)
                            .cancel(crate::res::str::cancel())
                            .present()
                            .await;
                        last.set(match choice {
                            Some(i) => format!("sheet-{i}"),
                            None => "sheet-cancel".into(),
                        });
                    });
                })
                .id("btn-sheet"),
            button(crate::res::str::modal_prompt())
                .bordered()
                .action(move || {
                    day::task(async move {
                        let name = prompt(crate::res::str::name_placeholder()).await;
                        last.set(match name {
                            Some(t) => format!("prompt-{t}"),
                            None => "prompt-none".into(),
                        });
                    });
                })
                .id("btn-prompt"),
        ))
        .spacing(8.0),
        labeled(
            crate::res::str::modal_result_label(),
            label(move || {
                let v = last.get();
                if v.is_empty() { "—".into() } else { v }
            })
            .id("modal-result"),
        ),
    ))
    .title(crate::res::str::menus_dialogs_section())
}
