# Chapter 43 — AI System Design & Infrastructure

> **Bạn sẽ học được**:
> - Kiến trúc hệ thống RAG (Retrieval-Augmented Generation) Scale Lớn
> - Thiết kế AI Gateway để bảo vệ LLMs
> - Quản lý trạng thái Multi-Agent bằng Message Brokers
> - Node.js/TypeScript đóng vai trò gì ở Backend AI?
>
> **Yêu cầu trước**: Chapter 42 (System Design)
> **Thời gian đọc**: ~40 phút | **Level**: Principal

---

## 43.1 — TypeScript/Node.js trong Kiến trúc AI

Hệ thống AI không chỉ có Python và CUDA. Để phục vụ 1 triệu requests mỗi giây, quản lý WebSockets cho Chat UI, và tích hợp với cơ sở dữ liệu doanh nghiệp, Node.js là ứng cử viên số một nhờ hệ sinh thái khổng lồ và Non-blocking I/O.

Vai trò chính của Backend TypeScript:
1. **AI API Gateway**: Routing, Rate Limiting, Load Balancing cho các cụm GPU.
2. **Orchestration**: Quản lý state của quá trình RAG, gọi các API bên ngoài, tổng hợp dữ liệu.
3. **Multi-Agent Runtime**: Điều phối các LLM agents giao tiếp với nhau qua hàng đợi tin nhắn (Message Queue).

---

## 43.2 — Thiết kế AI Gateway

Gateway đứng giữa User và LLM Server (chạy vLLM/Ollama).

```typescript
import { Hono } from 'hono'
import { rateLimiter } from 'hono-rate-limiter'

const app = new Hono()

// Rate Limiting (chặn lạm dụng API)
app.use(
  '/api/chat/*',
  rateLimiter({
    windowMs: 60 * 1000, // 1 phút
    limit: 10, // 10 requests / user / phút
    keyGenerator: (c) => c.req.header('Authorization') || c.req.ip,
  })
)

app.post('/api/chat/stream', async (c) => {
    const payload = await c.req.json();
    
    // 1. Kiểm tra Token / Quota hệ thống (Billing)
    // 2. Định tuyến (Route) đến GPU cluster rảnh rỗi nhất
    
    // Gọi GPU server (vLLM) trả về stream
    const response = await fetch('http://gpu-cluster:8000/v1/chat/completions', {
        method: 'POST',
        body: JSON.stringify(payload)
    });
    
    // Node.js stream phản hồi về cho Client cực kỳ nhẹ
    return new Response(response.body, {
        headers: { 'Content-Type': 'text/event-stream' }
    });
})
```

---

## 43.3 — RAG System Design Scale Lớn

Retrieval-Augmented Generation (RAG) không chỉ là nhét file PDF vào Prompt. Khi thiết kế hệ thống đọc 1 tỷ tài liệu doanh nghiệp:

1. **Ingestion Pipeline**: 
   - Không chạy trên 1 tiến trình Node.js (sẽ treo ngay). Cần dùng kiến trúc Event-Driven (Kafka, RabbitMQ) hoặc Redis Pub/Sub.
   - Worker A: Đọc file PDF, trích xuất text.
   - Worker B: Cắt text (Semantic Chunking).
   - Worker C: Gọi mô hình Embeddings và lưu vào Database.

2. **Vector Database Sharding**:
   - Dùng các Database chuyên dụng: Qdrant, Milvus. Cấu hình Cluster, Replication và Sharding key theo `tenant_id` (để tránh rò rỉ dữ liệu giữa các khách hàng).

3. **Hybrid Search**: Tìm kiếm vector đôi khi vô dụng với các từ khóa chính xác (VD: "Mã hợp đồng AB123"). Kiến trúc chuẩn phải kết hợp **Vector Search** + **Full-Text Search (Elasticsearch/MeiliSearch)** và dùng thuật toán **Reranking**.

---

## 43.4 — Multi-Agent Orchestration

Nhiều Agent tương tác với nhau (Agent Coder, Agent Tester). Hệ thống này phải mang tính bất đồng bộ hoàn toàn.

**Event Sourcing & CQRS (Đã học ở Phần 3 & 4)**

```typescript
// Các microservices giao tiếp qua Event
type AgentEvent = 
  | { type: 'CODE_WRITTEN', taskId: string, code: string }
  | { type: 'TESTS_FAILED', taskId: string, error: string }

// Kiến trúc Saga
// 1. Coder sinh ra code -> Publishes 'CODE_WRITTEN'
// 2. Tester lắng nghe (Subscribe) -> Chạy test sandbox -> Publishes 'TESTS_FAILED'
// 3. Coder lắng nghe -> Tự sửa lại code...
```

Nhờ DDD và FP, việc mô hình hóa các State và Event này trong TypeScript trở nên an toàn tuyệt đối nhờ Discriminated Unions.

---

## Tóm tắt

- Node.js cực kỳ lý tưởng để xây dựng vỏ bọc (Control Plane) cho các hệ thống AI.
- Hệ thống AI scale lớn đòi hỏi kiến trúc Event-Driven, Message Queues để xử lý Ingestion Pipeline bất đồng bộ.
- Multi-Agent Orchestration thực chất là bài toán Hệ Phân Tán (Distributed Systems), giải quyết bằng Event Sourcing.

## Tiếp theo
Đây là mảnh ghép Production cuối cùng. Hãy đến với Chapter 44 (Capstone) để đóng gói mọi kiến thức vào một dự án thực tế.
