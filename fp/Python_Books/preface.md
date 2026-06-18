# 🐍 Lời đề tựa

## Tại sao cuốn sách này tồn tại?

Python là ngôn ngữ dễ học nhất thế giới. Nhưng "dễ viết" không có nghĩa là "dễ maintain". Bạn đã bao giờ mở lại code Python mình viết 6 tháng trước và tự hỏi: "Cái này nhận vào kiểu gì? Trả về gì? Có thể None không?" Đó là lúc bạn cần **tư duy hệ thống** — không chỉ biết syntax, mà biết **thiết kế**.

Cuốn sách này dạy bạn **Functional Programming** và **Domain-Driven Design** trong Python — cách viết code rõ ràng, an toàn, dễ test, và dễ mở rộng. Không phải Python "cơ bản" hay Python "data science" — mà Python **engineering**.

Chúng tôi kết hợp **bốn cuốn sách kinh điển** mà cộng đồng FP thế giới đã công nhận:

| Nguồn | Đóng góp |
|-------|----------|
| **Domain Modeling Made Functional** (Scott Wlaschin) | DDD + Type-driven design |
| **FP Made Easier** (Charles Scalfani) | Nền tảng FP từ zero |
| **Learn Go with Tests** (Chris James) | TDD methodology |
| **F# for Fun and Profit** (Scott Wlaschin) | ROP, Monoids, Parser Combinators |

...và chuyển thể toàn bộ sang **Python** — ngôn ngữ với hệ sinh thái khổng lồ từ web (FastAPI, Django) đến AI/ML (PyTorch, TensorFlow) đến DevOps (Ansible, Salt).

## Cuốn sách này dành cho ai?

- 🧒 **Người mới hoàn toàn** — chưa biết lập trình, hoặc biết một ít Python
- 🎓 **Sinh viên** — muốn nền tảng vững chắc từ CS đến Production
- 💼 **Data Scientist / ML Engineer** — biết Python nhưng muốn viết production code tốt hơn
- 🏗️ **Senior/Principal** — muốn DDD, System Design, và Production patterns

Không quan trọng bạn ở đâu — **mọi lứa tuổi, mọi trình độ** đều có điểm xuất phát phù hợp.

## Tại sao FP trong Python?

Bạn có thể nghĩ: "Python đâu phải ngôn ngữ functional?" Đúng — Python không enforce immutability hay pure functions. Nhưng:

- Python có **first-class functions** (`lambda`, `map`, `filter`, `functools`)
- Python 3.10+ có **pattern matching** (`match/case`)
- Python 3.12+ có **type hints mạnh mẽ** (`mypy --strict`)
- `dataclasses(frozen=True)` = immutable records
- Thư viện `returns` = Railway-Oriented Programming
- `Pydantic` = smart constructors + validation + serialization

**FP trong Python không phải "ép cái tròn vào lỗ vuông" — mà là chọn đúng tools từ bộ đồ nghề sẵn có.**

## Triết lý viết sách

> *"Sách hay không phải sách viết đúng một cách máy móc — mà sách viết DỄ HIỂU."*

Mỗi chapter tuân theo nguyên tắc:

1. **Ẩn dụ trước, code sau** — "Dataclass frozen giống như hợp đồng đã ký — không ai sửa được"
2. **Từ cụ thể đến trừu tượng** — ví dụ bằng quán café, giao hàng, trước khi nói lý thuyết
3. **Code chạy được** — mọi đoạn code đều có `assert` hoặc `print` output, copy-paste là chạy
4. **Checkpoint kiểm tra** — dừng lại, tự hỏi "mình đã hiểu chưa?"
5. **Bài tập tăng dần** — 5 phút → 10 phút → 15 phút
6. **Troubleshooting** — lỗi thường gặp ở cuối mỗi chapter

---

# 📖 Hướng dẫn đọc sách

## Cấu trúc sách

