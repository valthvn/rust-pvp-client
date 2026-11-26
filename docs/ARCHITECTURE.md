# Architecture

Client PvP natif en Rust : un fork mince d'**Horizon** (Minecraft: Java Edition 26.3 réécrit en
Rust) dans lequel les fonctionnalités des mods **Polyfrost** deviennent des modules compilés dans le
binaire, distribué par un launcher dérivé de **OneLauncher / OneClient**.

Ce document décrit ce qui a été **constaté dans le code** (snapshot Horizon `ae632edf48`, 24/11/2026 ;
OneLauncher `main` du 25/11/2026), où se branche chaque type d'extension, et l'organisation du dépôt.
Les points non vérifiés sont signalés comme tels.

---

## 1. État des sources (constaté le 25/11/2026)

### 1.1 Horizon : amont indisponible, miroir provisoire

- Depuis le 25/11/2026 vers 19:45 UTC, **tous les dépôts de l'organisation `horizon-rust` renvoient
  404** (API, `git ls-remote`, page web), alors que la liste de l'organisation les montre publics et
  que playhorizon.dev pointe toujours dessus. Aucune notice DMCA publique ne les vise à cette date
  (les notices Mojang du 15 au 23/11 visent des dépôts Eaglercraft). Cause inconnue.
- Dernier `main` amont récupérable : **`ae632edf48f6cae3ba52a5c81523ec712488c9cc`** (24/11 17:30 UTC),
  tête commune de la plupart des forks ; aucun fork ne contient de commit amont plus récent.
- Sur indication du Discord d'Horizon, le miroir de référence en attendant le retour du dépôt est
  **`rdbtCVS/horizon`** (même tête `ae632edf48`). Il sert de remote `upstream-horizon` provisoire.
- **Licence** : `LGPL-3.0-or-later` pour le travail original (`Cargo.toml` : `license`), le code
  fusionné avant le changement reste aussi disponible sous MIT (`LICENSES/MIT.txt`). Le prompt initial
  indiquait MIT : c'est faux pour le code actuel. LGPL-3.0+ est compatible avec un ensemble GPL-3.0.
- Vélocité : ~32 000 commits, de l'ordre de 1 000 commits/semaine sur `main` (écart constaté entre
  forks). Cela conditionne la stratégie de synchronisation (§ 6).

### 1.2 Points incertains du contexte, vérifiés

