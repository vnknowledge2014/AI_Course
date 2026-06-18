# Chapter 20 — Introduction to DDD

> **Bạn sẽ học được**:
> - **Domain-Driven Design** là gì — và tại sao nó quan trọng
> - **Ubiquitous Language** — ngôn ngữ chung giữa dev và domain experts
> - **Bounded Contexts** — ranh giới rõ ràng giữa các phần hệ thống
> - **Subdomains** — Core, Supporting, Generic
> - **Event Storming** — kỹ thuật khám phá domain
> - Tại sao Rust + FP là **công cụ tuyệt vời** cho DDD
>
> **Yêu cầu trước**: Part II (Algebraic Types, Traits) + Part III (CQRS & Event Sourcing).
> **Thời gian đọc**: ~35 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn nhìn nhận software như **model của domain** — code phản ánh business, không ngược lại.

---

## Bước vào Part IV: DDD with Rust

Bạn đã bao giờ cầm bản đồ một thành phố chưa?

Bản đồ tốt không chỉ ghi tọa độ — nó **phản ánh thực tế** mà người dân sống mỗi ngày. "Chợ Bến Thành" trên bản đồ phải là nơi mà khi bạn đến, bạn thật sự thấy chợ Bến Thành — không phải một bãi đất trống mang mã số "ZONE-0x3F". Tên đường phải là tên mà dân địa phương sử dụng. Ranh giới quận phải rõ ràng — bạn biết mình đang ở đâu.

**Domain-Driven Design là việc vẽ bản đồ cho phần mềm.** Domain (lĩnh vực kinh doanh) = thành phố. Types = tên đường và địa danh. Bounded Contexts = ranh giới quận. Ubiquitous Language = ngôn ngữ mà dân địa phương dùng, không phải ký hiệu quy hoạch đô thị.

Tới đây, bạn đã có **mọi công cụ kỹ thuật**: types, traits, generics, pattern matching, error handling, event sourcing. Nhưng công cụ mà không biết vẽ gì thì vô nghĩa — giống thợ mộc có đầy đủ đục bào mà không có bản vẽ nhà. Part IV dạy bạn **dùng chúng để model domain thực tế** — biến business rules thành code mà domain expert đọc hiểu được. Khi code nói cùng ngôn ngữ với business — developer đọc code hiểu business, Product Owner đọc types hiểu logic. Không cần "phiên dịch".

---

## 20.1 — DDD là gì?

### Khi bản đồ không khớp thực tế

Hãy tưởng tượng bản đồ thành phố dùng mã số thay vì tên đường: "đường A7" thay vì "Nguyễn Huệ", "khu vực 0x3F" thay vì "Quận 1". Không ai dùng được — kể cả người vẽ bản đồ, sau vài tháng cũng quên mã nào ứng với gì. Đó chính xác là code không dùng DDD: `data.s = 1` thay vì `order.status = "confirmed"`.

Lãnh thổ = **domain** (thế giới thực: e-commerce, ngân hàng, logistics).
Bản đồ = **software model** (code, types, functions).

DDD nói: **hãy vẽ bản đồ thật giống lãnh thổ**. Nếu domain expert nói "Order phải qua trạng thái Confirmed trước khi Ship", code phải phản ánh **đúng điều đó**. Không phải `if status == 2 { status = 3 }` — mà là `ship(confirmed_order)`. Để nhìn vào code, bạn THẤY business rule:

```rust
// Domain expert: "Order goes through Draft → Confirmed → Shipped"
// Code phản ánh 1:1
enum OrderState {
    Draft,
    Confirmed { confirmed_at: String },
    Shipped { tracking: String },
}

// Domain expert: "Chỉ Confirmed order mới ship được"
fn ship(order: OrderState, tracking: &str) -> Result<OrderState, String> {
    match order {
        OrderState::Confirmed { .. } => Ok(OrderState::Shipped {
            tracking: tracking.to_string(),
        }),
        _ => Err("Only confirmed orders can be shipped".into()),
    }
}
```

Nhìn kỹ đoạn code trên: `OrderState::Confirmed` là điều kiện duy nhất để `ship()` thành công. Bất kỳ state nào khác — Draft, đã Shipped — đều trả về `Err`. Đây không phải logic kỹ thuật — đây là **business rule** được compiler enforce. Domain expert nói "chỉ confirmed order mới ship được" — và code phản ánh 1:1 điều đó.

