//! Tách mã nguồn thành token.
//!
//! Lexer cố tình **không dừng ở lỗi đầu tiên**: gặp ký tự lạ thì ghi nhận chẩn đoán
//! rồi bỏ qua và đi tiếp, để người học thấy được nhiều vấn đề trong một lần chạy.
//! Việc rút gọn xuống một lỗi là do `Diagnostics::rut_gon` quyết định ở tầng trên.

use crate::diag::{Diagnostic, Diagnostics};
use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokKind {
    // Literal
    SoNguyen(i64),
    SoThuc(f64),
    ChuoiVanBan(String),
    KyTu(char),
    DungSai(bool),

    Ten(String),
    TuKhoa(TuKhoa),

    // Toán tử & dấu
    Cong, Tru, Nhan, Chia, Du,
    Gan,
    CongGan, TruGan, NhanGan, ChiaGan, DuGan,
    Bang, KhacBang, NhoHon, LonHon, NhoBang, LonBang,
    Va, Hoac, Phu,
    VaBit, HoacBit, XorBit, DichTrai, DichPhai,
    MuiTen,      // ->
    MuiTenDam,   // =>
    Cham,        // .
    HaiCham,     // ..
    HaiChamBang, // ..=
    Phay,        // ,
    ChamPhay,    // ;
    HaiChamDung, // :
    DuongDan,    // ::
    Hoi,         // ?
    GachDuoi,    // _

    MoTron, DongTron,
    MoVuong, DongVuong,
    MoNhon, DongNhon,

    /// Kết thúc tệp. Luôn là token cuối cùng.
    HetTep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuKhoa {
    Let, Mut, Fn, If, Else, Match, Loop, While, For, In,
    Return, Break, Continue,
    Struct, Enum, Impl, Trait, Pub, Use, Mod, Const, Static,
    SelfHoa,   // Self
    SelfThuong, // self
    As, Ref, Where, Type,
}

impl TuKhoa {
    fn tu_chuoi(s: &str) -> Option<TuKhoa> {
        use TuKhoa::*;
        Some(match s {
            "let" => Let, "mut" => Mut, "fn" => Fn,
            "if" => If, "else" => Else, "match" => Match,
            "loop" => Loop, "while" => While, "for" => For, "in" => In,
            "return" => Return, "break" => Break, "continue" => Continue,
            "struct" => Struct, "enum" => Enum, "impl" => Impl, "trait" => Trait,
            "pub" => Pub, "use" => Use, "mod" => Mod, "const" => Const, "static" => Static,
            "Self" => SelfHoa, "self" => SelfThuong,
            "as" => As, "ref" => Ref, "where" => Where, "type" => Type,
            _ => return None,
        })
    }

    pub fn ten(self) -> &'static str {
        use TuKhoa::*;
        match self {
            Let => "let", Mut => "mut", Fn => "fn", If => "if", Else => "else",
            Match => "match", Loop => "loop", While => "while", For => "for", In => "in",
            Return => "return", Break => "break", Continue => "continue",
            Struct => "struct", Enum => "enum", Impl => "impl", Trait => "trait",
            Pub => "pub", Use => "use", Mod => "mod", Const => "const", Static => "static",
            SelfHoa => "Self", SelfThuong => "self",
            As => "as", Ref => "ref", Where => "where", Type => "type",
        }
    }
}

