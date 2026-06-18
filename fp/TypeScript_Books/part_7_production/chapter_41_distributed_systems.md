# Chapter 41 — Distributed Systems Fundamentals

> **Bạn sẽ học được**:
> - CAP Theorem — không thể có cả 3: Consistency, Availability, Partition tolerance
> - Message Queues — BullMQ, Kafka, RabbitMQ — async communication
> - Saga pattern — distributed transactions
> - Circuit Breaker — fail gracefully khi service DOWN
> - Retry + exponential backoff — xử lý transient failures
> - Observability — logging, metrics, tracing
>
> **Yêu cầu trước**: Chapter 34 (Backend), Chapter 37-38 (Data patterns).
> **Thời gian đọc**: ~50 phút | **Level**: Principal
> **Kết quả cuối cùng**: Hiểu challenges của distributed systems — và patterns để giải quyết.

---

Bạn biết chuỗi nhà hàng (franchise) không? McDonald's có 40,000 cửa hàng. Mỗi cửa hàng HOẠT ĐỘNG RIÊNG (distributed). Nếu cửa hàng Quận 1 hết Big Mac, cửa hàng Quận 3 VẪN BÁN bình thường (availability). Nhưng khuyến mãi mới chưa chắc đến TẤT CẢ cửa hàng CÙNG LÚC (eventual consistency). Và khi mạng giữa headquarters và cửa hàng bị đứt (partition) — cửa hàng vẫn phục vụ (nhưng có thể dùng giá cũ).

**Distributed systems** = nhiều servers hoạt động phối hợp. CAP theorem: pick 2 of 3.

Tại sao chương này quan trọng? Vì bất kỳ ứng dụng nào vượt ra MỘT server đều đối mặt với những thách thức này: mạng có thể đứt, servers có thể chết, data có thể stale. Bạn KHÔNG THỂ tránh — chỉ có thể TRANG BỊ patterns để xử lý. Patterns trong chương này: CAP theorem (để hiểu trade-offs), message queues (để decouple), circuit breaker (để fail gracefully), retry (để recover).

---

## Distributed Systems — CAP, Queues, Saga, Circuit Breaker

CAP theorem (pick CP or AP). Message queues (decouple services). Circuit breaker (fail gracefully). Retry + backoff (handle transient failures). Saga (distributed transactions via compensation). Patterns cho khi ứng dụng vượt ra MỘT server.


## 41.1 — CAP Theorem

CAP là constraint cơ bản nhất của distributed systems: bạn chỉ có thể đảm bảo 2 trong 3 tính chất: Consistency (tất cả nodes thấy data GIỐNG nhau), Availability (mọi request được trả lời), Partition Tolerance (hệ thống vẫn chạy khi mạng đứt). Cách nhau tưởng là “chọn 2” — thực tế: P là BẮT BUỘC (mạng SẼ fail), nên lựa chọn thực sự là CP hay AP.

```typescript
// filename: src/distributed/cap.ts
import assert from "node:assert/strict";

// CAP = Consistency + Availability + Partition Tolerance
// In a distributed system, you can only guarantee 2 of 3

// C = Consistency: all nodes see SAME data at same time
// A = Availability: every request gets a response (non-error)
// P = Partition Tolerance: system works even when network splits

// Reality: P is NOT OPTIONAL (networks WILL fail)
// So the real choice: CP or AP

// CP: consistent but may be unavailable during partition
// → Banks, inventory systems: better to DENY than give wrong data
// → PostgreSQL, MongoDB (with strong read concern)

// AP: available but may return stale data during partition
// → Social media feeds, caching: better to show SOMETHING than nothing
// → DynamoDB, Cassandra, DNS

// === Eventual Consistency simulation ===
type Node = { id: string; data: Map<string, string>; version: number };

const createNode = (id: string): Node => ({ id, data: new Map(), version: 0 });

const writeToNode = (node: Node, key: string, value: string): void => {
    node.data.set(key, value);
    node.version++;
};

const replicateTo = (from: Node, to: Node): void => {
    for (const [key, value] of from.data) {
        to.data.set(key, value);
    }
    to.version = from.version;
};

const node1 = createNode("N1");
const node2 = createNode("N2");

writeToNode(node1, "price", "20000000");
// node1 has "price", node2 doesn't yet (inconsistent!)
assert.strictEqual(node1.data.get("price"), "20000000");
assert.strictEqual(node2.data.has("price"), false);

// Replication (eventual consistency)
replicateTo(node1, node2);
assert.strictEqual(node2.data.get("price"), "20000000");
// NOW consistent!

console.log("CAP theorem OK ✅");
```

