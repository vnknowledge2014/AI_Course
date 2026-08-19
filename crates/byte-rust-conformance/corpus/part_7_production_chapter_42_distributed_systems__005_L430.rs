// filename: src/main.rs

use std::thread;
use std::time::Duration;

fn retry_with_backoff<T>(
    max_attempts: u32,
    base_delay_ms: u64,
    operation: impl Fn(u32) -> Result<T, String>,
) -> Result<T, String> {
    for attempt in 1..=max_attempts {
        match operation(attempt) {
            Ok(result) => return Ok(result),
            Err(e) if attempt == max_attempts => return Err(e),
            Err(e) => {
                let delay = base_delay_ms * 2u64.pow(attempt - 1);
                // Add jitter: ±25%
                let jitter = delay / 4;
                println!("  Attempt {} failed: {}. Retry in {}ms", attempt, e, delay);
                thread::sleep(Duration::from_millis(delay));
            }
        }
    }
    Err("Exhausted retries".into())
}

fn main() {
    let result = retry_with_backoff(4, 100, |attempt| {
        if attempt < 3 {
            Err(format!("Timeout on attempt {}", attempt))
        } else {
            Ok("Connected!")
        }
    });
    println!("Final: {:?}", result);
}
