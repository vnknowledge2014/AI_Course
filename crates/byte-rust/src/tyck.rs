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

/// Kiểu ở mức đủ dùng để bắt lỗi thường gặp. `Mo` nghĩa là "không suy ra được".
#[derive(Debug, Clone, PartialEq)]
pub enum T {
    SoNguyen,
    SoThuc,
    Bool,
    KyTu,
    Chuoi,
    Rong,
    Vec(Box<T>),
    Struct(String),
    Enum(String),
    Tham(Box<T>),
    /// Không suy ra được — không bao giờ báo lỗi dựa trên kiểu này.
    Mo,
}

impl T {
    pub fn hien_thi(&self) -> String {
        match self {
            T::SoNguyen => "số nguyên".into(),
            T::SoThuc => "số thực".into(),
            T::Bool => "bool".into(),
            T::KyTu => "char".into(),
            T::Chuoi => "String".into(),
            T::Rong => "()".into(),
            T::Vec(t) => format!("Vec<{}>", t.hien_thi()),
            T::Struct(n) | T::Enum(n) => n.clone(),
            T::Tham(t) => format!("&{}", t.hien_thi()),
            T::Mo => "?".into(),
        }
    }

    /// Hai kiểu này chắc chắn KHÔNG tương thích?
    ///
    /// Cố tình bảo thủ: chỉ trả `true` khi chắc chắn. `Mo` luôn tương thích.
    pub fn chac_chan_lech(&self, khac: &T) -> bool {
        use T::*;
        match (self, khac) {
            (Mo, _) | (_, Mo) => false,
            // Tham chiếu và giá trị: rustc phân biệt, nhưng ta chỉ bắt khi
            // kiểu bên trong cũng lệch — tránh báo oan chỗ auto-deref.
            (Tham(a), Tham(b)) => a.chac_chan_lech(b),
            // Một bên là tham chiếu, bên kia không: rustc phân biệt, nhưng
            // auto-deref khiến nhiều chỗ vẫn hợp lệ. Chỉ báo khi kiểu BÊN TRONG
            // cũng lệch — tránh báo oan.
            (Tham(a), b) => a.chac_chan_lech(b),
            (a, Tham(b)) => a.chac_chan_lech(b),
            (SoNguyen, SoNguyen) | (SoThuc, SoThuc) | (Bool, Bool)
            | (KyTu, KyTu) | (Chuoi, Chuoi) | (Rong, Rong) => false,
            (Vec(a), Vec(b)) => a.chac_chan_lech(b),
            (Struct(a), Struct(b)) | (Enum(a), Enum(b)) => a != b,
            // Số nguyên và số thực: rustc KHÔNG tự ép, nhưng literal chưa gắn
            // kiểu thì linh hoạt. Không báo, tránh oan.
            (SoNguyen, SoThuc) | (SoThuc, SoNguyen) => false,
            _ => true,
        }
    }
}

fn tu_kieu_ast(k: &Kieu) -> T {
    match k {
        Kieu::DuongDan { doan, tham_so, .. } => {
            let ten = doan.last().map(String::as_str).unwrap_or("");
            match ten {
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32"
                | "u64" | "u128" | "usize" => T::SoNguyen,
                "f32" | "f64" => T::SoThuc,
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
                HangSo::SoNguyen(_) => T::SoNguyen,
                HangSo::SoThuc(_) => T::SoThuc,
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
            BieuThuc::HaiNgoi { toan_tu, trai, phai, .. } => {
                use ToanTuHai::*;
                match toan_tu {
                    Bang | KhacBang | NhoHon | LonHon | NhoBang | LonBang | Va | Hoac => T::Bool,
                    _ => {
                        let a = self.kieu_cua(trai);
                        let b = self.kieu_cua(phai);
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
                            if let Some(m) = mong.get(i) {
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
                for a in doi_so { self.kieu_cua(a); }
                kieu_tra_ve_phuong_thuc(&chu, ten)
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
                CauLenh::Let { mau, kieu, gia_tri, .. } => {
                    let t_gt = gia_tri.as_ref().map(|e| self.kieu_cua(e)).unwrap_or(T::Mo);
                    let t = kieu.as_ref().map(|k| self.chuan_hoa(tu_kieu_ast(k))).unwrap_or(t_gt);
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

/// Kiểu trả về của các phương thức dựng sẵn. `Mo` khi chưa biết.
fn kieu_tra_ve_phuong_thuc(chu: &T, ten: &str) -> T {
    match ten {
        "len" | "count" => T::SoNguyen,
        "is_empty" | "contains" | "starts_with" | "is_some" | "is_none" | "is_ok" | "is_err" => T::Bool,
        "to_string" | "to_uppercase" | "to_lowercase" | "trim" | "push_str" => T::Chuoi,
        "sqrt" | "round" => T::SoThuc,
        "clone" => chu.clone(),
        "abs" | "pow" | "min" | "max" | "sum" => match chu {
            T::Vec(t) => (**t).clone(),
            khac => khac.clone(),
        },
        "chars" => T::Vec(Box::new(T::KyTu)),
        "split" => T::Vec(Box::new(T::Chuoi)),
        _ => T::Mo,
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
                if mong.chac_chan_lech(&t_than) {
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
