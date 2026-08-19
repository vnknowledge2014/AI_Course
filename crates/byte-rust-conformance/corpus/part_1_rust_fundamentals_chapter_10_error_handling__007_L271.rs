// filename: src/main.rs
use std::collections::HashMap;

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
}

fn parse_config(input: &str) -> Result<Config, String> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid line: '{}'", line));
        }
        map.insert(parts[0].trim(), parts[1].trim());
    }

    let host = map.get("host")
        .ok_or("Missing 'host'")?   // Option → Result via ok_or
        .to_string();

    let port_str = map.get("port")
        .ok_or("Missing 'port'")?;

    let port: u16 = port_str.parse()
        .map_err(|e| format!("Invalid port: {}", e))?;  // đổi error type

    Ok(Config { host, port })
}

fn main() {
    let valid = "host = localhost\nport = 8080";
    println!("{:?}", parse_config(valid));
    // Ok(Config { host: "localhost", port: 8080 })

    let missing = "host = localhost";
    println!("{:?}", parse_config(missing));
    // Err("Missing 'port'")

    let bad_port = "host = localhost\nport = abc";
    println!("{:?}", parse_config(bad_port));
    // Err("Invalid port: invalid digit found in string")
}
