// Thuật toán Token Bucket (Đơn giản hóa để minh họa)
struct TokenBucket {
    capacity: f64,
    tokens: f64,
    refill_rate_per_sec: f64,
    last_refill: std::time::Instant,
}

impl TokenBucket {
    fn allow_request(&mut self) -> bool {
        let now = std::time::Instant::now();
        let elapsed_secs = now.duration_since(self.last_refill).as_secs_f64();
        
        // Hồi phục token theo thời gian đã trôi qua
        self.tokens = (self.tokens + elapsed_secs * self.refill_rate_per_sec).min(self.capacity);
        self.last_refill = now;

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true // Đủ token -> Cho phép!
        } else {
            false // Rỗng xô -> Từ chối!
        }
    }
}

fn main() {}
