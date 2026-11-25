# Batch A — Polyfrost Mod Research (read-only inventory)

Research notes for reimplementing Polyfrost open-source Minecraft mods in another engine. Each section describes what a mod actually does, from its source code (config classes, feature classes, mixins), not just its README. All repos were cloned `--depth 1` from the **default branch**; a newer/alternate branch is noted where relevant.

Version terminology: "Ornithe" = the 1.8.9 toolchain (OrnitheMC/ploceus). "OneConfig v1" = the old `cc.polyfrost.oneconfig` API (0.x). "OneConfig v2" = the new `org.polyfrost.oneconfig.api.config.v1` API (1.x).

---

## PolySprint

- **URL:** https://github.com/Polyfrost/PolySprint — **archived:** no — **is-fork:** yes — **branch read:** `main` (also has `twoconfig`) — **last commit:** 2026-11-20
- **Targets / loader:** Stonecutter multi-version: MC 1.8.9 (Ornithe) + 1.21.1, 1.21.4, 1.21.5, 1.21.8, 1.21.10, 1.21.11, 26.1, 26.2, 26.3 (Fabric).
- **Licence:** AGPL-3.0
- **Dependencies:** OneConfig **1.8.0** (v2 API), Fabric Loader 0.19.3, PolyUI 2.1.7, Fabric API per-version.
- **Features** (config `src/main/kotlin/org/polyfrost/polysprint/client/PolySprintConfig.kt`, `object PolySprintConfig : Config(..., Category.COMBAT)`):
  - `isEnabled` — Switch — default `true` (L39)
  - `toggleSprint` "Enable Toggle Sprint" — Switch — `true` (L45)
  - `toggleSneak` "Enable Toggle Sneak" — Switch — `false` (L51)
  - `disableWTapSprint` "Disable W-Tap Sprint" — Switch — `false` (L54)
  - `toggleSprintState`, `toggleSneakState` — internal `@Include` booleans, `false` (L58/L62)
  - `keybindToggleSprint` "Separate Keybind for Toggle Sprint" — Switch — `false` (L69)
  - `keybindToggleSprintKey` — Keybind — unbound by default (L85)
  - `keybindToggleSneak` "Separate Keybind for Toggle Sneak" — Switch — `false` (L92)
  - `keybindToggleSneakKey` — Keybind — unbound (L106)
  - `showSprintTextWhileFlying` — Switch — `true` (L115)
  - `unsneakOnFlightStart` — Switch — `true` (L121)
  - `toggleFlyBoost` "Fly Boost" — Switch — `false` (singleplayer only) (L128)
  - `flyBoostAmount` "Fly Boost Amount" — Slider min 1 max 10 step 1 — `4.0` (L137)
- **Behaviour notes:** Implements toggle sprint/sneak by manipulating the vanilla key state (a `StickyKeyBindingSetter` mixin interface holds a "toggle" latch). Syncs its toggle options with vanilla `options.toggleSprint()` / `toggleCrouch()` on 1.9+. Fly boost modifies flight speed while a key is held. W-tap sprint disable blocks the vanilla "start sprint on tap" logic.
- **Mixins (targets):** `Mixin_ToggleSprint` → `KeyboardInput`; `Mixin_StickyKeyBindingSetter` → `ToggleKeyMapping` (+`SharedConstants`); `Mixin_SprintKeyDown` → `LocalPlayer`; `Mixin_SetFlyBoost` → `LocalPlayer`; `Mixin_RestoreToggleKeyStates` → `KeyMapping`; `Mixin_Options` → `Options`; `Mixin_DisableWTap` → `LocalPlayer`; event mixins `Mixin_SprintEvent`→`KeyboardInput`, `Mixin_SprintEvent_2EletricBoogaloo`→`LivingEntity`, `Mixin_FlyEvent`→`ClientPacketListener`, `Mixin_FlyEvent_2EletricBoogaloo`→`GameType`, `Mixin_FlyEvent_NumeroTres`→`LocalPlayer`, `Mixin_RideEvent`→`Entity`.
- **Fair-play:** No reach/aim/ESP/auto-action. Toggle sprint/sneak is a convenience (mirrors vanilla options). Fly boost is explicitly "singleplayer only" (L126).
- **Summary:** A toggle-sprint/toggle-sneak + fly-boost keybind mod.

---

## PolyCrosshair

- **URL:** https://github.com/Polyfrost/PolyCrosshair — **archived:** no — **is-fork:** no — **branch read:** `main` (also `twoconfig`, `port/1.21`) — **last commit:** 2026-11-24
- **Targets / loader:** Stonecutter: MC 1.8.9 (Ornithe) + 1.21.1…26.3 (Fabric). OneConfig **1.8.0**.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig 1.8.0, Fabric Loader 0.19.3.
- **Features** (config `org/polyfrost/crosshair/config/ModConfig.kt` + `CrosshairProfile.kt` + `VanillaCrosshair.kt` + `DebugCrosshair.kt` + `AttackIndicator.kt`):
  - `enabled` — Switch — default **false** (L25).
  - Per-target crosshair **profiles** (`Normal`, `Friendly`, `Enemy`, and `Elytra` indicator on 1.21+). Each profile has (`CrosshairProfile.kt`):
    - `enabled` "Use X Crosshair" — Switch — Normal `true`, others `false` (L42–48)
    - `image` — pixel-editor canvas (base64 PNG grid), `gridSize` dropdown (0.5x/1x/2x/3x, 7/15/31/63 cells, default 1x=15) (L61, L119)
    - `circles` — vector circles/lines layer (hidden) (L68)
    - `rotation` — Slider 0–360° — `0` (L70)
    - `offsetX`/`offsetY` (indicator only) — Slider −32..32 — `0`/`11` (L78/L86)
    - `orbit` "Follow Pitch" — group toggle — `true`, `orbitRadius` 0–32 — `11` (L94–109)
    - `outline` group — off by default; `outlineThickness` 0–1 — `0.5`; `outlineColor` black (L136–157)
    - `blending` "Vanilla Blending" — on by default — `true`; `blendingColor` white (L159–173)
    - `health` "Health Color Shift" — off; `healthThreshold` 1–20 — `6`; `healthColor` red (L175–196)
    - `bow` "Dynamic Bow" — off; `bowScale` 1–10 — `3` (L198–213)
    - `attack` "Dynamic Attack" — off; `attackScale` 1–10 — `3` (1.21+ only) (L216–232)
    - `scale` "Use Custom Scale" — off; `customScale` 0.5–6 — `2`; `smoothScale` — `false` (L234–256)
    - `showInThirdPerson` — `false` (1.21+) / `true` (1.8.9) (L258)
    - `visibleInSpectator` — `false` (L270)
  - **Vanilla crosshair** (`VanillaCrosshair.kt`): `enabled` "Customize Vanilla Crosshair" — `false`; `fixCentering` — `true`; `blending` on — `true`; `blendingColor`/`color` white; `scale` custom — off; `customScale` 0.5–6 — `2`; `showInThirdPerson` — `false`; `visibleInSpectator` — `false`.
  - **Debug crosshair** (`DebugCrosshair.kt`): `enabled` "Customize Debug Crosshair" — `false`; `displayMode` (Never/Overlay/Always) — Overlay; `hideCrosshair` — `true`; `perspective` "3D Perspective" — `true`; `axisWidth` 1–10 — `2`; `outlineWidth` 1–20 — `4`; per-axis (X/Y/Z) length 1–50 — `10`, outline — `true`, colors red/green/blue.
  - **Attack indicator** (1.21+, `AttackIndicator.kt`): `enabled` — `false`; `blending` on; `color` white; `chargeColors` off (charging red, ready green); `offsetX`/`offsetY` −64..64 — `0`; `scale` custom off (`2`); `attack` cooldown toggles for Swords/Axes/Tools/Tridents+Maces+Spears/Other — all `true`; `attackAlwaysReady` — `false`; `sleep`, `blockBreaking`, `itemCooldown`, `bowCharge`, `shieldCooldown`, `consumables`, `storage` — all `false`.
- **Behaviour notes:** Replaces the vanilla crosshair with a per-pixel / per-circle custom renderer. A `TargetClassifier` (based on the entity under the crosshair) picks Normal/Friendly/Enemy profile, so the crosshair recolours depending on whether you're looking at a friendly/enemy target. Vanilla crosshair can be independently recoloured/re-centred. The debug F3 axis cross and the 1.21 attack indicator can be customized. Import/export of share codes (including Lunar `LCCH-` codes) supported.
- **Mixins (targets):** `CrosshairMixin` → `GameGui` (hud crosshair render); `DebugCrosshairMixin` → `GameRenderer`; `EntityCrosshairMixin` → compat target `tomeko.entitycrosshair.config.CrosshairRenderer`; `MinecraftAccessor` → `Minecraft`.
- **Fair-play:** Provides crosshair colour-by-target-type (reveals friendly vs enemy at a glance) and "Always Show When Ready" attack-cooldown indicator. No reach, no aim assist, no wallhack — informational/aim-QoL only.
- **Summary:** A pixel-and-vector custom crosshair editor with per-target (friendly/enemy) profiles, vanilla/debug crosshair and attack-indicator customization.

