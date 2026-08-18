# Chapter 31 — Parser Combinators: Xây lâu đài từ những viên gạch nhỏ

> **Bạn sẽ học được**:
> - **Parser** = function `&str → Result<(T, &str)>` — ăn input, trả value + remaining
> - **Combinators**: `map`, `and_then`, `pair`, `alt`, `many0`
> - Build parser **từ nhỏ lên lớn** — compose tiny parsers thành complex ones
> - Functors + Monads trong thực hành!
> - Build: number parser, string parser, key-value parser, mini JSON parser
>
> **Yêu cầu trước**: Chapter 29 (Functors), Chapter 30 (Monads).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn build parsers bằng **function composition** — không regex, không manual index.

---

## Parser Combinators — Xây dựng parsers từ building blocks

Nếu bạn từng phải bóc tách dữ liệu (parsing) bằng các biểu thức chính quy (Regex) dài ngoằng hoặc dùng hàm `split` thủ công, bạn hẳn đã nếm mùi đau khổ. Regex cực kỳ dễ vỡ, khó bảo trì, và gần như không thể test riêng lẻ từng thành phần. "Hôm nay chạy ngon, ngày mai thêm một khoảng trắng là sập."

Parser combinators là cách lập trình hàm (FP) giải quyết bài toán này một cách thanh lịch: Thay vì viết một cỗ máy phân tích khổng lồ, chúng ta viết những máy phân tích **cực kỳ nhỏ và đơn giản**. Sau đó, ta dùng các chất keo dính (combinators như `map`, `or`, `many`) để ghép chúng lại với nhau.

Giống như trò xếp hình Lego: Bạn không tự đúc một khối lâu đài bằng nhựa. Bạn lấy những viên gạch 1x1, 2x2 lắp lại với nhau. Kết quả: mỗi viên gạch đều có thể test riêng, lâu đài vững chắc, và hệ thống type-safe tuyệt đối!

---

## 31.1 — Parser bản chất chỉ là một Function

Một Parser thực chất là gì? Hãy tưởng tượng nó như một cái máy xay thịt. Bạn nhét vào một miếng thịt dài (String input). Máy cắn một miếng, nhả ra viên thịt viên (giá trị đã parse được), và nhả ra phần thịt còn lại chưa xử lý. Nếu miếng thịt có xương (lỗi cú pháp), máy báo lỗi.

Bằng code Rust, chúng ta định nghĩa kiểu `ParseResult` để thể hiện khái niệm này:

```rust
// filename: src/main.rs

// Parser type: ăn &str, trả (parsed_value, remaining_input) hoặc Lỗi
type ParseResult<'a, T> = Result<(T, &'a str), String>;
```

Bây giờ, hãy tạo "viên gạch Lego" cơ bản nhất: Một hàm phân tích duy nhất một ký tự thỏa mãn điều kiện nào đó.

```rust
/// Parse 1 ký tự thỏa condition
fn satisfy(input: &str, pred: impl Fn(char) -> bool) -> ParseResult<char> {
    match input.chars().next() {
        Some(c) if pred(c) => Ok((c, &input[c.len_utf8()..])), // Cắn 1 miếng, trả phần còn lại
        Some(c) => Err(format!("Unexpected '{}'", c)),
        None => Err("Unexpected end of input".into()),
    }
}
```

Có viên gạch này rồi, việc tạo ra các parser cụ thể như "đọc số" hay "đọc chữ" trở nên dễ dàng như trở bàn tay:

```rust
/// Parse đúng 1 chữ số
fn digit(input: &str) -> ParseResult<char> {
    satisfy(input, |c| c.is_ascii_digit())
}

/// Parse đúng 1 chữ cái
fn letter(input: &str) -> ParseResult<char> {
    satisfy(input, |c| c.is_alphabetic())
}
```

Đôi khi ta cần đọc một chuỗi cụ thể, ví dụ như từ khóa "true" hoặc "false". Ta viết một parser `tag`:

```rust
/// Parse chính xác một chuỗi cho trước
fn tag<'a>(input: &'a str, expected: &str) -> ParseResult<'a, &'a str> {
    if input.starts_with(expected) {
        Ok((&input[..expected.len()], &input[expected.len()..]))
    } else {
        Err(format!("Expected '{}', got '{}'", expected, &input[..input.len().min(10)]))
    }
}
```

Hãy thử chạy chúng xem sao:

```rust
fn main() {
    println!("digit: {:?}", digit("42abc"));       // Ok(('4', "2abc"))
    println!("letter: {:?}", letter("hello"));      // Ok(('h', "ello"))
    println!("tag: {:?}", tag("hello world", "hello")); // Ok(("hello", " world"))

    // Error cases
    println!("digit err: {:?}", digit("abc"));      // Err: Unexpected 'a'
}
```

Rất gọn gàng! Nhưng đây chỉ mới là những viên gạch. Làm sao để xây nhà?

---

## 31.2 — Combinators: Chất keo dính vạn năng

Có parser cơ bản rồi, chúng ta cần các **combinators**: đây là những function nhận vào parser và trả về một parser lớn hơn.

### `map`: Biến đổi kết quả (Xin chào Functor!)

Giả sử `digit` trả về ký tự `'7'`, nhưng bạn lại muốn con số `7` kiểu `u32`. Bạn cần một cách để ánh xạ (map) kết quả bên trong. Đây chính là khái niệm Functor mà ta học ở chương 29!

```rust
// map: Chạy parser gốc, nếu thành công thì lấy kết quả chạy qua hàm f
fn map<'a, A, B>(
    parser: impl Fn(&'a str) -> ParseResult<'a, A>,
    f: impl Fn(A) -> B,
) -> impl Fn(&'a str) -> ParseResult<'a, B> {
    move |input| {
        // Cú pháp `?` giúp dừng sớm nếu parser gốc lỗi
        let (val, rest) = parser(input)?; 
        Ok((f(val), rest))
    }
}
```

### `many1`: Lặp đi lặp lại

Một chữ số thì không làm được gì nhiều. Chúng ta cần đọc số `42` (gồm nhiều chữ số). Hãy viết hàm `many1`, yêu cầu parse ít nhất 1 lần, và gom tất cả kết quả lại thành `Vec`.

```rust
// many1: Parse ít nhất 1 lần, lặp lại cho đến khi lỗi, thu thập kết quả
fn many1<'a, T>(
    parser: impl Fn(&'a str) -> ParseResult<'a, T>,
) -> impl Fn(&'a str) -> ParseResult<'a, Vec<T>> {
    move |input| {
        let (first, mut remaining) = parser(input)?; // Ít nhất 1 lần phải thành công
        let mut results = vec![first];
        
        // Cứ chạy tiếp chừng nào còn thành công
        while let Ok((val, rest)) = parser(remaining) {
            results.push(val);
            remaining = rest;
        }
        Ok((results, remaining))
    }
}
```

Giờ hãy kết hợp chúng lại để parse một con số hoàn chỉnh từ chuỗi:

```rust
fn main() {
    // Bước 1: Parse nhiều chữ số -> Vec<char>
    let digits_parser = many1(digit);
    
    // Bước 2: Dùng map để nối Vec<char> thành String rồi ép kiểu sang i64
    let number_parser = map(digits_parser, |chars| {
        chars.iter().collect::<String>().parse::<i64>().unwrap()
    });

    println!("number: {:?}", number_parser("12345+67")); // Ok((12345, "+67"))
}
```
Nhìn kìa! Không hề có vòng lặp for thủ công hay cắt chuỗi lằng nhằng trong code gọi hàm. Mọi thứ được định nghĩa theo hướng khai báo (declarative).

### `pair` và `alt`: Tuần tự và Lựa chọn

Đôi khi bạn cần đọc chuỗi A **rồi đến** chuỗi B. Hoặc bạn cần đọc chữ **hoặc** số. Chúng ta định nghĩa `pair` (tuần tự) và `alt` (thay thế).

