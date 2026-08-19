// filename: src/main.rs

// ❌ BÀI 1: Viết function cho TỪNG type? Duplicate!
fn max_i32(a: i32, b: i32) -> i32 { if a >= b { a } else { b } }
fn max_f64(a: f64, b: f64) -> f64 { if a >= b { a } else { b } }
fn max_str<'a>(a: &'a str, b: &'a str) -> &'a str { if a >= b { a } else { b } }

// ✅ Generic: viết 1 lần, chạy cho MỌI type có PartialOrd
fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

fn main() {
    println!("max(3, 5) = {}", max_of(3, 5));
    println!("max(3.14, 2.71) = {}", max_of(3.14, 2.71));
    println!("max(\"apple\", \"banana\") = {}", max_of("apple", "banana"));
}
