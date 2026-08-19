// filename: src/main.rs

// Nhận Fn — closure chỉ đọc, gọi nhiều lần
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))  // gọi f 2 lần
}

// Nhận FnMut — closure có thể sửa captured variables
fn call_many_times<F: FnMut()>(mut f: F, times: u32) {
    for _ in 0..times {
        f();
    }
}

// Nhận FnOnce — closure chỉ gọi 1 lần (có thể consume data)
fn call_once<F: FnOnce() -> String>(f: F) -> String {
    f()  // chỉ gọi 1 lần
    // f()  // ❌ gọi lần 2 → error: value used after move
}

fn main() {
    // Fn: closure chỉ đọc
    let double = |x: i32| x * 2;
    println!("apply_twice(double, 3) = {}", apply_twice(double, 3));
    // 3 → 6 → 12

    // FnMut: closure sửa biến
    let mut total = 0;
    call_many_times(|| {
        total += 10;
    }, 5);
    println!("Total: {}", total);  // 50

    // FnOnce: closure consume data
    let data = vec![1, 2, 3];
    let result = call_once(move || {
        format!("Data: {:?}", data)  // data bị consumed
    });
    println!("{}", result);

    // Output:
    // apply_twice(double, 3) = 12
    // Total: 50
    // Data: [1, 2, 3]
}
