// Cho struct Player
#[derive(Eq, PartialEq)]
struct Player {
    name: String,
    score: u32,
    level: u32,
}

// YÊU CẦU:
// Hãy implement `Ord` và `PartialOrd` cho Player sao cho:
// - So sánh Score (Điểm cao hơn lên trước / giảm dần).
// - Nếu trùng Score, so sánh Level (Level thấp hơn lên trước / tăng dần).
//
// Gợi ý: bạn có thể dùng `cmp` trên tuple. Ví dụ:
// `(other.score, self.level).cmp(&(self.score, other.level))`

fn main() {}
