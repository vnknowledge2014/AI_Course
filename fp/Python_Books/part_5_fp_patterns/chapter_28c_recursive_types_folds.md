# Chapter 28c — Recursive Types & Folds (Catamorphisms)

> **Bạn sẽ học được**:
> - Kiểu đệ quy (Recursive Types) là gì: Linked Lists, Trees, ASTs.
> - Hạn chế của đệ quy truyền thống: Trộn lẫn logic duyệt (traversal) và xử lý (computation).
> - **Fold (Catamorphism)**: Pattern tối thượng tách rời việc duyệt cấu trúc khỏi xử lý dữ liệu.
> - Hiểu bản chất của `functools.reduce` — nó chính là fold trên List.
> - Cách viết fold cho một Binary Tree và cho một Abstract Syntax Tree (AST).
>
> **Yêu cầu trước**: Chương này độc lập hơn, nhưng hiểu về Functors (Ch26) sẽ giúp bạn thấy rõ cấu trúc.
> **Thời gian đọc**: ~35 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Nhìn thấy mọi đệ quy xử lý dữ liệu dưới hình hài của Fold. Code sạch, tách biệt hoàn toàn cấu trúc và logic.

---

Hãy nhớ lại cách bạn tính tổng một mảng `[1, 2, 3]`: bạn không viết vòng `while` đếm index, bạn dùng `sum()` hoặc `reduce(lambda a, b: a + b, lst)`. Bạn giao việc **duyệt mảng** cho `reduce`, còn bạn chỉ cung cấp **logic xử lý** (phép cộng).

Nhưng khi xử lý một Cây (Tree) hoặc một Cây Cú Pháp Trừu Tượng (AST), nhiều người lại tự viết các hàm đệ quy trộn lẫn giữa việc đi từ node cha xuống node con VÀ logic xử lý dữ liệu.

**Fold (Catamorphism)** trong FP là khái niệm tổng quát hóa của `reduce` cho BẤT KỲ cấu trúc dữ liệu nào. Nó giúp bạn: **"Đưa tôi một cấu trúc đệ quy, tôi sẽ bóc tách nó từ dưới lên trên thành một giá trị duy nhất (collapse), bạn chỉ cần cho tôi biết phải làm gì ở mỗi loại Node."**

---

## 28c.1 — Recursive Types (Kiểu Đệ Quy)

Kiểu đệ quy là một cấu trúc dữ liệu tự tham chiếu đến chính nó. 
Ví dụ đơn giản nhất là Linked List và Binary Tree.

```python
# filename: recursive_types.py
from dataclasses import dataclass
from typing import Union

# ── Linked List ──
@dataclass(frozen=True)
class Empty:
    pass

@dataclass(frozen=True)
class Node:
    value: int
    next: Union["Node", Empty]

LinkedList = Union[Node, Empty]

# list: 1 -> 2 -> 3 -> Empty
my_list = Node(1, Node(2, Node(3, Empty())))

# ── Binary Tree ──
@dataclass(frozen=True)
class Leaf:
    value: int

@dataclass(frozen=True)
class Branch:
    left: Union["Branch", "Leaf"]
    right: Union["Branch", "Leaf"]

Tree = Union[Branch, Leaf]

# tree:
#       /\
#      /  \
#     1   /\
#        /  \
#       2    3
my_tree = Branch(Leaf(1), Branch(Leaf(2), Leaf(3)))

print("Recursive Types OK ✅")
```

Khi định nghĩa kiểu trong Python, ta dùng string forward reference (ví dụ `"Node"`) vì class `Node` chưa được định nghĩa xong tại thời điểm khai báo type hint.

---

## 28c.2 — Vấn đề của Đệ quy truyền thống

Giả sử ta muốn:
1. Tính tổng các phần tử trong Tree.
2. Tìm giá trị lớn nhất trong Tree.

Cách thông thường:

```python
# filename: naive_recursion.py
from dataclasses import dataclass
from typing import Union

@dataclass(frozen=True)
class Leaf:
    value: int
@dataclass(frozen=True)
class Branch:
    left: Union["Branch", "Leaf"]
    right: Union["Branch", "Leaf"]
Tree = Union[Branch, Leaf]

my_tree = Branch(Leaf(1), Branch(Leaf(2), Leaf(3)))

# ── Tính tổng ──
def sum_tree(tree: Tree) -> int:
    match tree:
        case Leaf(value=v):
            return v
        case Branch(left=l, right=r):
            return sum_tree(l) + sum_tree(r) # <-- Lại gọi đệ quy

# ── Tìm Max ──
def max_tree(tree: Tree) -> int:
    match tree:
        case Leaf(value=v):
            return v
        case Branch(left=l, right=r):
            return max(max_tree(l), max_tree(r)) # <-- Vẫn lại gọi đệ quy

assert sum_tree(my_tree) == 6
assert max_tree(my_tree) == 3

print("Naive Recursion OK ✅")
```

**Vấn đề**: Nhìn vào `sum_tree` và `max_tree`. Phần code điều hướng cấu trúc (gọi đệ quy `f(l)` và `f(r)`) BỊ LẶP LẠI. Nếu cấu trúc Tree phức tạp hơn (ví dụ có 5 loại node), bạn sẽ phải copy-paste logic điều hướng đó cho MỌI hàm thao tác trên Tree.

Giải pháp: Tách logic điều hướng ra một hàm duy nhất. Gọi nó là **Fold**.

---

## 28c.3 — Tree Fold (Catamorphism)

### Tách rời cấu trúc và logic

Hàm `fold_tree` nhận một cấu trúc `Tree` và hai hàm (handlers):
- `on_leaf`: Làm gì khi gặp một Leaf? (Biến giá trị Leaf thành kết quả)
- `on_branch`: Làm gì khi nhận được kết quả của hai nhánh con? (Gộp chúng lại)

Hàm `fold_tree` sẽ lo phần đệ quy (traversal). Bạn chỉ lo phần logic (computation).

```python
# filename: tree_fold.py
from dataclasses import dataclass
from typing import Union, Callable, TypeVar

T = TypeVar("T")

@dataclass(frozen=True)
class Leaf:
    value: int
@dataclass(frozen=True)
class Branch:
    left: Union["Branch", "Leaf"]
    right: Union["Branch", "Leaf"]
Tree = Union[Branch, Leaf]

# ── Hàm Fold Tổng Quát ──
def fold_tree(
    tree: Tree,
    on_leaf: Callable[[int], T],
    on_branch: Callable[[T, T], T]
) -> T:
    """Catamorphism for Binary Tree."""
    match tree:
        case Leaf(value=v):
            # Biến đổi Leaf thành giá trị T
            return on_leaf(v)
        case Branch(left=l, right=r):
            # Gọi đệ quy để có được kết quả của 2 nhánh
            left_result = fold_tree(l, on_leaf, on_branch)
            right_result = fold_tree(r, on_leaf, on_branch)
            # Dùng on_branch để gộp 2 kết quả
            return on_branch(left_result, right_result)

# ── Ứng dụng Fold ──
my_tree = Branch(Leaf(1), Branch(Leaf(2), Leaf(3)))

# 1. Tính tổng:
# - Gặp Leaf: Lấy giá trị của nó.
# - Gặp Branch: Cộng hai nhánh lại.
total = fold_tree(
    my_tree,
    on_leaf=lambda x: x,
    on_branch=lambda l, r: l + r
)
assert total == 6

# 2. Tìm Max:
maximum = fold_tree(
    my_tree,
    on_leaf=lambda x: x,
    on_branch=lambda l, r: max(l, r)
)
assert maximum == 3

# 3. Đếm số node lá (Count leaves):
# - Gặp Leaf: Tính là 1.
# - Gặp Branch: Tổng số lá của 2 nhánh.
leaf_count = fold_tree(
    my_tree,
    on_leaf=lambda x: 1,
    on_branch=lambda l, r: l + r
)
assert leaf_count == 3

# 4. Chuyển Tree thành mảng (In-order traversal):
array_rep = fold_tree(
    my_tree,
    on_leaf=lambda x: [x],
    on_branch=lambda l, r: l + r  # Nối 2 list
)
assert array_rep == [1, 2, 3]

print("Tree Fold OK ✅")
```

Bạn thấy sức mạnh của Fold chưa? Bạn KHÔNG CÒN phải viết chữ `match` hay gọi đệ quy trong từng nghiệp vụ tính tổng, tìm max, hay đếm node nữa. `fold_tree` đã abstract away (trừu tượng hóa) cấu trúc đệ quy!

