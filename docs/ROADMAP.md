# Roadmap

Plan par phases. Chaque tâche n'est « terminée » dans PROGRESS.md qu'avec un workflow CI vert (lien),
et, pour ce qui se voit en jeu, une capture horizon-ctl inspectée. Architecture et justification des
choix : [ARCHITECTURE.md](ARCHITECTURE.md). Inventaire des mods : [MODULES.md](MODULES.md).

## Phase 0 — Dépôt et CI (avant le code)

1. Monorepo privé : racine à nous, `engine/` = Horizon `ae632edf48` (subtree squash depuis
   `rdbtCVS/horizon`), `launcher/` = OneLauncher (subtree squash), remotes `upstream-horizon` et
   `upstream-onelauncher`. README, LICENSE GPL-3.0, ATTRIBUTION.md, .gitignore, PROGRESS.md.
2. `ci.yml` (fmt, clippy `-D warnings`, tests des crates `pvp-*` et des crates touchées ; Linux
   partout, Windows/macOS sur `main` et PR), `build.yml` (release du client `pvp-client` et du
   launcher, artifacts). Mesurer le temps d'un build froid de `horizon-game` sur un runner hébergé
   standard pour calibrer le cache et la matrice.
3. Vérifier en CI que le snapshot compile tel quel (`cargo build --profile fast -p horizon-game`) et
   que les tests d'Horizon passent avant toute modification : c'est la base de comparaison.
4. `e2e.yml` (si D4 est validé) : Xvfb + Mesa llvmpipe (Vulkan logiciel), préparation 26.3 sur le
   runner, `horizon-ctl launch --offscreen`, captures uploadées en artifacts.
5. Benchmark de référence : `horizon-ctl bench` et `fly` sur le binaire Horizon nu → `docs/PERF.md`.

## Phase 1 — Le socle

