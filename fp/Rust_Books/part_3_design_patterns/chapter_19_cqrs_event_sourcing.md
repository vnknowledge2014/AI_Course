# Chapter 19 — CQRS & Event Sourcing

> **Bạn sẽ học được**:
> - **CQRS**: Tách Read model (Query) và Write model (Command)
> - **Event Sourcing**: Lưu events thay vì state — rebuild bằng `fold`
> - Event store, projections, snapshots
> - Tại sao Event Sourcing + FP là "match made in heaven"
> - Khi nào dùng, khi nào KHÔNG nên dùng
>
> **Yêu cầu trước**: Chapter 12 (Immutability), Chapter 14 (Enums), Chapter 18 (Command pattern).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn hiểu kiến trúc event-driven — nền tảng cho DDD ở Part IV.

---

## 19.1 — CQRS: Tách đọc và ghi

Hãy tưởng tượng bạn mở một quán cà phê. Ban đầu nhỏ, bạn dùng một cuốn sổ duy nhất: ghi đơn hàng, tính tiền, xem báo cáo doanh thu, kiểm tra tồn kho — tất cả trong cùng một cuốn. Khi quán còn 10 khách/ngày thì không sao. Nhưng khi lên 200 đơn/ngày, cuốn sổ trở thành nút thắt: nhân viên thu ngân muốn ghi đơn mới phải chờ quản lý đọc xong báo cáo, và ngược lại.

Giải pháp rất tự nhiên: **tách thành 2 cuốn sổ** — một cuốn chuyên ghi đơn hàng (tối ưu cho tốc độ ghi), một cuốn chuyên xem báo cáo (tối ưu cho tốc độ đọc). Cuốn ghi sẽ cập nhật sang cuốn xem mỗi cuối ngày. Đơn giản vậy thôi, nhưng hiệu quả bất ngờ.

CQRS — Command Query Responsibility Segregation — lấy ý tưởng đó làm nền tảng.

### Vấn đề: 1 model cho mọi thứ

Trong code, "cuốn sổ duy nhất" trông như thế này — một struct `Order` khổng lồ vừa xử lý đọc vừa xử lý ghi:

```
Truyền thống (CRUD):
┌──────────────────────┐
│       Order           │  ← Cùng 1 struct cho:
│  - create()           │     đọc (query)
│  - update()           │     ghi (command)
│  - get_summary()      │     validation
│  - get_report()       │     reporting
│  - validate()         │  → Struct phình to!
└──────────────────────┘
```

Struct này phình to vì hai nhu cầu hoàn toàn khác nhau bị nhồi chung. Khi bạn tối ưu cho đọc (thêm cache, denormalize dữ liệu) thì lại ảnh hưởng đến ghi (cache invalidation, consistency). Và ngược lại.

### Giải pháp: CQRS — Command Query Responsibility Segregation

CQRS giải quyết bằng cách tách thành hai model hoàn toàn riêng biệt — giống 2 cuốn sổ ở quán cà phê:

```
CQRS:
┌─────────────────┐         ┌─────────────────┐
│  Write Model     │         │  Read Model      │
│  (Commands)      │         │  (Queries)       │
│                  │ ──────→ │                  │
│  - PlaceOrder    │ events  │  - OrderSummary  │
│  - CancelOrder   │         │  - OrderReport   │
│  - AddItem       │         │  - DashboardView │
└─────────────────┘         └─────────────────┘
   Optimized for             Optimized for
   CONSISTENCY               PERFORMANCE
```

Bên trái — Write Model — chỉ lo việc ghi: nhận lệnh, validate business rules, đảm bảo dữ liệu nhất quán. Bên phải — Read Model — chỉ lo việc đọc: trả dữ liệu nhanh, format phù hợp cho UI hoặc báo cáo. Mỗi bên tối ưu cho nhiệm vụ riêng mà không ảnh hưởng bên kia.

Trong Rust, Write Model nhận *commands* (yêu cầu thay đổi) qua enum, còn Read Model trả *views* (dữ liệu để hiển thị) qua struct riêng:

