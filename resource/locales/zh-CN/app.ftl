app_title = Day 演示
counter_value = { $count } 次点击
decrement = −
increment = +
name_placeholder = 你的名字
flavor_placeholder = 输入或选择口味
flavor_add = 添加
nav_controls = 控件
ctl_buttons = 按钮
ctl_press = 按下
ctl_plain = 普通
ctl_bordered = 描边
ctl_prominent = 突出
ctl_tinted = 着色
ctl_destructive = 危险操作
ctl_disabled = 已禁用
ctl_link = 链接
ctl_composed = 组合式
ctl_link_text = daybrite.dev
ctl_presses = 按下次数
ctl_switches = 开关与滑块
ctl_toggle = 开关
ctl_slider = 滑块
ctl_stepped = 分档滑块
ctl_stepper = 步进器
ctl_composed_stepper = 组合步进器
ctl_progress = 进度
ctl_activity = 活动指示
ctl_spinner = 加载指示
ctl_accessibility = 无障碍
ctl_a11y_heading = 供屏幕阅读器使用的标题
ctl_announce = 播报
ctl_announce_hint = 通过屏幕阅读器读出计数
ctl_announced_times = 已播报 { $count } 次
ctl_announce_emulated = 模拟（实时区域）
ctl_text_entry = 文本输入
ctl_text_field = 文本框
ctl_search_field = 搜索框
ctl_combo_box = 组合框
ctl_combo_note = 此工具包没有组合框控件，Day 在此显示占位符。
ctl_text_area = 文本区域
ctl_notes_placeholder = 几行备注
ctl_pickers = 选择器
ctl_date = 日期
ctl_time = 时间
ctl_color = 颜色
ctl_indicators = 指示器与组合
ctl_badge = 角标
ctl_gauge = 仪表
ctl_divider = 分隔线
ctl_state = 状态
ctl_on = 开
ctl_off = 关
nav_menus = 菜单与对话框
nav_text = 文字
nav_battery = 电池
nav_sensors = 传感器
nav_clipboard = 剪贴板
nav_media = 媒体
nav_compose = 卡片与环境
nav_files = 文件
nav_tabs = 标签页
nav_stack = 堆栈
nav_layout = 布局
layout_stacks_section = 行、列与层叠
layout_align_label = 对齐
layout_align_top = 顶部
layout_align_center = 居中
layout_align_bottom = 底部
layout_layered = 层叠
layout_scroll_section = 滚动
layout_scroll_top = 滚动到顶部
layout_scroll_end = 滚动到末尾
layout_size_section = 尺寸类别
layout_size_width = 宽度
layout_size_height = 高度
nav_list = 列表
refresh_tier_native = 下拉刷新：原生
refresh_tier_emulated = 下拉刷新：模拟
layout_row = 第 { $n } 项
nav_webview = 网页视图
nav_lottie = Lottie
nav_about = 关于

# Sidebar group headers (lib.rs `Group`): the sections the destinations sit under.
group_starred = 已加星标
group_overview = 概览
group_controls = 控件
group_layout = 布局
group_navigation = 导航与界面
group_data = 数据
group_graphics = 图形与媒体
group_platform = 平台
group_app = 应用与工具

picker_selected = 已选
picker_segmented = 分段
picker_menu = 菜单
picker_inline = 内联

# --- day::datetime ---
nav_dates = 日期与时间
dates_date_section = 日期
dates_time_section = 时间
dates_composed_section = 组合
date_compact = 紧凑
time_compact = 紧凑
time_seconds = 含秒
dates_composed = 日期和时间
date_bounded = 2026 年内
date_picked = 所选日期
time_picked = 所选时间

compose_card_title = 可复用的表面
compose_card_body = 内边距、背景和圆角。
compose_env_value = 使用提供的强调色着色
list_add = 添加 100 行
list_caption = { $count } 行
list_selection_none = 未选择任何行
list_selection = { $count ->
   *[other] 已选行：{ $rows }
}
list_clear_selection = 清除选择
list_reorder_hint = 拖动行以重新排序，滑动以删除；第一行已固定
list_order = 前几行：{ $rows }
list_shuffle = 打乱顺序
list_reset = 重置
list_delete = 删除
list_options = 选项…
list_options_title = 列表选项
list_options_layout = 布局
list_options_behavior = 行为
list_options_gestures = 手势
list_opt_sections = 分区标题
list_opt_row_height = 行高
list_opt_uniform = 统一
list_opt_automatic = 自动
list_opt_separators = 分隔线
list_opt_default = 默认
list_opt_on = 开
list_opt_off = 关
list_opt_content = 行内容
list_opt_plain = 标签
list_opt_item = 列表项
list_opt_multi = 多选
list_opt_pull = 下拉刷新
list_opt_editing = 编辑模式
list_opt_reorder = 拖动排序
list_opt_swipe = 滑动操作
list_opt_delete = 滑动删除
list_opt_gestures_footer = 显示分组标题时，行只能在所属分组内拖动，轻扫删除关闭。
list_section = 第 { $first }–{ $last } 行
list_row_note = 此行的第二行详细信息
list_row_note_long = 按内容调整高度的行会随更长的文字增高，比如这第三行在窄窗口中会自动换行。
list_swipe_select = 选择
list_swipe_deselect = 取消选择
list_align = 跳转位置
list_align_nearest = 最近
list_align_start = 顶部
list_align_center = 中间
list_align_end = 底部
list_insert = 插入
list_remove = 移除
list_remove_all = 全部移除
list_footer = 列表结束：共 { $count } 行
list_empty = 没有行。点“重置”即可恢复。
list_visible = 可见：{ $first }–{ $last }
list_visible_first = 从 { $first } 起可见
list_visible_none = 没有可见的行

nav_model = 模型
model_caption = 显示 { $count } 行
model_cost = 观察 { $paths } 条路径 · { $nodes } 个驻留节点
model_hide_done = 隐藏已完成
model_selected = 已选：{ $title }
model_selected_none = 未选择
model_add = 添加任务
model_delete = 删除所选
model_undo = 撤销
model_redo = 重做
model_storage_file = 存储于 { $file }
model_storage_memory = 在此平台上存储于内存
nav_query = 查询
query_caption = { $total } 条曲目中有 { $count } 条匹配
query_resident = 数据库工作线程已缓存 { $resident } / { $total } 行
query_starred = 仅显示星标
query_selected = 已选：{ $title }
query_selected_none = 未选择任何项
query_fts = 全文检索（FTS5）
query_viewport = 仅视口内
query_viewport_box = 纬度 { $from }–{ $to }
query_star = 切换星标