impl TokKind {
    /// Tên hiển thị trong thông báo lỗi, viết cho người mới đọc được.
    pub fn mo_ta(&self) -> String {
        use TokKind::*;
        match self {
            SoNguyen(n) => format!("số `{n}`"),
            SoThuc(f) => format!("số `{f}`"),
            ChuoiVanBan(_) => "một chuỗi văn bản".to_string(),
            KyTu(c) => format!("ký tự `'{c}'`"),
            DungSai(b) => format!("`{b}`"),
            Ten(t) => format!("tên `{t}`"),
            TuKhoa(k) => format!("từ khoá `{}`", k.ten()),
            Cong => "dấu `+`".into(), Tru => "dấu `-`".into(),
            Nhan => "dấu `*`".into(), Chia => "dấu `/`".into(), Du => "dấu `%`".into(),
            Gan => "dấu `=`".into(),
            CongGan => "dấu `+=`".into(), TruGan => "dấu `-=`".into(),
            NhanGan => "dấu `*=`".into(), ChiaGan => "dấu `/=`".into(), DuGan => "dấu `%=`".into(),
            Bang => "dấu `==`".into(), KhacBang => "dấu `!=`".into(),
            NhoHon => "dấu `<`".into(), LonHon => "dấu `>`".into(),
            NhoBang => "dấu `<=`".into(), LonBang => "dấu `>=`".into(),
            Va => "dấu `&&`".into(), Hoac => "dấu `||`".into(), Phu => "dấu `!`".into(),
            VaBit => "dấu `&`".into(), HoacBit => "dấu `|`".into(), XorBit => "dấu `^`".into(),
            DichTrai => "dấu `<<`".into(), DichPhai => "dấu `>>`".into(),
            MuiTen => "dấu `->`".into(), MuiTenDam => "dấu `=>`".into(),
            Cham => "dấu `.`".into(), HaiCham => "dấu `..`".into(), HaiChamBang => "dấu `..=`".into(),
            Phay => "dấu `,`".into(), ChamPhay => "dấu `;`".into(),
            HaiChamDung => "dấu `:`".into(), DuongDan => "dấu `::`".into(),
            Hoi => "dấu `?`".into(), GachDuoi => "dấu `_`".into(),
            MoTron => "dấu `(`".into(), DongTron => "dấu `)`".into(),
            MoVuong => "dấu `[`".into(), DongVuong => "dấu `]`".into(),
            MoNhon => "dấu `{`".into(), DongNhon => "dấu `}`".into(),
            HetTep => "hết chương trình".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokKind,
    pub span: Span,
}

pub struct Lexer<'a> {
    src: &'a str,
    b: Vec<char>,
    /// Offset **byte** ứng với từng phần tử của `b`.
    offsets: Vec<usize>,
    i: usize,
    pub diags: Diagnostics,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let b: Vec<char> = src.chars().collect();
        let mut offsets = Vec::with_capacity(b.len() + 1);
        let mut off = 0usize;
        for c in &b {
            offsets.push(off);
            off += c.len_utf8();
        }
        offsets.push(off);
        Self { src, b, offsets, i: 0, diags: Diagnostics::new() }
    }

    fn off(&self, idx: usize) -> usize {
        *self.offsets.get(idx).unwrap_or(&self.src.len())
    }

    fn peek(&self) -> Option<char> {
        self.b.get(self.i).copied()
    }

    fn peek2(&self) -> Option<char> {
        self.b.get(self.i + 1).copied()
    }

    fn peek3(&self) -> Option<char> {
        self.b.get(self.i + 2).copied()
    }

    fn tien(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.i += 1;
        Some(c)
    }

    fn span_tu(&self, bat_dau_idx: usize) -> Span {
        Span::new(self.off(bat_dau_idx), self.off(self.i))
    }

    /// Bỏ qua khoảng trắng và chú thích. Trả về `false` nếu gặp chú thích khối chưa đóng.
    fn bo_qua_rac(&mut self) -> bool {
        loop {
            match (self.peek(), self.peek2()) {
                (Some(c), _) if c.is_whitespace() => {
                    self.i += 1;
                }
                (Some('/'), Some('/')) => {
                    while let Some(c) = self.peek() {
                        if c == '\n' { break; }
                        self.i += 1;
                    }
                }
                (Some('/'), Some('*')) => {
                    let bat_dau = self.i;
                    self.i += 2;
                    let mut do_sau = 1usize;
                    while do_sau > 0 {
                        match (self.peek(), self.peek2()) {
                            (Some('/'), Some('*')) => { self.i += 2; do_sau += 1; }
                            (Some('*'), Some('/')) => { self.i += 2; do_sau -= 1; }
                            (Some(_), _) => { self.i += 1; }
                            (None, _) => {
                                self.diags.push(
                                    Diagnostic::loi("BR0002", "chú thích khối chưa được đóng")
                                        .tai(Span::new(self.off(bat_dau), self.off(bat_dau + 2)),
                                             "chú thích mở ở đây")
                                        .vi_sao("mọi `/*` đều phải có một `*/` tương ứng để đóng lại")
                                        .sua("thêm `*/` ở cuối phần chú thích")
                                        .khai_niem("chú thích"),
                                );
                                return false;
                            }
                        }
                    }
                }
                _ => return true,
            }
        }
    }

