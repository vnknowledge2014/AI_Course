//! Parser đệ quy xuống cho tập con Rust dùng trong giảng dạy.
//!
//! Hai nguyên tắc khác với parser thông thường:
//!
//! 1. **Phục hồi lỗi**: gặp lỗi thì đồng bộ lại tới `;` hoặc `}` gần nhất rồi đi
//!    tiếp, thay vì bỏ cuộc. Người học thường có nhiều lỗi cùng lúc.
//! 2. **Chẩn đoán đoán được ý định**: khi thiếu `;`, thiếu `)`, hay dùng `=` chỗ
//!    cần `==`, thông báo phải nói đúng thứ người học định làm — chứ không phải
//!    "unexpected token".

use crate::ast::*;
use crate::diag::{Diagnostic, Diagnostics, Label};
use crate::lexer::{TokKind, Token, TuKhoa};
use crate::span::Span;

pub struct Parser {
    toks: Vec<Token>,
    i: usize,
    pub diags: Diagnostics,
    /// Chặn đọc struct-literal khi đang ở điều kiện của `if`/`while`/`match`,
    /// nếu không `if x { }` sẽ bị hiểu là khởi tạo struct `x { }`.
    cam_struct_literal: bool,
}

type KetQua<T> = Result<T, ()>;

impl Parser {
    pub fn new(toks: Vec<Token>, diags: Diagnostics) -> Self {
        Self { toks, i: 0, diags, cam_struct_literal: false }
    }

    // ── Tiện ích trên luồng token ───────────────────────────────────────────

    fn xem(&self) -> &TokKind {
        &self.toks[self.i.min(self.toks.len() - 1)].kind
    }

    fn xem_n(&self, n: usize) -> &TokKind {
        &self.toks[(self.i + n).min(self.toks.len() - 1)].kind
    }

    fn span(&self) -> Span {
        self.toks[self.i.min(self.toks.len() - 1)].span
    }

    fn span_truoc(&self) -> Span {
        self.toks[self.i.saturating_sub(1).min(self.toks.len() - 1)].span
    }

    fn het(&self) -> bool {
        matches!(self.xem(), TokKind::HetTep)
    }

    fn tien(&mut self) -> Token {
        let t = self.toks[self.i.min(self.toks.len() - 1)].clone();
        if !self.het() {
            self.i += 1;
        }
        t
    }

    fn khop(&mut self, k: &TokKind) -> bool {
        if self.xem() == k {
            self.tien();
            true
        } else {
            false
        }
    }

    fn la(&self, k: &TokKind) -> bool {
        self.xem() == k
    }

    /// Bắt buộc phải gặp token `k`, nếu không thì báo lỗi có ngữ cảnh.
    fn can(&mut self, k: TokKind, dang_lam: &str) -> KetQua<Token> {
        if self.xem() == &k {
            return Ok(self.tien());
        }
        let thay = self.xem().mo_ta();
        let mong = k.mo_ta();
        let mut d = Diagnostic::loi("BR0100", format!("thiếu {mong}"))
            .nhan(Label::chinh(
                Span::at(self.span_truoc().end),
                format!("cần {mong} ở đây"),
            ))
            .nhan(Label::phu(self.span(), format!("nhưng gặp {thay}")))
            .vi_sao(format!("khi {dang_lam}, Rust cần {mong} ở vị trí này"));

        d = match &k {
            TokKind::ChamPhay => d
                .sua("thêm dấu `;` vào cuối câu lệnh")
                .khai_niem("câu lệnh"),
            TokKind::DongTron => d.sua("thêm dấu `)` để đóng lại dấu `(` đã mở"),
            TokKind::DongNhon => d.sua("thêm dấu `}` để đóng lại dấu `{` đã mở"),
            TokKind::DongVuong => d.sua("thêm dấu `]` để đóng lại dấu `[` đã mở"),
            _ => d,
        };
        self.diags.push(d);
        Err(())
    }

    fn can_ten(&mut self, dang_lam: &str) -> KetQua<(String, Span)> {
        match self.xem().clone() {
            TokKind::Ten(t) => {
                let sp = self.span();
                self.tien();
                Ok((t, sp))
            }
            TokKind::TuKhoa(k) => {
                let sp = self.span();
                self.diags.push(
                    Diagnostic::loi("BR0101", format!("`{}` là từ khoá, không dùng làm tên được", k.ten()))
                        .tai(sp, "từ khoá không thể làm tên")
                        .vi_sao("Rust dành riêng một số từ cho cú pháp; dùng chúng làm tên biến sẽ gây nhập nhằng")
                        .sua(format!("đổi thành tên khác, ví dụ `{}_` hoặc một tên mô tả rõ hơn", k.ten()))
                        .khai_niem("đặt tên"),
                );
                self.tien();
                Err(())
            }
            khac => {
                let sp = self.span();
                self.diags.push(
                    Diagnostic::loi("BR0102", "cần một cái tên ở đây")
                        .tai(sp, format!("gặp {} thay vì một cái tên", khac.mo_ta()))
                        .vi_sao(format!("khi {dang_lam}, vị trí này phải là tên do bạn đặt"))
                        .khai_niem("đặt tên"),
                );
                Err(())
            }
        }
    }

    /// Bỏ token cho tới ranh giới câu lệnh gần nhất để tiếp tục phân tích.
    ///
    /// **Bất biến quan trọng:** hàm này luôn tiến ít nhất một token. Nếu không,
    /// vòng lặp gọi nó sẽ quay vô hạn khi gặp lỗi ngay tại một token đồng bộ —
    /// và với một app học tập thì treo máy người học là lỗi không chấp nhận được.
    fn dong_bo(&mut self) {
        let bat_dau = self.i;
        self.dong_bo_trong();
        if self.i == bat_dau && !self.het() {
            self.tien();
        }
    }

    fn dong_bo_trong(&mut self) {
        let mut do_sau = 0i32;
        while !self.het() {
            match self.xem() {
                TokKind::MoNhon => do_sau += 1,
                TokKind::DongNhon => {
                    if do_sau == 0 {
                        return;
                    }
                    do_sau -= 1;
                }
                TokKind::ChamPhay if do_sau == 0 => {
                    self.tien();
                    return;
                }
                TokKind::TuKhoa(
                    TuKhoa::Let | TuKhoa::Fn | TuKhoa::Struct | TuKhoa::Enum
                    | TuKhoa::Impl | TuKhoa::Trait | TuKhoa::Use | TuKhoa::Const,
                ) if do_sau == 0 => return,
                _ => {}
            }
            self.tien();
        }
    }

    // ── Điểm vào ────────────────────────────────────────────────────────────

    pub fn phan_tich(mut self) -> (ChuongTrinh, Diagnostics) {
        let mut ct = ChuongTrinh::default();
        while !self.het() {
            let truoc = self.i;
            match self.muc() {
                Ok(m) => ct.muc.push(m),
                Err(()) => self.dong_bo(),
            }
            debug_assert!(self.i > truoc, "parser không tiến ở token {:?}", self.xem());
            if self.i == truoc {
                self.tien();
            }
        }
        (ct, self.diags)
    }

    // ── Khai báo cấp cao ────────────────────────────────────────────────────