webview_url_hint = 输入网址
webview_go = 前往
webview_back = 后退
webview_forward = 前进
webview_stop = 停止
webview_reload = 重新加载
webview_js_hint = 要在页面中运行的 JavaScript
webview_js_run = 运行 »
webview_js_result_hint = JSON 结果显示在此
webview_note_iframe = 此平台通过 iframe 嵌入页面，因此“后退”“前进”和“停止”不可用，地址栏也不会跟随链接。
webview_tab_remote = 远程
webview_tab_embedded = 内嵌
webview_embedded_unsupported = 此工具包构建中没有可用的网页引擎。
webview_embedded_status_none = 尚未跟随任何外部链接。
webview_embedded_opened = 已在应用外打开：{ $url }
webview_embedded_intercepted = 已拦截——已在应用内导航到“{ $route }”。

stack_push = 推入一个详情页
stack_detail_title = 第 { $depth } 层
stack_item_title = 条目 { $id }
stack_link_42 = 按路由打开第 42 项
stack_param_hint = 携带提示打开：{$hint}
tab_one = 概览
tab_two = 详情
tab_three = 设置
tab_state_note = 切换标签页再回来，计数仍在。
tab_detail_route = 路由键
tab_detail_link = 深层链接
tab_set_badges = 显示角标
tab_set_sounds = 播放声音
about_text = 一个用 day 构建的原生跨平台应用。
modal_alert = 显示警告框
modal_confirm = 确认
modal_delete = 删除…
modal_sheet = 选择口味
modal_prompt = 输入名字
alert_title = 提示
alert_body = 你的更改已保存。
ok = 好
confirm_title = 退出？
confirm_body = 确定要退出吗？
delete_title = 删除条目？
delete_body = 此操作无法撤销。
delete = 删除
flavor_title = 选择一种口味
cancel = 取消
vanilla = 香草
pistachio = 开心果

# Files playground (docs/files.md)
files_placeholder = 输入要保存的内容…
files_open = 打开文件…
files_save = 保存文件…
files_opened = 已打开 { $name }

# Battery playground (docs/battery.md)
battery_refresh = 读取设备电池
battery_level = 电量
battery_charging = 充电中
battery_reading = 电池：{ $percent } · { $state }
battery_reading_none = 电池：此平台没有电池 API
battery_reading_absent = 电池：此设备没有电池

# Sensors playground (docs/sensors.md)
sensor_accelerometer = 加速度计
sensor_gyroscope = 陀螺仪
sensor_magnetometer = 磁力计
sensor_reading = x { $x } · y { $y } · z { $z } { $unit }
sensor_waiting = 等待第一个样本…
sensor_unavailable = 此设备上不可用

# Clipboard playground (docs/clipboard.md)
clipboard_placeholder = 输入要复制的内容
clipboard_copy = 复制
clipboard_paste = 粘贴
clipboard_idle = 剪贴板未使用
clipboard_copied = 已复制到系统剪贴板
clipboard_copy_failed = 复制失败（此处没有剪贴板 API）
clipboard_pasted = 已从系统剪贴板粘贴
clipboard_empty = 剪贴板为空或无法读取

# Network playground (docs/network.md)
network_refresh = 读取网络
network_reading_online = 在线 · { $kind } · 计费：{ $expensive }
network_reading_offline = 离线
network_reading_none = 无法获取网络状态

# Media playground (docs/media.md)
media_play = 播放
media_pause = 暂停
media_load = 加载

# --- Localization page (docs/localization.md) ---
nav_localization = 本地化
loc_locale_section = 实时语言
loc_live_note = 布局方向在启动时确定。
loc_current_label = 当前
loc_reset = 重置
loc_numbers_section = 数字
loc_dates_section = 日期与时间
loc_plurals_section = 复数
loc_sorting_section = 排序
fmt_number_label = 分组
fmt_fraction_label = 两位小数
fmt_percent_label = 百分比
fmt_date_label = 长日期
fmt_time_label = 时间
fmt_datetime_label = 日期和时间
fmt_sorted_label = 排序结果
fmt_number = { NUMBER($n) }
fmt_fraction = { NUMBER($n, minimumFractionDigits: 2) }
fmt_percent = { NUMBER($p, style: "percent") }
fmt_date = { DATETIME($d, dateStyle: "long") }
fmt_time = { DATETIME($t, timeStyle: "short") }
fmt_datetime = { DATETIME($dt, dateStyle: "medium", timeStyle: "short") }
plural_items = { $count ->
    [0] 暂无条目
   *[other] { $count } 个条目
}

# Text playground (typography)
text_selectable_toggle = 可选中
text_styles_header = 样式
text_weights_header = 字重
text_styling_header = 粗体、斜体与颜色
text_custom_header = 自定义字号
text_fonts_header = 打包字体

# Menus playground
menus_last = 最近操作
menus_lifecycle = 生命周期
menus_target = 在此右键或长按

# --- day-part-haptics ---
nav_haptics = 触感反馈
haptics_supported_yes = 触感引擎可用
haptics_supported_no = 此平台没有触感引擎
haptics_light = 轻
haptics_medium = 中
haptics_heavy = 重
haptics_success = 成功
haptics_warning = 警告
haptics_error = 错误
haptics_selection = 选择
haptics_last = 最近播放
haptics_none = 尚未播放
haptics_last_played = 已播放：{ $style }

# --- day-part-sound ---
nav_sound = 音效
sound_click = 点击
sound_confirm = 确认
sound_error = 错误
sound_glass = 玻璃
sound_card = 纸牌
sound_jingle = 旋律
sound_pan_caption = 立体声声像
sound_left = 左
sound_center = 中
sound_right = 右
sound_volume = 音量
sound_last = 最近播放
sound_none = 尚未播放
sound_last_played = 已播放：{ $clip }
sound_loop = 循环播放嗡鸣
sound_loop_volume = 循环音量
sound_looping = 正在循环
sound_looping_none = 没有循环
sound_looping_clip = 正在循环：{ $clip }

# --- day-part-preferences ---
nav_prefs = 偏好设置
prefs_placeholder = 要记住的备注
prefs_empty = （未存储任何内容）
prefs_value_label = 已存储的值

# --- bundled resources (§18.3) ---
nav_resources = 资源
vectors_title = 矢量图
vectors_tints = 着色
vectors_zoom = 缩放
vectors_weights = 字重
vectors_sizes = 尺寸
resources_numbers = numbers.bin：{ $len } 字节，byte[100] = { $byte }
resources_greeting = greeting.txt：{ $text }

# --- day-part-device ---
nav_deviceinfo = 设备信息
deviceinfo_model = 型号：{$value}
deviceinfo_system = 系统：{$name} {$version}
deviceinfo_simulator = 模拟器：{$value}
deviceinfo_yes = 是
deviceinfo_no = 否
deviceinfo_refresh = 刷新

# --- Day core spinner ---
activity_on = 旋转中
activity_off = 已停止

# --- day::searchfield ---
search_clear = 清除


