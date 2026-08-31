---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.danh-gia-mot-cay-la-di-bo-de-quy
title: "Đánh giá một cây là ĐI BỘ qua nó, đệ quy"
summary: "Cây cú pháp của \"2 + 3 * 4\" (bài 7-10) không tự tính ra 14 — phải có ai đó ĐI BỘ qua nó. Viết một hàm đệ quy hỏi giá trị hai nhánh con rồi gộp lại, đúng kỹ năng T3.2 bài 30 (duyệt bằng ngăn xếp) và T3.3 bài 1-2 (đệ quy chính là ngăn xếp) đã chuẩn bị sẵn, dùng cho một việc thật lần đầu tiên."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ngu.tree-walk-eval]
requires: [ds.tree-traversal, alg.recursion-is-stack]
concepts: [ngu.tree-walk-eval]
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
Bạn vừa vẽ tay một cây, đối chiếu với `ast.dump()` thật. Cây đó không tự
tính ra con số nào cả — hôm nay bạn dạy nó biết TÍNH.
::::

::::explain{#di-bo-de-tinh}
`ast.parse("2 + 3 * 4", mode="eval")` dựng ra một cây (bài 7-10) — nhưng
tự bản thân cây đó không "bằng 14". Nó chỉ là một cấu trúc dữ liệu lồng
nhau, y hệt cây bảy số báo danh bài "Duyệt cây" từng dùng. Muốn có con
số 14, phải có ai đó ĐI BỘ qua cây, ghé từng nút, và biết phải làm gì ở
mỗi loại nút. Việc đó gọi là **đánh giá** (evaluate) cây.

Hai kỹ năng bạn đã có sẵn ghép lại vừa khít cho việc này:

- T3.2 bài "Duyệt cây: ba thứ tự, ba câu chuyện khác nhau" đi bộ qua một
  cây bằng `while` cộng một ngăn xếp tự quản — ghé từng nút, không sót.
- T3.3 bài "Đệ quy đã học — giờ nối lại với ngăn xếp" và bài "Viết lại
  đúng việc cũ bằng đệ quy" chỉ ra: đúng việc đi-bộ-qua-cây đó viết lại
  bằng một hàm tự gọi lại chính mình vẫn ra cùng kết quả — chỉ là trình
  thông dịch tự quản chồng lời gọi, không cần `ngan_xep`, không cần
  `while`.

Đánh giá cây cú pháp dùng ĐÚNG kỹ năng thứ hai, cho một việc thật: không
chỉ LIỆT KÊ giá trị các nút (như bài duyệt cây từng làm) mà TÍNH một con
số duy nhất từ toàn bộ cây. Luật của hàm `danh_gia(node)`:

- Gặp `ast.Constant` (một con số): trả lời NGAY — chính giá trị của nó,
  không cần hỏi ai.
- Gặp `ast.BinOp` (một phép toán hai ngôi): phải HỎI nhánh trái trước —
  gọi lại `danh_gia(node.left)` — rồi hỏi nhánh phải — gọi lại
  `danh_gia(node.right)` — rồi mới GỘP hai câu trả lời lại theo đúng
  phép toán ghi trong `node.op`.

`ast.parse(..., mode="eval")` trả về một nút `Expression` bọc ngoài —
phần cây thật sự cần đi bộ nằm trong thuộc tính `.body` của nó, đúng
điều bài 7 đã dựng.
::::

::::example{#vi-du-hai-cong-ba-nhan-bon}
Đúng biểu thức canonical của cả cụm bài: `"2 + 3 * 4"`. Trước tiên, hình
dạng cây thật (đã chạy `ast.dump(..., indent=2)`):

```text title=readonly
Expression(
  body=BinOp(
    left=Constant(value=2),
    op=Add(),
    right=BinOp(
      left=Constant(value=3),
      op=Mult(),
      right=Constant(value=4))))
```

Và hàm `danh_gia` đi bộ đúng cây đó:

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

cay = ast.parse("2 + 3 * 4", mode="eval")
print(danh_gia(cay.body))
```

```text title=readonly
14
```

Đi theo đúng lời gọi: `danh_gia` ở gốc (`BinOp`, phép `Add`) trước tiên
gọi `danh_gia(node.left)` — gặp `Constant(value=2)`, trả lời ngay: `2`.
Rồi gọi `danh_gia(node.right)` — gặp một `BinOp` khác (phép `Mult`), lại
phải hỏi hai nhánh con của NÓ trước: `danh_gia` trên `Constant(3)` trả
`3`, trên `Constant(4)` trả `4`, gộp lại `3 * 4 = 12`. Nhánh phải của
gốc trả lời `12`. Gốc gộp `2 + 12 = 14`. Không có `ngan_xep` nào viết
tay — trình thông dịch tự giữ chồng lời gọi, đúng như T3.3 đã chỉ ra.
::::

::::predict{#gia-tri-nao-xong-truoc commitOnce}
Cây của biểu thức `"5 * 3 + 2"`:

```text title=readonly
Expression(
  body=BinOp(
    left=BinOp(
      left=Constant(value=5),
      op=Mult(),
      right=Constant(value=3)),
    op=Add(),
    right=Constant(value=2)))
```

Đi bộ đệ quy đúng luật vừa học — hỏi nhánh trái xong hoàn toàn, rồi mới
hỏi nhánh phải, rồi mới gộp. **Trước khi chạy**, bạn đoán: giá trị NÀO
được TÍNH XONG đầu tiên (kể cả những giá trị chỉ là một con số có sẵn,
không cần tính gì)?

:::opt{correct}
`5` — nó là nút lá TRÁI NGOÀI CÙNG, hỏi tới trước tiên và trả lời ngay
:::

:::opt
`2` — vì nó trông đơn giản nhất, chỉ là một con số đứng riêng
::why
Gần đúng ở việc `2` đúng là một `Constant`, cũng trả lời ngay không cần
hỏi ai — quan sát đó không sai.

Chỗ lệch: `2` là nhánh PHẢI của gốc. Luật vừa học nói rõ — nhánh trái
phải hỏi XONG HOÀN TOÀN trước khi máy quay sang hỏi nhánh phải. Nhánh
trái của gốc là cả cụm `5 * 3` (một `BinOp` khác, không phải một con số
đơn) — phải đi bộ hết CẢ nhánh đó (gồm cả hai lá `5` và `3` bên trong)
trước khi tới lượt `2` được hỏi tới.
::
:::

:::opt
`15` (kết quả của `5 * 3`) — vì đây là phần được TÍNH trước, không phải
chỉ là một con số có sẵn
::why
Gần đúng ở việc `15` đúng là giá trị hoàn tất SỚM — sớm hơn cả `2` và
sớm hơn kết quả cuối `17`.

Chỗ lệch: trước khi GỘP được `5 * 3 = 15`, cả hai nhánh con của phép
nhân đó — `5` và `3` — phải trả lời XONG trước. `15` là một giá trị
GỘP, luôn tính SAU các nút lá của chính nó, không thể là giá trị hoàn
tất đầu tiên.
::
:::

:::opt
`17` (kết quả cuối cùng) — vì đó là điều máy đang cố tính ra
::why
Gần đúng ở việc `17` đúng là câu trả lời cuối cùng bạn đang chờ.

Chỗ lệch: câu hỏi hỏi giá trị hoàn tất ĐẦU TIÊN, không phải giá trị
đích. `17` nằm ở gốc cây — gốc luôn là nút GỘP SAU CÙNG, phải đợi trọn
vẹn mọi nhánh con (cả `5`, `3`, `15` lẫn `2`) xong hết mới tới lượt nó.
::
:::
::::

::::code{#dien-vao-cho-goi-lai}
Byte muốn tính `"5 * 3 + 2"` — một biểu thức mới, chưa dùng ở ví dụ.
Khung hàm `danh_gia` đã có sẵn phần trường hợp `Constant` và phần gộp
theo `Add`/`Mult`. Việc của bạn: điền đúng HAI lời gọi lại chính hàm để
hỏi nhánh trái, rồi nhánh phải — trước khi gộp.

```python title=starter
import ast

def danh_gia(node):
    if isinstance(node, ast.Constant):
        return node.value
    if isinstance(node, ast.BinOp):
        trai = ___                     # hỏi nhánh TRÁI trước
        phai = ___                     # rồi hỏi nhánh PHẢI
        if isinstance(node.op, ast.Add):
            return trai + phai
        if isinstance(node.op, ast.Mult):
            return trai * phai

cay = ast.parse("5 * 3 + 2", mode="eval")
ket_qua = danh_gia(cay.body)
print(ket_qua)
```

```python title=solution
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

cay = ast.parse("5 * 3 + 2", mode="eval")
ket_qua = danh_gia(cay.body)
print(ket_qua)
```

```python title=test
assert ket_qua == 17, f"5 * 3 + 2 = 17 — đang ra {ket_qua}"
assert ket_qua == 5 * 3 + 2, "kết quả của bộ đánh giá phải khớp đúng phép tính Python trực tiếp trên cùng biểu thức"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống đều là LỜI GỌI LẠI chính hàm danh_gia — một cho node.left, một cho node.right. Không phải .append, không phải while — đây là đệ quy, đúng kỹ năng T3.3 đã dựng.
- kind: strategy
  body: 'Muốn biết một BinOp đáng giá bao nhiêu, trước hết phải biết hai nhánh con của nó đáng giá bao nhiêu — gọi lại đúng hàm danh_gia trên node.left và trên node.right, giữ nguyên thứ tự trái trước phải sau.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là danh_gia(node.left) và danh_gia(node.right).'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải là LỜI GỌI LẠI chính danh_gia — một trên node.left, một trên node.right — điền một câu không làm gì (như True, 1, 0) không đi bộ xuống cây, kết quả sẽ sai
  requireAst:
  - kind: uses-call, target: danh_gia, min: 3
  # min: 3 — đếm thật trên solution: danh_gia(node.left), danh_gia(node.right)
  # (hai chỗ trống) cộng danh_gia(cay.body) (lời gọi ngoài cùng, có sẵn trong
  # khung) = 3 lời gọi tới chính tên danh_gia trong TOÀN BỘ mã nguồn. ĐÃ THỬ
  # THẬT bằng cả ba cách True/1/0 cho cả hai chỗ trống: cả ba đều CHỈ còn 1
  # lời gọi (lời gọi ngoài cùng) — dưới 3, luật chặn được. Cả ba đều DỪNG AN
  # TOÀN, không đệ quy, không lặp gì — trên cây "5 * 3 + 2": True/1 cho
  # trai=phai=True(hoặc 1), gộp theo Add ở gốc ra 2 (không lỗi); 0 cho ra 0 —
  # cả ba đều SAI so với 17 và đều bị assert/output bắt độc lập.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^17\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không `ngan_xep`, không `while` — chỉ hai lời gọi lại chính hàm, và cây
tự "tính ra" đúng giá trị của nó. Trình thông dịch giữ chồng lời gọi
giùm bạn, y hệt T3.3 đã hứa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn kỹ hàm `danh_gia` vừa viết: có đúng HAI loại nhánh. Nhánh
`ast.Constant` trả lời NGAY, không gọi lại chính mình một lần nào cả.
Nhánh `ast.BinOp` PHẢI gọi lại chính mình — đúng hai lần — trước khi có
gì để trả lời.

R1.T1.3 (bài "Trường hợp dừng lại") từng đặt tên cho loại nhánh không
gọi lại chính mình. Tên đó có áp đúng cho nhánh `Constant` ở đây không —
và nhánh `BinOp`, nhánh phải đợi câu trả lời từ chính nó gọi ra, thì gọi
là gì?

Bài sau gọi tên chính xác cho cả hai, và chỉ ra đúng lúc nào máy "trả
lời ngay", lúc nào máy "phải đợi con".
::::

::::checkpoint{mastery=0.8}
::::
