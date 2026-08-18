use day::prelude::*;
use day_piece_colorpicker::color_picker;
use day_piece_texteditor::text_editor;

use crate::widgets::page;

// Demo filler for the seed buttons — sample documents rather than interface text, so the two long
// ones stay in English on every locale, the way a document a user opened would. The SHORT one is
// different: it is what the editor holds before anyone presses anything, so under a French UI it
// is the page's own copy and it goes through the catalog (docs/localization.md).
const LONG: &str = "\
Day lays out native widgets from a declarative description — you write the shape of the UI once and \
each platform's real toolkit draws it. There is no webview and no custom renderer; a button is the \
platform's button, a text area is the platform's editor.

State is reactive: a signal drives the tree, and only the widgets that depend on a changed value are \
touched. Text you type here flows back into the bound signal, and programmatic writes flow back out — \
a controlled input, in both directions.

Scroll this text if it outgrows the editor's height band. The band grows with the content between a \
minimum and a maximum number of lines, then the editor scrolls internally.";
const MARKDOWN: &str = "\
# Text areas

A `text_area` is a native multi-line editor with three controllable attributes:

- **editable** — read-only when off
- **selectable** — copyable even when read-only
- **spell-check** — the red squiggles, where the toolkit has them

See the [documentation](https://daybrite.dev/docs/textarea) for the per-toolkit support matrix.

    // a fenced code sample renders as plain text here
    text_area(content).editable(false).spellcheck(false)";

/// The formatted note the styled editor opens on — Markdown, parsed into runs and paragraphs by
/// `StyledText::markdown`, which is the same parse a `.markdown()` label does.
const NOTE: &str = "\
# Release notes

The **styled editor** is the platform's own rich-text view: `NSTextView` on macOS, `UITextView` on \
iOS, a `GtkTextView` over its tag table, `QTextEdit`, an Android `EditText` over its live span \
buffer, a XAML `RichEditBox`, the ArkTS `RichEditor`, and a `contenteditable` element on the web.

- Select some text and press *B*, *I* or *U*.
- Everything the toolbar does is a pure function over the bound document.
- Nothing here is drawn by Day: the caret, the selection handles and the IME are the toolkit's.

> A quotation, to show a paragraph attribute travelling with its text.";

/// The syntax-highlighting sample. Deliberately small: it is re-tokenized on EVERY keystroke, and
/// the point is to show that re-styling does not disturb the caret.
const CODE: &str = "\
// A counter, in Day.
fn counter() -> AnyPiece {
    let n = Signal::new(0);
    column((
        label(move || format!(\"Count: {}\", n.get())),
        button(\"Add one\").action(move || n.update(|v| *v += 1)),
    ))
    .spacing(8.0)
    .any()
}";

/// Which document the styled editor is showing.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Doc {
    Note,
    Code,
    Empty,
}

pub(crate) fn text_areas_page() -> AnyPiece {
    page(
        crate::res::str::nav_textareas(),
        "textareas-title",
        Some(crate::res::str::textareas_caption()),
        {
            // The STYLED editor leads: it is what the page is now about, and the plain
            // `text_area` below it is the simpler control it grew out of.
            let (editor, seed, attrs) = plain_sections();
            form((styled_sections(), editor, seed, attrs)).any()
        },
    )
}

// ---------------------------------------------------------------------------
// The plain editor — still the right control for a chat composer, a commit message, a note.
// ---------------------------------------------------------------------------

/// The plain editor and its two control sections — unchanged from the page this one grew out of,
/// because `text_area` is still the right control for a chat composer or a commit message.
fn plain_sections() -> (AnyPiece, AnyPiece, AnyPiece) {
    // What the running toolkit can actually honor — an unsupported attribute grays out its toggle.
    // `Emulated` counts as honored: the attribute behaves, it just isn't one native property behind
    // the scenes (XAML has no TextBox selection flag, so it collapses selections as they form).
    let cap_editable = capability(Cap::TextEditable) != Support::Unsupported;
    let cap_selectable = capability(Cap::TextSelectable) != Support::Unsupported;
    let cap_spellcheck = capability(Cap::TextSpellCheck) != Support::Unsupported;

    let content = Signal::new(crate::res::str::textareas_sample_short().format());
    // The three attributes, each bound to a toggle. Live: flipping a toggle patches the editor.
    // Spell-check starts off where the toolkit has none (Qt/GTK), so the disabled toggle reads
    // "off" rather than falsely showing an active checker.
    let editable = Signal::new(true);
    let selectable = Signal::new(true);
    let spellcheck = Signal::new(cap_spellcheck);

    // Editing implies selection — no backend can present editable-but-unselectable text (on Android
    // an editable field is always selectable, so read-only is the only way to stop selection). So
    // turning Selectable off also turns Editable off, and the Editable toggle disables while it is.
    Effect::new(move || {
        if !selectable.get() {
            editable.set(false);
        }
    });

    // A fixed five-line editor: it scrolls internally, so seeding short vs. long text never changes
    // its height.
    let editor = section((text_area(content)
        .editable(editable)
        .selectable(selectable)
        .spellcheck(spellcheck)
        .min_lines(5)
        .max_lines(5)
        .id("textareas-editor"),))
    .title(crate::res::str::textareas_editor_section())
    .any();

    let seed = section((row((
        button(crate::res::str::textareas_seed_short())
            .action(move || content.set(crate::res::str::textareas_sample_short().format()))
            .id("ta-seed-short"),
        button(crate::res::str::textareas_seed_long())
            .action(move || content.set(LONG.into()))
            .id("ta-seed-long"),
        button(crate::res::str::textareas_seed_markdown())
            .action(move || content.set(MARKDOWN.into()))
            .tint(crate::widgets::primary())
            .id("ta-seed-markdown"),
    ))
    .spacing(8.0),))
    .title(crate::res::str::textareas_seed_section())
    .any();

    // Each toggle is disabled where the running toolkit can't honor the attribute (GTK can't stop
    // selection; GTK/Qt/ArkUI have no spell-check) — the `capability()` gating idiom. Editable also
    // disables whenever Selectable is off, since editing without selection is not a valid state.
    let attrs = section((
        labeled(
            crate::res::str::textareas_editable(),
            toggle(editable)
                .enabled(move || cap_editable && selectable.get())
                .id("ta-editable"),
        ),
        labeled(
            crate::res::str::textareas_selectable(),
            toggle(selectable)
                .enabled(cap_selectable)
                .id("ta-selectable"),
        ),
        labeled(
            crate::res::str::textareas_spellcheck(),
            toggle(spellcheck)
                .enabled(cap_spellcheck)
                .id("ta-spellcheck"),
        ),
    ))
    .title(crate::res::str::textareas_attrs_section())
    .any();

    (editor, seed, attrs)
}

// ---------------------------------------------------------------------------
// The styled editor (docs/texteditor.md).
// ---------------------------------------------------------------------------

fn styled_sections() -> AnyPiece {
    let doc = Signal::new(StyledText::markdown(NOTE, Font::Body));
    let sel = Signal::new(0..0);
    let typing = Signal::new(RunStyle::plain(Font::Body));
    let which = Signal::new(0usize);
    // The document the export section last wrote, and which format it used.
    let exported = Signal::new(String::new());

    // Live syntax highlighting: re-tokenize whenever the TEXT changes, and only then. The guard is
    // what makes this terminate — writing runs back into `doc` re-runs this effect, and the second
    // pass sees the same text and stops. Because only the attributes changed, the piece sends an
    // attributes patch rather than a document one, so the caret never moves.
    let last = Signal::new(String::new());
    Effect::new(move || {
        let text = doc.with(|d| d.text.clone());
        if which.get() != 1 || last.get_untracked() == text {
            return;
        }
        last.set(text.clone());
        let runs = highlight_rust(&text);
        doc.update(|d| d.runs = runs);
    });

    let toolbar = section((
        row((
            style_button("B", "ed-bold", doc, sel, typing, |s| {
                s.set_bold(!s.bold());
            }),
            style_button("I", "ed-italic", doc, sel, typing, |s| {
                s.set_italic(!s.italic());
            }),
            style_button("U", "ed-underline", doc, sel, typing, |s| {
                s.underline = s.underline.toggled();
            }),
            style_button("S", "ed-strike", doc, sel, typing, |s| {
                s.strikethrough = !s.strikethrough;
            }),
            style_button("A+", "ed-bigger", doc, sel, typing, |s| {
                s.font.scale = (s.font.scale * 1.25).min(4.0);
            }),
            style_button("A-", "ed-smaller", doc, sel, typing, |s| {
                s.font.scale = (s.font.scale / 1.25).max(0.5);
            }),
            style_button("H", "ed-highlight", doc, sel, typing, |s| {
                s.background = match s.background {
                    Some(_) => None,
                    None => Some(crate::palette::AMBER),
                };
            }),
        ))
        .spacing(6.0)
        .fit(RowFit::Wrap { run_spacing: 6.0 }),
        // The color picker piece, driving the selection's text color. Composed, so the page shows
        // the same panel on every target rather than depending on which of the nine has a native
        // chooser (docs/colorpicker.md).
        labeled(crate::res::str::textareas_text_color(), {
            let pen = Signal::new(crate::palette::VIOLET);
            Effect::new(move || {
                let color = pen.get();
                let range = sel.get_untracked();
                if range.is_empty() {
                    typing.update(|s| s.color = Some(color));
                } else {
                    doc.update(|d| d.apply(range.clone(), Font::Body, |s| s.color = Some(color)));
                }
            });
            color_picker(pen)
                .composed()
                .title(crate::res::str::textareas_text_color())
                .key("ed-color")
        }),
        labeled(
            crate::res::str::textareas_align(),
            picker(
                [
                    crate::res::str::textareas_align_natural().format(),
                    crate::res::str::textareas_align_center().format(),
                    crate::res::str::textareas_align_trailing().format(),
                ],
                {
                    let align = Signal::new(0usize);
                    Effect::new(move || {
                        let pick = align.get();
                        let range = sel.get_untracked();
                        doc.update(|d| {
                            d.apply_paragraph(range.clone(), |p| {
                                p.align = match pick {
                                    1 => ParagraphAlign::Center,
                                    2 => ParagraphAlign::Trailing,
                                    _ => ParagraphAlign::Natural,
                                };
                            })
                        });
                    });
                    align
                },
            )
            .segmented()
            .id("ed-align"),
        ),
    ))
    .title(crate::res::str::textareas_toolbar_section());

    let editor = section((
        text_editor(doc)
            .selection(sel)
            .typing_style(typing)
            .placeholder(crate::res::str::textareas_placeholder())
            // Prose wants the squiggles; the code sample does not.
            .spellcheck(true)
            .min_lines(8)
            .max_lines(14)
            .id("styled-editor"),
        // The selection inspector: what the app knows about the caret, with no round trip into
        // the toolkit — `style_of` is a pure function over the document.
        label(move || {
            let range = sel.get();
            let style = doc.with(|d| d.style_of(range.clone(), Font::Body));
            // Translated, not hard-coded: this line is the page's own copy, so it goes through
            // the catalog like every other string (docs/localization.md).
            let mut marks = Vec::new();
            if style.bold() {
                marks.push(crate::res::str::textareas_mark_bold().format());
            }
            if style.italic() {
                marks.push(crate::res::str::textareas_mark_italic().format());
            }
            if style.underline.is_on() {
                marks.push(crate::res::str::textareas_mark_underline().format());
            }
            if style.strikethrough {
                marks.push(crate::res::str::textareas_mark_strike().format());
            }
            let marks = if marks.is_empty() {
                crate::res::str::textareas_mark_plain().format()
            } else {
                marks.join(" + ")
            };
            // The generated accessor takes a message's arguments in NAME order, not in the order
            // they appear in the text — so this reads `end, marks, scale, start`.
            crate::res::str::textareas_inspector(
                range.end as i64,
                marks,
                format!("{:.2}", style.font.scale),
                range.start as i64,
            )
            .format()
        })
        .font(Font::Footnote)
        .id("ed-inspector"),
        // What the selection actually covers. Together with the button beside it this is the
        // regression guard for a class of bug the web arm had: a restyle that rebuilds the view
        // has to put the selection back on the SAME characters, and this line says which ones.
        row((
            button(crate::res::str::textareas_select_word())
                .action(move || {
                    let range = doc.with_untracked(|d| first_word(&d.text));
                    sel.set(range);
                })
                .id("ed-select-word"),
            label(move || {
                let range = sel.get();
                doc.with(|d| {
                    d.text
                        .get(range)
                        .filter(|s| !s.is_empty())
                        .map(str::to_string)
                        .unwrap_or_else(|| crate::res::str::textareas_no_selection().format())
                })
            })
            .font(Font::Footnote)
            .id("ed-selection-text"),
        ))
        .spacing(8.0)
        .align(VAlign::Center),
        row((picker(
            [
                crate::res::str::textareas_doc_note().format(),
                crate::res::str::textareas_doc_code().format(),
                crate::res::str::textareas_doc_empty().format(),
            ],
            which,
        )
        .segmented()
        .id("ed-document"),)),
    ))
    .title(crate::res::str::textareas_styled_section());

    // Swapping the document. The code sample arrives unstyled and the highlighter dresses it on
    // the next pass, which is exactly what happens when a file is opened.
    Effect::new(move || {
        let pick = match which.get() {
            1 => Doc::Code,
            2 => Doc::Empty,
            _ => Doc::Note,
        };
        last.set(String::new());
        doc.set(match pick {
            Doc::Note => StyledText::markdown(NOTE, Font::Body),
            Doc::Code => StyledText::plain(CODE),
            Doc::Empty => StyledText::default(),
        });
    });

    let export = section((
        row((
            button(crate::res::str::textareas_export_markdown())
                .action(move || exported.set(doc.with_untracked(|d| d.to_markdown(Font::Body))))
                .id("ed-export-md"),
            button(crate::res::str::textareas_export_html())
                .action(move || exported.set(doc.with_untracked(|d| d.to_html(Font::Body))))
                .id("ed-export-html"),
            button(crate::res::str::textareas_export_rtf())
                .action(move || exported.set(doc.with_untracked(|d| d.to_rtf(Font::Body))))
                .id("ed-export-rtf"),
            // The round trip: read the exported text back in as the document. Lossy by design —
            // see docs/texteditor.md for what each format cannot carry.
            button(crate::res::str::textareas_import())
                .action(move || {
                    let text = exported.get_untracked();
                    if text.is_empty() {
                        return;
                    }
                    last.set(String::new());
                    doc.set(if text.starts_with("{\\rtf") {
                        StyledText::rtf(&text, Font::Body)
                    } else if text.contains("<p") || text.contains("<span") {
                        StyledText::html(&text, Font::Body)
                    } else {
                        StyledText::markdown(&text, Font::Body)
                    });
                })
                .tint(crate::widgets::primary())
                .id("ed-import"),
        ))
        .spacing(8.0)
        .fit(RowFit::Wrap { run_spacing: 8.0 }),
        text_area(exported)
            .editable(false)
            .min_lines(4)
            .max_lines(8)
            .id("ed-export-out"),
    ))
    .title(crate::res::str::textareas_export_section());

    column((toolbar, editor, export)).any()
}

/// The byte range of the document's first word — what the "Select a word" button selects. Any
/// document has one, in any script, which is what makes the walkthrough's assertion the same on
/// every backend.
fn first_word(text: &str) -> std::ops::Range<usize> {
    let start = text
        .char_indices()
        .find(|(_, c)| c.is_alphanumeric())
        .map(|(i, _)| i)
        .unwrap_or(0);
    let end = text[start..]
        .char_indices()
        .find(|(_, c)| !c.is_alphanumeric())
        .map(|(i, _)| start + i)
        .unwrap_or(text.len());
    start..end
}

/// One toolbar button: apply `f` to the selection's style, or to what the next character will take
/// when the caret is collapsed.
///
/// This is the whole toolbar contract, and none of it reaches the toolkit: the document and the
/// selection are Day's, `style_of` reads the state a button renders, and `apply` writes it back.
fn style_button(
    title: &'static str,
    id: &'static str,
    doc: Signal<StyledText>,
    sel: Signal<std::ops::Range<usize>>,
    typing: Signal<RunStyle>,
    f: impl Fn(&mut RunStyle) + Copy + 'static,
) -> AnyPiece {
    button(title)
        .action(move || {
            let range = sel.get_untracked();
            if range.is_empty() {
                // No selection: the change belongs to the NEXT keystroke.
                typing.update(|s| f(s));
            } else {
                doc.update(|d| d.apply(range.clone(), Font::Body, f));
            }
        })
        .id(id)
        .any()
}

// ---------------------------------------------------------------------------
// A toy Rust highlighter — enough to show live re-styling, not a parser.
// ---------------------------------------------------------------------------

const KEYWORDS: &[&str] = &[
    "fn", "let", "move", "mut", "pub", "struct", "enum", "impl", "match", "if", "else", "for",
    "while", "loop", "return", "use", "crate", "self", "true", "false",
];

/// Tokenize `src` into runs. Byte offsets throughout, which is what the document indexes by — and
/// why a multi-byte character in a comment cannot shift the styling of the code after it.
fn highlight_rust(src: &str) -> Vec<TextRun> {
    let mut runs: Vec<TextRun> = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0usize;
    // Everything the tokenizer does not claim still has to be a run, because a code sample is
    // monospaced end to end — the gaps between tokens are the punctuation and the identifiers.
    let mut plain_from = 0usize;
    let close_gap = |runs: &mut Vec<TextRun>, upto: usize, from: &mut usize| {
        if *from < upto {
            let mut run = colored(*from..upto, crate::palette::INK, false);
            run.color = None; // the platform's own label color, whatever the appearance is
            runs.push(run);
        }
        *from = upto;
    };
    while i < bytes.len() {
        let rest = &src[i..];
        // A line comment runs to the newline.
        if rest.starts_with("//") {
            let end = rest.find('\n').map(|n| i + n).unwrap_or(src.len());
            close_gap(&mut runs, i, &mut plain_from);
            runs.push(colored(i..end, crate::palette::SLATE, true));
            plain_from = end;
            i = end;
            continue;
        }
        // A string literal, with escapes.
        if bytes[i] == b'"' {
            let mut j = i + 1;
            while j < bytes.len() {
                if bytes[j] == b'\\' {
                    j += 2;
                    continue;
                }
                if bytes[j] == b'"' {
                    j += 1;
                    break;
                }
                j += 1;
            }
            let end = j.min(src.len());
            close_gap(&mut runs, i, &mut plain_from);
            runs.push(colored(i..end, crate::palette::TEAL, false));
            plain_from = end;
            i = end;
            continue;
        }
        // A word: a keyword, a number, or neither.
        let ch = src[i..].chars().next().unwrap_or(' ');
        if ch.is_alphanumeric() || ch == '_' {
            let end = i + src[i..]
                .find(|c: char| !(c.is_alphanumeric() || c == '_'))
                .unwrap_or(src.len() - i);
            let word = &src[i..end];
            if KEYWORDS.contains(&word) {
                close_gap(&mut runs, i, &mut plain_from);
                let mut run = colored(i..end, crate::palette::VIOLET, false);
                run.font.weight = Some(FontWeight::Bold);
                runs.push(run);
                plain_from = end;
            } else if word.chars().all(|c| c.is_ascii_digit()) {
                close_gap(&mut runs, i, &mut plain_from);
                runs.push(colored(i..end, crate::palette::CORAL, false));
                plain_from = end;
            }
            i = end;
            continue;
        }
        i += ch.len_utf8();
    }
    close_gap(&mut runs, src.len(), &mut plain_from);
    runs
}

/// A monospaced run in one color — every token this highlighter emits, since a code sample is
/// monospaced from end to end.
fn colored(range: std::ops::Range<usize>, color: Color, italic: bool) -> TextRun {
    let mut style = RunStyle::plain(Font::Body);
    style.font.monospace = true;
    style.font.italic = italic;
    style.color = Some(color);
    TextRun::styled(range, style)
}
