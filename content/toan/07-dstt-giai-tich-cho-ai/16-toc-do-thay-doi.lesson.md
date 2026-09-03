---
id: toan.dstt-giai-tich-cho-ai.toc-do-thay-doi
title: Tốc độ thay đổi
summary: "Tốc độ thay đổi trung bình — Δy/Δx, độ DỐC giữa hai thời điểm ĐO; luống dài 2m NGÀY 1, 5m ngày 4 — tốc độ LỚN trung bình LÀ 1m/ngày."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.average-rate-of-change]
requires: [math.ratio]
concepts: [math.toc-do-thay-doi]
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
Vườn CỦA Byte đo được BAO NHIÊU (dài, nước, nắng) — nhưng CÁC con số
ĐÓ đo theo THỜI GIAN thì SAO? Luống LỚN lên bao NHANH?
::::

::::explain{#toc-do-thay-doi}
**Tốc độ thay đổi TRUNG BÌNH** — `Δy/Δx`, độ DỐC giữa hai thời điểm
ĐO (tỉ số của hai đại LƯỢNG khác LOẠI, T2.1: mét TRÊN ngày):

```python title=readonly
def toc_do_tb(y1, y2, x1, x2):
    return (y2 - y1) / (x2 - x1)


print(toc_do_tb(2.0, 5.0, 1, 4))
```

```text title=readonly
1.0
```

Luống dài `2m` NGÀY `1`, `5m` NGÀY `4` — TĂNG `3m` TRONG `3` ngày —
tốc độ LỚN trung bình LÀ `1m/ngày`. `Δy = y2−y1 = 3`, `Δx = x2−x1 =
3`, `Δy/Δx = 1`.
::::

::::example{#khong-doi}
KHÔNG thay đổi GÌ — tốc độ LÀ `0`:

```python title=readonly
def toc_do_tb(y1, y2, x1, x2):
    return (y2 - y1) / (x2 - x1)


print(toc_do_tb(5.0, 5.0, 1, 10))
```

```text title=readonly
0.0
```

`y1=y2=5.0` — CHIỀU dài KHÔNG đổi SUỐT `9` ngày — tốc độ thay đổi
LÀ `0`, DÙ khoảng THỜI gian trôi QUA khá lâu.
::::

::::predict{#doan-toc-do-am commitOnce}
Byte đo MỘT lượng nước ĐANG cạn dần — `10.0L` NGÀY `1`, `2.0L` ngày
`3`:

```python
def toc_do_tb(y1, y2, x1, x2):
    return (y2 - y1) / (x2 - x1)

print(toc_do_tb(10.0, 2.0, 1, 3))
```

Dòng cuối in ra gì?

:::opt{correct}
`-4.0`
:::

:::opt
`4.0` — vì "tốc độ" LUÔN LÀ một con số KHÔNG âm (đo mức độ NHANH,
không đo HƯỚNG), NÊN Python TỰ lấy trị TUYỆT đối
::why
Gần đúng ở việc bạn nghĩ "tốc độ" thường được HIỂU theo nghĩa
KHÔNG âm (như tốc độ XE chạy) — một trực GIÁC quen thuộc trong đời
sống.

Chỗ lệch: Ở đây `toc_do_tb` KHÔNG hề lấy trị TUYỆT đối — dấu ÂM
mang Ý nghĩa THẬT: `y` đang GIẢM (nước CẠN dần). Hiệu `y2 trừ y1`
LÀ số ÂM (nước Ở NGÀY sau ÍT hơn ngày trước), chia CHO hiệu `x2 trừ
x1` (số dương) VẪN cho kết quả ÂM — dấu ÂM báo hiệu chiều GIẢM, một
THÔNG tin quan TRỌNG bị mất NẾU lấy tuyệt đối.
::
:::

:::opt
Máy báo lỗi khi chạy — `y2 < y1` (`2.0 < 10.0`), Python KHÔNG cho
phép trừ một số LỚN cho ra kết quả ÂM trong ngữ cảnh "tốc độ"
::why
Gần đúng ở việc bạn để ý ĐÚNG `y2` NHỎ hơn `y1` — một quan sát VỀ
dữ liệu.

Chỗ lệch: Python KHÔNG có khái NIỆM "ngữ cảnh tốc độ" giới hạn phép
trừ — `y2 - y1` tính BÌNH thường, RA số ÂM khi `y2<y1`, KHÔNG lỗi
gì. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_toc_do_tb}
Viết `toc_do_tb(y1, y2, x1, x2)` — tính tốc độ thay đổi TRUNG BÌNH.

```python title=starter
def toc_do_tb(y1, y2, x1, x2):
    return ___


print(toc_do_tb(2.0, 5.0, 1, 4))
```

```python title=solution
def toc_do_tb(y1, y2, x1, x2):
    return (y2 - y1) / (x2 - x1)


print(toc_do_tb(2.0, 5.0, 1, 4))
```

```python title=test
assert toc_do_tb(2.0, 5.0, 1, 4) == 1.0, "tang deu"
assert toc_do_tb(5.0, 5.0, 1, 10) == 0.0, "khong doi"
assert toc_do_tb(10.0, 2.0, 1, 3) == -4.0, "giam -- toc do am"
assert toc_do_tb(0.0, 10.0, 0, 5) == 2.0, "tu goc"
```

:::hints
- kind: attention
  body: "Chia (y2 - y1) cho (x2 - x1)."
- kind: strategy
  body: "(y2 - y1) / (x2 - x1)"
- kind: one-line
  body: "___ = (y2 - y1) / (x2 - x1)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai chia (y2-y1) cho (x2-x1)
  requireAst:
  - kind: uses-operator, target: '-', min: 2
  - kind: uses-operator, target: '/', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^1\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tốc độ trung bình — độ dốc giữa hai mốc. Đo GIỮA hai mốc CỰC gần
nhau — con số CÓ ổn định lại không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đo tốc độ lớn GIỮA ngày `1` VÀ ngày `1.001` (khoảng CÁCH cực NHỎ) —
con số CÓ ổn định LẠI một giá trị, hay cứ đổi MÃI khi khoảng cách
càng NHỎ?
::::

::::checkpoint{mastery=0.8}
::::
