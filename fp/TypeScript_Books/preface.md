# 📘 Lời đề tựa

## Tại sao cuốn sách này tồn tại?

Bạn đã bao giờ viết một ứng dụng JavaScript, chạy thử — hoạt động. Deploy lên production — crash. Lý do? `undefined is not a function`. Hay `Cannot read property 'x' of null`. Những lỗi mà trình biên dịch ĐÁNG LẼ phải bắt được, nhưng JavaScript không có khả năng đó.

TypeScript ra đời để giải quyết vấn đề này: thêm type system lên trên JavaScript. Nhưng hầu hết mọi người dùng TypeScript như JavaScript + vài annotations. Cuốn sách này dạy bạn dùng TypeScript **như một hệ thống thiết kế** — nơi types không chỉ ngăn bugs, mà còn **mô hình hóa business logic** để compiler trở thành đồng đội đáng tin cậy nhất.

Chúng tôi kết hợp **bốn cuốn sách kinh điển** mà cộng đồng FP thế giới đã công nhận:

| Nguồn | Đóng góp |
|-------|----------|
| **Domain Modeling Made Functional** (Scott Wlaschin) | DDD + Type-driven design |
| **FP Made Easier** (Charles Scalfani) | Nền tảng FP từ zero |
| **Learn Go with Tests** (Chris James) | TDD methodology |
| **F# for Fun and Profit** (Scott Wlaschin) | ROP, Monoids, Parser Combinators |

...và chuyển thể toàn bộ sang **TypeScript** — ngôn ngữ mà hàng triệu developers đang dùng hàng ngày, với hệ sinh thái web lớn nhất thế giới.

## Cuốn sách này dành cho ai?

- 🧒 **Người mới hoàn toàn** — chưa biết lập trình, hoặc biết một ít JavaScript
- 🎓 **Sinh viên** — muốn nền tảng vững chắc từ CS đến Production
- 💼 **Developer OOP** — muốn chuyển sang tư duy Functional
- 🏗️ **Senior/Principal** — muốn DDD, System Design, và Production patterns

Không quan trọng bạn ở đâu — **mọi lứa tuổi, mọi trình độ** đều có điểm xuất phát phù hợp.

## Triết lý viết sách

> *"Sách hay không phải sách viết đúng một cách máy móc — mà sách viết DỄ HIỂU."*

Mỗi chapter tuân theo nguyên tắc:

1. **Ẩn dụ trước, code sau** — "Discriminated Union giống như vé xe buýt có nhiều loại"
2. **Từ cụ thể đến trừu tượng** — ví dụ bằng e-commerce, todo app, trước khi nói lý thuyết
3. **Code chạy được** — mọi đoạn code đều có `assert` hoặc `console.log` output, copy-paste là chạy
4. **Checkpoint kiểm tra** — dừng lại, tự hỏi "mình đã hiểu chưa?"
5. **Bài tập tăng dần** — 5 phút → 10 phút → 15 phút
6. **Troubleshooting** — lỗi thường gặp ở cuối mỗi chapter

---

# 📖 Hướng dẫn đọc sách

## Cấu trúc sách

```
    +--------------------------------------------+
    |   Chapter 0: TypeScript in 10 Minutes        |
    |   (Doc truoc neu chua biet TS - 10 phut)     |
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
    |      PART I: TypeScript Fundamentals        |
    |   Ch 4-10  |  Types -> Narrowing ->         |
    |              Advanced Type System -> Modules |
    |   Level: Beginner                           |
    +----------------------+---------------------+
                           |
                           v
    +--------------------------------------------+
    |       PART II: Thinking Functionally        |
    |   Ch 11-15  |  Immutability, Composition,   |
    |               ADTs, Branded Types, Generics |
    |   Level: Intermediate                       |
    +-----+----------------+----------------+----+
          |                |                |
          v                v                v
    +-----------+  +--------------+  +------------+
    | PART III  |  |   PART IV    |  |  PART V    |
    | Design    |  | DDD with     |  | FP         |
    | Patterns  |  | TypeScript   |  | Patterns   |
    | Ch 16-17  |  | Ch 18-24     |  | Ch 25-30   |
    | Advanced  |  | Advanced     |  | Advanced   |
    +-----+-----+  +------+------+  +-----+------+
          |                |               |
          +----------------+---------------+
                           |
                           v
    +--------------------------------------------+
    |    PART VI: Testing & Full-Stack            |
    |   Ch 31-36  |  TDD, PBT, Architecture,     |
    |               Backend, Frontend, Capstone   |
    |   Level: Principal                          |
    +----------------------+---------------------+
                           |
                           v
    +--------------------------------------------+
    |     PART VII: Production Engineering        |
    |   Ch 37-43  |  Database, Security,          |
    |               System Design, Capstone       |
    |   Level: Principal                          |
    +--------------------------------------------+
```

