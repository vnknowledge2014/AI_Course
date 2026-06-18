# Chapter 26 — Persistence & Side Effects at Edges

> **Bạn sẽ học được**:
> - **Repository pattern** — trait abstraction cho data access
> - **Dependency Injection** qua traits — không cần DI container
> - **CQS** — Command Query Separation tại repository level
> - **Transaction boundaries** — consistency across operations
> - In-memory vs real database implementations
> - Testing: mock repos, no DB required
>
> **Yêu cầu trước**: Chapter 21 (Architecture), Chapter 22 (Domain Modeling), Chapter 25 (Serialization).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Domain logic **không biết** database nào đang dùng — swap Postgres ↔ SQLite ↔ in-memory tự do.

---

## Persistence — Kết nối Domain Model với Database

Đến chapter này, bạn có domain model đẹp (Ch22), workflows rõ ràng (Ch23), error handling an toàn (Ch24), serialization boundaries (Ch25). Nhưng tất cả đều ở in-memory. Để ứng dụng hoạt động thực tế, data phải được lưu xuống database và đọc lại.

Thách thức: domain model (enums, newtypes, value objects) không map trực tiếp sang SQL tables (rows, columns, foreign keys). Chương này dạy bạn cách xây dựng **Repository pattern** — tầng trung gian giữ domain model pure và cô lập database implementation details.

---

Domain model đẹp (Ch22), workflows rõ ràng (Ch23), errors an toàn (Ch24), serialization boundaries (Ch25). Nhưng tất cả in-memory. Production cần **persistence** — lưu data xuống database, đọc lại.

Thách thức: domain model (enums, newtypes, value objects) không map 1:1 sang SQL tables (rows, columns). `OrderStatus::Paid { amount, payment_id }` trong Rust = 1 enum variant. Trong SQL = row với columns `status`, `amount`, `payment_id` + NULL cho fields của variants khác.

Repository pattern giải quyết: tầng trung gian convert domain ↔ SQL. Domain stays pure, database details isolated. Swap PostgreSQL sang MongoDB? Viết Repository mới, domain không đổi.

## 26.1 — Repository = Trait

### Ẩn dụ: Thủ thư

Repository giống **thủ thư** (librarian). Bạn nói: "Tìm sách ID 42" hoặc "Lưu sách mới". Thủ thư biết sách ở đâu (kệ nào, tầng nào) — bạn **không cần biết**.

### Domain-first Repository

