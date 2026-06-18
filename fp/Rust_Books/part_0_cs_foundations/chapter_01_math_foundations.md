# Chapter 1 — Math Foundations for FP

> **Bạn sẽ học được**:
> - Lambda Calculus là gì — và tại sao nó đơn giản hơn bạn tưởng
> - Curry-Howard Correspondence — compiler chính là "thầy giáo" kiểm tra bài cho bạn
> - Dùng phép cộng và phép nhân để đếm số trạng thái chương trình (**Algebraic Type Sizes**)
> - Tại sao "make illegal states unrepresentable" giúp code hết bug
>
> **Yêu cầu trước**: Chapter 0 (Rust in 10 Minutes) nếu bạn chưa biết Rust. Về toán, chỉ cần biết cộng và nhân là đủ.
> **Thời gian đọc**: ~45 phút | **Level**: CS Foundations
> **Kết quả cuối cùng**: Nhìn vào bất kỳ data model nào và tính được "có bao nhiêu trạng thái sai có thể xảy ra" — rồi sửa thiết kế để trạng thái sai = 0.

---

## Trước khi bắt đầu: Bạn không cần "giỏi toán"

Nếu bạn đọc tên chapter — "Math Foundations" — và thấy hơi lo, hoàn toàn bình thường. Nhưng chapter này **không** yêu cầu bạn giải phương trình hay nhớ công thức phức tạp.

Toàn bộ "toán" ở đây chỉ cần 3 thứ:
- Biết **cộng**: 2 + 3 = 5
- Biết **nhân**: 2 × 3 = 6
- Hiểu câu: "Nếu trời mưa **thì** mang ô"

Nếu bạn hiểu được 3 thứ trên — kể cả bạn mới học lớp 3 — bạn đã đủ nền tảng cho toàn bộ chapter này rồi. Mình sẽ đi từng bước nhỏ, với rất nhiều ví dụ từ đời thường.

---

## 1.1 — Lambda Calculus: Chiếc máy xay chỉ có một nút

### Câu chuyện

Hãy tưởng tượng bạn có một chiếc **máy xay sinh tố** thần kỳ. Bạn bỏ **trái cây** vào → máy xay → ra **sinh tố**. Máy này luôn hoạt động giống nhau: cùng loại trái cây vào, luôn ra cùng loại sinh tố. Không bao giờ bỏ cam vào mà ra nước dừa.

Đó chính là **function** — khái niệm quan trọng nhất trong lập trình.

Năm 1936, nhà toán học Alonzo Church đặt câu hỏi: *"Nếu ta chỉ dùng máy xay (functions) — không cần tủ lạnh (biến), không cần lò nướng (vòng lặp), không cần gì khác — liệu ta có thể nấu được MỌI món ăn (mọi phép tính) không?"*

Câu trả lời: **Có.** Và hệ thống đó gọi là **Lambda Calculus**.

### Lambda = "Cái máy xay" không tên

Trong toán, Lambda Calculus viết function thế này:

```
λx. x + 1
```

Đọc từng phần:
- `λ` (lambda) = "đây là một cái máy xay"
- `x` = "bỏ cái gì đó vào, gọi nó là x"
- `.` = "rồi làm phép tính sau đây"
- `x + 1` = "trả ra x cộng 1"

Ghép lại: **"Đây là cái máy, bỏ x vào, ra x + 1".**

Trong Rust, bạn viết gần như y hệt — chỉ đổi `λ` thành `||`:

```rust
// filename: src/main.rs
fn main() {
    // Tạo "cái máy xay" — nhận x, trả ra x + 1
    let add_one = |x: i32| x + 1;

    // Bỏ số 5 vào máy
    let result = add_one(5);

    println!("add_one(5) = {}", result);
    // Output: add_one(5) = 6

    assert_eq!(result, 6);
}
```

Dấu `||` trong Rust thay cho `λ` — vì bàn phím không có phím lambda! Đây gọi là **closure** — tên gọi chính thức của "chiếc máy xay không tên" trong Rust.

| Toán (Lambda Calculus) | Rust (Closure) | Nghĩa bằng lời |
|---|---|---|
| `λx. x + 1` | `\|x\| x + 1` | Máy cộng 1 |
| `λx. x * 2` | `\|x\| x * 2` | Máy nhân đôi |
| `λx. λy. x + y` | `\|x, y\| x + y` | Máy cộng hai số |