---

## CrosshairV2

- **URL:** https://github.com/Polyfrost/CrosshairV2 — **archived:** yes — **is-fork:** no — **branch read:** `main` — **last commit:** 2023-05-31
- **Targets / loader:** MC **1.8.9 Forge** only (ForgeGradle `1.8.9-11.15.1.2318-1.8.9`), Java 8. Uses **Essential** (Vigilant config), not OneConfig.
- **Licence:** LGPL-3.0
- **Dependencies:** `gg.essential:essential-1.8.9-forge:1626+release-launch`, Essential loader-launchwrapper 1.1.2.
- **Features** (config `com/nxtdelivery/crosshairv2/config/CrosshairConfig.java`, a `Vigilant` subclass — all "hidden" properties, edited via a custom GUI, `Crosshairs.java`/`CustomCrosshair.java`):
  - `enabled` — Switch — `true`; `preview` — `2`; `crosshair` (selected preset id) — `1`
  - `colorType` — `2`; `crosshairType` — `1`
  - `scale` — DECIMAL_SLIDER — `1f`; `thickness` — `0`; `lineLength` — `4`; `secondLineLength` — `2`; `secondGap` — `2`; `multiplier` — `1f`; `gap` — `1`; `dotSize` — `1`
  - `chromaSpeed` — `30f`
  - `dynamicMovement` — Switch — `false` (spreads crosshair with movement)
  - `dynamicAiming` — Switch — `true` (spreads with aiming)
  - `renderInF5` — `true`; `renderOnGuis` — `true`; `chroma` — `false`
  - Colours: `color` white, `colorEntityHostile` red, `colorEntityFriend` orange, `colorContainer` blue, `colorPlayer` green
  - `currentCustom` — `0`; updater: `showUpdate` — `true`
- **Behaviour notes:** Replaces the vanilla crosshair with a customizable one (cross/dot/lines, thickness, gap, length, scale). Can colour by the type of entity under the crosshair (hostile/friendly/player/container). "Dynamic" options expand the crosshair gap with movement and aiming (spread). Custom crosshairs downloadable from a server (DownloadGui/Updater).
- **Mixins:** none (overrides the crosshair via a Forge render hook rather than Mixin).
- **Fair-play:** Colours by hostile/friendly target type; dynamic spread is cosmetic. No reach/aim.
- **Summary:** A 1.8.9-Forge crosshair customizer (Essential-based) with per-target-type colours and dynamic spread.

---

## PolyHitbox

- **URL:** https://github.com/Polyfrost/PolyHitbox — **archived:** no — **is-fork:** no — **branch read:** `main` (also `twoconfig`) — **last commit:** 2026-11-21
- **Targets / loader:** Stonecutter: MC 1.8.9 (Ornithe) + 1.21.1…26.3 (Fabric). OneConfig **1.8.0**.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig 1.8.0, Fabric Loader 0.19.3, Fabric API per-version.
- **Features** (config `org/polyfrost/polyhitbox/config/ModConfig.kt` + `HitboxConfig.kt` + `HitboxCategory.kt`, `Category.COMBAT`):
  - Global (General category): `enabled` — `false`; `hideInF1` — `true`; `thinWithDistance` — `true`; `accurateHitboxes` — `true` (1.8.9 only); `toggled` — `false`; `retainToggle` — `true`; `toggleKeybind` — F3+B by default.
  - Per-entity-category accordions (Default/General, Player, Self, Mob, Monster, Projectile, Arrow, Fireball, Wither Skull, Frames, Armor Stand, Item, End Crystal, XP), each with:
    - `override` "Override General" — `false`
    - `showCondition` — radio Always/Toggled/Hovered/Never — default `Always` (0)
    - `lineMode` — dropdown Normal/Dashed — `Normal`; `dashFactor` — 1–20 — `10`
    - `drawOverEntity` — `false`
    - `hoverColor` "Different Color on Hover" — `false`
    - `iframeColor` "Different Color in I-Frames" — `false` (see fair-play)
    - Sides / Outline / Eye Height / View Ray groups, each with `show` toggle (Sides `false`, Outline `true`, Eye Height `true`, View Ray `true`) and normal/hover/i-frame colours (Outline white, Eye Height red, View Ray blue, Sides white@63 alpha), `outlineThickness`/`eyeHeightThickness`/`viewRayThickness` — Slider 0.5–3 step 0.25 — `1.0`
    - `showGrounded` "Show Grounded Arrows" (Arrow only) — `true`
- **Behaviour notes:** Renders entity hitboxes (AABB), eye-height line, and view-direction ray for many entity categories. Respects the vanilla F3+B hitbox key, applies its own toggle keybind, can thin lines with distance, and can draw over the entity. Hitbox colours can switch on hover (crosshair over entity) and on i-frames.
- **Mixins (targets):** `HitboxRenderMixin`→`LevelRenderer`; `VanillaHitboxMixin`→`EntityHitboxDebugRenderer`; `VanillaHitboxKeyMixin`→`KeyboardHandler`; `FogCaptureMixin`→`FogRenderer`; `FovAccessor`→`Camera`; `BlockCacheMixin`→`ClientLevel`.
- **Fair-play:** Hitbox visualization is generally "allowed" but **reveals precise entity bounds, eye height, and view direction**, and the "Different Color in I-Frames" option (L235–240) reveals exactly when a mob can be damaged again. Categorised as COMBAT.
- **Summary:** A per-entity-category hitbox renderer (outline/eye-height/view-ray, hover and i-frame colouring).

---

## PolyNametag

- **URL:** https://github.com/Polyfrost/PolyNametag — **archived:** no — **is-fork:** no — **branch read:** `main` (also `support/inventory-nametag`) — **last commit:** 2026-11-24
- **Targets / loader:** Stonecutter: MC 1.8.9 (Ornithe) + 1.21.1…26.3 (Fabric). OneConfig **1.8.0**.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig 1.8.0, Fabric Loader 0.19.3, Fabric API per-version.
- **Features** (config `org/polyfrost/polynametag/client/PolyNametagConfig.kt`, `Category.QOL`):
  - `isEnabled` — Switch — `true` (L15)
  - `isRemoveNametags` "Remove nametags" — Switch — `false` (L19)
  - `heightOffset` — Slider −10..10 step 0.25 — `0.0` (L22)
  - `scale` — Slider 0–1 step 0.05 — `1.0` (L27)
  - `textType` — Dropdown "No Shadow"/"Shadow" — `0` (L31)
  - `isShowOwnNametag` — Switch — `true` (L41)
  - `isShowInInventory` — Switch — `false` (L45)
  - `isHideEntityNametagsInHiddenHud` — `true` (L49); `isHidePlayerNametagsInHiddenHud` — `true` (L53); `isHideArmorStandNametagsInHiddenHud` — `true` (L57)
  - `background` — Switch — `true` (L59); `backgroundColor` — Color — `0x3F000000` (L63); `textColor` — Color — white (L66); `overrideTextColor` — Switch — `false` (L69)
  - `rounded` — Switch — `false` (L72); `cornerRadius` — Slider 0–10 — `3.0` (L75); `paddingX` — 0–10 — `0` (L79); `paddingY` — 0–10 — `0` (L83)
- **Behaviour notes:** Tweaks vanilla nametag rendering: scale/height offset, shadow style, background box (with rounded corners + padding + custom colours), optional own-nametag display (also in inventory), hide-in-F1 rules, and the ability to remove nametags entirely. Several version-specific "WrapNametagRender" mixins cover the different 1.21.x/26.x render paths.
- **Mixins (targets):** `Mixin_ApplyScaling`→`SubmitNodeCollection`; `Mixin_EnableSelfNametag`→`LivingEntityRenderer`; `Mixin_FixSneakingNametag`→`LivingEntityRenderer`; `Mixin_RemoveNametags`/`_Avatar`/`_Living`→`EntityRenderer`/`LivingEntityRenderer`; `Mixin_RenderBackgroundShape`/`262`/`263`→`NameTagFeatureRenderer`/`TextFeatureRenderer`; `Mixin_RespectCustomNameVisible`→`MobRenderer`; `Mixin_ShowInInventory`→`EntityRenderer`; `Mixin_WrapNametagRender`/`262`/`263`/`12110`→`EntityRenderer`/`NameTagFeatureRenderer`/`SubmitNodeCollection`.
- **Fair-play:** Cosmetic nametag styling only; no information or aim advantage (can only *remove* information).
- **Summary:** A nametag appearance mod (scale, offset, shadow, background box, self/inventory display, optional removal).

---

## Keystrokes-Reborn

