---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.them-mot-phep-toan-la-them-mot-nhanh-if
title: "Thêm một phép toán mới = thêm một nhánh if"
summary: "Mở rộng danh_gia bài 13 để hiểu ast.Sub — không viết lại từ đầu, chỉ thêm đúng một elif isinstance(node.op, ast.Sub), trước else báo lỗi. Bộ đánh giá lớn dần từng khái niệm một, đúng nhịp cả track đã đi."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ngu.extend-evaluator]
requires: [ngu.evaluator-limited]
concepts: [ngu.extend-evaluator]
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
Báo lỗi rõ là bước một. Bước hai: dạy nó luôn, để khỏi phải báo lỗi ấy
nữa.
::::

::::explain{#mot-nhanh-khong-hon}
Bài trước sửa `danh_gia` để nó không còn im lặng nữa — gặp `Sub`, nó
ném `ValueError` nêu đích danh. Đó là một điểm dừng ĐÚNG, nhưng vẫn là
một điểm DỪNG. Byte muốn hàm này THẬT SỰ tính được phép trừ.

Nhìn lại cấu trúc của nhánh `BinOp`:

```python title=readonly
if isinstance(node.op, ast.Add):
    return trai + phai
elif isinstance(node.op, ast.Mult):
    return trai * phai
else:
    raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
```

Dạy hàm này hiểu thêm MỘT phép toán không cần viết lại `Add`, không cần
đụng tới `Mult`, không cần sửa nhánh `else` — chỉ cần chen thêm đúng
MỘT `elif` mới, TRƯỚC `else`:

```python title=readonly
if isinstance(node.op, ast.Add):
    return trai + phai
elif isinstance(node.op, ast.Mult):
    return trai * phai
elif isinstance(node.op, ast.Sub):
    return trai - phai
else:
    raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
```

Đúng bốn dòng thêm vào, không dòng nào khác trong cả hàm bị đụng tới.
`Add` và `Mult` chạy y hệt trước, `else` vẫn giữ nguyên vai trò báo lỗi
— chỉ là giờ nó chỉ còn phải báo lỗi cho những phép NGOÀI bốn phép đã
biết. Đây chính là ý "bộ đánh giá lớn dần" — mỗi khái niệm mới thêm
đúng một nhánh, không viết lại từ đầu, giống cách cả track này đi từng
bước một kể từ bài 1.
::::

::::example{#truoc-sau-doi-chieu}
Chạy `tinh("10 - 3")` và `tinh("2 + 3 * 4")` với hàm ĐÃ thêm nhánh
`Sub`:

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
        else:
            raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}")

def tinh(bt):
    cay = ast.parse(bt, mode="eval")
    return danh_gia(cay.body)

print(tinh("10 - 3"))
print(tinh("2 + 3 * 4"))
```

```text title=readonly
7
14
```

`10 - 3` giờ tính đúng `7` — không còn ném lỗi nữa. Và `2 + 3 * 4` vẫn
ra đúng `14` như bài 11-13 — thêm nhánh mới không làm hỏng gì đã đúng
từ trước, đúng bằng chứng "mở rộng, không viết lại".
::::

::::predict{#doan-chia commitOnce}
Hàm vừa thêm đúng MỘT nhánh cho `Sub` — không đụng gì khác. Phép CHIA
(`ast.Div`, như trong `"8 / 2"`) chưa hề được nhắc tới trong nhánh mới
đó.

**Trước khi chạy**, bạn đoán: gọi `tinh("8 / 2")` sẽ ra sao?

:::opt{correct}
Vẫn ném `ValueError`, thông điệp vẫn nêu đích danh — nhưng lần này là
`Div`, không phải `Sub`
:::

:::opt
`4.0` — vì phép chia là phép toán cơ bản, hẳn bộ đánh giá đã hiểu sẵn
::why
Gần đúng ở việc `4.0` đúng là kết quả TOÁN HỌC thật của `8 / 2` — không
sai về mặt tính toán.

Chỗ lệch: hàm `danh_gia` chỉ hiểu ĐÚNG những phép có một nhánh `if`/
`elif` viết cho nó. Bài này chỉ thêm một nhánh DUY NHẤT — cho `Sub` —
không đụng gì tới `Div`. Chưa có nhánh nào kiểm `isinstance(node.op,
ast.Div)`, nên `Div` vẫn rơi thẳng vào `else`, y hệt `Sub` từng làm ở
bài trước.
::
:::

:::opt
Ném `ValueError`, nhưng thông điệp vẫn nêu `Sub` — vì đó là lỗi gần
nhất bộ đánh giá từng gặp
::why
Gần đúng ở việc bạn nhớ đúng: `Sub` từng là cái tên xuất hiện trong
thông điệp lỗi ở bài trước.

Chỗ lệch: thông điệp lỗi không "nhớ" lần lỗi trước — mỗi lần gọi
`raise ValueError(f"...{type(node.op).__name__}")`, nó đọc TRỰC TIẾP
kiểu THẬT của `node.op` ngay lúc đó. Với `"8 / 2"`, `node.op` là một
`ast.Div`, nên `type(node.op).__name__` cho ra đúng chuỗi `'Div'`,
không phải `'Sub'`.
::
:::

:::opt
Lặng lẽ trả về `None` — vì `Div` là phép mới, giống hệt `Sub` lúc chưa
sửa ở bài trước
::why
Gần đúng ở việc bạn nhớ đúng: `Sub` từng gây ra `None` âm thầm — TRƯỚC
KHI bài 13 sửa lỗ hổng đó.

Chỗ lệch: cái lỗ hổng ấy đã được vá Ở TẦNG `else`, không phải vá riêng
cho từng phép. Mọi phép chưa có nhánh riêng — kể cả `Div` — đều rơi vào
đúng `else` đó, và `else` giờ luôn `raise`, không còn đường nào để hàm
chạy hết thân mà không trả lời gì.
::
:::
::::

::::code{#them-nhanh-sub}
Byte muốn `danh_gia` tính được cả phép trừ. Khung hàm dưới đã có sẵn
nhánh `Add`, `Mult`, và cả nhánh `elif isinstance(node.op, ast.Sub):`
— chỉ còn thiếu đúng MỘT dòng: giá trị cần TRẢ VỀ cho phép trừ đó.

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
            ___                     # trả về đúng phép TRỪ: trai - phai
        else:
            raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}")

def tinh(bt):
    cay = ast.parse(bt, mode="eval")
    return danh_gia(cay.body)

ket_qua_tru = tinh("10 - 3")
ket_qua_cu = tinh("2 + 3 * 4")

try:
    tinh("8 / 2")
    thong_diep_chia = "KHÔNG NÉM LỖI NÀO"
except ValueError as loi:
    thong_diep_chia = str(loi)

print(ket_qua_tru)
print(ket_qua_cu)
print(thong_diep_chia)
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
        else:
            raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}")

def tinh(bt):
    cay = ast.parse(bt, mode="eval")
    return danh_gia(cay.body)

ket_qua_tru = tinh("10 - 3")
ket_qua_cu = tinh("2 + 3 * 4")

try:
    tinh("8 / 2")
    thong_diep_chia = "KHÔNG NÉM LỖI NÀO"
except ValueError as loi:
    thong_diep_chia = str(loi)

print(ket_qua_tru)
print(ket_qua_cu)
print(thong_diep_chia)
```

```python title=test
assert ket_qua_tru == 7, f"10 - 3 = 7 — đang ra {ket_qua_tru}"
assert ket_qua_cu == 14, f"2 + 3 * 4 vẫn phải ra 14 như trước — nhánh Sub mới không được làm hỏng Add/Mult, đang ra {ket_qua_cu}"
assert thong_diep_chia == "chưa hỗ trợ phép toán: Div", f"Div vẫn CHƯA được thêm nhánh nào — phải vẫn ném lỗi nêu đích danh Div — đang là {thong_diep_chia!r}"
```

:::hints
- kind: attention
  body: Chỗ trống chỉ là MỘT biểu thức — giá trị hàm phải trả về khi node.op là ast.Sub. Nhánh elif isinstance(node.op, ast.Sub) đã viết sẵn, chỉ thiếu return.
- kind: strategy
  body: 'Phép trừ trên hai giá trị đã hỏi được từ hai nhánh con (trai, phai) chính là trai - phai — đúng thứ tự, trái trừ phải, không phải phải trừ trái.'
- kind: one-line
  body: 'Chỗ trống là: return trai - phai'
:::

:::validate
# KHÔNG có tier `static` ở bước này — cố tình, không phải quên.
#
# Từng thử `forbidAst: [uses-call, target: eval]` để chặn việc lách bằng
# eval(). Đo thật bằng kiemAst() lộ ra: luật đó ĐẠT trên cả lời giải THẬT
# lẫn trên câu điền bừa `True`/`1`/`0` — vì không gọi eval() cũng không gọi
# eval(), luật không hề PHÂN BIỆT được hai bên, đúng thứ luật 3b của
# `kiem_ma_bai_hoc.mjs` bắt được: "một luật đạt trên cả hai thì nó không
# kiểm gì cả". Từng cân nhắc thay bằng `requireAst: uses-name target: trai`
# + `phai` — nhưng một lời giải ĐÚNG khác, ví dụ
# `return danh_gia(node.left) - danh_gia(node.right)` (tính lại thay vì
# dùng trai/phai đã có), không hề đọc hai tên đó — đúng lỗ luật 2 của
# `luat-cham-diem.md`. Không nghĩ ra luật static nào vừa CHẶN được điền bừa
# vừa KHÔNG chặn oan lời giải đúng khác, nên bỏ hẳn tầng này — đúng lời
# luật 2 dặn: "không nghĩ ra được thì thường luật ấy không cần thiết".
# Đúng-sai bằng SỐ đã có run/tests/output lo trọn: ĐÃ THỬ THẬT bằng cả ba
# cách True/1/0 cho chỗ trống — chỗ trống đó THAY THẾ NGUYÊN DÒNG (không có
# sẵn từ khoá `return` phía trước), nên True/1/0 chỉ là một BIỂU THỨC RỖNG
# không làm gì — nhánh Sub kết thúc mà không `return`, cả hàm rơi hết
# thân, trả về `None` ngầm định. Cả ba đều DỪNG AN TOÀN (không đệ quy
# thêm, không lặp gì) và ket_qua_tru ra `None` thay vì 7 — assert/output
# bắt được độc lập.
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^7\\n14\\nchưa hỗ trợ phép toán: Div\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn dòng thêm vào, không dòng nào khác bị đụng tới — và phép trừ giờ
đã tính đúng, `Add`/`Mult` chạy y hệt như trước.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`Sub` giờ đã xong — bằng đúng MỘT nhánh `elif` thêm vào, đúng công thức
bài này vừa dùng. Bộ đánh giá vẫn còn báo lỗi cho một phép rất quen
thuộc: chia (`Div`). Nếu thêm nó bằng đúng công thức đó — thêm đúng một
nhánh nữa — bộ đánh giá có tính đúng cả bốn phép `+ - * /` trên MỌI
biểu thức không, kể cả những biểu thức có NGOẶC như `"(2 + 3) * 4"`?
Hay ngoặc cần thêm một nhánh `if` riêng, như từng phép toán vậy?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
