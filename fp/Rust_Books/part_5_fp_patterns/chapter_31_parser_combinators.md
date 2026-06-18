# Chapter 31 — Parser Combinators

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

Nếu bạn từng viết parser bằng regex hoặc manual string splitting, bạn biết nó dễ vỡ, khó maintain, và gần như không thể test riêng lẻ. Parser combinators là cách FP giải quyết bài toán parsing: xây dựng parsers phức tạp từ **những parsers nhỏ, đơn giản**, kết hợp chúng bằng combinators (map, and_then, or, many).

Kết quả: mỗi parser nhỏ test được riêng, parser lớn composable, và tất cả type-safe. Đây là ứng dụng thực tế đẹp nhất của FP concepts (functors, monads) mà bạn học ở chapters trước.

---

## 31.1 — Parser = Function

Bạn có một string JSON, CSV, hoặc query. Cách thường thấy: regex hoặc split bằng tay, rồi fix bugs vì edge cases. Cách FP: viết từng parser nhỏ ("parse 1 chữ số", "parse 1 string") rồi **gắp chúng lại** thành parser lớn. Giống làm bánh: không cần 1 công thức khổng lồ, chỉ cần biết trộn bột, đánh kem, và lắp ráp.

Parser đơn giản nhất là một function: nhận string input, trả về **(giá trị parsed, phần còn lại)** hoặc error.

### Parser cơ bản

```rust
// filename: src/main.rs

// Parser type: ăn &str, trả (parsed_value, remaining_input)
type ParseResult<'a, T> = Result<(T, &'a str), String>;

// ═══ Basic parsers (building blocks) ═══

/// Parse 1 ký tự thỏa condition
fn satisfy(input: &str, pred: impl Fn(char) -> bool) -> ParseResult<char> {
    match input.chars().next() {
        Some(c) if pred(c) => Ok((c, &input[c.len_utf8()..])),
        Some(c) => Err(format!("Unexpected '{}'", c)),
        None => Err("Unexpected end of input".into()),
    }
}

/// Parse exact string
fn tag<'a>(input: &'a str, expected: &str) -> ParseResult<'a, &'a str> {
    if input.starts_with(expected) {
        Ok((&input[..expected.len()], &input[expected.len()..]))
    } else {
        Err(format!("Expected '{}', got '{}'", expected, &input[..input.len().min(10)]))
    }
}

/// Parse 1 digit
fn digit(input: &str) -> ParseResult<char> {
    satisfy(input, |c| c.is_ascii_digit())
}

/// Parse 1 letter
fn letter(input: &str) -> ParseResult<char> {
    satisfy(input, |c| c.is_alphabetic())
}

/// Skip whitespace
fn whitespace(input: &str) -> ParseResult<&str> {
    let trimmed = input.trim_start();
    let consumed = input.len() - trimmed.len();
    Ok((&input[..consumed], trimmed))
}

fn main() {
    println!("digit: {:?}", digit("42abc"));       // Ok(('4', "2abc"))
    println!("letter: {:?}", letter("hello"));      // Ok(('h', "ello"))
    println!("tag: {:?}", tag("hello world", "hello")); // Ok(("hello", " world"))
    println!("ws: {:?}", whitespace("  hello"));    // Ok(("  ", "hello"))

    // Error cases
    println!("digit err: {:?}", digit("abc"));      // Err
    println!("tag err: {:?}", tag("hi", "hello"));  // Err
}
```

---

## 31.2 — Combinators: Compose Parsers

Có parser cơ bản rồi — làm sao gắp chúng lại? Dùng **combinators**: functions nhận parser(s) và trả parser mới. `map` biến đổi kết quả (Functor!). `pair` chạy 2 parsers liên tiếp. `alt` thử parser này, nếu fail thì thử parser kia. `many1` lặp lại 1+ lần.

### `map`: Transform parser output (Functor!)