```rust
// filename: src/main.rs

// ═══════════════════════════════════════
// COMMANDS — Write side (what to do)
// ═══════════════════════════════════════
#[derive(Debug)]
enum OrderCommand {
    Create { customer: String },
    AddItem { name: String, price: u32, quantity: u32 },
    RemoveItem { name: String },
    Submit,
    Cancel { reason: String },
}

// ═══════════════════════════════════════
// QUERIES — Read side (what to show)
// ═══════════════════════════════════════
#[derive(Debug)]
struct OrderSummary {
    customer: String,
    item_count: usize,
    total: u32,
    status: String,
}

#[derive(Debug)]
struct OrderLineItem {
    name: String,
    price: u32,
    quantity: u32,
    subtotal: u32,
}

// ═══════════════════════════════════════
// WRITE MODEL — xử lý commands
// ═══════════════════════════════════════
#[derive(Debug, Clone)]
struct Order {
    customer: String,
    items: Vec<(String, u32, u32)>,
    status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum OrderStatus { Draft, Submitted, Cancelled }

impl Order {
    fn new(customer: &str) -> Self {
        Order { customer: customer.to_string(), items: vec![], status: OrderStatus::Draft }
    }

    fn handle(self, cmd: OrderCommand) -> Result<Self, String> {
        match cmd {
            OrderCommand::Create { customer } => Ok(Order::new(&customer)),
            OrderCommand::AddItem { name, price, quantity } => {
                if self.status != OrderStatus::Draft {
                    return Err("Can only add items to draft orders".into());
                }
                let mut items = self.items;
                items.push((name, price, quantity));
                Ok(Order { items, ..self })
            }
            OrderCommand::RemoveItem { name } => {
                let items: Vec<_> = self.items.into_iter()
                    .filter(|(n, _, _)| n != &name)
                    .collect();
                Ok(Order { items, ..self })
            }
            OrderCommand::Submit => {
                if self.items.is_empty() {
                    return Err("Cannot submit empty order".into());
                }
                Ok(Order { status: OrderStatus::Submitted, ..self })
            }
            OrderCommand::Cancel { reason } => {
                println!("  Cancelled: {}", reason);
                Ok(Order { status: OrderStatus::Cancelled, ..self })
            }
        }
    }

    // READ MODEL — tạo views từ write model
    fn summary(&self) -> OrderSummary {
        OrderSummary {
            customer: self.customer.clone(),
            item_count: self.items.len(),
            total: self.items.iter().map(|(_, p, q)| p * q).sum(),
            status: format!("{:?}", self.status),
        }
    }
}

fn main() {
    let order = Order::new("Minh");

    let order = order.handle(OrderCommand::AddItem {
        name: "Coffee".into(), price: 35_000, quantity: 2
    }).unwrap();

    let order = order.handle(OrderCommand::AddItem {
        name: "Cake".into(), price: 25_000, quantity: 1
    }).unwrap();

    // Query: read model
    println!("Summary: {:?}", order.summary());

    let order = order.handle(OrderCommand::Submit).unwrap();
    println!("After submit: {:?}", order.summary());

    // Thử add item sau submit → error!
    let err = order.handle(OrderCommand::AddItem {
        name: "Tea".into(), price: 20_000, quantity: 1
    });
    println!("Add after submit: {:?}", err);
}
```

Chú ý: `handle()` nhận `self` (chứ không phải `&mut self`) và trả `Result<Self, String>` — mỗi lần xử lý command tạo ra một Order **mới** thay vì sửa Order cũ. Đây là functional update — pattern bạn đã quen từ Chapter 12. Nó giữ cho mỗi bước đều immutable, dễ test, dễ trace khi debug.

Còn `summary()` nhận `&self` — chỉ đọc, không thay đổi gì — trả về `OrderSummary` được tối ưu cho hiển thị. Hai phía hoạt động độc lập: bạn có thể thêm 10 loại report mới mà không ảnh hưởng đến logic xử lý đơn hàng.

---

## ✅ Checkpoint 19.1

> Ghi nhớ:
> 1. **CQRS** = tách Command (write) và Query (read) thành models riêng
> 2. Write model optimize cho consistency + validation
> 3. Read model optimize cho performance + UI/reporting
>
> **Test nhanh**: Tại sao tách read/write?
> <details><summary>Đáp án</summary>Mỗi bên optimize khác nhau. Write cần validation + consistency. Read cần speed + denormalized views. Cùng model → trade-offs, ai cũng thiệt.</details>

---

## 19.2 — Event Sourcing: Lưu "chuyện gì đã xảy ra"

