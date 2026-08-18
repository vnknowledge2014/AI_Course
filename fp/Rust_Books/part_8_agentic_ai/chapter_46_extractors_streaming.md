# Chapter 46 — Core Abstractions: Extractors và Streaming

> **Bạn sẽ học được**:
> - Structured Output: ép LLM trả về đúng struct Rust thay vì chuỗi tự do
> - `Extractor` — "make illegal states unrepresentable" áp vào ranh giới AI
> - Xử lý streaming response để cải thiện cảm nhận độ trễ
> - Vì sao bắt lỗi ở network boundary rẻ hơn bắt lỗi trong business logic
>
> **Yêu cầu trước**: Chapter 45, Chapter 14 (Algebraic Types), Chapter 25 (serde)
> **Thời gian đọc**: ~35 phút | **Level**: Advanced
> **Kết quả cuối cùng**: RustyOps trả về `DiagnosisReport` có kiểu, không phải String.

> "LLM là một cỗ máy phi cấu trúc. Để xây dựng hệ thống phần mềm đáng tin cậy với nó, chúng ta phải áp đặt cấu trúc nghiêm ngặt lên output của nó ngay tại biên (boundary)."

Ở [Chương 45](chapter_45_intro_to_rig.md), chúng ta đã tạo một "bộ não" cho Agent và nhận về một câu trả lời dạng chuỗi văn bản (String). Nhưng trong thế giới phần mềm thực tế, chuỗi văn bản thuần túy hiếm khi hữu dụng nếu bạn muốn tự động hóa luồng nghiệp vụ. 

Làm thế nào để hệ thống DevOps của bạn biết chính xác cần thực hiện hành động gì dựa trên kết quả trả về? Bạn không thể parse chuỗi String bằng Regex được vì LLM có thể trả lời "Vâng, thưa sếp, tôi sẽ chạy lệnh `restart`" hoặc đơn giản là `{"action": "restart"}`. 

Trong chương này, chúng ta sẽ áp dụng lại tư duy **"Make illegal states unrepresentable"** (đã học ở Part 2 và Part 4) vào hệ thống Agentic AI thông qua kỹ thuật **Structured Output** và **Extractors**.

---

## 46.1 — Vấn đề Parsing Error trong AI

Khi làm việc với Python (LangChain hay AutoGen), lập trình viên thường dùng prompt engineering để "van xin" LLM trả về đúng chuẩn JSON:
```
You must respond in valid JSON format only. No extra text.
{"action": "restart", "server": "nginx"}
```
Tuy nhiên, thi thoảng LLM vẫn có thể "vui tính" thêm thắt vài chữ như `"Sure, here is your JSON: { ... }"`. Lúc này, hàm `json.loads()` của Python sẽ crash, gây lỗi cho toàn bộ chuỗi hệ thống. Đáng sợ hơn, kiểu dữ liệu bên trong JSON có thể bị sai (trả về String `"80"` thay vì Integer `80`).

### Cách giải quyết của Rig: Type-Safety by Construction

Thay vì dùng Agent thông thường, Rig cung cấp `Extractor` — một công cụ ép LLM phải trả về dữ liệu tương thích 100% với một cấu trúc dữ liệu Rust (`struct`). Rig tận dụng `schemars` để dịch `struct` của Rust thành JSON Schema chuẩn, sau đó đẩy JSON Schema này vào luồng API của LLM (thường thông qua cơ chế *Function Calling* hoặc *Structured Outputs API* của OpenAI/Gemini).

Khi LLM trả kết quả về, Rig sẽ dùng `serde` để deserialize kết quả đó lại thành Rust struct. Nếu có sai sót, nó bị chặn ngay tại ranh giới mạng (network boundary), trả về lỗi cụ thể cho LLM để nó thử sinh lại hoặc trả lỗi lên cấp trên.

---

## 46.2 — Triển khai Extractor trong RustyOps

Hãy nâng cấp hệ thống RustyOps. Giả sử người dùng yêu cầu RustyOps phân tích một chuỗi log lỗi (Error Log) và trích xuất ra các trường dữ liệu quan trọng như: mức độ nghiêm trọng (Severity), mã lỗi (Error Code), và đề xuất hành động (Recommended Action).

### Bước 1: Khai báo Cấu trúc Dữ liệu (Domain Model)

Chúng ta cần `serde` và `schemars` để định nghĩa cấu trúc.

