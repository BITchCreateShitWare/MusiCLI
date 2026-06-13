import { useEffect, useRef } from 'react';
import { SettingsProvider, SHADOW_PRESETS, applyCssVars } from './contexts/SettingsContext';
import { PlaylistProvider, usePlaylists, type PlayerSync } from './contexts/PlaylistContext';
import { PlayerProvider, usePlayer } from './contexts/PlayerContext';
import { TerminalProvider, useTerminal } from './contexts/TerminalContext';
import { TitleBar } from './components/TitleBar';
import { BackgroundLayer } from './components/BackgroundLayer';
import { Terminal } from './components/Terminal';
import { SelectList } from './components/SelectList';
import { NowPlaying } from './components/NowPlaying';
import { InputLine } from './components/InputLine';
import { FloatingLyrics } from './components/FloatingLyrics';
import { getStoredSettings } from './contexts/SettingsContext';
import { initConfig, setMusicFolder, saveSettings } from './configStore';
import { getBridge, isBridgeAvailable, initBridge } from './bridge';

function AppInitializer({ children }: { children: React.ReactNode }) {
  const player = usePlayer();
  const playlists = usePlaylists();
  const terminal = useTerminal();
  const initialized = useRef(false);

  useEffect(() => {
    if (initialized.current) return;
    initialized.current = true;

    // Initialize bridge first (detects Tauri/Electron environment)
    initBridge().then(() => {

    // Wire PlayerContext functions into PlaylistContext
    const sync: PlayerSync = {
      addToPlaylist: player.addToPlaylist,
      clearPlaylist: player.clearPlaylist,
      getPlaylist: player.getPlaylist,
    };
    playlists.registerPlayerSync(sync);

    // Wire terminal lyric printing into PlayerContext
    player.registerLyricPrinter((text, cls) => terminal.printLine(text, cls));

    playlists.ensureDefault();

    // Load current playlist tracks into player
    const pl = playlists.getCurrentPlaylist();
    if (pl && pl.tracks && pl.tracks.length > 0) {
      player.clearPlaylist();
      player.addToPlaylist(pl.tracks);
    }

    // Auto-detect music folder
    const s = getStoredSettings();
    if (!s.musicFolder && isBridgeAvailable()) {
      try {
        getBridge().getDefaultMusicDir().then(folder => {
          getBridge().dirExists(folder).then(exists => {
            if (exists) {
              const stored = getStoredSettings();
              stored.musicFolder = folder;
              setMusicFolder(folder);
              saveSettings(stored);
            }
          });
        });
      } catch { /* browser mode */ }
    }

    // Load config from files FIRST — must complete before any save operations
    // to avoid overwriting manual file edits with stale localStorage cache.
    initConfig().then((fileSettings) => {
      if (fileSettings) {
        applyCssVars(fileSettings);
      }
      // Refresh playlists from file-loaded config (replaces stale localStorage state)
      playlists.reloadFromStore();
      const reloadedPl = playlists.getCurrentPlaylist();
      if (reloadedPl && reloadedPl.tracks && reloadedPl.tracks.length > 0) {
        player.clearPlaylist();
        player.addToPlaylist(reloadedPl.tracks);
      }
      // Restore lyrics state AFTER file config is loaded (uses updated cache)
      const s2 = getStoredSettings();
      if (s2.volume != null) player.setVolume(s2.volume);
      if (s2.lyricsTerminal) {
        player.setLyricsTerminal(true);
      }
      if (s2.lyricsFloating) {
        player.setLyricsFloating(true);
      }
    });
    }); // end initBridge().then()
  }, []);

  // Force-sync lyrics settings 200ms after startup (blunt but reliable)
  useEffect(() => {
    const timer = setTimeout(() => {
      const s = getStoredSettings();
      const baseFonts = '"Consolas", "Courier New", "Fira Code", monospace';
      if (isBridgeAvailable()) {
        getBridge().sendLyricsTheme({
        font: s.customFont ? `"${s.customFont}", ${baseFonts}` : baseFonts,
        fontSize: s.fontSize || 14, fg: s.fg, fgDim: s['fg-dim'],
        accent: s.accent, bg: s.bg,
        lyricsAccent: s.lyricsAccent || '#b1b9f9',
        lyricsFg: s.lyricsFg || '#cccccc',
        lyricsNextCount: s.lyricsNextCount || 1,
        lyricsGap: s.lyricsGap || 10,
        lyricsShadow: SHADOW_PRESETS[s.lyricsShadow] || '0 0 10px rgba(0,0,0,0.85)',
        lyricsAlign: s.lyricsAlign || 'center',
        lyricsCurrentSize: s.lyricsCurrentSize || 24,
        lyricsNextSize: s.lyricsNextSize || 14,
        lyricsVertical: { off: 'horizontal-tb', rl: 'vertical-rl', lr: 'vertical-lr' }[s.lyricsVertical || 'off'],
        });
      }
    }, 200);
    return () => clearTimeout(timer);
  }, []);

  return <>{children}</>;
}

export default function App() {
  const isLyricsWindow = window.location.hash === '#/lyrics';

  if (isLyricsWindow) {
    return <FloatingLyrics />;
  }

  return (
    <SettingsProvider>
      <PlaylistProvider>
        <PlayerProvider>
          <TerminalProvider>
            <AppInitializer>
              <BackgroundLayer />
              <TitleBar />
              <Terminal />
              <NowPlaying />
              <SelectList />
              <InputLine />
            </AppInitializer>
          </TerminalProvider>
        </PlayerProvider>
      </PlaylistProvider>
    </SettingsProvider>
  );
}
