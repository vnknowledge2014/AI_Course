# Chapter 36 — Concurrency & Async

> **Bạn sẽ học được**:
> - **`std::thread`** — spawn, join, fearless concurrency
> - **`Arc<Mutex<T>>`** — shared mutable state an toàn
> - **Channels** (`mpsc`) — message passing giữa threads
> - **`async/await`** — cooperative concurrency
> - **`tokio`** — async runtime cho production
> - **Actors pattern** via channels
>
> **Yêu cầu trước**: Chapter 9 (Ownership), Chapter 16 (Traits).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn dùng concurrency mà **compiler đảm bảo** không data race.

---

## Concurrency & Async — Rust's Superpower

Concurrency là lĩnh vực Rust tỏa sáng nhất. Ownership system — thứ bạn học ở Ch9 — không chỉ ngăn memory bugs, nó ngăn cả **data races tại compile time**. Nếu code compile, bạn biết chắc không có data race. Đây là guarantee mà không ngôn ngữ nào khác cung cấp.

Async Rust cho phép viết code non-blocking hiệu quả: xử lý hàng ngàn connections đồng thời với overhead minimal. Tokio runtime làm nền tảng cho nhiều dự án production lớn (Cloudflare, Discord, Amazon).

---

Concurrency là lĩnh vực mà Rust thể hiện khác biệt rõ nhất so với mọi ngôn ngữ khác.

Trong Go, concurrency bugs (data races) bị bắt runtime qua race detector. Trong Java, bạn hy vọng `synchronized` đặt đúng chỗ. Trong Python, GIL giới hạn parallelism. Trong Rust? **Compiler bắt data races tại compile time**. Nếu code compile, không có data race. Đây không phải quảng cáo — đó là guarantee toán học từ ownership system.

Chapter này dạy bạn hai mô hình concurrency trong Rust: **threads** (OS-level parallelism) và **async/await** (cooperative multitasking). Threads cho CPU-bound work. Async cho IO-bound work (network, database). Cả hai đều an toàn nhờ Rust's type system.

Tokio — async runtime phổ biến nhất — là nền tảng cho nhiều production systems lớn: Cloudflare Workers, Discord, Amazon. Bạn sẽ dùng nó xuyên suốt Part VII.

## 36.1 — Threads: Chạy song song

### Spawn & Join

```rust
// filename: src/main.rs

use std::thread;
use std::time::Duration;

fn main() {
    // Spawn thread
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("[thread] Working... {}/5", i);
            thread::sleep(Duration::from_millis(100));
        }
        42 // return value
    });

    // Main thread continues
    println!("[main] Doing other work...");
    thread::sleep(Duration::from_millis(250));

    // Wait for thread to finish, get result
    let result = handle.join().unwrap();
    println!("[main] Thread returned: {}", result);
}
```

### Move closures

```rust
// filename: src/main.rs

use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    // `move` chuyển ownership vào thread
    let handle = thread::spawn(move || {
        let sum: i32 = data.iter().sum();
        println!("Sum: {}", sum);
        sum
    });

    // data không còn ở main! (moved)
    // println!("{:?}", data); // ❌ error: value moved

    let result = handle.join().unwrap();
    println!("Result: {}", result);
}
```

### Parallel computation

```rust
// filename: src/main.rs

use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let chunk_size = 5;

    // Split work across threads
    let mut handles = vec![];

    for chunk in data.chunks(chunk_size) {
        let chunk = chunk.to_vec(); // clone cho mỗi thread
        handles.push(thread::spawn(move || {
            let sum: i32 = chunk.iter().sum();
            println!("  Chunk {:?} → sum={}", chunk, sum);
            sum
        }));
    }

    // Collect results
    let total: i32 = handles.into_iter()
        .map(|h| h.join().unwrap())
        .sum();

    println!("Total: {}", total); // 55
}
```

---

## 36.2 — Shared State: Arc<Mutex<T>>

### Vấn đề: Nhiều threads, 1 data

