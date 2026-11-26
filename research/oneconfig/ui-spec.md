# OneConfig UI — implementation-ready visual specification

Source of truth: shallow clone at `C:\Users\Valentin\Documents\src-study\OneConfig` (Polyfrost OneConfig, Kotlin + Jetpack Compose, plus the closed-source-ish `org.polyfrost.compose` runtime — see §12).

Target renderer: a Rust immediate-mode GUI that can only draw filled rects, gradients, quads, textured sprites, clipped regions and rasterized-TTF text. Everything below is therefore expressed in **dp / sp numbers, ARGB hex and explicit geometry**, never in Compose APIs.

---

## 0. Conventions, path legend, and how to read this

### 0.1 Path prefixes used in citations

Citations are `path:line`, repo-relative, with these prefixes expanded:

| Prefix | Expands to |
|---|---|
| `A/` | `modules/ui/src/main/kotlin/org/polyfrost/oneconfig/api/ui/v1/` |
| `I/` | `modules/internal/src/main/kotlin/org/polyfrost/oneconfig/` |
| `H/` | `modules/hud/src/main/kotlin/org/polyfrost/oneconfig/api/hud/v1/` |
| `M/` | `minecraft/src/main/kotlin/org/polyfrost/oneconfig/` |
| `E/` | `modules/internal/src/main/java/org/polyfrost/oneconfig/internal/` |
| `U/` | `modules/utils/src/main/java/org/polyfrost/oneconfig/api/platform/v1/` |

So `A/theme/ThemeTokens.kt:56` means `modules/ui/src/main/kotlin/org/polyfrost/oneconfig/api/ui/v1/theme/ThemeTokens.kt`, line 56.

### 0.2 Colour notation

All colours are given as `#AARRGGBB` (Compose `Color(0xAARRGGBB)` argument order, which is also the ARGB int used by `PolyColor`). Where the source stores an opaque `0xFFRRGGBB` plus separate alpha logic, that is called out.

### 0.3 Unit mapping (dp/sp → pixels)

| Fact | Value | Citation |
|---|---|---|
| Design (logical) window size | **1391 × 700 dp** | `I/ui/OneConfigInterface.kt:217-218` |
| Max fraction of the real window the shell may occupy | **0.9** on both axes | `I/ui/OneConfigInterface.kt:219` |
| Shell scale factor | `min(0.9*windowW/(1391*density), 0.9*windowH/(700*density), 1f)`, clamped to ≥ **0.25** | `I/ui/OneConfigInterface.kt:166-175` |
| Density handed to Compose | `Density(density*scale, fontScale)` | `I/ui/OneConfigInterface.kt:176-177` |
| Pixel-grid snapping (Auto/Always/Never) | snaps the effective scale so a 14 sp anchor lands on a multiple of **5 px** (min 10 px), `pixelGrid` setting default `0` = Auto = only for Minecraft-style themes | `I/ui/themes/Provider.kt:91-95, 118-165`; `E/ThemeConfig.java:144` |
| Minecraft-side density hint | `densityScale() = uiScale` (default **1.0**) | `M/../platform/OneConfigComposeScreen.kt:94` |
| MC→window scale | `guiScale / surfaceRatio`, where `surfaceRatio = viewportWidth / windowWidth` | `U/ScreenPlatform.java:59-73` |
| UI "scale" preference | slider 50…200 %, step 5, default **100** | `E/ThemeConfig.java:62-70` |

Practical consequence: **1 dp = 1 Minecraft GUI pixel × (scale)**; at GUI scale 2 on a 1:1 display, 1 dp = 2 physical px, 1 sp = 2 physical px. The exact Compose-screen density rule lives in `org.polyfrost.compose.minecraft.screen.ComposeScreen`, which is **not in the clone** — UNKNOWN (not in clone); OneConfig only passes `densityScale()` and the pixel-grid multiplier above.

`sp` values are drawn exactly like dp (fontScale is not used by any OneConfig call site; all text styles specify explicit sp).

---

## 1. Theme

### 1.1 Which style is the default

| Fact | Value | Citation |
|---|---|---|
| Registered themes, in order | PolyfrostedDark, PolyfrostedLight, PolyGlassDark, PolyGlassLight, MinecraftDark, MinecraftLight, CatppuccinLatte/Frappé/Macchiato/Mocha, Nord, NordLight | `I/ui/themes/ThemeRegistry.kt:29-42` |
| Active theme on start / fallback | **`PolyfrostedDark`** (`DEFAULT_THEME_NAME = "Polyfrosted Dark"`) | `I/ui/themes/ThemeRegistry.kt:20, 46, 89` |
| Default persisted theme id | `"Polyfrosted Dark"` | `E/ThemeConfig.java:48` |
| `ThemeStyle` enum | `Flat`, `Glass`, `Pixel` | `A/theme/ThemeTokens.kt:17-23` |
| Style resolution | `Pixel` if Minecraft-style, else `Glass` if liquid-glass, else **`Flat`** | `I/ui/themes/Structs.kt:80-85` |
| → PolyfrostedDark style | **Flat** (not Minecraft, not liquid glass) | `I/ui/themes/Impls.kt:81-123` + `I/ui/themes/Structs.kt:80-85` |

**So the default theme is `PolyfrostedDark`, style `Flat`.**

### 1.2 PolyfrostedDark palette (complete)

Constructor order is defined at `I/ui/themes/Structs.kt:39-72`; values at `I/ui/themes/Impls.kt:84-122`.

| Token (theme field) | ARGB | UI role | Citation |
|---|---|---|---|
| `pageBackground` | `#C411171C` | main content pane fill (76.9 % alpha) | `I/ui/themes/Impls.kt:84` |
| `sidebarBackground` | `#B3151C22` | left sidebar fill (70.2 % alpha) | `I/ui/themes/Impls.kt:85` |
| `chipBackground` | `#B2232D32` | chips, search field fill | `I/ui/themes/Impls.kt:86` |
| `modCardBackground` | `#59232D32` | mod cards, option/group cards, text-field fill fallback (34.9 % alpha) | `I/ui/themes/Impls.kt:87` |
| `componentBackground` | `#FF1A2229` | opaque controls: buttons, fields, switches' neighbours, radio tray | `I/ui/themes/Impls.kt:88` |
| `popupBackground` | `#FF1A2229` | popups, menus, tooltips, toasts, colour picker | `I/ui/themes/Impls.kt:89` |
| `borderColor` | `#1AFFFFFF` | 1 dp borders everywhere | `I/ui/themes/Impls.kt:91` |
| `textColor` (primary) | `#FFD5DBFF` | titles, labels, icon tint | `I/ui/themes/Impls.kt:92` |
| `textColorSecondary` | `#FF757883` | descriptions, rest-state icons/nav labels | `I/ui/themes/Impls.kt:93` |
| `accentTextColor` (on-accent text) | `#FFFFFFFF` | text/icons on accent fills | `I/ui/themes/Impls.kt:94` |
| `shadowColor` | `#FF000000` | window/card shadows | `I/ui/themes/Impls.kt:96` |
| `controlThumbColor` | `#FFFFFFFF` | switch + slider thumbs | `I/ui/themes/Impls.kt:97` |
| `favoriteColor` | `#FFFFD700` | favourite star | `I/ui/themes/Impls.kt:98` |
| `shadowEnabled` | `true` | enables drop shadow + glass/vignette layers | `I/ui/themes/Impls.kt:100` |
| `controlTrackColor` (not overridden by this theme) | **`#FF74777F`** | switch track off, slider track | `I/ui/themes/Structs.kt:176`; `A/theme/OneConfigUITheme.kt:81` |
| Accent / brand | **`#FF2B4BFF`** | primary accent, default value of the "Accent Color" setting | `E/ThemeConfig.java:50-51` |
| Accent gradient | off by default (single colour); if enabled, the brush is a horizontal `accent → accentEnd` gradient | — | `I/ui/themes/Provider.kt:60-71` |

Derived / semantic colours (computed at runtime, not constants in the theme):

| Token | Rule | Value for PolyfrostedDark | Citation |
|---|---|---|---|
| `pageSurface` | `pageBackground` composited over black (dark theme) | `#0D1216` (computed) | `A/theme/OneConfigUITheme.kt:54` |
| `error` | `#FFE35B5B` made legible on pageSurface (min 4.5:1) | `#FFE35B5B` (already legible) | `A/theme/OneConfigUITheme.kt:56`, `A/theme/ThemeTokens.kt:95-103` |
| `warning` | `#FFFFAB1D` legible | `#FFFFAB1D` | `A/theme/OneConfigUITheme.kt:58` |
| `success` | `#FF3CAF77` legible | `#FF3CAF77` | `A/theme/OneConfigUITheme.kt:60` |
| `info` | `#FF5B9BE3` legible | `#FF5B9BE3` | `A/theme/OneConfigUITheme.kt:62` |
| `onError` / `onSuccess` | black or white, whichever contrasts more | white on all four | `A/theme/OneConfigUITheme.kt:64-66, 107-108` |
| `mutedText` | `textColorSecondary` if ≥4.5:1 on pageSurface, else `textColor` | `#FF757883` | `A/theme/OneConfigUITheme.kt:68-69` |
| `scrimColor` | black, α **0.5** dark / **0.35** light | `#80000000` | `A/theme/OneConfigUITheme.kt:71` |
| `disabledAlpha` | **0.5** | multiplies the whole subtree (`graphicsLayer alpha`) | `A/theme/OneConfigUITheme.kt:75` |
| `hoverOverlayAlpha` | **0.08** dark / 0.06 light | tint = `textColor` at this alpha | `A/theme/OneConfigUITheme.kt:77`; `A/components/Interaction.kt:94-98` |
| `pressedOverlayAlpha` | **0.14** dark / 0.12 light | | `A/theme/OneConfigUITheme.kt:79` |
| `onMedia` | `#FFFFFFFF` | text/icons over user media | `A/theme/OneConfigUITheme.kt:85` |
| `onMediaShadow` | black α 0.6 | | `A/theme/OneConfigUITheme.kt:87` |
| transparency checkerboard | `checkerLight` `#FFFFFFFF` on light themes / `#FF9A9A9A` on dark themes; `checkerDark` `#FFD0D0D0` on light / `#FF6A6A6A` on dark | for alpha swatches | `A/theme/OneConfigUITheme.kt:89-91` |
| `focusRingColor` | **accent** for Flat/Glass, black-or-white for Pixel | `#FF2B4BFF` | `I/ui/themes/Structs.kt:87-88` |

### 1.3 Surface opacity model (page + sidebar)

| Fact | Value | Citation |
|---|---|---|
| Default page opacity | **88 %** | `I/ui/themes/Structs.kt:20`; `E/ThemeConfig.java:154` |
| Default sidebar opacity | **80 %** | `I/ui/themes/Structs.kt:22`; `E/ThemeConfig.java:164` |
| Runtime holders | `ShellAppearance.pageOpacity/sidebarOpacity/glowOpacity` = 88 / 80 / 25 | `I/ui/shell/ShellController.kt:137-153` |
| Non-glass scaling | alpha := `percent/100` (absolute) | `I/ui/themes/Structs.kt:17-18, 36-37` |
| Glass scaling | alpha is *scaled* proportionally around the theme default | `I/ui/themes/Structs.kt:24-37` |
| "Reduce transparency" | composites every surface over the page colour, making it opaque | `I/ui/themes/Structs.kt:114-127` |

With the defaults (88/80) the theme already encodes those alphas (`#C4` = 196 ≈ 76.9 % page prepainted; the scale function keeps it as-is when `percent == default`), so **use the ARGB values in §1.2 directly** at default settings.

### 1.4 Hover / pressed / selected state table (default theme)

| Element | Rest | Hover | Pressed / active | Selected | Citation |
|---|---|---|---|---|---|
| Generic interactive (underlay) | transparent | `textColor` α 0.08 over shape | `textColor` α 0.14 | — | `A/theme/OneConfigUITheme.kt:77-79` |
| Nav item (sidebar) | text/icon α 0.8, no fill | text/icon full, 1 dp `borderColor` | underlay `textColor` α 0.08/0.14 | **solid accent fill** (α 1) + `accentTextColor` text + 1 dp `borderColor` | `A/components/NavItem.kt:32, 88-97`; `A/components/Selection.kt:46, 213-238, 250-254` |
| Chip | `chipBackground` | accent α 0.5 composited over chip | — | accent overlay α 1 (solid accent), text `accentTextColor` | `A/components/Chips.kt:37, 96-101` |
| List row (`selectableRow`) | transparent | `borderColor` border | — | accent α 0.2 tint | `A/components/Selection.kt:46, 240-248` |
| Card (mod/profile) | `modCardBackground`, gradient top border | `borderColor` stays, optional glass glow | — | accent ring 2 dp + accent fill α 0.08 | `A/components/Surfaces.kt:107-111`; `A/components/Selection.kt:82-86, 184-210` |
| Icon button (ghost) | no fill, icon `textColorSecondary` | fill `textColor` α 0.08, border `borderColor`, icon `textColor` | fill `textColor` α 0.14 | fill accent (selected) + `accentTextColor` | `A/components/IconButton.kt:233-268` |
| Primary button | fill transparent + **accent α 1**, text `accentTextColor` | accent α 0.75 | — | — | `A/components/TextButton.kt:158-185` |
| Secondary button | `componentBackground`, 1 dp `borderColor` | border `textColorSecondary` | — | — | `A/components/TextButton.kt:163,172` |

### 1.5 Other themes (brief)

