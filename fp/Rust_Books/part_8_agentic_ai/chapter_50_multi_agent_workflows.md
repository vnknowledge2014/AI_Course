# Chapter 50 — Multi-Agent Workflows & Orchestration

> **Bạn sẽ học được**:
> - Đóng gói một Agent thành `Tool` — vì sao tính kết hợp là ý tưởng trung tâm
> - Mô hình Manager–Worker (Orchestrator) và Agent Routing
> - Swarm behavior bằng Actor pattern trên `tokio::sync::mpsc`
> - Multi-agent debate để tăng độ chính xác, và cái giá của nó
>
> **Yêu cầu trước**: Chapter 47, Chapter 36 (Concurrency & Async)
> **Thời gian đọc**: ~30 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Một orchestrator điều phối nhiều agent chuyên biệt.

Khi hệ thống AI của bạn phát triển, việc cố gắng nhồi nhét 30+ Tools và hàng loạt trách nhiệm vào một Agent duy nhất (Monolithic Agent) sẽ dẫn đến thảm họa: Agent bắt đầu bị rối loạn, gọi sai Tool, ảo giác (hallucination) và làm cạn kiệt Context Window.

Giải pháp cho vấn đề này là mô hình **Multi-Agent Systems (MAS)** — phân chia công việc cho nhiều Agent chuyên biệt, tương tự như kiến trúc Microservices. Trong chương này, chúng sẽ xem xét cách Rig ứng dụng triết lý Functional Programming (đặc biệt là tính Composition) và Concurrency của Rust (thông qua `tokio`) để điều phối nhiều Agent.

## 50.1 — Tính Kết Hợp (Composition): Agent Bản Chất Là Một Tool

Trong Functional Programming, tính kết hợp (Composition) cho phép chúng ta xây dựng các hàm phức tạp từ các hàm nhỏ hơn. Rig mang nguyên lý này vào Agentic AI thông qua một thiết kế cực kỳ thanh lịch: **Mọi Agent đều implement trait `Tool`**.

Điều này có nghĩa là bạn có thể đóng gói một Agent và biến nó thành một Tool cho một Agent khác. Đây là cơ sở của mô hình **Manager - Worker (Orchestrator)**.

### Mô hình Manager - Worker

Trong mô hình này, `Manager Agent` chịu trách nhiệm lên kế hoạch và phân rã tác vụ (Task Decomposition), sau đó gọi các `Worker Agent` để thực hiện từng phần việc cụ thể.

```rust
use rig::{
    client::ProviderClient,
    completion::Prompt,
    providers::openai,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai_client = openai::Client::from_env()?;

    // 1. Khởi tạo Worker Agent (Bob)
    let bob = openai_client
        .agent("gpt-4o")
        .name("Bob")
        .description("Nhân viên phòng Hành chính. Xử lý các tác vụ như soạn email, lên lịch.")
        .preamble("Bạn là Bob, nhân viên Hành chính. Alice là quản lý của bạn. Hãy hoàn thành tốt nhiệm vụ.")
        .build();

    // 2. Khởi tạo Manager Agent (Alice) - Gán Bob như một Tool
    let alice = openai_client
        .agent("gpt-4o")
        .name("Alice")
        .description("Quản lý phòng Hành chính.")
        .preamble("Bạn là Alice, quản lý phòng Hành chính. Bạn có quyền giao việc cho Bob.")
        .tool(bob) // Composition: Bob trở thành một Tool của Alice
        .build();

    // 3. Thực thi
    let res = alice
        .prompt("Hãy bảo Bob soạn một email thông báo họp toàn công ty vào sáng mai, và cho tôi xem kết quả.")
        .await?;

    println!("Alice Response:\n{}", res);
    Ok(())
}
```

Cơ chế đằng sau rất đơn giản: LLM sẽ sinh ra một cấu trúc JSON yêu cầu "gọi Tool Bob". Rig sẽ tự động bắt lấy yêu cầu này, kích hoạt Agent Bob với Prompt tương ứng, thu thập kết quả và trả về cho Alice để tổng hợp thành câu trả lời cuối cùng. 

