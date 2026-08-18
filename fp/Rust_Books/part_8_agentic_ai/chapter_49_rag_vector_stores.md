# Chapter 49 — Retrieval-Augmented Generation (RAG) và Vector Stores

> **Bạn sẽ học được**:
> - Embeddings và cosine similarity — nền toán của RAG
> - Tích hợp vector store (LanceDB, Qdrant) qua trait của Rig
> - `.dynamic_context()` và `.dynamic_tools()` — nạp ngữ cảnh và tool theo nhu cầu
> - Re-ranking, Hybrid Search, RAG-as-Memory
>
> **Yêu cầu trước**: Chapter 48, Chapter 26 (Persistence)
> **Thời gian đọc**: ~30 phút | **Level**: Advanced
> **Kết quả cuối cùng**: RustyOps trả lời được từ runbook nội bộ thay vì bịa.

Trong các chương trước, chúng ta đã thảo luận về cách sử dụng LLM như một thành phần I/O ở "vành đai ngoài" (Imperative Shell) theo mô hình Hexagonal Architecture, đồng thời bảo vệ hệ thống bằng các kiểu dữ liệu mạnh mẽ của Rust. Tuy nhiên, LLM thường bị giới hạn bởi lượng kiến thức được huấn luyện và dễ bị "ảo giác" (hallucination) khi thiếu thông tin ngữ cảnh. 

Retrieval-Augmented Generation (RAG) là cơ chế giải quyết triệt để vấn đề này bằng cách kết nối LLM với các nguồn dữ liệu bên ngoài. Trong hệ sinh thái Rig, RAG không chỉ là một công cụ bổ sung mà được thiết kế như một quy trình luồng dữ liệu chuẩn hóa, tuân thủ nguyên tắc Type-Safety và Functional Programming.

## 49.1 — Bản Chất Của RAG: Từ Văn Bản Đến Vector (Embeddings)

Cốt lõi của RAG dựa trên hai khái niệm: **Embeddings** và **Cosine Similarity**.
- **Embeddings**: Biến đổi văn bản thành các vector số học mang ý nghĩa ngữ nghĩa.
- **Cosine similarity**: Đo lường khoảng cách giữa các vector để tìm ra sự tương đồng ngữ nghĩa.

Quy trình RAG tiêu chuẩn bao gồm hai giai đoạn:
1. **Ingestion (Nạp dữ liệu)**: Cắt tài liệu thành các chunk, tính toán embeddings và lưu trữ vào Vector Store.
2. **Retrieval (Truy xuất)**: Khi người dùng truy vấn, câu hỏi được chuyển thành vector (cùng một mô hình embedding), sau đó tìm kiếm các chunk có độ tương đồng cao nhất (Cosine Similarity) để nhúng vào Prompt.

## 49.2 — Abstraction Core của RAG trong Rig

Rig cung cấp hai trait chính để trừu tượng hóa quá trình này, giữ cho hệ thống luôn tuân thủ nguyên lý Dependency Inversion:
- `VectorStoreIndex`: Cho phép tìm kiếm tài liệu từ một Store.
- `InsertDocuments`: Hỗ trợ lưu trữ tài liệu vào Store.

Bằng cách sử dụng Traits, bạn có thể dễ dàng thay thế `InMemoryVectorStore` bằng các giải pháp Production-ready như LanceDB, Qdrant hoặc PostgreSQL (`pgvector`) mà không cần thay đổi logic lõi.

### Minimal RAG Agent

Dưới đây là một ví dụ minh họa việc thiết lập một RAG Agent đơn giản. Lưu ý cách các thao tác lỗi (như khởi tạo client, tạo embeddings) được xử lý thông qua `Result` (Railway-Oriented Programming).

