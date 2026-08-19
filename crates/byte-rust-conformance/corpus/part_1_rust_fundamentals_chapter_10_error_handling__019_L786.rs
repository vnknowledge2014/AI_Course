// filename: src/main.rs
use std::fmt;

#[derive(Debug)]
struct Record {
    name: String,
    age: u32,
    email: String,
}

#[derive(Debug)]
enum ParseError {
    EmptyInput,
    InvalidLine { line_num: usize, content: String },
    InvalidAge { line_num: usize, value: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Input is empty"),
            ParseError::InvalidLine { line_num, content } =>
                write!(f, "Line {}: invalid format '{}'", line_num, content),
            ParseError::InvalidAge { line_num, value } =>
                write!(f, "Line {}: invalid age '{}'", line_num, value),
        }
    }
}

fn parse_csv(input: &str) -> Result<Vec<Record>, ParseError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let mut records = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let line_num = i + 1;
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if parts.len() != 3 {
            return Err(ParseError::InvalidLine {
                line_num,
                content: line.to_string(),
            });
        }

        let age: u32 = parts[1].parse().map_err(|_| ParseError::InvalidAge {
            line_num,
            value: parts[1].to_string(),
        })?;

        records.push(Record {
            name: parts[0].to_string(),
            age,
            email: parts[2].to_string(),
        });
    }

    Ok(records)
}

fn main() {
    let valid = "Minh,25,minh@email.com\nLan,30,lan@email.com";
    match parse_csv(valid) {
        Ok(records) => {
            for r in &records {
                println!("✅ {} ({}): {}", r.name, r.age, r.email);
            }
        }
        Err(e) => println!("❌ {}", e),
    }

    let invalid = "Minh,25,minh@email.com\nBad Line\nLan,30,lan@email.com";
    match parse_csv(invalid) {
        Ok(_) => println!("OK"),
        Err(e) => println!("❌ {}", e),
    }

    // Output:
    // ✅ Minh (25): minh@email.com
    // ✅ Lan (30): lan@email.com
    // ❌ Line 2: invalid format 'Bad Line'
}
