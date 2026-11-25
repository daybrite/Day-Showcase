app_title = Day Showcase
counter_value = { $count ->
    [one] { $count } click
   *[other] { $count } clicks
}
decrement = −
increment = +
name_placeholder = Your name
flavor_placeholder = Type or pick a flavor
flavor_add = Add
nav_controls = Controls
ctl_buttons = Buttons
ctl_press = Press
ctl_plain = Plain
ctl_bordered = Bordered
ctl_prominent = Prominent
ctl_tinted = Tinted
ctl_destructive = Destructive
ctl_disabled = Disabled
ctl_link = Link
ctl_composed = Composed
ctl_link_text = daybrite.dev
ctl_presses = Presses
ctl_switches = Switches & sliders
ctl_toggle = Toggle
ctl_slider = Slider
ctl_stepped = Stepped slider
ctl_stepper = Stepper
ctl_composed_stepper = Composed stepper
ctl_progress = Progress
ctl_activity = Activity
ctl_spinner = Spinner
ctl_accessibility = Accessibility
ctl_a11y_heading = A heading, for screen readers
ctl_announce = Announce
ctl_announce_hint = Speaks the count through the screen reader
ctl_announced_times = Announced { $count } times
ctl_announce_emulated = Emulated (a live region)
ctl_text_entry = Text entry
ctl_text_field = Text field
ctl_search_field = Search field
ctl_combo_box = Combo box
ctl_combo_note = This toolkit has no combo box control, so Day shows a placeholder here.
ctl_text_area = Text area
ctl_notes_placeholder = A few lines of notes
ctl_pickers = Pickers
ctl_date = Date
ctl_time = Time
ctl_color = Color
ctl_indicators = Indicators & composition
ctl_badge = Badge
ctl_gauge = Gauge
ctl_divider = Divider
ctl_state = State
ctl_on = On
ctl_off = Off
nav_menus = Menus & dialogs
nav_text = Text
nav_battery = Battery
nav_sensors = Sensors
nav_clipboard = Clipboard
nav_media = Media
nav_compose = Cards & environment
nav_files = Files
nav_tabs = Tabs
nav_stack = Stack
nav_layout = Layout
layout_stacks_section = Rows, columns & layers
layout_align_label = Alignment
layout_align_top = Top
layout_align_center = Center
layout_align_bottom = Bottom
layout_layered = Layered
layout_scroll_section = Scrolling
layout_scroll_top = Scroll to top
layout_scroll_end = Scroll to end
layout_size_section = Size classes
layout_size_width = Width
layout_size_height = Height
nav_list = List
refresh_tier_native = Pull-to-refresh: native
refresh_tier_emulated = Pull-to-refresh: emulated
layout_row = Item { $n }
nav_webview = Web View
nav_lottie = Lottie
nav_about = About

# Sidebar group headers (lib.rs `Group`): the sections the destinations sit under.
group_starred = Starred
group_overview = Overview
group_controls = Controls
group_layout = Layout
group_navigation = Navigation & chrome
group_data = Data
group_graphics = Graphics & media
group_platform = Platform
group_app = App & tooling

picker_selected = Selected
picker_segmented = Segmented
picker_menu = Menu
picker_inline = Inline

# --- day::datetime ---
nav_dates = Date & time
dates_date_section = Date
dates_time_section = Time
dates_composed_section = Composed
date_compact = Compact
time_compact = Compact
time_seconds = With seconds
dates_composed = Date & time
date_bounded = Within 2026
date_picked = Picked date
time_picked = Picked time

compose_card_title = Reusable surface
compose_card_body = Padding, a background and rounded corners.
compose_env_value = Tinted by the provided accent
list_add = Add 100
list_caption = { $count } rows
list_selection_none = No rows selected
list_selection = { $count ->
    [one] Selected row: { $rows }
   *[other] Selected rows: { $rows }
}
list_clear_selection = Clear Selection
list_reorder_hint = Drag rows to reorder, swipe to delete; the first row is pinned
list_order = First rows: { $rows }
list_shuffle = Shuffle
list_reset = Reset
list_delete = Delete
list_options = Options…
list_options_title = List Options
list_options_layout = Layout
list_options_behavior = Behavior
list_options_gestures = Gestures
list_opt_sections = Section headers
list_opt_row_height = Row height
list_opt_uniform = Uniform
list_opt_automatic = Automatic
list_opt_separators = Separators
list_opt_default = Default
list_opt_on = On
list_opt_off = Off
list_opt_content = Row content
list_opt_plain = Label
list_opt_item = List item
list_opt_multi = Multiple selection
list_opt_pull = Pull to refresh
list_opt_editing = Editing mode
list_opt_reorder = Drag to reorder
list_opt_swipe = Swipe actions
list_opt_delete = Swipe to delete
list_opt_gestures_footer = With section headers, rows drag within their section and swipe to delete is off.
list_section = Rows { $first }–{ $last }
list_row_note = A second line of detail for this row
list_row_note_long = Rows sized by their content grow to fit longer text, like this third line, which wraps in a narrow window.
list_swipe_select = Select
list_swipe_deselect = Deselect
list_align = Jumps land
list_align_nearest = Nearest
list_align_start = Top
list_align_center = Middle
list_align_end = Bottom
list_insert = Insert
list_remove = Remove
list_remove_all = Remove all
list_footer = End of the list: { $count } rows
list_empty = No rows. Reset brings them back.
list_visible = In view: { $first }–{ $last }
list_visible_first = In view from { $first }
list_visible_none = No rows in view

nav_model = Model
model_caption = { $count } rows shown
model_cost = { $paths } observed paths · { $nodes } interned nodes
model_hide_done = Hide done
model_selected = Selected: { $title }
model_selected_none = Nothing selected
model_add = Add Task
model_delete = Delete Selected
model_undo = Undo
model_redo = Redo
model_storage_file = Stored in { $file }
model_storage_memory = Stored in memory on this platform
nav_query = Query
query_caption = { $count } of { $total } tracks match
query_resident = { $resident } of { $total } rows cached by the database worker
query_starred = Starred only
query_selected = Selected: { $title }
query_selected_none = Nothing selected
query_fts = Full-text (FTS5)
query_viewport = Viewport only
query_viewport_box = lat { $from }–{ $to }
query_star = Toggle Star

webview_url_hint = Enter a URL
webview_go = Go
webview_back = Back
webview_forward = Forward
webview_stop = Stop
webview_reload = Reload
webview_js_hint = JavaScript to run in the page
webview_js_run = Run JS »
webview_js_result_hint = The JSON result appears here
webview_note_iframe = This platform embeds the page in an iframe, so Back, Forward and Stop are unavailable and the field does not follow links.
webview_tab_remote = Remote
webview_tab_embedded = Embedded
webview_embedded_unsupported = No web engine is available in this toolkit build.
webview_embedded_status_none = No external link followed yet.
webview_embedded_opened = Opened outside the app: { $url }
webview_embedded_intercepted = Intercepted — navigated in-app to "{ $route }".

