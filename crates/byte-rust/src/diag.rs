//! Chẩn đoán lỗi bằng tiếng Việt, viết cho người mới học.
//!
//! Nguyên tắc: thông báo của `rustc` viết cho lập trình viên đã biết Rust. Người
//! mới học đọc "cannot borrow `x` as mutable more than once at a time" thì không
//! hiểu gì. Ở đây mỗi lỗi gồm ba tầng:
//!
//! 1. **Chuyện gì xảy ra** — một câu, không thuật ngữ.
//! 2. **Vì sao Rust không cho** — lý do, gắn với khái niệm đang học.
//! 3. **Sửa thế nào** — gợi ý cụ thể, kèm code nếu được.
//!
//! Mỗi lỗi có một mã ổn định (`BR####`) để bài học liên kết tới trang giải thích.

use crate::span::{SourceMap, Span};
use core::fmt::Write as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Mã của người học sai. Chặn việc chạy.
    Loi,
    /// Chạy được nhưng gần như chắc chắn không phải ý người viết.
    CanhBao,
    /// **Byte không đủ sức kiểm tra chỗ này.**
    ///
    /// Không phải lỗi của người học, nên không được hiển thị như lỗi và không
    /// được trừ điểm. Đây là lời thú nhận của công cụ.
    ChuaHoTro,
}

impl Severity {
    pub fn nhan(self) -> &'static str {
        match self {
            Severity::Loi => "lỗi",
            Severity::CanhBao => "cảnh báo",
            Severity::ChuaHoTro => "chưa hỗ trợ",
        }
    }
}

/// Kết cục của một lần chạy — ba khả năng, không phải hai.
///
/// Engine cũ chỉ có `Đạt`/`Không đạt`, nên mọi thứ nó không hiểu đều bị ép vào
/// một trong hai — và cả hai đều là lời nói dối. Xem `docs/decisions/ADR-002`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KetCuc {
    /// Byte đã chạy code và nó đúng.
    Dat,
    /// Byte đã chạy code và nó sai.
    KhongDat,
    /// Byte không đủ sức đưa ra phán quyết.
    ChuaHoTro,
}

impl KetCuc {
    pub fn ma(self) -> &'static str {
        match self {
            KetCuc::Dat => "dat",
            KetCuc::KhongDat => "khong_dat",
            KetCuc::ChuaHoTro => "chua_ho_tro",
        }
    }
}

/// Một nhãn trỏ vào mã nguồn kèm lời giải thích tại chỗ.
#[derive(Debug, Clone)]
pub struct Label {
    pub span: Span,
    pub text: String,
    /// Nhãn chính được hiển thị bằng ký hiệu `^`, nhãn phụ bằng `-`.
    pub chinh: bool,
}

impl Label {
    pub fn chinh(span: Span, text: impl Into<String>) -> Self {
        Self { span, text: text.into(), chinh: true }
    }
    pub fn phu(span: Span, text: impl Into<String>) -> Self {
        Self { span, text: text.into(), chinh: false }
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: &'static str,
    pub severity: Severity,
    /// Tầng 1 — chuyện gì xảy ra, một câu.
    pub message: String,
    pub labels: Vec<Label>,
    /// Tầng 2 — vì sao Rust không cho phép.
    pub vi_sao: Option<String>,
    /// Tầng 3 — sửa thế nào.
    pub cach_sua: Vec<String>,
    /// Khái niệm liên quan, để app mở đúng bài học.
    pub khai_niem: Option<&'static str>,
    /// Tên tính năng chưa hỗ trợ, ví dụ `"borrow-check-cfg"`, `"async"`.
    ///
    /// Chỉ có khi `severity == ChuaHoTro`. App dùng nó để gợi ý lối đi khác
    /// (đối chiếu `cargo` thật trên Desktop, hoặc chuyển sang bài khác).
    pub tinh_nang: Option<&'static str>,
}

impl Diagnostic {
    pub fn loi(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            severity: Severity::Loi,
            message: message.into(),
            labels: Vec::new(),
            vi_sao: None,
            cach_sua: Vec::new(),
            khai_niem: None,
            tinh_nang: None,
        }
    }

    pub fn canh_bao(code: &'static str, message: impl Into<String>) -> Self {
        Self { severity: Severity::CanhBao, ..Self::loi(code, message) }
    }

