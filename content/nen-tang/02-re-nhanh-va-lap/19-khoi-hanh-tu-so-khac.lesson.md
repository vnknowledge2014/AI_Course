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
estimatedMinutes: 12
teaches: [ctrl.range-start]
requires: [ctrl.for-range, ctrl.loop-variable, ctrl.if, ctrl.while-check-timing, core.fstring]
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
Bà chủ chỉ hỏi tuần thứ hai thôi, mà mình phải ghé cả tám ngày đầu tháng.
::::

::::explain{#cho-dung-thi-noi-duoc-cho-di-thi-chua}
Sổ chi tiêu của quán ghi cả tháng. Hôm nay bà chủ chỉ muốn xem tuần thứ hai —
**ngày 8 tới ngày 14**.

Với những gì đang có trong tay, bạn cho máy đi hết nửa tháng rồi lọc lại bằng
một câu `if`:

```python title=readonly
for ngay in range(15):
    if ngay >= 8:
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
vào tới dòng `if`, thấy câu hỏi sai, rồi quay ra tay không.

Chỗ vướng nằm ở chính dòng `for`. Bạn chưa có cách nào bảo `range` đừng phát ra
tám con số đầu, nên phải vá tạm ở dòng dưới.

Trước khi tìm cách nói, hãy nhìn kỹ lại con số bạn vẫn viết. Realm 0 dạy bạn đọc
`range(15)` là **mười lăm lượt**, và cách đọc đó chưa sai lần nào. Bây giờ viết
thẳng ra dãy số mà nó phát ra:

```text title=readonly
0 1 2 3 4 5 6 7 8 9 10 11 12 13 14
```

Cùng một dãy ấy còn đọc được cách thứ hai: **khởi hành từ 0, dừng trước 15**.
Đếm lại mà xem — vẫn đúng mười lăm con số đó, không thừa không thiếu. Hai cách
đọc trùng khít nhau, và chúng trùng được là vì điểm khởi hành tình cờ bằng 0.

Nhưng chỉ một trong hai cách đọc có chỗ để nói câu hôm nay bạn cần. "Mười lăm
lượt" là một con số đơn độc, không có khe nào nhét chữ *bắt đầu từ 8* vào.
"Khởi hành từ 0, dừng trước 15" thì có hai đầu — và một đầu đang bỏ trống, vì
tới giờ bạn chưa bao giờ phải nói tới nó.

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

Y hệt bảy dòng vừa nãy, và dòng `if` biến mất. Đọc hai con số:

- `8` là **điểm khởi hành** — cái đầu vừa nãy còn bỏ trống. Con số này **có**
  vào vòng: nó chính là giá trị của lượt đầu tiên.
- `15` là **chỗ dừng** — đúng cái đầu kia, cái mà `range(15)` vẫn giữ. Con số
  này **không bao giờ** vào vòng.

Nên số cuối cùng thật sự chạy là 14, số ngay trước chỗ dừng — y như `range(3)`
phát ra 0, 1, 2 mà không phát ra số 3 lần nào.

Và `range(7)` với `range(0, 7)` là một thứ. Viết một số thì máy hiểu ngầm điểm
khởi hành là 0; viết hai số là bạn nói thẳng nó ra. Cách đọc cũ không mất đi
đâu cả — nó là ca đặc biệt của cách đọc mới, ca mà điểm khởi hành đúng bằng 0.
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
Gần đúng ở chỗ bạn giữ chắc cách đọc cũ và áp nó đúng nguyên văn: con số trong
ngoặc là **số lượt**, và `range(3)` quả thật cho ra 0, 1, 2.

Chỗ lệch: cách đọc ấy trùng với dãy thật chỉ khi `range` có đúng **một** con
số, vì khi ấy điểm khởi hành bằng 0. Thêm con số thứ hai vào là hai cách đọc
tách nhau ra — `3` thôi làm số lượt, nó chuyển sang làm điểm khởi hành, và số
lượt không còn ai viết thẳng ra nữa.
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

- **Số lượt = chỗ dừng − điểm khởi hành, khi vòng đi từng bước một.**
  `range(8, 15)` chạy 15 − 8 = 7 lượt. Nhẩm được số lượt trước khi chạy là cách
  nhanh nhất để bắt lỗi lệch một đơn vị. (Mấy chữ *từng bước một* ở đây là có
  lý do — bài sau bạn sẽ thấy nó.)
- **Điểm khởi hành không nhỏ hơn chỗ dừng thì vòng chạy 0 lượt.** `range(15, 8)`
  không báo lỗi gì cả — thân vòng chỉ đơn giản không chạy lần nào. Đúng cảnh im
  lặng bạn đã gặp ở bài `while`: điều kiện sai ngay lần đọc đầu thì vòng chạy 0
  lượt, và 0 lượt là một kết quả hợp lệ chứ không phải một lỗi.
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
# Chấm bằng OUTPUT, và chấm TRỌN màn hình: phải đúng bảy dòng, dòng đầu là
# Ngày 15, dòng cuối là Ngày 21. Khởi hành sai một đơn vị thì dòng đầu sai;
# chỗ dừng sai một đơn vị thì thừa hoặc thiếu dòng cuối.
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
  match: regex
  expect: ^Ngày 15\nNgày 16\nNgày 17\nNgày 18\nNgày 19\nNgày 20\nNgày 21\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bảy lượt cho bảy dòng. Không lượt nào chạy không nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đoạn ngày 8–14 cắt được, đoạn ngày 15–21 cũng cắt được. Đoạn liền nhau nào cũng
vậy: nói điểm khởi hành, nói chỗ dừng, xong.

Nhưng sổ chi tiêu còn một câu hỏi khác: **mỗi thứ Hai** tiêu bao nhiêu. Thứ Hai
tháng này rơi vào ngày 1, 8, 15, 22 — và 29 nữa nếu tháng còn dài. Năm ngày ấy
nằm rải ra cả tháng chứ không liền nhau, nên không có đoạn nào để cắt.

Hai con số cắt được một khúc liền. Còn năm ngày cách nhau đúng bảy thì nói bằng
gì? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
