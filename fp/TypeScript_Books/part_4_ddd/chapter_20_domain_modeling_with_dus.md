# Chapter 20 — Domain Modeling with Discriminated Unions

> **Bạn sẽ học được**:
> - Value Objects = branded types + smart constructors (revisited từ Ch13–14)
> - State machines = discriminated unions — compiler enforces valid transitions
> - Making illegal states unrepresentable
> - Complex domain hierarchies — nested DUs cho real-world models
> - Aggregate roots — DU boundary enforcement
>
> **Yêu cầu trước**: Chapter 13 (ADTs, branded types), Chapter 14 (smart constructors), Chapter 18 (DDD, entities vs VOs).
> **Thời gian đọc**: ~50 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Model any domain chính xác — compiler bắt business rule violations.

---

Bạn đã thấy thợ đúc kim loại làm việc chưa?

Trước khi đổ đồng nóng chảy, họ tạo **khuôn đúc** — hình dạng chính xác, đúng kích thước, không có lỗ nào thừa, không có rãnh nào thiếu. Kim loại đổ vào khuôn chỉ có thể tạo ra hình ĐÚNG — "con tượng Phật" chứ không phải "cục đồng méo". Khuôn BẢO ĐẢM kết quả hợp lệ bằng cấu trúc vật lý, không phải bằng lời nhắc.

**Domain Modeling with DUs** = tạo khuôn đúc cho data. Branded types (Ch13) là khuôn đúc precision — `Email`, `VND`, `Percentage` chỉ chấp nhận giá trị hợp lệ. State machines (Ch16–17) là **bản thiết kế** (blueprint) — bày rõ mọi trạng thái hợp lệ và mọi chuyển đổi được phép. Kết hợp cả hai: data chỉ có thể tồn tại trong hình dạng hợp lệ, transitions chỉ có thể đi theo đường hợp lệ. Compiler thay thợ kiểm tra — nếu khuôn đúng, thành phẩm luôn đúng.

---

## 20.1 — Value Objects: Branded Types Revisited

### Khuôn đúc precision cho từng loại dữ liệu

Ch13 giới thiệu branded types, Ch14 giới thiệu smart constructors. Giờ kết hợp cả hai cho DDD: mỗi Value Object = khuôn đúc riêng. `Email` chỉ nhận string hợp lệ (normalize + validate). `VND` chỉ nhận số không âm. `Percentage` = [0, 100]. Hàm nhận `VND` không bao giờ nhận nhầm `Percentage` — vì khuôn khác nhau, kim loại khác nhau.

Operations trên Value Objects trả về Value Objects: `addVND(a, b)` → `VND`, `applyDiscount(price, discount)` → `VND`. Chain đảm bảo type safety xuyên suốt — bạn không bao giờ "lọt khuôn" giữa chừng.

```typescript
// filename: src/value_objects.ts
import assert from "node:assert/strict";

// --- Branded types = type-safe Value Objects ---

// Email — validated, immutable, comparable by value
type Email = string & { readonly __brand: "Email" };

const Email = (raw: string): Email | null => {
    const trimmed = raw.trim().toLowerCase();
    return /^[^@]+@[^@]+\.[^@]+$/.test(trimmed) ? trimmed as Email : null;
};

// Money — safe arithmetic
type VND = number & { readonly __brand: "VND" };

const VND = (amount: number): VND | null =>
    Number.isFinite(amount) && amount >= 0 ? amount as VND : null;

const addVND = (a: VND, b: VND): VND => (a + b) as VND;
const subtractVND = (a: VND, b: VND): VND | null => {
    const result = a - b;
    return result >= 0 ? result as VND : null;
};

// Percentage — [0, 100]
type Percentage = number & { readonly __brand: "Percentage" };

const Percentage = (value: number): Percentage | null =>
    value >= 0 && value <= 100 ? value as Percentage : null;

const applyDiscount = (price: VND, discount: Percentage): VND =>
    Math.round(price * (1 - discount / 100)) as VND;

// --- Test ---
const email = Email("  AN@Mail.Com  ");
assert.strictEqual(email, "an@mail.com");  // normalized!

const badEmail = Email("not-an-email");
assert.strictEqual(badEmail, null);

const price = VND(500000);
const discount = Percentage(10);
assert.ok(price !== null && discount !== null);
assert.strictEqual(applyDiscount(price!, discount!), 450000);

const zero = VND(0);
assert.ok(zero !== null);  // 0 is valid money

const negative = VND(-100);
assert.strictEqual(negative, null);  // negative money = invalid

console.log("Value Objects OK ✅");
```

### Composite Value Objects

Một số Value Objects chứa nhiều trường — nhưng vẫn không có identity. Hai `Address` cùng tỉnh/quận/phường/đường = BẰNG NHAU. Hai `DateRange` cùng start/end = BẰNG NHAU. So sánh bằng giá trị, tạo mới khi cần thay đổi (immutable).