```rust
// filename: src/main.rs

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc = Atomic Reference Count (shared ownership across threads)
    // Mutex = Mutual Exclusion (1 thread access lúc 1 thời điểm)
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // Mutex unlock tự động khi `num` ra khỏi scope
        }));
    }

    for h in handles { h.join().unwrap(); }

    println!("Counter: {}", *counter.lock().unwrap()); // 10
}
```

### Thread-safe accumulator

```rust
// filename: src/main.rs

use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashMap;

fn main() {
    let word_counts = Arc::new(Mutex::new(HashMap::<String, u32>::new()));

    let texts = vec![
        "hello world hello",
        "world foo bar",
        "hello bar baz",
        "foo hello world",
    ];

    let mut handles = vec![];

    for text in texts {
        let counts = Arc::clone(&word_counts);
        let text = text.to_string();
        handles.push(thread::spawn(move || {
            for word in text.split_whitespace() {
                let mut map = counts.lock().unwrap();
                *map.entry(word.to_string()).or_insert(0) += 1;
            }
        }));
    }

    for h in handles { h.join().unwrap(); }

    let counts = word_counts.lock().unwrap();
    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    println!("Word counts:");
    for (word, count) in sorted {
        println!("  {}: {}", word, count);
    }
}
```

---

## 36.3 — Channels: Message Passing

### mpsc — Multiple Producers, Single Consumer

```rust
// filename: src/main.rs

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Producer 1
    let tx1 = tx.clone();
    thread::spawn(move || {
        for i in 1..=3 {
            tx1.send(format!("[P1] Message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Producer 2
    let tx2 = tx.clone();
    thread::spawn(move || {
        for i in 1..=3 {
            tx2.send(format!("[P2] Message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });

    drop(tx); // Drop original sender so rx knows when all done

    // Consumer
    for received in rx {
        println!("Got: {}", received);
    }
    println!("All producers done!");
}
```

### Pipeline pattern

```rust
// filename: src/main.rs

use std::sync::mpsc;
use std::thread;

fn main() {
    // Stage 1: Generate → Stage 2: Process → Stage 3: Collect
    let (tx1, rx1) = mpsc::channel::<i32>();
    let (tx2, rx2) = mpsc::channel::<String>();

    // Stage 1: Generate numbers
    thread::spawn(move || {
        for i in 1..=10 { tx1.send(i).unwrap(); }
    });

    // Stage 2: double + format
    thread::spawn(move || {
        for n in rx1 {
            tx2.send(format!("{} → {}", n, n * 2)).unwrap();
        }
    });

    // Stage 3: Collect
    let results: Vec<String> = rx2.into_iter().collect();
    println!("Pipeline results:");
    for r in &results { println!("  {}", r); }
}
```

---

## ✅ Checkpoint 36.3

> Ghi nhớ:
> 1. **`thread::spawn`** — OS thread, `move` closure
> 2. **`Arc<Mutex<T>>`** — shared mutable state, lock/unlock auto
> 3. **`mpsc::channel`** — message passing, nhiều producer 1 consumer
> 4. Compiler **bắt data races lúc compile time** — Send + Sync traits tự động!

---

## 36.4 — Async/Await

### Tại sao async?

```
Threads:  1 thread per request → 10,000 requests = 10,000 threads → 💀 OOM

Async:    1 thread handles MANY requests → 10,000 requests = few threads → ✅
          (cooperative scheduling: task yields khi chờ IO)
```

### Setup tokio

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

### Basic async

```rust
// filename: src/main.rs

use std::time::Duration;

async fn fetch_data(name: &str, delay_ms: u64) -> String {
    // Simulate async IO (database query, HTTP call, etc.)
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    format!("{}: done in {}ms", name, delay_ms)
}

#[tokio::main]
async fn main() {
    // Sequential: total = 300ms
    println!("=== Sequential ===");
    let a = fetch_data("Users", 100).await;
    let b = fetch_data("Orders", 200).await;
    println!("{}", a);
    println!("{}", b);

    // Concurrent: total = 200ms (max of both)
    println!("\n=== Concurrent ===");
    let (a, b) = tokio::join!(
        fetch_data("Users", 100),
        fetch_data("Orders", 200),
    );
    println!("{}", a);
    println!("{}", b);
}
```

