# Chapter 47 — Quyền Năng Của Tools (Tool Calling)

> **Bạn sẽ học được**:
> - Định nghĩa `Tool` bằng trait thủ công và bằng macro `#[rig_tool]`
> - `ToolExecutionError` và `ToolErrorKind` — phân loại lỗi để Rig biết có nên retry
> - Railway-Oriented Programming cho AI: biến `Err` thành feedback để model tự sửa
> - `max_turns` — vì sao agent cần ngân sách lượt gọi
>
> **Yêu cầu trước**: Chapter 24 (ROP), Chapter 46
> **Thời gian đọc**: ~35 phút | **Level**: Advanced
> **Kết quả cuối cùng**: RustyOps gọi được tool thật và tự phục hồi khi tool trả lỗi.

Chào mừng bạn quay trở lại với hành trình xây dựng Agentic AI trong Rust. Ở các chương trước, chúng ta đã biến LLM từ một hàm nhận text/trả text thành một hệ thống type-safe với Structured Output. Nhưng sức mạnh thực sự của một Agent nằm ở khả năng tương tác với thế giới bên ngoài. Một Agent cần phải biết tra cứu database, gọi API, thao tác file, và thực thi các hành động vật lý.

Trong chương này, chúng ta sẽ tìm hiểu cách trang bị "chân tay" cho bộ não LLM thông qua khái niệm **Tool Calling** trong Rig. Và tất nhiên, chúng ta sẽ áp dụng triệt để những triết lý đã làm nên sức mạnh của Rust: Type-Safety và Railway-Oriented Programming (ROP).

## 47.1 — Giải phẫu một Tool: Trait `Tool`

Rig định nghĩa mọi tool thông qua trait `rig::tool::Tool`. Việc thiết kế này một lần nữa khẳng định triết lý Hexagonal Architecture (Part 6) mà chúng ta đang theo đuổi: Tool chính là một adapter kết nối Agent (Functional Core) với thế giới bên ngoài (Imperative Shell).

Hãy cùng mổ xẻ trait `Tool`:

```rust
pub trait Tool {
    const NAME: &'static str;
    type Args: for<'a> Deserialize<'a>;
    type Output;
    type Error: std::error::Error;

    async fn definition(&self, prompt: String) -> ToolDefinition;
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error>;
}
```

Hãy chú ý đến cách Rig sử dụng Type System:
- `Args`: Tham số đầu vào mà LLM phải truyền. Nó bắt buộc phải implement `Deserialize`, đảm bảo Type-Safety ngay tại network boundary.
- `Output`: Kiểu dữ liệu trả về nếu tool thành công.
- `Error`: Kiểu lỗi nếu tool thất bại, bắt buộc implement `std::error::Error`.

Đặc biệt nhất là hàm `call` trả về một `Result<Self::Output, Self::Error>`. Điều này có quen thuộc không? Đó chính là nền tảng của Railway-Oriented Programming (Part 4)!

### Tự tay build một Tool: Calculator

Giả sử chúng ta muốn Agent có khả năng tính toán chuẩn xác. Hãy implement một tool cộng hai số:

```rust
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, JsonSchema)]
struct MathArgs {
    /// Số hạng thứ nhất
    x: i32,
    /// Số hạng thứ hai
    y: i32,
}

#[derive(Debug)]
struct MathError(String);

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lỗi toán học: {}", self.0)
    }
}
impl std::error::Error for MathError {}

#[derive(Deserialize, Serialize)]
struct Adder;

impl Tool for Adder {
    const NAME: &'static str = "add";
    type Error = MathError;
    type Args = MathArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = schemars::schema_for!(MathArgs);
        
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Cộng hai số nguyên x và y".to_string(),
            parameters: serde_json::to_value(parameters).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Thực thi business logic của tool
        Ok(args.x + args.y)
    }
}
```

Chúng ta sử dụng `schemars` để tự động suy luận ra JSON Schema từ struct `MathArgs` của Rust. Chú ý các comment `///`: chúng sẽ được `schemars` chuyển thành `description` trong schema, giúp LLM hiểu rõ ý nghĩa của từng tham số.

## 47.2 — Macro `#[rig_tool]`: Ngắn gọn và thanh lịch

Tuy viết trait mang lại sự kiểm soát tối đa, việc tạo ra hàng chục tools sẽ sinh ra nhiều boilerplate. May mắn thay, Rig cung cấp attribute macro `#[rig_tool]` (crate `rig-agent` re-export từ `rig-derive`) giúp chuyển đổi trực tiếp một hàm Rust thông thường thành một `Tool` struct.

