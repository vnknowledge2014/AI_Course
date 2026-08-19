// filename: src/main.rs

// Unit struct: zero-size, chỉ đánh dấu type
struct Authenticated;
struct Guest;

// Dùng làm "phantom type" — chỉ tồn tại lúc compile
fn admin_panel(_token: &Authenticated) -> &str {
    "Welcome to admin panel"
}

fn main() {
    let token = Authenticated;
    println!("{}", admin_panel(&token));

    // Guest không thể gọi admin_panel!
    // admin_panel(&Guest);  // ❌ mismatched types
}
