---
id: ky-nghe-phan-mem.ddd.fromthrowable-cau-noi-code-cu
title: "fromThrowable — bọc hàm throw có sẵn thành Result, chỉ MỘT chỗ"
summary: "fromThrowable<T>(f: () => T): Result<T, string> — bọc code CÓ SẴN mà throw (vd JSON.parse) thành Result. try/catch chỉ xuất hiện MỘT LẦN DUY NHẤT, ở biên, không lan ra khắp domain. f PHẢI là một THUNK (() => ...) — gọi f() NGAY khi truyền vào sẽ throw TRƯỚC KHI fromThrowable kịp bảo vệ."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 28
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [ddd.from-throwable]
requires: [ddd.match-result]
concepts: [ddd.from-throwable]
gradingMatrix:
  web-chrome: [run, tests, output]
  web-firefox: [run, tests, output]
  macos: [run, tests, output]
  windows: [run, tests, output]
  linux: [run, tests, output]
  android: [run, tests, output]
  ios: [run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Domain của bạn dùng `Result` xuyên suốt — nhưng `JSON.parse` (thư viện
CÓ SẴN) vẫn `throw`. Viết `try/catch` MỖI nơi gọi nó thì phí quá.
::::

::::explain{#fromthrowable}
`fromThrowable<T>(f: () => T): Result<T, string>` — bọc MỘT hàm CÓ THỂ
throw, TRẢ VỀ `Result` thay vì để exception lan ra. `try/catch` xuất
hiện **MỘT LẦN DUY NHẤT**, BÊN TRONG `fromThrowable` — không lặp lại ở
MỌI nơi gọi:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function fromThrowable<T>(f: () => T): Result<T, string> {
  try {
    return ok(f());
  } catch (e) {
    if (e instanceof Error) return loi(e.message);
    return loi("lỗi không rõ dạng");
  }
}

const ketQua1 = fromThrowable(() => JSON.parse('{"ten":"An"}'));
console.log(JSON.stringify(ketQua1));

const ketQua2 = fromThrowable(() => JSON.parse('khong-phai-json'));
console.log(ketQua2.kind);
```

```text
{"kind":"ok","giaTri":{"ten":"An"}}
loi
```

Tham số `f` là `() => T` — một **THUNK** (hàm KHÔNG tham số, "trì
hoãn" một phép tính). `fromThrowable` GỌI `f()` BÊN TRONG khối `try`
của CHÍNH NÓ — mọi caller từ nay chỉ cần đọc `r.kind`, KHÔNG BAO GIỜ
phải viết `try/catch` lần nữa để dùng `JSON.parse` (hay bất kỳ API CÓ
SẴN nào khác throw).
::::

::::example{#phai-boc-trong-thunk}
Điểm SỐNG CÒN: `f` PHẢI là một **THUNK** (`() => JSON.parse(...)`),
KHÔNG được gọi `JSON.parse(...)` NGAY rồi truyền KẾT QUẢ vào —
JavaScript đánh giá THAM SỐ TRƯỚC khi hàm được gọi:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function fromThrowable<T>(f: () => T): Result<T, string> {
  try {
    return ok(f());
  } catch (e) {
    if (e instanceof Error) return loi(e.message);
    return loi("lỗi không rõ dạng");
  }
}

console.log("trước khi gọi");
// SAI: gọi JSON.parse NGAY, không bọc trong () =>
const ketQua = fromThrowable(JSON.parse("{ hỏng"));
console.log("sau khi gọi -- có in được không?");
```

```text title=readonly
trước khi gọi
Lỗi cú pháp: Expected property name or '}' in JSON at position 2
```

`"sau khi gọi..."` KHÔNG BAO GIỜ in ra — chương trình CRASH NGAY tại
`JSON.parse("{ hỏng")`, TRƯỚC CẢ KHI `fromThrowable` được GỌI (đối số
phải được TÍNH XONG trước khi truyền vào hàm). Bọc trong `() => ...`
(bài mẫu ở trên) TRÌ HOÃN việc gọi `JSON.parse` cho tới khi `f()` được
GỌI BÊN TRONG khối `try` của `fromThrowable` — CHỈ khi đó `catch` mới
BẢO VỆ được.
::::

::::predict{#doan-fromthrowable-qua-ham commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function fromThrowable<T>(f: () => T): Result<T, string> {
  try {
    return ok(f());
  } catch (e) {
    if (e instanceof Error) return loi(e.message);
    return loi("lỗi không rõ dạng");
  }
}

// đóng bởi một hàm domain -- gọi fromThrowable ĐÚNG cách, qua thunk
function phanTichAntoan(vanBan: string): Result<unknown, string> {
  return fromThrowable(() => JSON.parse(vanBan));
}

const ketQua = phanTichAntoan("{ dữ liệu hỏng");
console.log(ketQua.kind);
```

Dòng cuối in ra gì?

:::opt{correct}
`loi`
:::

:::opt
Chương trình crash — vì `JSON.parse(vanBan)` bên trong `() => ...`
vẫn được đánh giá NGAY khi `phanTichAntoan` được gọi, giống hệt ví dụ
"SAI" ở phần ví dụ trước
::why
Gần đúng ở việc bạn nhớ ĐÚNG có một trường hợp `JSON.parse` gây crash
(ở ví dụ ĐÃ học) — quan sát về NGUY CƠ đó đúng.

Chỗ lệch: `() => JSON.parse(vanBan)` LÀ một THUNK THẬT SỰ — khai một
HÀM (mũi tên), KHÔNG gọi `JSON.parse` NGAY. Bên trong `phanTichAntoan`,
`fromThrowable(() => JSON.parse(vanBan))` chỉ TRUYỀN cái HÀM đó (chưa
chạy) — `JSON.parse` CHỈ thực sự chạy KHI `fromThrowable` gọi `f()`
Ở BÊN TRONG khối `try` của CHÍNH NÓ. Ví dụ "SAI" trước đó khác Ở CHỖ
truyền THẲNG `JSON.parse("{ hỏng")` (đã GỌI, KHÔNG bọc `() =>`) — sự
khác biệt CỐT LÕI mà bài học này dạy.
::
:::

:::opt
Máy báo lỗi biên dịch — `phanTichAntoan` khai kiểu trả về
`Result<unknown, string>`, nhưng `fromThrowable(...)` trả về
`Result<unknown, string>` chỉ khi `T` được SUY RA đúng là `unknown`,
điều TypeScript KHÔNG tự làm được từ `JSON.parse`
::why
Gần đúng ở việc bạn để ý CÓ một khai kiểu trả về TƯỜNG MINH
(`Result<unknown, string>`) trên `phanTichAntoan` — quan sát về việc
CÓ kiểu khai báo đó đúng.

Chỗ lệch: `JSON.parse` có kiểu trả về LÀ `any` (kiểu ĐẶC BIỆT, tương
thích NGẦM với MỌI kiểu khác, kể cả `unknown`) — TypeScript suy ra `T
= any` từ thunk, và `Result<any, string>` gán được cho biến khai
`Result<unknown, string>` mà KHÔNG báo lỗi (đặc quyền riêng của `any`,
KHÁC với mọi kiểu khác). Biên dịch sạch.
::
:::
::::

::::code{#viet_fromthrowable}
Tự viết PHẦN THÂN nhánh THÀNH CÔNG của `fromThrowable`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function fromThrowable<T>(f: () => T): Result<T, string> {
  try {
    return ___;
  } catch (e) {
    if (e instanceof Error) return loi(e.message);
    return loi("lỗi không rõ dạng");
  }
}

const ketQua1 = fromThrowable(() => JSON.parse('{"ten":"An"}'));
console.log(JSON.stringify(ketQua1));
const ketQua2 = fromThrowable(() => JSON.parse('khong-phai-json'));
console.log(ketQua2.kind);
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function fromThrowable<T>(f: () => T): Result<T, string> {
  try {
    return ok(f());
  } catch (e) {
    if (e instanceof Error) return loi(e.message);
    return loi("lỗi không rõ dạng");
  }
}

const ketQua1 = fromThrowable(() => JSON.parse('{"ten":"An"}'));
console.log(JSON.stringify(ketQua1));
const ketQua2 = fromThrowable(() => JSON.parse('khong-phai-json'));
console.log(ketQua2.kind);
```

```typescript title=test
const thanhCong = fromThrowable(() => 42);
if (thanhCong.kind !== "ok") throw new Error("hàm KHÔNG throw phải ra ok");
if (thanhCong.kind === "ok" && thanhCong.giaTri !== 42) throw new Error("nhánh ok phải giữ ĐÚNG giá trị f() trả về");

const thatBai = fromThrowable(() => { throw new Error("hỏng rồi"); });
if (thatBai.kind !== "loi") throw new Error("hàm CÓ throw phải ra loi, không được để exception lọt ra ngoài");
if (thatBai.kind === "loi" && thatBai.loi !== "hỏng rồi") throw new Error("thông điệp lỗi phải khớp đúng e.message");

const boJson = fromThrowable(() => JSON.parse('{"a":1}'));
if (boJson.kind !== "ok") throw new Error("JSON hợp lệ phải ra ok");
```

:::hints
- kind: attention
  body: "Nhánh try phải GỌI f() (chạy thunk) rồi bọc kết quả bằng ok(...) — fromThrowable KHÔNG tự tính gì, chỉ gọi f() trong vùng an toàn của try."
- kind: strategy
  body: "ok(f()) — gọi f() trước để lấy giá trị thật, rồi bọc bằng ok."
- kind: one-line
  body: "___ (nhánh try) = ok(f())"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "ok"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
fromThrowable: `try/catch` chỉ MỘT chỗ, ở biên — domain còn lại KHÔNG
BAO GIỜ cần thấy `throw` nữa. Bước tiếp theo: gộp NHIỀU Result cùng
lúc.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Có NHIỀU `Result` ĐỘC LẬP (không phải chuỗi tuần tự phụ thuộc nhau) —
gộp TẤT CẢ lại thành MỘT mảng giá trị (nếu tất cả đều `ok`) trông
thế nào?
::::

::::checkpoint{mastery=0.8}
::::