```typescript
// filename: src/composite_vo.ts
import assert from "node:assert/strict";

// Address — composite Value Object (no ID, compare by value)
type Province = string & { readonly __brand: "Province" };
type District = string & { readonly __brand: "District" };
type Ward = string & { readonly __brand: "Ward" };

type Address = {
    readonly province: Province;
    readonly district: District;
    readonly ward: Ward;
    readonly street: string;
};

// Value Object equality — deep compare
const sameAddress = (a: Address, b: Address): boolean =>
    a.province === b.province &&
    a.district === b.district &&
    a.ward === b.ward &&
    a.street === b.street;

// DateRange — another composite VO
type DateRange = {
    readonly start: Date;
    readonly end: Date;
};

const DateRange = (start: Date, end: Date): DateRange | null =>
    start <= end ? { start, end } : null;

const overlaps = (a: DateRange, b: DateRange): boolean =>
    a.start <= b.end && b.start <= a.end;

const durationDays = (range: DateRange): number =>
    Math.ceil((range.end.getTime() - range.start.getTime()) / (1000 * 60 * 60 * 24));

// Test
const range1 = DateRange(new Date("2024-01-01"), new Date("2024-01-10"));
const range2 = DateRange(new Date("2024-01-05"), new Date("2024-01-15"));
const range3 = DateRange(new Date("2024-02-01"), new Date("2024-02-10"));

assert.ok(range1 !== null && range2 !== null && range3 !== null);
assert.strictEqual(overlaps(range1!, range2!), true);
assert.strictEqual(overlaps(range1!, range3!), false);
assert.strictEqual(durationDays(range1!), 9);

// Invalid: end before start
assert.strictEqual(DateRange(new Date("2024-02-01"), new Date("2024-01-01")), null);

console.log("Composite VO OK ✅");
```

> **💡 Value Objects = "Parse, don't validate" (Ch14)**: Smart constructors guarantee valid data AT CREATION TIME. Downstream code receives branded type → knows it's valid. No defensive checks needed.

---

## ✅ Checkpoint 20.1

> Đến đây bạn phải hiểu:
> 1. **Value Objects** = branded types + smart constructors. Valid BY CONSTRUCTION
> 2. **Composite VOs**: Address, DateRange — multiple fields, no ID
> 3. **Operations return branded types**: `addVND → VND`, `applyDiscount → VND`
> 4. **Equality by value**: `sameAddress(a, b)` compares all fields
>
> **Test nhanh**: `const x: VND = 500000;` — đúng hay sai?
> <details><summary>Đáp án</summary>**SAI!** `number` ≠ `VND`. Phải qua smart constructor: `VND(500000)`. Branded type ngăn raw number gán vào VND.</details>

---

## 20.2 — State Machines via Discriminated Unions

### Bản thiết kế chỉ cho phép lắp ráp đúng cách

Tưởng tượng bạn lắp ráp tên lửa theo bản thiết kế: giai đoạn 1 (draft) → kiểm tra nhiên liệu (pending_payment) → nạp nhiên liệu (paid) → phóng (shipped) → đến quỹ đạo (delivered). Bạn KHÔNG THỂ "phóng" trước khi "nạp nhiên liệu" — bản thiết kế VẬT LÝ ngăn bạn. DU state machine là bản thiết kế đó cho data.

Boolean flags = thảm họa: 4 boolean = 2⁴ = 16 tổ hợp, chỉ 5 hợp lệ. 11 trạng thái bất hợp pháp ẩn nấp trong code, CHỜ gây bug. DU = MỖI variant là MỘT trạng thái hợp lệ, với CHÍNH XÁC data cần thiết. Transition functions nhận state cụ thể: `shipOrder(paid)` — chỉ chấp nhận `OrderState & { tag: "paid" }`. Compiler enforces the blueprint.

```typescript
// filename: src/state_machine_problem.ts

// ❌ Boolean flags = illegal states ARE representable
type OrderBad = {
    isPaid: boolean;
    isShipped: boolean;
    isDelivered: boolean;
    isCancelled: boolean;
    trackingNumber?: string;
    deliveredAt?: Date;
    cancelReason?: string;
};

// Illegal: shipped BUT not paid → 💥 business error
// Illegal: cancelled AND delivered → 💥 contradiction
// Illegal: isShipped = true but trackingNumber = undefined → 💥
// 4 booleans = 2^4 = 16 combinations. Only 5 are VALID!
```

### Solution: DU = only valid states

```typescript
// filename: src/state_machine.ts
import assert from "node:assert/strict";

// ✅ DU: ONLY valid states exist. Illegal states = NOT representable
type OrderState =
    | { readonly tag: "draft"; readonly items: readonly string[] }
    | { readonly tag: "pending_payment"; readonly items: readonly string[]; readonly total: number }
    | { readonly tag: "paid"; readonly items: readonly string[]; readonly total: number; readonly paymentId: string }
    | { readonly tag: "shipped"; readonly items: readonly string[]; readonly trackingNumber: string; readonly shippedAt: Date }
    | { readonly tag: "delivered"; readonly deliveredAt: Date; readonly signedBy: string }
    | { readonly tag: "cancelled"; readonly reason: string; readonly cancelledAt: Date };

// Mỗi state có CHÍNH XÁC data cần thiết:
// - "shipped" PHẢI có trackingNumber (không optional!)
// - "delivered" PHẢI có deliveredAt
// - "cancelled" PHẢI có reason

// Transition functions — compiler enforces valid transitions
const submitOrder = (
    order: OrderState & { tag: "draft" },
    total: number
): OrderState & { tag: "pending_payment" } => ({
    tag: "pending_payment",
    items: order.items,
    total,
});

const payOrder = (
    order: OrderState & { tag: "pending_payment" },
    paymentId: string
): OrderState & { tag: "paid" } => ({
    tag: "paid",
    items: order.items,
    total: order.total,
    paymentId,
});

const shipOrder = (
    order: OrderState & { tag: "paid" },
    trackingNumber: string,
    now: Date
): OrderState & { tag: "shipped" } => ({
    tag: "shipped",
    items: order.items,
    trackingNumber,
    shippedAt: now,
});

const deliverOrder = (
    order: OrderState & { tag: "shipped" },
    signedBy: string,
    now: Date
): OrderState & { tag: "delivered" } => ({
    tag: "delivered",
    deliveredAt: now,
    signedBy,
});

const cancelOrder = (
    order: OrderState & { tag: "draft" | "pending_payment" },
    reason: string,
    now: Date
): OrderState & { tag: "cancelled" } => ({
    tag: "cancelled",
    reason,
    cancelledAt: now,
});

// Test: happy path
const now = new Date("2024-06-15T10:00:00Z");
const draft: OrderState = { tag: "draft", items: ["Laptop", "Mouse"] };

const pending = submitOrder(draft as OrderState & { tag: "draft" }, 21000000);
assert.strictEqual(pending.tag, "pending_payment");

const paid = payOrder(pending, "PAY-001");
assert.strictEqual(paid.tag, "paid");
assert.strictEqual(paid.paymentId, "PAY-001");

const shipped = shipOrder(paid, "TRK-001", now);
assert.strictEqual(shipped.tag, "shipped");
assert.strictEqual(shipped.trackingNumber, "TRK-001");

const delivered = deliverOrder(shipped, "Nguyễn Văn An", now);
assert.strictEqual(delivered.tag, "delivered");

// ❌ COMPILE ERROR (if properly typed):
// shipOrder(draft, "TRK", now);        // Can't ship draft!
// cancelOrder(delivered, "oops", now);  // Can't cancel delivered!

console.log("State machine OK ✅");
```

