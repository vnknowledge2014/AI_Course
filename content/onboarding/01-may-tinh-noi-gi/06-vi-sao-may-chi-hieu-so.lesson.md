---
id: onboarding.may-tinh-noi-gi.vi-sao-may-chi-hieu-so
title: Vì sao máy chỉ hiểu số
summary: Chữ nằm được trong máy nhờ một bảng quy ước — mỗi ký tự được gán một con số.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 6
tier: A
languages: [text]
defaultLanguage: text
level: intro
estimatedMinutes: 10
teaches: [core.bang-ma, core.ky-tu-la-so]
requires: [core.byte]
concepts: [core.bang-ma]
gradingMatrix:
  web-chrome: []
  web-firefox: []
  macos: []
  windows: []
  linux: []
  android: []
  ios: []
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Mình không có ô nào chứa nổi chữ P. Mình vẫn đọc được chữ — nhờ một cuốn sổ tra.
::::

::::explain{#cho-nao-cho-chu}
Bài trước để lại một chỗ hở.

Bạn đã biết: ô nhỏ nhất trong máy chỉ mang được **có** hoặc **không**, tức là
1 hoặc 0. Tám ô đứng cạnh nhau là một byte, và một byte đếm được 256 kiểu bật
tắt khác nhau.

Bây giờ nhìn vào chữ **Phở**. Nó gồm ba ký tự: `P`, `h`, `ở`.

Không ký tự nào trong ba ký tự đó là 1. Cũng không ký tự nào là 0.

Vậy chúng nằm ở đâu trong một cái máy chỉ có ô 0 và 1?

Câu trả lời có một chỗ bất ngờ: nó **không nằm trong máy**. Nó nằm trong một
thoả thuận giữa người với người.
::::

::::example{#thuc-don-danh-so}
Quán cơm bình dân đầu ngõ dán một tờ thực đơn:

```text title=readonly
1   Cơm sườn
2   Cơm gà
3   Cơm tấm bì chả
4   Canh chua
5   Trà đá
```

Khách ngồi xuống, gọi với vào bếp: *"Cho một suất số 3."*

Người bếp nghe đúng một tiếng — **ba** — rồi làm ra một đĩa cơm tấm bì chả.

Hãy để ý kỹ chuyện vừa xảy ra. Con số 3 không có mùi cơm tấm. Nó không mang
trong mình một hạt cơm nào. Nó chỉ là một tiếng.

Nó có nghĩa vì **hai bên cùng nhìn vào một tờ thực đơn**. Bỏ tờ giấy đó đi, tiếng
"ba" lập tức không còn là món ăn nào cả.

Một tờ giấy như thế — hai bên cùng đồng ý, dùng để đổi giữa *con số* và *thứ mà
con số ám chỉ* — gọi là một **bảng quy ước**.
::::

::::explain{#bang-cua-may}
Máy tính dùng đúng mẹo của quán cơm, chỉ khác là bảng của nó dành cho **ký tự**.

Người ta ngồi lại với nhau và lập một bảng: mỗi ký tự được gán một con số. Trích
ra vài dòng cho bạn xem:

| Ký tự | Con số |
|---|---|
| `A` | 65 |
| `B` | 66 |
| `a` | 97 |
| khoảng trắng | 32 |
| `?` | 63 |

Bảng đầu tiên loại này tên là **ASCII** (đọc là "át-ki"). Nó có 128 dòng — vừa
đủ cho chữ cái tiếng Anh, mười chữ số và mấy dấu câu.

Nên khi màn hình hiện ra chữ `AB`, thứ thật sự nằm trong máy là hai con số:
65 rồi 66. Mà mỗi con số ấy, đúng như bài trước, lại nằm dưới dạng tám ô bật
tắt.

Ba tầng, xếp chồng lên nhau:

- Bạn nhìn thấy: `A`
- Máy lưu con số: 65
- Con số đó trong các ô: bật tắt tám lần

Tầng giữa là tầng mới của bài hôm nay. Nó là cái bản lề nối chữ với ô.
::::

::::predict{#doan-so-97 commitOnce}
Một con số 97 vừa đi tới máy của bạn. Máy tra bảng ở trên rồi hiện lên màn hình.

**Trước khi xem đáp án**, bạn đoán màn hình hiện ra ký tự gì?

:::opt{correct}
Chữ `a` thường
:::

:::opt
Chữ `A` hoa — hoa hay thường thì cũng là chữ A
::why
Gần đúng ở một chỗ quan trọng: với mắt người, `A` và `a` đúng là cùng một chữ
cái. Đọc lên cùng một tiếng, tra từ điển cùng một mục.

Chỗ lệch nằm ở chữ *ký tự*. Bảng quy ước không gán số cho "chữ cái" mà cho từng
**ký tự** — từng hình dạng riêng biệt. `A` và `a` là hai hình khác nhau, nên
chúng chiếm hai dòng khác nhau trong bảng: 65 và 97.

Điều này sẽ quay lại với bạn nhiều lần trong khoá học. Với máy, `Pho` và `pho`
là hai thứ khác nhau, vì con số đầu tiên của chúng đã khác nhau rồi.
::
:::

:::opt
Không hiện gì cả, vì 97 không có trong bảng
::why
Gần đúng ở chỗ bạn quay lại kiểm tra cái bảng trước khi trả lời — đó đúng là
việc nên làm, và bạn sẽ làm nó suốt khoá học này.

Chỗ lệch chỉ là do bảng in ở trên bị cắt ngắn. Byte trích ra năm dòng cho dễ
nhìn, còn bảng thật có 128 dòng liền mạch, và 97 nằm gọn trong đó. Dòng 97 ghi:
`a`.
::
:::
::::

::::explain{#128-dong-la-it}
128 dòng đủ cho tiếng Anh. Tiếng Việt thì không.

Đếm thử riêng chữ o thôi: `o`, `ó`, `ò`, `ỏ`, `õ`, `ọ`, `ô`, `ố`, `ồ`, `ổ`,
`ỗ`, `ộ`, `ơ`, `ớ`, `ờ`, `ở`, `ỡ`, `ợ`. Mười tám ký tự, và mới chỉ một chữ cái.

Nên người ta làm một bảng lớn hơn hẳn, tên là **Unicode**. Bảng này có hơn một
trăm nghìn dòng — đủ chỗ cho chữ Việt, chữ Nhật, chữ Ả Rập, và cả những hình
mặt cười bạn gửi trong tin nhắn.

Trong bảng đó, chữ `ở` nằm ở dòng số 7903.

Bạn có nhớ một byte đếm được tới đâu không? 256. Con số 7903 không nhét vừa một
byte. Vì vậy một ký tự tiếng Việt phải nằm trên **nhiều byte** đứng cạnh nhau —
đó là lý do một file chữ tiếng Việt nặng hơn một file chữ tiếng Anh cùng số ký
tự.
::::

::::predict{#hai-ban-hai-bang commitOnce}
Chắc bạn từng gặp cảnh này: mở một trang web cũ hoặc một tin nhắn, và chữ hiện
ra thành `PhÃ¡Â»Ÿ` hay `Ph?? b??` thay vì `Phở bò`.

Theo những gì bạn vừa đọc, chuyện gì đã xảy ra?

:::opt{correct}
Bên gửi ghi số theo một bảng, bên nhận lại tra số theo một bảng khác.
:::

:::opt
File bị hỏng trên đường truyền, mất mấy ký tự nên chữ mới loạn.
::why
Gần đúng ở chỗ hỏng trên đường truyền là chuyện có thật, và nó gây ra chữ loạn
thật.

Chỗ lệch nằm ở **kiểu** loạn. Nếu hỏng trên đường, các con số sẽ sai lung tung
và không theo quy luật nào: chỗ mất hẳn, chỗ ra ký tự vô nghĩa, mỗi lần mở lại
một khác. Còn ở đây, `Phở bò` lần nào cũng ra đúng cái `PhÃ¡Â»Ÿ` ấy — đều đặn
tới mức có thể đoán trước.

Đều đặn nghĩa là các con số vẫn về tới nơi nguyên vẹn. Chỉ có cái bảng đem ra
tra là khác.
::
:::

:::opt
Máy bên nhận không cài tiếng Việt nên nó không viết được chữ Việt.
::why
Gần đúng, và đây là cách hầu hết mọi người mô tả chuyện này — kể cả dân trong
nghề, khi nói nhanh.

Chỗ lệch là ở chữ "biết tiếng". Máy không biết tiếng nào cả, kể cả tiếng Anh.
Nó tra bảng, chấm hết. Câu "máy không hỗ trợ tiếng Việt" thật ra là nói tắt của
"máy đang tra một cái bảng không có dòng nào dành cho chữ Việt".

Phân biệt được hai cách nói đó là một bước tiến: cách sau chỉ thẳng ra chỗ phải
sửa, cách trước thì không.
::
:::
::::

::::explain{#chot-lai}
Chốt lại thành một câu đáng nhớ:

> Con số nằm trong máy. Nghĩa nằm trong bảng.

Câu "máy tính chỉ hiểu số" hay bị nghe thành "máy tính dốt chữ". Nghĩa thật của
nó nhẹ nhàng hơn nhiều: mọi thứ bạn thấy trên màn hình — một câu chữ, một tấm
ảnh, một bài hát — bên trong đều là số, cộng thêm một bảng quy ước để biết phải
đọc dãy số ấy theo lối nào.

Ở bài về file và thư mục, Byte có để lại cho bạn một câu hỏi: bài hát, tấm ảnh
và một câu chữ giống nhau ở điểm nào khi nằm trong máy? Bây giờ bạn trả lời
được: chúng giống nhau ở chỗ **đều là số**, và khác nhau ở chỗ **dùng bảng nào
để đọc dãy số ấy**.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nếu mọi thứ trong máy đều là số, thì lúc bạn muốn nói một câu cho máy — bạn nói
**ở đâu**?

Màn hình trước mặt bạn đầy cửa sổ, nút bấm, hình vẽ. Nhưng chỗ nào là chỗ bạn gõ
thẳng một dòng cho máy, và nó trả lời lại bạn ngay dòng dưới?

Đừng trả lời vội. Bài sau mở đúng cái cửa sổ đó ra cho bạn nhìn.
::::

::::checkpoint{mastery=0.8}
::::
