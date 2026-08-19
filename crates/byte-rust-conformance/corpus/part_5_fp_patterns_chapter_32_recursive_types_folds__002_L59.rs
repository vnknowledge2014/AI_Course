// filename: src/main.rs

// ═══ Arithmetic expressions ═══
#[derive(Debug, Clone)]
enum Expr {
    Lit(i32),                           // Con búp bê cuối cùng: số 42
    Add(Box<Expr>, Box<Expr>),          // Tấm thẻ trỏ đến a + b trên Heap
    Mul(Box<Expr>, Box<Expr>),          // Tấm thẻ trỏ đến a * b trên Heap
    Neg(Box<Expr>),                     // Tấm thẻ trỏ đến -a
}

fn main() {}
