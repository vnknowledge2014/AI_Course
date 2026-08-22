# @byte/app — trình đọc bài học

Ứng dụng học liệu. Web trước (Chrome/Firefox), vỏ Tauri cho desktop và mobile
sẽ bọc chính bản build này.

```sh
pnpm --filter @byte/app noi-dung   # sinh nội dung từ content/ → public/noi-dung
pnpm --filter @byte/app dev        # http://localhost:5273
```

`public/noi-dung/` là **sản phẩm sinh ra**, không commit. Nguồn duy nhất của
nội dung là `content/**/*.lesson.md`; chạy `noi-dung` mỗi khi sửa bài.

## Vì sao nội dung là file tĩnh chứ không phải API

Ứng dụng phải chạy được khi mất mạng. Người học ngồi quán cà phê rớt wifi
không được mất bài đang học giữa chừng — và trên bản đóng gói Tauri thì không
có máy chủ nào để gọi cả.

## Bảy nền tảng, một bản build

| Nền tảng | Trạng thái | Lệnh |
|---|---|---|
| Chrome, Firefox | ✅ build được | `pnpm --filter @byte/app build` |
| macOS | ✅ `Byte.app` 4 MB | `pnpm --filter @byte/app tauri build` |
| Android | ✅ APK 12 MB | `tauri android build --apk` |
| iOS, iPadOS | ⚠️ mã biên dịch được, **chưa đóng gói được** | xem dưới |
| Linux, Windows | vỏ đã sẵn, phải build trên máy đích | cùng lệnh macOS |

Cùng một bản web nằm trong cả bảy. Vỏ Tauri chỉ thêm đúng một thứ: gọi
`byte-rust` NATIVE thay vì qua WASM — cùng hàm `kiem_va_chay`, nên kết quả
chấm bài giống hệt nhau ở mọi nền tảng. Một bài đạt trên Mac mà trượt trên
Android thì người học không còn tin công cụ nữa, và đó là thứ không lấy lại
được.

### iOS còn thiếu gì

Đã xong: dự án Xcode sinh ra, runtime simulator iOS 26.5 tải về, và mã Rust
biên dịch được cho cả `aarch64-apple-ios` lẫn `aarch64-apple-ios-sim`.

Chặn ở đúng một chỗ: **`Signing for "byte-app_iOS" requires a development
team`**. `security find-identity -v -p codesigning` trả về `0 valid
identities`. Cần một tài khoản Apple Developer — không có cách nào vòng qua,
kể cả cho bản simulator: `tauri ios build` luôn archive, và archive thì đòi ký.

Khi đã có tài khoản:

```sh
# Lấy Team ID ở https://developer.apple.com/account → Membership
export APPLE_DEVELOPMENT_TEAM=XXXXXXXXXX
pnpm --filter @byte/app tauri ios build
```

Hoặc ghi cố định vào `src-tauri/tauri.conf.json`:

```json
{ "bundle": { "iOS": { "developmentTeam": "XXXXXXXXXX" } } }
```

Dựng project Xcode độc lập bằng `xcodebuild` KHÔNG thay thế được: bước
"Build Rust Code" trong project cần Tauri CLI làm tiến trình cha (nó nói
chuyện qua một WebSocket cục bộ), nên chạy ngoài `tauri ios build` sẽ hỏng ở
đúng bước đó.

## Chạy Python

CPython thật, qua Pyodide biên dịch WASM — không phải bản mô phỏng. Người học
phải gặp đúng thông báo lỗi, đúng hành vi số học, đúng thứ tự `dict` mà họ sẽ
gặp khi rời khỏi ứng dụng này.

Mã chạy trong Worker riêng vì JavaScript không có cách nào ngắt một vòng lặp
từ bên trong. Người mới viết `while True:` là chuyện thường, và cách duy nhất
chắc chắn dừng được là giết cả worker từ bên ngoài.
