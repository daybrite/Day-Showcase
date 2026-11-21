# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Animation page (src/pages/animation.rs). The names of Day's APIs stay as they are in
# every language.

# Easing curves: one curve of each Easing variant, named after the variant.
animation_easing_title = Courbes d’accélération
animation_easing_subtitle = Chaque variante d’Easing sur une même horloge
animation_easing_play = Lancer
animation_curve_linear = Linéaire
animation_curve_bezier = Bézier
animation_curve_power = Puissance
animation_curve_sine = Sinus
animation_curve_circle = Cercle
animation_curve_exponential = Exponentielle
animation_curve_back = Recul
animation_curve_elastic = Élastique
animation_curve_bounce = Rebond

# Platform animator: the playground. Its controls' labels are the anim_* keys in app.ftl.
animation_play_title = Animateur de la plateforme
animation_play_subtitle = Six propriétés dans un seul with_animation

# Animated values: a planet that glides between four sets of figures.
animation_tween_title = Valeurs Animated
animation_tween_subtitle = Taille, distance et couleur glissent ensemble
animation_tween_mercury = Mercure
animation_tween_earth = Terre
animation_tween_jupiter = Jupiter
animation_tween_neptune = Neptune
animation_tween_diameter = Diamètre
animation_tween_distance = Distance au Soleil
animation_tween_km = { $km } km
animation_tween_au = { $au } ua

# Interpolation: one value that a tile's place, size, turn and color are mapped from.
animation_interp_title = Interpolation
animation_interp_subtitle = Une valeur pilote position, taille, rotation et couleur
animation_interp_value = Valeur
animation_interp_release = Relâché à
animation_interp_speed = { $speed } pt/s
animation_interp_flip = Basculer

# Spring: one spring set by three sliders, plotted and run.
animation_springs_title = Ressort
animation_springs_subtitle = Faites glisser la bille et relâchez
animation_springs_stiffness = Raideur
animation_springs_damping = Amortissement
animation_springs_mass = Masse
animation_springs_release = Lancer
animation_springs_ratio = ζ { $ratio }

# Choreography: bars moved by composed steps.
animation_compose_title = Chorégraphie
animation_compose_subtitle = Étapes composées sur l’horloge d’images
animation_compose_sequence = Séquence
animation_compose_parallel = Parallèle
animation_compose_stagger = Cascade
animation_compose_repeat = Boucle
animation_compose_play = Démarrer
animation_compose_stop = Arrêter
animation_compose_reset = Réinitialiser

# Implicit animation: a heading and a button over a scrolling log. A sol is a day on Mars.
animation_header_title = Animation implicite
animation_header_subtitle = Un titre et un bouton qui suivent le défilement
animation_header_row = Sol { $n }
animation_header_top = Haut
animation_header_heading = Journal de mission
animation_header_offset = { $y } pt
animation_header_untracked = Décalage non signalé

# Reflow: keyed tiles whose layout changes run inside with_animation.
animation_layout_title = Réagencement
animation_layout_subtitle = Changements de disposition dans with_animation
animation_layout_tiles = Tuiles
animation_layout_size = Taille
animation_layout_small = Petite
animation_layout_medium = Moyenne
animation_layout_large = Grande
animation_layout_shuffle = Mélanger
