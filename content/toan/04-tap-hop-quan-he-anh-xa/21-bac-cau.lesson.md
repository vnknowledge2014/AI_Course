---
id: toan.tap-hop-quan-he-anh-xa.bac-cau
title: Bắc cầu
summary: "Quan hệ bắc cầu — nếu A quan hệ B, và B quan hệ C, thì A PHẢI quan hệ C. \"Cùng khu vườn với\" bắc cầu; \"trồng chung ít nhất một loại\" thì KHÔNG (1 chung xà lách với 3, 3 chung cà rốt với 6, nhưng 1 và 6 có thể chẳng chung gì)."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.relation-transitive]
requires: [math.relation-symmetric]
concepts: [math.bac-cau]
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
Nếu luống A "cùng khu vườn với" B, và B "cùng khu vườn với" C, thì A
có "cùng khu vườn với" C không?
::::

::::explain{#bac-cau-la-gi}
Có — dĩ NHIÊN, cả BA cùng một khu vườn. Đó chính LÀ tính chất **bắc
cầu**: nếu A quan hệ B, VÀ B quan hệ C, THÌ A **PHẢI** quan hệ C.

```python title=readonly
luong = {"luong_1", "luong_2", "luong_3"}
cung_vuon = {(x, y) for x in luong for y in luong}

def la_bac_cau(quan_he):
    return all((x, z) in quan_he for (x, y) in quan_he for (y2, z) in quan_he if y2 == y)

print(la_bac_cau(cung_vuon))
```

```text title=readonly
True
```

`la_bac_cau` LẶP qua MỌI cặp `(x, y)`, RỒI với MỌI cặp `(y2, z)` mà
`y2` KHỚP `y` (đúng LÀ nối tiếp: kết thúc CỦA cặp đầu = bắt đầu CỦA
cặp sau), kiểm `(x, z)` CÓ mặt hay KHÔNG. `cung_vuon` chứa MỌI cặp
có thể — LUÔN thoả.
::::

::::example{#trong-chung-khong-bac-cau}
"Trồng chung ÍT NHẤT một loại rau" (dùng `∩`, bài 9) thì KHÔNG bắc
cầu:

```python title=readonly
luong_1 = {"cà chua", "xà lách"}
luong_3 = {"xà lách", "cà rốt"}
luong_6 = {"cà rốt", "khoai lang"}

trong_chung = set()
danh_sach = [("luong_1", luong_1), ("luong_3", luong_3), ("luong_6", luong_6)]
for (t1, s1) in danh_sach:
    for (t2, s2) in danh_sach:
        if s1 & s2:
            trong_chung.add((t1, t2))

print(("luong_1", "luong_3") in trong_chung)
print(("luong_3", "luong_6") in trong_chung)
print(("luong_1", "luong_6") in trong_chung)
```

```text title=readonly
True
True
False
```

Luống 1 chung `xà lách` VỚI luống 3 — CÓ quan hệ. Luống 3 chung
`cà rốt` VỚI luống 6 — CÓ quan hệ. NHƯNG luống 1 VÀ luống 6 (`{cà
chua, xà lách}` vs `{cà rốt, khoai lang}`) — KHÔNG chung LOẠI nào cả.
`A` quan hệ `B`, `B` quan hệ `C`, NHƯNG `A` **KHÔNG** quan hệ `C` —
phá vỡ bắc cầu.
::::

::::predict{#doan-tuoi-truoc-khong-bac-cau commitOnce}
Byte thử LẠI "tưới TRƯỚC" (T2.4 bài 19-20, quan hệ THỨ TỰ):

```python
def la_bac_cau(quan_he):
    return all((x, z) in quan_he for (x, y) in quan_he for (y2, z) in quan_he if y2 == y)

tuoi_truoc = {("luong_1", "luong_2"), ("luong_2", "luong_3")}
print(la_bac_cau(tuoi_truoc))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì "tưới TRƯỚC" LÀ một quan hệ THỨ TỰ hợp lý (luống 1 trước
luống 2, luống 2 trước luống 3), VÀ mọi quan hệ thứ TỰ tự nhiên đều
BẮC CẦU (nếu trước MỘT cái, TRƯỚC luôn cả cái sau nó)
::why
Gần đúng ở việc bạn nhận RA "tưới trước" LÀ một quan hệ THỨ TỰ có Ý
NGHĨA — VÀ nhiều quan hệ thứ tự (như `<` trên số) đúng THẬT SỰ bắc
cầu.

Chỗ lệch: BẮC CẦU không tự động đến TỪ "có vẻ LÀ thứ tự" — nó PHẢI
được KIỂM bằng đúng dữ liệu ĐANG có. `tuoi_truoc` CHỈ chứa HAI cặp:
`(luong_1, luong_2)` VÀ `(luong_2, luong_3)` — nhưng KHÔNG chứa
`(luong_1, luong_3)`! Về mặt Ý NGHĨA, luống 1 "tưới trước" luống 3
LÀ hợp lý — NHƯNG quan hệ NÀY (đúng NHƯ được VIẾT ra) KHÔNG hề ghi
cặp ĐÓ. Bắc cầu kiểm ĐÚNG những gì CÓ TRONG dữ liệu, KHÔNG suy luận
"lẽ ra phải có". `la_bac_cau(tuoi_truoc)` LÀ `False`.
::
:::

:::opt
Máy báo lỗi biên dịch — `la_bac_cau` gọi `quan_he` BA lần lồng nhau
(trong `all(...)`, VÀ hai vòng `for` bên trong) là một cấu trúc lồng
quá SÂU, Python giới hạn TỐI ĐA hai tầng lồng cho comprehension
::why
Gần đúng ở việc bạn để ý biểu thức LỒNG khá SÂU (một `all`, HAI `for`,
một `if`) — một quan sát đúng về ĐỘ PHỨC TẠP cú pháp.

Chỗ lệch: Python KHÔNG giới hạn số tầng LỒNG của generator expression
— lồng bao NHIÊU tầng CŨNG hợp lệ (dù đọc CÓ thể khó hơn). Biên dịch
sạch, chạy sạch.
::
:::
::::

::::code{#viet_la_bac_cau}
Viết `la_bac_cau(quan_he)` — kiểm tra `quan_he` có bắc cầu hay
không.

```python title=starter
def la_bac_cau(quan_he):
    return ___


luong = {"luong_1", "luong_2", "luong_3"}
cung_vuon = {(x, y) for x in luong for y in luong}

print(la_bac_cau(cung_vuon))
```

```python title=solution
def la_bac_cau(quan_he):
    return all((x, z) in quan_he for (x, y) in quan_he for (y2, z) in quan_he if y2 == y)


luong = {"luong_1", "luong_2", "luong_3"}
cung_vuon = {(x, y) for x in luong for y in luong}

print(la_bac_cau(cung_vuon))
```

```python title=test
tuoi_truoc = {("luong_1", "luong_2"), ("luong_2", "luong_3")}
assert la_bac_cau(tuoi_truoc) is False, "thieu (luong_1, luong_3)"
assert la_bac_cau(set()) is True, "quan he rong -- bac cau hien nhien"
assert la_bac_cau({("a", "b"), ("b", "c"), ("a", "c")}) is True, "du ca canh noi tiep"
assert la_bac_cau({("a", "b"), ("b", "c")}) is False, "thieu (a, c)"
```

:::hints
- kind: attention
  body: "Dung all() kiem: voi moi (x, y) va (y2, z) ma y2 bang y, cap (x, z) phai co trong quan_he."
- kind: strategy
  body: "all((x, z) in quan_he for (x, y) in quan_he for (y2, z) in quan_he if y2 == y)"
- kind: one-line
  body: "___ = all((x, z) in quan_he for (x, y) in quan_he for (y2, z) in quan_he if y2 == y)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung all() voi hai vong lap long nhau, noi cap (x,y) va (y2,z) khi y2==y, roi kiem (x,z) co trong quan_he
  requireAst:
  - kind: uses-call, target: all, min: 1
  - kind: uses-operator, target: '==', min: 1
  - kind: uses-operator, target: in, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bắc cầu = nối tiếp phải giữ được. Ba tính chất xong — bài sau: khi
CẢ BA cùng đúng, quan hệ đó có tên riêng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba tính chất vừa học — phản xạ, đối xứng, bắc cầu — "cùng khu vườn
với" có ĐỦ CẢ BA. Một quan hệ đủ cả ba tính chất ấy có tên riêng là
gì?
::::

::::checkpoint{mastery=0.8}
::::
