# @byte/simdisk

Block device mô phỏng cho Realm 6 (Database). Xem MASTERPLAN.md §4 mục
"SimDisk" và §9.2 (Realm 6) cho bối cảnh đầy đủ.

## Vì sao gói này tồn tại

Pyodide chạy trên MEMFS trong RAM: `os.fsync()` ở đó là no-op, không có khái
niệm sector, và khi Worker `terminate()` thì MEMFS mất sạch — nghĩa là "crash"
không để lại gì để phục hồi, và không có torn write / latent sector error nào
để dạy. Mọi bài storage của Realm 6 lập trình trên `SimDisk`, **không** trên
`open()` thật.

## Hai bản song sinh

- `src/index.ts` — bản tham chiếu TypeScript, có `dist/` biên dịch + test
  (`pnpm test`, chạy trong `pnpm -r test` ở gốc repo).
- `py/simdisk.py` — bản Python cùng ngữ nghĩa, dùng cho các bài Python (vd
  R6-1 q00, gắn nhãn `[PY, SimDisk]`).

**Bài học KHÔNG `import` gói này lúc chạy** — sandbox chấm bài (Pyodide/WASM)
chạy mỗi bài độc lập, không nạp được gói workspace ngoài. Mỗi bài tự chép
phần class cần dùng vào khối `readonly` của chính nó, đúng quy ước
không-import-chéo-giữa-các-bài đã dùng xuyên suốt dự án (vd `chia_co_du`/
`dong_du` ở T2.6, `cong_vector` ở T2.7). Gói này là **nguồn sự thật** để đối
chiếu ngữ nghĩa lúc viết bài — khi một lesson định nghĩa lại `SimDisk` (hay
một phần của nó) trong `readonly`, hành vi phải khớp với gói này, và có thể
kiểm bằng cách chạy test ở đây trước.

## Phạm vi v1

`read(sector)` / `write(sector, data)` / `fsync()` / `crash()` /
`coGhiChuaFsync()` (`co_ghi_chua_fsync()` bên Python) — đủ cho R6-1 q00
"Chiếc hộp giày của Byte" (byte/file/fixed-width record) và khái niệm nền
"ghi chưa fsync có thể biến mất".

**CHƯA có fault injection** (torn write theo ranh giới sector, fsync bị mất
mà không báo, latent sector error, misdirected write, crash tất định thứ N)
— những thứ này thuộc q02 "Khi điện mất" và q18 "Kẻ phá hoại có chủ đích".
Xây khi viết tới quest cần dùng, không xây trước — tránh đoán sai hình dạng
API rồi phải sửa lại giữa chừng (bài học từ AST-kind ở Realm 4: hạ tầng xây
đúng lúc bài đầu tiên CẦN nó, không sớm hơn).