| Theme (`id`) | Style | How it differs | Citation |
|---|---|---|---|
| Polyfrosted Dark (`polyfrosted-dark`) | Flat | **the default**, as above | `I/ui/themes/Impls.kt:81` |
| Polyfrosted Light (`polyfrosted-light`) | Flat | page `#B0D0D7F3`, sidebar `#B2D0D7F3`, chip `#59EBF2FF`, card `#59EBF2FF`, component `#FFEBF2FF`, popup `#FFD0D7F3`; border `#4DA3A3A3`; text `#FF000000` / `#FF4E5661`; accentText `#FFD5DBFF`; thumb `#FF1A2229`; favourite `#FFB57D00` | `I/ui/themes/Impls.kt:128-142` |
| PolyGlass Dark (`polyglass-dark`) | **Glass** | translucent surfaces (`#8C16181D`, `#99121419`, `#7322252C`, `#6620232A`, popup `#CC16181D`), border `#14FFFFFF`, text `#FFFFFFFF`, accentText white, `withoutGlow()`, `withInsetModFooter()`, `withTranslucentSelection()`, `withLiquidGlass()`, `withCursorEffects()` | `I/ui/themes/Impls.kt:245-276`; `I/ui/themes/Structs.kt:139-173` |
| PolyGlass Light (`polyglass-light`) | Glass | as above but light tints; text `#FF0E1118`; accentText `#FFFFFFFF`; favourite `#FFC98A00` | `I/ui/themes/Impls.kt:278-309` |
| Minecraft Dark (`minecraft-dark`) | **Pixel** | *all* radii 0, Minecraft font (`Minecraft-Bold/Regular.otf`), Minecraft icon set overrides, opaque surfaces `#FF11171C`, `withShapes(0.dp, 0.dp)` | `I/ui/themes/Impls.kt:168-205` |
| Minecraft Light (`minecraft-light`) | Pixel | page `#FFE8EDFF`, text black, radius 0 | `I/ui/themes/Impls.kt:206-243` |
| Catppuccin Latte / Frappé / Macchiato / Mocha | Flat | `paletteTheme()` generator; default accent = palette index **3** (`CATPPUCCIN_DEFAULT_ACCENT = 3` → Mauve) | `I/ui/themes/Impls.kt:334, 340-498` |
| Nord (`nord`) / Nord Light (`nord-light`) | Flat | accent palette "Nord"; default accent index **1** for dark (Frost Cyan `#FF88C0D0`), **3** for light (Frost Blue `#FF5E81AC`) | `I/ui/themes/Impls.kt:500-550` |

Theme-specific flags consumed by the renderer: `glowEnabled` (false for PolyGlass), `insetModFooter`, `translucentSelection`, `liquidGlass`, `cursorEffects`, `isMinecraftStyle` — all keyed by theme id in `I/ui/themes/Structs.kt:129-173`.

**Reproducing "Flat" faithfully means: no blur, no liquid glass, no cursor effects, no glass rim, but yes to the window drop shadow, the accent glow blob field, and the card vignette.**

---

## 2. Typography

### 2.1 Font family

| Fact | Value | Citation |
|---|---|---|
| Font family (Flat/Glass) | **Poppins**, 8 weights registered: Black(900), ExtraBold(800), ExtraLight(200), Regular(400), Light(300), Medium(500), SemiBold(600), Thin(100) | `I/ui/themes/Impls.kt:111-122` |
| Font files | `assets/oneconfig/fonts/Poppins/Poppins-{Black,ExtraBold,ExtraLight,Regular,Light,Medium,SemiBold,Thin}.ttf` (all 18 including italics ship) | `modules/ui/src/main/resources/assets/oneconfig/fonts/Poppins/` |
| Italic style | shipped but **not registered** in the family (only 8 upright faces) | `I/ui/themes/Impls.kt:112-121` |
| Pixel theme font | Minecraft OTFs: `Minecraft-Bold.otf` (Bold), `Minecraft-Regular.otf` (Normal) | `I/ui/themes/Impls.kt:198-203` |
| Fallback | `FontFamily.Monospace` for the `mono` style; Unifont (`unifont.otf`) is bundled for glyph fallback | `A/theme/ThemeTokens.kt:63` |
| HUD Poppins size | HUD text drawn with Poppins at **8 GUI px base** × `textScale` | `H/../HudText.kt:104, 261` |

### 2.2 Text styles

All styles are `TextStyle(fontFamily, fontSize, fontWeight, letterSpacing?)`. **No line-height is specified anywhere** except the inline overrides listed in §2.3, so text height = font ascent+descent (Compose "unspecified" leading, ≈1.0 em for Poppins because its metrics are tight). Implement as `lineHeight = fontSize * 1.0` unless you want Poppins' natural line box; mark this as a renderer choice.

| Style | Size | Weight (numeric) | Letter spacing | Typical use | Citation |
|---|---|---|---|---|---|
| `title` | **18 sp** | SemiBold (600) | 0 | default card/dialog titles | `A/theme/ThemeTokens.kt:56` |
| `heading` | **15 sp** | SemiBold (600) | 0 | accordion/group header labels | `A/theme/ThemeTokens.kt:57` |
| `body` | **14 sp** | Normal (400) | 0 | nav labels, general text, buttons base | `A/theme/ThemeTokens.kt:58` |
| `label` | **13 sp** | Medium (500) | 0 | field labels, tooltips, option descriptions (regular variant) | `A/theme/ThemeTokens.kt:59` |
| `small` | **12 sp** | Normal (400) | 0 | helper text, "On/Off" state labels, notification body | `A/theme/ThemeTokens.kt:60` |
| `caption` | **11 sp** | Normal (400) | 0 | timestamps, "+N more", sidebar footer version | `A/theme/ThemeTokens.kt:61` |
| `eyebrow` | **11 sp** | Medium (500) | **1.5 sp** | section eyebrows (uppercased by default) | `A/theme/ThemeTokens.kt:62` |
| `mono` | 12 sp | Normal | 0 | code | `A/theme/ThemeTokens.kt:63` |
| `titleSmall` | **16 sp** | Medium (500) | 0 | mod-card title, info headings | `A/theme/ThemeTokens.kt:45` |
| `titleLarge` | **22 sp** | Medium (500) | 0 | config subcategory headers | `A/theme/ThemeTokens.kt:46` |
| `display` | **24 sp** | SemiBold (600) | 0 | page title in the header (overridden to **Medium**) | `A/theme/ThemeTokens.kt:47`; `I/ui/components/Header.kt:217` |
| `bodyMedium` | 14 sp | Medium | 0 | header author byline | `A/theme/ThemeTokens.kt:48` |
| `smallMedium` | 12 sp | Medium | 0 | rare | `A/theme/ThemeTokens.kt:49` |
| `headingMedium` | 15 sp | Medium | 0 | **setting row title** | `A/theme/ThemeTokens.kt:50`; `I/ui/screens/ConfigScreen.kt:1177-1178` |
| `captionMedium` | 11 sp | Medium | 0 | rare | `A/theme/ThemeTokens.kt:51` |
| `titleMedium` | 18 sp | Medium | 0 | profile card label | `A/theme/ThemeTokens.kt:52`; `I/ui/screens/Profiles.kt:713` |
| `labelRegular` | 13 sp | Normal | 0 | setting-row description, tooltip text | `A/theme/ThemeTokens.kt:53`; `I/ui/screens/ConfigScreen.kt:1180-1181`; `A/components/HoverTooltip.kt:149` |

### 2.3 Explicit line-height / size overrides found

| Where | Override | Citation |
|---|---|---|
| Mod card fallback title | 16 sp Medium, **lineHeight 18 sp**, 12 dp horizontal padding | `I/ui/screens/Mods.kt:657-662` |
| Mod card footer title | 16 sp, **lineHeight 16 sp**, centred | `I/ui/screens/Mods.kt:710-717` |
| Info option heading | titleSmall, **lineHeight 19 sp** | `I/ui/components/settings/InfoOption.kt:39-45` |
| Info option body | label size, **lineHeight 16 sp** | `I/ui/components/settings/InfoOption.kt:46-54` |
| Section header eyebrow | `eyebrow.copy(letterSpacing = 0.sp)` | `A/components/LayoutPieces.kt:135-137` |
| Section header non-eyebrow | `heading` 15 sp SemiBold | `A/components/LayoutPieces.kt:324` |
| Notification row title | body size, lineHeight 20 sp, SemiBold | `I/ui/components/NotificationsCenter.kt:117-124` |
| Notification row message | small, lineHeight 21 sp | `I/ui/components/NotificationsCenter.kt:135-140` |
| Sidebar search placeholder etc. | body.copy(fontSize = metrics.fontSize) | `A/components/TextField.kt:519-528` |

### 2.4 Text colour rules

| Role | Colour | Citation |
|---|---|---|
| Primary text | `textColor` `#FFD5DBFF` | `I/ui/themes/Impls.kt:92` |
| Secondary text | `textColorSecondary` `#FF757883` | `I/ui/themes/Impls.kt:93` |
| Disabled | same colour × `disabledAlpha` **0.5** applied via graphics-layer alpha | `A/theme/OneConfigUITheme.kt:75` |
| Text on accent | `accentTextColor`, but recomputed: if contrast(accentTextColor, accent) < 3 then black/white whichever is readable | `I/ui/themes/CustomTheme.kt:487-500` |
| Legacy `§` codes | full 16-colour palette, `§l` bold, `§o` italic (skew 0.5), `§n` underline, `§m` strike, `§r` reset, `§k` dropped | `I/ui/components/Text.kt:45-113` |

---

## 3. Shapes, borders, spacing, shadows

### 3.1 Radii (default theme)

| Token | Theme field | Default radius | Used by | Citation |
|---|---|---|---|---|
| `small` | `checkBoxShape` | **6 dp** (`Radii.XS`) | checkbox, colour swatch, small icons, timeline bars | `A/theme/OneConfigUITheme.kt:95`; `I/ui/themes/Radii.kt:26`; `I/ui/themes/Impls.kt:105` |
| `medium` | `buttonShape` | **8 dp** (`Radii.SM`) | buttons, icon buttons, segmented tracks, pills | `A/theme/OneConfigUITheme.kt:96`; `I/ui/themes/Radii.kt:29`; `I/ui/themes/Impls.kt:103` |
| `large` | `modCardShape` | **16 dp** (`Radii.MD`) | mod cards, option cards, group cards, selection bar, changelog | `A/theme/OneConfigUITheme.kt:97`; `I/ui/themes/Radii.kt:32`; `I/ui/themes/Impls.kt:104` |
| `field` | `sideBarNavigationEntryShape` | **8 dp** (`Radii.SM`) | text fields, select fields, chips, search field, nav entries | `A/theme/OneConfigUITheme.kt:98`; `I/ui/themes/Impls.kt:106` |
| `pill` | derived | `RoundedCornerShape(percent = 50)` = full pill (rectangle in Pixel) | badges | `A/theme/OneConfigUITheme.kt:99` |
| `circle` | `circleShape` | circle | switch track/thumb, sliders' thumbs, avatar | `A/theme/OneConfigUITheme.kt:100`; `I/ui/themes/Impls.kt:108` |
| window / background | `backgroundShape` | **20 dp** (`Radii.LG`) | the whole shell | `A/theme/OneConfigUITheme.kt:94`; `I/ui/themes/Radii.kt:35`; `I/ui/themes/Impls.kt:102` |
| popup | `popupShape` | **16 dp** (`Radii.MD`) | menus, tooltips, toasts, colour picker, HUD panels | `A/theme/OneConfigUITheme.kt:42, 99`; `I/ui/themes/Radii.kt:32`; `I/ui/themes/Impls.kt:107` |
| mod icon | `modIconShape` | 4 dp if not overridden | mod grid icons | `I/ui/themes/Structs.kt:132, 178` |
| scrollbar | `scrollBarShape` | 8 dp if not overridden | scroll thumbs | `I/ui/themes/Structs.kt:134, 178` |

Concentricity rule for nested shapes: `inner = max(outer − padding, 0)` (`I/ui/themes/Radii.kt:57-74`). Example: menu item shape = `popupShape(16) ⊖ contentPadding(4)` = **12 dp**; card corner buttons = `modCardShape(16) ⊖ 4`, clamped to `min(buttonRadius, 24/4=6)` → **12/6/6/6** (top-start 12, others 6) (`I/ui/screens/Mods.kt:917-926`).

Pixel theme: every radius = 0 and `themedShape()` returns `RectangleShape` (`I/ui/themes/Radii.kt:79-83`).

### 3.2 Border widths

| Kind | Width | Colour | Citation |
|---|---|---|---|
| Window outline | **1 dp** | `borderColor` | `I/ui/shell/Shell.kt:280` |
| Sidebar right edge | **1 px** line | `borderColor` | `I/ui/components/Sidebar.kt:95` |
| Sidebar account top edge | 1 px | `borderColor` | `I/ui/components/Sidebar.kt:340-341` |
| Card / surface | **1 dp** | gradient: `borderColor` at top → transparent at bottom (`CardBorder.Gradient` default) | `A/components/Surfaces.kt:107-111` |
| Text field / select | **1 dp** | `borderColor` rest → `textColorSecondary` hover → accent focus → `error` invalid | `A/components/FieldDefaults.kt:68-74`; `A/components/TextField.kt:446-452` |
| Checkbox box | **1.5 dp** | `borderColor` → `textColorSecondary` hover → accent checked | `A/components/Checkbox.kt:32, 93-112` |
| Switch track | 1 dp, only on hover | `textColor` α 0.15 | `A/components/Switch.kt:39, 125-129` |
| Menu items | 1 dp only via indication/hover border | `borderColor` | `A/components/Selection.kt:263` |
| HUD editor selection | **1 px** + handles | accent | `I/ui/hud/screens/HudDesignStudio.kt:1064-1089` |
| Divider | 1 dp | `borderColor` | `A/components/LayoutPieces.kt:364-370` |
| Focus ring | 1–2 dp (implementation external) | `focusRingColor` | `A/components/Interaction.kt:123`; `I/ui/themes/Structs.kt:87` |

### 3.3 Spacing scale

| Token | Value | Citation |
|---|---|---|
| `xxs` | 2 dp | `A/theme/ThemeTokens.kt:69` |
| `xs` | 4 dp | `A/theme/ThemeTokens.kt:70` |
| `s` | 6 dp | `A/theme/ThemeTokens.kt:71` |
| `m` | 8 dp | `A/theme/ThemeTokens.kt:72` |
| `l` | 12 dp | `A/theme/ThemeTokens.kt:73` |
| `xl` | 16 dp | `A/theme/ThemeTokens.kt:74` |
| `xxl` | 24 dp | `A/theme/ThemeTokens.kt:75` |
| **page gutter** | **25 dp** | `A/theme/ThemeTokens.kt:76`; `I/ui/shell/Shell.kt:240` |

Other structural constants:

