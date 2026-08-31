---
id: lap-trinh-ham.cong-rust.mot-gia-tri-dung-mot-chu
title: "Một giá trị có ĐÚNG MỘT chủ sở hữu"
summary: "Rust đặt một luật lên đúng bức tranh 'cái tên chỉ tới giá trị' mà T3.1 đã dựng: tại một thời điểm, ĐÚNG MỘT cái tên được coi là chủ (owner) của một giá trị. Khi chủ ra khỏi phạm vi, giá trị bị dọn ngay — không cần bộ dọn rác."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 4
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.ownership-intro]
requires: [mem.name-is-reference, rs.enum-match]
concepts: [rs.ownership-intro]
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
Bài trước để lại một câu hỏi: Rust có luật nào lên đúng bức tranh "cái
tên chỉ tới giá trị" mà bạn đã học ở Python không? Câu trả lời là Ý mà
cả track này xoay quanh.
::::

::::explain{#dung-mot-chu}
Nhớ lại bài về tấm thẻ và cái nồi (`mem.name-is-reference`): một cái
tên không GIỮ giá trị, nó chỉ CHỈ TỚI giá trị — và ở Python, nhiều tấm
thẻ có thể cùng buộc vào một cái nồi, tuỳ ý, không luật nào ngăn cả
(`mem.aliasing-explained`). `b = a` chỉ chép thêm một tấm thẻ mới, cái
nồi cũ vẫn còn nguyên, cả hai tấm thẻ cùng dùng được.

Rust giữ nguyên bức tranh "tên chỉ tới giá trị" đó, nhưng đặt thêm một
luật mà Python không có:

> Tại một thời điểm, một giá trị có ĐÚNG MỘT cái tên được coi là **chủ
> sở hữu** (owner) của nó.

"Chủ sở hữu" không phải một khái niệm trừu tượng suông — nó có một hệ
quả cụ thể, đo được: khi cái tên đang là chủ ra khỏi **phạm vi**
(scope — vùng mã giữa một cặp dấu ngoặc `{ }` nơi cái tên đó được khai
báo), giá trị nó đang giữ bị **dọn** (drop) ngay lập tức. Không phải
"có thể bị dọn lúc nào đó" — dọn NGAY tại chỗ đóng ngoặc.

Đây là chỗ khác Python rõ nhất. Python dùng một **bộ dọn rác** (garbage
collector) — một tiến trình chạy nền, âm thầm quét bộ nhớ, tìm những
giá trị không còn tấm thẻ nào trỏ tới, rồi dọn chúng vào một lúc nào đó
sau này, không ai biết chính xác lúc nào. Rust không cần tiến trình
đó: vì mỗi giá trị luôn có ĐÚNG MỘT chủ, và phạm vi của cái tên đó biết
trước, tại lúc biên dịch, chính xác dòng nào giá trị sẽ hết chủ — nên
dọn được ngay tại dòng đó, không cần đoán, không cần quét.
::::

::::example{#pham-vi-va-don}
Một cái tên khai trong khối `{ }`, dùng đúng bên trong khối đó:

```rust title=readonly
fn main() {
    {
        let ten = String::from("Lan");
        println!("Trong khối: {}", ten);
    }
    println!("Hết chương trình.");
}
```

```text title=readonly
Trong khối: Lan
Hết chương trình.
```

`ten` là chủ của giá trị `String::from("Lan")` trong suốt khối `{ }`.
Đóng ngoặc lại — chủ ra khỏi phạm vi — giá trị bị dọn. Chương trình vẫn
chạy tiếp bình thường, chỉ là cái tên `ten` và thứ nó từng giữ không
còn tồn tại nữa.

Giờ thử dùng lại `ten` SAU khi khối đã đóng:

```rust title=readonly
fn main() {
    {
        let ten = String::from("Lan");
        println!("Trong khối: {}", ten);
    }
    println!("Ngoài khối: {}", ten);
}
```

```text title=readonly
(không biên dịch được)

lỗi [BR0340]: không tìm thấy tên `ten`
 --> dòng 6:32
  |
6 |     println!("Ngoài khối: {}", ten);
  |                                ^^^ chưa có gì mang tên này ở đây
  |
  vì sao: Một cái tên chỉ sống trong khối `{}` khai báo ra nó. Ra khỏi
  dấu ngoặc là nó biến mất — kể cả khi dòng đó nằm ngay bên dưới.
```

Rust từ chối biên dịch — không phải vì cú pháp sai, mà vì `ten` không
còn ở đâu để dùng nữa. Đây chính là bằng chứng cụ thể của luật "một
chủ, dọn khi hết phạm vi": không phải một câu mô tả suông, mà một điều
trình biên dịch THẬT SỰ kiểm tra và chặn lại nếu bạn vi phạm.
::::

::::predict{#diem-con-khong commitOnce}
Byte viết đoạn mã sau, KHÔNG chạy thử:

```rust
fn main() {
    let ten_khach = String::from("Mai");
    {
        let diem = 50;
        println!("Điểm: {}", diem);
    }
    println!("Khách: {}", ten_khach);
    println!("Điểm vẫn còn: {}", diem);
}
```

**Trước khi đọc đáp án**, đoạn mã này có biên dịch được không, và vì
sao?

:::opt{correct}
Không — dòng cuối dùng `diem`, nhưng `diem` được khai trong khối `{ }`
đã đóng lại từ ba dòng trước. Chủ của nó (cái tên `diem`) đã ra khỏi
phạm vi, giá trị đã bị dọn, nên `diem` không còn tồn tại để dùng nữa.
`ten_khach` không sao vì nó khai NGOÀI khối, phạm vi trải hết `main`
:::

:::opt
Không — nhưng lỗi nằm ở dòng in `ten_khach`, vì `ten_khach` khai TRƯỚC
khối `{ }` chứa `diem`, nên phạm vi của nó kết thúc ngay khi khối đó mở
ra
::why
Gần đúng ở việc bạn để ý thứ tự khai báo giữa `ten_khach` và khối chứa
`diem` — một quan sát hợp lý khi mới gặp khái niệm phạm vi.

Chỗ lệch: một khối `{ }` lồng bên trong không cắt đứt phạm vi của cái
tên khai TRƯỚC nó, nó chỉ tạo thêm một phạm vi CON bên trong. `ten_khach`
khai ở phạm vi của `main`, trải dài hết toàn bộ hàm — dòng in nó (dòng
7) hoàn toàn hợp lệ. Lỗi thật nằm ở `diem`, dùng SAU khi phạm vi của
chính nó đã đóng.
::
:::

:::opt
Có, biên dịch được — `diem` đã được in ra một lần trong khối rồi, nên
Rust nhớ giá trị đó và cho phép in lại lần nữa ở ngoài
::why
Gần đúng ở việc `diem` đúng là đã được in ra thành công một lần, bên
trong khối của nó — dòng đó không có gì sai.

Chỗ lệch: "đã in một lần" không có nghĩa gì với phạm vi cả. Rust không
ghi nhớ "giá trị này từng tồn tại" để cho dùng lại ngoài phạm vi — khi
khối `{ }` đóng, `diem` bị dọn dứt khoát. Dòng in thứ hai (ngoài khối)
vẫn bị từ chối thật, đúng mã `BR0340`.
::
:::

:::opt
Có, biên dịch được — số nguyên như `diem` (kiểu `i32`) không cần dọn
gì cả, nên phạm vi không áp dụng cho nó, chỉ áp dụng cho các kiểu như
`String`
::why
Gần đúng ở việc bạn nhớ đúng: kiểu số nguyên như `i32` có tính chất
đặc biệt so với `String` (bài sau sẽ đào đúng chuyện này — `Copy`).

Chỗ lệch: tính chất đặc biệt đó nói về chuyện SAO CHÉP lúc gán, không
nói về PHẠM VI. Mọi cái tên, bất kể kiểu gì, đều chỉ sống trong khối
`{ }` khai ra nó — `diem` dù là `i32` vẫn bị `BR0340` khi dùng ngoài
khối, y hệt như nếu nó là `String`.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một chủ, một phạm vi, dọn đúng lúc — không cần chờ ai quét bộ nhớ. Đó
là luật gốc. Mọi luật khác của Rust bạn sắp học đều là hệ quả của đúng
luật này.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy CHỦ ra khỏi phạm vi thì giá trị bị dọn. Nhưng còn một tình
huống khác chưa được nhắc tới: nếu bạn viết `let t = s;` — gán giá trị
của một cái tên ĐANG CÒN SỐNG sang một cái tên khác, không phải ra khỏi
phạm vi — chuyện gì xảy ra với quyền chủ sở hữu? `s` và `t` có cùng là
chủ một lúc không, giống hệt Python cho hai tấm thẻ cùng buộc một nồi?

Bài sau trả lời — và câu trả lời không giống Python chút nào.
::::

::::checkpoint{mastery=0.8}
::::
