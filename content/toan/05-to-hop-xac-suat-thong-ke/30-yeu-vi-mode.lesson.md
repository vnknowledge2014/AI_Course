---
id: toan.to-hop-xac-suat-thong-ke.yeu-vi-mode
title: Yếu vị (mode)
summary: "Mode — giá trị (hoặc CÁC giá trị, có thể có nhiều mode) xuất hiện NHIỀU LẦN nhất; KHÁC trung vị (vị trí giữa) VÀ trung bình (tổng chia đều) — đo \"phổ biến nhất\", không đo \"trung tâm\" hay \"đại diện tổng\"."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 30
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.mode]
requires: [math.mean-vs-median-robustness]
concepts: [math.yeu-vi]
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
Lan có sáu mùa mà HAI mùa cùng cho đúng `40` kg. Số nào xuất hiện
NHIỀU LẦN nhất trong một tập dữ liệu — có tên riêng không?
::::

::::explain{#mode-la-gi}
Có. **Mode (yếu vị)** — giá trị (hoặc CÁC giá trị, CÓ THỂ có NHIỀU
mode) xuất hiện NHIỀU LẦN nhất; KHÁC trung vị (VỊ TRÍ giữa, bài 28)
VÀ trung bình (TỔNG chia đều, bài 27) — đo "PHỔ BIẾN nhất", không đo
"trung tâm" hay "đại diện tổng":

```python title=readonly
def mode(du_lieu):
    tap_gia_tri = set(du_lieu)
    so_lan_max = max(du_lieu.count(x) for x in tap_gia_tri)
    return {x for x in tap_gia_tri if du_lieu.count(x) == so_lan_max}


mua_lan = [40, 40, 38, 41, 39, 42]

print(mode(mua_lan))
```

```text title=readonly
{40}
```

`40` xuất hiện HAI lần (`du_lieu.count(40)=2`); mọi giá trị khác chỉ
MỘT lần. `so_lan_max=2`, và CHỈ `40` đạt SỐ LẦN đó — `mode = {40}`.
::::

::::example{#nhieu-mode}
Khi HAI (hay nhiều) giá trị CÙNG đạt số lần CAO nhất — `mode` chứa
CẢ hai:

```python title=readonly
def mode(du_lieu):
    tap_gia_tri = set(du_lieu)
    so_lan_max = max(du_lieu.count(x) for x in tap_gia_tri)
    return {x for x in tap_gia_tri if du_lieu.count(x) == so_lan_max}


print(mode([1, 1, 2, 2, 3]))
```

```text title=readonly
{1, 2}
```

`1` VÀ `2` ĐỀU xuất hiện hai lần (CAO nhất); `3` chỉ một lần. CẢ hai
đạt `so_lan_max=2` — `mode` chứa CẢ HAI, không CHỌN một trong hai.
::::

::::predict{#doan-du-lieu-khong-co-trung commitOnce}
Byte tính `mode` TRÊN chính dữ liệu của Byte (không mùa nào TRÙNG
kilôgam với mùa khác):

```python
def mode(du_lieu):
    tap_gia_tri = set(du_lieu)
    so_lan_max = max(du_lieu.count(x) for x in tap_gia_tri)
    return {x for x in tap_gia_tri if du_lieu.count(x) == so_lan_max}

mua_byte = [40, 42, 41, 39, 43, 5]
print(len(mode(mua_byte)))
```

Dòng cuối in ra gì?

:::opt{correct}
`6`
:::

:::opt
`0` — vì KHÔNG giá trị nào LẶP LẠI, nên KHÔNG có mode nào cả (tập
mode RỖNG khi mọi giá trị đều DUY NHẤT)
::why
Gần đúng ở việc bạn nghĩ "không lặp thì không có mode" — một cảm
giác hợp lý VỀ Ý NGHĨA "phổ biến nhất" nghe như cần LẶP LẠI mới có.

Chỗ lệch: `so_lan_max = max(...)` LUÔN tính RA một con số, KỂ CẢ khi
MỌI giá trị chỉ xuất hiện MỘT lần — lúc đó `so_lan_max=1`, và MỌI
giá trị (đều đạt ĐÚNG `1` lần) đều được XẾP vào `mode`. Không lặp
lại KHÔNG nghĩa là "không mode" — nó nghĩa MỌI giá trị đều "phổ biến
NHƯ NHAU" (mỗi giá trị MỘT lần), nên TẤT CẢ đều LÀ mode.
::
:::

:::opt
Máy báo lỗi khi chạy — `max(du_lieu.count(x) for x in tap_gia_tri)`
cần Ít NHẤT một giá trị LẶP LẠI để có "MAX" thật sự, không lặp lại
gì thì `max()` không tính được
::why
Gần đúng ở việc bạn nghĩ TỚI `max()` như CẦN sự khác BIỆT giữa các
giá trị mới "có Ý nghĩa" — một trực giác dễ hiểu nhầm.

Chỗ lệch: `max()` chỉ đơn giản tìm SỐ LỚN NHẤT trong một dãy — dãy
`[1,1,1,1,1,1]` (mọi count đều là `1`) VẪN có max, đơn giản LÀ `1`.
Không cần "khác biệt" gì; `max` của MỘT dãy toàn số giống nhau chính
LÀ số đó.
::
:::
::::

::::code{#viet_mode}
Viết `mode(du_lieu)` — trả về TẬP các giá trị xuất hiện NHIỀU LẦN
nhất trong `du_lieu`.

```python title=starter
def mode(du_lieu):
    tap_gia_tri = set(du_lieu)
    so_lan_max = max(du_lieu.count(x) for x in tap_gia_tri)
    return ___


mua_lan = [40, 40, 38, 41, 39, 42]

print(mode(mua_lan))
```

```python title=solution
def mode(du_lieu):
    tap_gia_tri = set(du_lieu)
    so_lan_max = max(du_lieu.count(x) for x in tap_gia_tri)
    return {x for x in tap_gia_tri if du_lieu.count(x) == so_lan_max}


mua_lan = [40, 40, 38, 41, 39, 42]

print(mode(mua_lan))
```

```python title=test
assert mode([1, 1, 2, 2, 3]) == {1, 2}, "hai mode cung dat so lan cao nhat"
assert mode([5]) == {5}, "mot phan tu -- chinh no la mode"
assert mode([40, 40, 38, 41, 39, 42]) == {40}, "phai khop vi du chinh"
assert len(mode([40, 42, 41, 39, 43, 5])) == 6, "khong lap lai gi -- moi gia tri deu la mode"
```

:::hints
- kind: attention
  body: "Dung set comprehension: giu lai nhung gia tri x co du_lieu.count(x) bang so_lan_max."
- kind: strategy
  body: "{x for x in tap_gia_tri if du_lieu.count(x) == so_lan_max}"
- kind: one-line
  body: "___ = {x for x in tap_gia_tri if du_lieu.count(x) == so_lan_max}"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung set comprehension giu lai gia tri co count bang so_lan_max
  requireAst:
  - kind: comprehension, min: 2
  - kind: uses-call, target: count, min: 2
  - kind: uses-operator, target: '==', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\{40\}\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Trung bình, trung vị, mode — ba con số, ba câu hỏi khác nhau. Có
con số nào đo được dữ liệu THẬT "tản ra" xa trung bình bao nhiêu?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sáu mùa của Lan: `40, 40, 38, 41, 39, 42`. Mode là `40` (2 lần). Đổi
một mùa `38` thành `40` — giờ `40` xuất hiện BA lần, còn các số khác
vẫn mỗi số MỘT lần. `x̄` và trung vị đổi Ở MỨC nào so với mode?
::::

::::checkpoint{mastery=0.8}
::::
