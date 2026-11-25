# Polyfrost Mod Inventory — Batch B

Research compiled 2026-11-25. All repos cloned from `https://github.com/Polyfrost/<name>` at `--depth 1` (default branch, unless noted). Line numbers refer to the config class as cloned. "OneConfig" below means Polyfrost's OneConfig v1 API (`org.polyfrost.oneconfig`), "OC-legacy" means the old `cc.polyfrost.oneconfig` API used by 1.8.9 Forge mods.

---

## Chatting

- URL: https://github.com/Polyfrost/Chatting · archived: no · fork: no
- Branch read: `main` · last commit: 2026-11-20 (a9390ca)
- Targets: 1.8.9 (Ornithe/Fabric) + 1.21.1/1.21.4/1.21.5/1.21.8/1.21.10/1.21.11/26.1/26.2/26.3 (stonecutter). Fabric + Ornithe loaders.
- Licence: GPL-3.0 · Dependencies: OneConfig 1.8.0, PolyCompose, Fabric Language Kotlin.
- Config: `ChattingConfig.kt` (object, `chatting.json`). Mixins: 18 (`ChatComponent`, `ChatScreen`, `GuiMessage.Line`, `GuiRenderer`, `MouseHandler`, `Font`, etc.) plus TextTunnels/ChatHeads compat mixins.

Feature list (all options in `ChattingConfig.kt`; default value after `=`):

- General: `modEnabled = true` (global toggle, L28); `textRenderType = 1` dropdown "No Shadow"/"Shadow" (L34); `chatBackgroundColor` = black 255 (L41); `separateUnfocusedBackground = false` (L47); `unfocusedChatBackgroundColor` black (L53); `hoveredChatBackgroundColor` = rgba(80,80,80,128) (L59); `roundedChatCorners = false` (L65); `chatCornerRadius = 6f` slider 1–16 (L73); `fade = true` message fade (L79); `fadeTime = 10f` slider 0–20 (L87); `underlinedLinks = false` (L93).
- Chat Window: `customChatWidth = false` (L105); `chatWidth = 320` slider 40–2160 (L113); `customChatHeight = false` (L120); `focusedChatHeight = 180` (L128); `unfocusedChatHeight = 90` (L136).
- Chat Peek: `chatPeek = true` (L143); `peekScrolling = true` (L149); `chatPeekBind` keybind (L154); `peekMode = 0` radio "Held"/"Toggle" (L160).
- Animations: `smoothChat = true` (L167); `smoothChatMs = 500f` (L176); `smoothScrolling = true` (L183); `smoothScrollingMs = 150f` (L192); `removeScrollBar = true` (L199).
- Buttons: `chatButtonColor` white (L206); `chatButtonHoveredColor` rgba(255,255,160) (L212); `chatButtonBackgroundColor` rgba(0,0,0,128) (L218); `chatButtonHoveredBackgroundColor` rgba(255,255,255,128) (L224); `buttonShadow = true` (L230); `extendBG = true` (L236); `chatCopy = true` (L242); `rightClickCopy = false` (L249); `rightClickCopyCtrl = true` (L256); `chatDelete = true` (L262); `chatDeleteHistory = true` (L269); `chatScreenshot = true` (L276); `chatSearch = true` (L283).
- Chat Heads: `showChatHeads = true` (L290); `improvedHeads = true` (L297); `chatHeadShadow = 0` dropdown Off/Normal/Legacy (L303); `centerChatHeads = false` (L309); `offsetNonPlayerMessages = false` (L315); `hideChatHeadOnConsecutiveMessages = true` (L321).
- Screenshotting: `copyMode = 2` dropdown "Save To System"/"Add To Clipboard"/"Both" (L327); `screenshotBorder = true` (L333); `screenshotForceShadow = false` (L339); `screenshotBackground = false` (L345).
- Tabs: `chatTabs = true` (L351); `hypixelOnlyChatTabs = true` (L361).
- Shortcuts: `chatShortcuts = false` (L366); `hypixelOnlyChatShortcuts = true` (L375); "Edit Chat Shortcuts" button (L382).

Behaviour: full chat HUD replacement — custom dimensions, per-message fade, smooth entry/scroll animations, hover backgrounds, chat buttons (copy/delete/screenshot/search), chat heads (avatar rendering with optional 3D hat layer and shadow), chat tabs (filter messages), chat screenshots to disk/clipboard. Mixins target the chat component/screen to redraw message backgrounds and lines. Chat shortcuts are user-defined text aliases expanded to commands/messages (see fair-play).

Legacy note: works identically on 1.8.9 and modern; the 1.8.9 build uses `net.minecraft.client.gui.components.ChatComponent` equivalents via stonecutter remapping. Not version-specific.

Fair-play: **Chat Shortcuts** can send arbitrary chat messages/commands via typed aliases (hypixel-only by default). No combat advantage (client-side rendering/UX only).

Summary: an extremely feature-rich chat-overhaul mod (heads, tabs, animations, screenshots, shortcuts) replacing the vanilla chat rendering stack.

---

## ChatTweaks

- URL: https://github.com/Polyfrost/ChatTweaks · archived: no · fork: no
- Branch read: `master` · last commit: 2026-11-20 (00db20d)
- Targets: 1.8.9 + 1.21.x/26.x (stonecutter, same set as Chatting). Fabric + Ornithe.
- Licence: CC-BY-NC-SA-4.0 (NOASSERTION in API, LICENSE file is CC BY-NC-SA 4.0) · Deps: OneConfig 1.8.0. Mixins: 16 (`ChatComponent`, `GuiMessage`, `ChatScreen`, `EditBox`, `Hud`, `LiteralCommandNode`, `Screen`, `Style`, `MutableComponent`, `ComponentRenderUtils`).
- Config: `ChatTweaksConfig.java` (loads from `patcher.toml` — split from Patcher).

Options (default = `...`):

- `modEnabled = true` (L35).
- Cleanup: `removeBlankMessages = false` (L42).
- History: `increaseChatHistoryLimit = 1000` slider 100–16384 (L51); `dontClearChatHistory = true` (L58); `saveChatHistory = true` (L65); `saveChatHistoryInterval = 30` slider 5–60 (L73); `sessionMarkers = true` (L80).
- Compact Chat: `compactChat = true` (L87); `consecutiveCompactChat = false` (L94); `dontCompactScreenshots = true` (L101); `compactChatTime = 60` slider 1–120 (L109); `compactChatFormat = 0` dropdown 10 bracket styles (L117); `compactChatColor` = 0xAAAAAA (L125).
- Timestamps: `timestamps = false` (L133); `timestampsFormat = 0` 12/24h (L141); `timestampsStyle = 0` Always/Hover (L149); `timestampsBrackets = 0` (L157); `timestampsLeftBracket = "["` (L165); `timestampsRightBracket = "]"` (L173); `timestampsColor` 0xAAAAAA (L181); `onlyNewTimestamps = false` (L188); `secondsOnTimestamps = false` (L195).
- QOL: `shiftChat = false` (L203).
- Commands: `bypassCommandLimit = true` (L210); `fuzzyAutocomplete = false` (modern only, L218).
- Safe Chat Clicks: `safeChatClicks = false` (L226); `safeChatClicksHistory = false` (L233).
- Image Preview: `imagePreview = true` (L241); `imagePreviewWidth = 25F` slider 5–50 (L249).

Behaviour: chat history extension/persistence (per-server disk save), duplicate-message compaction with configurable counter format, timestamps with custom brackets, safe chat clicks (preview the command/link before running), image link previews, command length bypass. Mixins alter `ChatComponent` message queueing and `GuiMessage`/`Screen` rendering.

Legacy note: most features are version-agnostic chat tweaks; `fuzzyAutocomplete` is modern-only (`//? if > 1.8.9`). Not 1.8.9-specific by nature.

Fair-play: `bypassCommandLimit` lets commands exceed vanilla 256-char limit (a chat protocol relaxation, on 1.8.9). No combat advantage.

Summary: the "utility" companion to Chatting — chat history, compaction, timestamps, safe clicks, image previews (a clean split-out of old Patcher chat features).

---

## AutoText

- URL: https://github.com/Polyfrost/AutoText · archived: no · fork: no
- Branch read: `main` · last commit: 2026-11-21 (53a7bf0)
- Targets: 1.8.9 + 1.21.4/1.21.5/1.21.8/1.21.10/26.x (stonecutter). Fabric + Ornithe.
- Licence: AGPL-3.0 · Deps: OneConfig 1.9.0. Mixins: none (event/keybind based).
- Config: `AutoTextConfig.kt` (object). Only two persisted user options: `enabled = true` (switch, L22) and `macrosJson` (hidden JSON blob, L25). The actual macro list is edited via a GUI and stored as JSON (`MacroCodec`): each `MacroDefinition` has `id`, `text` (≤1024 chars, L276), `enabled`, `binding` (up to 16 keyboard keys 1–512, up to 16 mouse buttons 0–31, 4 modifier bits; L277–280) and a `delivery` field (chat message vs command). Max 100 macros (`MAX_MACROS`, L33).

Behaviour: press a bound key/mouse button to instantly send a saved chat message or run a saved command. Deliberately designed as a hotkey macro system, not a chat filter.

Fair-play: **explicitly sends automated chat messages/commands on keypress** — a macro/auto-command tool (e.g. auto-GG, auto-reply, `/play` macros). This is the mod's entire purpose; purely user-configured.

Summary: a keybind macro mod that sends pre-saved chat messages or commands on demand.

---

## Hytils-Reborn

- URL: https://github.com/Polyfrost/Hytils-Reborn · **archived: yes** · fork: no
- Branch read: `main` · last commit: 2026-11-15 (b66caa4)
- Targets: 1.21.4/1.21.5/1.21.8/1.21.10/1.21.11/26.1 (stonecutter, modern Fabric only; **no 1.8.9**).
- Licence: GPL-3.0 · Deps: OneConfig 1.6.2+post.1. Mixins: 31 (`ChatComponent`, `ClientPacketListener`, `BossHealthOverlay`, `PlayerTabOverlay`, `ParticleEngine`, `SoundEngine`, `Gui`, `Hud`, `FramerateLimitTracker`, `BlockRenderer`, `EntityRenderDispatcher`, `HumanoidMobRenderer`, render-state classes).
- Config: `HytilsRebornConfig.kt` (Category.HYPIXEL). ~150 options. Highlights (defaults):

Automatic (General): `isEnabled = true` (L26); `autoStart = false` (L34); `autocompletePlayCommands = true` (L41); `autoQueue = false` (L57); `autoQueueDelay = 0f` (L65); `autoGetGEXP = false` (L72); `gexpMode = 0` (L80); `autoGetWinstreak = false` (L87).

Chat/Automatic: `autoGG = true` (L115); `autoGGMessage = "gg"` (L122); `autoGGFirstMsgDelay = 1f` (L130); `autoGGSendSecondMessage = false` (L137); `autoGGSecondMessage = "Have a good day!"` (L144); `autoGGSecondMsgDelay = 1f` (L152); `casualAutoGG = false` (L159); `antiGG = false` (L166); `autoGL = false` (L173); `autoGLMessage = "glhf"` (L180); `antiGL = false` (L187); `autoFriend = false` (L194); `autoPartyWarpConfirm = false` (L201); `autoReplyAfk = false` (L208); `afkTimeout = 5f` (L216); `afkReplyMessage = "Hey %player%, I am currently AFK!"` (L223).

