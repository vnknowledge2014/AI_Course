fn a(x: i32) -> i32 { x * 2 }
fn b(items: &mut Vec<i32>) { items.push(1); }
fn c(name: &str) -> String { format!("Hi, {}", name) }
fn d() -> u64 { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() }
fn e(prices: &[u32]) -> u32 { prices.iter().sum() }

fn main() {}
