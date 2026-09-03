---
id: co-so-du-lieu.cay-cua-nguoi-thu-thu.chen-vao-la-con-cho
title: Chèn vào lá còn chỗ
summary: "chen_vao_la tìm ĐÚNG vị trí giữ khoa sắp xếp, rồi list.insert() cả khoa lẫn gia_tri tại vị trí đó — giữ nguyên thứ tự sau khi thêm. Chèn khoá đã tồn tại tạo ra HAI bản sao của khoá đó (chưa xử lý cập nhật — chỉ dành cho lá CÒN chỗ trống)."
locale: vi
track: co-so-du-lieu
module: cay-cua-nguoi-thu-thu
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [db.insert-in-leaf]
requires: [db.search-in-leaf]
concepts: [db.insert-in-leaf]
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
Node LÁ hiện có `3` khoá: `[3, 7, 9]`. Byte muốn THÊM khoá `5` — VỊ
trí ĐÚNG để chèn LÀ Ở giữa `3` VÀ `7`, giữ `khoa` VẪN sắp XẾP.
::::

::::explain{#tim-vi-tri-chen}
`chen_vao_la` DÒ qua `khoa`, tìm CHỈ số ĐẦU tiên LỚN hơn `khoa_moi`
— ĐÓ chính LÀ vị trí CHÈN:

```python title=readonly
def chen_vao_la(la, khoa_moi, gia_tri_moi):
    khoa = la['khoa']
    vi_tri = len(khoa)
    for i in range(len(khoa)):
        if khoa[i] > khoa_moi:
            vi_tri = i
            break
    khoa.insert(vi_tri, khoa_moi)
    la['gia_tri'].insert(vi_tri, gia_tri_moi)
    return la


la = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
chen_vao_la(la, 5, 'nam')
print(la['khoa'], la['gia_tri'])
```

```text title=readonly
[3, 5, 7, 9] ['ba', 'nam', 'bay', 'chin']
```

`vi_tri` BẮT đầu LÀ `len(khoa)` (mặc ĐỊNH: chèn VÀO cuối, NẾU khoá
mới LỚN hơn tất CẢ) — vòng lặp TÌM chỉ số ĐẦU tiên có `khoa[i] > 5`
(chính LÀ `7`, Ở chỉ số `1`), DỪNG ngay. `insert(1, ...)` CHÈN cả
khoá LẪN giá trị VÀO đúng vị TRÍ đó, ĐẨY phần CÒN lại lùi XUỐNG một
bậc.
::::

::::example{#chen-vao-dau-va-cuoi}
Chèn khoá NHỎ hơn TẤT cả (`1`) VÀ khoá LỚN hơn tất CẢ (`10`) — CẢ
hai đều RƠI đúng chỗ:

```python title=readonly
la_dau = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
chen_vao_la(la_dau, 1, 'mot')
print(la_dau['khoa'])

la_cuoi = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
chen_vao_la(la_cuoi, 10, 'muoi')
print(la_cuoi['khoa'])
```

```text title=readonly
[1, 3, 7, 9]
[3, 7, 9, 10]
```

`1` KHÔNG khớp `khoa[i] > 1` VỚI bất KỲ `i` NÀO Ở vòng lặp ĐẦU tiên
(`khoa[0]=3 > 1` LÀ ĐÚNG NGAY) — CHÈN Ở chỉ số `0`. `10` KHÔNG khớp
`khoa[i] > 10` LẦN nào (KHÔNG có phần TỬ nào LỚN hơn `10`) — `vi_tri`
GIỮ nguyên giá trị mặc ĐỊNH `len(khoa) = 3`, chèn VÀO CUỐI.
::::

::::predict{#doan-chen-khoa-trung commitOnce}
Byte CHÈN khoá `7` (ĐÃ tồn tại SẴN trong `khoa=[3, 7, 9]`) VỚI giá
trị MỚI `'BAY-2'`:

```python
la = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
chen_vao_la(la, 7, 'BAY-2')
print(la['khoa'])
```

Dòng cuối in ra gì?

:::opt{correct}
`[3, 7, 7, 9]`
:::

:::opt
`[3, 7, 9]` — vì `chen_vao_la` TỰ nhận ra `7` ĐÃ tồn tại, nên CẬP
nhật giá trị TẠI chỗ thay VÌ chèn thêm
::why
Gần đúng ở việc bạn nghĩ TỚI hành vi "CẬP nhật nếu trùng" hợp lý
CHO một `dict`/index THẬT — MỘT kỳ vọng hợp LÝ với các cấu trúc dữ
liệu KHÁC.

Chỗ lệch: `chen_vao_la` KHÔNG hề kiểm tra `khoa_moi` CÓ trùng khoá
NÀO sẵn CÓ không — nó chỉ TÌM vị trí "khoá ĐẦU tiên LỚN hơn", RỒI
`insert()`. VỚI `khoa_moi=7`, `khoa[1]=7` KHÔNG lớn hơn `7` (bằng
NHAU, không LỚN hơn), NÊN vòng lặp tiếp TỤC tới `khoa[2]=9 > 7` —
chèn Ở chỉ số `2`, TẠO ra HAI bản `7` liền NHAU.
::
:::

:::opt
Máy báo lỗi — vì khoá `7` KHÔNG được phép trùng LẶP trong một cây
B+Tree
::why
Gần đúng ở việc bạn nghĩ TỚI một RÀNG buộc "khoá DUY nhất" hợp lý
CHO một cây tìm KIẾM thật.

Chỗ lệch: `chen_vao_la` (Ở PHIÊN bản này) KHÔNG hề kiểm tra tính
DUY nhất — nó CHỈ đơn thuần chèn ĐÚNG vị trí sắp xếp, KHÔNG quan
tâm khoá CÓ trùng hay không. Xử lý "cập nhật KHI trùng" LÀ một
tính năng RIÊNG, chưa được xây Ở bài NÀY.
::
:::
::::

::::code{#viet_chen_vao_la}
Hoàn thiện `chen_vao_la(la, khoa_moi, gia_tri_moi)` — chèn khoá VÀ
giá trị VÀO đúng vị trí đã DÒ được.

```python title=starter
def chen_vao_la(la, khoa_moi, gia_tri_moi):
    khoa = la['khoa']
    vi_tri = len(khoa)
    for i in range(len(khoa)):
        if khoa[i] > khoa_moi:
            vi_tri = i
            break
    ___
    return la


la = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
chen_vao_la(la, 5, 'nam')
print(la['khoa'], la['gia_tri'])
```

```python title=solution
def chen_vao_la(la, khoa_moi, gia_tri_moi):
    khoa = la['khoa']
    vi_tri = len(khoa)
    for i in range(len(khoa)):
        if khoa[i] > khoa_moi:
            vi_tri = i
            break
    khoa.insert(vi_tri, khoa_moi)
    la['gia_tri'].insert(vi_tri, gia_tri_moi)
    return la


la = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
chen_vao_la(la, 5, 'nam')
print(la['khoa'], la['gia_tri'])
```

```python title=test
la = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
chen_vao_la(la, 5, 'nam')
assert la['khoa'] == [3, 5, 7, 9], "chen vao giua dung vi tri"
assert la['gia_tri'] == ['ba', 'nam', 'bay', 'chin'], "gia tri di theo dung khoa"

la_dau = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
chen_vao_la(la_dau, 1, 'mot')
assert la_dau['khoa'] == [1, 3, 7, 9], "chen vao dau"

la_cuoi = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
chen_vao_la(la_cuoi, 10, 'muoi')
assert la_cuoi['khoa'] == [3, 7, 9, 10], "chen vao cuoi"

la_rong = {'loai': 'la', 'khoa': [], 'gia_tri': [], 'la_tiep': None}
chen_vao_la(la_rong, 5, 'nam')
assert la_rong['khoa'] == [5], "chen vao la rong"
```

:::hints
- kind: attention
  body: "Goi khoa.insert(vi_tri, khoa_moi) va la['gia_tri'].insert(vi_tri, gia_tri_moi)."
- kind: strategy
  body: "khoa.insert(vi_tri, khoa_moi); la['gia_tri'].insert(vi_tri, gia_tri_moi)"
- kind: one-line
  body: "khoa.insert(vi_tri, khoa_moi); la['gia_tri'].insert(vi_tri, gia_tri_moi)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai chen khoa_moi va gia_tri_moi vao dung vi_tri trong ca hai danh sach
  requireAst:
  - kind: uses-name, target: vi_tri, min: 2
  - kind: uses-name, target: gia_tri_moi, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[3, 5, 7, 9\] \['ba', 'nam', 'bay', 'chin'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chèn ĐÚNG vị trí — node LÁ vẫn sắp XẾP. NHƯNG một lá KHÔNG thể phình
TO mãi — khi ĐẦY rồi, chèn THÊM một khoá NỮA thì SAO?
::::

::::reflect{#nghi-lai}
`chen_vao_la` tìm ĐÚNG vị trí sắp XẾP rồi `insert()` cả khoá LẪN
giá trị — giữ node LUÔN có thứ TỰ. Khoá TRÙNG tạo ra bản SAO thay
vì cập nhật (một GIỚI hạn của phiên bản NÀY, KHÔNG phải lỗi). Node
LÁ không thể phình TO vô hạn — MỘT sector chỉ chứa được BAO nhiêu
byte LÀ có giới hạn. Khi lá ĐÃ đầy, chèn THÊM một khoá nữa PHẢI làm
gì?
::::

::::checkpoint{mastery=0.8}
::::
