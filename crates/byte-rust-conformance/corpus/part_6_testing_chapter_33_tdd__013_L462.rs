// Viết Test TRƯỚC!
#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn reverse_hello() { assert_eq!(reverse_string("hello"), "olleh"); }
    #[test] fn reverse_empty() { assert_eq!(reverse_string(""), ""); }
    #[test] fn reverse_single() { assert_eq!(reverse_string("a"), "a"); }
    #[test] fn reverse_unicode() { assert_eq!(reverse_string("xin chào"), "oàhc nix"); }
}

// Giờ mới viết Code
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {}
