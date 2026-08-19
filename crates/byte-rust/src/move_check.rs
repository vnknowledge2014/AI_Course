//! Kiểm tra chuyển quyền sở hữu — pha **tĩnh**, chạy trước khi thực thi.
//!
//! # Vì sao không thể kiểm lúc chạy
//!
//! Bản đầu của interpreter đánh dấu biến "đã move" ngay lúc thực thi. Cách đó
//! có một lỗ hổng không vá được: **nó chỉ thấy nhánh đã chạy.**
//!
//! ```ignore
//! let a = String::from("x");
//! if false { let b = a; }   // nhánh này không chạy
//! println!("{}", a);        // kiểm-lúc-chạy: không sao. rustc: E0382.
//! ```
//!
//! Kiểm tra động **không bao giờ** khớp được với kiểm tra tĩnh. Và báo `Đạt`
//! cho chương trình mà `rustc` từ chối chính là chế độ hỏng nguy hiểm nhất —
//! đúng thứ đã làm engine giả cũ vô dụng.
//!
//! # Ranh giới cố ý hẹp
//!
//! Pha này **không** cố tái tạo borrow checker (xem ADR-002: NLL định nghĩa
//! trên MIR, không tái tạo trung thực trên AST được). Nó chỉ phân loại:
//!
//! | Tình huống | Kết luận |
//! |---|---|
//! | move rồi dùng lại, cùng block, thẳng hàng, không nhánh | `E0382` — khẳng định được |
//! | move nằm trong `if`/`match`/vòng lặp/closure, và biến còn được dùng sau | `ChuaHoTro` |
//! | còn lại | im lặng |
//!
//! Cả hai luật đều quyết định được **không cần CFG**. Khi không chắc, ta trả
//! `ChuaHoTro` — một lời thú nhận, không bao giờ là một phán quyết sai.

use crate::ast::*;
use crate::diag::{Diagnostic, Diagnostics, Label};
use crate::span::Span;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TrangThai {
    /// Còn giữ giá trị.
    Song,
    /// Đã move trên chuỗi thẳng hàng — dùng lại là lỗi chắc chắn.
    DaMoveThang(Span),
    /// Đã move bên trong một nhánh/vòng lặp — ta không đủ cơ sở phán quyết.
    DaMoveTrongNhanh(Span),
}

/// Biến này có thể là kiểu không-`Copy` không?
///
/// Không có suy luận kiểu, nên đây là ước lượng theo cú pháp. Nguyên tắc: khi
/// không chắc thì coi là **không-`Copy`** — vì đoán sai theo hướng đó chỉ dẫn
/// tới `ChuaHoTro` (vô hại), còn đoán sai theo hướng kia dẫn tới false accept.
fn co_the_khong_copy(kh: Option<&BieuThuc>) -> bool {
    match kh {
        None => false,
        Some(bt) => match bt {
            // Hằng số vô hướng luôn là Copy.
            BieuThuc::HangSo { gia_tri, .. } => !matches!(
                gia_tri,
                HangSo::SoNguyen(_) | HangSo::SoThuc(_) | HangSo::DungSai(_) | HangSo::KyTu(_)
            ),
            // Phép toán trên số cho ra số.
            BieuThuc::HaiNgoi { trai, phai, .. } => {
                co_the_khong_copy(Some(trai)) || co_the_khong_copy(Some(phai))
            }
            BieuThuc::MotNgoi { toan_hang, .. } => co_the_khong_copy(Some(toan_hang)),
            // Mượn là Copy (với `&T`); `&mut T` thì không, nhưng nó cũng không
            // bị "move" theo nghĩa ta đang xét.
            BieuThuc::Muon { .. } => false,
            BieuThuc::Dai { .. } => false,
            BieuThuc::Ep { .. } => false,
            // Còn lại: chuỗi, Vec, struct, enum, kết quả gọi hàm… coi là không-Copy.
            _ => true,
        },
    }
}

