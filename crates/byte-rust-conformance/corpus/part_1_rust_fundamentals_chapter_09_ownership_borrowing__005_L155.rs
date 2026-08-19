// filename: src/main.rs
fn main() {
    // Copy types: tất cả scalar types
    let x: i32 = 42;
    let y = x;           // COPY — x vẫn valid
    println!("x={}, y={}", x, y);  // ✅

    let flag = true;
    let flag2 = flag;    // COPY
    println!("{} {}", flag, flag2); // ✅

    let point = (1.0, 2.0);  // tuple of Copy types → cũng Copy
    let point2 = point;
    println!("{:?} {:?}", point, point2); // ✅

    // NON-Copy: String, Vec, HashMap, bất kỳ type chứa heap data
    // let v1 = vec![1, 2, 3];
    // let v2 = v1;  // MOVE — v1 invalid
}