| Point | Constat | Source |
|---|---|---|
| Windows / macOS | Supportés : « Windows on x64 or ARM64, macOS 12 or later, Linux on x86-64 ». `native-client.md` dit la même chose (« desktop Linux, macOS and Windows ») ; je n'ai pas trouvé de contradiction dans ce snapshot. Les harnais sont audités sur Linux et macOS (`docs/status.md`), pas sur Windows. | `README.md:48`, `docs/native-client.md:483`, `docs/status.md` |
| Multijoueur online-mode | **Implémenté** : handshake chiffré, `joinServer` sur le session service, signature du chat (parties a, c, d du plan), harnais `online_mode`. La connexion Microsoft est dans le launcher Horizon (device-code). La session passe au jeu par `--session-file` (fichier 0600 détruit à la lecture) ou `--session-stdin`, jamais en argv. | `docs/microsoft-account.md`, `docs/api/launch.md` |
| Client ID Microsoft | Une app Azure doit être **approuvée par Mojang** (formulaire App ID review, « des jours à des semaines »). OneLauncher utilise le client ID approuvé de Polyfrost (`oneclient_common::constants::MICROSOFT_CLIENT_ID`) : un fork ne peut pas légitimement le réutiliser. **Bloquant pour la phase 4.** | `docs/microsoft-account.md` §1-2, `oneclient_auth/src/msa.rs` |
| API de mods | Elle existe et est riche : mods de contenu (`.hmod`), **mods natifs Rust** compilés par le launcher, **mods WebAssembly** sandboxés, ~795 hooks versionnés (`crates/horizon-mod-vocab/src/hooks/`), dont environ 90 côté client (HUD, écrans, nametags, entrée, chat, paquets, FOV, caméra). Le « système Rust → WebAssembly » d'un fork tiers est en fait upstream (`horizon-mod-wasm`). | `docs/mods.md`, `docs/native-mods.md`, `docs/modding/` |
| Fichiers du jeu | Horizon **ne télécharge jamais** les fichiers Mojang : `horizon-prepare` construit `.local/horizon-client/26.3` à partir de l'installation du joueur, en lançant des exporteurs Java (JVM nécessaire **à la préparation**, pas au runtime). Les pins sont dans `research-lock.json`. La « Launch API » impose une barrière propriétaire : ne lancer/préparer que pour un compte qui possède le jeu. | `docs/launcher-preparation.md`, `docs/api/launch.md` |
| horizon-ctl | Confirmé : CLI + serveur MCP (`launch`, `state`, `ui`, `click`, `key`, `screenshot`, `bench SECONDS [raw]`, `fly X1 Y1 Z1 X2 Y2 Z2 SECONDS [PITCH [raw]]`…), plus `horizon-client --benchmark standard|travel|ab:SWITCH`. | `docs/horizon-ctl-commands.md`, `docs/benchmark.md` |
| PvP existant | **Horizon a déjà une page « PvP & HUD »** (tout désactivé par défaut) : crosshair, contour de bloc, couleur de coup, widgets HUD (armure, effets, ping, coordonnées, FPS, CPS, keystrokes, horloge, combo, vitesse, reach affichée), glint, enchantements en chiffres, heure et météo côté client, chat (horodatage, compact, copie, recherche), presets. | `docs/pvp.md`, `crates/horizon-client-render/src/pvp.rs` (1 835 lignes) |

### 1.3 OneLauncher / OneClient

- Monorepo **100 % Rust**, `GPL-3.0-only`, édition 2024, UI Freya, SQLx, workspace `packages/*` :
  `oneclient_app` (UI), `oneclient_auth` (MSA), `oneclient_core` (clusters, `game/launch.rs`),
  `oneclient_java`, `oneclient_mc`, `oneclient_content` (Modrinth/CurseForge), `oneclient_db`…
- Le lancement (`oneclient_core::game::launch::launch_cluster_with`) est entièrement pensé pour une
  **JVM** (arguments JVM, Java store, Fabric). Lancer Horizon demande une nouvelle « famille »
  d'instance qui suit la Launch API d'Horizon au lieu du chemin Java (phase 4).
- Workflows de release existants : `oneclient_release.yml`, `oneclient_nightly.yml` (cargo-packager),
  plus AUR/COPR/Flatpak.

---

## 2. Comment Horizon est structuré

Workspace Cargo à la racine (le launcher Horizon `launcher/` et le serveur de télémétrie sont des
workspaces séparés). Rust stable 1.97+, `RUSTC_BOOTSTRAP=1` + `-Zthreads=8` via `.cargo/config.toml`,
`[patch.crates-io]` sur `wgpu-hal`, `naga`, `libmimalloc-sys` (`third_party/`).

### 2.1 Trois couches (`docs/layers.md`, vérifiées par `tools/check_layers.py`)

| Couche | Rôle | Crates principales |
|---|---|---|
| **Engine** | Le jeu vanilla | `horizon-client` (gameplay, `game.rs` + `game/`), `horizon-client-lib`, `horizon-client-render` (wgpu, HUD, écrans, `pvp.rs`, `extras.rs`), `horizon-client-input`, `horizon-client-core`, `horizon-world`, `horizon-core`, `horizon-protocol` (protocole 777), `horizon-items`, `horizon-command`, `horizon-hooks` (la couture) |
| **Platform** | API publique pour ce qui est au-dessus | `horizon-platform` (ré-exporte `horizon-hooks` sans le dispatch), `horizon-game` (`horizon_game::main()`), `horizon-mod-host`, `horizon-mod-api` / `horizon-mod-vocab` (API de mods) |
| **Horizon** | Ce qui n'est pas vanilla | `horizon-official` (l'exécutable livré sous le nom `horizon-client`), `horizon-discord`, `horizon-friends`, `horizon-cosmetics`, `horizon-name-tag`, `horizon-heartbeat`… |

