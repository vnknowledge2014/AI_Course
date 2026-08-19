//! Bộ thực thi duyệt cây.
//!
//! Ba đặc điểm khác một interpreter thông thường:
//!
//! 1. **Có nhiên liệu (fuel).** Mọi bước đều trừ nhiên liệu. Người học viết nhầm
//!    `while true {}` là chuyện thường; treo máy họ thì không chấp nhận được.
//! 2. **Mô phỏng quyền sở hữu.** Gán một `String` sang biến khác sẽ *chuyển* nó,
//!    và biến cũ trở thành `DaChuyen`. Dùng lại nó sẽ báo lỗi có chỉ rõ nơi đã
//!    chuyển đi — đây chính là bài học trung tâm của Rust.
//! 3. **Lỗi lúc chạy cũng phải dạy được.** Chia cho 0, vượt chỉ số, `unwrap` trên
//!    `None` đều có giải thích và hướng sửa.

use crate::ast::*;
use crate::diag::{Diagnostic, Diagnostics, Label};
use crate::span::Span;
use crate::value::{BeQuan, GiaTri, O};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Nhiên liệu mặc định — đủ cho mọi bài tập trong giáo trình, nhưng chặn được
/// vòng lặp vô hạn trong khoảng một phần mười giây.
pub const NHIEN_LIEU_MAC_DINH: u64 = 2_000_000;

/// Giới hạn số ký tự in ra, chống `loop { println!(...) }` làm ngập giao diện.
pub const GIOI_HAN_XUAT: usize = 64 * 1024;

/// Độ sâu lời gọi tối đa.
///
/// Mỗi mức đệ quy của chương trình người học tốn khoảng 7 khung stack của chính
/// interpreter (`goi` → `thuc_thi_khoi_ngay` → `tinh` → `goi_ham` → …). Stack
/// mặc định của WASM chỉ 1 MB — nhỏ hơn cả stack của một thread thường — nên
/// giới hạn phải đặt thấp. 100 mức vẫn dư cho mọi bài đệ quy trong giáo trình
/// (giai thừa, Fibonacci, duyệt cây), mà chắc chắn không làm sập app.
///
/// **Con số 40 là đo được, không phải đoán.** Thử nghiệm trên bản debug (khung
/// stack lớn nhất): 64 mức vẫn tràn, 48 mức thì an toàn. Lấy 40 để có biên.
/// TODO: hạ mức tiêu thụ stack của `tinh()` bằng cách tách các nhánh match lớn
/// ra hàm riêng, rồi nâng giới hạn này lên.
pub const GIOI_HAN_DO_SAU: usize = 40;

/// Lý do dừng thực thi giữa chừng.
pub enum Ngat {
    TraVe(GiaTri),
    Thoat(Option<GiaTri>),
    TiepTuc,
    /// Lỗi có chẩn đoán kèm theo.
    Loi(Box<Diagnostic>),
}

type KQ<T> = Result<T, Ngat>;

#[derive(Default)]
struct Pham {
    bien: HashMap<String, O>,
}

pub struct MayChay {
    /// Ngăn xếp phạm vi; phần tử cuối là phạm vi trong cùng.
    pham: Vec<Pham>,
    ham: HashMap<String, Rc<Ham>>,
    /// `(tên kiểu, tên phương thức) -> hàm`
    phuong_thuc: HashMap<(String, String), Rc<Ham>>,
    struct_def: HashMap<String, Rc<Struct>>,
    /// `tên biến thể -> tên enum`
    bien_the_cua: HashMap<String, String>,
    enum_def: HashMap<String, Rc<Enum>>,
    pub xuat: String,
    nhien_lieu: u64,
    bi_cat_xuat: bool,
    /// Độ sâu lời gọi hàm.
    ///
    /// KHÔNG suy ra được từ `pham.len()`: `goi()` **thay** ngăn xếp phạm vi chứ
    /// không đẩy thêm, nên `pham.len()` luôn bằng 2 dù đệ quy sâu bao nhiêu.
    /// Đếm sai ở đây nghĩa là đệ quy vô hạn làm tràn stack và giết cả app.
    do_sau_goi: usize,
}

impl MayChay {
    pub fn moi() -> Self {
        Self {
            pham: vec![Pham::default()],
            ham: HashMap::new(),
            phuong_thuc: HashMap::new(),
            struct_def: HashMap::new(),
            bien_the_cua: HashMap::new(),
            enum_def: HashMap::new(),
            xuat: String::new(),
            nhien_lieu: NHIEN_LIEU_MAC_DINH,
            bi_cat_xuat: false,
            do_sau_goi: 0,
        }
    }

    pub fn voi_nhien_lieu(mut self, n: u64) -> Self {
        self.nhien_lieu = n;
        self
    }

    // ── Nhiên liệu ──────────────────────────────────────────────────────────

    fn tieu_hao(&mut self, span: Span) -> KQ<()> {
        if self.nhien_lieu == 0 {
            return Err(Ngat::Loi(Box::new(
                Diagnostic::loi("BR0500", "chương trình chạy quá lâu nên đã bị dừng")
                    .tai(span, "dừng ở đây")
                    .vi_sao("nhiều khả năng có một vòng lặp không bao giờ kết thúc — điều kiện dừng không bao giờ thành `false`")
                    .sua("kiểm tra xem biến trong điều kiện vòng lặp có thực sự thay đổi không, ví dụ `i += 1`")
                    .sua("với `loop`, chắc chắn có một nhánh `break` chạy tới được")
                    .khai_niem("vòng lặp"),
            )));
        }
        self.nhien_lieu -= 1;
        Ok(())
    }

    fn in_ra(&mut self, s: &str) {
        if self.xuat.len() + s.len() > GIOI_HAN_XUAT {
            if !self.bi_cat_xuat {
                self.bi_cat_xuat = true;
                self.xuat.push_str("\n… (đã in quá nhiều, phần sau bị cắt bớt)\n");
            }
            return;
        }
        self.xuat.push_str(s);
    }

    // ── Phạm vi & biến ──────────────────────────────────────────────────────

    fn vao_pham(&mut self) {
        self.pham.push(Pham::default());
    }

    fn roi_pham(&mut self) {
        self.pham.pop();
    }

    fn dat_bien(&mut self, ten: &str, gt: GiaTri) {
        let o = Rc::new(RefCell::new(gt));
        self.pham.last_mut().unwrap().bien.insert(ten.to_string(), o);
    }

    fn tim_o(&self, ten: &str) -> Option<O> {
        self.pham.iter().rev().find_map(|p| p.bien.get(ten).cloned())
    }

    fn loi_khong_thay_ten(&self, ten: &str, span: Span) -> Ngat {
        // Gợi ý tên gần giống — lỗi gõ nhầm là phổ biến nhất với người mới.
        let mut gan_nhat: Option<(usize, &String)> = None;
        for p in &self.pham {
            for co in p.bien.keys() {
                let d = khoang_cach_sua(ten, co);
                if d <= 2 && gan_nhat.map(|(bd, _)| d < bd).unwrap_or(true) {
                    gan_nhat = Some((d, co));
                }
            }
        }
        let mut d = Diagnostic::loi("BR0501", format!("không tìm thấy `{ten}`"))
            .tai(span, "tên này chưa được khai báo")
            .vi_sao("Rust chỉ dùng được biến sau khi nó đã được khai báo bằng `let`, và chỉ trong phạm vi `{ }` chứa nó")
            .khai_niem("phạm vi biến");
        if let Some((_, co)) = gan_nhat {
            d = d.sua(format!("có phải bạn định viết `{co}` không?"));
        } else {
            d = d.sua(format!("khai báo trước khi dùng: `let {ten} = ...;`"));
        }
        Ngat::Loi(Box::new(d))
    }

    // ── Nạp chương trình ────────────────────────────────────────────────────

    pub fn nap(&mut self, ct: &ChuongTrinh) {
        for m in &ct.muc {
            self.nap_muc(m);
        }
    }

    fn nap_muc(&mut self, m: &Muc) {
        match m {
            Muc::Ham(h) => {
                self.ham.insert(h.ten.clone(), Rc::new(h.clone()));
            }
            Muc::Struct(s) => {
                self.struct_def.insert(s.ten.clone(), Rc::new(s.clone()));
            }
            Muc::Enum(e) => {
                for bt in &e.bien_the {
                    self.bien_the_cua.insert(bt.ten.clone(), e.ten.clone());
                }
                self.enum_def.insert(e.ten.clone(), Rc::new(e.clone()));
            }
            Muc::Impl(i) => {
                let ten_kieu = match &i.kieu {
                    Kieu::DuongDan { doan, .. } => doan.last().cloned().unwrap_or_default(),
                    khac => khac.hien_thi(),
                };
                for h in &i.ham {
                    self.phuong_thuc
                        .insert((ten_kieu.clone(), h.ten.clone()), Rc::new(h.clone()));
                }
            }
            Muc::Trait(t) => {
                // Phương thức mặc định của trait dùng được cho mọi kiểu implement nó.
                for (h, co_than) in &t.ham {
                    if *co_than {
                        self.phuong_thuc
                            .insert((format!("<trait {}>", t.ten), h.ten.clone()), Rc::new(h.clone()));
                    }
                }
            }
            Muc::Const { ten, gia_tri, .. } => {
                if let Ok(v) = self.tinh(gia_tri) {
                    self.dat_bien(ten, v);
                }
            }
            Muc::Use { .. } => {}
        }
    }

