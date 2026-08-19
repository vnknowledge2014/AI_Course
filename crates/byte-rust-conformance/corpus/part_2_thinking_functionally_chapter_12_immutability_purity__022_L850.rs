// filename: src/main.rs

#[derive(Debug, Clone)]
struct Account {
    id: u32,
    name: String,
    balance: u64,
}

impl Account {
    fn new(id: u32, name: &str, balance: u64) -> Self {
        Account { id, name: name.to_string(), balance }
    }
    fn id(&self) -> u32 { self.id }
    fn name(&self) -> &str { &self.name }
    fn balance(&self) -> u64 { self.balance }
}

// Pure functions — trả Account MỚI
fn deposit(account: &Account, amount: u64) -> Account {
    Account { balance: account.balance + amount, ..account.clone() }
}

fn withdraw(account: &Account, amount: u64) -> Result<Account, String> {
    if amount > account.balance {
        Err(format!("Insufficient funds: have {}, need {}", account.balance, amount))
    } else {
        Ok(Account { balance: account.balance - amount, ..account.clone() })
    }
}

fn transfer(from: &Account, to: &Account, amount: u64) -> Result<(Account, Account), String> {
    let new_from = withdraw(from, amount)?;
    let new_to = deposit(to, amount);
    Ok((new_from, new_to))
}

fn main() {
    let alice = Account::new(1, "Alice", 500_000);
    let bob = Account::new(2, "Bob", 200_000);

    match transfer(&alice, &bob, 150_000) {
        Ok((new_alice, new_bob)) => {
            println!("{}: {}đ → {}đ", new_alice.name(), 500_000, new_alice.balance());
            println!("{}: {}đ → {}đ", new_bob.name(), 200_000, new_bob.balance());
        }
        Err(e) => println!("Error: {}", e),
    }
    // Alice/Bob ORIGINAL vẫn nguyên!
    println!("Original Alice: {}đ", alice.balance());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit() {
        let acc = Account::new(1, "Test", 100);
        let acc = deposit(&acc, 50);
        assert_eq!(acc.balance(), 150);
    }

    #[test]
    fn test_withdraw_ok() {
        let acc = Account::new(1, "Test", 100);
        let acc = withdraw(&acc, 30).unwrap();
        assert_eq!(acc.balance(), 70);
    }

    #[test]
    fn test_withdraw_insufficient() {
        let acc = Account::new(1, "Test", 100);
        assert!(withdraw(&acc, 200).is_err());
    }

    #[test]
    fn test_transfer() {
        let a = Account::new(1, "A", 500);
        let b = Account::new(2, "B", 200);
        let (new_a, new_b) = transfer(&a, &b, 150).unwrap();
        assert_eq!(new_a.balance(), 350);
        assert_eq!(new_b.balance(), 350);
    }
}
