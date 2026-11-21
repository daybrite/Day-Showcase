app_title = عرض Day
counter_value = { $count ->
    [zero] لا نقرات
    [one] نقرة واحدة
    [two] نقرتان
    [few] { $count } نقرات
   *[other] { $count } نقرة
}
decrement = −
increment = +
name_placeholder = اسمك
flavor_placeholder = اكتب نكهة أو اخترها
flavor_add = إضافة
nav_controls = عناصر التحكّم
ctl_buttons = الأزرار
ctl_press = اضغط
ctl_plain = عادي
ctl_bordered = بإطار
ctl_prominent = بارز
ctl_tinted = ملوّن
ctl_destructive = تدميري
ctl_disabled = معطّل
ctl_link = رابط
ctl_composed = مركّب
ctl_link_text = daybrite.dev
ctl_presses = الضغطات
ctl_switches = المفاتيح والمنزلقات
ctl_toggle = مفتاح
ctl_slider = منزلق
ctl_stepped = منزلق بدرجات
ctl_stepper = عدّاد
ctl_composed_stepper = عدّاد مركّب
ctl_progress = التقدّم
ctl_activity = النشاط
ctl_spinner = مؤشّر الانتظار
ctl_accessibility = إمكانية الوصول
ctl_a11y_heading = عنوان لقارئات الشاشة
ctl_announce = إعلان
ctl_announce_hint = ينطق العدّاد عبر قارئ الشاشة
ctl_announced_times = أُعلن { $count } مرات
ctl_announce_emulated = محاكى (منطقة حيّة)
ctl_text_entry = إدخال النص
ctl_text_field = حقل نص
ctl_search_field = حقل بحث
ctl_combo_box = صندوق مركّب
ctl_combo_note = لا تملك هذه المجموعة عنصر صندوق مركّب، لذا يعرض Day بديلًا هنا.
ctl_text_area = منطقة نص
ctl_notes_placeholder = بضعة أسطر من الملاحظات
ctl_pickers = المنتقيات
ctl_date = التاريخ
ctl_time = الوقت
ctl_color = اللون
ctl_indicators = المؤشّرات والتركيب
ctl_badge = الشارة
ctl_gauge = المقياس
ctl_divider = الفاصل
ctl_state = الحالة
ctl_on = مفعّل
ctl_off = معطّل
nav_menus = القوائم والحوارات
nav_text = النص
nav_battery = البطارية
nav_sensors = المستشعرات
nav_clipboard = الحافظة
nav_media = الوسائط
nav_compose = البطاقات والبيئة
nav_files = الملفات
nav_tabs = علامات التبويب
nav_stack = المكدّس
nav_layout = التخطيط
layout_stacks_section = الصفوف والأعمدة والطبقات
layout_align_label = المحاذاة
layout_align_top = أعلى
layout_align_center = وسط
layout_align_bottom = أسفل
layout_layered = متعدّد الطبقات
layout_scroll_section = التمرير
layout_scroll_top = التمرير إلى الأعلى
layout_scroll_end = التمرير إلى النهاية
layout_size_section = فئات الحجم
layout_size_width = العرض
layout_size_height = الارتفاع
nav_list = القائمة
refresh_tier_native = السحب للتحديث: أصلي
refresh_tier_emulated = السحب للتحديث: محاكى
layout_row = عنصر { $n }
nav_webview = عرض الويب
nav_lottie = Lottie
nav_about = حول

# Sidebar group headers (lib.rs `Group`): the sections the destinations sit under.
group_starred = المفضّلة
group_overview = نظرة عامة
group_controls = عناصر التحكّم
group_layout = التخطيط
group_navigation = التنقّل والواجهة
group_data = البيانات
group_graphics = الرسوميات والوسائط
group_platform = المنصة
group_app = التطبيق والأدوات

picker_selected = المحدد
picker_segmented = مقسّم
picker_menu = قائمة
picker_inline = مضمّن

# --- day::datetime ---
nav_dates = التاريخ والوقت
dates_date_section = التاريخ
dates_time_section = الوقت
dates_composed_section = مركّب
date_compact = مضغوط
time_compact = مضغوط
time_seconds = بالثواني
dates_composed = التاريخ والوقت
date_bounded = ضمن 2026
date_picked = التاريخ المختار
time_picked = الوقت المختار

compose_card_title = سطح قابل لإعادة الاستخدام
compose_card_body = حشوة وخلفية وزوايا مستديرة.
compose_env_value = ملوَّن باللون المميّز المُمرَّر
list_add = أضف ١٠٠
list_caption = الصفوف: { $count }
list_selection_none = لم يتم تحديد أي صف
list_selection = { $count ->
    [one] الصف المحدد: { $rows }
    [two] الصفان المحددان: { $rows }
   *[other] الصفوف المحددة: { $rows }
}
list_clear_selection = مسح التحديد
list_reorder_hint = اسحب الصفوف لإعادة ترتيبها، ومرّر للحذف؛ الصف الأول مثبّت
list_order = الصفوف الأولى: { $rows }
list_shuffle = خلط
list_reset = إعادة الضبط
list_delete = حذف
list_options = خيارات…
list_options_title = خيارات القائمة
list_options_layout = التخطيط
list_options_behavior = السلوك
list_options_gestures = الإيماءات
list_opt_sections = عناوين الأقسام
list_opt_row_height = ارتفاع الصف
list_opt_uniform = موحّد
list_opt_automatic = تلقائي
list_opt_separators = الفواصل
list_opt_default = افتراضي
list_opt_on = تشغيل
list_opt_off = إيقاف
list_opt_content = محتوى الصف
list_opt_plain = تسمية
list_opt_item = عنصر قائمة
list_opt_multi = تحديد متعدد
list_opt_pull = اسحب للتحديث
list_opt_editing = وضع التحرير
list_opt_reorder = اسحب لإعادة الترتيب
list_opt_swipe = إجراءات السحب
list_opt_delete = اسحب للحذف
list_opt_gestures_footer = مع عناوين الأقسام، تُسحب الصفوف داخل قسمها ويتوقف التمرير للحذف.
list_section = الصفوف { $first }–{ $last }
list_row_note = سطر ثانٍ من التفاصيل لهذا الصف
list_row_note_long = تتّسع الصفوف التي يحدّد محتواها ارتفاعَها لتستوعب نصًا أطول، مثل هذا السطر الثالث الذي يلتفّ في النافذة الضيقة.
list_swipe_select = تحديد
list_swipe_deselect = إلغاء التحديد
list_align = موضع الانتقال
list_align_nearest = الأقرب
list_align_start = الأعلى
list_align_center = الوسط
list_align_end = الأسفل
list_insert = إدراج
list_remove = إزالة
list_remove_all = إزالة الكل
list_footer = نهاية القائمة: الصفوف { $count }
list_empty = لا صفوف. «إعادة الضبط» تعيدها.
list_visible = المعروض: { $first }–{ $last }
list_visible_first = المعروض بدءًا من { $first }
list_visible_none = لا صفوف معروضة

nav_model = النموذج
model_caption = الصفوف المعروضة: { $count }
model_cost = المسارات المرصودة: { $paths } · العُقد المخزّنة: { $nodes }
model_hide_done = إخفاء المنجزة
model_selected = المحدد: { $title }
model_selected_none = لا شيء محدد
model_add = إضافة مهمة
model_delete = حذف المحدد
model_undo = تراجع
model_redo = إعادة
model_storage_file = مخزّنة في { $file }
model_storage_memory = مخزّنة في الذاكرة على هذه المنصة
nav_query = الاستعلام
query_caption = { $count } من { $total } مقطوعة مطابقة
query_resident = { $resident } من { $total } صفوف مخزنة مؤقتًا لدى عامل قاعدة البيانات
query_starred = المميزة فقط
query_selected = المحدد: { $title }
query_selected_none = لا شيء محدد
query_fts = نص كامل (FTS5)
query_viewport = داخل الإطار فقط
query_viewport_box = خط العرض { $from }–{ $to }
query_star = تبديل التمييز

webview_url_hint = أدخل عنوان URL
webview_go = انتقال
webview_back = رجوع
webview_forward = تقدّم
webview_stop = إيقاف
webview_reload = إعادة تحميل
webview_js_hint = كود JavaScript لتشغيله داخل الصفحة
webview_js_run = تشغيل »
webview_js_result_hint = تظهر نتيجة JSON هنا
webview_note_iframe = تضمّن هذه المنصة الصفحة داخل إطار iframe، لذا تتعطّل أزرار رجوع وتقدّم وإيقاف، ولا يتابع الحقل الروابط.
webview_tab_remote = بعيد
webview_tab_embedded = مضمّن
webview_embedded_unsupported = لا يتوفر محرك ويب في نسخة التولكيت هذه.
webview_embedded_status_none = لم يُتبع أي رابط خارجي بعد.
webview_embedded_opened = فُتح خارج التطبيق: { $url }
webview_embedded_intercepted = اعتُرض — انتقل التطبيق إلى "{ $route }".

