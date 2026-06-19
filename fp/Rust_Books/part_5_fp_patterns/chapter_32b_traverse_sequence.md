# Chapter 32b — Traverse & Sequence: Lộn ngược cấu trúc dữ liệu

> **Bạn sẽ học được**:
> - Khái niệm **Sequence**: Làm thế nào biến `Vec<Result>` thành `Result<Vec>`.
> - Sức mạnh kỳ diệu của hàm `.collect()` trong Rust: Nó chính là Sequence tích hợp sẵn!
> - Khái niệm **Traverse**: Kết hợp `map` và `sequence` trong một thao tác (Map-then-Sequence).
> - Sự khác biệt giữa Traversing bằng Monad (Fail-fast) và Traversing bằng Applicative (Collect-all).
>
> **Yêu cầu trước**: Chapter 31b (Applicative Validation).
> **Thời gian đọc**: ~35 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Biết cách xử lý hàng loạt phần tử mà không bị kẹt trong một mảng chứa đầy `Result`.

---

Khi làm việc với các hệ thống thực tế, bạn thường xuyên phải xử lý *một danh sách* các tác vụ có thể thất bại.
Ví dụ: Bạn có một danh sách `["12", "abc", "45"]`. Bạn muốn parse chúng thành số.

```rust
let strings = vec!["12", "abc", "45"];
let results: Vec<Result<i32, _>> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
```

Kết quả bạn nhận được là một `Vec<Result<i32, ParseIntError>>`.
Nhưng type này rất khó dùng! Để tính tổng, bạn phải duyệt qua mảng một lần nữa, mở từng cái `Result` ra.

Thứ chúng ta THỰC SỰ muốn là: **Nếu tất cả đều parse thành công, cho tôi MỘT cái mảng chứa toàn số. Nếu có BẤT KỲ lỗi nào, cho tôi cái lỗi đó.**
Nói cách khác, ta muốn **Lộn ngược (invert)** cấu trúc:
Biến đổi `Vec<Result<T, E>>` ➡️ `Result<Vec<T>, E>`.

Trong Toán học FP, thao tác lộn ngược này có một cái tên rất kêu: **Sequence**.

---

## 32b.1 — Sequence: Lộn ngược cấu trúc

Ở Haskell hay TypeScript (`fp-ts`), bạn phải gọi một hàm đặc biệt tên là `sequence`.
Nhưng ở Rust, thiết kế của ngôn ngữ đã đi trước một bước. Hàm **`.collect()`** đã được lập trình sẵn (thông qua trait `FromIterator`) để hiểu được `Result` và `Option`!

```rust
// filename: src/rust_sequence.rs

fn main() {
    // 1. Mảng toàn Ok
    let oks: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
    
    // SEQUENCE: Ép kiểu đích là Result<Vec<i32>, &str>
    // collect() tự động gom Ok lại, trả về Ok(Vec)
    let sequenced_oks: Result<Vec<i32>, &str> = oks.into_iter().collect();
    assert_eq!(sequenced_oks, Ok(vec![1, 2, 3]));

    // 2. Mảng có chứa Err
    let errs: Vec<Result<i32, &str>> = vec![Ok(1), Err("Lỗi ở đây!"), Ok(3)];
    
    // SEQUENCE fail-fast: Nó sẽ dừng ngay khi gặp Err đầu tiên!
    let sequenced_errs: Result<Vec<i32>, &str> = errs.into_iter().collect();
    assert_eq!(sequenced_errs, Err("Lỗi ở đây!"));
    
    println!("Sequence OK ✅");
}
```

> **💡 Bản chất**: `.collect::<Result<Vec<T>, E>>()` chính là **Monadic Sequence**. Nó duyệt mảng, nhặt từng giá trị bỏ vào mảng mới. Nhưng vì là Monadic, nó áp dụng cơ chế "Fail-fast": Chạm mặt `Err` đầu tiên là thoát và trả về `Err` luôn.

---

## 32b.2 — Traverse = Map + Sequence

Thông thường, ta không có sẵn một `Vec<Result>`. Ta có một mảng giá trị (vd: mảng String), và ta map một hàm sinh ra `Result` (vd: `parse()`).

`Traverse` (Duyệt) là thao tác: **Map** hàm đó lên mảng, sinh ra `Vec<Result>`, rồi ngay lập tức **Sequence** nó thành `Result<Vec>`.

Ở Rust, Traversing cực kỳ mượt mà, chỉ là chaining hai hàm iterator:

