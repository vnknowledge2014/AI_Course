struct Request {
    method: String,
    path: String,
}

fn route(req: &Request) -> String {
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/") => "🏠 Home page".into(),
        ("GET", "/users") => "👥 User list".into(),
        ("GET", path) if path.starts_with("/users/") => {
            let id = &path[7..];
            format!("👤 User profile: {}", id)
        }
        ("POST", "/users") => "➕ Create user".into(),
        ("DELETE", path) if path.starts_with("/users/") => {
            let id = &path[7..];
            format!("🗑️ Delete user: {}", id)
        }
        (method, path) => format!("❓ 404 {} {} Not Found", method, path),
    }
}

fn main() {
    let requests = vec![
        Request { method: "GET".into(), path: "/".into() },
        Request { method: "GET".into(), path: "/users".into() },
        Request { method: "GET".into(), path: "/users/42".into() },
        Request { method: "POST".into(), path: "/users".into() },
        Request { method: "GET".into(), path: "/unknown".into() },
    ];

    for req in &requests {
        println!("{} {} → {}", req.method, req.path, route(req));
    }
}