| Constant | Value | Citation |
|---|---|---|
| Page vertical padding | 19 dp (top and bottom of the content column) | `I/ui/shell/Shell.kt:322` |
| Gap between header and page content | 19 dp | `I/ui/shell/Shell.kt:323` |
| Gap between page sections (tabs ↔ grid) | 19 dp | `I/ui/screens/Mods.kt:190`; `I/ui/screens/ConfigScreen.kt:235` |
| Gap between setting rows | **8 dp** | `I/ui/screens/ConfigScreen.kt:331, 1152` |
| Gap between two option columns | **8 dp** | `I/ui/screens/ConfigScreen.kt:1235` |
| Option card inner horizontal padding | 16 dp | `I/ui/screens/ConfigScreen.kt:1330, 1386` |
| Option card inner vertical padding | 12 dp (non-nested), 0 + 8 bottom (nested) | `I/ui/screens/ConfigScreen.kt:1294, 1331` |
| Mod card grid gap | **19 dp** both axes | `I/ui/screens/Mods.kt:338-339` |
| Control heights | 28 / 32 / 38 / 45 dp (Compact / Regular / Large / ExtraLarge) | `A/components/ControlTypes.kt:15-25` |
| Control horizontal padding | Compact 10, Regular 16, Large 16, ExtraLarge 18 dp | `A/components/ControlTypes.kt:91-94` |
| Control font sizes | Compact 13, Regular 14, Large 14, ExtraLarge 16 sp | `A/components/ControlTypes.kt:91-94` |
| Control icon sizes | 14 / 16 / 16 / 20 dp | `A/components/ControlTypes.kt:91-94` |
| Control content spacing | 8 dp (buttons), 10 dp (fields) | `A/components/ButtonDefaults.kt:55`; `A/components/FieldDefaults.kt:61` |

### 3.4 Shadows, glow, blur

| Effect | Numbers | Citation |
|---|---|---|
| Window drop shadow | `Shadow(elevation/radius 50.dp, color shadowColor, spread 4.dp, offset (0, 13.dp), alpha 0.42)`; the shadow is *baked* with an erf-based gaussian of `sigma = radius*0.57735 = 28.87 dp` | `I/ui/shell/Shell.kt:252-254, 291-297` |
| Panel elevation (popups inside content) | `Modifier.shadow(16.dp, shape, clip=false, ambient=spot=shadowColor)` | `A/components/Surfaces.kt:44, 87-97` |
| Accent glow field ("glowOpacity" 25 %) | 3 blobs on a 512×512 radial sprite stretched to **radius 1400 px**; profile stops `(0,.95) (.2,.77) (.4,.42) (.5,.25) (.6,.13) (.7,.055) (.8,.018) (1,0)`; blob parameters below | `I/ui/shell/Shell.kt:96-100, 115-119, 156-176` |
| Glow blob 1 | pos (147, −199), strength 1.0, drift 60, periods 23 s / 29 s, breath 0.18 every 11 s, parallax depth 0.04, phase 0 | `I/ui/shell/Shell.kt:116` |
| Glow blob 2 | pos (893, 736), strength 1.0, drift 80, periods 31/19 s, breath 0.18/13 s, depth 0.07, phase 2.1 | `I/ui/shell/Shell.kt:117` |
| Glow blob 3 | pos (1330, 110), strength 0.55, drift 110, periods 17/26 s, breath 0.30/9 s, depth 0.11, phase 4.2 | `I/ui/shell/Shell.kt:118` |
| Parallax easing | exponential follow, rate **4 /s** | `I/ui/shell/Shell.kt:122, 145` |
| Card vignette (inside cards) | radial from centre, radius `0.9 × min(w,h)`, stops `textColor α 0 → α 0.04 → α 0.08` | `I/ui/screens/Mods.kt:721-757` |
| Card bottom glow (enabled mod) | 50 dp tall linear accent gradient with vertical mask α `0 → 0.2 → 0.4` at t `0,.4,1`, applied only at `glowStrength 0.5` when enabled | `I/ui/screens/Mods.kt:571, 619, 734-754` |
| Profile "create" card vignette | same but `0 → 0.02 → 0.05` | `I/ui/screens/Profiles.kt:649-667` |
| Glass backdrop blur | blurs the screen copy behind the shape; **blur radius is not specified in the clone** (renderer-side) — no dp value exists | `I/ui/components/BackdropBlur.kt:12-30` |
| Glass cursor effects (Glass theme only) | hover glow radius **90 dp**, white at α 0.055 (dark) / 0.2 (light); magnetic pull 0.75–1 dp; press scale 1.03; press bloom 520 ms `LinearOutSlowInEasing`, radial white α `glow*(1-b)` | `I/ui/components/GlassHover.kt:36, 49, 81, 97-101, 108-112` |
| Glass tint (Glass theme only) | fill = sampled tint at α 0.30 (dark) / 0.36 (light); rim = 55 % mix toward tint; light = 30 % white mix | `I/ui/themes/GlassTintColor.kt:30-34, 58-68` |
| Screen blur fade-in | **250 ms** linear alpha ramp | `M/../platform/OneConfigComposeScreen.kt:152`; `M/../impls/OneConfigUIScreen.kt:145` |
| Toolbar hover indicator | none (colour only) | — |

---

## 4. Main screen ("shell") layout

### 4.1 Window rules

| Rule | Value | Citation |
|---|---|---|
| Size | **fixed 1391 × 700 dp** (min = max), centred in the window | `I/ui/OneConfigInterface.kt:217-218`; `I/ui/shell/Shell.kt:265-270` |
| Scale | `min(0.9·wW/1391dp, 0.9·wH/700dp, 1)` ≥ 0.25 | `I/ui/OneConfigInterface.kt:166-175` |
| Position | always centred (`Alignment.Center`) | `I/ui/OneConfigInterface.kt:160-163` |
| Minimum usable window | the 0.25 clamp means the shell never scales below 25 %, so below ≈ 387 × 194 px of window the 1391 × 700 shell overflows the screen | derived from `I/ui/OneConfigInterface.kt:174-175` |
| Radius | 20 dp | `I/ui/themes/Impls.kt:102` |
| Border | 1 dp `borderColor` | `I/ui/shell/Shell.kt:280` |
| Shadow | see §3.4 | `I/ui/shell/Shell.kt:271` |
| Content clipping | `clipToBounds()` on the page pane (released while dragging a card) | `I/ui/shell/Shell.kt:330` |
| Background scrim (behind the whole UI) | `scrimColor` = black α 0.5, drawn by the MC screen when `dimBackground` and blur is unavailable | `A/theme/OneConfigUITheme.kt:71`; `M/../platform/OneConfigComposeScreen.kt:78-90` |
| Minimal/spotlight variant | when opened as "search only": 760 dp wide panel at top 120 dp, search bar 48 dp tall, results area 440 dp, 12 dp padding, 12 dp gap | `I/ui/shell/Shell.kt:363-415` |

### 4.2 Horizontal split

| Region | Width | Citation |
|---|---|---|
| Sidebar | **264 dp** | `I/ui/components/Sidebar.kt:77` |
| Content column | 1391 − 264 = **1127 dp** | derived |
| Page content inside gutter | 1127 − 2×25 = **1077 dp** | derived |

### 4.3 Title/header bar (there is no OS title bar; this is row 1 of the content column)

| Element | Numbers | Citation |
|---|---|---|
| Header height | **36 dp** | `I/ui/components/Header.kt:83` |
| Left group: back / forward | two `IconButton`s **32 × 32 dp**, 8 dp apart, ghost style, tint `textColor`, disabled at α 0.5 | `I/ui/components/Header.kt:90-112` |
| Gap back/forward → title | 24 dp | `I/ui/components/Header.kt:113` |
| Page title style | `display` (24 sp) with `SemiBold → Medium`, colour `textColor` | `I/ui/components/Header.kt:217, 225` |
| Breadcrumb separator | `/` in `textColorSecondary`, 6 dp horizontal padding | `I/ui/components/Header.kt:222` |
| Breadcrumb parent (clickable) | `textColorSecondary`, hover → `textColor` (150 ms) | `I/ui/components/Header.kt:228-242` |
| Author byline | `bodyMedium` (14 sp Medium), `textColorSecondary`, 8 dp after the breadcrumb | `I/ui/components/Header.kt:127-129, 268` |
| Info icon (tooltip trigger) | 14 dp glyph in a 22 dp box, `textColorSecondary` | `I/ui/components/Header.kt:307-312` |
| Right group: search + close | spaced **26 dp** apart | `I/ui/components/Header.kt:167` |
| Search field | **256 × 32 dp** (`ControlSize.Regular`), `field` radius (8 dp), fill `chipBackground`, 1 dp border, leading `search` icon 16 dp, trailing clear 22 dp button | `I/ui/components/Header.kt:390-399`; `A/components/SearchField.kt:21, 116-133`; `A/components/FieldDefaults.kt:43-44` |
| Close button | 32 × 32 dp, icon `close`, ghost style | `I/ui/components/Header.kt:170` |

### 4.4 Sidebar

| Element | Numbers | Citation |
|---|---|---|
| Width / fill / right border | 264 dp, `sidebarBackground`, 1 px `borderColor` | `I/ui/components/Sidebar.kt:77, 92-95` |
| Top padding of the nav column | 28 dp | `I/ui/components/Sidebar.kt:117` |
| Vertical spacing inside nav column | 28 dp | `I/ui/components/Sidebar.kt:118` |
| Branding logo | drawn at **167 × 19 dp**, accent-tinted | `I/ui/components/Sidebar.kt:195` |
| Nav-scroll area | start offset `(264 − 225)/2 = 19.5 dp`, 4 dp top/bottom padding | `I/ui/components/Sidebar.kt:130` |
| Entry width | **225 dp** | `I/ui/components/Sidebar.kt:164` |
| Gap between sections | **18 dp** | `I/ui/components/Sidebar.kt:217` |
| Gap between entries | **8 dp** | `I/ui/components/Sidebar.kt:251, 290` |
| Section header | width 225 dp, padding start/end 6 dp, bottom 2 dp; text style `eyebrow.copy(letterSpacing=0)` i.e. 11 sp Medium, **uppercased**, colour `textColor` (collapsible: α 0.8) | `I/ui/components/Sidebar.kt:255-261, 267, 280`; `A/components/LayoutPieces.kt:135-137, 341` |
| Section chevron | `up` icon 12 dp, rotated 90° collapsed / 180° expanded, 150 ms | `A/components/LayoutPieces.kt:87-88, 352-359` |
| Nav item height | **33 dp** | `A/components/NavItem.kt:46` |
| Nav item horizontal padding | **12 dp** | `A/components/NavItem.kt:48` |
| Nav icon→label gap | 16 dp (icon has 8 dp trailing padding, row gap 8 dp) | `A/components/NavItem.kt:50, 52, 114` |
| Nav label | `body` 14 sp Normal, 1 line, ellipsis | `A/components/NavItem.kt:76` |
| Nav rest text alpha | 0.8 | `A/components/NavItem.kt:32, 92` |
| Nav selected | solid **accent** pill (`field` radius 8 dp), text + icon `accentTextColor`, 1 dp `borderColor`; hover on unselected: text → `textColor` | `A/components/Selection.kt:46, 199-238, 250-254` |
| Selected indicator animation | selection amount 0→1 over **150 ms**, border colour 150 ms | `A/components/Selection.kt:221-222` |
| "＋N more" overflow hint | height 28 dp, start padding 26 dp, top 6 dp, `caption` 11 sp, `textColorSecondary` α 0.7 → 1.0 hover | `I/ui/components/Sidebar.kt:133-156, 162` |
| Footer account row | height **73 dp**, top border 1 px, padding 18 h / 14 v | `I/ui/components/Sidebar.kt:340-345` |
| Player head | 32 dp, radius = nav entry shape (8 dp), +2 dp x offset | `I/ui/components/Sidebar.kt:354` |
| Player name / version | name `body` `textColor`; version `caption` `textColorSecondary`; 2 dp apart | `I/ui/components/Sidebar.kt:355-366` |
| Footer buttons | info + bell, `ControlSize.Large` 38 dp, gap 4 dp, 12 dp start padding, x offset `(38−18)/2 = 10 dp` | `I/ui/components/Sidebar.kt:369-376, 421-427` |
| Bell unread dot | 6 dp accent dot + 8 dp blurred halo (7 dp blur), offset (2, −2) | `I/ui/components/Sidebar.kt:317-333` |
| Sidebar entries (runtime) | section "Mods & Options": Mods (`settings`), Profiles (`profiles`), Keybinds (`keyboard`); "Personalization": Themes (`paintbrush`), Preferences (`cog`); the "OneConfig" group (Changelog, Credits) is *not* in the sidebar — it lives in the ⓘ drop-up | `I/ui/navigation/Pages.kt:73-79`; `A/page/PageGroup.kt:15-24`; `I/ui/navigation/Routes.kt:17-27`; `I/ui/components/Sidebar.kt:387` |
| "Edit HUD" entry | pinned in its own section above the account row, icon `hud` | `I/ui/components/Sidebar.kt:200-209` |

Sidebar geometry with the 700 dp height: 28 (top) + 19 (logo) + 28 + [nav list] + 18 (bottom pad of list) + [edit-HUD section] + 18 + 73 (account) + … The exact stack is computed by Compose; the constants above reproduce it.

### 4.5 ASCII wireframe (1391 × 700 dp, not to scale vertically)

```
┌───────────────────────────────────────────────────────────────────────────────────────────────┐  radius 20, border 1 #1AFFFFFF
│ SIDEBAR 264 dp                          │  CONTENT 1127 dp                                     │
│                                         │  ╔═ header 36 dp, 25 dp gutter ═══════════════════╗  │
│  28 ↓                                   │  ║ (←)(→) 24 │ Title / sub / by / ⓘ   [search 256×32] (×) ║  │
│  [ logo 167×19 ]                        │  ╚══════════════════════════════════════════════╝  │
│  28 ↓                                   │  19 ↓                                                │
│  ┌ MODS & OPTIONS ────────── 225 ─┐     │  ┌ page content (1077 dp wide, 25 dp gutters) ─────┐  │
│  │ ▸ [icon] Mods            33 dp │     │  │                                                 │  │
│  │   8 ↓                          │     │  │  chips / tabs row            (32 dp tall)        │  │
│  │ ▸ [icon] Profiles              │     │  │  19 ↓                                           │  │
│  │   8 ↓                          │     │  │  ┌ card ┐ 19 ┌ card ┐ 19 ┌ card ┐ 19 ┌ card ┐    │  │
│  │ ▸ [icon] Keybinds              │     │  │  │255×140│   │      │    │      │    │      │    │  │
│  └────────────────────────────────┘     │  │  └───────┘   └──────┘    └──────┘    └──────┘    │  │
│  18 ↓                                   │  │  19 ↓                                           │  │
│  ┌ PERSONALIZATION ───────────────┐     │  │  … second row of cards …                        │  │
│  │ ▸ Themes / Preferences         │     │  │                                                 │  │
│  └────────────────────────────────┘     │  └─────────────────────────────────────────────────┘  │
│  … scroll … "+N more" 28 dp             │       ↑ scroll fade top/bottom                        │
│  ── 1px divider ─────────────────       │                                                       │
│  ┌ Edit HUD ──────────────────────┐     │                                                       │
│  └────────────────────────────────┘     │                                                       │
│  18 ↓                                   │                                                       │
│  ── account row 73 dp, top 1px ──       │                                                       │
│  (head 32) Player                       │                                                       │
│            OneConfig        (ⓘ)(🔔)     │                                                       │
└───────────────────────────────────────────────────────────────────────────────────────────────┘
```

