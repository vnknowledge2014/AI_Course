---
id: onboarding.may-tinh-noi-gi.bit-va-byte
title: Một ô chỉ chứa có hoặc không
summary: Bên trong mọi file chỉ có một thứ — những cái ô hai trạng thái, xếp thành từng nhóm tám.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 5
tier: A
languages: [text]
defaultLanguage: text
level: intro
estimatedMinutes: 11
teaches: [core.bit, core.byte]
requires: [core.file]
concepts: [core.bit, core.byte]
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
Bài này bạn biết vì sao mình tên là Byte. Cùng mở một cái hộp ra xem nhé.
::::

::::explain{#trong-o-cung-chi-co-mot-loai-cho}
Câu hỏi cuối bài trước: bản thực đơn, tấm ảnh khai trương và bài vọng cổ giống
nhau ở điểm nào khi nằm trong máy?

Câu trả lời nằm ở chỗ mà ta hay tưởng tượng sai nhất.

Ta dễ nghĩ trong ổ cứng có nhiều loại ngăn: ngăn cho chữ, ngăn cho ảnh, ngăn cho
tiếng hát — mỗi loại một kiểu, như tủ trong bếp có ngăn đựng bát, ngăn đựng đũa,
ngăn đựng nồi.

Không phải vậy. Trong ổ cứng chỉ có **một** loại chỗ chứa, lặp đi lặp lại hàng
tỉ lần. Mỗi chỗ ấy là một cái ô nhỏ, và mỗi ô chỉ giữ được một trong hai trạng
thái. Không có trạng thái thứ ba, không có nấc lưng chừng.

Chữ, ảnh, tiếng hát đều phải nằm vừa trong những cái ô đó. Cả ba không có lựa
chọn nào khác.
::::

::::example{#bang-dien-tam-cong-tac}
Để thấy hai trạng thái làm được gì, nhìn thứ đã có sẵn trong quán cô Bảy. Sau
quầy có một bảng điện tám công tắc:

```text title=readonly
đèn biển hiệu   ● bật
đèn trong nhà   ● bật
quạt trần       ○ tắt
quạt đứng       ● bật
tủ mát          ● bật
nồi nước dùng   ● bật
đèn nhà sau     ○ tắt
loa             ○ tắt
```

Mỗi công tắc chỉ có bật hoặc tắt. Gạt lưng chừng thì nó tự rơi về một trong hai
bên — không có nấc giữa để mà đứng.

Tối nay cô Bảy muốn nhắn cho người trực ca sau biết bảng điện đang để thế nào. Cô
không cần chép cả tám dòng. Cô quy ước gọn: **bật ghi là 1, tắt ghi là 0**, và
ghi từ trên xuống:

```text title=readonly
11011100
```

Tám ký tự đó nói đủ mọi thứ về bảng điện, không thiếu, không thừa. Người nhận
biết quy ước thì dựng lại được nguyên cái bảng.

Máy tính chứa đồ đúng theo kiểu này. Chỉ khác hai chỗ: ô của nó nhỏ tới mức một
mẩu bằng móng tay chứa được hàng tỉ ô, và "bật/tắt" của nó không phải cái cần
gạt mà là có dòng điện đi qua hay không.

Một cái ô như vậy — hai trạng thái, không có nấc giữa — gọi là một **bit**.
::::

::::predict{#ba-cong-tac commitOnce}
Lấy riêng ba công tắc đầu: đèn biển hiệu, đèn trong nhà, quạt trần.

Cô Bảy muốn liệt kê ra giấy **mọi kiểu** bật-tắt mà ba công tắc ấy có thể ở
trong — để tối nào cũng chỉ việc khoanh vào một kiểu.

**Trước khi xem đáp án**, bạn đoán cô phải ghi mấy dòng?

:::opt{correct}
8 dòng.
:::

:::opt
6 dòng — ba công tắc, mỗi cái hai trạng thái, ba nhân hai.
::why
Gần đúng, và hướng đi của bạn là hướng đúng: mỗi lần thêm một công tắc thì số
kiểu **nhân lên** chứ không cộng thêm. Nhiều người vấp ở chỗ cộng, bạn thì không.

Chỉ là nhân nhầm hai con số. Ba công tắc không thay phiên nhau — cả ba cùng có
mặt một lúc. Với **mỗi** cách đặt công tắc thứ nhất (2 cách), công tắc thứ hai
lại có 2 cách riêng của nó, thành 2 × 2 = 4. Thêm công tắc thứ ba thì mỗi kiểu
trong 4 kiểu đó lại tách làm đôi, thành 8. Nhân số **2** với chính nó ba lần,
chứ không nhân 3 với 2.

Đếm tay cho chắc: 000, 001, 010, 011, 100, 101, 110, 111 — đúng tám dòng.
::
:::

:::opt
3 dòng, mỗi công tắc một dòng.
::why
Gần đúng ở chỗ con số 3 có thật trong đề bài: đúng là có ba công tắc, bạn đọc
không sót.

Chỉ là câu hỏi hỏi thứ khác. Không phải "có mấy công tắc", mà "ba cái đặt cùng
lúc thì ra được mấy bức tranh khác nhau". Bật mỗi cái thứ nhất là một bức. Bật
cái thứ nhất cùng cái thứ ba là một bức khác hẳn. Ngay cả tắt sạch cả ba cũng đã
là một bức rồi.
::
:::
::::

::::explain{#tam-bit-thanh-mot-byte}
Cứ thêm một bit thì số kiểu gấp đôi:

```text
1 bit  →   2 kiểu
2 bit  →   4 kiểu
3 bit  →   8 kiểu
4 bit  →  16 kiểu
...
8 bit  → 256 kiểu
```

Tám bit đứng cạnh nhau đủ để phân biệt 256 thứ khác nhau. Con số đó vừa vặn cho
rất nhiều việc trong máy, tới mức người ta gộp luôn tám bit thành một nhóm và đặt
cho nhóm ấy một cái tên riêng: một **byte**.

Từ đây, cách gọn nhất để nói một file to bằng nào là đếm xem nó chiếm bao nhiêu
byte. Byte nhiều quá thì gọi tắt: khoảng một nghìn byte là một **kilobyte** (KB),
khoảng một triệu byte là một **megabyte** (MB). Dòng chữ "2,4 MB" dưới một tấm
ảnh nghĩa là chừng ấy triệu cái ô tám-bit đang nằm trong ổ cứng vì tấm ảnh đó.
::::

::::byte{trigger=enter mood=happy pose=jump}
Tám cái ô đứng cạnh nhau thì thành mình. Giờ bạn biết mình tên gì rồi.
::::

::::predict{#doi-duoi-jpg commitOnce}
Bài trước bạn thấy đổi tên file không đụng gì tới ruột. Lần này thử đẩy tới cùng.

Lấy `khai-truong.jpg`, đổi tên thành `khai-truong.mp3`, rồi mở nó bằng chương
trình nghe nhạc.

**Trước khi xem đáp án**, bạn đoán chuyện gì xảy ra?

:::opt{correct}
Dãy bit trong file không đổi một ô nào. Chương trình nghe nhạc đọc dãy đó theo luật của nhạc, gặp thứ không khớp, nên hoặc báo không mở được, hoặc phát ra tiếng rè.
:::

:::opt
Tấm ảnh biến thành một đoạn nhạc — chính vì mọi thứ trong máy đều là 0 với 1 nên chúng đổi qua lại được.
::why
Gần đúng ở nửa đầu, và nửa đầu ấy chính là ý lớn nhất của cả bài: ảnh và nhạc
đúng là cùng một chất liệu, cùng là một dãy 0 và 1.

Chỗ lệch nằm ở ba chữ "biến thành". Cùng chất liệu không có nghĩa là cùng cách
đọc. Vẫn dãy `01001101` ấy: đọc theo luật của ảnh thì ra màu của một chấm nhỏ,
đọc theo luật của nhạc thì ra một nấc âm thanh. Đổi cái tên là đổi luật được đem
ra dùng, chứ không đổi dãy số.

Giống như đưa một trang tiếng Việt cho người chỉ biết đọc tiếng Ý: chữ vẫn nguyên
chữ đó, chỉ là đọc lên ra thứ khác.
::
:::

:::opt
Máy thấy đuôi tên không khớp với ruột nên tự chuyển tấm ảnh sang dạng nhạc giúp bạn.
::why
Gần đúng, và bạn dùng đúng bài trước: đuôi tên là một lời hứa, và ở đây lời hứa
đang sai so với ruột. Bạn phát hiện ra mâu thuẫn — chỗ đó không sai chút nào.

Chỗ lệch là ai đứng ra giải quyết mâu thuẫn ấy. Không có dòng nào bảo máy chuyển
đổi, nên không có chuyển đổi nào xảy ra — quay lại bài 3: máy không tự thêm việc.

Và cũng nên nói cho hết: chuyển một tấm ảnh sang tiếng hát thì chẳng có nghĩa gì
để mà làm. Còn chuyển ảnh sang một dạng ảnh khác thì có thật, nhưng phải có một
chương trình được gọi ra và được bảo làm đúng việc đó.
::
:::
::::

::::explain{#cung-chat-lieu-khac-cach-doc}
Vậy trả lời gọn câu hỏi bài trước: bản thực đơn, tấm ảnh và bài vọng cổ giống
nhau ở chỗ cả ba đều là **một dãy byte**. Chỉ có thế.

Cái làm chúng khác nhau không nằm trong ổ cứng. Nó nằm ở **luật đọc** — cách một
chương trình cắt dãy byte đó ra thành từng mẩu và hiểu mỗi mẩu là gì.

Từ đây, một câu đáng nhớ hơn nó có vẻ:

> Trong máy không có "chữ", "ảnh", "nhạc". Chỉ có những dãy byte, và những luật
> đọc chúng.

Đây cũng là lý do đuôi tên ở bài trước tồn tại. Dãy byte không tự khai nó là gì,
nên cái tên phải nói hộ, để máy biết đem luật nào ra dùng.
::::

::::reflect{#nghi-lai}
Còn một chỗ hở, và nó nằm ngay trong bản thực đơn của cô Bảy.

Con số thì còn hình dung được: bật với tắt xếp lại thành số, như tám công tắc
thành `11011100`. Nhưng dòng đầu thực đơn là chữ **Phở**.

Chữ "Phở" không phải 0, cũng không phải 1. Nó không sáng, không tắt. Chữ *ơ* có
dấu móc, chữ *ở* còn thêm dấu hỏi — chẳng cái nào giống một cái công tắc cả.

Vậy nó nằm được trong những cái ô chỉ chứa 0 và 1 bằng cách nào?

Đừng trả lời vội. Bài sau chỉ ra một bảng quy ước mà gần như cả thế giới đã ngồi
lại đồng ý với nhau.
::::

::::checkpoint{mastery=0.8}
::::
