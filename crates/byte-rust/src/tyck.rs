//! Kiểm kiểu nhẹ (`typecheck-lite`) và kiểm tính vét cạn của `match`.
//!
//! # Vì sao cần, dù đã có interpreter
//!
//! Interpreter là **động**: nó chỉ thấy đường đã đi. `rustc` là **tĩnh**: nó
//! xét mọi đường. Nên có cả một lớp lỗi mà chạy thử không bao giờ lộ ra:
//!
//! ```ignore
//! enum Mau { Do, Xanh }
//! fn ten(m: Mau) -> i64 {
//!     match m { Mau::Do => 1 }   // thiếu Mau::Xanh -> rustc: E0004
//! }
//! fn main() { ten(Mau::Do); }    // chạy thử: khớp nhánh Do, không lỗi gì
//! ```
//!
//! Bộ đối chiếu `byte-rust-conformance` tìm ra đúng 10 ca như vậy ở lần chạy
//! đầu tiên, trong khi 104 test viết tay không bắt được ca nào — vì test viết
//! tay chỉ kiểm những gì tác giả đã nghĩ tới.
//!
//! # Nguyên tắc khi không suy ra được kiểu
//!
//! Im lặng. Module này chỉ báo lỗi khi **chắc chắn**; mọi chỗ mờ đều để
//! interpreter hoặc `ChuaHoTro` xử lý. Báo oan một chương trình đúng cũng tệ
//! ngang báo đạt một chương trình sai — chỉ khác là người học mất niềm tin
//! theo hướng ngược lại.

use crate::ast::*;
use crate::diag::{Diagnostic, Diagnostics, Label};
use crate::span::Span;
use std::collections::HashMap;

/// Bề rộng và tính dấu của một kiểu số nguyên.
///
/// Gộp mọi kiểu số nguyên thành một `SoNguyen` duy nhất là cách `byte-rust` từng
/// hành xử — và nó xoá mất một trong những bài học trung tâm nhất khi chuyển từ
/// Python/JavaScript sang Rust: **Rust không ép kiểu số ngầm**.
///
/// `usize`/`isize` cố định 64-bit cho MỌI build (ADR-002 §5): mượn `usize` của
/// host sẽ khiến `usize::MAX` cho ba kết quả khác nhau giữa WASM32, native 64-bit
/// và `rustc` trên CI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KieuNguyen {
    I8, I16, I32, I64, I128, Isize,
    U8, U16, U32, U64, U128, Usize,
    /// Literal chưa bị ghim kiểu — linh hoạt, hợp với mọi bề rộng.
    ///
    /// `let x: u8 = 5;` hợp lệ vì `5` chưa ghim. Nhưng `let x: u8 = y;` với
    /// `y: i64` thì không. Phân biệt được hai ca đó là toàn bộ lý do biến thể
    /// này tồn tại.
    ChuaGhim,
}

impl KieuNguyen {
    pub fn ten(self) -> &'static str {
        use KieuNguyen::*;
        match self {
            I8 => "i8", I16 => "i16", I32 => "i32", I64 => "i64",
            I128 => "i128", Isize => "isize",
            U8 => "u8", U16 => "u16", U32 => "u32", U64 => "u64",
            U128 => "u128", Usize => "usize",
            ChuaGhim => "{số nguyên}",
        }
    }

    pub fn tu_ten(s: &str) -> Option<KieuNguyen> {
        use KieuNguyen::*;
        Some(match s {
            "i8" => I8, "i16" => I16, "i32" => I32, "i64" => I64,
            "i128" => I128, "isize" => Isize,
            "u8" => U8, "u16" => U16, "u32" => U32, "u64" => U64,
            "u128" => U128, "usize" => Usize,
            _ => return None,
        })
    }

    /// Hai kiểu nguyên này chắc chắn không dùng thay nhau được?
    ///
    /// `ChuaGhim` hợp với mọi bề rộng (giữ tính linh hoạt của literal, tránh báo
    /// oan). Hai bề rộng cụ thể khác nhau thì lệch — đó chính là luật Rust.
    pub fn lech(self, khac: KieuNguyen) -> bool {
        use KieuNguyen::ChuaGhim;
        !matches!(self, ChuaGhim) && !matches!(khac, ChuaGhim) && self != khac
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KieuThuc {
    F32,
    F64,
    /// Literal số thực chưa ghim.
    ChuaGhim,
}

impl KieuThuc {
    pub fn ten(self) -> &'static str {
        match self {
            KieuThuc::F32 => "f32",
            KieuThuc::F64 => "f64",
            KieuThuc::ChuaGhim => "{số thực}",
        }
    }
    pub fn lech(self, khac: KieuThuc) -> bool {
        use KieuThuc::ChuaGhim;
        !matches!(self, ChuaGhim) && !matches!(khac, ChuaGhim) && self != khac
    }
}

