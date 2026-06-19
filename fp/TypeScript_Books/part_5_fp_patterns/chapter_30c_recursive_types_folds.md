# Chapter 30c — Recursive Types & Folds (Catamorphisms)

> **Bạn sẽ học được**:
> - Khái niệm **Recursive Types** (Kiểu đệ quy) trong TypeScript: LinkedList, Tree, AST.
> - Hạn chế của việc sử dụng đệ quy truyền thống (trộn lẫn logic duyệt và logic xử lý).
> - **Fold (Catamorphism)**: Pattern tối thượng để bóc tách hoàn toàn vòng lặp đệ quy ra khỏi nghiệp vụ dữ liệu.
> - Xây dựng một Tree Fold và một AST Evaluator thanh lịch, không bao giờ phải viết đệ quy lại lần 2.
>
> **Yêu cầu trước**: Hiểu cơ bản về Discriminated Unions (Ch13).
> **Thời gian đọc**: ~35 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Loại bỏ mọi lỗi "stack overflow" hoặc lỗi đệ quy nhàm chán bằng cách áp dụng pattern trừu tượng mạnh mẽ nhất cho cấu trúc lồng nhau.

---

Bạn có mảng `[1, 2, 3]`. Bạn muốn tính tổng. Bạn không viết vòng `for` chạy index thủ công, bạn dùng `array.reduce((a, b) => a + b)`. Hàm `reduce` đảm nhận việc DUYỆT (traversal), còn bạn cung cấp hàm `+` đảm nhận việc XỬ LÝ (computation).

Nhưng đối với cấu trúc dữ liệu dạng cây (Tree) hoặc cú pháp (AST), phần lớn lập trình viên lại cặm cụi tự viết các hàm đệ quy trộn lẫn cả hai việc đó lại với nhau!

**Fold (Catamorphism)** là sự tổng quát hóa của `reduce` cho BẤT KỲ cấu trúc dữ liệu nào. Nó nói rằng: "Cứ đưa cho tôi một cấu trúc đệ quy, tôi sẽ tự đi xuống tận đáy và cuộn nó dần lên thành một giá trị. Bạn chỉ việc cung cấp cho tôi hàm xử lý ở mỗi Node."

---

## 30c.1 — Recursive Types (Kiểu Đệ Quy)

Trong TypeScript, ta định nghĩa Kiểu Đệ Quy bằng cách sử dụng Type Aliases (Discriminated Unions) tự trỏ đến chính nó.

```typescript
// filename: src/recursive_types.ts

// ── 1. Linked List ──
type Empty = { readonly _tag: "Empty" };
type Node<A> = {
    readonly _tag: "Node";
    readonly value: A;
    readonly next: LinkedList<A>;
};
type LinkedList<A> = Empty | Node<A>;

// list: 1 -> 2 -> 3 -> Empty
const myList: LinkedList<number> = {
    _tag: "Node", value: 1, next: {
        _tag: "Node", value: 2, next: {
            _tag: "Node", value: 3, next: { _tag: "Empty" }
        }
    }
};

// ── 2. Binary Tree ──
type Leaf<A> = { readonly _tag: "Leaf"; readonly value: A };
type Branch<A> = {
    readonly _tag: "Branch";
    readonly left: Tree<A>;
    readonly right: Tree<A>;
};
type Tree<A> = Leaf<A> | Branch<A>;

// tree: Branch(Leaf(1), Branch(Leaf(2), Leaf(3)))
const myTree: Tree<number> = {
    _tag: "Branch",
    left: { _tag: "Leaf", value: 1 },
    right: {
        _tag: "Branch",
        left: { _tag: "Leaf", value: 2 },
        right: { _tag: "Leaf", value: 3 }
    }
};

console.log("Recursive Types OK ✅");
```

---

## 30c.2 — Vấn đề của Đệ quy truyền thống (Naive Recursion)

