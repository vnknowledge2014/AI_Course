//! Corpus **âm thủ công**: các chương trình Rust mà `rustc` thật TỪ CHỐI, nhưng
//! `byte-rust` trả `Dat`.
//!
//! # Vì sao cần tệp này bên cạnh `mutate.rs`
//!
//! Đột biến cơ học trong `mutate.rs` chỉ biết làm hỏng code theo mười khuôn mẫu
//! cố định (xoá `&`, xoá `.clone()`, xoá nhánh `match`…). Nó dò được đúng những
//! lỗ hổng nằm trên đường đi của mười khuôn mẫu đó. Mọi lỗ hổng khác vô hình
//! với nó — và một cổng merge xanh vì **không ai hỏi đúng câu hỏi** thì tệ hơn
//! không có cổng, vì nó phát ra sự tự tin.
//!
//! Tệp này là kết quả của việc đọc từng module `byte-rust` để tìm chỗ suy luận
//! bị bỏ dở, rồi viết chương trình chạm đúng vào chỗ đó. Mỗi ca dưới đây đã
//! được chạy qua `rustc --edition 2021 --emit metadata` (rustc 1.97.1) VÀ qua
//! `byte-rust`; ghi chú nêu mã lỗi thật của `rustc` và module chứa lỗ hổng.
//!
//! # Cách đọc kết quả
//!
//! Mỗi ca ở đây trả `Dat` là **một lần NHẬN OAN** — người học nhận tick xanh cho
//! chương trình `cargo` sẽ không chịu biên dịch. Theo ADR-002 §4 đó là FAIL
//! MERGE. Ca nào chuyển sang `KhongDat` (đúng mã lỗi) hoặc `ChuaHoTro` (thú
//! nhận) là ca đã được vá.
//!
//! # KHÔNG được vá byte-rust "cho vừa tệp này"
//!
//! Các ca dưới đây là **triệu chứng**, không phải đặc tả. Vá đến mức đúng từng
//! chương trình cụ thể ở đây (so tên biến, so hình dạng AST) sẽ làm cổng xanh
//! mà lỗ hổng vẫn còn nguyên — đúng chế độ hỏng mà ADR-002 tồn tại để chặn.
//! Phải vá ở gốc: chỗ `tyck` vứt kiểu đi, chỗ `move_check` thiếu khái niệm
//! đường dẫn nơi chốn, chỗ `interp` tự ép kiểu.

/// Một chương trình âm đã xác minh hai chiều.
pub struct HatGiongAm {
    /// Tên ca — dùng làm tên tệp tạm khi gọi `rustc`.
    pub ten: &'static str,
    /// Nhóm lỗ hổng, để tính riêng từng mảng trong báo cáo.
    pub nhom: &'static str,
    /// Mã lỗi `rustc` thật đã quan sát được.
    pub ma_loi: &'static str,
    /// Module `byte-rust` chứa lỗ hổng (theo thứ tự quan trọng dần).
    pub lo_hong: &'static str,
    pub ma: &'static str,
}

/// Tên các nhóm, dùng để in bảng tổng kết theo mảng.
pub const NHOM: &[(&str, &str)] = &[
    ("so-hoc", "Số học & ép kiểu ngầm"),
    ("muon", "Mượn, khả biến của tham chiếu, vòng đời"),
    ("move", "Chuyển quyền sở hữu"),
    ("khop-mau", "Khớp mẫu & tính vét cạn"),
    ("kieu-bieu-thuc", "Kiểu của biểu thức & vùng mù mã chết"),
    ("struct-enum", "Struct & enum"),
    ("phuong-thuc", "Phương thức dựng sẵn"),
    ("kha-bien", "Khả biến của binding & phân giải tên"),
];