### β-reduction = "Bỏ trái cây vào máy xay"

Khi bạn **bỏ một giá trị cụ thể vào function**, toán học gọi đó là **β-reduction** (beta reduction). Nghe phức tạp, nhưng thực ra cực kỳ đơn giản — chỉ là **thay số vào rồi tính**:

```
Máy:      λx. x + 1
Bỏ 5 vào: (λx. x + 1) 5
Thay x=5:  5 + 1
Kết quả:   6
```

Giống như: Máy xay cam → bỏ cam vào → ra nước cam. Chỉ vậy thôi.

```rust
// filename: src/main.rs
fn main() {
    let add_one = |x: i32| x + 1;

    // Mỗi dòng đều là β-reduction — thay x vào rồi tính:
    println!("add_one(5) = {}", add_one(5));    // thay x=5 → 6
    println!("add_one(0) = {}", add_one(0));    // thay x=0 → 1
    println!("add_one(99) = {}", add_one(99));  // thay x=99 → 100

    // Output:
    // add_one(5) = 6
    // add_one(0) = 1
    // add_one(99) = 100
}
```

> **💡 Ghi nhớ**: Mỗi lần bạn gọi function trong Rust, bạn đang thực hiện β-reduction. Nghe "oai" chứ thực ra chỉ là "thay giá trị vào rồi tính". Bạn đã làm hàng ngàn lần mà không biết nó có tên.

### Higher-Order Functions = "Máy xay chứa... máy xay khác"

Đây là lúc mọi thứ trở nên thú vị. Lambda Calculus cho phép **máy nhận máy khác làm nguyên liệu**.

Tưởng tượng một cái máy đặc biệt: bạn bỏ **một chiếc máy xay nhỏ** vào bên trong nó, rồi bỏ trái cây vào. Chiếc máy lớn sẽ dùng máy nhỏ để xay trái cây **hai lần**.

Hoặc nghĩ về **dây chuyền sản xuất** trong nhà máy:
- Máy A: cắt vải
- Máy B: may vải thành áo
- Dây chuyền: cho vải vào Máy A, kết quả đưa vào Máy B → ra áo

"Dây chuyền" ở đây là "máy lớn" nhận 2 "máy nhỏ" rồi nối chúng lại.

```rust
// filename: src/main.rs
fn main() {
    // Máy nhỏ 1: cộng 10
    let add_ten = |x: i32| x + 10;

    // Máy nhỏ 2: nhân đôi
    let double = |x: i32| x * 2;

    // "Máy lớn": nhận một máy nhỏ (f) và nguyên liệu (x)
    // Chạy máy nhỏ 2 lần liên tiếp
    let apply_twice = |f: &dyn Fn(i32) -> i32, x: i32| {
        let first = f(x);     // chạy máy lần 1
        let second = f(first); // lấy kết quả, chạy lần 2
        second
    };

    // Bỏ máy "cộng 10" vào, rồi bỏ số 5:
    // Lần 1: 5 + 10 = 15
    // Lần 2: 15 + 10 = 25
    let r1 = apply_twice(&add_ten, 5);
    println!("apply_twice(add_ten, 5) = {}", r1);
    assert_eq!(r1, 25);

    // Bỏ máy "nhân đôi" vào, rồi bỏ số 3:
    // Lần 1: 3 × 2 = 6
    // Lần 2: 6 × 2 = 12
    let r2 = apply_twice(&double, 3);
    println!("apply_twice(double, 3) = {}", r2);
    assert_eq!(r2, 12);

    // Output:
    // apply_twice(add_ten, 5) = 25
    // apply_twice(double, 3) = 12
}
```

`apply_twice` là **higher-order function** — function nhận function khác làm đầu vào. Bạn sẽ gặp pattern này rất nhiều trong Rust: `.map()`, `.filter()`, `.fold()` đều là "máy lớn" nhận "máy nhỏ" do bạn cung cấp.

> **💡 Nếu bạn là học sinh nhỏ tuổi**: Hãy nghĩ function như **hộp ma thuật**. Bạn bỏ thứ gì vào, nó phun ra thứ khác. Higher-order function = hộp ma thuật to hơn, bên trong chứa hộp ma thuật nhỏ!

### Church Encoding — Mọi thứ đều là function

