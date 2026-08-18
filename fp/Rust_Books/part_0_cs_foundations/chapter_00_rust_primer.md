# Chapter 0 — Rust in 10 Minutes: Khởi động hành trang lập trình

> **Bạn sẽ học được**:
> - Đủ cú pháp Rust để **đọc hiểu** mọi ví dụ trong sách
> - `let`, `fn`, kiểu vô hướng, `if`/`match`, closure, `Vec`
> - `Option<T>` và `Result<T, E>` ở mức nhận diện
> - **Không** học ownership ở đây — để dành cho Chapter 9
>
> **Yêu cầu trước**: không có. Đây là chương đầu tiên.
> **Thời gian đọc**: ~25 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Đọc được code Rust trong sách mà không phải tra cứu liên tục.

> **Mục đích**: Chào mừng bạn bước vào thế giới của Rust. Bài viết này đóng vai trò như một "phòng chờ" trước chuyến bay, nơi tôi sẽ trang bị cho bạn đủ lượng cú pháp Rust cơ bản để bạn có thể **đọc hiểu** các ví dụ mã nguồn trong Part 0 (Chapters 1–3) mà không cảm thấy bỡ ngỡ. Đây không phải là một cuốn bách khoa toàn thư về Rust — ở Part I, chúng ta sẽ cùng nhau mổ xẻ mọi thứ một cách chi tiết và sâu sắc hơn.
>
> **Yêu cầu trước**: Không có rào cản nào cả. Việc bạn đã từng làm quen với một ngôn ngữ lập trình nào đó sẽ là một lợi thế tuyệt vời, nhưng nếu đây là ngôn ngữ đầu tiên, đừng lo lắng, chúng ta sẽ đi cùng nhau.
> **Thời gian đọc**: ~10–15 phút | **Level**: Nhập môn (Intro)
> **Kết quả cuối cùng**: Sau khi đọc xong, bạn có thể tự tin lướt qua mọi đoạn code trong Part 0 mà không bị "kẹt" lại ở các chi tiết cú pháp lạ lẫm.

---

## 0.1 — Toàn cảnh Rust: Tại sao lại là 10 phút?

Hãy tưởng tượng bài viết này như một tấm bản đồ du lịch. Nó dành cho hai nhóm độc giả: những nhà phát triển (developers) muốn có cái nhìn chớp nhoáng về Rust trước khi thực sự dấn thân vào hành trình hơn 40 chương sắp tới, và những người đọc muốn có một nơi chốn để quay lại ôn tập (refresh) cú pháp sau khi đã đi được một chặng đường dài.

Ở mỗi phần dưới đây, tôi sẽ trình bày một vài dòng code cốt lõi đi kèm với những lời giải thích ngắn gọn, súc tích. Bạn sẽ còn gặp lại tất cả những khái niệm này, với những phân tích sâu sắc hơn rất nhiều, ở các chương sau. Giờ thì, hãy bắt đầu bằng việc chuẩn bị công cụ!

### Cài đặt & Tạo dự án đầu tay

Để bắt đầu, hãy mở terminal (cửa sổ dòng lệnh) của bạn và chạy dòng lệnh sau. Đây là cách chính thức và an toàn nhất để đưa Rust vào máy tính của bạn:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Sau khi quá trình cài đặt hoàn tất, hãy kiểm tra xem mọi thứ đã sẵn sàng chưa bằng cách gõ:

```bash
rustc --version
# Output (ví dụ): rustc 1.84.0 (9fc6b43 2025-01-07)

cargo --version
# Output (ví dụ): cargo 1.84.0 (fb7f12831 2024-12-18)
```

Ở đây, `rustc` đóng vai trò là trình biên dịch (compiler) thầm lặng phía sau. Tuy nhiên, trong 99% thời gian làm việc với Rust, người bạn đồng hành thực sự của bạn sẽ là `cargo` — công cụ quản lý dự án, thư viện và kiêm luôn cả việc build (xây dựng) chương trình.