stack_push = ادفع صفحة تفاصيل
stack_detail_title = المستوى { $depth }
stack_item_title = العنصر { $id }
stack_link_42 = فتح العنصر 42 عبر المسار
stack_param_hint = فُتح مع تلميح: {$hint}
tab_one = نظرة عامة
tab_two = التفاصيل
tab_three = الإعدادات
tab_state_note = بدّل التبويب ثم عُد: العدّاد باقٍ.
tab_detail_route = مفتاح المسار
tab_detail_link = رابط عميق
tab_set_badges = إظهار الشارات
tab_set_sounds = تشغيل الأصوات
about_text = تطبيق أصلي متعدد المنصات مبني بـ day.
modal_alert = إظهار تنبيه
modal_confirm = تأكيد
modal_delete = حذف…
modal_sheet = اختر نكهة
modal_prompt = أدخل الاسم
alert_title = إشعار
alert_body = حُفظت تغييراتك.
ok = حسنًا
confirm_title = إنهاء؟
confirm_body = هل أنت متأكد أنك تريد الإنهاء؟
delete_title = حذف العنصر؟
delete_body = لا يمكن التراجع عن هذا.
delete = حذف
flavor_title = اختر نكهة
cancel = إلغاء
vanilla = فانيليا
pistachio = فستق

# Files playground (docs/files.md)
files_placeholder = اكتب شيئًا لحفظه…
files_open = فتح ملف…
files_save = حفظ الملف…
files_opened = فُتح { $name }

# Battery playground (docs/battery.md)
battery_refresh = قراءة بطارية الجهاز
battery_level = المستوى
battery_charging = يُشحن
battery_reading = البطارية: { $percent } · { $state }
battery_reading_none = البطارية: لا توجد واجهة برمجة للبطارية على هذه المنصة
battery_reading_absent = البطارية: لا توجد بطارية في هذا الجهاز

# Sensors playground (docs/sensors.md)
sensor_accelerometer = مقياس التسارع
sensor_gyroscope = الجيروسكوب
sensor_magnetometer = مقياس المغناطيسية
sensor_reading = x { $x } · y { $y } · z { $z } { $unit }
sensor_waiting = في انتظار أول عيّنة…
sensor_unavailable = غير متوفر على هذا الجهاز

# Clipboard playground (docs/clipboard.md)
clipboard_placeholder = اكتب شيئًا لنسخه
clipboard_copy = نسخ
clipboard_paste = لصق
clipboard_idle = الحافظة لم تُمسّ
clipboard_copied = نُسخ إلى الحافظة النظامية
clipboard_copy_failed = فشل النسخ (لا توجد واجهة حافظة هنا)
clipboard_pasted = لُصق من الحافظة النظامية
clipboard_empty = الحافظة فارغة أو تتعذّر قراءتها

# Network playground (docs/network.md)
network_refresh = قراءة الشبكة
network_reading_online = متصل · { $kind } · مقنَّن: { $expensive }
network_reading_offline = غير متصل
network_reading_none = حالة الاتصال غير متاحة

# Media playground (docs/media.md)
media_play = تشغيل
media_pause = إيقاف مؤقت
media_load = تحميل

# --- Localization page (docs/localization.md) ---
nav_localization = التوطين
loc_locale_section = اللغة الحية
loc_live_note = اتجاه التخطيط يُحدَّد عند التشغيل.
loc_current_label = الحالية
loc_reset = إعادة الضبط
loc_numbers_section = الأرقام
loc_dates_section = التواريخ والأوقات
loc_plurals_section = الجموع
loc_sorting_section = الفرز
fmt_number_label = مجمّع
fmt_fraction_label = منزلتان عشريتان
fmt_percent_label = النسبة المئوية
fmt_date_label = تاريخ طويل
fmt_time_label = الوقت
fmt_datetime_label = التاريخ والوقت
fmt_sorted_label = مرتّب
fmt_number = { NUMBER($n) }
fmt_fraction = { NUMBER($n, minimumFractionDigits: 2) }
fmt_percent = { NUMBER($p, style: "percent") }
fmt_date = { DATETIME($d, dateStyle: "long") }
fmt_time = { DATETIME($t, timeStyle: "short") }
fmt_datetime = { DATETIME($dt, dateStyle: "medium", timeStyle: "short") }
plural_items = { $count ->
    [zero] لا عناصر
    [one] عنصر واحد
    [two] عنصران
    [few] { $count } عناصر
    [many] { $count } عنصرًا
   *[other] { $count } عنصر
}

# Text playground (typography)
text_selectable_toggle = قابل للتحديد
text_styles_header = الأنماط
text_weights_header = الأوزان
text_styling_header = عريض ومائل وملوّن
text_custom_header = أحجام مخصّصة
text_fonts_header = الخطوط المضمّنة

# Menus playground
menus_last = آخر إجراء
menus_lifecycle = دورة الحياة
menus_target = انقر بالزر الأيمن أو اضغط مطولًا هنا

# --- day-part-haptics ---
nav_haptics = الاهتزاز اللمسي
haptics_supported_yes = محرّك اللمس متوفّر
haptics_supported_no = لا يوجد محرّك لمس على هذه المنصة
haptics_light = خفيف
haptics_medium = متوسط
haptics_heavy = قوي
haptics_success = نجاح
haptics_warning = تحذير
haptics_error = خطأ
haptics_selection = تحديد
haptics_last = آخر تشغيل
haptics_none = لم يُشغَّل شيء بعد
haptics_last_played = شُغِّل: { $style }

# --- day-part-sound ---
nav_sound = المؤثرات الصوتية
sound_click = نقرة
sound_confirm = تأكيد
sound_error = خطأ
sound_glass = زجاج
sound_card = بطاقة
sound_jingle = لحن قصير
sound_pan_caption = التوزيع المجسّم
sound_left = يسار
sound_center = وسط
sound_right = يمين
sound_volume = مستوى الصوت
sound_last = آخر تشغيل
sound_none = لم يُشغَّل شيء بعد
sound_last_played = شُغِّل: { $clip }
sound_loop = تكرار طنين
sound_loop_volume = مستوى صوت التكرار
sound_looping = قيد التكرار
sound_looping_none = لا شيء يتكرر
sound_looping_clip = يتكرر: { $clip }

# --- day-part-preferences ---
nav_prefs = التفضيلات
prefs_placeholder = ملاحظة للتذكّر
prefs_empty = (لا شيء مخزَّن)
prefs_value_label = القيمة المخزَّنة

# --- bundled resources (§18.3) ---
nav_resources = الموارد
vectors_title = المتجهات
vectors_tints = التلوين
vectors_zoom = التكبير
vectors_weights = الأوزان
vectors_sizes = الأحجام
resources_numbers = ‏numbers.bin: { $len } بايت، byte[100] = { $byte }
resources_greeting = ‏greeting.txt: { $text }

# --- day-part-device ---
nav_deviceinfo = معلومات الجهاز
deviceinfo_model = الطراز: {$value}
deviceinfo_system = النظام: {$name} {$version}
deviceinfo_simulator = محاكٍ: {$value}
deviceinfo_yes = نعم
deviceinfo_no = لا
deviceinfo_refresh = تحديث

# --- Day core spinner ---
activity_on = يدور
activity_off = متوقف

# --- day::searchfield ---
search_clear = مسح


# --- tweaks page (docs/tweaks.md) ---
nav_tweaks = التوليفات
tweaks_stock = قياسي
tweaks_tweaked = مولَّف
tweaks_bezel_title = إطار الزر
tweaks_bezel_caption = AppKit فقط
tweaks_selectable_title = تسمية قابلة للتحديد
tweaks_selectable_text = حدّد هذا النص وانسخه.
tweaks_tooltip_title = تلميح
tweaks_tooltip_caption = ‏AppKit وGTK وAndroid. مرّر المؤشر فوق الزر أو اضغط عليه مطولًا.
tweaks_tooltip_label = حفظ تلقائي
tweaks_tooltip_hint = يُحفظ عملك تلقائيًا.
tweaks_ticks_title = علامات تدرّج المنزلق
tweaks_ticks_caption = ‏AppKit وGTK وAndroid وQt وXAML وArkUI. المنزلق المولَّف يلتصق بعلاماته.
tweaks_ref_title = حيوية NativeRef
tweaks_ref_caption = تركيب المنزلق
tweaks_ref_live = المرجع: حي
tweaks_ref_cleared = المرجع: ممسوح
tweaks_label_title = تعديل تسمية نصية
tweaks_label_caption = ‏AppKit وUIKit. على iOS، خيار «قابل للتحديد» يبدّل الصنف الأصلي.
tweaks_label_sample = هذه التسمية مخفّفة بتعديل أصلي.
tweaks_label_class = الفئة الأصلية: { $class }

