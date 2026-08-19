// Mô phỏng lõi của một interpreter Rust-subset: tokenize + eval biểu thức.
// Mục đích: đo kích thước và thời gian build wasm32 cho một crate có logic thật.
#[derive(Debug, Clone, PartialEq)]
enum Tok { Num(i64), Plus, Minus, Star, Slash, LParen, RParen }

fn lex(s: &str) -> Option<Vec<Tok>> {
    let mut out = Vec::new();
    let b: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        match c {
            ' ' | '\t' => { i += 1; }
            '+' => { out.push(Tok::Plus); i += 1; }
            '-' => { out.push(Tok::Minus); i += 1; }
            '*' => { out.push(Tok::Star); i += 1; }
            '/' => { out.push(Tok::Slash); i += 1; }
            '(' => { out.push(Tok::LParen); i += 1; }
            ')' => { out.push(Tok::RParen); i += 1; }
            '0'..='9' => {
                let mut n: i64 = 0;
                while i < b.len() && b[i].is_ascii_digit() {
                    n = n * 10 + (b[i] as i64 - '0' as i64);
                    i += 1;
                }
                out.push(Tok::Num(n));
            }
            _ => return None,
        }
    }
    Some(out)
}

struct P { t: Vec<Tok>, i: usize }
impl P {
    fn expr(&mut self) -> Option<i64> {
        let mut v = self.term()?;
        while self.i < self.t.len() {
            match self.t[self.i] {
                Tok::Plus  => { self.i += 1; v += self.term()?; }
                Tok::Minus => { self.i += 1; v -= self.term()?; }
                _ => break,
            }
        }
        Some(v)
    }
    fn term(&mut self) -> Option<i64> {
        let mut v = self.atom()?;
        while self.i < self.t.len() {
            match self.t[self.i] {
                Tok::Star  => { self.i += 1; v *= self.atom()?; }
                Tok::Slash => { self.i += 1; let d = self.atom()?; if d == 0 { return None; } v /= d; }
                _ => break,
            }
        }
        Some(v)
    }
    fn atom(&mut self) -> Option<i64> {
        match self.t.get(self.i)? {
            Tok::Num(n) => { let n = *n; self.i += 1; Some(n) }
            Tok::LParen => { self.i += 1; let v = self.expr()?; if self.t.get(self.i) == Some(&Tok::RParen) { self.i += 1; Some(v) } else { None } }
            Tok::Minus  => { self.i += 1; Some(-self.atom()?) }
            _ => None,
        }
    }
}

static mut BUF: [u8; 4096] = [0; 4096];

#[no_mangle]
pub extern "C" fn input_ptr() -> *mut u8 { unsafe { core::ptr::addr_of_mut!(BUF) as *mut u8 } }

#[no_mangle]
pub extern "C" fn eval(len: usize) -> i64 {
    let s = unsafe { core::str::from_utf8_unchecked(&*core::ptr::slice_from_raw_parts(core::ptr::addr_of!(BUF) as *const u8, len)) };
    match lex(s).and_then(|t| P { t, i: 0 }.expr()) {
        Some(v) => v,
        None => i64::MIN,
    }
}
