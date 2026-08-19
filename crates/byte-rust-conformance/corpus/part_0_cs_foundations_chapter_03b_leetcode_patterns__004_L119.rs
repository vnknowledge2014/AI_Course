pub fn fib_tab(n: usize) -> u64 {
    if n == 0 { return 0; }
    let mut dp = vec![0; n + 1];
    dp[1] = 1;

    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}

fn main() {}
