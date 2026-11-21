use crate::Section;
use crate::palette::{CORAL, RAMP, SKY, VIOLET};
use day::prelude::*;

/// The stack's typed destinations (docs/navigation.md): a data-carrying [`Route`].
/// `Depth(n)` ↔ `"n"` and `Item { id }` ↔ `"item-<id>"`, so the wire route
/// `stack/item-42?hint=linked` parses back into `Item { id: 42 }`; the destination builder
/// matches on the typed value instead of string-splitting keys.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Drill {
    Depth(u32),
    Item { id: u32 },
}

impl Route for Drill {
    fn key(&self) -> String {
        match self {
            Drill::Depth(n) => n.to_string(),
            Drill::Item { id } => format!("item-{id}"),
        }
    }
    fn from_key(key: &str) -> Option<Self> {
        if let Some(id) = key.strip_prefix("item-") {
            return id.parse().ok().map(|id| Drill::Item { id });
        }
        key.parse().ok().map(Drill::Depth)
    }
}

day::routes! {
    /// The fullscreen cover's route. A cover is a route surface as well as a signal, and a
    /// one-variant route claims only its own key, where a `String` key would claim every
    /// segment routed while this page is current (docs/cover.md). This one stays routed, so
    /// `navigate("cover")` opens it and `nav_back()` closes it; the three sheets are panels
    /// with nothing to link to, so each binds a `bool` instead (docs/cover.md "Open or
    /// closed").
    enum CoverKey { Open => "cover" }
}

