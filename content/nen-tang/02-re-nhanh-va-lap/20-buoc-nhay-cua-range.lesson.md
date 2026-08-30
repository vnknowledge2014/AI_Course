---
id: nen-tang.re-nhanh-va-lap.buoc-nhay-cua-range
title: Bước nhảy của range
summary: Con số thứ ba trong ngoặc là khoảng cách máy cộng thêm sau mỗi lượt — đặt nó là 7 thì vòng đi thưa, đặt nó là số âm thì vòng đi lùi.
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
requires: [ctrl.range-start, ctrl.for-range, ctrl.continue, ctrl.for-each, core.list]
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
  reviewed: true
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
không có đoạn nào để cắt.

Nhưng bạn không phải đi hết hai mươi chín ngày rồi `continue` hai mươi tư lần
đâu. Nước đi gọn nhất bạn đang có là bỏ `range` sang một bên, viết thẳng năm
con số thành một danh sách:

```python title=readonly
for ngay in [1, 8, 15, 22, 29]:
    print(f"Thứ Hai ngày {ngay}")
```

```text title=readonly
Thứ Hai ngày 1
Thứ Hai ngày 8
Thứ Hai ngày 15
Thứ Hai ngày 22
Thứ Hai ngày 29
```

Năm dòng đúng ý bà chủ, ngắn hơn hẳn `continue`, và chạy được thật. Đây là nước
đi đúng, không phải nước đi tạm.

Chỗ vướng nằm ở chỗ khác: năm con số ấy do **bạn** chép tay. Tháng sau thứ Hai
rơi vào ngày khác, bạn chép lại năm con số khác. Bà chủ hỏi cả năm thì bạn chép
năm mươi hai con số. Và có những câu mà chép tay thì hết buổi: nồi nước dùng
ninh bốn tiếng, hớt bọt mười lăm phút một lần — mười sáu con số; số nhà chẵn
của cả dãy phố — năm mươi con số.

Danh sách chép tay còn giấu mất một điều. Năm con số kia không phải năm con số
bất kỳ: chúng có một luật, và luật ấy ngắn hơn cả năm con số — **cách nhau đúng
bảy**. Từ 1 bước bảy tới 8, bước bảy nữa tới 15, cứ thế. Chép tay thì mỗi tháng
chép lại; viết ra được cái luật thì viết một lần dùng cho mọi tháng.

Nghĩ tới chuyến xe buýt quen. Nó không đỗ trước mọi số nhà — nó đỗ cách bảy căn
một lần. Muốn nói trọn chuyến xe ấy, bạn cần ba điều: lên ở đâu, xuống trước số
nhà nào, và **mỗi lần đỗ cách nhau mấy căn**.

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

Y hệt năm dòng của danh sách chép tay. Nhưng để ý: trong ngoặc không có con số
nào là ngày thứ Hai cả. Ba con số ấy là **cái luật**, không phải kết quả của
luật. Đọc chúng theo thứ tự:

- `1` — điểm khởi hành, đúng vai trò nó nhận ở bài trước. Con số này **có** vào
  vòng.
- `30` — chỗ dừng, cũng đúng vai trò cũ. Con số này **không bao giờ** vào vòng.
- `7` — chỗ mới của bài hôm nay: **bước nhảy**. Đó là con số máy cộng thêm vào
  sau mỗi lượt, tức khoảng cách từ một giá trị tới giá trị kế tiếp.

Máy đi thế này: đặt `ngay` ở 1, chạy thân vòng; cộng 7 thành 8, 8 vẫn chưa tới
chỗ dừng nên chạy thân tiếp; 15, rồi 22, rồi 29. Cộng 7 lần nữa thành 36 — 36
đã qua chỗ dừng, vòng kết thúc. Số 36 không được in ra, mà số 30 cũng chưa bao
giờ xuất hiện.

Tháng sau thứ Hai rơi vào ngày 5? Sửa đúng một con số: `range(5, 30, 7)`. Cái
luật vẫn là cái luật.

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
duy nhất trong `range(3)` vẫn giữ. Vòng này quả thật chạy ba lượt, nên con số
ba của bạn không phải từ trên trời rơi xuống.

Chỗ lệch: ba lượt ấy đến từ chỗ khác. Nó là hệ quả của cả 2, cả 11 lẫn cả 3 gộp
lại — không con số nào trong ngoặc khai ra nó. Hai số đầu là hai mốc trên trục
số, số thứ ba là khoảng cách mỗi bước, còn số lượt thì không ai viết ra. Thử đổi
11 thành 12 mà xem: vẫn ba con số trong ngoặc, nhưng `range(2, 12, 3)` chạy bốn
lượt — 2, 5, 8, 11.
::
:::
::::

