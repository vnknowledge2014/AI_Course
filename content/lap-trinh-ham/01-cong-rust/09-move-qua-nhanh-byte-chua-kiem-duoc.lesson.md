---
id: lap-trinh-ham.cong-rust.move-qua-nhanh-byte-chua-kiem-duoc
title: "Move qua nhánh if/vòng lặp — Byte chưa kiểm được, KHÔNG có nghĩa Rust cho phép"
summary: "Move một biến bên trong thân if, dùng lại nó sau khối if — byte-rust trả về ChuaHoTro, một kết cục THỨ BA, không phải lỗi của người học nhưng cũng không phải xanh. rustc thật thì từ chối (E0382): bài dạy đúng điều rustc nói, không phải điều Byte nói."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 9
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.move-through-branch-unchecked]
requires: [rs.move-into-function]
concepts: [rs.move-through-branch-unchecked]
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
Bài trước hỏi: nếu move xảy ra BÊN TRONG một khối `if`, Byte có còn
kiểm tra được rõ ràng như trước không? Câu trả lời thành thật: không
luôn luôn — và hôm nay Byte nói thẳng giới hạn của chính mình.
::::

::::explain{#gioi-han-cua-byte}
Mọi ví dụ move từ đầu track tới giờ nằm trên một chuỗi dòng lệnh THẲNG
HÀNG: dòng này chạy xong mới tới dòng kia, không rẽ nhánh. Byte (bộ
chấm bài đang chạy phía sau mỗi bài) chỉ kiểm được move-check chắc chắn
trong tình huống đó — vì xác định "dòng nào chạy trước, dòng nào chạy
sau" ở mã thẳng hàng không cần đoán gì cả.

Đưa move vào bên TRONG một khối `if`, rồi dùng lại biến đó SAU khối
`if`, mọi chuyện phức tạp hơn hẳn. Khối `if` có thể chạy hoặc không
chạy, tuỳ điều kiện — xác định move "có thật sự xảy ra hay không" ở
điểm dùng lại phía sau đòi hỏi phân tích toàn bộ **luồng điều khiển**
(control flow) của chương trình, không chỉ đọc tuần tự từng dòng.

Byte CỐ TÌNH không làm việc đó. Một bản kiểm tra "nửa vời" cho luồng
điều khiển sẽ từ chối OAN nhiều chương trình mà `rustc` thật cho phép —
tức dạy bạn một luật KHÔNG TỒN TẠI. Thay vì liều đoán, Byte thú nhận
thẳng: "chưa kiểm được", một kết cục THỨ BA, khác hẳn "đúng" và "sai".
Đây KHÔNG PHẢI lỗi của bạn — nhưng cũng không phải một dấu xanh. Nó là
một khoảng Byte biết mình không đủ sức phán quyết.
::::

::::example{#byte-thu-nhan}
Cùng hình dạng move đã quen thuộc, chỉ khác một chỗ: dòng move nằm bên
TRONG một khối `if`.

```rust title=readonly
fn main() {
    let s = String::from("hi");
    if true {
        let t = s;
        println!("{}", t);
    }
    println!("{}", s);
}
```

```text title=readonly
(Byte KHÔNG kết luận đúng/sai)

chưa hỗ trợ [BR0531]: Byte chưa kiểm được luật mượn cho `s` trong tình
huống này
 --> dòng 7:20
  |
7 |     println!("{}", s);
  |                    ^ biến này được dùng ở đây
  |
 --> dòng 4:17
  |
4 |         let t = s;
  |                 - và bị chuyển đi bên trong một nhánh hoặc vòng lặp
  |
  vì sao: Khi phép chuyển quyền sở hữu nằm trong `if`, `match` hay
  vòng lặp, việc xác định nó có thực sự xảy ra hay không cần phân tích
  toàn bộ luồng điều khiển. Byte cố tình không làm việc đó, vì làm nửa
  vời sẽ từ chối cả những chương trình mà `rustc` cho phép — tức dạy bạn
  một luật không tồn tại.
  cách sửa: Mở bản Desktop và bấm “Đối chiếu với cargo” để có câu trả
  lời chính xác từ compiler thật
```

Đọc kỹ: đây KHÔNG phải chẩn đoán "lỗi" như `BR0530` ở các bài trước —
nó mang nhãn "chưa hỗ trợ", không phải "lỗi". Byte không nói mã này
sai. Byte cũng không nói mã này đúng. Byte nói: "tôi không đủ sức phán
quyết chỗ này, đừng tin dấu xanh hay đỏ nào của tôi ở đây."

Cùng luật áp dụng khi move nằm bên trong THÂN một vòng lặp thay vì
`if` — Byte trả một chẩn đoán khác (mã `BR0532`, cũng mang nhãn "chưa
hỗ trợ"), cùng một lý do gốc: xác định vòng lặp chạy bao nhiêu lần, và
move có lặp lại hay không, cũng cần phân tích luồng điều khiển.
::::

::::predict{#rustc-noi-gi commitOnce}
Ví dụ trên, Byte đã nói thẳng: "chưa kiểm được". Nhưng `rustc` — trình
biên dịch Rust THẬT, không phải Byte — luôn có một câu trả lời, vì nó
không né tránh phân tích luồng điều khiển.

**Trước khi đọc đáp án**, nếu đưa đúng đoạn mã ở ví dụ trên cho `rustc`
thật biên dịch, nó có chấp nhận không?

:::opt{correct}
Không — `rustc` từ chối thật (mã lỗi `E0382`). `s` bị move vào `t`
ngay bên trong thân `if`; ở điểm `println!("{}", s)` sau khối `if`,
`rustc` không thể đảm bảo `s` còn giá trị (vì nhánh `if` — dù ở đây
luôn chạy — về nguyên tắc CÓ THỂ không chạy), nên nó từ chối an toàn.
"Byte chưa kiểm được" không có nghĩa "Rust cho phép" — chỉ có nghĩa
Byte không đủ sức tính ra câu trả lời mà `rustc` vẫn tính được
:::

:::opt
Có, `rustc` chấp nhận — vì Byte đã nói "chưa hỗ trợ", tức là Byte
không tìm thấy lỗi nào, nên coi như mã hợp lệ
::why
Gần đúng ở việc bạn đọc đúng chữ "chưa hỗ trợ" không phải chữ "lỗi" —
quan sát cẩn thận, đáng khen.

Chỗ lệch: "chưa hỗ trợ" (`ChuaHoTro`) là một kết cục THỨ BA, tách hẳn
khỏi cả "đúng" lẫn "sai" — nó có nghĩa "Byte không đủ sức phán quyết",
không có nghĩa "Byte đã kiểm và thấy ổn". `rustc` thật vẫn có câu trả
lời riêng, độc lập với việc Byte kiểm được hay không — và câu trả lời
đó là từ chối (`E0382`), đã xác minh bằng cách biên dịch thật.
::
:::

:::opt
Có, `rustc` chấp nhận — vì điều kiện `if true` luôn luôn đúng, nên
trình biên dịch biết chắc khối `if` chắc chắn chạy, move chắc chắn xảy
ra, không có gì mơ hồ để từ chối
::why
Gần đúng ở việc bạn suy luận đúng: với `if true`, khối `if` THẬT SỰ
luôn chạy — không sai chút nào về mặt logic của chương trình.

Chỗ lệch: bộ kiểm tra mượn của `rustc` (borrow checker) không chạy thử
giá trị điều kiện lúc phân tích kiểu này — nó xét cấu trúc `if`/`else`
một cách tổng quát, coi CẢ HAI nhánh đều "có thể chạy hoặc không", bất
kể điều kiện là hằng số `true` hay một biểu thức phức tạp. Chính vì
không đặc cách cho trường hợp "trông có vẻ luôn đúng", `rustc` vẫn từ
chối — đã xác minh thật bằng cách biên dịch đoạn mã này.
::
:::

:::opt
Không thể biết được — không có cách nào chắc chắn "chưa hỗ trợ" của
Byte tương ứng với "chấp nhận" hay "từ chối" của rustc, phải thử cả
hai trường hợp mới biết
::why
Gần đúng ở việc bạn không đoán liều — cẩn trọng đúng chỗ khi gặp một
kết cục mới lạ như `ChuaHoTro`.

Chỗ lệch: dù Byte không tính ra được câu trả lời cho CHÍNH XÁC đoạn mã
này, không có nghĩa không ai biết được. `rustc` thật đã biên dịch đúng
đoạn mã này (không phải suy đoán) và cho kết quả rõ ràng: từ chối,
`E0382`. "Byte không biết" và "không ai biết" là hai chuyện khác nhau —
`rustc` luôn có câu trả lời, chỉ là Byte không tái tạo được nó trên
mọi trường hợp.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Byte không giả vờ biết những gì nó không biết. "Chưa kiểm được" là một
câu trả lời trung thực — và bạn vừa học cách đọc đúng nó, thay vì hiểu
lầm thành "được phép".
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Track này còn một tham số đặc biệt chưa được nhắc kỹ: `self` — tham số
đầu tiên của một method, xuất hiện từ bài `struct`/`impl` đầu track
nhưng luôn viết kèm dấu `&` (`&self`) mà chưa giải thích vì sao. Bây
giờ bạn đã biết move là gì. Nếu một method viết `self` KHÔNG có dấu
`&` — nhận `self` theo giá trị — chuyện gì xảy ra với đối tượng gọi
method đó sau khi gọi xong?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