Selected nav item is drawn as a solid accent rounded rect (8 dp) filling 225 × 33 with white 14 sp text, i.e. it visually dominates the sidebar.

### 4.6 Page content order

Every page is `Column(spacedBy 19 dp)`:

1. category/tab row (`TabStyle.Chips`, `ChipDefaults.size` = 32 dp, gaps 8 dp, gutter 25 dp) — `I/ui/screens/Mods.kt:201-210`
2. scroll container with `contentPadding = 25 dp` horizontal and 19 dp between items.

Scroll behaviour: `ScrollContainer`/`scrollFade` adds a top+bottom fade mask **24 dp** long (`ScrollContainerFade`) whose alpha follows the remaining scroll distance (`A/components/Scrolling.kt:54, 195-217`), and a themed scrollbar overlaying the right edge: **8 dp** thick, minimum thumb **24 dp**, thumb colour `textColorSecondary` at α **0.40** idle / **0.70** hover with a **300 ms** hover transition, radius = `scrollBarShape` (8 dp default), inset `(gutter − 8)/2` (`A/components/Scrolling.kt:46-50, 158-172, 229-243`; `I/ui/themes/Provider.kt:105`). The `gutter = PageGutter` only positions the scrollbar; it does **not** shrink the content (`reserveGutter = false` default, `A/components/Scrolling.kt:142-153`).

---

## 5. Mods page

### 5.1 Controls at the top

| Element | Numbers | Citation |
|---|---|---|
| Category chips row | `TabRow(style = Chips, size = 32 dp, fill = false)`, gap 8 dp horizontal & vertical, gutter 25 dp | `I/ui/screens/Mods.kt:201-210`; `A/components/Tabs.kt:230-241` |
| Categories displayed | All, Favorites, Hypixel, Performance, Visuals, HUD, Utility, Quality of Life, Other, Hidden (condensed to those with content) | `I/ui/screens/Mods.kt:151-170, 192-200` |
| Category icons | All = none, Favorites `star`, Hypixel `hypixel`, Performance `lightning-01`, Visuals `paintbrush`, HUD `hud`, Utility `settings`, QoL `qol`, Other = none, Hidden `eye-off` | `I/ui/screens/Mods.kt:158-167` |
| Sort control | top-end of the page, right padding 25 dp, segmented tabs `ControlSize.Large` (38 dp), icon-only tabs: `dots-grid` (custom) and `capitalized` (A→Z) plus any plugin sort modes; tooltip below-centre | `I/ui/screens/ModSortControl.kt:25-26, 57-66` |
| Segmented track | height 38 dp, `buttonShape` 8 dp, inset **3 dp** (`ChoiceInset`), items 2 dp apart, selected item fill `textColor` α 0.1; item shape = 8 ⊖ 3 = 5 dp | `A/components/Tabs.kt:210-229, 53, 358`; `A/components/SlidingIndicator.kt:46` |
| Drafting/sort confirmation | popup below-end, primary confirm button | `I/ui/screens/ModSortControl.kt:68-78` |

### 5.2 Card grid

| Property | Value | Citation |
|---|---|---|
| Columns | **fixed 4** (never responsive) | `I/ui/screens/Mods.kt:337` |
| Column gap / row gap | **19 dp** / **19 dp** | `I/ui/screens/Mods.kt:338-339` |
| Grid horizontal padding | 25 dp | `I/ui/screens/Mods.kt:340` |
| Computed card width | `(1077 − 3×19) / 4 = **255 dp**` | derived from above |
| Card height | **140 dp** | `I/ui/screens/Mods.kt:630` |
| Card radius / fill / border | 16 dp / `modCardBackground` / 1 dp gradient `borderColor → transparent` (top→bottom) | `I/ui/screens/Mods.kt:629-644`; `A/components/Surfaces.kt:117-122` |
| Card type section headers | full-row span, `SectionHeader` with dividers on both sides, 14 dp icon, 11 sp Medium caption in `textColorSecondary`, `(count)` when collapsed, chevron | `I/ui/screens/Mods.kt:398-420` |
| Reorder animation | spring `StiffnessMediumLow`, visibility threshold `IntOffset.VisibilityThreshold` | `A/a11y/Motion.kt:56-58` |
| Drag ghost outline of the drop slot | dashed `6 dp on / 4 dp off`, 1.5 dp stroke; normal case `textColorSecondary` α 0.6 + fill α 0.08; drop-as-favourite case `favoriteColor` | `I/ui/screens/Mods.kt:547-564` |

### 5.3 Card anatomy (top→bottom)

| Part | Numbers | Citation |
|---|---|---|
| Body area (top, flexible) | centred preview, else 48 dp icon, else fallback title (16 sp Medium, lh 18, 12 dp h pad) | `I/ui/screens/Mods.kt:646-665` |
| Preview inset | 12 dp horizontal, 8 dp vertical | `I/ui/screens/Mods.kt:653` |
| Footer | min height **36 dp**, fill `textColor` α 0.08 when off, **solid accent** when on (animated), 8 dp horizontal / 2 dp vertical padding; title 16 sp `lineHeight 16 sp`, centred, `accentTextColor` when on (or `textColor` when contrast<3), `textColor` α 0.75 when off | `I/ui/screens/Mods.kt:569, 666-718` |
| Footer accent sheen | vertical white gradient α 0.2 → 0.04 when on | `I/ui/screens/Mods.kt:683`; `A/components/Slider.kt:75-78` |
| Inset-footer themes (Glass only) | footer inset 5 dp sides / 5 dp bottom, radius `16 ⊖ 6 = 10 dp` | `I/ui/screens/Mods.kt:573, 669-675` |
| Favourite star (top-start) | 24 dp hit box with 18 dp icon, inset 4 dp; not shown at all until card hover (alpha 0 → **0.6**), alpha **1** when favourited or when the button itself is hovered; colour `favoriteColor` (`#FFFFD700`) when set, else `textColor` | `I/ui/screens/Mods.kt:759-772, 833-856, 878-915, 929-932` |
| Hide eye (next to star) | same 24 dp/18 dp box, shown on hover or when hidden | `I/ui/screens/Mods.kt:859-876` |
| Toggle indicator (top-end) | box inset 11 dp; ring **12 dp** diameter, 1 px; dot **6 dp**; glow radius `12/2 + 5 = 11 dp`; ring colour = `lerp(Accent, #FF181600, 0.42)` (≈ `#FF233594` computed) → gradient to same for accentEnd; unavailable: `#FF4F505B` at α 0.5 | `I/ui/screens/Mods.kt:575-581, 773-829` |
| Locked mods | `lock` icon 12 dp instead of the ring; accent when enabled, `textColor` α 0.6 when off | `I/ui/screens/Mods.kt:783-788` |
| Hover | no fill change; the favourite star/eye fade in; optional glass glow (Glass theme only); footer is already accent-tinted by the enabled state, not hover | `I/ui/screens/Mods.kt:850, 871` |

### 5.4 Context menu (right-click a card, or ⋮)

| Item | Icon | Notes | Citation |
|---|---|---|---|
| Open settings | `settings` | | `I/ui/screens/Mods.kt:442-449` |
| Add/Remove from favorites | `star` / `star-filled` | | `I/ui/screens/Mods.kt:450-457` |
| Hide/Show mod | `eye-off` / `eye` | | `I/ui/screens/Mods.kt:458-464` |
| Enable/Disable | `check-circle` / `x-circle` | disabled for read-only toggles with a reason | `I/ui/screens/Mods.kt:465-484` |
| Open HUD editor | `hud` | after a separator | `I/ui/screens/Mods.kt:486-488` |
| Reset to defaults | `refresh`, danger style | after a separator | `I/ui/screens/Mods.kt:489-498` |
| Copy mod id | `copy`, detail = id | | `I/ui/screens/Mods.kt:499-500` |

Empty states (no favourites / no hidden mods): centred `EmptyState` — icon **32 dp**, padding **24 dp**, 8 dp gaps, text max width **420 dp**, optional 160 dp progress bar (see `A/components/EmptyState.kt:38-40, 82-116`).

---

## 6. Mod config page

### 6.1 Page header = the shell header

Back button, breadcrumb (`Mod / Sub / SubSub`), author byline, ⓘ tooltip, optional "Original menu" primary compact button (`external-link` icon) — see §4.3. The `mod config` route additionally resolves the title from the config tree (`I/ui/components/Header.kt:189-212`).

### 6.2 Category tabs & subcategory headers

| Element | Numbers | Citation |
|---|---|---|
| Category tab row | `TabStyle.Chips`, chip size Regular 32 dp, gutter 25 dp, only shown when >1 category and no local search | `I/ui/screens/ConfigScreen.kt:236-238, 422-438` |
| Category header row (inside the flat list) | `SectionHeader` non-eyebrow, padding top 8 / bottom 2, text `labelStyle` (eyebrow, 11 sp Medium, **not** uppercased here), colour `textColorSecondary` | `I/ui/screens/ConfigScreen.kt:623-633` |
| Subcategory header | `titleLarge` 22 sp Medium, `textColor`, padding top 8 / bottom 2 | `I/ui/screens/ConfigScreen.kt:635-645` |
| Option list gap | 8 dp | `I/ui/screens/ConfigScreen.kt:331` |
| List horizontal padding | 25 dp | `I/ui/screens/ConfigScreen.kt:332` |

### 6.3 Setting row ("option card")

Every leaf option is wrapped in its own card; groups/accordions get one card containing their header + body.

| Property | Value | Citation |
|---|---|---|
| Card | full content width (1077 dp), `modCardShape` 16 dp, fill `modCardBackground`, 1 dp gradient border | `I/ui/screens/ConfigScreen.kt:1115, 1120-1122`; `A/components/Surfaces.kt:117-122` |
| Row min height | `max(label, action, control, OptionControlHeight=32 dp) + 2×12 dp` | `I/ui/screens/ConfigScreen.kt:64, 1400-1401` |
| Horizontal padding | 16 dp left and right | `I/ui/screens/ConfigScreen.kt:1386` |
| Vertical padding | 12 dp (root rows); nested rows 0 top, 8 dp bottom | `I/ui/screens/ConfigScreen.kt:1294, 1331` |
| Label column | icon 32 dp + 12 dp gap + title/description column (2 dp apart); title `headingMedium` 15 sp Medium `textColor`; description `labelRegular` 13 sp Normal `textColorSecondary` | `I/ui/screens/ConfigScreen.kt:1424-1456, 1435-1452` |
| Control alignment | right edge, vertically centred; gap label↔control 16 dp | `I/ui/screens/ConfigScreen.kt:1395-1405` |
| Control default width | `LocalOptionWidth` = **300 dp** (HUD panel: 210/220 dp) | `I/ui/components/settings/Options.kt:61`; `I/ui/screens/ConfigScreen.kt:1474, 1511` |
| Stacked layout (label above control) | used when `titleWidth + iconWidth > available`, or when the control is full-width, or in compact contexts; then: label top-left, 10 dp gap, controls right-aligned on the next line, 12 dp gap between action button and control | `I/ui/screens/ConfigScreen.kt:1397-1419` |
| Full-width controls (sliders, info) | column layout: label row (10 dp gap), control full width, 12 dp action button gap | `I/ui/screens/ConfigScreen.kt:1322-1346` |
| Separators | **none** between rows — the 8 dp gap and distinct card surfaces do the separation | `I/ui/screens/ConfigScreen.kt:331` |
| Hover | row hover shows a "⋮ more" ghost icon button (compact 28 dp) that fades in at 150 ms; right click opens the same menu | `I/ui/screens/ConfigScreen.kt:1305-1320`; `I/ui/components/settings/Options.kt:314-320` |
| Disabled/locked | whole row alpha 0.5, interactions blocked | `I/ui/screens/ConfigScreen.kt:602, 1295`; `I/ui/components/Common.kt:71-84` |
| Jump-to-from-search highlight | accent fill α 0.14 + 1.5 dp accent stroke, in 150 ms, hold 700 ms, out 900 ms | `I/ui/screens/ConfigScreen.kt:374-394, 406-419` |

### 6.4 Groups / accordions

| Property | Value | Citation |
|---|---|---|
| Container | `GroupCard` = the same option card (full width, 16 dp radius, `modCardBackground`) | `I/ui/screens/ConfigScreen.kt:863-866` |
| Header | padding 16 dp h / 12 dp v; 16 dp gap between label and trailing; trailing row 12 dp gap between switch and chevron | `I/ui/screens/ConfigScreen.kt:975-992` |
| Header label | optional 32 dp icon; title `heading` 15 sp SemiBold; description uses `descriptionStyle()` = `labelRegular` 13 sp Normal `textColorSecondary`; 2 dp between them | `I/ui/screens/ConfigScreen.kt:996-1001, 788-802, 1180-1181` |
| Sticky head switch | `Switch` with "On/Off" state label, 12 dp before the chevron | `I/ui/screens/ConfigScreen.kt:948-955, 1077-1095` |
| Chevron | 12 dp `up`, 90° collapsed / 180° expanded, 150 ms | `A/components/LayoutPieces.kt:352-359` |
| Body | 2-column grid: `columnWidth = (maxWidth − 8)/2`, rows 8 dp apart; a control that doesn't fit becomes a single full-width or half-width row | `I/ui/screens/ConfigScreen.kt:1215-1262` |
| Body padding | 16 dp left/right, 12 dp bottom | `I/ui/screens/ConfigScreen.kt:852, 688` |
| Expand/collapse animation | `expandVertically(spring(StiffnessMediumLow, IntSize.VisibilityThreshold))`, symmetric shrink | `I/ui/screens/ConfigScreen.kt:695-703` |