> **💡 DDD principle**: Code là **model** của domain, không phải ngược lại. Domain expert nhìn code phải hiểu được business rules.

### Eric Evans và cuốn sách thay đổi ngành (2003)

Năm 2003, Eric Evans xuất bản *Domain-Driven Design: Tackling Complexity in the Heart of Software*. Cuốn sách đặt ra câu hỏi then chốt: **tại sao nhiều dự án phần mềm thất bại dù tech stack hoàn hảo?** Câu trả lời của Evans: vì developer không hiểu domain. Họ viết code giải quyết vấn đề kỹ thuật, trong khi vấn đề thật nằm ở domain — business logic bị hiểu sai, bị "dịch" sai từ yêu cầu sang code.

Evans đề xuất một bộ khái niệm để "vẽ bản đồ" chuẩn:

| Concept | Ý nghĩa |
|---------|---------|
| **Domain** | Lĩnh vực business bạn đang giải quyết |
| **Model** | Abstraction có chọn lọc của domain |
| **Ubiquitous Language** | Ngôn ngữ chung dev ↔ domain expert |
| **Bounded Context** | Ranh giới rõ ràng cho model |
| **Entity** | Object có identity (ID duy nhất) |
| **Value Object** | Object không identity, so sánh bằng giá trị |
| **Aggregate** | Cluster entities với consistency boundary |
| **Domain Event** | "Chuyện đã xảy ra" trong domain |

Bạn không cần nhớ hết bảng này ngay — mỗi concept sẽ được giải thích chi tiết qua code ở các sections tiếp theo. Điều quan trọng là hiểu **tinh thần**: code phải phản ánh domain, không phải ngược lại.

---

## 20.2 — Ubiquitous Language: Ngôn ngữ chung

### Tên đường phải là tên mà dân dùng

Quay lại bản đồ: nếu dân gọi là "Nguyễn Huệ" mà bản đồ ghi "Street #47" — bản đồ vô dụng. Ubiquitous Language nói: **dùng chính xác từ ngữ mà domain expert sử dụng**. `Customer`, không phải `UserRecord`. `Order`, không phải `TransactionEntity`. `ship()`, không phải `setStatus(3)`. Khi Product Owner đọc code và nói "Đúng, đây là cách business hoạt động" — bạn có Ubiquitous Language.

Nhưng thực tế thì developer hay "dịch" ngôn ngữ business sang ngôn ngữ kỹ thuật — như trò chơi "Điện thoại hỏng" (Telephone game mà Wlaschin nhắc tới trong DMMF): qua mỗi lần "dịch", ý nghĩa bị méo mó thêm một chút.

### Vấn đề: Dev nói một đằng, Business nói một nẻo

```
Business: "Customer đặt Order, chọn Payment method, Order được Fulfill"

Dev translation (sai):
  - Customer → UserRecord
  - Order → TransactionEntity  
  - Payment → PaymentGatewayRequest
  - Fulfill → setStatus(STATUS_COMPLETE)

→ Code KHÔNG phản ánh domain. Khi discuss với business, phải "dịch" liên tục.
```

Thấy không? Business nói "Customer", dev viết `UserRecord`. Business nói "Fulfill", dev viết `setStatus(STATUS_COMPLETE)`. Mỗi lần discuss feature mới, ai cũng phải "dịch" — và dịch sai thì code sai, code sai thì bug.

### Giải pháp: Dùng ĐÚNG ngôn ngữ domain trong code

Giải pháp đơn giản đến bất ngờ: **bỏ việc dịch**. Dùng ĐÚNG từ business trong code. Khi business nói "Customer đặt Order" → code có struct `Customer` và function `place_order()`. Không cần dịch, không bị méo mó:

```rust
// filename: src/main.rs

// ✅ Code NÓI CÙNG NGÔN NGỮ với domain expert
// Domain expert đọc code này HIỂU ĐƯỢC!

mod ordering {
    #[derive(Debug, Clone)]
    pub struct Customer {
        pub name: String,
        pub email: String,
    }

    #[derive(Debug, Clone)]
    pub struct OrderLine {
        pub product_name: String,
        pub quantity: u32,
        pub unit_price: u32,
    }

    #[derive(Debug)]
    pub enum PaymentMethod {
        CashOnDelivery,
        BankTransfer { account: String },
        CreditCard { last_four: String },
    }

    #[derive(Debug)]
    pub struct Order {
        pub customer: Customer,
        pub lines: Vec<OrderLine>,
        pub payment: PaymentMethod,
    }

    impl Order {
        // "Place an order" — domain language
        pub fn place(customer: Customer, lines: Vec<OrderLine>, payment: PaymentMethod) -> Result<Self, String> {
            if lines.is_empty() {
                return Err("Order must have at least one line item".into());
            }
            Ok(Order { customer, lines, payment })
        }

        // "Calculate order total" — domain language
        pub fn total(&self) -> u32 {
            self.lines.iter().map(|l| l.unit_price * l.quantity).sum()
        }
    }
}

fn main() {
    use ordering::*;

    let customer = Customer { name: "Minh".into(), email: "minh@co.com".into() };
    let lines = vec![
        OrderLine { product_name: "Coffee".into(), quantity: 2, unit_price: 35_000 },
        OrderLine { product_name: "Cake".into(), quantity: 1, unit_price: 25_000 },
    ];

    match Order::place(customer, lines, PaymentMethod::CashOnDelivery) {
        Ok(order) => {
            println!("Order placed for {}", order.customer.name);
            println!("Total: {}đ", order.total());
            println!("Payment: {:?}", order.payment);
        }
        Err(e) => println!("Failed: {}", e),
    }
}
```

Đọc lại đoạn code trên: bạn có cần comment giải thích `Order::place()` làm gì không? Không — tên tự giải thích. Domain expert đọc `PaymentMethod::CashOnDelivery` và hiểu ngay, không cần developer "dịch". Đó là sức mạnh của Ubiquitous Language — code **trở thành** tài liệu.

### Quy tắc Ubiquitous Language

1. **Dùng tên domain trong code**: `Customer`, `Order`, `PaymentMethod` — không `UserRecord`, `TransactionEntity`
2. **Method names = domain actions**: `place_order()`, `ship()`, `cancel()` — không `setStatus()`, `updateField()`
3. **Nếu domain expert không hiểu tên function → tên sai**
4. **Glossary**: Duy trì danh sách thuật ngữ chung, ai cũng đồng ý

> **💡 Litmus test**: Đưa code cho Product Owner đọc. Nếu họ nói "Ờ, đúng rồi, đây là cách business hoạt động" — bạn đang làm đúng. Nếu họ nhíu mày hỏi "setStatus(3) nghĩa là gì?" — bạn đang viết tech code, không phải domain code.

---

## ✅ Checkpoint 20.2

> Ghi nhớ:
> 1. **Ubiquitous Language** = dev và business dùng CÙNG từ vựng
> 2. Code phải phản ánh ngôn ngữ domain, không phải kỹ thuật
> 3. Nếu domain expert đọc code không hiểu → refactor tên
>
> **Test nhanh**: Function `setStatus(3)` có tuân theo Ubiquitous Language không?
> <details><summary>Đáp án</summary>Không! "3" nghĩa gì? Phải là <code>ship()</code>, <code>confirm()</code>, <code>cancel()</code> — domain actions rõ ràng.</details>

---

## 20.3 — Bounded Contexts: Ranh giới rõ ràng

### Cùng tên đường, khác quận

Trong thành phố Hồ Chí Minh, "đường Lê Lợi" ở Quận 1 khác hoàn toàn "đường Lê Lợi" ở Gò Vấp. Bạn cần **ranh giới quận** để biết đang nói đường nào. Tương tự, trong phần mềm, từ "Product" nghĩa khác nhau tùy context:

```
Catalog context:  Product { name, description, images, category }
Inventory context: Product { sku, location, quantity_in_stock }
Shipping context: Product { weight_kg, dimensions, is_fragile }
Pricing context:  Product { base_price, discount_rules, tax_rate }
```

Nếu bạn cố nhét MỌI thông tin vào MỘT struct `Product` — struct phình to, ai sửa cũng ảnh hưởng mọi người. Team Catalog thêm field `images` → team Shipping phải recompile dù không quan tâm đến ảnh. Team Pricing sửa `discount_rules` → team Inventory bị break test dù chẳng liên quan.

