# PROGRESS

Mémoire de travail du projet. À relire, avec `git log`, au début de chaque session.
Règle : une tâche n'est cochée que si sa vérification est verte : le workflow GitHub Actions (lien ; gratuit depuis
que le dépôt est public), ou à défaut `tools/ci-local.sh` (journal `engine/.local/ci/<commit>.log`) et
`tools/e2e-local.sh` pour le jeu. « constaté » = résultat d'outil observé, « supposé » = non vérifié.

## État courant (2026-11-26)

- GitHub Actions ne démarre plus de job (run 45764065757 : « recent account payments have failed or your spending
  limit needs to be increased ») et l'utilisateur ne paiera pas. Remplacé par `tools/ci-local.sh` (fmt, clippy
  -D warnings, tests de nos crates `pvp-*`, build du client ; 2 jobs pour tenir dans 8 Go). Première passe :
  `ci-local 6382b21+dirty: PASS` (fmt 3 s, clippy 49 s, tests 679 s, build 138 s). Le code Horizon n'est pas
  re-testé localement : nous ne le modifions pas.
- Fichiers 26.3 : téléchargés le 26/11 depuis les serveurs Mojang d'après le `26.3.json` du launcher officiel
  (5 263 fichiers vérifiés SHA-1, 248 Mio) à la demande de l'utilisateur ; ressources préparées
  (`horizon-prepare --prepare --complete`, 191 s) dans `engine/.local/horizon-client/26.3`.
- Travail en cours sur la branche `phase1` (main inchangé depuis 923c072).

- Documents validés le 25/11 (D1-D4, D6 ; fair-play : toutes les fonctionnalités Polyfrost autorisées).
- Nouvelle consigne : **reprendre à l'identique l'UI de OneClient/OneConfig** (menu, HUD, launcher).
- Dépôt **public** depuis le 26/11 (demande de l'utilisateur) : https://github.com/valthvn/rust-pvp-client (subtrees `engine/` = Horizon ae632edf48,
  `launcher/` = OneLauncher 39f0cf3).
- Phase 0 en cours (CI), phase 1 commencée (`pvp-core` écrit, testé localement).
- Session arrêtée à la demande de l'utilisateur après ce lot.

## Checklist

### Exploration
- [x] Horizon cloné : snapshot `ae632edf48` (shallow, `C:\Users\Valentin\Documents\src-study\horizon`)
      + historique complet partiel (`src-study\horizon-full`, depuis `rdbtCVS/horizon`)
- [x] OneLauncher cloné (`src-study\OneLauncher`)
- [x] Liste des 140 dépôts Polyfrost (`src-study\polyfrost_repos.json`)
- [x] docs/ARCHITECTURE.md
- [x] docs/MODULES.md (75 dépôts classés, ~65 écartés comme non-mods)
- [x] docs/ROADMAP.md
- [x] Validation utilisateur des documents (25/11 ; D5 à sa charge, D7-D8 ouvertes)

### Phase 0 — Dépôt et CI
- [x] Monorepo local (subtrees `engine/`, `launcher/`, remotes `upstream-horizon`, `upstream-onelauncher`) — constaté
- [x] Dépôt GitHub, push — privé jusqu'au 26/11, puis **public** à la demande de l'utilisateur (constaté
      `visibility PUBLIC`) ; historique audité avant : aucun fichier Mojang, ressource préparée ni secret (seule une
      clé de test d'Horizon, déjà publique en amont), auteurs en adresses noreply.
- [x] ci.yml vert (Linux) sur `phase1` : https://github.com/valthvn/rust-pvp-client/actions/runs/45818522378
      (engine : fmt, clippy -D warnings, tests du workspace ; launcher). Matrice complète : au merge sur main.
      - run 45702723724 (923c072) : engine macOS ✅, Windows ✅ ; engine Linux ❌ tué (exit 137, mémoire) pendant
        la compilation des tests ; launcher ❌ : `oneclient_db` (tests) utilise des `sqlx::query!` absents du cache
        hors ligne `.sqlx`. Correctif db48831 (branche phase1) : swap 12 Go + tests sans debuginfo, 3 jobs ;
        clippy amont `continue-on-error` sans `--all-targets`, tests launcher sans `oneclient_db`.
        Non vérifié : la CI ne démarre plus (facturation, voir État courant).
      - run 45695282544 : engine macOS ✅ (clippy horizon-game 2 min 23 s), engine Windows ✅,
        engine Linux : en cours à l'arrêt de session ; launcher ❌ sur les 3 OS (OneLauncher amont non
        conforme à rustfmt stable / clippy -D warnings) → ci.yml corrigé : vérification stricte seulement
        sur nos crates du launcher, clippy amont en rapport seulement. À revérifier.
