# Inventaire des mods Polyfrost

Source : les 140 dépôts de l'organisation GitHub `Polyfrost` (liste du 25/11/2026). 75 dépôts ont été
lus en détail : configuration, options et valeurs par défaut, cibles, mixins. Les fiches complètes,
avec chemins de fichiers et numéros de ligne, sont dans `research/polyfrost/batch-A.md` et
`batch-B.md`. Ces fiches ont été produites par délégation (OpenCode, deepseek-v4-pro), puis vérifiées
par échantillonnage (licence AGPL et cibles 1.8.9→26.3 de PolySprint, options et défauts de
PolyCrosshair : conformes). Les autres valeurs n'ont pas été revérifiées une par une : elles le seront
au moment de porter chaque module.

**Constat général.** La plupart des mods Polyfrost actifs ciblent désormais **1.8.9 (Ornithe) et
1.21.x → 26.3 (Fabric)** via Stonecutter, sur OneConfig 1.8-1.9. Leur comportement en 26.3 sert donc de
spécification directe pour notre version 26.3. Les mods archivés ou anciens sont limités à 1.8.9 Forge.

## Légende

- **Statut**
  - **portable** : se transpose en 26.3 tel quel.
  - **adaptation** : à revoir pour 26.3, pour Horizon ou pour le fair-play.
  - **non applicable** : n'a pas de sens en 26.3 ou dans un moteur Rust.
  - **exclu** : retiré du périmètre (aucun à ce jour : décision du 25/11, toutes les fonctionnalités Polyfrost sont autorisées).
  - **hors périmètre** : pas un mod jouable.
- **Branchement** : point d'extension Horizon prévu (voir ARCHITECTURE.md § 3.1). `mod_api` désigne
  l'API de mods, `Look` les boutons de rendu de la plateforme. « ⚠ hook » signale qu'une petite
  modification du moteur est probablement nécessaire.
- **Horizon** : ce qu'Horizon offre déjà nativement (page PvP & HUD, `docs/pvp.md`).
- **Priorité** : P1 (phase 2), P2 (phase 3), P3 (plus tard), — (aucune).
- **Licences** : on porte des fonctionnalités, jamais du code. Cette règle est impérative pour les
  licences AGPL (PolySprint, AutoText), CC-BY-NC-SA (OverlayTweaks, ChatTweaks, PolyPatcher,
  ConfirmDisconnect, WindowedFullscreen), PolyForm Shield (Polydium) et pour les dépôts sans licence.

## 1. PvP, HUD et visuel (lot A)

