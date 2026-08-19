// filename: src/main.rs
fn main() {
    // Với types trên stack (i32, bool, f64): COPY
    let a = 42;
    let b = a;     // COPY — a vẫn dùng được
    println!("a={}, b={}", a, b);  // ✅ OK

    // Với types trên heap (String, Vec): MOVE (chuyển ownership)
    let s1 = String::from("Hello");
    let s2 = s1;   // MOVE — s1 đã bị "chuyển" cho s2
    // println!("{}", s1);  // ❌ error: borrow of moved value: `s1`
    println!("s2={}", s2);  // ✅ OK — s2 là owner mới
}
