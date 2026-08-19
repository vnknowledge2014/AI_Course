// filename: src/main.rs
fn main() {
    // Stack: kích thước cố định, nhanh
    let x: i32 = 42;        // 4 bytes trên stack
    let y: f64 = 3.14;      // 8 bytes trên stack
    let z: bool = true;     // 1 byte trên stack
    let arr = [1, 2, 3];    // 12 bytes trên stack (3 × i32)

    // Heap: kích thước động, chậm hơn
    let name = String::from("Hello");  // data trên heap, pointer trên stack
    let numbers = vec![1, 2, 3, 4];    // data trên heap, metadata trên stack

    println!("Stack: x={}, y={}, z={}, arr={:?}", x, y, z, arr);
    println!("Heap: name={}, numbers={:?}", name, numbers);
}