    /// Chạy `fn main()`.
    pub fn chay(&mut self, ct: &ChuongTrinh) -> Result<(), Diagnostic> {
        self.nap(ct);
        let Some(main) = self.ham.get("main").cloned() else {
            return Err(
                Diagnostic::loi("BR0502", "chương trình thiếu hàm `main`")
                    .vi_sao("Rust bắt đầu chạy từ `fn main()`; không có nó thì không biết bắt đầu ở đâu")
                    .sua("thêm `fn main() { ... }` và đặt mã của bạn vào trong")
                    .khai_niem("cấu trúc chương trình"),
            );
        };
        match self.goi(&main, Vec::new(), main.span) {
            Ok(_) | Err(Ngat::TraVe(_)) => Ok(()),
            Err(Ngat::Loi(d)) => Err(*d),
            Err(Ngat::Thoat(_)) => Err(Diagnostic::loi("BR0503", "`break` nằm ngoài vòng lặp")
                .vi_sao("`break` chỉ dùng được bên trong `loop`, `while` hoặc `for`")
                .khai_niem("vòng lặp")),
            Err(Ngat::TiepTuc) => Err(Diagnostic::loi("BR0504", "`continue` nằm ngoài vòng lặp")
                .vi_sao("`continue` chỉ dùng được bên trong vòng lặp")
                .khai_niem("vòng lặp")),
        }
    }

    // ── Gọi hàm ─────────────────────────────────────────────────────────────

    fn goi(&mut self, h: &Ham, doi_so: Vec<GiaTri>, span: Span) -> KQ<GiaTri> {
        if self.do_sau_goi >= GIOI_HAN_DO_SAU {
            return Err(Ngat::Loi(Box::new(
                Diagnostic::loi("BR0505", "hàm gọi lồng nhau quá sâu")
                    .tai(span, "dừng ở đây")
                    .vi_sao("nhiều khả năng hàm đang gọi lại chính nó mà không có điều kiện dừng")
                    .sua("thêm trường hợp cơ sở, ví dụ `if n <= 1 { return 1; }`")
                    .khai_niem("đệ quy"),
            )));
        }
        if doi_so.len() != h.tham_so.len() {
            return Err(Ngat::Loi(Box::new(
                Diagnostic::loi(
                    "BR0506",
                    format!(
                        "hàm `{}` cần {} đối số nhưng nhận được {}",
                        h.ten,
                        h.tham_so.len(),
                        doi_so.len()
                    ),
                )
                .tai(span, "lời gọi ở đây")
                .nhan(Label::phu(h.span, format!("`{}` khai báo ở đây", h.ten)))
                .vi_sao("số đối số phải khớp đúng với khai báo; Rust không có tham số tuỳ chọn")
                .khai_niem("hàm"),
            )));
        }

        // Phạm vi hàm bắt đầu SẠCH: hàm không nhìn thấy biến cục bộ của nơi gọi.
        let toan_cuc = self.pham[0].bien.clone();
        let luu = core::mem::replace(self.pham_mut(), vec![Pham { bien: toan_cuc }, Pham::default()]);
        self.do_sau_goi += 1;

        let mut kq = Ok(GiaTri::Rong);
        for (ts, gt) in h.tham_so.iter().zip(doi_so) {
            if let Err(e) = self.rang_buoc_mau(&ts.mau, gt) {
                kq = Err(e);
                break;
            }
        }
        if kq.is_ok() {
            kq = self.thuc_thi_khoi_ngay(&h.than);
        }
        self.do_sau_goi -= 1;
        *self.pham_mut() = luu;

        match kq {
            Ok(v) => Ok(v),
            Err(Ngat::TraVe(v)) => Ok(v),
            Err(e) => Err(e),
        }
    }

    fn pham_mut(&mut self) -> &mut Vec<Pham> {
        &mut self.pham
    }

    // ── Câu lệnh & khối ─────────────────────────────────────────────────────

    fn thuc_thi_khoi(&mut self, k: &Khoi) -> KQ<GiaTri> {
        self.vao_pham();
        let kq = self.thuc_thi_khoi_ngay(k);
        self.roi_pham();
        kq
    }

    /// Chạy khối trong phạm vi hiện tại (không tự mở phạm vi mới).
    fn thuc_thi_khoi_ngay(&mut self, k: &Khoi) -> KQ<GiaTri> {
        for cl in &k.cau_lenh {
            self.tieu_hao(cl.span())?;
            match cl {
                CauLenh::Muc(m) => self.nap_muc(m),
                CauLenh::Let { mau, gia_tri, .. } => {
                    let v = match gia_tri {
                        Some(bt) => self.tinh_va_chuyen(bt)?,
                        None => GiaTri::Rong,
                    };
                    self.rang_buoc_mau(mau, v)?;
                }
                CauLenh::BieuThuc { bt, .. } => {
                    self.tinh(bt)?;
                }
            }
        }
        match &k.gia_tri_cuoi {
            Some(bt) => self.tinh(bt),
            None => Ok(GiaTri::Rong),
        }
    }

    // ── Ràng buộc & khớp mẫu ────────────────────────────────────────────────

    fn rang_buoc_mau(&mut self, m: &Mau, gt: GiaTri) -> KQ<()> {
        if !self.khop_mau(m, &gt, true)? {
            return Err(Ngat::Loi(Box::new(
                Diagnostic::loi("BR0507", "giá trị không khớp với mẫu bên trái dấu `=`")
                    .tai(m.span(), format!("mẫu này không nhận được `{}`", gt.ten_kieu()))
                    .vi_sao("hình dạng bên trái `let` phải khớp với hình dạng của giá trị bên phải")
                    .khai_niem("mẫu"),
            )));
        }
        Ok(())
    }

    /// Thử khớp `gt` với mẫu `m`. Nếu `rang_buoc` thì đồng thời tạo biến.
    fn khop_mau(&mut self, m: &Mau, gt: &GiaTri, rang_buoc: bool) -> KQ<bool> {
        let gt_that = gt.giai_tham();
        Ok(match m {
            Mau::BoQua { .. } => true,
            Mau::Ten { ten, .. } => {
                if rang_buoc {
                    self.dat_bien(ten, gt.clone());
                }
                true
            }
            Mau::HangSo { gia_tri, .. } => {
                let v = hang_so_thanh_gia_tri(gia_tri);
                v.bang(&gt_that)
            }
            Mau::Dai { tu, den, bao_gom_cuoi, .. } => {
                let (Mau::HangSo { gia_tri: a, .. }, Mau::HangSo { gia_tri: b, .. }) =
                    (&**tu, &**den) else { return Ok(false) };
                let (a, b) = (hang_so_thanh_gia_tri(a), hang_so_thanh_gia_tri(b));
                let tren = matches!(gt_that.so_sanh(&a), Some(o) if o != std::cmp::Ordering::Less);
                let duoi = match gt_that.so_sanh(&b) {
                    Some(std::cmp::Ordering::Less) => true,
                    Some(std::cmp::Ordering::Equal) => *bao_gom_cuoi,
                    _ => false,
                };
                tren && duoi
            }
            Mau::Tuple { phan_tu, .. } => match &gt_that {
                GiaTri::Tuple(v) if v.len() == phan_tu.len() => {
                    for (p, x) in phan_tu.iter().zip(v.iter()) {
                        if !self.khop_mau(p, x, rang_buoc)? {
                            return Ok(false);
                        }
                    }
                    true
                }
                _ => false,
            },
            Mau::Hoac { nhanh, .. } => {
                for n in nhanh {
                    if self.khop_mau(n, gt, rang_buoc)? {
                        return Ok(true);
                    }
                }
                false
            }
            Mau::BienThe { duong_dan, truong, .. } => {
                let ten_cuoi = duong_dan.last().cloned().unwrap_or_default();
                match &gt_that {
                    GiaTri::BienThe { bien_the, gia_tri, .. } => {
                        if **bien_the != *ten_cuoi {
                            return Ok(false);
                        }
                        match truong {
                            MauTruong::Khong => true,
                            MauTruong::TheoViTri(ps) => {
                                if ps.len() != gia_tri.len() {
                                    return Ok(false);
                                }
                                for (p, x) in ps.iter().zip(gia_tri.iter()) {
                                    if !self.khop_mau(p, x, rang_buoc)? {
                                        return Ok(false);
                                    }
                                }
                                true
                            }
                            MauTruong::TheoTen { .. } => false,
                        }
                    }
                    GiaTri::Struct { ten, truong: f } => {
                        if **ten != *ten_cuoi {
                            return Ok(false);
                        }
                        match truong {
                            MauTruong::Khong => true,
                            MauTruong::TheoTen { truong: ts, .. } => {
                                let f = f.borrow().clone();
                                for (k, p) in ts {
                                    let Some(x) = f.get(k) else { return Ok(false) };
                                    if !self.khop_mau(p, x, rang_buoc)? {
                                        return Ok(false);
                                    }
                                }
                                true
                            }
                            MauTruong::TheoViTri(_) => false,
                        }
                    }
                    _ => false,
                }
            }
        })
    }
}

fn hang_so_thanh_gia_tri(h: &HangSo) -> GiaTri {
    match h {
        HangSo::SoNguyen(n) => GiaTri::SoNguyen(*n),
        HangSo::SoThuc(f) => GiaTri::SoThuc(*f),
        HangSo::Chuoi(s) => GiaTri::Chuoi(Rc::new(s.clone())),
        HangSo::KyTu(c) => GiaTri::KyTu(*c),
        HangSo::DungSai(b) => GiaTri::DungSai(*b),
    }
}

/// Khoảng cách Levenshtein, dùng để gợi ý "có phải bạn định viết…".
fn khoang_cach_sua(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut truoc: Vec<usize> = (0..=b.len()).collect();
    let mut nay = vec![0usize; b.len() + 1];
    for i in 1..=a.len() {
        nay[0] = i;
        for j in 1..=b.len() {
            let gia = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            nay[j] = (truoc[j] + 1).min(nay[j - 1] + 1).min(truoc[j - 1] + gia);
        }
        core::mem::swap(&mut truoc, &mut nay);
    }
    truoc[b.len()]
}

