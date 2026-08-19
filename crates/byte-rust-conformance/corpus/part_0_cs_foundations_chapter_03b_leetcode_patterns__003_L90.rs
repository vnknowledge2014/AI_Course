use std::collections::HashMap;

// Top-Down DP (Memoization)
pub fn fib_memo(n: u32, memo: &mut HashMap<u32, u64>) -> u64 {
    // Base cases
    if n == 0 { return 0; }
    if n == 1 { return 1; }

    // Check if already computed
    if let Some(&ans) = memo.get(&n) {
        return ans;
    }

    // Compute and store
    let ans = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);
    memo.insert(n, ans);
    ans
}

// Cách gọi:
// let mut memo = HashMap::new();
// let result = fib_memo(50, &mut memo);

fn main() {}