```rust
use rig::client::{CompletionClient, EmbeddingsClient, ProviderClient};
use rig::completion::Prompt;
use rig::embeddings::EmbeddingsBuilder;
use rig::providers::openai::Client;
use rig::vector_store::in_memory_store::InMemoryVectorStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Khởi tạo Client (ROP: Bắt lỗi nếu thiếu API Key)
    let openai_client = Client::from_env()?;
    let embed_model = openai_client.embedding_model("text-embedding-3-small");

    // 2. Ingestion: Biến đổi văn bản thành Embeddings
    let embeddings = EmbeddingsBuilder::new(embed_model.clone())
        .documents(vec![
            "Rig là một thư viện Rust để xây dựng ứng dụng LLM.",
            "RAG kết hợp truy xuất và sinh văn bản để tăng độ chính xác.",
            "Vector stores hỗ trợ tìm kiếm ngữ nghĩa.",
        ])?
        .build()
        .await?;

    // 3. Khởi tạo Vector Store và Index
    let mut vector_store = InMemoryVectorStore::default();
    vector_store.add_documents(embeddings);
    let index = vector_store.index(embed_model);

    // 4. Khởi tạo Agent với Dynamic Context
    let agent = openai_client
        .agent("gpt-4o")
        .preamble("Bạn là một trợ lý ảo, hãy trả lời dựa trên ngữ cảnh được cung cấp.")
        .dynamic_context(2, index) // Truy xuất 2 tài liệu phù hợp nhất
        .build();

    // 5. Truy vấn
    let response = agent.prompt("Rig là gì và RAG hoạt động ra sao?").await?;
    println!("Response: {}", response);

    Ok(())
}
```

Hàm `dynamic_context(n, index)` cấu hình Agent để tự động lấy `n` tài liệu liên quan nhất và nhúng vào ngữ cảnh (context) trước khi gọi LLM. Việc này ẩn giấu hoàn toàn độ phức tạp của Retrieval Phase.

## 49.3 — Tool-RAG: Cấp Phát Tool Động

Một trong những tính năng mạnh mẽ nhất của Rig là **Dynamic Tool Retrieval (Tool-RAG)**. 
Khi ứng dụng của bạn lớn lên, Agent có thể cần sử dụng hàng chục Tool. Việc nhồi nhét tất cả các Tool vào System Prompt sẽ làm cạn kiệt Context Window và giảm độ chính xác của LLM.

Để giải quyết, Rig xem bản thân các Tool như những tài liệu (Documents) bằng cách bắt buộc implement trait `ToolEmbedding`. Tại thời điểm chạy (runtime), Agent sẽ sử dụng RAG để tìm ra các Tool phù hợp nhất với truy vấn của người dùng.

```rust
use rig::tool::ToolSet;

// Giả sử Adder là một Tool đã được định nghĩa
let toolset = ToolSet::builder().dynamic_tool(Adder).build();

// Chuyển đổi Schema của các Tool thành Embeddings
let embeddings = EmbeddingsBuilder::new(embed_model.clone())
    .documents(toolset.schemas()?)?
    .build()
    .await?;

let vector_store = InMemoryVectorStore::from_documents_with_id_f(
    embeddings, 
    |tool| tool.name.clone()
);
let index = vector_store.index(embed_model);

// Agent chỉ load tối đa 2 tool phù hợp nhất cho mỗi truy vấn
let agent = openai_client
    .agent("gpt-4o")
    .preamble("Bạn là một máy tính. Dùng các tool được cung cấp để trả lời.")
    .dynamic_tools(2, index, toolset)
    .build();
```

Thiết kế này thể hiện rõ triết lý Functional: Hệ thống chia nhỏ các khả năng (Tools), biến đổi chúng qua một Pipeline (Embeddings) và động đúc kết (Dynamic Composition) tùy vào Input.

## 49.4 — Các Mẫu RAG Hiện Đại (Modern RAG Patterns)

Để xây dựng hệ thống cấp Production, kiến trúc RAG thường kết hợp nhiều kỹ thuật nâng cao:

