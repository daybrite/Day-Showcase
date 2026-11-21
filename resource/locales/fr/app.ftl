app_title = Vitrine de Day
counter_value = { $count ->
    [one] { $count } clic
   *[other] { $count } clics
}
decrement = −
increment = +
name_placeholder = Votre nom
flavor_placeholder = Saisissez ou choisissez un parfum
flavor_add = Ajouter
nav_controls = Contrôles
ctl_buttons = Boutons
ctl_press = Appuyer
ctl_plain = Simple
ctl_bordered = Bordé
ctl_prominent = Mis en avant
ctl_tinted = Teinté
ctl_destructive = Destructif
ctl_disabled = Désactivé
ctl_link = Lien
ctl_composed = Composé
ctl_link_text = daybrite.dev
ctl_presses = Appuis
ctl_switches = Interrupteurs et curseurs
ctl_toggle = Interrupteur
ctl_slider = Curseur
ctl_stepped = Curseur à crans
ctl_stepper = Incrémenteur
ctl_composed_stepper = Incrémenteur composé
ctl_progress = Progression
ctl_activity = Activité
ctl_spinner = Indicateur
ctl_accessibility = Accessibilité
ctl_a11y_heading = Un titre, pour les lecteurs d’écran
ctl_announce = Annoncer
ctl_announce_hint = Lit le compteur via le lecteur d’écran
ctl_announced_times = Annoncé { $count } fois
ctl_announce_emulated = Émulé (une zone en direct)
ctl_text_entry = Saisie de texte
ctl_text_field = Champ de texte
ctl_search_field = Champ de recherche
ctl_combo_box = Liste déroulante
ctl_combo_note = Cette boîte à outils n'a pas de liste déroulante éditable, Day affiche donc un substitut ici.
ctl_text_area = Zone de texte
ctl_notes_placeholder = Quelques lignes de notes
ctl_pickers = Sélecteurs
ctl_date = Date
ctl_time = Heure
ctl_color = Couleur
ctl_indicators = Indicateurs et composition
ctl_badge = Badge
ctl_gauge = Jauge
ctl_divider = Séparateur
ctl_state = État
ctl_on = Activé
ctl_off = Désactivé
nav_menus = Menus et dialogues
nav_text = Texte
nav_battery = Batterie
nav_sensors = Capteurs
nav_clipboard = Presse-papiers
nav_media = Média
nav_compose = Cartes et environnement
nav_files = Fichiers
nav_tabs = Onglets
nav_stack = Pile
nav_layout = Disposition
layout_stacks_section = Rangées, colonnes et couches
layout_align_label = Alignement
layout_align_top = Haut
layout_align_center = Centre
layout_align_bottom = Bas
layout_layered = Superposé
layout_scroll_section = Défilement
layout_scroll_top = Défiler vers le haut
layout_scroll_end = Défiler vers la fin
layout_size_section = Classes de taille
layout_size_width = Largeur
layout_size_height = Hauteur
nav_list = Liste
refresh_tier_native = Tirer pour actualiser : natif
refresh_tier_emulated = Tirer pour actualiser : émulé
layout_row = Élément { $n }
nav_webview = Vue Web
nav_lottie = Lottie
nav_about = À propos

# Sidebar group headers (lib.rs `Group`): the sections the destinations sit under.
group_starred = Favoris
group_overview = Aperçu
group_controls = Contrôles
group_layout = Disposition
group_navigation = Navigation et interface
group_data = Données
group_graphics = Graphismes et médias
group_platform = Plateforme
group_app = Application et outils

picker_selected = Sélection
picker_segmented = Segmenté
picker_menu = Menu
picker_inline = Aligné

# --- day::datetime ---
nav_dates = Date et heure
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

compose_card_title = Surface réutilisable
compose_card_body = Une marge intérieure, un fond et des coins arrondis.
compose_env_value = Teinté par l'accent fourni
list_add = Ajouter 100
list_caption = { $count } lignes
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
list_options = Options…
list_options_title = Options de la liste
list_options_layout = Disposition
list_options_behavior = Comportement
list_options_gestures = Gestes
list_opt_sections = En-têtes de section
list_opt_row_height = Hauteur des lignes
list_opt_uniform = Uniforme
list_opt_automatic = Automatique
list_opt_separators = Séparateurs
list_opt_default = Par défaut
list_opt_on = Activés
list_opt_off = Désactivés
list_opt_content = Contenu des lignes
list_opt_plain = Libellé
list_opt_item = Élément de liste
list_opt_multi = Sélection multiple
list_opt_pull = Tirer pour actualiser
list_opt_editing = Mode édition
list_opt_reorder = Glisser pour réordonner
list_opt_swipe = Actions de balayage
list_opt_delete = Balayer pour supprimer
list_opt_gestures_footer = Avec les en-têtes de section, les lignes se déplacent dans leur section et le balayage pour supprimer est désactivé.
list_section = Lignes { $first } à { $last }
list_row_note = Une deuxième ligne de détail pour cette ligne
list_row_note_long = Les lignes dimensionnées par leur contenu s’agrandissent pour un texte plus long, comme cette troisième ligne qui passe à la ligne dans une fenêtre étroite.
list_swipe_select = Sélectionner
list_swipe_deselect = Désélectionner
list_align = Position d’arrivée
list_align_nearest = Au plus près
list_align_start = En haut
list_align_center = Au milieu
list_align_end = En bas
list_insert = Insérer
list_remove = Retirer
list_remove_all = Tout retirer
list_footer = Fin de la liste : { $count } lignes
list_empty = Aucune ligne. Réinitialiser les rétablit.
list_visible = Visibles : { $first }–{ $last }
list_visible_first = Visibles à partir de { $first }
list_visible_none = Aucune ligne visible