Bạn đã biết cách tách đọc và ghi. Bây giờ câu hỏi tiếp theo: Write Model lưu dữ liệu **kiểu gì**?

Hầu hết ứng dụng dùng CRUD — tức là mỗi khi có thay đổi, bạn **ghi đè** trạng thái cũ. Ví dụ: tài khoản ngân hàng có 1 triệu đồng, bạn rút 200 nghìn → database UPDATE balance = 800,000. Xong. Trạng thái cũ (1 triệu) biến mất vĩnh viễn.

Nhưng ngân hàng thật **không hoạt động thế**. Ngân hàng giữ lại **sổ cái** (ledger) — ghi **từng giao dịch một**: gửi, rút, chuyển, nhận. Số dư hiện tại? Chỉ cộng tất cả giao dịch từ đầu lại. Muốn biết số dư ngày hôm qua? Cộng đến hôm qua. Muốn kiểm tra giao dịch bất thường? Lật sổ ra xem.

Đây chính là ý tưởng cốt lõi của Event Sourcing: thay vì lưu "ảnh chụp" hiện tại (state), lưu "cuộn phim" toàn bộ quá trình (events). Ảnh chụp chỉ cho bạn thấy bây giờ. Cuộn phim cho bạn **mọi thứ** — quá khứ, hiện tại, và khả năng nhìn lại bất kỳ thời điểm nào.

### Ý tưởng cốt lõi

Thay vì lưu **state hiện tại** (CRUD: UPDATE balance = 500), lưu **chuỗi events** (chuyện đã xảy ra):

```
CRUD:          Account { balance: 500 }  ← Chỉ biết kết quả, không biết quá trình
Event Source:  [Opened(1000), Withdrawn(300), Deposited(200), Withdrawn(400)]
               → fold → balance = 500  ← Biết CHÍNH XÁC mọi thứ đã xảy ra
```

### Events = Enum (Immutable facts)

```rust
// filename: src/main.rs
use std::time::SystemTime;

// Events = chuyện đã xảy ra (PAST TENSE!)
#[derive(Debug, Clone)]
enum AccountEvent {
    Opened { id: u64, owner: String, initial_balance: u64 },
    Deposited { amount: u64, description: String },
    Withdrawn { amount: u64, description: String },
    TransferSent { amount: u64, to_account: u64 },
    TransferReceived { amount: u64, from_account: u64 },
    Closed { reason: String },
}

// State = được TÍNH TỪ events
#[derive(Debug, Clone)]
struct AccountState {
    id: u64,
    owner: String,
    balance: u64,
    is_closed: bool,
    transaction_count: u32,
}

impl AccountState {
    fn initial() -> Self {
        AccountState {
            id: 0, owner: String::new(), balance: 0,
            is_closed: false, transaction_count: 0,
        }
    }

    // CORE: apply 1 event → new state (PURE FUNCTION!)
    fn apply(self, event: &AccountEvent) -> Self {
        match event {
            AccountEvent::Opened { id, owner, initial_balance } => AccountState {
                id: *id,
                owner: owner.clone(),
                balance: *initial_balance,
                is_closed: false,
                transaction_count: 0,
            },
            AccountEvent::Deposited { amount, .. } => AccountState {
                balance: self.balance + amount,
                transaction_count: self.transaction_count + 1,
                ..self
            },
            AccountEvent::Withdrawn { amount, .. } => AccountState {
                balance: self.balance - amount,
                transaction_count: self.transaction_count + 1,
                ..self
            },
            AccountEvent::TransferSent { amount, .. } => AccountState {
                balance: self.balance - amount,
                transaction_count: self.transaction_count + 1,
                ..self
            },
            AccountEvent::TransferReceived { amount, .. } => AccountState {
                balance: self.balance + amount,
                transaction_count: self.transaction_count + 1,
                ..self
            },
            AccountEvent::Closed { .. } => AccountState {
                is_closed: true,
                ..self
            },
        }
    }
}

// Rebuild state = fold events
fn rebuild_state(events: &[AccountEvent]) -> AccountState {
    events.iter().fold(AccountState::initial(), |state, event| state.apply(event))
}

fn main() {
    let events = vec![
        AccountEvent::Opened { id: 1, owner: "Minh".into(), initial_balance: 1_000_000 },
        AccountEvent::Deposited { amount: 500_000, description: "Salary".into() },
        AccountEvent::Withdrawn { amount: 200_000, description: "Rent".into() },
        AccountEvent::TransferSent { amount: 100_000, to_account: 2 },
        AccountEvent::Deposited { amount: 50_000, description: "Refund".into() },
    ];

    // Rebuild current state từ events
    let state = rebuild_state(&events);
    println!("State: {:?}", state);
    // balance = 1_000_000 + 500_000 - 200_000 - 100_000 + 50_000 = 1_250_000

    // Replay đến bất kỳ thời điểm nào!
    println!("\n📜 State at each point:");
    for (i, event) in events.iter().enumerate() {
        let state_at = rebuild_state(&events[..=i]);
        println!("  After event {}: balance = {}đ ({:?})", i+1, state_at.balance, event);
    }
}
```

