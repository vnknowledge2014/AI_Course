---
id: lap-trinh-ham.cong-rust.clone-giu-lai-cho-ca-hai
title: "`.clone()` giữ lại quyền sở hữu cho CẢ HAI"
summary: "`let t = s.clone();` tạo một BẢN SAO thật sự — s vẫn còn chủ, t là chủ của một giá trị khác, cùng nội dung, khác vùng nhớ. Đánh đổi: clone tốn bộ nhớ và thời gian sao chép, không phải lựa chọn luôn đúng."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 7
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.clone]
requires: [rs.move-after-use-blocked]
concepts: [rs.clone]
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
Bài trước hỏi: nếu bạn CẦN cả hai tên cùng dùng được, có cách nào
không? Có. Hôm nay Byte giới thiệu nó.
::::

::::explain{#clone-tao-ban-sao-that}
Chẩn đoán `BR0530` ở bài trước gợi ý hai lối ra: `&ten` (mượn — cụm bài
sau mới đào) và `ten.clone()`. Bài này đào lối thứ hai.

**`.clone()`** là một phương thức có sẵn trên `String` (và nhiều kiểu
khác) làm đúng một việc: dựng một giá trị MỚI, ở một vùng nhớ MỚI, có
nội dung giống hệt bản gốc. Khác hẳn `let t = s;` (move — chỉ CHUYỂN
quyền chủ, không tạo gì mới), `let t = s.clone();` tạo hẳn một bản sao
độc lập. Kết quả: `s` vẫn còn là chủ của giá trị gốc, `t` là chủ của
một giá trị khác — hai giá trị riêng biệt, tình cờ giống nhau về nội
dung. Cả hai tên đều hợp lệ, dùng lại thoải mái sau đó, không đụng luật
`BR0530` nào cả.

Đây không phải "cách đúng luôn luôn nên dùng". Sao chép một `String` có
nghĩa là cấp phát thêm bộ nhớ và chép từng byte — tốn thời gian, tốn
chỗ. Với một chuỗi ngắn, chuyện đó không đáng bận tâm; với dữ liệu lớn,
gọi `.clone()` ở một vòng lặp chạy hàng triệu lần có thể chậm đi rõ
rệt. `.clone()` là một CÔNG CỤ — dùng khi bạn THẬT SỰ cần hai bản độc
lập, không phải phản xạ mặc định mỗi khi gặp `BR0530`.
::::

::::example{#hai-ten-hai-gia-tri}
Byte tạo một tên, clone nó, rồi dùng CẢ HAI tên sau đó:

```rust title=readonly
fn main() {
    let ten = String::from("Mai");
    let ten_sao_chep = ten.clone();

    println!("Bản gốc: {}", ten);
    println!("Bản sao: {}", ten_sao_chep);
}
```

```text title=readonly
Bản gốc: Mai
Bản sao: Mai
```

Chạy sạch — không lỗi `BR0530` nào, dù `ten` được dùng lại SAU dòng
gán, đúng chuyện bị chặn ở bài trước nếu không có `.clone()`. Hai dòng
in ra cùng nội dung `"Mai"`, nhưng đó là hai giá trị khác nhau nằm ở
hai chỗ khác nhau trong bộ nhớ — sửa một bên (nếu `String` cho sửa tại
chỗ) sẽ không ảnh hưởng bên kia. So sánh trực tiếp với hình dạng KHÔNG
có `.clone()`:

```
let ten_sao_chep = ten;          // move — ten mất quyền chủ
let ten_sao_chep = ten.clone();  // clone — ten vẫn còn quyền chủ
```

Một dấu `.clone()` là toàn bộ khác biệt giữa "mất quyền" và "giữ được
cả hai".
::::

::::predict{#can-clone-khong commitOnce}
Byte viết đoạn mã sau, KHÔNG chạy thử:

```rust
fn main() {
    let ma_so = String::from("MS-01");
    let ma_so_du_phong = ma_so.clone();
    let ma_so_thu_hai = ma_so;
    println!("{} {} {}", ma_so_du_phong, ma_so_thu_hai, ma_so);
}
```

**Trước khi đọc đáp án**, đoạn mã này có biên dịch được không, và vì
sao?

:::opt{correct}
Không — `println!` cuối dùng lại `ma_so`, nhưng `ma_so` đã move sang
`ma_so_thu_hai` ở dòng `let ma_so_thu_hai = ma_so;` (không có
`.clone()` ở dòng đó). `ma_so_du_phong` (được clone) và `ma_so_thu_hai`
(được move) đều hợp lệ để đọc — vấn đề chỉ nằm ở việc đọc lại `ma_so`
:::

:::opt
Có, biên dịch được — `ma_so` đã được `.clone()` một lần ở dòng đầu, nên
nó được phép dùng lại bao nhiêu lần cũng được sau đó
::why
Gần đúng ở việc bạn nhớ đúng `.clone()` giữ quyền chủ cho tên gốc — một
lượt gọi `.clone()` ĐÚNG là không làm `ma_so` mất quyền.

Chỗ lệch: `.clone()` chỉ bảo vệ `ma_so` khỏi lượt gán TẠO RA bản sao đó
(`let ma_so_du_phong = ma_so.clone();`). Nó không "miễn nhiễm" cho mọi
lượt gán sau này. Dòng kế tiếp, `let ma_so_thu_hai = ma_so;`, KHÔNG gọi
`.clone()` — đó là một move bình thường, và move đó mới là thứ khiến
`ma_so` mất quyền chủ.
::
:::

:::opt
Không — nhưng lỗi nằm ở dòng `let ma_so_du_phong = ma_so.clone();`, vì
không thể gọi `.clone()` trên một giá trị sẽ còn bị move ở dòng sau
::why
Gần đúng ở việc bạn nghi ngờ đúng khu vực có vấn đề — gần dòng
`.clone()` và dòng move kế tiếp.

Chỗ lệch: `.clone()` không quan tâm chuyện gì xảy ra với `ma_so` ở
NHỮNG DÒNG SAU nó — nó chỉ tạo bản sao tại đúng thời điểm được gọi,
không đặt điều kiện gì về tương lai. Dòng `.clone()` hoàn toàn hợp lệ.
Lỗi thật nằm ở dòng in cuối, đọc lại `ma_so` sau khi nó đã move.
::
:::

:::opt
Có, biên dịch được — `ma_so_thu_hai = ma_so` không phải move thật sự,
vì `ma_so` đã có một bản sao (`ma_so_du_phong`) tồn tại rồi nên Rust tự
hiểu là còn "dư" một bản để move tiếp mà không mất `ma_so`
::why
Gần đúng ở việc bạn cảm thấy có bản sao ở đâu đó "cứu" được `ma_so` —
trực giác không sai khi mới học `.clone()`, chỉ lệch chỗ áp dụng.

Chỗ lệch: mỗi lượt `let x = y;` (không `.clone()`) LUÔN LÀ move của
chính `y`, không quan tâm `y` từng có bao nhiêu bản sao ở nơi khác.
`ma_so_du_phong` là một giá trị RIÊNG, không "bù" gì cho `ma_so` cả.
`let ma_so_thu_hai = ma_so;` move `ma_so` thật, và dòng in cuối dùng
lại nó vẫn bị chặn thật, đúng mã `BR0530`.
::
:::
::::

::::code{#giu-ca-hai}
Một cái tên cần dùng ở hai chỗ SAU cùng — không được để chỗ nào mất
quyền chủ. Điền chỗ trống để cả `ten` lẫn `ten_sao_chep` đều còn dùng
được.

```rust title=starter
fn nhan_ban_sao() -> String {
    let ten = String::from("Mai");
    let ten_sao_chep = ___;
    format!("{} va {}", ten, ten_sao_chep)
}
```

```rust title=solution
fn nhan_ban_sao() -> String {
    let ten = String::from("Mai");
    let ten_sao_chep = ten.clone();
    format!("{} va {}", ten, ten_sao_chep)
}
```

```rust title=test
fn main() {
    let ket_qua = nhan_ban_sao();
    println!("{}", ket_qua);
    assert_eq!(
        ket_qua, "Mai va Mai",
        "nhan_ban_sao phải trả về đúng cả ten lẫn ten_sao_chep"
    );
}
```

:::hints
- kind: attention
  body: Dòng format! cuối đọc CẢ HAI tên, ten và ten_sao_chep. Một phép gán thường (move) sẽ khiến ten mất quyền chủ, làm dòng format! đó không biên dịch được — chỗ trống cần tránh điều đó.
- kind: strategy
  body: Gọi .clone() trên ten để tạo một bản sao độc lập, thay vì gán thẳng ten. Cú pháp là ten.clone().
- kind: one-line
  body: 'Chỗ trống là: ten.clone()'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Mai va Mai"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai tên, hai giá trị, không tên nào mất quyền. Đúng công cụ cho đúng
lúc — clone khi thật sự cần hai bản.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cho tới giờ, move chỉ xuất hiện ở một chỗ: `let t = s;`, gán một tên
cho một tên khác. Nhưng có một hành động rất quen thuộc khác cũng
"trao" một giá trị đi — gọi một hàm, truyền một biến làm đối số. Ở
Python, truyền một biến vào hàm không hề làm mất quyền gì — hàm và
người gọi cùng nhìn vào một giá trị. Rust có xử lý việc đó giống move
qua `let`, hay hoàn toàn khác?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