/// Chạy một chương trình, trả về những gì nó in ra hoặc chẩn đoán lỗi.
pub fn chay(src: &str) -> (String, Diagnostics) {
    let (ct, mut d) = crate::parser::phan_tich(src);
    if d.co_loi() {
        return (String::new(), d.rut_gon());
    }
    let mut may = MayChay::moi();
    match may.chay(&ct) {
        Ok(()) => (may.xuat, d),
        Err(loi) => {
            let xuat = may.xuat.clone();
            d.push(loi);
            (xuat, d)
        }
    }
}

// ── Tính biểu thức ──────────────────────────────────────────────────────────

impl MayChay {
    pub fn tinh(&mut self, bt: &BieuThuc) -> KQ<GiaTri> {
        self.tieu_hao(bt.span())?;
        match bt {
            BieuThuc::HangSo { gia_tri, .. } => Ok(hang_so_thanh_gia_tri(gia_tri)),

            BieuThuc::DuongDan { doan, span } => self.tinh_duong_dan(doan, *span),

            BieuThuc::Khoi(k) => self.thuc_thi_khoi(k),

            BieuThuc::MotNgoi { toan_tu, toan_hang, span } => {
                let v = self.tinh(toan_hang)?.giai_tham();
                match (toan_tu, &v) {
                    (ToanTuMot::Am, GiaTri::SoNguyen(n)) => Ok(GiaTri::SoNguyen(-n)),
                    (ToanTuMot::Am, GiaTri::SoThuc(f)) => Ok(GiaTri::SoThuc(-f)),
                    (ToanTuMot::Phu, GiaTri::DungSai(b)) => Ok(GiaTri::DungSai(!b)),
                    (ToanTuMot::Am, khac) => Err(loi_kieu(
                        *span,
                        format!("không thể lấy số âm của `{}`", khac.ten_kieu()),
                        "dấu `-` một ngôi chỉ dùng được với số",
                    )),
                    (ToanTuMot::Phu, khac) => Err(loi_kieu(
                        *span,
                        format!("không thể phủ định `{}`", khac.ten_kieu()),
                        "dấu `!` chỉ dùng được với `bool`",
                    )),
                }
            }

            BieuThuc::HaiNgoi { toan_tu, trai, phai, span } => {
                // `&&` và `||` phải đánh giá lười — vế phải chỉ chạy khi cần.
                if matches!(toan_tu, ToanTuHai::Va | ToanTuHai::Hoac) {
                    let t = self.tinh(trai)?.giai_tham();
                    let Some(tb) = t.la_dung() else {
                        return Err(loi_kieu(
                            trai.span(),
                            format!("`{}` cần `bool` nhưng nhận `{}`", toan_tu.ky_hieu(), t.ten_kieu()),
                            "Rust không tự coi số hay chuỗi là đúng/sai như một số ngôn ngữ khác",
                        ));
                    };
                    let ngan_mach = match toan_tu {
                        ToanTuHai::Va => !tb,
                        _ => tb,
                    };
                    if ngan_mach {
                        return Ok(GiaTri::DungSai(tb));
                    }
                    let p = self.tinh(phai)?.giai_tham();
                    let Some(pb) = p.la_dung() else {
                        return Err(loi_kieu(
                            phai.span(),
                            format!("`{}` cần `bool` nhưng nhận `{}`", toan_tu.ky_hieu(), p.ten_kieu()),
                            "cả hai vế của phép logic đều phải là `bool`",
                        ));
                    };
                    return Ok(GiaTri::DungSai(pb));
                }

                let t = self.tinh(trai)?.giai_tham();
                let p = self.tinh(phai)?.giai_tham();
                self.hai_ngoi(*toan_tu, t, p, *span, trai.span(), phai.span())
            }

            BieuThuc::Muon { co_the_sua, gia_tri, span } => {
                if let BieuThuc::DuongDan { doan, .. } = &**gia_tri {
                    if doan.len() == 1 {
                        let Some(o) = self.tim_o(&doan[0]) else {
                            return Err(self.loi_khong_thay_ten(&doan[0], *span));
                        };
                        return Ok(GiaTri::ThamChieu { o, co_the_sua: *co_the_sua });
                    }
                }
                let v = self.tinh(gia_tri)?;
                Ok(GiaTri::ThamChieu { o: Rc::new(RefCell::new(v)), co_the_sua: *co_the_sua })
            }

            BieuThuc::GiaiTham { gia_tri, span } => {
                let v = self.tinh(gia_tri)?;
                match v {
                    GiaTri::ThamChieu { o, .. } => Ok(o.borrow().clone()),
                    khac => Err(loi_kieu(
                        *span,
                        format!("không thể dùng `*` với `{}`", khac.ten_kieu()),
                        "dấu `*` chỉ dùng để lấy giá trị mà một tham chiếu đang trỏ tới",
                    )),
                }
            }

            BieuThuc::Tuple { phan_tu, .. } => {
                let mut v = Vec::with_capacity(phan_tu.len());
                for p in phan_tu {
                    v.push(self.tinh(p)?);
                }
                Ok(if v.is_empty() { GiaTri::Rong } else { GiaTri::Tuple(Rc::new(v)) })
            }

            BieuThuc::Mang { phan_tu, lap_lai, .. } => {
                if let Some(n) = lap_lai {
                    let mau = self.tinh(&phan_tu[0])?;
                    let so = match self.tinh(n)?.giai_tham() {
                        GiaTri::SoNguyen(k) if k >= 0 => k as usize,
                        _ => 0,
                    };
                    return Ok(GiaTri::Day(Rc::new(RefCell::new(vec![mau; so]))));
                }
                let mut v = Vec::with_capacity(phan_tu.len());
                for p in phan_tu {
                    v.push(self.tinh(p)?);
                }
                Ok(GiaTri::Day(Rc::new(RefCell::new(v))))
            }

            BieuThuc::Dai { tu, den, bao_gom_cuoi, span } => {
                let a = match tu {
                    Some(e) => so_nguyen(self.tinh(e)?.giai_tham(), e.span())?,
                    None => 0,
                };
                let b = match den {
                    Some(e) => so_nguyen(self.tinh(e)?.giai_tham(), e.span())?,
                    None => {
                        return Err(loi_kieu(
                            *span,
                            "dải phải có điểm kết thúc".into(),
                            "ví dụ `0..10`; dải mở như `0..` chưa được hỗ trợ",
                        ))
                    }
                };
                Ok(GiaTri::Dai { tu: a, den: b, bao_gom_cuoi: *bao_gom_cuoi })
            }

            BieuThuc::Neu { dieu_kien, than, nguoc_lai, .. } => {
                let dk = self.tinh(dieu_kien)?.giai_tham();
                let Some(b) = dk.la_dung() else {
                    return Err(loi_kieu(
                        dieu_kien.span(),
                        format!("điều kiện của `if` phải là `bool`, không phải `{}`", dk.ten_kieu()),
                        "khác với Python hay JavaScript, Rust không coi số 0 hay chuỗi rỗng là `false`",
                    ));
                };
                if b {
                    self.thuc_thi_khoi(than)
                } else if let Some(nl) = nguoc_lai {
                    self.tinh(nl)
                } else {
                    Ok(GiaTri::Rong)
                }
            }

            BieuThuc::KhopMau { gia_tri, nhanh, span } => {
                let v = self.tinh(gia_tri)?;
                for n in nhanh {
                    self.vao_pham();
                    let khop = self.khop_mau(&n.mau, &v, true)?;
                    if khop {
                        let qua = match &n.dieu_kien {
                            Some(dk) => self.tinh(dk)?.giai_tham().la_dung().unwrap_or(false),
                            None => true,
                        };
                        if qua {
                            let r = self.tinh(&n.than);
                            self.roi_pham();
                            return r;
                        }
                    }
                    self.roi_pham();
                }
                Err(Ngat::Loi(Box::new(
                    Diagnostic::loi("BR0508", "không có nhánh `match` nào khớp")
                        .tai(*span, format!("giá trị `{}` không rơi vào nhánh nào", v.go_ro()))
                        .vi_sao("`match` phải phủ HẾT mọi khả năng — đó là điều khiến nó an toàn hơn `switch`")
                        .sua("thêm nhánh `_ => ...` để bắt các trường hợp còn lại")
                        .khai_niem("match"),
                )))
            }

            BieuThuc::Lap { than, .. } => loop {
                self.tieu_hao(than.span)?;
                match self.thuc_thi_khoi(than) {
                    Ok(_) | Err(Ngat::TiepTuc) => {}
                    Err(Ngat::Thoat(v)) => return Ok(v.unwrap_or(GiaTri::Rong)),
                    Err(e) => return Err(e),
                }
            },

            BieuThuc::Trong { dieu_kien, than, .. } => {
                loop {
                    self.tieu_hao(than.span)?;
                    let dk = self.tinh(dieu_kien)?.giai_tham();
                    let Some(b) = dk.la_dung() else {
                        return Err(loi_kieu(
                            dieu_kien.span(),
                            format!("điều kiện của `while` phải là `bool`, không phải `{}`", dk.ten_kieu()),
                            "vòng lặp `while` chạy khi điều kiện còn đúng",
                        ));
                    };
                    if !b {
                        break;
                    }
                    match self.thuc_thi_khoi(than) {
                        Ok(_) | Err(Ngat::TiepTuc) => {}
                        Err(Ngat::Thoat(_)) => break,
                        Err(e) => return Err(e),
                    }
                }
                Ok(GiaTri::Rong)
            }

            BieuThuc::Cho { mau, day, than, .. } => {
                let d = self.tinh(day)?.giai_tham();
                let cac_gt: Vec<GiaTri> = match &d {
                    GiaTri::Dai { tu, den, bao_gom_cuoi } => {
                        let het = if *bao_gom_cuoi { *den + 1 } else { *den };
                        (*tu..het).map(GiaTri::SoNguyen).collect()
                    }
                    GiaTri::Day(v) => v.borrow().clone(),
                    GiaTri::Chuoi(s) => s.chars().map(GiaTri::KyTu).collect(),
                    khac => {
                        return Err(loi_kieu(
                            day.span(),
                            format!("không lặp được trên `{}`", khac.ten_kieu()),
                            "`for` cần một dãy: dải như `0..10`, một `Vec`, hoặc một mảng",
                        ))
                    }
                };
                for gt in cac_gt {
                    self.tieu_hao(than.span)?;
                    self.vao_pham();
                    let r = self
                        .khop_mau(mau, &gt, true)
                        .and_then(|_| self.thuc_thi_khoi_ngay(than));
                    self.roi_pham();
                    match r {
                        Ok(_) | Err(Ngat::TiepTuc) => {}
                        Err(Ngat::Thoat(_)) => break,
                        Err(e) => return Err(e),
                    }
                }
                Ok(GiaTri::Rong)
            }

            BieuThuc::TraVe { gia_tri, .. } => {
                let v = match gia_tri {
                    Some(e) => self.tinh(e)?,
                    None => GiaTri::Rong,
                };
                Err(Ngat::TraVe(v))
            }
            BieuThuc::Thoat { gia_tri, .. } => {
                let v = match gia_tri {
                    Some(e) => Some(self.tinh(e)?),
                    None => None,
                };
                Err(Ngat::Thoat(v))
            }
            BieuThuc::TiepTuc { .. } => Err(Ngat::TiepTuc),

            BieuThuc::BeQuan { tham_so, than, .. } => Ok(GiaTri::BeQuan(Rc::new(BeQuan {
                tham_so: tham_so.iter().map(|(m, _)| m.clone()).collect(),
                than: Rc::new((**than).clone()),
                bat: self.pham.iter().map(|p| p.bien.clone()).collect(),
            }))),

            BieuThuc::Gan { dich, toan_tu, gia_tri, span } => {
                let moi = match toan_tu {
                    // `x += y` không chuyển quyền sở hữu của `y` (chỉ đọc giá trị)
                    Some(_) => self.tinh(gia_tri)?,
                    None => self.tinh_va_chuyen(gia_tri)?,
                };
                let cuoi = match toan_tu {
                    None => moi,
                    Some(op) => {
                        let cu = self.tinh(dich)?.giai_tham();
                        self.hai_ngoi(*op, cu, moi.giai_tham(), *span, dich.span(), gia_tri.span())?
                    }
                };
                self.gan_vao(dich, cuoi)?;
                Ok(GiaTri::Rong)
            }

            BieuThuc::TruyCapTruong { doi_tuong, ten, span } => {
                let v = self.tinh(doi_tuong)?.giai_tham();
                match &v {
                    GiaTri::Struct { truong, ten: ten_struct } => truong
                        .borrow()
                        .get(ten)
                        .cloned()
                        .ok_or_else(|| loi_khong_co_truong(*span, ten_struct, ten, &truong.borrow())),
                    GiaTri::Tuple(v) => {
                        let i: usize = ten.parse().unwrap_or(usize::MAX);
                        v.get(i).cloned().ok_or_else(|| {
                            loi_kieu(
                                *span,
                                format!("bộ giá trị này chỉ có {} phần tử, không có `.{ten}`", v.len()),
                                "phần tử của tuple đánh số từ 0",
                            )
                        })
                    }
                    khac => Err(loi_kieu(
                        *span,
                        format!("`{}` không có trường nào", khac.ten_kieu()),
                        "dấu `.` dùng để lấy trường của struct hoặc phần tử của tuple",
                    )),
                }
            }

            BieuThuc::ChiSo { doi_tuong, chi_so, span } => {
                let v = self.tinh(doi_tuong)?.giai_tham();
                let i = self.tinh(chi_so)?.giai_tham();
                let GiaTri::Day(day) = &v else {
                    return Err(loi_kieu(
                        *span,
                        format!("không thể lấy phần tử của `{}` bằng `[...]`", v.ten_kieu()),
                        "chỉ `Vec` và mảng mới dùng được cú pháp `[chỉ số]`",
                    ));
                };
                let idx = so_nguyen(i, chi_so.span())?;
                let d = day.borrow();
                if idx < 0 || idx as usize >= d.len() {
                    return Err(Ngat::Loi(Box::new(
                        Diagnostic::loi("BR0509", "chỉ số vượt ngoài phạm vi")
                            .tai(chi_so.span(), format!("chỉ số {idx}, nhưng dãy chỉ có {} phần tử", d.len()))
                            .vi_sao("phần tử đánh số từ 0, nên dãy n phần tử có chỉ số hợp lệ từ 0 đến n-1")
                            .sua(if d.is_empty() {
                                "dãy đang rỗng — kiểm tra xem đã thêm phần tử vào chưa".to_string()
                            } else {
                                format!("chỉ số hợp lệ ở đây là từ 0 đến {}", d.len() - 1)
                            })
                            .sua("dùng `.get(i)` để nhận `Option` thay vì dừng chương trình")
                            .khai_niem("chỉ số"),
                    )));
                }
                Ok(d[idx as usize].clone())
            }

            BieuThuc::KhoiTaoStruct { duong_dan, truong, span, .. } => {
                let ten = duong_dan.last().cloned().unwrap_or_default();
                let mut m = HashMap::new();
                for (k, e) in truong {
                    let v = self.tinh_va_chuyen(e)?;
                    m.insert(k.clone(), v);
                }
                if let Some(sd) = self.struct_def.get(&ten).cloned() {
                    if let ThanStruct::TheoTen(cac) = &sd.than {
                        for (ten_t, _, _) in cac {
                            if !m.contains_key(ten_t) {
                                return Err(Ngat::Loi(Box::new(
                                    Diagnostic::loi("BR0510", format!("thiếu trường `{ten_t}`"))
                                        .tai(*span, format!("khởi tạo `{ten}` mà chưa có `{ten_t}`"))
                                        .vi_sao("Rust bắt buộc điền đủ mọi trường khi tạo struct — nhờ vậy không bao giờ có trường bỏ trống ngoài ý muốn")
                                        .sua(format!("thêm `{ten_t}: <giá trị>` vào trong dấu ngoặc nhọn"))
                                        .khai_niem("struct"),
                                )));
                            }
                        }
                    }
                }
                Ok(GiaTri::Struct { ten: Rc::from(ten.as_str()), truong: Rc::new(RefCell::new(m)) })
            }

            BieuThuc::Ep { gia_tri, kieu, .. } => {
                let v = self.tinh(gia_tri)?.giai_tham();
                let t = kieu.hien_thi();
                Ok(match (&v, t.as_str()) {
                    (GiaTri::SoNguyen(n), "f64" | "f32") => GiaTri::SoThuc(*n as f64),
                    (GiaTri::SoThuc(f), t) if t.starts_with('i') || t.starts_with('u') => {
                        GiaTri::SoNguyen(*f as i64)
                    }
                    (GiaTri::KyTu(c), t) if t.starts_with('u') || t.starts_with('i') => {
                        GiaTri::SoNguyen(*c as i64)
                    }
                    _ => v,
                })
            }

            BieuThuc::LanTruyenLoi { gia_tri, span } => {
                let v = self.tinh(gia_tri)?.giai_tham();
                match &v {
                    GiaTri::BienThe { bien_the, gia_tri: g, .. } => match &**bien_the {
                        "Ok" | "Some" => Ok(g.first().cloned().unwrap_or(GiaTri::Rong)),
                        _ => Err(Ngat::TraVe(v.clone())),
                    },
                    khac => Err(loi_kieu(
                        *span,
                        format!("`?` chỉ dùng được với `Result` hoặc `Option`, không phải `{}`", khac.ten_kieu()),
                        "`?` lấy giá trị bên trong `Ok`/`Some`, và trả về sớm khi gặp `Err`/`None`",
                    )),
                }
            }

            BieuThuc::Macro { ten, doi_so, span } => self.macro_dung_san(ten, doi_so, *span),

            BieuThuc::GoiHam { ham, doi_so, span } => self.goi_ham(ham, doi_so, *span),

            BieuThuc::GoiPhuongThuc { doi_tuong, ten, doi_so, span } => {
                self.goi_phuong_thuc(doi_tuong, ten, doi_so, *span)
            }
        }
    }
}

