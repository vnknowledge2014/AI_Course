---
id: toan.tap-hop-quan-he-anh-xa.tap-vu-tru-va-phan-bu
title: Tập vũ trụ và phần bù
summary: "Tập vũ trụ U (mọi loại rau CÓ TRONG toàn vườn) và phần bù Aᶜ = U − A. Phần bù chỉ có nghĩa KHI đã chốt vũ trụ đang xét — đổi vũ trụ thì phần bù đổi theo, dù A không đổi một chút nào."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.set-complement]
requires: [math.set-difference]
concepts: [math.tap-vu-tru, math.phan-bu]
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
Cả vườn Byte trồng đúng bảy loại rau. "Phần bù của luống 1" — những
loại rau CẢ VƯỜN có mà luống 1 KHÔNG trồng. Đó có phải MỘT phép `−`
nào đó không?
::::

::::explain{#vu-tru-va-phan-bu}
Đúng, NHƯNG cần thêm MỘT thứ trước: **tập vũ trụ `U`** — TOÀN BỘ
"thế giới" đang xét (Ở đây LÀ mọi loại rau CÓ TRONG cả vườn Byte).
**Phần bù `Aᶜ = U − A`** — mọi thứ THUỘC vũ trụ mà KHÔNG thuộc A.

```python title=readonly
U = {"cà chua", "xà lách", "cà rốt", "bí đỏ", "khoai lang", "cải bó xôi", "đậu que"}
luong_1 = {"cà chua", "xà lách"}

print(sorted(U - luong_1))
print(len(U - luong_1))
```

```text title=readonly
['bí đỏ', 'cà rốt', 'cải bó xôi', 'khoai lang', 'đậu que']
5
```

`luống_1ᶜ` (phần bù của luống 1, trong VŨ TRỤ `U` bảy loại) LÀ năm
loại rau CÒN LẠI — mọi thứ vườn Byte CÓ mà luống 1 KHÔNG có. Đây LÀ
phép `−` (bài 11), CHỈ khác Ở CHỖ vế TRÁI LUÔN LÀ `U`.
::::

::::example{#doi-vu-tru-doi-phan-bu}
Đổi `U` — CÙNG một `luống_1`, phần bù RA khác hẳn:

```python title=readonly
U_lan = {"cà chua", "xà lách", "cà rốt", "bí đỏ", "khoai lang", "cải bó xôi", "đậu que", "ớt", "hành"}
luong_1 = {"cà chua", "xà lách"}

print(sorted(U_lan - luong_1))
```

```text title=readonly
['bí đỏ', 'cà rốt', 'cải bó xôi', 'hành', 'khoai lang', 'đậu que', 'ớt']
```

Vườn Lan trồng CHÍN loại (nhiều hơn vườn Byte hai loại: `ớt`, `hành`)
— phần bù của `luống_1` trong vũ trụ MỚI ĐÓ có tới BẢY loại (không
phải năm như Ở TRÊN), DÙ `luống_1` KHÔNG hề đổi một CHÚT nào. Phần bù
KHÔNG PHẢI tính chất của riêng `A` — nó phụ thuộc CẢ vào `U`.
::::

::::predict{#doan-doi-vu-tru commitOnce}
Byte so sánh trực tiếp hai phần bù (Ở vũ trụ vườn Byte VÀ vũ trụ vườn
Lan) của CÙNG một `luống_1`:

```python
U = {"cà chua", "xà lách", "cà rốt", "bí đỏ", "khoai lang", "cải bó xôi", "đậu que"}
U_lan = {"cà chua", "xà lách", "cà rốt", "bí đỏ", "khoai lang", "cải bó xôi", "đậu que", "ớt", "hành"}
luong_1 = {"cà chua", "xà lách"}

print(len(U - luong_1) == len(U_lan - luong_1))
print((U - luong_1) == (U_lan - luong_1))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`False`, rồi `False`
:::

:::opt
`False`, rồi `True` — vì DÙ số LƯỢNG (`len`) hai phần bù khác nhau
(năm so BẢY), nội DUNG "cốt lõi" của chúng (loại rau vườn Byte VỐN CÓ
mà `luống_1` thiếu) VẪN LÀ MỘT tập con CHUNG bên trong, nên so `==`
TRỰC TIẾP hai TẬP hợp có thể VẪN khớp Ở PHẦN chung đó
::why
Gần đúng ở việc bạn để ý HAI phần bù CÓ chung MỘT lõi (năm loại của
vườn Byte) — một quan sát đúng về QUAN HỆ tập con giữa chúng.

Chỗ lệch: `==` giữa HAI tập hợp đòi hỏi chúng phải BẰNG NHAU HOÀN
TOÀN (bài 7: CẢ HAI chiều ⊆), KHÔNG PHẢI "có chung MỘT phần". Phần bù
Ở vũ trụ Lan CÓ THÊM `ớt`/`hành` mà phần bù Ở vũ trụ Byte KHÔNG CÓ —
nên `U − luống_1` LÀ tập CON THỰC SỰ của `U_lan − luống_1`, hai tập
KHÁC nhau, `==` cho `False`.
::
:::

:::opt
Máy báo lỗi biên dịch — so sánh ĐỘ DÀI (`len`) của HAI phép trừ tập
hợp khác vũ trụ (`U` VÀ `U_lan`) là một phép so sánh KHÔNG hợp lệ,
Python yêu CẦU hai vế `==` phải xuất phát TỪ CÙNG một tập vũ trụ
::why
Gần đúng ở việc bạn để ý `U` VÀ `U_lan` LÀ hai vũ trụ KHÁC nhau — một
quan sát đúng về NGỮ CẢNH toán học của phần bù.

Chỗ lệch: Python (VÀ TOÁN học nói CHUNG) KHÔNG hề "gắn nhãn vũ trụ"
vào một tập hợp — MỘT khi `U − luống_1` đã TÍNH xong, kết quả CHỈ LÀ
một `set` bình THƯỜNG, y hệt BẤT KỲ `set` nào khác. So SÁNH `len()`
HAY `==` giữa hai `set` BẤT KỲ LUÔN hợp lệ. Biên dịch sạch.
::
:::
::::

::::code{#viet_phan_bu}
Viết `phan_bu(a, u)` — trả về phần bù của `a` TRONG vũ trụ `u`.

```python title=starter
def phan_bu(a, u):
    return ___


vuon_U = {"cà chua", "xà lách", "cà rốt", "bí đỏ"}

print(sorted(phan_bu({"cà chua"}, vuon_U)))
```

```python title=solution
def phan_bu(a, u):
    return u - a


vuon_U = {"cà chua", "xà lách", "cà rốt", "bí đỏ"}

print(sorted(phan_bu({"cà chua"}, vuon_U)))
```

```python title=test
assert phan_bu(vuon_U, vuon_U) == set(), "phan bu cua CA vu tru la tap rong"
assert phan_bu(set(), vuon_U) == vuon_U, "phan bu cua tap rong la CA vu tru"
assert phan_bu({"cà chua", "xà lách"}, vuon_U) == {"cà rốt", "bí đỏ"}, "phan bu dung hai loai con lai"
```

:::hints
- kind: attention
  body: "Phan bu = vu tru TRU di a -- dung dung thu tu: u truoc, a sau."
- kind: strategy
  body: "u - a"
- kind: one-line
  body: "___ = u - a"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phan bu phai la u - a (vu tru TRU di a), khong phai a - u -- doi cho se ra ket qua sai (hieu khong doi xung, da hoc bai 11)
  requireAst:
  - kind: uses-name, target: a, min: 1
  - kind: uses-name, target: u, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\['bí đỏ', 'cà rốt', 'xà lách'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Phần bù phụ thuộc CẢ vũ trụ, không riêng tập hợp gốc. Bài chốt cụm:
đếm phần tử của một tập hợp — không đếm hai lần phần chung.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Luống 1 có 2 loại, luống 3 có 2 loại, giao nhau 1 loại. Cộng thẳng
`2 + 2` được 4 — nhưng hợp của chúng chỉ có 3 loại (bài 8). Chỗ dư ra
đi đâu?
::::

::::checkpoint{mastery=0.8}
::::