### Spawn concurrent tasks

```rust
// filename: src/main.rs

use std::time::Duration;

async fn process_order(id: u32) -> String {
    tokio::time::sleep(Duration::from_millis(50)).await;
    format!("Order #{} processed", id)
}

#[tokio::main]
async fn main() {
    // Spawn many concurrent tasks
    let mut handles = vec![];

    for id in 1..=10 {
        handles.push(tokio::spawn(async move {
            process_order(id).await
        }));
    }

    // Collect results
    for handle in handles {
        println!("{}", handle.await.unwrap());
    }
}
```

---

## 36.5 — Async Channels (tokio::sync::mpsc)

### Actor pattern

```rust
// filename: src/main.rs

use tokio::sync::mpsc;

// ═══ Actor messages ═══
#[derive(Debug)]
enum BankMessage {
    Deposit(u64),
    Withdraw(u64, tokio::sync::oneshot::Sender<Result<(), String>>),
    Balance(tokio::sync::oneshot::Sender<i64>),
}

// ═══ Actor ═══
async fn bank_actor(mut rx: mpsc::Receiver<BankMessage>) {
    let mut balance: i64 = 0;

    while let Some(msg) = rx.recv().await {
        match msg {
            BankMessage::Deposit(amount) => {
                balance += amount as i64;
                println!("  [Actor] Deposited {}đ → balance: {}đ", amount, balance);
            }
            BankMessage::Withdraw(amount, reply) => {
                if amount as i64 > balance {
                    reply.send(Err("Insufficient funds".into())).unwrap();
                } else {
                    balance -= amount as i64;
                    println!("  [Actor] Withdrawn {}đ → balance: {}đ", amount, balance);
                    reply.send(Ok(())).unwrap();
                }
            }
            BankMessage::Balance(reply) => {
                reply.send(balance).unwrap();
            }
        }
    }
}

// ═══ Actor handle (client-facing API) ═══
#[derive(Clone)]
struct BankHandle {
    tx: mpsc::Sender<BankMessage>,
}

impl BankHandle {
    fn new() -> (Self, mpsc::Receiver<BankMessage>) {
        let (tx, rx) = mpsc::channel(100);
        (BankHandle { tx }, rx)
    }

    async fn deposit(&self, amount: u64) {
        self.tx.send(BankMessage::Deposit(amount)).await.unwrap();
    }

    async fn withdraw(&self, amount: u64) -> Result<(), String> {
        let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
        self.tx.send(BankMessage::Withdraw(amount, reply_tx)).await.unwrap();
        reply_rx.await.unwrap()
    }

    async fn balance(&self) -> i64 {
        let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
        self.tx.send(BankMessage::Balance(reply_tx)).await.unwrap();
        reply_rx.await.unwrap()
    }
}

#[tokio::main]
async fn main() {
    let (handle, rx) = BankHandle::new();

    // Start actor
    tokio::spawn(bank_actor(rx));

    // Use from multiple concurrent tasks
    let h1 = handle.clone();
    let h2 = handle.clone();

    let t1 = tokio::spawn(async move {
        h1.deposit(1_000_000).await;
        h1.deposit(500_000).await;
    });

    let t2 = tokio::spawn(async move {
        // Wait a bit for deposits
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        h2.withdraw(300_000).await.unwrap();
    });

    t1.await.unwrap();
    t2.await.unwrap();

    println!("\nFinal balance: {}đ", handle.balance().await);
}
```

---

## 36.6 — Fearless Concurrency: Compiler Guardrails

| Trait | Ý nghĩa | Types |
|-------|---------|-------|
| **`Send`** | Can be **sent** to another thread | Most types ✅. NOT: `Rc<T>`, raw pointers |
| **`Sync`** | Can be **shared** between threads | Most types ✅. NOT: `Cell`, `RefCell` |

