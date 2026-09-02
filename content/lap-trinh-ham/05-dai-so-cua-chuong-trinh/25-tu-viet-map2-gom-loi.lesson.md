---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.tu-viet-map2-gom-loi
title: "Tự viết `map2GomLoi`"
summary: "map2GomLoi — bốn trường hợp (cả hai ok, ra lỗi, rb lỗi, CẢ HAI lỗi) đều phải đúng, đặc biệt trường hợp 'cả hai lỗi' phải gom ĐỦ cả hai vào một mảng, không mất cái nào."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 25
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [alg.write-map2-collect]
requires: [alg.collect-all-semantics]
concepts: [alg.write-map2-collect]
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
Bạn đã THẤY `map2GomLoi` hoạt động. Hôm nay tự viết nó, từ đầu — BỐN
trường hợp, không được bỏ sót trường hợp nào.
::::

::::explain{#bon-truong-hop}
`map2GomLoi(ra, rb, f)` có ĐÚNG BỐN trường hợp cần xử lý, không phải
ba như `map2` (fail-fast):

1. `ra` VÀ `rb` đều `"ok"` — gọi `f`, đóng gói kết quả vào `ok(...)`.
2. CHỈ `ra` lỗi (`rb` ổn) — trả lỗi CỦA `ra`.
3. CHỈ `rb` lỗi (`ra` ổn) — trả lỗi CỦA `rb`.
4. CẢ HAI đều lỗi — GHÉP hai mảng lỗi lại (`[...ra.loi, ...rb.loi]`),
   không bỏ sót lỗi nào.

Trường hợp thứ TƯ là trường hợp DỄ QUÊN nhất — nếu chỉ kiểm "ra lỗi
thì trả lỗi ra" TRƯỚC (giống `map2` fail-fast), lỗi của `rb` sẽ MẤT
khi CẢ HAI đều lỗi. Phải kiểm trường hợp "CẢ HAI lỗi" TRƯỚC hai trường
hợp còn lại — nếu không, code không bao giờ CHẠM tới nhánh gộp cả hai.
::::

::::example{#thu-tu-kiem-quan-trong}
Thứ tự các nhánh `if` QUYẾT ĐỊNH trường hợp "cả hai lỗi" có được xử lý
đúng hay không:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }

// SAI — kiểm "chỉ ra lỗi" TRƯỚC "cả hai lỗi"
function map2GomLoiSai<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

const ketQua = map2GomLoiSai(loiMot<number, string>("lỗi a"), loiMot<number, string>("lỗi b"), (a, b) => a + b);
console.log(ketQua);
```

```text title=readonly
{"kind":"loi","loi":["lỗi a"]}
```

`map2GomLoiSai` kiểm `ra.kind === "loi"` NGAY DÒNG ĐẦU, trả về NGAY
khi thấy `ra` lỗi — KHÔNG BAO GIỜ kiểm xem `rb` CÓ CŨNG lỗi hay không.
`"lỗi b"` bị MẤT, dù CẢ HAI đều lỗi — CHÍNH XÁC lỗi bài học này cảnh
báo: phải kiểm "CẢ HAI lỗi" TRƯỚC, không phải sau.
::::

::::predict{#doan-thu-tu-if-sai commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }

function map2GomLoiSai<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

const ok1: Result<number, string[]> = ok(5);
const loi1 = loiMot<number, string>("lỗi rb");

const ketQua = map2GomLoiSai(ok1, loi1, (a, b) => a + b);
console.log(ketQua);
```

Dòng cuối in ra gì? (Chú ý: chỉ `rb` lỗi ở đây, `ra` vẫn `ok`.)

:::opt{correct}
`{"kind":"loi","loi":["lỗi rb"]}`
:::

:::opt
`{"kind":"ok","giaTri":5}` — vì `map2GomLoiSai` chỉ kiểm `ra` lỗi hay
không, KHÔNG hề kiểm `rb`, nên bỏ qua lỗi của `rb` hoàn toàn
::why
Gần đúng ở việc bạn để ý ĐÚNG `map2GomLoiSai` có nhược điểm THẬT (kiểm
sai thứ tự khi CẢ HAI lỗi) — quan sát về NHƯỢC ĐIỂM đó đúng.

Chỗ lệch: `map2GomLoiSai` VẪN CÓ dòng `if (rb.kind === "loi") return
{ kind: "loi", loi: rb.loi };` (dòng THỨ HAI) — nó CHỈ sai khi CẢ HAI
đều lỗi (mất lỗi của `rb` trong trường hợp đó). Ở ĐÂY chỉ `rb` lỗi
(`ra` vẫn `ok`) — dòng đầu (`ra.kind === "loi"`) KHÔNG khớp, hàm CHẠY
TIẾP tới dòng thứ hai, ĐÚNG phát hiện `rb` lỗi, trả về lỗi đó.
::
:::

:::opt
Máy báo lỗi — `map2GomLoiSai` không hoạt động đúng khi CHỈ MỘT trong
hai tham số lỗi (chỉ hoạt động khi CẢ HAI cùng lỗi hoặc CẢ HAI cùng ok)
::why
Gần đúng ở việc bạn cảnh giác `map2GomLoiSai` có LỖI THIẾT KẾ — cảnh
giác đó có căn cứ, hàm này THẬT SỰ có lỗi (như ví dụ `example` đã cho
thấy).

Chỗ lệch: lỗi CỤ THỂ của `map2GomLoiSai` CHỈ xảy ra khi CẢ HAI `ra`
VÀ `rb` đều lỗi (mất lỗi của `rb`, chỉ báo lỗi của `ra`) — trường hợp
CHỈ MỘT bên lỗi (như ở đây) vẫn hoạt động ĐÚNG, không có lỗi biên
dịch hay lỗi hành vi nào.
::
:::
::::

::::code{#map2_gom_loi}
Tự viết `map2GomLoi<A, B, C, E>` — BỐN trường hợp, kiểm "CẢ HAI lỗi"
TRƯỚC.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }

function map2GomLoi<A, B, C, E>(
  ra: Result<A, E[]>,
  rb: Result<B, E[]>,
  f: (a: A, b: B) => C
): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return ___;
  if (ra.kind === "loi") return ___;
  if (rb.kind === "loi") return ___;
  return ___;
}