- **URL:** https://github.com/Polyfrost/Keystrokes-Reborn — **archived:** yes — **is-fork:** no — **branch read:** `main` (also `twoconfig`) — **last commit:** 2025-03-11
- **Targets / loader:** MC **1.8.9 Forge** (Polyfrost Gradle Toolkit; `root.gradle.kts` preprocess lists `1.8.9-forge` only; 1.12.2 commented out). OneConfig **v1** (`cc.polyfrost.oneconfig`, `0.2.1-alpha+`).
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig v1 `cc.polyfrost:oneconfig-1.8.9-forge:0.2.1-alpha+`, DevAuth 1.1.2.
- **Features** (config `org/polyfrost/polykeystrokes/config/ModConfig.kt` + `KeystrokesHud.kt` + `KeyElement.kt` + `MouseElement.kt`, HUD type):
  - `showMouseTracker` — Switch — `false` (L24)
  - `keystrokes` HUD (W/A/S/D + LMB/RMB + SPACE keys, editable layout); HUD options (`KeystrokesHud.kt`):
    - `textColor` white; `pressedTextColor` black; `backgroundColor` `(0,0,0,120)`; `pressedBackgroundColor` `(255,255,255,120)` (L19–29)
    - `border` — Switch — `false`; `borderColor` black; `borderSize` — Slider 0–10 — `2` (L31–38)
    - `roundedCorner` — Switch — `false`; `cornerRadius` — Slider 0–10 — `2` (L40–44)
    - `fadeTime` (ms) — Slider 0–1000 — `500` (L46)
  - Each `KeyElement` has `text` + `keybind` (movement keys default W/A/S/D, LMB/RMB, SPACE). `MouseElement` (mouse tracker): `circleColor` white, `circleRadius` — Number 1–100 — `5`, `speed` (sensitivity) — Number 1–100 — `2`.
- **Behaviour notes:** Draws a keystrokes HUD with fade animations and a custom layout editor; optionally a mouse-movement tracker dot. Position/scale are OneConfig HUD properties.
- **Mixins:** none (pure HUD drawing).
- **Fair-play:** Informational only (shows your own key presses). No advantage.
- **Summary:** A keystrokes + mouse-tracker HUD with a drag editor.

---

## Canelex-KeyStrokes-Revamp

- **URL:** https://github.com/Polyfrost/Canelex-KeyStrokes-Revamp — **archived:** yes — **is-fork:** no — **branch read:** `main` — **last commit:** 2023-11-24
- **Targets / loader:** MC **1.8.9 Forge** only (Polyfrost Gradle Toolkit, `root.gradle.kts` preprocess `1.8.9-forge`). OneConfig **v1**.
- **Licence:** **none** (no LICENSE file; API `license: null`).
- **Dependencies:** OneConfig v1 `cc.polyfrost:oneconfig-1.8.9-forge:0.2.1-alpha+`, DevAuth 1.1.2.
- **Features** (config `dev/salmon/keystrokes/config/KeystrokesConfig.java` + `hud/KeystrokesElement.java`, HUD type, package `dev.salmon`):
  - `keystrokesElement` HUD with (`KeystrokesElement.java`):
    - `mouseKeystrokes` — Switch — `true` (L18)
    - `jumpKeystrokes` "Jump (Space) Keystrokes" — Switch — `false` (L22)
    - `arrows` "Arrow Keystrokes" — Switch — `false` (L27)
    - `bgUnpressed` — Color — `-922746880` (L32); `textUnpressed` — white (L37); `bgPressed` — `-905969665` (L42); `textPressed` — black (L47)
    - `fadingTime` — Slider 1–250 — `100` (L51)
    - `shadow` — Switch — `false` (L56); `rounded` — Switch — `false` (L61); `cornerRadius` — Slider 0–10 — `2` (L68)
- **Behaviour notes:** A fixed-layout keystrokes HUD (W/A/S/D cluster + LMB/RMB + optional SPACE bar) in the Canelex style, with press-fade animation, arrow-key labels, shadow and rounded corners. Movement keys follow the game's keybinds.
- **Mixins:** none (pure HUD).
- **Fair-play:** Informational only.
- **Summary:** A Canelex-style keystrokes HUD (WASD + mouse + space) with fade and styling.

---

## EvergreenHUD

- **URL:** https://github.com/Polyfrost/EvergreenHUD — **archived:** no — **is-fork:** no — **branch read:** `oneconfig` (**default branch is `oneconfig`, not `main`**; also `experimental`, `rewrite`, etc.) — **last commit:** 2026-11-25
- **Targets / loader:** Stonecutter: MC 1.8.9 (Ornithe) + 1.21.1…26.3 (Fabric). OneConfig **1.9.0** + PolyCompose 3.0.7.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig 1.9.0, PolyCompose 3.0.7, Fabric Language Kotlin 1.14.1/kotlin 2.4.20, Fabric Loader 0.19.3, Fabric API per-version, commons-suncalc (sun).
- **Features** — two layers:
  1. **Global style config** (`client/config/GlobalConfig.kt`, `Category.HUD`), applied to every HUD:
     - `font` (Minecraft default); `textColor` white; `textScale` 0.5–4 — `1`; `textWeight` Regular; `caseType` Normal/UPPER/lower — `0`; `textBold`/`textItalic`/`textUnderline` — all `false`
     - `showBackground` — `true`; `bgColor` `0x80000000`; `bgRadius` 0–20 — `4`
     - `showShadow` — `false`; `shadowColor` `0x40000000`; `shadowOffsetX`/`Y` −10..10 — `2`
     - `padTop`/`padBottom`/`padLeft`/`padRight` 0–32 — `4`
     - `useGuiScale` — `true`; `customScale` 0.5–4 — `1`
  2. **~50 individual HUDs**, each with the standard OneConfig HUD options (position, scale, visibility) plus per-HUD settings. The HUD set (from `client/hud/`):
     Armor, Battery, Biome, BlockAbove, CCounter (combo), Clock (analogue) + DigitalClock, Combo, CPS, CustomImage, CustomText, Day, Direction, EntityCounter, Facing, FPS, InGameTime, Inventory, ItemCounter, ItemTracker, Keystrokes, Lore, Media (music card/lyrics), Memory, MouseStrokes, Ping, PlaceCount, PlayerHead, PlayerPreview, PlayTime, Position, PotionEffects, Reach, ResourcePack, Saturation, ServerAddress, Shader, Shape, SocialStats, Speed, TPS, Weather.
- **Behaviour notes:** An all-in-one HUD mod. HUDs are drawn via PolyCompose; several hooks integrate with the game (ping sampling via `ClientPacketListener`, saturation via `FoodData`, jump/entity-count events, shulker/ender-chest previews, player preview offscreen, real weather via WMO codes, media via OS DBus/Windows/Mac, battery via OS). A `SaturationTracker`, `FrameTimeHelper` (for TPS), etc. back the HUDs.
- **Mixins (targets):** `Mixin_AbstractContainerScreen_ShulkerPreviewTooltip`→`AbstractContainerScreen`; `Mixin_ClientCommonPacketListenerImpl_ServerChangedEvent`→`ClientCommonPacketListenerImpl`; `Mixin_ClientPacketListener_ForcePingSample`→`ClientPacketListener`; `Mixin_FoodStats_SaturationChangedEvent`→`FoodData`; `Mixin_InGameHud_SmuggleDrawContext`→`Hud`; `Mixin_LivingEntityRenderer_PlayerPreviewNameTag`→`LivingEntityRenderer`; `Mixin_LivingEntity_JumpEvent`→`LivingEntity`; `Mixin_RenderGlobal_EntityCounterEvent`→`LevelExtractor`.
- **Fair-play:** Mostly informational HUD (FPS/ping/TPS/position/clock/weather). The `ReachHud` and `EntityCounterHud` surfaces reach distance and entity counts; `PotionEffectsHud` shows active effects. No reach/aim/ESP/auto-action — these display values the client already knows (e.g. reach distance to a targeted entity).
- **Summary:** A large modular HUD suite (~50 elements) with a global text/background style system.

---

## VanillaHUD

