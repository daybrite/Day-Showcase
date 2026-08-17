app_title = Day Showcase
counter_value = { $count ->
    [one] { $count } click
   *[other] { $count } clicks
}
decrement = −
increment = +
name_placeholder = Your name
value_label = Value
progress_label = Progress
flavor_label = Flavor
flavor_placeholder = Type or pick a flavor
flavor_add = Add
flavor_ios_note = iOS has no combo box control, so Day shows a placeholder here.
history_entry = count became { $value }
nav_controls = Controls
nav_menus = Menus & dialogs
nav_text = Text
nav_battery = Battery
nav_sensors = Sensors
nav_clipboard = Clipboard
nav_network = Network
nav_media = Media
nav_compose = Compose
nav_files = Files
nav_tabs = Tabs
nav_stack = Stack
nav_layout = Layout
nav_list = List
nav_refresh = Refresh
refresh_caption = Pull the feed down — or use the button — to reload
refresh_status_idle = Idle
refresh_status_refreshing = Refreshing…
refresh_now = Refresh now
refresh_tier_native = Pull-to-refresh: native
refresh_tier_emulated = Pull-to-refresh: emulated
refresh_row = Item { $n }
nav_webview = Web View
nav_lottie = Lottie
nav_about = About

shapes_kinds = Kinds
gradients_title = Gradients
gradient_angle = Angle
shapes_angle = Angle

picker_selected = Selected
picker_segmented = Segmented
picker_menu = Menu
picker_inline = Inline

# — day-piece-datetime —
nav_dates = Date & time
dates_caption = Native date & time pickers bound two-way to civil date/time signals — pickers in the same section share one signal.
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

compose_caption = Pure-composition pieces — no native code, no cargo features, every backend for free.
compose_rating_label = Star rating
compose_rating_count = Stars selected:
compose_rating_placeholder = 1–5
compose_card_title = Reusable surface
compose_card_body = Padding + background + rounded corners, applied as a Modifier.
compose_plain_btn = Plain
compose_styled_btn = Filled
compose_env_value = Tinted by the provided accent
list_add = Add 100
list_caption = { $count } rows — only the visible cells are built
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

webview_url_hint = Enter a URL
webview_go = Go
webview_back = Back
webview_forward = Forward
webview_stop = Stop
webview_reload = Reload
webview_js_hint = JavaScript to run in the page
webview_js_run = Run JS »
webview_js_result_hint = The JSON result appears here
webview_note_iframe = This platform embeds the page in an iframe. The browser blocks cross-origin history and URL readback, so Back, Forward and Stop are unavailable and the field above will not follow links opened inside the page.
webview_tab_remote = Remote
webview_tab_embedded = Embedded
webview_embedded_caption = A complete site — pages, stylesheet, script, image — ships inside the app under resource/assets/web/minisite/ and loads with no network. Relative links stay in the site; external links open outside; day-showcase:// links are intercepted and navigate this app.
webview_embedded_unsupported = No web engine is available in this toolkit build.
webview_embedded_status_none = No external link followed yet.
webview_embedded_opened = Opened outside the app: { $url }
webview_embedded_intercepted = Intercepted — navigated in-app to "{ $route }".

lottie_caption = A native Lottie animation, bundled as JSON (lottie-ios / lottie-android)
lottie_speed = Speed
stack_root_body = A real push/pop stack. Its path is an app-owned signal.
stack_push = Push a detail
stack_detail_title = Level { $depth }
stack_detail_body = Pushed onto the path. The native back button writes the pop back.
stack_item_title = Item { $id }
stack_link_42 = Open item-42 with a hint (absolute route)
stack_param_hint = Opened with hint: {$hint}
tab_one = Overview
tab_two = Details
tab_three = Settings
tab_one_body = The overview tab. Each tab keeps its own state.
tab_two_body = The details tab, selected by its route key.
tab_three_body = The settings tab. Deep links and dayscript select tabs by key.
tab_state_note = Switch tabs and back — this tab's state survives.
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
files_caption = Native open/save file pickers. Open reads a text file into the editor; Save writes it back out.
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

