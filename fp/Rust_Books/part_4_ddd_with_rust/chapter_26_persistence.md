# Chapter 26 — Persistence & Side Effects at Edges

> **Bạn sẽ học được**:
> - **Repository pattern** — trait abstraction cho thao tác truy xuất dữ liệu
> - **Dependency Injection** qua traits — không cần DI container rườm rà
> - **CQS** — Command Query Separation tại cấp độ repository
> - **Transaction boundaries** — Đảm bảo tính toàn vẹn (consistency) khi thao tác nhiều bảng
> - In-memory vs real database implementations
> - Testing: Dùng Mock Repositories để kiểm thử siêu tốc (không cần DB)
>
> **Yêu cầu trước**: Chapter 21 (Architecture), Chapter 22 (Domain Modeling), Chapter 25 (Serialization).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Domain logic **không hề biết** dữ liệu đang được lưu vào hệ quản trị cơ sở dữ liệu nào — bạn có thể hoán đổi giữa Postgres ↔ SQLite ↔ in-memory một cách tự do.

---

## Persistence — Kết nối Domain Model với Thế Giới Thực (Database)

Đến chapter này, bạn đã xây dựng được một Domain Model tuyệt đẹp (Ch22), quy trình nghiệp vụ rõ ràng (Ch23), bắt lỗi cực kì an toàn (Ch24), và có Serialization sạch sẽ (Ch25). 

Thế nhưng... tất cả mọi thứ vẫn chỉ đang nằm trên RAM (in-memory). Khi bạn tắt máy, dữ liệu bay hơi. Để ứng dụng sống sót trên Production, dữ liệu phải được **Lưu xuống (Persist)** Database và sau đó **Đọc lại (Load)**.

Sự thách thức lớn nhất ở đây là: Domain Model (gồm Enums, Newtypes, Value Objects) **không thể ăn khớp 1-1** với thiết kế Bảng (Tables, Rows, Columns, Foreign Keys) của SQL. Ví dụ, `OrderStatus::Paid { amount, payment_id }` trong Rust là một biến thể của Enum, nhưng trong SQL nó có thể dàn ra thành nhiều cột (`status`, `amount`, `payment_id`) và các cột này chứa giá trị `NULL` nếu đơn hàng có trạng thái khác.

Chương này sẽ hướng dẫn bạn xây dựng **Repository Pattern** — một tầng trung gian vững chắc. Nó giữ cho Domain Model của bạn trong sạch (Pure), và cách ly hoàn toàn các chi tiết rườm rà của Database. Khi muốn chuyển từ PostgreSQL sang MongoDB, bạn chỉ cần viết Repository mới, còn Code Logic cốt lõi thì không đổi một dòng nào!

---

## 26.1 — Repository = Trait

### Ẩn dụ: Cô Thủ thư

Repository hoạt động giống hệt một **cô thủ thư** trong thư viện.
Bạn nói với cô ấy: "Lấy cho tôi cuốn sách ID 42", hoặc "Ghi chú cuốn sách mới này vào danh sách". Bạn **không cần phải biết** cô ấy sắp xếp nó lên kệ nào, ở tầng thứ mấy, mã hóa hệ thập phân Dewey ra sao.

Trong lập trình, Repository chính là Cô thủ thư. Ứng dụng đưa cho Repo một `Product` (Entity), Repo tự biết cách "dịch" nó thành 2 câu lệnh `INSERT` và `UPDATE` vào SQL.

### Bước 1: Domain Models và Logic Thuần túy

Đầu tiên, hãy định nghĩa Domain Object. Nó không chứa thư viện cơ sở dữ liệu nào (không Diesel, không SQLx).

```rust
// filename: src/main.rs

// ═══ LỚP 1: DOMAIN (Pure, No IO) ═══
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

    // Các nghiệp vụ cốt lõi
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
```

### Bước 2: Khai báo Hợp đồng (Trait)

Ở biên của Domain, ta định nghĩa một Trait. Nó là một bản Hợp đồng quy định rõ: "Bất kỳ ai muốn làm Thủ thư cho dữ liệu Product, đều phải cung cấp các tính năng này".

```rust
// ═══ LỚP 2: REPOSITORY TRAIT (Cổng giao tiếp - Port) ═══
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
```

### Bước 3: Tạo Adapter giả lập (In-memory)

