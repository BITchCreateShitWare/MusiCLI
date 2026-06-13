export interface MetadataResult {
  title: string;
  artist: string;
  album: string;
  year: number | null;
  genre: string | null;
  track: number | null;
  duration: number;
  bitrate: number | null;
  sampleRate: number | null;
  codec: string;
  error?: string;
}

export interface LyricsUpdateData {
  current: string;
  next: string[];
}

export interface LyricsThemeData {
  font?: string;
  fontSize?: number;
  fg?: string;
  fgDim?: string;
  accent?: string;
  bg?: string;
  lyricsAccent?: string;
  lyricsFg?: string;
  lyricsNextCount?: number;
  lyricsGap?: number;
  lyricsShadow?: string;
  lyricsAlign?: string;
  lyricsCurrentSize?: number;
  lyricsNextSize?: number;
  lyricsVertical?: string;
}
