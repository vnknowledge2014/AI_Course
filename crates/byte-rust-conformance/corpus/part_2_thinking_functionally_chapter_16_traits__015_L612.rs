// (a) HashMap key: cần Hash + Eq + PartialEq
// (b) sort: cần Ord + PartialOrd + Eq + PartialEq
// (c) println!("{}") = Display → phải impl tay

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct UserId(u64);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User#{}", self.0)
    }
}

fn main() {}