Règle : la couche Horizon n'utilise **jamais** une crate engine. L'amont est en train (étapes M5/M6)
de sortir ses « extras » (y compris la page PvP & HUD) du moteur vers des crates de la couche Horizon
branchées par la plateforme. **Notre client s'inscrit exactement dans cette trajectoire.**

### 2.2 Sous-systèmes (fichiers lus)

| Sous-système | Où |
|---|---|
| Boucle de jeu, tick, frame | `crates/horizon-client/src/game.rs` (`fn run`, `fn tick`, `fn tick_world`, `fn render`), `game/runtime/window_handler.rs` (`ApplicationHandler`), `clock.rs` |
| Rendu | `horizon-client-render/src/gfx.rs` (`Renderer::new`, `Renderer::render`), `gfx/`, shaders WGSL, `hud.rs`, `hud_layout.rs`, `hud_kit/` |
| UI / écrans | enum `Screen` dans `game.rs`, `options_ui.rs`, `settings_shell.rs`, `extras_ui.rs`, `ui.rs` (widgets), `screen_draw.rs` |
| Entrée | `horizon-client-input` (`keybinds.rs`, `keymapping.rs`, `input.rs`), `game/interface/play_keys.rs`, `play_input.rs` |
| Réseau client | `net.rs` (thread réseau), `game/network/remote/apply_packet.rs`, `horizon-protocol` |
| Réglages | `options.txt` reste strictement vanilla ; Horizon écrit `horizon-extras.txt`, `horizon-graphics.txt` ; chaque crate du dessus a son propre `horizon-<nom>.txt` via le *settings store* |
| Pilotage agent | `control.rs`, `bin/horizon-ctl/`, `game/runtime/control.rs` |

### 2.3 Les deux surfaces d'extension existantes

**a) Registres de la plateforme** (`horizon-hooks`, ré-exportés par `horizon-platform`) : enregistrés
avant le démarrage, scellés au démarrage, **coût nul quand rien n'est enregistré** (une lecture de
slice vide, aucune allocation).

| Registre | Contenu | Usage pour nous |
|---|---|---|
| `hooks::Hooks` | `started`, `shutdown`, `frame`, `tick(PlayerModels)`, `name_decoration`, `player_mark`, `camera`, `player_layers`, `own_name_tag`… | cycle de vie, nametag perso |
| `settings::Store` | lignes `clé:valeur` dans un fichier du dossier de jeu | **non retenu** pour la config des modules (pas de profils ni de versionnage), voir § 4.3 |
| `pages::Page` | pages de l'écran d'options (Toggle, Cycle, Slider, Button, Key, Field…) | entrée « PvP Client… » dans Options / Pause |
| `hud::Layer` | `Canvas` (text, fill, place dans un coin) | trop limité (pas d'items, pas de position libre) |
| `actions::register` | key mappings `(nom, touche par défaut, catégorie)` visibles dans Contrôles | raccourcis des modules |
| `players::Look` | « boutons » de rendu poussés (pose legacy, indicateur d'attaque, couleurs contour/hurt/glint, taille bouclier/totem, feu, eau/lave claire, chat, icônes d'effets, heure, météo) | glint, damage tint, heure, météo, feu bas… |

**b) API de mods** (`horizon-mod-api`, lib `mod_api`, générée depuis `horizon-mod-vocab`) : hooks
versionnés avec contrats `listen` / `cancel` / `replacement` / `collection`, priorité, `ClientHost`
typé. Côté client, constaté dans `docs/modding/src/how-to/client/` :