```rust
// filename: src/main.rs

// ═══ DOMAIN (pure, no IO) ═══
mod domain {
    #[derive(Debug, Clone, PartialEq)]
    pub struct ProductId(pub u64);

    #[derive(Debug, Clone)]
    pub struct Product {
        pub id: ProductId,
        pub name: String,
        pub price: u32,
        pub stock: u32,
    }

    impl Product {
        pub fn restock(&self, amount: u32) -> Self {
            Product { stock: self.stock + amount, ..self.clone() }
        }

        pub fn reserve(&self, qty: u32) -> Result<Self, String> {
            if qty > self.stock {
                Err(format!("Insufficient stock: have {}, need {}", self.stock, qty))
            } else {
                Ok(Product { stock: self.stock - qty, ..self.clone() })
            }
        }
    }
}

// ═══ REPOSITORY TRAIT (port) ═══
mod ports {
    use super::domain::*;

    pub trait ProductRepository {
        fn next_id(&self) -> ProductId;
        fn save(&mut self, product: &Product) -> Result<(), String>;
        fn find_by_id(&self, id: &ProductId) -> Option<Product>;
        fn find_all(&self) -> Vec<Product>;
        fn find_by_name(&self, name: &str) -> Vec<Product>;
        fn delete(&mut self, id: &ProductId) -> Result<(), String>;
    }
}

// ═══ IN-MEMORY IMPLEMENTATION (adapter) ═══
mod infrastructure {
    use super::domain::*;
    use super::ports::*;
    use std::collections::HashMap;

    pub struct InMemoryProductRepo {
        products: HashMap<u64, Product>,
        counter: u64,
    }

    impl InMemoryProductRepo {
        pub fn new() -> Self {
            InMemoryProductRepo { products: HashMap::new(), counter: 0 }
        }
    }

    impl ProductRepository for InMemoryProductRepo {
        fn next_id(&self) -> ProductId {
            ProductId(self.counter + 1)
        }

        fn save(&mut self, product: &Product) -> Result<(), String> {
            self.counter = self.counter.max(product.id.0);
            self.products.insert(product.id.0, product.clone());
            Ok(())
        }

        fn find_by_id(&self, id: &ProductId) -> Option<Product> {
            self.products.get(&id.0).cloned()
        }

        fn find_all(&self) -> Vec<Product> {
            self.products.values().cloned().collect()
        }

        fn find_by_name(&self, name: &str) -> Vec<Product> {
            let lower = name.to_lowercase();
            self.products.values()
                .filter(|p| p.name.to_lowercase().contains(&lower))
                .cloned()
                .collect()
        }

        fn delete(&mut self, id: &ProductId) -> Result<(), String> {
            self.products.remove(&id.0)
                .map(|_| ())
                .ok_or_else(|| format!("Product {:?} not found", id))
        }
    }
}

// ═══ APPLICATION (use case) ═══
mod application {
    use super::domain::*;
    use super::ports::*;

    pub fn add_product(
        repo: &mut dyn ProductRepository,
        name: &str,
        price: u32,
        initial_stock: u32,
    ) -> Result<Product, String> {
        if name.trim().len() < 2 { return Err("Name too short".into()); }
        if price == 0 { return Err("Price must be > 0".into()); }

        let product = Product {
            id: repo.next_id(),
            name: name.trim().into(),
            price, stock: initial_stock,
        };
        repo.save(&product)?;
        Ok(product)
    }

    pub fn restock_product(
        repo: &mut dyn ProductRepository,
        id: &ProductId,
        amount: u32,
    ) -> Result<Product, String> {
        let product = repo.find_by_id(id).ok_or("Product not found")?;
        let updated = product.restock(amount);
        repo.save(&updated)?;
        Ok(updated)
    }

    pub fn purchase(
        repo: &mut dyn ProductRepository,
        id: &ProductId,
        qty: u32,
    ) -> Result<Product, String> {
        let product = repo.find_by_id(id).ok_or("Product not found")?;
        let updated = product.reserve(qty)?;
        repo.save(&updated)?;
        Ok(updated)
    }
}

fn main() {
    use domain::*;
    use ports::ProductRepository;

    let mut repo = infrastructure::InMemoryProductRepo::new();

    // Add products
    let coffee = application::add_product(&mut repo, "Premium Coffee", 85_000, 50).unwrap();
    let tea = application::add_product(&mut repo, "Green Tea", 45_000, 100).unwrap();
    println!("Added: {:?}", coffee);
    println!("Added: {:?}\n", tea);

    // Purchase
    let updated = application::purchase(&mut repo, &coffee.id, 5).unwrap();
    println!("After purchase 5: stock={}\n", updated.stock);

    // Restock
    let updated = application::restock_product(&mut repo, &coffee.id, 20).unwrap();
    println!("After restock 20: stock={}\n", updated.stock);

    // Search
    let results = repo.find_by_name("coffee");
    println!("Search 'coffee': {} results", results.len());

    // List all
    println!("\nAll products:");
    for p in repo.find_all() {
        println!("  {} — {}đ (stock: {})", p.name, p.price, p.stock);
    }
}
```

---

## ✅ Checkpoint 26.1

> Ghi nhớ:
> 1. **Repository trait** = port. Domain/Application định nghĩa.
> 2. **In-memory implementation** = adapter cho testing.
> 3. Application **chỉ thấy trait**, không biết implementation cụ thể.
> 4. Swap `InMemoryProductRepo` → `PostgresProductRepo` → **không sửa application code**!

