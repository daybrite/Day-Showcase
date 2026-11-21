# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Media page's cards (src/pages/media.rs).

media_clip = 从空间站看飓风“米尔顿” · NASA
media_clip_title = 从空间站看飓风“米尔顿”
media_clip_credit = NASA
media_played = 已播放
media_buffered = 已缓冲
media_buffered_unavailable = 此工具包不报告缓冲范围。
media_playback_section = 播放
media_rate = 速度
media_volume = 音量
media_mute = 静音
media_volume_fixed = 此平台的视频只支持静音。
media_system_section = 系统控制
media_now_playing = 正在播放
media_remote_title = 最近的命令
media_system_note = 系统的媒体控制和媒体键会显示此视频并控制播放。
media_readout_section = 读数
media_stream_title = 流标题
media_error_title = 错误
media_none = 无
media_state = { $state ->
    [idle] 空闲
    [loading] 加载中
    [playing] 播放中
    [paused] 已暂停
    [ended] 已结束
   *[error] 失败
}
media_error = { $kind ->
    [invalid] 无效的值：{ $operation }
    [unsupported] 此处不支持：{ $operation }
    [playback] 播放失败
   *[command] 命令失败：{ $operation }
}
media_remote = { $command ->
    [play] 播放
    [pause] 暂停
    [toggle] 播放或暂停
    [stop] 停止
    [next] 下一项
    [previous] 上一项
    [seek] 跳转
    [forward] 快进
    [backward] 快退
   *[none] 无
}
