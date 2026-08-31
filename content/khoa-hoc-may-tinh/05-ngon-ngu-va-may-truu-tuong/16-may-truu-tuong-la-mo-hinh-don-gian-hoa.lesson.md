---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.may-truu-tuong-la-mo-hinh-don-gian-hoa
title: "Máy trừu tượng: một mô hình ĐƠN GIẢN HOÁ của máy thật"
summary: "T3.4 bài 6 tả máy THẬT: lấy-hiểu-làm trên bytecode CPython, có ngăn xếp tính toán, có thanh ghi. Bộ đánh giá bốn phép toán của cụm trước CŨNG là một cái máy — chỉ đơn giản hơn nhiều — nhưng cùng ý tưởng: nhận một cấu trúc mô tả phép tính, chạy ra kết quả."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ngu.abstract-machine]
requires: [ngu.full-evaluator, may.fetch-decode-execute]
concepts: [ngu.abstract-machine]
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
Bộ đánh giá bốn phép toán của bạn tính đúng mọi biểu thức có ngoặc. Nhưng
nó có ĐÁNG được gọi là một cái MÁY không — hay chỉ là một hàm Python
bình thường?
::::

::::explain{#dinh-nghia-may-truu-tuong}
T3.4 bài 6 dựng một cái máy tí hon, tự đặt tên: một danh sách lệnh phẳng
(`chuong_trinh`, các tuple như `("DAY", 3)`), cộng một vòng
`for lenh in chuong_trinh:` LẤY từng lệnh, HIỂU nó thuộc loại nào, LÀM
đúng việc lệnh đó ghi. Đó chính là **chu trình lấy-hiểu-làm**
(`may.fetch-decode-execute`) — và bài đó đã nói thẳng: CPython thật làm
đúng ý tưởng ấy, chỉ phức tạp hơn nhiều (viết bằng C, có ngăn xếp tính
toán riêng, có bytecode thật).

Nhìn lại bộ đánh giá bốn phép toán vừa hoàn thiện — hàm đệ quy nhận vào
một `ast.AST`, trả về một con số. Nó làm đúng CÙNG một việc, ở tầng khái
niệm: nhận vào một **cấu trúc mô tả phép tính** (ở đây là cây cú pháp,
không phải danh sách lệnh phẳng), áp một tập LUẬT CỐ ĐỊNH lên cấu trúc
đó (mỗi loại nút một cách xử lý — bài 12-14 đã dạy), rồi TẠO RA một kết
quả.

"Nhận cấu trúc mô tả phép tính, áp luật cố định, tạo ra kết quả" — đó
chính là định nghĩa của một **máy trừu tượng** (abstract machine): một
MÔ HÌNH đơn giản hoá của việc tính toán, giữ lại đúng cái LÕI đó, bỏ đi
mọi chi tiết không cần thiết để hiểu Ý TƯỞNG. Máy trừu tượng của bạn
KHÔNG có:

- Ngăn xếp tính toán riêng (bài "Máy tính bằng một ngăn xếp" T3.4) — nó
  dùng thẳng giá trị trả về của lời gọi đệ quy.
- Thanh ghi, hệ điều hành, hay bất cứ phần cứng thật nào.
- Một danh sách lệnh PHẲNG nào — nó đi thẳng trên CÂY, không cần "biên
  dịch" cây thành dãy lệnh trước (cụm bài sau mới làm việc đó).

Nhưng nó VẪN là một máy đúng nghĩa: đưa vào cùng một cấu trúc, luôn ra
cùng một kết quả, theo đúng luật cố định — không có gì "huyền bí" hay
"tình cờ" trong việc nó tính ra đúng số.
::::

::::example{#hai-may-canh-nhau}
Đặt hai máy cạnh nhau, cùng chạy một ý tưởng — máy tí hon của T3.4 bài 6
(danh sách lệnh phẳng, vòng `for`) và bộ đánh giá cây của cụm trước
(cây `ast.AST`, đệ quy):

```python title=readonly
import ast

# Máy 1 — T3.4 bài 6: danh sách lệnh PHẲNG, vòng lặp LẤY-HIỂU-LÀM
chuong_trinh = [("DAY", 4), ("DAY", 5), ("CONG",), ("DAY", 2), ("NHAN",)]
ngan_xep = []
for lenh in chuong_trinh:
    ten = lenh[0]
    if ten == "DAY":
        ngan_xep.append(lenh[1])
    elif ten == "CONG":
        b = ngan_xep.pop(); a = ngan_xep.pop(); ngan_xep.append(a + b)
    elif ten == "NHAN":
        b = ngan_xep.pop(); a = ngan_xep.pop(); ngan_xep.append(a * b)
print("máy 1 (danh sách lệnh):", ngan_xep[-1])

# Máy 2 — cụm trước: CÂY ast.AST, hàm đệ quy
def danh_gia(node):
    if isinstance(node, ast.Constant):
        return node.value
    if isinstance(node, ast.BinOp):
        trai = danh_gia(node.left)
        phai = danh_gia(node.right)
        if isinstance(node.op, ast.Add):
            return trai + phai
        if isinstance(node.op, ast.Mult):
            return trai * phai
    raise ValueError("chưa hiểu")

cay = ast.parse("(4 + 5) * 2", mode="eval").body
print("máy 2 (cây AST):", danh_gia(cay))
```

```text title=readonly
máy 1 (danh sách lệnh): 18
máy 2 (cây AST): 18
```

Cùng phép tính `(4 + 5) * 2 = 18`, hai máy khác hẳn nhau về HÌNH THỨC
đầu vào (danh sách phẳng và cây lồng nhau) và khác hẳn về cách "bước"
(vòng `for` đi từng lệnh, hàm đệ quy đi xuống từng nhánh) — nhưng CÙNG
một khuôn: nhận cấu trúc, áp luật cố định, ra kết quả. Cả hai đều là máy
trừu tượng, chỉ đơn giản hoá theo hai cách khác nhau.
::::

::::predict{#tranh-luan-voi-dong-nghiep commitOnce}
Một đồng nghiệp nói: "Bộ đánh giá của bạn không đáng gọi là MÁY. Máy
thật (CPython) có ngăn xếp tính toán riêng, có bytecode, có thanh ghi —
bộ đánh giá của bạn chỉ là một hàm đệ quy bình thường."

**Trước khi đọc đáp án**, bạn phản biện thế nào cho ĐÚNG theo định nghĩa
máy trừu tượng vừa học?

:::opt{correct}
Một máy trừu tượng không cần có MỌI chi tiết của một máy thật cụ thể —
nó chỉ cần nhận một cấu trúc mô tả phép tính và áp luật cố định để ra
kết quả; bộ đánh giá làm đúng việc đó, chỉ đơn giản hơn CPython thật
:::

:::opt
Đồng nghiệp nói đúng — bộ đánh giá không phải là một máy, vì nó thiếu
ngăn xếp tính toán và bytecode riêng
::why
Gần đúng ở chỗ bạn quan sát đúng: bộ đánh giá THẬT SỰ không có ngăn xếp
tính toán riêng, cũng không có bytecode — đúng như phần explain vừa nêu.

Chỗ lệch: có những chi tiết đó không phải ĐIỀU KIỆN để được gọi là máy.
Định nghĩa máy trừu tượng chỉ đòi "nhận cấu trúc, áp luật cố định, ra
kết quả" — bộ đánh giá thoả đúng ba điều đó, nên nó VẪN là một máy, chỉ
là một máy ĐƠN GIẢN HƠN, không thiếu tư cách.
::
:::

:::opt
Đúng một nửa — bộ đánh giá chỉ đáng gọi là máy NẾU viết lại bằng vòng
`while` thay vì đệ quy, vì đệ quy không đếm là một "máy"
::why
Gần đúng ở việc bạn để ý cách VIẾT (đệ quy hay vòng lặp) là một khác
biệt thật giữa hai máy vừa xem trong ví dụ.

Chỗ lệch: cách viết — đệ quy hay vòng lặp — không nằm trong định nghĩa
máy trừu tượng. Cả hai đều là cách hợp lệ để "áp luật cố định lên cấu
trúc, ra kết quả". Bài sau sẽ cho thấy chính bộ đánh giá đệ quy này VẪN
có một trạng thái máy rõ ràng, dù không viết bằng `while`.
::
:::

:::opt
Sai — bộ đánh giá không hề nhận "cấu trúc mô tả phép tính" nào cả, nó
chỉ nhận vào các con số
::why
Gần đúng ở việc bạn để ý số THẬT SỰ có mặt ở đâu đó trong bộ đánh giá —
đúng, các con số nằm ở LÁ của cây.

Chỗ lệch: đầu vào của hàm `danh_gia` không phải một con số đơn lẻ — nó
là `node`, một nút `ast.AST`, có thể là cả một cây con lồng nhau nhiều
tầng (`ast.BinOp` chứa `ast.BinOp` chứa...). Chính cây đó — không phải
từng con số riêng lẻ — mới là cấu trúc mô tả phép tính mà máy nhận vào.
::
:::
::::

::::code{#chay-may-tren-nhieu-bieu-thuc}
Bộ đánh giá bốn phép toán (đã hoàn thiện ở cụm trước) được gói lại thành
một hàm `chay_may` — đúng tinh thần "một máy nhận cấu trúc, ra kết quả".
Chạy nó trên BỐN biểu thức khác nhau, không tính tay: gọi thẳng
`chay_may(bt)` cho từng biểu thức trong `bieu_thuc`.

```python title=starter
import ast

def danh_gia(node):
    if isinstance(node, ast.Constant):
        return node.value
    if isinstance(node, ast.BinOp):
        trai = danh_gia(node.left)
        phai = danh_gia(node.right)
        if isinstance(node.op, ast.Add):
            return trai + phai
        if isinstance(node.op, ast.Sub):
            return trai - phai
        if isinstance(node.op, ast.Mult):
            return trai * phai
        if isinstance(node.op, ast.Div):
            return trai / phai
        raise ValueError(f"máy này chưa hiểu phép toán {type(node.op).__name__}")
    raise ValueError(f"máy này chưa hiểu loại nút {type(node).__name__}")

def chay_may(chuoi_bieu_thuc):
    cay = ast.parse(chuoi_bieu_thuc, mode="eval")
    return danh_gia(cay.body)

bieu_thuc = ["2 + 3 * 4", "(2 + 3) * 4", "10 - 4 / 2", "100 / 4 - 5"]

ket_qua_may = []
for bt in bieu_thuc:
    ket_qua_may.append(___)          # chạy chay_may(bt), không tính tay

ket_qua_tinh_truc_tiep = [2 + 3 * 4, (2 + 3) * 4, 10 - 4 / 2, 100 / 4 - 5]

print(ket_qua_may == ket_qua_tinh_truc_tiep)
print(ket_qua_may)
```

```python title=solution
import ast

def danh_gia(node):
    if isinstance(node, ast.Constant):
        return node.value
    if isinstance(node, ast.BinOp):
        trai = danh_gia(node.left)
        phai = danh_gia(node.right)
        if isinstance(node.op, ast.Add):
            return trai + phai
        if isinstance(node.op, ast.Sub):
            return trai - phai
        if isinstance(node.op, ast.Mult):
            return trai * phai
        if isinstance(node.op, ast.Div):
            return trai / phai
        raise ValueError(f"máy này chưa hiểu phép toán {type(node.op).__name__}")
    raise ValueError(f"máy này chưa hiểu loại nút {type(node).__name__}")

def chay_may(chuoi_bieu_thuc):
    cay = ast.parse(chuoi_bieu_thuc, mode="eval")
    return danh_gia(cay.body)

bieu_thuc = ["2 + 3 * 4", "(2 + 3) * 4", "10 - 4 / 2", "100 / 4 - 5"]

ket_qua_may = []
for bt in bieu_thuc:
    ket_qua_may.append(chay_may(bt))

ket_qua_tinh_truc_tiep = [2 + 3 * 4, (2 + 3) * 4, 10 - 4 / 2, 100 / 4 - 5]

print(ket_qua_may == ket_qua_tinh_truc_tiep)
print(ket_qua_may)
```

```python title=test
assert ket_qua_may == ket_qua_tinh_truc_tiep, f"ket_qua_may (chạy MÁY) phải khớp ket_qua_tinh_truc_tiep (tính tay Python) — đang lệch: ket_qua_may={ket_qua_may}"
assert ket_qua_may == [14, 20, 8.0, 20.0], f"bốn kết quả phải là [14, 20, 8.0, 20.0] — đang ra {ket_qua_may}"
```

:::hints
- kind: attention
  body: Chỗ trống nằm bên trong .append(...) — gọi đúng hàm chay_may đã định nghĩa ở trên, với bt (biểu thức của vòng lặp hiện tại) làm đối số.
- kind: strategy
  body: 'chay_may nhận một chuỗi biểu thức, tự parse rồi đánh giá — đúng việc bạn cần cho từng bt. Viết ket_qua_may.append(chay_may(bt)), không tính tay kết quả rồi gõ số vào.'
- kind: one-line
  body: 'Chỗ trống là: chay_may(bt)'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: chỗ trống phải THẬT SỰ gọi chay_may(bt) — đây là bài học về việc MỘT MÁY chạy trên nhiều đầu vào, không phải bài tính tay; gõ cứng bốn con số hay gọi thẳng danh_gia bỏ qua chay_may không thoả đúng đề
  requireAst:
  # min: 1 — đếm thật trên solution: chay_may xuất hiện 2 lần trong mã
  # nguồn (dòng def chay_may(...): — đây là ĐỊNH NGHĨA, không phải lời
  # gọi, uses-call không đếm — và đúng một lời GỌI chay_may(bt) ở chỗ
  # trống). Không có lời gọi chay_may nào khác trong khung. ĐÃ THỬ THẬT
  # bằng cả ba cách True/1/0: vòng for chạy trên bieu_thuc (list 4 phần
  # tử) cố định, không phụ thuộc chỗ trống — không cách nào chạy vô hạn,
  # cả ba dừng ngay. Cả ba đều làm static này trượt (0 lời gọi chay_may
  # < min 1), VÀ độc lập với static, ket_qua_may thành [True,True,True,True]
  # (hoặc [1,1,1,1]/[0,0,0,0]) != [14,20,8.0,20.0] nên tests/output cũng
  # bắt được độc lập.
  - kind: uses-call, target: chay_may, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^True\\n\\[14, 20, 8\\.0, 20\\.0\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một hàm, bốn đầu vào khác nhau, bốn kết quả đúng — không phép tính
nào bạn gõ tay. Đó chính là việc một MÁY làm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Con trỏ lệnh của máy thật (T3.4 bài 7) là một con số, luôn cho biết máy
đang đứng ở LỆNH NÀO trong danh sách lệnh phẳng. Bộ đánh giá đệ quy của
bạn không có con trỏ lệnh nào cả — không biến nào giữ "vị trí hiện tại"
tường minh như vậy.

Vậy máy trừu tượng của bạn biết nó "đang ở đâu trong cây" bằng cách nào?
Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
