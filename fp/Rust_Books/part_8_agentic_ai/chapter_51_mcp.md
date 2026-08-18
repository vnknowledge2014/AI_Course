# Chapter 51 — Khám Phá Model Context Protocol (MCP)

> **Bạn sẽ học được**:
> - MCP là gì và vì sao nó chính là Hexagonal Architecture ở tầng liên tiến trình
> - Dùng `rmcp` để kết nối Agent tới tool bên ngoài
> - Tự viết một MCP Server bằng Rust
>
> **Yêu cầu trước**: Chapter 47, Chapter 21 (Functional Architecture)
> **Thời gian đọc**: ~30 phút | **Level**: Advanced
> **Kết quả cuối cùng**: RustyOps dùng được tool do tiến trình khác cung cấp.

Chào mừng bạn đến với chương 51. Trong các chương trước, chúng ta đã tìm hiểu cách trang bị Tools cho Agent, giúp chúng vượt ra khỏi ranh giới của văn bản tĩnh và tương tác với thế giới thực. Tuy nhiên, nếu mỗi Agent, mỗi LLM framework (như LangChain, AutoGen, hay Rig) đều phải tự viết lại các Tools (ví dụ: Tool đọc file, Tool query database, Tool call API), chúng ta sẽ đối mặt với một sự lãng phí tài nguyên khổng lồ và sự phân mảnh hệ sinh thái.

Giải pháp cho bài toán này chính là **Model Context Protocol (MCP)**. 

MCP là một giao thức chuẩn hóa do Anthropic khởi xướng, nhằm mục đích tạo ra một chuẩn giao tiếp chung giữa các mô hình ngôn ngữ (hoặc Agent) và các nguồn dữ liệu/Tools bên ngoài. Trong chương này, chúng ta sẽ xem xét cách tích hợp MCP vào hệ sinh thái Rig thông qua crate `rmcp`, đồng thời liên hệ nó với các nguyên tắc kiến trúc mà chúng ta đã xây dựng trong cuốn sách này.

## 51.1 — Model Context Protocol (MCP) Dưới Góc Nhìn Hexagonal Architecture

Nếu bạn nhớ lại Part 6 về Hexagonal Architecture, cốt lõi của kiến trúc là tách biệt miền ứng dụng (Functional Core) khỏi các ranh giới ngoại vi (Imperative Shell). 

MCP hoàn toàn ăn khớp với triết lý này:
- **MCP Server** đóng vai trò là một cổng (Port) cung cấp các Tools và dữ liệu (như File System, Git, Database).
- **MCP Client (Agent)** chỉ cần kết nối đến Server và sử dụng các Tools mà không cần quan tâm đến chi tiết triển khai cụ thể của chúng. 

Nhờ vậy, bạn có thể tái sử dụng một MCP Server viết bằng TypeScript, Python, hoặc Go cho một Agent viết bằng Rust (Rig). Đây là sức mạnh của sự kết hợp (Composition) và chuẩn hóa giao diện (Interface).

## 51.2 — Tích hợp `rmcp` vào Rig

Rig hỗ trợ MCP một cách nguyên bản thông qua crate `rmcp`. Để bắt đầu, bạn cần bật feature flag trong `Cargo.toml`:

```toml
[dependencies]
# Feature "rmcp" của rig tự kéo theo "agent" + "rig-agent/rmcp"
rig = { version = "0.41", features = ["rmcp"] }
rmcp = { version = "2.2", features = [
  "client",
  "macros",
  "reqwest",
  "transport-streamable-http-client",
  "transport-streamable-http-client-reqwest",
] }
```

### Bước 2.1: Kết nối MCP Client

Chúng ta sẽ khởi tạo một Transport và kết nối đến một MCP Server đang chạy (ví dụ ở `localhost:8080`). Nhờ tính an toàn kiểu dữ liệu (Type-Safety) của Rust, cấu hình kết nối được mô hình hóa rõ ràng thông qua `ClientInfo`.

```rust
use rmcp::ServiceExt;
use rmcp::model::{ClientCapabilities, ClientInfo, Implementation};
use rmcp::transport::StreamableHttpClientTransport;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = StreamableHttpClientTransport::from_uri("http://localhost:8080");

    let client_info = ClientInfo {
        protocol_version: Default::default(),
        capabilities: ClientCapabilities::default(),
        client_info: Implementation {
            name: "rig-agent".to_string(),
            version: "1.0.0".to_string(),
        },
    };

    // Khởi chạy Client. Bắt lỗi tại ranh giới mạng (ROP)
    let client = client_info.serve(transport).await.inspect_err(|e| {
        tracing::error!("Lỗi khi kết nối MCP Server: {e:?}");
    })?;
    
    // Tiếp tục...
    Ok(())
}
```

### Bước 2.2: Fetch Tools và Trao Quyền cho Agent

Một khi đã kết nối, chúng ta có thể yêu cầu Server cung cấp danh sách Tools. Trong Rig, bạn chỉ cần ném danh sách này vào phương thức `.rmcp_tools()` của Agent Builder. Rig sẽ tự động bọc (wrap) các MCP tools thành các native Rig tools.

