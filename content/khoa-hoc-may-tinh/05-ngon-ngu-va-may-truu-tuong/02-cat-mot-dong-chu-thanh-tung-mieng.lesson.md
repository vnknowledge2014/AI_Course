---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.cat-mot-dong-chu-thanh-tung-mieng
title: "Cắt một dòng chữ thành từng miếng nhỏ — token hoá"
summary: "Trước khi hiểu Ý của `2 + 3 * 4`, máy phải cắt chuỗi ký tự thô đó thành các miếng có nghĩa: `2`, `+`, `3`, `*`, `4`. Module `tokenize` của Python làm đúng việc này — xem được thật bằng `tokenize.generate_tokens()`."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ngu.tokenize]
requires: [ngu.grammar-exists]
concepts: [ngu.tokenize]
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
Câu hỏi cuối bài trước: trước khi áp bất kỳ luật văn phạm nào, máy phải
nhìn thấy các MIẾNG rời rạc trong dòng chữ thô đã. Hôm nay xem đúng bước
đó.
::::

::::explain{#token-hoa-la-gi}
Một dòng chữ thô như `"2 + 3 * 4"`, với máy, ban đầu chỉ là một chuỗi ký
tự dính liền: `2`, khoảng trắng, `+`, khoảng trắng, `3`,... Trước khi
hiểu được Ý của dòng đó, máy phải cắt nó ra thành các miếng có NGHĨA
riêng: con số `2`, dấu `+`, con số `3`, dấu `*`, con số `4`. Việc cắt này
gọi là **token hoá** (tokenizing), và mỗi miếng cắt ra gọi là một
**token**.

Python có sẵn một module làm đúng việc này: `tokenize`. Hàm
`tokenize.generate_tokens(...)` nhận vào một hàm đọc từng dòng (thường
là `io.StringIO(nguon).readline` — biến một chuỗi có sẵn trong bộ nhớ
thành thứ đọc được TỪNG DÒNG, giống một tệp), và trả về lần lượt từng
token nó cắt ra được. Không cần tự viết bộ cắt chữ nào — công cụ này của
chính Python làm việc đó.
::::

::::example{#token-hoa-that}
Byte token hoá dòng `"2 + 3 * 4"`, in ra từng token một:

```python title=readonly
import tokenize, io

nguon = "2 + 3 * 4"
for tok in tokenize.generate_tokens(io.StringIO(nguon).readline):
    print(tok)
```

```text title=readonly
TokenInfo(type=2 (NUMBER), string='2', start=(1, 0), end=(1, 1), line='2 + 3 * 4')
TokenInfo(type=55 (OP), string='+', start=(1, 2), end=(1, 3), line='2 + 3 * 4')
TokenInfo(type=2 (NUMBER), string='3', start=(1, 4), end=(1, 5), line='2 + 3 * 4')
TokenInfo(type=55 (OP), string='*', start=(1, 6), end=(1, 7), line='2 + 3 * 4')
TokenInfo(type=2 (NUMBER), string='4', start=(1, 8), end=(1, 9), line='2 + 3 * 4')
TokenInfo(type=4 (NEWLINE), string='', start=(1, 9), end=(1, 10), line='2 + 3 * 4')
TokenInfo(type=0 (ENDMARKER), string='', start=(2, 0), end=(2, 0), line='')
```

Năm miếng đầu đúng như bạn hình dung: `2`, `+`, `3`, `*`, `4` — mỗi cái
nằm trong trường `string` của token. Máy còn tự thêm hai token nữa mà
không ai gõ ra: `NEWLINE` (đánh dấu hết dòng) và `ENDMARKER` (đánh dấu
hết luồng chữ) — hai cái này không phải một MIẾNG của phép tính, chỉ là
tín hiệu "hết rồi" cho phần đọc sau nó.

Mỗi token còn có một `type` — con số đứng trước tên viết hoa trong dấu
ngoặc, như `2 (NUMBER)` hay `55 (OP)`. Đó chính là chủ đề của bài sau;
hôm nay chỉ cần để ý cột `string` — đó là MIẾNG chữ đã được cắt ra.
::::

::::predict{#dem-mieng commitOnce}
Byte token hoá dòng `"9 * 5 + 1"`.

**Trước khi chạy thử**, bạn đoán: không tính hai token `NEWLINE` và
`ENDMARKER` mà máy luôn tự thêm ở cuối, có bao nhiêu miếng CÓ NGHĨA, và
chúng là gì?

:::opt{correct}
Năm miếng, đúng thứ tự: `9`, `*`, `5`, `+`, `1` — mỗi con số và mỗi dấu
toán tử là một miếng riêng
:::

:::opt
Ba miếng: `9 * 5`, `+`, `1` — vì phép nhân được ưu tiên nên máy gộp nó
thành MỘT miếng trước
::why
Gần đúng ở việc bạn nhớ đúng: phép nhân được TÍNH trước phép cộng — độ
ưu tiên toán tử có thật.

Chỗ lệch: đó là chuyện xảy ra ở bước ĐÁNH GIÁ giá trị (những bài xa hơn
trong track này), không phải ở bước TOKEN HOÁ. Token hoá chỉ cắt theo
KÝ TỰ, không hề biết gì về độ ưu tiên phép toán — `9`, `*`, `5` vẫn là
ba miếng tách rời, y hệt `+`, `1`.
::
:::

:::opt
Chín miếng — mỗi KÝ TỰ, kể cả khoảng trắng, là một miếng riêng
::why
Gần đúng ở việc bạn để ý dòng chữ gốc có đúng chín ký tự (kể cả
khoảng trắng) — đếm đó không sai.

Chỗ lệch: token hoá không cắt theo từng KÝ TỰ đơn lẻ. Nó gộp các ký tự
liền nhau THUỘC CÙNG một miếng lại làm một (ví dụ nếu có số nhiều chữ
số), và bỏ qua khoảng trắng hoàn toàn — khoảng trắng không tự nó tạo ra
token nào.
::
:::

:::opt
Bảy miếng, tính cả `NEWLINE` và `ENDMARKER` — vì máy luôn thêm chúng vào
bất kỳ dòng nào cũng nên tính là một phần của biểu thức
::why
Gần đúng — đúng là `NEWLINE`/`ENDMARKER` LUÔN được máy tự thêm vào,
bạn nhớ đúng điều đó.

Chỗ lệch: câu hỏi cố ý loại hai token đó ra khi đếm — chúng đánh dấu
"hết dòng"/"hết luồng chữ", không phải một MIẾNG có nghĩa của phép tính
`9 * 5 + 1`. Số miếng CÓ NGHĨA vẫn là năm, không phải bảy.
::
:::
::::

::::code{#cat-mieng-that}
Token hoá dòng `"10 - 4 / 2"`, thu lại đúng nội dung (`.string`) của mỗi
token CÓ NGHĨA — bỏ qua những token có `.string` chỉ toàn khoảng trắng
(như `NEWLINE`, `ENDMARKER`, đã lọc sẵn bằng điều kiện `if`).

```python title=starter
import tokenize, io

nguon = "10 - 4 / 2"
mieng = []
for tok in tokenize.generate_tokens(io.StringIO(nguon).readline):
    if tok.string.strip() != "":
        ___                     # thêm tok.string vào mieng

print(mieng)
```

```python title=solution
import tokenize, io

nguon = "10 - 4 / 2"
mieng = []
for tok in tokenize.generate_tokens(io.StringIO(nguon).readline):
    if tok.string.strip() != "":
        mieng.append(tok.string)

print(mieng)
```

```python title=test
assert mieng == ["10", "-", "4", "/", "2"], f"mieng phải đúng năm miếng theo thứ tự trong dòng chữ gốc — đang ra {mieng}"
assert len(mieng) == 5, f"phải có đúng 5 miếng có nghĩa, không tính NEWLINE/ENDMARKER — đang có {len(mieng)}"
```

:::hints
- kind: attention
  body: Chỉ một chỗ trống — thêm tok.string (nội dung của token đang xét trong vòng lặp) vào danh sách mieng, dùng .append(...).
- kind: strategy
  body: 'Ở mỗi vòng lặp, tok là một TokenInfo — tok.string chính là MIẾNG chữ (như "10" hay "-"). Điều kiện if đã lọc sẵn, chỉ còn việc thêm nó vào mieng.'
- kind: one-line
  body: 'Chỗ trống là: mieng.append(tok.string)'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ thêm tok.string vào mieng bằng .append(...) — điền một câu không làm gì (như True, 1, 0) sẽ để mieng trống rỗng
  requireAst:
  # min: 1 — đếm thật trên solution: .append() xuất hiện đúng 1 lần, đúng ở
  # chỗ trống — không có .append nào khác có sẵn trong khung.
  # ĐÃ THỬ THẬT bằng cả ba cách True/1/0: vòng for chạy trên đúng số token
  # cố định mà tokenize.generate_tokens cắt ra từ nguon (một chuỗi cố định,
  # không phụ thuộc chỗ trống) — cả ba dừng ngay, không vòng nào chạy vô
  # hạn. Cả ba để mieng trống rỗng ([]), sai với kết quả mong đợi ["10",
  # "-", "4", "/", "2"], nên assert cũng bắt được độc lập với luật này.
  - kind: uses-call, target: append, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\['10', '-', '4', '/', '2'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm miếng, đúng thứ tự trong dòng chữ gốc. Máy không "đọc" dòng chữ như
người — nó cắt trước, hiểu sau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mỗi miếng đã tách ra có một chuỗi ký tự — như `'+'`, hay `'2'`. Nhưng
miếng chữ `'+'` đó THỰC SỰ LÀ GÌ, với máy? Một ký tự nằm trong một câu
chữ bình thường, hay một toán tử? Cùng một chuỗi ký tự, hai vai trò khác
hẳn nhau — máy phân biệt bằng cách nào? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
