---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.thu-tu-token-khong-tuy-tien
title: "Thứ tự token không tuỳ tiện"
summary: "`NUMBER OP NUMBER` (như `2 * 3`) hợp lệ; `NUMBER NUMBER` (như `2 3`) thì không — văn phạm nói được LOẠI TOKEN nào được phép đứng ngay sau loại nào. Cắt token xong (bài 2-3) chỉ là bước đầu — tokenize thành công không có nghĩa là đúng thứ tự."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ngu.token-order-rule]
requires: [ngu.token-type]
concepts: [ngu.token-order-rule]
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
Biết loại rồi. Nhưng `2` rồi lại `3` đứng NGAY CẠNH NHAU — cả hai đều là
`NUMBER` hợp lệ riêng lẻ. Cặp đó, đứng liền kề như thế, có được chấp
nhận không?
::::

::::explain{#thu-tu-cung-la-luat}
Bài 2 cắt token. Bài 3 gán loại cho mỗi token. Cả hai đều chỉ nhìn vào
TỪNG TOKEN một, tách rời. Nhưng văn phạm (bài 1) còn nói một điều nữa,
chưa dùng tới: LOẠI TOKEN nào được phép đứng NGAY SAU loại nào.

`NUMBER` rồi `OP` rồi `NUMBER` — như `2 * 3` — hợp lệ. Nhưng `NUMBER`
rồi `NUMBER` — như `2 3`, hai con số đứng liền nhau, không có gì ở giữa
— thì không, dù từng token riêng lẻ (`2` và `3`) đều là những `NUMBER`
hoàn toàn bình thường.

Một điều quan trọng cần phân biệt: `tokenize` (bài 2-3) chỉ CẮT chữ theo
hình dạng — nó không hề biết gì về việc các miếng cắt ra có ĐỨNG ĐÚNG
THỨ TỰ hay không. Việc kiểm đúng thứ tự là việc của `ast.parse` (bài 1)
— nơi văn phạm thật sự được áp dụng. Cắt token thành công không có nghĩa
là dòng chữ đó hợp lệ.
::::

::::example{#cat-duoc-nhung-sai-thu-tu}
Byte so sánh hai dòng gần giống nhau: `"2 * 3"` (đúng thứ tự) và `"2 3"`
(sai thứ tự) — token hoá CẢ HAI trước, rồi mới hỏi `ast.parse`:

```python title=readonly
import tokenize, io, token, ast

def kiem_tra(nguon):
    print("nguồn:", repr(nguon))
    for tok in tokenize.generate_tokens(io.StringIO(nguon).readline):
        if tok.string.strip() != "":
            print("  token:", token.tok_name[tok.type], repr(tok.string))
    try:
        ast.parse(nguon, mode="eval")
        print("  ast.parse: HỢP LỆ")
    except SyntaxError as loi:
        print("  ast.parse:", loi.msg)

kiem_tra("2 * 3")
kiem_tra("2 3")
```

```text title=readonly
nguồn: '2 * 3'
  token: NUMBER '2'
  token: OP '*'
  token: NUMBER '3'
  ast.parse: HỢP LỆ
nguồn: '2 3'
  token: NUMBER '2'
  token: NUMBER '3'
  ast.parse: invalid syntax
```

Cả hai dòng đều token hoá THÀNH CÔNG — không lỗi nào ở bước cắt chữ, kể
cả `"2 3"`. Khác biệt chỉ lộ ra ở bước SAU: `ast.parse("2 * 3", ...)`
dựng được, còn `ast.parse("2 3", ...)` ném `SyntaxError`. Hai `NUMBER`
đứng liền nhau vi phạm đúng một luật thứ tự — dù mỗi token riêng lẻ
chẳng có gì sai.
::::

::::predict{#tokenize-co-bao-loi-khong commitOnce}
Byte token hoá hai dòng `"2 3"` và `"2 * 3"`.

**Trước khi đọc lại ví dụ**, bạn đoán: bước TOKEN HOÁ (hàm
`tokenize.generate_tokens`) có ném lỗi ở dòng nào trong hai dòng đó
không?

:::opt{correct}
Không dòng nào cả — token hoá thành công ở CẢ HAI. `"2 3"` cho ra hai
token `NUMBER` liền nhau; `"2 * 3"` có thêm một token `OP` xen giữa. Lỗi
chỉ xuất hiện SAU đó, khi `ast.parse("2 3", ...)` từ chối vì sai thứ tự
:::

:::opt
`"2 3"` báo lỗi ngay ở bước token hoá, vì hai số liền nhau không cắt
được thành miếng
::why
Gần đúng ở việc bạn đoán đúng `"2 3"` CÓ VẤN ĐỀ — chỉ là vấn đề đó xảy
ra ở bước SAU, không phải bước cắt miếng.

Chỗ lệch: cắt miếng chỉ nhìn KÝ TỰ, không quan tâm các miếng cắt ra có
đứng ĐÚNG THỨ TỰ hợp lý hay không. `2` và `3` cắt ra hoàn toàn bình
thường, mỗi cái một token `NUMBER` riêng, không lỗi nào ở bước này.
::
:::

:::opt
Cả hai đều báo lỗi ở bước token hoá, vì `tokenize` cũng kiểm tra luôn cả
văn phạm
::why
Gần đúng ở việc bạn nghĩ `tokenize` "thông minh", biết luôn cả luật.

Chỗ lệch: module `tokenize` chỉ làm đúng MỘT việc — cắt ký tự thành
miếng theo hình dạng, hoàn toàn không biết luật thứ tự nào của văn
phạm. Kiểm luật thứ tự là việc của `ast.parse`, một bước khác hẳn, ở
sau bước cắt miếng.
::
:::

:::opt
Không dòng nào báo lỗi ở đâu cả, kể cả `ast.parse`, vì cả hai đều chỉ
gồm số và toán tử
::why
Gần đúng ở việc CẢ HAI dòng cùng chỉ chứa số và toán tử — nhìn "vô hại"
như nhau ở bề ngoài.

Chỗ lệch: `ast.parse("2 3", mode="eval")` THẬT SỰ ném `SyntaxError`. Hai
token `NUMBER` đứng liền nhau vi phạm đúng luật thứ tự mà bài này đang
học, dù mỗi token riêng lẻ không có gì sai.
::
:::
::::

::::code{#loc-dung-thu-tu}
Bốn biểu thức. Với MỖI biểu thức: token hoá trước (luôn thành công, đã
viết sẵn), rồi thử `ast.parse` — nếu dựng được, thêm nó vào
`dung_thu_tu`.

```python title=starter
import tokenize, io, ast

cac_bieu_thuc = ["2 * 3", "2 3", "* 3", "5 * 5"]

token_hoa_duoc = []
dung_thu_tu = []

for bt in cac_bieu_thuc:
    list(tokenize.generate_tokens(io.StringIO(bt).readline))
    token_hoa_duoc.append(bt)
    try:
        ast.parse(bt, mode="eval")
        ___                          # thêm bt vào dung_thu_tu
    except SyntaxError:
        pass

print(token_hoa_duoc)
print(dung_thu_tu)
```

```python title=solution
import tokenize, io, ast

cac_bieu_thuc = ["2 * 3", "2 3", "* 3", "5 * 5"]

token_hoa_duoc = []
dung_thu_tu = []

for bt in cac_bieu_thuc:
    list(tokenize.generate_tokens(io.StringIO(bt).readline))
    token_hoa_duoc.append(bt)
    try:
        ast.parse(bt, mode="eval")
        dung_thu_tu.append(bt)
    except SyntaxError:
        pass

print(token_hoa_duoc)
print(dung_thu_tu)
```

```python title=test
assert token_hoa_duoc == ["2 * 3", "2 3", "* 3", "5 * 5"], f"token_hoa_duoc phải chứa CẢ BỐN biểu thức — token hoá không loại cái nào cả — đang ra {token_hoa_duoc}"
assert dung_thu_tu == ["2 * 3", "5 * 5"], f"dung_thu_tu chỉ giữ hai biểu thức đúng thứ tự token (NUMBER OP NUMBER) — đang ra {dung_thu_tu}"
```

:::hints
- kind: attention
  body: Chỉ một chỗ trống, nằm trong nhánh try, ngay sau ast.parse(bt, mode="eval") dựng thành công — thêm bt vào dung_thu_tu bằng .append(bt).
- kind: strategy
  body: 'Dòng ast.parse(bt, mode="eval") không ném lỗi nghĩa là bt đúng thứ tự token, được văn phạm chấp nhận — thêm nó vào dung_thu_tu. Nếu nó ném SyntaxError, nhánh except bắt lấy và bỏ qua (pass) — bt không đúng thứ tự, không được thêm vào đâu cả.'
- kind: one-line
  body: 'Chỗ trống là: dung_thu_tu.append(bt)'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ thêm bt vào dung_thu_tu bằng .append(bt) khi ast.parse dựng thành công — điền một câu không làm gì (như True, 1, 0) sẽ để dung_thu_tu trống rỗng dù token_hoa_duoc vẫn đúng
  requireAst:
  # min: 2 — đếm thật trên solution: .append() xuất hiện đúng 2 lần trong mã
  # nguồn — một lần đã có sẵn trong khung (token_hoa_duoc.append(bt)), cộng
  # đúng một lần ở chỗ trống (dung_thu_tu.append(bt)).
  # ĐÃ THỬ THẬT bằng cả ba cách True/1/0: vòng for chạy trên đúng 4 phần tử
  # cố định (cac_bieu_thuc), không phụ thuộc chỗ trống — cả ba dừng ngay,
  # không vòng nào chạy vô hạn. Cả ba cho token_hoa_duoc ĐÚNG (không phụ
  # thuộc chỗ trống) nhưng dung_thu_tu trống rỗng ([]), sai với kết quả
  # mong đợi ["2 * 3", "5 * 5"], nên assert cũng bắt được độc lập với luật
  # static này.
  - kind: uses-call, target: append, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\['2 \\* 3', '2 3', '\\* 3', '5 \\* 5'\\]\\n\\['2 \\* 3', '5 \\* 5'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn biểu thức, cả bốn đều cắt token được — nhưng chỉ hai đứng đúng thứ
tự. Cắt xong chưa phải xong; còn phải đúng luật thứ tự.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa tự tay KIỂM một vài biểu thức bằng cách CHẠY `ast.parse` thật.
Nhưng trước khi chạy, nếu tự BẠN dự đoán trước — không nhìn máy — toàn
bộ danh sách token đầy đủ (loại và nội dung) của một biểu thức nhỏ, bạn
có đoán ĐÚNG HẾT không, không sót một miếng nào? Bài chốt cụm sau để bạn
tự thử.
::::

::::checkpoint{mastery=0.8}
::::
