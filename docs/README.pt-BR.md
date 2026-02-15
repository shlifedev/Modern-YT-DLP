# yt-dlp Modern GUI


Um aplicativo de desktop moderno e multiplataforma para baixar vídeos usando yt-dlp.
Construído com Tauri 2.0 (Rust) e SvelteKit, fornecendo uma interface limpa e intuitiva para gerenciar downloads de vídeos.

[**English**](../README.md) | [**한국어**](README.ko.md) | [**日本語**](README.ja.md) | [**中文(简体)**](README.zh-CN.md) | [**中文(繁體)**](README.zh-TW.md) | [**Español**](README.es.md) | [**Français**](README.fr.md) | [**Deutsch**](README.de.md) | [**Русский**](README.ru.md) | [**Tiếng Việt**](README.vi.md) | **Português**

## Capturas de tela

<p align="center">
  <img src="App.png" alt="yt-dlp Modern GUI" width="450">
</p>
<p align="center">
  <img src="Downloading.png" alt="yt-dlp Modern GUI" width="450">
</p>

## Recursos

- Download de vídeos e playlists com seleção de formato e qualidade
- Fila de downloads concorrentes com cancelamento e repetição
- Histórico de downloads com pesquisa e gerenciamento
- Detecção automática de dependências yt-dlp e FFmpeg com guia de instalação
- Personalização de template de nome de arquivo (modos simples e avançado)
- Suporte a cookies para conteúdo autenticado
- Detecção de downloads duplicados
- Suporte multilíngue (English, 한국어, 日本語, 简体中文, 繁體中文, Français, Deutsch)
- 4 temas de cores (Dark, Violet, Red, Light)
- Suporte multiplataforma (Windows, macOS, Linux)

> **💡 Dica:** O aplicativo baixa automaticamente yt-dlp, FFmpeg e Deno na primeira execução. No entanto, os binários gerenciados automaticamente (empacotados com PyInstaller) podem ser lentos na inicialização. Para obtenção de metadados e downloads **significativamente mais rápidos**, instale-os previamente pelo gerenciador de pacotes do seu sistema — [Homebrew](https://brew.sh/) no macOS (`brew install yt-dlp ffmpeg`), [winget](https://learn.microsoft.com/windows/package-manager/winget/) no Windows (`winget install yt-dlp.yt-dlp ffmpeg`), ou `apt`/`pacman` no Linux. O aplicativo detectará e priorizará automaticamente as versões instaladas no PATH do sistema.

## Compilar a partir do código-fonte

### Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) (última versão stable)
- [Node.js](https://nodejs.org/) (v18+)
- [Bun](https://bun.sh/) (gerenciador de pacotes)
- Dependências específicas da plataforma para [Tauri 2.0](https://v2.tauri.app/start/prerequisites/)

### Passos

```bash
# Clonar o repositório
git clone https://github.com/shlifedev/yt-dlp-modern-gui.git
cd yt-dlp-modern-gui

# Instalar dependências do frontend
bun install

# Executar em modo de desenvolvimento
bun run tauri dev

# Compilar para produção
bun run tauri build
```

A saída da compilação de produção estará em `src-tauri/target/release/bundle/`.

## Roteiro

1. Aplicativo de download para usuários móveis (você pode hospedar seu próprio servidor yt-dlp)
2. Atualizador de versão

## Licença

Este projeto está licenciado sob a [Licença MIT](../LICENSE).