# --- tweaks page (docs/tweaks.md) ---
nav_tweaks = 微调
tweaks_stock = 原样
tweaks_tweaked = 已微调
tweaks_bezel_title = 按钮边框样式
tweaks_bezel_caption = 仅 AppKit
tweaks_selectable_title = 可选中标签
tweaks_selectable_text = 选中这段文字并复制。
tweaks_tooltip_title = 工具提示
tweaks_tooltip_caption = AppKit、GTK 和 Android。将指针悬停在按钮上或长按按钮。
tweaks_tooltip_label = 自动保存
tweaks_tooltip_hint = 你的工作会自动保存。
tweaks_ticks_title = 滑块刻度
tweaks_ticks_caption = AppKit、GTK、Android、Qt、XAML 和 ArkUI。微调后的滑块会吸附到刻度。
tweaks_ref_title = NativeRef 存活性
tweaks_ref_caption = 挂载滑块
tweaks_ref_live = 引用：存活
tweaks_ref_cleared = 引用：已清空
tweaks_label_title = 微调文本标签
tweaks_label_caption = AppKit 和 UIKit。在 iOS 上，“可选中”会更换原生类。
tweaks_label_sample = 此标签由原生微调调暗。
tweaks_label_class = 原生类：{ $class }

# --- merged section pages (design overhaul) ---
nav_canvas = 画布
nav_charts = 图表
nav_system = 设备与传感器
nav_network_http = 网络与 HTTP
network_status_section = 连接状态
nav_notify_badge = 通知与角标
nav_push = 推送通知
nav_speech_haptics = 语音、音效与触觉
nav_files_storage = 文件与存储
gauge_value_label = 数值
canvas_text_title = 文本与字体
canvas_anchors_title = 文本锚点
canvas_interaction_title = 点按、拖动与悬停
canvas_interaction_label = 已报告
canvas_interaction_none = —
canvas_interaction_readout = 悬停 { $hover } · 点按 { $tap } · 拖动 { $trail } 个点
canvas_metrics_cache = measure_text：{ $hits } 命中 · { $misses } 实测 · { $entries } 保留
canvas_font_label = 字体
canvas_font_default = 系统
canvas_text_specimen = 敏捷的棕色狐狸跳过懒狗
canvas_text_bold = 粗体
canvas_text_italic = 斜体
canvas_text_bold_italic = 粗斜体
canvas_image_title = 从字节解码的图像
canvas_image_loading = 正在解码…
canvas_image_failed = 这些字节不是本平台可解码的图像
canvas_image_readout = { $format } · { $width } × { $height } 像素

# --- data strings localized for the walkthrough locales (option lists, specimen rows) ---
chocolate = 巧克力
size_small = 小
size_medium = 中
size_large = 大
fruit_apple = 苹果
fruit_banana = 香蕉
fruit_cherry = 樱桃
fruit_date = 枣
fruit_elderberry = 接骨木莓
list_row = 第 { $n } 行
text_style_large_title = 大标题
text_style_title = 标题
text_style_title2 = 标题 2
text_style_title3 = 标题 3
text_style_headline = 头条标题
text_style_subheadline = 副标题
text_style_body = 正文
text_style_callout = 标注
text_style_footnote = 脚注
text_style_caption = 说明文字
text_style_caption2 = 说明文字 2
text_weight_ultralight = 极细
text_weight_light = 细体
text_weight_regular = 常规
text_weight_medium = 中等
text_weight_semibold = 半粗
text_weight_bold = 粗体
text_weight_heavy = 特粗
text_weight_black = 黑体
text_bold = 粗体文本
text_italic = 斜体文本
text_bolditalic = 粗斜体
text_emphasis_label = 强调
color_red = 红
color_green = 绿
color_blue = 蓝
color_orange = 橙

# 菜单与对话框（合并页面）
menus_appmenu_section = 应用菜单
menus_context_section = 上下文菜单
menus_dialogs_section = 对话框
modal_result_label = 结果

# 媒体页面
media_player_section = 视频

# 资源页面分区
resources_image_section = 内置图片
image_mode_fit = 适应
image_mode_fill = 填充
image_mode_stretch = 拉伸
resources_data_section = 内置数据

# 设置（docs/settings.md）：目录与诊断两个分组，与 Day 的“通用”分组并列。
# “关于”窗口的标签（版本、网站）和“通用”分组的标签来自 Day 自己的目录。
settings_catalog = 目录
settings_catalog_subtitle = 已加星标的页面
settings_starred_count = 已加星标的页面
settings_clear_starred = 清除星标
settings_clear_starred_confirm = 要移除所有页面的星标吗？
settings_diagnostics = 诊断
settings_diagnostics_subtitle = 工具包、系统、语言
about_toolkit = 工具包
about_id = 应用 ID
about_os = 系统
about_model = 型号
about_locale = 语言

# 焦点页（docs/focus.md）
nav_focus = 焦点
focus_group_section = 一个信号，一个表单
focus_name_label = 姓名
focus_email_label = 邮箱
focus_city_label = 城市
focus_current_label = 当前焦点
focus_next = 焦点后移
focus_clear = 清除焦点
focus_bool_section = 一个控件，一个布尔值
focus_bool_placeholder = 焦点落在这里
focus_focus_btn = 获取焦点
focus_blur_btn = 移除焦点
focus_state_label = 状态
focus_state_on = 有焦点
focus_state_off = 无焦点
focus_probe_section = 不只文本框
focus_probe_caption = 触屏平台只为文本输入保留焦点。
focus_probe_toggle = 开关
focus_probe_slider = 滑块
focus_probe_button = 按钮

# HTTP fetch demo (docs/http.md): the status readout stays raw "<status> <body>" so the
# walkthrough asserts it byte-for-byte in every locale.
http_idle = 尚未获取任何内容
http_checking = 检查中…
ws_title = WebSocket
ws_connect = 连接
ws_send = 发送
ws_close = 关闭

# Scrolling page (docs/scroll.md): programmatic scroll targets.
scroll_to_top = 滚动到顶部
scroll_to_bottom = 滚动到底部
scroll_to_item = 滚动到第 100 项

# Grid page (docs/grid.md): grid/grid_row from basics to a stress test.
nav_grid = 网格
grid_tab_basics = 基础
grid_tab_sizing = 尺寸
grid_tab_spanning = 跨列
grid_tab_composite = 组合
grid_tab_stress = 压力测试
grid_col_name = 姓名
grid_col_wins = 胜场
grid_col_points = 积分
grid_sizing_fixed = 固定 80 pt
grid_sizing_content = 按内容
grid_sizing_short = 短
grid_sizing_longer = 内容较长的单元格
grid_month_title = 周计划
grid_event_focus = 专注时段
grid_event_review = 评审
grid_day_n = 第 { $n } 天
grid_stress_add = 增加 50 行
grid_stress_bump = 递增首个单元格

nav_animation = 动画
nav_physics = 物理

# 动画页面
anim_scale = 缩放
anim_rotation = 旋转
anim_opacity = 不透明度
anim_offset_x = 偏移 X
anim_offset_y = 偏移 Y
anim_hue = 色相
anim_curve = 曲线
anim_duration = 时长
anim_randomize = 随机！
anim_go_label = 动画！
anim_reset_label = 重置
anim_curve_spring = 弹簧
anim_curve_ease_in_out = 缓入缓出
anim_curve_ease_out = 缓出
anim_curve_linear = 线性
anim_duration_ms = { $ms } 毫秒
anim_reduced_motion = 减弱动态效果

