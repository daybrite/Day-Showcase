//! The Content List page: an item list and its editor sharing one observable model
//! (https://daybrite.dev/docs/how-to/navigation "The content list").
//!
//! `model.rs`, `items.rs` and `detail.rs` began as the `day new` starter's list, editor and
//! model, and are the showcase's own code now; Day-Rise is the app that tracks what `day new`
//! produces. This file is the glue a crate root would supply: the route enum the model's scene
//! defaults to, and the two builders `lib.rs` hands the navigation host. The list is the
//! host's own content-list pane, shown only on this page (`content_list_for` in `lib.rs`), so
//! its Add and the editor's Delete ride the pane and leave the bar with it
//! (https://daybrite.dev/docs/reference/toolbars).

use day::prelude::*;

mod detail;
// `items::detail_title` titles the pushed editor after the model's Items section when no item is
// open; this app titles it after the Content List section instead (`detail_title` below), so
// that one is unread.
#[allow(dead_code)]
mod items;
// `Scene::section` is written by `new_item` and read by nothing: this app routes with
// `crate::Section` and mounts only the page.
#[allow(dead_code)]
mod model;

pub(crate) use items::item_list_pane;
use model::ItemFields;
pub(crate) use model::Scene;

day::routes! {
    /// The sections the model's `Scene::section` field names: a fresh scene opens on
    /// `Welcome` and moves to `Items` when it adds one. The showcase routes with
    /// `crate::Section`; this enum is only the type that field wants.
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