Church còn chứng minh điều kỳ diệu: **chỉ cần functions thôi, bạn có thể tạo ra số, boolean, if/else, và mọi thứ khác.**

Ý tưởng: `TRUE` là function **chọn cái đầu tiên**, `FALSE` là function **chọn cái thứ hai**.

```rust
// filename: src/main.rs
fn main() {
    // TRUE = luôn chọn cái đầu tiên
    let church_true = |a: &str, _b: &str| a;

    // FALSE = luôn chọn cái thứ hai
    let church_false = |_a: &str, b: &str| b;

    // Hỏi TRUE: "đi biển" hay "ở nhà"? → chọn "đi biển"
    println!("church_true = {}", church_true("đi biển", "ở nhà"));

    // Hỏi FALSE: "đi biển" hay "ở nhà"? → chọn "ở nhà"
    println!("church_false = {}", church_false("đi biển", "ở nhà"));

    // Output:
    // church_true = đi biển
    // church_false = ở nhà
}
```

Đây là **if/else viết bằng function thuần túy!** Bạn không cần nhớ Church Encoding để code Rust, nhưng nó cho thấy một sự thật quan trọng: **functions là viên gạch cơ bản nhất** — từ functions, ta xây được mọi thứ.

---

## ✅ Checkpoint 1.1

> Đến đây bạn cần nhớ 4 điều:
> 1. **Function = "cái máy xay"**: bỏ nguyên liệu vào → ra kết quả
> 2. **Closure `|x| x + 1`** trong Rust = `λx. x + 1` trong toán = "máy cộng 1"
> 3. **β-reduction** = bỏ giá trị vào function rồi tính (bạn làm hàng ngày)
> 4. **Higher-order function** = "máy lớn" chứa "máy nhỏ" bên trong
>
> **Test nhanh**: `(λx. λy. x + y) 3 5` = bao nhiêu?
> <details><summary>Đáp án</summary>8 — bỏ 3 vào chỗ x, bỏ 5 vào chỗ y → 3 + 5 = 8. Giống gọi <code>|x, y| x + y</code> với (3, 5).</details>

---

## 1.2 — Curry-Howard: Types là lời hứa, Compiler là thầy giáo

### Câu chuyện: Hợp đồng giữa bạn và compiler

Khi bạn mua một ly cà phê, có một "hợp đồng" ngầm:
- Bạn đưa tiền → quán đưa cà phê
- Nếu không đủ tiền → quán từ chối (không phải đưa ly nước lã)

Hoặc khi bạn cầm hộp sữa ở siêu thị, trên hộp ghi "Sữa tươi 100% — 1 lít". Bạn tin rằng bên trong **đúng là sữa** (không phải nước lọc), dung tích **đúng 1 lít** (không phải 500ml). Nếu sai, cơ quan quản lý sẽ phạt.

**Types trong lập trình hoạt động y như vậy.** Chúng là **lời hứa** (hợp đồng) về đầu vào và đầu ra. Và "cơ quan quản lý" ở đây chính là **compiler** — nó kiểm tra xem lời hứa có đúng không trước khi cho chương trình chạy.

### Curry-Howard Correspondence

Hai nhà toán học, Haskell Curry và William Howard, phát hiện một điều bất ngờ: **lập trình và logic toán học thực ra là cùng một thứ**, chỉ viết khác nhau.

Nghe hàn lâm, nhưng ý nghĩa thực tế rất đơn giản:

| Bạn đã biết trong đời thường | Trong Rust | Trong logic |
|---|---|---|
| "Nếu có gạo **thì** nấu cơm được" | `fn cook(rice: Rice) -> Meal` | A → B |
| "Mang **cả** áo **và** quần" | `struct Outfit { top: Top, bottom: Bottom }` | A AND B |
| "Trả bằng tiền mặt **hoặc** thẻ" | `enum Payment { Cash, Card }` | A OR B |
| "Luôn luôn đúng" | `()` (unit — luôn tồn tại) | TRUE |

Ý nghĩa: **Nếu bạn viết được function `fn foo(a: A) -> B` chạy đúng**, bạn đã *chứng minh* rằng "từ A có thể suy ra B". Compiler kiểm tra types = compiler kiểm tra logic chương trình.

### Ví dụ cụ thể: Ba loại "lời hứa"

