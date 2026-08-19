#[derive(Debug, Clone)]
struct Request {
    path: String,
    headers: Vec<(String, String)>,
    body: String,
}

struct MiddlewareStack {
    middlewares: Vec<Box<dyn Fn(Request) -> Request>>,
}

impl MiddlewareStack {
    fn new() -> Self { MiddlewareStack { middlewares: vec![] } }

    fn use_middleware<F: Fn(Request) -> Request + 'static>(mut self, f: F) -> Self {
        self.middlewares.push(Box::new(f));
        self
    }

    fn handle(&self, req: Request) -> Request {
        self.middlewares.iter().fold(req, |r, mw| mw(r))
    }
}

fn main() {
    let stack = MiddlewareStack::new()
        .use_middleware(|mut req| {
            println!("[Logger] {} {}", "→", req.path);
            req
        })
        .use_middleware(|mut req| {
            req.headers.push(("X-Request-Id".into(), "abc123".into()));
            req
        })
        .use_middleware(|mut req| {
            if !req.headers.iter().any(|(k, _)| k == "Authorization") {
                req.headers.push(("X-Auth".into(), "anonymous".into()));
            }
            req
        });

    let req = Request {
        path: "/api/users".into(),
        headers: vec![],
        body: "{}".into(),
    };

    let processed = stack.handle(req);
    println!("Headers: {:?}", processed.headers);
}