```rust
// filename: src/rust_traverse.rs

fn parse_even(s: &str) -> Result<i32, String> {
    let n = s.parse::<i32>().map_err(|e| e.to_string())?;
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(format!("Số {} không phải số chẵn", n))
    }
}

fn main() {
    let inputs = vec!["2", "4", "6"];
    
    // TRAVERSE!
    // Bước 1: .map(parse_even) sinh ra Iterator của Result
    // Bước 2: .collect() lộn ngược Iterator đó thành Result<Vec>
    let result: Result<Vec<i32>, String> = inputs
        .into_iter()
        .map(parse_even)
        .collect();
        
    assert_eq!(result, Ok(vec![2, 4, 6]));

    // Thử với mảng có lỗi
    let bad_inputs = vec!["2", "abc", "5"];
    let bad_result: Result<Vec<i32>, String> = bad_inputs
        .into_iter()
        .map(parse_even)
        .collect();
        
    // Dừng lại ngay lỗi đầu tiên (ParseIntError)
    assert_eq!(bad_result, Err("invalid digit found in string".to_string()));
}
```

---

## 32b.3 — Applicative Traverse: Thu thập mọi lỗi

Nhược điểm của `.collect::<Result<_, _>>()` là nó là **Monadic (Fail-fast)**.
Nếu ta muốn validate một danh sách các phần tử, và nếu có nhiều phần tử lỗi, ta muốn gom TẤT CẢ lỗi lại (Collect-all) thay vì chỉ lấy lỗi đầu tiên?

Ta không thể dùng `.collect()` mặc định nữa! Ta phải tự viết một hàm **Applicative Traverse** dựa trên cấu trúc `Validated` (hoặc `Result<T, Vec<E>>`) mà ta học ở chương trước.

```rust
// filename: src/applicative_traverse.rs

// Bí danh Applicative Validation
type Validated<T, E> = Result<T, Vec<E>>;

// Hàm zip quen thuộc từ Ch31b
fn zip_validated<A, B, E>(a: Validated<A, E>, b: Validated<B, E>) -> Validated<(A, B), E> {
    match (a, b) {
        (Ok(a), Ok(b)) => Ok((a, b)),
        (Err(mut ea), Err(mut eb)) => { ea.append(&mut eb); Err(ea) },
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

// ── APPLICATIVE TRAVERSE ──
// Nhận vào một Iterator, và một hàm sinh ra Validated.
// Trả về Validated của Vec.
fn traverse_validated<I, T, U, E, F>(iter: I, f: F) -> Validated<Vec<U>, E>
where
    I: IntoIterator<Item = T>,
    F: Fn(T) -> Validated<U, E>,
{
    // Bắt đầu với một mảng rỗng thành công Ok(vec![])
    let init: Validated<Vec<U>, E> = Ok(Vec::new());

    // Fold qua Iterator
    iter.into_iter().fold(init, |acc_val, item| {
        // Áp dụng hàm f lên item
        let item_val = f(item);
        
        // Zip kết quả (Mảng tích lũy, Giá trị hiện tại)
        // Sau khi zip thành công, đẩy giá trị mới vào mảng
        zip_validated(acc_val, item_val).map(|(mut vec, new_item)| {
            vec.push(new_item);
            vec
        })
    })
}

// Hàm test
fn validate_even(n: i32) -> Validated<i32, String> {
    if n % 2 == 0 { Ok(n) } 
    else { Err(vec![format!("{} is odd", n)]) }
}

fn main() {
    let numbers = vec![2, 3, 4, 5, 6];
    
    // Thực hiện Applicative Traverse
    let result = traverse_validated(numbers, validate_even);
    
    // NÓ THU THẬP ĐƯỢC CẢ 2 LỖI!
    println!("{:#?}", result);
    // Err([
    //     "3 is odd",
    //     "5 is odd",
    // ])
}
```

> **💡 Nhận xét**: `.fold()` chính là trái tim của Traverse. Bạn bắt đầu với `Ok([])`. Bạn duyệt qua mảng, và dùng `zip` để kết hợp trạng thái hiện tại với kết quả của phần tử mới. Bản chất đây chính là Catamorphism (Chapter 30c) áp dụng lên List!

---

## ✅ Checkpoint 32b.1-32b.3

