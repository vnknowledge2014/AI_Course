// filename: src/main.rs
fn main() {
    // loop + break — loop cũng là expression!
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // break VỚI giá trị → loop trả về 20
        }
    };
    println!("Result: {}", result);
    // Output: Result: 20
}