Bounded Context giải quyết vấn đề này: **mỗi "quận" có "đường" riêng, phù hợp với công việc riêng**. Giao tiếp giữa contexts qua Anti-Corruption Layer — như trạm biên giới dịch ngôn ngữ quận này sang quận khác.

### Giải pháp: Mỗi context có model riêng

```rust
// filename: src/main.rs

// Mỗi bounded context = 1 module với types RIÊNG
mod catalog {
    #[derive(Debug, Clone)]
    pub struct Product {
        pub id: String,
        pub name: String,
        pub description: String,
        pub category: String,
    }
}

mod inventory {
    #[derive(Debug, Clone)]
    pub struct StockItem {
        pub product_id: String,
        pub warehouse: String,
        pub quantity: u32,
    }

    impl StockItem {
        pub fn is_available(&self) -> bool { self.quantity > 0 }
    }
}

mod shipping {
    #[derive(Debug, Clone)]
    pub struct Parcel {
        pub product_id: String,
        pub weight_kg: f64,
        pub is_fragile: bool,
    }

    impl Parcel {
        pub fn shipping_cost(&self) -> u32 {
            let base = (self.weight_kg * 30_000.0) as u32;
            if self.is_fragile { base * 150 / 100 } else { base }
        }
    }
}

mod pricing {
    #[derive(Debug, Clone)]
    pub struct PricedItem {
        pub product_id: String,
        pub base_price: u32,
        pub tax_rate: f64,
    }

    impl PricedItem {
        pub fn final_price(&self) -> u32 {
            self.base_price + (self.base_price as f64 * self.tax_rate) as u32
        }
    }
}

fn main() {
    // Mỗi context chỉ biết thông tin CẦN THIẾT
    let product = catalog::Product {
        id: "PROD-001".into(),
        name: "Premium Coffee".into(),
        description: "Single origin, dark roast".into(),
        category: "Beverages".into(),
    };

    let stock = inventory::StockItem {
        product_id: "PROD-001".into(),
        warehouse: "HCM-01".into(),
        quantity: 150,
    };

    let parcel = shipping::Parcel {
        product_id: "PROD-001".into(),
        weight_kg: 0.5,
        is_fragile: false,
    };

    let priced = pricing::PricedItem {
        product_id: "PROD-001".into(),
        base_price: 200_000,
        tax_rate: 0.08,
    };

    println!("📦 {} — {}", product.name, product.category);
    println!("📊 Stock: {} (available: {})", stock.quantity, stock.is_available());
    println!("🚚 Shipping: {}đ", parcel.shipping_cost());
    println!("💰 Price: {}đ (incl. tax)", priced.final_price());
}
```

Chú ý: cùng `product_id = "PROD-001"` nhưng mỗi module chỉ biết thông tin **cần thiết cho công việc của nó**. Catalog biết tên và mô tả (để hiển thị). Shipping biết cân nặng và fragile (để tính phí). Pricing biết giá gốc và thuế. Không ai "ôm" hết tất cả — giống như bưu điện không cần biết giá bán sản phẩm, họ chỉ cần biết cân nặng.

### Context Map — Cách contexts giao tiếp

Nhưng các quận không hoàn toàn cô lập — chúng cần giao tiếp. Khi khách đặt hàng qua Ordering, Shipping cần biết để giao. Khi Inventory hết hàng, Catalog cần cập nhật. Câu hỏi là: giao tiếp **bằng cách nào** mà không tạo tight coupling?

```
┌──────────┐    events    ┌──────────┐
│ Ordering  │────────────→│ Shipping  │
└──────────┘              └──────────┘
     │
     │ events
     ▼
┌──────────┐    query     ┌──────────┐
│ Inventory │←────────────│ Catalog   │
└──────────┘              └──────────┘
```

Contexts giao tiếp qua:
- **Events** (loose coupling) — preferred in DDD: Ordering phát event "OrderPlaced", Shipping lắng nghe và tự xử lý
- **Shared types** (tight coupling) — avoid if possible: cả hai contexts cùng import struct → sửa một bên phá hai bên
- **Anti-corruption layer** (adapter) — khi integrate external systems: dịch từ model bên ngoài sang domain model của bạn

---

## 20.4 — Subdomains: Core, Supporting, Generic

### Không phải mọi khu phố đều được đầu tư như nhau