### Khởi tạo một dự án mới

Hãy tạo một thư mục mới cho dự án đầu tiên của chúng ta:

```bash
cargo new hello_rust
cd hello_rust
```

Lúc này, nếu nhìn vào thư mục `hello_rust`, bạn sẽ thấy một cấu trúc rất gọn gàng:

```
hello_rust/
├── Cargo.toml    # File cấu hình dự án (tương tự package.json trong Node.js)
└── src/
    └── main.rs   # Nơi chứa mã nguồn chính của bạn
```

Hãy thử chạy chương trình mặc định xem sao:

```bash
cargo run
# Output: Hello, world!
```

Lệnh `cargo run` thực chất làm hai việc cùng lúc: biên dịch mã nguồn và chạy file thực thi. Nếu bạn chỉ muốn biên dịch mà không chạy, hãy dùng `cargo build`. Thú vị hơn, nếu bạn chỉ muốn nhờ trình biên dịch "kiểm tra lỗi giúp tôi" mà không cần tạo ra file thực thi (tiết kiệm thời gian), hãy dùng lệnh `cargo check`.

---

## 0.2 — Câu chuyện của Biến và Kiểu dữ liệu

Trong bất kỳ ngôn ngữ nào, việc lưu trữ trạng thái là điều đầu tiên cần làm quen. Hãy xem Rust xử lý biến như thế nào qua từ khóa `let`:

```rust
fn main() {
    let x = 5;           // Rust rất thông minh, nó tự suy luận x là kiểu i32 (số nguyên 32-bit)
    let y: f64 = 3.14;   // Bạn cũng có thể ghi rõ kiểu nếu muốn (ở đây là số thực 64-bit)
    let name = "Rust";    // &str — một "lát cắt" chuỗi (string slice)
    let active = true;    // kiểu boolean (đúng/sai)

    println!("x = {}, y = {}, name = {}, active = {}", x, y, name, active);
    // Output: x = 5, y = 3.14, name = Rust, active = true
}
```

Có một điều vô cùng đặc biệt và quan trọng ở Rust: **biến mặc định là bất biến (immutable)**. Điều này có nghĩa là một khi bạn đã gán giá trị cho nó, bạn không thể thay đổi được nữa. Nếu bạn thực sự cần thay đổi, bạn phải xin phép một cách rõ ràng bằng từ khóa `mut`:

```rust
fn main() {
    let x = 5;
    // x = 10;  // ❌ Lỗi! Trình biên dịch sẽ ngăn cản bạn thay đổi một biến immutable

    let mut y = 5;
    y = 10;      // ✅ OK — vì bạn đã khai báo `mut`, Rust hiểu rằng biến này có thể thay đổi
    println!("y = {}", y);
    // Output: y = 10
}
```

> **💡 Tại sao lại là immutable mặc định?** 
> Quyết định thiết kế này giúp mã nguồn an toàn hơn rất nhiều. Khi một biến không thay đổi, bạn hoàn toàn yên tâm truyền nó đi khắp nơi mà không sợ "ai đó ở dòng 200 âm thầm thay đổi giá trị của nó". Đây chính là nền tảng của tư duy Lập trình Hàm (FP) mà chúng ta sẽ khám phá sâu sắc ở Part II.

### Điểm danh các kiểu dữ liệu cơ bản

Để tiện tra cứu, đây là các kiểu dữ liệu bạn sẽ gặp thường xuyên:

| Kiểu | Ý nghĩa | Ví dụ |
|------|---------|-------|
| `i32` | Số nguyên 32-bit (thường dùng nhất) | `42`, `-7` |
| `f64` | Số thực 64-bit (chính xác cao) | `3.14`, `-0.5` |
| `bool` | Đúng/Sai (True/False) | `true`, `false` |
| `char` | Một ký tự Unicode (hỗ trợ cả emoji) | `'a'`, `'🦀'` |
| `&str` | Chuỗi tham chiếu (mượn đọc) | `"hello"` |
| `String` | Chuỗi sở hữu (có quyền chỉnh sửa) | `String::from("hello")` |