```rust
// filename: src/main.rs

type ParseResult<'a, T> = Result<(T, &'a str), String>;

fn digit(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(c) if c.is_ascii_digit() => Ok((c, &input[c.len_utf8()..])),
        _ => Err("Expected digit".into()),
    }
}

// map: transform parser output
fn map<'a, A, B>(
    parser: impl Fn(&'a str) -> ParseResult<'a, A>,
    f: impl Fn(A) -> B,
) -> impl Fn(&'a str) -> ParseResult<'a, B> {
    move |input| {
        parser(input).map(|(val, rest)| (f(val), rest))
    }
}

// many1: parse 1+ times, collect results
fn many1<'a, T>(
    parser: impl Fn(&'a str) -> ParseResult<'a, T>,
) -> impl Fn(&'a str) -> ParseResult<'a, Vec<T>> {
    move |input| {
        let (first, mut remaining) = parser(input)?;
        let mut results = vec![first];
        while let Ok((val, rest)) = parser(remaining) {
            results.push(val);
            remaining = rest;
        }
        Ok((results, remaining))
    }
}

fn main() {
    // digit → char, map → u32
    let digit_num = map(digit, |c| c.to_digit(10).unwrap());
    println!("digit_num: {:?}", digit_num("7abc")); // Ok((7, "abc"))

    // many1(digit) → Vec<char>
    let digits = many1(digit);
    println!("digits: {:?}", digits("42abc")); // Ok((['4','2'], "abc"))

    // Compose: many1(digit) → map → number
    let number = map(many1(digit), |chars| {
        chars.iter().collect::<String>().parse::<i64>().unwrap()
    });
    println!("number: {:?}", number("12345+67")); // Ok((12345, "+67"))
}
```

### `pair`: Parse A then B

```rust
// filename: src/main.rs

type ParseResult<'a, T> = Result<(T, &'a str), String>;

fn tag<'a>(expected: &'a str) -> impl Fn(&'a str) -> ParseResult<'a, &'a str> {
    move |input| {
        if input.starts_with(expected) {
            Ok((&input[..expected.len()], &input[expected.len()..]))
        } else {
            Err(format!("Expected '{}'", expected))
        }
    }
}

fn digit(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(c) if c.is_ascii_digit() => Ok((c, &input[c.len_utf8()..])),
        _ => Err("Expected digit".into()),
    }
}

// pair: run parser A, then parser B on remaining
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

// alt: try parser A, if fails try parser B
fn alt<'a, T>(
    pa: impl Fn(&'a str) -> ParseResult<'a, T>,
    pb: impl Fn(&'a str) -> ParseResult<'a, T>,
) -> impl Fn(&'a str) -> ParseResult<'a, T> {
    move |input| {
        pa(input).or_else(|_| pb(input))
    }
}

fn main() {
    // pair: digit + digit
    let two_digits = pair(digit, digit);
    println!("pair: {:?}", two_digits("42abc"));  // Ok((('4','2'), "abc"))

    // alt: digit OR letter
    let letter = |input: &str| -> ParseResult<char> {
        match input.chars().next() {
            Some(c) if c.is_alphabetic() => Ok((c, &input[c.len_utf8()..])),
            _ => Err("Expected letter".into()),
        }
    };
    let digit_or_letter = alt(digit, letter);
    println!("alt '5': {:?}", digit_or_letter("5x"));   // Ok(('5', "x"))
    println!("alt 'a': {:?}", digit_or_letter("ax"));   // Ok(('a', "x"))
}
```

---

## ✅ Checkpoint 31.2

> Ghi nhớ:
> 1. **Parser** = `&str → Result<(T, &str)>`. Ăn input, trả value + remaining.
> 2. **`map`** = Functor! Transform output.
> 3. **`pair`** = Sequence. Parse A rồi B.
> 4. **`alt`** = Choice. Try A, fallback to B.
> 5. **`many1`** = Repetition. Parse 1+ times.
>
> Nhận ra pattern? `map` = Functor, `and_then` = Monad. Parser combinators **IS** FP in action!

---

## 31.3 — Build: Number & String Parsers

Bây giờ gắp các combinators lại để parse những thứ thực tế: số (có dấu, có phần thập phân) và strings (có escape sequences). Mỗi parser vẫn chỉ là function nhỏ, dùng `many1` và `satisfy` để xây.