---

## 41.2 — Message Queues

Message Queue giải quyết vấn đề COUPLING giữa services. Không có queue: Order Service gọi TRỰC TIẾP Notification Service. Notification Service chết? Order Service chờ. Có queue: Order Service gửi message vào queue, rồi LÀM VIỆC TIẾP. Notification Service xử lý khi sẵn sàng. Decoupled.

Queue cũng giải quyết SPIKE: Black Friday, 100x tràfic bình thường. Không có queue: server quá tải, crash. Có queue: messages được buffer, xử lý dần. Queue = bể chứa nước giữa mưa lớn và nhà máy lọc.

```typescript
// filename: src/distributed/message_queue.ts
import assert from "node:assert/strict";

// Message Queue = async communication between services
// Producer → Queue → Consumer

// Why? Decoupling, buffering, retry, scaling consumers independently

type Message<T> = { id: string; payload: T; timestamp: number; retries: number };

type Queue<T> = {
    publish: (payload: T) => string;
    consume: () => Message<T> | null;
    size: () => number;
};

const createQueue = <T>(): Queue<T> => {
    const messages: Message<T>[] = [];
    let nextId = 0;

    return {
        publish: (payload) => {
            const id = `MSG-${++nextId}`;
            messages.push({ id, payload, timestamp: Date.now(), retries: 0 });
            return id;
        },
        consume: () => messages.shift() ?? null,
        size: () => messages.length,
    };
};

// Example: Order events
type OrderEvent = { type: "order_confirmed"; orderId: string; total: number };

const orderQueue = createQueue<OrderEvent>();

// Producer (Order Service)
orderQueue.publish({ type: "order_confirmed", orderId: "ORD-1", total: 20_000_000 });
orderQueue.publish({ type: "order_confirmed", orderId: "ORD-2", total: 5_000_000 });

assert.strictEqual(orderQueue.size(), 2);

// Consumer (Notification Service)
const msg1 = orderQueue.consume();
assert.strictEqual(msg1!.payload.orderId, "ORD-1");

// Consumer (Analytics Service) — same queue, different consumer
const msg2 = orderQueue.consume();
assert.strictEqual(msg2!.payload.orderId, "ORD-2");

assert.strictEqual(orderQueue.size(), 0);  // all consumed

// Real: BullMQ (Redis-based), Kafka, RabbitMQ

console.log("Message queue OK ✅");
```

---

## 41.3 — Circuit Breaker & Retry

Khi một external service chết, bạn không muốn gọi nó LIÊN TỤC (lãng phí, chậm, có thể gây cascading failure). Circuit Breaker giống cầu dao điện: bình thường = đóng (cho request qua). Quá nhiều failures = mở (cắt điện, reject requests ngay). Sau một thời gian = half-open (thử một request, nếu OK → đóng lại).

Retry với exponential backoff: lần 1 chờ 100ms, lần 2 chờ 200ms, lần 3 chờ 400ms... Không đập server liên tục, cho nó thời gian hồi phục. pattern này cực kỳ phổ biến: AWS SDK, gRPC, và mọi HTTP client nghiêm túc đều dùng.