/// Kiểu ở mức đủ dùng để bắt lỗi thường gặp.
#[derive(Debug, Clone, PartialEq)]
pub enum T {
    SoNguyen(KieuNguyen),
    SoThuc(KieuThuc),
    Bool,
    KyTu,
    Chuoi,
    Rong,
    Vec(Box<T>),
    Struct(String),
    Enum(String),
    Tham(Box<T>),
    /// Chưa suy ra được, nhưng hợp lệ. Không bao giờ sinh lỗi.
    Mo,
    /// **Bộ kiểm không mô hình hoá được chỗ này.**
    ///
    /// Khác `Mo` ở chỗ: `Mo` nghĩa là "chưa cần biết", còn `ChuaBiet` nghĩa là
    /// "Byte không hiểu". Gộp hai thứ này lại chính là lỗ hổng khiến hàng chục
    /// chương trình sai lọt qua: mọi giá trị đi qua một chỗ không mô hình hoá
    /// được đều được *giặt sạch* khỏi mọi phép kiểm hạ nguồn.
    ///
    /// Điểm kiểm nào gặp `ChuaBiet` phải phát `ChuaHoTro`, không được im lặng.
    ChuaBiet(&'static str),
}

impl T {
    pub fn hien_thi(&self) -> String {
        match self {
            T::SoNguyen(k) => k.ten().into(),
            T::SoThuc(k) => k.ten().into(),
            T::Bool => "bool".into(),
            T::KyTu => "char".into(),
            T::Chuoi => "String".into(),
            T::Rong => "()".into(),
            T::Vec(t) => format!("Vec<{}>", t.hien_thi()),
            T::Struct(n) | T::Enum(n) => n.clone(),
            T::Tham(t) => format!("&{}", t.hien_thi()),
            T::Mo => "?".into(),
            T::ChuaBiet(_) => "?".into(),
        }
    }

    /// Byte không mô hình hoá được kiểu này?
    pub fn chua_biet(&self) -> Option<&'static str> {
        match self {
            T::ChuaBiet(ly_do) => Some(ly_do),
            T::Vec(x) | T::Tham(x) => x.chua_biet(),
            _ => None,
        }
    }

    /// Hai kiểu này chắc chắn KHÔNG tương thích?
    ///
    /// Cố tình bảo thủ: chỉ trả `true` khi chắc chắn. `Mo` luôn tương thích.
    pub fn chac_chan_lech(&self, khac: &T) -> bool {
        use T::*;
        match (self, khac) {
            // `Mo` và `ChuaBiet` không bao giờ sinh lỗi LỆCH KIỂU. `ChuaBiet` được
            // xử lý riêng ở từng điểm kiểm bằng `ChuaHoTro`.
            (Mo, _) | (_, Mo) | (ChuaBiet(_), _) | (_, ChuaBiet(_)) => false,
            // Tham chiếu và giá trị: rustc phân biệt, nhưng ta chỉ bắt khi
            // kiểu bên trong cũng lệch — tránh báo oan chỗ auto-deref.
            (Tham(a), Tham(b)) => a.chac_chan_lech(b),
            // Một bên là tham chiếu, bên kia không: rustc phân biệt, nhưng
            // auto-deref khiến nhiều chỗ vẫn hợp lệ. Chỉ báo khi kiểu BÊN TRONG
            // cũng lệch — tránh báo oan.
            (Tham(a), b) => a.chac_chan_lech(b),
            (a, Tham(b)) => a.chac_chan_lech(b),
            (SoNguyen(a), SoNguyen(b)) => a.lech(*b),
            (SoThuc(a), SoThuc(b)) => a.lech(*b),
            (Bool, Bool) | (KyTu, KyTu) | (Chuoi, Chuoi) | (Rong, Rong) => false,
            (Vec(a), Vec(b)) => a.chac_chan_lech(b),
            (Struct(a), Struct(b)) | (Enum(a), Enum(b)) => a != b,
            // Số nguyên và số thực KHÔNG dùng thay nhau được — đây đúng là bài
            // học "Rust không ép kiểu số ngầm". Ngoại lệ duy nhất: literal chưa
            // ghim, vì `let x: f64 = 5;` thì `5` vẫn có thể là `5.0`... KHÔNG:
            // Rust cũng từ chối ca đó (E0308). Nên không có ngoại lệ nào cả.
            (SoNguyen(_), SoThuc(_)) | (SoThuc(_), SoNguyen(_)) => true,
            _ => true,
        }
    }
}

fn tu_kieu_ast(k: &Kieu) -> T {
    match k {
        Kieu::DuongDan { doan, tham_so, .. } => {
            let ten = doan.last().map(String::as_str).unwrap_or("");
            match ten {
                _ if KieuNguyen::tu_ten(ten).is_some() => {
                    T::SoNguyen(KieuNguyen::tu_ten(ten).unwrap())
                }
                "f32" => T::SoThuc(KieuThuc::F32),
                "f64" => T::SoThuc(KieuThuc::F64),
                "bool" => T::Bool,
                "char" => T::KyTu,
                "String" | "str" => T::Chuoi,
                "Vec" => T::Vec(Box::new(
                    tham_so.first().map(tu_kieu_ast).unwrap_or(T::Mo),
                )),
                "Option" | "Result" => T::Enum(ten.to_string()),
                khac if khac.chars().next().is_some_and(char::is_uppercase) => {
                    T::Struct(khac.to_string())
                }
                _ => T::Mo,
            }
        }
        Kieu::ThamChieu { ben_trong, .. } => T::Tham(Box::new(tu_kieu_ast(ben_trong))),
        Kieu::Tuple { phan_tu, .. } if phan_tu.is_empty() => T::Rong,
        _ => T::Mo,
    }
}