### 6.5 Control-by-control

All controls below inherit: container `field` shape (8 dp) at 32 dp height unless stated, 1 dp border in the field-state colour, `componentBackground` fill (or `modCardBackground` for text fields), disabled = global α 0.5.

#### 6.5.1 Switch

| Property | Value | Citation |
|---|---|---|
| Track | **42 × 21 dp**, fully rounded (circle shape) | `A/components/Switch.kt:45-47` |
| Track fill — off | `controlTrackColor` = `#FF74777F` | `A/components/Switch.kt:41, 122`; `A/theme/OneConfigUITheme.kt:81` |
| Track fill — on | the same track plus an **accent overlay** at alpha 0→1 (accent brush drawn over the track shape), so on-colour = `#FF2B4BFF` | `A/components/Switch.kt:123, 135` |
| Mixed | overlay at α 0.5 | `A/components/Switch.kt:116` |
| Thumb | circle, height = 21 − 2×3 = **15 dp**, fill `controlThumbColor` `#FFFFFFFF` | `A/components/Switch.kt:49, 136-150` |
| Thumb travel | `(trackInnerW − thumb) × progress` = `(36 − 15) × progress`, i.e. **0 → 21 dp** | `A/components/Switch.kt:139-146` |
| Off-state label | "Off" 12 sp Normal `textColorSecondary`, 8 dp before the track; "On" when on | `A/components/Switch.kt:87, 105, 111-114` |
| Hover | 1 dp track border `textColor` α 0.15 | `A/components/Switch.kt:39, 125-129` |
| Animations | accent α **150 ms tween**; thumb `spring()` default (stiffness 1500, damping ratio 1.0 = no bounce) | `A/components/Switch.kt:123-124` |
| Row height | 32 dp min (control height) or track 21 dp — the switch row is 32 dp because of `OptionControlHeight` | `I/ui/components/settings/Options.kt:64` |

#### 6.5.2 Checkbox

| Property | Value | Citation |
|---|---|---|
| Box | **24 × 24 dp**, radius 6 dp | `A/components/Checkbox.kt:30, 102-113` |
| Fill | `componentBackground` `#FF1A2229` | `A/components/Checkbox.kt:109` |
| Border | **1.5 dp**: `borderColor` rest → `textColorSecondary` hover → accent checked | `A/components/Checkbox.kt:32, 93-112` |
| Checked overlay | accent brush alpha 0→1 over the box | `A/components/Checkbox.kt:92, 106-112` |
| Tick | `tick` icon **26 dp** (slightly larger than the box, clipped), tint `textColor` | `A/components/Checkbox.kt:31, 121` |
| State label | optional "On/Off" 12 sp `textColorSecondary`, 8 dp gap | `A/components/Checkbox.kt:54-57, 77-84` |
| Animations | accent 150 ms; tick fade in/out 150 ms | `A/components/Checkbox.kt:92, 116-120` |

#### 6.5.3 Slider (with number field)

| Property | Value | Citation |
|---|---|---|
| Default size | Regular → thumb **19 dp**, track **5 dp** | `A/components/Slider.kt:82, 86-98` |
| Other sizes | Compact 14/4, Large 21/6, ExtraLarge 24/7 | `A/components/Slider.kt:86-98` |
| Track fill (rest) | `controlTrackColor` `#FF74777F`, radius 6 dp (`checkBoxShape`) | `A/components/Slider.kt:100-101, 266-273`; `A/components/Slider.kt:146` |
| Track fill (elapsed) | **accent brush** (solid `#FF2B4BFF`), clipped to the track radius; the filled rect runs from x = 0 to x = `fraction × trackW` (full-bleed under the thumb, not thumb-centred) | `A/components/Slider.kt:260-296` |
| Thumb | circle `controlThumbColor`, 19 dp, x = `fraction × (trackW − 19 dp)` | `A/components/Slider.kt:299-318` |
| Muted state (multi-select mixed) | elapsed fill replaced by `textColorSecondary` α 0.4; thumb α 0.5 | `A/components/Slider.kt:74, 84, 103-104, 289-295` |
| Layout in a row | `Slider` width = `LocalOptionWidth` (300 dp), 16 dp gap, then `NumberField` | `I/ui/components/settings/SliderOption.kt:36-58` |
| Number field | **87 dp** wide, 32 dp min height, field radius, fill `componentBackground`, padding start 8 / end 2 dp; text 13 sp; up/down step arrows each **18 × 13 dp** with 14 dp glyph, stacked on the right | `A/components/NumberField.kt:69-71, 221-312`; `I/ui/components/settings/SliderOption.kt:48-57` |
| Step-arrow colours | rest `textColorSecondary`, hover `textColor`, disabled `textColorSecondary` α 0.5 | `A/components/NumberField.kt:372-379` |
| Value dragging | number field supports drag-scrub: **3 px per step**, shift ×10 | `A/components/NumberField.kt:73-74, 355-358` |
| Expression input | field accepts `+ - * / ( )` and evaluates | `A/components/NumberField.kt:243-249, 434-485` |
| Animations | thumb position spring(); muted 150 ms | `A/components/Slider.kt:139-140` |
| Tick sound | throttled to at most one per **70 ms** | `A/components/Slider.kt:71, 320-333` |

#### 6.5.4 Dropdown / Select

| Property | Value | Citation |
|---|---|---|
| Field | min width **160 dp**, height ≥ 32 dp, padding h 12 dp, radius 8 dp | `A/components/Select.kt:46-48, 305-318` |
| Fill | `componentBackground`; expanded tint = accent α **0.2** composited over the fill; `SelectOpenStyle.Accent` = solid accent with `accentTextColor` text | `A/components/Select.kt:69, 278-284` |
| Text | `body` at `ButtonDefaults.fontSize(size)` = 14 sp; rest `textColorSecondary`, hover/expanded `textColor`, muted `mutedText` | `A/components/Select.kt:285-293, 322` |
| Chevron | `up` icon 16 dp, rotation 180° closed → 0° open over **150 ms** (`animateFloatAsState` default) | `A/components/Select.kt:294, 330` |
| Border | field-state border, hover = `textColorSecondary` | `A/components/FieldDefaults.kt:68-74` |
| Popup | same width as the anchor (`matchAnchorWidth = true`), placed below-start by default, gap 6 dp, animated fade + scale 0.95 over 150 ms | `A/components/Select.kt:116`; `A/components/Dropdown.kt:99, 142, 189-199` |
| Menu surface | `popupBackground`, `popupShape` **16 dp**; the popup surface adds no border by default (a 1 dp accent border only when `selected`); the drop shadow is drawn by the bridge | `A/components/Surfaces.kt:57-61` |
| Menu geometry | min width 170 dp, max width 280 dp, content padding 4 dp, item spacing 2 dp, item shape = popup ⊖ 4 = **12 dp** | `A/components/Menu.kt:87-114` |
| Menu item | Compact: height 30 dp, text 13 sp, icon 14 dp, horizontal inset 8 dp; Regular: 38 dp / 14 sp / 18 dp / 10 dp | `A/components/Menu.kt:79-81` |
| Menu item colours | text `textColor`, hover fill `textColor` α **0.10** (`MenuDefaults.HOVER_ALPHA`), checked item shows a check glyph, disabled `textColorSecondary` | `A/components/Menu.kt:104, 259-274` |
| Submenu delay | 200 ms hover before a submenu opens | `A/components/Menu.kt:106` |

#### 6.5.5 Colour picker

Trigger (in the setting row):

| Property | Value | Citation |
|---|---|---|
| Field | height ≥ 32 dp, padding 12 dp h / 4 dp v, radius 8 dp, 1 dp border `textColor` α 0.3 on hover/open else `borderColor` | `I/ui/components/settings/ColorOption.kt:400-414` |
| Fill | the current colour (or a horizontal colour→gradient brush) | `I/ui/components/settings/ColorOption.kt:377-378, 412` |
| Content | `paintbrush` 14 dp + hex text 13 sp, both tinted black/white by contrast against the colour | `I/ui/components/settings/ColorOption.kt:371, 418-424` |
| Max width | fills the 300 dp option width by default | layout in §6.3 |
| Swatch-only variant (24 dp) | 24 dp square, 6 dp radius, checkerboard, 1 dp border (`textColor` α 0.3 hover/open), hover colour animation **120 ms** | `A/components/ColorPicker.kt:51, 254-304, 258-262` |

Popup (all values from `I/ui/components/settings/ColorOption.kt`):

| Property | Value | Citation |
|---|---|---|
| Panel | **280 dp** wide, padding 16 dp, 14 dp vertical gap between blocks, `popupShape` 16 dp, `popupBackground`, placed below-end | `I/ui/components/settings/ColorOption.kt:466-472`; `:432` |
| Title row | "Colour picker" 14 sp `textColor`, 14 dp `close` icon `textColorSecondary` | `:474-491` |
| Saturation/brightness pane | full width × **160 dp**, radius 8 dp, white→hue horizontal gradient + transparent→black vertical gradient | `:493-508` |
| SB selector | **14 dp** circle, 2 dp white border, filled with the current colour, centred on the position | `:531-546` |
| Hue bar | full width × **16 dp**, radius 8 dp, red→yellow→green→cyan→blue→magenta→red | `:548-561` |
| Hue knob | **10 × 10 dp** white circle, 1 dp white α 0.5 border, offset −5 dp | `:591-599` |
| Opacity row (when alpha) | label "Opacity" 12 sp `textColorSecondary`, right value field **56 dp** wide with "%" suffix, 8 dp h / 4 dp v padding, 12 sp | `:602-623, 887-903` |
| Alpha bar | full width × **16 dp**, checkerboard + opaque→transparent gradient, same 10 dp white knob | `:624-677` |
| Chroma row | 14 dp sweep-gradient dot + "Chroma" 13 sp `textColor` + Switch (when the property is a `PolyColor`) | `:683-714` |
| Gradient row | 14 dp colour→gradient dot + "Gradient" 13 sp + Switch | `:716-743` |
| Gradient swatch + hex field (when gradient on) | 24 dp swatch (8 dp radius, 1 dp border) + hex field, "#" prefix, max 8 chars | `:745-773` |
| Chroma speed (when chroma on) | label 12 sp + value field 56 dp ("x"), 16 dp bar `componentBackground → accent`, range 0.1–4.0, 10 dp knob | `:775-850` |
| Bottom row | 32 dp colour swatch (8 dp radius, 1 dp border) + hex field (`#RRGGBB` or `#AARRGGBB`), prefixed "#", 13 sp, filter `[A-Za-z0-9]`, max 8 | `:852-883` |

#### 6.5.6 Keybind field

| Property | Value | Citation |
|---|---|---|
| Field | min height 32 dp, radius 8 dp, padding **12 dp h / 4 dp v**, 8 dp gap, `keyboard` icon 16 dp | `A/components/KeybindField.kt:166, 177-207` |
| Fill | `componentBackground` rest; hover = componentBackground × 0.8 alpha; recording = accent α **0.2** | `A/components/KeybindField.kt:48-49, 138-146` |
| Border | `borderColor` rest; `textColor` α 0.2 hover; **accent** recording; **error** on conflict | `A/components/KeybindField.kt:50, 147-156` |
| Text | keybind name, `body` at 13 sp, `textColor` (accent when recording, error on conflict) | `A/components/KeybindField.kt:157-165, 198-206` |
| Recording placeholder | "Press keys..." then "Ctrl + Shift + K" style joined names | `A/components/KeybindField.kt:168-173` |
| Esc / Backspace / Delete | clears the bind | `A/components/KeybindField.kt:222-227` |
| Animations | fill/border/text 150 ms each | `A/components/KeybindField.kt:138-165` |

#### 6.5.7 Text field

| Property | Value | Citation |
|---|---|---|
| Frame | `field` 8 dp, 1 dp border, fill `modCardBackground` (text fields) / `chipBackground` (search) | `A/components/TextField.kt:66-70`; `A/components/SearchField.kt:154` |
| Height / padding | `max(32 dp, content)`; 12 dp h, 4 dp v (single line) or 7 dp (multi-line) | `A/components/TextField.kt:492-498` |
| Text | `textColor`, fontSize = control metric (14 sp Regular), caret = accent (blinking width is renderer default) | `A/components/TextField.kt:427, 328` |
| Placeholder | `textColorSecondary`, same size, `body` weight Normal | `A/components/TextField.kt:521-528` |
| Leading icon | 16 dp, colour animates `textColorSecondary → textColor` on hover/focus | `A/components/TextField.kt:490, 504` |
| Trailing | arbitrary (clear button 22 dp) | `A/components/SearchField.kt:21, 116-133` |
| Border animations | 150 ms colour | `A/components/TextField.kt:446` |

#### 6.5.8 Button (TextButton)

| Style | Fill | Border | Content | Citation |
|---|---|---|---|---|
| Primary | transparent + accent overlay **α 1**; hover α **0.75** | none | `accentTextColor` | `A/components/TextButton.kt:158-185`; `A/components/ButtonDefaults.kt:57, 105` |
| Secondary | `componentBackground` | 1 dp `borderColor`; hover `textColorSecondary` | `textColor` | `A/components/TextButton.kt:163, 172` |
| Ghost | transparent | none (underlay indication only) | `textColor` | `A/components/TextButton.kt:160` |
| Tinted | accent α 0.16 / hover 0.28 | accent α 0.55 → 1 | `textColor` | `A/components/TextButton.kt:164, 173` |
| Danger | error α 0.12 / hover 0.20 | none | `error` | `A/components/TextButton.kt:162` |
| DangerFilled | error solid, hover α 0.75 | none | white unless contrast < 3 → `onError` | `A/components/TextButton.kt:161`; `A/components/ButtonDefaults.kt:106` |
| Footer | `chipBackground` | 1 px top hairline `borderColor` → `textColorSecondary` hover | `textColor` | `A/components/TextButton.kt:166, 219-231` |
| Geometry | height ≥ 28/32/38/45 dp by size; 4 dp vertical padding, h padding per control metrics; text 13/13/14/16 sp Normal; icon 14/16/16/20 dp; gap 8 dp | | | `A/components/TextButton.kt:260-276`; `A/components/ButtonDefaults.kt:93-99` |
| Animations | fill/border/accent colour 150 ms | | | `A/components/TextButton.kt:158-184` |