Nếu bạn đang thắc mắc sự khác nhau giữa `&str` và `String`, đừng vội bận tâm. Tôi sẽ giải thích tường tận điều đó ở Chapter 9 (Ownership). Tạm thời, bạn chỉ cần nhớ: `"hello"` là `&str`, còn `String::from("hello")` hay `"hello".to_string()` là `String`.

---

## 0.3 — Functions (Hàm): Trái tim của chương trình

Khai báo một hàm trong Rust khá trực quan. Bạn dùng từ khóa `fn`, theo sau là tên hàm, các tham số và kiểu trả về (nếu có).

```rust
// Một hàm nhận hai tham số i32 và trả về một i32
fn add(a: i32, b: i32) -> i32 {
    a + b  // Chú ý: Dòng cuối KHÔNG có dấu chấm phẩy (;). Điều này mang ý nghĩa trả về (return) tự động.
}

fn greet(name: &str) {
    // Không có ký hiệu `-> ...` nghĩa là hàm này không trả về giá trị thực sự nào (trả về kiểu unit `()`)
    println!("Hello, {}!", name);
}

fn main() {
    let sum = add(3, 5);
    println!("3 + 5 = {}", sum);
    // Output: 3 + 5 = 8

    greet("Rust");
    // Output: Hello, Rust!
}
```

Có một "cú vấp" rất phổ biến ở những người mới học Rust: **Dòng cuối cùng của hàm dùng để trả về giá trị sẽ không có dấu `;`**. Nếu bạn vô tình thêm `;`, biểu thức đó biến thành một câu lệnh, và hàm sẽ trả về `()` (không có gì), dẫn đến lỗi kiểu (type mismatch).

```rust
fn double(x: i32) -> i32 {
    x * 2     // ✅ Đúng chuẩn: biểu thức này sẽ được trả về
    // x * 2; // ❌ Lỗi: Thêm dấu ; làm nó biến thành lệnh, hàm không tìm thấy giá trị trả về
}
```

---

## 0.4 — Nghệ thuật của `if/else`: Mọi thứ đều là Biểu thức

Không giống như nhiều ngôn ngữ khác coi `if/else` chỉ là công cụ điều hướng luồng (statement), trong Rust, `if/else` là một **biểu thức (expression)**. Nghĩa là nó có khả năng tự đánh giá và trả về một giá trị:

```rust
fn main() {
    let score = 85;

    // Thay vì tạo biến `grade` rồi dùng if/else để gán lại, ta gán thẳng kết quả của if/else vào biến
    let grade = if score >= 90 {
        "A"
    } else if score >= 70 {
        "B"
    } else {
        "C"
    };

    println!("Score {}: grade {}", score, grade);
    // Output: Score 85: grade B
}
```

> **💡 Ghi nhớ**: Hầu hết mọi cấu trúc trong Rust đều là **expression** (sinh ra giá trị) thay vì statement (chỉ thực thi lệnh). Lối tư duy này sẽ giúp mã nguồn của bạn ngắn gọn, mạch lạc và mang đậm chất Functional Programming.

---

## 0.5 — `Enum`: Sự lựa chọn thanh lịch

Khi bạn muốn biểu diễn một dữ liệu chỉ có thể mang **một trong số** các trạng thái nhất định, `enum` là công cụ hoàn hảo.

```rust
// Ví dụ kinh điển: Đèn giao thông chỉ có thể là Đỏ, Vàng, hoặc Xanh
#[derive(Debug)] // Dòng này báo cho Rust biết hãy tự động tạo code để có thể in struct/enum này ra màn hình
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light = TrafficLight::Red;
    println!("Light: {:?}", light);
    // Output: Light: Red
}
```

