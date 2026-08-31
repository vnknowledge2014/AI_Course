---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.nut-la-tra-ve-ngay-nut-trong-phai-hoi-con
title: "Nút lá trả về ngay, nút trong phải hỏi con trước"
summary: "ast.Constant là trường hợp cơ sở (R1.T1.3) của bộ đánh giá: trả lời ngay, không gọi lại chính mình. ast.BinOp là nhánh đệ quy: phải hỏi xong cả hai con rồi mới gộp. Một nhật ký ĐẨY/GỘP (đúng lối T3.3 bài 1) phơi ra đúng thứ tự đó trên cây thật."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ngu.leaf-vs-internal]
requires: [ngu.tree-walk-eval]
concepts: [ngu.leaf-vs-internal]
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
Hai loại nhánh trong hàm bạn vừa viết đã có tên rồi — chỉ là chưa ai gọi
đúng tên chúng trên chính cây này.
::::

::::explain{#co-so-va-doi-con}
R1.T1.3 (bài "Trường hợp dừng lại") đặt tên cho nhánh KHÔNG gọi lại
chính mình: **trường hợp cơ sở** — nhánh trả lời thẳng, không nhờ ai.
Nhánh còn lại — nhánh PHẢI gọi lại chính mình trước khi có gì để trả
lời — gọi là **nhánh đệ quy**.

Áp đúng cặp tên đó lên hàm `danh_gia` bài trước:

- `ast.Constant` là **trường hợp cơ sở**. Nó là một NÚT LÁ — không có
  nhánh con nào cả — nên trả lời ngay bằng chính giá trị nó giữ, không
  gọi `danh_gia` thêm lần nào.
- `ast.BinOp` là **nhánh đệ quy**. Nó là một NÚT TRONG — có hai nhánh
  con (`node.left`, `node.right`) — nên PHẢI hỏi xong CẢ HAI nhánh con
  (gọi lại `danh_gia` hai lần) rồi mới có gì để gộp và trả lời.

Không có đường tắt nào ở đây: một nút trong không thể trả lời trước khi
biết câu trả lời của cả hai con nó — đúng lý do bài trước phải gọi
`danh_gia(node.left)` XONG rồi mới tới `danh_gia(node.right)`, không
làm ngược hay làm cùng lúc được.

T3.3 bài "Đệ quy đã học — giờ nối lại với ngăn xếp" từng dựng một
**nhật ký** ghi lại đúng lúc nào một lời gọi ĐẨY thêm phiếu, lúc nào nó
LẤY phiếu ra. Bài này dựng một nhật ký tương tự cho `danh_gia` — không
ghi ĐẨY/LẤY nữa, mà ghi CƠ SỞ (nút lá trả lời ngay) và GỘP (nút trong
vừa hỏi xong cả hai con, sắp trả lời) — để nhìn thấy tận mắt thứ tự
"con trước, cha sau" trên chính cây `"2 + 3 * 4"`.
::::

::::example{#nhat-ky-co-so-gop}
Hàm `danh_gia` bài trước, thêm hai dòng ghi nhật ký — một ngay khi một
nút trả lời NGAY (trường hợp cơ sở), một ngay SAU KHI một nút trong vừa
hỏi xong cả hai con (trước khi trả lời):

```python title=readonly
import ast

nhat_ky = []

def danh_gia(node):
    if isinstance(node, ast.Constant):
        nhat_ky.append(f"CƠ SỞ: {node.value}")
        return node.value
    if isinstance(node, ast.BinOp):
        trai = danh_gia(node.left)
        phai = danh_gia(node.right)
        if isinstance(node.op, ast.Add):
            ket_qua = trai + phai
        elif isinstance(node.op, ast.Mult):
            ket_qua = trai * phai
        nhat_ky.append(f"GỘP: {trai} và {phai} -> {ket_qua}")
        return ket_qua

cay = ast.parse("2 + 3 * 4", mode="eval")
print(danh_gia(cay.body))
print(nhat_ky)
```

```text title=readonly
14
['CƠ SỞ: 2', 'CƠ SỞ: 3', 'CƠ SỞ: 4', 'GỘP: 3 và 4 -> 12', 'GỘP: 2 và 12 -> 14']
```

Ba dòng CƠ SỞ đứng TRƯỚC cả hai dòng GỘP — không phải tình cờ. Nút `2`
(lá) trả lời ngay, được ghi trước cả cụm `3 * 4`. Bên trong cụm đó,
`3` và `4` (cả hai đều là lá) trả lời NGAY trước khi phép nhân của
chúng được GỘP thành `12`. Và dòng GỘP cuối cùng — `2 và 12 -> 14`, ở
gốc — luôn là dòng SAU CÙNG, vì gốc phải đợi hết mọi nhánh con của toàn
bộ cây.
::::

::::predict{#dong-nao-dau-tien commitOnce}
Vẫn cây của `"2 + 3 * 4"`, vẫn nhật ký CƠ SỞ/GỘP như ví dụ. **Trước khi
chạy**, bạn đoán: dòng nào xuất hiện ĐẦU TIÊN trong `nhat_ky`?

:::opt{correct}
`'CƠ SỞ: 2'` — `2` là nhánh trái ngoài cùng của gốc, hỏi tới trước
tiên, và một `Constant` luôn trả lời ngay không cần hỏi ai
:::

:::opt
`'CƠ SỞ: 3'` — vì `3` cũng là một nút lá, trả lời ngay giống `2`
::why
Gần đúng ở việc `3` đúng là một nút lá, cũng ghi dòng CƠ SỞ — quan sát
đó không sai.

Chỗ lệch: `3` nằm BÊN TRONG nhánh phải của gốc (cụm `3 * 4`). Đệ quy
luôn hỏi XONG hoàn toàn nhánh trái của gốc (ở đây chỉ là một lá, `2`)
trước khi quay sang hỏi nhánh phải — nên `2` luôn ghi nhật ký trước `3`,
dù cả hai đều là lá.
::
:::

:::opt
`'GỘP: 3 và 4 -> 12'` — vì đây là dòng GỘP đầu tiên xuất hiện, và phép
nhân tính trước phép cộng
::why
Gần đúng ở việc `3 * 4` đúng là phép GỘP diễn ra trước phép cộng ở gốc
— đúng độ ưu tiên toán tử, và đúng là dòng GỘP đầu tiên trong TOÀN BỘ
các dòng GỘP.

Chỗ lệch: câu hỏi hỏi dòng ĐẦU TIÊN trong CẢ nhật ký, không chỉ trong
các dòng GỘP. Ba dòng CƠ SỞ (`2`, `3`, `4`) đều ghi TRƯỚC dòng GỘP này —
một nút trong không thể ghi GỘP trước khi cả hai con của nó (ở đây là
hai lá `3` và `4`) đã trả lời xong.
::
:::

:::opt
`'GỘP: 2 và 12 -> 14'` — vì đó là kết quả cuối cùng, điều máy đang tính
::why
Gần đúng ở việc đây đúng là dòng GỘP quan trọng nhất — kết quả cuối
cùng của cả phép tính.

Chỗ lệch: chính vì nó là kết quả CUỐI CÙNG nên nó luôn là dòng ghi SAU
CÙNG, không phải đầu tiên. Gốc cây phải đợi hết cả bốn dòng khác (ba
CƠ SỞ, một GỘP của nhánh phải) xong trước khi có gì để ghi cho chính
nó.
::
:::
::::

::::code{#dien-nhat-ky}
Byte muốn nhìn rõ thứ tự CƠ SỞ/GỘP trên chính cây `"2 + 3 * 4"`. Khung
hàm đã có sẵn phần tính (giống hệt bài trước) — việc của bạn là điền
đúng HAI dòng ghi nhật ký: một cho trường hợp cơ sở (nút lá, trả lời
ngay), một cho nhánh đệ quy (nút trong, CHỈ SAU KHI đã hỏi xong cả hai
con).

```python title=starter
import ast

nhat_ky = []

def danh_gia(node):
    if isinstance(node, ast.Constant):
        ___                            # trường hợp cơ sở: ghi nhật ký rồi trả lời NGAY
        return node.value
    if isinstance(node, ast.BinOp):
        trai = danh_gia(node.left)
        phai = danh_gia(node.right)
        if isinstance(node.op, ast.Add):
            ket_qua = trai + phai
        elif isinstance(node.op, ast.Mult):
            ket_qua = trai * phai
        ___                            # nhánh đệ quy: chỉ ghi nhật ký SAU KHI đã hỏi xong cả hai con
        return ket_qua

cay = ast.parse("2 + 3 * 4", mode="eval")
ket_qua_cuoi = danh_gia(cay.body)
print(ket_qua_cuoi)
print(nhat_ky)
```

```python title=solution
import ast

nhat_ky = []

def danh_gia(node):
    if isinstance(node, ast.Constant):
        nhat_ky.append(f"CƠ SỞ: {node.value}")
        return node.value
    if isinstance(node, ast.BinOp):
        trai = danh_gia(node.left)
        phai = danh_gia(node.right)
        if isinstance(node.op, ast.Add):
            ket_qua = trai + phai
        elif isinstance(node.op, ast.Mult):
            ket_qua = trai * phai
        nhat_ky.append(f"GỘP: {trai} và {phai} -> {ket_qua}")
        return ket_qua

cay = ast.parse("2 + 3 * 4", mode="eval")
ket_qua_cuoi = danh_gia(cay.body)
print(ket_qua_cuoi)
print(nhat_ky)
```

```python title=test
assert ket_qua_cuoi == 14, f"2 + 3 * 4 = 14 — đang ra {ket_qua_cuoi}"
assert len(nhat_ky) == 5, f"ba nút lá (2, 3, 4) ghi CƠ SỞ, hai nút trong (phép nhân, phép cộng) ghi GỘP — phải đúng 5 dòng, đang có {len(nhat_ky)}"
assert nhat_ky == [
    "CƠ SỞ: 2", "CƠ SỞ: 3", "CƠ SỞ: 4",
    "GỘP: 3 và 4 -> 12", "GỘP: 2 và 12 -> 14",
], f"thứ tự phải là ba CƠ SỞ trước (2, 3, 4), rồi hai GỘP (nhánh phải trước, gốc sau cùng) — đang ra {nhat_ky}"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống chỉ làm MỘT việc — ghi một dòng vào nhat_ky bằng .append(...), đúng công cụ nhật ký ĐẨY/LẤY của T3.3 bài 1. Chỗ trống đầu nằm TRONG nhánh Constant, chỗ trống sau nằm SAU KHI ket_qua đã tính xong trong nhánh BinOp.
- kind: strategy
  body: 'Trường hợp cơ sở ghi ngay giá trị của chính nút: nhat_ky.append(f"CƠ SỞ: {node.value}"). Nhánh đệ quy chỉ ghi SAU KHI trai, phai và ket_qua đã có đủ — không ghi trước, vì lúc đó chưa biết ket_qua là gì: nhat_ky.append(f"GỘP: {trai} và {phai} -> {ket_qua}").'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là nhat_ky.append(f"CƠ SỞ: {node.value}") và nhat_ky.append(f"GỘP: {trai} và {phai} -> {ket_qua}").'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ ghi vào nhat_ky bằng .append(...) — điền một câu không làm gì (như True, 1, 0) không tạo dòng nhật ký nào, và nhat_ky sẽ rỗng
  requireAst:
  - kind: uses-call, target: append, min: 2
  # min: 2 — đếm thật trên solution: nhat_ky.append(...) xuất hiện đúng 2 lần
  # trong MÃ NGUỒN, đúng bằng hai chỗ trống — không có .append nào khác có
  # sẵn trong khung. ĐÃ THỬ THẬT bằng cả ba cách True/1/0 cho cả hai chỗ
  # trống: cả ba đều còn 0 lần .append trong mã nguồn — dưới 2, luật chặn
  # được. Cả ba đều DỪNG AN TOÀN (ket_qua_cuoi vẫn đúng 14, vì phần TÍNH
  # không phụ thuộc dòng ghi nhật ký), nhưng nhat_ky rỗng ([]) thay vì đủ
  # năm dòng — assert độ dài và assert danh sách bắt được độc lập.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^14\\n\\['CƠ SỞ: 2', 'CƠ SỞ: 3', 'CƠ SỞ: 4', 'GỘP: 3 và 4 -> 12', 'GỘP: 2 và 12 -> 14'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba CƠ SỞ trước, hai GỘP sau — đúng thứ tự "con trả lời trước, cha gộp
sau" không hề lệch một bước nào, nhìn thấy tận mắt trong `nhat_ky`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bộ đánh giá hiện tại chỉ biết đúng hai phép: `Add` và `Mult`. Nếu đưa
nó một cây có phép TRỪ — một `ast.Sub`, phép nó CHƯA từng được dạy tới
— hàm sẽ làm gì?

Nó có DỪNG LẠI, báo cho bạn biết nó không hiểu phép này — hay lặng lẽ
chạy hết, trả về một thứ gì đó, mà bạn phải tự phát hiện ra là sai?

Bài sau trả lời — và câu trả lời không dễ chịu chút nào.
::::

::::checkpoint{mastery=0.8}
::::