---

## 26.2 — CQS: Command Query Separation

### Tách Read và Write

```rust
// filename: src/main.rs

use std::collections::HashMap;

// ═══ Domain ═══
#[derive(Debug, Clone)]
struct Order {
    id: u64,
    customer: String,
    items: Vec<(String, u32, u32)>,
    status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum OrderStatus { Draft, Confirmed, Shipped }

// ═══ CQS: tách Command và Query ═══

// COMMANDS — thay đổi state, không return data (trừ ID/confirmation)
trait OrderCommands {
    fn save(&mut self, order: &Order) -> Result<(), String>;
    fn update_status(&mut self, id: u64, status: OrderStatus) -> Result<(), String>;
    fn delete(&mut self, id: u64) -> Result<(), String>;
}

// QUERIES — đọc data, không thay đổi state
trait OrderQueries {
    fn find_by_id(&self, id: u64) -> Option<Order>;
    fn find_by_customer(&self, customer: &str) -> Vec<Order>;
    fn find_by_status(&self, status: &OrderStatus) -> Vec<Order>;
    fn count(&self) -> usize;
    fn total_revenue(&self) -> u64;
}

// ═══ Implementation ═══
struct OrderStore {
    orders: HashMap<u64, Order>,
}

impl OrderStore {
    fn new() -> Self { OrderStore { orders: HashMap::new() } }
}

impl OrderCommands for OrderStore {
    fn save(&mut self, order: &Order) -> Result<(), String> {
        self.orders.insert(order.id, order.clone());
        Ok(())
    }

    fn update_status(&mut self, id: u64, status: OrderStatus) -> Result<(), String> {
        let order = self.orders.get_mut(&id).ok_or("Not found")?;
        order.status = status;
        Ok(())
    }

    fn delete(&mut self, id: u64) -> Result<(), String> {
        self.orders.remove(&id).map(|_| ()).ok_or("Not found".into())
    }
}

impl OrderQueries for OrderStore {
    fn find_by_id(&self, id: u64) -> Option<Order> {
        self.orders.get(&id).cloned()
    }

    fn find_by_customer(&self, customer: &str) -> Vec<Order> {
        self.orders.values()
            .filter(|o| o.customer == customer)
            .cloned().collect()
    }

    fn find_by_status(&self, status: &OrderStatus) -> Vec<Order> {
        self.orders.values()
            .filter(|o| o.status == *status)
            .cloned().collect()
    }

    fn count(&self) -> usize { self.orders.len() }

    fn total_revenue(&self) -> u64 {
        self.orders.values()
            .filter(|o| o.status == OrderStatus::Shipped)
            .map(|o| o.items.iter().map(|(_, p, q)| *p as u64 * *q as u64).sum::<u64>())
            .sum()
    }
}

// Use case cần WRITE → nhận &mut dyn OrderCommands
fn place_order(cmds: &mut dyn OrderCommands, order: Order) -> Result<(), String> {
    cmds.save(&order)
}

// Use case cần READ → nhận &dyn OrderQueries
fn generate_report(queries: &dyn OrderQueries) -> String {
    format!(
        "Orders: {} | Revenue: {}đ | Shipped: {}",
        queries.count(),
        queries.total_revenue(),
        queries.find_by_status(&OrderStatus::Shipped).len(),
    )
}

fn main() {
    let mut store = OrderStore::new();

    store.save(&Order {
        id: 1, customer: "Minh".into(),
        items: vec![("Coffee".into(), 85_000, 2)],
        status: OrderStatus::Shipped,
    }).unwrap();

    store.save(&Order {
        id: 2, customer: "Lan".into(),
        items: vec![("Tea".into(), 45_000, 1)],
        status: OrderStatus::Confirmed,
    }).unwrap();

    println!("{}", generate_report(&store));

    // Update status
    store.update_status(2, OrderStatus::Shipped).unwrap();
    println!("{}", generate_report(&store));
}
```

