---
id: toan.dstt-giai-tich-cho-ai.cong-ma-tran
title: Cộng ma trận
summary: "A+B — cộng TỪNG phần tử Ở CÙNG vị trí hàng-cột; đúng lối cộng vector (bài 2), CHỈ thêm một CHIỀU."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.matrix-addition]
requires: [math.matrix]
concepts: [math.cong-ma-tran]
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
Cộng HAI ma trận (hai lần ĐO của cùng khu vườn) — cộng NHƯ thế nào,
đúng lối cộng vector (bài 2) không?
::::

::::explain{#cong-ma-tran}
ĐÚNG lối cộng vector, CHỈ thêm một CHIỀU. **`A+B`** — cộng TỪNG
phần tử Ở CÙNG vị trí hàng-cột: MỖI hàng CỦA `A` cộng VỚI hàng
TƯƠNG ứng của `B` (đúng cộng vector, bài 2), RỒI gộp tất cả hàng
KẾT quả LẠI:

```python title=readonly
def cong_ma_tran(A, B):
    return [tuple(a + b for a, b in zip(hang_a, hang_b)) for hang_a, hang_b in zip(A, B)]


A = [(1.0, 2.0), (3.0, 4.0)]
B = [(10.0, 20.0), (30.0, 40.0)]

print(cong_ma_tran(A, B))
```

```text title=readonly
[(11.0, 22.0), (33.0, 44.0)]
```

Hàng ĐẦU của `A` (`(1,2)`) cộng VỚI hàng ĐẦU của `B` (`(10,20)`) —
`(11,22)`. Hàng THỨ hai TƯƠNG tự. HAI ma trận `2×2` cộng RA một ma
trận `2×2` MỚI.
::::

::::example{#ma-tran-rong}
Ma trận RỖNG cộng ma trận RỖNG — RA ma trận rỗng:

```python title=readonly
def cong_ma_tran(A, B):
    return [tuple(a + b for a, b in zip(hang_a, hang_b)) for hang_a, hang_b in zip(A, B)]


print(cong_ma_tran([], []))
```

```text title=readonly
[]
```

ĐÚNG lối vector RỖNG (bài 2) — KHÔNG hàng nào để cộng THÌ kết quả
LÀ danh sách RỖNG.
::::

::::predict{#doan-khac-so-hang commitOnce}
Byte cộng HAI ma trận KHÁC số HÀNG — `A` CÓ hai hàng, `B` CHỈ có một
hàng:

```python
def cong_ma_tran(A, B):
    return [tuple(a + b for a, b in zip(hang_a, hang_b)) for hang_a, hang_b in zip(A, B)]

A = [(1.0, 2.0), (3.0, 4.0)]
B = [(10.0, 20.0)]
print(cong_ma_tran(A, B))
```

Dòng cuối in ra gì?

:::opt{correct}
`[(11.0, 22.0)]`
:::

:::opt
Máy báo lỗi khi chạy — `A` CÓ hai hàng, `B` chỉ có MỘT, `zip(A, B)`
sẽ TỪ chối ghép hai danh SÁCH có độ dài khác nhau
::why
Gần đúng ở việc bạn NGHĨ hai ma trận khác số hàng "KHÔNG hợp lệ" để
cộng — VỀ mặt TOÁN học, đây LÀ một quan sát ĐÚNG (cộng ma trận chỉ
định nghĩa khi cùng KÍCH thước).

Chỗ lệch: đúng LỐI bài 2 — `zip` KHÔNG báo lỗi khi hai dãy khác ĐỘ
dài, nó CHỈ ghép tới khi dãy NGẮN hơn hết RỒI dừng LẶNG lẽ. Hàng
THỨ hai của `A` (`(3.0, 4.0)`) bị BỎ hoàn TOÀN, không lỗi gì.
::
:::

:::opt
`[(11.0, 22.0), (3.0, 4.0)]` — vì hàng THỪA của `A` được GIỮ nguyên
Ở kết quả, KHÔNG cộng thêm GÌ (vì `B` không CÓ hàng tương ứng)
::why
Gần đúng ở việc bạn nghĩ Python "giữ LẠI" phần dư thay VÌ bỏ hẳn —
một cách xử LÝ hợp lý.

Chỗ lệch: `zip` KHÔNG "giữ lại" gì — nó DỪNG hoàn TOÀN khi dãy ngắn
hơn HẾT, hàng `(3.0, 4.0)` bị LOẠI khỏi kết quả HOÀN toàn, không
xuất hiện Ở BẤT KỲ đâu.
::
:::
::::

::::code{#viet_cong_ma_tran}
Viết `cong_ma_tran(A, B)` — cộng HAI ma trận, từng Ô tương ứng.

```python title=starter
def cong_ma_tran(A, B):
    return ___


A = [(1.0, 2.0), (3.0, 4.0)]
B = [(10.0, 20.0), (30.0, 40.0)]
print(cong_ma_tran(A, B))
```

```python title=solution
def cong_ma_tran(A, B):
    return [tuple(a + b for a, b in zip(hang_a, hang_b)) for hang_a, hang_b in zip(A, B)]


A = [(1.0, 2.0), (3.0, 4.0)]
B = [(10.0, 20.0), (30.0, 40.0)]
print(cong_ma_tran(A, B))
```

```python title=test
assert cong_ma_tran([], []) == [], "hai ma tran rong -- ket qua rong"
A = [(1.0, 2.0), (3.0, 4.0)]
B = [(10.0, 20.0), (30.0, 40.0)]
assert cong_ma_tran(A, B) == [(11.0, 22.0), (33.0, 44.0)], "cong tung o"
assert cong_ma_tran([(1.0,)], [(2.0,)]) == [(3.0,)], "ma tran 1x1"
assert cong_ma_tran(A, [(10.0, 20.0)]) == [(11.0, 22.0)], "khac so hang -- zip cat ngan"
```

:::hints
- kind: attention
  body: "Dung zip(A, B) de ghep tung cap hang, roi cong tung cap hang (dung zip lan nua ben trong)."
- kind: strategy
  body: "[tuple(a + b for a, b in zip(hang_a, hang_b)) for hang_a, hang_b in zip(A, B)]"
- kind: one-line
  body: "___ = [tuple(a + b for a, b in zip(hang_a, hang_b)) for hang_a, hang_b in zip(A, B)]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung zip(A, B) va zip tung cap hang de cong tung o
  requireAst:
  - kind: uses-call, target: tuple, min: 1
  - kind: uses-call, target: zip, min: 2
  - kind: uses-operator, target: '+', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[\(11\.0, 22\.0\), \(33\.0, 44\.0\)\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cộng ma trận — từng Ô một. Nhân CẢ ma trận VỚI một số `k` — phép
toán NÀY LÀ gì?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhân CẢ ma trận VỚI một số `k` (tăng đồng LOẠT mọi chỉ số đo LÊN `k`
lần) — phép toán NÀY LÀ gì?
::::

::::checkpoint{mastery=0.8}
::::