| # | Tâche | Critère de fin |
|---|---|---|
| 1.1 | `pvp-client` : exécutable qui reproduit `horizon-official` (sans heartbeat ni serveur officiel) et appelle `main_with` avec un `ModEntry` vide | le jeu démarre, `horizon-ctl state` répond ; bench identique au binaire Horizon (± bruit) |
| 1.2 | `pvp-core` : trait `Module`, schéma `Setting`, registre statique, cycle de vie, `catch_unwind` | tests unitaires (activation, désactivation, panique isolée) |
| 1.3 | Bus d'événements : listes d'abonnés par événement, reconstruites au toggle ; `pvp_runtime` (le `ModEntry` unique) | test : aucun module actif → aucun callback appelé ; bench sans régression |
| 1.4 | Config TOML versionnée, migrations, `.bak`, profils (créer/dupliquer/renommer/supprimer/importer/exporter) | tests : migration v1→v2, valeur hors bornes, fichier corrompu |
| 1.5 | Keybinds : mappings enregistrés (visibles dans Contrôles), état lu via `input_key` ; Shift droit ouvre le menu | e2e : `horizon-ctl key rshift tap` → `screen` = menu |
| 1.6 | `pvp-ui` : menu (recherche, catégories, interrupteurs, panneaux de réglages, profils) | e2e : capture du menu, `ui` liste les widgets, `click` bascule un module |
| 1.7 | Éditeur HUD : glisser-déposer, ancrage, magnétisme, échelle, réinitialisation | tests (calcul d'ancrage/magnétisme) + e2e capture |
| 1.8 | Module témoin (FPS) traversant toute la chaîne | e2e : activé → visible ; désactivé → capture identique au binaire nu |
| 1.9 | Rapport de phase + liste des choix de style | — |

## Phase 2 — Modules PvP essentiels

Ordre indicatif ; une fois l'API figée (fin 1.4), les modules indépendants peuvent être développés en
parallèle par des sous-agents, chacun avec une consigne autonome.

| Module | Inspiré de | Branchement principal |
|---|---|---|
| Toggle sprint / sneak (+ HUD d'état) | PolySprint | `set_mapping_down`, `client_tick_pre` |
| Crosshair | PolyCrosshair, CrosshairV2 | `gui_layer_pre` (annule `minecraft:crosshair`) + dessin |
| Keystrokes + CPS | Keystrokes (revamp) | `input_key`, HUD |
| HUD : FPS, ping, coordonnées, direction, armure, effets, horloge | EvergreenHUD, VanillaHUD | HUD (`item_stack` pour l'armure) |
| Hitbox | PolyHitbox | `render_level_stage` |
| Nametags | PolyNametag | `render_name_tag` (+ hook de style si nécessaire) |
| Glint | GlintColorizer | `Look.frame` |
| Heure / météo côté client | PolyTime, PolyWeather | `Look.time_of_day`, `Look.weather` |
| Damage tint | DamageTint | `Look.frame` (hurt overlay) |
| Flou de menu | PolyBlur / Blur | flou de fond des écrans (champ `Look` à ajouter si besoin) |
| Zoom (si le temps le permet) | PolyZoom | `compute_fov_modifier` |

Pour chaque module : code, tests unitaires, e2e horizon-ctl (activer, capture, état), bench
tout-activé / tout-désactivé, un commit par unité logique.

## Phase 3 — Chat, Hypixel, particules, animations

- Chat (Chatting) : horodatage, compactage, recherche, copie, onglets/filtres, captures de chat.
- Confort Hypixel (Hytils Reborn) : tout, y compris auto-GG, auto-queue, auto-GL (désactivés par défaut, D6).
- Particules (OverflowParticles) : multiplicateurs, couleurs, masquage — en 26.3.
- Animations (OverflowAnimations) : adaptations 26.3 seulement (Horizon a déjà `legacy_pose` et le
  « combat feel ») ; les animations 1.7 pures sont « non applicable ».

## Phase 4 — Launcher

- Nouvelle famille d'instance « client PvP natif » dans OneLauncher : télécharge la release de
  `pvp-client` + `horizon-prepare` (release GitHub, sommes de contrôle signées), prépare les
  ressources depuis l'installation Minecraft du joueur (`horizon-prepare --prepare --check
  --progress json`), lance via la Launch API (`--resources`, `--game-dir`, `--home`,
  `--session-stdin`).
- Compte Microsoft : réutiliser `oneclient_auth`, avec **notre propre client ID Azure approuvé par
  Mojang** (décision D5 ; délai de plusieurs semaines).
- Mises à jour du client et du launcher, gestion des profils de modules (même format TOML).
- `release.yml` : installateurs via cargo-packager, sur le modèle de `oneclient_release.yml`.

## Décisions (validées le 25/11/2026 : D1-D4, D6 ; D5 à la charge de l'utilisateur ; D7-D8 ouvertes)

| # | Décision | Recommandation | Bloque |
|---|---|---|---|
| D1 | **Modules sur l'API de mods d'Horizon** (`mod_api`, liée statiquement via `main_with`) plutôt que sur de nouveaux hooks moteur | **Oui** : surface versionnée, maintenue par l'amont, presque aucune modification du moteur. Risque : coût de l'instrumentation du mod host → mesuré en 1.1/1.3 ; repli sur les registres de la plateforme si le coût est mesurable | Phase 1 |
| D2 | Organisation : racine à nous + subtrees `engine/` et `launcher/`, synchro par **merge** (pas rebase) | **Oui** (raisons dans ARCHITECTURE.md § 6). C'est un écart à la consigne « rebaser » : le merge est plus sûr à ce rythme amont | Phase 0 |
| D3 | Page « PvP & HUD » native d'Horizon : la garder, la masquer, ou piloter ses fonctions depuis notre menu | **La garder telle quelle en phase 1-2** (désactivée par défaut, aucun coût) ; nos modules sont indépendants. Ajouter plus tard un hook « masquer la page » si la double UI gêne | Non |
| D4 | CI e2e et harnais de parité : télécharger le jar 26.3 depuis `piston-meta.mojang.com` sur le runner (jamais en cache ni artifact) | **Oui** pour le job e2e et les harnais touchés par nos hooks, comme l'autorise la consigne, en le documentant ; Horizon lui-même s'y refuse par principe (barrière propriétaire) | Phase 0 (e2e) |
| D5 | Client ID Microsoft : enregistrer une app Azure et demander l'approbation Mojang | **À lancer dès maintenant** par toi (compte personnel requis, délai de semaines) | Phase 4 |
| D6 | Fonctions Hypixel qui envoient des messages (auto-GG, auto-requeue, auto-tip) | **Validé 25/11 : incluses**, désactivées par défaut | Phase 3 |
| D7 | Nom du produit (actuellement « client PvP », crates `pvp-*`) | À choisir ; sans impact technique avant la phase 4 | Non |
| D8 | Brand envoyé au serveur | `vanilla`-compatible : garder celui d'Horizon ou un brand propre clairement assumé | Phase 2 |

## Risques

- **Amont indisponible** : si `horizon-rust/horizon` ne revient pas, le projet repose sur un snapshot
  figé et un miroir communautaire. Pas de bloquant immédiat ; la vélocité amont devient nulle.
- **Coût de build en CI** : `horizon-client` est un très gros crate ; l'amont utilise des runners
  16 vCPU. Sur runners standard, un build froid peut dépasser 30-60 min (à mesurer en phase 0).
- **Juridique** : la cause de la disparition de l'amont est inconnue. Le dépôt, d'abord privé, est public depuis le 26/11/2026 à la demande de l'utilisateur.
