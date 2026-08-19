use byte_rust::lexer::{quet, TokKind, TuKhoa};
use byte_rust::span::SourceMap;

fn kinds(src: &str) -> Vec<TokKind> {
    let (toks, d) = quet(src);
    assert!(!d.co_loi(), "không mong đợi lỗi cho {src:?}:\n{}", d.render(&SourceMap::new(src)));
    toks.into_iter().map(|t| t.kind).filter(|k| *k != TokKind::HetTep).collect()
}

fn loi_dau(src: &str) -> String {
    let (_t, d) = quet(src);
    assert!(d.co_loi(), "mong đợi có lỗi cho {src:?}");
    let code = d
        .iter()
        .find(|x| x.severity == byte_rust::Severity::Loi)
        .unwrap()
        .code
        .to_string();
    code
}

#[test]
fn so_nguyen_va_dau_gach_duoi_ngan_cach() {
    assert_eq!(kinds("1_000_000"), vec![TokKind::SoNguyen(1_000_000)]);
    assert_eq!(kinds("0"), vec![TokKind::SoNguyen(0)]);
}

#[test]
fn co_so_khac_muoi() {
    assert_eq!(kinds("0xFF"), vec![TokKind::SoNguyen(255)]);
    assert_eq!(kinds("0b1010"), vec![TokKind::SoNguyen(10)]);
    assert_eq!(kinds("0o17"), vec![TokKind::SoNguyen(15)]);
}

#[test]
fn so_thuc_va_hau_to_kieu() {
    assert_eq!(kinds("3.5"), vec![TokKind::SoThuc(3.5)]);
    assert_eq!(kinds("10i32"), vec![TokKind::SoNguyen(10)]);
    assert_eq!(kinds("2.5f64"), vec![TokKind::SoThuc(2.5)]);
}

#[test]
fn dai_khong_bi_nham_thanh_so_thuc() {
    // `1..5` phải ra ba token, không phải số thực `1.` rồi `.5`.
    assert_eq!(
        kinds("1..5"),
        vec![TokKind::SoNguyen(1), TokKind::HaiCham, TokKind::SoNguyen(5)]
    );
    assert_eq!(
        kinds("1..=5"),
        vec![TokKind::SoNguyen(1), TokKind::HaiChamBang, TokKind::SoNguyen(5)]
    );
}

#[test]
fn tu_khoa_khac_ten_thuong() {
    assert_eq!(kinds("let"), vec![TokKind::TuKhoa(TuKhoa::Let)]);
    assert_eq!(kinds("lets"), vec![TokKind::Ten("lets".into())]);
    assert_eq!(kinds("true"), vec![TokKind::DungSai(true)]);
}

#[test]
fn gach_duoi_don_la_token_rieng() {
    assert_eq!(kinds("_"), vec![TokKind::GachDuoi]);
    assert_eq!(kinds("_x"), vec![TokKind::Ten("_x".into())]);
}

#[test]
fn chuoi_va_ky_tu_thoat() {
    assert_eq!(kinds(r#""xin chào""#), vec![TokKind::ChuoiVanBan("xin chào".into())]);
    assert_eq!(kinds(r#""dòng\nmới""#), vec![TokKind::ChuoiVanBan("dòng\nmới".into())]);
    assert_eq!(kinds("'a'"), vec![TokKind::KyTu('a')]);
    assert_eq!(kinds(r"'\n'"), vec![TokKind::KyTu('\n')]);
}

#[test]
fn chu_thich_bi_bo_qua_ke_ca_long_nhau() {
    assert_eq!(kinds("1 // hai\n2"), vec![TokKind::SoNguyen(1), TokKind::SoNguyen(2)]);
    assert_eq!(kinds("1 /* a /* b */ c */ 2"), vec![TokKind::SoNguyen(1), TokKind::SoNguyen(2)]);
}

#[test]
fn toan_tu_nhieu_ky_tu_uu_tien_ban_dai_hon() {
    assert_eq!(kinds("=="), vec![TokKind::Bang]);
    assert_eq!(kinds("="), vec![TokKind::Gan]);
    assert_eq!(kinds("->"), vec![TokKind::MuiTen]);
    assert_eq!(kinds("=>"), vec![TokKind::MuiTenDam]);
    assert_eq!(kinds("::"), vec![TokKind::DuongDan]);
    assert_eq!(kinds("..="), vec![TokKind::HaiChamBang]);
}

#[test]
fn tieng_viet_trong_chuoi_van_hoat_dong() {
    assert_eq!(
        kinds(r#"let s = "Xin chào, thế giới!";"#).len(),
        5 // let, s, =, chuỗi, ;
    );
}

// ── Chẩn đoán ───────────────────────────────────────────────────────────────

#[test]
fn nhay_cong_duoc_bao_dung_ma_loi() {
    assert_eq!(loi_dau("let s = “abc”;"), "BR0001");
}

#[test]
fn chuoi_chua_dong() {
    assert_eq!(loi_dau(r#"let s = "abc;"#), "BR0007");
}

#[test]
fn chu_thich_khoi_chua_dong() {
    assert_eq!(loi_dau("/* mở mà không đóng"), "BR0002");
}

#[test]
fn so_nguyen_qua_lon() {
    assert_eq!(loi_dau("99999999999999999999"), "BR0005");
}

#[test]
fn hau_to_kieu_sai() {
    assert_eq!(loi_dau("let a = 10x;"), "BR0004");
}

#[test]
fn nhay_don_nhieu_ky_tu() {
    assert_eq!(loi_dau("let c = 'abc';"), "BR0009");
}

#[test]
fn thong_bao_loi_co_du_ba_tang() {
    let src = "let s = “abc”;";
    let (_t, d) = quet(src);
    let sm = SourceMap::new(src);
    let s = d.render(&sm);
    assert!(s.contains("vì sao:"), "thiếu tầng giải thích:\n{s}");
    assert!(s.contains("cách sửa:"), "thiếu tầng hướng dẫn sửa:\n{s}");
    assert!(s.contains("--> dòng 1:"), "thiếu vị trí:\n{s}");
}

#[test]
fn lexer_khong_dung_o_loi_dau_tien() {
    // Hai ký tự lạ -> phải thấy cả hai, việc rút gọn là của tầng trên.
    let (_t, d) = quet("let a = @; let b = @;");
    assert_eq!(d.iter().filter(|x| x.code == "BR0001").count(), 2);
}

#[test]
fn khong_panic_voi_dau_vao_ky_quai() {
    for src in ["", "\u{0}", "'", "\"", "/*", "0x", "..", "\u{1F600}", "let"] {
        let _ = quet(src); // chỉ cần không panic
    }
}