Giả sử ta muốn:
1. Tính tổng các phần tử trong Tree.
2. Tìm giá trị lớn nhất trong Tree.

```typescript
// filename: src/naive_recursion.ts
import assert from "node:assert/strict";

type Leaf<A> = { readonly _tag: "Leaf"; readonly value: A };
type Branch<A> = { readonly _tag: "Branch"; readonly left: Tree<A>; readonly right: Tree<A> };
type Tree<A> = Leaf<A> | Branch<A>;

const myTree: Tree<number> = {
    _tag: "Branch",
    left: { _tag: "Leaf", value: 1 },
    right: {
        _tag: "Branch",
        left: { _tag: "Leaf", value: 2 },
        right: { _tag: "Leaf", value: 3 }
    }
};

// ── Tính tổng ──
const sumTree = (tree: Tree<number>): number => {
    switch (tree._tag) {
        case "Leaf": return tree.value;
        case "Branch": return sumTree(tree.left) + sumTree(tree.right); // Đệ quy lặp lại!
    }
};

// ── Tìm max ──
const maxTree = (tree: Tree<number>): number => {
    switch (tree._tag) {
        case "Leaf": return tree.value;
        case "Branch": return Math.max(maxTree(tree.left), maxTree(tree.right)); // Lại đệ quy lặp lại!
    }
};

assert.strictEqual(sumTree(myTree), 6);
assert.strictEqual(maxTree(myTree), 3);
```

**Vấn đề**: Cấu trúc switch-case và gọi đệ quy `f(tree.left)` và `f(tree.right)` BỊ LẶP LẠI liên tục. Lỗi đánh máy một cái là stack overflow.
Giải pháp: Viết 1 hàm điều hướng (traversal) duy nhất, nhận tham số là các "handlers". Đó chính là **Fold**.

---

## 30c.3 — Tree Fold (Catamorphism)

### Bóc tách Traversal và Computation

```typescript
// filename: src/tree_fold.ts
import assert from "node:assert/strict";

type Leaf<A> = { readonly _tag: "Leaf"; readonly value: A };
type Branch<A> = { readonly _tag: "Branch"; readonly left: Tree<A>; readonly right: Tree<A> };
type Tree<A> = Leaf<A> | Branch<A>;

const myTree: Tree<number> = {
    _tag: "Branch",
    left: { _tag: "Leaf", value: 1 },
    right: {
        _tag: "Branch",
        left: { _tag: "Leaf", value: 2 },
        right: { _tag: "Leaf", value: 3 }
    }
};

// ── HÀM FOLD ──
// Rất quyền lực: biến một Tree<A> thành một giá trị kiểu R (Result).
const foldTree = <A, R>(
    onLeaf: (value: A) => R,
    onBranch: (leftRes: R, rightRes: R) => R
) => (tree: Tree<A>): R => {
    switch (tree._tag) {
        case "Leaf": 
            // Lá? Chuyển A thành R.
            return onLeaf(tree.value);
        case "Branch": 
            // Nhánh? Tính R của trái, R của phải, rồi gộp lại!
            const leftResult = foldTree(onLeaf, onBranch)(tree.left);
            const rightResult = foldTree(onLeaf, onBranch)(tree.right);
            return onBranch(leftResult, rightResult);
    }
};

// ── ỨNG DỤNG (Không bao giờ phải tự viết đệ quy nữa) ──

// 1. Tính tổng:
const sumTree = foldTree<number, number>(
    (val) => val,
    (l, r) => l + r
);
assert.strictEqual(sumTree(myTree), 6);

// 2. Tìm max:
const maxTree = foldTree<number, number>(
    (val) => val,
    (l, r) => Math.max(l, r)
);
assert.strictEqual(maxTree(myTree), 3);

// 3. Chuyển Tree thành Mảng (In-order traversal):
const toArray = foldTree<number, number[]>(
    (val) => [val],
    (l, r) => [...l, ...r]
);
assert.deepStrictEqual(toArray(myTree), [1, 2, 3]);

console.log("Tree Fold OK ✅");
```