#### 6.5.9 Radio group

| Property | Value | Citation |
|---|---|---|
| Tray | radius 8 dp (`field`), 1 dp `borderColor`, fill `componentBackground`, padding **3 dp** (`ChoiceInset`); item shape = 8 ⊖ 3 = 5 dp | `A/components/RadioGroup.kt:55, 60, 67-78`; `A/components/SlidingIndicator.kt:46` |
| Items | min height `32 − 2×3 = 26 dp`, padding 8 dp h / 4 dp v, text 12 sp | `A/components/RadioGroup.kt:39, 139-140` |
| Selected indicator | **solid accent** rounded rect behind the item (sliding) | `A/components/RadioGroup.kt:78` |
| Item text | `textColor` when selected or hovered, else `textColorSecondary`; 6 dp icon gap; optional 16 dp icon | `A/components/RadioGroup.kt:120-150` |
| Animation | indicator slides with the shared sliding-indicator spec (150 ms) | `A/components/RadioGroup.kt:62` |

#### 6.5.10 Info / notice row

| Property | Value | Citation |
|---|---|---|
| Layout | full width row, icon 24 dp + **10 dp** gap + text column (2 dp apart), top-aligned, 16 dp h padding, 12 dp v | `I/ui/components/settings/InfoOption.kt:29-56`; `I/ui/screens/ConfigScreen.kt:1298-1303` |
| Icon + colour | Info `info`/accent; Success `check-circle`/`success`; Warning `alert-circle`/`warning`; Error `x-circle`/`error` | `I/ui/components/settings/InfoOption.kt:22-27` |
| Heading | `titleSmall` 16 sp (Medium), **lineHeight 19 sp**, `textColor` | `I/ui/components/settings/InfoOption.kt:39-45` |
| Body | 13 sp, **lineHeight 16 sp**, `textColorSecondary` | `I/ui/components/settings/InfoOption.kt:46-54` |

#### 6.5.11 Other visualisers present (for completeness)

| Visualiser | Implementation | Notes |
|---|---|---|
| Range slider | `I/ui/components/settings/RangeOption.kt` | two thumbs, same geometry as slider |
| Inheritable slider | `InheritableSliderOption.kt` | adds an "inherit" chip and two number fields |
| Number (spin) | `NumberOption.kt` | `NumberField` only |
| Text | `TextOption.kt` | `TextField` |
| Dropdown | `DropdownOption.kt` | `Select` |
| Multi-select | `MultiSelectDropdownOption.kt` | `MultiSelect`, summary "a, b" or "N selected" (`SelectDefaults.MAX_NAMED = 2`) |
| Button | `ButtonOption.kt` | `TextButton` |
| Colour list / number list / text list / tile list / file / item / effect | `ColorListOption.kt`, `NumberListOption.kt`, `TextListOption.kt`, `TileListOption.kt`, `FileOption.kt`, `ItemListOption.kt`, `EffectListOption.kt` | list scaffolds in `ListOptionScaffold.kt`; each row is a card with 8 dp gap |

---

## 7. Search

### 7.1 Field behaviour

| Fact | Value | Citation |
|---|---|---|
| Placement | header, always visible on every page | `I/ui/components/Header.kt:169` |
| Size | 256 × 32 dp (`ControlSize.Regular`); minimal mode uses 48 dp tall / 16 sp | `I/ui/components/Header.kt:393-395`; `I/ui/shell/Shell.kt:382-390` |
| Fill / shape | `chipBackground`, 8 dp `field` radius, 1 dp border, glass frame in Glass theme | `A/components/SearchField.kt:154-156`; `A/components/FieldDefaults.kt:43-44` |
| Placeholder | per-page string `"Search <Title>..."`, default `"Search..."`; hidden while focused (`hidePlaceholderWhenFocused` when no explicit placeholder) | `I/ui/navigation/Pages.kt:271-277`; `I/ui/components/Header.kt:398` |
| Leading icon | `search`, 16 dp | `A/components/SearchField.kt:144` |
| Clear button | `close`, 22 dp hit box (compact 28 dp metrics), appears when non-empty | `A/components/SearchField.kt:21, 116-133` |
| Focus | on open, `instantSearch = true` (default) selects all text; on resume keeps focus | `E/OneConfigConfig.java:265`; `M/../impls/OneConfigUIScreen.kt:329-336` |
| Typing | every keystroke updates the query; while the Mods page is active and the query is non-blank the **whole page area is replaced** by the search results screen (header gap becomes 0) | `I/ui/shell/Shell.kt:316-334` |
| Local page search | inside a mod config page, the same global query also filters that page locally (categories → subcategories → options) | `I/ui/screens/ConfigScreen.kt:216, 240-273, 562-570` |
| Shortcut | search keybind focuses the field; Esc clears/leaves (shell behaviour) | `A/components/EscapeCloses.kt`; `I/ui/shell/Shell.kt` |
| Typing into the field does **not** take focus away when clicking options (`keepSearchFocus`) | | `I/ui/components/Common.kt:30-61` |

Search matching: case-insensitive substring across title, id, description, category and subcategory (`I/ui/screens/ConfigScreen.kt:562-570`); corpus search groups by mod and option (`I/ui/search/*`).

### 7.2 Results layout

| Element | Value | Citation |
|---|---|---|
| Container | full page area, `LazyColumn`, 25 dp horizontal padding, **8 dp** vertical gap | `I/ui/screens/SearchResultsScreen.kt:115-121` |
| "MODS" section header | `SectionHeader` (eyebrow 11 sp Medium), `textColorSecondary`, not uppercased by style here (label already "MODS") | `I/ui/screens/SearchResultsScreen.kt:124-134` |
| Mod results | same cards as the Mods page, laid out in **4 columns** with 19 dp gaps, extra 11 dp bottom gap between rows (19 − 8) | `I/ui/screens/SearchResultsScreen.kt:61-65, 137-153` |
| Option group header | mod title (or "mod / group"), `SectionHeader` eyebrow, `textColorSecondary`, 4 dp top padding | `I/ui/screens/SearchResultsScreen.kt:156-165` |
| Option rows | exactly the setting rows from §6.3 (full-width cards), with the owning mod's lock state applied (locked rows at α 0.45 and interaction blocked) | `I/ui/screens/SearchResultsScreen.kt:166-191` |
| Empty / loading | centred `EmptyState`: "Searching..." (spinner) then `No results for "<q>"` with `search` icon | `I/ui/screens/SearchResultsScreen.kt:102-113` |
| Reveal animation (off by default, `animateSearchResults = false`) | 520 ms `EaseOutCubic`, stagger 45 ms per item capped at 360 ms, rise 14 dp, scale 0.94; 260 ms when revealed by scrolling | `I/ui/screens/SearchResultsScreen.kt:199-229`; `E:OneConfigConfig.java:272` |
| Minimal (spotlight) results | 2 columns, 440 dp tall box, 12 dp top gap | `I/ui/shell/Shell.kt:392-396` |

---

## 8. HUD editor and default HUD style

### 8.1 Editor canvas

| Fact | Value | Citation |
|---|---|---|
| Coordinate space | Minecraft GUI space × `mcToScreenScale = guiScale / surfaceRatio` | `U/ScreenPlatform.java:71-73`; `I/ui/hud/screens/HudDesignStudio.kt:2703` |
| Background | the live game (or blurred game) + optional vanilla-layout ghosts | `I/ui/hud/screens/HudVanillaGhosts.kt`; `M/../impls/OneConfigUIScreen.kt:101` |
| Optional grid | every **8 GUI px**, white α **0.07**, 1 px, skipped when a cell < 2 px | `I/ui/hud/screens/HudSnapping.kt:12`; `I/ui/hud/screens/HudMeasure.kt:32, 221-235` |
| Grid snapping | positions snap to multiples of **8** when grid mode is on | `I/ui/hud/screens/HudSnapping.kt:313-315` |
| Snap distance | **6 Minecraft GUI px** (`SNAP_DISTANCE_PX = 6f` × `screenToMcScale`); holding **Alt** disables snapping for the drag | `I/ui/hud/screens/HudDesignStudio.kt:313, 2292-2301`; `HudSnapping.kt:132-164` |
| Snap lines available | screen left/centre/right, top/middle/bottom, screen padding lines at **4 px** from each edge, and for every other HUD: its low edge, centre and high edge, per axis | `I/ui/hud/screens/HudSnapping.kt:89-106, 11` |
| Gap snapping | equal-gap snapping against existing gaps > **2 px** (`HUD_MERGE_TOUCH`), centre-between-neighbours within `free/2` | `I/ui/hud/screens/HudSnapping.kt:13, 108-158` |
| Gap equality tolerance | 0.05 px; alignment tolerance 0.01 px | `I/ui/hud/screens/HudSnapping.kt:15-16` |
| Snap guide appearance | **1 px** line, accent `#FF2B4BFF`, spanning the union of the guide and the dragged rect | `I/ui/hud/screens/HudMeasure.kt:196-208` |
| Gap/side-gap measure lines | 1 px accent, 2 px round caps at both ends, dimension badge in the middle | `I/ui/hud/screens/HudMeasure.kt:155-171, 215-218` |
| Side gap badges | dashed 1 px lines (3 on/3 off) `onMedia` α 0.45, badge fill `pageSurface` α 0.9, text `textColor` | `I/ui/hud/screens/HudMeasure.kt:28-31, 199-214` |
| Dimension badge | height **16 px**, horizontal padding 5 px, corner radius 3 dp (theme-shape-scaled), text **10 px** Poppins/MC, "123 × 45" or "12.5" | `I/ui/hud/screens/HudMeasure.kt:36-38, 133-153` |
| Badge colours | accent fill + `accentTextOn(accent)` text (white for default theme) | `I/ui/hud/screens/HudMeasure.kt:173-176, 1091-1100` |

### 8.2 HUD boxes, selection, handles

| Element | Value | Citation |
|---|---|---|
| Idle HUD box | `onMedia` (white) α **0.2**, 1 px | `I/ui/hud/screens/HudDesignStudio.kt:272, 297` |
| Hovered HUD box | white α **0.5**, 1 px | `HudDesignStudio.kt:273, 298` |
| Entered-group member box | white α **0.35** | `HudDesignStudio.kt:266, 299` |
| Hidden HUD box | white α **0.4**, dashed 4 on / 3 off | `HudDesignStudio.kt:274, 262` |
| Locked HUD box | `warning` `#FFFFAB1D`, dashed 2 on / 4 off | `HudDesignStudio.kt:270, 301` |
| Group hover box | dashed 5 on / 3 off around the whole group | `HudDesignStudio.kt:267` |
| Selected box | accent `#FF2B4BFF`, **1 px** outline | `HudDesignStudio.kt:1064-1069, 307` |
| Dragged HUD fill | accent α **0.10** | `HudDesignStudio.kt:259, 2745-2750` |
| Marquee (rubber-band) | accent fill α **0.15** + **1 dp** accent border | `HudDesignStudio.kt:260, 2959-2970` |
| Resize handles | 4 corner circles radius **3.5 px**: fill `onMedia` white, stroke accent 1.5 px (if a rounded theme) or 7×7 px squares, 1 px accent stroke (Pixel theme) | `HudDesignStudio.kt:251, 1071-1089` |
| Handle hit reach | 4 px outside the box | `HudDesignStudio.kt:255, 2511-2514` |
| Size badge above the selection | centred, 9 px above the top edge, "W × H" (integer GUI px) | `HudDesignStudio.kt:250-253, 1091-1100` |
| External-placement badge | below the box, `locked` fill + `onLocked` text | `HudDesignStudio.kt:1112-1121, 2739-2741` |
| Anchor line (when picking an anchor) | `info` `#FF5B9BE3` | `HudDesignStudio.kt:304` |

### 8.3 Side panels (settings + library)

| Panel | Numbers | Citation |
|---|---|---|
| Settings ("design studio") panel | **500 dp** wide, 16 dp outer padding, `studioSurface()` (popup-like), 16 dp gutter | `I/ui/hud/screens/HudDesignStudio.kt:3820-3853, 4008` |
| Panel header | draggable title (grab handle at α 0.65 → 1 on hover), close button, breadcrumb of groups | `HudDesignStudio.kt:3828-3853, 3815` |
| Panel section stack | rows of `HudSettingTarget` + `LabeledControl`; 4–8 dp gaps | `HudDesignStudio.kt:3932-3950`; `hud/screens/sections/*` |
| Library panel ("HUDs") | **401 × 481 dp**, 16 dp padding, vertical gap 12 dp, search field in the header, hint line 11 sp | `HudDesignStudio.kt:4245-4273` |
| Library hint | "Click to add · Drag to place it yourself", 11 sp `textColorSecondary` | `HudDesignStudio.kt:4268-4273` |
| Library cards | grid computed against `maxWidth − 32 dp`, 10 dp scroll metrics | `HudDesignStudio.kt:4274-4283` |
| Side chrome dimming when the library is open | α **0.45** | `HudDesignStudio.kt:322` |
| Add-HUD flow | library card click = add at default position; drag = place at the pointer; a first-run "suggest HUD" state replaces the list | `HudDesignStudio.kt:4241-4242, 4264-4266` |
| Action bar above the selection | icon buttons (`ControlSize.Large` 38 dp), 4 icons + optional delete/group, 2 px icon gap, 4 px padding, placed above (or below) the selection avoiding the size badge | `HudDesignStudio.kt:859-888, 1155-1156` |
| Editor keyboard shortcuts / tour | `HudShortcuts.kt`, `HudTour.kt` | — |

### 8.4 Default HUD element style

`HudStyle` defaults (used when a mod registers a default and as the documented "built-in" style):