    /// Byte không đủ sức kiểm tra chỗ này.
    ///
    /// `tinh_nang` là mã ổn định của thứ chưa hỗ trợ (`"async"`,
    /// `"borrow-check-cfg"`, `"macro-nguoi-dung"`…) để app định tuyến.
    pub fn chua_ho_tro(
        code: &'static str,
        tinh_nang: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: Severity::ChuaHoTro,
            tinh_nang: Some(tinh_nang),
            ..Self::loi(code, message)
        }
    }

    pub fn nhan(mut self, label: Label) -> Self {
        self.labels.push(label);
        self
    }

    pub fn tai(mut self, span: Span, text: impl Into<String>) -> Self {
        self.labels.push(Label::chinh(span, text));
        self
    }

    pub fn vi_sao(mut self, text: impl Into<String>) -> Self {
        self.vi_sao = Some(text.into());
        self
    }

    pub fn sua(mut self, text: impl Into<String>) -> Self {
        self.cach_sua.push(text.into());
        self
    }

    pub fn khai_niem(mut self, k: &'static str) -> Self {
        self.khai_niem = Some(k);
        self
    }

    /// Span chính để app cuộn tới và tô sáng.
    pub fn span_chinh(&self) -> Option<Span> {
        self.labels
            .iter()
            .find(|l| l.chinh)
            .or_else(|| self.labels.first())
            .map(|l| l.span)
    }

    /// Kết xuất dạng text nhiều dòng, có trích mã nguồn và con trỏ chỉ lỗi.
    pub fn render(&self, sm: &SourceMap<'_>) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "{} [{}]: {}", self.severity.nhan(), self.code, self.message);

        for label in &self.labels {
            let lc = sm.line_col(label.span.start);
            let dong = sm.line_text(lc.line);
            let so_dong = lc.line.to_string();
            let le = " ".repeat(so_dong.len());

            let _ = writeln!(out, "{le}--> dòng {}:{}", lc.line, lc.col);
            let _ = writeln!(out, "{le} |");
            let _ = writeln!(out, "{so_dong} | {dong}");

            // Con trỏ: đếm theo ký tự để tiếng Việt có dấu không làm lệch.
            let dem = if label.span.is_empty() {
                1
            } else {
                sm.snippet(label.span).chars().count().max(1)
            };
            let ky_hieu = if label.chinh { '^' } else { '-' };
            let _ = writeln!(
                out,
                "{le} | {}{} {}",
                " ".repeat(lc.col.saturating_sub(1)),
                ky_hieu.to_string().repeat(dem),
                label.text
            );
            let _ = writeln!(out, "{le} |");
        }

        if let Some(vi_sao) = &self.vi_sao {
            let _ = writeln!(out, "  vì sao: {vi_sao}");
        }
        for s in &self.cach_sua {
            let _ = writeln!(out, "  cách sửa: {s}");
        }
        if let Some(k) = self.khai_niem {
            let _ = writeln!(out, "  đọc thêm: {k}");
        }
        out
    }
}

/// Tập chẩn đoán của một lần chạy.
#[derive(Debug, Clone, Default)]
pub struct Diagnostics {
    items: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, d: Diagnostic) {
        self.items.push(d);
    }

    pub fn co_loi(&self) -> bool {
        self.items.iter().any(|d| d.severity == Severity::Loi)
    }

    pub fn co_chua_ho_tro(&self) -> bool {
        self.items.iter().any(|d| d.severity == Severity::ChuaHoTro)
    }

    /// Kết cục cuối cùng.
    ///
    /// **`ChuaHoTro` thắng `Loi`.** Nếu ta không hiểu hết chương trình thì không
    /// có tư cách khẳng định nó sai — kể cả khi đã bắt được một lỗi trước đó.
    /// Chọn ngược lại sẽ dẫn tới đúng chế độ hỏng của engine giả cũ: tự tin đưa
    /// ra phán quyết về thứ mình không đọc nổi.
    pub fn ket_cuc(&self) -> KetCuc {
        if self.co_chua_ho_tro() {
            KetCuc::ChuaHoTro
        } else if self.co_loi() {
            KetCuc::KhongDat
        } else {
            KetCuc::Dat
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.items.iter()
    }

    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.items
    }

    /// Chỉ giữ lỗi đầu tiên cộng các cảnh báo.
    ///
    /// Người mới học mà nhận 12 lỗi cùng lúc thì bỏ cuộc; hơn nữa lỗi sau thường
    /// chỉ là hệ quả của lỗi đầu.
    pub fn rut_gon(mut self) -> Self {
        // Chẩn đoán ChuaHoTro luôn được giữ: nó quyết định kết cục.
        if self.co_chua_ho_tro() {
            let giu: Vec<Diagnostic> = self
                .items
                .iter()
                .filter(|d| d.severity != Severity::Loi)
                .cloned()
                .collect();
            self.items = giu;
            return self;
        }
        if let Some(vi_tri) = self.items.iter().position(|d| d.severity == Severity::Loi) {
            let mut giu: Vec<Diagnostic> =
                self.items.iter().take(vi_tri).cloned().collect();
            giu.push(self.items[vi_tri].clone());
            self.items = giu;
        }
        self
    }

    pub fn render(&self, sm: &SourceMap<'_>) -> String {
        self.items
            .iter()
            .map(|d| d.render(sm))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_co_tro_dung_cot() {
        let src = "let x = 1 + ;";
        let sm = SourceMap::new(src);
        let d = Diagnostic::loi("BR0001", "thiếu một giá trị sau dấu `+`")
            .tai(Span::new(12, 13), "chỗ này cần một giá trị")
            .vi_sao("phép `+` cần hai vế; vế phải đang trống")
            .sua("viết giá trị vào, ví dụ `let x = 1 + 2;`");
        let s = d.render(&sm);

        assert!(s.contains("lỗi [BR0001]"));
        assert!(s.contains("--> dòng 1:13"));
        assert!(s.contains("1 | let x = 1 + ;"));
        assert!(s.contains("vì sao:"));
        assert!(s.contains("cách sửa:"));
        // Con trỏ phải nằm dưới đúng cột 13.
        let dong_tro = s.lines().find(|l| l.contains('^')).unwrap();
        assert_eq!(dong_tro.find('^'), Some(4 + 12));
    }

    #[test]
    fn rut_gon_giu_dung_loi_dau_tien() {
        let mut ds = Diagnostics::new();
        ds.push(Diagnostic::canh_bao("BR9001", "biến không dùng"));
        ds.push(Diagnostic::loi("BR0001", "lỗi thật"));
        ds.push(Diagnostic::loi("BR0002", "lỗi hệ quả"));
        let ds = ds.rut_gon();
        assert_eq!(ds.len(), 2);
        assert_eq!(ds.iter().last().unwrap().code, "BR0001");
    }

    #[test]
    fn co_loi_bo_qua_canh_bao() {
        let mut ds = Diagnostics::new();
        ds.push(Diagnostic::canh_bao("BR9001", "chỉ là cảnh báo"));
        assert!(!ds.co_loi());
        ds.push(Diagnostic::loi("BR0001", "lỗi"));
        assert!(ds.co_loi());
    }
}