::::explain{#buoc-am-thi-di-lui}
Đọc lại đúng một câu vừa nói ở trên: bước nhảy là **con số máy cộng thêm sau
mỗi lượt**. Trong câu ấy không có chữ nào bắt con số đó phải dương.

Cộng thêm `-1` mỗi lượt thì giá trị đi xuống. Vòng lặp đi lùi:

```python title=readonly
for giay in range(5, 0, -1):
    print(f"Còn {giay} giây")
```

```text title=readonly
Còn 5 giây
Còn 4 giây
Còn 3 giây
Còn 2 giây
Còn 1 giây
```

Đi lại từng bước như lúc nãy: đặt `giay` ở 5, chạy thân vòng; cộng −1 thành 4,
chạy thân; rồi 3, rồi 2, rồi 1. Cộng −1 lần nữa thành 0 — mà 0 chính là chỗ
dừng, nên vòng kết thúc ở đó.

Chỗ này đáng dừng lại một nhịp, vì nó là chỗ trượt chân quen nhất của bước âm.
Muốn con số **cuối cùng in ra** là 1 thì chỗ dừng phải viết là **0**, chứ không
phải 1. Viết `range(5, 1, -1)` thì máy dừng ngay khi chạm 1, và dòng
`Còn 1 giây` không bao giờ hiện ra.

Vẫn đúng cái luật cũ thôi — chỗ dừng không bao giờ vào vòng. Chỉ có điều hướng
đi đổi nên chỗ dừng đổi bên: đi tới thì nó nằm **trên** con số cuối cùng một
bậc, đi lùi thì nó nằm **dưới** con số cuối cùng một bậc.

Hai chuyện nữa đáng cất vào túi:

- **Đi sai hướng thì vòng chạy 0 lượt.** `range(1, 10, -1)` bảo máy khởi hành ở
  1 rồi lùi dần, trong khi chỗ dừng lại nằm phía trên. Máy không kêu một tiếng
  nào, thân vòng chỉ đơn giản không chạy lần nào — đúng cảnh im lặng bạn đã gặp
  với `range(15, 8)` ở bài trước.
- **Bước 0 thì máy dừng và nói.** `range(1, 10, 0)` cho `ValueError` kèm dòng
  `range() arg 3 must not be zero`. Cũng phải thôi: bước 0 nghĩa là đứng yên
  mãi ở một chỗ, và đó là vòng lặp không bao giờ hết — thứ máy giữ bạn khỏi rơi
  vào.
::::

::::code{#hot-bot-noi-nuoc-dung}
Nồi nước dùng của quán ninh đúng **2 tiếng**, tức 120 phút. Nước vừa sôi thì bà
chủ hớt bọt lần đầu — **phút 30** — rồi cứ **20 phút** hớt một lần, tới lúc tắt
bếp ở phút 120 thì thôi.

Máy phải in ra đúng năm dòng này:

```text title=readonly
Hớt bọt: phút 30
Hớt bọt: phút 50
Hớt bọt: phút 70
Hớt bọt: phút 90
Hớt bọt: phút 110
```

Điền ba con số vào ba chỗ trống.

```python title=starter
for phut in range(___, ___, ___):
    print(f"Hớt bọt: phút {phut}")
```

```python title=solution
for phut in range(30, 120, 20):
    print(f"Hớt bọt: phút {phut}")
```

```python title=test
# Chấm bằng TRỌN VẸN màn hình theo đúng thứ tự dòng (`match: regex` ở dưới),
# không phải bằng một dòng lẻ. Lệch con số nào trong ba con số cũng lộ ra:
# lệch điểm khởi hành thì dòng đầu sai, lệch bước nhảy thì các dòng giữa sai,
# lệch chỗ dừng thì số dòng sai.
pass
```

:::hints
- kind: attention
  body: Ba chỗ trống, ba vai. Nhìn cột số trong màn hình cần in: con số đầu tiên là 30, con số cuối cùng là 110, và từ một dòng sang dòng ngay sau nó, con số nhích lên đúng bao nhiêu?
- kind: strategy
  body: Chỗ trống thứ nhất là điểm khởi hành — viết thẳng con số của dòng đầu tiên. Chỗ trống thứ ba là bước nhảy — chính là khoảng cách giữa hai dòng liền nhau. Chỗ trống thứ hai là chỗ dừng, và chỗ dừng thì không bao giờ vào vòng: đề bài đã cho sẵn con số ấy, chính là mốc tắt bếp.
- kind: one-line
  body: "Viết `range(30, 120, 20)`, giữ nguyên dấu hai chấm cuối dòng."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Hớt bọt: phút 30\nHớt bọt: phút 50\nHớt bọt: phút 70\nHớt bọt: phút 90\nHớt bọt: phút 110\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dòng nói trọn cả chuyến đi: lên ở đâu, xuống chỗ nào, mỗi bước dài bao nhiêu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bước nhảy giúp bạn đi thưa trên **một** hàng số: hàng ngày trong tháng, hàng
phút hớt bọt nồi nước dùng, hàng giây đếm ngược.

Nhưng tờ lịch dán trên tường quán không phải một hàng. Nó là **4 tuần × 7
ngày**: bốn dòng, mỗi dòng bảy ô.

Bạn viết được vòng lặp chạy qua bốn tuần — `range(1, 5)`, dễ thôi. Mỗi lượt của
nó in ra tên một tuần. Nhưng bên trong tuần ấy còn bảy ô ngày phải in nữa, và
mỗi ô lại là một lượt riêng.

Ai đứng ra in bảy ô đó? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
