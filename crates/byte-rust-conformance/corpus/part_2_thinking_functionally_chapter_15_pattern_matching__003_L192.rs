// filename: src/main.rs

#[derive(Debug)]
struct Address {
    city: String,
    zip: String,
}

#[derive(Debug)]
struct User {
    name: String,
    address: Option<Address>,
}

fn greeting(user: &User) -> String {
    match user {
        // Nested: User → Option → Address
        User { name, address: Some(Address { city, .. }) } =>
            format!("Hello {} from {}!", name, city),
        User { name, address: None } =>
            format!("Hello {}!", name),
    }
}

fn main() {
    let users = vec![
        User { name: "Minh".into(), address: Some(Address { city: "HCMC".into(), zip: "70000".into() }) },
        User { name: "Lan".into(), address: None },
    ];

    for u in &users {
        println!("{}", greeting(u));
    }
    // Hello Minh from HCMC!
    // Hello Lan!
}
