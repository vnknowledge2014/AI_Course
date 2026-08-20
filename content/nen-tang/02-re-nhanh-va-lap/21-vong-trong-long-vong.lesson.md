---
id: nen-tang.re-nhanh-va-lap.vong-trong-long-vong
title: Vòng trong lòng vòng
summary: Một vòng lặp đặt được trong thân một vòng lặp khác, và mỗi lượt của vòng ngoài chạy trọn vẹn cả vòng trong.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ctrl.nested-loop]
requires: [ctrl.range-step, ctrl.for-range, ctrl.block-indent, ctrl.if-nested]
concepts: [ctrl.lap, core.khoi-lenh]
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
Kim giờ nhích một nấc thì kim phút đã quay trọn một vòng. Mình cũng làm được thế.
::::

::::explain{#bay-o-ben-trong-mot-tuan}
Câu hỏi bỏ ngỏ của bài trước: tờ lịch dán tường là 4 tuần × 7 ngày. Ai in bảy ô
bên trong một tuần?

Bạn đã có sẵn cả hai nửa. Nửa thứ nhất — chạy qua bốn tuần:

```python title=readonly
for tuan in range(1, 5):
    print(f"Tuần {tuan}")
```

```text title=readonly
Tuần 1
Tuần 2
Tuần 3
Tuần 4
```

Nửa thứ hai — chạy qua bảy ngày — cũng chỉ là `for ngay in range(1, 8):`. Câu
hỏi duy nhất còn lại là **đặt nó ở đâu**.

Nhìn lên mặt đồng hồ treo trong quán. Kim giờ nhích từ 8 sang 9 đúng một nấc.
Trong khoảng đó kim phút không đứng đợi — nó quay **trọn vẹn một vòng sáu mươi
nấc**. Rồi kim giờ mới nhích tiếp sang 10, và kim phút lại quay trọn một vòng
nữa, đếm lại từ đầu.

Bốn tuần là kim giờ. Bảy ngày là kim phút.

Và bạn đã gặp đúng hình dạng này ở bài đầu track: một `if` đặt được trong thân
một `if` khác. Thân của một câu lệnh là **phần việc mỗi lượt**, mà phần việc
thì chứa được bất kỳ câu lệnh nào — kể cả một vòng lặp.
::::

::::example{#vong-trong-than-vong}
Thu nhỏ lại cho vừa màn hình: 2 tuần, mỗi tuần 3 ngày.

```python title=readonly
for tuan in range(1, 3):
    print(f"Tuần {tuan}")
    for ngay in range(1, 4):
        print(f"    ngày {ngay}")
```

```text title=readonly
Tuần 1
    ngày 1
    ngày 2
    ngày 3
Tuần 2
    ngày 1
    ngày 2
    ngày 3
```

Đọc theo đúng đường máy đi:

- **Lượt 1 của vòng ngoài.** `tuan` là 1. Máy in `Tuần 1`. Dòng kế tiếp trong
  thân là một vòng lặp, nên máy chạy nó — và chạy cho tới khi nó xong hẳn: ngày
  1, ngày 2, ngày 3. Tới đây lượt 1 mới kết thúc.
- **Lượt 2 của vòng ngoài.** `tuan` là 2. In `Tuần 2`. Lại gặp vòng trong, lại
  chạy trọn: ngày 1, ngày 2, ngày 3. Để ý nó đếm **lại từ 1**.

Đếm số lần dòng `print` ngày chạy: 2 × 3 = **6**, không phải 2 + 3 = 5. Vòng
trong chạy trọn một lượt của chính nó cho **mỗi** lượt vòng ngoài, nên số lượt
nhân với nhau.

Hình dạng này có tên: **vòng lặp lồng nhau**. Cái ôm bên ngoài gọi là **vòng
ngoài**, cái nằm trong thân gọi là **vòng trong**.
::::

::::predict{#ban-va-to commitOnce}
Bà chủ in phiếu bếp: hai bàn, mỗi bàn ba tô. **Trước khi bấm chạy**, bạn đoán
màn hình hiện ra gì?

```python title=readonly
for ban in range(1, 3):
    for to in range(1, 4):
        print(f"Bàn {ban} - tô {to}")
```

:::opt{correct}
Sáu dòng, theo thứ tự: Bàn 1 - tô 1, Bàn 1 - tô 2, Bàn 1 - tô 3, Bàn 2 - tô 1, Bàn 2 - tô 2, Bàn 2 - tô 3
:::

:::opt
Hai dòng: Bàn 1 - tô 1, rồi Bàn 2 - tô 2
::why
Gần đúng ở chỗ bạn thấy hai cái tên cùng đổi giá trị theo lượt, và hình dung
chúng bước song song với nhau — như hai người sóng đôi. Với hai dãy số đi cùng
nhịp thì cách hình dung ấy chính xác, và bạn sẽ gặp đúng cảnh đó về sau.

Chỗ lệch: ở đây hai vòng không sóng đôi, chúng **lồng vào nhau**. Vòng `to` nằm
trong thân vòng `ban`, nghĩa là nó là *phần việc* của một lượt bàn — và phần
việc thì phải làm xong hẳn rồi mới sang lượt sau. Bàn 1 phải xong cả ba tô mới
tới lượt bàn 2.
::
:::

:::opt
Sáu dòng, theo thứ tự: Bàn 1 - tô 1, Bàn 2 - tô 1, Bàn 1 - tô 2, Bàn 2 - tô 2, Bàn 1 - tô 3, Bàn 2 - tô 3
::why
Gần đúng ở phần khó nhất: bạn đếm ra sáu dòng, tức là bạn đã nắm được chuyện số
lượt nhân với nhau chứ không cộng vào nhau.

Chỗ lệch chỉ là thứ tự, và nó lệch vì bạn cho cái tên bên ngoài đổi nhanh hơn
cái tên bên trong. Máy làm ngược lại: vòng trong là phần việc phải xong trước
khi vòng ngoài dám nhích một nấc. Nên `to` chạy hết 1, 2, 3 rồi `ban` mới đổi —
tên bên ngoài luôn là cái đổi chậm nhất.
::
:::

:::opt
Năm dòng: hai lượt bàn rồi ba lượt tô
::why
Gần đúng ở chỗ bạn nhìn ra đúng hai vòng và đúng số lượt của từng vòng: hai và
ba. Đó là hai con số cần dùng.

Chỗ lệch nằm ở phép tính ghép chúng lại. Cộng là phép của hai vòng **nối đuôi**
nhau — vòng này chạy xong rồi tới vòng kia, cả hai cùng sát lề. Ở đây vòng `to`
lùi vào trong thân vòng `ban`, nên nó chạy lại trọn vẹn cho mỗi lượt bàn: 2 × 3
= 6.
::
:::
::::

::::explain{#bon-dieu-de-vap}
Bốn chuyện đáng cất vào túi:

- **Thụt lề là thứ duy nhất nói ai nằm trong ai.** Trong ví dụ lịch trên, kéo
  dòng `for ngay` ra sát lề bằng `for tuan` thì hai vòng thành hai vòng nối
  đuôi: máy in hai dòng tuần trước, xong hẳn, rồi mới in ba dòng ngày — năm dòng
  thay vì tám. Cũng đoạn code ấy, chỉ đổi bốn dấu cách, kết quả khác hẳn.
- **Hai vòng cần hai cái tên khác nhau.** Đặt cả hai là `ngay` thì máy không
  báo lỗi, nhưng vòng trong dán đè lên cái tên đó mỗi lượt — và sau khi vòng
  trong chạy xong, cái tên không còn giữ giá trị của vòng ngoài nữa.
- **Vòng trong đếm lại từ đầu mỗi lượt.** Nó không nhớ tuần trước nó đã chạy
  tới đâu. Mỗi lần được gọi tới, nó là một vòng lặp mới tinh.
- **Số lượt nhân lên rất nhanh.** 4 × 7 = 28 thì nhẹ. Nhưng 1000 × 1000 là một
  triệu lượt thân vòng, và lúc đó bạn sẽ thấy máy chậm hẳn đi.
::::

::::code{#in-to-lich-thang}
Bà chủ dán tờ lịch tháng lên tường bếp: **4 tuần, mỗi tuần 7 ngày**.

Máy phải in ra 28 dòng, mở đầu và kết thúc thế này:

```text title=readonly
Tuần 1 - ngày 1
Tuần 1 - ngày 2
...
Tuần 4 - ngày 6
Tuần 4 - ngày 7
```

Vòng ngoài đã dựng sẵn. Hãy viết vòng trong vào chỗ trống.

```python title=starter
for tuan in range(1, 5):
    ___
        print(f"Tuần {tuan} - ngày {ngay}")
```

```python title=solution
for tuan in range(1, 5):
    for ngay in range(1, 8):
        print(f"Tuần {tuan} - ngày {ngay}")
```

```python title=test
# Chấm bằng OUTPUT: 28 dòng, dòng cuối cùng là tuần 4 ngày 7.
pass
```

:::hints
- kind: attention
  body: Dòng `print` dùng hai cái tên: `tuan` và `ngay`. Một cái đã có chủ ở dòng đầu; cái còn lại chưa ai sinh ra, và chỗ trống chính là nơi sinh ra nó.
- kind: strategy
  body: Chỗ trống cần đúng một dòng mở vòng lặp, viết y như dòng đầu nhưng cho ngày: một cái tên, rồi một `range` chạy từ ngày 1 tới hết ngày 7. Nhớ dấu hai chấm cuối dòng, vì dòng `print` bên dưới đang lùi vào để làm thân cho nó.
- kind: one-line
  body: "Viết `for ngay in range(1, 8):` vào chỗ trống, giữ nguyên bốn dấu cách đầu dòng."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: Tuần 4 - ngày 7
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai dòng `for` mà ra hai mươi tám dòng chữ. Kim giờ với kim phút đấy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vẫn tờ lịch 4 × 7 ấy, nhưng lần này mỗi ô là số tiền tiêu trong ngày. Bà chủ
nhờ bạn tìm ngày đầu tiên vượt 500 nghìn, và tìm thấy thì thôi, khỏi xem tiếp.

Bạn đặt `break` vào đúng chỗ đã học ở bài dừng ngay khi đủ biết: trong thân,
ngay sau khi in ra ngày tìm được.

Chạy lên, nó dừng thật — ngày sau đó trong tuần ấy không hiện ra nữa. Nhưng
`Tuần 3`, `Tuần 4` vẫn cứ in ra đều đặn, và các ngày của chúng cũng vậy.

Vậy `break` vừa thoát khỏi cái gì?
::::

::::checkpoint{mastery=0.8}
::::