### State transition diagram (as code)

```
draft → pending_payment → paid → shipped → delivered
  ↓           ↓
cancelled   cancelled
```

```typescript
// filename: src/transition_map.ts

// Type-level state transition map
type ValidTransitions = {
    readonly draft: "pending_payment" | "cancelled";
    readonly pending_payment: "paid" | "cancelled";
    readonly paid: "shipped";
    readonly shipped: "delivered";
    readonly delivered: never;    // terminal state
    readonly cancelled: never;   // terminal state
};

// Generic transition function
const transition = <
    From extends keyof ValidTransitions,
    To extends ValidTransitions[From]
>(
    _from: From,
    to: To
): To => to;

// ✅ Valid transitions
const t1 = transition("draft", "pending_payment");           // OK
const t2 = transition("pending_payment", "paid");            // OK
const t3 = transition("draft", "cancelled");                 // OK

// ❌ Invalid transitions — COMPILE ERROR:
// transition("draft", "shipped");         // "shipped" not in ValidTransitions["draft"]
// transition("delivered", "cancelled");   // never — no transitions from delivered
```

> **💡 "Make illegal states unrepresentable"** (Yaron Minsky): Nếu state không hợp lệ CÓ THỂ TỒN TẠI trong type system → trước sau gì cũng có bug. DU guarantees CHỈ valid states.

---

## ✅ Checkpoint 20.2

> Đến đây bạn phải hiểu:
> 1. **Boolean flags** = 2^n combinations, most are ILLEGAL
> 2. **DU state machine** = only valid states. Mỗi variant = 1 state + CHÍNH XÁC data cần thiết
> 3. **Transition functions** = explicitly typed. `shipOrder(paid)` — only accepts "paid" state
> 4. **Compiler enforces**: `shipOrder(draft)` = compile error. Không cần runtime check
>
> **Test nhanh**: `OrderState & { tag: "shipped" }` guarantees gì?
> <details><summary>Đáp án</summary>Guarantees `trackingNumber: string` VÀ `shippedAt: Date` tồn tại. Không optional, không undefined. State = data guarantee.</details>

---

## 20.3 — Making Illegal States Unrepresentable

### Loại bỏ kẽ hở trong khuôn đúc

Optional fields = kẽ hở: `isVerified: boolean; verifiedAt?: Date` cho phép `isVerified = false, verifiedAt = new Date()` — nghịch lý, vô nghĩa, bug. Giải pháp: KHÔNG dùng optional + boolean. Dùng DU — mỗi variant chứa ĐÚNG data cần thiết cho trạng thái đó. `VerificationStatus = "unverified" | { tag: "verified"; verifiedAt: Date }`. "Verified mà không có verifiedAt" = TYPE ERROR, không tồn tại.

Nested DUs mạnh mẽ hơn: `User.auth` (password | google | github), `User.verification` (unverified | verified), `User.loginHistory` (never | has_logged_in) — mỗi DU độc lập, mỗi variant mang data riêng.

