# Chapter 45 — Bản chất của Agentic AI & Kiến trúc Rig

> **Bạn sẽ học được**:
> - Vì sao Rust là lựa chọn hợp lý cho tầng điều phối (orchestration) của AI
> - Ba trait lõi của Rig: `CompletionModel`, `Tool`, `Agent`
> - Dựng project RustyOps và gọi LLM đầu tiên
> - Vì sao "type-safety tại network boundary" quan trọng hơn bạn nghĩ
>
> **Yêu cầu trước**: Chapter 16 (Traits), Chapter 25 (serde), Chapter 36 (async/tokio)
> **Thời gian đọc**: ~35 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Một agent Rust chạy được, và hiểu rõ kiến trúc trait phía dưới nó.

> "Viết AI bằng Python giống như nặn đất sét: nhanh, linh hoạt, nhưng khi hệ thống phình to, nó dễ dàng đổ sụp. Viết AI bằng Rust giống như đúc bê tông cốt thép: chặt chẽ, vững chãi, và trường tồn."

Trong kỷ nguyên của Large Language Models (LLMs), AI không còn chỉ là một hộp thoại "Hỏi - Đáp". Sự tiến hóa tiếp theo chính là **Agentic AI** (Trí tuệ Nhân tạo Tự chủ) — nơi AI có quyền truy cập vào công cụ (Tools), có trí nhớ (Memory), và có khả năng tự lên kế hoạch (Planning) để giải quyết các vấn đề phức tạp trong thế giới thực.

Tuy nhiên, khi cấp quyền cho AI tương tác với hệ thống (như quyền đọc/ghi Database, cấp lệnh khởi động lại Server), chúng ta phải đối mặt với rủi ro khổng lồ: **Làm sao để đảm bảo AI trả về dữ liệu đúng định dạng và không thực thi các lệnh sai lệch?**

Đây là lúc Rust tỏa sáng.

---

## 45.1 — Tại sao lại là Rust cho AI Orchestration?

Trong các hệ thống Agentic AI (như LangChain hay AutoGen của Python), một trong những lỗi đau đầu nhất là **Parsing Error**: LLM trả về một chuỗi JSON thiếu dấu phẩy, thiếu một field, hoặc sai kiểu dữ liệu (trả về String thay vì Integer). Python, với bản chất *dynamic typing*, thường sẽ để lọt lỗi này cho đến khi nó đâm sầm (crash) vào hệ thống cơ sở dữ liệu.

Rust giải quyết bài toán này từ gốc thông qua **Type-Safety by Construction**:
1. **Strongly Typed JSON:** Nhờ `serde`, nếu LLM trả về sai cấu trúc, quá trình *deserialize* sẽ thất bại ngay lập tức ở ranh giới mạng (network boundary), không bao giờ lọt được vào Business Logic của ứng dụng.
2. **Concurrency không giới hạn:** Phần lớn thời gian AI Agent hoạt động là chờ đợi (I/O bound) từ API của LLM hoặc từ các Tools. Mô hình async/await của `tokio` cho phép một server Rust có thể điều phối hàng chục ngàn Agents cùng lúc mà chỉ tốn vài chục MB RAM.
3. **Triển khai nhẹ nhàng (Lean Deployment):** Một Agent viết bằng Rust được build thành một file nhị phân (binary) duy nhất, có thể nhúng vào AWS Lambda hoặc chạy trên các thiết bị Edge/IoT với tốc độ khởi động (Cold Start) gần như bằng không.

---

## 45.2 — Giải phẫu kiến trúc Rig (The Rig Architecture)

