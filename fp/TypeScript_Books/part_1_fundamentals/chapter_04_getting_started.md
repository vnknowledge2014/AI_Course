# Chapter 4 — The TypeScript Ecosystem & Tooling

> **Bạn sẽ học được**:
> - Hệ sinh thái công cụ của TypeScript: Node.js vs Bun — nên chọn cái nào?
> - `tsconfig.json` — "Trái tim" của mọi dự án TypeScript. Tại sao `strict: true` là sinh tử.
> - **Linh hồn của trải nghiệm lập trình**: Cài đặt IDE (VSCode) đúng chuẩn.
> - Linter & Formatter: `ESLint` và `Prettier` — Cặp bài trùng giúp code luôn sạch đẹp.
> - Chạy code TypeScript trực tiếp với `tsx` hoặc `Bun` mà không cần biên dịch thủ công.
>
> **Yêu cầu trước**: Không cần! Nếu bạn bỏ qua Part 0 vì quá nhiều lý thuyết, bạn đang ở đúng chỗ để bắt đầu thực hành.
> **Thời gian đọc**: ~30 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Bạn có một môi trường lập trình TypeScript chuẩn chỉ, chuyên nghiệp, với các công cụ bắt lỗi tự động hoạt động mượt mà.

---

## 4.1 — JavaScript, TypeScript và Runtime

TypeScript (TS) không thể chạy trực tiếp trên trình duyệt hay máy tính của bạn. Nó là một **ngôn ngữ biên dịch (compiled language)**, tức là nó phải được dịch ngược (transpile) về JavaScript (JS) trước khi chạy.

Vì vậy, để làm việc với TS, bạn cần một **Runtime** (Môi trường chạy JS). Hiện tại có 2 thế lực chính:

1. **Node.js**: Kẻ thống trị lâu năm. Cực kỳ ổn định, cộng đồng khổng lồ. Tuy nhiên, Node.js mặc định chỉ hiểu JS. Để chạy TS trên Node, bạn phải dùng thêm công cụ phụ trợ (như `tsc` hoặc `tsx`).
2. **Bun** (hoặc **Deno**): Kẻ thách thức thế hệ mới. Siêu nhanh và **hỗ trợ chạy trực tiếp file TypeScript** mà không cần cấu hình lằng nhằng.

> **💡 Lời khuyên**: Khi mới học, nếu bạn muốn mọi thứ đơn giản và nhanh nhất, hãy cài đặt **Bun**. Nếu bạn muốn học theo chuẩn công nghiệp hiện tại (hầu hết công ty đang dùng), hãy cài đặt **Node.js** (phiên bản LTS). Trong cuốn sách này, chúng ta sẽ dùng Node.js làm tiêu chuẩn, nhưng mã nguồn sẽ chạy hoàn hảo trên cả hai.

---

## 4.2 — Khởi tạo dự án và Package Manager

Package Manager (Trình quản lý gói) giúp bạn tải thư viện của người khác về dùng. Với Node.js, công cụ mặc định là `npm` (Node Package Manager).

Hãy tạo dự án đầu tiên:

```bash
# Tạo thư mục
mkdir ts_cafe
cd ts_cafe

# Khởi tạo file package.json (sổ hộ khẩu của dự án)
npm init -y

# Cài đặt TypeScript và công cụ chạy code (tsx)
npm install -D typescript @types/node tsx
```

*Lưu ý: Cờ `-D` nghĩa là cài đặt như một công cụ phát triển (Dev Dependency).*

---

## 4.3 — TSConfig: Trái Tim Của TypeScript

Nếu không có cấu hình, TypeScript sẽ hoạt động rất lỏng lẻo (chẳng khác gì JavaScript). Để bật "chế độ siêu nghiêm ngặt" (thứ bắt buộc phải có khi lập trình Functional), bạn cần tạo file `tsconfig.json`.

Chạy lệnh:
```bash
npx tsc --init
```

