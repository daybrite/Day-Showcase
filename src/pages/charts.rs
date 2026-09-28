//! Charts (day-piece-charts): every example page of the charting piece, with a picker between
//! them. The pages are day-piece-charts' own, from its `day-piece-charts-gallery` crate, the same
//! code its Charts Demo app is a sidebar of, so what this page shows is exactly what the demo and
//! the piece's README show, controls and animations included.
//!
//! Nothing here is chart code: the gallery brings the pages and their strings (a private catalog
//! that follows this app's language), and this page gives them a heading and the room to grow.

use day::prelude::*;
use day_piece_charts_gallery::{ChartPage, gallery};

use crate::widgets::heading;

thread_local! {
    /// The open chart, kept for the life of the app, so leaving the section and coming back
    /// returns to the same chart. Global rather than page-scoped for exactly that reason.
    static OPEN: Signal<ChartPage> = Signal::global(ChartPage::Pipeline);
}

pub(crate) fn charts_page() -> AnyPiece {
    let open = OPEN.with(|s| *s);
    // Not `widgets::page`: that scrolls, and a chart sizes itself by growing into the room it is
    // given, which a scroll view never offers. Each gallery page scrolls or grows on its own.
    column((
        heading(crate::res::str::nav_charts(), "charts-title"),
        gallery(open),
    ))
    .spacing(8.0)
    .padding(20.0)
    .grow()
    .any()
}