```typescript
// filename: src/illegal_states.ts
import assert from "node:assert/strict";

// ❌ BAD: optional fields → illegal states
type UserBad = {
    readonly name: string;
    readonly email: string;
    readonly isVerified: boolean;
    readonly verifiedAt?: Date;          // only if isVerified = true
    readonly password?: string;          // only if registered
    readonly googleId?: string;          // only if google auth
    readonly lastLoginAt?: Date;         // only if ever logged in
};
// Illegal: isVerified = false BUT verifiedAt has a date
// Illegal: password AND googleId both set (which auth method?)
// Illegal: lastLoginAt set but isVerified = false

// ✅ GOOD: DU eliminates illegal states
type AuthMethod =
    | { readonly tag: "password"; readonly passwordHash: string }
    | { readonly tag: "google"; readonly googleId: string; readonly googleEmail: string }
    | { readonly tag: "github"; readonly githubId: string; readonly githubUsername: string };

type VerificationStatus =
    | { readonly tag: "unverified" }
    | { readonly tag: "verified"; readonly verifiedAt: Date; readonly verificationMethod: "email" | "admin" };

type LoginHistory =
    | { readonly tag: "never_logged_in" }
    | { readonly tag: "has_logged_in"; readonly lastLoginAt: Date; readonly loginCount: number };

type User = {
    readonly id: string;
    readonly name: string;
    readonly email: string;
    readonly auth: AuthMethod;
    readonly verification: VerificationStatus;
    readonly loginHistory: LoginHistory;
};

// Functions that REQUIRE specific states
const getLastLogin = (
    user: User & { readonly loginHistory: LoginHistory & { tag: "has_logged_in" } }
): Date => user.loginHistory.lastLoginAt;

const getGoogleEmail = (
    user: User & { readonly auth: AuthMethod & { tag: "google" } }
): string => user.auth.googleEmail;

// Pattern: check state before accessing data
const displayLastLogin = (user: User): string => {
    switch (user.loginHistory.tag) {
        case "never_logged_in":
            return "Chưa từng đăng nhập";
        case "has_logged_in":
            return `Lần cuối: ${user.loginHistory.lastLoginAt.toLocaleDateString()}`;
    }
};

// Test
const user: User = {
    id: "U1",
    name: "An",
    email: "an@mail.com",
    auth: { tag: "google", googleId: "G123", googleEmail: "an@gmail.com" },
    verification: { tag: "verified", verifiedAt: new Date("2024-01-15"), verificationMethod: "email" },
    loginHistory: { tag: "has_logged_in", lastLoginAt: new Date("2024-06-15"), loginCount: 42 },
};

assert.strictEqual(displayLastLogin(user), "Lần cuối: 6/15/2024");

const newUser: User = {
    id: "U2",
    name: "Bình",
    email: "binh@mail.com",
    auth: { tag: "password", passwordHash: "hashed..." },
    verification: { tag: "unverified" },
    loginHistory: { tag: "never_logged_in" },
};

assert.strictEqual(displayLastLogin(newUser), "Chưa từng đăng nhập");

console.log("Illegal states OK ✅");
```

### Quy tắc: "When in doubt, make it a DU"

| ❌ Don't | ✅ Do |
|----------|-------|
| `isPaid: boolean; paidAt?: Date` | `PaymentStatus: "unpaid" \| { tag: "paid"; paidAt: Date }` |
| `role: string` | `Role: "admin" \| "editor" \| "viewer"` |
| `data: any; type: string` | `Content: { tag: "text"; body: string } \| { tag: "image"; url: string }` |
| `error?: string; success?: boolean` | `Result<T, E>` |

---

## ✅ Checkpoint 20.3

> Đến đây bạn phải hiểu:
> 1. **Optional fields** = breeding ground for illegal states
> 2. **DU eliminates optionals**: each variant has EXACTLY needed data
> 3. **Nested DUs**: `User.auth`, `User.verification`, `User.loginHistory` — each independent
> 4. **Rule**: "If a field is only valid in certain states → make it part of that state's variant"
>
> **Test nhanh**: `isAdmin: boolean; adminPermissions?: string[]` — vấn đề gì?
> <details><summary>Đáp án</summary>**Illegal state**: `isAdmin = false` BUT `adminPermissions = ["delete"]`. Fix: `Role: { tag: "admin"; permissions: string[] } | { tag: "user" }`. Admin ALWAYS has permissions.</details>

---

## 20.4 — Complex Domain Hierarchies

### Cây khuôn đúc: khuôn lớn chứa khuôn nhỏ

Thực tế phức tạp hơn khuôn đơn. Hệ thống e-commerce có Product — nhưng Product có 3 loại hoàn toàn khác nhau: Physical (cần cân nặng, kích thước, tồn kho), Digital (cần downloadUrl, license), Subscription (cần billing cycle, trial status). Và mỗi loại lại có sub-types: StockStatus có 4 trạng thái, LicenseType có 3 loại, TrialStatus có 4 giai đoạn.

Đây là **nested DU hierarchy** — cây khuôn đúc. Exhaustive switch tại mỗi cấp điều hướng cây. Thêm variant mới ở bất kỳ cấp nào? Compiler báo TẤT CẢ switch thiếu case. Bản đồ domain trở thành TYPE TREE mà compiler navigate giúp bạn.

