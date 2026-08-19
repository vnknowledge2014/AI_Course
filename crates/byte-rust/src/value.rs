//! Giá trị lúc chạy.
//!
//! Mô hình giá trị cố tình **gần với mô hình tư duy của người học** hơn là với
//! biểu diễn bộ nhớ thật của Rust. Ví dụ `Vec` ở đây là một `Vec` của Rust host
//! chứ không phải con trỏ + sức chứa + độ dài. Đổi lại, ta mô phỏng **quyền sở
//! hữu** một cách tường minh (`da_chuyen`) vì đó chính là thứ cần dạy.

use crate::ast::Mau;
use crate::span::Span;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Ô nhớ dùng chung — nền cho tham chiếu `&T` / `&mut T`.
pub type O = Rc<RefCell<GiaTri>>;

#[derive(Debug, Clone)]
pub enum GiaTri {
    Rong,
    SoNguyen(i64),
    SoThuc(f64),
    DungSai(bool),
    KyTu(char),
    Chuoi(Rc<String>),
    Tuple(Rc<Vec<GiaTri>>),
    /// Dùng cho cả mảng `[T; n]` lẫn `Vec<T>`.
    Day(Rc<RefCell<Vec<GiaTri>>>),
    Struct {
        ten: Rc<str>,
        truong: Rc<RefCell<HashMap<String, GiaTri>>>,
    },
    /// `Some(3)`, `Color::Red`, `Ok(x)`
    BienThe {
        enum_ten: Rc<str>,
        bien_the: Rc<str>,
        gia_tri: Rc<Vec<GiaTri>>,
    },
    BeQuan(Rc<BeQuan>),
    /// `0..10`
    Dai { tu: i64, den: i64, bao_gom_cuoi: bool },
    /// Tham chiếu tới một ô. `co_the_sua` phân biệt `&T` với `&mut T`.
    ThamChieu { o: O, co_the_sua: bool },
    /// Giá trị đã bị chuyển quyền sở hữu đi nơi khác.
    ///
    /// Không phải giá trị thật — chỉ là bia mộ, để khi người học dùng lại biến
    /// đã move ta chỉ đúng được chỗ nó bị chuyển đi.
    ///
    /// `trong_nhanh` phân biệt hai tình huống rất khác nhau:
    /// - `false` — move nằm trên chuỗi câu lệnh thẳng hàng. `rustc` chắc chắn
    ///   từ chối, nên ta báo lỗi thật (E0382).
    /// - `true` — move nằm trong `if`/`match`/vòng lặp/closure. Ta chỉ biết
    ///   nhánh **đã chạy**, còn `rustc` xét **mọi** nhánh. Kiểm kiểu này cần
    ///   CFG mà ta không có, nên phải trả `ChuaHoTro` thay vì đoán bừa.
    DaChuyen { chuyen_tai: Span, trong_nhanh: bool },
}

#[derive(Debug)]
pub struct BeQuan {
    pub tham_so: Vec<Mau>,
    pub than: Rc<crate::ast::BieuThuc>,
    /// Môi trường bắt được lúc tạo closure.
    pub bat: Vec<HashMap<String, O>>,
}

impl GiaTri {
    /// Tên kiểu để hiển thị trong thông báo lỗi.
    pub fn ten_kieu(&self) -> String {
        match self {
            GiaTri::Rong => "()".into(),
            GiaTri::SoNguyen(_) => "i64".into(),
            GiaTri::SoThuc(_) => "f64".into(),
            GiaTri::DungSai(_) => "bool".into(),
            GiaTri::KyTu(_) => "char".into(),
            GiaTri::Chuoi(_) => "String".into(),
            GiaTri::Tuple(v) => {
                let ps: Vec<String> = v.iter().map(|x| x.ten_kieu()).collect();
                format!("({})", ps.join(", "))
            }
            GiaTri::Day(v) => match v.borrow().first() {
                Some(x) => format!("Vec<{}>", x.ten_kieu()),
                None => "Vec<_>".into(),
            },
            GiaTri::Struct { ten, .. } => ten.to_string(),
            GiaTri::BienThe { enum_ten, .. } => enum_ten.to_string(),
            GiaTri::BeQuan(_) => "closure".into(),
            GiaTri::Dai { .. } => "Range".into(),
            GiaTri::ThamChieu { o, co_the_sua } => {
                let trong = o.borrow().ten_kieu();
                if *co_the_sua { format!("&mut {trong}") } else { format!("&{trong}") }
            }
            GiaTri::DaChuyen { .. } => "<đã chuyển>".into(),
        }
    }