    pub fn quet(mut self) -> (Vec<Token>, Diagnostics) {
        let mut toks = Vec::new();
        loop {
            if !self.bo_qua_rac() {
                break;
            }
            let bat_dau = self.i;
            let Some(c) = self.peek() else { break };

            let kind = if c.is_ascii_digit() {
                self.doc_so()
            } else if c == '_' && !self.la_ky_tu_ten(self.peek2()) {
                self.i += 1;
                Some(TokKind::GachDuoi)
            } else if c.is_alphabetic() || c == '_' {
                self.doc_ten()
            } else if c == '"' {
                self.doc_chuoi()
            } else if c == '\'' {
                self.doc_ky_tu()
            } else {
                self.doc_dau()
            };

            match kind {
                Some(k) => toks.push(Token { kind: k, span: self.span_tu(bat_dau) }),
                None => {
                    // `doc_dau` đã ghi chẩn đoán và tự tiến con trỏ.
                    if self.i == bat_dau {
                        self.i += 1; // bảo hiểm chống vòng lặp vô hạn
                    }
                }
            }
        }
        let cuoi = self.src.len();
        toks.push(Token { kind: TokKind::HetTep, span: Span::at(cuoi) });
        (toks, self.diags)
    }

    fn la_ky_tu_ten(&self, c: Option<char>) -> bool {
        matches!(c, Some(c) if c.is_alphanumeric() || c == '_')
    }

    fn doc_ten(&mut self) -> Option<TokKind> {
        let bat_dau = self.i;
        while self.la_ky_tu_ten(self.peek()) {
            self.i += 1;
        }
        let s: String = self.b[bat_dau..self.i].iter().collect();
        Some(match s.as_str() {
            "true" => TokKind::DungSai(true),
            "false" => TokKind::DungSai(false),
            _ => match TuKhoa::tu_chuoi(&s) {
                Some(k) => TokKind::TuKhoa(k),
                None => TokKind::Ten(s),
            },
        })
    }

    fn doc_so(&mut self) -> Option<TokKind> {
        let bat_dau = self.i;

        // Cơ số khác 10: 0x / 0b / 0o
        if self.peek() == Some('0') {
            if let Some(p) = self.peek2() {
                let co_so = match p {
                    'x' | 'X' => Some(16u32),
                    'b' | 'B' => Some(2),
                    'o' | 'O' => Some(8),
                    _ => None,
                };
                if let Some(co_so) = co_so {
                    self.i += 2;
                    let bd_so = self.i;
                    while matches!(self.peek(), Some(c) if c.is_alphanumeric() || c == '_') {
                        self.i += 1;
                    }
                    let raw: String =
                        self.b[bd_so..self.i].iter().filter(|c| **c != '_').collect();
                    return match i64::from_str_radix(&raw, co_so) {
                        Ok(n) => Some(TokKind::SoNguyen(n)),
                        Err(_) => {
                            self.diags.push(
                                Diagnostic::loi("BR0003", format!("số cơ số {co_so} không hợp lệ"))
                                    .tai(self.span_tu(bat_dau), "chỗ này không đọc được thành số")
                                    .vi_sao(match co_so {
                                        16 => "sau `0x` chỉ được dùng chữ số 0-9 và chữ cái a-f",
                                        2 => "sau `0b` chỉ được dùng chữ số 0 và 1",
                                        _ => "sau `0o` chỉ được dùng chữ số 0-7",
                                    })
                                    .khai_niem("số nguyên"),
                            );
                            None
                        }
                    };
                }
            }
        }

        while matches!(self.peek(), Some(c) if c.is_ascii_digit() || c == '_') {
            self.i += 1;
        }

        // Phần thập phân — nhưng `1..5` là dải, không phải số thực.
        let mut la_so_thuc = false;
        if self.peek() == Some('.') && matches!(self.peek2(), Some(c) if c.is_ascii_digit()) {
            la_so_thuc = true;
            self.i += 1;
            while matches!(self.peek(), Some(c) if c.is_ascii_digit() || c == '_') {
                self.i += 1;
            }
        }

        // Hậu tố kiểu: 10i32, 3.5f64 — chấp nhận và bỏ qua.
        let bd_hau_to = self.i;
        while self.la_ky_tu_ten(self.peek()) {
            self.i += 1;
        }
        let hau_to: String = self.b[bd_hau_to..self.i].iter().collect();
        if !hau_to.is_empty() && !matches!(hau_to.as_str(),
            "i8"|"i16"|"i32"|"i64"|"i128"|"isize"|
            "u8"|"u16"|"u32"|"u64"|"u128"|"usize"|"f32"|"f64")
        {
            self.diags.push(
                Diagnostic::loi("BR0004", format!("`{hau_to}` không phải hậu tố kiểu hợp lệ"))
                    .tai(Span::new(self.off(bd_hau_to), self.off(self.i)),
                         "hậu tố này không hiểu được")
                    .vi_sao("viết liền chữ sau số thì Rust hiểu đó là hậu tố chỉ kiểu, ví dụ `10i32`")
                    .sua("nếu định nhân thì thêm dấu `*`, ví dụ `10 * x` thay vì `10x`")
                    .khai_niem("số nguyên"),
            );
            return None;
        }
        if la_so_thuc && matches!(hau_to.as_str(), "f32" | "f64" | "") {
            // hợp lệ
        }

        let raw: String = self.b[bat_dau..bd_hau_to].iter().filter(|c| **c != '_').collect();
        if la_so_thuc {
            match raw.parse::<f64>() {
                Ok(f) => Some(TokKind::SoThuc(f)),
                Err(_) => {
                    self.diags.push(
                        Diagnostic::loi("BR0003", "số thực không hợp lệ")
                            .tai(self.span_tu(bat_dau), "chỗ này không đọc được thành số"),
                    );
                    None
                }
            }
        } else {
            match raw.parse::<i64>() {
                Ok(n) => Some(TokKind::SoNguyen(n)),
                Err(_) => {
                    self.diags.push(
                        Diagnostic::loi("BR0005", "số nguyên quá lớn")
                            .tai(self.span_tu(bat_dau), format!("`{raw}` vượt quá sức chứa"))
                            .vi_sao("kiểu số nguyên mặc định là `i64`, chứa được tới 9.223.372.036.854.775.807")
                            .khai_niem("số nguyên"),
                    );
                    None
                }
            }
        }
    }