```typescript
// filename: src/domain_hierarchy.ts
import assert from "node:assert/strict";

// Product = complex DU hierarchy

// Level 1: Product type
type Product =
    | PhysicalProduct
    | DigitalProduct
    | SubscriptionProduct;

// Level 2: Physical products
type PhysicalProduct = {
    readonly tag: "physical";
    readonly id: string;
    readonly name: string;
    readonly price: number;
    readonly weight: number;       // kg — for shipping
    readonly dimensions: Dimensions;
    readonly stock: StockStatus;
};

type Dimensions = {
    readonly length: number;
    readonly width: number;
    readonly height: number;
};

type StockStatus =
    | { readonly tag: "in_stock"; readonly quantity: number }
    | { readonly tag: "low_stock"; readonly quantity: number; readonly reorderThreshold: number }
    | { readonly tag: "out_of_stock"; readonly restockDate?: Date }
    | { readonly tag: "discontinued" };

// Level 2: Digital products
type DigitalProduct = {
    readonly tag: "digital";
    readonly id: string;
    readonly name: string;
    readonly price: number;
    readonly downloadUrl: string;
    readonly fileSizeMB: number;
    readonly format: "pdf" | "epub" | "mp4" | "zip";
    readonly license: LicenseType;
};

type LicenseType =
    | { readonly tag: "personal"; readonly maxDevices: number }
    | { readonly tag: "commercial"; readonly seats: number }
    | { readonly tag: "open_source"; readonly license: string };

// Level 2: Subscription products
type SubscriptionProduct = {
    readonly tag: "subscription";
    readonly id: string;
    readonly name: string;
    readonly monthlyPrice: number;
    readonly billingCycle: "monthly" | "yearly";
    readonly tier: SubscriptionTier;
    readonly trial: TrialStatus;
};

type SubscriptionTier =
    | { readonly tag: "basic"; readonly features: readonly string[] }
    | { readonly tag: "pro"; readonly features: readonly string[]; readonly priority: boolean }
    | { readonly tag: "enterprise"; readonly features: readonly string[]; readonly dedicatedSupport: boolean; readonly sla: string };

type TrialStatus =
    | { readonly tag: "no_trial" }
    | { readonly tag: "trial_available"; readonly durationDays: number }
    | { readonly tag: "in_trial"; readonly startedAt: Date; readonly endsAt: Date }
    | { readonly tag: "trial_expired"; readonly expiredAt: Date };

// --- Operations on product hierarchy ---

const getDisplayPrice = (product: Product): string => {
    switch (product.tag) {
        case "physical":
        case "digital":
            return `${product.price.toLocaleString()} VND`;
        case "subscription":
            return product.billingCycle === "monthly"
                ? `${product.monthlyPrice.toLocaleString()} VND/tháng`
                : `${(product.monthlyPrice * 12 * 0.8).toLocaleString()} VND/năm`;  // 20% off yearly
    }
};

const canPurchase = (product: Product): boolean => {
    switch (product.tag) {
        case "physical":
            return product.stock.tag === "in_stock" || product.stock.tag === "low_stock";
        case "digital":
            return true;  // always available
        case "subscription":
            return product.trial.tag !== "trial_expired";
    }
};

const getShippingWeight = (product: Product): number | null => {
    switch (product.tag) {
        case "physical": return product.weight;
        case "digital": return null;       // no shipping
        case "subscription": return null;  // no shipping
    }
};

// Test
const laptop: Product = {
    tag: "physical",
    id: "P1",
    name: "MacBook Pro",
    price: 50000000,
    weight: 2.1,
    dimensions: { length: 35, width: 25, height: 1.5 },
    stock: { tag: "in_stock", quantity: 15 },
};

const ebook: Product = {
    tag: "digital",
    id: "P2",
    name: "TypeScript Handbook",
    price: 200000,
    downloadUrl: "https://example.com/ts-book.pdf",
    fileSizeMB: 12,
    format: "pdf",
    license: { tag: "personal", maxDevices: 3 },
};

const cloud: Product = {
    tag: "subscription",
    id: "P3",
    name: "Cloud Pro",
    monthlyPrice: 500000,
    billingCycle: "yearly",
    tier: { tag: "pro", features: ["storage", "api", "support"], priority: true },
    trial: { tag: "trial_available", durationDays: 14 },
};

assert.strictEqual(getDisplayPrice(laptop), "50,000,000 VND");
assert.strictEqual(canPurchase(laptop), true);
assert.strictEqual(getShippingWeight(laptop), 2.1);

assert.strictEqual(canPurchase(ebook), true);
assert.strictEqual(getShippingWeight(ebook), null);

assert.strictEqual(canPurchase(cloud), true);

console.log("Domain hierarchy OK ✅");
```

> **💡 DU hierarchy = domain as TYPE TREE**: Product → Physical/Digital/Subscription. Physical.stock → InStock/LowStock/OutOfStock/Discontinued. Subscription.trial → NoTrial/Available/InTrial/Expired. Compiler navigates the tree. No runtime "what type is this?" guessing.

---

## ✅ Checkpoint 20.4

> Đến đây bạn phải hiểu:
> 1. **Domain hierarchy** = nested DUs. Product → subtypes → sub-subtypes
> 2. **Exhaustive switch** at each level navigates the hierarchy
> 3. **Each variant** carries EXACTLY its data. Physical has weight. Digital has downloadUrl
> 4. **Operations** on hierarchy = switch per level. Type system guides you
>
> **Test nhanh**: Thêm `BundleProduct` (gói nhiều products) — cần sửa gì?
> <details><summary>Đáp án</summary>1. Thêm variant `{ tag: "bundle"; products: readonly Product[] }` vào `Product` DU. 2. Mọi switch trên Product báo compile error (thiếu case "bundle"). Compiler chỉ chỗ cần sửa!</details>

---

## 20.5 — Aggregate Root: DU + Validation

### Khuôn đúc hoàn chỉnh: từ nguyên liệu thô đến thành phẩm

Aggregate root kết hợp tất cả: branded types cho Value Objects, DU cho state machine, smart constructors cho validation TẠI MỌI BƯỚC. Order aggregate: `createDraft` validate items → `confirmDraft` tính tax → `shipConfirmed` require tracking number → `deliverShipped` require signature. Mỗi transition = function nhận STATE CỤ THỂ, return STATE MỚI. Cancel logic tùy state: cancel draft = refund 0 (chưa trả tiền), cancel confirmed = full refund.

