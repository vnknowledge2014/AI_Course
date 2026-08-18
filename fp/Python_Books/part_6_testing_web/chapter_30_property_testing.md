# Chapter 30 — Property-Based Testing (PBT) với Hypothesis

> **Bạn sẽ học được**:
> - Sự khác biệt cốt lõi giữa Example-based testing và Property-based testing.
> - Sử dụng thư viện `hypothesis` để tự động sinh ra hàng vạn test case.
> - Cơ chế **Shrinking** — phép thuật tìm ra lỗi nhỏ nhất có thể.
> - 4 mẫu Property thường gặp: Round-trip, Idempotent, Invariant, Oracle.
>
> **Yêu cầu trước**: Chapter 29 (TDD)
> **Thời gian đọc**: ~35 phút | **Level**: Principal
> **Kết quả cuối cùng**: Chuyển từ việc tự nghĩ ra test case sang việc định nghĩa luật (rules) và để máy tính tìm ra các góc chết (edge cases) hộ bạn.

---

Trong Chapter trước, bạn đã học cách dùng `@pytest.mark.parametrize` để cung cấp 3-5 test case. Đó gọi là **Example-based Testing**. Bạn tự nghĩ ra input và tự tính output.
Nhược điểm lớn nhất là: Trí tưởng tượng của con người có hạn. Bạn rất dễ quên test với chuỗi rỗng `""`, số âm `-1`, số `0`, hoặc một chuỗi Unicode kỳ dị `🤔`.

**Property-Based Testing (PBT)** thay đổi hoàn toàn cuộc chơi. 
Thay vì viết: *"Nếu tôi đưa vào 2 và 3, kết quả phải là 5"*.
Bạn viết: *"Nếu tôi đưa vào hai số nguyên `a` và `b` BẤT KỲ, kết quả `a + b` phải bằng `b + a`"*.

Bạn định nghĩa **Luật (Properties)**, còn framework (`hypothesis`) sẽ nã hàng vạn test case ngẫu nhiên vào hàm của bạn để cố gắng phá vỡ luật đó.

## 30.1 — Từ Example đến Property

Hãy xem cách kiểm tra một hàm đảo ngược danh sách (`reversed`).

```python
# 1. EXAMPLE-BASED: Bạn tự nghĩ ra một ví dụ cụ thể
def test_reverse_example():
    assert list(reversed([1, 2, 3])) == [3, 2, 1]
    assert list(reversed([])) == []

# 2. PROPERTY-BASED: Bạn định nghĩa TÍNH CHẤT (Property)
from hypothesis import given
from hypothesis import strategies as st

# Định nghĩa luật: Đảo ngược một danh sách 2 lần thì sẽ trở về như cũ!
# Cho BẤT KỲ danh sách số nguyên nào (st.lists(st.integers()))
@given(st.lists(st.integers()))
def test_reverse_involution(lst):
    assert list(reversed(list(reversed(lst)))) == lst

# Định nghĩa luật: Đảo ngược danh sách không làm thay đổi độ dài!
@given(st.lists(st.integers()))
def test_reverse_preserves_length(lst):
    assert len(list(reversed(lst))) == len(lst)
```

Khi chạy `pytest`, `hypothesis` sẽ tự động thử nghiệm mảng rỗng `[]`, mảng 1 phần tử `[0]`, mảng chứa số âm `[-1, -5]`, mảng cực lớn... Nếu có bất kỳ trường hợp nào làm fail `assert`, nó sẽ dừng lại và báo cho bạn!

---

## 30.2 — Strategies: Dạy máy tính cách sinh dữ liệu

Để `hypothesis` ném data vào hàm của bạn, bạn phải dùng các `strategies` (chiến lược sinh data) do thư viện cung cấp. 

```python
from hypothesis import strategies as st

# -- CÁC KIỂU CƠ BẢN --
# st.integers(min_value=0, max_value=100) # Số nguyên từ 0 đến 100
# st.floats(allow_nan=False)              # Số thực, không chứa NaN
# st.text(min_size=1)                     # Chuỗi không rỗng
# st.booleans()                           # True / False

# -- CÁC KIỂU TẬP HỢP --
# st.lists(st.integers())                 # [1, 5, -2, ...]
# st.dictionaries(st.text(), st.integers()) # {"abc": 1, "xyz": 2}

# -- KẾT HỢP VỚI DOMAIN MODEL --
from dataclasses import dataclass

@dataclass(frozen=True)
class Money:
    amount: int
    currency: str

# Dùng st.builds để sinh ra Object phức tạp
money_strategy = st.builds(
    Money,
    amount=st.integers(min_value=0, max_value=1_000_000),
    currency=st.sampled_from(["VND", "USD", "EUR"]),
)

@given(money_strategy, money_strategy)
def test_money_add_commutative(a: Money, b: Money):
    if a.currency == b.currency:
        sum1 = Money(a.amount + b.amount, a.currency)
        sum2 = Money(b.amount + a.amount, b.currency)
        assert sum1 == sum2
```

---

## 30.3 — Shrinking: Phép thuật tìm kiếm lỗi

Đây là tính năng đáng giá ngàn vàng của `hypothesis`.
Giả sử thuật toán tính tổng của bạn bị lỗi khi `a + b > 500`. 

Ban đầu, `hypothesis` ném ngẫu nhiên hai số `a = 4328` và `b = 9812` và phát hiện ra lỗi.
Thay vì báo ngay cho bạn hai con số khổng lồ kia (rất khó debug), `hypothesis` bắt đầu quá trình **Shrinking (Thu nhỏ)**. Nó sẽ giảm dần `a` và `b` xuống: thử `(0, 0)` -> pass, thử `(100, 100)` -> pass, thử `(300, 300)` -> fail... Cứ thế cho đến khi nó tìm ra **test case thất bại nhỏ nhất có thể**.

