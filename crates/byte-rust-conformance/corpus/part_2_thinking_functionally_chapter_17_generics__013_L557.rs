// filename: src/main.rs

// HRTB: "function hoạt động cho MỌI lifetime 'a"
// for<'a> Fn(&'a str) -> &'a str

fn apply_transform<F>(items: &[String], transform: F) -> Vec<String>
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    items.iter().map(|s| transform(s).to_string()).collect()
}

fn main() {
    let items = vec!["  hello  ".into(), "  world  ".into()];
    let trimmed = apply_transform(&items, |s| s.trim());
    println!("{:?}", trimmed);  // ["hello", "world"]
}
