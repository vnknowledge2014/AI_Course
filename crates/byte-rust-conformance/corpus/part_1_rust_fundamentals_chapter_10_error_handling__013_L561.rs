// filename: src/main.rs
use std::collections::HashMap;

fn get_setting(config: &HashMap<&str, &str>, key: &str) -> Result<String, String> {
    config.get(key)
        .ok_or(format!("Missing setting: {}", key))?  // Option → Result
        .parse()
        .map_err(|e| format!("Invalid {}: {}", key, e))
}

fn main() {
    let mut config = HashMap::new();
    config.insert("port", "8080");
    config.insert("host", "localhost");

    println!("{:?}", get_setting(&config, "port")); // Ok("8080")
    println!("{:?}", get_setting(&config, "db"));   // Err("Missing setting: db")
}
