---
id: nen-tang.re-nhanh-va-lap.buoc-nhay-cua-range
title: Bước nhảy của range
summary: Con số thứ ba trong ngoặc là khoảng cách giữa hai giá trị liền nhau — nhờ nó vòng lặp đi thưa được, và đi lùi được.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ctrl.range-step]
requires: [ctrl.range-start, ctrl.for-range, ctrl.continue]
concepts: [ctrl.lap, core.pham-vi]
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
Từ trước tới giờ mình toàn bước từng bước một. Mình bước dài hơn cũng được mà.
::::

::::explain{#nam-ngay-nam-rai}
Câu hỏi bỏ ngỏ của bài trước: sổ chi tiêu ghi cả tháng, và bà chủ muốn xem
**mỗi thứ Hai** tiêu bao nhiêu. Thứ Hai tháng này rơi vào ngày 1, 8, 15, 22 và
29.

Hai con số trong `range` cắt được một **đoạn liền nhau** — ngày 8 tới ngày 14
thì gọn gàng. Năm ngày thứ Hai kia nằm rải ra cả tháng, không liền nhau, nên
không có đoạn nào để cắt. Với những gì đang có trong tay, bạn phải viết thế
này:

```python title=readonly
for ngay in range(1, 30):
    if ngay != 1 and ngay != 8 and ngay != 15 and ngay != 22 and ngay != 29:
        continue
    print(f"Thứ Hai ngày {ngay}")
```

```text title=readonly
Thứ Hai ngày 1
Thứ Hai ngày 8
Thứ Hai ngày 15
Thứ Hai ngày 22
Thứ Hai ngày 29
```

Ra đúng năm dòng. Nhưng nhìn dòng `if`: năm con số chép tay vào một điều kiện
dài. Tháng sau thứ Hai rơi vào ngày khác, bạn phải sửa cả năm chỗ — và tháng
nào cũng phải sửa lại.

Trong khi năm con số ấy có một luật rất gọn, gọn hơn cả câu điều kiện kia:
**cách nhau đúng bảy**. Từ 1 bước bảy tới 8, bước bảy nữa tới 15, cứ thế.

Nghĩ tới chuyến xe buýt quen. Nó không đỗ trước mọi số nhà — nó đỗ cách bảy
căn một lần. Muốn nói trọn chuyến xe ấy, bạn cần ba điều: lên ở đâu, xuống
trước số nhà nào, và **mỗi lần đỗ cách nhau mấy căn**.

`range` có đúng ba chỗ cho ba điều đó.
::::

::::example{#con-so-thu-ba}
Đặt **ba** con số trong ngoặc, cách nhau bằng dấu phẩy:

```python title=readonly
for ngay in range(1, 30, 7):
    print(f"Thứ Hai ngày {ngay}")
```

```text title=readonly
Thứ Hai ngày 1
Thứ Hai ngày 8
Thứ Hai ngày 15
Thứ Hai ngày 22
Thứ Hai ngày 29
```

Năm dòng y hệt, không còn `if`, không còn `continue`. Đọc ba con số theo thứ tự:

- `1` — điểm khởi hành, đúng vai trò nó nhận ở bài trước. Con số này **có** vào
  vòng.
- `30` — chỗ dừng, cũng đúng vai trò cũ. Con số này **không bao giờ** vào vòng.
- `7` — chỗ mới của bài hôm nay: **bước nhảy**. Đó là khoảng cách từ một giá
  trị tới giá trị kế tiếp.

Máy đi thế này: đặt `ngay` ở 1, chạy thân vòng; cộng 7 thành 8, 8 vẫn chưa tới
chỗ dừng nên chạy thân tiếp; 15, rồi 22, rồi 29. Cộng 7 lần nữa thành 36 — 36
đã qua chỗ dừng, vòng kết thúc. Số 36 không được in ra, mà số 30 cũng chưa bao
giờ xuất hiện.

Và `range(1, 30)` với `range(1, 30, 7)` khác nhau đúng một chỗ ấy. Không viết
con số thứ ba thì bước nhảy là **1** — nghĩa là `range(1, 30)` và
`range(1, 30, 1)` là một thứ. Từ trước tới giờ bạn vẫn đi bước 1, chỉ là chưa
phải nói ra.
::::

::::predict{#hai-den-muoi-mot commitOnce}
Byte sắp chạy đoạn dưới. **Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
for so in range(2, 11, 3):
    print(so)
```

:::opt{correct}
Ba dòng: 2, 5, 8
:::

:::opt
Bốn dòng: 2, 5, 8, 11
::why
Gần đúng gần hết. Bạn đọc đúng điểm khởi hành là 2, và đọc đúng bước nhảy là 3
— cộng 3 mỗi lượt, ra 2, 5, 8, 11. Phép đếm ấy chính xác.

Chỗ lệch nằm ở con số 11. Nó là **chỗ dừng**, và bước nhảy không hề đổi vai trò
của nó: máy dừng ngay khi giá trị chạm hoặc vượt qua đó. Giá trị 11 chạm đúng
chỗ dừng, nên nó không được vào vòng — y như 3 không vào vòng trong `range(3)`.
::
:::

:::opt
Ba dòng: 2, 6, 10
::why
Gần đúng ở chỗ bạn hiểu con số thứ ba là chuyện đi thưa — nhảy cách quãng chứ
không đi từng số một. Đó đúng là việc nó làm.

Chỗ lệch là cách đếm quãng. Bạn đang đọc `3` là "bỏ qua ba số rồi lấy số kế" —
bỏ 3, 4, 5 rồi lấy 6. Máy thì đọc `3` là **khoảng cách** giữa hai giá trị liền
nhau: 2 rồi 2 + 3 = 5. Nói ngắn gọn: bước nhảy là số được cộng thêm, không phải
số bị bỏ qua.
::
:::

:::opt
Ba dòng: 2, 3, 4
::why
Gần đúng ở chỗ bạn đang tìm chỗ đứng cho vai trò "số lượt" — vai trò mà con số
duy nhất trong `range(3)` vẫn giữ. Ba con số thì đúng là vòng này chạy ba lượt,
nên bạn không đếm sai lượt nào.

Chỗ lệch: khi `range` nhận ba số thì không con số nào là số lượt cả. Hai số đầu
là hai mốc trên trục số, số thứ ba là khoảng cách mỗi bước. Số lượt không ai
viết ra — nó là hệ quả của ba con số kia.
::
:::
::::

::::explain{#di-lui-va-di-hut}
Ba chuyện đáng cất vào túi về bước nhảy:

- **Bước âm thì đi lùi.** `range(5, 0, -1)` cho ra 5, 4, 3, 2, 1. Đi lùi thì
  chỗ dừng nằm **dưới** điểm khởi hành, và luật "chỗ dừng không bao giờ tới
  lượt" vẫn giữ nguyên: muốn số cuối cùng là 1 thì phải viết chỗ dừng là 0.
- **Đi sai hướng thì vòng chạy 0 lượt.** `range(1, 10, -1)` bảo máy khởi hành ở
  1 rồi lùi dần, trong khi chỗ dừng lại nằm phía trên. Máy không kêu một tiếng
  nào, thân vòng chỉ đơn giản không chạy lần nào — đúng cảnh im lặng bạn đã gặp
  với `range(15, 8)`.
- **Bước 0 thì máy dừng và nói.** `range(1, 10, 0)` cho `ValueError` kèm dòng
  `range() arg 3 must not be zero`. Cũng phải thôi: bước 0 nghĩa là đứng yên
  mãi ở một chỗ, và đó là vòng lặp vô hạn — thứ máy giữ bạn khỏi rơi vào.
::::

::::code{#dem-nguoc-khai-truong}
Quán phở mở thêm chi nhánh. Trước lúc bà chủ bấm chuông khai trương, cả nhà đếm
ngược **năm giây**.

Máy phải in ra đúng năm dòng này:

```text title=readonly
Còn 5 giây
Còn 4 giây
Còn 3 giây
Còn 2 giây
Còn 1 giây
```

Điền ba con số vào ba chỗ trống.

```python title=starter
for giay in range(___, ___, ___):
    print(f"Còn {giay} giây")
```

```python title=solution
for giay in range(5, 0, -1):
    print(f"Còn {giay} giây")
```

```python title=test
# Chấm bằng OUTPUT: năm dòng, con số đi lùi từ 5 xuống 1.
pass
```

:::hints
- kind: attention
  body: Con số đầu tiên in ra là 5, con số cuối cùng in ra là 1 — dãy này đi xuống chứ không đi lên. Chỗ trống thứ ba là chỗ nói hướng đi.
- kind: strategy
  body: Chỗ trống thứ nhất là điểm khởi hành, viết thẳng số đầu tiên. Chỗ trống thứ ba là bước nhảy, và mỗi lượt con số giảm đúng một đơn vị. Chỗ trống thứ hai là chỗ dừng — máy dừng NGAY TRƯỚC nó, nên với hướng đi xuống, nó phải nhỏ hơn 1 một đơn vị.
- kind: one-line
  body: "Viết `range(5, 0, -1)`, giữ nguyên dấu hai chấm cuối dòng."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: Còn 1 giây
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dòng nói trọn cả chuyến đi: lên ở đâu, xuống chỗ nào, mỗi bước dài bao nhiêu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bước nhảy giúp bạn đi thưa trên **một** hàng số: hàng ngày trong tháng, hàng
giây đếm ngược, hàng phút hớt bọt nồi nước dùng.

Nhưng tờ lịch dán trên tường quán không phải một hàng. Nó là **4 tuần × 7
ngày**: bốn dòng, mỗi dòng bảy ô.

Bạn viết được vòng lặp chạy qua bốn tuần — `range(1, 5)`, dễ thôi. Mỗi lượt của
nó in ra tên một tuần. Nhưng bên trong tuần ấy còn bảy ô ngày phải in nữa, và
mỗi ô lại là một lượt riêng.

Ai đứng ra in bảy ô đó?
::::

::::checkpoint{mastery=0.8}
::::
