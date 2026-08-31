---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.cay-cu-phap
title: "Cây cú pháp: mỗi nút là một mảnh ý nghĩa"
summary: "ast.parse('2 + 3 * 4', mode='eval') dựng ra một CÂY thật — nút cha, nút con, đúng cấu trúc phần Cấu trúc dữ liệu đã dạy — không còn là một dãy token phẳng. Đây là AST, đúng công cụ mà bộ chấm bài của khoá này cũng dùng."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ngu.ast]
requires: [ngu.tokens-need-structure, ds.tree]
concepts: [ngu.ast]
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
Không phải tự viết đâu. Python đã có sẵn một bộ dựng cây — và bạn sắp
nhìn thấy đúng cái cây nó dựng ra.
::::

::::explain{#module-ast}
Python có sẵn một module tên `ast` (viết tắt của Abstract Syntax Tree —
**cây cú pháp trừu tượng**). Hàm `ast.parse(nguon, mode="eval")` nhận
một chuỗi chứa MỘT biểu thức, và trả về không phải một dãy token phẳng
— mà một CÂY thật.

Nhớ lại cây đã học ở phần Cấu trúc dữ liệu: một nút giữ một giá trị,
và có thể có những nút CON, mỗi nút con lại có thể có nút con riêng của
nó. `ast.parse(...)` dựng đúng cấu trúc đó cho một biểu thức Python.
Kết quả trả về là một object `Expression`, có thuộc tính `.body` là
**nút gốc** của cây — với `2 + 3 * 4`, nút gốc đó là một `BinOp` (phép
toán hai ngôi), giữ ba thứ: `.left` (nhánh trái), `.op` (loại toán tử),
`.right` (nhánh phải). Mỗi nhánh, tới lượt nó, lại có thể là một nút
LÁ (một con số, gọi là `Constant`) hoặc một nút TRONG khác (một
`BinOp` khác, lồng bên trong) — đúng khái niệm "cây" bạn đã quen: nút
cha, nút con, và một nút con cũng có thể làm cha của những nút khác.

Có một điều đáng nói thẳng ra: `ast.parse()` không phải một công cụ
riêng của khoá này bịa ra để minh hoạ. Chính bộ chấm bài của khoá học —
hàm `kiemAst()` chấm mọi bài tập `code` bạn từng làm — dùng ĐÚNG hàm
`ast.parse()` này để đọc lời giải của bạn thành một cây, rồi kiểm cây
đó có đúng hình dạng bài yêu cầu không. Cây bạn sắp thấy hôm nay là
đúng thứ đã âm thầm chấm điểm bạn từ bài đầu tiên.
::::

::::example{#dung-cay-that}
Byte dựng cây cho `2 + 3 * 4`, rồi in ra bằng `ast.dump(..., indent=2)`
— một cách in nhiều dòng, dễ nhìn cấu trúc lồng:

```python title=readonly
import ast

cay = ast.parse("2 + 3 * 4", mode="eval")
print(ast.dump(cay, indent=2))
```

```text title=readonly
Expression(
  body=BinOp(
    left=Constant(value=2),
    op=Add(),
    right=BinOp(
      left=Constant(value=3),
      op=Mult(),
      right=Constant(value=4))))
```

Đọc từ ngoài vào trong: `Expression` là lớp vỏ ngoài cùng (luôn có mặt
khi dùng `mode="eval"`). Bên trong nó, `body=BinOp(...)` là NÚT GỐC —
một phép toán hai ngôi. Nút gốc đó có `left=Constant(value=2)` (một nút
LÁ — chỉ là số `2`, không có con nào), `op=Add()` (toán tử của chính nó
là phép cộng), và `right=BinOp(...)` — nhánh phải KHÔNG phải một số
đơn, mà là một `BinOp` KHÁC, lồng bên trong, với `left=Constant(3)`,
`op=Mult()`, `right=Constant(4)`.

Đây chính là điều bài trước còn thiếu: một dãy token phẳng không nói
được "3 và 4 phải gộp lại trước". Cây thì nói được — `3` và `4` cùng
nằm chung trong MỘT nút `BinOp` con, tách biệt hẳn khỏi số `2` ở ngoài.
::::

::::predict{#doan-khong-long commitOnce}
Biểu thức chỉ có MỘT toán tử: `9 - 4`.

**Trước khi chạy `ast.dump()`**, bạn đoán: cây của biểu thức này có một
nút `BinOp` nào LỒNG bên trong một nút `BinOp` khác không?

:::opt{correct}
Không — chỉ có đúng MỘT nút `BinOp` duy nhất; cả nhánh trái (`9`) lẫn
nhánh phải (`4`) đều là `Constant`, hai nút LÁ, không nút nào lồng bên
trong nút nào
:::

:::opt
Có — nhánh phải (`4`) là một `BinOp` lồng, giống hệt cấu trúc của
`2 + 3 * 4` vừa xem
::why
Gần đúng ở việc bạn nhớ đúng HÌNH DẠNG cây của `2 + 3 * 4` — nhánh phải
lồng một `BinOp` khác, điều đó có thật ở VÍ DỤ ĐÓ.

Chỗ lệch: `9 - 4` chỉ có ĐÚNG MỘT toán tử (`-`), không có toán tử thứ
hai nào để tạo ra một `BinOp` thứ hai cả. `2 + 3 * 4` lồng được vì nó
có HAI toán tử (`+` và `*`); một biểu thức chỉ một toán tử luôn cho ra
đúng một `BinOp`, không hơn.
::
:::

:::opt
Có — nhánh trái (`9`) là một `BinOp` lồng, vì số đứng trước luôn được
tính trước
::why
Gần đúng ở việc "số đứng trước" (`9`) đúng là được ĐỌC trước trong
biểu thức — quan sát về thứ tự đọc không sai.

Chỗ lệch: đọc trước không có nghĩa là "trở thành một `BinOp` lồng".
`9` chỉ là một con số đơn, không có toán tử nào áp lên riêng nó — nó là
nút LÁ (`Constant`), không phải `BinOp`. Chỉ khi có ÍT NHẤT hai toán tử
trong biểu thức mới có chỗ cho một `BinOp` lồng bên trong `BinOp` khác.
::
:::

:::opt
Không xác định được nếu không đếm số ký tự trong biểu thức
::why
Gần đúng ở sự thận trọng — cẩn thận trước khi đoán chắc là thái độ
đúng.

Chỗ lệch: không cần đếm KÝ TỰ. Đếm số TOÁN TỬ là đủ — `9 - 4` chỉ có
một dấu `-`, nghĩa là chỉ có một phép tính, nghĩa là chỉ có đúng một
`BinOp`. Không có phép tính thứ hai thì không có `BinOp` thứ hai để
lồng vào đâu cả.
::
:::
::::

::::code{#doc-cay-that}
Cây của `2 + 3 * 4`, đã dựng sẵn thành `cay`. Điền hai chỗ trống: LOẠI
của nhánh phải (`.right`), và LOẠI toán tử bên trong nhánh lồng đó.

```python title=starter
import ast

cay = ast.parse("2 + 3 * 4", mode="eval").body

loai_nhanh_trai = type(cay.left).__name__
loai_nhanh_phai = ___                       # LOẠI của cay.right — dùng type(...).__name__

nhanh_long = cay.right
loai_toan_tu_long = ___                     # LOẠI toán tử bên trong nhanh_long — type(nhanh_long.op).__name__

print(loai_nhanh_trai)
print(loai_nhanh_phai)
print(loai_toan_tu_long)
```

```python title=solution
import ast

cay = ast.parse("2 + 3 * 4", mode="eval").body

loai_nhanh_trai = type(cay.left).__name__
loai_nhanh_phai = type(cay.right).__name__

nhanh_long = cay.right
loai_toan_tu_long = type(nhanh_long.op).__name__

print(loai_nhanh_trai)
print(loai_nhanh_phai)
print(loai_toan_tu_long)
```

```python title=test
assert loai_nhanh_trai == "Constant", f"nhánh TRÁI của cay (số 2) phải là một nút LÁ, loại Constant — đang ra {loai_nhanh_trai!r}"
assert loai_nhanh_phai == "BinOp", f"nhánh PHẢI của cay (3 * 4) phải là một BinOp LỒNG bên trong — đang ra {loai_nhanh_phai!r}"
assert loai_toan_tu_long == "Mult", f"toán tử bên trong nhánh lồng phải là Mult — đang ra {loai_toan_tu_long!r}"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống dùng đúng một khuôn — type(mot_nut).__name__ — giống hệt cách dòng loai_nhanh_trai đã làm sẵn ở trên với cay.left.
- kind: strategy
  body: 'Chỗ trống thứ nhất hỏi LOẠI của cay.right — viết type(cay.right).__name__. Chỗ trống thứ hai hỏi LOẠI của toán tử bên trong nhanh_long (biến đã gán = cay.right ở dòng trên) — viết type(nhanh_long.op).__name__.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là: type(cay.right).__name__ và type(nhanh_long.op).__name__'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống thứ hai (loai_toan_tu_long) phải THẬT SỰ đọc từ nhanh_long — điền một câu không làm gì (như True, 1, 0) sẽ không cho ra chuỗi "Mult", và test sẽ bắt được ngay
  requireAst:
  # min: 1 — đếm thật trên solution: tên "nhanh_long" bị ĐỌC (Load) đúng
  # 1 lần trong toàn bài — bên trong chỗ trống thứ hai
  # (type(nhanh_long.op).__name__). Dòng "nhanh_long = cay.right" chỉ GÁN
  # (Store), uses-name không đếm chỗ gán. ĐÃ THỬ THẬT: điền True/1/0 vào
  # CẢ HAI chỗ trống — không vòng lặp nào trong bài này nên cả ba dừng
  # ngay, không treo; cả ba cho loai_nhanh_phai và loai_toan_tu_long là
  # 'bool'/'int' thay vì 'BinOp'/'Mult', test bắt được độc lập; và vì
  # không còn đọc "nhanh_long" nào cả, luật static này cũng bắt được. Thử
  # riêng CHỈ chỗ trống thứ hai bằng True/1/0 (chỗ trống thứ nhất giữ
  # đúng): "nhanh_long" cũng mất hẳn khỏi mã nguồn — luật vẫn bắt được.
  - kind: uses-name, target: nhanh_long, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Constant\\nBinOp\\nMult\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không còn một hàng ngang các miếng nữa — một cây thật, nút cha ôm lấy
nút con, đúng cấu trúc bạn đã học từ trước.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`3 * 4` nằm SÂU HƠN `2` trong cây vừa dựng — không phải tình cờ, cây
lúc nào cũng dựng đúng như vậy cho `2 + 3 * 4`. Điều đó có liên quan gì
tới luật "nhân trước, cộng sau" mà bạn học thuộc lòng từ hồi nhỏ không?
Hay đó chỉ là hai chuyện trùng hợp, không dính dáng gì tới nhau?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
