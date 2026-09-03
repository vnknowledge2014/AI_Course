---
id: co-so-du-lieu.bo-nao-cua-truy-van.hang-du-lieu-la-gi
title: "Hàng dữ liệu là gì"
summary: "struct Truong { ten: Vec<char>, gia_tri: i64 } — một CẶP tên/giá trị. Một HÀNG là Vec<Truong>. tra_cuu(hang, ten) quét tuyến tính, trả về giá trị của trường khớp TÊN, hoặc 0 nếu KHÔNG tìm thấy (không panic — trách nhiệm kiểm tra tồn tại thuộc về người gọi)."
locale: vi
track: co-so-du-lieu
module: bo-nao-cua-truy-van
order: 1
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 12
teaches: [db.executor-row]
requires: [db.parser-cau-truy-van]
concepts: [db.executor-row]
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
Quest TRƯỚC dựng được CÂY cú pháp của một câu truy vấn. Quest NÀY
CHẠY câu truy vấn đó — trên DỮ liệu thật. Một "hàng" dữ liệu LÀ gì?
::::

::::explain{#truong-va-hang}
`Truong` LÀ một CẶP tên/giá trị. Một HÀNG (row) LÀ `Vec<Truong>` —
nhiều trường gộp lại:

```rust title=readonly
struct Truong {
    ten: Vec<char>,
    gia_tri: i64,
}

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut i = 0;
    while i < hang.len() {
        if hang[i].ten == *ten {
            return hang[i].gia_tri;
        }
        i += 1;
    }
    0
}

fn main() {
    let hang: Vec<Truong> = vec![
        Truong { ten: vec!['t','u','o','i'], gia_tri: 20 },
        Truong { ten: vec!['d','i','e','m'], gia_tri: 85 },
    ];
    let ten_tuoi: Vec<char> = vec!['t','u','o','i'];
    println!("{}", tra_cuu(&hang, &ten_tuoi));
}
```

```text title=readonly
20
```

`tra_cuu` quét TUYẾN tính qua từng `Truong`, so KHỚP `ten` (dùng
`Vec<char>` — không `String`, đúng quy ước quest TRƯỚC), trả VỀ
`gia_tri` của trường ĐẦU tiên khớp. `ten: &Vec<char>` VÀ `hang:
&Vec<Truong>` đều LÀ tham chiếu BẤT BIẾN — dùng lại được xuyên suốt
vòng lặp, không cần lo gì thêm.
::::

::::example{#tra-cuu-khong-thay}
Tra cứu một tên trường KHÔNG tồn tại trong hàng — trả VỀ `0`, không
`panic`:

```rust title=readonly
let ten_la: Vec<char> = vec!['l','a'];
println!("{}", tra_cuu(&hang, &ten_la));
```

```text title=readonly
0
```

Vòng `while` chạy HẾT `hang` mà không tìm thấy trường TÊN `"la"` —
rơi thẳng xuống `0` Ở CUỐI hàm. Đây LÀ một quyết định thiết kế CÓ
chủ đích: `tra_cuu` KHÔNG kiểm tra "trường CÓ tồn tại không", chỉ
trả VỀ giá trị MẶC định khi không tìm thấy — trách nhiệm biết trường
NÀO thực sự tồn tại thuộc VỀ người gọi (executor sẽ xây SAU).
::::

::::predict{#doan-hang-rong commitOnce}
Tra cứu trên một HÀNG rỗng (không có trường nào):

```rust
let hang_rong: Vec<Truong> = Vec::new();
let ten_tuoi2: Vec<char> = vec!['t','u','o','i'];
println!("{}", tra_cuu(&hang_rong, &ten_tuoi2));
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
Máy báo lỗi — vì `Vec::new()` tạo một `Vec<Truong>` RỖNG, VÀ đưa nó
làm ĐẦU vào cho một hàm mong đợi DỮ liệu LÀ một trường hợp không
hợp lệ
::why
Gần đúng ở việc bạn để Ý `hang_rong` THỰC sự không CÓ trường nào —
một quan sát chính XÁC về dữ liệu.

Chỗ lệch: `while i < hang.len()` — VỚI `hang.len()` LÀ `0`, điều
kiện `0 < 0` SAI NGAY từ đầu, vòng lặp KHÔNG chạy lần nào, hàm rơi
thẳng XUỐNG `0`. Một `Vec` rỗng hoàn toàn LÀ dữ liệu hợp lệ trong
Rust — không CÓ khái niệm "rỗng LÀ lỗi" Ở đây.
::
:::
::::

::::code{#viet_tra_cuu}
Hoàn thiện `tra_cuu` — so khớp tên trường TẠI vị trí `i`.

```rust title=starter
struct Truong {
    ten: Vec<char>,
    gia_tri: i64,
}

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut i = 0;
    while i < hang.len() {
        if ___ {
            return hang[i].gia_tri;
        }
        i += 1;
    }
    0
}

fn main() {
    let hang: Vec<Truong> = vec![
        Truong { ten: vec!['t','u','o','i'], gia_tri: 20 },
        Truong { ten: vec!['d','i','e','m'], gia_tri: 85 },
    ];
    let ten_tuoi: Vec<char> = vec!['t','u','o','i'];
    println!("{}", tra_cuu(&hang, &ten_tuoi));
}
```

```rust title=solution
struct Truong {
    ten: Vec<char>,
    gia_tri: i64,
}

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut i = 0;
    while i < hang.len() {
        if hang[i].ten == *ten {
            return hang[i].gia_tri;
        }
        i += 1;
    }
    0
}

fn main() {
    let hang: Vec<Truong> = vec![
        Truong { ten: vec!['t','u','o','i'], gia_tri: 20 },
        Truong { ten: vec!['d','i','e','m'], gia_tri: 85 },
    ];
    let ten_tuoi: Vec<char> = vec!['t','u','o','i'];
    println!("{}", tra_cuu(&hang, &ten_tuoi));
}
```

```rust title=test
fn main() {
    let hang: Vec<Truong> = vec![
        Truong { ten: vec!['t','u','o','i'], gia_tri: 20 },
        Truong { ten: vec!['d','i','e','m'], gia_tri: 85 },
    ];
    let ten_tuoi: Vec<char> = vec!['t','u','o','i'];
    println!("{}", tra_cuu(&hang, &ten_tuoi));
    assert_eq!(tra_cuu(&hang, &ten_tuoi), 20, "tra cuu tuoi phai ra 20");

    let ten_diem: Vec<char> = vec!['d','i','e','m'];
    assert_eq!(tra_cuu(&hang, &ten_diem), 85, "tra cuu diem phai ra 85");

    let ten_la: Vec<char> = vec!['l','a'];
    assert_eq!(tra_cuu(&hang, &ten_la), 0, "khong tim thay -- tra ve 0");

    let hang_rong: Vec<Truong> = Vec::new();
    assert_eq!(tra_cuu(&hang_rong, &ten_tuoi), 0, "hang rong -- tra ve 0");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "So sanh hang[i].ten voi *ten bang == -- mot dong."
- kind: strategy
  body: "hang[i].ten == *ten"
- kind: one-line
  body: "hang[i].ten == *ten"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "20"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một hàng tra cứu được rồi. Nhiều hàng gộp lại thành một BẢNG trông
ra sao?
::::

::::reflect{#nghi-lai}
`Truong`/`Hang` LÀ đơn vị dữ liệu NHỎ nhất của quest NÀY — một hàng
CHỈ LÀ danh sách cặp tên/giá trị, tra cứu bằng quét TUYẾN tính (đơn
giản, phù hợp quy mô bài học — cơ sở dữ liệu THẬT dùng chỉ mục để
tránh quét toàn bộ mỗi lần, ĐÃ học Ở q00-q01). `tra_cuu` KHÔNG phân
biệt "trường không tồn tại" VỚI "trường CÓ giá trị 0" — một giới
hạn CÓ thật của thiết kế NÀY, chấp nhận được Ở quy mô một quest giới
thiệu. Một hàng đã tra cứu được — NHIỀU hàng gộp lại thành một BẢNG
trong bộ nhớ trông ra sao?
::::

::::checkpoint{mastery=0.8}
::::