Mọi lỗi parsing trong quá trình gọi Tool đều được xử lý bằng cơ chế ROP (Railway-Oriented Programming), giúp hệ thống tự phục hồi (Self-correction) mà không gây panic.

## 50.2 — Swarm Behavior với Actor Pattern

Khi bạn cần một hệ thống phi tập trung hơn, nơi các Agent hoạt động tự trị và tương tác ngang hàng (Peer-to-Peer), mô hình Manager-Worker sẽ trở thành "nút thắt cổ chai" (bottleneck). Lúc này, chúng ta cần vận dụng khả năng xử lý Concurrency xuất sắc của Rust với **Actor Pattern**.

Trong Actor Pattern, mỗi Agent (Actor) chạy trong một luồng độc lập (`tokio::spawn`), duy trì trạng thái nội bộ của riêng nó thông qua `Arc<RwLock<T>>`, và giao tiếp bằng cách gửi tin nhắn qua `mpsc::channel`.

### Thiết Kế Giao Tiếp Giữa Các Agent

```rust
use tokio::sync::{mpsc, RwLock};
use std::sync::Arc;

// Các loại tin nhắn (Message) mà Agent có thể nhận
#[derive(Debug, Clone)]
enum AgentMessage {
    Task(String),
    Response(String, String), // (from_agent_id, content)
    Trigger(String),
    Shutdown,
}

// Trạng thái nội bộ của Agent
struct AgentState {
    conversation_history: Vec<String>,
}

// Cấu trúc một Autonomous Agent
struct AutonomousAgent {
    id: String,
    client: rig::providers::openai::Client,
    state: Arc<RwLock<AgentState>>,
    inbox: mpsc::Receiver<AgentMessage>, // Nhận tin nhắn
    peer_channels: Arc<RwLock<Vec<mpsc::Sender<AgentMessage>>>>, // Gửi tin nhắn cho Peers
}
```

### Vòng Lặp Phản Ứng (Event Loop)

Mỗi Agent sẽ chạy một vòng lặp `run()` bất đồng bộ, sử dụng `tokio::select!` để xử lý đồng thời nhiều sự kiện: nhận tin nhắn từ hòm thư hoặc thực hiện tự kiểm tra định kỳ (Cron/Tick).

```rust
use tokio::time::{interval, Duration};

impl AutonomousAgent {
    async fn run(mut self) {
        let mut tick = interval(Duration::from_secs(10));

        loop {
            tokio::select! {
                // Nhận tin nhắn từ Channel
                Some(msg) = self.inbox.recv() => match msg {
                    AgentMessage::Shutdown => break,
                    AgentMessage::Task(task) => {
                        // Gọi LLM xử lý Task
                        if let Ok(result) = self.process_autonomous_task(&task).await {
                            // Cập nhật trạng thái thông qua RwLock
                            self.state.write().await.conversation_history.push(result.clone());
                            
                            // Phát tín hiệu (Broadcast) kết quả cho các Agent khác
                            self.broadcast_to_peers(
                                AgentMessage::Response(self.id.clone(), result)
                            ).await;
                        }
                    },
                    AgentMessage::Response(from, content) => {
                        self.state.write().await
                            .conversation_history.push(format!("Từ {}: {}", from, content));
                    },
                    _ => {}
                },
                
                // Chu kỳ tự kiểm tra (Tick)
                _ = tick.tick() => {
                    // Logic tự động (VD: tóm tắt lịch sử, lên kế hoạch tiếp theo)
                }
            }
        }
    }
    
    async fn process_autonomous_task(&self, task: &str) -> Result<String, rig::completion::PromptError> {
        // Khởi tạo Agent (có thể cấu hình thêm Tools/RAG ở đây)
        let agent = self.client.agent("gpt-4o")
            .preamble(&format!("Tên bạn là {}. Hãy xử lý tác vụ tự trị.", self.id))
            .build();
            
        agent.prompt(task).await
    }
    
    async fn broadcast_to_peers(&self, message: AgentMessage) {
        for peer in self.peer_channels.read().await.iter() {
            let _ = peer.send(message.clone()).await;
        }
    }
}
```

