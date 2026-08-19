// Option<T> thực chất là:
enum Option<T> {
    None,    // 1 state
    Some(T), // |T| states
}
// Tổng states = 1 + |T|

// Option<bool> = 1 + 2 = 3 states: None, Some(true), Some(false)
// Option<u8>   = 1 + 256 = 257 states

fn main() {}
