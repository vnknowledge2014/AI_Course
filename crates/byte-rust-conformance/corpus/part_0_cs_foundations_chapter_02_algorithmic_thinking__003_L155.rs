// filename: src/main.rs

// Cách 1: Recursion
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1           // Base case: con búp bê bé nhất
    } else {
        n * factorial(n - 1)  // Mở con búp bê tiếp theo
    }
}

// Cách 2: Iteration (vòng lặp)
fn factorial_loop(n: u64) -> u64 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}

fn main() {
    // Cả hai cho cùng kết quả
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial_loop(5), 120);

    for n in 0..=10 {
        println!("{}! = {}", n, factorial(n));
    }
    // Output:
    // 0! = 1
    // 1! = 1
    // 2! = 2
    // 3! = 6
    // 4! = 24
    // 5! = 120
    // 6! = 720
    // 7! = 5040
    // 8! = 40320
    // 9! = 362880
    // 10! = 3628800
}
