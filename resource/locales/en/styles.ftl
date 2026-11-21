# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Styles page: a surface, corners, outlines, gradients, shadows, layers, transforms, digits and direction.

# One word beside a specimen that is drawn and not a view.
styles_canvas = Canvas
styles_points = { NUMBER($n, maximumFractionDigits: 0) } pt
styles_degrees = { NUMBER($n, maximumFractionDigits: 0) }°
styles_percent = { NUMBER($p, style: "percent") }

styles_surface_title = Surface
styles_surface_subtitle = Fill, corners, border and shadow on one view
styles_surface_fill = Fill
styles_surface_solid = Solid
styles_surface_linear = Linear
styles_surface_radial = Radial
styles_surface_conic = Conic
styles_surface_corners = Corners
styles_surface_all = All
styles_surface_bottom = Bottom
styles_surface_radius = Radius
styles_surface_border = Border
styles_surface_shadow = Shadow

styles_corners_title = Rounded corners
styles_corners_subtitle = Radius tokens, named corners and capsules
styles_corners_radius = Radius tokens
styles_corners_top = Top
styles_corners_right = Right
styles_corners_diagonal = Diagonal
styles_corners_one = Single corner
styles_corners_four = Four radii
styles_corners_capsule = Capsule

styles_borders_title = Outlines
styles_borders_subtitle = Borders on views, dashes on a canvas
styles_borders_weights = Weights
styles_borders_rounded = Along the corners
styles_borders_piece = Around a piece
# A catalog designation, the same in every language.
styles_borders_sample = M31
styles_borders_ring = Ring
styles_borders_overlay = Over a gradient
styles_borders_dashes = Dashes and dots

styles_gradients_title = Gradients
styles_gradients_subtitle = Linear, radial and conic fills on views
styles_gradients_vertical = Vertical
styles_gradients_horizontal = Horizontal
styles_gradients_clear = Fade to clear
styles_gradients_radial = Radial
styles_gradients_conic = Conic

styles_shadows_title = Shadows
styles_shadows_subtitle = Cast by a surface, in its shape
styles_shadows_blur = Blur
styles_shadows_glow = Halo
styles_shadows_offset = Offset
styles_shadows_shaped = Named corners
# A catalog designation, the same in every language.
styles_shadows_sample = M42
styles_shadows_lift = Lift
styles_shadows_image = Under a picture

styles_layers_title = Layers
styles_layers_subtitle = Clipping, order and opacity
styles_layers_clip = Clipping
styles_layers_clipped = Clipped
styles_layers_visible = Overflowing
styles_layers_stack = Order
styles_layers_front = Front
styles_layers_opacity = Opacity

styles_transforms_title = Transforms
styles_transforms_subtitle = Rotation and scale about an anchor
styles_transforms_rotate = Rotate
styles_transforms_scale = Scale
styles_transforms_anchor = Anchor
styles_transforms_center = Center
styles_transforms_corner = Corner
styles_transforms_orbit = Orbit
styles_transforms_shear = Shear
styles_transforms_reset = Reset

styles_numerals_title = Tabular digits
styles_numerals_subtitle = Distance from the Sun, million km
styles_numerals_proportional = Default
styles_numerals_tabular = Tabular
styles_numerals_figure = { NUMBER($n, minimumFractionDigits: 2) }

styles_planet_mercury = Mercury
styles_planet_venus = Venus
styles_planet_earth = Earth
styles_planet_mars = Mars
styles_planet_jupiter = Jupiter
styles_planet_saturn = Saturn
styles_planet_uranus = Uranus
styles_planet_neptune = Neptune

styles_rtl_title = Layout direction
styles_rtl_subtitle_ltr = This language reads left to right
styles_rtl_subtitle_rtl = This language reads right to left
styles_rtl_leading = Leading
styles_rtl_trailing = Trailing
# Two scripts on one line: the Milky Way, in this language and in Arabic or English.
styles_rtl_mixed = Milky Way · درب التبانة
