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

---

## ✅ Checkpoint 39

1. Vì sao không nên cho client gọi thẳng vLLM mà phải qua một AI Gateway?
2. Hệ AI khác hệ web truyền thống ở ba ràng buộc nào?
3. Trong RAG quy mô lớn, khâu nào thường là nút cổ chai — embedding, vector search, hay LLM inference?

<details>
<summary>Đáp án</summary>

1. Vì Gateway là chỗ đặt xác thực, rate limit theo token (không phải theo request), tính chi phí, ghi log prompt, kiểm duyệt nội dung và định tuyến model. Không có nó thì mọi chính sách phải nhét vào từng client.
2. (a) **Độ trễ**: một request tính bằng giây–chục giây, không phải mili-giây; (b) **Chi phí**: GPU đắt gấp trăm lần CPU nên batching quyết định kinh tế; (c) **Không tất định**: cùng đầu vào cho ra đầu ra khác nhau, nên test và cache đều phải nghĩ lại.
3. Hầu như luôn là **LLM inference** — nó chiếm phần lớn thời gian và gần như toàn bộ chi phí. Embedding và vector search thường tính bằng mili-giây. Vì vậy tối ưu nên bắt đầu ở caching và batching phía LLM.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Thêm rate limit **theo token** (không phải theo request) vào AI Gateway. Vì sao giới hạn theo request là sai chỗ này?

**Bài 2 (15 phút).** Cài semantic cache: trước khi gọi LLM, tìm trong vector store xem có prompt tương tự (cosine > 0,95) đã trả lời chưa. Đo tỉ lệ cache hit trên một tập câu hỏi thật.

**Bài 3 (25 phút).** Thiết kế (bằng sơ đồ + kiểu dữ liệu) một hàng đợi ưu tiên cho request LLM: người dùng trả phí được phục vụ trước. Xử lý cả trường hợp starvation của hàng đợi thấp.

<details>
<summary>Gợi ý bài 1</summary>

Một request "tóm tắt 3 câu" và một request "phân tích 200 trang PDF" đều là **một**
request, nhưng chênh nhau hàng nghìn lần về chi phí. Rate limit theo request sẽ
vừa quá chặt với người dùng nhẹ, vừa quá lỏng với người dùng nặng.
</details>

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| GPU OOM khi tải cao | Batch size / context length quá lớn | Giảm `max_model_len`, bật paged attention của vLLM |
| Độ trễ cao dù GPU rảnh | Request xử lý tuần tự | Bật continuous batching |
| Chi phí tăng bất thường | Không cache, prompt lặp lại nhiều | Thêm semantic cache + prompt caching |
| Kết quả RAG lệch chủ đề | Chunking kém hoặc thiếu re-ranking | Giảm chunk size, thêm bước re-rank |
| Timeout ở tầng gateway | Request LLM dài hơn timeout mặc định | Tăng timeout **và** chuyển sang streaming |

## Tóm tắt

- Các thành phần của Hệ thống AI: Backend App (FastAPI), GPU Inference (vLLM), Task Queue (Celery/Kafka), và Vector DB (Qdrant).
- Để AI scale mạnh mẽ, áp dụng tư tưởng Microservices và Event-Driven Architecture.
- RAG mức Enterprise phức tạp hơn rất nhiều so với vài dòng code Langchain, đòi hỏi Hybrid Search, Reranking và Async Ingestion.

## Tiếp theo
Khung thiết kế phần mềm, từ Code Architecture đến System Architecture của bạn đã hoàn thiện! Chương cuối cùng, **Chapter 40 (Capstone)**, sẽ hướng dẫn bạn gói ghém một backend FastAPI AI hoàn chỉnh chuẩn bị lên Production.
