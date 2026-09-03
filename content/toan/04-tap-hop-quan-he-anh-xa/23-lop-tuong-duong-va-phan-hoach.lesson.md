---
id: toan.tap-hop-quan-he-anh-xa.lop-tuong-duong-va-phan-hoach
title: Lớp tương đương và phân hoạch
summary: "Lớp tương đương — nhóm mọi luống tương đương nhau thành MỘT lớp; các lớp KHÔNG chồng lấn (đối xứng+bắc cầu) và GỘP LẠI vừa khít cả bảy luống (phản xạ) — gọi là một phân hoạch."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.equivalence-class]
requires: [math.equivalence-relation]
concepts: [math.lop-tuong-duong, math.phan-hoach]
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
Luống 1 và luống 3 cùng lịch tưới Hai-Năm. Luống 2 và luống 5 cùng
lịch tưới Ba. Gom MỌI luống theo "ai cùng lịch với ai" — mỗi luống
rơi vào ĐÚNG MẤY nhóm?
::::

::::explain{#lop-tuong-duong-la-gi}
ĐÚNG MỘT nhóm. **Lớp tương đương** của `x` — TẤT CẢ phần tử tương
đương VỚI `x` (theo quan hệ tương đương, bài 22):

```python title=readonly
luong = {"luong_1", "luong_2", "luong_3", "luong_5"}
lich = {
    "luong_1": {"Hai", "Nam"},
    "luong_2": {"Ba"},
    "luong_3": {"Hai", "Nam"},
    "luong_5": {"Ba"},
}
cung_lich = {(x, y) for x in luong for y in luong if lich[x] == lich[y]}

def lop_cua(x, quan_he, a):
    return {y for y in a if (x, y) in quan_he}

print(sorted(lop_cua("luong_1", cung_lich, luong)))
print(sorted(lop_cua("luong_2", cung_lich, luong)))
```

```text title=readonly
['luong_1', 'luong_3']
['luong_2', 'luong_5']
```

Lớp của `luong_1` LÀ `{luong_1, luong_3}` (CÙNG lịch Hai-Năm). Lớp
của `luong_2` LÀ `{luong_2, luong_5}` (CÙNG lịch Ba). Bốn luống, HAI
lớp — MỖI luống rơi vào ĐÚNG một lớp.
::::

::::example{#dai-dien-nao-cung-duoc}
Chọn `luong_3` (THAY VÌ `luong_1`) LÀM đại diện — RA CÙNG một lớp:

```python title=readonly
luong = {"luong_1", "luong_2", "luong_3", "luong_5"}
lich = {
    "luong_1": {"Hai", "Nam"},
    "luong_2": {"Ba"},
    "luong_3": {"Hai", "Nam"},
    "luong_5": {"Ba"},
}
cung_lich = {(x, y) for x in luong for y in luong if lich[x] == lich[y]}
def lop_cua(x, quan_he, a):
    return {y for y in a if (x, y) in quan_he}

print(lop_cua("luong_1", cung_lich, luong) == lop_cua("luong_3", cung_lich, luong))
```

```text title=readonly
True
```

`luong_1` VÀ `luong_3` CÙNG lớp — nên "lớp CỦA `luong_1`" VÀ "lớp
CỦA `luong_3`" LÀ CÙNG một tập hợp, DÙ gọi TÊN khác nhau. Bất kỳ
THÀNH VIÊN nào trong lớp CŨNG "đại diện" được cho CẢ lớp.
::::

::::predict{#doan-hai-lop-khac-nhau-khong-chong-lan commitOnce}
Byte kiểm HAI lớp KHÁC nhau (của `luong_1` VÀ của `luong_2`) có
CHUNG luống NÀO không:

```python
luong = {"luong_1", "luong_2", "luong_3", "luong_5"}
lich = {
    "luong_1": {"Hai", "Nam"},
    "luong_2": {"Ba"},
    "luong_3": {"Hai", "Nam"},
    "luong_5": {"Ba"},
}
cung_lich = {(x, y) for x in luong for y in luong if lich[x] == lich[y]}
def lop_cua(x, quan_he, a):
    return {y for y in a if (x, y) in quan_he}

lop_1 = lop_cua("luong_1", cung_lich, luong)
lop_2 = lop_cua("luong_2", cung_lich, luong)
print(lop_1 & lop_2)
```

Dòng cuối in ra gì?

:::opt{correct}
`set()`
:::

:::opt
Một tập KHÔNG rỗng — vì `luong_1` VÀ `luong_2` ĐỀU thuộc CÙNG một
khu vườn Byte, nên LỚP của chúng PHẢI chồng lấn Ở ÍT NHẤT một luống
chung, đúng NHƯ quan hệ "cùng khu vườn với" (bài 19-21) đối XỬ
::why
Gần đúng ở việc bạn nhớ TỚI quan hệ "cùng khu vườn với" TỪ các bài
trước — MỘT liên tưởng hợp lý VÌ cả hai đều LÀ quan hệ tương đương.

Chỗ lệch: bài NÀY đang dùng quan hệ "cùng LỊCH tưới", KHÔNG PHẢI
"cùng khu vườn với" — HAI quan hệ KHÁC nhau, chia LỚP khác nhau.
Theo "cùng lịch", `luong_1` (lịch Hai-Năm) VÀ `luong_2` (lịch Ba)
KHÔNG cùng lớp — VÀ đây LÀ tính chất CHUNG của MỌI quan hệ tương
đương: hai lớp KHÁC nhau LUÔN rời nhau (bài 10), KHÔNG BAO GIỜ chồng
lấn — nếu chồng lấn, chúng ĐÃ là CÙNG một lớp RỒI (nhờ đối xứng + bắc
cầu). `lop_1 & lop_2` LUÔN LÀ `set()` khi `lop_1 ≠ lop_2`.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `lop_cua` HAI lần VỚI hai giá trị `x`
khác nhau (`"luong_1"`, `"luong_2"`) rồi đem GIAO (`&`) hai kết quả
LÀ một thao TÁC không hợp lệ giữa HAI lớp tương đương
::why
Gần đúng ở việc bạn để ý ĐANG giao (`&`) HAI kết quả từ HAI lời gọi
KHÁC nhau — một quan sát đúng về CẤU TRÚC lời gọi.

Chỗ lệch: `lop_1` VÀ `lop_2` (SAU khi hàm trả VỀ) chỉ đơn giản LÀ
hai biến `set` bình thường — `&` hoạt động TRÊN chúng y hệt BẤT KỲ
hai `set` nào khác (bài 9). Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_phan_hoach}
Viết `phan_hoach(quan_he, a)` — trả về DANH SÁCH các LỚP tương
đương, gộp LẠI TOÀN BỘ `a` mà KHÔNG lớp nào chồng lấn.

```python title=starter
def lop_cua(x, quan_he, a):
    return {y for y in a if (x, y) in quan_he}


def phan_hoach(quan_he, a):
    da_xu_ly = set()
    cac_lop = []
    for x in a:
        if x not in da_xu_ly:
            lop = ___
            cac_lop.append(lop)
            da_xu_ly = da_xu_ly | lop
    return cac_lop


luong = {"luong_1", "luong_2", "luong_3", "luong_5"}
lich = {
    "luong_1": {"Hai", "Nam"},
    "luong_2": {"Ba"},
    "luong_3": {"Hai", "Nam"},
    "luong_5": {"Ba"},
}
cung_lich = {(x, y) for x in luong for y in luong if lich[x] == lich[y]}

print(len(phan_hoach(cung_lich, luong)))
```

```python title=solution
def lop_cua(x, quan_he, a):
    return {y for y in a if (x, y) in quan_he}


def phan_hoach(quan_he, a):
    da_xu_ly = set()
    cac_lop = []
    for x in a:
        if x not in da_xu_ly:
            lop = lop_cua(x, quan_he, a)
            cac_lop.append(lop)
            da_xu_ly = da_xu_ly | lop
    return cac_lop


luong = {"luong_1", "luong_2", "luong_3", "luong_5"}
lich = {
    "luong_1": {"Hai", "Nam"},
    "luong_2": {"Ba"},
    "luong_3": {"Hai", "Nam"},
    "luong_5": {"Ba"},
}
cung_lich = {(x, y) for x in luong for y in luong if lich[x] == lich[y]}

print(len(phan_hoach(cung_lich, luong)))
```

```python title=test
cac_lop = phan_hoach(cung_lich, luong)
assert sum(len(lop) for lop in cac_lop) == len(luong), "tong so luong trong cac lop phai bang tong so luong"
tong = set()
for lop in cac_lop:
    tong = tong | lop
assert tong == luong, "gop het cac lop phai ra dung tap luong ban dau"
assert len(phan_hoach(set(), set())) == 0, "tap rong -- khong lop nao"
```

:::hints
- kind: attention
  body: "Goi lai lop_cua(x, quan_he, a) da co san de tinh lop cua x."
- kind: strategy
  body: "lop_cua(x, quan_he, a)"
- kind: one-line
  body: "___ = lop_cua(x, quan_he, a)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi lop_cua(x, quan_he, a) de tinh lop cua x, roi gop vao cac_lop
  requireAst:
  - kind: uses-call, target: lop_cua, min: 1
  - kind: uses-name, target: x, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^2\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Phân hoạch = chia trọn vẹn, không sót không chồng. Cụm cuối: mỗi
luống gán cho ĐÚNG MỘT người phụ trách — gọi là gì?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bốn luống chia thành đúng hai lớp lịch-tưới. Đếm số luống mỗi lớp
rồi CỘNG — có đúng bằng bốn không? Trùng hợp, hay quan hệ tương
đương LUÔN bảo đảm điều đó?
::::

::::checkpoint{mastery=0.8}
::::