struct BoKiem<'a> {
    trang_thai: Vec<HashMap<String, TrangThai>>,
    do_sau_nhanh: usize,
    /// Chiều cao ngăn xếp phạm vi tại mỗi lần vào một vòng lặp.
    ///
    /// Dùng để phân biệt "biến khai báo BÊN TRONG vòng lặp" (move thoải mái,
    /// mỗi vòng một giá trị mới) với "biến khai báo BÊN NGOÀI" (move là hỏng,
    /// vì vòng thứ hai sẽ dùng lại giá trị đã chuyển đi).
    moc_vong_lap: Vec<usize>,
    diags: &'a mut Diagnostics,
    /// Đã báo cho biến nào rồi, để không lặp lại cùng một chẩn đoán.
    da_bao: Vec<String>,
}

impl<'a> BoKiem<'a> {
    fn moi(diags: &'a mut Diagnostics) -> Self {
        Self {
            trang_thai: vec![HashMap::new()],
            do_sau_nhanh: 0,
            moc_vong_lap: Vec::new(),
            diags,
            da_bao: Vec::new(),
        }
    }

    fn vao(&mut self) {
        self.trang_thai.push(HashMap::new());
    }

    fn ra(&mut self) {
        self.trang_thai.pop();
    }

    fn khai_bao(&mut self, ten: &str) {
        self.trang_thai.last_mut().unwrap().insert(ten.to_string(), TrangThai::Song);
    }

    fn tra(&self, ten: &str) -> Option<TrangThai> {
        self.trang_thai.iter().rev().find_map(|p| p.get(ten).copied())
    }

    /// Chỉ số phạm vi chứa `ten`, đếm từ ngoài vào.
    fn vi_tri_pham(&self, ten: &str) -> Option<usize> {
        self.trang_thai.iter().rposition(|p| p.contains_key(ten))
    }

    /// Biến này được khai báo BÊN NGOÀI vòng lặp trong cùng đang chạy?
    fn ngoai_vong_lap(&self, ten: &str) -> bool {
        match (self.moc_vong_lap.last(), self.vi_tri_pham(ten)) {
            (Some(&moc), Some(vt)) => vt < moc,
            _ => false,
        }
    }

    fn dat(&mut self, ten: &str, tt: TrangThai) {
        for p in self.trang_thai.iter_mut().rev() {
            if p.contains_key(ten) {
                p.insert(ten.to_string(), tt);
                return;
            }
        }
    }

    /// Tên biến gốc của một đường dẫn nơi chốn.
    ///
    /// `p` -> `p` · `p.ten` -> `p` · `t.0` -> `t` · `v[0]` -> `v`
    ///
    /// Bản trước chỉ nhận tên trần, nên mọi phép move qua đường dẫn thoát im
    /// lặng ngay dòng đầu — kể cả khi nhánh duyệt phía trên đã làm đúng phần
    /// của nó. Phải vá ở gốc, vá từng nhánh là chưa đủ.
    fn goc_cua(bt: &BieuThuc) -> Option<(String, Span, bool)> {
        match bt {
            BieuThuc::DuongDan { doan, span } if doan.len() == 1 => {
                Some((doan[0].clone(), *span, false))
            }
            BieuThuc::TruyCapTruong { doi_tuong, .. } | BieuThuc::ChiSo { doi_tuong, .. } => {
                Self::goc_cua(doi_tuong).map(|(t, s, _)| (t, s, true))
            }
            _ => None,
        }
    }

    /// Ghi nhận một phép move khỏi `bt`.
    fn ghi_move(&mut self, bt: &BieuThuc) {
        let Some((ten, span, mot_phan)) = Self::goc_cua(bt) else { return };
        let span = bt.span().merge(span);
        let _ = mot_phan;
        let ten = &ten;
        if self.tra(ten) != Some(TrangThai::Song) {
            return;
        }
        // Move một biến NGOÀI vào trong vòng lặp là hỏng ngay, không cần đợi
        // xem nó có được dùng lại hay không: vòng thứ hai sẽ dùng giá trị đã
        // chuyển đi. `rustc` báo E0382 "value moved here, in previous iteration".
        // Ta không chắc vòng lặp có chạy quá một lần hay không (cần CFG), nên
        // trả `ChuaHoTro` thay vì khẳng định.
        if self.ngoai_vong_lap(ten) {
            let ten = ten.clone();
            self.bao_chua_ho_tro_vong_lap(&ten, span);
            self.dat(&ten, TrangThai::DaMoveTrongNhanh(span));
            return;
        }

        let tt = if self.do_sau_nhanh > 0 {
            TrangThai::DaMoveTrongNhanh(span)
        } else {
            TrangThai::DaMoveThang(span)
        };
        self.dat(ten, tt);
    }

