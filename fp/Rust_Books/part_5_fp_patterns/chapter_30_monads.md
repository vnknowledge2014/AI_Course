# Chapter 30 — Monads in Rust — The Practical View

> **Bạn sẽ học được**:
> - **Monad** = Functor + `and_then` (flat_map / bind)
> - `Option` + `and_then` = **Maybe Monad**
> - `Result` + `?` = **Either Monad**
> - `Iterator` + `flat_map` = **List Monad**
> - Bạn đã dùng monads **hàng ngày** — chapter này chỉ đặt tên cho chúng
> - So sánh với Haskell `do`-notation
>
> **Yêu cầu trước**: Chapter 29 (Functors), Chapter 10 (Error Handling).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: "Monad" không còn là từ đáng sợ — nó chỉ là `.and_then()`.

---

## 30.1 — Từ Functor đến Monad

### Hai lớp giấy gói

Nhớ Chapter 29? Functor là hộp quà và `.map()` là thay đồ bên trong hộp. Nhưng nếu function của bạn **cũng trả về hộp**? Bạn sẽ có hộp trong hộp — quà bị gói hai lớn giấy. Người nhận mửng rỡ, mở ra... thấy thêm một lớp gói. Phải mở tiếp. Và nếu mỗi bước trong pipeline đều gói thêm một lớp? `Option<Option<Option<T>>>` — 3 lớp giấy, đau đầu.

Monad giải quyết vấn đề này: `.and_then()` là **gỡ giấy cũ trước khi gói mới**. Kết quả luôn là 1 lớp, không bao giờ lồng.

Nếu bạn đã dùng `.and_then()` ở Chapter 23-24, bạn đã dùng Monad rồi. Chapter này chỉ đặt tên cho thứ bạn đã biết.

Vấn đề xuất phát từ `.map()`: nếu function bên trong trả về **một container** (Option, Result), bạn sẽ bị **lồng 2 lớp** — `Option<Option<T>>`. Không ai muốn vậy.

### Functor: `.map()` — không đủ!

```rust
// filename: src/main.rs

fn parse_int(s: &str) -> Option<i32> {
    s.parse().ok()
}

fn main() {
    let input: Option<&str> = Some("42");

    // Dùng .map() → KẾT QUẢ BỊ NESTED!
    let nested: Option<Option<i32>> = input.map(|s| parse_int(s));
    println!("Nested: {:?}", nested);  // Some(Some(42)) ← 2 lớp Option!

    // Muốn: Option<i32>, không phải Option<Option<i32>>
    // Giải pháp: .and_then() = map + flatten
    let flat: Option<i32> = input.and_then(|s| parse_int(s));
    println!("Flat: {:?}", flat);  // Some(42) ← 1 lớp!
}
```

### Vấn đề và giải pháp

```
Functor (.map):     f: A → B        Result: F<B>
                    Option<A>.map(f)  → Option<B>           ✅ OK

Khi f trả container:  f: A → F<B>   Result: F<F<B>>  ← NESTED!
                    Option<A>.map(f)  → Option<Option<B>>   ❌ Oops

Monad (.and_then): f: A → F<B>      Result: F<B>
                    Option<A>.and_then(f)  → Option<B>      ✅ Flattened!
```

---

## 30.2 — Option = Maybe Monad

