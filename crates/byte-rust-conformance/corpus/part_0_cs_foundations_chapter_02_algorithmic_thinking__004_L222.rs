// filename: src/main.rs

// ❌ Recursive thường: phải nhớ n * kết_quả_sau
fn factorial_normal(n: u64) -> u64 {
    if n <= 1 { 1 }
    else { n * factorial_normal(n - 1) }
    //     ↑ phải chờ kết quả rồi mới nhân → KHÔNG phải tail
}

// ✅ Tail recursive: mang theo accumulator
fn factorial_tail(n: u64, acc: u64) -> u64 {
    if n <= 1 { acc }
    else { factorial_tail(n - 1, n * acc) }
    //     ↑ recursive call là việc CUỐI CÙNG → IS tail recursive
}

fn main() {
    // Cả hai cho cùng kết quả
    assert_eq!(factorial_normal(10), 3628800);
    assert_eq!(factorial_tail(10, 1), 3628800);  // acc bắt đầu = 1

    println!("10! = {}", factorial_tail(10, 1));
    // Output: 10! = 3628800
}