    fn muc(&mut self) -> KetQua<Muc> {
        let bat_dau = self.span();
        let cong_khai = self.khop(&TokKind::TuKhoa(TuKhoa::Pub));

        match self.xem().clone() {
            TokKind::TuKhoa(TuKhoa::Fn) => Ok(Muc::Ham(self.ham(cong_khai, bat_dau)?)),
            TokKind::TuKhoa(TuKhoa::Struct) => Ok(Muc::Struct(self.khai_bao_struct(cong_khai, bat_dau)?)),
            TokKind::TuKhoa(TuKhoa::Enum) => Ok(Muc::Enum(self.khai_bao_enum(cong_khai, bat_dau)?)),
            TokKind::TuKhoa(TuKhoa::Impl) => Ok(Muc::Impl(self.khai_bao_impl(bat_dau)?)),
            TokKind::TuKhoa(TuKhoa::Trait) => Ok(Muc::Trait(self.khai_bao_trait(cong_khai, bat_dau)?)),
            TokKind::TuKhoa(TuKhoa::Const) => self.khai_bao_const(cong_khai, bat_dau),
            TokKind::TuKhoa(TuKhoa::Use) => self.khai_bao_use(bat_dau),
            khac => {
                let sp = self.span();
                let mut d = Diagnostic::loi("BR0103", "chỉ được khai báo ở ngoài cùng của chương trình")
                    .tai(sp, format!("gặp {} ở đây", khac.mo_ta()))
                    .vi_sao("ngoài cùng của một file Rust chỉ chứa khai báo: `fn`, `struct`, `enum`, `impl`, `trait`, `const`, `use`")
                    .khai_niem("cấu trúc chương trình");
                if matches!(khac, TokKind::TuKhoa(TuKhoa::Let)) {
                    d = d.sua("`let` chỉ dùng được bên trong hàm — hãy đặt nó vào trong `fn main() { ... }`");
                } else {
                    d = d.sua("bọc đoạn mã này vào trong `fn main() { ... }`");
                }
                self.diags.push(d);
                Err(())
            }
        }
    }

    fn tham_so_kieu(&mut self) -> KetQua<Vec<String>> {
        if !self.khop(&TokKind::NhoHon) {
            return Ok(Vec::new());
        }
        let mut ra = Vec::new();
        loop {
            if self.la(&TokKind::LonHon) {
                break;
            }
            let (t, _) = self.can_ten("khai báo tham số kiểu")?;
            ra.push(t);
            if !self.khop(&TokKind::Phay) {
                break;
            }
        }
        self.can(TokKind::LonHon, "đóng danh sách tham số kiểu")?;
        Ok(ra)
    }

    fn ham(&mut self, cong_khai: bool, bat_dau: Span) -> KetQua<Ham> {
        self.tien(); // fn
        let (ten, _) = self.can_ten("khai báo hàm")?;
        let tham_so_kieu = self.tham_so_kieu()?;
        self.can(TokKind::MoTron, "khai báo danh sách tham số của hàm")?;

        let mut tham_so = Vec::new();
        while !self.la(&TokKind::DongTron) && !self.het() {
            let sp0 = self.span();
            // `self`, `&self`, `&mut self` trong impl
            if self.la(&TokKind::TuKhoa(TuKhoa::SelfThuong))
                || (self.la(&TokKind::VaBit)
                    && matches!(
                        self.xem_n(1),
                        TokKind::TuKhoa(TuKhoa::SelfThuong) | TokKind::TuKhoa(TuKhoa::Mut)
                    ))
            {
                let co_muon = self.khop(&TokKind::VaBit);
                let co_the_sua = self.khop(&TokKind::TuKhoa(TuKhoa::Mut));
                self.can(TokKind::TuKhoa(TuKhoa::SelfThuong), "khai báo tham số `self`")?;
                let sp = sp0.merge(self.span_truoc());
                let kieu_self = Kieu::DuongDan { doan: vec!["Self".into()], tham_so: vec![], span: sp };
                let kieu = if co_muon {
                    Kieu::ThamChieu { co_the_sua, ben_trong: Box::new(kieu_self), span: sp }
                } else {
                    kieu_self
                };
                tham_so.push(ThamSo {
                    mau: Mau::Ten { ten: "self".into(), co_the_sua: false, la_ref: false, span: sp },
                    kieu,
                    span: sp,
                });
            } else {
                let mau = self.mau()?;
                self.can(TokKind::HaiChamDung, "khai báo kiểu cho tham số")?;
                let kieu = self.kieu()?;
                tham_so.push(ThamSo { span: sp0.merge(self.span_truoc()), mau, kieu });
            }
            if !self.khop(&TokKind::Phay) {
                break;
            }
        }
        self.can(TokKind::DongTron, "đóng danh sách tham số của hàm")?;

        let kieu_tra_ve = if self.khop(&TokKind::MuiTen) {
            Some(self.kieu()?)
        } else {
            None
        };

        let than = self.khoi()?;
        Ok(Ham { ten, tham_so_kieu, tham_so, kieu_tra_ve, span: bat_dau.merge(than.span), than, cong_khai })
    }

    fn than_struct(&mut self) -> KetQua<ThanStruct> {
        if self.khop(&TokKind::MoNhon) {
            let mut truong = Vec::new();
            while !self.la(&TokKind::DongNhon) && !self.het() {
                let ck = self.khop(&TokKind::TuKhoa(TuKhoa::Pub));
                let (ten, _) = self.can_ten("khai báo trường của struct")?;
                self.can(TokKind::HaiChamDung, "khai báo kiểu cho trường")?;
                let kieu = self.kieu()?;
                truong.push((ten, kieu, ck));
                if !self.khop(&TokKind::Phay) {
                    break;
                }
            }
            self.can(TokKind::DongNhon, "đóng thân struct")?;
            Ok(ThanStruct::TheoTen(truong))
        } else if self.khop(&TokKind::MoTron) {
            let mut truong = Vec::new();
            while !self.la(&TokKind::DongTron) && !self.het() {
                let ck = self.khop(&TokKind::TuKhoa(TuKhoa::Pub));
                truong.push((self.kieu()?, ck));
                if !self.khop(&TokKind::Phay) {
                    break;
                }
            }
            self.can(TokKind::DongTron, "đóng struct dạng tuple")?;
            Ok(ThanStruct::TheoViTri(truong))
        } else {
            Ok(ThanStruct::Rong)
        }
    }

    fn khai_bao_struct(&mut self, cong_khai: bool, bat_dau: Span) -> KetQua<Struct> {
        self.tien(); // struct
        let (ten, _) = self.can_ten("khai báo struct")?;
        let tham_so_kieu = self.tham_so_kieu()?;
        let than = self.than_struct()?;
        if matches!(than, ThanStruct::Rong | ThanStruct::TheoViTri(_)) {
            self.khop(&TokKind::ChamPhay);
        }
        Ok(Struct { ten, tham_so_kieu, than, cong_khai, span: bat_dau.merge(self.span_truoc()) })
    }

    fn khai_bao_enum(&mut self, cong_khai: bool, bat_dau: Span) -> KetQua<Enum> {
        self.tien(); // enum
        let (ten, _) = self.can_ten("khai báo enum")?;
        let tham_so_kieu = self.tham_so_kieu()?;
        self.can(TokKind::MoNhon, "mở thân enum")?;
        let mut bien_the = Vec::new();
        while !self.la(&TokKind::DongNhon) && !self.het() {
            let sp0 = self.span();
            let (vt, _) = self.can_ten("khai báo biến thể của enum")?;
            let than = self.than_struct()?;
            bien_the.push(BienThe { ten: vt, than, span: sp0.merge(self.span_truoc()) });
            if !self.khop(&TokKind::Phay) {
                break;
            }
        }
        self.can(TokKind::DongNhon, "đóng thân enum")?;
        Ok(Enum { ten, tham_so_kieu, bien_the, cong_khai, span: bat_dau.merge(self.span_truoc()) })
    }