```rust
// pair: Chạy parser A, rồi lấy phần còn lại nạp vào parser B
fn pair<'a, A, B>(
    pa: impl Fn(&'a str) -> ParseResult<'a, A>,
    pb: impl Fn(&'a str) -> ParseResult<'a, B>,
) -> impl Fn(&'a str) -> ParseResult<'a, (A, B)> {
    move |input| {
        let (a, rest1) = pa(input)?;
        let (b, rest2) = pb(rest1)?;
        Ok(((a, b), rest2))
    }
}

// alt: Thử chạy A, nếu A lỗi thì thử chạy B
fn alt<'a, T>(
    pa: impl Fn(&'a str) -> ParseResult<'a, T>,
    pb: impl Fn(&'a str) -> ParseResult<'a, T>,
) -> impl Fn(&'a str) -> ParseResult<'a, T> {
    move |input| {
        pa(input).or_else(|_| pb(input))
    }
}
```

---

## ✅ Checkpoint 31.2

> Ghi nhớ:
> 1. **Parser** = `&str → Result<(T, &str)>`. Ăn input, trả value + remaining.
> 2. **`map`** = Functor! Biến đổi kết quả (A -> B).
> 3. **`pair`** = Sequence. Parse A rồi tới B.
> 4. **`alt`** = Choice. Thử A, nếu hỏng thì chuyển sang B.
> 5. **`many1`** = Repetition. Lặp lại 1 hoặc nhiều lần.
>
> Bạn có thấy sự tương đồng? `map` chính là Functor, và nếu bạn viết `and_then` nó chính là Monad. Parser combinators **CHÍNH LÀ** FP trong thực tế!

---

## 31.3 — Xây dựng Parser thực tế: Số và Chuỗi

Với các viên gạch đã có, hãy thử phân tích (parse) một số thập phân (float) hỗ trợ cả dấu âm. Cách tư duy:
1. Đọc dấu trừ (nếu có).
2. Đọc phần nguyên (các chữ số).
3. Đọc dấu chấm `.`.
4. Đọc phần thập phân.

```rust
fn parse_number(input: &str) -> ParseResult<f64> {
    // 1. Lấy dấu
    let (sign, rest) = match input.starts_with('-') {
        true => (-1.0, &input[1..]),
        false => (1.0, input),
    };

    // 2. Lấy phần nguyên
    let integer = many1(digit);
    let (int_chars, rest) = integer(rest)?;
    let int_str: String = int_chars.iter().collect();

    // 3. Nếu có dấu chấm, lấy phần thập phân
    if rest.starts_with('.') {
        let decimal = many1(digit);
        let (dec_chars, rest) = decimal(&rest[1..])?;
        let dec_str: String = dec_chars.iter().collect();
        
        let full = format!("{}.{}", int_str, dec_str);
        let num: f64 = full.parse().unwrap();
        Ok((sign * num, rest))
    } else {
        // Chỉ là số nguyên
        let num: f64 = int_str.parse().unwrap();
        Ok((sign * num, rest))
    }
}
```

Đối với chuỗi được bọc trong ngoặc kép (String parser), logic hơi khác một chút vì chúng ta phải đối mặt với các ký tự escape như `\n` hoặc `\"`. Trong trường hợp này, viết một vòng lặp `loop` bên trong parser sẽ tối ưu và dễ đọc hơn là cố nhồi nhét quá nhiều combinator:

```rust
fn parse_string(input: &str) -> ParseResult<String> {
    if !input.starts_with('"') {
        return Err("Expected opening quote".into());
    }
    let rest = &input[1..];
    let mut result = String::new();
    let mut chars = rest.char_indices();

    loop {
        match chars.next() {
            Some((i, '"')) => {
                // Gặp ngoặc đóng -> Hoàn thành. Trả về kết quả và chuỗi còn lại.
                return Ok((result, &input[1 + i + 1..]));
            }
            Some((_, '\\')) => { // Ký tự escape
                match chars.next() {
                    Some((_, 'n')) => result.push('\n'),
                    Some((_, 't')) => result.push('\t'),
                    Some((_, '"')) => result.push('"'),
                    Some((_, '\\')) => result.push('\\'),
                    Some((_, c)) => result.push(c),
                    None => return Err("Unexpected end in escape".into()),
                }
            }
            Some((_, c)) => result.push(c), // Ký tự thường
            None => return Err("Unterminated string".into()),
        }
    }
}
```

