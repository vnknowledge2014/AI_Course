// filename: src/main.rs

// 'static = giá trị sống toàn bộ chương trình
// String literals là 'static:
fn get_greeting() -> &'static str {
    "Hello, world!"  // embedded trong binary → sống mãi
}

// T: 'static KHÔNG có nghĩa "T phải là reference"
// Mà: T KHÔNG chứa non-'static references
// → String: 'static ✅ (owned, no borrows)
// → &'a str: KHÔNG 'static (trừ khi 'a = 'static)
// → Vec<i32>: 'static ✅ (owned)

fn spawn_task<T: Send + 'static>(data: T) {
    // Thread mới cần 'static vì thread có thể sống lâu hơn caller
    std::thread::spawn(move || {
        println!("Task completed");
        drop(data);
    }).join().unwrap();
}

fn main() {
    println!("{}", get_greeting());

    // ✅ String is 'static (owned)
    spawn_task(String::from("hello"));

    // ✅ Vec is 'static (owned)
    spawn_task(vec![1, 2, 3]);

    // ❌ &str (reference) is NOT 'static unless literal
    // let s = String::from("hi");
    // spawn_task(&s);  // error: borrowed value does not live long enough
}
