---
id: co-so-du-lieu.nhung-loi-tat-cua-byte.xay-va-tra-cuu-chi-muc-phu
title: "Xây và tra cứu chỉ mục phụ"
summary: "trong_khoang(chi_muc, tu, den) quét chi_muc, giữ lại MỌI MucChiMuc có khoa nằm TRONG [tu, den] — so sánh Vec<u8> trực tiếp bằng >= và <=, KHÔNG giải mã ngược về i64. Với chi_muc đã xây (bài 5) trên trường tuoi, trong_khoang(chi_muc, ma_hoa_so(18), ma_hoa_so(27)) trả về đúng những vi_tri có tuoi trong [18,27]."
locale: vi
track: co-so-du-lieu
module: nhung-loi-tat-cua-byte
order: 6
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 12
teaches: [db.secondary-index-build]
requires: [db.secondary-index-idea]
concepts: [db.secondary-index-build]
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
`xay_chi_muc` (bài trước) đã GỘP mọi hàng thành một danh sách khoá
đã mã hoá. Tra cứu "MỌI hàng có tuổi TRONG khoảng [18, 27]" TRÊN
danh sách đó thế NÀO?
::::

::::explain{#tra-cuu-trong-khoang}
`trong_khoang` quét TỪNG mục của `chi_muc`, giữ LẠI vị trí của
những mục CÓ khoá nằm TRONG đoạn `[tu, den]` (cả hai ĐẦU) — so sánh
TRỰC tiếp trên `Vec<u8>`, dùng `>=` VÀ `<=`:

```rust title=readonly
struct Truong { ten: Vec<char>, gia_tri: i64 }

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

fn ma_hoa_so(n: i64) -> Vec<u8> {
    let cao: u8 = (n / 256) as u8;
    let thap: u8 = (n % 256) as u8;
    let mut ra: Vec<u8> = Vec::new();
    ra.push(cao);
    ra.push(thap);
    ra
}

struct MucChiMuc { khoa: Vec<u8>, vi_tri: usize }

fn xay_chi_muc(bang: &Vec<Vec<Truong> >, ten: &Vec<char>) -> Vec<MucChiMuc> {
    let mut chi_muc: Vec<MucChiMuc> = Vec::new();
    let mut i = 0;
    while i < bang.len() {
        let gia_tri = tra_cuu(&bang[i], ten);
        chi_muc.push(MucChiMuc { khoa: ma_hoa_so(gia_tri), vi_tri: i });
        i = 1 + i;
    }
    chi_muc
}

fn trong_khoang(chi_muc: &Vec<MucChiMuc>, tu: &Vec<u8>, den: &Vec<u8>) -> Vec<usize> {
    let mut ket_qua: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < chi_muc.len() {
        if chi_muc[i].khoa >= *tu && chi_muc[i].khoa <= *den {
            ket_qua.push(chi_muc[i].vi_tri);
        }
        i = 1 + i;
    }
    ket_qua
}

fn main() {
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
    ];
    let chi_muc = xay_chi_muc(&bang, &tuoi);
    let tu = ma_hoa_so(18);
    let den = ma_hoa_so(27);
    let r = trong_khoang(&chi_muc, &tu, &den);
    println!("{:?}", r);
    println!("{}", r.len());
}
```

```text title=readonly
[1, 2]
2
```

Bốn hàng CÓ `tuoi` LẦN lượt LÀ `15, 20, 25, 30` (vị trí `0..3`).
Khoảng `[18, 27]` chỉ khớp `20` (vị trí `1`) VÀ `25` (vị trí `2`) —
`15` VÀ `30` đều nằm NGOÀI khoảng. So sánh `chi_muc[i].khoa >= *tu
&& chi_muc[i].khoa <= *den` diễn ra HOÀN toàn trên `Vec<u8>`, không
CÓ bước "giải mã khoá TRỞ lại thành số RỒI mới so" NÀO cả.
::::

::::example{#khoang-rong}
Khoảng KHÔNG khớp hàng nào — `trong_khoang` trả VỀ danh sách rỗng,
KHÔNG lỗi:

```rust title=readonly
let tu2 = ma_hoa_so(0);
let den2 = ma_hoa_so(10);
let r2 = trong_khoang(&chi_muc, &tu2, &den2);
println!("{}", r2.len());
```

```text title=readonly
0
```

Không CÓ hàng nào `tuoi` TRONG `[0, 10]` (nhỏ nhất LÀ `15`) — vòng
`while` chạy HẾT bốn mục mà KHÔNG mục nào thoả điều kiện, `ket_qua`
GIỮ nguyên rỗng từ lúc khởi tạo tới lúc trả VỀ.
::::

::::predict{#doan-bien-dong commitOnce}
Bảng CÓ bốn hàng `tuoi` LÀ `15, 20, 25, 30`. Khoảng tra cứu LÀ `[20,
25]` (CẢ hai đầu MÚT — dùng chính giá trị `20` VÀ `25` LÀM `tu`/`den`).
`trong_khoang(&chi_muc, &tu, &den).len()` LÀ bao nhiêu?

:::opt{correct}
`2` — CẢ hàng `tuoi=20` LẪN hàng `tuoi=25` đều nằm TRONG kết quả
:::

:::opt
`0` — khoảng `[tu, den]` giống `Range` của Rust (`a..b`), LOẠI trừ đầu
mút CUỐI, nên `tuoi=20` VÀ `tuoi=25` đều bị BỎ
::why
Gần đúng ở việc bạn liên hệ TỚI `Range` (`a..b`) — một kiểu khoảng THẬT
Ở Rust, LOẠI trừ đầu mút cuối.

Chỗ lệch: `trong_khoang` KHÔNG dùng `Range` — điều kiện của nó LÀ
`chi_muc[i].khoa >= *tu && chi_muc[i].khoa <= *den` (bài NÀY), CẢ hai
dấu ĐỀU LÀ `>=`/`<=` (đóng Ở cả hai đầu), không phải `<`/`<` (mở Ở cuối
như `Range`). `tuoi=20` khớp `khoa >= ma_hoa_so(20)` (BẰNG, thoả `>=`)
VÀ `tuoi=25` khớp `khoa <= ma_hoa_so(25)` (BẰNG, thoả `<=`) — CẢ hai
đều được GIỮ, kết quả `len()` LÀ `2`, không phải `0`.
::
:::
::::

::::code{#viet_trong_khoang}
Hoàn thiện `trong_khoang` — điều kiện giữ lại một MỤC: khoá của nó
nằm TRONG đoạn `[tu, den]`.

```rust title=starter
struct Truong { ten: Vec<char>, gia_tri: i64 }

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

fn ma_hoa_so(n: i64) -> Vec<u8> {
    let cao: u8 = (n / 256) as u8;
    let thap: u8 = (n % 256) as u8;
    let mut ra: Vec<u8> = Vec::new();
    ra.push(cao);
    ra.push(thap);
    ra
}

struct MucChiMuc { khoa: Vec<u8>, vi_tri: usize }

fn xay_chi_muc(bang: &Vec<Vec<Truong> >, ten: &Vec<char>) -> Vec<MucChiMuc> {
    let mut chi_muc: Vec<MucChiMuc> = Vec::new();
    let mut i = 0;
    while i < bang.len() {
        let gia_tri = tra_cuu(&bang[i], ten);
        chi_muc.push(MucChiMuc { khoa: ma_hoa_so(gia_tri), vi_tri: i });
        i = 1 + i;
    }
    chi_muc
}

fn trong_khoang(chi_muc: &Vec<MucChiMuc>, tu: &Vec<u8>, den: &Vec<u8>) -> Vec<usize> {
    let mut ket_qua: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < chi_muc.len() {
        if ___ {
            ket_qua.push(chi_muc[i].vi_tri);
        }
        i = 1 + i;
    }
    ket_qua
}

fn main() {
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
    ];
    let chi_muc = xay_chi_muc(&bang, &tuoi);
    let tu = ma_hoa_so(18);
    let den = ma_hoa_so(27);
    let r = trong_khoang(&chi_muc, &tu, &den);
    println!("{}", r.len());
}
```

```rust title=solution
struct Truong { ten: Vec<char>, gia_tri: i64 }

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

fn ma_hoa_so(n: i64) -> Vec<u8> {
    let cao: u8 = (n / 256) as u8;
    let thap: u8 = (n % 256) as u8;
    let mut ra: Vec<u8> = Vec::new();
    ra.push(cao);
    ra.push(thap);
    ra
}

struct MucChiMuc { khoa: Vec<u8>, vi_tri: usize }

fn xay_chi_muc(bang: &Vec<Vec<Truong> >, ten: &Vec<char>) -> Vec<MucChiMuc> {
    let mut chi_muc: Vec<MucChiMuc> = Vec::new();
    let mut i = 0;
    while i < bang.len() {
        let gia_tri = tra_cuu(&bang[i], ten);
        chi_muc.push(MucChiMuc { khoa: ma_hoa_so(gia_tri), vi_tri: i });
        i = 1 + i;
    }
    chi_muc
}

fn trong_khoang(chi_muc: &Vec<MucChiMuc>, tu: &Vec<u8>, den: &Vec<u8>) -> Vec<usize> {
    let mut ket_qua: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < chi_muc.len() {
        if chi_muc[i].khoa >= *tu && chi_muc[i].khoa <= *den {
            ket_qua.push(chi_muc[i].vi_tri);
        }
        i = 1 + i;
    }
    ket_qua
}

fn main() {
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
    ];
    let chi_muc = xay_chi_muc(&bang, &tuoi);
    let tu = ma_hoa_so(18);
    let den = ma_hoa_so(27);
    let r = trong_khoang(&chi_muc, &tu, &den);
    println!("{}", r.len());
}
```

```rust title=test
fn main() {
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
    ];
    let chi_muc = xay_chi_muc(&bang, &tuoi);
    let tu = ma_hoa_so(18);
    let den = ma_hoa_so(27);
    let r = trong_khoang(&chi_muc, &tu, &den);
    println!("{}", r.len());
    assert_eq!(r.len(), 2, "phai co dung 2 vi tri trong khoang 18..27");
    assert_eq!(r[0], 1, "vi tri dau tien phai la 1 (tuoi=20)");
    assert_eq!(r[1], 2, "vi tri thu hai phai la 2 (tuoi=25)");

    let tu2 = ma_hoa_so(0);
    let den2 = ma_hoa_so(10);
    let r2 = trong_khoang(&chi_muc, &tu2, &den2);
    assert_eq!(r2.len(), 0, "khong ai trong khoang 0..10");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Dieu kien giu lai mot muc: khoa cua no >= tu VA <= den -- dung && de ghep hai so sanh, mot dong."
- kind: strategy
  body: "chi_muc[i].khoa >= *tu && chi_muc[i].khoa <= *den"
- kind: one-line
  body: "if chi_muc[i].khoa >= *tu && chi_muc[i].khoa <= *den {"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chỉ mục theo GIÁ trị đã xong. Một cạnh ĐỒ thị — nối hai HÀNG với
nhau — có tận dụng được ĐÚNG kỹ thuật này không?
::::

::::reflect{#nghi-lai}
`trong_khoang` LÀ minh chứng CỤ thể cho lý do "khoá giữ thứ TỰ"
(bài 1-2) có ý nghĩa: TRUY vấn khoảng (range QUERY) trở thành ĐÚNG
hai phép so sánh (`>=` VÀ `<=`) trên `Vec<u8>`, không CẦN một bước
giải mã NÀO xen giữa. Chỉ mục Ở đây VẪN LÀ quét TUYẾN tính — một hệ
THẬT còn SẮP xếp vật lý các mục THEO khoá (giúp dừng SỚM, thậm chí
tìm nhị PHÂN), nằm NGOÀI phạm vi bài NÀY. `MucChiMuc{khoa, vi_tri}`
LÀ một cấu trúc CHUNG — không CHỈ dùng cho giá trị TRƯỜNG, nó CÓ ánh
xạ được một QUAN hệ khác: "đỉnh NÀY nối tới đỉnh nào" TRONG một đồ
thị không?
::::

::::checkpoint{mastery=0.8}
::::
