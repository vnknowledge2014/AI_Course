// filename: src/main.rs
fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];

    // Kiểm tra
    println!("Contains 5? {}", numbers.contains(&5));        // true
    println!("Is empty? {}", numbers.is_empty());            // false

    // Tìm kiếm
    println!("First: {:?}", numbers.first());                // Some(3)
    println!("Last: {:?}", numbers.last());                  // Some(6)
    println!("Position of 9: {:?}", numbers.iter().position(|&x| x == 9)); // Some(5)

    // Sort (cần mut)
    let mut sorted = numbers.clone();
    sorted.sort();
    println!("Sorted: {:?}", sorted);  // [1, 1, 2, 3, 4, 5, 6, 9]
    sorted.dedup();                    // loại bỏ duplicates liên tiếp
    println!("Dedup: {:?}", sorted);   // [1, 2, 3, 4, 5, 6, 9]

    // Chunks
    let chunks: Vec<&[i32]> = numbers.chunks(3).collect();
    println!("Chunks of 3: {:?}", chunks); // [[3,1,4], [1,5,9], [2,6]]

    // Windows (sliding window)
    let windows: Vec<&[i32]> = numbers.windows(2).collect();
    println!("Windows of 2: {:?}", windows);
    // [[3,1], [1,4], [4,1], [1,5], [5,9], [9,2], [2,6]]
}
