# M2 — bảy nền tảng: cái nào build được THẬT

Đo ngày 2026-08-30 trên máy đang phát triển (macOS, Apple Silicon). Ghi lại vì
"4/7 build được" tới giờ vẫn là một lời khai, chưa ai chạy lại để xác nhận.

| nền tảng | trạng thái | bằng chứng |
|---|---|---|
| Web — Chrome | ✅ build được | `pnpm run build`: 288 tệp, **0 lỗi**, `vite` ra `dist/` trong 16s |
| Web — Firefox | ✅ build được | cùng một artifact với Chrome; không có nhánh build riêng |
| macOS (desktop) | ✅ build được | `cargo build --release` trên `apps/byte/src-tauri` xong sau 3 phút 4 giây |
| Android | 🔴 **chặn** | khung `gen/android` đã dựng (Gradle, buildSrc đủ), nhưng máy này **không có Android SDK cũng không có NDK** |
| iOS / iPadOS | 🔴 **chặn** | khung `gen/apple` đã dựng (`byte-app.xcodeproj`, `ExportOptions.plist`), nhưng `security find-identity -v -p codesigning` trả về **`0 valid identities found`** |
| Linux | 🔴 **chặn** | chưa có máy đích để build và thử thật |
| Windows | 🔴 **chặn** | target `x86_64-pc-windows-msvc` đã cài, nhưng thiếu trình liên kết MSVC — không cross-build từ macOS được |

**Đếm cho đúng: 3 artifact chạy được, phủ 3/7 nền tảng.** Con số "4/7" trước đây
đếm Chrome và Firefox thành hai lần build, trong khi chúng dùng chung đúng một
bản `dist/`. Nếu đếm theo *nền tảng người học mở ra* thì là 4; nếu đếm theo
*thứ phải dựng và phải thử riêng* thì là 3.

## Bốn cái còn lại cần gì

Không cái nào là việc viết thêm mã. Cả bốn đều cần thứ ở ngoài repo:

- **Android** — cài Android SDK + NDK trên máy build. Rust target đã sẵn
  (`aarch64-linux-android`, `armv7-linux-androideabi`, `i686-linux-android`,
  `x86_64-linux-android` đều đã cài).
- **iOS / iPadOS** — một tài khoản Apple Developer, để có chứng chỉ ký. Rust
  target đã sẵn (`aarch64-apple-ios`, `aarch64-apple-ios-sim`,
  `x86_64-apple-ios`).
- **Linux** — một máy Linux (hoặc container) để build và thử.
- **Windows** — một máy Windows, hoặc một chuỗi cross-build có trình liên kết
  MSVC.

Ghi rõ ở đây để lần sau không ai phải đoán lại: **M2 không bị chặn bởi mã, nó
bị chặn bởi máy và giấy tờ.**
