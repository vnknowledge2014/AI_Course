---
id: nen-tang.re-nhanh-va-lap.break-thoat-khoi-vong-nao
title: "`break` thoát khỏi vòng nào"
summary: Trong vòng lồng, break và continue chỉ tác động lên vòng gần nhất bao quanh chúng — ra khỏi tuần, vẫn còn trong tháng.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ctrl.break-scope]
requires: [ctrl.nested-loop, ctrl.break, ctrl.continue, ctrl.for-range]
concepts: [ctrl.lap, ctrl.pham-vi-vong]
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
Mình bước ra khỏi tuần rồi. Nhưng mình vẫn còn đứng trong tháng.
::::

::::explain{#ra-khoi-tuan-van-con-trong-thang}
Bài trước để lại một câu hỏi khó chịu. Bạn xếp lịch tháng thành 4 tuần × 7 ngày,
đặt `break` vào chỗ gặp ngày vượt ngưỡng. Nó dừng thật — mà những tuần sau vẫn
cứ in ra.

Bài "Dừng ngay khi đã đủ biết" gọi `break` là *gấp sổ lại*. Cách gọi ấy đúng khi
trong tay bạn chỉ có **một** cuốn sổ. Bây giờ bạn có hai lớp: cuốn lịch tháng, và
trong mỗi trang là một tuần. Gấp cái gì lại?

Hình dung khác cho dễ: mỗi vòng lặp là một **căn phòng**. Vòng lồng là phòng nằm
trong phòng. Bạn đang đứng ở phòng trong cùng, và `break` là cánh cửa **của căn
phòng bạn đang đứng**. Mở nó ra, bạn bước sang phòng ngoài — chứ không ra tới
đường.

Nói bằng chữ của Python: `break` thoát khỏi **vòng lặp gần nhất bao quanh nó**.
Không phải mọi vòng, không phải vòng ngoài cùng. Gần nhất.

"Gần nhất" đo bằng thụt lề — thứ bạn đã dùng từ bài đầu track này. Đứng ở dòng
`break`, nhìn ngược lên trên: dòng `for` hay `while` đầu tiên có mức thụt lề
**nhỏ hơn** nó chính là cái vòng nó sẽ thoát ra.
::::

::::example{#bon-tuan-van-in-du}
Sổ chi tiêu cả tháng của Byte, 28 con số, tính bằng nghìn đồng. Mỗi hàng trong
danh sách là một tuần, cho dễ nhìn:

```python title=readonly
chi_tieu = [80, 120, 95, 240, 60, 110, 210,
            70, 90, 100, 85, 140, 75, 180,
            60, 95, 260, 120, 100, 70, 130,
            90, 110, 100, 230, 150, 95, 220]

for tuan in range(1, 5):
    print(f"Tuần {tuan}:")
    for ngay_trong_tuan in range(1, 8):
        ngay = (tuan - 1) * 7 + ngay_trong_tuan
        tien = chi_tieu[ngay - 1]
        if tien > 200:
            print(f"  Ngày {ngay} tiêu {tien} nghìn — vượt ngưỡng")
            break
```

Hai dòng đầu trong thân vòng ngày chỉ là việc tra sổ, không phải phần mới của
bài:

- `ngay = (tuan - 1) * 7 + ngay_trong_tuan` đổi *tuần mấy, ngày thứ mấy trong
  tuần* thành *ngày thứ mấy trong tháng*. Tuần 2, ngày thứ 3 trong tuần là ngày
  10 của tháng.
- `tien = chi_tieu[ngay - 1]` lấy con số của ngày đó ra khỏi sổ. Trừ 1 vì ô đầu
  tiên của một danh sách mang chỗ đứng 0.

Chạy lên, màn hình hiện ra:

```text title=readonly
Tuần 1:
  Ngày 4 tiêu 240 nghìn — vượt ngưỡng
Tuần 2:
Tuần 3:
  Ngày 17 tiêu 260 nghìn — vượt ngưỡng
Tuần 4:
  Ngày 25 tiêu 230 nghìn — vượt ngưỡng
```

Đọc kỹ sáu dòng này, vì chúng nói hết bài hôm nay:

- `break` **có** chạy, ở tuần 1, ngay ngày 4. Tuần 1 có tới hai ngày vượt ngưỡng
  — ngày 4 và ngày 7 — mà chỉ ngày 4 được in. Vòng trong đã bị cắt.
- Rồi dòng `Tuần 2:` vẫn hiện ra. Vòng **ngoài** chưa hề bị đụng tới: nó chỉ
  thấy lượt của tuần 1 đã xong, và nó sang lượt tiếp theo như mọi lượt bình
  thường.
- `Tuần 2:` đứng một mình, không có dòng nào dưới nó — tuần đó không ngày nào
  vượt 200, nên vòng trong chạy trọn 7 lượt mà `break` không tới lượt.

Nếu bạn muốn đúng cái này — *mỗi tuần một dòng báo cáo, gặp ngày đầu tiên là
thôi* — thì đoạn code trên đã hoàn hảo. Còn nếu bạn muốn cả tháng chỉ dừng một
lần rồi thôi hẳn, thì `break` này chưa đủ.
::::

::::predict{#doan-man-hinh commitOnce}
Đoạn dưới nhỏ hơn, để bạn dò được bằng mắt: 3 tuần, mỗi tuần 3 ngày. **Trước khi
bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python title=readonly
for tuan in range(1, 4):
    print(f"Tuần {tuan}")
    for ngay in range(1, 4):
        if ngay == 2:
            break
        print(f"  ngày {ngay}")

print("Xong")
```

:::opt{correct}
Tuần 1, ngày 1, Tuần 2, ngày 1, Tuần 3, ngày 1, rồi Xong
:::

:::opt
Tuần 1, ngày 1, rồi Xong
::why
Gần đúng ở chỗ bạn đọc `break` rất chuẩn cho tới thời điểm nó chạy: tuần 1 in
được `ngày 1`, tới `ngay == 2` thì máy cắt ngang. Và bạn nhớ đúng rằng dòng sát
lề trái vẫn chạy sau khi vòng lặp kết thúc.

Chỗ lệch là số vòng lặp mà một chữ `break` cắt được. Nó cắt **một** vòng — vòng
gần nhất bao quanh nó, ở đây là `for ngay`. Sau khi cắt, máy đứng ở thân của
`for tuan`, không còn dòng nào bên dưới, nên nó làm việc mà mọi vòng lặp vẫn làm
khi hết một lượt: sang lượt kế. `Tuần 2` in ra từ đó.
::
:::

:::opt
Tuần 1, ngày 1, ngày 3, Tuần 2, ngày 1, ngày 3, Tuần 3, ngày 1, ngày 3, rồi Xong
::why
Gần đúng ở chỗ bạn thấy đúng chuyện vòng ngoài vẫn chạy trọn ba tuần — đó là
nửa quan trọng của bài này, và bạn đã nắm được.

Chỗ lệch nằm ở việc `break` làm gì với **phần còn lại của vòng trong**. Dòng
in ra `ngày 3` chỉ xuất hiện nếu máy quay lại đầu vòng trong và chạy tiếp lượt
`ngay = 3`. Đó đúng là việc của `continue` — bỏ lượt này, đi tiếp lượt sau.
`break` thì không quay lại: nó rời hẳn vòng trong, nên `ngay` không bao giờ tới 3.
::
:::

:::opt
Tuần 1, ngày 1, Tuần 2, ngày 1, Tuần 3, ngày 1 — rồi hết, không có Xong
::why
Gần đúng ở toàn bộ phần bên trong hai vòng lặp: bạn đếm chính xác ba tuần, mỗi
tuần đúng một dòng `ngày 1`. Phần khó nhất của bài, bạn đã đọc đúng.

Chỗ lệch nằm ở chữ "hết". `break` kết thúc một vòng lặp, không kết thúc chương
trình. Sau khi `for tuan` chạy hết ba lượt, máy đi tiếp từ trên xuống như mọi
bài trước, và dòng đầu tiên nó gặp là `print("Xong")`.
::
:::
::::

::::explain{#continue-cung-mot-luat}
`continue` theo đúng luật ấy, không khác một chữ: nó nhảy lên đầu lượt kế của
**vòng gần nhất bao quanh nó**.

```python title=readonly
for tuan in range(1, 3):
    print(f"Tuần {tuan}")
    for ngay in range(1, 4):
        if ngay == 2:
            continue
        print(f"  ngày {ngay}")
```

```text title=readonly
Tuần 1
  ngày 1
  ngày 3
Tuần 2
  ngày 1
  ngày 3
```

`continue` ở đây bỏ qua ngày 2 của **vòng ngày**, không bỏ qua tuần nào cả. Muốn
bỏ qua cả một tuần thì `continue` phải nằm trong thân vòng tuần, ở mức thụt lề
của vòng tuần.

Gom lại thành một câu đáng nhớ: **`break` và `continue` luôn nói về cái vòng gần
nhất bao quanh chúng.** Muốn chúng nói về vòng ngoài, bạn phải viết chúng ở
trong thân vòng ngoài — chứ không có cách nào bảo `break` "thoát hai tầng".

Có ngôn ngữ khác cho phép đánh số tầng để `break` nhảy ra mấy lớp một lúc.
Python thì không, và đó là chọn lựa có chủ ý: một dòng `break` bao giờ cũng chỉ
cần nhìn lên vài dòng là biết nó đi đâu.
::::

::::code{#dem-so-tuan-vuot}
Cuối tháng, Byte cần một con số: **bao nhiêu tuần trong tháng có ngày vượt
ngưỡng 200 nghìn**. Tuần nào có hai ba ngày vượt thì vẫn chỉ tính là một tuần.

Nghĩa là: dò từng ngày trong tuần, gặp ngày đầu tiên vượt ngưỡng thì cộng 1 vào
`so_tuan_vuot` rồi **thôi không dò nốt tuần ấy nữa** — nhưng vẫn phải dò những
tuần sau.

Hãy điền vào chỗ trống dòng làm việc "thôi không dò nốt tuần này".

```python title=starter
chi_tieu = [80, 120, 95, 240, 60, 110, 210,
            70, 90, 100, 85, 140, 75, 180,
            60, 95, 260, 120, 100, 70, 130,
            90, 110, 100, 230, 150, 95, 220]
so_tuan_vuot = 0

for tuan in range(1, 5):
    for ngay_trong_tuan in range(1, 8):
        ngay = (tuan - 1) * 7 + ngay_trong_tuan
        if chi_tieu[ngay - 1] > 200:
            so_tuan_vuot += 1
            ___

print(f"Có {so_tuan_vuot} tuần vượt ngưỡng")
```

```python title=solution
chi_tieu = [80, 120, 95, 240, 60, 110, 210,
            70, 90, 100, 85, 140, 75, 180,
            60, 95, 260, 120, 100, 70, 130,
            90, 110, 100, 230, 150, 95, 220]
so_tuan_vuot = 0

for tuan in range(1, 5):
    for ngay_trong_tuan in range(1, 8):
        ngay = (tuan - 1) * 7 + ngay_trong_tuan
        if chi_tieu[ngay - 1] > 200:
            so_tuan_vuot += 1
            break

print(f"Có {so_tuan_vuot} tuần vượt ngưỡng")
```

```python title=test
# Tuần 1, 3, 4 có ngày vượt ngưỡng; tuần 2 không. Đáp số đúng là 3.
# Thiếu dòng cắt ngang thì tuần 1 và tuần 4 mỗi tuần bị đếm hai lần, ra 5.
assert so_tuan_vuot == 3
```

:::hints
- kind: attention
  body: Chỗ trống nằm trong thân `if`, cùng mức thụt lề với dòng `so_tuan_vuot += 1`. Nhìn ngược lên trên từ chỗ đó — dòng `for` đầu tiên có thụt lề nhỏ hơn là dòng nào?
- kind: strategy
  body: Bạn cần cắt ngang vòng dò ngày, chứ không cắt vòng dò tuần. May là dòng bạn viết nằm trong thân vòng ngày, mà lệnh cắt ngang thì luôn nói về vòng gần nhất bao quanh nó — nên viết đúng một từ khoá quen thuộc là đủ.
- kind: one-line
  body: "Viết `break` vào chỗ trống, thụt vào đúng bằng dòng `so_tuan_vuot += 1` ngay trên nó."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Có 3 tuần vượt ngưỡng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba tuần. Mỗi tuần mình chỉ dò tới ngày đầu tiên vượt ngưỡng rồi sang tuần sau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Quay lại việc bạn thật sự muốn làm ở đầu bài: dò cả tháng, gặp ngày vượt ngưỡng
**đầu tiên** là dừng hẳn, rồi in ra một dòng kết luận.

Bây giờ bạn biết vì sao nó chưa chạy được. `break` trong vòng ngày cắt xong vòng
ngày là hết nhiệm vụ. Vòng tuần bên ngoài không nhận được tin gì cả — với nó,
tuần vừa rồi chỉ là một tuần đã dò xong. Những dòng nằm sau **cả hai** vòng lại
càng không biết chuyện gì đã xảy ra bên trong.

Vòng trong đã biết là tìm thấy. Vòng ngoài thì không. Vậy làm sao báo cho vòng
ngoài — và cho những dòng nằm sau cả hai vòng — biết rằng chuyện đó đã xảy ra?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