- **URL:** https://github.com/Polyfrost/VanillaHUD — **archived:** no — **is-fork:** no — **branch read:** `main` (also `twoconfig`, `port/1.21`, many feature branches) — **last commit:** 2026-11-25
- **Targets / loader:** Stonecutter: MC 1.8.9 (Ornithe) + 1.21.1…26.3 (Fabric). OneConfig **1.9.3** + PolyCompose 3.0.7.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig 1.9.3, PolyCompose 3.0.7, Fabric Language Kotlin, Fabric Loader 0.19.5, Fabric API per-version.
- **Features** — the mod turns each vanilla HUD element into a draggable/resizable OneConfig HUD (base class `hud/VanillaHud.kt`), and captures the game's own rendering so elements can be re-placed. Vanilla elements covered (from `hud/Elements.kt` + mixins): **Health, Hunger, Armor, Air, Experience bar, Experience level, Scoreboard, Tab list, Title, Subtitle, Action bar, Status effects, Mount health, Boss bar, Hotbar, Held item tooltip, Locator bar, Closed captions.**
  - Shared per-HUD options (`VanillaHud.kt`): `ignoreModPosition`/`ignoreModScale` — `false`; plus the full OneConfig Hud style set (text scale/colour/chroma, shadow, background, border, padding, static size, alignment).
  - Element-specific options include: health `Render Text`/`Render Health`/`Smooth Health` (lerp) + `Lerp Speed` 1–1000 ms; hotbar `Reverse Direction`; armor `Alignment` Left/Right; hunger `Animation`; scoreboard `Fade Out`/`Instant Fade`, score/title colours, title background; tab list `Mode` Held/Toggle, `Animation`, `Fix TabList Entry Width`, `Show Header/Footer/Self At Top/Player's Head/3D Heads/Ping`, ping colours per band (0–75, 75–145, 145–200, 200–300, 300–400, >400); title/subtitle `Instant Fade`; status effects; boss bar (lerp); locator bar; captions.
- **Behaviour notes:** This is the most architecturally complex mod in the batch — it captures vanilla HUD draw calls (via a `HudCapture` replay system and many `Capture*`/`GuiMixin*` mixins), then re-draws each element through Compose at a user-chosen position/scale, with "ignore position/scale from other mods" and default-position pinning so elements track their vanilla spot until moved.
- **Mixins (targets):** numerous `GuiMixin<Element>` → `Hud` (`net.minecraft.client.gui.Hud`) and `GameGui`; `BossHealthOverlayMixin`→`BossHealthOverlay`; `LerpingBossEventMixin`→`LerpingBossEvent`; `LocatorBarMixin`→`LocatorBar`; `PlayerTabOverlayMixin`→`PlayerTabOverlay`; `SubtitleOverlayMixin`→`SubtitleOverlay`; accessors `IGui`, `IBossHealthOverlay`, `IPlayerTabOverlay`, `ISubtitleOverlay`, `IWindow`; capture mixins `CaptureColorMixin`→`GlStateManager`, `CaptureFontMixin`→`Font`, `CaptureItemMixin`→`ItemRenderer`, `CaptureTextureMixin`→`TextureManager`, `CaptureSwapMixin`→`Gui`/`GameRenderer`, `DrawCaptureMixin`→`GuiGraphicsExtractor`/`GuiElement`, `WindowScaleMixin`→`Window`, `ScoreboardRevisionMixin`→`Scoreboard`, `TextShadowMixin`, `UprightIconMixin`, `GuiFrameMixin`, `GuiEditorMixin`.
- **Fair-play:** Repositions/restyles vanilla HUD only; no new information or combat advantage.
- **Summary:** Makes every vanilla HUD element individually draggable/scalable/stylable.

---

## PolyBlur

- **URL:** https://github.com/Polyfrost/PolyBlur — **archived:** no — **is-fork:** no — **branch read:** `main` (also `twoconfig`) — **last commit:** 2026-11-23
- **Targets / loader:** Stonecutter: MC 1.8.9 (Ornithe) + 1.21.1…26.3 (Fabric). OneConfig **1.8.0**.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig 1.8.0, Fabric Loader 0.19.3, Fabric API per-version.
- **Features** (config `client/PolyBlurConfig.kt`, `KtConfig`, `Category.COMBAT`):
  - `isEnabled` — Switch — `true` (L12)
  - `blurType` — Dropdown Phosphor/Unity/Hybrid — `2` (Hybrid) (L13)
  - `phosphorMode` — Dropdown Weighted Max/Linear Mix/Alpha Decay — `2` (Alpha Decay) (L19)
  - `strength` "Blur Strength" — Slider 1–10 — `3` (L25)
  - `handBlurStrength` — Slider 1–10 — `5` (L32)
  - `motionBlurSamples` — Slider 4–32 — `16` (L39)
  - `velocityBuffer` — Switch — `true` (L47)
  - `translationParallax` — Switch — `true` (L52)
  - `blurHand` — Switch — `true` (L57)
- **Behaviour notes:** A motion-blur / post-process mod. "Phosphor" mode accumulates/smears old frames; "Unity" blurs along camera movement (per-pixel velocity buffer); "Hybrid" uses Unity for the world and Phosphor for the held hand. Includes velocity + reprojection passes (with 1.21.5 and legacy variants), Iris shader compatibility (velocity buffer ignored while Iris is active), and a frame clock / profiler. Category is COMBAT (marketing) but the effect is purely visual.
- **Mixins (targets):** `Mixin_ApplyPhosphorBlur`→`GameRenderer`; `Mixin_CaptureWorldMatrices`→`LevelRenderer`; `PostChainAccessor`→`PostChain`. (WIP `wip/` folder has unused monkey/moul blur experiments.)
- **Fair-play:** Visual motion blur only; no gameplay advantage.
- **Summary:** A configurable motion-blur mod (Phosphor/Unity/Hybrid) with a per-pixel velocity buffer.

---

## Blur

- **URL:** https://github.com/Polyfrost/Blur — **archived:** no — **is-fork:** yes (fork of MidnightDust "Blur" → "Blur+"-style) — **branch read:** `ornithe-1.8.9` (**default**; also `multiversion`, `plus-1.21.1…1.21.9`, `fabric-1.16…1.20.4`) — **last commit:** 2026-10-21
- **Targets / loader:** Default branch = MC **1.8.9 Ornithe** only (`build.gradle.kts` pins `com.mojang:minecraft:1.8.9`); other branches add modern Fabric/NeoForge. OneConfig **1.5.0**.
- **Licence:** MIT
- **Dependencies:** OneConfig 1.5.0, Fabric Loader 0.19.3, OSL 0.20.3, ModMenu 0.5.0, PolyCompose.
- **Features** (config `eu/midnightdust/blur/BlurConfig.java`, "Legacy GUI Blur", `Category.VISUALS`):
  - `enabled` "Enable GUI blur" — Switch — `true` (L28)
  - `blurContainers` — `true`; `blurBooks` — `true`; `blurSigns` — `true`; `blurCommandBlocks` — `true`; `blurDeathScreen` — `false`; `blurTitleScreen` — `false`; `darkenTitleScreen` — `false`; `reduceInGameBlur` — `false` (L31–52)
  - `showScreenID` — `false` (L55)
  - `forceEnabledScreens` / `forceDisabledScreens` — TextLists (JEI/REI/EMI recipe screens enabled; Iris shader screen disabled) (L58/L65)
  - `useGradient` — `true`; `radius` — Slider 0–100 — `8`; `gradientStartColor`/`gradientEndColor` — `75<<24`; `gradientRotation` — Slider 0–360 — `0` (L69–82)
  - `fadeTimeMillis` — Slider 1–2000 — `300`; `fadeOutTimeMillis` — `300` (L85/L88)
  - `blurAnimationCurve` / `backgroundAnimationCurve` — Dropdown (Flat/Sine/Quad/Cubic/Quart/Quint/Expo/Circ/Back/Elastic) — `Flat` (L91/L94)
- **Behaviour notes:** Applies a radial-blur shader (PostChain) behind GUI screens with animated fade-in/out, an optional gradient tint, per-screen-type toggles, and per-screen override lists. "Reduce container blur" lowers blur while a container is open. Reimplements the upstream "Blur"/"Blur+" behaviour on 1.8.9.
- **Mixins (targets):** `ScreenMixin`→`Screen`; `TitleScreenMixin`→`TitleScreen`; `GameRendererMixin`→`GameRenderer`; `PostChainAccessor`→`PostChain`.
- **Fair-play:** Visual GUI blur only.
- **Summary:** A GUI-background blur mod with gradients, animation and per-screen control.

---

## PolyTime

- **URL:** https://github.com/Polyfrost/PolyTime — **archived:** no — **is-fork:** no — **branch read:** `main` — **last commit:** 2026-11-20
- **Targets / loader:** Stonecutter: MC 1.8.9 (Ornithe) + 1.21.1…26.3 (Fabric). OneConfig **1.8.0**.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig 1.8.0, Fabric Loader 0.19.3, commons-suncalc 0.1.0.
- **Features** (config `client/PolyTimeConfig.kt`, `Category.QOL`):
  - `isEnabled` — Switch — `false` (L30)
  - `isIrlTime` "Use IRL time" — Checkbox — `false` (L35)
  - `irlLunarPhases` "Use IRL lunar phase" — Checkbox — `false` (L40)
  - `moonPhase` — Dropdown (Default/Full Moon/Waning Gibbous/Last Quarter/Waning Crescent/New Moon/Waxing Crescent/First Quarter/Waxing Gibbous) — `0` (L46)
  - `time` — Slider 0–24 step 0.5 — `12` (L56)
  - `fastTime` "Fast Time" — Checkbox — `false` (L61); `fastSpeed` — Slider 0.1–100 — `10` (L67)
  - `forwardKeyBind` (RBRACKET) / `backwardKeybind` (LBRACKET) — Keybinds (L73/L79)
  - `keybindSpeed` — Slider 0.1–12 — `1` (L86)
