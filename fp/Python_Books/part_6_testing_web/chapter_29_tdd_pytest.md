# Chapter 29 — TDD & Test Pyramid với pytest

> **Bạn sẽ học được**:
> - Khái niệm **Test Pyramid** và tại sao Functional Programming thống trị ở tầng Unit Test.
> - Vòng lặp **TDD (Test-Driven Development)**: Red → Green → Refactor.
> - Cấu trúc một bài test chuẩn mực: **Arrange - Act - Assert** (AAA).
> - Tại sao Pure Functions giúp bạn nói không với Mocking/Stubbing.
> - Tận dụng sức mạnh của **`pytest`**: Fixtures và Parametrize.
>
> **Yêu cầu trước**: Nắm vững Pure Functions (Chapter 11).
> **Thời gian đọc**: ~35 phút | **Level**: Principal
> **Kết quả cuối cùng**: Hiểu sâu sắc triết lý viết test, không chỉ dừng lại ở cú pháp `assert`.

---

Khi bước vào giai đoạn Production (Part VI và Part VII), khả năng viết mã không còn quan trọng bằng khả năng **chứng minh mã của bạn chạy đúng**. 
Testing không phải là "việc vặt" sau khi code xong. Trong những hệ thống lớn, **Testing là thiết kế** (Testing is Design).

## 29.1 — Test Pyramid & TDD Philosophy

Test Pyramid (Kim tự tháp kiểm thử) là một mô hình nổi tiếng giúp định hình chiến lược viết test:
1. **Unit Tests (Đáy kim tự tháp)**: Chiếm số lượng lớn nhất (70-80%). Chạy cực nhanh, test các hàm độc lập.
2. **Integration Tests (Giữa)**: Kiểm tra sự giao tiếp giữa các module (ví dụ: code của bạn kết nối với Database hoặc API bên thứ ba). Chạy chậm hơn.
3. **E2E Tests (Đỉnh)**: (End-to-End). Mô phỏng thao tác của người dùng trên UI hoặc luồng API hoàn chỉnh. Chiếm số lượng ít nhất vì chạy rất chậm và dễ bị "flaky" (lúc pass lúc fail).

**Functional Programming tỏa sáng rực rỡ nhất ở tầng Unit Test**. Nhờ tính thuần khiết (Purity), bạn có thể nhét đầu vào (Input) và dự đoán chính xác đầu ra (Output) mà không cần quan tâm đến trạng thái của hệ thống hay các kết nối bên ngoài.

### Vòng lặp TDD (Red → Green → Refactor)

TDD (Test-Driven Development) lật ngược quy trình truyền thống. Bạn **không viết code trước**. Bạn viết test trước!

1. **🔴 RED (Viết Test trước)**: Nghĩ về *yêu cầu* (requirement). Viết một bài test cho yêu cầu đó. Chạy test, và tất nhiên nó phải **FAIL** (báo lỗi màu đỏ), vì hàm thậm chí còn chưa tồn tại.
2. **🟢 GREEN (Viết Code để Pass)**: Viết một đoạn code *đơn giản nhất, xấu xí nhất* miễn là làm cho test vượt qua (màu xanh). Mục tiêu lúc này không phải là hoàn hảo, mà là đúng.
3. **♻️ REFACTOR (Tối ưu Code)**: Dọn dẹp lại đoạn code vừa viết. Áp dụng Design Patterns, cấu trúc lại thuật toán. Nhờ có bài test (Green) bảo vệ, bạn có thể tự tin sửa code mà không sợ làm hỏng logic.

```python
# 🔴 Step 1: RED
def test_add():
    # Hàm add chưa tồn tại, trình biên dịch sẽ chửi hoặc test sẽ fail
    assert add(2, 3) == 5

# 🟢 Step 2: GREEN
def add(a: int, b: int) -> int:
    # Viết hàm ngu ngốc nhất có thể để pass
    return a + b

# ♻️ Step 3: REFACTOR
# Thêm Type hints, xử lý edge cases... Test vẫn PASS.
def test_add_edge_cases():
    assert add(2, 3) == 5
    assert add(0, 0) == 0
    assert add(-1, 1) == 0
```

