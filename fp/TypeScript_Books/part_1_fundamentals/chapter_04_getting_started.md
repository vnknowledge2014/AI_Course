# Chapter 4 — Getting Started

> **Bạn sẽ học được**:
> - Cài đặt Node.js, npm, và TypeScript
> - Cấu hình `tsconfig.json` — đặc biệt `strict: true`
> - Tổ chức project TypeScript chuẩn
> - Viết, compile, và chạy chương trình TypeScript đầu tiên
> - Dùng `tsx` để phát triển nhanh, `tsc` để kiểm tra types
>
> **Yêu cầu trước**: Chapter 0 (TypeScript in 10 Minutes) — bạn đã cài Node.js và biết cú pháp cơ bản.
> **Thời gian đọc**: ~30 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Một project TypeScript hoạt động, với `strict: true`, cấu trúc thư mục chuẩn, và scripts để build + run.

---

## Chào mừng đến Part I

Ở Part 0, bạn đã học **tư duy** — Lambda Calculus, types, Big-O, cấu trúc dữ liệu. Bây giờ bắt đầu **thực hành**. Part I dạy bạn viết TypeScript thực sự: từ setup project đến modules, từ functions đến type system nâng cao.

Chapter này là nền móng. Giống xây nhà — trước khi xây tường, phải đổ móng chắc. Một project cấu hình đúng từ đầu giúp bạn tránh hàng tá headaches sau này.

---

## 4.1 — Cài đặt môi trường

### Node.js và npm

TypeScript chạy trên **Node.js** — runtime JavaScript ngoài trình duyệt. `npm` (Node Package Manager) đi kèm Node.js, dùng để cài thư viện và quản lý dependencies.

```bash
# Kiểm tra đã cài chưa
node --version     # v20.x.x trở lên
npm --version      # 10.x.x trở lên
```