```typescript
// filename: src/distributed/circuit_breaker.ts
import assert from "node:assert/strict";

// Circuit Breaker = stop calling failing service
// States: CLOSED (normal) → OPEN (failing, reject calls) → HALF-OPEN (test one call)

type CircuitState = "closed" | "open" | "half-open";

type CircuitBreaker = {
    call: <T>(fn: () => Promise<T>) => Promise<T>;
    state: () => CircuitState;
    stats: () => { failures: number; successes: number };
};

const createCircuitBreaker = (opts: {
    failureThreshold: number;
    resetTimeoutMs: number;
}): CircuitBreaker => {
    let state: CircuitState = "closed";
    let failures = 0;
    let successes = 0;
    let lastFailureTime = 0;

    return {
        call: async (fn) => {
            if (state === "open") {
                if (Date.now() - lastFailureTime > opts.resetTimeoutMs) {
                    state = "half-open";
                } else {
                    throw new Error("Circuit breaker OPEN — request rejected");
                }
            }

            try {
                const result = await fn();
                if (state === "half-open") state = "closed";
                successes++;
                failures = 0;
                return result;
            } catch (e) {
                failures++;
                lastFailureTime = Date.now();
                if (failures >= opts.failureThreshold) state = "open";
                throw e;
            }
        },
        state: () => state,
        stats: () => ({ failures, successes }),
    };
};

// === Test ===
const run = async () => {
    const breaker = createCircuitBreaker({ failureThreshold: 3, resetTimeoutMs: 5000 });

    let callCount = 0;
    const failingService = async (): Promise<string> => {
        callCount++;
        throw new Error("Service unavailable");
    };

    // 3 failures → circuit opens
    for (let i = 0; i < 3; i++) {
        try { await breaker.call(failingService); } catch {}
    }
    assert.strictEqual(breaker.state(), "open");

    // Next call rejected immediately (no actual call to service)
    const beforeCount = callCount;
    try { await breaker.call(failingService); } catch (e: any) {
        assert.ok(e.message.includes("OPEN"));
    }
    assert.strictEqual(callCount, beforeCount);  // service NOT called!

    console.log("Circuit breaker OK ✅");
};

run();
```

```typescript
// filename: src/distributed/retry.ts
import assert from "node:assert/strict";

// Retry with exponential backoff
const retryWithBackoff = async <T>(
    fn: () => Promise<T>,
    maxRetries: number,
    baseDelayMs = 100,
): Promise<T> => {
    for (let attempt = 0; attempt <= maxRetries; attempt++) {
        try {
            return await fn();
        } catch (e) {
            if (attempt === maxRetries) throw e;
            const delay = baseDelayMs * Math.pow(2, attempt);
            // In real code: await new Promise(r => setTimeout(r, delay));
        }
    }
    throw new Error("Unreachable");
};

// Test: fails twice, succeeds third time
let attempts = 0;
const flakyService = async (): Promise<string> => {
    attempts++;
    if (attempts < 3) throw new Error("Temporary failure");
    return "success";
};

retryWithBackoff(flakyService, 5, 0).then(result => {
    assert.strictEqual(result, "success");
    assert.strictEqual(attempts, 3);
    console.log("Retry OK ✅");
});
```

---

## 41.4 — Saga Pattern

Saga giải quyết vấn đề **distributed transactions**. Trong monolith: một database transaction = atomicity (tất cả hoặc không gì). Trong microservices: nhiều databases, không có global transaction. Saga = chuỗi LOCAL transactions + COMPENSATION actions.

