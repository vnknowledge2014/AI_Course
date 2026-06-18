# Chapter 44 — AI System Design & Infrastructure

> **Bạn sẽ học được**:
> - LLM Inference Architecture
> - RAG Scaling & Vector DB
> - Multi-Agent Event Sourcing
>
> **Yêu cầu trước**: Chapter 43 (System Design)
> **Thời gian đọc**: ~35 phút | **Level**: Principal

---

## 44.1 — LLM Inference Gateway

Khi phục vụ mô hình LLM lớn, bạn không chỉ chạy Python/PyTorch thẳng ra ngoài internet.
- **vLLM / TensorRT-LLM**: Engine tối ưu hóa PagedAttention (Memory cho KV Cache).
- **Gateway (Rust)**: Xử lý rate limiting, API Key validation, load balancing.

```rust
// Rust làm API Gateway cho LLM
async fn llm_gateway(req: Request) -> Result<Response, Error> {
    // 1. Kiểm tra Token / Quota
    // 2. Chuyển tiếp request tới vLLM cluster (Streaming)
    // 3. Ghi nhận log số lượng token sử dụng (Billing)
    Ok(stream_response())
}
```

## 44.2 — RAG (Retrieval-Augmented Generation) Scale Lớn

- **Vector Database**: Qdrant, Milvus. Cần phân chia (Sharding) khi số lượng vector vượt 100 triệu.
- **Chunking Pipeline**: Dùng Kafka để xử lý PDF -> Text -> Embeddings bất đồng bộ.

## 44.3 — Multi-Agent Orchestration

Nhiều AI Agents giao tiếp với nhau. Sử dụng Event Sourcing (đã học ở Chapter 17) để lưu trữ trạng thái.

- Agent A hoàn thành Task -> Bắn `TaskCompletedEvent`.
- Agent B lắng nghe Queue -> Bắt đầu làm việc.

## Tóm tắt
Hệ thống AI không chỉ có mô hình ML. Nó bao gồm Data Pipelines, GPU Orchestration, và Distributed State.
