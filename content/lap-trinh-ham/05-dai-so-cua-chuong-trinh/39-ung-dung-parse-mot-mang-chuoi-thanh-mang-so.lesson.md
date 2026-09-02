---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.ung-dung-parse-mot-mang-chuoi-thanh-mang-so
title: "Ứng dụng: parse MỘT MẢNG chuỗi thành mảng số, thất bại nếu MỘT phần tử sai"
summary: "parseMangSo(ds): Result<number[], string> — traverseResult(ds, chuyenSoResult). Result, không phải Option, vì lỗi phải ghi rõ CHUỖI NÀO gây lỗi (nối T4.5's bài học 'Result mang lý do')."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 39
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [alg.apply-parse-array]
requires: [alg.traverse-via-reduce]
concepts: [alg.apply-parse-array]
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
`traverse` đã viết xong, cho `Option`. Bài toán THẬT thường cần biết
CHUỖI NÀO gây lỗi — `Result`, không phải `Option`.
::::

::::explain{#traverseresult-va-parsemangso}
`traverse` cho `Result` CÙNG khuôn `traverse` cho `Option` (bài 36),
chỉ đổi tên biến thể (`"co"/"khong"` → `"ok"/"loi"`) VÀ giữ nguyên
LÝ DO lỗi thay vì bỏ nó (đúng bài học cụm 2, bài 12: "Result mang lý
do, Option chỉ biết có/không"):

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function traverseResult<T, U, E>(ds: T[], f: (x: T) => Result<U, E>): Result<U[], E> {
  const kq: U[] = [];
  for (const x of ds) {
    const r = f(x);
    if (r.kind === "loi") return loi(r.loi);
    kq.push(r.giaTri);
  }
  return ok(kq);
}

function chuyenSoResult(vb: string): Result<number, string> {
  const n = Number(vb);
  return Number.isNaN(n) ? loi(`"${vb}" không phải số`) : ok(n);
}

function parseMangSo(ds: string[]): Result<number[], string> {
  return traverseResult(ds, chuyenSoResult);
}

console.log(JSON.stringify(parseMangSo(["1", "2", "3"])));
```

```text
{"kind":"ok","giaTri":[1,2,3]}
```

`parseMangSo` KHÔNG tự viết vòng lặp — nó chỉ GHÉP `traverseResult`
(công cụ TỔNG QUÁT) với `chuyenSoResult` (hàm áp dụng CỤ THỂ cho bài
toán này). Đúng tinh thần đại số: `traverseResult` viết MỘT LẦN, dùng
lại cho BẤT KỲ phép chuyển đổi nào trả về `Result`.
::::

::::example{#loi-ghi-ro-chuoi-nao}
Khác `Option` (chỉ biết "không có", bài 34-38), `Result` GHI RÕ chuỗi
NÀO gây lỗi:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function traverseResult<T, U, E>(ds: T[], f: (x: T) => Result<U, E>): Result<U[], E> {
  const kq: U[] = [];
  for (const x of ds) {
    const r = f(x);
    if (r.kind === "loi") return loi(r.loi);
    kq.push(r.giaTri);
  }
  return ok(kq);
}
function chuyenSoResult(vb: string): Result<number, string> {
  const n = Number(vb);
  return Number.isNaN(n) ? loi(`"${vb}" không phải số`) : ok(n);
}
function parseMangSo(ds: string[]): Result<number[], string> {
  return traverseResult(ds, chuyenSoResult);
}

const ketQua = parseMangSo(["10", "hai_muoi", "30"]);
console.log(ketQua.kind === "loi" ? ketQua.loi : "không lỗi");
```

```text title=readonly
"hai_muoi" không phải số
```

Thông điệp lỗi GHI RÕ `"hai_muoi"` — CHÍNH chuỗi gây lỗi, không phải
"có gì đó sai" chung chung. `"10"` (phần tử TRƯỚC nó) đã chuyển được,
nhưng KHÔNG XUẤT HIỆN trong kết quả cuối — cả `parseMangSo` LỖI hoàn
toàn khi DÙ CHỈ MỘT phần tử sai, và lỗi trả về là lỗi của phần tử ĐẦU
TIÊN gây ra nó.
::::

::::predict{#doan-parsemangso-loi-cuoi commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function traverseResult<T, U, E>(ds: T[], f: (x: T) => Result<U, E>): Result<U[], E> {
  const kq: U[] = [];
  for (const x of ds) {
    const r = f(x);
    if (r.kind === "loi") return loi(r.loi);
    kq.push(r.giaTri);
  }
  return ok(kq);
}
function chuyenSoResult(vb: string): Result<number, string> {
  const n = Number(vb);
  return Number.isNaN(n) ? loi(`"${vb}" không phải số`) : ok(n);
}
function parseMangSo(ds: string[]): Result<number[], string> {
  return traverseResult(ds, chuyenSoResult);
}

const ketQua = parseMangSo(["5", "10", "x"]);
console.log(ketQua.kind);
console.log(ketQua.kind === "loi" ? ketQua.loi : "không lỗi");
```

Hai dòng cuối in ra gì?

:::opt{correct}
`loi` rồi `"x" không phải số`
:::

:::opt
`loi` rồi `"5,10,x" không phải số` — vì thông điệp lỗi gộp CẢ mảng
gốc vào một chuỗi để người đọc thấy TOÀN CẢNH
::why
Gần đúng ở việc bạn nhận ra ĐÚNG kết quả CUỐI là `"loi"` — quan sát đó
đúng.

Chỗ lệch: `chuyenSoResult` chỉ nhận MỘT phần tử `vb` MỖI LẦN gọi — nó
KHÔNG hề biết tới cả mảng gốc, chỉ biết CHUỖI ĐANG XÉT lúc đó
(`"x"`). Thông điệp lỗi ghi rõ ĐÚNG chuỗi gây lỗi (`"x"`), không phải
toàn bộ mảng gộp lại.
::
:::

:::opt
`ok` rồi `"không lỗi"` — vì `"5"` và `"10"` chuyển được, `traverseResult`
gom được HAI phần tử ĐÓ, chỉ bỏ qua `"x"`
::why
Gần đúng ở việc bạn nhớ ĐÚNG `"5"` và `"10"` TỰ chúng chuyển được
(không lỗi) — quan sát cục bộ đó đúng.

Chỗ lệch: `traverseResult` (bài 39's khuôn `traverse`, cụm 6) KHÔNG
"bỏ qua phần tử lỗi rồi tiếp tục gom phần còn lại" — nó dừng NGAY và
trả về LỖI của phần tử ĐẦU TIÊN gây lỗi. `"5"`, `"10"` VẪN chuyển
được, nhưng KHÔNG XUẤT HIỆN trong kết quả cuối — cả `parseMangSo` là
`"loi"`, không phải `"ok"` một phần.
::
:::
::::

::::code{#parse_mang_so}
Tự viết `chuyenSoResult` và `parseMangSo(ds: string[]): Result<number[],
string>` — dùng `traverseResult` đã có sẵn.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function traverseResult<T, U, E>(ds: T[], f: (x: T) => Result<U, E>): Result<U[], E> {
  const kq: U[] = [];
  for (const x of ds) {
    const r = f(x);
    if (r.kind === "loi") return loi(r.loi);
    kq.push(r.giaTri);
  }
  return ok(kq);
}

function chuyenSoResult(vb: string): Result<number, string> {
  const n = Number(vb);
  return Number.isNaN(n) ? ___ : ___;
}

function parseMangSo(ds: string[]): Result<number[], string> {
  return ___;
}

console.log(JSON.stringify(parseMangSo(["1", "2", "3"])));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function traverseResult<T, U, E>(ds: T[], f: (x: T) => Result<U, E>): Result<U[], E> {
  const kq: U[] = [];
  for (const x of ds) {
    const r = f(x);
    if (r.kind === "loi") return loi(r.loi);
    kq.push(r.giaTri);
  }
  return ok(kq);
}

function chuyenSoResult(vb: string): Result<number, string> {
  const n = Number(vb);
  return Number.isNaN(n) ? loi(`"${vb}" không phải số`) : ok(n);
}

function parseMangSo(ds: string[]): Result<number[], string> {
  return traverseResult(ds, chuyenSoResult);
}

console.log(JSON.stringify(parseMangSo(["1", "2", "3"])));
```

```typescript title=test
const a = parseMangSo(["1", "2", "3"]);
if (a.kind !== "ok") throw new Error("mọi phần tử hợp lệ phải ra ok");
if (a.kind === "ok" && JSON.stringify(a.giaTri) !== JSON.stringify([1, 2, 3])) throw new Error("kq phải là [1, 2, 3]");

const b = parseMangSo(["10", "hai_muoi", "30"]);
if (b.kind !== "loi") throw new Error("một phần tử sai phải khiến cả kết quả là loi");
if (b.kind === "loi" && b.loi !== '"hai_muoi" không phải số') throw new Error("thông điệp lỗi phải ghi rõ chuỗi gây lỗi");

const c = parseMangSo([]);
if (c.kind !== "ok") throw new Error("mảng rỗng phải ra ok([])");
if (c.kind === "ok" && c.giaTri.length !== 0) throw new Error("mảng rỗng phải ra kq rỗng");
```

:::hints
- kind: attention
  body: "chuyenSoResult: nhánh lỗi trả loi với THÔNG ĐIỆP ghi rõ vb; nhánh hợp lệ trả ok(n). parseMangSo: chỉ GHÉP traverseResult với chuyenSoResult, không tự viết vòng lặp."
- kind: strategy
  body: "loi(`\"${vb}\" không phải số`) : ok(n) — cho chuyenSoResult. traverseResult(ds, chuyenSoResult) — cho parseMangSo."
- kind: one-line
  body: "___ (nhánh lỗi) = loi(`\"${vb}\" không phải số`)\n___ (nhánh hợp lệ) = ok(n)\n___ (parseMangSo) = traverseResult(ds, chuyenSoResult)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1,2,3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`traverseResult` viết MỘT LẦN, dùng lại cho bất kỳ phép chuyển đổi
nào — `parseMangSo` chỉ là MỘT cách ghép cụ thể của nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài chốt cụm — một tình huống MỚI, dùng `traverseOption` (không phải
`Result`) lần nữa.
::::

::::checkpoint{mastery=0.8}
::::
