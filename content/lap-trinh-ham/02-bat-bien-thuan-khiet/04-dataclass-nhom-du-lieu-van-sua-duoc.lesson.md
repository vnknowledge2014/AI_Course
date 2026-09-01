---
id: lap-trinh-ham.bat-bien-thuan-khiet.dataclass-nhom-du-lieu-van-sua-duoc
title: "`@dataclass` nhóm dữ liệu lại — nhưng field vẫn sửa được"
summary: "@dataclass thay dict lỏng lẻo bằng field có tên cố định — nhưng @dataclass TRẦN vẫn cho sửa field bình thường. Đúng vấn đề bài 1-2 nêu ra, giờ ở cấp OBJECT thay vì list."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.dataclass-still-mutable]
requires: [fp.tuple-immutable, core.dict]
concepts: [fp.dataclass-still-mutable]
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
`tuple` chỉ có chỉ số — 0, 1, 2. Muốn tên gọi hẳn hoi cho từng phần dữ
liệu, Python có một công cụ khác. Byte giới thiệu nó hôm nay.
::::

::::explain{#nhom-du-lieu-co-ten}
Từ đầu khoá, cách quen thuộc để nhóm nhiều dữ liệu liên quan lại là
`dict`: `{"ten": "Phở bò", "gia": 40000}`. Cách này hoạt động, nhưng có
một điểm yếu — KHOÁ là một chuỗi tự do, gõ nhầm `"gia"` thành `"gía"`
Python không hề báo lỗi lúc gõ, chỉ lộ ra khi tra cứu sau này thất bại.

`@dataclass` (trong thư viện `dataclasses`, có sẵn trong Python) là một
cách nhóm dữ liệu khác: khai một `class`, liệt kê từng phần dữ liệu như
một FIELD có TÊN và KIỂU cố định.

```python
from dataclasses import dataclass

@dataclass
class MonAn:
    ten: str
    gia: int
```

Viết `MonAn("Phở bò", 40000)` dựng một đối tượng, đọc bằng `.ten` và
`.gia` — không còn chuỗi khoá tự do nào để gõ nhầm, và một trình soạn
thảo hiểu Python còn tự gợi ý đúng tên field cho bạn.

Nhưng khoan mừng vội. `@dataclass` TRẦN — không tham số nào trong dấu
ngoặc — chỉ lo việc NHÓM dữ liệu lại cho gọn. Nó không hứa hẹn gì về
việc field có sửa được sau khi tạo hay không.
::::

::::example{#field-van-sua-duoc}
Byte tạo một `MonAn`, rồi thử sửa giá của nó — y hệt kiểu bài 1 từng làm
với `list`, chỉ khác đối tượng.

```python title=readonly
from dataclasses import dataclass

@dataclass
class MonAn:
    ten: str
    gia: int

mon = MonAn("Phở bò", 40000)
print(f"Trước: {mon}")

mon.gia = 99999
print(f"Sau:   {mon}")
```

```text title=readonly
Trước: MonAn(ten='Phở bò', gia=40000)
Sau:   MonAn(ten='Phở bò', gia=99999)
```

Dòng `mon.gia = 99999` chạy trót lọt, không một lời cảnh báo. `@dataclass`
trần cho `mon` một hình dạng gọn gàng, có tên field rõ ràng — nhưng field
đó vẫn là một Ô có thể GHI ĐÈ bất cứ lúc nào, đúng vấn đề bài 1 từng nêu
ra với `list`, chỉ chuyển từ cấp PHẦN TỬ CỦA DANH SÁCH sang cấp FIELD
CỦA ĐỐI TƯỢNG.
::::

::::predict{#doan-hai-ten-mot-mon commitOnce}
Byte lấy một `MonAn`, rồi mượn tạm một cái tên thứ hai cho nó — y hệt
bài `hai-the-mot-noi` từng làm với `list`, giờ thử với `@dataclass`.

**Trước khi chạy**, bạn đoán dòng cuối in ra gì?

```python
from dataclasses import dataclass

@dataclass
class MonAn:
    ten: str
    gia: int

thuc_don = MonAn("Trà đá", 5000)
gia_km = thuc_don
gia_km.gia = 3000

print(thuc_don)
```

:::opt{correct}
`MonAn(ten='Trà đá', gia=3000)`
:::

:::opt
`MonAn(ten='Trà đá', gia=5000)` — không đổi, vì `@dataclass` trông có
cấu trúc chặt chẽ hơn `list`/`dict`
::why
Gần đúng ở việc bạn cảm nhận `@dataclass` trông "nghiêm chỉnh" hơn — có
khai kiểu, có tên field rõ ràng, không lỏng lẻo như `dict`.

Chỗ lệch: `@dataclass` TRẦN (không tham số) không hề tự động bất biến —
field vẫn gán lại được bình thường, và `gia_km = thuc_don` trùng địa chỉ
với `thuc_don` (đúng luật `hai-the-mot-noi`, y hệt hai tên cùng trỏ một
`list`). Sửa field qua `gia_km` cũng làm `thuc_don` đổi theo, vì cả hai
đang trỏ đúng một đối tượng.
::
:::

:::opt
Máy dừng lại báo lỗi kiểu, vì `gia` đã khai `int` mà gán số khác cũng
phải khớp kiểu mới được chấp nhận
::why
Gần đúng ở việc bạn nhớ đúng: `gia: int` là một chú thích kiểu có thật
trong khai báo `MonAn`.

Chỗ lệch: chú thích kiểu (`gia: int`) trong Python chỉ là GHI CHÚ cho
người đọc và cho công cụ kiểm kiểu TĨNH (như `mypy`) — không có gì kiểm
tra hay chặn NÓ LÚC CHẠY. Gán `3000` (cũng là `int`, và kể cả gán một
kiểu khác hẳn) đều chạy trót lọt, không lỗi gì.
::
:::

:::opt
`MonAn(ten='Trà đá', gia=5000)` — vì `gia_km = thuc_don` tạo ra một bản
sao riêng, `thuc_don` không hề bị đụng tới
::why
Gần đúng ở việc bạn tin mỗi cái tên giữ dữ liệu "riêng" của nó — trực
giác dễ hiểu cho người mới, và với vài kiểu dữ liệu (như số, chuỗi) thì
việc đó không thành vấn đề vì chúng không sửa tại chỗ được.

Chỗ lệch: `gia_km = thuc_don` không dựng đối tượng mới — đúng luật đã
học ở `mem.aliasing-explained`, nó chỉ buộc THÊM một tấm thẻ vào ĐÚNG
đối tượng mà `thuc_don` đang trỏ tới. Sửa field qua `gia_km` là sửa
đúng đối tượng mà `thuc_don` cũng đang giữ.
::
:::
::::

::::code{#khai-mon-an}
Byte đang khai một `MonAn` gồm hai field: tên và giá. Điền field còn
thiếu — `gia`, kiểu `int` — rồi quan sát: field đó vẫn sửa được, đúng
điều bài này chỉ ra.

```python title=starter
from dataclasses import dataclass

@dataclass
class MonAn:
    ten: str
    ___                      # thêm field gia, kiểu int

mon = MonAn("Bún bò", 45000)
print(f"Trước khi sửa: {mon}")

mon.gia = 50000              # sửa TẠI CHỖ field gia
print(f"Sau khi sửa:   {mon}")
```

```python title=solution
from dataclasses import dataclass

@dataclass
class MonAn:
    ten: str
    gia: int

mon = MonAn("Bún bò", 45000)
print(f"Trước khi sửa: {mon}")

mon.gia = 50000              # sửa TẠI CHỖ field gia
print(f"Sau khi sửa:   {mon}")
```

```python title=test
assert mon.ten == "Bún bò", "ten không được đổi trong bài này"
assert mon.gia == 50000, "field gia phải sửa được — @dataclass TRẦN không chặn gán lại field, đúng điều bài này chỉ ra"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ngay dưới field ten trong thân class — thêm field gia, kiểu int, viết đúng cú pháp "tên: kiểu" như field ten ở trên.
- kind: strategy
  body: 'Một field trong @dataclass viết bằng "tên_field: kiểu_dữ_liệu" — không có dấu bằng, không phải một câu lệnh gán. field ten ở trên là mẫu: "ten: str". Viết tương tự cho gia với kiểu int.'
- kind: one-line
  body: 'Điền `gia: int` vào chỗ trống, thụt vào bốn dấu cách cho thẳng hàng với dòng ten: str.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Trước khi sửa: MonAn\(ten='Bún bò', gia=45000\)\nSau khi sửa:   MonAn\(ten='Bún bò', gia=50000\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Field có tên rõ ràng — nhưng vẫn ghi đè được như trước. `@dataclass`
trần giải quyết vấn đề "gõ nhầm tên khoá", không giải quyết vấn đề "sửa
tại chỗ".
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài `tuple-khi-khong-can-sua` cho bạn thấy MỘT cách khiến sửa tại chỗ
không viết được: dùng một kiểu dữ liệu vốn không có phương thức sửa.
`@dataclass` trần không tự có tính chất đó — nhưng cú pháp của nó cho
phép viết tham số NGAY TRONG dấu ngoặc (`@dataclass(...)`), y hệt một
hàm nhận đối số.

Nếu có một tham số như vậy khiến `@dataclass` chặn hẳn việc gán lại
field — giống cách `tuple` chặn `.append()` — nó sẽ có tên là gì, và
báo lỗi kiểu gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