Điều làm nên sức mạnh thực sự của `enum` trong Rust là khả năng **đính kèm dữ liệu** vào từng biến thể (variant). Nó vượt xa những enum truyền thống mà bạn từng biết:

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),                           // Hình tròn lưu giữ bán kính
    Rectangle { width: f64, height: f64 }, // Hình chữ nhật cần cả chiều rộng và cao
}

fn main() {
    let c = Shape::Circle(5.0);
    let r = Shape::Rectangle { width: 3.0, height: 4.0 };
    
    println!("{:?}", c);  // Output: Circle(5.0)
    println!("{:?}", r);  // Output: Rectangle { width: 3.0, height: 4.0 }
}
```

---

## 0.6 — `Struct`: Gom nhóm dữ liệu

Nếu `enum` là "hoặc cái này, hoặc cái kia", thì `struct` là "và". Nó giúp bạn gom nhóm nhiều mảnh dữ liệu lại thành một thực thể duy nhất:

```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p = Point { x: 1.0, y: 2.5 };
    println!("Point: ({}, {})", p.x, p.y);
    // Output: Point: (1, 2.5)
}
```
Mỗi khi tạo ra một `Point`, bạn bắt buộc phải cung cấp đầy đủ cả `x` và `y`.

---

## 0.7 — `Match`: Kỹ thuật Pattern Matching đỉnh cao

`match` thường bị lầm tưởng là giống `switch/case` ở các ngôn ngữ khác, nhưng thực chất nó mạnh mẽ hơn rất nhiều. Trình biên dịch của Rust đặc biệt nghiêm khắc với `match`: nó **bắt buộc** bạn phải xử lý *mọi* trường hợp có thể xảy ra, không được bỏ sót dù chỉ một.

```rust
#[derive(Debug)]
enum Season { Spring, Summer, Fall, Winter }

fn describe(season: &Season) -> &str {
    match season {
        Season::Spring => "Hoa nở 🌸",
        Season::Summer => "Nắng nóng ☀️",
        Season::Fall   => "Lá rụng 🍂",
        Season::Winter => "Lạnh giá ❄️",
    }
    // Nếu bạn vô tình quên mất mùa Đông, trình biên dịch sẽ chặn đứng bạn ngay lập tức!
}

fn main() {
    let now = Season::Summer;
    println!("{}: {}", format!("{:?}", now), describe(&now));
    // Output: Summer: Nắng nóng ☀️
}
```

Sức mạnh của `match` càng bộc lộ rõ khi kết hợp với các `enum` có chứa dữ liệu. Bạn có thể "bóc tách" (destructure) dữ liệu ra để tính toán một cách vô cùng tự nhiên:

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    match shape {
        // Bóc tách biến 'radius' ra khỏi khối Circle
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        
        // Bóc tách 'width' và 'height' ra khỏi khối Rectangle
        Shape::Rectangle { width, height } => width * height,
    }
}

fn main() {
    let c = Shape::Circle(5.0);
    let r = Shape::Rectangle { width: 3.0, height: 4.0 };

    println!("Circle area: {:.2}", area(&c));
    println!("Rect area: {:.2}", area(&r));
}
```

---

## 0.8 — Closures: Những Hàm Vô Danh Nhỏ Gọn

Đôi khi việc định nghĩa một hàm hoàn chỉnh bằng từ khóa `fn` là quá cồng kềnh. Đó là lúc **Closure** (hàm vô danh/hàm ẩn danh) lên ngôi. Closure cho phép bạn định nghĩa các logic ngắn gọn ngay tại nơi bạn cần dùng đến chúng, thông qua cú pháp cặp dấu gạch đứng `||`:

```rust
fn main() {
    // Định nghĩa một closure nhận tham số x và trả về x + 1
    let add_one = |x: i32| x + 1;
    let multiply = |a: i32, b: i32| a * b;

    println!("add_one(5) = {}", add_one(5));       // Kết quả: 6
    println!("multiply(3, 4) = {}", multiply(3, 4)); // Kết quả: 12

    // Closure đặc biệt tỏa sáng khi làm việc với các collection (tập hợp)
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    // Output: Doubled: [2, 4, 6, 8, 10]
}
```

> **💡 Góc toán học**: Nếu bạn thấy cú pháp `|x| x + 1` quen quen, thì chúc mừng, nó chính là hiện thân của biểu thức `λx. x + 1` trong Toán học. Chúng ta sẽ đào sâu về nó trong Chapter 1.

---

## 0.9 — Hành trang thường ngày: `Vec`, vòng lặp, in ấn và kiểm thử

### `Vec` — Mảng động linh hoạt

Khi cần lưu trữ một danh sách các phần tử có thể thay đổi kích thước, `Vec` (Vector) là lựa chọn mặc định.

```rust
fn main() {
    // Sử dụng macro vec! để khởi tạo nhanh
    let fruits = vec!["🍎", "🍊", "🍇"];

    println!("First: {}", fruits[0]);     // Lấy phần tử đầu tiên
    println!("Length: {}", fruits.len()); // Lấy số lượng phần tử

    // Duyệt qua từng phần tử một cách thanh lịch
    for fruit in &fruits {
        println!("- {}", fruit);
    }
}
```

### Nghệ thuật in ấn với `println!`

Macro `println!` là người bạn không thể thiếu khi debug hoặc tương tác. Rust cung cấp nhiều cách định dạng chuỗi mạnh mẽ:

```rust
fn main() {
    let name = "Rust";
    let version = 1.84;

    // Sử dụng {} làm "chỗ trống", giá trị sẽ được điền vào theo thứ tự
    println!("Hello, {}!", name);
    println!("{} version {}", name, version);

    // Dùng {:?} để in cấu trúc dữ liệu theo định dạng Debug (rất tiện để soi nội dung struct/enum/mảng)
    let nums = vec![1, 2, 3];
    println!("nums = {:?}", nums);

    // Định dạng số thực hiển thị đúng 2 chữ số thập phân với {:.2}
    println!("Pi ≈ {:.2}", std::f64::consts::PI);
}
```

### Tự kiểm chứng với `assert_eq!`

Thay vì dùng mắt để dò kết quả in ra màn hình, bạn có thể buộc chương trình tự động kiểm tra tính đúng đắn.

```rust
fn main() {
    let result = 2 + 3;
    assert_eq!(result, 5);  // ✅ Pass: Mọi thứ êm đẹp, chương trình tiếp tục chạy

    // assert_eq!(result, 99); // ❌ Lỗi! Chương trình sẽ hoảng loạn (panic) và dừng lại kèm thông báo lỗi
    println!("All assertions passed!");
}
```

> **💡 Ghi chú**: Bạn sẽ thấy `assert_eq!` xuất hiện dày đặc trong cuốn sách này. Đây là cách tốt nhất để minh họa kết quả mã nguồn một cách minh bạch và chính xác.

---

## 0.10 — Làm quen với `Option` và `Result`

Trong phần khởi động này, bạn chưa cần phải nắm bắt tường tận hai khái niệm này. Chỉ cần học cách **nhận diện** chúng khi đọc mã nguồn trong các chương tiếp theo.

### `Option<T>` — Câu trả lời cho sự "vắng mặt"

Rust không có khái niệm `null`. Thay vào đó, nếu một giá trị có thể tồn tại hoặc không, Rust dùng `Option`. 

