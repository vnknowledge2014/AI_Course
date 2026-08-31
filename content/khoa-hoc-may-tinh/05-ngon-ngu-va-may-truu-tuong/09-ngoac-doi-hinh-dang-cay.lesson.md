---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.ngoac-doi-hinh-dang-cay
title: "Ngoặc thay đổi hình dạng cây, không đổi token"
summary: "(2 + 3) * 4 và 2 + 3 * 4 cùng bốn số/toán tử, khác đúng một cặp ngoặc — nhưng ast.dump() lộ ra cây khác hẳn: Add giờ lồng bên trong Mult, đảo ngược hoàn toàn. Ngoặc không phải một token 'có nghĩa' — nó là chỉ dẫn CHO NGƯỜI DỰNG CÂY."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ngu.parens-change-tree]
requires: [ngu.precedence-is-tree-shape]
concepts: [ngu.parens-change-tree]
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
Không trang trí đâu. Nó ra lệnh thẳng cho hình dạng cây — và bạn sắp
thấy nó ra lệnh thế nào.
::::

::::explain{#ngoac-ra-lenh-cho-cay}
Bài "Token không đủ" đã chỉ ra một sự thật: dấu ngoặc `(` và `)` có
LOẠI token là `OP` — giống hệt `+`, `-`, `*`, `/`. Không có loại riêng
nào tên "NGOẶC" để phân biệt chúng với các toán tử khác. Vậy dấu ngoặc
LÀM GÌ, nếu bản thân nó chỉ là hai token `OP` bình thường?

Câu trả lời nằm ở CÂY, không nằm ở token. Khi `ast.parse()` đọc một
biểu thức, nó dùng cặp dấu ngoặc như một CHỈ DẪN cho chính NÓ — người
dựng cây — biết phải GỘP những gì lại thành một nhóm trước khi gộp
tiếp với phần còn lại. Và một khi cây đã dựng xong, cặp dấu ngoặc đó
**biến mất khỏi cây hoàn toàn**: không có nút nào tên `Paren` trong
`ast.dump()` cả. Ngoặc chỉ để LẠI DẤU VẾT bằng cách làm hình dạng cây
khác đi — nút nào lồng ở đâu — chứ bản thân nó không có mặt trong cây
kết quả, giống như giàn giáo được tháo đi sau khi công trình xây xong.

Nhớ lại bài trước: `2 + 3 * 4` có `Add` ở NÚT GỐC, `Mult` lồng bên
trong — vì `*` có độ ưu tiên cao hơn `+`. Thêm đúng một cặp ngoặc
quanh `2 + 3`, thành `(2 + 3) * 4`, KHÔNG đổi một chữ số hay một toán
tử nào — chỉ thêm hai token `OP`. Nhưng cặp ngoặc đó RA LỆNH cho phép
cộng phải được gộp lại và tính trước phép nhân — đảo ngược hoàn toàn
độ ưu tiên tự nhiên của `+` và `*`. Cây phải đổi hình dạng để phản ánh
đúng mệnh lệnh đó.
::::

::::example{#doi-chieu-hai-cay-ngoac}
Byte dựng cây cho cả hai, đặt cạnh nhau:

```python title=readonly
import ast

print(ast.dump(ast.parse("2 + 3 * 4", mode="eval")))
print(ast.dump(ast.parse("(2 + 3) * 4", mode="eval")))
```

```text title=readonly
Expression(body=BinOp(left=Constant(value=2), op=Add(), right=BinOp(left=Constant(value=3), op=Mult(), right=Constant(value=4))))
Expression(body=BinOp(left=BinOp(left=Constant(value=2), op=Add(), right=Constant(value=3)), op=Mult(), right=Constant(value=4)))
```

Dòng đầu: `Add` ở NGOÀI CÙNG, `Mult` lồng bên trong. Dòng sau (có
ngoặc): hoàn toàn lật ngược — `Mult` ở NGOÀI CÙNG, `Add` lồng bên
trong. Không có token `(` hay `)` nào xuất hiện trong `ast.dump()` cả —
chúng đã "tan biến" ngay khi hoàn thành nhiệm vụ, chỉ để lại dấu vết
bằng cách khiến `Add` chuyển từ vị trí GỐC sang vị trí LỒNG. Đúng hai
token thêm vào, và toàn bộ hình dạng cây đảo lộn.
::::

::::predict{#doan-cap-tru-ngoac commitOnce}
Cặp `10 - 3 - 2` (không ngoặc, giá trị `5`) và `10 - (3 - 2)` (có
ngoặc, giá trị `9`).

**Trước khi chạy `ast.dump()` đối chiếu**, bạn đoán: phép trừ nào —
`10 - 3` hay `3 - 2` — là phép trừ LỒNG (con của phép trừ kia) trong
MỖI biểu thức?

:::opt{correct}
Trong `10 - 3 - 2` (không ngoặc), `10 - 3` là phép trừ LỒNG (nhánh
trái); trong `10 - (3 - 2)` (có ngoặc), `3 - 2` mới là phép trừ LỒNG
(nhánh phải) — ngoặc đã đảo ngược hoàn toàn cặp nào lồng vào cặp nào
:::

:::opt
Cả hai biểu thức đều có `10 - 3` là phép trừ lồng — dấu ngoặc chỉ đổi
GIÁ TRỊ tính ra, không đổi CÂY
::why
Gần đúng ở việc bạn nhớ đúng: `10 - 3` đúng là phép trừ lồng trong
biểu thức KHÔNG NGOẶC — phần đó đúng.

Chỗ lệch chính là điều bài học này phủ nhận: dấu ngoặc THAY ĐỔI hình
dạng cây, không chỉ đổi giá trị tính ra. Trong `10 - (3 - 2)`, ngoặc
buộc `3 - 2` phải gộp lại và tính trước — nó mới là phép trừ lồng, còn
`10 - 3` không hề tồn tại như một cặp riêng trong cây đó nữa.
::
:::

:::opt
Cả hai biểu thức đều có `3 - 2` là phép trừ lồng, vì cứ có ngoặc là
đánh dấu phần lồng bên trong, bất kể ngoặc có thật sự xuất hiện hay
không
::why
Gần đúng ở việc bạn nắm đúng NGUYÊN TẮC: ngoặc đánh dấu phần lồng bên
trong — nguyên tắc đó áp dụng đúng cho biểu thức CÓ ngoặc.

Chỗ lệch: `10 - 3 - 2` KHÔNG có ngoặc nào cả. Không có ngoặc thì cây
dựng theo quy tắc đọc trái sang phải — `10 - 3` được gộp và tính TRƯỚC
(vì nó đứng trước), không phải `3 - 2`.
::
:::

:::opt
Không xác định được nếu không đếm số dấu ngoặc trong mỗi biểu thức
::why
Gần đúng ở sự thận trọng khi chưa chắc — thái độ đó không sai khi gặp
biểu thức lạ.

Chỗ lệch: ở đây không cần đếm gì thêm. Chỉ cần NHÌN xem biểu thức có
ngoặc hay không, và ngoặc bao quanh cặp số nào — nhìn thẳng vào
`10 - (3 - 2)` đã thấy ngay ngoặc bao quanh `3 - 2`, đủ để xác định
chắc chắn, không cần đếm số lượng dấu ngoặc.
::
:::
::::

::::code{#nhanh-long-doi-ben}
Cặp `khong_ngoac = "8 - 2 - 1"` (giá trị `5`) và
`co_ngoac = "8 - (2 - 1)"` (giá trị `7`). Điền hai chỗ trống: nhánh nào
(`.left` hay `.right`) của mỗi cây là `BinOp` LỒNG.

```python title=starter
import ast

khong_ngoac = ast.parse("8 - 2 - 1", mode="eval").body
co_ngoac = ast.parse("8 - (2 - 1)", mode="eval").body

nhanh_long_khong_ngoac = ___     # nhánh nào của khong_ngoac là BinOp lồng?
nhanh_long_co_ngoac = ___        # nhánh nào của co_ngoac là BinOp lồng?

la_binop_khong_ngoac = isinstance(nhanh_long_khong_ngoac, ast.BinOp)
la_binop_co_ngoac = isinstance(nhanh_long_co_ngoac, ast.BinOp)

gia_tri_khong_ngoac = 8 - 2 - 1
gia_tri_co_ngoac = 8 - (2 - 1)

print(la_binop_khong_ngoac)
print(la_binop_co_ngoac)
print(gia_tri_khong_ngoac, gia_tri_co_ngoac)
```

```python title=solution
import ast

khong_ngoac = ast.parse("8 - 2 - 1", mode="eval").body
co_ngoac = ast.parse("8 - (2 - 1)", mode="eval").body

nhanh_long_khong_ngoac = khong_ngoac.left
nhanh_long_co_ngoac = co_ngoac.right

la_binop_khong_ngoac = isinstance(nhanh_long_khong_ngoac, ast.BinOp)
la_binop_co_ngoac = isinstance(nhanh_long_co_ngoac, ast.BinOp)

gia_tri_khong_ngoac = 8 - 2 - 1
gia_tri_co_ngoac = 8 - (2 - 1)

print(la_binop_khong_ngoac)
print(la_binop_co_ngoac)
print(gia_tri_khong_ngoac, gia_tri_co_ngoac)
```

```python title=test
assert la_binop_khong_ngoac == True, "nhánh lồng của khong_ngoac phải là một BinOp thật"
assert nhanh_long_khong_ngoac.left.value == 8 and nhanh_long_khong_ngoac.right.value == 2, f"trong '8 - 2 - 1' (không ngoặc), phép trừ LỒNG phải chính là '8 - 2' (nhánh TRÁI) — đang thấy {ast.dump(nhanh_long_khong_ngoac)}"
assert la_binop_co_ngoac == True, "nhánh lồng của co_ngoac phải là một BinOp thật"
assert nhanh_long_co_ngoac.left.value == 2 and nhanh_long_co_ngoac.right.value == 1, f"trong '8 - (2 - 1)' (có ngoặc), phép trừ LỒNG phải chính là '2 - 1' (nhánh PHẢI) — đang thấy {ast.dump(nhanh_long_co_ngoac)}"
assert (gia_tri_khong_ngoac, gia_tri_co_ngoac) == (5, 7), f"8-2-1 phải là 5, 8-(2-1) phải là 7 — đang ra {(gia_tri_khong_ngoac, gia_tri_co_ngoac)}"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống chỉ đọc một thuộc tính có sẵn — .left hoặc .right — của khong_ngoac/co_ngoac, không gọi hàm nào cả.
- kind: strategy
  body: 'Không có ngoặc, "8 - 2 - 1" gộp trái trước (đọc trái sang phải): phép lồng là "8 - 2", nằm ở khong_ngoac.left. Có ngoặc, "8 - (2 - 1)" buộc "2 - 1" gộp trước: phép lồng là "2 - 1", nằm ở co_ngoac.right.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là: khong_ngoac.left và co_ngoac.right'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ đọc thuộc tính .left/.right từ khong_ngoac/co_ngoac — điền một câu không làm gì (như True, 1, 0) không phải một BinOp thật, và test isinstance(..., ast.BinOp) sẽ bắt được ngay
  requireAst:
  # min: 1 mỗi cái — đếm thật trên solution: "khong_ngoac" bị ĐỌC (Load)
  # đúng 1 lần (trong chỗ trống thứ nhất — không có chỗ nào khác trong cả
  # bài đọc lại tên này); "co_ngoac" bị ĐỌC đúng 1 lần tương tự. Các dòng
  # gán ban đầu chỉ Store, không tính. ĐÃ THỬ THẬT: điền True/1/0 vào CẢ
  # HAI chỗ trống — không vòng lặp nào trong bài này nên cả ba dừng ngay,
  # không treo; cả ba cho la_binop_khong_ngoac/la_binop_co_ngoac là False
  # (True/1/0 không phải ast.BinOp), test bắt được độc lập; và không còn
  # đọc "khong_ngoac"/"co_ngoac" nào — luật static cũng bắt được. Thử
  # riêng TỪNG chỗ trống một (chỗ kia giữ đúng) bằng True/1/0: cùng kết
  # quả ở đúng phía bị điền hụt.
  - kind: uses-name, target: khong_ngoac, min: 1
  - kind: uses-name, target: co_ngoac, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^True\\nTrue\\n5 7\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đúng hai token thêm vào, và cả hai phép trừ đổi chỗ cho nhau trong
cây. Ngoặc không trang trí — nó ra lệnh.
::::

::::reflect{#nghi-lai}
Một câu hỏi khép lại cả cụm bài về cây cú pháp.

Bốn bài qua, bạn luôn XEM máy dựng cây rồi mới đối chiếu — `ast.dump()`
luôn chạy TRƯỚC, bạn đọc kết quả SAU. Giống hệt bài "Tokenize tay" đã
làm với token: dự đoán TRƯỚC khi chạy máy, để tự kiểm mình đã thật sự
nắm được luật hay chưa. Với một biểu thức có ngoặc bạn CHƯA từng chạy
thử, bạn có tự vẽ (hoặc mô tả) được hình dạng cây của nó TRƯỚC, rồi mới
đối chiếu với `ast.dump()` thật không?

Bài sau thử đúng việc đó.
::::

::::checkpoint{mastery=0.8}
::::