Ví dụ đặt vé máy bay: (1) Giữ chỗ → (2) Thu tiền → (3) Xuất vé. Nếu step 3 fail? Compensation: (3') Hủy vé → (2') Hoàn tiền → (1') Giải phóng chỗ. Mỗi step có compensation ngược lại.

```typescript
// filename: src/distributed/saga.ts
import assert from "node:assert/strict";

type SagaStep<T> = {
    name: string;
    execute: (ctx: T) => Promise<T>;
    compensate: (ctx: T) => Promise<T>;
};

type SagaResult<T> = 
    | { tag: "ok"; value: T }
    | { tag: "err"; failedStep: string; error: string; compensated: boolean };

const runSaga = async <T>(steps: SagaStep<T>[], initial: T): Promise<SagaResult<T>> => {
    let ctx = initial;
    const completed: SagaStep<T>[] = [];

    for (const step of steps) {
        try {
            ctx = await step.execute(ctx);
            completed.push(step);
        } catch (e: any) {
            // Compensate in REVERSE order
            for (const done of completed.reverse()) {
                try { ctx = await done.compensate(ctx); } catch { /* log and continue */ }
            }
            return { tag: "err", failedStep: step.name, error: e.message, compensated: true };
        }
    }
    return { tag: "ok", value: ctx };
};

// Test: successful saga
type BookingCtx = { seated: boolean; paid: boolean; ticketed: boolean };

const steps: SagaStep<BookingCtx>[] = [
    {
        name: "reserve_seat",
        execute: async (ctx) => ({ ...ctx, seated: true }),
        compensate: async (ctx) => ({ ...ctx, seated: false }),
    },
    {
        name: "charge_payment",
        execute: async (ctx) => ({ ...ctx, paid: true }),
        compensate: async (ctx) => ({ ...ctx, paid: false }),
    },
    {
        name: "issue_ticket",
        execute: async (ctx) => ({ ...ctx, ticketed: true }),
        compensate: async (ctx) => ({ ...ctx, ticketed: false }),
    },
];

const run = async () => {
    const ok = await runSaga(steps, { seated: false, paid: false, ticketed: false });
    assert.strictEqual(ok.tag, "ok");
    if (ok.tag === "ok") {
        assert.strictEqual(ok.value.seated, true);
        assert.strictEqual(ok.value.paid, true);
        assert.strictEqual(ok.value.ticketed, true);
    }

    // Test: failing saga (ticket fails → compensate payment → compensate seat)
    const failSteps: SagaStep<BookingCtx>[] = [
        steps[0], steps[1],
        { ...steps[2], execute: async () => { throw new Error("Ticket service down"); } },
    ];
    const err = await runSaga(failSteps, { seated: false, paid: false, ticketed: false });
    assert.strictEqual(err.tag, "err");
    if (err.tag === "err") {
        assert.strictEqual(err.failedStep, "issue_ticket");
        assert.strictEqual(err.compensated, true);
    }

    console.log("Saga pattern OK ✅");
};
run();
```

> **💡 FP connection**: Saga = Railway Oriented Programming (Ch22) ở quy mô distributed. `execute` = happy path. `compensate` = error recovery. Mỗi step là một function trong pipeline. Fail → switch to compensation track.

---

## 🏋️ Bài tập

**Bài 1** (10 phút): Thêm dead letter queue

```typescript
// Mở rộng message queue:
// 1. Nếu consumer fail 3 lần → chuyển message sang dead letter queue
// 2. Dead letter queue cho phép inspect và retry manually
// Gợi ý: thêm retries counter vào Message type
```

<details><summary>✅ Lời giải</summary>

```typescript
import assert from "node:assert/strict";

type Message<T> = { id: string; payload: T; retries: number };
type QueueWithDLQ<T> = {
    publish: (payload: T) => string;
    consume: () => Message<T> | null;
    fail: (msg: Message<T>) => void;  // mark as failed
    deadLetters: () => Message<T>[];
};

const createQueueWithDLQ = <T>(maxRetries = 3): QueueWithDLQ<T> => {
    const queue: Message<T>[] = [];
    const dlq: Message<T>[] = [];
    let id = 0;
    return {
        publish: (payload) => { const mid = `M-${++id}`; queue.push({ id: mid, payload, retries: 0 }); return mid; },
        consume: () => queue.shift() ?? null,
        fail: (msg) => {
            if (msg.retries >= maxRetries) { dlq.push(msg); }
            else { queue.push({ ...msg, retries: msg.retries + 1 }); }
        },
        deadLetters: () => [...dlq],
    };
};

const q = createQueueWithDLQ<string>(2);
q.publish("task-1");
const m1 = q.consume()!;
q.fail(m1);  // retry 1
const m2 = q.consume()!;
q.fail(m2);  // retry 2 → DLQ!
assert.strictEqual(q.deadLetters().length, 1);
assert.strictEqual(q.consume(), null);  // queue empty
console.log("DLQ OK ✅");
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Circuit breaker never closes | Reset timeout quá lâu | Giảm resetTimeoutMs, monitor half-open success rate |
| Messages lost in queue | Consumer crashes giữa chừng | Dùng ACK/NACK pattern (consume → process → ACK) |
| Saga compensation fails | Compensation action cũng có thể fail | Log + alert + manual intervention. Idempotent compensations |
| Retry storm | Nhiều instances retry cùng lúc | Thêm jitter: `delay * (1 + random * 0.3)` |
| Eventual consistency confusing users | User thấy stale data | Show "updating..." indicator, optimistic UI |

---

## ✅ Checkpoint 41

> Đến đây bạn phải hiểu:
> 1. **CAP**: pick CP or AP. Networks WILL fail (P mandatory)
> 2. **Message queues**: async, decoupled, buffered. BullMQ/Kafka/RabbitMQ
> 3. **Circuit breaker**: CLOSED → OPEN (stop calling) → HALF-OPEN (test)
> 4. **Retry + backoff**: exponential delay. Don't hammer failing service
> 5. **Saga**: distributed transactions via compensation. ROP at scale
>
> **Test nhanh**: Queue message bị xử lý SAI (bug trong consumer). Message đã bị consume. Làm sao?
> <details><summary>Đáp án</summary>Dùng **dead letter queue** (DLQ). Messages fail được chuyển vào DLQ để inspect và retry. Hoặc: dùng ACK pattern — chỉ ACK khi xử lý thành công.</details>

---

## 💬 Đối thoại với bản thân: Distributed Systems Q&A

**Q: App tôi chạy trên MỘT server. Cần học Distributed Systems không?**

A: Khi app grow: thêm Redis (cache), thêm message queue (async tasks), thêm 3rd party APIs (payment, email). Bạn ĐÃ distributed rồi mà chưa biết. CAP, retry, circuit breaker sẽ cứu bạn khi Redis/API chết lúc 2h sáng.

**Q: CAP theorem — pick 2? Vậy chọn gì?**

A: P (partition tolerance) là BẮT BUỘC — mạng WILL fail. Nên chọn thực sự = CP (consistent, mất availability khi network issues) hay AP (available, chấp nhận eventual consistency). Hầu hết web apps chọn AP + eventual consistency.

**Q: Message queue hay trực tiếp gọi service?**

A: Trực tiếp: đơn giản, nhưng tight coupling + nếu service chết thì caller chết luôn. Queue: decoupled, buffer spikes, retry tự động. Rule: nếu caller KHÔNG CẦN response ngay → queue. Nếu cần response → direct call + circuit breaker.

---

## Tóm tắt

Distributed systems không "khó" vì một thứ cụ thể — chúng khó vì COMBINATION của nhiều thứ có thể sai CÙNG LÚC. CAP cho bạn mental model. Patterns (queue, circuit breaker, retry) cho bạn TOOLS. Chương này là nền tảng — để khi gặp vấn đề, bạn BIẾT TÊN của nó và BIẾT pattern để giải.

- ✅ **CAP**: Consistency + Availability + Partition tolerance. Pick 2.
- ✅ **Message queues**: producer → queue → consumer. Decoupled.
- ✅ **Circuit breaker**: protect from cascading failures.
- ✅ **Retry + backoff**: handle transient failures gracefully.
- ✅ **Saga**: distributed transactions via compensation.

## Tiếp theo

→ Chapter 42: **System Design Thinking** — capacity estimation, load balancing, caching layers, microservices. Tư duy architect.