### CQS rules

| | Command | Query |
|---|---|---|
| **Purpose** | Thay đổi state | Đọc state |
| **Return** | `Result<(), E>` hoặc ID | Data (struct, Vec, aggregate) |
| **Side effect** | Yes (writes) | No (reads only) |
| **Rust trait** | `&mut self` | `&self` |
| **Cacheable?** | No | Yes ✅ |

---

## 26.3 — Transaction Boundaries

### Unit of Work pattern

```rust
// filename: src/main.rs

use std::collections::HashMap;

// ═══ Transaction abstraction ═══

trait UnitOfWork {
    fn begin(&mut self);
    fn commit(&mut self) -> Result<(), String>;
    fn rollback(&mut self);
}

// ═══ Transactional Repository ═══
#[derive(Clone)]
struct Account {
    id: u64,
    name: String,
    balance: i64,
}

struct AccountStore {
    accounts: HashMap<u64, Account>,
    // Staged changes
    pending: HashMap<u64, Account>,
    in_transaction: bool,
}

impl AccountStore {
    fn new() -> Self {
        AccountStore { accounts: HashMap::new(), pending: HashMap::new(), in_transaction: false }
    }

    fn seed(&mut self, account: Account) {
        self.accounts.insert(account.id, account);
    }

    fn find(&self, id: u64) -> Option<Account> {
        // In transaction? Check pending first
        if self.in_transaction {
            if let Some(a) = self.pending.get(&id) { return Some(a.clone()); }
        }
        self.accounts.get(&id).cloned()
    }

    fn save(&mut self, account: Account) {
        if self.in_transaction {
            self.pending.insert(account.id, account);
        } else {
            self.accounts.insert(account.id, account);
        }
    }
}

impl UnitOfWork for AccountStore {
    fn begin(&mut self) {
        self.pending.clear();
        self.in_transaction = true;
        println!("  [TX] Begin");
    }

    fn commit(&mut self) -> Result<(), String> {
        // Apply all pending changes
        for (id, account) in self.pending.drain() {
            self.accounts.insert(id, account);
        }
        self.in_transaction = false;
        println!("  [TX] Committed");
        Ok(())
    }

    fn rollback(&mut self) {
        self.pending.clear();
        self.in_transaction = false;
        println!("  [TX] Rolled back");
    }
}

// ═══ Use case: Transfer money (transactional) ═══
fn transfer(store: &mut AccountStore, from_id: u64, to_id: u64, amount: i64) -> Result<(), String> {
    store.begin();

    let from = store.find(from_id).ok_or("Source account not found")?;
    let to = store.find(to_id).ok_or("Target account not found")?;

    if from.balance < amount {
        store.rollback();
        return Err(format!("Insufficient funds: {}đ < {}đ", from.balance, amount));
    }

    store.save(Account { balance: from.balance - amount, ..from });
    store.save(Account { balance: to.balance + amount, ..to });

    store.commit()?;
    Ok(())
}

fn main() {
    let mut store = AccountStore::new();
    store.seed(Account { id: 1, name: "Minh".into(), balance: 1_000_000 });
    store.seed(Account { id: 2, name: "Lan".into(), balance: 500_000 });

    // Successful transfer
    println!("Transfer 200k:");
    transfer(&mut store, 1, 2, 200_000).unwrap();
    println!("  Minh: {}đ", store.find(1).unwrap().balance);
    println!("  Lan: {}đ\n", store.find(2).unwrap().balance);

    // Failed transfer (insufficient funds)
    println!("Transfer 2M (should fail):");
    match transfer(&mut store, 1, 2, 2_000_000) {
        Err(e) => println!("  ❌ {}", e),
        Ok(_) => println!("  ✅ ok"),
    }
    // Balances unchanged after rollback
    println!("  Minh: {}đ (unchanged)", store.find(1).unwrap().balance);
    println!("  Lan: {}đ (unchanged)", store.find(2).unwrap().balance);
}
```