Để viết Code và Test liền mạch mà không cần cài đặt Cơ sở dữ liệu thật, ta tạo ra một bản Implementation bằng RAM (sử dụng Hashmap).

```rust
// ═══ LỚP 3: IN-MEMORY IMPLEMENTATION (Adapter) ═══
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

    // Cô thủ thư bằng RAM
    impl ProductRepository for InMemoryProductRepo {
        fn next_id(&self) -> ProductId { ProductId(self.counter + 1) }

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
            self.products.remove(&id.0).map(|_| ()).ok_or_else(|| format!("Product {:?} not found", id))
        }
    }
}
```

### Bước 4: Ứng dụng ghép nối (Application)

Lớp Application chứa các Use case (Kịch bản sử dụng). Nó yêu cầu truyền vào một cái `dyn ProductRepository`. Khi đó, nó có thể ra lệnh cho cơ sở dữ liệu làm việc mà không cần biết đó là MySQL, Redis hay chỉ là Hashmap ở bước 3!

```rust
// ═══ LỚP 4: APPLICATION (Use case) ═══
mod application {
    use super::domain::*;
    use super::ports::*;

    pub fn add_product(
        repo: &mut dyn ProductRepository, // Nhận bất kì Thủ thư nào!
        name: &str, price: u32, initial_stock: u32,
    ) -> Result<Product, String> {
        if name.trim().len() < 2 { return Err("Name too short".into()); }
        if price == 0 { return Err("Price must be > 0".into()); }

        let product = Product {
            id: repo.next_id(), name: name.trim().into(), price, stock: initial_stock,
        };
        repo.save(&product)?; // Ra lệnh lưu
        Ok(product)
    }

    pub fn purchase(
        repo: &mut dyn ProductRepository,
        id: &ProductId, qty: u32,
    ) -> Result<Product, String> {
        // Tìm → Chạy logic Pure → Lưu lại
        let product = repo.find_by_id(id).ok_or("Product not found")?;
        let updated = product.reserve(qty)?;
        repo.save(&updated)?;
        Ok(updated)
    }
}

// Chạy thử!
fn main() {
    let mut repo = infrastructure::InMemoryProductRepo::new();
    let coffee = application::add_product(&mut repo, "Premium Coffee", 85_000, 50).unwrap();
    println!("Added: {:?}", coffee);
}
```

---

## ✅ Checkpoint 26.1

> Ghi nhớ:
> 1. **Repository trait** = Giao thức (port). Được định nghĩa bởi lớp Application/Domain.
> 2. **In-memory implementation** = Adapter để kiểm thử hoặc tạo Prototype nhanh chóng.
> 3. Lớp Application **chỉ thấy trait**, không quan tâm tới cách thức lưu trữ.
> 4. Khi nâng cấp hệ thống: Bạn viết `PostgresProductRepo` và ném vào hàm `main()`. **Không cần sửa một dòng nào trong code Application**!

---

## 26.2 — CQS: Tách bạch Command và Query

Khi hệ thống lớn lên, một Trait duy nhất chứa cả hàm Đọc (Read) lẫn Ghi (Write) sẽ phình to khủng khiếp.
Nên nhớ, Đọc và Ghi là hai nhu cầu trái ngược: Đọc thường cần Tốc độ, Join nhiều bảng, và Giao diện tìm kiếm linh hoạt. Ghi thường tập trung vào Tính chính xác (Validation) và Transaction.

CQS (Command Query Separation) khuyên ta tách đôi Repository.

### Ví dụ về tách Read / Write

```rust
// filename: src/main.rs

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Order {
    id: u64, customer: String, status: OrderStatus,
}
#[derive(Debug, Clone, PartialEq)]
enum OrderStatus { Draft, Confirmed, Shipped }

// ═══ COMMANDS (Ghi) — thay đổi State ═══
trait OrderCommands {
    fn save(&mut self, order: &Order) -> Result<(), String>;
    fn update_status(&mut self, id: u64, status: OrderStatus) -> Result<(), String>;
}

// ═══ QUERIES (Đọc) — Đọc Data, không có side effects ═══
trait OrderQueries {
    fn find_by_id(&self, id: u64) -> Option<Order>;
    fn find_by_customer(&self, customer: &str) -> Vec<Order>;
    fn count(&self) -> usize;
}

// Implementation
struct OrderStore { orders: HashMap<u64, Order> }
impl OrderStore { fn new() -> Self { OrderStore { orders: HashMap::new() } } }

impl OrderCommands for OrderStore {
    fn save(&mut self, order: &Order) -> Result<(), String> {
        self.orders.insert(order.id, order.clone()); Ok(())
    }
    fn update_status(&mut self, id: u64, status: OrderStatus) -> Result<(), String> {
        let order = self.orders.get_mut(&id).ok_or("Not found")?;
        order.status = status; Ok(())
    }
}

impl OrderQueries for OrderStore {
    fn find_by_id(&self, id: u64) -> Option<Order> { self.orders.get(&id).cloned() }
    fn find_by_customer(&self, c: &str) -> Vec<Order> { 
        self.orders.values().filter(|o| o.customer == c).cloned().collect() 
    }
    fn count(&self) -> usize { self.orders.len() }
}
```

