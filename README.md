# Client PvP natif (nom de travail)

Client Minecraft: Java Edition 26.3 orienté PvP, **entièrement natif en Rust, sans JVM au runtime**.

- **Moteur** : fork mince d'[Horizon](https://github.com/horizon-rust/horizon) (`engine/`).
- **Modules** : les fonctionnalités des mods [Polyfrost](https://github.com/Polyfrost) (toggle sprint,
  crosshair, keystrokes, HUD, nametags, hitbox…) réimplémentées comme modules compilés dans le binaire,
  configurables depuis un menu in-game unique (Shift droit).
- **Launcher** : dérivé de [OneLauncher / OneClient](https://github.com/Polyfrost/OneLauncher) (`launcher/`).

État : en développement, dépôt public depuis le 26/11/2026. Voir [PROGRESS.md](PROGRESS.md).

## Documentation

- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) — structure d'Horizon, points d'extension, organisation du dépôt
- [docs/MODULES.md](docs/MODULES.md) — inventaire des mods Polyfrost et statut de chaque fonctionnalité
- [docs/ROADMAP.md](docs/ROADMAP.md) — phases et décisions

## Organisation

```
engine/     Horizon (git subtree) + nos crates engine/crates/pvp-*
launcher/   OneLauncher (git subtree)
docs/       documentation du projet
research/   notes d'inventaire (mods Polyfrost)
```

Synchronisation avec l'amont (merge, pas rebase ; voir ARCHITECTURE.md § 6) :

```sh
git fetch upstream-horizon main && git subtree pull --prefix=engine upstream-horizon main --squash
git fetch upstream-onelauncher main && git subtree pull --prefix=launcher upstream-onelauncher main --squash
```

## Licence

GPL-3.0 pour l'ensemble ; voir [LICENSE](LICENSE) et [ATTRIBUTION.md](ATTRIBUTION.md).
Minecraft est une marque de Mojang/Microsoft ; aucun fichier du jeu n'est inclus.