```rust
// filename: src/main.rs

// Lời hứa 1: "Cho số nguyên → trả số nguyên. Luôn luôn."
// Đơn giản, rõ ràng, 100% đáng tin.
fn double(x: i32) -> i32 {
    x * 2
}

// Lời hứa 2: "Cho chuỗi → CÓ THỂ trả số, HOẶC không"
// Option = "có thể có, có thể không" — trung thực!
fn parse_number(s: &str) -> Option<i32> {
    s.parse::<i32>().ok()
}

// Lời hứa 3: "Cho hai số → trả kết quả chia HOẶC lỗi"
// Result = "thành công hoặc thất bại — tôi nói rõ cả hai"
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero!".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    // Lời hứa 1: luôn luôn trả số
    println!("double(5) = {}", double(5));

    // Lời hứa 2: có thể thành công, có thể không
    // Compiler BẮT BUỘC bạn xử lý CẢ HAI trường hợp
    match parse_number("42") {
        Some(n) => println!("parse_number(\"42\") = {}", n),
        None    => println!("Không parse được!"),
    }
    match parse_number("abc") {
        Some(n) => println!("parse_number(\"abc\") = {}", n),
        None    => println!("\"abc\" không phải số!"),
    }

    // Lời hứa 3: thành công hoặc lỗi — phải xử lý cả hai
    match safe_divide(10.0, 3.0) {
        Ok(result) => println!("10 / 3 = {:.2}", result),
        Err(e) => println!("Lỗi: {}", e),
    }
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {:.2}", result),
        Err(e) => println!("Lỗi: {}", e),
    }

    // Output:
    // double(5) = 10
    // parse_number("42") = 42
    // "abc" không phải số!
    // 10 / 3 = 3.33
    // Lỗi: Cannot divide by zero!
}
```

### Tại sao điều này quan trọng?

Vì Rust compiler **bắt lỗi logic cho bạn**. Nếu function hứa trả `Result` nhưng bạn quên xử lý trường hợp `Err`, compiler sẽ **từ chối chạy chương trình**.

Đây không phải compiler khó tính. Nghĩ về compiler như **thầy giáo nghiêm khắc nhưng tốt bụng**: thầy không cho nộp bài nếu bạn chưa trả lời hết câu hỏi. Có thể phiền lúc đầu, nhưng khi thi (deploy lên production) thì bạn không bao giờ bỏ sót.

> **💡 Tip**: Khi compiler báo lỗi, đừng bực. Nghĩ rằng: "Compiler đang nhắc mình chưa xử lý hết trường hợp — bổ sung đi rồi thầy cho nộp." Đó là Curry-Howard đang bảo vệ bạn.

Đây là nền tảng cho toàn bộ phần Domain-Driven Design ở Part IV: thay vì kiểm tra lỗi bằng `if/else` lúc runtime, ta dùng types để **loại bỏ khả năng sai** ngay lúc compile.

---

## ✅ Checkpoint 1.2

> Ghi nhớ 3 điều:
> 1. **Types = lời hứa** (hợp đồng). `fn(A) -> B` = "đưa A, tôi trả B"
> 2. `Option<T>` = "có thể có, có thể không". `Result<T, E>` = "thành công hoặc lỗi" — cả hai buộc bạn xử lý đầy đủ
> 3. Compiler kiểm tra types = kiểm tra logic → type đúng thì chương trình rất khó sai
>
> **Test nhanh**: `fn process(input: String) -> Result<Output, Error>` hứa gì?
> <details><summary>Đáp án</summary>"Đưa tôi một String, tôi trả lại Output hoặc Error — tôi nói rõ cả hai trường hợp, và bạn phải xử lý cả hai."</details>

---

## 1.3 — Phép cộng và phép nhân... cho Types

Bạn chỉ cần biết **cộng** và **nhân** là hiểu phần này.

### Ẩn dụ: Menu quán cà phê

Bạn vào quán cà phê và thấy menu:

**Combo bữa sáng** (chọn **tất cả**):
- Một món chính **VÀ** một đồ uống **VÀ** một tráng miệng

**Đồ uống riêng** (chọn **một**):
- Trà **HOẶC** cà phê **HOẶC** nước cam

Hai kiểu chọn này tương ứng với hai loại type trong lập trình:
- **VÀ** (chọn tất cả) → **Product type** → `struct` trong Rust
- **HOẶC** (chọn một) → **Sum type** → `enum` trong Rust

