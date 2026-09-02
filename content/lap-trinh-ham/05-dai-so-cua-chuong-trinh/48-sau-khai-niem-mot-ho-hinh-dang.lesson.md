---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.sau-khai-niem-mot-ho-hinh-dang
title: "Sáu khái niệm — MỘT họ hình dạng, không phải sáu thứ rời rạc"
summary: "Monoid, Functor, Applicative, Monad, Traverse, Catamorphism — sáu công cụ đã tự tay viết suốt track này, xếp cạnh nhau lần đầu. Cùng một câu hỏi lặp lại sáu lần: có một khuôn lặp lại, đặt tên nó, viết MỘT LẦN, dùng lại thay vì chép tay mỗi lần."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 48
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [alg.six-concepts-one-family]
requires: [alg.review-catamorphism]
concepts: [alg.six-concepts-one-family]
gradingMatrix:
  web-chrome: []
  web-firefox: []
  macos: []
  windows: []
  linux: []
  android: []
  ios: []
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Bài trước đo lại Catamorphism — công cụ CUỐI trong sáu cái track này
dạy, từ Monoid (bài 1) tới đây. Hôm nay không có công cụ mới — dừng
lại, xếp CẢ SÁU cạnh nhau: sáu thứ rời rạc phải nhớ riêng, hay MỘT câu
hỏi lặp lại sáu lần?
::::

