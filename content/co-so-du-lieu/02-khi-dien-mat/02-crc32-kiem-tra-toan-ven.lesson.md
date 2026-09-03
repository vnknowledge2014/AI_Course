---
id: co-so-du-lieu.khi-dien-mat.crc32-kiem-tra-toan-ven
title: CRC32 kiểm tra toàn vẹn
summary: "Thêm header 4-byte CRC32 (zlib.crc32) TRƯỚC phần thân bản ghi — đọc lại tính LẠI checksum, so khớp với checksum đã lưu. Khớp thì dữ liệu CÒN nguyên vẹn; lệch dù chỉ MỘT byte thì dữ liệu đã hỏng — không cần biết hỏng CHỖ nào, chỉ cần biết CÓ hỏng."
locale: vi
track: co-so-du-lieu
module: khi-dien-mat
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.crc32-integrity]
requires: [db.wal-ordering]
concepts: [db.crc32-integrity]
gradingMatrix:
  web-chrome: [static, run, tests, output]
  web-firefox: [static, run, tests, output]
  macos: [static, run, tests, output]
  windows: [static, run, tests, output]
  linux: [static, run, tests, output]
  android: [static, run, tests, output]
  ios: [static, run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Một bản ghi ĐÃ fsync — NHƯNG LÀM sao BIẾT chắc TỪNG byte của nó VẪN
còn ĐÚNG NHƯ lúc ghi, chứ KHÔNG bị hỏng MỘT phần?
::::

::::explain{#them-header-crc32}
Thêm một header `4` byte **CRC32** (một con SỐ "chữ ký" tính TỪ nội
DUNG, thư viện chuẩn `zlib`) TRƯỚC toàn bộ THÂN bản ghi:

```python title=readonly
import zlib

def ghi_kv_crc(khoa, gia_tri, kich_thuoc):
    than = len(khoa).to_bytes(4, 'big') + khoa + len(gia_tri).to_bytes(4, 'big') + gia_tri
    crc = zlib.crc32(than).to_bytes(4, 'big')
    return (crc + than).ljust(kich_thuoc, b'\x00')


rec = ghi_kv_crc(b'id', b'7', 28)
print(rec)
```

```text title=readonly
b'\x05Of\xa5\x00\x00\x00\x02id\x00\x00\x00\x017\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
```

`than` LÀ đúng phần THÂN quen thuộc (q01: `len_khoá + khoá +
len_giá_trị + giá_trị`) — `zlib.crc32(than)` TÍNH ra một con số DUY
NHẤT phụ thuộc VÀO từng byte của `than`, mã HOÁ thành `4` byte
(`to_bytes(4, 'big')`, bài 1 CỦA q00), đặt Ở **TRƯỚC** thân.
::::

::::example{#doc-lai-va-kiem-crc}
Đọc lại: TÍNH LẠI CRC32 TRÊN cùng phần thân ĐÃ đọc được, SO khớp
VỚI checksum đã LƯU trong header:

```python title=readonly
import zlib

def doc_kv_crc(sector_bytes):
    crc_luu = sector_bytes[0:4]
    do_dai_khoa = int.from_bytes(sector_bytes[4:8], 'big')
    khoa = sector_bytes[8:8 + do_dai_khoa]
    vi_tri = 8 + do_dai_khoa
    do_dai_gia_tri = int.from_bytes(sector_bytes[vi_tri:vi_tri + 4], 'big')
    gia_tri = sector_bytes[vi_tri + 4:vi_tri + 4 + do_dai_gia_tri]
    than = sector_bytes[4:vi_tri + 4 + do_dai_gia_tri]
    hop_le = zlib.crc32(than).to_bytes(4, 'big') == crc_luu
    return (khoa, gia_tri, hop_le)


rec = b'\x05Of\xa5\x00\x00\x00\x02id\x00\x00\x00\x017\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
print(doc_kv_crc(rec))
```

```text title=readonly
(b'id', b'7', True)
```

`crc_luu` LÀ `4` byte ĐẦU (checksum lúc GHI), `than` LÀ phần CÒN
lại (tính LẠI TỪ chính byte đọc ĐƯỢC). `hop_le = True` — checksum
TÍNH lại KHỚP checksum đã lưu, nghĩa LÀ `than` KHÔNG hề đổi kể TỪ
lúc ghi.
::::

::::predict{#doan-mot-byte-bi-hong commitOnce}
MỘT byte TRONG record bị hỏng (giả LẬP torn write — bài SAU sẽ THẤY
CÁCH thật) — ĐÚNG một byte, giữa CHỪNG phần thân:

```python
rec = bytearray(b'\x05Of\xa5\x00\x00\x00\x02id\x00\x00\x00\x017\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00')
rec[10] ^= 0xFF   # lat NGUOC tam bit CUA dung mot byte
print(doc_kv_crc(bytes(rec))[2])
```

Dòng cuối in ra gì (`hop_le` — phần TỬ thứ BA của tuple)?

:::opt{correct}
`False`
:::

:::opt
`True` — vì CHỈ đổi ĐÚNG một byte TRONG số `28` byte, sai LỆCH quá
NHỎ để CRC32 phát hiện RA
::why
Gần đúng ở việc bạn nghĩ TỚI "sai lệch NHỎ" như một mối lo HỢP lý
— trực GIÁC "đổi ít THÌ ảnh hưởng ít".

Chỗ lệch: CRC32 được thiết KẾ để nhạy VỚI **bất KỲ** thay đổi NÀO,
dù CHỈ một BIT — đổi MỘT byte làm checksum TÍNH lại gần NHƯ chắc
chắn khác checksum đã LƯU (xác suất TRÙNG NGẪU nhiên cực NHỎ). Đây
chính LÀ lý do CRC32 hữu ÍCH: KHÔNG "gần đúng LÀ được", CHỈ khớp
tuyệt đối HAY không khớp CHÚT nào.
::
:::

:::opt
Máy báo lỗi — vì `doc_kv_crc` KHÔNG xử lý được dữ liệu ĐÃ hỏng
::why
Gần đúng ở việc bạn nghĩ TỚI hệ QUẢ hợp lý của dữ liệu HỎNG — MỘT
lo ngại chính ĐÁNG.

Chỗ lệch: `doc_kv_crc` vẫn chạy BÌNH thường TRÊN dữ liệu hỏng — cắt
lát VÀ giải mã KHÔNG hề kiểm TRA "hợp lý" (như q00/q01 đã dạy),
CHỈ có `hop_le` LÀ tín hiệu RÕ ràng nói "đừng TIN dữ liệu này",
KHÔNG có exception NÀO được ném.
::
:::
::::

::::code{#viet_doc_kv_crc}
Hoàn thiện `doc_kv_crc(sector_bytes)` — tính `hop_le` bằng cách SO
khớp CRC32 tính LẠI với checksum đã LƯU.

```python title=starter
import zlib


def doc_kv_crc(sector_bytes):
    crc_luu = sector_bytes[0:4]
    do_dai_khoa = int.from_bytes(sector_bytes[4:8], 'big')
    khoa = sector_bytes[8:8 + do_dai_khoa]
    vi_tri = 8 + do_dai_khoa
    do_dai_gia_tri = int.from_bytes(sector_bytes[vi_tri:vi_tri + 4], 'big')
    gia_tri = sector_bytes[vi_tri + 4:vi_tri + 4 + do_dai_gia_tri]
    than = sector_bytes[4:vi_tri + 4 + do_dai_gia_tri]
    ___
    return (khoa, gia_tri, hop_le)


rec = b'\x05Of\xa5\x00\x00\x00\x02id\x00\x00\x00\x017\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
print(doc_kv_crc(rec))
```

```python title=solution
import zlib


def doc_kv_crc(sector_bytes):
    crc_luu = sector_bytes[0:4]
    do_dai_khoa = int.from_bytes(sector_bytes[4:8], 'big')
    khoa = sector_bytes[8:8 + do_dai_khoa]
    vi_tri = 8 + do_dai_khoa
    do_dai_gia_tri = int.from_bytes(sector_bytes[vi_tri:vi_tri + 4], 'big')
    gia_tri = sector_bytes[vi_tri + 4:vi_tri + 4 + do_dai_gia_tri]
    than = sector_bytes[4:vi_tri + 4 + do_dai_gia_tri]
    hop_le = zlib.crc32(than).to_bytes(4, 'big') == crc_luu
    return (khoa, gia_tri, hop_le)


rec = b'\x05Of\xa5\x00\x00\x00\x02id\x00\x00\x00\x017\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
print(doc_kv_crc(rec))
```

```python title=test
def ghi_kv_crc(khoa, gia_tri, kich_thuoc):
    than = len(khoa).to_bytes(4, 'big') + khoa + len(gia_tri).to_bytes(4, 'big') + gia_tri
    crc = zlib.crc32(than).to_bytes(4, 'big')
    return (crc + than).ljust(kich_thuoc, b'\x00')

rec1 = ghi_kv_crc(b'id', b'7', 28)
assert doc_kv_crc(rec1) == (b'id', b'7', True), "ban ghi nguyen ven -- hop_le True"

rec2 = bytearray(rec1)
rec2[10] ^= 0xFF
assert doc_kv_crc(bytes(rec2))[2] is False, "mot byte hong -- hop_le False"

rec3 = ghi_kv_crc(b'ten', b'Byte', 28)
assert doc_kv_crc(rec3) == (b'ten', b'Byte', True), "ban ghi khac van dung"
```

:::hints
- kind: attention
  body: "Tinh zlib.crc32(than), doi ve 4 byte, so sanh voi crc_luu."
- kind: strategy
  body: "hop_le = zlib.crc32(than).to_bytes(4, 'big') == crc_luu"
- kind: one-line
  body: "hop_le = zlib.crc32(than).to_bytes(4, 'big') == crc_luu"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tinh zlib.crc32(than) roi so sanh voi crc_luu
  requireAst:
  - kind: uses-name, target: than, min: 1
  - kind: uses-name, target: crc_luu, min: 1
  - kind: uses-call, target: crc32, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(b'id', b'7', True\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
CRC32 phát hiện hỏng — KHÔNG cần biết hỏng chỗ NÀO, chỉ cần biết CÓ
hỏng. Byte muốn THẤY chính XÁC điều GÌ tạo ra một byte hỏng NHƯ vậy.
::::

::::reflect{#nghi-lai}
CRC32 — thêm header `4` byte, tính LẠI lúc đọc, so KHỚP — phát hiện
BẤT kỳ thay đổi nào, dù CHỈ một bit, KHÔNG cần biết hỏng Ở đâu. Ví
dụ VỪA rồi tự "lật BIT" một byte để mô phỏng hỏng — NHƯNG trong THỰC
tế, điều GÌ khiến MỘT byte thật sự đổi NHƯ vậy giữa lúc GHI?
::::

::::checkpoint{mastery=0.8}
::::
