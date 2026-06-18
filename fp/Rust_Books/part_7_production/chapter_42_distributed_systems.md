# Chapter 42 — Distributed Systems Fundamentals

> **Bạn sẽ học được**:
> - **CAP Theorem**: Consistency, Availability, Partition tolerance
> - **Consistency models**: strong, eventual, causal
> - **Replication**: leader-follower, multi-leader, leaderless
> - **Consensus**: Raft basics (leader election, log replication)
> - **Message Queues**: delivery guarantees, patterns
> - **Patterns**: Saga, Circuit Breaker, Retry, Outbox
> - **Observability**: logging, metrics, tracing
>
> **Yêu cầu trước**: Chapter 36 (Concurrency), Chapter 38 (Database).
> **Thời gian đọc**: ~45 phút | **Level**: Principal
> **Kết quả cuối cùng**: Bạn hiểu **trade-offs** khi build systems chạy trên nhiều machines.

---

## Distributed Systems — Thách thức và Patterns

Khi ứng dụng vượt quá một server, mọi thứ thay đổi. Network failures, partial failures, eventual consistency, split-brain — đây là thế giới mới với quy luật riêng.

Chapter này không dạy bạn xây distributed system từ đầu (đó là công việc của chuyên gia). Thay vào đó, nó trang bị cho bạn **mental models** để hiểu trade-offs (CAP theorem), **patterns** để xử lý failures (retry, circuit breaker, saga), và **tools** trong Rust ecosystem để implement chúng.

---

## 42.1 — CAP Theorem

Khi ứng dụng chạy trên 1 máy, mọi thứ đơn giản: đọc/ghi cùng 1 database, không lo mất kết nối. Nhưng khi scale ra nhiều máy, bạn phải chọn: ưu tiên dữ liệu luôn đúng (Consistency), luôn phản hồi (Availability), hay chịu được mạng đứt (Partition Tolerance)? CAP nói bạn chỉ được 2/3 — và vì mạng luôn có thể đứt, thực tế bạn chọn giữa CP và AP.

### "Pick 2 out of 3" (nhưng thực tế phức tạp hơn)

```
         Consistency
            ╱╲
           ╱  ╲
          ╱ CP ╲        CP = Consistent + Partition-tolerant
         ╱      ╲           (sacrifice Availability)
        ╱────────╲          Example: Bank transfers, HBase
       ╱          ╲
      ╱     CA     ╲    CA = Consistent + Available
     ╱              ╲       (no network partitions allowed)
    ╱────────────────╲      Example: Single-node PostgreSQL
   ╱        AP        ╲
  ╱                     ╲ AP = Available + Partition-tolerant
 ╱                       ╲    (sacrifice Consistency)
╱─────────────────────────╲   Example: DNS, Cassandra
  Availability    Partition
                  Tolerance
```

| | Nhất quán (Consistency) | Sẵn sàng (Availability) | Chịu phân vùng (Partition Tolerance) |
|---|---|---|---|
| **CP** | ✅ Mọi nơi thấy cùng data | ❌ Có thể từ chối request | ✅ Sống sót khi mạng đứt |
| **AP** | ❌ Có thể thấy data cũ | ✅ Luôn phản hồi | ✅ Sống sót khi mạng đứt |
| **CA** | ✅ | ✅ | ❌ Chỉ 1 node |

### Lựa chọn thực tế

| Hệ thống | Loại | Lý do |
|--------|------|-----|
| **Chuyển tiền** | CP | Tiền PHẢI nhất quán |
| **Mạng xã hội** | AP | Post cũ OK, luôn sẵn sàng |
| **Giỏ hàng** | AP | Merge sau, không chặn user |
| **Tồn kho** | CP | Không bán quá số lượng! |
| **DNS** | AP | Delay lan truyền chấp nhận được |

---

## 42.2 — Consistency Models

