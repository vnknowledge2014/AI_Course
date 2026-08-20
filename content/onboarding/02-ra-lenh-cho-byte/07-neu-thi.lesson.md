---
id: onboarding.ra-lenh-cho-byte.neu-thi
title: Nếu... thì
summary: Một câu lệnh chỉ chạy khi câu trả lời là True. Máy bắt đầu tự quyết định.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ctrl.if]
requires: [core.boolean, ctrl.comparison]
concepts: [core.dung-sai, ctrl.re-nhanh]
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
Bài trước máy trả lời có hoặc không. Bài này nó bắt đầu làm gì đó vì câu trả lời.
::::

::::explain{#to-giay-dan-tuong}
Trong bếp quán phở có tờ giấy dán trên tường. Trên đó có một dòng:

> Nếu khách gọi thêm quẩy thì lấy đĩa nhỏ.

Người phụ bếp đọc dòng này mỗi lần có khách. Nhưng người ấy **không phải lúc nào
cũng lấy đĩa nhỏ**. Dòng chữ có hai phần dính vào nhau:

- một câu hỏi có–không: *khách có gọi thêm quẩy không?*
- một việc: *lấy đĩa nhỏ*.

Việc chỉ được làm khi câu hỏi trả lời là **có**. Khách không gọi quẩy thì dòng
đó bị đọc lướt qua, không ai lấy đĩa nào cả.

Máy tính có đúng một câu lệnh để viết loại dòng này. Nó tên là `if` — tiếng Anh
nghĩa là "nếu".
::::

::::example{#if-dau-tien}
Đây là tờ giấy dán tường viết bằng Python:

```python title=readonly
tien_trong_vi = 50000
if tien_trong_vi >= 45000:
    print("Đủ tiền, mời vào ăn phở")
print("Chào bạn")
```

Máy in ra:

```text title=readonly
Đủ tiền, mời vào ăn phở
Chào bạn
```

Đọc dòng `if` từ trái sang phải:

- `if` — từ khoá, báo cho máy biết: *phần sau đây có điều kiện*.
- `tien_trong_vi >= 45000` — chính là câu hỏi có–không của bài trước. Nó cho ra
  `True` hoặc `False`.
- Dấu hai chấm `:` ở cuối dòng — bắt buộc. Nó có nghĩa: *câu hỏi hết ở đây, phần
  việc bắt đầu từ dòng dưới*.
- Dòng dưới **lùi vào** bốn dấu cách. Đó là phần việc, chỉ chạy khi câu trả lời
  là `True`.

Vì sao lại phải lùi vào? Có một lý do rất cụ thể, và bài sau nói riêng về nó.
Bây giờ cứ lùi vào bốn dấu cách và tin rằng nó quan trọng.

Để ý dòng cuối cùng: `print("Chào bạn")` **không** lùi vào. Nó nằm ngoài tờ
giấy dán tường — chạy trong mọi trường hợp, dù đủ tiền hay không.
::::

::::predict{#chua-du-tuoi commitOnce}
Đổi con số một chút. Đoạn dưới có `tuoi` là 15. **Trước khi bấm chạy**, bạn đoán
màn hình hiện ra những dòng nào?

```python title=readonly
tuoi = 15
if tuoi >= 18:
    print("Được thuê xe máy")
print("Xin mời vào quán")
```

:::opt{correct}
Chỉ một dòng: Xin mời vào quán
:::

:::opt
Cả hai dòng: Được thuê xe máy, rồi Xin mời vào quán
::why
Gần đúng ở chỗ bạn đang giữ một thói quen đọc rất đúng cho mọi bài trước đây:
máy chạy lần lượt từ trên xuống, dòng nào cũng chạy. Cho tới bài này, điều đó
luôn đúng.

`if` là câu lệnh đầu tiên phá lệ đó. Máy vẫn đi từ trên xuống, nhưng khi tới
dòng `if` nó dừng lại hỏi trước: `15 >= 18` — sai. Câu trả lời là `False`, nên
máy **nhảy qua** cả phần việc lùi vào, không chạy dòng nào trong đó.
::
:::

:::opt
Không in ra dòng nào cả
::why
Gần đúng ở chỗ bạn nhận ra `15 >= 18` là sai, nên dòng "Được thuê xe máy" bị bỏ
qua. Phần đó bạn suy luận chính xác.

Chỗ lệch nằm ở dòng cuối. `print("Xin mời vào quán")` viết sát lề trái, không
lùi vào — nên nó **không thuộc** phần việc của `if`. Nó là một dòng bình thường,
chạy bất kể câu trả lời là gì.

Bạn vừa tự tìm ra câu hỏi của bài sau: máy dựa vào đâu để biết dòng nào thuộc
`if`, dòng nào không?
::
:::

:::opt
Máy báo lỗi vì `tuoi` là 15 mà điều kiện đòi 18
::why
Gần đúng ở chỗ bạn đang cảnh giác với lỗi — thói quen tốt sau mấy bài vừa rồi.

Nhưng điều kiện sai **không phải là lỗi**. `False` là một câu trả lời hợp lệ,
đàng hoàng, y như `True`. Máy chỉ báo lỗi khi nó không hiểu bạn viết gì. Ở đây
nó hiểu rất rõ, và nó chọn không làm.
::
:::
::::

::::explain{#hai-duong-di}
Từ bài này trở đi, một chương trình không còn là một đường thẳng nữa.

Trước đây bạn viết mười dòng thì máy chạy đúng mười dòng, lần nào cũng vậy. Bây
giờ cùng một chương trình, chạy hai lần với hai con số khác nhau, có thể in ra
hai thứ khác nhau.

Đó là lúc chương trình bắt đầu giống một cái máy có ích: nó phản ứng với dữ liệu
chứ không đọc thuộc lòng.

Một chi tiết dễ vấp: **quên dấu hai chấm** cuối dòng `if`. Máy sẽ báo
`SyntaxError` — loại lỗi bạn đã gặp, loại mà máy phát hiện trước cả khi chạy
dòng đầu tiên. Gặp nó, hãy nhìn ngay cuối dòng `if`.
::::

::::code{#dat-mien-phi}
Quán phở có khuyến mãi: khách trên 65 tuổi được tặng một chén trà.

Đoạn dưới đã có sẵn tuổi khách. Hãy viết điều kiện để dòng chữ tặng trà chỉ hiện
ra khi khách **trên 65 tuổi**.

```python title=starter
tuoi_khach = 70
if ___:
    print("Tặng bác một chén trà")
```

```python title=solution
tuoi_khach = 70
if tuoi_khach > 65:
    print("Tặng bác một chén trà")
```

```python title=test
# Chấm bằng OUTPUT: với tuổi 70, dòng chữ tặng trà phải hiện ra.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa `if` và dấu hai chấm. Chỗ đó cần một câu hỏi có–không, giống hệt loại câu hỏi bài trước.
- kind: strategy
  body: Câu hỏi là "tuổi của khách có lớn hơn 65 không". Bên trái dấu so sánh là cái tên đang giữ tuổi khách, bên phải là con số 65.
- kind: one-line
  body: "Viết `tuoi_khach > 65` vào chỗ trống, giữ nguyên dấu hai chấm ở cuối dòng."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: Tặng bác một chén trà
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Máy vừa tự quyết định lần đầu tiên. Bạn không bảo nó in, bạn bảo nó khi nào in.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Trong cả ba đoạn code hôm nay, dòng việc bên trong `if` luôn lùi vào bốn dấu
cách, còn dòng bình thường thì sát lề trái. Byte bảo bạn cứ lùi vào, chưa nói vì
sao.

Nếu bạn **không** lùi vào — viết dòng `print` sát lề trái ngay dưới `if` — bạn
nghĩ máy sẽ hiểu thế nào?

Bài sau trả lời, và câu trả lời sẽ giải thích luôn một lỗi mà ai học Python cũng
gặp trong tuần đầu.
::::

::::checkpoint{mastery=0.8}
::::
