# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Text page's newer cards (src/pages/text.rs). Its other strings are in app.ftl.

# Styled runs: the names of the styles, each drawn in its style.
text_run_code = شفرة
text_run_color = لون
text_run_highlight = تظليل
text_run_larger = أكبر
text_underline_single = مفرد
text_underline_double = مزدوج
text_underline_dotted = منقّط
text_underline_wavy = متموّج

# Relative size: a semantic style and the factor it is scaled by.
text_scale_header = الحجم النسبي
text_scale_of = { $style } × { $scale }

# Presets: two readings that share three named styles.
text_preset_header = الأنماط الجاهزة
text_preset_moon = من الأرض إلى القمر
text_preset_moon_value = 384,400
text_preset_km = كم
text_preset_sun = من الأرض إلى الشمس
text_preset_sun_value = 149.6
text_preset_million_km = مليون كم

# Line limits and alignment share one paragraph.
text_sample_paragraph = يستغرق ضوء الشمس نحو ثماني دقائق ليصل إلى الأرض، وأكثر من أربع ساعات ليصل إلى نبتون، وأكثر من أربع سنوات ليصل إلى أقرب نجم.
text_lines_header = حدود الأسطر
text_lines_one = سطر واحد
text_lines_two = سطران
text_lines_all = بلا حد
text_align_header = المحاذاة
text_align_leading = البداية

# Scripts and direction: the names of the specimens.
text_scripts_header = أنظمة الكتابة والاتجاه
text_scripts_cjk = الصينية واليابانية والكورية
text_scripts_rtl_in_ltr = عربية وعبرية داخل نص لاتيني
text_scripts_ltr_in_rtl = لاتينية داخل نص عربي

# Decorations and font features.
text_decorations_header = الزخارف والأرقام
text_small_caps = أحرف استهلالية صغيرة
text_small_caps_sample = Mission Control, Houston
text_figures = الأرقام

# The reader's text size, and a badge that keeps its size.
text_scale_reading = حجم النص: { $scale }×
text_scale_fixed = حجم ثابت

# Line limits: how many lines, and where the ellipsis goes.
text_lines_count = الأسطر
text_lines_truncate = علامة الحذف
text_trunc_head = البداية
text_trunc_middle = الوسط
text_trunc_tail = النهاية

# Letter spacing and line height.
text_spacing_header = التباعد
text_spacing_letters = الأحرف
text_spacing_lines = الأسطر
text_align_justified = مضبوط

# A row that reads in the direction the picker names.
text_direction = الاتجاه
text_direction_ltr = من اليسار إلى اليمين
text_direction_rtl = من اليمين إلى اليسار
text_direction_first = 1 الإطلاق
text_direction_second = 2 المدار
text_direction_third = 3 الهبوط

# Links and selection.
text_link_handled_label = يعالجه التطبيق
text_selection_none = حدّد جزءًا من النص أعلاه.
text_selection_value = المحدَّد: { $text }

# The Markdown view: a reply streamed in word by word.
text_md_view_header = عرض Markdown
text_md_stream = بثّ ردّ
text_md_copy = نسخ
text_md_copied = نُسخ الرمز
text_md_reply =
    ## فحص المدار

    يرسل المسبار **ثلاث** قراءات في كل دورة:

    - الارتفاع
    - السرعة
    - البطارية

    تُحسب دورته من حجم المدار:

    ```rust
    let period = 2.0 * PI * (a.powi(3) / mu).sqrt();
    ```

    خطّط للدفعة التالية حين تستقر القراءات.
