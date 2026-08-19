//! Kiểm tính khả biến — pha tĩnh.
//!
//! # Vì sao cần một pha riêng
//!
//! Thông tin đã được thu thập từ lâu rồi bị vứt đi: `Mau::ten_rang_buoc` trả về
//! `(tên, co_the_sua, span)`, nhưng không tầng nào đọc `co_the_sua`. Hệ quả là
//! `let a = 10; a = 99;` chạy ngon lành, dù `rustc` báo `E0384`.
//!
//! Đây không phải chi tiết vụn. **`mut` là mặc-định-ngược của Rust** so với mọi
//! ngôn ngữ người học từng biết: ở Python, JavaScript, Java, biến gán lại được
//! là chuyện hiển nhiên. Nếu Byte im lặng, người học sẽ mang thói quen cũ sang
//! `cargo` thật rồi bị chặn mà không hiểu vì sao.
//!
//! # Ba luật, đều quyết định được không cần CFG
//!
//! | Luật | Mã `rustc` |
//! |---|---|
//! | Gán lại một binding không `mut` | `E0384` |
//! | Mượn `&mut` một binding không `mut` | `E0596` |
//! | Gán qua một tham chiếu `&T` (không phải `&mut T`) | `E0594` |

use crate::ast::*;
use crate::diag::{Diagnostic, Diagnostics, Label};
use crate::span::Span;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct ThongTinBien {
    co_the_sua: bool,
    /// Đã được gán giá trị lần đầu chưa. `let x; x = 1;` là hợp lệ với binding
    /// bất biến — Rust gọi đó là "khởi tạo trễ", không phải "gán lại".
    da_khoi_tao: bool,
    span_khai_bao: Span,
    /// Binding này là tham chiếu khả biến (`&mut T`)?
    la_tham_chieu_kha_bien: bool,
    /// Binding này là tham chiếu bất biến (`&T`)?
    la_tham_chieu_bat_bien: bool,
}

struct BoKiem<'a> {
    pham: Vec<HashMap<String, ThongTinBien>>,
    diags: &'a mut Diagnostics,
    da_bao: Vec<String>,
}

impl<'a> BoKiem<'a> {
    fn moi(diags: &'a mut Diagnostics) -> Self {
        Self { pham: vec![HashMap::new()], diags, da_bao: Vec::new() }
    }

    fn vao(&mut self) { self.pham.push(HashMap::new()); }
    fn ra(&mut self) { self.pham.pop(); }

    fn khai_bao(&mut self, ten: &str, tt: ThongTinBien) {
        self.pham.last_mut().unwrap().insert(ten.to_string(), tt);
    }

    fn tra(&self, ten: &str) -> Option<ThongTinBien> {
        self.pham.iter().rev().find_map(|p| p.get(ten).copied())
    }

    fn danh_dau_da_khoi_tao(&mut self, ten: &str) {
        for p in self.pham.iter_mut().rev() {
            if let Some(tt) = p.get_mut(ten) {
                tt.da_khoi_tao = true;
                return;
            }
        }
    }

    fn mot_lan(&mut self, khoa: &str) -> bool {
        if self.da_bao.iter().any(|x| x == khoa) {
            return false;
        }
        self.da_bao.push(khoa.to_string());
        true
    }

    // ── Ba luật ─────────────────────────────────────────────────────────────