# Sensors playground (docs/sensors.md)
sensor_accelerometer = Accelerometer
sensor_gyroscope = Gyroscope
sensor_magnetometer = Magnetometer
sensor_reading = x { $x } · y { $y } · z { $z } { $unit }
sensor_waiting = waiting for first sample…
sensor_unavailable = unavailable on this device

# Clipboard playground (docs/clipboard.md)
clipboard_caption = The day-part-clipboard part reads and writes the system clipboard natively.
clipboard_placeholder = Type something to copy
clipboard_copy = Copy
clipboard_paste = Paste
clipboard_idle = Clipboard untouched
clipboard_copied = Copied to the system clipboard
clipboard_copy_failed = Copy failed (no clipboard API here)
clipboard_pasted = Pasted from the system clipboard
clipboard_empty = Clipboard is empty (or unreadable in the background)

# Network playground (docs/network.md)
network_refresh = Read Network
network_reading_online = Online · { $kind } · metered: { $expensive }
network_reading_offline = Offline
network_reading_none = No connectivity API on this platform

# Media playground (docs/media.md)
media_play = Play
media_pause = Pause
media_load = Load

# — Localization page (docs/localization.md: live locale, NUMBER/DATETIME, plurals, collation) —
nav_localization = Localization
fmt_caption = One set of translations — ICU-correct rendering per locale: numbers, dates, plural grammar, and sort order all follow the language.
loc_locale_section = Live locale
loc_live_note = The locale is a signal — switching re-renders every string instantly. Layout direction is fixed at launch (launch with ar for the mirrored UI).
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
text_caption = Semantic styles map to the platform's native text styles and accessibility text scaling.
text_selectable_toggle = Selectable
text_styles_header = Styles
text_weights_header = Weights
text_styling_header = Bold & italic
text_colors_header = Color
text_custom_header = Custom sizes
text_custom_note = Font.System(pt) — still scaled by the accessibility text size (Dynamic Type / font scale).
text_fonts_header = Bundled fonts
text_fonts_note = Font.Custom("Family", pt) — files from the app's resource/fonts/ directory, bundled by day build and resolved by family name on every platform.

# Menus playground
menus_caption = The transient native surfaces: the menu bar, per-piece context menus, and imperative dialogs.
menus_last = Last action
menus_lifecycle = Lifecycle
menus_target = Right-click here (long-press on mobile) for a context menu
menus_shortcut_hint = Keyboard shortcuts (⌘/Ctrl + key) are shown in the menu bar and work while the app is focused — e.g. New (N), Save (S), Reload (R), Save As (⇧/Shift + S).

# --- day-part-haptics ---
nav_haptics = Haptics
haptics_supported_yes = Haptic engine available on this platform
haptics_supported_no = No haptic engine on this platform (buttons are silent)
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

# --- day-part-prefs ---
nav_prefs = Preferences
prefs_caption = Persist a string across launches with day-part-prefs.
prefs_placeholder = Value to remember
prefs_save = Save
prefs_load = Load
prefs_clear = Clear
prefs_idle = Type a value, then Save.
prefs_empty = (nothing stored)
prefs_saved = Saved.
prefs_save_failed = Save failed.
prefs_loaded = Loaded from storage.
prefs_missing = Nothing stored yet.
prefs_cleared = Cleared.
prefs_value_label = Stored value:

# --- bundled resources (§18.3) ---
nav_resources = Resources
resources_caption = An image loaded by name from a bundled resource, plus random-access reads of embedded data.
vectors_title = Vectors
vectors_note = The sidebar's glyphs, drawn from resource/vectors/ — one SVG per icon, resolution-independent on every backend.
vectors_tints = Tints
vectors_scene = Full-colour art
vectors_zoom = Zoom
vectors_scene_note = One 240-path SVG, redrawn from the vector at every zoom step rather than magnified — the edges stay crisp at any size. On Android it ships as a VectorDrawable, not a raster.
vectors_weights = Weights
vectors_sizes = Sizes
resources_numbers = numbers.bin: { $len } bytes, byte[100] = { $byte }
resources_greeting = greeting.txt: { $text }

