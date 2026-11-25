# PROGRESS

Mémoire de travail du projet. À relire, avec `git log`, au début de chaque session.
Règle : une tâche n'est cochée que si son workflow CI est vert (lien) ; « constaté » = résultat
d'outil observé, « supposé » = non vérifié.

## État courant (2026-11-25)

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
- [ ] Validation utilisateur des documents et décisions D1-D8

### Phase 0 — Dépôt et CI
- [x] Monorepo local (subtrees `engine/`, `launcher/`, remotes `upstream-horizon`, `upstream-onelauncher`) — constaté
- [x] Dépôt GitHub privé, push — constaté (visibility PRIVATE)
- [ ] ci.yml vert (Linux), puis matrice complète sur main
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
- [ ] 1.1 `pvp-client` (exécutable, `main_with`), 1.5 keybinds/Shift droit, 1.6 menu (UI OneConfig), 1.7 éditeur HUD
- Prochaine étape : étudier OneConfig/PolyUI (UI à reproduire), écrire `pvp-client`, e2e horizon-ctl.

### Phase 2 / 3 / 4
Voir docs/ROADMAP.md ; détaillées ici au démarrage de chaque phase.

## Blocages et constats importants

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
| 2026-11-25 | Inventaire Polyfrost lot B (45 dépôts chat/Hypixel/perf/divers) | opencode-go/deepseek-v4-pro (high) | livré (`research/polyfrost/batch-B.md`, 749 l.) ; non revérifié ligne à ligne ; statuts fair-play reclassés par moi dans MODULES.md |
