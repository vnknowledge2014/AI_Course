// filename: src/main.rs

fn process_chunk(data: &[i32]) -> i32 {
    data.iter().filter(|&&x| x > 0).sum()
}

fn main() {
    let data = vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10];

    // Chia data thành chunks, xử lý song song
    let total: i32 = data.chunks(3)
        .map(|chunk| process_chunk(chunk))
        .sum();

    println!("Sum of positives: {}", total);  // 1+3+5+7+9 = 25
}