### Product Type = struct = VÀ = Phép nhân

Khi bạn kết hợp nhiều thứ bằng **VÀ**, bạn có **product type**. Tại sao gọi là "phép nhân"? Hãy đếm:

```
Món chính: Phở, Bún bò, Cơm tấm     → 3 lựa chọn
Đồ uống:  Trà đá, Cà phê             → 2 lựa chọn
Tráng miệng: Chè, Kem dừa, Flan      → 3 lựa chọn

Tổng số combo = 3 × 2 × 3 = 18 combo
```

Giống hệt phép nhân trong toán!

```rust
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
```

**Quy tắc nhân**: `struct` có bao nhiêu fields → **nhân** số lựa chọn của từng field.

### Sum Type = enum = HOẶC = Phép cộng

Khi bạn chọn **một trong nhiều** lựa chọn, bạn có **sum type**. Tại sao gọi là "phép cộng"? Hãy đếm:

```
Thanh toán bằng:
  Tiền mặt    → 1 cách
  Thẻ         → 1 cách
  Ví điện tử  → 1 cách

Tổng = 1 + 1 + 1 = 3 cách
```

```rust
// filename: src/main.rs

// Sum type: enum = chọn MỘT = phép CỘNG
#[derive(Debug)]
enum Payment {
    Cash(u64),                        // HOẶC
    Card { number: String },          // HOẶC
    EWallet { provider: String },
}

fn describe_payment(payment: &Payment) {
    match payment {
        Payment::Cash(amount) => {
            println!("💵 Cash: {}đ", amount);
        }
        Payment::Card { number } => {
            // Chỉ hiện 4 số cuối
            println!("💳 Card: ****{}", &number[number.len()-4..]);
        }
        Payment::EWallet { provider } => {
            println!("📱 E-Wallet: {}", provider);
        }
    }
}

fn main() {
    describe_payment(&Payment::Cash(50_000));
    describe_payment(&Payment::Card { number: "4111111111111234".to_string() });
    describe_payment(&Payment::EWallet { provider: "MoMo".to_string() });
    // Output:
    // 💵 Cash: 50000đ
    // 💳 Card: ****1234
    // 📱 E-Wallet: MoMo
}
```

**Quy tắc cộng**: `enum` có bao nhiêu variants → **cộng** số trạng thái.

> **💡 Mẹo ghi nhớ cho bạn nhỏ tuổi**:
> - **struct** giống **bộ đồ chơi Lego**: bạn cần TẤT CẢ các miếng ghép (AND) mới ráp được
> - **enum** giống **tiệm kem**: bạn chỉ chọn MỘT vị (OR), không lấy tất cả

### Bảng tóm tắt nhanh

| Loại | Từ khóa Rust | Nghĩa | Phép tính | Ví dụ |
|------|---|---|---|---|
| Product | `struct` | VÀ (tất cả) | Nhân × | 2 bánh × 3 nước = **6** combo |
| Sum | `enum` | HOẶC (một) | Cộng + | 2 bánh + 3 nước = **5** lựa chọn |

---

## ✅ Checkpoint 1.3

> Ghi nhớ:
> 1. `struct` = VÀ → **nhân** số trạng thái
> 2. `enum` = HOẶC → **cộng** số trạng thái
> 3. Chỉ cần biết cộng và nhân là đếm được!
>
> **Test nhanh**: Một `struct` có 2 field kiểu `bool` có bao nhiêu trạng thái?
> <details><summary>Đáp án</summary>2 × 2 = 4 trạng thái. Bool có 2 giá trị, struct nhân chúng lại.</details>

---

## 1.4 — Đếm trạng thái: Vũ khí diệt bug

Đây là phần bạn sẽ dùng nhiều nhất trong cả cuốn sách.

### Bài toán: Đơn hàng quán cà phê

Bạn xây hệ thống quản lý đơn hàng. Một đơn hàng đi qua 4 bước: mới → đang pha → sẵn sàng → khách nhận.

#### ❌ Cách sai: Boolean flags

Cách "tự nhiên" nhất là tạo biến `bool` cho mỗi trạng thái:

```rust
// filename: src/main.rs

// ❌ SAI — dùng 4 cái bool
struct OrderBad {
    is_confirmed: bool,
    is_preparing: bool,
    is_ready: bool,
    is_delivered: bool,
}

fn main() {
    // Mỗi bool có 2 giá trị → 2 × 2 × 2 × 2 = 16 trạng thái
    println!("Total states with 4 bools: {}", 2_u32.pow(4));
    // Output: Total states with 4 bools: 16

    // Nhưng chỉ ~4 trạng thái HỢP LÝ:
    // 1. (true,  false, false, false) — mới xác nhận
    // 2. (true,  true,  false, false) — đang pha
    // 3. (true,  true,  true,  false) — sẵn sàng
    // 4. (true,  true,  true,  true)  — đã giao

    // 16 - 4 = 12 trạng thái VÔ NGHĨA! Ví dụ:
    let nonsense = OrderBad {
        is_confirmed: false,
        is_preparing: false,
        is_ready: false,
        is_delivered: true, // 🤯 Giao cho ai khi chưa ai đặt?!
    };

    println!("is_delivered={} but is_confirmed={} → VÔ LÝ!",
        nonsense.is_delivered, nonsense.is_confirmed);
    // Output: is_delivered=true but is_confirmed=false → VÔ LÝ!
}
```

Tưởng tượng bạn có 16 ô trong bảng, nhưng chỉ 4 ô hợp lệ. 12 ô còn lại là **"bẫy"** — nếu chương trình vô tình rơi vào, bạn có bug. Năm nay không sao, nhưng ai đó vào sửa code năm sau thì... 💥

#### ✅ Cách đúng: Enum

```rust
// filename: src/main.rs

// ✅ ĐÚNG — liệt kê chính xác các trạng thái hợp lệ
#[derive(Debug)]
enum OrderStatus {
    Confirmed,
    Preparing,
    Ready,
    Delivered,
}

#[derive(Debug)]
struct Order {
    id: u32,
    item: String,
    status: OrderStatus,
}

fn advance(order: &mut Order) {
    order.status = match order.status {
        OrderStatus::Confirmed => {
            println!("  ☕ Start preparing #{}", order.id);
            OrderStatus::Preparing
        }
        OrderStatus::Preparing => {
            println!("  ✅ Order #{} ready!", order.id);
            OrderStatus::Ready
        }
        OrderStatus::Ready => {
            println!("  🎉 Customer picked up #{}", order.id);
            OrderStatus::Delivered
        }
        OrderStatus::Delivered => {
            println!("  📌 Order #{} already done!", order.id);
            OrderStatus::Delivered
        }
    };
}

fn main() {
    let mut order = Order {
        id: 1,
        item: "Cà phê sữa đá".to_string(),
        status: OrderStatus::Confirmed,
    };

    // Chỉ 4 trạng thái — TẤT CẢ hợp lệ!
    println!("📝 New order: {:?}", order.status);

    advance(&mut order);  // Confirmed → Preparing
    advance(&mut order);  // Preparing → Ready
    advance(&mut order);  // Ready → Delivered

    // Output:
    // 📝 New order: Confirmed
    //   ☕ Start preparing #1
    //   ✅ Order #1 ready!
    //   🎉 Customer picked up #1
}
```

4 trạng thái, **tất cả đều hợp lệ**. Không thể tạo bug "giao hàng mà chưa xác nhận" — vì trạng thái đó không tồn tại trong type.

#### So sánh hai cách

| | Bool flags (❌) | Enum (✅) |
|---|---|---|
| Tổng trạng thái | 16 | 4 |
| Trạng thái hợp lệ | 4 | 4 |
| Trạng thái thừa = bugs | **12** | **0** |
| Compiler bảo vệ? | Không | **Có** |

> **💡 Nguyên tắc vàng**: Số trạng thái có thể = Số trạng thái hợp lệ → thiết kế tốt. Khác nhau → có bug tiềm ẩn.

### Bảng công thức (chỉ cần cộng và nhân!)

| Type | Số trạng thái | Ví dụ Rust |
|------|--------------|------------|
| `()` (unit) | 1 | Chỉ có 1 giá trị: `()` |
| `bool` | 2 | `true` hoặc `false` |
| Enum N variants | N | `enum Season { Spring, Summer, Fall, Winter }` → 4 |
| Struct (a, b) | a × b | `struct { x: bool, y: bool }` → 2×2 = 4 |
| Enum + data | cộng từng variant | `enum { A(bool), B(bool, bool) }` → 2 + 2×2 = 6 |
| `Option<T>` | \|T\| + 1 | `Option<bool>` → 2 + 1 = 3 |
| `Result<T, E>` | \|T\| + \|E\| | `Result<bool, bool>` → 2 + 2 = 4 |

