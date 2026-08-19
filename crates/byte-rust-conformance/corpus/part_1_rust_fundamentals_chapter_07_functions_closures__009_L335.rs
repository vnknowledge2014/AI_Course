// filename: src/main.rs

// fn pointer: fn(i32) -> i32 — chỉ cho function KHÔNG capture
// Closure trait: Fn(i32) -> i32 — cho cả function VÀ closure

// Dùng generic → chấp nhận cả hai
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn double(x: i32) -> i32 { x * 2 }

fn main() {
    // Truyền function thường
    println!("double(5) = {}", apply(double, 5));

    // Truyền closure
    let offset = 10;
    println!("add_10(5) = {}", apply(|x| x + offset, 5));

    // Output:
    // double(5) = 10
    // add_10(5) = 15
}
