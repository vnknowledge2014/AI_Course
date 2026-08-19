// filename: src/main.rs

// ✅ PURE: cùng input → cùng output, không side-effects
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn discount(price: u32, percent: u32) -> u32 {
    price - (price * percent / 100)
}

fn classify_age(age: u32) -> &'static str {
    match age {
        0..=12  => "Child",
        13..=17 => "Teen",
        18..=64 => "Adult",
        _       => "Senior",
    }
}

// ❌ IMPURE: side-effects
fn impure_greet(name: &str) {
    println!("Hello, {}!", name);  // side-effect: I/O
}

static mut COUNTER: u32 = 0;
fn impure_count() -> u32 {
    unsafe {
        COUNTER += 1;  // side-effect: mutate global
        COUNTER
    }
}

fn main() {
    // Pure: gọi bao nhiêu lần kết quả cũng giống nhau
    assert_eq!(add(3, 5), 8);
    assert_eq!(add(3, 5), 8);  // luôn = 8

    assert_eq!(discount(100_000, 20), 80_000);
    assert_eq!(classify_age(25), "Adult");

    println!("35000đ giảm 10% = {}đ", discount(35_000, 10));
    println!("Age 25 = {}", classify_age(25));
}
