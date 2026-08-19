// filename: src/main.rs
use std::rc::Rc;

#[derive(Debug)]
enum Stack<T> {
    Empty,
    Node(T, Rc<Stack<T>>),
}

impl<T: Clone + std::fmt::Debug> Stack<T> {
    fn new() -> Rc<Stack<T>> {
        Rc::new(Stack::Empty)
    }

    fn push(value: T, stack: &Rc<Stack<T>>) -> Rc<Stack<T>> {
        Rc::new(Stack::Node(value, Rc::clone(stack)))
    }

    fn pop(stack: &Rc<Stack<T>>) -> (Option<T>, Rc<Stack<T>>) {
        match stack.as_ref() {
            Stack::Empty => (None, Rc::clone(stack)),
            Stack::Node(val, rest) => (Some(val.clone()), Rc::clone(rest)),
        }
    }

    fn peek(stack: &Rc<Stack<T>>) -> Option<&T> {
        match stack.as_ref() {
            Stack::Empty => None,
            Stack::Node(val, _) => Some(val),
        }
    }

    fn is_empty(stack: &Rc<Stack<T>>) -> bool {
        matches!(stack.as_ref(), Stack::Empty)
    }
}

fn main() {
    let s0 = Stack::<i32>::new();
    let s1 = Stack::push(10, &s0);
    let s2 = Stack::push(20, &s1);
    let s3 = Stack::push(30, &s2);

    println!("Top: {:?}", Stack::peek(&s3));  // Some(30)

    let (val, s4) = Stack::pop(&s3);
    println!("Popped: {:?}", val);            // Some(30)
    println!("New top: {:?}", Stack::peek(&s4)); // Some(20)

    // s3 vẫn tồn tại nguyên vẹn!
    println!("s3 top: {:?}", Stack::peek(&s3)); // Some(30)

    // Output:
    // Top: Some(30)
    // Popped: Some(30)
    // New top: Some(20)
    // s3 top: Some(30)
}
