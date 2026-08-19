// filename: src/main.rs

fn grade_report(scores: [f64; 5]) -> (f64, f64, f64, &'static str) {
    let sum: f64 = scores.iter().sum();
    let average = sum / scores.len() as f64;

    let min = scores.iter().cloned().reduce(f64::min).unwrap();
    let max = scores.iter().cloned().reduce(f64::max).unwrap();

    let rank = if average >= 8.0 {
        "Giỏi"
    } else if average >= 6.5 {
        "Khá"
    } else if average >= 5.0 {
        "Trung bình"
    } else {
        "Yếu"
    };

    (average, min, max, rank)
}

fn main() {
    let scores = [8.5, 7.0, 9.0, 6.5, 8.0];
    let (avg, min, max, rank) = grade_report(scores);

    println!("📊 Grade Report");
    println!("Scores: {:?}", scores);
    println!("Average: {:.1}", avg);
    println!("Min: {:.1}, Max: {:.1}", min, max);
    println!("Rank: {}", rank);
    // Output:
    // 📊 Grade Report
    // Scores: [8.5, 7.0, 9.0, 6.5, 8.0]
    // Average: 7.8
    // Min: 6.5, Max: 9.0
    // Rank: Khá
}
