---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.do-tong-hop-functor
title: "Đo tổng hợp: Functor"
summary: "Viết mapResult từ đầu (chưa cho sẵn khung), rồi TỰ kiểm nó tuân luật identity (bằng test, không phải chứng minh hình thức). Không khái niệm mới — đo khả năng tự dựng một Functor ĐÚNG luật, không chỉ đúng kiểu."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 20
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [alg.review-functor]
requires: [alg.functor-law-composition]
concepts: [alg.review-functor]
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
Functor: định nghĩa, hai luật, ba ví dụ (`Option`, `Result`, `Array`).
Hôm nay: TỰ viết `mapResult` từ đầu, TỰ kiểm nó tuân luật identity.
::::

::::explain{#tu-viet-va-tu-kiem}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function mapResult<T, U, E>(r: Result<T, E>, f: (x: T) => U): Result<U, E> {
  switch (r.kind) {
    case "ok":
      return ok(f(r.giaTri));
    case "loi":
      return loi(r.loi);
  }
}

const id = (x: number) => x;
const a: Result<number, string> = ok(7);
const daMap = mapResult(a, id);

console.log(daMap);
console.log(JSON.stringify(daMap) === JSON.stringify(a));
```

```text
{"kind":"ok","giaTri":7}
true
```

Không chỉ VIẾT `mapResult` (bài 15 đã làm việc đó) — bài này còn TỰ
KIỂM nó (bằng cách CHẠY THỬ với `id`, SO SÁNH kết quả với giá trị
gốc, đúng phương pháp bài 18 đã dạy). `mapResult(a, id)` ra ĐÚNG `a`
— luật identity ĐÚNG với `mapResult` tự viết này.

Đây là điều bài 18 đã nhấn mạnh: một `map` ĐÚNG CHỮ KÝ chưa đủ — phải
CHẠY THỬ để chắc chắn nó tuân luật, không chỉ tin vào việc code "trông
đúng".
::::

::::example{#kiem-luat-tren-nhieu-gia-tri}
Kiểm luật identity trên NHIỀU giá trị khác nhau — không chỉ MỘT lần
thử là đủ tin tưởng:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function mapResult<T, U, E>(r: Result<T, E>, f: (x: T) => U): Result<U, E> {
  switch (r.kind) {
    case "ok": return ok(f(r.giaTri));
    case "loi": return loi(r.loi);
  }
}

const id = (x: number) => x;
const cacGiaTri: Result<number, string>[] = [ok(0), ok(-5), ok(100), loi("lỗi mẫu")];

for (const r of cacGiaTri) {
  const daMap = mapResult(r, id);
  console.log(JSON.stringify(daMap) === JSON.stringify(r));
}
```

```text title=readonly
true
true
true
true
```

Kiểm trên `ok(0)` (giá trị "falsy", dễ gây bẫy — bài 8-9 đã dạy),
`ok(-5)` (số âm), `ok(100)`, VÀ `loi("lỗi mẫu")` (nhánh lỗi) — TẤT CẢ
đều `true`. Kiểm nhiều trường hợp khác nhau (không chỉ MỘT ví dụ đẹp)
là cách THẬT SỰ đáng tin để xác nhận một luật.
::::

::::predict{#doan-tong-ket-functor commitOnce}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function mapOptionSai3<T>(o: Option<T>, f: (x: T) => T): Option<T> {
  switch (o.kind) {
    case "co":
      return o;
    case "khong":
      return khong();
  }
}

const id = (x: number) => x;
const congMuoi = (x: number) => x + 10;

console.log(JSON.stringify(mapOptionSai3(co(5), id)) === JSON.stringify(co(5)));
console.log(JSON.stringify(mapOptionSai3(co(5), congMuoi)) === JSON.stringify(co(15)));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`true` rồi `false`
:::

:::opt
`true` rồi `true` — vì `mapOptionSai3` LUÔN trả về `o` nguyên vẹn, và
`co(5)` với `congMuoi` áp dụng đúng vẫn PHẢI ra `co(15)`
::why
Gần đúng ở việc bạn tính đúng KẾT QUẢ MONG MUỐN của `congMuoi` áp dụng
lên `5` (`5 + 10 = 15`) — phép tính đó đúng NẾU `f` THẬT SỰ được áp
dụng.

