// Mô hình Event cho Multi-Agent
enum AgentEvent {
    CodeWritten { task_id: String, code: String },
    TestsFailed { task_id: String, errors: String },
    FeatureCompleted { task_id: String },
}

// Workflow (Saga Pattern)
// 1. Coder sinh code -> Emit CodeWritten
// 2. Tester nghe CodeWritten -> Chạy Test
// 3. Nếu lỗi -> Emit TestsFailed. Coder nghe lỗi và sửa lại.

fn main() {}