stack_push = Push a detail
stack_detail_title = Level { $depth }
stack_item_title = Item { $id }
stack_link_42 = Open item 42 by route
stack_param_hint = Opened with hint: {$hint}
tab_one = Overview
tab_two = Details
tab_three = Settings
tab_state_note = Switch tabs and come back: the count is still here.
tab_detail_route = Route key
tab_detail_link = Deep link
tab_set_badges = Show badges
tab_set_sounds = Play sounds
about_text = A native cross-platform app built with day.
modal_alert = Show alert
modal_confirm = Confirm
modal_delete = Delete…
modal_sheet = Pick flavor
modal_prompt = Enter name
alert_title = Notice
alert_body = Your changes have been saved.
ok = OK
confirm_title = Quit?
confirm_body = Are you sure you want to quit?
delete_title = Delete item?
delete_body = This cannot be undone.
delete = Delete
flavor_title = Choose a flavor
cancel = Cancel
vanilla = vanilla
pistachio = pistachio

# Files playground (docs/files.md)
files_placeholder = Type something to save…
files_open = Open File…
files_save = Save File…
files_opened = Opened { $name }

# Battery playground (docs/battery.md)
battery_refresh = Read Device Battery
battery_level = Level
battery_charging = Charging
battery_reading = Battery: { $percent } · { $state }
battery_reading_none = Battery: no battery API on this platform
battery_reading_absent = Battery: this device has no battery

# Sensors playground (docs/sensors.md)
sensor_accelerometer = Accelerometer
sensor_gyroscope = Gyroscope
sensor_magnetometer = Magnetometer
sensor_reading = x { $x } · y { $y } · z { $z } { $unit }
sensor_waiting = waiting for first sample…
sensor_unavailable = unavailable on this device

# Clipboard playground (docs/clipboard.md)
clipboard_placeholder = Type something to copy
clipboard_copy = Copy
clipboard_paste = Paste
clipboard_idle = Clipboard untouched
clipboard_copied = Copied to the system clipboard
clipboard_copy_failed = Copy failed (no clipboard API here)
clipboard_pasted = Pasted from the system clipboard
clipboard_empty = Clipboard is empty or unreadable

# Network playground (docs/network.md)
network_refresh = Read Network
network_reading_online = Online · { $kind } · metered: { $expensive }
network_reading_offline = Offline
network_reading_none = Connectivity unavailable

# Media playground (docs/media.md)
media_play = Play
media_pause = Pause
media_load = Load

# --- Localization page (docs/localization.md: live locale, NUMBER/DATETIME, plurals,
#     collation) ---
nav_localization = Localization
loc_locale_section = Live locale
loc_live_note = Layout direction is fixed at launch.
loc_current_label = Current
loc_reset = Reset
loc_numbers_section = Numbers
loc_dates_section = Dates & times
loc_plurals_section = Plurals
loc_sorting_section = Sorting
fmt_number_label = Grouped
fmt_fraction_label = Two decimals
fmt_percent_label = Percent
fmt_date_label = Long date
fmt_time_label = Time
fmt_datetime_label = Date & time
fmt_sorted_label = Sorted
fmt_number = { NUMBER($n) }
fmt_fraction = { NUMBER($n, minimumFractionDigits: 2) }
fmt_percent = { NUMBER($p, style: "percent") }
fmt_date = { DATETIME($d, dateStyle: "long") }
fmt_time = { DATETIME($t, timeStyle: "short") }
fmt_datetime = { DATETIME($dt, dateStyle: "medium", timeStyle: "short") }
plural_items = { $count ->
    [0] Nothing yet
    [one] One item
   *[other] { $count } items
}

# Text playground (typography)
text_selectable_toggle = Selectable
text_styles_header = Styles
text_weights_header = Weights
text_styling_header = Bold, italic and color
text_custom_header = Custom sizes
text_fonts_header = Bundled fonts

# Menus playground
menus_last = Last action
menus_lifecycle = Lifecycle
menus_target = Right-click or long-press here

# --- day-part-haptics ---
nav_haptics = Haptics
haptics_supported_yes = Haptic engine available
haptics_supported_no = No haptic engine on this platform
haptics_light = Light
haptics_medium = Medium
haptics_heavy = Heavy
haptics_success = Success
haptics_warning = Warning
haptics_error = Error
haptics_selection = Selection
haptics_last = Last played
haptics_none = Nothing played yet
haptics_last_played = Played: { $style }

# --- day-part-sound ---
nav_sound = Sound effects
sound_click = Click
sound_confirm = Confirm
sound_error = Error
sound_glass = Glass
sound_card = Card
sound_jingle = Jingle
sound_pan_caption = Stereo pan
sound_left = Left
sound_center = Center
sound_right = Right
sound_volume = Volume
sound_last = Last played
sound_none = Nothing played yet
sound_last_played = Played: { $clip }
sound_loop = Loop a hum
sound_loop_volume = Loop volume
sound_looping = Looping
sound_looping_none = Nothing looping
sound_looping_clip = Looping: { $clip }

# --- day-part-preferences ---
nav_prefs = Preferences
prefs_placeholder = A note to remember
prefs_empty = (nothing stored)
prefs_value_label = Stored value

# --- bundled resources (§18.3) ---
nav_resources = Resources
vectors_title = Vectors
vectors_tints = Tints
vectors_zoom = Zoom
vectors_weights = Weights
vectors_sizes = Sizes
resources_numbers = numbers.bin: { $len } bytes, byte[100] = { $byte }
resources_greeting = greeting.txt: { $text }

# --- day-part-device ---
nav_deviceinfo = Device Info
deviceinfo_model = Model: {$value}
deviceinfo_system = System: {$name} {$version}
deviceinfo_simulator = Simulator: {$value}
deviceinfo_yes = yes
deviceinfo_no = no
deviceinfo_refresh = Refresh

# --- Day core spinner ---
activity_on = Spinning
activity_off = Stopped

# --- day::searchfield ---
search_clear = Clear


