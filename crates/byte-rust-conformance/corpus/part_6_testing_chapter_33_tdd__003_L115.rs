// filename: src/lib.rs

// 🔴 RED — Viết test trước!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }
}
// Chạy test lúc này sẽ báo lỗi: "Báo cáo anh, làm gì có cái Struct nào tên là Stack đâu!"

fn main() {}