    fn khai_bao_impl(&mut self, bat_dau: Span) -> KetQua<Impl> {
        self.tien(); // impl
        let tham_so_kieu = self.tham_so_kieu()?;
        let dau = self.kieu()?;
        let (trait_ten, kieu) = if self.khop(&TokKind::TuKhoa(TuKhoa::For)) {
            let k = self.kieu()?;
            let ten = match &dau {
                Kieu::DuongDan { doan, .. } => Some(doan.clone()),
                _ => None,
            };
            (ten, k)
        } else {
            (None, dau)
        };
        self.can(TokKind::MoNhon, "mở thân impl")?;
        let mut ham = Vec::new();
        while !self.la(&TokKind::DongNhon) && !self.het() {
            let sp = self.span();
            let ck = self.khop(&TokKind::TuKhoa(TuKhoa::Pub));
            if self.la(&TokKind::TuKhoa(TuKhoa::Fn)) {
                match self.ham(ck, sp) {
                    Ok(h) => ham.push(h),
                    Err(()) => self.dong_bo(),
                }
            } else {
                self.diags.push(
                    Diagnostic::loi("BR0104", "trong `impl` chỉ được khai báo hàm")
                        .tai(self.span(), format!("gặp {} ở đây", self.xem().mo_ta()))
                        .vi_sao("`impl` gom các phương thức của một kiểu; mọi thứ bên trong phải bắt đầu bằng `fn`")
                        .khai_niem("impl"),
                );
                self.dong_bo();
            }
        }
        self.can(TokKind::DongNhon, "đóng thân impl")?;
        Ok(Impl { trait_ten, kieu, tham_so_kieu, ham, span: bat_dau.merge(self.span_truoc()) })
    }

    fn khai_bao_trait(&mut self, cong_khai: bool, bat_dau: Span) -> KetQua<Trait> {
        self.tien(); // trait
        let (ten, _) = self.can_ten("khai báo trait")?;
        let tham_so_kieu = self.tham_so_kieu()?;
        self.can(TokKind::MoNhon, "mở thân trait")?;
        let mut ham = Vec::new();
        while !self.la(&TokKind::DongNhon) && !self.het() {
            let sp = self.span();
            if !self.la(&TokKind::TuKhoa(TuKhoa::Fn)) {
                self.diags.push(
                    Diagnostic::loi("BR0105", "trong `trait` chỉ được khai báo hàm")
                        .tai(self.span(), format!("gặp {} ở đây", self.xem().mo_ta()))
                        .khai_niem("trait"),
                );
                self.dong_bo();
                continue;
            }
            // Chữ ký không có thân: `fn ten(&self) -> T;`
            let luu = self.i;
            self.tien(); // fn
            let Ok((ten_ham, _)) = self.can_ten("khai báo phương thức của trait") else {
                self.dong_bo();
                continue;
            };
            self.i = luu;
            let _ = ten_ham;
            match self.ham_hoac_chu_ky(sp) {
                Ok(x) => ham.push(x),
                Err(()) => self.dong_bo(),
            }
        }
        self.can(TokKind::DongNhon, "đóng thân trait")?;
        Ok(Trait { ten, tham_so_kieu, ham, cong_khai, span: bat_dau.merge(self.span_truoc()) })
    }

    /// Phương thức trong trait: có thể có thân (mặc định) hoặc chỉ có chữ ký.
    fn ham_hoac_chu_ky(&mut self, bat_dau: Span) -> KetQua<(Ham, bool)> {
        let luu = self.i;
        // Thử đọc như hàm đầy đủ.
        let luu_diag = self.diags.len();
        if let Ok(h) = self.ham(false, bat_dau) {
            return Ok((h, true));
        }
        // Không được thì đọc như chữ ký kết thúc bằng `;`.
        self.i = luu;
        while self.diags.len() > luu_diag {
            self.diags = Diagnostics::from_vec({
                let mut v = self.diags.clone().into_vec();
                v.pop();
                v
            });
        }
        self.tien(); // fn
        let (ten, _) = self.can_ten("khai báo phương thức của trait")?;
        let tham_so_kieu = self.tham_so_kieu()?;
        self.can(TokKind::MoTron, "khai báo tham số")?;
        let mut do_sau = 1;
        while do_sau > 0 && !self.het() {
            match self.xem() {
                TokKind::MoTron => do_sau += 1,
                TokKind::DongTron => do_sau -= 1,
                _ => {}
            }
            self.tien();
        }
        let kieu_tra_ve = if self.khop(&TokKind::MuiTen) { Some(self.kieu()?) } else { None };
        self.can(TokKind::ChamPhay, "kết thúc chữ ký phương thức")?;
        let sp = bat_dau.merge(self.span_truoc());
        Ok((
            Ham {
                ten,
                tham_so_kieu,
                tham_so: Vec::new(),
                kieu_tra_ve,
                than: Khoi { cau_lenh: vec![], gia_tri_cuoi: None, span: sp },
                cong_khai: false,
                span: sp,
            },
            false,
        ))
    }

    fn khai_bao_const(&mut self, cong_khai: bool, bat_dau: Span) -> KetQua<Muc> {
        self.tien(); // const
        let (ten, _) = self.can_ten("khai báo hằng")?;
        self.can(TokKind::HaiChamDung, "khai báo kiểu cho hằng")?;
        let kieu = self.kieu()?;
        self.can(TokKind::Gan, "gán giá trị cho hằng")?;
        let gia_tri = self.bieu_thuc()?;
        self.can(TokKind::ChamPhay, "kết thúc khai báo hằng")?;
        Ok(Muc::Const { ten, kieu, gia_tri, cong_khai, span: bat_dau.merge(self.span_truoc()) })
    }

    fn khai_bao_use(&mut self, bat_dau: Span) -> KetQua<Muc> {
        self.tien(); // use
        let mut doan = Vec::new();
        loop {
            match self.xem().clone() {
                TokKind::Ten(t) => {
                    doan.push(t);
                    self.tien();
                }
                TokKind::Nhan => {
                    doan.push("*".into());
                    self.tien();
                }
                TokKind::MoNhon => {
                    // `use a::{b, c};` — bỏ qua nội dung, chưa hỗ trợ module thật.
                    let mut do_sau = 0;
                    loop {
                        match self.xem() {
                            TokKind::MoNhon => do_sau += 1,
                            TokKind::DongNhon => {
                                do_sau -= 1;
                                if do_sau == 0 {
                                    self.tien();
                                    break;
                                }
                            }
                            TokKind::HetTep => break,
                            _ => {}
                        }
                        self.tien();
                    }
                }
                _ => break,
            }
            if !self.khop(&TokKind::DuongDan) {
                break;
            }
        }
        self.can(TokKind::ChamPhay, "kết thúc khai báo `use`")?;
        Ok(Muc::Use { doan, span: bat_dau.merge(self.span_truoc()) })
    }

    // ── Kiểu ────────────────────────────────────────────────────────────────

    fn kieu(&mut self) -> KetQua<Kieu> {
        let bat_dau = self.span();
        match self.xem().clone() {
            TokKind::VaBit => {
                self.tien();
                let co_the_sua = self.khop(&TokKind::TuKhoa(TuKhoa::Mut));
                let ben_trong = Box::new(self.kieu()?);
                Ok(Kieu::ThamChieu { co_the_sua, ben_trong, span: bat_dau.merge(self.span_truoc()) })
            }
            TokKind::GachDuoi => {
                self.tien();
                Ok(Kieu::SuyLuan { span: bat_dau })
            }
            TokKind::MoTron => {
                self.tien();
                let mut phan_tu = Vec::new();
                while !self.la(&TokKind::DongTron) && !self.het() {
                    phan_tu.push(self.kieu()?);
                    if !self.khop(&TokKind::Phay) {
                        break;
                    }
                }
                self.can(TokKind::DongTron, "đóng kiểu tuple")?;
                Ok(Kieu::Tuple { phan_tu, span: bat_dau.merge(self.span_truoc()) })
            }
            TokKind::MoVuong => {
                self.tien();
                let phan_tu = Box::new(self.kieu()?);
                let so_luong = if self.khop(&TokKind::ChamPhay) {
                    match self.xem().clone() {
                        TokKind::SoNguyen(n) => {
                            self.tien();
                            Some(n as u64)
                        }
                        _ => None,
                    }
                } else {
                    None
                };
                self.can(TokKind::DongVuong, "đóng kiểu mảng")?;
                Ok(Kieu::Mang { phan_tu, so_luong, span: bat_dau.merge(self.span_truoc()) })
            }
            TokKind::Ten(_) | TokKind::TuKhoa(TuKhoa::SelfHoa) => {
                let mut doan = Vec::new();
                loop {
                    match self.xem().clone() {
                        TokKind::Ten(t) => {
                            doan.push(t);
                            self.tien();
                        }
                        TokKind::TuKhoa(TuKhoa::SelfHoa) => {
                            doan.push("Self".into());
                            self.tien();
                        }
                        _ => break,
                    }
                    if !self.khop(&TokKind::DuongDan) {
                        break;
                    }
                }
                let mut tham_so = Vec::new();
                if self.khop(&TokKind::NhoHon) {
                    while !self.la(&TokKind::LonHon) && !self.het() {
                        tham_so.push(self.kieu()?);
                        if !self.khop(&TokKind::Phay) {
                            break;
                        }
                    }
                    self.can(TokKind::LonHon, "đóng tham số kiểu")?;
                }
                Ok(Kieu::DuongDan { doan, tham_so, span: bat_dau.merge(self.span_truoc()) })
            }
            khac => {
                self.diags.push(
                    Diagnostic::loi("BR0106", "cần một kiểu ở đây")
                        .tai(self.span(), format!("gặp {} thay vì tên kiểu", khac.mo_ta()))
                        .vi_sao("Rust cần biết kiểu dữ liệu ở vị trí này để kiểm tra chương trình lúc biên dịch")
                        .sua("ví dụ kiểu thường dùng: `i64` (số nguyên), `f64` (số thực), `bool`, `String`, `Vec<i64>`")
                        .khai_niem("kiểu dữ liệu"),
                );
                Err(())
            }
        }
    }
}

