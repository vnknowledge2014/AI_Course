//! Kiểm tra mượn — pha **tĩnh**, ba hình dạng hẹp mà AST khẳng định được.
//!
//! # Vì sao module này tồn tại
//!
//! ADR-002 §4 chốt rằng NLL định nghĩa trên MIR, không tái tạo trung thực trên
//! AST được — và `move_check.rs` giữ đúng ranh giới ấy. Nhưng "không tái tạo
//! được borrow checker" không có nghĩa là "không khẳng định được gì cả".
//!
//! Cổng đối chiếu `rustc` để lại đúng ba ca NHẬN OAN, và cả ba đều là mã
//! **thẳng hàng trong một khối**, không nhánh, không vòng lặp:
//!
//! ```ignore
//! let a = &mut v;  let b = &mut v;  a.push(2);   // E0499
//! let r = &mut v;  let n = v.len(); r.push(4);   // E0502
//! let r; { let x = 5; r = &x; }    println!(r);  // E0597
//! ```
//!
//! Trên mã thẳng hàng thì thứ tự trong AST **chính là** thứ tự thi hành, nên
//! ba hình dạng này quyết định được mà không cần CFG. Đó là toàn bộ phạm vi
//! của module này.
//!
//! # Nguyên tắc: thà im lặng còn hơn phán bừa
//!
//! Cổng theo dõi hai chế độ hỏng ngược nhau: **nhận oan** (báo Đạt cho mã
//! `rustc` từ chối) và **từ chối oan** (báo lỗi cho mã `rustc` chấp nhận). Cái
//! thứ hai còn tệ hơn với người học: họ viết đúng mà máy nói sai, và không có
//! cách nào biết vì sao.
//!
//! Nên mọi luật ở đây đều bỏ cuộc ngay khi gặp thứ nó không đọc nổi — có
//! nhánh, có vòng lặp, có closure. Bỏ cuộc nghĩa là **im lặng**; nó không bao
//! giờ nghĩa là "chắc không sao".

use crate::ast::*;
use crate::diag::{Diagnostic, Diagnostics, Label};
use crate::span::Span;

/// Một lần mượn `&mut x` đang được theo dõi, gắn với cái tên nhận nó.
struct MuonMut {
    /// Tên biến bị mượn (`v` trong `let a = &mut v`).
    bi_muon: String,
    /// Tên nhận lần mượn ấy (`a`).
    ten_muon: String,
    span: Span,
}

/// `x` trong `&mut x` / `&x` — chỉ nhận đường dẫn một đoạn.
///
/// Cố ý hẹp: `&mut v[0]`, `&mut s.f`, `&mut *p` đều trả `None` để module này
/// đứng ngoài. Mở rộng cần biết kiểu và nơi trỏ tới, mà pha này không có.
fn ten_bi_muon(bt: &BieuThuc) -> Option<(&str, bool)> {
    match bt {
        BieuThuc::Muon { co_the_sua, gia_tri, .. } => match &**gia_tri {
            BieuThuc::DuongDan { doan, .. } if doan.len() == 1 => {
                Some((doan[0].as_str(), *co_the_sua))
            }
            _ => None,
        },
        _ => None,
    }
}

