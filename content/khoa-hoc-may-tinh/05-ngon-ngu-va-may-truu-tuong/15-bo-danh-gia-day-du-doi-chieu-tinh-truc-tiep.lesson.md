---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.bo-danh-gia-day-du-doi-chieu-tinh-truc-tiep
title: "Bộ đánh giá đầy đủ +,-,*,/ và ngoặc — đối chiếu tính trực tiếp"
summary: "Bài chốt cụm: thêm nhánh cuối (Div) theo đúng công thức bài 14, hoàn thiện +,-,*,/. Chạy trên nhiều biểu thức có ngoặc, đối chiếu với phép tính Python trực tiếp — không dùng eval(). Ngoặc không cần nhánh riêng: cây bài 9 đã gói xong việc đó."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ngu.full-evaluator]
requires: [ngu.extend-evaluator]
concepts: [ngu.full-evaluator]
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
Một phép cuối cùng, đúng công thức cũ — rồi thử nó trên những biểu thức
rắc rối nhất Byte tìm được, kể cả những cái có ngoặc.
::::

::::explain{#nhanh-cuoi-va-cau-hoi-ngoac}
Thêm `Div` không có gì mới — đúng công thức bài trước đã dùng cho `Sub`:
chen thêm một `elif isinstance(node.op, ast.Div): return trai / phai`,
trước `else`. Bốn phép `+ - * /` xong đủ.

Còn câu hỏi bài trước để lại: NGOẶC thì sao? `"(2 + 3) * 4"` có ngoặc,
`"2 + 3 * 4"` thì không — `danh_gia` có cần biết gì về dấu `(` `)`
không?

Không. Bài "Ngoặc thay đổi hình dạng cây, không đổi token" (bài 9) đã
chỉ ra: `ast.parse` xử lý ngoặc ngay lúc DỰNG CÂY — cặp ngoặc không tồn
tại như một nút riêng trong cây, nó chỉ quyết định nút nào LỒNG SÂU
HƠN nút nào. Với `"(2 + 3) * 4"`, phép cộng nằm sâu hơn, tính trước —
`danh_gia` chỉ đơn giản đi bộ đúng cây đó, y hệt cách nó đi bộ mọi cây
khác từ bài 11. Không nhánh `if` nào cho dấu ngoặc, vì không có nút nào
đại diện cho dấu ngoặc để mà hỏi.
::::

::::example{#doi-chieu-nhieu-bieu-thuc}
Hàm `danh_gia` đầy đủ bốn phép, chạy trên bốn biểu thức — hai cái có
ngoặc — rồi đối chiếu với kết quả TÍNH TRỰC TIẾP bằng Python (không
dùng `eval()` — track này dạy TỰ VIẾT bộ đánh giá, không mượn công cụ
có sẵn nguy hiểm):

```python title=readonly
import ast

def danh_gia(node):
    if isinstance(node, ast.Constant):
        return node.value
    elif isinstance(node, ast.BinOp):
        trai = danh_gia(node.left)
        phai = danh_gia(node.right)
        if isinstance(node.op, ast.Add):
            return trai + phai
        elif isinstance(node.op, ast.Mult):
            return trai * phai
        elif isinstance(node.op, ast.Sub):
            return trai - phai
        elif isinstance(node.op, ast.Div):
            return trai / phai
        else:
            raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}")

def tinh(bt):
    cay = ast.parse(bt, mode="eval")
    return danh_gia(cay.body)

bieu_thuc = ["(2 + 3) * 4", "8 / 2 - 1", "(10 - 4) / 3"]
for bt in bieu_thuc:
    print(bt, "->", tinh(bt))
```

```text title=readonly
(2 + 3) * 4 -> 20
8 / 2 - 1 -> 3.0
(10 - 4) / 3 -> 2.0
```

Cả ba khớp đúng phép tính tay: `(2 + 3) * 4 = 20`, `8 / 2 - 1 = 3.0`,
`(10 - 4) / 3 = 2.0`. `danh_gia` không hề biết "đây là biểu thức có
ngoặc" — nó chỉ đi bộ đúng cây `ast.parse` đã dựng sẵn, và cây đó đã tự
gói ngoặc vào HÌNH DẠNG của nó từ bài 9.
::::

::::predict{#doan-chia-het commitOnce}
Bộ đánh giá vừa hoàn thiện, có nhánh `Div` dùng `/`. Biểu thức mới:
`"10 / 2"` — chia hết, không có phần dư.

**Trước khi chạy**, bạn đoán: `tinh("10 / 2")` trả về giá trị nào, và
đó là kiểu dữ liệu gì?

:::opt{correct}
`5.0` — kiểu `float`, vì toán tử `/` của Python LUÔN trả về `float`, kể
cả khi chia hết
:::

:::opt
`5` — kiểu `int`, vì `10` chia hết cho `2` nên kết quả phải là số
nguyên tròn trịa
::why
Gần đúng ở trực giác toán học thông thường — `10 / 2` đúng là "chia
hết", không có phần dư, và với phép chia trên giấy thì `5` là câu trả
lời tự nhiên.

Chỗ lệch: Python không phân biệt "chia hết" với "chia không hết" ở
toán tử `/`. Dòng `return trai / phai` dùng đúng phép chia thường của
Python — phép này LUÔN trả về `float`, bất kể `trai` và `phai` có chia
hết cho nhau hay không. Muốn kết quả `int` khi chia hết phải dùng một
toán tử khác (`//`, chia lấy phần nguyên) — nhánh này không dùng nó.
::
:::

:::opt
`"5.0"` — kiểu chuỗi, vì kết quả được `print` ra màn hình
::why
Gần đúng ở việc bạn để ý `print(...)` đúng là thứ khiến giá trị HIỆN
RA thành chữ trên màn hình — quan sát đó không sai.

Chỗ lệch: `print` chỉ ĐỔI CÁCH HIỂN THỊ, nó không đổi kiểu dữ liệu bên
trong biến. `tinh("10 / 2")` trả về một giá trị kiểu `float` (`5.0`) —
`print` chỉ vẽ nó ra màn hình dưới dạng chữ, giá trị bên trong hàm và
biến vẫn nguyên là số thực, không phải chuỗi.
::
:::

:::opt
Ném `ValueError`, vì `10` chia hết cho `2` là một trường hợp đặc biệt
chưa được xử lý
::why
Gần đúng ở việc bạn cẩn trọng nghĩ tới trường hợp đặc biệt — thái độ đó
đúng khi đọc mã lạ.

Chỗ lệch: nhánh `Div` xử lý MỌI phép chia bằng đúng MỘT dòng
`return trai / phai`, không phân biệt chia hết hay không — không có
nhánh `if` nào tách riêng "chia hết" ra để xử lý khác, nên không có
`else` nào bị chạm tới, không lỗi nào được ném.
::
:::
::::

::::code{#hoan-thien-va-doi-chieu}
Byte muốn `danh_gia` hiểu đủ cả bốn phép. Khung hàm đã có `Add`, `Mult`,
`Sub`, và cả nhánh `elif isinstance(node.op, ast.Div):` — chỉ còn thiếu
đúng một dòng trả về. Sau đó, chạy nó trên bốn biểu thức (hai cái có
ngoặc) và đối chiếu với kết quả tính TRỰC TIẾP bằng Python.

```python title=starter
import ast

def danh_gia(node):
    if isinstance(node, ast.Constant):
        return node.value
    elif isinstance(node, ast.BinOp):
        trai = danh_gia(node.left)
        phai = danh_gia(node.right)
        if isinstance(node.op, ast.Add):
            return trai + phai
        elif isinstance(node.op, ast.Mult):
            return trai * phai
        elif isinstance(node.op, ast.Sub):
            return trai - phai
        elif isinstance(node.op, ast.Div):
            ___                     # trả về đúng phép CHIA: trai / phai
        else:
            raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}")

def tinh(bt):
    cay = ast.parse(bt, mode="eval")
    return danh_gia(cay.body)

bieu_thuc = ["(2 + 3) * 4", "8 / 2 - 1", "2 + 3 * 4 - 1", "(10 - 4) / 3"]
ket_qua_may = [tinh(bt) for bt in bieu_thuc]
ket_qua_tay = [(2 + 3) * 4, 8 / 2 - 1, 2 + 3 * 4 - 1, (10 - 4) / 3]

print(ket_qua_may)
print(ket_qua_may == ket_qua_tay)
```

```python title=solution
import ast

def danh_gia(node):
    if isinstance(node, ast.Constant):
        return node.value
    elif isinstance(node, ast.BinOp):
        trai = danh_gia(node.left)
        phai = danh_gia(node.right)
        if isinstance(node.op, ast.Add):
            return trai + phai
        elif isinstance(node.op, ast.Mult):
            return trai * phai
        elif isinstance(node.op, ast.Sub):
            return trai - phai
        elif isinstance(node.op, ast.Div):
            return trai / phai
        else:
            raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}")

def tinh(bt):
    cay = ast.parse(bt, mode="eval")
    return danh_gia(cay.body)

bieu_thuc = ["(2 + 3) * 4", "8 / 2 - 1", "2 + 3 * 4 - 1", "(10 - 4) / 3"]
ket_qua_may = [tinh(bt) for bt in bieu_thuc]
ket_qua_tay = [(2 + 3) * 4, 8 / 2 - 1, 2 + 3 * 4 - 1, (10 - 4) / 3]

print(ket_qua_may)
print(ket_qua_may == ket_qua_tay)
```

```python title=test
assert ket_qua_may == [20, 3.0, 13, 2.0], f"bốn biểu thức phải ra [20, 3.0, 13, 2.0] — đang ra {ket_qua_may}"
assert ket_qua_may == ket_qua_tay, f"kết quả bộ đánh giá phải khớp CHÍNH XÁC kết quả tính trực tiếp bằng Python trên cùng bốn biểu thức — đang lệch: máy={ket_qua_may}, tay={ket_qua_tay}"
```

:::hints
- kind: attention
  body: Chỗ trống chỉ là MỘT biểu thức — giá trị hàm phải trả về khi node.op là ast.Div. Nhánh elif isinstance(node.op, ast.Div) đã viết sẵn, chỉ thiếu return.
- kind: strategy
  body: 'Phép chia trên hai giá trị đã hỏi được từ hai nhánh con (trai, phai) chính là trai / phai — đúng thứ tự, trái chia phải. Dùng / (chia thường), không phải // (chia lấy phần nguyên) — bài không dạy phép đó.'
- kind: one-line
  body: 'Chỗ trống là: return trai / phai'
:::

:::validate
# KHÔNG có tier `static` ở bước này — cố tình, không phải quên.
#
# Từng thử `forbidAst: [uses-call, target: eval]`. Đo thật bằng kiemAst()
# lộ ra: luật đó ĐẠT trên cả lời giải THẬT lẫn câu điền bừa `True`/`1`/`0`
# — không gọi eval() cũng không gọi eval(), luật không PHÂN BIỆT được hai
# bên, đúng thứ luật 3b của `kiem_ma_bai_hoc.mjs` bắt được: "một luật đạt
# trên cả hai thì nó không kiểm gì cả". Cũng từng cân nhắc `requireAst`
# đếm dấu "/": dòng `ket_qua_tay` có sẵn trong khung (phần đối chiếu tính
# trực tiếp) đã TỰ chứa 2 phép chia thật ("8 / 2" và "(10 - 4) / 3") —
# min:1 không phân biệt nổi điền đúng với điền hụt vì 2 phép có sẵn đã đủ
# qua ngưỡng. Nâng min lên 3 vá được chỗ đó, nhưng mở đúng lỗ luật 2: một
# lời giải ĐÚNG khác, ví dụ `operator.truediv(trai, phai)`, không hề chứa
# dấu "/" nào, sẽ bị đánh trượt oan. Không nghĩ ra luật static nào vừa
# CHẶN được điền bừa vừa KHÔNG chặn oan lời giải đúng khác, nên bỏ hẳn
# tầng này — đúng lời luật 2 dặn: "không nghĩ ra được thì thường luật ấy
# không cần thiết". Đúng-sai bằng SỐ đã có run/tests/output lo trọn: ĐÃ
# THỬ THẬT bằng cả ba cách True/1/0 cho chỗ trống — chỗ trống đó THAY THẾ
# NGUYÊN DÒNG (không có sẵn `return` phía trước), nên True/1/0 chỉ là một
# BIỂU THỨC RỖNG không làm gì — nhánh Div kết thúc mà không `return`, hàm
# rơi hết thân, trả về `None` ngầm định. Cả ba đều DỪNG AN TOÀN (không
# treo, không vượt trần bước) nhưng KHÔNG chạy hết vòng for: biểu thức thứ
# hai ("8 / 2 - 1") có Div lồng bên trong Sub — danh_gia trên nhánh Div
# trả về None, rồi None - 1 NÉM TypeError ngay lập tức. Chương trình dừng
# bằng lỗi thay vì in ra một danh sách sai — vẫn là dừng AN TOÀN, không
# phải "chạy xong rồi mới sai". run/tests/output đều không khớp kỳ vọng,
# bắt được độc lập.
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\[20, 3\\.0, 13, 2\\.0\\]\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn phép, ngoặc lồng nhau, không một chỗ nào phải mượn `eval()` — chỉ
một hàm đệ quy đi bộ đúng cây, gộp kết quả con lại từng bước, y hệt bài
11 bắt đầu.
::::

::::reflect{#nghi-lai}
Một câu hỏi khép lại cả cụm bài về bộ đánh giá.

Nhìn lại toàn bộ hàm `danh_gia` vừa hoàn thiện: nó nhận một cấu trúc mô
tả phép tính (cây cú pháp), và CHẠY nó — đọc từng phần, quyết định phải
làm gì, rồi đi tiếp cho tới khi ra kết quả. T3.4 bài "Máy có MỘT vòng
lặp riêng" từng dạy đúng một cỗ máy THẬT làm việc y hệt vậy với
bytecode: LẤY một lệnh, HIỂU nó là lệnh gì, LÀM nó, rồi lấy lệnh kế
tiếp.

`danh_gia` không có thanh ghi, không có hệ điều hành, không chạy trên
phần cứng nào cả — nhưng nó có "lấy" (gọi `danh_gia` trên một nút),
"hiểu" (các nhánh `isinstance`), "làm" (`return` một giá trị) hay
không? Nếu có, nó có phải MỘT CÁI MÁY không — chỉ đơn giản hơn máy thật
rất nhiều?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