nav_model = Modèle
model_caption = { $count } lignes affichées
model_cost = { $paths } chemins observés · { $nodes } nœuds internés
model_hide_done = Masquer les terminées
model_selected = Sélection : { $title }
model_selected_none = Aucune sélection
model_add = Ajouter une tâche
model_delete = Supprimer la sélection
model_undo = Annuler
model_redo = Rétablir
model_storage_file = Stocké dans { $file }
model_storage_memory = Stocké en mémoire sur cette plateforme
nav_query = Requête
query_caption = { $count } morceaux sur { $total } correspondent
query_resident = { $resident } lignes sur { $total } dans le cache du processus de base de données
query_starred = Favoris uniquement
query_selected = Sélection : { $title }
query_selected_none = Aucune sélection
query_fts = Texte intégral (FTS5)
query_viewport = Fenêtre uniquement
query_viewport_box = lat { $from }–{ $to }
query_star = Basculer le favori

webview_url_hint = Saisir une URL
webview_go = Aller
webview_back = Précédent
webview_forward = Suivant
webview_stop = Arrêter
webview_reload = Recharger
webview_js_hint = JavaScript à exécuter dans la page
webview_js_run = Exécuter »
webview_js_result_hint = Le résultat JSON s’affiche ici
webview_note_iframe = Cette plateforme intègre la page dans une iframe : Précédent, Suivant et Arrêter sont indisponibles, et le champ ne suit pas les liens.
webview_tab_remote = Distant
webview_tab_embedded = Intégré
webview_embedded_unsupported = Aucun moteur web n'est disponible dans cette version du toolkit.
webview_embedded_status_none = Aucun lien externe suivi pour l'instant.
webview_embedded_opened = Ouvert hors de l'app : { $url }
webview_embedded_intercepted = Intercepté — navigation dans l'app vers « { $route } ».

stack_push = Empiler un détail
stack_detail_title = Niveau { $depth }
stack_item_title = Élément { $id }
stack_link_42 = Ouvrir l'élément 42 par sa route
stack_param_hint = Ouvert avec l'indice : {$hint}
tab_one = Aperçu
tab_two = Détails
tab_three = Réglages
tab_state_note = Changez d'onglet puis revenez : le compteur est conservé.
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
battery_reading_absent = Batterie : cet appareil n’a pas de batterie

# Aire de jeu Capteurs (docs/sensors.md)
sensor_accelerometer = Accéléromètre
sensor_gyroscope = Gyroscope
sensor_magnetometer = Magnétomètre
sensor_reading = x { $x } · y { $y } · z { $z } { $unit }
sensor_waiting = en attente du premier échantillon…
sensor_unavailable = indisponible sur cet appareil