### Thực hành cùng nhau: Hệ thống xác thực

```rust
// filename: src/main.rs

#[derive(Debug)]
enum BanReason { Spam, Fraud, Harassment }  // 3 lý do

#[derive(Debug)]
enum AuthState {
    Anonymous,                                   // 1 trạng thái
    LoggedIn { user_id: u8, is_admin: bool },    // 256 × 2 = 512
    Banned { reason: BanReason },                // 3
}
// Tổng: 1 + 512 + 3 = 516 trạng thái — TẤT CẢ hợp lệ!

fn greet(state: &AuthState) {
    match state {
        AuthState::Anonymous => {
            println!("👤 Xin chào khách! Hãy đăng nhập.");
        }
        AuthState::LoggedIn { user_id, is_admin } => {
            if *is_admin {
                println!("👑 Chào Admin #{}", user_id);
            } else {
                println!("😊 Chào User #{}", user_id);
            }
        }
        AuthState::Banned { reason } => {
            println!("🚫 Tài khoản bị cấm: {:?}", reason);
        }
    }
}

fn main() {
    println!("Total states: 1 + 512 + 3 = {}", 1 + 512 + 3);

    let users = vec![
        AuthState::Anonymous,
        AuthState::LoggedIn { user_id: 42, is_admin: false },
        AuthState::LoggedIn { user_id: 1, is_admin: true },
        AuthState::Banned { reason: BanReason::Spam },
    ];

    for user in &users {
        greet(user);
    }
    // Output:
    // Total states: 1 + 512 + 3 = 516
    // 👤 Xin chào khách! Hãy đăng nhập.
    // 😊 Chào User #42
    // 👑 Chào Admin #1
    // 🚫 Tài khoản bị cấm: Spam
}
```

Không thể tạo user vừa `Anonymous` vừa `LoggedIn`. Không thể tạo user `Banned` nhưng vẫn có `is_admin`. Compiler ngăn chặn hết.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Đếm trạng thái — chỉ cần cộng và nhân

```rust
// a) struct có 2 field bool
struct Point { x: bool, y: bool }

// b) enum 3 variants
enum TrafficLight { Red, Yellow, Green }

// c) struct chứa 2 enum
struct Intersection { north: TrafficLight, east: TrafficLight }

// d) enum với data
enum Shape {
    Circle(bool),           // có tô màu?
    Rectangle(bool, bool),  // tô màu?, có viền?
}
```

<details><summary>✅ Lời giải Bài 1</summary>

```
a) struct (nhân): 2 × 2 = 4
b) enum (cộng): 3
c) struct (nhân): 3 × 3 = 9
d) enum (cộng): Circle có 2 + Rectangle có 2×2=4 → 2 + 4 = 6
```

</details>

---

**Bài 2** (10 phút): Sửa thiết kế sai — chuyển bool flags → enum

```rust
// ❌ Có bao nhiêu trạng thái? Bao nhiêu hợp lệ?
struct UserAccount {
    is_active: bool,
    is_verified: bool,
    is_suspended: bool,
}
```

<details><summary>💡 Gợi ý</summary>Lifecycle tài khoản: đăng ký → xác minh → hoạt động → (có thể bị khóa). Dùng enum cho lifecycle.</details>

<details><summary>✅ Lời giải Bài 2</summary>

```rust
// Cũ: 2 × 2 × 2 = 8 trạng thái, hợp lệ chỉ ~4
// → 4 trạng thái thừa = bugs tiềm ẩn

// ✅ Mới: chính xác 4 trạng thái, tất cả hợp lệ
enum AccountStatus {
    Registered,                     // chưa xác minh
    Verified,                       // đã xác minh email
    Active,                         // tài khoản đang hoạt động
    Suspended { reason: String },   // bị tạm khóa (kèm lý do)
}
```

</details>

---

**Bài 3** (15 phút): Thiết kế domain model cho quán cà phê