```rust
fn find_even(numbers: &[i32]) -> Option<i32> {
    // Thử tìm số chẵn đầu tiên
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);  // Nếu thấy, gói nó trong hộp `Some()`
        }
    }
    None  // Nếu tìm hoài không thấy, trả về hộp rỗng `None`
}

fn main() {
    match find_even(&[1, 3, 4, 7]) {
        Some(n) => println!("Found even: {}", n),
        None    => println!("No even number"),
    }
}
```
Khi dùng hàm trả về `Option`, bạn luôn nhận được một "chiếc hộp". Bạn bắt buộc phải mở hộp (bằng `match` hoặc các phương thức khác) để biết bên trong có dữ liệu (`Some`) hay không (`None`). Không bao giờ có chuyện chương trình vô tình "nổ tung" vì truy cập vào một giá trị `null` vô hình.

### `Result<T, E>` — Đương đầu với lỗi lầm

Nếu một hành động có nguy cơ thất bại (như đọc file, kết nối mạng, tính toán sai), hàm sẽ trả về `Result`.

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())  // Trả về lỗi được bọc trong Err
    } else {
        Ok(a / b)  // Trả về kết quả thành công được bọc trong Ok
    }
}

fn main() {
    match divide(10.0, 3.0) {
        Ok(result) => println!("10 / 3 = {:.2}", result),
        Err(e)     => println!("Error: {}", e),
    }
}
```
Cũng giống như `Option`, `Result` ép bạn phải tường minh đối diện với cả hai kịch bản: mọi việc suôn sẻ (`Ok`) hoặc có biến cố xảy ra (`Err`).

---

## Tổng kết hành trang

Bài viết này đã cung cấp cho bạn vừa đủ những "vũ khí" cú pháp để đọc hiểu mã nguồn trong Part 0. Bạn **không cần phải cố gắng ghi nhớ mọi thứ ngay lập tức**. Hãy cứ thoải mái quay lại bảng tham chiếu dưới đây mỗi khi bạn thấy bối rối.

### 🔧 Bảng Tham Chiếu Nhanh

| Cú pháp | Ý nghĩa | Ví dụ |
|---------|---------|-------|
| `let x = 5;` | Tạo biến bất biến (immutable) | `let name = "Rust";` |
| `let mut x = 5;` | Tạo biến có thể thay đổi (mutable) | `x = 10;` (hợp lệ) |
| `fn foo(a: i32) -> i32` | Khai báo hàm | Có tham số truyền vào và kiểu trả về |
| `\|x\| x + 1` | Closure | Hàm vô danh, xử lý nhanh gọn |
| `enum { A, B }` | Enum (Sum type) | Lưu trữ một trong nhiều khả năng |
| `struct { x, y }` | Struct (Product type) | Lưu trữ tổ hợp dữ liệu |
| `match val { ... }` | Pattern matching | Bắt buộc xử lý mọi trường hợp (exhaustive) |
| `Some(v)` / `None` | Kiểu `Option` | Dữ liệu có thể có hoặc không |
| `Ok(v)` / `Err(e)` | Kiểu `Result` | Kết quả có thể thành công hoặc lỗi |
| `vec![1, 2, 3]` | `Vec` (Mảng động) | Tập hợp dữ liệu |
| `println!("{}", x)` | In ra console | In theo định dạng (dùng `{:?}` để debug) |
| `assert_eq!(a, b)` | Kiểm tra logic | Ứng dụng hoảng loạn (panic) nếu sai lệch |

Có một số khái niệm mạnh mẽ tạo nên linh hồn của Rust **chưa được đề cập ở đây**, nhưng bạn đừng lo, chúng ta sẽ dần dần bóc tách chúng ở Part I:
- **Ownership & Borrowing** (Chapter 9): Hệ thống quản lý bộ nhớ không cần Garbage Collector độc quyền của Rust.
- **Traits** (Chapter 16): Cách Rust định nghĩa hành vi chung (tương tự như Interface).
- **Lifetimes** (Chapter 9): Cách trình biên dịch theo dõi vòng đời của dữ liệu.
- **Error handling chuyên sâu** (Chapter 10).
- **Modules & Crates** (Chapter 11): Tổ chức kiến trúc dự án lớn.

Giờ thì hành lý đã sẵn sàng, hãy cùng nhau bắt đầu hành trình!

## Bước tiếp theo

→ Đi tới **Chapter 1: Math Foundations for FP** — Nơi bạn sẽ khám phá Lambda Calculus, định lý Curry-Howard (xem logic như kiểu dữ liệu) và học cách áp dụng những phép cộng, phép nhân để kiểm soát trạng thái của toàn bộ hệ thống.

---

## ✅ Checkpoint 0

1. `let x = 5;` và `let mut x = 5;` khác nhau thế nào, và mặc định nào là bất biến?
2. `if` trong Rust là câu lệnh hay biểu thức? Điều đó cho phép viết gì?
3. `&str` và `String` khác nhau ra sao ở mức "đủ để đọc code"?

<details>
<summary>Đáp án</summary>

1. `let` tạo binding **bất biến** — mặc định của Rust. `mut` mới cho phép gán lại. Ngược hoàn toàn với hầu hết ngôn ngữ khác, và đó là một quyết định thiết kế có chủ đích.
2. **Biểu thức** — nó trả về giá trị. Nhờ vậy viết được `let x = if cond { 1 } else { 2 };` mà không cần toán tử ba ngôi riêng.
3. `&str` là một **lát cắt mượn** vào chuỗi có sẵn (không sở hữu). `String` **sở hữu** dữ liệu trên heap và co giãn được. Quy tắc đọc code: tham số hàm thường nhận `&str`, giá trị trả về thường là `String`.
</details>

---

## 🏋️ Bài tập

**Bài 1 (5 phút).** Viết hàm `fn describe(n: i32) -> &'static str` dùng `if` như một biểu thức, trả `"âm"`, `"không"`, hoặc `"dương"`.

