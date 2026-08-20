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
