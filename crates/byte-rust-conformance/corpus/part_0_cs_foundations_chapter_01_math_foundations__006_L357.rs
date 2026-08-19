// filename: src/main.rs

// Product type: struct = chọn TẤT CẢ = phép NHÂN
#[derive(Debug, Clone, Copy)]
enum MainDish { Pho, BunBo, ComTam }         // 3 loại

#[derive(Debug, Clone, Copy)]
enum Drink { Tea, Coffee }                    // 2 loại

#[derive(Debug, Clone, Copy)]
enum Dessert { Che, CoconutIceCream, Flan }   // 3 loại

#[derive(Debug)]
struct BreakfastCombo {
    main_dish: MainDish,   // VÀ
    drink: Drink,          // VÀ
    dessert: Dessert,
}

fn main() {
    let my_combo = BreakfastCombo {
        main_dish: MainDish::Pho,
        drink: Drink::Coffee,
        dessert: Dessert::Flan,
    };

    println!("Combo: {:?} + {:?} + {:?}",
        my_combo.main_dish, my_combo.drink, my_combo.dessert);
    // Output: Combo: Pho + Coffee + Flan

    // Tổng combo = 3 × 2 × 3 = 18
    println!("Total combos: {}", 3 * 2 * 3);
    // Output: Total combos: 18
}
