---
id: toan.tap-hop-quan-he-anh-xa.doi-xung
title: Đối xứng
summary: "Quan hệ đối xứng — nếu A quan hệ với B thì B CŨNG quan hệ với A. \"Cùng khu vườn với\" đối xứng, còn \"tưới TRƯỚC\" (luống 1 trước luống 3) thì KHÔNG."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.relation-symmetric]
requires: [math.relation-reflexive]
concepts: [math.doi-xung]
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
Nếu luống A "cùng khu vườn với" luống B, thì B có "cùng khu vườn
với" A không?
::::

::::explain{#doi-xung-la-gi}
Có — MỘT quan HỆ CÙNG-VƯỜN thì rõ ràng phải NHƯ vậy. Đó chính LÀ tính
chất **đối xứng**: MỘT quan hệ LÀ đối xứng khi — VỚI MỌI cặp `(x, y)`
ĐÃ có trong quan hệ — cặp NGƯỢC LẠI `(y, x)` CŨNG có mặt.

```python title=readonly
luong = {"luong_1", "luong_2", "luong_3"}
cung_vuon = {(x, y) for x in luong for y in luong}

def la_doi_xung(quan_he):
    return all((y, x) in quan_he for (x, y) in quan_he)

print(la_doi_xung(cung_vuon))
```

```text title=readonly
True
```

`cung_vuon` LÀ toàn bộ tích Descartes — VỚI MỌI `(x, y)` Ở trong,
`(y, x)` ĐƯƠNG NHIÊN cũng Ở trong (vì `cung_vuon` chứa MỌI cặp có
thể). Đối xứng.
::::

::::example{#khong-phai-quan-he-nao-cung-doi-xung}
"Tưới TRƯỚC" (luống 1 tưới trước luống 3, MỘT quan hệ THỨ TỰ) thì
KHÔNG đối xứng:

```python title=readonly
tuoi_truoc = {("luong_1", "luong_2"), ("luong_2", "luong_3")}

def la_doi_xung(quan_he):
    return all((y, x) in quan_he for (x, y) in quan_he)

print(la_doi_xung(tuoi_truoc))
```

```text title=readonly
False
```

`("luong_1", "luong_2")` CÓ trong `tuoi_truoc` — NHƯNG `("luong_2",
"luong_1")` thì KHÔNG (luống 2 KHÔNG tưới trước luống 1, nó tưới
SAU). MỘT phản ví dụ LÀ đủ — KHÔNG đối xứng.
::::

::::predict{#doan-cap-tu-minh commitOnce}
Byte thử một quan hệ CHỈ có ĐÚNG MỘT cặp — VÀ cặp ĐÓ LÀ một luống với
CHÍNH nó:

```python
def la_doi_xung(quan_he):
    return all((y, x) in quan_he for (x, y) in quan_he)

chi_co_tu_minh = {("luong_1", "luong_1")}
print(la_doi_xung(chi_co_tu_minh))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì quan hệ CHỈ CÓ ĐÚNG MỘT cặp DUY NHẤT, và MỘT quan hệ
"quá nhỏ" (chỉ MỘT phần tử) KHÔNG đủ ĐIỀU KIỆN để tính LÀ đối xứng —
đối xứng CẦN ít nhất HAI cặp KHÁC nhau để "soi chiếu" lẫn nhau
::why
Gần đúng ở việc bạn để ý quan hệ NÀY CHỈ có ĐÚNG một cặp — một quan
sát đúng về KÍCH THƯỚC.

Chỗ lệch: đối xứng KHÔNG đòi hỏi "ÍT NHẤT hai cặp" — nó chỉ đòi hỏi
VỚI MỌI cặp `(x, y)` CÓ trong quan hệ, `(y, x)` CŨNG có mặt. Ở ĐÂY
`x = y = "luong_1"` — cặp `(x, y)` LÀ `("luong_1", "luong_1")`, và
cặp NGƯỢC `(y, x)` CŨNG chính LÀ `("luong_1", "luong_1")` (đảo MỘT
cặp CÓ hai phần tử GIỐNG nhau thì RA lại CHÍNH nó). Cặp đó ĐÃ có
trong quan hệ — điều kiện thoả NGAY, `True`.
::
:::

:::opt
Máy báo lỗi biên dịch — `chi_co_tu_minh` chỉ khai MỘT phần tử duy
nhất trong tập hợp, Python YÊU CẦU một quan hệ (tập hợp các cặp)
phải có TỐI THIỂU hai phần tử để `all()` có ý nghĩa kiểm tra
::why
Gần đúng ở việc bạn để ý `chi_co_tu_minh` CHỈ có MỘT phần tử — một
quan sát đúng về KÍCH THƯỚC tập hợp.

Chỗ lệch: `all()` (và MỌI tập hợp Python) hoạt động HOÀN HẢO với BẤT
KỲ số lượng phần tử NÀO, kể CẢ một hay KHÔNG phần tử nào (chân lý
rỗng, bài 6). Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_la_doi_xung}
Viết `la_doi_xung(quan_he)` — kiểm tra `quan_he` có đối xứng hay
không.

```python title=starter
def la_doi_xung(quan_he):
    return ___


luong = {"luong_1", "luong_2", "luong_3"}
cung_vuon = {(x, y) for x in luong for y in luong}

print(la_doi_xung(cung_vuon))
```

```python title=solution
def la_doi_xung(quan_he):
    return all((y, x) in quan_he for (x, y) in quan_he)


luong = {"luong_1", "luong_2", "luong_3"}
cung_vuon = {(x, y) for x in luong for y in luong}

print(la_doi_xung(cung_vuon))
```

```python title=test
tuoi_truoc = {("luong_1", "luong_2"), ("luong_2", "luong_3")}
assert la_doi_xung(tuoi_truoc) is False, "khong co canh nguoc"
assert la_doi_xung(set()) is True, "quan he rong -- doi xung hien nhien"
assert la_doi_xung({("a", "b"), ("b", "a")}) is True, "du ca hai chieu"
assert la_doi_xung({("a", "b")}) is False, "thieu chieu nguoc (b, a)"
```

:::hints
- kind: attention
  body: "Dung all() kiem MOI cap (x, y) trong quan_he, cap dao nguoc (y, x) co cung nam trong quan_he khong."
- kind: strategy
  body: "all((y, x) in quan_he for (x, y) in quan_he)"
- kind: one-line
  body: "___ = all((y, x) in quan_he for (x, y) in quan_he)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung all() kiem MOI cap (x, y) trong quan_he, xem cap dao (y, x) co nam trong quan_he khong
  requireAst:
  - kind: uses-call, target: all, min: 1
  - kind: uses-operator, target: in, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đối xứng = mỗi cặp có chiều ngược lại. Bài sau: nếu A quan hệ B, B
quan hệ C, thì A có quan hệ C không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

"Luống A trồng chung ÍT NHẤT một loại rau với luống B" (dùng lại `∩`
bài 9, khác `∅`) — quan hệ này có đối xứng không? Thử luống 1 và
luống 3 xem.
::::

::::checkpoint{mastery=0.8}
::::