- **Behaviour notes:** Client-side time changer (the classic "TimeChanger" concept, credited to Fyu). Overrides the client world time, optionally fast-forwards it, can drive it by real (IRL) time, and can override the moon phase (including IRL lunar phase via suncalc). Only affects the client view — server time is untouched.
- **Mixins (targets):** `Mixin_ClientClockManager`→`ClientClockManager.ClientClockInstance`; `Mixin_ClientLevel`→`ClientLevel`; `Mixin_ClientLevelData`→`ClientLevel.ClientLevelData`; `Mixin_FixMoonPhases`→`SkyRenderer`; `Mixin_GameRenderer`→`GameRenderer`; `Mixin_NuitSkyboxManager`→`me.flashyreese.mods.nuit.SkyboxManager` (compat).
- **Fair-play:** Client-side cosmetic time/moon-phase change; no advantage (server time/mob spawning unaffected).
- **Summary:** A client-side time/lunar-phase changer.

---

## PolyWeather

- **URL:** https://github.com/Polyfrost/PolyWeather — **archived:** no — **is-fork:** no — **branch read:** `main` — **last commit:** 2026-11-23
- **Targets / loader:** Stonecutter: MC 1.8.9 (Ornithe) + 1.21.1…26.3 (Fabric). OneConfig **1.8.0**.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig 1.8.0, Fabric Loader 0.19.3, commons-suncalc (via util).
- **Features** (config `client/PolyWeatherConfig.kt`, `Category.QOL`):
  - `isEnabled` "Enable PolyWeather" — Switch — `false` (L17)
  - `weather` — Dropdown Clear/Rain/Storm/Snow — `0` (L23)
  - `isIrlWeather` "Use IRL weather" — Checkbox — `false` (L31)
  - `enableSounds` — Switch — `true` (L37)
  - `rainStrength` — Slider 0–1 step 0.1 — `1` (L43); `snowStrength` — `1` (L49); `thunderStrength` — `1` (L55)
- **Behaviour notes:** Client-side weather changer. Forces clear/rain/storm/snow regardless of biome, drives rain/snow/thunder intensity, plays matching weather sounds, and (optionally) uses real weather via WMO codes. Many mixins adjust precipitation/thunder strength, fog colours, sky, world colours, celestial body visibility, and cancel ground particles for the forced weather. Compat mixins for Argentum and Cera shader mods.
- **Mixins (targets):** `Mixin_AllowWeatherEverywhere`→`ClientLevel`; `Mixin_CancelGroundParticles`→`ClientLevel`; `Mixin_ModifyPrecipitationStrength`→`WeatherEffectRenderer`; `Mixin_ModifyThunderStrength`/`Mixin_ModifyWeatherFog`→`AtmosphericFogEnvironment`; `Mixin_ModifyFogColor`→`ClientLevel`; `Mixin_HideCelestialBodies`/`Mixin_FixWorldColors`/`Mixin_WeatherSound`; compat `Mixin_ArgentumWeather`, `Mixin_CeraCustomSky`, `Mixin_CeraLightMaps`, `Mixin_CeraShaderState`.
- **Fair-play:** Client-side cosmetic weather change; no advantage.
- **Summary:** A client-side weather changer (clear/rain/storm/snow + intensity + real-weather option).

---

## GlintColorizer

- **URL:** https://github.com/Polyfrost/GlintColorizer — **archived:** no — **is-fork:** no — **branch read:** `main` (also `threeconfig`, `fourconfig`, `twoconfig`) — **last commit:** 2026-11-24
- **Targets / loader:** **MC 1.8.9 (Ornithe) only** on this branch (stonecutter `versions` lists `"1.8.9"` alone). OneConfig **1.8.0**.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig 1.8.0, Fabric Loader 0.19.3, OSL 0.21.0, PolyCompose (PolyColor).
- **Features** (config `config/GlintConfig.kt` + `GlintEffectOptions.kt`, `Category.VISUALS`):
  - `enabled` — Switch — `true` (L21)
  - `globalColor` "Global Glint Color" — Color — `-8372020` (L46); "Apply Global"/"1.7 Glint Color" buttons.
  - `heldItem` / `guiItem` / `droppedItem` / `framedItem` accordions, each a `GlintEffectOptions` with:
    - `individualStrokes` — Switch — `false`; `glintColor`, `strokeOneColor`, `strokeTwoColor` — Colors — `-8372020`
    - `speed` — Slider 0.1–10 — `1.0`; `strokeRotOne` — Slider −180..180 — `-50`; `strokeRotTwo` — `10`; `scale` — Slider 0–8 — `1.0`
  - Armor: `armorGlintToggle` "Disable Armor Glint" — `false`; `armorColor` — `-8372020`
  - Shiny Pots: `potionGlint` — `false`; `potionGlintSize` — `false`; `potionGlintForeground` — `false`; `potionGlintOpaque` — `false`; `potionGlintBackground` — `false`; `shinyPots` accordion; `potionBasedColor` "Potion Color Based Glint" — `false`
- **Behaviour notes:** Recolours/retunes the enchantment glint (two-stroke glint texture rotation, speed, scale) per context (held item, GUI item, dropped item, item frame, armor, potions). Adds an optional "shiny" potion effect and can colour potion glints by potion colour. Compat mixins for OverflowAnimations, PolyCompose, Argentum.
- **Mixins (targets):** `RenderItemMixin_GlintCustomizer`→`RenderItem`; `RenderItemMixin_ShinyEffect`→`RenderItem`; `ItemRendererMixin_HandleCameraTransforms`→`RenderItem`; `ItemEntityRendererMixin_HandleCameraTransforms`→`RenderEntityItem`; `LayerArmorBaseMixin`→(armor layer); `RenderItemAccessor`→`RenderItem`; compat `GuiItemGlintsMixin_Argentum`, `GuiItemIconsMixin_Argentum`, `LegacyGlintMixin_OverflowAnimations`, `McItemGlintMixin_PolyCompose`.
- **Fair-play:** Cosmetic glint recolouring only.
- **Summary:** A per-context enchantment-glint colour/speed/rotation customizer.

---

## DamageTint

- **URL:** https://github.com/Polyfrost/DamageTint — **archived:** no — **is-fork:** no — **branch read:** `main` — **last commit:** 2026-11-20
- **Targets / loader:** Stonecutter: MC 1.8.9 (Ornithe) + 1.21.1…26.3 (Fabric). OneConfig **1.8.0**.
- **Licence:** LGPL-3.0
- **Dependencies:** OneConfig 1.8.0, Fabric Loader 0.19.3, PolyCompose (PolyColor).
- **Features** (config `client/DamageTintConfig.kt`, `Category.QOL`):
  - `enabled` "Enable Damage Tint" — Switch — `true` (L24)
  - `colorV2` "Damage Tint Color" — Color — `0x4DFF0000` (red, ~30% alpha) (L32)
  - `DamageTypeColors` accordion (`enabled` — `false`): `melee`/`mace`/`ranged`/`explosion`/`magic` all `0x4DFF0000`, `crit` `0x66CFA62B`, `block` `0x99000000` (L34–69)
  - `fade` "Fade Out Damage Tint" — Switch — `false` (L73); `fadeDuration` — Slider 1–10 ticks — `10` (L76)
  - `fadeDeath` "Fade Out Dead Entities" — Switch — `false` (L79)
