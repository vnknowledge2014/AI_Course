---
id: lap-trinh-ham.cong-rust.doc-va-sua-xen-ke-qua-nhieu-cau-lenh
title: "Mượn đọc rồi mượn sửa, xen kẽ qua NHIỀU câu lệnh — luật phụ thuộc vào ai còn 'sống'"
summary: "Mượn &x rồi DÙNG NÓ LẦN CUỐI, sau đó mới mượn &mut x — ĐƯỢC PHÉP (vùng sống của &x đã kết thúc ở lần dùng cuối, không phải ở hết block). Nhưng mượn &x, rồi &mut x, RỒI MỚI dùng &x — BỊ TỪ CHỐI (E0502). Luật thật: không phải THỨ TỰ dòng lệnh, mà là VÙNG SỐNG — lý do byte-rust không kiểm được chuyện này, đã xác minh bằng rustc thật."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 15
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.mixed-borrow-nll]
requires: [rs.multiple-shared-ok]
concepts: [rs.mixed-borrow-nll]
gradingMatrix:
  web-chrome: [run, tests, output]
  web-firefox: [run, tests, output]
  macos: [run, tests, output]
  windows: [run, tests, output]
  linux: [run, tests, output]
  android: [run, tests, output]
  ios: [run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Bài trước, mọi tham chiếu nằm gọn trong MỘT lời gọi. Hôm nay chúng
rải ra qua NHIỀU câu lệnh — và luật không còn nhìn được bằng mắt
thường nữa.
::::

::::explain{#vung-song-khong-phai-thu-tu-dong}
Hai bài trước, luật luôn xét trong phạm vi MỘT lời gọi hàm: nhìn vào
đúng một dòng, đếm xem có mấy `&mut` trỏ cùng chỗ. Nhưng phần lớn
chương trình thật không gói mọi tham chiếu vào một dòng — bạn mượn
`&x` ở một câu lệnh, dùng nó, RỒI vài dòng sau mới mượn `&mut x`.
Câu hỏi đặt ra: tham chiếu `&x` cũ đó có còn "sống" không, khi
`&mut x` xuất hiện?

Câu trả lời KHÔNG dựa vào việc `&x` được khai ở dòng nào, hay nó còn
nằm trong cùng một khối `{ }` hay không. Nó dựa vào **vùng sống**
(lifetime) thật sự của tham chiếu đó — tức là: dòng CUỐI CÙNG nó còn
được dùng tới nằm ở đâu. Rust hiện đại (từ edition 2018, cơ chế gọi
là **non-lexical lifetimes**, viết tắt NLL) tính vùng sống theo LẦN
DÙNG CUỐI, không theo hết cả khối lệnh bao quanh nó.

Điều này có nghĩa: nếu `&x` được dùng LẦN CUỐI trước khi `&mut x`
xuất hiện, vùng sống của `&x` đã KẾT THÚC — dù dòng khai `&x` và dòng
khai `&mut x` cùng nằm trong một hàm, không có xung đột nào cả. Còn
nếu `&x` còn được dùng SAU khi `&mut x` đã xuất hiện và sửa dữ liệu,
hai vùng sống đó chồng lên nhau — bị từ chối, đúng luật cũ (nhiều
đọc thì được, nhưng không được vừa đọc vừa sửa cùng lúc).

Đây chính xác là lý do bài này KHÔNG có bước chấm điểm sống bằng
code: xác định một vùng sống kết thúc ở đâu đòi hỏi nhìn theo DÒNG
CHẢY của chương trình (dòng nào chạy trước, dòng nào là lần dùng
cuối) — không phải nhìn cây cú pháp của một lời gọi đơn lẻ như hai
bài trước. `byte-rust` (bộ chấm bài của track này) không phân tích
luồng chương trình theo cách đó, nên với tình huống này, nó KHÔNG có
gì đáng tin để nói ra — không báo lỗi, không báo "chưa hỗ trợ", nó
đơn giản CHẠY code đó, kể cả khi Rust thật sẽ từ chối. Đây không phải
lời khen cho `byte-rust` — im lặng của nó ở tình huống này KHÔNG có
nghĩa là chương trình đúng. Câu trả lời đúng trong bài này tới từ
`rustc` thật, biên dịch trực tiếp, không phải từ engine chấm bài.
::::

::::example{#dung-xong-truoc-khi-mut-toi}
`&x` được dùng LẦN CUỐI ở dòng `println!`, RỒI mới mượn `&mut x`:

```rust title=readonly
fn main() {
    let mut x = 5;
    let r = &x;
    println!("{}", r);   // r dùng LẦN CUỐI ở đây
    let m = &mut x;       // vùng sống của r đã kết thúc — không xung đột
    *m += 1;
    println!("{}", x);
}
```

```text title=readonly
5
6
```

`rustc` thật CHO PHÉP đoạn mã này biên dịch (đã xác minh bằng
`rustc --edition 2021` trực tiếp, không suy đoán). Vùng sống của `r`
chấm dứt ngay sau dòng `println!("{}", r)` — dòng cuối cùng nó còn
được đọc tới. Từ đó trở đi, `x` coi như "rảnh", `&mut x` mượn được
bình thường.
::::

::::predict{#doc-sau-khi-mut-da-sua commitOnce}
Byte đổi đúng MỘT chi tiết trong chương trình vừa rồi: chuyển dòng
`println!("{}", r)` xuống DƯỚI đoạn sửa qua `m`, thay vì để nó dùng
`r` trước:

```rust
fn main() {
    let mut x = 5;
    let r = &x;
    let m = &mut x;
    *m += 1;
    println!("{} {}", r, x);
}
```

**Trước khi đọc đáp án**, đoạn mã này có được `rustc` thật chấp nhận
không?

:::opt{correct}
Không — `rustc` từ chối (mã lỗi `E0502`). `r` (mượn đọc) vẫn còn
được DÙNG ở dòng `println!` cuối, tức vùng sống của nó kéo dài tới
tận đó. `m` (mượn sửa) xuất hiện VÀ SỬA `x` trong khi vùng sống của
`r` chưa kết thúc — hai vùng sống chồng lên nhau, một bên đọc một
bên sửa, cùng lúc
:::

:::opt
Có — hai chương trình này về bản chất GIỐNG HỆT ví dụ trước, chỉ đổi
chỗ một dòng lệnh, và thứ tự các dòng lệnh không ảnh hưởng gì tới
việc Rust có chấp nhận chương trình hay không
::why
Gần đúng ở việc bạn nhận ra hai chương trình rất giống nhau về hình
dạng — cùng khai `r`, cùng khai `m`, chỉ đổi vị trí một dòng
`println!`.

Chỗ lệch: chính cái "đổi vị trí một dòng" đó lại là điều duy nhất
quan trọng ở đây. Bài học cốt lõi của bài này là VÙNG SỐNG không cố
định theo hình dạng khối lệnh — nó phụ thuộc vào DÒNG NÀO CÒN DÙNG
tới biến. Chuyển `println!("{}", r)` xuống sau `*m += 1` kéo dài
vùng sống của `r` tới đúng chỗ nó chồng lên vùng sống của `m` — thứ
tự dòng lệnh ở đây quyết định TRỰC TIẾP kết quả, khác hẳn ví dụ
trước.
::
:::

:::opt
Có — thử chạy đoạn mã y hệt trong Byte cho ra kết quả "6 6", chạy
sạch không báo lỗi gì, nên đây là bằng chứng chương trình hợp lệ
::why
Gần đúng ở việc Byte đúng là CHẠY được đoạn mã này và in ra "6 6" —
một sự thật đã đo, không bịa.

Chỗ lệch: kết luận rút ra từ đó. Byte chỉ kiểm được hai luật hẹp
(dùng sau move thẳng hàng, và hai `&mut` cùng lời gọi) — mượn xen kẽ
qua nhiều câu lệnh nằm HOÀN TOÀN ngoài phạm vi đó, nên với tình
huống này Byte không phân tích gì cả, nó chỉ diễn giải và chạy code
như một trình thông dịch bình thường. "Byte chạy không báo lỗi"
KHÔNG đồng nghĩa "chương trình hợp lệ theo Rust thật" — đây đúng là
lỗ hổng mà cả track này liên tục nhắc: chỉ `rustc` thật (qua
`cargo`) mới có câu trả lời đáng tin ở tình huống ngoài hai luật hẹp.
::
:::

:::opt
Không — nhưng không phải vì `r` và `m` xung đột; lỗi thật sự là
`println!("{} {}", r, x)` không được phép đọc CẢ `r` LẪN `x` trong
cùng một lời gọi `println!`, vì chúng là hai cách khác nhau để trỏ
tới cùng một giá trị
::why
Gần đúng ở việc bạn nghi ngờ đúng dòng `println!` cuối là nơi có vấn
đề — vị trí đó đúng là nơi lỗi xảy ra.

Chỗ lệch: không có luật nào cấm một lời gọi `println!` đọc cả một
tham chiếu (`r`) lẫn biến gốc (`x`) cùng lúc, MIỄN LÀ không có mượn
khả biến nào đang "sống" chồng lên chúng. Vấn đề thật không nằm ở
việc đọc hai thứ cùng lúc trong `println!` — nó nằm ở việc `m` (mượn
SỬA) đã xuất hiện và sửa `x` TRƯỚC dòng này, trong khi `r` (mượn ĐỌC)
vẫn còn cần dùng tới ở đây. Đó là xung đột đọc-trong-lúc-sửa, đúng
mã `E0502`.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không phải dòng nào đứng trước dòng nào. Là ai CÒN DÙNG TỚI, ở đâu.
Byte không nhìn được điều đó — nhưng giờ bạn đã biết cách tự nhìn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Track này đã đi qua move (một chủ, chuyển quyền), rồi mượn đọc/sửa
(nhiều chủ được xem, một chủ được sửa), và giờ là vùng sống (ai còn
"sống" quyết định điều gì hợp lệ). Đó là toàn bộ nền tảng.

Bài sau không giới thiệu luật mới nào cả — nó chỉ hỏi: bạn có GHÉP
LẠI được move và mượn, đúng chỗ, trong MỘT chương trình nhỏ không?
::::

::::checkpoint{mastery=0.8}
::::
