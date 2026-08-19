// filename: src/main.rs

#[derive(Debug, Clone)]
enum BST<T> {
    Empty,
    Node {
        value: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>,
    },
}

fn main() {}
