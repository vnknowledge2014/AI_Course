// filename: src/main.rs

fn bmi_calculator(height_cm: f64, weight_kg: f64) -> (f64, &'static str) {
    let height_m = height_cm / 100.0;
    let bmi = weight_kg / (height_m * height_m);

    let category = if bmi < 18.5 {
        "Underweight"
    } else if bmi < 25.0 {
        "Normal"
    } else if bmi < 30.0 {
        "Overweight"
    } else {
        "Obese"
    };

    (bmi, category)
}

fn main() {
    let people = [
        ("Minh", 170.0, 65.0),
        ("Lan", 160.0, 45.0),
        ("Hùng", 175.0, 95.0),
    ];

    for (name, height, weight) in &people {
        let (bmi, category) = bmi_calculator(*height, *weight);
        println!("{}: BMI = {:.1} → {}", name, bmi, category);
    }
    // Output:
    // Minh: BMI = 22.5 → Normal
    // Lan: BMI = 17.6 → Underweight
    // Hùng: BMI = 31.0 → Obese
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_bmi() {
        let (bmi, cat) = bmi_calculator(170.0, 65.0);
        assert!((bmi - 22.49).abs() < 0.1);
        assert_eq!(cat, "Normal");
    }
}
