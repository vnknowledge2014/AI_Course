//! Kiểm tính vét cạn của `match` — phủ không gian giá trị.
//!
//! # Vì sao cần thuật toán chứ không phải vài câu `if`
//!
//! Bản đầu chỉ xử lý đúng một hình dạng: `match` trên một enum, với các mẫu là
//! biến thể phẳng. Mọi hình dạng khác im lặng:
//!
//! ```ignore
//! match b { true => 1 }                     // bool  — E0004
//! match n { 0 => 10, 1 => 20 }              // số    — E0004
//! match (a, b) { (true,true)=>1, (false,false)=>2 }  // tuple — E0004
//! match h { Hop::Chua(Mau::Do) => 1 }       // lồng  — E0004
//! match m { Mau::Do => 1, _ if false => 2 } // guard không phủ — E0004
//! ```
//!
//! Đây không phải chi tiết vụn: **tính vét cạn chính là thứ làm `match` an toàn
//! hơn `switch`**. Nếu Byte im lặng, người học mất đúng bài học mà `match` tồn
//! tại để dạy — và tệ hơn, họ tin rằng bỏ sót nhánh là chuyện bình thường.
//!
//! # Cách làm
//!
//! Mỗi kiểu ứng với một **không gian giá trị**. Mỗi nhánh **không có guard** trừ
//! đi phần nó phủ. Còn sót lại thì báo lỗi kèm một **nhân chứng** — một giá trị
//! cụ thể không nhánh nào bắt, để người học thấy ngay mình quên gì.
//!
//! Nhánh có guard (`if …`) **không tính là phủ**, đúng như `rustc`: guard chỉ
//! biết được lúc chạy.

use crate::ast::{Mau, MauTruong};
use std::collections::HashMap;

/// Không gian giá trị của một kiểu.
#[derive(Debug, Clone)]
pub enum KhongGian {
    /// Tập hữu hạn các nhãn, mỗi nhãn có thể mang theo các không gian con.
    /// `bool` → [("true",[]), ("false",[])]; `Option<T>` → [("Some",[T]), ("None",[])]
    HuuHan(Vec<(String, Vec<KhongGian>)>),
    /// Vô hạn: số nguyên, chuỗi, ký tự. Chỉ `_` hoặc một binding mới phủ nổi.
    VoHan,
    /// Tích Descartes: `(A, B)`.
    Tich(Vec<KhongGian>),
    /// Không mô hình hoá được — im lặng, không bao giờ báo lỗi.
    KhongBiet,
}

impl KhongGian {
    fn huu_han(&self) -> bool {
        match self {
            KhongGian::HuuHan(v) => v.iter().all(|(_, con)| con.iter().all(|c| c.huu_han())),
            KhongGian::Tich(v) => v.iter().all(|c| c.huu_han()),
            _ => false,
        }
    }
}

/// Mẫu có phủ TRỌN không gian không (`_`, hoặc một tên trần)?
fn la_bao_quat(m: &Mau) -> bool {
    matches!(m, Mau::BoQua { .. } | Mau::Ten { .. })
}

/// Tìm một giá trị mà không nhánh nào bắt. `None` = đã phủ hết.
///
/// `mau` chỉ gồm các nhánh **không có guard**.
pub fn thieu(kg: &KhongGian, mau: &[&Mau]) -> Option<String> {
    // Một mẫu bao quát là đủ cho mọi không gian.
    if mau.iter().any(|m| la_bao_quat(m)) {
        return None;
    }
    // Mẫu Hoặc: tách phẳng ra rồi xét lại.
    if mau.iter().any(|m| matches!(m, Mau::Hoac { .. })) {
        let mut phang: Vec<&Mau> = Vec::new();
        for m in mau {
            match m {
                Mau::Hoac { nhanh, .. } => phang.extend(nhanh.iter()),
                khac => phang.push(khac),
            }
        }
        return thieu(kg, &phang);
    }

    match kg {
        KhongGian::KhongBiet => None,

        // Không gian vô hạn mà không có mẫu bao quát ⇒ chắc chắn thiếu.
        KhongGian::VoHan => Some("_".to_string()),

        KhongGian::HuuHan(nhan) => {
            for (ten, con) in nhan {
                // Các mẫu chạm tới đúng nhãn này.
                let cua_nhan: Vec<&Mau> = mau
                    .iter()
                    .filter(|m| khop_nhan(m, ten))
                    .copied()
                    .collect();
                if cua_nhan.is_empty() {
                    return Some(nhan_chung_nhan(ten, con));
                }
                // Nhãn có dữ liệu đi kèm: phải phủ hết cả phần bên trong.
                if !con.is_empty() {
                    for (i, kg_con) in con.iter().enumerate() {
                        let mau_con: Vec<&Mau> = cua_nhan
                            .iter()
                            .filter_map(|m| mau_thanh_phan(m, i))
                            .collect();
                        if mau_con.len() < cua_nhan.len() {
                            // Có nhánh không nêu rõ thành phần ⇒ coi như bao quát.
                            continue;
                        }
                        if let Some(w) = thieu(kg_con, &mau_con) {
                            return Some(format!("{ten}({w})"));
                        }
                    }
                }
            }
            None
        }

        KhongGian::Tich(cac) => {
            if !kg.huu_han() {
                // Có thành phần vô hạn: chỉ mẫu bao quát mới phủ nổi, mà ta đã
                // kiểm ở trên rồi.
                return Some("(…)".to_string());
            }
            // Duyệt tích Descartes — chỉ khả thi vì mọi thành phần đều hữu hạn.
            for to_hop in tich_descartes(cac) {
                let duoc_bat = mau.iter().any(|m| bat_duoc_tuple(m, &to_hop, cac));
                if !duoc_bat {
                    return Some(format!("({})", to_hop.join(", ")));
                }
            }
            None
        }
    }
}

