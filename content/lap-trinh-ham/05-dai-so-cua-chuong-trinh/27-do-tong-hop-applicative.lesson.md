---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.do-tong-hop-applicative
title: "Đo tổng hợp: Applicative"
summary: "Viết map3GomLoi (ba Result độc lập, không phải hai) và dùng nó validate một form BA trường MỚI. Không khái niệm mới — đo khả năng mở rộng khuôn map2 lên nhiều đối số hơn, bằng cách LỒNG map2GomLoi (đúng kỹ thuật bài 26 đã dạy)."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 27
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [alg.review-applicative]
requires: [alg.apply-form-validation]
concepts: [alg.review-applicative]
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
Cụm 4 chốt lại: fail-fast, collect-all, ghép form nhiều trường. Hôm
nay: viết `map3GomLoi` — không phải khái niệm mới, chỉ MỞ RỘNG.
::::

::::explain{#map3-tu-map2}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

function map3GomLoi<A, B, C, D, E>(
  ra: Result<A, E[]>,
  rb: Result<B, E[]>,
  rc: Result<C, E[]>,
  f: (a: A, b: B, c: C) => D
): Result<D, E[]> {
  const ab = map2GomLoi(ra, rb, (a, b) => ({ a, b }));
  return map2GomLoi(ab, rc, (cap, c) => f(cap.a, cap.b, c));
}
```

`map3GomLoi` KHÔNG viết lại logic gộp lỗi từ đầu — nó DÙNG LẠI
`map2GomLoi` (đã có, đã kiểm) HAI LẦN: bước ĐẦU ghép `ra`/`rb` thành
một cặp `{a, b}` (giữ CẢ HAI giá trị, gộp lỗi nếu có); bước SAU ghép
kết quả ĐÓ với `rc`, rồi gọi `f` với ĐỦ CẢ BA giá trị gốc
(`cap.a`, `cap.b`, `c`). Đúng kỹ thuật bài 26 đã dùng để ghép BA
trường form — giờ ĐÓNG GÓI thành một hàm CÓ TÊN, dùng lại được, không
phải viết lồng tay mỗi lần.
::::

::::example{#map3-tren-form-moi}
Dùng `map3GomLoi` cho một form MỚI — địa chỉ giao hàng:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}
function map3GomLoi<A, B, C, D, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, rc: Result<C, E[]>, f: (a: A, b: B, c: C) => D): Result<D, E[]> {
  const ab = map2GomLoi(ra, rb, (a, b) => ({ a, b }));
  return map2GomLoi(ab, rc, (cap, c) => f(cap.a, cap.b, c));
}

function kiemDuong(duong: string): Result<string, string[]> {
  return duong.length > 0 ? ok(duong) : loiMot("địa chỉ đường không được rỗng");
}
function kiemThanhPho(tp: string): Result<string, string[]> {
  return tp.length > 0 ? ok(tp) : loiMot("thành phố không được rỗng");
}
function kiemMaBuuChinh(ma: string): Result<string, string[]> {
  return /^\d{5,6}$/.test(ma) ? ok(ma) : loiMot("mã bưu chính phải có 5-6 chữ số");
}

const ketQua = map3GomLoi(kiemDuong(""), kiemThanhPho("Hà Nội"), kiemMaBuuChinh("abc"), (duong, tp, ma) => ({ duong, tp, ma }));
console.log(ketQua);
```

```text title=readonly
{"kind":"loi","loi":["địa chỉ đường không được rỗng","mã bưu chính phải có 5-6 chữ số"]}
```

`duong` rỗng VÀ `maBuuChinh` sai định dạng (`"abc"` không phải chữ
số) — CẢ HAI lỗi được gộp ĐỦ (`thanhPho` hợp lệ, không góp lỗi nào).
`map3GomLoi` xử lý ĐÚNG form BA trường HOÀN TOÀN MỚI, không lặp lại
form `ten`/`tuoi`/`email` của bài 26 — chứng minh nó dùng lại được cho
BẤT KỲ ba `Result` độc lập nào.
::::

::::predict{#doan-map3-mot-loi commitOnce}
Dùng lại ba hàm kiểm ở trên. Nếu CHỈ `kiemThanhPho("")` sai (rỗng), CÒN
`kiemDuong`/`kiemMaBuuChinh` hợp lệ — mảng lỗi cuối cùng có MẤY phần
tử?

:::opt{correct}
Một phần tử
:::

:::opt
Không phần tử nào (mảng rỗng) — vì `map2GomLoi` bên TRONG chỉ gộp lỗi
của `ra`/`rb` bước ĐẦU, không "truyền" được lỗi của `rc` (thành phố)
ra bước ghép THỨ HAI
::why
Gần đúng ở việc bạn để ý CẤU TRÚC `map3GomLoi` GHÉP theo HAI BƯỚC
(`ab` trước, rồi ghép với `rc` sau) — cấu trúc đó ĐÚNG.

Chỗ lệch: bước GHÉP THỨ HAI (`map2GomLoi(ab, rc, ...)`) VẪN kiểm
`rc.kind === "loi"` bình thường — nếu `rc` (ở đây là `kiemThanhPho`)
lỗi, lỗi ĐÓ được gộp vào kết quả CUỐI đúng như MỌI lời gọi `map2GomLoi`
khác, không hề "biến mất" chỉ vì nó là tham số THỨ BA thay vì thứ
nhất/thứ hai.
::
:::

:::opt
Máy báo lỗi — `map3GomLoi` không xử lý đúng khi CHỈ tham số THỨ HAI
(`rb`, ở giữa) lỗi, chỉ hoạt động đúng khi `ra` hoặc `rc` lỗi
::why
Gần đúng ở việc bạn cân nhắc VỊ TRÍ của tham số lỗi (đầu/giữa/cuối) có
thể ảnh hưởng gì đó tới cách `map3GomLoi` xử lý — một câu hỏi hợp lý
khi hàm có cấu trúc HAI BƯỚC như thế này.

Chỗ lệch: KHÔNG có sự khác biệt nào giữa "lỗi ở vị trí đầu/giữa/cuối"
— cả ba tham số (`ra`, `rb`, `rc`) đều được `map2GomLoi` (bên trong)
kiểm ĐÚNG NHƯ NHAU, không có tham số nào "đặc biệt" hay "khó xử lý
hơn" vì vị trí của nó.
::
:::
::::

::::code{#map3_gom_loi}
Viết `map3GomLoi<A, B, C, D, E>` bằng cách LỒNG `map2GomLoi` HAI lần.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

function map3GomLoi<A, B, C, D, E>(
  ra: Result<A, E[]>,
  rb: Result<B, E[]>,
  rc: Result<C, E[]>,
  f: (a: A, b: B, c: C) => D
): Result<D, E[]> {
  const ab = ___;
  return ___;
}

function kiemTen(ten: string): Result<string, string[]> {
  return ten.length > 0 ? ok(ten) : loiMot("tên rỗng");
}
function kiemTuoi(tuoi: number): Result<number, string[]> {
  return tuoi >= 0 ? ok(tuoi) : loiMot("tuổi âm");
}
function kiemDiem(diem: number): Result<number, string[]> {
  return diem >= 0 && diem <= 10 ? ok(diem) : loiMot("điểm ngoài 0-10");
}

console.log(map3GomLoi(kiemTen(""), kiemTuoi(-5), kiemDiem(15), (t, tu, d) => ({ t, tu, d })));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

function map3GomLoi<A, B, C, D, E>(
  ra: Result<A, E[]>,
  rb: Result<B, E[]>,
  rc: Result<C, E[]>,
  f: (a: A, b: B, c: C) => D
): Result<D, E[]> {
  const ab = map2GomLoi(ra, rb, (a, b) => ({ a, b }));
  return map2GomLoi(ab, rc, (cap, c) => f(cap.a, cap.b, c));
}

function kiemTen(ten: string): Result<string, string[]> {
  return ten.length > 0 ? ok(ten) : loiMot("tên rỗng");
}
function kiemTuoi(tuoi: number): Result<number, string[]> {
  return tuoi >= 0 ? ok(tuoi) : loiMot("tuổi âm");
}
function kiemDiem(diem: number): Result<number, string[]> {
  return diem >= 0 && diem <= 10 ? ok(diem) : loiMot("điểm ngoài 0-10");
}

console.log(map3GomLoi(kiemTen(""), kiemTuoi(-5), kiemDiem(15), (t, tu, d) => ({ t, tu, d })));
```

```typescript title=test
const a = map3GomLoi(kiemTen("An"), kiemTuoi(30), kiemDiem(8), (t, tu, d) => ({ t, tu, d }));
if (a.kind !== "ok") throw new Error("cả ba hợp lệ phải ra ok");

const b = map3GomLoi(kiemTen(""), kiemTuoi(-5), kiemDiem(15), (t, tu, d) => ({ t, tu, d }));
if (b.kind !== "loi") throw new Error("cả ba sai phải ra loi");
if (b.kind === "loi" && b.loi.length !== 3) throw new Error("phải gom ĐỦ cả ba lỗi");

const c = map3GomLoi(kiemTen("An"), kiemTuoi(-5), kiemDiem(8), (t, tu, d) => ({ t, tu, d }));
if (c.kind !== "loi") throw new Error("một trường sai phải ra loi");
if (c.kind === "loi" && c.loi.length !== 1) throw new Error("chỉ một trường sai thì mảng lỗi phải có đúng một phần tử");
```

:::hints
- kind: attention
  body: "Hai chỗ trống: bước ĐẦU ghép ra/rb thành một cặp {a, b} bằng map2GomLoi. Bước SAU (return) ghép cặp đó với rc, gọi f với ĐỦ ba giá trị gốc."
- kind: strategy
  body: 'const ab = map2GomLoi(ra, rb, (a, b) => ({ a, b })); return map2GomLoi(ab, rc, (cap, c) => f(cap.a, cap.b, c)); — dùng LẠI map2GomLoi, không viết logic gộp lỗi mới.'
- kind: one-line
  body: "const ab = map2GomLoi(ra, rb, (a, b) => ({ a, b }));\nreturn map2GomLoi(ab, rc, (cap, c) => f(cap.a, cap.b, c));"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "tên rỗng"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`map3GomLoi` không phải khái niệm mới — nó là `map2GomLoi` DÙNG LẠI
hai lần. Applicative mở rộng được lên bao nhiêu giá trị độc lập cũng
được, cùng một kỹ thuật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cụm 4 chốt lại — Applicative ghép NHIỀU giá trị ĐỘC LẬP. Nhưng nếu một
phép tính PHỤ THUỘC kết quả của phép tính TRƯỚC (không độc lập) — như
`chia(100, x)` cần `x` đã tính XONG — `map2`/`map3` có đủ không?

Cụm sau trả lời — Monad.
::::

::::checkpoint{mastery=0.8}
::::