# --- tweaks page (docs/tweaks.md) ---
nav_tweaks = Tweaks
tweaks_stock = Stock
tweaks_tweaked = Tweaked
tweaks_bezel_title = Button bezel
tweaks_bezel_caption = AppKit only
tweaks_selectable_title = Selectable label
tweaks_selectable_text = Select this text and copy it.
nav_feedback = Tooltips, popovers & toasts
detail_feedback = Try hints, anchored panels and temporary messages.
feedback_tooltip = Tooltip
feedback_popover = Popover
feedback_toast = Toast
feedback_length = Text length
feedback_short = Short
feedback_medium = Medium
feedback_long = Long
feedback_tooltip_help = Hover over Draft or touch and hold it until the hint appears.
feedback_draft = Draft
feedback_tooltip_short = Only you can see this draft.
feedback_tooltip_medium = Only you can see this draft. Share it when you’re ready for someone else to read it.
feedback_tooltip_long = Only you can see this draft. When you’re ready for a second opinion, invite a reader. You can keep editing while they review your work.
feedback_popover_help = Choose a text length, then select Open popover. Close it or tap outside, change the settings and open it again to compare.
feedback_narrow = Narrow panel
feedback_open = Open popover
feedback_close = Close
feedback_popover_short = Your draft is private.
feedback_popover_medium = Your draft is private. Invite a reader when you’re ready for feedback.
feedback_popover_long = Your draft is private. When you invite a reader, they can leave comments alongside the text. You can keep editing as you read their suggestions. Invite another reader at any time to get a different perspective.
feedback_toast_help = Select Show toast to see a message that disappears on its own. Try another length to compare how much fits.
feedback_show = Show toast
feedback_toast_short = Draft saved.
feedback_toast_medium = Draft saved. You can close the editor.
feedback_toast_long = Draft saved. Your latest changes will be ready when you open the editor again.
tweaks_ticks_title = Slider tick marks
tweaks_ticks_caption = AppKit, GTK, Android, Qt, XAML and ArkUI. The tweaked slider snaps to its marks.
tweaks_ref_title = NativeRef liveness
tweaks_ref_caption = Mount the slider
tweaks_ref_live = ref: live
tweaks_ref_cleared = ref: cleared
tweaks_label_title = Tweak a text label
tweaks_label_caption = AppKit and UIKit. On iOS, Selectable swaps the native class.
tweaks_label_sample = This label is dimmed by a native tweak.
tweaks_label_class = Native class: { $class }

# --- merged section pages (design overhaul) ---
nav_canvas = Canvas
nav_charts = Charts
nav_system = Device & sensors
nav_network_http = Network & HTTP
network_status_section = Connectivity
nav_notify_badge = Notifications & badge
nav_push = Push notifications
nav_speech_haptics = Speech, sound & haptics
nav_files_storage = Files & storage
gauge_value_label = Value
canvas_text_title = Text & fonts
canvas_anchors_title = Text anchors
canvas_interaction_title = Tap, drag & hover
canvas_interaction_label = Reported
canvas_interaction_none = —
canvas_interaction_readout = hover { $hover } · tap { $tap } · drag { $trail } points
canvas_metrics_cache = measure_text: { $hits } cached · { $misses } measured · { $entries } held
canvas_font_label = Font
canvas_font_default = System
canvas_text_specimen = The quick brown fox jumps over the lazy dog
canvas_text_bold = Bold
canvas_text_italic = Italic
canvas_text_bold_italic = Bold Italic
canvas_image_title = Image from bytes
canvas_image_loading = Decoding…
canvas_image_failed = Those bytes are not an image this platform decodes
canvas_image_readout = { $format } · { $width } × { $height } px

# --- data strings localized for the walkthrough locales (option lists, specimen rows) ---
chocolate = chocolate
size_small = Small
size_medium = Medium
size_large = Large
fruit_apple = Apple
fruit_banana = Banana
fruit_cherry = Cherry
fruit_date = Date
fruit_elderberry = Elderberry
list_row = Row { $n }
text_style_large_title = Large Title
text_style_title = Title
text_style_title2 = Title 2
text_style_title3 = Title 3
text_style_headline = Headline
text_style_subheadline = Subheadline
text_style_body = Body
text_style_callout = Callout
text_style_footnote = Footnote
text_style_caption = Caption
text_style_caption2 = Caption 2
text_weight_ultralight = Ultra Light
text_weight_light = Light
text_weight_regular = Regular
text_weight_medium = Medium
text_weight_semibold = Semibold
text_weight_bold = Bold
text_weight_heavy = Heavy
text_weight_black = Black
text_bold = Bold text
text_italic = Italic text
text_bolditalic = Bold italic
text_emphasis_label = Emphasis
color_red = Red
color_green = Green
color_blue = Blue
color_orange = Orange

# Menus & dialogs (merged page)
menus_appmenu_section = App menu
menus_context_section = Context menu
menus_dialogs_section = Dialogs
modal_result_label = Result

# Media page
media_player_section = Video

# Resources page sections
resources_image_section = Bundled image
image_mode_fit = Fit
image_mode_fill = Fill
image_mode_stretch = Stretch
resources_data_section = Data assets

# Settings (docs/settings.md): the Catalog and Diagnostics groups beside Day's General group.
# The About's labels (Version, Website) and the General group's come from Day's own catalog.
settings_catalog = Catalog
settings_catalog_subtitle = Starred pages
settings_starred_count = Starred pages
settings_clear_starred = Clear Starred
settings_clear_starred_confirm = Remove the star from every page?
settings_diagnostics = Diagnostics
settings_diagnostics_subtitle = Toolkit, OS, locale
about_toolkit = Toolkit
about_id = App ID
about_os = OS
about_model = Model
about_locale = Locale

# Focus page (docs/focus.md)
nav_focus = Focus
focus_group_section = One signal, one form
focus_name_label = Name
focus_email_label = Email
focus_city_label = City
focus_current_label = Focused
focus_next = Focus next
focus_clear = Clear focus
focus_bool_section = One control, one Bool
focus_bool_placeholder = Focus lands here
focus_focus_btn = Focus
focus_blur_btn = Blur
focus_state_label = State
focus_state_on = focused
focus_state_off = blurred
focus_probe_section = Beyond text fields
focus_probe_caption = Touch platforms keep focus for text input.
focus_probe_toggle = Toggle
focus_probe_slider = Slider
focus_probe_button = Button

# HTTP fetch demo (docs/http.md): the status readout stays raw "<status> <body>" so the
# walkthrough asserts it byte-for-byte in every locale.
http_idle = Nothing fetched yet
http_checking = Checking…
ws_title = WebSocket
ws_connect = Connect
ws_send = Send
ws_close = Close

# Scrolling page (docs/scroll.md): programmatic scroll targets.
scroll_to_top = Scroll to top
scroll_to_bottom = Scroll to bottom
scroll_to_item = Scroll to item 100

# Grid page (docs/grid.md): grid/grid_row from basics to a stress test.
nav_grid = Grid
grid_tab_basics = Basics
grid_tab_sizing = Sizing
grid_tab_spanning = Spanning
grid_tab_composite = Composite
grid_tab_stress = Stress
grid_col_name = Name
grid_col_wins = Wins
grid_col_points = Points
grid_sizing_fixed = Fixed 80 pt
grid_sizing_content = Content
grid_sizing_short = Short
grid_sizing_longer = A longer content cell
grid_month_title = Week planner
grid_event_focus = Focus block
grid_event_review = Review
grid_day_n = Day { $n }
grid_stress_add = Add 50 rows
grid_stress_bump = Bump the first cell

