// filename: src/main.rs
fn main() {
    // Tạo "cái máy xay" — nhận x, trả ra x + 1
    let add_one = |x: i32| x + 1;

    // Bỏ số 5 vào máy
    let result = add_one(5);

    println!("add_one(5) = {}", result);
    // Output: add_one(5) = 6

    assert_eq!(result, 6);
}
