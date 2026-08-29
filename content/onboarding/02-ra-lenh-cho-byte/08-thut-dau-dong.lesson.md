---
id: onboarding.ra-lenh-cho-byte.thut-dau-dong
title: Thụt đầu dòng, máy biết đâu là bên trong
summary: Khoảng trắng đầu dòng là cách máy biết dòng nào thuộc về if và dòng nào không.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ctrl.block-indent]
requires: [ctrl.if]
concepts: [ctrl.re-nhanh, core.khoi-lenh]
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
Bốn dấu cách đầu dòng không phải cho đẹp. Nó nói cho máy biết ai thuộc về ai.
::::

::::explain{#danh-sach-viec}
Bà chủ quán viết cho người phụ bếp một tờ dặn dò:

> Mở quán
>
> Khi khách gọi phở tái:
>
> &nbsp;&nbsp;&nbsp;&nbsp;— chần thịt tái
>
> &nbsp;&nbsp;&nbsp;&nbsp;— chan nước dùng
>
> Lau bàn
>
> Đóng quán

Bạn đọc tờ này và hiểu ngay: *chần thịt* và *chan nước dùng* chỉ làm khi có
khách gọi phở tái. Còn *lau bàn* thì làm dù có khách hay không.

Bạn hiểu được nhờ đúng một thứ: **hai dòng kia thụt vào**. Không ai phải viết
thêm chữ nào để giải thích.

Python làm y hệt. Những dòng thụt vào cùng một mức, nằm ngay dưới dòng `if`, họp
thành một nhóm gọi là **khối lệnh** — cả khối cùng chạy khi câu trả lời là
`True`, cả khối cùng bị bỏ qua khi câu trả lời là `False`.

Khối không phải một dòng. Nó có thể là một dòng, ba dòng, hay ba mươi dòng.
Thứ quyết định ranh giới là khoảng trắng đầu dòng, không phải số dòng.
::::

::::example{#cung-code-khac-le}
Hai đoạn dưới đây khác nhau đúng bốn dấu cách. Kết quả thì khác hẳn.

```python title=readonly
gio = 20
if gio < 8:
    print("Giảm giá buổi sáng")
    print("Bớt 5000 đồng")
```

Lúc 20 giờ, `20 < 8` là `False`. Cả **hai** dòng thụt vào đều bị bỏ qua. Màn
hình trắng trơn, không in gì cả.

```python title=readonly
gio = 20
if gio < 8:
    print("Giảm giá buổi sáng")
print("Bớt 5000 đồng")
```

Dòng cuối bây giờ sát lề trái, nên nó **không nằm trong khối**. Nó là một dòng
bình thường của chương trình, chạy bất kể giờ nào. Màn hình in ra:

```text title=readonly
Bớt 5000 đồng
```

Cùng một chữ, cùng một điều kiện, chỉ khác chỗ đứng. Trong Python, chỗ đứng của
một dòng **là một phần ý nghĩa của nó**.

> Quy ước: thụt vào bằng **bốn dấu cách**. Đừng trộn dấu cách với phím Tab
> trong cùng một tệp — nhìn thì giống nhau, nhưng với máy đó là hai thứ khác
> nhau, và nó sẽ từ chối chạy.
::::

::::predict{#khoi-hai-dong commitOnce}
Đoạn dưới có khối lệnh gồm **hai** dòng. Khách mới đi ăn 3 lần.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python title=readonly
so_lan_den_quan = 3
if so_lan_den_quan >= 10:
    print("Tặng bạn một tô phở")
    print("Số lần đã trừ: 10")
print("Hẹn gặp lại")
```

:::opt{correct}
Chỉ một dòng: Hẹn gặp lại
:::

:::opt
Số lần đã trừ: 10, rồi Hẹn gặp lại
::why
Gần đúng ở chỗ bạn nắm chắc luật của bài trước: điều kiện sai thì dòng việc bị
bỏ qua. Và trong **mọi** ví dụ bạn từng thấy, dòng việc đó đúng là một dòng duy
nhất — nên nghĩ `if` quản một dòng là suy luận rất tự nhiên.

Chỗ lệch: `if` không quản theo số dòng, nó quản theo **mức thụt**. Ở đây có hai
dòng cùng thụt vào bốn dấu cách, nên cả hai cùng nằm trong khối, và cả hai cùng
bị bỏ qua.
::
:::

:::opt
Cả ba dòng
::why
Gần đúng ở chỗ bạn đọc chương trình như đọc một trang giấy: chữ nào có trên
trang thì rồi cũng đọc tới. Với mọi bài trước bài `if`, điều đó luôn đúng.

Nhưng khoảng trắng đầu dòng ở đây không phải để trình bày cho dễ nhìn. Nó là
**dấu hiệu ngữ pháp**, y như dấu nháy hay dấu ngoặc. Hai dòng thụt vào chỉ chạy
khi `so_lan_den_quan >= 10` cho ra `True`. Mà 3 thì chưa tới 10.
::
:::

:::opt
Không in ra dòng nào cả
::why
Gần đúng ở chỗ bạn nhận ra `3 >= 10` là `False` nên khối lệnh bị bỏ qua — phần
đó chính xác hoàn toàn.

Chỗ lệch nằm ở dòng `print("Hẹn gặp lại")`. Nó viết sát lề trái, tức là đứng
**ngoài** khối. Máy bỏ qua khối xong thì đi tiếp xuống dòng dưới như bình
thường, và dòng dưới vẫn chạy.
::
:::
::::

::::explain{#loi-thut-le}
Vì thụt lề mang ý nghĩa, viết sai nó là một lỗi ngữ pháp thật sự.

```python title=readonly
gio = 6
if gio < 8:
print("Giảm giá buổi sáng")
```

Máy dừng lại trước khi chạy dòng nào, và nói:

```text title=readonly
IndentationError: expected an indented block after 'if' statement on line 2
```

Dịch ra: *"tôi đang chờ một khối thụt vào sau câu `if` ở dòng 2, mà không thấy."*

Bạn đã có công cụ để đọc dòng này rồi: đọc từ dòng cuối lên, dòng cuối nói tên
lỗi, và ở đây nó còn chỉ thẳng số dòng. `IndentationError` là bà con gần với
`SyntaxError` — cả hai đều bị phát hiện **trước khi** chương trình chạy, vì máy
chưa đọc nổi câu bạn viết thì lấy gì mà chạy.
::::

::::code{#chao-moi-khach}
Đoạn dưới đang chào sai. Lúc 20 giờ, khách vẫn phải được chào "Chúc ngon miệng"
— câu chào đó dành cho mọi khách, không riêng khách buổi sáng.

Hãy sửa **chỗ đứng** của một dòng để câu chào luôn hiện ra.

```python title=starter
gio = 20
if gio < 8:
    print("Giảm giá buổi sáng")
    print("Chúc ngon miệng")
```

```python title=solution
gio = 20
if gio < 8:
    print("Giảm giá buổi sáng")
print("Chúc ngon miệng")
```

```python title=test
# Chấm bằng OUTPUT: lúc 20 giờ chỉ được hiện đúng câu chào chung.
pass
```

:::hints
- kind: attention
  body: Không cần thêm hay xoá chữ nào. Chỉ có một dòng đang đứng nhầm chỗ — nhìn khoảng trắng đầu mỗi dòng.
- kind: strategy
  body: Dòng nào thụt vào là dòng nằm trong khối của `if`, và chỉ chạy khi điều kiện đúng. Câu chào chung thì không được nằm trong khối đó.
- kind: one-line
  body: "Xoá bốn dấu cách đầu dòng `print(\"Chúc ngon miệng\")` để nó sát lề trái."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Chúc ngon miệng\s*$
:::
::::

::::byte{trigger=success mood=happy pose=point-editor}
Bạn vừa đổi nghĩa cả chương trình mà không gõ thêm chữ nào. Khoảng trắng biết nói.
::::

::::reflect{#nghi-lai}
`if` lo phần "câu trả lời là `True` thì làm gì".

Nhưng nhìn lại quán phở: lúc 20 giờ, máy im lặng hoàn toàn. Khách chẳng nhận
được lời nào về chuyện giảm giá. Ngoài đời thì người bán sẽ nói: *"Giờ này hết
khuyến mãi rồi, bác ạ."*

Vậy khi câu trả lời là `False`, chẳng lẽ máy không làm gì được sao? Có cách nào
viết phần "còn không thì..." không?

Đừng trả lời vội. Bài sau là đúng một từ để làm việc đó.
::::

::::checkpoint{mastery=0.8}
::::