# --- day-part-deviceinfo ---
nav_deviceinfo = Device Info
deviceinfo_model = Model: {$value}
deviceinfo_system = System: {$name} {$version}
deviceinfo_simulator = Simulator: {$value}
deviceinfo_yes = yes
deviceinfo_no = no
deviceinfo_refresh = Refresh

# --- day-piece-activity ---
activity_animating = Animating
activity_on = Spinning
activity_off = Stopped

# --- day-piece-searchfield ---
nav_search = Search
search_clear = Clear

# --- day-piece-map ---
nav_map = Map
map_caption = A native MKMapView — Apple platforms only. Tap a preset to recenter the map live.
map_boston = Boston
map_paris = Paris

# — tweaks page (docs/tweaks.md) —
nav_tweaks = Tweaks
tweaks_intro = Packaged tweaks configure the native widget behind a built-in piece, per toolkit. On toolkits a tweak doesn't cover, it is a no-op — the pieces below simply look stock.
tweaks_stock = Stock
tweaks_tweaked = Tweaked
tweaks_bezel_title = Button bezel
tweaks_bezel_caption = day-tweak-button-bezel — AppKit only: NSBezelStyle constants on the real NSButton.
tweaks_selectable_title = Selectable label
tweaks_selectable_caption = The .selectable() core modifier — the platform's own text selection on a label, opt-in, wherever the toolkit supports it.
tweaks_selectable_text = This label's text can be selected and copied — try it.
tweaks_tooltip_title = Tooltip
tweaks_tooltip_caption = day-tweak-tooltip — AppKit, GTK, Android: a native help tooltip, one modifier across three access tiers.
tweaks_tooltip_label = Auto-save
tweaks_tooltip_hint = Your work is saved automatically. Hover (or long-press on Android) for details.
tweaks_ticks_title = Slider tick marks
tweaks_ticks_caption = day-tweak-slider-tickmarks — AppKit, GTK, Android, Qt, XAML, ArkUI: native ticks, snapping where the platform supports it. The tweaked slider snaps; the stock one glides.
tweaks_ref_title = NativeRef liveness
tweaks_ref_caption = A NativeRef reaches the tweaked slider after mount; unmount it and the ref clears instead of dangling.
tweaks_ref_live = ref: live
tweaks_ref_cleared = ref: cleared
tweaks_label_title = Tweak a text label
tweaks_label_caption = An inline tweak (AppKit, UIKit) dims this label through its native view and reports the class it saw. On iOS, Selectable rebuilds the label as a read-only UITextView — the tweak runs after .selectable(), so it lands on the widget that ships.
tweaks_label_sample = This label is dimmed by a native tweak.
tweaks_label_class = Native class: { $class }

# — merged section pages (design overhaul) —
nav_canvas = Canvas & shapes
nav_system = Device & sensors
nav_services = Platform services
controls_basics = Basics
canvas_caption = Shapes, transforms, gestures, and composition-tier widgets — all drawn through the canvas.
paths_title = Paths, strokes & clipping
canvas_gauge = Canvas gauge
gauge_value_label = Value
system_caption = The headless device-state parts: battery, connectivity, motion sensors, and device identity.
services_caption = The headless "do something with the OS" parts: HTTP, clipboard, preferences, haptics, and file pickers.
subscribe_label = Subscribe

# — data strings localized for the walkthrough locales (option lists, specimen rows) —
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
media_caption = A native media player — the platform's own view, transport driven by triggers.
media_player_section = Video

# Resources page sections
resources_image_section = Bundled image
resources_modes_note = One image, three content modes — Fit preserves aspect, Fill crops, Stretch distorts.
image_mode_fit = Fit
image_mode_fill = Fill
image_mode_stretch = Stretch
resources_data_section = Data assets

# About page
about_app_section = This app
about_version = Version
about_toolkit = Toolkit
about_id = App ID
about_os = OS
about_model = Model
about_locale = Locale
history_hint = Tap + or − above and each change lands here.