```rust
// filename: src/main.rs

type ParseResult<'a, T> = Result<(T, &'a str), String>;

// ═══ Primitive parsers ═══
fn satisfy(input: &str, pred: impl Fn(char) -> bool, desc: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(c) if pred(c) => Ok((c, &input[c.len_utf8()..])),
        _ => Err(format!("Expected {}", desc)),
    }
}

fn many0<'a, T>(
    parser: impl Fn(&'a str) -> ParseResult<'a, T>,
) -> impl Fn(&'a str) -> ParseResult<'a, Vec<T>> {
    move |mut input| {
        let mut results = vec![];
        while let Ok((val, rest)) = parser(input) {
            results.push(val);
            input = rest;
        }
        Ok((results, input))
    }
}

fn many1<'a, T>(
    parser: impl Fn(&'a str) -> ParseResult<'a, T>,
) -> impl Fn(&'a str) -> ParseResult<'a, Vec<T>> {
    move |input| {
        let (first, mut remaining) = parser(input)?;
        let mut results = vec![first];
        while let Ok((val, rest)) = parser(remaining) {
            results.push(val);
            remaining = rest;
        }
        Ok((results, remaining))
    }
}

// ═══ Number parser ═══
fn parse_number(input: &str) -> ParseResult<f64> {
    let (sign, rest) = match input.starts_with('-') {
        true => (-1.0, &input[1..]),
        false => (1.0, input),
    };

    let integer = many1(|i| satisfy(i, |c| c.is_ascii_digit(), "digit"));
    let (int_chars, rest) = integer(rest)?;
    let int_str: String = int_chars.iter().collect();

    // Optional decimal part
    if rest.starts_with('.') {
        let decimal = many1(|i| satisfy(i, |c| c.is_ascii_digit(), "digit"));
        let (dec_chars, rest) = decimal(&rest[1..])?;
        let dec_str: String = dec_chars.iter().collect();
        let full = format!("{}.{}", int_str, dec_str);
        let num: f64 = full.parse().unwrap();
        Ok((sign * num, rest))
    } else {
        let num: f64 = int_str.parse().unwrap();
        Ok((sign * num, rest))
    }
}

// ═══ String parser (quoted) ═══
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
                // i = vị trí của closing quote trong `rest`
                // input offset = 1 (open quote) + i (content) + 1 (close quote)
                return Ok((result, &input[1 + i + 1..]));
            }
            Some((_, '\\')) => {
                match chars.next() {
                    Some((_, 'n')) => result.push('\n'),
                    Some((_, 't')) => result.push('\t'),
                    Some((_, '"')) => result.push('"'),
                    Some((_, '\\')) => result.push('\\'),
                    Some((_, c)) => result.push(c),
                    None => return Err("Unexpected end in escape".into()),
                }
            }
            Some((_, c)) => result.push(c),
            None => return Err("Unterminated string".into()),
        }
    }
}

fn main() {
    // Numbers
    println!("Integer: {:?}", parse_number("42 rest"));
    println!("Float: {:?}", parse_number("3.14xyz"));
    println!("Negative: {:?}", parse_number("-99.5!"));

    // Strings
    println!("\nString: {:?}", parse_string(r#""hello" rest"#));
    println!("Escape: {:?}", parse_string(r#""line\nnew""#));
    println!("Empty: {:?}", parse_string(r#""""#));
}
```

---

## 31.4 — Build: Key-Value & Mini JSON Parser

và đây là điểm đỉnh: gắp tất cả parsers đã viết thành một JSON parser hoàn chỉnh — parse null, boolean, number, string, array, và object (lồng nhau). Tất cả chỉ ~100 dòng, không dùng regex hay library bên ngoài.