> **Tên macro:** tên chuẩn là **`rig_tool`**. `rig_agent` còn export thêm alias
> `tool_macro` (`pub use rig_derive::rig_tool as tool_macro;`) cho code cũ —
> tài liệu chính thức dùng `rig_tool`, nên chương này theo tên đó.

```rust
use rig_agent::rig_tool;
use rig_core::tool::ToolExecutionError;

#[rig_tool(description = "Trừ hai số nguyên, trả về x - y")]
async fn subtract(x: i32, y: i32) -> Result<i32, ToolExecutionError> {
    Ok(x - y)
}
```

> **Về `required(...)`:** từ 0.4x, tính bắt buộc được **suy ra từ kiểu tham số** —
> mọi tham số không phải `Option<T>` đều là required. Chỉ khai báo `required(...)`
> khi bạn muốn ghi đè mặc định đó.

Macro này sẽ sinh ra một struct tên là `Subtract` (PascalCase của tên hàm) implement trait `Tool` ở dưới background, xử lý việc parse arguments và tạo schema tự động. 

### Gắn Tools vào Agent

Chỉ cần gọi `.tool()` trong AgentBuilder:

```rust
let calculator = client
    .agent("gpt-4o")
    .preamble("Bạn là một máy tính bỏ túi. Hãy dùng các tools được cung cấp để giải toán.")
    .max_turns(5) // Cho phép agent gọi tool tối đa 5 lần trong một phiên
    .tool(Adder)
    .tool(Subtract)
    .build();

let answer = calculator.prompt("10 cộng 5, rồi lấy kết quả trừ đi 3 bằng bao nhiêu?").await?;
println!("{}", answer);
```

**Tại sao cần `max_turns`?**
Khi Agent cần gọi nhiều tools nối tiếp nhau (chaining tools), Rig sẽ lặp lại chu trình (ReAct loop). Bạn cần cấp "ngân sách" số lượt (turns) bằng `max_turns(n)`, nếu không sẽ gặp lỗi `MaxTurnsError`.

## 47.3 — Railway-Oriented Programming (ROP) cho AI: Self-Correction

Trong lập trình thông thường, một lỗi (`Err`) có thể làm crash hệ thống, hoặc ta phải chủ động xử lý nó (Pattern Matching). Vậy nếu Tool trả về lỗi thì sao?

Trong Rig, nếu hàm `call()` trả về `Err`, Agent **không** bị crash. Thay vào đó, Rig ép chuỗi lỗi thành String và *trả về lại cho LLM* như là kết quả của việc gọi Tool.

Điều này tạo ra một cơ chế tự sửa lỗi (Self-Correction loop) cực kỳ mạnh mẽ, nơi LLM đóng vai trò như một kỹ sư xử lý lỗi:

```rust
use rig_agent::rig_tool;
use rig_core::tool::ToolExecutionError;

#[rig_tool(description = "Chia x cho y")]
async fn divide(x: i32, y: i32) -> Result<i32, ToolExecutionError> {
    if y == 0 {
        // `invalid_args` phân loại lỗi là ToolErrorKind::InvalidArgs —
        // Rig dùng phân loại này cho telemetry và chính sách retry.
        return Err(ToolExecutionError::invalid_args(
            "Không thể chia cho 0. Hãy truyền y khác 0.",
        ));
    }
    Ok(x / y)
}
```

> **`ToolExecutionError` chứ không phải `ToolError`.** Rig 0.41 phân loại lỗi qua
> `ToolErrorKind` (`InvalidArgs`, `Timeout`, `NotFound`, `PermissionDenied`,
> `RateLimited`, `Provider`, `Network`, `Other`) với constructor tương ứng
> (`invalid_args`, `timeout`, ...). Phân loại này quyết định lỗi có được retry
> tự động hay không — thông tin mà một `String` lỗi trần không mang được.

Nếu LLM truyền `y = 0`, nó sẽ nhận được message: `"Lỗi: Không thể chia cho 0. Vui lòng kiểm tra lại tham số y."`.
Nhờ có ngữ cảnh từ lỗi, LLM có thể nhận ra sai lầm của mình, sửa lại tham số và gọi lại tool ngay trong lượt (turn) tiếp theo.