impl MayChay {
    /// Tính giá trị, đồng thời áp dụng luật **chuyển quyền sở hữu**.
    ///
    /// Dùng ở MỌI nơi giá trị đi từ chỗ này sang chỗ khác: `let b = a;`,
    /// `b = a;`, đối số hàm, và trường khi khởi tạo struct. Nếu chỉ cài ở một
    /// nơi thì bài học ownership sẽ đúng lúc này sai lúc khác — tệ hơn là không
    /// dạy gì cả, vì người học sẽ rút ra quy tắc sai.
    fn tinh_va_chuyen(&mut self, e: &BieuThuc) -> KQ<GiaTri> {
        let v = self.tinh(e)?;
        if !v.la_copy() {
            if let BieuThuc::DuongDan { doan, span } = e {
                if doan.len() == 1 {
                    if let Some(o) = self.tim_o(&doan[0]) {
                        // Chỉ đánh dấu nếu ô đang thực sự giữ giá trị (không phải
                        // đã bị chuyển từ trước — trường hợp đó `tinh` đã báo lỗi).
                        let can_danh_dau = !matches!(&*o.borrow(), GiaTri::DaChuyen { .. });
                        if can_danh_dau {
                            *o.borrow_mut() = GiaTri::DaChuyen { chuyen_tai: *span };
                        }
                    }
                }
            }
        }
        Ok(v)
    }
}

fn so_nguyen(v: GiaTri, span: Span) -> KQ<i64> {
    match v {
        GiaTri::SoNguyen(n) => Ok(n),
        khac => Err(loi_kieu(
            span,
            format!("cần một số nguyên, nhận được `{}`", khac.ten_kieu()),
            "vị trí này chỉ nhận số nguyên",
        )),
    }
}

