// filename: src/lib.rs

pub fn first_element(list: &[i32]) -> i32 {
    list[0]  // panics nếu list rỗng!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "index out of bounds")] // Báo cho Rust: Hàm này chạy MÀ KHÔNG LỖI LÀ SAI!
    fn first_element_panics_on_empty() {
        first_element(&[]);
    }
}

fn main() {}
