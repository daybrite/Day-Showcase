//! The Content List page: the `day new` scaffold's own item list and editor, exactly as a new
//! app starts with them (https://daybrite.dev/docs/navigation "The content list").
//!
//! `model.rs`, `navigate.rs` and `detail.rs` are GENERATED from the scaffold template by
//! `scripts/template-sync.sh` and checked against it in CI — edit the template, not them. What
//! is hand-written here is the glue a crate root normally supplies: the route enum the model's
//! scene defaults to, and the two builders `lib.rs` hands the navigation host. The list is the
//! host's own content-list pane, shown only on this page (`content_list_for` in `lib.rs`), so
//! its Filter and Add and the editor's Done ride the pane and leave the bar with it
//! (https://daybrite.dev/docs/toolbars).

use day::prelude::*;

mod detail;
// The scaffold's `lib.rs` is the one consumer of `Scene::section` (its selector binds it);
// this app routes with `crate::Section` and mounts only the page, so the field is carried
// unread rather than edited out of the generated copy.
#[allow(dead_code)]
mod model;
// The scaffold titles its pushed editor after its own Navigate section when no item is open;
// `lib.rs` titles this app's after the Content List section instead, so the generated
// `detail_title` is carried unread like `Scene::section` above.
#[allow(dead_code)]
mod navigate;

pub(crate) use model::Scene;
pub(crate) use navigate::item_list_pane;

day::routes! {
    /// The scaffold's sections, as the copied model expects them: it opens a fresh scene on
    /// `Welcome` and reads nothing else. The showcase's own routing is `crate::Section`; this
    /// enum is only the type the copied `Scene::section` field wants.
    pub(crate) enum Section {
        Welcome => "welcome",
        Navigate => "navigate",
        Settings => "settings",
    }
}

/// The page: the editor for whichever row the content-list pane selected, or the empty state.
pub(crate) fn content_list_page() -> AnyPiece {
    navigate::navigate_page().any()
}