pub const CORPUS_AM: &[HatGiongAm] = &[
    // ═══════════════════════════════════════════════════════════════════════
    // NHÓM 1 — SỐ HỌC & ÉP KIỂU NGẦM
    //
    // Gốc rễ chung: `tu_kieu_ast` (tyck.rs:93-99) gộp i8..i128, u8..u128,
    // usize, isize thành MỘT `T::SoNguyen`; và `chac_chan_lech` (tyck.rs:84)
    // tuyên bố `(SoNguyen, SoThuc) => false`. Hai quyết định này xoá sạch bài
    // học trung tâm nhất của Rust về số: **không có ép kiểu số ngầm**.
    // ═══════════════════════════════════════════════════════════════════════

    // rustc: E0277 — không có `impl Add<f64> for i64`. byte-rust in ra 12.5 vì
    // interp.rs:1180 tự nâng i64 lên f64 như JavaScript.
    HatGiongAm {
        ten: "cong_i64_f64",
        nhom: "so-hoc",
        ma_loi: "E0277",
        lo_hong: "tyck.rs (chac_chan_lech + HaiNgoi không đối chiếu hai vế) + interp.rs:1180",
        ma: r#"
fn main() {
    let a: i64 = 10;
    let b: f64 = 2.5;
    let c = a + b;
    println!("{}", c);
}
"#,
    },
    // rustc: E0308 — `n` đã bị ghim `i64`, literal linh hoạt không cứu được.
    // byte-rust CÓ bộ kiểm đối số BR0301, tính đúng mong=SoThuc/thực=SoNguyen,
    // rồi tự nuốt lỗi vì điều kiện phát là `chac_chan_lech`.
    HatGiongAm {
        ten: "doi_so_f64_nhan_i64",
        nhom: "so-hoc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (chac_chan_lech vô hiệu hoá BR0301)",
        ma: r#"
fn nua(x: f64) -> f64 {
    x / 2.0
}
fn main() {
    let n: i64 = 7;
    println!("{}", nua(n));
}
"#,
    },
    // rustc: E0308 — hướng nguy hiểm hơn: số thực chảy vào vị trí chỉ số/độ dài.
    // byte-rust in "3.5" cho một hàm khai báo `-> usize`.
    HatGiongAm {
        ten: "doi_so_usize_nhan_f64",
        nhom: "so-hoc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (chac_chan_lech; tu_kieu_ast gộp usize) + interp.rs (hai_ngoi nâng kiểu)",
        ma: r#"
fn lap_lai(n: usize) -> usize {
    n + 1
}
fn main() {
    let x: f64 = 2.5;
    println!("{}", lap_lai(x));
}
"#,
    },
    // rustc: E0308 — thân cho `{integer}`, chữ ký hứa `f64`. BR0302 mù hoàn
    // toàn với mọi lệch số nguyên/số thực vì cũng dựa trên `chac_chan_lech`.
    HatGiongAm {
        ten: "tra_ve_f64_than_i64",
        nhom: "so-hoc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (kiem_tra: BR0302 dựa trên chac_chan_lech)",
        ma: r#"
fn ty_le() -> f64 {
    3
}
fn main() {
    println!("{}", ty_le());
}
"#,
    },
    // rustc: E0277 — `SliceIndex` chỉ impl cho `usize`. Bài học usize-vs-i64,
    // chính điều rustc dạy, bị xoá sạch: tyck không ràng buộc kiểu chỉ số.
    HatGiongAm {
        ten: "chi_so_i64",
        nhom: "so-hoc",
        ma_loi: "E0277",
        lo_hong: "tyck.rs (duyet_con: ChiSo chỉ duyệt con) + interp.rs:829-848 (so_nguyen())",
        ma: r#"
fn main() {
    let v = vec![10, 20, 30];
    let i: i64 = 2;
    println!("{}", v[i]);
}
"#,
    },
    // rustc: E0308 — `PartialOrd` chỉ so hai giá trị cùng kiểu. tyck trả thẳng
    // `T::Bool` cho mọi toán tử so sánh mà KHÔNG hề suy kiểu hai vế.
    HatGiongAm {
        ten: "so_sanh_i64_f64",
        nhom: "so-hoc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (HaiNgoi: so sánh trả T::Bool không suy vế) + value.rs:206 (nhánh chéo)",
        ma: r#"
fn main() {
    let a: i64 = 3;
    let b: f64 = 3.5;
    if a < b {
        println!("nho hon");
    } else {
        println!("khong nho hon");
    }
}
"#,
    },
    // rustc: lint deny-by-default `overflowing_literals` (không có mã E) —
    // literal out of range for `u8`. byte-rust in ra "300".
    HatGiongAm {
        ten: "tran_literal_u8",
        nhom: "so-hoc",
        ma_loi: "lint overflowing_literals (không mã E)",
        lo_hong: "tyck.rs (khoi/CauLenh::Let không đối chiếu; tu_kieu_ast mất bề rộng)",
        ma: r#"
fn main() {
    let x: u8 = 300;
    println!("{}", x);
}
"#,
    },
    // rustc: E0600 — `usize` không impl `Neg`. byte-rust in ra "-5" cho một
    // biến khai báo `usize`: sai cả kiểu lẫn giá trị.
    HatGiongAm {
        ten: "am_gan_usize",
        nhom: "so-hoc",
        ma_loi: "E0600",
        lo_hong: "tyck.rs (MotNgoi::Am trả nguyên kiểu toán hạng; Let không kiểm; mất tính dấu)",
        ma: r#"
fn main() {
    let n: usize = -5;
    println!("{}", n);
}
"#,
    },
    // rustc: E0605 — `as` chỉ dùng cho kiểu nguyên thuỷ. byte-rust cho `n`
    // mang kiểu tĩnh i64 nhưng lúc chạy vẫn là chuỗi: trông y hệt parse thành công.
    HatGiongAm {
        ten: "ep_chuoi_sang_i64",
        nhom: "so-hoc",
        ma_loi: "E0605",
        lo_hong: "tyck.rs (Ep bỏ qua kiểu nguồn) + interp.rs:894 (Ep: nhánh `_ => v`)",
        ma: r#"
fn main() {
    let s = String::from("42");
    let n = s as i64;
    println!("{}", n);
}
"#,
    },
    // rustc: E0308 — `a / b` là i64, gán vào biến khai báo f64. byte-rust
    // chẳng những nhận mà còn IN RA 3 thay vì 3.5: tick xanh KÈM kết quả sai.
    HatGiongAm {
        ten: "chia_nguyen_gan_f64",
        nhom: "so-hoc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (khoi/CauLenh::Let hoàn toàn không đối chiếu kiểu khai báo)",
        ma: r#"
fn main() {
    let a: i64 = 7;
    let b: i64 = 2;
    let c: f64 = a / b;
    println!("{}", c);
}
"#,
    },
    // rustc: E0308 + E0277 (hai lỗi) — không cộng được hai bề rộng khác nhau.
    // byte-rust mù vì hai vế trông giống hệt nhau sau khi gộp về T::SoNguyen.
    HatGiongAm {
        ten: "u32_cong_i64",
        nhom: "so-hoc",
        ma_loi: "E0308, E0277",
        lo_hong: "tyck.rs (tu_kieu_ast gộp mọi kiểu nguyên; HaiNgoi không đối chiếu)",
        ma: r#"
fn main() {
    let a: u32 = 5;
    let b: i64 = 7;
    let tong = a + b;
    println!("{}", tong);
}
"#,
    },
    // rustc: E0599 — `abs()` chỉ có trên số có dấu. Việc chọn phương thức theo
    // tính dấu, một điểm rất hay để dạy, bị xoá.
    HatGiongAm {
        ten: "abs_tren_u32",
        nhom: "so-hoc",
        ma_loi: "E0599",
        lo_hong: "tyck.rs (kieu_tra_ve_phuong_thuc tra theo TÊN) + interp.rs:1717",
        ma: r#"
fn main() {
    let x: u32 = 5;
    println!("{}", x.abs());
}
"#,
    },
    // rustc: E0308 — `i64::pow(self, exp: u32)`, số mũ BẮT BUỘC u32.
    // Ràng buộc u32 của thư viện chuẩn bị vô hiệu hoàn toàn.
    HatGiongAm {
        ten: "pow_doi_so_i64",
        nhom: "so-hoc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (GoiPhuongThuc không có bảng chữ ký) + interp.rs:1718 (tự ép u32)",
        ma: r#"
fn main() {
    let co_so: i64 = 2;
    let mu: i64 = 10;
    println!("{}", co_so.pow(mu));
}
"#,
    },

    // ═══════════════════════════════════════════════════════════════════════
    // NHÓM 2 — MƯỢN, KHẢ BIẾN CỦA THAM CHIẾU, VÒNG ĐỜI
    //
    // `enum T` (tyck.rs:34-46) chỉ có `Tham(Box<T>)` — KHÔNG phân biệt `&T` với
    // `&mut T`. `tu_kieu_ast` (tyck.rs:113) viết `Kieu::ThamChieu { ben_trong,
    // .. }`, dấu `..` vứt bỏ đúng cờ khả biến. Ba luật borrow mà ADR-002 §2
    // nhắc tên (E0499) hoặc người học gặp hằng ngày (E0502, E0596, E0507,
    // E0597) đều KHÔNG TỒN TẠI trong mã: `grep -rn 'E0499\|E0502\|E0507\|
    // E0596\|E0597\|E0106' crates/byte-rust/src/` không ra kết quả nào.
    // ═══════════════════════════════════════════════════════════════════════

    // rustc: E0308 — `&T` và `&mut T` là hai kiểu khác nhau; chỉ có chiều
    // `&mut T -> &T` mới ép ngầm được. interp bắt được (BR0536) nhưng là kiểm
    // ĐỘNG: đặt phép ghi vào nhánh không chạy là thoát sạch.
    HatGiongAm {
        ten: "tham_chieu_thuong_cho_tham_so_mut",
        nhom: "muon",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (T::Tham không mang cờ khả biến; tu_kieu_ast:113 vứt bằng `..`)",
        ma: r#"
fn tang(n: &mut i64, co_tang: bool) {
    if co_tang {
        *n += 1;
    }
}

fn main() {
    let mut x = 5;
    tang(&x, false);
    println!("{}", x);
}
"#,
    },
    // rustc: E0594 — ghi qua `&i64`. rustc kiểm TĨNH thân hàm, không quan tâm
    // nhánh có chạy hay không. Chỗ DUY NHẤT biết `co_the_sua` là lúc thực thi.
    HatGiongAm {
        ten: "ghi_qua_tham_chieu_chi_doc",
        nhom: "muon",
        ma_loi: "E0594",
        lo_hong: "tyck.rs (không mô hình khả biến) + interp.rs:1263 (BR0536 chỉ kiểm động)",
        ma: r#"
fn dat(n: &i64, co_dat: bool) {
    if co_dat {
        *n = 10;
    }
}

fn main() {
    let x = 5;
    dat(&x, false);
    println!("{}", x);
}
"#,
    },
    // rustc: E0106 — `&i64` không có nguồn lifetime nào. Không module nào kiểm
    // tính hợp lệ của lifetime trong chữ ký; interp cấp phát qua Rc nên ngữ
    // nghĩa runtime KHÔNG THỂ tái tạo lớp lỗi này.
    HatGiongAm {
        ten: "tra_ve_tham_chieu_toi_bien_cuc_bo",
        nhom: "muon",
        ma_loi: "E0106",
        lo_hong: "tyck.rs (không có luật lifetime nào; mong == t_than == T::Tham(SoNguyen))",
        ma: r#"
fn lay() -> &i64 {
    let x = 5;
    &x
}

fn main() {
    println!("{}", lay());
}
"#,
    },
    // rustc: E0614 — `i64` không cài `Deref`. rustc kiểm kiểu MỌI hàm, kể cả
    // hàm không ai gọi. tyck.rs:258-261 có nhánh `khac => khac` nuốt im lặng.
    HatGiongAm {
        ten: "giai_tham_gia_tri_khong_phai_tham_chieu",
        nhom: "muon",
        ma_loi: "E0614",
        lo_hong: "tyck.rs:258-261 (GiaiTham: `khac => khac` nuốt mọi phép giải tham chiếu sai)",
        ma: r#"
fn xau(x: i64) -> i64 {
    *x
}

fn main() {
    let x = 7;
    println!("{}", x);
}
"#,
    },
    // rustc: E0499 — hai `&mut x` cùng sống trong một lời gọi. ĐÂY LÀ LỆCH
    // GIỮA TÀI LIỆU VÀ MÃ: ADR-002 dòng 42 hứa đích danh luật này, nhưng
    // `grep -rn E0499 crates/byte-rust/src/` KHÔNG CÓ KẾT QUẢ.
    HatGiongAm {
        ten: "hai_muon_mut_trong_cung_mot_cau_lenh",
        nhom: "muon",
        ma_loi: "E0499",
        lo_hong: "move_check.rs (LUẬT ADR-002 §2.2 KHÔNG TỒN TẠI: TrangThai không có trạng thái mượn)",
        ma: r#"
fn cong(a: &mut i64, b: &mut i64) -> i64 {
    *a + *b
}

fn main() {
    let mut x = 1;
    let t = cong(&mut x, &mut x);
    println!("{}", t);
}
"#,
    },
    // rustc: E0499 — biến thể qua nhiều câu lệnh, trên chuỗi THẲNG HÀNG không
    // rẽ nhánh: đúng loại tình huống ADR-002 nói "quyết định được không cần CFG".
    HatGiongAm {
        ten: "hai_muon_mut_qua_hai_cau_lenh",
        nhom: "muon",
        ma_loi: "E0499",
        lo_hong: "move_check.rs:68 (`BieuThuc::Muon { .. } => false` nên a,b không được theo dõi)",
        ma: r#"
fn main() {
    let mut v = vec![1];
    let a = &mut v;
    let b = &mut v;
    a.push(2);
    b.push(3);
    println!("{}", v.len());
}
"#,
    },
    // rustc: E0502 — không trộn một `&mut` với một `&` cùng lúc.
    // Không có bảng theo dõi phép mượn nào trong crate.
    HatGiongAm {
        ten: "doc_bien_khi_dang_bi_muon_mut",
        nhom: "muon",
        ma_loi: "E0502",
        lo_hong: "move_check.rs:280,291 (Muon chỉ duyệt con; GoiPhuongThuc coi receiver vô hại)",
        ma: r#"
fn main() {
    let mut v = vec![1, 2, 3];
    let r = &mut v;
    let n = v.len();
    r.push(4);
    println!("{}", n);
}
"#,
    },
    // rustc: E0596 — `let v` không `mut` thì không lấy được `&mut v`. Thông tin
    // cần thiết ĐÃ ĐƯỢC THU THẬP RỒI BỊ VỨT ĐI: `ten_rang_buoc` (ast.rs:117)
    // trả `(ten, co_the_sua, span)`, tyck.rs:425 và :572 đều viết `(n, _, _)`.
    HatGiongAm {
        ten: "muon_mut_tu_binding_khong_khai_bao_mut",
        nhom: "muon",
        ma_loi: "E0596",
        lo_hong: "tyck.rs:425,572 (`for (n, _, _) in ten` vứt cờ co_the_sua đã thu thập)",
        ma: r#"
fn them(v: &mut Vec<i64>) {
    v.push(9);
}

fn main() {
    let v = vec![1, 2];
    them(&mut v);
    println!("{}", v.len());
}
"#,
    },
    // rustc: E0507 — mượn thì không cho chuyển quyền sở hữu đi. move_check gom
    // `&x` và `*x` vào CÙNG một nhánh: coi phép move bị cấm là phép đọc vô hại.
    HatGiongAm {
        ten: "move_ra_khoi_tham_chieu_chia_se",
        nhom: "muon",
        ma_loi: "E0507",
        lo_hong: "move_check.rs:280 (Muon và GiaiTham chung nhánh) + tyck.rs:258 (bóc T::Tham)",
        ma: r#"
fn lay(s: &String) -> String {
    *s
}

fn main() {
    let a = String::from("x");
    let b = lay(&a);
    println!("{}", b);
}
"#,
    },
    // rustc: E0594 — tham chiếu chỉ đọc không cho ghi, kể cả ghi vào trường con.
    // Ca DUY NHẤT trong nhóm mà phép ghi sai THỰC SỰ CHẠY mà vẫn lọt: ghi qua
    // `*r` bị chặn (BR0536) còn ghi qua `r.truong` thì lọt.
    HatGiongAm {
        ten: "ghi_truong_struct_qua_tham_chieu_chi_doc",
        nhom: "muon",
        ma_loi: "E0594",
        lo_hong: "interp.rs:1232-1238 (gan_vao/TruyCapTruong: giai_tham() xoá cờ rồi insert)",
        ma: r#"
struct Diem {
    x: i64,
}

fn doi(p: &Diem) {
    p.x = 9;
}

fn main() {
    let p = Diem { x: 1 };
    doi(&p);
    println!("{}", p.x);
}
"#,
    },
    // rustc: E0597 — con trỏ treo kinh điển. interp KHÔNG THỂ phát hiện về
    // nguyên tắc: giá trị nằm trong Rc nên khi khối đóng, ô nhớ vẫn sống.
    HatGiongAm {
        ten: "tham_chieu_song_lau_hon_thu_no_tro_toi",
        nhom: "muon",
        ma_loi: "E0597",
        lo_hong: "move_check.rs (ngăn xếp phạm vi chỉ tra move, không đối chiếu vùng sống mượn)",
        ma: r#"
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("{}", r);
}
"#,
    },
    // rustc: E0308 — không có ép ngầm `&i64 -> i64`. LỖ HỔNG KÉP: `Let` không
    // so kiểu, và kể cả nếu có so thì `chac_chan_lech` nhánh `(a, Tham(b))`
    // cũng bóc lớp tham chiếu ra rồi trả false.
    HatGiongAm {
        ten: "let_khai_bao_kieu_gia_tri_nhung_gan_tham_chieu",
        nhom: "muon",
        ma_loi: "E0308",
        lo_hong: "tyck.rs:420-425 (Let ghi đè) + tyck.rs:78 (`(a, Tham(b))` bóc lớp tham chiếu)",
        ma: r#"
fn main() {
    let x = 5;
    let y: i64 = &x;
    println!("{}", y);
}
"#,
    },

    // ═══════════════════════════════════════════════════════════════════════
    // NHÓM 3 — CHUYỂN QUYỀN SỞ HỮU
    //
    // Gốc rễ chung: `ghi_move` (move_check.rs:141-142) mở đầu bằng
    // `let BieuThuc::DuongDan { doan, span } = bt else { return };` — nó chỉ
    // biết TÊN BIẾN TRẦN. Mọi phép move qua đường dẫn (`p.ten`, `t.0`, `v[0]`)
    // đều thoát im lặng. Và bốn nhánh duyệt (Tuple, Macro, KhopMau, Cho) đọc
    // toán hạng mà không bao giờ gọi `ghi_move`.
    // ═══════════════════════════════════════════════════════════════════════

    // rustc: E0382 — đưa `s` vào tuple literal là chuyển quyền sở hữu.
    // Đối chiếu: nhánh `Mang` (:349) và `GoiHam` (:284) đều `ghi_move`.
    HatGiongAm {
        ten: "tuple_nuot_move",
        nhom: "move",
        ma_loi: "E0382",
        lo_hong: "move_check.rs:344-348 (Tuple|Macro chỉ `bieu_thuc(e)`, thiếu `ghi_move(e)`)",
        ma: r#"
fn main() {
    let s = String::from("chao");
    let t = (s, 1);
    println!("{}", s);
    println!("{}", t.1);
}
"#,
    },
    // rustc: E0382 (hai lỗi riêng biệt). Dạng hay gặp trong bài dạy hoán đổi
    // biến ⇒ mức nguy hiểm sư phạm cao.
    HatGiongAm {
        ten: "let_tuple_thao",
        nhom: "move",
        ma_loi: "E0382",
        lo_hong: "move_check.rs:344 (Tuple) + :248 (Let gọi ghi_move nhưng vế phải không phải DuongDan)",
        ma: r#"
fn main() {
    let a = String::from("x");
    let b = String::from("y");
    let (c, d) = (a, b);
    println!("{} {} {} {}", c, d, a, b);
}
"#,
    },
    // rustc: E0382 — `match x` với mẫu `Some(s)` ràng buộc theo giá trị nên
    // nuốt `x`. Đối tượng match được đánh giá THẲNG HÀNG trước khi rẽ nhánh,
    // nên đây đúng là ca luật 1 của ADR-002 §2 phải bắt được.
    HatGiongAm {
        ten: "match_doi_tuong_bi_move",
        nhom: "move",
        ma_loi: "E0382",
        lo_hong: "move_check.rs:384-401 (KhopMau chỉ `bieu_thuc(gia_tri)`, không `ghi_move`)",
        ma: r#"
fn main() {
    let x: Option<String> = Some(String::from("chao"));
    match x {
        Some(s) => println!("{}", s),
        None => println!("rong"),
    }
    match x {
        Some(s) => println!("{}", s),
        None => println!("rong"),
    }
}
"#,
    },
    // rustc: E0382 — `IntoIterator::into_iter(v)` nhận `self` theo giá trị.
    // Lỗi kinh điển bậc nhất của người mới học ownership; tick xanh oan ở đây
    // gây hại nhất. `moc_vong_lap` được xây công phu để bắt move vào TRONG
    // thân vòng lặp, nhưng bỏ trống chính cái bị vòng lặp nuốt.
    HatGiongAm {
        ten: "for_nuot_vec",
        nhom: "move",
        ma_loi: "E0382",
        lo_hong: "move_check.rs:417-418 (Cho: `day` được coi là ĐỌC chứ không phải MOVE)",
        ma: r#"
fn main() {
    let v = vec![1, 2, 3];
    for x in v {
        println!("{}", x);
    }
    println!("{}", v.len());
}
"#,
    },
    // rustc: E0382 — partial move: trường `ten` kiểu String rời khỏi `p`.
    // `TrangThai` gắn với TÊN BIẾN TRẦN chứ không gắn với đường dẫn nơi chốn.
    HatGiongAm {
        ten: "truong_struct_move_mot_phan",
        nhom: "move",
        ma_loi: "E0382",
        lo_hong: "move_check.rs:141-142 (ghi_move chỉ nhận DuongDan) + :336 (TruyCapTruong = đọc)",
        ma: r#"
struct Nguoi { ten: String, tuoi: i64 }
fn main() {
    let p = Nguoi { ten: String::from("Byte"), tuoi: 7 };
    let t = p.ten;
    println!("{} {}", t, p.ten);
}
"#,
    },
    // rustc: E0382 — biến thể tuple của cùng lỗ hổng partial move. Parser biểu
    // diễn `t.0` cũng bằng `TruyCapTruong`, nên bản vá chỉ xử lý trường struct
    // có tên vẫn phải phủ luôn ca này.
    HatGiongAm {
        ten: "truong_tuple_move_mot_phan",
        nhom: "move",
        ma_loi: "E0382",
        lo_hong: "move_check.rs:141-142, :336 (biến thể tuple của partial move)",
        ma: r#"
fn main() {
    let t = (String::from("a"), String::from("b"));
    let x = t.0;
    println!("{} {}", x, t.0);
}
"#,
    },
    // rustc: E0382 — `fn an(self)` nhận receiver theo GIÁ TRỊ. Giả định "mọi
    // receiver đều là mượn" đúng cho phương thức dựng sẵn nhưng SAI với
    // `fn f(self)` do người học tự viết; chữ ký nằm sẵn trong AST (Muc::Impl).
    HatGiongAm {
        ten: "phuong_thuc_nuot_self",
        nhom: "move",
        ma_loi: "E0382",
        lo_hong: "move_check.rs:291-300 (GoiPhuongThuc cố ý bỏ qua tên phương thức: `let _ = ten;`)",
        ma: r#"
struct Hop { n: i64 }
impl Hop {
    fn an(self) -> i64 { self.n }
}
fn main() {
    let h = Hop { n: 5 };
    let a = h.an();
    let b = h.an();
    println!("{} {}", a, b);
}
"#,
    },
    // rustc: E0382 — `vec![s]` nuốt `s`. Gộp `Macro` chung với `Tuple` an toàn
    // cho `println!`/`format!` nhưng sai với `vec!`: phải tách theo TÊN macro.
    HatGiongAm {
        ten: "vec_macro_nuot_move",
        nhom: "move",
        ma_loi: "E0382",
        lo_hong: "move_check.rs:344-348 (Macro gộp chung Tuple, chỉ ĐỌC đối số)",
        ma: r#"
fn main() {
    let s = String::from("chao");
    let v = vec![s];
    println!("{} {}", v.len(), s);
}
"#,
    },
    // rustc: E0382 — `impl Add<&str> for String` nhận `self` theo giá trị
    // (chính lý do Rust bắt viết `a + &b`). `co_the_khong_copy` (:62) ĐÃ xử lý
    // HaiNgoi nhưng lượt duyệt không dùng thông tin đó.
    HatGiongAm {
        ten: "cong_chuoi_nuot_move",
        nhom: "move",
        ma_loi: "E0382",
        lo_hong: "move_check.rs:332-335 (HaiNgoi chỉ `bieu_thuc`, không bao giờ `ghi_move`)",
        ma: r#"
fn main() {
    let a = String::from("chao");
    let b = a + "!";
    println!("{} {}", b, a);
}
"#,
    },
    // rustc: E0507 — `Index` chỉ trả tham chiếu. Thiếu luật là chấp nhận được
    // theo ADR-002, NHƯNG khi đó ca này phải rơi vào `ChuaHoTro`, không phải `Dat`.
    HatGiongAm {
        ten: "chi_so_move_ra_khoi_vec",
        nhom: "move",
        ma_loi: "E0507",
        lo_hong: "move_check.rs:337-340 (ChiSo chỉ đọc); không có luật E0507 nào trong crate",
        ma: r#"
fn main() {
    let v = vec![String::from("a"), String::from("b")];
    let s = v[0];
    println!("{}", s);
}
"#,
    },
    // rustc: E0507 — gộp `&x` với `*x` là sai hướng: `&x` mượn (an toàn), `*x`
    // lấy hẳn. Dạy NGƯỢC đúng luật "không lấy được đồ ra khỏi thứ mình chỉ mượn".
    HatGiongAm {
        ten: "giai_tham_move",
        nhom: "move",
        ma_loi: "E0507",
        lo_hong: "move_check.rs:280-282 (GiaiTham gộp chung nhánh với Muon)",
        ma: r#"
fn lay(r: &String) -> String {
    *r
}
fn main() {
    let s = String::from("chao");
    println!("{}", lay(&s));
}
"#,
    },
    // rustc: E0382 — chứng minh vá riêng từng nhánh duyệt là CHƯA ĐỦ: nhánh
    // `KhoiTaoStruct` (:301-309) LÀM ĐÚNG phần của nó (`ghi_move` mỗi trường),
    // nhưng `ghi_move` vẫn thoát vì `h.s` không phải `DuongDan`.
    HatGiongAm {
        ten: "khoi_tao_struct_tu_truong",
        nhom: "move",
        ma_loi: "E0382",
        lo_hong: "move_check.rs:141-142 (gốc rễ: ghi_move chỉ nhận tên biến trần)",
        ma: r#"
struct Hop { s: String }
fn main() {
    let h = Hop { s: String::from("a") };
    let g = Hop { s: h.s };
    println!("{} {}", g.s, h.s);
}
"#,
    },

    // ═══════════════════════════════════════════════════════════════════════
    // NHÓM 4 — KHỚP MẪU & TÍNH VÉT CẠN
    //
    // `khop_mau` trong tyck.rs bọc toàn bộ khối kiểm vét cạn trong
    // `if let T::Enum(ten_enum) = &t_gt`. Mọi scrutinee kiểu khác (Bool,
    // SoNguyen, Tuple, Mo) đi thẳng qua. Và ngay với enum, phép đếm phủ chỉ
    // làm ở TẦNG NGOÀI CÙNG, không đệ quy vào mẫu con, không xét guard.
    // ═══════════════════════════════════════════════════════════════════════

    // rustc: E0004 — nhánh có guard KHÔNG được tính là phủ.
    HatGiongAm {
        ten: "bao_quat_co_dieu_kien",
        nhom: "khop-mau",
        ma_loi: "E0004",
        lo_hong: "tyck.rs (khop_mau: BoQua/Ten đặt co_bao_quat=true mà không xét n.dieu_kien)",
        ma: r#"
enum Mau {
    Do,
    Xanh,
}

fn ma_mau(m: Mau) -> i64 {
    match m {
        Mau::Do => 1,
        _ if false => 2,
    }
}

fn main() {
    println!("{}", ma_mau(Mau::Do));
}
"#,
    },
    // rustc: E0004 — mẫu con `Mau::Do` bác bỏ được nên nhánh không phủ hết.
    HatGiongAm {
        ten: "enum_long_nhau",
        nhom: "khop-mau",
        ma_loi: "E0004",
        lo_hong: "tyck.rs (khop_mau: `duong_dan.last()` coi biến thể đã phủ trọn, không đệ quy mẫu con)",
        ma: r#"
enum Mau {
    Do,
    Xanh,
}

enum Hop {
    Chua(Mau),
}

fn ma_hop(h: Hop) -> i64 {
    match h {
        Hop::Chua(Mau::Do) => 1,
    }
}

fn main() {
    println!("{}", ma_hop(Hop::Chua(Mau::Do)));
}
"#,
    },
    // rustc: E0004 — `match` trên `bool` thiếu nhánh `false`.
    HatGiongAm {
        ten: "bool_thieu_nhanh",
        nhom: "khop-mau",
        ma_loi: "E0004",
        lo_hong: "tyck.rs (khối vét cạn bọc trong `if let T::Enum(..)`; Mau::HangSo không vào danh sách phủ)",
        ma: r#"
fn phan_loai(b: bool) -> i64 {
    match b {
        true => 1,
    }
}

fn main() {
    println!("{}", phan_loai(true));
}
"#,
    },
    // rustc: E0004 — thiếu `i64::MIN..=-1` và `2..=i64::MAX`.
    HatGiongAm {
        ten: "so_nguyen_thieu_nhanh",
        nhom: "khop-mau",
        ma_loi: "E0004",
        lo_hong: "tyck.rs (`if let T::Enum(..)` bỏ qua T::SoNguyen; Mau::HangSo rơi vào `_ => {}`)",
        ma: r#"
fn diem(n: i64) -> i64 {
    match n {
        0 => 10,
        1 => 20,
    }
}

fn main() {
    println!("{}", diem(0));
}
"#,
    },
    // rustc: E0004 — thiếu `(true, false)` và `(false, true)`. `kieu_cua` chỉ
    // trả `T::Rong` cho tuple RỖNG; tuple không rỗng cho `T::Mo` ⇒ tắt kiểm.
    HatGiongAm {
        ten: "tuple_thieu_nhanh",
        nhom: "khop-mau",
        ma_loi: "E0004",
        lo_hong: "tyck.rs (kieu_cua: Tuple không rỗng -> T::Mo, `Mo` không bao giờ sinh lỗi)",
        ma: r#"
fn ket_hop(a: bool, b: bool) -> i64 {
    match (a, b) {
        (true, true) => 1,
        (false, false) => 2,
    }
}

fn main() {
    println!("{}", ket_hop(true, true));
}
"#,
    },
    // rustc: E0308 — mọi nhánh `match` phải cho ra cùng một kiểu.
    HatGiongAm {
        ten: "nhanh_lech_kieu",
        nhom: "khop-mau",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (khop_mau: chỉ `if kieu_nhanh == T::Mo { kieu_nhanh = t; }`, không đối chiếu)",
        ma: r#"
enum Mau {
    Do,
    Xanh,
}

fn ma_mau(m: Mau) -> i64 {
    match m {
        Mau::Do => 1,
        Mau::Xanh => String::from("xanh"),
    }
}

fn main() {
    println!("{}", ma_mau(Mau::Do));
}
"#,
    },
    // rustc: E0308 — scrutinee kiểu `A` nhưng mẫu là biến thể của `B`. CẢ HAI
    // TẦNG bỏ qua tiền tố đường dẫn: `B::Do` vừa được tính là phủ `A::Do`
    // vừa khớp lúc chạy.
    HatGiongAm {
        ten: "mau_sai_enum",
        nhom: "khop-mau",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (khop_mau: `duong_dan.last()`) + interp.rs:403-407 (chỉ so tên biến thể)",
        ma: r#"
enum A {
    Do,
    Xanh,
}

enum B {
    Do,
    Xanh,
}

fn ma_a(a: A) -> i64 {
    match a {
        B::Do => 1,
        B::Xanh => 2,
    }
}

fn main() {
    println!("{}", ma_a(A::Do));
}
"#,
    },
    // rustc: E0308 — mẫu `Mau::Do` không thể khớp giá trị kiểu `i64`.
    HatGiongAm {
        ten: "mau_khong_the_xay_ra",
        nhom: "khop-mau",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (khop_mau không gọi chac_chan_lech giữa t_gt và kiểu mẫu) + interp.rs:444",
        ma: r#"
enum Mau {
    Do,
    Xanh,
}

fn phan_loai(n: i64) -> i64 {
    match n {
        Mau::Do => 1,
        _ => 2,
    }
}

fn main() {
    println!("{}", phan_loai(7));
}
"#,
    },
    // rustc: E0408 — mọi nhánh của mẫu `Or` phải ràng buộc CÙNG một tập biến.
    HatGiongAm {
        ten: "or_rang_buoc_lech",
        nhom: "khop-mau",
        ma_loi: "E0408",
        lo_hong: "tyck.rs (khop_mau gọi ten_rang_buoc một lần cho cả mẫu Or, không so tập tên)",
        ma: r#"
fn chon(p: (i64, i64)) -> i64 {
    match p {
        (x, 0) | (0, _) => 1,
        _ => 0,
    }
}

fn main() {
    println!("{}", chon((5, 0)));
}
"#,
    },
    // rustc: E0004 — `Some(1)` chỉ phủ đúng một giá trị. Cùng lỗ hổng với
    // enum_long_nhau nhưng trên `Option` dựng sẵn, tức trúng ngay ví dụ dạy
    // học phổ biến nhất.
    HatGiongAm {
        ten: "payload_hang_so",
        nhom: "khop-mau",
        ma_loi: "E0004",
        lo_hong: "tyck.rs (khop_mau coi mọi Mau::BienThe là phủ TRỌN, không phân biệt payload)",
        ma: r#"
fn doc(o: Option<i64>) -> i64 {
    match o {
        Some(1) => 10,
        None => 0,
    }
}

fn main() {
    println!("{}", doc(Some(1)));
}
"#,
    },
    // rustc: E0023 — `Goi::Mot` có 1 trường nhưng mẫu viết 2. interp biến một
    // LỖI KIỂU thành "không khớp" lúc chạy, rồi nhánh `_` nuốt nốt hậu quả.
    HatGiongAm {
        ten: "sai_so_truong_bien_the",
        nhom: "khop-mau",
        ma_loi: "E0023",
        lo_hong: "interp.rs:412 (`if ps.len() != gia_tri.len() { return Ok(false) }`) + tyck.rs (không lưu arity)",
        ma: r#"
enum Goi {
    Mot(i64),
}

fn tong(g: Goi) -> i64 {
    match g {
        Goi::Mot(x, y) => x + y,
        _ => 0,
    }
}

fn main() {
    println!("{}", tong(Goi::Mot(3)));
}
"#,
    },
    // rustc: E0599 — `s` có kiểu `i64`, không có `len`. Payload của biến thể
    // không bao giờ được suy kiểu: mọi biến ràng buộc thành `T::Mo`.
    HatGiongAm {
        ten: "phuong_thuc_sai_tren_bien_mau",
        nhom: "khop-mau",
        ma_loi: "E0599",
        lo_hong: "tyck.rs (khop_mau: `self.dat(&t, T::Mo)` cho mọi biến ràng buộc) + kieu_tra_ve_phuong_thuc",
        ma: r#"
fn do_dai(o: Option<i64>) -> i64 {
    match o {
        Some(s) => s.len(),
        None => 0,
    }
}

fn main() {
    println!("{}", do_dai(None));
}
"#,
    },
    // rustc: E0004 — hai khoảng không phủ hết `i64`. Kiểm phủ khoảng đòi phân
    // tích khoảng — hoàn toàn vắng mặt (Mau::Dai rơi vào `_ => {}`).
    HatGiongAm {
        ten: "dai_thieu_nhanh",
        nhom: "khop-mau",
        ma_loi: "E0004",
        lo_hong: "tyck.rs (khop_mau không có nhánh nào xử lý Mau::Dai)",
        ma: r#"
fn xep_hang(diem: i64) -> i64 {
    match diem {
        0..=5 => 1,
        6..=9 => 2,
    }
}

fn main() {
    println!("{}", xep_hang(3));
}
"#,
    },
    // rustc: E0026 — `Diem` không có trường `z`. interp biến "trường không tồn
    // tại" thành "không khớp"; tyck có sẵn bảng `truong_struct` mà không tra.
    HatGiongAm {
        ten: "truong_struct_khong_ton_tai",
        nhom: "khop-mau",
        ma_loi: "E0026",
        lo_hong: "interp.rs:434 (`f.get(k)` None -> không khớp) + tyck.rs (khop_mau không tra truong_struct)",
        ma: r#"
struct Diem {
    x: i64,
    y: i64,
}

fn tong(d: Diem) -> i64 {
    match d {
        Diem { x, z } => x + z,
        _ => 0,
    }
}

fn main() {
    let d = Diem { x: 1, y: 2 };
    println!("{}", tong(d));
}
"#,
    },
    // rustc: E0027 — mẫu struct thiếu trường `y` mà không có `..`.
    HatGiongAm {
        ten: "thieu_truong_struct",
        nhom: "khop-mau",
        ma_loi: "E0027",
        lo_hong: "interp.rs:431-440 (chỉ duyệt trường CÓ trong mẫu; cờ `..` không được xét)",
        ma: r#"
struct Diem {
    x: i64,
    y: i64,
}

fn lay_x(d: Diem) -> i64 {
    match d {
        Diem { x } => x,
        _ => 0,
    }
}

fn main() {
    let d = Diem { x: 7, y: 2 };
    println!("{}", lay_x(d));
}
"#,
    },

    // ═══════════════════════════════════════════════════════════════════════
    // NHÓM 5 — KIỂU CỦA BIỂU THỨC & VÙNG MÙ MÃ CHẾT
    //
    // Hai mẫu tấn công tổng quát:
    //   (a) Biểu thức rẽ nhánh (`if`, `match`) chỉ giữ kiểu nhánh ĐẦU TIÊN
    //       khác `Mo` và VỨT kiểu các nhánh còn lại — không bao giờ đối chiếu.
    //   (b) Đẩy mã sai vào nhánh không chạy hoặc hàm không được gọi, để vô hiệu
    //       hoá TOÀN BỘ tầng interp. rustc kiểm tĩnh mọi hàm; interp thì không.
    // ═══════════════════════════════════════════════════════════════════════

    // rustc: E0308 — hai nhánh `if` phải cùng kiểu. Kiểu nhánh `else` bị vứt
    // hoàn toàn nên `T::Chuoi` không bao giờ va chạm với `T::SoNguyen`.
    HatGiongAm {
        ten: "nhanh_if_lech_kieu",
        nhom: "kieu-bieu-thuc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (BieuThuc::Neu: `if a == T::Mo { b } else { a }`, không gọi chac_chan_lech)",
        ma: r#"
fn phan_loai(x: i64) -> i64 {
    if x > 0 {
        1
    } else {
        String::from("am")
    }
}

fn main() {
    println!("{}", phan_loai(5));
}
"#,
    },
    // rustc: E0308 — MỌI đường `return` phải khớp chữ ký, không chỉ biểu thức
    // cuối. Bộ kiểm không mang theo ngữ cảnh "kiểu trả về hàm hiện tại".
    HatGiongAm {
        ten: "tra_ve_som_sai_kieu",
        nhom: "kieu-bieu-thuc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (duyet_con/TraVe: tính kiểu rồi VỨT, không đối chiếu chữ ký hàm bao ngoài)",
        ma: r#"
fn kiem_tra(x: i64) -> i64 {
    if x > 100 {
        return true;
    }
    x * 2
}

fn main() {
    println!("{}", kiem_tra(3));
}
"#,
    },
    // rustc: E0061 — rustc kiểm số đối số TĨNH nên nhánh không bao giờ chạy
    // vẫn bị từ chối. Kiểm arity CHỈ tồn tại ở interp (BR0506), tức chỉ ĐỘNG.
    HatGiongAm {
        ten: "thieu_doi_so_nhanh_chet",
        nhom: "kieu-bieu-thuc",
        ma_loi: "E0061",
        lo_hong: "tyck.rs (GoiHam không bao giờ so doi_so.len() với ck.tham_so.len())",
        ma: r#"
fn cong(a: i64, b: i64) -> i64 {
    a + b
}

fn main() {
    let n = 2;
    if n > 5 {
        println!("{}", cong(1));
    }
    println!("{}", cong(1, 2));
}
"#,
    },
    // rustc: E0061 — đối số dư ra rơi vào `mong.get(i) == None` nên bị BỎ QUA
    // im lặng bởi `if let Some(m) = mong.get(i)`.
    HatGiongAm {
        ten: "thua_doi_so_nhanh_chet",
        nhom: "kieu-bieu-thuc",
        ma_loi: "E0061",
        lo_hong: "tyck.rs (GoiHam: `if let Some(m) = mong.get(i)` nuốt đối số thừa)",
        ma: r#"
fn binh_phuong(x: i64) -> i64 {
    x * x
}

fn main() {
    let n = 0;
    while n > 0 {
        println!("{}", binh_phuong(2, 3));
    }
    println!("{}", binh_phuong(4));
}
"#,
    },
    // rustc: E0308 — các nhánh `match` phải cùng kiểu.
    HatGiongAm {
        ten: "nhanh_match_lech_kieu",
        nhom: "kieu-bieu-thuc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (khop_mau: chỉ ghi nhận kiểu nhánh đầu tiên khác Mo)",
        ma: r#"
fn ma_hoa(x: i64) -> i64 {
    match x {
        0 => 0,
        _ => false,
    }
}

fn main() {
    println!("{}", ma_hoa(0));
}
"#,
    },
    // rustc: E0308 — nhánh `else` gọi hàm trả `bool`. Đáng chú ý: tyck ĐÃ tra
    // đúng chữ ký `la_chan` và tính ra `T::Bool`, nhưng vứt ngay giá trị đó.
    // Thông tin cần thiết đã có sẵn, chỉ thiếu bước đối chiếu hai nhánh.
    HatGiongAm {
        ten: "de_quy_ho_tuong_lech_kieu",
        nhom: "kieu-bieu-thuc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (BieuThuc::Neu vứt kiểu nhánh else dù đã tính đúng)",
        ma: r#"
fn la_chan(n: i64) -> bool {
    n == 0
}

fn la_le(n: i64) -> i64 {
    if n == 0 {
        0
    } else {
        la_chan(n - 1)
    }
}

fn main() {
    println!("{}", la_le(0));
}
"#,
    },
    // rustc: E0308 — `String::push_str` trả `()`. BẢNG SAI, không phải lỗ `Mo`:
    // tyck TỰ TIN xác nhận một chương trình sai. Nguy hiểm hơn lỗ `Mo` vì bộ
    // kiểm chủ động khẳng định sai thay vì im lặng.
    HatGiongAm {
        ten: "push_str_tra_ve_unit",
        nhom: "kieu-bieu-thuc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (kieu_tra_ve_phuong_thuc: \"push_str\" => T::Chuoi — SAI, thực tế trả `()`)",
        ma: r#"
fn noi_chuoi() -> String {
    let mut s = String::from("xin ");
    s.push_str("chao")
}

fn main() {
    println!("{}", noi_chuoi());
}
"#,
    },
    // rustc: E0308 — dấu `;` biến nhánh `else` thành `()`. Bất đối xứng đã
    // kiểm chứng: `;` ở CUỐI THÂN HÀM thì tyck BẮT ĐƯỢC (BR0302), nhưng `;` ở
    // cuối một NHÁNH `else` thì lọt hoàn toàn.
    HatGiongAm {
        ten: "cham_phay_o_nhanh_else",
        nhom: "kieu-bieu-thuc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (BieuThuc::Neu: `b` = T::Rong bị vứt, `a` = T::SoNguyen được trả về)",
        ma: r#"
fn lay_hoac_khong(x: i64) -> i64 {
    if x > 0 {
        x
    } else {
        0;
    }
}

fn main() {
    println!("{}", lay_hoac_khong(5));
}
"#,
    },
    // rustc: E0425 — phân giải tên là TĨNH, nhánh chết vẫn bị từ chối. tyck
    // không hề có nhánh `else` báo "không tìm thấy hàm"; lỗi chỉ ở interp (BR0550).
    HatGiongAm {
        ten: "ham_khong_ton_tai_nhanh_chet",
        nhom: "kieu-bieu-thuc",
        ma_loi: "E0425",
        lo_hong: "tyck.rs (GoiHam: `if let Some(ck) = self.ham.get(..)` không có else, trả T::Mo)",
        ma: r#"
fn main() {
    let n = 0;
    if n > 0 {
        println!("{}", ham_chua_viet(1));
    }
    println!("xong");
}
"#,
    },
    // rustc: E0308 — `cac[0]` là `bool`. Phép chỉ số vào Vec LUÔN cho `T::Mo`,
    // mà `chac_chan_lech` có luật đầu tiên `(Mo, _) | (_, Mo) => false`.
    HatGiongAm {
        ten: "doi_so_sai_kieu_qua_chi_so",
        nhom: "kieu-bieu-thuc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (kieu_cua KHÔNG có nhánh riêng cho BieuThuc::ChiSo -> `khac => T::Mo`)",
        ma: r#"
fn nhan_so(x: i64) -> i64 {
    x
}

fn main() {
    let cac = vec![true, false];
    let phan_tu = cac[0];
    println!("{}", nhan_so(phan_tu));
}
"#,
    },
    // rustc: E0061 — `str::len` không nhận đối số nào. Không có bảng arity cho
    // phương thức dựng sẵn ở CẢ tyck LẪN interp: lỗ hổng hai tầng.
    HatGiongAm {
        ten: "phuong_thuc_thua_doi_so",
        nhom: "kieu-bieu-thuc",
        ma_loi: "E0061",
        lo_hong: "tyck.rs (GoiPhuongThuc: đối số chỉ được duyệt cho có) + interp.rs (bỏ qua đối số thừa)",
        ma: r#"
fn main() {
    let s = String::from("xin chao");
    println!("{}", s.len(3));
}
"#,
    },
    // rustc: E0308 — MỌI hàm có biểu thức cuối là phép chỉ số đều thoát kiểm
    // chữ ký BR0302, kể cả khi kiểu phần tử đã biết rõ từ tham số `ds: Vec<i64>`.
    HatGiongAm {
        ten: "than_ham_tra_ve_chi_so",
        nhom: "kieu-bieu-thuc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (ChiSo -> T::Mo, `(Mo, _) => false` vô hiệu hoá BR0302)",
        ma: r#"
fn lay_ten(ds: Vec<i64>) -> String {
    ds[0]
}

fn main() {
    let v = vec![7, 8];
    println!("{}", lay_ten(v));
}
"#,
    },

    // ═══════════════════════════════════════════════════════════════════════
    // NHÓM 6 — STRUCT & ENUM
    //
    // `tyck::nap` NẠP SẴN `truong_struct: HashMap<String, HashMap<String, T>>`
    // với đúng kiểu từng trường — nhưng bảng đó chỉ được tra khi ĐỌC trường.
    // Chiều khởi tạo, chiều mẫu, chiều ghi đều không tra. Ở tầng interp,
    // `KhoiTaoStruct` chỉ kiểm chiều THIẾU (BR0510), không kiểm chiều THỪA,
    // và dựng struct cho cả tên chưa khai báo.
    // ═══════════════════════════════════════════════════════════════════════

    // rustc: E0560 — `Diem` không có trường `z` ("all struct fields are already
    // assigned"). interp CHỈ kiểm chiều THIẾU (duyệt ThanStruct::TheoTen xem
    // trường nào vắng -> BR0510); chiều THỪA không có luật nào.
    HatGiongAm {
        ten: "truong_thua",
        nhom: "struct-enum",
        ma_loi: "E0560",
        lo_hong: "interp.rs:~858 (m.insert vô điều kiện, chỉ có BR0510 chiều thiếu) + tyck.rs:~272",
        ma: r#"
struct Diem { x: i64, y: i64 }

fn main() {
    let d = Diem { x: 1, y: 2, z: 3 };
    println!("{}", d.x);
}
"#,
    },
    // rustc: E0308 — expected `i64`, found `&str` tại `x: "ba"`. `tyck::nap`
    // (:172-178) CÓ nạp `truong_struct` với đúng kiểu từng trường, nhưng bảng
    // đó chỉ được tra ở nhánh `TruyCapTruong` khi ĐỌC trường. Nhánh
    // `KhoiTaoStruct` không tra bảng và không gọi `chac_chan_lech` trường nào.
    HatGiongAm {
        ten: "truong_sai_kieu",
        nhom: "struct-enum",
        ma_loi: "E0308",
        lo_hong: "tyck.rs (KhoiTaoStruct không đối chiếu kiểu trường với bảng truong_struct đã nạp sẵn)",
        ma: r#"
struct Diem { x: i64, y: i64 }

fn main() {
    let d = Diem { x: "ba", y: 2 };
    println!("{}", d.y);
}
"#,
    },
    // rustc: E0609 — rustc kiểm kiểu MỌI thân hàm, kể cả `lay_z` không được
    // gọi. Hai tầng cùng thủng: tyck trả `T::Mo` (im lặng theo nguyên tắc),
    // interp có luật đúng (BR0511) nhưng là luật ĐỘNG.
    HatGiongAm {
        ten: "doc_truong_khong_ton_tai",
        nhom: "struct-enum",
        ma_loi: "E0609",
        lo_hong: "tyck.rs:~324 (TruyCapTruong trả T::Mo khi tra trường trượt, dù ĐÃ biết chắc là T::Struct)",
        ma: r#"
struct Diem { x: i64, y: i64 }

fn lay_z(d: &Diem) -> i64 {
    d.z
}

fn main() {
    let d = Diem { x: 1, y: 2 };
    println!("{}", d.x);
}
"#,
    },
    // rustc: E0609 — nằm trên chính đường chạy của main (không cần thủ thuật
    // hàm-không-gọi). Đường ĐỌC có BR0511; đường GHI chỉ `insert` thẳng.
    // Byte-rust cho phép struct mọc thêm trường lúc chạy, y như JavaScript.
    HatGiongAm {
        ten: "ghi_truong_khong_ton_tai",
        nhom: "struct-enum",
        ma_loi: "E0609",
        lo_hong: "interp.rs:~1232 (gan_vao/TruyCapTruong insert trường mới vô điều kiện)",
        ma: r#"
struct Diem { x: i64, y: i64 }

fn main() {
    let mut d = Diem { x: 1, y: 2 };
    d.z = 9;
    println!("{}", d.x);
}
"#,
    },
    // rustc: E0609 — `Email` là struct tuple một phần tử nên chỉ có `.0`.
    // `nap` CHỈ nạp trường cho `ThanStruct::TheoTen`; struct tuple không bao
    // giờ vào `truong_struct` nên MỌI chỉ số (kể cả `.99`) đều trả `T::Mo`.
    HatGiongAm {
        ten: "tuple_struct_sai_chi_so",
        nhom: "struct-enum",
        ma_loi: "E0609",
        lo_hong: "tyck.rs:~171 (fn nap chỉ nạp ThanStruct::TheoTen, bỏ hẳn ThanStruct::TheoViTri)",
        ma: r#"
struct Email(String);

fn phan_hai(e: &Email) -> String {
    e.1
}

fn main() {
    let e = Email(String::from("a@b.c"));
    println!("{}", e.0);
}
"#,
    },
    // rustc: E0422 — `ToaDo` chưa khai báo. interp dựng map trường TRƯỚC rồi
    // `if let Some(sd) = self.struct_def.get(..)`: struct không tồn tại thì bỏ
    // qua luôn cả khối kiểm, và vẫn trả `Ok(GiaTri::Struct { .. })` từ hư không.
    HatGiongAm {
        ten: "struct_khong_ton_tai",
        nhom: "struct-enum",
        ma_loi: "E0422",
        lo_hong: "interp.rs:~858 (dựng GiaTri::Struct cho tên chưa khai báo) + tyck.rs (-> T::Mo)",
        ma: r#"
struct Diem { x: i64, y: i64 }

fn main() {
    let d = ToaDo { x: 1, y: 2 };
    println!("{}", d.x);
}
"#,
    },
    // rustc: E0308 + E0609. Mỉa mai là `chac_chan_lech` ĐÃ CÓ SẴN luật đúng
    // cho ca này (`(Struct(a), Struct(b)) => a != b`) và tyck ĐÃ dùng nó cho
    // đối số hàm (BR0301) và kiểu trả về (BR0302) — chỉ riêng `let` là quên.
    HatGiongAm {
        ten: "gan_sai_struct",
        nhom: "struct-enum",
        ma_loi: "E0308",
        lo_hong: "tyck.rs:~420 (fn khoi/CauLenh::Let: kiểu khai báo ghi đè, không đối chiếu)",
        ma: r#"
struct A { x: i64 }
struct B { y: i64 }

fn main() {
    let a = A { x: 1 };
    let b: B = a;
    println!("{}", b.x);
}
"#,
    },
    // rustc: E0026 — biến thể không có nhánh `_` đỡ, khác ca cùng nhóm khớp mẫu.
    HatGiongAm {
        ten: "mau_truong_khong_ton_tai",
        nhom: "struct-enum",
        ma_loi: "E0026",
        lo_hong: "tyck.rs:~438 (khop_mau không đối chiếu MauTruong::TheoTen với truong_struct)",
        ma: r#"
struct Diem { x: i64, y: i64 }

fn xem(d: Diem) -> i64 {
    match d {
        Diem { x, z } => x + z,
    }
}

fn main() {
    let d = Diem { x: 1, y: 2 };
    println!("{}", d.x);
}
"#,
    },
    // rustc: E0599 — `Mau` không có biến thể `Tim`. interp CHỈ kiểm VẾ TRÁI:
    // hễ enum tồn tại thì MỌI biến thể bịa ra đều hợp lệ.
    HatGiongAm {
        ten: "enum_bien_the_khong_ton_tai",
        nhom: "struct-enum",
        ma_loi: "E0599",
        lo_hong: "interp.rs:~1067 (tinh_duong_dan chỉ kiểm tên enum, không kiểm tên biến thể)",
        ma: r#"
enum Mau { Do, Xanh }

fn main() {
    let m = Mau::Tim;
    match m {
        Mau::Do => println!("do"),
        _ => println!("khac"),
    }
}
"#,
    },
    // rustc: E0061 — biến thể nhận 1 đối số, gọi với 2. Cả hai tầng bỏ qua
    // ARITY: tyck return sớm ở nhánh thuoc_enum, interp nhận bao nhiêu cũng được.
    HatGiongAm {
        ten: "enum_thua_tham_so",
        nhom: "struct-enum",
        ma_loi: "E0061",
        lo_hong: "tyck.rs:~283 (GoiHam: biến thể enum return sớm) + interp.rs:~1441 (bien_the_cua mọi arity)",
        ma: r#"
enum Hinh { Tron(i64), Vuong(i64) }

fn main() {
    let h = Hinh::Tron(1, 2);
    match h {
        Hinh::Vuong(c) => println!("{}", c),
        _ => println!("tron"),
    }
}
"#,
    },
    // rustc: E0308 — `Hinh::Tron` không kèm ngoặc là một fn item, không phải
    // giá trị `Hinh`. interp lặng lẽ dựng một `Tron` RỖNG — một giá trị mà
    // kiểu `Hinh` không cho phép tồn tại.
    HatGiongAm {
        ten: "enum_thieu_tham_so",
        nhom: "struct-enum",
        ma_loi: "E0308",
        lo_hong: "interp.rs:~1067 (dựng BienThe gia_tri rỗng cho biến thể có tham số) + tyck.rs (Let)",
        ma: r#"
enum Hinh { Tron(i64) }

fn main() {
    let h: Hinh = Hinh::Tron;
    match h {
        _ => println!("xong"),
    }
}
"#,
    },
    // rustc: E0559 — `Tron` là biến thể dạng tuple, khởi tạo bằng cú pháp
    // struct là sai hoàn toàn. interp tra nhầm `struct_def` (chỉ chứa STRUCT)
    // nên bỏ qua mọi kiểm tra và dựng `GiaTri::Struct { ten: "Tron" }`:
    // byte-rust còn không nhận ra đây là enum.
    HatGiongAm {
        ten: "enum_tuple_khoi_tao_nhu_struct",
        nhom: "struct-enum",
        ma_loi: "E0559",
        lo_hong: "interp.rs:~858 (KhoiTaoStruct tra nhầm struct_def cho đường dẫn Enum::BienThe)",
        ma: r#"
enum Hinh { Tron(i64) }

fn main() {
    let h = Hinh::Tron { ban_kinh: 5 };
    match h {
        _ => println!("xong"),
    }
}
"#,
    },

    // ═══════════════════════════════════════════════════════════════════════
    // NHÓM 7 — PHƯƠNG THỨC DỰNG SẴN
    //
    // `tyck.rs:319` — toàn bộ thân nhánh `GoiPhuongThuc` là:
    //   `let chu = kieu_cua(doi_tuong); for a in doi_so { kieu_cua(a); }
    //    kieu_tra_ve_phuong_thuc(&chu, ten)`
    // Kiểu đối số được tính RỒI VỨT ĐI. Không có bảng chữ ký phương thức nào
    // trong crate. Đối lập hẳn với nhánh `GoiHam` vốn có kiểm `mong.get(i)` +
    // `lech_tham_chieu`. Ở tầng interp, `phuong_thuc_dung_san` mở đầu bằng hai
    // nhánh bắt-tất-cả (`clone`, `to_string`) đặt TRƯỚC mọi nhánh theo kiểu.
    // ═══════════════════════════════════════════════════════════════════════

    // rustc: E0308 — `Vec::<i64>::push(&mut self, value: i64)`.
    HatGiongAm {
        ten: "push_sai_kieu_phan_tu",
        nhom: "phuong-thuc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs:319 (GoiPhuongThuc vứt kiểu đối số) + interp.rs:1605 (Day/\"push\" nhận mọi GiaTri)",
        ma: r#"
fn main() {
    let mut so: Vec<i64> = vec![1, 2, 3];
    so.push(String::from("bon"));
    println!("{}", so.len());
}
"#,
    },
    // rustc: E0308 — Rust chỉ có `impl Add<&str> for String`. value.rs không
    // tách `String` với `&str`: cả hai là `GiaTri::Chuoi`, cần ba trạng thái
    // nhưng chỉ có một.
    HatGiongAm {
        ten: "string_cong_string",
        nhom: "phuong-thuc",
        ma_loi: "E0308",
        lo_hong: "interp.rs:1121-1123 (Cong cho (Chuoi, Chuoi)) + value.rs (gộp String và &str)",
        ma: r#"
fn main() {
    let a = String::from("xin ");
    let b = String::from("chao");
    let c = a + b;
    println!("{}", c);
}
"#,
    },
    // rustc: E0369 — `&str` hoàn toàn không có `impl Add`. Cùng gốc với ca
    // trên nhưng MÃ LỖI KHÁC và là lỗi kinh điển của người mới.
    HatGiongAm {
        ten: "str_cong_str",
        nhom: "phuong-thuc",
        ma_loi: "E0369",
        lo_hong: "interp.rs:1121-1123 + value.rs (GiaTri::Chuoi một trạng thái cho ba khái niệm)",
        ma: r#"
fn main() {
    let c = "xin " + "chao";
    println!("{}", c);
}
"#,
    },
    // rustc: E0308 — `Vec::len` trả `usize`, không phải `i64`. Ca DUY NHẤT
    // trong lô mà tyck THỰC SỰ chạy kiểm đối số mà vẫn lọt, vì độ phân giải
    // kiểu quá thô. Nói lên rằng thêm chữ ký phương thức thôi CHƯA ĐỦ.
    HatGiongAm {
        ten: "len_usize_dung_nhu_i64",
        nhom: "phuong-thuc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs:98 (gộp mọi bề rộng số nguyên) + tyck.rs:525 (\"len\" => T::SoNguyen)",
        ma: r#"
fn tong(a: i64, b: i64) -> i64 {
    a + b
}

fn main() {
    let v: Vec<i64> = vec![1, 2, 3];
    let n = tong(v.len(), 10);
    println!("{}", n);
}
"#,
    },
    // rustc: E0308 — LỖ HỔNG GỐC, đầu độc mọi kiểm tra hạ nguồn. `let` trở
    // thành máy giặt kiểu: mọi `let x: T = <sai kiểu>` đều lọt, và từ đó tyck
    // TIN một điều sai rồi lan ra các suy luận sau.
    HatGiongAm {
        ten: "vec_i64_gan_cho_vec_string",
        nhom: "phuong-thuc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs:420-424 (CauLenh::Let — kiểu khai báo ghi đè kiểu giá trị)",
        ma: r#"
fn main() {
    let ten: Vec<String> = vec![1, 2, 3];
    println!("{}", ten.len());
}
"#,
    },
    // rustc: E0308 — LỖ HỔNG KÉP TỰ NHẤT QUÁN: tyck khai `push_str` trả Chuoi,
    // interp cũng trả Chuoi (và không đột biến `s`). Vì hai tầng sai CÙNG một
    // hướng, phép kiểm kiểu trả về tự xác nhận cái sai của chính nó.
    HatGiongAm {
        ten: "push_str_tra_ve_rong",
        nhom: "phuong-thuc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs:527 (\"push_str\" -> T::Chuoi) + interp.rs:1711 (trả Chuoi thay vì Rong)",
        ma: r#"
fn noi_them(dau: String) -> String {
    let mut s = dau;
    s.push_str(" them")
}

fn main() {
    println!("{}", noi_them(String::from("goc")));
}
"#,
    },
    // rustc: E0599 — `sum` là phương thức của trait `Iterator`, `Vec` không có.
    // Tệ hơn: interp.rs:1799 liệt kê chính "sum" trong danh sách gợi ý phương
    // thức hợp lệ khi báo BR0552 — engine không chỉ bỏ sót lỗi, nó DẠY một API
    // không tồn tại. Học viên sẽ mang đúng thói quen này sang cargo thật.
    HatGiongAm {
        ten: "sum_goi_thang_tren_vec",
        nhom: "phuong-thuc",
        ma_loi: "E0599",
        lo_hong: "interp.rs:1631 (Day/\"sum\"), :1799 (danh sách gợi ý quảng cáo sai) + tyck.rs:529",
        ma: r#"
fn main() {
    let so = vec![1, 2, 3];
    let t = so.sum();
    println!("{}", t);
}
"#,
    },
    // rustc: E0308 — `fn contains(&self, x: &T) -> bool`, phải `contains(&2)`.
    // Đúng chỗ dạy về mượn mà học viên hay vấp.
    HatGiongAm {
        ten: "contains_thieu_dau_muon",
        nhom: "phuong-thuc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs:319 + interp.rs:1617 (so sánh trên GiaTri nên `&2` và `2` không phân biệt được)",
        ma: r#"
fn main() {
    let so = vec![1, 2, 3];
    if so.contains(2) {
        println!("co");
    }
}
"#,
    },
    // rustc: E0599 — `Vec` không có `push_front` (đó là `VecDeque`). Lớp VÙNG
    // MÙ MÃ CHẾT: BR0552 CÓ bắt phương thức lạ nhưng CHỈ LÚC CHẠY. Chính chế
    // độ hỏng mà chú thích ở interp.rs:485-488 đã cảnh báo — nhưng nó chỉ được
    // vá cho move, không vá cho phương thức.
    HatGiongAm {
        ten: "phuong_thuc_khong_ton_tai_trong_ham_khong_goi",
        nhom: "phuong-thuc",
        ma_loi: "E0599",
        lo_hong: "tyck.rs:536 (tên lạ -> T::Mo) + interp.rs:1795 (BR0552 chỉ kiểm động)",
        ma: r#"
fn them_dau(v: Vec<i64>) -> i64 {
    v.push_front(0)
}

fn main() {
    println!("{}", 1);
}
"#,
    },
    // rustc: E0599 — `ToString` chỉ có blanket impl cho `T: Display`.
    // `.to_string()` gọi được trên vector, struct, enum, `()`… bất kể có
    // Display hay không, vì nhánh bắt-tất-cả đứng TRƯỚC mọi nhánh theo kiểu.
    HatGiongAm {
        ten: "to_string_tren_vec",
        nhom: "phuong-thuc",
        ma_loi: "E0599",
        lo_hong: "interp.rs:1600 (nhánh bắt-tất-cả `(v, \"to_string\")`) + tyck.rs:527",
        ma: r#"
fn main() {
    let v = vec![1, 2, 3];
    let s = v.to_string();
    println!("{}", s);
}
"#,
    },
    // rustc: E0308 — `Vec::first` trả `Option<&i64>`. Ca tinh vi và đáng giữ
    // nhất về mặt chẩn đoán: hàng phòng thủ TỐT NHẤT của tyck (`lech_tham_chieu`
    // ở :302-305) bị vô hiệu bởi chính quy ước "Mo không bao giờ sinh lỗi",
    // vì luật đó loại trừ tường minh `T::Mo`.
    HatGiongAm {
        ten: "first_tra_tham_chieu",
        nhom: "phuong-thuc",
        ma_loi: "E0308",
        lo_hong: "tyck.rs:536 (\"first\"/\"unwrap\" -> T::Mo, vô hiệu lech_tham_chieu :302) + interp.rs:1610",
        ma: r#"
fn nhan_doi(n: i64) -> i64 {
    n * 2
}

fn main() {
    let v = vec![10, 20];
    println!("{}", nhan_doi(v.first().unwrap()));
}
"#,
    },
    // rustc: E0308 — `push_str(&mut self, s: &str)`, đưa integer. Ca đáng xấu
    // hổ nhất: interp ÉP NGẦM mọi giá trị thành chuỗi qua `hien_thi()`, đúng
    // thứ mà thông báo lỗi BR0531 của chính engine này tuyên bố Rust không làm.
    // Cùng chiêu dùng lại nguyên xi ở "contains", "starts_with", "split":
    // một MẪU sai lặp bốn lần, không phải một ca lẻ. byte-rust in ra "so: "
    // (KHÔNG có số 7) — ngay cả kết quả chạy cũng sai mà vẫn Đạt.
    HatGiongAm {
        ten: "push_str_ep_ngam_so_thanh_chuoi",
        nhom: "phuong-thuc",
        ma_loi: "E0308",
        lo_hong: "interp.rs:1711-1714 (ép ngầm hien_thi(); lặp ở :1697, :1701, :1706) + tyck.rs:319",
        ma: r#"
fn main() {
    let mut s = String::from("so: ");
    s.push_str(7);
    println!("{}", s);
}
"#,
    },

    // ═══════════════════════════════════════════════════════════════════════
    // NHÓM 8 — KHẢ BIẾN CỦA BINDING & PHÂN GIẢI TÊN
    //
    // Hai lỗ hổng hệ thống:
    //   (a) KHÔNG có pha kiểm khả biến nào trong engine. `Mau::Ten` mang trường
    //       `co_the_sua` (ast.rs:80) và `ten_rang_buoc` (ast.rs:119) trả nó ra,
    //       nhưng MỌI điểm tiêu thụ đều viết `for (n, _, _) in ten` — vứt thẳng.
    //   (b) KHÔNG có pha phân giải tên tĩnh. `tyck::tra` (:209-211) trả `T::Mo`
    //       cho tên lạ, và theo nguyên tắc của module, `Mo` không bao giờ sinh
    //       lỗi. Tên không tồn tại im lặng trôi qua.
    // ═══════════════════════════════════════════════════════════════════════

    // rustc: E0384 — `let` không `mut` thì không gán lần thứ hai.
    HatGiongAm {
        ten: "gan_lai_bien_khong_mut",
        nhom: "kha-bien",
        ma_loi: "E0384",
        lo_hong: "interp.rs:1225-1230 (gan_vao không hỏi binding có mut); không pha tĩnh nào kiểm",
        ma: r#"
fn main() {
    let diem = 10;
    diem = 20;
    println!("diem = {}", diem);
}
"#,
    },
    // rustc: E0384 — tham số hàm cũng là một binding.
    HatGiongAm {
        ten: "gan_lai_tham_so_ham",
        nhom: "kha-bien",
        ma_loi: "E0384",
        lo_hong: "interp.rs (rang_buoc_mau bỏ qua co_the_sua) + tyck.rs:151-163 (ChuKy không lưu khả biến)",
        ma: r#"
fn tang(n: i64) -> i64 {
    n = n + 1;
    n
}

fn main() {
    println!("{}", tang(4));
}
"#,
    },
    // rustc: E0384 — biến lặp `for` là binding bất biến trừ khi viết `for mut i`.
    HatGiongAm {
        ten: "gan_lai_bien_lap_for",
        nhom: "kha-bien",
        ma_loi: "E0384",
        lo_hong: "tyck.rs:376-384 (BieuThuc::Cho: `for (t, _, _) in ten` vứt cờ co_the_sua)",
        ma: r#"
fn main() {
    for i in 0..3 {
        i = i + 10;
        println!("{}", i);
    }
}
"#,
    },
    // rustc: E0384 — `let n = n + 1` tạo binding MỚI che binding cũ, và binding
    // mới KHÔNG có `mut`. Đúng cái bẫy shadowing người học hay dính: tưởng
    // `mut` còn hiệu lực.
    HatGiongAm {
        ten: "shadow_mat_mut",
        nhom: "kha-bien",
        ma_loi: "E0384",
        lo_hong: "tyck.rs:420-426 (Let ghi đè tên không mang trạng thái mut) + interp.rs (dat_bien)",
        ma: r#"
fn main() {
    let mut n = 1;
    n = 2;
    let n = n + 1;
    n = 10;
    println!("{}", n);
}
"#,
    },
    // rustc: E0596 — không mượn được `&mut` từ binding bất biến. interp dựng
    // `co_the_sua` từ CÚ PHÁP `&mut` chứ không kiểm biến gốc có `mut`.
    HatGiongAm {
        ten: "muon_mut_tu_bien_khong_mut",
        nhom: "kha-bien",
        ma_loi: "E0596",
        lo_hong: "interp.rs:574-580 (BieuThuc::Muon lấy co_the_sua từ cú pháp) + tyck.rs (T::Tham mù)",
        ma: r#"
fn tang(x: &mut i64) {
    *x = *x + 1;
}

fn main() {
    let n = 5;
    tang(&mut n);
    println!("{}", n);
}
"#,
    },
    // rustc: E0596 — `Vec::push` nhận `&mut self`. move_check CỐ Ý coi mọi
    // phương thức là mượn để tránh báo oan, nhưng không phân biệt mượn thường
    // với mượn khả biến.
    HatGiongAm {
        ten: "push_vec_khong_mut",
        nhom: "kha-bien",
        ma_loi: "E0596",
        lo_hong: "move_check.rs:291-296 (GoiPhuongThuc coi mọi receiver là mượn thường)",
        ma: r#"
fn main() {
    let v = vec![1, 2];
    v.push(3);
    println!("{}", v.len());
}
"#,
    },
    // rustc: E0594 — khả biến ở Rust là thuộc tính của BINDING, lan xuống mọi
    // trường. Không có pha tĩnh nào truy ngược từ place `d.x` về gốc `d`.
    HatGiongAm {
        ten: "sua_truong_struct_khong_mut",
        nhom: "kha-bien",
        ma_loi: "E0594",
        lo_hong: "interp.rs:1232-1238 (gan_vao/TruyCapTruong: borrow_mut().insert không kiểm gì)",
        ma: r#"
struct Diem {
    x: i64,
    y: i64,
}

fn main() {
    let d = Diem { x: 1, y: 2 };
    d.x = 10;
    println!("{} {}", d.x, d.y);
}
"#,
    },
    // rustc: E0070 — `const` không phải nơi lưu giá trị, nó bị thay thế tại
    // chỗ. interp biến const thành BIẾN THƯỜNG ở phạm vi toàn cục; tyck::nap
    // bỏ qua hoàn toàn `Muc::Const`.
    HatGiongAm {
        ten: "gan_lai_hang_const",
        nhom: "kha-bien",
        ma_loi: "E0070",
        lo_hong: "interp.rs:218-222 (Muc::Const gọi dat_bien) + tyck.rs:151-172 (bỏ qua Muc::Const)",
        ma: r#"
const GIOI_HAN: i64 = 10;

fn main() {
    GIOI_HAN = 5;
    println!("{}", GIOI_HAN);
}
"#,
    },
    // rustc: E0425 — `bi_mat` chết khi khối `{ }` đóng lại. rustc phân giải
    // tên TĨNH nên nhánh có chạy hay không hoàn toàn không liên quan.
    HatGiongAm {
        ten: "dung_bien_ngoai_pham_vi_khoi",
        nhom: "kha-bien",
        ma_loi: "E0425",
        lo_hong: "tyck.rs:209-211 (tra() trả T::Mo cho tên lạ; không có pha phân giải tên tĩnh)",
        ma: r#"
fn main() {
    {
        let bi_mat = 42;
        println!("trong khoi: {}", bi_mat);
    }
    if false {
        println!("ngoai khoi: {}", bi_mat);
    }
    println!("xong");
}
"#,
    },
    // rustc: E0425 — nguy hiểm hơn về sư phạm: điều kiện là biến runtime chứ
    // không phải hằng `false` lộ liễu, nên nhánh sai KHÔNG BAO GIỜ chạy trong
    // khi trông vẫn "sống". Interpreter động không thể bắt lớp lỗi này về
    // nguyên tắc — đúng lập luận move_check.rs viết ở đầu file cho move, nhưng
    // chưa áp dụng cho phân giải tên.
    HatGiongAm {
        ten: "dung_bien_khai_bao_trong_if",
        nhom: "kha-bien",
        ma_loi: "E0425",
        lo_hong: "tyck.rs:209-211 (tra() -> T::Mo) + interp.rs (BR0501 chỉ nổ khi dòng đó THỰC SỰ chạy)",
        ma: r#"
fn main() {
    let bat = true;
    if bat {
        let trong_if = 1;
        println!("{}", trong_if);
    }
    if !bat {
        println!("{}", trong_if);
    }
}
"#,
    },
    // rustc: E0425 — trong thân hàm, `let` KHÔNG được cẩu lên (no hoisting).
    // Ca RẺ NHẤT để vá: `khoi` duyệt câu lệnh TUẦN TỰ nên đã biết `sau_nay`
    // chưa được `dat`; chỉ cần một cờ "tên chưa từng thấy ở bất kỳ phạm vi nào
    // đang mở", không cần CFG, không đụng lập luận NLL của ADR-002 §2.
    HatGiongAm {
        ten: "dung_truoc_khi_khai_bao",
        nhom: "kha-bien",
        ma_loi: "E0425",
        lo_hong: "tyck.rs:209-211 (khoi() ĐÃ biết tuần tự nhưng tra() vứt thông tin đó)",
        ma: r#"
fn main() {
    let chay = false;
    if chay {
        println!("{}", sau_nay);
    }
    let sau_nay = 7;
    println!("{}", sau_nay);
}
"#,
    },
    // rustc: E0308 — shadowing hợp lệ và ĐỔI KIỂU của `ma` từ `{integer}` sang
    // `&str`. `T::Chuoi` vs `T::SoNguyen` đã đủ để `chac_chan_lech` trả true,
    // tức vá được bằng ~5 dòng.
    HatGiongAm {
        ten: "shadow_doi_kieu_roi_gan_sai",
        nhom: "kha-bien",
        ma_loi: "E0308",
        lo_hong: "tyck.rs:420-426 (CauLenh::Let: kiểu suy ra t_gt bị VỨT THẲNG khi có chú thích)",
        ma: r#"
fn main() {
    let ma = 5;
    let ma = "nam";
    let so: i64 = ma;
    println!("{}", so);
}
"#,
    },
];
