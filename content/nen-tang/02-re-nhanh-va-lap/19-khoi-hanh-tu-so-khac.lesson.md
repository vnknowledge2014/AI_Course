---
id: nen-tang.re-nhanh-va-lap.khoi-hanh-tu-so-khac
title: Khởi hành từ số khác 0
summary: range nhận hai số — số thứ nhất là chỗ khởi hành, số thứ hai vẫn là chỗ dừng.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ctrl.range-start]
requires: [ctrl.for-range, ctrl.loop-variable, ctrl.continue]
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
Bài trước bạn bảo mình bỏ qua tám ngày đầu. Hôm nay mình khỏi phải ghé chúng.
::::

::::explain{#cho-dung-thi-noi-duoc-cho-di-thi-chua}
Câu hỏi bỏ ngỏ của bài trước: sổ chi tiêu ghi cả tháng, nhưng bạn chỉ muốn xem
tuần thứ hai — **ngày 8 tới ngày 14**. `continue` làm được, nhưng máy vẫn phải
ghé từng ngày rồi mới bỏ.

Viết ra thì thấy rõ chỗ vướng:

```python title=readonly
for ngay in range(15):
    if ngay < 8:
        continue
    print(f"Ngày {ngay}")
```

```text title=readonly
Ngày 8
Ngày 9
Ngày 10
Ngày 11
Ngày 12
Ngày 13
Ngày 14
```

Bảy dòng đúng ý. Nhưng thân vòng chạy **15 lượt** để in bảy dòng: tám lượt đầu
vào tới dòng `if` rồi quay ra tay không.

Để ý một chuyện: chỗ **dừng** thì bạn đã nói được từ lâu. `range(15)` nghĩa là
"dừng trước 15", và bạn không phải viết `continue` nào cho phần đuôi cả. Chỉ
chỗ **khởi hành** là chưa có cách nói — nên bạn phải vá tạm bằng `continue`.

Nghĩ tới dãy nhà mặt phố. Bạn cần thăm nhà số 8 đến nhà số 14. Không ai đi bộ
từ đầu phố rồi lướt qua tám căn đầu — người ta bảo bác xe ôm thả xuống ngay
trước nhà số 8.

`range` có sẵn một chỗ để nói câu đó.
::::

::::example{#hai-so-trong-ngoac}
Đặt **hai** con số trong ngoặc, cách nhau một dấu phẩy:

```python title=readonly
for ngay in range(8, 15):
    print(f"Ngày {ngay}")
```

```text title=readonly
Ngày 8
Ngày 9
Ngày 10
Ngày 11
Ngày 12
Ngày 13
Ngày 14
```

Y hệt bảy dòng vừa nãy, không còn `if`, không còn `continue`. Đọc hai con số:

- `8` là **điểm khởi hành**. Con số này **có** vào vòng — nó chính là giá trị
  của lượt đầu tiên.
- `15` là **chỗ dừng**, đúng vai trò nó vẫn giữ từ trước tới giờ. Con số này
  **không bao giờ** vào vòng.

Nên số cuối cùng thật sự chạy là 14 — số ngay trước chỗ dừng. Đây là cùng một
luật bạn đã dùng với `range(3)`: ba con số phát ra là 0, 1, 2, còn số 3 không
xuất hiện lượt nào.

Và `range(7)` với `range(0, 7)` là một thứ. Viết một số thì máy hiểu ngầm điểm
khởi hành là 0; viết hai số là bạn nói thẳng nó ra.
::::

::::predict{#tu-ba-toi-bay commitOnce}
Byte sắp chạy đoạn dưới. **Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
for so in range(3, 7):
    print(so)
```

:::opt{correct}
Bốn dòng: 3, 4, 5, 6
:::

:::opt
Năm dòng: 3, 4, 5, 6, 7
::why
Gần đúng gần hết. Điểm khởi hành bạn đọc chính xác — vòng bắt đầu từ 3, không
phải từ 0 — và đó đúng là điều mới của bài này.

Chỗ lệch nằm ở đầu kia. `7` là **chỗ dừng**, và máy dừng ngay trước nó. Vai trò
ấy không đổi khi thêm số thứ nhất vào: `range(7)` cho 0 tới 6, `range(3, 7)` cho
3 tới 6. Con số thứ hai luôn là cái biển "tới đây thì quay đầu", không phải số
cuối cùng của dãy.
::
:::

:::opt
Ba dòng: 0, 1, 2
::why
Gần đúng ở chỗ bạn đang giữ chắc một luật đã học và áp dụng nó đúng nguyên văn:
con số trong ngoặc là **số lượt**, và `range(3)` đúng là cho ra 0, 1, 2.

Chỗ lệch: luật ấy là luật của `range` khi nó nhận **một** con số. Có hai con số
thì vai trò đổi hẳn — số thứ nhất thôi làm số lượt, nó chuyển sang làm điểm
khởi hành. Số lượt không còn ai viết ra nữa.
::
:::

:::opt
Bảy dòng: 3, 4, 5, 6, 7, 8, 9
::why
Gần đúng ở hai chỗ: bạn nhận ra `3` là điểm khởi hành, và bạn tìm một chỗ để
gán vai trò "số lượt" — vai trò mà `range` một số vẫn giữ. Cách đọc đó có lý.

Chỗ lệch: khi `range` nhận hai số thì **cả hai** đều là mốc trên cùng một trục
số, không con số nào là số lượt. Số lượt là hiệu của hai mốc: 7 − 3 = 4. Muốn
bảy lượt kể từ 3 thì phải viết chỗ dừng là 10.
::
:::
::::

::::explain{#vai-chuyen-gon-ve-hai-moc}
Ba chuyện đáng cất vào túi:

- **Số lượt = chỗ dừng − điểm khởi hành.** `range(8, 15)` chạy 15 − 8 = 7 lượt.
  Nhẩm được số lượt trước khi chạy là cách nhanh nhất để bắt lỗi lệch một đơn vị.
- **Điểm khởi hành không nhỏ hơn chỗ dừng thì vòng chạy 0 lượt.** `range(15, 8)`
  không báo lỗi gì cả — thân vòng chỉ đơn giản không chạy lần nào. Bạn đã gặp
  đúng cảnh im lặng này ở bài điều kiện `while` sai ngay từ lượt đầu.
- **Số âm cũng dùng được.** `range(-3, 2)` cho ra −3, −2, −1, 0, 1. Trục số vẫn
  là trục số.
::::

::::code{#nhan-cho-tuan-thu-ba}
Bà chủ cắt nhãn dán vào sổ chi tiêu cho **tuần thứ ba**: từ ngày 15 tới ngày 21.

Máy phải in ra đúng bảy dòng thế này:

```text title=readonly
Ngày 15
Ngày 16
Ngày 17
Ngày 18
Ngày 19
Ngày 20
Ngày 21
```

Điền hai con số vào hai chỗ trống.

```python title=starter
for ngay in range(___, ___):
    print(f"Ngày {ngay}")
```

```python title=solution
for ngay in range(15, 22):
    print(f"Ngày {ngay}")
```

```python title=test
# Chấm bằng OUTPUT: bảy dòng, ngày chạy từ 15 tới 21.
pass
```

:::hints
- kind: attention
  body: Ngày đầu tiên cần in là 15, ngày cuối cùng cần in là 21. Hai chỗ trống là điểm khởi hành và chỗ dừng — và chỉ một trong hai được vào vòng.
- kind: strategy
  body: Chỗ trống thứ nhất viết thẳng ngày đầu tiên. Chỗ trống thứ hai là chỗ dừng, mà máy dừng NGAY TRƯỚC nó — nên nó phải lớn hơn ngày cuối cùng một đơn vị.
- kind: one-line
  body: "Viết `range(15, 22)`, giữ nguyên dấu hai chấm cuối dòng."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: Ngày 21
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bảy lượt cho bảy dòng. Không lượt nào chạy không nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ra đúng đoạn ngày 8–14 rồi, và đoạn nào cũng cắt được như vậy: nói điểm khởi
hành, nói chỗ dừng, xong.

Nhưng sổ chi tiêu còn một câu hỏi khác: **mỗi thứ Hai** tiêu bao nhiêu. Thứ Hai
tháng này rơi vào ngày 1, 8, 15, 22 — và 29 nữa nếu tháng còn dài. Năm ngày ấy
nằm rải ra cả tháng chứ không liền nhau, nên không có đoạn nào để cắt.

Với hai con số trong tay, bạn vẫn phải cho máy đi qua đủ hai mươi chín ngày rồi
`continue` hai mươi tư lần?
::::

::::checkpoint{mastery=0.8}
::::
