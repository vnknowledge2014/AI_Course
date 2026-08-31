---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.bao-loi-co-nghia
title: "Báo lỗi có nghĩa — nói đúng chỗ sai"
summary: "moi_truong[node.id] nổ KeyError khi tên lạ — đúng nhưng mơ hồ, không nói rõ đây là một cái TÊN chưa gán. Ngôn ngữ nhỏ phải tự raise NameError rõ ràng, đúng chỗ, đúng lúc — không âm thầm trả None, không treo máy."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ngu.meaningful-errors]
requires: [ngu.add-variables, err.name-error]
concepts: [ngu.meaningful-errors]
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
Bạn vừa thử rồi đúng không? Máy CÓ dừng lại — nhưng lời nó nói ra chưa
đủ rõ.
::::

::::explain{#loi-mo-ho-va-loi-ro-rang}
Realm 1 đã cho bạn gặp `NameError` nhiều lần: dùng một biến chưa từng
được gán, Python dừng lại và nói thẳng — `NameError: name 'thue' is
not defined`. Rõ LOẠI lỗi (một cái tên có vấn đề), rõ TÊN nào
(`'thue'`), rõ VẤN ĐỀ gì (chưa được định nghĩa).

Ngôn ngữ nhỏ của bạn, tới cuối bài trước, chưa làm được như vậy.
`moi_truong[node.id]` là một phép tra `dict` bình thường — gặp khoá lạ,
Python tự nổ `KeyError`, không phải vì bộ đánh giá CHỦ ĐỘNG báo lỗi, mà
vì `dict` tình cờ có sẵn hành vi đó. Thông báo `KeyError: 'thue'` không
nói đây là một cái TÊN, không nói nó "chưa được gán" — nó chỉ nói
"khoá này không có trong dict", đúng về mặt kỹ thuật của `dict`, nhưng
sai NGỮ CẢNH của một ngôn ngữ đang cố gắng nói chuyện với người dùng nó.

Còn một cách tệ hơn KeyError: **âm thầm trả về `None`** khi không tìm
thấy tên, thay vì báo lỗi gì cả. Nghe như "an toàn hơn" — không bao giờ
làm chương trình dừng đột ngột. Nhưng `None` rồi bị đưa tiếp vào một
phép tính khác, và lỗi thật sự chỉ lộ ra ở MỘT CHỖ KHÁC, xa hẳn chỗ cái
tên bị thiếu, với một thông báo chẳng còn liên quan gì tới cái tên đó
nữa. Ví dụ ngay dưới đây cho thấy đúng điều này xảy ra thật.

Một ngôn ngữ tốt phải làm đúng ba việc khi gặp tên lạ: (1) dừng lại
NGAY tại chỗ tra cứu, không đi tiếp; (2) nói rõ LOẠI lỗi và TÊN nào;
(3) không bao giờ ÂM THẦM trả sai hay TREO máy. `raise NameError(...)`
— dùng lại đúng lớp lỗi Python thật đã dạy, với một thông điệp tự viết
— làm cả ba việc đó cùng lúc.
::::

::::example{#ba-cach-that}
Ba cách bộ đánh giá phản ứng với cùng một biểu thức thiếu tên, chạy
thật cả ba:

```python title=readonly
import ast

def danh_gia_tho(node, moi_truong):
    if isinstance(node, ast.Constant):
        return node.value
    elif isinstance(node, ast.Name):
        return moi_truong[node.id]              # cach 1: tra truc tiep
    elif isinstance(node, ast.BinOp):
        trai = danh_gia_tho(node.left, moi_truong)
        phai = danh_gia_tho(node.right, moi_truong)
        if isinstance(node.op, ast.Mult):
            return trai * phai
        else:
            raise ValueError("chưa hỗ trợ phép toán")
    else:
        raise ValueError("chưa hỗ trợ loại nút")

cay = ast.parse("z * 2", mode="eval")
try:
    danh_gia_tho(cay.body, {"x": 5})
except Exception as loi:
    print(f"cách 1: {type(loi).__name__}: {loi}")

def danh_gia_am_tham(node, moi_truong):
    if isinstance(node, ast.Constant):
        return node.value
    elif isinstance(node, ast.Name):
        return moi_truong.get(node.id)           # cach 2: am tham tra None
    elif isinstance(node, ast.BinOp):
        trai = danh_gia_am_tham(node.left, moi_truong)
        phai = danh_gia_am_tham(node.right, moi_truong)
        if isinstance(node.op, ast.Mult):
            return trai * phai
        else:
            raise ValueError("chưa hỗ trợ phép toán")
    else:
        raise ValueError("chưa hỗ trợ loại nút")

try:
    danh_gia_am_tham(cay.body, {"x": 5})
except Exception as loi:
    print(f"cách 2: {type(loi).__name__}: {loi}")
```

```text title=readonly
cách 1: KeyError: 'z'
cách 2: TypeError: unsupported operand type(s) for *: 'NoneType' and 'int'
```

Cách 1 dừng ĐÚNG chỗ (tra `moi_truong['z']`), nhưng nói mơ hồ — chỉ có
`'z'` trơ trọi, không nói đây là một cái tên. Cách 2 còn tệ hơn: nó
KHÔNG dừng ở chỗ tên bị thiếu — nó lặng lẽ trả `None`, rồi lỗi thật sự
nổ ra HAI BƯỚC SAU, ở phép nhân `None * 2`, với một thông điệp không
hề nhắc tới `z` — người debug phải tự lần ngược mới biết gốc rễ ở đâu.

Sửa lại đúng chỗ, dừng đúng lúc, nói đúng tên:

```python title=readonly
def danh_gia(node, moi_truong):
    if isinstance(node, ast.Name):
        if node.id not in moi_truong:
            raise NameError(f"tên {node.id!r} chưa được gán trong môi trường")
        return moi_truong[node.id]
    # ... các nhánh khác như cũ
```

```python title=readonly
try:
    danh_gia(cay.body, {"x": 5})
except NameError as loi:
    print("bắt được:", loi)
```

```text title=readonly
bắt được: tên 'z' chưa được gán trong môi trường
```

Đúng lớp lỗi Python thật (`NameError`, đã quen từ Realm 1), đúng tên
(`'z'`), đúng lý do (`chưa được gán`) — và dừng lại NGAY tại chỗ tra
cứu, không đợi tới một phép tính nào sau đó.
::::

::::predict{#khong-bao-gio-nan commitOnce}
Byte thử một cách viết "an toàn hơn" cho nhánh `ast.Name` — dùng
`.get()` để không bao giờ nổ `KeyError`:

```python
if isinstance(node, ast.Name):
    return moi_truong.get(node.id)
```

Chạy trên biểu thức `z * 2`, với `moi_truong` chỉ có `{"x": 5}` (không
có `z`). **Trước khi đọc đáp án**, chuyện gì xảy ra?

:::opt{correct}
Máy vẫn dừng lại, nhưng báo `TypeError: unsupported operand type(s)
for *: 'NoneType' and 'int'` — không hề nhắc tới cái tên `z`
:::

:::opt
Chương trình chạy êm, in ra `None` — vì `.get()` được thiết kế để
không bao giờ làm chương trình dừng lại
::why
Gần đúng ở việc bạn nhớ đúng: `.get()` KHÔNG tự nổ `KeyError` khi
thiếu khoá — điều đó có thật, và đúng là lý do người ta chọn `.get()`.

Chỗ lệch: `.get()` chỉ tránh lỗi Ở CHÍNH NÓ. Giá trị `None` nó trả về
không biến mất — nó bị đưa TIẾP vào phép nhân `* 2` ngay bước sau, và
phép nhân giữa `None` với một số thì Python không hiểu, nên chương
trình vẫn dừng lại — chỉ là dừng muộn hơn một bước, ở một chỗ khác.
::
:::

:::opt
Máy báo đúng `NameError: name 'z' is not defined` — giống hệt lỗi
thật của Python khi dùng một biến chưa gán
::why
Gần đúng ở việc bạn nhớ đúng LOẠI lỗi HỢP LÝ cho tình huống "một tên
không tồn tại" — `NameError` đúng là cái tên nên dùng cho ca này.

Chỗ lệch: `NameError` đó là lỗi của PYTHON THẬT, chỉ xảy ra khi chính
PYTHON tra cứu một biến trong CHƯƠNG TRÌNH BẠN GÕ. Ở đây `z` không phải
một biến Python — nó chỉ là một chuỗi nằm trong `node.id`, và đoạn code
này (dùng `.get()`) không hề viết dòng `raise` nào để tự tạo lỗi đó.
Không có `raise`, không có lỗi đó xuất hiện — dù tình huống ĐÁNG LẼ nên
báo `NameError`.
::
:::

:::opt
Máy dừng ngay tại chỗ đọc `node.id`, báo `KeyError: 'z'`, giống hệt
bản dùng ngoặc vuông trực tiếp
::why
Gần đúng ở việc bạn nhớ đúng: một bản KHÁC (dùng `moi_truong[node.id]`
trực tiếp) thật sự báo `KeyError: 'z'` — đó là hành vi có thật, chỉ là
của một đoạn code khác.

Chỗ lệch: đây là bản dùng `.get()`, và `.get()` được viết ra CHÍNH LÀ
để không bao giờ nổ `KeyError` — nó luôn trả về một giá trị (ở đây là
`None`) thay vì dừng lại ngay tại chỗ tra cứu. Lỗi không xảy ra ở bước
này của bản `.get()`, dù nó có xảy ra ở bản kia.
::
:::
::::

::::code{#raise-ro-rang}
Sổ giá của Byte: `{"gia": 45000, "so_luong": 2}`. Bộ đánh giá dưới đây
đã tính đúng khi tên có mặt — việc của bạn là thêm phần KIỂM TRA và
BÁO LỖI khi tên KHÔNG có mặt, ngay tại nhánh `ast.Name`, trước khi tra
cứu.

```python title=starter
import ast

def danh_gia(node, moi_truong):
    if isinstance(node, ast.Constant):
        return node.value
    elif isinstance(node, ast.Name):
        if node.id not in moi_truong:
            ___                            # bao loi ro: ten nao, tai sao
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

moi_truong = {"gia": 45000, "so_luong": 2}

cay_dung = ast.parse("gia * so_luong", mode="eval")
ket_qua_dung = danh_gia(cay_dung.body, moi_truong)

cay_sai = ast.parse("gia * thue", mode="eval")
loi_bat_duoc = None
loi_la_name_error = False
try:
    danh_gia(cay_sai.body, moi_truong)
except NameError as loi:
    loi_bat_duoc = str(loi)
    loi_la_name_error = True
except Exception as loi:
    loi_bat_duoc = str(loi)
    loi_la_name_error = False

print(ket_qua_dung)
print(loi_la_name_error)
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

moi_truong = {"gia": 45000, "so_luong": 2}

cay_dung = ast.parse("gia * so_luong", mode="eval")
ket_qua_dung = danh_gia(cay_dung.body, moi_truong)

cay_sai = ast.parse("gia * thue", mode="eval")
loi_bat_duoc = None
loi_la_name_error = False
try:
    danh_gia(cay_sai.body, moi_truong)
except NameError as loi:
    loi_bat_duoc = str(loi)
    loi_la_name_error = True
except Exception as loi:
    loi_bat_duoc = str(loi)
    loi_la_name_error = False

print(ket_qua_dung)
print(loi_la_name_error)
```

```python title=test
assert ket_qua_dung == 90000, f"gia * so_luong với gia=45000, so_luong=2 phải là 90000 — đang ra {ket_qua_dung}"
assert loi_la_name_error is True, f"lỗi bắt được phải là NameError, không phải một lỗi mơ hồ khác (như KeyError) — đang là loi_la_name_error={loi_la_name_error}"
assert loi_bat_duoc == "tên 'thue' chưa được gán trong môi trường", f"thông điệp lỗi phải nói ĐÚNG NGUYÊN VĂN dạng \"tên 'thue' chưa được gán trong môi trường\" — nói rõ đây là một TÊN, tên nào, và LÝ DO (chưa được gán) — đang là {loi_bat_duoc!r}"
```

:::hints
- kind: attention
  body: Chỗ trống nằm TRONG nhánh "node.id không có trong moi_truong" — việc của nó là báo lỗi RÕ RÀNG, không phải trả về giá trị gì cả.
- kind: strategy
  body: 'Dùng raise NameError(...) — đúng lớp lỗi Python thật, đã quen từ Realm 1 — với một thông điệp f-string nhắc rõ tên bị thiếu bằng node.id. Ví dụ: raise NameError(f"tên {node.id!r} chưa được gán trong môi trường").'
- kind: one-line
  body: 'Chỗ trống là: raise NameError(f"tên {node.id!r} chưa được gán trong môi trường")'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ raise NameError(...) — điền một câu không làm gì (như True, 1, 0) khiến chương trình rơi thẳng xuống dòng tra cứu và nổ KeyError mơ hồ, không phải NameError rõ ràng
  requireAst:
  # min: 1 — đếm thật trên solution: NameError(...) được GỌI đúng 1 lần
  # trong toàn bộ mã nguồn (đúng ở chỗ trống). ĐÃ THỬ THẬT bằng cả ba cách
  # True/1/0: không vòng lặp/đệ quy nào phụ thuộc chỗ trống (nhánh if chỉ
  # rẽ theo node.id có trong moi_truong hay không, không theo giá trị điền
  # vào), nên cả ba dừng ngay — không treo, không vượt trần bước. Cả ba
  # khiến luồng chạy rơi thẳng xuống "return moi_truong[node.id]" ngay sau
  # đó, nổ KeyError (bị bắt vào nhánh except Exception chung, không phải
  # except NameError) — loi_la_name_error ở lại False, không phải True,
  # nên assert bắt được độc lập với luật static này.
  - kind: uses-call, target: NameError, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^90000\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`KeyError: 'z'` mơ hồ, `TypeError` sai chỗ, hay `NameError` nói đúng
tên đúng lý do — bạn vừa chọn đúng cái thứ ba, và tự tay viết ra nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn có đủ ba mảnh rời: **tokenize** (cụm 1, cắt chữ thô thành
miếng), **dựng cây** bằng `ast.parse` (cụm 2), và một bộ **đánh giá**
hiểu cả số lẫn tên, báo lỗi rõ ràng khi sai (cụm 3-4 và hai bài vừa
rồi). Nhưng ba mảnh đó tới giờ vẫn nằm RIÊNG — bạn gọi `tokenize` ở
một bài, `ast.parse` ở bài khác, `danh_gia` ở bài khác nữa, chưa bài
nào ghép cả ba lại thành MỘT hàm duy nhất.

Nếu ghép chúng lại — nhận một chuỗi bất kỳ, tự tokenize, tự dựng cây,
tự đánh giá, trả lời đúng luôn — bạn có tin ba mảnh rời rạc đó khớp
được với nhau ngay, không cần sửa gì thêm không?

Bài sau ghép thử, và đo xem có đúng như bạn tin không.
::::

::::checkpoint{mastery=0.8}
::::
