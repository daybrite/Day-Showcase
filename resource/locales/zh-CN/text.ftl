# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Text page's newer cards (src/pages/text.rs). Its other strings are in app.ftl.

# Styled runs: the names of the styles, each drawn in its style.
text_run_code = 代码
text_run_color = 颜色
text_run_highlight = 高亮
text_run_larger = 放大
text_underline_single = 单线
text_underline_double = 双线
text_underline_dotted = 点线
text_underline_wavy = 波浪线

# Relative size: a semantic style and the factor it is scaled by.
text_scale_header = 相对大小
text_scale_of = { $style } × { $scale }

# Presets: two readings that share three named styles.
text_preset_header = 预设样式
text_preset_moon = 地球到月球
text_preset_moon_value = 384,400
text_preset_km = 千米
text_preset_sun = 地球到太阳
text_preset_sun_value = 1.496
text_preset_million_km = 亿千米

# Line limits and alignment share one paragraph.
text_sample_paragraph = 阳光到达地球大约需要八分钟，到达海王星需要四个多小时，到达最近的恒星则需要四年多。
text_lines_header = 行数限制
text_lines_one = 一行
text_lines_two = 两行
text_lines_all = 不限
text_align_header = 对齐
text_align_leading = 起始

# Scripts and direction: the names of the specimens.
text_scripts_header = 文字与书写方向
text_scripts_cjk = 中文、日文和韩文
text_scripts_rtl_in_ltr = 拉丁文中的阿拉伯文和希伯来文
text_scripts_ltr_in_rtl = 阿拉伯文中的拉丁文

# Decorations and font features.
text_decorations_header = 装饰线与数字
text_small_caps = 小型大写字母
text_small_caps_sample = Mission Control, Houston
text_figures = 数字

# The reader's text size, and a badge that keeps its size.
text_scale_reading = 文字大小：{ $scale }×
text_scale_fixed = 固定大小

# Line limits: how many lines, and where the ellipsis goes.
text_lines_count = 行数
text_lines_truncate = 省略号
text_trunc_head = 开头
text_trunc_middle = 中间
text_trunc_tail = 末尾

# Letter spacing and line height.
text_spacing_header = 间距
text_spacing_letters = 字距
text_spacing_lines = 行高
text_align_justified = 两端对齐

# A row that reads in the direction the picker names.
text_direction = 方向
text_direction_ltr = 从左到右
text_direction_rtl = 从右到左
text_direction_first = 1 发射
text_direction_second = 2 入轨
text_direction_third = 3 着陆

# Links and selection.
text_link_handled_label = 由应用处理
text_selection_none = 选中上方的部分文字。
text_selection_value = 已选中：{ $text }

# The Markdown view: a reply streamed in word by word.
text_md_view_header = Markdown 视图
text_md_stream = 流式生成回复
text_md_copy = 复制
text_md_copied = 代码已复制
text_md_reply =
    ## 轨道检查

    探测器每圈发回 **三项** 读数：

    - 高度
    - 速度
    - 电量

    它的周期由轨道大小决定：

    ```rust
    let period = 2.0 * PI * (a.powi(3) / mu).sqrt();
    ```

    读数稳定后再规划下一次点火。
