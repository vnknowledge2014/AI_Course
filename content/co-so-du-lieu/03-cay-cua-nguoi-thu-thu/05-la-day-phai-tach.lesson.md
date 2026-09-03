---
id: co-so-du-lieu.cay-cua-nguoi-thu-thu.la-day-phai-tach
title: Lá đầy phải tách
summary: "Khi số khoá VƯỢT quá bac_toi_da, tach_la CẮT đôi node lá THÀNH hai — nửa đầu Ở LẠI, nửa sau thành một node MỚI. khoa đầu tiên của nửa MỚI (khoa_tach) sẽ cần được ĐẨY lên node cha để phân biệt hai nhánh. Đủ ĐÚNG bac_toi_da khoá VẪN chưa cần tách — chỉ VƯỢT quá mới tách."
locale: vi
track: co-so-du-lieu
module: cay-cua-nguoi-thu-thu
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.leaf-split]
requires: [db.insert-in-leaf]
concepts: [db.leaf-split]
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
Một sector chỉ CHỨA được BAO nhiêu byte LÀ có giới hạn — node LÁ
KHÔNG thể phình TO vô hạn. Đầy RỒI, chèn thêm MỘT khoá nữa thì SAO?
::::

::::explain{#tach-doi-node}
`tach_la` CẮT đôi node — NỬA đầu Ở LẠI, nửa SAU thành một node
MỚI, khi số khoá VƯỢT quá `bac_toi_da` (số khoá tối ĐA cho phép
MỘT node):

```python title=readonly
def tach_la(la, bac_toi_da):
    if len(la['khoa']) <= bac_toi_da:
        return None
    giua = len(la['khoa']) // 2
    la_moi = {'loai': 'la', 'khoa': la['khoa'][giua:], 'gia_tri': la['gia_tri'][giua:], 'la_tiep': la['la_tiep']}
    la['khoa'] = la['khoa'][:giua]
    la['gia_tri'] = la['gia_tri'][:giua]
    khoa_tach = la_moi['khoa'][0]
    return (khoa_tach, la_moi)


la = {'loai': 'la', 'khoa': [1, 2, 3, 4], 'gia_tri': ['a', 'b', 'c', 'd'], 'la_tiep': None}
kq = tach_la(la, 3)
print(la['khoa'], kq[0], kq[1]['khoa'])
```

```text title=readonly
[1, 2] 3 [3, 4]
```

`4` khoá VƯỢT quá `bac_toi_da=3` — cắt ĐÔI Ở `giua = 4 // 2 = 2`:
`la` giữ LẠI `[1, 2]`, node MỚI nhận `[3, 4]`. `khoa_tach = 3` (khoá
ĐẦU tiên của node MỚI) — con SỐ này sẽ cần ĐẨY lên node CHA để phân
biệt "nhánh NÀO chứa khoá NÀO".
::::

::::example{#du-dung-chua-can-tach}
ĐỦ đúng `bac_toi_da` khoá (KHÔNG nhiều hơn) — VẪN CHƯA cần tách:

```python title=readonly
la_vua_du = {'loai': 'la', 'khoa': [1, 2, 3], 'gia_tri': ['a', 'b', 'c'], 'la_tiep': None}
print(tach_la(la_vua_du, 3))
```

```text title=readonly
None
```

Điều KIỆN LÀ `len(khoa) <= bac_toi_da` → trả VỀ `None` (KHÔNG tách).
`3` khoá VỚI `bac_toi_da=3` VẪN "vừa đủ CHỖ" — chỉ khi VƯỢT quá
(`4` trở LÊN) mới thật SỰ cần tách.
::::

::::predict{#doan-du-dung-bac-toi-da commitOnce}
Byte gọi `tach_la` TRÊN một node CÓ đúng `bac_toi_da=3` khoá:

```python
la = {'loai': 'la', 'khoa': [10, 20, 30], 'gia_tri': ['a', 'b', 'c'], 'la_tiep': None}
print(tach_la(la, 3))
```

Dòng cuối in ra gì?

:::opt{correct}
`None`
:::

:::opt
Một CẶP `(khoa_tach, node_moi)` — vì `3` khoá VỚI `bac_toi_da=3`
nghĩa LÀ node ĐÃ "đầy", NÊN phải tách NGAY
::why
Gần đúng ở việc bạn nghĩ "ĐẦY" nghĩa LÀ "đủ ĐÚNG giới hạn CHO phép"
— một cách HIỂU hợp lý CỦA từ "đầy" TRONG đời sống hằng ngày.

Chỗ lệch: `tach_la` DÙNG điều kiện `len(khoa) <= bac_toi_da` — TỨC
LÀ `bac_toi_da` LÀ số khoá TỐI ĐA CHO PHÉP mà node VẪN còn HỢP lệ,
KHÔNG phải "ngưỡng phải TÁCH". Chỉ khi SỐ khoá VƯỢT quá con SỐ đó
(`bac_toi_da + 1` trở LÊN) mới thật SỰ kích hoạt tách.
::
:::

:::opt
Máy báo lỗi — vì `bac_toi_da=3` LÀ một giá trị BIÊN (ranh giới)
KHÔNG hợp lệ để truyền VÀO
::why
Gần đúng ở việc bạn LO ngại các giá trị BIÊN thường gây LỖI trong
lập trình — một trực GIÁC thận trọng hợp LÝ.

Chỗ lệch: `bac_toi_da=3` HOÀN toàn hợp lệ — nó chỉ LÀ một con SỐ
so sánh BÌNH thường TRONG `if len(khoa) <= bac_toi_da`, KHÔNG có
`raise` nào ĐƯỢC viết cho trường hợp NÀY.
::
:::
::::

::::code{#viet_tach_la}
Hoàn thiện `tach_la(la, bac_toi_da)` — TÍNH điểm giữa, tạo node
MỚI chứa nửa SAU.

```python title=starter
def tach_la(la, bac_toi_da):
    if len(la['khoa']) <= bac_toi_da:
        return None
    ___
    la['khoa'] = la['khoa'][:giua]
    la['gia_tri'] = la['gia_tri'][:giua]
    khoa_tach = la_moi['khoa'][0]
    return (khoa_tach, la_moi)


la = {'loai': 'la', 'khoa': [1, 2, 3, 4], 'gia_tri': ['a', 'b', 'c', 'd'], 'la_tiep': None}
kq = tach_la(la, 3)
print(la['khoa'], kq[0], kq[1]['khoa'])
```

```python title=solution
def tach_la(la, bac_toi_da):
    if len(la['khoa']) <= bac_toi_da:
        return None
    giua = len(la['khoa']) // 2
    la_moi = {'loai': 'la', 'khoa': la['khoa'][giua:], 'gia_tri': la['gia_tri'][giua:], 'la_tiep': la['la_tiep']}
    la['khoa'] = la['khoa'][:giua]
    la['gia_tri'] = la['gia_tri'][:giua]
    khoa_tach = la_moi['khoa'][0]
    return (khoa_tach, la_moi)


la = {'loai': 'la', 'khoa': [1, 2, 3, 4], 'gia_tri': ['a', 'b', 'c', 'd'], 'la_tiep': None}
kq = tach_la(la, 3)
print(la['khoa'], kq[0], kq[1]['khoa'])
```

```python title=test
la = {'loai': 'la', 'khoa': [1, 2, 3, 4], 'gia_tri': ['a', 'b', 'c', 'd'], 'la_tiep': None}
kq = tach_la(la, 3)
assert la['khoa'] == [1, 2], "nua dau o lai"
assert la['gia_tri'] == ['a', 'b'], "gia tri di theo khoa"
assert kq[0] == 3, "khoa tach la khoa dau cua nua sau"
assert kq[1]['khoa'] == [3, 4], "node moi chua nua sau"
assert kq[1]['gia_tri'] == ['c', 'd'], "gia tri node moi dung"
assert kq[1]['loai'] == 'la', "node moi van la la"

la_du = {'loai': 'la', 'khoa': [1, 2, 3], 'gia_tri': ['a', 'b', 'c'], 'la_tiep': None}
assert tach_la(la_du, 3) is None, "du dung bac toi da -- chua can tach"

la5 = {'loai': 'la', 'khoa': [1, 2, 3, 4, 5], 'gia_tri': ['a', 'b', 'c', 'd', 'e'], 'la_tiep': 99}
kq5 = tach_la(la5, 4)
assert kq5[1]['la_tiep'] == 99, "node moi ke thua con tro la_tiep cu"
```

:::hints
- kind: attention
  body: "Tinh giua = len(la['khoa']) // 2, roi tao la_moi tu nua sau."
- kind: strategy
  body: "giua = len(la['khoa']) // 2; la_moi = {'loai': 'la', 'khoa': la['khoa'][giua:], 'gia_tri': la['gia_tri'][giua:], 'la_tiep': la['la_tiep']}"
- kind: one-line
  body: "giua = len(la['khoa']) // 2; la_moi = {'loai': 'la', 'khoa': la['khoa'][giua:], 'gia_tri': la['gia_tri'][giua:], 'la_tiep': la['la_tiep']}"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tinh giua roi tao la_moi tu nua sau cua khoa/gia_tri
  requireAst:
  - kind: uses-name, target: giua, min: 4
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[1, 2\] 3 \[3, 4\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lá TÁCH thành hai — NHƯNG hai lá RỜI rạc chưa LÀ một cây. Khoá TÁCH
(`3`) cần được LƯU Ở đâu đó để BIẾT "khoá NÀO thuộc nhánh NÀO"?
::::

::::reflect{#nghi-lai}
Lá đầy TÁCH đôi — nửa đầu Ở LẠI, nửa sau THÀNH node mới, `khoa_tach`
LÀ chìa khoá phân BIỆT hai nhánh. NHƯNG hai lá rời RẠC (KHÔNG liên
kết) chưa LÀ một cây — `khoa_tach` cần được LƯU Ở một node CHA
MỚI, để BIẾT khoá nhỏ hơn `3` Ở lá NÀO, khoá LỚN hơn hoặc bằng `3`
Ở lá NÀO. Node cha ĐÓ trông NHƯ thế nào?
::::

::::checkpoint{mastery=0.8}
::::