Chat/Restyler: `gameStatusRestyle = true` (L230); `playerCountBeforePlayerName = true` (L237); `playerCountOnPlayerLeave = true` (L244); `padPlayerCount = false` (L251).

Chat/Visual: `whiteChat = false` (L258); `whitePrivateMessages = true` (L265); `coloredStatuses = true` (L272); `compactGameStartAnnouncements = true` (L279); `shortChannelNames = false` (L286); `shortPMChannelNames = false` (L293); `replaceChatEmotes = false` (L300); `chatEmotesReplacementMode = 0` (L308).

Chat/Parties: `chatSwapper = false` (L315); `chatSwapperReturnChannel = 0` (L323); `chatSwapperChattingIntegration = false` (L330); `chatSwapperHideAllChannelMsg = false` (L337); `notifyWhenKick = false` (L344); `notifyWhenKickInCaps = false` (L351).

Chat/Guild & Watchdog: `broadcastAchievements = false` (L358); `broadcastLevelUp = false` (L365); `guildWelcomeMessage = false` (L372); `thankWatchdog = false` (L379).

Chat/Cooldown: `preventNonCooldown = false` (L386); `preventShoutingOnCooldown = true` (L393).

Chat/Toggles (all default false): hideKarmaMessages, removeLobbyJoin, removeSoulWellAnnouncements, removeGameAnnouncements, removeHypeLimitReminder, removePlayerAds, removePlayerBedwarsAds, removeConnectionStatus, removeGuildMotd, removedServerConnectedMsgs, removeGameTips, removeAutoQuests, removeViewStats, removeCurseOfSpam, removeBridgeOwnGoalDeathMsg, removeDuelsNoStatsChange, removeDuelsBlockTrailDisabled, removeSkyblockWelcome, removeGiftedRanksAmount, removeSimulatorCollectedMsgs, removeEarnedCoinsAndExp, removeReplayMessage, removeTipMessages, removeOnlineStatus, removeLobbyFishingMsgs, removeHotPotato, removeDiscordSafetyWarning (L400–582).

Chat/AutoWB: `autoWB = false` (L589); `guildAutoWB = true` (L596); `friendsAutoWB = true` (L603); `autoWBCooldown = 2f` (L611); `autoWBMessage1 = "Welcome Back!"` (L618); `randomAutoWB = false` (L625); `autoWBMessage2..10` (L631–679).

Tab: `highlightSelfInTab = 0` (L695); `hideGuildTagsInTab = false` (L702); `hidePlayerRanksInTab = false` (L709); `hidePingInTab = true` (L716); `cleanerSkyblockTabInfo = true` (L723); `hideAdsInTab = true` (L730).

Game: `hideHudElements = false` (L739); `hideGameAdsBossbar = true` (L746); `hardcoreHearts = true` (L753); hide titles (3 toggles, false, L760/767/774); `hideArmor = false` (L781); `hideUselessArmorStandsGame = false` (L788); `blockNotify = false` (L795); `blockNumber = 10f` (L803); `blockNotifySound = 0` (L811); `miniWallsMiddleBeacon = false` (L818); `miniWallsMiddleBeaconColor` red (L825); `hideArcadeCosmetics = false` (L832); `heightOverlay = false` (L848); `heightOverlayMinBuild = false` (L855); `overlayAmount = 500f` (L863); `HeightOverlayCustomColors.enabled = false` + 16 map-colors (L870–916); `hideDropperActionBar = false` (L948); `muteDropperHurtSound = false` (L955); `sumoRenderDistance = false` (L962); `sumoRenderDistanceAmount = 2f` (L970); `hideDuelsCosmetics = false` (L977); `hideHousingActionBar = false` (L984); `pitLagReducer = true` (L991); `hideNonNPCs = false` (L998); `highlightChests = false` (L1005); `highlightChestsColor` red (L1012); `uhcOverlay = false` (L1019); `uhcOverlayScale = 2f` (L1027); `uhcMiddleWaypoint = false` (L1034); `uhcMiddleWaypointText = "0,0"` (L1041); `notifyMiningFatigue = true` (L1048); `miningFatigueNotificationType = 0` (L1056).

Lobby: `hideLobbyNPCs = false` (L1065); `hideUselessArmorStands = false` (L1072); `lobbyBossbar = true` (L1079); `silentLobby = false` (L1086); `DisableSpecificLobbySounds` accordion with 18 per-sound toggles (L1093–1201); `limboLimiter = false` (L1208); `limboPmDing = true` (L1215).

Behaviour: Hypixel-specific QoL — chat message filtering/restyling, tab-list editing (hide ranks/guild/ping, highlight self), HUD hiding, game-mode-specific helpers (height overlay, UHC overlay, chest highlights, sumo render distance), lobby sound muting, limbo FPS limiting. Mixins filter chat via `ChatComponent`, edit `PlayerTabOverlay`, adjust `ParticleEngine`/`SoundEngine`.

Legacy note: not for 1.8.9 (modern-only port); the original Hytils was a 1.8.9 Forge mod. Many features are Hypixel-server-specific, not Minecraft-version-specific.

Fair-play: **extensive automated chat/command sending** — `autoGG` (default **true**), `autoGL`, `autoFriend` (auto-accepts friend requests), `autoReplyAfk`, `autoWB`, `autoPartyWarpConfirm`, `autoQueue`/`autoStart`/`autoGetGEXP`/`autoGetWinstreak` (auto-commands), `thankWatchdog`, `broadcastAchievements`/`broadcastLevelUp`, `guildWelcomeMessage`. `autoQueue` shows a warning ("will require you to interact with the game … to prevent abuse"). None give a PvP combat advantage; they automate social/queueing chat.

Summary: the Hypixel QoL suite (chat cleanup, tab editing, game overlays, auto-GG/queue) — archived.

---

## Tabulous

- URL: https://github.com/Polyfrost/Tabulous · **archived: yes** · fork: no
- Branch read: `main` · last commit: 2024-05-05 (cc6fc2f)
- Targets: Minecraft 1.8.9, **Forge** only (uses `net.minecraftforge`, `mcp_stable:22-1.8.9`, VigilanceMigrator). Mod version 1.1.0-oc1.
- Licence: none (no LICENSE file). Deps: OC-legacy (cc.polyfrost.oneconfig), Vigilance migrator. Mixins: 4 (`GuiPlayerTabOverlay`, `GuiIngame`, `GuiIngameForge`, `Gui`).
- Config: `TabulousConfig.java` (OC-legacy). "SUPERSEDED BY VANILLAHUD".

Options: `disabled = false` (L17); `animations = true` (L23); `animSpeed = 10f` slider 2–30 (L30); `position = 0` dropdown Top/TopLeft/TopRight/Left/Bottom (L36); `alwaysAtTop = false` (L42); `cancelBossbar = false` (L48); `closeInGUIs = true` (L54); `topPosition = 10` (L60); `textShadow = true` (L66); `renderScoreboardValues = true` (L72); `hideInvalidNames = false` (L78); `showHeader = true` (L84); `showFooter = true` (L90); `renderPing = true` (L96); `hidePingInGame = true` (L102); `marginFix = 0` (L108); `dontShowHeads = false` (L114); `headPos = false` dual left/right (L121); `headerText = "default"` (L132); `footerText = "default"` (L139); `myNameText = "default"` (L145); `hideCustomNameIngame = false` (L151); `tabColor` rgba(50,50,50,128) (L157); `tabItemColor` rgba(255,255,255,32) (L163); `hideGuilds = false` (L170); `hidePlayerRanksInTab = false` (L176); `hideNPCs = true` (L182); `customNameBW = false` (L194); `cleanerSkyBlockTabInfo = true` (L200); `defaultSkyBlockTab = false` (L205); `showUpdate = true` (L212).

Behaviour: replaces the vanilla tab list rendering (does not overwrite the rendering system) with a custom-positioned, animated tab list — custom header/footer/name text, colors, player heads, ping, margins. Hypixel extras: hide guild/rank tags, hide NPCs, cleaner SkyBlock tab, BedWars custom name.

Legacy note: entirely 1.8.9 Forge (tab rendering differs across versions; superseded by VanillaHUD). Makes little sense in modern 26.x (tab is reimplemented by VanillaHUD).

Fair-play: none — cosmetic tab list only.

Summary: a 1.8.9 Forge tab-list cosmetics mod (position, colors, custom text, heads), superseded by VanillaHUD.

---

## ColoredBedsDataModV1

- URL: https://github.com/Polyfrost/ColoredBedsDataModV1 · archived: no · fork: no
- Branch read: `modern-fabric` (default) · last commit: 2026-11-25 (a9132d5)
- Targets: Minecraft **26.3** Fabric (fabricloader ≥0.20.1). Mod version 2.0.0.
- Licence: Unlicense (public domain). Deps: Fabric loader only (no OneConfig). Mixins: none. 3 source files (`BedData.java`, `BedScanner.java`, `ColoredBedsData.java`).

Behaviour: a **data-collection bot**, not a user-facing mod. Joins Hypixel, visits every Bed Wars map, records which bed color is on each side, and writes `bed_locations.json` for the "colored beds" feature. Commands: `/coloredbedsdata start|reset|export`. Requires Hypixel Mod API and MVP+.

Legacy note: not 1.8.9 — 26.3-only tooling. Fair-play: none (single-player data gathering; no combat). Summary: a dev tool that scrapes Hypixel Bed Wars map bed colors into a data file.

---

## OverflowAnimationsV2

- URL: https://github.com/Polyfrost/OverflowAnimationsV2 · archived: no · fork: no
- Branch read: `stonecutter` (default) · last commit: 2026-11-24 (b2121cb)
- Targets: 1.8.9 + 1.21.4/1.21.6–1.21.8/1.21.9–1.21.10/1.21.11/26.1–26.1.2 (stonecutter). Fabric + Ornithe.
- Licence: GPL-3.0 (with "MINECRAFT linking exception"). Deps: OneConfig 1.8.0. Mixins: 259 (huge render-stack surface: `ItemInHandRenderer`, `PlayerRenderer`, `Camera`, `EntityRenderer`, `GameRenderer`, `LevelRenderer`, `LightTexture`, `Hud`, `Gui`, `ItemRenderer`, `FishingHookRenderer`, `PanoramaRenderer`, `SkyRenderer`, `FogRenderer`, model/render-state classes, etc.).
- Config: 5 category classes + `OverflowAnimationsConfig` (uses `EntryBundle` UI builder; `LEGACY` flag = 1.8.9 build, defaults differ per version).

Options by category (enum options abbreviated; defaults shown):

