---
id: ky-nghe-phan-mem.ddd.combine-gop-nhieu-result-doc-lap
title: "combine — gộp NHIỀU Result ĐỘC LẬP, báo lỗi ĐẦU TIÊN tìm thấy"
summary: "combine<T,E>(ds: Result<T,E>[]): Result<T[],E> — nhận MẢNG Result ĐÃ có sẵn, trả ok(mảng giá trị) nếu TẤT CẢ đều ok, hoặc loi(...) của phần tử LỖI ĐẦU TIÊN. Đối chiếu trực tiếp map2GomLoi (T4.5): combine dừng ở lỗi đầu, map2GomLoi gom HẾT — hai combinator phục vụ hai nhu cầu khác nhau."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 29
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [ddd.combine]
requires: [ddd.from-throwable]
concepts: [ddd.combine]
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
`chainResult` ghép các bước PHỤ THUỘC tuần tự. Nếu có SẴN một MẢNG
`Result` ĐỘC LẬP (không bước nào cần kết quả của bước khác) thì sao?
::::

::::explain{#combine}
`combine<T, E>(ds: Result<T, E>[]): Result<T[], E>` — nhận MỘT MẢNG
các `Result` ĐÃ CÓ SẴN (khác `chainResult`: KHÔNG có hàm nào để GỌI
tuần tự) — trả `ok(mảng giá trị)` NẾU TẤT CẢ đều `ok`, hoặc `loi(...)`
của phần tử LỖI ĐẦU TIÊN quét thấy:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function combine<T, E>(ds: Result<T, E>[]): Result<T[], E> {
  const giaTriList: T[] = [];
  for (const r of ds) {
    if (r.kind === "loi") return loi(r.loi);
    giaTriList.push(r.giaTri);
  }
  return ok(giaTriList);
}

const tatCaOk: Result<number, string>[] = [ok(1), ok(2), ok(3)];
console.log(JSON.stringify(combine(tatCaOk)));

const coLoi: Result<number, string>[] = [ok(1), loi("lỗi thứ nhất"), loi("lỗi thứ hai")];
console.log(JSON.stringify(combine(coLoi)));
```

```text
{"kind":"ok","giaTri":[1,2,3]}
{"kind":"loi","loi":"lỗi thứ nhất"}
```

`combine` DUYỆT mảng TỪ ĐẦU — gặp phần tử `loi` ĐẦU TIÊN, TRẢ VỀ NGAY
lỗi ĐÓ, KHÔNG duyệt tiếp (`"lỗi thứ hai"` KHÔNG BAO GIỜ xuất hiện trong
kết quả, dù nó CÓ tồn tại trong mảng). Nếu TOÀN BỘ đều `ok`, `combine`
trả `ok` chứa MẢNG các giá trị THẬT, ĐÚNG thứ tự gốc.
::::

::::example{#combine-vs-map2gomloi}
So sánh TRỰC TIẾP với `map2GomLoi` (đã học T4.5 bài 25) TRÊN CÙNG hai
lỗi: `combine` CHỈ báo lỗi ĐẦU TIÊN, `map2GomLoi` gom **HẾT**:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function combine<T, E>(ds: Result<T, E>[]): Result<T[], E> {
  const giaTriList: T[] = [];
  for (const r of ds) {
    if (r.kind === "loi") return loi(r.loi);
    giaTriList.push(r.giaTri);
  }
  return ok(giaTriList);
}
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}
function validateTen(ten: string): Result<string, string[]> {
  if (ten.trim() === "") return loi(["tên không được để trống"]);
  return ok(ten);
}
function validateEmail(email: string): Result<string, string[]> {
  if (!email.includes("@")) return loi(["email không hợp lệ"]);
  return ok(email);
}

// CẢ HAI trường đều SAI
const quaCombine = combine([validateTen(""), validateEmail("khong-hop-le")]);
console.log(JSON.stringify(quaCombine));

const quaMap2GomLoi = map2GomLoi(validateTen(""), validateEmail("khong-hop-le"), (t, e) => ({ ten: t, email: e }));
console.log(JSON.stringify(quaMap2GomLoi));
```

```text title=readonly
{"kind":"loi","loi":["tên không được để trống"]}
{"kind":"loi","loi":["tên không được để trống","email không hợp lệ"]}
```

CÙNG hai validate LỖI — `combine` CHỈ báo `"tên không được để trống"`
(dừng ở lỗi ĐẦU TIÊN quét thấy), `map2GomLoi` báo **CẢ HAI**. Hai
combinator phục vụ hai NHU CẦU khác nhau: `combine` khi chỉ cần biết
"CÓ lỗi hay không, một lý do là đủ" (ví dụ: kiểm TRA NHANH một danh
sách N bước độc lập trước khi xử lý gì đó nặng); `map2GomLoi` khi cần
liệt kê **HẾT** lỗi cho người dùng SỬA MỘT LẦN (form validation, bài
sau).
::::

::::predict{#doan-combine-mang-rong commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function combine<T, E>(ds: Result<T, E>[]): Result<T[], E> {
  const giaTriList: T[] = [];
  for (const r of ds) {
    if (r.kind === "loi") return loi(r.loi);
    giaTriList.push(r.giaTri);
  }
  return ok(giaTriList);
}

const mangRong: Result<number, string>[] = [];
const ketQua = combine(mangRong);
console.log(ketQua.kind);
console.log(ketQua.kind === "ok" ? ketQua.giaTri.length : -1);
```

Truyền vào MẢNG RỖNG (không phần tử nào). Hai dòng cuối in ra gì?

:::opt{correct}
`ok` rồi `0`
:::

:::opt
`loi` rồi `-1` — vì mảng rỗng KHÔNG có phần tử `ok` nào để xác nhận
"tất cả hợp lệ", nên `combine` coi đó là trường hợp KHÔNG XÁC ĐỊNH và
trả về lỗi
::why
Gần đúng ở việc bạn nghĩ tới việc "không có gì để xác nhận" LÀ một
trạng thái ĐẶC BIỆT — một trực giác hợp lý cho một số API kiểm tra
khác.

Chỗ lệch: đọc LẠI thân hàm `combine` — vòng `for (const r of ds)` với
`ds` RỖNG đơn giản KHÔNG CHẠY LẦN NÀO (không có phần tử để lặp), nên
KHÔNG có `return loi(...)` nào được thực thi bên trong vòng lặp. Luồng
chạy THẲNG tới dòng CUỐI CÙNG `return ok(giaTriList)` — với `giaTriList`
vẫn là mảng RỖNG (`[]`, chưa từng `push` gì). Kết quả LUÔN là
`ok([])`, tương tự "chân lý rỗng" (vacuous truth) trong logic: không
có phần tử nào VI PHẠM điều kiện, nên coi như TẤT CẢ đều thoả.
::
:::

:::opt
Máy báo lỗi biên dịch — `Result<number, string>[]` không cho phép một
mảng RỖNG, vì kiểu phần tử `Result<number, string>` không phải kiểu
"nullable"
::why
Gần đúng ở việc bạn nghĩ tới ràng buộc VỀ NỘI DUNG của mảng — một mối
lo hợp lý khi làm việc với kiểu union phức tạp.

Chỗ lệch: `T[]` (mảng kiểu `T` bất kỳ) trong TypeScript LUÔN cho phép
`[]` (mảng rỗng) — đây là quy tắc CHUNG của MỌI kiểu mảng, không liên
quan gì tới `Result` là union hay không. Biên dịch sạch, `mangRong:
Result<number, string>[] = []` hoàn toàn hợp lệ.
::
:::
::::

::::code{#viet_combine}
Tự viết PHẦN THÂN (implementation) của `combine`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function combine<T, E>(ds: Result<T, E>[]): Result<T[], E> {
  const giaTriList: T[] = [];
  for (const r of ds) {
    if (r.kind === "loi") return ___;
    giaTriList.push(r.giaTri);
  }
  return ___;
}

console.log(JSON.stringify(combine([ok(1), ok(2), ok(3)])));
console.log(JSON.stringify(combine([ok(1), loi("hỏng"), ok(3)])));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function combine<T, E>(ds: Result<T, E>[]): Result<T[], E> {
  const giaTriList: T[] = [];
  for (const r of ds) {
    if (r.kind === "loi") return loi(r.loi);
    giaTriList.push(r.giaTri);
  }
  return ok(giaTriList);
}

console.log(JSON.stringify(combine([ok(1), ok(2), ok(3)])));
console.log(JSON.stringify(combine([ok(1), loi("hỏng"), ok(3)])));
```

```typescript title=test
const okHet = combine<number, string>([ok(1), ok(2), ok(3)]);
if (okHet.kind !== "ok") throw new Error("mảng toàn ok phải ra ok");
if (okHet.kind === "ok" && (okHet.giaTri.length !== 3 || okHet.giaTri[0] !== 1 || okHet.giaTri[2] !== 3)) throw new Error("kết quả ok phải giữ ĐÚNG THỨ TỰ và ĐỦ mọi giá trị gốc");

const coLoiTest = combine<number, string>([ok(1), loi("lỗi A"), loi("lỗi B")]);
if (coLoiTest.kind !== "loi") throw new Error("mảng có phần tử loi phải ra loi");
if (coLoiTest.kind === "loi" && coLoiTest.loi !== "lỗi A") throw new Error("phải báo lỗi ĐẦU TIÊN quét thấy (lỗi A), không phải lỗi sau (lỗi B)");

const mangRongTest = combine<number, string>([]);
if (mangRongTest.kind !== "ok") throw new Error("mảng rỗng phải ra ok([]), không phải loi");
if (mangRongTest.kind === "ok" && mangRongTest.giaTri.length !== 0) throw new Error("mảng rỗng phải ra mảng giá trị RỖNG");
```

:::hints
- kind: attention
  body: "Nhánh loi trong vòng lặp: bọc r.loi bằng loi(...) và return NGAY (dừng duyệt). Nhánh cuối hàm (sau vòng lặp): bọc giaTriList (đã tích luỹ) bằng ok(...)."
- kind: strategy
  body: "loi(r.loi) : ok(giaTriList) — return NGAY khi gặp loi (dừng sớm), return ok ở CUỐI nếu vòng lặp chạy hết mà không gặp loi nào."
- kind: one-line
  body: "___ (trong vòng lặp, gặp loi) = loi(r.loi)\n___ (sau vòng lặp) = ok(giaTriList)"
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
combine: gộp mảng Result ĐỘC LẬP, dừng ở lỗi ĐẦU TIÊN. Muốn liệt kê
HẾT lỗi thay vì dừng sớm — dùng `map2GomLoi` (đã học). Bước tiếp theo:
áp dụng vào một form thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`combine` phù hợp khi chỉ cần biết "có lỗi hay không". Nhưng một FORM
đăng ký thật có NHIỀU trường — người dùng muốn thấy HẾT lỗi MỘT LẦN,
không phải sửa từng trường rồi submit lại nhiều lần. Áp dụng
`map2GomLoi` vào tình huống đó trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
