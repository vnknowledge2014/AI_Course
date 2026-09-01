---
id: lap-trinh-ham.bat-bien-thuan-khiet.frozen-true-chan-sua-field-that
title: "`frozen=True` — chặn sửa field THẬT SỰ, không chỉ quy ước"
summary: "@dataclass(frozen=True) — sửa field ném dataclasses.FrozenInstanceError, một chẩn đoán riêng có thể assert được, không phải kiểm bằng convention hay code review."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.frozen-dataclass]
requires: [fp.dataclass-still-mutable]
concepts: [fp.frozen-dataclass]
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
Bài trước để lại một câu hỏi: có tham số nào khiến `@dataclass` chặn hẳn
việc sửa field không? Có đúng một chữ cần thêm.
::::

::::explain{#mot-tham-so-doi-tat-ca}
`@dataclass` nhận tham số ngay trong dấu ngoặc của chính nó:
`@dataclass(frozen=True)`. Cú pháp KHAI FIELD bên trong class không đổi
một chữ nào — vẫn `ten: str`, `gia: int`, y hệt bài trước. Chỉ có đúng
MỘT chỗ khác: dòng decorator.

Nhưng hiệu ứng thì khác hẳn. Với `frozen=True`, MỌI phép gán vào một
field sau khi đối tượng đã được tạo (`mon.gia = 50000`) đều bị chặn
NGAY LẬP TỨC — không phải bằng quy ước, không phải bằng code review, mà
bằng một ngoại lệ riêng: `dataclasses.FrozenInstanceError`. Đây là một
loại lỗi có TÊN, có thể bắt bằng `except`, có thể kiểm bằng `assert` —
khác hẳn việc chỉ hy vọng không ai lỡ tay viết dòng sửa field.

So với `tuple` (chặn bằng cách KHÔNG CÓ phương thức sửa) và
`@dataclass` trần (không chặn gì), `frozen=True` là một cách chặn thứ
ba: field vẫn CÓ đó, đọc được bình thường qua `.ten`, `.gia` — chỉ riêng
việc GÁN LẠI nó là bị từ chối.
::::

::::example{#gan-lai-bi-tu-choi}
Cùng `MonAn`, chỉ thêm `frozen=True` vào decorator:

```python title=readonly
from dataclasses import dataclass

@dataclass(frozen=True)
class MonAn:
    ten: str
    gia: int

mon = MonAn("Bún bò", 45000)
print(f"mon: {mon}")

mon.gia = 50000
```

```text title=readonly
mon: MonAn(ten='Bún bò', gia=45000)
```

```text title=readonly
dataclasses.FrozenInstanceError: cannot assign to field 'gia'
```

Dòng `print(f"mon: {mon}")` chạy bình thường — TẠO đối tượng và ĐỌC
field không hề bị chặn. Chỉ khi chạm tới dòng `mon.gia = 50000` —
GÁN LẠI một field đã có — chương trình mới dừng lại, với đúng thông báo
lỗi ở trên. So với bài trước (cùng đoạn mã, chỉ thiếu `frozen=True`),
đây là khác biệt DUY NHẤT: một chữ trong decorator, một hành vi hoàn
toàn khác lúc chạy.
::::

::::predict{#doan-loai-loi-frozen commitOnce}
Một đơn hàng dùng `@dataclass(frozen=True)`, có hai field.

**Trước khi chạy**, bạn đoán dòng cuối gây ra chuyện gì?

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class DonHang:
    ma_don: str
    trang_thai: str

don = DonHang("DH-88", "cho_xac_nhan")
don.trang_thai = "da_giao"
```

:::opt{correct}
Máy dừng lại, báo `dataclasses.FrozenInstanceError: cannot assign to
field 'trang_thai'`
:::

:::opt
Máy báo `AttributeError: 'DonHang' object has no attribute 'trang_thai'`
::why
Gần đúng ở việc bạn nhớ đúng: CÓ một công cụ bất biến trong track này
ném `AttributeError` — đó là `tuple` (hai bài trước), khi gọi một
PHƯƠNG THỨC nó không có, như `.append()`.

Chỗ lệch: đây không phải gọi phương thức — `don.trang_thai = ...` là
GÁN vào một field ĐÃ CÓ SẴN, của một đối tượng `frozen=True`. Dataclass
đóng băng ném hẳn một loại lỗi RIÊNG cho đúng tình huống này:
`FrozenInstanceError`, không phải `AttributeError`.
::
:::

:::opt
Máy báo `TypeError: 'DonHang' object does not support item assignment`
::why
Gần đúng ở việc bạn nhớ đúng có một `TypeError` khi gán trực tiếp vào
một cấu trúc bất biến — nhưng đó là chuyện của bài `tuple`, khi gán vào
một Ô bằng chỉ số (`size_ly[0] = ...`).

Chỗ lệch: `don.trang_thai = ...` là gán vào một FIELD (dùng dấu chấm),
không phải một Ô (dùng dấu ngoặc vuông). Dataclass đóng băng có lỗi
riêng cho việc gán field: `FrozenInstanceError` — khác chữ với
`TypeError` của `tuple`.
::
:::

:::opt
Không có lỗi nào — chạy trót lọt, vì `frozen=True` chỉ áp dụng lúc TẠO
đối tượng, không áp dụng cho việc gán field về sau
::why
Gần đúng ở việc constructor `DonHang(...)` đúng là vẫn hoạt động bình
thường — `frozen=True` không hề cấm việc TẠO đối tượng.

Chỗ lệch: `frozen=True` chặn CHÍNH XÁC việc gán field SAU KHI đối tượng
đã tồn tại — đó là toàn bộ lý do tham số này tồn tại. Gán
`don.trang_thai = "da_giao"` bị chặn ngay, ném `FrozenInstanceError`.
::
:::
::::

::::code{#dong-bang-mon-an}
Thêm đúng một cụm vào decorator để field của `MonAn` không sửa được
nữa, rồi kiểm tra: gán lại field phải bị chặn bằng
`FrozenInstanceError`.

```python title=starter
from dataclasses import dataclass

___
class MonAn:
    ten: str
    gia: int

mon = MonAn("Bún bò", 45000)
print(f"mon: {mon}")

da_chan_dung = False
ten_loi = None
try:
    mon.gia = 50000
except Exception as loi:
    da_chan_dung = True
    ten_loi = type(loi).__name__

print(f"Bị chặn: {da_chan_dung}")
print(f"Loại lỗi: {ten_loi}")
```

```python title=solution
from dataclasses import dataclass

@dataclass(frozen=True)
class MonAn:
    ten: str
    gia: int

mon = MonAn("Bún bò", 45000)
print(f"mon: {mon}")

da_chan_dung = False
ten_loi = None
try:
    mon.gia = 50000
except Exception as loi:
    da_chan_dung = True
    ten_loi = type(loi).__name__

print(f"Bị chặn: {da_chan_dung}")
print(f"Loại lỗi: {ten_loi}")
```

```python title=test
assert mon.ten == "Bún bò", "ten không được đổi"
assert mon.gia == 45000, "mon.gia phải giữ nguyên 45000 — dòng gán field bên dưới phải bị CHẶN, không chạy trót lọt"
assert da_chan_dung is True, "gán mon.gia = 50000 phải bị chặn bằng một ngoại lệ — thêm frozen=True vào decorator"
assert ten_loi == "FrozenInstanceError", f"lỗi ném ra phải là FrozenInstanceError, đang là {ten_loi}"
```

:::hints
- kind: attention
  body: Chỗ trống là toàn bộ dòng decorator đứng ngay trên class MonAn — không đổi gì bên trong thân class.
- kind: strategy
  body: Decorator @dataclass nhận tham số ngay trong dấu ngoặc của chính nó. Thêm frozen=True vào đó để chặn việc gán lại field sau khi đối tượng đã tạo.
- kind: one-line
  body: 'Điền `@dataclass(frozen=True)` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: lời giải phải khai frozen=True TƯỜNG MINH trong decorator @dataclass của class MonAn — @dataclass trần (không tham số) không đủ, vì nó không chặn sửa field lúc chạy
  requireAst:
  - kind: frozen-dataclass, target: MonAn, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^mon: MonAn\(ten='Bún bò', gia=45000\)\nBị chặn: True\nLoại lỗi: FrozenInstanceError\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một chữ trong decorator, và giờ Python thật sự canh gác — không phải
bạn tự canh gác nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`frozen=True` chặn được việc GÁN LẠI field — nhưng đơn hàng thật thì
luôn cần đổi trạng thái: từ "chờ xác nhận" sang "đã giao". Nếu field
không sửa được nữa, làm sao có được một đối tượng MỚI, mang trạng thái
mới, mà không phải gõ lại từ đầu MỌI field còn lại — kể cả những field
không hề đổi?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
