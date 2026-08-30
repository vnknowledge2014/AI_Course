---
id: onboarding.ra-lenh-cho-byte.lam-lai-nhieu-lan
title: Bảo máy làm lại
summary: Viết phần việc một lần, nói cho máy biết bao nhiêu lượt, rồi để nó đếm.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ctrl.for-range]
requires: [ctrl.block-indent, core.output]
concepts: [ctrl.lap]
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
Việc phải làm mười lượt, bạn vẫn viết một lần thôi. Phần đếm để mình lo.
::::

::::explain{#muoi-dong-giong-het}
Bài trước kết bằng một câu hỏi: in bảng giá mười cỡ tô mà gõ mười dòng `print`
thì sao?

Thì nó ra thế này:

```python title=readonly
print("Mời xem bảng giá")
print("Mời xem bảng giá")
print("Mời xem bảng giá")
print("Mời xem bảng giá")
print("Mời xem bảng giá")
print("Mời xem bảng giá")
```

Còn bốn dòng nữa mới đủ mười. Ba chuyện phiền cùng lúc:

- Mỏi tay, và đếm nhầm lúc nào không biết.
- Muốn đổi câu chữ thì phải sửa mười chỗ, sót một chỗ là sai.
- Đọc lại đoạn code, mắt bạn không thấy ngay là **mười** — phải ngồi đếm.

Ở quán phở, bà chủ không viết mười dòng dặn dò như vậy. Tờ giấy dán bếp của bà
có đúng một dòng:

> Làm mười lượt: xếp một cái bát ra bàn.

Một câu, hai phần dính vào nhau: **số lượt** và **việc cần làm**. Người phụ bếp
đọc một lần, làm mười lần.

Python có một câu lệnh viết đúng kiểu đó. Nó tên là `for`.
::::

::::example{#vong-lap-dau-tien}
Đây là tờ giấy dán bếp viết bằng Python:

```python title=readonly
for lan in range(3):
    print("Xếp một cái bát ra bàn")
```

Máy in ra:

```text title=readonly
Xếp một cái bát ra bàn
Xếp một cái bát ra bàn
Xếp một cái bát ra bàn
```

Một câu `print` duy nhất trong code, ba dòng trên màn hình.

Đọc dòng đầu từ trái sang phải:

- `for` — từ khoá, báo cho máy biết: *phần việc phía dưới sẽ được làm lại nhiều lượt*.
- `range(3)` — số lượt. Ba lượt.
- `lan` — một cái tên. Máy đặt nó ở đó cho mỗi lượt. Hôm nay ta chưa dùng tới nó lần nào; bài sau sẽ mở ra xem bên trong.
- Dấu hai chấm `:` cuối dòng — bắt buộc, y như ở `if`. Nó có nghĩa: *phần đầu hết ở đây, phần việc bắt đầu từ dòng dưới*.
- Dòng dưới **lùi vào** bốn dấu cách. Bạn đã biết từ bài thụt đầu dòng: lùi vào nghĩa là "thuộc về dòng phía trên". Đây chính là phần việc được làm lại.
::::

::::predict{#dem-may-dong commitOnce}
Đoạn dưới có hai câu `print`, một câu lùi vào và một câu sát lề trái. **Trước khi
bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python title=readonly
for lan in range(4):
    print("Phở đây!")
print("Cảm ơn quý khách")
```

:::opt{correct}
Bốn dòng "Phở đây!", rồi một dòng "Cảm ơn quý khách" — năm dòng tất cả
:::

:::opt
Bốn dòng "Phở đây!" và bốn dòng "Cảm ơn quý khách", xen kẽ nhau
::why
Gần đúng ở chỗ bạn nhận ra `range(4)` bảo máy làm bốn lượt — phần đó bạn đọc
chính xác.

Chỗ lệch nằm ở lề trái. `print("Cảm ơn quý khách")` viết sát lề, không lùi vào,
nên nó **không thuộc** phần việc của `for` — đúng cái luật bạn đã học ở bài thụt
đầu dòng, chỉ là lần này nó áp cho `for` thay vì `if`. Máy chạy trọn bốn lượt
của phần lùi vào, xong xuôi rồi mới đi tiếp xuống dòng sát lề, đúng một lần.
::
:::

:::opt
Chỉ bốn dòng "Phở đây!"
::why
Gần đúng ở phần khó nhất: bạn đếm đúng bốn lượt, và bạn nhận ra dòng "Cảm ơn
quý khách" nằm ngoài vòng lặp. Hai điều đó đều chuẩn.

Chỗ lệch là bạn dừng sớm hơn máy một nhịp. Nằm ngoài vòng lặp không có nghĩa là
bị bỏ qua — nó chỉ có nghĩa là không được làm lại. Máy vẫn đi từ trên xuống như
mọi bài trước, nên sau bốn lượt nó chạy nốt dòng cuối, đúng một lần.
::
:::

:::opt
Một dòng "Phở đây!" rồi một dòng "Cảm ơn quý khách"
::why
Gần đúng ở chỗ bạn đọc rất sát mặt chữ: trong cả đoạn chỉ có đúng một câu
`print("Phở đây!")`, nên in ra một dòng nghe rất hợp lý.

Và cho tới bài này thì điều đó luôn đúng — một dòng lệnh nhiều nhất chạy một
lần, hoặc chạy, hoặc bị `if` nhảy qua. `for` là câu lệnh đầu tiên phá lệ theo
hướng ngược lại: `range(4)` bảo máy quay lại làm **cùng một dòng** thêm lượt
nữa, đủ bốn lượt mới thôi.
::
:::
::::

::::explain{#vong-lap-la-gi}
Cái bạn vừa viết có tên: **vòng lặp**. Mỗi lượt chạy gọi là một **vòng**.

Điều đáng giá nhất không phải là đỡ gõ. Là thế này: đổi `range(3)` thành
`range(300)` thì chương trình vẫn dài đúng hai dòng. Số lượng việc không còn
dính vào số lượng chữ bạn viết nữa.

> Bẫy hay vấp: quên dấu hai chấm cuối dòng `for`, hoặc quên lùi vào bốn dấu cách
> ở dòng dưới. Cả hai đều khiến máy dừng trước khi chạy dòng nào. Gặp lỗi loại
> này, nhìn ngay vào dòng `for` và dòng ngay dưới nó.
::::

::::code{#chan-nuoc-dung}
Quán sắp mở cửa. Bà chủ cần chan sẵn **sáu** tô nước dùng, và muốn máy nhắc đủ
sáu lượt.

Câu nhắc đã viết sẵn. Hãy điền phần nói cho máy biết số lượt.

```python title=starter
for lan in ___:
    print("Chan một tô nước dùng")
```

```python title=solution
for lan in range(6):
    print("Chan một tô nước dùng")
```

```python title=test
# Chấm bằng OUTPUT: câu nhắc phải hiện ra, và hiện ra đủ sáu lượt.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm ngay sau chữ `in`. Đó là chỗ nói cho máy biết **bao nhiêu lượt**.
- kind: strategy
  body: Nhìn lại ví dụ xếp bát ở trên. Số lượt được viết bằng `range` kèm một con số đặt trong ngoặc tròn. Ở đây con số ấy là sáu.
- kind: one-line
  body: "Viết `range(6)` vào chỗ trống, giữ nguyên dấu hai chấm ở cuối dòng."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  # Đếm ĐỦ SÁU dòng, không chỉ "có xuất hiện".
  #
  # Luật cũ là `contains`, nên `range(7)` — thậm chí `range(600)` — cũng qua.
  # Chú thích ngay trong khối test lại viết "hiện ra đủ sáu lượt", tức là bài
  # tự nhận một điều mà cách chấm của nó không hề kiểm. Mà đếm đủ số lượt CHÍNH
  # LÀ thứ bài này dạy.
  match: regex
  expect: ^(?:Chan một tô nước dùng\n){5}Chan một tô nước dùng\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai dòng code, sáu lượt việc. Đổi con số trong ngoặc là đổi luôn số lượt.
::::

::::sandbox{#cho-byte-di}
Byte đang đứng ở góc trái. Viên đá cuối hàng cách nó **sáu ô**.

Đây là sân chơi — không có đáp án đúng, không ai chấm. Đổi con số, đổi lệnh,
chạy lại bao nhiêu lần cũng được. Thử cả những thứ bạn nghĩ là sẽ hỏng: đi
bảy ô xem sao, hay bỏ hẳn vòng lặp đi.

```python title=starter
for lan in range(6):
    di_toi()
```

:::world{grid-bot}
{ "rong": 7, "cao": 1, "bat_dau": { "x": 0, "y": 0 }, "huong": 0,
  "vien": [ { "x": 6, "y": 0 } ] }
:::
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả ba đoạn code hôm nay có chung một điểm: **mọi lượt in ra y hệt nhau**. Sáu tô
nước dùng nhưng sáu dòng chữ không phân biệt được tô nào với tô nào.

Mà trong đoạn code có một thứ ta viết ra rồi bỏ đó, chưa dùng lần nào: cái tên
`lan`, đứng giữa `for` và `in`. Nó nằm đó để làm gì?

Nếu đem nó vào trong `print` — `print(lan)` — bạn nghĩ ba lượt sẽ in ra ba thứ
giống nhau, hay ba thứ khác nhau?

Bài sau mở cái tên đó ra.
::::

::::checkpoint{mastery=0.8}
::::