    fn doc_thoat(&mut self, mo_dau: usize) -> Option<char> {
        // Con trỏ đang ở ngay sau dấu `\`.
        let c = self.tien()?;
        Some(match c {
            'n' => '\n', 't' => '\t', 'r' => '\r',
            '0' => '\0', '\\' => '\\', '\'' => '\'', '"' => '"',
            khac => {
                self.diags.push(
                    Diagnostic::loi("BR0006", format!("`\\{khac}` không phải chuỗi thoát hợp lệ"))
                        .tai(Span::new(self.off(self.i - 2), self.off(self.i)),
                             "chuỗi thoát này không hiểu được")
                        .vi_sao("dấu `\\` báo cho Rust biết ký tự sau nó có nghĩa đặc biệt")
                        .sua("các chuỗi thoát dùng được: `\\n` xuống dòng, `\\t` tab, `\\\\` dấu gạch chéo, `\\\"` dấu nháy kép")
                        .khai_niem("chuỗi văn bản"),
                );
                let _ = mo_dau;
                khac
            }
        })
    }

    fn doc_chuoi(&mut self) -> Option<TokKind> {
        let mo_dau = self.i;
        self.i += 1; // bỏ qua `"`
        let mut s = String::new();
        loop {
            match self.tien() {
                Some('"') => return Some(TokKind::ChuoiVanBan(s)),
                Some('\\') => {
                    if let Some(c) = self.doc_thoat(mo_dau) {
                        s.push(c);
                    }
                }
                Some('\n') | None => {
                    self.diags.push(
                        Diagnostic::loi("BR0007", "chuỗi văn bản chưa được đóng")
                            .tai(Span::new(self.off(mo_dau), self.off(mo_dau + 1)),
                                 "chuỗi mở ở đây")
                            .vi_sao("mọi dấu nháy kép mở đều cần một dấu nháy kép đóng trên cùng một dòng")
                            .sua("thêm dấu `\"` vào cuối chuỗi")
                            .khai_niem("chuỗi văn bản"),
                    );
                    return None;
                }
                Some(c) => s.push(c),
            }
        }
    }

    fn doc_ky_tu(&mut self) -> Option<TokKind> {
        let mo_dau = self.i;
        self.i += 1; // bỏ qua `'`
        let c = match self.tien() {
            Some('\\') => self.doc_thoat(mo_dau)?,
            Some('\'') => {
                self.diags.push(
                    Diagnostic::loi("BR0008", "ký tự rỗng")
                        .tai(Span::new(self.off(mo_dau), self.off(self.i)), "không có ký tự nào ở đây")
                        .vi_sao("`char` phải chứa đúng một ký tự")
                        .sua("nếu muốn chuỗi rỗng thì dùng nháy kép: `\"\"`")
                        .khai_niem("ký tự"),
                );
                return None;
            }
            Some(c) => c,
            None => return None,
        };
        if self.peek() == Some('\'') {
            self.i += 1;
            Some(TokKind::KyTu(c))
        } else {
            self.diags.push(
                Diagnostic::loi("BR0009", "ký tự chưa được đóng, hoặc chứa nhiều hơn một ký tự")
                    .tai(Span::new(self.off(mo_dau), self.off(self.i.min(self.b.len()))),
                         "chỗ này cần đóng bằng dấu `'`")
                    .vi_sao("nháy đơn `'a'` dùng cho MỘT ký tự; nhiều ký tự phải dùng nháy kép `\"abc\"`")
                    .sua("nếu định viết chuỗi văn bản, đổi `'` thành `\"`")
                    .khai_niem("ký tự"),
            );
            None
        }
    }

