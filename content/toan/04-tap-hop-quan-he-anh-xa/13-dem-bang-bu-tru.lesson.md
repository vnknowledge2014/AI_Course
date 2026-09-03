---
id: toan.tap-hop-quan-he-anh-xa.dem-bang-bu-tru
title: Đếm bằng bù trừ
summary: "`|A ∪ B| = |A| + |B| − |A ∩ B|` — cộng thẳng |A|+|B| đếm phần chung HAI LẦN, phải TRỪ lại một lần. Bài đếm PHẦN TỬ của tập hợp (một con số), không đếm chính tập hợp."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.inclusion-exclusion]
requires: [math.set-complement, core.len]
concepts: [math.bu-tru, math.dem-hai-lan]
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
Luống 1 có 2 loại, luống 3 có 2 loại, giao nhau 1 loại. Cộng thẳng
`2 + 2` được `4` — nhưng hợp của chúng chỉ có 3 loại (bài 8). Chỗ dư
ra đi đâu?
::::

::::explain{#bu-tru-la-gi}
Đi vào chỗ bị ĐẾM HAI LẦN. **`|A ∪ B| = |A| + |B| − |A ∩ B|`**: cộng
thẳng `|A| + |B|` đếm phần CHUNG (`A ∩ B`) HAI LẦN — MỘT lần khi đếm
`A`, MỘT lần khi đếm `B` — nên phải TRỪ LẠI đúng MỘT lần.

```python title=readonly
luong_1 = {"cà chua", "xà lách"}
luong_3 = {"xà lách", "cà rốt"}

print(len(luong_1) + len(luong_3) - len(luong_1 & luong_3))
print(len(luong_1 | luong_3))
```

```text title=readonly
3
3
```

`2 + 2 − 1 = 3`, khớp ĐÚNG số phần tử THẬT của `luống_1 ∪ luống_3`
(đã đếm trực tiếp Ở bài 8: `{cà chua, xà lách, cà rốt}`). `xà lách`
(phần CHUNG) bị cộng hai lần Ở `2 + 2`, rồi TRỪ lại đúng một lần.
::::

::::example{#cong-thuc-luon-khop}
Công thức LUÔN khớp với đếm trực tiếp — thử VỚI cặp luống KHÁC:

```python title=readonly
luong_4 = {"bí đỏ", "khoai lang", "đậu que"}
luong_6 = {"khoai lang"}

print(len(luong_4) + len(luong_6) - len(luong_4 & luong_6))
print(len(luong_4 | luong_6))
```

```text title=readonly
3
3
```

`3 + 1 − 1 = 3` (giao CHỈ có `khoai lang`, đếm TRỪ đúng một lần) —
khớp `len(luống_4 ∪ luống_6)` = 3 (`bí đỏ`, `khoai lang`, `đậu que`).
::::

::::predict{#doan-hai-tap-roi-nhau commitOnce}
Byte thử công thức VỚI hai tập RỜI NHAU (bài 10 — KHÔNG chung phần
tử nào):

```python
luong_a = {"a", "b"}
luong_b = {"c", "d"}

print(len(luong_a) + len(luong_b) - len(luong_a & luong_b))
print(len(luong_a | luong_b))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`4`, rồi `4`
:::

:::opt
`3`, rồi `4` — vì công THỨC bù trừ LUÔN trừ đi MỘT (`− 1`), BẤT KỂ
hai tập CÓ chung phần tử hay KHÔNG, nên `2 + 2 − 1 = 3` LUÔN LÀ kết
quả của phần `len() + len() - len(&)`
::why
Gần đúng ở việc bạn NHỚ công thức có phần `− |A ∩ B|` — MỘT quan sát
đúng về HÌNH DẠNG công thức.

Chỗ lệch: `− |A ∩ B|` KHÔNG PHẢI "luôn trừ MỘT" — nó trừ ĐÚNG số
phần tử của GIAO, và giao CÓ THỂ LÀ `0` (tập rỗng, KHI hai tập rời
nhau — bài 10). `luống_a ∩ luống_b` LÀ `∅` (không chung `a,b,c,d`
nào) — `len(∅)` LÀ `0`, KHÔNG PHẢI `1`. Công thức trở THÀNH
`2 + 2 − 0 = 4`, khớp đúng đếm trực TIẾP `len({a,b,c,d})` = `4`. Đây
chính LÀ trường hợp "cộng thẳng KHÔNG bị đếm sai" — vì KHÔNG có gì
chung để đếm HAI lần.
::
:::

:::opt
Máy báo lỗi biên dịch — công thức `len(a) + len(b) - len(a & b)` áp
DỤNG cho hai tập RỜI NHAU (`a ∩ b = ∅`) sẽ khiến `len(a & b)` cố
GỌI `len` TRÊN một giá TRỊ `None` thay vì một `set`, vì "không CÓ
giao" nghĩa LÀ giao KHÔNG TỒN TẠI
::why
Gần đúng ở việc bạn nghĩ TỚI khả năng "không CÓ giao" nghĩa LÀ "giao
không TỒN TẠI" theo MỘT nghĩa nào đó — một cách hiểu CÓ VẺ hợp lý
nếu chưa quen `∅`.

Chỗ lệch: bài 5-6 ĐÃ xác lập RÕ: tập RỖNG `∅` LÀ một tập hợp HỢP LỆ,
KHÔNG PHẢI "không TỒN TẠI" hay `None`. `a & b` (VỚI `a`, `b` rời
nhau) tính RA CHÍNH XÁC `set()` — MỘT `set` THẬT, chỉ là RỖNG —
`len(set())` hoàn toàn hợp lệ, trả VỀ `0`. Biên dịch sạch.
::
:::
::::

::::code{#viet_so_luong_hop}
Viết `so_luong_hop(a, b)` — trả về SỐ LƯỢNG phần tử của `a ∪ b`,
tính bằng CÔNG THỨC bù trừ (KHÔNG gọi `len(a | b)` trực tiếp).

```python title=starter
def so_luong_hop(a, b):
    return ___


print(so_luong_hop({"a", "b"}, {"b", "c"}))
```

```python title=solution
def so_luong_hop(a, b):
    return len(a) + len(b) - len(a & b)


print(so_luong_hop({"a", "b"}, {"b", "c"}))
```

```python title=test
assert so_luong_hop(set(), {"a", "b"}) == 2, "mot ben rong -- ket qua bang do dai ben kia"
assert so_luong_hop(set(), set()) == 0, "ca hai rong -- ket qua 0"
assert so_luong_hop({"a", "b", "c"}, {"a", "b", "c"}) == 3, "hai tap giong het -- khong dem trung"
assert so_luong_hop({"a"}, {"b"}) == 2, "roi nhau -- cong thang, khong tru gi"
assert so_luong_hop({"a", "b"}, {"b", "c"}) == len({"a", "b"} | {"b", "c"}), "khop voi dem truc tiep bang len(a | b)"
```

:::hints
- kind: attention
  body: "Cong thuc: len(a) + len(b) - len(giao cua a va b). Dung & de tinh giao truoc khi dem."
- kind: strategy
  body: "len(a) + len(b) - len(a & b)"
- kind: one-line
  body: "___ = len(a) + len(b) - len(a & b)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung CONG THUC bu tru len(a) + len(b) - len(a giao b), khong duoc goi thang len(a | b) -- bai nay muon ban tu tay ap dung cong thuc, khong phai dem truc tiep
  requireAst:
  - kind: uses-call, target: len, min: 3
  - kind: uses-name, target: a, min: 1
  - kind: uses-name, target: b, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^3\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bù trừ = trừ lại đúng phần bị đếm hai lần. Cụm tiếp theo: một chuyện
mà THỨ TỰ kể lại CÓ nghĩa — trái ngược hẳn tập hợp.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

"Luống 1 tưới thứ Hai" viết thành cặp `(luống_1, "Hai")`. Byte có bảy
luống, tuần có bảy ngày. Liệt kê TẤT CẢ cặp (luống, ngày) CÓ THỂ có —
kể cả cặp không tưới thật — thì được bao nhiêu cặp?
::::

::::checkpoint{mastery=0.8}
::::