Cập nhật `Cargo.toml`:
```toml
[dependencies]
rig-core = "0.41"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
schemars = "1.0" # Bắt buộc phải có để sinh JSON schema cho Extractor/Tool
```

Viết code định nghĩa model:
```rust
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Định nghĩa Enums để giới hạn số lượng trạng thái hợp lệ
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

// Cấu trúc dữ liệu cuối cùng mà ta muốn nhận
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
struct LogAnalysis {
    /// Đánh giá mức độ nghiêm trọng của lỗi này
    severity: Severity,
    /// Mã lỗi hoặc tên tiến trình gây lỗi (VD: OOMKilled, Timeout)
    error_code: String,
    /// Đề xuất cách sửa lỗi ngắn gọn
    recommended_action: String,
}
```
*Lưu ý: Các comments `///` bên trên các field sẽ được `schemars` biên dịch thành trường `description` trong JSON Schema. LLM sẽ đọc các mô tả này để hiểu nó phải điền gì vào trường đó.*

### Bước 2: Sử dụng Extractor

Thay vì gọi `.agent()`, ta dùng `.extractor()`:

```rust
use rig::providers::openai;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let openai_client = openai::Client::from_env();

    // Khởi tạo một Extractor thay vì Agent thông thường
    let log_extractor = openai_client
        .extractor::<LogAnalysis>("gpt-4o-mini") // Ép kiểu trả về là LogAnalysis
        .preamble(
            "Bạn là một chuyên gia phân tích log hệ thống Linux. \
             Nhiệm vụ của bạn là đọc log, phân tích và trích xuất dữ liệu chính xác."
        )
        .build();

    let raw_log = "2023-10-24 10:15:30 [FATAL] [kernel] Out of memory: Killed process 1234 (java). \
                   System memory exhausted.";

    println!("Đang phân tích log...");
    
    // Gọi prompt. Kết quả trả về không phải là String, mà là struct LogAnalysis!
    let analysis: LogAnalysis = log_extractor
        .extract(raw_log)
        .await?;

    println!("Phân tích thành công!");
    println!("Mức độ: {:?}", analysis.severity);
    println!("Mã lỗi: {}", analysis.error_code);
    println!("Hành động: {}", analysis.recommended_action);

    Ok(())
}
```

Kết quả: Bạn nhận được một biến `analysis` mang kiểu `LogAnalysis` chuẩn chỉnh. Không cần phải lo lắng về việc parse chuỗi thủ công. Bạn có thể tự tin đưa `analysis` vào các hàm xử lý tiếp theo của domain logic một cách an toàn.

---

## 46.3 — Tối ưu UX với Streaming Response

Khi tương tác với con người (chatbot), độ trễ (latency) là yếu tố sống còn. Một phản hồi dài của LLM có thể mất từ 5-10 giây để sinh xong toàn bộ. Nếu bạn dùng phương thức `.prompt()`, user sẽ phải nhìn màn hình "loading" trong suốt 10 giây đó.

Streaming giải quyết vấn đề này bằng cách trả về từng phần nhỏ (chunk) của phản hồi ngay khi LLM vừa sinh ra xong. 

Rig tích hợp chặt chẽ với hệ sinh thái async của Rust (cụ thể là trait `Stream` từ crate `futures`).

```rust
use rig::providers::openai;
use rig::completion::Prompt;
use futures::stream::StreamExt; // Cần thiết để gọi .next() trên stream
use dotenvy::dotenv;
use std::io::{Write, stdout};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let openai = openai::Client::from_env();

    let agent = openai
        .agent("gpt-4o-mini")
        .preamble("Bạn là một chuyên gia giải thích công nghệ bằng các ẩn dụ thú vị.")
        .build();

    println!("Đang kết nối...");
    
    // Sử dụng .chat_stream() thay vì .prompt()
    let mut stream = agent
        .chat_stream("Giải thích cho tôi về Quantum Computing trong 3 đoạn văn.")
        .await?;

    // Nhận từng chunk dữ liệu ngay khi nó được sinh ra
    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                // In ngay chunk ra màn hình mà không cần xuống dòng
                print!("{}", chunk);
                // Flush stdout để hiển thị ngay lập tức
                stdout().flush()?;
            }
            Err(e) => {
                eprintln!("\nLỗi stream: {}", e);
                break;
            }
        }
    }

    println!("\n\n-- Hoàn thành --");
    Ok(())
}
```