```rust
// filename: src/main.rs

type ParseResult<'a, T> = Result<(T, &'a str), String>;

// ═══ JSON Value type ═══
#[derive(Debug, Clone, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

// ═══ Helper parsers ═══
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

fn parse_number(input: &str) -> ParseResult<JsonValue> {
    let input = ws(input);
    let end = input.find(|c: char| !c.is_ascii_digit() && c != '.' && c != '-' && c != '+' && c != 'e' && c != 'E')
        .unwrap_or(input.len());
    if end == 0 { return Err("Expected number".into()); }
    let num: f64 = input[..end].parse().map_err(|_| "Invalid number".to_string())?;
    Ok((JsonValue::Number(num), &input[end..]))
}

fn parse_string_raw(input: &str) -> ParseResult<String> {
    let input = ws(input);
    if !input.starts_with('"') { return Err("Expected string".into()); }
    let mut end = 1;
    let bytes = input.as_bytes();
    while end < bytes.len() {
        if bytes[end] == b'\\' { end += 2; continue; }
        if bytes[end] == b'"' {
            let s = &input[1..end];
            return Ok((s.to_string(), &input[end+1..]));
        }
        end += 1;
    }
    Err("Unterminated string".into())
}

fn parse_string(input: &str) -> ParseResult<JsonValue> {
    parse_string_raw(input).map(|(s, r)| (JsonValue::Str(s), r))
}

fn parse_array(input: &str) -> ParseResult<JsonValue> {
    let input = ws(input);
    if !input.starts_with('[') { return Err("Expected [".into()); }
    let mut rest = ws(&input[1..]);
    let mut items = vec![];

    if rest.starts_with(']') {
        return Ok((JsonValue::Array(items), &rest[1..]));
    }

    loop {
        let (val, r) = parse_value(rest)?;
        items.push(val);
        rest = ws(r);
        if rest.starts_with(',') { rest = ws(&rest[1..]); }
        else if rest.starts_with(']') { return Ok((JsonValue::Array(items), &rest[1..])); }
        else { return Err("Expected , or ]".into()); }
    }
}

fn parse_object(input: &str) -> ParseResult<JsonValue> {
    let input = ws(input);
    if !input.starts_with('{') { return Err("Expected {".into()); }
    let mut rest = ws(&input[1..]);
    let mut pairs = vec![];

    if rest.starts_with('}') {
        return Ok((JsonValue::Object(pairs), &rest[1..]));
    }

    loop {
        let (key, r) = parse_string_raw(rest)?;
        let r = ws(r);
        if !r.starts_with(':') { return Err("Expected :".into()); }
        let r = ws(&r[1..]);
        let (val, r) = parse_value(r)?;
        pairs.push((key, val));
        let r = ws(r);
        if r.starts_with(',') { rest = ws(&r[1..]); }
        else if r.starts_with('}') { return Ok((JsonValue::Object(pairs), &r[1..])); }
        else { return Err("Expected , or }".into()); }
    }
}

// ═══ Main parser: alt over all value types ═══
fn parse_value(input: &str) -> ParseResult<JsonValue> {
    let input = ws(input);
    parse_null(input)
        .or_else(|_| parse_bool(input))
        .or_else(|_| parse_number(input))
        .or_else(|_| parse_string(input))
        .or_else(|_| parse_array(input))
        .or_else(|_| parse_object(input))
}

fn main() {
    let json = r#"{
        "name": "Minh",
        "age": 28,
        "active": true,
        "scores": [95, 87, 92],
        "address": {
            "city": "HCM",
            "zip": "70000"
        },
        "nickname": null
    }"#;

    match parse_value(json) {
        Ok((value, remaining)) => {
            println!("✅ Parsed JSON:");
            print_json(&value, 0);
            println!("\nRemaining: '{}'", remaining.trim());
        }
        Err(e) => println!("❌ {}", e),
    }
}

fn print_json(val: &JsonValue, indent: usize) {
    let pad = "  ".repeat(indent);
    match val {
        JsonValue::Null => print!("null"),
        JsonValue::Bool(b) => print!("{}", b),
        JsonValue::Number(n) => print!("{}", n),
        JsonValue::Str(s) => print!("\"{}\"", s),
        JsonValue::Array(items) => {
            println!("[");
            for (i, item) in items.iter().enumerate() {
                print!("{}  ", pad);
                print_json(item, indent + 1);
                if i < items.len() - 1 { print!(","); }
                println!();
            }
            print!("{}]", pad);
        }
        JsonValue::Object(pairs) => {
            println!("{{");
            for (i, (key, val)) in pairs.iter().enumerate() {
                print!("{}  \"{}\": ", pad, key);
                print_json(val, indent + 1);
                if i < pairs.len() - 1 { print!(","); }
                println!();
            }
            print!("{}}}", pad);
        }
    }
}
```

---

## 31.5 — Connection to FP Concepts

Nếu bạn đọc đến đây và nhận ra `map`, `and_then`, `alt` quen quen — đúng vậy. Parser combinators là Functor + Monad + Alternative đang hoạt động. Đây là bảng mapping:

### Parser combinator = Functor + Monad in action

| Combinator | FP Concept | Ý nghĩa |
|------------|-----------|---------|
| `map(parser, f)` | **Functor** | Transform parsed value |
| `and_then(pa, f)` | **Monad** | Use parsed value to choose next parser |
| `pair(pa, pb)` | **Applicative** | Run both, combine results |
| `alt(pa, pb)` | **Alternative** | Try first, fallback to second |
| `many0(p)` | **MonadPlus** | Repeat 0+ times |
| `or_else` | **MonadError** | Error recovery |