```typescript
// filename: src/aggregate_root.ts
import assert from "node:assert/strict";

// === Types ===

type OrderId = string & { readonly __brand: "OrderId" };
type CustomerId = string & { readonly __brand: "CustomerId" };
type Money = number & { readonly __brand: "Money" };
const Money = (n: number): Money => n as Money;

type OrderItem = {
    readonly productId: string;
    readonly productName: string;
    readonly quantity: number;
    readonly unitPrice: Money;
    readonly lineTotal: Money;
};

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// === Order state machine (aggregate root) ===

type Order =
    | DraftOrder
    | ConfirmedOrder
    | ShippedOrder
    | DeliveredOrder
    | CancelledOrder;

type DraftOrder = {
    readonly tag: "draft";
    readonly id: OrderId;
    readonly customerId: CustomerId;
    readonly items: readonly OrderItem[];
    readonly subtotal: Money;
};

type ConfirmedOrder = {
    readonly tag: "confirmed";
    readonly id: OrderId;
    readonly customerId: CustomerId;
    readonly items: readonly OrderItem[];
    readonly subtotal: Money;
    readonly tax: Money;
    readonly total: Money;
    readonly confirmedAt: Date;
};

type ShippedOrder = {
    readonly tag: "shipped";
    readonly id: OrderId;
    readonly customerId: CustomerId;
    readonly items: readonly OrderItem[];
    readonly total: Money;
    readonly trackingNumber: string;
    readonly shippedAt: Date;
};

type DeliveredOrder = {
    readonly tag: "delivered";
    readonly id: OrderId;
    readonly customerId: CustomerId;
    readonly total: Money;
    readonly deliveredAt: Date;
    readonly signedBy: string;
};

type CancelledOrder = {
    readonly tag: "cancelled";
    readonly id: OrderId;
    readonly customerId: CustomerId;
    readonly reason: string;
    readonly cancelledAt: Date;
    readonly refundAmount: Money;
};

// === Domain operations ===

const createDraft = (
    id: OrderId,
    customerId: CustomerId,
    items: readonly OrderItem[]
): Result<DraftOrder, string> => {
    if (items.length === 0) return err("Order must have at least 1 item");

    const subtotal = Money(items.reduce((sum, item) => sum + item.lineTotal, 0));

    return ok({
        tag: "draft",
        id,
        customerId,
        items,
        subtotal,
    });
};

const TAX_RATE = 0.1;

const confirmDraft = (
    order: DraftOrder,
    now: Date
): ConfirmedOrder => {
    const tax = Money(Math.round(order.subtotal * TAX_RATE));
    return {
        tag: "confirmed",
        id: order.id,
        customerId: order.customerId,
        items: order.items,
        subtotal: order.subtotal,
        tax,
        total: Money(order.subtotal + tax),
        confirmedAt: now,
    };
};

const shipConfirmed = (
    order: ConfirmedOrder,
    trackingNumber: string,
    now: Date
): Result<ShippedOrder, string> => {
    if (trackingNumber.trim().length === 0)
        return err("Tracking number required");

    return ok({
        tag: "shipped",
        id: order.id,
        customerId: order.customerId,
        items: order.items,
        total: order.total,
        trackingNumber: trackingNumber.trim(),
        shippedAt: now,
    });
};

const deliverShipped = (
    order: ShippedOrder,
    signedBy: string,
    now: Date
): Result<DeliveredOrder, string> => {
    if (signedBy.trim().length === 0)
        return err("Signature required");

    return ok({
        tag: "delivered",
        id: order.id,
        customerId: order.customerId,
        total: order.total,
        deliveredAt: now,
        signedBy: signedBy.trim(),
    });
};

const cancelDraft = (
    order: DraftOrder,
    reason: string,
    now: Date
): CancelledOrder => ({
    tag: "cancelled",
    id: order.id,
    customerId: order.customerId,
    reason,
    cancelledAt: now,
    refundAmount: Money(0),  // draft — chưa thanh toán
});

const cancelConfirmed = (
    order: ConfirmedOrder,
    reason: string,
    now: Date
): CancelledOrder => ({
    tag: "cancelled",
    id: order.id,
    customerId: order.customerId,
    reason,
    cancelledAt: now,
    refundAmount: order.total,  // full refund
});

// === Display (works on ANY order state) ===

const getOrderSummary = (order: Order): string => {
    switch (order.tag) {
        case "draft":
            return `📝 Draft: ${order.items.length} items, ${order.subtotal.toLocaleString()} VND`;
        case "confirmed":
            return `✅ Confirmed: ${order.total.toLocaleString()} VND (tax: ${order.tax.toLocaleString()})`;
        case "shipped":
            return `🚚 Shipped: tracking ${order.trackingNumber}`;
        case "delivered":
            return `📦 Delivered: signed by ${order.signedBy}`;
        case "cancelled":
            return `❌ Cancelled: ${order.reason}`;
    }
};

// === Test ===

const now = new Date("2024-06-15T10:00:00Z");

const item: OrderItem = {
    productId: "P1",
    productName: "Laptop",
    quantity: 1,
    unitPrice: Money(20000000),
    lineTotal: Money(20000000),
};

// Happy path: draft → confirmed → shipped → delivered
const draftResult = createDraft("ORD-1" as OrderId, "C-1" as CustomerId, [item]);
assert.strictEqual(draftResult.tag, "ok");
const draft = draftResult.tag === "ok" ? draftResult.value : null;
assert.ok(draft !== null);

const confirmed = confirmDraft(draft!, now);
assert.strictEqual(confirmed.tag, "confirmed");
assert.strictEqual(confirmed.subtotal, 20000000);
assert.strictEqual(confirmed.tax, 2000000);       // 20M * 10%
assert.strictEqual(confirmed.total, 22000000);     // 20M + 2M

const shippedResult = shipConfirmed(confirmed, "TRK-001", now);
assert.strictEqual(shippedResult.tag, "ok");

// Cancel path
const draftForCancel = createDraft("ORD-2" as OrderId, "C-2" as CustomerId, [item]);
if (draftForCancel.tag === "ok") {
    const cancelled = cancelDraft(draftForCancel.value, "Changed mind", now);
    assert.strictEqual(cancelled.refundAmount, 0);

    const confirmedForCancel = confirmDraft(draftForCancel.value, now);
    const cancelledConfirmed = cancelConfirmed(confirmedForCancel, "Out of stock", now);
    assert.strictEqual(cancelledConfirmed.refundAmount, 22000000);  // full refund
}

// Error: empty items
const emptyOrder = createDraft("ORD-3" as OrderId, "C-3" as CustomerId, []);
assert.strictEqual(emptyOrder.tag, "err");

console.log("Aggregate root OK ✅");
```

