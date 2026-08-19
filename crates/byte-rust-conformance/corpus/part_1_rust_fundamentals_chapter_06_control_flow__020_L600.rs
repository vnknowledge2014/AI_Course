// filename: src/main.rs

fn parse_command(input: &str) -> String {
    match input.trim() {
        "quit" | "exit" => "Goodbye!".to_string(),
        "help" => "Available: quit, help, version, greet NAME".to_string(),
        "version" => "v1.0.0".to_string(),
        cmd if cmd.starts_with("greet ") => {
            let name = &cmd[6..]; // sau "greet "
            format!("Hello, {}!", name)
        }
        other => format!("Unknown command: '{}'", other),
    }
}

fn main() {
    let commands = ["help", "version", "greet Rust", "quit", "dance"];
    for cmd in &commands {
        println!("> {} → {}", cmd, parse_command(cmd));
    }
    // Output:
    // > help → Available: quit, help, version, greet NAME
    // > version → v1.0.0
    // > greet Rust → Hello, Rust!
    // > quit → Goodbye!
    // > dance → Unknown command: 'dance'
}