/// Duyệt mọi biểu thức con. KHÔNG đi vào `Khoi` — khối con được quét riêng.
fn duyet(bt: &BieuThuc, f: &mut impl FnMut(&BieuThuc)) {
    f(bt);
    match bt {
        BieuThuc::MotNgoi { toan_hang, .. } => duyet(toan_hang, f),
        BieuThuc::HaiNgoi { trai, phai, .. } => {
            duyet(trai, f);
            duyet(phai, f);
        }
        BieuThuc::Muon { gia_tri, .. }
        | BieuThuc::GiaiTham { gia_tri, .. }
        | BieuThuc::LanTruyenLoi { gia_tri, .. } => duyet(gia_tri, f),
        BieuThuc::GoiHam { ham, doi_so, .. } => {
            duyet(ham, f);
            for d in doi_so {
                duyet(d, f);
            }
        }
        BieuThuc::GoiPhuongThuc { doi_tuong, doi_so, .. } => {
            duyet(doi_tuong, f);
            for d in doi_so {
                duyet(d, f);
            }
        }
        BieuThuc::TruyCapTruong { doi_tuong, .. } => duyet(doi_tuong, f),
        BieuThuc::ChiSo { doi_tuong, chi_so, .. } => {
            duyet(doi_tuong, f);
            duyet(chi_so, f);
        }
        BieuThuc::Gan { dich, gia_tri, .. } => {
            duyet(dich, f);
            duyet(gia_tri, f);
        }
        BieuThuc::Tuple { phan_tu, .. } | BieuThuc::Mang { phan_tu, .. } => {
            for p in phan_tu {
                duyet(p, f);
            }
        }
        // `println!("{}", r)` là một nút `Macro`, không phải `GoiHam`.
        //
        // Quên nhánh này thì mọi lần dùng qua macro trở nên vô hình, và cả ba
        // luật ở đây đều hỏng theo hướng nguy hiểm: chúng kết luận "cái tên ấy
        // không còn được dùng nữa" nên im lặng. Ca E0597 lọt đúng vì lẽ đó.
        BieuThuc::Macro { doi_so, .. } => {
            for d in doi_so {
                duyet(d, f);
            }
        }
        BieuThuc::TraVe { gia_tri, .. } | BieuThuc::Thoat { gia_tri, .. } => {
            if let Some(g) = gia_tri {
                duyet(g, f);
            }
        }
        _ => {}
    }
}

/// Câu lệnh này có chứa nhánh, vòng lặp hay closure không?
///
/// Gặp một cái là module bỏ cuộc: thứ tự trong AST thôi không còn nói được
/// thứ tự thi hành nữa.
fn co_re_nhanh(bt: &BieuThuc) -> bool {
    let mut thay = false;
    duyet(bt, &mut |b| {
        if matches!(
            b,
            BieuThuc::Neu { .. }
                | BieuThuc::KhopMau { .. }
                | BieuThuc::Lap { .. }
                | BieuThuc::Trong { .. }
                | BieuThuc::Cho { .. }
                | BieuThuc::BeQuan { .. }
        ) {
            thay = true;
        }
    });
    thay
}

/// Biểu thức này có ĐỌC tên `ten` không?
fn co_doc_ten(bt: &BieuThuc, ten: &str) -> bool {
    let mut thay = false;
    duyet(bt, &mut |b| {
        if let BieuThuc::DuongDan { doan, .. } = b {
            if doan.len() == 1 && doan[0] == ten {
                thay = true;
            }
        }
    });
    thay
}

/// Có đọc `ten` ở chỗ KHÔNG phải bên trong chính lần mượn `&mut ten` không?
///
/// Phân biệt này là cả điểm của E0502: `let r = &mut v;` có đọc `v`, nhưng đó
/// là chính lần mượn, không phải một lần dùng xen vào.
fn co_doc_ngoai_muon(bt: &BieuThuc, ten: &str) -> bool {
    if let BieuThuc::Muon { gia_tri, .. } = bt {
        if let BieuThuc::DuongDan { doan, .. } = &**gia_tri {
            if doan.len() == 1 && doan[0] == ten {
                return false;
            }
        }
    }
    co_doc_ten(bt, ten)
}

/// `ten` còn được đọc ở câu lệnh nào SAU chỉ số `tu` không?
fn con_dung_sau(k: &Khoi, tu: usize, ten: &str) -> bool {
    for cl in k.cau_lenh.iter().skip(tu + 1) {
        let bt = match cl {
            CauLenh::BieuThuc { bt, .. } => bt,
            CauLenh::Let { gia_tri: Some(g), .. } => g,
            _ => continue,
        };
        if co_doc_ten(bt, ten) {
            return true;
        }
    }
    match &k.gia_tri_cuoi {
        Some(cuoi) => co_doc_ten(cuoi, ten),
        None => false,
    }
}

