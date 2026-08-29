---
id: nen-tang.re-nhanh-va-lap.break-thoat-khoi-vong-nao
title: "`break` thoát khỏi vòng nào"
summary: Trong vòng lồng, break chỉ tác động lên vòng gần nhất bao quanh nó — ra khỏi tuần, vẫn còn trong tháng.
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
requires: [ctrl.nested-loop, ctrl.break, ctrl.for-range, core.list-index, core.fstring, core.augmented-assign]
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
cứ in ra, mỗi tuần lại tìm ra ngày vượt ngưỡng của riêng nó.

Bài trước cũng đã đưa sẵn hình ảnh để trả lời: kim giờ và kim phút. Kim phút bị
giữ lại giữa chừng thì kim giờ vẫn nhích tiếp sang nấc sau như thường — nó không
nhận được tin gì về chuyện vừa xảy ra ở kim kia. Bốn tuần của bạn là kim giờ, và
nó vừa làm đúng như thế.

Bài "Dừng ngay khi đã đủ biết" gọi `break` là *gấp sổ lại*. Cách gọi ấy đúng khi
trong tay bạn chỉ có **một** cuốn sổ. Bây giờ bạn có hai lớp: cuốn lịch tháng, và
trong mỗi trang là một tuần. Gấp cái gì lại?

Đổi sang một hình ảnh chịu được hai lớp: mỗi vòng lặp là một **căn phòng**. Vòng
lồng là phòng nằm trong phòng — kim phút ở phòng trong, kim giờ ở phòng ngoài.
Bạn đang đứng ở phòng trong cùng, và `break` là cánh cửa **của căn phòng bạn
đang đứng**. Mở nó ra, bạn bước sang phòng ngoài — chứ không ra tới đường.

Nói bằng chữ của Python: `break` thoát khỏi **vòng lặp gần nhất bao quanh nó**.
Không phải mọi vòng, không phải vòng ngoài cùng. Gần nhất.

"Gần nhất" đo bằng thụt lề — thứ bạn đã dùng từ bài đầu track này. Nhưng phải
đi từng bậc một, không quét một lượt:

1. Đứng ở dòng `break`, nhìn ngược lên tới dòng đầu tiên có thụt lề **nhỏ
   hơn** nó. Đó là dòng mở **khối đang chứa** nó.
2. Nếu dòng ấy là `for` hay `while` thì xong — đó là vòng bị cắt.
3. Nếu nó là `if`, hay bất cứ khối nào khác, thì dóng theo cột **của dòng ấy**
   rồi làm lại từ bước 1.

Bước 3 là bước dễ bỏ sót nhất, và bỏ sót nó thì đọc sai. Xét đoạn này:

```python title=readonly
for tuan in range(1, 5):        # cột 0
    for ngay in range(1, 4):    # cột 4
        if xau(tuan, ngay):     # cột 8
            da_vuot = True
            break               # cột 12 — cắt vòng NGÀY
    if da_vuot:                 # cột 4
        break                   # cột 8 — cắt vòng TUẦN
```

Dòng `break` cột 8: quét một lượt lên trên thì gặp `for ngay` ở cột 4 trước,
và kết luận nó cắt vòng ngày. Sai. Đi từng bậc thì bậc đầu tiên là `if da_vuot`
ở cột 4 — một `if`, không phải vòng — nên dóng sang cột 4 rồi đi tiếp, và gặp
`for tuan` ở cột 0. Nó cắt vòng **tuần**. Chạy thử sẽ thấy `tuan` dừng đúng ở
tuần xảy ra chuyện, không chạy hết bốn tuần.
::::