```rust
use rmcp::model::Tool;
use rig::providers;

// 1. Lấy danh sách tools từ server
let tools: Vec<Tool> = client.list_tools(Default::default()).await?.tools;

// 2. Khởi tạo LLM Provider
let openai_client = providers::openai::Client::from_env()?;

// 3. Trao quyền cho Agent
let agent = openai_client
    .agent("gpt-4o")
    .rmcp_tools(tools, client.peer().to_owned()) // Inject MCP tools!
    .build();

// 4. Thực thi
let response = agent.prompt("Hãy dùng tool để tính 10 + 10").await?;
println!("Agent response: {response:?}");
```

Chỉ với vài dòng code, Agent của bạn đã có khả năng gọi bất kỳ Tool nào từ MCP Server, bảo toàn hoàn toàn tính bất biến (immutability) và luồng dữ liệu một chiều.

## 51.3 — Xây Dựng MCP Server Bằng Rust

Bạn không chỉ là người tiêu thụ (Client), bạn còn có thể đóng vai trò là nhà cung cấp (Server). `rmcp` cung cấp các macros cực kỳ mạnh mẽ để biến các hàm Rust thông thường thành các MCP Tools.

Hãy xem cách chúng ta định nghĩa một `CalculatorServer` với trait và macro:

```rust
use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content, ErrorData, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};
use serde::Deserialize;

// Định nghĩa Input Schema (Type-safety by construction)
#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct AddRequest {
    a: f64,
    b: f64,
}

#[derive(Clone)]
struct CalculatorServer {
    tool_router: ToolRouter<CalculatorServer>,
}

#[tool_router]
impl CalculatorServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Cộng hai số")]
    fn add(
        &self,
        Parameters(AddRequest { a, b }): Parameters<AddRequest>,
    ) -> Result<CallToolResult, ErrorData> { // Sử dụng Result cho Railway-Oriented Programming
        let result = a + b;
        Ok(CallToolResult::success(vec![Content::text(result.to_string())]))
    }
}

// Gắn router vào handler
#[tool_handler]
impl ServerHandler for CalculatorServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::default()
    }
}
```

Sau khi định nghĩa xong, bạn có thể bọc `CalculatorServer` trong một `StreamableHttpService` và chạy nó phía sau một web framework như `axum`. Toàn bộ quá trình validate input từ LLM, chuyển đổi JSON thành Rust struct (`AddRequest`) được xử lý hoàn toàn tại compile-time thông qua Serde và Schemars. Nếu LLM gửi sai định dạng JSON, hệ thống sẽ trả về lỗi mà không làm sập (crash) server của bạn — một ví dụ hoàn hảo của nguyên tắc "Make illegal states unrepresentable".

---

## ✅ Checkpoint 51

1. MCP tương ứng với "Port" hay "Adapter" trong Hexagonal Architecture?
2. Tool qua MCP đắt hơn tool nội bộ ở điểm nào? Bù lại được gì?
3. MCP Server của bạn crash giữa lúc agent đang chạy. Điều gì nên xảy ra?

<details>
<summary>Đáp án</summary>

1. MCP là **Port** — một giao diện chuẩn hoá. Mỗi server cụ thể (filesystem, Chrome DevTools, database) là một **Adapter** cắm vào port đó.
2. Đắt hơn vì thêm một chặng IPC/mạng, cộng serialize/deserialize. Bù lại: tool viết bằng ngôn ngữ nào cũng được, nâng cấp độc lập, và chạy trong tiến trình riêng nên tool lỗi không kéo sập agent.
3. Lời gọi tool phải trả `ToolExecutionError` kiểu `Network` (retryable), **không** phải panic. Đây đúng là ROP: lỗi hạ tầng là dữ liệu, không phải sự cố chương trình.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Kết nối RustyOps tới một MCP filesystem server và cho nó liệt kê file trong một thư mục. Quan sát tool được nạp **động** lúc chạy chứ không biên dịch sẵn.

**Bài 2 (25 phút).** Viết một MCP Server tối giản expose đúng một tool `uptime()`. Kết nối agent vào nó.

**Bài 3 (20 phút).** Thêm timeout cho mọi lời gọi MCP bằng `tokio::time::timeout`, và ánh xạ timeout thành `ToolExecutionError::timeout(...)`. Vì sao phân loại đúng lại quan trọng ở đây?

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| `connection refused` | Server chưa chạy hoặc sai transport | Kiểm tra khớp transport (stdio vs streamable-http) hai phía |
| Agent không thấy tool nào | Chưa fetch tool list sau khi kết nối | Gọi bước liệt kê tool trước `.build()` |
| Feature không tồn tại khi build | Thiếu feature flag | `rig = { version = "0.41", features = ["rmcp"] }` |
| Agent treo khi server chậm | Không có timeout | Bọc lời gọi trong `tokio::time::timeout` |

## Tóm tắt
Model Context Protocol (MCP) là mảnh ghép cuối cùng giúp hệ sinh thái Agentic AI trở nên module hóa và mở rộng không giới hạn. Thay vì viết lại logic cho từng framework, hãy đóng gói chúng thành các MCP Servers.

Trong chương tiếp theo, chúng ta sẽ giải quyết một thách thức khổng lồ khác trong thế giới LLM: Làm sao để kiểm thử (Test) và đánh giá (Evaluate) tính đúng đắn của Agent một cách tự động và nghiêm ngặt? Hẹn gặp lại bạn ở Chương 52.
