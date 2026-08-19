// filename: src/main.rs
fn main() {
    let score = 75;

    let grade = match score {
        90..=100 => "A",
        80..=89  => "B",
        70..=79  => "C",
        60..=69  => "D",
        0..=59   => "F",
        _        => "Invalid",
    };

    println!("Score {}: {}", score, grade);
    // Output: Score 75: C
}
