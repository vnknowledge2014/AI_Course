use byte_rust::ast::*;
use byte_rust::parser::phan_tich;
use byte_rust::span::SourceMap;

fn phai_sach(src: &str) -> ChuongTrinh {
    let (ct, d) = phan_tich(src);
    assert!(
        !d.co_loi(),
        "không mong đợi lỗi cho:\n{src}\n---\n{}",
        d.render(&SourceMap::new(src))
    );
    ct
}

fn ma_loi(src: &str) -> String {
    let (_ct, d) = phan_tich(src);
    assert!(d.co_loi(), "mong đợi có lỗi cho:\n{src}");
    let code = d
        .iter()
        .find(|x| x.severity == byte_rust::Severity::Loi)
        .unwrap()
        .code
        .to_string();
    code
}

// ── Phân tích đúng ──────────────────────────────────────────────────────────

#[test]
fn ham_main_rong() {
    let ct = phai_sach("fn main() {}");
    assert_eq!(ct.muc.len(), 1);
    match &ct.muc[0] {
        Muc::Ham(h) => {
            assert_eq!(h.ten, "main");
            assert!(h.tham_so.is_empty());
            assert!(h.kieu_tra_ve.is_none());
        }
        khac => panic!("mong đợi hàm, gặp {khac:?}"),
    }
}

#[test]
fn let_co_kieu_va_gia_tri() {
    let ct = phai_sach("fn main() { let x: i64 = 42; }");
    let Muc::Ham(h) = &ct.muc[0] else { panic!() };
    let CauLenh::Let { mau, kieu, gia_tri, .. } = &h.than.cau_lenh[0] else { panic!() };
    assert!(matches!(mau, Mau::Ten { ten, co_the_sua: false, .. } if ten == "x"));
    assert_eq!(kieu.as_ref().unwrap().hien_thi(), "i64");
    assert!(matches!(gia_tri, Some(BieuThuc::HangSo { gia_tri: HangSo::SoNguyen(42), .. })));
}

#[test]
fn uu_tien_toan_tu_dung() {
    // 1 + 2 * 3 phải là 1 + (2 * 3)
    let ct = phai_sach("fn main() { let x = 1 + 2 * 3; }");
    let Muc::Ham(h) = &ct.muc[0] else { panic!() };
    let CauLenh::Let { gia_tri: Some(bt), .. } = &h.than.cau_lenh[0] else { panic!() };
    let BieuThuc::HaiNgoi { toan_tu, phai, .. } = bt else { panic!("mong đợi phép hai ngôi") };
    assert_eq!(*toan_tu, ToanTuHai::Cong, "phép cộng phải ở ngoài cùng");
    assert!(matches!(**phai, BieuThuc::HaiNgoi { toan_tu: ToanTuHai::Nhan, .. }));
}

#[test]
fn so_sanh_long_hon_logic() {
    // a == b && c  phải là (a == b) && c
    let ct = phai_sach("fn main() { let x = a == b && c; }");
    let Muc::Ham(h) = &ct.muc[0] else { panic!() };
    let CauLenh::Let { gia_tri: Some(bt), .. } = &h.than.cau_lenh[0] else { panic!() };
    let BieuThuc::HaiNgoi { toan_tu, trai, .. } = bt else { panic!() };
    assert_eq!(*toan_tu, ToanTuHai::Va);
    assert!(matches!(**trai, BieuThuc::HaiNgoi { toan_tu: ToanTuHai::Bang, .. }));
}

