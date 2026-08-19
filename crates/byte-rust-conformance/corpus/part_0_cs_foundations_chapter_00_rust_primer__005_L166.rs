fn main() {
    let score = 85;

    // Thay vì tạo biến `grade` rồi dùng if/else để gán lại, ta gán thẳng kết quả của if/else vào biến
    let grade = if score >= 90 {
        "A"
    } else if score >= 70 {
        "B"
    } else {
        "C"
    };

    println!("Score {}: grade {}", score, grade);
    // Output: Score 85: grade B
}