```rust
// filename: src/main.rs

use std::collections::HashMap;

fn find_user(id: u64) -> Option<String> {
    let users: HashMap<u64, &str> = [(1, "Minh"), (2, "Lan")].into();
    users.get(&id).map(|s| s.to_string())
}

fn find_email(name: &str) -> Option<String> {
    match name {
        "Minh" => Some("minh@co.com".into()),
        "Lan" => Some("lan@co.com".into()),
        _ => None,
    }
}

fn find_domain(email: &str) -> Option<String> {
    email.split('@').nth(1).map(String::from)
}

fn main() {
    // Chaining với and_then — mỗi step có thể fail (None)
    let domain = find_user(1)
        .and_then(|name| find_email(&name))
        .and_then(|email| find_domain(&email));
    println!("User 1 domain: {:?}", domain);  // Some("co.com")

    // Fail ở giữa → tất cả sau skip
    let domain = find_user(99)                  // None!
        .and_then(|name| find_email(&name))     // skipped
        .and_then(|email| find_domain(&email)); // skipped
    println!("User 99 domain: {:?}", domain);  // None

    // Mix map và and_then
    let greeting = find_user(2)
        .and_then(|name| find_email(&name))    // Option<String>
        .map(|email| format!("Hello! Your email: {}", email));  // transform
    println!("Greeting: {:?}", greeting);
}
```

> **Maybe Monad** = "chaining operations that might fail (return None), automatically stopping at the first None."

---

## 30.3 — Result = Either Monad

Tương tự Option, `Result` cũng là Monad. `.and_then()` chain nhiều steps có thể fail — dừng ở lỗi đầu tiên, giữ lại thông tin lỗi (không như Option mất tích với None).

```rust
// filename: src/main.rs

#[derive(Debug)]
enum AppError {
    Parse(String),
    Validation(String),
    Database(String),
}

fn parse_config(input: &str) -> Result<(String, u16), AppError> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        return Err(AppError::Parse("Expected host:port".into()));
    }
    let port = parts[1].parse::<u16>()
        .map_err(|_| AppError::Parse(format!("Invalid port: {}", parts[1])))?;
    Ok((parts[0].to_string(), port))
}

fn validate_config(host: &str, port: u16) -> Result<(String, u16), AppError> {
    if host.is_empty() {
        Err(AppError::Validation("Host empty".into()))
    } else if port < 1024 {
        Err(AppError::Validation(format!("Port {} < 1024 (privileged)", port)))
    } else {
        Ok((host.to_string(), port))
    }
}

fn connect(host: &str, port: u16) -> Result<String, AppError> {
    if host == "localhost" {
        Ok(format!("Connected to {}:{}", host, port))
    } else {
        Err(AppError::Database(format!("Cannot reach {}:{}", host, port)))
    }
}

fn main() {
    // and_then chain = Either Monad
    let result = parse_config("localhost:8080")
        .and_then(|(host, port)| validate_config(&host, port))
        .and_then(|(host, port)| connect(&host, port));
    println!("OK: {:?}", result);

    // ? operator = and_then sugar
    fn connect_from(input: &str) -> Result<String, AppError> {
        let (host, port) = parse_config(input)?;           // and_then
        let (host, port) = validate_config(&host, port)?;  // and_then
        let conn = connect(&host, port)?;                   // and_then
        Ok(conn)
    }

    println!("?:  {:?}", connect_from("localhost:8080"));
    println!("Err: {:?}", connect_from("remote:80"));
}
```

### `?` = Monadic bind (do-notation)

```
Haskell do-notation:        Rust ? operator:
do                          fn pipeline() -> Result<C, E> {
  a <- getA                    let a = get_a()?;
  b <- process a               let b = process(a)?;
  c <- finalize b              let c = finalize(b)?;
  return c                     Ok(c)
                            }
```

> **💡 Insight**: `?` operator IS Rust's do-notation for the Result/Option monad. Bạn đã viết monadic code hàng ngày!

---

## ✅ Checkpoint 30.3

> Ghi nhớ:
> 1. **Functor** = `.map(f: A → B)` — transform inside container
> 2. **Monad** = `.and_then(f: A → F<B>)` — chain operations that return containers
> 3. `.map()` khi `f` returns plain value. `.and_then()` khi `f` returns `Option`/`Result`
> 4. `?` = syntactic sugar cho `.and_then()` + early return

---

## 30.4 — Iterator = List Monad

Monad không chỉ dùng cho error handling. `.flat_map()` trên Iterator cũng là Monad — mỗi phần tử sinh ra nhiều phần tử, `flat_map` gộp tất cả lại thành một stream duy nhất.