> **💡 Catamorphism**: Trong toán học Category Theory, "Cata" nghĩa là đi xuống (downwards), "morph" là hình thái. Catamorphism là quá trình "phá hủy" (collapse) một cấu trúc đệ quy từ đáy (lá) lên đỉnh (gốc) thành một giá trị duy nhất.

---

## 28c.4 — AST Evaluator bằng Fold

### Abstract Syntax Tree là Kiểu Đệ Quy thực tiễn nhất

Trình biên dịch (Compilers), máy tính bỏ túi, trình phân tích JSON... tất cả đều parse chuỗi text thành AST (Chapter 28), và sau đó evaluate cái AST đó.
AST chính là một dạng cấu trúc đệ quy (Recursive Type).

Hãy xây dựng một AST đơn giản cho biểu thức toán học và evaluate nó bằng Fold.

```python
# filename: ast_fold.py
from dataclasses import dataclass
from typing import Union, Callable, TypeVar

T = TypeVar("T")

# ── AST Definition ──
@dataclass(frozen=True)
class Literal:
    value: float

@dataclass(frozen=True)
class Add:
    left: "Expr"
    right: "Expr"

@dataclass(frozen=True)
class Mul:
    left: "Expr"
    right: "Expr"

@dataclass(frozen=True)
class Negate:
    expr: "Expr"

Expr = Union[Literal, Add, Mul, Negate]

# Biểu thức: (2 + 3) * -4
# Dạng AST:
ast = Mul(
    Add(Literal(2), Literal(3)),
    Negate(Literal(4))
)

# ── Hàm Fold cho AST ──
def fold_expr(
    expr: Expr,
    on_literal: Callable[[float], T],
    on_add: Callable[[T, T], T],
    on_mul: Callable[[T, T], T],
    on_negate: Callable[[T], T],
) -> T:
    match expr:
        case Literal(value=v):
            return on_literal(v)
        case Add(left=l, right=r):
            return on_add(
                fold_expr(l, on_literal, on_add, on_mul, on_negate),
                fold_expr(r, on_literal, on_add, on_mul, on_negate)
            )
        case Mul(left=l, right=r):
            return on_mul(
                fold_expr(l, on_literal, on_add, on_mul, on_negate),
                fold_expr(r, on_literal, on_add, on_mul, on_negate)
            )
        case Negate(e):
            return on_negate(fold_expr(e, on_literal, on_add, on_mul, on_negate))

# ── Ứng dụng: AST Evaluator ──
def evaluate(expr: Expr) -> float:
    return fold_expr(
        expr,
        on_literal=lambda v: v,
        on_add=lambda l, r: l + r,
        on_mul=lambda l, r: l * r,
        on_negate=lambda e: -e
    )

assert evaluate(ast) == -20.0

# ── Ứng dụng 2: AST Pretty Printer ──
def pretty_print(expr: Expr) -> str:
    return fold_expr(
        expr,
        on_literal=lambda v: str(v),
        on_add=lambda l, r: f"({l} + {r})",
        on_mul=lambda l, r: f"({l} * {r})",
        on_negate=lambda e: f"(-{e})"
    )

assert pretty_print(ast) == "((2.0 + 3.0) * (-4.0))"

print("AST Fold OK ✅")
```

Với `fold_expr`, một khi bạn đã viết nó một lần, bạn có thể tạo ra máy tính toán học, trình in code format, trình đếm số lượng phép toán... MÀ KHÔNG BAO GIỜ phải nhìn thấy chữ `match expr` hay gọi đệ quy một lần nào nữa!

---

## ✅ Checkpoint 28c.1-28c.4

> Đến đây bạn phải hiểu:
> 1. **Kiểu Đệ Quy**: Cấu trúc dữ liệu chứa chính nó (List, Tree, AST).
> 2. **Vấn đề Đệ Quy thường**: Trộn lẫn "duyệt cây" và "xử lý node".
> 3. **Fold (Catamorphism)**: Pattern tách việc "duyệt" vào một hàm duy nhất.
> 4. Tham số của Fold là các `handlers` xử lý việc "gộp" kết quả của con thành kết quả của cha.
> 5. **Tái sử dụng**: Dùng Fold để Evaluate, Print, Compile MỘT AST mà không cần viết lại đệ quy.
>
> **Test nhanh**: Phép `reduce(lambda a, b: a+b, [1,2,3])` trong Python bản chất là một fold. Vậy nếu ta đổi cấu trúc từ List sang Tree, sự khác biệt của logic cộng là gì?
> <details><summary>Đáp án</summary>Logic cộng `lambda a, b: a+b` GẦN NHƯ KHÔNG ĐỔI! `reduce` là fold qua 1 chiều (List có 1 đuôi). `fold_tree` là fold qua 2 chiều (Branch có left và right). Bản chất vẫn là dùng `+` để nối kết quả của các node con.</details>