impl Diagnostics {
    fn from_vec(v: Vec<Diagnostic>) -> Self {
        let mut d = Diagnostics::new();
        for x in v {
            d.push(x);
        }
        d
    }
}

// ── Mẫu, câu lệnh, biểu thức ────────────────────────────────────────────────

impl Parser {
    fn mau(&mut self) -> KetQua<Mau> {
        let dau = self.mau_don()?;
        if !self.la(&TokKind::HoacBit) {
            return Ok(dau);
        }
        let mut nhanh = vec![dau];
        while self.khop(&TokKind::HoacBit) {
            nhanh.push(self.mau_don()?);
        }
        let span = nhanh[0].span().merge(nhanh.last().unwrap().span());
        Ok(Mau::Hoac { nhanh, span })
    }

    fn mau_don(&mut self) -> KetQua<Mau> {
        let bat_dau = self.span();
        let mau = match self.xem().clone() {
            TokKind::GachDuoi => {
                self.tien();
                Mau::BoQua { span: bat_dau }
            }
            TokKind::TuKhoa(TuKhoa::Mut) => {
                self.tien();
                let (ten, sp) = self.can_ten("khai báo biến có thể thay đổi")?;
                Mau::Ten { ten, co_the_sua: true, la_ref: false, span: bat_dau.merge(sp) }
            }
            TokKind::TuKhoa(TuKhoa::Ref) => {
                self.tien();
                let co_the_sua = self.khop(&TokKind::TuKhoa(TuKhoa::Mut));
                let (ten, sp) = self.can_ten("khai báo mẫu tham chiếu")?;
                Mau::Ten { ten, co_the_sua, la_ref: true, span: bat_dau.merge(sp) }
            }
            TokKind::SoNguyen(_) | TokKind::SoThuc(_) | TokKind::ChuoiVanBan(_)
            | TokKind::KyTu(_) | TokKind::DungSai(_) | TokKind::Tru => {
                let am = self.khop(&TokKind::Tru);
                let hs = match self.xem().clone() {
                    TokKind::SoNguyen(n) => HangSo::SoNguyen(if am { -n } else { n }),
                    TokKind::SoThuc(f) => HangSo::SoThuc(if am { -f } else { f }),
                    TokKind::ChuoiVanBan(s) => HangSo::Chuoi(s),
                    TokKind::KyTu(c) => HangSo::KyTu(c),
                    TokKind::DungSai(b) => HangSo::DungSai(b),
                    _ => {
                        self.diags.push(
                            Diagnostic::loi("BR0107", "cần một giá trị sau dấu `-`")
                                .tai(self.span(), "chỗ này cần một con số"),
                        );
                        return Err(());
                    }
                };
                self.tien();
                Mau::HangSo { gia_tri: hs, span: bat_dau.merge(self.span_truoc()) }
            }
            TokKind::MoTron => {
                self.tien();
                let mut phan_tu = Vec::new();
                while !self.la(&TokKind::DongTron) && !self.het() {
                    phan_tu.push(self.mau()?);
                    if !self.khop(&TokKind::Phay) {
                        break;
                    }
                }
                self.can(TokKind::DongTron, "đóng mẫu tuple")?;
                Mau::Tuple { phan_tu, span: bat_dau.merge(self.span_truoc()) }
            }
            TokKind::Ten(_) => {
                let mut doan = Vec::new();
                loop {
                    match self.xem().clone() {
                        TokKind::Ten(t) => {
                            doan.push(t);
                            self.tien();
                        }
                        _ => break,
                    }
                    if !self.khop(&TokKind::DuongDan) {
                        break;
                    }
                }
                // Tên đơn viết thường, không kèm gì → ràng buộc biến.
                let don_gian = doan.len() == 1
                    && !self.la(&TokKind::MoTron)
                    && !self.la(&TokKind::MoNhon)
                    && doan[0].chars().next().is_some_and(|c| c.is_lowercase() || c == '_');
                if don_gian {
                    Mau::Ten {
                        ten: doan.pop().unwrap(),
                        co_the_sua: false,
                        la_ref: false,
                        span: bat_dau.merge(self.span_truoc()),
                    }
                } else {
                    let truong = if self.khop(&TokKind::MoTron) {
                        let mut ps = Vec::new();
                        while !self.la(&TokKind::DongTron) && !self.het() {
                            ps.push(self.mau()?);
                            if !self.khop(&TokKind::Phay) {
                                break;
                            }
                        }
                        self.can(TokKind::DongTron, "đóng mẫu biến thể")?;
                        MauTruong::TheoViTri(ps)
                    } else if self.khop(&TokKind::MoNhon) {
                        let mut ts = Vec::new();
                        let mut ba_cham = false;
                        while !self.la(&TokKind::DongNhon) && !self.het() {
                            if self.khop(&TokKind::HaiCham) {
                                ba_cham = true;
                                break;
                            }
                            let (ten, sp) = self.can_ten("khai báo mẫu theo tên trường")?;
                            let p = if self.khop(&TokKind::HaiChamDung) {
                                self.mau()?
                            } else {
                                Mau::Ten { ten: ten.clone(), co_the_sua: false, la_ref: false, span: sp }
                            };
                            ts.push((ten, p));
                            if !self.khop(&TokKind::Phay) {
                                break;
                            }
                        }
                        self.can(TokKind::DongNhon, "đóng mẫu struct")?;
                        MauTruong::TheoTen { truong: ts, dau_ba_cham: ba_cham }
                    } else {
                        MauTruong::Khong
                    };
                    Mau::BienThe { duong_dan: doan, truong, span: bat_dau.merge(self.span_truoc()) }
                }
            }
            TokKind::TuKhoa(k) => {
                let sp = self.span();
                self.tien();
                self.diags.push(
                    Diagnostic::loi("BR0102", format!("`{}` là từ khoá, không dùng làm tên biến được", k.ten()))
                        .tai(sp, "từ khoá không thể đứng ở đây")
                        .vi_sao("Rust dành riêng một số từ cho cú pháp, nên chúng không thể vừa là tên biến vừa là từ khoá")
                        .sua(format!("đổi sang tên khác, ví dụ `{}_` hoặc một tên mô tả rõ hơn", k.ten()))
                        .khai_niem("đặt tên"),
                );
                return Err(());
            }
            khac => {
                self.diags.push(
                    Diagnostic::loi("BR0108", "cần một mẫu ở đây")
                        .tai(self.span(), format!("gặp {} thay vì một mẫu", khac.mo_ta()))
                        .vi_sao("mẫu (pattern) là thứ nằm bên trái dấu `=` của `let`, hoặc trước `=>` trong `match`")
                        .sua("ví dụ: tên biến `x`, dấu bỏ qua `_`, hay bộ đôi `(a, b)`")
                        .khai_niem("mẫu"),
                );
                return Err(());
            }
        };

        // Dải: `1..=5`
        if self.la(&TokKind::HaiCham) || self.la(&TokKind::HaiChamBang) {
            let bao_gom_cuoi = self.la(&TokKind::HaiChamBang);
            self.tien();
            let den = self.mau_don()?;
            let span = mau.span().merge(den.span());
            return Ok(Mau::Dai { tu: Box::new(mau), den: Box::new(den), bao_gom_cuoi, span });
        }
        Ok(mau)
    }