- **Behaviour notes:** Recolours the red "hurt" flash on entities (the `OverlayTexture` hit-colour columns), with optional per-damage-type colours (melee/mace/ranged/explosion/magic/critical/blocked), a fade-out, and fade on death. On 1.8.9 the damage-type colours are hidden (server doesn't report types). Works by writing into the overlay texture's colour columns and by hooking entity render state.
- **Mixins (targets):** `Mixin_ModifyTintColor`→`OverlayTexture`; `Mixin_LivingEntityRenderer`→`LivingEntityRenderer`; `Mixin_LivingEntity`→`LivingEntity`; `Mixin_EntityRenderState`→`EntityRenderState`; `Mixin_EnderDragonRenderer`→`EnderDragonRenderer`; `Mixin_ClientPacketListener`→`ClientPacketListener`.
- **Fair-play:** Visual only; recolours the standard hurt flash (does not reveal hidden info).
- **Summary:** Recolours (and can fade) the entity hurt-tint flash, per damage type.

---

## BlockOverlay

- **URL:** https://github.com/Polyfrost/BlockOverlay — **archived:** yes — **is-fork:** no — **branch read:** `main` — **last commit:** 2024-06-17
- **Targets / loader:** MC **1.8.9 Forge** only (PGT, `root.gradle.kts` preprocess `1.8.9-forge`). OneConfig **v1**.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig v1 `cc.polyfrost:oneconfig-1.8.9-forge:0.2.1-alpha+`, DevAuth 1.1.2.
- **Features** (config `org/polyfrost/blockoverlay/config/ModConfig.kt`, `ModType.UTIL_QOL`):
  - `lineWidth` — Slider 0.1–5.0 — `1.0` (L17)
  - `mode` — Dropdown Hidden/Vanilla/Side/Full — `1` (Vanilla) (L23)
  - `hidePlants` — Switch — `false` (L26)
  - `barriers` — Switch — `true` (L29)
  - `ignoreDepth` — Switch — `false` (L32)
  - `persistent` — Switch — `false` (L35)
  - `gradient` — Switch — `false` (L38)
  - `overlayColor` "Draw Overlay" — Checkbox — `false` (L41); `shapedOverlay` — Switch — `false` (L44); `colorFO`/`colorSO` — Colors — white@0 alpha (L47/L50)
  - `outlineColor` "Draw Outline" — Checkbox — `true` (L53); `colorFU`/`colorSU` — Colors — black (L56/L59)
- **Behaviour notes:** The classic "block overlay" mod — draws an outline (and optional translucent face overlay) over the block the player is looking at, with modes (hidden/vanilla/side/full), depth-ignore, barrier visibility, plant hiding, and a two-colour gradient. Rendered via its own `OverlayRenderer`/`TextureManager` using captured world matrices.
- **Mixins:** none (pure render hook via `OverlayEventHandler`).
- **Fair-play:** Shows the block you're aimed at with a clearer outline (minor aim-aid, standard and widely allowed); no reach change.
- **Summary:** A block-target outline overlay mod (1.8.9 Forge).

---

## ColorSaturation

- **URL:** https://github.com/Polyfrost/ColorSaturation — **archived:** no — **is-fork:** no — **branch read:** `main` (also `twoconfig`) — **last commit:** 2026-11-22
- **Targets / loader:** Stonecutter: MC 1.8.9 (Ornithe) + 1.21.1…26.3 (Fabric). OneConfig **1.9.1**.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig 1.9.1, Fabric Loader 0.19.3, Fabric API per-version.
- **Features** (config `client/ColorSaturationConfig.kt`, `Category.QOL`):
  - `isEnabled` — Switch — `false` (L17)
  - `strength` "Saturation Strength" — Slider −1–5 step 0.05 — `1` (L21)
  - `contrast` — Slider 0–2 step 0.05 — `1` (L25)
  - `brightness` — Slider 0–2 step 0.05 — `1` (L29)
  - `hue` "Hue Shift" — Slider −180–180 step 1 — `0` (L33)
- **Behaviour notes:** A full-screen post-process shader (a fragment shader applied to the final framebuffer) that adjusts saturation, contrast, brightness and hue. When all values are identity the pass is skipped. Uses a custom `SaturationUniforms`/`SaturationHandler` and a `FullscreenTriangle` pass; on 1.8.9 the shader assets are excluded (different pipeline).
- **Mixins (targets):** `Mixin_ApplySaturation`→`GameRenderer`; `PostChainAccessor`→`PostChain`.
- **Fair-play:** Visual post-processing only.
- **Summary:** A saturation/contrast/brightness/hue screen shader.

---

## PolyFullBright

- **URL:** https://github.com/Polyfrost/PolyFullBright — **archived:** no — **is-fork:** no — **branch read:** `master` (**default**) — **last commit:** 2026-11-21
- **Targets / loader:** Stonecutter: MC 1.8.9 (Ornithe) + 1.21.1…26.3 (Fabric). OneConfig **1.8.0**.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig 1.8.0, Fabric Loader 0.19.5.
- **Features** (config `org/polyfrost/fullbright/config/FullBrightConfig.java`, `Category.QOL`):
  - `enable` "Enable FullBright" — Switch — `true` (L18)
  - `lightLevel` "Light Level" — Slider 0–15 step 1 — `15` (L25)
- **Behaviour notes:** Sets the lightmap to a fixed brightness ("fullbright"/gamma). The light-level slider controls the rendered light level; the config explicitly notes "Server-side logic such as mob spawning is unaffected." Has a legacy 1.8.9 path (WorldMixin + GameRendererMixin) and a modern path (LightTextureMixin).
- **Mixins (targets):** `LightTextureMixin`→`LightTexture`; `legacy/GameRendererMixin`→`GameRenderer` (1.8.9); `legacy/WorldMixin`→`World` (1.8.9).
- **Fair-play:** **Removes darkness / night-vision (rendered at full light), giving a vision advantage in dark areas**, though it does not change server-side spawn/sight logic. This is the classic "fullbright" grey-area mod.
- **Summary:** A fullbright mod with a light-level slider.

---

## MountOpacity

- **URL:** https://github.com/Polyfrost/MountOpacity — **archived:** no — **is-fork:** no — **branch read:** `main` — **last commit:** 2025-11-19
- **Targets / loader:** Deftu multi-version: MC 1.8.9 Forge/Fabric + 1.12.2 Forge/Fabric. OneConfig v2 (`1.0.0-alpha.177`, PolyMixin 0.8.4).
- **Licence:** LGPL-3.0
- **Dependencies:** OneConfig v2 1.0.0-alpha.177, PolyMixin, MixinExtras 0.5.0, DevAuth 1.2.1.
- **Features** (config `org/polyfrost/mountopacity/config/MountOpacityConfig.java`, `Category.QOL`):
  - `riddenHorseOpacity` "Ridden Horse Opacity (%)" — Slider 0–100 — `100` (L19)
  - `riddenPigOpacity` "Ridden Pig Opacity (%)" — Slider 0–100 — `100` (L26)
- **Behaviour notes:** Makes the horse/pig you are currently riding translucent so it doesn't block your view (horse and pig handled separately). Config imports Patcher's `patcher.toml` for migration.
- **Mixins:** none (uses entity render-state hooks in `entities/` package, `HorseOpacity`/`PigOpacity`/`EventHelper`).
- **Fair-play:** Cosmetic visibility aid while mounted.
- **Summary:** Makes ridden horses/pigs translucent for visibility.

---

## NumericalEnchantments

- **URL:** https://github.com/Polyfrost/NumericalEnchantments — **archived:** no — **is-fork:** no — **branch read:** `main` — **last commit:** 2025-11-21
- **Targets / loader:** Deftu multi-version: MC 1.8.9 Forge/Fabric + 1.12.2 Forge/Fabric. OneConfig v2 (`1.0.0-alpha.177`).
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig v2 1.0.0-alpha.177, PolyMixin, MixinExtras, DevAuth.
- **Features** (config `org/polyfrost/numericalenchantments/config/NumericalEnchantmentsConfig.java`, `Category.QOL`):
  - `numericalEnchants` "Numerical Enchantments" — Switch — (uninitialised/`false`) (L18)
  - `betterRomanNumerals` "Translate Unknown Roman Numerals" — Switch — `true` (L24)
- **Behaviour notes:** Replaces Roman-numeral enchantment levels with readable Arabic numbers (e.g. "Sharpness IV" → "Sharpness 4") and, optionally, generates Roman numerals from levels rather than relying on the language file (handles levels the lang file lacks). `RomanNumerals.java` provides the conversion.
- **Mixins (targets):** `EnchantmentMixin`→`Enchantment`; `InventoryEffectRendererMixin`→`InventoryEffectRenderer`.
- **Fair-play:** Readability only.
- **Summary:** Shows enchantment levels as numbers instead of Roman numerals.

---

## PolyZoom

- **URL:** https://github.com/Polyfrost/PolyZoom — **archived:** no — **is-fork:** no — **branch read:** `main` — **last commit:** 2026-11-23
- **Targets / loader:** MC **1.8.9 Ornithe** (pins `minecraft_version=1.8.9`, fabric-loader 0.19.3, OSL 0.21.0). OneConfig **1.8.0**.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig 1.8.0, Fabric Loader 0.19.3, OSL 0.21.0.
- **Features** (config `ZoomConfig.java` + `KeybindConfig.java`, `Category.QOL`):
  - `modEnabled` "Global Mod Toggle" — Switch — `true` (L14)
  - `zoomKeyBehaviour` — Dropdown hold/toggle — `hold` (L17)
  - `initialZoom` — Slider 1–50 — `4` (L20)
  - `zoomInTime` — Slider 0–5 step 0.05 — `1.0` (L23); `zoomOutTime` — `0.5` (L26)
  - `zoomInTransition` / `zoomOutTransition` — Dropdown (instant/linear/ease_in/out sine/quad/cubic/exp) — `ease_out_exp` (L31/L36)
  - `affectHandFov` — Switch — `true` (L39)
  - `retainZoomSteps` — Switch — `false` (L42)
  - `scrollZoom` — Switch — `true` (L45); `keybindScrolling` — Switch — `false` (L48)
  - `scrollStepCount` — Slider 0–50 — `10` (L51); `zoomPerStep` — Slider 100–500 — `150` (L54); `scrollZoomSmoothness` — Slider 0–100 — `70` (L57)
  - `relativeSensitivity` — Slider 0–100 — `100` (L60); `relativeViewBobbing` — Switch — `true` (L63); `cinematicCamera` — Slider 0–100 — `0` (L66)
  - Secondary zoom: `secondaryZoomAmount` — `4`; `secondaryZoomInTime` — `10.0`; `secondaryZoomOutTime` — `1.0`; `secondaryHideHUDOnZoom` — `true` (L69–78)
  - Keybinds (`KeybindConfig.java`): Zoom = C, Secondary Zoom = F6, Zoom In/Out unbound (L22–31).
- **Behaviour notes:** OptiFine/Zoomify-style zoom with two zoom levels (primary + secondary), eased transitions, scroll-wheel stepping, relative sensitivity/bobbing scaling, and cinematic-camera smoothing. Targets the Zoomify schema ("zoomify.json") for compatibility.
- **Mixins (targets):** `GameRendererMixin`→`GameRenderer` (FOV); `MinecraftMixin`→`Minecraft`.
- **Fair-play:** Zoom only (no reach/aim/ESP).
- **Summary:** A two-level zoom mod with eased transitions and scroll stepping.

---

## PolyZoomOld

- **URL:** https://github.com/Polyfrost/PolyZoomOld — **archived:** yes — **is-fork:** no — **branch read:** `main` — **last commit:** 2024-06-17
- **Targets / loader:** MC **1.8.9 Forge** only (PGT `root.gradle.kts` preprocess `1.8.9-forge`, SRG). OneConfig **v1**.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig v1 `cc.polyfrost:oneconfig-1.8.9-forge:0.2.1-alpha+`.
- **Features** (config `org/polyfrost/polyzoom/config/ModConfig.kt`, `ModType.UTIL_QOL`):
  - `keybind` — KeyBind (size 2) — F3 + B (L14)
  - `keybindToggle` "Keybind Activate" — DualOption Hold/Toggle — `false` (Hold) (L17)
  - `scale` — Slider 1–10 — `4` (L20)
  - `animationDuration` "Animation Speed" — Slider 0.5–2 — `1` (L23)
  - `dynamicSens` "Dynamic Zoom Sensitivity" — Switch — `true` (L29)
  - `smoothCamera` — Switch — `true` (L32)
  - `scrollToScale` "Scroll to Scale" — Switch — `false` (L38); `scrollAmount` — Slider 0.2–10 — `1` (L41)
- **Behaviour notes:** A simple zoom mod (hold/toggle, fixed scale, animation, optional dynamic mouse sensitivity and smooth camera, scroll-to-zoom). Predecessor of PolyZoom.
- **Mixins (targets):** `EntityRendererMixin`→`EntityRenderer`; `InventoryPlayerMixin`→`InventoryPlayer`.
- **Fair-play:** Zoom only.
- **Summary:** A basic 1.8.9-Forge zoom mod (hold/toggle + scale + scroll).

---

## BehindYouV3

- **URL:** https://github.com/Polyfrost/BehindYouV3 — **archived:** no — **is-fork:** no — **branch read:** `3.x` (**default**; also `2.x`) — **last commit:** 2026-11-22
- **Targets / loader:** Stonecutter: MC 1.8.9 (Ornithe) + 1.21.1…26.3 (Fabric). OneConfig **1.8.0**.
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig 1.8.0, Fabric Loader 0.19.3.
- **Features** (config `client/BehindYouConfig.kt`, `Category.QOL`):
  - `isEnabled` "Enable BehindYou" — Switch — `false` (L35)
  - Keybinds accordion: `backKeybind` (front/back view, action-driven), `backKeybindHandleMode` — Radio Hold/Toggle — Hold (L53); `frontKeybind`, `frontKeybindHandleMode` — Hold (L69); `enableF5` "Enable for Minecraft Perspective Key" — Switch — `false` (L72)
  - Animation accordion: `enabled` — `true`; `speed` "Animation Time (secs)" — Slider 0.1–2 — `0.4` (L81); `back`/`front` "…Animation" — Dropdown (Never/When Entering…/When Returning…/Always) — Both (L87)
  - FOV accordion: `enabled` — `true`; `back`/`front` FOV — Slider 30–110 — `90` (L96/L99)
  - Distance accordion: `back`/`front` "Distance" — Slider 1–4 step 0.1 — `4` (L105/L108)
- **Behaviour notes:** A modern "BehindYou"/"Snap Look" mod — keybinds to snap the camera to the third-person back/front view (like an F5 toggle), with hold/toggle modes, animated transitions, per-direction FOV and camera distance. When disabled it releases its hold on the perspective. Integrates with the vanilla F5 perspective key.
- **Mixins (targets):** `Mixin_KeepThirdPerson`→`CameraType`; `Mixin_CapturePOVSet`→`Minecraft`; `Mixin_AdjustLevel`→`Camera`; `Mixin_AdjustFov`→`Camera`.
- **Fair-play:** Perspective/camera convenience (like F5); no gameplay advantage.
- **Summary:** A keybind "snap to back/front view" camera mod with animation, FOV and distance.

---

## View-Model

- **URL:** https://github.com/Polyfrost/View-Model — **archived:** no — **is-fork:** yes (fork of `I-No-oNe/View-Model`) — **branch read:** `26.2.x` (**default**; also `1.21.11`, `26.1.x`, `old-versions`) — **last commit:** 2026-08-03
- **Targets / loader:** MC **26.2 Fabric** (also 26.1.x / 1.21.11 branches). **MidnightLib** config (not OneConfig).
- **Licence:** MIT
- **Dependencies:** Fabric Loader 0.19.3, Fabric API 0.154.0, MidnightLib 1.9.3 (bundled via `include`).
- **Features** (config `net/i_no_am/viewmodel/config/Config.java`, MidnightConfig):
  - Transforms: `HandsMode` "Transform Target" — enum Both/Hands Only/Arms Only — `BOTH`; per-hand (main + off): `Scale` 0.1–5 — `1.0`; `Position X/Y` −45..45 — `0`; `Position Z` −30..30 — `0`; `Rotation X (Pitch)` −70..70 — `0`; `Rotation Y (Yaw)` −60..60 — `0`; `Rotation Z (Roll)` −60..60 — `0`
  - Swing: `swingMode` "Active Swing Mode" — enum Default/Mainhand/Offhand — `DEFAULT`; `handSpeedSwing` "Speed Modifier (Ticks)" −5..20 — `0`; `mainSwingProgress`/`offSwingProgress` "…Offset" 0–1 — `0.0`; `noHandSwing` "Disable Swing Animation" — `false`
  - Eating: `customEatAnim` — `false`; `noFoodSwing` "Disable Vanilla Animation" — `false`; `eatX`/`eatY` "Intensity" 0–3 — `1.0`
  - Misc: `oldAnimations` "Old 1.8 Animations" — `false`; `swordSlash` "Idle Sword Slash Pose" — `false`; `skipSwapping` "Disable Item Equip Animation" — `false`; `noHandRender` "Hide Hand Models Completely" — `false`
- **Behaviour notes:** Moves/rotates/scales the first-person hand (and optionally arm) models for main and off hand independently, with swing speed/offset controls, custom eating animation, and options to hide the hand or restore 1.8-style animations. Uses item-in-hand renderer hooks.
- **Mixins (targets):** `MixinItemInHandRenderer`→`ItemInHandRenderer`; `MixinLivingEntity`→`LivingEntity`.
- **Fair-play:** Cosmetic hand-position changes only.
- **Summary:** Repositions/rotates/scales the first-person hand models (MidnightLib config).

---

## betterhurtcam

- **URL:** https://github.com/Polyfrost/betterhurtcam — **archived:** no — **is-fork:** yes (fork of `uku3lig/BetterHurtCam`) — **branch read:** `ornithe-1.8.9` (**default**; also 1.17…26.2, `multiloader`, etc.) — **last commit:** 2026-10-21
- **Targets / loader:** Default branch = MC **1.8.9 Ornithe** (`minecraft:1.8.9`, OSL 0.21.0). OneConfig **1.2.3**.
- **Licence:** MIT
- **Dependencies:** OneConfig 1.2.3 (config + config-impl), Fabric Loader 0.19.3, OSL 0.21.0, ModMenu 0.5.0, OverflowAnimations (optional compile).
- **Features** (config `net/uku3lig/betterhurtcam/BetterHurtCamConfig.java`, `Category.VISUALS`; persists to the upstream `betterhurtcam.toml`):
  - `enabled` "Enable HurtCam" — Switch — `true` (L22)
  - `multiplier` "HurtCam multiplier" — Number (float range) — `1.0` (L31)
  - `heartBlink` "Health bar blinking" — Switch — `true` (L38)
  - `type` "HurtCam type" — Dropdown OLD/YAW_BASED — `YAW_BASED` (L46)
  - `oldTypeIn18` "Use OLD HurtCam type" — Switch — `true` (L53)
- **Behaviour notes:** "nohurtcam but better" — removes/modifies the camera tilt/rotation that happens when you take damage (hurt cam). OLD type removes horizontal damage-direction rotation; YAW_BASED keeps directional tilt. Also makes the health bar blink on damage (unless OverflowAnimations takes over).
- **Mixins (targets):** `GameRendererMixin`→`GameRenderer`; `GameGuiMixin`→`GameGui`.
- **Fair-play:** Cosmetic camera/UI tweak only.
- **Summary:** Removes/tunes the damage "hurt cam" and adds health-bar blinking.

---

## ShakeTweaks

- **URL:** https://github.com/Polyfrost/ShakeTweaks — **archived:** no — **is-fork:** no — **branch read:** `main` (also `1.19`, `1.20`, `multiversion`) — **last commit:** 2025-11-21
- **Targets / loader:** Deftu multi-version: MC 1.8.9 Forge/Fabric + 1.12.2 Forge/Fabric. OneConfig v2 (`1.0.0-alpha.177`).
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig v2 1.0.0-alpha.177, PolyMixin, MixinExtras, DevAuth.
- **Features** (config `org/polyfrost/shaketweaks/config/ShakeTweaksConfig.java`, `Category.QOL`):
  - `removeViewBobbing` "Remove Screen Bobbing" — Switch — (uninitialised/`false`) (L18)
  - `mapBobbing` "Remove Map Bobbing" — Switch — (uninitialised/`false`) (L24)
- **Behaviour notes:** While "View Bobbing" is enabled, removes only the view bobbing but keeps the hand bobbing; and removes hand bobbing while holding a map. (Effectively "nohurtcam"-adjacent: kills view bob only.)
- **Mixins (targets):** `EntityRendererMixin`→`EntityRenderer`.
- **Fair-play:** Cosmetic.
- **Summary:** Removes view (and map) bobbing while keeping hand bobbing.

---

## FOVTweaks

- **URL:** https://github.com/Polyfrost/FOVTweaks — **archived:** yes (README points to https://codeberg.org/chromatic/FOVChanger instead) — **is-fork:** no — **branch read:** `master` (**default**) — **last commit:** 2026-07-07
- **Targets / loader:** Deftu multi-version: MC 1.8.9 Forge/Fabric + 1.12.2 Forge/Fabric. OneConfig v2 (`1.0.0-alpha.174`).
- **Licence:** GPL-3.0
- **Dependencies:** OneConfig v2 1.0.0-alpha.174, PolyMixin 0.8.4, DevAuth 1.2.1.
- **Features** (config `org/polyfrost/fovtweaks/config/FOVTweaksConfig.kt`, `Category.QOL`):
  - `useFovModifiers` "FOV Modifier" — Switch — `false` (L24)
  - `sprintingModifier` "Sprinting FOV" — Slider −5–5 step 0.1 — `1` (L32)
  - `flyingModifier` "Flying FOV" — `1` (L40)
  - `bowModifier` "Bow FOV" — `1` (L48)
  - `speedModifier` "Speed FOV" — `1` (L56)
  - `slownessModifier` "Slowness FOV" — `1` (L64)
  - `removeWaterFov` "Remove Water FOV" — Switch — `false` (L71)
- **Behaviour notes:** Multiplies specific FOV-change states (sprinting, flying, bow pull, speed effect, slowness effect) and can remove the underwater FOV change. Migrates Patcher's `patcher.toml`.
- **Mixins (targets):** `MixinAbstractClientPlayer`→`AbstractClientPlayer`.
- **Fair-play:** Cosmetic FOV tweaks.
- **Summary:** Per-state FOV modifiers (sprint/fly/bow/speed/slowness) + remove water FOV.

---

## OverlayTweaks

- **URL:** https://github.com/Polyfrost/OverlayTweaks — **archived:** no — **is-fork:** no — **branch read:** `main` — **last commit:** 2025-11-25
- **Targets / loader:** Deftu multi-version: MC 1.8.9 Forge/Fabric + 1.12.2 Forge/Fabric. OneConfig v2 (`1.0.0-alpha.177`).
- **Licence:** CC-BY-NC-SA-4.0 (LICENSE file; GitHub API reported `NOASSERTION`)
- **Dependencies:** OneConfig v2 1.0.0-alpha.177, PolyMixin, MixinExtras, DevAuth.
- **Features** (config `org/polyfrost/overlaytweaks/config/OverlayTweaksConfig.java`, `Category.QOL`):
  - `waterDensity` "Water Fog Density (%)" — Slider 0–100 — `100` (L19)
  - `pumpkinOverlayOpacity` — Slider 0–100 — `100` (L28)
  - `renderHandWhenZoomed` — Switch — (uninit/`false`) (L34)
  - `removeWaterOverlay` — Switch — (uninit/`false`) (L46)
  - `fireOverlayHeight` — Slider −0.5–1.5 — (uninit/`0`) (L53); `fireOverlayOpacity` — Slider 0–100 — `100` (L60); `hideFireOverlayWithFireResistance` — Switch — (uninit/`false`) (L67)
  - `customVignetteDarkness` — Switch — `false` (L81); `customVignetteDarknessValue` — Slider 0–100 — `0` (L88)
  - `suffocationOverlayBrightness` — Slider 0–100 — `100` (L95)
  - `netherPortalOpacity` — Slider 0–100 — `100` (L102)
- **Behaviour notes:** Adjusts a set of screen overlays: reduces underwater fog density (visibility), pumpkin-overlay opacity, water-overlay removal, fire-overlay height/opacity (optionally auto-hiding with fire resistance), custom vignette darkness, suffocation-overlay brightness, and nether-portal-overlay opacity. (Many further options are commented out, e.g. shield/elder-guardian/freezing overlays.)
- **Mixins (targets):** `GuiIngameMixin`→`GuiIngame`; `EntityRendererMixin`→`EntityRenderer`; `ItemRendererMixin`→`ItemRenderer`.
- **Fair-play:** **Reducing water fog density / removing the water overlay improves underwater visibility** (a mild vision advantage); the rest is cosmetic.
- **Summary:** Tunes HUD/effect overlays (water fog, fire, pumpkin, vignette, portal).

---

## RenderTweaks

- **URL:** https://github.com/Polyfrost/RenderTweaks — **archived:** no — **is-fork:** no — **branch read:** `main` — **last commit:** 2025-11-21
- **Targets / loader:** Deftu multi-version: MC 1.8.9 Forge/Fabric + 1.12.2 Forge/Fabric. OneConfig v2 (`1.0.0-alpha.177`).
- **Licence:** LGPL-3.0
- **Dependencies:** OneConfig v2 1.0.0-alpha.177, PolyMixin, MixinExtras, DevAuth.
- **Features** (config `org/polyfrost/rendertweaks/config/RenderTweaksConfig.java`, `Category.QOL`):
  - `disableLightning` "Disable Lightning Bolts" — Switch — `false` (L18)
  - `disableFishingLine` "Disable Fishing Line" — Switch — `false` (L32)
  - (many commented-out recolour options for lightning/fishing line/world border)
- **Behaviour notes:** Hides lightning bolts and the fishing line (early-stage mod; world-border and recolour options are stubbed out in comments).
- **Mixins (targets):** `RenderLightningBoltMixin`→`RenderLightningBolt`; `RenderFishMixin`→`RenderFish`.
- **Fair-play:** Cosmetic.
- **Summary:** Hides lightning bolts and the fishing line.

---

## DroppedItemTweaks

- **URL:** https://github.com/Polyfrost/DroppedItemTweaks — **archived:** no — **is-fork:** no — **branch read:** `main` (also `fletching-table-test`) — **last commit:** 2025-11-21
- **Targets / loader:** Deftu multi-version: MC 1.8.9 Forge/Fabric + 1.12.2 Forge/Fabric. OneConfig v2 (`1.0.0-alpha.177`).
- **Licence:** LGPL-3.0
- **Dependencies:** OneConfig v2 1.0.0-alpha.177, PolyMixin, MixinExtras, DevAuth.
- **Features** (config `org/polyfrost/droppeditemtweaks/config/DroppedItemTweaksConfig.java`, `Category.QOL`):
  - `staticItems` "Static Items" — Switch — (uninit/`false`) (L20)
  - `preventItemSpinning` "Prevent Item Spinning" — Switch — (uninit/`false`) (L26)
  - `droppedStackItemCount` "Dropped Stack Item Count" — Slider 0–64 — `0` (L33)
- **Behaviour notes:** Stops dropped items from bobbing up/down and spinning (so they don't face you), and caps how many dropped items are rendered (a "stack" count, not vanilla's scale). Migrates Patcher `patcher.toml`.
- **Mixins (targets):** `RenderEntityItemMixin`→`RenderEntityItem`.
- **Fair-play:** Cosmetic/performance.
- **Summary:** Stops dropped-item bob/spin and caps rendered dropped items.
