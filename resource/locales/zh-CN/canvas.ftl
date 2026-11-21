# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Canvas page (src/pages/canvas.rs).

canvas_dial_title = 3 号涡轮负载
canvas_dial_subtitle = 指针与弧形由同一个弹簧值驱动
canvas_dial_unit = % 负载
canvas_dial_idle = 怠速
canvas_dial_cruise = 巡航
canvas_dial_peak = 峰值
canvas_dial_zone = 区间
canvas_dial_zone_redline = 红线
canvas_dial_zone_high = 偏高
canvas_dial_zone_nominal = 正常
canvas_dial_spool = 转速
canvas_dial_spool_value = { $rpm } rpm
canvas_dial_limit = 上限

canvas_clock_title = 变换
canvas_clock_subtitle = translate · rotate · save / restore
canvas_clock_ticks = 刻度
canvas_clock_ticks_note = 60 次旋转
canvas_clock_hands = 指针
canvas_clock_hands_note = save, restore
canvas_clock_second = 秒针
canvas_clock_second_note = 一个 FrameClock 订阅

canvas_svg_title = SVG 路径
canvas_svg_subtitle = 路径数据，由 build_path! 在编译期解析
canvas_svg_heart = 心形
canvas_svg_star = 星形
canvas_svg_bolt = 闪电
canvas_svg_check = 对勾

canvas_compositing_title = 合成
canvas_compositing_subtitle = 裁剪、阴影、擦除镂空、混合模式
canvas_compositing_image = 裁剪后的图像
canvas_compositing_cutout = 镂空
canvas_compositing_radial = 径向渐变
canvas_compositing_blend = 混合模式
canvas_compositing_blend_code = Draw::blended
canvas_blend_normal = 正常
canvas_blend_multiply = 正片叠底
canvas_blend_screen = 滤色
canvas_blend_add = 相加

canvas_sketch_title = 涂鸦
canvas_sketch_subtitle = 拖动 → 用 on_drag 绘制
canvas_sketch_clear = 清除
canvas_sketch_empty = 在此绘制
canvas_sketch_strokes = 笔画
canvas_sketch_count = { $strokes } · { $points } 个点

canvas_text_subtitle = measure_text 报告的度量，适用于任意字体族
canvas_anchors_subtitle = 十字线上的十二种 TextAnchor 位置
canvas_interaction_subtitle = on_hover · on_tap_at · on_drag
canvas_image_subtitle = 先 decode_image，再 Draw::image
