# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Gestures page's cards (src/pages/gestures.rs). The names of Day's APIs stay as they are
# in every language.

# Drag, pinch and rotate: a picture that follows three gestures at once.
gestures_transform_title = Glisser, pincer et pivoter
gestures_transform_subtitle = Trois gestes écrits dans une seule transformation
gestures_transform_stage_label = Scène avec une image de Saturne à faire glisser, pincer et pivoter
gestures_transform_offset = Décalage
gestures_transform_scale = Échelle
gestures_transform_angle = Angle
gestures_transform_hint_touch = Faites glisser avec un doigt. Pincez et tournez avec deux.
gestures_transform_hint_pointer = Faites glisser avec la souris. Pincez et pivotez sur un pavé tactile.
gestures_reset = Réinitialiser

# Pointer: what on_drag, on_touches, on_tap_at and on_hover report, drawn and in figures.
gestures_pointer_title = Pointeur
gestures_pointer_subtitle = Ce que rapportent on_drag, on_touches, on_tap_at et on_hover
gestures_pointer_stage_label = Scène qui dessine les rapports du pointeur
gestures_pointer_hint_touch = Touchez, faites glisser et relâchez
gestures_pointer_hint_pointer = Survolez, cliquez, ou faites glisser et relâchez
gestures_pointer_phase = Phase
gestures_pointer_location = Position
gestures_pointer_contacts = Contacts
gestures_pointer_translation = Translation
gestures_pointer_speed = Vitesse
gestures_pointer_speed_value = { $speed } pt/s
gestures_pointer_waiting = En attente
gestures_pointer_began = Début
gestures_pointer_changed = Changement
gestures_pointer_ended = Fin
gestures_pointer_hover = Survol
gestures_pointer_tap = Toucher
gestures_pointer_one_contact = Cette plateforme rapporte un contact à la fois.

# Tap, double tap and hold: Day's press gestures on one pad.
gestures_press_title = Toucher, double toucher et appui long
gestures_press_subtitle = on_tap, on_double_tap, on_long_press et on_pressed
gestures_press_pad_label = Zone qui compte les touchers et les appuis longs
gestures_press_pad_touch = Touchez, touchez deux fois, ou maintenez jusqu’à ce que l’anneau se ferme.
gestures_press_pad_pointer = Cliquez, cliquez deux fois, ou maintenez jusqu’à ce que l’anneau se ferme.
gestures_press_taps = Touchers
gestures_press_doubles = Doubles touchers
gestures_press_holds = Appuis longs
gestures_press_derived_note = Ici, Day déduit l’appui des glissements : bougez légèrement pendant l’appui.

# Swipe: a row that drags along one axis inside the page's vertical scroll.
gestures_swipe_title = Balayage
gestures_swipe_subtitle = on_drag_along dans un défilement vertical
gestures_swipe_row_title = Balayez cette ligne
gestures_swipe_row_touch = Balayez sur le côté. Vers le haut ou le bas, la page défile.
gestures_swipe_row_pointer = Faites glisser sur le côté et relâchez.
gestures_swipe_count = Balayages
gestures_swipe_release = Vitesse au lâcher

# Hit testing: a dot with a larger tap area, and a button under a canvas with no handlers.
gestures_hit_title = Test d’atteinte
gestures_hit_subtitle = Où arrive un toucher
gestures_slop_target_label = Point avec une zone tactile plus grande
gestures_slop_caption = Un point de { $dot } pt dans une zone tactile de { $area } pt
gestures_slop_unsupported = Un point de { $dot } pt, atteint seulement dans ses propres limites ici
gestures_slop_count = { $count ->
    [one] { $count } toucher
   *[other] { $count } touchers
}
gestures_through_button = Sous le canevas
gestures_through_caption = Sur iOS et Android, un canevas sans gestionnaire ne reçoit aucun toucher.