    fn khoi(&mut self) -> KetQua<Khoi> {
        let bat_dau = self.span();
        self.can(TokKind::MoNhon, "mở một khối lệnh")?;
        let cam = core::mem::replace(&mut self.cam_struct_literal, false);

        let mut cau_lenh = Vec::new();
        let mut gia_tri_cuoi = None;

        while !self.la(&TokKind::DongNhon) && !self.het() {
            // Khai báo lồng bên trong hàm
            if matches!(
                self.xem(),
                TokKind::TuKhoa(
                    TuKhoa::Fn | TuKhoa::Struct | TuKhoa::Enum | TuKhoa::Impl
                    | TuKhoa::Trait | TuKhoa::Use | TuKhoa::Const
                )
            ) {
                match self.muc() {
                    Ok(m) => cau_lenh.push(CauLenh::Muc(m)),
                    Err(()) => self.dong_bo(),
                }
                continue;
            }

            if self.la(&TokKind::TuKhoa(TuKhoa::Let)) {
                match self.cau_lenh_let() {
                    Ok(s) => cau_lenh.push(s),
                    Err(()) => self.dong_bo(),
                }
                continue;
            }

            if self.khop(&TokKind::ChamPhay) {
                continue; // `;` thừa — vô hại
            }

            let sp0 = self.span();
            let bt = match self.bieu_thuc() {
                Ok(b) => b,
                Err(()) => {
                    self.dong_bo();
                    continue;
                }
            };

            if self.khop(&TokKind::ChamPhay) {
                cau_lenh.push(CauLenh::BieuThuc { span: sp0.merge(self.span_truoc()), bt });
            } else if self.la(&TokKind::DongNhon) {
                gia_tri_cuoi = Some(bt);
                break;
            } else if la_bieu_thuc_khoi(&bt) {
                // `if`, `match`, `loop`, `while`, `for` dùng như câu lệnh thì không cần `;`
                cau_lenh.push(CauLenh::BieuThuc { span: sp0.merge(self.span_truoc()), bt });
            } else {
                let thay = self.xem().mo_ta();
                self.diags.push(
                    Diagnostic::loi("BR0109", "thiếu dấu `;` ở cuối câu lệnh")
                        .nhan(Label::chinh(
                            Span::at(self.span_truoc().end),
                            "cần dấu `;` ở đây",
                        ))
                        .nhan(Label::phu(self.span(), format!("nên {thay} bị coi là tiếp nối của câu trên")))
                        .vi_sao("trong Rust, mỗi câu lệnh kết thúc bằng `;`. Biểu thức KHÔNG có `;` ở cuối khối chính là giá trị trả về của khối.")
                        .sua("thêm `;` vào cuối dòng")
                        .khai_niem("câu lệnh"),
                );
                cau_lenh.push(CauLenh::BieuThuc { span: sp0.merge(self.span_truoc()), bt });
            }
        }

        self.cam_struct_literal = cam;
        self.can(TokKind::DongNhon, "đóng khối lệnh")?;
        Ok(Khoi { cau_lenh, gia_tri_cuoi, span: bat_dau.merge(self.span_truoc()) })
    }

    fn cau_lenh_let(&mut self) -> KetQua<CauLenh> {
        let bat_dau = self.span();
        self.tien(); // let
        let mau = self.mau()?;
        let kieu = if self.khop(&TokKind::HaiChamDung) {
            Some(self.kieu()?)
        } else {
            None
        };
        let gia_tri = if self.khop(&TokKind::Gan) {
            Some(self.bieu_thuc()?)
        } else {
            None
        };
        if !self.khop(&TokKind::ChamPhay) {
            let thay = self.xem().mo_ta();
            self.diags.push(
                Diagnostic::loi("BR0109", "thiếu dấu `;` ở cuối câu lệnh")
                    .nhan(Label::chinh(Span::at(self.span_truoc().end), "cần dấu `;` ở đây"))
                    .nhan(Label::phu(self.span(), format!("nên {thay} bị coi là tiếp nối của câu trên")))
                    .vi_sao("mỗi câu lệnh `let` phải kết thúc bằng dấu `;`")
                    .sua("thêm `;` vào cuối dòng")
                    .khai_niem("câu lệnh"),
            );
            return Err(());
        }
        Ok(CauLenh::Let { mau, kieu, gia_tri, span: bat_dau.merge(self.span_truoc()) })
    }

    // ── Biểu thức: leo độ ưu tiên ───────────────────────────────────────────

    pub fn bieu_thuc(&mut self) -> KetQua<BieuThuc> {
        self.bieu_thuc_gan()
    }

    fn bieu_thuc_gan(&mut self) -> KetQua<BieuThuc> {
        let trai = self.bieu_thuc_dai()?;

        let toan_tu = match self.xem() {
            TokKind::Gan => None,
            TokKind::CongGan => Some(ToanTuHai::Cong),
            TokKind::TruGan => Some(ToanTuHai::Tru),
            TokKind::NhanGan => Some(ToanTuHai::Nhan),
            TokKind::ChiaGan => Some(ToanTuHai::Chia),
            TokKind::DuGan => Some(ToanTuHai::Du),
            _ => return Ok(trai),
        };
        self.tien();
        let gia_tri = self.bieu_thuc_gan()?;
        let span = trai.span().merge(gia_tri.span());
        Ok(BieuThuc::Gan { dich: Box::new(trai), toan_tu, gia_tri: Box::new(gia_tri), span })
    }

    fn bieu_thuc_dai(&mut self) -> KetQua<BieuThuc> {
        // Dải mở đầu: `..5`
        if self.la(&TokKind::HaiCham) || self.la(&TokKind::HaiChamBang) {
            let bat_dau = self.span();
            let bao_gom_cuoi = self.la(&TokKind::HaiChamBang);
            self.tien();
            let den = if self.bat_dau_bieu_thuc() {
                Some(Box::new(self.bieu_thuc_nhi_phan(0)?))
            } else {
                None
            };
            return Ok(BieuThuc::Dai { tu: None, den, bao_gom_cuoi, span: bat_dau.merge(self.span_truoc()) });
        }

        let trai = self.bieu_thuc_nhi_phan(0)?;
        if self.la(&TokKind::HaiCham) || self.la(&TokKind::HaiChamBang) {
            let bao_gom_cuoi = self.la(&TokKind::HaiChamBang);
            self.tien();
            let den = if self.bat_dau_bieu_thuc() {
                Some(Box::new(self.bieu_thuc_nhi_phan(0)?))
            } else {
                None
            };
            let span = trai.span().merge(self.span_truoc());
            return Ok(BieuThuc::Dai { tu: Some(Box::new(trai)), den, bao_gom_cuoi, span });
        }
        Ok(trai)
    }

    fn bat_dau_bieu_thuc(&self) -> bool {
        matches!(
            self.xem(),
            TokKind::SoNguyen(_) | TokKind::SoThuc(_) | TokKind::ChuoiVanBan(_)
                | TokKind::KyTu(_) | TokKind::DungSai(_) | TokKind::Ten(_)
                | TokKind::MoTron | TokKind::MoVuong | TokKind::MoNhon
                | TokKind::Tru | TokKind::Phu | TokKind::VaBit | TokKind::Nhan
                | TokKind::TuKhoa(_)
        )
    }

