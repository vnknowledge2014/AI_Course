---
id: onboarding.ra-lenh-cho-byte.khi-doi-kieu-that-bai
title: Khi đổi kiểu không thành
summary: Đưa đúng thứ mà int() nhận, nhưng nội dung không đọc ra số. Máy dừng và gọi tên lỗi đó là ValueError.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [core.value-error]
requires: [core.int-cast, err.type-error]
concepts: [core.kieu-gia-tri, core.loi]
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
Bài trước bạn đổi chữ thành số. Bài này: khi thứ bạn đưa không đổi được.
::::

::::explain{#dua-chau-cong-tien}
Cuối ngày, bà chủ quán phở đưa cho đứa cháu một xấp giấy — mỗi tờ là tiền một
bàn — và bảo: *"Cộng hộ bà."*

Đứa cháu cộng được những tờ ghi `45000`, `90000`, `135000`. Đến tờ ghi **"bốn
lăm nghìn"** thì nó dừng lại và hỏi bà.

Để ý cháu dừng vì cái gì. Không phải vì bà đưa nhầm đồ — bà vẫn đưa một tờ
giấy, đúng thứ cháu nhận. Nó dừng vì **nội dung** tờ giấy đó không cộng được.

Bài trước bạn đã gặp `int()` — công cụ đổi một câu chữ thành số thật. `int()`
làm việc y như đứa cháu kia: nó nhận chuỗi, và nó chỉ đọc được chuỗi gồm toàn
chữ số.

Đưa cho nó `"hai mươi lăm"`, nó không tự suy ra 25. Máy **không đoán** — nó chỉ
làm đúng điều có quy ước sẵn, và không có quy ước nào bảo nó đọc hiểu tiếng
Việt.

Khi ấy máy dừng chương trình và nói ra một cái tên mới: **`ValueError`**.
*Value* nghĩa là **giá trị**, tức là nội dung. Cái tên đó nói thẳng chỗ sai:
không phải bạn đưa nhầm loại đồ, mà nội dung món đồ ấy không dùng được.
::::

::::example{#xem-tan-mat}
Bạn hỏi tuổi khách, và khách gõ vào bằng chữ:

```python title=readonly
tuoi_go_vao = "hai mươi lăm"
tuoi = int(tuoi_go_vao)
print(tuoi)
```

Máy in ra:

```text title=readonly
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    tuoi = int(tuoi_go_vao)
ValueError: invalid literal for int() with base 10: 'hai mươi lăm'
```

Đọc **từ dòng cuối lên**, đúng như bài đọc thông báo lỗi đã dạy:

- `ValueError` — tên loại lỗi. Đây là từ đầu tiên cần nhìn.
- `invalid literal for int() with base 10` — *"thứ này không phải một con số
  viết theo hệ mười mà `int()` đọc được"*. `base 10` là hệ đếm mười chữ số
  `0`–`9` bạn dùng hằng ngày.
- `'hai mươi lăm'` — máy in **nguyên văn** thứ nó không đọc nổi. Đây là chi tiết
  đắt nhất của cả thông báo: nó chỉ thẳng vào thủ phạm, bạn không phải đi đoán.
- Dòng trên nữa: `line 2`. Lỗi nằm ở dòng 2.

Và `print(tuoi)` ở dòng 3 **không chạy**. Máy dừng ngay tại dòng 2; những dòng
sau đó nằm im.

Còn đây là ranh giới giữa hai cái tên lỗi bạn đã có:

| bạn viết | máy nói | máy đang chê cái gì |
|---|---|---|
| `"45000" + 5000` | `TypeError` | **loại** — một bên là chữ, một bên là số, không có quy ước nào ghép hai loại đó |
| `int("bốn lăm")` | `ValueError` | **nội dung** — đúng loại rồi (một chuỗi), nhưng bên trong không phải chữ số |

Và đây là những chuỗi `int()` đọc được hay không đọc được:

| chuỗi | `int()` đọc được? |
|---|---|
| `"25"` | được — cho ra số 25 |
| `"  25  "` | được — máy bỏ qua khoảng trắng ở hai đầu |
| `"25 tuổi"` | không — có chữ cái lẫn vào |
| `"hai mươi lăm"` | không — toàn chữ cái |
| `"25.5"` | không — dấu chấm không phải chữ số, và `int` chỉ nhận số nguyên |
| `""` | không — chuỗi rỗng, chẳng có chữ số nào |
::::

::::predict{#khach-go-chu commitOnce}
Khách được hỏi *"Mấy tô?"* và gõ vào chữ **ba**. **Trước khi bấm chạy**, bạn
đoán máy làm gì?

```python title=readonly
khach_go = "ba"
so_to = int(khach_go)
print(so_to)
```

:::opt{correct}
Máy dừng lại ở dòng 2 và báo ValueError
:::

:::opt
In ra 3
::why
Gần đúng ở chỗ bạn đọc `"ba"` ra con số 3 — và bạn đọc đúng, vì bạn biết tiếng
Việt.

Chỗ lệch là bạn đang cho máy mượn cái vốn tiếng Việt ấy. `int()` không tra từ
điển; nó chỉ có một bảng mười ký tự `0`–`9`. Chữ `b`, `a` không nằm trong bảng
đó, nên nó dừng.

Đưa cho `int()` chuỗi `"3"` thì mới ra số 3.
::
:::

:::opt
In ra chữ ba, y như lúc gõ vào
::why
Gần đúng ở chỗ bạn nhớ luật cũ: thứ nằm trong dấu nháy được đọc nguyên văn. Với
riêng dòng `khach_go = "ba"` thì điều đó vẫn đúng nguyên.

Chỗ lệch nằm ở dòng sau. Bạn đã **giao việc** cho `int()`: đổi chuỗi này thành
số. `int()` không có chế độ "đổi không được thì thôi, trả lại nguyên si" — làm
vậy thì dòng sau đó bạn tưởng đang cầm một con số mà thật ra đang cầm một câu
chữ, và cái nhầm ấy sẽ nổ ra ở chỗ khác, khó tìm hơn nhiều.

Máy chọn cách ngược lại: dừng ngay, nói ngay.
::
:::

:::opt
Máy báo TypeError
::why
Gần đúng ở chỗ bạn nhận ra chuyện này thuộc về **kiểu** — chữ với số. Đó chính
là trục đúng.

Chỗ lệch là ở chỗ ai đang chê ai. `TypeError` xuất hiện khi bạn đưa nhầm **loại
đồ** cho một việc, như đem cộng một câu chữ với một con số. Còn ở đây bạn đưa
`int()` đúng thứ nó nhận — một chuỗi. Nó nhận, nó mở ra xem, rồi mới thấy nội
dung bên trong không xài được.

Chê loại thì `TypeError`. Chê nội dung thì `ValueError`.
::
:::
::::

::::explain{#hai-ten-loi-hai-cho-sua}
Vì sao phải nhớ hai cái tên thay vì gộp chung thành "lỗi"?

Vì mỗi tên chỉ bạn tới một chỗ sửa khác nhau:

- Gặp `TypeError`, hãy nhìn **hai bên** của phép toán. Có phải bạn đang ghép hai
  thứ khác loại?
- Gặp `ValueError`, hãy nhìn **nội dung** đang bị đem đi đổi. Máy đã in nguyên
  văn nội dung đó ra ở cuối dòng lỗi rồi.

`ValueError` sẽ là người bạn thường gặp nhất từ đây trở đi, vì nội dung ấy
thường do người dùng gõ vào. Bạn viết ô hỏi tuổi, bạn mong người ta gõ `25`.
Có người gõ `hai lăm`, có người gõ `25 tuổi`, có người bấm nhầm rồi gõ chuỗi
rỗng. Máy không trách ai cả — nó chỉ dừng lại và nói rõ nó vướng ở đâu.
::::

::::code{#sua-so-chi-tieu}
Trong sổ chi tiêu, tiền một tô phở được ghi thành chuỗi. Bà chủ muốn cộng thêm
5000 tiền quẩy.

Đoạn dưới đang dừng ở giữa chừng vì `ValueError`. Hãy sửa **nội dung** chuỗi ở
dòng đầu để `int()` đọc được, rồi máy in ra tổng tiền.

```python title=starter
tien_pho = "45000đ"
tien_quay = 5000
print(int(tien_pho) + tien_quay)
```

```python title=solution
tien_pho = "45000"
tien_quay = 5000
print(int(tien_pho) + tien_quay)
```

```python title=test
# Chấm bằng OUTPUT: sửa xong thì máy phải in ra tổng tiền 50000.
pass
```

:::hints
- kind: attention
  body: Cứ bấm chạy trước đã. Dòng cuối của thông báo lỗi in ra nguyên văn chuỗi mà `int()` không đọc nổi — nhìn kỹ chuỗi đó, nó có gì không phải chữ số?
- kind: strategy
  body: "`int()` chỉ đọc được chuỗi gồm toàn chữ số. Một ký tự lạ nằm lẫn trong chuỗi là đủ để nó dừng, và đơn vị tiền không phải chữ số. Bạn chỉ sửa phần nằm giữa hai dấu nháy, giữ nguyên hai dòng dưới."
- kind: one-line
  body: "Sửa dòng đầu thành `tien_pho = \"45000\"` — bỏ chữ `đ` ra khỏi dấu nháy."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: 50000
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bạn vừa sửa lỗi bằng cách đọc đúng một dòng chữ. Máy đã chỉ sẵn chỗ cho bạn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn có hai cái tên trong tay: `ten_khach` đang giữ `"Lan"`, và `tuoi_khach`
đang giữ số `25` — số thật, đã đổi xong.

Bạn muốn máy nói đúng một câu: `Chào Lan, bạn 25 tuổi`.

Công cụ duy nhất bạn có lúc này là dấu `+` để nối chuỗi. Thử viết ra xem:
`print("Chào " + ten_khach + ", bạn " + tuoi_khach + " tuổi")`. Bạn phải cắt câu
chào thành bốn mẩu, phải nhớ chừa dấu cách ở đúng chỗ, phải đếm dấu nháy — và
tới `+ tuoi_khach` thì chương trình gãy vì `TypeError`, bởi 25 là số chứ không
phải chữ.

Ngần ấy việc, cho một câu chào.

Có cách nào viết thẳng cả câu ra như bạn muốn đọc thấy, rồi chỉ **chừa hai chỗ
trống** cho hai cái tên kia tự điền vào không?

Đừng trả lời vội. Bài sau là đúng một chữ cái, đặt ngay trước dấu nháy mở.
::::

::::checkpoint{mastery=0.8}
::::
