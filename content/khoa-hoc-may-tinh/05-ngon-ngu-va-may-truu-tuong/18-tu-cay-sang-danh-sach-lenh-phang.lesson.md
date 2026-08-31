---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.tu-cay-sang-danh-sach-lenh-phang
title: "Từ cây sang danh sách lệnh phẳng — biên dịch xuống máy ngăn xếp"
summary: "T3.4 bài 3 tả máy tính THẬT bằng một ngăn xếp: LOAD đẩy vào, BINARY_OP lấy ra tính. Đi CÙNG HƯỚNG với CPython thật: biến cây cú pháp thành một DANH SÁCH LỆNH PHẲNG (PUSH 2, PUSH 3, PUSH 4, MUL, ADD) — đây chính là một bước BIÊN DỊCH, thu nhỏ."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ngu.compile-tree-to-stack-instructions]
requires: [ngu.machine-state, may.eval-stack]
concepts: [ngu.compile-tree-to-stack-instructions]
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
Cây lồng nhau cần biến thành đúng MỘT thứ: một danh sách PHẲNG, đọc
được từ trái sang phải, không còn nhánh nào lồng trong nhánh nào.
::::

::::explain{#bien-dich-la-gi}
T3.4 bài 3 tả máy tính THẬT của CPython bằng một **ngăn xếp tính toán**:
`LOAD_FAST` ĐẨY một giá trị vào đỉnh; `BINARY_OP` LẤY hai giá trị ở đỉnh
ra, tính, rồi ĐẨY kết quả lại. Máy đó không đọc CÂY — nó đọc một DANH
SÁCH LỆNH PHẲNG, `dis.dis()` in ra, từng dòng một, không lồng nhau.

Cây cú pháp bài 7-10 dựng lại LỒNG NHAU — đúng thứ máy tính toán ngăn
xếp không đọc thẳng được. Muốn đưa bộ đánh giá của bạn đi CÙNG HƯỚNG với
CPython thật, cây đó phải được **biên dịch** (compile) — đổi thành một
danh sách lệnh phẳng, dùng đúng hai động tác `LOAD`/`BINARY_OP` bài 3 đã
dạy, viết bằng tên Việt hoá quen thuộc từ T3.4 bài 6: `DAY` (đẩy một giá
trị), `CONG`/`TRU`/`NHAN`/`CHIA` (lấy hai giá trị đỉnh ra, tính, đẩy kết
quả lại).

Luật biên dịch chỉ có một, và nó quyết định TOÀN BỘ thứ tự: **lệnh của
một nút chỉ được phát ra SAU KHI lệnh của cả hai nhánh con đã phát ra
xong.** Vì khi `NHAN` (hay `CONG`, `TRU`, `CHIA`) chạy, nó cần cả hai
giá trị đã NẰM SẴN trên đỉnh ngăn xếp — nếu lệnh toán tử phát ra TRƯỚC
lệnh của các con, tới lúc nó chạy, ngăn xếp chưa có gì để lấy.

Viết thành hàm đệ quy `bien_dich(node)`:

- Nút LÁ (`ast.Constant`): trả về một danh sách CHỈ MỘT lệnh —
  `[("DAY", giá_trị)]`.
- Nút TRONG (`ast.BinOp`): biên dịch nhánh TRÁI trước, biên dịch nhánh
  PHẢI sau, rồi NỐI hai danh sách đó lại, thêm đúng MỘT lệnh toán tử vào
  cuối cùng — `lenh_trai + lenh_phai + [lenh_toan_tu]`.

Với `2 + 3 * 4`, cây (bài 8) là `BinOp(2, Add, BinOp(3, Mult, 4))`.
Biên dịch nhánh trái (`Constant(2)`) ra `[("DAY", 2)]`. Biên dịch nhánh
phải (`BinOp(3, Mult, 4)`) đệ quy tiếp: `[("DAY", 3), ("DAY", 4),
("NHAN",)]`. Nối cả hai, thêm `("CONG",)` vào cuối:
`[("DAY", 2), ("DAY", 3), ("DAY", 4), ("NHAN",), ("CONG",)]` — đúng
`PUSH 2, PUSH 3, PUSH 4, MUL, ADD` mà một trình biên dịch thật sẽ phát
ra cho biểu thức này.
::::

::::example{#bien-dich-that}
Chạy `bien_dich` thật trên hai biểu thức — một không ngoặc, một có
ngoặc (bài 9 đã dạy: ngoặc đổi HÌNH DẠNG cây, không đổi token):

```python title=readonly
import ast

def bien_dich(node):
    if isinstance(node, ast.Constant):
        return [("DAY", node.value)]
    if isinstance(node, ast.BinOp):
        lenh_trai = bien_dich(node.left)
        lenh_phai = bien_dich(node.right)
        if isinstance(node.op, ast.Add):
            lenh_toan_tu = [("CONG",)]
        elif isinstance(node.op, ast.Sub):
            lenh_toan_tu = [("TRU",)]
        elif isinstance(node.op, ast.Mult):
            lenh_toan_tu = [("NHAN",)]
        elif isinstance(node.op, ast.Div):
            lenh_toan_tu = [("CHIA",)]
        else:
            raise ValueError(f"chưa hiểu phép toán {type(node.op).__name__}")
        return lenh_trai + lenh_phai + lenh_toan_tu
    raise ValueError(f"chưa hiểu loại nút {type(node).__name__}")

for bt in ["2 + 3 * 4", "(2 + 3) * 4"]:
    cay = ast.parse(bt, mode="eval").body
    print(bt, "->", bien_dich(cay))
```

```text title=readonly
2 + 3 * 4 -> [('DAY', 2), ('DAY', 3), ('DAY', 4), ('NHAN',), ('CONG',)]
(2 + 3) * 4 -> [('DAY', 2), ('DAY', 3), ('CONG',), ('DAY', 4), ('NHAN',)]
```

Bốn token số/toán tử giống hệt nhau ở cả hai biểu thức (bài 9) — nhưng
`(2 + 3)` giờ nằm SÂU HƠN trong cây, nên `("CONG",)` phát ra SỚM HƠN,
ngay sau `("DAY", 3)`, thay vì phát ra cuối cùng như ở `2 + 3 * 4`. Cùng
bốn số, cùng bốn toán tử, khác hẳn thứ tự lệnh — đúng như cây khác hẳn
hình dạng.
::::

::::predict{#doan-danh-sach-lenh commitOnce}
Biểu thức `"10 - 4 / 2"`. Theo đúng độ ưu tiên toán tử, phép chia tính
trước phép trừ — cây là `BinOp(10, Sub, BinOp(4, Div, 2))`.

**Trước khi chạy máy**, `bien_dich` trên biểu thức này trả về danh sách
lệnh nào?

:::opt{correct}
`[("DAY", 10), ("DAY", 4), ("DAY", 2), ("CHIA",), ("TRU",)]`
:::

:::opt
`[("DAY", 10), ("DAY", 4), ("DAY", 2), ("TRU",), ("CHIA",)]` — đổi chỗ
hai lệnh cuối, vì `TRU` đứng trước trong biểu thức gốc `"10 - 4 / 2"`
::why
Gần đúng ở việc bạn đếm đúng cả bốn giá trị `DAY` cần đẩy vào — không
thiếu số nào.

Chỗ lệch: thứ tự các LỆNH TOÁN TỬ không đi theo thứ tự chúng xuất hiện
trong CHUỖI KÝ TỰ gốc — nó đi theo HÌNH DẠNG CÂY. `Div` nằm SÂU HƠN
(nhánh phải của `Sub`), nên `bien_dich` phải biên dịch xong nhánh đó
trước — `("CHIA",)` phát ra trước `("TRU",)`, dù dấu trừ đứng trước dấu
chia khi đọc từ trái sang phải trên trang giấy.
::
:::

:::opt
`[("DAY", 10), ("TRU",), ("DAY", 4), ("DAY", 2), ("CHIA",)]` — đẩy 10,
trừ ngay, rồi mới xử lý phần chia
::why
Gần đúng ở việc bạn đẩy đúng `("DAY", 10)` lên đầu tiên — nhánh trái của
cây quả thật được biên dịch trước.

Chỗ lệch: `("TRU",)` không thể phát ra ngay sau `("DAY", 10)` — lúc đó
ngăn xếp mới có MỘT giá trị (`10`), trong khi `TRU` cần LẤY HAI giá trị
đỉnh ra để tính. Lệnh toán tử của một nút LUÔN phát ra SAU CÙNG, sau khi
lệnh của CẢ HAI nhánh con (trái lẫn phải) đã phát ra hết.
::
:::

:::opt
`[("DAY", 4), ("DAY", 2), ("CHIA",), ("DAY", 10), ("TRU",)]` — biên
dịch nhánh phải (phép chia) trước, nhánh trái (số 10) sau
::why
Gần đúng ở việc bạn xếp đúng THỨ TỰ NỘI BỘ của nhánh chia:
`("DAY", 4), ("DAY", 2), ("CHIA",)` — ba lệnh đó đúng liền nhau.

Chỗ lệch: `bien_dich` biên dịch nhánh TRÁI trước, nhánh PHẢI sau —
`lenh_trai + lenh_phai + lenh_toan_tu`, đúng thứ tự đó, không đảo
ngược. Nhánh trái ở đây chỉ là một lá đơn giản (`Constant(10)`), nhưng
nó vẫn phải phát lệnh `("DAY", 10)` TRƯỚC toàn bộ nhánh phải, không phải
sau.
::
:::
::::

::::code{#tu-viet-bo-bien-dich}
Bộ biên dịch cho biểu thức `"(5 - 1) * 2 + 8"`. Nhánh `Add`/`Sub`/`Mult`/
`Div` chọn đúng lệnh toán tử đã viết sẵn — còn thiếu đúng hai chỗ: lệnh
của một LÁ, và cách NỐI lệnh của hai nhánh con với lệnh toán tử.

```python title=starter
import ast

def bien_dich(node):
    if isinstance(node, ast.Constant):
        return ___                     # MỘT lệnh: đẩy giá trị của lá này vào ngăn xếp
    if isinstance(node, ast.BinOp):
        lenh_trai = bien_dich(node.left)
        lenh_phai = bien_dich(node.right)
        if isinstance(node.op, ast.Add):
            lenh_toan_tu = [("CONG",)]
        elif isinstance(node.op, ast.Sub):
            lenh_toan_tu = [("TRU",)]
        elif isinstance(node.op, ast.Mult):
            lenh_toan_tu = [("NHAN",)]
        elif isinstance(node.op, ast.Div):
            lenh_toan_tu = [("CHIA",)]
        else:
            raise ValueError(f"chưa hiểu phép toán {type(node.op).__name__}")
        return ___                     # NHÁNH TRÁI trước, NHÁNH PHẢI sau, TOÁN TỬ cuối cùng
    raise ValueError(f"chưa hiểu loại nút {type(node).__name__}")

bieu_thuc = "(5 - 1) * 2 + 8"
cay = ast.parse(bieu_thuc, mode="eval").body
danh_sach_lenh = bien_dich(cay)

print(danh_sach_lenh)
```

```python title=solution
import ast

def bien_dich(node):
    if isinstance(node, ast.Constant):
        return [("DAY", node.value)]
    if isinstance(node, ast.BinOp):
        lenh_trai = bien_dich(node.left)
        lenh_phai = bien_dich(node.right)
        if isinstance(node.op, ast.Add):
            lenh_toan_tu = [("CONG",)]
        elif isinstance(node.op, ast.Sub):
            lenh_toan_tu = [("TRU",)]
        elif isinstance(node.op, ast.Mult):
            lenh_toan_tu = [("NHAN",)]
        elif isinstance(node.op, ast.Div):
            lenh_toan_tu = [("CHIA",)]
        else:
            raise ValueError(f"chưa hiểu phép toán {type(node.op).__name__}")
        return lenh_trai + lenh_phai + lenh_toan_tu
    raise ValueError(f"chưa hiểu loại nút {type(node).__name__}")

bieu_thuc = "(5 - 1) * 2 + 8"
cay = ast.parse(bieu_thuc, mode="eval").body
danh_sach_lenh = bien_dich(cay)

print(danh_sach_lenh)
```

```python title=test
assert danh_sach_lenh == [("DAY", 5), ("DAY", 1), ("TRU",), ("DAY", 2), ("NHAN",), ("DAY", 8), ("CONG",)], f"(5 - 1) * 2 + 8 phải biên dịch ra đúng bảy lệnh này, đúng thứ tự — đang ra {danh_sach_lenh}"
assert len(danh_sach_lenh) == 7, f"phải có đúng 7 lệnh: bốn DAY (5,1,2,8) và ba lệnh toán tử (TRU, NHAN, CONG) — đang có {len(danh_sach_lenh)}"
```

:::hints
- kind: attention
  body: Chỗ trống đầu chỉ trả về MỘT lệnh cho một lá — không đệ quy gì cả. Chỗ trống sau nối ba danh sách đã có sẵn tên (lenh_trai, lenh_phai, lenh_toan_tu) lại thành một.
- kind: strategy
  body: 'Một lá chỉ cần đẩy giá trị của chính nó: return [("DAY", node.value)]. Một nút trong cần lệnh của nhánh trái, RỒI lệnh của nhánh phải, RỒI lệnh toán tử — nối ba list Python bằng dấu +: return lenh_trai + lenh_phai + lenh_toan_tu.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là: [("DAY", node.value)]  và  lenh_trai + lenh_phai + lenh_toan_tu'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: chỗ trống đầu phải trả về một lệnh DAY thật (không gõ cứng danh sách lệnh của cả biểu thức); chỗ trống sau phải NỐI đúng lenh_trai, lenh_phai, lenh_toan_tu — không được bỏ sót một trong ba hoặc đảo thứ tự thành lệnh toán tử đứng trước
  requireAst:
  # has-literal "DAY" min:1 — đếm thật trên solution: chuỗi "DAY" chỉ xuất
  # hiện đúng MỘT chỗ trong toàn bộ mã nguồn, bên trong chỗ trống đầu
  # ([("DAY", node.value)]) — không nhánh CONG/TRU/NHAN/CHIA nào dùng chữ
  # "DAY". uses-name lenh_trai/lenh_phai/lenh_toan_tu min:1 mỗi tên — cả
  # ba tên này chỉ được ĐỌC (Load) đúng một lần mỗi tên, ở chỗ trống sau
  # (chỗ gán "lenh_trai = bien_dich(...)" là Store, uses-name không đếm).
  # ĐÃ THỬ THẬT bằng cả ba cách True/1/0 cho CẢ HAI chỗ trống: đệ quy vẫn
  # chạy hết trên cây cố định của "(5 - 1) * 2 + 8" (7 nút, sâu tối đa 3),
  # không phụ thuộc giá trị chỗ trống nên không vòng nào chạy vô hạn — cả
  # ba dừng ngay lập tức, trả về True/1/0 (không phải list) thay vì danh
  # sách bảy lệnh. Cả ba đều có 0 lần "DAY" và 0 lần đọc lenh_trai/lenh_phai/
  # lenh_toan_tu — static trượt độc lập; và assert cũng trượt độc lập vì
  # danh_sach_lenh không còn là list nào khớp được.
  - kind: has-literal, target: "DAY", min: 1
  - kind: uses-name, target: lenh_trai, min: 1
  - kind: uses-name, target: lenh_phai, min: 1
  - kind: uses-name, target: lenh_toan_tu, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: exact
  expect: "[('DAY', 5), ('DAY', 1), ('TRU',), ('DAY', 2), ('NHAN',), ('DAY', 8), ('CONG',)]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cây bảy nút, bảy lệnh phẳng, đúng thứ tự — mỗi lệnh toán tử chờ đủ hai
nhánh con phát lệnh xong mới tới lượt mình. Đó là một bước biên dịch,
dù nhỏ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa có một DANH SÁCH LỆNH PHẲNG — bảy lệnh, đúng thứ tự. Nhưng
T3.4 bài 6 đã nói thẳng: một danh sách CHỈ LÀ một danh sách, nó "nằm yên
như một tờ công thức nấu ăn dán trên tường." Không có gì trong bảy lệnh
`("DAY", 5), ("DAY", 1), ("TRU",), ...` tự đứng dậy mà tính ra `20`.

Muốn danh sách này THẬT SỰ chạy — cần THỨ GÌ đọc nó, lệnh nào cũng ẩn
sau một ngăn xếp tính toán y hệt bài 3 T3.4 đã tả? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