CP hay AP là lựa chọn nhị phân, nhưng thực tế có phổ rộng hơn. Strong consistency (đọc luôn thấy giá trị mới nhất) đắt tiền nhưng an toàn. Eventual consistency (các node sẽ hội tụ dần) rẻ và nhanh. Code dưới mô phỏng 2 replicas và quá trình sync:

```
Strong ────────────────────────────────────── Eventual
(expensive, slow)                           (cheap, fast)

   Strong           Causal          Eventual
   │                │               │
   │ Read always    │ Reads respect │ Eventually
   │ sees latest    │ causal order  │ all nodes
   │ write          │               │ converge
   │                │               │
   └── PostgreSQL   └── CRDTs      └── DynamoDB
       (single)         Riak           Cassandra
```

### Eventual consistency in practice

```rust
// filename: src/main.rs

use std::collections::HashMap;
use std::time::{Duration, Instant};

// Simulated eventually-consistent replicas
#[derive(Debug, Clone)]
struct Replica {
    name: String,
    data: HashMap<String, (String, Instant)>, // key → (value, timestamp)
}

impl Replica {
    fn new(name: &str) -> Self {
        Replica { name: name.into(), data: HashMap::new() }
    }

    fn write(&mut self, key: &str, value: &str) {
        self.data.insert(key.into(), (value.into(), Instant::now()));
        println!("  [{}] WRITE: {} = {}", self.name, key, value);
    }

    fn read(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|(v, _)| v.as_str())
    }

    // Sync: Last-Write-Wins (LWW)
    fn sync_from(&mut self, other: &Replica) {
        for (key, (value, timestamp)) in &other.data {
            match self.data.get(key) {
                Some((_, my_ts)) if my_ts >= timestamp => {} // mine is newer
                _ => {
                    self.data.insert(key.clone(), (value.clone(), *timestamp));
                    println!("  [{}] SYNC: {} = {} (from {})", self.name, key, value, other.name);
                }
            }
        }
    }
}

fn main() {
    let mut node_a = Replica::new("Node-A");
    let mut node_b = Replica::new("Node-B");

    // Write to different nodes
    node_a.write("user:1", "Minh");
    node_b.write("user:2", "Lan");

    // Before sync: each node sees different data
    println!("\nBefore sync:");
    println!("  A sees user:2 = {:?}", node_a.read("user:2")); // None
    println!("  B sees user:1 = {:?}", node_b.read("user:1")); // None

    // Sync (eventual consistency)
    println!("\nSyncing...");
    let a_clone = node_a.clone();
    let b_clone = node_b.clone();
    node_a.sync_from(&b_clone);
    node_b.sync_from(&a_clone);

    // After sync: eventual consistency achieved
    println!("\nAfter sync:");
    println!("  A sees user:2 = {:?}", node_a.read("user:2")); // Some("Lan")
    println!("  B sees user:1 = {:?}", node_b.read("user:1")); // Some("Minh")
}
```

---

## 42.3 — Message Queues & Delivery Guarantees

Khi 2 services cần giao tiếp, gọi trực tiếp (HTTP) đơn giản nhưng dễ vỡ nếu service kia chậm hoặc down. Message queue đứng giữa: producer gửi vào queue, consumer lấy ra xử lý. Nếu consumer chết, message vẫn còn trong queue.

Nhưng có câu hỏi: message được gửi đúng bao nhiêu lần?

### Point-to-Point vs Pub/Sub

```
Point-to-Point:                  Pub/Sub:
  Producer → Queue → Consumer     Producer → Topic ─┬→ Consumer A
                                                    ├→ Consumer B
                                                    └→ Consumer C
```

### Delivery guarantees

| Đảm bảo | Ý nghĩa | Dùng khi |
|-----------|---------|----------|
| **Tối đa một lần** (At-most-once) | Có thể mất tin nhắn | Metrics, logs (mất được) |
| **Ít nhất một lần** (At-least-once) | Có thể trùng lặp | Đơn hàng (handler idempotent!) |
| **Đúng một lần** (Exactly-once) | Giao nhận hoàn hảo | Chuyển tiền (khó + đắt) |