Trong thành phố, có khu trung tâm thương mại (đầu tư cao, đông khách, lợi nhuận lớn), có khu dân cư (cần thiết nhưng không flashy), và có hạ tầng chung (đường xá, điện nước — mọi thành phố đều cần, không ai tự xây nhà máy điện riêng).

DDD áp dụng tư duy tương tự cho phần mềm. Không phải mọi phần code đều xứng đáng cùng mức đầu tư. Câu hỏi then chốt để phân loại: **"Cái gì làm bạn KHÁC đối thủ?"** — đó là Core domain.

| Loại | Mô tả | Ví dụ (E-commerce) | Invest? |
|------|-------|---------------------|---------|
| **Core** | Lợi thế cạnh tranh. **Unique** | Pricing engine, Recommendation | Đầu tư nhiều nhất, custom code |
| **Supporting** | Cần thiết nhưng không unique | Order management, Inventory | Code tốt, có thể đơn giản |
| **Generic** | Mọi business đều cần | Auth, Email, Payment gateway | Buy/use SaaS, đừng build |

Tại sao phân loại quan trọng? Vì thời gian có hạn. Nếu bạn dành 3 tháng tự build auth system thay vì dùng Auth0 — đó là 3 tháng bạn KHÔNG dành cho pricing engine (core domain). Kết quả: đối thủ đã ra sản phẩm, bạn vẫn đang debug "reset password".

```rust
// filename: src/main.rs

// Core domain — ĐẦU TƯ nhiều nhất! Code cẩn thận, types chặt.
mod pricing {
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct PricingRule {
        pub name: String,
        pub condition: PriceCondition,
        pub discount: Discount,
    }

    #[derive(Debug, Clone)]
    pub enum PriceCondition {
        MinQuantity(u32),
        CustomerTier(String),
        TimeRange { from: String, to: String },
        Combined(Vec<PriceCondition>),
    }

    #[derive(Debug, Clone)]
    pub enum Discount {
        Percentage(f64),
        FixedAmount(u32),
        BuyXGetY { buy: u32, free: u32 },
    }

    pub fn apply_best_rule(base_price: u32, quantity: u32, rules: &[PricingRule]) -> u32 {
        // Business logic phức tạp — core domain!
        let mut best_price = base_price * quantity;

        for rule in rules {
            let discounted = match &rule.discount {
                Discount::Percentage(pct) => {
                    let total = base_price * quantity;
                    total - (total as f64 * pct / 100.0) as u32
                }
                Discount::FixedAmount(amt) => {
                    (base_price - amt) * quantity
                }
                Discount::BuyXGetY { buy, free } => {
                    let sets = quantity / (buy + free);
                    let remainder = quantity % (buy + free);
                    let paid = sets * buy + remainder.min(*buy);
                    base_price * paid
                }
            };

            if discounted < best_price {
                best_price = discounted;
            }
        }

        best_price
    }
}

// Supporting domain — cần, nhưng logic đơn giản hơn
mod inventory {
    pub fn check_stock(product_id: &str) -> bool {
        // Simplified — trong thực tế query DB
        product_id.starts_with("PROD")
    }
}

// Generic domain — dùng thư viện/SaaS, không build
// mod auth { use jsonwebtoken; }
// mod email { use lettre; }

fn main() {
    use pricing::*;

    let rules = vec![
        PricingRule {
            name: "VIP 20%".into(),
            condition: PriceCondition::CustomerTier("VIP".into()),
            discount: Discount::Percentage(20.0),
        },
        PricingRule {
            name: "Buy 3 Get 1 Free".into(),
            condition: PriceCondition::MinQuantity(4),
            discount: Discount::BuyXGetY { buy: 3, free: 1 },
        },
    ];

    let price = apply_best_rule(100_000, 4, &rules);
    println!("Best price for 4 items: {}đ", price);
    // Buy3Get1Free: pay 3 × 100k = 300k (vs 4 × 80k = 320k)
    // → Best: 300000đ
}
```

---

## 20.5 — Event Storming: Khám phá Domain

### Vẽ bản đồ cùng dân địa phương