## Bạn nên bắt đầu từ đâu?

### 🟢 Người mới bắt đầu (Beginner)

> *"Tôi chưa biết lập trình, hoặc mới biết một ít."*

```
    Ch0 ------> Ch1-3 ------> Ch4-10 ------> Dung lai, lam bai tap
    10 min      Nền tảng       TS cơ bản      Tổng: ~2-3 tuần
```

**Cách đọc:**
- Đọc **TẤT CẢ** từ Chapter 0, **theo thứ tự**
- Mỗi ngày đọc **1 chapter**, làm hết bài tập
- Không hiểu? Đọc lại phần ẩn dụ (analogy), xem code output
- Gặp Checkpoint → tự trả lời trước khi xem đáp án
- **Không skip** — mỗi chapter xây trên chapter trước

### 🔵 Developer có kinh nghiệm (Intermediate)

> *"Tôi biết JavaScript/một ngôn ngữ khác, muốn học TypeScript + FP."*

```
    Ch0 ----> Skim Ch1-3 ----> Ch4-10 ----> Ch11-15 ----> Ch16-17
    10 min    Đọc nhanh        Nếu biết     FP           Patterns
                               basics:      Thinking
                               skip 4-6
```

**Cách đọc:**
- Chapter 0: đọc 10 phút để nắm syntax
- Part 0 (Ch1-3): **skim** nếu bạn đã biết Big-O, recursion
- Part I (Ch4-10): đọc kỹ **Ch6 (Narrowing)** và **Ch9 (Advanced Types)** — đây là TypeScript-unique
- Part II trở đi: đọc tuần tự, đây là phần chính
- Bài tập: **Bài 2 và 3** mỗi chapter (skip Bài 1 nếu quá dễ)

### 🟣 Senior Developer (Advanced)

> *"Tôi biết TypeScript cơ bản, muốn DDD + advanced patterns."*

```
    Ch0 ----> Ch9 --------> Ch11-15 ----> Ch18-24 ----> Ch25-30
    Syntax    Advanced      FP basics     DDD           FP deep
    10 min    Types         Skim/read     FOCUS         dive
```

**Cách đọc:**
- Skip Part 0 hoàn toàn (hoặc dùng làm reference)
- **Ch9 Advanced Type System**: bắt buộc — conditional types, mapped types
- Part II: đọc nhanh tìm TS-specific idioms (branded types, Zod, discriminated unions)
- **Part III + IV: ĐÂY LÀ PHẦN CHÍNH** — DDD, CQRS, ROP, Persistence
- Part V: đọc kỹ nếu muốn hiểu fp-ts/Effect ecosystem

### 🔴 Principal / Architect (Principal)

> *"Tôi muốn system design, production patterns, security."*

```
    Ch0+Ch9 ----> Ch18-24 ----> Ch31-36 -----> Ch37-43
    Syntax +      DDD core      Testing        PRODUCTION
    Types         patterns      engineering    The goal
    1 ngày        1 tuần        3 ngày         2 tuần
```

**Cách đọc:**
- Syntax + Advanced Types = 1 ngày (Ch0 + Ch9)
- Skip thẳng tới Part III-IV cho DDD patterns
- Part VI (Testing): đọc kỹ Ch32 (PBT) và Ch33 (Architecture)
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

```typescript
// Code TypeScript trong sách dùng quy ước:
// - Tên biến/function: tiếng Anh (chuẩn TypeScript)
// - Comment giải thích: tiếng Việt
// - Output: luôn có assert hoặc console.log ở cuối

import assert from "node:assert/strict";

const area = (radius: number): number =>
    Math.PI * radius * radius;  // tính diện tích hình tròn

const result = area(5);
assert.ok(Math.abs(result - 78.54) < 0.01);
console.log(`Area: ${result.toFixed(2)}`);
// Output: Area: 78.54
```

## Thời gian ước tính

| Trình độ | Phần đọc | Thời gian |
|----------|----------|-----------|
| Beginner | Part 0-I (Ch0-10) | 2-3 tuần |
| Intermediate | Part II (Ch11-15) | 1 tuần |
| Advanced | Part III-V (Ch16-30) | 3-4 tuần |
| Principal | Part VI-VII (Ch31-43) | 2-3 tuần |
| **Toàn bộ sách** | **Ch0-43** | **~8-11 tuần** |

> **Lời khuyên cuối**: Đừng vội. Mỗi chapter là một viên gạch. Xây chắc nền tảng trước, tầng trên tự vững.

---

*Chúc bạn một hành trình coding thú vị!* 📘