Dòng quan trọng nhất trong toàn bộ section này là `rebuild_state`: `events.iter().fold(AccountState::initial(), |state, event| state.apply(event))`. Đó là **tất cả** Event Sourcing — lấy trạng thái ban đầu, lần lượt apply từng event, ra trạng thái hiện tại. Nếu bạn đã quen `.fold()` từ Chapter 13, bạn sẽ nhận ra ngay: Event Sourcing chẳng qua là `fold` trên chuỗi events.

Và vì `apply()` là pure function — cùng input luôn cho cùng output — bạn có thể replay events bao nhiêu lần tùy thích, luôn ra cùng kết quả. Đây là lý do Event Sourcing và FP là cặp bài trùng hoàn hảo.

> **💡 Event Sourcing + FP = perfect match**: Events = immutable data. State = fold(events). Apply = pure function. Đây chính là FP thuần túy!

---

## 19.3 — Command → Event: Validation layer

Có một điểm tinh tế mà nhiều người bỡ ngỡ khi mới học Event Sourcing: **không phải mọi yêu cầu đều thành event**.

Khi khách hàng nói "tôi muốn rút 5 triệu" — đó là *command* (yêu cầu). Ngân hàng phải kiểm tra: tài khoản có đủ tiền không? Tài khoản có bị khóa không? Nếu hợp lệ, ngân hàng mới ghi vào sổ cái "đã rút 5 triệu" — đó là *event* (sự kiện đã xảy ra). Nếu không hợp lệ, command bị từ chối, không có event nào được tạo.

Nói cách khác: commands là **yêu cầu** (có thể bị từ chối). Events là **facts** (đã xảy ra, không thể undo). Command handler chính là lớp validation ở giữa — kiểm tra business rules rồi quyết định có tạo events hay không.

```rust
// filename: src/main.rs

#[derive(Debug)]
enum BankCommand {
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
    Transfer { amount: u64, to: u64 },
}

#[derive(Debug, Clone)]
enum BankEvent {
    Deposited { amount: u64 },
    Withdrawn { amount: u64 },
    TransferSent { amount: u64, to: u64 },
}

#[derive(Debug, Clone)]
struct BankState {
    balance: u64,
    is_locked: bool,
}

// Command handler: validate → produce events
fn handle_command(state: &BankState, cmd: &BankCommand) -> Result<Vec<BankEvent>, String> {
    if state.is_locked {
        return Err("Account is locked".into());
    }

    match cmd {
        BankCommand::Deposit { amount } => {
            if *amount == 0 {
                Err("Amount must be positive".into())
            } else {
                Ok(vec![BankEvent::Deposited { amount: *amount }])
            }
        }
        BankCommand::Withdraw { amount } => {
            if *amount > state.balance {
                Err(format!("Insufficient funds: have {}đ, need {}đ", state.balance, amount))
            } else {
                Ok(vec![BankEvent::Withdrawn { amount: *amount }])
            }
        }
        BankCommand::Transfer { amount, to } => {
            if *amount > state.balance {
                Err("Insufficient funds for transfer".into())
            } else {
                // 1 command → nhiều events!
                Ok(vec![
                    BankEvent::Withdrawn { amount: *amount },
                    BankEvent::TransferSent { amount: *amount, to: *to },
                ])
            }
        }
    }
}

fn apply_event(state: BankState, event: &BankEvent) -> BankState {
    match event {
        BankEvent::Deposited { amount } =>
            BankState { balance: state.balance + amount, ..state },
        BankEvent::Withdrawn { amount } =>
            BankState { balance: state.balance - amount, ..state },
        BankEvent::TransferSent { .. } => state, // balance đã trừ ở Withdrawn
    }
}

fn main() {
    let mut state = BankState { balance: 500_000, is_locked: false };
    let mut all_events: Vec<BankEvent> = vec![];

    let commands = vec![
        BankCommand::Deposit { amount: 200_000 },
        BankCommand::Withdraw { amount: 100_000 },
        BankCommand::Transfer { amount: 50_000, to: 2 },
        BankCommand::Withdraw { amount: 999_999 },  // sẽ fail!
    ];

    for cmd in &commands {
        match handle_command(&state, cmd) {
            Ok(events) => {
                println!("✅ {:?}", cmd);
                for event in &events {
                    state = apply_event(state, event);
                    all_events.push(event.clone());
                    println!("   Event: {:?} → balance: {}đ", event, state.balance);
                }
            }
            Err(e) => println!("❌ {:?} → {}", cmd, e),
        }
    }

    println!("\n📊 Final: {:?}", state);
    println!("📜 Total events: {}", all_events.len());
}
```

