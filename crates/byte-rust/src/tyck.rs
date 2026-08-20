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
    /// Bộ lặp sinh ra giá trị kiểu `T` — kết quả của `.iter()`, `.map()`…
    ///
    /// Tách khỏi `Vec` là bắt buộc, không phải tinh chỉnh: `v.sum()` là lỗi
    /// còn `v.iter().sum()` thì đúng, và nếu cả hai cùng mang kiểu `Vec<i64>`
    /// thì không có cách nào phân biệt. Đây cũng đúng là chỗ người mới vấp
    /// nhiều nhất khi chuyển từ Python sang.
    Lap(Box<T>),
    Tuple(Vec<T>),
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
            T::Lap(t) => format!("bộ lặp sinh {}", t.hien_thi()),
            T::Tuple(cac) => format!("({})", cac.iter().map(|x| x.hien_thi()).collect::<Vec<_>>().join(", ")),
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
            T::Vec(x) | T::Lap(x) | T::Tham(x) => x.chua_biet(),
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
            (Vec(a), Vec(b)) | (Lap(a), Lap(b)) => a.chac_chan_lech(b),
            // Một `Vec` và một bộ lặp không dùng thay nhau được.
            (Vec(_), Lap(_)) | (Lap(_), Vec(_)) => true,
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
        Kieu::Tuple { phan_tu, .. } => T::Tuple(phan_tu.iter().map(tu_kieu_ast).collect()),
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
    /// tên enum -> [(biến thể, tên kiểu của từng payload)] — cho kiểm vét cạn lồng nhau
    payload: HashMap<String, Vec<(String, Vec<String>)>>,
    /// tên biến thể -> tên enum
    thuoc_enum: HashMap<String, String>,
    struct_co: Vec<String>,
    /// Tên mọi phương thức người học tự viết trong khối `impl`. Bộ kiểm không
    /// được phán "phương thức này không tồn tại" với những tên nằm ở đây.
    phuong_thuc_tu_viet: std::collections::HashSet<String>,
    /// tên struct -> (tên trường -> kiểu)
    truong_struct: HashMap<String, HashMap<String, T>>,
    bien: Vec<HashMap<String, T>>,
    /// Kiểu trả về khai báo của hàm đang duyệt, cùng span của chú thích ấy.
    /// `return` ở giữa thân hàm phải đối chiếu với nó — biểu thức cuối thân
    /// hàm đã được kiểm ở `kiem_tra`, nhưng `return` sớm thì chưa từng.
    tra_ve_ham: Option<(T, Span)>,
    diags: &'a mut Diagnostics,
}

