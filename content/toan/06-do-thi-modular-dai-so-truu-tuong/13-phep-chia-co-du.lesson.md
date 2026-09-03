---
id: toan.do-thi-modular-dai-so-truu-tuong.phep-chia-co-du
title: Phép chia có dư
summary: "a = q·n + r, 0 ≤ r < n — phép chia có dư (ý nghĩa toán của %, R1 core.modulo đã dạy CÁCH GÕ); MỌI số nguyên a VÀ MỘT số n>0 xác định DUY NHẤT cặp (q, r) — nền cho đồng dư (bài 14)."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.division-with-remainder]
requires: [math.remainder]
concepts: [math.phep-chia-co-du]
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
Lịch tưới LẶP mỗi BA ngày: Hai, Tư, Bảy, Hai, Tư, Bảy,... Ngày thứ
`10` (đếm từ `1`) LÀ ngày NÀO trong chu kỳ?
::::

::::explain{#phep-chia-co-du-la-gi}
**Phép chia có dư**: `a = q·n + r`, VỚI `0 ≤ r < n` — MỌI số nguyên
`a` VÀ MỘT số `n>0` xác định DUY NHẤT cặp `(q, r)` (`%` VÀ `//`, R1
`core.modulo` ĐÃ dạy CÁCH gõ; bài NÀY đặt Ý NGHĨA):

```python title=readonly
def chia_co_du(a, n):
    return (a // n, a % n)


print(chia_co_du(10, 3))
```

```text title=readonly
(3, 1)
```

Ngày thứ `10`, chu kỳ BA ngày: `10 = 3×3 + 1` — đi qua ĐÚNG BA chu
kỳ TRỌN VẸN (chín ngày), RỒI dư ĐÚNG MỘT bước — CÙNG "vị trí" trong
chu kỳ với ngày thứ `1` (r cũng LÀ `1`).
::::

::::example{#khep-dung-chu-ky}
Ngày thứ `9` — KHÉP đúng chu kỳ, KHÔNG dư bước nào:

```python title=readonly
def chia_co_du(a, n):
    return (a // n, a % n)


print(chia_co_du(9, 3))
```

```text title=readonly
(3, 0)
```

`9 = 3×3 + 0` — dư `r=0` nghĩa LÀ ngày thứ `9` KHÉP ĐÚNG chu kỳ thứ
ba, KHÔNG dư bước nào ("Bảy", ngày CUỐI chu kỳ ba ngày).
::::

::::predict{#doan-a-nho-hon-n commitOnce}
Byte tính `chia_co_du` cho `a=2, n=5` — `a` NHỎ HƠN `n`:

```python
def chia_co_du(a, n):
    return (a // n, a % n)

print(chia_co_du(2, 5))
```

Dòng cuối in ra gì?

:::opt{correct}
`(0, 2)`
:::

:::opt
Máy báo lỗi khi chạy — `a` NHỎ hơn `n` (`2 < 5`) nên phép chia
"KHÔNG thực hiện được", `//` VÀ `%` đòi `a` PHẢI lớn hơn HOẶC bằng
`n`
::why
Gần đúng ở việc bạn để ý ĐÚNG `a < n` LÀ một TÌNH huống KHÁC thường
so với ví dụ TRƯỚC (`10`, `9` đều LỚN hơn `3`) — một quan sát VỀ
DỮ liệu.

Chỗ lệch: `//` VÀ `%` HOẠT động BÌNH thường VỚI MỌI `a ≥ 0` VÀ
`n > 0`, KHÔNG cần `a ≥ n` — khi `a < n`, thương `q` ĐƠN GIẢN LÀ
`0` (đi được `0` chu kỳ TRỌN VẸN), VÀ dư `r` CHÍNH LÀ `a` (CẢ `a`
đều LÀ phần "dư", chưa đủ MỘT chu kỳ). Biên dịch sạch, chạy sạch.
::
:::

:::opt
`(0, 5)` — vì `a` nhỏ hơn `n` nên thương LÀ `0`, VÀ dư LÀ CHÍNH `n`
(khoảng CÁCH còn thiếu để đủ MỘT chu kỳ)
::why
Gần đúng ở việc bạn đoán ĐÚNG thương `q=0` — khi `a<n`, ĐÚNG LÀ đi
được `0` chu kỳ trọn vẹn, một QUAN sát chính xác VỀ phần THƯƠNG.

Chỗ lệch: dư `r` KHÔNG phải "khoảng cách CÒN thiếu" — nó LÀ phần
CỦA `a` CHƯA đủ một chu kỳ, TỨC LÀ CHÍNH `a` (`r=2`, không phải `5`
— VÀ theo định nghĩa `0 ≤ r < n`, `r` KHÔNG BAO GIỜ được PHÉP bằng
`n`, `5` sẽ VI PHẠM ràng buộc ĐÓ).
::
:::
::::

::::code{#viet_chia_co_du}
Viết `chia_co_du(a, n)` — trả VỀ cặp `(q, r)` sao cho `a = q·n + r`
VÀ `0 ≤ r < n`.

```python title=starter
def chia_co_du(a, n):
    return ___


print(chia_co_du(10, 3))
```

```python title=solution
def chia_co_du(a, n):
    return (a // n, a % n)


print(chia_co_du(10, 3))
```

```python title=test
assert chia_co_du(10, 3) == (3, 1), "10 = 3x3+1"
assert chia_co_du(9, 3) == (3, 0), "9 = 3x3+0 -- khep dung chu ky"
assert chia_co_du(2, 5) == (0, 2), "a nho hon n -- thuong 0, du la chinh a"
assert chia_co_du(0, 7) == (0, 0), "a bang 0 -- thuong va du deu 0"
q, r = chia_co_du(23, 7)
assert q * 7 + r == 23, "dang thuc a = q*n+r phai dung"
assert 0 <= r < 7, "du phai nam trong [0, n)"
```

:::hints
- kind: attention
  body: "// cho thuong (q), % cho du (r) -- Python da co san hai toan tu nay."
- kind: strategy
  body: "(a // n, a % n)"
- kind: one-line
  body: "___ = (a // n, a % n)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tra ve tuple (a // n, a % n)
  requireAst:
  - kind: uses-operator, target: '//', min: 1
  - kind: uses-operator, target: '%', min: 1
  - kind: uses-name, target: a, min: 2
  - kind: uses-name, target: n, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(3, 1\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`(q, r)` DUY NHẤT cho MỖI cặp `(a, n)`. Ngày thứ `10` VÀ ngày thứ
`13` — CÙNG dư `1` khi chia CHO `3` — chúng CÓ "CÙNG một ngày" trong
chu kỳ không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ngày thứ `10` VÀ ngày thứ `13` (chu kỳ BA ngày) — có ĐỒNG DƯ modulo
`3` không? Nếu có, chúng có LUÔN rơi VÀO CÙNG một NGÀY trong chu kỳ
không?
::::

::::checkpoint{mastery=0.8}
::::