Chú ý rằng `handle_command` trả `Result<Vec<BankEvent>, String>` — một command có thể tạo ra **nhiều events** (ví dụ: Transfer tạo cả `Withdrawn` lẫn `TransferSent`), hoặc **không event nào** (khi bị từ chối). Đây là sức mạnh của pattern: logic validation tập trung một chỗ, events chỉ được tạo khi mọi thứ hợp lệ.

---

## 19.4 — Projections: Read models từ Events

Bạn có chuỗi events. Bạn có thể rebuild state hiện tại. Nhưng CQRS nói rằng Read Model nên tối ưu cho từng use case. Vậy nếu bạn cần 3 loại báo cáo khác nhau — inventory, doanh thu, đánh giá sao — thì sao?

Câu trả lời: tạo **3 projections riêng biệt**, mỗi cái "xem" cùng chuỗi events nhưng rút ra thông tin khác nhau. Giống như 3 người xem cùng một trận bóng đá: bình luận viên thấy chiến thuật, cầu thủ thấy đối thủ, khán giả thấy bàn thắng. Cùng sự kiện, khác góc nhìn.

Trong code, mỗi projection là một struct với method `apply(&mut self, event)` — nó "lắng nghe" event stream và chỉ xử lý events liên quan, bỏ qua phần còn lại:

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
enum ShopEvent {
    ProductAdded { name: String, price: u32, category: String },
    ProductSold { name: String, quantity: u32, revenue: u32 },
    ReviewPosted { product: String, stars: u32 },
}

// Projection 1: Inventory view
#[derive(Debug, Default)]
struct InventoryView {
    products: std::collections::HashMap<String, u32>, // name → stock
}

impl InventoryView {
    fn apply(&mut self, event: &ShopEvent) {
        match event {
            ShopEvent::ProductAdded { name, .. } => {
                *self.products.entry(name.clone()).or_insert(0) += 1;
            }
            ShopEvent::ProductSold { name, quantity, .. } => {
                if let Some(stock) = self.products.get_mut(name) {
                    *stock = stock.saturating_sub(*quantity);
                }
            }
            _ => {} // bỏ qua events không liên quan
        }
    }
}

// Projection 2: Revenue report
#[derive(Debug, Default)]
struct RevenueReport {
    total_revenue: u64,
    sales_count: u32,
    top_products: std::collections::HashMap<String, u64>,
}

impl RevenueReport {
    fn apply(&mut self, event: &ShopEvent) {
        if let ShopEvent::ProductSold { name, revenue, .. } = event {
            self.total_revenue += *revenue as u64;
            self.sales_count += 1;
            *self.top_products.entry(name.clone()).or_insert(0) += *revenue as u64;
        }
    }
}

// Projection 3: Rating summary
#[derive(Debug, Default)]
struct RatingView {
    ratings: std::collections::HashMap<String, Vec<u32>>,
}

impl RatingView {
    fn apply(&mut self, event: &ShopEvent) {
        if let ShopEvent::ReviewPosted { product, stars } = event {
            self.ratings.entry(product.clone()).or_default().push(*stars);
        }
    }

    fn average(&self, product: &str) -> Option<f64> {
        self.ratings.get(product).map(|ratings| {
            ratings.iter().sum::<u32>() as f64 / ratings.len() as f64
        })
    }
}

