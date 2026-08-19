// filename: src/main.rs

use std::collections::HashMap;

// ═══ URL Shortener design ═══

const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn id_to_short(mut id: u64) -> String {
    if id == 0 { return "0".into(); }
    let mut result = vec![];
    while id > 0 {
        result.push(BASE62[(id % 62) as usize] as char);
        id /= 62;
    }
    result.into_iter().rev().collect()
}

struct UrlShortener {
    urls: HashMap<String, UrlEntry>,
    next_id: u64,
}

struct UrlEntry {
    long_url: String,
    short_code: String,
    clicks: u64,
}

impl UrlShortener {
    fn new() -> Self { UrlShortener { urls: HashMap::new(), next_id: 1_000_000 } }

    fn shorten(&mut self, long_url: &str) -> String {
        let code = id_to_short(self.next_id);
        self.next_id += 1;
        let short = format!("https://short.vn/{}", code);
        self.urls.insert(code.clone(), UrlEntry {
            long_url: long_url.into(), short_code: code, clicks: 0,
        });
        short
    }

    fn redirect(&mut self, code: &str) -> Option<&str> {
        self.urls.get_mut(code).map(|entry| {
            entry.clicks += 1;
            entry.long_url.as_str()
        })
    }

    fn stats(&self, code: &str) -> Option<u64> {
        self.urls.get(code).map(|e| e.clicks)
    }
}

fn main() {
    let mut shortener = UrlShortener::new();

    let short1 = shortener.shorten("https://example.com/very/long/path/to/page");
    let short2 = shortener.shorten("https://rust-lang.org/book/chapter42");

    println!("Short URLs:");
    println!("  {} → long URL", short1);
    println!("  {} → long URL", short2);

    // Simulate redirects
    let code = short1.split('/').last().unwrap();
    for _ in 0..5 { shortener.redirect(code); }
    println!("\nClicks on {}: {}", code, shortener.stats(code).unwrap());

    println!("\nArchitecture:");
    println!("  Write: Client → API → ID Generator → DB (write)");
    println!("  Read:  Client → CDN → Cache (Redis) → DB (fallback)");
    println!("  Cache hit ratio: ~99% (URLs rarely change)");
}
