//! Cây cú pháp trừu tượng cho tập con Rust dùng trong giảng dạy.
//!
//! Mọi nút đều mang `Span` để chẩn đoán ở các tầng sau (kiểm kiểu, kiểm mượn,
//! thực thi) trỏ được đúng vào mã nguồn.
//!
//! Phạm vi tập con — cố ý hẹp, mở rộng dần theo từng Realm của giáo trình:
//! - Realm "Nhập môn": `let`, số, chuỗi, `fn`, `if`, `while`, `for`, `println!`
//! - Realm "Dữ liệu":  `struct`, `enum`, `match`, `Option`, `Result`, `Vec`
//! - Realm "Sở hữu":   `&`, `&mut`, move, clone — nơi kiểm mượn bắt đầu hoạt động
//! - Realm "Trừu tượng hoá": `trait`, generic, closure

use crate::span::Span;

// ── Kiểu ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Kieu {
    /// `i64`, `String`, `Vec<i64>`, `Option<T>`, `my_mod::Thing`
    DuongDan { doan: Vec<String>, tham_so: Vec<Kieu>, span: Span },
    /// `&T` hoặc `&mut T`
    ThamChieu { co_the_sua: bool, ben_trong: Box<Kieu>, span: Span },
    /// `(A, B)` — `()` là tuple rỗng, tức kiểu unit
    Tuple { phan_tu: Vec<Kieu>, span: Span },
    /// `[T; n]`
    Mang { phan_tu: Box<Kieu>, so_luong: Option<u64>, span: Span },
    /// `_` — nhờ trình suy luận điền
    SuyLuan { span: Span },
}

impl Kieu {
    pub fn span(&self) -> Span {
        match self {
            Kieu::DuongDan { span, .. }
            | Kieu::ThamChieu { span, .. }
            | Kieu::Tuple { span, .. }
            | Kieu::Mang { span, .. }
            | Kieu::SuyLuan { span } => *span,
        }
    }

    /// Tên hiển thị trong thông báo lỗi, ví dụ `&mut Vec<i64>`.
    pub fn hien_thi(&self) -> String {
        match self {
            Kieu::DuongDan { doan, tham_so, .. } => {
                let ten = doan.join("::");
                if tham_so.is_empty() {
                    ten
                } else {
                    let ts: Vec<String> = tham_so.iter().map(|t| t.hien_thi()).collect();
                    format!("{ten}<{}>", ts.join(", "))
                }
            }
            Kieu::ThamChieu { co_the_sua, ben_trong, .. } => {
                if *co_the_sua {
                    format!("&mut {}", ben_trong.hien_thi())
                } else {
                    format!("&{}", ben_trong.hien_thi())
                }
            }
            Kieu::Tuple { phan_tu, .. } => {
                let ps: Vec<String> = phan_tu.iter().map(|t| t.hien_thi()).collect();
                format!("({})", ps.join(", "))
            }
            Kieu::Mang { phan_tu, so_luong, .. } => match so_luong {
                Some(n) => format!("[{}; {n}]", phan_tu.hien_thi()),
                None => format!("[{}]", phan_tu.hien_thi()),
            },
            Kieu::SuyLuan { .. } => "_".to_string(),
        }
    }
}

// ── Mẫu (pattern) ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Mau {
    /// `_`
    BoQua { span: Span },
    /// `x`, `mut x`, `ref x`
    Ten { ten: String, co_the_sua: bool, la_ref: bool, span: Span },
    /// `42`, `"abc"`, `true`
    HangSo { gia_tri: HangSo, span: Span },
    /// `(a, b)`
    Tuple { phan_tu: Vec<Mau>, span: Span },
    /// `Some(x)`, `Point { x, y }`, `Color::Red`
    BienThe { duong_dan: Vec<String>, truong: MauTruong, span: Span },
    /// `1..=5`
    Dai { tu: Box<Mau>, den: Box<Mau>, bao_gom_cuoi: bool, span: Span },
    /// `A | B`
    Hoac { nhanh: Vec<Mau>, span: Span },
}

