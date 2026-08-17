app_title = Vitrine de Day
counter_value = { $count ->
    [one] { $count } clic
   *[other] { $count } clics
}
decrement = −
increment = +
name_placeholder = Votre nom
value_label = Valeur
progress_label = Progression
flavor_label = Parfum
flavor_placeholder = Saisissez ou choisissez un parfum
flavor_add = Ajouter
flavor_ios_note = iOS n'a pas de contrôle combo box, Day affiche donc un espace réservé ici.
history_entry = le compteur est passé à { $value }
nav_controls = Contrôles
nav_menus = Menus et dialogues
nav_text = Texte
nav_battery = Batterie
nav_sensors = Capteurs
nav_clipboard = Presse-papiers
nav_network = Réseau
nav_media = Média
nav_compose = Composition
nav_files = Fichiers
nav_tabs = Onglets
nav_stack = Pile
nav_layout = Disposition
nav_list = Liste
nav_refresh = Actualiser
refresh_caption = Tirez le flux vers le bas — ou utilisez le bouton — pour recharger
refresh_status_idle = Inactif
refresh_status_refreshing = Actualisation…
refresh_now = Actualiser maintenant
refresh_tier_native = Tirer pour actualiser : natif
refresh_tier_emulated = Tirer pour actualiser : émulé
refresh_row = Élément { $n }
nav_webview = Vue Web
nav_lottie = Lottie
nav_about = À propos

shapes_kinds = Types
gradients_title = Dégradés
gradient_angle = Angle
shapes_angle = Angle

picker_selected = Sélection
picker_segmented = Segmenté
picker_menu = Menu
picker_inline = Aligné

# — day-piece-datetime —
nav_dates = Date et heure
dates_caption = Sélecteurs natifs de date et d'heure liés en double sens à des signaux civils — les sélecteurs d'une même section partagent le même signal.
dates_date_section = Date
dates_time_section = Heure
dates_composed_section = Composé
date_compact = Compact
time_compact = Compact
time_seconds = Avec les secondes
dates_composed = Date et heure
date_bounded = En 2026
date_picked = Date choisie
time_picked = Heure choisie

compose_caption = Pièces de pure composition — sans code natif, sans fonctionnalités cargo, sur tous les backends gratuitement.
compose_rating_label = Note en étoiles
compose_rating_count = Étoiles sélectionnées :
compose_rating_placeholder = 1–5
compose_card_title = Surface réutilisable
compose_card_body = Marge + arrière-plan + coins arrondis, appliqués comme Modificateur.
compose_plain_btn = Simple
compose_styled_btn = Rempli
compose_env_value = Teinté par l'accent fourni
list_add = Ajouter 100
list_caption = { $count } lignes — seules les cellules visibles sont créées
list_selection_none = Aucune ligne sélectionnée
list_selection = { $count ->
    [one] Ligne sélectionnée : { $rows }
   *[other] Lignes sélectionnées : { $rows }
}
list_clear_selection = Effacer la sélection
list_reorder_hint = Glissez pour réordonner, balayez pour supprimer ; la première ligne est épinglée
list_order = Premières lignes : { $rows }
list_shuffle = Mélanger
list_reset = Réinitialiser
list_delete = Supprimer

webview_url_hint = Saisir une URL
webview_go = Aller
webview_back = Précédent
webview_forward = Suivant
webview_stop = Arrêter
webview_reload = Recharger
webview_js_hint = JavaScript à exécuter dans la page
webview_js_run = Exécuter »
webview_js_result_hint = Le résultat JSON s’affiche ici
webview_note_iframe = Cette plateforme intègre la page dans une iframe. Le navigateur bloque l'historique et la lecture de l'URL entre origines : Précédent, Suivant et Arrêter sont indisponibles, et le champ ci-dessus ne suivra pas les liens ouverts dans la page.
webview_tab_remote = Distant
webview_tab_embedded = Intégré
webview_embedded_caption = Un site complet — pages, feuille de style, script, image — embarqué dans l'app sous resource/assets/web/minisite/, chargé sans réseau. Les liens relatifs restent dans le site ; les liens externes s'ouvrent hors de l'app ; les liens day-showcase:// sont interceptés et naviguent dans cette app.
webview_embedded_unsupported = Aucun moteur web n'est disponible dans cette version du toolkit.
webview_embedded_status_none = Aucun lien externe suivi pour l'instant.
webview_embedded_opened = Ouvert hors de l'app : { $url }
webview_embedded_intercepted = Intercepté — navigation dans l'app vers « { $route } ».

