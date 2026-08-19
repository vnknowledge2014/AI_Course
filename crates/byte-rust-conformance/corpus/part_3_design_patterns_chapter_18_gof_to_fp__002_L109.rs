// filename: src/main.rs

// Command = enum variant chứa data
#[derive(Debug, Clone)]
enum EditorCommand {
    InsertText { position: usize, text: String },
    DeleteRange { start: usize, end: usize },
    Replace { from: String, to: String },
    Undo,
}

#[derive(Debug, Clone)]
struct Document {
    content: String,
    history: Vec<EditorCommand>,
}

impl Document {
    fn new(content: &str) -> Self {
        Document { content: content.to_string(), history: vec![] }
    }

    // Execute: pattern match trên command
    fn execute(&self, cmd: EditorCommand) -> Self {
        let new_content = match &cmd {
            EditorCommand::InsertText { position, text } => {
                let mut s = self.content.clone();
                let pos = (*position).min(s.len());
                s.insert_str(pos, text);
                s
            }
            EditorCommand::DeleteRange { start, end } => {
                let mut s = self.content.clone();
                let end = (*end).min(s.len());
                let start = (*start).min(end);
                s.drain(start..end);
                s
            }
            EditorCommand::Replace { from, to } => {
                self.content.replace(from.as_str(), to.as_str())
            }
            EditorCommand::Undo => {
                // Simplified: chỉ print thông báo
                println!("  (Undo not fully implemented in this demo)");
                return self.clone();
            }
        };

        let mut new_history = self.history.clone();
        new_history.push(cmd);

        Document { content: new_content, history: new_history }
    }
}

fn main() {
    let doc = Document::new("Hello World");
    println!("Start: '{}'", doc.content);

    let doc = doc.execute(EditorCommand::InsertText {
        position: 5,
        text: ", Rust".into(),
    });
    println!("Insert: '{}'", doc.content);

    let doc = doc.execute(EditorCommand::Replace {
        from: "World".into(),
        to: "FP".into(),
    });
    println!("Replace: '{}'", doc.content);

    let doc = doc.execute(EditorCommand::DeleteRange { start: 0, end: 7 });
    println!("Delete: '{}'", doc.content);

    println!("\nHistory ({} commands):", doc.history.len());
    for (i, cmd) in doc.history.iter().enumerate() {
        println!("  {}. {:?}", i + 1, cmd);
    }
}
