// filename: src/main.rs

// Function trả closure — Factory pattern!
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor  // capture factor bằng move
}

// Function trả closure với config
fn make_validator(min: i32, max: i32) -> impl Fn(i32) -> bool {
    move |value| value >= min && value <= max
}

fn main() {
    // Tạo "máy nhân" tùy chỉnh
    let double = make_multiplier(2);
    let triple = make_multiplier(3);

    println!("double(5) = {}", double(5));   // 10
    println!("triple(5) = {}", triple(5));   // 15

    // Tạo "máy kiểm tra" tùy chỉnh
    let is_valid_age = make_validator(0, 150);
    let is_valid_score = make_validator(0, 100);

    println!("Age 25: {}", is_valid_age(25));      // true
    println!("Age 200: {}", is_valid_age(200));     // false
    println!("Score 85: {}", is_valid_score(85));   // true
    println!("Score 105: {}", is_valid_score(105)); // false
}