Chỗ lệch: đọc kỹ `mapOptionSai3` — nhánh `case "co"` trả về `o` NGUYÊN
VẸN (`return o;`), KHÔNG hề gọi `f` ở bất kỳ đâu! Với `f = id`, "không
gọi f" TÌNH CỜ vẫn ra ĐÚNG kết quả (`id` không đổi gì nên "không gọi
nó" và "gọi nó" trông giống nhau) — luật identity trông như ĐÚNG. Nhưng
với `f = congMuoi` (một hàm THẬT SỰ đổi giá trị), "không gọi f" lộ rõ:
kết quả VẪN là `co(5)` (giá trị GỐC, chưa cộng gì), không phải `co(15)`.
::
:::

:::opt
`false` rồi `false` — vì `mapOptionSai3` có lỗi CẢ HAI trường hợp,
không riêng gì `congMuoi`
::why
Gần đúng ở việc bạn nghi ngờ `mapOptionSai3` CÓ LỖI — nghi ngờ đó đúng,
hàm này THẬT SỰ có lỗi (không gọi `f`).

Chỗ lệch: LUẬT IDENTITY cụ thể (`f = id`) KHÔNG PHÁT HIỆN được lỗi này
— vì "không gọi `id`" và "gọi `id`" cho CÙNG kết quả (đúng điều bài
này minh hoạ: một `map` sai có thể TÌNH CỜ qua được MỘT luật). Dòng
ĐẦU vẫn là `true`. Chỉ khi thử với hàm KHÁC `id` (như `congMuoi`) lỗi
mới lộ ra — đúng lý do cần kiểm NHIỀU trường hợp, không chỉ luật
identity.
::
:::
::::

::::code{#tu_kiem_mapresult}
Viết `mapResult<T, U, E>` từ đầu, VÀ một hàm `kiemLuatIdentity` tự
kiểm nó tuân luật identity.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function mapResult<T, U, E>(r: Result<T, E>, f: (x: T) => U): Result<U, E> {
  ___
}

function kiemLuatIdentity(r: Result<number, string>): boolean {
  const id = (x: number) => x;
  const daMap = mapResult(r, id);
  return JSON.stringify(daMap) === JSON.stringify(r);
}

console.log(kiemLuatIdentity(ok(7)));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function mapResult<T, U, E>(r: Result<T, E>, f: (x: T) => U): Result<U, E> {
  switch (r.kind) {
    case "ok":
      return ok(f(r.giaTri));
    case "loi":
      return loi(r.loi);
  }
}

function kiemLuatIdentity(r: Result<number, string>): boolean {
  const id = (x: number) => x;
  const daMap = mapResult(r, id);
  return JSON.stringify(daMap) === JSON.stringify(r);
}

console.log(kiemLuatIdentity(ok(7)));
```

```typescript title=test
if (kiemLuatIdentity(ok(7)) !== true) throw new Error("luật identity phải đúng với ok(7)");
if (kiemLuatIdentity(ok(0)) !== true) throw new Error("luật identity phải đúng với ok(0), kể cả giá trị falsy");
if (kiemLuatIdentity(loi("lỗi mẫu")) !== true) throw new Error("luật identity phải đúng với loi(...)");
const b = mapResult(ok<number, string>(5), (x) => x * 3);
if (b.kind !== "ok") throw new Error("map trên ok phải giữ kind ok");
if (b.kind === "ok" && b.giaTri !== 15) throw new Error("mapResult(ok(5), x*3) phải ra 15");
```

:::hints
- kind: attention
  body: "mapResult phải THẬT SỰ áp dụng f lên r.giaTri ở nhánh \"ok\" (không chỉ trả về r nguyên vẹn), và giữ nguyên r.loi ở nhánh \"loi\"."
- kind: strategy
  body: 'switch (r.kind) { case "ok": return ok(f(r.giaTri)); case "loi": return loi(r.loi); } — đúng khuôn bài 15 đã viết.'
- kind: one-line
  body: "switch (r.kind) {\n  case \"ok\": return ok(f(r.giaTri));\n  case \"loi\": return loi(r.loi);\n}"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một `map` đúng phải qua được luật identity TRÊN NHIỀU giá trị, và
composition — không chỉ "trông giống" một Functor hợp lệ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn có `mapOption`/`mapResult` — biến đổi giá trị BÊN TRONG một
`Option`/`Result`. Nhưng nếu có BA `Result` ĐỘC LẬP, muốn kiểm tra
CẢ BA cùng lúc — `map` (chỉ xử lý MỘT giá trị) có đủ không?

Cụm sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
