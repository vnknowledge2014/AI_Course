---
id: toan.dstt-giai-tich-cho-ai.cuc-tri-dao-ham-bang-0
title: "Cực trị: đạo hàm bằng 0"
summary: "Tại cực ĐẠI/cực tiểu, f'(x)=0 — độ dốc BẰNG không TẠI đỉnh/đáy (KHÔNG tăng cũng KHÔNG giảm); tìm GIÁ trị x tối ưu bằng cách GIẢI f'(x)=0."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.critical-point]
requires: [math.chain-rule]
concepts: [math.cuc-tri]
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
Hàm `f(x) = (x²)²` CÓ đúng MỘT điểm mà đạo hàm BẰNG `0`. Điểm ĐÓ có
Ý nghĩa GÌ?
::::

::::explain{#cuc-tri-dao-ham-bang-0}
NÓ LÀ điểm cực TRỊ (lớn NHẤT hoặc nhỏ NHẤT). **Tại cực ĐẠI/cực
tiểu, `f'(x)=0`** — độ dốc BẰNG không TẠI đỉnh/đáy (KHÔNG tăng cũng
KHÔNG giảm ngay ĐÓ); tìm giá TRỊ `x` tối ưu bằng cách GIẢI `f'(x)=0`:

```python title=readonly
def f_dao_ham(x):
    return 2 * (x - 3)

def tim_cuc_tri(f_dao_ham, cac_diem):
    for x in cac_diem:
        if f_dao_ham(x) == 0:
            return x
    return None


print(tim_cuc_tri(f_dao_ham, [1, 2, 3, 4, 5]))
```

```text title=readonly
3
```

`f(x)=(x−3)²+5` CÓ đạo hàm `f'(x)=2(x−3)` — GIẢI `f'(x)=0` cho
`x=3`. Thử TỪNG điểm TRONG danh sách, `x=3` LÀ điểm DUY nhất khiến
`f'(x)=0`.
::::

::::example{#xac-nhan-la-cuc-tieu}
XÁC nhận `x=3` LÀ cực TIỂU — SO sánh VỚI hai điểm LÂN cận:

```python title=readonly
def f(x):
    return (x - 3) ** 2 + 5


print(f(3), f(2), f(4))
```

```text title=readonly
5 6 6
```

`f(3)=5` NHỎ hơn CẢ `f(2)=6` LẪN `f(4)=6` — hai điểm KỀ (một BÊN
mỗi PHÍA) đều CAO hơn `x=3`. `x=3` chính LÀ ĐÁY (cực TIỂU) của
parabol.
::::

::::predict{#doan-diem-khong-co-trong-danh-sach commitOnce}
Byte tìm cực trị TRÊN một danh sách KHÔNG chứa điểm `x=3`:

```python
def f_dao_ham(x):
    return 2 * (x - 3)

def tim_cuc_tri(f_dao_ham, cac_diem):
    for x in cac_diem:
        if f_dao_ham(x) == 0:
            return x
    return None

print(tim_cuc_tri(f_dao_ham, [1, 2, 4, 5]))
```

Dòng cuối in ra gì?

:::opt{correct}
`None`
:::

:::opt
`2` — vì `x=2` LÀ điểm GẦN cực trị THẬT NHẤT TRONG danh sách, hàm
tự ĐỘNG chọn điểm gần NHẤT khi không tìm THẤY chính XÁC
::why
Gần đúng ở việc bạn nghĩ TỚI một CƠ chế "tìm gần ĐÚNG nhất" — một
cách xử LÝ hợp lý cho nhiều bài TOÁN tìm kiếm.

Chỗ lệch: `tim_cuc_tri` CHỈ kiểm `f_dao_ham(x) == 0` CHÍNH XÁC —
KHÔNG có CƠ chế "gần đúng" nào. `f_dao_ham(2) = 2×(2−3) = -2 ≠ 0`,
KHÔNG khớp — vòng lặp TIẾP tục thử HẾT danh sách, KHÔNG tìm thấy
gì, trả VỀ `None`.
::
:::

:::opt
Máy báo lỗi khi chạy — danh sách `[1, 2, 4, 5]` "CỐ Ý" thiếu `3`,
Python phát hiện SỰ thiếu HỤT có chủ đích VÀ báo lỗi
::why
Gần đúng ở việc bạn để ý ĐÚNG `3` bị THIẾU KHỎI danh sách — một
quan sát VỀ dữ liệu.

Chỗ lệch: Python KHÔNG "phát hiện Ý đồ" gì cả — danh sách THIẾU một
GIÁ trị CHỈ đơn giản LÀ danh sách NGẮN hơn, vòng lặp chạy HẾT rồi
trả VỀ `None` (dòng `return None` NGOÀI vòng LẶP), KHÔNG lỗi gì.
::
:::
::::

::::code{#viet_tim_cuc_tri}
Viết `tim_cuc_tri(f_dao_ham, cac_diem)` — tìm điểm TRONG `cac_diem`
mà đạo hàm BẰNG `0`.

```python title=starter
def f_dao_ham(x):
    return 2 * (x - 3)

def tim_cuc_tri(f_dao_ham, cac_diem):
    for x in cac_diem:
        if ___:
            return x
    return None


print(tim_cuc_tri(f_dao_ham, [1, 2, 3, 4, 5]))
```

```python title=solution
def f_dao_ham(x):
    return 2 * (x - 3)

def tim_cuc_tri(f_dao_ham, cac_diem):
    for x in cac_diem:
        if f_dao_ham(x) == 0:
            return x
    return None


print(tim_cuc_tri(f_dao_ham, [1, 2, 3, 4, 5]))
```

```python title=test
assert tim_cuc_tri(lambda x: 2 * (x - 3), []) is None, "danh sach rong"
assert tim_cuc_tri(lambda x: 2 * (x - 3), [1, 2, 3, 4, 5]) == 3, "co diem cuc tri"
assert tim_cuc_tri(lambda x: 2 * (x - 3), [1, 2, 4, 5]) is None, "khong co diem cuc tri"
assert tim_cuc_tri(lambda x: 2 * x, [-1, 0, 1]) == 0, "cuc tri tai 0"
```

:::hints
- kind: attention
  body: "Kiem f_dao_ham(x) co bang 0 khong."
- kind: strategy
  body: "f_dao_ham(x) == 0"
- kind: one-line
  body: "if f_dao_ham(x) == 0:"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai kiem f_dao_ham(x) == 0
  requireAst:
  - kind: uses-call, target: f_dao_ham, min: 1
  - kind: uses-operator, target: '==', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^3\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cực trị — nơi đạo hàm bằng `0`. Hàm CHI PHÍ tưới nước CÓ NHIỀU biến
— "độ dốc" theo TỪNG biến RIÊNG tính thế NÀO?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hàm CHI PHÍ tưới nước CÓ NHIỀU biến (không chỉ MỘT `x`) — "độ dốc"
theo TỪNG biến RIÊNG tính thế NÀO?
::::

::::checkpoint{mastery=0.8}
::::