> **📌 Phiên bản API dùng trong Part VIII:** toàn bộ code ở đây bám theo
> **Rig 0.41** (`rig-core` + `rig-agent`). Rig còn đang phát triển nhanh và đã
> nhiều lần đổi API — trước khi copy code, hãy đối chiếu với
> [docs.rs/rig-core](https://docs.rs/rig-core) đúng phiên bản bạn cài.

[Rig](https://rig.rs/) không phải là một bộ thư viện đồ sộ, "ôm đồm" mọi thứ như LangChain. Triết lý của Rig cực kỳ mang tính "Rust": **Cung cấp các Traits lõi (Core Traits) mỏng, linh hoạt, và có tính kết dính cao.**

Hãy cùng mổ xẻ các Traits cấu thành nên hệ thống của Rig:

### 1. `CompletionModel` (Trái tim của LLM)
Trait này định nghĩa bất kỳ thực thể nào có khả năng nhận vào một luồng tin nhắn (Chat History) và sinh ra phản hồi. Dù bạn dùng OpenAI (`gpt-4o`), Anthropic (`claude-3-5-sonnet`), hay chạy mô hình cục bộ với `llama.cpp`, tất cả đều phải implement trait này.

```rust
// rig-core 0.41 — crates/rig-core/src/completion/request.rs
pub trait CompletionModel: Clone + WasmCompatSend + WasmCompatSync {
    /// Kiểu response thô của provider (OpenAI, Anthropic, ...).
    type Response: WasmCompatSend + WasmCompatSync + Serialize + DeserializeOwned;
    /// Kiểu response khi stream từng token.
    type StreamingResponse: Clone + Unpin + Serialize + DeserializeOwned + GetTokenUsage /* + ... */;
    /// Client của provider dùng để dựng model handle này.
    type Client;

    fn make(client: &Self::Client, model: impl Into<String>) -> Self;

    fn completion(&self, request: CompletionRequest)
        -> impl Future<Output = Result<CompletionResponse<Self::Response>, CompletionError>>;
}
```

> **Lưu ý về async:** Rig **không** dùng `#[async_trait]`. Từ Rust 1.75, trait có thể
> trả thẳng `impl Future` — tránh được một lần cấp phát `Box<dyn Future>` cho mỗi
> lời gọi. `WasmCompatSend`/`WasmCompatSync` là alias của `Send`/`Sync` trên native,
> và rỗng trên `wasm32` (nơi không có đa luồng).
Nhờ trait này, code ứng dụng của bạn không bị "khóa chặt" (vendor lock-in) vào bất kỳ hãng AI nào. Việc chuyển đổi từ OpenAI sang một model mã nguồn mở chạy local chỉ mất đúng 1 dòng code cấu hình.

### 2. `Tool` (Bàn tay của Agent)
Để AI có thể hành động, nó cần công cụ. Trait `Tool` trong Rig bắt buộc bạn phải định nghĩa rõ:
- Tên công cụ (Name).
- Mô tả chi tiết (Description) để LLM biết khi nào nên dùng.
- Tham số đầu vào (Args) được kiểm tra chặt chẽ bởi hệ thống Type của Rust.

```rust
// rig-agent 0.41 — crates/rig-agent/src/tool/mod.rs
pub trait Tool: Sized + WasmCompatSend + WasmCompatSync {
    /// Tên duy nhất, cũng là tên LLM nhìn thấy.
    const NAME: &'static str;
    /// Tham số vào, deserialize từ JSON do LLM sinh ra.
    type Args: for<'de> Deserialize<'de> + WasmCompatSend + WasmCompatSync;
    /// Kết quả trả về; mọi giá trị Serialize đều tự động thoả `IntoToolOutput`.
    type Output: IntoToolOutput;
    /// Lỗi có kiểu của riêng tool — `?` hoạt động bình thường trong `call`.
    type Error: std::error::Error + WasmCompatSend + WasmCompatSync + 'static;

    /// Mô tả cho LLM biết khi nào nên dùng tool này.
    fn description(&self) -> String;
    /// JSON Schema của `Args`.
    fn parameters(&self) -> serde_json::Value;

    fn call(&self, context: &mut ToolContext, args: Self::Args)
        -> impl Future<Output = Result<Self::Output, Self::Error>> + WasmCompatSend;
}
```

> **Ba điểm dễ nhầm:** (1) `Tool` nằm ở crate `rig-agent`, không phải `rig-core`.
> (2) `call` nhận thêm `&mut ToolContext` — đây là chỗ tool đọc/ghi state của
> phiên chạy. (3) `parameters()` là method bạn phải cung cấp; macro `#[tool]`
> sinh nó tự động từ `schemars::JsonSchema`.
Lưu ý ràng buộc `DeserializeOwned` và `JsonSchema`. Rig sẽ tự động chuyển đổi struct Rust của bạn thành một JSON Schema chuẩn hóa để gửi cho LLM. Nếu LLM trả về sai schema, Rig sẽ tự động báo lỗi hoặc yêu cầu LLM sinh lại. Đỉnh cao của sự an toàn!

### 3. `Agent` (Bộ não điều phối)
Agent là nơi kết hợp `CompletionModel` và một tập hợp các `Tool`. Nó quản lý ngữ cảnh, duy trì vòng lặp suy nghĩ-hành động (ReAct loop) và tương tác với người dùng.

---

## 45.3 — Dự án xuyên suốt: "RustyOps" (DevOps AI Assistant)

Để học cách sử dụng Rig, chúng ta sẽ không viết các ví dụ "cộng trừ nhân chia" nhàm chán. Từ chương này đến hết Part 8, chúng ta sẽ cùng nhau xây dựng **RustyOps** — một Agent tự động hóa DevOps.

RustyOps sẽ có khả năng:
- Tra cứu Log hệ thống từ Server (Sử dụng Tools).
- Đọc tài liệu Runbook nội bộ (Sử dụng Vector Database & RAG).
- Xin quyền phê duyệt từ Admin trước khi Restart Server (Sử dụng Stateful Graph).

### Bước 1: Khởi tạo dự án và Cấu hình

Khởi tạo một dự án mới:
```bash
cargo new rustyops
cd rustyops
```

Cập nhật `Cargo.toml` với các crates cần thiết từ hệ sinh thái Rig:
```toml
[dependencies]
# Trái tim của hệ thống: provider clients, completion, embeddings
rig-core = "0.41"
# Agent loop, trait `Tool`, ToolContext — tách khỏi rig-core từ 0.4x
rig-agent = "0.41"

# Orchestrator
tokio = { version = "1.0", features = ["full"] }

# Quản lý lỗi và nạp biến môi trường
anyhow = "1.0"
dotenvy = "0.15"

# Serialize dữ liệu
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Bước 2: Viết dòng code đầu tiên (Khởi động Não bộ)

Trước khi trang bị Tools cho RustyOps, chúng ta cần cấp cho nó một "bộ não" (CompletionModel) và thiết lập tính cách (Preamble) cho nó. Tạo file `.env` chứa `OPENAI_API_KEY=sk-...` và viết đoạn mã sau vào `src/main.rs`:

```rust
use rig::providers::openai;
use rig::completion::Prompt;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Nạp biến môi trường an toàn
    dotenv().ok();
    
    // 2. Khởi tạo Provider (Hãng cung cấp AI). 
    // `from_env` tự động tìm OPENAI_API_KEY.
    let openai_client = openai::Client::from_env();

    // 3. Khởi tạo Agent (Bộ não) bằng Builder Pattern
    let rustyops_agent = openai_client
        .agent("gpt-4o-mini") // Chọn model
        .preamble(
            "Bạn là RustyOps, một kỹ sư DevOps dày dặn kinh nghiệm, chuyên về hệ thống Linux và Cloud. \
             Câu trả lời của bạn cần ngắn gọn, đi thẳng vào vấn đề kỹ thuật."
        )
        .max_tokens(1024)
        .temperature(0.5) // Giữ mức tính ngẫu nhiên thấp để AI trả lời chính xác kỹ thuật
        .build();

    // 4. Tương tác thử nghiệm
    println!("Đang hỏi RustyOps...");
    let response = rustyops_agent
        .prompt("Lỗi 'OOMKilled' trên Kubernetes thường do nguyên nhân gì?")
        .await?;

    println!("RustyOps: {}", response);

    Ok(())
}
```

Hãy chạy ứng dụng bằng `cargo run`. Bạn sẽ thấy Agent phản hồi với phong cách của một kỹ sư DevOps chuyên nghiệp. Tuy nhiên, lúc này Agent vẫn bị "mù và điếc" — nó chỉ có thể trả lời dựa trên kiến thức được huấn luyện từ trước (Pre-trained knowledge) chứ không hề biết server của bạn đang chạy hệ điều hành gì hay file log đang báo lỗi ra sao.

Trong **Chương 46**, chúng ta sẽ chính thức cấp "mắt" và "tay" cho RustyOps bằng cách implement các `Tool` để nó có thể thọc sâu vào hệ thống của bạn.

---

## ✅ Checkpoint 45

1. `CompletionModel` yêu cầu associated type `Client` và hàm `make()`. Điều đó ép ta thiết kế provider như thế nào?
2. Vì sao Rig trả `impl Future` thay vì dùng `#[async_trait]`?
3. `Tool::Args` chỉ đòi `Deserialize`, nhưng schema gửi cho LLM lại đến từ `parameters()`. Ai chịu trách nhiệm giữ hai thứ đó khớp nhau?

<details>
<summary>Đáp án</summary>

1. Model **không** tự tạo kết nối — nó luôn được dựng từ một client sẵn có. Nhờ vậy nhiều model dùng chung một connection pool, và `Clone` trên model là rẻ.
2. `#[async_trait]` bọc mỗi lời gọi trong `Box<dyn Future>` — một lần cấp phát heap cho mỗi lần gọi. Từ Rust 1.75, trait trả thẳng `impl Future` được nên không cần cái giá đó nữa.
3. Nếu bạn tự implement `Tool` thì **bạn** chịu. Nếu dùng `#[rig_tool]`, macro sinh `parameters()` từ `schemars::JsonSchema` của chính kiểu `Args` — đó là lý do nên ưu tiên macro.
</details>

---

## 🏋️ Bài tập

**Bài 1 (5 phút).** Sửa `main.rs` để đọc tên model từ biến môi trường `RUSTYOPS_MODEL`, mặc định `gpt-4o` khi biến không tồn tại. Dùng `std::env::var(...).unwrap_or_else(...)`.

**Bài 2 (10 phút).** Viết `preamble` cho RustyOps sao cho agent **luôn** từ chối mọi yêu cầu xoá dữ liệu, và trả lời bằng tiếng Việt. Thử prompt "xoá toàn bộ log" và xem preamble có giữ được không.

**Bài 3 (15 phút).** `CompletionModel` yêu cầu `Clone`. Hãy giải thích bằng lời (không cần code): điều gì sẽ hỏng nếu Rig bỏ ràng buộc `Clone` khỏi trait này, khi một `Agent` cần chạy nhiều request song song bằng `tokio::spawn`?

<details>
<summary>Gợi ý bài 3</summary>

`tokio::spawn` đòi future phải `'static` — nó không mượn được từ stack của caller. Không có `Clone`, mỗi task phải chia sẻ model qua `Arc`, và mọi API sẽ phải nhận `Arc<M>` thay vì `M`. `Clone` (rẻ, vì model chỉ giữ handle) khiến chuyện đó biến mất khỏi API bề mặt.
</details>

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| `error[E0432]: unresolved import rig_agent` | Thiếu crate `rig-agent` trong `Cargo.toml` | Từ 0.4x, `Tool` và `AgentBuilder` nằm ở `rig-agent`, không phải `rig-core` |
| `OPENAI_API_KEY not set` lúc chạy | Chưa gọi `dotenvy::dotenv()` trước khi tạo client | Đặt `dotenvy::dotenv().ok();` ở dòng đầu `main()` |
| Biên dịch được nhưng treo mãi | Thiếu `#[tokio::main]` hoặc thiếu feature `full` của tokio | `tokio = { version = "1", features = ["full"] }` |
| API đổi so với sách | Rig bump minor rất nhanh | Đối chiếu `docs.rs/rig-core` đúng version bạn cài; sách bám 0.41 |

---

## Tóm tắt

- **Rust cho tầng orchestration, không phải tầng training.** Giá trị nằm ở
  type-safety tại network boundary, concurrency rẻ, và binary triển khai gọn.
- **Rig mỏng, không "ôm đồm".** Ba trait lõi — `CompletionModel` (bộ não),
  `Tool` (bàn tay), `Agent` (điều phối) — và bạn ghép chúng lại.
- `CompletionModel` sống ở `rig-core`; `Tool` và `AgentBuilder` ở `rig-agent`.
- Không có `#[async_trait]`: trait trả thẳng `impl Future`, tiết kiệm một lần
  cấp phát heap mỗi lời gọi.
- Nhờ trait, đổi provider (OpenAI → model local) chỉ là đổi một dòng khởi tạo —
  đúng tinh thần "phụ thuộc vào abstraction" của Chapter 21.

## Tiếp theo

Agent hiện trả về `String`. Chương sau biến nó thành dữ liệu **có kiểu** bằng
Extractor → **[Chapter 46 — Core Abstractions: Extractors và Streaming](chapter_46_extractors_streaming.md)**.
