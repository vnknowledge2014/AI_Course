---
id: toan.tap-hop-quan-he-anh-xa.hai-tap-bang-nhau
title: Hai tập hợp bằng nhau
summary: "A = B khi và chỉ khi (A ⊆ B) VÀ (B ⊆ A) — chứng minh hai tập bằng nhau bằng cách chứng minh HAI chiều tập con, đúng kỹ thuật \"khi và chỉ khi\" (T2.3) áp dụng vào tập hợp."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.set-equality]
requires: [math.subset, logic.biconditional]
concepts: [math.bang-nhau-qua-tap-con, math.hai-chieu]
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
Lan kể luống 1 theo một thứ tự, Minh kể theo thứ tự khác — Python `==`
xác nhận chúng bằng nhau NGAY (bài 1). Nhưng CHỨNG MINH bằng tay thì
phải chỉ ra điều gì?
::::

::::explain{#bang-nhau-qua-hai-chieu}
Chứng minh **A = B** bằng cách chứng minh **HAI CHIỀU tập con**: `(A ⊆
B)` VÀ `(B ⊆ A)`. Đúng kỹ thuật "khi và chỉ khi" đã học Ở T2.3 (bài 15)
— hai chiều kéo theo GỘP LẠI thành một câu tương đương.

```python title=readonly
def la_tap_con(a, b):
    return all(x in b for x in a)


luong_1 = {"cà chua", "xà lách"}
luong_1_ban_2 = {"xà lách", "cà chua", "cà chua"}

print(la_tap_con(luong_1, luong_1_ban_2) and la_tap_con(luong_1_ban_2, luong_1))
```

```text title=readonly
True
```

Chiều MỘT: mọi phần tử của `luong_1` có mặt trong `luong_1_ban_2`.
Chiều HAI: mọi phần tử của `luong_1_ban_2` có mặt trong `luong_1`. Cả
hai chiều đúng → hai tập BẰNG nhau — KHÔNG cần đếm ký tự, KHÔNG cần
nhìn thứ tự viết ra.
::::

::::example{#chi-mot-chieu-thi-khong-du}
CHỈ một chiều đúng thì KHÔNG đủ để kết luận bằng nhau:

```python title=readonly
def la_tap_con(a, b):
    return all(x in b for x in a)


a = {"cà chua", "xà lách", "cà rốt"}
b = {"cà chua", "xà lách"}

print(la_tap_con(b, a))
print(la_tap_con(a, b))
```

```text title=readonly
True
False
```

`b ⊆ a` đúng — MỌI phần tử của `b` có mặt trong `a`. NHƯNG `a ⊆ b` sai
— `a` có thêm `cà rốt` mà `b` không có. CHỈ chiều một đúng: `a` và `b`
KHÔNG bằng nhau, `a` chỉ là tập LỚN HƠN chứa `b` (một khái niệm khác,
"tập con THỰC SỰ" — không phải trọng tâm bài này).
::::

::::predict{#doan-mot-chieu-that-bai commitOnce}
Byte thử kiểm tra `a` và `b` (từ ví dụ trên) có bằng nhau không, đối
chiếu với `==` của Python:

```python
def la_tap_con(a, b):
    return all(x in b for x in a)


a = {"cà chua", "xà lách", "cà rốt"}
b = {"cà chua", "xà lách"}

print(a == b)
print(la_tap_con(a, b) and la_tap_con(b, a))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`False`, rồi `False`
:::

:::opt
`False`, rồi `True` — vì `b ⊆ a` đúng (đã thấy Ở ví dụ trên), và MỘT
chiều tập con đúng LÀ đủ để tính LÀ "bằng nhau MỘT PHẦN", nên phép `and`
CHỈ cần MỘT trong hai chiều đúng là đủ để cho `True`
::why
Gần đúng ở việc bạn nhớ ĐÚNG `b ⊆ a` là `True` (đã thấy Ở ví dụ trên)
— một quan sát chính xác về MỘT chiều.

Chỗ lệch: `and` (đã học từ R0/T2.3) đòi CẢ HAI vế cùng `True` mới cho
`True` — KHÔNG PHẢI "một trong hai đủ" (đó là `or`). `la_tap_con(a, b)`
là `False` (chiều còn lại, `a ⊆ b`, sai vì `a` có thêm `cà rốt`) — nên
`False and True` là `False`. Chỉ MỘT chiều đúng thì phép `and` LUÔN
cho `False`, khớp đúng `a == b` cũng là `False`.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `la_tap_con` hai lần TRONG cùng một biểu
thức `and` (`la_tap_con(a, b) and la_tap_con(b, a)`) là đệ quy GIÁN
TIẾP, Python cấm một hàm gọi chính kiểu hàm của nó hai lần liên tiếp
::why
Gần đúng ở việc bạn để ý `la_tap_con` được gọi HAI lần trong CÙNG một
dòng — một quan sát đúng về SỐ LẦN gọi.

Chỗ lệch: gọi CÙNG một hàm nhiều lần trong một biểu thức (với đối số
KHÁC nhau mỗi lần, ở đây là `(a, b)` rồi `(b, a)`) hoàn toàn không phải
đệ quy — đệ quy nghĩa là một hàm gọi CHÍNH NÓ TỪ BÊN TRONG THÂN của nó.
Đây chỉ là hai LỜI GỌI độc lập, y hệt gọi `print()` hai lần. Biên dịch
sạch, chạy sạch.
::
:::
::::

::::code{#viet_hai_tap_bang_nhau}
Viết `hai_tap_bang_nhau(a, b)` — trả về `True` khi VÀ CHỈ KHI `a` và
`b` bằng nhau, chứng minh bằng HAI chiều tập con.

```python title=starter
def la_tap_con(a, b):
    return all(x in b for x in a)


def hai_tap_bang_nhau(a, b):
    return ___


print(hai_tap_bang_nhau({"a", "b"}, {"b", "a", "a"}))
```

```python title=solution
def la_tap_con(a, b):
    return all(x in b for x in a)


def hai_tap_bang_nhau(a, b):
    return la_tap_con(a, b) and la_tap_con(b, a)


print(hai_tap_bang_nhau({"a", "b"}, {"b", "a", "a"}))
```

```python title=test
assert hai_tap_bang_nhau({"a", "b"}, {"a", "b", "c"}) is False, "b thieu c -- khong bang nhau"
assert hai_tap_bang_nhau({"a", "b", "c"}, {"a", "b"}) is False, "dao lai -- van khong bang nhau"
assert hai_tap_bang_nhau(set(), set()) is True, "hai tap rong bang nhau"
assert hai_tap_bang_nhau({"a"}, set()) is False, "mot tap co phan tu, mot tap rong -- khong bang nhau"
assert hai_tap_bang_nhau({"x", "y", "z"}, {"z", "y", "x"}) is True, "thu tu viet khac nhau van la cung mot tap hop"
```

:::hints
- kind: attention
  body: "Dung and de ghep HAI chieu la_tap_con: chieu a vao b, VA chieu b vao a."
- kind: strategy
  body: "la_tap_con(a, b) and la_tap_con(b, a)"
- kind: one-line
  body: "___ = la_tap_con(a, b) and la_tap_con(b, a)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi la_tap_con CA HAI chieu (a,b) VA (b,a) roi ghep bang and -- chi mot chieu, hoac dung == truc tiep, khong dung y bai nay muon ban tu tay chung minh
  requireAst:
  - kind: uses-call, target: la_tap_con, min: 2
  - kind: uses-operator, target: and, min: 1
  - kind: uses-name, target: a, min: 1
  - kind: uses-name, target: b, min: 1
  forbidAst:
  - kind: uses-operator, target: '=='
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bằng nhau = hai chiều tập con, không cần đếm ký tự. Bài sau: gộp hai
tập hợp lại thành một — cả hai, không sót cái nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Luống 1 trồng {cà chua, xà lách}, luống 3 trồng {xà lách, cà rốt}. Gộp
CẢ HAI luống lại thành một tập — xà lách xuất hiện Ở CẢ HAI, vậy nó có
bị đếm hai lần trong tập kết quả không?
::::

::::checkpoint{mastery=0.8}
::::
