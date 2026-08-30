---
id: onboarding.may-tinh-noi-gi.loi-truoc-khi-chay
title: Lỗi máy thấy trước khi chạy
summary: Thiếu một dấu nháy hay một dấu ngoặc thì máy không chạy dòng nào cả, kể cả dòng viết đúng.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [err.syntax-error]
requires: [err.traceback, core.string-literal]
concepts: [core.thong-bao-loi, core.cu-phap]
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
Có loại lỗi mình thấy ngay lúc đọc, trước khi bắt tay làm việc gì.
::::

::::explain{#doc-truoc-khi-lam}
Bài 2 có tờ giấy dặn việc dán trong bếp quán phở. Người phụ bếp mới đến cầm tờ
giấy lên, đọc lướt một lượt từ đầu đến cuối **trước khi** động tay vào bất cứ
việc gì.

Giả sử tờ giấy viết:

> 1. Bắc nồi nước lên bếp.
> 2. Cho vào nồi hai
> 3. Đun nhỏ lửa ba tiếng.

Dòng 2 đứt giữa chừng. Hai cái gì? Hai củ hành? Hai lít nước? Hai thìa muối?

Người phụ bếp lúc này làm gì? Không phải bắc nồi lên rồi tới dòng 2 mới thắc
mắc. Người ấy đặt tờ giấy xuống và đi hỏi ngay — dù dòng 1 viết đầy đủ, làm
được, và chẳng liên quan gì tới chỗ đứt.

Máy tính cư xử y hệt. Trước khi chạy dòng đầu tiên, nó đọc hết chương trình một
lượt để xem có câu nào nó không đọc nổi hay không. Việc đọc trước ấy dẫn tới
một hệ quả — và hệ quả đó là thứ bạn sắp đoán.
::::

::::predict{#doan-dong-mot-co-chay-khong commitOnce}
Đoạn dưới có hai dòng. Dòng 1 viết đúng hoàn toàn, y như bài 1 đã dạy. Dòng 2
thiếu dấu nháy đóng — dấu nháy mở ra trước chữ `Mời` mà không có dấu nào đóng
lại.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print("Quán mở cửa")
print("Mời vào)
```

:::opt{correct}
Chỉ có thông báo lỗi. Chữ "Quán mở cửa" không hiện ra
:::

:::opt
Hiện "Quán mở cửa" trước, rồi mới tới thông báo lỗi
::why
Gần đúng ở chỗ bạn áp dụng đúng hai điều đã học: máy chạy lần lượt từ trên
xuống, và dòng 1 viết chuẩn không có gì phải sửa. Với hai bài lỗi vừa rồi, suy
luận này cho ra kết quả chính xác — ở đó máy thật sự chạy xong dòng 1 và dòng 2
rồi mới vấp ở dòng 3.

Chỗ lệch nằm ở **thời điểm** máy phát hiện. `TypeError` chỉ lộ ra khi máy đang
chạy và vừa đặt chân tới đúng dòng đó. Lỗi lần này lộ ra sớm hơn nhiều: lúc máy
mới đọc qua, chưa chạy gì cả. Đã dừng ngay từ lúc đọc thì dòng 1 dù đúng cũng
chưa tới lượt.
::
:::

:::opt
Máy tự thêm dấu nháy còn thiếu vào rồi chạy cả hai dòng
::why
Gần đúng ở chỗ chỗ thiếu nằm ngay trước mắt, và một người đọc hộ thì đoán ra
ngay bạn định viết gì. Bạn đang trông đợi máy cư xử như một người đọc biết
thông cảm — mong muốn ấy rất tự nhiên.

Chỗ lệch quay lại tính cách ở bài 3: máy không tự bổ sung phần bạn quên nói.
Với dòng `print("Mời vào)`, nó không có cách nào biết câu chữ của bạn kết thúc
ở đâu — sau chữ `vào`? hay sau dấu `)`, tức là dấu ngoặc ấy cũng nằm trong câu
chữ luôn? Nó dừng lại hỏi thay vì chọn hộ bạn.

(Máy biết chắc một điều: câu chữ ấy phải đóng **trước khi hết dòng**. Nên khi
đọc tới cuối dòng mà chưa thấy dấu nháy thứ hai, nó không đợi thêm dòng nào
nữa — nó báo ngay. Đó chính là chữ *detected at line 1* trong thông báo.)
::
:::

:::opt
Cả hai dòng in ra bình thường, thiếu một dấu nháy thì đã sao
::why
Gần đúng ở chỗ với mắt người, `print("Mời vào)` vẫn đọc ra được ý — mắt bạn tự
vá chỗ thiếu mà chính bạn không để ý. Đó cũng là lý do loại lỗi này khó tự tìm
ra nhất khi mới học.

Chỗ lệch: dấu nháy không phải trang trí, nó là **ranh giới** (bài 1). Thiếu dấu
đóng thì câu chữ không có điểm kết thúc, và máy không ráp nổi dòng đó thành một
câu lệnh hoàn chỉnh.
::
:::
::::

::::example{#thieu-mot-dau-nhay}
Đây là thứ máy in ra cho đoạn code vừa rồi:

```text
  File "quan_pho.py", line 2
    print("Mời vào)
          ^
SyntaxError: unterminated string literal (detected at line 2)
```

Vẫn đọc từ dưới lên, đúng thói quen bài trước:

- **Dòng dưới cùng.** `SyntaxError` ghép từ *syntax* (cú pháp, tức là cách viết) và *error* (lỗi). Dịch sát: **lỗi cú pháp** — câu này viết sai cách nên máy không đọc được thành lệnh. Câu lý do `unterminated string literal` nghĩa là "một câu chữ mở ra mà chưa đóng lại".
- **Dòng `line 2`.** Chỗ máy đọc không nổi. Máy chép lại nguyên văn dòng đó, và dấu `^` chỉ thẳng vào cái dấu nháy mở đơn độc.

Nhưng chỗ đáng chú ý nhất lại là **thứ vắng mặt**. So với bài trước, thông báo
này thiếu hai thứ:

- Không có dòng `Traceback (most recent call last):` mở đầu.
- Trên màn hình không hề có chữ `Quán mở cửa`.

Hai điều đó thật ra là một điều. Không có đoạn đường nào để kể lại, bởi vì máy
chưa đi bước nào. Dòng 1 viết đúng, chạy được, nhưng nó không được chạy.
::::

::::explain{#hai-loai-loi-canh-nhau}
Bạn vừa có đủ hai loại lỗi để đặt cạnh nhau:

| | `TypeError` (bài 16, 17) | `SyntaxError` (bài này) |
|---|---|---|
| Máy phát hiện lúc nào | đang chạy, vừa tới đúng dòng đó | lúc đọc, trước khi chạy dòng nào |
| Các dòng phía trên đã chạy chưa | chạy xong rồi | chưa dòng nào chạy |
| Có dòng `Traceback` mở đầu không | có | không |
| Thường do đâu | hai giá trị mang hai kiểu khác nhau | thiếu một dấu: nháy, ngoặc, hai chấm |

Nhìn cột bên phải, nghe như tin xấu: sai một dấu mà cả chương trình đứng im.
Nhưng đổi góc nhìn một chút thì đây là loại lỗi dễ chịu nhất trong nghề. Máy
chỉ đích danh dòng, đặt mũi `^` vào gần đúng chỗ, và nói tên thứ còn thiếu.
Không có gì phải suy đoán.

Loại lỗi khó là loại chương trình vẫn chạy trơn tru mà kết quả sai. Loại đó bạn
sẽ gặp về sau — và lúc đó bạn sẽ thấy nhớ những dòng `SyntaxError` thẳng thắn
này.
::::

::::code{#them-dau-con-thieu}
Byte muốn in ra câu **Chúc bạn ngon miệng**, và gõ thiếu đúng một dấu. Máy trả
lời:

```text
  File "quan_pho.py", line 1
    print("Chúc bạn ngon miệng"
         ^
SyntaxError: '(' was never closed
```

Đọc dòng dưới cùng: `'(' was never closed` — "dấu `(` mở ra mà không bao giờ
được đóng lại". Hãy điền đúng một ký tự vào chỗ trống để câu lệnh đọc được.

```python title=starter
print("Chúc bạn ngon miệng"___
```

```python title=solution
print("Chúc bạn ngon miệng")
```

```python title=test
# Chấm bằng OUTPUT: người học chưa biết hàm nên chưa assert được gì.
# Khối này chỉ khẳng định chương trình chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Trong dòng này có hai loại dấu đi theo cặp. Dấu nháy kép đã đủ đôi — mở trước chữ `Chúc`, đóng sau chữ `miệng`. Còn loại dấu kia thì sao?
- kind: strategy
  body: Dòng dưới cùng của thông báo gọi thẳng tên thủ phạm là dấu `(`. Mỗi dấu `(` cần một dấu cùng cặp đi kèm để đóng lại phần nó bọc.
- kind: one-line
  body: "Viết dấu `)` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Chúc bạn ngon miệng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dấu ngoặc thôi mà cả chương trình đứng im. Giờ thì nó nói được rồi.
::::

::::reflect{#nghi-lai}
Nhìn lại mười tám bài vừa qua: chương trình của bạn nói khá giỏi. Nó nói ra một
câu, nó nối hai câu, nó tính tiền, và khi có gì không ổn thì nó báo cho bạn
biết bằng những dòng bạn đã đọc được.

Nhưng nó nói một mình. Bạn viết sẵn `20000` vào trong code thì nó cộng `20000`;
muốn đổi thành `35000` thì bạn phải mở file ra sửa rồi chạy lại. Người ngồi
trước màn hình không góp được một chữ nào vào cuộc trò chuyện.

Vậy câu hỏi cho bài sau: làm sao để chương trình **hỏi** bạn một câu, rồi
**dừng lại chờ** bạn gõ câu trả lời — như hai người nói chuyện, chứ không phải
một người đọc diễn văn?

Máy vốn giỏi nhất chuyện chờ (bài 3). Bài sau bạn sẽ bảo nó chờ đúng lúc.
::::

::::checkpoint{mastery=0.8}
::::