impl<'a> BoKiemKieu<'a> {
    fn moi(diags: &'a mut Diagnostics) -> Self {
        Self {
            ham: HashMap::new(),
            enum_bien_the: HashMap::new(),
            payload: HashMap::new(),
            thuoc_enum: HashMap::new(),
            struct_co: Vec::new(),
            phuong_thuc_tu_viet: std::collections::HashSet::new(),
            tra_ve_ham: None,
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
                Muc::Impl(i) => {
                    for h in &i.ham {
                        self.phuong_thuc_tu_viet.insert(h.ten.clone());
                    }
                }
                Muc::Enum(e) => {
                    self.payload.insert(
                        e.ten.clone(),
                        e.bien_the
                            .iter()
                            .map(|b| {
                                let kieu_con = match &b.than {
                                    ThanStruct::TheoViTri(cac) => cac
                                        .iter()
                                        .map(|(k, _)| match k {
                                            Kieu::DuongDan { doan, .. } => {
                                                doan.last().cloned().unwrap_or_default()
                                            }
                                            khac => khac.hien_thi(),
                                        })
                                        .collect(),
                                    _ => Vec::new(),
                                };
                                (b.ten.clone(), kieu_con)
                            })
                            .collect(),
                    );
                    let ds: Vec<String> = e.bien_the.iter().map(|b| b.ten.clone()).collect();
                    for b in &ds {
                        self.thuoc_enum.insert(b.clone(), e.ten.clone());
                    }
                    self.enum_bien_the.insert(e.ten.clone(), ds);
                }
                Muc::Struct(s) => {
                    self.struct_co.push(s.ten.clone());
                    // Struct dạng tuple `struct Email(String);` cũng phải nạp:
                    // bỏ qua nó nghĩa là `e.1` trên một struct 1 trường lọt qua.
                    if let ThanStruct::TheoViTri(cac) = &s.than {
                        let m: HashMap<String, T> = cac
                            .iter()
                            .enumerate()
                            .map(|(i, (k, _))| (i.to_string(), tu_kieu_ast(k)))
                            .collect();
                        self.truong_struct.insert(s.ten.clone(), m);
                    }
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
        self.payload.insert("Option".into(), vec![("Some".into(), vec!["_".into()]), ("None".into(), vec![])]);
        self.payload.insert("Result".into(), vec![("Ok".into(), vec!["_".into()]), ("Err".into(), vec!["_".into()])]);
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
            T::Lap(x) => T::Lap(Box::new(self.chuan_hoa(*x))),
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
                    let cuoi = doan.last().unwrap().clone();
                    let truoc = doan[doan.len() - 2].clone();
                    match self.thuoc_enum.get(&cuoi).cloned() {
                        Some(e) => {
                            // `Hinh::Tron` khi `Tron(i64)` cần payload: đó là
                            // một hàm dựng chưa gọi, không phải giá trị `Hinh`.
                            if let Some(n) = self.so_payload(&e, &cuoi) {
                                if n > 0 {
                                    self.bao_thieu_payload(bt.span(), &e, &cuoi, n);
                                    return T::ChuaBiet("biến thể enum thiếu tham số");
                                }
                            }
                            T::Enum(e)
                        }
                        None => {
                            // `Mau::Tim` khi `Mau` có mà `Tim` không: gõ sai tên
                            // biến thể — compiler biết chắc, phải nói ra.
                            if self.enum_bien_the.contains_key(&truoc) {
                                self.bao_khong_co_bien_the(bt.span(), &truoc, &cuoi);
                                return T::ChuaBiet("biến thể enum không tồn tại");
                            }
                            T::Mo
                        }
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
                        } else {
                            let (x, y) = (bo_tham_chieu(&a), bo_tham_chieu(&b));
                            if x.chac_chan_lech(&y) {
                                self.bao_toan_hang_lech(*span, toan_tu.ky_hieu(), &x, &y);
                            }
                        }
                        T::Bool
                    }
                    _ => {
                        // Số học tự bỏ tham chiếu (Rust có impl cho &T), nhưng
                        // hai vế vẫn phải CÙNG kiểu. Bản trước chỉ suy kiểu mà
                        // không kiểm, nên `10 + 2.5` và `u32 + i64` đều lọt.
                        let (a, b) = (bo_tham_chieu(&a), bo_tham_chieu(&b));
                        if a.chac_chan_lech(&b) {
                            self.bao_toan_hang_lech(*span, toan_tu.ky_hieu(), &a, &b);
                        }
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
            BieuThuc::Macro { ten, doi_so, .. } => {
                // Đối số macro là biểu thức Rust bình thường và rustc kiểm chúng
                // y như mọi biểu thức khác. Bản trước rơi vào `_ => T::Rong` mà
                // không thăm đối số nào — nên `println!("{}", cong(1))` chỉ bị
                // bắt lúc CHẠY, và nếu nó nằm trong nhánh không bao giờ chạy tới
                // thì không ai bắt. Vì `println!` có mặt gần như mọi dòng mã của
                // người mới, đây là vùng mù lớn nhất của bộ kiểm tĩnh.
                let kieu_dau: Vec<T> = doi_so.iter().map(|e| self.kieu_cua(e)).collect();
                match ten.as_str() {
                    "format" => T::Chuoi,
                    "vec" => T::Vec(Box::new(kieu_dau.first().cloned().unwrap_or(T::Mo))),
                    _ => T::Rong,
                }
            }
            BieuThuc::KhoiTaoStruct { duong_dan, truong, span, .. } => {
                let ten = duong_dan.last().cloned().unwrap_or_default();
                for (_, e) in truong {
                    self.kieu_cua(e);
                }

                // Biến thể enum viết theo cú pháp struct: `Hinh::Tron { r: 5 }`
                // trong khi `Tron` là biến thể dạng tuple.
                if self.thuoc_enum.contains_key(&ten) && !self.struct_co.iter().any(|s| *s == ten) {
                    let en = self.thuoc_enum[&ten].clone();
                    self.bao_khoi_tao_sai_dang(*span, &en, &ten);
                    return T::Enum(en);
                }

                if !self.struct_co.iter().any(|s| *s == ten) {
                    self.bao_khong_co_struct(*span, &ten);
                    return T::ChuaBiet("struct chưa khai báo");
                }

                if let Some(bang) = self.truong_struct.get(&ten).cloned() {
                    for (k, e) in truong {
                        match bang.get(k) {
                            None => self.bao_truong_thua(e.span(), &ten, k, &bang),
                            Some(mong) => {
                                let thuc = self.kieu_cua(e);
                                let mong = self.chuan_hoa(mong.clone());
                                if thuc.chua_biet().is_none() && mong.chac_chan_lech(&thuc) {
                                    self.bao_truong_sai_kieu(e.span(), &ten, k, &mong, &thuc);
                                }
                            }
                        }
                    }
                    let mut khoa: Vec<&String> = bang.keys().collect();
                    khoa.sort();
                    for k in khoa {
                        if !truong.iter().any(|(t, _)| t == k) {
                            self.bao_thieu_truong(*span, &ten, k);
                        }
                    }
                }
                T::Struct(ten)
            }
            BieuThuc::GoiHam { ham, doi_so, span } => {
                if let BieuThuc::DuongDan { doan, .. } = &**ham {
                    let cuoi = doan.last().cloned().unwrap_or_default();
                    if let Some(e) = self.thuoc_enum.get(&cuoi).cloned() {
                        for a in doi_so { self.kieu_cua(a); }
                        if let Some(n) = self.so_payload(&e, &cuoi) {
                            if n != doi_so.len() {
                                self.bao_sai_arity_bien_the(*span, &e, &cuoi, n, doi_so.len());
                                return T::ChuaBiet("biến thể enum sai số tham số");
                            }
                        }
                        return T::Enum(e);
                    }
                    if doan.len() >= 2 && self.enum_bien_the.contains_key(&doan[doan.len() - 2]) {
                        let en = doan[doan.len() - 2].clone();
                        for a in doi_so { self.kieu_cua(a); }
                        self.bao_khong_co_bien_the(*span, &en, &cuoi);
                        return T::ChuaBiet("biến thể enum không tồn tại");
                    }
                    if doan.len() >= 2 {
                        let k = &doan[doan.len() - 2];
                        if k == "String" { for a in doi_so { self.kieu_cua(a); } return T::Chuoi; }
                        if k == "Vec" { return T::Vec(Box::new(T::Mo)); }
                    }
                    // Tên hàm hoàn toàn lạ. Nó có thể nằm trong một nhánh
                    // không bao giờ chạy tới, nhưng rustc vẫn từ chối: phân giải
                    // tên xảy ra lúc biên dịch, không phụ thuộc luồng chạy. Đây
                    // chính là vùng mù mà một bộ kiểm động không bao giờ thấy.
                    if doan.len() == 1
                        && !self.ham.contains_key(&cuoi)
                        && !self.thuoc_enum.contains_key(&cuoi)
                        && !TEN_DUNG_SAN.contains(&cuoi.as_str())
                        && self.tra(&cuoi) == T::Mo
                    {
                        for a in doi_so { self.kieu_cua(a); }
                        self.bao_ham_khong_ton_tai(ham.span(), &cuoi);
                        return T::ChuaBiet("gọi hàm chưa khai báo");
                    }
                    if let Some(ck) = self.ham.get(&cuoi) {
                        // Chuẩn hoá Ở ĐÂY chứ không ở `nap`: lúc `nap` chạy,
                        // bảng enum chưa đầy, nên một hàm khai báo TRƯỚC enum
                        // sẽ giữ nguyên `T::Struct("Mau")` trong khi lời gọi
                        // cho ra `T::Enum("Mau")`. Hai kiểu ấy in ra giống hệt
                        // nhau nên lỗi hiện thành "đây là Mau, nhưng cần Mau".
                        let mong: Vec<T> =
                            ck.tham_so.clone().into_iter().map(|k| self.chuan_hoa(k)).collect();
                        let tra = self.chuan_hoa(ck.tra_ve.clone());
                        let ck_span = ck.span;
                        // Số đối số phải khớp — kiểm TRƯỚC vòng đối chiếu kiểu.
                        // Vòng ấy dùng `mong.get(i)` nên đối số thừa rơi vào
                        // `None` và bị bỏ qua im lặng, còn đối số thiếu thì
                        // chẳng có gì để lặp qua.
                        if doi_so.len() != mong.len() {
                            for a in doi_so { self.kieu_cua(a); }
                            self.bao_sai_so_doi_so(*span, ck_span, &cuoi, mong.len(), doi_so.len());
                            return T::ChuaBiet("gọi hàm sai số đối số");
                        }
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
                match &ten_struct {
                    Some(n) => match self.truong_struct.get(n) {
                        Some(bang) => match bang.get(ten) {
                            Some(t) => {
                                let t = t.clone();
                                self.chuan_hoa(t)
                            }
                            None => {
                                // Đã biết CHẮC đây là struct nào; trường không có
                                // thì đó là lỗi, không phải "chưa suy ra được".
                                let bang = bang.clone();
                                self.bao_truong_khong_co(bt.span(), n, ten, &bang);
                                T::ChuaBiet("trường không tồn tại")
                            }
                        },
                        None => T::Mo,
                    },
                    None => T::Mo,
                }
            }
            BieuThuc::ChiSo { doi_tuong, chi_so, .. } => {
                let t_dt = self.kieu_cua(doi_tuong);
                let t_cs = bo_tham_chieu(&self.kieu_cua(chi_so));
                if let T::SoNguyen(k) = t_cs {
                    if k != KieuNguyen::Usize && k != KieuNguyen::ChuaGhim {
                        self.bao_chi_so_khong_usize(chi_so.span(), k);
                    }
                }
                // `v[0]` có kiểu phần tử của `v`. Trả `T::Mo` ở đây làm câm
                // mọi kiểm tra phía sau: `chac_chan_lech` coi `Mo` là tương
                // thích với tất cả, nên một `Vec<bool>` truyền vào hàm nhận
                // `i64` đi lọt.
                match bo_tham_chieu(&t_dt) {
                    T::Vec(x) => (*x).clone(),
                    _ => T::Mo,
                }
            }
            BieuThuc::Khoi(k) => self.khoi(k),
            BieuThuc::Neu { dieu_kien, than, nguoc_lai, .. } => {
                self.kieu_cua(dieu_kien);
                let a = self.khoi(than);
                match nguoc_lai {
                    Some(nl) => {
                        let b = self.kieu_cua(nl);
                        // Hai nhánh của một `if` dùng làm giá trị phải cùng kiểu.
                        // Bản trước lấy `a` rồi vứt `b` — nên nhánh `else` sai
                        // kiểu, hay nhánh `else` kết bằng dấu chấm phẩy (thành
                        // `()`), đều lọt sạch.
                        if a.chac_chan_lech(&b) {
                            self.bao_nhanh_lech(bt.span(), "if", &a, &b, nl.span());
                            return T::ChuaBiet("hai nhánh if lệch kiểu");
                        }
                        if a == T::Mo { b } else { a }
                    }
                    None => T::Rong,
                }
            }
            BieuThuc::KhopMau { .. } => self.khop_mau(bt),
            BieuThuc::Tuple { phan_tu, .. } if phan_tu.is_empty() => T::Rong,
            BieuThuc::Tuple { phan_tu, .. } => {
                T::Tuple(phan_tu.iter().map(|e| self.kieu_cua(e)).collect())
            }
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
            BieuThuc::Thoat { gia_tri, .. } => {
                if let Some(e) = gia_tri { self.kieu_cua(e); }
            }
            BieuThuc::TraVe { gia_tri, span } => {
                let thuc = match gia_tri {
                    Some(e) => self.kieu_cua(e),
                    None => T::Rong,
                };
                let sp = gia_tri.as_ref().map(|e| e.span()).unwrap_or(*span);
                if let Some((mong, sp_kb)) = self.tra_ve_ham.clone() {
                    if mong.chac_chan_lech(&thuc) {
                        self.bao_tra_ve_som_lech(sp, sp_kb, &mong, &thuc);
                    }
                }
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
            // Mọi nhánh `match` phải cho ra cùng một kiểu. Bản trước chỉ giữ
            // kiểu của nhánh đầu tiên khác `Mo` rồi bỏ qua phần còn lại —
            // nên `match x { 0 => 0, _ => false }` đi lọt.
            if kieu_nhanh != T::Mo && kieu_nhanh.chac_chan_lech(&t) {
                self.bao_nhanh_lech(*span, "match", &kieu_nhanh, &t, n.than.span());
                kieu_nhanh = T::ChuaBiet("các nhánh match lệch kiểu");
            } else if kieu_nhanh == T::Mo {
                kieu_nhanh = t;
            }
        }

        // Vét cạn — phủ không gian giá trị.
        //
        // Bản trước chỉ xử lý một hình dạng: match trên enum với mẫu biến thể
        // phẳng, và bọc trong `if !co_bao_quat` mà `co_bao_quat` lại đặt bởi
        // BẤT KỲ mẫu `_`/tên nào, kể cả nhánh CÓ guard. Nay dùng thuật toán:
        // mỗi nhánh KHÔNG guard trừ đi phần nó phủ, còn sót thì báo kèm nhân chứng.
        let khong_guard: Vec<&Mau> = nhanh
            .iter()
            .filter(|n| n.dieu_kien.is_none())
            .map(|n| &n.mau)
            .collect();
        let kg = self.khong_gian(&t_gt);
        if let Some(nhan_chung) = crate::vet_can::thieu(&kg, &khong_guard) {
            self.bao_thieu_nhanh(*span, &t_gt, &nhan_chung);
        }
        let _ = (co_bao_quat, da_phu);
        kieu_nhanh
    }

    /// Kiểu trả về của một lời gọi phương thức, có tính tới closure truyền vào.
    ///
    /// Không suy được kiểu qua `map`/`filter` thì mọi chuỗi iterator — thứ người
    /// học FP dùng liên tục — đều rơi vào `ChuaHoTro`, và công cụ thành vô dụng.
    /// Đây là chỗ đáng bỏ công mô hình hoá cho tử tế.
    fn kieu_phuong_thuc(&mut self, chu: &T, ten: &str, doi_so: &[BieuThuc]) -> T {
        let goc = bo_tham_chieu(chu);

        // ── Bộ lặp: `.iter()` mượn từng phần tử nên item là `&T`, không phải
        // `T`. Chi tiết này quan trọng thật: `v.iter().filter(|x| x > 4)` là
        // E0308, vì `&i64` không so sánh trực tiếp với số nguyên được.
        match (&goc, ten) {
            (T::Vec(x), "iter") => {
                for a in doi_so { self.kieu_cua(a); }
                return T::Lap(Box::new(T::Tham(x.clone())));
            }
            (T::Vec(x), "into_iter") => {
                for a in doi_so { self.kieu_cua(a); }
                return T::Lap(x.clone());
            }
            (T::Chuoi, "chars") => {
                for a in doi_so { self.kieu_cua(a); }
                return T::Lap(Box::new(T::KyTu));
            }
            (T::Lap(x), "map") => {
                let ra = self.kieu_than_be_quan(doi_so.first(), x);
                return T::Lap(Box::new(ra));
            }
            (T::Lap(x), "filter") => {
                // Closure của `filter` nhận thêm một tầng tham chiếu nữa.
                self.kieu_than_be_quan(doi_so.first(), &T::Tham(x.clone()));
                return T::Lap(x.clone());
            }
            (T::Lap(_), "rev" | "take" | "skip" | "peekable") => {
                for a in doi_so { self.kieu_cua(a); }
                return goc.clone();
            }
            (T::Lap(x), "enumerate") => {
                for a in doi_so { self.kieu_cua(a); }
                return T::Lap(Box::new(T::Tuple(vec![
                    T::SoNguyen(KieuNguyen::Usize),
                    (**x).clone(),
                ])));
            }
            (T::Lap(x), "collect") => {
                for a in doi_so { self.kieu_cua(a); }
                return T::Vec(Box::new(bo_tham_chieu(x)));
            }
            (T::Lap(x), "sum" | "fold" | "product") => {
                for a in doi_so { self.kieu_cua(a); }
                return bo_tham_chieu(x);
            }
            (T::Lap(_), "count") => {
                for a in doi_so { self.kieu_cua(a); }
                return T::SoNguyen(KieuNguyen::Usize);
            }
            (T::Lap(_), "any" | "all") => {
                for a in doi_so { self.kieu_cua(a); }
                return T::Bool;
            }
            _ => {}
        }

        for a in doi_so { self.kieu_cua(a); }

        // ── Gọi phương thức của bộ lặp thẳng trên `Vec` ────────────────────
        // Đây là chỗ vấp kinh điển khi chuyển từ Python sang: `sum(ds)` bên
        // Python thành `ds.iter().sum()` bên Rust, không phải `ds.sum()`.
        if matches!(goc, T::Vec(_)) && CHI_TREN_BO_LAP.contains(&ten) {
            self.bao_thieu_iter(doi_so, chu, ten);
            return T::ChuaBiet("phương thức của bộ lặp gọi thẳng trên Vec");
        }

        // ── Phương thức không tồn tại trên bộ thu này ──────────────────────
        if let Some(nhom) = nhom_bo_thu(&goc) {
            let co = phuong_thuc_cua(nhom).contains(&ten);
            let tu_viet = self.phuong_thuc_tu_viet.contains(ten);
            if !co && !tu_viet {
                self.bao_khong_co_phuong_thuc(doi_so, &goc, ten, nhom);
                return T::ChuaBiet("phương thức không tồn tại");
            }
        }

        if let Some(n) = so_doi_so_phuong_thuc(ten) {
            if n != doi_so.len() {
                self.bao_sai_so_doi_so_phuong_thuc(ten, n, doi_so.len(), doi_so);
                return T::ChuaBiet("phương thức sai số đối số");
            }
        }

        self.kiem_doi_so_phuong_thuc(&goc, ten, doi_so);
        kieu_tra_ve_phuong_thuc(&goc, ten)
    }

    /// Đối chiếu kiểu đối số cho những phương thức có chữ ký chắc chắn.
    fn kiem_doi_so_phuong_thuc(&mut self, chu: &T, ten: &str, doi_so: &[BieuThuc]) {
        let Some(a0) = doi_so.first() else { return };
        let thuc = self.kieu_cua(a0);
        let mong = match (chu, ten) {
            (T::Vec(x), "push") => (**x).clone(),
            (T::Vec(x), "contains") => T::Tham(x.clone()),
            (T::Chuoi, "push_str") => T::Tham(Box::new(T::Chuoi)),
            (T::Chuoi, "push") => T::KyTu,
            _ => return,
        };
        if thuc.chua_biet().is_some() || mong.chua_biet().is_some() {
            return;
        }
        // `contains` cần `&T`: `so.contains(2)` là E0308, phải viết `&2`. Đây
        // là lỗi người mới gặp rất sớm và thông báo gốc của rustc khó hiểu.
        let lech_tham_chieu = matches!(&mong, T::Tham(_)) && !matches!(&thuc, T::Tham(_));
        if lech_tham_chieu || mong.chac_chan_lech(&thuc) {
            self.bao_doi_so_phuong_thuc_lech(a0.span(), ten, &mong, &thuc, lech_tham_chieu);
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

    /// Không gian giá trị của một kiểu, để kiểm vét cạn.
    fn khong_gian(&self, t: &T) -> crate::vet_can::KhongGian {
        use crate::vet_can::{kg_bool, kg_enum, KhongGian};
        match t {
            T::Bool => kg_bool(),
            T::Enum(ten) => kg_enum(ten, &self.payload, 0),
            // Số, chuỗi, ký tự: miền vô hạn — chỉ `_` hoặc một binding mới phủ nổi.
            T::SoNguyen(_) | T::SoThuc(_) | T::KyTu | T::Chuoi => KhongGian::VoHan,
            T::Tuple(cac) => KhongGian::Tich(cac.iter().map(|x| self.khong_gian(x)).collect()),
            T::Tham(x) => self.khong_gian(x),
            _ => KhongGian::KhongBiet,
        }
    }

    /// Tên gần đúng nhất trong `co` so với `go` — để gợi ý "ý bạn là `x`?".
    /// Khoảng cách Levenshtein; chỉ gợi ý khi đủ gần (≤ 1/3 độ dài).
    fn gan_nhat<'b, I: Iterator<Item = &'b String>>(go: &str, co: I) -> Option<String> {
        let mut tot: Option<(usize, String)> = None;
        for ung in co {
            let d = khoang_cach(go, ung);
            if tot.as_ref().is_none_or(|(td, _)| d < *td) {
                tot = Some((d, ung.clone()));
            }
        }
        tot.filter(|(d, _)| *d * 3 <= go.chars().count().max(3)).map(|(_, s)| s)
    }

    /// Số tham số của một biến thể enum, `None` nếu chưa nạp được.
    /// `Option`/`Result` dựng sẵn không nằm trong bảng nên trả `None` — đúng
    /// ý: chưa mô hình hoá thì đừng phán.
    fn so_payload(&self, en: &str, bien_the: &str) -> Option<usize> {
        self.payload
            .get(en)?
            .iter()
            .find(|(b, _)| b == bien_the)
            .map(|(_, k)| k.len())
    }

    fn bao_khong_co_bien_the(&mut self, span: Span, en: &str, bien_the: &str) {
        let ds_v = self.enum_bien_the.get(en).cloned().unwrap_or_default();
        let ds = ds_v.iter().map(|s| format!("`{s}`")).collect::<Vec<_>>().join(", ");
        let mut d = Diagnostic::loi("BR0326", format!("`{en}` không có biến thể `{bien_the}`"))
            .tai(span, "biến thể này không nằm trong khai báo enum")
            .vi_sao("Một enum liệt kê ĐẦY ĐỦ các khả năng — đó là toàn bộ sức mạnh của nó. Compiler biết chính xác danh sách ấy, nên tên lạ bị chặn ngay; cũng chính nhờ vậy `match` mới kiểm tra được bạn đã phủ hết hay chưa.");
        if let Some(g) = Self::gan_nhat(bien_the, ds_v.iter()) {
            d = d.sua(format!("có phải bạn muốn viết `{en}::{g}` không?"));
        }
        self.diags.push(d.sua(format!("`{en}` có: {ds}")).khai_niem("enum"));
    }

    fn bao_thieu_payload(&mut self, span: Span, en: &str, bien_the: &str, can: usize) {
        self.diags.push(
            Diagnostic::loi("BR0327", format!("`{en}::{bien_the}` cần {can} tham số nhưng đang dùng trần"))
                .tai(span, "đây mới là hàm dựng, chưa phải một giá trị")
                .vi_sao(format!("`{bien_the}` được khai báo là mang dữ liệu, nên bản thân cái tên chỉ là hàm để TẠO ra giá trị. Viết tên trần giống như nhắc đến `String::from` mà không gọi nó — bạn đang cầm công cụ chứ chưa cầm kết quả."))
                .sua(format!("gọi nó: `{en}::{bien_the}(...)`"))
                .khai_niem("enum"),
        );
    }

    fn bao_sai_arity_bien_the(&mut self, span: Span, en: &str, bien_the: &str, can: usize, co: usize) {
        self.diags.push(
            Diagnostic::loi(
                "BR0328",
                format!("`{en}::{bien_the}` nhận {can} tham số nhưng được truyền {co}"),
            )
            .tai(span, format!("ở đây truyền {co}"))
            .vi_sao("Số lượng dữ liệu mà mỗi biến thể mang theo được ghim lúc khai báo enum. Nhờ nó cố định, `match` mới tách được đúng từng phần ra mà không bao giờ trượt chỉ số.")
            .sua(format!("truyền đúng {can} giá trị"))
            .sua(format!("hoặc sửa khai báo `{bien_the}` cho khớp"))
            .khai_niem("enum"),
        );
    }

    fn bao_nhanh_lech(&mut self, span: Span, dang: &str, a: &T, b: &T, span_b: Span) {
        let ten_dang = if dang == "if" { "hai nhánh của `if`" } else { "các nhánh của `match`" };
        self.diags.push(
            Diagnostic::loi(
                "BR0330",
                format!("{ten_dang} cho ra hai kiểu khác nhau: {} và {}", a.hien_thi(), b.hien_thi()),
            )
            .nhan(Label::chinh(span_b, format!("nhánh này cho ra {}", b.hien_thi())))
            .nhan(Label::phu(span, format!("nhánh trước cho ra {}", a.hien_thi())))
            .vi_sao("Trong Rust `if` và `match` là BIỂU THỨC — chúng có giá trị, và giá trị đó phải có đúng một kiểu dù chạy vào nhánh nào. Nhờ vậy bạn viết được `let x = if ... {} else {}` mà không sợ `x` lúc là số lúc là chữ. Một dấu chấm phẩy thừa ở cuối nhánh cũng đủ đổi kiểu nhánh đó thành `()`.")
            .sua("cho hai nhánh trả về cùng một kiểu")
            .sua("hoặc kiểm tra xem có dấu `;` thừa ở cuối một nhánh không")
            .khai_niem(if dang == "if" { "if" } else { "match" }),
        );
    }

    fn bao_tra_ve_som_lech(&mut self, span: Span, span_kb: Span, mong: &T, thuc: &T) {
        self.diags.push(
            Diagnostic::loi(
                "BR0331",
                format!("`return` trả {} nhưng hàm khai báo trả {}", thuc.hien_thi(), mong.hien_thi()),
            )
            .nhan(Label::chinh(span, format!("giá trị này là {}", thuc.hien_thi())))
            .nhan(Label::phu(span_kb, format!("chữ ký hàm ghi {}", mong.hien_thi())))
            .vi_sao("Mọi lối ra của hàm đều phải tôn trọng cùng một chữ ký — cả `return` giữa chừng lẫn giá trị cuối thân hàm. Người gọi chỉ nhìn thấy chữ ký, nên nếu một lối ra trả kiểu khác thì lời hứa ấy gãy.")
            .sua("sửa giá trị của `return` cho khớp kiểu trả về")
            .sua("hoặc sửa kiểu trả về trong chữ ký hàm")
            .khai_niem("hàm"),
        );
    }

    fn bao_sai_so_doi_so(&mut self, span: Span, ck_span: Span, ten: &str, can: usize, co: usize) {
        let (dong_tu, lam) = if co < can { ("thiếu", "thêm") } else { ("thừa", "bớt") };
        self.diags.push(
            Diagnostic::loi("BR0332", format!("`{ten}` nhận {can} đối số nhưng được gọi với {co}"))
                .nhan(Label::chinh(span, format!("{dong_tu} đối số ở đây")))
                .nhan(Label::phu(ck_span, format!("`{ten}` khai báo {can} tham số")))
                .vi_sao("Rust không có tham số tuỳ chọn và không có giá trị mặc định cho tham số. Số đối số phải khớp chính xác — kể cả khi lời gọi nằm trong một nhánh không bao giờ chạy tới, vì việc kiểm này xảy ra lúc biên dịch chứ không lúc chạy.")
                .sua(format!("{lam} đối số cho đủ {can}"))
                .khai_niem("hàm"),
        );
    }

    fn bao_ham_khong_ton_tai(&mut self, span: Span, ten: &str) {
        let mut d = Diagnostic::loi("BR0333", format!("không tìm thấy hàm tên `{ten}`"))
            .tai(span, "chưa có `fn` nào mang tên này")
            .vi_sao("Rust phân giải mọi cái tên lúc biên dịch. Kể cả khi lời gọi nằm trong nhánh `if` không bao giờ đúng, tên vẫn phải tồn tại — đây đúng là chỗ mà một bộ kiểm chạy-rồi-mới-biết không bao giờ nhìn thấy.");
        if let Some(g) = Self::gan_nhat(ten, self.ham.keys()) {
            d = d.sua(format!("có phải bạn muốn gọi `{g}` không?"));
        }
        self.diags.push(d.sua(format!("hoặc viết `fn {ten}(...)` trước khi gọi")).khai_niem("hàm"));
    }

    fn bao_sai_so_doi_so_phuong_thuc(&mut self, ten: &str, can: usize, co: usize, doi_so: &[BieuThuc]) {
        let span = doi_so.first().map(|a| a.span()).unwrap_or_default();
        self.diags.push(
            Diagnostic::loi(
                "BR0334",
                format!("`.{ten}()` nhận {can} đối số nhưng được gọi với {co}"),
            )
            .tai(span, format!("ở đây truyền {co}"))
            .vi_sao(format!("`{ten}` là phương thức dựng sẵn, số tham số của nó cố định. Giá trị đứng trước dấu chấm là `self` — nó KHÔNG được tính là một đối số."))
            .sua(format!("truyền đúng {can} đối số"))
            .khai_niem("phương thức"),
        );
    }

    fn bao_thieu_iter(&mut self, doi_so: &[BieuThuc], chu: &T, ten: &str) {
        let span = doi_so.first().map(|a| a.span()).unwrap_or_default();
        self.diags.push(
            Diagnostic::loi(
                "BR0335",
                format!("`.{ten}()` là phương thức của BỘ LẶP, không phải của `{}`", chu.hien_thi()),
            )
            .tai(span, format!("`{}` chưa phải bộ lặp", chu.hien_thi()))
            .vi_sao("Rust tách rời chỗ CHỨA dữ liệu và cách ĐI QUA dữ liệu. Một `Vec` chỉ biết giữ giá trị; muốn cộng dồn, lọc hay biến đổi thì phải xin nó một bộ lặp trước. Nghe rườm rà, nhưng chính nhờ tách đôi như vậy mà cùng một chuỗi `.map().filter().sum()` chạy được trên `Vec`, trên `HashMap`, trên dòng đọc từ file — và không đoạn nào tạo ra danh sách trung gian.")
            .sua(format!("thêm `.iter()` vào trước: `.iter().{ten}()`"))
            .sua("hoặc `.into_iter()` nếu bạn muốn chuyển hẳn quyền sở hữu vào bộ lặp")
            .khai_niem("bộ lặp"),
        );
    }

    fn bao_khong_co_phuong_thuc(&mut self, doi_so: &[BieuThuc], chu: &T, ten: &str, nhom: NhomBoThu) {
        let span = doi_so.first().map(|a| a.span()).unwrap_or_default();
        let ds = phuong_thuc_cua(nhom);
        let mut d = Diagnostic::loi(
            "BR0336",
            format!("`{}` không có phương thức `{ten}`", chu.hien_thi()),
        )
        .tai(span, "không tìm thấy phương thức này")
        .vi_sao("Mỗi kiểu chỉ có đúng những phương thức đã được định nghĩa cho nó. Không có kế thừa ngầm, không có phương thức mọc thêm lúc chạy — nên gõ sai tên bị bắt ngay lúc biên dịch.");
        let ds_owned: Vec<String> = ds.iter().map(|s| (*s).to_string()).collect();
        if let Some(g) = Self::gan_nhat(ten, ds_owned.iter()) {
            d = d.sua(format!("có phải bạn muốn gọi `.{g}()` không?"));
        }
        self.diags.push(d.khai_niem("phương thức"));
    }

    fn bao_doi_so_phuong_thuc_lech(
        &mut self,
        span: Span,
        ten: &str,
        mong: &T,
        thuc: &T,
        thieu_dau_muon: bool,
    ) {
        let mut d = Diagnostic::loi(
            "BR0337",
            format!("`.{ten}()` cần {} nhưng nhận {}", mong.hien_thi(), thuc.hien_thi()),
        )
        .tai(span, format!("giá trị này là {}", thuc.hien_thi()));
        d = if thieu_dau_muon {
            d.vi_sao(format!("`{ten}` chỉ MƯỢN giá trị để xem, không lấy quyền sở hữu — nên nó nhận một tham chiếu. Dấu `&` là cách bạn nói “cho xem thôi, không đưa hẳn”."))
                .sua("thêm dấu `&` trước đối số")
        } else {
            d.vi_sao("Rust không tự chuyển đổi kiểu ngầm. Một phương thức khai báo nhận kiểu nào thì chỉ nhận đúng kiểu đó.")
                .sua(format!("đưa vào một giá trị {}", mong.hien_thi()))
        };
        self.diags.push(d.khai_niem("phương thức"));
    }

    fn bao_khong_co_struct(&mut self, span: Span, ten: &str) {
        let mut d = Diagnostic::loi("BR0320", format!("không tìm thấy struct tên `{ten}`"))
            .tai(span, "chưa có `struct` nào mang tên này")
            .vi_sao("Rust không tự tạo kiểu khi bạn viết ra một cái tên lạ. Mọi struct phải được khai báo trước — đó là lý do gõ sai tên bị bắt ngay lúc biên dịch thay vì đẻ ra một object rỗng lúc chạy như trong JavaScript.");
        if let Some(g) = Self::gan_nhat(ten, self.struct_co.iter()) {
            d = d.sua(format!("có phải bạn muốn viết `{g}` không?"));
        }
        self.diags.push(d.sua(format!("hoặc khai báo `struct {ten} {{ ... }}`")).khai_niem("struct"));
    }

    fn bao_khoi_tao_sai_dang(&mut self, span: Span, en: &str, bien_the: &str) {
        self.diags.push(
            Diagnostic::loi("BR0321", format!("`{en}::{bien_the}` là biến thể dạng tuple, không phải dạng struct"))
                .tai(span, "ở đây đang viết theo cú pháp `{ tên: giá_trị }`")
                .vi_sao("Một biến thể enum mang dữ liệu theo VỊ TRÍ (`Tron(i64)`) hoặc theo TÊN (`Tron { r: i64 }`) — bạn chọn lúc khai báo enum, và phải dựng đúng theo cách đã chọn. Hai dạng không thay thế nhau được.")
                .sua(format!("dựng theo vị trí: `{en}::{bien_the}(giá_trị)`"))
                .sua(format!("hoặc đổi khai báo thành `{bien_the} {{ ... }}` nếu bạn muốn đặt tên trường"))
                .khai_niem("enum"),
        );
    }

    fn bao_truong_thua(&mut self, span: Span, ten: &str, truong: &str, bang: &HashMap<String, T>) {
        let mut co: Vec<&String> = bang.keys().collect();
        co.sort();
        let ds = co.iter().map(|s| format!("`{s}`")).collect::<Vec<_>>().join(", ");
        let mut d = Diagnostic::loi("BR0322", format!("struct `{ten}` không có trường `{truong}`"))
            .tai(span, "trường này không nằm trong khai báo")
            .vi_sao("Struct trong Rust có hình dạng cố định, quyết định lúc biên dịch. Không thể đính thêm trường lúc dựng như thêm key vào dict Python — bù lại compiler biết chính xác kích thước và bố cục của nó trong bộ nhớ.");
        if let Some(g) = Self::gan_nhat(truong, bang.keys()) {
            d = d.sua(format!("có phải bạn muốn viết `{g}` không?"));
        }
        self.diags.push(d.sua(format!("`{ten}` có các trường: {ds}")).khai_niem("struct"));
    }

    fn bao_thieu_truong(&mut self, span: Span, ten: &str, truong: &str) {
        self.diags.push(
            Diagnostic::loi("BR0323", format!("thiếu trường `{truong}` khi dựng `{ten}`"))
                .tai(span, format!("chưa gán giá trị cho `{truong}`"))
                .vi_sao("Rust không có `null` và không có giá trị mặc định ngầm. Muốn dựng một struct thì MỌI trường phải có giá trị — nhờ vậy không bao giờ tồn tại một giá trị nửa vời mà chương trình phải đoán xem đã khởi tạo xong chưa.")
                .sua(format!("thêm `{truong}: ...` vào phần khởi tạo"))
                .khai_niem("struct"),
        );
    }

    fn bao_truong_sai_kieu(&mut self, span: Span, ten: &str, truong: &str, mong: &T, thuc: &T) {
        self.diags.push(
            Diagnostic::loi(
                "BR0324",
                format!("trường `{truong}` cần {} nhưng nhận {}", mong.hien_thi(), thuc.hien_thi()),
            )
            .tai(span, format!("giá trị này là {}", thuc.hien_thi()))
            .vi_sao(format!("Khai báo `struct {ten}` đã ghim kiểu của `{truong}`. Kiểu của một trường là lời hứa với mọi đoạn mã đọc nó về sau; Rust không cho phá lời hứa đó ngay tại chỗ dựng."))
            .sua(format!("đưa vào một giá trị {}", mong.hien_thi()))
            .khai_niem("struct"),
        );
    }

    fn bao_truong_khong_co(&mut self, span: Span, ten: &str, truong: &str, bang: &HashMap<String, T>) {
        let mut co: Vec<&String> = bang.keys().collect();
        co.sort();
        let ds = co.iter().map(|s| format!("`{s}`")).collect::<Vec<_>>().join(", ");
        let mut d = Diagnostic::loi("BR0325", format!("`{ten}` không có trường `{truong}`"))
            .tai(span, "không đọc/ghi được trường không tồn tại")
            .vi_sao(format!("Compiler biết chắc giá trị này là `{ten}` và biết đầy đủ danh sách trường của nó, nên nó bắt lỗi gõ sai tên ngay — không cần chạy tới dòng đó mới phát hiện."));
        if let Some(g) = Self::gan_nhat(truong, bang.keys()) {
            d = d.sua(format!("có phải bạn muốn viết `{g}` không?"));
        }
        self.diags.push(d.sua(format!("`{ten}` có các trường: {ds}")).khai_niem("struct"));
    }

    fn bao_thieu_nhanh(&mut self, span: Span, t: &T, nhan_chung: &str) {
        self.diags.push(
            Diagnostic::loi("BR0300", "`match` chưa phủ hết mọi khả năng")
                .tai(span, format!("chưa có nhánh nào bắt `{nhan_chung}`"))
                .vi_sao("Đây chính là điều làm `match` an toàn hơn `switch` của các ngôn ngữ khác: compiler bắt buộc bạn xử lý MỌI trường hợp. Nhờ vậy khi ai đó thêm một biến thể mới vào enum, mọi chỗ quên xử lý sẽ báo lỗi ngay lúc biên dịch chứ không âm thầm chạy sai.")
                .sua(format!("thêm nhánh cho `{nhan_chung}`"))
                .sua("hoặc thêm `_ => ...` để bắt các trường hợp còn lại")
                .khai_niem("match"),
        );
        let _ = t;
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

    fn bao_chi_so_khong_usize(&mut self, span: Span, k: KieuNguyen) {
        self.diags.push(
            Diagnostic::loi("BR0306", format!("chỉ số phải là `usize`, không phải `{}`", k.ten()))
                .tai(span, format!("đây là `{}`", k.ten()))
                .vi_sao("Chỉ số vào `Vec` hay mảng luôn là `usize` — kiểu số nguyên không dấu có bề rộng bằng con trỏ. Dùng kiểu có dấu sẽ cho phép chỉ số âm, thứ không bao giờ hợp lệ.")
                .sua("ép tường minh: `v[i as usize]`")
                .sua("hoặc khai báo biến chỉ số là `usize` ngay từ đầu")
                .khai_niem("chỉ số"),
        );
    }

    fn bao_toan_hang_lech(&mut self, span: Span, ky_hieu: &str, a: &T, b: &T) {
        self.diags.push(
            Diagnostic::loi("BR0305", format!("`{}` không dùng được giữa {} và {}", ky_hieu, a.hien_thi(), b.hien_thi()))
                .tai(span, "hai vế khác kiểu")
                .vi_sao("Rust KHÔNG tự ép kiểu số. `10 + 2.5` là lỗi, `u32 + i64` cũng là lỗi. Khác Python và JavaScript, nơi ép ngầm âm thầm làm tròn hoặc mất chính xác mà không ai biết.")
                .sua(format!("ép tường minh một vế, ví dụ `x as {}`", b.hien_thi()))
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
            bk.tra_ve_ham = h
                .kieu_tra_ve
                .as_ref()
                .map(|kb| (bk.chuan_hoa(tu_kieu_ast(kb)), kb.span()));
            let t_than = bk.khoi(&h.than);
            bk.tra_ve_ham = None;
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

/// Khoảng cách Levenshtein giữa hai chuỗi, đếm theo ký tự Unicode.
fn khoang_cach(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut truoc: Vec<usize> = (0..=b.len()).collect();
    let mut nay = vec![0usize; b.len() + 1];
    for (i, ca) in a.iter().enumerate() {
        nay[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let doi = truoc[j] + usize::from(ca != cb);
            nay[j + 1] = doi.min(truoc[j + 1] + 1).min(nay[j] + 1);
        }
        std::mem::swap(&mut truoc, &mut nay);
    }
    truoc[b.len()]
}

/// Những cái tên gọi-được nhưng không phải `fn` do người học viết: macro dựng
/// sẵn và hàm dựng của `Option`/`Result`. Thiếu bảng này thì mọi `println!`
/// đều bị báo là hàm không tồn tại.
const TEN_DUNG_SAN: &[&str] = &[
    "println", "print", "eprintln", "eprint", "format", "vec", "panic", "assert",
    "assert_eq", "assert_ne", "write", "writeln", "dbg", "todo", "unimplemented",
    "Some", "None", "Ok", "Err", "String", "Vec", "Box", "drop",
];

/// Số đối số của một phương thức dựng sẵn — KHÔNG tính `self`.
///
/// Chỉ liệt kê những phương thức có đúng một arity. `min`/`max` cố tình vắng
/// mặt: `a.min(b)` trên số nhận 1, còn `iter.min()` nhận 0, và ở tầng này ta
/// chưa phân biệt được hai trường hợp — đoán bừa thì báo oan, mà báo oan còn
/// tệ hơn bỏ lọt (ADR-002 §3).
fn so_doi_so_phuong_thuc(ten: &str) -> Option<usize> {
    Some(match ten {
        "len" | "is_empty" | "pop" | "clear" | "clone" | "trim" | "to_string"
        | "to_uppercase" | "to_lowercase" | "chars" | "bytes" | "as_str" | "as_bytes"
        | "to_owned" | "first" | "last" | "sort" | "reverse" | "count" | "abs"
        | "sqrt" | "floor" | "ceil" | "round" | "signum" | "unwrap" | "is_some"
        | "is_none" | "is_ok" | "is_err" | "enumerate" | "parse" | "next" => 0,
        "push" | "push_str" | "contains" | "remove" | "get" | "starts_with"
        | "ends_with" | "split" | "join" | "repeat" | "truncate" | "extend"
        | "unwrap_or" | "expect" | "take" | "skip" | "zip" | "pow"
        | "saturating_sub" | "saturating_add" | "checked_add" | "checked_sub"
        | "wrapping_add" | "wrapping_sub" | "position" | "any" | "all" | "find" => 1,
        "insert" | "replace" | "splitn" => 2,
        _ => return None,
    })
}

/// Những phương thức chỉ tồn tại trên bộ lặp, không tồn tại trên `Vec`.
///
/// `len` cố tình vắng mặt: `Vec` có `len()`, bộ lặp thì không — nhầm chiều.
const CHI_TREN_BO_LAP: &[&str] = &[
    "map", "filter", "fold", "collect", "sum", "product", "rev", "enumerate",
    "zip", "take", "skip", "any", "all", "find", "position", "flat_map",
    "filter_map", "peekable", "chain", "step_by", "take_while", "skip_while",
    "for_each", "max_by_key", "min_by_key", "partition", "next",
];

#[derive(Clone, Copy, PartialEq)]
enum NhomBoThu {
    Chuoi,
    Vec,
    So,
    BoLap,
}

/// Nhóm bộ thu, hoặc `None` khi chưa biết chắc.
///
/// Trả `None` cho struct, enum và `Mo` là cố ý: ở đó người học có thể tự viết
/// `impl`, nên phán "không có phương thức này" sẽ báo oan.
fn nhom_bo_thu(t: &T) -> Option<NhomBoThu> {
    match t {
        T::Chuoi => Some(NhomBoThu::Chuoi),
        T::Vec(_) => Some(NhomBoThu::Vec),
        T::Lap(_) => Some(NhomBoThu::BoLap),
        T::SoNguyen(_) | T::SoThuc(_) => Some(NhomBoThu::So),
        _ => None,
    }
}

fn phuong_thuc_cua(nhom: NhomBoThu) -> &'static [&'static str] {
    match nhom {
        NhomBoThu::Chuoi => &[
            "len", "is_empty", "push", "push_str", "chars", "bytes", "split",
            "splitn", "split_whitespace", "lines", "trim", "trim_start", "trim_end",
            "to_string", "to_owned", "to_uppercase", "to_lowercase", "contains",
            "starts_with", "ends_with", "replace", "parse", "as_str", "as_bytes",
            "repeat", "find", "char_indices", "clone", "insert", "remove", "clear",
            "capacity", "into", "eq", "cmp", "get",
        ],
        NhomBoThu::Vec => &[
            "len", "is_empty", "push", "pop", "insert", "remove", "clear",
            "contains", "iter", "into_iter", "iter_mut", "first", "last", "get",
            "sort", "sort_by", "sort_by_key", "reverse", "truncate", "extend",
            "join", "concat", "to_vec", "clone", "dedup", "retain", "swap",
            "split_off", "append", "capacity", "as_slice", "windows", "chunks",
            "binary_search", "into", "get_mut", "resize", "fill",
        ],
        NhomBoThu::So => &[
            "abs", "pow", "powi", "powf", "sqrt", "floor", "ceil", "round",
            "trunc", "signum", "min", "max", "clamp", "to_string", "clone",
            "checked_add", "checked_sub", "checked_mul", "checked_div",
            "saturating_add", "saturating_sub", "saturating_mul",
            "wrapping_add", "wrapping_sub", "wrapping_mul", "rem_euclid",
            "count_ones", "leading_zeros", "is_nan", "is_finite", "into", "cmp",
        ],
        NhomBoThu::BoLap => CHI_TREN_BO_LAP,
    }
}