- [x] build.yml : https://github.com/valthvn/rust-pvp-client/actions/runs/45818523221 — client et launcher release
      sur Linux, Windows, macOS. Constaté : le client Windows de l'artifact démarre, écrit ses manifestes de mods
      intégrés, Shift droit ouvre le menu ; 46 FPS (release, iGPU UHD 620, 2560×1440) contre 29 en debug.
- [ ] e2e.yml (selon D4)
- [ ] Benchmark de référence (docs/PERF.md)

### Phase 1
- [~] 1.2-1.4 `pvp-core` : trait `Module` (une méthode par événement), schéma `Setting`, registre à masques
      atomiques (dispatch sans allocation, panique → module désactivé), profils TOML versionnés (migrations,
      .bak, écriture atomique, dupliquer/renommer/supprimer/importer/exporter), placement HUD (ancres,
      magnétisme), adaptateur `mod_api` (`runtime.rs`, keybinds `pvp:<module>.<id>`).
      Constaté localement : 17 tests OK, clippy -D warnings OK. Pas encore vérifié en CI ni en jeu.
      Limite : l'échelle des éléments HUD est stockée mais pas rendue (pas de mise à l'échelle dans `DrawContext`).
- [~] 1.1 `pvp-client` : exécutable écrit (`registry::init(pvp_modules::ALL)` puis
      `main_with(vec![pvp_core::runtime::entry(), pvp_ui::entry()])`, sans les parties en ligne d'Horizon :
      heartbeat, serveur officiel, amis, Discord). Constaté : compile en local (debug).
