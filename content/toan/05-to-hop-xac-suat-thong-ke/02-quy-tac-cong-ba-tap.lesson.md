---
id: toan.to-hop-xac-suat-thong-ke.quy-tac-cong-ba-tap
title: Quy tắc cộng tổng quát — ba tập chồng lấn
summary: "|A∪B∪C| = |A|+|B|+|C| − |A∩B| − |A∩C| − |B∩C| + |A∩B∩C| — mở rộng bù trừ hai tập (T2.4 bài 13) lên ba, trả lời câu hỏi T2.4 để ngỏ."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.inclusion-exclusion-three]
requires: [math.rule-of-sum]
concepts: [math.bu-tru-ba-tap]
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
T2.4 bài 13 sửa được công thức cho HAI nhãn chồng lấn. Ba nhãn cùng
lúc — "giống mới", "chịu hạn", "ít nước" — trừ một lần còn đủ không?
::::

::::explain{#bu-tru-ba-tap}
Không đủ. **`|A∪B∪C| = |A|+|B|+|C| − |A∩B| − |A∩C| − |B∩C| +
|A∩B∩C|`** — cộng ba, TRỪ ba cặp giao, rồi CỘNG LẠI phần giao BA
(vì bước trừ vừa rồi lỡ trừ nó BA lần, phải bù lại):

```python title=readonly
def dem_hop_ba_tap(a, b, c):
    return len(a) + len(b) + len(c) - len(a & b) - len(a & c) - len(b & c) + len(a & b & c)


gm = {"h1", "h2", "h3", "h4"}
ch = {"h2", "h3", "h5", "h6"}
it_nuoc = {"h3", "h6", "h7"}

print(dem_hop_ba_tap(gm, ch, it_nuoc))
print(len(gm | ch | it_nuoc))
```

```text title=readonly
7
7
```

`h3` mang CẢ BA nhãn. Cộng thẳng `4+4+3=11` đếm `h3` BA lần (một lần
mỗi tập), rồi trừ ba cặp giao (`|gm∩ch|=2, |gm∩it_nuoc|=1,
|ch∩it_nuoc|=2`, tổng 5) đã trừ `h3` mất BA lần (nó nằm trong CẢ BA
cặp giao) — trừ HẾT rồi lại THIẾU nó hẳn, nên cộng lại đúng MỘT lần
`|gm∩ch∩it_nuoc|=1`. `11 − 5 + 1 = 7`, khớp đếm hợp thật.
::::

::::example{#khong-chong-tam}
Khi phần giao BA rỗng (không hạt nào mang cả ba nhãn), số hạng cuối
biến mất — công thức TỰ rút gọn:

```python title=readonly
def dem_hop_ba_tap(a, b, c):
    return len(a) + len(b) + len(c) - len(a & b) - len(a & c) - len(b & c) + len(a & b & c)


a = {"x1", "x2"}
b = {"x2", "x3"}
c = {"x4", "x5"}

print(a & b & c == set())
print(dem_hop_ba_tap(a, b, c))
print(len(a | b | c))
```

```text title=readonly
True
5
5
```

`a∩b∩c = ∅` nên `|a∩b∩c| = 0` — cộng lại "không gì". Công thức VẪN
đúng nguyên vẹn (`2+2+2−1−0−0+0=5`), chỉ số hạng cuối không đóng góp.
::::

::::predict{#doan-mot-cap-roi-nhau commitOnce}
Byte thử BA nhãn mà HAI trong ba đôi một rời nhau hoàn toàn — `a` và
`c` KHÔNG chung hạt nào (`a∩c=∅`), NHƯNG `b` chồng lấn CẢ HAI:

```python
def dem_hop_ba_tap(a, b, c):
    return len(a) + len(b) + len(c) - len(a & b) - len(a & c) - len(b & c) + len(a & b & c)

a = {"y1", "y2"}
b = {"y2", "y3"}
c = {"y3", "y4"}

print(len(a & c))
print(dem_hop_ba_tap(a, b, c))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`0`, rồi `4`
:::

:::opt
`0`, rồi `6` — vì `a` và `c` rời nhau (dòng đầu `0` đúng), công thức
CHỈ cần trừ MỘT cặp giao (`a∩b` hoặc `b∩c`), không phải cả ba số hạng
trừ
::why
Gần đúng ở việc bạn đọc ĐÚNG `a ∩ c = ∅` (dòng đầu `0`) — quan sát đó
chính xác.

Chỗ lệch: công thức KHÔNG "bỏ bớt" số hạng trừ chỉ vì MỘT cặp rời
nhau — nó vẫn TRỪ đủ ba cặp giao (`a∩b`, `a∩c`, `b∩c`), CHỈ LÀ
`|a∩c| = 0` nên số hạng đó KHÔNG đóng góp gì (trừ đi 0). Tính đủ:
`|a|+|b|+|c| = 2+2+2 = 6`, trừ `|a∩b|=1`, trừ `|a∩c|=0`, trừ
`|b∩c|=1`, cộng `|a∩b∩c|=0` (không hạt nào mang cả ba) → `6−1−0−1+0
= 4`. `y1,y2,y3,y4` — đúng 4 phần tử trong hợp thật.
::
:::

:::opt
Máy báo lỗi khi chạy — công thức có `a & c` nhưng `a` VÀ `c` rời
nhau, Python TỪ CHỐI tính giao của hai tập không chung phần tử nào
::why
Gần đúng ở việc bạn để ý `a` VÀ `c` không chung hạt nào — quan sát
ĐÓ đúng.

Chỗ lệch: `a & c` (giao của hai tập RỜI NHAU) hoàn toàn HỢP LỆ trong
Python — nó chỉ trả về `set()`, tập RỖNG, không phải lỗi. Rời nhau
LÀ một trường hợp BÌNH THƯỜNG của phép giao, không phải trường hợp
đặc biệt bị cấm.
::
:::
::::

::::code{#viet_dem_hop_ba_tap}
Viết `dem_hop_ba_tap(a, b, c)` — đếm số phần tử của `a ∪ b ∪ c` bằng
công thức bù trừ ba tập, KHÔNG dựng hợp trước.

```python title=starter
def dem_hop_ba_tap(a, b, c):
    return ___


gm = {"h1", "h2", "h3", "h4"}
ch = {"h2", "h3", "h5", "h6"}
it_nuoc = {"h3", "h6", "h7"}

print(dem_hop_ba_tap(gm, ch, it_nuoc))
```

```python title=solution
def dem_hop_ba_tap(a, b, c):
    return len(a) + len(b) + len(c) - len(a & b) - len(a & c) - len(b & c) + len(a & b & c)


gm = {"h1", "h2", "h3", "h4"}
ch = {"h2", "h3", "h5", "h6"}
it_nuoc = {"h3", "h6", "h7"}

print(dem_hop_ba_tap(gm, ch, it_nuoc))
```

```python title=test
assert dem_hop_ba_tap(set(), set(), set()) == 0, "ba tap rong -- tong rong"
assert dem_hop_ba_tap({"a"}, set(), set()) == 1, "hai tap rong -- tong bang tap con lai"
a = {"y1", "y2"}
b = {"y2", "y3"}
c = {"y3", "y4"}
assert dem_hop_ba_tap(a, b, c) == 4, "mot cap roi nhau (a,c), b chong len ca hai"
assert dem_hop_ba_tap(gm, ch, it_nuoc) == len(gm | ch | it_nuoc), "phai khop dem hop that khi ca ba chong lan"
assert dem_hop_ba_tap(gm, gm, gm) == len(gm), "ba tap giong het nhau -- hop van la chinh no"
```

:::hints
- kind: attention
  body: "Cong ba do dai, tru ba cap giao, roi cong lai giao ca ba."
- kind: strategy
  body: "len(a) + len(b) + len(c) - len(a & b) - len(a & c) - len(b & c) + len(a & b & c)"
- kind: one-line
  body: "___ = len(a) + len(b) + len(c) - len(a & b) - len(a & c) - len(b & c) + len(a & b & c)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung du bay so hang cua cong thuc bu tru ba tap -- ba do dai, ba giao doi, mot giao ba
  requireAst:
  - kind: uses-call, target: len, min: 7
  - kind: uses-operator, target: '+', min: 1
  - kind: uses-operator, target: '-', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^7\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhãn xong. Giờ Byte muốn XẾP THỨ TỰ thu hoạch bốn luống — quy tắc
cộng có trả lời được câu ấy không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đếm nhãn xong. Giờ Byte muốn biết có bao nhiêu cách XẾP THỨ TỰ thu
hoạch bốn luống trong một buổi sáng — luống nào trước, luống nào
sau. Quy tắc cộng có trả lời được câu "bao nhiêu cách xếp" không,
hay đây là một câu hỏi khác hẳn?
::::

::::checkpoint{mastery=0.8}
::::
