fn process(items: &[i32], transform: impl Fn(i32) -> i32) -> Vec<i32> {
    items.iter().map(|&x| transform(x)).collect()
}

fn main() {}
