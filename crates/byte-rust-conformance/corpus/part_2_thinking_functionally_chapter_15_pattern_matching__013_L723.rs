fn sum_options(a: Option<i32>, b: Option<i32>) -> i32 {
    match (a, b) {
        (Some(x), Some(y)) => x + y,
        (Some(x), None) | (None, Some(x)) => x,
        (None, None) => 0,
    }
}

fn main() {}
