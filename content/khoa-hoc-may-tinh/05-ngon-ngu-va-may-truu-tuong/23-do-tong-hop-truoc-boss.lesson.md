---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.do-tong-hop-truoc-boss
title: "Đo tổng hợp trước BOSS: token hoá + dựng cây + đánh giá"
summary: "Ba mảnh rời — tokenize, ast.parse, danh_gia — ghép lại thành MỘT hàm duy nhất, nhận một chuỗi, trả về đúng kết quả tính tay. ast.parse tự tokenize hoá bên trong nó; đoạn tokenize riêng của bạn chỉ để XEM, không phải nguyên liệu nó cần."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ngu.combined-pipeline]
requires: [ngu.meaningful-errors]
concepts: [ngu.combined-pipeline]
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
Bạn nói bạn TIN ba mảnh rời khớp được với nhau. Giờ đo thử, đừng tin
suông.
::::

::::explain{#ba-manh-thanh-mot}
Đếm lại đúng hành trình track này đã đi: cụm 1 (bài 2-5) cắt một chuỗi
thô thành TOKEN. Cụm 2 (bài 7-10) dựng token đó thành một CÂY cú pháp,
bằng `ast.parse`. Cụm 3-4 (bài 11-20) đi bộ đệ quy trên cây đó, tính ra
một con số. Hai bài vừa rồi thêm TÊN và lỗi rõ ràng vào bước cuối đó.

Ba bước — token hoá, dựng cây, đánh giá — nghe như một dây chuyền: đầu
ra bước này là đầu vào bước sau. Nhưng có một sự thật đáng nói thẳng
ra trước khi ghép: **`ast.parse()` không cần bạn tokenize trước tay.**
Nó nhận thẳng một CHUỖI, và tự lo phần cắt-thành-token của riêng nó ở
bên trong — bạn chưa từng phải đưa danh sách token bạn tự tính cho nó.

Vậy đoạn `tokenize.generate_tokens()` bạn viết ở cụm 1 vẫn có giá trị
— nhưng giá trị đó là để BẠN XEM và ĐỐI CHIẾU (đúng như bài 5 và bài 10
đã làm), không phải nguyên liệu mà `ast.parse` cần nhận vào. Ba khái
niệm — token, cây, giá trị — vẫn ĐÚNG thứ tự sư phạm để HỌC. Nhưng khi
GHÉP thành một hàm chạy thật, hàm đó gọi đúng hai thứ: dựng cây từ
chuỗi gốc, rồi đánh giá cây đó.

```python
def chay_bieu_thuc(bieu_thuc, moi_truong):
    danh_sach_token = [...]     # chỉ để XEM/ĐẾM, không bắt buộc phải có
    cay = ast.parse(bieu_thuc, mode="eval")
    return danh_gia(cay.body, moi_truong)
```
::::

::::example{#ghep-that-chay-that}
Sổ giá của Byte: `{"gia": 45000, "so_luong": 2, "giam_gia": 5000}`.
Hàm ghép cả ba bước, chạy trên hai biểu thức khác nhau:

```python title=readonly
import ast, tokenize, io

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

def chay_bieu_thuc(bieu_thuc, moi_truong):
    danh_sach_token = [
        t for t in tokenize.generate_tokens(io.StringIO(bieu_thuc).readline)
        if t.string.strip() != ""
    ]
    cay = ast.parse(bieu_thuc, mode="eval")
    ket_qua = danh_gia(cay.body, moi_truong)
    return ket_qua, len(danh_sach_token)

moi_truong = {"gia": 45000, "so_luong": 2, "giam_gia": 5000}

kq1, sl1 = chay_bieu_thuc("gia * so_luong - giam_gia", moi_truong)
print(kq1, sl1)
```

```text title=readonly
85000 5
```

`gia * so_luong - giam_gia` = `45000 * 2 - 5000` = `85000` — tính tay
khớp đúng. Năm token có nghĩa: `gia`, `*`, `so_luong`, `-`, `giam_gia`
— đối chiếu bằng `tokenize` thật, đúng cách bài 2-5 đã làm:

```python title=readonly
for t in tokenize.generate_tokens(io.StringIO("gia * so_luong - giam_gia").readline):
    if t.string.strip() != "":
        print(tokenize.tok_name[t.type], repr(t.string))
```

```text title=readonly
NAME 'gia'
OP '*'
NAME 'so_luong'
OP '-'
NAME 'giam_gia'
```
::::

::::predict{#tokenize-co-can-khong commitOnce}
Nếu XOÁ HẲN đoạn tính `danh_sach_token` trong `chay_bieu_thuc`, chỉ
còn lại `ast.parse` và `danh_gia`:

```python
def chay_khong_tokenize(bieu_thuc, moi_truong):
    cay = ast.parse(bieu_thuc, mode="eval")
    return danh_gia(cay.body, moi_truong)
```

**Trước khi chạy**, `chay_khong_tokenize("gia * so_luong - giam_gia",
moi_truong)` có còn ra đúng `85000` không?

:::opt{correct}
Có — vẫn ra đúng `85000`. `ast.parse()` tự tokenize hoá RIÊNG bên
trong nó; đoạn `danh_sach_token` chỉ để bạn XEM, không phải nguyên
liệu `ast.parse` cần
:::

:::opt
Không — `ast.parse()` sẽ dùng CHÍNH `danh_sach_token` vừa tính làm đầu
vào, xoá nó đi thì không còn gì để dựng cây
::why
Gần đúng ở việc bạn nghĩ đây là một bước NỐI TIẾP, dùng đầu ra bước
trước làm đầu vào bước sau — đúng với chính sơ đồ ba bước (token → cây
→ giá trị) mà cả track này dạy.

Chỗ lệch: nhìn kỹ dòng gọi `ast.parse(bieu_thuc, mode="eval")` — tham
số truyền vào là `bieu_thuc`, chuỗi GỐC, không phải `danh_sach_token`.
Hai lệnh này không hề nối vào nhau qua một biến chung nào cả. Xoá
`danh_sach_token` không đụng gì tới `cay` hay `ket_qua`.
::
:::

:::opt
Nổ lỗi, vì `ast.parse()` bên trong nó gọi tới biến `danh_sach_token`
để tham chiếu, mà biến đó không còn tồn tại
::why
Gần đúng ở việc bạn nghĩ xoá một biến ĐANG ĐƯỢC DÙNG sẽ gây lỗi — đúng,
nói chung, với những biến thật sự được dùng ở chỗ khác.

Chỗ lệch: `danh_sach_token` không hề được truyền vào `ast.parse()` hay
`danh_gia()` — hai lời gọi đó chỉ dùng `bieu_thuc`, `cay`, `moi_truong`.
Xoá một biến không ai đọc tới không gây lỗi gì, dù trông nó có vẻ là
một "bước" quan trọng trong sơ đồ ba bước.
::
:::

:::opt
Vẫn chạy được, nhưng ra kết quả SAI — vì thiếu bước "xác nhận token
hợp lệ" trước khi dựng cây, nên `ast.parse()` bỏ qua một phần biểu thức
::why
Gần đúng ở việc bạn nhớ token hoá LÀ một bước có thật trong việc phân
tích một biểu thức — điều đó đúng, cụm 1 đã dạy kỹ.

Chỗ lệch: `ast.parse()` không "bỏ qua" phần nào cả khi thiếu bước riêng
của bạn — chính NÓ đã tự làm đủ phần token hoá của nó, một cách hoàn
toàn độc lập, trước khi dựng cây. Đoạn `danh_sach_token` bạn tự viết
không hề "xác nhận" hay "cấp phép" gì cho `ast.parse()` — nó chỉ là một
phép tính song song, để bạn nhìn thấy, không phải một điều kiện.
::
:::
::::

::::code{#ghep-hai-buoc}
Byte cần một hàm `chay_bieu_thuc` DUY NHẤT — nhận một chuỗi và một môi
trường, trả về kết quả TÍNH ĐƯỢC cùng số token có nghĩa của chuỗi đó.
Đoạn đếm token đã viết sẵn. Điền hai chỗ trống: dựng cây từ chuỗi gốc,
rồi đánh giá cây đó.

```python title=starter
import ast, tokenize, io

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

def chay_bieu_thuc(bieu_thuc, moi_truong):
    danh_sach_token = [
        t for t in tokenize.generate_tokens(io.StringIO(bieu_thuc).readline)
        if t.string.strip() != ""
    ]
    cay = ___                                 # dung cay tu bieu_thuc
    ket_qua = ___                              # danh gia cay.body
    return ket_qua, len(danh_sach_token)

moi_truong = {"gia": 45000, "so_luong": 2, "giam_gia": 5000}

kq1, sl1 = chay_bieu_thuc("gia * so_luong - giam_gia", moi_truong)
kq2, sl2 = chay_bieu_thuc("gia + gia", moi_truong)

print(kq1, sl1)
print(kq2, sl2)
```

```python title=solution
import ast, tokenize, io

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

def chay_bieu_thuc(bieu_thuc, moi_truong):
    danh_sach_token = [
        t for t in tokenize.generate_tokens(io.StringIO(bieu_thuc).readline)
        if t.string.strip() != ""
    ]
    cay = ast.parse(bieu_thuc, mode="eval")
    ket_qua = danh_gia(cay.body, moi_truong)
    return ket_qua, len(danh_sach_token)

moi_truong = {"gia": 45000, "so_luong": 2, "giam_gia": 5000}

kq1, sl1 = chay_bieu_thuc("gia * so_luong - giam_gia", moi_truong)
kq2, sl2 = chay_bieu_thuc("gia + gia", moi_truong)

print(kq1, sl1)
print(kq2, sl2)
```

```python title=test
assert kq1 == 85000, f"gia * so_luong - giam_gia = 45000*2 - 5000 = 85000 — đang ra {kq1}"
assert sl1 == 5, f"'gia * so_luong - giam_gia' có 5 token có nghĩa (gia, *, so_luong, -, giam_gia) — đang ra {sl1}"
assert kq2 == 90000, f"gia + gia = 45000 + 45000 = 90000 — đang ra {kq2}"
assert sl2 == 3, f"'gia + gia' có 3 token có nghĩa (gia, +, gia) — đang ra {sl2}"
```

:::hints
- kind: attention
  body: Hai chỗ trống làm đúng hai việc đã học ở bài 7 và bài 15 — dựng cây từ chuỗi gốc bieu_thuc, rồi đưa PHẦN THÂN của cây đó (cay.body) cho danh_gia cùng với moi_truong.
- kind: strategy
  body: 'Chỗ trống 1 là ast.parse(bieu_thuc, mode="eval") — đúng công cụ bài 7 đã dùng, trên chuỗi GỐC, không phải danh_sach_token. Chỗ trống 2 là danh_gia(cay.body, moi_truong) — bỏ lớp Expression bọc ngoài (như bài 11 đã làm) rồi đưa cho bộ đánh giá bài 21-22 đã hoàn thiện.'
- kind: one-line
  body: 'Chỗ trống 1: ast.parse(bieu_thuc, mode="eval") — chỗ trống 2: danh_gia(cay.body, moi_truong)'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: hai chỗ trống phải THẬT SỰ gọi ast.parse(bieu_thuc, mode="eval") rồi danh_gia(cay.body, moi_truong) — điền một câu không làm gì (như True, 1, 0) làm hàm không còn dựng cây thật hoặc không còn đánh giá thật
  requireAst:
  # min: 1 cho parse — đếm thật trên solution: ast.parse(...) chỉ xuất hiện
  # ĐÚNG 1 lần trong toàn mã nguồn (ở chỗ trống 1). Điền True/1/0 vào chỗ
  # trống 1 xoá mất lần gọi duy nhất đó, tụt xuống 0 — luật chặn được.
  - kind: uses-call, target: parse, min: 1
  # min: 3 cho danh_gia — đếm thật trên solution: danh_gia bị GỌI 3 lần
  # trong mã nguồn — 2 lần bên TRONG định nghĩa hàm danh_gia (nhánh BinOp
  # gọi trai/phai — luôn có mặt, không phụ thuộc chỗ trống nào), cộng đúng
  # 1 lần ở chỗ trống 2. Điền True/1/0 vào chỗ trống 2 xoá mất lần gọi đó,
  # tụt xuống 2, dưới ngưỡng 3 — luật chặn được. ĐÃ THỬ THẬT bằng cả ba cách
  # True/1/0 trên CẢ HAI chỗ trống cùng lúc và từng chỗ riêng lẻ: không
  # vòng lặp/đệ quy nào phụ thuộc giá trị điền vào (đệ quy trong danh_gia
  # đi theo HÌNH DẠNG cây, không theo nội dung chỗ trống), nên mọi tổ hợp
  # đều dừng ngay — không treo, không vượt trần bước. Điền hụt chỗ trống 1
  # làm chương trình NỔ LỖI ngay (danh_gia nhận một bool/int thay vì một
  # ast.AST, rơi vào "raise ValueError: chưa hỗ trợ loại nút"). Điền hụt
  # chỗ trống 2 làm kq1/kq2 bằng đúng giá trị điền (không phải 85000/90000)
  # — assert bắt được độc lập.
  - kind: uses-call, target: danh_gia, min: 3
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: "^85000 5\\n90000 3\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba mảnh rời suốt hai mươi ba bài — token, cây, giá trị — giờ chạy
trong đúng MỘT hàm, trên bất kỳ biểu thức nào bạn đưa vào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi vào bài cuối.

`chay_bieu_thuc` mới xử lý được đúng MỘT dạng câu: một BIỂU THỨC, như
`gia * so_luong - giam_gia`. Nó tính ra giá trị, nhưng không hề GHI lại
giá trị đó vào đâu cả — gọi lại lần sau, mọi thứ lại tính từ đầu.

Nhưng một "ngôn ngữ" thật, kể cả một cái nhỏ xíu, còn cần làm được một
việc khác hẳn: **gán** — như `tong = gia * so_luong`, rồi DÙNG LẠI
`tong` ở dòng sau, không cần tính lại. Đó không còn là một biểu thức
đơn thuần — nhìn lại cây của `gia = 45000` mà xem, gốc của nó không
phải `BinOp` hay `Constant` nữa.

Ghép nốt mảnh cuối đó vào, và ngôn ngữ nhỏ của Byte coi như xong. Bài
sau làm chính việc ấy.
::::

::::checkpoint{mastery=0.8}
::::
