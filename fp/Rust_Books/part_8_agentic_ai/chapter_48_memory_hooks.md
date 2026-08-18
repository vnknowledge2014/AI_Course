# Chapter 48 — Nâng cao Tool Use: Context, Memory & Hooks

> **Bạn sẽ học được**:
> - `ChatHistory` và vì sao LLM API vốn stateless
> - `rig-memory`: sliding window và token budget để chặn context phình vô hạn
> - `AgentHook` — chặn trước khi tool chạy, dựng Human-in-the-Loop
> - Phục hồi từ `InvalidToolCall` bằng `Flow`
>
> **Yêu cầu trước**: Chapter 47
> **Thời gian đọc**: ~35 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Agent nhớ được ngữ cảnh và phải xin phép trước khi làm việc nguy hiểm.

Ở chương trước, Agent của chúng ta đã được trang bị tools để hành động. Tuy nhiên, một đặc điểm của việc lập trình Stateless API với LLM là mô hình bị "mất trí nhớ" sau mỗi request. Nếu không có bộ nhớ (Memory) và bối cảnh (Context), Agent sẽ giống như một người bị bệnh Alzheimer, không thể theo dõi một luồng công việc dài hơi. Hơn nữa, khi giao cho AI quyền gọi tools nguy hiểm, chúng ta cần cơ chế kiểm soát chặt chẽ vòng đời của một request.

Trong chương này, chúng ta sẽ khai phá Memory System và Agent Hooks trong Rig để xây dựng những Agent có tính liên tục và an toàn, sẵn sàng cho môi trường production.

## 48.1 — Hệ thống Memory: Quản lý Context

Mọi trao đổi giữa người dùng và Agent được mô tả dưới dạng `Message` (System, User, Assistant). Vấn đề là làm sao truyền tải đoạn lịch sử chat (History) này một cách hiệu quả và an toàn.

### Automatic Conversation Memory

Cách tiếp cận nguyên thủy nhất là tự lưu một `Vec<Message>` và truyền vào `with_history(...)`. Dù linh hoạt, việc tự tay duy trì state này dễ sinh lỗi lặt vặt.

Rig cung cấp trait `ConversationMemory` để tự động hóa quy trình load và lưu lịch sử chat. Rig cung cấp sẵn một backend in-memory là `InMemoryConversationMemory`.

```rust
use rig::memory::InMemoryConversationMemory;

let agent = client
    .agent("gpt-4o")
    .preamble("Bạn là trợ lý đắc lực của tôi.")
    .memory(InMemoryConversationMemory::new())
    .build();

// Session 1: Gán cho agent một Conversation ID
let _ = agent.prompt("Tên tôi là Mike.").conversation("session-123").await?;

// Session 2: Vẫn dùng ID đó, Agent sẽ "nhớ" ra
let reply = agent.prompt("Tôi tên là gì nhỉ?").conversation("session-123").await?;
println!("{reply}"); // In ra: Tên của bạn là Mike.
```

Nguyên tắc ở đây: nếu bạn set `.conversation("id")`, Rig sẽ dùng trait `ConversationMemory` để `load` lịch sử trước khi gọi LLM, và `append` cả prompt lẫn response (kể cả quá trình tool call) vào memory sau khi gọi thành công.

### Bounding History với Policies (rig-memory)

Memory không thể phình to mãi mãi. Context window của LLM có hạn, và càng nhét nhiều rác, mô hình càng dễ "hallucinate". Để chống lại điều này, thư viện bổ trợ `rig-memory` cung cấp các Policy để cắt gọt memory một cách có hệ thống.

Một policy kinh điển là `SlidingWindowMemory` (chỉ giữ N messages gần nhất) hoặc `TokenWindowMemory` (giữ theo số lượng tokens):

```rust
use rig_memory::{IntoFilter, SlidingWindowMemory};

let memory = InMemoryConversationMemory::new()
    .with_filter(SlidingWindowMemory::last_messages(20).into_filter());
```

Khi sử dụng `rig-memory`, nếu một tool call bị cắt khỏi cửa sổ trượt nhưng tool result vẫn còn (hoặc ngược lại), policy sẽ tự động dọn dẹp để đảm bảo tính hợp lệ (bảo vệ khỏi việc gửi request rác đến OpenAI).

## 48.2 — Hooks và Approval Policy (Human-in-the-Loop)

