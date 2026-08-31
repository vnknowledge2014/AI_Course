---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.so-sanh-hai-cach-danh-gia
title: "So sánh hai cách đánh giá: đệ quy trên cây và máy ngăn xếp"
summary: "Chạy CÙNG nhiều biểu thức qua cả bộ đánh giá cây đệ quy lẫn máy ngăn xếp, xác nhận cùng kết quả, rồi nói ra đánh đổi thật: cây đệ quy dễ đọc/dễ viết hơn; máy ngăn xếp không đụng trần đệ quy (T3.4 bài 26) vì vòng lặp chạy nó không có lời gọi hàm nào lồng nhau."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ngu.compare-eval-strategies]
requires: [ngu.stack-machine-no-recursion, ngu.tree-walk-eval]
concepts: [ngu.compare-eval-strategies]
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
Khớp trên một biểu thức chưa nói lên nhiều. Khớp trên NHIỀU biểu thức
khác nhau, rồi nói thẳng ra hai cách này thật sự đánh đổi cái gì.
::::

::::explain{#hai-cach-mot-danh-doi}
Bạn có đúng hai cách đánh giá biểu thức, cả hai đã tự tay viết:

- `danh_gia` (bài 11-17): đệ quy THẲNG trên cây — một hàm, mỗi loại nút
  một nhánh xử lý, ánh xạ trực tiếp cấu trúc `if isinstance(...)` lên
  hình dạng cây.
- `bien_dich` + `chay_may` (bài 18-19): biên dịch cây thành danh sách
  lệnh phẳng, rồi CHẠY danh sách đó bằng một vòng `for` cộng một ngăn
  xếp tính toán.

Chạy cả hai trên CÙNG một biểu thức luôn cho CÙNG một kết quả — không
phải tình cờ, mà vì cả hai đều tuân đúng một luật: nhánh phải nhân trước
cộng sau (bài 8), ngoặc đổi hình dạng cây (bài 9), mỗi loại nút chỉ một
cách xử lý cố định (bài 13-14). Hai cách khác nhau, cùng một NGỮ NGHĨA.

Nhưng chúng đánh đổi thật, ở hai chỗ:

1. **Dễ đọc, dễ viết** — `danh_gia` thắng. Đọc thẳng một hàm là đọc
   được toàn bộ luật của ngôn ngữ nhỏ này: gặp `Add` thì cộng, gặp
   `Mult` thì nhân. `bien_dich`/`chay_may` cần HAI hàm, và để hiểu
   `chay_may` đang làm gì tại một thời điểm, phải tưởng tượng ra trạng
   thái ngăn xếp tính toán (bài 19) — thêm một tầng gián tiếp.

2. **Không đụng trần đệ quy** — `chay_may` thắng. Vòng `for` của nó đi
   qua từng lệnh, KHÔNG gọi lại chính `chay_may` cho mỗi lệnh — số khung
   Python cần cho `chay_may` LUÔN LÀ MỘT, bất kể danh sách lệnh dài bao
   nhiêu (bài 19). Còn `danh_gia` GỌI LẠI CHÍNH NÓ mỗi khi xuống một
   tầng cây (bài 17) — cây càng lồng sâu, càng nhiều khung chồng lên
   nhau, và có thể chạm mức trần `sys.getrecursionlimit()` (đo được là
   `1000` trên máy của khoá này, T3.4 bài 26), ném `RecursionError`,
   trong khi `chay_may` — CHẠY trên một danh sách lệnh đã có sẵn — không
   hề đụng tới giới hạn đó, dù danh sách dài cỡ nào.

Không có cách nào "tốt hơn tuyệt đối" — mỗi cách thắng ở một tiêu chí
khác nhau. Đây là một đánh đổi kinh điển của khoa học máy tính: dễ hiểu
đối lại khả năng chịu quy mô, và bạn vừa tự tay dựng cả hai vế để thấy
nó thật, không phải nghe kể.
::::

::::example{#chay-ca-hai-tren-nhieu-bieu-thuc}
Cả hai hàm (đã hoàn thiện ở các bài trước), chạy trên năm biểu thức
khác nhau, đối chiếu từng cặp kết quả:

```python title=readonly
import ast

def danh_gia(node):
    if isinstance(node, ast.Constant):
        return node.value
    trai = danh_gia(node.left)
    phai = danh_gia(node.right)
    if isinstance(node.op, ast.Add):
        return trai + phai
    if isinstance(node.op, ast.Sub):
        return trai - phai
    if isinstance(node.op, ast.Mult):
        return trai * phai
    return trai / phai

def bien_dich(node):
    if isinstance(node, ast.Constant):
        return [("DAY", node.value)]
    lenh_trai = bien_dich(node.left)
    lenh_phai = bien_dich(node.right)
    ten = {ast.Add: "CONG", ast.Sub: "TRU", ast.Mult: "NHAN", ast.Div: "CHIA"}[type(node.op)]
    return lenh_trai + lenh_phai + [(ten,)]

def chay_may(lenh_list):
    ngan_xep = []
    for lenh in lenh_list:
        if lenh[0] == "DAY":
            ngan_xep.append(lenh[1])
        else:
            b = ngan_xep.pop(); a = ngan_xep.pop()
            phep = {"CONG": a + b, "TRU": a - b, "NHAN": a * b, "CHIA": a / b}
            ngan_xep.append(phep[lenh[0]])
    return ngan_xep.pop()

bieu_thuc = ["2 + 3 * 4", "(2 + 3) * 4", "10 - 4 / 2", "(5 - 1) * 2 + 8", "100 / 4 - 5"]
for bt in bieu_thuc:
    cay = ast.parse(bt, mode="eval").body
    ket_qua_cay = danh_gia(cay)
    ket_qua_may = chay_may(bien_dich(cay))
    print(bt, "-> cây:", ket_qua_cay, "| máy:", ket_qua_may, "| khớp:", ket_qua_cay == ket_qua_may)
```

```text title=readonly
2 + 3 * 4 -> cây: 14 | máy: 14 | khớp: True
(2 + 3) * 4 -> cây: 20 | máy: 20 | khớp: True
10 - 4 / 2 -> cây: 8.0 | máy: 8.0 | khớp: True
(5 - 1) * 2 + 8 -> cây: 16 | máy: 16 | khớp: True
100 / 4 - 5 -> cây: 20.0 | máy: 20.0 | khớp: True
```

Năm biểu thức, năm lần khớp — không phải may mắn trên đúng một ca, mà
đúng trên MỌI ca thử được. Hai cơ chế khác hẳn nhau, cùng một ngữ nghĩa.
::::

::::predict{#ai-cham-tran-truoc commitOnce}
Byte đã CÓ SẴN (biên dịch từ trước, không tính bước biên dịch) một danh
sách lệnh phẳng RẤT DÀI — tương ứng một biểu thức có mười nghìn tầng
ngoặc lồng nhau. Nếu thay vì chạy `chay_may` trên danh sách đó, Byte lại
đánh giá ĐÚNG cây gốc (mười nghìn tầng) bằng `danh_gia` (đệ quy):

**Trước khi đọc đáp án**, cách nào trong hai cách SẼ báo `RecursionError`
trên biểu thức mười nghìn tầng này, cách nào KHÔNG?

:::opt{correct}
`danh_gia` (đệ quy trên cây) sẽ báo `RecursionError` — mười nghìn tầng
vượt xa mức trần `1000` (T3.4 bài 26), mỗi tầng tốn thêm một khung.
Chạy `chay_may` trên danh sách lệnh ĐÃ CÓ SẴN thì KHÔNG báo lỗi này, vì
vòng `for` của nó không gọi lại chính `chay_may` cho mỗi lệnh
:::

:::opt
Cả hai đều báo `RecursionError` như nhau, vì cả hai cùng xử lý một biểu
thức có cùng "độ sâu logic" mười nghìn tầng
::why
Gần đúng ở việc bạn nhận ra "mười nghìn tầng" là một con số LỚN, đáng lo
— đúng, nó lớn hơn nhiều so với mức trần.

Chỗ lệch: "độ sâu logic" của biểu thức gốc không TỰ ĐỘNG biến thành số
khung của `chay_may`. Vòng `for` của nó đi qua TỪNG LỆNH một, không GỌI
LẠI chính hàm `chay_may` cho mỗi lệnh — số khung Python cần cho
`chay_may` LUÔN LÀ MỘT, bất kể danh sách lệnh dài mười, hay mười nghìn
lệnh.
::
:::

:::opt
`chay_may` sẽ báo lỗi trước, vì danh sách mười nghìn lệnh dài hơn nhiều
so với mức trần đệ quy `1000`
::why
Gần đúng ở việc bạn so sánh đúng hai con số — mười nghìn LỚN HƠN một
nghìn thật.

Chỗ lệch: `sys.getrecursionlimit()` giới hạn SỐ KHUNG trên ngăn xếp GỌI
HÀM — không giới hạn ĐỘ DÀI của một `list` Python. Một `list` mười
nghìn, hay một triệu phần tử là chuyện hoàn toàn bình thường, chỉ giới
hạn bởi RAM thật (T3.4 bài 26 đã phân biệt rõ: ngăn xếp có trần cố
định, đống thì không).
::
:::

:::opt
Không cách nào biết trước — còn tuỳ máy tính đang chạy nhanh hay chậm
lúc đó
::why
Gần đúng ở sự thận trọng khi không chắc chắn — thái độ đó hợp lý với
nhiều loại lỗi khác.

Chỗ lệch: `RecursionError` không phải lỗi về TỐC ĐỘ. Nó là một luật CỐ
ĐỊNH — đếm số khung đang mở trên ngăn xếp gọi hàm, so với
`sys.getrecursionlimit()`, một con số CẤU HÌNH, không đổi theo CPU nhanh
hay chậm. Với một cây có độ sâu cho trước, kết quả (lỗi hay không) LUÔN
giống nhau, dù chạy trên máy nào.
::
:::
::::

::::code{#doi-chieu-hang-loat}
Chạy cả `danh_gia` VÀ `chay_may` (qua `bien_dich`) trên NĂM biểu thức,
đếm xem có bao nhiêu cặp khớp nhau. Cả hai hàm đánh giá đã viết sẵn —
việc của bạn: gọi đúng máy ngăn xếp cho mỗi biểu thức trong vòng lặp.

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
        raise ValueError(f"chưa hiểu phép toán {type(node.op).__name__}")
    raise ValueError(f"chưa hiểu loại nút {type(node).__name__}")

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

def chay_may(danh_sach_lenh):
    ngan_xep = []
    for lenh in danh_sach_lenh:
        ten = lenh[0]
        if ten == "DAY":
            ngan_xep.append(lenh[1])
        elif ten == "CONG":
            b = ngan_xep.pop(); a = ngan_xep.pop(); ngan_xep.append(a + b)
        elif ten == "TRU":
            b = ngan_xep.pop(); a = ngan_xep.pop(); ngan_xep.append(a - b)
        elif ten == "NHAN":
            b = ngan_xep.pop(); a = ngan_xep.pop(); ngan_xep.append(a * b)
        elif ten == "CHIA":
            b = ngan_xep.pop(); a = ngan_xep.pop(); ngan_xep.append(a / b)
    return ngan_xep.pop()

bieu_thuc = ["2 + 3 * 4", "(2 + 3) * 4", "10 - 4 / 2", "(5 - 1) * 2 + 8", "100 / 4 - 5"]

so_khop = 0
for bt in bieu_thuc:
    cay = ast.parse(bt, mode="eval").body
    ket_qua_cay = danh_gia(cay)
    ket_qua_may = ___                     # chạy máy ngăn xếp trên cùng cây: chay_may(bien_dich(cay))
    if ket_qua_cay == ket_qua_may:
        so_khop += 1

print(so_khop)
print(so_khop == len(bieu_thuc))
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
        raise ValueError(f"chưa hiểu phép toán {type(node.op).__name__}")
    raise ValueError(f"chưa hiểu loại nút {type(node).__name__}")

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

def chay_may(danh_sach_lenh):
    ngan_xep = []
    for lenh in danh_sach_lenh:
        ten = lenh[0]
        if ten == "DAY":
            ngan_xep.append(lenh[1])
        elif ten == "CONG":
            b = ngan_xep.pop(); a = ngan_xep.pop(); ngan_xep.append(a + b)
        elif ten == "TRU":
            b = ngan_xep.pop(); a = ngan_xep.pop(); ngan_xep.append(a - b)
        elif ten == "NHAN":
            b = ngan_xep.pop(); a = ngan_xep.pop(); ngan_xep.append(a * b)
        elif ten == "CHIA":
            b = ngan_xep.pop(); a = ngan_xep.pop(); ngan_xep.append(a / b)
    return ngan_xep.pop()

bieu_thuc = ["2 + 3 * 4", "(2 + 3) * 4", "10 - 4 / 2", "(5 - 1) * 2 + 8", "100 / 4 - 5"]

so_khop = 0
for bt in bieu_thuc:
    cay = ast.parse(bt, mode="eval").body
    ket_qua_cay = danh_gia(cay)
    ket_qua_may = chay_may(bien_dich(cay))
    if ket_qua_cay == ket_qua_may:
        so_khop += 1

print(so_khop)
print(so_khop == len(bieu_thuc))
```

```python title=test
assert so_khop == 5, f"cả năm biểu thức phải khớp giữa hai cách đánh giá — đang chỉ khớp {so_khop}"
assert so_khop == len(bieu_thuc), "so_khop phải đúng bằng số biểu thức đã thử, không thiếu ca nào"
```

:::hints
- kind: attention
  body: Chỗ trống phải chạy MÁY NGĂN XẾP — ghép hai hàm đã có sẵn: bien_dich (biên dịch cây thành lệnh) rồi chay_may (chạy lệnh đó).
- kind: strategy
  body: 'cay đã có sẵn ở dòng trên. Biên dịch nó bằng bien_dich(cay), rồi đưa thẳng kết quả đó cho chay_may(...) để chạy. Viết gộp một dòng: chay_may(bien_dich(cay)).'
- kind: one-line
  body: 'Chỗ trống là: chay_may(bien_dich(cay))'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: chỗ trống phải THẬT SỰ gọi chay_may(bien_dich(cay)) — dùng đúng cả hai hàm máy ngăn xếp đã học, không gọi lại danh_gia hay gõ cứng kết quả
  requireAst:
  # Đếm thật trên solution: chay_may xuất hiện 2 lần trong mã nguồn — 1 lần
  # là ĐỊNH NGHĨA (def chay_may(...), không phải Call, uses-call không đếm)
  # và đúng 1 lần là LỜI GỌI, ở chỗ trống -> min: 1. bien_dich tương tự:
  # xuất hiện 3 lần trong mã nguồn — định nghĩa (không đếm) + 2 lần GỌI ĐỆ
  # QUY bên trong chính nó (bien_dich(node.left), bien_dich(node.right),
  # đã có sẵn trong khung) + 1 lần gọi ở chỗ trống -> tổng lời gọi = 3,
  # min: 3 (2 sẵn có + 1 mới). ĐÃ THỬ THẬT bằng cả ba cách True/1/0: vòng
  # for chạy trên bieu_thuc (list 5 phần tử) cố định, không phụ thuộc chỗ
  # trống — không cách nào chạy vô hạn, cả ba dừng ngay. Cả ba giữ nguyên
  # lời gọi chay_may ở mức 0 (<1) và bien_dich ở mức 2 (<3) — static trượt
  # độc lập; và so_khop thành 0 (không phải 5, vì True/1/0 không khớp bất
  # kỳ kết quả cây nào trong 14/20/8.0/16/20.0) — assert cũng trượt độc
  # lập với luật static này.
  - kind: uses-call, target: chay_may, min: 1
  - kind: uses-call, target: bien_dich, min: 3
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^5\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm biểu thức, năm lần khớp — không phải may mắn. Cây đệ quy dễ đọc
hơn; máy ngăn xếp không đụng trần đệ quy. Cả hai đều thật, cả hai đều là
của bạn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Tới giờ, MỌI biểu thức đánh giá được — dù qua cây hay qua máy ngăn xếp —
chỉ chứa SỐ. Không tên nào cả. Nhưng một "máy tính bỏ túi" thật cần hiểu
được cả những biểu thức như `x + 3`, nơi `x` là một cái TÊN, không phải
một con số viết sẵn.

Cả hai máy của bạn — cây đệ quy lẫn ngăn xếp — hiện chưa biết `x` là gì.
Chúng cần thêm THỨ GÌ để biết một cái tên như `x` đang GIỮ giá trị nào?
Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
