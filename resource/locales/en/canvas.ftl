# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Canvas page (src/pages/canvas.rs).

canvas_dial_title = Turbine 3 load
canvas_dial_subtitle = Needle and sweep on one sprung value
canvas_dial_unit = % load
canvas_dial_idle = Idle
canvas_dial_cruise = Cruise
canvas_dial_peak = Peak
canvas_dial_zone = Zone
canvas_dial_zone_redline = Redline
canvas_dial_zone_high = High
canvas_dial_zone_nominal = Nominal
canvas_dial_spool = Spool
canvas_dial_spool_value = { $rpm } rpm
canvas_dial_limit = Limit

canvas_clock_title = Transforms
canvas_clock_subtitle = translate · rotate · save / restore
canvas_clock_ticks = Ticks
canvas_clock_ticks_note = 60 rotations
canvas_clock_hands = Hands
canvas_clock_hands_note = save, restore
canvas_clock_second = Second hand
canvas_clock_second_note = A FrameClock subscription

canvas_svg_title = SVG paths
canvas_svg_subtitle = Path data, parsed by build_path! at compile time
canvas_svg_heart = Heart
canvas_svg_star = Star
canvas_svg_bolt = Bolt
canvas_svg_check = Check

canvas_compositing_title = Compositing
canvas_compositing_subtitle = Clip, shadow, erased cut-outs, blend modes
canvas_compositing_image = Clipped image
canvas_compositing_cutout = Cut-out
canvas_compositing_radial = Radial gradient
canvas_compositing_blend = Blend modes
canvas_compositing_blend_code = Draw::blended
canvas_blend_normal = Normal
canvas_blend_multiply = Multiply
canvas_blend_screen = Screen
canvas_blend_add = Add

canvas_sketch_title = Sketch
canvas_sketch_subtitle = Drag → draw with on_drag
canvas_sketch_clear = Clear
canvas_sketch_empty = Draw here
canvas_sketch_strokes = Strokes
canvas_sketch_count = { $strokes } · { $points } points

canvas_text_subtitle = What measure_text reports, in any family
canvas_anchors_subtitle = Twelve TextAnchor placements on a crosshair
canvas_interaction_subtitle = on_hover · on_tap_at · on_drag
canvas_image_subtitle = decode_image, then Draw::image
