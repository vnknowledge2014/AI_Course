fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // a) Chẵn × 2
    let a: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).map(|&x| x * 2).collect();
    println!("a: {:?}", a);  // [4, 8, 12, 16, 20]

    // b) Tổng lẻ
    let b: i32 = v.iter().filter(|&&x| x % 2 != 0).sum();
    println!("b: {}", b);  // 25

    // c) Đầu tiên > 7
    let c = v.iter().find(|&&x| x > 7);
    println!("c: {:?}", c);  // Some(8)
}
