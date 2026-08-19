// filename: src/main.rs

// "Kết quả sống ít nhất lâu bằng x VÀ y"
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let name1 = String::from("Rust Programming");
    let result;

    {
        let name2 = String::from("Go");
        result = longer(&name1, &name2);
        println!("Longer: {}", result);  // ✅ cả name1 và name2 còn sống
    }
    // println!("{}", result);  // ❌ name2 đã bị drop — result có thể trỏ vào nó

    // Output: Longer: Rust Programming
}