    fn kiem_gan(&mut self, dich: &BieuThuc) {
        match dich {
            BieuThuc::DuongDan { doan, span } if doan.len() == 1 => {
                let ten = &doan[0];
                let Some(tt) = self.tra(ten) else { return };
                if tt.co_the_sua {
                    return;
                }
                if !tt.da_khoi_tao {
                    // Khởi tạo trễ — hợp lệ.
                    self.danh_dau_da_khoi_tao(ten);
                    return;
                }
                if !self.mot_lan(&format!("gan:{ten}")) {
                    return;
                }
                self.diags.push(
                    Diagnostic::loi("BR0400", format!("không gán lại được cho `{ten}`"))
                        .nhan(Label::chinh(*span, "gán lại ở đây"))
                        .nhan(Label::phu(tt.span_khai_bao, format!("`{ten}` khai báo bất biến ở đây")))
                        .vi_sao("Trong Rust, biến **mặc định là bất biến** — ngược với Python, JavaScript hay Java. Đây là lựa chọn có chủ đích: phần lớn biến không cần thay đổi, và nói rõ cái nào có thay đổi giúp người đọc code biết ngay chỗ nào cần để mắt.")
                        .sua(format!("nếu thật sự cần thay đổi, khai báo `let mut {ten} = ...`"))
                        .sua("nếu chỉ cần một giá trị mới, đặt tên khác — hoặc dùng shadowing: `let ten = ...;` một lần nữa")
                        .khai_niem("khả biến"),
                );
            }
            BieuThuc::GiaiTham { gia_tri, span } => {
                // Gán qua tham chiếu: phải là `&mut`.
                if let BieuThuc::DuongDan { doan, .. } = &**gia_tri {
                    if doan.len() == 1 {
                        if let Some(tt) = self.tra(&doan[0]) {
                            if tt.la_tham_chieu_bat_bien && self.mot_lan(&format!("ghi_qua:{}", doan[0])) {
                                self.diags.push(
                                    Diagnostic::loi("BR0401", format!("không sửa được qua `{}`", doan[0]))
                                        .nhan(Label::chinh(*span, "ghi qua tham chiếu ở đây"))
                                        .nhan(Label::phu(tt.span_khai_bao, "đây là `&T`, chỉ đọc"))
                                        .vi_sao("`&T` cho quyền ĐỌC, `&mut T` mới cho quyền GHI. Rust tách hai quyền này để bảo đảm: khi bạn đang giữ một `&mut`, không ai khác đọc hay ghi được cùng lúc — đó là cách Rust chặn data race mà không cần khoá.")
                                        .sua("đổi tham số thành `&mut T`, và bên gọi truyền `&mut x`")
                                        .khai_niem("mượn"),
                                );
                            }
                        }
                    }
                }
            }
            BieuThuc::TruyCapTruong { doi_tuong, .. } | BieuThuc::ChiSo { doi_tuong, .. } => {
                // Sửa một trường/phần tử cũng cần binding gốc là `mut`.
                self.kiem_gan(doi_tuong);
            }
            _ => {}
        }
    }

    fn kiem_muon_kha_bien(&mut self, gia_tri: &BieuThuc, span: Span) {
        let BieuThuc::DuongDan { doan, .. } = gia_tri else { return };
        if doan.len() != 1 {
            return;
        }
        let ten = &doan[0];
        let Some(tt) = self.tra(ten) else { return };
        if tt.co_the_sua || !self.mot_lan(&format!("muon_mut:{ten}")) {
            return;
        }
        self.diags.push(
            Diagnostic::loi("BR0402", format!("không mượn `&mut` từ `{ten}` được"))
                .nhan(Label::chinh(span, "mượn khả biến ở đây"))
                .nhan(Label::phu(tt.span_khai_bao, format!("`{ten}` khai báo bất biến ở đây")))
                .vi_sao("Muốn cho ai đó quyền sửa một giá trị, trước hết chính bạn phải có quyền đó. `&mut x` chỉ hợp lệ khi `x` được khai báo `let mut x`.")
                .sua(format!("đổi thành `let mut {ten} = ...`"))
                .khai_niem("mượn"),
        );
    }

    // ── Duyệt cây ───────────────────────────────────────────────────────────

    fn khai_bao_tu_mau(&mut self, mau: &Mau, kieu: Option<&Kieu>) {
        let mut ten = Vec::new();
        mau.ten_rang_buoc(&mut ten);
        let (la_tc_kb, la_tc_bb) = match kieu {
            Some(Kieu::ThamChieu { co_the_sua, .. }) => (*co_the_sua, !*co_the_sua),
            _ => (false, false),
        };
        for (n, co_the_sua, sp) in ten {
            self.khai_bao(&n, ThongTinBien {
                co_the_sua,
                da_khoi_tao: true,
                span_khai_bao: sp,
                la_tham_chieu_kha_bien: la_tc_kb,
                la_tham_chieu_bat_bien: la_tc_bb,
            });
        }
    }