nav_animation = Animation
nav_physics = Physics

# Animation page (localized labels; the ! is part of the button voice)
anim_scale = Scale
anim_rotation = Rotation
anim_opacity = Opacity
anim_offset_x = Offset X
anim_offset_y = Offset Y
anim_hue = Hue
anim_curve = Curve
anim_duration = Duration
anim_randomize = Randomize!
anim_go_label = Animate!
anim_reset_label = Reset
anim_curve_spring = Spring
anim_curve_ease_in_out = Ease-in-out
anim_curve_ease_out = Ease-out
anim_curve_linear = Linear
anim_duration_ms = { $ms } ms
anim_reduced_motion = Reduced motion

# --- Benchmark page (the Day-Bench Grids benchmark; on the Apple-native backends a segmented
#     picker also hosts its hand-written SwiftUI twin via day-piece-swiftui, docs/swiftui.md) ---
nav_benchmark = Benchmark
bench_parameters = Parameters
bench_seed = Random Seed
bench_count = Total Count
bench_rows = { $rows } { $rows ->
    [one] row
   *[other] rows
}
# %d templates for the hosted SwiftUI pane, whose row count lives in Swift @State (one/other
# only, the same fidelity for every locale, since printf templates cannot carry Fluent's plural
# rules).
bench_rows_one = %d row
bench_rows_other = %d rows
bench_tab_day = Day Native
bench_tab_swiftui = SwiftUI Grid

# Menu bar + context menu items (menus page)
menu_file = File
menu_open = Open…
menu_open_recent = Open Recent
menu_clear_menu = Clear Menu
menu_save = Save
menu_save_as = Save As…
menu_edit = Edit
menu_view = View
menu_reload = Reload
menu_actual_size = Actual Size
menu_context = Context
menu_rename = Rename
menu_duplicate = Duplicate
menu_move_to = Move To
menu_inbox = Inbox
menu_archive = Archive