- Items (`ItemsConfigCategory.java`): itemGlint (V1_15 legacy / VANILLA modern, L48); armorGlint (V1_8/VANILLA, L49); potionGlint (V1_7/VANILLA, L50); glintOnItemDrops2D (L51); glintOnItemFramed2D (L52); itemDropsFaceCamera=false (L54); itemDropsFaceCameraRotationFix (L55); droppedItems2D=MODEL (L56); itemFramed2D=false (L57); firstPersonItemPositions=AUTO (L59); thirdPersonItemPositions=AUTO (L60); strictItemPositionsInThirdPerson=true (L61); thinBlockPositions=false (L62); skullPosition (L63); fishingRodVersion=V1_7 (L64); thinFishingRodLineThickness (L66); punchingDuringUsage=V1_7 (L67); punchingDuringUsageParticles=SHOWN (L68); disableSwingOnUse (L69); dropSwing (L70); disableSwingOnEntityInteract (L71); usingTextureInGUI (L72); equipAnimationVersion (L73); durabilityBarColors (L74); legacyItemRarities (L75); heldItemVisibilityInBoat (L76); itemPickupPosition (L77); eggSnowballParticles (L78); customSwingSpeed=false (L80); itemSwingSpeed/hasteSwingSpeed/miningFatigueSwingSpeed=0 (L81–83); ignoreHasteSpeed/ignoreMiningFatigueSpeed (L84–85); offhandUsageSwinging (L86); alwaysUsageSwing (L87); fakeMissPenaltySwing=true (L88); fakeMissPenaltyParticles=false (L89); disableSwingTranslate/disableSwingPivot (L90–91); legacySwingAnimation (L92); swapSwingAnimation=V26_2 (L93); itemScaleX/Y/Z=1.0 (L95–97); itemOffset/Rotation X/Y/Z=0 (L98–103); applyCustomizationToBlockItems=true (L104); scaleAnimationsWithItem=true (L105); scaleHandWithItem (L106); itemDrops2DColors (L108); legacyProjectiles (L109); xpOrbPosition (L110); fireballModel (L111); disableItemPickupAnimation (L112); itemPickupOffset=0 (L113); entityItemPositions=true (L114); disableHandSway (L115); reequipSpeed=0.4 (L116); smartSwingScaling (L117); disableDropSwingInContainers (L118); resetMiningOnUse (L119); blockHitWhileMining=true (L120); disableAdventureSwing (L121); disableAdventureUsageSwinging (L122); disableAdventureUsageParticles (L123); lunarBlockHitPosition (L124); lunarItemPositions (L125); potionColors=VANILLA (L126); coloredPotionBottles (L127); fishingRodLineFov=true (L128); fishingRodLineThickness=0 (L129); customRodLine (L130); rodLinePosition/swingPosition/consumePosition/blockingPosition/droppedPosition/projectilePosition/fireballPosition — dozens of float transform sliders (L131–175).
- Movement (`MovementConfigCategory.java`): capeMovement (V1_12 legacy/V1_13 modern, L43); disableCapeLean (L44); disableCapeSwingRotation (L45); capeChestplateTranslation (L46); capeSneakPosition=false (L47); sneakAnimation (V1_7/VANILLA, L49); longerUnsneak (L50); sneakEyeHeight=VANILLA (L51); backwardsWalking=VANILLA (L52); uncapBlockingHeadRotation (L53); headRotationInterpolation (L54); sneakBobbing (L55); deathLimbs (L56); bowArmMovement (L57); damageTilt (V1_8/VANILLA, L58); offsetHurtTiltTime (L59); slimArmPosition (L60); disableHurtCamera=false (L62); dinnerboneMode=false (L63); dinnerboneModeEntities=false (L64); wavyArms=false (L65).
- Screen (`ScreenConfigCategory.java`): cameraVersion (V1_8/VANILLA, L41); thirdPersonCrosshair (L42); healthBarFlashing (V1_7/V1_8, L43); centerScrollableListWidgets=false (L44); listWidgetSelectedBorderColor (L45); legacyWidgetHoverTextColor (L46); disableDebugHudBackground=false (L47); debugHudTextShadow=false (L48); disableCameraTransparentPassthrough (L49); tooltipStyleRendering (L50); slotHoverStyleRendering (L51); listBackgroundGradient (L52); inventoryEffects (V1_8/VANILLA, L53); panoramaRendering (L54); legacyLoadingScreen (L55); legacyLoadingScreenProgressBar=false (L56); oldChatPosition (L57); oldCrosshairPosition (L58); disconnectServerToTitleScreen=false (L59); oldCraftingSlotsPosition=false (L60); debugCrosshairStyle=VANILLA (L62); tabListStyle=V1_8 (L63); legacyDebugScreen=false (L64).
- Fixes (`FixesConfigCategory.java`, all default true): fixSneakingFeetPosition, fixMirrorArmSwing, fixOffHandUsingPose, fixCastLineCheck, fixCastLineSwing, fixFireballClientsideVisual, fixTextStrikethroughStyle, fixHighAttackSpeedIndicator (L36–43); viewBobbingTilt (L44); upMinPixelTransparencyLimit=true (L45); useEquipAnimation=V1_8 (L46); fixItemUsageVisualInGUI, fixDoubleUsageVisual, oldSkyRenderingCheck (L47–49); smoothParticles=true (≥26.1, L51).
- Other (`OtherConfigCategory.java`): blueVoidSky, planarSkyFog, cloudHeight, playerVoidBox, oldCloudRendering, voidFog (LEGACY), hitColorArmor=RED (L55), glintAffectsArmorTint (L56), damageTintHeldItems=OFF (L57), damageTintCape (L58), damageTintStyle=VANILLA (L59), restoreParticleBlending, lockBlockingArmRotation, blockMiningProgress, projectileAgeCheck, blockOutlineRendering, disableModelWhilstSleeping, flameDimensions, heldItemArmLogic, thirdPersonBlockAnimation=V1_7 (L69), disableInventoryEntityScissor, maxGlintProperties, flameOffset, persistentBlockOutline, fastGrass, oldY0Height, oldWaterOverlayOpacity, oldWaterColorFog, oldWaterColorEffects, disableRandomBlockRotations, legacyDiffuseLighting, legacyLightmap, legacyFogDarkness, legacySplashPosition (L48–83).

Behaviour: the definitive "old animations" mod — restores 1.7/1.8-era item positions, swing/blockhit, sneak, cape, damage tilt, glint, 2D drops, fishing rod, potion colors, sky/fog/water/lighting, GUI/HUD legacy looks, and an array of vanilla bug fixes. Each setting is a per-version "V1_7/V1_8/V1_9…/VANILLA" enum so the user picks which era's behaviour to emulate. Massive mixin surface into the whole render stack.

Legacy note: **fundamentally about 1.7/1.8 aesthetics**; on modern versions it recreates old animation/render behaviour, on 1.8.9 it restores even-older (1.7) behaviour and fixes 1.8 bugs. Highly version-specific; reimplementing in a 26.x engine means reproducing 1.7/1.8 render semantics, not "enabling" something vanilla 26.x already has.

Fair-play: no chat/command automation. Purely cosmetic/animation; some options (custom swing speed, blockhit visuals, damage tint) could marginally aid PvP *perception* but do not alter hit detection or send packets — no combat advantage.

Summary: an enormous per-version animation/visual parity mod bringing 1.7/1.8-era rendering back to modern Minecraft (and refining 1.8.9).

---

## OverflowAnimations

- URL: https://github.com/Polyfrost/OverflowAnimations · **archived: yes** · fork: no
- Branch read: `main` · last commit: 2023-12-03 (d97de23)
- Targets: Minecraft **1.8.9 Forge** (Vigilance migrator; also had Fabric via preprocessor `//#if MODERN==0 || FABRIC==1`). Mod version 1.2.0-alpha1.
- Licence: LGPL-3.0 · Deps: OC-legacy. Mixins: 12 (`ItemRenderer`, `RenderItem`, `RenderEntityItem`, `RendererLivingEntity`, `LayerArmorBase`, `RenderSnowball`, `PlayerControllerMP`, `Minecraft`, `AnimationHandler`, `ForgeHooksClient`).
- Config: `AnimationsConfig.java` (OC-legacy, PVP mod type). Options: `items2D = false` (L20); `itemSprites = true` (L26); `enchantmentGlint = true` (L32); `enchantmentGlintNew = true` (L38); `thirdPersonItemRendering = true` (L44); `itemSwitch = true` (L50); `spiderfrogPunching = true` (L56); `mixcesAnimations = true` (L62); `firstPersonCarpetPosition = false` (L68); `oldProjectiles = true` (L74); `projectileSprites = true` (L80).

Behaviour: the original 1.8.9 "old animations" mod (predecessor of V2): 2D item drops/sprites, cleaner/1.15 enchantment glint, 1.7 third-person blockhit, 1.7 item switching, blockhit punching, 1.7 Mixces hand positions, 1.7 projectiles.

Legacy note: 1.8.9 Forge only; fully superseded by OverflowAnimationsV2. Would make no sense in modern 26.x (V2 does that job). Fair-play: cosmetic only. Summary: archived 1.8.9 Forge old-animations mod, superseded by V2.

---

## OverflowParticles

- URL: https://github.com/Polyfrost/OverflowParticles · archived: no · fork: no
- Branch read: `main` · last commit: 2026-11-23 (47d9ea4)
- Targets: 1.8.9 + 1.21.1/1.21.4/1.21.5/1.21.8/1.21.10/26.x (stonecutter). Fabric + Ornithe.
- Licence: GPL-3.0 · Deps: OneConfig 1.8.0. Mixins: 48 (`ParticleEngine`, `Particle`, `SingleQuadParticle`, `BlockParticle`, `TerrainParticle`, `BreakingItemParticle`, `ItemParticle`, `FootstepParticle`, `HugeExplosionParticle`, `LargeExplosionParticle`, `BarrierParticle`, `AbstractArrow`, `TrackingEmitter`, `LivingEntity`, `Player`, `Entity`, `ClientPacketListener`, `ClientLevel`).
- Config: `OverflowParticlesConfig.kt` + `ParticleConfig.kt` (per-particle). Global options:

`modEnabled = true` (L34); `isCleanView = true` (L42) — hide own potion particles; `isStaticParticleColor = false` (L50); `particleNoClip = false` (L57); `particleShield = false` (L65); `shieldSize = 1.4f` slider 0–5 (L74); `alwaysCritical = false` (L80); `alwaysSharp = false` (L86); `invulnerabilityCheck = 0` dropdown Off/Skip Undamageable/Only When Damaged (L94).

Per-particle config (`ParticleConfig.kt`, one tree per particle type via `PerParticleConfigManager`): each particle has its own enabled toggle plus multipliers (size/velocity/alpha/color) — e.g. critical, sharpness, block break, explosions, etc. (mirrors ParticlesEnhanced mod).

Behaviour: per-particle customization (scale, color, alpha), clean-view, particle shield (hide particles near the camera), no-clip, always-show critical/sharpness hit particles, invulnerability checks so hit particles don't render on holograms/undamageable entities. Duck-configs for old Patcher and ParticlesEnhanced.

Legacy note: targets both 1.8.9 and modern; the *feature set* is PvP/combat-oriented (hit particles) common to 1.8.9 PvP and modern.

Fair-play: `alwaysCritical`/`alwaysSharp` force hit particles to always render, and `invulnerabilityCheck=Off` shows particles on every hit — these change combat *feedback* but not hit detection. No chat/command automation.

Summary: a PvP-focused particle mod (per-particle customization, clean view, particle shield, hit-particle controls).

---

## PolyPatcher

