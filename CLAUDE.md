# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run Commands

```bash
pnpm dev              # Vite dev server only (browser, no Electron IPC)
pnpm start            # Full Electron app: starts Vite + launches Electron
pnpm build            # TypeScript check + production build to dist/
pnpm lint             # ESLint
pnpm electron:build:win  # Build Windows installer (needs PowerShell in PATH)
```

Package manager is **pnpm**. The `package.json` does NOT have `"type": "module"` — Electron main/preload use CommonJS (`require`), while the renderer (React/TS) is bundled by Vite as ESM.

### Packaging

```bash
# On Windows, PowerShell must be in PATH for electron-builder:
export PATH="$PATH:/c/Windows/System32/WindowsPowerShell/v1.0"
pnpm electron:build:win
```

Output: `release/Musicli 2.0.0.exe` (portable) and `release/Musicli-2.0.0-win.zip`.

## Architecture Overview

**Electron** desktop music player with a **pseudo-CLI terminal aesthetic**. User types commands into an input line at the bottom; output scrolls in a terminal-like area above. A now-playing bar sits between them. Supports floating lyrics in a separate transparent window.

### Process Model

- **Main process** (`electron/main.js`): CJS. Creates a frameless BrowserWindow + transparent floating lyrics window. Exposes IPC handlers for file dialogs, music-metadata, directory listing (including recursive LRC search), file I/O, LRC offset persistence, lyrics theme replay, mouse passthrough toggle, window auto-size. Uses `webSecurity: false` only in dev mode (`file://` audio cross-origin with `http://localhost` page).
- **Preload** (`electron/preload.js`): CJS. Bridges IPC to renderer via `contextBridge.exposeInMainWorld('musicPlayer', { ... })`. All renderer↔main communication goes through this single surface.
- **Renderer** (`src/`): React 19 + TypeScript, bundled by Vite. No Node.js integration (`contextIsolation: true`, `nodeIntegration: false`).

### Renderer Component Tree & Context Hierarchy

```
App
├── SettingsProvider          (CSS variables, themes, language)
│   └── PlaylistProvider      (named playlists, syncs to player)
│       └── PlayerProvider    (audio element, playback state, lyrics)
│           └── TerminalProvider (output lines, select/imode/seek states, commands)
│               └── AppInitializer (wires contexts together, startup sync)
│                   ├── BackgroundLayer
│                   ├── TitleBar
│                   ├── Terminal        (renders lines + banner)
│                   ├── NowPlaying      (track info, progress bar, volume)
│                   ├── SelectList      (fuzzy select / interactive multi-select)
│                   └── InputLine       (command input, history, keybindings)
```

The nesting order matters: `PlaylistProvider` wraps `PlayerProvider` because playlist operations (add/remove/switch tracks) must sync into the player's live playlist array.

### Context Cross-Communication

**PlaylistContext ↔ PlayerContext**: `PlaylistContext.registerPlayerSync(sync: PlayerSync)` receives `addToPlaylist`, `clearPlaylist`, `getPlaylist` from the player. `AppInitializer` wires this once. Then `addTracksToCurrent()`, `replaceCurrentTracks()`, `switchPlaylist()`, and `deletePlaylist()` all automatically sync to the player's active playlist.

**PlayerContext → TerminalContext**: `PlayerContext.registerLyricPrinter(fn)` receives `terminal.printLine`. `AppInitializer` wires this. Terminal-mode lyrics use this to print timed lines.

### Command System

`src/commands/registry.ts` — Flat command registry. `register(name, aliases, handler, helpKey)` stores commands keyed by lowercase name/alias.

`src/commands/handlers.ts` — All commands defined here. Uses module-level `_ctx: CommandContext` set by `setCommandContext()`. The `CommandContext` bundles functions from all four contexts.

**CRITICAL**: `registerAllCommands()` is called at **module level** (not in `useEffect`). If called in `useEffect`, Vite HMR resets the module-level `commands` object but the effect never re-runs, silently losing all commands.

### IPC Return Types

All IPC handlers that can fail return `T | { error: string }`. Use `hasError(obj)` helper: `typeof obj === 'object' && obj !== null && 'error' in obj`. **Do not** use raw `'error' in result` — TypeScript 6 requires `object` type for `in`.

### Interactive Modes

1. **Fuzzy select** (`selectMode`): After `play <name>` with multiple matches. Arrow keys + Enter, Esc, mouse wheel.
2. **Interactive multi-select** (`imode`): For `import` and `track pl`. Space toggles, Enter confirms, Esc cancels, typing filters.
3. **Seek mode** (`seekMode`): `seek` with no args. Left/Right arrows seek by configurable step. Any other key exits.