#[derive(Debug, Clone, PartialEq)]
pub enum MauTruong {
    /// `Color::Red` — không có gì đi kèm
    Khong,
    /// `Some(x)` — theo vị trí
    TheoViTri(Vec<Mau>),
    /// `Point { x, y: b }` — theo tên; `dau_ba_cham` cho `..`
    TheoTen { truong: Vec<(String, Mau)>, dau_ba_cham: bool },
}

impl Mau {
    pub fn span(&self) -> Span {
        match self {
            Mau::BoQua { span }
            | Mau::Ten { span, .. }
            | Mau::HangSo { span, .. }
            | Mau::Tuple { span, .. }
            | Mau::BienThe { span, .. }
            | Mau::Dai { span, .. }
            | Mau::Hoac { span, .. } => *span,
        }
    }

    /// Mọi tên biến mà mẫu này ràng buộc.
    pub fn ten_rang_buoc(&self, ra: &mut Vec<(String, bool, Span)>) {
        match self {
            Mau::Ten { ten, co_the_sua, span, .. } => ra.push((ten.clone(), *co_the_sua, *span)),
            Mau::Tuple { phan_tu, .. } => phan_tu.iter().for_each(|p| p.ten_rang_buoc(ra)),
            Mau::BienThe { truong, .. } => match truong {
                MauTruong::Khong => {}
                MauTruong::TheoViTri(ps) => ps.iter().for_each(|p| p.ten_rang_buoc(ra)),
                MauTruong::TheoTen { truong, .. } => {
                    truong.iter().for_each(|(_, p)| p.ten_rang_buoc(ra))
                }
            },
            Mau::Hoac { nhanh, .. } => {
                if let Some(dau) = nhanh.first() {
                    dau.ten_rang_buoc(ra)
                }
            }
            Mau::BoQua { .. } | Mau::HangSo { .. } | Mau::Dai { .. } => {}
        }
    }
}

// ── Hằng số ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum HangSo {
    SoNguyen(i64),
    SoThuc(f64),
    Chuoi(String),
    KyTu(char),
    DungSai(bool),
}

// ── Toán tử ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToanTuHai {
    Cong, Tru, Nhan, Chia, Du,
    Bang, KhacBang, NhoHon, LonHon, NhoBang, LonBang,
    Va, Hoac,
    VaBit, HoacBit, XorBit, DichTrai, DichPhai,
}