- URL: https://github.com/Polyfrost/PolyPatcher · archived: no · **fork: yes** (upstream Sk1erLLC/Patcher)
- Branch read: `main` · last commit: 2026-11-16 (71e7fa1d)
- Targets: Minecraft **1.8.9 and 1.12.2 Forge** (`//#if MC==10809` / `MC==11202` preprocessor). Mod version 1.10.4.
- Licence: CC-BY-NC-SA-4.0 (LICENSE.md) · Deps: OC-legacy (cc.polyfrost.oneconfig), Essential (Vigilance) for GUI. Mixins: 289 (huge: `FontRenderer`, `RenderGlobal`, `WorldRenderer`, `EntityRenderer`, `Minecraft`, `GuiNewChat`, `GuiContainer`, `GuiIngame`, `RenderManager`, `ItemRenderer`, `LayerCape`/`LayerArmorBase`, `BlockRendererDispatcher`, `ChunkRenderDispatcher`, `EffectRenderer`, `SoundManager`, `KeyBinding`, `GameSettings`, `ModelLoader`, OptiFine classes, Forge internals, etc.).
- Config: `PatcherConfig.java` (OC-legacy, 1797 lines) + `PatcherSoundConfig.java` (per-sound volume multipliers 0–200, default 100). A large set of options is marked "HIDDEN/DEPRECATED" and replaced by other Polyfrost mods (VanillaHUD, PolyCrosshair, PolyNametag, OverflowParticles, OverflowAnimations, Chatting). The active user-facing options:

Bug Fixes: `keepShadersOnPerspectiveChange = true` (L36); `parallaxFix = false` (L43); `cullingFix = false` (L57); `resourceExploitFix = true` (L64); `layersInTab = true` (L71); `playerVoidRendering = true` (L78); `customSkyFix = true` (L92); `fixedAlexArms = true` (L106); `bookBackground = false` (L113); `keyboardLayout = 0` (L121); `heldItemLighting = true` (L128); `vanillaGlassPanes = false` (L135).

Miscellaneous: `newKeybindHandling = true` (L152); `separateResourceLoading = false` (L159); `fullbright = true` (L174); `smartFullbright = true` (L181); `disableNightVision = false` (L188); `cleanerNightVision = false` (L195); `disableAchievements = false` (L202); `fireOverlayHeight = 0` slider −0.5–1.5 (L210); `fireOverlayOpacityI = 100` (L218); `hideFireOverlayWithFireResistance = false` (L226); `pumpkinOverlayOpacity = 100` (L234); `autoTitleScale = false` (L241); `removeWaterFov = true` (L254); `allowFovModifying = false` (L261); sprinting/flying/bow/speed/slowness FOV modifiers = 1 (L269–301); `disableHotbarScrolling`/`invertHotbarScrolling`/`preventOverflowHotbarScrolling` = false (L308/315/322); `unfocusedSounds = 1.0` (L330); `unfocusedFPS = false` (L337); `unfocusedFPSAmount = 60` (L345); `customFpsLimit = 0` (L354); `removeGroundFoliage = false` (L361); `cleanProjectiles = false` (L368); `riddenHorseOpacityI = 100` (L376); `distortionEffect = 100` (L384); `waterDensity = 100` (L392); `hideAuraOnInvisibleWither = false` (L399); OptiFine zoom options (`scrollToZoom = true` L406, `normalZoomSensitivity` L413, `renderHandWhenZoomed` L420, `customZoomSensitivity = 1.0` L428, `dynamicZoomSensitivity` L435, `smoothZoomAnimation` L442, `smoothZoomAnimationWhenScrolling` L449, `smoothZoomAlgorithm = 0` L457, `toggleToZoom` L464); `normalFpsCounter = true` (L471); `useVanillaMetricsRenderer = true` (L478); `numericalEnchants = false` (L485); `betterRomanNumerals = true` (L492); `windowedFullscreen = false` (L499); `instantFullscreen = false` (L506); `removeWaterOverlay = false` (L513); `disableLightningBolts = false` (L520); `logOptimizer = false` (L534); `logOptimizerLength = 30` (L542); `futureHitBoxes = true` (L557); `cactusHitboxExclusion = true` (L572); `alternateTextShadow = false` (L579); `disableShadowedText = false` (L593); `leftHandInFirstPerson = false` (L600); `betterCamera = true` (L607); `betterHideGui = false` (L614); `removeViewBobbing = false` (L621); `mapBobbing = false` (L628); `staticItems = false` (L635); "Modify Every Sound" button (L643); `naturalCapes = false` (L652); `smoothScrolling = false` (L659).

Performance: `optimizedWorldSwapping = true` (L668); `limitChunks = false` (L675); `chunkUpdateLimit = 50` (L683); `downscalePackImages = true` (L690); `lowAnimationTick = true` (L697); `batchModelRendering = true` (L704); `optimizedFontRenderer = true` (L711); `cacheFontData = true` (L718); `disableArmorstands`/`disableSemitransparentEntities`/`disableEnchantmentBooks`/`disableItemFrames`/`disableMappedItemFrames`/`disableGroundedArrows`/`disableAttachedArrows`/`disableMovingArrows`/`disableSkulls`/`disableFallingBlocks` = false (L733–796); `disableEndPortals = false` (L803); `unstackedItems = false` (L810); `entityCulling = true` (L825); `cullingInterval = 0` (L833); `smartEntityCulling = true` (L840); `dontCullEnderDragons`/`dontCullWithers`/`dontCullNametags`/`dontCullEntityNametags`/`dontCullArmorStandNametags`/`checkArmorstandRules` = true (L847–884); `disableEnchantmentGlint = false` (L891); `entityBackFaceCulling`/`playerBackFaceCulling` = false (L914/922); `entityRenderDistanceToggle = false` (L929); global/player/passive/hostile/tile entity render distances = 64 (L938–970).

Screens: `inventoryPosition = true` (L980); `clickOutOfContainers = false` (L987); `inventoryScale = 0` (L995); `containerBackgroundOpacity = 81.5` (L1007); `containerOpacity = 100` (L1015); `extendedChatLength = true` (L1038); `removeChatMessageLimit = true` (L1045); `compactChat = true` (L1052); `consecutiveCompactChat = false` (L1059); `compactChatTime = 60` (L1067); `removeBlankMessages = false` (L1074); `shiftChat = false` (L1081); `chatDelay = 0` (L1089); `startupNotification = true` (L1096); combat glances (`damageGlance`/`itemCountGlance`/`enchantmentsGlance`/`protectionPercentage`/`projectileProtectionPercentage` = false, L1103–1131); `timestamps = false` + format/style/seconds (L1138–1161); `cleanMainMenu = true` (L1168); `openToLanReplacement = 0` (L1176); `imagePreview = true` (L1184); `imagePreviewWidth = 0.5` (L1192); `safeChatClicks`/`safeChatClicksHistory` = false (L1199/1206); `smartDisconnect = false` (L1213); `confirmQuit = false` (L1220).

Screenshots: `screenshotManager = false` (L1237); `screenshotNoFeedback`/`autoCopyScreenshot`/`screenshotPreview` = false (L1244/1251/1258); `previewTime = 3` (L1266); `previewAnimationStyle = 0` (L1274); `previewScale = 1.0` (L1282); `compactScreenshotResponse` (L1289); `favoriteScreenshot`/`deleteScreenshot`/`uploadScreenshot`/`copyScreenshot`/`openScreenshotsFolder` = true (L1296–1324).

Experimental: `cacheEntrypoints = true` (L1341); `improvedSkinRendering = false` (L1363); `hudCaching = false` (L1377); `cacheFPS = 20` (L1386).

Behaviour: the classic 1.8.9/1.12.2 "everything" mod — vanilla bug fixes (culling, resource exploit, Alex arms, shader persistence), performance (entity culling, render distances, font renderer optimization, batch model rendering, low animation tick, chunk update limit), screen/QoL (inventory scale, container opacity, combat glances = damage/enchant/armor % shown above hotbar), zoom/OptiFine integration, screenshot manager, windowed fullscreen, FOV modifiers, and sound multipliers. Many options were later split into dedicated Polyfrost mods (Chatting, VanillaHUD, OverflowParticles, PolyCrosshair, PolyNametag) and are now disabled/deprecated here.

Legacy note: 1.8.9/1.12.2 Forge only; deeply tied to legacy class layout (Forge + OptiFine). Almost every "bug fix" targets 1.8/1.12 rendering quirks that modern 26.x doesn't have. Would make little sense as-is in 26.x.

Fair-play: no chat/command automation. `extendedChatLength` (256-char chat) is a protocol relaxation; combat glances show item/enchant/armor info (informational HUD, not packet-level). No combat advantage.

Summary: the flagship 1.8.9/1.12.2 Forge bugfix + performance + QoL mod (fork of Patcher).

---

## REDACTION

- URL: https://github.com/Polyfrost/REDACTION · archived: no · fork: no
- Branch read: `main` · last commit: 2026-11-23 (e57f794)
- Targets: 1.8.9 + 1.21.x/26.x (stonecutter). Fabric + Ornithe.
- Licence: GPL-3.0 · Deps: OneConfig 1.8.0. Mixins: 16 (`Camera`, `GameRenderer`, `GameGui`, `Hud`, `Minecraft`, `TitleScreen`, `DirectConnectScreen`, `JoinMultiplayerScreen`, `InventoryMenuScreen`, `ServerListEntryWidget`, `SpectatorGui`).
- Config: `RedactionConfig.kt` ("THE CLEANEST MINECRAFT EXPERIENCE EVER"). Options: `modEnabled = true` (L25); `customHandFOV = false` (L38); `handFOV = 125` slider 0–180 (L46); `serverPreview = false` (L52); `lastServerJoined = false` (L58); `blackbar = false` (L66); `blackbarSlotNumbers = false` (L72); `blackbarColor` black alpha 85 (L78); `blackbarItemColor` white (L84); `addSnow = false` (L91); `particles = 100` slider 50–1000 (L100); `snowColor` white (L108); `connectSnow = false` (L115); `lineWidth = 1f` (L125); `lineColor` white (L133).

Behaviour: minimalist visual cleanup — replace the hotbar with a transparent "blackbar", customizable hand FOV, server preview in direct connect, last-server-joined button, and snow particles with connecting lines inside the inventory.

Legacy note: version-agnostic cosmetics (works 1.8.9→modern). Not 1.8.9-specific. Fair-play: cosmetic only. Summary: a minimalist cosmetics mod (blackbar, hand FOV, inventory snow).

---

## Spice

- URL: https://github.com/Polyfrost/Spice · archived: no · fork: no
- Branch read: `master` · last commit: 2025-11-03 (2d9d829)
- Targets: Minecraft **1.8.9 Fabric** (stonecutter `versions/1.8.9-fabric`). Mod version 1.0.0.
- Licence: GPL-3.0 · Deps: none notable (pure mixin mod). Mixins: 5 (`common.MinecraftMixin`, `common.GuiOverlayDebugMixin`, `lwjgl3.GuiControlsMixin`, `lwjgl3.MinecraftMixin`, `lwjgl3.compat.EssentialGlobalMouseOverrideMixin`).
- Config: **none** — no user-facing options (a library-style optimization mod).

Behaviour: optimization & QoL for 1.8 (LWJGL3-focused): GUI controls handling and Minecraft-level tweaks, an Essential global-mouse-override compat mixin, and a debug overlay mixin. Small, low-level.

Legacy note: 1.8.9-only, LWJGL3-specific. Fair-play: none. Summary: a small 1.8.9 Fabric optimization/QoL mixin mod (no config).

---

## Polydium

