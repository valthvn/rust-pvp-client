# PROGRESS

Mémoire de travail du projet. À relire, avec `git log`, au début de chaque session.
Règle : une tâche n'est cochée que si son workflow CI est vert (lien) ; « constaté » = résultat
d'outil observé, « supposé » = non vérifié.

## État courant (2026-11-25)

- Étape : **exploration et documents terminés, en attente de validation** (point d'arrêt n° 1).
- Aucun code écrit, aucun dépôt GitHub créé, rien poussé.

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
- [ ] Monorepo local (subtrees `engine/`, `launcher/`, remotes upstream)
- [ ] Dépôt GitHub privé, push
- [ ] ci.yml vert (Linux), puis matrice complète sur main
- [ ] build.yml (artifacts)
- [ ] e2e.yml (selon D4)
- [ ] Benchmark de référence (docs/PERF.md)

### Phase 1 / 2 / 3 / 4
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
