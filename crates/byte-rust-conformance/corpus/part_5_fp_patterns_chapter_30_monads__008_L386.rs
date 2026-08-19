// filename: src/main.rs

#[derive(Debug)]
struct Department {
    name: String,
    employees: Vec<String>,
}

fn main() {
    let departments = vec![
        Department { name: "Engineering".into(), employees: vec!["Minh".into(), "Lan".into(), "Hải".into()] },
        Department { name: "Design".into(), employees: vec!["An".into(), "Bình".into()] },
        Department { name: "Marketing".into(), employees: vec!["Chi".into()] },
    ];

    // flat_map: departments → all employees
    let all_employees: Vec<&str> = departments.iter()
        .flat_map(|dept| dept.employees.iter().map(|e| e.as_str()))
        .collect();
    println!("All: {:?}", all_employees);

    // flat_map + filter
    let engineering: Vec<String> = departments.iter()
        .filter(|d| d.name == "Engineering")
        .flat_map(|d| d.employees.clone())
        .map(|name| format!("{} (Eng)", name))
        .collect();
    println!("Eng: {:?}", engineering);
}