Bây giờ, Use case nào làm nhiệm vụ gì thì sẽ chỉ yêu cầu Trait tương ứng:

```rust
// Use case cần WRITE → nhận &mut dyn OrderCommands
fn place_order(cmds: &mut dyn OrderCommands, order: Order) -> Result<(), String> {
    cmds.save(&order)
}

// Use case cần READ → nhận &dyn OrderQueries (Tuyệt đối an toàn, không sợ hàm này vô tình sửa DB!)
fn generate_report(queries: &dyn OrderQueries) -> String {
    format!("Total orders: {}", queries.count())
}
```

### So sánh Command vs Query

| | Command | Query |
|---|---|---|
| **Mục đích** | Đổi dữ liệu | Đọc dữ liệu |
| **Giá trị trả về** | `Result<(), Error>` hoặc ID | Struct, Dto, Vector... |
| **Rust Trait Ref** | `&mut self` | `&self` |
| **Có thể Cache không?**| Không! (Write) | Có! Rất nên (Read) |

---

## 26.3 — Transaction Boundaries (Ranh giới giao dịch)

Khi chuyển tiền, bạn Trừ tiền của A và Cộng tiền cho B. Cả 2 thao tác này phải **cùng thành công** hoặc **cùng thất bại (Rollback)**. 
Nếu dùng Repository từng hàm đơn lẻ, rủi ro lỗi nằm ở giữa (A bị trừ, B chưa được cộng) là rất lớn.

Chúng ta cần `UnitOfWork` (Đơn vị công việc) để đóng gói Transaction.

```rust
// filename: src/main.rs

use std::collections::HashMap;

// ═══ Transaction Abstraction ═══
trait UnitOfWork {
    fn begin(&mut self);
    fn commit(&mut self) -> Result<(), String>;
    fn rollback(&mut self);
}

// ═══ Mock Implementation ═══
#[derive(Clone)]
struct Account { id: u64, name: String, balance: i64 }

struct AccountStore {
    accounts: HashMap<u64, Account>,
    pending: HashMap<u64, Account>, // Dữ liệu nháp
    in_transaction: bool,
}

impl AccountStore {
    fn new() -> Self {
        AccountStore { accounts: HashMap::new(), pending: HashMap::new(), in_transaction: false }
    }
    fn seed(&mut self, account: Account) { self.accounts.insert(account.id, account); }
    
    fn find(&self, id: u64) -> Option<Account> {
        if self.in_transaction {
            if let Some(a) = self.pending.get(&id) { return Some(a.clone()); } // Đọc từ bản nháp
        }
        self.accounts.get(&id).cloned()
    }
    
    fn save(&mut self, account: Account) {
        if self.in_transaction {
            self.pending.insert(account.id, account); // Chỉ ghi nháp
        } else {
            self.accounts.insert(account.id, account);
        }
    }
}

// Implement Transaction logic cho RAM
impl UnitOfWork for AccountStore {
    fn begin(&mut self) {
        self.pending.clear();
        self.in_transaction = true;
    }

    fn commit(&mut self) -> Result<(), String> {
        for (id, account) in self.pending.drain() {
            self.accounts.insert(id, account); // Đổ nháp vào thật
        }
        self.in_transaction = false;
        Ok(())
    }

    fn rollback(&mut self) {
        self.pending.clear(); // Xóa sạch nháp
        self.in_transaction = false;
    }
}
```

Bây giờ hãy xem Use Case thực thi Giao dịch an toàn:

```rust
// ═══ Use case: Transfer money (transactional) ═══
fn transfer(store: &mut AccountStore, from_id: u64, to_id: u64, amount: i64) -> Result<(), String> {
    store.begin();

    let from = store.find(from_id).ok_or("Source account not found")?;
    let to = store.find(to_id).ok_or("Target account not found")?;

    if from.balance < amount {
        store.rollback(); // Hủy bỏ ngay lập tức!
        return Err(format!("Insufficient funds"));
    }

    // Sửa đổi trên RAM nháp
    store.save(Account { balance: from.balance - amount, ..from });
    store.save(Account { balance: to.balance + amount, ..to });

    store.commit()?; // Ghi thật
    Ok(())
}
```

---

## 26.4 — Mô hình Dữ Liệu (Domain Model vs Persistence Model)

Domain Model thường được thiết kế phân cấp, đóng gói mạnh bằng Enum và NewType để bảo vệ Logic.
Nhưng Database SQL thì lại chuộng dạng phẳng (Flat), các kiểu nguyên thủy (String, Integer).

Không nên ép Database phải hiểu Domain, cũng không nên làm Domain xấu đi để chiều ý DB. Giải pháp: **Hai Mô Hình**.

```rust
// ═══ 1. DOMAIN MODEL (Rich, Validated) ═══
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
    pub enum Role { Admin, Editor, Viewer }

    #[derive(Debug, Clone)]
    pub struct User { pub id: UserId, pub email: Email, pub role: Role }
}

// ═══ 2. PERSISTENCE MODEL (Flat, DB-friendly) ═══
mod persistence {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct UserRow {
        pub id: u64,
        pub email: String,
        pub role: String, // "admin" | "editor" | "viewer"
        pub created_at: String,
    }
}
```

Sau đó, hãy viết các hàm Convert (chuyển đổi) qua lại giữa 2 dạng này nằm ở lớp Adapter (Infrastructure).
Repository sẽ dùng các hàm Convert này:
- Lấy `Domain` → Đổi thành `Persistence` → Nhét vào Database.
- Đọc Database → Trả ra `Persistence` → Đổi thành `Domain` → Gửi về cho Ứng dụng xử lý.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): CQS Identification

Phân loại các methods sau thành Command hay Query:
- `fn deposit(id: u64, amount: u64) -> Result<(), Error>`
- `fn list_transactions(id: u64) -> Vec<Transaction>`
- `fn transfer(from: u64, to: u64, amount: u64) -> Result<(), Error>`

<details><summary>✅ Lời giải</summary>

- `deposit` → **Command** (Ghi)
- `list_transactions` → **Query** (Đọc)
- `transfer` → **Command** (Ghi vào 2 account)

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Repository của tôi có tận 50 methods, quá lớn!" | Vi phạm SRP, nhồi cả Read lẫn Write vào một cục. | Tách đôi thành `ReaderRepo` và `WriterRepo` (Mô hình CQS). |
| "Mock Database In-Memory thỉnh thoảng hoạt động khác Database thật" | Truy vấn LIKE, JSON trong HashMap không giống DB. | Những Use Case quan trọng phải được test bằng DB Thật. Mock chỉ dùng cho Domain Logic! |
| "Code mapping giữa Domain và Persistence quá thủ công" | Đúng vậy. Sự tự do đi kèm với cái giá là Boilerplate. | Bạn có thể xài Macro, hoặc dùng ORM (như Diesel/SeaORM) làm Persistence Model để giảm bớt việc. |

---

## Tóm tắt

- ✅ **Repository = Trait**: Che giấu hoàn toàn SQL/Database khỏi Core Logic. Có thể "đóng - mở" các loại DB tùy thích.
- ✅ **CQS**: Phân tách rõ ràng giữa Việc thay đổi (Command) và Việc truy xuất (Query).
- ✅ **Transaction**: Sử dụng Unit of Work để đảm bảo tính an toàn Atomic khi chỉnh sửa hệ thống.
- ✅ **3 Models**: Bạn học được nghệ thuật chuyển đổi `Domain Model (Rich)` ↔ `Persistence Model (Flat)` ↔ `Database`.

## Tiếp theo

→ Chapter 27: **Evolving the Design** — chapter cuối cùng của Part IV! Bạn sẽ học cách nâng cấp kiến trúc mà không làm gãy vỡ hệ thống: thêm features an toàn qua cờ tính năng (Feature Flags), tận dụng Type System để ép buộc các bản Update phải tương thích ngược (Backward Compatibility).