    fn toan_tu_hai(&self) -> Option<ToanTuHai> {
        use ToanTuHai::*;
        Some(match self.xem() {
            TokKind::Cong => Cong, TokKind::Tru => Tru,
            TokKind::Nhan => Nhan, TokKind::Chia => Chia, TokKind::Du => Du,
            TokKind::Bang => Bang, TokKind::KhacBang => KhacBang,
            TokKind::NhoHon => NhoHon, TokKind::LonHon => LonHon,
            TokKind::NhoBang => NhoBang, TokKind::LonBang => LonBang,
            TokKind::Va => Va, TokKind::Hoac => Hoac,
            TokKind::VaBit => VaBit, TokKind::HoacBit => HoacBit, TokKind::XorBit => XorBit,
            TokKind::DichTrai => DichTrai, TokKind::DichPhai => DichPhai,
            _ => return None,
        })
    }

    fn bieu_thuc_nhi_phan(&mut self, uu_tien_toi_thieu: u8) -> KetQua<BieuThuc> {
        let mut trai = self.bieu_thuc_mot_ngoi()?;

        while let Some(op) = self.toan_tu_hai() {
            let ut = op.uu_tien();
            if ut < uu_tien_toi_thieu {
                break;
            }

            // `x = 1` ở chỗ cần `x == 1` — lỗi kinh điển của người mới.
            if op == ToanTuHai::Bang && self.xem_n(1) == &TokKind::Gan {
                // `===` không tồn tại trong Rust
                self.diags.push(
                    Diagnostic::loi("BR0110", "Rust không có toán tử `===`")
                        .tai(self.span(), "chỉ cần `==` là đủ")
                        .vi_sao("`===` là của JavaScript. Trong Rust, `==` đã so sánh theo giá trị và có kiểm tra kiểu lúc biên dịch.")
                        .sua("đổi `===` thành `==`")
                        .khai_niem("so sánh"),
                );
            }

            self.tien();
            let phai = self.bieu_thuc_nhi_phan(ut + 1)?;
            let span = trai.span().merge(phai.span());
            trai = BieuThuc::HaiNgoi { toan_tu: op, trai: Box::new(trai), phai: Box::new(phai), span };
        }
        Ok(trai)
    }

    fn bieu_thuc_mot_ngoi(&mut self) -> KetQua<BieuThuc> {
        let bat_dau = self.span();
        match self.xem().clone() {
            TokKind::Tru => {
                self.tien();
                let t = self.bieu_thuc_mot_ngoi()?;
                Ok(BieuThuc::MotNgoi { toan_tu: ToanTuMot::Am, span: bat_dau.merge(t.span()), toan_hang: Box::new(t) })
            }
            TokKind::Phu => {
                self.tien();
                let t = self.bieu_thuc_mot_ngoi()?;
                Ok(BieuThuc::MotNgoi { toan_tu: ToanTuMot::Phu, span: bat_dau.merge(t.span()), toan_hang: Box::new(t) })
            }
            TokKind::VaBit => {
                self.tien();
                let co_the_sua = self.khop(&TokKind::TuKhoa(TuKhoa::Mut));
                let t = self.bieu_thuc_mot_ngoi()?;
                Ok(BieuThuc::Muon { co_the_sua, span: bat_dau.merge(t.span()), gia_tri: Box::new(t) })
            }
            TokKind::Nhan => {
                self.tien();
                let t = self.bieu_thuc_mot_ngoi()?;
                Ok(BieuThuc::GiaiTham { span: bat_dau.merge(t.span()), gia_tri: Box::new(t) })
            }
            _ => self.bieu_thuc_hau_to(),
        }
    }

    fn bieu_thuc_hau_to(&mut self) -> KetQua<BieuThuc> {
        let mut bt = self.bieu_thuc_co_ban()?;
        loop {
            match self.xem().clone() {
                TokKind::Cham => {
                    self.tien();
                    match self.xem().clone() {
                        TokKind::Ten(ten) => {
                            self.tien();
                            if self.khop(&TokKind::MoTron) {
                                let doi_so = self.danh_sach_doi_so()?;
                                let span = bt.span().merge(self.span_truoc());
                                bt = BieuThuc::GoiPhuongThuc { doi_tuong: Box::new(bt), ten, doi_so, span };
                            } else {
                                let span = bt.span().merge(self.span_truoc());
                                bt = BieuThuc::TruyCapTruong { doi_tuong: Box::new(bt), ten, span };
                            }
                        }
                        TokKind::SoNguyen(n) => {
                            self.tien();
                            let span = bt.span().merge(self.span_truoc());
                            bt = BieuThuc::TruyCapTruong { doi_tuong: Box::new(bt), ten: n.to_string(), span };
                        }
                        khac => {
                            self.diags.push(
                                Diagnostic::loi("BR0111", "sau dấu `.` cần tên trường hoặc tên phương thức")
                                    .tai(self.span(), format!("gặp {}", khac.mo_ta()))
                                    .vi_sao("dấu `.` dùng để lấy một trường (`p.x`) hoặc gọi một phương thức (`v.len()`)")
                                    .khai_niem("truy cập trường"),
                            );
                            return Err(());
                        }
                    }
                }
                TokKind::MoTron => {
                    self.tien();
                    let doi_so = self.danh_sach_doi_so()?;
                    let span = bt.span().merge(self.span_truoc());
                    bt = BieuThuc::GoiHam { ham: Box::new(bt), doi_so, span };
                }
                TokKind::MoVuong => {
                    self.tien();
                    let cam = core::mem::replace(&mut self.cam_struct_literal, false);
                    let chi_so = self.bieu_thuc()?;
                    self.cam_struct_literal = cam;
                    self.can(TokKind::DongVuong, "đóng phép lấy phần tử theo chỉ số")?;
                    let span = bt.span().merge(self.span_truoc());
                    bt = BieuThuc::ChiSo { doi_tuong: Box::new(bt), chi_so: Box::new(chi_so), span };
                }
                TokKind::Hoi => {
                    self.tien();
                    let span = bt.span().merge(self.span_truoc());
                    bt = BieuThuc::LanTruyenLoi { gia_tri: Box::new(bt), span };
                }
                TokKind::TuKhoa(TuKhoa::As) => {
                    self.tien();
                    let kieu = self.kieu()?;
                    let span = bt.span().merge(kieu.span());
                    bt = BieuThuc::Ep { gia_tri: Box::new(bt), kieu, span };
                }
                _ => break,
            }
        }
        Ok(bt)
    }

    fn danh_sach_doi_so(&mut self) -> KetQua<Vec<BieuThuc>> {
        let cam = core::mem::replace(&mut self.cam_struct_literal, false);
        let mut ra = Vec::new();
        while !self.la(&TokKind::DongTron) && !self.het() {
            ra.push(self.bieu_thuc()?);
            if !self.khop(&TokKind::Phay) {
                break;
            }
        }
        self.cam_struct_literal = cam;
        self.can(TokKind::DongTron, "đóng danh sách đối số")?;
        Ok(ra)
    }
}

fn la_bieu_thuc_khoi(bt: &BieuThuc) -> bool {
    matches!(
        bt,
        BieuThuc::Neu { .. }
            | BieuThuc::KhopMau { .. }
            | BieuThuc::Lap { .. }
            | BieuThuc::Trong { .. }
            | BieuThuc::Cho { .. }
            | BieuThuc::Khoi(_)
    )
}

// ── Biểu thức cơ bản ────────────────────────────────────────────────────────