    fn khoi(&mut self, k: &Khoi) {
        self.vao();
        for cl in &k.cau_lenh {
            match cl {
                CauLenh::Let { mau, kieu, gia_tri, .. } => {
                    if let Some(e) = gia_tri {
                        self.bieu_thuc(e);
                    }
                    let mut ten = Vec::new();
                    mau.ten_rang_buoc(&mut ten);
                    let (kb, bb) = match (kieu, gia_tri) {
                        (Some(Kieu::ThamChieu { co_the_sua, .. }), _) => (*co_the_sua, !*co_the_sua),
                        (_, Some(BieuThuc::Muon { co_the_sua, .. })) => (*co_the_sua, !*co_the_sua),
                        _ => (false, false),
                    };
                    for (n, co_the_sua, sp) in ten {
                        self.khai_bao(&n, ThongTinBien {
                            co_the_sua,
                            da_khoi_tao: gia_tri.is_some(),
                            span_khai_bao: sp,
                            la_tham_chieu_kha_bien: kb,
                            la_tham_chieu_bat_bien: bb,
                        });
                    }
                }
                CauLenh::BieuThuc { bt, .. } => self.bieu_thuc(bt),
                CauLenh::Muc(_) => {}
            }
        }
        if let Some(e) = &k.gia_tri_cuoi {
            self.bieu_thuc(e);
        }
        self.ra();
    }

