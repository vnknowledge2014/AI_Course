// filename: src/main.rs
fn main() {
    // ✅ OK khi bạn CHẮC CHẮN sẽ Ok/Some
    let x: i32 = "42".parse().unwrap();  // luôn parse được
    println!("x = {}", x);

    // ❌ NGUY HIỂM — panic nếu file không tồn tại
    // let content = std::fs::read_to_string("maybe.txt").unwrap();
    // → panic: called `Result::unwrap()` on an `Err` value

    // ✅ expect — giống unwrap nhưng có message giải thích
    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("PORT must be a valid number");
    println!("port = {}", port);
}
