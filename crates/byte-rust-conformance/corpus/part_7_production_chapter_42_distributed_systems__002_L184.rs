// filename: src/main.rs

use std::collections::HashSet;

// ═══ Idempotent message handler ═══
struct IdempotentHandler {
    processed_ids: HashSet<String>,
}

impl IdempotentHandler {
    fn new() -> Self { IdempotentHandler { processed_ids: HashSet::new() } }

    fn handle(&mut self, message_id: &str, payload: &str) -> Result<String, String> {
        // Dedup: skip already-processed messages
        if self.processed_ids.contains(message_id) {
            println!("  [SKIP] Already processed: {}", message_id);
            return Ok("Already processed".into());
        }

        // Process
        println!("  [PROCESS] {}: {}", message_id, payload);
        let result = format!("Processed: {}", payload);

        // Mark as done
        self.processed_ids.insert(message_id.into());

        Ok(result)
    }
}

fn main() {
    let mut handler = IdempotentHandler::new();

    // Simulate at-least-once delivery (message delivered 3 times!)
    let messages = vec![
        ("msg-001", "Order #1"),
        ("msg-002", "Order #2"),
        ("msg-001", "Order #1"),  // duplicate!
        ("msg-003", "Order #3"),
        ("msg-002", "Order #2"),  // duplicate!
    ];

    for (id, payload) in messages {
        handler.handle(id, payload).unwrap();
    }
}