Mở file `tsconfig.json` vừa tạo, và đảm bảo bạn có ít nhất các dòng sau:

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "CommonJS",
    "strict": true,               // BẮT BUỘC: Bật mọi tính năng kiểm tra nghiêm ngặt nhất
    "noImplicitAny": true,        // Không cho phép kiểu 'any' ẩn
    "strictNullChecks": true,     // Bắt buộc kiểm tra null/undefined
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  }
}
```

> **🔥 Tại sao `strict: true` lại quan trọng?**
> Nếu không có dòng này, TypeScript sẽ cho phép các biến bị gán giá trị `null` hoặc `undefined` một cách lỏng lẻo. Lỗi "Cannot read properties of undefined" (Lỗi tỷ đô của ngành phần mềm) sẽ quay trở lại ám ảnh bạn. `strict: true` ép bạn phải xử lý triệt để mọi trường hợp lỗi tiềm ẩn!

---

## 4.4 — IDE & Trải Nghiệm Viết Code (VSCode)

TypeScript được tạo ra bởi Microsoft, và VSCode cũng vậy. Do đó, VSCode là trình soạn thảo tốt nhất cho TypeScript.

**Bạn không cần cài extension nào để VSCode hiểu TypeScript**, vì nó đã được tích hợp sẵn. Tuy nhiên, tính năng tuyệt vời nhất mà bạn PHẢI BẬT là **Inlay Hints (Gợi ý kiểu ngầm định)**.

Cách bật:
1. Mở Cài đặt (Settings) trong VSCode (`Ctrl + ,`).
2. Tìm kiếm từ khóa: `Inlay Hints`.
3. Bật tùy chọn: `TypeScript > Inlay Hints: Enum Member Values`, `Function Like Return Types`, `Parameter Names`, và `Variable Types`.

Khi bật tính năng này, VSCode sẽ hiện chữ xám mờ mờ báo cho bạn biết kiểu dữ liệu của biến mà bạn không cần phải gõ tay khai báo. Nó như một người thầy luôn đứng cạnh nhắc nhở bạn.

---

## 4.5 — Linter và Formatter: Code Sạch Và Đẹp

### Prettier (Formatter)
Đừng bao giờ tốn thời gian căn chỉnh phím Space hay Tab. Hãy để máy làm việc đó.
1. Cài extension **Prettier - Code formatter** trên VSCode.
2. Bật tính năng **Format On Save** trong Settings của VSCode.
Từ giờ, cứ mỗi lần bấm `Ctrl + S`, code của bạn sẽ tự động được xếp lại đẹp mắt.

### ESLint (Linter)
TypeScript chỉ kiểm tra LỖI KIỂU (Type errors). Nhưng nếu bạn viết code tồi (ví dụ: khai báo biến mà không bao giờ dùng, hoặc dùng `==` thay vì `===`), TS có thể không báo lỗi. **ESLint** sinh ra để bắt những lỗi phong cách này.

Để bắt đầu nhanh, bạn có thể thiết lập sau khi đã quen với ngôn ngữ. Còn hiện tại, sự kết hợp giữa TypeScript (strict mode) + Prettier là đã đủ tuyệt vời cho người mới.

---

## 4.6 — Workflow Chạy Code Hàng Ngày

Tạo một file `src/index.ts`:

```typescript
const greeting = "Hello Functional TypeScript!";
console.log(greeting);
```

**Làm sao để chạy?**

*Cách cũ (Khổ)*: Biên dịch ra JS rồi chạy JS.
```bash
npx tsc
node src/index.js
```

*Cách hiện đại với `tsx` (Sướng)*: Chạy thẳng file TypeScript!
```bash
npx tsx src/index.ts
```

*Cách tối ưu với chế độ Watch (Tự động chạy lại khi code thay đổi)*:
```bash
npx tsx watch src/index.ts
```
Cứ mỗi lần bạn bấm Lưu (Save), chương trình sẽ tự động chạy lại. Đây là workflow hoàn hảo để học tập!

---

## 🎉 Tóm tắt
- Cài đặt **Node.js** hoặc **Bun**.
- Khởi tạo dự án với `npm init -y` và cài `typescript`, `tsx`.
- Bắt buộc phải có `tsconfig.json` với `"strict": true`.
- Bật **Inlay Hints** trong VSCode.
- Dùng `npx tsx watch` để code và xem kết quả tức thì.

Môi trường của bạn đã hoàn hảo. Hãy chuyển sang **Chapter 5** để học những dòng code đầu tiên nhé!

---

## ✅ Checkpoint 4

1. `tsc` và `tsx`/`esbuild` khác nhau vai trò thế nào trong một dự án?
2. `strict: true` bật những flag nào, và cái nào bắt nhiều lỗi nhất?
3. `verbatimModuleSyntax` giải quyết vấn đề gì?

<details>
<summary>Đáp án</summary>

1. `tsc` là **type checker** (và có thể sinh mã). `tsx`/`esbuild`/`swc` chỉ **strip type** rất nhanh mà **không** kiểm kiểu. Thực tế: dùng esbuild để chạy/bundle, và `tsc --noEmit` trong CI để kiểm.
2. Nó bật `noImplicitAny`, `strictNullChecks`, `strictFunctionTypes`, `strictBindCallApply`, `strictPropertyInitialization`, `noImplicitThis`, `useUnknownInCatchVariables`, `alwaysStrict`. **`strictNullChecks`** bắt nhiều lỗi thật nhất — nó là ranh giới giữa "có null safety" và "không".
3. Nó buộc `import type` phải viết tường minh, nên bundler biết chắc import nào bị xoá lúc biên dịch. Không có nó, một import chỉ dùng cho type vẫn có thể kéo cả module vào bundle runtime.
</details>

---

## 🏋️ Bài tập

**Bài 1 (5 phút).** Tạo project mới, bật `strict: true`, viết một hàm nhận `string | null` và quan sát TypeScript ép bạn xử lý nhánh `null`.

**Bài 2 (10 phút).** Bật thêm `noUncheckedIndexedAccess` và xem `arr[0]` đổi kiểu thành `T | undefined`. Vì sao đó là mô tả **đúng** thực tế?

**Bài 3 (10 phút).** Cấu hình `paths` trong `tsconfig` để `@domain/*` trỏ tới `src/domain/*`. Nhớ cấu hình tương ứng cho cả bundler.

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| `Cannot find module '@/...'` lúc chạy | `paths` chỉ dành cho tsc, bundler không biết | Cấu hình alias tương ứng ở vite/tsup/jest |
| `tsc` chậm trên dự án lớn | Kiểm lại toàn bộ mỗi lần | Bật `incremental: true`; dùng project references |
| Type sai mà build vẫn qua | Bundler không kiểm kiểu | Thêm `tsc --noEmit` vào CI |
| `esModuleInterop` gây lỗi import | Trộn CommonJS và ESM | Bật `esModuleInterop` + `allowSyntheticDefaultImports` |
| `any` len lỏi vào khắp nơi | Thư viện không có type | `noImplicitAny` + cài `@types/*`, hoặc tự khai báo |
