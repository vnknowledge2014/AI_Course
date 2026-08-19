// filename: src/main.rs

fn main() {
    let raw_input = "  Hello,   World!  This  IS    a   TEST.  ";

    // Pipeline: normalize text
    let normalized: String = raw_input
        .trim()                           // bỏ whitespace đầu/cuối
        .to_lowercase()                   // lowercase
        .split_whitespace()               // tách words (bỏ extra spaces)
        .collect::<Vec<_>>()
        .join(" ");                       // nối lại 1 space

    println!("Normalized: '{}'", normalized);
    // Output: Normalized: 'hello, world! this is a test.'

    // Pipeline: extract + transform
    let word_lengths: Vec<(String, usize)> = raw_input
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .map(|w| {
            let clean: String = w.chars().filter(|c| c.is_alphanumeric()).collect();
            let len = clean.len();
            (clean, len)
        })
        .filter(|(_, len)| *len > 2)     // chỉ words dài > 2
        .collect();

    println!("Words: {:?}", word_lengths);
    // Output: Words: [("hello", 5), ("world", 5), ("this", 4), ("test", 4)]
}
