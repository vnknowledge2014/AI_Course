---
id: onboarding.may-tinh-noi-gi.khi-khong-ai-bao-gi
title: Khi không ai bảo, máy làm gì
summary: Máy không nghỉ và cũng không đoán. Nó chờ — rồi làm đúng những gì được bảo.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 3
tier: A
languages: [text]
defaultLanguage: text
level: intro
estimatedMinutes: 9
teaches: [core.may-cho-lenh, core.nghia-den]
requires: [core.chuong-trinh]
concepts: [core.may-cho-lenh, core.nghia-den]
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
Lúc bạn đi pha trà, mình không nghỉ. Mình chờ. Chờ là việc chính của mình.
::::

::::explain{#ai-lam-to-giay-chay}
Bài trước để lại hai câu hỏi. Trả lời câu thứ nhất trước.

Tờ giấy dán trên tường bếp nhà chị Hạnh chưa từng tự làm ra một tô phở nào. Phải
có cậu em họ đứng đó: đọc dòng một, làm dòng một; đọc dòng hai, làm dòng hai.
Hết dòng cuối thì ngừng.

Vậy trong chuyện này có hai vai, và chúng rời hẳn nhau:

- **bản dặn việc** — nằm im trên tường, tự nó không làm gì cả;
- **người thi hành** — đọc từng dòng và làm theo.

Chương trình là vai thứ nhất. Máy tính là vai thứ hai.

Việc máy đọc một chương trình rồi làm theo từng dòng, người ta gọi là **chạy**
chương trình (hoặc **thi hành** nó). Ở bài 1, lúc bạn bấm nút và Byte nói ra
"Xin chào", đó là máy chạy một chương trình dài đúng một dòng.
::::

::::explain{#cau-hoi-kho-hon}
Câu thứ hai khó hơn: máy tính của bạn đang bật, không ai gõ gì. Nó đang làm gì?

Nó **chờ**.

Không phải ngủ, không phải nghỉ, cũng không phải đang nghĩ xem nên làm gì tiếp.

Chỗ khác nhau giữa nó và cậu em họ nằm đúng ở đây. Cậu em đứng chờ trong bếp một
lúc là sốt ruột, rồi tự tìm việc: lau bàn, rửa bát, xếp lại đũa. Không ai bảo cả,
nhưng cậu ấy nhìn quanh và tự thấy có việc phải làm.

Máy thì không nhìn quanh. Không có dòng nào bảo lau bàn thì bàn không được lau,
dù bàn bẩn tới đâu.

Chỗ này quan trọng hơn vẻ ngoài của nó, nên xin nói thẳng thành một câu:

> Máy làm đúng những gì được bảo. Không nhiều hơn, không ít hơn, và không thay
> bạn nghĩ ra phần bạn quên nói.
::::

::::example{#con-tro-nhap-nhay}
Chỗ dễ thấy nhất của việc chờ là cái dấu nhấp nháy này:

```text title=readonly
$ ▮
```

Đó là máy đang nói: *tôi đang chờ một dòng*.

Nó nhấp nháy như vậy năm phút cũng được, cả đêm cũng được, và trong suốt thời
gian đó không có gì xảy ra. Bạn gõ một dòng rồi bấm Enter thì nó làm; làm xong nó
quay về đúng cái dấu nhấp nháy ấy. Chờ tiếp.

Cửa sổ có cái dấu nhấp nháy này mang một cái tên riêng, và bài thứ bảy sẽ nói kỹ
về nó. Bây giờ chỉ cần giữ lấy một điều: giữa hai lần bạn gõ, máy không tự làm
thêm gì.
::::

::::predict{#thieu-mot-dong commitOnce}
Chị Hạnh chép lại tờ giấy dặn việc, lần này chép **thiếu**: rơi mất dòng "Cho
bánh phở vào tô". Tờ giấy còn bốn dòng:

```text title=readonly
1. Trụng bánh phở trong nồi nước sôi
2. Xếp thịt bò tái lên trên
3. Chan nước dùng ngập bánh phở
4. Rắc hành lá
```

Lần này người làm theo không phải cậu em họ, mà là một cái máy. Nó đọc được đủ
bốn dòng và làm được đủ bốn việc.

**Trước khi xem đáp án**, bạn đoán máy làm gì?

:::opt{correct}
Nó làm đủ bốn dòng có trên giấy, và không làm dòng đã rơi mất — thịt xếp vào tô rỗng, nước dùng chan vào tô không có bánh phở.
:::

:::opt
Nó tự thêm bước cho bánh phở vào tô, vì tô phở nào cũng phải có bánh phở.
::why
Gần đúng ở chỗ bạn hiểu món ăn này rất rõ: thiếu bánh phở thì không còn là tô
phở nữa. Người nào đứng bếp cũng nhận ra chuyện đó trong một giây.

Nhưng cái "nhận ra trong một giây" ấy là của bạn, không phải của máy. Máy chưa
từng ăn phở, chưa từng nhìn thấy một tô phở nào. Với nó, tờ giấy này không phải
công thức nấu ăn — nó là bốn dòng, hết dòng bốn thì xong việc. Không có dòng nào
bảo cho bánh phở vào tô, nên chuyện đó không xảy ra.
::
:::

:::opt
Nó dừng lại ở dòng 3 và hỏi: bánh phở đâu?
::why
Gần đúng, và đây là điều một người phụ bếp tử tế sẽ làm thật: thấy vô lý thì
dừng tay hỏi lại.

Chỗ lệch nằm ở hai chữ "thấy vô lý". Muốn thấy vô lý thì phải giữ sẵn trong đầu
một hình dung về tô phở đúng ra phải thế nào, rồi đem cái trước mắt ra so. Máy
không giữ sẵn hình dung nào cả. Dòng 3 bảo chan nước dùng — nước dùng chan được,
thế là xong dòng 3, sang dòng 4.

Máy chỉ dừng lại và kêu lên khi nó **không đọc nổi** một dòng, hoặc **không làm
nổi** dòng đó. Còn "làm được nhưng ra kết quả ngớ ngẩn" thì nó im lặng làm.
::
:::
::::

::::explain{#cho-la-mot-tinh-chat}
Nghe tới đây dễ tưởng chờ là nhược điểm của máy. Ngược lại: đó là chỗ dựa của
bạn.

Vì máy chờ nên nó không tự tiện. Nó không đổi thực đơn lúc bạn ngủ, không tự sửa
một con số trong sổ chi tiêu, không tự nhắn tin thay bạn. Mọi thứ nó làm đều lần
ngược về được tới một dòng do ai đó viết ra.

Và cũng vì thế, khi máy làm không đúng ý bạn, câu hỏi đầu tiên **không** phải là
"máy hỏng à?". Câu hỏi đầu tiên là:

> Mình đã bảo nó cái gì?

Phần lớn nghề này nằm gọn trong câu đó. Máy hầu như không bao giờ hiểu sai. Nó
hiểu **đúng** thứ bạn viết — chỉ là thứ bạn viết chưa phải thứ bạn định nói.
::::

::::predict{#may-bat-ca-dem commitOnce}
Tối nay bạn để máy bật, không tắt, không đụng vào. Sáng ra mở lên thì thấy vài
thứ đã khác: một phần mềm vừa cập nhật xong, mấy tấm ảnh trong máy đã được chép
lên mạng.

Vậy máy có tự làm việc lúc không ai bảo không?

:::opt{correct}
Không. Mỗi việc đó là một chương trình có sẵn, được người ta viết và hẹn giờ từ trước — máy vẫn đang chờ rồi làm theo lời dặn.
:::

:::opt
Có. Không ai ngồi đó bảo nó cả, mà việc vẫn xong — nghĩa là nó tự làm.
::why
Gần đúng ở phần quan sát, và bạn quan sát không sai một chi tiết nào: **bạn**
không bảo gì cả, và việc vẫn xong thật.

Chỗ lệch nằm ở chữ "ai". Lời dặn không nhất thiết phải do bạn viết, cũng không
nhất thiết viết tối qua. Người làm ra phần mềm ấy đã viết sẵn một tờ giấy dặn
việc từ lâu, và kèm vào đó một dòng: *hai giờ sáng thì bắt đầu*. Đến hai giờ, máy
làm theo tờ giấy đó. Vẫn là chờ rồi làm theo — chỉ khác là lần này nó chờ một cái
đồng hồ chứ không chờ bạn.
::
:::

:::opt
Có, và đó là lý do máy càng dùng lâu thì càng biết nhiều hơn về mình.
::why
Gần đúng ở một cảm giác có thật: máy đúng là giữ lại rất nhiều thứ về bạn, và
càng ngày nó càng đưa ra đúng thứ bạn hay mở.

Nhưng thứ giữ lại được ấy là do đã có một chương trình được viết ra để ghi và để
sắp xếp. Không có dòng nào bảo ghi thì không có gì được ghi. Cái ta cảm thấy như
"máy hiểu mình" luôn luôn nằm trong một bản dặn việc do người viết — dài hơn,
khó hơn, nhiều dòng hơn tờ giấy trong bếp chị Hạnh rất nhiều, nhưng vẫn là một
bản dặn việc.
::
:::
::::

::::reflect{#nghi-lai}
Máy chờ, và nó không nghĩ hộ bạn phần bạn quên nói. Nhưng có một chuyện nó nhớ
được thật, và đáng ngạc nhiên hơn ta tưởng.

Ở bài 1 bạn viết `print("Xin chào")`. Bây giờ tắt máy đi, đi ăn cơm, tối bật lại
— dòng đó vẫn còn nguyên, mở ra là thấy.

Mà tắt máy nghĩa là cắt điện. Trong suốt lúc không có điện ấy, dòng chữ của bạn
**nằm ở đâu**? Và lúc bật lên, máy dựa vào cái gì để tìm lại đúng nó giữa hàng
triệu thứ khác?

Đừng trả lời vội. Bài sau bắt đầu từ đúng chỗ đó.
::::

::::checkpoint{mastery=0.8}
::::