- URL: https://github.com/Polyfrost/Polydium · archived: no · fork: no
- Branch read: `main` · last commit: 2026-11-25 (1e28c2b)
- Targets: 1.21.1/1.21.4/1.21.5/1.21.8/1.21.10/1.21.11/26.1/26.2/26.3 (stonecutter, **modern only**, no 1.8.9). Fabric.
- Licence: PolyForm Shield License 1.0.0 (source-available, non-commercial) · Deps: OneConfig 1.7.1. Mixins: 662 (largest surface: Sodium, Iris, RenderPearl/Vulkan backends, Starlight/ScalableLux, LambDynamicLights, Distant Horizons, Xaero's, ImmediatelyFast, CullFewerLeaves, Skyboxify, flashback, etc.).
- Config: `PolydiumToggles.java` (exposed via `PolydiumConfig`/OneConfigCollector). Options (all default `true` unless noted; several are version-gated):

`deferredSky` (≥1.21.10, L9); `guiItemBake` (≥1.21.8, L12); `particleSectionCull` (L14); `sectionCacheSize` (L16); `debugEntryCache` (≥1.21.10, L19); `soundListenerThrottle` (L21); `preciseFrameLimiter` (<26.1, L24); `hideLevelLoadingScreen` (≥1.21.10, L27); `hideReloadOverlay` (L29); `hideTitleFade` (L31); `spriteMipRelease` (L33); `textEffectMerge` (L35); `opaqueItemCutout` (<26.1, false, L38); `singlePassTextShadow` (<1.21.4, L41); `skyMainPass` (≥26.3, L44); `experimentalBorderless` (≥26.3, **false**, L47); `occlusionCulling` (≥1.21.8, L50); `skipItemQuadStack` (L51); `outlineScissor` (L53); `idleBlockEntityTicks` (L55); `earlyAcquire` (≥1.21.11, L58); `framesInFlight` (≥1.21.11 && <26.2, slider 2–4, =2, L61); `dynamicLightOptimisticRead` (L63); `clientLightBudget` (L65); `chunkLoadDirtyOnce` (L67); `particleLightCacheRead` (L69); `particleTickOverlap` (≥1.21.10, L72); `syncedDataCopies` (L74); `parallelBlockStateCache` (L76); `initialScreensProfile` (L78); `animatedSpriteOrder` (≥1.21.11, L81); `nearestPlaneQuadOrder` (≥26.2, L84); `backgroundVertexCopy` (≥26.2, L87); `xaeroMinimapHudCache` (L89); `xaeroRadarIconCache` (L91); `xaeroWorldMapRegionThrottle` (L93).

Behaviour: a modern-Minecraft performance mod that patches the render/GPU stack (Vulkan/RenderPearl in 26.2+, Sodium/Iris on older), adds culling, defers sky, throttles sound/listener updates, bakes GUI items, frees sprite mipmaps, merges text decorations, and integrates/optimizes many popular mods (Xaero's, LambDynamicLights, Distant Horizons, Starlight, ImmediatelyFast, CullFewerLeaves, flashback, skyboxify).

Legacy note: explicitly **modern-only** (26.x/Vulkan focused); the 1.8.9 equivalent is Polydium-Legacy. Deeply tied to 26.x internals.

Fair-play: none — performance only. Summary: a broad modern-Minecraft (26.x/Vulkan) performance optimization mod.

---

## Polydium-Legacy

- URL: https://github.com/Polyfrost/Polydium-Legacy · archived: no · fork: no
- Branch read: `main` · last commit: 2026-11-25 (6cf3ed1)
- Targets: Minecraft **1.8.9** (Fabric/Ornithe). Mod version 1.4.23.
- Licence: PolyForm Shield License 1.0.0 · Deps: none (GSON file config). Mixins: 24 (`WorldRenderer`, `WorldChunk`, `ClientChunkCache`, `Entity`, `LivingEntityRenderer`, `Particle`, `GlStateManager`, `ChatGui`, `ConnectScreen`, `Camera`, `GameGui`, `EndPortalRenderer`, `EntityShadowBatch`, `ServerAddress`, `Minecraft`, `GameRenderer`, `GLX`).
- Config: `PolydiumConfig.java` (plain JSON, not OneConfig). Options (defaults): `particleCollision = true` (L22); `entityBlockCaching = true` (L23); `hudCaching = true` (L24); `hudFps = 60` (L25); `connectSrvLookup = true` (L26); `preciseFrameLimiter = true` (L27); `worldBorderCaching = true` (L28); `tickCulling = true` (L29); `hologramFastPath = true` (L30); `entityShadowCache = true` (L31); `glStateMirror = true` (L32); `directRenderingAutoValue`/`directRenderingUserSet` (L33–34).

Behaviour: the 1.8.9 equivalent of Polydium — client-side optimizations: particle collision, entity block caching, HUD caching at a fixed FPS, server lookup caching, precise frame limiter, world-border/tick culling, hologram fast path, entity shadow caching, GL state mirroring, optional direct rendering.

Legacy note: 1.8.9-only; counterpart to modern Polydium. Fair-play: none. Summary: the 1.8.9 client-side optimization mod (Polydium's legacy twin).

---

## Argentum

- URL: https://github.com/Polyfrost/Argentum · archived: no · **fork: yes** (upstream QuicksilverMC/Argentum)
- Branch read: `main` · last commit: 2026-11-25 (a46d915)
- Targets: Minecraft **1.8.9** (Fabric/Ornithe; Embeddium-derived rendering stack). Mod version 1.0.3-beta.16.
- Licence: none (no LICENSE; gh API reports null) · Deps: Embeddium/Sodium-like internals, ModMenu (optional). Mixins: 221 (`WorldRenderer`, `BlockRenderDispatcher`, `EntityRenderDispatcher`, `ItemRenderer`, `TileEntityRenderDispatcher`, `RenderChunk`, `FastBlockRenderer`, `ModelBakery`, `Stitcher`, `TextureAtlasSprite`, plus a whole Entity Shadow batch and name-tag instancing).
- Config: three files — `ArgentumConfig.java` (core), `CeraConfig.java` (OptiFine-features module "Cera"), `ArgentumExtrasConfig.java` (extras). Plain JSON, no OneConfig.

ArgentumConfig defaults: `chunkBuilderThreads = 0` (L11); `deferChunkUpdates = true` (L12); `asyncOcclusion = EVERYTHING` (L13); `fogCulling = true` (L14); `entityCulling = true` (L15); `entityInstancing = true` (L16); `bakeBlockEntities = true` (L17); `nameTagBatching = true` (L18); `entityOcclusionIntervalMs = 50` (L19); `particleCulling = true` (L20); `fastParticles = true` (L21); `particleLightCache = true` (L22); `blockFaceCulling = true` (L23); `rasterCulling = false` (L24); `translucencySorting = true` (L25); `renderPassOptimization = true` (L26); `animateOnlyVisibleTextures = true` (L27); `chunkFadeInDuration = 0` (L28); `fogShape = SPHERICAL` (L29); `useSkyFog = false` (L30); `rgssTextureFiltering = false` (L31); `fasterClouds = true` (L32); `defaultClouds = false` (L33); `fasterWeather = true` (L34); `fasterEndPortals = true` (L35); `cpuRenderAheadLimit = 3` (L36); `biomeBlendRadius = 3` (L37); `safeChunkEdges = false` (L38); `vertexFormat = COMPACT` (L39); `checkGlErrors = false` (L40); `fontBatching = true` (L41); `guiItemAtlas = true` (L42); `directRendering = false` (L43); `greedyRenderThread = false` (L44); `explicitVsyncOption = true` (L45); `exclusiveFullscreen = false` (L46); `overhangingBorderless = true` (L47); fullscreen width/height/refresh = 0 (L48–50).

CeraConfig (OptiFine feature emulation) defaults: betterGrass/connectedTextures/dynamicLights = FANCY (L10–12); naturalTextures/animatedTextures/customColors/customLightmaps/randomEntities/customEntityModels/customSky/customGuis/customItems/optifineCosmetics/emissiveTextures/customBlockLayers/customLoadingScreens/customPanorama/hdFonts = true (L13–28); `cemAnimationLodDistance = 24` (L19); `shaderPack` = disabled (L29); `referenceShaderPack = false` (L30); `renderQuality`/`shadowQuality`/`handDepth` = 1 (L31–33); `depthOfField = false` (L34); `shaderErrorReports = true` (L35).

ArgentumExtrasConfig defaults: particle toggles (explosion/spell/portal/smokeAndFlame/redstone/water/misc) = true (L4–10); `steadyDebugHud = true` (L11); `debugHudRefreshIntervalMs = 250` (L12); `debugHudScale = 0` (L13); `fpsHud = false` (L14); `dynamicFovStrength`/`portalDistortionStrength`/`viewBobbingStrength`/`hurtCameraStrength` = 100 (L15–18); `cloudRenderDistance = 0` (L19); `cloudHeightOffset = 0` (L20); `cloudSpeed = 100` (L21); `cloudFog = true` (L22); `terrainFogDensity`/`fluidFogDensity` = 100 (L23–24); `entityShadowDistance = 16` (L25); entity-render toggles (armor/glint/heldItems/wornHeads/capes/ears/arrows/leashes/nameTags/beaconBeam/enchantingTableBook/pistons/itemFrames/paintings) = true (L26–40); `sky = true` (L41); `forceSky = false` (L42); `lowerSky = true` (L43); `sunAndMoon`/`stars` = true (L44–45); `vignetteStrength = 100` (L46); `weatherRenderDistance = 0` (L47); `weatherDensity = 100` (L48); `leafQuality = FANCY` (L49); `disableTextShadows = false` (L50); `achievementToasts = true` (L51); `fxaa = false` (L52); `smoothBlockOutlines = false` (L53); `blockOutlineWidth = 2.0` (L54); `scaledBlockOutlineWidth = false` (L55); `macosSmoothScrolling = true` (L56); `macosRightClickEmulation = false` (L57); `highDpiScreen = false` (L58).

Behaviour: a modern "Sodium/Embeddium-like" rendering engine backported to 1.8.9 — full chunk rebuild pipeline, async occlusion culling, entity culling/instancing, font batching, GUI item atlas, and a "Cera" module that reimplements OptiFine features (CTM, better grass, dynamic lights, shaders, CEM, emissive textures, HD fonts, custom sky/colors), plus extras (particle filters, fog/clouds/weather controls, entity toggles, FXAA).

Legacy note: 1.8.9-only, replaces the entire 1.8.9 renderer. OptiFine parity for 1.8.9. The concepts (render pipeline, culling, OptiFine features) are already native or provided by Sodium/Iris in modern 26.x, so a direct port would be redundant.

Fair-play: none — rendering only. Summary: an Embeddium/OptiFine-style rendering engine for 1.8.9 (fork).

---

## Sarcio

- URL: https://github.com/Polyfrost/Sarcio · archived: no · **fork: yes** (upstream QuicksilverMC/Sarcio)
- Branch read: `main` · last commit: 2026-11-22 (2fbbc61)
- Targets: Minecraft **1.8.9** (Fabric/Ornithe). Mod version 1.2.0-beta.1.
- Licence: LGPL-3.0 · Deps: none (JSON config). Mixins: 162 (`ChunkMap`, `ClientChunkCache`, `AnvilChunkStorage`, `ChunkNibbleStorage`, `LiquidRenderer`, `ItemRenderer`, `ModelManager`, `Stitcher`, `NbtCompound`, `HttpTexture`, `ConnectScreen`, `ServerAddress`, `GameRenderer`, `ParticleManager`, `TextRenderer`, `SkullRenderer`, render-layer classes, etc.).
- Config: `SarcioConfig.java` (plain JSON). Options: `releaseCrashReserve = false` (L20); `disableRealms = false` (L21); `persistentKeybindings = true` (L22); `logServerDisconnect = true` (L23).

Behaviour: targeted memory optimizations and bugfixes for 1.8.9 — frees the crash-reserve memory, disables Realms, persists keybindings, logs server disconnects, plus many memory-allocation/caching mixins (NBT, chunk storage, texture stitching, particle/liquid rendering). The config exposes only 4 toggles; most optimizations are always-on mixins.

Legacy note: 1.8.9-only memory/bugfix work; the equivalent concerns are handled by ModernFix/FerriteCore in modern versions. Fair-play: none. Summary: a 1.8.9 memory-optimization + bugfix mixin mod (fork).

---

## Dynamic-FPS

- URL: https://github.com/Polyfrost/Dynamic-FPS · archived: no · **fork: yes** (upstream juliand665/Dynamic-FPS)
- Branch read: `1.8.9` (default) · last commit: 2026-09-04 (99f3c39)
- Targets: Minecraft **1.8.9** (Fabric/Ornithe; upstream supports modern too, this fork's default branch is 1.8.9). Mod version 1.0.0.
- Licence: MIT · Deps: none (JSON config). Mixins: 5 (`Minecraft`, `GameGui`, `KeyBinding`, `DebugOverlay`).
- Config: `DynamicFPSConfig` + per-power-state `Config` + `IdleConfig` + `BatteryTrackerConfig` + `VolumeTransitionConfig`. Per-state (`FOCUSED`, `HOVERED`, `UNFOCUSED`, `INVISIBLE`, `ABANDONED`): `frameRateTarget`, `enableVsync`, per-sound-category `volumeMultipliers`, `graphicsState` (default/change), `showToasts`, `runGarbageCollector`. Idle: `timeout` + `condition`. Battery tracker: `enabled`, `criticalLevel`, `switchStates`, `notifications`, `showWhenDebug`, `display.condition`, `display.placement`. Volume transition: `up`/`down` speeds.

Behaviour: reduces frame rate and volume when Minecraft is unfocused/hidden, switches graphics state, optionally runs GC, tracks battery, transitions volume smoothly.

Legacy note: 1.8.9 fork; feature is version-agnostic (also exists for modern). Fair-play: none. Summary: lowers FPS/volume when the game window is inactive (1.8.9 fork).

---

## FPSLimiter

- URL: https://github.com/Polyfrost/FPSLimiter · archived: no · fork: no
- Branch read: `master` · last commit: 2025-11-10 (fed4fbf)
- Targets: multi-loader (Fabric `fabric.mod.json` + Forge `mcmod.info`), multi-version via preprocessor (dev.deftu template). Loads from `patcher.toml`.
- Licence: GPL-3.0 · Deps: OneConfig. Mixins: 2 (`Minecraft`, `SoundHandler`).
- Config: `FPSLimiterConfig.java`. Options: `useFpsLimit = false` (L19); `customFpsLimit = 120` number 5–MAX (L26); `useMaxMainMenuFpsLimit = false` (L31); `maxMainMenuFpsLimit = 30` (L37); `useUnfocusedFpsLimit = false` (L43); `unfocusedFpsLimit = 60` (L48); `unfocusedSounds = 1f` (L54).

Behaviour: split-out of Patcher's FPS limiting — custom global FPS cap, main-menu FPS cap, unfocused FPS cap, unfocused sound volume. Mixins `Minecraft` (limit frame) and `SoundHandler` (unfocused volume).

Legacy note: a Patcher-split feature (1.8.9-era); the concept is trivial and largely redundant in modern clients (which have their own frame limiter). Fair-play: none. Summary: FPS/sound limiting when unfocused (Patcher split).

---

## PolyReflex

- URL: https://github.com/Polyfrost/PolyReflex · archived: no · fork: no
- Branch read: `main` · last commit: 2026-11-15 (7a1793d)
- Targets: Minecraft **26.2+** (Vulkan). Mod version 0.2.1.
- Licence: GPL-3.0 · Deps: depends on Polydium (frame path). Mixins: 10 (`Minecraft`, `RenderSystem`, `VulkanBackend`, `VulkanGpuSurface`, `VulkanQueue.Submission`, Polydium frame-path/present-thread classes).
- Config: none (no config class).

Behaviour: implements NVIDIA Reflex / AMD Anti-Lag 2 low-latency on MC 26.2+ Vulkan — reduces input latency by aligning CPU/GPU frame submission (frame path + present thread integration with Polydium's Vulkan backend).

Legacy note: modern 26.2+ Vulkan only. Fair-play: it is a **latency-reduction** feature — in competitive PvP this is a legitimate, widely-accepted advantage (same category as low-latency drivers), not an exploit. Summary: Reflex/Anti-Lag 2 low-latency implementation for 26.2+ Vulkan.

---

## PolyDebug

- URL: https://github.com/Polyfrost/PolyDebug · archived: no · fork: no
- Branch read: `master` · last commit: 2026-09-01 (162ad13)
- Targets: a **standalone Java CLI/GUI tool** (not a Minecraft mod), plus a `probe-mod` that runs inside Minecraft **≥26.2**.
- Licence: GPL-3.0 · Deps: Prism Launcher integration. Mixins: none in the app; `probe-mod` is a separate module.
- Config: `Config.java` — a command-line argument parser, not a runtime config. Key options (defaults): `bundles = performance,qol,hud,utility` (L34); `warmupSeconds = 75` (L48); `measureSeconds = 90` (L49); `worldTimeoutSeconds = 300` (L50); `calibrationRuns = 3` (L51); `maxRepeats = 3` (L52); `thresholdFraction = 0.5` (L54); `memoryMegabytes = 4096` (L60); window 1280×720 (L61–62); `renderDistance = 12` (L63); plus `--base-url`, `--minecraft`, `--loader`, `--exclude`, `--instance`, `--player`, `--world`, `--seed`, `--server`, `--find-all`, `--setup-only`, `--dry-run`, `--no-oneclient-settings`, etc.

Behaviour: installs the OneClient modpack into its own Prism instance, copies the user's settings, measures frame rate in a fixed world, then binary-searches (halves) the mod list to identify which mod costs frames. Writes reports to `polydebug-reports`.

Legacy note: modern (26.2+ measurement); a dev/diagnostic tool, not a gameplay mod. Fair-play: none. Summary: a benchmarking harness that finds which mod is costing you FPS.

---

## CrashPatch

- URL: https://github.com/Polyfrost/CrashPatch · archived: no · fork: no
- Branch read: `main` · last commit: 2026-11-23 (f05bc5a)
- Targets: 1.8.9 + 1.21.x/26.x (stonecutter). Fabric + Ornithe.
- Licence: GPL-3.0 · Deps: OneConfig 1.9.0. Mixins: 27 (`Minecraft`, `CrashReport`, `ConnectScreen`, `DisconnectedScreen`, `BufferBuilder`, `Gui`, entry-point catchers, `ModLoaders`).
- Config: `CrashPatchConfig.kt`. Options: `inGameCrashPatch = true` (L19); `initCrashPatch = true` (L26); `disconnectCrashPatch = true` (L33); `leaveLimit = 3` slider 1–20 (L50); `crashLimit = 5` slider 1–20 (L59); `crashLogUploadMethod = HASTEBIN` (L66).

Behaviour: catches crashes during gameplay/launch and prevents the game from closing, displays disconnect causes, limits the number of recoveries before advising to quit, uploads crash logs (hst.sh / mclo.gs).

Legacy note: version-agnostic; useful on 1.8.9 (fragile mod ecosystem) and modern. Fair-play: none. Summary: keeps the game alive through crashes and uploads crash logs.

---

## ConfirmDisconnect

- URL: https://github.com/Polyfrost/ConfirmDisconnect · archived: no · fork: no
- Branch read: `main` · last commit: 2025-12-05 (e281e15)
- Targets: multi-loader/multi-version (preprocessor; dev.deftu template), loads `patcher.toml`. Mixin: 1 (`GuiIngameMenu`).
- Licence: CC-BY-NC-SA-4.0 · Deps: OneConfig.
- Config: `ConfirmDisconnectConfig.java`. Single option: `mainToggle = false` (L10).

Behaviour: adds a confirmation dialog before disconnecting from a server (prevents accidental disconnect).

Legacy note: a Patcher-split QoL; trivial/version-agnostic. Fair-play: none. Summary: confirms before leaving a server.

---

## ScrollTweaks

- URL: https://github.com/Polyfrost/ScrollTweaks · archived: no · fork: no
- Branch read: `main` · last commit: 2025-11-19 (ebf5edf)
- Targets: multi-loader/multi-version (preprocessor; dev.deftu template), loads `patcher.toml`. Mixin: 1 (`InventoryPlayer`).
- Licence: LGPL-3.0 · Deps: OneConfig.
- Config: `ScrollTweaksConfig.java`. Options: `disableHotbarScrolling = false` (L19); `invertHotbarScrolling = false` (L25); `preventOverflowHotbarScrolling = false` (L31).

Behaviour: hotbar scroll wheel tweaks (disable/invert/prevent overflow wrap). Split from Patcher.

Legacy note: 1.8.9-era hotbar behaviour; hotbar scrolling is native everywhere so this is a minor tweak. Fair-play: none. Summary: hotbar scroll-wheel tweaks.

---

## SoundTweaks

- URL: https://github.com/Polyfrost/SoundTweaks · archived: no · fork: no
- Branch read: `master` · last commit: 2026-11-20 (2a9a436)
- Targets: 1.8.9 + 1.21.x/26.x (stonecutter). Fabric + Ornithe.
- Licence: GPL-3.0 · Deps: OneConfig 1.8.0. Mixins: 5 (`SoundEngine`, `SoundManager`, `SoundRegistry`, `AbstractSoundInstance`).
- Config: `SoundTweaksConfig.kt`. Options: `enabled = true` (L19); `removeVolumeCap = false` (L30); plus a dynamically-generated per-sound volume slider (0–200, default 100) for every registered sound (L35–63).

Behaviour: per-sound volume multipliers (0–200%), and removing Minecraft's 100% volume cap. Mirrors Patcher's "Modify Every Sound".

Legacy note: version-agnostic. Fair-play: none. Summary: per-sound volume control and volume-cap removal.

---

## SmartSearch

- URL: https://github.com/Polyfrost/SmartSearch · archived: no · fork: no
- Branch read: `master` · last commit: 2026-11-17 (6935504)
- Targets: 1.8.9 + 1.21.x/26.x (OneConfig 1.5.0; builds for many versions). Mixins: none (integrates with OneConfig's search).
- Licence: GPL-3.0 · Deps: OneConfig 1.5.0, an ML embedding model (semantic search).
- Config: `SmartSearchConfig.kt`. Options: `enabled = true` (L29); `enableSemantic = true` (L36); `overwriteDefault = false` (L86); plus ~24 advanced tuning numbers (lexical boosts, fuzzy length, KNN semantic weights) with values read from `SearchParams.DEFAULT` (L106–330).

Behaviour: replaces OneConfig's default mod/option search with a hybrid lexical + semantic (ML embedding) search across all installed mods' options, with an indexer and embedding cache.

Legacy note: version-agnostic (search over OneConfig registry). Fair-play: none. Summary: an ML-powered search for mod settings.

---

## PackManager

- URL: https://github.com/Polyfrost/PackManager · archived: no · fork: no
- Branch read: `main` · last commit: 2026-07-09
- **Contains NO code** — the repository holds only a `LICENSE` file (GPL-3.0). No source to document.

Summary: empty repository (LICENSE only).

---

## PolyWaypoints

- URL: https://github.com/Polyfrost/PolyWaypoints · archived: no · fork: no
- Branch read: `main` (HEAD, no commits) · **empty repository** (cloned as empty; no default branch content).

Summary: empty repository (no code).

---

## PolySnapper

- URL: https://github.com/Polyfrost/PolySnapper · archived: no · fork: no
- Branch read: `main` (HEAD, no commits) · **empty repository**.

Summary: empty repository (no code). Description describes it as "Expansion mod for Snapper adding opinionated defaults and a PolyUI-based GUI", but the repo is empty.

---

## Snapper

- URL: https://github.com/Polyfrost/Snapper · archived: no · **fork: yes** (upstream SpiritGameStudios/Snapper)
- Branch read: `multiversion` (default) · last commit: 2025-08-18 (5276d58)
- Targets: multi-version Fabric (1.8.9 through modern; 1.21.5+ special-cased). Mod version 1.1.x.
- Licence: MPL-2.0 · Deps: OneConfig. Mixins: 9 (`ScreenshotRecorder`, `Camera`, `GameMenuScreen`, `TitleScreen`, `MinecraftClient`, `Keyboard`, `EntryListWidget`).
- Config: `SnapperConfig.java`. Options: `copyTakenScreenshot = false` (L21); `showSnapperTitleScreen = true` (L27); `showSnapperGameMenu = true` (L33); `viewMode = GRID` (L39); `termsAccepted = UNSET` (L45); `useCustomScreenshotFolder = false` (L51); `customScreenshotFolder` (L53).

Behaviour: a screenshot manager — screenshot viewer screen (grid/list), copy-to-clipboard on capture, custom screenshot folder, upload to AxolotlClient image host.

Legacy note: multiversion (works 1.8.9→modern). Fair-play: none. Summary: a screenshot manager/viewer with upload.

---

## WindowedFullscreen

- URL: https://github.com/Polyfrost/WindowedFullscreen · archived: no · fork: no
- Branch read: `master` · last commit: 2025-11-12 (f8b93a2)
- Targets: multi-loader/multi-version (preprocessor; dev.deftu template), loads `patcher.toml`. Mixin: 2 (`Minecraft`).
- Licence: CC-BY-NC-SA-4.0 · Deps: OneConfig.
- Config: `WindowedFullscreenConfig.java`. Single option: `windowedFullscreen = false` (L18).

Behaviour: borderless windowed fullscreen (drag mouse outside the window). Split from Patcher.

Legacy note: 1.8.9-era; modern clients have borderless fullscreen natively. Fair-play: none. Summary: borderless windowed fullscreen toggle.

---

## BetterScreens

- URL: https://github.com/Polyfrost/BetterScreens · archived: no · fork: no
- Branch read: `main` · last commit: 2025-11-17 (15e8620)
- Targets: multi-loader/multi-version (preprocessor; `official-like` mappings), loads `patcher.toml`. Mixins: 9 (`GuiContainer`, `GuiChest`, `GuiInventory`, `GuiScreen`, `InventoryEffectRenderer`, `ForgeHooksClient`, `NetHandlerPlayClient`, `ScaledResolution`).
- Licence: GPL-3.0 · Deps: OneConfig.
- Config: `BetterScreensConfig.java`. Options: `inventoryPosition = true` (L20, 1.8-only); `preventClosingScreens = false` (L27); `clickOutOfContainers = false` (L34); `inventoryScale = 0` dropdown Off/1–4/5-Auto (L41); `containerBackgroundOpacity = 81.5` (L51); `containerOpacity = 100` (L57).

Behaviour: screen/inventory tweaks split from Patcher — fixed inventory position (potion effects don't shift it), prevent servers closing screens, click-out-of-containers, inventory scale, container background/opacity.

Legacy note: `inventoryPosition` is 1.8-only (modern fixed it); the rest is version-agnostic. Fair-play: none. Summary: inventory/screen tweaks (Patcher split).

---

## Legacy-Starlight

- URL: https://github.com/Polyfrost/Legacy-Starlight · archived: no · fork: no
- Branch read: `main` · last commit: 2026-11-24 (95b30d1)
- Targets: Minecraft **1.8.9** Fabric (Ornithe). Mod version 1.0.0.
- Licence: GPL-3.0 · Deps: Fabric loader. Mixins: 8 (`World`, `ServerWorld`, `WorldChunk`, `AnvilChunkStorage`, `ClientChunkCache`, `ClientWorld`, `WorldChunkS2CPacket`).
- Config: none.

Behaviour: ports Starlight's lighting engine to 1.8.9 — rewrites light propagation for correctness and speed.

Legacy note: 1.8.9-only. Modern Minecraft already has Starlight-scale lighting improvements, so this is purely a legacy performance fix. Fair-play: none. Summary: Starlight lighting engine for 1.8.9.

---

## LegacySkinLayers

- URL: https://github.com/Polyfrost/LegacySkinLayers · archived: no · fork: no
- Branch read: `master` · last commit: 2026-10-11 (b811d88)
- Targets: Minecraft **1.8.9** Fabric (Ornithe). Uses OSL (Ornithe Standard Libraries) config.
- Licence: GPL-3.0 · Deps: OSL config. Mixins: 7 (`PlayerEntityRenderer`, `PlayerEntityModel`, `ModelPart`, `SkullBlockEntityRenderer`, `SkullOverlayEntityModel`, `PlayerSkinTexture`, `BlockEntityItemStackRenderHelper`).
- Config: `SkinLayersConfig.kt` (OSL BaseConfig, `skinlayers.json`). Options: enableHat/Jacket/LeftSleeve/RightSleeve/LeftPants/RightPants = true (L24–29); `baseVoxelSize = 1.15` (L31); `bodyVoxelWidthSize = 1.05` (L32); `headVoxelSize = 1.18` (L33); `firstPersonPixelScaling = 1.1` (L34); `enableSkulls = true` (L36); `enableSkullsItems = true` (L37); `skullVoxelSize = 1.1` (L38); `renderDistanceLOD = 14` (L40); `fastRender = true` (L41); `compatibilityMode = true`/`irisCompatibilityMode = false`/`applySodiumWorkaround = false` (L43–45, "unused on 1.8.9").

Behaviour: reimplements 3D Skin Layers for 1.8.9 — renders the skin's outer hat/jacket/sleeves/pants as 3D voxel layers, with 3D hat on worn/placed player heads, LOD render distance, fast-render box mode.

Legacy note: 1.8.9-only; the modern equivalent is the 3D Skin Layers mod. Fair-play: none. Summary: 3D skin layers for 1.8.9.

---

## LegacyWaveyCapes

- URL: https://github.com/Polyfrost/LegacyWaveyCapes · archived: no · fork: no
- Branch read: `master` · last commit: 2026-11-24 (4d377a2)
- Targets: Minecraft **1.8.9** Fabric (Ornithe). Uses OSL config.
- Licence: GPL-3.0 · Deps: OSL config. Mixins: 2 (`CapeFeatureRenderer`, `PlayerEntity`).
- Config: `WaveyCapesConfig.kt` (OSL, `waveycapes.json`). Options: `capeStyle = SMOOTH` (L18); `windMode = NONE` (L19); `capeMovement = BASIC_SIMULATION_3D` (L21); `gravity = 25` (L22); `heightMultiplier = 6` (L23); `straveMultiplier = 2` (L24).

Behaviour: reimplements WaveyCapes for 1.8.9 — physics-simulated animated capes (wind waves, gravity, movement multipliers).

Legacy note: 1.8.9-only; the modern equivalent is the WaveyCapes mod. Fair-play: none. Summary: wavey/animated capes for 1.8.9.

---

## LegacyTweaks

- URL: https://github.com/Polyfrost/LegacyTweaks · archived: no · fork: no
- Branch read: `master` · last commit: 2025-11-16 (cd83264)
- Targets: multi-loader/multi-version (preprocessor; `official-like` mappings; `//#if MC == 1.8.9` guards), loads `patcher.toml`. Mixins: 9 (`LayerCape`, `RenderItem`, `RenderFish`, `RenderSnowball`, `EntityRenderer`, `EntityPlayer`, `Entity`, `ItemRenderer`).
- Licence: GPL-3.0 · Deps: OneConfig.
- Config: `LegacyTweaksConfig.java`. Options: `leftHandInFirstPerson = false` (L20, 1.8.9 only); `naturalCapes = false` (L27); `cleanProjectiles = false` (L33); `distortionEffect = 100` slider 0–100 (L40).

Behaviour: Patcher-split visual tweaks — natural cape physics, clean projectiles (hide for 2 ticks), distortion-effect strength, and (1.8.9) left-hand-in-first-person.

Legacy note: 1.8.9-oriented (left-hand rendering is native in modern). Fair-play: none. Summary: small visual tweaks split from Patcher.

---

## Legacy-Crafty-Crashes

- URL: https://github.com/Polyfrost/Legacy-Crafty-Crashes · archived: no · fork: no
- Branch read: `main` · last commit: 2024-07-23 (2d0de72)
- Targets: Minecraft **1.8.9** Forge. Mod version 1.0.0.
- Licence: none (no LICENSE). Mixins: 3 (`CrashReport`, `CrashReportCategory`, `Minecraft`).
- Config: none.

Behaviour: a 1.8.9 port of Crafty-Crashes — improves crash report quality/readability (formatting, extra detail in crash reports).

Legacy note: 1.8.9 Forge only. Fair-play: none. Summary: improved 1.8.9 crash reports (Crafty-Crashes port).

---

## Tooltip-Scroll-Fabric

- URL: https://github.com/Polyfrost/Tooltip-Scroll-Fabric · archived: no · **fork: yes** (upstream Provismet/Tooltip-Scroll-Fabric)
- Branch read: `26.1` (default) · last commit: 2026-06-24 (3af1505)
- Targets: Minecraft **≥26.1** Fabric. Mod version 1.5.1.
- Licence: MIT · Deps: Cloth Config. Mixins: 5 (`ClientTextTooltip`, `GuiGraphicsExtractor`, `KeyMapping`, `MouseHandler`, `Screen`).
- Config: `TooltipConfig.java` (Cloth Config screen). Options: `canScroll = true` (L22); `useWASD = false` (L28); `startOnTop = false` (L34); `resetOnUnlock = true` (L40); `useLShift = true` (L46); `invertXScroll = false` (L52); `invertYScroll = false` (L58); `scrollspeed = 10` (L64); `scrollspeedKeys = 5` (L70); `smoothness = 0.25` (L76); `matrixMode = false` (L85).

Behaviour: lets long tooltips be scrolled with the scroll wheel or WASD/arrow keybinds, with smoothing and invert options.

Legacy note: 26.x fork; feature is version-agnostic (upstream covers other versions). Fair-play: none. Summary: scrollable tooltips (26.x fork).

---

## MediaSession

- URL: https://github.com/Polyfrost/MediaSession · archived: no · fork: no
- Branch read: `master` · last commit: 2026-11-24 (7df809d)
- Targets: **a Kotlin library** (build.gradle.kts; not a Minecraft mod — no fabric.mod.json, no mixins).
- Licence: Apache-2.0 · Deps: platform media APIs (Windows SMTC, macOS Now Playing, Linux MPRIS/DBus).
- Config: none.

Behaviour: reads the system's currently-playing media (title/artist/album/art) on Windows/macOS/Linux and fetches time-synced lyrics from online providers. Consumed by PolyPlus (lobby "now playing" + lyrics display).

Legacy note: N/A (library, platform-agnostic). Fair-play: none. Summary: a media-now-playing + synced-lyrics library used by PolyPlus.

---

## PolyPlus

- URL: https://github.com/Polyfrost/PolyPlus · archived: no · fork: no
- Branch read: `main` · last commit: 2026-11-25 (483dc15)
- Targets: 1.8.9 (Ornithe) + 1.21.1/1.21.4/1.21.5/1.21.8/1.21.10/1.21.11/26.1/26.2/26.3 (stonecutter). Fabric + Ornithe. Mod version 2.4.1.
- Licence: GPL-3.0 · Deps: OneConfig 1.9.0, PolyCompose, Ktor, Sentry, mixin extras. Mixins: 329 (render/menu/social stack + many third-party compat mixins: Sodium, Iris, Embeddium, Distant Horizons, Essential, Dynamic-FPS, WaveyCapes, SkinLayers, EvergreenHUD, W-Waypoints, PolyBlur, FullBright, VoiceChat, etc.).
- Config: split across 7 config files (`polyplus` umbrella + cosmetics/lobby/main-menu/party/pause/screens). This is the "OneClient modpack" itself.

Feature groups (from `PolyPlusConfig.kt` and sub-configs):

- General (`PolyPlusConfig`): RAM analysis notifications (`ramAdviceNotifications=true` L92), problematic-mod warnings (`badModWarnings=true` L99), branded startup screen (L106), startup-time notification (L113), resource-pack version warnings (L120), OneConfig section (L127), OneClient watermark (position/scale/offset/color, L141–202), PolyPlus user indicators (`showPolyPlusIndicator=true` L297), chat emoji (`showChatEmoji=true` L304), disable narrator while muted (L311), auto-refresh session (L318), API URL (L538).
- Voice (`PolyPlusConfig` Multiplayer/Voice): `voiceEnabled=true` (L327), `voiceAutoJoin=false` (L336), input mode (L346), push-to-talk keybind (L354), activity threshold 50 (L369), input/output volume 100 (L381/393), proximity range 48 (L405), Java sound fallback (L414), Linux RTC (L423), `voiceHideIp=true` (L432), keybinds (socials, emote wheel, wardrobe, mute/deafen/join-leave, ping — L457–535).
- Cosmetics (`PolyPlusCosmeticsConfig`): capes, wings, gloves, head/feet/back/shoulders/aura cosmetics; hide-with-helmet/boots toggles; force PolyPlus capes.
- Lobby (`PolyPlusLobbyConfig`): lobby reveal, lobby time, vignette, lobby music + volume + mute (unfocused/media), quit confirmation ("Click Twice to Quit"), scene capture/view, automatic scene limit.
- Main menu (`PolyPlusMainMenuConfig`): custom main menu with hide quickplay/alt-manager/social/wardrobe/realms/host-world/mod-buttons/footer, main-menu FPS limit, panorama backdrop/custom panorama, modern in-game menus.
- Party (`PolyPlusPartyConfig`): share server with party, share scenes, let party join hosted world, join when leader plays, social menu size, pings (show party pings, ping sound, marker size).
- Pause (`PolyPlusPauseConfig`): pause-screen style, always-pan camera, player preview, compact buttons, vignette, open animation, widget position, host-world button.
- Screens (`PolyPlusScreensConfig`): modern options layout, create/edit world, connecting, realms screens.

Behaviour: a full "client" layering on top of Minecraft — branded main menu/lobby/3D scene with shaders, party system with voice chat (EOS/peer-to-peer), cosmetics (capes/wings/etc.), ping system, social overlay, chat emoji, and re-skinned pause/options/screens. Large 3D lobby rendering with post-processing shaders.

Legacy note: works on 1.8.9 (Ornithe) and modern; the lobby/shaders are version-gated (1.8.9 uses legacy shaders). The lobby/scene rendering is a substantial modern-feature build, not a legacy-only mod.

Fair-play: no combat automation. Social/voice/party features; `voiceHideIp` (privacy) and pings are client-side. No combat advantage.

Summary: the "OneClient" umbrella mod — branded main menu/lobby, party + voice, cosmetics, pings, and re-skinned menus.

---

## quick-pack

- URL: https://github.com/Polyfrost/quick-pack · archived: no · **fork: yes** (upstream DrexHD/quick-pack)
- Branch read: `main` · last commit: 2026-08-08 (1750636)
- Targets: multi-loader multi-version (Fabric/Forge/NeoForge; 1.20.1, 1.21.1, 1.21.11, 26.1/26.2/26.3). Mod version 1.4.0.
- Licence: GPL-3.0 · Deps: none. Mixins: 10 (`FilePackResources`, `FolderRepositorySource`, `SpriteLoader`, `LoadingOverlay`, `Gui`, `MinecraftServer`, `Minecraft`).
- Config: `Config.java`. Options: `removeLoadingOverlayFadeOut = true` (L4); `parallelMipmapGeneration = true` (L5).

Behaviour: speeds up datapack/resourcepack ZIP loading — parallel mipmap generation and removal of the loading-overlay fade-out.

Legacy note: version-agnostic (multiloader). Fair-play: none. Summary: faster resource/datapack loading (parallel mipmaps).

---

## packed_packs

- URL: https://github.com/Polyfrost/packed_packs · archived: no · **fork: yes** (upstream fishstiz/packed_packs)
- Branch read: `mc/1.8.9` (default) · last commit: 2026-10-14 (daed084)
- Targets: Minecraft **1.8.9** Fabric (Ornithe). Mod version 2.3.1+1.8.9.
- Licence: MIT · Deps: none. Mixins: 7 (`Minecraft`, `ResourcePacks`, `ResourcePacksScreen`, `ResourcePackEntry`, `ZippedResourcePack`).
- Config: `Config.java` (JSON). Options: `replaceOriginal = true` (L19); `applyOnClose = true` (L20); `devMode` (L21); `hideIncompatible`/`hideIncompatibleWarnings` (L22–23); `foldersModuleByDefault` (L24); `rememberLastViewedProfile` (L25); `sort` (L26); `additionalFolders` (L27); `actionBar` (L28); `preferences` map (L29); `activeProfile`/`profiles` (L30–31) with per-pack `hidden`/`required`/`position` overrides.

Behaviour: organize resource/data packs into named profiles; hide/require/position packs, additional pack folders, apply-on-close.

Legacy note: 1.8.9 fork (upstream targets modern too). Fair-play: none. Summary: resource/data-pack profile manager for 1.8.9.

---

## flashback-legacy

- URL: https://github.com/Polyfrost/flashback-legacy · archived: no · fork: no
- Branch read: `main` · last commit: 2026-11-15 (73bcf56)
- Targets: Minecraft **1.8.9** Fabric (Ornithe). Uses OSL/ploceus.
- Licence: GPL-3.0 · Deps: Flashback replay engine, OSL. Mixins: 63 (`Recorder`, `ReplayUI`, `SaveReplayScreen`, `ReplayCombiner`, `GameRenderer`, `RenderTarget`, `ParticleManager`, `ItemRenderer`, `PlayerRenderer`, `WorldRenderer`, `MinecraftServer`, network handlers, `MouseSdl`/`DisplaySdl`/`KeyboardInput`, etc.).
- Config: no OneConfig config; a `LegacyConfigScreen` and keybinds (replay recording/playback).

Behaviour: ports Flashback (a full game-recording/replay mod) to 1.8.9 — record gameplay, export video, replay it in-game.

Legacy note: 1.8.9-only; Flashback itself is a modern mod, this backports it. Fair-play: none (replay/recording). Summary: Flashback replay mod for 1.8.9.

---

## ForgeCompat

- URL: https://github.com/Polyfrost/ForgeCompat · archived: no · fork: no
- Branch read: `main` · last commit: 2026-11-21 (09f6adc)
- Targets: Minecraft **1.8.9** (Ornithe/Fabric via ploceus). Mod version 1.1.6.
- Licence: GPL-3.0 · Deps: Pylon, Essential API (universalcraft/elementa/vigilance stubs), java3d:vecmath. Mixins: 45 (`GameRenderer`, `WorldRenderer`, `Minecraft`, `MinecraftServer`, `Connection`, `ConnectScreen`, `Screen`, `RenderTarget`, `TextRenderer`, `TextureAtlas`, `GameOptions`, `StatusEffect`, `Biome`, `Block`, `Item`, `PlayerRenderer`, `LivingEntityRenderer`, etc.).
- Config: none user-facing (the `LegacyOneConfig`/Forge config classes are shims).

Behaviour: a compatibility layer that lets selected Forge 1.8.9 mods run on Ornithe — reimplements `net.minecraftforge.*` APIs (config, events, FML config GUI) as shims and bundles an Essential API (`EssentialAPI`, `EssentialApi`, `Essential` entry points), so Essential-dependent Forge mods (e.g. 1.8.9 Essential/Patcher ecosystem) can load. Ships viafabric/creamykeys stub jars.

Legacy note: 1.8.9 Forge-compat shim only; irrelevant to modern clients (modern is Fabric/NeoForge natively). Fair-play: none (it's infrastructure). Summary: runs Forge 1.8.9 mods on Ornithe via a Forge/API shim layer.

---

## VintageFix

- URL: https://github.com/Polyfrost/VintageFix · **archived: yes** · **fork: yes** (upstream embeddedt/VintageFix)
- Branch read: `main` · last commit: 2024-08-22 (4237a58)
- Targets: Minecraft **1.12.2** Forge. Mod version 1.0.0.alpha.1.
- Licence: LGPL-3.0 (portions MIT from FerriteCore) · Deps: none. Mixins: 78 (`BlockModelShapes`, `ModelManager`, `ModelBakery`, `Stitcher`, `ItemModelMesherForge`, `ItemLayerModel`, `RenderChunk`, `RenderGlobal`, `Chunk`, `ChunkCache`, `CompiledChunk`, `BlockStateContainer`, `FMLClientHandler`, `GuiChat.ChatTabCompleter`, `TextureMap`, `VertexBuffer`, etc.).
- Config: none (uses a `MixinConfigPlugin` for conditional mixins).

Behaviour: FerriteCore + ModernFix "venturing into the stone age of 1.12" — memory reductions (block state, model, chunk, texture stitching), model-loading fixes, and performance patches for 1.12.2 Forge.

Legacy note: 1.12.2 Forge only; the same fixes are native/ModernFix in modern versions. Fair-play: none. Summary: FerriteCore/ModernFix-style memory+performance fixes for 1.12.2 (archived fork).

---

## OneClient

- URL: https://github.com/Polyfrost/OneClient · archived: no · fork: no
- Branch read: `main` · last commit: 2026-11-21 (c2b2bff)
- Targets: 1.8.9 (Ornithe) + 1.21.1/1.21.4/1.21.5/1.21.8/1.21.10/1.21.11/26.1/26.2/26.3 (stonecutter). Fabric + Ornithe. Mod version 1.0.0.
- Licence: GPL-3.0 · Deps: OneConfig 1.9.0, PolyCompose. Mixins: 5 (`TitleScreen`, `GameRenderer`, `Gui`, `Minecraft`).
- Config: `OneClientConfig.kt`. Single option: `useVanillaMainMenu = false` (L16).

Behaviour: the OneClient main-menu branding mod — replaces the title screen with the OneClient main menu (the branded menu normally part of PolyPlus), standalone, with a toggle back to the vanilla main menu. Declares `breaks: polyplus`.

Legacy note: version-agnostic (works 1.8.9→26.x). Fair-play: none. Summary: the OneClient main menu as a standalone mod.