- dessin : `gui_render_post` / `render_gui_*` / `gui_layer_draw` avec un `DrawContext`
  (`fill`, `text`, `sprite`, `item_stack`, push/pop de pose) ; `screen_render_*` ;
- entrée : `input_key`, `key_mapping_pre`, `movement_input`, `player_input` ; `ClientHost::set_mapping_down`
  / `click_mapping` (passent par le chemin vanilla des touches) ; mappings propres aux mods ;
- nametags : `render_name_tag` (`visible`, `text`) ; FOV : `compute_fov_modifier` ; `camera_pose` ;
- chat : `client_chat_received` (annulable, texte mutable), `client_chat`, `customize_chat_overlay` ;
- réseau : `client_packet` (lecture), `client_packet_send` (annulable) ;
- ticks : `client_tick_pre/post` ; écrans : `screen_opening`, `screen_closing`, `screen_*_pre`.

**Point clé, vérifié** : `horizon_mod_host::main_with(mods: Vec<ModEntry>, …)` accepte des mods
**fournis statiquement par l'exécutable**. L'amont s'en sert lui-même
(`crates/horizon-mod-host/examples/native_gui_overlay_probe.rs`,
`crates/horizon-hot-paths-game/src/main.rs`), et `horizon_game::main()` n'est rien d'autre que
`main_with(Vec::new(), &[], &[], startup)`. Un exécutable peut donc lier des modules Rust qui
implémentent `mod_api::Hooks`, **sans aucun build par le launcher et sans modifier le moteur**.

---

## 3. Architecture retenue (proposée)

```
                ┌───────────────────────── pvp-client (exécutable) ─────────────────────────┐
                │ horizon_official-like register() : pages, key mappings, Look, Hooks        │
                │ horizon_mod_host::main_with(vec![pvp_runtime::ENTRY], …)                   │
                └──────────────┬───────────────────────────────┬────────────────────────────┘
                               │                               │
          ┌────────────────────▼──────────┐       ┌────────────▼─────────────┐
          │ pvp-modules                    │       │ pvp-ui                   │
          │ un module par fonctionnalité   │       │ menu Shift droit,        │
          │ (sprint, crosshair, keystrokes,│       │ recherche, catégories,   │
          │  HUD, nametags, glint…)        │       │ éditeur HUD, profils     │
          └────────────────────┬──────────┘       └────────────┬─────────────┘
                               │        dépend de              │
                       ┌───────▼───────────────────────────────▼───────┐
                       │ pvp-core : trait Module, registre, cycle de   │
                       │ vie, bus d'événements, config versionnée,     │
                       │ keybinds, HUD (positions, ancrage, magnétisme)│
                       └───────────────────────┬───────────────────────┘
                                               │ n'utilise que
                       ┌───────────────────────▼───────────────────────┐
                       │ API publique d'Horizon : mod_api (ClientHost, │
                       │ DrawContext, hooks client) + horizon-platform │
                       └───────────────────────────────────────────────┘
```

- **Aucune crate pvp-\* ne dépend d'une crate engine** (même règle que la couche Horizon, vérifiable
  avec `tools/check_layers.py` en déclarant nos crates comme couche Horizon).
- Un **seul `ModEntry`** (`pvp_runtime`) est passé à `main_with` ; il souscrit aux hooks dont au moins
  un module a besoin et redistribue aux modules **activés**. Chaque hook commence par une lecture
  atomique de la liste des abonnés actifs : si elle est vide, retour immédiat (pas d'allocation, pas
  de travail). C'est ce qui garantit « module désactivé = aucun effet observable ».
- `pvp-client` remplace `horizon-official` comme exécutable : il réutilise sélectivement les parties
  Horizon utiles (nametag perso, Discord Rich Presence) et **n'inclut pas** le ping heartbeat ni le
  serveur officiel dans la liste (choix produit, réversible).