| Mod | Cible (branche lue) | Licence | Fonctionnalités et options clés | Statut | Branchement | Horizon | Prio |
|---|---|---|---|---|---|---|---|
| PolySprint | 1.8.9 + 1.21.1→26.3 (`main`) | AGPL-3.0 | toggle sprint (on), toggle sneak (off), désactiver le sprint au W-tap, touches séparées, texte d'état (aussi en vol), sneak relâché au décollage, fly boost (1-10, solo uniquement) | **portable** (fly boost : solo uniquement, conservé) | `set_mapping_down`, `client_tick_pre`, HUD | toggle sprint/sneak vanilla (options) | P1 |
| PolyCrosshair | 1.8.9 + 1.21.1→26.3 (`main`) | GPL-3.0 | éditeur pixel (grille 7/15/31/63), profils Normal/Ami/Ennemi/Élytre, rotation, contour, fusion vanilla, couleur selon la vie, bow/attack dynamiques, échelle, 3e personne, spectateur, crosshair de debug, indicateur d'attaque | **portable** ; profil Ami/Ennemi → **adaptation** (couleur « sur entité vivante » seulement, comme Horizon) | `gui_layer_pre` (annule `minecraft:crosshair`) + `DrawContext` | crosshair à formes (croix, T, X, cercle, carré, point), couleur sur entité | P1 |
| CrosshairV2 | 1.8.9 Forge (archivé) | LGPL-3.0 | ancêtre de PolyCrosshair : formes, couleurs par type de cible, espacement dynamique | **non applicable** (remplacé par PolyCrosshair) | — | — | — |
| PolyHitbox | 1.8.9 + 1.21.1→26.3 | GPL-3.0 | hitboxes par catégorie (joueur, soi, mob, monstre, projectiles, cadres, items, cristal, XP) : contour, faces, hauteur des yeux, rayon de visée, pointillés, épaisseur selon la distance, toggle F3+B, couleur au survol, **couleur pendant les i-frames** | **portable** : toutes les options, y compris la couleur pendant les i-frames (décision du 25/11) | `render_level_stage` (`WorldDrawContext`) | — (hitboxes F3+B vanilla) | P1 |
| PolyNametag | 1.8.9 + 1.21.1→26.3 | GPL-3.0 | suppression, décalage en hauteur (−10..10), échelle, ombre, son propre nametag (aussi en inventaire), masquage en F1 par type, fond (couleur `0x3F000000`), coins arrondis, marges, couleur du texte | **portable** | `render_name_tag` (visibilité, texte) + ⚠ hook de style (fond, ombre, échelle) | son propre nametag (`horizon-name-tag`) | P1 |
| Keystrokes-Reborn | 1.8.9 Forge (archivé) | GPL-3.0 | keystrokes + trace de souris, éditeur glisser-déposer | **portable** (fonctionnalité reprise dans notre module Keystrokes) | `input_key`, HUD | keystrokes fixes (`hudKeys`) | P1 |
| Canelex-KeyStrokes-Revamp | 1.8.9 Forge (archivé) | aucune | keystrokes WASD, souris, espace, fondu, style | **portable** (fusionné dans Keystrokes) ; code sans licence : référence de comportement uniquement | idem | idem | P1 |
| EvergreenHUD | 1.8.9 + 1.21.1→26.3 (`oneconfig`) | GPL-3.0 | style global (police, échelle, fond, rayon, ombre, marges) + ~50 éléments : armure, batterie, biome, horloges, combo, CPS, texte/image perso, direction, FPS, heure de jeu, inventaire, compteurs d'items, mémoire, ping, position, effets, **reach**, **compteur d'entités**, saturation, TPS, météo réelle, média… | **portable** pour le cœur (FPS, ping, coords, direction, armure, effets, CPS, horloge, combo, mémoire, saturation, texte perso) ; **reach** et **compteur d'entités** inclus ; média/batterie/météo réelle : P3 | HUD `mod_api` (`item_stack`, texte, sprites), `client_packet` (ping) | FPS, ping, coords, armure, effets, CPS, horloge, combo, vitesse, reach | P1 (cœur) |
| VanillaHUD | 1.8.9 + 1.21.1→26.3 | GPL-3.0 | chaque élément du HUD vanilla déplaçable, redimensionnable et stylable (hotbar, barres, boss, scoreboard, titres, action bar…) | **adaptation** : faisable couche par couche via `gui_layer_pre` (annuler) + redessin, mais redessiner fidèlement le HUD vanilla est coûteux | `gui_layer_pre` + redessin ; ⚠ hook de décalage de couche plus simple | — | P2 |
| PolyBlur | 1.8.9 + 1.21.1→26.3 | GPL-3.0 | flou de mouvement (Phosphor, Unity, Hybrid), tampon de vitesse par pixel | **adaptation** : passe de post-traitement dans le renderer wgpu ; doit respecter `docs/hdr-passes.md` | ⚠ hook de passe post-process | — | P2 |
| Blur | 1.8.9 Ornithe (fork MidnightDust) | MIT | flou d'arrière-plan des menus : intensité, dégradé, animation, réglage par écran | **portable** (= « flou de menu » de la phase 2) | Horizon a déjà le flou de menu vanilla (`screen_backgrounds.rs`) ; ⚠ champ `Look` (intensité, durée) | flou de menu vanilla | P1 |
| PolyTime | 1.8.9 + 1.21.1→26.3 | GPL-3.0 | heure côté client, phase de lune | **portable** | `Look.time_of_day` (phase de lune : à vérifier) | `viewTime` | P1 |
| PolyWeather | 1.8.9 + 1.21.1→26.3 | GPL-3.0 | météo côté client (clair, pluie, orage, neige) + intensité, météo réelle | **portable** (météo réelle : P3, appel réseau externe) | `Look.weather` | `viewWeather` | P1 |
| GlintColorizer | 1.8.9 seulement (`main`) | GPL-3.0 | couleur, vitesse et rotation du glint par contexte (main, armure, item au sol, inventaire) | **adaptation** : glint 26.3 différent (shader) ; couleur et vitesse | `Look.frame` (glint) ; vitesse : ⚠ champ `Look` | `glintColour` | P1 |
| DamageTint | 1.8.9 + 1.21.1→26.3 | LGPL-3.0 | couleur du flash de dégâts, fondu, par type de dégâts | **portable** | `Look.frame` (hurt overlay) ; par type : ⚠ | `hit` (couleur de coup) | P1 |
| BlockOverlay | 1.8.9 Forge (archivé) | GPL-3.0 | contour et remplissage du bloc visé | **portable** | `Look.frame` (contour) | `outline` | P2 |
| ColorSaturation | 1.8.9 + 1.21.1→26.3 | GPL-3.0 | shader saturation, contraste, luminosité, teinte | **adaptation** (passe post-process) | ⚠ hook post-process | — | P3 |
| PolyFullBright | 1.8.9 + 1.21.1→26.3 (`master`) | GPL-3.0 | luminosité maximale, curseur de niveau | **portable** (autorisé, décision du 25/11) | ⚠ `Look` (lumière minimale) ou fullbright d'Horizon | fullbright (extra Horizon) | P2 |
| MountOpacity | 1.8.9 / 1.12.2 | LGPL-3.0 | monture translucide | **portable** | ⚠ hook d'opacité d'entité | — | P3 |
| NumericalEnchantments | 1.8.9 / 1.12.2 | GPL-3.0 | niveaux d'enchantement en chiffres | **portable** | tooltips `mod_api` | `numericEnchants` | P3 |
| PolyZoom | 1.8.9 Ornithe | GPL-3.0 | zoom à 2 niveaux, transitions, molette, sensibilité relative, caméra cinématique | **portable** | `compute_fov_modifier`, `input_key` | zoom (extra Horizon) | P2 |
| PolyZoomOld | 1.8.9 Forge (archivé) | GPL-3.0 | zoom simple | **non applicable** (remplacé par PolyZoom) | — | — | — |
| BehindYouV3 | 1.8.9 + 1.21.1→26.3 (`3.x`) | GPL-3.0 | touche « vue arrière/avant » animée, FOV et distance | **portable** | `camera_pose`, `compute_fov_modifier` | — | P3 |
| View-Model | 26.2 Fabric (fork) | MIT | position, rotation et échelle des mains en 1re personne | **adaptation** | ⚠ hook de transformation de la main (`arm_pose_transform` à évaluer) | taille bouclier/totem | P3 |
| betterhurtcam | 1.8.9 Ornithe (fork) | MIT | intensité du hurt cam, clignotement de la barre de vie | **portable** | `camera_pose` (à vérifier) ou ⚠ `Look` | — | P2 |
| ShakeTweaks | 1.8.9 / 1.12.2 | GPL-3.0 | supprime le balancement de la vue en gardant celui de la main | **adaptation** | ⚠ `Look` | — | P3 |
| FOVTweaks | archivé (→ FOVChanger) | GPL-3.0 | modificateurs de FOV par état (sprint, vol, arc, vitesse, lenteur, eau) | **portable** | `compute_fov_modifier` | — | P3 |
| OverlayTweaks | 1.8.9 / 1.12.2 | CC-BY-NC-SA-4.0 | brouillard d'eau, feu, citrouille, vignette, portail | **portable** : toutes les options, brouillard d'eau réduit compris | `Look` (feu existant), ⚠ autres | feu bas, eau claire (désactivé par défaut) | P3 |
| RenderTweaks | 1.8.9 / 1.12.2 | LGPL-3.0 | masquer la foudre et la ligne de pêche | **portable** | ⚠ `Look` | — | P3 |
| DroppedItemTweaks | 1.8.9 / 1.12.2 (non revérifié) | LGPL-3.0 | items au sol sans rebond ni rotation, plafond du nombre rendu | **portable** | ⚠ `Look` | — | P3 |