```
    +--------------------------------------------+
    |        PART 0: CS Foundations               |
    |   Ch 1-3  |  Math, Algorithms, Data Struct  |
    |   Level: Pre-requisite                      |
    +----------------------+---------------------+
                           |
                           v
    +--------------------------------------------+
    |      PART I: Python Fundamentals            |
    |   Ch 4-10  |  Setup -> Types -> Functions   |
    |              -> Dataclasses -> Modules       |
    |   Level: Beginner                           |
    +----------------------+---------------------+
                           |
                           v
    +--------------------------------------------+
    |       PART II: Thinking Functionally        |
    |   Ch 11-15  |  Immutability, Composition,   |
    |               ADTs, Pydantic, Protocols     |
    |   Level: Intermediate                       |
    +-----+----------------+----------------+----+
          |                |                |
          v                v                v
    +-----------+  +--------------+  +------------+
    | PART III  |  |   PART IV    |  |  PART V    |
    | Design    |  | DDD with     |  | FP         |
    | Patterns  |  | Python       |  | Patterns   |
    | Ch 16-17  |  | Ch 18-24     |  | Ch 25-28   |
    | Advanced  |  | Advanced     |  | Advanced   |
    +-----+-----+  +------+------+  +-----+------+
          |                |               |
          +----------------+---------------+
                           |
                           v
    +--------------------------------------------+
    |      PART VI: Testing & Web                 |
    |   Ch 29-32  |  pytest, PBT, FastAPI,        |
    |               Capstone Domain               |
    |   Level: Principal                          |
    +----------------------+---------------------+
                           |
                           v
    +--------------------------------------------+
    |     PART VII: Production Engineering        |
    |   Ch 33-39  |  Database, Security,          |
    |               System Design, Capstone       |
    |   Level: Principal                          |
    +--------------------------------------------+
```

## Bạn nên bắt đầu từ đâu?

### 🟢 Người mới bắt đầu (Beginner)
```
    Ch1-3 ------> Ch4-10 ------> Dung lai, lam bai tap
    Nền tảng       Python cơ bản  Tổng: ~2-3 tuần
```

### 🔵 Developer có kinh nghiệm (Intermediate)
```
    Skim Ch1-3 ----> Ch4-10 ----> Ch11-15 ----> Ch16-17
    Đọc nhanh        skip 4-6     FP            Patterns
```

### 🟣 Senior Developer (Advanced)
```
    Ch9 --------> Ch11-15 ----> Ch18-24 ----> Ch25-28
    Dataclasses    FP basics     DDD           FP deep
```

### 🔴 Principal / Architect (Principal)
```
    Ch9 ----> Ch18-24 ----> Ch29-32 -----> Ch33-39
    Dataclass  DDD core     Testing        PRODUCTION
```

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

```python
# Code Python trong sách dùng quy ước:
# - Tên biến/function: tiếng Anh (chuẩn PEP 8)
# - Comment giải thích: tiếng Việt
# - Output: luôn có assert hoặc print ở cuối

import math

def area(radius: float) -> float:
    """Tính diện tích hình tròn."""
    return math.pi * radius * radius

result = area(5.0)
assert abs(result - 78.54) < 0.01
print(f"Area: {result:.2f}")
# Output: Area: 78.54
```

## Thời gian ước tính

| Trình độ | Phần đọc | Thời gian |
|----------|----------|-----------|
| Beginner | Part 0-I (Ch1-10) | 2-3 tuần |
| Intermediate | Part II (Ch11-15) | 1 tuần |
| Advanced | Part III-V (Ch16-28) | 3-4 tuần |
| Principal | Part VI-VII (Ch29-39) | 2-3 tuần |
| **Toàn bộ sách** | **Ch1-39** | **~8-11 tuần** |

> **Lời khuyên cuối**: Đừng vội. Mỗi chapter là một viên gạch. Xây chắc nền tảng trước, tầng trên tự vững.

---

*Chúc bạn một hành trình coding thú vị!* 🐍