struct ChuKy {
    tham_so: Vec<T>,
    tra_ve: T,
    span: Span,
}

pub struct BoKiemKieu<'a> {
    ham: HashMap<String, ChuKy>,
    /// tên enum -> danh sách biến thể
    enum_bien_the: HashMap<String, Vec<String>>,
    /// tên biến thể -> tên enum
    thuoc_enum: HashMap<String, String>,
    struct_co: Vec<String>,
    /// tên struct -> (tên trường -> kiểu)
    truong_struct: HashMap<String, HashMap<String, T>>,
    bien: Vec<HashMap<String, T>>,
    diags: &'a mut Diagnostics,
}

impl<'a> BoKiemKieu<'a> {
    fn moi(diags: &'a mut Diagnostics) -> Self {
        Self {
            ham: HashMap::new(),
            enum_bien_the: HashMap::new(),
            thuoc_enum: HashMap::new(),
            struct_co: Vec::new(),
            truong_struct: HashMap::new(),
            bien: vec![HashMap::new()],
            diags,
        }
    }

    fn nap(&mut self, ct: &ChuongTrinh) {
        for m in &ct.muc {
            match m {
                Muc::Ham(h) => {
                    self.ham.insert(
                        h.ten.clone(),
                        ChuKy {
                            tham_so: h.tham_so.iter().map(|t| tu_kieu_ast(&t.kieu)).collect(),
                            tra_ve: h.kieu_tra_ve.as_ref().map(tu_kieu_ast).unwrap_or(T::Rong),
                            span: h.span,
                        },
                    );
                }
                Muc::Enum(e) => {
                    let ds: Vec<String> = e.bien_the.iter().map(|b| b.ten.clone()).collect();
                    for b in &ds {
                        self.thuoc_enum.insert(b.clone(), e.ten.clone());
                    }
                    self.enum_bien_the.insert(e.ten.clone(), ds);
                }
                Muc::Struct(s) => {
                    self.struct_co.push(s.ten.clone());
                    if let ThanStruct::TheoTen(cac) = &s.than {
                        let m: HashMap<String, T> = cac
                            .iter()
                            .map(|(ten, k, _)| (ten.clone(), tu_kieu_ast(k)))
                            .collect();
                        self.truong_struct.insert(s.ten.clone(), m);
                    }
                }
                _ => {}
            }
        }
        // Option/Result dựng sẵn
        self.enum_bien_the.insert("Option".into(), vec!["Some".into(), "None".into()]);
        self.enum_bien_the.insert("Result".into(), vec!["Ok".into(), "Err".into()]);
        for (b, e) in [("Some", "Option"), ("None", "Option"), ("Ok", "Result"), ("Err", "Result")] {
            self.thuoc_enum.insert(b.into(), e.into());
        }
    }

    /// `tu_kieu_ast` không biết `Mau` là enum hay struct — chỉ thấy chữ hoa.
    /// Sau khi nạp xong mới phân biệt được, nên phải chuẩn hoá lại.
    /// Bỏ bước này thì `match` trên enum mất luôn kiểm tính vét cạn.
    fn chuan_hoa(&self, t: T) -> T {
        match t {
            T::Struct(n) if self.enum_bien_the.contains_key(&n) => T::Enum(n),
            T::Vec(x) => T::Vec(Box::new(self.chuan_hoa(*x))),
            T::Tham(x) => T::Tham(Box::new(self.chuan_hoa(*x))),
            khac => khac,
        }
    }

    fn vao(&mut self) { self.bien.push(HashMap::new()); }
    fn ra(&mut self) { self.bien.pop(); }
    fn dat(&mut self, ten: &str, t: T) {
        self.bien.last_mut().unwrap().insert(ten.to_string(), t);
    }
    fn tra(&self, ten: &str) -> T {
        self.bien.iter().rev().find_map(|p| p.get(ten).cloned()).unwrap_or(T::Mo)
    }

    // ── Suy kiểu ────────────────────────────────────────────────────────────

