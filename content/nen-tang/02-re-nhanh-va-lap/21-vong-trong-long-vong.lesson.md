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
  reviewed: true
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

- **Thụt lề là thứ duy nhất nói ai nằm trong ai.** Trong ví dụ 2 tuần × 3 ngày
  ở trên, kéo dòng `for ngay` ra sát lề bằng `for tuan` thì hai vòng thành hai
  vòng nối đuôi: máy in hai dòng tuần trước, xong hẳn, rồi mới in ba dòng ngày
  — năm dòng thay vì tám. Cũng đoạn code ấy, chỉ đổi bốn dấu cách, kết quả khác
  hẳn.
- **Hai vòng cần hai cái tên khác nhau.** Đặt cả hai là `ngay` thì máy không
  báo lỗi, nhưng vòng trong dán đè lên cái tên đó mỗi lượt — và sau khi vòng
  trong chạy xong, cái tên không còn giữ giá trị của vòng ngoài nữa.
- **Vòng trong đếm lại từ đầu mỗi lượt.** Nó không nhớ tuần trước nó đã chạy
  tới đâu. Mỗi lần được gọi tới, nó là một vòng lặp mới tinh.
- **Số lượt nhân lên rất nhanh.** 4 × 7 = 28 thì nhẹ. Nhưng 1000 × 1000 là một
  triệu lượt thân vòng, và lúc đó bạn sẽ thấy máy chậm hẳn đi.
::::

::::repair{#in-to-lich-thang}
Bà chủ dán tờ lịch tháng lên tường bếp: **4 tuần, mỗi tuần 7 ngày**. Byte gõ
đoạn dưới. Đủ hai vòng lặp, đủ hai dòng `print`, không thiếu một chữ nào — mà
màn hình ra thế này:

```text title=readonly
Tuần 1
Tuần 2
Tuần 3
Tuần 4
    ngày 1
    ngày 2
    ngày 3
    ngày 4
    ngày 5
    ngày 6
    ngày 7
```

Mười một dòng: bốn cái tên tuần dồn hết lên đầu, rồi bảy ô ngày trơ trọi ở
cuối. Đúng cái cảnh 4 **cộng** 7 mà gạch đầu dòng thứ nhất vừa nói tới, trong
khi tờ lịch cần 4 **nhân** 7.

Tờ lịch thật in ra 32 dòng — mỗi tên tuần kèm ngay bảy ô ngày của tuần đó:

```text title=readonly
Tuần 1
    ngày 1
    ngày 2
...
Tuần 4
    ngày 6
    ngày 7
```

Sửa đoạn code cho ra đúng tờ lịch ấy. Không thêm dòng nào, không xoá dòng nào,
không đổi một con số nào.

```python title=starter
for tuan in range(1, 5):
    print(f"Tuần {tuan}")
for ngay in range(1, 8):
    print(f"    ngày {ngay}")
```

```python title=solution
for tuan in range(1, 5):
    print(f"Tuần {tuan}")
    for ngay in range(1, 8):
        print(f"    ngày {ngay}")
```

```python title=test
# Chấm bằng OUTPUT, và luật chấm phải phân biệt được ba cảnh khác nhau:
#
#   hai vòng nối đuôi (code khởi đầu)  → 11 dòng, bốn tuần dồn lên đầu;
#   chỉ lùi dòng `for ngay` vào 4      → IndentationError, không ra dòng nào;
#   chỉ lùi dòng `print` ngày vào 8    → máy vẫn chạy trơn tru và vẫn ra đúng
#                                        11 dòng ấy: lùi sâu thêm một mức
#                                        trong cùng một thân không đổi được
#                                        chuyện ai lồng trong ai.
#
# Chỉ khi CẢ HAI dòng lùi đúng mức thì mới ra 32 dòng đúng thứ tự. Regex dưới
# ghim hai dòng đầu, hai dòng cuối, và ghim luôn SỐ DÒNG ở giữa — `contains`
# nhìn một mẩu nên bản nối đuôi vẫn có `ngày 7` và vẫn lọt.
pass
```

:::hints
- kind: attention
  body: Đặt hai dòng `for` cạnh nhau rồi nhìn vào **đầu dòng**: cả hai đang bắt đầu ở cùng một chỗ, sát lề trái. Gạch đầu dòng thứ nhất ở mục trên vừa nói cái gì xảy ra với hai vòng cùng sát lề?
- kind: strategy
  body: Vòng ngày phải trở thành *phần việc mỗi lượt* của vòng tuần — nghĩa là nó nằm trong thân vòng tuần, đứng cùng mức với dòng `print` tên tuần. Rồi tới lượt dòng `print` ngày: nó là thân của vòng ngày, nên nó phải lùi thêm một mức nữa so với vòng ngày. Hai dòng cùng phải dịch, mỗi dòng một mức.
- kind: one-line
  body: "Thêm bốn dấu cách vào đầu dòng `for ngay in range(1, 8):` để nó đứng ở cột 4, rồi thêm bốn dấu cách vào đầu dòng `print` ngày để nó đứng ở cột 8."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Tuần 1\n    ngày 1\n(?:.+\n){28}    ngày 6\n    ngày 7$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai dòng lùi vào bốn dấu cách, mười một dòng thành ba mươi hai. Kim giờ với kim
phút đấy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vẫn tờ lịch 4 × 7 ấy, nhưng lần này mỗi ô là số tiền tiêu trong ngày. Bà chủ
nhờ bạn tìm ngày đầu tiên vượt 500 nghìn, và tìm thấy thì thôi, khỏi xem tiếp.

Bạn đặt `break` vào đúng chỗ đã học ở bài dừng ngay khi đủ biết: trong thân,
ngay sau khi in ra ngày tìm được.

Chạy lên, nó dừng thật — những ngày sau đó trong tuần ấy không hiện ra nữa.
Nhưng `Tuần 3`, `Tuần 4` vẫn cứ in ra đều đặn, và mỗi tuần ấy lại tìm ra ngày
vượt ngưỡng của riêng nó.

Vậy `break` vừa thoát khỏi cái gì? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
