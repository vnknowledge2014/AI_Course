---
id: lap-trinh-ham.bat-bien-thuan-khiet.frozenset-set-bat-bien
title: "`frozenset` — một `set` không sửa được"
summary: "frozenset(...) không có .add()/.remove() — cùng cơ chế như tuple: không phải bị chặn, mà THẬT SỰ KHÔNG CÓ phương thức đó. Đủ bốn công cụ bất biến của track: tuple, frozenset, frozen dataclass, MappingProxyType."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.frozenset]
requires: [fp.mappingproxy]
concepts: [fp.frozenset]
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
`set` không bị bỏ sót. Nó có bản bất biến — và bạn đã biết cơ chế của
nó rồi, từ hai bài trước `MappingProxyType`.
::::

::::explain{#khong-co-chu-khong-phai-bi-chan}
`frozenset` đứng cùng phe với `tuple`, không cùng phe với
`MappingProxyType`. `tuple` không CÓ `.append()` — không phải Python
chặn, mà kiểu dữ liệu đó chưa từng được định nghĩa có phương thức đó.
`frozenset` đi đúng con đường ấy cho `set`: nó là một `set` không có
`.add()`, `.remove()`, `.discard()`, `.update()`, `.pop()`, hay bất kỳ
phương thức sửa tại chỗ nào.

```python
vai_tro = frozenset(["thanh_vien", "khach"])

vai_tro.add("admin")
```

Gọi `.add(...)` ném thẳng `AttributeError: 'frozenset' object has no
attribute 'add'` (đã đo thật) — cùng loại lỗi, cùng lý do như `tuple`
hai bài `.append()` trước đây: KHÔNG CÓ phương thức đó để gọi.

Đọc vẫn hoạt động đầy đủ: `in` để kiểm tra có mặt, `len()`, duyệt bằng
`for`, và cả các phép TOÁN TẬP HỢP — hợp (`|`), giao (`&`), hiệu
(`-`) — những phép đó không SỬA `frozenset` gốc, chúng TẠO một
`frozenset` MỚI, đúng tinh thần xuyên suốt track: không sửa, chỉ tạo
mới.

Một điểm khác `MappingProxyType` đáng nhớ: `MappingProxyType` là một
CỬA SỔ trỏ vào `dict` gốc — sửa `dict` gốc thì cửa sổ thấy ngay.
`frozenset(mot_set)` thì KHÔNG — nó chép các phần tử ra một tập hợp
ĐỘC LẬP, không còn dính gì tới `set` gốc nữa. Hai công cụ, hai cách
"chỉ đọc" khác nhau.
::::

::::example{#hop-tu-va-hop-moi}
```python title=readonly
vai_tro_goc = {"thanh_vien", "khach"}
vai_tro = frozenset(vai_tro_goc)

vai_tro_goc.add("quan_ly")
print(f"goc:    {sorted(vai_tro_goc)}")
print(f"frozen: {sorted(vai_tro)}")

mo_rong = vai_tro | {"admin"}
print(f"mo_rong: {sorted(mo_rong)}")
print(f"vai_tro sau khi hợp: {sorted(vai_tro)}")
```

```text title=readonly
goc:    ['khach', 'quan_ly', 'thanh_vien']
frozen: ['khach', 'thanh_vien']
mo_rong: ['admin', 'khach', 'thanh_vien']
vai_tro sau khi hợp: ['khach', 'thanh_vien']
```

Thêm `"quan_ly"` vào `vai_tro_goc` không hề động tới `vai_tro` — đúng
như giải thích ở trên, `frozenset` đã chép độc lập từ lúc tạo ra, không
còn là cửa sổ trỏ ngược về `set` gốc. Và `vai_tro | {"admin"}` không
sửa `vai_tro` — nó TẠO một `frozenset` mới (`mo_rong`), còn `vai_tro`
giữ nguyên hai phần tử ban đầu.
::::

::::predict{#doan-loai-loi-frozenset commitOnce}
Một danh sách khách mời được đóng băng bằng `frozenset`.

**Trước khi chạy**, bạn đoán dòng cuối gây ra chuyện gì?

```python
khach_moi = frozenset(["An", "Bình"])
khach_moi.add("Chi")
```

:::opt{correct}
Máy dừng lại, báo `AttributeError: 'frozenset' object has no attribute
'add'`
:::

:::opt
Máy báo `TypeError: 'frozenset' object does not support item
assignment`
::why
Gần đúng ở việc bạn nhớ đúng có một `TypeError` xuất hiện ở đâu đó
trong track này khi ghi vào một cấu trúc chỉ đọc — đúng, đó là chuyện
của `MappingProxyType` khi gán vào một Ô.

Chỗ lệch: `khach_moi.add("Chi")` không phải gán vào một Ô (không có
dấu ngoặc vuông nào ở đây) — nó GỌI MỘT PHƯƠNG THỨC. `frozenset` không
có phương thức đó, nên lỗi là `AttributeError`, không phải `TypeError`.
::
:::

:::opt
Không có lỗi nào — `frozenset` cho thêm phần tử vào, chỉ `tuple` và
`dict` mới thật sự chặn
::why
Gần đúng ở việc bạn tin `set` (không đóng băng) đúng là cho `.add()`
chạy bình thường — điều đó đúng.

Chỗ lệch: đây là `frozenset`, không phải `set`. `frozenset` không hề có
phương thức `.add()` — gọi nó ném lỗi ngay, dừng chương trình tại đúng
dòng đó, đúng như bài `tuple` đã cho bạn thấy với `.append()`.
::
:::

:::opt
Máy báo `dataclasses.FrozenInstanceError: cannot assign to field 'add'`
::why
Gần đúng ở việc bạn nhớ đúng chữ "frozen" gắn với một loại lỗi riêng
trong track này — đó là `FrozenInstanceError`, đúng.

Chỗ lệch: lỗi đó chỉ xảy ra với `@dataclass(frozen=True)`, khi GÁN LẠI
một FIELD của một OBJECT. `frozenset` không phải dataclass, không có
field nào cả — nó là một kiểu dữ liệu có sẵn của Python, và lỗi của nó
là `AttributeError` khi gọi một phương thức nó không có.
::
:::
::::

::::code{#dong-bang-vai-tro}
Điền chỗ trống để `vai_tro` là bản đóng băng của `vai_tro_goc`, rồi
kiểm tra: gọi `.add(...)` trên nó phải bị chặn bằng `AttributeError`.

```python title=starter
vai_tro_goc = {"thanh_vien", "khach"}
vai_tro = ___

da_chan_dung = False
loai_loi = None
try:
    vai_tro.add("admin")
except Exception as loi:
    da_chan_dung = True
    loai_loi = type(loi).__name__

print(f"vai_tro: {sorted(vai_tro)}")
print(f"Bị chặn: {da_chan_dung}")
print(f"Loại lỗi: {loai_loi}")
```

```python title=solution
vai_tro_goc = {"thanh_vien", "khach"}
vai_tro = frozenset(vai_tro_goc)

da_chan_dung = False
loai_loi = None
try:
    vai_tro.add("admin")
except Exception as loi:
    da_chan_dung = True
    loai_loi = type(loi).__name__

print(f"vai_tro: {sorted(vai_tro)}")
print(f"Bị chặn: {da_chan_dung}")
print(f"Loại lỗi: {loai_loi}")
```

```python title=test
assert isinstance(vai_tro, frozenset), "vai_tro phải là một frozenset — không phải chính vai_tro_goc, không phải một set thường"
assert vai_tro == frozenset({"thanh_vien", "khach"}), "vai_tro phải mang đúng hai vai trò của vai_tro_goc"
assert da_chan_dung is True, "gọi vai_tro.add('admin') phải bị chặn — dùng frozenset(vai_tro_goc), không phải set(vai_tro_goc) hay chính vai_tro_goc"
assert loai_loi == "AttributeError", f"lỗi ném ra phải là AttributeError, đang là {loai_loi}"
```

:::hints
- kind: attention
  body: Chỗ trống là một lời gọi bọc quanh vai_tro_goc — không phải set(vai_tro_goc), không phải gán thẳng vai_tro_goc.
- kind: strategy
  body: Cùng cách tuple() bọc quanh một list để có dãy không sửa được, frozenset() bọc quanh một set để có tập hợp không sửa được.
- kind: one-line
  body: 'Điền `frozenset(vai_tro_goc)` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải gọi frozenset(vai_tro_goc) — bọc đúng set vai_tro_goc đã có, không dùng set() (vẫn sửa được) hay gán thẳng vai_tro_goc
  requireAst:
  - kind: uses-call, target: frozenset, min: 1
  - kind: uses-name, target: vai_tro_goc, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^vai_tro: \['khach', 'thanh_vien'\]\nBị chặn: True\nLoại lỗi: AttributeError\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn công cụ trọn vẹn: `tuple` cho dãy, `frozenset` cho tập hợp,
`frozen=True` cho đối tượng có tên trường, `MappingProxyType` cho ánh
xạ. Mỗi kiểu dữ liệu, đúng một công cụ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn có bốn công cụ rồi. Nhưng thử tưởng tượng một `@dataclass(frozen=True)`
có MỘT field kiểu `list` — không phải `tuple`. `frozen=True` có chặn
được việc sửa BÊN TRONG field đó (`.append(...)` vào chính list ấy),
hay nó chỉ chặn gán LẠI cả field?

Bài sau trả lời — và câu trả lời không phải điều bạn nghĩ.
::::

::::checkpoint{mastery=0.8}
::::