fn loi_kieu(span: Span, thong_diep: String, vi_sao: &str) -> Ngat {
    Ngat::Loi(Box::new(
        Diagnostic::loi("BR0520", thong_diep)
            .tai(span, "ở đây")
            .vi_sao(vi_sao)
            .khai_niem("kiểu dữ liệu"),
    ))
}

fn loi_khong_co_truong(
    span: Span,
    ten_struct: &str,
    truong: &str,
    co_san: &HashMap<String, GiaTri>,
) -> Ngat {
    let mut ten: Vec<&String> = co_san.keys().collect();
    ten.sort();
    let ds = ten.iter().map(|s| format!("`{s}`")).collect::<Vec<_>>().join(", ");
    let mut d = Diagnostic::loi("BR0511", format!("`{ten_struct}` không có trường `{truong}`"))
        .tai(span, "trường này không tồn tại")
        .khai_niem("struct");
    d = if ds.is_empty() {
        d.vi_sao(format!("`{ten_struct}` không có trường nào"))
    } else {
        d.vi_sao(format!("`{ten_struct}` chỉ có các trường: {ds}"))
    };
    Ngat::Loi(Box::new(d))
}

// ── Đường dẫn, phép hai ngôi, gán, macro, lời gọi ───────────────────────────

impl MayChay {
    fn tinh_duong_dan(&mut self, doan: &[String], span: Span) -> KQ<GiaTri> {
        // Biến thường
        if doan.len() == 1 {
            let ten = &doan[0];
            if let Some(o) = self.tim_o(ten) {
                let v = o.borrow().clone();
                if let GiaTri::DaChuyen { chuyen_tai } = v {
                    return Err(Ngat::Loi(Box::new(
                        Diagnostic::loi("BR0530", format!("`{ten}` đã bị chuyển quyền sở hữu đi nơi khác"))
                            .nhan(Label::chinh(span, "dùng lại ở đây thì không còn giá trị nữa"))
                            .nhan(Label::phu(chuyen_tai, "quyền sở hữu bị chuyển đi tại đây"))
                            .vi_sao("Rust cho mỗi giá trị đúng MỘT chủ sở hữu. Khi bạn gán nó sang biến khác hoặc truyền vào hàm, chủ cũ mất quyền — nhờ luật này Rust không cần bộ dọn rác mà vẫn không bao giờ dùng nhầm bộ nhớ đã giải phóng.")
                            .sua(format!("nếu muốn giữ cả hai, hãy nhân bản: `{ten}.clone()`"))
                            .sua(format!("hoặc chỉ mượn thay vì lấy hẳn: `&{ten}`"))
                            .khai_niem("quyền sở hữu"),
                    )));
                }
                return Ok(v);
            }
            // `None` viết trần — rất phổ biến, phải nhận ra.
            if ten == "None" {
                return Ok(GiaTri::BienThe {
                    enum_ten: Rc::from("Option"),
                    bien_the: Rc::from("None"),
                    gia_tri: Rc::new(vec![]),
                });
            }
            // Biến thể enum không tham số: `Red` khi đã `use Color::*`
            if let Some(en) = self.bien_the_cua.get(ten).cloned() {
                return Ok(GiaTri::BienThe {
                    enum_ten: Rc::from(en.as_str()),
                    bien_the: Rc::from(ten.as_str()),
                    gia_tri: Rc::new(vec![]),
                });
            }
            // Struct rỗng: `struct Marker;`
            if let Some(sd) = self.struct_def.get(ten) {
                if matches!(sd.than, ThanStruct::Rong) {
                    return Ok(GiaTri::Struct {
                        ten: Rc::from(ten.as_str()),
                        truong: Rc::new(RefCell::new(HashMap::new())),
                    });
                }
            }
            if self.ham.contains_key(ten) {
                return Ok(GiaTri::Chuoi(Rc::new(format!("<hàm {ten}>"))));
            }
            return Err(self.loi_khong_thay_ten(ten, span));
        }

        // `Enum::BienThe`
        let cuoi = doan.last().unwrap();
        let dau = &doan[doan.len() - 2];
        if self.enum_def.contains_key(dau) || matches!(dau.as_str(), "Option" | "Result") {
            return Ok(GiaTri::BienThe {
                enum_ten: Rc::from(dau.as_str()),
                bien_the: Rc::from(cuoi.as_str()),
                gia_tri: Rc::new(vec![]),
            });
        }
        if matches!(cuoi.as_str(), "None") {
            return Ok(GiaTri::BienThe {
                enum_ten: Rc::from("Option"),
                bien_the: Rc::from("None"),
                gia_tri: Rc::new(vec![]),
            });
        }
        // `Kieu::phuong_thuc` — trả về đánh dấu để `goi_ham` xử lý.
        Ok(GiaTri::Chuoi(Rc::new(format!("<đường dẫn {}>", doan.join("::")))))
    }

    #[allow(clippy::too_many_arguments)]
    fn hai_ngoi(
        &mut self,
        op: ToanTuHai,
        t: GiaTri,
        p: GiaTri,
        span: Span,
        span_t: Span,
        _span_p: Span,
    ) -> KQ<GiaTri> {
        use GiaTri::*;
        use ToanTuHai::*;

        // So sánh dùng được cho mọi kiểu so sánh được.
        match op {
            Bang => return Ok(DungSai(t.bang(&p))),
            KhacBang => return Ok(DungSai(!t.bang(&p))),
            NhoHon | LonHon | NhoBang | LonBang => {
                let Some(o) = t.so_sanh(&p) else {
                    return Err(loi_kieu(
                        span,
                        format!("không so sánh được `{}` với `{}`", t.ten_kieu(), p.ten_kieu()),
                        "hai vế của phép so sánh phải cùng kiểu",
                    ));
                };
                use std::cmp::Ordering::*;
                return Ok(DungSai(match op {
                    NhoHon => o == Less,
                    LonHon => o == Greater,
                    NhoBang => o != Greater,
                    _ => o != Less,
                }));
            }
            _ => {}
        }

        // Nối chuỗi: `String + &str`
        if op == Cong {
            if let (Chuoi(a), Chuoi(b)) = (&t, &p) {
                return Ok(Chuoi(Rc::new(format!("{a}{b}"))));
            }
            if matches!(t, Chuoi(_)) || matches!(p, Chuoi(_)) {
                return Err(Ngat::Loi(Box::new(
                    Diagnostic::loi("BR0531", format!("không cộng được `{}` với `{}`", t.ten_kieu(), p.ten_kieu()))
                        .tai(span, "phép cộng ở đây")
                        .vi_sao("Rust không tự đổi số thành chuỗi như JavaScript hay Python — vì đổi ngầm là nguồn của rất nhiều lỗi khó tìm")
                        .sua("dùng `format!(\"{}{}\", a, b)` để ghép chuỗi với số")
                        .sua("hoặc đổi số thành chuỗi trước: `a + &b.to_string()`")
                        .khai_niem("chuỗi văn bản"),
                )));
            }
        }

        match (&t, &p) {
            (SoNguyen(a), SoNguyen(b)) => {
                let (a, b) = (*a, *b);
                let r = match op {
                    Cong => a.checked_add(b),
                    Tru => a.checked_sub(b),
                    Nhan => a.checked_mul(b),
                    Chia | Du => {
                        if b == 0 {
                            let la_chia = op == Chia;
                            return Err(Ngat::Loi(Box::new(
                                Diagnostic::loi("BR0532", if la_chia { "chia cho 0" } else { "lấy dư cho 0" })
                                    .tai(span, "phép tính này không hợp lệ")
                                    .vi_sao("chia cho 0 không có kết quả trong toán học, nên Rust dừng chương trình thay vì trả về giá trị vô nghĩa")
                                    .sua("kiểm tra mẫu số trước: `if b != 0 { a / b }`")
                                    .sua("hoặc dùng `a.checked_div(b)` để nhận `Option` thay vì dừng")
                                    .khai_niem("phép chia"),
                            )));
                        }
                        if la_chia_op(op) { a.checked_div(b) } else { a.checked_rem(b) }
                    }
                    VaBit => Some(a & b),
                    HoacBit => Some(a | b),
                    XorBit => Some(a ^ b),
                    DichTrai => a.checked_shl(b as u32),
                    DichPhai => a.checked_shr(b as u32),
                    _ => None,
                };
                match r {
                    Some(v) => Ok(SoNguyen(v)),
                    None => Err(Ngat::Loi(Box::new(
                        Diagnostic::loi("BR0533", "phép tính bị tràn số")
                            .tai(span, format!("`{a} {} {b}` vượt quá sức chứa của `i64`", op.ky_hieu()))
                            .vi_sao("`i64` chứa được từ -9,2 tỷ tỷ đến 9,2 tỷ tỷ. Rust phát hiện tràn và dừng, thay vì lặng lẽ cho ra số sai như C.")
                            .sua("dùng `checked_add`/`checked_mul` để nhận `Option` và tự xử lý")
                            .khai_niem("số nguyên"),
                    ))),
                }
            }
            (SoThuc(_), _) | (_, SoThuc(_)) => {
                let (a, b) = match (&t, &p) {
                    (SoThuc(a), SoThuc(b)) => (*a, *b),
                    (SoThuc(a), SoNguyen(b)) => (*a, *b as f64),
                    (SoNguyen(a), SoThuc(b)) => (*a as f64, *b),
                    _ => {
                        return Err(loi_kieu(
                            span,
                            format!("không tính được `{}` {} `{}`", t.ten_kieu(), op.ky_hieu(), p.ten_kieu()),
                            "hai vế phải cùng kiểu số",
                        ))
                    }
                };
                Ok(SoThuc(match op {
                    Cong => a + b,
                    Tru => a - b,
                    Nhan => a * b,
                    Chia => a / b,
                    Du => a % b,
                    _ => {
                        return Err(loi_kieu(
                            span,
                            format!("không dùng được `{}` với số thực", op.ky_hieu()),
                            "phép trên bit chỉ dùng cho số nguyên",
                        ))
                    }
                }))
            }
            (DungSai(_), _) | (_, DungSai(_)) => Err(Ngat::Loi(Box::new(
                Diagnostic::loi("BR0534", format!("không dùng được `{}` với `bool`", op.ky_hieu()))
                    .tai(span, "ở đây")
                    .vi_sao("`true` và `false` không phải số, nên không cộng trừ được")
                    .sua("nếu muốn kết hợp điều kiện, dùng `&&` (và) hoặc `||` (hoặc)")
                    .khai_niem("kiểu bool"),
            ))),
            _ => Err(Ngat::Loi(Box::new(
                Diagnostic::loi("BR0535", format!("không tính được `{}` {} `{}`", t.ten_kieu(), op.ky_hieu(), p.ten_kieu()))
                    .nhan(Label::chinh(span, "phép tính ở đây"))
                    .nhan(Label::phu(span_t, format!("vế trái là `{}`", t.ten_kieu())))
                    .vi_sao("Rust yêu cầu hai vế cùng kiểu và kiểu đó phải hỗ trợ phép này")
                    .khai_niem("kiểu dữ liệu"),
            ))),
        }
    }