/// Trong `trong`, có câu `r = &x` với `x` khai báo ngay trong `trong` không?
fn gan_muon_bien_noi_bo(trong: &Khoi, r: &str) -> Option<(String, Span)> {
    let mut khai_bao_noi_bo: Vec<&str> = Vec::new();
    for cl in &trong.cau_lenh {
        match cl {
            CauLenh::Let { mau: Mau::Ten { ten, .. }, .. } => khai_bao_noi_bo.push(ten),
            CauLenh::BieuThuc { bt: BieuThuc::Gan { dich, gia_tri, span, .. }, .. } => {
                let BieuThuc::DuongDan { doan, .. } = &**dich else { continue };
                if doan.len() != 1 || doan[0] != r {
                    continue;
                }
                if let Some((x, _)) = ten_bi_muon(gia_tri) {
                    if khai_bao_noi_bo.contains(&x) {
                        return Some((x.to_string(), *span));
                    }
                }
            }
            _ => {}
        }
    }
    None
}

/// Mọi khối con nằm thẳng trong `k` (một tầng).
fn khoi_con(k: &Khoi) -> Vec<&Khoi> {
    k.cau_lenh
        .iter()
        .filter_map(|cl| match cl {
            CauLenh::BieuThuc { bt: BieuThuc::Khoi(kk), .. } => Some(&**kk),
            _ => None,
        })
        .collect()
}

struct BoKiem<'a> {
    diags: &'a mut Diagnostics,
}

