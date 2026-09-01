---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.closure-khong-thuan-nonlocal
title: "Closure KHÔNG THUẦN — `nonlocal` sửa biến ngoài mỗi lần gọi"
summary: "def lam_dem(bat_dau=0): dem = bat_dau; def tang(): nonlocal dem; dem += 1; return dem; return tang — dem += 1 SỬA biến của hàm NGOÀI, mỗi lần gọi ra một kết quả KHÁC. Đây CHÍNH LÀ 'phụ thuộc trạng thái bên ngoài tham số' của T4.1."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [fp.closure-impure]
requires: [fp.closure-pure, fp.nondeterministic-impure]
concepts: [fp.closure-impure]
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
Bài trước: closure chỉ ĐỌC biến ngoài, vẫn thuần. Hôm nay: closure SỬA
được biến ngoài không? Nếu được, chuyện gì xảy ra với tính thuần?
::::

::::explain{#nonlocal-sua-bien-ngoai}
```python
def lam_dem(bat_dau=0):
    dem = bat_dau
    def tang():
        nonlocal dem
        dem += 1
        return dem
    return tang

dem1 = lam_dem()
print(dem1())
print(dem1())
print(dem1())
```

```text
1
2
3
```

`tang` là một closure lồng BÊN TRONG `lam_dem` — nó bắt biến `dem` của
`lam_dem`. Từ khoá `nonlocal dem` khai báo: "khi tôi gán `dem`, tôi
SỬA biến `dem` của hàm NGOÀI, không tạo một biến CỤC BỘ mới cùng tên".
Không có `nonlocal`, dòng `dem += 1` sẽ báo lỗi (Python coi `dem` là
biến cục bộ MỚI của `tang`, chưa có giá trị để cộng thêm).

`dem1()` gọi BA lần, CÙNG không đối số nào khác — nhưng ra BA kết quả
KHÁC nhau (`1`, `2`, `3`). Đây là điều CHƯA từng xảy ra ở bài 13: một
closure có thể cho kết quả KHÁC nhau giữa các lần gọi, dù đối số y hệt
(ở đây thậm chí không CÓ đối số nào).
::::

::::example{#noi-thang-t41}
T4.1 đã dạy `fp.nondeterministic-impure`: một hàm không thuần khi nó
phụ thuộc TRẠNG THÁI BÊN NGOÀI tham số. Đây CHÍNH LÀ điều đó — chỉ khác
chỗ trạng thái đó nằm TRONG CHÍNH closure, không phải một biến toàn
cục ở module:

```python title=readonly
def lam_dem(bat_dau=0):
    dem = bat_dau
    def tang():
        nonlocal dem
        dem += 1
        return dem
    return tang

dem1 = lam_dem()
a = dem1()
b = dem1()
print(a == b)
```

```text title=readonly
False
```

`a` và `b` gọi CÙNG `dem1()`, KHÔNG đối số nào khác nhau — nhưng
`a != b`. Đây LÀ chữ ký của một hàm không thuần: gọi lại y hệt, kết quả
đổi. Bài 13's `cong5(10) == cong5(10)` luôn `True`; ở đây `dem1() ==
dem1()` luôn `False`. Khác biệt DUY NHẤT giữa hai bài: bài 13 chỉ ĐỌC
biến bắt được, bài này SỬA nó bằng `nonlocal`.
::::

::::predict{#doan-hai-lan-goi-khac-nhau commitOnce}
```python
def lam_dem(bat_dau=0):
    dem = bat_dau
    def tang():
        nonlocal dem
        dem += 1
        return dem
    return tang

dem_x = lam_dem(5)
print(dem_x())
print(dem_x())
```

Hai dòng cuối in ra gì?

:::opt{correct}
`6` rồi `7`
:::

:::opt
`6` rồi `6` — vì cả hai lần gọi CÙNG một closure `dem_x`, phải ra CÙNG
kết quả
::why
Gần đúng ở việc bạn tính đúng lần gọi ĐẦU (`bat_dau=5`, `dem += 1` →
`6`) — bước tính đó đúng.

Chỗ lệch: closure này KHÔNG THUẦN — mỗi lần gọi `dem_x()`, `nonlocal
dem; dem += 1` SỬA `dem` NGAY TẠI CHỖ, giữ nguyên giá trị mới đó cho
lần gọi SAU. Lần gọi thứ hai bắt đầu từ `dem = 6` (đã bị sửa), cộng
thêm `1` ra `7` — không quay lại `6`.
::
:::

:::opt
Máy báo lỗi ở dòng `dem_x()` thứ hai — một closure không thuần chỉ gọi
được ĐÚNG MỘT LẦN
::why
Gần đúng ở việc bạn nghi ngờ closure không thuần "có gì đó khác thường"
— có sự khác thường thật (không dự đoán được), nhưng không phải lỗi.

Chỗ lệch: closure không thuần vẫn gọi được BAO NHIÊU LẦN TUỲ Ý — không
có giới hạn số lần gọi. Vấn đề không phải "gọi được hay không", mà là
"kết quả có ĐOÁN TRƯỚC được không" — và với closure này thì KHÔNG, mỗi
lần gọi cho một kết quả khác.
::
:::
::::

::::code{#lam_dem}
Viết `lam_dem(bat_dau=0)` — trả về một closure `tang()` KHÔNG THUẦN:
mỗi lần gọi tăng bộ đếm nội bộ thêm `1` (bắt đầu từ `bat_dau`) và trả
về giá trị mới.

```python title=starter
def lam_dem(bat_dau=0):
    dem = bat_dau
    def tang():
        ___
    return tang

dem1 = lam_dem()
print(dem1())
print(dem1())
print(dem1())
```

```python title=solution
def lam_dem(bat_dau=0):
    dem = bat_dau
    def tang():
        nonlocal dem
        dem += 1
        return dem
    return tang

dem1 = lam_dem()
print(dem1())
print(dem1())
print(dem1())
```

```python title=test
dem_a = lam_dem()
assert dem_a() == 1, "lần gọi đầu tiên phải ra 1"
assert dem_a() == 2, "lần gọi thứ hai phải ra 2 — bộ đếm phải TĂNG, không lặp lại 1"
assert dem_a() == 3, "lần gọi thứ ba phải ra 3"
dem_b = lam_dem(10)
assert dem_b() == 11, "bat_dau=10 thì lần gọi đầu phải ra 11"
assert dem_a() == 4, "dem_a không được bị ảnh hưởng bởi việc tạo dem_b"
```

:::hints
- kind: attention
  body: "Khai báo nonlocal dem trước khi sửa nó — không có nonlocal, dem += 1 sẽ báo lỗi (Python coi dem là biến cục bộ mới, chưa có giá trị)."
- kind: strategy
  body: 'nonlocal dem; dem += 1; return dem — ba dòng: khai báo sửa biến ngoài, cộng thêm 1, trả về giá trị mới.'
- kind: one-line
  body: "nonlocal dem\ndem += 1\nreturn dem"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: tang phải SỬA biến dem của hàm ngoài (dùng nonlocal) rồi trả về giá trị đã sửa — không chỉ đọc bat_dau hay dem mà không cộng dồn.
  requireAst:
  - kind: gan-ten, target: dem, min: 1
  - kind: uses-name, target: dem, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một bộ đếm hữu ích thật — nhưng nó không thuần: gọi lại y hệt, kết quả
đổi. `nonlocal` là điểm chốt: nó cho closure SỬA, không chỉ ĐỌC.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Một bộ đếm không thuần vẫn hữu ích — nhưng dùng nó mà KHÔNG BIẾT nó
không thuần thì dễ viết test sai (đoán `dem_a() == dem_a()` là `True`,
như bài 13, trong khi thực tế luôn `False`).

Bài sau: làm sao dùng closure không thuần một cách CÓ CHỦ ĐÍCH, không
phải vô tình?
::::

::::checkpoint{mastery=0.8}
::::
