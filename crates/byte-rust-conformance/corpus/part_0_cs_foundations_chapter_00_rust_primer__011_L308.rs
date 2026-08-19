fn main() {
    // Định nghĩa một closure nhận tham số x và trả về x + 1
    let add_one = |x: i32| x + 1;
    let multiply = |a: i32, b: i32| a * b;

    println!("add_one(5) = {}", add_one(5));       // Kết quả: 6
    println!("multiply(3, 4) = {}", multiply(3, 4)); // Kết quả: 12

    // Closure đặc biệt tỏa sáng khi làm việc với các collection (tập hợp)
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    // Output: Doubled: [2, 4, 6, 8, 10]
}