    /// Cách hiển thị của `{}` trong `println!` — tương ứng trait `Display`.
    pub fn hien_thi(&self) -> String {
        match self {
            GiaTri::Rong => "()".into(),
            GiaTri::SoNguyen(n) => n.to_string(),
            GiaTri::SoThuc(f) => {
                if f.fract() == 0.0 && f.is_finite() {
                    format!("{f:.0}")
                } else {
                    f.to_string()
                }
            }
            GiaTri::DungSai(b) => b.to_string(),
            GiaTri::KyTu(c) => c.to_string(),
            GiaTri::Chuoi(s) => s.to_string(),
            GiaTri::ThamChieu { o, .. } => o.borrow().hien_thi(),
            khac => khac.go_ro(),
        }
    }

    /// Cách hiển thị của `{:?}` — tương ứng trait `Debug`.
    pub fn go_ro(&self) -> String {
        match self {
            GiaTri::Rong => "()".into(),
            GiaTri::SoNguyen(n) => n.to_string(),
            GiaTri::SoThuc(f) => {
                if f.fract() == 0.0 && f.is_finite() { format!("{f:.1}") } else { f.to_string() }
            }
            GiaTri::DungSai(b) => b.to_string(),
            GiaTri::KyTu(c) => format!("'{c}'"),
            GiaTri::Chuoi(s) => format!("{:?}", s.as_str()),
            GiaTri::Tuple(v) => {
                let ps: Vec<String> = v.iter().map(|x| x.go_ro()).collect();
                if ps.len() == 1 { format!("({},)", ps[0]) } else { format!("({})", ps.join(", ")) }
            }
            GiaTri::Day(v) => {
                let ps: Vec<String> = v.borrow().iter().map(|x| x.go_ro()).collect();
                format!("[{}]", ps.join(", "))
            }
            GiaTri::Struct { ten, truong } => {
                let t = truong.borrow();
                let mut khoa: Vec<&String> = t.keys().collect();
                khoa.sort();
                let ps: Vec<String> =
                    khoa.iter().map(|k| format!("{k}: {}", t[*k].go_ro())).collect();
                if ps.is_empty() { ten.to_string() } else { format!("{ten} {{ {} }}", ps.join(", ")) }
            }
            GiaTri::BienThe { bien_the, gia_tri, .. } => {
                if gia_tri.is_empty() {
                    bien_the.to_string()
                } else {
                    let ps: Vec<String> = gia_tri.iter().map(|x| x.go_ro()).collect();
                    format!("{bien_the}({})", ps.join(", "))
                }
            }
            GiaTri::BeQuan(_) => "<closure>".into(),
            GiaTri::Dai { tu, den, bao_gom_cuoi } => {
                if *bao_gom_cuoi { format!("{tu}..={den}") } else { format!("{tu}..{den}") }
            }
            GiaTri::ThamChieu { o, .. } => o.borrow().go_ro(),
            GiaTri::DaChuyen { .. } => "<đã chuyển>".into(),
        }
    }