```rust
// filename: src/main.rs

fn main() {
    // .map() = Functor: transform each element
    let nums = vec![1, 2, 3];
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("map: {:?}", doubled);  // [2, 4, 6]

    // .flat_map() = Monad: each element → multiple elements
    let expanded: Vec<i32> = nums.iter()
        .flat_map(|&x| vec![x, x * 10, x * 100])
        .collect();
    println!("flat_map: {:?}", expanded);  // [1, 10, 100, 2, 20, 200, 3, 30, 300]

    // Practical: generate combinations
    let colors = vec!["Red", "Blue"];
    let sizes = vec!["S", "M", "L"];

    // Cartesian product (List Monad!)
    let combos: Vec<String> = colors.iter()
        .flat_map(|&color| {
            sizes.iter().map(move |&size| format!("{}-{}", color, size))
        })
        .collect();
    println!("Combos: {:?}", combos);
    // ["Red-S", "Red-M", "Red-L", "Blue-S", "Blue-M", "Blue-L"]

    // flat_map with filtering
    fn divisors(n: u32) -> Vec<u32> {
        (1..=n).filter(|&d| n % d == 0).collect()
    }

    let all_divisors: Vec<(u32, u32)> = vec![6, 10, 15].iter()
        .flat_map(|&n| divisors(n).into_iter().map(move |d| (n, d)))
        .collect();
    println!("Divisors: {:?}", all_divisors);
}
```

---

## 30.5 — Monad Laws

Giống Functor Laws ở Chapter 29, Monad có 3 quy tắc đảm bảo `.and_then()` hoạt động nhất quán. Bạn không cần thuộc lòng, nhưng hiểu chúng giúp debug khi code không hoạt động như mong đợi.


```rust
// filename: src/main.rs

fn main() {
    // Setup
    let f = |x: i32| if x > 0 { Some(x * 2) } else { None };
    let g = |x: i32| Some(x + 10);

    // ═══ Law 1: Left Identity ═══
    // return(a).and_then(f) == f(a)
    let a = 5;
    assert_eq!(Some(a).and_then(f), f(a));  // ✅
    println!("Law 1 (Left Identity): {:?} == {:?}", Some(a).and_then(f), f(a));

    // ═══ Law 2: Right Identity ═══
    // m.and_then(return) == m
    let m = Some(42);
    assert_eq!(m.and_then(Some), m);  // ✅
    println!("Law 2 (Right Identity): {:?} == {:?}", m.and_then(Some), m);

    // ═══ Law 3: Associativity ═══
    // m.and_then(f).and_then(g) == m.and_then(|x| f(x).and_then(g))
    let m = Some(5);
    let left = m.and_then(f).and_then(g);
    let right = m.and_then(|x| f(x).and_then(g));
    assert_eq!(left, right);  // ✅
    println!("Law 3 (Associativity): {:?} == {:?}", left, right);
}
```

### Bảng tóm tắt laws

| Law | Ý nghĩa | Code |
|-----|---------|------|
| **Left Identity** | Wrap rồi bind = gọi trực tiếp | `Some(a).and_then(f) == f(a)` |
| **Right Identity** | Bind với wrap = không đổi | `m.and_then(Some) == m` |
| **Associativity** | Thứ tự grouping không quan trọng | `m.and_then(f).and_then(g) == m.and_then(\|x\| f(x).and_then(g))` |

---

## 30.6 — Bảng So Sánh: Functor vs Monad

| | **Functor** | **Monad** |
|---|---|---|
| **Operation** | `.map(f)` | `.and_then(f)` / `.flat_map(f)` |
| **Function type** | `f: A → B` | `f: A → F<B>` |
| **Result** | `F<B>` | `F<B>` (flattened) |
| **Nesting?** | Nếu f trả container → nested | Tự flatten |
| **Option** | `map` | `and_then` |
| **Result** | `map` | `and_then`, `?` |
| **Iterator** | `map` | `flat_map` |
| **Ý nghĩa** | Transform value | Chain computations |

