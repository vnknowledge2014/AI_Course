# Chapter 53 — Tích Hợp Mô Hình Cục Bộ (Local Models)

> **Bạn sẽ học được**:
> - Vì sao dữ liệu nhạy cảm buộc phải chạy model cục bộ
> - Tự implement `CompletionModel` — bài tập Traits thực tế nhất trong sách
> - `rig-candle`: chạy Llama / SmolLM2 / Qwen3 hoàn toàn bằng Rust
> - Khi nào vẫn nên dùng `llama.cpp` qua OpenAI-compatible server
> - ROP cho local model: model nhỏ sai schema nhiều hơn, và cách phục hồi
>
> **Yêu cầu trước**: Chapter 16 (Traits), Chapter 45, Chapter 47
> **Thời gian đọc**: ~30 phút | **Level**: Advanced
> **Kết quả cuối cùng**: RustyOps chạy được offline, không gửi byte nào ra ngoài.

Trong các chương trước, chúng ta đã xây dựng các Agent dựa trên các API thương mại (như OpenAI hay Anthropic). Mặc dù mạnh mẽ, các API này đi kèm với rủi ro về quyền riêng tư dữ liệu và chi phí trên mỗi token. Khi xây dựng các hệ thống AI cấp độ Production (Production-Grade AI), đặc biệt trong lĩnh vực tài chính, y tế, hay các dự án đòi hỏi bảo mật cao, khả năng chạy LLM cục bộ (Local Models) trở thành một yêu cầu thiết yếu.

Trong chương này, chúng ta sẽ áp dụng các nguyên lý cốt lõi của Rust (Type-Safety, Functional Programming, và Railway-Oriented Programming) để đưa các mô hình GGUF chạy trên CPU/GPU local vào hệ thống Rig, sử dụng provider local chính thức của hệ sinh thái: `rig-candle`.

## 53.1 — Bài Toán Bảo Mật và Quyền Riêng Tư

Trong Domain-Driven Design (DDD), chúng ta luôn muốn giữ "Core Domain" (lõi nghiệp vụ) tách biệt và an toàn. Khi gửi dữ liệu nhạy cảm ra ngoài qua API, chúng ta đang phá vỡ ranh giới bảo mật. Việc đưa mô hình AI về chạy cục bộ (on-premise hoặc edge) giúp chúng ta:

- **Bảo toàn dữ liệu (Data Privacy):** Dữ liệu không bao giờ rời khỏi server.
- **Giảm độ trễ mạng (Zero Network Latency):** Phản hồi nhanh hơn trong mạng nội bộ.
- **Chi phí dự đoán được (Predictable Costs):** Không bị tính phí theo token.

## 53.2 — Abstraction Core của Rig: `CompletionModel`

Rig được thiết kế theo tư duy Hexagonal Architecture (Ports and Adapters). Trong đó, Agent không cần biết nó đang nói chuyện với OpenAI hay Llama3 chạy local. Tất cả đều được abstract qua trait `CompletionModel`.

Đây là cách tư duy Interface-first (Trait-based) giúp hệ thống linh hoạt:

```rust
use rig_core::completion::{
    CompletionError, CompletionModel, CompletionRequest, CompletionResponse,
};
use std::future::Future;

/// Client sở hữu trọng số model đã nạp vào RAM/VRAM.
#[derive(Clone)]
pub struct LocalClient { /* weights, tokenizer, device ... */ }

/// Model handle — rẻ để clone, chỉ trỏ vào client.
#[derive(Clone)]
pub struct LocalLlamaModel {
    client: LocalClient,
    model_id: String,
}

impl CompletionModel for LocalLlamaModel {
    type Response = String;          // response thô của engine local
    type StreamingResponse = String; // từng token khi stream
    type Client = LocalClient;       // provider client dựng ra model này

    fn make(client: &Self::Client, model: impl Into<String>) -> Self {
        Self { client: client.clone(), model_id: model.into() }
    }

    fn completion(
        &self,
        request: CompletionRequest,
    ) -> impl Future<Output = Result<CompletionResponse<Self::Response>, CompletionError>> {
        async move {
            // Gọi vào engine local (Candle). Áp dụng ROP: lỗi engine -> CompletionError
            let raw = self
                .run_inference(&request)
                .await
                .map_err(|e| CompletionError::ProviderError(e.to_string()))?;

            Ok(CompletionResponse::from(raw))
        }
    }
}
```

