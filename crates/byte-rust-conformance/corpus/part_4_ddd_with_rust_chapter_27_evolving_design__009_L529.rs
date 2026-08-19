struct Config {
    host: String,
    port: u16,
    max_connections: u32,
    timeout_ms: u64,
    tls_enabled: bool,
}

impl Config {
    // Old API vẫn hoạt động!
    fn new(host: &str, port: u16) -> Self {
        Config {
            host: host.into(), port,
            max_connections: 100,  // sensible default
            timeout_ms: 5000,     // sensible default
            tls_enabled: false,   // sensible default
        }
    }

    // Builder for new fields
    fn with_max_connections(mut self, n: u32) -> Self { self.max_connections = n; self }
    fn with_timeout(mut self, ms: u64) -> Self { self.timeout_ms = ms; self }
    fn with_tls(mut self) -> Self { self.tls_enabled = true; self }
}

// Old code: Config::new("localhost", 8080)  ← works!
// New code: Config::new("localhost", 8080).with_tls().with_timeout(3000)

fn main() {}