    /// Gán giá trị vào một vị trí, áp dụng luật chuyển quyền sở hữu.
    fn gan_vao(&mut self, dich: &BieuThuc, gt: GiaTri) -> KQ<()> {
        match dich {
            BieuThuc::DuongDan { doan, span } if doan.len() == 1 => {
                let Some(o) = self.tim_o(&doan[0]) else {
                    return Err(self.loi_khong_thay_ten(&doan[0], *span));
                };
                *o.borrow_mut() = gt;
                Ok(())
            }
            BieuThuc::TruyCapTruong { doi_tuong, ten, span } => {
                let v = self.tinh(doi_tuong)?.giai_tham();
                match v {
                    GiaTri::Struct { truong, .. } => {
                        truong.borrow_mut().insert(ten.clone(), gt);
                        Ok(())
                    }
                    khac => Err(loi_kieu(
                        *span,
                        format!("`{}` không có trường để gán", khac.ten_kieu()),
                        "chỉ struct mới gán được vào trường",
                    )),
                }
            }
            BieuThuc::ChiSo { doi_tuong, chi_so, span } => {
                let v = self.tinh(doi_tuong)?.giai_tham();
                let i = so_nguyen(self.tinh(chi_so)?.giai_tham(), chi_so.span())?;
                let GiaTri::Day(day) = v else {
                    return Err(loi_kieu(*span, "không gán được vào vị trí này".into(), "chỉ `Vec` và mảng mới gán được theo chỉ số"));
                };
                let mut d = day.borrow_mut();
                if i < 0 || i as usize >= d.len() {
                    return Err(Ngat::Loi(Box::new(
                        Diagnostic::loi("BR0509", "chỉ số vượt ngoài phạm vi")
                            .tai(chi_so.span(), format!("chỉ số {i}, dãy có {} phần tử", d.len()))
                            .khai_niem("chỉ số"),
                    )));
                }
                d[i as usize] = gt;
                Ok(())
            }
            BieuThuc::GiaiTham { gia_tri, span } => {
                let v = self.tinh(gia_tri)?;
                match v {
                    GiaTri::ThamChieu { o, co_the_sua: true } => {
                        *o.borrow_mut() = gt;
                        Ok(())
                    }
                    GiaTri::ThamChieu { .. } => Err(Ngat::Loi(Box::new(
                        Diagnostic::loi("BR0536", "không sửa được qua tham chiếu chỉ đọc")
                            .tai(*span, "tham chiếu này là `&T`, không phải `&mut T`")
                            .vi_sao("`&T` cho phép ĐỌC; muốn ghi thì phải mượn dạng `&mut T`")
                            .sua("đổi chỗ mượn thành `&mut`, và biến gốc phải khai báo `let mut`")
                            .khai_niem("mượn"),
                    ))),
                    khac => Err(loi_kieu(*span, format!("`*` cần tham chiếu, nhận `{}`", khac.ten_kieu()), "dấu `*` chỉ dùng với tham chiếu")),
                }
            }
            khac => Err(loi_kieu(
                khac.span(),
                "vế trái của phép gán không phải nơi lưu được giá trị".into(),
                "chỉ gán được vào biến, trường của struct, hoặc phần tử của dãy",
            )),
        }
    }

    fn macro_dung_san(&mut self, ten: &str, doi_so: &[BieuThuc], span: Span) -> KQ<GiaTri> {
        match ten {
            "println" | "print" | "format" | "eprintln" => {
                let s = self.dinh_dang(doi_so, span)?;
                match ten {
                    "format" => Ok(GiaTri::Chuoi(Rc::new(s))),
                    "print" => {
                        self.in_ra(&s);
                        Ok(GiaTri::Rong)
                    }
                    _ => {
                        self.in_ra(&s);
                        self.in_ra("\n");
                        Ok(GiaTri::Rong)
                    }
                }
            }
            "vec" => {
                let mut v = Vec::new();
                for e in doi_so {
                    v.push(self.tinh(e)?);
                }
                Ok(GiaTri::Day(Rc::new(RefCell::new(v))))
            }
            "assert" => {
                let v = self.tinh(&doi_so[0])?.giai_tham();
                if v.la_dung() == Some(true) {
                    Ok(GiaTri::Rong)
                } else {
                    Err(Ngat::Loi(Box::new(
                        Diagnostic::loi("BR0540", "khẳng định `assert!` không đúng")
                            .tai(doi_so[0].span(), "biểu thức này cho ra `false`")
                            .vi_sao("`assert!` dùng để nói \"chỗ này chắc chắn phải đúng\"; nếu sai thì chương trình dừng ngay để lỗi không lan xa hơn")
                            .khai_niem("kiểm thử"),
                    )))
                }
            }
            "assert_eq" | "assert_ne" => {
                let a = self.tinh(&doi_so[0])?.giai_tham();
                let b = self.tinh(&doi_so[1])?.giai_tham();
                let bang = a.bang(&b);
                let mong_bang = ten == "assert_eq";
                if bang == mong_bang {
                    Ok(GiaTri::Rong)
                } else {
                    Err(Ngat::Loi(Box::new(
                        Diagnostic::loi("BR0541", if mong_bang { "hai giá trị không bằng nhau" } else { "hai giá trị lại bằng nhau" })
                            .tai(span, format!("trái  = {}\n           phải = {}", a.go_ro(), b.go_ro()))
                            .vi_sao("`assert_eq!` kiểm tra hai vế bằng nhau và in ra cả hai khi sai, để bạn thấy ngay chênh ở đâu")
                            .khai_niem("kiểm thử"),
                    )))
                }
            }
            khac => Err(Ngat::Loi(Box::new(
                Diagnostic::loi("BR0542", format!("chưa hỗ trợ macro `{khac}!`"))
                    .tai(span, "macro này chưa dùng được trong bài học")
                    .vi_sao("Byte Academy chỉ chạy phần Rust cần cho giáo trình, để thông báo lỗi luôn dễ hiểu")
                    .sua("các macro dùng được: `println!`, `print!`, `format!`, `vec!`, `assert!`, `assert_eq!`")
                    .khai_niem("macro"),
            ))),
        }
    }

    /// Xử lý chuỗi định dạng `"{} và {:?}"`.
    fn dinh_dang(&mut self, doi_so: &[BieuThuc], span: Span) -> KQ<String> {
        if doi_so.is_empty() {
            return Ok(String::new());
        }
        let mau = match self.tinh(&doi_so[0])?.giai_tham() {
            GiaTri::Chuoi(s) => s.to_string(),
            khac => return Ok(khac.hien_thi()),
        };
        let mut gia_tri = Vec::new();
        for e in &doi_so[1..] {
            gia_tri.push(self.tinh(e)?);
        }

        let mut ra = String::new();
        let mut i = 0usize;
        let b: Vec<char> = mau.chars().collect();
        let mut k = 0usize;
        while k < b.len() {
            match b[k] {
                '{' if b.get(k + 1) == Some(&'{') => { ra.push('{'); k += 2; }
                '}' if b.get(k + 1) == Some(&'}') => { ra.push('}'); k += 2; }
                '{' => {
                    let mut j = k + 1;
                    let mut spec = String::new();
                    while j < b.len() && b[j] != '}' {
                        spec.push(b[j]);
                        j += 1;
                    }
                    if j >= b.len() {
                        return Err(Ngat::Loi(Box::new(
                            Diagnostic::loi("BR0543", "dấu `{` trong chuỗi định dạng chưa được đóng")
                                .tai(span, "thiếu `}`")
                                .sua("mỗi `{` cần một `}` đi kèm; muốn in dấu ngoặc thật thì viết `{{`")
                                .khai_niem("println"),
                        )));
                    }
                    let Some(v) = gia_tri.get(i) else {
                        return Err(Ngat::Loi(Box::new(
                            Diagnostic::loi("BR0544", "chuỗi định dạng cần nhiều giá trị hơn số đã truyền")
                                .tai(span, format!("có {} chỗ `{{}}` nhưng chỉ truyền {} giá trị", i + 1, gia_tri.len()))
                                .vi_sao("mỗi `{}` trong chuỗi lấy một giá trị theo thứ tự")
                                .sua("thêm giá trị vào sau chuỗi, ví dụ `println!(\"{} {}\", a, b)`")
                                .khai_niem("println"),
                        )));
                    };
                    ra.push_str(&if spec.contains('?') { v.go_ro() } else { v.hien_thi() });
                    i += 1;
                    k = j + 1;
                }
                c => { ra.push(c); k += 1; }
            }
        }
        Ok(ra)
    }
}

fn la_chia_op(op: ToanTuHai) -> bool {
    matches!(op, ToanTuHai::Chia)
}

// ── Lời gọi hàm & phương thức ───────────────────────────────────────────────

