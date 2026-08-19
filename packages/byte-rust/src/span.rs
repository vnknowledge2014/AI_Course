//! Vị trí trong mã nguồn.
//!
//! Mọi token, nút AST và chẩn đoán đều mang một `Span`. Nhờ vậy thông báo lỗi
//! chỉ đúng vào ký tự gây lỗi thay vì chỉ nói "có lỗi ở đâu đó".

/// Khoảng byte nửa mở `[start, end)` trong mã nguồn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Span rỗng tại một vị trí — dùng cho lỗi kiểu "thiếu thứ gì đó ở đây".
    pub const fn at(pos: usize) -> Self {
        Self { start: pos, end: pos }
    }

    /// Span nhỏ nhất chứa cả hai.
    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Vị trí dạng người đọc được: dòng và cột, đếm từ 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineCol {
    pub line: usize,
    pub col: usize,
}

/// Bảng tra dòng/cột từ offset byte.
///
/// Dựng một lần cho mỗi lần chạy, rồi tra `O(log n)` cho từng chẩn đoán.
pub struct SourceMap<'a> {
    src: &'a str,
    /// Offset byte của ký tự đầu mỗi dòng.
    line_starts: Vec<usize>,
}

impl<'a> SourceMap<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut line_starts = vec![0usize];
        for (i, b) in src.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(i + 1);
            }
        }
        Self { src, line_starts }
    }

    pub fn source(&self) -> &'a str {
        self.src
    }

    /// Đổi offset byte thành dòng/cột (đếm từ 1).
    ///
    /// Cột đếm theo **ký tự**, không theo byte — để tiếng Việt có dấu không làm
    /// lệch con trỏ chỉ lỗi.
    pub fn line_col(&self, offset: usize) -> LineCol {
        let offset = offset.min(self.src.len());
        let line_idx = match self.line_starts.binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        let line_start = self.line_starts[line_idx];
        let col = self.src[line_start..offset].chars().count() + 1;
        LineCol { line: line_idx + 1, col }
    }

    /// Lấy nguyên văn một dòng (không kèm ký tự xuống dòng). `line` đếm từ 1.
    pub fn line_text(&self, line: usize) -> &'a str {
        if line == 0 || line > self.line_starts.len() {
            return "";
        }
        let start = self.line_starts[line - 1];
        let end = self
            .line_starts
            .get(line)
            .map(|&e| e.saturating_sub(1))
            .unwrap_or(self.src.len());
        self.src[start..end].trim_end_matches('\r')
    }

    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /// Đoạn mã nguồn mà span trỏ tới.
    pub fn snippet(&self, span: Span) -> &'a str {
        let start = span.start.min(self.src.len());
        let end = span.end.min(self.src.len()).max(start);
        &self.src[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_col_dem_tu_mot() {
        let sm = SourceMap::new("let x = 1;\nlet y = 2;\n");
        assert_eq!(sm.line_col(0), LineCol { line: 1, col: 1 });
        assert_eq!(sm.line_col(4), LineCol { line: 1, col: 5 });
        assert_eq!(sm.line_col(11), LineCol { line: 2, col: 1 });
    }

    #[test]
    fn cot_dem_theo_ky_tu_khong_theo_byte() {
        // "á" chiếm 2 byte trong UTF-8; cột phải là 2 chứ không phải 3.
        let src = "áx";
        let sm = SourceMap::new(src);
        let offset_cua_x = "á".len();
        assert_eq!(sm.line_col(offset_cua_x), LineCol { line: 1, col: 2 });
    }

    #[test]
    fn line_text_bo_ky_tu_xuong_dong() {
        let sm = SourceMap::new("alpha\nbeta\r\ngamma");
        assert_eq!(sm.line_text(1), "alpha");
        assert_eq!(sm.line_text(2), "beta");
        assert_eq!(sm.line_text(3), "gamma");
        assert_eq!(sm.line_text(99), "");
    }

    #[test]
    fn merge_lay_bao_ngoai() {
        assert_eq!(Span::new(2, 5).merge(Span::new(10, 12)), Span::new(2, 12));
        assert_eq!(Span::new(10, 12).merge(Span::new(2, 5)), Span::new(2, 12));
    }

    #[test]
    fn snippet_khong_panic_khi_span_vuot_bien() {
        let sm = SourceMap::new("abc");
        assert_eq!(sm.snippet(Span::new(1, 99)), "bc");
        assert_eq!(sm.snippet(Span::new(99, 200)), "");
    }
}
