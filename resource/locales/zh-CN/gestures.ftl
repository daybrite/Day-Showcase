# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Gestures page's cards (src/pages/gestures.rs). The names of Day's APIs stay as they are
# in every language.

# Drag, pinch and rotate: a picture that follows three gestures at once.
gestures_transform_title = 拖动、捏合与旋转
gestures_transform_subtitle = 三种手势写入同一个变换
gestures_transform_stage_label = 舞台上有一张可拖动、捏合和旋转的土星图片
gestures_transform_offset = 偏移
gestures_transform_scale = 缩放
gestures_transform_angle = 角度
gestures_transform_hint_touch = 单指拖动，双指捏合和旋转。
gestures_transform_hint_pointer = 用鼠标拖动。在触控板上捏合和旋转。
gestures_reset = 重置

# Pointer: what on_drag, on_touches, on_tap_at and on_hover report, drawn and in figures.
gestures_pointer_title = 指针
gestures_pointer_subtitle = on_drag、on_touches、on_tap_at 和 on_hover 的报告
gestures_pointer_stage_label = 绘制指针报告的舞台
gestures_pointer_hint_touch = 触摸、拖动，然后松开
gestures_pointer_hint_pointer = 悬停、点击，或拖动后松开
gestures_pointer_phase = 阶段
gestures_pointer_location = 位置
gestures_pointer_contacts = 触点
gestures_pointer_translation = 位移
gestures_pointer_speed = 速度
gestures_pointer_speed_value = { $speed } pt/s
gestures_pointer_waiting = 等待中
gestures_pointer_began = 开始
gestures_pointer_changed = 变化
gestures_pointer_ended = 结束
gestures_pointer_hover = 悬停
gestures_pointer_tap = 点按
gestures_pointer_one_contact = 此平台每次只报告一个触点。

# Tap, double tap and hold: Day's press gestures on one pad.
gestures_press_title = 点按、双击与长按
gestures_press_subtitle = on_tap、on_double_tap、on_long_press 和 on_pressed
gestures_press_pad_label = 统计点按和长按的触摸区
gestures_press_pad_touch = 点按、连点两次，或按住直到圆环闭合。
gestures_press_pad_pointer = 点击、双击，或按住直到圆环闭合。
gestures_press_taps = 点按
gestures_press_doubles = 双击
gestures_press_holds = 长按
gestures_press_derived_note = 这里 Day 从拖动事件推导按压，按住时请稍微移动一下。

# Swipe: a row that drags along one axis inside the page's vertical scroll.
gestures_swipe_title = 轻扫
gestures_swipe_subtitle = 垂直滚动中的 on_drag_along
gestures_swipe_row_title = 轻扫这一行
gestures_swipe_row_touch = 向侧面轻扫。上下移动仍会滚动页面。
gestures_swipe_row_pointer = 向侧面拖动后松开。
gestures_swipe_count = 轻扫次数
gestures_swipe_release = 松开速度

# Hit testing: a dot with a larger tap area, and a button under a canvas with no handlers.
gestures_hit_title = 命中测试
gestures_hit_subtitle = 点按落在哪里
gestures_slop_target_label = 点按区域更大的圆点
gestures_slop_caption = { $dot } pt 的圆点，{ $area } pt 的点按区域
gestures_slop_unsupported = { $dot } pt 的圆点，此处只在自身范围内命中
gestures_slop_count = { $count ->
   *[other] { $count } 次点按
}
gestures_through_button = 画布下方
gestures_through_caption = 在 iOS 和 Android 上，没有处理器的画布不接收触摸。
