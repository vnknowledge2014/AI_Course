# 🦀 Lời đề tựa

## Tại sao cuốn sách này tồn tại?

Hãy hình dung bạn đang xây một ngôi nhà. Bạn có thể học cách đóng đinh, cưa gỗ, trộn xi măng — những kỹ năng riêng lẻ. Hoặc bạn có thể học **tư duy kiến trúc** — cách nhìn tổng thể, cách biến một mảnh đất trống thành nơi ở an toàn, đẹp, và bền vững.

Cuốn sách này dạy bạn **tư duy kiến trúc phần mềm**, không chỉ cú pháp của ngôn ngữ lập trình.

Chúng tôi kết hợp **bốn cuốn sách kinh điển** mà cộng đồng FP thế giới đã công nhận:

| Nguồn | Đóng góp |
|-------|----------|
| **Domain Modeling Made Functional** (Scott Wlaschin) | DDD + Type-driven design |
| **FP Made Easier** (Charles Scalfani) | Nền tảng FP từ zero |
| **Learn Go with Tests** (Chris James) | TDD methodology |
| **F# for Fun and Profit** (Scott Wlaschin) | ROP, Monoids, Parser Combinators |

...và chuyển thể toàn bộ sang **Rust** — ngôn ngữ mà bạn vừa viết code an toàn, vừa đạt hiệu suất ngang C/C++.

## Cuốn sách này dành cho ai?

- 🧒 **Người mới hoàn toàn** — chưa biết lập trình, hoặc biết một ít
- 🎓 **Sinh viên** — muốn nền tảng vững chắc từ CS đến Production
- 💼 **Developer OOP** — muốn chuyển sang tư duy Functional
- 🏗️ **Senior/Principal** — muốn DDD, System Design, và Production patterns

Không quan trọng bạn ở đâu — **mọi lứa tuổi, mọi trình độ** đều có điểm xuất phát phù hợp.

## Triết lý viết sách

> *"Sách hay không phải sách viết đúng một cách máy móc — mà sách viết DỄ HIỂU."*

Mỗi chapter tuân theo nguyên tắc:

1. **Ẩn dụ trước, code sau** — "Ownership giống như ai giữ chìa khóa nhà"
2. **Từ cụ thể đến trừu tượng** — ví dụ bằng café, đèn giao thông, trước khi nói lý thuyết
3. **Code chạy được** — mọi đoạn code đều có output comment, copy-paste là chạy
4. **Checkpoint kiểm tra** — dừng lại, tự hỏi "mình đã hiểu chưa?"
5. **Bài tập tăng dần** — 5 phút → 10 phút → 15 phút
6. **Troubleshooting** — lỗi thường gặp ở cuối mỗi chapter

---

# 📖 Hướng dẫn đọc sách

## Cấu trúc sách

```
    +--------------------------------------------+
    |      Chapter 0: Rust in 10 Minutes          |
    |   (Doc truoc neu chua biet Rust - 10 phut)  |
    +----------------------+---------------------+
                           |
                           v
    +--------------------------------------------+
    |        PART 0: CS Foundations               |
    |   Ch 1-3  |  Math, Algorithms, Data Struct  |
    |   Level: Pre-requisite                      |
    +----------------------+---------------------+
                           |
                           v
    +--------------------------------------------+
    |        PART I: Rust Fundamentals            |
    |   Ch 4-11  |  Variables -> Ownership ->     |
    |              Error Handling -> Modules      |
    |   Level: Beginner                           |
    +----------------------+---------------------+
                           |
                           v
    +--------------------------------------------+
    |       PART II: Thinking Functionally        |
    |   Ch 12-17  |  Immutability, HOF, Traits,   |
    |               Generics, Pattern Matching    |
    |   Level: Intermediate                       |
    +-----+----------------+----------------+----+
          |                |                |
          v                v                v
    +-----------+  +--------------+  +------------+
    | PART III  |  |   PART IV    |  |  PART V    |
    | Design    |  | DDD with     |  | FP         |
    | Patterns  |  | Rust         |  | Patterns   |
    | Ch 18-19  |  | Ch 20-27     |  | Ch 28-32   |
    | Advanced  |  | Advanced     |  | Advanced   |
    +-----+-----+  +------+------+  +-----+------+
          |                |               |
          +----------------+---------------+
                           |
                           v
    +--------------------------------------------+
    |      PART VI: Testing & Engineering         |
    |   Ch 33-36  |  TDD, PBT, Mocking, Async     |
    |   Ch 36B    |  Web Services with Axum       |
    |   Level: Principal                          |
    +----------------------+---------------------+
                           |
                           v
    +--------------------------------------------+
    |     PART VII: Production Engineering        |
    |   Ch 37-44  |  Database, Security,          |
    |               System Design, Capstone       |
    |   Level: Principal                          |
    +--------------------------------------------+
```

## Bạn nên bắt đầu từ đâu?

### 🟢 Người mới bắt đầu (Beginner)

