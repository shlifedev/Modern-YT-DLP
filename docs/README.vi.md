# yt-dlp Modern GUI


Một ứng dụng máy tính để bàn hiện đại, đa nền tảng để tải xuống video bằng yt-dlp.
Được xây dựng bằng Tauri 2.0 (Rust) và SvelteKit, cung cấp một giao diện sạch sẽ và trực quan để quản lý tải xuống video.

[**English**](../README.md) | [**한국어**](README.ko.md) | [**日本語**](README.ja.md) | [**中文(简体)**](README.zh-CN.md) | [**中文(繁體)**](README.zh-TW.md) | [**Español**](README.es.md) | [**Français**](README.fr.md) | [**Deutsch**](README.de.md) | [**Português**](README.pt-BR.md) | [**Русский**](README.ru.md) | **Tiếng Việt**

## Ảnh chụp màn hình

<p align="center">
  <img src="App.png" alt="yt-dlp Modern GUI" width="450">
</p>
<p align="center">
  <img src="Downloading.png" alt="yt-dlp Modern GUI" width="450">
</p>

## Các tính năng

- Tải xuống video & danh sách phát với lựa chọn định dạng và chất lượng
- Hàng đợi tải xuống song song với tính năng hủy và thử lại
- Lịch sử tải xuống với tính năng tìm kiếm và quản lý
- Phát hiện tự động các phần phụ thuộc yt-dlp và FFmpeg với hướng dẫn cài đặt
- Tùy chỉnh mẫu tên tệp (chế độ đơn giản & nâng cao)
- Hỗ trợ cookie cho nội dung xác thực
- Phát hiện tải xuống trùng lặp
- Hỗ trợ đa ngôn ngữ (English, 한국어, 日本語, 简体中文, 繁體中文, Français, Deutsch)
- 4 chủ đề màu sắc (Dark, Violet, Red, Light)
- Hỗ trợ đa nền tảng (Windows, macOS, Linux)

> **💡 Mẹo:** Ứng dụng tự động tải xuống yt-dlp, FFmpeg và Deno khi khởi chạy lần đầu. Tuy nhiên, các tệp nhị phân được quản lý tự động (đóng gói bằng PyInstaller) có thể khởi động chậm lần đầu. Để truy xuất siêu dữ liệu và tải xuống **nhanh hơn đáng kể**, hãy cài đặt trước qua trình quản lý gói hệ thống — [Homebrew](https://brew.sh/) trên macOS (`brew install yt-dlp ffmpeg`), [winget](https://learn.microsoft.com/windows/package-manager/winget/) trên Windows (`winget install yt-dlp.yt-dlp ffmpeg`), hoặc `apt`/`pacman` trên Linux. Ứng dụng sẽ tự động phát hiện và ưu tiên sử dụng phiên bản đã cài trong PATH hệ thống.

## Biên dịch từ mã nguồn

### Yêu cầu

- [Rust](https://www.rust-lang.org/tools/install) (phiên bản stable mới nhất)
- [Node.js](https://nodejs.org/) (v18+)
- [Bun](https://bun.sh/) (trình quản lý gói)
- Các phụ thuộc theo nền tảng cho [Tauri 2.0](https://v2.tauri.app/start/prerequisites/)

### Các bước

```bash
# Clone kho lưu trữ
git clone https://github.com/shlifedev/yt-dlp-modern-gui.git
cd yt-dlp-modern-gui

# Cài đặt các phụ thuộc frontend
bun install

# Chạy ở chế độ phát triển
bun run tauri dev

# Biên dịch cho môi trường sản xuất
bun run tauri build
```

Kết quả biên dịch sản xuất nằm trong `src-tauri/target/release/bundle/`.

## Lộ trình

1. Ứng dụng tải xuống cho người dùng di động (bạn có thể tự lưu trữ máy chủ yt-dlp của riêng mình)
2. Trình cập nhật phiên bản

## Giấy phép

Dự án này được cấp phép theo [MIT License](../LICENSE).
