// filename: src/main.rs

// compose(f, g) = |x| g(f(x))
fn compose<A, B, C>(
    f: impl Fn(A) -> B,
    g: impl Fn(B) -> C,
) -> impl Fn(A) -> C {
    move |x| g(f(x))
}

fn main() {
    let add_tax = |price: u32| (price as f64 * 1.08) as u32;
    let format_price = |price: u32| format!("{}đ", price);

    // Compose: add_tax → format_price
    let price_with_tax = compose(add_tax, format_price);
    println!("{}", price_with_tax(35_000));  // 37800đ
    println!("{}", price_with_tax(50_000));  // 54000đ
}