# --- merged section pages (design overhaul) ---
nav_canvas = اللوحة
nav_charts = المخططات
nav_system = الجهاز والمستشعرات
nav_network_http = الشبكة وHTTP
network_status_section = الاتصال
nav_notify_badge = الإشعارات والشارة
nav_push = الإشعارات الفورية
nav_speech_haptics = الكلام والصوت واللمس
nav_files_storage = الملفات والتخزين
gauge_value_label = القيمة
canvas_text_title = النص والخطوط
canvas_anchors_title = مراسي النص
canvas_interaction_title = النقر والسحب والتمرير
canvas_interaction_label = المُبلَّغ
canvas_interaction_none = —
canvas_interaction_readout = ‏تمرير { $hover } · نقر { $tap } · سحب { $trail } نقطة
canvas_metrics_cache = ‏measure_text: { $hits } مخزَّنة · { $misses } مقيسة · { $entries } محفوظة
canvas_font_label = الخط
canvas_font_default = النظام
canvas_text_specimen = نص حكيم له سر قاطع وذو شأن عظيم
canvas_text_bold = عريض
canvas_text_italic = مائل
canvas_text_bold_italic = عريض مائل
canvas_image_title = صورة من وحدات بايت
canvas_image_loading = جارٍ فك الترميز…
canvas_image_failed = هذه البايتات ليست صورة تفك هذه المنصة ترميزها
canvas_image_readout = ‏{ $format } · { $width } × { $height } بكسل

# --- data strings localized for the walkthrough locales (option lists, specimen rows) ---
chocolate = شوكولاتة
size_small = صغير
size_medium = متوسط
size_large = كبير
fruit_apple = تفاحة
fruit_banana = موزة
fruit_cherry = كرزة
fruit_date = تمرة
fruit_elderberry = بيلسان
list_row = الصف { $n }
text_style_large_title = عنوان كبير
text_style_title = عنوان
text_style_title2 = عنوان 2
text_style_title3 = عنوان 3
text_style_headline = عنوان رئيسي
text_style_subheadline = عنوان فرعي
text_style_body = متن
text_style_callout = تنويه
text_style_footnote = حاشية
text_style_caption = تسمية توضيحية
text_style_caption2 = تسمية توضيحية 2
text_weight_ultralight = رقيق جدًا
text_weight_light = رقيق
text_weight_regular = عادي
text_weight_medium = متوسط
text_weight_semibold = شبه غامق
text_weight_bold = غامق
text_weight_heavy = ثقيل
text_weight_black = أسود
text_bold = نص غامق
text_italic = نص مائل
text_bolditalic = غامق مائل
text_emphasis_label = توكيد
color_red = أحمر
color_green = أخضر
color_blue = أزرق
color_orange = برتقالي

# القوائم والحوارات (صفحة مدموجة)
menus_appmenu_section = قائمة التطبيق
menus_context_section = القائمة السياقية
menus_dialogs_section = الحوارات
modal_result_label = النتيجة

# صفحة الوسائط
media_player_section = الفيديو

# أقسام صفحة الموارد
resources_image_section = صورة مضمّنة
image_mode_fit = ملاءمة
image_mode_fill = ملء
image_mode_stretch = تمديد
resources_data_section = بيانات مضمّنة

# الإعدادات (docs/settings.md): مجموعتا الفهرس والتشخيص بجانب مجموعة «عام» من Day.
# تسميات نافذة «حول» (الإصدار، الموقع) ومجموعة «عام» تأتي من فهرس Day نفسه.
settings_catalog = الفهرس
settings_catalog_subtitle = الصفحات المفضّلة
settings_starred_count = الصفحات المفضّلة
settings_clear_starred = مسح المفضّلة
settings_clear_starred_confirm = هل تريد إزالة النجمة من كل الصفحات؟
settings_diagnostics = التشخيص
settings_diagnostics_subtitle = عدة الأدوات، النظام، اللغة
about_toolkit = عدة الأدوات
about_id = معرّف التطبيق
about_os = النظام
about_model = الطراز
about_locale = اللغة

# صفحة التركيز (docs/focus.md)
nav_focus = التركيز
focus_group_section = إشارة واحدة، نموذج واحد
focus_name_label = الاسم
focus_email_label = البريد الإلكتروني
focus_city_label = المدينة
focus_current_label = التركيز
focus_next = التركيز التالي
focus_clear = مسح التركيز
focus_bool_section = عنصر واحد، قيمة منطقية واحدة
focus_bool_placeholder = التركيز يصل هنا
focus_focus_btn = تركيز
focus_blur_btn = إزالة التركيز
focus_state_label = الحالة
focus_state_on = مركّز
focus_state_off = غير مركّز
focus_probe_section = ما بعد حقول النص
focus_probe_caption = المنصات اللمسية تخصّص التركيز لإدخال النص.
focus_probe_toggle = مفتاح
focus_probe_slider = شريط تمرير
focus_probe_button = زر

# HTTP fetch demo (docs/http.md): the status readout stays raw "<status> <body>" so the
# walkthrough asserts it byte-for-byte in every locale.
http_idle = لم يُجلب شيء بعد
http_checking = يجري الفحص…
ws_title = WebSocket
ws_connect = اتصال
ws_send = إرسال
ws_close = إغلاق

# Scrolling page (docs/scroll.md): programmatic scroll targets.
scroll_to_top = التمرير إلى الأعلى
scroll_to_bottom = التمرير إلى الأسفل
scroll_to_item = التمرير إلى العنصر 100

# Grid page (docs/grid.md): grid/grid_row from basics to a stress test.
nav_grid = الشبكة
grid_tab_basics = الأساسيات
grid_tab_sizing = الأحجام
grid_tab_spanning = الامتداد
grid_tab_composite = التركيب
grid_tab_stress = اختبار الضغط
grid_col_name = الاسم
grid_col_wins = الانتصارات
grid_col_points = النقاط
grid_sizing_fixed = ثابت 80 نقطة
grid_sizing_content = حسب المحتوى
grid_sizing_short = قصير
grid_sizing_longer = خلية بمحتوى أطول
grid_month_title = مخطط الأسبوع
grid_event_focus = فترة تركيز
grid_event_review = مراجعة
grid_day_n = اليوم { $n }
grid_stress_add = أضف 50 صفًا
grid_stress_bump = زد الخلية الأولى

nav_animation = الرسوم المتحركة
nav_physics = الفيزياء

# صفحة الحركة
anim_scale = المقياس
anim_rotation = الدوران
anim_opacity = العتامة
anim_offset_x = الإزاحة س
anim_offset_y = الإزاحة ص
anim_hue = تدرّج اللون
anim_curve = المنحنى
anim_duration = المدة
anim_randomize = عشوائي!
anim_go_label = حرِّك!
anim_reset_label = إعادة تعيين
anim_curve_spring = نابض
anim_curve_ease_in_out = متدرّج
anim_curve_ease_out = تباطؤ
anim_curve_linear = خطّي
anim_duration_ms = { $ms } مللي ثانية
anim_reduced_motion = تقليل الحركة

# --- صفحة قياس الأداء (اختبار الشبكات من Day-Bench؛ على منصات Apple الأصلية يستضيف منتقٍ مقسّم
#     أيضًا نسخته التوأم المكتوبة بـ SwiftUI عبر day-piece-swiftui، docs/swiftui.md) ---
nav_benchmark = قياس الأداء
bench_parameters = المعلمات
bench_seed = بذرة عشوائية
bench_count = العدد الإجمالي
bench_rows = { $rows ->
    [zero] لا صفوف
    [one] صف واحد
    [two] صفان
    [few] { $rows } صفوف
    [many] { $rows } صفًا
   *[other] { $rows } صف
}
# قوالب %d للوحة SwiftUI المستضافة (عدّادها يعيش في حالة Swift؛ صيغتا مفرد/جمع فقط).
bench_rows_one = صف واحد
bench_rows_other = %d صفوف
bench_tab_day = Day أصلي
bench_tab_swiftui = شبكة SwiftUI