impl MayChay {
    /// Tính đối số, đồng thời áp dụng luật chuyển quyền sở hữu.
    ///
    /// Truyền một `String` vào hàm là **chuyển** nó đi; biến gốc trở thành
    /// `DaChuyen`, và lần dùng sau sẽ báo lỗi chỉ đúng chỗ đã chuyển.
    fn tinh_doi_so(&mut self, doi_so: &[BieuThuc]) -> KQ<Vec<GiaTri>> {
        let mut ra = Vec::with_capacity(doi_so.len());
        for e in doi_so {
            ra.push(self.tinh_va_chuyen(e)?);
        }
        Ok(ra)
    }

    fn goi_ham(&mut self, ham: &BieuThuc, doi_so: &[BieuThuc], span: Span) -> KQ<GiaTri> {
        // Hàm dựng sẵn của enum: `Some(x)`, `Ok(x)`, `Err(e)`, hoặc biến thể tự định nghĩa.
        if let BieuThuc::DuongDan { doan, .. } = ham {
            let cuoi = doan.last().cloned().unwrap_or_default();

            if matches!(cuoi.as_str(), "Some" | "Ok" | "Err") {
                let gt = self.tinh_doi_so(doi_so)?;
                let en = if cuoi == "Some" { "Option" } else { "Result" };
                return Ok(GiaTri::BienThe {
                    enum_ten: Rc::from(en),
                    bien_the: Rc::from(cuoi.as_str()),
                    gia_tri: Rc::new(gt),
                });
            }
            if let Some(en) = self.bien_the_cua.get(&cuoi).cloned() {
                let gt = self.tinh_doi_so(doi_so)?;
                return Ok(GiaTri::BienThe {
                    enum_ten: Rc::from(en.as_str()),
                    bien_the: Rc::from(cuoi.as_str()),
                    gia_tri: Rc::new(gt),
                });
            }
            // Struct dạng tuple: `Email("a@b.c")`
            if let Some(sd) = self.struct_def.get(&cuoi).cloned() {
                if let ThanStruct::TheoViTri(_) = sd.than {
                    let gt = self.tinh_doi_so(doi_so)?;
                    let mut m = HashMap::new();
                    for (i, v) in gt.into_iter().enumerate() {
                        m.insert(i.to_string(), v);
                    }
                    return Ok(GiaTri::Struct {
                        ten: Rc::from(cuoi.as_str()),
                        truong: Rc::new(RefCell::new(m)),
                    });
                }
            }
            // Hàm tĩnh dựng sẵn
            if doan.len() >= 2 {
                let kieu = &doan[doan.len() - 2];
                match (kieu.as_str(), cuoi.as_str()) {
                    ("Vec", "new") => return Ok(GiaTri::Day(Rc::new(RefCell::new(Vec::new())))),
                    ("String", "new") => return Ok(GiaTri::Chuoi(Rc::new(String::new()))),
                    ("String", "from") => {
                        let gt = self.tinh_doi_so(doi_so)?;
                        return Ok(GiaTri::Chuoi(Rc::new(
                            gt.first().map(|v| v.hien_thi()).unwrap_or_default(),
                        )));
                    }
                    _ => {}
                }
                // Phương thức tĩnh do người học định nghĩa: `Point::moi(...)`
                if let Some(h) = self.phuong_thuc.get(&(kieu.clone(), cuoi.clone())).cloned() {
                    let gt = self.tinh_doi_so(doi_so)?;
                    return self.goi(&h, gt, span);
                }
            }
            // Hàm người học định nghĩa
            if let Some(h) = self.ham.get(&cuoi).cloned() {
                let gt = self.tinh_doi_so(doi_so)?;
                return self.goi(&h, gt, span);
            }
            // Không có: gợi ý tên gần giống
            let mut gan: Option<&String> = None;
            let mut tot = usize::MAX;
            for k in self.ham.keys() {
                let d = khoang_cach_sua(&cuoi, k);
                if d < tot && d <= 2 {
                    tot = d;
                    gan = Some(k);
                }
            }
            let mut d = Diagnostic::loi("BR0550", format!("không tìm thấy hàm `{cuoi}`"))
                .tai(span, "hàm này chưa được định nghĩa")
                .vi_sao("Rust cần thấy khai báo của hàm trước khi gọi nó")
                .khai_niem("hàm");
            d = match gan {
                Some(g) => d.sua(format!("có phải bạn định gọi `{g}` không?")),
                None => d.sua(format!("định nghĩa nó: `fn {cuoi}(...) {{ ... }}`")),
            };
            return Err(Ngat::Loi(Box::new(d)));
        }

        // Gọi closure đang giữ trong biến
        let f = self.tinh(ham)?.giai_tham();
        let gt = self.tinh_doi_so(doi_so)?;
        self.goi_be_quan(f, gt, span)
    }

    fn goi_be_quan(&mut self, f: GiaTri, doi_so: Vec<GiaTri>, span: Span) -> KQ<GiaTri> {
        let GiaTri::BeQuan(bq) = f else {
            return Err(loi_kieu(
                span,
                format!("`{}` không phải là hàm nên không gọi được", f.ten_kieu()),
                "chỉ hàm và closure mới đứng trước dấu `(` để gọi",
            ));
        };
        if doi_so.len() != bq.tham_so.len() {
            return Err(Ngat::Loi(Box::new(
                Diagnostic::loi("BR0506", format!("closure cần {} đối số nhưng nhận {}", bq.tham_so.len(), doi_so.len()))
                    .tai(span, "lời gọi ở đây")
                    .khai_niem("closure"),
            )));
        }
        if self.do_sau_goi >= GIOI_HAN_DO_SAU {
            return Err(Ngat::Loi(Box::new(
                Diagnostic::loi("BR0505", "hàm gọi lồng nhau quá sâu")
                    .tai(span, "dừng ở đây")
                    .vi_sao("nhiều khả năng closure đang gọi lại chính nó mà không có điều kiện dừng")
                    .khai_niem("đệ quy"),
            )));
        }
        let luu = core::mem::replace(
            &mut self.pham,
            bq.bat.iter().map(|b| Pham { bien: b.clone() }).collect(),
        );
        self.do_sau_goi += 1;
        self.vao_pham();
        let mut kq = Ok(GiaTri::Rong);
        for (m, v) in bq.tham_so.iter().zip(doi_so) {
            if let Err(e) = self.rang_buoc_mau(m, v) {
                kq = Err(e);
                break;
            }
        }
        if kq.is_ok() {
            kq = self.tinh(&bq.than);
        }
        self.do_sau_goi -= 1;
        self.pham = luu;
        match kq {
            Err(Ngat::TraVe(v)) => Ok(v),
            khac => khac,
        }
    }

    fn goi_phuong_thuc(
        &mut self,
        doi_tuong: &BieuThuc,
        ten: &str,
        doi_so: &[BieuThuc],
        span: Span,
    ) -> KQ<GiaTri> {
        let chu = self.tinh(doi_tuong)?;
        let chu_that = chu.giai_tham();

        // Phương thức do người học định nghĩa được ưu tiên.
        let ten_kieu = match &chu_that {
            GiaTri::Struct { ten, .. } => ten.to_string(),
            GiaTri::BienThe { enum_ten, .. } => enum_ten.to_string(),
            khac => khac.ten_kieu(),
        };
        if let Some(h) = self.phuong_thuc.get(&(ten_kieu.clone(), ten.to_string())).cloned() {
            let mut gt = vec![chu.clone()];
            gt.extend(self.tinh_doi_so(doi_so)?);
            return self.goi(&h, gt, span);
        }

        let ds = self.tinh_doi_so(doi_so)?;
        self.phuong_thuc_dung_san(&chu_that, ten, ds, span, doi_tuong.span())
    }