| Field | Default | Citation |
|---|---|---|
| font | `Font.Minecraft` | `H/../HudStyle.kt:16` |
| textColor | `#FFFFFFFF` | `H/../HudStyle.kt:17` |
| shadow | **false** | `H/../HudStyle.kt:18` |
| shadowColor | `#40000000` | `H/../HudStyle.kt:19` |
| background | **true** | `H/../HudStyle.kt:20` |
| backgroundColor | **`#80000000`** (black 50 %) | `H/../HudStyle.kt:21`; `H/Hud.kt:1314` |
| border | **false** | `H/../HudStyle.kt:22` |
| borderColor | `#FF000000` | `H/../HudStyle.kt:23` |
| borderSize | **1** | `H/../HudStyle.kt:24` |
| radius | **4** | `H/../HudStyle.kt:25`; `H/Hud.kt:1323` |
| padding (all sides) | **4** in `HudStyle`, but the live `Hud` fields default to **0** | `H/../HudStyle.kt:26`; `H/Hud.kt:1237-1247` |
| per-side padding overrides | `null` (fall back to `padding`) | `H/../HudStyle.kt:36-39` |
| scale / textScale | 1 / 1 | `H/../HudStyle.kt:27-28` |
| textWeight | `Regular` | `H/../HudStyle.kt:29` |
| caseType | 0 (unchanged; 1 upper, 2 lower) | `H/../HudStyle.kt:30`; `H/../HudText.kt:118-122` |
| bold / italic / underline | false | `H/../HudStyle.kt:31-33` |
| shadow offsets | 2, 2 | `H/../HudStyle.kt:34-35` |
| useGuiScale | true | `H/../HudStyle.kt:40` |
| Background rendering | `background(backgroundColor, radius)` then `border(borderColor, borderSize, radius)` — i.e. one filled rounded rect + optional stroke | `H/Hud.kt:1992-1997` |
| Text layout | lines with **2 px** extra gap between lines at scale 1; ascent/descent from the font metrics | `H/../HudText.kt:103, 280-291` |
| Line pitch (Poppins HUD text) | `lineHeight + 2*scale` | `H/../HudText.kt:328` |
| Text alignment inside the box | horizontal factor 0 / 0.5 / 1 (left/centre/right) from `alignment` | `H/../HudText.kt:112-116, 330` |
| Default static size (if made static) | 200 × 48 GUI px | `H/Hud.kt:1249-1253` |

Minecraft-style fallback text drawing uses `ctx.mcText(...)` with a 1-px shadow offset and the bundled Unifont fallback (`H/../HudText.kt:367-376`).

---

## 9. Profiles, notifications/toasts, tooltips, dialogs

### 9.1 Profiles page

| Element | Value | Citation |
|---|---|---|
| Grid | **5 columns**, 19 dp gaps; the page itself is padded 25 dp (gutter) and the grid adds **12 dp** right padding (`spacing.l`) for the scrollbar, no horizontal content padding | `I/ui/screens/Profiles.kt:259, 518-527` |
| Card height | **180 dp** | `I/ui/screens/Profiles.kt:123` |
| Card | `modCardBackground`, 16 dp, 1 dp gradient border, hover glow (Glass only), radial vignette `0 → 0.02 → 0.05` | `I/ui/screens/Profiles.kt:633-667` |
| Create card | 64 dp circle `textColor` α 0.06 (accent α 0.18 when hovered/creating), `plus` 30 dp; 24 dp gap; label `titleMedium` 18 sp Medium at α 0.86; name field 150 dp when creating | `I/ui/screens/Profiles.kt:686-715, 626-631` |
| Header row | title + "profile-specific controls" switch, right-aligned actions in an 8 dp row | `I/ui/screens/Profiles.kt:273-306` |
| Busy overlay | full-width progress bar + explanatory text, blocks the page | `I/ui/screens/Profiles.kt:430-450` |
| Keybind clearance | 12 dp above card content | `I/ui/screens/Profiles.kt:124` |

### 9.2 Toasts (in-UI feedback)

| Property | Value | Citation |
|---|---|---|
| Max visible | **3** | `A/components/Toasts.kt:144` |
| Width / margin / alignment | max **360 dp**, margin **16 dp**, `TopCenter` | `A/components/Toasts.kt:146-150` |
| Stack gap | 8 dp | `A/components/Toasts.kt:193` |
| Card | `popupShape` 16 dp, `popupBackground`, padding start 12 / end 4 / top 4 / bottom 4, gap 10 dp, optional 1 dp border | `A/components/Toasts.kt:235-241, 152` |
| Icon | 16 dp, coloured by type: success/error/warning/info/accent | `A/components/Toasts.kt:243, 249-259` |
| Text | `label` 13 sp, `textColor` | `A/components/Toasts.kt:244` |
| Dismiss button | icon button Compact 28 dp | `A/components/Toasts.kt:245` |
| Default duration | **4000 ms**; `≤0` = persistent; hover pauses the timer | `A/components/Toasts.kt:74, 209-218`; `modules/notifications/.../Notification.kt:280` |
| Enter/exit | fade + vertical expand/shrink from the top (default AnimatedVisibility specs) | `A/components/Toasts.kt:220-226` |

### 9.3 Notification centre (bell popup)

| Property | Value | Citation |
|---|---|---|
| Panel | **368 dp** wide, `themedSurface(Popup)`, padding 14 h / 16 v, sections 16 dp apart, history scroll max **420 dp** | `I/ui/components/NotificationsCenter.kt:58-96` |
| Row | 32 dp icon box (24 dp glyph) + 13 dp gap + text column 2 dp apart; title `body` SemiBold lh 20, time `caption`, unread 6 dp accent dot | `I/ui/components/NotificationsCenter.kt:100-141` |
| Notification theme passed to the OS-style renderer | background = popup α ≥ **0.92**, border `borderColor`, text primary/secondary, accent, card radius from `shapes.large` at 340×72, button radius from `shapes.medium` at 80×26, progress radius from pill at 312×6 | `I/ui/themes/Provider.kt:244-266` |

### 9.4 Tooltips

| Property | Value | Citation |
|---|---|---|
| Max width | **240 dp** (long descriptions 260 dp) | `A/components/HoverTooltip.kt:100-104` |
| Delay | **300 ms** | `A/components/HoverTooltip.kt:102` |
| Gap to anchor / window edge | 8 dp / 8 dp | `A/components/HoverTooltip.kt:49-50` |
| Surface | `popupShape` 16 dp, padding 12 h / 8 v | `A/components/HoverTooltip.kt:171-179` |
| Text | `labelRegular` 13 sp Normal, `textColor`, centred | `A/components/HoverTooltip.kt:147-156` |
| Placement | above-centre default; flips when there is no room; shifts to stay inside the window | `A/components/HoverTooltip.kt:68-95, 106` |
| Transition | **none** (popup appears immediately after the delay) | `A/components/HoverTooltip.kt:171` |
| Rich header tooltip | rows of `small` 12 sp, 4 dp apart, below-start placement | `I/ui/components/Header.kt:291-305` |

### 9.5 Dialogs / confirm / context menus

| Property | Value | Citation |
|---|---|---|
| Popup scale-in | from **0.95** to 1 + fade, **150 ms**, transform origin at the anchor corner | `A/components/Dropdown.kt:189-199, 210-222` |
| Context menu | same menu surface as dropdowns (min 170, max 280 dp, 4 dp padding, 2 dp item gap, 30 dp compact items) | `A/components/Menu.kt:87-114` |
| Confirm dialog | anchored confirm popup (used for sort/reset), primary confirm button, text from `Confirm.ask` | `A/components/Confirm.kt`; `I/ui/screens/Mods.kt:508-517` |
| Selection bar (multi-select) | min height `38 + 2×8`, `popupBackground`, 1 dp border, padding start 16 / end 6 / v 6, 8 dp gaps | `A/components/Selection.kt:300-315` |
| Scrollbar | themed thumb, `textColorSecondary` α 0.40 idle / 0.70 hover, 8 dp thick, min 24 dp, radius = `scrollBarShape` (8 dp default), hover fade 300 ms | `A/components/Scrolling.kt:46-50, 229-243`; `I/ui/themes/Structs.kt:134, 178` |

---

## 10. Animation table

Global switches: `animations = true`, `animationTime = 0.6 s`, `showPageAnimations = true`, `pageAnimationDuration = 0.3 s`, `showOpeningPageAnimation = false`, `reducedMotion = false` (`E/ThemeConfig.java:53-132`). "Reduced motion" replaces every spec with a snap; `Motion.enter/exit` substitute a 150 ms fade (`A/a11y/Motion.kt:28-58`).

| Interaction | Duration | Easing / spring | Notes | Citation |
|---|---|---|---|---|
| Shell **open** | `animationTime × 1000` = **600 ms** (min 1 ms), **not** clamped | **EaseOutExpo** `1 − 2^(−10x)` | alpha 0→1, scale **0.9→1** | `I/ui/OneConfigInterface.kt:55-59, 185-197, 221`; `I/ui/components/RetainedVisibility.kt:137-164` |
| Shell **close** | `animationTime×1000` clamped to **≤160 ms** | **EaseOutCubic** | same alpha/scale reversed | `I/ui/OneConfigInterface.kt:55-59, 187, 197` |
| Opening page animation (first navigation) | same as page transition | 300 ms | disabled by default | `E/ThemeConfig.java:132`; `I/ui/OneConfigInterface.kt:96` |
| **Page transition** (nav) | **pageAnimationDuration = 300 ms** | Compose tween default = **FastOutSlowIn** (cubic-bezier 0.4, 0, 0.2, 1) | incoming slides from **+width/5** (forward) or **−width/5** (back) while fading in; outgoing slides the opposite way and fades out | `I/ui/components/ContentSlide.kt:24-38, 26` |
| Config category/tab switch | 300 ms | same | only the options slide; the header stays; fade only on the leaving content | `I/ui/components/ContentSlide.kt:40-62`; `I/ui/screens/ConfigScreen.kt:246-265` |
| Hover colour changes (nav, chips, borders, text) | **150 ms** | FastOutSlowIn | default `Motion.standard` | `A/a11y/Motion.kt:32, 51-62` |
| Fast colour (colour swatch border) | **120 ms** | FastOutSlowIn | `Motion.fast` | `A/a11y/Motion.kt:30`; `A/components/ColorPicker.kt:260` |
| Pressed feedback | 150 ms | FastOutSlowIn | overlay α 0→0.14 | `A/theme/OneConfigUITheme.kt:79` |
| Popup open/close | **150 ms** | FastOutSlowIn | fade + scale 0.95→1 at the anchor corner | `A/components/Dropdown.kt:189-199` |
| Tooltip | 300 ms **delay** | none | | `A/components/HoverTooltip.kt:102` |
| Switch accent | 150 ms | FastOutSlowIn | | `A/components/Switch.kt:123` |
| Switch thumb | spring() default | spring | | `A/components/Switch.kt:124` |
| Checkbox accent / tick | 150 ms | FastOutSlowIn | | `A/components/Checkbox.kt:92, 116-120` |
| Slider thumb | spring() default | spring | value changes eased, not linear | `A/components/Slider.kt:139` |
| Slider muted blend | 150 ms | FastOutSlowIn | | `A/components/Slider.kt:140` |
| Nav/row selection tint | 150 ms | FastOutSlowIn | accent α 0→1 (or 0→0.2) | `A/components/Selection.kt:193, 221, 242` |
| Accordion fold | `spring(StiffnessMediumLow, IntSize.VisibilityThreshold)` | spring | expand/shrink vertically | `I/ui/screens/ConfigScreen.kt:695-703` |
| Chevron rotation | 150 ms | FastOutSlowIn | 90° → 180° | `A/components/LayoutPieces.kt:353` |
| Card reorder settle | spring(StiffnessMediumLow) | spring | also used for lazy-list placement | `A/a11y/Motion.kt:56-58` |
| Toast enter/exit | default visibility specs | spring expand + tween fade | from top | `A/components/Toasts.kt:220-226` |
| Search result reveal (opt-in) | 520 ms / 260 ms on scroll | **EaseOutCubic** | stagger 45 ms, cap 360 ms, rise 14 dp, scale 0.94 | `I/ui/screens/SearchResultsScreen.kt:199-229` |
| Jump-to-option highlight | 150 ms in, hold 700 ms, 900 ms out | tween | accent fill α 0.14 + 1.5 dp stroke | `I/ui/screens/ConfigScreen.kt:374-394` |
| Screen blur fade | 250 ms | linear | alpha 0→1 | `M/../platform/OneConfigComposeScreen.kt:152` |
| Glass press bloom | 520 ms | **LinearOutSlowIn** | radial white α 0.2·(1−b) (dark) | `I/ui/components/GlassHover.kt:81, 49` |
| Glass magnetic hover | spring(StiffnessMediumLow) | spring | pull 0.75–1 dp | `I/ui/components/GlassHover.kt:66-74` |
| Glow blob drift | 9–31 s periods | sine | breath 0.18–0.30 | `I/ui/shell/Shell.kt:115-119` |
| HUD box hover | 150 ms (`animateFloatAsState` default) | FastOutSlowIn | | `I/ui/hud/screens/HudDesignStudio.kt` (`Motion.animateFloatAsState` usages) |
| HUD editor panel resize | spring(StiffnessMediumLow) | spring | | `HudDesignStudio.kt:258` |
| HUD nudge settle | 300 ms | — | | `HudDesignStudio.kt:256` |

**Implementation note:** Compose's `tween` default easing is `FastOutSlowInEasing` = cubic-bezier(0.4, 0.0, 0.2, 1.0). Every `Motion.tweenSpec()` call therefore uses that curve. `spring()` with no arguments is Compose's `Spring.DampingRatioNoBouncy` (1.0) / `StiffnessMedium` (1500).

---

## 11. Asset inventory

### 11.1 Icons (SVG, `modules/ui/src/main/resources/assets/oneconfig/ico/`)

All icons are single-colour SVGs tinted at draw time (accent/text colour), except Minecraft-theme overrides in `ico/minecraft/`.