---

## 31.4 — Đỉnh cao: Xây dựng Mini JSON Parser

Bây giờ là màn trình diễn ngoạn mục nhất. Chúng ta sẽ "xếp Lego" tất cả những thứ trên thành một JSON Parser hoàn chỉnh. Không Regex, không thư viện ngoài.

Đầu tiên, hãy định nghĩa kiểu dữ liệu JSON:

```rust
#[derive(Debug, Clone, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}
```

Tiếp theo là viết các parser cơ bản cho Null và Boolean. Hãy nhớ luôn "ăn" các khoảng trắng thừa bằng hàm `ws` (whitespace).

```rust
fn ws(input: &str) -> &str { input.trim_start() }

fn parse_null(input: &str) -> ParseResult<JsonValue> {
    let input = ws(input);
    if input.starts_with("null") {
        Ok((JsonValue::Null, &input[4..]))
    } else { Err("Expected null".into()) }
}

fn parse_bool(input: &str) -> ParseResult<JsonValue> {
    let input = ws(input);
    if input.starts_with("true") {
        Ok((JsonValue::Bool(true), &input[4..]))
    } else if input.starts_with("false") {
        Ok((JsonValue::Bool(false), &input[5..]))
    } else { Err("Expected bool".into()) }
}
```

Tương tự, ta tái sử dụng hàm `parse_string` ở trên và bọc nó vào `JsonValue::Str`.
Nhưng JSON thú vị nhất là ở Mảng (Array) và Đối tượng (Object), vì chúng có tính **đệ quy**. Một Array chứa các `JsonValue`, mà bản thân `JsonValue` lại có thể là một Array khác!

Hãy xem cách chúng ta parse một Array: đọc `[`, sau đó lặp lại việc đọc `JsonValue` và dấu phẩy `,`, kết thúc khi gặp `]`.

```rust
fn parse_array(input: &str) -> ParseResult<JsonValue> {
    let input = ws(input);
    if !input.starts_with('[') { return Err("Expected [".into()); }
    let mut rest = ws(&input[1..]);
    let mut items = vec![];

    if rest.starts_with(']') { // Mảng rỗng
        return Ok((JsonValue::Array(items), &rest[1..]));
    }

    loop {
        // parse_value là hàm tổng, sẽ định nghĩa bên dưới
        let (val, r) = parse_value(rest)?;
        items.push(val);
        rest = ws(r);
        
        if rest.starts_with(',') { 
            rest = ws(&rest[1..]); // Ăn dấu phẩy và đi tiếp
        }
        else if rest.starts_with(']') { 
            return Ok((JsonValue::Array(items), &rest[1..])); // Xong!
        }
        else { return Err("Expected , or ]".into()); }
    }
}
```

Với Object, logic hoàn toàn tương tự, chỉ khác là ta phải đọc thêm Key (một chuỗi) và dấu hai chấm `:`.
Và cuối cùng, "Bộ não" của toàn bộ Parser này chính là hàm `parse_value`. Hàm này kết nối tất cả các parser con lại bằng combinator `or_else` (tương đương với `alt`):

```rust
// ═══ Main parser: alt over all value types ═══
fn parse_value(input: &str) -> ParseResult<JsonValue> {
    let input = ws(input);
    
    // Thử lần lượt. Nếu hỏng cái này, thử cái kia!
    parse_null(input)
        .or_else(|_| parse_bool(input))
        .or_else(|_| parse_number(input)) // Bỏ qua implement chi tiết vì giống ở trên
        .or_else(|_| parse_string(input))
        .or_else(|_| parse_array(input))
        .or_else(|_| parse_object(input)) // Tương tự array nhưng parse cặp key-value
}
```

Tuyệt vời! Chỉ với chưa đầy 150 dòng code, bạn đã tự tay viết một JSON Parser đệ quy hoàn chỉnh. 

---