::::explain{#sau-cong-cu-mot-luot}
```typescript
interface Monoid<T> { ketHop: (a: T, b: T) => T; rong: T; }
const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };
function gopTatCa<T>(m: Monoid<T>, ds: T[]): T { return ds.reduce(m.ketHop, m.rong); }

type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co": return co(f(o.giaTri));
    case "khong": return khong();
  }
}
function chainOption<T, U>(o: Option<T>, f: (x: T) => Option<U>): Option<U> {
  switch (o.kind) {
    case "co": return f(o.giaTri);
    case "khong": return khong();
  }
}

type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

function traverseOption<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const ketQua: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    ketQua.push(o.giaTri);
  }
  return co(ketQua);
}

type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };
interface BoXuLyExpr<R> { so: (giaTri: number) => R; cong: (trai: R, phai: R) => R; nhan: (trai: R, phai: R) => R; }
function foldExpr<R>(e: Expr, bxl: BoXuLyExpr<R>): R {
  switch (e.kind) {
    case "so": return bxl.so(e.giaTri);
    case "cong": return bxl.cong(foldExpr(e.trai, bxl), foldExpr(e.phai, bxl));
    case "nhan": return bxl.nhan(foldExpr(e.trai, bxl), foldExpr(e.phai, bxl));
  }
}

console.log(gopTatCa(monoidCong, [1, 2, 3]));
console.log(mapOption(co(5), (x) => x * 2));
console.log(chainOption(co(5), (x) => (x > 0 ? co(x * 2) : khong())));
console.log(map2GomLoi(ok<number, string[]>(2), ok<number, string[]>(3), (a, b) => a + b));
console.log(traverseOption([1, 2, 3], (x) => co(x * 10)));
const bieuThuc: Expr = { kind: "cong", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } };
console.log(foldExpr(bieuThuc, { so: (g) => g, cong: (t, p) => t + p, nhan: (t, p) => t * p }));
```

```text
6
{"kind":"co","giaTri":10}
{"kind":"co","giaTri":10}
{"kind":"ok","giaTri":5}
{"kind":"co","giaTri":[10,20,30]}
5
```

Sáu dòng, sáu công cụ đã viết ở sáu chỗ khác nhau suốt track — không
cái nào mới ở đây. Xếp cạnh nhau, mỗi dòng trả lời một câu hỏi khác
nhau về HÌNH DẠNG dữ liệu:

- `gopTatCa` (Monoid, bài 1-7): `ketHop` kết hợp HAI giá trị CÙNG kiểu
  thành MỘT — `gopTatCa` lặp lại phép đó qua CẢ một mảng.
- `mapOption` (Functor, bài 14-20): biến đổi giá trị BÊN TRONG một
  `Option`, giữ nguyên CÁI VỎ (`co`/`khong`) không đổi.
- `map2GomLoi` (Applicative, bài 21-27): kết hợp NHIỀU giá trị ĐỘC LẬP
  cùng lúc — nếu có lỗi, GOM hết lại, không dừng ở lỗi đầu tiên.
- `chainOption` (Monad, bài 29-34): chuỗi bước PHỤ THUỘC — bước sau
  cần giá trị THẬT của bước trước, làm PHẲNG kết quả thay vì lồng.
- `traverseOption` (Traverse, bài 35-40): lật một MẢNG `Option` thành
  một `Option` chứa MẢNG — đổi thứ tự hai lớp container cho nhau.
- `foldExpr` (Catamorphism, bài 41-47): tách phần ĐỆ QUY (đi qua cây)
  ra RIÊNG khỏi phần TÍNH TOÁN (`so`/`cong`/`nhan` làm gì) — viết đệ
  quy MỘT LẦN, dùng cho bao nhiêu phép tính cũng được.

Sáu câu hỏi khác nhau — về HAI giá trị, về BÊN TRONG một container, về
NHIỀU giá trị độc lập, về CHUỖI phụ thuộc, về LẬT hai lớp, về TÁCH đệ
quy. Nhưng cách trả lời CẢ SÁU câu hỏi đó GIỐNG HỆT nhau: có một khuôn
CODE lặp lại nhiều lần trong chương trình — đặt tên nó (`Monoid`,
`map`, `map2GomLoi`, `chain`, `traverse`, `fold`), viết đúng MỘT lần,
gọi lại mỗi khi khuôn đó xuất hiện, thay vì chép tay lại từng chỗ.
::::

::::example{#mot-ho-dung-duoc-o-noi-moi}
Nếu sáu công cụ trên thật sự là MỘT họ (cùng một cách trả lời), chúng
phải DÙNG LẠI được ở bất kỳ kiểu dữ liệu MỚI nào — không chỉ những ví
dụ đã quen suốt track. Thử với hai `Monoid` chưa từng gặp, và một phép
chia an toàn:

```typescript title=readonly
interface Monoid<T> { ketHop: (a: T, b: T) => T; rong: T; }
function gopTatCa<T>(m: Monoid<T>, ds: T[]): T { return ds.reduce(m.ketHop, m.rong); }

const monoidNhan: Monoid<number> = { ketHop: (a, b) => a * b, rong: 1 };
const monoidNoiChuoi: Monoid<string> = { ketHop: (a, b) => a + b, rong: "" };

type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function chainOption<T, U>(o: Option<T>, f: (x: T) => Option<U>): Option<U> {
  switch (o.kind) {
    case "co": return f(o.giaTri);
    case "khong": return khong();
  }
}

const chiaAnToan = (a: number, b: number): Option<number> => (b === 0 ? khong() : co(a / b));

console.log(gopTatCa(monoidNhan, [2, 3, 4]));
console.log(gopTatCa(monoidNoiChuoi, ["mot", "hai", "ba"]));
console.log(chainOption(co(100), (x) => chiaAnToan(x, 5)));
console.log(chainOption(co(100), (x) => chiaAnToan(x, 0)));
```

```text title=readonly
24
mothaiba
{"kind":"co","giaTri":20}
{"kind":"khong"}
```

`gopTatCa` không đổi MỘT dòng nào — vẫn ĐÚNG interface `Monoid<T>` bài
3 định nghĩa — nhưng chạy đúng cho `monoidNhan` (tích) VÀ
`monoidNoiChuoi` (nối chuỗi), hai giá trị hoàn toàn MỚI, chưa từng
xuất hiện ở bài trước. `chainOption` cũng vậy: cùng MỘT hàm, áp dụng
cho `chiaAnToan` — một phép tính chưa từng dùng chung với `chainOption`
trong track — vẫn chạy đúng cả hai kết cục (chia được thì `co`, chia
cho `0` thì `khong`), không cần sửa lại `chainOption` một chữ nào. Đây
CHÍNH LÀ lợi ích của việc đặt tên khuôn: viết `gopTatCa`/`chainOption`
đúng MỘT lần, dùng ở bất kỳ đâu khuôn đó xuất hiện — kể cả những chỗ
chưa hề nghĩ tới lúc viết chúng.
::::

::::predict{#doan-chain-khong-gom-loi commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return r;
  }
}
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }

