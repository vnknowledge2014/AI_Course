// filename: src/custom_ord.rs
use std::cmp::Ordering;

#[derive(Debug)]
struct Employee {
    id: u32,
    name: String,
    salary: u32,
}

// 1. Impl PartialEq (Bắt buộc)
impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id // Chỉ so sánh ID
    }
}

// 2. Impl Eq (Marker trait)
impl Eq for Employee {}

// 3. Impl Ord (Bắt buộc)
impl Ord for Employee {
    fn cmp(&self, other: &Self) -> Ordering {
        // Chỉ sắp xếp dựa trên ID
        self.id.cmp(&other.id)
    }
}

// 4. Impl PartialOrd (Bắt buộc, gọi ngược lại Ord)
impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut team = vec![
        Employee { id: 3, name: String::from("Charlie"), salary: 100 },
        Employee { id: 1, name: String::from("Alice"), salary: 500 },
        Employee { id: 2, name: String::from("Bob"), salary: 300 },
    ];

    // Sắp xếp tự động!
    team.sort();
    
    println!("{:#?}", team);
    // Kết quả: Alice (id 1) -> Bob (id 2) -> Charlie (id 3)
}
