// filename: src/main.rs

#[derive(Debug)]
struct Request {
    auth_token: Option<String>,
    body: Option<String>,
    method: String,
}

fn handle_request(req: &Request) -> String {
    // Chain if let: mỗi điều kiện phải pass
    if let Some(token) = &req.auth_token {
        if let Some(body) = &req.body {
            if req.method == "POST" {
                return format!("✅ POST with auth={} body_len={}", token, body.len());
            }
        }
    }

    // Fallback
    format!("❌ Unauthorized or invalid request")
}

// Cleaner alternative: extract + match tuple
fn handle_request_v2(req: &Request) -> String {
    match (&req.auth_token, &req.body, req.method.as_str()) {
        (Some(token), Some(body), "POST") =>
            format!("✅ POST auth={} body_len={}", token, body.len()),
        (Some(_), _, method) =>
            format!("⚠️ {} — no body or wrong method", method),
        (None, _, _) =>
            "❌ Unauthorized".to_string(),
    }
}

fn main() {
    let req = Request {
        auth_token: Some("abc123".into()),
        body: Some("{\"key\": \"value\"}".into()),
        method: "POST".into(),
    };
    println!("{}", handle_request_v2(&req));

    let no_auth = Request { auth_token: None, body: None, method: "GET".into() };
    println!("{}", handle_request_v2(&no_auth));
}
