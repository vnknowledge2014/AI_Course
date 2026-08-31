---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.bo-danh-gia-chi-hieu-loai-nut-no-biet
title: "Một bộ đánh giá chỉ hiểu được các LOẠI NÚT nó biết"
summary: "danh_gia bài 11-12 chỉ xử lý Constant/BinOp với Add/Mult — gặp Sub, nó không nổ lỗi, nó lặng lẽ trả về None. Sai âm thầm tệ hơn sai ồn ào. Thêm else báo lỗi RÕ, nêu đích danh loại nút hay phép toán chưa hỗ trợ."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ngu.evaluator-limited]
requires: [ngu.leaf-vs-internal]
concepts: [ngu.evaluator-limited]
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
Đưa nó một phép nó chưa từng học. Xem nó làm gì — và tại sao câu trả
lời khiến Byte lo hơn là một dòng lỗi đỏ chót.
::::

::::explain{#chi-biet-cai-no-duoc-day}
Hàm `danh_gia` bài 11-12 chỉ có đúng bốn nhánh `if`: `Constant`, và bên
trong `BinOp` là `Add` với `Mult`. Nó chưa từng được dạy `ast.Sub` (phép
trừ) là gì — và cũng chưa được dạy phải làm gì khi gặp một LOẠI NÚT lạ,
không phải `Constant` cũng không phải `BinOp`.

Câu hỏi: đưa nó `"10 - 3"` thì chuyện gì xảy ra? Trực giác thường đoán
"nó sẽ báo lỗi". Chạy thật mới thấy sự thật tệ hơn thế.
::::

::::example{#sai-am-tham}
Đúng hàm `danh_gia` bài 12 (bỏ phần nhật ký, giữ nguyên phần tính),
chạy trên `"10 - 3"`:

```python title=readonly
import ast

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
        # không có nhánh nào cho Sub — và không có else

def tinh(bt):
    cay = ast.parse(bt, mode="eval")
    return danh_gia(cay.body)

print(tinh("10 - 3"))
```

```text title=readonly
None
```

Không có dòng lỗi đỏ nào. Không có `Traceback`. Chương trình chạy hết,
in ra `None`, và dừng — y hệt như nó vừa "làm xong việc". Chuyện thật
sự xảy ra: cả `isinstance(node.op, ast.Add)` lẫn `isinstance(node.op,
ast.Mult)` đều `False` (vì phép toán là `Sub`), nên KHÔNG nhánh `if`
nào chạy `return`. Hàm chạy hết thân mình mà không gặp `return` nào —
và một hàm Python không `return` gì thì tự động trả về `None`.

Đây là lối sai NGUY HIỂM nhất: không phải sai ồn ào (crash, dễ thấy),
mà là sai IM LẶNG — `None` trôi tiếp vào bất cứ phép tính nào dùng nó,
và người gọi hàm không có cách nào biết được lỗi nằm ở đâu, thậm chí
không biết có lỗi.
::::

::::predict{#doan-loi-mod commitOnce}
Bộ đánh giá vừa được SỬA — mỗi nhánh `if`/`elif` giờ có thêm một `else`
báo lỗi RÕ RÀNG khi gặp phép toán hoặc loại nút nó không biết (chi tiết
cách sửa ở phần bài tập).

Biểu thức mới: `"9 % 2"` (phép chia lấy dư) — đúng cây `BinOp`, đúng
loại nút bộ đánh giá đã biết xử lý, chỉ có phép toán (`ast.Mod`) là lạ,
y hệt tình cảnh của `Sub` lúc nãy nhưng chưa hề gặp qua.

**Trước khi chạy**, bạn đoán: gọi `tinh("9 % 2")` sau khi sửa sẽ ra
sao?

:::opt{correct}
Ném ra `ValueError`, thông điệp lỗi nêu đích danh `Mod` — đúng nhánh
"phép toán lạ" bên TRONG xử lý `BinOp`
:::

:::opt
Ném ra `ValueError`, nhưng thông điệp lỗi nêu `BinOp` — đúng nhánh
"loại nút lạ" ở ngoài cùng
::why
Gần đúng ở việc bạn đúng: MỘT lỗi `ValueError` chắc chắn được ném ra
sau khi sửa — không còn đường nào lọt qua âm thầm nữa.

Chỗ lệch nằm ở nhánh NÀO ném lỗi. Nút gốc của `"9 % 2"` VẪN LÀ `BinOp`
— đúng loại nút bộ đánh giá đã biết xử lý, không hề lạ. Cái lạ nằm ở
`node.op` (kiểu `Mod`), không phải bản thân `node`. Lỗi phải ném từ
nhánh xử lý "phép toán lạ" (bên TRONG, sau khi đã vào đúng `BinOp`),
nêu tên `Mod` — không phải nhánh "loại nút lạ" (bên NGOÀI), nêu tên
`BinOp`.
::
:::

:::opt
Lặng lẽ trả về `None` — giống hệt hành vi của `"10 - 3"` trước khi sửa
::why
Gần đúng ở việc đây đúng là hành vi CŨ, TRƯỚC khi sửa — bạn nhớ đúng
lỗ hổng vừa thấy ở ví dụ.

Chỗ lệch: bài học vừa sửa CHÍNH lỗ hổng đó. Sau khi sửa, MỌI nhánh
không nhận diện được phép toán hoặc loại nút đều rơi vào một `else`
ném lỗi ngay — không còn đường nào để hàm chạy hết thân mình mà không
`return` và không `raise` nữa.
::
:::

:::opt
Chương trình treo, chạy mãi không dừng
::why
Gần đúng ở sự thận trọng khi gặp một phép toán chưa quen — thái độ đó
đúng đắn khi đọc mã lạ.

Chỗ lệch: không có `while` hay đệ quy không-đáy nào ở đây. Mọi nhánh
`if`/`elif` đều kết thúc bằng `else: raise ...` — một câu lệnh `raise`
luôn NÉM RA NGAY LẬP TỨC và kết thúc lời gọi đó, không lặp lại gì cả.
Đệ quy trên `"9 % 2"` cũng chỉ sâu đúng một tầng (một `BinOp`, hai
`Constant` con) — không có chỗ nào để "chạy mãi".
::
:::
::::

::::code{#them-else-bao-loi}
Sửa `danh_gia` để nó không còn sai ÂM THẦM nữa. Khung hàm đã đủ hai
nhánh `Add`/`Mult` như cũ — việc của bạn: điền đúng HAI nhánh `else`,
mỗi nhánh ném một `ValueError`, thông điệp NÊU ĐÍCH DANH đúng cái gì
chưa được hỗ trợ (dùng `type(...).__name__` để lấy tên thật, không
đoán tay).

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
        else:
            ___                     # phép toán lạ (ví dụ Sub): báo lỗi RÕ, đừng lọt qua âm thầm
    else:
        ___                         # loại nút lạ (không phải Constant/BinOp): báo lỗi RÕ

def tinh(bt):
    cay = ast.parse(bt, mode="eval")
    return danh_gia(cay.body)

ket_qua_binh_thuong = tinh("2 + 3 * 4")

try:
    tinh("10 - 3")
    thong_diep_phep_toan = "KHÔNG NÉM LỖI NÀO"
except ValueError as loi:
    thong_diep_phep_toan = str(loi)

try:
    tinh("-3")
    thong_diep_loai_nut = "KHÔNG NÉM LỖI NÀO"
except ValueError as loi:
    thong_diep_loai_nut = str(loi)

print(ket_qua_binh_thuong)
print(thong_diep_phep_toan)
print(thong_diep_loai_nut)
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
        else:
            raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}")

def tinh(bt):
    cay = ast.parse(bt, mode="eval")
    return danh_gia(cay.body)

ket_qua_binh_thuong = tinh("2 + 3 * 4")

try:
    tinh("10 - 3")
    thong_diep_phep_toan = "KHÔNG NÉM LỖI NÀO"
except ValueError as loi:
    thong_diep_phep_toan = str(loi)

try:
    tinh("-3")
    thong_diep_loai_nut = "KHÔNG NÉM LỖI NÀO"
except ValueError as loi:
    thong_diep_loai_nut = str(loi)

print(ket_qua_binh_thuong)
print(thong_diep_phep_toan)
print(thong_diep_loai_nut)
```

```python title=test
assert ket_qua_binh_thuong == 14, f"2 + 3 * 4 vẫn phải tính đúng như cũ — đang ra {ket_qua_binh_thuong}"
assert thong_diep_phep_toan == "chưa hỗ trợ phép toán: Sub", f"phải ném ValueError nêu đích danh Sub — đang là {thong_diep_phep_toan!r}"
assert thong_diep_loai_nut == "chưa hỗ trợ loại nút: UnaryOp", f"'-3' là một ast.UnaryOp (dấu âm một ngôi), không phải Constant hay BinOp — phải ném ValueError nêu đích danh UnaryOp — đang là {thong_diep_loai_nut!r}"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống đều là một câu raise ValueError(...) — chỗ trống đầu nằm TRONG nhánh BinOp (biết node.op lạ), chỗ trống sau nằm ở nhánh else NGOÀI CÙNG (biết cả node lạ, không phải Constant hay BinOp).
- kind: strategy
  body: 'Dùng type(...).__name__ để LẤY TÊN THẬT của thứ chưa hỗ trợ, đừng viết tay một chuỗi cố định. Nhánh trong: raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}"). Nhánh ngoài: raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}").'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}") và raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}").'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ ném một ValueError — điền một câu không làm gì (như True, 1, 0) để hàm rơi vào im lặng, trả về None như lúc chưa sửa
  requireAst:
  - kind: uses-call, target: ValueError, min: 2
  # min: 2 — đếm thật trên solution: ValueError(...) được GỌI đúng 2 lần
  # trong MÃ NGUỒN, đúng khớp hai chỗ trống — không có lời gọi ValueError
  # nào khác có sẵn trong khung (except ValueError chỉ là TÊN LOẠI lỗi
  # trong mệnh đề except, không phải một lời GỌI). ĐÃ THỬ THẬT bằng cả ba
  # cách True/1/0 cho cả hai chỗ trống: cả ba đều còn 0 lời gọi ValueError
  # — dưới 2, luật chặn được. Cả ba đều DỪNG AN TOÀN: ket_qua_binh_thuong
  # vẫn đúng 14 (không đụng nhánh else), nhưng tinh("10 - 3") và tinh("-3")
  # không ném lỗi nào (hàm rơi hết thân mà không return, trả về None) —
  # nên cả hai nhánh try/except đều đi vào đường "KHÔNG NÉM LỖI NÀO", sai
  # với thông điệp mong đợi — assert bắt được độc lập với luật static này.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^14\\nchưa hỗ trợ phép toán: Sub\\nchưa hỗ trợ loại nút: UnaryOp\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không còn `None` trốn trong bóng tối nữa. Gặp cái gì lạ, nó dừng lại và
nói thẳng: đây, đúng chỗ tôi không biết.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bộ đánh giá giờ đã báo lỗi RÕ khi gặp `Sub` thay vì trả về `None` âm
thầm. Nhưng báo lỗi không phải đích cuối Byte muốn — Byte muốn nó THẬT
SỰ tính được cả phép trừ. Sửa đúng CHỖ NÀO trong hàm để nó không còn
phải báo lỗi cho `Sub` nữa — mà không cần viết lại từ đầu, không đụng
tới phần `Add`/`Mult` đã chạy đúng?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
