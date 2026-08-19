// filename: src/main.rs

// Iterator đếm theo bước tùy chỉnh
struct StepCounter {
    current: u32,
    step: u32,
    max: u32,
}

impl StepCounter {
    fn new(start: u32, step: u32, max: u32) -> Self {
        StepCounter { current: start, step, max }
    }
}

impl Iterator for StepCounter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max {
            None  // kết thúc
        } else {
            let value = self.current;
            self.current += self.step;
            Some(value)
        }
    }
}

fn main() {
    // Đếm từ 0 đến 20, bước 3
    let steps: Vec<u32> = StepCounter::new(0, 3, 20).collect();
    println!("Steps: {:?}", steps);
    // Steps: [0, 3, 6, 9, 12, 15, 18]

    // Kết hợp với iterator chains!
    let sum: u32 = StepCounter::new(1, 2, 15)  // odds: 1,3,5,7,9,11,13,15
        .filter(|x| x % 3 != 0)                 // bỏ chia hết cho 3
        .map(|x| x * x)                         // bình phương
        .sum();
    println!("Sum of squares: {}", sum);
    // 1,5,7,11,13 → 1,25,49,121,169 → 365
}
