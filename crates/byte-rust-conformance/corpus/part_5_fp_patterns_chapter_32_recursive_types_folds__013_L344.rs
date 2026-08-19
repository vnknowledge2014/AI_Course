// filename: src/main.rs

#[derive(Debug, Clone)]
enum FSEntry {
    File { name: String, size: u64 },
    Dir { name: String, children: Vec<FSEntry> },
}

fn main() {}