> *"Tôi chưa biết lập trình, hoặc mới biết một ít."*

```
    Ch0 ------> Ch1-3 ------> Ch4-11 ------> Dung lai, lam bai tap
    10 min      Nền tảng       Rust cơ bản    Tổng: ~2-3 tuần
```

**Cách đọc:**
- Đọc **TẤT CẢ** từ Chapter 0, **theo thứ tự**
- Mỗi ngày đọc **1 chapter**, làm hết bài tập
- Không hiểu? Đọc lại phần ẩn dụ (analogy), xem code output
- Gặp Checkpoint → tự trả lời trước khi xem đáp án
- **Không skip** — mỗi chapter xây trên chapter trước

### 🔵 Developer có kinh nghiệm (Intermediate)

> *"Tôi biết một ngôn ngữ khác, muốn học Rust + FP."*

```
    Ch0 ----> Skim Ch1-3 ----> Ch4-11 ----> Ch12-17 ----> Ch18-19
    10 min    Đọc nhanh        Nếu biết      Functional    Patterns
                               basics:       Thinking
                               skip 4-6
```

**Cách đọc:**
- Chapter 0: đọc 10 phút để nắm syntax
- Part 0 (Ch1-3): **skim** nếu bạn đã biết Big-O, recursion
- Part I (Ch4-11): đọc kỹ **Ch9 (Ownership)** — đây là nơi mọi dev đều cần học
- Part II trở đi: đọc tuần tự, đây là phần chính
- Bài tập: **Bài 2 và 3** mỗi chapter (skip Bài 1 nếu quá dễ)

### 🟣 Senior Developer (Advanced)

> *"Tôi biết FP cơ bản, muốn DDD + advanced patterns trong Rust."*

```
    Ch0 ----> Ch9 -------> Ch12-17 ----> Ch18-27 ----> Ch28-32
    Syntax    Ownership    FP basics     DDD           FP deep
    10 min    1 ngày       Skim/read     FOCUS         dive
```

**Cách đọc:**
- Skip Part 0 hoàn toàn (hoặc dùng làm reference)
- **Ch9 Ownership**: bắt buộc dù bạn giỏi đến đâu — đây là Rust-unique
- Part II: đọc nhanh tìm Rust-specific idioms (`Fn`/`FnMut`/`FnOnce`, trait bounds)
- **Part III + IV: ĐÂY LÀ PHẦN CHÍNH** — DDD, CQRS, ROP, Persistence
- Part V: đọc kỹ nếu muốn hiểu Functor/Monad trong Rust context

### 🔴 Principal / Architect (Principal)

> *"Tôi muốn system design, production patterns, security."*

```
    Ch0+Ch9 ----> Ch18-27 ----> Ch33-36 -----> Ch37-44
    Syntax +      DDD core      Testing        PRODUCTION
    Ownership     patterns      engineering    The goal
    1 ngày        1 tuần        3 ngày         2 tuần
```

**Cách đọc:**
- Syntax + Ownership = 1 ngày (Ch0 + Ch9)
- Skip thẳng tới Part III-IV cho DDD patterns
- Part VI (Testing): đọc kỹ Ch34 (PBT) và Ch35 (Hexagonal)
- **Part VII: ĐÂY LÀ MỤC TIÊU** — DB, Security, Distributed Systems, Capstone

---

## Quy ước trong sách

| Ký hiệu | Ý nghĩa |
|----------|---------|
| 💡 | Mẹo hoặc insight quan trọng |
| ⭐ | Chapter đặc biệt quan trọng |
| ✅ Checkpoint | Dừng lại tự kiểm tra |
| 🏋️ Bài tập | Thực hành (có lời giải) |
| 🔧 Troubleshooting | Lỗi thường gặp + cách sửa |
| 📋 Tóm tắt | Tổng hợp kiến thức cuối chapter |
| → Tiếp theo | Link sang chapter tiếp |

## Code trong sách

```rust
// Code Rust trong sách dùng quy ước:
// - Tên biến/function: tiếng Anh (chuẩn Rust)
// - Comment giải thích: tiếng Việt
// - Output: luôn có comment ở cuối

fn area(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius  // tính diện tích hình tròn
}

fn main() {
    let a = area(5.0);
    println!("Area: {:.2}", a);
    // Output: Area: 78.54
}
```

## Thời gian ước tính

| Trình độ | Phần đọc | Thời gian |
|----------|----------|-----------|
| Beginner | Part 0-I (Ch0-11) | 2-3 tuần |
| Intermediate | Part II (Ch12-17) | 1 tuần |
| Advanced | Part III-V (Ch18-32) | 3-4 tuần |
| Principal | Part VI-VII (Ch33-44) | 2-3 tuần |
| **Toàn bộ sách** | **Ch0-44** | **~8-11 tuần** |

> **Lời khuyên cuối**: Đừng vội. Mỗi chapter là một viên gạch. Xây chắc nền tảng trước, tầng trên tự vững.

---

*Chúc bạn một hành trình coding thú vị!* 🦀