## 2. Chat, Hypixel, animations, performance et divers (lot B)

| Mod | Cible | Licence | Fonctionnalités clés | Statut | Branchement | Horizon | Prio |
|---|---|---|---|---|---|---|---|
| Chatting | 1.8.9 + 1.21.1→26.3 | GPL-3.0 | têtes dans le chat, onglets, animations, captures du chat, raccourcis, horodatage, recherche… | **portable** (raccourcis : seulement sur action du joueur) | `client_chat_received`, `customize_chat_overlay`, ⚠ rendu des lignes | horodatage, compact, copie, recherche | P2 |
| ChatTweaks | 1.8.9 + 1.21.x/26.x | CC-BY-NC-SA-4.0 | historique, compactage, horodatage, clics sûrs, aperçus d'images | **portable** (aperçu d'images : télécharge des URL → désactivé par défaut) | idem | partiel | P2 |
| AutoText | 1.8.9 + 1.21.4→26.x | AGPL-3.0 | macros : une touche envoie un message ou une commande | **adaptation** : un envoi par appui explicite (comme une touche de chat) ; aucune répétition automatique | `input_key` + envoi de chat | — | P3 |
| Hytils-Reborn | 1.21.4→26.1 (**archivé**) | GPL-3.0 | Hypixel : filtres et restylage du chat, tab (rangs, guilde, ping), masquage du HUD, overlays (hauteur, UHC, coffres, sumo), **auto-GG (activé par défaut), auto-queue, auto-GL, auto-ami** | **adaptation** : affichage et filtres oui ; envois automatiques (auto-GG, auto-queue, auto-GL, auto-ami) **inclus**, désactivés par défaut (D6) ; surbrillance des coffres incluse | `client_chat_received`, `client_packet`, HUD | — | P2 |
| Tabulous | 1.8.9 Forge | aucune | tab : position, couleurs, texte, têtes | **portable** (référence de comportement uniquement, pas de licence) | ⚠ hook tab list | — | P3 |
| ColoredBedsDataModV1 | 26.3 Fabric | Unlicense | outil de dev qui extrait les couleurs de lit de Bed Wars | **hors périmètre** (outil de données) | — | — | — |
| OverflowAnimationsV2 | 1.8.9 + 1.21.4→26.1.2 | GPL-3.0 (+ exception de lien) | animations et rendu 1.7/1.8 (épée bloquante, pose, items, armure…) | **adaptation** pour ce qu'Horizon n'a pas (Horizon : `legacy_pose`, combat feel) ; animations purement 1.7 **non applicable** | `Look.legacy_pose`, ⚠ hooks d'animation | pose legacy, combat feel | P2 |
| OverflowAnimations | 1.8.9 Forge (archivé) | LGPL-3.0 | ancêtre de V2 | **non applicable** (remplacé par V2) | — | — | — |
| OverflowParticles | 1.8.9 + 1.21.1→26.x | GPL-3.0 | particules : multiplicateurs, couleurs, masquage, vue dégagée, « bouclier », particules de coup | **portable** | ⚠ hook particules (`particles.rs`) | particules d'attaque masquables | P2 |
| PolyPatcher | 1.8.9 / 1.12.2 Forge | CC-BY-NC-SA-4.0 | correctifs 1.8.9/1.12.2, performance, confort (289 mixins) | **non applicable** pour les correctifs et l'optimisation propres à 1.8.9 ; quelques options de confort sont déjà couvertes ailleurs (ConfirmDisconnect, BetterScreens…) | — | — | — |
| REDACTION | 1.8.9 + 1.21.x/26.x | GPL-3.0 | cosmétiques minimalistes : bandes noires, FOV de la main, neige dans l'inventaire | **portable** | ⚠ `Look` | — | P3 |
| Spice | 1.8.9 Fabric | GPL-3.0 | optimisations LWJGL3 pour 1.8.9 | **non applicable** (propre à 1.8.9 et LWJGL) | — | — | — |
| Polydium | 1.21.1→26.3 | PolyForm Shield | optimisations modernes (662 mixins, compat Sodium/Iris) | **non applicable** : optimise le moteur Java ; Horizon a son propre moteur | — | — | — |
| Polydium-Legacy | 1.8.9 | PolyForm Shield | optimisations 1.8.9 | **non applicable** | — | — | — |
| Argentum | 1.8.9 (fork) | aucune | moteur de rendu type Embeddium | **non applicable** | — | — | — |
| Sarcio | 1.8.9 (fork) | LGPL-3.0 | mémoire et correctifs 1.8.9 | **non applicable** | — | — | — |
| Dynamic-FPS | 1.8.9 (fork) | MIT | FPS et volume réduits quand la fenêtre est inactive | **adaptation** : Horizon a déjà `inactivityFpsLimit` vanilla ; ajouter le volume | options vanilla, ⚠ volume | limiteur d'inactivité vanilla | P3 |
| FPSLimiter | multi (Patcher) | GPL-3.0 | limite les FPS et le son sans focus | **non applicable** (vanilla 26.3 le fait) | — | oui | — |
| PolyReflex | 26.2+ Vulkan | GPL-3.0 | NVIDIA Reflex / AMD Anti-Lag 2 | **adaptation** : réduction de latence dans le renderer wgpu (Performance) ; légitime | ⚠ renderer | à vérifier | P3 |
| PolyDebug | outil Java + probe 26.2+ | GPL-3.0 | trouve le mod qui coûte des FPS | **hors périmètre** (`horizon-perf` et `bench` couvrent le besoin) | — | `horizon-perf` | — |
| CrashPatch | 1.8.9 + 1.21.x/26.x | GPL-3.0 | survit aux crashs, téléverse les logs | **non applicable** tel quel (pas de JVM) ; écran de crash et rapport existent dans Horizon | — | rapports de crash | — |
| ConfirmDisconnect | multi (Patcher) | CC-BY-NC-SA-4.0 | confirmation avant de quitter un serveur | **portable** | `screen_opening` (pause → déconnexion) | — | P3 |
| ScrollTweaks | multi (Patcher) | LGPL-3.0 | réglages de la molette sur la hotbar | **adaptation** (Horizon a des réglages de molette) | — | réglages molette | — |
| SoundTweaks | 1.8.9 + 1.21.x/26.x | GPL-3.0 | volume par son, plafond levé | **adaptation** | ⚠ hook son | canaux de sons UI | P3 |
| SmartSearch | multi | GPL-3.0 | recherche sémantique des réglages (modèle d'embeddings) | **adaptation** : notre menu a une recherche instantanée textuelle ; pas de modèle ML | pvp-ui | — | — |
| PackManager | — | — | dépôt vide (LICENSE seulement) | **hors périmètre** | — | — | — |
| PolyWaypoints | — | — | dépôt vide | **hors périmètre** | — | — | — |
| PolySnapper | — | — | dépôt vide | **hors périmètre** | — | — | — |
| Snapper | multi Fabric | MPL-2.0 | gestionnaire et visionneuse de captures, envoi | **adaptation** (envoi : service externe à choisir) | `screenshot` | captures vanilla | P3 |
| WindowedFullscreen | multi (Patcher) | CC-BY-NC-SA-4.0 | plein écran fenêtré sans bordure | **non applicable** (natif dans Horizon et winit) | — | oui | — |
| BetterScreens | multi (Patcher) | GPL-3.0 | réglages des écrans et inventaires | **adaptation** (la position d'inventaire, propre à 1.8, est non applicable) | `screen_*` | — | P3 |
| Legacy-Starlight | 1.8.9 | GPL-3.0 | moteur de lumière Starlight | **non applicable** | — | — | — |
| LegacySkinLayers | 1.8.9 | GPL-3.0 | couches de skin en 3D | **adaptation** (cosmétique 26.3, rendu de modèle) | `player_layers` (plateforme) | — | P3 |
| LegacyWaveyCapes | 1.8.9 | GPL-3.0 | capes ondulantes | **adaptation** | `player_layers` / modèles | — | P3 |
| LegacyTweaks | multi (Patcher) | GPL-3.0 | petits réglages visuels 1.8.9 (main gauche…) | **non applicable** pour l'essentiel (natif en 26.3) | — | — | — |
| Legacy-Crafty-Crashes | 1.8.9 Forge | aucune | meilleurs rapports de crash 1.8.9 | **non applicable** | — | — | — |
| Tooltip-Scroll-Fabric | ≥26.1 (fork) | MIT | tooltips défilants | **portable** | `screen_tooltip` | — | P3 |
| MediaSession | bibliothèque Kotlin | Apache-2.0 | média en cours de lecture et paroles synchronisées (utilisée par PolyPlus) | **hors périmètre** (bibliothèque) ; l'élément HUD média d'EvergreenHUD est en P3 | — | — | — |
| PolyPlus | 1.8.9 + 1.21.1→26.3 | GPL-3.0 | menu principal OneClient, lobby, groupes et voix, cosmétiques, pings | **hors périmètre** : services Polyfrost (backend propriétaire) | — | cosmétiques Horizon | — |
| quick-pack | multi | GPL-3.0 | chargement de packs plus rapide (mipmaps en parallèle) | **non applicable** (chargement de packs propre à Horizon) | — | — | — |
| packed_packs | 1.8.9 (fork) | MIT | profils de resource packs | **adaptation** | écran de packs | — | P3 |
| flashback-legacy | 1.8.9 | GPL-3.0 | replay Flashback | **non applicable** (Horizon a son propre système de replay) | — | replay | — |
| ForgeCompat | 1.8.9 | GPL-3.0 | fait tourner des mods Forge sur Ornithe | **non applicable** | — | — | — |
| VintageFix | 1.12.2 Forge (archivé) | LGPL-3.0 | correctifs mémoire 1.12.2 | **non applicable** | — | — | — |
| OneClient (mod) | 1.8.9 + 1.21.1→26.3 | GPL-3.0 | menu principal à la marque OneClient | **hors périmètre** (marque Polyfrost) ; notre écran titre sera traité en phase 4 | — | — | — |

## 3. Dépôts écartés sans fiche (pas des mods)

Bibliothèques, outils et infrastructure : OneConfig, OneConfig-Bootstrap, OneConfigLoader,
OneConfigWrapper, OneConfigMigrator, OneConfigExampleMod, OneConfig-Documentation, PolyCompose,
OmniCore, UniversalCraft, Elementa, keventbus-forge, PolyMixin, PolyIO, JTokens, Crashylizer,
PolyBot, PolySigner, polyfrost-gradle-toolkit, architectury-loom, blossom, BuildFormat, pylon,
lwjgl3-repacked, isolated-lwjgl3-loader, arm-macos-lwjgl2, java-25-mojave, interlaced, freetype, skia,
skiko, freya, skinview3d, packwiz, sodium, spark, SkyblockAPI, ursa-minor, backend,
interfrost-workers, plus-backend, plus-website, plus-admin-dashboard, plusrenderer,
polyplus-nameplate-blockbench, polyfrost-crowdin-relay, PolyHelper, PolyHelperV2, DataStorage,
DataStorageV2, CrashData, Knowledgebase, Legal, Branding, infra, website, Server-API, defrost,
Deleter, Easeify, ExamplePolyUIMod, PolyPack, ColoredBedsDataMod, OneLauncher (le launcher : voir
ARCHITECTURE.md), `.github`.

## 4. Synthèse pour la phase 2

| Module (phase 2) | Mods sources | Modification d'Horizon nécessaire ? |
|---|---|---|
| Toggle sprint / sneak | PolySprint | non |
| Crosshair | PolyCrosshair | non |
| Keystrokes + CPS | Keystrokes-Reborn, Canelex, EvergreenHUD (CPS) | non |
| HUD : FPS, ping, coordonnées, direction, armure, effets, horloge, combo | EvergreenHUD | non |
| Hitbox | PolyHitbox | non (redessin) ; style F3+B natif : ⚠ |
| Nametags | PolyNametag | **oui** pour le fond, l'ombre et l'échelle (hook de style) |
| Glint | GlintColorizer | couleur : non (`Look`) ; vitesse : ⚠ |
| Heure / météo | PolyTime, PolyWeather | non (`Look`) |
| Damage tint | DamageTint | non (`Look`) ; par type de dégâts : ⚠ |
| Flou de menu | Blur | ⚠ champ `Look` (intensité) |

Bilan : 6 modules sur 10 sans aucune modification du moteur. Les 4 autres demandent un petit hook,
générique et proposable en amont (style des nametags, intensité du flou, vitesse du glint, teinte par
type de dégâts).