### Idempotent handler (for at-least-once)

```rust
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
```

---

## 42.4 — Patterns: Saga, Circuit Breaker, Retry

Khi lỗi xảy ra trong hệ thống phân tán, không có ROLLBACK giống SQL. 3 patterns dưới giúp bạn xử lý: **Saga** (đảo ngược các bước đã làm khi có lỗi), **Circuit Breaker** (ngừng gọi service đang chết), và **Retry** (thử lại với delay tăng dần).

### Saga pattern (distributed transactions)

```
Order Saga (choreography):
  1. OrderService: CreateOrder ──── event ────▶
  2. PaymentService: ChargePayment ──── event ────▶
  3. InventoryService: ReserveStock ──── event ────▶
  4. ShippingService: ScheduleShipment

  If step 3 fails:
  3'. InventoryService: (compensate: nothing to undo)
  2'. PaymentService: RefundPayment ◀──── compensating event
  1'. OrderService: CancelOrder ◀──── compensating event
```

```rust
// filename: src/main.rs

// ═══ Saga: sequence of steps with compensations ═══
type StepFn = Box<dyn Fn() -> Result<String, String>>;
type CompensateFn = Box<dyn Fn()>;

struct SagaStep {
    name: String,
    execute: StepFn,
    compensate: CompensateFn,
}

struct Saga {
    steps: Vec<SagaStep>,
}

impl Saga {
    fn new() -> Self { Saga { steps: vec![] } }

    fn add_step(mut self, name: &str, execute: StepFn, compensate: CompensateFn) -> Self {
        self.steps.push(SagaStep { name: name.into(), execute, compensate });
        self
    }

    fn execute(&self) -> Result<Vec<String>, String> {
        let mut results = vec![];
        let mut completed = vec![];

        for (i, step) in self.steps.iter().enumerate() {
            print!("  Step {}: {}... ", i + 1, step.name);
            match (step.execute)() {
                Ok(result) => {
                    println!("✅ {}", result);
                    results.push(result);
                    completed.push(i);
                }
                Err(e) => {
                    println!("❌ {}", e);
                    // Compensate all completed steps (reverse order)
                    println!("  Rolling back...");
                    for &idx in completed.iter().rev() {
                        println!("    Compensate: {}", self.steps[idx].name);
                        (self.steps[idx].compensate)();
                    }
                    return Err(e);
                }
            }
        }

        Ok(results)
    }
}

fn main() {
    // Happy path saga
    println!("=== Order Saga (Success) ===");
    let saga = Saga::new()
        .add_step("Create Order",
            Box::new(|| Ok("ORD-001 created".into())),
            Box::new(|| println!("      ↩ Cancel order")))
        .add_step("Charge Payment",
            Box::new(|| Ok("500,000đ charged".into())),
            Box::new(|| println!("      ↩ Refund payment")))
        .add_step("Reserve Stock",
            Box::new(|| Ok("5 items reserved".into())),
            Box::new(|| println!("      ↩ Release stock")));

    println!("{:?}\n", saga.execute());

    // Failure saga
    println!("=== Order Saga (Failure at Stock) ===");
    let failing_saga = Saga::new()
        .add_step("Create Order",
            Box::new(|| Ok("ORD-002 created".into())),
            Box::new(|| println!("      ↩ Cancel order")))
        .add_step("Charge Payment",
            Box::new(|| Ok("300,000đ charged".into())),
            Box::new(|| println!("      ↩ Refund payment")))
        .add_step("Reserve Stock",
            Box::new(|| Err("Out of stock!".into())),
            Box::new(|| println!("      ↩ Release stock")));

    println!("{:?}", failing_saga.execute());
}
```

### Circuit Breaker