## 31.5 — Mối liên kết với các khái niệm FP

Nếu bạn đọc đến đây và nhận ra `map`, `or_else`, `alt` quen quen — đúng vậy. Parser combinators là Functor, Monad và Alternative đang hoạt động trong thế giới thực! Dưới đây là bảng đối chiếu để bạn củng cố kiến thức:

| Combinator | Khái niệm FP | Ý nghĩa trong ngữ cảnh Parser |
|------------|-----------|---------|
| `map(parser, f)` | **Functor** | Biến đổi giá trị lấy ra được mà không làm hỏng chuỗi còn lại |
| `and_then(pa, f)` | **Monad** | Dùng giá trị vừa parse được để "quyết định" xem sẽ dùng parser nào tiếp theo |
| `pair(pa, pb)` | **Applicative** | Chạy cả hai, gộp kết quả lại |
| `alt(pa, pb)` | **Alternative** | Thử phương án A, nếu thất bại thì lui lại và thử phương án B |
| `many0(p)` | **MonadPlus** | Lặp lại 0 hoặc nhiều lần (List Monad) |

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| Lặp vô tận (Infinite loop) trong `many0` | Parser không chịu tiêu thụ (consume) input, nên nó lặp mãi ở một vị trí. | Đảm bảo parser luôn tiêu thụ ít nhất 1 byte khi trả về `Ok`. |
| Lỗi Lifetime | `&str` references quá phức tạp khi trả về. | Dùng `String` sở hữu (owned) cho kết quả output, chỉ dùng `&str` cho input đầu vào. |
| "Backtracking chậm quá!" | Dùng `alt` quá đà khiến chương trình phải thử đi thử lại nhiều lần. | Dùng `peek` (nhìn trước ký tự đầu tiên) để chuyển hướng logic nhanh thay vì thử mù quáng. |
| Hàm parser quá dài | Bạn đang cố nhét mọi logic vào một hàm. | Cắt nó ra thành các hàm parser siêu nhỏ, rồi gom lại bằng `pair` và `map`. |

---

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Viết parser cho số nguyên có dấu (`-42`, `+7`, `13`) bằng `nom`. Test cả input hợp lệ lẫn không hợp lệ.

**Bài 2 (20 phút).** Mở rộng JSON parser trong chương để hỗ trợ escape sequence trong chuỗi (`\n`, `\t`, `\"`, `\uXXXX`).

**Bài 3 (30 phút).** Viết parser cho biểu thức số học có độ ưu tiên đúng: `1 + 2 * 3` phải ra `7`, không phải `9`. Gợi ý: tách thành `expr → term → factor`.

<details>
<summary>Gợi ý bài 3</summary>

Độ ưu tiên được mã hoá bằng **tầng của ngữ pháp**, không phải bằng bảng ưu tiên:
`expr = term (('+' | '-') term)*`, `term = factor (('*' | '/') factor)*`,
`factor = số | '(' expr ')'`. Phép nào nằm ở tầng sâu hơn thì buộc chặt hơn.
</details>

## Tóm tắt

- ✅ **Parser** = `&str → Result<(T, &str)>`. Tiêu thụ chuỗi, tạo ra giá trị.
- ✅ **Combinators**: `map` (Functor), `pair` (sequence), `alt` (choice), `many0/many1` (repeat). Chúng là chất keo dính.
- ✅ **Sức mạnh của sự kết hợp**: Từ những parser tí hon, ta lắp ráp thành parser khổng lồ như JSON mà code vẫn sạch và testable.
- ✅ **Sản xuất thực tế**: Trong dự án thật, đừng tự viết combinators. Hãy dùng các thư viện nổi tiếng như `nom` (hướng macro) hoặc `chumsky` (hướng type-based).

## Tiếp theo

→ Chapter 32: **Recursive Types & Folds** — chapter cuối cùng của Part V! Bạn sẽ học cách mô hình hóa cây dữ liệu (Trees), biểu thức toán học, và các cấu trúc đệ quy. `fold` sẽ trở thành công cụ vạn năng để xử lý mọi dữ liệu đệ quy.
