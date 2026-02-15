# yt-dlp Modern GUI

 
A modern, cross-platform desktop application for downloading videos using yt-dlp.
Built with Tauri 2.0 (Rust) and SvelteKit, providing a clean and intuitive interface for managing video downloads.

[**한국어**](docs/README.ko.md) | [**日本語**](docs/README.ja.md) | [**中文(简体)**](docs/README.zh-CN.md) | [**中文(繁體)**](docs/README.zh-TW.md) | [**Español**](docs/README.es.md) | [**Français**](docs/README.fr.md) | [**Deutsch**](docs/README.de.md) | [**Português**](docs/README.pt-BR.md) | [**Русский**](docs/README.ru.md) | [**Tiếng Việt**](docs/README.vi.md)

## ScreenShots

<p align="center">
  <img src="docs/App.png" alt="yt-dlp Modern GUI" width="450">
</p>
<p align="center">
  <img src="docs/Downloading.png" alt="yt-dlp Modern GUI" width="450">
</p>


## Features

- Video & playlist download with format and quality selection
- Concurrent download queue with cancel and retry
- Download history with search and management
- Automatic yt-dlp and FFmpeg dependency detection with installation guide
- Filename template customization (simple & advanced modes)
- Cookie support for authenticated content
- Duplicate download detection
- Multi-language support (English, 한국어, 日本語, 简体中文, 繁體中文, Français, Deutsch)
- 4 color themes (Dark, Violet, Red, Light)
- Cross-platform support (Windows, macOS, Linux)

> **💡 Tip:** The app automatically downloads yt-dlp, FFmpeg, and Deno on first launch. However, these auto-managed binaries (PyInstaller-packaged) can be slow on initial startup. For **significantly faster** metadata fetching and downloads, pre-install them via your system package manager — [Homebrew](https://brew.sh/) on macOS (`brew install yt-dlp ffmpeg`), [winget](https://learn.microsoft.com/windows/package-manager/winget/) on Windows (`winget install yt-dlp.yt-dlp ffmpeg`), or `apt`/`pacman` on Linux. The app will automatically detect and prefer the system-installed versions from your PATH.

## Build from Source

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (v18+)
- [Bun](https://bun.sh/) (package manager)
- Platform-specific dependencies for [Tauri 2.0](https://v2.tauri.app/start/prerequisites/)

### Steps

```bash
# Clone the repository
git clone https://github.com/shlifedev/yt-dlp-modern-gui.git
cd yt-dlp-modern-gui

# Install frontend dependencies
bun install

# Run in development mode
bun run tauri dev

# Build for production
bun run tauri build
```

The production build output will be in `src-tauri/target/release/bundle/`.

## Roadmap

1. Downloader app for mobile users (you can self-host your own yt-dlp server)
2. Version updater

## License

This project is licensed under the [MIT License](LICENSE).