---

## 26.4 — Persistence Model vs Domain Model

```rust
// filename: src/main.rs
use serde::{Serialize, Deserialize};

// ═══ DOMAIN MODEL (rich, validated) ═══
mod domain {
    #[derive(Debug, Clone)]
    pub struct UserId(pub u64);

    #[derive(Debug, Clone)]
    pub struct Email(pub String);
    impl Email {
        pub fn new(s: &str) -> Result<Self, String> {
            if s.contains('@') { Ok(Email(s.to_lowercase())) }
            else { Err("Invalid email".into()) }
        }
    }

    #[derive(Debug, Clone)]
    pub struct User {
        pub id: UserId,
        pub name: String,
        pub email: Email,
        pub role: Role,
    }

    #[derive(Debug, Clone)]
    pub enum Role { Admin, Editor, Viewer }
}

// ═══ PERSISTENCE MODEL (flat, DB-friendly) ═══
mod persistence {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct UserRow {
        pub id: u64,
        pub name: String,
        pub email: String,
        pub role: String,          // "admin" | "editor" | "viewer"
        pub created_at: String,
        pub updated_at: String,
    }
}

// ═══ MAPPING ═══
impl From<domain::User> for persistence::UserRow {
    fn from(user: domain::User) -> Self {
        persistence::UserRow {
            id: user.id.0,
            name: user.name,
            email: user.email.0,
            role: match user.role {
                domain::Role::Admin => "admin",
                domain::Role::Editor => "editor",
                domain::Role::Viewer => "viewer",
            }.into(),
            created_at: "2024-01-01T00:00:00Z".into(), // DB handles this
            updated_at: "2024-01-01T00:00:00Z".into(),
        }
    }
}

impl TryFrom<persistence::UserRow> for domain::User {
    type Error = String;

    fn try_from(row: persistence::UserRow) -> Result<Self, String> {
        let email = domain::Email::new(&row.email)?;
        let role = match row.role.as_str() {
            "admin" => domain::Role::Admin,
            "editor" => domain::Role::Editor,
            "viewer" => domain::Role::Viewer,
            other => return Err(format!("Unknown role: {}", other)),
        };

        Ok(domain::User {
            id: domain::UserId(row.id),
            name: row.name,
            email,
            role,
        })
    }
}

fn main() {
    // Domain → Persistence (save)
    let user = domain::User {
        id: domain::UserId(1),
        name: "Minh".into(),
        email: domain::Email::new("minh@co.com").unwrap(),
        role: domain::Role::Admin,
    };
    let row: persistence::UserRow = user.into();
    let json = serde_json::to_string_pretty(&row).unwrap();
    println!("Save to DB:\n{}\n", json);

    // Persistence → Domain (load)
    let loaded_row: persistence::UserRow = serde_json::from_str(&json).unwrap();
    let loaded_user = domain::User::try_from(loaded_row).unwrap();
    println!("Load from DB: {:?}", loaded_user);
}
```

### Ba models

```
┌──────────────┐    From     ┌──────────────┐    serde    ┌──────────┐
│  Domain      │ ──────────→ │  Persistence │ ──────────→ │  Database│
│  Model       │ ←────────── │  Model       │ ←────────── │  (JSON)  │
│  (rich)      │   TryFrom   │  (flat)      │   deser     │          │
└──────────────┘             └──────────────┘             └──────────┘
   Email, Money,                UserRow,                    SQL rows,
   Role enum                    Strings, ints               JSON docs
```

---

## 26.5 — Testing Repository

