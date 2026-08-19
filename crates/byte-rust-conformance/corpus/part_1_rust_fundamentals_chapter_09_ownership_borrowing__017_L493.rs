// filename: src/main.rs
fn main() {
    let borrowed: &str = "hello";

    let owned1: String = borrowed.to_string();    // &str → String
    let owned2: String = borrowed.to_owned();     // tương tự
    let owned3: String = String::from(borrowed);  // tương tự

    println!("{} {} {}", owned1, owned2, owned3);
}