fn main() {
    let events = vec![
        ShopEvent::ProductAdded { name: "Coffee".into(), price: 35_000, category: "Drinks".into() },
        ShopEvent::ProductAdded { name: "Tea".into(), price: 25_000, category: "Drinks".into() },
        ShopEvent::ProductSold { name: "Coffee".into(), quantity: 1, revenue: 35_000 },
        ShopEvent::ReviewPosted { product: "Coffee".into(), stars: 5 },
        ShopEvent::ProductSold { name: "Coffee".into(), quantity: 2, revenue: 70_000 },
        ShopEvent::ReviewPosted { product: "Coffee".into(), stars: 4 },
        ShopEvent::ProductSold { name: "Tea".into(), quantity: 1, revenue: 25_000 },
        ShopEvent::ReviewPosted { product: "Tea".into(), stars: 3 },
    ];

    // CÙNG events → NHIỀU views khác nhau!
    let mut inventory = InventoryView::default();
    let mut revenue = RevenueReport::default();
    let mut ratings = RatingView::default();

    for event in &events {
        inventory.apply(event);
        revenue.apply(event);
        ratings.apply(event);
    }

    println!("📦 Inventory: {:?}", inventory.products);
    println!("💰 Revenue: {}đ ({} sales)", revenue.total_revenue, revenue.sales_count);
    println!("   Top: {:?}", revenue.top_products);
    println!("⭐ Coffee avg: {:.1}", ratings.average("Coffee").unwrap_or(0.0));
    println!("⭐ Tea avg: {:.1}", ratings.average("Tea").unwrap_or(0.0));
}
```

Điều đặc biệt ở đây: cả 3 projections đều nhận cùng event stream, nhưng mỗi cái chỉ quan tâm đến events liên quan (xem `_ => {}` — bỏ qua events không cần). Muốn thêm projection mới — ví dụ "TopCustomerView"? Chỉ cần tạo struct mới với `.apply()`, replay lại toàn bộ events. Không cần sửa database schema, không cần migration.

> **💡 Key insight**: 1 event stream → nhiều projections → nhiều views tối ưu cho từng use case. Thêm view mới? Chỉ cần replay events!

---

## 19.5 — Snapshots: Tối ưu performance

Event Sourcing đẹp về lý thuyết, nhưng có một vấn đề thực tế: nếu tài khoản ngân hàng hoạt động 10 năm và có 500,000 giao dịch, mỗi lần muốn xem số dư, bạn phải replay **nửa triệu events** từ đầu? Rõ ràng không ổn.

Giải pháp giống cách bạn chơi game: thỉnh thoảng **save game** (snapshot). Khi cần load, bắt đầu từ save gần nhất, chỉ replay phần còn lại. Nếu save tại event 499,000, bạn chỉ cần replay 1,000 events cuối — nhanh gấp 500 lần.

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
struct Snapshot<S> {
    state: S,
    version: usize,  // event index
}

#[derive(Debug, Clone, PartialEq)]
struct Counter {
    value: i64,
    operations: u32,
}

#[derive(Debug, Clone)]
enum CounterEvent {
    Incremented(i64),
    Decremented(i64),
    Reset,
}

fn apply(state: Counter, event: &CounterEvent) -> Counter {
    match event {
        CounterEvent::Incremented(n) => Counter {
            value: state.value + n,
            operations: state.operations + 1,
        },
        CounterEvent::Decremented(n) => Counter {
            value: state.value - n,
            operations: state.operations + 1,
        },
        CounterEvent::Reset => Counter { value: 0, operations: state.operations + 1 },
    }
}

// Rebuild với optional snapshot
fn rebuild_with_snapshot(
    events: &[CounterEvent],
    snapshot: Option<&Snapshot<Counter>>,
) -> Counter {
    let (initial_state, start_index) = match snapshot {
        Some(snap) => (snap.state.clone(), snap.version),
        None => (Counter { value: 0, operations: 0 }, 0),
    };

    events[start_index..].iter().fold(initial_state, |s, e| apply(s, e))
}

fn main() {
    // 1000 events
    let events: Vec<CounterEvent> = (0..1000)
        .map(|i| if i % 3 == 0 {
            CounterEvent::Decremented(1)
        } else {
            CounterEvent::Incremented(1)
        })
        .collect();

    // Full rebuild: replay TẤT CẢ 1000 events
    let full = rebuild_with_snapshot(&events, None);
    println!("Full rebuild (1000 events): {:?}", full);

    // Snapshot tại event 900
    let snapshot = Snapshot {
        state: rebuild_with_snapshot(&events[..900], None),
        version: 900,
    };
    println!("Snapshot at 900: {:?}", snapshot.state);

    // Partial rebuild: chỉ replay 100 events cuối
    let partial = rebuild_with_snapshot(&events, Some(&snapshot));
    println!("Partial rebuild (100 events): {:?}", partial);

    // Kết quả PHẢI giống nhau!
    assert_eq!(full, partial);
    println!("✅ Full rebuild == Partial rebuild");
}
```

