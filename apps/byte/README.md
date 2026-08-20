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

| Nền tảng | Lệnh | Trạng thái |
|---|---|---|
| Chrome, Firefox | `pnpm --filter @byte/app build` | ✅ chạy |
| macOS | `pnpm --filter @byte/app tauri build` | ✅ `Byte.app` 4 MB |
| Linux, Windows | cùng lệnh, chạy trên máy đích | vỏ đã sẵn, chưa build thử |
| iOS, iPadOS | `pnpm --filter @byte/app tauri ios init` rồi `ios build` | cần Xcode (đã có) |
| Android | `pnpm --filter @byte/app tauri android init` rồi `android build` | cần Android SDK + NDK |

Cùng một bản web nằm trong cả bảy. Vỏ Tauri chỉ thêm đúng một thứ: gọi
`byte-rust` NATIVE thay vì qua WASM — cùng hàm `kiem_va_chay`, nên kết quả
chấm bài giống hệt nhau ở mọi nền tảng. Một bài đạt trên Mac mà trượt trên
Android thì người học không còn tin công cụ nữa.

## Chạy Python

CPython thật, qua Pyodide biên dịch WASM — không phải bản mô phỏng. Người học
phải gặp đúng thông báo lỗi, đúng hành vi số học, đúng thứ tự `dict` mà họ sẽ
gặp khi rời khỏi ứng dụng này.

Mã chạy trong Worker riêng vì JavaScript không có cách nào ngắt một vòng lặp
từ bên trong. Người mới viết `while True:` là chuyện thường, và cách duy nhất
chắc chắn dừng được là giết cả worker từ bên ngoài.
