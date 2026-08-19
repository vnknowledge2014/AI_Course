// filename: src/main.rs
#[derive(Debug)]
enum DrinkType { Coffee, Tea, Smoothie }

#[derive(Debug)]
enum Size { S, M, L }

#[derive(Debug)]
enum Topping { None, BubblePearl, Jelly }

#[derive(Debug)]
enum Status { New, Preparing, Ready, PickedUp }

#[derive(Debug)]
struct CafeOrder {
    drink: DrinkType,
    size: Size,
    topping: Topping,
    status: Status,
}

fn describe(order: &CafeOrder) -> String {
    let name = match order.drink {
        DrinkType::Coffee => "Cà phê",
        DrinkType::Tea => "Trà",
        DrinkType::Smoothie => "Sinh tố",
    };
    let size = match order.size {
        Size::S => "nhỏ", Size::M => "vừa", Size::L => "lớn",
    };
    let topping = match order.topping {
        Topping::None => "".to_string(),
        Topping::BubblePearl => " + trân châu".to_string(),
        Topping::Jelly => " + thạch".to_string(),
    };
    let icon = match order.status {
        Status::New => "📝",
        Status::Preparing => "☕",
        Status::Ready => "✅",
        Status::PickedUp => "🎉",
    };
    format!("{} {} size {}{}", icon, name, size, topping)
}

fn main() {
    // Tổng combo: 3 × 3 × 3 = 27 loại đồ uống
    // Kể cả status: 27 × 4 = 108 trạng thái
    println!("Total drink combos: {}", 3 * 3 * 3);
    println!("Total states: {}", 3 * 3 * 3 * 4);

    let order = CafeOrder {
        drink: DrinkType::Coffee,
        size: Size::M,
        topping: Topping::BubblePearl,
        status: Status::Preparing,
    };
    println!("{}", describe(&order));
    // Output:
    // Total drink combos: 27
    // Total states: 108
    // ☕ Cà phê size vừa + trân châu
}
