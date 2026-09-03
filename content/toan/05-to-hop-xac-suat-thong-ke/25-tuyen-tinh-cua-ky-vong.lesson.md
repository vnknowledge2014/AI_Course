---
id: toan.to-hop-xac-suat-thong-ke.tuyen-tinh-cua-ky-vong
title: Tuyến tính của kỳ vọng
summary: "E[X+Y] = E[X] + E[Y] — LUÔN đúng, kể cả khi X, Y KHÔNG độc lập (khác hẳn bài 16, nơi độc lập là ĐIỀU KIỆN bắt buộc). Tổng kỳ vọng hai luống = kỳ vọng của tổng, cộng thẳng không cần biết X, Y có liên quan nhau hay không."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.linearity-of-expectation]
requires: [math.expectation]
concepts: [math.tuyen-tinh-ky-vong]
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
Bốn luống, MỖI luống một `Xᵢ` riêng (số hạt nảy mầm luống đó, cùng
`E[Xᵢ]=2.0`). Kỳ vọng của TỔNG bốn luống — cộng thẳng bốn kỳ vọng
được không?
::::

::::explain{#tuyen-tinh-ky-vong}
Được — LUÔN được. **`E[X+Y] = E[X] + E[Y]`** — ĐÚNG kể cả khi `X`,
`Y` KHÔNG độc lập (khác hẳn bài 16, nơi độc lập LÀ điều kiện BẮT
BUỘC cho `P(A∩B)=P(A)P(B)`). Tổng kỳ vọng bốn luống = kỳ vọng của
tổng, cộng thẳng KHÔNG cần biết chúng có liên quan nhau hay không:

```python title=readonly
def tong_ky_vong(cac_ky_vong):
    tong = 0
    for e in cac_ky_vong:
        tong = tong + e
    return tong


print(tong_ky_vong([2.0, 2.0, 2.0, 2.0]))
```

```text title=readonly
8.0
```

Bốn luống, MỖI luống `E[Xᵢ]=2.0` (bài 24) — cộng thẳng `2+2+2+2=8.0`
LÀ kỳ vọng của TỔNG số hạt nảy mầm CẢ bốn luống. KHÔNG cần tính
phân phối của TỔNG (16×16×16×16 kết hợp có thể) rồi mới lấy kỳ vọng
— NHANH hơn hẳn.
::::

::::example{#khong-can-doc-lap}
Kể cả `X` VÀ CHÍNH `X` (cực kỳ PHỤ THUỘC — cùng một biến) — tuyến
tính VẪN đúng:

```python title=readonly
def tong_ky_vong(cac_ky_vong):
    tong = 0
    for e in cac_ky_vong:
        tong = tong + e
    return tong


ev_x = 2.0
print(tong_ky_vong([ev_x, ev_x]))
print(2 * ev_x)
```

```text title=readonly
4.0
4.0
```

`E[X+X] = E[X]+E[X] = 2×E[X] = 4.0` — ĐÚNG dù `X` cộng với CHÍNH nó
LÀ trường hợp phụ thuộc NHẤT có thể (biết `X` là biết NGAY giá trị
kia). Tuyến tính KHÔNG cần độc lập, khác HẲN quy tắc nhân xác suất
(bài 16, 17).
::::

::::predict{#doan-ky-vong-am commitOnce}
Byte có MỘT luống bị sâu bệnh — kỳ vọng THIỆT HẠI ghi bằng số ÂM
(`-1.0`, quy ước: âm nghĩa là mất), cộng chung VỚI hai luống khoẻ
(`3.0` VÀ `2.0`):

```python
def tong_ky_vong(cac_ky_vong):
    tong = 0
    for e in cac_ky_vong:
        tong = tong + e
    return tong

print(tong_ky_vong([3.0, 2.0, -1.0]))
```

Dòng cuối in ra gì?

:::opt{correct}
`4.0`
:::

:::opt
Máy báo lỗi khi chạy — kỳ vọng LÀ một xác suất-CÓ-trọng-số, và giá
trị `-1.0` (ÂM) không hợp lệ CHO một phép tính LIÊN QUAN xác suất
::why
Gần đúng ở việc bạn nhớ ĐÚNG xác suất (`P(A)`) LUÔN nằm giữa `0` và
`1`, KHÔNG bao giờ âm — một sự thật đã học đúng (bài 12).

Chỗ lệch: `-1.0` Ở ĐÂY KHÔNG PHẢI một xác suất — nó LÀ một GIÁ TRỊ
kỳ vọng (`E[X]`), và `X` (biến ngẫu nhiên) hoàn toàn có thể mang giá
trị ÂM (ví dụ: "thiệt hại", "lãi/lỗ") — CHỈ có XÁC SUẤT mới bị ràng
buộc trong `[0,1]`, không phải giá trị mà biến ngẫu nhiên NHẬN. Cộng
`3.0+2.0+(-1.0)=4.0` hoàn toàn hợp lệ.
::
:::

:::opt
`5.0` — vì cộng dồn CHỈ tính các giá trị DƯƠNG (luống khoẻ, `3.0+2.0
=5.0`), giá trị ÂM (luống sâu bệnh) bị bỏ QUA khỏi tổng vì nó KHÔNG
phải một kỳ vọng "thật"
::why
Gần đúng ở việc bạn tính ĐÚNG tổng hai luống khoẻ (`3.0+2.0=5.0`) —
phép cộng RIÊNG đó chính xác.

Chỗ lệch: vòng lặp cộng dồn trong `tong_ky_vong` đi qua MỌI phần tử
của danh sách, KHÔNG lọc riêng số ÂM hay DƯƠNG — nó cộng `3.0`, RỒI
`2.0`, RỒI `-1.0`, ra ĐÚNG `4.0`. Không có bước lọc nào trong hàm
này cả.
::
:::
::::

::::code{#viet_tong_ky_vong}
Viết `tong_ky_vong(cac_ky_vong)` — cộng dồn TOÀN BỘ kỳ vọng trong
`cac_ky_vong`.

```python title=starter
def tong_ky_vong(cac_ky_vong):
    tong = 0
    for e in cac_ky_vong:
        tong = ___
    return tong


print(tong_ky_vong([2.0, 2.0, 2.0, 2.0]))
```

```python title=solution
def tong_ky_vong(cac_ky_vong):
    tong = 0
    for e in cac_ky_vong:
        tong = tong + e
    return tong


print(tong_ky_vong([2.0, 2.0, 2.0, 2.0]))
```

```python title=test
assert tong_ky_vong([]) == 0, "khong bien nao -- tong 0"
assert tong_ky_vong([5.0]) == 5.0, "mot bien -- ket qua bang chinh ky vong do"
assert tong_ky_vong([2.0, 2.0]) == 4.0, "E[X+X] = 2E[X], du X phu thuoc tuyet doi voi chinh no"
assert tong_ky_vong([3.0, 2.0, -1.0]) == 4.0, "gia tri am hop le, khong bi loc bo"
```

:::hints
- kind: attention
  body: "Cong don tong voi e moi luot, dung vong lap da co."
- kind: strategy
  body: "tong + e"
- kind: one-line
  body: "tong = tong + e"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai cong don tong voi e moi luot
  requireAst:
  - kind: uses-operator, target: '+', min: 1
  - kind: uses-name, target: tong, min: 1
  - kind: uses-name, target: e, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^8\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Kỳ vọng đo TRUNG TÂM của `X`. Có con số nào đo `X` "TẢN RA" xa trung
tâm bao nhiêu không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Kỳ vọng (bài 24) đo được TRUNG TÂM của `X` — một con số DUY NHẤT tóm
tắt "trung bình". NHƯNG hai biến ngẫu nhiên có THỂ cùng kỳ vọng mà
"trải" RẤT khác nhau (một LUÔN gần trung tâm, một hay NHẢY xa). Có
con số nào đo được `X` "TẢN RA" xa trung tâm bao nhiêu không?
::::

::::checkpoint{mastery=0.8}
::::