# Focus page (docs/focus.md)
nav_focus = Focus
focus_caption = Focus is a two-way binding: native changes write the signal, and writing the signal moves focus.
focus_group_section = One signal, one form
focus_group_caption = Three fields bound to one optional enum signal. Click or Tab between them and the readout follows; Return hops to the next field.
focus_name_label = Name
focus_email_label = Email
focus_city_label = City
focus_current_label = Focused
focus_next = Focus next
focus_clear = Clear focus
focus_bool_section = One control, one Bool
focus_bool_caption = The same field bound to a Bool signal — the buttons write it; clicking in and out of the field writes it back.
focus_bool_placeholder = Focus lands here
focus_focus_btn = Focus
focus_blur_btn = Blur
focus_state_label = State
focus_state_on = focused
focus_state_off = blurred
focus_probe_section = Beyond text fields
focus_probe_caption = Desktop toolkits focus buttons, toggles, and sliders too; touch platforms mostly reserve focus for text input.
focus_probe_toggle = Toggle
focus_probe_slider = Slider
focus_probe_button = Button

# HTTP fetch demo (docs/http.md) — the status readout stays raw "<status> <body>" so the
# walkthrough asserts it byte-for-byte in every locale.
http_title = HTTP
http_caption = The day-part-http part fetches through the platform's own HTTP stack — its proxies, VPN, and TLS.
http_fetch = Fetch from localhost
http_idle = Nothing fetched yet
http_tier = Stack
http_url_placeholder = https://example.com
http_check = Check
http_checking = Checking…
http_patch = PATCH
http_res_label = Resource
http_res_refetch = Refetch

# Scrolling page (docs/scroll.md) — programmatic scroll targets.
scroll_to_top = Scroll to top
scroll_to_bottom = Scroll to bottom
scroll_to_item = Scroll to item 100

# Grid page (docs/grid.md) — grid/grid_row from basics to a stress test.
nav_grid = Grid
grid_caption = Columns sized by content, cells that span, and flexible cells that share the leftover width
grid_tab_basics = Basics
grid_tab_sizing = Sizing
grid_tab_spanning = Spanning
grid_tab_composite = Composite
grid_tab_stress = Stress
grid_basics_caption = Each column takes the width of its widest cell. No fixed widths, no placeholder spacers.
grid_col_name = Name
grid_col_wins = Wins
grid_col_points = Points
grid_sizing_caption = Fixed, content-sized, and flexible columns in one grid.
grid_sizing_fixed = Fixed 80 pt
grid_sizing_content = Content
grid_sizing_short = Short
grid_sizing_longer = A longer content cell
grid_spanning_caption = A cell can span columns; a bare child outside any row spans the whole grid.
grid_month_title = Week planner
grid_event_focus = Focus block
grid_event_review = Review
grid_composite_caption = Shapes and grid together: glyph groups in content columns beside a flexible range bar.
grid_day_n = Day { $n }
grid_stress_cells = { $n } rows of 8 cells, all laid out eagerly. Updating one cell re-measures only that cell.
grid_stress_add = Add 50 rows
grid_stress_bump = Bump the first cell

nav_animation = Animation
anim_caption = Queue scale, rotation, opacity, offset, and hue, then tap Animate! to run them all together with the chosen curve and duration.

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