---

## 19.6 — Khi nào dùng Event Sourcing?

### ✅ Nên dùng khi

| Use case | Lý do |
|----------|-------|
| **Audit trail** bắt buộc | Mọi thay đổi được ghi lại, không xóa |
| **Undo/Redo** cần thiết | Replay events ngược |
| **Time travel debugging** | Rebuild state tại bất kỳ thời điểm |
| **CQRS** với multiple read models | 1 event stream → N projections |
| Domain phức tạp với invariants | Events = ngôn ngữ domain |

### ❌ Không nên dùng khi

| Use case | Lý do |
|----------|-------|
| CRUD đơn giản | Over-engineering |
| Data ít thay đổi | Events ít → không cần |
| Team chưa quen ES | Learning curve cao |
| Cần query phức tạp trên entity | Projections thêm complexity |
| Privacy (GDPR delete) | Events immutable → xóa data khó |

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Events là gì?

Cho domain "Library" (thư viện sách), liệt kê 5 events:

<details><summary>✅ Lời giải Bài 1</summary>

```rust
enum LibraryEvent {
    BookAdded { isbn: String, title: String },
    BookBorrowed { isbn: String, borrower: String },
    BookReturned { isbn: String },
    BookLost { isbn: String, borrower: String },
    MemberRegistered { name: String, member_id: u64 },
}
```

</details>

---

**Bài 2** (10 phút): Shopping cart event sourcing

Viết events cho shopping cart: `ItemAdded`, `ItemRemoved`, `QuantityChanged`, `CartCleared`. Implement `apply(state, event)` và rebuild cart state.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum CartEvent {
    ItemAdded { product: String, price: u32 },
    ItemRemoved { product: String },
    QuantityChanged { product: String, new_qty: u32 },
    CartCleared,
}

#[derive(Debug, Clone, Default)]
struct CartState {
    items: HashMap<String, (u32, u32)>, // name → (price, quantity)
}

impl CartState {
    fn total(&self) -> u32 {
        self.items.values().map(|(p, q)| p * q).sum()
    }
}

fn apply(state: CartState, event: &CartEvent) -> CartState {
    let mut items = state.items;
    match event {
        CartEvent::ItemAdded { product, price } => {
            let entry = items.entry(product.clone()).or_insert((*price, 0));
            entry.1 += 1;
        }
        CartEvent::ItemRemoved { product } => { items.remove(product); }
        CartEvent::QuantityChanged { product, new_qty } => {
            if let Some(entry) = items.get_mut(product) {
                entry.1 = *new_qty;
            }
        }
        CartEvent::CartCleared => { items.clear(); }
    }
    CartState { items }
}

