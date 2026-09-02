---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.do-tong-hop-traverse
title: "Đo tổng hợp: Traverse"
summary: "chuyenDoiLichTuan(ds): Option<number[]> — traverseOption(ds, layThuTuNgay). Tình huống MỚI (tra bảng tên ngày, không phải parse số) — không khái niệm mới, đo khả năng nhận diện 'lật Array<Container<_>> thành Container<Array<_>>' là đúng bài toán."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 40
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [alg.review-traverse]
requires: [alg.apply-parse-array]
concepts: [alg.review-traverse]
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
Bài chốt cụm — không khái niệm mới. Một tình huống MỚI: tra bảng tên
ngày thành số, không phải parse chuỗi số nữa. Bạn tự nhận ra đây là
bài toán `traverse`.
::::

::::explain{#bai-toan-tra-bang-ngay}
`chuyenDoiLichTuan(ds: string[]): Option<number[]>` — nhận một mảng
TÊN ngày trong tuần (`"Thứ Hai"`, `"Thứ Ba"`, ...), tra MỘT BẢNG để ra
thứ tự SỐ của từng ngày, ra `Option<number[]>` DUY NHẤT — có TOÀN BỘ
mảng số NẾU mọi tên hợp lệ, không có gì NẾU dù chỉ MỘT tên sai (không
có trong bảng). Đây LÀ đúng hình dạng `Array<Option<T>>` →
`Option<Array<T>>` bài 35 đặt vấn đề — chỉ khác MIỀN dữ liệu (tra
bảng tên, không phải parse chuỗi số):

```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function traverseOption<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}

const BANG_NGAY: Record<string, number> = {
  "Thứ Hai": 1, "Thứ Ba": 2, "Thứ Tư": 3, "Thứ Năm": 4,
  "Thứ Sáu": 5, "Thứ Bảy": 6, "Chủ Nhật": 7,
};

function layThuTuNgay(ten: string): Option<number> {
  const so = BANG_NGAY[ten];
  return so === undefined ? khong() : co(so);
}

function chuyenDoiLichTuan(ds: string[]): Option<number[]> {
  return traverseOption(ds, layThuTuNgay);
}

console.log(JSON.stringify(chuyenDoiLichTuan(["Thứ Hai", "Thứ Tư", "Thứ Sáu"])));
```

```text
{"kind":"co","giaTri":[1,3,5]}
```

`chuyenDoiLichTuan` KHÔNG tự viết vòng lặp — đúng bài học bài 39: chỉ
GHÉP `traverseOption` (công cụ TỔNG QUÁT, không đổi gì so với bài 36)
với `layThuTuNgay` (hàm áp dụng CỤ THỂ cho MIỀN dữ liệu này). Nhận ra
"đây LÀ bài toán lật container" là bước KHÓ — MỘT KHI nhận ra, viết
chỉ còn là ghép hai thứ đã có.
::::

::::example{#mot-ten-sai-ca-mang-khong}
Một tên KHÔNG có trong bảng — cả kết quả `khong()`, dù các tên khác
đều hợp lệ:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function traverseOption<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}
const BANG_NGAY: Record<string, number> = {
  "Thứ Hai": 1, "Thứ Ba": 2, "Thứ Tư": 3, "Thứ Năm": 4,
  "Thứ Sáu": 5, "Thứ Bảy": 6, "Chủ Nhật": 7,
};
function layThuTuNgay(ten: string): Option<number> {
  const so = BANG_NGAY[ten];
  return so === undefined ? khong() : co(so);
}
function chuyenDoiLichTuan(ds: string[]): Option<number[]> {
  return traverseOption(ds, layThuTuNgay);
}

console.log(JSON.stringify(chuyenDoiLichTuan(["Thứ Hai", "Ngày Lạ", "Thứ Sáu"])));
console.log(JSON.stringify(chuyenDoiLichTuan([])));
```

```text title=readonly
{"kind":"khong"}
{"kind":"co","giaTri":[]}
```

`"Ngày Lạ"` không có trong `BANG_NGAY` — `layThuTuNgay` trả `khong()`
cho nó, `traverseOption` dừng NGAY, cả `chuyenDoiLichTuan` ra
`khong()`, dù `"Thứ Hai"` (trước đó) và `"Thứ Sáu"` (sau đó, không
được kiểm tới) đều hợp lệ. Mảng RỖNG — đúng bài học bài 36 — ra
`co([])`, không phải `khong()`.
::::

::::predict{#doan-chuyendoilichtuan-hai-ngay commitOnce}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function traverseOption<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}
const BANG_NGAY: Record<string, number> = {
  "Thứ Hai": 1, "Thứ Ba": 2, "Thứ Tư": 3, "Thứ Năm": 4,
  "Thứ Sáu": 5, "Thứ Bảy": 6, "Chủ Nhật": 7,
};
function layThuTuNgay(ten: string): Option<number> {
  const so = BANG_NGAY[ten];
  return so === undefined ? khong() : co(so);
}
function chuyenDoiLichTuan(ds: string[]): Option<number[]> {
  return traverseOption(ds, layThuTuNgay);
}

console.log(JSON.stringify(chuyenDoiLichTuan(["Chủ Nhật", "Thứ Ba"])));
```

Dòng cuối in ra gì?

:::opt{correct}
`{"kind":"co","giaTri":[7,2]}`
:::

:::opt
`{"kind":"co","giaTri":[1,2,3,4,5,6,7]}` — vì `traverseOption` tra
TOÀN BỘ bảng `BANG_NGAY`, không chỉ hai tên trong mảng đầu vào
::why
Gần đúng ở việc bạn nhớ ĐÚNG `BANG_NGAY` có bảy mục — kích thước bảng
đó đúng.

Chỗ lệch: `traverseOption(ds, layThuTuNgay)` chỉ áp `layThuTuNgay` lên
TỪNG PHẦN TỬ của `ds` (mảng ĐẦU VÀO, ở đây chỉ có HAI tên) — nó KHÔNG
hề duyệt qua BẢNG `BANG_NGAY`. Kết quả có ĐÚNG HAI phần tử, khớp số
lượng tên trong `ds`, không phải bảy.
::
:::

:::opt
`{"kind":"co","giaTri":[2,7]}` — vì thứ tự kết quả khớp thứ tự CÁC
GIÁ TRỊ trong bảng (`BANG_NGAY`), không phải thứ tự trong mảng đầu vào
::why
Gần đúng ở việc bạn nhận ra ĐÚNG cả hai số `2` và `7` đều xuất hiện
trong kết quả — hai giá trị đó đúng.

Chỗ lệch: `traverseOption` duyệt `ds` THEO ĐÚNG THỨ TỰ nó xuất hiện
(`for (const x of ds)`, bài 36) — `ds = ["Chủ Nhật", "Thứ Ba"]`,
"Chủ Nhật" đứng TRƯỚC, tra ra `7` TRƯỚC; "Thứ Ba" đứng SAU, tra ra `2`
SAU. Kết quả giữ ĐÚNG thứ tự mảng ĐẦU VÀO: `[7, 2]`, không phải
`[2, 7]`.
::
:::
::::

::::code{#chuyen_doi_lich_tuan}
Tự viết `layThuTuNgay` và `chuyenDoiLichTuan(ds: string[]):
Option<number[]>` — dùng `traverseOption` đã có sẵn.

```typescript title=starter
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function traverseOption<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}

const BANG_NGAY: Record<string, number> = {
  "Thứ Hai": 1, "Thứ Ba": 2, "Thứ Tư": 3, "Thứ Năm": 4,
  "Thứ Sáu": 5, "Thứ Bảy": 6, "Chủ Nhật": 7,
};

function layThuTuNgay(ten: string): Option<number> {
  const so = BANG_NGAY[ten];
  return ___;
}

function chuyenDoiLichTuan(ds: string[]): Option<number[]> {
  return ___;
}

console.log(JSON.stringify(chuyenDoiLichTuan(["Thứ Hai", "Thứ Tư", "Thứ Sáu"])));
```

```typescript title=solution
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function traverseOption<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}

const BANG_NGAY: Record<string, number> = {
  "Thứ Hai": 1, "Thứ Ba": 2, "Thứ Tư": 3, "Thứ Năm": 4,
  "Thứ Sáu": 5, "Thứ Bảy": 6, "Chủ Nhật": 7,
};

function layThuTuNgay(ten: string): Option<number> {
  const so = BANG_NGAY[ten];
  return so === undefined ? khong() : co(so);
}

function chuyenDoiLichTuan(ds: string[]): Option<number[]> {
  return traverseOption(ds, layThuTuNgay);
}

console.log(JSON.stringify(chuyenDoiLichTuan(["Thứ Hai", "Thứ Tư", "Thứ Sáu"])));
```

```typescript title=test
const a = chuyenDoiLichTuan(["Thứ Hai", "Thứ Tư", "Thứ Sáu"]);
if (a.kind !== "co") throw new Error("mọi tên hợp lệ phải ra co");
if (a.kind === "co" && JSON.stringify(a.giaTri) !== JSON.stringify([1, 3, 5])) throw new Error("kq phải là [1, 3, 5]");

const b = chuyenDoiLichTuan(["Thứ Hai", "Ngày Lạ", "Thứ Sáu"]);
if (b.kind !== "khong") throw new Error("một tên không có trong bảng phải khiến cả kết quả là khong");

const c = chuyenDoiLichTuan([]);
if (c.kind !== "co") throw new Error("mảng rỗng phải ra co([])");
if (c.kind === "co" && c.giaTri.length !== 0) throw new Error("mảng rỗng phải ra kq rỗng");
```

:::hints
- kind: attention
  body: "layThuTuNgay: so đã tra được từ bảng, có thể undefined nếu tên không có — kiểm so === undefined để chọn khong() hay co(so). chuyenDoiLichTuan: chỉ GHÉP traverseOption với layThuTuNgay."
- kind: strategy
  body: "so === undefined ? khong() : co(so) — cho layThuTuNgay. traverseOption(ds, layThuTuNgay) — cho chuyenDoiLichTuan."
- kind: one-line
  body: "___ (layThuTuNgay) = so === undefined ? khong() : co(so)\n___ (chuyenDoiLichTuan) = traverseOption(ds, layThuTuNgay)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1,3,5"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cụm Traverse xong. `Array<Option<T>>` LẬT thành `Option<Array<T>>` —
một công cụ, dùng lại được ở BẤT KỲ miền dữ liệu nào cần "tất cả hoặc
không gì cả".
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

T4.3 đã dạy `Expr` đệ quy và `tinh(e: Expr): number`. Có hàm đệ quy
KHÁC trên CÙNG `Expr` — cấu trúc đệ quy đó có LẶP LẠI không?
::::

::::checkpoint{mastery=0.8}
::::
