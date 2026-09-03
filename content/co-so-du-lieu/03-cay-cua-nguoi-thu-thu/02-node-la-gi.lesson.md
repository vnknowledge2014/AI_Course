---
id: co-so-du-lieu.cay-cua-nguoi-thu-thu.node-la-gi
title: Node là gì
summary: "Một NODE lá chứa NHIỀU cặp khoá-giá trị đã sắp xếp trong MỘT sector — không còn 'một bản ghi = một sector' (q00-q02) mà 'nhiều bản ghi = một node'. Mã hoá bằng JSON (json.dumps + đệm khoảng trắng) thay vì đóng gói byte thủ công — trọng tâm q03 LÀ cấu trúc cây, byte-packing đã thành thạo."
locale: vi
track: co-so-du-lieu
module: cay-cua-nguoi-thu-thu
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [db.node-format]
requires: [db.why-a-tree]
concepts: [db.node-format]
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
q00-q02: MỘT sector CHỨA đúng MỘT bản ghi. Một NODE cây CHỨA nhiều
khoá CÙNG lúc — MỘT node TRÔNG như thế nào?
::::

::::explain{#node-la-mot-dict}
Một node LÁ LÀ một `dict` Python: `loai` (đánh dấu ĐÂY là lá),
`khoa` (danh SÁCH khoá đã SẮP xếp), `gia_tri` (danh sách GIÁ trị
tương ỨNG), `la_tiep` (con TRỎ tới lá KẾ tiếp — dùng SAU). Mã hoá
BẰNG `json.dumps` THAY vì đóng gói byte thủ CÔNG (q00-q02):

```python title=readonly
import json


def ma_hoa_node(node, kich_thuoc):
    d = json.dumps(node).encode('utf-8')
    if len(d) > kich_thuoc:
        raise ValueError(f'node qua lon: {len(d)} > {kich_thuoc}')
    return d.ljust(kich_thuoc, b' ')


la = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
enc = ma_hoa_node(la, 128)
print(enc)
print(len(enc))
```

```text title=readonly
b'{"loai": "la", "khoa": [3, 7, 9], "gia_tri": ["ba", "bay", "chin"], "la_tiep": null}                                            '
128
```

`json.dumps(node).encode('utf-8')` biến CẢ dict THÀNH byte MỘT
lần (thay VÌ ghép TỪNG trường như `ghi_kv` Ở q01) — RỒI `.ljust`
đệm KHOẢNG trắng (`b' '`, KHÔNG phải `b'\x00'` — JSON chữ KHÔNG bao
GIỜ tự chứa khoảng TRẮNG thừa Ở cuối) cho ĐỦ `kich_thuoc`.
::::

::::example{#giai-ma-lai}
Giải mã NGƯỢC: bỏ khoảng TRẮNG đệm, decode UTF-8, `json.loads`:

```python title=readonly
def giai_ma_node(sector_bytes):
    return json.loads(sector_bytes.rstrip(b' ').decode('utf-8'))


print(giai_ma_node(enc))
print(giai_ma_node(enc) == la)
```

```text title=readonly
{'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
True
```

`giai_ma_node(enc) == la` — Y HỆT node GỐC, kể cả `la_tiep: None`
(JSON `null` giải mã LẠI đúng thành Python `None`).
::::

::::predict{#doan-node-qua-lon commitOnce}
Byte thử mã hoá MỘT node CÓ quá nhiều khoá VÀO một sector QUÁ nhỏ
(`kich_thuoc=20`, KHÔNG đủ chỗ CHO cả `3` khoá):

```python
la = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
ma_hoa_node(la, 20)
```

Dòng cuối XẢY ra điều gì?

:::opt{correct}
Máy báo lỗi (`ValueError`)
:::

:::opt
Trả VỀ đúng `20` byte — JSON tự ĐỘNG cắt bớt CÁC khoá THỪA cho vừa
KÍCH thước
::why
Gần đúng ở việc bạn nghĩ TỚI một cơ chế "TỰ điều chỉnh" hợp lý khi
dữ liệu QUÁ lớn — MỘT kỳ vọng dễ hiểu SAU khi thấy `[:N]` cắt bớt
Ở nhiều bài TRƯỚC (q00-q01).

Chỗ lệch: `ma_hoa_node` CÓ kiểm tra `len(d) > kich_thuoc` RÕ ràng
VÀ chủ động NÉM `ValueError` — nó KHÔNG hề "âm thầm cắt BỚT" như
`[:N]` từng LÀM, vì cắt BỚT một node ĐÃ mã hoá SẼ làm hỏng CẤU trúc
JSON (mất dấu `}` đóng), KHÔNG còn giải MÃ lại được.
::
:::

:::opt
Trả VỀ node CŨ, KHÔNG lưu gì cả — im lặng bỏ QUA
::why
Gần đúng ở việc bạn nghĩ TỚI một cách xử LÝ "an toàn, im lặng" khi
GẶP lỗi — một LỰA chọn thiết kế hợp LÝ cho một SỐ hệ thống.

Chỗ lệch: `ma_hoa_node` CHỦ động báo lỗi NGAY LẬP tức (`raise
ValueError`), KHÔNG lặng lẽ "bỏ qua" — người GỌI hàm BIẾT ngay có
vấn ĐỀ, thay VÌ phát hiện SAU khi dữ liệu ĐÃ mất MÀ không hay.
::
:::
::::

::::code{#viet_ma_hoa_node}
Hoàn thiện `ma_hoa_node(node, kich_thuoc)` — đệm KHOẢNG trắng cho
ĐỦ `kich_thuoc` sau khi kiểm TRA kích thước.

```python title=starter
import json


def ma_hoa_node(node, kich_thuoc):
    d = json.dumps(node).encode('utf-8')
    if len(d) > kich_thuoc:
        raise ValueError(f'node qua lon: {len(d)} > {kich_thuoc}')
    ___


def giai_ma_node(sector_bytes):
    return json.loads(sector_bytes.rstrip(b' ').decode('utf-8'))


la = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
enc = ma_hoa_node(la, 128)
print(len(enc))
```

```python title=solution
import json


def ma_hoa_node(node, kich_thuoc):
    d = json.dumps(node).encode('utf-8')
    if len(d) > kich_thuoc:
        raise ValueError(f'node qua lon: {len(d)} > {kich_thuoc}')
    return d.ljust(kich_thuoc, b' ')


def giai_ma_node(sector_bytes):
    return json.loads(sector_bytes.rstrip(b' ').decode('utf-8'))


la = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
enc = ma_hoa_node(la, 128)
print(len(enc))
```

```python title=test
la = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
enc = ma_hoa_node(la, 128)
assert len(enc) == 128, "dem dung 128 byte"
assert giai_ma_node(enc) == la, "giai ma lai dung y het"

la2 = {'loai': 'la', 'khoa': [1], 'gia_tri': ['mot'], 'la_tiep': 5}
enc2 = ma_hoa_node(la2, 64)
assert len(enc2) == 64, "kich thuoc khac van dung"
assert giai_ma_node(enc2) == la2, "la_tiep la so nguyen van dung"

try:
    ma_hoa_node(la, 10)
    raise AssertionError("phai nem ValueError")
except ValueError:
    pass
```

:::hints
- kind: attention
  body: "Dung d.ljust(kich_thuoc, b' ') de dem cho du kich thuoc."
- kind: strategy
  body: "return d.ljust(kich_thuoc, b' ')"
- kind: one-line
  body: "return d.ljust(kich_thuoc, b' ')"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tra ve d.ljust(kich_thuoc, b' ')
  requireAst:
  - kind: uses-name, target: d, min: 3
  - kind: uses-name, target: kich_thuoc, min: 3
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^128\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một node ĐÃ mã hoá VÀ giải mã đúng — chứa NHIỀU khoá CÙNG lúc.
NHƯNG làm sao TÌM một khoá CỤ thể TRONG số nhiều khoá ĐÓ?
::::

::::reflect{#nghi-lai}
Một node LÁ LÀ một `dict` MÃ hoá bằng JSON — chứa NHIỀU khoá đã sắp
xếp TRONG một sector, KHÁC hẳn "một sector MỘT bản ghi" (q00-q02).
`ma_hoa_node` báo lỗi RÕ ràng khi node QUÁ lớn, KHÔNG âm thầm cắt
bớt. Node NÀY chứa `3` khoá — LÀM sao tìm ĐÚNG một khoá CỤ thể
TRONG số đó, KHÔNG cần đọc lại TỪ đầu MỖI lần?
::::

::::checkpoint{mastery=0.8}
::::
