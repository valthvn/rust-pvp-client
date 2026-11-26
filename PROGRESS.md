# PROGRESS

Mémoire de travail du projet. À relire, avec `git log`, au début de chaque session.
Règle : une tâche n'est cochée que si son workflow CI est vert (lien) ; « constaté » = résultat
d'outil observé, « supposé » = non vérifié.

## État courant (2026-11-26)

- **Bloquant (action utilisateur) : GitHub Actions ne démarre plus aucun job** — annotation constatée sur le
  run 45764065757 (branche `phase1`) : « recent account payments have failed or your spending limit needs to be
  increased ». À régler dans Billing & plans du compte. En attendant : travail et tests en local, CI à rejouer.
- Travail en cours sur la branche `phase1` (main inchangé depuis 923c072).

- Documents validés le 25/11 (D1-D4, D6 ; fair-play : toutes les fonctionnalités Polyfrost autorisées).
- Nouvelle consigne : **reprendre à l'identique l'UI de OneClient/OneConfig** (menu, HUD, launcher).
- Dépôt privé : https://github.com/valthvn/rust-pvp-client (subtrees `engine/` = Horizon ae632edf48,
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
- [x] Dépôt GitHub privé, push — constaté (visibility PRIVATE)
- [ ] ci.yml vert (Linux), puis matrice complète sur main
      - run 45702723724 (923c072) : engine macOS ✅, Windows ✅ ; engine Linux ❌ tué (exit 137, mémoire) pendant
        la compilation des tests ; launcher ❌ : `oneclient_db` (tests) utilise des `sqlx::query!` absents du cache
        hors ligne `.sqlx`. Correctif db48831 (branche phase1) : swap 12 Go + tests sans debuginfo, 3 jobs ;
        clippy amont `continue-on-error` sans `--all-targets`, tests launcher sans `oneclient_db`.
        Non vérifié : la CI ne démarre plus (facturation, voir État courant).
      - run 45695282544 : engine macOS ✅ (clippy horizon-game 2 min 23 s), engine Windows ✅,
        engine Linux : en cours à l'arrêt de session ; launcher ❌ sur les 3 OS (OneLauncher amont non
        conforme à rustfmt stable / clippy -D warnings) → ci.yml corrigé : vérification stricte seulement
        sur nos crates du launcher, clippy amont en rapport seulement. À revérifier.
- [ ] build.yml (artifacts) — run 45695282620 : les builds macOS client et launcher compilent, mais
      **l'upload échoue : quota de stockage d'artifacts du compte GitHub atteint** (partagé entre dépôts
      privés). Rétention réduite à 3 jours. Action utilisateur : libérer des artifacts ou augmenter le quota.
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
- [~] 1.5 Shift droit (`pvp_ui:menu`, `key.keyboard.right.shift`) → `mod_screen_open`. Écrit, non testé en jeu.
- [~] 1.6 menu : shell OneConfig (fenêtre 1391×700 dp, ombre, barre latérale 264 dp, en-tête, recherche,
      ouverture 600 ms ease-out-expo), page Mods (puces de catégorie, grille 4×255×140), page module (cartes
      d'option et contrôle par type de réglage, popup select, sélecteur de couleur), Profils, Raccourcis.
      Tests unitaires : 17 OK (pvp-ui). Non testé en jeu. Écarts connus : bordure en dégradé des cartes rendue
      unie, pas de tête de joueur (pas d'API client du nom/skin), pas de transition de page (300 ms).
- [~] 1.8 module témoin FPS (`pvp-modules`) : écrit, test unitaire OK.
- [ ] 1.7 éditeur HUD ; e2e horizon-ctl — **bloqué : ressources du jeu non préparables** (voir Blocages).

### Phase 2 / 3 / 4
Voir docs/ROADMAP.md ; détaillées ici au démarrage de chaque phase.

## Blocages et constats importants

- 2026-11-26 : `horizon-prepare --prepare` refuse : `%APPDATA%/.minecraft/versions/26.3/26.3.jar` absent (seul le
  JSON existe ; index d'assets 34 et 40 bibliothèques manquent aussi). Action utilisateur : lancer Minecraft 26.3
  une fois depuis le launcher officiel. Horizon ne télécharge pas le jeu lui-même (choix voulu).

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
