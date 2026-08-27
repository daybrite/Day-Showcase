use day::prelude::*;
use day_tweak_tree_style::{TreeStyle, TreeStyleTweak};

use crate::widgets::heading;

/// A hierarchical tree over app-owned rows (docs/tree.md): a mock project — folders, files,
/// a nested folder — small enough to read, deep enough to exercise the API. The page drives
/// every portable option from real controls: an app-owned expansion set (Expand/Collapse
/// All), two-way selection with a live readout, `reveal` (expands ancestors and scrolls),
/// drag-to-reparent through `movable` + `on_move` with a `move_guard` the Lock toggle arms
/// (macOS shows the native drop-deny affordance; every platform moves rows through each
/// row's context menu too), per-row context menus built at summon time, native type-ahead,
/// and a multi-select toggle that rebuilds the tree with the other flag.
///
/// The rows are a plain `Vec` behind one signal — `branches(items, key, parent)` derives the
/// hierarchy from the parent keys, and sibling order is the Vec's order, which is what the
/// context menu's Move Up/Down and a committed drag rotate.
#[derive(Clone)]
struct FileNode {
    id: u32,
    parent: Option<u32>,
    name: &'static str,
    folder: bool,
}

/// The last committed move, for the readout: (node, new parent, sibling index).
type LastMove = (u32, Option<u32>, Option<usize>);

/// The folder the Lock toggle guards drops into.
const DOCS: u32 = 7;
/// The deep leaf the Reveal button targets (two collapsed ancestors above it).
const REVEAL_TARGET: u32 = 5;

fn seed() -> Vec<FileNode> {
    let n = |id, parent, name, folder| FileNode {
        id,
        parent,
        name,
        folder,
    };
    vec![
        n(1, None, "src", true),
        n(2, Some(1), "main.rs", false),
        n(3, Some(1), "lib.rs", false),
        n(4, Some(1), "pages", true),
        n(5, Some(4), "tree.rs", false),
        n(6, Some(4), "list.rs", false),
        n(7, None, "docs", true),
        n(8, Some(7), "README.md", false),
        n(9, None, "Cargo.toml", false),
    ]
}

/// Move `id` under `new_parent` at sibling position `index` (`None` = append): the exact
/// contract `on_move` hands the app (docs/tree.md "Moving nodes"). Sibling order IS the
/// Vec's relative order, so the rotation is: take the node out, then insert it back where
/// the target slot falls.
fn apply_move(v: &mut Vec<FileNode>, id: u32, new_parent: Option<u32>, index: Option<usize>) {
    let Some(from) = v.iter().position(|n| n.id == id) else {
        return;
    };
    let mut node = v.remove(from);
    node.parent = new_parent;
    let mut seen = 0usize;
    let mut at = v.len();
    for (i, n) in v.iter().enumerate() {
        if n.parent == new_parent {
            if index == Some(seen) {
                at = i;
                break;
            }
            seen += 1;
            at = i + 1; // append lands after the last sibling, not at the Vec's end
        }
    }
    if index.is_none() && seen == 0 {
        // First child of an empty folder: right after the folder row itself.
        if let Some(p) = new_parent
            && let Some(i) = v.iter().position(|n| n.id == p)
        {
            at = i + 1;
        }
    }
    v.insert(at.min(v.len()), node);
}

/// Swap `id` with its previous/next SIBLING (same parent) in the Vec — the context menu's
/// Move Up/Down, which is what rearranges rows on the platforms without native tree drag.
fn nudge(v: &mut [FileNode], id: u32, up: bool) {
    let Some(i) = v.iter().position(|n| n.id == id) else {
        return;
    };
    let parent = v[i].parent;
    let sibling = if up {
        v[..i].iter().rposition(|n| n.parent == parent)
    } else {
        v[i + 1..]
            .iter()
            .position(|n| n.parent == parent)
            .map(|d| i + 1 + d)
    };
    if let Some(j) = sibling {
        v.swap(i, j);
    }
}

/// `id` and every descendant, gone — the context menu's Delete.
fn remove_subtree(v: &mut Vec<FileNode>, id: u32) {
    let mut doomed = vec![id];
    let mut i = 0;
    while i < doomed.len() {
        let p = doomed[i];
        doomed.extend(v.iter().filter(|n| n.parent == Some(p)).map(|n| n.id));
        i += 1;
    }
    v.retain(|n| !doomed.contains(&n.id));
}