1. **Re-ranking**: Vector Search thông thường chỉ dựa trên Cosine Similarity có thể bỏ sót sự tinh tế của ngữ cảnh. Bạn có thể kết hợp thư viện `fastembed` trong Rust để thực hiện Re-ranking, sắp xếp lại các kết quả từ Vector Store để đạt độ chính xác cao hơn.
2. **Hybrid Search**: Kết hợp tìm kiếm toàn văn bản (Full-text search) như Elasticsearch và tìm kiếm ngữ nghĩa (Semantic search) thông qua Reciprocal Rank Fusion (RRF).
3. **RAG as Memory**: Biến RAG thành bộ nhớ dài hạn (Long-term memory) cho Agent, lưu trữ lịch sử hội thoại và sự kiện thay vì chỉ lưu tài liệu tĩnh. 

---

## ✅ Checkpoint 49

1. RAG chữa được hallucination, hay chỉ giảm bớt? Vì sao?
2. `.dynamic_tools()` tiết kiệm gì so với việc đăng ký sẵn 50 tool?
3. Chunk 200 token và chunk 2.000 token — mỗi bên đánh đổi điều gì?

<details>
<summary>Đáp án</summary>

1. Chỉ **giảm**. RAG cấp thêm ngữ cảnh đúng, nhưng model vẫn có thể diễn giải sai hoặc bịa thêm phần không có trong ngữ cảnh. Muốn chặt hơn thì phải bắt trích dẫn nguồn và kiểm tra câu trả lời có bám nguồn không.
2. Tiết kiệm **token đầu vào và độ chính xác**. Mọi định nghĩa tool đều nằm trong prompt; 50 tool là hàng nghìn token mỗi request, và model càng nhiều lựa chọn thì càng dễ chọn sai.
3. Chunk nhỏ: khớp chính xác hơn nhưng dễ mất ngữ cảnh xung quanh (câu trả lời bị cắt đôi). Chunk lớn: giữ ngữ cảnh nhưng loãng — vector đại diện cho quá nhiều ý nên độ tương đồng kém phân biệt. Thực tế hay dùng chunk vừa + overlap.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Nạp 5 file runbook markdown vào vector store, rồi hỏi một câu mà câu trả lời **không** có trong đó. Agent nói "không biết" hay bịa? Sửa preamble để nó buộc phải thừa nhận khi thiếu thông tin.

**Bài 2 (15 phút).** So sánh `top_k = 1` và `top_k = 5` trên cùng bộ câu hỏi. Ghi lại chất lượng câu trả lời và số token tiêu thụ. Có điểm nào tăng `top_k` không còn cải thiện gì không?

**Bài 3 (25 phút).** Thêm trích dẫn: sửa để agent phải nêu tên file nguồn cho mỗi khẳng định. Dùng Extractor với struct `{ answer: String, sources: Vec<String> }` — kết hợp Chapter 46 với chương này.

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| Kết quả retrieval không liên quan | Chunk quá lớn, vector bị loãng | Giảm chunk size, thêm overlap |
| `dimension mismatch` khi query | Đổi embedding model nhưng không index lại | Index lại toàn bộ; số chiều gắn liền với model |
| RAG chậm | Chưa có index ANN, đang quét tuyến tính | Bật HNSW/IVF trên vector store |
| Agent bỏ qua ngữ cảnh vừa nạp | Preamble không yêu cầu ưu tiên ngữ cảnh | Ghi rõ "chỉ trả lời dựa trên ngữ cảnh được cấp" |
| Kết quả tốt lúc test, kém lúc thật | Câu hỏi thật dài và nhiều ý hơn | Thử query rewriting hoặc hybrid search |

## Tóm tắt
RAG trong Rig không phải là một module "rời rạc" mà tích hợp sâu vào kiến trúc cốt lõi thông qua hệ thống Trait mạnh mẽ của Rust. Từ việc nhúng Context bằng `.dynamic_context()` đến cấp phát Tool động bằng `.dynamic_tools()`, Rig mang lại sự linh hoạt tối đa cho các kỹ sư muốn xây dựng Agent thông minh, tiết kiệm chi phí và an toàn bộ nhớ.