fn nhan_chung_nhan(ten: &str, con: &[KhongGian]) -> String {
    if con.is_empty() {
        ten.to_string()
    } else {
        format!("{ten}({})", vec!["_"; con.len()].join(", "))
    }
}

fn khop_nhan(m: &Mau, ten: &str) -> bool {
    match m {
        Mau::BienThe { duong_dan, .. } => duong_dan.last().map(String::as_str) == Some(ten),
        Mau::HangSo { gia_tri, .. } => match gia_tri {
            crate::ast::HangSo::DungSai(b) => (if *b { "true" } else { "false" }) == ten,
            _ => false,
        },
        _ => false,
    }
}

/// Mẫu con thứ `i` của một mẫu biến thể, nếu mẫu nêu rõ các thành phần.
fn mau_thanh_phan(m: &Mau, i: usize) -> Option<&Mau> {
    match m {
        Mau::BienThe { truong: MauTruong::TheoViTri(ps), .. } => ps.get(i),
        _ => None,
    }
}

fn tich_descartes(cac: &[KhongGian]) -> Vec<Vec<String>> {
    let mut ra: Vec<Vec<String>> = vec![Vec::new()];
    for kg in cac {
        let nhan: Vec<String> = match kg {
            KhongGian::HuuHan(v) => v.iter().map(|(t, _)| t.clone()).collect(),
            _ => return Vec::new(),
        };
        let mut moi = Vec::new();
        for cu in &ra {
            for n in &nhan {
                let mut x = cu.clone();
                x.push(n.clone());
                moi.push(x);
            }
        }
        ra = moi;
    }
    ra
}

fn bat_duoc_tuple(m: &Mau, to_hop: &[String], cac: &[KhongGian]) -> bool {
    match m {
        Mau::BoQua { .. } | Mau::Ten { .. } => true,
        Mau::Tuple { phan_tu, .. } if phan_tu.len() == to_hop.len() => {
            phan_tu.iter().zip(to_hop).enumerate().all(|(i, (p, nhan))| {
                la_bao_quat(p)
                    || khop_nhan(p, nhan)
                    || matches!(cac.get(i), Some(KhongGian::KhongBiet))
            })
        }
        Mau::Hoac { nhanh, .. } => nhanh.iter().any(|n| bat_duoc_tuple(n, to_hop, cac)),
        _ => false,
    }
}

/// Không gian giá trị của `bool`.
pub fn kg_bool() -> KhongGian {
    KhongGian::HuuHan(vec![("true".into(), vec![]), ("false".into(), vec![])])
}