Thiết kế types cho quy trình gọi đồ uống:
- 3 loại: cà phê, trà, sinh tố
- 3 sizes: S, M, L
- 3 toppings: không, trân châu, thạch
- 4 trạng thái: mới → đang pha → sẵn sàng → khách nhận

Yêu cầu: (1) Viết types, (2) Tính tổng combo, (3) Viết function mô tả đơn hàng bằng `match`.

<details><summary>💡 Gợi ý</summary>

- DrinkType, Size, Topping → enum (chọn MỘT = sum type)
- Đơn hàng = DrinkType + Size + Topping + Status → struct (cần TẤT CẢ = product type)
- Tổng combo đồ uống = 3 × 3 × 3 = ?

</details>

<details><summary>✅ Lời giải Bài 3</summary>

```rust
// filename: src/main.rs
#[derive(Debug)]
enum DrinkType { Coffee, Tea, Smoothie }

#[derive(Debug)]
enum Size { S, M, L }

#[derive(Debug)]
enum Topping { None, BubblePearl, Jelly }

#[derive(Debug)]
enum Status { New, Preparing, Ready, PickedUp }

#[derive(Debug)]
struct CafeOrder {
    drink: DrinkType,
    size: Size,
    topping: Topping,
    status: Status,
}

fn describe(order: &CafeOrder) -> String {
    let name = match order.drink {
        DrinkType::Coffee => "Cà phê",
        DrinkType::Tea => "Trà",
        DrinkType::Smoothie => "Sinh tố",
    };
    let size = match order.size {
        Size::S => "nhỏ", Size::M => "vừa", Size::L => "lớn",
    };
    let topping = match order.topping {
        Topping::None => "".to_string(),
        Topping::BubblePearl => " + trân châu".to_string(),
        Topping::Jelly => " + thạch".to_string(),
    };
    let icon = match order.status {
        Status::New => "📝",
        Status::Preparing => "☕",
        Status::Ready => "✅",
        Status::PickedUp => "🎉",
    };
    format!("{} {} size {}{}", icon, name, size, topping)
}

fn main() {
    // Tổng combo: 3 × 3 × 3 = 27 loại đồ uống
    // Kể cả status: 27 × 4 = 108 trạng thái
    println!("Total drink combos: {}", 3 * 3 * 3);
    println!("Total states: {}", 3 * 3 * 3 * 4);

    let order = CafeOrder {
        drink: DrinkType::Coffee,
        size: Size::M,
        topping: Topping::BubblePearl,
        status: Status::Preparing,
    };
    println!("{}", describe(&order));
    // Output:
    // Total drink combos: 27
    // Total states: 108
    // ☕ Cà phê size vừa + trân châu
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `error[E0004]: non-exhaustive patterns` | Quên xử lý một nhánh trong `match` | Thêm nhánh còn thiếu, hoặc dùng `_ => ...` |
| `error[E0308]: mismatched types` | Type không khớp (truyền `i32` cho `bool`) | Kiểm tra type trong signature function |
| Tính sai số trạng thái | Nhầm cộng và nhân | `struct` → **nhân**, `enum` → **cộng** |
| Không biết dùng struct hay enum | Chưa rõ AND hay OR | Hỏi: "Cần **tất cả** → struct. Chọn **một** → enum" |

---

## Tóm tắt

- ✅ **Lambda Calculus**: Function = "cái máy xay". Closure `|x| x + 1` = `λx. x+1` = "máy cộng 1". Gọi function = bỏ nguyên liệu vào máy (β-reduction).
- ✅ **Curry-Howard**: Types = lời hứa (hợp đồng). Compiler = thầy giáo kiểm tra bài. Type đúng → chương trình rất khó sai.
- ✅ **Product types** (`struct`) = VÀ → **nhân**. **Sum types** (`enum`) = HOẶC → **cộng**.
- ✅ **Đếm trạng thái** = vũ khí diệt bug. Nếu tổng trạng thái > trạng thái hợp lệ → thiết kế cần sửa. Dùng `enum` thay boolean flags.

## Tiếp theo

→ Chapter 2: **Algorithmic Thinking & Complexity** — bạn sẽ học Big-O (tại sao code này chạy nhanh hơn code kia), recursion (function gọi chính nó), và tail recursion (cách tối ưu recursion). Đây là nền tảng để chọn đúng data structures ở Chapter 3.
