---
id: onboarding.ra-lenh-cho-byte.duyet-tung-mon
title: Đi qua từng món
summary: Vòng lặp lấy lần lượt từng món trong danh sách, không cần nhắc tới chỉ số nữa.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ctrl.for-each]
requires: [core.list-index, ctrl.for-range, ctrl.loop-variable]
concepts: [ctrl.lap, core.danh-sach]
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
Bảng hai mươi món mà gõ hai mươi dòng thì mỏi tay. Mình chỉ bạn cách gõ một lần.
::::

::::explain{#hai-muoi-dong-gan-giong-nhau}
Bài trước bạn lấy được đúng một món bằng chỉ số. Muốn đọc hết cả bảng thì viết
thế này:

> `print(thuc_don[0])` · `print(thuc_don[1])` · `print(thuc_don[2])` · …

Ba món thì ba dòng. Hai mươi món thì hai mươi dòng, và cả hai mươi dòng giống
hệt nhau trừ đúng một con số. Phiền hơn nữa: hôm sau quán bỏ bớt một món, bạn
phải ngồi sửa lại số của mọi dòng phía sau.

Trong quán không ai làm vậy. Người phục vụ cầm tấm bảng rồi đọc **từ trên
xuống**: đọc dòng nào thì làm việc với dòng ấy, xong thì xuống dòng kế, hết bảng
thì dừng. Không ai đánh số dòng, cũng không ai đếm trước xem bảng có bao nhiêu
dòng.

Python có đúng một câu lệnh cho kiểu đọc đó, và bạn đã gặp nó rồi: `for`.
::::

::::example{#for-tren-danh-sach}
Lần trước `for` đi cùng `range`. Lần này nó đi thẳng cùng danh sách:

```python title=readonly
thuc_don = ["Phở tái", "Phở chín", "Phở nạm"]
for mon in thuc_don:
    print(f"Mời bạn dùng {mon}")
```

Màn hình hiện ra:

```text title=readonly
Mời bạn dùng Phở tái
Mời bạn dùng Phở chín
Mời bạn dùng Phở nạm
```

Đọc dòng `for` từ trái sang phải:

- `for` — báo cho máy biết: phần việc bên dưới sẽ được làm lại nhiều lượt.
- `mon` — một cái tên do **bạn** đặt, y như `lan` hay `so` ở bài lặp trước. Mỗi
  lượt, máy dán cái tên này lên một món.
- `in thuc_don` — lấy món từ dãy nào.
- Dấu hai chấm cuối dòng, và dòng dưới lùi vào bốn dấu cách — đúng luật khối
  lệnh bạn đã học ở bài thụt đầu dòng.

Máy chạy như thế này:

> Lượt 1: `mon` giữ `"Phở tái"` → in *Mời bạn dùng Phở tái*. Lượt 2: `mon` giữ
> `"Phở chín"`. Lượt 3: `mon` giữ `"Phở nạm"`. Hết món, vòng lặp dừng.
::::

::::explain{#khac-gi-voi-range}
Hai kiểu `for` khác nhau ở đúng một chỗ: **thứ đứng sau chữ `in`**.

- `for so in range(3)` — sau `in` là một máy đếm. Cái tên nhận 0, rồi 1, rồi 2.
- `for mon in thuc_don` — sau `in` là một dãy có sẵn. Cái tên nhận thẳng *nội
  dung* từng ô: "Phở tái", rồi "Phở chín", rồi "Phở nạm".

Chỉ số vẫn còn đó — máy vẫn đi từ ô 0 tới ô cuối, theo đúng thứ tự bạn viết. Chỉ
là bạn không phải nhắc tới nó nữa: bạn nói *"từng món trong thực đơn"*, thay vì
*"ô số 0, ô số 1, ô số 2"*.

Đổi lại bạn được hai thứ. Một: bảng ba món hay hai mươi món thì đoạn code vẫn
dài đúng ba dòng. Hai: quán thêm bớt món tuỳ ý, bạn không phải sửa con số nào —
vòng lặp tự chạy đúng số lượt.

Việc đi qua lần lượt từng phần tử như vậy có tên riêng: **duyệt** danh sách.
::::

::::predict{#doan-vong-lap commitOnce}
Đoạn dưới bỏ luôn câu mời, chỉ in ra cái tên trong vòng lặp. **Trước khi bấm
chạy**, bạn đoán màn hình hiện ra những gì?

```python title=readonly
thuc_don = ["Phở tái", "Phở chín", "Phở nạm"]
for mon in thuc_don:
    print(mon)
```

:::opt{correct}
Ba dòng: Phở tái, rồi Phở chín, rồi Phở nạm
:::

:::opt
Ba dòng: 0, rồi 1, rồi 2
::why
Gần đúng ở chỗ bạn nhớ rất chính xác bài lặp trước: với `for so in range(3)`,
cái tên trong vòng lặp lần lượt giữ 0, 1, 2. Bạn cũng đếm đúng số lượt.

Chỗ lệch nằm ở thứ đứng sau chữ `in`. Lần này không phải `range(3)` mà là chính
danh sách, nên thứ máy dán lên `mon` là *món*, không phải *chỗ đứng của món*.

Muốn thấy 0, 1, 2 thì viết lại thành `for so in range(3)`.
::
:::

:::opt
Một dòng: ['Phở tái', 'Phở chín', 'Phở nạm']
::why
Gần đúng, và gần đúng theo một cách đáng khen: bạn nhớ rằng `thuc_don` giữ cả ba
món cùng lúc, và in cả cái tên ấy ra thì đúng là được nguyên một dãy như thế.
`print(thuc_don)` cho ra chính dòng bạn chọn.

Chỗ lệch: bên trong `print` lần này không phải `thuc_don` mà là `mon`. `mon`
không giữ cả dãy — mỗi lượt nó chỉ giữ đúng một món.
::
:::

:::opt
Một dòng: Phở tái
::why
Gần đúng ở chỗ bạn lần được lượt đầu tiên hoàn toàn chính xác: `mon` nhận
"Phở tái", màn hình hiện *Phở tái*. Đó đúng là những gì xảy ra ở lượt một.

Chỗ lệch nằm ở chữ "vòng". Làm xong lượt đầu, máy quay lại đầu khối, lấy món kế
tiếp dán lên `mon`, rồi chạy lại đúng dòng `print` ấy. Nó chỉ dừng khi trong dãy
không còn ô nào chưa lấy. Ba ô thì ba lượt — một dòng `print` in ra ba dòng chữ.
::
:::
::::

::::code{#in-ca-bang}
Chủ quán muốn in thực đơn ra giấy, mỗi món một dòng, có dấu gạch đầu dòng cho dễ
đọc.

Phần việc bên trong đã viết sẵn — nó gọi món bằng cái tên `mon`. Hãy viết nốt
dòng `for` để Byte đọc hết bảng.

```python title=starter
thuc_don = ["Phở tái", "Phở chín", "Phở nạm"]
for ___:
    print(f"- {mon}")
```

```python title=solution
thuc_don = ["Phở tái", "Phở chín", "Phở nạm"]
for mon in thuc_don:
    print(f"- {mon}")
```

```python title=test
# Chạy xong vòng lặp, cái tên trong vòng lặp còn giữ giá trị của lượt cuối cùng.
# Nếu nó đang giữ "Phở nạm" thì vòng lặp đã đi qua từng MÓN của danh sách, chứ
# không phải đi qua các con số chỉ chỗ đứng.
assert mon == "Phở nạm", "đi hết bảng ba món thì món cuối cùng vòng lặp cầm trên tay là Phở nạm — món nằm cuối thực đơn, chứ không phải con số chỗ đứng của nó"
assert thuc_don == ["Phở tái", "Phở chín", "Phở nạm"], "in bảng ra giấy là đọc bảng chứ không sửa bảng, nên hết vòng lặp thực đơn vẫn còn nguyên ba món theo đúng thứ tự cũ"
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa chữ `for` và dấu hai chấm. Nhìn lại ví dụ ở trên - chỗ đó gồm hai phần: một cái tên, rồi chữ `in` kèm nơi lấy món.
- kind: strategy
  body: Dòng `print` bên dưới đang gọi món bằng cái tên `mon`, nên cái tên bạn đặt phải đúng là `mon`. Còn nơi lấy món chính là danh sách ở dòng trên.
- kind: one-line
  body: Viết `mon in thuc_don` vào chỗ trống, giữ nguyên dấu hai chấm ở cuối dòng.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba dòng code đọc hết cả bảng. Bảng hai mươi món cũng vẫn ba dòng ấy thôi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vòng lặp của bạn đọc đúng số món có trong dãy — ba ô thì ba lượt. Nhưng cả ba ô
ấy do bạn gõ sẵn ở dòng đầu chương trình, trước khi quán mở cửa.

Giữa buổi, bếp báo lên: hôm nay có thêm **Phở gầu**. Tấm bảng phấn ngoài cửa thì
chỉ cần viết thêm một dòng là xong.

Còn cái dãy đang nằm trong chương trình — nó có nhận thêm một ô nữa không? Hay
ba ô là ba ô, muốn khác đi thì phải sửa dòng đầu rồi chạy lại từ đầu?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
