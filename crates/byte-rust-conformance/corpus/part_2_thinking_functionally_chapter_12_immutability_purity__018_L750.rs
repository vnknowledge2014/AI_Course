// Mutation BÊN TRONG function, không leak ra ngoài = pure!
fn sort_and_dedup(items: &[i32]) -> Vec<i32> {
    let mut result = items.to_vec();  // mut nhưng LOCAL
    result.sort();
    result.dedup();
    result  // trả immutable — caller không biết bên trong có mut
}

fn main() {}
