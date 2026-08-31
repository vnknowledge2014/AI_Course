---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.them-bien-vao-ngon-ngu-nho
title: "Thêm biến (tên) vào ngôn ngữ nhỏ"
summary: "Bộ đánh giá tới giờ chỉ hiểu SỐ. Thêm một cái TÊN như x cần một MÔI TRƯỜNG — một dict ánh xạ tên sang giá trị — vì cái tên không giữ giá trị, nó chỉ CHỈ TỚI một chỗ giữ giá trị."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ngu.add-variables]
requires: [ngu.compare-eval-strategies, mem.name-is-reference]
concepts: [ngu.add-variables]
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
Hai mươi bài, và ngôn ngữ nhỏ của bạn mới hiểu được đúng một thứ: số.
Hôm nay nó học đọc một cái TÊN.
::::

::::explain{#bo-danh-gia-moi-hieu-so}
Bài trước chạy CÙNG một biểu thức qua hai cách đánh giá khác hẳn nhau —
cây đệ quy và máy ngăn xếp — và cả hai cùng ra một con số. Nhưng nhìn
kỹ lại: cả hai bộ đánh giá đó chỉ từng gặp hai loại nút, `Constant`
(một con số nằm sẵn trong biểu thức) và `BinOp` (một phép toán giữa
hai nhánh con). Không nút nào trong số đó là một cái TÊN.

Thử `ast.parse("x + 3", mode="eval")` thì cây vẫn dựng được — nhưng
đưa cây đó cho bộ đánh giá cũ, nó không biết phải làm gì với nút mới:
`ast.Name`. Không phải vì `x` khó tính — mà vì bộ đánh giá chưa từng
được DẠY loại nút này tồn tại.

Cái khó không nằm ở việc NHẬN RA một cái tên. Cái khó là: một cái tên
tự nó không mang giá trị nào cả. `node.id` của một `ast.Name` chỉ là
một CHUỖI — đúng ba ký tự `'x'`, không phải con số 5. Đây chính là
điều T3.1 đã dạy qua `mem.name-is-reference`: cái tên không GIỮ giá
trị, nó CHỈ TỚI một chỗ giữ giá trị. Bộ đánh giá cần một chỗ để tra
cứu — gõ tên `'x'` vào, nhận lại giá trị nó đang trỏ tới.

Chỗ tra cứu đó gọi là **môi trường** (environment): một `dict` bình
thường, ánh xạ mỗi tên (chuỗi) sang giá trị hiện tại của nó.
```python
moi_truong = {"x": 5, "y": 2}
```
Gặp một nút `ast.Name`, bộ đánh giá không tự tính ra giá trị — nó
**tra `moi_truong[node.id]`** và trả về đúng cái nằm ở đó. Thêm một
LOẠI NÚT mới vào bộ đánh giá vẫn đúng nhịp bài 13-14 đã đi: thêm một
nhánh `elif`, không viết lại từ đầu — chỉ lần này mọi lời gọi đệ quy
cần thêm một tham số nữa: chính cái môi trường đó.
::::

::::example{#tra-cuu-that}
Cây của `"x + 3 * y"`, dựng bằng đúng công cụ đã quen từ bài 7:

```python title=readonly
import ast
print(ast.dump(ast.parse("x + 3 * y", mode="eval"), indent=2))
```

```text title=readonly
Expression(
  body=BinOp(
    left=Name(id='x', ctx=Load()),
    op=Add(),
    right=BinOp(
      left=Constant(value=3),
      op=Mult(),
      right=Name(id='y', ctx=Load()))))
```

Hai nút `Name` nằm ngay trong cây — `id='x'` và `id='y'`, y hệt
`Constant(value=3)` nhưng KHÔNG có giá trị đính kèm, chỉ có một cái
tên. Bộ đánh giá bài 15 chỉ nhận một tham số (`node`) — thêm môi
trường nghĩa là thêm `moi_truong` vào MỌI lời gọi đệ quy, và thêm đúng
một nhánh `elif` cho `ast.Name`:

```python title=readonly
def danh_gia(node, moi_truong):
    if isinstance(node, ast.Constant):
        return node.value
    elif isinstance(node, ast.Name):
        return moi_truong[node.id]
    elif isinstance(node, ast.BinOp):
        trai = danh_gia(node.left, moi_truong)
        phai = danh_gia(node.right, moi_truong)
        if isinstance(node.op, ast.Add):
            return trai + phai
        elif isinstance(node.op, ast.Mult):
            return trai * phai
        else:
            raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}")

cay = ast.parse("x + 3 * y", mode="eval")
moi_truong = {"x": 5, "y": 2}
print(danh_gia(cay.body, moi_truong))
```

```text title=readonly
11
```

Đúng `5 + 3 * 2 = 11` — tính tay ra y hệt. Cái mới không nằm ở phép
cộng hay phép nhân, cả hai đã có từ bài 15. Cái mới là dòng
`moi_truong[node.id]`: mỗi lần cây có một nút `Name`, bộ đánh giá
dừng lại, tra cứu, rồi mới đi tiếp. `cay.body` vẫn đúng cách bài 11 đã
làm — bỏ lớp `Expression` bọc ngoài trước khi đưa cho `danh_gia`.
::::

::::predict{#tra-cuu-luc-nao commitOnce}
Byte viết ba dòng, đúng thứ tự này:

```python
moi_truong = {"x": 3}
cay = ast.parse("x + x", mode="eval")
moi_truong["x"] = 10
ket_qua = danh_gia(cay.body, moi_truong)
```

Cây được dựng lúc `moi_truong["x"]` còn là `3`. Nhưng `danh_gia` chỉ
chạy ở dòng CUỐI, sau khi `moi_truong["x"]` đã bị đổi thành `10`.

**Trước khi chạy**, bạn đoán `ket_qua` bằng bao nhiêu?

:::opt{correct}
`20` — `danh_gia` tra `moi_truong["x"]` vào ĐÚNG LÚC nó chạy, không
phải lúc cây được dựng, nên cả hai nút `Name` đều đọc ra `10`
:::

:::opt
`6` — cây được dựng lúc `x` còn là `3`, nên nó "nhớ" con số 3 cho cả
hai lần dùng, ra `3 + 3 = 6`
::why
Gần đúng ở việc bạn nhớ đúng THỜI ĐIỂM cây được dựng — dòng
`ast.parse` đúng là chạy trước dòng gán lại `moi_truong["x"] = 10`.

Chỗ lệch: `ast.parse()` không hề lưu con số nào cả. Nút `Name` trong
cây chỉ giữ đúng một chuỗi `'x'` (`node.id`) — không có ô nhớ nào bên
trong cây cất giữ giá trị 3. Giá trị chỉ xuất hiện khi `danh_gia` tra
`moi_truong`, và nó tra vào đúng lúc NÓ chạy — dòng cuối cùng, sau khi
`x` đã là `10`.
::
:::

:::opt
Máy dừng lại, báo lỗi — vì `moi_truong` bị đổi SAU khi cây đã dựng
xong, nên cây và môi trường không còn khớp nhau nữa
::why
Gần đúng ở sự thận trọng: đổi một thứ đang được dùng ở chỗ khác nghe
như một rủi ro hợp lý.

Chỗ lệch: cây và `moi_truong` là HAI thứ tách biệt hoàn toàn — cây chỉ
giữ mỗi cái TÊN `'x'`, không giữ tham chiếu gì tới `moi_truong`. Đổi
một entry trong `moi_truong` không đụng gì tới cây cả. `danh_gia` chỉ
biết đọc `moi_truong` khi nó thật sự chạy tới nút `Name` — không có
bước kiểm tra "khớp nhau" nào ở giữa.
::
:::

:::opt
`13` — lần đọc `x` đầu tiên ra `3` (giá trị lúc cây dựng), lần đọc thứ
hai ra `10` (giá trị lúc đánh giá), cộng lại thành `13`
::why
Gần đúng ở việc bạn nghĩ có một "lịch sử" các giá trị của `x`, mỗi lần
đọc lấy một mốc khác nhau — một cách nghĩ hợp lý nếu tên THẬT SỰ giữ
giá trị theo dòng thời gian.

Chỗ lệch: không tồn tại "giá trị lúc cây dựng" nào cả — cây không lưu
giá trị, chỉ lưu tên. Cả HAI nút `Name(id='x')` trong biểu thức `x + x`
đều được `danh_gia` tra `moi_truong["x"]` ở CÙNG một thời điểm (khi
dòng `ket_qua = danh_gia(...)` chạy), nên cả hai đều đọc ra CÙNG một
giá trị — `10`, không lệch nhau.
::
:::
::::

::::code{#them-nhanh-name}
Byte có một sổ giá: `moi_truong = {"x": 5, "y": 2, "z": 10}`. Bộ đánh
giá dưới đây đã có đủ `Constant` và `BinOp` từ bài 15 — chỉ thiếu đúng
một nhánh để hiểu `ast.Name`. Điền vào chỗ trống để nó tra đúng giá
trị của cái tên trong `moi_truong`.

```python title=starter
import ast

def danh_gia(node, moi_truong):
    if isinstance(node, ast.Constant):
        return node.value
    elif isinstance(node, ast.Name):
        return ___                        # tra ten trong moi_truong
    elif isinstance(node, ast.BinOp):
        trai = danh_gia(node.left, moi_truong)
        phai = danh_gia(node.right, moi_truong)
        if isinstance(node.op, ast.Add):
            return trai + phai
        elif isinstance(node.op, ast.Sub):
            return trai - phai
        elif isinstance(node.op, ast.Mult):
            return trai * phai
        elif isinstance(node.op, ast.Div):
            return trai / phai
        else:
            raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}")

moi_truong = {"x": 5, "y": 2, "z": 10}

cay1 = ast.parse("x + 3 * y", mode="eval")
ket_qua_1 = danh_gia(cay1.body, moi_truong)

cay2 = ast.parse("z - x", mode="eval")
ket_qua_2 = danh_gia(cay2.body, moi_truong)

print(ket_qua_1)
print(ket_qua_2)
```

```python title=solution
import ast

def danh_gia(node, moi_truong):
    if isinstance(node, ast.Constant):
        return node.value
    elif isinstance(node, ast.Name):
        return moi_truong[node.id]
    elif isinstance(node, ast.BinOp):
        trai = danh_gia(node.left, moi_truong)
        phai = danh_gia(node.right, moi_truong)
        if isinstance(node.op, ast.Add):
            return trai + phai
        elif isinstance(node.op, ast.Sub):
            return trai - phai
        elif isinstance(node.op, ast.Mult):
            return trai * phai
        elif isinstance(node.op, ast.Div):
            return trai / phai
        else:
            raise ValueError(f"chưa hỗ trợ phép toán: {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hỗ trợ loại nút: {type(node).__name__}")

moi_truong = {"x": 5, "y": 2, "z": 10}

cay1 = ast.parse("x + 3 * y", mode="eval")
ket_qua_1 = danh_gia(cay1.body, moi_truong)

cay2 = ast.parse("z - x", mode="eval")
ket_qua_2 = danh_gia(cay2.body, moi_truong)

print(ket_qua_1)
print(ket_qua_2)
```

```python title=test
assert ket_qua_1 == 11, f"x + 3*y với x=5, y=2 phải là 5 + 3*2 = 11 — đang ra {ket_qua_1}"
assert ket_qua_2 == 5, f"z - x với z=10, x=5 phải là 10 - 5 = 5 — đang ra {ket_qua_2}"
```

:::hints
- kind: attention
  body: Chỗ trống là giá trị trả về khi gặp một nút Name — không phải một con số cố định, mà phải tra trong moi_truong bằng đúng cái tên node.id đang giữ.
- kind: strategy
  body: 'node.id là một chuỗi, ví dụ "x". moi_truong là một dict ánh xạ chuỗi đó sang giá trị. Tra bằng cách lập chỉ mục — moi_truong[node.id] — giống hệt cách bạn tra một dict bình thường.'
- kind: one-line
  body: 'Chỗ trống là: moi_truong[node.id]'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ tra cứu trong moi_truong bằng node.id — không được để trống bằng một câu không làm gì (như True, 1, 0), vì như vậy mọi cái tên đều trả về cùng một giá trị cố định thay vì giá trị thật của nó
  requireAst:
  # min: 5 — đếm thật trên solution: moi_truong bị ĐỌC (uses-name, ctx Load)
  # đúng 5 lần trong mã nguồn — 1 lần ở chỗ trống (tra cứu), 2 lần trong
  # nhánh BinOp (truyền cho trai/phai), và 2 lần ở ket_qua_1/ket_qua_2 (gọi
  # danh_gia). ĐÃ THỬ THẬT bằng cả ba cách True/1/0: không vòng lặp/đệ quy
  # nào phụ thuộc chỗ trống theo kiểu gây lặp thêm (đệ quy chỉ đi theo hình
  # dạng CÂY, không theo giá trị trả về của nhánh Name), nên cả ba dừng
  # ngay — không treo, không vượt trần bước. Cả ba làm số lần đọc moi_truong
  # tụt xuống còn 4 (mất đúng lần đọc ở chỗ trống), dưới ngưỡng 5, luật này
  # chặn được. Đồng thời cả ba cho ket_qua_1 = ket_qua_2 = chính giá trị
  # điền (4, 1 hoặc 0 tương ứng) thay vì 11 và 5 — assert bắt được độc lập.
  - kind: uses-name, target: moi_truong, min: 5
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^11\\n5\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`x` không còn là một chữ cái lạ nữa — nó là một cái tên, và bộ đánh giá
của bạn biết đi tra nó ở đúng chỗ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp — tự bạn thử, đừng đoán suông.

Chạy thử dòng này, với `moi_truong` chỉ có `{"x": 5}`:

```python
cay = ast.parse("gia * thue", mode="eval")
danh_gia(cay.body, {"x": 5})
```

`"thue"` không có mặt trong `moi_truong`. Máy sẽ dừng lại và báo lỗi
— nhưng đọc kỹ dòng lỗi đó. Nó có nói rõ TÊN NÀO đang bị thiếu không?
Nó có nghe giống cái lỗi quen thuộc `NameError` mà Python thật báo khi
bạn dùng một biến chưa gán không, hay là một lỗi khác hẳn, mơ hồ hơn?

Bài sau sửa đúng chỗ đó.
::::

::::checkpoint{mastery=0.8}
::::