#[test]
fn if_khong_bi_nham_voi_khoi_tao_struct() {
    // `if x { }` — `x { }` KHÔNG được hiểu là struct literal
    // `if` đứng cuối khối chính là giá trị trả về của khối — đúng ngữ nghĩa Rust.
    let ct = phai_sach("fn main() { if x { let a = 1; } }");
    let Muc::Ham(h) = &ct.muc[0] else { panic!() };
    assert!(matches!(h.than.gia_tri_cuoi, Some(BieuThuc::Neu { .. })));

    // Còn khi có câu lệnh đứng sau thì nó là câu lệnh.
    let ct = phai_sach("fn main() { if x { } let a = 1; }");
    let Muc::Ham(h) = &ct.muc[0] else { panic!() };
    let CauLenh::BieuThuc { bt, .. } = &h.than.cau_lenh[0] else { panic!() };
    assert!(matches!(bt, BieuThuc::Neu { .. }));
}

#[test]
fn khoi_tao_struct_hoat_dong_ngoai_dieu_kien() {
    let ct = phai_sach("fn main() { let p = Point { x: 1, y: 2 }; }");
    let Muc::Ham(h) = &ct.muc[0] else { panic!() };
    let CauLenh::Let { gia_tri: Some(BieuThuc::KhoiTaoStruct { duong_dan, truong, .. }), .. } =
        &h.than.cau_lenh[0] else { panic!("mong đợi khởi tạo struct") };
    assert_eq!(duong_dan, &vec!["Point".to_string()]);
    assert_eq!(truong.len(), 2);
}

#[test]
fn struct_enum_impl_trait() {
    let ct = phai_sach(
        r#"
struct Point { x: i64, y: i64 }
enum Color { Red, Green, Custom(u8, u8, u8) }
trait Ve { fn ve(&self) -> String; }
impl Ve for Point { fn ve(&self) -> String { String::new() } }
"#,
    );
    assert_eq!(ct.muc.len(), 4);
    assert!(matches!(ct.muc[0], Muc::Struct(_)));
    assert!(matches!(ct.muc[1], Muc::Enum(_)));
    assert!(matches!(ct.muc[2], Muc::Trait(_)));
    let Muc::Impl(i) = &ct.muc[3] else { panic!() };
    assert_eq!(i.trait_ten.as_ref().unwrap(), &vec!["Ve".to_string()]);
    assert_eq!(i.ham.len(), 1);
}

#[test]
fn match_voi_nhieu_dang_mau() {
    let ct = phai_sach(
        r#"
fn main() {
    match x {
        0 => "khong",
        1..=9 => "mot chu so",
        Some(n) if n > 100 => "lon",
        Point { x, .. } => "diem",
        _ => "khac",
    };
}
"#,
    );
    let Muc::Ham(h) = &ct.muc[0] else { panic!() };
    let CauLenh::BieuThuc { bt, .. } = &h.than.cau_lenh[0] else { panic!() };
    let BieuThuc::KhopMau { nhanh, .. } = bt else { panic!() };
    assert_eq!(nhanh.len(), 5);
    assert!(nhanh[2].dieu_kien.is_some(), "nhánh có `if` phải giữ điều kiện");
}

#[test]
fn vong_lap_va_dai() {
    phai_sach("fn main() { for i in 0..10 { println!(\"{}\", i); } }");
    phai_sach("fn main() { while x < 10 { x += 1; } }");
    phai_sach("fn main() { loop { break; } }");
}

#[test]
fn closure_va_goi_phuong_thuc_noi_chuoi() {
    let ct = phai_sach("fn main() { let s: i64 = v.iter().map(|x| x + 1).sum(); }");
    let Muc::Ham(h) = &ct.muc[0] else { panic!() };
    let CauLenh::Let { gia_tri: Some(bt), .. } = &h.than.cau_lenh[0] else { panic!() };
    assert!(matches!(bt, BieuThuc::GoiPhuongThuc { ten, .. } if ten == "sum"));
}

