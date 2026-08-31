---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.moi-token-co-mot-loai
title: "Mỗi token có một LOẠI"
summary: "`2` không chỉ là một miếng chữ — nó có LOẠI NUMBER; `+` có loại OP. Cùng miếng chữ khác loại thì nghĩa khác hẳn: bên trong một chuỗi ký tự, dấu `+` không còn là toán tử — nó chỉ là một ký tự trong nội dung của MỘT token STRING."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ngu.token-type]
requires: [ngu.tokenize]
concepts: [ngu.token-type]
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
Biết miếng chữ là gì rồi. Nhưng miếng chữ `'+'` đó THỰC SỰ LÀ GÌ — một ký
tự bình thường, hay một toán tử? Hôm nay trả lời.
::::

::::explain{#loai-token}
Bài trước dừng lại ở việc nhìn thấy `.string` của mỗi token — nội dung
chữ của nó. Nhưng một token còn giữ một thông tin khác, quan trọng
không kém: **loại** (type) của nó.

`2` có loại `NUMBER` — nó là một con số. `+` có loại `OP` — nó là một
toán tử. Python có sẵn module `token`, ánh xạ con số loại (khó nhớ) sang
một cái tên đọc được: `token.tok_name[tok.type]` trả về đúng chuỗi như
`'NUMBER'` hay `'OP'`.

Vì sao phải phân biệt LOẠI, không chỉ dựa vào chuỗi ký tự? Vì CÙNG một
chuỗi ký tự có thể mang hai vai trò khác hẳn nhau, tuỳ NGỮ CẢNH nó xuất
hiện. Dấu `+` đứng giữa hai số là một toán tử. Nhưng nếu dấu `+` đó nằm
BÊN TRONG một cặp dấu nháy — tức là một phần nội dung của một chuỗi ký
tự — nó không còn là toán tử nữa. Cả cụm nằm trong dấu nháy, kể cả dấu
`+`, bị "nuốt" vào làm nội dung của MỘT token duy nhất, loại `STRING`.
::::

::::example{#cung-chu-khac-loai}
Byte token hoá hai dòng gần giống hệt nhau về mặt CHỮ, chỉ khác một cặp
dấu nháy:

```python title=readonly
import tokenize, io, token

def token_hoa(nguon):
    print("nguồn:", repr(nguon))
    for tok in tokenize.generate_tokens(io.StringIO(nguon).readline):
        if tok.string.strip() != "":
            print(" ", token.tok_name[tok.type], repr(tok.string))

token_hoa("5 + 2")
token_hoa("'5 + 2'")
```

```text title=readonly
nguồn: '5 + 2'
  NUMBER '5'
  OP '+'
  NUMBER '2'
nguồn: "'5 + 2'"
  STRING "'5 + 2'"
```

Dòng đầu là MÃ: ba token riêng biệt, `NUMBER`, `OP`, `NUMBER` — đúng như
bài trước đã thấy. Dòng sau là một CHUỖI KÝ TỰ (bọc trong dấu nháy đơn):
đúng MỘT token duy nhất, loại `STRING`, nội dung là cả cụm `'5 + 2'`
nguyên vẹn — kể cả dấu `+` bên trong. Dấu `+` đó không tách ra thành
token `OP` riêng nào cả. Cùng năm ký tự `5 + 2`, khác đúng một cặp dấu
nháy bao quanh, ra hai kết quả token hoá hoàn toàn khác nhau.
::::

::::predict{#nhan-trong-chuoi commitOnce}
Byte token hoá dòng `'3 * 9'` — một CHUỖI, có dấu nháy đơn bao quanh cả
cụm (khác ví dụ vừa xem: lần này là dấu nhân `*`, không phải dấu cộng).

**Trước khi chạy thử**, bạn đoán: miếng chữ `*` bên trong chuỗi này có
xuất hiện như một token riêng, loại `OP`, không?

:::opt{correct}
Không — dòng này chỉ có ĐÚNG MỘT token có nghĩa, loại `STRING`, nội dung
là cả cụm `'3 * 9'` kể cả dấu nháy; dấu `*` bên trong không tách ra
thành token riêng nào cả
:::

:::opt
Có — dấu `*` luôn luôn là loại `OP`, bất kể nó nằm ở đâu trong dòng chữ
::why
Gần đúng khi dòng chữ KHÔNG có dấu nháy bao quanh — lúc đó đúng là `*`
luôn là `OP`, như ví dụ trước đã thấy với dấu `+` trong `"5 + 2"` (không
dấu nháy).

Chỗ lệch: một khi nó nằm TRONG cặp dấu nháy, toàn bộ cụm — kể cả dấu `*`
— đã bị "nuốt" vào làm nội dung của MỘT token `STRING` duy nhất. Dấu `*`
đó không còn đóng vai trò toán tử nữa, và không tách ra thành token nào
của riêng nó. Luật này không chỉ đúng cho dấu `+` — đúng cho MỌI ký tự
nằm trong cặp dấu nháy.
::
:::

:::opt
Có, nhưng chỉ token đầu tiên (dấu nháy mở) mới có loại `STRING`, còn `*`
vẫn tách riêng thành `OP`
::why
Gần đúng ở việc bạn nghĩ dấu nháy có thể tự đứng riêng một token.

Chỗ lệch: `tokenize` KHÔNG tách dấu nháy ra làm một token riêng. Toàn bộ
chuỗi — từ dấu nháy mở tới dấu nháy đóng, kể cả `*` ở giữa — đi chung vào
đúng MỘT token `STRING`, không có token nào khác đi kèm.
::
:::

:::opt
Không xác định được — còn tuỳ vào những gì đứng TRƯỚC dòng chữ này
::why
Gần đúng ở sự thận trọng khi gặp một trường hợp mới — thái độ đó đúng
đắn.

Chỗ lệch: ở đây không có gì mơ hồ. LOẠI của một token do chính HÌNH DẠNG
ký tự của nó quyết định — có nằm trong cặp dấu nháy hay không — cố định,
không phụ thuộc vào bất cứ gì đứng trước dòng đó.
::
:::
::::

::::code{#gan-loai-that}
Token hoá dòng `"7 * 6"`, thu lại một danh sách CẶP `(nội_dung, loại)`
cho mỗi token có nghĩa — dùng `token.tok_name[tok.type]` để lấy tên loại
đọc được.

```python title=starter
import tokenize, io, token

nguon = "7 * 6"
mieng_va_loai = []
for tok in tokenize.generate_tokens(io.StringIO(nguon).readline):
    if tok.string.strip() != "":
        loai = token.tok_name[tok.type]
        ___                     # thêm (tok.string, loai) vào mieng_va_loai

print(mieng_va_loai)
```

```python title=solution
import tokenize, io, token

nguon = "7 * 6"
mieng_va_loai = []
for tok in tokenize.generate_tokens(io.StringIO(nguon).readline):
    if tok.string.strip() != "":
        loai = token.tok_name[tok.type]
        mieng_va_loai.append((tok.string, loai))

print(mieng_va_loai)
```

```python title=test
assert mieng_va_loai == [("7", "NUMBER"), ("*", "OP"), ("6", "NUMBER")], f"mieng_va_loai phải đúng ba cặp (nội_dung, loại) theo thứ tự — đang ra {mieng_va_loai}"
assert mieng_va_loai[1][1] == "OP", f"miếng thứ hai ('*') phải có loại OP — đang là {mieng_va_loai[1][1]}"
```

:::hints
- kind: attention
  body: Chỉ một chỗ trống — thêm MỘT CẶP (tok.string, loai) vào mieng_va_loai, dùng .append(...). Biến loai đã được tính sẵn ở dòng ngay trên.
- kind: strategy
  body: 'Mỗi phần tử của mieng_va_loai phải là một tuple hai giá trị: nội dung token trước, loại token sau — đúng thứ tự (tok.string, loai).'
- kind: one-line
  body: 'Chỗ trống là: mieng_va_loai.append((tok.string, loai))'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ thêm cặp (tok.string, loai) vào mieng_va_loai bằng .append(...) — điền một câu không làm gì (như True, 1, 0) sẽ để mieng_va_loai trống rỗng
  requireAst:
  # min: 1 — đếm thật trên solution: .append() xuất hiện đúng 1 lần, đúng ở
  # chỗ trống — không có .append nào khác có sẵn trong khung.
  # ĐÃ THỬ THẬT bằng cả ba cách True/1/0: vòng for chạy trên đúng số token
  # cố định mà tokenize.generate_tokens cắt ra từ nguon (chuỗi cố định,
  # không phụ thuộc chỗ trống) — cả ba dừng ngay, không vòng nào chạy vô
  # hạn. Cả ba để mieng_va_loai trống rỗng ([]), sai với kết quả mong đợi,
  # nên assert cũng bắt được độc lập với luật này.
  - kind: uses-call, target: append, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\[\\('7', 'NUMBER'\\), \\('\\*', 'OP'\\), \\('6', 'NUMBER'\\)\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không chỉ CHỮ GÌ — còn LOẠI GÌ. Hai thứ khác nhau, và cả hai đều cần
thiết để hiểu đúng một dòng lệnh.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Biết loại của từng token riêng lẻ rồi. Nhưng thử tưởng tượng dòng chữ
`"2 3"` — hai con số đứng NGAY CẠNH NHAU, không có gì ở giữa. Từng token
riêng lẻ đều hợp lệ: `2` là một `NUMBER` hợp lệ, `3` cũng vậy. Vậy CẶP
đó, đứng liền nhau như thế, có được văn phạm chấp nhận không? Bài sau
trả lời.
::::

::::checkpoint{mastery=0.8}
::::
