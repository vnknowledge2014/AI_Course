---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.may-ngan-xep-chay-khong-can-de-quy
title: "Máy ngăn xếp chạy danh sách lệnh phẳng — không cần đệ quy nữa"
summary: "Bộ đánh giá bài 11-17 ĐỆ QUY xuống cây, một khung cho mỗi tầng. Máy ngăn xếp chạy danh sách lệnh phẳng bài 18 chỉ cần MỘT vòng lặp, đẩy/lấy trên MỘT ngăn xếp — không hàm nào gọi hàm nào. Hai cách đánh giá cùng một biểu thức, khác hẳn cơ chế."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ngu.stack-machine-no-recursion]
requires: [ngu.compile-tree-to-stack-instructions]
concepts: [ngu.stack-machine-no-recursion]
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
Chỉ cần một thứ: một vòng lặp, đi qua từng lệnh, đẩy rồi lấy trên MỘT
ngăn xếp. Không hàm nào gọi hàm nào nữa cả.
::::

::::explain{#khong-can-goi-lai-chinh-no}
Bộ đánh giá cây (bài 11-17) tính bằng ĐỆ QUY: mỗi lần gặp `ast.BinOp`,
hàm `danh_gia` GỌI LẠI CHÍNH NÓ hai lần — một cho nhánh trái, một cho
nhánh phải. Mỗi lời gọi đặt thêm một KHUNG lên ngăn xếp cuộc gọi hàm
(bài 17) — cây càng sâu, càng nhiều khung chồng lên nhau cùng lúc.

Danh sách lệnh phẳng bài 18 tạo ra không cần cách đó. Đọc lại chính bảy
lệnh của `"(5 - 1) * 2 + 8"`: `("DAY", 5), ("DAY", 1), ("TRU",), ...` —
đây là một DÃY, đọc từ trái sang phải, KHÔNG lồng nhau. Một vòng `for`
đơn giản — đúng chu trình lấy-hiểu-làm T3.4 bài 6 đã dạy — đi qua từng
lệnh MỘT LẦN, thao tác lên MỘT ngăn xếp tính toán duy nhất (bài 3 T3.4:
`DAY` ĐẨY, `CONG`/`TRU`/`NHAN`/`CHIA` LẤY hai ra tính rồi ĐẨY kết quả
lại) là đủ. **Không có lệnh nào trong vòng lặp đó gọi lại chính vòng lặp
— không hàm nào gọi hàm nào cả.**

Hệ quả: dù biểu thức gốc có bao nhiêu tầng ngoặc lồng nhau, ngăn xếp
cuộc gọi hàm của Python (bài 17) trong lúc `chay_may` chạy LUÔN chỉ có
đúng MỘT khung — khung của chính `chay_may`. Độ sâu của biểu thức không
còn làm chồng lời gọi phình to nữa; nó chỉ làm danh sách lệnh DÀI ra,
và một vòng `for` đi qua một danh sách dài không tốn thêm khung nào cả.
::::

::::example{#theo-doi-ngan-xep-tung-buoc}
Chạy máy ngăn xếp trên bảy lệnh của `"(5 - 1) * 2 + 8"` (bài 18 biên
dịch ra), in ngăn xếp SAU mỗi lệnh:

```python title=readonly
danh_sach_lenh = [("DAY", 5), ("DAY", 1), ("TRU",), ("DAY", 2), ("NHAN",), ("DAY", 8), ("CONG",)]

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
    print(lenh, "->", ngan_xep)

print("kết quả cuối:", ngan_xep[-1])
```

```text title=readonly
('DAY', 5) -> [5]
('DAY', 1) -> [5, 1]
('TRU',) -> [4]
('DAY', 2) -> [4, 2]
('NHAN',) -> [8]
('DAY', 8) -> [8, 8]
('CONG',) -> [16]
kết quả cuối: 16
```

Đúng `(5 - 1) * 2 + 8 = 4 * 2 + 8 = 8 + 8 = 16`. Nhìn `ngan_xep` phình
tới hai phần tử (`[5, 1]`, `[4, 2]`, `[8, 8]`) rồi co lại còn một sau
mỗi lệnh toán tử — không có khung hàm nào xuất hiện ở đây, chỉ một biến
`list` duy nhất, bạn tự `.append`/`.pop`, đúng như bài "Duyệt cây" T3.2
đã làm với ngăn xếp tự quản, không phải như đệ quy T3.3 vẫn giấu nó.
::::

::::predict{#truoc-luc-chia-chay commitOnce}
Danh sách lệnh của `"10 - 4 / 2"` (bài 18):
`[("DAY", 10), ("DAY", 4), ("DAY", 2), ("CHIA",), ("TRU",)]`.

**Trước khi chạy máy**, ngay TRƯỚC KHI lệnh `("CHIA",)` (lệnh thứ tư)
chạy, ngăn xếp đang chứa bao nhiêu giá trị, và đó là những giá trị nào?

:::opt{correct}
Ba giá trị, theo đúng thứ tự đẩy vào: `[10, 4, 2]`
:::

:::opt
Hai giá trị: `[4, 2]` — vì phép chia chỉ cần đúng hai toán hạng đó
::why
Gần đúng ở việc bạn nhận ra `CHIA` sẽ LẤY đúng hai giá trị (`4` và `2`)
— đúng, đó là hai giá trị nó dùng.

Chỗ lệch: ba lệnh `DAY` đầu tiên đều đã chạy TRƯỚC khi `CHIA` tới lượt —
`("DAY", 10)` đẩy `10` vào trước cả `4` và `2`, và chưa lệnh nào LẤY nó
ra cả. `10` vẫn còn nằm YÊN dưới đáy ngăn xếp, chờ tới lượt `TRU` (lệnh
thứ năm) mới được dùng tới.
::
:::

:::opt
Một giá trị: `[10]` — vì `DAY 4` và `DAY 2` chỉ là bước chuẩn bị, chưa
tính vào ngăn xếp thật sự
::why
Gần đúng ở việc bạn nhớ đúng `10` được đẩy vào ĐẦU TIÊN.

Chỗ lệch: `DAY` không phải "bước chuẩn bị" tách biệt — mỗi lần `DAY`
chạy, nó THẬT SỰ đẩy một giá trị vào đỉnh ngăn xếp, y hệt `.append()`.
Ba lệnh `DAY` liên tiếp đẩy đủ BA giá trị vào, không phải một.
::
:::

:::opt
Ba giá trị, nhưng thứ tự ngược lại: `[2, 4, 10]`
::why
Gần đúng ở việc bạn đếm đúng SỐ LƯỢNG — ba giá trị, không thiếu không
thừa.

Chỗ lệch: `.append()` luôn thêm vào ĐUÔI danh sách, không phải đầu. Đẩy
`10` trước, `4` sau, `2` sau cùng cho ra `[10, 4, 2]` — `10` nằm ở vị
trí ĐẦU (đáy ngăn xếp), `2` nằm ở vị trí CUỐI (đỉnh ngăn xếp), không
phải đảo ngược.
::
:::
::::

::::code{#hoan-tat-lenh-chia}
Máy ngăn xếp đã hiểu `DAY`, `CONG`, `TRU`, `NHAN` — còn thiếu đúng một
lệnh: `CHIA`. Viết nhánh đó, giống hệt cách `TRU`/`NHAN` đã làm, chỉ
khác phép tính. Sau đó, ghép với `bien_dich` (bài 18, đã viết sẵn) để
biên dịch VÀ chạy trọn một biểu thức mới.

```python title=starter
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
            b = ngan_xep.pop(); a = ngan_xep.pop(); ngan_xep.append(___)   # đẩy kết quả a CHIA b
        else:
            raise ValueError(f"máy chưa hiểu lệnh {ten}")
    return ngan_xep.pop()

bieu_thuc = "100 / 4 - 5"
cay = ast.parse(bieu_thuc, mode="eval").body
ket_qua_may = chay_may(bien_dich(cay))
ket_qua_that = 100 / 4 - 5

print(ket_qua_may == ket_qua_that)
print(ket_qua_may)
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
        else:
            raise ValueError(f"máy chưa hiểu lệnh {ten}")
    return ngan_xep.pop()

bieu_thuc = "100 / 4 - 5"
cay = ast.parse(bieu_thuc, mode="eval").body
ket_qua_may = chay_may(bien_dich(cay))
ket_qua_that = 100 / 4 - 5

print(ket_qua_may == ket_qua_that)
print(ket_qua_may)
```

```python title=test
assert ket_qua_may == ket_qua_that, f"ket_qua_may (máy ngăn xếp) phải khớp ket_qua_that (100 / 4 - 5 tính trực tiếp) — đang lệch: ket_qua_may={ket_qua_may}"
assert ket_qua_may == 20.0, f"100 / 4 - 5 phải là 25.0 - 5 = 20.0 — đang ra {ket_qua_may}"
```

:::hints
- kind: attention
  body: Chỗ trống chỉ thiếu MỘT phép tính — chia a cho b, rồi đẩy kết quả vào ngăn xếp. Nhìn đúng nhánh TRU hoặc NHAN ngay phía trên để thấy khuôn.
- kind: strategy
  body: 'Nhánh CHIA đã lấy sẵn b (đỉnh) rồi a (kế đỉnh) ra khỏi ngăn xếp — giống hệt ba nhánh trước. Việc còn lại chỉ là tính a / b (a chia cho b, đúng thứ tự — phép chia không đổi chỗ được) và đẩy kết quả đó vào bằng ngan_xep.append(...).'
- kind: one-line
  body: 'Chỗ trống là: a / b'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: nhánh CHIA phải THẬT SỰ chia a cho b — không gõ cứng kết quả, không để một câu không làm gì (True/1/0) thay cho phép chia
  requireAst:
  # Đếm thật trên toàn bộ solution: "/" (BinOp Div) xuất hiện 2 lần — một
  # lần sẵn có trong khung ở dòng `ket_qua_that = 100 / 4 - 5`, cộng đúng
  # một lần ở chỗ trống (a / b) -> min: 2. Tên "a" được ĐỌC 4 lần (nhánh
  # CONG, TRU, NHAN mỗi nhánh 1 lần sẵn có, cộng 1 lần ở chỗ trống) -> min:
  # 4; "b" tương tự -> min: 4. ĐÃ THỬ THẬT bằng cả ba cách True/1/0: vòng
  # for chạy trên đúng 5 lệnh cố định (độ dài danh_sach_lenh không phụ
  # thuộc chỗ trống), không cách nào chạy vô hạn — cả ba dừng ngay. Cả ba
  # còn "/" chỉ 1 lần (<2), "a" và "b" chỉ 3 lần mỗi tên (<4) — static
  # trượt độc lập; và ket_qua_may thành -4/-4/-5 (không phải 20.0), khác
  # ket_qua_that — assert cũng trượt độc lập với luật static này.
  - kind: uses-operator, target: "/", min: 2
  - kind: uses-name, target: a, min: 4
  - kind: uses-name, target: b, min: 4
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^True\\n20\\.0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm lệnh, một vòng `for`, một ngăn xếp — không lời gọi hàm nào lồng vào
lời gọi hàm nào. Cùng kết quả với bộ đánh giá cây, khác hẳn cách tới đó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn có ĐÚNG HAI cách đánh giá cùng một biểu thức: bộ đánh giá cây
đệ quy (bài 11-17) và máy ngăn xếp chạy danh sách lệnh phẳng (bài 18-19)
vừa xong. Cả hai đều chạy trên chính `"100 / 4 - 5"` và ra cùng `20.0`.

Nhưng "ra cùng kết quả trên MỘT biểu thức" chưa chứng minh được gì
nhiều. Nếu chạy CẢ HAI cách trên NHIỀU biểu thức khác nhau, chúng có
LUÔN khớp nhau không — và hai cách này thật ra ĐÁNH ĐỔI cái gì, nếu
không phải chỉ là "hai cách viết cho vui"? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