impl ToanTuHai {
    pub fn ky_hieu(self) -> &'static str {
        use ToanTuHai::*;
        match self {
            Cong => "+", Tru => "-", Nhan => "*", Chia => "/", Du => "%",
            Bang => "==", KhacBang => "!=", NhoHon => "<", LonHon => ">",
            NhoBang => "<=", LonBang => ">=",
            Va => "&&", Hoac => "||",
            VaBit => "&", HoacBit => "|", XorBit => "^",
            DichTrai => "<<", DichPhai => ">>",
        }
    }

    /// Độ ưu tiên; số lớn hơn buộc chặt hơn. Theo đúng bảng của Rust.
    pub fn uu_tien(self) -> u8 {
        use ToanTuHai::*;
        match self {
            Hoac => 1,
            Va => 2,
            Bang | KhacBang | NhoHon | LonHon | NhoBang | LonBang => 3,
            HoacBit => 4,
            XorBit => 5,
            VaBit => 6,
            DichTrai | DichPhai => 7,
            Cong | Tru => 8,
            Nhan | Chia | Du => 9,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToanTuMot {
    Am,   // -x
    Phu,  // !x
}

// ── Biểu thức ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum BieuThuc {
    HangSo { gia_tri: HangSo, span: Span },
    /// `x`, `Color::Red`, `Vec::new`
    DuongDan { doan: Vec<String>, span: Span },
    MotNgoi { toan_tu: ToanTuMot, toan_hang: Box<BieuThuc>, span: Span },
    HaiNgoi { toan_tu: ToanTuHai, trai: Box<BieuThuc>, phai: Box<BieuThuc>, span: Span },
    /// `&x`, `&mut x`
    Muon { co_the_sua: bool, gia_tri: Box<BieuThuc>, span: Span },
    /// `*x`
    GiaiTham { gia_tri: Box<BieuThuc>, span: Span },
    /// `f(a, b)`
    GoiHam { ham: Box<BieuThuc>, doi_so: Vec<BieuThuc>, span: Span },
    /// `x.method(a)`
    GoiPhuongThuc { doi_tuong: Box<BieuThuc>, ten: String, doi_so: Vec<BieuThuc>, span: Span },
    /// `x.field` hoặc `t.0`
    TruyCapTruong { doi_tuong: Box<BieuThuc>, ten: String, span: Span },
    /// `v[i]`
    ChiSo { doi_tuong: Box<BieuThuc>, chi_so: Box<BieuThuc>, span: Span },
    /// `x = 1`, `x += 1`
    Gan { dich: Box<BieuThuc>, toan_tu: Option<ToanTuHai>, gia_tri: Box<BieuThuc>, span: Span },
    Khoi(Box<Khoi>),
    Neu { dieu_kien: Box<BieuThuc>, than: Box<Khoi>, nguoc_lai: Option<Box<BieuThuc>>, span: Span },
    KhopMau { gia_tri: Box<BieuThuc>, nhanh: Vec<NhanhKhop>, span: Span },
    Lap { nhan: Option<String>, than: Box<Khoi>, span: Span },
    Trong { dieu_kien: Box<BieuThuc>, than: Box<Khoi>, span: Span },
    Cho { mau: Mau, day: Box<BieuThuc>, than: Box<Khoi>, span: Span },
    /// `|x| x + 1`
    BeQuan { tham_so: Vec<(Mau, Option<Kieu>)>, than: Box<BieuThuc>, span: Span },
    /// `(a, b)`; `()` là unit
    Tuple { phan_tu: Vec<BieuThuc>, span: Span },
    /// `[1, 2, 3]` hoặc `[0; 10]`
    Mang { phan_tu: Vec<BieuThuc>, lap_lai: Option<Box<BieuThuc>>, span: Span },
    /// `Point { x: 1, y: 2 }`
    KhoiTaoStruct { duong_dan: Vec<String>, truong: Vec<(String, BieuThuc)>, con_lai: Option<Box<BieuThuc>>, span: Span },
    /// `1..5`, `1..=5`
    Dai { tu: Option<Box<BieuThuc>>, den: Option<Box<BieuThuc>>, bao_gom_cuoi: bool, span: Span },
    TraVe { gia_tri: Option<Box<BieuThuc>>, span: Span },
    Thoat { nhan: Option<String>, gia_tri: Option<Box<BieuThuc>>, span: Span },
    TiepTuc { nhan: Option<String>, span: Span },
    /// `expr?`
    LanTruyenLoi { gia_tri: Box<BieuThuc>, span: Span },
    /// `x as i64`
    Ep { gia_tri: Box<BieuThuc>, kieu: Kieu, span: Span },
    /// `println!("{}", x)` — macro là dạng riêng vì cú pháp đặc biệt
    Macro { ten: String, doi_so: Vec<BieuThuc>, span: Span },
}

impl BieuThuc {
    pub fn span(&self) -> Span {
        use BieuThuc::*;
        match self {
            HangSo { span, .. } | DuongDan { span, .. } | MotNgoi { span, .. }
            | HaiNgoi { span, .. } | Muon { span, .. } | GiaiTham { span, .. }
            | GoiHam { span, .. } | GoiPhuongThuc { span, .. } | TruyCapTruong { span, .. }
            | ChiSo { span, .. } | Gan { span, .. } | Neu { span, .. }
            | KhopMau { span, .. } | Lap { span, .. } | Trong { span, .. }
            | Cho { span, .. } | BeQuan { span, .. } | Tuple { span, .. }
            | Mang { span, .. } | KhoiTaoStruct { span, .. } | Dai { span, .. }
            | TraVe { span, .. } | Thoat { span, .. } | TiepTuc { span, .. }
            | LanTruyenLoi { span, .. } | Ep { span, .. } | Macro { span, .. } => *span,
            Khoi(k) => k.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NhanhKhop {
    pub mau: Mau,
    /// `if cond` sau mẫu
    pub dieu_kien: Option<BieuThuc>,
    pub than: BieuThuc,
    pub span: Span,
}

// ── Câu lệnh & khối ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum CauLenh {
    /// `let mau: Kieu = gia_tri;`
    Let { mau: Mau, kieu: Option<Kieu>, gia_tri: Option<BieuThuc>, span: Span },
    /// Biểu thức kết thúc bằng `;`
    BieuThuc { bt: BieuThuc, span: Span },
    /// Khai báo lồng trong hàm
    Muc(Muc),
}

impl CauLenh {
    pub fn span(&self) -> Span {
        match self {
            CauLenh::Let { span, .. } | CauLenh::BieuThuc { span, .. } => *span,
            CauLenh::Muc(m) => m.span(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Khoi {
    pub cau_lenh: Vec<CauLenh>,
    /// Biểu thức cuối không có `;` — giá trị của khối.
    pub gia_tri_cuoi: Option<BieuThuc>,
    pub span: Span,
}

// ── Khai báo cấp cao ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ThamSo {
    pub mau: Mau,
    pub kieu: Kieu,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ham {
    pub ten: String,
    pub tham_so_kieu: Vec<String>,
    pub tham_so: Vec<ThamSo>,
    pub kieu_tra_ve: Option<Kieu>,
    pub than: Khoi,
    pub cong_khai: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThanStruct {
    /// `struct P { x: i64 }`
    TheoTen(Vec<(String, Kieu, bool)>), // (tên, kiểu, công khai)
    /// `struct Email(String);`
    TheoViTri(Vec<(Kieu, bool)>),
    /// `struct Unit;`
    Rong,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub ten: String,
    pub tham_so_kieu: Vec<String>,
    pub than: ThanStruct,
    pub cong_khai: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BienThe {
    pub ten: String,
    pub than: ThanStruct,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub ten: String,
    pub tham_so_kieu: Vec<String>,
    pub bien_the: Vec<BienThe>,
    pub cong_khai: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Impl {
    /// `impl Trait for Kieu` — `None` nghĩa là impl trần.
    pub trait_ten: Option<Vec<String>>,
    pub kieu: Kieu,
    pub tham_so_kieu: Vec<String>,
    pub ham: Vec<Ham>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Trait {
    pub ten: String,
    pub tham_so_kieu: Vec<String>,
    /// Hàm không có thân = phương thức bắt buộc.
    pub ham: Vec<(Ham, bool)>, // (hàm, có thân mặc định)
    pub cong_khai: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Muc {
    Ham(Ham),
    Struct(Struct),
    Enum(Enum),
    Impl(Impl),
    Trait(Trait),
    /// `const TEN: Kieu = gia_tri;`
    Const { ten: String, kieu: Kieu, gia_tri: BieuThuc, cong_khai: bool, span: Span },
    /// `use a::b::c;` — ghi nhận nhưng chưa xử lý module thật.
    Use { doan: Vec<String>, span: Span },
}

impl Muc {
    pub fn span(&self) -> Span {
        match self {
            Muc::Ham(h) => h.span,
            Muc::Struct(s) => s.span,
            Muc::Enum(e) => e.span,
            Muc::Impl(i) => i.span,
            Muc::Trait(t) => t.span,
            Muc::Const { span, .. } | Muc::Use { span, .. } => *span,
        }
    }

    pub fn ten(&self) -> Option<&str> {
        match self {
            Muc::Ham(h) => Some(&h.ten),
            Muc::Struct(s) => Some(&s.ten),
            Muc::Enum(e) => Some(&e.ten),
            Muc::Trait(t) => Some(&t.ten),
            Muc::Const { ten, .. } => Some(ten),
            Muc::Impl(_) | Muc::Use { .. } => None,
        }
    }
}

/// Toàn bộ chương trình của người học.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ChuongTrinh {
    pub muc: Vec<Muc>,
}
