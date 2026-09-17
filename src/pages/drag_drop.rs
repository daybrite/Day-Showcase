//! Native transfer between views, windows, toolkits, and independent processes.
use day::prelude::*;
use day::transfer::{Drop, Item, Offer, Operation, Representation, Target};
use std::{
    io::Read,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};
const CARD: &str = "application/vnd.day.showcase-card";
const ORIGIN: &str = "application/vnd.day.showcase-origin";
const REJECTION_RADIUS: f64 = 24.;
type Slot = Signal<Option<Item>>;
static NEXT: AtomicU64 = AtomicU64::new(1);

// Keep the original transfer item: decoding a preview must not turn a file reference into
// an image-byte offer when the user drags it onward.
fn preview_bytes(item: &Item) -> Option<Arc<Vec<u8>>> {
    if let Some(png) = item.representations.iter().find(|r| r.mime == "image/png") {
        return Some(png.bytes.clone());
    }
    let paths = day::transfer::file_paths(item.get("text/uri-list")?)?;
    // One tile represents one item, not a gallery of files bundled into a URI list.
    if paths.len() != 1 {
        return None;
    }
    let file = std::fs::File::open(&paths[0]).ok()?;
    let metadata = file.metadata().ok()?;
    if !metadata.is_file() || metadata.len() > day::transfer::MAX_BYTES as u64 {
        return None;
    }
    let mut bytes = Vec::new();
    file.take(day::transfer::MAX_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .ok()?;
    (bytes.len() <= day::transfer::MAX_BYTES).then(|| Arc::new(bytes))
}

fn put(slot: Slot, bitmap: Signal<Option<day::Bitmap>>, item: Item) {
    bitmap.set(None);
    slot.set(Some(item.clone()));
    day::task(async move {
        if let Some(bytes) = preview_bytes(&item)
            && let Ok(decoded) = day::decode_image(bytes).await
            // A decode may complete after another drop, a move, or leaving this page.
            && slot.try_get().flatten().as_ref() == Some(&item)
        {
            bitmap.set(Some(decoded));
        }
    });
}
pub(crate) fn drag_drop_page() -> AnyPiece {
    let page_id = NEXT.fetch_add(1, Ordering::Relaxed).to_string();
    let slots: [Slot; 6] = std::array::from_fn(|_| Signal::new(None));
    let bitmaps = std::array::from_fn::<_, 6, _>(|_| Signal::new(None::<day::Bitmap>));
    let enabled = Signal::new(true);
    let disallowed_regions = Signal::new(false);
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
        // Keep every edge open to drag entry. Each tile has a stable interior rejection
        // circle; drawing and acceptance use the same center and radius.
        let rejection = [
            Point::new(134., 36.),
            Point::new(36., 76.),
            Point::new(130., 76.),
            Point::new(40., 36.),
            Point::new(116., 56.),
            Point::new(54., 68.),
        ][index];
        let rejection_color = [
            Color::rgba(0.85, 0.12, 0.16, 1.),
            Color::rgba(0.78, 0.32, 0.05, 1.),
            Color::rgba(0.12, 0.55, 0.29, 1.),
            Color::rgba(0.12, 0.38, 0.85, 1.),
            Color::rgba(0.48, 0.22, 0.75, 1.),
            Color::rgba(0.75, 0.15, 0.48, 1.),
        ][index];
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
                let dx = at.position.x - rejection.x;
                let dy = at.position.y - rejection.y;
                if enabled.get_untracked()
                    && (!disallowed_regions.get_untracked()
                        || dx * dx + dy * dy > REJECTION_RADIUS * REJECTION_RADIUS)
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
                if disallowed_regions.get() {
                    // Draw last so the rejection marker stays visible over dropped content.
                    d.fill(
                        Shape::Ellipse(Rect::new(
                            rejection.x - REJECTION_RADIUS,
                            rejection.y - REJECTION_RADIUS,
                            2. * REJECTION_RADIUS,
                            2. * REJECTION_RADIUS,
                        )),
                        rejection_color,
                    );
                    d.stroke(
                        PathBuilder::new()
                            .move_to(Point::new(rejection.x - 8., rejection.y - 8.))
                            .line_to(Point::new(rejection.x + 8., rejection.y + 8.))
                            .move_to(Point::new(rejection.x + 8., rejection.y - 8.))
                            .line_to(Point::new(rejection.x - 8., rejection.y + 8.))
                            .build(),
                        Color::rgba(1., 1., 1., 1.),
                        4.,
                    );
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
            .id(format!("dnd-value-{index}"))
            .width(170.),
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
                labeled(
                    crate::res::str::dnd_disallowed(),
                    toggle(disallowed_regions).id("dnd-disallowed"),
                ),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_file_reference_provides_preview_bytes_without_changing_offer() {
        let path =
            std::env::temp_dir().join(format!("day-showcase-drop-{} ü.png", std::process::id()));
        let png = include_bytes!("../../resource/images/day_logo.png");
        std::fs::write(&path, png).unwrap();
        let encoded: String = path
            .to_string_lossy()
            .replace('\\', "/")
            .bytes()
            .map(|b| {
                if b.is_ascii_alphanumeric() || b"/-._~:".contains(&b) {
                    (b as char).to_string()
                } else {
                    format!("%{b:02X}")
                }
            })
            .collect();
        let uri = format!(
            "file://{}{encoded}\r\n",
            if encoded.starts_with('/') { "" } else { "/" }
        );
        let item = Item::new(vec![Representation::new("text/uri-list", uri.clone())]);
        let bytes = preview_bytes(&item);
        std::fs::remove_file(path).unwrap();
        assert_eq!(bytes.unwrap().as_slice(), png);
        assert_eq!(item.get("text/uri-list"), Some(uri.as_bytes()));
        assert!(item.get("image/png").is_none());
        assert!(preview_bytes(&item).is_none()); // missing file
        for uris in [
            "https://example.com/image.png",
            "file://remote/image.png",
            &uri.repeat(2),
        ] {
            assert!(
                preview_bytes(&Item::new(vec![Representation::new("text/uri-list", uris)]))
                    .is_none()
            );
        }
    }

    #[test]
    fn inline_png_preview_shares_the_existing_bytes() {
        let image = Representation::new("image/png", vec![1, 2, 3]);
        let bytes = image.bytes.clone();
        assert!(Arc::ptr_eq(
            &preview_bytes(&Item::new(vec![image])).unwrap(),
            &bytes
        ));
        assert!(preview_bytes(&Item::new(vec![Representation::new(CARD, vec![42])])).is_none());
    }
}