**Bài 2 (10 phút).** Viết `fn first_even(v: &[i32]) -> Option<i32>`. Dùng `match` để xử lý cả hai nhánh ở chỗ gọi.

**Bài 3 (10 phút).** Viết một closure cộng thêm một giá trị bắt từ môi trường, rồi dùng nó với `.map()` trên một `Vec`.

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| `cannot assign twice to immutable variable` | Quên `mut` | Thêm `mut`, hoặc tốt hơn: tạo binding mới (shadowing) |
| `expected &str, found String` | Nhầm kiểu sở hữu và kiểu mượn | Thêm `&` trước `String`, hoặc `.as_str()` |
| `mismatched types` ở nhánh `if` | Hai nhánh trả kiểu khác nhau | `if` là biểu thức nên mọi nhánh phải cùng kiểu |
| `non-exhaustive patterns` | `match` thiếu nhánh | Liệt kê đủ variant, hoặc thêm `_ =>` |
| Lỗi ownership khó hiểu | Chưa học chương 9 | Bình thường ở giai đoạn này — Chapter 9 sẽ giải thích trọn vẹn |

---

## Tóm tắt

- **Bất biến là mặc định.** `let` khoá giá trị; `mut` là ngoại lệ bạn phải nói ra.
- **Gần như mọi thứ là biểu thức**, kể cả `if` và `match` — nên chúng trả giá trị được.
- **`Option<T>` và `Result<T, E>` thay cho null và exception.** Ở chương này chỉ
  cần *đọc hiểu*; Chapter 10 sẽ dạy dùng chúng cho tử tế.
- **Chương này cố tình bỏ qua ownership.** Nếu gặp lỗi borrow checker, cứ ghi
  lại và đi tiếp — Chapter 9 dành trọn cho nó.

## Tiếp theo

Bạn đã đọc được code Rust. Giờ đến phần nền tảng lý thuyết:
**[Chapter 1 — Math Foundations for FP](chapter_01_math_foundations.md)** —
hoặc nhảy thẳng tới [Part I](../part_1_rust_fundamentals/chapter_04_getting_started.md)
nếu muốn code ngay.