```rust
// filename: src/main.rs

use std::time::{Duration, Instant};

#[derive(Debug, PartialEq)]
enum CircuitState { Closed, Open, HalfOpen }

struct CircuitBreaker {
    state: CircuitState,
    failure_count: u32,
    failure_threshold: u32,
    success_count: u32,
    success_threshold: u32,
    last_failure: Option<Instant>,
    timeout: Duration,
}

impl CircuitBreaker {
    fn new(failure_threshold: u32, success_threshold: u32, timeout_secs: u64) -> Self {
        CircuitBreaker {
            state: CircuitState::Closed,
            failure_count: 0, failure_threshold,
            success_count: 0, success_threshold,
            last_failure: None,
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    fn call<T>(&mut self, f: impl FnOnce() -> Result<T, String>) -> Result<T, String> {
        match self.state {
            CircuitState::Open => {
                if let Some(last) = self.last_failure {
                    if last.elapsed() > self.timeout {
                        self.state = CircuitState::HalfOpen;
                        self.success_count = 0;
                        println!("  [CB] Open → HalfOpen (trying...)");
                    } else {
                        return Err("Circuit OPEN: request blocked".into());
                    }
                }
            }
            _ => {}
        }

        match f() {
            Ok(result) => {
                if self.state == CircuitState::HalfOpen {
                    self.success_count += 1;
                    if self.success_count >= self.success_threshold {
                        self.state = CircuitState::Closed;
                        self.failure_count = 0;
                        println!("  [CB] HalfOpen → Closed (recovered!)");
                    }
                } else {
                    self.failure_count = 0;
                }
                Ok(result)
            }
            Err(e) => {
                self.failure_count += 1;
                self.last_failure = Some(Instant::now());
                if self.failure_count >= self.failure_threshold {
                    self.state = CircuitState::Open;
                    println!("  [CB] Closed → Open (too many failures!)");
                }
                Err(e)
            }
        }
    }
}

fn main() {
    let mut cb = CircuitBreaker::new(3, 2, 5);

    for i in 1..=7 {
        let result = cb.call(|| {
            if i <= 4 { Err("Service down".into()) }
            else { Ok(format!("Response {}", i)) }
        });
        println!("Request {}: {:?} (state: {:?})", i, result, cb.state);
    }
}
```

### Retry with exponential backoff

```rust
// filename: src/main.rs

use std::thread;
use std::time::Duration;

fn retry_with_backoff<T>(
    max_attempts: u32,
    base_delay_ms: u64,
    operation: impl Fn(u32) -> Result<T, String>,
) -> Result<T, String> {
    for attempt in 1..=max_attempts {
        match operation(attempt) {
            Ok(result) => return Ok(result),
            Err(e) if attempt == max_attempts => return Err(e),
            Err(e) => {
                let delay = base_delay_ms * 2u64.pow(attempt - 1);
                // Add jitter: ±25%
                let jitter = delay / 4;
                println!("  Attempt {} failed: {}. Retry in {}ms", attempt, e, delay);
                thread::sleep(Duration::from_millis(delay));
            }
        }
    }
    Err("Exhausted retries".into())
}

fn main() {
    let result = retry_with_backoff(4, 100, |attempt| {
        if attempt < 3 {
            Err(format!("Timeout on attempt {}", attempt))
        } else {
            Ok("Connected!")
        }
    });
    println!("Final: {:?}", result);
}
```

---

## 42.5 — Observability

Khi service gặp vấn đề lúc 3 giờ sáng, bạn cần biết đã xảy ra gì (logs), nhiều như thế nào (metrics), và đi qua đâu (traces). Ba trụ cột này là "mắt" của bạn trong production.

### Three pillars

```
┌───────────┐  ┌───────────┐  ┌───────────┐
│  Logging  │  │  Metrics  │  │  Tracing  │
│           │  │           │  │           │
│ What      │  │ How much  │  │ Where     │
│ happened  │  │ & how fast│  │ it went   │
│           │  │           │  │           │
│ tracing   │  │ prometheus│  │ opentel.  │
│ crate     │  │           │  │ jaeger    │
└───────────┘  └───────────┘  └───────────┘
```