Khi các tool trở nên nguy hiểm (như Xóa Database, Chuyển Tiền, Chạy Shell Script), ta không thể phó mặc 100% cho AI tự quyết. Chúng ta cần những điểm chặn (Hooks) để theo dõi, kiểm soát, và thay đổi luồng thực thi.

Trong Rig, bạn có thể can thiệp vào vòng lặp của `AgentRunner` thông qua trait `AgentHook`.

### Giải phẫu AgentHook

Mọi hook trong Rig đều implement một hàm duy nhất: `on_event`. Nó nhận vào một `StepEvent` (như CompletionCall, ToolCall, ToolResult, v.v.) và phải trả về một `Flow`.

- `StepEvent`: Cho biết chuyện gì đang xảy ra (AI đang nghĩ? Tool sắp chạy? Lỗi ở đâu?).
- `Flow`: Quyết định của bạn (Tiếp tục `cont()`, Huỷ bỏ `terminate()`, Bỏ qua `skip()`, hoặc Bắt thử lại `retry()`).

### Viết một Guardrail Hook: Human-In-The-Loop

Hãy xây dựng một Approval Policy: Nếu AI gọi tool "transfer_funds" với số tiền quá 1,000 USD, Hook sẽ nhảy vào can thiệp (chặn lại hoặc yêu cầu approve). Dưới đây ta sẽ từ chối tự động chuyển tiền bằng `Flow::skip()`.

```rust
use rig::agent::{AgentHook, Flow, StepEvent};
use rig::completion::CompletionModel;

struct FinancialGuardrail {
    max_amount: u64,
}

impl<M: CompletionModel> AgentHook<M> for FinancialGuardrail {
    async fn on_event(&self, event: StepEvent<'_, M>) -> Flow {
        // Chỉ quan tâm sự kiện ToolCall
        let StepEvent::ToolCall { tool_name, args, .. } = event else {
            return Flow::cont();
        };

        if tool_name != "transfer_funds" {
            return Flow::cont();
        }

        // Tự bóc tách JSON args (vì lúc này args vẫn đang là chuỗi thô)
        let amount = serde_json::from_str::<serde_json::Value>(args)
            .ok()
            .and_then(|v| v.get("amount").and_then(|a| a.as_u64()));

        match amount {
            Some(n) if n <= self.max_amount => {
                // Hợp lệ, cho phép tool chạy bình thường
                Flow::cont()
            },
            Some(n) => {
                // Vi phạm policy, BỎ QUA tool này và trả lỗi ngược về cho AI hiểu
                Flow::skip(format!(
                    "Chính sách bảo mật từ chối: Giao dịch ${n} vượt ngưỡng tự động ${}.",
                    self.max_amount
                ))
            },
            None => Flow::skip("Chính sách bảo mật từ chối: Không tìm thấy lượng tiền."),
        }
    }
}
```

Để gắn hook vào request, chỉ cần dùng `.add_hook()`:

```rust
let response = agent
    .runner("Hãy chuyển 5000 USD vào tài khoản Bob.")
    .max_turns(3)
    .add_hook(FinancialGuardrail { max_amount: 1000 })
    .run()
    .await?;
```

Luồng chạy lúc này diễn ra như sau:
1. LLM nhận lệnh, trả về yêu cầu gọi tool `transfer_funds` với `args: {"amount": 5000}`.
2. Hook `FinancialGuardrail` kích hoạt tại sự kiện `StepEvent::ToolCall`.
3. Đoạn code của ta phát hiện số tiền quá 1000, nên trả về `Flow::skip(...)`.
4. Vòng lặp Agent bắt được `Flow::skip`, nó **chặn tool chạy**, giả lập một kết quả Tool trả về chuỗi báo lỗi *"Chính sách bảo mật từ chối..."*, và truyền ngược lại OpenAI.
5. Ở Turn 2, LLM đọc được kết quả bị từ chối và xin lỗi người dùng.

### Phục hồi lỗi với `InvalidToolCall`

Trong thực tế, AI có khả năng bị "ảo giác" (hallucinate) tên tool, gọi một tool không hề tồn tại (ví dụ gọi tool `email_user` thay vì `send_email`). 

Rig có sự kiện `InvalidToolCall`. Ta có thể dùng `Flow::retry(feedback)` để cung cấp cho mô hình cơ hội sửa sai:

```rust
struct RepairToolHook;

impl<M: CompletionModel> AgentHook<M> for RepairToolHook {
    async fn on_event(&self, event: StepEvent<'_, M>) -> Flow {
        match event {
            StepEvent::InvalidToolCall(ctx) => {
                // Nhắc AI nhớ lại các tool có sẵn
                Flow::retry(format!("Tool {} không tồn tại. Vui lòng dùng một trong các tool: {:?}", 
                                     ctx.tool_name, ctx.available_tools))
            }
            _ => Flow::cont(),
        }
    }
}
```

Việc kết hợp ROP (từ Tool trait) và Hooks biến một vòng lặp LLM mỏng manh, dễ gãy gọn trở thành một hệ thống linh hoạt, chống chịu lỗi tốt (Resilient System) — đúng với lý tưởng mà các hệ thống Rust sinh ra để phục vụ.

---

## ✅ Checkpoint 48

1. Vì sao không thể cứ nối mãi lịch sử chat vào mỗi request?
2. Hook chạy **trước** hay **sau** khi tool thực thi? Vì sao thứ tự đó là bắt buộc cho Human-in-the-Loop?
3. Sliding window và token budget khác nhau ở đâu? Khi nào chọn cái nào?

<details>
<summary>Đáp án</summary>

1. Ba lý do cộng dồn: (a) context window có giới hạn cứng; (b) chi phí tính theo token **đầu vào**, nên lịch sử dài làm mọi request sau đắt hơn; (c) độ chính xác giảm khi thông tin quan trọng bị chôn giữa hàng nghìn token nhiễu.
2. **Trước**. Nếu hook chạy sau, tool đã xoá database rồi mới hỏi "bạn có chắc không?". Approval chỉ có nghĩa khi nó chặn được.
3. Sliding window giữ N tin nhắn gần nhất — đơn giản, dễ đoán. Token budget giữ nhiều nhất có thể trong X token — dùng context hiệu quả hơn nhưng số lượt nhớ được thì thay đổi. Hội thoại ngắn đều đặn dùng window; tin nhắn dài ngắn lẫn lộn dùng token budget.
</details>

---

## 🏋️ Bài tập

**Bài 1 (5 phút).** Đặt sliding window = 2 rồi hỏi agent ba câu liên tiếp, trong đó câu thứ ba tham chiếu tới câu thứ nhất. Quan sát nó quên như thế nào.

**Bài 2 (15 phút).** Viết hook chặn mọi tool có tên bắt đầu bằng `delete_` hoặc `restart_`, in ra `[APPROVAL] Cho phép chạy {tool}? (y/n)` và đọc từ stdin.

**Bài 3 (20 phút).** Thêm audit log vào hook: mỗi lời gọi tool ghi một dòng JSON gồm timestamp, tên tool, tham số, và quyết định approve/deny. Đây chính là structured logging của Chapter 42 áp dụng cho agent.

<details>
<summary>Gợi ý bài 3</summary>

Dùng crate `tracing` với `tracing_subscriber::fmt().json()`. Đừng `println!` — audit log cần chảy vào cùng pipeline với log hệ thống, nếu không nó vô dụng khi truy vết sự cố.
</details>

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| Chi phí token tăng dần theo hội thoại | Không giới hạn history | Áp policy của `rig-memory` |
| Hook không được gọi | Chưa đăng ký vào `AgentBuilder` | Kiểm tra `.hook(...)` trước `.build()` |
| Agent quên system prompt sau vài lượt | Policy cắt nhầm cả message hệ thống | Dùng policy giữ nguyên message `System`, chỉ cắt `User`/`Assistant` |
| Approval prompt hiện SAU khi tool đã chạy | Đặt logic ở hook hậu-thực-thi | Chuyển sang hook tiền-thực-thi và trả `Flow` chặn |

## Tóm tắt

- LLM API là **stateless** — mọi cảm giác "nhớ" đều do bạn gửi lại lịch sử.
- Lịch sử không giới hạn là một khoản nợ: đắt hơn, chậm hơn, và kém chính xác hơn. `rig-memory` cho bạn sliding window và token budget.
- `AgentHook` chạy **trước** tool, nên nó là chỗ duy nhất đặt được Human-in-the-Loop thật sự.
- Kết hợp ROP (lỗi có kiểu) với Hook (chính sách) biến vòng lặp LLM mỏng manh thành hệ thống chịu lỗi.
