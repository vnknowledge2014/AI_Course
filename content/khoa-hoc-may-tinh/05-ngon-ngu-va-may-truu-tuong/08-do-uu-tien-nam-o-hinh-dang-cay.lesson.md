---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.do-uu-tien-nam-o-hinh-dang-cay
title: "Độ ưu tiên phép toán nằm ở HÌNH DẠNG cây"
summary: "BinOp(2, Add, BinOp(3, Mult, 4)) — phép nhân nằm SÂU HƠN, nên tính TRƯỚC, vì cây phải xử lý xong nút con trước khi báo cáo nút cha. 'Nhân trước cộng sau' không phải luật máy nhớ riêng — nó LÀ hình dạng cây."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ngu.precedence-is-tree-shape]
requires: [ngu.ast]
concepts: [ngu.precedence-is-tree-shape]
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
Không trùng hợp đâu. Và mối liên hệ đó rõ ràng hơn bạn tưởng.
::::

::::explain{#uu-tien-la-do-sau}
Bài phần Cấu trúc dữ liệu đã dạy một thứ tự duyệt cây tên
**Trái-Phải-Gốc** (post-order): xử lý xong CẢ HAI nhánh con rồi mới
báo cáo chính nút — giống việc tính dung lượng một thư mục, phải cộng
hết dung lượng các thư mục con xong mới biết tổng của thư mục cha. Để
TÍNH được giá trị của một nút `BinOp`, máy bắt buộc phải theo đúng thứ
tự đó: biết `.left` bằng bao nhiêu, biết `.right` bằng bao nhiêu, RỒI
mới áp `.op` lên hai giá trị đó. Không có cách nào tính nút cha trước
khi biết giá trị của nút con.

Nhìn lại cây của `2 + 3 * 4`: nút gốc là `BinOp(Add)`, nhánh phải của
nó không phải một số đơn — mà là MỘT `BinOp(Mult)` KHÁC, lồng bên
trong. Muốn tính nút gốc (phép cộng), máy phải biết giá trị nhánh phải
trước — mà nhánh phải LÀ một phép nhân chưa tính. Vậy máy buộc phải
tính `3 * 4` XONG trước, được `12`, rồi mới cộng `2 + 12` được `14`.

Đây không phải một luật máy "nhớ riêng" kiểu ghi vào sổ tay ("nhân
đứng trước cộng trong bảng ưu tiên"). Đây là hệ quả TRỰC TIẾP của việc
`ast.parse()` luôn đặt toán tử có độ ưu tiên CAO HƠN vào vị trí LỒNG
SÂU HƠN trong cây — để đúng thứ tự tính-con-trước-cha-sau ở trên tự
động cho ra kết quả đúng. "Nhân trước, cộng sau" không phải một luật
ghi nhớ tách rời khỏi cây — nó CHÍNH LÀ hình dạng cây.
::::

::::example{#doi-vi-tri-van-long}
Byte dựng cây cho hai biểu thức: `2 + 3 * 4` (phép nhân đứng SAU, bên
phải) và `5 * 2 + 1` (phép nhân đứng TRƯỚC, bên trái) — cố ý đổi VỊ TRÍ
của phép nhân, xem hình dạng cây có đổi theo không.

```python title=readonly
import ast

print(ast.dump(ast.parse("2 + 3 * 4", mode="eval")))
print(ast.dump(ast.parse("5 * 2 + 1", mode="eval")))
```

```text title=readonly
Expression(body=BinOp(left=Constant(value=2), op=Add(), right=BinOp(left=Constant(value=3), op=Mult(), right=Constant(value=4))))
Expression(body=BinOp(left=BinOp(left=Constant(value=5), op=Mult(), right=Constant(value=2)), op=Add(), right=Constant(value=1)))
```

Dòng đầu: `Mult` lồng ở nhánh PHẢI (`right=BinOp(...)`) của `Add`. Dòng
sau: `Mult` lồng ở nhánh TRÁI (`left=BinOp(...)`) của `Add`. Vị trí bên
trong cây đảo hẳn — trái hoá phải, phải hoá trái. Nhưng có một điều
KHÔNG đổi ở cả hai: `Add` luôn luôn là toán tử ở NÚT GỐC (ngoài cùng),
`Mult` luôn luôn là toán tử LỒNG bên trong — bất kể `Mult` đứng bên
trái hay bên phải trong CHỮ VIẾT của biểu thức. Cái quyết định độ sâu
không phải VỊ TRÍ đọc trái-phải, mà là ĐỘ ƯU TIÊN của chính toán tử đó.
::::

::::predict{#doan-do-sau commitOnce}
Biểu thức `1 * 5 + 7`.

**Trước khi chạy `ast.dump()`**, bạn đoán: toán tử nào nằm SÂU HƠN
trong cây — `Add` hay `Mult`?

:::opt{correct}
`Mult` — nằm lồng bên trong (ở nhánh trái của nút gốc), vì phép nhân
luôn có độ ưu tiên cao hơn phép cộng, bất kể nó đứng bên trái hay bên
phải trong biểu thức
:::

:::opt
`Add` — vì phép cộng luôn là nút gốc của toàn bộ cây, mà nút gốc là nút
"lớn nhất", nên chắc chắn nó nằm SÂU nhất
::why
Gần đúng ở việc bạn nhớ đúng: `Add` đúng là nút GỐC ở đây — quan sát đó
không sai.

Chỗ lệch nằm ở việc lẫn lộn "nút gốc" với "nút sâu nhất". Nút GỐC là
nút NGOÀI CÙNG, được xử lý SAU CÙNG — tức là nút NÔNG nhất, độ sâu 0,
không phải sâu nhất. Nút LỒNG bên trong nó (ở đây là `Mult`) mới là nút
sâu hơn, được xử lý trước.
::
:::

:::opt
Cả hai nằm cùng một độ sâu, vì biểu thức chỉ có đúng hai toán tử,
không có toán tử thứ ba để tạo thêm một tầng
::why
Gần đúng ở việc đếm đúng số toán tử: `1 * 5 + 7` đúng là chỉ có hai
toán tử (`*` và `+`) — con số đó không sai.

Chỗ lệch: hai toán tử không có nghĩa là "cùng một tầng". Một trong hai
LUÔN LÀ nút gốc (độ sâu 0), toán tử còn lại LUÔN LÀ con của nút gốc đó
(độ sâu 1) — hai toán tử tạo ra đúng hai TẦNG khác nhau, không phải một
tầng chung.
::
:::

:::opt
`Mult` — nhưng chỉ vì nó đứng TRƯỚC trong biểu thức khi đọc từ trái
sang phải, không liên quan gì tới độ ưu tiên
::why
Gần đúng ở kết luận cuối cùng — `Mult` đúng là nằm sâu hơn trong
trường hợp NÀY.

Chỗ lệch nằm ở LÝ DO. Ví dụ ngay phía trên vừa cho thấy: trong
`2 + 3 * 4`, `Mult` đứng SAU (bên phải) mà vẫn lồng sâu hơn `Add` đứng
trước. Vị trí đọc trái-phải không quyết định độ sâu — độ ưu tiên của
toán tử mới là thứ quyết định, đúng như bài này vừa chỉ ra.
::
:::
::::

::::code{#doi-chieu-hai-cay}
Hai biểu thức: `bieu_thuc_a = "2 + 3 * 4"` (nhân đứng bên phải) và
`bieu_thuc_b = "5 * 2 + 1"` (nhân đứng bên trái). Điền hai chỗ trống:
LOẠI toán tử ở NÚT GỐC của mỗi cây.

```python title=starter
import ast

bieu_thuc_a = "2 + 3 * 4"
bieu_thuc_b = "5 * 2 + 1"

cay_a = ast.parse(bieu_thuc_a, mode="eval").body
cay_b = ast.parse(bieu_thuc_b, mode="eval").body

toan_tu_ngoai_a = ___          # LOẠI toán tử ở NÚT GỐC của cay_a
toan_tu_ngoai_b = ___          # LOẠI toán tử ở NÚT GỐC của cay_b

hai_bieu_thuc_cung_toan_tu_ngoai = toan_tu_ngoai_a == toan_tu_ngoai_b

print(toan_tu_ngoai_a)
print(toan_tu_ngoai_b)
print(hai_bieu_thuc_cung_toan_tu_ngoai)
```

```python title=solution
import ast

bieu_thuc_a = "2 + 3 * 4"
bieu_thuc_b = "5 * 2 + 1"

cay_a = ast.parse(bieu_thuc_a, mode="eval").body
cay_b = ast.parse(bieu_thuc_b, mode="eval").body

toan_tu_ngoai_a = type(cay_a.op).__name__
toan_tu_ngoai_b = type(cay_b.op).__name__

hai_bieu_thuc_cung_toan_tu_ngoai = toan_tu_ngoai_a == toan_tu_ngoai_b

print(toan_tu_ngoai_a)
print(toan_tu_ngoai_b)
print(hai_bieu_thuc_cung_toan_tu_ngoai)
```

```python title=test
assert toan_tu_ngoai_a == "Add", f"toán tử ở NÚT GỐC của '2 + 3 * 4' phải là Add (phép nhân nằm SÂU hơn, lồng bên trong) — đang ra {toan_tu_ngoai_a!r}"
assert toan_tu_ngoai_b == "Add", f"toán tử ở NÚT GỐC của '5 * 2 + 1' phải là Add (phép nhân nằm SÂU hơn dù đứng ở nhánh TRÁI lần này) — đang ra {toan_tu_ngoai_b!r}"
assert hai_bieu_thuc_cung_toan_tu_ngoai == True, "cả hai biểu thức phải có CÙNG toán tử ở nút gốc (Add) — dù vị trí của phép nhân trong mỗi cây khác nhau (trái/phải)"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống dùng đúng một khuôn — type(mot_cay.op).__name__ — lấy LOẠI của toán tử ở NÚT GỐC (thuộc tính .op), không phải toán tử bên trong một nhánh con.
- kind: strategy
  body: 'Chỗ trống thứ nhất là toán tử ngoài cùng của cay_a — viết type(cay_a.op).__name__. Chỗ trống thứ hai tương tự cho cay_b — viết type(cay_b.op).__name__. Cả hai cùng cho ra "Add", dù Mult nằm ở nhánh khác nhau trong mỗi cây.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là: type(cay_a.op).__name__ và type(cay_b.op).__name__'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ đọc từ cay_a/cay_b — điền một câu không làm gì (như True, 1, 0) sẽ không cho ra chuỗi "Add", và test sẽ bắt được ngay
  requireAst:
  # min: 1 mỗi cái — đếm thật trên solution: "cay_a" bị ĐỌC (Load) đúng 1
  # lần (trong chỗ trống thứ nhất), "cay_b" bị ĐỌC đúng 1 lần (trong chỗ
  # trống thứ hai). Các dòng "cay_a = ast.parse(...)" chỉ GÁN (Store),
  # không tính. ĐÃ THỬ THẬT: điền True/1/0 vào CẢ HAI chỗ trống — không
  # vòng lặp nào trong bài này nên cả ba dừng ngay, không treo; cả ba cho
  # toan_tu_ngoai_a/b là 'True'/'1'/'0' thay vì 'Add', test bắt được độc
  # lập; và không còn đọc "cay_a"/"cay_b" nào — luật static cũng bắt
  # được. Thử riêng TỪNG chỗ trống một (chỗ kia giữ đúng) bằng True/1/0:
  # cùng kết quả — test và static đều bắt được ở cả hai vị trí.
  - kind: uses-name, target: cay_a, min: 1
  - kind: uses-name, target: cay_b, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Add\\nAdd\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhân đứng trái hay đứng phải không quan trọng — nó luôn lồng sâu hơn
cộng. Độ ưu tiên không phải trí nhớ, nó là hình dạng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Độ ưu tiên nằm ở hình dạng cây, không phải ở trí nhớ. Nhưng nếu bạn
CHỦ ĐỘNG thêm một cặp dấu ngoặc vào biểu thức — thêm hẳn hai token `OP`
mới, như bài "token không đủ" đã thấy — hình dạng cây có BUỘC PHẢI đổi
theo không? Hay dấu ngoặc chỉ là ký hiệu "trang trí", không đụng gì
tới cây thật bên dưới?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
