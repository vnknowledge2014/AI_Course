---
id: co-so-du-lieu.ngon-ngu-cua-byte.bo-qua-khoang-trang
title: "Bỏ qua khoảng trắng giữa các token"
summary: "bo_qua_trang(ky_tu: &Vec<char>, vi_tri: usize) -> usize tăng vi_tri qua mọi ký tự ' ' liên tiếp, trả về vị trí token TIẾP THEO thật sự bắt đầu. Gọi TRƯỚC mỗi lần đọc token — '   42' phải đọc thành số 42, không phải lỗi vì gặp dấu cách trước."
locale: vi
track: co-so-du-lieu
module: ngon-ngu-cua-byte
order: 2
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 10
teaches: [db.lexer-whitespace]
requires: [db.lexer-token]
concepts: [db.lexer-whitespace]
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
`doc_so` (bài TRƯỚC) chạy đúng khi số bắt đầu NGAY tại `bat_dau`.
Nhưng câu truy vấn thật CÓ dấu cách giữa các token — `"   42"` thì
sao?
::::

::::explain{#bo-qua-trang}
`bo_qua_trang` tăng `vi_tri` qua MỌI ký tự dấu cách liên tiếp, trả
về vị trí ký tự TIẾP theo KHÔNG phải dấu cách:

```rust title=readonly
fn bo_qua_trang(ky_tu: &Vec<char>, vi_tri: usize) -> usize {
    let mut i = vi_tri;
    while i < ky_tu.len() && ky_tu[i] == ' ' {
        i += 1;
    };
    i
}

fn main() {
    let s = String::from("   12");
    let ky_tu: Vec<char> = s.chars().collect();
    let vt = bo_qua_trang(&ky_tu, 0);
    println!("{}", vt);
}
```

```text title=readonly
3
```

Ba ký tự ĐẦU (`' '`, `' '`, `' '`) khớp điều kiện, `i` tăng BA lần —
tại `i=3`, ký tự LÀ `'1'`, điều kiện `ky_tu[3] == ' '` sai, vòng lặp
dừng. `bo_qua_trang` trả VỀ đúng vị trí ký tự có Ý nghĩa ĐẦU tiên.
::::

::::example{#ghep-voi-doc-so}
Gọi `bo_qua_trang` TRƯỚC `doc_so` (bài trước) — đọc đúng số dù có
dấu cách dẫn đầu:

```rust title=readonly
let s2 = String::from("   42");
let ky_tu2: Vec<char> = s2.chars().collect();
let vt2 = bo_qua_trang(&ky_tu2, 0);
let kq2 = doc_so(&ky_tu2, vt2);
println!("{} {}", kq2.gia_tri, kq2.vi_tri);
```

```text title=readonly
42 5
```

`bo_qua_trang` trả VỀ `3` (bỏ qua ba dấu cách) — `doc_so` NHẬN đúng
`3` làm `bat_dau`, đọc `"42"` bắt đầu TỪ đó, kết thúc TẠI `5` (hết
chuỗi). Không cần thay đổi GÌ Ở `doc_so` — nó CHƯA bao giờ quan tâm
`bat_dau` LÀ `0` hay LÀ giá trị nào khác.
::::

::::predict{#doan-toan-khoang-trang commitOnce}
Một chuỗi CHỈ toàn dấu cách, không có gì khác:

```rust
let s3 = String::from("      ");
let ky_tu3: Vec<char> = s3.chars().collect();
let vt3 = bo_qua_trang(&ky_tu3, 0);
println!("{}", vt3);
```

Chuỗi CÓ đúng sáu ký tự dấu cách. Dòng cuối in ra gì?

:::opt{correct}
`6`
:::

:::opt
Máy báo lỗi — vì không CÒN ký tự nào có Ý nghĩa Ở CUỐI, hàm phải
báo "không tìm thấy token"
::why
Gần đúng ở việc bạn nghĩ TỚI "hết chuỗi mà chưa thấy gì có Ý nghĩa"
như một trường hợp ĐÁNG chú ý — một phản xạ hợp LÝ khi thiết kế API.

Chỗ lệch: `bo_qua_trang` KHÔNG hề có nhiệm vụ "tìm token" — nó CHỈ
đếm dấu cách. Điều kiện vòng lặp LÀ `i < ky_tu.len() && ...` — khi
`i` chạm ĐÚNG `ky_tu.len()` (LÀ `6`), vế đầu SAI, vòng lặp dừng êm,
KHÔNG hề đọc quá giới hạn mảng. Trả về `6` — đúng nghĩa "đã bỏ qua
hết, không còn gì".
::
:::

:::opt
`5` — vì chỉ số CUỐI cùng hợp lệ của một chuỗi sáu ký tự LÀ `5`
(đếm từ `0`)
::why
Gần đúng ở việc bạn nhớ ĐÚNG quy tắc chỉ số bắt đầu từ `0` — đúng
với việc TRUY CẬP một phần tử CỤ THỂ trong mảng sáu phần tử.

Chỗ lệch: `vi_tri` KHÔNG phải "chỉ số một ký tự đang tồn tại" — nó
LÀ "vị trí nơi lần đọc TIẾP theo nên bắt đầu", và giá trị ĐÓ hoàn
toàn có thể LÀ `ky_tu.len()` (đúng bằng độ dài, một chỉ số VƯỢT quá
phần tử cuối) khi KHÔNG còn gì để đọc — chính giá trị NÀY báo hiệu
"đã hết chuỗi" cho lệnh gọi TIẾP theo.
::
:::
::::

::::code{#viet_bo_qua_trang}
Hoàn thiện `bo_qua_trang` — tăng `i` qua mọi dấu cách liên tiếp.

```rust title=starter
fn bo_qua_trang(ky_tu: &Vec<char>, vi_tri: usize) -> usize {
    let mut i = vi_tri;
    while i < ky_tu.len() && ky_tu[i] == ' ' {
        ___
    };
    i
}

fn main() {
    let s = String::from("   12");
    let ky_tu: Vec<char> = s.chars().collect();
    let vt = bo_qua_trang(&ky_tu, 0);
    println!("{}", vt);
}
```

```rust title=solution
fn bo_qua_trang(ky_tu: &Vec<char>, vi_tri: usize) -> usize {
    let mut i = vi_tri;
    while i < ky_tu.len() && ky_tu[i] == ' ' {
        i += 1;
    };
    i
}

fn main() {
    let s = String::from("   12");
    let ky_tu: Vec<char> = s.chars().collect();
    let vt = bo_qua_trang(&ky_tu, 0);
    println!("{}", vt);
}
```

```rust title=test
fn main() {
    let s1 = String::from("   12");
    let k1: Vec<char> = s1.chars().collect();
    println!("{}", bo_qua_trang(&k1, 0));
    assert_eq!(bo_qua_trang(&k1, 0), 3, "ba dau cach dau tien phai bi bo qua");

    let s2 = String::from("12");
    let k2: Vec<char> = s2.chars().collect();
    assert_eq!(bo_qua_trang(&k2, 0), 0, "khong co dau cach -- vi tri khong doi");

    let s3 = String::from("      ");
    let k3: Vec<char> = s3.chars().collect();
    assert_eq!(bo_qua_trang(&k3, 0), 6, "toan dau cach -- vi tri di het chuoi");

    let s4 = String::from("a  b");
    let k4: Vec<char> = s4.chars().collect();
    assert_eq!(bo_qua_trang(&k4, 1), 3, "bat dau tu giua chuoi cung phai bo qua dung so dau cach");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Trong than vong while, tang i len 1 -- mot dong."
- kind: strategy
  body: "i += 1;"
- kind: one-line
  body: "i += 1;"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khoảng trắng bỏ qua được rồi. Nhưng token đâu CHỈ có SỐ — tên trường
như `tuoi`, tên bảng như `nguoi` đọc thế NÀO?
::::

::::reflect{#nghi-lai}
`bo_qua_trang` giải quyết đúng MỘT vấn đề: giữa hai token LUÔN có
thể có khoảng trắng, và lexer PHẢI "nuốt" nó trước khi đọc token
tiếp theo — không hề phức tạp, chỉ LÀ một vòng lặp đếm. Gọi nó
TRƯỚC `doc_so` (hoặc bất kỳ hàm đọc token nào sắp xây tiếp) LÀ đủ.
Số VÀ khoảng trắng đã xong — token còn LẠI (tên trường, tên bảng,
từ khoá `SELECT`/`FROM`/`WHERE`) đều LÀ chữ cái nối tiếp nhau, gọi
CHUNG là "định danh". Đọc định danh khác đọc số Ở CHỖ nào?
::::

::::checkpoint{mastery=0.8}
::::
