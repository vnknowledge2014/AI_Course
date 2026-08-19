// filename: src/main.rs

trait Printable {
    // Method BẮT BUỘC implement
    fn content(&self) -> String;

    // Methods MẶC ĐỊNH — có sẵn, override được
    fn print(&self) {
        println!("{}", self.content());
    }

    fn print_bordered(&self) {
        let content = self.content();
        let border = "─".repeat(content.len().min(50) + 4);
        println!("┌{}┐", border);
        println!("│  {}  │", content);
        println!("└{}┘", border);
    }

    fn is_empty(&self) -> bool {
        self.content().is_empty()
    }
}

struct Note {
    text: String,
}

impl Printable for Note {
    // Chỉ implement content() — print() và print_bordered() dùng mặc định
    fn content(&self) -> String {
        self.text.clone()
    }
}

struct ImportantNote {
    text: String,
}

impl Printable for ImportantNote {
    fn content(&self) -> String {
        self.text.clone()
    }

    // Override default method
    fn print(&self) {
        println!("⚠️ IMPORTANT: {}", self.content());
    }
}

fn main() {
    let note = Note { text: "Remember to buy milk".into() };
    note.print();            // dùng default
    note.print_bordered();   // dùng default

    let urgent = ImportantNote { text: "Server is down!".into() };
    urgent.print();          // dùng override
    urgent.print_bordered(); // dùng default (không override)
}
