---
id: lap-trinh-ham.bat-bien-thuan-khiet.mappingproxytype-dict-chi-doc
title: "`MappingProxyType` — một `dict` CHỈ ĐỌC thật sự"
summary: "MappingProxyType bọc một dict có sẵn: đọc bình thường, nhưng gán vào một ô ném TypeError. Công cụ bất biến thứ năm của track, dành riêng cho ánh xạ — sau tuple (dãy) và frozen dataclass (đối tượng)."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.mappingproxy]
requires: [fp.review-immutable-pure]
concepts: [fp.mappingproxy]
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
Bài trước ghép xong bất biến với thuần khiết trong một hàm. Nhưng công
cụ bất biến của track mới phủ được DÃY (`tuple`) và ĐỐI TƯỢNG có tên
trường (`frozen=True`). Còn `dict` — thứ bạn tra cứu khắp nơi — có bản
chỉ đọc không?
::::

::::explain{#mot-cua-so-chi-doc}
Có. Module `types` có sẵn `MappingProxyType` — không phải một kiểu dữ
liệu MỚI để bạn tạo trực tiếp, mà là một CỬA SỔ bọc quanh một `dict` đã
có sẵn.

```python
from types import MappingProxyType

cau_hinh = {"che_do": "sang", "am_luong": 50}
chi_doc = MappingProxyType(cau_hinh)
```

Qua `chi_doc`, đọc vẫn hoạt động y hệt `dict` thường —
`chi_doc["che_do"]`, `chi_doc.get(...)`, `len(chi_doc)`, duyệt bằng
`for`. Nhưng GHI thì bị từ chối: `chi_doc["che_do"] = "toi"` ném
`TypeError: 'mappingproxy' object does not support item assignment`
(đã đo thật). Khác `frozen=True` — cái đó chặn GÁN LẠI một FIELD của
một object. `MappingProxyType` chặn GÁN VÀO MỘT Ô của một ánh xạ. Hai
cơ chế, hai loại dữ liệu, cùng một tinh thần.

Có một điều cần nói thẳng, kẻo hiểu lầm: `MappingProxyType` không TẠO
ra một bản sao độc lập. Nó chỉ là một CỬA SỔ nhìn vào đúng `dict` gốc
bạn đưa vào. Ai đó CÒN GIỮ tên `cau_hinh` (không đi qua `chi_doc`) vẫn
sửa được `dict` gốc bình thường — và `chi_doc` sẽ THẤY thay đổi đó
ngay lập tức, vì nó đang nhìn vào đúng chỗ đó. `chi_doc` chặn đúng MỘT
cửa: cửa ghi ĐI QUA chính nó. Nó không khoá luôn cánh cửa khác đang mở
vào cùng dữ liệu.
::::

::::example{#cau-hinh-chi-doc}
```python title=readonly
from types import MappingProxyType

cau_hinh_goc = {"che_do": "sang", "am_luong": 50}
cau_hinh = MappingProxyType(cau_hinh_goc)

print(f"Qua proxy: {cau_hinh['che_do']}")

cau_hinh_goc["che_do"] = "toi"
print(f"Qua proxy sau khi đổi gốc: {cau_hinh['che_do']}")

cau_hinh["am_luong"] = 80
```

```text title=readonly
Qua proxy: sang
Qua proxy sau khi đổi gốc: toi
```

```text title=readonly
TypeError: 'mappingproxy' object does not support item assignment
```

Hai dòng đầu tiên chỉ đọc — chạy bình thường. Dòng
`cau_hinh_goc["che_do"] = "toi"` sửa thẳng vào `dict` GỐC, không đi qua
`cau_hinh` — và `cau_hinh` thấy thay đổi đó ngay, đúng như lời giải
thích ở trên. Chỉ khi CHÍNH `cau_hinh` bị đem ra gán vào một ô
(`cau_hinh["am_luong"] = 80`), Python mới từ chối, với đúng thông báo ở
trên.
::::

::::predict{#doan-loai-loi-mappingproxy commitOnce}
Một bảng giá món ăn được bọc bằng `MappingProxyType`.

**Trước khi chạy**, bạn đoán dòng cuối gây ra chuyện gì?

```python
from types import MappingProxyType

bang_gia = MappingProxyType({"phở": 45000, "trà đá": 5000})
bang_gia["phở"] = 50000
```

:::opt{correct}
Máy dừng lại, báo `TypeError: 'mappingproxy' object does not support
item assignment`
:::

:::opt
Máy báo `dataclasses.FrozenInstanceError: cannot assign to field 'phở'`
::why
Gần đúng ở việc bạn nhớ đúng: có một công cụ trong track này ném
`FrozenInstanceError` — đó là `@dataclass(frozen=True)`, khi gán lại
một FIELD của một OBJECT.

Chỗ lệch: `bang_gia` không phải một object có field — nó là một ánh xạ
(dict được bọc). Gán vào một Ô của ánh xạ (`bang_gia["phở"] = ...`)
không đi qua cơ chế field của dataclass, nên không thể ném đúng loại
lỗi đó. `MappingProxyType` có lỗi riêng: `TypeError`.
::
:::

:::opt
Máy báo `AttributeError: 'mappingproxy' object has no attribute
'__setitem__'`
::why
Gần đúng ở việc bạn nhớ đúng có công cụ trong track này ném
`AttributeError` — đó là `tuple` và `frozenset`, khi gọi một PHƯƠNG
THỨC chúng không có (`.append()`, `.add()`).

Chỗ lệch: `bang_gia["phở"] = ...` không gọi một phương thức bị thiếu —
nó dùng cú pháp GÁN VÀO Ô, một cú pháp hoàn toàn hợp lệ về mặt ngữ
pháp. `mappingproxy` CÓ nhận cú pháp đó, chỉ là nó CHỦ ĐỘNG từ chối lúc
chạy — nên lỗi là `TypeError`, không phải `AttributeError` báo thiếu
phương thức.
::
:::

:::opt
Không có lỗi nào — `MappingProxyType` chỉ là một gợi ý, không thật sự
chặn lúc chạy
::why
Gần đúng ở việc bạn cảnh giác đúng chỗ — có một công cụ trong track này
đúng là chỉ là TÍN HIỆU cho công cụ kiểm kiểu tĩnh, không chặn lúc
chạy (đó là `Final`, không nằm trong bốn công cụ track này dùng để
chấm điểm).

Chỗ lệch: `MappingProxyType` không phải loại đó — nó đã được ĐO THẬT
trên chính engine chấm bài này, và gán vào một ô của nó DỪNG chương
trình lại ngay lập tức, với `TypeError` ở trên.
::
:::
::::

::::code{#dong-bang-cau-hinh}
Điền chỗ trống để `cau_hinh` là một bản CHỈ ĐỌC của `cau_hinh_goc`, rồi
kiểm tra: gán vào một ô của nó phải bị chặn bằng `TypeError`.

```python title=starter
from types import MappingProxyType

cau_hinh_goc = {"che_do": "sang", "am_luong": 50}
cau_hinh = ___

da_chan_dung = False
loai_loi = None
try:
    cau_hinh["che_do"] = "toi"
except Exception as loi:
    da_chan_dung = True
    loai_loi = type(loi).__name__

print(f"che_do: {cau_hinh['che_do']}")
print(f"Bị chặn: {da_chan_dung}")
print(f"Loại lỗi: {loai_loi}")
```

```python title=solution
from types import MappingProxyType

cau_hinh_goc = {"che_do": "sang", "am_luong": 50}
cau_hinh = MappingProxyType(cau_hinh_goc)

da_chan_dung = False
loai_loi = None
try:
    cau_hinh["che_do"] = "toi"
except Exception as loi:
    da_chan_dung = True
    loai_loi = type(loi).__name__

print(f"che_do: {cau_hinh['che_do']}")
print(f"Bị chặn: {da_chan_dung}")
print(f"Loại lỗi: {loai_loi}")
```

```python title=test
assert type(cau_hinh).__name__ == "mappingproxy", "cau_hinh phải là một MappingProxyType bọc quanh cau_hinh_goc, không phải một dict thường hay chính cau_hinh_goc"
assert cau_hinh["che_do"] == "sang", "cau_hinh phải đọc đúng giá trị từ cau_hinh_goc"
assert da_chan_dung is True, "gán cau_hinh['che_do'] = 'toi' phải bị chặn — dùng MappingProxyType(cau_hinh_goc), không phải dict(cau_hinh_goc) hay chính cau_hinh_goc"
assert loai_loi == "TypeError", f"lỗi ném ra phải là TypeError, đang là {loai_loi}"
```

:::hints
- kind: attention
  body: Chỗ trống là một lời gọi bọc quanh cau_hinh_goc — không phải tạo một dict mới, không phải gán thẳng cau_hinh_goc.
- kind: strategy
  body: Import MappingProxyType đã có sẵn ở dòng đầu. Gọi nó với cau_hinh_goc làm đối số để có một cửa sổ chỉ đọc trỏ vào đúng dict đó.
- kind: one-line
  body: 'Điền `MappingProxyType(cau_hinh_goc)` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải gọi MappingProxyType(cau_hinh_goc) — bọc đúng dict cau_hinh_goc đã có, không tạo dict mới hay gán thẳng cau_hinh_goc
  requireAst:
  - kind: uses-call, target: MappingProxyType, min: 1
  - kind: uses-name, target: cau_hinh_goc, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^che_do: sang\nBị chặn: True\nLoại lỗi: TypeError\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`dict` giờ cũng có bản chỉ đọc. Bốn kiểu dữ liệu quen thuộc, một kiểu
còn lại chưa có bản bất biến trong track này: `set`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`MappingProxyType` chặn sửa Ô của `dict`. Nhưng `set` — một tập hợp
không thứ tự, bạn thường dùng để kiểm tra "có mặt hay không" — cũng sửa
được tại chỗ y hệt vậy: `.add(...)`, `.remove(...)`. Nó có bản bất biến
không, hay `set` là kiểu duy nhất của Python bị bỏ sót?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