    fn phuong_thuc_dung_san(
        &mut self,
        chu: &GiaTri,
        ten: &str,
        ds: Vec<GiaTri>,
        span: Span,
        span_chu: Span,
    ) -> KQ<GiaTri> {
        use GiaTri::*;
        match (chu, ten) {
            // ── chung ──
            (v, "clone") => Ok(v.clone()),
            (v, "to_string") => Ok(Chuoi(Rc::new(v.hien_thi()))),

            // ── Vec ──
            (Day(d), "len") => Ok(SoNguyen(d.borrow().len() as i64)),
            (Day(d), "is_empty") => Ok(DungSai(d.borrow().is_empty())),
            (Day(d), "push") => {
                d.borrow_mut().push(ds.into_iter().next().unwrap_or(Rong));
                Ok(Rong)
            }
            (Day(d), "pop") => Ok(tuy_chon(d.borrow_mut().pop())),
            (Day(d), "first") => Ok(tuy_chon(d.borrow().first().cloned())),
            (Day(d), "last") => Ok(tuy_chon(d.borrow().last().cloned())),
            (Day(d), "get") => {
                let i = so_nguyen(ds.into_iter().next().unwrap_or(Rong), span)?;
                let d = d.borrow();
                Ok(tuy_chon(if i >= 0 { d.get(i as usize).cloned() } else { None }))
            }
            (Day(d), "contains") => {
                let x = ds.into_iter().next().unwrap_or(Rong);
                Ok(DungSai(d.borrow().iter().any(|y| y.bang(&x))))
            }
            (Day(d), "reverse") => {
                d.borrow_mut().reverse();
                Ok(Rong)
            }
            (Day(d), "sort") => {
                let mut v = d.borrow_mut();
                v.sort_by(|a, b| a.so_sanh(b).unwrap_or(std::cmp::Ordering::Equal));
                Ok(Rong)
            }
            (Day(_), "iter" | "into_iter" | "iter_mut") => Ok(chu.clone()),
            (Day(d), "sum") => {
                let v = d.borrow();
                if v.iter().any(|x| matches!(x, SoThuc(_))) {
                    let mut t = 0.0;
                    for x in v.iter() {
                        match x.giai_tham() {
                            SoThuc(f) => t += f,
                            SoNguyen(n) => t += n as f64,
                            _ => {}
                        }
                    }
                    Ok(SoThuc(t))
                } else {
                    let mut t: i64 = 0;
                    for x in v.iter() {
                        if let SoNguyen(n) = x.giai_tham() {
                            t = t.checked_add(n).ok_or_else(|| {
                                Ngat::Loi(Box::new(
                                    Diagnostic::loi("BR0533", "tổng bị tràn số").tai(span, "ở đây"),
                                ))
                            })?;
                        }
                    }
                    Ok(SoNguyen(t))
                }
            }
            (Day(d), "count") => Ok(SoNguyen(d.borrow().len() as i64)),
            (Day(d), "collect") => Ok(Day(Rc::new(RefCell::new(d.borrow().clone())))),
            (Day(d), "map") => {
                let f = ds.into_iter().next().unwrap_or(Rong);
                let cu = d.borrow().clone();
                let mut ra = Vec::with_capacity(cu.len());
                for x in cu {
                    ra.push(self.goi_be_quan(f.clone(), vec![x], span)?);
                }
                Ok(Day(Rc::new(RefCell::new(ra))))
            }
            (Day(d), "filter") => {
                let f = ds.into_iter().next().unwrap_or(Rong);
                let cu = d.borrow().clone();
                let mut ra = Vec::new();
                for x in cu {
                    if self.goi_be_quan(f.clone(), vec![x.clone()], span)?.giai_tham().la_dung() == Some(true) {
                        ra.push(x);
                    }
                }
                Ok(Day(Rc::new(RefCell::new(ra))))
            }
            (Day(d), "fold") => {
                let mut it = ds.into_iter();
                let mut acc = it.next().unwrap_or(Rong);
                let f = it.next().unwrap_or(Rong);
                let cu = d.borrow().clone();
                for x in cu {
                    acc = self.goi_be_quan(f.clone(), vec![acc, x], span)?;
                }
                Ok(acc)
            }

            // ── Chuỗi ──
            (Chuoi(s), "len") => Ok(SoNguyen(s.chars().count() as i64)),
            (Chuoi(s), "is_empty") => Ok(DungSai(s.is_empty())),
            (Chuoi(s), "to_uppercase") => Ok(Chuoi(Rc::new(s.to_uppercase()))),
            (Chuoi(s), "to_lowercase") => Ok(Chuoi(Rc::new(s.to_lowercase()))),
            (Chuoi(s), "trim") => Ok(Chuoi(Rc::new(s.trim().to_string()))),
            (Chuoi(s), "contains") => {
                let x = ds.first().map(|v| v.hien_thi()).unwrap_or_default();
                Ok(DungSai(s.contains(&x)))
            }
            (Chuoi(s), "starts_with") => {
                let x = ds.first().map(|v| v.hien_thi()).unwrap_or_default();
                Ok(DungSai(s.starts_with(&x)))
            }
            (Chuoi(s), "chars") => Ok(Day(Rc::new(RefCell::new(s.chars().map(KyTu).collect())))),
            (Chuoi(s), "split") => {
                let x = ds.first().map(|v| v.hien_thi()).unwrap_or_default();
                Ok(Day(Rc::new(RefCell::new(
                    s.split(&x as &str).map(|p| Chuoi(Rc::new(p.to_string()))).collect(),
                ))))
            }
            (Chuoi(s), "push_str") => {
                let x = ds.first().map(|v| v.hien_thi()).unwrap_or_default();
                Ok(Chuoi(Rc::new(format!("{s}{x}"))))
            }

            // ── số ──
            (SoNguyen(n), "abs") => Ok(SoNguyen(n.abs())),
            (SoNguyen(n), "pow") => {
                let e = so_nguyen(ds.into_iter().next().unwrap_or(Rong), span)?;
                n.checked_pow(e.max(0) as u32).map(SoNguyen).ok_or_else(|| {
                    Ngat::Loi(Box::new(
                        Diagnostic::loi("BR0533", "luỹ thừa bị tràn số").tai(span, "kết quả quá lớn cho `i64`"),
                    ))
                })
            }
            (SoNguyen(n), "min") => Ok(SoNguyen(*n.min(&so_nguyen(ds.into_iter().next().unwrap_or(Rong), span)?))),
            (SoNguyen(n), "max") => Ok(SoNguyen(*n.max(&so_nguyen(ds.into_iter().next().unwrap_or(Rong), span)?))),
            (SoThuc(f), "sqrt") => Ok(SoThuc(f.sqrt())),
            (SoThuc(f), "abs") => Ok(SoThuc(f.abs())),
            (SoThuc(f), "round") => Ok(SoThuc(f.round())),

            // ── Option / Result ──
            (BienThe { bien_the, gia_tri, .. }, "unwrap") => match &**bien_the {
                "Some" | "Ok" => Ok(gia_tri.first().cloned().unwrap_or(Rong)),
                khac => Err(Ngat::Loi(Box::new(
                    Diagnostic::loi("BR0551", format!("`unwrap()` gọi trên `{khac}`"))
                        .nhan(Label::chinh(span, "chương trình dừng ở đây"))
                        .nhan(Label::phu(span_chu, format!("giá trị này là `{}`", chu.go_ro())))
                        .vi_sao("`unwrap()` nghĩa là \"tôi chắc chắn có giá trị\". Khi không có, Rust dừng ngay thay vì đi tiếp với dữ liệu rỗng.")
                        .sua("dùng `match` để xử lý cả hai trường hợp")
                        .sua("hoặc `.unwrap_or(giá_trị_mặc_định)` để có phương án dự phòng")
                        .khai_niem("Option và Result"),
                ))),
            },
            (BienThe { bien_the, gia_tri, .. }, "unwrap_or") => match &**bien_the {
                "Some" | "Ok" => Ok(gia_tri.first().cloned().unwrap_or(Rong)),
                _ => Ok(ds.into_iter().next().unwrap_or(Rong)),
            },
            (BienThe { bien_the, .. }, "is_some") => Ok(DungSai(&**bien_the == "Some")),
            (BienThe { bien_the, .. }, "is_none") => Ok(DungSai(&**bien_the == "None")),
            (BienThe { bien_the, .. }, "is_ok") => Ok(DungSai(&**bien_the == "Ok")),
            (BienThe { bien_the, .. }, "is_err") => Ok(DungSai(&**bien_the == "Err")),
            (BienThe { enum_ten, bien_the, gia_tri }, "map") => {
                let f = ds.into_iter().next().unwrap_or(Rong);
                match &**bien_the {
                    "Some" | "Ok" => {
                        let x = gia_tri.first().cloned().unwrap_or(Rong);
                        let r = self.goi_be_quan(f, vec![x], span)?;
                        Ok(BienThe { enum_ten: enum_ten.clone(), bien_the: bien_the.clone(), gia_tri: Rc::new(vec![r]) })
                    }
                    _ => Ok(chu.clone()),
                }
            }
            (BienThe { bien_the, gia_tri, .. }, "and_then") => {
                let f = ds.into_iter().next().unwrap_or(Rong);
                match &**bien_the {
                    "Some" | "Ok" => {
                        let x = gia_tri.first().cloned().unwrap_or(Rong);
                        self.goi_be_quan(f, vec![x], span)
                    }
                    _ => Ok(chu.clone()),
                }
            }

            // ── Dải ──
            (Dai { tu, den, bao_gom_cuoi }, "rev") => {
                let het = if *bao_gom_cuoi { *den + 1 } else { *den };
                Ok(Day(Rc::new(RefCell::new((*tu..het).rev().map(SoNguyen).collect()))))
            }
            (Dai { tu, den, bao_gom_cuoi }, "collect" | "iter" | "into_iter") => {
                let het = if *bao_gom_cuoi { *den + 1 } else { *den };
                Ok(Day(Rc::new(RefCell::new((*tu..het).map(SoNguyen).collect()))))
            }
            (Dai { tu, den, bao_gom_cuoi }, "sum") => {
                let het = if *bao_gom_cuoi { *den + 1 } else { *den };
                Ok(SoNguyen((*tu..het).sum()))
            }
            (Dai { tu, den, bao_gom_cuoi }, m @ ("map" | "filter" | "fold" | "count")) => {
                let het = if *bao_gom_cuoi { *den + 1 } else { *den };
                let day = Day(Rc::new(RefCell::new((*tu..het).map(SoNguyen).collect())));
                self.phuong_thuc_dung_san(&day, m, ds, span, span_chu)
            }

            _ => {
                let mut d = Diagnostic::loi("BR0552", format!("`{}` không có phương thức `{ten}`", chu.ten_kieu()))
                    .tai(span, "phương thức này không tồn tại")
                    .khai_niem("phương thức");
                let goi_y: &[&str] = match chu {
                    Day(_) => &["len", "push", "pop", "get", "iter", "map", "filter", "fold", "sum", "sort", "contains"],
                    Chuoi(_) => &["len", "to_uppercase", "to_lowercase", "trim", "chars", "split", "contains"],
                    BienThe { .. } => &["unwrap", "unwrap_or", "is_some", "is_none", "map", "and_then"],
                    _ => &["clone", "to_string"],
                };
                if let Some(g) = goi_y.iter().find(|g| khoang_cach_sua(ten, g) <= 2) {
                    d = d.sua(format!("có phải bạn định gọi `{g}` không?"));
                } else {
                    d = d.sua(format!("`{}` dùng được: {}", chu.ten_kieu(), goi_y.join(", ")));
                }
                Err(Ngat::Loi(Box::new(d)))
            }
        }
    }
}

fn tuy_chon(v: Option<GiaTri>) -> GiaTri {
    match v {
        Some(x) => GiaTri::BienThe {
            enum_ten: Rc::from("Option"),
            bien_the: Rc::from("Some"),
            gia_tri: Rc::new(vec![x]),
        },
        None => GiaTri::BienThe {
            enum_ten: Rc::from("Option"),
            bien_the: Rc::from("None"),
            gia_tri: Rc::new(vec![]),
        },
    }
}