---

## 29.2 — Tại sao FP làm Testing trở nên dễ dàng?

Trong OOP truyền thống hoặc mã Imperative (mệnh lệnh), hàm thường có **Side Effects** (gọi API, đọc file, đổi global variable). Để test chúng, bạn phải sử dụng **Mocks** (Giả mạo hành vi) hoặc **Stubs** (Dữ liệu giả).

Nhược điểm của Mock là nó khiến bài test bị gắn chặt (coupled) vào *cách cài đặt* (implementation details) thay vì kết quả. Khi bạn refactor code, test bị vỡ dù logic vẫn đúng!

Ngược lại, **Pure Functions** (Hàm thuần khiết) là một giấc mơ đối với TDD.
- Không cần setup database.
- Không cần Mock `requests.get`.
- Không cần dọn dẹp (teardown) sau khi chạy.

```python
# MỘT HÀM PURE FUNCTION TRONG FP
def calculate_discount(price: float, percent: float) -> float:
    return price * (1 - percent / 100)

# Việc test cực kỳ nhàm chán (và đó là một điều TỐT!)
def test_discount_10_percent():
    assert calculate_discount(100_000, 10) == 90_000

def test_discount_zero():
    assert calculate_discount(100_000, 0) == 100_000

def test_discount_100_percent():
    assert calculate_discount(100_000, 100) == 0
```

---

## 29.3 — Arrange-Act-Assert (AAA Pattern)

Bất kỳ bài Unit Test nào cũng nên được chia làm 3 khối rõ ràng:
1. **Arrange**: Chuẩn bị dữ liệu đầu vào, khởi tạo object.
2. **Act**: Thực thi hành động cần test (chỉ nên gọi đúng 1 hàm duy nhất).
3. **Assert**: Kiểm tra kết quả có đúng như kỳ vọng hay không.

```python
def test_confirm_order():
    # 1. Arrange: Chuẩn bị state ban đầu
    order = {"status": "pending", "items": ["Coffee"]}

    # 2. Act: Gọi pure function để tạo state mới (Immutable update)
    confirmed = {**order, "status": "confirmed"}

    # 3. Assert: Kiểm tra state mới
    assert confirmed["status"] == "confirmed"
    assert confirmed["items"] == order["items"]
```

---

## 29.4 — Pytest: Vũ khí tối thượng

`pytest` là framework testing phổ biến và mạnh mẽ nhất của Python. Bạn không cần dùng `unittest` rườm rà của thư viện chuẩn.

### 1. Parametrize: Test theo lô
Thay vì viết 10 hàm `test_xxx` cho 10 trường hợp, bạn có thể truyền data dạng bảng. Rất hữu dụng khi test Data-driven workflows!

```python
import pytest
from dataclasses import dataclass

@dataclass(frozen=True)
class Money:
    amount: int
    currency: str = "VND"
    
    def add(self, other: "Money") -> "Money":
        if self.currency != other.currency:
            raise ValueError("Mismatched currencies")
        return Money(self.amount + other.amount, self.currency)

# Test theo bảng (Data-Driven Testing)
@pytest.mark.parametrize("a, b, expected", [
    (100, 200, 300),
    (0, 0, 0),
    (1000, -500, 500),
])
def test_money_add_success(a: int, b: int, expected: int):
    assert Money(a).add(Money(b)) == Money(expected)
    
# Test bắt lỗi (Exception)
def test_money_add_error():
    a = Money(100, "VND")
    b = Money(200, "USD")
    with pytest.raises(ValueError, match="Mismatched currencies"):
        a.add(b)
```

### 2. Fixtures: Chia sẻ Setup Data
Trong FP, chúng ta thường làm việc với các Value Objects lớn (như một cái Cart chứa nhiều Item). Bạn có thể dùng `fixture` để tạo data dùng chung cho nhiều test.