| Icon file | Used by | Citation |
|---|---|---|
| `alert-circle.svg` | warning/info rows, "mod not found" empty state | `I/ui/components/settings/InfoOption.kt:25`; `I/ui/screens/ModConfig.kt:43` |
| `align.svg`, `align-center.svg`, `align-left.svg`, `align-right.svg` | HUD alignment picker (used by name) | `I/ui/hud/components/AlignmentPicker.kt`; `I/ui/hud/screens/sections/Layout.kt:181-199` |
| `align-top.svg`, `align-middle.svg`, `align-bottom.svg` | shipped, **not referenced by literal name** in the clone — spares | `modules/ui/src/main/resources/assets/oneconfig/ico/` (files only) |
| `bell.svg` | sidebar notification bell | `I/ui/components/Sidebar.kt:308` |
| `bold.svg`, `italic.svg`, `underline.svg`, `strikethrough.svg`, `uppercase.svg`, `lowercase.svg`, `capitalized.svg` | HUD text-style toggles | `I/ui/hud/screens/sections/Style.kt` |
| `check-circle.svg` | mod menu "Enable", success info rows | `I/ui/screens/Mods.kt:478`; `I/ui/components/settings/InfoOption.kt:24` |
| `clipboard.svg` | colour menu "Paste hex" | `A/components/ColorPicker.kt:349` |
| `close.svg` | header close, search clear, toast dismiss, tooltips/panels | `I/ui/components/Header.kt:170`; `A/components/SearchField.kt:119` |
| `cloud.svg` | profile cloud/sync action, theme import action | `I/ui/screens/Profiles.kt:138`; `I/ui/screens/Themes.kt:389` |
| `cog.svg` | Preferences nav entry | `I/ui/navigation/Pages.kt:77` |
| `combat.svg` | profile card glyph | `I/ui/screens/Profiles.kt:136` |
| `console.svg` | no built-in use; only in the Minecraft override map | `I/ui/themes/Impls.kt:26` |
| `copy.svg` | "Copy mod id", "Copy hex" | `I/ui/screens/Mods.kt:500`; `A/components/ColorPicker.kt:344` |
| `copy-plus.svg` | HUD duplicate, custom-theme duplicate | `I/ui/hud/HudSettingsReset.kt:517`; `I/ui/screens/CustomThemeEditor.kt:192` |
| `dots-grid.svg` | custom sort mode | `I/ui/screens/ModSortControl.kt:25` |
| `dots-vertical.svg` | "more actions" affordance | `A/components/MoreButton.kt:58` |
| `down.svg` | number field decrement | `A/components/NumberField.kt:308` |
| `external-link.svg` | "Original menu", external open | `I/ui/components/Header.kt:256`; `I/ui/screens/ModConfig.kt:57` |
| `eye.svg` / `eye-off.svg` | show/hide mod, Hidden category | `I/ui/screens/Mods.kt:462, 868` |
| `folder.svg` | "open folder" actions (profiles export, config folder) | `I/ui/screens/Profiles.kt:304`; `I/ui/components/Button.kt:7` |
| `group.svg` / `ungroup.svg` | HUD grouping | `I/ui/hud/screens/HudDesignStudio.kt:1147-1152` |
| `hud.svg` | "Edit HUD" sidebar + card menu | `I/ui/components/Sidebar.kt:203`; `I/ui/screens/Mods.kt:487` |
| `hypixel.svg` | Hypixel category | `I/ui/screens/Mods.kt:160` |
| `info.svg` | info icon / About button / header ⓘ | `I/ui/components/Sidebar.kt:392`; `I/ui/components/Header.kt:311` |
| `keyboard.svg` | Keybinds page + keybind field | `I/ui/navigation/Pages.kt:75`; `A/components/KeybindField.kt:197` |
| `layers.svg` | HUD layers/merge action | `I/ui/hud/HudSettingsReset.kt:473-479` |
| `left-arrow.svg` / `right-arrow.svg` | header back/forward | `I/ui/components/Header.kt:95, 104` |
| `lightning-01.svg` | Performance category | `I/ui/screens/Mods.kt:161` |
| `lock.svg` / `unlock.svg` | locked mod toggle, HUD lock | `I/ui/screens/Mods.kt:785`; `I/ui/hud/HudSettingsReset.kt:444` |
| `maximise.svg` | no built-in use; only in the Minecraft override map | `I/ui/themes/Impls.kt:47` |
| `minus.svg` | HUD "less" action, keymap zoom-out | `I/ui/hud/HudSettingsReset.kt:496`; `I/ui/screens/KeybindKeymap.kt:203` |
| `moon.svg` / `sun.svg` | dark / light theme pickers | `I/ui/screens/Themes.kt:383`; `I/ui/screens/CustomThemeEditor.kt:480` |
| `move.svg` | HUD "move to anchor" action | `I/ui/hud/HudSettingsReset.kt:531` |
| `pad-left/right/top/bottom.svg` | HUD per-side padding section | `I/ui/hud/screens/sections/Layout.kt:209-258` |
| `paintbrush.svg` | accent colour setting, Visuals category, Themes page | `I/ui/screens/Mods.kt:162`; `E/ThemeConfig.java:50` |
| `plus.svg` | new profile | `I/ui/screens/Profiles.kt:696` |
| `profiles.svg` | Profiles page | `I/ui/navigation/Pages.kt:74` |
| `qol.svg` | Quality-of-Life category | `I/ui/screens/Mods.kt:165` |
| `redo.svg` / `undo.svg` | HUD editor history | `I/ui/hud/HudSettingsReset.kt:596-606` |
| `refresh.svg` | Changelog page, "reset", animations setting | `I/ui/navigation/Pages.kt:78`; `I/ui/screens/Mods.kt:494`; `E/ThemeConfig.java:53` |
| `right-arrow.svg` | header forward | `I/ui/components/Header.kt:104` |
| `scissors.svg` | HUD scissors/cut action | `I/ui/hud/HudSettingsReset.kt:510` |
| `search.svg` | search field leading icon, empty search | `A/components/SearchField.kt:144` |
| `select-all.svg` | HUD multi-select action | `I/ui/hud/HudSettingsReset.kt:573` |
| `settings.svg` | Mods page, "Open settings" | `I/ui/navigation/Pages.kt:73`; `I/ui/screens/Mods.kt:446` |
| `shuffle.svg` | no built-in use; only in the Minecraft override map | `I/ui/themes/Impls.kt:62` |
| `spanner.svg` | no built-in use; only in the Minecraft override map | `I/ui/themes/Impls.kt:63` |
| `star.svg` / `star-filled.svg` | favourites, Credits page | `I/ui/screens/Mods.kt:455, 846`; `I/ui/navigation/Pages.kt:79` |
| `text-input.svg` | profile rename field leading icon | `I/ui/screens/Profiles.kt:760` |
| `text.svg` | shipped; not referenced by literal name in built-in UI | `modules/ui/src/main/resources/assets/oneconfig/ico/` (file only) |
| `tick.svg` | checkbox tick, "create profile" | `A/components/Checkbox.kt:121`; `I/ui/screens/Profiles.kt:671` |
| `trash.svg` | delete (notification row, list rows) | `I/ui/components/NotificationsCenter.kt:189`; `I/ui/components/Button.kt:8` |
| `up.svg` | dropdown chevron, section chevron, number increment | `A/components/Select.kt:330`; `A/components/LayoutPieces.kt:355`; `A/components/NumberField.kt:305` |
| `upload.svg` | import action | `modules/ui/src/main/resources/assets/oneconfig/ico/` (file; import flows in `I/ui/screens/Themes.kt`) |
| `warning.svg` | only in the Minecraft override map; built-in warnings use `alert-circle` | `I/ui/themes/Impls.kt:77`; `modules/notifications/.../Notification.kt:47` |
| `x-circle.svg` | mod menu "Disable", error rows | `I/ui/screens/Mods.kt:478`; `I/ui/components/settings/InfoOption.kt:26` |

Theme overrides: the whole set is remapped to `ico/minecraft/*.svg` for Pixel themes via `MinecraftIconOverrides` (`I/ui/themes/Impls.kt:11-79`), plus `preferences.svg` exists only in the Minecraft folder.

Other image assets:

| Asset | Use | Citation |
|---|---|---|
| `brand/oneconfig.svg`, `-white`, `-light`, `-mc`, `-mc-light`, `oneconfig-icon.svg` | sidebar logo per theme (drawn 167 × 19 dp, accent-tinted) | `I/ui/themes/Impls.kt:110, 154, 197, 235, 274, 307`; `I/ui/components/Sidebar.kt:189-196` |
| `brand/polyfrost.png`, `brand/atmofrost.svg`, `brand/by-polyfrost.svg` | Credits page | `I/ui/screens/Credits.kt` |
| `ico/default_mod.svg` | fallback mod icon | `I/ui/api/ConfigData.kt` |
| `images/polyfrosted-dark.svg` etc. (12 files) | theme preview thumbnails | `I/ui/themes/Impls.kt:82` (`previewImage` = file stem) |
| `images/head.png` (internal assets) | fallback player head | `I/ui/components/PlayerHead.kt` |
| `color/hue.png`, `color/alpha.png` | legacy colour-slider strips (the current picker draws gradients instead) | `assets/oneconfig/color/` |
| `hud/align/background.svg`, `hud/align/circle.svg`, `hud/selector_curve.svg` | HUD alignment picker background/curve | `modules/hud/src/main/resources/assets/oneconfig/hud/` |

### 11.2 Fonts

| File | Size/weight | Use |
|---|---|---|
| `fonts/Poppins/Poppins-Regular.ttf` | 400 | body text |
| `Poppins-Medium.ttf` | 500 | labels, titles |
| `Poppins-SemiBold.ttf` | 600 | headings, display |
| `Poppins-Light.ttf` | 300 | registered, unused by the default styles |
| `Poppins-ExtraLight.ttf` | 200 | registered |
| `Poppins-Bold.ttf` / `-Black.ttf` / `-ExtraBold.ttf` | 700/900/800 | HUD "bold"/"black" text weights |
| `Poppins-Thin.ttf` | 100 | HUD "thin" |
| `Poppins-*-Italic.ttf` (9 files) | — | **not registered** in the theme family |
| `fonts/minecraft/Minecraft-Bold.otf`, `Minecraft-Regular.otf` | — | Pixel themes |
| `fonts/unifont/unifont.otf` | — | glyph fallback (CJK etc.) |

### 11.3 Licences

| Asset group | Licence | Evidence |
|---|---|---|
| OneConfig source + bundled assets | **LGPL-3.0 + Polyfrost Additional Terms v1.1** | `LICENSE:1-12` |
| `unifont.otf` | **SIL OFL 1.1** (dual-licensed OFL / GPL-2+ with font exception; OneConfig redistributes under OFL) | `modules/ui/src/main/resources/assets/oneconfig/fonts/unifont/LICENSE.txt:1-14` |
| Poppins | **No licence file ships in this clone.** Poppins upstream is OFL-1.1, but nothing in the repository confirms it. | searched all `*LICENSE*`/`*OFL*` files — only the two above exist |
| Minecraft font (`Minecraft-*.otf`) | **UNKNOWN (not in clone)** — no licence file, no attribution | searched |
| SVG icon set | covered by the repository `LICENSE` (LGPL + additional terms); no per-icon licence | `LICENSE` |

If you redistribute, LGPL-3.0 + the additional terms apply to OneConfig-derived code and assets; Unifont is OFL. **Flag Poppins and the Minecraft font as "licence unknown in this clone" to whoever ships the Rust port** — do not assume OFL without checking upstream.

---

## 12. Things that are external / not in the clone

| Feature | Where it lives | What the clone tells us |
|---|---|---|
| Compose→Minecraft host, density, blur pipeline, `ComposeScreen`, `polyGlass`, `polyGlassSplit`, `polyGlassGlow`, `animatedFrame`, `Popup`, `ColorPickerPanel`, scrollbar style | `org.polyfrost.compose.*` (dependency) | Only call sites and the values passed in. E.g. `densityScale() = uiScale` (`M/../platform/OneConfigComposeScreen.kt:94`); glass tint/rim colours (§3.4); popup anchor math is re-implemented in `AnchoredPopup`/`RetainedVisibility.kt:63-134`; **blur radius, rim width, popup shadow: UNKNOWN (not in clone)** |
| Focus-ring ring width | bridge | UNKNOWN (not in clone); colour is `focusRingColor` (`A/theme/OneConfigUITheme.kt:73`) |
| Sound design (click/open/close/slider tick) | `modules/notifications`, `assets/sounds` | Optional for the Rust port; events are enumerated in `I/ui/sound/UiSoundEvent.kt` |
| GPU-side liquid glass sampling | renderer | Glass theme will need a real backdrop blur; Flat (default) needs none |

---

## 13. Ten most important files (in reading order)

1. `modules/internal/src/main/kotlin/org/polyfrost/oneconfig/internal/ui/themes/Impls.kt` — every theme's literal palette, shapes, fonts, flags (the single most load-bearing file for visuals).
2. `modules/internal/src/main/kotlin/org/polyfrost/oneconfig/internal/ui/themes/Structs.kt` — theme field order, style resolution, control-track default, glass/inset/translucent flags.
3. `modules/ui/src/main/kotlin/org/polyfrost/oneconfig/api/ui/v1/theme/ThemeTokens.kt` — `ThemeStyle`, the full typography ramp, spacing scale.
4. `modules/internal/src/main/kotlin/org/polyfrost/oneconfig/internal/ui/shell/Shell.kt` — window size/scale, shadow, glow field, sidebar/content split, page gutter and 19 dp rhythm.
5. `modules/internal/src/main/kotlin/org/polyfrost/oneconfig/internal/ui/components/Sidebar.kt` + `.../api/ui/v1/components/NavItem.kt` — sidebar geometry, entry height/width, selected accent pill.
6. `modules/internal/src/main/kotlin/org/polyfrost/oneconfig/internal/ui/screens/Mods.kt` — mods grid (4 × 255 × 140, 19 dp gaps), card anatomy, toggle ring, footer states.
7. `modules/internal/src/main/kotlin/org/polyfrost/oneconfig/internal/ui/screens/ConfigScreen.kt` — option card layout, 16 dp side padding, 12 dp vertical, 2-column accordions, all row measurements.
8. `modules/ui/src/main/kotlin/org/polyfrost/oneconfig/api/ui/v1/components/` — the control kit: `Switch.kt`, `Checkbox.kt`, `Slider.kt`, `NumberField.kt`, `Select.kt`, `Menu.kt`, `TextField.kt`, `TextButton.kt`, `FieldDefaults.kt`, `ControlTypes.kt`.
9. `modules/internal/src/main/kotlin/org/polyfrost/oneconfig/internal/ui/hud/screens/HudDesignStudio.kt` (plus `HudSnapping.kt`, `HudMeasure.kt`) — selection colours, handles, snap thresholds, grid, badges, side panels.
10. `modules/hud/src/main/kotlin/org/polyfrost/oneconfig/api/hud/v1/HudStyle.kt` + `HudText.kt` + `Hud.kt:1992-1997` — default HUD background/text/border/padding/radius and how the background rect is drawn.