### 3.1 Où brancher chaque type de hook

| Besoin | Point d'extension | Modification d'Horizon ? |
|---|---|---|
| Tick client | `mod_api` `client_tick_pre/post` | non |
| Rendu HUD (2D) | `mod_api` `gui_render_post` + `DrawContext` (texte, rectangles, sprites, `item_stack`) | non |
| Masquer le crosshair vanilla / élément HUD vanilla | `mod_api` `gui_layer_pre` : **annulable couche par couche** (`minecraft:crosshair`, `hotbar`, `effects`, `boss_overlay`…) — vérifié dans `horizon-mod-vocab/src/hooks/gui_layer_pre.rs` | non |
| Rendu monde (hitbox, contour) | `Look.frame` couvre contour/hurt/glint ; hitboxes : `render_level_stage` (`WorldDrawContext`, lignes et quads, étape `AfterLevel`) — vérifié dans `hooks/render_level_stage.rs` | non (style exact F3+B à confirmer) |
| Entrée / keybinds | `horizon_platform::actions::register` (apparaît dans Contrôles) + `mod_api` `input_key` | non |
| Ouvrir notre menu (Shift droit) | écrans de mod : `screen_opening` peut répondre `ScreenOpening::ModScreen`, l'API fournit boutons et champs pour écrans de mod (`hooks/notes.rs`, `client/mod.rs`) ; ouverture directe sur une touche : appel exact à confirmer au début de la phase 1 | probablement non |
| Chat | `client_chat_received` (annulable, texte mutable), `client_chat` | non |
| Paquets | `client_packet` (lecture), `client_packet_send` (annulable) | non |
| Nametags | `render_name_tag` (visibilité, texte) ; style (fond, ombre) : non exposé | **probable** (`Look` ou hook) |
| FOV / zoom | `compute_fov_modifier` | non |
| Heure / météo côté client | `Look.time_of_day`, `Look.weather` | non |
| Flou de menu | `screen_backgrounds.rs` a déjà le flou de menu vanilla ; intensité : à vérifier | probable (1 champ `Look`) |
| Réglages par page dans Options | `pages::register` (place `Pause` / `HorizonSettings`) | non |

Règle de conduite pour toute modification du moteur : **un champ ou une méthode avec un défaut
vanilla**, nommé par ce qu'il change à l'image ou au son, jamais par la fonctionnalité qui l'utilise
(convention d'Horizon pour `Look`), sur une branche `hooks/<nom>` propre, proposable telle quelle en PR
upstream.

### 3.2 Parité vanilla

- Tous les modules sont **désactivés par défaut** ; un client fraîchement installé est vanilla.
- Le jeu de base reste celui d'Horizon : les harnais de parité ne voient que le moteur. Nos
  modifications du moteur se limitent aux hooks ci-dessus (défaut vanilla) ; la CI lance les
  harnais sélectionnés par `tools/parity.py --changed-files` sur ces fichiers.
- Au niveau protocole, le client se comporte comme Horizon (donc comme vanilla), plus un *brand*
  assumé ; aucun plugin channel n'est ajouté en phases 1-2.

### 3.3 Fair-play

Décision du 25/11/2026 : **toutes les fonctionnalités des mods Polyfrost sont autorisées**, y compris
fullbright, hitboxes avec couleur d'i-frames, brouillard d'eau réduit, affichage de la reach, compteur
d'entités et automatismes Hypixel (auto-GG, auto-queue). Chacune reste désactivée par défaut.
Restent hors projet, comme le prévoyait la consigne initiale, les fonctionnalités de triche qui ne
viennent d'aucun mod Polyfrost : modification de la reach, aide à la visée, autoclick (même si
`ClientHost::click_mapping` le permettrait) et ESP à travers les murs.

---

## 4. API de modules (proposition à valider)

### 4.1 Le trait

