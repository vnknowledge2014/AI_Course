---
id: toan.to-hop-xac-suat-thong-ke.phat-hien-ngoai-le-iqr
title: Phát hiện ngoại lệ bằng IQR
summary: "IQR = Q3 − Q1; một điểm dữ liệu LÀ ngoại lệ (outlier) nếu nó nằm NGOÀI khoảng [Q1 − 1.5·IQR, Q3 + 1.5·IQR] — một QUY TẮC cụ thể để trả lời \"con số nào là bất thường\", không còn phải NHÌN bằng mắt."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 33
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.outlier-detection]
requires: [math.quartile]
concepts: [math.ngoai-le-iqr]
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
`Q1=38`, `Q3=44`. Một mùa tụt xuống `10` kg — trông "bất thường",
nhưng "bất thường" theo QUY TẮC nào, không chỉ NHÌN bằng mắt?
::::

::::explain{#iqr-la-gi}
**`IQR = Q3 − Q1`**; một điểm dữ liệu LÀ **ngoại lệ (outlier)** NẾU
nó nằm NGOÀI khoảng `[Q1 − 1.5·IQR, Q3 + 1.5·IQR]` — một QUY TẮC cụ
thể để trả lời "con số nào LÀ bất thường" (bài 27 để ngỏ), không còn
phải NHÌN bằng mắt:

```python title=readonly
def la_ngoai_le(x, q1, q3):
    iqr = q3 - q1
    return x < q1 - 1.5 * iqr or x > q3 + 1.5 * iqr


print(la_ngoai_le(10, 38, 44))
```

```text title=readonly
True
```

`IQR = 44 − 38 = 6`. Khoảng "bình thường" LÀ `[38 − 1.5×6, 44 +
1.5×6] = [29, 53]`. `10 < 29` — NẰM NGOÀI khoảng, LÀ ngoại lệ.
::::

::::example{#khong-phai-ngoai-le}
Một giá trị GẦN trung tâm — NẰM TRONG khoảng, KHÔNG phải ngoại lệ:

```python title=readonly
def la_ngoai_le(x, q1, q3):
    iqr = q3 - q1
    return x < q1 - 1.5 * iqr or x > q3 + 1.5 * iqr


print(la_ngoai_le(40, 38, 44))
```

```text title=readonly
False
```

`40` NẰM giữa `38` và `44` (thậm chí giữa CHÍNH khoảng tứ phân vị,
chưa cần TỚI hệ số `1.5`) — chắc chắn KHÔNG phải ngoại lệ.
::::

::::predict{#doan-ngoai-le-tai-bien commitOnce}
Byte thử ĐÚNG giá trị BIÊN — `x = 29`, đúng CẬN dưới của khoảng
`[29, 53]`:

```python
def la_ngoai_le(x, q1, q3):
    iqr = q3 - q1
    return x < q1 - 1.5 * iqr or x > q3 + 1.5 * iqr

print(la_ngoai_le(29, 38, 44))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì `29` LÀ đúng ranh giới của khoảng "bình thường", và một
điểm NẰM ĐÚNG TRÊN ranh giới thường được coi LÀ đã "chạm ngưỡng",
tức LÀ ngoại lệ
::why
Gần đúng ở việc bạn nhận ra `29` LÀ một điểm ĐẶC BIỆT (đúng cận) —
một quan sát tinh Ý VỀ VỊ TRÍ của nó.

Chỗ lệch: công thức dùng phép so sánh NGHIÊM NGẶT `<` VÀ `>` (KHÔNG
PHẢI `<=`/`>=`) — `29 < 29` LÀ `False` (không NHỎ HƠN chính nó), và
`29 > 53` CŨNG `False`. Ranh giới ĐƯỢC TÍNH LÀ THUỘC khoảng "bình
thường" (bao gồm CẢ hai đầu mút), không phải "đã chạm ngưỡng".
::
:::

:::opt
Máy báo lỗi khi chạy — `29` TRÙNG chính XÁC giá trị cận dưới
(`q1 − 1.5·iqr = 29`), và so sánh MỘT số VỚI CHÍNH NÓ bằng `<` gây
lỗi
::why
Gần đúng ở việc bạn để ý `29` TRÙNG với chính CẬN dưới — một quan
sát đúng VỀ mặt SỐ HỌC.

Chỗ lệch: so sánh MỘT số với CHÍNH NÓ bằng `<` (hay `>`) hoàn toàn
HỢP LỆ trong Python — nó chỉ đơn giản trả về `False` (một số KHÔNG
nhỏ hơn CHÍNH nó). Biên dịch sạch, chạy sạch, không có gì đặc biệt.
::
:::
::::

::::code{#viet_la_ngoai_le}
Viết `la_ngoai_le(x, q1, q3)` — kiểm `x` có nằm NGOÀI khoảng
`[Q1−1.5·IQR, Q3+1.5·IQR]` hay không.

```python title=starter
def la_ngoai_le(x, q1, q3):
    iqr = q3 - q1
    return ___


print(la_ngoai_le(10, 38, 44))
```

```python title=solution
def la_ngoai_le(x, q1, q3):
    iqr = q3 - q1
    return x < q1 - 1.5 * iqr or x > q3 + 1.5 * iqr


print(la_ngoai_le(10, 38, 44))
```

```python title=test
assert la_ngoai_le(40, 38, 44) is False, "gan trung tam -- khong phai ngoai le"
assert la_ngoai_le(29, 38, 44) is False, "dung can duoi -- van thuoc khoang binh thuong"
assert la_ngoai_le(28, 38, 44) is True, "vua qua can duoi mot chut -- ngoai le"
assert la_ngoai_le(54, 38, 44) is True, "vua qua can tren mot chut -- ngoai le"
assert la_ngoai_le(5, 38, 44) is True, "mua mat trang cua Byte -- ngoai le that"
```

:::hints
- kind: attention
  body: "Tinh iqr = q3 - q1, roi kiem x < q1 - 1.5*iqr hoac x > q3 + 1.5*iqr."
- kind: strategy
  body: "x < q1 - 1.5 * iqr or x > q3 + 1.5 * iqr"
- kind: one-line
  body: "___ = x < q1 - 1.5 * iqr or x > q3 + 1.5 * iqr"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai kiem x < q1-1.5*iqr HOAC x > q3+1.5*iqr, dung or
  requireAst:
  - kind: uses-operator, target: or, min: 1
  - kind: uses-operator, target: '<', min: 1
  - kind: uses-operator, target: '>', min: 1
  - kind: has-literal, target: '1.5'
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mùa `5` kg của Byte — kiểm bằng IQR có thật sự là ngoại lệ không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mùa `5` kg của Byte (bài 27) — kiểm bằng IQR có thật sự LÀ ngoại lệ,
hay chỉ "trông có vẻ thấp"? Giờ track đã có: đếm (1-10), xác suất
(11-21), biến ngẫu nhiên (22-26), thống kê mô tả (27-33). Ghép cả
bốn phần lên MỘT vụ mùa trông như thế nào?
::::

::::checkpoint{mastery=0.8}
::::
