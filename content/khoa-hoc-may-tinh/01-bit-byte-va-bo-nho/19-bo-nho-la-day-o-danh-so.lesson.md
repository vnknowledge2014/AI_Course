---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.bo-nho-la-day-o-danh-so
title: "Bộ nhớ là một dãy ô có đánh số"
summary: "Mọi ô nhớ trong máy có một số thứ tự riêng gọi là địa chỉ — và lệnh id() trong Python để bạn đọc con số ấy ra, ngay trên chương trình đang chạy."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [mem.address]
requires: [mem.byte-range, core.list, core.variable, core.assignment, core.builtin-function, core.fstring, ctrl.comparison, core.boolean]
concepts: [mem.address]
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
Ba byte của chữ `ở` phải nằm ở một chỗ cụ thể, không lơ lửng. Chỗ cụ thể
đó gọi tên được — bằng một con số.
::::

::::explain{#pho-dai-danh-nha}
Bài 4 đã cho bạn hình dung một ô nhớ: một byte, chứa được một số từ 0 tới
255. Nhưng bài đó chỉ nói tới MỘT ô, như thể nó đứng lẻ loi một mình.

Thật ra bộ nhớ của máy là **hàng tỉ** ô như vậy, xếp thành một dãy dài
liền mạch — hình dung như một con phố cực dài, mỗi nhà một ô. Đầu phố là
nhà số 0, kế đó là nhà số 1, rồi 2, rồi 3, cứ thế đếm tới hết dãy.

Con số thứ tự của một ô — số nhà của nó trên con phố ấy — gọi là **địa
chỉ**. Mỗi ô nhớ mang theo hai thứ khác hẳn nhau: **giá trị** nó đang chứa
(một số từ 0 tới 255, như bài 4 đã nói), và **địa chỉ** nơi nó đứng (số
thứ tự của chính nó trên dãy).

Ba byte của chữ `ở` — 225, 187, 159 — mỗi byte nằm ở một địa chỉ riêng,
và ba địa chỉ đó liền kề nhau: byte đầu ở địa chỉ nào thì byte kế nằm ngay
địa chỉ tiếp theo. Máy tìm lại đúng cả ba bằng cách nhớ đúng MỘT con số —
địa chỉ của byte đầu tiên — rồi biết hai byte kia nằm ngay sau đó.

Và địa chỉ, xét cho cùng, cũng chỉ là một con số — y như mọi thứ khác bạn
đã gặp trong track này.
::::

::::example{#id-doc-dia-chi}
Python cho bạn đọc thẳng con số ấy bằng một lệnh có sẵn: `id(...)`. Đưa
vào nó bất kỳ giá trị nào, nó trả lại địa chỉ nơi giá trị ấy đang nằm
trong bộ nhớ — trên đúng phiên bản Python mà khoá này chạy, con số đó
CHÍNH LÀ địa chỉ thật.

Thử với hai giỏ hạt giống ở vườn rau, nội dung giống hệt nhau:

```python title=readonly
gio_1 = [10, 20, 5]
gio_2 = [10, 20, 5]

print(gio_1 == gio_2)
print(id(gio_1))
print(id(gio_2))
print(id(gio_1) == id(gio_2))
```

Máy in ra (hai con số địa chỉ cụ thể sẽ khác mỗi lần chạy, nhưng luôn khác
nhau giữa `gio_1` và `gio_2`):

```text title=readonly
True
139879183651136
139879183648960
False
```

Dòng đầu và dòng cuối là chỗ đáng nhìn kỹ, vì chúng nói hai chuyện khác
hẳn nhau. `gio_1 == gio_2` hỏi "hai giỏ có cùng NỘI DUNG không" — cùng ba
con số 10, 20, 5, nên trả lời `True`. Còn `id(gio_1) == id(gio_2)` hỏi
"hai giỏ có cùng CHỖ ĐỂ không" — và câu trả lời là `False`, vì `gio_1` với
`gio_2` là hai lệnh tạo danh sách riêng biệt, nên máy dựng ra hai giỏ nằm
ở hai địa chỉ khác nhau, dù bên trong hai giỏ ấy đựng y hệt nhau.

Nội dung giống nhau không có nghĩa là cùng một chỗ.
::::

::::predict{#doan-hai-danh-sach commitOnce}
Byte dựng thêm hai danh sách khác, cũng giống hệt nhau về nội dung.

**Trước khi bấm chạy**, bạn đoán hai dòng dưới in ra gì?

```python
a = [1, 2, 3]
b = [1, 2, 3]
print(a == b)
print(id(a) == id(b))
```

:::opt{correct}
`True`, rồi `False`
:::

:::opt
`True`, rồi `True`
::why
Gần đúng ở dòng đầu: `a` và `b` đúng là bằng nhau về nội dung, cả hai đều
là `[1, 2, 3]`.

Chỗ lệch là bạn để dòng đầu "kéo theo" dòng sau, như thể nội dung giống
nhau thì chỗ để cũng giống nhau luôn. Hai dòng lệnh `a = [1, 2, 3]` và
`b = [1, 2, 3]` là hai lệnh TẠO DANH SÁCH riêng biệt — mỗi lệnh dựng ra
một giỏ mới toanh, nằm ở một chỗ mới toanh. Giống hệt nội dung, khác hẳn
địa chỉ.
::
:::

:::opt
`False`, rồi `False`
::why
Gần đúng ở chỗ bạn đoán đúng dòng sau — hai danh sách quả thật nằm ở hai
địa chỉ khác nhau.

Chỗ lệch nằm ở dòng đầu. `==` giữa hai danh sách so sánh TỪNG PHẦN TỬ một,
không quan tâm chúng nằm ở đâu. `[1, 2, 3]` so với `[1, 2, 3]`: phần tử
đầu bằng phần tử đầu, phần tử hai bằng phần tử hai, phần tử ba bằng phần
tử ba — khớp hết, nên `True`.
::
:::

:::opt
Máy báo lỗi, vì hai cái tên `a` và `b` không được trỏ tới cùng nội dung
::why
Gần đúng ở chỗ bạn cảm thấy có gì đó "trùng" đáng ngờ giữa `a` và `b` —
đúng là chúng trùng nội dung thật.

Chỗ lệch là Python không cấm điều đó. Hai cái tên trỏ tới hai giá trị
giống hệt nhau về nội dung là chuyện hoàn toàn hợp lệ, chạy trơn tru,
không một tiếng báo lỗi nào. Không có luật nào nói "hai danh sách không
được giống nhau".
::
:::
::::

::::code{#hai-gio-hat-giong}
Vườn rau có hai giỏ hạt giống, `gio_1` và `gio_2`, cùng đựng ba loại hạt
với số lượng y hệt nhau. Byte muốn kiểm tra hai điều cùng lúc: nội dung có
giống nhau không, và chúng có phải CHUNG một giỏ không.

Dùng `id(...)` để lấy con số địa chỉ của mỗi giỏ, rồi so sánh hai con số
đó — đừng so sánh bằng cách khác.

```python title=starter
gio_1 = [10, 20, 5]
gio_2 = [10, 20, 5]

noi_dung_giong_nhau = gio_1 == gio_2
dia_chi_giong_nhau = ___

print(f"Nội dung giống nhau: {noi_dung_giong_nhau}")
print(f"Địa chỉ giống nhau: {dia_chi_giong_nhau}")
```

```python title=solution
gio_1 = [10, 20, 5]
gio_2 = [10, 20, 5]

noi_dung_giong_nhau = gio_1 == gio_2
dia_chi_giong_nhau = id(gio_1) == id(gio_2)

print(f"Nội dung giống nhau: {noi_dung_giong_nhau}")
print(f"Địa chỉ giống nhau: {dia_chi_giong_nhau}")
```

```python title=test
assert gio_1 == [10, 20, 5] and gio_2 == [10, 20, 5], "hai giỏ không được sửa nội dung — bài này chỉ ĐỌC, không sửa"
assert noi_dung_giong_nhau is True, "hai giỏ cùng đựng [10, 20, 5], nên so sánh bằng == phải ra True"
assert dia_chi_giong_nhau is False, "gio_1 và gio_2 là hai lệnh tạo danh sách riêng biệt, nên chúng nằm ở hai địa chỉ khác nhau — so sánh bằng id(...) phải ra False"
```

:::hints
- kind: attention
  body: Chỗ trống phải trả lời câu hỏi "hai giỏ có CHUNG một chỗ để không" — khác hẳn dòng ngay trên, vốn hỏi "hai giỏ có chung NỘI DUNG không". Bài này vừa cho bạn công cụ đọc địa chỉ; chỗ trống là nơi dùng nó.
- kind: strategy
  body: Lấy địa chỉ của từng giỏ bằng id(gio_1) và id(gio_2), rồi so sánh hai con số đó bằng ==. Đừng so sánh gio_1 == gio_2 lần nữa ở đây — dòng trên đã hỏi câu đó rồi, và câu bài này cần hỏi là một câu khác.
- kind: one-line
  body: "Điền `id(gio_1) == id(gio_2)` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải thật sự gọi id(...) trên cả hai giỏ gio_1 và gio_2 rồi so sánh hai con số địa chỉ đó, không phải so sánh nội dung, gõ thẳng True/False, hay gọi id trên thứ khác
  requireAst:
  - kind: uses-call, target: id, min: 2
  - kind: uses-name, target: gio_1, min: 2
  - kind: uses-name, target: gio_2, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Nội dung giống nhau: True\nĐịa chỉ giống nhau: False\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng nội dung, khác địa chỉ. Bây giờ "chỗ để" không còn là chuyện mơ hồ
nữa — nó là một con số bạn đọc được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Địa chỉ là một con số, và mỗi giá trị bạn tạo ra đều có một địa chỉ riêng.
Nhưng cái TÊN bạn đặt cho giá trị đó — như `gio_1` — nó có PHẢI LÀ cái giỏ
hay không? Hay nó chỉ đang giữ con số địa chỉ của cái giỏ, và bản thân cái
tên với cái giỏ là hai thứ khác hẳn nhau?

Đừng trả lời vội. Bài sau nói thẳng ra.
::::

::::checkpoint{mastery=0.8}
::::