```rust
// Compiler prevents data races AT COMPILE TIME:

// ❌ Rc<T> is not Send — cannot share across threads
// use std::rc::Rc;
// let data = Rc::new(42);
// thread::spawn(move || println!("{}", data)); // COMPILE ERROR

// ✅ Arc<T> is Send + Sync — safe!
use std::sync::Arc;
let data = Arc::new(42);
let d = data.clone();
std::thread::spawn(move || println!("{}", d));
```

### Cheat sheet

```
Cần shared ownership?
├── Single thread → Rc<T>
└── Multi thread → Arc<T>

Cần interior mutability?
├── Single thread → RefCell<T>
└── Multi thread → Mutex<T> hoặc RwLock<T>

Shared + Mutable + Multi-thread?
└── Arc<Mutex<T>>

Message passing?
├── Sync → std::sync::mpsc
└── Async → tokio::sync::mpsc
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Parallel sum

Chia `vec![1..=1000]` thành 4 chunks, tính sum mỗi chunk song song, tổng hợp kết quả.

<details><summary>✅ Lời giải</summary>

```rust
use std::thread;

fn main() {
    let data: Vec<i64> = (1..=1000).collect();
    let chunks: Vec<Vec<i64>> = data.chunks(250).map(|c| c.to_vec()).collect();

    let handles: Vec<_> = chunks.into_iter()
        .map(|chunk| thread::spawn(move || chunk.iter().sum::<i64>()))
        .collect();

    let total: i64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("Total: {}", total); // 500500
}
```

</details>

---

**Bài 2** (10 phút): Producer-Consumer

3 producers gửi numbers (1-10, 11-20, 21-30) qua channel. 1 consumer sum tất cả. Kết quả = 465.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let ranges = vec![1..=10, 11..=20, 21..=30];

    for range in ranges {
        let tx = tx.clone();
        thread::spawn(move || {
            for n in range { tx.send(n).unwrap(); }
        });
    }
    drop(tx);

    let sum: i32 = rx.into_iter().sum();
    println!("Sum: {}", sum); // 465
}
```

</details>

---

**Bài 3** (15 phút): Async Worker Pool

Tạo worker pool xử lý 20 "jobs" song song (max 4 concurrent):
- Mỗi job = sleep random 50-200ms + return result
- Dùng `tokio::sync::Semaphore` để limit concurrency

<details><summary>✅ Lời giải Bài 3</summary>

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;
use std::time::Duration;

async fn process_job(id: u32) -> String {
    let delay = 50 + (id * 7) % 150;
    tokio::time::sleep(Duration::from_millis(delay as u64)).await;
    format!("Job {} done ({}ms)", id, delay)
}

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(4)); // max 4 concurrent
    let mut handles = vec![];

    for id in 1..=20 {
        let sem = semaphore.clone();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            process_job(id).await
        }));
    }

    for handle in handles {
        println!("{}", handle.await.unwrap());
    }
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Rc not Send" | Rc chỉ cho 1 thread | Dùng `Arc` |
| "Deadlock" | 2 Mutex lock nhau | Lock ordering, hoặc dùng channels |
| "Block inside async" | `thread::sleep` trong async | Dùng `tokio::time::sleep` |
| "Future not Send" | Holding non-Send across .await | Restructure: drop trước .await |

---

## Tóm tắt

- ✅ **Threads**: `thread::spawn` + `move` + `join()`. Parallel computation.
- ✅ **`Arc<Mutex<T>>`**: Shared mutable state. Lock tự unlock khi drop.
- ✅ **Channels**: `mpsc` cho message passing. Pipeline pattern.
- ✅ **Async/Await**: Cooperative concurrency. `tokio::join!` cho concurrent IO.
- ✅ **Actor pattern**: `mpsc` + message enum + reply channel (`oneshot`).
- ✅ **Fearless concurrency**: `Send`/`Sync` traits → compiler **bắt data races lúc compile time**.

---

## Tiếp theo

→ Chapter 36B: **Web Services with Axum** — bạn sẽ học cách build REST API hoàn chỉnh: routes, extractors, middleware, error handling. Đây là bước cuối trước khi bước vào Part VII Production Engineering!