```rust
// pvp-core
pub trait Module: Sync + 'static {
    /// Identifiant stable : clé de config, id horizon-ctl, jamais traduit. Ex. "toggle_sprint".
    fn id(&self) -> &'static str;
    /// Clé de traduction du nom affiché, et de sa description.
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn category(&self) -> Category;            // Hud, Pvp, Chat, Visual, Performance
    /// Schéma déclaratif des réglages : id, type, défaut, bornes. Sert au menu, à la (dé)sérialisation
    /// et aux migrations.
    fn settings(&self) -> &'static [Setting];
    /// Événements auxquels le module veut être abonné quand il est actif.
    fn events(&self) -> Events;                // bitset : TICK | HUD | KEY | CHAT | PACKET_IN | …
    /// Raccourcis déclarés statiquement (enregistrés dans Contrôles avant le démarrage).
    fn keybinds(&self) -> &'static [Keybind] { &[] }
    /// Élément(s) HUD déplaçables.
    fn hud(&self) -> Option<&dyn HudElement> { None }

    fn on_enable(&self, _cx: &mut Cx) {}
    fn on_disable(&self, _cx: &mut Cx) {}
    fn on_setting(&self, _id: &str, _cx: &mut Cx) {}
    // Une méthode par événement, avec un défaut vide, comme `mod_api::Hooks` :
    fn tick(&self, _cx: &Cx, _host: &mut dyn ClientHost) {}
    fn key(&self, _cx: &Cx, _host: &mut dyn ClientHost, _ev: &InputKeyEvent<'_>) {}
    fn gui_layer(&self, _cx: &Cx, _host: &dyn ClientHost, _ev: &mut GuiLayerPreEvent<'_>) -> Flow { Flow::Continue }
    // … name_tag, level_stage, fov ; implémenté dans engine/crates/pvp-core/src/module.rs
}

pub enum Setting { Toggle{..}, Slider{min, max, step, ..}, Choice{..}, Color{..}, Key{..}, Text{..} }
pub enum Event<'a> { Tick, Hud(&'a mut HudCx<'a>), Key(KeyEvent), ChatIn(&'a mut ChatIn<'a>),
                     PacketIn(&'a PacketView<'a>), ScreenOpen(..), NameTag(&'a mut NameTag<'a>), … }
```

Choix :

- **Modules = `static`** (`pub static TOGGLE_SPRINT: ToggleSprint = …;`), état interne dans des
  atomiques ou un `Mutex` local au module, comme le demande Horizon pour ses registres (l'état ne vit
  jamais dans une struct du moteur). Pas de `Box<dyn>` créé à la volée, pas de framework de plugins.
- **Bus d'événements** : un tableau de listes par type d'événement, reconstruit seulement quand un
  module est activé/désactivé (jamais par frame). Le dispatch d'un événement sans abonné coûte une
  lecture.
- **Valeurs des réglages** lues par les modules via des accesseurs typés générés depuis le schéma
  (`self.cfg().get::<f32>("gap")` remplacé par des champs atomiques mis à jour à chaque changement :
  aucune recherche par chaîne dans un chemin par frame).
