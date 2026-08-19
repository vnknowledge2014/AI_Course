use byte_rust::interp::chay;
use byte_rust::span::SourceMap;

/// Chạy chương trình, khẳng định không lỗi, trả về phần in ra.
fn xuat(src: &str) -> String {
    let (out, d) = chay(src);
    assert!(
        !d.co_loi(),
        "không mong đợi lỗi:\n{src}\n---\n{}",
        d.render(&SourceMap::new(src))
    );
    out
}

/// Chạy chương trình, khẳng định CÓ lỗi, trả về mã lỗi.
fn loi(src: &str) -> String {
    let (_out, d) = chay(src);
    assert!(d.co_loi(), "mong đợi có lỗi:\n{src}");
    let code = d
        .iter()
        .find(|x| x.severity == byte_rust::Severity::Loi)
        .unwrap()
        .code
        .to_string();
    code
}

fn ct(than: &str) -> String {
    format!("fn main() {{\n{than}\n}}")
}

// ── Chạy đúng ───────────────────────────────────────────────────────────────

#[test]
fn in_ra_chuoi() {
    assert_eq!(xuat(&ct(r#"println!("Xin chào, Byte!");"#)), "Xin chào, Byte!\n");
}

#[test]
fn so_hoc_va_uu_tien() {
    assert_eq!(xuat(&ct(r#"println!("{}", 1 + 2 * 3);"#)), "7\n");
    assert_eq!(xuat(&ct(r#"println!("{}", (1 + 2) * 3);"#)), "9\n");
    assert_eq!(xuat(&ct(r#"println!("{}", 7 / 2);"#)), "3\n");
    assert_eq!(xuat(&ct(r#"println!("{}", 7 % 2);"#)), "1\n");
    assert_eq!(xuat(&ct(r#"println!("{}", 7.0 / 2.0);"#)), "3.5\n");
}

#[test]
fn bien_va_thay_doi() {
    assert_eq!(
        xuat(&ct("let mut x = 1;\nx += 4;\nprintln!(\"{}\", x);")),
        "5\n"
    );
}

#[test]
fn ham_co_tra_ve() {
    assert_eq!(
        xuat("fn cong(a: i64, b: i64) -> i64 { a + b }\nfn main() { println!(\"{}\", cong(2, 3)); }"),
        "5\n"
    );
}

#[test]
fn de_quy_giai_thua() {
    assert_eq!(
        xuat(
            r#"
fn gt(n: i64) -> i64 { if n <= 1 { 1 } else { n * gt(n - 1) } }
fn main() { println!("{}", gt(10)); }
"#
        ),
        "3628800\n"
    );
}

#[test]
fn vong_lap_for_va_dai() {
    assert_eq!(
        xuat(&ct("let mut t = 0;\nfor i in 1..=5 { t += i; }\nprintln!(\"{}\", t);")),
        "15\n"
    );
}

#[test]
fn vong_lap_while_va_break() {
    assert_eq!(
        xuat(&ct("let mut i = 0;\nwhile true { i += 1; if i == 3 { break; } }\nprintln!(\"{}\", i);")),
        "3\n"
    );
}

#[test]
fn match_tren_so_va_dai() {
    assert_eq!(
        xuat(&ct(r#"
let x = 7;
let s = match x { 0 => "khong", 1..=5 => "nho", _ => "lon" };
println!("{}", s);"#)),
        "lon\n"
    );
}

#[test]
fn struct_va_phuong_thuc() {
    assert_eq!(
        xuat(r#"
struct Diem { x: i64, y: i64 }
impl Diem {
    fn moi(x: i64, y: i64) -> Diem { Diem { x: x, y: y } }
    fn tong(&self) -> i64 { self.x + self.y }
}
fn main() {
    let d = Diem::moi(3, 4);
    println!("{}", d.tong());
}
"#),
        "7\n"
    );
}

#[test]
fn enum_va_khop_mau() {
    assert_eq!(
        xuat(r#"
enum Mau { Do, Xanh, Tuy(i64) }
fn ten(m: Mau) -> String {
    match m {
        Mau::Do => String::from("do"),
        Mau::Xanh => String::from("xanh"),
        Mau::Tuy(n) => format!("tuy {}", n),
    }
}
fn main() { println!("{}", ten(Mau::Tuy(42))); }
"#),
        "tuy 42\n"
    );
}

#[test]
fn option_va_result() {
    assert_eq!(xuat(&ct(r#"
let a: Option<i64> = Some(5);
println!("{}", a.unwrap_or(0));
let b: Option<i64> = None;
println!("{}", b.unwrap_or(99));"#)), "5\n99\n");
}

#[test]
fn vec_va_phuong_thuc() {
    assert_eq!(
        xuat(&ct(r#"
let mut v = vec![3, 1, 2];
v.push(4);
v.sort();
println!("{:?}", v);
println!("{}", v.len());"#)),
        "[1, 2, 3, 4]\n4\n"
    );
}

#[test]
fn closure_va_iterator() {
    assert_eq!(
        xuat(&ct(r#"
let v = vec![1, 2, 3, 4];
// `iter()` mượn phần tử nên closure của `filter` nhận `&i64` — phải `*x`.
// Bản trước viết `x > 4` và rustc TỪ CHỐI (E0308); test cũ đang dạy Rust sai.
let t: i64 = v.iter().map(|x| x * 2).filter(|x| *x > 4).sum();
println!("{}", t);"#)),
        "14\n" // 6 + 8
    );
}

#[test]
fn go_ro_khac_hien_thi() {
    assert_eq!(xuat(&ct(r#"println!("{:?}", "abc");"#)), "\"abc\"\n");
    assert_eq!(xuat(&ct(r#"println!("{}", "abc");"#)), "abc\n");
}

#[test]
fn toan_tu_hoi_lan_truyen_loi() {
    assert_eq!(
        xuat(r#"
fn chia(a: i64, b: i64) -> Result<i64, String> {
    if b == 0 { return Err(String::from("chia 0")); }
    Ok(a / b)
}
fn tinh() -> Result<i64, String> {
    let x = chia(10, 2)?;
    let y = chia(x, 0)?;
    Ok(y)
}
fn main() {
    match tinh() {
        Ok(v) => println!("ok {}", v),
        Err(e) => println!("loi: {}", e),
    }
}
"#),
        "loi: chia 0\n"
    );
}

// ── Lỗi lúc chạy phải dạy được ──────────────────────────────────────────────

#[test]
fn chia_cho_khong() {
    assert_eq!(loi(&ct("let x = 1 / 0;")), "BR0532");
}

#[test]
fn vuot_chi_so() {
    assert_eq!(loi(&ct("let v = vec![1, 2];\nprintln!(\"{}\", v[5]);")), "BR0509");
}

#[test]
fn unwrap_tren_none() {
    assert_eq!(loi(&ct("let a: Option<i64> = None;\nlet b = a.unwrap();")), "BR0551");
}

#[test]
fn bien_chua_khai_bao() {
    assert_eq!(loi(&ct("println!(\"{}\", chua_co);")), "BR0501");
}

#[test]
fn tran_so_nguyen() {
    assert_eq!(loi(&ct("let x = 9223372036854775807 + 1;")), "BR0533");
}

#[test]
fn vong_lap_vo_han_bi_chan() {
    // Đây là điểm sống còn: KHÔNG được treo.
    assert_eq!(loi(&ct("let mut i = 0;\nwhile true { i += 1; }")), "BR0500");
}

#[test]
fn de_quy_vo_han_bi_chan() {
    assert!(matches!(
        loi("fn f(n: i64) -> i64 { f(n + 1) }\nfn main() { f(0); }").as_str(),
        "BR0505" | "BR0500"
    ));
}

#[test]
fn cong_chuoi_voi_so_bao_loi_ro_rang() {
    // Nay tyck bắt TĨNH (BR0305) trước khi interp kịp chạy (BR0531). Bắt tĩnh
    // tốt hơn: đúng thời điểm rustc bắt, và người học không thấy nửa chương
    // trình chạy rồi mới lỗi.
    assert_eq!(loi(&ct(r#"let s = String::from("a") + 1;"#)), "BR0305");
}

#[test]
fn dieu_kien_khong_phai_bool() {
    assert_eq!(loi(&ct("if 1 { }")), "BR0520");
}

#[test]
fn dung_bien_sau_khi_da_chuyen_quyen_so_huu() {
    // Bài học trung tâm của Rust.
    let src = r#"
fn nhan(s: String) -> i64 { s.len() as i64 }
fn main() {
    let a = String::from("chao");
    nhan(a);
    println!("{}", a);
}
"#;
    assert_eq!(loi(src), "BR0530");
}

#[test]
fn clone_thi_khong_bi_chuyen() {
    assert_eq!(
        xuat(r#"
fn nhan(s: String) -> i64 { s.len() as i64 }
fn main() {
    let a = String::from("chao");
    nhan(a.clone());
    println!("{}", a);
}
"#),
        "chao\n"
    );
}

#[test]
fn thieu_ham_main() {
    assert_eq!(loi("fn khac() {}"), "BR0502");
}

#[test]
fn goi_ham_sai_ten_duoc_goi_y() {
    let src = "fn tinh_tong() -> i64 { 1 }\nfn main() { tinh_tonng(); }";
    let (_o, d) = chay(src);
    let s = d.render(&SourceMap::new(src));
    assert!(s.contains("tinh_tong"), "phải gợi ý tên đúng:\n{s}");
}

#[test]
fn khong_panic_voi_dau_vao_ky_quai() {
    for src in ["", "fn main(){}", "fn main(){ let x = ; }", "fn main(){ }}}", "fn main(){ x.y.z(); }"] {
        let _ = chay(src);
    }
}

// ── Quyền sở hữu: phải nhất quán ở MỌI vị trí chuyển giá trị ────────────────

#[test]
fn move_khi_gan_sang_bien_khac() {
    let src = r#"
fn main() {
    let ten = String::from("Byte");
    let khac = ten;
    println!("{}", ten);
}"#;
    assert_eq!(loi(src), "BR0530");
}

#[test]
fn move_khi_dua_vao_truong_struct() {
    let src = r#"
struct Nguoi { ten: String }
fn main() {
    let t = String::from("Byte");
    let n = Nguoi { ten: t };
    println!("{}", t);
}"#;
    assert_eq!(loi(src), "BR0530");
}

#[test]
fn kieu_copy_khong_bi_move() {
    // Số là Copy — gán xong vẫn dùng được biến cũ.
    assert_eq!(
        xuat(&ct("let a = 5;\nlet b = a;\nprintln!(\"{} {}\", a, b);")),
        "5 5\n"
    );
}

#[test]
fn muon_thay_vi_move_thi_khong_sao() {
    assert_eq!(
        xuat(r#"
fn dai(s: &String) -> i64 { s.len() as i64 }
fn main() {
    let t = String::from("chao");
    dai(&t);
    println!("{}", t);
}"#),
        "chao\n"
    );
}

#[test]
fn cong_don_khong_lam_move() {
    assert_eq!(
        xuat(&ct("let mut s = String::from(\"a\");\nlet mut n = 1;\nn += 1;\nprintln!(\"{} {}\", s, n);")),
        "a 2\n"
    );
}

// ── Ba kết cục: Đạt / Không đạt / Chưa hỗ trợ ───────────────────────────────
//
// Đây là bất biến quan trọng nhất của cả engine. Xem docs/decisions/ADR-002.

use byte_rust::diag::KetCuc;

fn ket_cuc(src: &str) -> KetCuc {
    let (ct, d) = byte_rust::parser::phan_tich(src);
    if d.co_loi() {
        return d.ket_cuc();
    }
    let mut d = d;
    byte_rust::move_check::kiem_tra(&ct, &mut d);
    if d.co_loi() || d.co_chua_ho_tro() {
        return d.ket_cuc();
    }
    let mut may = byte_rust::interp::MayChay::moi();
    if let Err(loi) = may.chay(&ct) {
        d.push(loi);
    }
    d.ket_cuc()
}

#[test]
fn code_dung_cho_ket_cuc_dat() {
    assert_eq!(ket_cuc(r#"fn main() { println!("{}", 1 + 1); }"#), KetCuc::Dat);
}

#[test]
fn code_sai_cho_ket_cuc_khong_dat() {
    assert_eq!(ket_cuc("fn main() { let x = 1 / 0; }"), KetCuc::KhongDat);
}

#[test]
fn move_thang_hang_van_la_loi_that() {
    // Trên chuỗi câu lệnh thẳng hàng, rustc CHẮC CHẮN từ chối -> ta được phép khẳng định.
    let src = r#"
fn main() {
    let a = String::from("x");
    let b = a;
    println!("{}", a);
}"#;
    assert_eq!(ket_cuc(src), KetCuc::KhongDat);
    assert_eq!(loi(src), "BR0530");
}

#[test]
fn move_trong_nhanh_cho_chua_ho_tro_chu_khong_doan_bua() {
    // Ta chỉ thấy nhánh ĐÃ chạy; rustc xét MỌI nhánh. Không đủ cơ sở phán quyết.
    let src = r#"
fn main() {
    let a = String::from("x");
    if true {
        let b = a;
    }
    println!("{}", a);
}"#;
    assert_eq!(
        ket_cuc(src),
        KetCuc::ChuaHoTro,
        "move trong nhánh KHÔNG được khẳng định đúng hay sai"
    );
}

#[test]
fn move_trong_nhanh_khong_chay_cung_cho_chua_ho_tro() {
    // Nhánh KHÔNG chạy: engine dynamic sẽ thấy `a` còn nguyên và báo Đạt —
    // nhưng rustc vẫn từ chối. Đây đúng là chế độ false-accept nguy hiểm.
    let src = r#"
fn main() {
    let a = String::from("x");
    if false {
        let b = a;
    }
    println!("{}", a);
}"#;
    let kq = ket_cuc(src);
    assert_ne!(kq, KetCuc::Dat, "KHÔNG được báo Đạt cho code rustc từ chối");
}

#[test]
fn move_trong_vong_lap_cho_chua_ho_tro() {
    let src = r#"
fn main() {
    let a = String::from("x");
    for i in 0..1 {
        let b = a;
    }
}"#;
    assert_eq!(ket_cuc(src), KetCuc::ChuaHoTro);
}

#[test]
fn macro_ngoai_pham_vi_la_chua_ho_tro_khong_phai_loi() {
    let src = r#"fn main() { write!("x"); }"#;
    assert_eq!(ket_cuc(src), KetCuc::ChuaHoTro);
    let (_o, d) = chay(src);
    let cd = d.iter().next().unwrap();
    assert_eq!(cd.severity, byte_rust::Severity::ChuaHoTro);
    assert_eq!(cd.tinh_nang, Some("macro"));
}

#[test]
fn chua_ho_tro_thang_loi_khi_ca_hai_cung_xuat_hien() {
    // Nếu không hiểu hết chương trình thì không có tư cách khẳng định nó sai.
    let mut d = byte_rust::Diagnostics::new();
    d.push(byte_rust::Diagnostic::loi("BR9999", "một lỗi"));
    d.push(byte_rust::Diagnostic::chua_ho_tro("BR9998", "async", "chưa hỗ trợ"));
    assert_eq!(d.ket_cuc(), KetCuc::ChuaHoTro);
}

#[test]
fn rut_gon_khong_lam_mat_chan_doan_chua_ho_tro() {
    let mut d = byte_rust::Diagnostics::new();
    d.push(byte_rust::Diagnostic::loi("BR9999", "lỗi hệ quả"));
    d.push(byte_rust::Diagnostic::chua_ho_tro("BR9998", "async", "chưa hỗ trợ"));
    let d = d.rut_gon();
    assert_eq!(d.ket_cuc(), KetCuc::ChuaHoTro);
    assert!(d.iter().any(|x| x.tinh_nang == Some("async")));
}
