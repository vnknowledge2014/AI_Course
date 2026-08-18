# Chapter 52 — Evaluation, Testing và Tool Servers

> **Bạn sẽ học được**:
> - Vì sao test hệ AI khác test phần mềm tất định
> - LLM-as-a-Judge: dùng AI chấm điểm AI, và các bẫy của nó
> - Metrics bổ sung ngoài điểm của judge
> - `ToolServer` — gỡ nút thắt lock contention
>
> **Yêu cầu trước**: Chapter 33 (TDD), Chapter 34 (Property-Based Testing)
> **Thời gian đọc**: ~25 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Một bộ eval chạy được trong CI cho agent của bạn.

Kiểm thử (Testing) là trái tim của phần mềm bền vững (Production Engineering) mà chúng ta đã thảo luận kĩ ở Part 6 và 7. Trong thế giới của lập trình truyền thống (Deterministic Programming), viết test rất đơn giản: Đầu vào $A$ luôn luôn cho ra kết quả $B$.

Tuy nhiên, với LLMs, tính tất định (Determinism) biến mất. Cùng một câu prompt có thể sinh ra nhiều câu trả lời khác nhau. Vậy làm thế nào để đảm bảo rằng Agent của chúng ta đang hoạt động đúng như thiết kế? Làm sao để bắt lỗi (Regression) khi chúng ta thay đổi system prompt? 

Trong chương này, chúng ta sẽ khám phá framework `rig::evals` – một công cụ tuyệt vời áp dụng trực tiếp tư duy Railway-Oriented Programming (ROP) để chấm điểm và đánh giá (Evaluate) outputs của LLM. Chúng ta cũng sẽ giải quyết bài toán đồng thời (Concurrency) khi nhiều Agent chia sẻ chung một bộ Tools qua `ToolServer`.

## 52.1 — Bản Chất Của Evaluation (Evals) Trong Rig

Để bật tính năng evals, bạn cần kích hoạt cờ `experimental`:
```bash
cargo add rig -F experimental
```

Tại trung tâm của framework này là trait `Eval`. Thay vì trả về `Result<T, E>` thông thường, `Eval` trả về một cấu trúc Enum gọi là `EvalOutcome`. Đây là một dạng thức mở rộng của ROP:

```rust
pub enum EvalOutcome<Output> {
    Pass(Output),     // LLM trả lời đúng tiêu chí (Thành công)
    Fail(Output),     // LLM trả lời sai tiêu chí (Thất bại logic)
    Invalid(String),  // Lỗi hệ thống: API chết, JSON parse lỗi (Lỗi hạ tầng)
}
```

Kiến trúc này phân tách rõ ràng giữa **Lỗi Logic** (`Fail`) và **Lỗi Hạ Tầng** (`Invalid`). Nếu kết quả là `Fail`, LLM của bạn cần được cải thiện prompt hoặc fine-tuning. Nếu kết quả là `Invalid`, bạn cần kiểm tra mạng, hoặc LLM đã sinh ra JSON không hợp lệ (Make illegal states unrepresentable).

## 52.2 — LLM-as-a-Judge: Dùng AI Để Đánh Giá AI

Cách hiệu quả nhất để chấm điểm văn bản của LLM là... dùng một LLM khác làm giám khảo (Judge). Rig tận dụng sức mạnh của **Extractors** (Part 4) để ép LLM Giám Khảo trả về cấu trúc JSON nghiêm ngặt.

Hãy định nghĩa một tiêu chuẩn đánh giá tính chính xác của sự thật (Factuality) bằng Rust Struct:

```rust
use rig::evals::{Eval, EvalOutcome, Judgment, LlmJudgeBuilder};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, JsonSchema)]
struct FactualityJudgment {
    is_factual: bool,
    reasoning: String,
}

// Áp dụng trait Judgment
impl Judgment for FactualityJudgment {
    fn passes(&self) -> bool {
        self.is_factual
    }
}
```

Sau đó, chúng ta truyền struct này vào LLM Judge:

```rust
// client là một OpenAI hoặc Anthropic client
let ext = client.extractor::<FactualityJudgment>("gpt-4o");
let judge = LlmJudgeBuilder::new(ext).build();

let outcome = judge.eval("Thủ đô của Pháp là Paris.".to_string()).await;

match outcome {
    EvalOutcome::Pass(score) => println!("Đạt: {}", score.reasoning),
    EvalOutcome::Fail(score) => println!("Trượt: {}", score.reasoning),
    EvalOutcome::Invalid(reason) => eprintln!("Lỗi hạ tầng: {reason}"),
}
```

Nhờ ROP, flow xử lý của chúng ta cực kỳ an toàn và mạch lạc. Nếu LLM trả về JSON bị lỗi, kết quả tự động rẽ nhánh vào đường ray `Invalid`.

## 52.3 — Các Metrics Bổ Sung

Bên cạnh `LlmJudgeMetric`, Rig còn hỗ trợ:

- **LlmScoreMetric**: Yêu cầu LLM chấm điểm theo thang số (ví dụ từ 0.0 đến 1.0) dựa trên một tiêu chí cụ thể. Nếu điểm số (`score`) lớn hơn hoặc bằng `threshold` định trước, kết quả là `Pass`.
- **SemanticSimilarityMetric**: (Không dùng LLM, chỉ dùng Embeddings). Đo lường khoảng cách Vector (Cosine Similarity) giữa câu trả lời của Agent và câu trả lời mẫu (Reference Answer). Rất hữu ích và rẻ tiền cho RAG pipelines.

Ví dụ về Semantic Similarity:

```rust
use rig::evals::{Eval, EvalOutcome, SemanticSimilarityMetric};

let metric = SemanticSimilarityMetric::builder(embedding_model)
    .reference_answer("Con mèo đang ngồi trên thảm")
    .threshold(0.85) // Cosine similarity >= 0.85 là Đạt
    .build()
    .await?;

let outcome = metric.eval("Có một con mèo trên tấm thảm".to_string()).await;
assert!(outcome.is_pass());
```

## 52.4 — Giải Quyết Bài Toán Tranh Chấp Trạng Thái Bằng Tool Servers

Trong một hệ thống Multi-Agent, nhiều Agent có thể sẽ cùng gọi vào một Tool (Ví dụ: ghi log vào Database). Nếu Tool đó thay đổi trạng thái (mutate state), chúng ta sẽ đối mặt với bài toán Data Race quen thuộc ở Part 6 (Concurrency).

Rig giải quyết vấn đề này qua kiến trúc **ToolServer**. `ToolServer` hoạt động bằng cách quản lý các Tools bên trong một lớp Actor hoặc bọc qua các smart pointers an toàn luồng (`Arc<Mutex<T>>`). Thay vì trực tiếp sở hữu Tool, các Agent nhận được các `ToolServerHandle`.

Điều này đảm bảo:
1. **Tính độc quyền (Mutual Exclusion):** Tại một thời điểm chỉ có một yêu cầu được thao tác với Tool có thay đổi trạng thái.
2. **Khả năng mở rộng (Scalability):** Hệ thống có thể Spawn hàng nghìn worker agents (thông qua `tokio::spawn`) mà không làm hỏng bộ nhớ cục bộ.

---

## ✅ Checkpoint 52

1. Vì sao `assert_eq!(output, "kết quả mong đợi")` gần như luôn sai cách khi test agent?
2. Nêu hai điểm yếu của LLM-as-a-Judge.
3. Property-Based Testing (Chapter 34) áp vào agent thế nào, khi output không tất định?

<details>
<summary>Đáp án</summary>

1. Vì output không tất định — cùng prompt cho ra câu chữ khác nhau mỗi lần. So khớp chuỗi sẽ đỏ liên tục mà chẳng phát hiện được lỗi thật. Phải kiểm **tính chất**, không kiểm chuỗi.
2. (a) Judge cũng là LLM nên cũng sai, và sai một cách có hệ thống; (b) thiên vị đã ghi nhận: ưu ái câu trả lời dài, và ưu ái output của chính model cùng họ với nó.
3. Kiểm **bất biến** thay vì giá trị: "câu trả lời luôn chứa ít nhất một trích dẫn nguồn", "agent không bao giờ gọi tool bị cấm", "output luôn parse được thành `Report`". Những tính chất đó tất định ngay cả khi câu chữ thì không.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Viết 5 cặp (prompt, tiêu chí) cho RustyOps và chạy thành một bộ eval. Tiêu chí là mệnh đề kiểm được, không phải câu trả lời mẫu.

**Bài 2 (20 phút).** Cài LLM-as-a-Judge chấm thang 1–5. Chạy **cùng một** output qua judge 5 lần — điểm có ổn định không? Điều đó nói gì về mức tin cậy của một lần chấm?

**Bài 3 (25 phút).** Thêm ba metric tất định vào bộ eval: tổng token, độ trễ, số lượt gọi tool. Cho CI fail khi bất kỳ metric nào xấu đi quá 20% so với baseline.

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| Eval đỏ ngẫu nhiên | So khớp chuỗi chính xác | Chuyển sang kiểm tính chất/tiêu chí |
| Judge luôn cho điểm cao | Rubric quá mơ hồ | Viết tiêu chí cụ thể cho từng mức điểm |
| Eval chạy quá lâu trong CI | Gọi tuần tự | Chạy song song với `buffer_unordered`, giới hạn concurrency |
| Lock contention khi eval song song | Nhiều agent chia sẻ state của tool | Dùng `ToolServer` (mục 52.4) |

## Tóm tắt
Việc ứng dụng Agentic AI lên Production không thể thiếu Testing và Evaluation. Thông qua việc biến các kết quả phi cấu trúc của LLM thành các Struct định hình chặt chẽ (Extractors), và xử lý luồng sự kiện qua ROP (`EvalOutcome`), chúng ta đã mang lại sự an toàn (Type-Safety) của hệ sinh thái Rust vào trung tâm của Generative AI.

Trong chương tiếp theo, chúng ta sẽ tìm hiểu cách tối ưu hoá bảo mật dữ liệu doanh nghiệp bằng việc triển khai Local Models (GGUF, Llama.cpp) tích hợp vào Rig. Cùng đón xem!