pub(crate) fn tree_page() -> AnyPiece {
    let nodes: Signal<Vec<FileNode>> = Signal::new(seed());
    // The app-owned expansion set (docs/tree.md "Expansion"): src/ starts open, docs/ and
    // pages/ closed — so both disclosure directions are one click (and one assert) away.
    let open: Signal<std::collections::HashSet<u32>> =
        Signal::new(std::collections::HashSet::from([1]));
    let selection: Signal<Vec<u32>> = Signal::new(Vec::new());
    let reveal: Signal<Option<u32>> = Signal::new(None);
    let multi = Signal::new(false);
    let locked = Signal::new(true);
    // The last committed move, for the readout (and the walkthrough's structure assert).
    let last_move: Signal<Option<LastMove>> = Signal::new(None);
    let next_id = Signal::new(100u32);

    let name_of = move |id: u32| {
        nodes.with_untracked(|v| {
            v.iter()
                .find(|n| n.id == id)
                .map(|n| n.name.to_string())
                .unwrap_or_default()
        })
    };

    let add_into = move || {
        // Into the first SELECTED folder, else the root — and open the folder so the new
        // row is visible immediately.
        let parent = selection
            .get_untracked()
            .iter()
            .copied()
            .find(|id| nodes.with_untracked(|v| v.iter().any(|n| n.id == *id && n.folder)));
        let id = next_id.get_untracked();
        next_id.set(id + 1);
        nodes.update(|v| {
            v.push(FileNode {
                id,
                parent,
                name: "new-note.md",
                folder: false,
            });
        });
        if let Some(p) = parent {
            open.update(|s| {
                s.insert(p);
            });
        }
        selection.set(vec![id]);
    };

    // One builder for both multi-select arms: the flag is a BUILD-TIME option, so the
    // toggle rebuilds the tree (a `when` swap) rather than patching it.
    let build_tree = move |multi_on: bool| {
        tree(
            branches(
                move || nodes.get(),
                |n: &FileNode| n.id,
                |n: &FileNode| n.parent,
            ),
            move |slot: ItemSlot<FileNode, u32>| {
                row((
                    label(move || {
                        if slot.field(|n| n.folder) {
                            "📁".to_string()
                        } else {
                            "📄".to_string()
                        }
                    })
                    .width(24.0),
                    label(move || slot.field(|n| n.name.to_string())),
                ))
                .spacing(6.0)
                .padding(Insets::symmetric(6.0, 0.0))
            },
        )
        .row_height(RowHeight::Uniform(30.0))
        .indent(16.0)
        .expanded(open)
        .expandable(move |id: &u32| {
            nodes.with_untracked(|v| v.iter().any(|n| n.id == *id && n.folder))
        })
        .multi_select(multi_on)
        .selected(move || selection.get())
        .on_selection(move |keys: Vec<u32>| selection.set(keys))
        .movable(true)
        .on_move(move |id: u32, parent: Option<u32>, index: Option<usize>| {
            nodes.update(|v| apply_move(v, id, parent, index));
            last_move.set(Some((id, parent, index)));
        })
        // The structural refusals (into itself, into a descendant, into a leaf) are built
        // in; this is the APP's rule: docs/ takes no drops while the lock is on — macOS
        // shows the native no-drop cursor live from this verdict.
        .move_guard(
            move |_id: &u32, parent: Option<&u32>, _index: Option<usize>| {
                if locked.get_untracked() && parent == Some(&DOCS) {
                    MoveVerdict::Deny
                } else {
                    MoveVerdict::Allow
                }
            },
        )
        .type_ahead(move |id: &u32| name_of(*id))
        .reveal(reveal)
        .row_id(|id: &u32| format!("tree-node-{id}"))
        // Built at SUMMON time (docs/menus.md "Dynamic context menus"), so the items match
        // the row under the pointer — and a summon outside the selection selects it first.
        .row_context_menu(move |id: &u32| {
            let id = *id;
            if !selection.get_untracked().contains(&id) {
                selection.set(vec![id]);
            }
            let folder = nodes.with_untracked(|v| v.iter().any(|n| n.id == id && n.folder));
            let mut entries = Vec::new();
            if folder {
                entries.push(
                    menu_item(crate::res::str::tree_ctx_new_file().format()).action(move || {
                        let nid = next_id.get_untracked();
                        next_id.set(nid + 1);
                        nodes.update(|v| {
                            v.push(FileNode {
                                id: nid,
                                parent: Some(id),
                                name: "new-note.md",
                                folder: false,
                            });
                        });
                        open.update(|s| {
                            s.insert(id);
                        });
                        selection.set(vec![nid]);
                    }),
                );
            } else {
                entries.push(
                    menu_item(crate::res::str::tree_ctx_duplicate().format()).action(move || {
                        let nid = next_id.get_untracked();
                        next_id.set(nid + 1);
                        nodes.update(|v| {
                            if let Some(i) = v.iter().position(|n| n.id == id) {
                                let mut copy = v[i].clone();
                                copy.id = nid;
                                v.insert(i + 1, copy);
                            }
                        });
                        selection.set(vec![nid]);
                    }),
                );
            }
            entries.push(menu_separator());
            entries.push(
                menu_item(crate::res::str::tree_ctx_move_up().format())
                    .action(move || nodes.update(|v| nudge(v, id, true))),
            );
            entries.push(
                menu_item(crate::res::str::tree_ctx_move_down().format())
                    .action(move || nodes.update(|v| nudge(v, id, false))),
            );
            entries.push(menu_separator());
            entries.push(
                menu_item(crate::res::str::tree_ctx_delete().format()).action(move || {
                    nodes.update(|v| remove_subtree(v, id));
                    selection.update(|s| s.retain(|k| *k != id));
                }),
            );
            entries
        })
        // `.id` chains on the TREE itself — the driver `expand:`/`tree_move:` address lives
        // on its node, and a decorator like `.height` wraps a new one (memory: .id before
        // wrapper decorators).
        .id("demo-tree")
        // day-tweak-tree-style: the sidebar treatment — clear backgrounds over the pane on
        // macOS, Adwaita's navigation-sidebar class on GTK — and a no-op on the toolkits
        // whose trees are composed from day pieces (docs/tweaks.md).
        .tree_style(TreeStyle::sidebar())
        .height(340.0)
    };

    let folder_ids =
        move || nodes.with_untracked(|v| v.iter().filter(|n| n.folder).map(|n| n.id).collect());

    column((
        row((
            heading(crate::res::str::nav_tree(), "tree-title", None),
            spacer(),
            button(crate::res::str::tree_add_file())
                .prominent()
                .action(add_into)
                .tint(crate::widgets::primary())
                .id("tree-add"),
        ))
        .spacing(8.0),
        label(crate::res::str::tree_caption())
            .font(Font::Footnote)
            .id("tree-caption"),
        row((
            button(crate::res::str::tree_expand_all())
                .bordered()
                .action(move || open.set(folder_ids()))
                .id("tree-expand-all"),
            button(crate::res::str::tree_collapse_all())
                .bordered()
                .action(move || open.set(std::collections::HashSet::new()))
                .id("tree-collapse-all"),
            button(crate::res::str::tree_reveal())
                .bordered()
                // Two collapsed ancestors sit above the target: reveal expands them through
                // the SAME app-owned signal the chevrons write, then scrolls.
                .action(move || reveal.set(Some(REVEAL_TARGET)))
                .id("tree-reveal"),
        ))
        .spacing(8.0)
        .fit(RowFit::Wrap { run_spacing: 8.0 }),
        row((
            labeled(
                crate::res::str::tree_multi(),
                toggle(multi).id("tree-multi"),
            ),
            labeled(crate::res::str::tree_lock(), toggle(locked).id("tree-lock")),
        ))
        .spacing(16.0)
        .fit(RowFit::Wrap { run_spacing: 8.0 }),
        label(crate::res::str::tree_hint())
            .font(Font::Footnote)
            .id("tree-hint"),
        when(move || multi.get(), move || build_tree(true)).otherwise(move || build_tree(false)),
        label(move || crate::res::str::tree_count(nodes.get().len() as i64).format())
            .font(Font::Footnote)
            .tabular()
            .id("tree-count"),
        label(move || {
            let sel = selection.get();
            if sel.is_empty() {
                crate::res::str::tree_selection_none().format()
            } else {
                let names: Vec<String> = sel.iter().map(|id| name_of(*id)).collect();
                crate::res::str::tree_selection(sel.len() as i64, names.join(", ")).format()
            }
        })
        .font(Font::Footnote)
        .id("tree-selection"),
        label(move || match last_move.get() {
            None => crate::res::str::tree_move_none().format(),
            Some((id, parent, index)) => {
                let target = match parent {
                    Some(p) => name_of(p),
                    None => crate::res::str::tree_root().format(),
                };
                match index {
                    Some(i) => crate::res::str::tree_move(i as i64, name_of(id), target).format(),
                    None => crate::res::str::tree_move_append(name_of(id), target).format(),
                }
            }
        })
        .font(Font::Footnote)
        .id("tree-last-move"),
    ))
    .spacing(10.0)
    .align(HAlign::Leading)
    .padding(16.0)
    .any()
}