Bạn không thể vẽ bản đồ chính xác nếu ngồi trong văn phòng đoán. Bạn phải ra thực địa, nói chuyện với dân địa phương: "Con đường này dẫn đến đâu?", "Khu này có gì đặc biệt?". Event Storming (do Alberto Brandolini sáng tạo) chính là "khảo sát thực địa" cho phần mềm — thay vì developer tự vẽ bản đồ (và vẽ sai), team (dev + PO + domain experts) **cùng** khám phá domain.

Hãy tưởng tượng workshop diễn ra thế này:

**You**: "Ai bắt đầu trước? Hãy kể một sự kiện xảy ra trong công việc hàng ngày."

**Ollie** (bộ phận đặt hàng): "Chúng tôi nhận đơn hàng từ khách. Khi đơn đến, chúng tôi xác nhận và chuyển cho kho."

**You**: "Vậy sự kiện đầu tiên là 'Order Received'?"

**Ollie**: "Chính xác. Và sau đó là 'Order Confirmed' khi chúng tôi kiểm tra xong."

**Sam** (bộ phận giao hàng): "Chúng tôi chỉ bắt đầu khi nhận được order đã confirmed. Sự kiện của chúng tôi là 'Order Shipped'."

Bạn thấy không? Mỗi câu hỏi dẫn đến một insight mới. Mỗi sticky note trên tường là một mảnh ghép của domain. Và kết quả — sticky notes — sẽ trở thành types trong code.

```
🟧 Domain Event     "Order Placed"
🟦 Command          "Place Order"
🟨 Actor            "Customer"
🟪 Policy           "When Order Placed → Reserve Inventory"
🟩 Read Model       "Order Summary"
🔴 Hotspot          "Pricing logic unclear!"
```

### Từ sticky notes đến Rust types

Sau workshop, bạn nhìn tường đầy sticky notes và nghĩ: "Làm sao biến đống này thành code?" Câu trả lời đơn giản đến bất ngờ: mỗi màu sticky note map vào một loại type. Events (cam) = enum variants. Commands (xanh) = enum variants. Actors (vàng) = enum variants. Policies (tím) = functions. Bản đồ trên tường trở thành bản đồ trong code — và bây giờ compiler kiểm tra bản đồ cho bạn.

```rust
// filename: src/main.rs

// Event Storming output → code trực tiếp!
// Sticky notes → types

// 🟧 Domain Events (orange)
#[derive(Debug, Clone)]
enum OrderEvent {
    OrderPlaced { order_id: u64, customer: String, items: Vec<String> },
    OrderConfirmed { order_id: u64 },
    OrderShipped { order_id: u64, tracking: String },
    OrderDelivered { order_id: u64 },
    OrderCancelled { order_id: u64, reason: String },
}

// 🟦 Commands (blue)
#[derive(Debug)]
enum OrderCommand {
    PlaceOrder { customer: String, items: Vec<String> },
    ConfirmOrder { order_id: u64 },
    ShipOrder { order_id: u64, tracking: String },
    CancelOrder { order_id: u64, reason: String },
}

// 🟨 Actor (yellow) — ai trigger command
#[derive(Debug)]
enum Actor {
    Customer(String),
    Staff(String),
    System,
}

// 🟪 Policy (purple) — side-effects từ events
fn apply_policies(event: &OrderEvent) -> Vec<String> {
    match event {
        OrderEvent::OrderPlaced { items, .. } => {
            vec![
                format!("📦 Reserve inventory for {} items", items.len()),
                "📧 Send order confirmation email".into(),
                "📊 Update sales dashboard".into(),
            ]
        }
        OrderEvent::OrderShipped { tracking, .. } => {
            vec![
                format!("📧 Send shipping notification (tracking: {})", tracking),
            ]
        }
        OrderEvent::OrderCancelled { order_id, .. } => {
            vec![
                format!("📦 Release inventory for order #{}", order_id),
                "💰 Process refund".into(),
            ]
        }
        _ => vec![],
    }
}

fn main() {
    // Simulate Event Storming flow
    let events = vec![
        OrderEvent::OrderPlaced {
            order_id: 1,
            customer: "Minh".into(),
            items: vec!["Coffee".into(), "Cake".into()],
        },
        OrderEvent::OrderConfirmed { order_id: 1 },
        OrderEvent::OrderShipped { order_id: 1, tracking: "VN123".into() },
    ];

    for event in &events {
        println!("🟧 Event: {:?}", event);
        let policies = apply_policies(event);
        for p in &policies {
            println!("   🟪 Policy: {}", p);
        }
        println!();
    }
}
```