> **So với `Tool`, `CompletionModel` khó implement hơn nhiều** vì bạn phải cung
> cấp cả `StreamingResponse` và `Client`. Chỉ tự viết khi bạn thực sự tích hợp
> một engine chưa có provider — còn lại hãy dùng crate có sẵn ở mục 53.3.

Nhờ cơ chế này, bạn chỉ cần "plug-and-play" model local vào `AgentBuilder` mà không phải sửa đổi bất kỳ logic nghiệp vụ nào của Agent.

## 53.3 — Chạy model cục bộ với `rig-candle`

Thay vì tự viết wrapper, hệ sinh thái Rig đã có crate provider sẵn.

> **⚠️ Cảnh báo nguồn tài liệu:** nhiều bài blog (và cả bản thảo trước của chương
> này) nhắc tới một crate tên `rig-llama-cpp`. **Crate đó không tồn tại trong
> workspace Rig 0.41.** Provider local chính thức là **`rig-candle`**:
> *"Local Candle Llama, SmolLM2, and Qwen3 completion models for Rig"*.
> Hãy luôn kiểm tra `crates/` trong repo Rig hoặc crates.io trước khi tin một tên crate.

### Rust-native với Candle

`candle` là framework tensor của HuggingFace, viết 100% bằng Rust — không đụng
tới C/C++ toolchain, không cần build `llama.cpp`. `rig-candle` bọc nó lại thành
một `CompletionModel` chuẩn của Rig.

```toml
[dependencies]
rig-core = "0.41"
rig-agent = "0.41"
rig-candle = "0.41"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

```rust
use rig_agent::AgentBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Nạp model cục bộ (Llama / SmolLM2 / Qwen3) qua Candle.
    // Trọng số tải từ HuggingFace Hub rồi cache lại ở lần chạy sau.
    let model = rig_candle::client()
        .completion_model("Qwen/Qwen3-0.6B")?;

    let agent = AgentBuilder::new(model)
        .preamble("Bạn là trợ lý DevOps an toàn, chạy hoàn toàn cục bộ.")
        .build();

    let response = agent.prompt("Kiểm tra trạng thái server").await?;
    println!("Agent (Local): {response}");
    Ok(())
}
```

Vì `rig-candle` implement đúng trait `CompletionModel` ở mục 53.2, phần còn lại
của RustyOps — Tools, Memory, Hooks, RAG — **không phải sửa một dòng nào**. Đây
chính là giá trị thực tế của việc lập trình theo trait thay vì theo class cụ thể.

### Khi nào vẫn nên dùng llama.cpp?

Candle chưa nhanh bằng `llama.cpp` ở khâu quantization cực thấp (Q2–Q4) trên CPU.
Nếu bạn bắt buộc cần GGUF quantized, cách làm hiện tại là chạy `llama.cpp` ở chế
độ **OpenAI-compatible server** rồi trỏ provider OpenAI của Rig vào đó:

```rust
// llama-server -m model.gguf --port 8080
let client = rig_core::providers::openai::Client::builder("not-needed")
    .base_url("http://localhost:8080/v1")
    .build()?;