#[test]
fn muon_va_giai_tham_chieu() {
    let ct = phai_sach("fn main() { let a = &x; let b = &mut y; let c = *a; }");
    let Muc::Ham(h) = &ct.muc[0] else { panic!() };
    let CauLenh::Let { gia_tri: Some(BieuThuc::Muon { co_the_sua: false, .. }), .. } = &h.than.cau_lenh[0] else { panic!() };
    let CauLenh::Let { gia_tri: Some(BieuThuc::Muon { co_the_sua: true, .. }), .. } = &h.than.cau_lenh[1] else { panic!() };
    let CauLenh::Let { gia_tri: Some(BieuThuc::GiaiTham { .. }), .. } = &h.than.cau_lenh[2] else { panic!() };
}

#[test]
fn gia_tri_cuoi_khoi_khong_can_cham_phay() {
    let ct = phai_sach("fn cong(a: i64, b: i64) -> i64 { a + b }");
    let Muc::Ham(h) = &ct.muc[0] else { panic!() };
    assert!(h.than.cau_lenh.is_empty());
    assert!(h.than.gia_tri_cuoi.is_some(), "biểu thức cuối phải là giá trị trả về");
}

#[test]
fn toan_tu_hoi_va_ep_kieu() {
    phai_sach("fn f() -> Result<i64, String> { let x = g()?; Ok(x as i64) }");
}

#[test]
fn macro_println_voi_nhieu_doi_so() {
    let ct = phai_sach(r#"fn main() { println!("{} {}", a, b); }"#);
    let Muc::Ham(h) = &ct.muc[0] else { panic!() };
    let CauLenh::BieuThuc { bt: BieuThuc::Macro { ten, doi_so, .. }, .. } = &h.than.cau_lenh[0] else { panic!() };
    assert_eq!(ten, "println");
    assert_eq!(doi_so.len(), 3);
}

// ── Chẩn đoán ───────────────────────────────────────────────────────────────

#[test]
fn thieu_cham_phay() {
    assert_eq!(ma_loi("fn main() { let x = 1 let y = 2; }"), "BR0109");
}

#[test]
fn thieu_ngoac_nhon_sau_if() {
    assert_eq!(ma_loi("fn main() { if x let y = 1; }"), "BR0114");
}

#[test]
fn thieu_mui_ten_dam_trong_match() {
    assert_eq!(ma_loi("fn main() { match x { 1 -> 2, } }"), "BR0115");
}

#[test]
fn thieu_in_trong_for() {
    assert_eq!(ma_loi("fn main() { for i 0..10 {} }"), "BR0112");
}

#[test]
fn let_o_ngoai_ham() {
    assert_eq!(ma_loi("let x = 1;"), "BR0103");
}

#[test]
fn tu_khoa_lam_ten_bien() {
    assert_eq!(ma_loi("fn main() { let fn = 1; }"), "BR0102");
}

#[test]
fn thieu_ngoac_dong() {
    assert_eq!(ma_loi("fn main() { let x = (1 + 2; }"), "BR0100");
}

#[test]
fn goi_y_dat_code_vao_main() {
    let (_ct, d) = phan_tich("let x = 1;");
    let sm = SourceMap::new("let x = 1;");
    let s = d.render(&sm);
    assert!(s.contains("`let` chỉ dùng được bên trong hàm"), "gợi ý chưa đúng ngữ cảnh:\n{s}");
}

#[test]
fn phuc_hoi_loi_van_tim_duoc_muc_sau() {
    // Lỗi ở hàm đầu không được nuốt mất hàm thứ hai.
    let (ct, d) = phan_tich("fn a() { let x = ; }\nfn b() {}");
    assert!(d.co_loi());
    let ten: Vec<_> = ct.muc.iter().filter_map(|m| m.ten()).collect();
    assert!(ten.contains(&"b"), "parser phải phục hồi và tìm được `fn b`, chỉ thấy {ten:?}");
}

#[test]
fn khong_panic_voi_dau_vao_ky_quai() {
    for src in ["", "fn", "fn(", "fn a(", "{{{{", "}}}}", "match", "if if if",
                "let let let", "impl", "trait", "struct S {", "fn a() -> {"] {
        let _ = phan_tich(src);
    }
}