**Nguyên tắc thiết kế lỗi cho AI:**
1. **Viết thông báo lỗi cho LLM đọc:** "Lỗi chia cho không" tốt hơn nhiều so với mã lỗi vô tri như "Error 500". Hãy coi LLM như một Developer đang đọc log; hãy chỉ cho nó biết nó sai ở đâu và cách khắc phục.
2. **Dự trù số Turns cho việc khôi phục (Recovery budget):** Mỗi lần thử lại sẽ tiêu tốn một turn. Hãy thiết lập `max_turns` hợp lý để Agent có không gian "tự ngã tự đứng dậy".

Bằng cách tận dụng Result của Rust, chúng ta đã xây dựng một cơ chế ROP vượt ra khỏi biên giới code, lan rộng vào chính khả năng suy luận của LLM, biến hệ thống trở nên cực kỳ "Robust" trước những "ảo giác" (hallucinations) của AI.

---

## ✅ Checkpoint 47

1. Tool trả `Err`. Agent crash, dừng, hay tiếp tục? Điều đó tạo ra cơ chế gì?
2. Vì sao `ToolExecutionError` phân loại bằng `ToolErrorKind` thay vì chỉ mang một `String`?
3. Bỏ `max_turns` đi thì rủi ro cụ thể là gì?

<details>
<summary>Đáp án</summary>

1. Tiếp tục. Rig đưa nội dung lỗi trở lại cho LLM như kết quả của lời gọi tool — tạo thành **self-correction loop**: model đọc lỗi, sửa tham số, gọi lại.
2. Vì phân loại quyết định **chính sách**: `RateLimited` và `Network` nên retry, `InvalidArgs` thì retry vô ích (phải để model sửa), `PermissionDenied` thì tuyệt đối không retry. Một `String` không mang được thông tin đó.
3. Vòng lặp không giới hạn: model gọi tool → lỗi → gọi lại → lỗi… Mỗi vòng là một lần tính tiền API. Một bug nhỏ trong tool có thể đốt sạch quota trong vài phút.
</details>

---

## 🏋️ Bài tập

**Bài 1 (5 phút).** Viết tool `disk_usage(path: String) -> Result<u64, ToolExecutionError>` trả về `ToolExecutionError::not_found(...)` khi đường dẫn không tồn tại. Quan sát agent phản ứng thế nào khi bạn hỏi về một thư mục không có thật.

**Bài 2 (10 phút).** Đổi tham số của một tool sang `Option<T>` và quan sát JSON schema sinh ra thay đổi ra sao. Đối chiếu với quy tắc "required suy từ kiểu tham số" ở mục 47.2.

**Bài 3 (20 phút).** Viết tool `restart_service(name: String)` **luôn** trả `permission_denied` kèm thông điệp "cần phê duyệt của admin". Chạy agent với `max_turns(5)` và ghi log số lượt nó thử lại. Đây là bài chuẩn bị cho Approval Policy ở Chapter 48.

<details>
<summary>Gợi ý bài 3</summary>

Điều đáng chú ý: model **không nên** thử lại một lỗi `PermissionDenied` — nó phải báo lại cho người dùng. Nếu nó vẫn thử lại nhiều lần, đó là tín hiệu preamble của bạn chưa nói rõ về quyền hạn. Đây là lý do `ToolErrorKind` tồn tại.
</details>

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| `MaxTurnsError` | Agent kẹt trong vòng gọi tool | Tăng `max_turns`, hoặc sửa mô tả tool cho rõ hơn |
| Model không bao giờ gọi tool | `description()` mơ hồ | Viết mô tả theo hướng "khi nào dùng", không phải "nó làm gì" |
| Model gọi tool sai tham số liên tục | Schema thiếu ràng buộc | Dùng kiểu hẹp (`enum`, newtype) thay vì `String` |
| `the trait Tool is not implemented` | Quên `use rig_agent::rig_tool` | Macro nằm ở `rig-agent`, không phải `rig-core` |

## Tóm tắt

- **Tool là bàn tay của Agent** — nhưng ranh giới an toàn nằm ở hệ kiểu, không ở lời nhắc.
- `#[rig_tool]` sinh `parameters()` từ `JsonSchema`, giữ schema và deserializer luôn khớp.
- `ToolExecutionError` + `ToolErrorKind` biến lỗi thành **dữ liệu có phân loại**, để Rig quyết định retry.
- `Err` không làm sập agent — nó quay lại LLM như feedback, tạo self-correction loop. Đây chính là ROP (Chapter 24) áp dụng cho AI.
- `max_turns` là ngân sách, không phải tuỳ chọn: thiếu nó là thiếu cầu chì.
