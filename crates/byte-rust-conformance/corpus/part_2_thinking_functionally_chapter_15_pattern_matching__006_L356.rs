// filename: src/main.rs

#[derive(Debug)]
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
}

fn execute(cmd: &Command) {
    match cmd {
        // @ bắt toàn bộ variant VÀ destructure
        c @ Command::Move { x, y } if *x == 0 && *y == 0 => {
            println!("Ignoring zero move: {:?}", c);
        }
        Command::Move { x, y } => {
            println!("Moving to ({}, {})", x, y);
        }
        Command::Write(text) if text.is_empty() => {
            println!("Ignoring empty write");
        }
        Command::Write(text) => {
            println!("Writing: {}", text);
        }
        Command::Color(r, g, b) => {
            println!("Color: #{:02X}{:02X}{:02X}", r, g, b);
        }
        Command::Quit => println!("Quitting"),
    }
}

fn main() {
    let commands = vec![
        Command::Move { x: 0, y: 0 },
        Command::Move { x: 10, y: 20 },
        Command::Write("".into()),
        Command::Write("hello".into()),
        Command::Color(255, 128, 0),
        Command::Quit,
    ];

    for cmd in &commands {
        execute(cmd);
    }
}