    fn doc_dau(&mut self) -> Option<TokKind> {
        use TokKind::*;
        let bat_dau = self.i;
        let c = self.peek()?;
        let c2 = self.peek2();
        let c3 = self.peek3();

        // Ba ký tự
        if c == '.' && c2 == Some('.') && c3 == Some('=') {
            self.i += 3;
            return Some(HaiChamBang);
        }

        // Hai ký tự
        let hai = match (c, c2) {
            ('+', Some('=')) => Some(CongGan),
            ('-', Some('=')) => Some(TruGan),
            ('*', Some('=')) => Some(NhanGan),
            ('/', Some('=')) => Some(ChiaGan),
            ('%', Some('=')) => Some(DuGan),
            ('=', Some('=')) => Some(Bang),
            ('!', Some('=')) => Some(KhacBang),
            ('<', Some('=')) => Some(NhoBang),
            ('>', Some('=')) => Some(LonBang),
            ('&', Some('&')) => Some(Va),
            ('|', Some('|')) => Some(Hoac),
            ('<', Some('<')) => Some(DichTrai),
            ('>', Some('>')) => Some(DichPhai),
            ('-', Some('>')) => Some(MuiTen),
            ('=', Some('>')) => Some(MuiTenDam),
            ('.', Some('.')) => Some(HaiCham),
            (':', Some(':')) => Some(DuongDan),
            _ => None,
        };
        if let Some(k) = hai {
            self.i += 2;
            return Some(k);
        }

        // Một ký tự
        let mot = match c {
            '+' => Some(Cong), '-' => Some(Tru), '*' => Some(Nhan),
            '/' => Some(Chia), '%' => Some(Du),
            '=' => Some(Gan), '<' => Some(NhoHon), '>' => Some(LonHon),
            '!' => Some(Phu), '&' => Some(VaBit), '|' => Some(HoacBit), '^' => Some(XorBit),
            '.' => Some(Cham), ',' => Some(Phay), ';' => Some(ChamPhay),
            ':' => Some(HaiChamDung), '?' => Some(Hoi),
            '(' => Some(MoTron), ')' => Some(DongTron),
            '[' => Some(MoVuong), ']' => Some(DongVuong),
            '{' => Some(MoNhon), '}' => Some(DongNhon),
            _ => None,
        };
        if let Some(k) = mot {
            self.i += 1;
            return Some(k);
        }

        // Ký tự không thuộc Rust — đây là chỗ người mới hay vấp: dấu câu tiếng Việt,
        // dấu nháy cong do soạn thảo văn bản, ký tự toàn giác.
        self.i += 1;
        let span = self.span_tu(bat_dau);
        let mut d = Diagnostic::loi("BR0001", format!("ký tự `{c}` không dùng được trong Rust"))
            .tai(span, "ký tự này không có ý nghĩa gì ở đây")
            .khai_niem("cú pháp cơ bản");
        d = match c {
            '“' | '”' => d
                .vi_sao("đây là dấu nháy kép cong, thường do trình soạn thảo văn bản tự đổi")
                .sua("thay bằng dấu nháy kép thẳng `\"`"),
            '‘' | '’' => d
                .vi_sao("đây là dấu nháy đơn cong, thường do trình soạn thảo văn bản tự đổi")
                .sua("thay bằng dấu nháy đơn thẳng `'`"),
            '；' | '，' | '（' | '）' => d
                .vi_sao("đây là dấu câu toàn giác (full-width), không phải dấu ASCII mà Rust hiểu")
                .sua("gõ lại bằng bàn phím ở chế độ tiếng Anh"),
            '#' => d
                .vi_sao("`#` trong Rust chỉ dùng cho thuộc tính như `#[derive(Debug)]`")
                .sua("nếu muốn viết chú thích, dùng `//`"),
            '$' => d
                .vi_sao("`$` chỉ có nghĩa bên trong macro_rules!")
                .sua("nếu muốn chèn giá trị vào chuỗi, dùng `println!(\"{}\", x)`"),
            _ => d.sua("xoá ký tự này đi"),
        };
        self.diags.push(d);
        None
    }
}

/// Tiện ích: quét một chuỗi thành token.
pub fn quet(src: &str) -> (Vec<Token>, Diagnostics) {
    Lexer::new(src).quet()
}