```python
@pytest.fixture
def sample_cart():
    # Setup data
    return {"customer": "An", "items": [("Coffee", 35_000)], "total": 35_000}

# Truyền tên fixture vào làm tham số, pytest sẽ tự động inject (DI)
def test_order_total(sample_cart):
    assert sample_cart["total"] == 35_000
    
def test_add_item_to_cart(sample_cart):
    # Act
    new_cart = add_item(sample_cart, ("Tea", 20_000))
    # Assert
    assert new_cart["total"] == 55_000
```

---

---

## ✅ Checkpoint 29

1. Vì sao code FP dễ test hơn code OOP có trạng thái?
2. Bước "Refactor" trong Red-Green-Refactor hay bị bỏ qua nhất. Bỏ nó đi thì mất gì?
3. Khi nào nên dùng `pytest.fixture` thay vì tạo dữ liệu ngay trong test?

<details>
<summary>Đáp án</summary>

1. Vì hàm thuần chỉ cần *đầu vào → kiểm đầu ra*. Không phải dựng object, không phải mock dependency, không phải dọn state giữa các test. Test trở thành bảng ánh xạ.
2. Mất chính lợi ích chính của TDD. Test là **lưới an toàn cho phép refactor**; không refactor thì bạn chỉ đang trả giá cho lưới mà không bao giờ dùng.
3. Khi việc chuẩn bị dữ liệu lặp lại **và** không phải trọng tâm của test. Nếu giá trị cụ thể quan trọng cho việc hiểu test, hãy để ngay trong test — fixture giấu nó đi sẽ khiến test khó đọc.
</details>

---

## 🏋️ Bài tập

**Bài 1 (5 phút).** Viết test **trước** cho hàm `slugify(title: str) -> str`, chạy để thấy đỏ, rồi mới cài đặt.

**Bài 2 (10 phút).** Dùng `@pytest.mark.parametrize` gộp 6 test gần giống nhau thành một. So sánh số dòng.

**Bài 3 (20 phút).** Chọn một hàm trong dự án bạn có coverage 0%. Viết test cho nó **trước khi** đọc phần cài đặt — chỉ dựa vào tên hàm và chữ ký. Bạn đoán sai chỗ nào? Đó là chỗ thiết kế đang không tự giải thích được.

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| Test pass riêng, fail khi chạy cả bộ | Chia sẻ state giữa các test | Dùng fixture với scope `function`; tránh biến module-level |
| `ModuleNotFoundError` khi chạy pytest | Thiếu cấu hình rootdir | Đặt `[tool.pytest.ini_options] pythonpath = ["src"]` |
| Test chậm | Mỗi test dựng lại DB/app | Nâng scope fixture lên `session` cho thứ bất biến |
| Coverage cao nhưng bug vẫn lọt | Test gọi code mà không assert gì đáng kể | Đo chất lượng assertion, không chỉ đo % dòng |
| Test phụ thuộc thứ tự chạy | Có state ngầm | `pytest -p no:randomly` để xác nhận, rồi sửa gốc |

## Tóm tắt

- ✅ **TDD (Red → Green → Refactor)**: Giúp bạn tự tin đập đi xây lại code (Refactor) mà không sợ sinh bug mới.
- ✅ **Pure Functions**: Là chìa khóa để giảm bớt Mock/Stub rườm rà. Test của FP chạy độc lập, cực nhanh và dễ bảo trì.
- ✅ **Arrange-Act-Assert**: Đảm bảo mỗi bài test đều trong suốt, dễ đọc như một câu chuyện.
- ✅ **pytest**: Dùng `@pytest.mark.parametrize` cho Data-driven test, và `@pytest.fixture` để chia sẻ dữ liệu mô phỏng.

## Tiếp theo

Đôi khi, việc viết Parametrize với 5-10 test case vẫn chưa đủ an toàn. Nhỡ có một trường hợp (edge case) như số âm, `NaN`, hoặc chuỗi unicode kỳ dị lọt vào thì sao? 
Thay vì tự nghĩ ra test case, làm sao để máy tính **tự động sinh ra hàng vạn test case ngẫu nhiên** để bẻ gãy hàm của bạn? Hẹn gặp bạn ở **Chapter 30: Property-Based Testing**.