    fn kieu_cua(&mut self, bt: &BieuThuc) -> T {
        match bt {
            BieuThuc::HangSo { gia_tri, .. } => match gia_tri {
                HangSo::SoNguyen(_) => T::SoNguyen(KieuNguyen::ChuaGhim),
                HangSo::SoThuc(_) => T::SoThuc(KieuThuc::ChuaGhim),
                HangSo::DungSai(_) => T::Bool,
                HangSo::KyTu(_) => T::KyTu,
                HangSo::Chuoi(_) => T::Chuoi,
            },
            BieuThuc::DuongDan { doan, .. } => {
                if doan.len() == 1 {
                    let t = self.tra(&doan[0]);
                    if t != T::Mo {
                        return t;
                    }
                    if let Some(e) = self.thuoc_enum.get(&doan[0]) {
                        return T::Enum(e.clone());
                    }
                    T::Mo
                } else {
                    let cuoi = doan.last().unwrap();
                    match self.thuoc_enum.get(cuoi) {
                        Some(e) => T::Enum(e.clone()),
                        None => T::Mo,
                    }
                }
            }
            BieuThuc::HaiNgoi { toan_tu, trai, phai, span } => {
                use ToanTuHai::*;
                let a = self.kieu_cua(trai);
                let b = self.kieu_cua(phai);
                match toan_tu {
                    Va | Hoac => T::Bool,
                    Bang | KhacBang | NhoHon | LonHon | NhoBang | LonBang => {
                        // Rust có `impl Mul<i64> for &i64` nhưng KHÔNG có
                        // `PartialOrd<i64> for &i64`. Nên số học tự bỏ tham chiếu
                        // được, còn so sánh thì không — `v.iter().filter(|x| x > 4)`
                        // là E0308 thật, và người mới vấp chỗ này rất nhiều.
                        let lech_tc = matches!(a, T::Tham(_)) != matches!(b, T::Tham(_))
                            && !matches!(a, T::Mo | T::ChuaBiet(_))
                            && !matches!(b, T::Mo | T::ChuaBiet(_));
                        if lech_tc {
                            self.bao_so_sanh_tham_chieu(*span, &a, &b);
                        }
                        T::Bool
                    }
                    _ => {
                        let (a, b) = (bo_tham_chieu(&a), bo_tham_chieu(&b));
                        if a == T::Mo { b } else { a }
                    }
                }
            }
            BieuThuc::MotNgoi { toan_tu, toan_hang, .. } => match toan_tu {
                ToanTuMot::Phu => T::Bool,
                ToanTuMot::Am => self.kieu_cua(toan_hang),
            },
            BieuThuc::Muon { gia_tri, .. } => T::Tham(Box::new(self.kieu_cua(gia_tri))),
            BieuThuc::GiaiTham { gia_tri, .. } => match self.kieu_cua(gia_tri) {
                T::Tham(t) => *t,
                khac => khac,
            },
            BieuThuc::Mang { phan_tu, .. } => T::Vec(Box::new(
                phan_tu.first().map(|e| self.kieu_cua(e)).unwrap_or(T::Mo),
            )),
            BieuThuc::Macro { ten, doi_so, .. } => match ten.as_str() {
                "format" => T::Chuoi,
                "vec" => T::Vec(Box::new(
                    doi_so.first().map(|e| self.kieu_cua(e)).unwrap_or(T::Mo),
                )),
                _ => T::Rong,
            },
            BieuThuc::KhoiTaoStruct { duong_dan, .. } => {
                let ten = duong_dan.last().cloned().unwrap_or_default();
                if self.struct_co.iter().any(|s| *s == ten) {
                    T::Struct(ten)
                } else {
                    T::Mo
                }
            }
            BieuThuc::GoiHam { ham, doi_so, span } => {
                if let BieuThuc::DuongDan { doan, .. } = &**ham {
                    let cuoi = doan.last().cloned().unwrap_or_default();
                    if let Some(e) = self.thuoc_enum.get(&cuoi).cloned() {
                        for a in doi_so { self.kieu_cua(a); }
                        return T::Enum(e);
                    }
                    if doan.len() >= 2 {
                        let k = &doan[doan.len() - 2];
                        if k == "String" { for a in doi_so { self.kieu_cua(a); } return T::Chuoi; }
                        if k == "Vec" { return T::Vec(Box::new(T::Mo)); }
                    }
                    if let Some(ck) = self.ham.get(&cuoi) {
                        let mong: Vec<T> = ck.tham_so.clone();
                        let tra = ck.tra_ve.clone();
                        let ck_span = ck.span;
                        for (i, a) in doi_so.iter().enumerate() {
                            let thuc = self.kieu_cua(a);
                            if let Some(ly_do) = thuc.chua_biet() {
                                self.bao_chua_biet(a.span(), ly_do);
                            } else if let Some(m) = mong.get(i) {
                                // Với ĐỐI SỐ hàm, `&T` và `T` là hai kiểu khác
                                // nhau — auto-deref chỉ áp dụng cho receiver của
                                // phương thức, không áp dụng ở đây.
                                let lech_tham_chieu = matches!(
                                    (m, &thuc),
                                    (T::Tham(_), t) | (t, T::Tham(_)) if !matches!(t, T::Tham(_) | T::Mo)
                                );
                                if lech_tham_chieu || m.chac_chan_lech(&thuc) {
                                    self.bao_lech_kieu(
                                        a.span(), ck_span, &cuoi, i, m, &thuc, *span,
                                    );
                                }
                            }
                        }
                        return tra;
                    }
                }
                for a in doi_so { self.kieu_cua(a); }
                T::Mo
            }
            BieuThuc::GoiPhuongThuc { doi_tuong, ten, doi_so, .. } => {
                let chu = self.kieu_cua(doi_tuong);
                self.kieu_phuong_thuc(&chu, ten, doi_so)
            }
            BieuThuc::TruyCapTruong { doi_tuong, ten, .. } => {
                let chu = self.kieu_cua(doi_tuong);
                let ten_struct = match &chu {
                    T::Struct(n) => Some(n.clone()),
                    T::Tham(b) => match &**b {
                        T::Struct(n) => Some(n.clone()),
                        _ => None,
                    },
                    _ => None,
                };
                match ten_struct.and_then(|n| self.truong_struct.get(&n).and_then(|m| m.get(ten).cloned())) {
                    Some(t) => self.chuan_hoa(t),
                    None => T::Mo,
                }
            }
            BieuThuc::Khoi(k) => self.khoi(k),
            BieuThuc::Neu { dieu_kien, than, nguoc_lai, .. } => {
                self.kieu_cua(dieu_kien);
                let a = self.khoi(than);
                match nguoc_lai {
                    Some(nl) => {
                        let b = self.kieu_cua(nl);
                        if a == T::Mo { b } else { a }
                    }
                    None => T::Rong,
                }
            }
            BieuThuc::KhopMau { .. } => self.khop_mau(bt),
            BieuThuc::Tuple { phan_tu, .. } if phan_tu.is_empty() => T::Rong,
            BieuThuc::Ep { kieu, gia_tri, .. } => {
                self.kieu_cua(gia_tri);
                tu_kieu_ast(kieu)
            }
            khac => {
                self.duyet_con(khac);
                T::Mo
            }
        }
    }

