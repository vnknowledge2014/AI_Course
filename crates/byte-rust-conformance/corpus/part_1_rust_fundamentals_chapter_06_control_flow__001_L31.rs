// filename: src/main.rs
fn main() {
    let score = 85;

    // ❌ Kiểu "statement" (vẫn chạy nhưng dài dòng)
    let grade_verbose;
    if score >= 90 {
        grade_verbose = "A";
    } else if score >= 70 {
        grade_verbose = "B";
    } else {
        grade_verbose = "C";
    }

    // ✅ Kiểu "expression" (idiomatic Rust) — gọn, rõ, an toàn
    let grade = if score >= 90 {
        "A"
    } else if score >= 70 {
        "B"
    } else {
        "C"
    };  // ← chú ý dấu ;

    assert_eq!(grade_verbose, grade);
    println!("Score {}: {}", score, grade);
    // Output: Score 85: B
}
