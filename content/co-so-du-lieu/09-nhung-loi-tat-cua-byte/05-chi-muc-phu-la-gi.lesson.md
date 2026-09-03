---
id: co-so-du-lieu.nhung-loi-tat-cua-byte.chi-muc-phu-la-gi
title: "Chỉ mục phụ là gì"
summary: "MucChiMuc{khoa: Vec<u8>, vi_tri: usize} — một MỤC ánh xạ khoá đã mã hoá (bài 2) của MỘT trường tới vị trí hàng trong Bang. xay_chi_muc(bang, ten) quét bảng MỘT lần, tra_cuu (q08) giá trị trường, mã hoá, gộp thành một DANH SÁCH riêng — song song với bảng chính, không sửa bảng chính."
locale: vi
track: co-so-du-lieu
module: nhung-loi-tat-cua-byte
order: 5
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 10
teaches: [db.secondary-index-idea]
requires: [db.composite-key-encoding]
concepts: [db.secondary-index-idea]
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
Số VÀ khoá ghép đã mã hoá xong (bài 2, bài 4). Một khoá đã mã hoá
thì DÙNG để làm gì — cấu trúc nào GIỮ nó lại để tra cứu SAU này?
::::

::::explain{#muc-va-chi-muc}
Một MỤC chỉ mục (`MucChiMuc`) LÀ một CẶP: khoá đã mã hoá VÀ vị trí
hàng nó trỏ tới TRONG bảng. `xay_chi_muc` quét `Bang` (q08) MỘT
lần, TRA cứu giá trị của MỘT trường CHỌN trước (dùng `tra_cuu`, q08
bài 1), MÃ hoá giá trị đó (bài 2), rồi lưu CẶP (khoá, vị trí):

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

fn main() {
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
    ];
    let chi_muc = xay_chi_muc(&bang, &tuoi);
    println!("{}", chi_muc.len());
    println!("{:?}", chi_muc[0].khoa);
    println!("{}", chi_muc[0].vi_tri);
}
```

```text title=readonly
3
[0, 30]
0
```

`chi_muc.len()` bằng `bang.len()` — MỘT mục CHO mỗi hàng, KHÔNG lọc
bớt gì. `chi_muc[0]` ứng VỚI hàng ĐẦU tiên của `bang` (`tuoi=30`,
`vi_tri=0`), khoá LÀ `ma_hoa_so(30) = [0, 30]`. Chỉ mục LÀ một cấu
trúc RIÊNG, SONG song với `bang` — không hề SỬA `bang`.
::::

::::example{#thu-tu-vat-ly-khong-can-sap-xep}
`chi_muc` GIỮ đúng thứ TỰ hàng gốc của `bang` — hàng ĐẦU (`tuoi=30`)
vẫn Ở vị trí `0` của `chi_muc`, dù VỀ mặt GIÁ trị `30` LỚN hơn cả
`15` lẫn `20`:

```rust title=readonly
println!("{:?}", chi_muc[1].khoa);
```

```text title=readonly
[0, 15]
```

`chi_muc[1].khoa` LÀ `ma_hoa_so(15) = [0, 15]` — ứng VỚI hàng THỨ
hai của `bang` (`tuoi=15`), CHỨ không phải "khoá NHỎ thứ hai". Chỉ
mục Ở bài NÀY chưa hề được SẮP xếp lại — nó chỉ LÀ một BẢN ghi song
song, khoá đã mã hoá GIỮ được thứ tự SỐ (bài 2), nhưng thứ TỰ *vật
lý* của các MỤC trong `chi_muc` vẫn LÀ thứ tự chèn (thứ tự hàng
trong `bang`). Việc SO sánh hai khoá đã mã hoá VẪN cho kết quả đúng
dù chi_muc CHƯA được sắp — đó chính LÀ điều cho phép quét TOÀN bộ
`chi_muc` VÀ giữ lại đúng những MỤC nằm TRONG một khoảng giá trị,
mà KHÔNG cần giải mã ngược khoá VỀ số để so sánh.
::::

::::predict{#doan-so-luong-muc commitOnce}
Một `Bang` CÓ `7` hàng. `xay_chi_muc` chạy trên TRƯỜNG `"diem"` —
`chi_muc.len()` LÀ bao nhiêu?

:::opt{correct}
`7` — MỘT mục cho MỖI hàng, kể cả khi giá TRỊ `"diem"` giống nhau
giữa nhiều hàng
:::

:::opt
Ít hơn `7`, NẾU có hàng trùng giá trị `"diem"` — chỉ mục sẽ GỘP
những giá trị TRÙNG lại thành MỘT mục
::why
Gần đúng ở việc bạn nghĩ tới "gộp giá trị TRÙNG" như MỘT cách tiết
kiệm KHÔNG gian hợp lý — nhiều cấu trúc dữ liệu THẬT (VÍ dụ tập hợp,
set) LÀM đúng điều này.

Chỗ lệch: `xay_chi_muc` KHÔNG hề kiểm tra trùng LẶP — vòng `while i
< bang.len()` chạy ĐÚNG `bang.len()` lần, MỖI lần đẩy THÊM đúng MỘT
`MucChiMuc` VÀO `chi_muc`, bất kể khoá đó ĐÃ xuất hiện trước đó hay
chưa. Mục ĐÍCH của chỉ mục Ở đây LÀ ánh XẠ "hàng NÀO có giá trị GÌ",
không phải liệt kê giá TRỊ duy nhất — hai hàng CÙNG `diem=85` vẫn
tạo RA hai mục RIÊNG, trỏ tới hai vị trí KHÁC nhau.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chỉ mục đã xây xong. Tra cứu THEO một khoảng giá trị TRÊN chỉ mục
đó — không cần giải mã lại từng khoá — trông ra sao?
::::

::::reflect{#nghi-lai}
Chỉ mục PHỤ (secondary index) LÀ một cấu trúc TÁCH biệt khỏi bảng
chính, ánh XẠ (khoá đã mã hoá CỦA một trường) → (vị trí hàng), CHO
phép tra cứu THEO trường ĐÓ mà không cần đi tìm trong TỪNG hàng của
bảng — Ở q08, `tra_cuu` chỉ tìm được TRƯỜNG bên TRONG một hàng ĐÃ
biết trước, không tìm được HÀNG nào chứa một GIÁ trị cho trước. Việc
khoá được mã hoá GIỮ thứ tự (bài 2) chưa tự nó tạo ra tốc ĐỘ — chỉ
mục Ở bài này VẪN LÀ một danh sách quét TUYẾN tính. Điều nó CHO
phép LÀ so sánh khoảng GIÁ trị TRỰC tiếp trên byte, không cần giải
mã ngược VỀ số trước khi so — tra cứu "MỌI hàng có tuổi TRONG
khoảng [18, 27]" trông RA sao?
::::

::::checkpoint{mastery=0.8}
::::
