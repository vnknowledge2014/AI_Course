// filename: src/main.rs
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct LogEntry {
    date: String,
    level: String,
    message: String,
}

fn parse_log(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.splitn(3, ' ').collect();
    if parts.len() < 3 {
        return None;
    }
    Some(LogEntry {
        date: parts[0].to_string(),
        level: parts[1].to_string(),
        message: parts[2].to_string(),
    })
}

fn main() {
    let logs = vec![
        "2024-01-15 ERROR Database connection failed",
        "2024-01-15 INFO Server started",
        "2024-01-16 WARN Memory usage high",
        "2024-01-16 ERROR Payment timeout",
        "2024-01-16 INFO User logged in",
        "2024-01-17 WARN Disk space low",
        "2024-01-17 ERROR API rate limited",
    ];

    // Pipeline: parse → filter → group
    let important: Vec<LogEntry> = logs.iter()
        .filter_map(|line| parse_log(line))
        .filter(|entry| entry.level == "ERROR" || entry.level == "WARN")
        .collect();

    // Group by level
    let mut grouped: HashMap<String, Vec<&LogEntry>> = HashMap::new();
    for entry in &important {
        grouped.entry(entry.level.clone())
            .or_default()
            .push(entry);
    }

    // Format summary
    println!("📊 Log Summary ({} important entries):", important.len());
    for (level, entries) in &grouped {
        println!("\n  {} ({}):", level, entries.len());
        for e in entries {
            println!("    {} — {}", e.date, e.message);
        }
    }
}