```rust
// filename: src/main.rs

use std::collections::HashMap;

// ═══ Simplified example for testing ═══
#[derive(Debug, Clone, PartialEq)]
struct Task { id: u64, title: String, done: bool }

trait TaskRepository {
    fn save(&mut self, task: &Task) -> Result<(), String>;
    fn find(&self, id: u64) -> Option<Task>;
    fn find_all(&self) -> Vec<Task>;
    fn find_pending(&self) -> Vec<Task>;
}

struct InMemoryTaskRepo {
    tasks: HashMap<u64, Task>,
}

impl InMemoryTaskRepo {
    fn new() -> Self { InMemoryTaskRepo { tasks: HashMap::new() } }
}

impl TaskRepository for InMemoryTaskRepo {
    fn save(&mut self, task: &Task) -> Result<(), String> {
        self.tasks.insert(task.id, task.clone());
        Ok(())
    }
    fn find(&self, id: u64) -> Option<Task> { self.tasks.get(&id).cloned() }
    fn find_all(&self) -> Vec<Task> { self.tasks.values().cloned().collect() }
    fn find_pending(&self) -> Vec<Task> {
        self.tasks.values().filter(|t| !t.done).cloned().collect()
    }
}

// Use case under test
fn complete_task(repo: &mut dyn TaskRepository, id: u64) -> Result<Task, String> {
    let task = repo.find(id).ok_or("Not found")?;
    if task.done { return Err("Already completed".into()); }
    let updated = Task { done: true, ..task };
    repo.save(&updated)?;
    Ok(updated)
}

// ═══ TESTS ═══
#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> InMemoryTaskRepo {
        let mut repo = InMemoryTaskRepo::new();
        repo.save(&Task { id: 1, title: "Write code".into(), done: false }).unwrap();
        repo.save(&Task { id: 2, title: "Review PR".into(), done: false }).unwrap();
        repo.save(&Task { id: 3, title: "Deploy".into(), done: true }).unwrap();
        repo
    }

    #[test]
    fn complete_task_marks_as_done() {
        let mut repo = setup();
        let task = complete_task(&mut repo, 1).unwrap();
        assert!(task.done);
        assert_eq!(repo.find(1).unwrap().done, true);
    }

    #[test]
    fn complete_already_done_returns_error() {
        let mut repo = setup();
        let result = complete_task(&mut repo, 3);
        assert_eq!(result.unwrap_err(), "Already completed");
    }

    #[test]
    fn complete_nonexistent_returns_error() {
        let mut repo = setup();
        let result = complete_task(&mut repo, 999);
        assert_eq!(result.unwrap_err(), "Not found");
    }

    #[test]
    fn find_pending_excludes_done() {
        let repo = setup();
        let pending = repo.find_pending();
        assert_eq!(pending.len(), 2);
        assert!(pending.iter().all(|t| !t.done));
    }
}

fn main() {
    let mut repo = InMemoryTaskRepo::new();
    repo.save(&Task { id: 1, title: "Write code".into(), done: false }).unwrap();

    let completed = complete_task(&mut repo, 1).unwrap();
    println!("Completed: {:?}", completed);
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): CQS Identification

Phân loại các methods sau thành Command hoặc Query:

```rust
fn get_balance(id: u64) -> Option<u64> { ... }
fn deposit(id: u64, amount: u64) -> Result<(), Error> { ... }
fn list_transactions(id: u64) -> Vec<Transaction> { ... }
fn transfer(from: u64, to: u64, amount: u64) -> Result<(), Error> { ... }
fn count_active_accounts() -> usize { ... }
```

<details><summary>✅ Lời giải</summary>

- `get_balance` → **Query** (đọc, &self)
- `deposit` → **Command** (ghi, &mut self)
- `list_transactions` → **Query** (đọc)
- `transfer` → **Command** (ghi, thay đổi 2 accounts)
- `count_active_accounts` → **Query** (đọc, aggregate)

</details>

---

**Bài 2** (10 phút): Generic Repository

Viết generic `Repository<T>` trait:
```rust
trait Repository<T> {
    type Id;
    fn save(&mut self, entity: &T) -> Result<(), String>;
    fn find_by_id(&self, id: &Self::Id) -> Option<T>;
    fn delete(&mut self, id: &Self::Id) -> Result<(), String>;
}
```
Implement cho `InMemoryRepo<T>` sử dụng `HashMap`.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
use std::collections::HashMap;
use std::hash::Hash;

trait HasId {
    type Id: Eq + Hash + Clone;
    fn id(&self) -> &Self::Id;
}

struct InMemoryRepo<T: HasId> {
    data: HashMap<T::Id, T>,
}

impl<T: HasId + Clone> InMemoryRepo<T> {
    fn new() -> Self { InMemoryRepo { data: HashMap::new() } }

    fn save(&mut self, entity: &T) -> Result<(), String> {
        self.data.insert(entity.id().clone(), entity.clone());
        Ok(())
    }

    fn find_by_id(&self, id: &T::Id) -> Option<T> {
        self.data.get(id).cloned()
    }

    fn delete(&mut self, id: &T::Id) -> Result<(), String> {
        self.data.remove(id).map(|_| ()).ok_or("Not found".into())
    }

    fn find_all(&self) -> Vec<T> {
        self.data.values().cloned().collect()
    }
}
```

