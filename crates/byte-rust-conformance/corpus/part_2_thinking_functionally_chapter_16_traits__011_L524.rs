// filename: src/main.rs

// Return impl Trait — ẩn concrete type, compiler infer
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n  // closure — type phức tạp, nhưng caller chỉ thấy "Fn(i32)->i32"
}

// Return impl Iterator — rất phổ biến
fn fibonacci() -> impl Iterator<Item = u64> {
    let mut state = (0u64, 1u64);
    std::iter::from_fn(move || {
        let next = state.0;
        state = (state.1, state.0 + state.1);
        Some(next)
    })
}

fn main() {
    let add5 = make_adder(5);
    println!("{}", add5(10));  // 15

    let fibs: Vec<u64> = fibonacci().take(10).collect();
    println!("{:?}", fibs);  // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
}