Với kiến trúc này, bạn hoàn toàn làm chủ Data Flow. Các Actor không chia sẻ chung bộ nhớ mutable, mọi tương tác đều qua tin nhắn (Message Passing). Rust và Tokio đảm bảo rằng Swarm của bạn không gặp phải Deadlock hay Data Race, cung cấp một nền tảng cực kỳ vững chắc (Rock-solid) cho các hệ thống Agentic phức tạp.

---

## ✅ Checkpoint 50

1. Vì sao "Agent cũng là một Tool" lại là ý tưởng quan trọng chứ không chỉ là mẹo cài đặt?
2. Actor pattern với `mpsc` giải quyết vấn đề gì mà `Arc<Mutex<State>>` không giải quyết tốt?
3. Multi-agent debate tăng độ chính xác. Nó tăng cái gì khác nữa?

<details>
<summary>Đáp án</summary>

1. Vì nó khiến hệ thống **đệ quy và đồng nhất**: orchestrator không cần biết nhánh dưới là một tool đơn giản hay cả một agent con. Đây đúng là tinh thần composition của Part II — cùng một interface ở mọi cấp.
2. Với `Arc<Mutex<...>>`, mọi agent tranh nhau một khoá; agent nào giữ khoá lâu (đang chờ LLM!) sẽ chặn tất cả. Actor thì mỗi agent sở hữu state của mình và giao tiếp bằng message, nên không có lock contention.
3. **Chi phí và độ trễ** — mỗi vòng debate là N lời gọi LLM. Ba agent tranh luận hai vòng là 6 lần tính tiền cho một câu trả lời. Chỉ dùng khi sai lầm đắt hơn token.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Tạo hai agent chuyên biệt: `LogAnalyst` (chỉ đọc log) và `MetricsAnalyst` (chỉ đọc metric). Đóng gói cả hai thành tool của một `IncidentManager`.

**Bài 2 (20 phút).** Cho orchestrator chạy hai worker **song song** bằng `tokio::join!` thay vì tuần tự. Đo lại tổng thời gian. Vì sao mức cải thiện gần bằng thời gian của worker chậm nhất chứ không phải tổng?

**Bài 3 (30 phút).** Cài multi-agent debate: ba agent trả lời độc lập, một agent thứ tư tổng hợp. Ghi lại số token của cả bốn và so với phương án một agent. Bạn có thấy đáng không?

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| Deadlock giữa các agent | Hai agent chờ message của nhau | Thiết kế luồng message một chiều, hoặc thêm timeout |
| Chi phí tăng vọt | Agent con gọi lẫn nhau không giới hạn | Đặt `max_turns` cho **mọi** cấp, kể cả agent con |
| Worker chạy tuần tự dù đã spawn | Vẫn `await` ngay sau mỗi lời gọi | Thu future vào rồi mới `join!`/`try_join!` |
| Kết quả tổng hợp mâu thuẫn | Manager không được cấp tiêu chí phân xử | Ghi rõ quy tắc ưu tiên vào preamble của manager |

## Tóm tắt
Mô hình Multi-Agent trong Rig chứng minh sức mạnh của nguyên lý Composition trong Rust. Bằng cách implement trait `Tool`, Agent dễ dàng biến thành các block logic có thể lắp ghép linh hoạt. Khi cần mở rộng quy mô, sự kết hợp giữa Actor Pattern và hệ thống kiểu dữ liệu nghiêm ngặt giúp xây dựng các Swarm an toàn, dễ bảo trì và có hiệu năng cao cho môi trường Production.
