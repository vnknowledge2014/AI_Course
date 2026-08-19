use std::collections::HashMap;

struct User { name: String, address_id: Option<u64> }
struct Address { city: String, zip: String }
struct Zone { name: String, rate: u32 }

fn find_user(id: u64) -> Option<User> {
    match id {
        1 => Some(User { name: "Minh".into(), address_id: Some(10) }),
        2 => Some(User { name: "Lan".into(), address_id: None }),
        _ => None,
    }
}

fn find_address(user: &User) -> Option<Address> {
    user.address_id.and_then(|id| match id {
        10 => Some(Address { city: "HCM".into(), zip: "70000".into() }),
        _ => None,
    })
}

fn find_zone(addr: &Address) -> Option<Zone> {
    match addr.city.as_str() {
        "HCM" => Some(Zone { name: "Zone A".into(), rate: 15_000 }),
        "HN" => Some(Zone { name: "Zone B".into(), rate: 25_000 }),
        _ => None,
    }
}

fn shipping_cost(zone: &Zone) -> u32 { zone.rate }

fn get_shipping(user_id: u64) -> Option<u32> {
    find_user(user_id)
        .and_then(|user| find_address(&user))
        .and_then(|addr| find_zone(&addr))
        .map(|zone| shipping_cost(&zone))
}

fn main() {
    println!("User 1: {:?}đ", get_shipping(1)); // Some(15000)
    println!("User 2: {:?}đ", get_shipping(2)); // None (no address)
    println!("User 9: {:?}đ", get_shipping(9)); // None (no user)
}
