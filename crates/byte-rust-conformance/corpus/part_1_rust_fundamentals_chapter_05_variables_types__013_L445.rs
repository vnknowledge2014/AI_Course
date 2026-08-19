// filename: src/main.rs
fn main() {
    // Range: start..end (exclusive end)
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();  // 0 1 2 3 4

    // Range inclusive: start..=end
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();  // 1 2 3 4 5

    // Range trong slicing
    let data = [10, 20, 30, 40, 50];
    println!("data[1..4] = {:?}", &data[1..4]);   // [20, 30, 40]
    println!("data[..3] = {:?}", &data[..3]);      // [10, 20, 30]
    println!("data[2..] = {:?}", &data[2..]);      // [30, 40, 50]
}
