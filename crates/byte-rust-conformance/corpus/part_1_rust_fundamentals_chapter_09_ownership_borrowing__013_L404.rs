// filename: src/main.rs

// Struct chứa reference → cần lifetime annotation
#[derive(Debug)]
struct Highlight<'a> {
    text: &'a str,
    label: &'a str,
}

impl<'a> Highlight<'a> {
    fn display(&self) -> String {
        format!("[{}] {}", self.label, self.text)
    }
}

fn main() {
    let content = String::from("Ownership is Rust's most unique feature");
    let label = "Important";

    let highlight = Highlight {
        text: &content[0..9],  // "Ownership"
        label,
    };

    println!("{}", highlight.display());
    println!("{:?}", highlight);

    // Output:
    // [Important] Ownership
    // Highlight { text: "Ownership", label: "Important" }
}