- [~] 1.6 socle de rendu `pvp-ui` : OneConfig actuel est en Jetpack Compose (PolyUI n'est plus utilisé). Constaté
      dans l'API : textures dynamiques (`create_dynamic_texture`, 1024 px de côté max, 4 Mio par mod),
      `mod_screen_open` (écran du mod, en jeu seulement), hooks souris/clavier d'écran, `gui_scale()`,
      `render_frame_pre` (&mut host, avant le rendu). Donc police Poppins (OFL) rastérisée via ab_glyph et coins
      arrondis rastérisés dans un atlas 1024×512 → reproduction fidèle sans toucher au moteur.
      Écrit : `atlas.rs` (shelf packer, plein → vidé avant l'image suivante), `font.rs`, `shape.rs`, `icons.rs`
      (33 SVG OneConfig via resvg), `gfx.rs` (Painter en unités dp, échelle et opacité), `theme.rs` (palette
      Polyfrosted Dark exacte), `ctx.rs` (entrées, animations 150 ms FastOutSlowIn), `widgets.rs` (switch,
      checkbox, slider + champ 87 dp, select, keybind, champ texte, recherche, boutons, puces, entrées de nav).
      Spécification : `research/oneconfig/ui-spec.md` (déléguée, vérifiée par échantillon : palette `Impls.kt:81-98`,
      fenêtre 1391×700 `OneConfigInterface.kt:217`, accent `ThemeConfig.java:51` — conformes).
- [x] 1.5 Shift droit (`pvp_ui:menu`, `key.keyboard.right.shift`) → `mod_screen_open` — e2e PASS.
- [~] 1.6 menu : shell OneConfig (fenêtre 1391×700 dp, ombre, barre latérale 264 dp, en-tête, recherche,
      ouverture 600 ms ease-out-expo), page Mods (puces de catégorie, grille 4×255×140), page module (cartes
      d'option et contrôle par type de réglage, popup select, sélecteur de couleur), Profils, Raccourcis.
      Tests unitaires : 17 OK (pvp-ui). Non testé en jeu. Écarts connus : bordure en dégradé des cartes rendue
      unie, pas de tête de joueur (pas d'API client du nom/skin), pas de transition de page (300 ms).
- [~] 1.8 module témoin FPS (`pvp-modules`) : écrit, test unitaire OK.
- [~] 1.7 éditeur HUD (`pvp-ui/src/hud_editor.rs`, « Edit HUD » dans la barre latérale) : éléments des modules
      actifs dessinés en place, contours OneConfig (blanc 0.2 / 0.5 survol / accent sélection), glisser avec
      magnétisme 6 px (bords, marges 4 px, centres, autres éléments ; Alt désactive), guides accent 1 px, badge
      « L × H ». Pas de redimensionnement : `DrawContext` n'a pas d'échelle (limite notée). Non testé en jeu.
- [x] e2e en jeu (`tools/e2e-local.sh`, horizon-ctl, dossier de jeu `engine/.local/e2e-game`) :
      `e2e-local 3b388fb(+modifs): PASS` le 26/11 — monde chargé ; Shift droit ouvre le menu (`screen mod`) ; le
      pied de la carte FPS active le module ; la carte ouvre sa page ; Edit HUD ; glisser déplace l'élément ;
      le profil sur disque a `enabled = true` et la nouvelle position. Captures dans `.local/e2e-game/shots/`.
      Constaté en jeu : texte Poppins, icônes, coins arrondis, palette OneConfig ; 28-37 FPS (build debug, iGPU
      Intel UHD 620, échelle GUI 6, 2560×1440).
      Correctifs trouvés en jeu : pile du thread principal (build.rs, `/STACK`), manifestes des mods intégrés
      (`horizon-native-mods/`), `blur_behind` floutait le menu, atlas saturé (étagères par paliers de 8 px,
      rastérisation à l'échelle finale pendant l'ouverture, un seul rayon pour l'ombre), glisser rapide de
      l'éditeur (événements traités dans l'ordre, avec leur position).
- Changement moteur (fork) : `horizon-client/src/game/runtime/control.rs` +1 ligne, touche `rshift` du socket de
  contrôle (outil de test, remontable en amont).
- Restes : traductions des clés `pvp.*` (repli lisible en attendant), tête du joueur, transitions de page,
  mise à l'échelle HUD.

### Phase 2 (branche `phase2` ; `main` = e0b87d5, phase 1 fusionnée en avance rapide le 26/11 après e2e PASS)
- [x] Événement souris `Events::MOUSE` dans `pvp-core` (hook `mouse_button_post`) pour le CPS.
- [x] Lot 1, tous sur l'API de mods (aucune modification moteur) ; style de texte HUD partagé (`text.rs`) :
      Ping, Coordonnées (+ direction), Horloge, Keystrokes + CPS, Armure (durabilité), Effets, Toggle Sprint/Sneak
      (touche R, tient le mapping vanilla), Crosshair (croix/T/point, contour, cache le calque vanilla), Hitbox
      (boîtes + regard, `render_level_stage`), Name tags (le sien en 3e personne, masquer joueurs/autres), Zoom (C).
      Constaté en jeu le 26/11 (profil tout activé, `.local/p2-game/shots/`) : tous s'affichent ; armure et
      effets après `/item replace` et `/effect give` ; R → « [Sprinting (Toggled)] » ; hitbox visible en F5.
      Tests unitaires 12 OK, clippy -D warnings OK.
      Écarts : nametag du joueur local non observé en F5 (le jeu ne consulte peut-être pas le hook pour lui) ;
      la hitbox suit la position du tick (pas d'interpolation exposée) ; crosshair : l'indicateur d'attaque
      vanilla disparaît avec le calque ; libellés de keystrokes fixes (W A S D, QWERTY).
- [~] Heure et météo côté client (`sky.rs`) via le hook `sky_environment` (`Events::SKY`). Tests unitaires OK ;
      non vérifié en jeu : le jeu ne pose la question que connecté à un serveur (en solo il dessine le monde qu'il
      fait tourner, choix d'Horizon testé par `sky_environment_tests`).
- [x] Glint et Damage tint (`look.rs`). Pas d'API de mods pour ces couleurs → hook moteur minimal (fork, +17/-2 lignes) :
      `horizon_client_render::pvp::set_frame_override` (une fonction installée une fois au démarrage par l'exécutable)
      appliquée dans `pvp_render_frame` / `pvp_glint_texel` ; sans override, comportement inchangé. Constaté en jeu le
      26/11 : épée enchantée à glint rouge ; vache frappée teintée en bleu à 70 % (`shots/cow-hurt.png`, `cow.png`).
- [ ] Flou de menu.
- [ ] e2e par module, bench tout activé / tout désactivé.

### Phase 3 (branche `phase3` ; `main` = 8c79168, phase 2 fusionnée le 26/11)
- Périmètre : chat, particules, animations. Le confort Hypixel (Hytils) a été retiré de docs/ROADMAP.md (modification
  locale non commitée, pas de moi) : traité comme une décision de l'utilisateur, non implémenté.
- Horizon a déjà ces fonctions en interne sans API de mods → trois boutons moteur de plus, même forme que
  `set_frame_override` (rien d'installé = comportement inchangé) : `pvp::set_chat_override`,
  `pvp::set_combat_override` (apparence seulement : `Combat::rules` n'est jamais remplacé),
  `ParticleEngine::set_scale` (multiplicateur par type dans `add_with`, fractions accumulées, sans RNG).
- [x] Chat (`chat.rs`) : horodatage, compactage, copie au clic, Ctrl+F. Constaté en jeu le 26/11 : « [19:16] <…> hello
      (x3) » après trois envois.
- [x] Particules (`particles.rs`) : crit, enchanted_hit (×0-10), sweep, cœurs de dégâts, explosions (masquer), autres
      (×0-2). Constaté via `horizon-ctl status` : 300 flammes demandées → 0 (autres = 0) ; 100 crits → 100 (×1) puis
      300 (×3).
- [x] Animations 1.8 (`animations.rs`, via Combat feel Legacy d'Horizon) : blocage à l'épée, pose et swing 1.8.9,
      indicateur d'attaque, sons 1.9 coupés. Constaté en jeu : clic droit maintenu → épée en position de blocage
      (`shots/p3-block.png` vs `p3-idle.png`).
- [ ] Recherche/copie du chat non testées en jeu (Ctrl+F, clic) ; onglets/filtres et captures de chat non faits.

### Phase 4 (branche `phase4` ; `main` = f9a32d1, phase 3 fusionnée le 26/11)
- Client ID Microsoft : `MICROSOFT_CLIENT_ID` vient de `PVP_MICROSOFT_CLIENT_ID` à la compilation ; sans lui (build
  local), celui de Polyfrost hérité d'OneLauncher, **pour tester seulement** (décision de l'utilisateur le 26/11, jamais
  distribué). `release.yml` refuse de partir sans notre secret (ou avec l'id de Polyfrost).
- Constaté : `horizon-prepare` seul (binaire + research-lock.json) échoue (« mods/horizon/src/test/java/dev/horizonlab/
  parity/… introuvable ») ; avec ces sources Java d'Horizon à côté, `--prepare --complete` réussit en 191 s et `--check`
  rend 0. → `build.yml` les embarque (aucun fichier Mojang).
- [x] `pvp_launch` (crate à nous, GPL) : porte propriétaire (`Session::for_account` : compte Microsoft connecté
      seulement), session JSON sur stdin, commandes prepare/check/launch, jar serveur lu dans le JSON de version du
      joueur. 4 tests OK.
- [x] Launcher : bouton « Play PvP Client » sur l'accueil → `pvp_client.rs` (compte par défaut rafraîchi, jar serveur
      téléchargé chez Mojang et vérifié SHA-1, préparation si `--check` échoue, lancement). Compile ; lancé localement.
      Non vérifié : clic → connexion → partie (demande une connexion Microsoft par l'utilisateur).
- [x] `release.yml` (à la main seulement, brouillon non publié) ; `build.yml` réutilisable (`workflow_call`).
- [ ] Téléchargement du client depuis une release (aucune release publiée ; `PVP_CLIENT_BIN` en attendant).
- [ ] Installateurs cargo-packager, mises à jour, profils de modules dans le launcher.
- À noter pour la distribution : OneLauncher embarque aussi des clés Polyfrost (CurseForge, Discord) dans
  `oneclient_common/src/constants.rs`.

### Phase 4 (suite)
Voir docs/ROADMAP.md ; détaillées ici au démarrage de chaque phase.

## Blocages et constats importants

- 2026-11-26 : `26.3.jar` absent de `.minecraft` (seul le JSON) → résolu le jour même (voir État courant).

- 2026-11-25 ~19:45 UTC : organisation `horizon-rust` entièrement en 404 (constaté via API, git, web).
  Remote provisoire : `rdbtCVS/horizon` (indiqué par le Discord d'Horizon, tête `ae632edf48` constatée).
- Licence Horizon : LGPL-3.0-or-later (constaté `Cargo.toml`), pas MIT.
- Client ID Microsoft : nécessite une app Azure approuvée par Mojang (D5) — bloquant phase 4.

## Décisions prises

- (à valider) D1-D8, voir docs/ROADMAP.md.

## Délégations

| Date | Tâche | Modèle | Résultat / corrections |
|---|---|---|---|
| 2026-11-25 | Inventaire Polyfrost lot A (30 dépôts HUD/PvP/visuel) | opencode-go/deepseek-v4-pro (high) | livré (`research/polyfrost/batch-A.md`, 627 l.) ; vérifié par échantillon (PolySprint, PolyCrosshair : conformes) ; aucune correction |
| 2026-11-26 | Spécification visuelle OneConfig (`research/oneconfig/ui-spec.md`) | opencode-go/deepseek-v4.1-flash (high) | en cours (1er essai `opencode/` refusé : solde ; 2e sur v4-pro annulé à la demande de l'utilisateur) |
| 2026-11-25 | Inventaire Polyfrost lot B (45 dépôts chat/Hypixel/perf/divers) | opencode-go/deepseek-v4-pro (high) | livré (`research/polyfrost/batch-B.md`, 749 l.) ; non revérifié ligne à ligne ; statuts fair-play reclassés par moi dans MODULES.md |
