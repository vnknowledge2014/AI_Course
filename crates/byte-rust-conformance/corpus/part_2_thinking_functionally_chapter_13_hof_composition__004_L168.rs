// filename: src/main.rs

// pipe: value |> f1 |> f2 |> f3 (giống Elixir/F#)
fn pipe<T>(value: T, steps: &[&dyn Fn(T) -> T]) -> T
where T: Copy
{
    steps.iter().fold(value, |acc, f| f(acc))
}

fn main() {
    let result = pipe(100_u32, &[
        &|x| x + 50,        // +50 = 150
        &|x| x * 2,         // ×2 = 300
        &|x| x - 20,        // -20 = 280
    ]);
    println!("pipe(100): {}", result);  // 280

    // Thực tế: price pipeline
    let final_price = pipe(35_000_u32, &[
        &|p| p * 90 / 100,      // discount 10%
        &|p| p + p * 8 / 100,   // add tax 8%
        &|p| (p / 1000) * 1000, // round to nearest 1000
    ]);
    println!("Final: {}đ", final_price);  // 34000đ
}
