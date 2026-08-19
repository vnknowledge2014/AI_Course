pub enum EvalOutcome<Output> {
    Pass(Output),     // LLM trả lời đúng tiêu chí (Thành công)
    Fail(Output),     // LLM trả lời sai tiêu chí (Thất bại logic)
    Invalid(String),  // Lỗi hệ thống: API chết, JSON parse lỗi (Lỗi hạ tầng)
}

fn main() {}
