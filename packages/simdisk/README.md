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

## Phạm vi v1 (q00)

`read(sector)` / `write(sector, data)` / `fsync()` / `crash()` /
`coGhiChuaFsync()` (`co_ghi_chua_fsync()` bên Python) — đủ cho R6-1 q00
"Chiếc hộp giày của Byte" (byte/file/fixed-width record) và khái niệm nền
"ghi chưa fsync có thể biến mất".

## Phạm vi v2 (q02 "Khi điện mất")

Hai kiểu lỗi, CHỦ ĐỘNG kích hoạt (không random — bài học cần tái lập được
y hệt mỗi lần chạy, không phụ thuộc hạt giống ngẫu nhiên):

- `boQuaFsyncKeTiep()` (`bo_qua_fsync_ke_tiep()`) — "lost fsync": lần
  `fsync()` tiếp theo là no-op hoàn toàn, không báo lỗi. `read()` vẫn thấy
  đúng dữ liệu (cache còn nguyên) — lời nói dối chỉ lộ ra khi `crash()`.
- `danhDauTornGhi(sector, soByteThanhCong)` (`danh_dau_torn_ghi(sector,
  so_byte_thanh_cong)`) — "torn write": lần `fsync()` tiếp theo đụng sector
  đó chỉ ghi thành công `soByteThanhCong` byte đầu, phần còn lại giữ dữ
  liệu CŨ (hoặc số 0 nếu sector chưa từng ghi).

Cả hai tự tắt sau đúng một lần `fsync()` bị ảnh hưởng. CRC32 để phát hiện
bản ghi hỏng KHÔNG cần thay đổi gì ở `SimDisk` — đó là việc của định dạng
bản ghi trong lesson (tính/so khớp checksum trên byte đọc được), không
phải của block device.

**CHƯA có**: latent sector error, misdirected write, crash tất định thứ N
— thuộc q18 "Kẻ phá hoại có chủ đích". Xây khi viết tới quest đó, không
xây trước (bài học từ AST-kind ở Realm 4: hạ tầng xây đúng lúc bài đầu
tiên CẦN nó, không sớm hơn).
