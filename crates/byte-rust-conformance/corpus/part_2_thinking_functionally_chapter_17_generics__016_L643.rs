#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self { Stack { items: vec![] } }
    fn push(&mut self, item: T) { self.items.push(item); }
    fn pop(&mut self) -> Option<T> { self.items.pop() }
    fn peek(&self) -> Option<&T> { self.items.last() }
    fn is_empty(&self) -> bool { self.items.is_empty() }
    fn len(&self) -> usize { self.items.len() }
}

// Conditional: chỉ Display khi T: Display
impl<T: std::fmt::Display> Stack<T> {
    fn display(&self) -> String {
        let items: Vec<String> = self.items.iter().map(|i| i.to_string()).collect();
        format!("[{}]", items.join(" → "))
    }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);
    println!("{}", stack.display());         // [10 → 20 → 30]
    println!("Peek: {:?}", stack.peek());    // Some(30)
    println!("Pop: {:?}", stack.pop());      // Some(30)
    println!("Len: {}", stack.len());        // 2
}
