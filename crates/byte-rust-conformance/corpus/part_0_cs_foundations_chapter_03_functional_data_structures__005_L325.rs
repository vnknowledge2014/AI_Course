// filename: src/main.rs

#[derive(Debug)]
struct FunctionalQueue<T> {
    inbox: Vec<T>,   // thêm vào đây (push)
    outbox: Vec<T>,  // lấy ra từ đây (pop)
}

impl<T: std::fmt::Debug> FunctionalQueue<T> {
    fn new() -> Self {
        FunctionalQueue { inbox: vec![], outbox: vec![] }
    }

    // Enqueue: thêm vào inbox — O(1)
    fn enqueue(mut self, item: T) -> Self {
        self.inbox.push(item);
        self
    }

    // Dequeue: lấy từ outbox — amortized O(1)
    fn dequeue(mut self) -> (Option<T>, Self) {
        if self.outbox.is_empty() {
            // Đảo ngược inbox → outbox
            // Chỉ xảy ra khi outbox rỗng → amortized O(1)
            while let Some(item) = self.inbox.pop() {
                self.outbox.push(item);
            }
        }
        let item = self.outbox.pop();
        (item, self)
    }

    fn len(&self) -> usize {
        self.inbox.len() + self.outbox.len()
    }
}

fn main() {
    let q = FunctionalQueue::new();

    // Enqueue: A, B, C
    let q = q.enqueue("A");
    let q = q.enqueue("B");
    let q = q.enqueue("C");
    println!("Queue length: {}", q.len());

    // Dequeue — phải ra A trước (FIFO)
    let (item, q) = q.dequeue();
    println!("Dequeue: {:?}", item);   // Some("A")

    let (item, q) = q.dequeue();
    println!("Dequeue: {:?}", item);   // Some("B")

    // Enqueue thêm D
    let q = q.enqueue("D");

    let (item, q) = q.dequeue();
    println!("Dequeue: {:?}", item);   // Some("C")

    let (item, _q) = q.dequeue();
    println!("Dequeue: {:?}", item);   // Some("D")

    // Output:
    // Queue length: 3
    // Dequeue: Some("A")
    // Dequeue: Some("B")
    // Dequeue: Some("C")
    // Dequeue: Some("D")
}