---

## 30.7 — Practical Patterns

### Pattern 1: Option chaining cho config

```rust
// filename: src/main.rs

use std::collections::HashMap;

fn get_database_url(config: &HashMap<String, String>) -> Option<String> {
    config.get("database")
        .and_then(|db_section| {
            // In real app: parse db section
            if db_section.contains("postgres") { Some(db_section.clone()) }
            else { None }
        })
        .map(|url| format!("postgresql://{}", url))
}

fn main() {
    let mut config = HashMap::new();
    config.insert("database".into(), "postgres://localhost:5432/mydb".into());
    println!("{:?}", get_database_url(&config));

    config.insert("database".into(), "sqlite://local.db".into());
    println!("{:?}", get_database_url(&config));
}
```

### Pattern 2: Result chain cho API

```rust
// filename: src/main.rs

fn api_pipeline(raw_body: &str) -> Result<String, String> {
    parse_json(raw_body)
        .and_then(validate_fields)
        .and_then(process_order)
        .map(format_response)
}

fn parse_json(body: &str) -> Result<Vec<(&str, &str)>, String> {
    if body.starts_with('{') {
        Ok(vec![("name", "Coffee"), ("qty", "2")])
    } else {
        Err("Invalid JSON".into())
    }
}

fn validate_fields(fields: Vec<(&str, &str)>) -> Result<(String, u32), String> {
    let name = fields.iter().find(|(k, _)| *k == "name")
        .map(|(_, v)| v.to_string())
        .ok_or("Missing 'name'")?;
    let qty = fields.iter().find(|(k, _)| *k == "qty")
        .and_then(|(_, v)| v.parse().ok())
        .ok_or("Missing/invalid 'qty'")?;
    Ok((name, qty))
}

fn process_order((name, qty): (String, u32)) -> Result<String, String> {
    if qty == 0 { Err("Quantity must be > 0".into()) }
    else { Ok(format!("Order: {} x{}", name, qty)) }
}

fn format_response(order: String) -> String {
    format!("{{\"status\": \"ok\", \"order\": \"{}\"}}", order)
}

fn main() {
    println!("{:?}", api_pipeline("{body}"));
    println!("{:?}", api_pipeline("not json"));
}
```

### Pattern 3: flat_map cho data transformation

```rust
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
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): map hay and_then?

Cho mỗi case, chọn `.map()` hay `.and_then()`:

```rust
let x: Option<i32> = Some(5);
// a) x.???(|n| n + 1)           → Option<i32>
// b) x.???(|n| Some(n + 1))     → Option<i32>
// c) x.???(|n| n.to_string())   → Option<String>
// d) x.???(|n| if n > 0 { Some(n) } else { None })  → Option<i32>
```

<details><summary>✅ Lời giải</summary>

```
a) .map()      — f returns i32 (plain value)
b) .and_then() — f returns Option<i32> (container)
c) .map()      — f returns String (plain value)
d) .and_then() — f returns Option<i32> (container)
```

</details>

---

**Bài 2** (10 phút): Monad law verification

Verify cả 3 laws cho `Result<i32, String>`:
- `f(x) = if x > 0 then Ok(x*2) else Err("negative")`
- `g(x) = Ok(x + 100)`

<details><summary>✅ Lời giải Bài 2</summary>

```rust
fn f(x: i32) -> Result<i32, String> {
    if x > 0 { Ok(x * 2) } else { Err("negative".into()) }
}
fn g(x: i32) -> Result<i32, String> { Ok(x + 100) }