/// Không gian của một enum, tra từ bảng định nghĩa.
///
/// `do_sau` chặn đệ quy vô hạn với enum tự tham chiếu (`enum List { Cons(List) }`).
pub fn kg_enum(
    ten: &str,
    bien_the: &HashMap<String, Vec<(String, Vec<String>)>>,
    do_sau: usize,
) -> KhongGian {
    if do_sau > 3 {
        return KhongGian::KhongBiet;
    }
    match bien_the.get(ten) {
        None => KhongGian::KhongBiet,
        Some(cac) => KhongGian::HuuHan(
            cac.iter()
                .map(|(vt, kieu_con)| {
                    let con = kieu_con
                        .iter()
                        .map(|k| match k.as_str() {
                            "bool" => kg_bool(),
                            _ if bien_the.contains_key(k) => kg_enum(k, bien_the, do_sau + 1),
                            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16"
                            | "u32" | "u64" | "u128" | "usize" | "f32" | "f64" | "char"
                            | "String" | "str" => KhongGian::VoHan,
                            _ => KhongGian::KhongBiet,
                        })
                        .collect();
                    (vt.clone(), con)
                })
                .collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::phan_tich;
    use crate::ast::*;

    fn mau_cua(src: &str) -> Vec<Mau> {
        let (ct, d) = phan_tich(src);
        assert!(!d.co_loi(), "mã mẫu phải phân tích được");
        let Muc::Ham(h) = &ct.muc[0] else { panic!() };
        let bt = h.than.gia_tri_cuoi.as_ref().or_else(|| {
            h.than.cau_lenh.iter().find_map(|c| match c {
                CauLenh::BieuThuc { bt, .. } => Some(bt),
                _ => None,
            })
        }).unwrap();
        let BieuThuc::KhopMau { nhanh, .. } = bt else { panic!("mong đợi match") };
        nhanh.iter().filter(|n| n.dieu_kien.is_none()).map(|n| n.mau.clone()).collect()
    }

    #[test]
    fn bool_thieu_mot_nhanh() {
        let m = mau_cua("fn f(b: bool) -> i64 { match b { true => 1 } }");
        let r: Vec<&Mau> = m.iter().collect();
        assert_eq!(thieu(&kg_bool(), &r), Some("false".into()));
    }

    #[test]
    fn bool_du_hai_nhanh() {
        let m = mau_cua("fn f(b: bool) -> i64 { match b { true => 1, false => 0 } }");
        let r: Vec<&Mau> = m.iter().collect();
        assert_eq!(thieu(&kg_bool(), &r), None);
    }

    #[test]
    fn so_nguyen_luon_thieu_neu_khong_co_bao_quat() {
        let m = mau_cua("fn f(n: i64) -> i64 { match n { 0 => 1, 1 => 2 } }");
        let r: Vec<&Mau> = m.iter().collect();
        assert_eq!(thieu(&KhongGian::VoHan, &r), Some("_".into()));
    }

    #[test]
    fn dau_gach_duoi_phu_moi_thu() {
        let m = mau_cua("fn f(n: i64) -> i64 { match n { 0 => 1, _ => 2 } }");
        let r: Vec<&Mau> = m.iter().collect();
        assert_eq!(thieu(&KhongGian::VoHan, &r), None);
    }

    #[test]
    fn tuple_bool_thieu_hai_to_hop() {
        let m = mau_cua(
            "fn f(a: bool, b: bool) -> i64 { match (a, b) { (true, true) => 1, (false, false) => 2 } }",
        );
        let r: Vec<&Mau> = m.iter().collect();
        let kg = KhongGian::Tich(vec![kg_bool(), kg_bool()]);
        assert!(thieu(&kg, &r).is_some(), "phải phát hiện thiếu tổ hợp");
    }

    #[test]
    fn tuple_bool_du_bon_to_hop() {
        let m = mau_cua(
            "fn f(a: bool, b: bool) -> i64 { match (a, b) { (true, true) => 1, (true, false) => 2, (false, true) => 3, (false, false) => 4 } }",
        );
        let r: Vec<&Mau> = m.iter().collect();
        let kg = KhongGian::Tich(vec![kg_bool(), kg_bool()]);
        assert_eq!(thieu(&kg, &r), None);
    }

    #[test]
    fn enum_long_nhau_thieu_ben_trong() {
        let mut bt = HashMap::new();
        bt.insert("Mau".to_string(), vec![("Do".to_string(), vec![]), ("Xanh".to_string(), vec![])]);
        bt.insert("Hop".to_string(), vec![("Chua".to_string(), vec!["Mau".to_string()])]);
        let m = mau_cua("fn f(h: Hop) -> i64 { match h { Hop::Chua(Mau::Do) => 1 } }");
        let r: Vec<&Mau> = m.iter().collect();
        let kg = kg_enum("Hop", &bt, 0);
        assert!(thieu(&kg, &r).is_some(), "phải thấy thiếu Mau::Xanh bên trong");
    }

    #[test]
    fn mau_hoac_duoc_tach_phang() {
        let m = mau_cua("fn f(b: bool) -> i64 { match b { true | false => 1 } }");
        let r: Vec<&Mau> = m.iter().collect();
        assert_eq!(thieu(&kg_bool(), &r), None);
    }
}
