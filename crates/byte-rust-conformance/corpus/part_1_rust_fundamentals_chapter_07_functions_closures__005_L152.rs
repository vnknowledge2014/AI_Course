// filename: src/main.rs
fn main() {
    // 1. Capture by shared reference (&T) — đọc nhưng không sửa
    let name = String::from("Rust");
    let greet = || println!("Hello, {}!", name);  // chỉ đọc name
    greet();
    greet();  // gọi nhiều lần OK
    println!("name still exists: {}", name);  // name vẫn dùng được

    // 2. Capture by mutable reference (&mut T) — đọc VÀ sửa
    let mut count = 0;
    let mut increment = || {
        count += 1;  // sửa count → capture &mut
        println!("Count: {}", count);
    };
    increment();  // Count: 1
    increment();  // Count: 2
    // println!("{}", count);  // ❌ không dùng count được khi increment còn sống
    drop(increment);  // giải phóng mutable borrow
    println!("Final count: {}", count);  // ✅ OK — 2

    // 3. Capture by value (move) — lấy luôn ownership
    let data = vec![1, 2, 3];
    let consume = move || {
        println!("Data: {:?}", data);  // data bị move vào closure
    };
    consume();
    // println!("{:?}", data);  // ❌ data đã bị move!
}