---

## 🏋️ Bài tập

**Bài 1** (10 phút): Fold cho một cấu trúc `LinkedList`

```python
# Cho cấu trúc LinkedList như sau:
from dataclasses import dataclass
from typing import Union, Callable, TypeVar

T = TypeVar("T")

@dataclass(frozen=True)
class Empty:
    pass

@dataclass(frozen=True)
class Node:
    value: int
    next: Union["Node", Empty]

LinkedList = Union[Node, Empty]

# YÊU CẦU:
# 1. Viết hàm fold_list(lst: LinkedList, on_empty: Callable[[], T], on_node: Callable[[int, T], T]) -> T
# 2. Dùng fold_list để tính tổng của danh sách: Node(1, Node(2, Node(3, Empty())))
```

<details><summary>✅ Lời giải Bài 1</summary>

```python
def fold_list(
    lst: LinkedList, 
    on_empty: Callable[[], T], 
    on_node: Callable[[int, T], T]
) -> T:
    match lst:
        case Empty():
            return on_empty()
        case Node(value=v, next=nxt):
            # Tính phần đuôi trước
            tail_result = fold_list(nxt, on_empty, on_node)
            # Gộp node hiện tại với kết quả của đuôi
            return on_node(v, tail_result)

my_list = Node(1, Node(2, Node(3, Empty())))

# Tính tổng
total = fold_list(
    my_list,
    on_empty=lambda: 0,
    on_node=lambda val, tail_res: val + tail_res
)
assert total == 6

# Thú vị: Chuyển LinkedList về Python List thông thường
py_list = fold_list(
    my_list,
    on_empty=lambda: [],
    on_node=lambda val, tail_res: [val] + tail_res
)
assert py_list == [1, 2, 3]
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Lỗi "Maximum recursion depth exceeded" | Thiếu base case (Trường hợp lá/Empty) | Cấu trúc đệ quy phải luôn kết thúc bằng một Node không chứa tham chiếu đệ quy (Leaf/Empty/Literal). |
| Handlers bị sai kiểu | Nhầm lẫn giữa Node con và Kết quả đã được tính của Node con | Trong hàm fold, tham số đưa vào `on_branch` là kết quả ĐÃ ĐƯỢC TÍNH (Fold) của con, không phải là đối tượng Node con! |
| Code nhìn cồng kềnh | Viết lại fold nhiều lần | Bạn chỉ viết `fold` ĐÚNG 1 LẦN cho mỗi loại cấu trúc dữ liệu. Sau đó dùng handlers (lambda) cho mọi việc. |

---

## Tóm tắt

- ✅ **Recursive Types**: Các kiểu dữ liệu self-referential (AST, Tree, List).
- ✅ **Fold (Catamorphism)**: Kỹ thuật bóc tách hoàn toàn vòng lặp đệ quy ra khỏi logic xử lý. Nó biến cấu trúc thành một giá trị.
- ✅ **Tách biệt quan tâm**: Hàm Fold chịu trách nhiệm DUYỆT. Bạn (người gọi) cung cấp `handlers` (lamdas) chịu trách nhiệm TÍNH TOÁN ở mỗi bước gộp.
- ✅ Khi làm việc với AST hoặc Compilers, **Fold là pattern bắt buộc phải biết**. Nó làm cho codebase sạch, dễ test và tránh vô số bug lặp lại khi đệ quy.

## Tiếp theo
Chúc mừng bạn đã hoàn thành Phần 5: FP Patterns trong Python. Bằng việc nắm vững Functor, Monad, Applicative, Parsers, và Recursive Folds, bạn không chỉ học Python — bạn đã thực sự thấu hiểu cái cốt lõi toán học làm cho Functional Programming mạnh mẽ đến vậy. Cấu trúc là thứ bất biến, chỉ có hàm là thứ thay hình đổi dạng!