fn main() {
    let events = vec![
        CartEvent::ItemAdded { product: "Coffee".into(), price: 35_000 },
        CartEvent::ItemAdded { product: "Cake".into(), price: 25_000 },
        CartEvent::ItemAdded { product: "Coffee".into(), price: 35_000 },
        CartEvent::QuantityChanged { product: "Coffee".into(), new_qty: 3 },
    ];

    let state = events.iter().fold(CartState::default(), |s, e| apply(s, e));
    println!("{:?}", state);
    println!("Total: {}đ", state.total());
    // Coffee: 3 × 35000 = 105000, Cake: 1 × 25000 = 25000
    // Total: 130000đ
}
```

</details>

---

**Bài 3** (15 phút): Full system — Task tracker

Viết event-sourced task tracker:
- Events: `TaskCreated`, `TaskStarted`, `TaskCompleted`, `TaskArchived`
- Command handler: validate transitions (e.g., can't complete an archived task)
- 2 projections: `ActiveTasksView`, `CompletedStatsView`

<details><summary>✅ Lời giải Bài 3</summary>

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum TaskEvent {
    Created { id: u64, title: String },
    Started { id: u64 },
    Completed { id: u64 },
    Archived { id: u64 },
}

#[derive(Debug, Clone, PartialEq)]
enum TaskStatus { Todo, InProgress, Done, Archived }

#[derive(Debug, Clone)]
struct TaskState {
    tasks: HashMap<u64, (String, TaskStatus)>,
}

impl TaskState {
    fn new() -> Self { TaskState { tasks: HashMap::new() } }

    fn apply(mut self, event: &TaskEvent) -> Self {
        match event {
            TaskEvent::Created { id, title } => {
                self.tasks.insert(*id, (title.clone(), TaskStatus::Todo));
            }
            TaskEvent::Started { id } => {
                if let Some(task) = self.tasks.get_mut(id) { task.1 = TaskStatus::InProgress; }
            }
            TaskEvent::Completed { id } => {
                if let Some(task) = self.tasks.get_mut(id) { task.1 = TaskStatus::Done; }
            }
            TaskEvent::Archived { id } => {
                if let Some(task) = self.tasks.get_mut(id) { task.1 = TaskStatus::Archived; }
            }
        }
        self
    }
}

// Projection 1: active tasks
fn active_tasks(state: &TaskState) -> Vec<(u64, String, String)> {
    state.tasks.iter()
        .filter(|(_, (_, status))| *status != TaskStatus::Archived)
        .map(|(id, (title, status))| (*id, title.clone(), format!("{:?}", status)))
        .collect()
}

// Projection 2: completion stats
fn completion_stats(state: &TaskState) -> (usize, usize, f64) {
    let total = state.tasks.len();
    let done = state.tasks.values().filter(|(_, s)| *s == TaskStatus::Done || *s == TaskStatus::Archived).count();
    let pct = if total > 0 { done as f64 / total as f64 * 100.0 } else { 0.0 };
    (done, total, pct)
}

fn main() {
    let events = vec![
        TaskEvent::Created { id: 1, title: "Design API".into() },
        TaskEvent::Created { id: 2, title: "Implement auth".into() },
        TaskEvent::Created { id: 3, title: "Write tests".into() },
        TaskEvent::Started { id: 1 },
        TaskEvent::Completed { id: 1 },
        TaskEvent::Started { id: 2 },
    ];

    let state = events.iter().fold(TaskState::new(), |s, e| s.apply(e));

    println!("Active tasks:");
    for (id, title, status) in active_tasks(&state) {
        println!("  #{}: {} [{}]", id, title, status);
    }

    let (done, total, pct) = completion_stats(&state);
    println!("\nStats: {}/{} done ({:.0}%)", done, total, pct);
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| Events quá nhiều → rebuild chậm | Không có snapshot | Thêm snapshots mỗi N events |
| Event schema thay đổi | Evolution over time | Event versioning / upcasters |
| Khó query across entities | Events per entity | Dùng projections denormalized |
| GDPR: cần xóa data | Events immutable | Crypto-shredding hoặc tombstone events |

---

## Tóm tắt

- ✅ **CQRS** = tách Command (write, consistency) và Query (read, performance).
- ✅ **Event Sourcing** = lưu events thay state. `state = fold(events, apply)`. Pure FP!
- ✅ **Command → Event**: validate command → produce events. Events = immutable facts.
- ✅ **Projections** = multiple read models từ 1 event stream. Thêm view = replay events.
- ✅ **Snapshots** = lưu state tại checkpoint, chỉ replay events sau đó.
- ✅ **Khi nào dùng**: audit trail, undo, time travel, multiple views. **Không**: simple CRUD, GDPR.

---

## 🎉 Kết thúc Part III!

Bạn đã học **Design Patterns qua góc nhìn FP** — GoF patterns đơn giản hơn với closures/enums/traits, và CQRS+Event Sourcing cho kiến trúc event-driven.

## Tiếp theo

→ **Part IV: Domain-Driven Design with Rust** bắt đầu! Chapter 20: **Introduction to DDD** — Ubiquitous Language, Bounded Contexts, Event Storming. Bạn sẽ áp dụng mọi thứ đã học vào thiết kế domain thực tế.
