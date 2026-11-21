# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Animation page (src/pages/animation.rs). The names of Day's APIs stay as they are in
# every language.

# Easing curves: one curve of each Easing variant, named after the variant.
animation_easing_title = Easing curves
animation_easing_subtitle = Every Easing variant on one clock
animation_easing_play = Run
animation_curve_linear = Linear
animation_curve_bezier = Bézier
animation_curve_power = Power
animation_curve_sine = Sine
animation_curve_circle = Circle
animation_curve_exponential = Exponential
animation_curve_back = Back
animation_curve_elastic = Elastic
animation_curve_bounce = Bounce

# Platform animator: the playground. Its controls' labels are the anim_* keys in app.ftl.
animation_play_title = Platform animator
animation_play_subtitle = Six properties in one with_animation

# Animated values: a planet that glides between four sets of figures.
animation_tween_title = Animated values
animation_tween_subtitle = Size, distance and color glide together
animation_tween_mercury = Mercury
animation_tween_earth = Earth
animation_tween_jupiter = Jupiter
animation_tween_neptune = Neptune
animation_tween_diameter = Diameter
animation_tween_distance = From the Sun
animation_tween_km = { $km } km
animation_tween_au = { $au } AU

# Interpolation: one value that a tile's place, size, turn and color are mapped from.
animation_interp_title = Interpolation
animation_interp_subtitle = One value drives place, size, turn and color
animation_interp_value = Value
animation_interp_release = Released at
animation_interp_speed = { $speed } pt/s
animation_interp_flip = Flip

# Spring: one spring set by three sliders, plotted and run.
animation_springs_title = Spring
animation_springs_subtitle = Drag the knob and let go
animation_springs_stiffness = Stiffness
animation_springs_damping = Damping
animation_springs_mass = Mass
animation_springs_release = Launch
animation_springs_ratio = ζ { $ratio }

# Choreography: bars moved by composed steps.
animation_compose_title = Choreography
animation_compose_subtitle = Composed steps on the frame clock
animation_compose_sequence = Sequence
animation_compose_parallel = Parallel
animation_compose_stagger = Stagger
animation_compose_repeat = Repeat
animation_compose_play = Start
animation_compose_stop = Stop
animation_compose_reset = Reset

# Implicit animation: a heading and a button over a scrolling log. A sol is a day on Mars.
animation_header_title = Implicit animation
animation_header_subtitle = A heading and a button that follow the scroll
animation_header_row = Sol { $n }
animation_header_top = Top
animation_header_heading = Mission log
animation_header_offset = { $y } pt
animation_header_untracked = No offset reported

# Reflow: keyed tiles whose layout changes run inside with_animation.
animation_layout_title = Reflow
animation_layout_subtitle = Layout changes inside with_animation
animation_layout_tiles = Tiles
animation_layout_size = Size
animation_layout_small = Small
animation_layout_medium = Medium
animation_layout_large = Large
animation_layout_shuffle = Shuffle
