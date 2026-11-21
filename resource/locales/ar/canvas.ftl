# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Canvas page (src/pages/canvas.rs).

canvas_dial_title = حمل التوربين 3
canvas_dial_subtitle = إبرة وقوس تحرّكهما قيمة واحدة بنابض
canvas_dial_unit = ٪ من الحمل
canvas_dial_idle = خامل
canvas_dial_cruise = إبحار
canvas_dial_peak = ذروة
canvas_dial_zone = النطاق
canvas_dial_zone_redline = الخط الأحمر
canvas_dial_zone_high = مرتفع
canvas_dial_zone_nominal = اعتيادي
canvas_dial_spool = سرعة الدوران
canvas_dial_spool_value = { $rpm } rpm
canvas_dial_limit = الحد

canvas_clock_title = التحويلات
canvas_clock_subtitle = translate · rotate · save / restore
canvas_clock_ticks = العلامات
canvas_clock_ticks_note = 60 تدويرًا
canvas_clock_hands = العقارب
canvas_clock_hands_note = save, restore
canvas_clock_second = عقرب الثواني
canvas_clock_second_note = اشتراك في FrameClock

canvas_svg_title = مسارات SVG
canvas_svg_subtitle = بيانات المسار، يحلّلها build_path! وقت التصريف
canvas_svg_heart = قلب
canvas_svg_star = نجمة
canvas_svg_bolt = برق
canvas_svg_check = علامة صح

canvas_compositing_title = التركيب
canvas_compositing_subtitle = قصّ، ظل، تفريغ بالمحو، أنماط المزج
canvas_compositing_image = صورة مقصوصة
canvas_compositing_cutout = تفريغ
canvas_compositing_radial = تدرّج شعاعي
canvas_compositing_blend = أنماط المزج
canvas_compositing_blend_code = Draw::blended
canvas_blend_normal = عادي
canvas_blend_multiply = ضرب
canvas_blend_screen = تفتيح
canvas_blend_add = جمع

canvas_sketch_title = رسم حر
canvas_sketch_subtitle = اسحب ← ارسم عبر on_drag
canvas_sketch_clear = مسح
canvas_sketch_empty = ارسم هنا
canvas_sketch_strokes = الخطوط
canvas_sketch_count = { $strokes } · { $points } نقطة

canvas_text_subtitle = ما يبلّغ به measure_text، مع أي عائلة خطوط
canvas_anchors_subtitle = اثنا عشر موضعًا لـ TextAnchor على علامة تصويب
canvas_interaction_subtitle = on_hover · on_tap_at · on_drag
canvas_image_subtitle = ‏decode_image ثم Draw::image
