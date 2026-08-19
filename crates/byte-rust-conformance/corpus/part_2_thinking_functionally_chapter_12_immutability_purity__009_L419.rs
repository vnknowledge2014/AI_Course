// filename: src/main.rs
use std::collections::HashMap;

// Pure function: kết quả chỉ phụ thuộc input → cache được!
fn fibonacci(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&result) = cache.get(&n) {
        return result;
    }
    let result = match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1, cache) + fibonacci(n - 2, cache),
    };
    cache.insert(n, result);
    result
}

fn main() {
    let mut cache = HashMap::new();
    println!("fib(50) = {}", fibonacci(50, &mut cache));
    // Lần gọi thứ 2: instant — đã cache!
    println!("fib(50) = {}", fibonacci(50, &mut cache));
    println!("Cache size: {}", cache.len());
}
