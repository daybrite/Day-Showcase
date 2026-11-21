# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Text page's newer cards (src/pages/text.rs). Its other strings are in app.ftl.

# Styled runs: the names of the styles, each drawn in its style.
text_run_code = code
text_run_color = color
text_run_highlight = highlight
text_run_larger = larger
text_underline_single = single
text_underline_double = double
text_underline_dotted = dotted
text_underline_wavy = wavy

# Relative size: a semantic style and the factor it is scaled by.
text_scale_header = Relative size
text_scale_of = { $style } × { $scale }

# Presets: two readings that share three named styles.
text_preset_header = Presets
text_preset_moon = Earth to Moon
text_preset_moon_value = 384,400
text_preset_km = km
text_preset_sun = Earth to Sun
text_preset_sun_value = 149.6
text_preset_million_km = million km

# Line limits and alignment share one paragraph.
text_sample_paragraph = Sunlight takes about eight minutes to reach Earth, more than four hours to reach Neptune, and over four years to reach the nearest star.
text_lines_header = Line limits
text_lines_one = One line
text_lines_two = Two lines
text_lines_all = No limit
text_align_header = Alignment
text_align_leading = Leading

# Scripts and direction: the names of the specimens.
text_scripts_header = Scripts and direction
text_scripts_cjk = Chinese, Japanese and Korean
text_scripts_rtl_in_ltr = Arabic and Hebrew inside Latin
text_scripts_ltr_in_rtl = Latin inside Arabic

# Decorations and font features.
text_decorations_header = Decorations and figures
text_small_caps = Small caps
text_small_caps_sample = Mission Control, Houston
text_figures = Figures

# The reader's text size, and a badge that keeps its size.
text_scale_reading = Text size: { $scale }×
text_scale_fixed = Fixed size

# Line limits: how many lines, and where the ellipsis goes.
text_lines_count = Lines
text_lines_truncate = Ellipsis
text_trunc_head = Start
text_trunc_middle = Middle
text_trunc_tail = End

# Letter spacing and line height.
text_spacing_header = Spacing
text_spacing_letters = Letters
text_spacing_lines = Lines
text_align_justified = Justified

# A row that reads in the direction the picker names.
text_direction = Direction
text_direction_ltr = Left to right
text_direction_rtl = Right to left
text_direction_first = 1 Launch
text_direction_second = 2 Orbit
text_direction_third = 3 Land

# Links and selection.
text_link_handled_label = Handled by the app
text_selection_none = Select some of the text above.
text_selection_value = Selected: { $text }

# The Markdown view: a reply streamed in word by word.
text_md_view_header = Markdown view
text_md_stream = Stream a reply
text_md_copy = Copy
text_md_copied = Code copied
text_md_reply =
    ## Orbit check

    The probe sends **three** readings on every pass:

    - altitude
    - speed
    - battery

    Its period follows from the orbit's size:

    ```rust
    let period = 2.0 * PI * (a.powi(3) / mu).sqrt();
    ```

    Plan the next burn when the readings settle.