> **💡 Catamorphism**: Xuất phát từ tiếng Hy Lạp, "Cata" là đi xuống (downwards), "morph" là biến đổi hình thái. Catamorphism có nghĩa là "phá hủy" một cấu trúc đệ quy (như Tree) từ lá lên đỉnh để thu về một giá trị duy nhất.

---

## 30c.4 — AST Evaluator bằng Fold

Abstract Syntax Tree (AST) là trái tim của mọi trình biên dịch, công cụ parse hay evaluator. Việc evaluate một AST chính là trường hợp sử dụng Fold kinh điển nhất.

```typescript
// filename: src/ast_fold.ts
import assert from "node:assert/strict";

// ── AST DEFINITION ──
type Literal = { readonly _tag: "Literal"; readonly value: number };
type Add = { readonly _tag: "Add"; readonly left: Expr; readonly right: Expr };
type Mul = { readonly _tag: "Mul"; readonly left: Expr; readonly right: Expr };
type Negate = { readonly _tag: "Negate"; readonly expr: Expr };

type Expr = Literal | Add | Mul | Negate;

// Biểu thức: (2 + 3) * -4
const ast: Expr = {
    _tag: "Mul",
    left: { _tag: "Add", left: { _tag: "Literal", value: 2 }, right: { _tag: "Literal", value: 3 } },
    right: { _tag: "Negate", expr: { _tag: "Literal", value: 4 } }
};

// ── HÀM FOLD ──
const foldExpr = <R>(
    onLiteral: (value: number) => R,
    onAdd: (l: R, r: R) => R,
    onMul: (l: R, r: R) => R,
    onNegate: (e: R) => R
) => (expr: Expr): R => {
    const f = foldExpr(onLiteral, onAdd, onMul, onNegate); // Tạo sẵn curried function cho con
    switch (expr._tag) {
        case "Literal": return onLiteral(expr.value);
        case "Add": return onAdd(f(expr.left), f(expr.right));
        case "Mul": return onMul(f(expr.left), f(expr.right));
        case "Negate": return onNegate(f(expr.expr));
    }
};

// ── ỨNG DỤNG 1: Máy tính (Evaluator) ──
const evaluate = foldExpr<number>(
    v => v,
    (l, r) => l + r,
    (l, r) => l * r,
    e => -e
);

assert.strictEqual(evaluate(ast), -20); // (2 + 3) * -4 = -20

// ── ỨNG DỤNG 2: Pretty Printer (Compiler to String) ──
const prettyPrint = foldExpr<string>(
    v => v.toString(),
    (l, r) => `(${l} + ${r})`,
    (l, r) => `(${l} * ${r})`,
    e => `(-${e})`
);

assert.strictEqual(prettyPrint(ast), "((2 + 3) * (-4))");

console.log("AST Fold OK ✅");
```

Viết 1 hàm Fold, có ngay 1 Evaluator và 1 Stringifier. Nếu cấu trúc Expr sau này thêm phép Chia (Div), bạn chỉ cần update `foldExpr` đúng một chỗ, trình biên dịch TS sẽ lập tức gào lên (Compile Error) ở tất cả các chỗ dùng `foldExpr` bắt bạn phải cung cấp `onDiv`!

---

## ✅ Checkpoint 30c.1-30c.4