> Đến đây bạn phải hiểu:
> 1. **Sequence**: Biến `Vec<Result<T, E>>` thành `Result<Vec<T>, E>`.
> 2. **Rust `.collect::<Result<_, _>>()`**: Là thao tác Monadic Sequence được bọc đường cú pháp (syntactic sugar) cực kỳ đẹp của Rust. Fail-fast!
> 3. **Traverse**: `map` kết hợp với `sequence`. (Trong Rust: `.map(f).collect()`).
> 4. **Applicative Traverse**: Dùng `.fold()` kết hợp hàm `zip` để lộn ngược cấu trúc mà không bị mất dữ liệu lỗi, hỗ trợ thu thập toàn bộ (Collect-all).
>
> **Test nhanh**: Nếu tôi có mảng `Vec<Option<T>>`, tôi có thể dùng `.collect()` để biến nó thành `Option<Vec<T>>` không?
> <details><summary>Đáp án</summary>CÓ THỂ! Bất kỳ Monad nào triển khai `FromIterator` đều có thể collect. `.collect::<Option<Vec<T>>>()` sẽ trả về `None` nếu có BẤT KỲ phần tử nào trong mảng là `None`. Rất tiện lợi!</details>

---

## 🏋️ Bài tập

**Bài 1** (15 phút): Parallel Task Simulation

```rust
// Tưởng tượng bạn đang gọi API để lấy thông tin của danh sách User IDs.
fn fetch_user(id: u32) -> Result<String, String> {
    if id == 404 {
        Err(format!("User {} not found", id))
    } else if id == 500 {
        Err(format!("Server error on {}", id))
    } else {
        Ok(format!("User {}", id))
    }
}

// YÊU CẦU:
// Hãy viết một hàm `fetch_all_monadic(ids: Vec<u32>) -> Result<Vec<String>, String>`
// dùng `.into_iter().map().collect()` (Fail-fast).
// Test nó với mảng `vec![1, 404, 500]`. Lỗi nào sẽ được trả về?
```

<details><summary>✅ Lời giải Bài 1</summary>

```rust
fn fetch_all_monadic(ids: Vec<u32>) -> Result<Vec<String>, String> {
    ids.into_iter()
        .map(fetch_user)
        .collect()
}

fn main() {
    let ids = vec![1, 404, 500];
    let result = fetch_all_monadic(ids);
    
    // Sẽ trả về lỗi ĐẦU TIÊN gặp phải. (User 404 not found)
    // Task 500 không bao giờ được execute nếu nó là iterator lazy (map), 
    // .collect() short-circuit ngay khi gặp 404.
    assert_eq!(result, Err("User 404 not found".to_string()));
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Lỗi "value of type `Result<Vec<T>, E>` cannot be built from `std::iter::Iterator<Item=Result<T, E>>`" | Trình biên dịch không đoán được kiểu đích. | Cung cấp type hint cho `.collect()`. Có 2 cách: `let x: Result<Vec<T>, E> = ...collect();` hoặc dùng turbofish: `.collect::<Result<Vec<T>, E>>()`. |
| Muốn Traverse một mảng nhưng bỏ qua các Err (chỉ giữ Ok) thay vì trả về lỗi | Hiểu sai công cụ | Traverse/Sequence lộn ngược cấu trúc. Nếu bạn chỉ muốn **lọc**, hãy dùng `.filter_map(Result::ok)` hoặc `.flatten()`. Đừng dùng collect ra Result. |

---

## Tóm tắt

- ✅ **Lộn ngược cấu trúc (Inversion of Control)**: Khi đối mặt với `Vec<Result>`, đừng hoảng loạn mở vòng `for`. Hãy dùng `Sequence` để kéo `Result` ra ngoài mảng.
- ✅ Rust hỗ trợ Sequence tận răng qua `.collect()`. Nó là Monadic, ngắn gọn, và an toàn nhờ cơ chế short-circuit.
- ✅ Nếu bài toán yêu cầu **Thu thập lỗi (Collect-all)**, ta không thể dựa vào Iterator có sẵn. Ta dùng **Applicative Traverse** bằng cách thiết lập một `.fold()` đi qua mảng, kết hợp từng giá trị bằng hàm `zip` (cộng gộp mảng lỗi).

## Tiếp theo

Bạn đã đi đến cuối chặng đường FP Patterns trên Rust! Từ `pipe` đến `Option/Result`, từ `Result` đến `Monad ?`, và cuối cùng là `Applicative` cùng `Traverse/Sequence`. Khung tư duy của bạn giờ đây đã sẵn sàng để đối phó với những luồng dữ liệu lồng nhau phức tạp nhất mà không phải viết một dòng `unwrap()` bẩn nào!