impl Parser {
    fn bieu_thuc_co_ban(&mut self) -> KetQua<BieuThuc> {
        let bat_dau = self.span();
        match self.xem().clone() {
            TokKind::SoNguyen(n) => { self.tien(); Ok(BieuThuc::HangSo { gia_tri: HangSo::SoNguyen(n), span: bat_dau }) }
            TokKind::SoThuc(f)   => { self.tien(); Ok(BieuThuc::HangSo { gia_tri: HangSo::SoThuc(f), span: bat_dau }) }
            TokKind::ChuoiVanBan(s) => { self.tien(); Ok(BieuThuc::HangSo { gia_tri: HangSo::Chuoi(s), span: bat_dau }) }
            TokKind::KyTu(c)     => { self.tien(); Ok(BieuThuc::HangSo { gia_tri: HangSo::KyTu(c), span: bat_dau }) }
            TokKind::DungSai(b)  => { self.tien(); Ok(BieuThuc::HangSo { gia_tri: HangSo::DungSai(b), span: bat_dau }) }

            TokKind::MoTron => {
                self.tien();
                let cam = core::mem::replace(&mut self.cam_struct_literal, false);
                if self.khop(&TokKind::DongTron) {
                    self.cam_struct_literal = cam;
                    return Ok(BieuThuc::Tuple { phan_tu: vec![], span: bat_dau.merge(self.span_truoc()) });
                }
                let dau = self.bieu_thuc()?;
                if self.khop(&TokKind::Phay) {
                    let mut phan_tu = vec![dau];
                    while !self.la(&TokKind::DongTron) && !self.het() {
                        phan_tu.push(self.bieu_thuc()?);
                        if !self.khop(&TokKind::Phay) { break; }
                    }
                    self.cam_struct_literal = cam;
                    self.can(TokKind::DongTron, "đóng bộ giá trị (tuple)")?;
                    Ok(BieuThuc::Tuple { phan_tu, span: bat_dau.merge(self.span_truoc()) })
                } else {
                    self.cam_struct_literal = cam;
                    self.can(TokKind::DongTron, "đóng ngoặc đơn")?;
                    Ok(dau)
                }
            }

            TokKind::MoVuong => {
                self.tien();
                let cam = core::mem::replace(&mut self.cam_struct_literal, false);
                let mut phan_tu = Vec::new();
                let mut lap_lai = None;
                if !self.la(&TokKind::DongVuong) {
                    let dau = self.bieu_thuc()?;
                    if self.khop(&TokKind::ChamPhay) {
                        lap_lai = Some(Box::new(self.bieu_thuc()?));
                        phan_tu.push(dau);
                    } else {
                        phan_tu.push(dau);
                        while self.khop(&TokKind::Phay) {
                            if self.la(&TokKind::DongVuong) { break; }
                            phan_tu.push(self.bieu_thuc()?);
                        }
                    }
                }
                self.cam_struct_literal = cam;
                self.can(TokKind::DongVuong, "đóng mảng")?;
                Ok(BieuThuc::Mang { phan_tu, lap_lai, span: bat_dau.merge(self.span_truoc()) })
            }

            TokKind::MoNhon => Ok(BieuThuc::Khoi(Box::new(self.khoi()?))),

            TokKind::HoacBit | TokKind::Hoac => self.be_quan(bat_dau),

            TokKind::TuKhoa(TuKhoa::If)    => self.bieu_thuc_neu(bat_dau),
            TokKind::TuKhoa(TuKhoa::Match) => self.bieu_thuc_khop(bat_dau),
            TokKind::TuKhoa(TuKhoa::Loop)  => {
                self.tien();
                let than = self.khoi()?;
                Ok(BieuThuc::Lap { nhan: None, span: bat_dau.merge(than.span), than: Box::new(than) })
            }
            TokKind::TuKhoa(TuKhoa::While) => {
                self.tien();
                let cam = core::mem::replace(&mut self.cam_struct_literal, true);
                let dieu_kien = self.bieu_thuc()?;
                self.cam_struct_literal = cam;
                let than = self.khoi()?;
                Ok(BieuThuc::Trong { dieu_kien: Box::new(dieu_kien), span: bat_dau.merge(than.span), than: Box::new(than) })
            }
            TokKind::TuKhoa(TuKhoa::For) => {
                self.tien();
                let mau = self.mau()?;
                if !self.khop(&TokKind::TuKhoa(TuKhoa::In)) {
                    self.diags.push(
                        Diagnostic::loi("BR0112", "thiếu từ khoá `in` trong vòng lặp `for`")
                            .nhan(Label::chinh(Span::at(self.span_truoc().end), "cần `in` ở đây"))
                            .vi_sao("cú pháp vòng lặp của Rust là `for <biến> in <dãy>`, ví dụ `for i in 0..10`")
                            .sua("thêm `in` giữa tên biến và dãy giá trị")
                            .khai_niem("vòng lặp for"),
                    );
                    return Err(());
                }
                let cam = core::mem::replace(&mut self.cam_struct_literal, true);
                let day = self.bieu_thuc()?;
                self.cam_struct_literal = cam;
                let than = self.khoi()?;
                Ok(BieuThuc::Cho { mau, day: Box::new(day), span: bat_dau.merge(than.span), than: Box::new(than) })
            }

            TokKind::TuKhoa(TuKhoa::Return) => {
                self.tien();
                let gia_tri = if self.bat_dau_bieu_thuc() && !self.la(&TokKind::DongNhon) {
                    Some(Box::new(self.bieu_thuc()?))
                } else { None };
                Ok(BieuThuc::TraVe { gia_tri, span: bat_dau.merge(self.span_truoc()) })
            }
            TokKind::TuKhoa(TuKhoa::Break) => {
                self.tien();
                let gia_tri = if self.bat_dau_bieu_thuc() && !self.la(&TokKind::DongNhon) {
                    Some(Box::new(self.bieu_thuc()?))
                } else { None };
                Ok(BieuThuc::Thoat { nhan: None, gia_tri, span: bat_dau.merge(self.span_truoc()) })
            }
            TokKind::TuKhoa(TuKhoa::Continue) => {
                self.tien();
                Ok(BieuThuc::TiepTuc { nhan: None, span: bat_dau })
            }

            TokKind::Ten(_) | TokKind::TuKhoa(TuKhoa::SelfHoa) | TokKind::TuKhoa(TuKhoa::SelfThuong) => {
                let mut doan = Vec::new();
                loop {
                    match self.xem().clone() {
                        TokKind::Ten(t) => { doan.push(t); self.tien(); }
                        TokKind::TuKhoa(TuKhoa::SelfHoa) => { doan.push("Self".into()); self.tien(); }
                        TokKind::TuKhoa(TuKhoa::SelfThuong) => { doan.push("self".into()); self.tien(); }
                        _ => break,
                    }
                    if !self.khop(&TokKind::DuongDan) { break; }
                    // `Vec::<i64>::new` — bỏ qua turbofish
                    if self.la(&TokKind::NhoHon) {
                        let mut d = 0;
                        loop {
                            match self.xem() {
                                TokKind::NhoHon => d += 1,
                                TokKind::LonHon => { d -= 1; if d == 0 { self.tien(); break; } }
                                TokKind::HetTep => break,
                                _ => {}
                            }
                            self.tien();
                        }
                        if !self.khop(&TokKind::DuongDan) { break; }
                    }
                }

                // Macro: `println!(...)`
                if self.la(&TokKind::Phu) && doan.len() == 1 {
                    self.tien();
                    let mo = if self.khop(&TokKind::MoTron) { TokKind::DongTron }
                        else if self.khop(&TokKind::MoVuong) { TokKind::DongVuong }
                        else { self.can(TokKind::MoTron, "gọi macro")?; TokKind::DongTron };
                    let cam = core::mem::replace(&mut self.cam_struct_literal, false);
                    let mut doi_so = Vec::new();
                    while !self.la(&mo) && !self.het() {
                        doi_so.push(self.bieu_thuc()?);
                        if !self.khop(&TokKind::Phay) { break; }
                    }
                    self.cam_struct_literal = cam;
                    self.can(mo, "đóng lời gọi macro")?;
                    return Ok(BieuThuc::Macro { ten: doan.pop().unwrap(), doi_so, span: bat_dau.merge(self.span_truoc()) });
                }

                // Khởi tạo struct: `Point { x: 1 }`
                let bat_dau_hoa = doan.last().and_then(|s| s.chars().next()).is_some_and(|c| c.is_uppercase());
                if self.la(&TokKind::MoNhon) && !self.cam_struct_literal && bat_dau_hoa {
                    self.tien();
                    let mut truong = Vec::new();
                    let mut con_lai = None;
                    while !self.la(&TokKind::DongNhon) && !self.het() {
                        if self.khop(&TokKind::HaiCham) {
                            con_lai = Some(Box::new(self.bieu_thuc()?));
                            break;
                        }
                        let (ten, sp) = self.can_ten("khởi tạo trường của struct")?;
                        let gt = if self.khop(&TokKind::HaiChamDung) {
                            self.bieu_thuc()?
                        } else {
                            BieuThuc::DuongDan { doan: vec![ten.clone()], span: sp }
                        };
                        truong.push((ten, gt));
                        if !self.khop(&TokKind::Phay) { break; }
                    }
                    self.can(TokKind::DongNhon, "đóng phần khởi tạo struct")?;
                    return Ok(BieuThuc::KhoiTaoStruct { duong_dan: doan, truong, con_lai, span: bat_dau.merge(self.span_truoc()) });
                }

                Ok(BieuThuc::DuongDan { doan, span: bat_dau.merge(self.span_truoc()) })
            }

            khac => {
                let mut d = Diagnostic::loi("BR0113", "cần một giá trị ở đây")
                    .tai(self.span(), format!("gặp {} thay vì một giá trị", khac.mo_ta()))
                    .khai_niem("biểu thức");
                d = match khac {
                    TokKind::ChamPhay => d
                        .vi_sao("dấu `;` kết thúc câu lệnh, nên trước nó phải có một giá trị hoàn chỉnh")
                        .sua("viết nốt giá trị còn thiếu, ví dụ `let x = 1;`"),
                    TokKind::DongTron => d
                        .vi_sao("dấu `)` đóng lại một biểu thức, nhưng bên trong đang trống")
                        .sua("viết giá trị vào giữa hai dấu ngoặc"),
                    TokKind::TuKhoa(TuKhoa::Let) => d
                        .vi_sao("`let` bắt đầu một câu lệnh mới, nó không phải là giá trị")
                        .sua("có thể bạn quên `;` ở dòng trên"),
                    _ => d.sua("kiểm tra lại xem có thiếu tên biến, con số, hay dấu ngoặc không"),
                };
                self.diags.push(d);
                Err(())
            }
        }
    }