> Đến đây bạn phải hiểu:
> 1. **Recursive Type**: Union Types trong TypeScript trỏ tới chính nó (như `Expr` chứa `Add` chứa lại `Expr`).
> 2. **Vấn đề Đệ Quy**: Trộn lẫn traversal (duyệt) và logic xử lý.
> 3. **Fold**: Tách Traversal thành 1 hàm duy nhất. Truyền Logic dưới dạng các Handlers (`onAdd`, `onMul`).
> 4. **Tham số của Handler**: Là KẾT QUẢ ĐÃ ĐƯỢC TÍNH (kiểu `R`) của node con, không phải bản thân node con đó!
>
> **Test nhanh**: Trong hàm `onAdd: (l: R, r: R) => R`, biến `l` là một Node của AST hay là một số (number)?
> <details><summary>Đáp án</summary>Nó là kiểu `R`. Nếu bạn đang evaluate, nó ĐÃ được tính thành số (`number`). Đây là sức mạnh của Fold: nó tính con xong mới nhét vào hàm của cha.</details>

---

## 🏋️ Bài tập

**Bài 1** (10 phút): Fold cho LinkedList

```typescript
// Cho kiểu LinkedList:
type Empty = { readonly _tag: "Empty" };
type Node<A> = { readonly _tag: "Node"; readonly value: A; readonly next: LinkedList<A> };
type LinkedList<A> = Empty | Node<A>;

// YÊU CẦU:
// 1. Viết hàm foldList(onEmpty, onNode) cho cấu trúc này.
// 2. Dùng nó tính tổng mảng sau:
const list: LinkedList<number> = { _tag: "Node", value: 1, next: { _tag: "Node", value: 2, next: { _tag: "Empty" } } };
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
const foldList = <A, R>(
    onEmpty: () => R,
    onNode: (val: A, tailResult: R) => R
) => (lst: LinkedList<A>): R => {
    switch (lst._tag) {
        case "Empty": return onEmpty();
        case "Node": return onNode(lst.value, foldList(onEmpty, onNode)(lst.next));
    }
};

const total = foldList<number, number>(
    () => 0,
    (val, tailSum) => val + tailSum
)(list);

assert.strictEqual(total, 3);
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Lỗi "Type alias circularly references itself" | Sai cấu trúc khai báo | TypeScript hỗ trợ đệ quy cho Type Alias (từ TS 3.7) nếu nó là phần của object/tuple. `type Tree = Leaf | Branch; type Branch = { left: Tree }` là hợp lệ. |
| Hàm Handler nhận được Object thay vì Giá Trị cuối | Quên gọi đệ quy trong hàm fold | Trong ruột hàm `foldExpr`, bạn phải gọi `f(expr.left)` ĐỂ LẤY KIỂU `R` trước khi đẩy vào `onAdd`. |
| Code nhìn cồng kềnh với currying | Lạm dụng Currying | Currying (`foldTree(handlers)(tree)`) rất hợp với `pipe`. Nhưng bạn có thể đổi thành `foldTree(tree, handlers)` nếu team không quen `pipe`. |

---

## Tóm tắt

- ✅ **Recursive Types**: LinkedList, BinaryTree, AST là các cấu trúc lặp lại chính nó.
- ✅ **Fold (Catamorphism)**: Là mẫu thiết kế giải phẫu cấu trúc đệ quy, biến một cấu trúc phức tạp thành một giá trị (`R`) từ dưới lên trên.
- ✅ Hàm Fold chứa toán bộ vòng lặp đệ quy. Các `handlers` chứa toàn bộ business logic. Sự phân ly (Separation of Concerns) hoàn hảo.
- ✅ Trình biên dịch TypeScript (kết hợp với Discriminated Unions) khiến cho việc viết hàm Fold cực kỳ an toàn — nó sẽ cảnh báo nếu bạn lỡ quên xử lý một nhánh kiểu nào đó.

## Tiếp theo

Đến đây, bạn đã hoàn tất hành trình trở thành Master Functional Programming trong hệ sinh thái TypeScript, từ nền tảng tự xây dựng (Ch1-24) cho tới những công cụ sắc bén nhất của nền công nghiệp (Effect-TS) và các pattern hàn lâm (Algebra, Fold). Đã đến lúc mang tất cả sức mạnh này ra sản xuất!
