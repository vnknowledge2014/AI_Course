# Chapter 39 — AI System Design & Infrastructure

> **Bạn sẽ học được**:
> - Kiến trúc của một hệ thống LLM Production lớn
> - LLM Inference Server (vLLM, TensorRT-LLM)
> - Thiết kế AI Gateway bằng FastAPI
> - RAG Pipeline Scale lớn và Vector Databases
> - Tương tác đa tác tử (Multi-Agent) bằng Message Queues
>
> **Yêu cầu trước**: Chapter 38 (System Design Thinking)
> **Thời gian đọc**: ~45 phút | **Level**: Principal

---

## 39.1 — Cấu trúc phân tầng của hệ thống AI (AI Stack)

Một dự án AI thực tế không chỉ có một file `script.py` chạy Langchain. Để phục vụ triệu người dùng, ta thiết kế hệ thống theo 3 tầng (3-tier architecture for AI):

1. **Inference Layer**: Tầng tính toán GPU thấp nhất. (vLLM, Ollama, Triton Inference Server).
2. **AI Gateway Layer**: Proxy đứng trước GPU để bảo mật và định tuyến (FastAPI, Redis).
3. **Orchestration Layer**: Quản lý Agent logic, RAG pipelines, Reranking, và Prompt Management.

---

## 39.2 — Inference Server (vLLM)

Tại sao không dùng `transformers` pipeline mặc định của HuggingFace trên server production?
Vì HuggingFace `pipeline` tạo ra KV Cache khổng lồ liên tiếp và phân mảnh VRAM. Kết quả: Nó xử lý được rất ít requests đồng thời (Concurrent Requests).

**Giải pháp: vLLM**
vLLM sử dụng **PagedAttention** — vay mượn khái niệm Virtual Memory (Bộ nhớ ảo) của hệ điều hành. KV Cache được chia thành các mảnh (Blocks) nhỏ để tái sử dụng. Tốc độ Inference tăng gấp 24 lần.

Khởi chạy vLLM server:
```bash
# vLLM chạy độc lập, tự động wrap model HuggingFace thành 1 HTTP API Server
python -m vllm.entrypoints.openai.api_server \
    --model meta-llama/Llama-3-8B \
    --gpu-memory-utilization 0.9 \
    --max-model-len 4096
```

---

## 39.3 — AI Gateway bằng FastAPI

FastAPI đứng ở giữa User và vLLM. Chức năng chính:
- **Xác thực API Key / OAuth 2.0.**
- **Rate Limiting**: Ngăn chặn User gọi API liên tục làm kiệt quệ GPU.
- **Load Balancing**: Đẩy request tới các server vLLM khác nhau.
- **Auditing/Billing**: Ghi lại lượng tokens đã xài để trừ tiền.

```python
from fastapi import FastAPI, Request
from fastapi.responses import StreamingResponse
import httpx
import asyncio

app = FastAPI()
gpu_client = httpx.AsyncClient(base_url="http://vllm-cluster:8000/v1")

@app.post("/api/chat")
async def chat_endpoint(request: Request):
    payload = await request.json()
    user_id = "user_123" # Lấy từ Auth Header
    
    # 1. Gọi Redis Rate Limiter
    # 2. Định tuyến (Routing) Request
    
    # 3. Stream phản hồi từ GPU server
    async def stream_generator():
        async with gpu_client.stream("POST", "/chat/completions", json=payload) as resp:
            async for chunk in resp.aiter_bytes():
                yield chunk
                
    # 4. Ghi nhận Token Usage qua Background Tasks (Dramatiq/Celery)
                
    return StreamingResponse(stream_generator(), media_type="text/event-stream")
```

---

## 39.4 — RAG System Scale Lớn (Enterprise RAG)

Khi làm RAG cho một công ty có 1 triệu trang tài liệu, bạn không thể load ChromaDB vào bộ nhớ được nữa.

**1. Data Ingestion Pipeline (Event-Driven)**
- User upload PDF.
- FastAPI bắn Event vào **Kafka** hoặc **RabbitMQ**.
- Background Worker lắng nghe: Cắt text -> Chạy model Embedding (VD: `bge-m3`) -> Đẩy vào Database.

**2. Vector Database Sharding**
- Sử dụng Qdrant, Milvus hoặc Pinecone. 
- Thiết kế **Tenant Isolation**: Dữ liệu của khách hàng A không bao giờ được search trộn lẫn vào khách hàng B (Sử dụng Filters Metadata của VectorDB).

**3. Hybrid Search & Reranker**
- Kết hợp Vector Search (Độ tương đồng ngữ nghĩa) với Keyword Search (BM25 - ElasticSearch).
- Dùng một model nhỏ (Reranker như BGE-Reranker) chấm điểm lại 20 kết quả thu được trước khi nhét vào Prompt.

---

## 39.5 — Multi-Agent System (Event Sourcing)

Khi nhiều Agent (Lập trình viên, Tester, Reviewer) cần tương tác liên tục: Không nên dùng HTTP gọi nhau trực tiếp (Synchronous), vì Agent nghĩ rất lâu (có thể tốn 5 phút). 

Áp dụng **Event Sourcing** (Chapter 17):
1. **Agent Coder** viết code xong -> Bắn sự kiện `CodeWrittenEvent(task_id)`.
2. **Agent Tester** bắt được event -> Chạy test sandbox -> Bắn `TestFailedEvent`.
3. **Agent Coder** nhận lỗi -> Sửa code.

Quản lý luồng này bằng Apache Kafka hoặc Temporal Workflow, hệ thống của bạn sẽ chạy bất đồng bộ trơn tru và dễ dàng mở rộng.

---

## Tóm tắt

- Các thành phần của Hệ thống AI: Backend App (FastAPI), GPU Inference (vLLM), Task Queue (Celery/Kafka), và Vector DB (Qdrant).
- Để AI scale mạnh mẽ, áp dụng tư tưởng Microservices và Event-Driven Architecture.
- RAG mức Enterprise phức tạp hơn rất nhiều so với vài dòng code Langchain, đòi hỏi Hybrid Search, Reranking và Async Ingestion.

## Tiếp theo
Khung thiết kế phần mềm, từ Code Architecture đến System Architecture của bạn đã hoàn thiện! Chương cuối cùng, **Chapter 40 (Capstone)**, sẽ hướng dẫn bạn gói ghém một backend FastAPI AI hoàn chỉnh chuẩn bị lên Production.