# Aire de jeu Presse-papiers (docs/clipboard.md)
clipboard_placeholder = Saisissez un texte à copier
clipboard_copy = Copier
clipboard_paste = Coller
clipboard_idle = Presse-papiers intact
clipboard_copied = Copié dans le presse-papiers système
clipboard_copy_failed = Échec de la copie (pas d'API presse-papiers ici)
clipboard_pasted = Collé depuis le presse-papiers système
clipboard_empty = Presse-papiers vide ou illisible

# Aire de jeu Réseau (docs/network.md)
network_refresh = Lire le réseau
network_reading_online = En ligne · { $kind } · facturé : { $expensive }
network_reading_offline = Hors ligne
network_reading_none = Connectivité indisponible

# Aire de jeu Média (docs/media.md)
media_play = Lecture
media_pause = Pause
media_load = Charger

# --- Localization page (docs/localization.md) ---
nav_localization = Localisation
loc_locale_section = Locale en direct
loc_live_note = Le sens de la mise en page est fixé au lancement.
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
text_selectable_toggle = Sélectionnable
text_styles_header = Styles
text_weights_header = Graisses
text_styling_header = Gras, italique et couleur
text_custom_header = Tailles personnalisées
text_fonts_header = Polices embarquées

# Aire de jeu Menus
menus_last = Dernière action
menus_lifecycle = Cycle de vie
menus_target = Clic droit ou appui long ici

# --- day-part-haptics ---
nav_haptics = Haptique
haptics_supported_yes = Moteur haptique disponible
haptics_supported_no = Aucun moteur haptique sur cette plateforme
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

# --- day-part-sound ---
nav_sound = Effets sonores
sound_click = Clic
sound_confirm = Confirmation
sound_error = Erreur
sound_glass = Verre
sound_card = Carte
sound_jingle = Jingle
sound_pan_caption = Panoramique stéréo
sound_left = Gauche
sound_center = Centre
sound_right = Droite
sound_volume = Volume
sound_last = Dernier joué
sound_none = Rien joué pour l'instant
sound_last_played = Joué : { $clip }
sound_loop = Bourdonnement en boucle
sound_loop_volume = Volume de la boucle
sound_looping = En boucle
sound_looping_none = Rien en boucle
sound_looping_clip = En boucle : { $clip }

# --- day-part-preferences ---
nav_prefs = Préférences
prefs_placeholder = Une note à mémoriser
prefs_empty = (rien d'enregistré)
prefs_value_label = Valeur enregistrée

# --- bundled resources (§18.3) ---
nav_resources = Ressources
vectors_title = Vecteurs
vectors_tints = Teintes
vectors_zoom = Zoom
vectors_weights = Graisses
vectors_sizes = Tailles
resources_numbers = numbers.bin : { $len } octets, byte[100] = { $byte }
resources_greeting = greeting.txt : { $text }

# --- day-part-device ---
nav_deviceinfo = Appareil
deviceinfo_model = Modèle : {$value}
deviceinfo_system = Système : {$name} {$version}
deviceinfo_simulator = Simulateur : {$value}
deviceinfo_yes = oui
deviceinfo_no = non
deviceinfo_refresh = Actualiser

# --- Day core spinner ---
activity_on = En rotation
activity_off = Arrêté

# --- day::searchfield ---
search_clear = Effacer


# --- page tweaks (docs/tweaks.md) ---
nav_tweaks = Tweaks
tweaks_stock = D'origine
tweaks_tweaked = Ajustée
tweaks_bezel_title = Biseau du bouton
tweaks_bezel_caption = AppKit uniquement
tweaks_selectable_title = Libellé sélectionnable
tweaks_selectable_text = Sélectionnez ce texte et copiez-le.
tweaks_tooltip_title = Info-bulle
tweaks_tooltip_caption = AppKit, GTK et Android. Survolez le bouton ou maintenez-le appuyé.
tweaks_tooltip_label = Enregistrement auto
tweaks_tooltip_hint = Votre travail est enregistré automatiquement.
tweaks_ticks_title = Graduations du curseur
tweaks_ticks_caption = AppKit, GTK, Android, Qt, XAML et ArkUI. Le curseur ajusté s'aimante sur ses graduations.
tweaks_ref_title = Vivacité du NativeRef
tweaks_ref_caption = Monter le curseur
tweaks_ref_live = réf : vivante
tweaks_ref_cleared = réf : vidée
tweaks_label_title = Ajuster un libellé
tweaks_label_caption = AppKit et UIKit. Sur iOS, Sélectionnable change la classe native.
tweaks_label_sample = Ce libellé est atténué par un ajustement natif.
tweaks_label_class = Classe native : { $class }

# --- merged section pages (design overhaul) ---
nav_canvas = Canevas
nav_charts = Graphiques
nav_system = Appareil et capteurs
nav_network_http = Réseau et HTTP
network_status_section = Connectivité
nav_notify_badge = Notifications et badge
nav_push = Notifications push
nav_speech_haptics = Voix, sons et haptique
nav_files_storage = Fichiers et stockage
gauge_value_label = Valeur
canvas_text_title = Texte et polices
canvas_anchors_title = Ancrages du texte
canvas_interaction_title = Toucher, glisser et survoler
canvas_interaction_label = Signalé
canvas_interaction_none = —
canvas_interaction_readout = survol { $hover } · touche { $tap } · glissement { $trail } points
canvas_metrics_cache = measure_text : { $hits } en cache · { $misses } mesurés · { $entries } retenus
canvas_font_label = Police
canvas_font_default = Système
canvas_text_specimen = Portez ce vieux whisky au juge blond qui fume
canvas_text_bold = Gras
canvas_text_italic = Italique
canvas_text_bold_italic = Gras italique
canvas_image_title = Image à partir d'octets
canvas_image_loading = Décodage…
canvas_image_failed = Ces octets ne sont pas une image que cette plateforme décode
canvas_image_readout = { $format } · { $width } × { $height } px

# --- data strings localized for the walkthrough locales (option lists, specimen rows) ---
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
media_player_section = Vidéo

# Sections de la page Ressources
resources_image_section = Image embarquée
image_mode_fit = Ajuster
image_mode_fill = Remplir
image_mode_stretch = Étirer
resources_data_section = Données embarquées

# Réglages (docs/settings.md) : les groupes Catalogue et Diagnostic à côté du groupe Général de Day.
# Les libellés de la fenêtre À propos (Version, Site web) et du groupe Général viennent du catalogue de Day.
settings_catalog = Catalogue
settings_catalog_subtitle = Pages favorites
settings_starred_count = Pages favorites
settings_clear_starred = Effacer les favoris
settings_clear_starred_confirm = Retirer l’étoile de toutes les pages ?
settings_diagnostics = Diagnostic
settings_diagnostics_subtitle = Boîte à outils, système, langue
about_toolkit = Boîte à outils
about_id = Identifiant
about_os = Système
about_model = Modèle
about_locale = Langue

# Page Focus (docs/focus.md)
nav_focus = Focus
focus_group_section = Un signal, un formulaire
focus_name_label = Nom
focus_email_label = E-mail
focus_city_label = Ville
focus_current_label = Focus
focus_next = Focus suivant
focus_clear = Effacer le focus
focus_bool_section = Un contrôle, un booléen
focus_bool_placeholder = Le focus arrive ici
focus_focus_btn = Donner le focus
focus_blur_btn = Retirer le focus
focus_state_label = État
focus_state_on = avec focus
focus_state_off = sans focus
focus_probe_section = Au-delà des champs de texte
focus_probe_caption = Les plateformes tactiles réservent le focus à la saisie de texte.
focus_probe_toggle = Interrupteur
focus_probe_slider = Curseur
focus_probe_button = Bouton

# HTTP fetch demo (docs/http.md): the status readout stays raw "<status> <body>" so the
# walkthrough asserts it byte-for-byte in every locale.
http_idle = Rien de récupéré pour l'instant
http_checking = Vérification…
ws_title = WebSocket
ws_connect = Se connecter
ws_send = Envoyer
ws_close = Fermer

# Scrolling page (docs/scroll.md): programmatic scroll targets.
scroll_to_top = Aller en haut
scroll_to_bottom = Aller en bas
scroll_to_item = Aller à l'élément 100

# Grid page (docs/grid.md): grid/grid_row from basics to a stress test.
nav_grid = Grille
grid_tab_basics = Bases
grid_tab_sizing = Dimensions
grid_tab_spanning = Fusion
grid_tab_composite = Composition
grid_tab_stress = Endurance
grid_col_name = Nom
grid_col_wins = Victoires
grid_col_points = Points
grid_sizing_fixed = Fixe 80 pt
grid_sizing_content = Contenu
grid_sizing_short = Court
grid_sizing_longer = Une cellule au contenu plus long
grid_month_title = Semainier
grid_event_focus = Bloc de concentration
grid_event_review = Revue
grid_day_n = Jour { $n }
grid_stress_add = Ajouter 50 lignes
grid_stress_bump = Incrémenter la première cellule

nav_animation = Animation
nav_physics = Physique

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
anim_reduced_motion = Mouvement réduit

# --- Page Benchmark (le benchmark Grilles de Day-Bench ; sur les backends Apple natifs, un
#     sélecteur segmenté héberge aussi sa réplique SwiftUI via day-piece-swiftui,
#     docs/swiftui.md) ---
nav_benchmark = Benchmark
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
text_markdown_sample = Day analyse **gras**, *italique*, `code`, ~~barré~~ et [liens](https://daybrite.dev) à l'exécution. Un balisage inachevé comme ** reste littéral.
text_markdown_opened = Ouvert : {$url}
text_baseline_section = Alignement sur la ligne de base
text_baseline_toggle = Aligner les lignes de base
text_baseline_quantity = Quantité
text_baseline_unit = articles
text_baseline_total = Total
text_baseline_currency = USD
text_baseline_due = Échéance
text_link_icons_label = Material Symbols sur Google Fonts
text_link_mail_label = Écrire à l'équipe

# Section Fichiers : texte initial de l'éditeur
files_initial_content =
    Bonjour de Day !
    Modifiez-moi, puis Enregistrer.

# Page Rapports de plantage (day-piece-crash-reporter, docs/break.md)
nav_crash = Rapports de plantage
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
crash_empty = Aucun rapport pour l'instant. Déclenchez un plantage, puis relancez.

# Page Champs de texte (docs/textfield.md)
nav_textfields = Champs de texte
textfields_password_section = Mot de passe
textfields_password = Mot de passe
textfields_password_placeholder = Saisissez un mot de passe
textfields_show_password = Afficher le mot de passe
textfields_length = Longueur
textfields_purpose_section = Clavier et remplissage automatique
textfields_email = E-mail
textfields_phone = Téléphone
textfields_url = Site web
textfields_number = Quantité
textfields_decimal = Montant
textfields_code = Code à usage unique
textfields_pin_section = Code PIN
textfields_pin = Code PIN
textfields_attrs_section = Attributs
textfields_reference = Référence
textfields_read_only = Lecture seule
textfields_limited = Jusqu’à 12 caractères
textfields_message = Message
textfields_message_placeholder = Saisissez puis appuyez sur Envoyer
textfields_sent = Envoyés

# Page Zones de texte
nav_textareas = Édition de texte
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

# Éditeur de texte enrichi (docs/texteditor.md)
textareas_styled_section = Éditeur enrichi
textareas_toolbar_section = Mise en forme
textareas_export_section = Import et export
textareas_text_color = Couleur du texte
textareas_align = Alignement
textareas_align_natural = Naturel
textareas_align_center = Centré
textareas_align_trailing = À la fin
textareas_doc_note = Note
textareas_doc_code = Code
textareas_doc_empty = Vide
textareas_placeholder = Commencez à écrire, ou choisissez un document ci-dessus.
textareas_inspector = Sélection { $start }–{ $end } · { $marks } · taille ×{ $scale }
textareas_export_markdown = Markdown
textareas_export_html = HTML
textareas_export_rtf = RTF
textareas_import = Relire
textareas_select_word = Sélectionner un mot
textareas_no_selection = aucune sélection
textareas_mark_bold = gras
textareas_mark_italic = italique
textareas_mark_underline = souligné
textareas_mark_strike = barré
textareas_mark_plain = simple
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
stack_cover_button = Couverture
cover_title = Écran couvrant
cover_dismiss = Fermer

# Demi-feuille (docs/cover.md) : la moitié basse de la fenêtre, la page restant visible dessous.
stack_half_button = Demi-feuille
half_title = Demi-feuille
half_level = Niveau
half_readout = Niveau de la feuille : { $level }

stack_unsaved = Modifications non enregistrées
stack_discard_title = Abandonner les modifications ?
stack_discard_body = Cette page a des modifications non enregistrées.
stack_discard_ok = Abandonner

tab_dynamic_title = Onglets pilotés par les données
tab_dynamic_add = Ajouter un onglet
tab_dynamic_remove = Retirer un onglet

# Stockage de fichiers local (page services, docs/fs.md).
storage_title = Fichiers locaux
storage_placeholder = Texte à stocker
storage_save = Enregistrer le fichier
storage_load = Charger
storage_delete = Supprimer
storage_files_label = Fichiers stockés
storage_idle = Rien d'enregistré pour l'instant

prefs_window_title = Préférences

# --- Tree page (docs/tree.md) ---
nav_tree = Arborescence
tree_expand_all = Tout déplier
tree_collapse_all = Tout replier
tree_reveal = Révéler tree.rs
tree_add_file = Ajouter un fichier
tree_multi = Sélection multiple
tree_lock = Verrouiller docs
tree_hint = Faites glisser une ligne sur un dossier pour la déplacer. Tant que le verrou est actif, docs refuse les dépôts.
tree_count = { $count ->
    [one] { $count } nœud
   *[other] { $count } nœuds
}
tree_selection_none = Aucune sélection
tree_selection = { $count ->
    [one] Sélection : { $names }
   *[other] Sélection ({ $count }) : { $names }
}
tree_move_none = Aucun déplacement pour l'instant
tree_move = { $name } déplacé dans { $parent } en position { $index }
tree_move_append = { $name } déplacé dans { $parent }
tree_root = la racine
tree_ctx_new_file = Nouveau fichier
tree_ctx_duplicate = Dupliquer
tree_ctx_move_up = Monter
tree_ctx_move_down = Descendre
tree_ctx_delete = Supprimer

# --- Toolbars page (docs/toolbars.md) ---
nav_toolbars = Barres d'outils
toolbar_unsupported = Cette boîte à outils n'a pas de barre d'outils de fenêtre.
toolbar_readout_title = Ce que fait la barre d'outils
toolbar_controls_title = La piloter depuis ici
# Item labels: these appear in the toolbar, so they stay short.
toolbar_new = Nouvelle fenêtre
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
toolbar_last_extra = Infos de version copiées
# The page's own controls.
toolbar_extra_label = Afficher l'élément Copier les infos
toolbar_enabled_label = Afficher la source activé
toolbar_clear_search = Effacer la recherche
toolbar_seed_search = Remplir la recherche
toolbar_seed_text = barres d'outils
# --- day-part-local-notify (docs/notify.md) ---
nav_notify = Notifications
notify_caps_post = Publication prise en charge
notify_caps_unsupported = Indisponibles dans cette version. Sur macOS, les notifications exigent une app signée (day pack).
notify_caps_schedule_os = Les notifications programmées survivent à la fermeture de l'application
notify_caps_schedule_process = Les notifications programmées sont perdues si l'app se ferme
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
notify_post = Publier
notify_cancel = Tout annuler
notify_status_idle = Rien n'a encore été publié
notify_status_posted = Publiée
notify_status_scheduled = Programmée
notify_status_cancelled = Toutes les notifications ont été annulées
notify_status_failed = Publication impossible
notify_last = Dernier résultat
notify_perm_granted = Autorisation de notification accordée
notify_perm_missing = Autorisation non accordée : les notifications sont ignorées
notify_perm_request = Demander l'autorisation
haptics_songs_caption = Séquences
haptics_song_celebration = Célébration
haptics_song_levelup = Niveau supérieur
haptics_song_heartbeat = Battement
haptics_song_cascade = Cascade

# --- scripting (enregistreur dayscript) ---
nav_scripting = Scripts
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
badge_caps_native = L'icône de l'app affiche une pastille
badge_caps_emulated = Envoyée ; le shell décide de l'afficher ou non
badge_caps_none = Aucune API de pastille sur cette plateforme
badge_android_note = Android déduit la pastille des notifications publiées : utilisez plutôt le compteur d'une notification.
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
voice_search_placeholder = Filtrer les parfums…

# Context-menu demos (menus page): the message-list rows and the media card.
menu_reply = Répondre
menu_forward = Transférer
menu_share = Partager…
menu_copy_image = Copier l’image
menu_save_image = Enregistrer dans Photos
menu_get_info = Lire les informations
menus_messages_section = Une liste de messages
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
menu_script = Script
cmd_star = Suivre
cmd_unstar = Ne plus suivre
cmd_toggle_pseudo_locale = Basculer la pseudo-locale

# Speech (day-part-speech) : la référence daybridge. Une API, un langage par plateforme.
speech_title = Synthèse vocale
speech_phrase = Une journée claire, avec un risque de pluie plus tard.
speech_speak = Parler
speech_stop = Arrêter
speech_support_label = Prise en charge ici
speech_state_speaking = Lecture en cours…
speech_state_finished = Terminé
speech_state_stopped = Arrêté
speech_native = Native
speech_emulated = Émulée (partielle)
speech_unsupported = Non prise en charge sur cette cible

# Support banners (widgets.rs): shown over a demo the target cannot run.
support_missing_here = Non pris en charge sur cette plateforme — la démo reste visible, mais elle ne fonctionnera pas ici.
support_emulated_here = Partiellement pris en charge ici — la démo fonctionne, mais sans l'implémentation native de la plateforme.
vectors_live_tint = Teinte dynamique
vectors_cycle_tint = Changer

# Layout page (docs/size-classes.md "Row fit policies")
layout_note = Choisissez une politique, puis ajoutez des composants jusqu'à ce que la rangée manque de place.
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

vectors_alias_note = Pas de variante grasse : le glyphe de base est dessiné
vectors_pick_tint = Choisir une teinte

# La page Liste de contenu (src/pages/content_list) : la liste et l'éditeur du modèle `day new`,
# donc les clés ci-dessous sont celles du modèle, telles quelles ; le code copié les lit par nom.
nav_content_list = Liste de contenu
nav_items = Éléments
cmd_add = Nouvel élément
cmd_delete = Supprimer
items_empty = Aucun élément pour le moment. Ajoutez votre premier élément.
item_none = Sélectionnez un élément à modifier.
item_untitled = Sans titre
field_name = Nom
field_name_hint = Nommez cet élément…
field_done = Terminé

# La page Curseurs (day/docs/cursor.md) : le décorateur `.cursor()`, forme par forme.
# La page Partage (day/docs/split.md) : deux volets, un séparateur, un même état lu des deux côtés.
nav_split = Partage
split_support_title = Sur cette boîte à outils
split_support_label = Séparateur dessiné par
split_native = Le séparateur natif de la boîte à outils
split_composed = Day (cette boîte à outils n'a pas de séparateur)
split_hint = Faites glisser le séparateur ou déplacez le curseur de part.
split_arrangement_title = Disposition
split_stacked = Empilés
split_share = Part du formulaire
split_form_title = Formes
split_caption = Légende
split_caption_placeholder = Dessinée par-dessus les formes
split_count = Nombre
split_radius = Rayon
split_hue = Teinte
split_filled = Pleines
split_shape = Forme
split_shape_circle = Cercle
split_shape_square = Carré
split_shape_diamond = Losange
split_readout = { $count } formes de rayon { $radius }

nav_cursors = Curseurs
cursors_support_title = Sur cette boîte à outils
cursors_support_label = Prise en charge ici
cursors_native = Native
cursors_emulated = Émulée (formes les plus proches)
cursors_unsupported = Non prise en charge sur cette cible
cursors_hint = Passez le pointeur sur une tuile.
cursors_shapes_title = Formes du pointeur
cursors_drag_title = Glisser-déposer
cursors_resize_title = Redimensionner et zoomer
cursors_busy_title = Occupé
cursors_reactive_title = Une boîte, plusieurs formes
cursors_reactive_pick = Forme
cursors_nested_title = L'ancêtre le plus proche l'emporte
cursors_nested_outer = Un réticule ici
cursors_nested_inner = Une main ici
cursors_native_title = Propres à cette boîte à outils
cursors_native_none = Cette boîte à outils ne nomme aucune forme au-delà du jeu commun.

# Motifs des autorisations : le texte affiché par iOS et HarmonyOS quand l’app demande.
permission_camera = Afficher l'aperçu et prendre une photo sur la page Appareil photo.
permission_location_when_in_use = Afficher vos coordonnées en direct sur la page Appareil et capteurs.
permission_motion = Afficher en direct les courbes de l'accéléromètre, du gyroscope et du magnétomètre sur la page Appareil et capteurs.
permission_NSBluetoothAlwaysUsageDescription = Montrer comment Day demande le Bluetooth — la vitrine ne se connecte jamais à un appareil.

# Camera (src/pages/camera.rs): the viewfinder, its permission, and the last photo.
nav_camera = Appareil photo
camera_permission_section = Autorisation
camera_permission_label = Accès à l'appareil photo
camera_viewfinder_section = Viseur
camera_take_photo = Prendre une photo
camera_flip = Retourner
camera_state_label = Appareil photo
camera_last_photo_section = Dernière photo
camera_photo_label = Photo
camera_no_photo = Aucune pour l'instant
camera_photo_size = { $width } × { $height } · { $name }

list_activated = Dernière ligne activée : { $row } (0 = aucune)

ctl_icon_label = Icône et libellé
ctl_icon_only = Icône seule
ctl_image = Image intégrée

nav_drag_drop = Glisser-déposer
dnd_hint = Faites glisser une tuile sur une autre tuile ou une autre fenêtre de Showcase. Les cercles marqués d'un X refusent les dépôts.
dnd_ready = Prêt à recevoir un dépôt.
dnd_empty = Déposer ici
dnd_new_window = Nouvelle fenêtre
dnd_choose_file = Choisir un fichier…
dnd_accept = Accepter les dépôts

dnd_data = { $mime } : { $count } octets

files_reopen = Rouvrir le dernier fichier
files_bookmark_failed = Impossible de rétablir l’accès au fichier enregistré. Sélectionnez-le à nouveau avec Ouvrir.

dnd_disallowed = Ajouter des zones interdites

# Native display frame demo
frame_hint = Touchez pour lancer les billes. Faites-en glisser une pour la projeter.
frame_bounce = Rebondir
frame_shuffle = Mélanger
frame_pause = Pause
frame_waiting = En attente d’une image
frame_running = En mouvement
frame_paused = En pause
frame_resting = Au repos
frame_balls_label = Billes
frame_fps = FPS · min { $min } · max { $max } · moy { $avg }

commands_title = Commandes réutilisables
commands_add = Ajouter un

# The Physics page (src/pages/physics.rs): the ball mass and planetary gravity sliders, and the
# bodies the gravity readout names.
physics_gravity_label = Gravité
physics_gravity = { $value } m/s²
physics_gravity_named = { $value } m/s² ({ $body })
physics_mass_label = Masse
physics_mass = { $value } Mt
physics_elasticity_label = Élasticité
physics_elasticity = { $value } %
physics_fullscreen = Plein écran
physics_exit_fullscreen = Quitter le plein écran
physics_zero_g = apesanteur
physics_moon = Lune
physics_mars = Mars
physics_earth = Terre
physics_jupiter = Jupiter

# HTTP sessions, interception and payload statistics.
net_transport = Transport
net_provider = Fournisseur HTTP
net_native = Natif
net_reqwest = reqwest
net_native_only = Cette cible utilise le fournisseur natif.
net_provider_note = S’applique aux nouvelles requêtes et aux nouveaux sockets.
net_simulation = Simulation réseau
net_intercept = Intercepter les requêtes
net_speed = Débit
net_speed_value = { $speed } Kio/s par transfert
net_failures = Taux d’échec
net_failure_value = { $rate } % des requêtes
net_pause = Suspendre la réception
net_offline = Hors ligne
net_method = Méthode
net_url = URL
net_body = Corps de la requête
net_cancel = Annuler
net_cancelled = Annulé
net_download_preset = Préréglage téléchargement
net_upload_preset = Préréglage envoi
net_received = { $bytes } octets reçus
net_error = Échec de la requête : { $error }
net_request = Requête HTTP
net_ws_note = Saisissez une URL WebSocket réelle, ou activez l’interception pour l’écho simulé.
net_message = Message
net_connected = Connecté
net_closed = Fermé
net_real_down = Réception · réel
net_real_up = Envoi · réel
net_sim_down = Réception · simulé
net_sim_up = Envoi · simulé
net_seconds = Secondes
net_rate = Kio/s
net_series = Trafic
net_totals = Envoyé { $sent } octets · reçu { $received } octets · actifs { $active } · échecs { $failed }
net_average = Moyenne de session : envoi { $upload } Kio/s · réception { $download } Kio/s
net_statistics = Trafic en direct
net_send_request = Envoyer la requête
net_options = Options de requête
net_headers = En-têtes (JSON)
net_timeout = Délai d’inactivité
net_timeout_value = { $seconds } secondes

# SI units used by the motion-sensor readouts.
sensor_unit_acceleration = m/s²
sensor_unit_rotation = rad/s
sensor_unit_magnetic = µT

sensor_live = Mesures en direct
sensor_compass = Boussole magnétique
sensor_compass_hint = Tenez le téléphone à plat. Le rouge indique le nord magnétique.
sensor_north = N
sensor_east = E
sensor_south = S
sensor_west = O
sensor_heading = Cap magnétique : { $degrees }°

sensor_calibrated = Calibré
sensor_tilt = Niveau d’accélération X/Y
sensor_tilt_hint = La bulle suit l'accélération X/Y, un g au bord.
sensor_rotation = Indicateurs de vitesse de rotation
sensor_rotation_hint = Vitesse de rotation par axe, ±π rad/s aux extrémités.
sensor_uncalibrated_hint = Non calibré : l'aiguille peut ne pas indiquer le nord.
sensor_axis_x = X
sensor_axis_y = Y
sensor_axis_z = Z

# --- Sécurité : le trousseau et l'invite biométrique (docs/keychain.md, docs/biometrics.md) ---
nav_security = Sécurité
keychain_title = Trousseau
keychain_support_label = Coffre ici
keychain_backend_label = Moteur
keychain_key_label = Clé
keychain_key_placeholder = showcase.session
keychain_secret_label = Secret
keychain_secret_placeholder = Quelque chose à protéger
keychain_protection_label = Protection
keychain_level_unlocked = Appareil déverrouillé
keychain_level_first_unlock = Après le premier déverrouillage
keychain_level_presence = Présence de l'utilisateur
keychain_save = Enregistrer
keychain_read = Lire
keychain_read_presence = Lire avec vérification
keychain_remove = Supprimer
keychain_idle = Rien d'enregistré pour l'instant
keychain_saved = { $key } enregistré
keychain_read_value = Relu : { $value }
keychain_removed = { $key } supprimé
keychain_entries_label = Clés enregistrées
keychain_entries_none = Aucune
keychain_presence_reason = Lire le secret enregistré par la page Sécurité.
biometrics_title = Biométrie
biometrics_support_label = Prise en charge ici
biometrics_availability_label = Disponibilité
biometrics_biometry_label = Capteur
biometrics_reason_placeholder = Confirmez votre identité pour continuer.
biometrics_credential_label = Autoriser le code de l'appareil
biometrics_authenticate = S'authentifier
biometrics_cancel = Annuler
biometrics_idle = Pas encore demandé
biometrics_prompting = En attente de l'invite…
biometrics_verified = Vérifié ({ $method })
permission_biometrics = Confirmez votre identité sur la page Sécurité.

# --- Connexion : day-part-auth (navigateur d'authentification, OpenID Connect, clés d'accès, Apple, Google) ---
nav_signin = Connexion
signin_oidc_title = OpenID Connect
signin_provider_demo = Fournisseur de démonstration (intégré)
signin_provider_google = Google
signin_provider_apple = Apple (flux web)
signin_provider_custom = Tout émetteur (découverte)
signin_issuer_placeholder = URL de l'émetteur, découverte à la connexion
signin_client_id_placeholder = Identifiant client, tel qu'enregistré chez le fournisseur
passkeys_rp_placeholder = Partie utilisatrice (un domaine associé à cette application)
signin_sign_in = Se connecter
signin_sign_in_silent = Connexion silencieuse
signin_refresh = Rafraîchir
signin_userinfo = Userinfo
signin_revoke = Révoquer
signin_sign_out = Se déconnecter
signin_idle = Non connecté
signin_working = En attente du fournisseur…
signin_signed_in = Connecté en tant que { $name } ({ $email }), sujet { $sub }
signin_tokens = Jeton d'accès valable { $secs } s · portée { $scope } · { $refresh }
signin_refresh_yes = jeton de rafraîchissement émis
signin_refresh_no = pas de jeton de rafraîchissement
signin_need_client_id = Collez d'abord l'identifiant client enregistré chez le fournisseur.
signin_need_discovery = Indiquez d'abord l'émetteur et l'identifiant client ; la découverte a lieu à la connexion.
signin_silent_demo_only = La connexion silencieuse est celle du fournisseur de démonstration ; les autres passent par leur page de consentement.
signin_nothing_to_refresh = Connectez-vous d'abord : il n'y a pas de jeton de rafraîchissement à échanger.
signin_refreshed = Rafraîchi : un nouveau jeton d'accès a été émis
signin_userinfo_ok = Userinfo a répondu pour { $sub }
signin_revoked = Le jeton d'accès a été révoqué
signin_signed_out = Déconnecté
passkeys_title = Clés d'accès
passkeys_available_yes = Authentificateur de la plateforme disponible
passkeys_available_no = Aucun authentificateur de plateforme sur cet appareil
passkeys_create = Créer une clé d'accès
passkeys_get = Se connecter avec une clé d'accès
passkeys_idle = Aucun identifiant pour l'instant
passkeys_prompting = En attente de l'authentificateur…
passkeys_created = Identifiant créé { $id }
passkeys_asserted = Connecté avec l'identifiant { $id }
apple_title = Se connecter avec Apple
apple_sign_in = Se connecter avec Apple
apple_signed_in = Utilisateur Apple { $user }
google_title = Google
google_client_placeholder = Identifiant client du serveur
google_sign_in = Se connecter avec Google
google_signed_in = Compte Google { $id }

# Numeric notification count.
badge_count = { $count }

# Map (src/pages/map.rs): day-piece-map over the built-in sample island or a live OpenFreeMap
# style, the camera calls on buttons, and a readout of what the piece reports.
nav_map = Carte
map_source_sample = Île d'exemple
map_source_liberty = OpenFreeMap Liberty
map_source_positron = OpenFreeMap Positron
map_source_bright = OpenFreeMap Bright
map_source_fiord = OpenFreeMap Fiord
map_source_dark = OpenFreeMap Dark
map_source_colorful = VersaTiles Colorful
map_source_colorful_dark = VersaTiles Colorful Dark
map_fly = Survoler
map_fit = Cadrer
map_rotate = Pivoter
map_north = Nord
map_sight = { $sight ->
        [center] Centre de l'île
        [coast] Côte est
        [trafalgar] Trafalgar Square
        [here] Votre position
       *[bridge] Tower Bridge
    }
map_readout = { $status ->
        [ready] Prête
        [failed] Échec : { $reason }
       *[loading] Chargement
    } · { $lat }, { $lng } · zoom { $zoom } · { $bearing ->
        [0] Nord en haut
       *[other] Cap { $bearing }°
    }
map_tap_prompt = Touchez la carte
map_tap_marker = Repère : { $sight ->
        [center] Centre de l'île
        [coast] Côte est
        [trafalgar] Trafalgar Square
        [here] Votre position
       *[bridge] Tower Bridge
    }
map_tap_feature = { $layer } : { $name }
map_tap_layer = { $layer }
map_tap_nothing = Rien ici
map_locate = Ma position
map_locating = Recherche de votre position…
map_locate_failed = { $why ->
        [denied] L’accès à la position est désactivé pour cette app. Activez-le dans les réglages.
        [off] Les services de localisation sont désactivés.
        [timeout] Pas encore de position. Réessayez.
        [unavailable] La localisation n’est pas disponible sur cet appareil.
       *[other] Impossible de trouver votre position.
    }

# Interactive layout sizing and translation.
layout_reactive_title = Dimensions réactives
layout_reactive_width = Largeur
layout_reactive_height = Hauteur
layout_reactive_horizontal = Décalage horizontal
layout_reactive_vertical = Décalage vertical
layout_reactive_reset = Réinitialiser
layout_reactive_points = { NUMBER($value, maximumFractionDigits: 0) } pt
layout_reactive_shape = Forme : { NUMBER($width, maximumFractionDigits: 0) } sur { NUMBER($height, maximumFractionDigits: 0) } points

push_caption = Envoyez une notification push de test à cet appareil par le relais Day.
push_unavailable = Les notifications push ne sont pas disponibles dans cette version.
push_send_now = Envoyer maintenant
push_send_later = { $seconds ->
    [one] Envoyer dans 1 seconde
   *[other] Envoyer dans { $seconds } secondes
}
push_hint = Le premier envoi demande l'autorisation.
push_sending = Envoi…
push_requested = En attente du jeton de l'appareil fourni par le service push…
push_no_token = Le service push n'a pas fourni de jeton. Réessayez dans un instant.
push_denied = Les notifications sont désactivées pour cette app. Autorisez-les dans les Réglages, puis renvoyez.
push_too_soon = Le relais accepte un test toutes les 30 secondes. Patientez un instant, puis renvoyez.
push_network_busy = Le relais a inscrit beaucoup d'appareils depuis ce réseau au cours de la dernière heure. Réessayez plus tard.
push_accepted = Envoyée. Elle devrait arriver dans un instant.
push_scheduled = { $seconds ->
    [one] Arrivée dans 1 seconde. Quittez l'app pour la voir en arrière-plan.
   *[other] Arrivée dans { $seconds } secondes. Quittez l'app pour la voir en arrière-plan.
}
push_received = Reçue pendant que l'app était ouverte : { $title }
push_opened = Ouverte depuis la notification : { $title }
push_advanced = Avancé
push_token = Jeton de l'appareil
push_token_apns = Reçu d'APNs
push_token_fcm = Reçu de FCM
push_token_none = Pas encore reçu
push_relay = Inscription au relais
push_relay_none = Non inscrit
push_sandbox = Utiliser l'environnement de test APNs (versions de développement)
push_waiting = Aucune notification reçue pour l'instant
push_remove = Retirer du relais
push_removed = Appareil retiré du relais
push_error = Échec de l'envoi push : { $error }

# Démonstrations du partage, des listes groupées et du retour de l’application.
list_opt_inset_grouped = Groupes en retrait
list_opt_inset_hint = Lignes en retrait sur macOS et groupes arrondis sur iOS. Les autres plateformes conservent leur liste habituelle.
share_demo_title = Partage
share_demo_fallback = Cette plateforme n’affiche pas d’actions de partage personnalisées. Utilisez les boutons ci-dessous.
share_demo_open = Partager le site Day…
share_demo_remember = Retenir le lien
share_demo_clear = Effacer le résultat
share_demo_remembered = Action « Retenir le lien » sélectionnée
share_demo_cleared = Résultat du partage effacé
share_demo_failed = Impossible d’ouvrir le menu de partage.
share_demo_receive_hint = Partagez un texte ou un lien vers Day Showcase depuis une autre app.
share_demo_none = Aucun contenu reçu.
system_back_status = Retour de l’application : { $remaining } restants, { $handled } traités
system_back_press = Retour
system_back_stop = Désactiver
system_back_hint = Les pages empilées et les couvertures traitent Retour avant l'app.
system_back_arm = Activer 3 retours

# Map provider selection and setup.
map_provider_osm = OpenStreetMap
map_provider_apple = Apple Maps
map_provider_google = Google Maps
map_missing_api_key = Configurez une clé API Google Maps pour cette application Android.
map_play_services_unavailable = Google Maps nécessite les services Google Play.
map_load_failed = Impossible de charger la carte. Vérifiez la connexion et la configuration du fournisseur.

resources_missing = Ressource intégrée indisponible : { $name }

# The gallery pages added with the card layout, and the line under every page's title.
nav_styles = Styles
nav_images = Images
nav_gestures = Gestes
detail_controls = Boutons, interrupteurs, curseurs, incrémenteurs, sélecteurs et progression
detail_dates = Sélecteurs de date et d’heure, dans le calendrier de la plateforme
detail_focus = Le focus clavier comme état de l’application
detail_text = Polices, segments stylés, Markdown, sélection et troncature
detail_text_areas = Édition multiligne, modes code et Markdown en direct
detail_text_fields = Saisie sur une ligne, claviers, saisie sécurisée et recherche
detail_toolbars = Commandes de fenêtre et de page dans les barres de la plateforme
detail_localization = Quatre langues, pluriels et disposition de droite à gauche
detail_canvas = Tracés, dégradés, texte et images dessinés à chaque image
detail_styles = Coins, bordures, ombres, découpe et transformations
detail_images = Modes d’ajustement, coins, images distantes, images d’attente et fondus
detail_gestures = Glisser, pincer, pivoter, appuyer et survoler sur n’importe quelle vue
detail_charts = Une grammaire de marques : lignes, barres, aires et sélections
detail_animation = Animations de la plateforme, interpolations, courbes et ressorts
detail_physics = Des billes et un ressort sur l’horloge d’images
detail_benchmark = Un patchwork de vues vivantes, chronométré face à SwiftUI
detail_grid = Colonnes fixes, fusions, alignement et dimensions
detail_layout = Rangées, colonnes, couches, retour à la ligne et classes de taille
detail_split = Volets redimensionnables avec les séparateurs de la plateforme
detail_list = Lignes recyclées, sections, balayages, réordonnancement et sélection
detail_tree = Des arborescences qui se déplient, se sélectionnent, se réordonnent et se chargent à la demande
detail_drag_drop = Texte, fichiers et images glissés entre vues et applications
detail_model = Stores typés, valeurs dérivées et annulation
detail_query = Une base de données interrogée en direct, avec recherche et tri
detail_tabs = Barres d’onglets et pages segmentées, chacune avec son état
detail_stack = Pages empilées, un chemin qui vous appartient et le geste de retour
detail_content_list = Une liste, ses éléments et un détail en trois colonnes
detail_media = Un lecteur vidéo natif avec ses commandes de lecture
detail_lottie = Des animations vectorielles à lire, mettre en pause et parcourir
detail_map = Tuiles vectorielles sur le canevas, à côté des cartes de la plateforme
detail_web_view = Un site embarqué et le web en direct, avec JavaScript dans les deux sens
detail_menus = Menus, menus contextuels, alertes, feuilles et invites
detail_system = Batterie, capteurs, localisation, infos sur l’appareil et apparence
detail_network = Requêtes, envois, téléchargements, proxys et connectivité
detail_notify = Notifications locales et pastille de l’application
detail_push = Notifications distantes via un relais à essayer
detail_speech = Voix, effets sonores et haptique
detail_camera = Aperçu en direct, prise de vue et la photo obtenue
detail_cursors = Toutes les formes de pointeur que propose la plateforme
detail_files = Presse-papiers, préférences, sélecteurs de fichiers et stockage de l’application
detail_security = Le trousseau et la biométrie
detail_sign_in = Connexion avec Apple, Google, une clé d’accès ou tout fournisseur OpenID
detail_scripting = Les scripts qui pilotent cette application et en font des captures d’écran
detail_resources = Images embarquées, glyphes vectoriels, polices et fichiers de données
detail_tweaks = Ajustements des vues natives, propres à chaque boîte à outils
detail_crash_reporting = Plantages capturés et signalés au lancement suivant
detail_about = Ce qu’est Day, et ce que montre cette application
detail_settings = Apparence, langue et préférences de l’application