let model = client.completion_model("local-gguf");
```

Đổi đúng một dòng `base_url` — toàn bộ Agent chạy y nguyên. Đó là lợi ích của
việc mọi provider đều quy về cùng một trait.

## 53.4 — Bắt Lỗi và Phục Hồi (ROP trong Local AI)

Local models thường có kích thước nhỏ hơn (7B - 8B params) so với GPT-4 (1T+ params), do đó khả năng chúng tuân thủ định dạng JSON (Structured Output) hoặc gọi Tool có thể kém hơn.

Đây là lúc ROP (Railway-Oriented Programming) tỏa sáng. Khi model trả về chuỗi JSON lỗi, Extractor của Rig sẽ trả về `Err`. Chúng ta có thể dùng lỗi này làm "feedback" nạp lại vào prompt để yêu cầu model tự sửa (Self-correction loop).

```rust
loop {
    match agent.prompt("Phân tích log và trả về định dạng JSON nghiêm ngặt").await {
        Ok(parsed_data) => {
            // Happy path
            process_data(parsed_data);
            break;
        },
        Err(e) => {
            // Error path: Phản hồi lỗi lại cho LLM
            println!("LLM trả về sai cấu trúc. Đang yêu cầu sửa lỗi...");
            agent.update_memory(format!("Lỗi parse JSON: {}. Hãy thử lại.", e));
        }
    }
}
```

---

## ✅ Checkpoint 53

1. `CompletionModel` đòi cả `Response` lẫn `StreamingResponse`. Vì sao không gộp làm một?
2. Vì sao chuyển sang model local **không** cần sửa Tools, Memory hay Hooks?
3. Model 7B tuân thủ JSON schema kém hơn GPT-4o. Cơ chế nào của Rig gánh phần chênh lệch đó?

<details>
<summary>Đáp án</summary>

1. Vì hai chế độ trả về hình dạng khác nhau: chế độ thường trả một response hoàn chỉnh, chế độ stream trả từng mảnh cộng thông tin token usage tích luỹ. Ép chung một kiểu sẽ buộc mọi provider phải giả lập cái mình không có.
2. Vì tất cả chúng phụ thuộc vào **trait**, không phụ thuộc provider cụ thể. Đây chính xác là Dependency Inversion của Chapter 21 — và là lý do đáng để học trait cho tử tế.
3. Extractor (Ch46) trả `Err` khi schema sai, rồi self-correction loop (Ch47) nạp lỗi ngược lại làm prompt. Với model nhỏ, bạn thường phải tăng số lần thử — nhưng cơ chế thì y hệt.
</details>

---

## 🏋️ Bài tập

**Bài 1 (15 phút).** Chạy RustyOps với Qwen3-0.6B qua `rig-candle`. So sánh chất lượng và độ trễ với `gpt-4o` trên cùng 5 prompt.

**Bài 2 (20 phút).** Bật `llama.cpp` ở chế độ OpenAI-compatible server và trỏ provider OpenAI của Rig vào `http://localhost:8080/v1`. Xác nhận rằng bạn chỉ phải đổi đúng một dòng `base_url`.

**Bài 3 (40 phút).** Viết một `CompletionModel` giả lập (`MockModel`) trả về câu trả lời cố định — dùng để test agent trong CI mà không tốn tiền API và không cần mạng. Đây là ứng dụng trực tiếp của mocking ở Chapter 35.

<details>
<summary>Gợi ý bài 3</summary>

`MockModel` chỉ cần `Response = String`, `StreamingResponse = String`, `Client = ()`.
`make()` bỏ qua client, `completion()` trả sẵn `CompletionResponse`. Đây là bài
tập tốt nhất để hiểu vì sao trait lại có ba associated type đó — bạn buộc phải
điền hết chúng.
</details>

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| `rig-llama-cpp` không tìm thấy trên crates.io | Crate đó không tồn tại | Dùng `rig-candle`, hoặc llama.cpp qua OpenAI-compatible server |
| Lần chạy đầu rất lâu | Đang tải trọng số từ HuggingFace Hub | Bình thường; lần sau lấy từ cache |
| Hết RAM khi nạp model | Model quá lớn so với máy | Dùng bản nhỏ hơn (0.6B/1.5B) hoặc bản đã quantize |
| Model local bỏ qua tool | Model nhỏ yếu về tool-calling | Chọn model có fine-tune cho tool use; giảm số tool cấp cùng lúc |
| Output JSON hỏng liên tục | Model nhỏ bám schema kém | Tăng số lần retry trong self-correction loop; đơn giản hoá schema |

## Tóm tắt
Bằng cách tận dụng `CompletionModel` trait, việc chuyển đổi từ Cloud LLM sang Local LLM trong Rig trở nên liền mạch. Kiến trúc này không chỉ đảm bảo an toàn dữ liệu mà còn tuân thủ nghiêm ngặt các nguyên tắc SOLID và Type-Safety của Rust. Trong chương tiếp theo, chúng ta sẽ nhìn rộng ra hệ sinh thái Awesome Rig để xem cộng đồng đang xây dựng những ứng dụng Production nào bằng bộ công cụ này.
