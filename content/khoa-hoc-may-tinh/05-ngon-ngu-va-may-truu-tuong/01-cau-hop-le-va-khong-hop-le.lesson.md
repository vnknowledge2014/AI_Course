---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.cau-hop-le-va-khong-hop-le
title: "Câu hợp lệ và câu không hợp lệ — luật chơi của một ngôn ngữ"
summary: "R1 đã cho gặp SyntaxError hàng chục lần — hôm nay gọi tên đúng cái luật bị vi phạm: một văn phạm, tập luật cố định nói câu nào hợp lệ. `ast.parse()` là công cụ đối chiếu đúng văn phạm đó, không phải máy 'khó tính'."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ngu.grammar-exists]
requires: [err.syntax-error]
concepts: [ngu.grammar-exists]
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
Track trước để lại một câu hỏi: muốn tự tay xây một trình thông dịch, cần
gì, ngoài một vòng lấy-hiểu-làm? Câu trả lời đầu tiên bắt đầu từ đây —
trước khi CHẠY được gì, nó phải biết PHÂN BIỆT câu nào mình hiểu, câu nào
thì không.
::::

::::explain{#mot-tap-luat-co-dinh}
Suốt Realm 1, bạn đã gặp `SyntaxError` không biết bao nhiêu lần — thiếu
dấu hai chấm, lệch thụt lề, thiếu dấu ngoặc đóng. Mỗi lần vậy, Python từ
chối chạy dòng mã, và bạn sửa lại cho tới khi nó chịu chạy.

Hôm nay gọi tên đúng cái luật máy đang dùng để từ chối: một **văn phạm**
(grammar) — tập luật cố định, viết ra được, nói CÂU NÀO thuộc về ngôn ngữ,
câu nào thì không. Không phải máy "khó tính" hay "kén chọn". Nó đối chiếu
dòng chữ bạn gõ với một tập luật CỤ THỂ, và luật đó áp dụng như nhau cho
mọi người, mọi lúc, không đổi giữa hôm nay và ngày mai.

Python có sẵn công cụ đối chiếu đúng văn phạm đó: `ast.parse(...)`.
Truyền vào một chuỗi, nó thử DỰNG một cấu trúc theo đúng luật ngữ pháp —
dựng được thì chuỗi hợp lệ; dựng không được, nó ném ra `SyntaxError`,
đúng loại lỗi bạn đã quen mặt. (Tham số `mode="eval"` báo cho nó biết:
đây là MỘT biểu thức đơn — như `2 + 3` — không phải cả một chương trình
nhiều dòng; đúng phạm vi track này đang xét, từ giờ tới hết cụm bài này.)
::::

::::example{#doi-chieu-hop-le-khong-hop-le}
Byte thử ba dòng chữ khác nhau, dùng đúng công cụ `ast.parse`:

```python title=readonly
import ast

def kiem_tra(bieu_thuc):
    try:
        ast.parse(bieu_thuc, mode="eval")
        print(f"{bieu_thuc!r} -> HỢP LỆ")
    except SyntaxError as loi:
        print(f"{bieu_thuc!r} -> KHÔNG HỢP LỆ: {loi.msg}")

kiem_tra("2 + 3")
kiem_tra("2 3")
kiem_tra("(2 + 3")
```

```text title=readonly
'2 + 3' -> HỢP LỆ
'2 3' -> KHÔNG HỢP LỆ: invalid syntax
'(2 + 3' -> KHÔNG HỢP LỆ: '(' was never closed
```

`'2 + 3'` dựng được — hợp lệ. `'2 3'` thì không: hai con số đứng liền
nhau, không có gì ở giữa, vi phạm đúng một luật (bài sau sẽ nói rõ luật
đó là gì). `'(2 + 3'` cũng không: dấu ngoặc mở ra mà chưa có ngoặc đóng
tương ứng — `ast.parse` thậm chí còn nói đúng chỗ sai: `'(' was never
closed`. Ba câu, ba số phận khác nhau, không phải ngẫu nhiên — mỗi câu bị
từ chối vì một lý do CỤ THỂ, đọc được.
::::

::::predict{#hai-dau-cong-lien-nhau commitOnce}
Byte gõ biểu thức `2 + + 3` — hai dấu `+` đứng sát nhau.

**Trước khi chạy thử**, bạn đoán: biểu thức này có hợp lệ với văn phạm
Python không?

:::opt{correct}
Hợp lệ — dấu `+` thứ hai đóng vai trò MỘT NGÔI (unary), đứng ngay trước
số `3`, giống hệt cách dấu trừ một ngôi hoạt động trong `-5`. Toàn bộ
biểu thức là `2 + (+3)`
:::

:::opt
Không hợp lệ — hai toán tử đứng liền nhau luôn là lỗi
::why
Gần đúng ở việc hai ký hiệu `+` đứng sát nhau NHÌN đáng ngờ — dễ đoán là
lỗi.

Chỗ lệch: dấu `+` đứng ngay TRƯỚC một số (không đứng GIỮA hai số) được
văn phạm Python cho phép đóng vai MỘT NGÔI — báo hiệu "số dương", không
phải phép cộng. `+3` tự nó đã là một biểu thức hợp lệ, nên `2 + (+3)`
cũng vậy — đã chạy thử thật: `ast.parse("2 + + 3", mode="eval")` không
ném lỗi nào.
::
:::

:::opt
Không xác định được — phải chạy thử máy mới biết chắc
::why
Gần đúng ở sự thận trọng, đúng tinh thần track này — không đoán suông.

Chỗ lệch: văn phạm là một tập LUẬT viết ra được từ trước, không phải một
hộp đen ngẫu nhiên phải đoán mò. Bài học hôm nay chính là học ĐỌC RA
được luật đó — không phải mọi lúc đều buộc phải "thử thì mới biết".
::
:::

:::opt
Không hợp lệ — dấu `+` chỉ được phép đứng GIỮA đúng hai số, không được
đứng đầu một cụm
::why
Gần đúng ở việc bạn nhớ đúng vai trò PHỔ BIẾN của dấu `+`: đứng giữa hai
số, như một toán tử HAI NGÔI.

Chỗ lệch: bên cạnh vai trò đó, dấu `+` còn một vai trò khác trong đúng
văn phạm Python — toán tử MỘT NGÔI, đứng trước đúng một số. Cả hai vai
trò đều hợp lệ, chỉ là dùng ở hai chỗ khác nhau.
::
:::
::::

::::code{#phan-loai-hop-le}
Sáu biểu thức, mỗi biểu thức là một chuỗi. Phân loại từng cái vào đúng
danh sách — `hop_le` nếu `ast.parse` dựng được, `khong_hop_le` nếu nó ném
`SyntaxError`.

```python title=starter
import ast

cac_bieu_thuc = ["2 + 3", "2 3", "2 + 3 *", "10 * 4", "(2 + 3", "5 - 2"]

hop_le = []
khong_hop_le = []

for bt in cac_bieu_thuc:
    try:
        ast.parse(bt, mode="eval")
        ___                      # HỢP LỆ: thêm bt vào hop_le
    except SyntaxError:
        ___                      # KHÔNG HỢP LỆ: thêm bt vào khong_hop_le

print(hop_le)
print(khong_hop_le)
```

```python title=solution
import ast

cac_bieu_thuc = ["2 + 3", "2 3", "2 + 3 *", "10 * 4", "(2 + 3", "5 - 2"]

hop_le = []
khong_hop_le = []

for bt in cac_bieu_thuc:
    try:
        ast.parse(bt, mode="eval")
        hop_le.append(bt)
    except SyntaxError:
        khong_hop_le.append(bt)

print(hop_le)
print(khong_hop_le)
```

```python title=test
assert hop_le == ["2 + 3", "10 * 4", "5 - 2"], f"hop_le phải đúng ba biểu thức dựng được bằng ast.parse — đang ra {hop_le}"
assert khong_hop_le == ["2 3", "2 + 3 *", "(2 + 3"], f"khong_hop_le phải đúng ba biểu thức bị SyntaxError — đang ra {khong_hop_le}"
assert len(hop_le) + len(khong_hop_le) == 6, "mỗi biểu thức trong cac_bieu_thuc phải rơi vào ĐÚNG MỘT trong hai danh sách, không thiếu không thừa"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều chỉ làm một việc — thêm bt (biến đang xét ở vòng lặp) vào đúng danh sách, bằng .append(bt). Chỗ trống đầu nằm trong nhánh try (đã dựng được), chỗ trống sau nằm trong nhánh except (bị SyntaxError).
- kind: strategy
  body: 'Nhánh try chạy tới dòng ast.parse(bt, mode="eval") mà không ném lỗi nghĩa là bt HỢP LỆ — thêm nó vào hop_le. Nhánh except chỉ chạy khi SyntaxError bị ném ra — nghĩa là bt KHÔNG HỢP LỆ — thêm nó vào khong_hop_le.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là: hop_le.append(bt) và khong_hop_le.append(bt)'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ thêm bt vào đúng danh sách bằng .append(bt) — điền một câu không làm gì (như True, 1, 0) sẽ để cả hai danh sách trống rỗng
  requireAst:
  # min: 2 — đếm thật trên solution: .append() xuất hiện đúng 2 lần trong mã
  # nguồn, một ở mỗi chỗ trống — không có .append nào khác có sẵn trong khung.
  # ĐÃ THỬ THẬT bằng cả ba cách True/1/0 ở cả hai chỗ trống: vòng for chạy
  # trên đúng 6 phần tử cố định (cac_bieu_thuc), không phụ thuộc chỗ trống,
  # nên cả ba dừng ngay, không vòng nào chạy vô hạn — và cả ba để hop_le,
  # khong_hop_le trống rỗng ([], []), sai với kết quả mong đợi, nên assert
  # cũng bắt được độc lập với luật static này.
  - kind: uses-call, target: append, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\['2 \\+ 3', '10 \\* 4', '5 - 2'\\]\\n\\['2 3', '2 \\+ 3 \\*', '\\(2 \\+ 3'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba câu hợp lệ, ba câu không — mỗi câu vì một lý do cụ thể, đọc được bằng
`ast.parse`. Không phải máy đoán mò, và giờ bạn cũng không phải đoán mò
nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`ast.parse` trả lời được câu "hợp lệ hay không" cho cả MỘT chuỗi. Nhưng
trước khi nó có thể áp bất kỳ luật văn phạm nào lên dòng chữ đó, nó phải
làm một việc còn sớm hơn: nhìn ra được các MIẾNG rời rạc bên trong dòng
chữ thô ấy trước đã — `2`, `+`, `3` là ba miếng khác nhau, không phải một
khối chữ dính liền.

Một dòng chữ thô, không có gì đánh dấu ranh giới giữa các miếng ngoài
khoảng trắng — máy tách nó ra bằng cách nào? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
