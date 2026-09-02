---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.do-tong-hop-monad
title: "Đo tổng hợp: Monad"
summary: "xuLyPhanTu(ds, i): Option<number> — chainOption(chainOption(layPhanTu(ds, i), x => nghichDao(x)), y => canBac2(y)). Chuỗi ba bước KHÁC bài 33 (mảng, không phải chuỗi ký tự) — không khái niệm mới, đo khả năng tự nhận diện lúc nào cần chain."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 34
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [alg.review-monad]
requires: [alg.real-chain-example]
concepts: [alg.review-monad]
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
Bài chốt cụm — không khái niệm mới. Một chuỗi BA bước KHÁC, dùng
`Option` thay vì `Result`. Bạn tự ghép, không nhắc lại từng bước.
::::

::::explain{#bai-toan-mang}
`xuLyPhanTu(ds: number[], i: number): Option<number>` — nhận một mảng
và một chỉ số, làm BA việc theo thứ tự:

1. Lấy phần tử tại chỉ số `i` (`layPhanTu`) — không có nếu chỉ số nằm
   NGOÀI mảng.
2. Lấy nghịch đảo (`1 / x`) của phần tử đó (`nghichDao`) — không có
   nếu phần tử là `0`.
3. Lấy căn bậc hai kết quả (`canBac2`) — không có nếu kết quả âm.

CÙNG HÌNH DẠNG bài 33 (ba bước, mỗi bước CẦN giá trị THẬT của bước
trước) nhưng miền dữ liệu KHÁC HẲN (mảng số, không phải chuỗi ký tự)
và kiểu bọc KHÁC (`Option`, không phải `Result` — không cần LÝ DO lỗi,
chỉ cần biết CÓ hay KHÔNG):

```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function chainOption<T, U>(o: Option<T>, f: (x: T) => Option<U>): Option<U> {
  switch (o.kind) {
    case "co": return f(o.giaTri);
    case "khong": return khong();
  }
}

function layPhanTu(ds: number[], i: number): Option<number> {
  const gt = ds[i];
  return gt === undefined ? khong() : co(gt);
}
function nghichDao(x: number): Option<number> {
  return x === 0 ? khong() : co(1 / x);
}
function canBac2(x: number): Option<number> {
  return x < 0 ? khong() : co(Math.sqrt(x));
}

function xuLyPhanTu(ds: number[], i: number): Option<number> {
  return chainOption(
    chainOption(layPhanTu(ds, i), (x) => nghichDao(x)),
    (y) => canBac2(y),
  );
}

const ds = [4, 0, -9, 16];
console.log(JSON.stringify(xuLyPhanTu(ds, 0)));
```

```text
{"kind":"co","giaTri":0.5}
```

`layPhanTu(ds, 0)` ra `co(4)` → `nghichDao(4)` ra `co(0.25)` →
`canBac2(0.25)` ra `co(0.5)`. Cùng khuôn `chainResult(chainResult(...),
...)` bài 33, đổi tên biến thể — đúng bản chất Monad: MỘT khuôn, nhiều
kiểu bọc khác nhau đều dùng được.
::::

::::example{#ba-duong-loi-khac-nhau}
Ba chỉ số khác nhau, ba đường "không có" khác nhau — mỗi bước tự lo
phần của nó:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function chainOption<T, U>(o: Option<T>, f: (x: T) => Option<U>): Option<U> {
  switch (o.kind) {
    case "co": return f(o.giaTri);
    case "khong": return khong();
  }
}
function layPhanTu(ds: number[], i: number): Option<number> {
  const gt = ds[i];
  return gt === undefined ? khong() : co(gt);
}
function nghichDao(x: number): Option<number> {
  return x === 0 ? khong() : co(1 / x);
}
function canBac2(x: number): Option<number> {
  return x < 0 ? khong() : co(Math.sqrt(x));
}
function xuLyPhanTu(ds: number[], i: number): Option<number> {
  return chainOption(
    chainOption(layPhanTu(ds, i), (x) => nghichDao(x)),
    (y) => canBac2(y),
  );
}

const ds = [4, 0, -9, 16];
console.log(xuLyPhanTu(ds, 1).kind);
console.log(xuLyPhanTu(ds, 2).kind);
console.log(xuLyPhanTu(ds, 5).kind);
```

```text title=readonly
khong
khong
khong
```

`ds[1] = 0` → `nghichDao(0)` KHÔNG có (bước 2 lỗi, bước 3 không chạy).
`ds[2] = -9` → `nghichDao(-9)` ra `co(-1/9)`, NHƯNG `canBac2(-1/9)`
KHÔNG có vì `-1/9` âm (bước 3 lỗi). `ds[5]` NGOÀI mảng (chỉ có 4 phần
tử) → `layPhanTu` KHÔNG có NGAY (bước 1 lỗi, bước 2, 3 không chạy).
Ba lý do khác nhau, CÙNG kết quả `"khong"` — `Option` không phân biệt
LÝ DO (khác `Result`, bài 33), chỉ biết "không có".
::::

::::predict{#doan-xulyphantu-index-3 commitOnce}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function chainOption<T, U>(o: Option<T>, f: (x: T) => Option<U>): Option<U> {
  switch (o.kind) {
    case "co": return f(o.giaTri);
    case "khong": return khong();
  }
}
function layPhanTu(ds: number[], i: number): Option<number> {
  const gt = ds[i];
  return gt === undefined ? khong() : co(gt);
}
function nghichDao(x: number): Option<number> {
  return x === 0 ? khong() : co(1 / x);
}
function canBac2(x: number): Option<number> {
  return x < 0 ? khong() : co(Math.sqrt(x));
}
function xuLyPhanTu(ds: number[], i: number): Option<number> {
  return chainOption(
    chainOption(layPhanTu(ds, i), (x) => nghichDao(x)),
    (y) => canBac2(y),
  );
}

const ds = [4, 0, -9, 16];
console.log(JSON.stringify(xuLyPhanTu(ds, 3)));
```

Dòng cuối in ra gì?

:::opt{correct}
`{"kind":"co","giaTri":0.25}`
:::

:::opt
`{"kind":"khong"}` — vì chỉ số `3` vượt quá phạm vi hợp lệ của mảng
bốn phần tử
::why
Gần đúng ở việc bạn cẩn thận kiểm tra biên mảng trước khi tính — thói
quen đó đúng.

Chỗ lệch: `ds = [4, 0, -9, 16]` có ĐÚNG bốn phần tử, chỉ số hợp lệ là
`0`, `1`, `2`, `3` — chỉ số `3` (không phải `4`) VẪN nằm TRONG phạm
vi, trỏ tới phần tử CUỐI CÙNG (`16`), không phải ngoài mảng.
::
:::

:::opt
`{"kind":"co","giaTri":4}` — vì `canBac2` áp trực tiếp lên phần tử
lấy ra, bỏ qua bước nghịch đảo ở giữa
::why
Gần đúng ở việc bạn tính ĐÚNG `canBac2(16) = 4` — phép tính đó tự nó
không sai.

Chỗ lệch: `xuLyPhanTu` có BA bước theo thứ tự, không phải hai —
nghịch đảo LUÔN chạy TRƯỚC căn bậc hai, không bị bỏ qua. `nghichDao(16)
= 1/16 = 0.0625` (bước 2), RỒI `canBac2(0.0625) = 0.25` (bước 3) —
kết quả cuối là `0.25`, không phải `4`.
::
:::
::::

::::code{#xu_ly_phan_tu}
Tự viết `xuLyPhanTu(ds: number[], i: number): Option<number>` — ghép
`layPhanTu` → `nghichDao` → `canBac2` qua `chainOption`.

```typescript title=starter
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function chainOption<T, U>(o: Option<T>, f: (x: T) => Option<U>): Option<U> {
  switch (o.kind) {
    case "co": return f(o.giaTri);
    case "khong": return khong();
  }
}

function layPhanTu(ds: number[], i: number): Option<number> {
  const gt = ds[i];
  return gt === undefined ? khong() : co(gt);
}
function nghichDao(x: number): Option<number> {
  return x === 0 ? khong() : co(1 / x);
}
function canBac2(x: number): Option<number> {
  return x < 0 ? khong() : co(Math.sqrt(x));
}

function xuLyPhanTu(ds: number[], i: number): Option<number> {
  return chainOption(
    chainOption(___, (x) => nghichDao(x)),
    (y) => ___,
  );
}

console.log(JSON.stringify(xuLyPhanTu([4, 0, -9, 16], 0)));
```

```typescript title=solution
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function chainOption<T, U>(o: Option<T>, f: (x: T) => Option<U>): Option<U> {
  switch (o.kind) {
    case "co": return f(o.giaTri);
    case "khong": return khong();
  }
}

function layPhanTu(ds: number[], i: number): Option<number> {
  const gt = ds[i];
  return gt === undefined ? khong() : co(gt);
}
function nghichDao(x: number): Option<number> {
  return x === 0 ? khong() : co(1 / x);
}
function canBac2(x: number): Option<number> {
  return x < 0 ? khong() : co(Math.sqrt(x));
}

function xuLyPhanTu(ds: number[], i: number): Option<number> {
  return chainOption(
    chainOption(layPhanTu(ds, i), (x) => nghichDao(x)),
    (y) => canBac2(y),
  );
}

console.log(JSON.stringify(xuLyPhanTu([4, 0, -9, 16], 0)));
```

```typescript title=test
const ds = [4, 0, -9, 16];

const a = xuLyPhanTu(ds, 0);
if (a.kind !== "co") throw new Error("chỉ số hợp lệ, phần tử khác 0 và kết quả không âm phải ra co");
if (a.kind === "co" && a.giaTri !== 0.5) throw new Error("nghịch đảo của 4 là 0.25, căn bậc hai của 0.25 là 0.5");

const b = xuLyPhanTu(ds, 1);
if (b.kind !== "khong") throw new Error("phần tử là 0 khiến nghịch đảo lỗi, phải dừng ở bước 2");

const c = xuLyPhanTu(ds, 2);
if (c.kind !== "khong") throw new Error("nghịch đảo của số âm ra số âm, căn bậc hai phải lỗi ở bước 3");

const d = xuLyPhanTu(ds, 5);
if (d.kind !== "khong") throw new Error("chỉ số ngoài mảng phải lỗi ngay ở bước 1");
```

:::hints
- kind: attention
  body: "Chỗ trống ĐẦU: giá trị Option BAN ĐẦU để chain từ đó — chính là layPhanTu(ds, i). Chỗ trống SAU: bước thứ ba, áp lên giá trị y đã có từ bước nghịch đảo — chính là canBac2(y)."
- kind: strategy
  body: "chainOption(chainOption(layPhanTu(ds, i), x => nghichDao(x)), y => canBac2(y)) — đúng khuôn bài 33, đổi chainResult thành chainOption, đổi ba hàm áp dụng."
- kind: one-line
  body: "___ (chỗ 1) = layPhanTu(ds, i)\n___ (chỗ 2) = canBac2(y)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "0.5"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cụm Monad xong. `chain` làm phẳng, ghép được nhiều bước phụ thuộc
nhau, dùng được cho BẤT KỲ kiểu bọc nào có hình dạng đúng — `Option`,
`Result`, hay bất cứ thứ gì khác cùng khuôn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Có `chuyenSo: (s: string) => Option<number>`. Áp lên MỘT mảng chuỗi
(`["1","2","3"]`) bằng `.map` ra một MẢNG các `Option`. Nhưng nếu
MUỐN một `Option` DUY NHẤT bọc CẢ mảng số — có nếu TẤT CẢ chuyển
được, không có nếu dù chỉ MỘT phần tử thất bại — làm sao?
::::

::::checkpoint{mastery=0.8}
::::