    fn be_quan(&mut self, bat_dau: Span) -> KetQua<BieuThuc> {
        // `||` rỗng hoặc `|a, b|`
        let mut tham_so = Vec::new();
        if self.khop(&TokKind::Hoac) {
            // closure không tham số
        } else {
            self.can(TokKind::HoacBit, "mở danh sách tham số của closure")?;
            while !self.la(&TokKind::HoacBit) && !self.het() {
                // Dùng `mau_don` chứ KHÔNG dùng `mau`: `mau` nuốt dấu `|` thành
                // mẫu Hoặc, khiến `|x| x + 1` không phân tích được.
                let m = self.mau_don()?;
                let k = if self.khop(&TokKind::HaiChamDung) { Some(self.kieu()?) } else { None };
                tham_so.push((m, k));
                if !self.khop(&TokKind::Phay) { break; }
            }
            self.can(TokKind::HoacBit, "đóng danh sách tham số của closure")?;
        }
        let than = self.bieu_thuc()?;
        Ok(BieuThuc::BeQuan { tham_so, span: bat_dau.merge(than.span()), than: Box::new(than) })
    }

    fn bieu_thuc_neu(&mut self, bat_dau: Span) -> KetQua<BieuThuc> {
        self.tien(); // if
        let cam = core::mem::replace(&mut self.cam_struct_literal, true);
        let dieu_kien = self.bieu_thuc()?;
        self.cam_struct_literal = cam;

        if !self.la(&TokKind::MoNhon) {
            self.diags.push(
                Diagnostic::loi("BR0114", "thân của `if` phải nằm trong dấu `{ }`")
                    .nhan(Label::chinh(Span::at(self.span_truoc().end), "cần `{` ở đây"))
                    .vi_sao("Rust luôn bắt buộc dấu ngoặc nhọn cho thân `if`, kể cả khi chỉ có một dòng — nhờ vậy không bao giờ có lỗi kiểu \"thụt lề đánh lừa\"")
                    .sua("bọc phần thân lại: `if dieu_kien { ... }`")
                    .khai_niem("if"),
            );
            return Err(());
        }
        let than = self.khoi()?;

        let nguoc_lai = if self.khop(&TokKind::TuKhoa(TuKhoa::Else)) {
            if self.la(&TokKind::TuKhoa(TuKhoa::If)) {
                let sp = self.span();
                Some(Box::new(self.bieu_thuc_neu(sp)?))
            } else {
                let k = self.khoi()?;
                Some(Box::new(BieuThuc::Khoi(Box::new(k))))
            }
        } else { None };

        let het_span = nguoc_lai.as_ref().map(|e| e.span()).unwrap_or(than.span);
        Ok(BieuThuc::Neu { dieu_kien: Box::new(dieu_kien), than: Box::new(than), nguoc_lai, span: bat_dau.merge(het_span) })
    }

    fn bieu_thuc_khop(&mut self, bat_dau: Span) -> KetQua<BieuThuc> {
        self.tien(); // match
        let cam = core::mem::replace(&mut self.cam_struct_literal, true);
        let gia_tri = self.bieu_thuc()?;
        self.cam_struct_literal = cam;
        self.can(TokKind::MoNhon, "mở thân `match`")?;

        let mut nhanh = Vec::new();
        while !self.la(&TokKind::DongNhon) && !self.het() {
            let sp0 = self.span();
            let mau = match self.mau() {
                Ok(m) => m,
                Err(()) => { self.dong_bo(); continue; }
            };
            let dieu_kien = if self.khop(&TokKind::TuKhoa(TuKhoa::If)) {
                Some(self.bieu_thuc()?)
            } else { None };

            if !self.khop(&TokKind::MuiTenDam) {
                self.diags.push(
                    Diagnostic::loi("BR0115", "thiếu dấu `=>` trong nhánh của `match`")
                        .nhan(Label::chinh(Span::at(self.span_truoc().end), "cần `=>` ở đây"))
                        .nhan(Label::phu(sp0, "nhánh bắt đầu từ đây"))
                        .vi_sao("mỗi nhánh `match` có dạng `<mẫu> => <giá trị>`. Chú ý là `=>` chứ không phải `->`.")
                        .sua("thêm `=>` sau mẫu")
                        .khai_niem("match"),
                );
                self.dong_bo();
                continue;
            }

            let than = match self.bieu_thuc() {
                Ok(b) => b,
                Err(()) => { self.dong_bo(); continue; }
            };
            let can_phay = !la_bieu_thuc_khoi(&than);
            nhanh.push(NhanhKhop { mau, dieu_kien, span: sp0.merge(than.span()), than });
            if !self.khop(&TokKind::Phay) && can_phay && !self.la(&TokKind::DongNhon) {
                self.diags.push(
                    Diagnostic::loi("BR0116", "thiếu dấu `,` giữa hai nhánh `match`")
                        .nhan(Label::chinh(Span::at(self.span_truoc().end), "cần `,` ở đây"))
                        .vi_sao("các nhánh của `match` ngăn cách nhau bằng dấu phẩy")
                        .sua("thêm `,` vào cuối nhánh")
                        .khai_niem("match"),
                );
            }
        }
        self.can(TokKind::DongNhon, "đóng thân `match`")?;
        Ok(BieuThuc::KhopMau { gia_tri: Box::new(gia_tri), nhanh, span: bat_dau.merge(self.span_truoc()) })
    }
}

/// Phân tích mã nguồn thành cây cú pháp.
pub fn phan_tich(src: &str) -> (ChuongTrinh, Diagnostics) {
    let (toks, diags) = crate::lexer::quet(src);
    Parser::new(toks, diags).phan_tich()
}