# — Benchmark page (the Day-Bench Grids benchmark; on the Apple-native backends a segmented
#   picker also hosts its hand-written SwiftUI twin via day-piece-swiftui, docs/swiftui.md) —
nav_benchmark = Benchmark
bench_caption = A pseudo-random patchwork of grid cells that tiles the pane exactly. Both parameters repack every row, so the layout engine renegotiates all of it at once.
bench_parameters = Parameters
bench_seed = Random Seed
bench_count = Total Count
bench_rows = { $rows } { $rows ->
    [one] row
   *[other] rows
}
# %d templates for the hosted SwiftUI pane, whose row count lives in Swift @State (one/other only —
# the same fidelity for every locale, since printf templates cannot carry Fluent's plural rules).
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
text_markdown_caption = Type below. The label under the divider re-parses on every keystroke — the same path a translated string or a value off the network takes.
text_markdown_sample = Day parses **bold**, *italic*, `code`, ~~strikethrough~~ and [links](https://daybrite.dev) at run time. Unfinished markup like ** stays literal.
text_markdown_opened = Opened: {$url}
text_baseline_section = Baseline alignment
text_baseline_caption = Rows sit their text on one line rather than centering boxes of different heights. Turn it off to see the difference: the field's text, the label beside it, and the trailing unit drift apart.
text_baseline_toggle = Align baselines
text_baseline_quantity = Quantity
text_baseline_unit = items
text_baseline_total = Total
text_baseline_currency = USD
text_baseline_due = Due
text_baseline_unsupported = This toolkit reports no text baselines, so these rows stay centered.
text_links_caption = Tap a link to open it in the system browser.
text_link_icons_label = Material Symbols on Google Fonts
text_link_mail_label = Email the team

# Files section: the editor's seed text
files_initial_content =
    Hello from Day!
    Edit me, then Save.

# Crash Reporting page (day-break, docs/break.md)
nav_crash = Crash Reporting
crash_caption = Register crash handlers, review the report on next launch, and choose whether to send it.
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
crash_empty = No crash report yet. Trigger a crash, then relaunch to see it here.

# Text Areas page
nav_textareas = Text areas
textareas_caption = A native multi-line editor with live editable, selectable, and spell-check attributes (toolkit-permitting).
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
stack_cover_button = Present a cover
cover_title = Fullscreen cover
cover_body = Presented over the whole window by cover(signal). Native on iOS and Android, a topmost child elsewhere.
cover_dismiss = Dismiss

stack_unsaved = Unsaved changes
stack_discard_title = Discard changes?
stack_discard_body = You have unsaved changes on this page.
stack_discard_ok = Discard

tab_dynamic_title = Data-driven tabs
tab_dynamic_add = Add tab
tab_dynamic_remove = Remove tab

# App-local file storage (services page, docs/fs.md).
storage_title = Local files
storage_caption = The day-part-fs part stores app-private files on every target — real files natively, OPFS in the browser.
storage_placeholder = Text to store
storage_save = Save file
storage_load = Load
storage_delete = Delete
storage_files_label = Stored files
storage_idle = Nothing stored yet

# --- Preferences window (docs/windows.md) ---
prefs_window_title = Preferences
prefs_window_caption = Theme and language apply to every window and persist across launches.

# — Toolbars page (docs/toolbars.md) —
nav_toolbars = Toolbars
toolbars_caption = The window's own toolbar, in the platform's native chrome — an NSToolbar on macOS, the AdwHeaderBar on GNOME, a QToolBar on KDE, a CommandBar on Windows. Look at the top of this window; the controls below drive it.
toolbar_unsupported = This toolkit has no window toolbar, so nothing was installed. A phone puts these commands in the content instead.
toolbar_readout_title = What the toolbar is doing
toolbar_controls_title = Driving it from here
toolbar_vocabulary_title = The item vocabulary
# Item labels — these appear IN the toolbar, so they stay short.
toolbar_sidebar = Sidebar
toolbar_new = New Window
toolbar_star = Star
toolbar_menu = More
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
toolbar_last_star = Star toggled
toolbar_last_extra = Build info copied
# The page's own controls.
toolbar_extra_label = Show the Copy Info item
toolbar_enabled_label = Clear recording enabled
toolbar_clear_search = Clear search
toolbar_seed_search = Fill search
toolbar_seed_text = toolbars
# One line per item kind.
toolbar_kind_button = Button
toolbar_kind_button_note = Runs a command, and can share its closure with a menu item.
toolbar_kind_toggle = Toggle
toolbar_kind_toggle_note = Two-way with a signal, so the switch above and the button agree.
toolbar_kind_menu = Menu
toolbar_kind_menu_note = A pull-down built from the same entries the menu bar takes.
toolbar_kind_search = Search
toolbar_kind_search_note = The platform's search control, bound two-way to a signal.
toolbar_kind_space = Spacers
toolbar_kind_space_note = A flexible space splits leading items from trailing ones.
# --- day-part-local-notify (docs/notify.md) ---
nav_notify = Notifications
notify_caption = Post a local notification through the platform's own notification system. Scheduled ones are held by the OS where it can — see the capability line.
notify_caps_post = Posting supported on this platform
notify_caps_unsupported = Local notifications are unavailable in this build (on macOS they need a signed app bundle — use day pack)
notify_caps_schedule_os = Scheduled notifications survive the app closing
notify_caps_schedule_process = Scheduled notifications are an in-process timer and are lost if the app exits
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
notify_route = Tap opens
notify_post = Post
notify_cancel = Cancel all
notify_status_idle = Nothing posted yet
notify_status_posted = Posted
notify_status_scheduled = Scheduled
notify_status_cancelled = Cancelled every notification
notify_status_failed = Could not post
notify_last = Last result
notify_perm_granted = Notification permission granted
notify_perm_missing = Notification permission not granted — posts will be dropped
notify_perm_request = Request permission
haptics_songs_caption = Longer sequences — the gaps carry the rhythm as much as the taps do.
haptics_song_celebration = Celebration
haptics_song_levelup = Level up
haptics_song_heartbeat = Heartbeat
haptics_song_cascade = Cascade

# — scripting (dayscript recorder) —
nav_scripting = Scripting
scripting_caption = Record your taps and navigation into a replayable dayscript. Hit Record, move around and act, then come back and Stop. Edit the YAML to tweak it, Play to replay it, and Copy or Export to keep it.
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
badge_caps_native = This platform draws a badge on the app icon
badge_caps_emulated = The badge is sent, but whether it appears depends on the shell or on the app being installed
badge_caps_none = This platform has no app-badge API
badge_android_note = Android has no API for setting a badge: launchers derive the dot from posted notifications, so use a notification's badge count instead.
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
mix_untitled = Untitled mix
mix_summary = {$name} · {$preset} at {$level}%
voice_search_placeholder = Filter flavors…

# Context-menu demos (menus page): the message-list rows and the media card.
menu_reply = Reply
menu_forward = Forward
menu_archive = Archive
menu_share = Share…
menu_copy_image = Copy Image
menu_save_image = Save to Photos
menu_get_info = Get Info
menus_messages_section = A message list
menus_messages_hint = Each row carries its own menu — the action names the message it came from.
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
menu_record = Record
cmd_star = Star
cmd_unstar = Unstar

# Speech (day-part-speech): the daybridge reference part — one API, a different language per
# platform (docs/bridge.md).
speech_title = Text to speech
speech_caption = One Rust API; the platform's own voice underneath — Swift on Apple, Java on Android, ArkTS on HarmonyOS, JavaScript on the web, C++ on Windows.
speech_phrase = A clear day, with a chance of rain later.
speech_speak = Speak
speech_stop = Stop
speech_support_label = Support here
speech_native = Native
speech_emulated = Emulated (partial)
speech_unsupported = Unsupported on this target

# Support banners (widgets.rs): shown over a demo the target cannot run.
support_missing_here = Not supported on this platform — the demo stays visible so you can see it, but it will not work here.
support_emulated_here = Partly supported here — this demo runs, but not on the platform's own native implementation.
vectors_live_tint = Live tint
vectors_cycle_tint = Cycle

# Layout page (docs/size-classes.md "Row fit policies")
layout_caption = How one row of buttons behaves under each fit policy when the window is too narrow for it.
layout_note = Pick a policy, then add components until the row runs out of room. Clip lets the tail fall offscreen (debug builds log it), Wrap breaks onto new lines at each button's own width, Even columns aligns those lines into a grid, Column stacks below compact width, Scroll keeps one swipeable line. On a desktop, drag the window narrower to watch Column engage.
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

# The Resources page's Weight picker reuses the Text page's weight names (text_weight_*) —
# same three words, same meaning — so only the notes below are new here.
vectors_alias_note = A plain SVG asked for Bold: it has no weight axis, so the alias resolves back to the base glyph rather than drawing nothing.
# The two color wells beside Cycle (docs/colorpicker.md).
vectors_pick_tint = Pick a tint
vectors_tint_idioms = Two color wells, one bound color: the first opens the platform's own chooser, the second opens the panel Day draws itself — the same picker on every target.
