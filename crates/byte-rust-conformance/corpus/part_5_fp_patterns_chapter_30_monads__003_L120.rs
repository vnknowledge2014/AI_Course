// filename: src/main.rs

#[derive(Debug)]
enum AppError {
    Parse(String),
    Validation(String),
    Database(String),
}

fn parse_config(input: &str) -> Result<(String, u16), AppError> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        return Err(AppError::Parse("Expected host:port".into()));
    }
    let port = parts[1].parse::<u16>()
        .map_err(|_| AppError::Parse(format!("Invalid port: {}", parts[1])))?;
    Ok((parts[0].to_string(), port))
}

fn validate_config(host: &str, port: u16) -> Result<(String, u16), AppError> {
    if host.is_empty() {
        Err(AppError::Validation("Host empty".into()))
    } else if port < 1024 {
        Err(AppError::Validation(format!("Port {} < 1024 (privileged)", port)))
    } else {
        Ok((host.to_string(), port))
    }
}

fn connect(host: &str, port: u16) -> Result<String, AppError> {
    if host == "localhost" {
        Ok(format!("Connected to {}:{}", host, port))
    } else {
        Err(AppError::Database(format!("Cannot reach {}:{}", host, port)))
    }
}

fn main() {
    // and_then chain = Either Monad
    let result = parse_config("localhost:8080")
        .and_then(|(host, port)| validate_config(&host, port))
        .and_then(|(host, port)| connect(&host, port));
    println!("OK: {:?}", result);

    // ? operator = and_then sugar
    fn connect_from(input: &str) -> Result<String, AppError> {
        let (host, port) = parse_config(input)?;           // and_then
        let (host, port) = validate_config(&host, port)?;  // and_then
        let conn = connect(&host, port)?;                   // and_then
        Ok(conn)
    }

    println!("?:  {:?}", connect_from("localhost:8080"));
    println!("Err: {:?}", connect_from("remote:80"));
}