# شريط القوائم وقائمة السياق
menu_file = ملف
menu_open = فتح…
menu_open_recent = فتح الأخيرة
menu_clear_menu = مسح القائمة
menu_save = حفظ
menu_save_as = حفظ باسم…
menu_edit = تحرير
menu_view = عرض
menu_reload = إعادة تحميل
menu_actual_size = الحجم الفعلي
menu_context = السياق
menu_rename = إعادة تسمية
menu_duplicate = تكرار
menu_move_to = نقل إلى
menu_inbox = الوارد
menu_archive = الأرشيف

# صفحة النص: الأحجام والخطوط المرفقة والروابط
text_size_pt = { $pt } نقطة
text_font_pacifico = Pacifico — خط انسيابي
text_font_bungee = BUNGEE — خط عرض ملوّن
text_font_specialelite = Special Elite — مفاتيح آلة كاتبة
text_font_pacifico_lg = Pacifico بحجم 36 نقطة
text_links_section = روابط
text_runs_section = مقاطع منسّقة
text_markdown_section = ماركداون، يُحلَّل مباشرة
text_markdown_sample = يحلّل Day **العريض** و*المائل* و`الشفرة` و~~المشطوب~~ و[الروابط](https://daybrite.dev) أثناء التشغيل. أما الترميز غير المكتمل مثل ** فيبقى كما هو.
text_markdown_opened = فُتح: {$url}
text_baseline_section = المحاذاة على خط الأساس
text_baseline_toggle = محاذاة خطوط الأساس
text_baseline_quantity = الكمية
text_baseline_unit = عنصرًا
text_baseline_total = الإجمالي
text_baseline_currency = دولار
text_baseline_due = تاريخ الاستحقاق
text_link_icons_label = رموز Material على Google Fonts
text_link_mail_label = راسل الفريق

# قسم الملفات: النص الأولي للمحرر
files_initial_content =
    مرحبًا من Day!
    حرِّرني ثم احفظ.

# صفحة الإبلاغ عن الأعطال (day-piece-crash-reporter, docs/break.md)
nav_crash = الإبلاغ عن الأعطال
crash_trigger_section = إحداث عطل
crash_report_section = آخر تقرير عطل
crash_abort = تعطيل (إنهاء)
crash_abort_label = إنهاء أصلي ← SIGABRT
crash_segv = تعطيل (خطأ تجزئة)
crash_segv_label = مرجع فارغ ← SIGSEGV
crash_contained = ذعر (محتوى)
crash_contained_label = يلتقطه day، ويبقى التطبيق
crash_send = إرسال التقرير
crash_clear = مسح التقارير
crash_empty = لا يوجد تقرير عطل بعد. أحدِث عطلاً ثم أعِد التشغيل.

# صفحة حقول النص (docs/textfield.md)
nav_textfields = حقول النص
textfields_password_section = كلمة المرور
textfields_password = كلمة المرور
textfields_password_placeholder = أدخل كلمة مرور
textfields_show_password = إظهار كلمة المرور
textfields_length = الطول
textfields_purpose_section = لوحة المفاتيح والتعبئة التلقائية
textfields_email = البريد الإلكتروني
textfields_phone = الهاتف
textfields_url = الموقع الإلكتروني
textfields_number = الكمية
textfields_decimal = المبلغ
textfields_code = رمز لمرة واحدة
textfields_pin_section = الرمز السري
textfields_pin = الرمز السري
textfields_attrs_section = الخصائص
textfields_reference = المرجع
textfields_read_only = للقراءة فقط
textfields_limited = حتى 12 حرفًا
textfields_message = الرسالة
textfields_message_placeholder = اكتب ثم اضغط إرسال
textfields_sent = المُرسَلة

# صفحة مناطق النص
nav_textareas = تحرير النص
textareas_editor_section = المحرّر
textareas_seed_section = تعبئة بـ
textareas_attrs_section = السمات
textareas_seed_short = قصير
textareas_seed_long = طويل
textareas_seed_markdown = ماركداون
textareas_editable = قابل للتحرير
textareas_selectable = قابل للتحديد
textareas_spellcheck = التدقيق الإملائي
textareas_sample_short = ملاحظة قصيرة. عدّلها، أو أدرج نصًا أطول أو منسّقًا بالأزرار أدناه.

# محرر النص المنسق (docs/texteditor.md)
textareas_styled_section = محرر منسق
textareas_toolbar_section = التنسيق
textareas_export_section = الاستيراد والتصدير
textareas_text_color = لون النص
textareas_align = المحاذاة
textareas_align_natural = طبيعية
textareas_align_center = وسط
textareas_align_trailing = النهاية
textareas_doc_note = ملاحظة
textareas_doc_code = شيفرة
textareas_doc_empty = فارغ
textareas_placeholder = ابدأ الكتابة، أو اختر مستندًا من الأعلى.
textareas_inspector = التحديد { $start }–{ $end } · { $marks } · الحجم ×{ $scale }
textareas_export_markdown = ماركداون
textareas_export_html = HTML
textareas_export_rtf = RTF
textareas_import = إعادة القراءة
textareas_select_word = تحديد كلمة
textareas_no_selection = لا يوجد تحديد
textareas_mark_bold = عريض
textareas_mark_italic = مائل
textareas_mark_underline = تحته خط
textareas_mark_strike = يتوسطه خط
textareas_mark_plain = عادي
sensor_permission = الوصول إلى الحركة
perm_request = طلب
perm_open_settings = فتح الإعدادات
nav_location = الموقع
location_permission = الوصول إلى الموقع
location_start = ابدأ
location_stop = إيقاف
location_waiting = …في انتظار تحديد الموقع
location_unavailable = لا توجد خدمة موقع على هذه المنصة
location_coords = { $lat }, { $lon }
location_altitude = الارتفاع
location_accuracy = الدقة
location_unknown = —
chart_axes = س أحمر · ص فيروزي · ع بنفسجي

# غطاء بملء الشاشة (docs/cover.md).
stack_cover_button = غطاء
cover_title = غطاء بملء الشاشة
cover_dismiss = إغلاق

# نصف ورقة (docs/cover.md): النصف السفلي من النافذة مع بقاء الصفحة ظاهرة تحتها.
stack_half_button = نصف ورقة
half_title = نصف ورقة
half_level = المستوى
half_readout = مستوى الورقة: { $level }

stack_unsaved = تغييرات غير محفوظة
stack_discard_title = تجاهل التغييرات؟
stack_discard_body = لديك تغييرات غير محفوظة في هذه الصفحة.
stack_discard_ok = تجاهل

tab_dynamic_title = علامات تبويب مبنية على البيانات
tab_dynamic_add = إضافة علامة تبويب
tab_dynamic_remove = إزالة علامة تبويب

# تخزين الملفات المحلي (صفحة الخدمات، docs/fs.md).
storage_title = الملفات المحلية
storage_placeholder = نص للتخزين
storage_save = حفظ الملف
storage_load = تحميل
storage_delete = حذف
storage_files_label = الملفات المخزنة
storage_idle = لا شيء مخزّن بعد

prefs_window_title = الإعدادات

# --- Tree page (docs/tree.md) ---
nav_tree = شجرة
tree_expand_all = توسيع الكل
tree_collapse_all = طي الكل
tree_reveal = إظهار tree.rs
tree_add_file = إضافة ملف
tree_multi = تحديد متعدد
tree_lock = قفل docs
tree_hint = اسحب صفًا إلى مجلد لنقله. ما دام القفل مفعّلًا يرفض docs الإفلات.
tree_count = { $count ->
    [one] عقدة واحدة
    [two] عقدتان
    [few] { $count } عقد
   *[other] { $count } عقدة
}
tree_selection_none = لا شيء محدد
tree_selection = { $count ->
    [one] المحدد: { $names }
    [two] المحددان: { $names }
   *[other] المحدد ({ $count }): { $names }
}
tree_move_none = لا نقل بعد
tree_move = نُقل { $name } إلى { $parent } في الموضع { $index }
tree_move_append = نُقل { $name } إلى { $parent }
tree_root = الجذر
tree_ctx_new_file = ملف جديد
tree_ctx_duplicate = تكرار
tree_ctx_move_up = نقل لأعلى
tree_ctx_move_down = نقل لأسفل
tree_ctx_delete = حذف

# --- Toolbars page (docs/toolbars.md) ---
nav_toolbars = أشرطة الأدوات
toolbar_unsupported = لا تملك مجموعة الأدوات هذه شريط أدوات للنافذة.
toolbar_readout_title = ما الذي يفعله شريط الأدوات
toolbar_controls_title = قيادته من هنا
# Item labels: these appear in the toolbar, so they stay short.
toolbar_new = نافذة جديدة
toolbar_menu_open_scripting = فتح صفحة البرمجة النصية
toolbar_menu_copy_script = نسخ البرنامج النصي
toolbar_extra_tooltip = نسخ مجموعة الأدوات والإصدار، لتقرير خلل
toolbar_extra = نسخ المعلومات
toolbar_search_placeholder = بحث
show_source = عرض المصدر
# The live readout.
toolbar_query_label = نص البحث
toolbar_query_empty = (فارغ)
toolbar_star_label = النجمة
toolbar_on = مفعّلة
toolbar_off = معطّلة
toolbar_presses_label = الضغطات
toolbar_presses = { $count ->
    [zero] لا ضغطات
    [one] ضغطة واحدة
    [two] ضغطتان
    [few] { $count } ضغطات
    [many] { $count } ضغطةً
   *[other] { $count } ضغطة
}
toolbar_last_label = آخر إجراء
toolbar_last_none = لا شيء بعد
toolbar_appearance_label = المظهر
toolbar_appearance_ignored = { $mode } (يُتجاهل هنا)
toolbar_transport_label = المُسجِّل
toolbar_transport_idle = خامل
toolbar_transport_recording = جارٍ التسجيل
toolbar_transport_playing = جارٍ التشغيل
toolbar_transport_paused = مُوقَف مؤقتًا
toolbar_last_new = نافذة جديدة
toolbar_last_extra = نُسخت معلومات الإصدار
# The page's own controls.
toolbar_extra_label = إظهار عنصر نسخ المعلومات
toolbar_enabled_label = عرض المصدر مفعّل
toolbar_clear_search = مسح البحث
toolbar_seed_search = تعبئة البحث
toolbar_seed_text = أشرطة الأدوات
# --- day-part-local-notify (docs/notify.md) ---
nav_notify = الإشعارات
notify_caps_post = النشر مدعوم
notify_caps_unsupported = غير متاحة في هذه النسخة. على macOS تتطلب الإشعارات حزمة تطبيق موقّعة (day pack).
notify_caps_schedule_os = تبقى الإشعارات المجدولة بعد إغلاق التطبيق
notify_caps_schedule_process = تُفقد الإشعارات المجدولة إذا أُغلق التطبيق
notify_title_label = العنوان
notify_title_placeholder = عنوان الإشعار
notify_title_default = مرحبًا من Day
notify_body_label = النص
notify_body_placeholder = نص الإشعار
notify_body_default = نُشر من معرض Day.
notify_delay = التأخير
notify_delay_now = الآن
notify_delay_5s = ٥ ثوانٍ
notify_delay_15s = ١٥ ثانية
notify_delay_60s = دقيقة واحدة
notify_importance = الأهمية
notify_importance_low = منخفضة
notify_importance_default = افتراضية
notify_importance_high = عالية
notify_importance_urgent = عاجلة
notify_sound = تشغيل صوت
notify_badge = عدد الشارة
notify_post = نشر
notify_cancel = إلغاء الكل
notify_status_idle = لم يُنشر شيء بعد
notify_status_posted = تم النشر
notify_status_scheduled = تمت الجدولة
notify_status_cancelled = أُلغيت جميع الإشعارات
notify_status_failed = تعذّر النشر
notify_last = آخر نتيجة
notify_perm_granted = تم منح إذن الإشعارات
notify_perm_missing = الإذن غير ممنوح: تُهمَل الإشعارات
notify_perm_request = طلب الإذن
haptics_songs_caption = تسلسلات
haptics_song_celebration = احتفال
haptics_song_levelup = ترقية المستوى
haptics_song_heartbeat = نبض القلب
haptics_song_cascade = تتابع

# --- البرمجة النصية (مسجّل dayscript) ---
nav_scripting = البرمجة النصية
scripting_record = تسجيل
scripting_stop = إيقاف
scripting_copy = نسخ
scripting_export = تصدير
scripting_copied = تم النسخ إلى الحافظة
scripting_delay_label = التأخير بين الخطوات
scripting_delay_unit = ثانية
scripting_saved_label = النصوص المحفوظة
scripting_pick = تحميل نص برمجي محفوظ…
scripting_save = حفظ
scripting_name_hint = نصي البرمجي
scripting_saved = تم الحفظ

# --- app badge (docs/badge.md) ---
nav_badge = شارة التطبيق
badge_caps_native = أيقونة التطبيق تعرض شارة
badge_caps_emulated = تُرسل الشارة، والصدفة تقرّر إظهارها
badge_caps_none = لا توجد واجهة لشارة التطبيق على هذه المنصة
badge_android_note = يستنتج Android الشارة من الإشعارات المنشورة: استخدم عدّاد شارة الإشعار بدلًا من ذلك.
badge_count_label = العدد
badge_minus = −
badge_plus = +
badge_set = تعيين الشارة
badge_clear = مسح
badge_set_text = تعيين النص "beta"
badge_last = آخر إجراء
badge_status_idle = لم يتم التعيين بعد
badge_status_set = تم تعيين الشارة إلى { $count }
badge_status_cleared = تم مسح الشارة
badge_status_text = تم تعيين الشارة إلى "beta"

# The Controls page mixer: one shared state, many editors.
mix_custom = مخصص
voice_search_placeholder = تصفية النكهات…

# Context-menu demos (menus page): the message-list rows and the media card.
menu_reply = رد
menu_forward = إعادة توجيه
menu_share = مشاركة…
menu_copy_image = نسخ الصورة
menu_save_image = حفظ في الصور
menu_get_info = عرض المعلومات
menus_messages_section = قائمة رسائل
menus_photo_section = بطاقة وسائط
msg_subject_one = أرقام الربع، مسودة أولى
msg_subject_two = خطة عطلة نهاية الأسبوع، الجواب النهائي
msg_subject_three = رسومات من الورشة

# The Star command (commands.rs): one label per state, shared by the toolbar, the
# application menu and each navigation row's context menu.
# Save a picture of this window (commands.rs, docs/window-image.md).
cmd_screenshot = لقطة شاشة…
cmd_appearance_light = فاتح
cmd_appearance_system = النظام
cmd_appearance_dark = داكن
cmd_record = تسجيل
cmd_stop_recording = إيقاف
cmd_play = تشغيل
cmd_pause = إيقاف مؤقت
cmd_resume = استئناف
cmd_clear_recording = مسح التسجيل
menu_appearance = المظهر
menu_script = سكربت
cmd_star = تمييز بنجمة
cmd_unstar = إزالة النجمة
cmd_toggle_pseudo_locale = تبديل اللغة الزائفة

# النطق (day-part-speech): مرجع daybridge. واجهة واحدة، ولغة مختلفة لكل منصة.
speech_title = تحويل النص إلى كلام
speech_phrase = يوم صافٍ، مع احتمال هطول المطر لاحقًا.
speech_speak = انطق
speech_stop = أوقف
speech_support_label = الدعم هنا
speech_state_speaking = يتحدث…
speech_state_finished = انتهى
speech_state_stopped = توقف
speech_native = أصلي
speech_emulated = محاكى (جزئي)
speech_unsupported = غير مدعوم على هذا الهدف

# Support banners (widgets.rs): shown over a demo the target cannot run.
support_missing_here = غير مدعوم على هذه المنصة — يبقى العرض ظاهرًا لتراه، لكنه لن يعمل هنا.
support_emulated_here = مدعوم جزئيًا هنا — يعمل هذا العرض، لكن ليس عبر التنفيذ الأصلي للمنصة.
vectors_live_tint = تلوين حي
vectors_cycle_tint = تبديل

# Layout page (docs/size-classes.md "Row fit policies")
layout_note = اختر سياسة، ثم أضف مكوّنات حتى يضيق الصف.
layout_row_section = ملاءمة الصف
layout_fit_label = السياسة
layout_fit_clip = قصّ
layout_fit_wrap = التفاف
layout_fit_wrap_columns = أعمدة متساوية
layout_fit_column = عمود
layout_fit_scroll = تمرير
layout_count_label = المكوّنات
layout_item = عنصر { $n }
layout_item_wide = عنصر { $n } (أعرض)

vectors_alias_note = لا توجد نسخة عريضة: يُرسَم الرمز الأساسي
vectors_pick_tint = اختر لونًا

# صفحة قائمة المحتوى (src/pages/content_list): قائمة العناصر والمحرّر من قالب `day new` نفسه،
# لذا فالمفاتيح أدناه هي مفاتيح القالب كما هي؛ الكود المنسوخ يقرأها بالاسم.
nav_content_list = قائمة المحتوى
nav_items = العناصر
cmd_add = عنصر جديد
cmd_delete = حذف
items_empty = لا توجد عناصر بعد. أضف عنصرك الأول.
item_none = اختر عنصرًا لتعديله.
item_untitled = بلا عنوان
field_name = الاسم
field_name_hint = سمِّ هذا العنصر…
field_done = منجز

# صفحة المؤشّرات (day/docs/cursor.md): المزخرف `.cursor()`، شكلًا شكلًا.
# صفحة التقسيم (day/docs/split.md): لوحان وفاصل وحالة واحدة تُقرأ من الجانبين.
nav_split = التقسيم
split_support_title = في هذه المجموعة
split_support_label = يرسم الفاصل
split_native = فاصل المجموعة نفسها
split_composed = Day (لا فاصل لهذه المجموعة)
split_hint = اسحب الفاصل أو حرّك شريط الحصة.
split_arrangement_title = الترتيب
split_stacked = متراكبان
split_share = حصة النموذج
split_form_title = الأشكال
split_caption = تعليق
split_caption_placeholder = يُرسم فوق الأشكال
split_count = العدد
split_radius = نصف القطر
split_hue = الصبغة
split_filled = مملوءة
split_shape = الشكل
split_shape_circle = دائرة
split_shape_square = مربع
split_shape_diamond = معيّن
split_readout = { $count } أشكال بنصف قطر { $radius }

nav_cursors = المؤشّرات
cursors_support_title = في مجموعة الأدوات هذه
cursors_support_label = الدعم هنا
cursors_native = أصلي
cursors_emulated = محاكى (أقرب الأشكال)
cursors_unsupported = غير مدعوم على هذا الهدف
cursors_hint = حرّك المؤشّر فوق بلاطة.
cursors_shapes_title = أشكال المؤشّر
cursors_drag_title = السحب والإفلات
cursors_resize_title = تغيير الحجم والتكبير
cursors_busy_title = مشغول
cursors_reactive_title = صندوق واحد، أشكال كثيرة
cursors_reactive_pick = الشكل
cursors_nested_title = الأقرب من الأسلاف يفوز
cursors_nested_outer = علامة تصويب هنا
cursors_nested_inner = يد هنا
cursors_native_title = الخاصّة بمجموعة الأدوات هذه
cursors_native_none = لا تسمّي مجموعة الأدوات هذه أشكالًا خارج الطقم المشترك.

# أسباب الأذونات: النص الذي يعرضه iOS وHarmonyOS عندما يطلب التطبيق الإذن.
permission_camera = معاينة الكاميرا والتقاط صورة في صفحة الكاميرا.
permission_location_when_in_use = عرض إحداثياتك مباشرةً في صفحة الجهاز والمستشعرات.
permission_motion = عرض قراءات مقياس التسارع والجيروسكوب والمغناطيسية مباشرةً في صفحة الجهاز والمستشعرات.
permission_NSBluetoothAlwaysUsageDescription = عرض كيفية طلب Day لبلوتوث — لا يتصل العرض التوضيحي بأي جهاز أبدًا.

# Camera (src/pages/camera.rs): the viewfinder, its permission, and the last photo.
nav_camera = الكاميرا
camera_permission_section = الإذن
camera_permission_label = الوصول إلى الكاميرا
camera_viewfinder_section = المعاينة
camera_take_photo = التقاط صورة
camera_flip = تبديل الكاميرا
camera_state_label = الكاميرا
camera_last_photo_section = آخر صورة
camera_photo_label = الصورة
camera_no_photo = لا شيء بعد
camera_photo_size = { $width } × { $height } · { $name }

list_activated = آخر صف تم تفعيله: { $row } (0 = لا يوجد)

ctl_icon_label = أيقونة وتسمية
ctl_icon_only = أيقونة فقط
ctl_image = صورة مضمّنة

nav_drag_drop = السحب والإفلات
dnd_hint = اسحب بلاطة إلى بلاطة أخرى أو إلى نافذة Showcase أخرى. الدوائر التي عليها X ترفض الإفلات.
dnd_ready = جاهز لاستقبال عنصر.
dnd_empty = أفلت هنا
dnd_new_window = نافذة جديدة
dnd_choose_file = اختر ملفًا…
dnd_accept = قبول الإفلات

dnd_data = { $mime }: { $count } بايت

files_reopen = إعادة فتح آخر ملف
files_bookmark_failed = تعذرت استعادة الوصول إلى الملف المحفوظ. حدده مجددًا باستخدام فتح.

dnd_disallowed = إضافة مناطق محظورة

# Native display frame demo
frame_hint = انقر لقذف الكرات. اسحب كرة لقذفها.
frame_bounce = ارتداد
frame_shuffle = خلط
frame_pause = إيقاف مؤقت
frame_waiting = بانتظار إطار
frame_running = تتحرك
frame_paused = متوقفة مؤقتًا
frame_resting = ساكنة
frame_balls_label = الكرات
frame_fps = إطار/ث · الأدنى { $min } · الأقصى { $max } · المتوسط { $avg }

commands_title = أوامر قابلة لإعادة الاستخدام
commands_add = إضافة واحد

# The Physics page (src/pages/physics.rs): the ball mass and planetary gravity sliders, and the
# bodies the gravity readout names.
physics_gravity_label = الجاذبية
physics_gravity = { $value } م/ث²
physics_gravity_named = { $value } م/ث² ({ $body })
physics_mass_label = الكتلة
physics_mass = { $value } ميغاطن
physics_elasticity_label = المرونة
physics_elasticity = { $value }٪
physics_fullscreen = ملء الشاشة
physics_exit_fullscreen = الخروج من ملء الشاشة
physics_zero_g = انعدام الجاذبية
physics_moon = القمر
physics_mars = المريخ
physics_earth = الأرض
physics_jupiter = المشتري

# HTTP sessions, interception and payload statistics.
net_transport = النقل
net_provider = موفر HTTP
net_native = الأصلي
net_reqwest = reqwest
net_native_only = يستخدم هذا الهدف الموفر الأصلي.
net_provider_note = يسري على الطلبات والمقابس الجديدة.
net_simulation = محاكاة الشبكة
net_intercept = اعتراض الطلبات
net_speed = سرعة النقل
net_speed_value = { $speed } كيبيبايت/ث لكل عملية نقل
net_failures = معدل الفشل
net_failure_value = { $rate }٪ من الطلبات
net_pause = إيقاف التسليم مؤقتًا
net_offline = غير متصل
net_method = الطريقة
net_url = الرابط
net_body = محتوى الطلب
net_cancel = إلغاء
net_cancelled = أُلغي
net_download_preset = إعداد تنزيل
net_upload_preset = إعداد رفع
net_received = استُلم { $bytes } بايت
net_error = فشل الطلب: { $error }
net_request = طلب HTTP
net_ws_note = أدخل رابط WebSocket مباشرًا، أو فعّل الاعتراض للصدى المحاكى.
net_message = الرسالة
net_connected = متصل
net_closed = مغلق
net_real_down = تنزيل · حقيقي
net_real_up = رفع · حقيقي
net_sim_down = تنزيل · محاكى
net_sim_up = رفع · محاكى
net_seconds = ثوانٍ
net_rate = كيبيبايت/ث
net_series = حركة البيانات
net_totals = أُرسل { $sent } بايت · استُلم { $received } بايت · نشط { $active } · فشل { $failed }
net_average = متوسط الجلسة: رفع { $upload } كيبيبايت/ث · تنزيل { $download } كيبيبايت/ث
net_statistics = حركة البيانات المباشرة
net_send_request = إرسال الطلب
net_options = خيارات الطلب
net_headers = الترويسات (JSON)
net_timeout = مهلة الخمول
net_timeout_value = { $seconds } ثانية

# SI units used by the motion-sensor readouts.
sensor_unit_acceleration = m/s²
sensor_unit_rotation = rad/s
sensor_unit_magnetic = µT

sensor_live = قراءات مباشرة
sensor_compass = بوصلة مغناطيسية
sensor_compass_hint = أمسك الهاتف بشكل مسطّح. يشير الأحمر إلى الشمال المغناطيسي.
sensor_north = ش
sensor_east = ق
sensor_south = ج
sensor_west = غ
sensor_heading = الاتجاه المغناطيسي: { $degrees }°

sensor_calibrated = معاير
sensor_tilt = مستوى التسارع X/Y
sensor_tilt_hint = تتبع الفقاعة التسارع على X/Y، وعند الحافة 1 g.
sensor_rotation = مؤشرات سرعة الدوران
sensor_rotation_hint = سرعة الدوران لكل محور، و±π rad/s عند الطرفين.
sensor_uncalibrated_hint = غير معايَر: قد لا تشير الإبرة إلى الشمال.
sensor_axis_x = X
sensor_axis_y = Y
sensor_axis_z = Z

# --- الأمان: سلسلة المفاتيح وطلب التحقق البيومتري (docs/keychain.md, docs/biometrics.md) ---
nav_security = الأمان
keychain_title = سلسلة المفاتيح
keychain_support_label = المخزن هنا
keychain_backend_label = المحرّك
keychain_key_label = المفتاح
keychain_key_placeholder = showcase.session
keychain_secret_label = السر
keychain_secret_placeholder = شيء يستحق الحماية
keychain_protection_label = الحماية
keychain_level_unlocked = عند فتح القفل
keychain_level_first_unlock = بعد أول فتح للقفل
keychain_level_presence = حضور المستخدم
keychain_save = حفظ
keychain_read = قراءة
keychain_read_presence = قراءة مع التحقق
keychain_remove = إزالة
keychain_idle = لم يُحفظ شيء بعد
keychain_saved = تم حفظ { $key }
keychain_read_value = تمت القراءة: { $value }
keychain_removed = تمت إزالة { $key }
keychain_entries_label = المفاتيح المحفوظة
keychain_entries_none = لا شيء
keychain_presence_reason = قراءة السر الذي حفظته صفحة الأمان.
biometrics_title = القياسات البيومترية
biometrics_support_label = الدعم هنا
biometrics_availability_label = التوفر
biometrics_biometry_label = المستشعر
biometrics_reason_placeholder = أكّد هويتك للمتابعة.
biometrics_credential_label = السماح برمز الجهاز
biometrics_authenticate = مصادقة
biometrics_cancel = إلغاء
biometrics_idle = لم يُطلب بعد
biometrics_prompting = في انتظار الطلب…
biometrics_verified = تم التحقق ({ $method })
permission_biometrics = أكّد هويتك في صفحة الأمان.

# --- تسجيل الدخول: day-part-auth (متصفح المصادقة، OpenID Connect، مفاتيح المرور، Apple، Google) ---
nav_signin = تسجيل الدخول
signin_oidc_title = OpenID Connect
signin_provider_demo = مزوّد العرض (مدمج)
signin_provider_google = Google
signin_provider_apple = Apple (تدفق الويب)
signin_provider_custom = أي مُصدِر (استكشاف)
signin_issuer_placeholder = عنوان المُصدِر، يُستكشف عند تسجيل الدخول
signin_client_id_placeholder = معرّف العميل كما سُجّل عند المزوّد
passkeys_rp_placeholder = الطرف المعتمِد (نطاق مرتبط بهذا التطبيق)
signin_sign_in = تسجيل الدخول
signin_sign_in_silent = تسجيل دخول صامت
signin_refresh = تحديث
signin_userinfo = Userinfo
signin_revoke = إبطال
signin_sign_out = تسجيل الخروج
signin_idle = غير مسجّل الدخول
signin_working = في انتظار المزوّد…
signin_signed_in = تم تسجيل الدخول باسم { $name } ({ $email })، الموضوع { $sub }
signin_tokens = رمز الوصول صالح لمدة { $secs } ث · النطاق { $scope } · { $refresh }
signin_refresh_yes = صدر رمز تحديث
signin_refresh_no = لا رمز تحديث
signin_need_client_id = الصق أولًا معرّف العميل الذي سجّلته عند المزوّد.
signin_need_discovery = حدّد أولًا المُصدِر ومعرّف العميل؛ يجري الاستكشاف عند تسجيل الدخول.
signin_silent_demo_only = تسجيل الدخول الصامت خاص بمزوّد العرض؛ أما الآخرون فيحتاجون صفحة الموافقة.
signin_nothing_to_refresh = سجّل الدخول أولًا: لا يوجد رمز تحديث لمبادلته.
signin_refreshed = تم التحديث: صدر رمز وصول جديد
signin_userinfo_ok = أجاب Userinfo عن { $sub }
signin_revoked = تم إبطال رمز الوصول
signin_signed_out = تم تسجيل الخروج
passkeys_title = مفاتيح المرور
passkeys_available_yes = مُصادِق المنصة متاح
passkeys_available_no = لا مُصادِق منصة على هذا الجهاز
passkeys_create = إنشاء مفتاح مرور
passkeys_get = تسجيل الدخول بمفتاح مرور
passkeys_idle = لا بيانات اعتماد بعد
passkeys_prompting = في انتظار المُصادِق…
passkeys_created = أُنشئت بيانات الاعتماد { $id }
passkeys_asserted = تم تسجيل الدخول ببيانات الاعتماد { $id }
apple_title = تسجيل الدخول باستخدام Apple
apple_sign_in = تسجيل الدخول باستخدام Apple
apple_signed_in = مستخدم Apple { $user }
google_title = Google
google_client_placeholder = معرّف عميل الخادم
google_sign_in = تسجيل الدخول باستخدام Google
google_signed_in = حساب Google { $id }

# Numeric notification count.
badge_count = { $count }

# Map (src/pages/map.rs): day-piece-map over the built-in sample island or a live OpenFreeMap
# style, the camera calls on buttons, and a readout of what the piece reports.
nav_map = الخريطة
map_source_sample = جزيرة تجريبية
map_source_liberty = OpenFreeMap Liberty
map_source_positron = OpenFreeMap Positron
map_source_bright = OpenFreeMap Bright
map_source_fiord = OpenFreeMap Fiord
map_source_dark = OpenFreeMap Dark
map_source_colorful = VersaTiles Colorful
map_source_colorful_dark = VersaTiles Colorful Dark
map_fly = حلّق
map_fit = ملاءمة
map_rotate = تدوير
map_north = الشمال
map_sight = { $sight ->
        [center] وسط الجزيرة
        [coast] الساحل الشرقي
        [trafalgar] ميدان ترافالغار
        [here] موقعك
       *[bridge] جسر البرج
    }
map_readout = { $status ->
        [ready] جاهزة
        [failed] تعذّر التحميل: { $reason }
       *[loading] جارٍ التحميل
    } · { $lat }، { $lng } · التكبير { $zoom } · { $bearing ->
        [0] الشمال للأعلى
       *[other] الاتجاه { $bearing }°
    }
map_tap_prompt = انقر على الخريطة
map_tap_marker = علامة: { $sight ->
        [center] وسط الجزيرة
        [coast] الساحل الشرقي
        [trafalgar] ميدان ترافالغار
        [here] موقعك
       *[bridge] جسر البرج
    }
map_tap_feature = { $layer }: { $name }
map_tap_layer = { $layer }
map_tap_nothing = لا شيء هنا
map_locate = الموقع الحالي
map_locating = …جارٍ تحديد موقعك
map_locate_failed = { $why ->
        [denied] الوصول إلى الموقع متوقف لهذا التطبيق. فعّله من الإعدادات.
        [off] خدمات الموقع متوقفة.
        [timeout] لم يُحدَّد الموقع بعد. حاول مرة أخرى.
        [unavailable] الموقع غير متاح على هذا الجهاز.
       *[other] تعذّر العثور على موقعك.
    }

# Interactive layout sizing and translation.
layout_reactive_title = تحجيم تفاعلي
layout_reactive_width = العرض
layout_reactive_height = الارتفاع
layout_reactive_horizontal = الإزاحة الأفقية
layout_reactive_vertical = الإزاحة الرأسية
layout_reactive_reset = إعادة الضبط
layout_reactive_points = { NUMBER($value, maximumFractionDigits: 0) } نقطة
layout_reactive_shape = شكل بعرض { NUMBER($width, maximumFractionDigits: 0) } وارتفاع { NUMBER($height, maximumFractionDigits: 0) } نقطة

push_caption = أرسل إشعارًا فوريًا تجريبيًا إلى هذا الجهاز عبر خدمة ترحيل Day.
push_unavailable = الإشعارات الفورية غير متاحة في هذا الإصدار.
push_send_now = إرسال الآن
push_send_later = { $seconds ->
    [one] إرسال بعد ثانية واحدة
    [two] إرسال بعد ثانيتين
    [few] إرسال بعد { $seconds } ثوانٍ
   *[other] إرسال بعد { $seconds } ثانية
}
push_hint = الإرسال الأول يطلب الإذن.
push_sending = جارٍ الإرسال…
push_requested = بانتظار رمز الجهاز من خدمة الإشعارات…
push_no_token = لم تقدّم خدمة الإشعارات رمزًا للجهاز. حاول مرة أخرى بعد قليل.
push_denied = الإشعارات متوقفة لهذا التطبيق. اسمح بها من الإعدادات، ثم أعد الإرسال.
push_too_soon = تسمح خدمة الترحيل باختبار واحد كل 30 ثانية. انتظر قليلًا، ثم أعد الإرسال.
push_network_busy = سجّلت خدمة الترحيل أجهزة كثيرة من هذه الشبكة خلال الساعة الماضية. أعد المحاولة لاحقًا.
push_accepted = تم الإرسال. سيصل بعد لحظات.
push_scheduled = { $seconds ->
    [one] يصل بعد ثانية واحدة. غادر التطبيق لتراه في الخلفية.
    [two] يصل بعد ثانيتين. غادر التطبيق لتراه في الخلفية.
    [few] يصل بعد { $seconds } ثوانٍ. غادر التطبيق لتراه في الخلفية.
   *[other] يصل بعد { $seconds } ثانية. غادر التطبيق لتراه في الخلفية.
}
push_received = وصل والتطبيق مفتوح: { $title }
push_opened = فُتح من الإشعار: { $title }
push_advanced = خيارات متقدمة
push_token = رمز الجهاز
push_token_apns = تم استلامه من APNs
push_token_fcm = تم استلامه من FCM
push_token_none = لم يُستلم بعد
push_relay = التسجيل في خدمة الترحيل
push_relay_none = غير مسجّل
push_sandbox = استخدام بيئة اختبار APNs (إصدارات التطوير)
push_waiting = لم يصل أي إشعار بعد
push_remove = إزالة الجهاز من خدمة الترحيل
push_removed = تمت إزالة الجهاز من خدمة الترحيل
push_error = فشل الإشعار الفوري: { $error }

# أمثلة المشاركة والقوائم المجمّعة والرجوع على مستوى التطبيق.
list_opt_inset_grouped = مجموعات بهوامش
list_opt_inset_hint = صفوف بهوامش على macOS ومجموعات مستديرة على iOS. تحتفظ المنصات الأخرى بقوائمها العادية.
share_demo_title = المشاركة
share_demo_fallback = لا تعرض هذه المنصة إجراءات مشاركة مخصّصة. استخدم الأزرار أدناه.
share_demo_open = مشاركة موقع Day…
share_demo_remember = تذكّر الرابط
share_demo_clear = مسح النتيجة
share_demo_remembered = تم اختيار إجراء تذكّر الرابط
share_demo_cleared = تم مسح نتيجة المشاركة
share_demo_failed = تعذّر فتح قائمة المشاركة.
share_demo_receive_hint = شارك نصًا أو رابطًا إلى Day Showcase من تطبيق آخر.
share_demo_none = لم يتم استلام شيء بعد.
system_back_status = رجوع التطبيق: المتبقي { $remaining }، المنفّذ { $handled }
system_back_press = رجوع
system_back_stop = تعطيل
system_back_hint = الصفحات المدفوعة والأغطية تعالج الرجوع قبل التطبيق.
system_back_arm = تفعيل 3 ضغطات رجوع

# Map provider selection and setup.
map_provider_osm = OpenStreetMap
map_provider_apple = Apple Maps
map_provider_google = Google Maps
map_missing_api_key = اضبط مفتاح واجهة برمجة تطبيقات خرائط Google لتطبيق Android هذا.
map_play_services_unavailable = تتطلب خرائط Google خدمات Google Play.
map_load_failed = تعذر تحميل الخريطة. تحقق من الاتصال وإعدادات مزود الخدمة.

resources_missing = المورد المضمّن غير متاح: { $name }

# The gallery pages added with the card layout, and the line under every page's title.
nav_styles = الأنماط
nav_images = الصور
nav_gestures = الإيماءات
detail_controls = أزرار ومفاتيح ومنزلقات وعدّادات ومنتقيات ومؤشّرات تقدّم
detail_dates = منتقيات التاريخ والوقت، بتقويم المنصة
detail_focus = تركيز لوحة المفاتيح كحالة في التطبيق
detail_text = خطوط ومقاطع منسّقة وماركداون وتحديد واقتطاع
detail_text_areas = تحرير متعدد الأسطر وأوضاع الشيفرة وماركداون حيّ
detail_text_fields = إدخال من سطر واحد ولوحات مفاتيح وإدخال آمن وبحث
detail_toolbars = أوامر النافذة والصفحة في أشرطة المنصة
detail_localization = أربع لغات وصيغ الجمع وتخطيط من اليمين إلى اليسار
detail_canvas = مسارات وتدرّجات لونية ونصوص وصور تُرسم في كل إطار
detail_styles = زوايا وحدود وظلال وقصّ وتحويلات
detail_images = أوضاع الملاءمة والزوايا والصور البعيدة والعناصر النائبة والتلاشي
detail_gestures = سحب وقرص وتدوير وضغط وتمرير للمؤشّر على أي عرض
detail_charts = قواعد للعلامات: خطوط وأعمدة ومساحات وتحديدات
detail_animation = تحريك المنصّة والقيم المتدرّجة ومنحنيات التوقيت والنوابض
detail_physics = كرات ونابض على ساعة الإطارات
detail_benchmark = رقعة من العروض الحيّة، يُقاس زمنها مقابل SwiftUI
detail_grid = أعمدة ثابتة وامتدادات ومحاذاة وأحجام
detail_layout = صفوف وأعمدة وطبقات والتفاف وفئات الحجم
detail_split = ألواح قابلة لتغيير الحجم بفواصل المنصة نفسها
detail_list = صفوف معاد استخدامها وأقسام وسحب جانبي وإعادة ترتيب وتحديد
detail_tree = مخططات هرمية تتوسّع وتُحدَّد ويُعاد ترتيبها وتُحمَّل عند الحاجة
detail_drag_drop = نصوص وملفات وصور تُسحب بين العروض والتطبيقات
detail_model = مخازن محدّدة الأنواع وقيم مشتقّة وتراجع
detail_query = قاعدة بيانات تُستعلم مباشرةً، مع بحث وفرز
detail_tabs = أشرطة تبويب وصفحات مقسّمة، لكلٍّ منها حالتها
detail_stack = صفحات مدفوعة، ومسار تملكه، وإيماءة الرجوع
detail_content_list = قائمة وعناصرها وتفاصيل في ثلاثة أعمدة
detail_media = مشغّل فيديو أصلي بعناصر تحكّمه في التشغيل
detail_lottie = رسوم متحركة متجهية تُشغَّل وتُوقَف مؤقتًا ويُتنقَّل فيها
detail_map = بلاطات متجهية على اللوحة بجانب خرائط المنصة
detail_web_view = موقع مضمَّن والويب الحي، مع JavaScript في الاتجاهين
detail_menus = قوائم وقوائم سياق وتنبيهات وأوراق وطلبات إدخال
detail_system = البطارية والمستشعرات والموقع ومعلومات الجهاز والمظهر
detail_network = طلبات ورفع وتنزيل ووسطاء واتصال
detail_notify = الإشعارات المحلية وشارة التطبيق
detail_push = إشعارات عن بُعد عبر خدمة ترحيل يمكنك تجربتها
detail_speech = الكلام والمؤثرات الصوتية والاهتزاز اللمسي
detail_camera = معاينة حيّة والتقاط والصورة الناتجة
detail_cursors = كل أشكال المؤشّر التي تتيحها المنصة
detail_files = الحافظة والتفضيلات ومنتقيات الملفات وتخزين التطبيق
detail_security = سلسلة المفاتيح والقياسات البيومترية
detail_sign_in = تسجيل الدخول عبر Apple أو Google أو مفتاح مرور أو أي مزوّد OpenID
detail_scripting = النصوص البرمجية التي تشغّل هذا التطبيق وتلتقط صور شاشته
detail_resources = صور مضمَّنة ومحارف متجهية وخطوط وملفات بيانات
detail_tweaks = تعديلات على العروض الأصلية خاصة بكل مجموعة أدوات
detail_crash_reporting = أعطال تُلتقط ويُبلَّغ عنها عند التشغيل التالي
detail_about = ما هو Day، وما يعرضه هذا التطبيق
detail_settings = المظهر واللغة وتفضيلات التطبيق