- **`Cx`** donne accès à : `ClientHost` (lecture d'état du jeu), la config du module, l'envoi de
  notifications (toasts), et `Look::update` pour les boutons de rendu.

### 4.2 Registre et cycle de vie

1. `pvp-client::main` construit le registre statique (`pvp_modules::ALL: &[&dyn Module]`).
2. Avant le démarrage : enregistrement des key mappings, de la page d'options, du `ModEntry` unique.
3. `started` : chargement de la config (profil actif), activation des modules marqués actifs
   (`on_enable`), calcul des listes d'abonnés.
4. En jeu : bascule depuis le menu → `on_disable`/`on_enable`, mise à jour des listes, sauvegarde
   différée (debounce 500 ms, hors thread de rendu).
5. `shutdown` : sauvegarde synchrone.

Un module qui panique est désactivé et signalé (toast), sans faire tomber le jeu (`catch_unwind`
autour des callbacks ; à mesurer : coût nul sur le chemin sans panique).

### 4.3 Configuration

- Fichier `pvp-client/profiles/<nom>.toml` dans le dossier de jeu, plus `pvp-client/state.toml`
  (profil actif, version). `options.txt` et les fichiers d'Horizon ne sont jamais touchés.
- En-tête `schema = N`. Migrations = liste ordonnée de fonctions `fn(&mut toml::Table)` appliquées de
  la version lue à la version courante ; sauvegarde `.bak` avant toute migration. Valeur inconnue ou
  hors bornes → défaut du schéma + avertissement, jamais d'échec de chargement.
- Profils : créer, dupliquer, renommer, supprimer, importer/exporter (fichier TOML) ; le launcher
  pourra les gérer en phase 4 (même format).

### 4.4 Éléments HUD

`HudElement` : `size(&self, cx) -> (w, h)`, `draw(&self, dc, cx)`, `preview()` pour l'éditeur.
Position stockée en **ancre + décalage normalisé** (coin/centre le plus proche + offset en pixels GUI),
pour rester correcte quand la taille de fenêtre ou l'échelle GUI change. Éditeur : glisser-déposer,
magnétisme sur les bords d'écran, le centre et les autres éléments (seuil 4 px GUI), échelle par
élément, grille optionnelle.

---

## 5. Menu in-game (pvp-ui)

**Décision du 25/11/2026 : reprendre à l'identique l'UI de OneClient** (menu OneConfig, HUD et éditeur de HUD,
launcher OneClient), au lieu d'un design original. La consigne de style initiale (« sans copier l'identité
visuelle ») est remplacée par celle-ci. `pvp-ui` reproduira la mise en page, les couleurs, la typographie et les
interactions de OneConfig/PolyUI après étude de leur code (licences des assets à vérifier).


Ouverture par **Shift droit** (mapping enregistré, re-bindable) ; recherche instantanée sur nom,
description et réglages ; catégories HUD, PvP, Chat, Visuel, Performance ; carte par module avec
interrupteur et panneau de réglages ; éditeur HUD ; gestion des profils. Il est dessiné avec le
`DrawContext` de l'API de mods (mêmes primitives que le HUD) sur un écran de mod. Les choix de style
seront listés après la première version, conformément à la consigne.

Le menu ne remplace pas la page « PvP & HUD » d'Horizon, qui reste dans Options → Horizon Settings.
Voir la décision ouverte D3 dans ROADMAP.md.

---

## 6. Organisation du dépôt et synchronisation amont

```
/ (dépôt public, GPL-3.0)
├── README.md  LICENSE (GPL-3.0)  ATTRIBUTION.md  PROGRESS.md  .gitignore
├── docs/                 ARCHITECTURE.md, MODULES.md, ROADMAP.md, PERF.md
├── .github/workflows/    ci.yml, build.yml, e2e.yml, (release.yml)
├── engine/               Horizon (git subtree, LGPL-3.0+ / MIT)
│   └── crates/pvp-core, pvp-modules, pvp-ui, pvp-client   ← notre code, membres du workspace Horizon
└── launcher/             OneLauncher (git subtree, GPL-3.0-only) + nos changements
```

Pourquoi ce choix :

- **Racine à nous** : README, LICENSE GPL-3.0, ATTRIBUTION, docs et workflows sans conflit avec
  l'amont. Sous Windows, `docs/ARCHITECTURE.md` entrerait en collision avec `docs/architecture.md`
  d'Horizon (système de fichiers insensible à la casse) si Horizon était à la racine. Les ~30
  workflows d'Horizon (runners Blacksmith et self-hosted) restent inertes dans `engine/.github/`.
