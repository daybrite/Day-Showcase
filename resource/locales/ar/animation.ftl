# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Animation page (src/pages/animation.rs). The names of Day's APIs stay as they are in
# every language.

# Easing curves: one curve of each Easing variant, named after the variant.
animation_easing_title = منحنيات التوقيت
animation_easing_subtitle = كل أنواع Easing على ساعة واحدة
animation_easing_play = تشغيل
animation_curve_linear = خطّي
animation_curve_bezier = بيزييه
animation_curve_power = أُسّ
animation_curve_sine = جيبي
animation_curve_circle = دائري
animation_curve_exponential = أسّي
animation_curve_back = تراجع
animation_curve_elastic = مطاطي
animation_curve_bounce = ارتداد

# Platform animator: the playground. Its controls' labels are the anim_* keys in app.ftl.
animation_play_title = محرّك المنصّة
animation_play_subtitle = ست خصائص في with_animation واحدة

# Animated values: a planet that glides between four sets of figures.
animation_tween_title = قيم Animated
animation_tween_subtitle = الحجم والمسافة واللون تنساب معًا
animation_tween_mercury = عطارد
animation_tween_earth = الأرض
animation_tween_jupiter = المشتري
animation_tween_neptune = نبتون
animation_tween_diameter = القطر
animation_tween_distance = البعد عن الشمس
animation_tween_km = { $km } كم
animation_tween_au = { $au } وحدة فلكية

# Interpolation: one value that a tile's place, size, turn and color are mapped from.
animation_interp_title = الاستيفاء
animation_interp_subtitle = قيمة واحدة تحرّك الموضع والحجم والدوران واللون
animation_interp_value = القيمة
animation_interp_release = سرعة الإفلات
animation_interp_speed = { $speed } نقطة/ث
animation_interp_flip = اقلب

# Spring: one spring set by three sliders, plotted and run.
animation_springs_title = نابض
animation_springs_subtitle = اسحب المقبض ثم أفلته
animation_springs_stiffness = الصلابة
animation_springs_damping = التخميد
animation_springs_mass = الكتلة
animation_springs_release = أطلِق
animation_springs_ratio = ζ { $ratio }

# Choreography: bars moved by composed steps.
animation_compose_title = تصميم الحركة
animation_compose_subtitle = خطوات مركّبة على ساعة الإطارات
animation_compose_sequence = تسلسل
animation_compose_parallel = تزامن
animation_compose_stagger = تعاقب
animation_compose_repeat = تكرار
animation_compose_play = ابدأ
animation_compose_stop = إيقاف
animation_compose_reset = إعادة تعيين

# Implicit animation: a heading and a button over a scrolling log. A sol is a day on Mars.
animation_header_title = تحريك ضمني
animation_header_subtitle = عنوان وزرّ يتبعان التمرير
animation_header_row = اليوم المريخي { $n }
animation_header_top = الأعلى
animation_header_heading = سجلّ المهمة
animation_header_offset = { $y } نقطة
animation_header_untracked = لا يُبلَّغ عن الإزاحة

# Reflow: keyed tiles whose layout changes run inside with_animation.
animation_layout_title = إعادة الترتيب
animation_layout_subtitle = تغييرات التخطيط داخل with_animation
animation_layout_tiles = البلاطات
animation_layout_size = الحجم
animation_layout_small = صغير
animation_layout_medium = متوسط
animation_layout_large = كبير
animation_layout_shuffle = خلط
