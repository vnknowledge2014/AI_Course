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

## Phạm vi v3 (q18 "Kẻ phá hoại có chủ đích")

Ba kiểu lỗi/công cụ nữa, TypeScript only (R6-3 không có bản Python song
sinh — pillar "TigerBeetle = correctness" viết TypeScript xuyên suốt):

- `danhDauLoiSectorAn(sector)` — "latent sector error": MỌI `read()` tiếp
  theo trên sector đó ném lỗi, CHO TỚI KHI sector được ghi lại VÀ `fsync()`
  thành công (reallocate-on-rewrite, đúng hành vi đĩa thật). KHÔNG tự tắt
  theo số lần gọi như hai lỗi v2 — là một TRẠNG THÁI của sector, không phải
  một bẫy một lần.
- `danhDauGhiSaiDich(sectorDinhGhi, sectorThucTe)` — "misdirected write":
  lần `fsync()` tiếp theo, dữ liệu định ghi cho `sectorDinhGhi` lại ghi vào
  `sectorThucTe` — `sectorDinhGhi` giữ nguyên dữ liệu cũ, `sectorThucTe` bị
  ghi đè nhầm. Nguy hiểm hơn torn write vì dữ liệu (và checksum của nó, nếu
  có) đều "đúng" — chỉ SAI vị trí, checksum-trên-từng-sector không bắt được.
- `datCrashSauThaoTacThuN(n)` — sau đúng `n` lần gọi `write()` kể từ lúc
  gọi hàm này, `crash()` TỰ ĐỘNG kích hoạt — "crash tất định thứ N": thời
  điểm crash do một con số quyết định (tính được từ seed một vũ trụ tất
  định, xem `content/co-so-du-lieu/17-vu-tru-tat-dinh`), không phải một độ
  trễ thời gian thật hay nguồn ngẫu nhiên nào.

Test đầy đủ ở `test/simdisk.test.mjs` (chạy qua `pnpm test` ở đây, hoặc
`pnpm -r --silent test` ở gốc repo — gate "Test JS" trong `tools/cong.sh`).
