# Attribution

Ce dépôt assemble plusieurs projets libres. Chacun garde sa licence ; l'ensemble distribué est sous
**GPL-3.0** (voir `LICENSE`).

| Composant | Emplacement | Origine | Licence |
|---|---|---|---|
| Horizon (moteur de jeu) | `engine/` | https://github.com/horizon-rust/horizon (miroir provisoire : https://github.com/rdbtCVS/horizon), snapshot `ae632edf48` | LGPL-3.0-or-later ; code antérieur au changement de licence aussi sous MIT (`engine/LICENSE`, `engine/LICENSES/`, `engine/THIRD_PARTY_NOTICES.md`) |
| OneLauncher / OneClient (launcher) | `launcher/` | https://github.com/Polyfrost/OneLauncher | GPL-3.0-only (`launcher/LICENSE`, `launcher/ATTRIBUTION.md`) |
| Notre code (modules, menu, client) | `engine/crates/pvp-*`, racine | ce dépôt | GPL-3.0 |

## Mods Polyfrost

Les fonctionnalités des mods Polyfrost (https://github.com/Polyfrost) sont **réimplémentées** en Rust
à partir de leur comportement observable et de leur documentation : aucun code Java/Kotlin n'est copié.
Les mods dont s'inspire chaque module sont listés dans `docs/MODULES.md`, avec leur licence.
Merci à Polyfrost et à leurs contributeurs.

## Minecraft

Minecraft est une marque de Mojang AB / Microsoft. Ce dépôt ne contient aucun code, asset ou donnée
de Minecraft ; le jeu se prépare à partir de l'installation du joueur (`horizon-prepare`).
Projet non officiel, non approuvé par Mojang ni Microsoft.
