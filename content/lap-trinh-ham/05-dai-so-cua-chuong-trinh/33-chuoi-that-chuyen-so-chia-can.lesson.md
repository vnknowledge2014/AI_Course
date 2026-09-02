---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.chuoi-that-chuyen-so-chia-can
title: "Một chuỗi THẬT: chuyển chuỗi → chia → căn bậc hai, mỗi bước có thể lỗi"
summary: "tinhToanChuoi(vb): Result<number, string> — chainResult(chainResult(chuyenSo(vb), x => chiaAnToan(100, x)), y => canBac2(y)). Ba bước, mỗi bước có thể lỗi, dừng ĐÚNG chỗ."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 33
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [alg.real-chain-example]
requires: [alg.map-vs-chain]
concepts: [alg.real-chain-example]
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
Đủ lý thuyết rồi. Ghép MỘT chuỗi THẬT — ba bước, mỗi bước tự lo phần
lỗi của mình.
::::

::::explain{#bai-toan-ba-buoc}
`tinhToanChuoi(vb: string): Result<number, string>` — nhận một chuỗi,
làm BA việc theo thứ tự:

1. Chuyển `vb` thành số (`chuyenSo`) — lỗi nếu `vb` không phải số.
2. Chia `100` cho số đó (`chiaAnToan`) — lỗi nếu số đó là `0`.
3. Lấy căn bậc hai kết quả (`canBac2`) — lỗi nếu kết quả âm.

Mỗi bước SAU cần giá trị THẬT của bước TRƯỚC (chia cần biết SỐ NÀO để
chia; căn bậc hai cần biết KẾT QUẢ chia) — đúng hình dạng `chain`
(bài 31), không phải `map`. Ghép bằng `chainResult`, lồng hai lần:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

function chuyenSo(vb: string): Result<number, string> {
  const n = Number(vb);
  return Number.isNaN(n) ? loi(`"${vb}" không phải số`) : ok(n);
}
function chiaAnToan(a: number, b: number): Result<number, string> {
  return b === 0 ? loi("chia cho 0") : ok(a / b);
}
function canBac2(x: number): Result<number, string> {
  return x < 0 ? loi("không lấy được căn bậc hai của số âm") : ok(Math.sqrt(x));
}

function tinhToanChuoi(vb: string): Result<number, string> {
  return chainResult(
    chainResult(chuyenSo(vb), (x) => chiaAnToan(100, x)),
    (y) => canBac2(y),
  );
}

console.log(JSON.stringify(tinhToanChuoi("4")));
```

```text
{"kind":"ok","giaTri":5}
```

`chuyenSo("4")` ra `4` → `chiaAnToan(100, 4)` ra `25` → `canBac2(25)`
ra `5`. Ba bước, không bước nào lỗi, kết quả cuối là `ok(5)`. Đúng nối
thẳng T4.2's pipeline error-handling (bài 28) — chỉ khác: ở đó ghép
bằng `pipe`, ở đây ghép bằng `chain` (vì mỗi bước ở đây CẦN giá trị
THẬT của bước trước để CHẠY, không chỉ "áp một hàm lên kết quả").
::::

::::example{#loi-dung-tung-buoc}
Lỗi ở TỪNG bước khác nhau — dừng ĐÚNG chỗ, thông điệp lỗi ĐÚNG bước
gây ra nó:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}
function chuyenSo(vb: string): Result<number, string> {
  const n = Number(vb);
  return Number.isNaN(n) ? loi(`"${vb}" không phải số`) : ok(n);
}
function chiaAnToan(a: number, b: number): Result<number, string> {
  return b === 0 ? loi("chia cho 0") : ok(a / b);
}
function canBac2(x: number): Result<number, string> {
  return x < 0 ? loi("không lấy được căn bậc hai của số âm") : ok(Math.sqrt(x));
}
function tinhToanChuoi(vb: string): Result<number, string> {
  return chainResult(
    chainResult(chuyenSo(vb), (x) => chiaAnToan(100, x)),
    (y) => canBac2(y),
  );
}

console.log(JSON.stringify(tinhToanChuoi("abc")));
console.log(JSON.stringify(tinhToanChuoi("0")));
console.log(JSON.stringify(tinhToanChuoi("-25")));
```

```text title=readonly
{"kind":"loi","loi":"\"abc\" không phải số"}
{"kind":"loi","loi":"chia cho 0"}
{"kind":"loi","loi":"không lấy được căn bậc hai của số âm"}
```

`"abc"` lỗi ngay BƯỚC 1 (`chuyenSo`) — bước 2, 3 không chạy. `"0"` qua
được bước 1 (`0` là số hợp lệ) nhưng lỗi BƯỚC 2 (`100 / 0`) — bước 3
không chạy. `"-25"` qua bước 1 VÀ bước 2 (`100 / -25 = -4`, không
lỗi) nhưng lỗi BƯỚC 3 (`canBac2(-4)`, số âm). Ba đường lỗi khác nhau,
dừng ĐÚNG NGAY tại bước gây ra, thông điệp ĐÚNG bước đó.
::::

::::predict{#doan-tinhtoanchuoi commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}
function chuyenSo(vb: string): Result<number, string> {
  const n = Number(vb);
  return Number.isNaN(n) ? loi(`"${vb}" không phải số`) : ok(n);
}
function chiaAnToan(a: number, b: number): Result<number, string> {
  return b === 0 ? loi("chia cho 0") : ok(a / b);
}
function canBac2(x: number): Result<number, string> {
  return x < 0 ? loi("không lấy được căn bậc hai của số âm") : ok(Math.sqrt(x));
}

let soLanGoiCanBac2 = 0;
const canBac2CoDem = (x: number): Result<number, string> => {
  soLanGoiCanBac2 = soLanGoiCanBac2 + 1;
  return canBac2(x);
};

function tinhToanChuoi(vb: string): Result<number, string> {
  return chainResult(
    chainResult(chuyenSo(vb), (x) => chiaAnToan(100, x)),
    (y) => canBac2CoDem(y),
  );
}

tinhToanChuoi("0");
console.log(soLanGoiCanBac2);
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
`1` — vì hàm CUỐI CÙNG trong chuỗi (`canBac2CoDem`) luôn được gọi để
ra kết quả cuối, bất kể bước trước lỗi hay không
::why
Gần đúng ở việc bạn nghĩ tới CƠ CHẾ "chuỗi luôn chạy hết tới hàm cuối"
— một mô hình HỢP LÝ cho pipeline KHÔNG có fail-fast.

Chỗ lệch: `chainResult` LÀ fail-fast (bài 31 đã đo) — bước SAU chỉ
chạy khi bước TRƯỚC là `"ok"`. `chuyenSo("0")` ra `ok(0)` (`0` là số
hợp lệ), nhưng `chiaAnToan(100, 0)` (bước hai) lỗi NGAY (`"chia cho
0"`). `chainResult` ngoài cùng thấy đối số của nó (`chiaAnToan(100,
0)`) LÀ `"loi"`, trả về NGUYÊN VẸN lỗi đó, KHÔNG hề gọi hàm bọc
`canBac2CoDem`. `soLanGoiCanBac2` giữ nguyên `0`.
::
:::

:::opt
Máy báo lỗi biên dịch — `chainResult` lồng hai lần không hợp lệ về
kiểu khi hai lớp `chainResult` có kiểu lỗi `E` khác nhau
::why
Gần đúng ở việc bạn để ý `chainResult` LỒNG có RÀNG BUỘC về kiểu (`E`
phải khớp giữa các lớp) — quan sát về CÓ ràng buộc kiểu đó đúng.

Chỗ lệch: ở đây CẢ BA hàm (`chuyenSo`, `chiaAnToan`, `canBac2CoDem`)
CÙNG dùng `E = string` — kiểu lỗi KHỚP nhau xuyên suốt, không có xung
đột nào. Biên dịch và chạy hoàn toàn bình thường.
::
:::
::::

::::code{#tinh_toan_chuoi}
Tự viết `tinhToanChuoi(vb: string): Result<number, string>` — ghép
`chuyenSo` → `chiaAnToan(100, _)` → `canBac2` qua `chainResult`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

function chuyenSo(vb: string): Result<number, string> {
  const n = Number(vb);
  return Number.isNaN(n) ? loi(`"${vb}" không phải số`) : ok(n);
}
function chiaAnToan(a: number, b: number): Result<number, string> {
  return b === 0 ? loi("chia cho 0") : ok(a / b);
}
function canBac2(x: number): Result<number, string> {
  return x < 0 ? loi("không lấy được căn bậc hai của số âm") : ok(Math.sqrt(x));
}

function tinhToanChuoi(vb: string): Result<number, string> {
  return chainResult(
    chainResult(___, (x) => chiaAnToan(100, x)),
    (y) => ___,
  );
}

console.log(JSON.stringify(tinhToanChuoi("4")));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

function chuyenSo(vb: string): Result<number, string> {
  const n = Number(vb);
  return Number.isNaN(n) ? loi(`"${vb}" không phải số`) : ok(n);
}
function chiaAnToan(a: number, b: number): Result<number, string> {
  return b === 0 ? loi("chia cho 0") : ok(a / b);
}
function canBac2(x: number): Result<number, string> {
  return x < 0 ? loi("không lấy được căn bậc hai của số âm") : ok(Math.sqrt(x));
}

function tinhToanChuoi(vb: string): Result<number, string> {
  return chainResult(
    chainResult(chuyenSo(vb), (x) => chiaAnToan(100, x)),
    (y) => canBac2(y),
  );
}

console.log(JSON.stringify(tinhToanChuoi("4")));
```

```typescript title=test
const a = tinhToanChuoi("4");
if (a.kind !== "ok") throw new Error("input hợp lệ phải chạy hết ba bước, ra ok");
if (a.kind === "ok" && a.giaTri !== 5) throw new Error("100/4 rồi căn bậc hai phải ra 5");

const b = tinhToanChuoi("abc");
if (b.kind !== "loi") throw new Error("bước 1 (chuyển số) lỗi phải dừng ngay");
if (b.kind === "loi" && b.loi !== '"abc" không phải số') throw new Error("thông điệp lỗi phải đúng bước 1");

const c = tinhToanChuoi("0");
if (c.kind !== "loi") throw new Error("bước 2 (chia) lỗi phải dừng ngay");
if (c.kind === "loi" && c.loi !== "chia cho 0") throw new Error("thông điệp lỗi phải đúng bước 2");

const d = tinhToanChuoi("-25");
if (d.kind !== "loi") throw new Error("bước 3 (căn bậc hai) lỗi phải dừng ngay");
if (d.kind === "loi" && d.loi !== "không lấy được căn bậc hai của số âm") throw new Error("thông điệp lỗi phải đúng bước 3");
```

:::hints
- kind: attention
  body: "Chỗ trống ĐẦU: giá trị Result BAN ĐẦU để chain từ đó — chính là chuyenSo(vb). Chỗ trống SAU: bước thứ ba, áp lên giá trị y đã có từ bước chia — chính là canBac2(y)."
- kind: strategy
  body: "chainResult(chainResult(chuyenSo(vb), x => chiaAnToan(100, x)), y => canBac2(y)) — hai lớp chainResult lồng nhau, lớp NGOÀI ăn kết quả của lớp TRONG."
- kind: one-line
  body: "___ (chỗ 1) = chuyenSo(vb)\n___ (chỗ 2) = canBac2(y)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "5"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba bước, mỗi bước tự lo lỗi của mình, ghép qua `chainResult` — đúng
một chuỗi THẬT, không phải ví dụ đồ chơi hai bước nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chuỗi này dùng `Result`. Một chuỗi khác — dùng `Option` thay vì
`Result` — có gì khác không?
::::

::::checkpoint{mastery=0.8}
::::
