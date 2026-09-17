//! Native transfer between views, windows, toolkits, and independent processes.
use day::prelude::*;
use day::transfer::{Drop, Item, Offer, Operation, Representation, Target};
use std::{
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};
const CARD: &str = "application/vnd.day.showcase-card";
const ORIGIN: &str = "application/vnd.day.showcase-origin";
type Slot = Signal<Option<Item>>;
static NEXT: AtomicU64 = AtomicU64::new(1);

fn put(slot: Slot, bitmap: Signal<Option<day::Bitmap>>, item: Item) {
    bitmap.set(None);
    slot.set(Some(item.clone()));
    if let Some(bytes) = item.get("image/png") {
        let bytes = std::sync::Arc::new(bytes.to_vec());
        day::task(async move {
            if let Ok(decoded) = day::decode_image(bytes).await
                && slot.try_get().flatten().as_ref() == Some(&item)
            {
                bitmap.set(Some(decoded));
            }
        });
    }
}
pub(crate) fn drag_drop_page() -> AnyPiece {
    let page_id = NEXT.fetch_add(1, Ordering::Relaxed).to_string();
    let slots: [Slot; 6] = std::array::from_fn(|_| Signal::new(None));
    let bitmaps = std::array::from_fn::<_, 6, _>(|_| Signal::new(None::<day::Bitmap>));
    let enabled = Signal::new(true);
    let status = Signal::new(crate::res::str::dnd_ready().format());
    put(
        slots[0],
        bitmaps[0],
        Item::new(vec![Representation::new(
            "image/png",
            include_bytes!("../../resource/images/day_logo.png").as_slice(),
        )]),
    );
    put(
        slots[2],
        bitmaps[2],
        Item::new(vec![Representation::new(CARD, vec![0, 255, 128, 0, 42])]),
    );
    let zone = |index: usize| {
        let source_page = page_id.clone();
        let target_page = page_id.clone();
        let source = move |_| {
            let mut item = slots[index].get_untracked()?;
            item.representations.retain(|r| r.mime != ORIGIN);
            item.representations.push(Representation::new(
                ORIGIN,
                format!("{source_page}\n{index}"),
            ));
            Some(Offer { items: vec![item] })
        };
        let target = Target {
            types: vec!["image/png".into(), "text/uri-list".into(), CARD.into()],
            accept: Rc::new(move |at| {
                // The left 24 logical points are deliberately forbidden, even for valid data.
                if enabled.get_untracked()
                    && at.position.x >= 24.
                    && (at.has("image/png") || at.has("text/uri-list") || at.has(CARD))
                {
                    Operation::Copy
                } else {
                    Operation::None
                }
            }),
            receive: Rc::new(move |drop: Drop| {
                if drop.items.len() != 1 {
                    return false;
                }
                let Some(mut item) = drop.items.into_iter().next() else {
                    return false;
                };
                if item.get("image/png").is_none()
                    && item.get("text/uri-list").is_none()
                    && item.get(CARD).is_none()
                {
                    return false;
                }
                if let Some(origin) = item.get(ORIGIN).and_then(|b| std::str::from_utf8(b).ok())
                    && let Some((page, from)) = origin.split_once('\n')
                    && drop.local
                    && page == target_page
                    && let Ok(from) = from.parse::<usize>()
                    && from < slots.len()
                    && from != index
                {
                    slots[from].set(None);
                    bitmaps[from].set(None);
                }
                item.representations.retain(|r| r.mime != ORIGIN);
                let description = item
                    .representations
                    .iter()
                    .map(|r| {
                        tr("dnd_data")
                            .arg("mime", r.mime.clone())
                            .arg("count", r.bytes.len() as i64)
                            .format()
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                put(slots[index], bitmaps[index], item);
                status.set(description);
                true
            }),
        };
        column((
            canvas(move |d, size| {
                d.fill(
                    Shape::Rect(Rect::from_size(size)),
                    Color::rgba(0.2, 0.5, 0.8, 0.12),
                );
                d.fill(
                    Shape::Rect(Rect::new(0., 0., 24., size.height)),
                    Color::rgba(0.9, 0.2, 0.2, 0.35),
                );
                if let Some(image) = bitmaps[index].get() {
                    d.image(&image, Rect::new(36., 8., 96., 96.));
                } else if let Some(item) = slots[index].get() {
                    let color = if item.get(CARD).is_some() {
                        Color::rgba(0.95, 0.65, 0.15, 1.)
                    } else {
                        Color::rgba(0.25, 0.65, 0.45, 1.)
                    };
                    d.fill(Shape::Rect(Rect::new(54., 20., 64., 72.)), color);
                    for y in [36., 50., 64.] {
                        d.fill(
                            Shape::Rect(Rect::new(64., y, 44., 4.)),
                            Color::rgba(1., 1., 1., 0.85),
                        );
                    }
                }
            })
            .height(112.)
            .width(170.),
            label(move || {
                slots[index]
                    .get()
                    .map(|i| {
                        i.representations
                            .iter()
                            .map(|r| r.mime.clone())
                            .collect::<Vec<_>>()
                            .join("\n")
                    })
                    .unwrap_or_else(|| crate::res::str::dnd_empty().format())
            })
            .font(Font::Caption)
            .width(170.)
            .id(format!("dnd-value-{index}")),
        ))
        .drag_source(source)
        .drop_target(target)
        .id(format!("dnd-slot-{index}"))
    };
    crate::widgets::page_wide(
        crate::res::str::nav_drag_drop(),
        "dnd-title",
        column((
            label(crate::res::str::dnd_hint()),
            row((
                button(crate::res::str::dnd_new_window())
                    .action(|| {
                        day::open_new_window();
                    })
                    .id("dnd-new-window"),
                button(crate::res::str::dnd_choose_file())
                    .action(move || {
                        day::task(async move {
                            if let Some(file) = open_file().await {
                                let raw = file.as_str();
                                let uri = if raw.contains("://") {
                                    raw.to_string()
                                } else {
                                    let path = raw.replace('\\', "/");
                                    let encoded: String = path
                                        .bytes()
                                        .map(|b| {
                                            if b.is_ascii_alphanumeric() || b"/-._~:".contains(&b) {
                                                (b as char).to_string()
                                            } else {
                                                format!("%{b:02X}")
                                            }
                                        })
                                        .collect();
                                    format!(
                                        "file://{}{encoded}",
                                        if encoded.starts_with('/') { "" } else { "/" }
                                    )
                                };
                                put(
                                    slots[1],
                                    bitmaps[1],
                                    Item::new(vec![Representation::new(
                                        "text/uri-list",
                                        format!("{uri}\r\n"),
                                    )]),
                                );
                            }
                        });
                    })
                    .id("dnd-choose-file"),
                labeled(crate::res::str::dnd_accept(), toggle(enabled)).id("dnd-accept"),
            ))
            .spacing(12.)
            .fit(RowFit::Wrap { run_spacing: 12. }),
            row((zone(0), zone(1), zone(2)))
                .spacing(12.)
                .fit(RowFit::Wrap { run_spacing: 12. }),
            row((zone(3), zone(4), zone(5)))
                .spacing(12.)
                .fit(RowFit::Wrap { run_spacing: 12. }),
            label(move || status.get()).id("dnd-status"),
            crate::widgets::support_note(capability(Cap::DragDrop)),
        ))
        .spacing(16.)
        .any(),
    )
    .any()
}