    /// Duyệt các nút không cần suy kiểu, chỉ để kiểm tra bên trong.
    fn duyet_con(&mut self, bt: &BieuThuc) {
        match bt {
            BieuThuc::Gan { dich, gia_tri, .. } => {
                self.kieu_cua(dich);
                self.kieu_cua(gia_tri);
            }
            BieuThuc::Lap { than, .. } => { self.khoi(than); }
            BieuThuc::Trong { dieu_kien, than, .. } => {
                self.kieu_cua(dieu_kien);
                self.khoi(than);
            }
            BieuThuc::Cho { mau, day, than, .. } => {
                self.kieu_cua(day);
                self.vao();
                let mut ten = Vec::new();
                mau.ten_rang_buoc(&mut ten);
                for (t, _, _) in ten { self.dat(&t, T::Mo); }
                self.khoi(than);
                self.ra();
            }
            BieuThuc::BeQuan { tham_so, than, .. } => {
                self.vao();
                for (m, _) in tham_so {
                    let mut ten = Vec::new();
                    m.ten_rang_buoc(&mut ten);
                    for (t, _, _) in ten { self.dat(&t, T::Mo); }
                }
                self.kieu_cua(than);
                self.ra();
            }
            BieuThuc::TraVe { gia_tri, .. } | BieuThuc::Thoat { gia_tri, .. } => {
                if let Some(e) = gia_tri { self.kieu_cua(e); }
            }
            BieuThuc::ChiSo { doi_tuong, chi_so, .. } => {
                self.kieu_cua(doi_tuong);
                self.kieu_cua(chi_so);
            }
            BieuThuc::LanTruyenLoi { gia_tri, .. } => {
                self.kieu_cua(gia_tri);
            }
            BieuThuc::Tuple { phan_tu, .. } => {
                for e in phan_tu { self.kieu_cua(e); }
            }
            BieuThuc::Dai { tu, den, .. } => {
                if let Some(e) = tu { self.kieu_cua(e); }
                if let Some(e) = den { self.kieu_cua(e); }
            }
            _ => {}
        }
    }

    fn khoi(&mut self, k: &Khoi) -> T {
        self.vao();
        for cl in &k.cau_lenh {
            match cl {
                CauLenh::Let { mau, kieu, gia_tri, span } => {
                    let t_gt = gia_tri.as_ref().map(|e| self.kieu_cua(e)).unwrap_or(T::Mo);
                    // Chú thích kiểu KHÔNG được ghi đè im lặng kiểu suy ra.
                    //
                    // Bản trước làm đúng thế, và đó là lỗ hổng gốc: kiểu sai được
                    // ghi vào bảng biến rồi lan ra mọi suy luận sau. `tyck` không
                    // chỉ bỏ sót — nó TIN một điều sai rồi dùng điều sai đó để
                    // xác nhận tiếp.
                    let t = match kieu {
                        Some(k) => {
                            let khai = self.chuan_hoa(tu_kieu_ast(k));
                            let sp = gia_tri.as_ref().map(|e| e.span()).unwrap_or(*span);
                            if let Some(ly_do) = t_gt.chua_biet().or_else(|| khai.chua_biet()) {
                                self.bao_chua_biet(sp, ly_do);
                            } else if lech_trong_let(&khai, &t_gt) {
                                self.bao_let_lech(sp, k.span(), &khai, &t_gt);
                            }
                            khai
                        }
                        None => t_gt,
                    };
                    let mut ten = Vec::new();
                    mau.ten_rang_buoc(&mut ten);
                    for (n, _, _) in ten { self.dat(&n, t.clone()); }
                }
                CauLenh::BieuThuc { bt, .. } => { self.kieu_cua(bt); }
                CauLenh::Muc(_) => {}
            }
        }
        let t = k.gia_tri_cuoi.as_ref().map(|e| self.kieu_cua(e)).unwrap_or(T::Rong);
        self.ra();
        t
    }

    // ── match: kiểu + tính vét cạn ──────────────────────────────────────────

