---
id: toan.to-hop-xac-suat-thong-ke.cay-xac-suat
title: Cây xác suất
summary: "Sơ đồ cây — mỗi tầng một phép thử, mỗi nhánh một kết quả kèm xác suất riêng. Xác suất một ĐƯỜNG ĐI là TÍCH các nhánh trên đường (quy tắc nhân); cộng các đường cùng đích LÀ hợp (quy tắc cộng) — công cụ trực quan ghép lại bài 14-20, không khái niệm số học mới."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: []
practices: [math.rule-of-sum, math.rule-of-product, math.bayes-theorem]
requires: [math.bayes-theorem]
concepts: [math.cay-xac-suat]
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
Chín bài liền toàn công thức. Có cách nào VẼ cả bài toán "giống mới
hay cũ, nảy hay không" thành MỘT hình, nhìn một cái thấy hết?
::::

::::explain{#cay-xac-suat-la-gi}
Có. **Sơ đồ cây** — mỗi TẦNG một phép thử, mỗi NHÁNH một kết quả kèm
xác suất RIÊNG (có điều kiện, bài 18). Tầng 1: giống mới hay cũ.
Tầng 2 (TỪ mỗi nhánh tầng 1): nảy hay không. Xác suất một ĐƯỜNG ĐI
(từ gốc TỚI lá) LÀ TÍCH các nhánh trên đường (quy tắc nhân, bài 17):

```python title=readonly
def xac_suat_duong_di(cac_nhanh):
    ket_qua = 1
    for p in cac_nhanh:
        ket_qua = ket_qua * p
    return ket_qua


# duong: giong moi (0.6) -> nay mam (0.2)
print(xac_suat_duong_di([0.6, 0.2]))
```

```text title=readonly
0.12
```

Đường "giống mới VÀ nảy mầm" đi qua HAI nhánh: `P(\text{giống
mới})=0.6` rồi `P(\text{nảy}|\text{giống mới})=0.2`. Nhân lại
`0.6×0.2=0.12` — ĐÚNG con số bài 20 đã dùng làm một số hạng của
`P(\text{nảy mầm})`.
::::

::::example{#cong-cac-duong-cung-dich}
HAI đường CÙNG đích ("nảy mầm", bất kể giống) — CỘNG lại (quy tắc
cộng, bài 1: hai đường RỜI NHAU, một hạt không thể VỪA đi đường
"mới" VỪA đi đường "cũ"):

```python title=readonly
def xac_suat_tong_hop(cac_duong):
    tong = 0
    for duong in cac_duong:
        xs_duong = 1
        for p in duong:
            xs_duong = xs_duong * p
        tong = tong + xs_duong
    return tong


duong_moi_nay = [0.6, 0.2]
duong_cu_nay = [0.4, 0.1]

print(xac_suat_tong_hop([duong_moi_nay, duong_cu_nay]))
```

```text title=readonly
0.16
```

`xac_suat_tong_hop` NHÂN dọc MỖI đường (quy tắc nhân), rồi CỘNG
ngang GIỮA các đường (quy tắc cộng) — hai vòng lặp lồng nhau, ĐÚNG
`P(\text{nảy mầm})=0.16` mà bài 20 đã tính BẰNG công thức, giờ nhìn
qua CÂY.
::::

::::predict{#doan-mot-nhanh-bang-khong commitOnce}
Byte thử: giống cũ KHÔNG BAO GIỜ nảy mầm (`P(\text{nảy}|\text{giống
cũ})=0.0`):

```python
def xac_suat_tong_hop(cac_duong):
    tong = 0
    for duong in cac_duong:
        xs_duong = 1
        for p in duong:
            xs_duong = xs_duong * p
        tong = tong + xs_duong
    return tong

duong_moi_nay = [0.6, 0.2]
duong_cu_nay = [0.4, 0.0]

print(xac_suat_tong_hop([duong_moi_nay, duong_cu_nay]))
```

Dòng cuối in ra gì?

:::opt{correct}
`0.12`
:::

:::opt
`0.0` — vì MỘT trong hai đường có nhánh `0.0`, và một đường "hỏng"
kéo theo TOÀN BỘ phép cộng về `0`
::why
Gần đúng ở việc bạn nhớ ĐÚNG một nhánh `0.0` khiến CẢ đường đó về
`0` (bài 17: nhân với `0` luôn ra `0`) — quan sát ĐÓ chính xác cho
riêng đường "cũ-nảy".

Chỗ lệch: `xac_suat_tong_hop` CỘNG các đường LẠI, không NHÂN chúng —
đường "cũ-nảy" VỀ `0` (`0.4×0.0=0.0`) NHƯNG đường "mới-nảy" VẪN giữ
nguyên `0.12` (không nhánh nào của NÓ bị `0`). Cộng `0.12 + 0.0 =
0.12` — một đường hỏng KHÔNG kéo đường KHÁC theo, vì phép TOÁN nối
CÁC đường LÀ cộng, không phải nhân.
::
:::

:::opt
Máy báo lỗi khi chạy — một nhánh mang xác suất `0.0` khiến cây "cụt"
Ở đó, và hàm từ chối tính TIẾP đường đi qua nhánh cụt
::why
Gần đúng ở việc bạn hình dung nhánh `0.0` như một "ngõ cụt" không đi
tiếp được — một hình ảnh trực quan hợp lý.

Chỗ lệch: `0.0` chỉ LÀ một con số bình thường — nhân với nó KHÔNG
"cụt" chương trình, chỉ khiến kết quả VỀ `0`. Vòng lặp vẫn chạy hết,
hàm vẫn `return` bình thường. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_xac_suat_tong_hop}
Viết `xac_suat_tong_hop(cac_duong)` — với `cac_duong` LÀ một danh
sách các đường (mỗi đường là một danh sách xác suất trên từng
nhánh), tính TỔNG xác suất mọi đường (nhân dọc, cộng ngang).

```python title=starter
def xac_suat_tong_hop(cac_duong):
    tong = 0
    for duong in cac_duong:
        xs_duong = 1
        for p in duong:
            xs_duong = ___
        tong = tong + xs_duong
    return tong


duong_moi_nay = [0.6, 0.2]
duong_cu_nay = [0.4, 0.1]

print(xac_suat_tong_hop([duong_moi_nay, duong_cu_nay]))
```

```python title=solution
def xac_suat_tong_hop(cac_duong):
    tong = 0
    for duong in cac_duong:
        xs_duong = 1
        for p in duong:
            xs_duong = xs_duong * p
        tong = tong + xs_duong
    return tong


duong_moi_nay = [0.6, 0.2]
duong_cu_nay = [0.4, 0.1]

print(xac_suat_tong_hop([duong_moi_nay, duong_cu_nay]))
```

```python title=test
assert xac_suat_tong_hop([]) == 0, "khong duong nao -- tong 0"
assert xac_suat_tong_hop([[0.5]]) == 0.5, "mot duong mot nhanh -- ket qua bang chinh xac suat do"
assert xac_suat_tong_hop([[0.6, 0.2], [0.4, 0.0]]) == 0.12, "mot duong ve 0, duong kia van giu nguyen"
assert xac_suat_tong_hop([[1.0], [0.0]]) == 1.0, "duong chac chan cong duong khong the -- van chac chan"
```

:::hints
- kind: attention
  body: "Vong trong nhan don xs_duong theo tung nhanh cua MOT duong -- giu nguyen vong ngoai cong don tong."
- kind: strategy
  body: "xs_duong * p"
- kind: one-line
  body: "xs_duong = xs_duong * p"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai nhan don xs_duong voi p trong vong lap trong
  requireAst:
  - kind: uses-operator, target: '*', min: 1
  - kind: uses-name, target: xs_duong, min: 1
  - kind: uses-name, target: p, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^0\.16\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chín bài liền, đúng MỘT hạt hay MỘT lần thử. Gieo BỐN MƯƠI hạt, hỏi
TRUNG BÌNH bao nhiêu hạt nảy — câu hỏi mới cần công cụ gì?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chín bài liền (11-19, cộng bài 20 vừa Bayes) đều xoay quanh MỘT hạt
hay MỘT lần thử. Byte giờ gieo BỐN MƯƠI hạt, MỖI hạt độc lập nảy mầm
hay không — muốn hỏi "TRUNG BÌNH bao nhiêu hạt nảy mầm", không phải
"hạt NÀY nảy mầm hay không". Câu hỏi mới cần công cụ gì?
::::

::::checkpoint{mastery=0.8}
::::