fn main() {
    let a = 5;
    // Law 1: Left Identity
    assert_eq!(Ok::<_, String>(a).and_then(f), f(a));

    // Law 2: Right Identity
    let m: Result<i32, String> = Ok(42);
    assert_eq!(m.and_then(Ok), m);

    // Law 3: Associativity
    let m: Result<i32, String> = Ok(5);
    assert_eq!(m.and_then(f).and_then(g), m.and_then(|x| f(x).and_then(g)));
}
```

</details>

---

**Bài 3** (15 phút): Practical monad chain

Viết "User lookup pipeline" dùng monadic chaining:
1. `find_user(id) -> Option<User>`
2. `find_address(user) -> Option<Address>`
3. `find_delivery_zone(address) -> Option<Zone>`
4. `calculate_shipping(zone) -> Option<Money>`

Chain 4 steps. Test happy path và failure ở mỗi step.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
use std::collections::HashMap;

struct User { name: String, address_id: Option<u64> }
struct Address { city: String, zip: String }
struct Zone { name: String, rate: u32 }

fn find_user(id: u64) -> Option<User> {
    match id {
        1 => Some(User { name: "Minh".into(), address_id: Some(10) }),
        2 => Some(User { name: "Lan".into(), address_id: None }),
        _ => None,
    }
}

fn find_address(user: &User) -> Option<Address> {
    user.address_id.and_then(|id| match id {
        10 => Some(Address { city: "HCM".into(), zip: "70000".into() }),
        _ => None,
    })
}

fn find_zone(addr: &Address) -> Option<Zone> {
    match addr.city.as_str() {
        "HCM" => Some(Zone { name: "Zone A".into(), rate: 15_000 }),
        "HN" => Some(Zone { name: "Zone B".into(), rate: 25_000 }),
        _ => None,
    }
}

fn shipping_cost(zone: &Zone) -> u32 { zone.rate }

fn get_shipping(user_id: u64) -> Option<u32> {
    find_user(user_id)
        .and_then(|user| find_address(&user))
        .and_then(|addr| find_zone(&addr))
        .map(|zone| shipping_cost(&zone))
}

fn main() {
    println!("User 1: {:?}đ", get_shipping(1)); // Some(15000)
    println!("User 2: {:?}đ", get_shipping(2)); // None (no address)
    println!("User 9: {:?}đ", get_shipping(9)); // None (no user)
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| `Option<Option<T>>` nested | Dùng `.map()` khi f trả Option | Đổi sang `.and_then()` |
| "Monad quá abstract" | Lý thuyết | Nhớ: monad = `.and_then()`. Bạn đã dùng rồi! |
| "Rust không có Monad trait" | HKT limitation (như Functor) | Dùng `.and_then()` trực tiếp, không cần trait |
| `?` vs `and_then` | Khi nào dùng gì? | `?` cho Result/Option in functions. `and_then` cho inline chains |

---

## Tóm tắt

Chapter này gỡ lớp giấy bí ẩn khỏi từ "Monad" — và bên trong chỉ là `.and_then()` mà bạn đã biết:

- ✅ **Monad** = Functor + `.and_then()`. "Chain computations that return containers" — gỡ giấy cũ trước khi gói mới.
- ✅ **Option** monad: `.and_then()` = "nếu Some → chạy tiếp, nếu None → dừng."
- ✅ **Result** monad: `.and_then()` / `?` = "nếu Ok → chạy tiếp, nếu Err → return error."
- ✅ **Iterator** monad: `.flat_map()` = "mỗi phần tử → nhiều phần tử, flatten kết quả."
- ✅ **`?` = do-notation**: Rust's syntactic sugar cho monadic bind.
- ✅ **Practical rule**: `.map()` khi `f: A → B`. `.and_then()` khi `f: A → F<B>`.

## Tiếp theo

Bạn đã biết Functors và Monads — giờ hãy **áp dụng chúng** vào một bài toán thực tế: viết parser. Parser Combinators là nơi Functors và Monads **tỏa sáng** — compose functions nhỏ thành parser lớn.

→ Chapter 31: **Parser Combinators** — bạn sẽ build parsers từ nhỏ lên lớn bằng combinators: `tag`, `alt`, `many0`, `map`, `pair`. Functors + Monads trong thực tế!