console.log(map2GomLoi(loiMot<number, string>("thiếu tên"), loiMot<number, string>("thiếu tuổi"), (a, b) => a + b));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }

function map2GomLoi<A, B, C, E>(
  ra: Result<A, E[]>,
  rb: Result<B, E[]>,
  f: (a: A, b: B) => C
): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

console.log(map2GomLoi(loiMot<number, string>("thiếu tên"), loiMot<number, string>("thiếu tuổi"), (a, b) => a + b));
```

```typescript title=test
const a = map2GomLoi(ok<number, string[]>(3), ok<number, string[]>(4), (x, y) => x + y);
if (a.kind !== "ok") throw new Error("cả hai ok phải ra ok");
if (a.kind === "ok" && a.giaTri !== 7) throw new Error("phải ra 7");

const b = map2GomLoi(loiMot<number, string>("lỗi a"), ok<number, string[]>(4), (x, y) => x + y);
if (b.kind !== "loi") throw new Error("ra lỗi phải ra loi");
if (b.kind === "loi" && JSON.stringify(b.loi) !== JSON.stringify(["lỗi a"])) throw new Error("phải có đúng lỗi a");

const c = map2GomLoi(ok<number, string[]>(3), loiMot<number, string>("lỗi b"), (x, y) => x + y);
if (c.kind !== "loi") throw new Error("rb lỗi phải ra loi");
if (c.kind === "loi" && JSON.stringify(c.loi) !== JSON.stringify(["lỗi b"])) throw new Error("phải có đúng lỗi b");

const d = map2GomLoi(loiMot<number, string>("lỗi a"), loiMot<number, string>("lỗi b"), (x, y) => x + y);
if (d.kind !== "loi") throw new Error("cả hai lỗi phải ra loi");
if (d.kind === "loi" && JSON.stringify(d.loi) !== JSON.stringify(["lỗi a", "lỗi b"])) throw new Error("phải gom ĐỦ cả hai lỗi, không mất cái nào");
```

:::hints
- kind: attention
  body: "Bốn chỗ trống, bốn trường hợp — QUAN TRỌNG: kiểm \"cả hai lỗi\" TRƯỚC hai trường hợp còn lại, không phải sau (nếu sau, lỗi thứ hai sẽ không bao giờ được gom)."
- kind: strategy
  body: '1. Cả hai lỗi: { kind: "loi", loi: [...ra.loi, ...rb.loi] } (gộp hai mảng). 2. ra lỗi: { kind: "loi", loi: ra.loi }. 3. rb lỗi: { kind: "loi", loi: rb.loi }. 4. Cả hai ok: ok(f(ra.giaTri, rb.giaTri)).'
- kind: one-line
  body: "{ kind: \"loi\", loi: [...ra.loi, ...rb.loi] }  /  { kind: \"loi\", loi: ra.loi }  /  { kind: \"loi\", loi: rb.loi }  /  ok(f(ra.giaTri, rb.giaTri))"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "thiếu tên"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn trường hợp, thứ tự ĐÚNG — `map2GomLoi` gom ĐỦ mọi lỗi, không bỏ
sót trường hợp nào, kể cả trường hợp DỄ QUÊN nhất.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã có `map2GomLoi` hoạt động đúng. Áp dụng nó vào một tình huống
THẬT — validate một FORM có nhiều trường — trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
