# Chapter 54 — Capstone Project: Hệ sinh thái "Rusty-Context"

> **Bạn sẽ học được**:
> - Agentic Context Engineering — vì sao ngữ cảnh là bài toán hạ tầng, không phải mẹo prompt
> - Rusty-ConDB: kiến trúc Lossless Memory (STM/LTM)
> - Rusty-PageIndex: AST-aware chunking và incremental sync
> - Dreaming Mechanism: hợp nhất ký ức khi nhàn rỗi
>
> **Yêu cầu trước**: Toàn bộ Part VIII
> **Thời gian đọc**: ~40 phút | **Level**: Principal
> **Kết quả cuối cùng**: Bản thiết kế cho một hạ tầng ngữ cảnh dùng lại được giữa nhiều agent.

Trong suốt 53 chương vừa qua, chúng ta đã đi từ những khái niệm cơ bản nhất của Rust, tư duy Functional Programming, Domain-Driven Design, cho đến việc xây dựng các Agent độc lập với Rig Core. 

Nhưng nếu chỉ dừng lại ở các Agent đơn lẻ, chúng ta mới chỉ chạm đến bề nổi của Agentic AI. Điểm yếu chí mạng của hầu hết các hệ thống AI hiện nay là **Sự thoái hóa ngữ cảnh (Context Rot)**: khi hội thoại hoặc lượng tài liệu quá dài, Agent bắt đầu "quên", sinh ảo giác (hallucination) và đánh mất tư duy logic. Các hệ thống RAG (Retrieval-Augmented Generation) truyền thống dựa vào Vector Database chỉ giải quyết được một phần vấn đề bằng cách tìm kiếm sự tương đồng (similarity) nhưng lại phá vỡ tính liên kết ngữ nghĩa (semantic continuity) do phải băm nhỏ dữ liệu (chunking).

Để khép lại cuốn sách này, chúng ta sẽ không xây dựng một ứng dụng nhỏ giọt. Chúng ta sẽ cùng nhau thiết kế và phác thảo một **Core AI Infrastructure** (Hạ tầng AI lõi) mang tên **Rusty-Context** — một hệ sinh thái quản lý ngữ cảnh có khả năng tự tiến hóa (Self-evolving), được lấy cảm hứng từ các nền tảng SOTA (State-Of-The-Art) như VectifyAI, MemPalace và CocoIndex.

---

## 54.1 — Tầm nhìn: Kỹ nghệ Ngữ cảnh (Agentic Context Engineering)

Rusty-Context không chỉ là một cơ sở dữ liệu, nó là một "Hệ điều hành" về mặt ngữ cảnh cho các Agent (như OmniUltraAgent). Nó được xây dựng dựa trên 5 trụ cột (The 5 Pillars):

1. **`Rusty-ConDB` (Lõi Bộ Nhớ):** Phân tách rõ ràng giữa **STM** (Bộ nhớ ngắn hạn - KV-Cache native) và **LTM** (Bộ nhớ dài hạn). Lưu trữ dữ liệu nguyên bản 100% (Lossless / Verbatim) thay vì tóm tắt phá hủy gốc.
2. **`Rusty-PageIndex` (Chỉ mục gia tăng & Đa phương thức):** Đọc PDF/Code và xây dựng Cây Cấu trúc. Sử dụng cơ chế Incremental (chỉ xử lý phần $\Delta$ bị thay đổi) và AST-Aware để không bao giờ cắt vỡ logic của Code.
3. **`Rusty-ChatIndex` (Cây hội thoại động):** Gom nhóm hội thoại dài thành các Topic Tree, giúp Agent duy trì một phiên làm việc vô tận.
4. **`Rusty-OpenKB` (Trí tuệ tổng hợp):** Một Agent chạy ngầm làm nhiệm vụ biên dịch dữ liệu thô thành các trang Wiki liên kết với nhau.
5. **`Rusty-CLI` & `SKILL.md`:** Đóng gói tất cả thành một công cụ dòng lệnh (CLI). Cung cấp `SKILL.md` để các Agent biết cách tự gọi lệnh CLI và quản trị bộ nhớ của chính nó.