Nếu chưa cài, tải từ [nodejs.org](https://nodejs.org/) — chọn **LTS** (Long Term Support).

> **💡 Tại sao Node.js LTS?** LTS = ổn định, được support 30 tháng. Current = features mới, nhưng có thể có breaking changes. Cho production code, luôn chọn LTS.

### TypeScript compiler (`tsc`)

```bash
# Cài TypeScript global (để có lệnh tsc)
npm install -g typescript

# Kiểm tra
tsc --version      # Version 5.x.x trở lên
```

**`tsc` làm gì?** Nó biến file `.ts` thành `.js` — vì Node.js chỉ chạy JavaScript. Trong quá trình biên dịch, `tsc` **kiểm tra types** — đây là "thầy giáo" từ Chapter 1.

### `tsx` — Chạy trực tiếp, không cần compile

Trong khi phát triển, compile mỗi lần mất thời gian. `tsx` cho phép chạy TypeScript **trực tiếp**:

```bash
npm install -g tsx

# Chạy file .ts không cần compile
tsx src/main.ts
```

> **💡 `tsc` vs `tsx`**: `tsc` = compile + kiểm tra types (dùng cho CI/CD, production). `tsx` = chạy ngay, bỏ qua type checking (dùng cho development). Cả hai đều cần, cho 2 mục đích khác nhau.

---

## ✅ Checkpoint 4.1

> Đến đây bạn đã có:
> 1. **Node.js** + **npm** đã cài
> 2. **`tsc`** (TypeScript compiler) — kiểm tra types + compile
> 3. **`tsx`** — chạy trực tiếp .ts files cho development
>
> **Test nhanh**: `tsc` và `tsx` khác nhau thế nào?
> <details><summary>Đáp án</summary>`tsc` compile .ts → .js VÀ kiểm tra types. `tsx` chạy .ts trực tiếp, KHÔNG kiểm tra types (nhanh hơn cho dev). Dùng cả hai: `tsx` để dev, `tsc` để build/CI.</details>

---

## 4.2 — Khởi tạo Project

### `npm init` — Tạo `package.json`

Mọi project TypeScript bắt đầu với `package.json` — file mô tả project:

```bash
# Tạo thư mục project
mkdir my-fp-project
cd my-fp-project

# Khởi tạo package.json
npm init -y
```

```json
// package.json (tự động tạo)
{
    "name": "my-fp-project",
    "version": "1.0.0",
    "description": "",
    "main": "index.js",
    "scripts": {
        "test": "echo \"Error: no test specified\" && exit 1"
    }
}
```

### Cài TypeScript local

Ngoài global `tsc`, cài TypeScript **local** trong project — để đảm bảo mọi người dùng cùng version:

```bash
npm install --save-dev typescript
```

`--save-dev` = "chỉ dùng khi develop, không cần cho production". TypeScript là **dev dependency** — code JavaScript cuối cùng không cần TypeScript compiler.

Cài luôn **Node.js types** — để TypeScript hiểu `node:assert/strict` và các module built-in:

```bash
npm install --save-dev @types/node
```

### `npx tsc --init` — Tạo `tsconfig.json`

```bash
npx tsc --init
```

Lệnh này tạo `tsconfig.json` — file **quan trọng nhất** của project TypeScript. Nó nói cho `tsc` biết: compile bằng quy tắc nào, strict ra sao, output ở đâu.

---

## ✅ Checkpoint 4.2

> Đến đây bạn đã có:
> 1. **`package.json`** — khai sinh project
> 2. **TypeScript local** — cài qua `npm install --save-dev typescript`
> 3. **`tsconfig.json`** — cấu hình compiler (chưa chỉnh — Section 4.3)
>
> **Test nhanh**: Tại sao cần cài TypeScript cả global VÀ local?
> <details><summary>Đáp án</summary>Global = có lệnh `tsc` ở mọi nơi (tiện). Local = mọi dev trong team dùng đúng version (an toàn). `npx tsc` tự dùng local version.</details>

---

## 4.3 — `tsconfig.json`: Bản thiết kế của project

`tsconfig.json` do `tsc --init` tạo ra có **rất nhiều** options (100+). Bạn không cần hiểu hết — chỉ cần biết 10 options quan trọng nhất.

### Config mẫu cho sách này

```jsonc
// tsconfig.json (đây dùng jsonc — JSON với comments. File thật không cần comments)
{
    "compilerOptions": {
        // === Output ===
        "target": "ES2022",              // compile sang JavaScript version nào
        "module": "NodeNext",            // hệ thống module
        "moduleResolution": "NodeNext",  // cách tìm file import
        "outDir": "./dist",              // thư mục chứa JS output
        "rootDir": "./src",              // thư mục chứa TS source

        // === Strict Mode — QUAN TRỌNG NHẤT ===
        "strict": true,                  // bật TẤT CẢ strict checks

        // === Safety ===
        "noUncheckedIndexedAccess": true, // arr[0] trả T | undefined
        "noUnusedLocals": true,          // lỗi nếu biến khai báo mà không dùng
        "noUnusedParameters": true,      // lỗi nếu parameter không dùng
        "noFallthroughCasesInSwitch": true, // lỗi nếu switch case quên break

        // === Niceties ===
        "esModuleInterop": true,         // import thư viện CommonJS dễ hơn
        "skipLibCheck": true,            // bỏ qua type check thư viện (nhanh hơn)
        "forceConsistentCasingInFileNames": true,
        "declaration": true              // tạo .d.ts cho consumers
    },
    "include": ["src/**/*"],
    "exclude": ["node_modules", "dist"]
}
```

### `strict: true` — Dòng quan trọng nhất

Nhớ Curry-Howard từ Chapter 1? **Type = lời hứa, compiler = thầy giáo.** `strict: true` làm thầy giáo **nghiêm khắc nhất**. Nó bật 7 flags cùng lúc:

| Flag | Tác dụng |
|------|---------|
| `strictNullChecks` | `string` ≠ `string \| null`. Phải xử lý null rõ ràng |
| `strictFunctionTypes` | Kiểm tra function parameter types chặt chẽ |
| `strictBindCallApply` | `.bind()`, `.call()`, `.apply()` kiểm tra types |
| `strictPropertyInitialization` | Class properties phải khởi tạo |
| `noImplicitAny` | **Cấm** `any` ngầm — phải khai báo type rõ ràng |
| `noImplicitThis` | `this` phải có type rõ ràng |
| `alwaysStrict` | Thêm `"use strict"` vào mọi file JS output |

```typescript
// filename: src/strict_demo.ts

// ❌ Không strict: compiler bỏ qua
// function greet(name) { return "Hi " + name; }
// → name là "any" — mất type safety hoàn toàn

// ✅ Strict: compiler đòi khai báo type
const greet = (name: string): string => `Hi ${name}`;

// ❌ strictNullChecks: phải xử lý null
// const len = (s: string | null): number => s.length;
// → Error: 's' is possibly 'null'

// ✅ Xử lý null rõ ràng
const len = (s: string | null): number => s?.length ?? 0;
```

> **💡 Không strict = mất hết benefits của TypeScript.** Không strict giống mang ô đi mưa nhưng không mở ô. "Tôi dùng TypeScript" mà không strict = tự lừa mình.

### `noUncheckedIndexedAccess` — Safety hay bị bỏ quên

Đây KHÔNG nằm trong `strict: true` nhưng cực kỳ quan trọng:

```typescript
// filename: src/unchecked_index.ts

const fruits = ["apple", "banana", "cherry"];

// ❌ Không bật noUncheckedIndexedAccess:
// const first: string = fruits[0];    // compiler nghĩ chắc chắn là string
// const oops: string = fruits[999];   // cũng "string" — nhưng runtime = undefined!

// ✅ Bật noUncheckedIndexedAccess:
// const first: string | undefined = fruits[0];  // compiler CẢNH BÁO
// → Phải kiểm tra trước khi dùng:
const first = fruits[0];
if (first !== undefined) {
    console.log(first.toUpperCase());  // an toàn!
}
```

> **💡 Quy tắc**: Luôn bật `noUncheckedIndexedAccess`. Nó biến bugs runtime thành lỗi compile-time — đúng tinh thần "make illegal states unrepresentable" từ Chapter 1.

---

## ✅ Checkpoint 4.3

> Đến đây bạn phải hiểu:
> 1. **`tsconfig.json`** = não bộ của project TypeScript
> 2. **`strict: true`** = bật 7 strict flags. **KHÔNG THƯƠNG LƯỢNG**
> 3. **`noUncheckedIndexedAccess`** = `arr[i]` trả `T | undefined` — an toàn
> 4. `target`, `module`, `outDir`, `rootDir` = cấu hình compile cơ bản
>
> **Test nhanh**: `strict: true` có bao gồm `noUncheckedIndexedAccess` không?
> <details><summary>Đáp án</summary>KHÔNG! `noUncheckedIndexedAccess` phải bật riêng. Đây là "gotcha" phổ biến — nhiều dev bật strict nhưng quên flag này.</details>

---

## 4.4 — Cấu trúc thư mục

### Layout chuẩn

```
my-fp-project/
├── package.json
├── tsconfig.json
├── src/                  ← code TypeScript
│   ├── main.ts           ← entry point
│   ├── domain/           ← business logic
│   │   └── order.ts
│   └── utils/
│       └── helpers.ts
├── dist/                 ← JavaScript output (tsc)
│   ├── main.js
│   └── ...
├── tests/                ← test files
│   └── order.test.ts
└── node_modules/         ← dependencies (npm install)
```

### Tại sao `src/` và `dist/`?

- **`src/`** = source code TypeScript. Bạn chỉ edit files ở đây.
- **`dist/`** = output JavaScript. `tsc` tạo ra — **không edit tay**, thêm vào `.gitignore`.

```bash
# .gitignore
node_modules/
dist/
```

### Scripts trong `package.json`

```json
// package.json
{
    "scripts": {
        "build": "tsc",
        "start": "node dist/main.js",
        "dev": "tsx src/main.ts",
        "typecheck": "tsc --noEmit"
    }
}
```

| Script | Lệnh | Khi nào dùng |
|--------|-------|-------------|
| `npm run build` | `tsc` | Compile TS → JS |
| `npm start` | `node dist/main.js` | Chạy code đã compile |
| `npm run dev` | `tsx src/main.ts` | Dev nhanh (không compile) |
| `npm run typecheck` | `tsc --noEmit` | Kiểm tra types mà không output JS |

> **💡 `--noEmit`**: `tsc --noEmit` = "kiểm tra types nhưng đừng tạo file `.js` nào". Nhanh, dùng cho CI/pre-commit hook.

---

## ✅ Checkpoint 4.4

> Đến đây bạn phải hiểu:
> 1. **`src/`** = TypeScript source. **`dist/`** = JavaScript output (git-ignored)
> 2. **4 scripts**: `build` (tsc), `start` (node), `dev` (tsx), `typecheck` (tsc --noEmit)
> 3. **`--noEmit`** = kiểm tra types mà không tạo file — dùng cho CI
>
> **Test nhanh**: `npm run typecheck` khác `npm run build` thế nào?
> <details><summary>Đáp án</summary>Cả hai đều kiểm tra types. Nhưng `build` tạo files `.js` trong `dist/`, còn `typecheck` không tạo gì (nhanh hơn, dùng cho CI).</details>

---

## 4.5 — Chương trình đầu tiên

### Hello, Functional World!

Đến lúc ghép mọi thứ lại. Tạo project thật:

```bash
mkdir hello-fp && cd hello-fp
npm init -y
npm install --save-dev typescript
npx tsc --init
```

Chỉnh `tsconfig.json` theo mẫu Section 4.3, rồi tạo `src/main.ts`:

```typescript
// filename: src/main.ts
import assert from "node:assert/strict";

// === Domain Types ===
type Drink =
    | { readonly tag: "coffee"; readonly shots: 1 | 2 | 3 }
    | { readonly tag: "tea"; readonly flavor: string }
    | { readonly tag: "smoothie"; readonly fruit: string };

type Size = "S" | "M" | "L";

type Order = {
    readonly drink: Drink;
    readonly size: Size;
    readonly customer: string;
};

// === Pure Functions ===
const basePrice = (drink: Drink): number => {
    switch (drink.tag) {
        case "coffee":   return 30000 + drink.shots * 5000;
        case "tea":      return 25000;
        case "smoothie": return 35000;
    }
};

const sizeMultiplier = (size: Size): number => {
    switch (size) {
        case "S": return 0.8;
        case "M": return 1.0;
        case "L": return 1.2;
    }
};

const orderPrice = (order: Order): number =>
    Math.round(basePrice(order.drink) * sizeMultiplier(order.size));

const describeOrder = (order: Order): string =>
    `${order.customer}: ${order.drink.tag} size ${order.size} — ${orderPrice(order).toLocaleString()}đ`;

// === Test with assertions ===
const order1: Order = {
    drink: { tag: "coffee", shots: 2 },
    size: "M",
    customer: "An",
};

const order2: Order = {
    drink: { tag: "smoothie", fruit: "mango" },
    size: "L",
    customer: "Bình",
};

assert.strictEqual(orderPrice(order1), 40000);   // (30000 + 10000) × 1.0
assert.strictEqual(orderPrice(order2), 42000);   // 35000 × 1.2

// === Output ===
const orders: readonly Order[] = [order1, order2];

orders
    .map(describeOrder)
    .forEach(line => console.log(line));

// Output:
// An: coffee size M — 40,000đ
// Bình: smoothie size L — 42,000đ

// === Pipeline demo — FP in action ===
const totalRevenue = orders
    .map(orderPrice)
    .reduce((sum, price) => sum + price, 0);

console.log(`\nTotal revenue: ${totalRevenue.toLocaleString()}đ`);
// Output: Total revenue: 82,000đ

console.log("\n✅ All assertions passed. Project setup complete!");
```

### Chạy thử

```bash
# Cách 1: Dev nhanh
npx tsx src/main.ts

# Cách 2: Build rồi chạy
npx tsc              # compile sang dist/
node dist/main.js    # chạy JavaScript

# Cách 3: Typecheck only
npx tsc --noEmit     # chỉ kiểm tra types
```

### Điều gì vừa xảy ra?

Chương trình nhỏ này đã dùng **mọi thứ** bạn học ở Part 0:

| Concept | Xuất hiện ở đâu |
|---------|-----------------|
| **Arrow functions** (Ch0, Ch1) | `basePrice`, `sizeMultiplier`, `orderPrice` |
| **Discriminated unions** (Ch0, Ch1) | `Drink` với tag "coffee" / "tea" / "smoothie" |
| **Exhaustive switch** (Ch0, Ch1) | `basePrice` không cần `default` — compiler biết đã đủ |
| **Algebraic types** (Ch1) | `Drink` = sum type, `Order` = product type |
| **`.map()` + `.reduce()`** (Ch2) | Pipeline tính `totalRevenue` |
| **`readonly`** (Ch0, Ch3) | Mọi field đều immutable |
| **`node:assert/strict`** (Ch0) | Test inline |

> **💡 Đây là cách viết FP**: Types mô tả domain. Pure functions xử lý logic. Pipeline kết nối mọi thứ. Không mutation, không side effects (ngoại trừ `console.log`).

---

## ✅ Checkpoint 4.5

> Đến đây bạn đã:
> 1. Tạo project TypeScript từ đầu — `package.json` + `tsconfig.json`
> 2. Viết chương trình dùng **discriminated unions + pure functions + pipeline**
> 3. Chạy bằng `tsx` (dev) và `tsc` + `node` (production)
> 4. Kết nối mọi concept từ Part 0 vào code thực tế
>
> **Test nhanh**: Thêm loại đồ uống `"juice"` vào `Drink`. Compiler sẽ báo lỗi ở đâu?
> <details><summary>Đáp án</summary>Ở function `basePrice` — switch không cover case "juice", nên compiler báo: "Function lacks ending return statement and return type does not include 'undefined'". Exhaustive checking từ Chapter 1 đang hoạt động!</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Thêm feature

Thêm field `readonly iceLevel: "none" | "less" | "normal" | "extra"` vào `Order`. Sửa `describeOrder` để hiển thị ice level.

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
type Order = {
    readonly drink: Drink;
    readonly size: Size;
    readonly customer: string;
    readonly iceLevel: "none" | "less" | "normal" | "extra";
};

const describeOrder = (order: Order): string =>
    `${order.customer}: ${order.drink.tag} size ${order.size}, ice: ${order.iceLevel} — ${orderPrice(order).toLocaleString()}đ`;
```

</details>

---

**Bài 2** (10 phút): Config từ scratch

Tạo project mới. Viết `tsconfig.json` **từ tay** (không dùng `tsc --init`). Bao gồm:
- `strict: true`
- `noUncheckedIndexedAccess: true`
- `target: "ES2022"`, `module: "NodeNext"`
- `outDir: "./dist"`, `rootDir: "./src"`

Tạo `src/main.ts` in "Hello from TypeScript!" và chạy bằng `npx tsx`.

<details><summary>✅ Lời giải Bài 2</summary>

```json
{
    "compilerOptions": {
        "target": "ES2022",
        "module": "NodeNext",
        "moduleResolution": "NodeNext",
        "outDir": "./dist",
        "rootDir": "./src",
        "strict": true,
        "noUncheckedIndexedAccess": true,
        "esModuleInterop": true,
        "skipLibCheck": true,
        "forceConsistentCasingInFileNames": true
    },
    "include": ["src/**/*"],
    "exclude": ["node_modules", "dist"]
}
```

```typescript
// src/main.ts
console.log("Hello from TypeScript!");
```

```bash
npx tsx src/main.ts
# Output: Hello from TypeScript!
```

</details>

---

**Bài 3** (15 phút): Pipeline + types

Viết chương trình quản lý điểm sinh viên:

```typescript
// 1. Type Student = { name: string, scores: readonly number[] }
// 2. averageScore(student) → number (dùng .reduce())
// 3. grade(avg) → "A" | "B" | "C" | "D" | "F"
// 4. Pipeline: students.map(s => ({ ...s, avg: averageScore(s), grade: grade(averageScore(s)) }))

// Input:
const students = [
    { name: "An", scores: [85, 92, 78] },
    { name: "Bình", scores: [60, 55, 70] },
    { name: "Cường", scores: [95, 98, 100] },
];

// Expected output:
// An: avg 85.0 → B
// Bình: avg 61.7 → D
// Cường: avg 97.7 → A
```

<details><summary>💡 Gợi ý</summary>

- `averageScore`: `.reduce()` sum rồi chia `length`
- `grade`: chain of conditions — A ≥ 90, B ≥ 80, C ≥ 70, D ≥ 60, else F

</details>

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type Student = {
    readonly name: string;
    readonly scores: readonly number[];
};

type Grade = "A" | "B" | "C" | "D" | "F";

const averageScore = (student: Student): number =>
    student.scores.reduce((sum, s) => sum + s, 0) / student.scores.length;

const grade = (avg: number): Grade => {
    if (avg >= 90) return "A";
    if (avg >= 80) return "B";
    if (avg >= 70) return "C";
    if (avg >= 60) return "D";
    return "F";
};

const students: readonly Student[] = [
    { name: "An", scores: [85, 92, 78] },
    { name: "Bình", scores: [60, 55, 70] },
    { name: "Cường", scores: [95, 98, 100] },
];

students
    .map(s => {
        const avg = averageScore(s);
        return `${s.name}: avg ${avg.toFixed(1)} → ${grade(avg)}`;
    })
    .forEach(line => console.log(line));

// Output:
// An: avg 85.0 → B
// Bình: avg 61.7 → D
// Cường: avg 97.7 → A

assert.strictEqual(grade(averageScore(students[0]!)), "B");
assert.strictEqual(grade(averageScore(students[2]!)), "A");
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| `tsc: command not found` | Chưa cài global TypeScript | `npm install -g typescript` |
| `tsx: command not found` | Chưa cài tsx | `npm install -g tsx` |
| `Cannot find module 'node:assert/strict'` | TypeScript cần biết Node.js types | `npm install --save-dev @types/node` |
| `error TS18003: No inputs were found` | `tsconfig.json` chỉ vào thư mục rỗng | Kiểm tra `include` path và đảm bảo có `.ts` files |
| Types lỏng lẻo, compiler không báo lỗi | Chưa bật `strict: true` | Mở `tsconfig.json`, thêm `"strict": true` |
| `arr[0]` không trả `T \| undefined` | Chưa bật `noUncheckedIndexedAccess` | Thêm `"noUncheckedIndexedAccess": true` |
| Import file `.ts` báo lỗi module | `module` sai trong tsconfig | Dùng `"module": "NodeNext"` + `"moduleResolution": "NodeNext"` |

---

## Tóm tắt

- ✅ **Node.js + npm** = nền tảng. `npm init -y` khởi tạo project.
- ✅ **`tsc`** = compile + kiểm tra types. **`tsx`** = chạy trực tiếp (dev).
- ✅ **`tsconfig.json`** = não bộ project. **`strict: true`** = KHÔNG THƯƠNG LƯỢNG.
- ✅ **`noUncheckedIndexedAccess`** = bổ sung cần thiết — `arr[i]` trả `T | undefined`.
- ✅ **`src/` + `dist/`** = layout chuẩn. `build`, `dev`, `typecheck` scripts.
- ✅ **Discriminated unions + pure functions + pipeline** = FP trong thực tế.

## Tiếp theo

→ Chapter 5: **Types & Variables** — đào sâu vào type system: `const` vs `let`, type inference & widening, primitives, `unknown` vs `any`, và `readonly` deep.
