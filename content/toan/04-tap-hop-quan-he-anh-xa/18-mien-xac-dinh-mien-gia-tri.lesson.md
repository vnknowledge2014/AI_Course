---
id: toan.tap-hop-quan-he-anh-xa.mien-xac-dinh-mien-gia-tri
title: Miền xác định và miền giá trị
summary: "Miền xác định (mọi phần tử XUẤT HIỆN Ở VỊ TRÍ ĐẦU ít nhất một cặp) và miền giá trị (mọi phần tử Ở VỊ TRÍ SAU) — có thể KHÁC A và B gốc nếu vài phần tử chưa tham gia cặp nào."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.relation-domain-range]
requires: [math.relation-table, math.set-builder]
concepts: [math.mien-xac-dinh, math.mien-gia-tri]
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
Đọc theo HÀNG của luống 1: hai ô đánh dấu. Đọc theo CỘT của thứ Hai:
mấy ô — cột đó nói lên điều gì mà hàng không nói được?
::::

::::explain{#mien-xac-dinh-va-mien-gia-tri}
Cả hàng LẪN cột đều gọi TÊN một tập hợp RIÊNG. **Miền xác định** —
mọi phần tử XUẤT HIỆN Ở VỊ TRÍ ĐẦU của ÍT NHẤT một cặp (mọi TÊN hàng
CÓ ô đánh dấu). Dùng set-builder (bài 4) để dựng NÓ từ quan hệ:

```python title=readonly
tuoi_nuoc = {("luong_1", "Hai"), ("luong_2", "Ba"), ("luong_2", "Hai")}

mien_xd = {x for (x, y) in tuoi_nuoc}
print(sorted(mien_xd))
```

```text title=readonly
['luong_1', 'luong_2']
```

`mien_xd` LÀ tập hợp mọi `x` sao cho TỒN TẠI `y` để `(x, y) ∈
tuoi_nuoc` — CHÍNH XÁC những luống CÓ ÍT NHẤT một ô đánh dấu Ở BẢNG
bài 17. `luong_1` VÀ `luong_2` ĐỀU có mặt — CẢ HAI đều tưới NGÀY nào
đó.
::::

::::example{#mien-gia-tri}
**Miền giá trị** — mọi phần tử Ở VỊ TRÍ SAU của ÍT NHẤT một cặp
(mọi TÊN cột CÓ ô đánh dấu):

```python title=readonly
tuoi_nuoc = {("luong_1", "Hai"), ("luong_2", "Ba"), ("luong_2", "Hai")}

mien_gt = {y for (x, y) in tuoi_nuoc}
print(sorted(mien_gt))
```

```text title=readonly
['Ba', 'Hai']
```

`mien_gt` LÀ mọi `y` sao cho TỒN TẠI `x` để `(x, y) ∈ tuoi_nuoc` —
những NGÀY có ÍT NHẤT một luống được tưới. Chỉ `"Hai"` VÀ `"Ba"` —
NẾU tuần có bảy ngày, năm ngày CÒN LẠI KHÔNG có mặt Ở `mien_gt` (dù
CHÚNG có mặt trong tập BẢY ngày gốc).
::::

::::predict{#doan-luong-chua-tuoi commitOnce}
Byte có bảy luống (`luong_1`, `luong_2`, VÀ NĂM luống khác, gồm
`luong_7` — CHƯA từng được ghi Ở lịch tưới `tuoi_nuoc` bên trên):

```python
tuoi_nuoc = {("luong_1", "Hai"), ("luong_2", "Ba"), ("luong_2", "Hai")}
mien_xd = {x for (x, y) in tuoi_nuoc}

luong_full = {"luong_1", "luong_2", "luong_7"}

print("luong_7" in luong_full)
print("luong_7" in mien_xd)
```

Hai dòng cuối in ra gì?

:::opt{correct}
`True`, rồi `False`
:::

:::opt
`True`, rồi `True` — vì `luong_7` LÀ MỘT trong bảy luống của cả
vườn, và miền xác định LẼ RA phải CHỨA MỌI luống Ở TRONG vườn, KHÔNG
CHỈ những luống ĐANG có lịch tưới GHI SẴN
::why
Gần đúng ở việc bạn nghĩ TỚI "miền xác định" NHƯ LÀ "toàn bộ tập A
gốc" (`luong_full`) — MỘT cách hiểu DỄ nhầm nếu chưa phân biệt hai
khái niệm.

Chỗ lệch: miền xác định KHÔNG PHẢI `A` (tập gốc); nó CHỈ LÀ những
phần tử của `A` THẬT SỰ xuất hiện Ở VỊ TRÍ ĐẦU trong quan hệ — CÓ THỂ
NHỎ HƠN `A`. `luong_7` KHÔNG hề xuất hiện trong BẤT KỲ cặp nào của
`tuoi_nuoc` — nó CHƯA từng được tưới. `"luong_7" in mien_xd` LÀ
`False`, DÙ `"luong_7" in luong_full` LÀ `True`. Miền xác định VÀ
tập gốc CÓ THỂ khác nhau — đó chính LÀ Ý nghĩa của bài NÀY.
::
:::

:::opt
Máy báo lỗi biên dịch — `luong_full` khai BA phần tử NHƯNG
`tuoi_nuoc` chỉ nhắc TỚI HAI luống (`luong_1`, `luong_2`), Python
CẤM một tập hợp `luong_full` chứa phần tử KHÔNG xuất hiện Ở BẤT KỲ
tập hợp NÀO khác trong CÙNG chương trình
::why
Gần đúng ở việc bạn để ý `luong_7` KHÔNG xuất hiện trong `tuoi_nuoc`
— một quan sát đúng về DỮ LIỆU.

Chỗ lệch: Python KHÔNG hề "liên KẾT" các tập hợp với nhau theo cách
ĐÓ — MỖI tập hợp HOÀN TOÀN độc LẬP, và MỘT phần tử của tập NÀY
KHÔNG BẮT BUỘC phải xuất hiện Ở tập KHÁC. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_mien_xac_dinh_gia_tri}
Viết `mien_xac_dinh(quan_he)` VÀ `mien_gia_tri(quan_he)` — trả về
tập hợp CÁC phần tử Ở VỊ TRÍ ĐẦU / VỊ TRÍ SAU của quan hệ.

```python title=starter
def mien_xac_dinh(quan_he):
    return ___


def mien_gia_tri(quan_he):
    return ___


tuoi_nuoc = {("luong_1", "Hai"), ("luong_2", "Ba"), ("luong_2", "Hai")}

print(sorted(mien_xac_dinh(tuoi_nuoc)))
```

```python title=solution
def mien_xac_dinh(quan_he):
    return {x for (x, y) in quan_he}


def mien_gia_tri(quan_he):
    return {y for (x, y) in quan_he}


tuoi_nuoc = {("luong_1", "Hai"), ("luong_2", "Ba"), ("luong_2", "Hai")}

print(sorted(mien_xac_dinh(tuoi_nuoc)))
```

```python title=test
assert mien_gia_tri(tuoi_nuoc) == {"Hai", "Ba"}, "mien gia tri dung hai ngay"
assert mien_xac_dinh(set()) == set(), "quan he rong -- mien xac dinh rong"
assert mien_gia_tri(set()) == set(), "quan he rong -- mien gia tri rong"
assert mien_xac_dinh({("a", 1), ("a", 2)}) == {"a"}, "cung mot vi tri dau, chi giu mot lan"
assert mien_gia_tri({("a", 1), ("a", 2)}) == {1, 2}, "hai vi tri sau khac nhau -- giu ca hai"
```

:::hints
- kind: attention
  body: "Dung set-builder (bai 4): {x for (x, y) in quan_he} lay vi tri dau, {y for (x, y) in quan_he} lay vi tri sau."
- kind: strategy
  body: "{x for (x, y) in quan_he} : {y for (x, y) in quan_he}"
- kind: one-line
  body: "___ (mien_xac_dinh) = {x for (x, y) in quan_he}\n___ (mien_gia_tri) = {y for (x, y) in quan_he}"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung set comprehension tren quan_he, tach cap (x, y) -- mien_xac_dinh lay x, mien_gia_tri lay y
  requireAst:
  - kind: comprehension, min: 2
  - kind: uses-name, target: quan_he, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\['luong_1', 'luong_2'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Miền xác định có thể nhỏ hơn tập gốc — không phải mọi phần tử đều
tham gia quan hệ. Bài sau: một quan hệ MỚI — "cùng khu vườn với".
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Luống 7 chưa từng được tưới — nó có mặt trong tập bảy luống, nhưng
KHÔNG có mặt trong miền xác định. Điều đó nói gì về luống 7, so với
luống 1 (đã tưới ít nhất một lần)?
::::

::::checkpoint{mastery=0.8}
::::