---

## 20.6 — Tại sao Rust + FP tuyệt vời cho DDD

Nếu bạn đã quen với DDD trong thế giới OOP (Java, C#), bạn có thể thắc mắc: "DDD thường dùng classes, inheritance, mutable state — Rust không có mấy thứ đó. Liệu Rust có phù hợp?"

Câu trả lời: Rust không chỉ phù hợp — nó **tốt hơn** cho DDD. Lý do nằm ở chỗ DDD bản chất là về **modeling chính xác** domain, và Rust + FP cho bạn công cụ modeling chính xác hơn OOP. Hãy nhìn bảng mapping:

| DDD Concept | Rust Feature | Lợi ích |
|---|---|---|
| Value Objects | Newtypes `struct Email(String)` | Immutable by default, validated |
| Entities | Struct + ID field | Ownership clear |
| Domain Events | Enum variants | Exhaustive matching |
| Aggregates | Module boundaries + `pub` | Compiler enforce encapsulation |
| State machines | Enum states + transitions | Make illegal states unrepresentable |
| Workflows | Function composition, pipe | Type-safe pipelines |
| Ubiquitous Language | Rust naming conventions | Types = domain vocabulary |
| Bounded Contexts | Module system | `pub(crate)`, `pub(super)` boundaries |

Hãy chú ý cột "Lợi ích": hầu hết đều có từ **compiler**. Đây là điều OOP DDD không có — trong Java, bạn có thể tạo invalid state và chỉ phát hiện khi runtime crash. Trong Rust, compiler **từ chối compile** nếu bạn cố ship một order chưa confirmed, hoặc truyền `Username` vào chỗ cần `Email`. Domain rules nằm trong type system — không phải trong comments hay unit tests.

Đó là why "Make illegal states unrepresentable" — mantra từ Yaron Minsky (Jane Street) và Scott Wlaschin — trở thành hiện thực hoàn toàn tự nhiên trong Rust.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Ubiquitous Language

Refactor tên sau cho đúng domain language (e-commerce):

```rust
fn set_flag(r: &mut Record, f: i32) { r.status = f; }
struct Record { id: i64, status: i32, data: String }
```

<details><summary>✅ Lời giải Bài 1</summary>

```rust
fn confirm_order(order: Order) -> Order { ... }
fn ship_order(order: Order, tracking: &str) -> Order { ... }

enum OrderStatus { Draft, Confirmed, Shipped, Delivered, Cancelled }
struct Order { id: OrderId, status: OrderStatus, customer: Customer }
```

</details>

---

**Bài 2** (10 phút): Bounded Contexts

Thiết kế 3 bounded contexts cho hệ thống "Restaurant": `MenuContext`, `KitchenContext`, `BillingContext`. Mỗi context có struct riêng cho "Dish" (món ăn — khác nhau ở mỗi context).

<details><summary>✅ Lời giải Bài 2</summary>

```rust
mod menu {
    pub struct Dish {
        pub id: String,
        pub name: String,
        pub description: String,
        pub price: u32,
        pub category: String,
        pub is_available: bool,
    }
}

mod kitchen {
    pub struct KitchenOrder {
        pub dish_id: String,
        pub dish_name: String,
        pub special_instructions: String,
        pub prep_time_minutes: u32,
        pub station: String,      // grill, fryer, salad...
    }
}

mod billing {
    pub struct BillItem {
        pub dish_id: String,
        pub name: String,
        pub quantity: u32,
        pub unit_price: u32,
        pub tax_rate: f64,
    }

    impl BillItem {
        pub fn subtotal(&self) -> u32 { self.unit_price * self.quantity }
        pub fn tax(&self) -> u32 { (self.subtotal() as f64 * self.tax_rate) as u32 }
    }
}
```

</details>

---

**Bài 3** (15 phút): Event Storming → Code

Chủ đề: "Online Bookstore". Thực hiện Event Storming:
1. Liệt kê 5+ domain events
2. 3+ commands
3. 2+ policies (event → side-effect)
4. Viết code Rust cho events + commands + policy handler

<details><summary>✅ Lời giải Bài 3</summary>

```rust
#[derive(Debug, Clone)]
enum BookstoreEvent {
    BookListed { isbn: String, title: String, price: u32 },
    BookPurchased { isbn: String, buyer: String, quantity: u32 },
    BookReviewed { isbn: String, reviewer: String, stars: u32 },
    StockDepleted { isbn: String },
    RefundIssued { order_id: u64, amount: u32 },
}

#[derive(Debug)]
enum BookstoreCommand {
    ListBook { isbn: String, title: String, price: u32 },
    PurchaseBook { isbn: String, buyer: String, quantity: u32 },
    ReviewBook { isbn: String, reviewer: String, stars: u32 },
}

fn apply_policies(event: &BookstoreEvent) -> Vec<String> {
    match event {
        BookstoreEvent::BookPurchased { isbn, buyer, .. } => vec![
            format!("📧 Send purchase confirmation to {}", buyer),
            format!("📦 Update inventory for {}", isbn),
            "📊 Update sales analytics".into(),
        ],
        BookstoreEvent::StockDepleted { isbn } => vec![
            format!("⚠️ Notify supplier to restock {}", isbn),
            format!("🔴 Mark {} as 'Out of Stock' on website", isbn),
        ],
        BookstoreEvent::BookReviewed { isbn, stars, .. } => vec![
            format!("⭐ Update average rating for {}", isbn),
            if *stars <= 2 { format!("🔔 Alert team: low review for {}", isbn) } else { String::new() },
        ].into_iter().filter(|s| !s.is_empty()).collect(),
        _ => vec![],
    }
}

fn main() {
    let events = vec![
        BookstoreEvent::BookPurchased {
            isbn: "978-0-13-468599-1".into(),
            buyer: "Minh".into(), quantity: 1,
        },
        BookstoreEvent::StockDepleted { isbn: "978-0-13-468599-1".into() },
        BookstoreEvent::BookReviewed {
            isbn: "978-0-13-468599-1".into(),
            reviewer: "Lan".into(), stars: 5,
        },
    ];

    for event in &events {
        println!("🟧 {:?}", event);
        for p in apply_policies(event) {
            println!("   🟪 {}", p);
        }
        println!();
    }
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Domain expert không hiểu code" | Tên kỹ thuật thay tên domain | Refactor: dùng Ubiquitous Language |
| "1 struct Product quá lớn" | Thiếu Bounded Contexts | Tách thành contexts riêng, mỗi cái có Product riêng |
| "Contexts share quá nhiều code" | Tight coupling | Giao tiếp qua events, dùng anti-corruption layer |
| "Không biết đâu là Core domain" | Chưa phân tích subdomains | Hỏi business: "Cái gì làm bạn KHÁC đối thủ?" = Core |

---

## Tóm tắt

Chapter này vẽ bức tranh toàn cảnh của DDD — bản đồ tổng thể trước khi đi vào từng con đường:

- ✅ **DDD** = software model phản ánh domain. Code = bản đồ, domain = lãnh thổ. Vẽ sai = đi lạc.
- ✅ **Ubiquitous Language** = dev + business dùng cùng từ vựng. `Customer`, không phải `UserRecord`. Nếu PO đọc code không hiểu → refactor tên.
- ✅ **Bounded Contexts** = ranh giới quận. "Product" ở Catalog ≠ Shipping. Mỗi context có types riêng, giao tiếp qua events hoặc ACL.
- ✅ **Subdomains**: Core (đầu tư nhiều, lợi thế cạnh tranh), Supporting (code tốt), Generic (buy SaaS, đừng build).
- ✅ **Event Storming** = khảo sát thực địa cùng domain experts. Sticky notes → types. Bản đồ trên tường → bản đồ trong code.
- ✅ **Rust + FP**: Compiler enforce domain rules. Make illegal states unrepresentable. Types = domain vocabulary.

## Tiếp theo

Bạn đã có **bản đồ tổng thể** — giờ cần biết **cấu trúc mỗi tòa nhà** trong thành phố.

→ Chapter 21: **Functional Architecture** — Onion Architecture: IO ở rìa, pure domain logic ở core. Hãy nghĩ như quả trứng: lòng đỏ (pure domain) được bảo vệ bởi lòng trắng (IO shell). Rust module system enforce boundaries. Bạn sẽ **cấu trúc toàn bộ project** theo DDD.