    fn khop_mau(&mut self, bt: &BieuThuc) -> T {
        let BieuThuc::KhopMau { gia_tri, nhanh, span } = bt else { return T::Mo };
        let t_gt = self.kieu_cua(gia_tri);

        let mut kieu_nhanh = T::Mo;
        let mut co_bao_quat = false;
        let mut da_phu: Vec<String> = Vec::new();

        for n in nhanh {
            match &n.mau {
                Mau::BoQua { .. } | Mau::Ten { .. } => co_bao_quat = true,
                Mau::BienThe { duong_dan, .. } => {
                    if let Some(c) = duong_dan.last() { da_phu.push(c.clone()); }
                }
                Mau::Hoac { nhanh: cac, .. } => {
                    for m in cac {
                        if let Mau::BienThe { duong_dan, .. } = m {
                            if let Some(c) = duong_dan.last() { da_phu.push(c.clone()); }
                        }
                    }
                }
                _ => {}
            }
            self.vao();
            let mut ten = Vec::new();
            n.mau.ten_rang_buoc(&mut ten);
            for (t, _, _) in ten { self.dat(&t, T::Mo); }
            if let Some(dk) = &n.dieu_kien { self.kieu_cua(dk); }
            let t = self.kieu_cua(&n.than);
            self.ra();
            if kieu_nhanh == T::Mo { kieu_nhanh = t; }
        }

        // Vét cạn: chỉ kiểm khi biết chắc đang match trên enum nào và không có
        // nhánh bao quát. Nhánh có `if` không tính là phủ (rustc cũng vậy).
        if !co_bao_quat {
            if let T::Enum(ten_enum) = &t_gt {
                if let Some(tat_ca) = self.enum_bien_the.get(ten_enum).cloned() {
                    let phu_khong_dieu_kien: Vec<String> = nhanh
                        .iter()
                        .filter(|n| n.dieu_kien.is_none())
                        .filter_map(|n| match &n.mau {
                            Mau::BienThe { duong_dan, .. } => duong_dan.last().cloned(),
                            _ => None,
                        })
                        .collect();
                    let thieu: Vec<String> = tat_ca
                        .iter()
                        .filter(|v| !phu_khong_dieu_kien.contains(v))
                        .cloned()
                        .collect();
                    if !thieu.is_empty() {
                        let ds = thieu.iter().map(|v| format!("`{ten_enum}::{v}`"))
                            .collect::<Vec<_>>().join(", ");
                        self.diags.push(
                            Diagnostic::loi("BR0300", format!("`match` chưa phủ hết mọi khả năng của `{ten_enum}`"))
                                .tai(*span, format!("còn thiếu: {ds}"))
                                .vi_sao("Đây chính là điều làm `match` an toàn hơn `switch` của các ngôn ngữ khác: compiler bắt buộc bạn xử lý MỌI trường hợp. Nhờ vậy khi ai đó thêm một biến thể mới vào enum, mọi chỗ quên xử lý sẽ báo lỗi ngay lúc biên dịch chứ không âm thầm chạy sai.")
                                .sua(format!("thêm nhánh cho {ds}"))
                                .sua("hoặc thêm `_ => ...` để bắt các trường hợp còn lại")
                                .khai_niem("match"),
                        );
                    }
                }
            }
        }
        kieu_nhanh
    }

    /// Kiểu trả về của một lời gọi phương thức, có tính tới closure truyền vào.
    ///
    /// Không suy được kiểu qua `map`/`filter` thì mọi chuỗi iterator — thứ người
    /// học FP dùng liên tục — đều rơi vào `ChuaHoTro`, và công cụ thành vô dụng.
    /// Đây là chỗ đáng bỏ công mô hình hoá cho tử tế.
    fn kieu_phuong_thuc(&mut self, chu: &T, ten: &str, doi_so: &[BieuThuc]) -> T {
        let phan_tu = match chu {
            T::Vec(x) => (**x).clone(),
            _ => T::Mo,
        };

        match ten {
            // `iter()` mượn từng phần tử: item là `&T`, KHÔNG phải `T`.
            // Chi tiết này quan trọng: `v.iter().filter(|x| x > 4)` là E0308 thật,
            // vì `&i64` không so sánh trực tiếp được với số nguyên.
            "iter" => {
                for a in doi_so { self.kieu_cua(a); }
                T::Vec(Box::new(T::Tham(Box::new(phan_tu))))
            }
            "into_iter" => {
                for a in doi_so { self.kieu_cua(a); }
                chu.clone()
            }
            "map" => {
                let ra = self.kieu_than_be_quan(doi_so.first(), &phan_tu);
                T::Vec(Box::new(ra))
            }
            "filter" => {
                // Closure của `filter` nhận thêm một tầng tham chiếu nữa.
                self.kieu_than_be_quan(doi_so.first(), &T::Tham(Box::new(phan_tu.clone())));
                T::Vec(Box::new(phan_tu))
            }
            "collect" => chu.clone(),
            "sum" | "fold" => {
                for a in doi_so { self.kieu_cua(a); }
                bo_tham_chieu(&phan_tu)
            }
            "rev" => {
                for a in doi_so { self.kieu_cua(a); }
                chu.clone()
            }
            _ => {
                for a in doi_so { self.kieu_cua(a); }
                kieu_tra_ve_phuong_thuc(chu, ten)
            }
        }
    }

    /// Kiểu thân closure, với tham số đầu tiên gán kiểu `kieu_tham`.
    fn kieu_than_be_quan(&mut self, bt: Option<&BieuThuc>, kieu_tham: &T) -> T {
        let Some(BieuThuc::BeQuan { tham_so, than, .. }) = bt else {
            if let Some(e) = bt { self.kieu_cua(e); }
            return T::Mo;
        };
        self.vao();
        if let Some((m, _)) = tham_so.first() {
            let mut ten = Vec::new();
            m.ten_rang_buoc(&mut ten);
            for (n, _, _) in ten { self.dat(&n, kieu_tham.clone()); }
        }
        let ra = self.kieu_cua(than);
        self.ra();
        ra
    }

