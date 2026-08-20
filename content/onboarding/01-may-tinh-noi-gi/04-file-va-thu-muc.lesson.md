---
id: onboarding.may-tinh-noi-gi.file-va-thu-muc
title: "Cái hộp có tên: file và thư mục"
summary: Thứ sống sót qua một lần tắt máy là thứ đã được cất vào một cái hộp có tên và có chỗ đứng.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 4
tier: A
languages: [text]
defaultLanguage: text
level: intro
estimatedMinutes: 10
teaches: [core.file, core.thu-muc, core.duong-dan]
requires: [core.may-cho-lenh]
concepts: [core.file, core.thu-muc]
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
Mất điện là mình quên sạch. Trừ thứ bạn kịp cất vào một cái hộp có tên.
::::

::::explain{#dien-tat-thi-con-gi}
Bài trước kết ở một câu hỏi: tắt máy là cắt điện, vậy dòng `print("Xin chào")`
của bạn nằm ở đâu trong lúc đó?

Cô Bảy bán phở cuối ngõ, cách quán chị Hạnh mấy nhà. Trong quán cô có hai chỗ
ghi chữ, và chúng khác nhau hoàn toàn:

- **Cái bảng con** treo cạnh bếp, ghi tô đang làm dở: *bàn 3, hai tái nạm*. Ghi
  nhanh, xoá nhanh, lúc nào cũng đúng chuyện đang xảy ra ngay lúc này. Hết ca thì
  lau sạch bảng.
- **Cuốn sổ** cất trong ngăn kéo: tiền chợ, thực đơn, số điện thoại nhà cung cấp.
  Đóng cửa quán, khoá cửa, mai mở ra vẫn nguyên từng dòng.

Máy tính có đúng hai chỗ như vậy. Một chỗ để ghi tạm trong lúc đang chạy — cắt
điện là mất sạch, không sót một chữ. Một chỗ để cất lâu dài, nằm trong cái ổ cứng
gắn bên trong máy — cắt điện thì nó vẫn còn.

Thứ sống sót qua một lần tắt máy là thứ đã được **lưu** xuống chỗ thứ hai. Và một
thứ đã lưu như vậy gọi là một **file** (tiếng Việt gọi là *tệp*; sách vở lẫn màn
hình đều dùng cả hai từ, bạn sẽ gặp cả hai).
::::

::::example{#ba-thu-cua-mot-file}
Cô Bảy mỗi tối ngồi gõ tiền chợ vào máy. Mở máy cô ra thì thấy thế này:

```text title=readonly
Tài liệu/
    Quán phở/
        thuc-don.txt
        tien-cho-thang-8.txt
    Ảnh/
        khai-truong.jpg
```

Nhìn kỹ dòng `tien-cho-thang-8.txt`. Nó gồm ba thứ, và thiếu thứ nào cũng không
xong:

- **Ruột** — thứ nằm bên trong: mấy chục dòng tiền chợ cô đã gõ.
- **Tên** — `tien-cho-thang-8.txt`. Máy không mở ruột ra đọc để đoán xem file này
  là gì. Nó gọi file theo tên, y như bạn gọi người theo tên.
- **Chỗ đứng** — file này nằm trong `Quán phở`, mà `Quán phở` lại nằm trong
  `Tài liệu`.

Mấy dòng có gạch chéo ở cuối — `Tài liệu/`, `Quán phở/`, `Ảnh/` — tự chúng không
chứa chữ nào. Chúng chỉ chứa những thứ khác. Một cái như vậy gọi là **thư mục**
(trên màn hình thường vẽ hình cái cặp đựng hồ sơ, và nhiều người quen gọi là
*folder*).

Ghép tên với chỗ đứng lại thì được tên đầy đủ của file:

```text title=readonly
Tài liệu/Quán phở/tien-cho-thang-8.txt
```

Chuỗi đó có tên riêng: **đường dẫn**. Nó là địa chỉ nhà của file — đọc từ ngoài
vào trong, mỗi dấu gạch chéo là một lần mở thêm một cái cặp ra.

Đó cũng chính là thứ máy dựa vào để tìm lại dòng chữ bạn viết hôm qua. Nó không
đi lục từng chỗ để xem chỗ nào giống giống. Nó đi theo địa chỉ.
::::

::::predict{#hai-file-cung-ten commitOnce}
Cô Bảy mở thêm quán bún ở phố bên. Cô tạo một thư mục mới tên `Quán bún`, ngang
hàng với `Quán phở`, rồi muốn đặt trong đó một file cũng tên `thuc-don.txt`.

**Trước khi xem đáp án**, bạn đoán máy cho hay không cho?

:::opt{correct}
Cho. Hai file trùng tên nhưng khác chỗ đứng, nên hai đường dẫn vẫn khác nhau.
:::

:::opt
Không cho, vì trong một cái máy thì mỗi file phải mang một tên riêng, không được trùng.
::why
Gần đúng ở chỗ quan trọng nhất, và bạn nên giữ lấy chỗ đó: hai file **phải phân
biệt được**, nếu không thì máy biết lấy cái nào. Đòi hỏi ấy có thật.

Chỉ lệch ở phạm vi. Cái buộc phải khác nhau là **đường dẫn**, chứ không phải cái
tên trần. `Quán phở/thuc-don.txt` và `Quán bún/thuc-don.txt` đã khác nhau ngay từ
chữ đầu tiên rồi. Cũng như trong một xóm có hai người cùng tên Hạnh thì thư từ
vẫn tới đúng nhà, miễn là khác số nhà.

Còn trong **cùng một** thư mục thì đúng là không được trùng — chỗ đó bạn nghĩ
chính xác, chỉ là nó áp cho một cái cặp, không áp cho cả máy.
::
:::

:::opt
Cho, nhưng máy sẽ tự đổi tên file thứ hai thành `thuc-don-2.txt` cho khỏi lẫn.
::why
Gần đúng, và bạn nhớ đúng một chuyện có thật: chép một file vào chỗ đã có file
trùng tên thì máy hay hiện ra `thuc-don (2).txt`.

Nhưng để ý chuyện đó xảy ra khi nào — khi hai file **cùng một chỗ đứng**, tức là
thật sự đụng nhau. Và nó xảy ra vì có người đã viết sẵn một chương trình xử lý
đúng tình huống ấy. Ở đây hai file nằm hai thư mục khác nhau, chẳng có gì đụng
nhau để mà phải xử lý.

Nhớ bài trước: máy không tự thêm việc khi không có dòng nào bảo nó thêm.
::
:::
::::

::::explain{#cai-duoi-la-mot-loi-hua}
Còn ba chữ cuối tên file thì sao — `.txt`, `.jpg`?

Phần đó gọi là **đuôi tên** (hay *phần mở rộng*). Nó vẫn nằm trong cái tên, không
nằm trong ruột. Nó là một lời hứa dán bên ngoài hộp:

- `.txt` — trong này là chữ thường.
- `.jpg` — trong này là một tấm ảnh.
- `.py` — trong này là lệnh viết bằng Python, đúng loại lệnh bạn đã gõ ở bài 1.

Máy nhìn lời hứa ấy để chọn xem mở file bằng chương trình nào: gặp `.jpg` thì gọi
chương trình xem ảnh, gặp `.txt` thì gọi chương trình xem chữ.

Chú ý hai chữ **lời hứa**. Lời hứa nằm ngoài hộp, và không ai bắt nó phải khớp
với thứ trong hộp.
::::

::::predict{#doi-duoi-ten commitOnce}
Bạn có một file tên `bai-mot.py`, bên trong đúng một dòng:

```text title=readonly
print("Xin chào")
```

Bạn đổi tên nó thành `bai-mot.txt`. Không mở ra, không sửa gì bên trong, chỉ đổi
mỗi cái tên.

Bên trong file bây giờ là gì?

:::opt{correct}
Vẫn đúng dòng `print("Xin chào")`, không sai một ký tự nào.
:::

:::opt
File thành ra rỗng, vì đổi sang loại khác thì nội dung cũ không còn dùng được nữa.
::why
Gần đúng ở một cảm giác đúng: sau khi đổi tên, có gì đó không còn khớp giữa cái
nhãn mới và cái ruột cũ. Cảm giác ấy có cơ sở, và bài sau sẽ nói kỹ về nó.

Nhưng đổi tên là đổi cái nhãn dán ngoài hộp, không phải đổi thứ trong hộp. Cô Bảy
lấy bút xoá chữ "Tháng 8" trên bìa cuốn sổ rồi ghi đè "Tháng 9" — mấy chục dòng
tiền chợ bên trong vẫn nằm nguyên, từng dòng một.
::
:::

:::opt
Máy chuyển nội dung sang dạng chữ thường, nên dòng đó vẫn còn nhưng không chạy được nữa.
::why
Gần đúng ở kết luận, và kết luận ấy đúng thật: sau khi đổi tên, bấm chạy file này
thì phần lớn công cụ sẽ không chịu chạy nữa.

Chỉ là nguyên nhân nằm chỗ khác. Máy không **chuyển** gì cả — nó không đụng vào
một ký tự nào trong ruột. Thứ thay đổi là lời hứa dán ngoài: `.py` hứa "trong này
là lệnh Python", `.txt` hứa "trong này là chữ thường". Công cụ nhìn lời hứa để
quyết định có mở ra chạy hay không.

Bằng chứng: đổi tên ngược lại thành `.py` là chạy được ngay, không mất gì. Ruột
chưa từng bị đụng tới.
::
:::
::::

::::explain{#ten-va-ruot-la-hai-thu}
Vậy là bạn có ba thứ rời nhau, và rời nhau thật sự:

- **ruột** — thứ nằm bên trong;
- **tên** — cái nhãn, kể cả đuôi tên;
- **chỗ đứng** — thư mục chứa nó, ghép lại thành đường dẫn.

Đổi tên không đụng ruột. Chuyển file sang thư mục khác cũng không đụng ruột. Cắt
cuốn sổ ra khỏi ngăn kéo này bỏ sang ngăn kéo kia thì chữ trong sổ vẫn thế.

Từ đây trở đi, mỗi lần bạn lưu một chương trình, bạn đang làm đúng ba việc: viết
ruột, đặt tên, chọn chỗ đứng.
::::

::::reflect{#nghi-lai}
Bạn vừa thấy tên và ruột là hai thứ rời nhau. Nhưng cả bài này chưa hề mở ruột ra
xem lấy một lần.

Trong máy cô Bảy có ba thứ trông chẳng liên quan gì tới nhau: bản thực đơn toàn
chữ, tấm ảnh khai trương, và một bài vọng cổ cô hay mở lúc dọn quán.

Chữ, ảnh, tiếng hát — với bạn thì ba thứ này khác hẳn nhau. Vậy mà cả ba cùng cất
trên một cái ổ cứng, cùng gọi ra được bằng một đường dẫn, cùng chép sang USB được
như nhau.

Khi nằm trong máy, ba thứ ấy giống nhau ở điểm nào?

Đừng trả lời vội. Bài sau mở một cái hộp ra, xem tận bên trong.
::::

::::checkpoint{mastery=0.8}
::::
