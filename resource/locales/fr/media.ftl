# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Media page's cards (src/pages/media.rs).

media_clip = L’ouragan Milton vu de la station spatiale · NASA
media_clip_title = L’ouragan Milton vu de la station spatiale
media_clip_credit = NASA
media_played = Lu
media_buffered = En tampon
media_buffered_unavailable = Ce toolkit n’indique pas les plages en tampon.
media_playback_section = Lecture
media_rate = Vitesse
media_volume = Volume
media_mute = Couper le son
media_volume_fixed = Sur cette plateforme, la vidéo permet seulement de couper le son.
media_system_section = Commandes du système
media_now_playing = Lecture en cours
media_remote_title = Dernière commande
media_system_note = Les commandes multimédias du système et les touches multimédias affichent cette vidéo et la pilotent.
media_readout_section = Relevé
media_stream_title = Titre du flux
media_error_title = Erreur
media_none = Aucun
media_state = { $state ->
    [idle] Inactif
    [loading] Chargement
    [playing] En lecture
    [paused] En pause
    [ended] Terminé
   *[error] Échec
}
media_error = { $kind ->
    [invalid] Valeur non valide : { $operation }
    [unsupported] Non pris en charge ici : { $operation }
    [playback] Échec de la lecture
   *[command] Échec de la commande : { $operation }
}
media_remote = { $command ->
    [play] Lecture
    [pause] Pause
    [toggle] Lecture ou pause
    [stop] Arrêt
    [next] Suivant
    [previous] Précédent
    [seek] Déplacement
    [forward] Avance
    [backward] Retour
   *[none] Aucune
}
