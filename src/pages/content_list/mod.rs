//! The Content List page: the `day new` scaffold's own item list and editor, exactly as a new
//! app starts with them (https://daybrite.dev/docs/navigation "The content list").
//!
//! `model.rs`, `items.rs` and `detail.rs` are generated from the scaffold template by
//! `scripts/template-sync.sh` and checked against it in CI; edit the template, not them. What
//! is hand-written here is the glue a crate root normally supplies: the route enum the model's
//! scene defaults to, and the two builders `lib.rs` hands the navigation host. The list is the
//! host's own content-list pane, shown only on this page (`content_list_for` in `lib.rs`), so
//! its Add and the editor's Delete ride the pane and leave the bar with it
//! (https://daybrite.dev/docs/toolbars).

use day::prelude::*;

mod detail;
// The scaffold titles its pushed editor after its own Items section when no item is open;
// this app titles it after the Content List section instead (`detail_title` below), so the
// generated `detail_title` is carried unread.
#[allow(dead_code)]
mod items;
// The scaffold's `lib.rs` is the one reader of `Scene::section` (its nav binds it); this app
// routes with `crate::Section` and mounts only the page, so the field is written by the
// generated `new_item` and read by nothing.
#[allow(dead_code)]
mod model;

pub(crate) use items::item_list_pane;
use model::ItemFields;
pub(crate) use model::Scene;

day::routes! {
    /// The scaffold's sections, as the copied model expects them: it opens a fresh scene on
    /// `Welcome` and moves to `Items` when it adds one. The showcase's own routing is
    /// `crate::Section`; this enum is only the type the copied `Scene::section` field wants.
    pub(crate) enum Section {
        Welcome => "welcome",
        Items => "items",
        Settings => "settings",
    }
}

/// The page: the editor for whichever row the content-list pane selected, or the empty state.
pub(crate) fn content_list_page() -> AnyPiece {
    items::items_page().any()
}

/// The pushed editor's bar title on a phone: the open item's name, live, and this section's
/// own title while the name is empty or nothing is open.
pub(crate) fn detail_title(scene: Scene) -> String {
    let name = scene
        .selected
        .get()
        .map(|id| scene.items.elem(id).name().read())
        .unwrap_or_default();
    if name.is_empty() {
        crate::res::str::nav_content_list().format()
    } else {
        name
    }
}