const kiemTen = (ten: string): Result<string, string[]> => (ten.length > 0 ? ok(ten) : loiMot("tên rỗng"));
const kiemTuoi = (tuoi: number): Result<number, string[]> => (tuoi >= 0 ? ok(tuoi) : loiMot("tuổi âm"));

const ketQua = chainResult(kiemTen(""), () => kiemTuoi(-5));
console.log(ketQua);
```

Dòng cuối in ra gì?

:::opt{correct}
`{"kind":"loi","loi":["tên rỗng"]}`
:::

:::opt
`{"kind":"loi","loi":["tên rỗng","tuổi âm"]}` — vì `kiemTen("")` VÀ
`kiemTuoi(-5)` đều sai, `chainResult` phải GOM cả hai lỗi lại, giống
`map2GomLoi` đã làm ở bài 21-27
::why
Gần đúng ở việc bạn nhớ ĐÚNG có một công cụ THẬT gộp lỗi của nhiều
bước — `map2GomLoi` (bài 21-27) làm chính xác việc đó khi HAI `Result`
ĐỘC LẬP, không phụ thuộc nhau.

Chỗ lệch: `chainResult` không phải `map2GomLoi`. `chainResult` diễn tả
một chuỗi PHỤ THUỘC — bước sau chỉ CHẠY khi bước trước đã `"ok"`.
`kiemTen("")` thất bại NGAY (`loi: ["tên rỗng"]`), nên `chainResult`
trả về CHÍNH `r` đó và KHÔNG BAO GIỜ gọi `f` — nghĩa là `kiemTuoi(-5)`
không hề chạy, `"tuổi âm"` không hề được sinh ra để mà gộp vào đâu cả.
::
:::

:::opt
Máy báo lỗi biên dịch — callback `() => kiemTuoi(-5)` không nhận tham
số nào, trong khi `chainResult` yêu cầu `f` phải nhận đúng giá trị đã
unwrap từ `r`
::why
Gần đúng ở việc bạn để ý callback `() => kiemTuoi(-5)` không dùng tới
giá trị mà `chainResult` đã unwrap từ `r` (tham số bị bỏ qua hoàn
toàn) — quan sát về CHỮ KÝ đó đúng.

Chỗ lệch: TypeScript CHO PHÉP một callback khai báo ÍT tham số hơn
kiểu hàm yêu cầu (phần thân không bắt buộc phải dùng hết những gì
được truyền vào) — biên dịch sạch, không lỗi gì. Vấn đề DUY NHẤT ở
đây là bước hai (`kiemTuoi`) không CHẠY vì bước một đã lỗi trước —
một chuyện xảy ra lúc CHẠY, không phải lỗi lúc biên dịch.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Monoid, Functor, Applicative, Monad, Traverse, Catamorphism — sáu cái
tên, một câu hỏi: có một khuôn lặp lại, đặt tên nó, viết MỘT LẦN, dùng
lại thay vì chép tay mỗi lần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sáu công cụ đã đứng RIÊNG LẺ suốt bài học — mỗi ví dụ chỉ dùng ĐÚNG
MỘT trong sáu. Nhưng một bài toán THẬT thường cần NHIỀU công cụ CÙNG
LÚC — chẳng hạn: biến đổi (Functor) MỖI phần tử của một mảng, RỒI gộp
(Monoid) tất cả lại thành MỘT giá trị. Hai công cụ, một bài toán —
ghép được không?

Bài sau ghép CẶP ĐẦU TIÊN: Monoid + Functor.
::::

::::checkpoint{mastery=0.8}
::::
