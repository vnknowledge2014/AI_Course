// filename: src/main.rs

fn api_pipeline(raw_body: &str) -> Result<String, String> {
    parse_json(raw_body)
        .and_then(validate_fields)
        .and_then(process_order)
        .map(format_response)
}

fn parse_json(body: &str) -> Result<Vec<(&str, &str)>, String> {
    if body.starts_with('{') {
        Ok(vec![("name", "Coffee"), ("qty", "2")])
    } else {
        Err("Invalid JSON".into())
    }
}

fn validate_fields(fields: Vec<(&str, &str)>) -> Result<(String, u32), String> {
    let name = fields.iter().find(|(k, _)| *k == "name")
        .map(|(_, v)| v.to_string())
        .ok_or("Missing 'name'")?;
    let qty = fields.iter().find(|(k, _)| *k == "qty")
        .and_then(|(_, v)| v.parse().ok())
        .ok_or("Missing/invalid 'qty'")?;
    Ok((name, qty))
}

fn process_order((name, qty): (String, u32)) -> Result<String, String> {
    if qty == 0 { Err("Quantity must be > 0".into()) }
    else { Ok(format!("Order: {} x{}", name, qty)) }
}

fn format_response(order: String) -> String {
    format!("{{\"status\": \"ok\", \"order\": \"{}\"}}", order)
}

fn main() {
    println!("{:?}", api_pipeline("{body}"));
    println!("{:?}", api_pipeline("not json"));
}
