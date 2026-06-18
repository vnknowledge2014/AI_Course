# Chapter 44 — AI System Design & Infrastructure

> **Bạn sẽ học được**:
> - Kiến trúc của LLM Inference Gateway (Tại sao lại cần Proxy bằng Rust?)
> - KV Cache và PagedAttention (vLLM / TensorRT-LLM)
> - Thiết kế hệ thống RAG quy mô lớn (Vector DB Sharding & Embedding Pipelines)
> - Quản lý trạng thái của Multi-Agent Systems bằng Event Sourcing
>
> **Yêu cầu trước**: Chapter 43 (System Design)
> **Thời gian đọc**: ~45 phút | **Level**: Principal

---

## 44.1 — Tại sao Software Engineering truyền thống chưa đủ cho AI?

Trong web backend thông thường (CRUD), request gửi tới server sẽ lấy dữ liệu từ DB, parse ra JSON và trả về. Quá trình này mất vài chục ms, tiêu tốn rất ít RAM và CPU. Hệ thống stateless scale cực kỳ dễ dàng.

**Trong thế giới AI/LLM thì khác:**
1. **Model state là khổng lồ**: Một model 70B parameter cần 140GB VRAM chỉ để load lên (với độ chính xác FP16).
2. **Stateful Inference (KV Cache)**: Khi LLM sinh ra từng chữ (token), nó phải nhớ các token đã sinh ra trước đó. Bộ nhớ KV Cache phình to lên theo từng token mới sinh ra.
3. **Thời gian chạy siêu lâu**: Một request có thể chạy mất 10-30 giây.
4. **Giới hạn phần cứng**: GPU cực đắt đỏ và không thể "tạo ra vô hạn" như việc spin-up máy ảo CPU.

Chính vì vậy, thiết kế hệ thống AI là một nghệ thuật về phân bổ tài nguyên. Và **Rust** đang trở thành ngôn ngữ thống trị ở tầng hạ tầng AI nhờ tốc độ và khả năng kiểm soát bộ nhớ.

---

## 44.2 — LLM Inference Gateway (Rust ở Frontline)

Khi bạn deploy model cho triệu người dùng, bạn không mở port PyTorch trực tiếp ra internet. Bạn cần một **Inference Gateway**.

### Vai trò của Gateway:
- **Rate Limiting & Authentication**: Kiểm tra token người dùng, ngăn chặn spam.
- **Routing & Load Balancing**: Đẩy request tới các GPU node rảnh rỗi.
- **Billing**: Đếm số input/output tokens và tính tiền.
- **Semantic Caching**: Nếu 2 người hỏi cùng 1 câu, trả về từ Redis thay vì gọi GPU.

```rust
use axum::{extract::State, routing::post, Json, Router};
use std::sync::Arc;

struct GatewayState {
    redis_pool: RedisPool,
    gpu_cluster_client: ReqwestClient,
}

// Handler nhận request chat từ user
async fn chat_handler(
    State(state): State<Arc<GatewayState>>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, GatewayError> {
    
    // 1. Rate Limiting check qua Redis
    if is_rate_limited(&state.redis_pool, &payload.user_id).await? {
        return Err(GatewayError::TooManyRequests);
    }
    
    // 2. Gọi GPU Server (VD: vLLM)
    let gpu_response = state.gpu_cluster_client
        .post("http://gpu-cluster:8000/v1/chat/completions")
        .json(&payload)
        .send()
        .await?;
        
    let tokens_used = gpu_response.usage.total_tokens;
    
    // 3. Billing (Bất đồng bộ - Fire and Forget)
    tokio::spawn(async move {
        record_billing(&state.redis_pool, &payload.user_id, tokens_used).await;
    });

    Ok(Json(gpu_response.json().await?))
}
```

### Tại sao dưới tầng GPU lại dùng vLLM?
Bản thân việc chạy Pytorch thuần rất kém trong việc gộp nhiều user (batching). **vLLM** là công nghệ mang tính cách mạng vì nó áp dụng khái niệm **Paged Memory** của Hệ điều hành vào VRAM của GPU.
- Thay vì cấp phát KV Cache một cục dài liên tục (gây phân mảnh và phí phạm), vLLM chia KV Cache thành các Block nhỏ.
- Kết quả: Inference nhanh hơn 24x lần so với HuggingFace cơ bản!

---

## 44.3 — RAG System Design quy mô lớn

Retrieval-Augmented Generation (RAG) không chỉ là việc gọi API OpenAI và nhét text vào. Ở scale lớn (hàng triệu file PDF của enterprise), kiến trúc của bạn cần:

1. **Ingestion Pipeline bất đồng bộ**: 
   - User upload PDF.
   - Rust Backend đẩy event `DocumentUploaded` vào Kafka.
   - GPU Worker lắng nghe Kafka, chia nhỏ text (Chunking), chạy mô hình Embedding (như `BGE-m3`) và ghi vector vào Database.
   
2. **Vector Database Sharding**:
   - Ở scale nhỏ, bạn có thể dùng `pgvector` trong PostgreSQL.
   - Ở scale hàng tỷ vector, bạn cần các DB sinh ra cho nó như **Qdrant** (viết bằng Rust!) hay **Milvus**. Kiến trúc lúc này phải sharding dữ liệu trên nhiều nodes, dùng HNSW graph cho tìm kiếm xấp xỉ (Approximate Nearest Neighbors - ANN).

---

## 44.4 — Multi-Agent Orchestration

Khi có nhiều Agent AI tự động hoạt động (VD: Agent Coding, Agent Tester, Agent Reviewer), bạn không thể để chúng gọi hàm trực tiếp cho nhau. Quá trình sinh token chậm sẽ làm treo toàn bộ hệ thống.

**Giải pháp: Event Sourcing & CQRS (Đã học ở Chapter 17)**

- Các Agents là các microservices độc lập.
- Giao tiếp với nhau qua Message Queue (Kafka / RabbitMQ).

```rust
// Mô hình Event cho Multi-Agent
enum AgentEvent {
    CodeWritten { task_id: String, code: String },
    TestsFailed { task_id: String, errors: String },
    FeatureCompleted { task_id: String },
}

// Workflow (Saga Pattern)
// 1. Coder sinh code -> Emit CodeWritten
// 2. Tester nghe CodeWritten -> Chạy Test
// 3. Nếu lỗi -> Emit TestsFailed. Coder nghe lỗi và sửa lại.
```

Kiến trúc này giúp bạn scale từng loại agent riêng biệt và giữ lại toàn bộ lịch sử (Log) của "suy nghĩ" các AI.

---

## Tóm tắt

- Hạ tầng AI (AI Infrastructure) là nơi Software Engineering tỏa sáng. LLMs chỉ là động cơ, bạn cần làm toàn bộ khung xe, bánh xe và tay lái.
- **Rust** là công cụ hoàn hảo để viết AI Gateways, Vector Databases, và Streaming pipelines nhờ khả năng xử lý concurrency an toàn và không bị giật lag bởi Garbage Collector.
- **vLLM / TensorRT-LLM**: Engine bên dưới để inference model.
- Multi-Agent thực chất là bài toán Hệ Tán Tán (Distributed Systems), giải quyết hiệu quả bằng Event-Driven Architecture.

## Lời kết

Đây là phần cuối cùng cho các kiến thức về Production AI Systems. Mời bạn bước sang Chapter 45 (Capstone) để hoàn tất việc kết nối mọi thứ lại với nhau bằng một dự án hoàn chỉnh.