/// Push/pop navigation (docs/navigation.md): `stack` bound to a `Signal<Vec<Drill>>`
/// path. Pushing a detail appends a typed value to the path; Day reconciles the native
/// UINavigationController / AdwNavigationView / back-stack; the native back button writes the
/// pop back into the path.
pub(crate) fn stack_page() -> AnyPiece {
    fn push(path: Signal<Vec<Drill>>) {
        let mut v = path.get_untracked();
        let n = v.len() as u32 + 1;
        v.push(Drill::Depth(n));
        path.set(v);
    }
    let path = Signal::new(Vec::<Drill>::new());
    // Back-interception demo (docs/navigation.md): the detail page has an "unsaved changes"
    // toggle; while set, on_back shows a confirm dialog and pops only if the user agrees. The
    // native back gesture/button routes through this guard on every backend.
    let dirty = Signal::new(false);
    // Fullscreen cover (docs/cover.md): a projection of an app-owned Option signal, a native
    // modal on iOS and Android and a topmost full-window child elsewhere. Its switch hides the
    // status bar while the cover is up, on the toolkits that have one.
    let cover_open = Signal::new(Option::<CoverKey>::None);
    let hide_status_bar = Signal::new(false);
    let has_status_bar = capability(Cap::StatusBarHidden) == Support::Native;
    // Half sheet (docs/cover.md "Half sheet"): a cover bound to a bool, presented over the bottom
    // half of the window, the page staying on show. A medium-detent sheet on iOS, a Material
    // bottom sheet on Android, a bottom-anchored panel on the web. The slider in the sheet and
    // the readout on the page share one signal, so the page can be seen following the sheet.
    let half_open = Signal::new(false);
    // Large sheet (docs/cover.md "Large sheet"): the modal sheet with room for a picture or a
    // document. The window sheet on macOS, a page sheet on iPadOS, a composed card on the other
    // wide windows, and the full-window cover on a compact one.
    let large_open = Signal::new(false);
    // Locked sheet (docs/cover.md "The shield modifiers"): while its switch is on, the sheet
    // mounts `interactive_dismiss_disabled`, and Escape, the pull-down and the system Back leave
    // it open. Its button writes the signal, which always closes it.
    let locked_open = Signal::new(false);
    let locked = Signal::new(true);
    let level = Signal::new(50.0_f64);
    let back = crate::scene();
    // The root page, composed as the gallery's card pages are: the motif beside what the page
    // is, then a card for each thing a stack does.
    let intro = row((
        stack_glyph(),
        label(crate::res::str::detail_stack())
            .id(day::id!(stack_root))
            .grow(Dimension::Width),
    ))
    .spacing(Space::M)
    .align(VAlign::Center)
    .grow(Dimension::Width);
    let back_card = crate::gallery::panel(
        crate::res::str::stack_back_title(),
        (
            crate::gallery::note(crate::res::str::system_back_hint()),
            button(crate::res::str::system_back_arm())
                .icon(Symbol::History)
                .action(move || {
                    back.back_handled.set(0);
                    back.back_pending.set(3);
                })
                .id(day::id!(system_back_arm)),
            label(move || {
                crate::res::str::system_back_status(crate::res::str::SystemBackStatus {
                    remaining: back.back_pending.get(),
                    handled: back.back_handled.get(),
                })
                .format()
            })
            .id(day::id!(system_back_page_status)),
        ),
    );
    let push_card = crate::gallery::panel(
        crate::res::str::stack_pages_title(),
        (crate::gallery::actions((
            button(crate::res::str::stack_push())
                .prominent()
                .action(move || push(path))
                .tint(crate::widgets::primary())
                .id(day::id!(stack_push)),
            // An absolute route with query params (docs/navigation.md), built typed: it anchors
            // the enclosing nav at Section::Stack, resets this stack, pushes Item { id: 42 };
            // the destination builder reads the ?hint= param via route_param().
            nav_link_to(
                crate::res::str::stack_link_42(),
                route(&Section::Stack)
                    .then(&Drill::Item { id: 42 })
                    .param("hint", "linked"),
            )
            .id(day::id!(stack_link)),
        )),),
    );
    let cover_card = crate::gallery::panel(
        crate::res::str::stack_covers_title(),
        (
            // The presenters share a row. The covers themselves mount no view of their own
            // here, so they ride at the row's end, where they cost the card no gap.
            crate::gallery::actions((
                button(crate::res::str::stack_cover_button())
                    .action(move || cover_open.set(Some(CoverKey::Open)))
                    .tint(crate::widgets::secondary())
                    .id(day::id!(cover_present)),
                button(crate::res::str::stack_half_button())
                    .action(move || half_open.set(true))
                    .tint(crate::widgets::secondary())
                    .id(day::id!(half_present)),
                button(crate::res::str::stack_large_button())
                    .action(move || large_open.set(true))
                    .tint(crate::widgets::secondary())
                    .id(day::id!(large_present)),
                button(crate::res::str::stack_locked_button())
                    .action(move || locked_open.set(true))
                    .tint(crate::widgets::secondary())
                    .id(day::id!(locked_present)),
                cover(cover_open, move |_| {
                    let content = column((
                        label(crate::res::str::cover_title())
                            .font(Font::Title)
                            .id(day::id!(cover_title)),
                        column((
                            labeled(
                                crate::res::str::stack_status_bar(),
                                toggle(hide_status_bar)
                                    .enabled(move || has_status_bar)
                                    .id(day::id!(cover_status_bar)),
                            ),
                            // Desktop and web windows have no status bar to hide.
                            (!has_status_bar).then(|| {
                                crate::gallery::note(crate::res::str::stack_status_bar_none())
                            }),
                        ))
                        .spacing(Space::S)
                        .align(HAlign::Leading)
                        .max_width(360.0),
                        button(crate::res::str::cover_dismiss())
                            .prominent()
                            .action(move || cover_open.set(None))
                            .id(day::id!(cover_dismiss)),
                    ))
                    .spacing(Space::M)
                    .padding(Space::XL)
                    .status_bar_hidden(hide_status_bar);
                    nav_stack(Signal::new(Vec::<Drill>::new()), content)
                        .title(crate::res::str::cover_title())
                        .toolbar(
                            toolbar_button(
                                day::id!(cover_toolbar_dismiss),
                                crate::res::str::cover_dismiss(),
                            )
                            .action(move || cover_open.set(None)),
                        )
                        .any()
                }),
                cover(half_open, move |_| {
                    column((
                        label(crate::res::str::half_title())
                            .font(Font::Title)
                            .id(day::id!(half_title)),
                        labeled(
                            crate::res::str::half_level(),
                            slider(level).range(0.0..=100.0).id(day::id!(half_level)),
                        ),
                        button(crate::res::str::cover_dismiss())
                            .prominent()
                            .action(move || half_open.set(false))
                            .id(day::id!(half_dismiss)),
                    ))
                    .spacing(Space::M)
                    .padding(Space::XL)
                    .any()
                })
                .half_sheet(),
                cover(large_open, move |_| {
                    column((
                        label(crate::res::str::stack_large_title())
                            .font(Font::Title)
                            .id(day::id!(large_title)),
                        day::http_image(crate::gallery::sky::url(
                            crate::gallery::sky::SATURN,
                            crate::gallery::sky::Size::Large,
                        ))
                        .rounded(12.0)
                        .placeholder_color(crate::gallery::INSET)
                        .grow(Dimension::Both),
                        button(crate::res::str::cover_dismiss())
                            .prominent()
                            .action(move || large_open.set(false))
                            .id(day::id!(large_dismiss)),
                    ))
                    .spacing(Space::M)
                    .padding(Space::XL)
                    .grow(Dimension::Both)
                    .any()
                })
                .large_sheet(),
                cover(locked_open, move |_| {
                    // The shield holds while the subtree that carries it is mounted, so the
                    // title is built with it while the switch is on and without it otherwise.
                    let title = || {
                        label(crate::res::str::stack_locked_title())
                            .font(Font::Title)
                            .id(day::id!(locked_title))
                    };
                    column((
                        when(
                            move || locked.get(),
                            move || title().interactive_dismiss_disabled(),
                        )
                        .otherwise(title),
                        labeled(
                            crate::res::str::stack_locked_toggle(),
                            toggle(locked).id(day::id!(locked_toggle)),
                        ),
                        button(crate::res::str::cover_dismiss())
                            .prominent()
                            .action(move || locked_open.set(false))
                            .id(day::id!(locked_dismiss)),
                    ))
                    .spacing(Space::M)
                    .padding(Space::XL)
                    .any()
                })
                .sheet(),
            )),
            label(move || {
                crate::res::str::half_readout((level.get().round() as i64).to_string()).format()
            })
            .id(day::id!(half_readout)),
        ),
    );
    let root = scroll(
        column((column((
            intro,
            crate::gallery::balanced(2, vec![push_card, cover_card, back_card]),
        ))
        .spacing(Space::L)
        .align(HAlign::Leading)
        .max_width(crate::widgets::CARDS_MAX_WIDTH),))
        .align(HAlign::Center)
        .grow(Dimension::Width)
        .padding(20.0),
    );
    nav_stack(path, root)
        .destination(move |d: &Drill| {
            // The typed value arrives parsed: match, don't split strings.
            let (title, depth) = match d {
                Drill::Depth(n) => (crate::res::str::stack_detail_title(n.to_string()), *n),
                Drill::Item { id } => (crate::res::str::stack_item_title(id.to_string()), 1),
            };
            // Params travel with navigate(): a push performed by writing the path signal
            // carries its data in the route value itself (docs/navigation.md). The hint row
            // exists only when the param does (an `Option` child builds nothing for `None`),
            // so paramless pushes get no phantom gap.
            scroll(
                column((
                    depth_dots(depth),
                    label(title).font(Font::Title).id(day::id!(stack_detail)),
                    route_param("hint").map(|h| {
                        label(crate::res::str::stack_param_hint(h))
                            .font(Font::Footnote)
                            .id(day::id!(stack_param))
                    }),
                    labeled(
                        crate::res::str::stack_unsaved(),
                        toggle(dirty).id(day::id!(stack_dirty)),
                    ),
                    button(crate::res::str::stack_push())
                        .prominent()
                        .action(move || push(path))
                        .id(day::id!(stack_deeper)),
                ))
                .spacing(Space::M)
                .align(HAlign::Leading)
                .padding(Space::L),
            )
        })
        .on_back(move |req| {
            if dirty.get() {
                // Confirm before losing edits; pop only on "yes" (docs/navigation.md).
                day::task(async move {
                    if confirm(crate::res::str::stack_discard_title())
                        .message(crate::res::str::stack_discard_body())
                        .confirm_label(crate::res::str::stack_discard_ok())
                        .await
                    {
                        dirty.set(false);
                        req.proceed();
                    }
                });
                BackResponse::Handled
            } else {
                BackResponse::Proceed
            }
        })
        .id(day::id!(demo_stack))
        .any()
}

/// The page's motif: three offset cards, drawn as one canvas leaf, a stack you can see.
fn stack_glyph() -> impl Piece {
    shape_group([
        rounded_rectangle(8.0).fill(SKY).at(0.16, 0.0, 0.68, 0.42),
        rounded_rectangle(8.0)
            .fill(VIOLET)
            .at(0.08, 0.26, 0.84, 0.42),
        rounded_rectangle(8.0).fill(CORAL).at(0.0, 0.54, 1.0, 0.46),
    ])
    .frame(84.0, 60.0)
}

/// The path so far, one numbered dot per level in the sunrise ramp's order; each push adds
/// a dot, each pop removes one, mirroring the `Vec<Drill>` behind the native stack.
fn depth_dots(depth: u32) -> impl Piece {
    let dots: Vec<_> = (1..=depth)
        .map(|n| {
            zstack((
                circle()
                    .fill(RAMP[((n - 1) % RAMP.len() as u32) as usize])
                    .frame(24.0, 24.0),
                label(n.to_string())
                    .font(Font::Caption)
                    .color(Color::WHITE)
                    .bold(),
            ))
        })
        .collect();
    row(dots).spacing(6.0)
}