::::predict{#doan-man-hinh commitOnce}
Thử luật ấy trên một đoạn nhỏ, dò được bằng mắt: 3 tuần, mỗi tuần 3 ngày.
**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

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
`ngày 3` chỉ hiện ra nếu máy quay lại đầu vòng trong rồi chạy tiếp lượt
`ngay = 3`. `break` không quay lại: nó bước hẳn ra khỏi vòng trong ngay tại chỗ
nó đứng, nên `ngay` không bao giờ tới 3. Vòng trong bị **cắt đứt**, không phải
bị bỏ qua một lượt rồi chạy tiếp.
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

::::explain{#doi-chu-bang-bon-dau-cach}
Cũng chữ `break` ấy, chỉ lùi ra bốn dấu cách, nó **đổi chủ**:

```python title=readonly
for tuan in range(1, 4):
    print(f"Tuần {tuan}")
    for ngay in range(1, 4):
        print(f"  ngày {ngay}")
    break
```

```text title=readonly
Tuần 1
  ngày 1
  ngày 2
  ngày 3
```

Bây giờ nhìn ngược lên từ dòng `break` ở cột 4: dòng `for` đầu tiên có thụt lề
nhỏ hơn nó là `for tuan`. Nên nó cắt vòng **tuần**. Vòng ngày không bị đụng tới
— tuần 1 vẫn dò trọn cả ba ngày, chỉ có tuần 2 và tuần 3 là không bao giờ tới
lượt.

Hai đoạn code vừa rồi có cùng một chữ `break`, cùng một cặp vòng lặp. Thứ duy
nhất khác nhau là **cột** mà chữ ấy đứng, và kết quả ngược hẳn nhau.
::::

::::example{#bon-tuan-van-in-du}
Đem luật ấy áp vào việc thật. Sổ chi tiêu cả tháng của Byte, 28 con số. Byte chép
sổ theo đơn vị **nghìn đồng** cho gọn mắt — số `240` trong sổ là 240 nghìn — và
ngưỡng bà chủ dặn để ý là **200 nghìn**. Mỗi hàng trong danh sách là một tuần,
cho dễ nhìn:

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

Đọc kỹ sáu dòng này, vì chúng là luật vừa học đứng trên một cuốn sổ thật:

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

::::explain{#khong-co-cach-nao-nhay-hai-tang}
Gom lại thành một câu đáng nhớ: **`break` luôn nói về cái vòng gần nhất bao
quanh nó.** Muốn nó nói về vòng ngoài, bạn phải viết nó ở trong thân vòng ngoài
— chứ không có cách nào bảo `break` "thoát hai tầng".

Nghĩa là khi đọc một dòng `break` trong vòng lồng, bạn không cần đọc cả đoạn:
chỉ cần dóng cột của nó lên trên tới dòng `for` gần nhất là biết nó đi đâu.

Có ngôn ngữ khác cho phép đặt tên cho từng vòng lặp rồi bảo `break` thoát theo
tên, nhảy ra mấy lớp một lúc. Python thì không, và đó là chọn lựa có chủ ý: một
dòng `break` bao giờ cũng chỉ cần nhìn lên vài dòng là biết nó đi đâu.
::::

::::repair{#dat-break-dung-tang}
Cuối tháng, Byte cần một con số: **bao nhiêu tuần trong tháng có ngày vượt
ngưỡng 200 nghìn**. Tuần nào có hai ba ngày vượt thì vẫn chỉ tính là một tuần.

Nghĩa là: dò từng ngày trong tuần, gặp ngày đầu tiên vượt ngưỡng thì cộng 1 vào
`so_tuan_vuot` rồi **thôi không dò nốt tuần ấy nữa** — nhưng vẫn phải dò những
tuần sau. Tuần 1, tuần 3 và tuần 4 đều có ngày vượt, tuần 2 thì không, nên màn
hình phải ra thế này:

```text title=readonly
Tuần 1 có ngày vượt ngưỡng
Tuần 3 có ngày vượt ngưỡng
Tuần 4 có ngày vượt ngưỡng
Có 3 tuần vượt ngưỡng
```

Byte đã gõ đủ mọi dòng cần có, kể cả chữ `break`. Chạy lên thì ra:

```text title=readonly
Tuần 1 có ngày vượt ngưỡng
Tuần 1 có ngày vượt ngưỡng
Có 2 tuần vượt ngưỡng
```

`Tuần 1` in hai lần rồi hết. Chữ `break` của Byte đúng, chỗ nó đứng thì không.
Dịch đúng một dòng cho ra đúng bốn dòng trên — không thêm dòng nào, không xoá
chữ nào, không đổi một con số nào.

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
            print(f"Tuần {tuan} có ngày vượt ngưỡng")
    break

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
            print(f"Tuần {tuan} có ngày vượt ngưỡng")
            break

print(f"Có {so_tuan_vuot} tuần vượt ngưỡng")
```

```python title=test
# Bốn chỗ đứng của dòng `break` cho bốn kết quả khác nhau — nên cách chấm này
# phân biệt được HIỂU ĐÚNG PHẠM VI với chỉ nhớ mặt chữ `break`:
#
#   cột 4  (thân vòng tuần, code khởi đầu) → "Tuần 1" hai lần, đếm ra 2;
#   cột 8  (thân vòng ngày, ngoài thân if) → không dòng nào, đếm ra 0;
#   cột 12 (thân if — đáp án)              → ba dòng tuần, đếm ra 3;
#   xoá hẳn dòng break                     → năm dòng tuần, đếm ra 5.
assert so_tuan_vuot == 3, "tuần 1, tuần 3 và tuần 4 đều có ngày tiêu quá 200 nghìn, tuần 2 thì không — và tuần có tới hai ngày vượt vẫn chỉ tính là một tuần"
```

:::hints
- kind: attention
  body: Đặt ngón tay lên dòng `break` rồi dóng thẳng cột của nó lên trên. Dòng `for` đầu tiên có thụt lề nhỏ hơn nó là dòng nào — `for tuan` hay `for ngay_trong_tuan`?
- kind: strategy
  body: Đọc lại hai dòng `Tuần 1` trong output hỏng. Vòng ngày vẫn dò nốt tuần 1 nên gặp cả ngày 4 lẫn ngày 7; còn vòng tuần thì thôi hẳn sau tuần 1. Đúng ngược với thứ bạn cần. Muốn cắt vòng NGÀY thì `break` phải nằm trong thân vòng ngày — và chỉ được cắt vào lúc vừa tìm thấy, tức là trong thân `if`.
- kind: one-line
  body: "Xoá dòng `break` đang đứng ở cột 4, rồi viết `break` vào ngay dưới dòng `print`, thụt vào đúng bằng dòng `print` ấy."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Tuần 1 có ngày vượt ngưỡng\nTuần 3 có ngày vượt ngưỡng\nTuần 4 có ngày vượt ngưỡng\nCó 3 tuần vượt ngưỡng$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba tuần. Cũng chữ `break` ấy, lùi vào tám dấu cách là nó đổi hẳn chủ.
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
