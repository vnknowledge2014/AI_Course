// filename: src/main.rs
fn main() {
    let result: Vec<i32> = (1..=10)
        .inspect(|x| print!("→{} ", x))  // debug: thấy input
        .filter(|x| x % 2 == 0)
        .inspect(|x| print!("[{}] ", x))  // debug: thấy sau filter
        .map(|x| x * x)
        .collect();

    println!();
    println!("Result: {:?}", result);
    // →1 →2 [2] →3 →4 [4] →5 →6 [6] →7 →8 [8] →9 →10 [10]
    // Result: [4, 16, 36, 64, 100]
}
