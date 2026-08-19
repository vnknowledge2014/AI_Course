// filename: src/main.rs
fn main() {
    // Máy nhỏ 1: cộng 10
    let add_ten = |x: i32| x + 10;

    // Máy nhỏ 2: nhân đôi
    let double = |x: i32| x * 2;

    // "Máy lớn": nhận một máy nhỏ (f) và nguyên liệu (x)
    // Chạy máy nhỏ 2 lần liên tiếp
    let apply_twice = |f: &dyn Fn(i32) -> i32, x: i32| {
        let first = f(x);     // chạy máy lần 1
        let second = f(first); // lấy kết quả, chạy lần 2
        second
    };

    // Bỏ máy "cộng 10" vào, rồi bỏ số 5:
    // Lần 1: 5 + 10 = 15
    // Lần 2: 15 + 10 = 25
    let r1 = apply_twice(&add_ten, 5);
    println!("apply_twice(add_ten, 5) = {}", r1);
    assert_eq!(r1, 25);

    // Bỏ máy "nhân đôi" vào, rồi bỏ số 3:
    // Lần 1: 3 × 2 = 6
    // Lần 2: 6 × 2 = 12
    let r2 = apply_twice(&double, 3);
    println!("apply_twice(double, 3) = {}", r2);
    assert_eq!(r2, 12);

    // Output:
    // apply_twice(add_ten, 5) = 25
    // apply_twice(double, 3) = 12
}
