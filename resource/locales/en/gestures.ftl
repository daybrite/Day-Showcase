# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Gestures page's cards (src/pages/gestures.rs). The names of Day's APIs stay as they are
# in every language.

# Drag, pinch and rotate: a picture that follows three gestures at once.
gestures_transform_title = Drag, pinch and rotate
gestures_transform_subtitle = Three gestures written into one transform
gestures_transform_stage_label = Stage with a picture of Saturn to drag, pinch and rotate
gestures_transform_offset = Offset
gestures_transform_scale = Scale
gestures_transform_angle = Angle
gestures_transform_hint_touch = Drag with one finger. Pinch and turn with two.
gestures_transform_hint_pointer = Drag with the mouse. Pinch and rotate on a trackpad.
gestures_reset = Reset

# Pointer: what on_drag, on_touches, on_tap_at and on_hover report, drawn and in figures.
gestures_pointer_title = Pointer
gestures_pointer_subtitle = What on_drag, on_touches, on_tap_at and on_hover report
gestures_pointer_stage_label = Stage that draws pointer reports
gestures_pointer_hint_touch = Touch, drag and let go
gestures_pointer_hint_pointer = Hover, click, or drag and let go
gestures_pointer_phase = Phase
gestures_pointer_location = Location
gestures_pointer_contacts = Contacts
gestures_pointer_translation = Translation
gestures_pointer_speed = Speed
gestures_pointer_speed_value = { $speed } pt/s
gestures_pointer_waiting = Waiting
gestures_pointer_began = Began
gestures_pointer_changed = Changed
gestures_pointer_ended = Ended
gestures_pointer_hover = Hover
gestures_pointer_tap = Tap
gestures_pointer_one_contact = This platform reports one contact at a time.

# Tap, double tap and hold: Day's press gestures on one pad.
gestures_press_title = Tap, double tap and hold
gestures_press_subtitle = on_tap, on_double_tap, on_long_press and on_pressed
gestures_press_pad_label = Pad that counts taps and holds
gestures_press_pad_touch = Tap, tap twice, or hold until the ring closes.
gestures_press_pad_pointer = Click, click twice, or hold until the ring closes.
gestures_press_taps = Taps
gestures_press_doubles = Double taps
gestures_press_holds = Holds
gestures_press_derived_note = Day derives the press from drag events here, so move slightly while you hold.

# Swipe: a row that drags along one axis inside the page's vertical scroll.
gestures_swipe_title = Swipe
gestures_swipe_subtitle = on_drag_along inside a vertical scroll
gestures_swipe_row_title = Swipe this row
gestures_swipe_row_touch = Swipe sideways. Up and down still scrolls the page.
gestures_swipe_row_pointer = Drag sideways and let go.
gestures_swipe_count = Swipes
gestures_swipe_release = Release speed

# Hit testing: a dot with a larger tap area, and a button under a canvas with no handlers.
gestures_hit_title = Hit testing
gestures_hit_subtitle = Where a tap lands
gestures_slop_target_label = Dot with a larger tap area
gestures_slop_caption = A { $dot } pt dot in a { $area } pt tap area
gestures_slop_unsupported = A { $dot } pt dot, hit inside its own bounds here
gestures_slop_count = { $count ->
    [one] { $count } tap
   *[other] { $count } taps
}
gestures_through_button = Under the canvas
gestures_through_caption = On iOS and Android a canvas with no handlers takes no touches.
