// filename: src/main.rs
fn main() {
    let names = vec!["Minh", "Lan", "Hùng"];
    let scores = vec![85, 92, 78];

    // zip: ghép 2 iterators thành tuples
    let report: Vec<String> = names.iter()
        .zip(scores.iter())
        .enumerate()
        .map(|(i, (name, score))| {
            let rank = if *score >= 90 { "🌟" } else { "  " };
            format!("#{} {} {}: {}pts", i + 1, rank, name, score)
        })
        .collect();

    for line in &report {
        println!("{}", line);
    }
    // #1    Minh: 85pts
    // #2 🌟 Lan: 92pts
    // #3    Hùng: 78pts
}
