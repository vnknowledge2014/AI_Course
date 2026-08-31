---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.boss-ngon-ngu-nho-cua-byte
title: "BOSS — Ngôn ngữ nhỏ của Byte"
summary: "Khép cả track: đọc một dòng chữ, tokenize, dựng cây, GÁN biến, đánh giá, báo lỗi rõ ràng khi sai — một máy tính bỏ túi nhỏ, tự tay xây từ con số 0, không còn chỉ quan sát một trình thông dịch có sẵn."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ngu.mini-language]
requires: [ngu.combined-pipeline]
concepts: [ngu.mini-language]
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

::::byte{trigger=enter mood=happy pose=jump}
Hai mươi ba bài để tới đây. Bài cuối cùng của track — ráp mảnh cuối,
và ngôn ngữ nhỏ của bạn thành hình trọn vẹn.
::::

::::explain{#manh-cuoi-la-gan}
`chay_bieu_thuc` của bài trước xử lý được BIỂU THỨC — thứ luôn có một
GIÁ TRỊ, như `gia * so_luong - giam_gia`. Nhưng nó không GHI được gì
cả. Mảnh còn thiếu là **gán**: `gia = 45000`, rồi dùng lại `gia` ở dòng
sau, không tính lại từ đầu.

Một câu gán có hình dạng cây KHÁC hẳn một biểu thức. `ast.parse(...,
mode="eval")` chỉ hiểu được BIỂU THỨC — cho nó một câu gán, nó từ chối
ngay (`gán` không phải một GIÁ TRỊ). Cần đổi sang chế độ `mode="exec"`,
chế độ hiểu được CÂU LỆNH, không chỉ biểu thức:

```python
import ast
print(ast.dump(ast.parse("gia = 45000", mode="exec"), indent=2))
```
```text
Module(
  body=[
    Assign(
      targets=[
        Name(id='gia', ctx=Store())],
      value=Constant(value=45000))])
```

Gốc của cây giờ là `Module`, chứa một danh sách `body` các CÂU LỆNH
(dù chỉ một dòng cũng vậy). Câu gán là một `ast.Assign`: `targets` là
nơi giá trị được CẤT vào (`ctx=Store()` — khác `ctx=Load()` bạn đã quen
từ `ast.Name` trong biểu thức), `value` là biểu thức bên phải dấu `=`,
y hệt những gì `danh_gia` đã biết tính. Còn một dòng KHÔNG có dấu `=`,
như `tong - 5000`, ở `mode="exec"` lại bọc trong `ast.Expr` — một câu
lệnh chỉ để TÍNH một biểu thức, không cất đi đâu cả:

```text
Module(
  body=[
    Expr(
      value=BinOp(
        left=Name(id='tong', ctx=Load()),
        op=Sub(),
        right=Constant(value=5000)))])
```

Vậy máy tính bỏ túi nhỏ của Byte xử lý mỗi dòng theo đúng hai ca đó:
gặp `Assign` thì tính vế phải rồi CẤT vào `moi_truong`; gặp `Expr` thì
chỉ tính và trả về, không cất. Và vì `moi_truong` là MỘT dict sống suốt
cả chương trình — không tạo mới mỗi dòng — dòng sau nhìn thấy được kết
quả dòng trước đã cất, đúng tinh thần T3.1 dạy: cái tên trỏ tới một chỗ
nhớ SỐNG LÂU HƠN một câu lệnh.
::::

::::example{#may-tinh-chay-that}
Ghép trọn: `danh_gia` (bài 11-22, giờ nhận thẳng biểu thức, không cần
bọc `ast.Expression` vì `mode="exec"` không tạo nút đó), `chay_dong`
(xử lý một dòng — gán hoặc tính), và `may_tinh_bo_tui` (chạy nhiều
dòng, giữ một `moi_truong` chung, dừng và báo lỗi rõ ràng khi gặp tên
lạ):

```python title=readonly
import ast

def danh_gia(node, moi_truong):
    if isinstance(node, ast.Constant):
        return node.value
    elif isinstance(node, ast.Name):
        if node.id not in moi_truong:
            raise NameError(f"tên {node.id!r} chưa được gán trong môi trường")
        return moi_truong[node.id]
    elif isinstance(node, ast.BinOp):
        trai = danh_gia(node.left, moi_truong)
        phai = danh_gia(node.right, moi_truong)
        if isinstance(node.op, ast.Add):
            return trai + phai
        elif isinstance(node.op, ast.Sub):
            return trai - phai
        elif isinstance(node.op, ast.Mult):
            return trai * phai
        elif isinstance(node.op, ast.Div):
            return trai / phai
        else:
            raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}")

def chay_dong(dong, moi_truong):
    cay = ast.parse(dong, mode="exec")
    lenh = cay.body[0]
    if isinstance(lenh, ast.Assign):
        ten = lenh.targets[0].id
        gia_tri = danh_gia(lenh.value, moi_truong)
        moi_truong[ten] = gia_tri
        return ten, gia_tri
    elif isinstance(lenh, ast.Expr):
        return None, danh_gia(lenh.value, moi_truong)
    else:
        raise ValueError(f"chưa hỗ trợ câu lệnh: {type(lenh).__name__}")

def may_tinh_bo_tui(chuong_trinh):
    moi_truong = {}
    nhat_ky = []
    for dong in chuong_trinh.strip().split("\n"):
        dong = dong.strip()
        if dong == "":
            continue
        try:
            ten, gia_tri = chay_dong(dong, moi_truong)
        except NameError as loi:
            nhat_ky.append(f"LỖI: {loi}")
            break
        if ten is None:
            nhat_ky.append(f"{dong} => {gia_tri}")
        else:
            nhat_ky.append(f"{ten} = {gia_tri}")
    return nhat_ky, moi_truong

chuong_trinh = "\n".join([
    "gia = 45000",
    "so_luong = 3",
    "tong = gia * so_luong",
    "tong - 5000",
])
nhat_ky, moi_truong = may_tinh_bo_tui(chuong_trinh)
print(nhat_ky)
print(moi_truong)
```

```text title=readonly
['gia = 45000', 'so_luong = 3', 'tong = 135000', 'tong - 5000 => 130000']
{'gia': 45000, 'so_luong': 3, 'tong': 135000}
```

Bốn dòng, bốn kết quả — ba dòng gán CẤT giá trị vào `moi_truong`, dòng
cuối chỉ TÍNH (`135000 - 5000 = 130000`), không cất gì. Không dòng nào
gọi `eval()`. Không nút nào của cây do một bộ tách chữ tự chế tạo ra —
`ast.parse` của chính CPython làm việc đó. Toàn bộ phần TÍNH đi qua
đúng một hàm đệ quy bạn tự viết, đi bộ trên cây, đúng cách bài 11 dạy.
::::

::::predict{#dung-o-dau commitOnce}
Chương trình ba dòng, dòng THỨ HAI dùng một tên chưa từng được gán:

```python
chuong_trinh = "\n".join([
    "gia = 10",
    "tong = gia * thue",
    "von = tong + 1",
])
nhat_ky, moi_truong = may_tinh_bo_tui(chuong_trinh)
```

**Trước khi chạy**, bạn đoán `nhat_ky` và `moi_truong` cuối cùng là gì?

:::opt{correct}
`nhat_ky` chỉ có hai phần tử: `'gia = 10'` và một dòng `'LỖI: ...'`
nhắc tên `thue`. `moi_truong` chỉ có `{'gia': 10}` — dòng thứ ba
(`von = ...`) không hề chạy
:::

:::opt
`nhat_ky` có đủ ba phần tử — hai dòng đầu chạy bình thường, dòng thứ ba
được ghi là "LỖI" vì nó dùng `tong`, mà `tong` tính sai
::why
Gần đúng ở việc bạn đúng là có LỖI xảy ra, và nó liên quan tới một tên
chưa gán — không sai ở phần đó.

Chỗ lệch: dòng `except NameError as loi: ... break` trong
`may_tinh_bo_tui` DỪNG HẲN vòng lặp `for` ngay khi bắt được lỗi ở dòng
thứ hai. Dòng thứ ba, `von = tong + 1`, chưa bao giờ được đưa vào
`chay_dong` — nó không "chạy và bị đánh dấu lỗi", nó KHÔNG CHẠY.
::
:::

:::opt
`moi_truong` rỗng hoàn toàn (`{}`) — vì có lỗi xảy ra thì mọi thứ đã
làm trước đó cũng bị coi là hỏng, không được giữ lại
::why
Gần đúng ở việc bạn cảm nhận một lỗi giữa chừng "làm hỏng" một điều gì
đó — cảm giác đó không sai ở một số ngôn ngữ có cơ chế hoàn tác.

Chỗ lệch: dòng `gia = 10` đã CHẠY XONG và đã GHI vào `moi_truong` TRƯỚC
KHI dòng thứ hai gây lỗi — không có bước nào "hoàn tác" (undo) việc đã
làm ở đây. `moi_truong` giữ nguyên `{'gia': 10}`, đúng phần đã thành
công trước khi lỗi xảy ra.
::
:::

:::opt
Chương trình vẫn tính ra `von`, dựa trên giá trị CŨ của `tong` từ một
lần chạy trước đó, vì `moi_truong` không bị xoá giữa các lần gọi
::why
Gần đúng ở tinh thần: bạn đang nghĩ đúng hướng "môi trường sống lâu
hơn một dòng lệnh" — chính bài này vừa dạy điều đó.

Chỗ lệch: "sống lâu hơn MỘT DÒNG" không có nghĩa là "sống qua NHIỀU LẦN
GỌI HÀM". `moi_truong = {}` được khởi tạo MỚI TINH ngay đầu mỗi lần
`may_tinh_bo_tui` chạy — không có giá trị cũ nào từ lần chạy trước để
dùng lại. Và vòng lặp đã dừng ở dòng hai (`break`) trước khi tới được
dòng ba, nên câu hỏi "tính `von` bằng gì" còn chưa có cơ hội đặt ra.
::
:::
::::

::::code{#rap-may-tinh-bo-tui}
Mảnh cuối cùng: hoàn thiện `chay_dong` để nó xử lý được câu GÁN, không
chỉ câu TÍNH. Hàm `danh_gia` và `may_tinh_bo_tui` đã có sẵn, đúng như
ví dụ vừa xem. Điền hai chỗ trống trong nhánh `ast.Assign`: tính giá
trị vế phải, rồi CẤT nó vào `moi_truong`.

```python title=starter
import ast

def danh_gia(node, moi_truong):
    if isinstance(node, ast.Constant):
        return node.value
    elif isinstance(node, ast.Name):
        if node.id not in moi_truong:
            raise NameError(f"tên {node.id!r} chưa được gán trong môi trường")
        return moi_truong[node.id]
    elif isinstance(node, ast.BinOp):
        trai = danh_gia(node.left, moi_truong)
        phai = danh_gia(node.right, moi_truong)
        if isinstance(node.op, ast.Add):
            return trai + phai
        elif isinstance(node.op, ast.Sub):
            return trai - phai
        elif isinstance(node.op, ast.Mult):
            return trai * phai
        elif isinstance(node.op, ast.Div):
            return trai / phai
        else:
            raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}")

def chay_dong(dong, moi_truong):
    cay = ast.parse(dong, mode="exec")
    lenh = cay.body[0]
    if isinstance(lenh, ast.Assign):
        ten = lenh.targets[0].id
        gia_tri = ___                      # tinh gia tri ve phai
        ___                                # cat gia_tri vao moi_truong, ung voi ten
        return ten, gia_tri
    elif isinstance(lenh, ast.Expr):
        return None, danh_gia(lenh.value, moi_truong)
    else:
        raise ValueError(f"chưa hỗ trợ câu lệnh: {type(lenh).__name__}")

def may_tinh_bo_tui(chuong_trinh):
    moi_truong = {}
    nhat_ky = []
    for dong in chuong_trinh.strip().split("\n"):
        dong = dong.strip()
        if dong == "":
            continue
        try:
            ten, gia_tri = chay_dong(dong, moi_truong)
        except NameError as loi:
            nhat_ky.append(f"LỖI: {loi}")
            break
        if ten is None:
            nhat_ky.append(f"{dong} => {gia_tri}")
        else:
            nhat_ky.append(f"{ten} = {gia_tri}")
    return nhat_ky, moi_truong

chuong_trinh_1 = "\n".join([
    "gia = 45000",
    "so_luong = 3",
    "tong = gia * so_luong",
    "tong - 5000",
])
nhat_ky_1, moi_truong_1 = may_tinh_bo_tui(chuong_trinh_1)

chuong_trinh_2 = "\n".join([
    "gia = 45000",
    "tong = gia * thue",
])
nhat_ky_2, moi_truong_2 = may_tinh_bo_tui(chuong_trinh_2)

print(nhat_ky_1)
print(moi_truong_1)
print(nhat_ky_2)
print(moi_truong_2)
```

```python title=solution
import ast

def danh_gia(node, moi_truong):
    if isinstance(node, ast.Constant):
        return node.value
    elif isinstance(node, ast.Name):
        if node.id not in moi_truong:
            raise NameError(f"tên {node.id!r} chưa được gán trong môi trường")
        return moi_truong[node.id]
    elif isinstance(node, ast.BinOp):
        trai = danh_gia(node.left, moi_truong)
        phai = danh_gia(node.right, moi_truong)
        if isinstance(node.op, ast.Add):
            return trai + phai
        elif isinstance(node.op, ast.Sub):
            return trai - phai
        elif isinstance(node.op, ast.Mult):
            return trai * phai
        elif isinstance(node.op, ast.Div):
            return trai / phai
        else:
            raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}")

def chay_dong(dong, moi_truong):
    cay = ast.parse(dong, mode="exec")
    lenh = cay.body[0]
    if isinstance(lenh, ast.Assign):
        ten = lenh.targets[0].id
        gia_tri = danh_gia(lenh.value, moi_truong)
        moi_truong[ten] = gia_tri
        return ten, gia_tri
    elif isinstance(lenh, ast.Expr):
        return None, danh_gia(lenh.value, moi_truong)
    else:
        raise ValueError(f"chưa hỗ trợ câu lệnh: {type(lenh).__name__}")

def may_tinh_bo_tui(chuong_trinh):
    moi_truong = {}
    nhat_ky = []
    for dong in chuong_trinh.strip().split("\n"):
        dong = dong.strip()
        if dong == "":
            continue
        try:
            ten, gia_tri = chay_dong(dong, moi_truong)
        except NameError as loi:
            nhat_ky.append(f"LỖI: {loi}")
            break
        if ten is None:
            nhat_ky.append(f"{dong} => {gia_tri}")
        else:
            nhat_ky.append(f"{ten} = {gia_tri}")
    return nhat_ky, moi_truong

chuong_trinh_1 = "\n".join([
    "gia = 45000",
    "so_luong = 3",
    "tong = gia * so_luong",
    "tong - 5000",
])
nhat_ky_1, moi_truong_1 = may_tinh_bo_tui(chuong_trinh_1)

chuong_trinh_2 = "\n".join([
    "gia = 45000",
    "tong = gia * thue",
])
nhat_ky_2, moi_truong_2 = may_tinh_bo_tui(chuong_trinh_2)

print(nhat_ky_1)
print(moi_truong_1)
print(nhat_ky_2)
print(moi_truong_2)
```

```python title=test
assert nhat_ky_1 == [
    "gia = 45000", "so_luong = 3", "tong = 135000", "tong - 5000 => 130000",
], f"bốn dòng chương trình 1 phải cho đúng bốn dòng nhật ký này — đang ra {nhat_ky_1}"
assert moi_truong_1 == {"gia": 45000, "so_luong": 3, "tong": 135000}, f"môi trường sau chương trình 1 phải giữ đúng ba tên đã gán — đang ra {moi_truong_1}"
assert nhat_ky_2 == [
    "gia = 45000", "LỖI: tên 'thue' chưa được gán trong môi trường",
], f"chương trình 2 phải dừng ở dòng thứ hai với lỗi nhắc tên 'thue' — đang ra {nhat_ky_2}"
assert moi_truong_2 == {"gia": 45000}, f"môi trường sau chương trình 2 chỉ có gia (dòng gán tong không chạy xong) — đang ra {moi_truong_2}"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm trong nhánh Assign — chỗ 1 tính giá trị của lenh.value (vế phải dấu =), chỗ 2 CẤT giá trị đó vào moi_truong, ứng với tên ten đã lấy sẵn ở dòng trên.
- kind: strategy
  body: 'Chỗ trống 1 dùng đúng bộ đánh giá đã có: danh_gia(lenh.value, moi_truong) — lenh.value là biểu thức bên phải, không cần bọc ast.Expression vì mode="exec" không tạo nút đó. Chỗ trống 2 là một phép gán dict bình thường: moi_truong[ten] = gia_tri.'
- kind: one-line
  body: 'Chỗ trống 1: danh_gia(lenh.value, moi_truong) — chỗ trống 2: moi_truong[ten] = gia_tri'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: hai chỗ trống phải THẬT SỰ tính danh_gia(lenh.value, moi_truong) rồi cất kết quả vào moi_truong[ten] — điền một câu không làm gì (như True, 1, 0) khiến giá trị gán sai hoặc môi trường không hề được cập nhật
  requireAst:
  # min: 4 cho danh_gia — đếm thật trên solution: danh_gia bị GỌI 4 lần
  # trong toàn mã nguồn — 2 lần LUÔN có mặt bên trong định nghĩa của chính
  # nó (nhánh BinOp gọi trai/phai), 1 lần trong nhánh Expr của chay_dong
  # (có sẵn trong khung, không phải chỗ trống), và đúng 1 lần ở chỗ trống 1.
  # Điền True/1/0 vào chỗ trống 1 xoá mất lần gọi đó, tụt xuống 3, dưới
  # ngưỡng 4 — luật chặn được.
  - kind: uses-call, target: danh_gia, min: 4
  # min: 1 cho subscript-assign — đếm thật trên solution: một phép gán VÀO
  # MỘT Ô của moi_truong (moi_truong[ten] = ...) xuất hiện đúng 1 lần, ở
  # chỗ trống 2. Điền True/1/0 vào chỗ trống 2 xoá mất câu gán đó — 0 lần,
  # dưới ngưỡng 1, luật chặn được. ĐÃ THỬ THẬT bằng cả ba cách True/1/0,
  # trên cả hai chỗ trống cùng lúc và riêng từng chỗ: không vòng lặp/đệ quy
  # nào phụ thuộc chỗ trống (vòng for trong may_tinh_bo_tui chạy theo SỐ
  # DÒNG cố định của chuong_trinh, không theo giá trị điền vào), nên mọi tổ
  # hợp đều dừng ngay — không treo, không vượt trần bước. Điền hụt chỗ
  # trống 2 (chỗ trống 1 đúng) làm moi_truong không hề được cập nhật, nên
  # dòng SAU dùng lại tên đó nổ NameError ngay — nhat_ky/moi_truong sai hẳn
  # so với kỳ vọng; điền hụt chỗ trống 1 (chỗ trống 2 đúng) làm mọi giá trị
  # gán ra đúng bằng con số điền vào (True/1/0), không phải số thật — cả
  # hai trường hợp đều bị assert bắt, độc lập với hai luật static trên.
  - kind: subscript-assign, target: moi_truong, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "'tong = 135000'"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đọc một dòng chữ. Cắt thành token. Dựng thành cây. Gán biến, nhớ lại
được. Tính ra giá trị. Báo lỗi rõ ràng khi sai. Không `eval()` nào cả
— toàn bộ là của bạn.
::::

::::reflect{#nghi-lai}
Câu hỏi cuối cùng của cả track này, và cả Realm 3.

T3.4 khép lại bằng một câu hỏi để ngỏ: suốt track ấy, bạn luôn QUAN SÁT
một trình thông dịch ĐÃ CÓ SẴN — CPython, viết bằng C, ai đó khác đã
xây xong. Câu hỏi để lại là: "Nếu tự bạn phải xây một trình thông dịch
— nhận vào một chuỗi lệnh của riêng bạn, và THẬT SỰ chạy nó — bạn cần
những gì?"

Hai mươi bốn bài sau, bạn vừa trả lời bằng chính tay mình: một VĂN
PHẠM nói câu nào hợp lệ (bài 1-4), một bước TOKEN HOÁ cắt chữ thô thành
miếng có loại (bài 2-5), một CÂY CÚ PHÁP nói rõ cái gì lồng trong cái
gì (bài 6-10), một bộ ĐÁNH GIÁ đi bộ đệ quy trên cây đó, ra một con số
(bài 11-20), một MÔI TRƯỜNG để tên nhớ được giá trị (bài 21), lỗi RÕ
RÀNG khi sai (bài 22), và tất cả ghép lại thành MỘT máy chạy được thật
(bài 23-24). Không phần nào trong số đó bạn chỉ đứng nhìn.

Ngôn ngữ nhỏ của Byte chỉ hiểu bốn phép tính và một phép gán — nhỏ hơn
rất nhiều so với Python thật. Nhưng hình dạng của nó — token, cây, môi
trường, đánh giá đệ quy — là đúng hình dạng mà mọi trình phân tích cú
pháp thật sự dùng, dù là một trình phân tích cho một ngôn ngữ lập trình
khác, hay một trình phân tích cho một câu truy vấn dữ liệu. Bạn sẽ còn
gặp lại đúng bốn mảnh này, dưới những cái tên khác, ở những chỗ khác.

Realm 3 khép lại ở đây. Bit, byte, cấu trúc dữ liệu, thuật toán, cách
máy chạy, và giờ là cách một ngôn ngữ được hiểu — năm track, một câu
chuyện nối liền từ đáy lên đỉnh.
::::

::::checkpoint{mastery=0.85}
::::
