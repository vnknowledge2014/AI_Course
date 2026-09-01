---
id: lap-trinh-ham.bat-bien-thuan-khiet.ghep-du-lieu-bat-bien-thanh-mot-ham
title: "Ghép một hàm nhận và trả về dữ liệu bất biến nhiều tầng"
summary: "Một frozen dataclass chứa một tuple các frozen dataclass khác — đổi một phần lồng sâu bên trong bằng replace() ở NHIỀU TẦNG: tầng trong đổi từng món, tầng ngoài ráp một tuple mới. Không khái niệm mới, chỉ ghép frozen=True, replace(), và tuple lại với nhau."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.compose-immutable]
requires: [fp.deepcopy-vs-copy]
concepts: [fp.compose-immutable]
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
Không cần khái niệm mới nào cả. Bạn chỉ cần ráp lại đúng những gì đã
học — ở hai tầng cùng lúc.
::::

::::explain{#hai-tang-hai-lan-replace}
Một đơn hàng có nhiều món. Viết bằng đúng công cụ track này đã dạy: một
`@dataclass(frozen=True)` cho `DonHang`, field `mon` kiểu `tuple` (né
hẳn bẫy bài 15 — không `list` nào cả), chứa BÊN TRONG nó nhiều
`MonAn`, mỗi `MonAn` cũng là `@dataclass(frozen=True)`.

```python
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class MonAn:
    ten: str
    gia: int

@dataclass(frozen=True)
class DonHang:
    ma_don: str
    mon: tuple
```

Muốn đổi giá MỘT món bên trong đơn hàng — không sửa gì tại chỗ, không
`deepcopy()` cả đơn hàng chỉ để đổi một con số — cần đúng HAI lần
`replace()`, ở hai TẦNG khác nhau:

- **Tầng TRONG**: với món cần đổi, `replace(mon, gia=gia_moi)` tạo một
  `MonAn` MỚI, giá đã đổi, tên giữ nguyên. Món KHÔNG cần đổi thì giữ
  nguyên, không tạo gì cả.
- **Tầng NGOÀI**: ráp lại một `tuple` MỚI chứa mọi món (đổi lẫn không
  đổi), rồi `replace(don, mon=tuple_moi)` tạo một `DonHang` MỚI, chỉ
  field `mon` đổi, `ma_don` giữ nguyên.

Không có động từ SỬA nào ở đây — hai lần `replace()`, một lần ráp
`tuple`, toàn TẠO MỚI. Đây chính là bài 5, 6, 14, 15 ghép lại thành một
hàm.
::::

::::example{#doi-gia-tra-da}
```python title=readonly
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class MonAn:
    ten: str
    gia: int

@dataclass(frozen=True)
class DonHang:
    ma_don: str
    mon: tuple

def doi_gia(don, ten_mon, gia_moi):
    mon_moi = tuple(
        replace(m, gia=gia_moi) if m.ten == ten_mon else m
        for m in don.mon
    )
    return replace(don, mon=mon_moi)

don = DonHang("DH-9", (MonAn("Phở", 45000), MonAn("Trà đá", 5000)))
don_moi = doi_gia(don, "Trà đá", 8000)

print(f"don:     {don}")
print(f"don_moi: {don_moi}")
```

```text title=readonly
don:     DonHang(ma_don='DH-9', mon=(MonAn(ten='Phở', gia=45000), MonAn(ten='Trà đá', gia=5000)))
don_moi: DonHang(ma_don='DH-9', mon=(MonAn(ten='Phở', gia=45000), MonAn(ten='Trà đá', gia=8000)))
```

`don` không hề đổi — cả hai món của nó vẫn nguyên giá cũ. `don_moi` là
một `DonHang` khác hẳn, với đúng "Trà đá" đổi giá, còn "Phở" — món
KHÔNG khớp `ten_mon` — giữ nguyên nguyên vẹn, kể cả GIỮ NGUYÊN CÙNG một
`MonAn` (nhánh `else m` trong hàm không tạo gì mới, chỉ trả lại chính
`m`).
::::

::::predict{#doan-sua-truc-tiep commitOnce}
Một cách viết KHÁC cho `doi_gia` — thử sửa `gia` ngay trên món cũ, thay
vì gọi `replace()`.

**Trước khi chạy**, bạn đoán dòng cuối gây ra chuyện gì?

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class MonAn:
    ten: str
    gia: int

m = MonAn("Trà đá", 5000)
m.gia = 8000
```

:::opt{correct}
Máy dừng lại, báo `dataclasses.FrozenInstanceError: cannot assign to
field 'gia'`
:::

:::opt
Chạy trót lọt — `m.gia` đổi thành `8000`, vì đây là gán vào MỘT FIELD
đơn lẻ, không phải sửa cả object
::why
Gần đúng ở việc bạn tin field lẻ tẻ thì "nhẹ" hơn — trực giác đó dễ
hiểu, nhưng không đúng với `frozen=True`.

Chỗ lệch: `frozen=True` canh gác MỌI field, không phân biệt "nặng" hay
"nhẹ". `m.gia = 8000` là GÁN LẠI field `gia` của một object đã tạo —
đúng chính xác việc `frozen=True` sinh ra để chặn. Nó dừng lại ngay,
không có ngoại lệ nào cho việc "chỉ đổi một field".
::
:::

:::opt
Máy báo `AttributeError: 'MonAn' object has no attribute 'gia'`
::why
Gần đúng ở việc bạn nhớ đúng có `AttributeError` xuất hiện đâu đó trong
track này — đúng, với `tuple`/`frozenset` khi gọi phương thức không
tồn tại.

Chỗ lệch: `MonAn` CÓ field `gia` — nó đọc được bình thường
(`m.gia` trả về `5000` trước dòng cuối). Vấn đề không phải "không có
field này", mà là "field này có rồi, nhưng bị `frozen=True` cấm GÁN
LẠI". Đúng loại lỗi là `FrozenInstanceError`, không phải
`AttributeError`.
::
:::

:::opt
Không lỗi gì, nhưng `m` vẫn giữ `gia=5000` — phép gán bị âm thầm bỏ qua
::why
Gần đúng ở việc bạn tin đúng: `m.gia` phải giữ nguyên `5000` sau dòng
này — track này luôn dạy `frozen=True` chặn được thật.

Chỗ lệch là CÁCH nó chặn: không âm thầm bỏ qua — nó NÉM MỘT NGOẠI LỆ,
dừng hẳn chương trình lại tại đúng dòng gán đó. Nếu bạn không bắt lỗi
này bằng `try/except`, chương trình sẽ dừng, không phải lặng lẽ chạy
tiếp.
::
:::
::::

::::code{#doi-gia-nhieu-tang}
Hoàn thiện `doi_gia`: món khớp `ten_mon` phải có giá MỚI, món khác giữ
nguyên, `don` gốc không được đổi.

```python title=starter
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class MonAn:
    ten: str
    gia: int

@dataclass(frozen=True)
class DonHang:
    ma_don: str
    mon: tuple

def doi_gia(don, ten_mon, gia_moi):
    mon_moi = tuple(
        ___
        for m in don.mon
    )
    return replace(don, mon=mon_moi)

don = DonHang("DH-9", (MonAn("Phở", 45000), MonAn("Trà đá", 5000)))
don_moi = doi_gia(don, "Trà đá", 8000)

print(f"don:     {don}")
print(f"don_moi: {don_moi}")
```

```python title=solution
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class MonAn:
    ten: str
    gia: int

@dataclass(frozen=True)
class DonHang:
    ma_don: str
    mon: tuple

def doi_gia(don, ten_mon, gia_moi):
    mon_moi = tuple(
        replace(m, gia=gia_moi) if m.ten == ten_mon else m
        for m in don.mon
    )
    return replace(don, mon=mon_moi)

don = DonHang("DH-9", (MonAn("Phở", 45000), MonAn("Trà đá", 5000)))
don_moi = doi_gia(don, "Trà đá", 8000)

print(f"don:     {don}")
print(f"don_moi: {don_moi}")
```

```python title=test
assert don.mon[1].gia == 5000, "don gốc không được đổi — cả tầng ngoài (DonHang) lẫn tầng trong (MonAn) đều phải giữ nguyên"
assert don_moi.mon[1].gia == 8000, "món Trà đá trong don_moi phải đổi giá thành 8000"
assert don_moi.mon[0].gia == 45000, "món Phở không khớp ten_mon, giá phải giữ nguyên 45000"
assert don.mon[0] is don_moi.mon[0], "món không đổi (Phở) nên KHÔNG cần tạo mới — phải là chính object cũ, không phải một bản sao"
assert don.mon[1] is not don_moi.mon[1], "món Trà đá đã đổi giá phải là một MonAn MỚI — replace() không sửa món cũ tại chỗ"
assert don is not don_moi, "don_moi phải là một DonHang MỚI, không phải chính don"
assert isinstance(don_moi.mon, tuple), "don_moi.mon phải vẫn là tuple, không phải list"
```

:::hints
- kind: attention
  body: Chỗ trống nằm bên trong một biểu thức sinh ra giá trị cho TỪNG món m — dùng đúng if/else trên một dòng (biểu thức điều kiện), không phải câu lệnh if nhiều dòng.
- kind: strategy
  body: Với món m khớp ten_mon, gọi replace(m, gia=gia_moi) để có một MonAn mới. Với món không khớp, trả lại chính m — không tạo gì cả, không gán gì vào m.
- kind: one-line
  body: 'Điền `replace(m, gia=gia_moi) if m.ten == ten_mon else m` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: lời giải phải gọi replace() ở CẢ hai tầng (một lần cho món khớp bên trong vòng lặp, một lần đã có sẵn ở dòng cuối cho DonHang), phải dùng biến vòng lặp m, và không được sửa field tại chỗ (m.gia = ...)
  requireAst:
  - kind: uses-call, target: replace, min: 2
  - kind: uses-name, target: m, min: 1
  - kind: no-mutation
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^don:     DonHang\(ma_don='DH-9', mon=\(MonAn\(ten='Phở', gia=45000\), MonAn\(ten='Trà đá', gia=5000\)\)\)\ndon_moi: DonHang\(ma_don='DH-9', mon=\(MonAn\(ten='Phở', gia=45000\), MonAn\(ten='Trà đá', gia=8000\)\)\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai tầng, hai lần `replace()`, không một chỗ nào sửa tại chỗ. Dữ liệu
lồng nhau bất biến vẫn ghép lại được thành một hàm gọn gàng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa ghép `frozen=True`, `tuple`, và `replace()` ở nhiều tầng thành
một hàm hoàn chỉnh. Track này có bốn công cụ bất biến —
`tuple`, `frozenset`, `frozen=True`, `MappingProxyType` — mỗi công cụ
hợp với một kiểu dữ liệu khác nhau. Cho một bài toán mới, với dữ liệu
cụ thể chưa nói trước đó cần công cụ nào, bạn có nhận ra ngay công cụ
nào hợp không — hay còn phải được chỉ định sẵn?

Bài sau kiểm đúng chuyện đó.
::::

::::checkpoint{mastery=0.8}
::::