### Structured logging (Rust: `tracing`)

```rust
// filename: src/main.rs

// Conceptual (tracing crate):
// use tracing::{info, warn, error, instrument};
//
// #[instrument(fields(order_id = %order_id))]
// async fn process_order(order_id: &str) -> Result<(), Error> {
//     info!("Starting order processing");
//     let payment = charge_payment(order_id).await;
//     match payment {
//         Ok(_) => info!("Payment successful"),
//         Err(e) => {
//             error!(error = %e, "Payment failed");
//             return Err(e);
//         }
//     }
//     info!("Order processed successfully");
//     Ok(())
// }

fn main() {
    println!("Observability stack:");
    println!("  Logging:  tracing crate (structured, spans)");
    println!("  Metrics:  prometheus (counters, histograms, gauges)");
    println!("  Tracing:  opentelemetry (distributed request tracing)");
    println!();
    println!("Key metrics to track:");
    println!("  - Request rate (QPS)");
    println!("  - Error rate (% of 5xx)");
    println!("  - Latency (p50, p95, p99)");
    println!("  - Saturation (CPU, memory, connections)");
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Phân loại CAP

Phân loại: (a) sổ cái ngân hàng, (b) cache hồ sơ người dùng, (c) tin nhắn chat thời gian thực.

<details><summary>✅ Lời giải</summary>

- (a) Sổ cái ngân hàng: **CP** — nhất quán là tối quan trọng, có thể tạm từ chối ghi
- (b) Cache hồ sơ: **AP** — avatar cũ chấp nhận được, luôn sẵn sàng
- (c) Tin nhắn chat: **AP** — giao sau chấp nhận được, không bao giờ chặn user

</details>

---

**Bài 2** (10 phút): Thiết kế Saga

Thiết kế saga cho "Đặt vé máy bay + Khách sạn":
1. Reserve flight
2. Reserve hotel
3. Charge payment

Liệt kê compensating actions cho mỗi bước khi thất bại.

<details><summary>✅ Lời giải Bài 2</summary>

```
Step 1: Reserve Flight     → Compensate: Cancel flight reservation
Step 2: Reserve Hotel      → Compensate: Cancel hotel reservation
Step 3: Charge Payment     → Compensate: Refund payment

Failure at step 3:
  3. Payment fails
  2'. Cancel hotel reservation
  1'. Cancel flight reservation

Failure at step 2:
  2. Hotel unavailable
  1'. Cancel flight reservation
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Bộ não phân tách" (Split brain) | Phân vùng mạng, 2 leaders | Consensus protocol (Raft), quorum |
| "Tin nhắn sai thứ tự" | Queue async sắp xếp lại | Sắp xếp theo partition (Kafka) |
| "Saga bị kẹt" | Compensation thất bại | Retry compensation, dead letter queue |
| "Circuit breaker nhấp nháy" | Threshold quá thấp | Tăng threshold + timeout |

---

## Tóm tắt

- ✅ **CAP**: Pick CP (bank) or AP (social). Partition tolerance = non-negotiable in distributed.
- ✅ **Consistency**: Strong (sync) → Causal (vector clocks) → Eventual (merge later).
- ✅ **Message queues**: At-least-once + idempotent handlers = practical choice.
- ✅ **Saga**: Distributed transaction via compensating actions. Choreography (events) or orchestration.
- ✅ **Circuit Breaker**: Closed→Open→HalfOpen. Prevent cascade failures.
- ✅ **Retry**: Exponential backoff + jitter. Never retry without delay.
- ✅ **Observability**: Logs (what) + Metrics (how much) + Traces (where).

## Tiếp theo

→ Chapter 43: **System Design Thinking** — capacity estimation, load balancing, caching layers, API design, microservices.
