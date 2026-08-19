// filename: src/main.rs

#[derive(Debug)]
struct Student {
    name: String,
    scores: Vec<u32>,
}

impl Student {
    fn average(&self) -> f64 {
        let sum: u32 = self.scores.iter().sum();
        sum as f64 / self.scores.len() as f64
    }
}

fn main() {
    let students = vec![
        Student { name: "Minh".into(), scores: vec![8, 9, 7, 8, 9] },
        Student { name: "Lan".into(), scores: vec![5, 6, 4, 5, 6] },
        Student { name: "Hùng".into(), scores: vec![9, 10, 8, 9, 10] },
        Student { name: "Mai".into(), scores: vec![7, 7, 8, 7, 6] },
        Student { name: "Dũng".into(), scores: vec![3, 4, 5, 4, 3] },
    ];

    // Pipeline
    let mut honor_roll: Vec<(String, f64)> = students.iter()
        .map(|s| (s.name.clone(), s.average()))
        .filter(|(_, avg)| *avg >= 7.0)
        .collect();

    honor_roll.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("🏆 Honor Roll (avg ≥ 7.0):");
    for (i, (name, avg)) in honor_roll.iter().enumerate() {
        let medal = match i {
            0 => "🥇",
            1 => "🥈",
            2 => "🥉",
            _ => "  ",
        };
        println!("{} {}: {:.1}", medal, name, avg);
    }

    // Output:
    // 🏆 Honor Roll (avg ≥ 7.0):
    // 🥇 Hùng: 9.2
    // 🥈 Minh: 8.2
    // 🥉 Mai: 7.0
}