    fn bao_let_lech(&mut self, span_gt: Span, span_kieu: Span, khai: &T, thuc: &T) {
        self.diags.push(
            Diagnostic::loi(
                "BR0303",
                format!("khai báo kiểu {} nhưng giá trị là {}", khai.hien_thi(), thuc.hien_thi()),
            )
            .nhan(Label::chinh(span_gt, format!("giá trị này là {}", thuc.hien_thi())))
            .nhan(Label::phu(span_kieu, format!("nhưng ở đây ghi {}", khai.hien_thi())))
            .vi_sao("Rust KHÔNG tự chuyển đổi kiểu ngầm. Khác Python và JavaScript, `7 / 2` giữa hai số nguyên cho ra số nguyên `3`, và gán nó vào `f64` là lỗi chứ không âm thầm thành `3.0`. Chính luật này chặn cả một lớp lỗi làm tròn khó tìm.")
            .sua("sửa chú thích kiểu cho khớp giá trị")
            .sua("hoặc ép kiểu tường minh, ví dụ `a as f64 / b as f64`")
            .khai_niem("kiểu dữ liệu"),
        );
    }

    fn bao_so_sanh_tham_chieu(&mut self, span: Span, a: &T, b: &T) {
        let (tc, gt) = if matches!(a, T::Tham(_)) { (a, b) } else { (b, a) };
        self.diags.push(
            Diagnostic::loi("BR0304", format!("không so sánh trực tiếp {} với {}", tc.hien_thi(), gt.hien_thi()))
                .tai(span, "phép so sánh ở đây")
                .vi_sao("Rust cho phép `&i64 * 2` (có sẵn phép nhân cho tham chiếu) nhưng KHÔNG cho `&i64 > 2`. Đây là chỗ `v.iter()` hay làm người mới vấp: `iter()` mượn từng phần tử nên bạn nhận được `&T`, không phải `T`.")
                .sua("thêm dấu `*` để lấy giá trị ra: `*x > 4`")
                .sua("hoặc dùng `into_iter()` nếu bạn muốn lấy hẳn phần tử")
                .khai_niem("tham chiếu"),
        );
    }