lottie_caption = Une animation Lottie native, fournie en JSON (lottie-ios / lottie-android)
lottie_speed = Vitesse
stack_root_body = Une vraie pile push/pop. Son chemin est un signal de l'application.
stack_push = Empiler un détail
stack_detail_title = Niveau { $depth }
stack_detail_body = Empilé sur le chemin. Le bouton retour natif réécrit le dépilement.
stack_item_title = Élément { $id }
stack_link_42 = Ouvrir item-42 avec un indice (route absolue)
stack_param_hint = Ouvert avec l'indice : {$hint}
tab_one = Aperçu
tab_two = Détails
tab_three = Réglages
tab_one_body = L'onglet aperçu. Chaque onglet conserve son propre état.
tab_two_body = L'onglet détails, sélectionné par sa clé de route.
tab_three_body = L'onglet réglages. Les liens profonds et dayscript choisissent les onglets par clé.
tab_state_note = Changez d'onglet puis revenez — l'état de cet onglet est conservé.
tab_detail_route = Clé de route
tab_detail_link = Lien profond
tab_set_badges = Afficher les badges
tab_set_sounds = Jouer les sons
about_text = Une application native multiplateforme construite avec day.
modal_alert = Afficher l'alerte
modal_confirm = Confirmer
modal_delete = Supprimer…
modal_sheet = Choisir un parfum
modal_prompt = Saisir le nom
alert_title = Avis
alert_body = Vos modifications ont été enregistrées.
ok = OK
confirm_title = Quitter ?
confirm_body = Voulez-vous vraiment quitter ?
delete_title = Supprimer l'élément ?
delete_body = Cette action est irréversible.
delete = Supprimer
flavor_title = Choisissez un parfum
cancel = Annuler
vanilla = vanille
pistachio = pistache

# Files playground (docs/files.md)
files_caption = Sélecteurs de fichiers natifs. « Ouvrir » lit un fichier texte dans l'éditeur ; « Enregistrer » l'écrit.
files_placeholder = Saisissez du texte à enregistrer…
files_open = Ouvrir un fichier…
files_save = Enregistrer le fichier…
files_opened = Ouvert : { $name }

# Battery playground (docs/battery.md)
battery_refresh = Lire la batterie
battery_level = Niveau
battery_charging = En charge
battery_reading = Batterie : { $percent } · { $state }
battery_reading_none = Batterie : aucune API batterie sur cette plateforme

# Aire de jeu Capteurs (docs/sensors.md)
sensor_accelerometer = Accéléromètre
sensor_gyroscope = Gyroscope
sensor_magnetometer = Magnétomètre
sensor_reading = x { $x } · y { $y } · z { $z } { $unit }
sensor_waiting = en attente du premier échantillon…
sensor_unavailable = indisponible sur cet appareil