Filter input: `onInput` reads `inputRef.current.value` → `updateFilter()`. **Never** intercept Backspace/Delete/printable keys with `preventDefault()`. Only intercept arrows, space, enter, escape.

### Settings & Persistence

**Config files** are stored as JSON in `{musicFolder}/config/`:
- `settings.json` — All AppSettings (colors, fonts, lyrics, etc.)
- `themes.json` — Named themes
- `playlists.json` — `{ playlists: Record<string, Playlist>, current: string }`
- `lang.json` — Language code (`"en"` | `"zh"` | `"ja"`)

**Architecture**: `src/configStore.ts` is the single persistence layer.
- Module-level in-memory cache populated synchronously from localStorage at import time
- `initConfig()` (called once in AppInitializer useEffect) reads files asynchronously, updates cache + localStorage
- All save functions (`saveSettings`, `saveThemes`, `savePlaylists`, `saveLang`) write to BOTH file AND localStorage
- `initConfig()` is **read-only** — it never writes to files to avoid overwriting manual edits
- `musicFolder` is the only bootstrap key stored solely in localStorage (`musicli-musicfolder`)
- Synchronous getters (`getSettings()`, `getThemes()`, `getPlaylists()`, `getLang()`) read from in-memory cache for backward compat with existing sync call sites

**Startup order** (critical for correctness):
1. Module load: configStore cache ← localStorage (sync)
2. React render
3. AppInitializer useEffect: `initConfig()` reads files → updates cache → applyCssVars → `playlists.reloadFromStore()` → restore lyrics
4. All writes (saves) happen AFTER files are loaded, so manual file edits survive restart

**Key rules:**
- `ensureDefault()` only persists if it actually made changes (not unconditionally)
- Never write to files during startup — only on explicit user actions (commands)
- `saveSettings` accepts `Partial<AppSettings>` — does `Object.assign` on the cache, then writes to file + localStorage

### Floating Lyrics Window

Separate BrowserWindow (`transparent: true, alwaysOnTop: true`), loads same app with `#/lyrics` hash. Fixed width 600px, auto-height via `ResizeObserver` + IPC `lyrics-window:auto-size`.

**Theme sync**: Sent via `sendLyricsTheme()` IPC → main process stores `lastLyricsTheme` → replays on `did-finish-load`. Three sync points fire the same payload on a 200ms delay:
1. `AppInitializer` startup effect
2. `PlayerContext.setLyricsFloating(true)` on window open  
3. `SettingsContext.saveSettings()` on any config change

All floating lyrics CSS (`--lyrics-*`) has defaults on `:root`. IPC sets CSS variables on the lyrics window's document root. CSS `var()` reads directly without fallback (defaults are on `:root`).

Shadow presets: `SHADOW_PRESETS` map in `SettingsContext.tsx` (large/medium/small → CSS text-shadow values).

### LRC Timing Offset

Per-track offset (ms) stored in `lrc/offsets.json` in the LRC directory. `lyric offset <ms>` writes via IPC `lrc:writeOffset`. On `loadLRC`, offsets are read via `lrc:readOffsets` and applied to parsed lines. Wrapped in try-catch so missing IPC doesn't crash lyrics loading.

### Independent Lyrics States

Terminal and floating lyrics are independent booleans (`lyricsTerminal`, `lyricsFloating`). Both can be on simultaneously. `lyric t` toggles terminal, `lyric f` toggles floating, `lyric off` disables both. Vertical mode (`lyric v`) cycles: off → vertical-rl → vertical-lr.

### Key Lessons Learned

- **Don't fight the browser for text input.** Use `onInput`, only intercept semantic keys.
- **Load persisted state synchronously** in `useState` initializer or module level, not `useEffect`.
- **Combine related state updates into single functions** (e.g. `updateFilter(newFilter)` vs `setX()` + `calcY()`).
- **Standalone helper functions > memoized context methods** for derived state (e.g. `filterItems(items, query)`).
- **Module-level registration** for commands — survives HMR.
- **200ms delay sync** is more reliable than complex multi-source sync for config that must survive restarts.
- **`var()` in CSS can only have ONE fallback** — comma-separated values break. Define defaults on `:root` instead.
- **React StrictMode double-invokes nested state setters** — use refs instead of `setOuter(prev => { setInner(...) })`.
- **Context values created in render are always latest** but callbacks close over stale state — pass values as arguments, use refs.
