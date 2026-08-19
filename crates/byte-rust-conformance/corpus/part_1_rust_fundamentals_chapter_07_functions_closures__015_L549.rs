// filename: src/main.rs

fn make_greeting(prefix: String) -> impl Fn(&str) -> String {
    move |name| format!("{}, {}!", prefix, name)
}

// 💡 Phiên bản nâng cao: dùng &str + lifetime (học thêm ở Chapter 9)
// fn make_greeting_ref(prefix: &str) -> impl Fn(&str) -> String + '_ {
//     move |name| format!("{}, {}!", prefix, name)
// }

fn main() {
    let hi = make_greeting("Hi".to_string());
    let hello = make_greeting("Hello".to_string());

    println!("{}", hi("Rust"));     // Hi, Rust!
    println!("{}", hello("World")); // Hello, World!
}