Cuối cùng nó sẽ báo lỗi cho bạn: `Falsifying example: test_addition(a=0, b=501)`.
Tính năng này giúp bạn debug cực kỳ nhàn nhã!

---

## 30.4 — Bốn mẫu Property thường gặp

Viết tính chất (Property) đôi khi khá khó vì bạn không thể dùng lại chính code của hàm cần test để test kết quả (như vậy là vô nghĩa). Dưới đây là 4 mẫu tư duy phổ biến:

### 1. Round-Trip (Đi rồi trở lại)
Test các quá trình có tính chất 2 chiều: `encode/decode`, `serialize/deserialize`.
```python
import json
@given(st.dictionaries(st.text(), st.integers()))
def test_json_roundtrip(data):
    # Biến dict thành chuỗi JSON, rồi biến ngược lại thành dict phải ra cái cũ!
    assert json.loads(json.dumps(data)) == data
```

### 2. Idempotent (Làm nhiều lần cũng như 1 lần)
Test các hàm thao tác trên dữ liệu mà gọi lần thứ 2 không làm thay đổi trạng thái.
```python
@given(st.lists(st.integers()))
def test_sort_is_idempotent(lst):
    # Sort một mảng đã sort thì kết quả vẫn vậy!
    assert sorted(sorted(lst)) == sorted(lst)
```

### 3. Invariant (Đặc tính bất biến)
Cho dù dữ liệu biến đổi thế nào, một số thuộc tính nhất định không bao giờ thay đổi. Ví dụ: map một mảng không bao giờ làm thay đổi số phần tử của nó.
```python
@given(st.lists(st.integers()))
def test_map_preserves_length(lst):
    mapped = list(map(lambda x: x * 2, lst))
    assert len(mapped) == len(lst)
```

### 4. Test Oracle (Kiểm chứng bằng cách khác)
So sánh một thuật toán mới (nhanh, phức tạp) với một thuật toán cũ (chậm, đơn giản, nhưng chắc chắn đúng).
```python
@given(st.lists(st.integers()))
def test_my_fancy_sort_against_builtin(lst):
    assert my_fancy_quicksort(lst) == sorted(lst)
```

---

---

## ✅ Checkpoint 30

1. Nêu ba loại property phổ biến và một ví dụ cho mỗi loại.
2. Shrinking làm gì, và vì sao thiếu nó thì PBT gần như vô dụng?
3. PBT thay thế hay bổ sung cho example-based test?

<details>
<summary>Đáp án</summary>

1. **Round-trip**: `decode(encode(x)) == x`. **Idempotent**: `sort(sort(x)) == sort(x)`. **Oracle**: so kết quả với một cài đặt chậm-nhưng-chắc-đúng. (Còn: invariant, metamorphic.)
2. Thu nhỏ ca lỗi về bản tối giản nhất còn fail. Thiếu nó, bạn nhận được một mảng 500 phần tử ngẫu nhiên và không biết phần tử nào gây lỗi — về cơ bản là không debug được.
3. **Bổ sung**. Example test ghi lại các ca cụ thể mà bạn quan tâm (và làm tài liệu); PBT lùng những ca bạn chưa nghĩ ra. Cần cả hai.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Viết property round-trip cho `to_dto`/`from_dto` của bạn bằng `hypothesis`. Nó có tìm ra ca lỗi nào không?

**Bài 2 (15 phút).** Viết strategy sinh `Email` hợp lệ, dùng nó test smart constructor. Rồi viết strategy sinh chuỗi **không** hợp lệ và khẳng định constructor luôn từ chối.

**Bài 3 (25 phút).** Dùng oracle: cài một `sort` ngây thơ O(n²) rồi so kết quả với `sorted()` trên 1.000 input ngẫu nhiên. Đây là mẫu dùng khi bạn tối ưu một hàm có sẵn.

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| `Unsatisfiable` từ hypothesis | `assume()` lọc quá gắt | Viết strategy sinh trực tiếp dữ liệu hợp lệ thay vì lọc |
| Test PBT chạy quá lâu | Số ví dụ mặc định quá cao cho hàm nặng | `@settings(max_examples=50)` |
| Fail không tái hiện được | Seed ngẫu nhiên | Bật `.hypothesis` database, dùng `@reproduce_failure` |
| PBT tìm ra "lỗi" mà thực ra không phải | Property phát biểu sai | Kiểm lại property trước khi sửa code — thường property mới là chỗ sai |
| Float so sánh luôn fail | Sai số dấu phẩy động | Dùng `math.isclose`; hoặc tránh float trong domain |

## Tóm tắt

- ✅ **PBT**: Đẩy trách nhiệm nghĩ test case cho máy tính, con người chỉ định nghĩa luật (Properties).
- ✅ **Hypothesis**: Thư viện số 1 của Python cho PBT. Dùng `@given` và `st.<strategy>`.
- ✅ **Shrinking**: `hypothesis` không chỉ tìm lỗi, nó tìm lỗi nhỏ nhất và đơn giản nhất để bạn dễ debug.
- ✅ **4 Patterns**: Round-trip, Idempotent, Invariant, Oracle là những mẫu thiết kế tính chất phổ biến nhất để áp dụng PBT.

## Tiếp theo

Testing đã xong. Giờ là lúc biến domain logic của chúng ta thành những dịch vụ Web Service. Làm sao để áp dụng toàn bộ kiến thức FP (Pure Functions, Results, Pydantic) vào một Framework nổi tiếng nhất hiện nay? 
Mời bạn đến với **Chapter 31: FastAPI + DDD**.
