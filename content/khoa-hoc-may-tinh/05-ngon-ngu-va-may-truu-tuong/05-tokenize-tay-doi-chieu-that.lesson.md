---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.tokenize-tay-doi-chieu-that
title: "Tokenize tay một biểu thức, đối chiếu với máy thật"
summary: "Bài chốt cụm: tự liệt kê danh sách token dự đoán (loại + nội dung) của một biểu thức nhỏ, TRƯỚC khi chạy máy — rồi chạy tokenize.generate_tokens() thật để đối chiếu. Không đoán suông; máy thật là trọng tài."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ngu.hand-tokenize]
requires: [ngu.token-order-rule]
concepts: [ngu.hand-tokenize]
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
Bốn bài để hiểu token hoá bằng cách XEM máy làm. Hôm nay đảo lại: bạn dự
đoán TRƯỚC, máy chỉ chấm điểm dự đoán của bạn thôi.
::::

::::explain{#doan-truoc-doi-chieu-sau}
Cụm bài này đi đúng ba bước: một dòng chữ thô bị cắt thành MIẾNG (bài
2), mỗi miếng có một LOẠI (bài 3), và thứ tự các LOẠI đó phải đúng luật
(bài 4). Bài chốt cụm hôm nay không thêm khái niệm mới — nó bắt bạn LÀM
đúng ba bước đó bằng chính đầu mình, trước khi hỏi máy.

Cách làm: cho một biểu thức nhỏ, tự liệt kê ra danh sách token bạn dự
đoán — mỗi phần tử là một cặp (loại, nội dung), đúng thứ tự — TRƯỚC khi
chạy `tokenize.generate_tokens()` thật. Rồi đối chiếu hai danh sách:
của bạn, và của máy. Không đoán suông rồi bỏ qua — máy thật luôn là
trọng tài cuối cùng.

Một cái bẫy hay gặp khi tự liệt kê tay: quên mất hai token máy LUÔN tự
thêm ở cuối dòng — `NEWLINE` và `ENDMARKER` (bài 2 đã gặp). Ví dụ dưới
đây cho thấy đúng cái bẫy đó.
::::

::::example{#doan-tay-quen-mot-thu}
Byte tự đoán tay token của `"6 + 7"`, viết ra TRƯỚC khi chạy máy:

```text title=readonly
Dự đoán của Byte: NUMBER '6', OP '+', NUMBER '7' — hết.
```

Rồi chạy thật:

```python title=readonly
import tokenize, io

nguon = "6 + 7"
for tok in tokenize.generate_tokens(io.StringIO(nguon).readline):
    print(tok)
```

```text title=readonly
TokenInfo(type=2 (NUMBER), string='6', start=(1, 0), end=(1, 1), line='6 + 7')
TokenInfo(type=55 (OP), string='+', start=(1, 2), end=(1, 3), line='6 + 7')
TokenInfo(type=2 (NUMBER), string='7', start=(1, 4), end=(1, 5), line='6 + 7')
TokenInfo(type=4 (NEWLINE), string='', start=(1, 5), end=(1, 6), line='6 + 7')
TokenInfo(type=0 (ENDMARKER), string='', start=(2, 0), end=(2, 0), line='')
```

Ba token đầu khớp đúng dự đoán của Byte: `NUMBER '6'`, `OP '+'`,
`NUMBER '7'`. Nhưng máy còn thêm `NEWLINE` và `ENDMARKER` ở cuối — hai
token Byte quên mất, vì chúng không phải một MIẾNG của phép tính. Nếu
bài tập chỉ so những MIẾNG CÓ NGHĨA (đã lọc theo `.string` khác rỗng,
như bài 2-4 làm), dự đoán của Byte khớp hoàn toàn. Nếu so NGUYÊN VẸN
danh sách máy trả về, dự đoán của Byte thiếu mất hai token cuối.
::::

::::predict{#doan-tay-truoc commitOnce}
Biểu thức `"18 / 3 - 1"`. Chỉ tính các token CÓ NGHĨA (không tính
`NEWLINE`/`ENDMARKER`, như bài 2-4 vẫn làm).

**Trước khi chạy máy**, bạn đoán: danh sách cặp (loại, nội dung), đúng
thứ tự, là gì?

:::opt{correct}
`[('NUMBER', '18'), ('OP', '/'), ('NUMBER', '3'), ('OP', '-'), ('NUMBER', '1')]`
:::

:::opt
`[('NUMBER', '18'), ('OP', '/'), ('NUMBER', '3'), ('OP', '-1')]` — gộp
dấu trừ và số `1` lại thành một token vì `"-1"` là một số âm quen mắt
::why
Gần đúng ở việc `"-1"` NHÌN giống một số âm quen thuộc trong toán học
thông thường.

Chỗ lệch: trong biểu thức có phép TRỪ đứng giữa hai giá trị (`18 / 3`
rồi trừ `1`), `tokenize` cắt riêng dấu `-` (loại `OP`) và số `1` (loại
`NUMBER`) thành HAI token khác nhau. Nó không "đoán" bạn định viết số âm
hay phép trừ — chỉ cắt theo hình dạng ký tự liền kề, và dấu `-` đứng
tách biệt khỏi số `1` bằng một khoảng trắng.
::
:::

:::opt
`[('NUM', '18'), ('OP', '/'), ('NUM', '3'), ('OP', '-'), ('NUM', '1')]`
— tên loại số là `NUM`, viết tắt
::why
Gần đúng ở việc bạn nhớ đúng CÓ một loại dành riêng cho số — không sai
ở việc có một loại như vậy.

Chỗ lệch: sai đúng cái TÊN. `token.tok_name[tok.type]` của Python trả
về đúng chuỗi `'NUMBER'`, viết đủ — không phải dạng viết tắt `'NUM'`.
::
:::

:::opt
`[('NUMBER', '18'), ('NUMBER', '3'), ('NUMBER', '1'), ('OP', '/'), ('OP', '-')]`
— gom hết số lại trước, dấu toán tử ra sau
::why
Gần đúng ở việc bạn liệt kê đủ cả năm token, không thiếu không thừa cái
nào — đúng số lượng.

Chỗ lệch: `tokenize` trả về token theo đúng THỨ TỰ chúng xuất hiện
trong dòng chữ gốc, đọc từ trái sang phải — không gom nhóm lại theo
LOẠI. Thứ tự đúng phải xen kẽ số và toán tử, giống hệt cách chúng nằm
trong `"18 / 3 - 1"`.
::
:::
::::

::::code{#doi-chieu-that}
Biểu thức `"4 * 5 + 6"`. Trước khi chạy máy, tự tay viết dự đoán của bạn
vào `du_doan` — một danh sách các cặp `(loại, nội_dung)`, đúng thứ tự,
chỉ tính token CÓ NGHĨA. Phần tính `token_that` bằng máy thật đã viết
sẵn, đặt SAU dự đoán của bạn — bạn không nhìn thấy nó khi viết `du_doan`.

```python title=starter
import tokenize, io, token

nguon = "4 * 5 + 6"

# Bước 1: TỰ TAY liệt kê token bạn dự đoán, mỗi phần tử là (loại, nội_dung)
du_doan = ___

# Bước 2: chạy máy thật để lấy token_that, rồi đối chiếu
token_that = []
for tok in tokenize.generate_tokens(io.StringIO(nguon).readline):
    if tok.string.strip() != "":
        token_that.append((token.tok_name[tok.type], tok.string))

print(du_doan == token_that)
print(token_that)
```

```python title=solution
import tokenize, io, token

nguon = "4 * 5 + 6"

# Bước 1: TỰ TAY liệt kê token bạn dự đoán, mỗi phần tử là (loại, nội_dung)
du_doan = [("NUMBER", "4"), ("OP", "*"), ("NUMBER", "5"), ("OP", "+"), ("NUMBER", "6")]

# Bước 2: chạy máy thật để lấy token_that, rồi đối chiếu
token_that = []
for tok in tokenize.generate_tokens(io.StringIO(nguon).readline):
    if tok.string.strip() != "":
        token_that.append((token.tok_name[tok.type], tok.string))

print(du_doan == token_that)
print(token_that)
```

```python title=test
assert token_that == [("NUMBER", "4"), ("OP", "*"), ("NUMBER", "5"), ("OP", "+"), ("NUMBER", "6")], f"token_that (do máy thật tính ra) phải đúng năm cặp này — đang ra {token_that}"
assert du_doan == token_that, f"du_doan (dự đoán tay của bạn) phải khớp CHÍNH XÁC với token_that (máy thật) — đang lệch: du_doan={du_doan}, token_that={token_that}"
```

:::hints
- kind: attention
  body: Chỗ trống là một danh sách năm cặp (loại, nội_dung) — viết TAY, không gọi tokenize ở đây. Nghĩ về biểu thức "4 * 5 + 6" giống cách bài 2-3 đã làm — số nào cũng là NUMBER, dấu toán tử nào cũng là OP.
- kind: strategy
  body: 'Đọc "4 * 5 + 6" từ trái sang phải, mỗi ký tự có nghĩa thành một cặp: số 4 -> ("NUMBER", "4"); dấu * -> ("OP", "*"); số 5 -> ("NUMBER", "5"); dấu + -> ("OP", "+"); số 6 -> ("NUMBER", "6"). Ghép cả năm cặp thành một list, đúng thứ tự đó.'
- kind: one-line
  body: 'Chỗ trống là: [("NUMBER", "4"), ("OP", "*"), ("NUMBER", "5"), ("OP", "+"), ("NUMBER", "6")]'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: du_doan phải là một danh sách các cặp (loại, nội_dung) viết TAY, có mặt cả hai loại "NUMBER" và "OP" — điền một câu không làm gì (như True, 1, 0) không tạo ra danh sách nào cả
  requireAst:
  # min: 1 mỗi cái — đếm thật trên solution: hằng chuỗi "NUMBER" xuất hiện
  # 3 lần, "OP" xuất hiện 2 lần trong du_doan — không hằng nào trong số đó
  # xuất hiện ở phần TÍNH token_that (chỗ đó dùng token.tok_name[tok.type],
  # tra bảng lúc chạy, không có literal "NUMBER"/"OP" nào viết tay).
  # ĐÃ THỬ THẬT bằng cả ba cách True/1/0 cho du_doan: không vòng lặp nào
  # phụ thuộc du_doan (vòng for chỉ chạy trên nguon cố định), nên cả ba
  # dừng ngay, không treo. Cả ba cho du_doan == token_that là False (so
  # một bool/int với một list không bao giờ bằng nhau, không ném lỗi nào),
  # in ra "False" thay vì "True" — assert cũng bắt được độc lập với luật
  # static này.
  - kind: has-literal, target: "NUMBER", min: 1
  - kind: has-literal, target: "OP", min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^True\\n\\[\\('NUMBER', '4'\\), \\('OP', '\\*'\\), \\('NUMBER', '5'\\), \\('OP', '\\+'\\), \\('NUMBER', '6'\\)\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Dự đoán khớp máy thật — không phải vì bạn đoán may, mà vì bạn đã nắm
đúng ba luật của cả cụm bài: cắt miếng, gán loại, đúng thứ tự.
::::

::::reflect{#nghi-lai}
Một câu hỏi khép lại cả cụm bài về token hoá.

Token hoá `"2 + 3 * 4"` cho ra một dãy PHẲNG: `NUMBER`, `OP`, `NUMBER`,
`OP`, `NUMBER` — năm miếng, một hàng ngang. Token hoá `"(2 + 3) * 4"`
cũng cho ra một dãy gần giống hệt, chỉ thêm hai token `OP` nữa cho cặp
ngoặc. Nhưng `2 + 3 * 4` bằng `14`, còn `(2 + 3) * 4` bằng `20` — hai
kết quả khác hẳn nhau.

Nếu chỉ nhìn vào một dãy PHẲNG các token, xếp thành một hàng ngang, làm
sao biết được máy phải tính phép nào TRƯỚC? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