    fn bao_chua_ho_tro_vong_lap(&mut self, ten: &str, span: Span) {
        if self.da_bao.iter().any(|x| x == ten) {
            return;
        }
        self.da_bao.push(ten.to_string());
        self.diags.push(
            Diagnostic::chua_ho_tro(
                "BR0532",
                "borrow-check-cfg",
                format!("Byte chưa kiểm được việc chuyển `{ten}` ra khỏi vòng lặp"),
            )
            .nhan(Label::chinh(span, "quyền sở hữu bị chuyển đi ở đây, bên trong vòng lặp"))
            .vi_sao("`{ten}` được khai báo bên ngoài vòng lặp, nên nếu vòng lặp chạy lần thứ hai, nó sẽ dùng lại một giá trị đã bị chuyển đi. `rustc` từ chối kiểu này. Xác định vòng lặp có chạy quá một lần hay không cần phân tích luồng điều khiển — thứ Byte cố tình không làm.")
            .sua(format!("nhân bản bên trong vòng lặp: `{ten}.clone()`"))
            .sua(format!("hoặc chỉ mượn: `&{ten}`"))
            .sua("Hoặc mở bản Desktop và bấm “Đối chiếu với cargo” để có câu trả lời chính xác")
            .khai_niem("quyền sở hữu"),
        );
    }

    /// Ghi nhận một lần **đọc** biến; báo lỗi nếu nó đã bị move.
    fn ghi_doc(&mut self, ten: &str, span: Span) {
        match self.tra(ten) {
            Some(TrangThai::DaMoveThang(noi_move)) => {
                if self.da_bao.iter().any(|x| x == ten) {
                    return;
                }
                self.da_bao.push(ten.to_string());
                self.diags.push(
                    Diagnostic::loi("BR0530", format!("`{ten}` đã bị chuyển quyền sở hữu đi nơi khác"))
                        .nhan(Label::chinh(span, "dùng lại ở đây thì không còn giá trị nữa"))
                        .nhan(Label::phu(noi_move, "quyền sở hữu bị chuyển đi tại đây"))
                        .vi_sao("Rust cho mỗi giá trị đúng MỘT chủ sở hữu. Khi bạn gán nó sang biến khác hoặc truyền vào hàm, chủ cũ mất quyền — nhờ luật này Rust không cần bộ dọn rác mà vẫn không bao giờ dùng nhầm bộ nhớ đã giải phóng.")
                        .sua(format!("nếu muốn giữ cả hai, hãy nhân bản: `{ten}.clone()`"))
                        .sua(format!("hoặc chỉ mượn thay vì lấy hẳn: `&{ten}`"))
                        .khai_niem("quyền sở hữu"),
                );
            }
            Some(TrangThai::DaMoveTrongNhanh(noi_move)) => {
                if self.da_bao.iter().any(|x| x == ten) {
                    return;
                }
                self.da_bao.push(ten.to_string());
                self.diags.push(
                    Diagnostic::chua_ho_tro(
                        "BR0531",
                        "borrow-check-cfg",
                        format!("Byte chưa kiểm được luật mượn cho `{ten}` trong tình huống này"),
                    )
                    .nhan(Label::chinh(span, "biến này được dùng ở đây"))
                    .nhan(Label::phu(noi_move, "và bị chuyển đi bên trong một nhánh hoặc vòng lặp"))
                    .vi_sao("Khi phép chuyển quyền sở hữu nằm trong `if`, `match` hay vòng lặp, việc xác định nó có thực sự xảy ra hay không cần phân tích toàn bộ luồng điều khiển. Byte cố tình không làm việc đó, vì làm nửa vời sẽ từ chối cả những chương trình mà `rustc` cho phép — tức dạy bạn một luật không tồn tại.")
                    .sua("Mở bản Desktop và bấm “Đối chiếu với cargo” để có câu trả lời chính xác từ compiler thật")
                    .sua("Hoặc viết lại sao cho phép chuyển nằm thẳng hàng, không nằm trong nhánh")
                    .khai_niem("quyền sở hữu"),
                );
            }
            _ => {}
        }
    }

