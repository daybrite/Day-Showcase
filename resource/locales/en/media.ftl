# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Media page's cards (src/pages/media.rs).

media_clip = Hurricane Milton from the space station · NASA
media_clip_title = Hurricane Milton from the space station
media_clip_credit = NASA
media_played = Played
media_buffered = Buffered
media_buffered_unavailable = This toolkit reports no buffered ranges.
media_playback_section = Playback
media_rate = Speed
media_volume = Volume
media_mute = Mute
media_volume_fixed = This platform's video has mute only.
media_system_section = System controls
media_now_playing = Now playing
media_remote_title = Last command
media_system_note = The system's media controls and media keys show this video and control it.
media_readout_section = Readout
media_stream_title = Stream title
media_error_title = Error
media_none = None
media_state = { $state ->
    [idle] Idle
    [loading] Loading
    [playing] Playing
    [paused] Paused
    [ended] Ended
   *[error] Failed
}
media_error = { $kind ->
    [invalid] Invalid value: { $operation }
    [unsupported] Not supported here: { $operation }
    [playback] Playback failed
   *[command] Command failed: { $operation }
}
media_remote = { $command ->
    [play] Play
    [pause] Pause
    [toggle] Play or pause
    [stop] Stop
    [next] Next
    [previous] Previous
    [seek] Seek
    [forward] Skip forward
    [backward] Skip back
   *[none] None
}