```
parse_value  =  alt(null, bool, number, string, array, object)
                 ↑ Alternative pattern

parse_object =  '{' → many(pair(string, ':', value)) → '}'
                       ↑ Monad (pair) + Repetition (many)

parse_array  =  '[' → many(value, ',') → ']'
                       ↑ Repetition + Separator
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Basic parser

Viết `parse_identifier`: chữ cái đầu, sau đó chữ/số/underscore. Return `String`.

<details><summary>✅ Lời giải</summary>

```rust
fn parse_identifier(input: &str) -> ParseResult<String> {
    let mut chars = input.char_indices();
    match chars.next() {
        Some((_, c)) if c.is_alphabetic() || c == '_' => {
            let mut end = c.len_utf8();
            for (i, c) in chars {
                if c.is_alphanumeric() || c == '_' { end = i + c.len_utf8(); }
                else { break; }
            }
            Ok((input[..end].to_string(), &input[end..]))
        }
        _ => Err("Expected identifier".into()),
    }
}
```

</details>

---

**Bài 2** (10 phút): CSV parser

Viết parser cho CSV line: `"Minh,28,HCM"` → `vec!["Minh", "28", "HCM"]`.
- Fields separated by `,`
- Dùng combinator approach

<details><summary>✅ Lời giải Bài 2</summary>

```rust
fn parse_field(input: &str) -> ParseResult<String> {
    let end = input.find(',').unwrap_or(input.len());
    let end = end.min(input.find('\n').unwrap_or(input.len()));
    if end == 0 { return Err("Empty field".into()); }
    Ok((input[..end].to_string(), &input[end..]))
}

fn parse_csv_line(input: &str) -> ParseResult<Vec<String>> {
    let (first, mut rest) = parse_field(input)?;
    let mut fields = vec![first];
    while rest.starts_with(',') {
        let (field, r) = parse_field(&rest[1..])?;
        fields.push(field);
        rest = r;
    }
    Ok((fields, rest))
}

fn main() {
    println!("{:?}", parse_csv_line("Minh,28,HCM,Active"));
    // Ok((["Minh", "28", "HCM", "Active"], ""))
}
```

</details>

---

**Bài 3** (15 phút): Expression parser

Viết parser cho biểu thức toán đơn giản: `"3 + 5 * 2"`.
- Parse numbers (integers)
- Parse operators: `+`, `-`, `*`, `/`
- Trả `Vec<Token>` where `Token = Num(i64) | Op(char)`

<details><summary>✅ Lời giải Bài 3</summary>

```rust
#[derive(Debug)]
enum Token { Num(i64), Op(char) }

fn parse_tokens(input: &str) -> ParseResult<Vec<Token>> {
    let mut tokens = vec![];
    let mut rest = input.trim();

    while !rest.is_empty() {
        rest = rest.trim_start();
        if rest.is_empty() { break; }

        if rest.starts_with(|c: char| c.is_ascii_digit()) {
            let end = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
            let num: i64 = rest[..end].parse().unwrap();
            tokens.push(Token::Num(num));
            rest = &rest[end..];
        } else if rest.starts_with(['+', '-', '*', '/']) {
            tokens.push(Token::Op(rest.chars().next().unwrap()));
            rest = &rest[1..];
        } else {
            return Err(format!("Unexpected: '{}'", &rest[..1]));
        }
    }

    Ok((tokens, rest))
}

fn main() {
    println!("{:?}", parse_tokens("3 + 5 * 2 - 10"));
    // Ok(([Num(3), Op('+'), Num(5), Op('*'), Num(2), Op('-'), Num(10)], ""))
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| Infinite loop trong `many0` | Parser không consume input | Đảm bảo parser luôn consume ≥1 byte khi Ok |
| Lifetime errors | `&str` references phức tạp | Dùng owned `String` cho output, `&str` cho input |
| "Backtracking chậm" | `alt` thử rồi bỏ | Dùng `peek` (nhìn char đầu) để chọn nhanh |
| "Parser quá dài" | Monolithic function | Tách thành tiny parsers, compose bằng combinators |

---

## Tóm tắt

- ✅ **Parser** = `&str → Result<(T, &str)>`. Consume input, produce value.
- ✅ **Combinators**: `map` (Functor), `pair` (sequence), `alt` (choice), `many0/many1` (repeat).
- ✅ **Compose**: Tiny parsers → complex parsers. `parse_value = alt(null, bool, number, string, array, object)`.
- ✅ **JSON parser** hoàn chỉnh: ~100 LOC, parse nested objects/arrays — all from combinators!
- ✅ **FP in action**: Parser combinators **ARE** Functors + Monads + Alternatives.
- ✅ **Production**: Dùng crate `nom` (macro-style) hoặc `chumsky` (type-based) cho real projects.

## Tiếp theo

→ Chapter 32: **Recursive Types & Folds** — chapter cuối Part V! Bạn sẽ model trees, expressions, và recursive data structures. `fold` trở thành universal pattern cho processing recursive types.
