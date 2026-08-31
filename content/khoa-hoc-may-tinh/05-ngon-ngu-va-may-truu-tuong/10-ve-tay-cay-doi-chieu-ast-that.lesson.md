---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.ve-tay-cay-doi-chieu-ast-that
title: "Vẽ tay một cây, đối chiếu với ast.dump() thật"
summary: "Bài chốt cụm: cho một biểu thức có ngoặc, tự viết ra dự đoán hình dạng cây TRƯỚC khi chạy ast.dump() thật — rồi đối chiếu. Không đoán suông; máy thật là trọng tài, đúng cách bài chốt cụm token hoá đã làm."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ngu.hand-draw-tree]
requires: [ngu.parens-change-tree]
concepts: [ngu.hand-draw-tree]
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
Bốn bài để hiểu cây cú pháp bằng cách XEM máy dựng. Hôm nay đảo lại:
bạn dự đoán TRƯỚC, máy chỉ đối chiếu dự đoán của bạn thôi.
::::

::::explain{#doan-truoc-doi-chieu-sau}
Cụm bài này đi đúng bốn bước: một dãy token phẳng không đủ (bài 6),
`ast.parse()` dựng ra một cây thật (bài 7), độ ưu tiên phép toán chính
là độ SÂU trong cây đó (bài 8), và ngoặc thay đổi hẳn hình dạng cây dù
không đổi token (bài 9). Bài chốt cụm hôm nay không thêm khái niệm
mới — nó bắt bạn LÀM đúng bốn bước đó bằng chính đầu mình, trước khi
hỏi máy, đúng cách bài "Tokenize tay" đã làm chốt cụm token hoá.

Cách làm: cho một biểu thức có ngoặc, tự viết ra dự đoán hình dạng cây
— dưới dạng một CHUỖI, đúng khuôn `ast.dump()` compact (không `indent`)
— TRƯỚC khi chạy `ast.dump()` thật. Rồi so hai chuỗi: của bạn, và của
máy. Không đoán suông rồi bỏ qua — máy thật luôn là trọng tài cuối
cùng, đúng tinh thần suốt cụm bài này.

Một cái bẫy hay gặp khi tự viết tay: quên mất lớp `Expression(body=…)`
bọc ngoài cùng — bài 7 đã nói `mode="eval"` luôn thêm lớp vỏ đó. Ví dụ
dưới đây cho thấy đúng cái bẫy đó, giống hệt cách bài "Tokenize tay"
từng quên mất `NEWLINE`/`ENDMARKER`.
::::

::::example{#doan-tay-quen-vo-ngoai}
Byte tự đoán tay cây của `3 * (4 + 1)`, viết ra TRƯỚC khi chạy máy:

```text title=readonly
Dự đoán của Byte: BinOp(left=Constant(value=3), op=Mult(), right=BinOp(left=Constant(value=4), op=Add(), right=Constant(value=1)))
```

Rồi chạy thật:

```python title=readonly
import ast

print(ast.dump(ast.parse("3 * (4 + 1)", mode="eval")))
```

```text title=readonly
Expression(body=BinOp(left=Constant(value=3), op=Mult(), right=BinOp(left=Constant(value=4), op=Add(), right=Constant(value=1))))
```

Phần BÊN TRONG khớp đúng dự đoán của Byte: `Mult` ở ngoài (vì phần
`4 + 1` bị ngoặc buộc phải gộp và tính trước, nên `Mult` mới là toán
tử áp dụng SAU CÙNG, đúng như bài "Độ ưu tiên" đã dạy — toán tử áp
dụng sau cùng luôn là toán tử NGOÀI CÙNG). Nhưng máy còn bọc thêm một
lớp `Expression(body=...)` ở ngoài cùng — lớp mà Byte quên mất, vì nó
không phải một PHÉP TOÁN, chỉ là cái vỏ mà `mode="eval"` luôn thêm vào.
::::

::::predict{#doan-cay-truoc commitOnce}
Biểu thức `(1 + 5) * 7`.

**Trước khi chạy máy**, bạn đoán: bản `ast.dump()` (compact, không
`indent`) đầy đủ của biểu thức này là chuỗi nào?

:::opt{correct}
`Expression(body=BinOp(left=BinOp(left=Constant(value=1), op=Add(), right=Constant(value=5)), op=Mult(), right=Constant(value=7)))`
:::

:::opt
`BinOp(left=BinOp(left=Constant(value=1), op=Add(), right=Constant(value=5)), op=Mult(), right=Constant(value=7))`
— thiếu lớp `Expression(body=...)` bọc ngoài
::why
Gần đúng ở toàn bộ phần BÊN TRONG — cấu trúc `BinOp` lồng `BinOp`, đúng
cả thứ tự `Add` rồi `Mult` — phần đó dựng chính xác.

Chỗ lệch: quên mất lớp NGOÀI CÙNG mà `ast.parse(..., mode="eval")`
luôn thêm — nút `Expression(body=...)` bọc quanh toàn bộ cây, đúng cái
bẫy ví dụ vừa nhắc.
::
:::

:::opt
`Expression(body=BinOp(left=Constant(value=1), op=Add(), right=BinOp(left=Constant(value=5), op=Mult(), right=Constant(value=7))))`
— `Add` nằm ngoài, `Mult` lồng bên trong
::why
Gần đúng ở việc bạn nhớ đúng công thức chung: có một `BinOp` lồng bên
trong một `BinOp` khác — cấu trúc lồng nhau đó không sai.

Chỗ lệch: đảo NGƯỢC vị trí. Dấu ngoặc `(1 + 5)` buộc phép CỘNG phải
gộp và tính TRƯỚC — nghĩa là `Add` phải nằm SÂU HƠN (lồng bên trong),
còn `Mult` (áp dụng SAU CÙNG lên cả kết quả) mới là toán tử NGOÀI
CÙNG, không phải ngược lại.
::
:::

:::opt
`Expression(body=BinOp(left=Constant(value=1), op=Add(), right=Constant(value=5)))`
— chỉ có phép cộng, không thấy phép nhân với 7 đâu cả
::why
Gần đúng ở phần bên trong dấu ngoặc: `1 + 5` đúng là một `BinOp Add` —
phần đó dựng đúng.

Chỗ lệch: bỏ sót hẳn phép NHÂN với `7`. Biểu thức đầy đủ là
`(1 + 5) * 7`, không phải chỉ `1 + 5` — cây phải có một `BinOp Mult`
NGOÀI CÙNG, bọc lấy toàn bộ kết quả của `(1 + 5)` và nhân với `7`.
::
:::
::::

::::code{#doi-chieu-cay-that}
Biểu thức `"(6 + 2) * 3"`. Điền `du_doan` — một CHUỖI, tự tay viết,
đúng khuôn `ast.dump()` compact — TRƯỚC khi so với `cay_that` (máy thật
tính, đặt SAU dự đoán của bạn, bạn không nhìn thấy khi viết `du_doan`).

```python title=starter
import ast

bieu_thuc = "(6 + 2) * 3"

# TỰ TAY viết dự đoán của bạn — một CHUỖI, đúng định dạng ast.dump()
# compact (không indent) — TRƯỚC khi máy tính ở dòng dưới.
du_doan = ___

cay_that = ast.dump(ast.parse(bieu_thuc, mode="eval"))

print(du_doan == cay_that)
print(cay_that)
```

```python title=solution
import ast

bieu_thuc = "(6 + 2) * 3"

# TỰ TAY viết dự đoán của bạn — một CHUỖI, đúng định dạng ast.dump()
# compact (không indent) — TRƯỚC khi máy tính ở dòng dưới.
du_doan = "Expression(body=BinOp(left=BinOp(left=Constant(value=6), op=Add(), right=Constant(value=2)), op=Mult(), right=Constant(value=3)))"

cay_that = ast.dump(ast.parse(bieu_thuc, mode="eval"))

print(du_doan == cay_that)
print(cay_that)
```

```python title=test
assert cay_that == "Expression(body=BinOp(left=BinOp(left=Constant(value=6), op=Add(), right=Constant(value=2)), op=Mult(), right=Constant(value=3)))", f"cay_that (do máy thật tính ra) không đúng bản dump mong đợi — đang ra {cay_that}"
assert isinstance(du_doan, str) and du_doan.startswith("Expression("), f"du_doan phải là một CHUỖI, tự tay viết, đúng khuôn ast.dump() — đang là {du_doan!r}"
assert du_doan == cay_that, f"du_doan (dự đoán tay của bạn) phải khớp CHÍNH XÁC với cay_that (máy thật) — đang lệch: du_doan={du_doan!r}, cay_that={cay_that!r}"
```

:::hints
- kind: attention
  body: Chỗ trống là một CHUỖI viết TAY, không gọi ast.dump()/ast.parse() nào ở đó cả — hai hàm đó chỉ dùng đúng một lần, ở dòng tính cay_that đã có sẵn.
- kind: strategy
  body: '"(6 + 2) * 3" giống hệt cấu trúc của "(2 + 3) * 4" bài trước đã dựng: ngoặc buộc phép CỘNG gộp và tính trước, nên Add lồng bên trong, Mult ở ngoài cùng. Viết đủ ba lớp: Expression(body=...) bọc ngoài, BinOp(..., op=Mult(), ...) ở giữa, và BinOp(left=Constant(value=6), op=Add(), right=Constant(value=2)) làm nhánh trái lồng bên trong.'
- kind: one-line
  body: 'Chỗ trống là: "Expression(body=BinOp(left=BinOp(left=Constant(value=6), op=Add(), right=Constant(value=2)), op=Mult(), right=Constant(value=3)))"'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: du_doan phải là một chuỗi VIẾT TAY — không được tính lại bằng cách gọi thêm ast.dump()/ast.parse() ở chỗ trống, vì bài này đang chấm khả năng DỰ ĐOÁN trước khi chạy máy, không chấm khả năng gọi lại đúng hàm
  requireAst:
  # max: 1 mỗi cái — đếm thật trên solution: ast.dump() và ast.parse()
  # mỗi hàm chỉ được gọi ĐÚNG 1 lần trong cả bài, ở dòng tính cay_that đã
  # có sẵn trong khung (không sửa được). Nếu chỗ trống "gian lận" bằng
  # cách gọi lại ast.dump(ast.parse(bieu_thuc, mode="eval")) thay vì viết
  # tay, số lần gọi mỗi hàm tăng lên 2 — vượt trần max:1, luật static bắt
  # được ngay, dù du_doan == cay_that vẫn đúng về mặt giá trị.
  # ĐÃ THỬ THẬT: điền True/1/0 vào chỗ trống — không vòng lặp nào trong
  # bài này nên cả ba dừng ngay, không treo; cả ba khiến du_doan == cay_that
  # là False (so một bool/int với một chuỗi dài không bao giờ bằng nhau),
  # in ra "False" thay vì "True" — test bắt được độc lập với luật static.
  #
  # LỖ ĐÃ TÌM RA VÀ VÁ: hai luật uses-call ở trên, ĐỨNG RIÊNG, không phân
  # biệt nổi True/1/0 với lời giải thật — điền True/1/0 không đổi số lần
  # gọi dump()/parse() ở dòng cay_that (dòng đó cố định, ngoài tầm chỗ
  # trống), nên cả hai luật max:1 vẫn ĐẠT trên cả ba câu điền bừa
  # (tools/kiem_ma_bai_hoc.mjs bắt được đúng lỗ này). Toàn bộ khối vẫn
  # chặn được nhờ tầng test (isinstance/so sánh chuỗi), nhưng luật static
  # tự nó chưa phân biệt gì. has-literal với CHUỖI dump đầy đủ làm target
  # là cách trực tiếp nhất, nhưng chuỗi đó có dấu phẩy — bộ đọc
  # `.lesson.md` tách một mục danh sách bằng dấu phẩy (`doc_muc` trong
  # content-compiler), nên một target có phẩy bên trong bị CẮT CỤT ngay
  # tại dấu phẩy đầu tiên (đã thử thật, thấy target biên dịch ra nửa
  # chuỗi). Dùng forbidAst cấm thẳng literal True (giá trị cụ thể mà
  # tools/kiem_ma_bai_hoc.mjs dùng để dò lỗ) — không dấu phẩy, biên dịch
  # đúng. ĐÃ THỬ THẬT: điền True vào chỗ trống tạo ra một Constant(True)
  # trong mã, forbidAst bắt ngay; lời giải thật không có literal True
  # nào nên không bị vướng.
  - kind: uses-call, target: dump, max: 1
  - kind: uses-call, target: parse, max: 1
  forbidAst:
  - kind: has-literal, target: "True"
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^True\\nExpression\\(body=BinOp\\(left=BinOp\\(left=Constant\\(value=6\\), op=Add\\(\\), right=Constant\\(value=2\\)\\), op=Mult\\(\\), right=Constant\\(value=3\\)\\)\\)\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Dự đoán khớp máy thật — không phải vì bạn đoán may, mà vì bạn đã nắm
đúng cả bốn luật của cụm bài: token không đủ, cây thật thay thế nó,
độ ưu tiên là độ sâu, và ngoặc ra lệnh cho hình dạng.
::::

::::reflect{#nghi-lai}
Một câu hỏi khép lại cả cụm bài về cây cú pháp.

Suốt cụm này, bạn luôn ĐỌC cây — nhận diện nút nào lồng ở đâu, đối
chiếu hình dạng dự đoán với `ast.dump()` thật. Nhưng ĐỌC một cây và
TÍNH GIÁ TRỊ của nó là hai việc khác nhau. May thay, bạn đã biết cách
ĐI BỘ qua một cây từ trước — phần Cấu trúc dữ liệu dạy bằng vòng lặp
với một ngăn xếp tự quản, rồi phần Thuật toán viết lại đúng việc đó
bằng đệ quy. Nếu dùng ĐÚNG kỹ năng đi bộ ấy lên cây cú pháp vừa học —
không phải để LIỆT KÊ các nút, mà để GỘP giá trị của chúng lại — bạn
có tính ra được kết quả thật của biểu thức không?

Bài sau bắt đầu làm đúng việc đó.
::::

::::checkpoint{mastery=0.8}
::::
