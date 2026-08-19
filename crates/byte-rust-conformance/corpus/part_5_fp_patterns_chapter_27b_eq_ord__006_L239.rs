use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug)]
struct Player {
    name: String,
    score: u32,
    level: u32,
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        // So sánh tuple. 
        // Lấy other.score cmp self.score (để giảm dần)
        // Lấy self.level cmp other.level (để tăng dần)
        (other.score, self.level).cmp(&(self.score, other.level))
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut players = vec![
        Player { name: String::from("A"), score: 100, level: 5 },
        Player { name: String::from("B"), score: 100, level: 2 },
        Player { name: String::from("C"), score: 200, level: 10 },
    ];

    players.sort();
    
    // Kết quả: C (score 200) -> B (score 100, lvl 2) -> A (score 100, lvl 5)
}
