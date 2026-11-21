# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Text page's newer cards (src/pages/text.rs). Its other strings are in app.ftl.

# Styled runs: the names of the styles, each drawn in its style.
text_run_code = code
text_run_color = couleur
text_run_highlight = surlignage
text_run_larger = plus grand
text_underline_single = simple
text_underline_double = double
text_underline_dotted = pointillé
text_underline_wavy = ondulé

# Relative size: a semantic style and the factor it is scaled by.
text_scale_header = Taille relative
text_scale_of = { $style } × { $scale }

# Presets: two readings that share three named styles.
text_preset_header = Préréglages
text_preset_moon = De la Terre à la Lune
text_preset_moon_value = 384 400
text_preset_km = km
text_preset_sun = De la Terre au Soleil
text_preset_sun_value = 149,6
text_preset_million_km = millions de km

# Line limits and alignment share one paragraph.
text_sample_paragraph = La lumière du Soleil met environ huit minutes pour atteindre la Terre, plus de quatre heures pour atteindre Neptune et plus de quatre ans pour atteindre l’étoile la plus proche.
text_lines_header = Limites de lignes
text_lines_one = Une ligne
text_lines_two = Deux lignes
text_lines_all = Sans limite
text_align_header = Alignement
text_align_leading = Au début

# Scripts and direction: the names of the specimens.
text_scripts_header = Écritures et sens de lecture
text_scripts_cjk = Chinois, japonais et coréen
text_scripts_rtl_in_ltr = Arabe et hébreu dans du latin
text_scripts_ltr_in_rtl = Latin dans de l’arabe

# Decorations and font features.
text_decorations_header = Décorations et chiffres
text_small_caps = Petites capitales
text_small_caps_sample = Centre de contrôle, Houston
text_figures = Chiffres

# The reader's text size, and a badge that keeps its size.
text_scale_reading = Taille du texte : { $scale }×
text_scale_fixed = Taille fixe

# Line limits: how many lines, and where the ellipsis goes.
text_lines_count = Lignes
text_lines_truncate = Points de suspension
text_trunc_head = Début
text_trunc_middle = Milieu
text_trunc_tail = Fin

# Letter spacing and line height.
text_spacing_header = Espacement
text_spacing_letters = Lettres
text_spacing_lines = Lignes
text_align_justified = Justifié

# A row that reads in the direction the picker names.
text_direction = Sens
text_direction_ltr = De gauche à droite
text_direction_rtl = De droite à gauche
text_direction_first = 1 Lancement
text_direction_second = 2 Orbite
text_direction_third = 3 Atterrissage

# Links and selection.
text_link_handled_label = Géré par l’app
text_selection_none = Sélectionnez une partie du texte ci-dessus.
text_selection_value = Sélection : { $text }

# The Markdown view: a reply streamed in word by word.
text_md_view_header = Vue Markdown
text_md_stream = Diffuser une réponse
text_md_copy = Copier
text_md_copied = Code copié
text_md_reply =
    ## Contrôle d’orbite

    La sonde envoie **trois** mesures à chaque passage :

    - altitude
    - vitesse
    - batterie

    Sa période découle de la taille de l’orbite :

    ```rust
    let period = 2.0 * PI * (a.powi(3) / mu).sqrt();
    ```

    Planifiez la prochaine poussée quand les mesures se stabilisent.
