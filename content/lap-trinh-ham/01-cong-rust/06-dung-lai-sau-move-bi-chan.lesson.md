---
id: lap-trinh-ham.cong-rust.dung-lai-sau-move-bi-chan
title: "Dùng lại sau move bị chặn TRƯỚC KHI CHẠY"
summary: "Dùng `s` sau khi đã move sang `t` bị Rust từ chối biên dịch (BR0530) — đối lập kiểu số nguyên (Copy), dùng lại sau gán vẫn chạy bình thường. Bài code có chấm điểm sống ĐẦU TIÊN dùng move-check thật."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 6
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.move-after-use-blocked]
requires: [rs.move-assign]
concepts: [rs.move-after-use-blocked]
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
Bài trước né chuyện dùng lại tên cũ sau move. Hôm nay không né nữa —
Byte thử thật, và cho bạn xem chính xác Rust nói gì.
::::

::::explain{#dung-lai-bi-chan}
`let t = s;` chuyển quyền sở hữu từ `s` sang `t` (bài trước). Sau dòng
đó, `s` không còn là chủ của gì cả. Vậy nếu dòng tiếp theo vẫn viết
`println!("{}", s);` — dùng lại cái tên đã mất quyền chủ — chuyện gì
xảy ra?

Rust không đợi tới lúc chạy để phát hiện. Nó từ chối biên dịch NGAY,
với một chẩn đoán chỉ đích danh cả hai dòng liên quan: dòng nào chuyển
quyền đi, và dòng nào cố dùng lại giá trị đã mất chủ. Đây là luật hẹp
đầu tiên trong hai luật mà bộ chấm bài của khoá này kiểm tra được thật
sự (không suy đoán, không mô phỏng nửa vời): dùng lại một biến sau khi
đã move, trên một chuỗi câu lệnh thẳng hàng, không rẽ nhánh.

Có một đối lập đáng chú ý: không phải MỌI kiểu dữ liệu đều bị luật này
chạm tới. Kiểu số nguyên như `i32` mang tính chất **`Copy`** — gán nó
cho một tên khác không phải move, mà tạo hẳn một BẢN SAO độc lập ngay
lập tức. Cả hai tên đều là chủ của hai giá trị khác nhau (cùng nội
dung), nên dùng lại tên cũ sau đó hoàn toàn hợp lệ. `String` không
mang tính chất này — nó luôn move khi gán, không bao giờ tự sao chép.
::::

::::example{#hai-hanh-vi-doi-lap}
Cùng một hình dạng dòng lệnh (`let x = a; let y = x;` rồi in cả hai),
áp dụng lên hai kiểu khác nhau — kết quả khác hẳn nhau:

```rust title=readonly
fn main() {
    let a: i32 = 5;
    let b = a;
    println!("a = {}, b = {}", a, b);
}
```

```text title=readonly
a = 5, b = 5
```

`i32` là `Copy` — `b = a` tạo bản sao, `a` vẫn còn nguyên, dùng lại
được bình thường. Giờ đúng hình dạng đó, nhưng với `String`:

```rust title=readonly
fn main() {
    let ten = String::from("Lan");
    let chu_moi = ten;
    println!("Chủ mới: {}", ten);
}
```

```text title=readonly
(không biên dịch được)

lỗi [BR0530]: `ten` đã bị chuyển quyền sở hữu đi nơi khác
 --> dòng 4:29
  |
4 |     println!("Chủ mới: {}", ten);
  |                             ^^^ dùng lại ở đây thì không còn giá trị nữa
  |
 --> dòng 3:19
  |
3 |     let chu_moi = ten;
  |                   --- quyền sở hữu bị chuyển đi tại đây
  |
  vì sao: Rust cho mỗi giá trị đúng MỘT chủ sở hữu. Khi bạn gán nó
  sang biến khác hoặc truyền vào hàm, chủ cũ mất quyền — nhờ luật này
  Rust không cần bộ dọn rác mà vẫn không bao giờ dùng nhầm bộ nhớ đã
  giải phóng.
  cách sửa: nếu muốn giữ cả hai, hãy nhân bản: `ten.clone()`
  cách sửa: hoặc chỉ mượn thay vì lấy hẳn: `&ten`
```

Chẩn đoán chỉ ra CẢ HAI vị trí: nơi quyền sở hữu bị chuyển đi (dòng 3)
và nơi bạn cố dùng lại nó (dòng 4). Đây không phải lỗi lúc chạy —
chương trình còn chưa kịp chạy dòng nào, Rust đã từ chối biên dịch toàn
bộ. Hai gợi ý cách sửa ở cuối (`.clone()` và `&ten`) là đúng hai công
cụ hai bài sắp tới sẽ đào sâu.
::::

::::predict{#dong-nao-bi-chan commitOnce}
Byte viết đoạn mã sau, KHÔNG chạy thử:

```rust
fn main() {
    let diem: i32 = 90;
    let diem_sao_chep = diem;
    println!("Bản gốc: {}", diem);

    let ghi_chu = String::from("Giỏi");
    let ghi_chu_moi = ghi_chu;
    println!("Ghi chú: {}", ghi_chu);
}
```

**Trước khi đọc đáp án**, dòng nào bị Rust từ chối biên dịch, và vì
sao?

:::opt{correct}
Dòng cuối (`println!("Ghi chú: {}", ghi_chu);`) — `ghi_chu` là
`String`, đã move sang `ghi_chu_moi` ở dòng trước, nên dùng lại nó bị
chặn (BR0530). Dòng `println!("Bản gốc: {}", diem);` không sao vì
`i32` là `Copy` — `diem` không hề move khi gán cho `diem_sao_chep`
:::

:::opt
Dòng `println!("Bản gốc: {}", diem);` — `diem` đã được gán cho
`diem_sao_chep` ở dòng trước, nên nó cũng mất quyền chủ giống `String`
::why
Gần đúng ở việc bạn áp dụng đúng luật "gán thì cũ mất quyền" — luật đó
có thật, chỉ là không phải cho MỌI kiểu.

Chỗ lệch: `i32` mang tính chất `Copy` — `let diem_sao_chep = diem;`
tạo một BẢN SAO độc lập, không move. `diem` vẫn còn nguyên quyền chủ
sau dòng đó, dùng lại hoàn toàn hợp lệ. Chỉ `String` (và các kiểu không
`Copy` khác) mới move khi gán.
::
:::

:::opt
Cả hai dòng in `diem` và `ghi_chu` đều bị chặn — vì cả hai đều được gán
cho một tên khác trước khi in lại, không phân biệt kiểu
::why
Gần đúng ở việc cả hai dòng đúng là có hình dạng giống nhau: gán rồi in
lại tên cũ.

Chỗ lệch: hình dạng dòng lệnh giống nhau không có nghĩa hành vi giống
nhau — kiểu dữ liệu mới là thứ quyết định. `i32` là `Copy` nên không
move; `String` không phải `Copy` nên move. Chỉ dòng in `ghi_chu` (sau
`String`) bị chặn thật.
::
:::

:::opt
Không dòng nào bị chặn — Rust chỉ từ chối khi bạn cố SỬA giá trị sau
move, còn chỉ ĐỌC (như `println!`) luôn được phép
::why
Gần đúng ở việc bạn phân biệt "đọc" và "sửa" — một phân biệt thật sự
tồn tại trong Rust (mượn `&` so với `&mut`, học ở cụm sau).

Chỗ lệch: luật move không quan tâm bạn định ĐỌC hay SỬA giá trị sau đó
— nó chặn NGAY tại việc dùng lại cái tên đã mất quyền chủ, bất kể mục
đích gì. `println!("Ghi chú: {}", ghi_chu)` chỉ đọc, nhưng vẫn bị
`BR0530` thật.
::
:::
::::

::::code{#chu-moi-la-ai}
Một số điểm (kiểu `Copy`, dùng lại thoải mái) và một cái tên (kiểu
`String`, move khi gán). Điền chỗ trống để in đúng chủ MỚI của cái tên
— không phải chủ cũ đã mất quyền.

```rust title=starter
fn chu_hien_tai() -> String {
    let so_dem: i32 = 5;
    let so_dem_2 = so_dem;
    println!("Bản gốc: {}, bản sao: {}", so_dem, so_dem_2);

    let ten = String::from("Lan");
    let chu_moi = ten;
    let hien_thi = ___;
    hien_thi
}
```

```rust title=solution
fn chu_hien_tai() -> String {
    let so_dem: i32 = 5;
    let so_dem_2 = so_dem;
    println!("Bản gốc: {}, bản sao: {}", so_dem, so_dem_2);

    let ten = String::from("Lan");
    let chu_moi = ten;
    let hien_thi = chu_moi;
    hien_thi
}
```

```rust title=test
fn main() {
    let ten_hien_thi = chu_hien_tai();
    println!("Chủ mới: {}", ten_hien_thi);
    assert_eq!(
        ten_hien_thi, "Lan",
        "chu_hien_tai phải trả về đúng giá trị đã move từ ten (qua chu_moi)"
    );
}
```

:::hints
- kind: attention
  body: ten đã move sang chu_moi ở dòng trước. Chỗ trống phải lấy giá trị từ NGƯỜI CHỦ HIỆN TẠI, không phải từ ten — ten không còn quyền chủ nữa.
- kind: strategy
  body: Sau 'let chu_moi = ten;', chu_moi mới là chủ hợp lệ của giá trị "Lan". Gán chỗ trống bằng đúng cái tên đó.
- kind: one-line
  body: 'Chỗ trống là: chu_moi'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Chủ mới: Lan"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không phải một quy ước lịch sự — một luật trình biên dịch thật sự
kiểm, chặn TRƯỚC KHI chương trình kịp chạy dòng nào. Dùng lại `chu_moi`
đúng, tránh nhắc `ten` sau khi nó đã hết quyền chủ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hai gợi ý cách sửa mà chẩn đoán `BR0530` đưa ra ở ví dụ trên là
`ten.clone()` và `&ten`. Bạn vừa thấy `.clone()` xuất hiện — nhưng chưa
biết nó làm gì thật sự. Nếu bạn CẦN cả `ten` lẫn `chu_moi` cùng dùng
được sau đó — không đánh đổi cái nào — có cách nào không, hay Rust luôn
buộc bạn chọn đúng một trong hai?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