Trong chương này, chúng ta sẽ đi sâu vào việc lập trình phần Lõi (Pillar 1, 2, 5) bằng Rust.

---

## 54.2 — Pillar 1: Rusty-ConDB — Kiến trúc Lossless Memory

Cảm hứng từ **MemPalace**, chúng ta thiết kế LTM không phải là một danh sách phẳng các Vector, mà là một "Cung điện Ký ức" (Memory Palace) phân cấp: `Wings` (Khu vực/Dự án) $\rightarrow$ `Rooms` (Chủ đề) $\rightarrow$ `Drawers` (Dữ liệu gốc).

Sử dụng `serde` để định nghĩa mô hình dữ liệu:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PalaceLevel {
    Wing(String),
    Room(String),
    /// Lưu trữ nguyên bản 100% dữ liệu gốc (Verbatim) kèm mã băm (hash) để đối chiếu
    Drawer { content: String, hash: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextNode {
    pub id: String,
    pub summary: String, // LLM sinh tóm tắt cho Node này
    pub level: PalaceLevel,
    pub children: Vec<ContextNode>, // Cấu trúc đệ quy (Tree)
}
```

Bằng cách dùng cấu trúc đệ quy (Tree), `Rusty-ConDB` hỗ trợ **Reasoning-based Retrieval**. Khi có câu truy vấn, LLM không dùng tìm kiếm vector mù quáng. Nó đóng vai trò một Agent tự lướt qua các `Wings`, đánh giá xem `Wing` nào phù hợp, sau đó đi sâu vào `Rooms`, và cuối cùng mở `Drawers` ra để lấy dữ liệu gốc. 

> **[Bài học FP]**: Việc sử dụng Enum `PalaceLevel` giúp chúng ta tận dụng tối đa Pattern Matching của Rust. Trình biên dịch sẽ buộc bạn phải xử lý riêng biệt logic khi duyệt qua một Khu vực (Wing) so với khi đọc dữ liệu thô (Drawer), loại bỏ hoàn toàn các lỗi ép kiểu tiềm ẩn.

---

## 54.3 — Pillar 2: Rusty-PageIndex — Hybrid Vision & Incremental AST

### 54.3.1 — Đa phương thức (Multimodal) với Trait Pattern

Truyền thống, xử lý PDF đòi hỏi phải có OCR (Optical Character Recognition) để lấy Text. Nhưng với các mô hình Vision mạnh như GPT-4o hay Claude 3.5 Sonnet, việc dùng OCR là "phá hủy" cấu trúc trình bày (Layout) của tài liệu.

Chúng ta sẽ thiết kế một Strategy Pattern bằng `async_trait` để điều phối:

```rust
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait DocumentParser {
    async fn parse(&self, file_path: &str) -> Result<String>;
}

/// Dành cho các mô hình đa phương thức: Gửi thẳng ảnh (chụp từ PDF) vào LLM
pub struct VisionNativeStrategy {
    // rig::providers::openai::Client (Cấu hình model gpt-4o)
}

#[async_trait]
impl DocumentParser for VisionNativeStrategy {
    async fn parse(&self, file_path: &str) -> Result<String> {
        println!("Extracting {} using Native Vision...", file_path);
        // ... Render PDF to images -> Send to Vision API
        Ok("Parsed via Vision".to_string())
    }
}

/// Dành cho các LLM chỉ có Text (Llama 3): Đi qua Tesseract OCR trước
pub struct OcrFallbackStrategy {
    pub ocr_engine_path: String,
}

#[async_trait]
impl DocumentParser for OcrFallbackStrategy {
    async fn parse(&self, file_path: &str) -> Result<String> {
        println!("Extracting {} using OCR Fallback...", file_path);
        // ... Command::new("tesseract")
        Ok("Parsed via OCR".to_string())
    }
}
```

Nhờ `Trait`, hàm index chính của chúng ta không cần quan tâm nó đang dùng OCR hay Vision, nó chỉ cần gọi `.parse()`.

### 54.3.2 — Incremental Sync (Chỉ mục gia tăng $\Delta$) và AST-Aware (Mã giả)

Lấy cảm hứng từ **CocoIndex**, một bộ RAG SOTA không bao giờ index lại toàn bộ hệ thống khi chỉ có 1 dòng code thay đổi. 
Hơn thế nữa, đối với Source Code, chúng ta sử dụng **AST (Abstract Syntax Tree)** để phân mảnh thay vì cắt ngang chuỗi ký tự. Bằng cách dùng `tree-sitter` (Rust wrapper), hệ thống trích xuất nguyên vẹn từng `impl` block, từng `function`.

*Cơ chế hoạt động:*
1. Hệ thống tính Hash (SHA-256) của từng AST Node.
2. So sánh với Hash lưu trong `Drawer`. 
3. Chỉ khi Hash khác biệt (Delta $\Delta$), hệ thống mới gọi LLM để đọc và tóm tắt lại Node đó. Tiết kiệm 80% chi phí Token.

---

## 54.4 — The Dreaming Mechanism (Giấc mơ của AI)

Làm sao để một Agent có khả năng "tự học" (Self-taught)? 
Bí mật nằm ở quá trình **Dreaming (Ngủ mơ)** — tương tự như cơ chế của **Claude Code** và tiến trình `omni dream` của **OmniUltraAgent**.

Khi Agent nhàn rỗi (idle) hoặc vừa kết thúc một Task khổng lồ, nó sinh ra rất nhiều ngữ cảnh rác trong **STM (Short-Term Memory)**. CLI của chúng ta cung cấp lệnh `rusty-context dream` để chạy vòng lặp đánh giá:

```rust
pub async fn run_dream_cycle() -> anyhow::Result<()> {
    println!("Starting Dreaming Cycle...");
    
    // 1. Quét STM lấy các dấu vết (Traces)
    println!("Extracting traces from STM...");

    // 2. Tự đánh giá bằng Rig Core (Self-reflection)
    // Agent sẽ phân tích: "Đâu là bài học cốt lõi? Đâu là tri thức mới?"
    let confidence_score = 0.85; // Kết quả giả lập từ LLM Evaluator

    // 3. Phân loại và Củng cố (Consolidation)
    if confidence_score >= 0.75 {
        println!("Insight đạt chuẩn. Đẩy vĩnh viễn vào LTM (Drawers).");
        // condb::store_in_ltm(...)
    } else if confidence_score >= 0.50 {
        println!("Insight khả nghi. Đưa vào hàng đợi Review.");
    } else {
        println!("Dữ liệu rác. Tiến hành Flush (xóa) khỏi STM.");
    }

    Ok(())
}
```
Nhờ vòng lặp này, kiến thức của Agent được "mài giũa" qua thời gian. Nó học được từ những sai lầm trước đó và không bao giờ lặp lại lỗi cũ ở phiên làm việc tiếp theo.

---

## 54.5 — Tích hợp Native vào Hệ điều hành Agent (SKILL.md)

Thay vì bọc toàn bộ hệ thống này thành một MCP Server rườm rà, chúng ta sẽ tiếp cận theo hướng "Native CLI". Ta dùng `clap` để tạo giao diện dòng lệnh:

```rust
// src/main.rs
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Index { #[arg(short, long)] path: std::path::PathBuf },
    Search { #[arg(short, long)] query: String },
    Dream,
}
```

Để các Omni Agents có thể sử dụng CLI này, ta chỉ cần định nghĩa một file **`SKILL.md`**:

```markdown
# Rusty-Context Skill

Kỹ năng giúp Agent tự quản trị ngữ cảnh. 
- Khi có file thay đổi, gọi: `cargo run --bin rusty-context -- index --path <file>`
- Khi cần tìm kiếm tri thức sâu, gọi: `cargo run --bin rusty-context -- search --query "..."`
- KHI KẾT THÚC CÔNG VIỆC, PHẢI GỌI LỆNH SAU ĐỂ LƯU TRÍ NHỚ: 
  `cargo run --bin rusty-context -- dream`
```

Khi được cung cấp `SKILL.md`, các Agent (với khả năng Tool Calling của Rig) sẽ tự động sinh ra các bash script gọi thẳng vào CLI của chúng ta. Nó giao tiếp với thế giới như một kỹ sư thực thụ gõ phím trên Terminal.

---

---

## ✅ Checkpoint 54

1. "Lossless memory" nghĩa là gì, và nó khác gì với việc cứ giữ toàn bộ lịch sử chat?
2. Vì sao chunk theo AST tốt hơn chunk theo số dòng khi index mã nguồn?
3. Dreaming chạy lúc nhàn rỗi. Nó đánh đổi tài nguyên nào lấy tài nguyên nào?

<details>
<summary>Đáp án</summary>

1. Lossless = **không mất thông tin gốc**, nhưng thứ nạp vào context là bản đã chắt lọc. Giữ nguyên lịch sử chat thì lossless mà không dùng được (tràn context); tóm tắt rồi vứt bản gốc thì dùng được mà mất thông tin. Lossless memory giữ cả hai tầng.
2. Vì chunk theo dòng cắt ngang hàm — nửa hàm ở chunk này, nửa ở chunk kia, và cả hai nửa đều vô nghĩa khi retrieval. Chunk theo AST giữ trọn đơn vị ngữ nghĩa (hàm, impl block, struct).
3. Đổi **compute lúc rảnh** lấy **latency lúc bận**. Hợp nhất và nén ký ức là việc nặng; làm trước thì lúc phục vụ request chỉ còn tra cứu.
</details>

---

## 🏋️ Bài tập

**Bài 1 (15 phút).** Vẽ sơ đồ luồng dữ liệu của Rusty-Context: từ file nguồn tới lúc nạp vào prompt. Đánh dấu chỗ nào là pure function, chỗ nào là I/O — đối chiếu với Functional Core / Imperative Shell (Chapter 35).

**Bài 2 (25 phút).** Cài incremental sync tối giản: theo dõi mtime của file, chỉ re-index file đã đổi. Đo thời gian so với index lại toàn bộ.

**Bài 3 (30 phút).** Thiết kế (chỉ cần kiểu dữ liệu và chữ ký hàm, không cần cài) một `MemoryStore` trait có `store`, `recall`, `consolidate`. Chứng minh nó tách được STM và LTM sau lưng cùng một interface.

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| Index phình theo thời gian | Không có bước prune/dedup | Chạy dreaming định kỳ với ngưỡng cosine |
| Recall trả ký ức lỗi thời | Không có trọng số thời gian | Cho điểm giảm dần theo tuổi của ký ức |
| Re-index toàn bộ mỗi lần chạy | Thiếu phát hiện thay đổi | So sánh mtime + content hash |
| Chunk cắt giữa hàm | Chunk theo dòng | Chuyển sang AST-aware (tree-sitter) |

## Tóm tắt

Chúng ta đã cùng nhau xây dựng **Rusty-Context** — một hình mẫu kiến trúc (Reference Architecture) hoàn hảo cho các hệ thống Agentic AI trong tương lai. Bằng cách kết hợp Type-safety của Rust, triết lý Functional Programming, tư duy Domain-Driven Design và sức mạnh của Rig Core, bạn không còn viết những đoạn script AI đồ chơi (toy script) nữa. Bạn đang thiết kế những hệ thống có khả năng vận hành ở quy mô doanh nghiệp (Enterprise-grade).

Kỷ nguyên của AI không thay thế Software Engineers. Nó đòi hỏi những kỹ sư có tư duy sắc bén hơn bao giờ hết để thiết kế các kiến trúc cốt lõi như thế này. Và Rust chính là thứ vũ khí tối thượng cho kỷ nguyên đó.

Chúc mừng bạn đã hoàn thành cuốn sách! Hãy tiếp tục kiến tạo.
