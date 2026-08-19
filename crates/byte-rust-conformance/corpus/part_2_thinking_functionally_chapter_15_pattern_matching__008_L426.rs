// filename: src/main.rs

fn process_config(map: &std::collections::HashMap<String, String>) -> Result<String, String> {
    // let-else: match → tiếp tục, else → return/break/continue
    let Some(host) = map.get("host") else {
        return Err("Missing 'host'".into());
    };

    let Some(port_str) = map.get("port") else {
        return Err("Missing 'port'".into());
    };

    let Ok(port) = port_str.parse::<u16>() else {
        return Err(format!("Invalid port: {}", port_str));
    };

    // Tại đây: host và port chắc chắn valid
    Ok(format!("{}:{}", host, port))
}

fn main() {
    use std::collections::HashMap;

    let mut config = HashMap::new();
    config.insert("host".into(), "localhost".into());
    config.insert("port".into(), "8080".into());

    println!("{:?}", process_config(&config));
    // Ok("localhost:8080")

    config.remove("port");
    println!("{:?}", process_config(&config));
    // Err("Missing 'port'")
}