---

## ✅ Checkpoint 20.5

> Đến đây bạn phải hiểu:
> 1. **Aggregate root** = Order (DU) owns all transitions. No external mutation
> 2. **Each transition** = function accepting SPECIFIC state → returning NEW state
> 3. **Validation** at creation (`createDraft`) AND at transitions (`shipConfirmed`)
> 4. **Refund logic** embedded in state: cancel draft = 0, cancel confirmed = full refund
>
> **Test nhanh**: `cancelConfirmed(shippedOrder, ...)` — có compile error không?
> <details><summary>Đáp án</summary>**CÓ!** `cancelConfirmed` nhận `ConfirmedOrder`, không nhận `ShippedOrder`. Cancel sau ship cần policy khác (partial refund?). Compiler enforces business rules.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Value Objects

```typescript
// Viết Value Objects cho:
// 1. PhoneNumber — 10 digits, starts with 0
// 2. Age — integer, 0-150
// 3. Hàm formatPhone(phone: PhoneNumber): string → "0xxx-xxx-xxx"
// Test: PhoneNumber("0987654321") → OK, PhoneNumber("123") → null
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type PhoneNumber = string & { readonly __brand: "PhoneNumber" };

const PhoneNumber = (raw: string): PhoneNumber | null => {
    const digits = raw.replace(/\D/g, "");
    return /^0\d{9}$/.test(digits) ? digits as PhoneNumber : null;
};

type Age = number & { readonly __brand: "Age" };

const Age = (value: number): Age | null =>
    Number.isInteger(value) && value >= 0 && value <= 150 ? value as Age : null;

const formatPhone = (phone: PhoneNumber): string =>
    `${phone.slice(0, 4)}-${phone.slice(4, 7)}-${phone.slice(7)}`;

// Test
const phone = PhoneNumber("0987654321");
assert.ok(phone !== null);
assert.strictEqual(formatPhone(phone!), "0987-654-321");

assert.strictEqual(PhoneNumber("123"), null);
assert.strictEqual(PhoneNumber("1234567890"), null);  // doesn't start with 0

const age = Age(25);
assert.ok(age !== null);
assert.strictEqual(Age(-1), null);
assert.strictEqual(Age(200), null);
assert.strictEqual(Age(3.5), null);  // not integer
```

</details>

---

**Bài 2** (10 phút): State machine

```typescript
// Viết state machine cho "Ticket System":
// States: open → assigned → in_progress → resolved → closed
//         open → cancelled (at any non-terminal state)
//
// 1. Type mỗi state với data riêng
// 2. Viết transition functions (chỉ chấp nhận correct state)
// 3. Test: happy path + cancel + invalid transition
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type TicketState =
    | { readonly tag: "open"; readonly title: string; readonly createdAt: Date }
    | { readonly tag: "assigned"; readonly title: string; readonly assignee: string; readonly assignedAt: Date }
    | { readonly tag: "in_progress"; readonly title: string; readonly assignee: string; readonly startedAt: Date }
    | { readonly tag: "resolved"; readonly title: string; readonly assignee: string; readonly resolution: string; readonly resolvedAt: Date }
    | { readonly tag: "closed"; readonly closedAt: Date; readonly satisfaction?: number }
    | { readonly tag: "cancelled"; readonly reason: string; readonly cancelledAt: Date };

const createTicket = (title: string, now: Date): TicketState & { tag: "open" } => ({
    tag: "open",
    title,
    createdAt: now,
});

const assignTicket = (
    ticket: TicketState & { tag: "open" },
    assignee: string,
    now: Date
): TicketState & { tag: "assigned" } => ({
    tag: "assigned",
    title: ticket.title,
    assignee,
    assignedAt: now,
});

const startWork = (
    ticket: TicketState & { tag: "assigned" },
    now: Date
): TicketState & { tag: "in_progress" } => ({
    tag: "in_progress",
    title: ticket.title,
    assignee: ticket.assignee,
    startedAt: now,
});

const resolveTicket = (
    ticket: TicketState & { tag: "in_progress" },
    resolution: string,
    now: Date
): TicketState & { tag: "resolved" } => ({
    tag: "resolved",
    title: ticket.title,
    assignee: ticket.assignee,
    resolution,
    resolvedAt: now,
});

const closeTicket = (
    ticket: TicketState & { tag: "resolved" },
    now: Date,
    satisfaction?: number
): TicketState & { tag: "closed" } => ({
    tag: "closed",
    closedAt: now,
    satisfaction,
});

const cancelTicket = (
    ticket: TicketState & { tag: "open" | "assigned" | "in_progress" },
    reason: string,
    now: Date
): TicketState & { tag: "cancelled" } => ({
    tag: "cancelled",
    reason,
    cancelledAt: now,
});

// Test
const now = new Date("2024-06-15");

const t = createTicket("Bug #123", now);
assert.strictEqual(t.tag, "open");

const assigned = assignTicket(t, "Agent-A", now);
assert.strictEqual(assigned.assignee, "Agent-A");

const started = startWork(assigned, now);
const resolved = resolveTicket(started, "Fixed in v2.1", now);
const closed = closeTicket(resolved, now, 5);
assert.strictEqual(closed.tag, "closed");

// Cancel
const t2 = createTicket("Feature #456", now);
const cancelled = cancelTicket(t2, "Duplicate", now);
assert.strictEqual(cancelled.tag, "cancelled");
```