    /// Byte không mô hình hoá được chỗ này — nói thẳng thay vì im lặng.
    fn bao_chua_biet(&mut self, span: Span, ly_do: &'static str) {
        self.diags.push(
            Diagnostic::chua_ho_tro(
                "BR0310",
                "tyck-chua-mo-hinh-hoa",
                "Byte chưa kiểm được kiểu ở chỗ này",
            )
            .tai(span, ly_do)
            .vi_sao("Bộ kiểm kiểu của Byte cố tình hẹp, để mọi thông báo lỗi đều dễ hiểu. Gặp cấu trúc nằm ngoài phạm vi, Byte nói thẳng là chưa kiểm được — thay vì im lặng cho qua rồi để bạn tin nhầm là code đúng.")
            .sua("Mở bản Desktop và bấm “Đối chiếu với cargo” để có câu trả lời chính xác")
            .khai_niem("kiểu dữ liệu"),
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn bao_lech_kieu(
        &mut self, span_dt: Span, span_ham: Span, ten_ham: &str,
        vi_tri: usize, mong: &T, thuc: &T, _span: Span,
    ) {
        self.diags.push(
            Diagnostic::loi("BR0301", format!("đối số thứ {} của `{ten_ham}` sai kiểu", vi_tri + 1))
                .nhan(Label::chinh(span_dt, format!("đây là {}, nhưng cần {}", thuc.hien_thi(), mong.hien_thi())))
                .nhan(Label::phu(span_ham, format!("`{ten_ham}` khai báo ở đây")))
                .vi_sao("Rust kiểm tra kiểu lúc biên dịch và KHÔNG tự chuyển đổi ngầm — đổi kiểu ngầm là nguồn của rất nhiều lỗi khó tìm trong các ngôn ngữ khác.")
                .khai_niem("kiểu dữ liệu"),
        );
    }
}

/// Bỏ mọi tầng tham chiếu.
fn bo_tham_chieu(t: &T) -> T {
    match t {
        T::Tham(x) => bo_tham_chieu(x),
        khac => khac.clone(),
    }
}

/// So kiểu trong ngữ cảnh `let` — nghiêm ngặt hơn `chac_chan_lech`.
///
/// `chac_chan_lech` có một nhánh nới lỏng `(a, Tham(b)) => a.chac_chan_lech(b)`
/// để tránh báo oan chỗ auto-deref. Nhưng auto-deref **không áp dụng** cho chú
/// thích kiểu của `let`: `let s: String = &t;` là E0308 thật.
fn lech_trong_let(khai: &T, thuc: &T) -> bool {
    let mot_ben_tham_chieu = matches!(khai, T::Tham(_)) != matches!(thuc, T::Tham(_));
    let khong_mo = !matches!(khai, T::Mo | T::ChuaBiet(_))
        && !matches!(thuc, T::Mo | T::ChuaBiet(_));
    (mot_ben_tham_chieu && khong_mo) || khai.chac_chan_lech(thuc)
}

/// Kiểu trả về của các phương thức dựng sẵn.
///
/// Phương thức không có trong bảng trả `ChuaBiet` chứ KHÔNG trả `Mo`: trả `Mo`
/// nghĩa là "kiểu này hợp lệ, chỉ chưa cần biết", và giá trị đó sau đó đi qua
/// mọi phép kiểm mà không bị chặn. Đó chính là cách `v.first().unwrap()` vô hiệu
/// hoá được luật phân biệt `&T` với `T` — luật phòng thủ tốt nhất của module này.
fn kieu_tra_ve_phuong_thuc(chu: &T, ten: &str) -> T {
    use KieuNguyen::Usize;
    match ten {
        // `len()` trả `usize`, KHÔNG phải "một số nguyên nào đó". Dùng nó như
        // `i64` là lỗi E0308 mà người mới gặp rất sớm.
        "len" | "count" => T::SoNguyen(Usize),
        "is_empty" | "contains" | "starts_with" | "is_some" | "is_none" | "is_ok"
        | "is_err" => T::Bool,
        "to_string" | "to_uppercase" | "to_lowercase" | "trim" => T::Chuoi,
        "push_str" | "push" | "sort" | "reverse" => T::Rong,
        "sqrt" | "round" => T::SoThuc(KieuThuc::F64),
        "clone" => chu.clone(),
        "abs" | "min" | "max" => chu.clone(),
        "pow" => chu.clone(),
        "sum" => match chu {
            T::Vec(t) => (**t).clone(),
            khac => khac.clone(),
        },
        "chars" => T::Vec(Box::new(T::KyTu)),
        "split" => T::Vec(Box::new(T::Chuoi)),
        // `first`/`last`/`get` trả `Option<&T>`; `unwrap` trên đó cho `&T`.
        // Chưa mô hình hoá được `Option` nên phải nói thẳng là chưa biết.
        _ => T::ChuaBiet("phương thức chưa có trong bảng kiểu"),
    }
}

/// Kiểm kiểu toàn chương trình.
pub fn kiem_tra(ct: &ChuongTrinh, diags: &mut Diagnostics) {
    let mut bk = BoKiemKieu::moi(diags);
    bk.nap(ct);

    for m in &ct.muc {
        let (cac_ham, kieu_self): (Vec<&Ham>, Option<T>) = match m {
            Muc::Ham(h) => (vec![h], None),
            Muc::Impl(i) => {
                let ten = match &i.kieu {
                    Kieu::DuongDan { doan, .. } => doan.last().cloned().unwrap_or_default(),
                    khac => khac.hien_thi(),
                };
                (i.ham.iter().collect(), Some(bk.chuan_hoa(T::Struct(ten))))
            }
            _ => continue,
        };
        for h in cac_ham {
            bk.vao();
            for ts in &h.tham_so {
                // `self` mang kiểu của khối `impl`, không phải `Self` trừu tượng.
                let t = if matches!(&ts.mau, Mau::Ten { ten, .. } if ten == "self") {
                    match (&kieu_self, &ts.kieu) {
                        (Some(k), Kieu::ThamChieu { .. }) => T::Tham(Box::new(k.clone())),
                        (Some(k), _) => k.clone(),
                        (None, _) => T::Mo,
                    }
                } else {
                    bk.chuan_hoa(tu_kieu_ast(&ts.kieu))
                };
                let mut ten = Vec::new();
                ts.mau.ten_rang_buoc(&mut ten);
                for (n, _, _) in ten { bk.dat(&n, t.clone()); }
            }
            let t_than = bk.khoi(&h.than);
            bk.ra();

            // Kiểu của biểu thức cuối phải khớp kiểu trả về khai báo.
            if let Some(kb) = &h.kieu_tra_ve {
                let mong = bk.chuan_hoa(tu_kieu_ast(kb));
                if let Some(ly_do) = t_than.chua_biet() {
                    let sp = h.than.gia_tri_cuoi.as_ref().map(|e| e.span()).unwrap_or(h.than.span);
                    bk.bao_chua_biet(sp, ly_do);
                } else if mong.chac_chan_lech(&t_than) {
                    let span_than = h.than.gia_tri_cuoi.as_ref().map(|e| e.span()).unwrap_or(h.than.span);
                    bk.diags.push(
                        Diagnostic::loi("BR0302", format!("`{}` khai báo trả về {} nhưng thân hàm cho ra {}",
                                                          h.ten, mong.hien_thi(), t_than.hien_thi()))
                            .nhan(Label::chinh(span_than, format!("giá trị này là {}", t_than.hien_thi())))
                            .nhan(Label::phu(kb.span(), format!("nhưng ở đây khai báo trả về {}", mong.hien_thi())))
                            .vi_sao("Chữ ký hàm là một lời hứa với người gọi. Rust kiểm tra lời hứa đó lúc biên dịch, nên người gọi không bao giờ nhận về thứ khác với thứ được ghi trong chữ ký.")
                            .sua("sửa kiểu trả về cho khớp, hoặc sửa giá trị cuối của hàm")
                            .khai_niem("hàm"),
                    );
                }
            }
        }
    }
}
