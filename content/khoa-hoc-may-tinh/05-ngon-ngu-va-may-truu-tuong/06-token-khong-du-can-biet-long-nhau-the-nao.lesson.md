---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.token-khong-du-can-biet-long-nhau-the-nao
title: "Token không đủ — cần biết chúng LỒNG NHAU thế nào"
summary: "Token hoá 2 + 3 * 4 và (2 + 3) * 4 cho ra hai dãy gần giống hệt nhau — chỉ khác đúng hai token dấu ngoặc. Nhưng 14 khác hẳn 20. Một dãy PHẲNG các token không nói được máy phải tính phép nào trước."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ngu.tokens-need-structure]
requires: [ngu.hand-tokenize]
concepts: [ngu.tokens-need-structure]
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
Nhìn thẳng vào hai dãy token đó đi — chúng gần như là MỘT dãy, chỉ khác
đúng hai miếng.
::::

::::explain{#day-phang-khong-noi-duoc}
Bốn bài trước cắt một dòng chữ thành từng miếng (token), gán cho mỗi
miếng một loại, rồi kiểm thứ tự các loại đó có hợp lệ không. Cả bốn bài
đều làm việc trên một dãy **PHẲNG** — token này đứng ngay sau token kia,
xếp thành một hàng ngang, không có khái niệm "bên trong" hay "bên
ngoài".

Đặt hai biểu thức cạnh nhau: `2 + 3 * 4` và `(2 + 3) * 4`. Token hoá cả
hai (chỉ giữ token có nghĩa), dãy thứ hai chỉ THÊM đúng hai token —
`OP '('` và `OP ')'` — so với dãy thứ nhất. Bỏ hai token đó đi, hai dãy
còn lại **giống hệt nhau**: cùng số, cùng dấu, cùng thứ tự.

Nhưng `2 + 3 * 4` bằng `14`, còn `(2 + 3) * 4` bằng `20`. Hai kết quả
khác hẳn nhau, trong khi phần "lõi" của hai dãy token lại y hệt. Đọc
dãy token từ trái sang phải, không có chỗ nào NÓI RA rằng `2` và `3`
phải gộp lại trước rồi mới nhân với `4` — thứ đó không nằm trong danh
sách các miếng, nó nằm ở cách các miếng **LỒNG VÀO NHAU**. Và một dãy
phẳng, theo định nghĩa, không có khái niệm lồng nhau nào cả.

Ngay cả bản thân hai token dấu ngoặc cũng không cứu được điều đó: bài 3
đã dạy `token.tok_name` gán LOẠI cho mỗi token — và loại của `(` với `)`
chỉ là `OP`, **giống hệt loại của `+` và `*`**. Không có loại riêng nào
tên là "NGOẶC" để đánh dấu "mọi thứ từ đây tới đó phải gộp lại trước".
Nhìn vào LOẠI token không phân biệt được dấu ngoặc với một toán tử bình
thường.
::::

::::example{#loc-ngoac-con-lai-y-het}
Byte token hoá cả hai biểu thức, in nguyên danh sách:

```python title=readonly
import tokenize, io, token

def token_hoa(nguon):
    return [(token.tok_name[t.type], t.string)
            for t in tokenize.generate_tokens(io.StringIO(nguon).readline)
            if t.string.strip() != ""]

print(token_hoa("2 + 3 * 4"))
print(token_hoa("(2 + 3) * 4"))
```

```text title=readonly
[('NUMBER', '2'), ('OP', '+'), ('NUMBER', '3'), ('OP', '*'), ('NUMBER', '4')]
[('OP', '('), ('NUMBER', '2'), ('OP', '+'), ('NUMBER', '3'), ('OP', ')'), ('OP', '*'), ('NUMBER', '4')]
```

Dãy thứ hai dài hơn đúng hai phần tử: `('OP', '(')` ở đầu, `('OP', ')')`
ở giữa. Bỏ đúng hai phần tử đó khỏi dãy thứ hai, phần còn lại là
`[('NUMBER', '2'), ('OP', '+'), ('NUMBER', '3'), ('OP', '*'), ('NUMBER', '4')]`
— so từng phần tử một với dãy thứ nhất, khớp tuyệt đối, không lệch một
ký tự.

Vậy mà `2 + 3 * 4` tính ra `14`, còn `(2 + 3) * 4` tính ra `20`. Cùng
một "bộ nguyên liệu" token, hai kết quả khác hẳn nhau — vì thứ quyết
định kết quả không nằm trong DANH SÁCH các token, mà nằm trong cách
chúng **lồng vào nhau**, một thứ dãy phẳng không mang theo được.
::::

::::predict{#doan-cap-doi-tru commitOnce}
Một cặp biểu thức khác: `10 - 3 - 2` (không ngoặc) và `10 - (3 - 2)`
(có ngoặc). Byte token hoá cả hai, rồi bỏ đúng hai token dấu ngoặc khỏi
dãy thứ hai.

**Trước khi chạy máy**, bạn đoán: hai dãy token (loại, nội dung) — dãy
thứ nhất, và dãy thứ hai ĐÃ BỎ NGOẶC — trông thế nào?

:::opt{correct}
Giống hệt nhau — cả hai đều là năm cặp
`[('NUMBER','10'), ('OP','-'), ('NUMBER','3'), ('OP','-'), ('NUMBER','2')]`
— dù giá trị tính ra khác hẳn nhau (`5` và `9`)
:::

:::opt
Khác nhau ở một chỗ — biểu thức có ngoặc phải có thêm một token đặc
biệt đánh dấu "đây là phần lồng bên trong", không đơn thuần là bỏ hai
dấu ngoặc đi là xong
::why
Gần đúng ở trực giác rằng biểu thức có ngoặc "phải mang thêm thông tin"
nào đó so với biểu thức không ngoặc — đúng là nó khác nhau về Ý NGHĨA.

Chỗ lệch: không có token "đặc biệt" nào như vậy trong `tokenize`. Bài
này vừa chỉ ra: dấu ngoặc chỉ là hai token loại `OP` bình thường, không
khác gì `+`/`-`. Bỏ đúng hai token đó đi, phần còn lại y hệt dãy không
ngoặc — không thiếu, không thừa, không có token "đánh dấu" nào khác.
::
:::

:::opt
Giống nhau, và vì giống nhau nên giá trị tính ra cũng phải giống nhau —
cùng số, cùng dấu thì phải cùng kết quả
::why
Gần đúng ở việc bạn quan sát đúng: hai dãy token (đã bỏ ngoặc) THẬT SỰ
giống hệt nhau — quan sát đó chính xác.

Chỗ lệch nằm ở suy luận tiếp theo. `10 - 3 - 2` bằng `5`,
`10 - (3 - 2)` bằng `9` — hai giá trị KHÁC nhau, dù token giống hệt.
Đây chính là điều bài học này muốn chỉ ra: hai dãy token phẳng giống
nhau không hề đảm bảo hai kết quả giống nhau, vì token phẳng không giữ
thông tin về cách các phép tính LỒNG vào nhau.
::
:::

:::opt
Không so sánh được, vì hai biểu thức có độ dài token khác nhau (bảy
token có ngoặc, năm token không ngoặc)
::why
Gần đúng ở việc đếm đúng: `10 - (3 - 2)` (chưa bỏ ngoặc) đúng là có bảy
token, dài hơn `10 - 3 - 2` (năm token) — con số đó không sai.

Chỗ lệch: câu hỏi hỏi về dãy thứ hai SAU KHI ĐÃ BỎ hai token dấu ngoặc
— lúc đó độ dài của nó tụt xuống còn năm, bằng đúng dãy thứ nhất, và so
sánh được bình thường.
::
:::
::::

::::code{#token-giong-gia-tri-khac}
Đúng cặp biểu thức mở đầu bài: `"2 + 3 * 4"` và `"(2 + 3) * 4"`. Phần
token hoá và lọc bỏ ngoặc đã viết sẵn. Việc của bạn: điền một cặp hai
phép so sánh — token đã lọc bỏ ngoặc của hai biểu thức có giống nhau
không, và giá trị thật của hai biểu thức có giống nhau không.

```python title=starter
import tokenize, io, token

def token_co_nghia(nguon):
    return [(token.tok_name[t.type], t.string)
            for t in tokenize.generate_tokens(io.StringIO(nguon).readline)
            if t.string.strip() != ""]

bieu_thuc_1 = "2 + 3 * 4"
bieu_thuc_2 = "(2 + 3) * 4"

token_1 = token_co_nghia(bieu_thuc_1)
token_2 = token_co_nghia(bieu_thuc_2)
token_2_bo_ngoac = [tv for tv in token_2 if tv[1] not in ("(", ")")]

gia_tri_1 = 2 + 3 * 4
gia_tri_2 = (2 + 3) * 4

# một CẶP: (token đã bỏ ngoặc có giống nhau?, giá trị thật có giống nhau?)
ket_luan = ___

print(ket_luan)
```

```python title=solution
import tokenize, io, token

def token_co_nghia(nguon):
    return [(token.tok_name[t.type], t.string)
            for t in tokenize.generate_tokens(io.StringIO(nguon).readline)
            if t.string.strip() != ""]

bieu_thuc_1 = "2 + 3 * 4"
bieu_thuc_2 = "(2 + 3) * 4"

token_1 = token_co_nghia(bieu_thuc_1)
token_2 = token_co_nghia(bieu_thuc_2)
token_2_bo_ngoac = [tv for tv in token_2 if tv[1] not in ("(", ")")]

gia_tri_1 = 2 + 3 * 4
gia_tri_2 = (2 + 3) * 4

# một CẶP: (token đã bỏ ngoặc có giống nhau?, giá trị thật có giống nhau?)
ket_luan = (token_1 == token_2_bo_ngoac, gia_tri_1 == gia_tri_2)

print(ket_luan)
```

```python title=test
assert isinstance(ket_luan, tuple) and len(ket_luan) == 2, f"ket_luan phải là một CẶP hai giá trị True/False — đang là {ket_luan!r}"
assert all(isinstance(x, bool) for x in ket_luan), f"cả hai phần tử của ket_luan phải là kết quả một phép so sánh THẬT (True/False) — đang là {ket_luan!r}"
assert ket_luan == (True, False), f"token đã bỏ ngoặc phải GIỐNG nhau (True) nhưng giá trị thật phải KHÁC nhau (False) — đang ra {ket_luan}"
```

:::hints
- kind: attention
  body: Chỗ trống là một CẶP (tuple) hai phần tử, mỗi phần tử là kết quả một phép so sánh == — không phải hai dòng riêng, không phải một con số.
- kind: strategy
  body: 'Phần tử thứ nhất so token_1 với token_2_bo_ngoac (đã lọc ngoặc) bằng ==. Phần tử thứ hai so gia_tri_1 với gia_tri_2 bằng ==. Ghép cả hai vào một cặp trong ngoặc đơn: (so_sanh_thu_nhat, so_sanh_thu_hai).'
- kind: one-line
  body: 'Chỗ trống là: (token_1 == token_2_bo_ngoac, gia_tri_1 == gia_tri_2)'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải là một CẶP hai phép so sánh == thật, so ĐÚNG bốn cái tên đã có sẵn — token_1 == token_2_bo_ngoac, gia_tri_1 == gia_tri_2 — điền một câu không làm gì (như True, 1, 0), hoặc so sánh một tên với chính nó, hoặc lấy nhầm tên khác đều bị luật này chặn
  requireAst:
  # min: 2 cho ==, min: 1 cho từng cái tên trong bốn tên tham gia — đếm
  # thật trên solution: toán tử == xuất hiện đúng hai lần, và mỗi tên
  # trong (token_1, token_2_bo_ngoac, gia_tri_1, gia_tri_2) chỉ bị ĐỌC
  # đúng một lần duy nhất, y hệt ở chỗ trống này, không nơi nào khác
  # trong khung. ĐÃ THỬ THẬT: điền True/1/0 — cả ba dừng tức khắc, không
  # treo, và == lẫn cả bốn tên đều tụt về 0.
  #
  # LỖ ĐÃ TÌM RA VÀ VÁ bằng bốn dòng uses-name dưới: chạy
  # tools/kiem_dot_bien.mjs thật (sau khi sửa lệch dòng starter/solution)
  # lộ ra RẰNG chỉ đếm số lần == không đủ — đổi MỌI chỗ đọc "token_1"
  # thành "token_2_bo_ngoac" biến vế đầu thành so sánh MỘT TÊN VỚI CHÍNH
  # NÓ (luôn True, trùng đáp án đúng); đổi "gia_tri_1"/"gia_tri_2" thành
  # "token_1" biến vế sau thành so sánh list với int (luôn False, cũng
  # trùng đáp án đúng) — cả bốn đột biến qua sạch static cũ (== vẫn đếm
  # đủ 2) lẫn test lẫn output. Thêm bốn uses-name min:1 buộc CẢ BỐN tên
  # gốc phải còn bị đọc ít nhất một lần — mỗi đột biến trên xoá mất đúng
  # một trong bốn tên khỏi toàn bài, luật mới bắt được cả bốn, đã xác
  # nhận lại bằng tools/kiem_dot_bien.mjs sau khi sửa: 0 lỗ.
  - kind: uses-operator, target: "==", min: 2
  - kind: uses-name, target: token_1, min: 1
  - kind: uses-name, target: token_2_bo_ngoac, min: 1
  - kind: uses-name, target: gia_tri_1, min: 1
  - kind: uses-name, target: gia_tri_2, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\(True, False\\)\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Token giống hệt nhau, giá trị khác hẳn nhau. Một dãy phẳng không đủ —
máy cần biết chúng LỒNG NHAU thế nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vậy máy cần một cấu trúc biết nói "miếng này nằm BÊN TRONG miếng kia" —
đúng thứ mà phần Cấu trúc dữ liệu đã gọi tên: một CÂY, nút cha có nút
con. Nhưng viết một bộ dựng cây đúng luật cho toàn bộ ngôn ngữ Python là
việc khổng lồ — hàng trăm luật văn phạm, hàng chục kiểu câu lệnh. Có
phải tự bạn sẽ phải tự tay viết bộ dựng cây đó không, hay Python đã có
sẵn một bộ dựng cây, dùng được ngay?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