    fn bieu_thuc(&mut self, bt: &BieuThuc) {
        match bt {
            BieuThuc::Gan { dich, gia_tri, .. } => {
                self.bieu_thuc(gia_tri);
                self.kiem_gan(dich);
            }
            BieuThuc::Muon { co_the_sua: true, gia_tri, span } => {
                self.kiem_muon_kha_bien(gia_tri, *span);
                self.bieu_thuc(gia_tri);
            }
            BieuThuc::Muon { gia_tri, .. } | BieuThuc::GiaiTham { gia_tri, .. }
            | BieuThuc::Ep { gia_tri, .. } | BieuThuc::LanTruyenLoi { gia_tri, .. } => {
                self.bieu_thuc(gia_tri)
            }
            BieuThuc::MotNgoi { toan_hang, .. } => self.bieu_thuc(toan_hang),
            BieuThuc::HaiNgoi { trai, phai, .. } => {
                self.bieu_thuc(trai);
                self.bieu_thuc(phai);
            }
            BieuThuc::GoiHam { ham, doi_so, .. } => {
                self.bieu_thuc(ham);
                for a in doi_so { self.bieu_thuc(a); }
            }
            BieuThuc::GoiPhuongThuc { doi_tuong, ten, doi_so, span } => {
                // Phương thức làm thay đổi đối tượng cần receiver là `mut`.
                if matches!(ten.as_str(), "push" | "pop" | "sort" | "reverse" | "insert"
                                          | "remove" | "clear" | "push_str" | "truncate") {
                    if let BieuThuc::DuongDan { doan, .. } = &**doi_tuong {
                        if doan.len() == 1 {
                            if let Some(tt) = self.tra(&doan[0]) {
                                if !tt.co_the_sua && !tt.la_tham_chieu_kha_bien
                                    && self.mot_lan(&format!("pt_mut:{}", doan[0]))
                                {
                                    let ten_bien = doan[0].clone();
                                    self.diags.push(
                                        Diagnostic::loi("BR0403", format!("`{ten}` làm thay đổi `{ten_bien}`, nên `{ten_bien}` phải là `mut`"))
                                            .nhan(Label::chinh(*span, format!("`{ten}` cần quyền sửa")))
                                            .nhan(Label::phu(tt.span_khai_bao, format!("`{ten_bien}` khai báo bất biến ở đây")))
                                            .vi_sao("Những phương thức làm thay đổi giá trị nhận `&mut self`. Rust bắt bạn nói trước rằng biến này sẽ thay đổi, để người đọc code biết ngay.")
                                            .sua(format!("đổi thành `let mut {ten_bien} = ...`"))
                                            .khai_niem("khả biến"),
                                    );
                                }
                            }
                        }
                    }
                }
                self.bieu_thuc(doi_tuong);
                for a in doi_so { self.bieu_thuc(a); }
            }
            BieuThuc::TruyCapTruong { doi_tuong, .. } => self.bieu_thuc(doi_tuong),
            BieuThuc::ChiSo { doi_tuong, chi_so, .. } => {
                self.bieu_thuc(doi_tuong);
                self.bieu_thuc(chi_so);
            }
            BieuThuc::Tuple { phan_tu, .. } | BieuThuc::Macro { doi_so: phan_tu, .. } => {
                for e in phan_tu { self.bieu_thuc(e); }
            }
            BieuThuc::Mang { phan_tu, lap_lai, .. } => {
                for e in phan_tu { self.bieu_thuc(e); }
                if let Some(n) = lap_lai { self.bieu_thuc(n); }
            }
            BieuThuc::KhoiTaoStruct { truong, con_lai, .. } => {
                for (_, e) in truong { self.bieu_thuc(e); }
                if let Some(c) = con_lai { self.bieu_thuc(c); }
            }
            BieuThuc::Dai { tu, den, .. } => {
                if let Some(e) = tu { self.bieu_thuc(e); }
                if let Some(e) = den { self.bieu_thuc(e); }
            }
            BieuThuc::TraVe { gia_tri, .. } | BieuThuc::Thoat { gia_tri, .. } => {
                if let Some(e) = gia_tri { self.bieu_thuc(e); }
            }
            BieuThuc::Khoi(k) => self.khoi(k),
            BieuThuc::Neu { dieu_kien, than, nguoc_lai, .. } => {
                self.bieu_thuc(dieu_kien);
                self.khoi(than);
                if let Some(nl) = nguoc_lai { self.bieu_thuc(nl); }
            }
            BieuThuc::KhopMau { gia_tri, nhanh, .. } => {
                self.bieu_thuc(gia_tri);
                for n in nhanh {
                    self.vao();
                    self.khai_bao_tu_mau(&n.mau, None);
                    if let Some(dk) = &n.dieu_kien { self.bieu_thuc(dk); }
                    self.bieu_thuc(&n.than);
                    self.ra();
                }
            }
            BieuThuc::Lap { than, .. } => self.khoi(than),
            BieuThuc::Trong { dieu_kien, than, .. } => {
                self.bieu_thuc(dieu_kien);
                self.khoi(than);
            }
            BieuThuc::Cho { mau, day, than, .. } => {
                self.bieu_thuc(day);
                self.vao();
                self.khai_bao_tu_mau(mau, None);
                self.khoi(than);
                self.ra();
            }
            BieuThuc::BeQuan { tham_so, than, .. } => {
                self.vao();
                for (m, k) in tham_so {
                    self.khai_bao_tu_mau(m, k.as_ref());
                }
                self.bieu_thuc(than);
                self.ra();
            }
            BieuThuc::HangSo { .. } | BieuThuc::DuongDan { .. } | BieuThuc::TiepTuc { .. } => {}
        }
    }
}

pub fn kiem_tra(ct: &ChuongTrinh, diags: &mut Diagnostics) {
    for m in &ct.muc {
        let cac: Vec<&Ham> = match m {
            Muc::Ham(h) => vec![h],
            Muc::Impl(i) => i.ham.iter().collect(),
            _ => continue,
        };
        for h in cac {
            let mut bk = BoKiem::moi(diags);
            bk.vao();
            for ts in &h.tham_so {
                bk.khai_bao_tu_mau(&ts.mau, Some(&ts.kieu));
            }
            bk.khoi(&h.than);
            bk.ra();
        }
    }
}