impl BoKiem<'_> {
    fn khoi(&mut self, k: &Khoi) {
        self.hai_muon_mut(k);
        self.doc_khi_dang_muon(k);
        self.song_lau_hon_thu_no_tro_toi(k);
        for con in khoi_con(k) {
            self.khoi(con);
        }
    }

    /// E0499 — hai `&mut` của cùng một biến cùng sống.
    ///
    /// NLL: lần mượn thứ nhất sống tới lần DÙNG cuối của nó. Nên chỉ báo khi
    /// cái tên nhận lần mượn thứ nhất còn được đọc SAU chỗ lần mượn thứ hai
    /// được tạo. Không có điều kiện ấy thì `rustc` cho qua, và báo lỗi ở đây
    /// sẽ là một lần từ chối oan.
    fn hai_muon_mut(&mut self, k: &Khoi) {
        let mut dang_mo: Vec<MuonMut> = Vec::new();
        for (i, cl) in k.cau_lenh.iter().enumerate() {
            let CauLenh::Let { mau, gia_tri: Some(gt), span, .. } = cl else { continue };
            if co_re_nhanh(gt) {
                return;
            }
            let Mau::Ten { ten: ten_moi, .. } = mau else { continue };
            let Some((bi_muon, true)) = ten_bi_muon(gt) else { continue };

            if let Some(truoc) = dang_mo.iter().find(|m| m.bi_muon == bi_muon) {
                if con_dung_sau(k, i, &truoc.ten_muon) {
                    let msg = format!("`{bi_muon}` bị mượn khả biến hai lần cùng lúc");
                    let phu = format!(
                        "`{}` mượn lần thứ nhất ở đây, và còn được dùng sau",
                        truoc.ten_muon
                    );
                    self.diags.push(
                        Diagnostic::loi("BR0500", msg)
                            .nhan(Label::chinh(*span, "lần mượn thứ hai ở đây"))
                            .nhan(Label::phu(truoc.span, phu))
                            .vi_sao(
                                "Một lúc chỉ được có ĐÚNG MỘT tham chiếu khả biến tới một giá \
                                 trị. Hai cái cùng sống thì hai chỗ trong chương trình cùng sửa \
                                 được một thứ, và không ai đoán nổi thứ tự.",
                            ),
                    );
                    return;
                }
            }
            dang_mo.push(MuonMut {
                bi_muon: bi_muon.to_string(),
                ten_muon: ten_moi.clone(),
                span: *span,
            });
        }
    }

    /// E0502 — đọc thẳng biến trong lúc nó đang bị mượn khả biến.
    fn doc_khi_dang_muon(&mut self, k: &Khoi) {
        for (i, cl) in k.cau_lenh.iter().enumerate() {
            let CauLenh::Let { mau, gia_tri: Some(gt), span, .. } = cl else { continue };
            if co_re_nhanh(gt) {
                return;
            }
            let Mau::Ten { ten: ten_muon, .. } = mau else { continue };
            let Some((bi_muon, true)) = ten_bi_muon(gt) else { continue };

            for j in (i + 1)..k.cau_lenh.len() {
                let bt = match &k.cau_lenh[j] {
                    CauLenh::BieuThuc { bt, .. } => bt,
                    CauLenh::Let { gia_tri: Some(g), .. } => g,
                    _ => continue,
                };
                if co_re_nhanh(bt) {
                    return;
                }
                if co_doc_ngoai_muon(bt, bi_muon) && con_dung_sau(k, j, ten_muon) {
                    let msg = format!("`{bi_muon}` đang bị mượn khả biến nên chưa đọc được");
                    let chinh = format!("đọc `{bi_muon}` ở đây");
                    let phu = format!("`{ten_muon}` đang mượn nó, và còn được dùng sau chỗ này");
                    self.diags.push(
                        Diagnostic::loi("BR0501", msg)
                            .nhan(Label::chinh(k.cau_lenh[j].span(), chinh))
                            .nhan(Label::phu(*span, phu))
                            .vi_sao(
                                "Trong lúc một tham chiếu khả biến còn sống, nó là lối duy nhất \
                                 tới giá trị ấy. Đọc thẳng biến gốc lúc này là mở lối thứ hai.",
                            ),
                    );
                    return;
                }
            }
        }
    }

    /// E0597 — tham chiếu sống lâu hơn thứ nó trỏ tới.
    ///
    /// Hình dạng hẹp: `let r;` ở khối ngoài, `r = &x` bên trong một khối con
    /// với `x` khai báo trong chính khối con ấy, rồi `r` được dùng sau khi
    /// khối con đóng.
    fn song_lau_hon_thu_no_tro_toi(&mut self, k: &Khoi) {
        for (i, cl) in k.cau_lenh.iter().enumerate() {
            let CauLenh::Let { mau: Mau::Ten { ten: r, .. }, gia_tri: None, .. } = cl else {
                continue;
            };
            for j in (i + 1)..k.cau_lenh.len() {
                let CauLenh::BieuThuc { bt: BieuThuc::Khoi(trong), .. } = &k.cau_lenh[j] else {
                    continue;
                };
                let Some((x, span_gan)) = gan_muon_bien_noi_bo(trong, r) else { continue };
                if con_dung_sau(k, j, r) {
                    let msg = format!("`{x}` không sống đủ lâu cho `{r}`");
                    let chinh = format!("`{r}` trỏ tới `{x}` ở đây");
                    let phu =
                        format!("`{x}` chết khi khối này đóng, nhưng `{r}` còn được dùng sau");
                    let vi_sao = format!(
                        "Một tham chiếu không giữ giá trị sống hộ. Khi khối đóng, `{x}` biến \
                         mất, và cái tham chiếu còn lại trỏ vào chỗ không còn gì."
                    );
                    self.diags.push(
                        Diagnostic::loi("BR0502", msg)
                            .nhan(Label::chinh(span_gan, chinh))
                            .nhan(Label::phu(trong.span, phu))
                            .vi_sao(vi_sao),
                    );
                    return;
                }
            }
        }
    }
}

pub fn kiem_tra(ct: &ChuongTrinh, diags: &mut Diagnostics) {
    let mut bk = BoKiem { diags };
    for muc in &ct.muc {
        if let Muc::Ham(h) = muc {
            bk.khoi(&h.than);
        }
    }
}