</details>

---

**Bài 3** (15 phút): Transactional workflow

Viết "Place Order" workflow với transaction:
1. Begin transaction
2. Check stock → reserve items
3. Charge payment → create receipt
4. Save order
5. Commit (hoặc rollback nếu bất kỳ step fail)

Dùng in-memory repos cho Product + Order.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
fn place_order(
    products: &mut InMemoryRepo<Product>,
    orders: &mut InMemoryRepo<Order>,
) -> Result<Order, String> {
    // Pseudo-transactional (save original state for rollback)
    let original_products: Vec<_> = products.find_all();

    // Step 1: Reserve stock
    let product = products.find_by_id(&ProductId(1)).ok_or("Product not found")?;
    let reserved = product.reserve(2).map_err(|e| {
        // No rollback needed: nothing changed yet
        e
    })?;
    products.save(&reserved).unwrap();

    // Step 2: Create order
    let order = Order { id: OrderId(1), product_id: ProductId(1), quantity: 2, status: "confirmed".into() };

    // Step 3: Charge payment (simulate failure)
    let payment_ok = true; // toggle to test rollback
    if !payment_ok {
        // Rollback: restore original products
        for p in original_products { products.save(&p).unwrap(); }
        return Err("Payment failed".into());
    }

    orders.save(&order).unwrap();
    Ok(order)
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Repository có quá nhiều methods" | Fat interface | Tách: `ReaderRepo` + `WriterRepo` (CQS) |
| "Generic repo khó dùng" | `T` constraints quá phức | Dùng `HasId` trait bound |
| "In-memory khác behavior với real DB" | Khác query semantics | Test critical queries với real DB riêng |
| "Transaction rollback phức tạp" | Manual state management | Dùng pattern: save originals → try → restore on error |

---

## Tóm tắt

- ✅ **Repository = Trait**: Domain/App định nghĩa, Infrastructure implement. Swap DB tự do.
- ✅ **CQS**: Commands (`&mut self`, write) vs Queries (`&self`, read). Tách rõ ràng.
- ✅ **Transaction boundaries**: Begin → operations → Commit/Rollback. Consistency guaranteed.
- ✅ **3 models**: Domain (rich, validated) ↔ Persistence (flat, serde) ↔ Database (rows/JSON).
- ✅ **Testing**: In-memory repos = fast, no DB setup. Test use cases trực tiếp.

## Tiếp theo

→ Chapter 27: **Evolving the Design** — chapter cuối Part IV! Bạn sẽ học thêm features mà không break design, refactoring với compiler guidance, feature flags bằng enums, và backward-compatible type evolution.