    /// So sánh theo giá trị — tương ứng trait `PartialEq`.
    pub fn bang(&self, khac: &GiaTri) -> bool {
        use GiaTri::*;
        match (self, khac) {
            (ThamChieu { o, .. }, b) => o.borrow().bang(b),
            (a, ThamChieu { o, .. }) => a.bang(&o.borrow()),
            (Rong, Rong) => true,
            (SoNguyen(a), SoNguyen(b)) => a == b,
            (SoThuc(a), SoThuc(b)) => a == b,
            (SoNguyen(a), SoThuc(b)) | (SoThuc(b), SoNguyen(a)) => (*a as f64) == *b,
            (DungSai(a), DungSai(b)) => a == b,
            (KyTu(a), KyTu(b)) => a == b,
            (Chuoi(a), Chuoi(b)) => a == b,
            (Tuple(a), Tuple(b)) => {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.bang(y))
            }
            (Day(a), Day(b)) => {
                let (a, b) = (a.borrow(), b.borrow());
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.bang(y))
            }
            (Struct { ten: t1, truong: f1 }, Struct { ten: t2, truong: f2 }) => {
                if t1 != t2 { return false; }
                let (f1, f2) = (f1.borrow(), f2.borrow());
                f1.len() == f2.len()
                    && f1.iter().all(|(k, v)| f2.get(k).is_some_and(|w| v.bang(w)))
            }
            (
                BienThe { enum_ten: e1, bien_the: b1, gia_tri: g1 },
                BienThe { enum_ten: e2, bien_the: b2, gia_tri: g2 },
            ) => {
                e1 == e2
                    && b1 == b2
                    && g1.len() == g2.len()
                    && g1.iter().zip(g2.iter()).all(|(x, y)| x.bang(y))
            }
            _ => false,
        }
    }

    /// Thứ tự — trả `None` khi hai giá trị không so sánh được.
    pub fn so_sanh(&self, khac: &GiaTri) -> Option<std::cmp::Ordering> {
        use GiaTri::*;
        match (self, khac) {
            (ThamChieu { o, .. }, b) => o.borrow().so_sanh(b),
            (a, ThamChieu { o, .. }) => a.so_sanh(&o.borrow()),
            (SoNguyen(a), SoNguyen(b)) => Some(a.cmp(b)),
            (SoThuc(a), SoThuc(b)) => a.partial_cmp(b),
            (SoNguyen(a), SoThuc(b)) => (*a as f64).partial_cmp(b),
            (SoThuc(a), SoNguyen(b)) => a.partial_cmp(&(*b as f64)),
            (KyTu(a), KyTu(b)) => Some(a.cmp(b)),
            (Chuoi(a), Chuoi(b)) => Some(a.cmp(b)),
            (DungSai(a), DungSai(b)) => Some(a.cmp(b)),
            _ => None,
        }
    }

    /// Giá trị này có thoả điều kiện của `if` không?
    pub fn la_dung(&self) -> Option<bool> {
        match self {
            GiaTri::DungSai(b) => Some(*b),
            GiaTri::ThamChieu { o, .. } => o.borrow().la_dung(),
            _ => None,
        }
    }

    /// Kiểu này có `Copy` không?
    ///
    /// Quyết định giá trị bị **sao chép** hay bị **chuyển quyền sở hữu** khi gán
    /// — chính là ranh giới mà bài học về ownership xoay quanh.
    pub fn la_copy(&self) -> bool {
        matches!(
            self,
            GiaTri::Rong
                | GiaTri::SoNguyen(_)
                | GiaTri::SoThuc(_)
                | GiaTri::DungSai(_)
                | GiaTri::KyTu(_)
                | GiaTri::Dai { .. }
                | GiaTri::ThamChieu { co_the_sua: false, .. }
        )
    }

    /// Đi hết chuỗi tham chiếu để lấy giá trị thật.
    pub fn giai_tham(&self) -> GiaTri {
        match self {
            GiaTri::ThamChieu { o, .. } => o.borrow().giai_tham(),
            khac => khac.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hien_thi_khac_go_ro_o_chuoi_va_ky_tu() {
        let s = GiaTri::Chuoi(Rc::new("chào".into()));
        assert_eq!(s.hien_thi(), "chào");
        assert_eq!(s.go_ro(), "\"chào\"");
        let c = GiaTri::KyTu('a');
        assert_eq!(c.hien_thi(), "a");
        assert_eq!(c.go_ro(), "'a'");
    }

    #[test]
    fn so_thuc_hien_thi_gon() {
        assert_eq!(GiaTri::SoThuc(2.0).hien_thi(), "2");
        assert_eq!(GiaTri::SoThuc(2.0).go_ro(), "2.0");
        assert_eq!(GiaTri::SoThuc(2.5).hien_thi(), "2.5");
    }

    #[test]
    fn tuple_mot_phan_tu_co_dau_phay() {
        let t = GiaTri::Tuple(Rc::new(vec![GiaTri::SoNguyen(1)]));
        assert_eq!(t.go_ro(), "(1,)");
    }

    #[test]
    fn la_copy_phan_biet_dung_nhom_kieu() {
        assert!(GiaTri::SoNguyen(1).la_copy());
        assert!(GiaTri::DungSai(true).la_copy());
        assert!(!GiaTri::Chuoi(Rc::new("x".into())).la_copy());
        assert!(!GiaTri::Day(Rc::new(RefCell::new(vec![]))).la_copy());
    }

    #[test]
    fn bang_xuyen_qua_tham_chieu() {
        let o = Rc::new(RefCell::new(GiaTri::SoNguyen(5)));
        let r = GiaTri::ThamChieu { o, co_the_sua: false };
        assert!(r.bang(&GiaTri::SoNguyen(5)));
        assert!(GiaTri::SoNguyen(5).bang(&r));
    }

    #[test]
    fn so_sanh_tra_none_khi_khong_cung_kieu() {
        assert!(GiaTri::SoNguyen(1).so_sanh(&GiaTri::Chuoi(Rc::new("a".into()))).is_none());
        assert!(GiaTri::SoNguyen(1).so_sanh(&GiaTri::SoNguyen(2)).is_some());
    }
}