# --- 基准测试页（Day-Bench 的网格基准测试；在 Apple 原生后端上，分段选择器还通过
#     day-piece-swiftui 承载其手写的 SwiftUI 孪生实现，docs/swiftui.md）---
nav_benchmark = 基准测试
bench_parameters = 参数
bench_seed = 随机种子
bench_count = 总数
bench_rows = { $rows ->
   *[other] { $rows } 行
}
# 供承载的 SwiftUI 面板使用的 %d 模板（其计数保存在 Swift 状态中）。
bench_rows_one = %d 行
bench_rows_other = %d 行
bench_tab_day = Day 原生
bench_tab_swiftui = SwiftUI 网格

# 菜单栏与上下文菜单
menu_file = 文件
menu_open = 打开…
menu_open_recent = 最近打开
menu_clear_menu = 清除菜单
menu_save = 保存
menu_save_as = 另存为…
menu_edit = 编辑
menu_view = 视图
menu_reload = 重新加载
menu_actual_size = 实际大小
menu_context = 上下文
menu_rename = 重命名
menu_duplicate = 创建副本
menu_move_to = 移动到
menu_inbox = 收件箱
menu_archive = 归档

# 文字页面：字号、内置字体说明、链接
text_size_pt = { $pt } 磅
text_font_pacifico = Pacifico — 流畅手写体
text_font_bungee = BUNGEE — 彩色展示体
text_font_specialelite = Special Elite — 打字机字体
text_font_pacifico_lg = 36 磅 Pacifico
text_links_section = 链接
text_runs_section = 富文本片段
text_markdown_section = 实时解析的 Markdown
text_markdown_sample = Day 在运行时解析 **粗体**、*斜体*、`代码`、~~删除线~~ 和[链接](https://daybrite.dev)。像 ** 这样未闭合的标记保持原样。
text_markdown_opened = 已打开：{$url}
text_baseline_section = 基线对齐
text_baseline_toggle = 对齐基线
text_baseline_quantity = 数量
text_baseline_unit = 件
text_baseline_total = 合计
text_baseline_currency = 美元
text_baseline_due = 到期
text_link_icons_label = Google Fonts 上的 Material Symbols
text_link_mail_label = 给团队发邮件

# 文件区：编辑器的初始文本
files_initial_content =
    来自 Day 的问候！
    编辑我，然后保存。

# 崩溃报告页面（day-piece-crash-reporter，docs/break.md）
nav_crash = 崩溃报告
crash_trigger_section = 触发崩溃
crash_report_section = 最近的崩溃报告
crash_abort = 崩溃（中止）
crash_abort_label = 原生中止 → SIGABRT
crash_segv = 崩溃（段错误）
crash_segv_label = 空指针解引用 → SIGSEGV
crash_contained = 崩溃（已捕获）
crash_contained_label = 被 day 捕获，应用继续运行
crash_send = 发送报告
crash_clear = 清除报告
crash_empty = 暂无崩溃报告。触发一次崩溃，然后重新启动。

# 文本框页面 (docs/textfield.md)
nav_textfields = 文本框
textfields_password_section = 密码
textfields_password = 密码
textfields_password_placeholder = 输入密码
textfields_show_password = 显示密码
textfields_length = 长度
textfields_purpose_section = 键盘与自动填充
textfields_email = 电子邮件
textfields_phone = 电话
textfields_url = 网站
textfields_number = 数量
textfields_decimal = 金额
textfields_code = 一次性验证码
textfields_pin_section = PIN 码
textfields_pin = PIN 码
textfields_attrs_section = 属性
textfields_reference = 参考编号
textfields_read_only = 只读
textfields_limited = 最多 12 个字符
textfields_message = 消息
textfields_message_placeholder = 输入后按“发送”
textfields_sent = 已发送

# 文本区域页面
nav_textareas = 文本编辑
textareas_editor_section = 编辑器
textareas_seed_section = 填充为
textareas_attrs_section = 属性
textareas_seed_short = 短文本
textareas_seed_long = 长文本
textareas_seed_markdown = Markdown
textareas_editable = 可编辑
textareas_selectable = 可选择
textareas_spellcheck = 拼写检查
textareas_sample_short = 一段简短的备注。可以直接编辑，或用下面的按钮填入更长的或带格式的文本。

# 富文本编辑器 (docs/texteditor.md)
textareas_styled_section = 富文本编辑器
textareas_toolbar_section = 格式
textareas_export_section = 导入与导出
textareas_text_color = 文字颜色
textareas_align = 对齐
textareas_align_natural = 自然
textareas_align_center = 居中
textareas_align_trailing = 末端
textareas_doc_note = 笔记
textareas_doc_code = 代码
textareas_doc_empty = 空白
textareas_placeholder = 开始输入，或在上方选择一个文档。
textareas_inspector = 选区 { $start }–{ $end } · { $marks } · 字号 ×{ $scale }
textareas_export_markdown = Markdown
textareas_export_html = HTML
textareas_export_rtf = RTF
textareas_import = 读回
textareas_select_word = 选择一个词
textareas_no_selection = 未选择内容
textareas_mark_bold = 粗体
textareas_mark_italic = 斜体
textareas_mark_underline = 下划线
textareas_mark_strike = 删除线
textareas_mark_plain = 无格式
sensor_permission = 运动权限
perm_request = 请求
perm_open_settings = 打开设置
nav_location = 定位
location_permission = 定位权限
location_start = 开始
location_stop = 停止
location_waiting = 正在等待定位…
location_unavailable = 此平台没有定位服务
location_coords = { $lat }, { $lon }
location_altitude = 海拔
location_accuracy = 精度
location_unknown = —
chart_axes = x 红 · y 青 · z 紫

# 全屏覆盖层（docs/cover.md）。
stack_cover_button = 覆盖层
cover_title = 全屏覆盖层
cover_dismiss = 关闭

# 半高面板（docs/cover.md）：窗口下半部分，页面仍在其下可见。
stack_half_button = 半高面板
half_title = 半高面板
half_level = 级别
half_readout = 面板级别：{ $level }

stack_unsaved = 未保存的更改
stack_discard_title = 放弃更改？
stack_discard_body = 此页面有未保存的更改。
stack_discard_ok = 放弃

tab_dynamic_title = 数据驱动的标签页
tab_dynamic_add = 添加标签页
tab_dynamic_remove = 移除标签页

# 本地文件存储（服务页，docs/fs.md）。
storage_title = 本地文件
storage_placeholder = 要存储的文本
storage_save = 保存文件
storage_load = 加载
storage_delete = 删除
storage_files_label = 已存文件
storage_idle = 尚未存储任何内容

prefs_window_title = 偏好设置

# --- Tree page (docs/tree.md) ---
nav_tree = 树
tree_expand_all = 全部展开
tree_collapse_all = 全部折叠
tree_reveal = 定位到 tree.rs
tree_add_file = 添加文件
tree_multi = 多选
tree_lock = 锁定 docs
tree_hint = 将行拖到文件夹上即可移动。锁定时，docs 拒绝放入。
tree_count = { $count ->
   *[other] { $count } 个节点
}
tree_selection_none = 未选择任何内容
tree_selection = { $count ->
   *[other] 已选（{ $count }）：{ $names }
}
tree_move_none = 尚无移动
tree_move = 已将 { $name } 移动到 { $parent } 的第 { $index } 位
tree_move_append = 已将 { $name } 移动到 { $parent }
tree_root = 根级
tree_ctx_new_file = 新建文件
tree_ctx_duplicate = 复制
tree_ctx_move_up = 上移
tree_ctx_move_down = 下移
tree_ctx_delete = 删除

# --- Toolbars page (docs/toolbars.md) ---
nav_toolbars = 工具栏
toolbar_unsupported = 此工具包没有窗口工具栏。
toolbar_readout_title = 工具栏正在做什么
toolbar_controls_title = 从这里驱动它
# Item labels: these appear in the toolbar, so they stay short.
toolbar_new = 新建窗口
toolbar_menu_open_scripting = 打开脚本页面
toolbar_menu_copy_script = 复制脚本
toolbar_extra_tooltip = 复制工具包与版本，便于提交缺陷报告
toolbar_extra = 复制信息
toolbar_search_placeholder = 搜索
show_source = 查看源代码
# The live readout.
toolbar_query_label = 搜索文字
toolbar_query_empty = （空）
toolbar_star_label = 星标
toolbar_on = 开
toolbar_off = 关
toolbar_presses_label = 按下次数
toolbar_presses = { $count ->
   *[other] { $count } 次按下
}
toolbar_last_label = 最近操作
toolbar_last_none = 暂无
toolbar_appearance_label = 外观
toolbar_appearance_ignored = { $mode }（此处忽略）
toolbar_transport_label = 录制器
toolbar_transport_idle = 空闲
toolbar_transport_recording = 录制中
toolbar_transport_playing = 播放中
toolbar_transport_paused = 已暂停
toolbar_last_new = 新建窗口
toolbar_last_extra = 已复制版本信息
# The page's own controls.
toolbar_extra_label = 显示“复制信息”项
toolbar_enabled_label = 已启用“显示源代码”
toolbar_clear_search = 清除搜索
toolbar_seed_search = 填充搜索
toolbar_seed_text = 工具栏
# --- day-part-local-notify (docs/notify.md) ---
nav_notify = 通知
notify_caps_post = 支持发布通知
notify_caps_unsupported = 此版本不可用。在 macOS 上，通知需要已签名的应用包（day pack）。
notify_caps_schedule_os = 定时通知在应用关闭后仍会送达
notify_caps_schedule_process = 应用退出后，已计划的通知会丢失
notify_title_label = 标题
notify_title_placeholder = 通知标题
notify_title_default = 来自 Day 的问候
notify_body_label = 正文
notify_body_placeholder = 通知正文
notify_body_default = 由 Day 展示应用发送。
notify_delay = 延迟
notify_delay_now = 立即
notify_delay_5s = 5 秒
notify_delay_15s = 15 秒
notify_delay_60s = 1 分钟
notify_importance = 重要性
notify_importance_low = 低
notify_importance_default = 默认
notify_importance_high = 高
notify_importance_urgent = 紧急
notify_sound = 播放提示音
notify_badge = 角标数字
notify_post = 发送
notify_cancel = 全部取消
notify_status_idle = 尚未发送任何通知
notify_status_posted = 已发送
notify_status_scheduled = 已定时
notify_status_cancelled = 已取消所有通知
notify_status_failed = 无法发送
notify_last = 最近结果
notify_perm_granted = 已授予通知权限
notify_perm_missing = 未授予权限：通知会被丢弃
notify_perm_request = 请求权限
haptics_songs_caption = 序列
haptics_song_celebration = 庆祝
haptics_song_levelup = 升级
haptics_song_heartbeat = 心跳
haptics_song_cascade = 级联

# --- 脚本录制（dayscript 录制器）---
nav_scripting = 脚本录制
scripting_record = 录制
scripting_stop = 停止
scripting_copy = 复制
scripting_export = 导出
scripting_copied = 已复制到剪贴板
scripting_delay_label = 步骤间延迟
scripting_delay_unit = 秒
scripting_saved_label = 已保存的脚本
scripting_pick = 加载已保存的脚本…
scripting_save = 保存
scripting_name_hint = 我的脚本
scripting_saved = 已保存

# --- app badge (docs/badge.md) ---
nav_badge = 应用角标
badge_caps_native = 应用图标会显示角标
badge_caps_emulated = 已发送；是否显示由桌面环境决定
badge_caps_none = 此平台没有应用角标 API
badge_android_note = Android 根据已发布的通知显示角标：请改用通知的角标计数。
badge_count_label = 数量
badge_minus = −
badge_plus = +
badge_set = 设置角标
badge_clear = 清除
badge_set_text = 设为文本 “beta”
badge_last = 最近操作
badge_status_idle = 尚未设置
badge_status_set = 角标已设为 { $count }
badge_status_cleared = 角标已清除
badge_status_text = 角标已设为 “beta”

# The Controls page mixer: one shared state, many editors.
mix_custom = 自定义
voice_search_placeholder = 筛选口味…

# Context-menu demos (menus page): the message-list rows and the media card.
menu_reply = 回复
menu_forward = 转发
menu_share = 分享…
menu_copy_image = 拷贝图像
menu_save_image = 存储到照片
menu_get_info = 显示简介
menus_messages_section = 消息列表
menus_photo_section = 媒体卡片
msg_subject_one = 季度数据，初稿
msg_subject_two = 周末计划，最终答复
msg_subject_three = 工作坊的草图

# The Star command (commands.rs): one label per state, shared by the toolbar, the
# application menu and each navigation row's context menu.
# Save a picture of this window (commands.rs, docs/window-image.md).
cmd_screenshot = 截屏…
cmd_appearance_light = 浅色
cmd_appearance_system = 跟随系统
cmd_appearance_dark = 深色
cmd_record = 录制
cmd_stop_recording = 停止
cmd_play = 播放
cmd_pause = 暂停
cmd_resume = 继续
cmd_clear_recording = 清除录制
menu_appearance = 外观
menu_script = 脚本
cmd_star = 收藏
cmd_unstar = 取消收藏
cmd_toggle_pseudo_locale = 切换伪语言环境

# 语音（day-part-speech）：daybridge 的参考部件——一套 API，每个平台一种语言。
speech_title = 文本转语音
speech_phrase = 天气晴朗，稍后可能有雨。
speech_speak = 朗读
speech_stop = 停止
speech_support_label = 此平台支持
speech_state_speaking = 朗读中…
speech_state_finished = 已完成
speech_state_stopped = 已停止
speech_native = 原生
speech_emulated = 模拟（部分）
speech_unsupported = 此目标不支持

# Support banners (widgets.rs): shown over a demo the target cannot run.
support_missing_here = 此平台不支持——演示仍会显示，方便查看，但在这里无法运行。
support_emulated_here = 此平台部分支持——演示可以运行，但并非平台的原生实现。
vectors_live_tint = 实时着色
vectors_cycle_tint = 切换

# Layout page (docs/size-classes.md "Row fit policies")
layout_note = 选择一种策略，然后添加组件，直到一行放不下。
layout_row_section = 行适配
layout_fit_label = 适配策略
layout_fit_clip = 裁剪
layout_fit_wrap = 换行
layout_fit_wrap_columns = 等宽列
layout_fit_column = 竖排
layout_fit_scroll = 滚动
layout_count_label = 组件数
layout_item = 项目 { $n }
layout_item_wide = 项目 { $n }（较宽）

vectors_alias_note = 没有粗体版本：绘制基础图形
vectors_pick_tint = 选择着色

# 内容列表页面（src/pages/content_list）：`day new` 脚手架自带的条目列表与编辑器，
# 因此下面的键就是脚手架的键，原样保留——复制的代码按名称读取它们。
nav_content_list = 内容列表
nav_items = 条目
cmd_add = 新建条目
cmd_delete = 删除
items_empty = 暂无条目。添加你的第一个条目吧。
item_none = 请选择要编辑的条目。
item_untitled = 无标题
field_name = 名称
field_name_hint = 为该条目命名…
field_done = 完成

# 光标页（day/docs/cursor.md）：`.cursor()` 装饰器，逐个形状。
# 分割页（day/docs/split.md）：两个窗格、一条分隔线、两侧读取同一状态。
nav_split = 分割
split_support_title = 在此工具包上
split_support_label = 分隔线由谁绘制
split_native = 工具包自带的分割器
split_composed = Day（此工具包没有自带分割器）
split_hint = 拖动分隔条，或移动占比滑块。
split_arrangement_title = 排列
split_stacked = 上下堆叠
split_share = 表单占比
split_form_title = 图形
split_caption = 标题
split_caption_placeholder = 绘制在图形之上
split_count = 数量
split_radius = 半径
split_hue = 色相
split_filled = 填充
split_shape = 形状
split_shape_circle = 圆形
split_shape_square = 方形
split_shape_diamond = 菱形
split_readout = { $count } 个半径为 { $radius } 的图形

nav_cursors = 光标
cursors_support_title = 在此工具包上
cursors_support_label = 此处支持
cursors_native = 原生
cursors_emulated = 模拟（取最接近的形状）
cursors_unsupported = 此目标不支持
cursors_hint = 把指针移到任一方块上。
cursors_shapes_title = 指针形状
cursors_drag_title = 拖放
cursors_resize_title = 调整大小与缩放
cursors_busy_title = 忙碌
cursors_reactive_title = 一个方块，多种形状
cursors_reactive_pick = 形状
cursors_nested_title = 最近的祖先优先
cursors_nested_outer = 这里是十字线
cursors_nested_inner = 这里是手形
cursors_native_title = 本工具包独有
cursors_native_none = 本工具包没有共享集之外的形状。

# 权限说明：应用请求权限时 iOS 和 HarmonyOS 显示的文字。
permission_camera = 在“相机”页面预览并拍照。
permission_location_when_in_use = 在“设备与传感器”页面实时显示您的坐标。
permission_motion = 在“设备与传感器”页面实时显示加速度计、陀螺仪和磁力计曲线。
permission_NSBluetoothAlwaysUsageDescription = 演示 Day 如何请求蓝牙权限——展示应用从不连接任何设备。

# Camera (src/pages/camera.rs): the viewfinder, its permission, and the last photo.
nav_camera = 相机
camera_permission_section = 权限
camera_permission_label = 相机访问
camera_viewfinder_section = 取景器
camera_take_photo = 拍照
camera_flip = 翻转
camera_state_label = 相机
camera_last_photo_section = 最近的照片
camera_photo_label = 照片
camera_no_photo = 暂无
camera_photo_size = { $width } × { $height } · { $name }

list_activated = 上次激活的行：{ $row }（0 表示无）

ctl_icon_label = 图标和文字
ctl_icon_only = 仅图标
ctl_image = 内置图像

nav_drag_drop = 拖放
dnd_hint = 将图块拖到另一个图块或另一个 Showcase 窗口。带 X 的圆圈拒绝放入。
dnd_ready = 等待拖入内容。
dnd_empty = 放置于此
dnd_new_window = 新建窗口
dnd_choose_file = 选择文件…
dnd_accept = 接受拖放

dnd_data = { $mime }：{ $count } 字节

files_reopen = 重新打开上次的文件
files_bookmark_failed = 无法恢复对已保存文件的访问权限。请使用“打开”重新选择文件。

dnd_disallowed = 添加禁止放置区域

# Native display frame demo
frame_hint = 轻点抛出弹珠，拖动一颗可将它甩出。
frame_bounce = 弹跳
frame_shuffle = 随机
frame_pause = 暂停
frame_waiting = 等待下一帧
frame_running = 运动中
frame_paused = 已暂停
frame_resting = 静止
frame_balls_label = 弹珠
frame_fps = FPS · 最低 { $min } · 最高 { $max } · 平均 { $avg }

commands_title = 可复用命令
commands_add = 加一

# The Physics page (src/pages/physics.rs): the ball mass and planetary gravity sliders, and the
# bodies the gravity readout names.
physics_gravity_label = 重力
physics_gravity = { $value } m/s²
physics_gravity_named = { $value } m/s²（{ $body }）
physics_mass_label = 质量
physics_mass = { $value } 百万吨
physics_elasticity_label = 弹性
physics_elasticity = { $value }%
physics_fullscreen = 全屏
physics_exit_fullscreen = 退出全屏
physics_zero_g = 零重力
physics_moon = 月球
physics_mars = 火星
physics_earth = 地球
physics_jupiter = 木星

# HTTP sessions, interception and payload statistics.
net_transport = 传输
net_provider = HTTP 提供程序
net_native = 原生
net_reqwest = reqwest
net_native_only = 此目标使用原生提供程序。
net_provider_note = 对新的请求和套接字生效。
net_simulation = 网络模拟
net_intercept = 拦截请求
net_speed = 传输速度
net_speed_value = 每次传输 { $speed } KiB/s
net_failures = 故障率
net_failure_value = { $rate }% 的请求
net_pause = 暂停交付
net_offline = 离线
net_method = 方法
net_url = URL
net_body = 请求正文
net_cancel = 取消
net_cancelled = 已取消
net_download_preset = 下载预设
net_upload_preset = 上传预设
net_received = 已接收 { $bytes } 字节
net_error = 请求失败：{ $error }
net_request = HTTP 请求
net_ws_note = 输入实际 WebSocket URL，或启用拦截使用模拟回显。
net_message = 消息
net_connected = 已连接
net_closed = 已关闭
net_real_down = 下载 · 实际
net_real_up = 上传 · 实际
net_sim_down = 下载 · 模拟
net_sim_up = 上传 · 模拟
net_seconds = 秒
net_rate = KiB/s
net_series = 流量
net_totals = 发送 { $sent } 字节 · 接收 { $received } 字节 · 活动 { $active } · 失败 { $failed }
net_average = 会话平均：上传 { $upload } KiB/s · 下载 { $download } KiB/s
net_statistics = 实时流量
net_send_request = 发送请求
net_options = 请求选项
net_headers = 标头 (JSON)
net_timeout = 空闲超时
net_timeout_value = { $seconds } 秒

# SI units used by the motion-sensor readouts.
sensor_unit_acceleration = m/s²
sensor_unit_rotation = rad/s
sensor_unit_magnetic = µT

sensor_live = 实时读数
sensor_compass = 磁罗盘
sensor_compass_hint = 将手机平放。红色指向磁北。
sensor_north = 北
sensor_east = 东
sensor_south = 南
sensor_west = 西
sensor_heading = 磁航向：{ $degrees }°

sensor_calibrated = 已校准
sensor_tilt = X/Y 加速度水平仪
sensor_tilt_hint = 气泡跟随 X/Y 加速度，边缘为 1 g。
sensor_rotation = 转速表
sensor_rotation_hint = 各轴的旋转速度，两端为 ±π rad/s。
sensor_uncalibrated_hint = 未校准：指针可能不指向北方。
sensor_axis_x = X
sensor_axis_y = Y
sensor_axis_z = Z

# --- 安全：钥匙串与生物识别提示（docs/keychain.md、docs/biometrics.md）---
nav_security = 安全
keychain_title = 钥匙串
keychain_support_label = 此处的存储
keychain_backend_label = 后端
keychain_key_label = 键
keychain_key_placeholder = showcase.session
keychain_secret_label = 密文
keychain_secret_placeholder = 值得保护的内容
keychain_protection_label = 保护级别
keychain_level_unlocked = 解锁时
keychain_level_first_unlock = 首次解锁后
keychain_level_presence = 用户在场
keychain_save = 保存
keychain_read = 读取
keychain_read_presence = 验证后读取
keychain_remove = 移除
keychain_idle = 尚未保存任何内容
keychain_saved = 已保存 { $key }
keychain_read_value = 读回：{ $value }
keychain_removed = 已移除 { $key }
keychain_entries_label = 已保存的键
keychain_entries_none = 无
keychain_presence_reason = 读取“安全”页面保存的密文。
biometrics_title = 生物识别
biometrics_support_label = 此处的支持
biometrics_availability_label = 可用性
biometrics_biometry_label = 传感器
biometrics_reason_placeholder = 请确认是您本人以继续。
biometrics_credential_label = 允许使用设备密码
biometrics_authenticate = 认证
biometrics_cancel = 取消
biometrics_idle = 尚未请求
biometrics_prompting = 等待提示…
biometrics_verified = 已验证（{ $method }）
permission_biometrics = 在“安全”页面确认是您本人。

# --- 登录：day-part-auth（认证浏览器、OpenID Connect、通行密钥、Apple、Google）---
nav_signin = 登录
signin_oidc_title = OpenID Connect
signin_provider_demo = 演示提供方（内置）
signin_provider_google = Google
signin_provider_apple = Apple（网页流程）
signin_provider_custom = 任意发行方（发现）
signin_issuer_placeholder = 发行方 URL，登录时进行发现
signin_client_id_placeholder = 客户端 ID，与在提供方注册的一致
passkeys_rp_placeholder = 依赖方（与本应用关联的域名）
signin_sign_in = 登录
signin_sign_in_silent = 静默登录
signin_refresh = 刷新
signin_userinfo = Userinfo
signin_revoke = 撤销
signin_sign_out = 退出登录
signin_idle = 未登录
signin_working = 等待提供方…
signin_signed_in = 已以 { $name }（{ $email }）登录，主体 { $sub }
signin_tokens = 访问令牌有效 { $secs } 秒 · 范围 { $scope } · { $refresh }
signin_refresh_yes = 已签发刷新令牌
signin_refresh_no = 无刷新令牌
signin_need_client_id = 请先粘贴你在提供方注册的客户端 ID。
signin_need_discovery = 请先填写发行方和客户端 ID；登录时进行发现。
signin_silent_demo_only = 静默登录仅适用于演示提供方；其他提供方需要其同意页面。
signin_nothing_to_refresh = 请先登录：没有可交换的刷新令牌。
signin_refreshed = 已刷新：签发了新的访问令牌
signin_userinfo_ok = Userinfo 已回答 { $sub }
signin_revoked = 访问令牌已撤销
signin_signed_out = 已退出登录
passkeys_title = 通行密钥
passkeys_available_yes = 平台认证器可用
passkeys_available_no = 此设备没有平台认证器
passkeys_create = 创建通行密钥
passkeys_get = 用通行密钥登录
passkeys_idle = 尚无凭据
passkeys_prompting = 等待认证器…
passkeys_created = 已创建凭据 { $id }
passkeys_asserted = 已用凭据 { $id } 登录
apple_title = 通过 Apple 登录
apple_sign_in = 通过 Apple 登录
apple_signed_in = Apple 用户 { $user }
google_title = Google
google_client_placeholder = 服务器客户端 ID
google_sign_in = 通过 Google 登录
google_signed_in = Google 账户 { $id }

# Numeric notification count.
badge_count = { $count }

# Map (src/pages/map.rs): day-piece-map over the built-in sample island or a live OpenFreeMap
# style, the camera calls on buttons, and a readout of what the piece reports.
nav_map = 地图
map_source_sample = 示例岛屿
map_source_liberty = OpenFreeMap Liberty
map_source_positron = OpenFreeMap Positron
map_source_bright = OpenFreeMap Bright
map_source_fiord = OpenFreeMap Fiord
map_source_dark = OpenFreeMap Dark
map_source_colorful = VersaTiles Colorful
map_source_colorful_dark = VersaTiles Colorful Dark
map_fly = 飞往
map_fit = 全览
map_rotate = 旋转
map_north = 正北
map_sight = { $sight ->
        [center] 岛屿中心
        [coast] 东海岸
        [trafalgar] 特拉法加广场
        [here] 你的位置
       *[bridge] 塔桥
    }
map_readout = { $status ->
        [ready] 就绪
        [failed] 失败：{ $reason }
       *[loading] 正在加载
    } · { $lat }, { $lng } · 缩放 { $zoom } · { $bearing ->
        [0] 上北
       *[other] 方位 { $bearing }°
    }
map_tap_prompt = 轻点地图
map_tap_marker = 标记：{ $sight ->
        [center] 岛屿中心
        [coast] 东海岸
        [trafalgar] 特拉法加广场
        [here] 你的位置
       *[bridge] 塔桥
    }
map_tap_feature = { $layer }：{ $name }
map_tap_layer = { $layer }
map_tap_nothing = 此处无内容
map_locate = 当前位置
map_locating = 正在查找你的位置…
map_locate_failed = { $why ->
        [denied] 此应用的位置访问已关闭。请在设置中开启。
        [off] 定位服务已关闭。
        [timeout] 尚未获得定位，请重试。
        [unavailable] 此设备无法使用定位。
       *[other] 无法找到你的位置。
    }

# Interactive layout sizing and translation.
layout_reactive_title = 响应式尺寸
layout_reactive_width = 宽度
layout_reactive_height = 高度
layout_reactive_horizontal = 水平偏移
layout_reactive_vertical = 垂直偏移
layout_reactive_reset = 重置
layout_reactive_points = { NUMBER($value, maximumFractionDigits: 0) } 点
layout_reactive_shape = 形状：宽 { NUMBER($width, maximumFractionDigits: 0) } 点，高 { NUMBER($height, maximumFractionDigits: 0) } 点

push_caption = 通过 Day 中继服务向此设备发送一条测试推送。
push_unavailable = 此版本不支持推送通知。
push_send_now = 立即发送
push_send_later = { $seconds } 秒后发送
push_hint = 首次发送时会请求权限。
push_sending = 正在发送…
push_requested = 正在等待推送服务提供设备令牌…
push_no_token = 推送服务未提供设备令牌。请稍后重试。
push_denied = 此应用的通知已关闭。请在“设置”中允许通知，然后重新发送。
push_too_soon = 中继服务每 30 秒只允许发送一次测试。请稍候再发送。
push_network_busy = 过去一小时内，中继服务已从此网络注册了大量设备。请稍后再试。
push_accepted = 已发送，稍后即可收到。
push_scheduled = 将在 { $seconds } 秒后送达。离开应用即可在后台看到它。
push_received = 应用打开时收到：{ $title }
push_opened = 从通知打开：{ $title }
push_advanced = 高级
push_token = 设备令牌
push_token_apns = 已从 APNs 收到
push_token_fcm = 已从 FCM 收到
push_token_none = 尚未收到
push_relay = 中继服务注册
push_relay_none = 未注册
push_sandbox = 使用 APNs 沙盒（开发版本）
push_waiting = 尚未收到通知
push_remove = 从中继服务移除
push_removed = 设备已从中继服务移除
push_error = 推送失败：{ $error }

# 分享、分组列表及应用级返回示例。
list_opt_inset_grouped = 内缩分组
list_opt_inset_hint = macOS 上显示内缩行，iOS 上显示圆角分组。其他平台保持普通列表样式。
share_demo_title = 分享
share_demo_fallback = 此平台不显示自定义分享操作。请使用下方按钮。
share_demo_open = 分享 Day 网站…
share_demo_remember = 记住链接
share_demo_clear = 清除结果
share_demo_remembered = 已选择“记住链接”操作
share_demo_cleared = 已清除分享结果
share_demo_failed = 无法打开分享菜单。
share_demo_receive_hint = 从其他应用向 Day Showcase 分享文本或链接。
share_demo_none = 尚未收到内容。
system_back_status = 应用返回：剩余 { $remaining } 次，已处理 { $handled } 次
system_back_press = 返回
system_back_stop = 停用
system_back_hint = 已推入的页面和覆盖层先于应用处理返回。
system_back_arm = 启用 3 次返回

# Map provider selection and setup.
map_provider_osm = OpenStreetMap
map_provider_apple = Apple Maps
map_provider_google = Google Maps
map_missing_api_key = 请为此 Android 应用配置 Google 地图 API 密钥。
map_play_services_unavailable = Google 地图需要 Google Play 服务。
map_load_failed = 无法加载地图。请检查网络连接和提供商配置。

resources_missing = 打包的资源不可用：{ $name }

# The gallery pages added with the card layout, and the line under every page's title.
nav_styles = 样式
nav_images = 图片
nav_gestures = 手势
detail_controls = 按钮、开关、滑块、步进器、选择器与进度
detail_dates = 日期与时间选择器，使用平台的日历
detail_focus = 把键盘焦点作为应用状态
detail_text = 字体、带样式的文本片段、Markdown、选择与截断
detail_text_areas = 多行编辑、代码模式与实时 Markdown
detail_text_fields = 单行输入、键盘类型、安全输入与搜索
detail_toolbars = 窗口与页面命令，显示在平台的工具栏中
detail_localization = 四种语言、复数形式与从右到左布局
detail_canvas = 路径、渐变、文字与图像，每帧绘制
detail_styles = 圆角、边框、阴影、裁剪与变换
detail_images = 适配模式、圆角、远程图片、占位图与淡入
detail_gestures = 在任意视图上拖动、捏合、旋转、按压与悬停
detail_charts = 一套图形标记语法：折线、柱形、面积与选区
detail_animation = 平台动画、补间、缓动与弹簧
detail_physics = 帧时钟驱动的弹珠与弹簧
detail_benchmark = 由实时视图拼成的画面，与 SwiftUI 对比计时
detail_grid = 固定列、跨列、对齐与尺寸
detail_layout = 行、列、层叠、换行与尺寸类别
detail_split = 可调整大小的窗格，使用平台自带的分割器
detail_list = 行复用、分区、滑动、重新排序与选择
detail_tree = 可展开、选择、重新排序并按需加载的大纲
detail_drag_drop = 在视图与应用之间拖放文字、文件和图片
detail_model = 类型化存储、派生值与撤销
detail_query = 实时查询的数据库，支持搜索与排序
detail_tabs = 标签栏与分段页面，各自保留状态
detail_stack = 推入的页面、由你掌控的路径，以及返回手势
detail_content_list = 列表、条目与详情，分三栏显示
detail_media = 原生视频播放器，带自己的播放控件
detail_lottie = 矢量动画，可播放、暂停和拖动进度
detail_map = 画布上的矢量瓦片，与平台地图并列
detail_web_view = 打包的站点与实时网页，JavaScript 双向调用
detail_menus = 菜单、上下文菜单、警告框、面板与输入提示
detail_system = 电池、传感器、定位、设备信息与外观
detail_network = 请求、上传、下载、代理与连接状态
detail_notify = 本地通知与应用角标
detail_push = 通过可试用的中继服务发送远程通知
detail_speech = 语音、音效与触感反馈
detail_camera = 实时预览、拍摄与返回的照片
detail_cursors = 平台提供的所有指针形状
detail_files = 剪贴板、偏好设置、文件选择器与应用存储
detail_security = 钥匙串与生物识别
detail_sign_in = 通过 Apple、Google、通行密钥或任意 OpenID 提供方登录
detail_scripting = 驱动本应用并为其截图的脚本
detail_resources = 打包的图片、矢量图标、字体与数据文件
detail_tweaks = 针对各工具包的原生视图微调
detail_crash_reporting = 捕获崩溃，并在下次启动时报告
detail_about = Day 是什么，以及这个应用展示了什么
detail_settings = 外观、语言与应用自身的偏好设置
