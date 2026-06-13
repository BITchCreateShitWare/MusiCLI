# MusicLI

> 拟终端风格本地音乐播放器 | Pseudo-CLI Local Music Player

[中文](#中文) | [English](#english)

---

## 中文

### 简介

MusicLI 是一款**拟终端命令行风格**的桌面音乐播放器，使用 Tauri v2 + Rust + React + TypeScript 构建。音频引擎基于 Symphonia 解码 + cpal 输出，支持 WASAPI（共享/默认）和 ASIO（独占）模式。支持 MP3/FLAC/WAV/OGG/M4A 等格式、ID3 元数据解析、LRC 歌词显示（终端内嵌 + 透明悬浮桌面歌词）、主题系统和歌单分享。

### 特性

- **命令行风格界面** — 键入命令控制一切，方向键历史
- **Rust 音频引擎** — Symphonia 解码 → rubato 重采样 → cpal 输出，支持 WASAPI/ASIO
- **多种播放模式** — 顺序 / 单曲循环 / 列表循环 / 随机
- **LRC 歌词** — 终端内嵌 + 透明悬浮桌面歌词，竖排/横排、颜色/大小/阴影/对齐全可配
- **子目录歌词检索** — 递归搜索音乐文件夹和 MP3 父目录
- **歌词时序偏移** — 每首歌独立调整 LRC 偏移，自动保存
- **歌单管理** — 创建/编辑/切换歌单，批量导入，模糊搜索
- **元数据展示** — ID3 标签，显示专辑、年份、码率等
- **主题系统** — 内置暗色 / Claude Desktop 主题，支持导入导出
- **外观定制** — 自定义字体、背景图片、模糊度、进度条、窗口圆角
- **三语言** — 简体中文 / English / 日本語
- **跨平台** — Windows / Linux / macOS
- **歌单分享 (Sync)** — ZIP 打包（音频 + LRC + 元数据），跨设备导入
- **配置持久化** — JSON 文件存储在音乐文件夹 `config/` 目录下，可手动编辑

### 安装

从 [Releases](../../releases) 下载：

- **Windows**: `MusicLI_3.0.0_x64-setup.exe` 或 `MusicLI_3.0.0_x64_en-US.msi`
- **Linux**: `musicli_3.0.0_amd64.deb` 或 `MusicLI_3.0.0_amd64.AppImage`

### 从源码构建

**前置要求**
- [Rust 工具链](https://rustup.rs)
- [LLVM/Clang](https://github.com/llvm/llvm-project/releases) — ASIO SDK 编译需要（Windows）
- [Node.js](https://nodejs.org) 22+
- [pnpm](https://pnpm.io)

```bash
git clone https://github.com/KirariNeko/MusicLI.git
cd MusicLI
pnpm install

# 开发模式（仅前端，无原生 IPC）
pnpm dev

# 完整 Tauri 开发（启动 Vite + Tauri 窗口）
pnpm tauri dev

# 生产构建
pnpm tauri build
```

### 命令

#### 文件
| 命令 | 说明 |
|------|------|
| `open` | 选择音频文件 |
| `folder` / `open dir` | 打开文件夹加载全部音频 |
| `import` | 导入至歌单（搜索 + 多选） |

#### 播放
| 命令 | 说明 |
|------|------|
| `play [n\|name]` | 播放 / 恢复（模糊搜索） |
| `pause` / `stop` | 暂停 / 停止 |
| `next` / `prev` | 下一首 / 上一首 |
| `mode` | 循环模式 |
| `vol <0-100>` | 音量 |
| `seek [sec]` | 跳转；无参数进入方向键模式 |
| `bar` | 进度条 |
| `audio mode [normal\|asio]` | 音频输出模式 |
| `audio devices` | 列出音频设备 |

#### 歌词
| 命令 | 说明 |
|------|------|
| `lyric t` | 切换终端歌词 |
| `lyric f` | 切换悬浮歌词 |
| `lyric off` | 关闭全部 |
| `lyric accent\|fg <#hex>` | 当前行/后续行颜色 |
| `lyric next <0-10>` | 后续行数 |
| `lyric gap <px>` | 行间距 |
| `lyric shadow <off\|s\|m\|l>` | 文字阴影 |
| `lyric align <l\|c\|r>` | 对齐 |
| `lyric v` | 竖排模式 |
| `lyric size current\|next <px>` | 字体大小 |
| `lyric lock` | 鼠标穿透 |
| `lyric offset <ms>` | LRC 时序偏移 |

#### 歌单
| 命令 | 说明 |
|------|------|
| `cd [name]` | 切换歌单 |
| `pl create <name>` | 创建歌单 |
| `pl list` | 列出歌单 |
| `pl info` | 歌单详情 |
| `pl edit <name> <field> <value>` | 编辑歌单 |
| `pl delete <name>` | 删除歌单 |
| `track info <n>` | 曲目信息 |
| `track pl <n>` | 编辑曲目所属歌单 |

#### 外观
| 命令 | 说明 |
|------|------|
| `color <type> <#hex>` | 设置颜色 |
| `colors` | 显示颜色 |
| `set bg [clear]` | 背景图 |
| `set blur <0-50>` | 模糊度 |
| `set font size\|weight\|import` | 字体 |
| `set maxlines <n>` | 终端最大行数 |
| `theme list\|save\|load\|delete\|export\|import` | 主题管理 |
| `reset` | 恢复默认 |

#### 分享
| 命令 | 说明 |
|------|------|
| `sync pl export [name]` | 导出歌单（ZIP：音频文件 + LRC + 元数据） |
| `sync pl import` | 导入歌单 |
| `sync theme export [name]` | 导出主题 |
| `sync theme import` | 导入主题 |

#### 系统
| 命令 | 说明 |
|------|------|
| `lang <en\|zh\|ja>` | 切换语言 |
| `help` | 帮助 |
| `clear` | 清屏 |
| `quit` | 退出 |

### Sync 分享

`sync pl export` 将歌单打包为 ZIP：

```
MusicLI_MyPlaylist_sync.zip
  ├── README.txt         # NekoCraft / 仓库地址
  ├── manifest.json      # 歌单元数据 + 曲目信息
  ├── audio/             # 音频文件
  └── lrc/               # LRC 歌词文件
```

导入时自动创建独立歌单，音频和歌词放入 `MusicLI_Imports/<playlist>/` 目录。

### 配置

所有配置存储在音乐文件夹的 `config/` 子目录：

```
Music/config/
  settings.json    # 外观、播放、歌词设置
  themes.json      # 主题
  playlists.json   # 歌单
  lang.json        # 语言
```

可直接编辑 JSON 文件，重启生效。

### 技术栈

Tauri v2 · Rust 2021 · React 19 · TypeScript · Vite 8 · Symphonia · cpal · rubato · Lofty

---

## English

### Overview

MusicLI is a **pseudo-CLI terminal-style** desktop music player. Built with Tauri v2 + Rust + React + TypeScript. The audio engine uses Symphonia for decoding and cpal for output, with rubato sample rate conversion. Supports MP3/FLAC/WAV/OGG/M4A, ID3 metadata, LRC lyrics (inline terminal + floating desktop overlay), themes, and playlist sharing.

### Quick Start

**Prerequisites**
- [Rust toolchain](https://rustup.rs)
- [LLVM/Clang](https://github.com/llvm/llvm-project/releases) — Required for ASIO SDK build (Windows)
- [Node.js](https://nodejs.org) 22+
- [pnpm](https://pnpm.io)

```bash
git clone https://github.com/KirariNeko/MusicLI.git
cd MusicLI
pnpm install
pnpm tauri dev     # Full Tauri app (Vite + native window)
pnpm tauri build   # Production build
```

Type `help` for all commands. Type `lang en` for English UI.

### Tech Stack

Tauri v2 · Rust 2021 · React 19 · TypeScript · Vite 8 · Symphonia · cpal · rubato · Lofty

### License

MIT