    // ── Duyệt cây ───────────────────────────────────────────────────────────

    fn khoi(&mut self, k: &Khoi) {
        self.vao();
        for cl in &k.cau_lenh {
            self.cau_lenh(cl);
        }
        if let Some(bt) = &k.gia_tri_cuoi {
            self.bieu_thuc(bt);
            self.ghi_move(bt);
        }
        self.ra();
    }

    fn cau_lenh(&mut self, cl: &CauLenh) {
        match cl {
            CauLenh::Let { mau, gia_tri, .. } => {
                if let Some(bt) = gia_tri {
                    self.bieu_thuc(bt);
                    self.ghi_move(bt);
                }
                let mut ten = Vec::new();
                mau.ten_rang_buoc(&mut ten);
                let khong_copy = co_the_khong_copy(gia_tri.as_ref());
                for (t, _, _) in ten {
                    if khong_copy {
                        self.khai_bao(&t);
                    } else {
                        // Kiểu Copy: không bao giờ bị move, khỏi theo dõi.
                        self.trang_thai.last_mut().unwrap().remove(&t);
                    }
                }
            }
            CauLenh::BieuThuc { bt, .. } => self.bieu_thuc(bt),
            CauLenh::Muc(_) => {}
        }
    }

    /// Duyệt một biểu thức trong ngữ cảnh **đọc**.
    fn bieu_thuc(&mut self, bt: &BieuThuc) {
        match bt {
            BieuThuc::DuongDan { doan, span } => {
                if doan.len() == 1 {
                    self.ghi_doc(&doan[0], *span);
                }
            }
            BieuThuc::HangSo { .. } | BieuThuc::TiepTuc { .. } => {}

            // `&x` là mượn, không phải move — nhưng vẫn là một lần đọc.
            BieuThuc::Muon { gia_tri, .. } => self.bieu_thuc(gia_tri),
            // `*x` LẤY HẲN giá trị ra. Gộp nó với `&x` đang dạy NGƯỢC đúng cái
            // luật quan trọng nhất: "không lấy được đồ ra khỏi thứ mình chỉ mượn".
            BieuThuc::GiaiTham { gia_tri, span } => {
                let _ = span;
                self.bieu_thuc(gia_tri);
                // `*x` lấy hẳn giá trị; nếu `x` là `&T` thì đó là E0507.
                // Chưa theo dõi được kiểu ở tầng này nên để `tyck` lo.
            }

            BieuThuc::GoiHam { ham, doi_so, .. } => {
                self.bieu_thuc(ham);
                for a in doi_so {
                    self.bieu_thuc(a);
                    self.ghi_move(a);
                }
            }
            BieuThuc::GoiPhuongThuc { doi_tuong, ten, doi_so, .. } => {
                self.bieu_thuc(doi_tuong);
                // `.clone()`/`.len()`… mượn `self`, không lấy hẳn. Ta không phân
                // biệt được nên coi là mượn — hướng an toàn, tránh báo lỗi oan.
                let _ = ten;
                for a in doi_so {
                    self.bieu_thuc(a);
                    self.ghi_move(a);
                }
            }
            BieuThuc::KhoiTaoStruct { truong, con_lai, .. } => {
                for (_, e) in truong {
                    self.bieu_thuc(e);
                    self.ghi_move(e);
                }
                if let Some(c) = con_lai {
                    self.bieu_thuc(c);
                }
            }
            BieuThuc::Gan { dich, toan_tu, gia_tri, .. } => {
                self.bieu_thuc(gia_tri);
                // `x = y` chuyển quyền sở hữu của `y`.
                // `x += y` thì KHÔNG: các trait `*Assign` nhận vế phải qua
                // tham chiếu, nên `y` vẫn còn nguyên chủ.
                if toan_tu.is_none() {
                    self.ghi_move(gia_tri);
                }
                // Gán đè hẳn thì biến sống lại; gán cộng dồn thì không đổi
                // trạng thái (nó đang đọc chính giá trị cũ).
                if let BieuThuc::DuongDan { doan, .. } = &**dich {
                    if doan.len() == 1 {
                        if toan_tu.is_none() && self.tra(&doan[0]).is_some() {
                            self.dat(&doan[0], TrangThai::Song);
                        } else if toan_tu.is_some() {
                            self.ghi_doc(&doan[0], dich.span());
                        }
                    }
                }
            }

            BieuThuc::MotNgoi { toan_hang, .. } => self.bieu_thuc(toan_hang),
            // `let b = a + "!";` với `a: String` nuốt `a` — `Add` cho String
            // nhận `self` chứ không nhận `&self`.
            BieuThuc::HaiNgoi { toan_tu, trai, phai, .. } => {
                self.bieu_thuc(trai);
                self.bieu_thuc(phai);
                if matches!(toan_tu, ToanTuHai::Cong) {
                    self.ghi_move(trai);
                }
            }
            BieuThuc::TruyCapTruong { doi_tuong, .. } => self.bieu_thuc(doi_tuong),
            BieuThuc::ChiSo { doi_tuong, chi_so, .. } => {
                self.bieu_thuc(doi_tuong);
                self.bieu_thuc(chi_so);
            }
            BieuThuc::Ep { gia_tri, .. } | BieuThuc::LanTruyenLoi { gia_tri, .. } => {
                self.bieu_thuc(gia_tri)
            }
            // Tuple LẤY quyền sở hữu của từng phần tử: `let t = (s, 1);` nuốt `s`.
            BieuThuc::Tuple { phan_tu, .. } => {
                for e in phan_tu {
                    self.bieu_thuc(e);
                    self.ghi_move(e);
                }
            }
            // Macro phải tách theo tên: `vec![s]` nuốt `s`, còn `println!("{}", s)`
            // thì chỉ MƯỢN. Gộp chung sẽ báo oan mọi lệnh in.
            BieuThuc::Macro { ten, doi_so, .. } => {
                let nuot = matches!(ten.as_str(), "vec");
                for e in doi_so {
                    self.bieu_thuc(e);
                    if nuot {
                        self.ghi_move(e);
                    }
                }
            }
            BieuThuc::Mang { phan_tu, lap_lai, .. } => {
                for e in phan_tu {
                    self.bieu_thuc(e);
                    self.ghi_move(e);
                }
                if let Some(n) = lap_lai {
                    self.bieu_thuc(n);
                }
            }
            BieuThuc::Dai { tu, den, .. } => {
                if let Some(e) = tu {
                    self.bieu_thuc(e);
                }
                if let Some(e) = den {
                    self.bieu_thuc(e);
                }
            }
            BieuThuc::TraVe { gia_tri, .. } | BieuThuc::Thoat { gia_tri, .. } => {
                if let Some(e) = gia_tri {
                    self.bieu_thuc(e);
                    self.ghi_move(e);
                }
            }
            BieuThuc::Khoi(k) => self.khoi(k),

            // ── Các cấu trúc TẠO RA nhánh ───────────────────────────────────
            BieuThuc::Neu { dieu_kien, than, nguoc_lai, .. } => {
                self.bieu_thuc(dieu_kien);
                self.do_sau_nhanh += 1;
                self.khoi(than);
                if let Some(nl) = nguoc_lai {
                    self.bieu_thuc(nl);
                }
                self.do_sau_nhanh -= 1;
            }
            BieuThuc::KhopMau { gia_tri, nhanh, .. } => {
                self.bieu_thuc(gia_tri);
                // Đối tượng `match` được đánh giá THẲNG HÀNG trước khi rẽ nhánh,
                // nên đây là ca luật 1 của ADR-002 §2 — không cần CFG.
                // `match x { Some(s) => … }` nuốt `x` nếu mẫu ràng buộc theo giá trị.
                let rang_buoc_gia_tri = nhanh.iter().any(|n| {
                    let mut ten = Vec::new();
                    n.mau.ten_rang_buoc(&mut ten);
                    !ten.is_empty() && !matches!(n.mau, Mau::Ten { la_ref: true, .. })
                });
                if rang_buoc_gia_tri {
                    self.ghi_move(gia_tri);
                }
                self.do_sau_nhanh += 1;
                for n in nhanh {
                    self.vao();
                    let mut ten = Vec::new();
                    n.mau.ten_rang_buoc(&mut ten);
                    for (t, _, _) in ten {
                        self.khai_bao(&t);
                    }
                    if let Some(dk) = &n.dieu_kien {
                        self.bieu_thuc(dk);
                    }
                    self.bieu_thuc(&n.than);
                    self.ra();
                }
                self.do_sau_nhanh -= 1;
            }
            BieuThuc::Lap { than, .. } => {
                self.do_sau_nhanh += 1;
                self.moc_vong_lap.push(self.trang_thai.len());
                self.khoi(than);
                self.moc_vong_lap.pop();
                self.do_sau_nhanh -= 1;
            }
            BieuThuc::Trong { dieu_kien, than, .. } => {
                self.bieu_thuc(dieu_kien);
                self.do_sau_nhanh += 1;
                self.moc_vong_lap.push(self.trang_thai.len());
                self.khoi(than);
                self.moc_vong_lap.pop();
                self.do_sau_nhanh -= 1;
            }
            BieuThuc::Cho { mau, day, than, .. } => {
                self.bieu_thuc(day);
                // `for x in v` gọi `into_iter(self)` — NUỐT `v`. Đây là lỗi kinh
                // điển bậc nhất của người mới, và tick xanh ở đúng chỗ này gây
                // hại hơn bất cứ đâu: người học kết luận `for` không lấy quyền
                // sở hữu, rồi mang niềm tin đó sang cargo thật.
                //
                // Ngoại lệ: `&v`, `v.iter()`, và dải `0..n` đều chỉ mượn.
                let chi_muon = matches!(&**day, BieuThuc::Muon { .. } | BieuThuc::Dai { .. })
                    || matches!(&**day, BieuThuc::GoiPhuongThuc { ten, .. }
                                if matches!(ten.as_str(), "iter" | "iter_mut" | "chars"));
                if !chi_muon {
                    self.ghi_move(day);
                }
                self.do_sau_nhanh += 1;
                // Mốc phải lấy TRƯỚC `vao()`: phạm vi mới sẽ nằm ở đúng chỉ số
                // này, nên biến lặp `i` được tính là bên TRONG vòng lặp.
                self.moc_vong_lap.push(self.trang_thai.len());
                self.vao();
                let mut ten = Vec::new();
                mau.ten_rang_buoc(&mut ten);
                for (t, _, _) in ten {
                    self.khai_bao(&t);
                }
                self.khoi(than);
                self.moc_vong_lap.pop();
                self.ra();
                self.do_sau_nhanh -= 1;
            }
            BieuThuc::BeQuan { tham_so, than, .. } => {
                // Closure có thể chạy 0 lần, 1 lần, hay nhiều lần — đúng như nhánh.
                self.do_sau_nhanh += 1;
                self.vao();
                for (m, _) in tham_so {
                    let mut ten = Vec::new();
                    m.ten_rang_buoc(&mut ten);
                    for (t, _, _) in ten {
                        self.khai_bao(&t);
                    }
                }
                self.bieu_thuc(than);
                self.ra();
                self.do_sau_nhanh -= 1;
            }
        }
    }
}

/// Kiểm tra toàn bộ chương trình.
pub fn kiem_tra(ct: &ChuongTrinh, diags: &mut Diagnostics) {
    for m in &ct.muc {
        let ham = match m {
            Muc::Ham(h) => vec![h],
            Muc::Impl(i) => i.ham.iter().collect(),
            _ => continue,
        };
        for h in ham {
            let mut bk = BoKiem::moi(diags);
            bk.vao();
            for ts in &h.tham_so {
                let mut ten = Vec::new();
                ts.mau.ten_rang_buoc(&mut ten);
                for (t, _, _) in ten {
                    bk.khai_bao(&t);
                }
            }
            bk.khoi(&h.than);
            bk.ra();
        }
    }
}
