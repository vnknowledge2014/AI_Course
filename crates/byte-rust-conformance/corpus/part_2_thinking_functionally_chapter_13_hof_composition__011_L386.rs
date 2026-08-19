// filename: src/main.rs

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(value)  // vô hạn — luôn trả Some
    }
}

fn main() {
    // Lấy 10 số Fibonacci đầu tiên
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("Fib: {:?}", fibs);
    // Fib: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

    // Tổng Fibonacci < 1000
    let sum: u64 = Fibonacci::new()
        .take_while(|&x| x < 1000)
        .sum();
    println!("Sum of Fib < 1000: {}", sum);
    // Sum of Fib < 1000: 1596
}