# Aire de jeu Presse-papiers (docs/clipboard.md)
clipboard_caption = La part day-part-clipboard lit et écrit le presse-papiers système nativement.
clipboard_placeholder = Saisissez un texte à copier
clipboard_copy = Copier
clipboard_paste = Coller
clipboard_idle = Presse-papiers intact
clipboard_copied = Copié dans le presse-papiers système
clipboard_copy_failed = Échec de la copie (pas d'API presse-papiers ici)
clipboard_pasted = Collé depuis le presse-papiers système
clipboard_empty = Presse-papiers vide (ou illisible en arrière-plan)

# Aire de jeu Réseau (docs/network.md)
network_refresh = Lire le réseau
network_reading_online = En ligne · { $kind } · facturé : { $expensive }
network_reading_offline = Hors ligne
network_reading_none = Aucune API de connectivité sur cette plateforme

# Aire de jeu Média (docs/media.md)
media_play = Lecture
media_pause = Pause
media_load = Charger

# — Localization page (docs/localization.md) —
nav_localization = Localisation
fmt_caption = Un seul jeu de traductions — rendu conforme à ICU pour chaque locale : nombres, dates, grammaire du pluriel et ordre de tri suivent la langue.
loc_locale_section = Locale en direct
loc_live_note = La locale est un signal — changer de langue re-rend chaque chaîne instantanément. Le sens de lecture est fixé au lancement (lancez en ar pour l'interface miroir).
loc_current_label = Actuelle
loc_reset = Réinitialiser
loc_numbers_section = Nombres
loc_dates_section = Dates et heures
loc_plurals_section = Pluriels
loc_sorting_section = Tri
fmt_number_label = Groupé
fmt_fraction_label = Deux décimales
fmt_percent_label = Pourcentage
fmt_date_label = Date longue
fmt_time_label = Heure
fmt_datetime_label = Date et heure
fmt_sorted_label = Trié
fmt_number = { NUMBER($n) }
fmt_fraction = { NUMBER($n, minimumFractionDigits: 2) }
fmt_percent = { NUMBER($p, style: "percent") }
fmt_date = { DATETIME($d, dateStyle: "long") }
fmt_time = { DATETIME($t, timeStyle: "short") }
fmt_datetime = { DATETIME($dt, dateStyle: "medium", timeStyle: "short") }
plural_items = { $count ->
    [0] Rien pour l'instant
    [one] Un élément
   *[other] { $count } éléments
}

# Aire de jeu Texte (typographie)
text_caption = Les styles sémantiques correspondent aux styles natifs et à l'échelle de texte d'accessibilité.
text_selectable_toggle = Sélectionnable
text_styles_header = Styles
text_weights_header = Graisses
text_styling_header = Gras et italique
text_colors_header = Couleur
text_custom_header = Tailles personnalisées
text_custom_note = Font.System(pt) — mis à l'échelle par la taille de texte d'accessibilité (Dynamic Type).
text_fonts_header = Polices embarquées
text_fonts_note = Font.Custom("Famille", pt) — fichiers du dossier resource/fonts/ de l'application, embarqués par day build et résolus par nom de famille sur chaque plateforme.

# Aire de jeu Menus
menus_caption = Menus natifs — la barre de menus de l'application et les menus contextuels par élément — avec sous-menus imbriqués, raccourcis clavier et commandes d'édition standard.
menus_last = Dernière action
menus_lifecycle = Cycle de vie
menus_target = Clic droit ici (appui long sur mobile) pour un menu contextuel
menus_shortcut_hint = Les raccourcis clavier (⌘/Ctrl + touche) apparaissent dans la barre de menus et fonctionnent quand l'application est active — p. ex. Nouveau (N), Enregistrer (S), Recharger (R).

# --- day-part-haptics ---
nav_haptics = Haptique
haptics_supported_yes = Moteur haptique disponible sur cette plateforme
haptics_supported_no = Aucun moteur haptique sur cette plateforme (les boutons sont silencieux)
haptics_light = Léger
haptics_medium = Moyen
haptics_heavy = Fort
haptics_success = Succès
haptics_warning = Avertissement
haptics_error = Erreur
haptics_selection = Sélection
haptics_last = Dernier joué
haptics_none = Rien joué pour l'instant
haptics_last_played = Joué : { $style }

# --- day-part-prefs ---
nav_prefs = Préférences
prefs_caption = Conserver une chaîne entre les lancements avec day-part-prefs.
prefs_placeholder = Valeur à mémoriser
prefs_save = Enregistrer
prefs_load = Charger
prefs_clear = Effacer
prefs_idle = Saisissez une valeur, puis Enregistrer.
prefs_empty = (rien d'enregistré)
prefs_saved = Enregistré.
prefs_save_failed = Échec de l'enregistrement.
prefs_loaded = Chargé depuis le stockage.
prefs_missing = Rien d'enregistré pour l'instant.
prefs_cleared = Effacé.
prefs_value_label = Valeur enregistrée :

# --- bundled resources (§18.3) ---
nav_resources = Ressources
resources_caption = Une image chargée par nom depuis une ressource, avec accès aléatoire à des données embarquées.
vectors_title = Vecteurs
vectors_note = Les glyphes de la barre latérale, issus de resource/vectors/ — un SVG par icône, indépendant de la résolution sur chaque backend.
vectors_tints = Teintes
vectors_scene = Illustration en couleurs
vectors_zoom = Zoom
vectors_scene_note = Un SVG de 240 tracés, redessiné à partir du vecteur à chaque palier de zoom plutôt qu'agrandi — les contours restent nets à toute taille. Sur Android, il est livré en VectorDrawable, pas en image matricielle.
vectors_weights = Graisses
vectors_sizes = Tailles
resources_numbers = numbers.bin : { $len } octets, byte[100] = { $byte }
resources_greeting = greeting.txt : { $text }

# --- day-part-deviceinfo ---
nav_deviceinfo = Appareil
deviceinfo_model = Modèle : {$value}
deviceinfo_system = Système : {$name} {$version}
deviceinfo_simulator = Simulateur : {$value}
deviceinfo_yes = oui
deviceinfo_no = non
deviceinfo_refresh = Actualiser

# --- day-piece-activity ---
activity_animating = Animation
activity_on = En rotation
activity_off = Arrêté

# --- day-piece-searchfield ---
nav_search = Recherche
search_clear = Effacer

# --- day-piece-map ---
nav_map = Carte
map_caption = Une MKMapView native — plateformes Apple uniquement. Touchez un préréglage pour recentrer la carte en direct.
map_boston = Boston
map_paris = Paris

# — page tweaks (docs/tweaks.md) —
nav_tweaks = Tweaks
tweaks_intro = Les tweaks empaquetés configurent le composant natif derrière une pièce intégrée, par toolkit. Là où un tweak n'est pas couvert, il est sans effet — les pièces ci-dessous restent d'origine.
tweaks_stock = D'origine
tweaks_tweaked = Ajustée
tweaks_bezel_title = Biseau du bouton
tweaks_bezel_caption = day-tweak-button-bezel — AppKit uniquement : les constantes NSBezelStyle sur le vrai NSButton.
tweaks_selectable_title = Libellé sélectionnable
tweaks_selectable_caption = Le modificateur .selectable() du cœur — la sélection de texte native sur un libellé, activable, là où la boîte à outils la prend en charge.
tweaks_selectable_text = Le texte de ce libellé peut être sélectionné et copié — essayez.
tweaks_tooltip_title = Info-bulle
tweaks_tooltip_caption = day-tweak-tooltip — AppKit, GTK, Android : une info-bulle native, un modificateur sur trois niveaux d'accès.
tweaks_tooltip_label = Enregistrement auto
tweaks_tooltip_hint = Votre travail est enregistré automatiquement. Survolez (ou appui long sur Android) pour les détails.
tweaks_ticks_title = Graduations du curseur
tweaks_ticks_caption = day-tweak-slider-tickmarks — AppKit, GTK, Android, Qt, XAML, ArkUI : graduations natives, avec aimantation là où la plateforme la propose. Le curseur ajusté s'aimante ; celui d'origine glisse.
tweaks_ref_title = Vivacité du NativeRef
tweaks_ref_caption = Un NativeRef atteint le curseur ajusté après montage ; démontez-le et la référence se vide au lieu de pendre.
tweaks_ref_live = réf : vivante
tweaks_ref_cleared = réf : vidée
tweaks_label_title = Ajuster un libellé
tweaks_label_caption = Un ajustement direct (AppKit, UIKit) atténue ce libellé via sa vue native et rapporte la classe rencontrée. Sur iOS, « Sélectionnable » reconstruit le libellé en UITextView en lecture seule — l'ajustement s'applique après .selectable(), donc au widget réellement affiché.
tweaks_label_sample = Ce libellé est atténué par un ajustement natif.
tweaks_label_class = Classe native : { $class }

# — merged section pages (design overhaul) —
nav_canvas = Canevas et formes
nav_system = Appareil et capteurs
nav_services = Services système
controls_basics = Essentiels
canvas_caption = Formes, transformations, gestes et widgets composés — tous dessinés via le canevas.
paths_title = Tracés, contours et découpe
canvas_gauge = Jauge canevas
gauge_value_label = Valeur
system_caption = Les modules d'état de l'appareil : batterie, connectivité, capteurs et identité.
services_caption = Les modules « agir avec l'OS » : HTTP, presse-papiers, préférences, haptique et fichiers.
subscribe_label = S'abonner

# — data strings localized for the walkthrough locales (option lists, specimen rows) —
chocolate = chocolat
size_small = Petit
size_medium = Moyen
size_large = Grand
fruit_apple = Pomme
fruit_banana = Banane
fruit_cherry = Cerise
fruit_date = Datte
fruit_elderberry = Sureau
list_row = Ligne { $n }
text_style_large_title = Grand titre
text_style_title = Titre
text_style_title2 = Titre 2
text_style_title3 = Titre 3
text_style_headline = En-tête
text_style_subheadline = Sous-en-tête
text_style_body = Corps
text_style_callout = Encadré
text_style_footnote = Note de bas de page
text_style_caption = Légende
text_style_caption2 = Légende 2
text_weight_ultralight = Ultra-fin
text_weight_light = Fin
text_weight_regular = Normal
text_weight_medium = Moyen
text_weight_semibold = Demi-gras
text_weight_bold = Gras
text_weight_heavy = Très gras
text_weight_black = Noir
text_bold = Texte gras
text_italic = Texte italique
text_bolditalic = Gras italique
text_emphasis_label = Emphase
color_red = Rouge
color_green = Vert
color_blue = Bleu
color_orange = Orange

# Menus & dialogues (page fusionnée)
menus_appmenu_section = Menu de l’application
menus_context_section = Menu contextuel
menus_dialogs_section = Dialogues
modal_result_label = Résultat

# Page Média
media_caption = Un lecteur multimédia natif — la vue de la plateforme, transport piloté par déclencheurs.
media_player_section = Vidéo

# Sections de la page Ressources
resources_image_section = Image embarquée
resources_modes_note = Une image, trois modes — Ajuster préserve les proportions, Remplir rogne, Étirer déforme.
image_mode_fit = Ajuster
image_mode_fill = Remplir
image_mode_stretch = Étirer
resources_data_section = Données embarquées

# Page À propos
about_app_section = Cette app
about_version = Version
about_toolkit = Boîte à outils
about_id = Identifiant
about_os = Système
about_model = Modèle
about_locale = Langue
history_hint = Touchez + ou − ci-dessus : chaque changement s’affiche ici.

# Page Focus (docs/focus.md)
nav_focus = Focus
focus_caption = Le focus est une liaison bidirectionnelle : les changements natifs écrivent le signal, et écrire le signal déplace le focus.
focus_group_section = Un signal, un formulaire
focus_group_caption = Trois champs liés à un même signal optionnel. Cliquez ou tabulez de l’un à l’autre et l’indicateur suit ; Entrée passe au champ suivant.
focus_name_label = Nom
focus_email_label = E-mail
focus_city_label = Ville
focus_current_label = Focus
focus_next = Focus suivant
focus_clear = Effacer le focus
focus_bool_section = Un contrôle, un booléen
focus_bool_caption = Le même champ lié à un signal booléen — les boutons l’écrivent ; entrer dans le champ ou en sortir l’écrit en retour.
focus_bool_placeholder = Le focus arrive ici
focus_focus_btn = Donner le focus
focus_blur_btn = Retirer le focus
focus_state_label = État
focus_state_on = avec focus
focus_state_off = sans focus
focus_probe_section = Au-delà des champs de texte
focus_probe_caption = Les toolkits de bureau donnent aussi le focus aux boutons, interrupteurs et curseurs ; les plateformes tactiles le réservent surtout à la saisie de texte.
focus_probe_toggle = Interrupteur
focus_probe_slider = Curseur
focus_probe_button = Bouton

# HTTP fetch demo (docs/http.md) — the status readout stays raw "<status> <body>" so the
# walkthrough asserts it byte-for-byte in every locale.
http_title = HTTP
http_caption = Le module day-part-http passe par la pile HTTP de la plateforme — ses proxys, son VPN et son TLS.
http_fetch = Récupérer depuis localhost
http_idle = Rien de récupéré pour l'instant
http_tier = Pile
http_url_placeholder = https://example.com
http_check = Vérifier
http_checking = Vérification…
http_patch = PATCH
http_res_label = Ressource
http_res_refetch = Recharger

# Scrolling page (docs/scroll.md) — programmatic scroll targets.
scroll_to_top = Aller en haut
scroll_to_bottom = Aller en bas
scroll_to_item = Aller à l'élément 100

# Grid page (docs/grid.md) — grid/grid_row from basics to a stress test.
nav_grid = Grille
grid_caption = Des colonnes dimensionnées par leur contenu, des cellules qui fusionnent, et des cellules flexibles qui se partagent la largeur restante
grid_tab_basics = Bases
grid_tab_sizing = Dimensions
grid_tab_spanning = Fusion
grid_tab_composite = Composition
grid_tab_stress = Endurance
grid_basics_caption = Chaque colonne prend la largeur de sa cellule la plus large. Pas de largeurs fixes, pas d'espaceurs de remplissage.
grid_col_name = Nom
grid_col_wins = Victoires
grid_col_points = Points
grid_sizing_caption = Colonnes fixes, ajustées au contenu et flexibles dans une même grille.
grid_sizing_fixed = Fixe 80 pt
grid_sizing_content = Contenu
grid_sizing_short = Court
grid_sizing_longer = Une cellule au contenu plus long
grid_spanning_caption = Une cellule peut couvrir plusieurs colonnes ; un enfant hors de toute ligne couvre la grille entière.
grid_month_title = Semainier
grid_event_focus = Bloc de concentration
grid_event_review = Revue
grid_composite_caption = Formes et grille réunies : des glyphes groupés dans des colonnes au contenu, à côté d'une barre de plage flexible.
grid_day_n = Jour { $n }
grid_stress_cells = { $n } lignes de 8 cellules, toutes disposées d'avance. Modifier une cellule ne remesure que celle-ci.
grid_stress_add = Ajouter 50 lignes
grid_stress_bump = Incrémenter la première cellule

nav_animation = Animation
anim_caption = Mettez en file d’attente échelle, rotation, opacité, décalage et teinte, puis touchez « Animer ! » pour tout animer ensemble avec la courbe et la durée choisies.

# Page Animation
anim_scale = Échelle
anim_rotation = Rotation
anim_opacity = Opacité
anim_offset_x = Décalage X
anim_offset_y = Décalage Y
anim_hue = Teinte
anim_curve = Courbe
anim_duration = Durée
anim_randomize = Aléatoire !
anim_go_label = Animer !
anim_reset_label = Réinitialiser
anim_curve_spring = Ressort
anim_curve_ease_in_out = Progressif
anim_curve_ease_out = Décéléré
anim_curve_linear = Linéaire
anim_duration_ms = { $ms } ms

# — Page Benchmark (le benchmark Grilles de Day-Bench ; sur les backends Apple natifs, un
#   sélecteur segmenté héberge aussi sa réplique SwiftUI via day-piece-swiftui, docs/swiftui.md) —
nav_benchmark = Benchmark
bench_caption = Un patchwork pseudo-aléatoire de cellules de grille qui pave exactement le panneau. Les deux paramètres réempaquettent chaque rangée, le moteur de mise en page renégocie donc tout d'un coup.
bench_parameters = Paramètres
bench_seed = Graine aléatoire
bench_count = Nombre total
bench_rows = { $rows } { $rows ->
    [one] rangée
   *[other] rangées
}
# Gabarits %d pour le panneau SwiftUI hébergé (son compteur vit dans l'état Swift).
bench_rows_one = %d rangée
bench_rows_other = %d rangées
bench_tab_day = Day natif
bench_tab_swiftui = Grille SwiftUI

# Barre de menus + menu contextuel
menu_file = Fichier
menu_open = Ouvrir…
menu_open_recent = Ouvrir récent
menu_clear_menu = Effacer le menu
menu_save = Enregistrer
menu_save_as = Enregistrer sous…
menu_edit = Édition
menu_view = Affichage
menu_reload = Recharger
menu_actual_size = Taille réelle
menu_context = Contexte
menu_rename = Renommer
menu_duplicate = Dupliquer
menu_move_to = Déplacer vers
menu_inbox = Boîte de réception
menu_archive = Archiver

# Page Texte : tailles, polices embarquées, liens
text_size_pt = { $pt } pt
text_font_pacifico = Pacifico — script fluide
text_font_bungee = BUNGEE — display chromatique
text_font_specialelite = Special Elite — touches de machine à écrire
text_font_pacifico_lg = Pacifico en 36 points
text_links_section = Liens
text_runs_section = Fragments stylés
text_markdown_section = Markdown, analysé en direct
text_markdown_caption = Saisissez ci-dessous. Le libellé sous le séparateur est réanalysé à chaque frappe, par le même chemin qu'une chaîne traduite ou une valeur venue du réseau.
text_markdown_sample = Day analyse **gras**, *italique*, `code`, ~~barré~~ et [liens](https://daybrite.dev) à l'exécution. Un balisage inachevé comme ** reste littéral.
text_markdown_opened = Ouvert : {$url}
text_baseline_section = Alignement sur la ligne de base
text_baseline_caption = Les lignes posent leur texte sur une même ligne de base au lieu de centrer des boîtes de hauteurs différentes. Désactivez l'option pour voir la différence : le texte du champ, l'étiquette à côté et l'unité qui suit se décalent.
text_baseline_toggle = Aligner les lignes de base
text_baseline_quantity = Quantité
text_baseline_unit = articles
text_baseline_total = Total
text_baseline_currency = USD
text_baseline_due = Échéance
text_baseline_unsupported = Cette boîte à outils ne fournit aucune ligne de base ; ces lignes restent centrées.
text_links_caption = Touchez un lien pour l'ouvrir dans le navigateur du système.
text_link_icons_label = Material Symbols sur Google Fonts
text_link_mail_label = Écrire à l'équipe

# Section Fichiers : texte initial de l'éditeur
files_initial_content =
    Bonjour de Day !
    Modifiez-moi, puis Enregistrer.

# Page Rapports de plantage (day-break, docs/break.md)
nav_crash = Rapports de plantage
crash_caption = Installez les gestionnaires de plantage, consultez le rapport au prochain lancement et choisissez de l'envoyer.
crash_trigger_section = Déclencher un plantage
crash_report_section = Dernier rapport de plantage
crash_abort = Planter (abandon)
crash_abort_label = Abandon natif → SIGABRT
crash_segv = Planter (erreur de segmentation)
crash_segv_label = Déréférencement nul → SIGSEGV
crash_contained = Panique (contenue)
crash_contained_label = Interceptée par day, l'application survit
crash_send = Envoyer le rapport
crash_clear = Effacer les rapports
crash_empty = Aucun rapport pour l'instant. Déclenchez un plantage, puis relancez pour le voir ici.

# Page Zones de texte
nav_textareas = Zones de texte
textareas_caption = Un éditeur multiligne natif avec des attributs modifiable, sélectionnable et correction orthographique en direct (selon la boîte à outils).
textareas_editor_section = Éditeur
textareas_seed_section = Remplir avec
textareas_attrs_section = Attributs
textareas_seed_short = Court
textareas_seed_long = Long
textareas_seed_markdown = Markdown
textareas_editable = Modifiable
textareas_selectable = Sélectionnable
textareas_spellcheck = Correction orthographique
textareas_sample_short = Une courte note. Modifiez-la, ou insérez un texte plus long ou structuré avec les boutons ci-dessous.
sensor_permission = Accès aux mouvements
perm_request = Demander
perm_open_settings = Ouvrir les réglages
nav_location = Localisation
location_permission = Accès à la position
location_start = Démarrer
location_stop = Arrêter
location_waiting = en attente d’une position…
location_unavailable = aucun service de localisation sur cette plateforme
location_coords = { $lat }, { $lon }
location_altitude = Altitude
location_accuracy = Précision
location_unknown = —
chart_axes = x rouge · y turquoise · z violet

# Écran couvrant plein écran (docs/cover.md).
stack_cover_button = Présenter un écran couvrant
cover_title = Écran couvrant
cover_body = Présenté au-dessus de toute la fenêtre par cover(signal). Natif sur iOS et Android, enfant au premier plan ailleurs.
cover_dismiss = Fermer

stack_unsaved = Modifications non enregistrées
stack_discard_title = Abandonner les modifications ?
stack_discard_body = Cette page a des modifications non enregistrées.
stack_discard_ok = Abandonner

tab_dynamic_title = Onglets pilotés par les données
tab_dynamic_add = Ajouter un onglet
tab_dynamic_remove = Retirer un onglet

# Stockage de fichiers local (page services, docs/fs.md).
storage_title = Fichiers locaux
storage_caption = Le module day-part-fs stocke des fichiers privés de l'app sur chaque cible — de vrais fichiers en natif, OPFS dans le navigateur.
storage_placeholder = Texte à stocker
storage_save = Enregistrer le fichier
storage_load = Charger
storage_delete = Supprimer
storage_files_label = Fichiers stockés
storage_idle = Rien d'enregistré pour l'instant

prefs_window_title = Préférences
prefs_window_caption = Le thème et la langue s'appliquent à toutes les fenêtres et sont conservés entre les lancements.

# — Toolbars page (docs/toolbars.md) —
nav_toolbars = Barres d'outils
toolbars_caption = La barre d'outils de la fenêtre elle-même, dans l'habillage natif de la plateforme — un NSToolbar sur macOS, l'AdwHeaderBar sur GNOME, une QToolBar sur KDE, une CommandBar sur Windows. Regardez en haut de cette fenêtre ; les contrôles ci-dessous la pilotent.
toolbar_unsupported = Cette boîte à outils n'a pas de barre d'outils de fenêtre ; rien n'a donc été installé. Sur téléphone, ces commandes vont dans le contenu.
toolbar_readout_title = Ce que fait la barre d'outils
toolbar_controls_title = La piloter depuis ici
toolbar_vocabulary_title = Le vocabulaire des éléments
# Item labels — these appear IN the toolbar, so they stay short.
toolbar_sidebar = Barre latérale
toolbar_new = Nouvelle fenêtre
toolbar_star = Étoile
toolbar_menu = Plus
toolbar_menu_open_scripting = Ouvrir la page Scripts
toolbar_menu_copy_script = Copier le script
toolbar_extra_tooltip = Copier la boîte à outils et la version, pour un rapport de bogue
toolbar_extra = Copier les infos
toolbar_search_placeholder = Rechercher
show_source = Afficher la source
# The live readout.
toolbar_query_label = Texte de recherche
toolbar_query_empty = (vide)
toolbar_star_label = Étoile
toolbar_on = Activée
toolbar_off = Désactivée
toolbar_presses_label = Appuis
toolbar_presses = { $count ->
    [one] { $count } appui
   *[other] { $count } appuis
}
toolbar_last_label = Dernière action
toolbar_last_none = Rien pour l'instant
toolbar_appearance_label = Apparence
toolbar_appearance_ignored = { $mode } (ignoré ici)
toolbar_transport_label = Enregistreur
toolbar_transport_idle = Inactif
toolbar_transport_recording = Enregistrement
toolbar_transport_playing = Lecture
toolbar_transport_paused = En pause
toolbar_last_new = Nouvelle fenêtre
toolbar_last_star = Étoile basculée
toolbar_last_extra = Infos de version copiées
# The page's own controls.
toolbar_extra_label = Afficher l'élément Copier les infos
toolbar_enabled_label = Élément Effacer activé
toolbar_clear_search = Effacer la recherche
toolbar_seed_search = Remplir la recherche
toolbar_seed_text = barres d'outils
# One line per item kind.
toolbar_kind_button = Bouton
toolbar_kind_button_note = Exécute une commande, et peut partager son action avec un élément de menu.
toolbar_kind_toggle = Interrupteur
toolbar_kind_toggle_note = Lié en double sens à un signal, donc l'interrupteur ci-dessus et le bouton s'accordent.
toolbar_kind_menu = Menu
toolbar_kind_menu_note = Un menu déroulant construit à partir des mêmes entrées que la barre de menus.
toolbar_kind_search = Recherche
toolbar_kind_search_note = Le champ de recherche de la plateforme, lié en double sens à un signal.
toolbar_kind_space = Espaceurs
toolbar_kind_space_note = Un espace flexible sépare les éléments de tête de ceux de fin.
# --- day-part-local-notify (docs/notify.md) ---
nav_notify = Notifications
notify_caption = Publiez une notification locale via le système de notifications de la plateforme. Les notifications programmées sont conservées par le système lorsque c'est possible — voir la ligne de capacités.
notify_caps_post = Publication prise en charge sur cette plateforme
notify_caps_unsupported = Notifications locales indisponibles dans cette version (sur macOS elles exigent une app signée — utilisez day pack)
notify_caps_schedule_os = Les notifications programmées survivent à la fermeture de l'application
notify_caps_schedule_process = Les notifications programmées reposent sur une minuterie interne et sont perdues si l'application se ferme
notify_title_label = Titre
notify_title_placeholder = Titre de la notification
notify_title_default = Bonjour de Day
notify_body_label = Corps
notify_body_placeholder = Corps de la notification
notify_body_default = Publiée par la vitrine Day.
notify_delay = Délai
notify_delay_now = Maintenant
notify_delay_5s = 5 secondes
notify_delay_15s = 15 secondes
notify_delay_60s = 1 minute
notify_importance = Importance
notify_importance_low = Faible
notify_importance_default = Normale
notify_importance_high = Élevée
notify_importance_urgent = Urgente
notify_sound = Émettre un son
notify_badge = Nombre sur la pastille
notify_route = Le tap ouvre
notify_post = Publier
notify_cancel = Tout annuler
notify_status_idle = Rien n'a encore été publié
notify_status_posted = Publiée
notify_status_scheduled = Programmée
notify_status_cancelled = Toutes les notifications ont été annulées
notify_status_failed = Publication impossible
notify_last = Dernier résultat
notify_perm_granted = Autorisation de notification accordée
notify_perm_missing = Autorisation de notification refusée — les envois seront ignorés
notify_perm_request = Demander l'autorisation
haptics_songs_caption = Séquences plus longues — les silences portent le rythme autant que les impulsions.
haptics_song_celebration = Célébration
haptics_song_levelup = Niveau supérieur
haptics_song_heartbeat = Battement
haptics_song_cascade = Cascade

# — scripting (enregistreur dayscript) —
nav_scripting = Scripts
scripting_caption = Enregistrez vos appuis et votre navigation dans un dayscript rejouable. Appuyez sur Enregistrer, parcourez l'application et agissez, puis revenez et appuyez sur Arrêter. Modifiez le YAML pour l'ajuster, Lire pour le rejouer, Copier ou Exporter pour le conserver.
scripting_record = Enregistrer
scripting_stop = Arrêter
scripting_copy = Copier
scripting_export = Exporter
scripting_copied = Copié dans le presse-papiers
scripting_delay_label = Délai entre les étapes
scripting_delay_unit = s
scripting_saved_label = Scripts enregistrés
scripting_pick = Charger un script enregistré…
scripting_save = Enregistrer
scripting_name_hint = Mon script
scripting_saved = Enregistré

# --- app badge (docs/badge.md) ---
nav_badge = Pastille d'application
badge_caps_native = Cette plateforme affiche une pastille sur l'icône
badge_caps_emulated = La pastille est envoyée, mais son affichage dépend du shell ou de l'installation de l'application
badge_caps_none = Cette plateforme n'a pas d'API de pastille
badge_android_note = Android n'a aucune API pour définir une pastille : les lanceurs déduisent le point des notifications publiées ; utilisez le compteur d'une notification.
badge_count_label = Nombre
badge_minus = −
badge_plus = +
badge_set = Définir
badge_clear = Effacer
badge_set_text = Texte « beta »
badge_last = Dernière action
badge_status_idle = Rien de défini
badge_status_set = Pastille définie sur { $count }
badge_status_cleared = Pastille effacée
badge_status_text = Pastille définie sur « beta »

# The Controls page mixer: one shared state, many editors.
mix_custom = Personnalisé
mix_untitled = Mixage sans titre
mix_summary = {$name} · {$preset} à {$level} %
voice_search_placeholder = Filtrer les parfums…

# Context-menu demos (menus page): the message-list rows and the media card.
menu_reply = Répondre
menu_forward = Transférer
menu_archive = Archiver
menu_share = Partager…
menu_copy_image = Copier l’image
menu_save_image = Enregistrer dans Photos
menu_get_info = Lire les informations
menus_messages_section = Une liste de messages
menus_messages_hint = Chaque ligne porte son propre menu — l’action nomme le message d’origine.
menus_photo_section = Une carte média
msg_subject_one = Chiffres trimestriels, première passe
msg_subject_two = Plan du week-end, réponse définitive
msg_subject_three = Croquis de l’atelier

# The Star command (commands.rs): one label per state, shared by the toolbar, the
# application menu and each navigation row's context menu.
# Save a picture of this window (commands.rs, docs/window-image.md).
cmd_screenshot = Capture d'écran…
cmd_appearance_light = Clair
cmd_appearance_system = Système
cmd_appearance_dark = Sombre
cmd_record = Enregistrer
cmd_stop_recording = Arrêter
cmd_play = Lire
cmd_pause = Pause
cmd_resume = Reprendre
cmd_clear_recording = Effacer l'enregistrement
menu_appearance = Apparence
menu_record = Enregistrement
cmd_star = Suivre
cmd_unstar = Ne plus suivre

# Speech (day-part-speech) : la référence daybridge — une API, un langage par plateforme.
speech_title = Synthèse vocale
speech_caption = Une seule API Rust ; la voix native en dessous — Swift sur Apple, Java sur Android, ArkTS sur HarmonyOS, JavaScript sur le web, C++ sur Windows.
speech_phrase = Une journée claire, avec un risque de pluie plus tard.
speech_speak = Parler
speech_stop = Arrêter
speech_support_label = Prise en charge ici
speech_native = Native
speech_emulated = Émulée (partielle)
speech_unsupported = Non prise en charge sur cette cible

# Support banners (widgets.rs): shown over a demo the target cannot run.
support_missing_here = Non pris en charge sur cette plateforme — la démo reste visible, mais elle ne fonctionnera pas ici.
support_emulated_here = Partiellement pris en charge ici — la démo fonctionne, mais sans l'implémentation native de la plateforme.
vectors_live_tint = Teinte dynamique
vectors_cycle_tint = Changer

# Layout page (docs/size-classes.md "Row fit policies")
layout_caption = Comment une même rangée de boutons se comporte selon chaque politique d'ajustement quand la fenêtre est trop étroite.
layout_note = Choisissez une politique, puis ajoutez des composants jusqu'à ce que la rangée manque de place. Rognage laisse la fin sortir de l'écran (les builds de débogage le signalent), Retour à la ligne passe à la ligne à la largeur de chaque bouton, Colonnes égales aligne ces lignes en grille, Colonne empile en largeur compacte, Défilement garde une seule ligne à faire glisser. Sur un ordinateur, rétrécissez la fenêtre pour voir Colonne s'activer.
layout_row_section = Ajustement de rangée
layout_fit_label = Politique
layout_fit_clip = Rognage
layout_fit_wrap = Retour à la ligne
layout_fit_wrap_columns = Colonnes égales
layout_fit_column = Colonne
layout_fit_scroll = Défilement
layout_count_label = Composants
layout_item = Élément { $n }
layout_item_wide = Élément { $n } (plus large)

vectors_alias_note = Un SVG simple auquel on demande la graisse Grasse : il n'a pas d'axe de graisse, donc l'alias revient au glyphe de base au lieu de ne rien dessiner.
vectors_pick_tint = Choisir une teinte
vectors_tint_idioms = Deux sélecteurs de couleur, une seule couleur liée : le premier ouvre le sélecteur de la plateforme, le second ouvre le panneau que Day dessine lui-même — le même sélecteur sur chaque cible.
