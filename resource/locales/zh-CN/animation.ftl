# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Animation page (src/pages/animation.rs). The names of Day's APIs stay as they are in
# every language.

# Easing curves: one curve of each Easing variant, named after the variant.
animation_easing_title = 缓动曲线
animation_easing_subtitle = 同一时钟下的每种 Easing
animation_easing_play = 运行
animation_curve_linear = 线性
animation_curve_bezier = 贝塞尔
animation_curve_power = 幂
animation_curve_sine = 正弦
animation_curve_circle = 圆
animation_curve_exponential = 指数
animation_curve_back = 回退
animation_curve_elastic = 弹性
animation_curve_bounce = 弹跳

# Platform animator: the playground. Its controls' labels are the anim_* keys in app.ftl.
animation_play_title = 平台动画器
animation_play_subtitle = 一次 with_animation 驱动六个属性

# Animated values: a planet that glides between four sets of figures.
animation_tween_title = Animated 数值
animation_tween_subtitle = 大小、距离与颜色一同过渡
animation_tween_mercury = 水星
animation_tween_earth = 地球
animation_tween_jupiter = 木星
animation_tween_neptune = 海王星
animation_tween_diameter = 直径
animation_tween_distance = 与太阳的距离
animation_tween_km = { $km } 千米
animation_tween_au = { $au } 天文单位

# Interpolation: one value that a tile's place, size, turn and color are mapped from.
animation_interp_title = 插值
animation_interp_subtitle = 一个值驱动位置、大小、旋转和颜色
animation_interp_value = 数值
animation_interp_release = 松手速度
animation_interp_speed = { $speed } pt/秒
animation_interp_flip = 翻转

# Spring: one spring set by three sliders, plotted and run.
animation_springs_title = 弹簧
animation_springs_subtitle = 拖动滑块后松手
animation_springs_stiffness = 刚度
animation_springs_damping = 阻尼
animation_springs_mass = 质量
animation_springs_release = 发射
animation_springs_ratio = ζ { $ratio }

# Choreography: bars moved by composed steps.
animation_compose_title = 编排
animation_compose_subtitle = 帧时钟上的组合步骤
animation_compose_sequence = 序列
animation_compose_parallel = 并行
animation_compose_stagger = 交错
animation_compose_repeat = 循环
animation_compose_play = 开始
animation_compose_stop = 停止
animation_compose_reset = 重置

# Implicit animation: a heading and a button over a scrolling log. A sol is a day on Mars.
animation_header_title = 隐式动画
animation_header_subtitle = 跟随滚动的标题和按钮
animation_header_row = 火星日 { $n }
animation_header_top = 顶部
animation_header_heading = 任务日志
animation_header_offset = { $y } pt
animation_header_untracked = 未报告偏移

# Reflow: keyed tiles whose layout changes run inside with_animation.
animation_layout_title = 重排
animation_layout_subtitle = with_animation 中的布局变化
animation_layout_tiles = 方块
animation_layout_size = 大小
animation_layout_small = 小
animation_layout_medium = 中
animation_layout_large = 大
animation_layout_shuffle = 打乱
