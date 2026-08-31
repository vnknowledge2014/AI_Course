---
id: lap-trinh-ham.cong-rust.gan-la-chuyen-quyen-so-huu
title: "Gán một giá trị là CHUYỂN quyền sở hữu, không phải sao chép"
summary: "`let s = String::from(\"hi\"); let t = s;` — quyền sở hữu CHUYỂN từ s sang t (move), không tạo bản sao. Không giống Python, nơi hai tấm thẻ tự do cùng buộc một nồi."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 5
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.move-assign]
requires: [rs.ownership-intro]
concepts: [rs.move-assign]
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
Bài trước hỏi: gán một giá trị đang sống sang một cái tên khác thì
quyền chủ sở hữu đi đâu? Byte trả lời ngay — và đây là chỗ Rust rẽ hẳn
khỏi Python.
::::

::::explain{#gan-la-move}
Ở Python, `b = a` chỉ chép thêm một tấm thẻ mới, buộc vào ĐÚNG cái nồi
`a` đang buộc. Sau dòng đó, cả `a` lẫn `b` cùng là "chủ" theo nghĩa
Python hiểu — không ai mất quyền gì, cả hai cùng dùng được, mãi mãi,
cho tới khi một trong hai tự gán lại.

Rust không cho phép điều đó xảy ra, vì luật bài trước đã chốt: một giá
trị có ĐÚNG MỘT chủ tại một thời điểm — không phải "tối đa một chủ nếu
bạn cẩn thận", mà là một luật cứng. Vậy `let t = s;` phải làm gì với
quyền sở hữu của `s`? Nó không thể vừa giữ nguyên `s` là chủ, vừa biến
`t` thành chủ — hai chủ cùng lúc là điều luật cấm.

Rust giải quyết bằng cách **chuyển** (move) quyền sở hữu: `let t = s;`
lấy quyền chủ sở hữu ra khỏi `s`, trao hẳn cho `t`. Sau dòng đó, `t` là
chủ MỚI duy nhất. `s` không còn là chủ của bất cứ gì nữa — không phải
vì giá trị bị xoá hay đổi chỗ trong bộ nhớ (dữ liệu thật vẫn nằm nguyên
một chỗ), mà vì tấm thẻ `s` không còn được công nhận là chủ của nó.

Đây KHÔNG phải sao chép. Không có bản sao thứ hai nào được tạo ra —
chỉ có một giá trị, và quyền gọi nó "của tôi" vừa đổi từ `s` sang `t`.
Bài này chỉ dừng ở việc thấy move XẢY RA; chuyện gì xảy ra nếu bạn cố
dùng lại `s` sau đó là chủ đề của bài sau.
::::

::::example{#move-xay-ra}
Byte tạo một giá trị, move nó sang một tên khác, rồi CHỈ dùng cái tên
mới:

```rust title=readonly
fn main() {
    let s = String::from("hi");
    let t = s;
    println!("{}", t);
}
```

```text title=readonly
hi
```

Chạy sạch, không lỗi gì. `t` in ra đúng nội dung `s` từng giữ — vì dữ
liệu không hề bị sao chép hay mất, chỉ có QUYỀN SỞ HỮU đổi tên người
giữ. `s` không xuất hiện ở dòng nào sau `let t = s;` trong đoạn này, để
tránh chạm ngay vào chuyện "dùng `s` sau khi đã move" — Rust có phản
ứng RẤT rõ ràng với chuyện đó, và bài kế tiếp mổ xẻ đúng nó.
::::

::::predict{#move-hay-sao-chep commitOnce}
Byte viết đoạn mã sau, KHÔNG chạy thử:

```rust
fn main() {
    let don_hang = String::from("DH-2024");
    let don_hang_moi = don_hang;
    println!("{}", don_hang_moi);
}
```

**Trước khi đọc đáp án**, sau dòng `let don_hang_moi = don_hang;`, ai
là chủ sở hữu của giá trị `"DH-2024"`?

:::opt{correct}
Chỉ `don_hang_moi` — quyền sở hữu đã CHUYỂN từ `don_hang` sang
`don_hang_moi`. Không có bản sao nào được tạo, và `don_hang` không còn
là chủ của gì cả sau dòng đó
:::

:::opt
Cả `don_hang` lẫn `don_hang_moi` — giống Python, cả hai tên giờ cùng
trỏ tới một giá trị, ai dùng cũng được
::why
Gần đúng ở việc đây đúng là cách Python xử lý một dòng gán tương tự —
`don_hang_moi = don_hang` ở Python sẽ cho cả hai tên cùng dùng được.

Chỗ lệch: Rust không phải Python. Luật "một giá trị, đúng một chủ tại
một thời điểm" (bài trước) buộc `let don_hang_moi = don_hang;` phải
CHUYỂN quyền, không phải nhân đôi nó. Sau dòng này, chỉ `don_hang_moi`
còn là chủ.
::
:::

:::opt
Chỉ `don_hang` — dòng gán chỉ đọc giá trị của `don_hang` để in thử qua
`don_hang_moi`, không hề đổi ai là chủ thật sự
::why
Gần đúng ở việc bạn nghi ngờ tên BÊN TRÁI phép gán mới là chủ "chính
thức" — một trực giác dễ hiểu nếu quen nhìn `=` như "đặt tên tạm".

Chỗ lệch: `let don_hang_moi = don_hang;` không phải một phép đọc tạm —
nó CHUYỂN quyền sở hữu hẳn sang cái tên MỚI được khai (`don_hang_moi`).
Cái tên cũ (`don_hang`) mất quyền chủ hoàn toàn kể từ dòng đó, không
phải giữ lại.
::
:::

:::opt
Không ai cả — giá trị bị dọn ngay khi gán, vì Rust không cho hai tên
cùng nhắc tới một giá trị trong bất cứ trường hợp nào
::why
Gần đúng ở việc bạn nhớ đúng "hai tên cùng là chủ" bị cấm — phản xạ
đúng hướng sau bài trước.

Chỗ lệch: bị cấm không có nghĩa giá trị biến mất. Quyền sở hữu chỉ
CHUYỂN sang một chủ khác (`don_hang_moi`), giá trị vẫn còn nguyên, vẫn
in ra được — dòng `println!` trong đoạn mã này thật sự chạy và in ra
`DH-2024`, không hề bị dọn.
::
:::
::::

::::code{#chuyen-ve}
Một tấm vé được tạo ra cho Lan, sau đó chuyển hẳn quyền sở hữu sang
Nam. Điền chỗ trống để Nam thật sự trở thành chủ mới, giữ nguyên nội
dung tấm vé.

```rust title=starter
fn tao_ve_cho_nam() -> String {
    let ve_cua_lan = String::from("Vé số 07");
    let ve_cua_nam = ___;
    ve_cua_nam
}
```

```rust title=solution
fn tao_ve_cho_nam() -> String {
    let ve_cua_lan = String::from("Vé số 07");
    let ve_cua_nam = ve_cua_lan;
    ve_cua_nam
}
```

```rust title=test
fn main() {
    let ve = tao_ve_cho_nam();
    println!("Nam đang giữ: {}", ve);
    assert_eq!(
        ve, "Vé số 07",
        "tao_ve_cho_nam phải trả về đúng giá trị đã move từ ve_cua_lan"
    );
}
```

:::hints
- kind: attention
  body: Chỗ trống phải khiến ve_cua_nam trở thành chủ MỚI của đúng giá trị mà ve_cua_lan vừa tạo ra — không phải một chuỗi mới, tự viết tay.
- kind: strategy
  body: Chuyển quyền sở hữu bằng cách gán thẳng cái tên đang giữ giá trị đó. Chỗ trống chỉ cần đúng một cái tên đã có sẵn phía trên.
- kind: one-line
  body: 'Chỗ trống là: ve_cua_lan'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Nam đang giữ: Vé số 07"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Quyền sở hữu vừa đổi chủ, không sao chép gì cả. Đúng như luật bài
trước nói: một giá trị, một chủ, tại một thời điểm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài này bạn tránh né một chuyện: không dòng nào ở đây dùng lại
`ve_cua_lan` SAU khi nó đã move sang `ve_cua_nam`. Nếu bạn cứ thử — cứ
in `ve_cua_lan` ở dòng cuối, sau khi nó đã hết quyền chủ — điều gì xảy
ra? Rust có lặng lẽ cho qua, có in ra giá trị cũ, hay có phản ứng gì
khác?

Bài sau trả lời trực tiếp — và lần này KHÔNG né nữa.
::::

::::checkpoint{mastery=0.8}
::::
