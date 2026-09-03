---
id: co-so-du-lieu.cay-cua-nguoi-thu-thu.node-trong-la-gi
title: Node trong là gì
summary: "chon_nhanh so sánh khoá cần tìm với TỪNG khoá trong node trong (đã sắp xếp) — khoá cần tìm nhỏ hơn khoá NÀO trước tiên thì đi nhánh con TƯƠNG ứng; không nhỏ hơn khoá nào cả thì đi nhánh CUỐI. Một khoá bằng ĐÚNG khoá phân tách LUÔN đi nhánh phải (quy ước: điều kiện là < chứ không phải <=)."
locale: vi
track: co-so-du-lieu
module: cay-cua-nguoi-thu-thu
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [db.internal-node-format]
requires: [db.push-key-to-parent]
concepts: [db.internal-node-format]
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
Bài TRƯỚC, root ĐÃ có — MỘT khoá, hai con TRỎ. NHƯNG node trong THẬT
sự "CHỌN nhánh" ra SAO, dựa TRÊN khoá cần TÌM?
::::

::::explain{#chon-nhanh}
`chon_nhanh` SO sánh khoá cần TÌM với TỪNG khoá TRONG node (đã sắp
XẾP) — nhỏ hơn khoá NÀO trước TIÊN thì đi nhánh CON tương ứng, KHÔNG
nhỏ hơn khoá NÀO cả thì đi nhánh CUỐI:

```python title=readonly
def chon_nhanh(node_trong, khoa_can_tim):
    for i, k in enumerate(node_trong['khoa']):
        if khoa_can_tim < k:
            return node_trong['con'][i]
    return node_trong['con'][-1]


root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
print(chon_nhanh(root, 3))
print(chon_nhanh(root, 9))
```

```text title=readonly
1
2
```

`chon_nhanh(root, 3)`: `3 < 5` ĐÚNG NGAY Ở khoá đầu TIÊN (chỉ số
`0`) — trả VỀ `con[0]=1`. `chon_nhanh(root, 9)`: `9 < 5` SAI, vòng
lặp CHẠY hết KHÔNG khớp gì — RƠI về `con[-1]=2`.
::::

::::example{#node-nhieu-khoa}
Một node CÓ `2` khoá (`[10, 20]`) — `3` con TRỎ, ĐÚNG `len(khoa)+1`:

```python title=readonly
n2 = {'loai': 'trong', 'khoa': [10, 20], 'con': [100, 101, 102]}
print(chon_nhanh(n2, 5))
print(chon_nhanh(n2, 15))
print(chon_nhanh(n2, 25))
```

```text title=readonly
100
101
102
```

`5 < 10` KHỚP ngay — `con[0]=100`. `15`: `15 < 10` SAI, `15 < 20`
ĐÚNG (chỉ số `1`) — `con[1]=101`. `25`: KHÔNG khớp khoá NÀO — RƠI
về `con[-1]=102`.
::::

::::predict{#doan-khoa-bang-khoa-phan-tach commitOnce}
Byte tìm ĐÚNG khoá PHÂN tách của root (`khoa=[5]`) — `chon_nhanh(root,
5)`:

```python
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
print(chon_nhanh(root, 5))
```

Dòng cuối in ra gì?

:::opt{correct}
`2`
:::

:::opt
`1` — vì khoá cần TÌM (`5`) khớp ĐÚNG với khoá phân TÁCH (`5`), NÊN
đi nhánh BÊN trái
::why
Gần đúng ở việc bạn nghĩ "TRÙNG khoá" nghĩa LÀ "Ở phía TRƯỚC nó" —
một trực GIÁC hợp lý theo cách ĐỌC thông thường.

Chỗ lệch: điều kiện TRONG `chon_nhanh` LÀ SO sánh NGHIÊM ngặt
`khoa_can_tim < k`, KHÔNG phải `<=` — `5 < 5` LÀ `False`, vòng lặp
KHÔNG trả về SỚM, rơi VỀ `con[-1]` (nhánh PHẢI). Một khoá BẰNG đúng
khoá phân TÁCH LUÔN đi nhánh PHẢI, theo quy ƯỚC này.
::
:::

:::opt
Máy báo lỗi — vì khoá TRÙNG với khoá đã CÓ trong node LÀ không hợp
lệ
::why
Gần đúng ở việc bạn nghĩ TỚI một ràng buộc "khoá DUY nhất" hợp lý.

Chỗ lệch: `chon_nhanh` CHỈ đơn thuần LÀ một phép so SÁNH quét qua —
KHÔNG có `raise` nào cả, VÀ khoá TRÙNG hoàn TOÀN hợp lệ (đó chính LÀ
lý do CẦN một quy ước RÕ ràng cho trường hợp NÀY).
::
:::
::::

::::code{#viet_chon_nhanh}
Hoàn thiện `chon_nhanh(node_trong, khoa_can_tim)` — trả VỀ con trỏ
ĐÚNG theo quy ước `<`.

```python title=starter
def chon_nhanh(node_trong, khoa_can_tim):
    for i, k in enumerate(node_trong['khoa']):
        ___
    return node_trong['con'][-1]


root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
print(chon_nhanh(root, 3), chon_nhanh(root, 5), chon_nhanh(root, 9))
```

```python title=solution
def chon_nhanh(node_trong, khoa_can_tim):
    for i, k in enumerate(node_trong['khoa']):
        if khoa_can_tim < k:
            return node_trong['con'][i]
    return node_trong['con'][-1]


root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
print(chon_nhanh(root, 3), chon_nhanh(root, 5), chon_nhanh(root, 9))
```

```python title=test
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
assert chon_nhanh(root, 3) == 1, "nho hon khoa phan tach -- nhanh trai"
assert chon_nhanh(root, 5) == 2, "bang khoa phan tach -- nhanh phai"
assert chon_nhanh(root, 9) == 2, "lon hon khoa phan tach -- nhanh phai"
assert chon_nhanh(root, 0) == 1, "nho hon nua -- van nhanh trai"

n2 = {'loai': 'trong', 'khoa': [10, 20], 'con': [100, 101, 102]}
assert chon_nhanh(n2, 15) == 101, "node nhieu khoa -- giua"
assert chon_nhanh(n2, 25) == 102, "node nhieu khoa -- cuoi"
```

:::hints
- kind: attention
  body: "Neu khoa_can_tim < k, return node_trong['con'][i] -- mot dong."
- kind: strategy
  body: "if khoa_can_tim < k: return node_trong['con'][i]"
- kind: one-line
  body: "if khoa_can_tim < k: return node_trong['con'][i]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai return node_trong['con'][i] khi khoa_can_tim < k
  requireAst:
  - kind: uses-name, target: i, min: 1
  - kind: uses-name, target: k, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^1 2 2\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chọn ĐÚNG một nhánh — MỘT bước điều hướng. NHƯNG một cây THẬT cần
đi qua NHIỀU node trong (nếu có), TỚI tận LÁ — điều hướng TRỌN vẹn
diễn ra thế NÀO?
::::

::::reflect{#nghi-lai}
`chon_nhanh` chọn ĐÚNG một nhánh — SO sánh nghiêm NGẶT `<`, khoá
BẰNG khoá phân tách LUÔN đi nhánh PHẢI (một quy ƯỚC, không phải luật
tự NHIÊN). Đây mới LÀ MỘT bước điều hướng — TỪ root, Byte CHỈ vừa
CHỌN được nhánh nào TRONG số hai lá. Muốn tìm MỘT khoá THẬT sự, đi
TỪ root xuống TẬN lá diễn RA thế nào?
::::

::::checkpoint{mastery=0.8}
::::
