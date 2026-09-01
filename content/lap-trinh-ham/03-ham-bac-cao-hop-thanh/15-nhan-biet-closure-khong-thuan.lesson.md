---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.nhan-biet-closure-khong-thuan
title: "Dùng closure không thuần có CHỦ ĐÍCH, không phải vô tình"
summary: "Một bộ đếm (closure không thuần) hữu ích THẬT — nhưng phải BIẾT nó không thuần, không được assert dem() == dem() (luôn False, KHÁC closure thuần bài 13 luôn True). Bài code: viết CẢ HAI loại trong CÙNG chương trình, test đúng từng loại."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.closure-impure-aware]
requires: [fp.closure-impure]
concepts: [fp.closure-impure-aware]
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
Hai bài trước dạy hai loại closure trái ngược nhau: THUẦN (bài 13) và
KHÔNG THUẦN (bài 14). Hôm nay: chương trình thật thường cần CẢ HAI.
::::

::::explain{#biet-loai-nao-de-test-dung}
Closure không thuần (bộ đếm bài 14) không phải "sai" — nó hữu ích THẬT
(sinh mã đơn hàng tăng dần, đếm số lượt truy cập...). Vấn đề không phải
"có nên dùng nó không", mà là "có BIẾT nó không thuần không" — vì điều
đó quyết định cách TEST nó:

```python
def lam_dem(bat_dau=0):
    dem = bat_dau
    def tang():
        nonlocal dem
        dem += 1
        return dem
    return tang

dem1 = lam_dem()
```

Test SAI (áp dụng nhầm quy tắc của closure THUẦN — bài 13 — vào đây):

```python
assert dem1() == dem1()  # LUÔN False — assert này LUÔN thất bại!
```

Test ĐÚNG (biết closure này không thuần, kỳ vọng đúng bản chất đó):

```python
a = dem1()
b = dem1()
assert a != b        # đúng — hai lần gọi PHẢI khác nhau
assert b == a + 1     # đúng — bộ đếm phải TĂNG đúng 1 mỗi lần
```

Bài học không phải "tránh dùng closure không thuần" — mà là "biết rõ
một closure thuộc loại nào, để không viết test dựa trên kỳ vọng SAI".
::::

::::example{#hai-loai-cung-mot-chuong-trinh}
Một chương trình thật thường cần CẢ hai loại, cho hai việc KHÁC nhau:

```python title=readonly
def lam_tinh_gia(thue_suat):
    return lambda gia: gia * (1 + thue_suat)

def lam_sinh_ma(tien_to):
    dem = 0
    def sinh():
        nonlocal dem
        dem += 1
        return f"{tien_to}-{dem}"
    return sinh

tinh_gia = lam_tinh_gia(0.1)
sinh_ma = lam_sinh_ma("HD")

print(tinh_gia(100))
print(tinh_gia(100))
print(sinh_ma())
print(sinh_ma())
```

```text title=readonly
110.00000000000001
110.00000000000001
HD-1
HD-2
```

`tinh_gia` (closure THUẦN, bài 13's kiểu) gọi lại CÙNG đối số (`100`)
luôn ra CÙNG kết quả. `sinh_ma` (closure KHÔNG THUẦN, bài 14's kiểu)
gọi lại KHÔNG đối số nào cũng ra kết quả KHÁC mỗi lần. Cả hai đứng
CHUNG một chương trình, mỗi cái làm ĐÚNG việc nó được thiết kế để làm —
không cái nào "sai" so với cái kia.
::::

::::predict{#doan-test-dung-loai commitOnce}
```python
def lam_sinh_ma(tien_to):
    dem = 0
    def sinh():
        nonlocal dem
        dem += 1
        return f"{tien_to}-{dem}"
    return sinh

sinh_ma = lam_sinh_ma("PO")
a = sinh_ma()
b = sinh_ma()
print(a == b)
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì `sinh_ma` được gọi với CÙNG một cách (không đối số nào) cả
hai lần, nên phải ra cùng kết quả
::why
Gần đúng ở việc bạn để ý "gọi giống hệt nhau" (đúng, không đối số nào ở
cả hai lần gọi) — quan sát đó đúng.

Chỗ lệch: `sinh_ma` là closure KHÔNG THUẦN (bài 14's kiểu, dùng
`nonlocal dem; dem += 1`) — "gọi giống hệt nhau" KHÔNG đảm bảo "ra kết
quả giống hệt nhau" cho loại closure này. `a = "PO-1"`, `b = "PO-2"` —
khác nhau, đúng bản chất một bộ đếm.
::
:::

:::opt
Máy báo lỗi — so sánh `a == b` không hợp lệ khi `a`, `b` là kết quả từ
một closure không thuần
::why
Gần đúng ở việc bạn cảnh giác điều gì đó "không bình thường" với closure
không thuần — có sự khác thường thật (kết quả không đoán trước được).

Chỗ lệch: `a == b` là một PHÉP SO SÁNH HOÀN TOÀN HỢP LỆ, chạy bình
thường không lỗi — nó chỉ trả về `False` (vì `a` và `b` là hai chuỗi
KHÁC nhau: `"PO-1"` và `"PO-2"`), không phải một lỗi cú pháp hay lỗi
runtime nào.
::
:::
::::

::::code{#lam_tinh_gia_va_lam_sinh_ma}
Viết HAI hàm trong CÙNG chương trình: `lam_tinh_gia(thue_suat)` — một
closure THUẦN, trả về hàm tính giá đã cộng thuế — và `lam_sinh_ma(tien_to)`
— một closure KHÔNG THUẦN, trả về hàm sinh mã tăng dần (`"HD-1"`,
`"HD-2"`, ...).

```python title=starter
def lam_tinh_gia(thue_suat):
    ___

def lam_sinh_ma(tien_to):
    ___

tinh_gia = lam_tinh_gia(0.1)
print(tinh_gia(100))

sinh_ma = lam_sinh_ma("HD")
print(sinh_ma())
print(sinh_ma())
```

```python title=solution
def lam_tinh_gia(thue_suat):
    return lambda gia: gia * (1 + thue_suat)

def lam_sinh_ma(tien_to):
    dem = 0
    def sinh():
        nonlocal dem
        dem += 1
        return f"{tien_to}-{dem}"
    return sinh

tinh_gia = lam_tinh_gia(0.1)
print(tinh_gia(100))

sinh_ma = lam_sinh_ma("HD")
print(sinh_ma())
print(sinh_ma())
```

```python title=test
tinh_gia2 = lam_tinh_gia(0.1)
assert tinh_gia2(100) == tinh_gia2(100), "lam_tinh_gia phải THUẦN — gọi lại cùng đối số phải ra cùng kết quả"
assert abs(tinh_gia2(100) - 110) < 0.0001, "thuế 10% trên 100 phải xấp xỉ 110"

sinh_ma2 = lam_sinh_ma("PO")
a = sinh_ma2()
b = sinh_ma2()
assert a != b, "lam_sinh_ma phải KHÔNG THUẦN — gọi lại (không đối số) phải ra mã KHÁC nhau"
assert a == "PO-1" and b == "PO-2", "mã phải tăng dần đúng thứ tự, đúng tiền tố"
```

:::hints
- kind: attention
  body: "lam_tinh_gia trả về một lambda CHỈ ĐỌC thue_suat (không print, không sửa gì). lam_sinh_ma dùng nonlocal dem; dem += 1 để đếm lên, ghép vào chuỗi bằng f-string."
- kind: strategy
  body: 'lam_tinh_gia: return lambda gia: gia * (1 + thue_suat). lam_sinh_ma: dem = 0; def sinh(): nonlocal dem; dem += 1; return f"{tien_to}-{dem}"; return sinh.'
- kind: one-line
  body: "# lam_tinh_gia:\nreturn lambda gia: gia * (1 + thue_suat)\n\n# lam_sinh_ma:\ndem = 0\ndef sinh():\n    nonlocal dem\n    dem += 1\n    return f\"{tien_to}-{dem}\"\nreturn sinh"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: lam_tinh_gia phải là closure THUẦN có dùng đến thue_suat (không print, không sửa gì bên ngoài tham số, không được bỏ qua tham số). lam_sinh_ma phải SỬA một biến dem bằng nonlocal để đếm lên mỗi lần gọi.
  requireAst:
  - kind: pure-fn, target: lam_tinh_gia
  - kind: uses-name, target: thue_suat, min: 1
  - kind: gan-ten, target: dem, min: 1
  - kind: uses-name, target: dem, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "HD-1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không phải mọi closure phải thuần — nhưng mọi closure phải được BIẾT
rõ nó thuộc loại nào, để test đúng cách, không đoán nhầm hành vi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn tự tay viết `def sinh(): nonlocal dem; ...` mỗi lần cần một closure
"nhớ sẵn" một phần dữ liệu. Có công cụ CÓ SẴN nào làm việc tương tự,
không cần viết `def`/`lambda` lồng nhau mỗi lần không?

Bài sau giới thiệu `functools.partial`.
::::

::::checkpoint{mastery=0.8}
::::