</details>

---

**Bài 3** (15 phút): Complex domain hierarchy

```typescript
// Viết domain model cho "Media Library":
// Media = Image | Video | Audio | Document
//
// Image: width, height, format (jpg|png|webp), altText
// Video: duration, resolution (720p|1080p|4K), subtitles (none|srt|vtt)
// Audio: duration, bitrate, format (mp3|flac|wav)
// Document: pageCount, format (pdf|docx), isSearchable
//
// 1. Viết DU hierarchy
// 2. getFileSize(media): number (estimated)
// 3. canStream(media): boolean (only video/audio)
// 4. getThumbnail(media): string | null (only image/video)
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type Media =
    | ImageMedia
    | VideoMedia
    | AudioMedia
    | DocumentMedia;

type ImageMedia = {
    readonly tag: "image";
    readonly id: string;
    readonly name: string;
    readonly width: number;
    readonly height: number;
    readonly format: "jpg" | "png" | "webp";
    readonly altText: string;
    readonly fileSizeKB: number;
};

type VideoMedia = {
    readonly tag: "video";
    readonly id: string;
    readonly name: string;
    readonly durationSeconds: number;
    readonly resolution: "720p" | "1080p" | "4K";
    readonly subtitles: SubtitleStatus;
    readonly thumbnailUrl: string;
    readonly fileSizeMB: number;
};

type SubtitleStatus =
    | { readonly tag: "none" }
    | { readonly tag: "available"; readonly format: "srt" | "vtt"; readonly language: string };

type AudioMedia = {
    readonly tag: "audio";
    readonly id: string;
    readonly name: string;
    readonly durationSeconds: number;
    readonly bitrate: number;  // kbps
    readonly format: "mp3" | "flac" | "wav";
    readonly fileSizeMB: number;
};

type DocumentMedia = {
    readonly tag: "document";
    readonly id: string;
    readonly name: string;
    readonly pageCount: number;
    readonly format: "pdf" | "docx";
    readonly isSearchable: boolean;
    readonly fileSizeKB: number;
};

const getFileSizeMB = (media: Media): number => {
    switch (media.tag) {
        case "image": return media.fileSizeKB / 1024;
        case "video": return media.fileSizeMB;
        case "audio": return media.fileSizeMB;
        case "document": return media.fileSizeKB / 1024;
    }
};

const canStream = (media: Media): boolean => {
    switch (media.tag) {
        case "video":
        case "audio":
            return true;
        case "image":
        case "document":
            return false;
    }
};

const getThumbnail = (media: Media): string | null => {
    switch (media.tag) {
        case "image": return `thumbnail_${media.id}.${media.format}`;
        case "video": return media.thumbnailUrl;
        case "audio": return null;
        case "document": return null;
    }
};

// Test
const photo: Media = {
    tag: "image",
    id: "IMG-1",
    name: "sunset.jpg",
    width: 1920,
    height: 1080,
    format: "jpg",
    altText: "Beautiful sunset",
    fileSizeKB: 2048,
};

const video: Media = {
    tag: "video",
    id: "VID-1",
    name: "tutorial.mp4",
    durationSeconds: 600,
    resolution: "1080p",
    subtitles: { tag: "available", format: "vtt", language: "vi" },
    thumbnailUrl: "https://example.com/thumb.jpg",
    fileSizeMB: 150,
};

assert.strictEqual(getFileSizeMB(photo), 2);
assert.strictEqual(canStream(photo), false);
assert.strictEqual(getThumbnail(photo), "thumbnail_IMG-1.jpg");

assert.strictEqual(canStream(video), true);
assert.strictEqual(getThumbnail(video), "https://example.com/thumb.jpg");
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "Type too complex" error | Deeply nested DUs | Break into smaller types. Flatten hierarchy if > 3 levels |
| Can't narrow DU in function | Missing `switch` or `if` check | Use exhaustive switch or `tag` check before access |
| State transitions too many | Overly granular states | Combine similar states. "assigned" + "in_progress" → "active" |
| Branded type leaks `as` | Casting everywhere | Use smart constructors at boundaries. ONLY constructors use `as` |
| Optional fields creeping back | Habit from OOP | Ask: "Is this field only valid in some states?" → separate variant |

---

## Tóm tắt

Chương này hoàn thiện bộ công cụ domain modeling — từ khuôn đúc precision (branded types) qua bản thiết kế chính xác (DU state machines) đến hệ thống khuôn phân cấp (nested DU hierarchies). Mọi concept kết nối: Ch13 cho branded types, Ch14 cho smart constructors, Ch18 cho DDD vocabulary, và giờ Ch20 gom tất cả thành aggregate roots hoàn chỉnh.

- ✅ **Value Objects** = branded types + smart constructors. Valid by construction. Compare by value.
- ✅ **State machines** = DU. Each variant = 1 valid state + its exact data. No illegal states.
- ✅ **Transition functions** = typed: `(StateA) → StateB`. Compiler enforces valid transitions.
- ✅ **Make illegal states unrepresentable**: no booleans + optionals. DU + exact data per state.
- ✅ **Domain hierarchies**: nested DUs for complex models. Product → Physical/Digital/Subscription.
- ✅ **Aggregate roots**: DU-based state machines with validation at every transition.

## Tiếp theo

→ Chapter 21: **Workflows as Pipelines** — `pipe()`, `flow()`, async pipelines, composing domain operations into workflows.
