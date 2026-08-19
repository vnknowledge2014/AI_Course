// filename: src/main.rs

use std::collections::HashMap;

fn get_database_url(config: &HashMap<String, String>) -> Option<String> {
    config.get("database")
        .and_then(|db_section| {
            // In real app: parse db section
            if db_section.contains("postgres") { Some(db_section.clone()) }
            else { None }
        })
        .map(|url| format!("postgresql://{}", url))
}

fn main() {
    let mut config = HashMap::new();
    config.insert("database".into(), "postgres://localhost:5432/mydb".into());
    println!("{:?}", get_database_url(&config));

    config.insert("database".into(), "sqlite://local.db".into());
    println!("{:?}", get_database_url(&config));
}
