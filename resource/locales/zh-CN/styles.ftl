# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Styles page: a surface, corners, outlines, gradients, shadows, layers, transforms, digits and direction.

# One word beside a specimen that is drawn and not a view.
styles_canvas = 画布
styles_points = { NUMBER($n, maximumFractionDigits: 0) } 点
styles_degrees = { NUMBER($n, maximumFractionDigits: 0) }°
styles_percent = { NUMBER($p, style: "percent") }

styles_surface_title = 表面
styles_surface_subtitle = 一个视图上的填充、圆角、边框与阴影
styles_surface_fill = 填充
styles_surface_solid = 纯色
styles_surface_linear = 线性
styles_surface_radial = 径向
styles_surface_conic = 锥形
styles_surface_corners = 圆角
styles_surface_all = 全部
styles_surface_bottom = 底部
styles_surface_radius = 半径
styles_surface_border = 边框
styles_surface_shadow = 阴影

styles_corners_title = 圆角
styles_corners_subtitle = 半径标记、指定的角与胶囊
styles_corners_radius = 半径标记
styles_corners_top = 顶部
styles_corners_right = 右侧
styles_corners_diagonal = 对角
styles_corners_one = 单个角
styles_corners_four = 四个半径
styles_corners_capsule = 胶囊

styles_borders_title = 轮廓
styles_borders_subtitle = 视图上的边框，画布上的虚线
styles_borders_weights = 粗细
styles_borders_rounded = 沿着圆角
styles_borders_piece = 围绕一个部件
# A catalog designation, the same in every language.
styles_borders_sample = M31
styles_borders_ring = 圆环
styles_borders_overlay = 渐变之上
styles_borders_dashes = 虚线与点线

styles_gradients_title = 渐变
styles_gradients_subtitle = 视图上的线性、径向与锥形填充
styles_gradients_vertical = 垂直
styles_gradients_horizontal = 水平
styles_gradients_clear = 渐变至透明
styles_gradients_radial = 径向
styles_gradients_conic = 锥形

styles_shadows_title = 阴影
styles_shadows_subtitle = 由表面投下，随其形状
styles_shadows_blur = 模糊
styles_shadows_glow = 光晕
styles_shadows_offset = 偏移
styles_shadows_shaped = 指定的角
# A catalog designation, the same in every language.
styles_shadows_sample = M42
styles_shadows_lift = 抬起
styles_shadows_image = 图片之下

styles_layers_title = 图层
styles_layers_subtitle = 裁剪、层序与不透明度
styles_layers_clip = 裁剪
styles_layers_clipped = 已裁剪
styles_layers_visible = 溢出
styles_layers_stack = 层序
styles_layers_front = 最前
styles_layers_opacity = 不透明度

styles_transforms_title = 变换
styles_transforms_subtitle = 围绕锚点旋转与缩放
styles_transforms_rotate = 旋转
styles_transforms_scale = 缩放
styles_transforms_anchor = 锚点
styles_transforms_center = 中心
styles_transforms_corner = 角
styles_transforms_orbit = 轨道
styles_transforms_shear = 错切
styles_transforms_reset = 重置

styles_numerals_title = 等宽数字
styles_numerals_subtitle = 与太阳的距离，百万千米
styles_numerals_proportional = 默认
styles_numerals_tabular = 等宽
styles_numerals_figure = { NUMBER($n, minimumFractionDigits: 2) }

styles_planet_mercury = 水星
styles_planet_venus = 金星
styles_planet_earth = 地球
styles_planet_mars = 火星
styles_planet_jupiter = 木星
styles_planet_saturn = 土星
styles_planet_uranus = 天王星
styles_planet_neptune = 海王星

styles_rtl_title = 布局方向
styles_rtl_subtitle_ltr = 这种语言从左到右书写
styles_rtl_subtitle_rtl = 这种语言从右到左书写
styles_rtl_leading = 起始侧
styles_rtl_trailing = 末尾侧
# Two scripts on one line: the Milky Way, in this language and in Arabic or English.
styles_rtl_mixed = 银河 · درب التبانة
