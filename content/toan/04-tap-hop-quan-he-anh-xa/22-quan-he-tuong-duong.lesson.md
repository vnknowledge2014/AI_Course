---
id: toan.tap-hop-quan-he-anh-xa.quan-he-tuong-duong
title: Quan hệ tương đương
summary: "Quan hệ tương đương = phản xạ + đối xứng + bắc cầu, đủ cả ba. \"Cùng lịch tưới nước\" (hai luống tưới ĐÚNG cùng những ngày) là một quan hệ tương đương THẬT trên bảy luống, kiểm được bằng all() trên mọi cặp."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.equivalence-relation]
requires: [math.relation-transitive]
concepts: [math.quan-he-tuong-duong]
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
Phản xạ, đối xứng, bắc cầu — "cùng khu vườn với" có ĐỦ CẢ BA. Một
quan hệ đủ cả ba tính chất ấy có tên riêng là gì?
::::

::::explain{#quan-he-tuong-duong-la-gi}
Gọi LÀ **quan hệ tương đương**: phản xạ **VÀ** đối xứng **VÀ** bắc
cầu — đủ cả BA. Thử VỚI quan hệ MỚI "cùng lịch tưới nước" (hai luống
tưới ĐÚNG cùng những ngày):

```python title=readonly
def la_phan_xa(quan_he, a):
    return all((x, x) in quan_he for x in a)
def la_doi_xung(quan_he):
    return all((y, x) in quan_he for (x, y) in quan_he)
def la_bac_cau(quan_he):
    return all((x, z) in quan_he for (x, y) in quan_he for (y2, z) in quan_he if y2 == y)

def la_tuong_duong(quan_he, a):
    return la_phan_xa(quan_he, a) and la_doi_xung(quan_he) and la_bac_cau(quan_he)


luong = {"luong_1", "luong_2", "luong_3", "luong_5"}
lich = {
    "luong_1": {"Hai", "Nam"},
    "luong_2": {"Ba"},
    "luong_3": {"Hai", "Nam"},
    "luong_5": {"Ba"},
}
cung_lich = {(x, y) for x in luong for y in luong if lich[x] == lich[y]}

print(la_tuong_duong(cung_lich, luong))
```

```text title=readonly
True
```

`cung_lich` LÀ quan hệ tương đương THẬT: MỌI luống tưới cùng LỊCH
với chính nó (phản xạ — `lich["luong_1"] == lich["luong_1"]` LUÔN
đúng, DÙ so với bất kỳ giá trị NÀO); "cùng lịch VỚI" đối xứng tự
nhiên (`x` cùng LỊCH `y` thì `y` cũng cùng LỊCH `x`); VÀ bắc cầu
("cùng lịch" bắc CẦU vì so sánh BẰNG NHAU luôn bắc cầu: `a=b` VÀ
`b=c` thì `a=c`).
::::

::::example{#khong-phai-quan-he-nao-cung-tuong-duong}
"Trồng chung ÍT NHẤT một loại" (bài 9, 21) KHÔNG PHẢI quan hệ tương
đương — nó THIẾU tính bắc cầu:

```python title=readonly
def la_phan_xa(quan_he, a):
    return all((x, x) in quan_he for x in a)
def la_doi_xung(quan_he):
    return all((y, x) in quan_he for (x, y) in quan_he)
def la_bac_cau(quan_he):
    return all((x, z) in quan_he for (x, y) in quan_he for (y2, z) in quan_he if y2 == y)
def la_tuong_duong(quan_he, a):
    return la_phan_xa(quan_he, a) and la_doi_xung(quan_he) and la_bac_cau(quan_he)

luong_1s = {"cà chua", "xà lách"}
luong_3s = {"xà lách", "cà rốt"}
luong_6s = {"cà rốt", "khoai lang"}
trong_chung = set()
danh_sach = [("luong_1", luong_1s), ("luong_3", luong_3s), ("luong_6", luong_6s)]
for (t1, s1) in danh_sach:
    for (t2, s2) in danh_sach:
        if s1 & s2:
            trong_chung.add((t1, t2))

print(la_tuong_duong(trong_chung, {"luong_1", "luong_3", "luong_6"}))
```

```text title=readonly
False
```

"Trồng chung" ĐÚNG LÀ phản xạ (mỗi luống chung VỚI chính nó) VÀ đối
xứng — NHƯNG KHÔNG bắc cầu (đã thấy Ở bài 21: luống 1 chung xà lách
với luống 3, luống 3 chung cà rốt VỚI luống 6, nhưng luống 1 VÀ 6
không chung gì). CHỈ ĐỦ hai trong BA — KHÔNG PHẢI tương đương.
::::

::::predict{#doan-kiem-cap-cu-the commitOnce}
Luống 1 VÀ luống 5 CÙNG lịch tưới Hai-Năm — Byte kiểm HAI cặp cụ
thể trong `cung_lich`:

```python
luong = {"luong_1", "luong_2", "luong_3", "luong_5"}
lich = {
    "luong_1": {"Hai", "Nam"},
    "luong_2": {"Ba"},
    "luong_3": {"Hai", "Nam"},
    "luong_5": {"Ba"},
}
cung_lich = {(x, y) for x in luong for y in luong if lich[x] == lich[y]}

print(("luong_1", "luong_3") in cung_lich)
print(("luong_1", "luong_2") in cung_lich)
```

Hai dòng cuối in ra gì?

:::opt{correct}
`True`, rồi `False`
:::

:::opt
`True`, rồi `True` — vì `cung_lich` LÀ quan hệ tương đương (đã kiểm
Ở trên), VÀ MỌI cặp luống trong MỘT quan hệ tương đương ĐỀU liên
quan tới nhau, BẤT KỂ lịch tưới cụ THỂ của chúng LÀ gì
::why
Gần đúng ở việc bạn nhớ ĐÚNG `cung_lich` LÀ một quan hệ tương đương
— một quan sát ĐÚNG về TÍNH CHẤT tổng thể của quan hệ.

Chỗ lệch: "LÀ quan hệ tương đương" KHÔNG có nghĩa "MỌI cặp đều liên
quan" — nó chỉ nói quan hệ ĐÓ tuân theo BA luật (phản xạ/đối xứng/
bắc cầu) MỘT cách NHẤT quán. `lich["luong_1"]` LÀ `{"Hai", "Nam"}`,
`lich["luong_2"]` LÀ `{"Ba"}` — HAI tập KHÁC nhau, nên
`lich["luong_1"] == lich["luong_2"]` LÀ `False`, cặp `("luong_1",
"luong_2")` KHÔNG thuộc `cung_lich`. Ngược lại `luong_1` VÀ `luong_3`
CÙNG lịch (`{"Hai", "Nam"}`) — cặp ĐÓ CÓ mặt.
::
:::

:::opt
Máy báo lỗi biên dịch — `lich` khai HAI luống (`luong_1`, `luong_3`)
CÙNG một giá trị `{"Hai", "Nam"}`, Python CẤM hai KHOÁ khác nhau của
CÙNG một `dict` trỏ tới giá trị GIỐNG hệt nhau
::why
Gần đúng ở việc bạn để ý `luong_1` VÀ `luong_3` CÓ giá trị GIỐNG hệt
— một quan sát đúng về DỮ LIỆU.

Chỗ lệch: Python KHÔNG hề cấm điều ĐÓ — MỘT `dict` HOÀN TOÀN cho
phép NHIỀU khoá TRỎ tới giá trị TRÙNG nhau (thậm chí ĐÓ chính LÀ Ý
nghĩa của "hai luống cùng LỊCH" trong bài NÀY). Biên dịch sạch, chạy
sạch.
::
:::
::::

::::code{#viet_la_tuong_duong}
Viết `la_tuong_duong(quan_he, a)` — kiểm tra `quan_he` có LÀ quan
hệ tương đương TRÊN `a` hay không, GHÉP LẠI cả ba tính chất đã học.

```python title=starter
def la_phan_xa(quan_he, a):
    return all((x, x) in quan_he for x in a)


def la_doi_xung(quan_he):
    return all((y, x) in quan_he for (x, y) in quan_he)


def la_bac_cau(quan_he):
    return all((x, z) in quan_he for (x, y) in quan_he for (y2, z) in quan_he if y2 == y)


def la_tuong_duong(quan_he, a):
    return ___


luong = {"luong_1", "luong_2", "luong_3", "luong_5"}
lich = {
    "luong_1": {"Hai", "Nam"},
    "luong_2": {"Ba"},
    "luong_3": {"Hai", "Nam"},
    "luong_5": {"Ba"},
}
cung_lich = {(x, y) for x in luong for y in luong if lich[x] == lich[y]}

print(la_tuong_duong(cung_lich, luong))
```

```python title=solution
def la_phan_xa(quan_he, a):
    return all((x, x) in quan_he for x in a)


def la_doi_xung(quan_he):
    return all((y, x) in quan_he for (x, y) in quan_he)


def la_bac_cau(quan_he):
    return all((x, z) in quan_he for (x, y) in quan_he for (y2, z) in quan_he if y2 == y)


def la_tuong_duong(quan_he, a):
    return la_phan_xa(quan_he, a) and la_doi_xung(quan_he) and la_bac_cau(quan_he)


luong = {"luong_1", "luong_2", "luong_3", "luong_5"}
lich = {
    "luong_1": {"Hai", "Nam"},
    "luong_2": {"Ba"},
    "luong_3": {"Hai", "Nam"},
    "luong_5": {"Ba"},
}
cung_lich = {(x, y) for x in luong for y in luong if lich[x] == lich[y]}

print(la_tuong_duong(cung_lich, luong))
```

```python title=test
luong_1s = {"cà chua", "xà lách"}
luong_3s = {"xà lách", "cà rốt"}
luong_6s = {"cà rốt", "khoai lang"}
trong_chung = set()
danh_sach = [("luong_1", luong_1s), ("luong_3", luong_3s), ("luong_6", luong_6s)]
for (t1, s1) in danh_sach:
    for (t2, s2) in danh_sach:
        if s1 & s2:
            trong_chung.add((t1, t2))
assert la_tuong_duong(trong_chung, {"luong_1", "luong_3", "luong_6"}) is False, "thieu bac cau"
assert la_tuong_duong(set(), set()) is True, "quan he rong tren tap rong -- ca ba dieu thoa"
assert la_tuong_duong({("a", "a"), ("b", "b")}, {"a", "b"}) is True, "moi phan tu chi lien quan chinh no"
```

:::hints
- kind: attention
  body: "Ghep ca ba ham da co bang and: la_phan_xa VA la_doi_xung VA la_bac_cau."
- kind: strategy
  body: "la_phan_xa(quan_he, a) and la_doi_xung(quan_he) and la_bac_cau(quan_he)"
- kind: one-line
  body: "___ = la_phan_xa(quan_he, a) and la_doi_xung(quan_he) and la_bac_cau(quan_he)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai ghep ca BA ham la_phan_xa/la_doi_xung/la_bac_cau bang and -- thieu mot ham la khong kiem du dieu kien quan he tuong duong
  requireAst:
  - kind: uses-call, target: la_phan_xa, min: 1
  - kind: uses-call, target: la_doi_xung, min: 1
  - kind: uses-call, target: la_bac_cau, min: 1
  - kind: uses-operator, target: and, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tương đương = đủ cả ba, không thiếu một. Bài chốt cụm: gom những
luống tương đương thành từng nhóm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Luống 1 và luống 3 cùng lịch tưới Hai-Năm. Luống 2 và luống 5 cùng
lịch tưới Ba. Gom MỌI luống theo "ai cùng lịch với ai" — mỗi luống
rơi vào ĐÚNG MẤY nhóm?
::::

::::checkpoint{mastery=0.8}
::::
