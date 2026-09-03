---
id: co-so-du-lieu.ngon-ngu-cua-byte.token-so
title: "Token đầu tiên: đọc một số"
summary: "doc_so(ky_tu: &Vec<char>, bat_dau: usize) gộp các chữ số liên tiếp thành MỘT số nguyên bằng tích luỹ (n = n*10 + chu_so), trả về KetQuaDoc{gia_tri, vi_tri} — struct, KHÔNG tuple (kiểu trả về hàm là tuple tường minh bị Byte báo nhầm lỗi kiểu). So sánh ký tự trực tiếp c >= '0' && c <= '9', không is_ascii_digit."
locale: vi
track: co-so-du-lieu
module: ngon-ngu-cua-byte
order: 1
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 13
teaches: [db.lexer-token]
requires: [db.version-gc]
concepts: [db.lexer-token]
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
Realm mới, ngôn ngữ mới: Rust. Một câu truy vấn (`SELECT tuoi FROM
nguoi`) LÀ một chuỗi ký tự — việc ĐẦU tiên là nhận ra đâu LÀ một con
số.
::::

::::explain{#doc-mot-so}
`doc_so` gộp các chữ số LIÊN tiếp thành MỘT số nguyên, bằng tích luỹ
— KHÔNG dùng `String` để "xây" chuỗi số rồi parse (`String` ở đây
CHỈ đọc, không có `.push`):

```rust title=readonly
struct KetQuaDoc {
    gia_tri: i64,
    vi_tri: usize,
}

fn doc_so(ky_tu: &Vec<char>, bat_dau: usize) -> KetQuaDoc {
    let mut n: i64 = 0;
    let mut i = bat_dau;
    while i < ky_tu.len() && ky_tu[i] >= '0' && ky_tu[i] <= '9' {
        let chu_so = (ky_tu[i] as i64) - ('0' as i64);
        n = n * 10 + chu_so;
        i += 1;
    };
    KetQuaDoc { gia_tri: n, vi_tri: i }
}

fn main() {
    let s = String::from("1234");
    let ky_tu: Vec<char> = s.chars().collect();
    let kq = doc_so(&ky_tu, 0);
    println!("{} {}", kq.gia_tri, kq.vi_tri);
}
```

```text title=readonly
1234 4
```

Mỗi vòng lặp: lấy ký tự tại `i`, đổi nó thành CHỮ SỐ bằng phép trừ mã
ký tự (`'5' as i64` LÀ `53`, trừ `'0' as i64` LÀ `48`, còn lại `5`),
rồi `n = n * 10 + chu_so` — đúng cách viết một số nhiều chữ số bằng
tay: dịch các chữ số CŨ sang trái một hàng, thêm chữ số MỚI vào cuối.
Kết quả trả VỀ qua `KetQuaDoc` — một **struct**, không phải tuple:
kiểu trả về hàm LÀ tuple tường minh (`-> (i64, usize)`) bị chấm nhầm
lỗi kiểu dù giá trị đúng, nên track NÀY dùng struct cho mọi hàm cần
trả nhiều giá trị.
::::

::::example{#dung-lai-truoc-chu-cai}
Gặp ký tự KHÔNG phải chữ số, vòng lặp dừng NGAY — phần còn lại chưa
đọc tới:

```rust title=readonly
let s2 = String::from("42abc");
let ky_tu2: Vec<char> = s2.chars().collect();
let kq2 = doc_so(&ky_tu2, 0);
println!("{} {}", kq2.gia_tri, kq2.vi_tri);
```

```text title=readonly
42 2
```

Điều kiện `ky_tu[i] >= '0' && ky_tu[i] <= '9'` sai NGAY tại `i=2`
(ký tự `'a'`) — vòng `while` dừng, `vi_tri` LÀ `2`: đúng vị trí ký
tự ĐẦU tiên CHƯA đọc, không phải vị trí cuối chuỗi số.
::::

::::predict{#doan-so-0-dau commitOnce}
Chuỗi `"007"` có HAI chữ số `0` Ở đầu:

```rust
let s3 = String::from("007");
let ky_tu3: Vec<char> = s3.chars().collect();
let kq3 = doc_so(&ky_tu3, 0);
println!("{} {}", kq3.gia_tri, kq3.vi_tri);
```

Dòng cuối in ra gì?

:::opt{correct}
`7 3`
:::

:::opt
`007 3` — vì chuỗi GỐC có hai số 0 ở đầu, kết quả phải GIỮ nguyên
hình dạng đó
::why
Gần đúng ở việc bạn nhớ ĐÚNG chuỗi gốc CÓ hai chữ số `0` — một quan
sát chính xác về DỮ liệu đầu vào.

Chỗ lệch: `gia_tri` có kiểu `i64` — một CON SỐ, không phải một
CHUỖI ký tự. Số nguyên KHÔNG hề "nhớ" cách nó được VIẾT ra — `n =
n*10 + chu_so` chạy qua `'0'`,`'0'`,`'7'` cho RA đúng `0`, `0`,
`7` — kết quả CUỐI là `7`, hệt như `007` VÀ `7` LÀ cùng một số
trong toán học.
::
:::

:::opt
Máy báo lỗi — vì bắt đầu một số bằng chữ số `0` (không phải `0` một
mình) LÀ cú pháp không hợp lệ trong nhiều ngôn ngữ
::why
Gần đúng ở việc bạn nhớ ĐÚNG một luật CÓ thật trong một số ngôn ngữ
lập trình (số nguyên literal bắt đầu bằng `0` đôi khi mang Ý nghĩa
đặc biệt, như hệ bát phân).

Chỗ lệch: `doc_so` không hề PHÂN tích cú pháp literal Rust — nó chỉ
đọc CHỮ SỐ từ một `Vec<char>` do NGƯỜI dùng cung cấp, gộp bằng phép
tính số học ĐƠN giản. Không có `raise`/panic nào cho trường hợp
NÀY — `0` dẫn đầu hoàn toàn hợp lệ, chỉ đơn giản không đóng góp GÌ
vào giá trị cuối (nhân `0` với `10` vẫn LÀ `0`).
::
:::
::::

::::code{#viet_doc_so}
Hoàn thiện `doc_so` — gộp các chữ số bằng tích luỹ đúng cách (dịch
trái rồi cộng chữ số mới).

```rust title=starter
struct KetQuaDoc {
    gia_tri: i64,
    vi_tri: usize,
}

fn doc_so(ky_tu: &Vec<char>, bat_dau: usize) -> KetQuaDoc {
    let mut n: i64 = 0;
    let mut i = bat_dau;
    while i < ky_tu.len() && ky_tu[i] >= '0' && ky_tu[i] <= '9' {
        let chu_so = (ky_tu[i] as i64) - ('0' as i64);
        ___
        i += 1;
    };
    KetQuaDoc { gia_tri: n, vi_tri: i }
}

fn main() {
    let s = String::from("1234");
    let ky_tu: Vec<char> = s.chars().collect();
    let kq = doc_so(&ky_tu, 0);
    println!("{} {}", kq.gia_tri, kq.vi_tri);
}
```

```rust title=solution
struct KetQuaDoc {
    gia_tri: i64,
    vi_tri: usize,
}

fn doc_so(ky_tu: &Vec<char>, bat_dau: usize) -> KetQuaDoc {
    let mut n: i64 = 0;
    let mut i = bat_dau;
    while i < ky_tu.len() && ky_tu[i] >= '0' && ky_tu[i] <= '9' {
        let chu_so = (ky_tu[i] as i64) - ('0' as i64);
        n = n * 10 + chu_so;
        i += 1;
    };
    KetQuaDoc { gia_tri: n, vi_tri: i }
}

fn main() {
    let s = String::from("1234");
    let ky_tu: Vec<char> = s.chars().collect();
    let kq = doc_so(&ky_tu, 0);
    println!("{} {}", kq.gia_tri, kq.vi_tri);
}
```

```rust title=test
fn main() {
    let s1 = String::from("1234");
    let k1: Vec<char> = s1.chars().collect();
    let r1 = doc_so(&k1, 0);
    println!("{} {}", r1.gia_tri, r1.vi_tri);
    assert_eq!(r1.gia_tri, 1234, "doc_so tren '1234' phai ra 1234");
    assert_eq!(r1.vi_tri, 4, "vi_tri phai la 4 sau khi doc het 4 chu so");

    let s2 = String::from("42abc");
    let k2: Vec<char> = s2.chars().collect();
    let r2 = doc_so(&k2, 0);
    assert_eq!(r2.gia_tri, 42, "doc_so phai dung lai truoc chu cai");
    assert_eq!(r2.vi_tri, 2, "vi_tri phai dung o vi tri chu cai dau tien");

    let s3 = String::from("007");
    let k3: Vec<char> = s3.chars().collect();
    let r3 = doc_so(&k3, 0);
    assert_eq!(r3.gia_tri, 7, "so 0 dau khong lam sai gia tri");

    let s4 = String::from("ab12");
    let k4: Vec<char> = s4.chars().collect();
    let r4 = doc_so(&k4, 2);
    assert_eq!(r4.gia_tri, 12, "doc_so bat dau tu vi tri 2 phai bo qua ab");
    assert_eq!(r4.vi_tri, 4, "vi_tri cuoi phai la 4");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Cong don n bang cach dich trai (nhan 10) roi cong chu_so moi -- mot dong, gan lai n."
- kind: strategy
  body: "n = n * 10 + chu_so;"
- kind: one-line
  body: "n = n * 10 + chu_so;"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "1234 4"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Số đọc được rồi — nhưng giữa hai token luôn có khoảng TRẮNG. Bỏ qua
chúng thế nào?
::::

::::reflect{#nghi-lai}
`doc_so` LÀ token đầu tiên của một bộ lexer thật: gộp ký tự thành
một ĐƠN vị có Ý nghĩa (ở đây LÀ một số) bằng tích luỹ số học, KHÔNG
cần "xây" chuỗi trung gian. Trả kết quả qua STRUCT (không tuple) —
một ràng buộc của engine track NÀY, nhưng cũng LÀ một thói quen tốt:
tên trường (`gia_tri`, `vi_tri`) tự giải thích Ý nghĩa hơn hẳn một
tuple vô danh. Số ĐỌC xong — nhưng `"12 34"` (có dấu cách Ở giữa)
sẽ đọc SAI nếu không biết bỏ qua khoảng trắng trước MỖI token.
::::

::::checkpoint{mastery=0.8}
::::
