# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Gestures page's cards (src/pages/gestures.rs). The names of Day's APIs stay as they are
# in every language.

# Drag, pinch and rotate: a picture that follows three gestures at once.
gestures_transform_title = السحب والقرص والتدوير
gestures_transform_subtitle = ثلاث إيماءات تُكتب في تحويل واحد
gestures_transform_stage_label = ساحة فيها صورة لزحل للسحب والقرص والتدوير
gestures_transform_offset = الإزاحة
gestures_transform_scale = المقياس
gestures_transform_angle = الزاوية
gestures_transform_hint_touch = اسحب بإصبع واحد. اقرص ودوّر بإصبعين.
gestures_transform_hint_pointer = اسحب بالفأرة. اقرص ودوّر على لوحة اللمس.
gestures_reset = إعادة الضبط

# Pointer: what on_drag, on_touches, on_tap_at and on_hover report, drawn and in figures.
gestures_pointer_title = المؤشّر
gestures_pointer_subtitle = ما يبلّغ عنه on_drag وon_touches وon_tap_at وon_hover
gestures_pointer_stage_label = ساحة ترسم ما يبلّغ عنه المؤشّر
gestures_pointer_hint_touch = المس واسحب ثم أفلت
gestures_pointer_hint_pointer = مرّر المؤشّر، أو انقر، أو اسحب ثم أفلت
gestures_pointer_phase = المرحلة
gestures_pointer_location = الموضع
gestures_pointer_contacts = نقاط التلامس
gestures_pointer_translation = الإزاحة
gestures_pointer_speed = السرعة
gestures_pointer_speed_value = { $speed } pt/s
gestures_pointer_waiting = بانتظار
gestures_pointer_began = بدء
gestures_pointer_changed = تغيّر
gestures_pointer_ended = انتهاء
gestures_pointer_hover = تمرير
gestures_pointer_tap = نقر
gestures_pointer_one_contact = تبلّغ هذه المنصّة عن نقطة تلامس واحدة في كل مرة.

# Tap, double tap and hold: Day's press gestures on one pad.
gestures_press_title = نقر ونقر مزدوج وضغط مطوّل
gestures_press_subtitle = ‏on_tap وon_double_tap وon_long_press وon_pressed
gestures_press_pad_label = منطقة تعدّ النقرات والضغطات المطوّلة
gestures_press_pad_touch = انقر، أو انقر مرتين، أو اضغط حتى تكتمل الحلقة.
gestures_press_pad_pointer = انقر بالفأرة، أو انقر مرتين، أو اضغط حتى تكتمل الحلقة.
gestures_press_taps = النقرات
gestures_press_doubles = النقرات المزدوجة
gestures_press_holds = الضغطات المطوّلة
gestures_press_derived_note = يستنتج Day الضغط هنا من أحداث السحب، فحرّك قليلًا أثناء الضغط.

# Swipe: a row that drags along one axis inside the page's vertical scroll.
gestures_swipe_title = التمرير الجانبي
gestures_swipe_subtitle = ‏on_drag_along داخل تمرير عمودي
gestures_swipe_row_title = مرّر هذا الصف جانبيًا
gestures_swipe_row_touch = مرّر إلى الجانب. الحركة لأعلى ولأسفل تمرّر الصفحة.
gestures_swipe_row_pointer = اسحب إلى الجانب ثم أفلت.
gestures_swipe_count = مرات التمرير
gestures_swipe_release = سرعة الإفلات

# Hit testing: a dot with a larger tap area, and a button under a canvas with no handlers.
gestures_hit_title = اختبار الإصابة
gestures_hit_subtitle = أين تقع النقرة
gestures_slop_target_label = نقطة بمنطقة نقر أوسع
gestures_slop_caption = نقطة بقياس { $dot } pt في منطقة نقر بقياس { $area } pt
gestures_slop_unsupported = نقطة بقياس { $dot } pt، تُصاب داخل حدودها فقط هنا
gestures_slop_count = { $count ->
    [zero] لا نقرات
    [one] نقرة واحدة
    [two] نقرتان
    [few] { $count } نقرات
   *[other] { $count } نقرة
}
gestures_through_button = تحت اللوحة
gestures_through_caption = في iOS وAndroid لا تستقبل اللوحة الخالية من المعالِجات أي لمسة.
