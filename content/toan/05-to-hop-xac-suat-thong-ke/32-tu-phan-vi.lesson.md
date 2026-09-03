---
id: toan.to-hop-xac-suat-thong-ke.tu-phan-vi
title: Tứ phân vị
summary: "Tứ phân vị Q1, Q2, Q3 — chia dữ liệu ĐÃ SẮP thành BỐN phần đều nhau về SỐ LƯỢNG điểm (Q2 chính LÀ trung vị, bài 28); Q1 là trung vị của NỬA dưới, Q3 là trung vị của NỬA trên."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 32
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.quartile]
requires: [math.sample-variance]
concepts: [math.tu-phan-vi]
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
`s` đo "bấp bênh" bằng MỘT con số. Có cách nào mô tả HÌNH DÁNG chi
tiết hơn — không chỉ trung tâm, mà cả "nửa dưới" và "nửa trên" ra
sao?
::::

::::explain{#tu-phan-vi-la-gi}
Có. **Tứ phân vị `Q1`, `Q2`, `Q3`** — chia dữ liệu ĐÃ SẮP thành BỐN
phần đều nhau về SỐ LƯỢNG điểm (`Q2` CHÍNH LÀ trung vị, bài 28).
`Q1` LÀ trung vị của NỬA DƯỚI, `Q3` LÀ trung vị của NỬA TRÊN:

```python title=readonly
def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]

def tu_phan_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    nua_duoi = s[:n // 2]
    nua_tren = s[n // 2:]
    return trung_vi(nua_duoi), trung_vi(nua_tren)


mua_8 = [36, 37, 39, 40, 41, 43, 45, 47]

print(tu_phan_vi(mua_8))
```

```text title=readonly
(38.0, 44.0)
```

Tám mùa (đã sắp): `36,37,39,40,41,43,45,47`. NỬA dưới (bốn số đầu)
`36,37,39,40` — trung vị `(37+39)/2=38`. NỬA trên `41,43,45,47` —
trung vị `(43+45)/2=44`.
::::

::::example{#khoang-tu-phan-vi}
Khoảng cách `Q3 − Q1` gọi LÀ **khoảng tứ phân vị (IQR)** — chứa ĐÚNG
"nửa GIỮA" dữ liệu, bỏ qua phần TƯ thấp nhất VÀ phần tư cao nhất:

```python title=readonly
def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]

def tu_phan_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    nua_duoi = s[:n // 2]
    nua_tren = s[n // 2:]
    return trung_vi(nua_duoi), trung_vi(nua_tren)


mua_8 = [36, 37, 39, 40, 41, 43, 45, 47]
q1, q3 = tu_phan_vi(mua_8)

print(q3 - q1)
```

```text title=readonly
6.0
```

`IQR = 44 − 38 = 6` kg — MỘT mùa TỤT xuống `10` kg (rất xa so với
khoảng `38` tới `44` bình thường) CÓ NẰM trong khoảng đó không? Bài
sau trả lời chính xác.
::::

::::predict{#doan-tu-phan-vi-doi-xung commitOnce}
Byte thử tám giá trị ĐỀU nhau (`20` kg cả tám — hoàn toàn ĐỒNG ĐỀU):

```python
def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]

def tu_phan_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    nua_duoi = s[:n // 2]
    nua_tren = s[n // 2:]
    return trung_vi(nua_duoi), trung_vi(nua_tren)

mua_deu = [20, 20, 20, 20, 20, 20, 20, 20]
q1, q3 = tu_phan_vi(mua_deu)
print(q3 - q1)
```

Dòng cuối in ra gì?

:::opt{correct}
`0.0`
:::

:::opt
Máy báo lỗi khi chạy — TÁM giá trị GIỐNG hệt nhau khiến `nua_duoi`
VÀ `nua_tren` LÀ hai danh sách TRÙNG nhau HOÀN TOÀN, và `tu_phan_vi`
từ chối chia đôi một dãy KHÔNG có sự khác biệt
::why
Gần đúng ở việc bạn để ý `nua_duoi` VÀ `nua_tren` sẽ chứa CÙNG giá
trị (`20`) — một quan sát đúng VỀ NỘI DUNG hai nửa.

Chỗ lệch: `s[:n//2]` VÀ `s[n//2:]` LÀ hai PHÉP CẮT (slice, T1.4)
bình thường — chúng cắt theo VỊ TRÍ, không quan tâm nội dung hai
phần CÓ giống nhau hay không. Cả hai nửa ĐỀU LÀ `[20,20,20,20]`,
trung vị MỖI nửa ĐỀU LÀ `20`, `Q3−Q1=20−20=0.0`. Biên dịch sạch,
chạy sạch.
::
:::

:::opt
`20.0` — vì `Q1` VÀ `Q3` cùng "kế thừa" giá trị chung của dữ liệu
(`20`), và hiệu của HAI thứ giống nhau trong thống kê LÀ CHÍNH giá
trị đó, không phải `0`
::why
Gần đúng ở việc bạn nhận ra `Q1` VÀ `Q3` sẽ CÙNG LÀ `20` — quan sát
đó ĐÚNG.

Chỗ lệch: "hiệu của hai thứ GIỐNG NHAU" trong TOÁN HỌC (VÀ trong
Python) LUÔN LÀ `0`, không có quy tắc THỐNG KÊ đặc biệt nào khác đi.
`20 − 20 = 0.0`, KHÔNG PHẢI `20.0`.
::
:::
::::

::::code{#viet_tu_phan_vi}
Viết `tu_phan_vi(du_lieu)` — trả về CẶP `(Q1, Q3)`, LÀ trung vị của
NỬA dưới VÀ nửa trên (sau khi sắp).

```python title=starter
def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]

def tu_phan_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    nua_duoi = ___
    nua_tren = ___
    return trung_vi(nua_duoi), trung_vi(nua_tren)


mua_8 = [36, 37, 39, 40, 41, 43, 45, 47]

print(tu_phan_vi(mua_8))
```

```python title=solution
def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]

def tu_phan_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    nua_duoi = s[:n // 2]
    nua_tren = s[n // 2:]
    return trung_vi(nua_duoi), trung_vi(nua_tren)


mua_8 = [36, 37, 39, 40, 41, 43, 45, 47]

print(tu_phan_vi(mua_8))
```

```python title=test
assert tu_phan_vi([20] * 8) == (20, 20), "deu nhau -- Q1 va Q3 trung nhau"
assert tu_phan_vi([36, 37, 39, 40, 41, 43, 45, 47]) == (38.0, 44.0), "phai khop vi du chinh"
assert tu_phan_vi([1, 2, 3, 4]) == (1.5, 3.5), "bon phan tu -- nua duoi [1,2] (trung vi 1.5), nua tren [3,4] (trung vi 3.5)"
```

:::hints
- kind: attention
  body: "Dung slice: nua_duoi la s[:n//2], nua_tren la s[n//2:]."
- kind: strategy
  body: "nua_duoi = s[:n // 2]  --  nua_tren = s[n // 2:]"
- kind: one-line
  body: "s[:n // 2]  va  s[n // 2:]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai cat s thanh hai nua bang slice, chia tai vi tri n//2
  requireAst:
  - kind: uses-name, target: s, min: 4
  - kind: uses-operator, target: '//', min: 3
  - kind: uses-name, target: n, min: 4
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(38\.0, 44\.0\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`IQR=6`. Một mùa tụt xuống `10` kg — có "bất thường" đủ để gọi là
NGOẠI LỆ không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Với tám mùa: `Q1=38`, `Q3=44`. Khoảng cách `Q3−Q1=6` gọi LÀ IQR. Một
mùa TỤT xuống `10` kg (rất xa so với khoảng `38-44` bình thường) có
nằm trong khoảng đó không — và nếu KHÔNG, có QUY TẮC cụ thể nào gọi
nó LÀ "ngoại lệ", thay vì chỉ NHÌN bằng mắt?
::::

::::checkpoint{mastery=0.8}
::::