- **Nos crates dans le workspace d'Horizon** (`engine/crates/pvp-*`) : elles héritent du
  `Cargo.lock`, des `[patch.crates-io]` et de `.cargo/config.toml` d'Horizon, et tous ses outils
  (`pr_ready.py`, `parity.py`, `check_layers.py`) fonctionnent tels quels depuis `engine/`. Ce sont
  des fichiers nouveaux : ils ne créent jamais de conflit. Le seul fichier amont touché par
  l'intégration est la liste `members` de `engine/Cargo.toml` (1 ligne).
- **Subtree plutôt que sous-modules** : un seul dépôt privé, un seul checkout en CI (des sous-modules
  privés exigeraient un jeton supplémentaire), une seule PR pour un changement qui touche moteur et
  launcher.
- **Synchronisation par merge (`git subtree pull --squash`), pas par rebase.** À ~1 000 commits par
  semaine, rejouer nos commits sur l'amont à chaque synchro serait coûteux et un rebase ne
  traverse pas proprement les merges de subtree. Avec le merge, les conflits se limitent aux
  quelques fichiers de hooks. Les remotes `upstream-horizon` (aujourd'hui `rdbtCVS/horizon`, puis
  `horizon-rust/horizon` à son retour) et `upstream-onelauncher` (`Polyfrost/OneLauncher`) sont
  conservés.
- **Hooks proposables upstream** : chaque modification du moteur est un commit isolé sur
  `engine/`, rejoué sur un clone de l'amont (`git format-patch --relative=engine`) pour en faire une
  PR. Inventaire tenu dans PROGRESS.md.

`.gitignore` : `target/`, `.local/`, toute ressource préparée (`**/horizon-client/26.*`, `prepared/`),
fichiers Mojang (`*.jar` hors outils), dossiers de jeu de test.

---

## 7. CI (GitHub Actions)

- Runners GitHub hébergés (l'amont utilise Blacksmith 16 vCPU et des runners self-hosted, non
  disponibles ici) ; cache `Swatinem/rust-cache` sur `engine/` et `launcher/`.
- `ci.yml` : `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, tests.
  Pour `engine/`, on vise d'abord les crates `pvp-*` et les crates touchées par nos hooks ; la suite
  complète d'Horizon est très lourde (l'amont la répartit sur plusieurs runners de 16 vCPU). Linux
  sur toutes les branches ; Windows et macOS sur `main` et les PR.
- **Fichiers Minecraft en CI** : les harnais de parité et le test e2e en jeu ont besoin des fichiers
  26.3 préparés. Horizon, lui, ne les télécharge jamais (barrière propriétaire). La consigne du projet
  autorise de les obtenir depuis les sources officielles de Mojang (`piston-meta.mojang.com`) pendant
  le job, sans les mettre en cache ni en artifact. Voir la décision ouverte D4.

---

## 8. Limites d'Horizon constatées (pistes d'issues upstream)

1. `hud::Layer` de la plateforme ne permet pas de dessiner d'items ni de positionner librement (seuls
   les coins) ; l'API de mods le permet. Converger vers un seul chemin de dessin HUD.
2. Le style des nametags (fond, ombre, opacité) n'est pas exposé (`render_name_tag` ne règle que
   visibilité et texte).
3. Les hitboxes F3+B ne sont pas paramétrables (couleur, épaisseur) : un mod doit les redessiner via
   `render_level_stage` et ne peut pas masquer celles du moteur. À confirmer en phase 2.
4. Les réglages PvP d'Horizon (`pvp.rs`) vivent dans le moteur sans API de lecture/écriture depuis
   la couche du dessus : un client tiers ne peut ni les piloter ni les masquer (étape M5 en cours
   chez l'amont).
5. Documentation et code supposent un dépôt `horizon-rust/horizon` joignable (liens, SDK des mods
   natifs épinglé par URL git) : avec l'amont indisponible, la construction de mods natifs par le
   launcher Horizon est cassée pour tout le monde. Notre approche (modules liés statiquement) n'en
   dépend pas.
