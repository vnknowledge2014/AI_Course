// filename: src/main.rs
use std::fmt;

// Wrap Vec trong newtype → bây giờ là "type của bạn"
struct PrettyList(Vec<String>);

impl fmt::Display for PrettyList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

fn main() {
    let list = PrettyList(vec!["Rust".into(), "Go".into(), "Zig".into()]);
    println!("Languages: {}", list);
    // Languages: Rust, Go, Zig
}