Nhờ kiến trúc non-blocking của `tokio`, trong khi đang đợi (await) chunk tiếp theo từ mạng, tiểu trình (task) hiện tại sẽ nhường quyền điều khiển (yield) cho các tiểu trình khác. Điều này giúp server của bạn có thể stream dữ liệu cho hàng ngàn người dùng cùng lúc mà không bị treo.

---

---

## ✅ Checkpoint 46

1. LLM trả về JSON thiếu một field bắt buộc. Lỗi xảy ra ở đâu — trong Extractor hay trong business logic? Vì sao đó là tin tốt?
2. Vì sao `Extractor` cần `schemars::JsonSchema` chứ `serde::Deserialize` là chưa đủ?
3. Streaming cải thiện *thời gian phản hồi thực tế* hay *cảm nhận về độ trễ*? Điều đó thay đổi cách bạn đo lường ra sao?

<details>
<summary>Đáp án</summary>

1. Trong Extractor, ngay tại network boundary. Tin tốt vì domain type của bạn **không bao giờ** tồn tại ở trạng thái nửa vời — bạn không phải rải `if field.is_none()` khắp nơi.
2. `Deserialize` chỉ dùng để *đọc* JSON đã có. `JsonSchema` dùng để *sinh ra* mô tả gửi cho LLM, để nó biết phải tạo hình dạng gì. Thiếu nó, bạn chỉ biết từ chối chứ không hướng dẫn được model.
3. Chỉ *cảm nhận*. Tổng thời gian tới token cuối gần như không đổi — điều thay đổi là time-to-first-token. Vậy nên phải đo **cả hai**: TTFT cho UX, tổng thời gian cho chi phí và throughput.
</details>

---

## 🏋️ Bài tập

**Bài 1 (5 phút).** Thêm field `confidence: f32` vào struct kết quả và mô tả qua doc comment "0.0 đến 1.0". Chạy thử và quan sát LLM có tôn trọng khoảng giá trị không — rồi thêm validation phía Rust để không phụ thuộc vào thiện chí của model.

**Bài 2 (10 phút).** Đổi field `severity: String` thành `enum Severity { Low, Medium, High, Critical }` với `#[derive(Deserialize, JsonSchema)]`. So sánh JSON schema sinh ra trước và sau. Đây chính là bài học Chapter 14 áp dụng vào AI.

**Bài 3 (15 phút).** Viết một retry loop: khi Extractor trả `Err`, nạp lại thông báo lỗi vào prompt và yêu cầu model sửa, tối đa 3 lần. Đây là bản rút gọn của self-correction loop ở Chapter 47.

<details>
<summary>Gợi ý bài 3</summary>

```rust
let mut last_err = None;
for attempt in 0..3 {
    match extractor.extract(&prompt_with(last_err.as_deref())).await {
        Ok(v) => return Ok(v),
        Err(e) => last_err = Some(e.to_string()),
    }
}
```
Điểm mấu chốt: **đưa nội dung lỗi vào prompt**. Retry mù (gửi lại y nguyên prompt cũ) chỉ tốn tiền chứ không tăng tỉ lệ thành công.
</details>

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| `missing field` dù LLM có trả field đó | Tên field khác nhau về kiểu chữ | Thêm `#[serde(rename_all = "camelCase")]` |
| Model bịa giá trị cho enum | Doc comment mô tả chưa rõ | Viết doc comment cho từng variant — `schemars` đưa chúng vào schema |
| Streaming in ra cả cục ở cuối | Có `.collect()` ở đâu đó làm gom hết stream | Dùng `while let Some(chunk) = stream.next().await` và flush mỗi chunk |
| `the trait JsonSchema is not implemented` | Thiếu `#[derive(JsonSchema)]` hoặc sai version schemars | Rig 0.41 dùng `schemars = "1.0"`, không phải 0.8 |

## Tóm tắt
Qua chương này, chúng ta đã biến AI từ một công cụ sinh chữ (text generator) thành một thành phần phần mềm đáng tin cậy. Bằng cách áp dụng **Extractors**, chúng ta ép buộc LLM tuân thủ Type-System của Rust, biến ranh giới giữa AI và Code trở nên vững chãi.

Tuy nhiên, `Extractor` chỉ là luồng đi một chiều (One-way): User → LLM → Dữ liệu. Để xây dựng một Agentic AI thực sự, AI cần có khả năng chủ động tương tác ngược lại với hệ thống (Two-way). Đó chính là khái niệm về **Tools (Công cụ)** mà chúng ta sẽ đào sâu ở Chương 47.
