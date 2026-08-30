---
id: onboarding.may-tinh-noi-gi.terminal-la-gi
title: Terminal — chỗ nói chuyện với máy
summary: Một cửa sổ chỉ toàn chữ. Bạn gõ một dòng, máy đáp một dòng, luân phiên nhau.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 7
tier: A
languages: [text]
defaultLanguage: text
level: intro
estimatedMinutes: 9
teaches: [core.terminal, core.dau-nhac]
requires: [core.bang-ma]
concepts: [core.terminal]
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
Có một cửa sổ chỉ toàn chữ. Bạn nói một dòng, mình đáp một dòng. Cứ thế luân phiên.
::::

::::explain{#o-cua-ban-ve}
Ga Hà Nội, quầy bán vé. Một ô cửa kính nhỏ, vừa đủ đẩy tờ tiền qua.

Bạn ghé sát vào và nói: *"Cho một vé đi Hải Phòng chuyến sáng mai."*

Người bên trong nghe hết câu, rồi đáp: *"Một trăm tám mươi nghìn."*

Rồi im. Chờ.

Ba chi tiết của cái ô cửa ấy đáng để bạn giữ lại, vì bài hôm nay chỉ nói về
đúng ba chi tiết đó:

- **Mỗi lần một lượt.** Bạn nói xong thì tới người kia, người kia nói xong thì tới bạn. Không ai chen ngang.
- **Phải dứt câu thì bên kia mới trả lời.** Nói được nửa câu rồi đứng nghĩ, ô cửa vẫn im — người bên trong đang đợi bạn nói hết.
- **Trả lời xong là quay lại chờ.** Người bán vé không tự nghĩ ra chuyện gì để nói tiếp. Họ đợi câu sau của bạn.

Máy tính có đúng một cái ô cửa như thế. Nó tên là **terminal**.
::::

::::example{#cua-so-toan-chu}
Terminal là một cửa sổ trên máy bạn, và nó trông nghèo nàn tới mức đáng ngạc
nhiên: nền một màu, chữ một cỡ, không nút bấm, không hình vẽ, không thanh cuộn
màu mè.

Mở nó ra, bạn thấy một dòng ngắn rồi một vạch nhỏ nhấp nháy:

```text title=readonly
mai@may-cua-mai ~ %
```

Dòng ngắn đó gọi là **dấu nhắc**. Nó nói với bạn đúng một chuyện:

> Tôi đang chờ. Tới lượt bạn.

Dấu nhắc trên mỗi máy trông một kiểu: máy này là `%`, máy kia là `$`, máy chạy
Windows thì là một dòng dài kết thúc bằng `>`. Hình dạng khác nhau, ý nghĩa
giống hệt nhau — *đang chờ lượt của bạn*.

Bạn gõ một dòng. Máy chưa động đậy. Bạn bấm phím **Enter** — đó là cách
bạn nói "tôi dứt câu rồi". Lúc ấy máy mới đọc và mới trả lời.
::::

::::example{#mot-luot-day-du}
Đây là một lượt trò chuyện đầy đủ. Bạn đọc từ trên xuống, đúng thứ tự thời gian:

```text title=readonly
mai@may-cua-mai ~ % python3
Python 3.12.2
>>> print("Xin chào")
Xin chào
>>>
```

Năm dòng này không cùng một loại. Có dòng do bạn gõ, có dòng do máy in ra, và
chúng nằm chen nhau trong cùng một cột chữ — chỗ này khiến rất nhiều người mới
đọc nhầm.

Có một mẹo đọc, dùng được mãi về sau:

- Dòng **có dấu nhắc ở đầu** (`%` hoặc `>>>`): phần sau dấu nhắc là chữ **bạn** gõ.
- Dòng **không có dấu nhắc**: đó là máy đang trả lời.

Chiếu mẹo đó vào năm dòng trên:

| Dòng | Ai làm | Chuyện gì xảy ra |
|---|---|---|
| `... % python3` | bạn gõ | mời Python vào ngồi trong cửa sổ này |
| `Python 3.12.2` | máy đáp | Python chào, báo phiên bản của nó |
| `>>> print("Xin chào")` | bạn gõ | câu lệnh của bài 1, gõ thẳng vào đây |
| `Xin chào` | máy đáp | kết quả |
| `>>>` | máy đáp | dấu nhắc quay lại: tới lượt bạn |

Để ý dấu nhắc đã đổi từ `%` sang `>>>` sau dòng đầu tiên. Cửa sổ vẫn là cửa sổ
đó, ô cửa vẫn là ô cửa đó — chỉ là **người ngồi bên trong đã đổi**. Trước dòng
`python3`, người trả lời bạn là máy. Sau dòng đó, người trả lời bạn là Python.

Từ đây tới hết Realm 0, mọi câu bạn nói đều nói với `>>>`.
::::

::::predict{#chua-bam-enter commitOnce}
Bạn gõ xong `print("Phở bò")` sau dấu nhắc `>>>`. Chữ đã hiện lên màn hình đủ
cả. Nhưng bạn **chưa** bấm **Enter** — bạn đang ngồi nhìn nó.

**Trước khi xem đáp án**, bạn đoán lúc này máy đang làm gì?

:::opt{correct}
Chưa làm gì cả. Nó chưa nhận được câu nào, vẫn đang chờ bạn dứt câu.
:::

:::opt
Nó đã đọc rồi, đang chuẩn bị in ra, chỉ chậm một nhịp.
::why
Gần đúng ở chỗ chữ **đúng là** đã hiện lên màn hình rồi, và mắt bạn thấy nó rất
rõ. Nghĩ rằng máy cũng thấy như bạn là một suy luận tự nhiên.

Chỗ lệch: chữ hiện trên màn hình mới chỉ nằm trên **dòng đang soạn**, giống câu
bạn đang nói dở trước ô cửa bán vé. Người bán vé nghe thấy tiếng bạn, nhưng họ
chưa trả lời vì bạn chưa nói hết câu.

**Enter** chính là dấu chấm hết câu. Trước khi bạn bấm nó, bạn còn xoá đi
sửa lại thoải mái — và máy vẫn chưa biết gì.
::
:::

:::opt
Nó báo lỗi, vì câu lệnh còn dang dở.
::why
Gần đúng ở chỗ bạn cảnh giác với lỗi. Cảnh giác là tốt, và bạn sẽ dùng tới nó
nhiều.

Chỗ lệch là về **thời điểm**. Máy chỉ có thể nói câu của bạn sai khi nó đã nhận
được câu ấy. Ở đây nó chưa nhận, nên nó cũng chưa có gì để chê. Một dòng chưa
bấm **Enter** thì chưa tồn tại đối với máy.
::
:::
::::

::::predict{#sau-khi-may-dap commitOnce}
Bạn bấm **Enter**. Máy in ra `Phở bò`.

Ngay sau đó, màn hình có gì?

:::opt{correct}
Dấu nhắc `>>>` hiện lại ở dòng dưới, và máy đứng chờ câu tiếp theo.
:::

:::opt
Cửa sổ đóng lại, vì việc đã xong.
::why
Gần đúng ở chỗ bạn đang nghĩ theo lối "một việc — làm xong — kết thúc", và với
phần lớn phần mềm bạn dùng hằng ngày thì đúng là như vậy: bấm nút, xong, đóng.

Chỗ lệch: terminal không phải một cái nút, nó là một **cuộc trò chuyện**. Người
bán vé bán xong một vé thì không đứng dậy bỏ về; họ nhìn ra chờ khách sau. Máy
cũng vậy — in xong là hiện lại dấu nhắc, và ngồi đó chờ bạn bao lâu cũng được.
::
:::

:::opt
Máy in `Phở bò` liên tục cho tới khi bạn bảo dừng.
::why
Gần đúng ở chỗ bạn nhớ một tính chất rất thật của máy: nó lặp lại một việc bao
nhiêu lần cũng không mệt. Tính chất ấy có thật, và tới Module 2 bạn sẽ sai khiến
nó làm đúng chuyện đó.

Chỗ lệch: máy chỉ làm lại khi **được bảo** làm lại. Bạn mới nói một câu, nên nó
làm đúng một lần. Đây chính là điều bài 3 đã nói: máy chờ, và nó không tự bổ
sung thêm ý nào bạn chưa nói ra.
::
:::
::::

::::explain{#vi-sao-hoc-o-day}
Có thể bạn đang nghĩ: sao lại bắt đầu ở một cửa sổ trơ trụi như thế, trong khi
màn hình đầy phần mềm đẹp đẽ?

Vì cái ô cửa này có một thứ mà phần mềm đẹp đẽ không có: **bạn thấy hết**.

Không có nút nào giấu việc gì bên trong. Bạn nói gì, máy đáp gì, tất cả nằm cả
trên màn hình, từng dòng, theo đúng thứ tự đã xảy ra. Khi có chuyện lạ, bạn cuộn
lên và đọc lại đúng những gì hai bên đã nói.

Trong suốt khoá học, đây là chỗ bạn thử một ý nghĩ trong mười giây: gõ một dòng,
bấm **Enter**, nhìn câu trả lời.
::::

::::reflect{#nghi-lai}
Bây giờ bạn đã có chỗ để gõ. Byte muốn trả lại cho bạn một món nợ từ bài 1.

Hôm đó bạn viết `print("Xin chào")`, và câu hỏi để ngỏ là: nếu **bỏ hai dấu
nháy** đi thì sao?

Giờ bạn có thêm một manh mối mà hôm ấy chưa có. Bài 6 nói: chữ nằm giữa hai dấu
nháy được máy tra bảng ra số rồi giữ nguyên văn, không cố hiểu.

Vậy khi hai dấu nháy biến mất, `Phở` không còn là chữ để đọc nguyên văn nữa.
Máy sẽ **thấy** nó là cái gì? Và nó sẽ đi tìm cái gì?

Đừng trả lời vội. Bài sau gõ đúng dòng đó vào `>>>` và xem máy đáp lại thế nào.
::::

::::checkpoint{mastery=0.8}
::::