# Text page: sizes, bundled-font descriptions (family names stay Latin), links
text_size_pt = { $pt } pt
text_font_pacifico = Pacifico — flowing script
text_font_bungee = BUNGEE — chromatic display
text_font_specialelite = Special Elite — typewriter keys
text_font_pacifico_lg = Pacifico at 36 points
text_links_section = Links
text_runs_section = Styled runs
text_markdown_section = Markdown, parsed live
text_markdown_sample = Day parses **bold**, *italic*, `code`, ~~strikethrough~~ and [links](https://daybrite.dev) at run time. Unfinished markup like ** stays literal.
text_markdown_opened = Opened: {$url}
text_baseline_section = Baseline alignment
text_baseline_toggle = Align baselines
text_baseline_quantity = Quantity
text_baseline_unit = items
text_baseline_total = Total
text_baseline_currency = USD
text_baseline_due = Due
text_link_icons_label = Material Symbols on Google Fonts
text_link_mail_label = Email the team

# Files section: the editor's seed text
files_initial_content =
    Hello from Day!
    Edit me, then Save.

# Crash Reporting page (day-piece-crash-reporter, docs/break.md)
nav_crash = Crash Reporting
crash_trigger_section = Trigger a crash
crash_report_section = Last crash report
crash_abort = Crash (abort)
crash_abort_label = Native abort → SIGABRT
crash_segv = Crash (segfault)
crash_segv_label = Null dereference → SIGSEGV
crash_contained = Panic (contained)
crash_contained_label = Caught by day, app survives
crash_send = Send report
crash_clear = Clear reports
crash_empty = No crash report yet. Trigger a crash, then relaunch.

# Text Fields page (docs/textfield.md)
nav_textfields = Text fields
textfields_password_section = Password
textfields_password = Password
textfields_password_placeholder = Enter a password
textfields_show_password = Show Password
textfields_length = Length
textfields_purpose_section = Keyboard and autofill
textfields_email = Email
textfields_phone = Phone
textfields_url = Website
textfields_number = Quantity
textfields_decimal = Amount
textfields_code = One-time code
textfields_pin_section = PIN
textfields_pin = PIN
textfields_attrs_section = Attributes
textfields_reference = Reference
textfields_read_only = Read-only
textfields_limited = Up to 12 characters
textfields_message = Message
textfields_message_placeholder = Type and press Send
textfields_sent = Sent

# Text Areas page
nav_textareas = Text editing
textareas_editor_section = Editor
textareas_seed_section = Seed with
textareas_attrs_section = Attributes
textareas_seed_short = Short
textareas_seed_long = Long
textareas_seed_markdown = Markdown
textareas_editable = Editable
textareas_selectable = Selectable
textareas_spellcheck = Spell-check
textareas_sample_short = A short note. Edit it, or seed longer or structured text with the buttons below.

# Styled text editor (docs/texteditor.md)
textareas_styled_section = Styled editor
textareas_toolbar_section = Formatting
textareas_export_section = Import and export
textareas_text_color = Text color
textareas_align = Alignment
textareas_align_natural = Natural
textareas_align_center = Center
textareas_align_trailing = Trailing
textareas_doc_note = Note
textareas_doc_code = Code
textareas_doc_empty = Empty
textareas_placeholder = Start typing, or pick a document above.
textareas_inspector = Selection { $start }–{ $end } · { $marks } · size ×{ $scale }
textareas_export_markdown = Markdown
textareas_export_html = HTML
textareas_export_rtf = RTF
textareas_import = Read back
textareas_select_word = Select a word
textareas_no_selection = nothing selected
textareas_mark_bold = bold
textareas_mark_italic = italic
textareas_mark_underline = underline
textareas_mark_strike = strikethrough
textareas_mark_plain = plain
sensor_permission = Motion access
perm_request = Request
perm_open_settings = Open Settings
nav_location = Location
location_permission = Location access
location_start = Start
location_stop = Stop
location_waiting = waiting for a fix…
location_unavailable = no location service on this platform
location_coords = { $lat }, { $lon }
location_altitude = Altitude
location_accuracy = Accuracy
location_unknown = —
chart_axes = x red · y teal · z violet

# Fullscreen cover (docs/cover.md): presented over the whole window from cover(signal).
stack_cover_button = Cover
cover_title = Fullscreen cover
cover_dismiss = Dismiss

# Half sheet (docs/cover.md): the bottom half of the window, the page on show beneath it.
stack_half_button = Half sheet
half_title = Half sheet
half_level = Level
half_readout = Sheet level: { $level }

stack_unsaved = Unsaved changes
stack_discard_title = Discard changes?
stack_discard_body = You have unsaved changes on this page.
stack_discard_ok = Discard

tab_dynamic_title = Data-driven tabs
tab_dynamic_add = Add tab
tab_dynamic_remove = Remove tab

# App-local file storage (services page, docs/fs.md).
storage_title = Local files
storage_placeholder = Text to store
storage_save = Save file
storage_load = Load
storage_delete = Delete
storage_files_label = Stored files
storage_idle = Nothing stored yet

# --- The Settings surface's title (docs/settings.md): the window on a desktop, the section on a phone ---
prefs_window_title = Preferences

# --- Tree page (docs/tree.md) ---
nav_tree = Tree
tree_expand_all = Expand All
tree_collapse_all = Collapse All
tree_reveal = Reveal tree.rs
tree_add_file = Add File
tree_multi = Multi-select
tree_lock = Lock docs
tree_hint = Drag a row onto a folder to move it. While the lock is on, docs refuses drops.
tree_count = { $count ->
    [one] { $count } node
   *[other] { $count } nodes
}
tree_selection_none = Nothing selected
tree_selection = { $count ->
    [one] Selected: { $names }
   *[other] Selected { $count }: { $names }
}
tree_move_none = No moves yet
tree_move = Moved { $name } into { $parent } at { $index }
tree_move_append = Moved { $name } into { $parent }
tree_root = the root
tree_ctx_new_file = New File
tree_ctx_duplicate = Duplicate
tree_ctx_move_up = Move Up
tree_ctx_move_down = Move Down
tree_ctx_delete = Delete

# --- Toolbars page (docs/toolbars.md) ---
nav_toolbars = Toolbars
toolbar_unsupported = This toolkit has no window toolbar.
toolbar_readout_title = What the toolbar is doing
toolbar_controls_title = Driving it from here
# Item labels: these appear in the toolbar, so they stay short.
toolbar_new = New Window
toolbar_menu_open_scripting = Open the Scripting page
toolbar_menu_copy_script = Copy the script
toolbar_extra_tooltip = Copy the toolkit and version, for a bug report
toolbar_extra = Copy Info
toolbar_search_placeholder = Search
show_source = Show Source
# The live readout.
toolbar_query_label = Search text
toolbar_query_empty = (empty)
toolbar_star_label = Star
toolbar_on = On
toolbar_off = Off
toolbar_presses_label = Presses
toolbar_presses = { $count ->
    [one] { $count } press
   *[other] { $count } presses
}
toolbar_last_label = Last action
toolbar_last_none = Nothing yet
toolbar_appearance_label = Appearance
toolbar_appearance_ignored = { $mode } (ignored here)
toolbar_transport_label = Recorder
toolbar_transport_idle = Idle
toolbar_transport_recording = Recording
toolbar_transport_playing = Playing
toolbar_transport_paused = Paused
toolbar_last_new = New window
toolbar_last_extra = Build info copied
# The page's own controls.
toolbar_extra_label = Show the Copy Info item
toolbar_enabled_label = Show Source enabled
toolbar_clear_search = Clear search
toolbar_seed_search = Fill search
toolbar_seed_text = toolbars
# --- day-part-local-notify (docs/notify.md) ---
nav_notify = Notifications
notify_caps_post = Posting is supported
notify_caps_unsupported = Unavailable in this build. On macOS, notifications need a signed app bundle (day pack).
notify_caps_schedule_os = Scheduled notifications survive the app closing
notify_caps_schedule_process = Scheduled notifications are lost if the app exits
notify_title_label = Title
notify_title_placeholder = Notification title
notify_title_default = Hello from Day
notify_body_label = Body
notify_body_placeholder = Notification body
notify_body_default = Posted by the Day showcase.
notify_delay = Delay
notify_delay_now = Now
notify_delay_5s = 5 seconds
notify_delay_15s = 15 seconds
notify_delay_60s = 1 minute
notify_importance = Importance
notify_importance_low = Low
notify_importance_default = Default
notify_importance_high = High
notify_importance_urgent = Urgent
notify_sound = Play a sound
notify_badge = Badge count
notify_post = Post
notify_cancel = Cancel all
notify_status_idle = Nothing posted yet
notify_status_posted = Posted
notify_status_scheduled = Scheduled
notify_status_cancelled = Cancelled every notification
notify_status_failed = Could not post
notify_last = Last result
notify_perm_granted = Notification permission granted
notify_perm_missing = Permission not granted: posts are dropped
notify_perm_request = Request permission
haptics_songs_caption = Sequences
haptics_song_celebration = Celebration
haptics_song_levelup = Level up
haptics_song_heartbeat = Heartbeat
haptics_song_cascade = Cascade

# --- scripting (dayscript recorder) ---
nav_scripting = Scripting
scripting_record = Record
scripting_stop = Stop
scripting_copy = Copy
scripting_export = Export
scripting_copied = Copied to clipboard
scripting_delay_label = Delay between steps
scripting_delay_unit = sec
scripting_saved_label = Saved scripts
scripting_pick = Load a saved script…
scripting_save = Save
scripting_name_hint = My Script
scripting_saved = Saved

# --- app badge (docs/badge.md) ---
nav_badge = App badge
badge_caps_native = The app icon shows a badge
badge_caps_emulated = Sent; the shell decides whether it appears
badge_caps_none = No app-badge API on this platform
badge_android_note = Android derives the badge from posted notifications: set a notification's badge count instead.
badge_count_label = Count
badge_minus = −
badge_plus = +
badge_set = Set badge
badge_clear = Clear
badge_set_text = Set text "beta"
badge_last = Last action
badge_status_idle = Nothing set yet
badge_status_set = Badge set to { $count }
badge_status_cleared = Badge cleared
badge_status_text = Badge set to "beta"

# The Controls page mixer: one shared state, many editors.
mix_custom = Custom
voice_search_placeholder = Filter flavors…

# Context-menu demos (menus page): the message-list rows and the media card.
menu_reply = Reply
menu_forward = Forward
menu_share = Share…
menu_copy_image = Copy Image
menu_save_image = Save to Photos
menu_get_info = Get Info
menus_messages_section = A message list
menus_photo_section = A media card
msg_subject_one = Quarterly numbers, first pass
msg_subject_two = Weekend plan, final answer
msg_subject_three = Sketches from the workshop

# The Star command (commands.rs): one label per state, shared by the toolbar, the
# application menu and each navigation row's context menu.
# Save a picture of this window (commands.rs, docs/window-image.md).
cmd_screenshot = Screenshot…
cmd_appearance_light = Light
cmd_appearance_system = System
cmd_appearance_dark = Dark
cmd_record = Record
cmd_stop_recording = Stop
cmd_play = Play
cmd_pause = Pause
cmd_resume = Resume
cmd_clear_recording = Clear Recording
menu_appearance = Appearance
menu_script = Script
cmd_star = Star
cmd_unstar = Unstar
cmd_toggle_pseudo_locale = Toggle Pseudo-Locale

# Speech (day-part-speech): the daybridge reference part. One API, a different language per
# platform (docs/bridge.md).
speech_title = Text to speech
speech_phrase = A clear day, with a chance of rain later.
speech_speak = Speak
speech_stop = Stop
speech_support_label = Support here
speech_state_speaking = Speaking…
speech_state_finished = Finished
speech_state_stopped = Stopped
speech_native = Native
speech_emulated = Emulated (partial)
speech_unsupported = Unsupported on this target

# Support banners (widgets.rs): shown over a demo the target cannot run.
support_missing_here = Not supported on this platform — the demo stays visible so you can see it, but it will not work here.
support_emulated_here = Partly supported here — this demo runs, but not on the platform's own native implementation.
vectors_live_tint = Live tint
vectors_cycle_tint = Cycle

# Layout page (docs/size-classes.md "Row fit policies")
layout_note = Pick a policy, then add components until the row runs out of room.
layout_row_section = Row fit
layout_fit_label = Fit policy
layout_fit_clip = Clip
layout_fit_wrap = Wrap
layout_fit_wrap_columns = Even columns
layout_fit_column = Column
layout_fit_scroll = Scroll
layout_count_label = Components
layout_item = Item { $n }
layout_item_wide = Item { $n } (wider)

# The Resources page's Weight picker reuses the Text page's weight names (text_weight_*),
# same three words, same meaning, so only the notes below are new here.
vectors_alias_note = No Bold art: the base glyph is drawn
# The two color wells beside Cycle (docs/colorpicker.md).
vectors_pick_tint = Pick a tint

# The Content List page (src/pages/content_list): the `day new` scaffold's own item list and
# editor, so the keys below are the scaffold's, verbatim; the copied code reads them by name.
nav_content_list = Content list
nav_items = Items
cmd_add = New Item
cmd_delete = Delete
items_empty = No items yet. Add your first item.
item_none = Select an item to edit.
item_untitled = Untitled
field_name = Name
field_name_hint = Name this item…
field_done = Done

# The Split page (day/docs/split.md): two panes, a divider, one state read from both sides.
nav_split = Split
split_support_title = On this toolkit
split_support_label = Divider drawn by
split_native = The toolkit's own splitter
split_composed = Day (this toolkit has no splitter of its own)
split_hint = Drag the divider or move the share slider.
split_arrangement_title = Arrangement
split_stacked = Stacked
split_share = Form's share
split_form_title = Shapes
split_caption = Caption
split_caption_placeholder = Drawn over the shapes
split_count = Count
split_radius = Radius
split_hue = Hue
split_filled = Filled
split_shape = Shape
split_shape_circle = Circle
split_shape_square = Square
split_shape_diamond = Diamond
split_readout = { $count } shapes of radius { $radius }

# The Cursors page (day/docs/cursor.md): the `.cursor()` decorator, shape by shape.
nav_cursors = Cursors
cursors_support_title = On this toolkit
cursors_support_label = Support here
cursors_native = Native
cursors_emulated = Emulated (nearest shapes)
cursors_unsupported = Unsupported on this target
cursors_hint = Move the pointer over a tile.
cursors_shapes_title = Pointer shapes
cursors_drag_title = Drag and drop
cursors_resize_title = Resize and zoom
cursors_busy_title = Busy
cursors_reactive_title = One box, many shapes
cursors_reactive_pick = Shape
cursors_nested_title = Nearest ancestor wins
cursors_nested_outer = Crosshair out here
cursors_nested_inner = A hand in here
cursors_native_title = This toolkit's own
cursors_native_none = This toolkit names no shapes beyond the shared set.

# Permission reasons: what iOS and HarmonyOS show when the app asks (Day.toml [permissions]).
permission_camera = Preview and take a photo on the Camera page.
permission_location_when_in_use = Show your coordinates live on the Device & sensors page.
permission_motion = Show live accelerometer, gyroscope and magnetometer traces on the Device & sensors page.
permission_NSBluetoothAlwaysUsageDescription = Show how Day asks for Bluetooth — the showcase never connects to a device.

# Camera (src/pages/camera.rs): the viewfinder, its permission, and the last photo.
nav_camera = Camera
camera_permission_section = Permission
camera_permission_label = Camera access
camera_viewfinder_section = Viewfinder
camera_take_photo = Take photo
camera_flip = Flip
camera_state_label = Camera
camera_last_photo_section = Last photo
camera_photo_label = Photo
camera_no_photo = None yet
camera_photo_size = { $width } × { $height } · { $name }

list_activated = Last activated row: { $row } (0 = none)

ctl_icon_label = Icon and label
ctl_icon_only = Icon only
ctl_image = Bundled image

nav_drag_drop = Drag & Drop
dnd_hint = Drag a tile onto another tile or another Showcase window. Circles marked with an X reject drops.
dnd_ready = Ready to receive a drop.
dnd_empty = Drop here
dnd_new_window = New window
dnd_choose_file = Choose file…
dnd_accept = Accept drops

dnd_data = { $mime }: { $count } bytes

files_reopen = Reopen last file
files_bookmark_failed = Access to the saved file could not be restored. Choose it again using Open.

dnd_disallowed = Add disallowed regions

# Native display frame demo
frame_hint = Tap to throw the marbles. Drag one to fling it.
frame_bounce = Bounce
frame_shuffle = Shuffle
frame_pause = Pause
frame_waiting = Waiting for a frame
frame_running = Running
frame_paused = Paused
frame_resting = Resting
frame_balls_label = Marbles
frame_fps = FPS · min { $min } · max { $max } · avg { $avg }

commands_title = Reusable commands
commands_add = Add one

# The Physics page (src/pages/physics.rs): the ball mass and planetary gravity sliders, and the
# bodies the gravity readout names.
physics_gravity_label = Gravity
physics_gravity = { $value } m/s²
physics_gravity_named = { $value } m/s² ({ $body })
physics_mass_label = Mass
physics_mass = { $value } Mt
physics_elasticity_label = Elasticity
physics_elasticity = { $value }%
physics_fullscreen = Full screen
physics_exit_fullscreen = Exit full screen
physics_zero_g = zero-G
physics_moon = Moon
physics_mars = Mars
physics_earth = Earth
physics_jupiter = Jupiter

# HTTP sessions, interception and payload statistics.
net_transport = Transport
net_provider = HTTP provider
net_native = Native
net_reqwest = reqwest
net_native_only = This target uses the native provider.
net_provider_note = Applies to new requests and sockets.
net_simulation = Network simulation
net_intercept = Intercept requests
net_speed = Transfer speed
net_speed_value = { $speed } KiB/s per transfer
net_failures = Failure rate
net_failure_value = { $rate }% of requests
net_pause = Pause delivery
net_offline = Offline
net_method = Method
net_url = URL
net_body = Request body
net_cancel = Cancel
net_cancelled = Cancelled
net_download_preset = Download preset
net_upload_preset = Upload preset
net_received = Received { $bytes } bytes
net_error = Request failed: { $error }
net_request = HTTP request
net_ws_note = Enter a live WebSocket URL, or turn on interception for the simulated echo.
net_message = Message
net_connected = Connected
net_closed = Closed
net_real_down = Download · real
net_real_up = Upload · real
net_sim_down = Download · simulated
net_sim_up = Upload · simulated
net_seconds = Seconds
net_rate = KiB/s
net_series = Traffic
net_totals = Sent { $sent } bytes · received { $received } bytes · active { $active } · failed { $failed }
net_average = Session average: upload { $upload } KiB/s · download { $download } KiB/s
net_statistics = Live traffic
net_send_request = Send request
net_options = Request options
net_headers = Headers (JSON)
net_timeout = Idle timeout
net_timeout_value = { $seconds } seconds

# SI units used by the motion-sensor readouts.
sensor_unit_acceleration = m/s²
sensor_unit_rotation = rad/s
sensor_unit_magnetic = µT

sensor_live = Live readings
sensor_compass = Magnetic compass
sensor_compass_hint = Hold the phone flat. Red points to magnetic north.
sensor_north = N
sensor_east = E
sensor_south = S
sensor_west = W
sensor_heading = Magnetic heading: { $degrees }°

sensor_calibrated = Calibrated
sensor_tilt = X/Y acceleration level
sensor_tilt_hint = The bubble follows X/Y acceleration, one g at the rim.
sensor_rotation = Rotation-rate gauges
sensor_rotation_hint = Rotation speed per axis, ±π rad/s at the ends.
sensor_uncalibrated_hint = Uncalibrated: the needle may not point north.
sensor_axis_x = X
sensor_axis_y = Y
sensor_axis_z = Z

# --- Security: the keychain and the biometric prompt (docs/keychain.md, docs/biometrics.md) ---
nav_security = Security
keychain_title = Keychain
keychain_support_label = Store here
keychain_backend_label = Backend
keychain_key_label = Key
keychain_key_placeholder = showcase.session
keychain_secret_label = Secret
keychain_secret_placeholder = Something worth protecting
keychain_protection_label = Protection
keychain_level_unlocked = When unlocked
keychain_level_first_unlock = After first unlock
keychain_level_presence = User presence
keychain_save = Save
keychain_read = Read
keychain_read_presence = Read with verification
keychain_remove = Remove
keychain_idle = Nothing stored yet
keychain_saved = Saved { $key }
keychain_read_value = Read back: { $value }
keychain_removed = Removed { $key }
keychain_entries_label = Stored keys
keychain_entries_none = None
keychain_presence_reason = Read the secret the Security page stored.
biometrics_title = Biometrics
biometrics_support_label = Support here
biometrics_availability_label = Availability
biometrics_biometry_label = Sensor
biometrics_reason_placeholder = Confirm it is you to continue.
biometrics_credential_label = Allow the device passcode
biometrics_authenticate = Authenticate
biometrics_cancel = Cancel
biometrics_idle = Not asked yet
biometrics_prompting = Waiting for the prompt…
biometrics_verified = Verified ({ $method })
permission_biometrics = Confirm it is you on the Security page.

# --- Sign-in: day-part-auth (the auth browser, OpenID Connect, passkeys, Apple, Google) ---
nav_signin = Sign-in
signin_oidc_title = OpenID Connect
signin_provider_demo = Demo provider (built in)
signin_provider_google = Google
signin_provider_apple = Apple (web flow)
signin_provider_custom = Any issuer (discovery)
signin_issuer_placeholder = Issuer URL, discovered when you sign in
signin_client_id_placeholder = Client id, as registered at the provider
passkeys_rp_placeholder = Relying party (a domain this app is associated with)
signin_sign_in = Sign in
signin_sign_in_silent = Sign in silently
signin_refresh = Refresh
signin_userinfo = Userinfo
signin_revoke = Revoke
signin_sign_out = Sign out
signin_idle = Not signed in
signin_working = Waiting for the provider…
signin_signed_in = Signed in as { $name } ({ $email }), subject { $sub }
signin_tokens = Access token good for { $secs } s · scope { $scope } · { $refresh }
signin_refresh_yes = refresh token issued
signin_refresh_no = no refresh token
signin_need_client_id = Paste the client id you registered at the provider first.
signin_need_discovery = Name the issuer and the client id first; discovery runs when you sign in.
signin_silent_demo_only = The silent sign-in is the demo provider's; the others need their consent page.
signin_nothing_to_refresh = Sign in first: there is no refresh token to trade.
signin_refreshed = Refreshed: a new access token was issued
signin_userinfo_ok = Userinfo answered for { $sub }
signin_revoked = The access token was revoked
signin_signed_out = Signed out
passkeys_title = Passkeys
passkeys_available_yes = Platform authenticator available
passkeys_available_no = No platform authenticator on this device
passkeys_create = Create a passkey
passkeys_get = Sign in with a passkey
passkeys_idle = No credential yet
passkeys_prompting = Waiting for the authenticator…
passkeys_created = Created credential { $id }
passkeys_asserted = Signed in with credential { $id }
apple_title = Sign in with Apple
apple_sign_in = Sign in with Apple
apple_signed_in = Apple user { $user }
google_title = Google
google_client_placeholder = Server client id
google_sign_in = Sign in with Google
google_signed_in = Google account { $id }

# Numeric notification count.
badge_count = { $count }

# Map (src/pages/map.rs): day-piece-map over the built-in sample island or a live OpenFreeMap
# style, the camera calls on buttons, and a readout of what the piece reports.
nav_map = Map
map_source_sample = Sample island
map_source_liberty = OpenFreeMap Liberty
map_source_positron = OpenFreeMap Positron
map_source_bright = OpenFreeMap Bright
map_source_fiord = OpenFreeMap Fiord
map_source_dark = OpenFreeMap Dark
map_source_colorful = VersaTiles Colorful
map_source_colorful_dark = VersaTiles Colorful Dark
map_fly = Fly
map_fit = Fit
map_rotate = Rotate
map_north = North
# The places each source marks: the island's center and east coast, or two London sights.
map_sight = { $sight ->
        [center] Island center
        [coast] East coast
        [trafalgar] Trafalgar Square
        [here] Your location
       *[bridge] Tower Bridge
    }
# Status · latitude, longitude · zoom · bearing.
map_readout = { $status ->
        [ready] Ready
        [failed] Failed: { $reason }
       *[loading] Loading
    } · { $lat }, { $lng } · zoom { $zoom } · { $bearing ->
        [0] North up
       *[other] Bearing { $bearing }°
    }
# What the last tap found: a marker, a feature's layer and name, or nothing.
map_tap_prompt = Tap the map
map_tap_marker = Marker: { $sight ->
        [center] Island center
        [coast] East coast
        [trafalgar] Trafalgar Square
        [here] Your location
       *[bridge] Tower Bridge
    }
map_tap_feature = { $layer }: { $name }
map_tap_layer = { $layer }
map_tap_nothing = Nothing there
map_locate = Current Location
map_locating = Finding your location…
map_locate_failed = { $why ->
        [denied] Location access is off for this app. Turn it on in Settings.
        [off] Location services are off.
        [timeout] No location fix yet. Try again.
        [unavailable] Location isn’t available on this device.
       *[other] Couldn’t find your location.
    }

# Interactive layout sizing and translation.
layout_reactive_title = Reactive sizing
layout_reactive_width = Width
layout_reactive_height = Height
layout_reactive_horizontal = Horizontal offset
layout_reactive_vertical = Vertical offset
layout_reactive_reset = Reset
layout_reactive_points = { NUMBER($value, maximumFractionDigits: 0) } pt
layout_reactive_shape = Shape: { NUMBER($width, maximumFractionDigits: 0) } by { NUMBER($height, maximumFractionDigits: 0) } points

push_caption = Send a test push to this device through the Day relay.
push_unavailable = Push notifications are unavailable in this build.
push_send_now = Send now
push_send_later = { $seconds ->
    [one] Send in 1 second
   *[other] Send in { $seconds } seconds
}
push_hint = The first send asks for permission.
push_sending = Sending…
push_requested = Waiting for a device token from the push service…
push_no_token = The push service did not provide a device token. Try again in a moment.
push_denied = Notifications are turned off for this app. Allow them in Settings, then send again.
push_too_soon = The relay allows one test every 30 seconds. Wait a moment, then send again.
push_network_busy = The relay has registered many devices from this network in the last hour. Try again later.
push_accepted = Sent. It should arrive in a moment.
push_scheduled = { $seconds ->
    [one] Arriving in 1 second. Leave the app to see it in the background.
   *[other] Arriving in { $seconds } seconds. Leave the app to see it in the background.
}
push_received = Arrived while the app was open: { $title }
push_opened = Opened from the notification: { $title }
push_advanced = Advanced
push_token = Device token
push_token_apns = Received from APNs
push_token_fcm = Received from FCM
push_token_none = Not received yet
push_relay = Relay registration
push_relay_none = Not registered
push_sandbox = Use the APNs sandbox (development builds)
push_waiting = No notification received yet
push_remove = Remove from relay
push_removed = Device removed from the relay
push_error = Push failed: { $error }

# Sharing, grouped lists, and app-level Back demonstrations.
list_opt_inset_grouped = Inset grouped
list_opt_inset_hint = Inset rows on macOS and rounded groups on iOS. Other platforms keep their plain list.
share_demo_title = Sharing
share_demo_fallback = This platform shows no custom share actions. Use the buttons below.
share_demo_open = Share Day website…
share_demo_remember = Remember link
share_demo_clear = Clear result
share_demo_remembered = Remember link selected
share_demo_cleared = Share result cleared
share_demo_failed = The share chooser could not be opened.
share_demo_receive_hint = Share text or a link to Day Showcase from another app.
share_demo_none = Nothing received yet.
system_back_status = App Back: { $remaining } remaining, { $handled } handled
system_back_press = Back
system_back_stop = Disarm
system_back_hint = Pushed pages and covers take Back before the app does.
system_back_arm = Arm 3 Back presses

# Map provider selection and setup.
map_provider_osm = OpenStreetMap
map_provider_apple = Apple Maps
map_provider_google = Google Maps
map_missing_api_key = Configure a Google Maps API key for this Android app.
map_play_services_unavailable = Google Maps requires Google Play services.
map_load_failed = The map could not load. Check the connection and provider configuration.

resources_missing = Bundled resource unavailable: { $name }

# The gallery pages added with the card layout, and the line under every page's title.
nav_styles = Styles
nav_images = Images
nav_gestures = Gestures
detail_controls = Buttons, switches, sliders, steppers, pickers and progress
detail_dates = Date and time pickers, in the platform's calendar
detail_focus = Keyboard focus as app state
detail_text = Fonts, styled runs, Markdown, selection and truncation
detail_text_areas = Multi-line editing, code modes and live Markdown
detail_text_fields = Single-line input, keyboards, secure entry and search
detail_toolbars = Window and page commands in the platform's bars
detail_localization = Four languages, plurals and right-to-left layout
detail_canvas = Paths, gradients, text and images drawn every frame
detail_styles = Corners, borders, shadows, clipping and transforms
detail_images = Fit modes, corners, remote pictures, placeholders and fades
detail_gestures = Drag, pinch, rotate, press and hover on any view
detail_charts = A grammar of marks: lines, bars, areas and selections
detail_animation = Platform animations, tweens, easing and springs
detail_physics = Marbles and a spring on the frame clock
detail_benchmark = A patchwork of live views, timed against SwiftUI
detail_grid = Fixed columns, spans, alignment and sizing
detail_layout = Rows, columns, stacks, wrapping and size classes
detail_split = Resizable panes with the platform's own splitters
detail_list = Recycled rows, sections, swipes, reordering and selection
detail_tree = Outlines that expand, select, reorder and load lazily
detail_drag_drop = Text, files and images dragged between views and apps
detail_model = Typed stores, derived values and undo
detail_query = A database queried live, with search and sorting
detail_tabs = Tab bars and segmented pages, each with its own state
detail_stack = Pushed pages, a path you own, and the back gesture
detail_content_list = A list, its items and a detail in three columns
detail_media = A native video player with its own transport
detail_lottie = Vector animations, played, paused and scrubbed
detail_map = Vector tiles on the canvas beside the platform's maps
detail_web_view = A bundled site and the live web, with JavaScript both ways
detail_menus = Menus, context menus, alerts, sheets and prompts
detail_system = Battery, sensors, location, device facts and appearance
detail_network = Requests, uploads, downloads, proxies and connectivity
detail_notify = Local notifications and the app's badge
detail_push = Remote notifications through a relay you can try
detail_speech = Speech, sound effects and haptics
detail_camera = Live preview, capture and the photo that comes back
detail_cursors = Every pointer shape the platform offers
detail_files = The clipboard, preferences, file pickers and app storage
detail_security = The keychain and biometrics
detail_sign_in = Sign in with Apple, Google, a passkey or any OpenID provider
detail_scripting = The scripts that drive and screenshot this app
detail_resources = Bundled images, vector glyphs, fonts and data files
detail_tweaks = Per-toolkit adjustments to the native views
detail_crash_reporting = Crashes caught and reported on the next launch
detail_about = What Day is, and what this app shows
detail_settings = Appearance, language and the app's own preferences

async_data_title = Async data
async_data_note = Edit the field, then refresh or fail a request. The content and your edit stay visible. Changing the source starts fresh.
async_data_refresh = Refresh
async_data_cancel = Cancel
async_data_source = Change source
async_data_fail = Fail the next request
async_data_value = Source { $source }, request { $request }
async_data_edit = This edit survives refresh
async_data_loading = Loading…
async_data_failed = The request failed.
async_data_refreshing = Refreshing…
async_data_retained = Refresh failed; previous data is still shown.
