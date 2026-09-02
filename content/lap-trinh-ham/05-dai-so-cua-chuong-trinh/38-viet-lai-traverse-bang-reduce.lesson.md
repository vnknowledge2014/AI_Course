---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.viet-lai-traverse-bang-reduce
title: "Viết lại `traverse` bằng `reduce` — nối thẳng cụm 1's `gopTatCa`"
summary: "traverseReduce<T,U>(ds, f): Option<U[]> — ds.reduce((acc, x) => chainOption(acc, mang => chainOption(f(x), gt => co([...mang, gt]))), co([])). reduce là vòng lặp đóng gói (T4.2); chainOption bên trong mỗi bước gộp cho CÙNG short-circuit."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 38
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [alg.traverse-via-reduce]
requires: [alg.predict-traverse-shortcircuit]
concepts: [alg.traverse-via-reduce]
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
T4.2 phát hiện: `reduce` là vòng lặp `for` ĐÓNG GÓI (bài 10). `traverse`
viết bằng `for` (bài 36) — viết lại bằng `reduce` được không?
::::

::::explain{#traverse-bang-reduce}
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

function traverseReduce<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  return ds.reduce(
    (acc: Option<U[]>, x) =>
      chainOption(acc, (mang) =>
        chainOption(f(x), (gt) => co([...mang, gt])),
      ),
    co<U[]>([]),
  );
}

function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

console.log(JSON.stringify(traverseReduce(["1", "2", "3"], chuyenSo)));
```

```text
{"kind":"co","giaTri":[1,2,3]}
```

Giá trị KHỞI ĐẦU của `reduce` là `co<U[]>([])` (nối thẳng cụm 1's
`gopTatCa` — MỘT giá trị TRUNG TÍNH để bắt đầu gộp, `co([])` đóng vai
"đã có mảng RỖNG"). Ở MỖI bước, gộp `acc` (kết quả TÍCH LŨY tới nay)
với `x` (phần tử HIỆN TẠI) qua HAI lớp `chainOption` LỒNG: lớp NGOÀI
mở `acc` ra thành `mang` (mảng ĐÃ gom được); lớp TRONG mở `f(x)` ra
thành `gt` (giá trị phần tử NÀY, nếu hợp lệ), rồi TẠO mảng mới
`[...mang, gt]`, bọc lại bằng `co(...)`.
::::

::::example{#reduce-van-short-circuit}
`reduce` (Array.reduce) LUÔN gọi hàm gộp trên MỌI phần tử của mảng —
KHÔNG tự dừng sớm như vòng lặp `for` có `return` (bài 36). Nhưng
`chainOption` BÊN TRONG mỗi bước VẪN cho short-circuit — chỉ khác
CHỖ nó xảy ra:

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
function traverseReduce<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  return ds.reduce(
    (acc: Option<U[]>, x) =>
      chainOption(acc, (mang) =>
        chainOption(f(x), (gt) => co([...mang, gt])),
      ),
    co<U[]>([]),
  );
}
function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

let soLanGoi = 0;
const chuyenSoCoDem = (vb: string): Option<number> => {
  soLanGoi = soLanGoi + 1;
  return chuyenSo(vb);
};

traverseReduce(["1", "abc", "3"], chuyenSoCoDem);
console.log(soLanGoi);
```

```text title=readonly
2
```

`ds.reduce(...)` VẪN duyệt qua CẢ BA chỉ số của mảng (`0`, `1`, `2`)
— bản thân `reduce` không hề dừng sớm. Nhưng ở bước THỨ BA (`x =
"3"`), `acc` LÚC ĐÓ đã là `khong()` (từ bước lỗi `"abc"` trước đó).
`chainOption(acc, (mang) => ...)` thấy `acc.kind === "khong"`, trả về
`khong()` NGAY — hàm callback `(mang) => chainOption(f(x), ...)`
KHÔNG BAO GIỜ được GỌI, nên `f("3")` (bên TRONG callback đó) không hề
chạy. `soLanGoi` chỉ tăng lên `2` (cho `"1"` và `"abc"`), dù `reduce`
"đi qua" cả ba phần tử — short-circuit xảy ra Ở TỪNG BƯỚC gộp, không
phải ở tầng `reduce`.
::::

::::predict{#doan-traverse-reduce-loi-dau commitOnce}
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
function traverseReduce<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  return ds.reduce(
    (acc: Option<U[]>, x) =>
      chainOption(acc, (mang) =>
        chainOption(f(x), (gt) => co([...mang, gt])),
      ),
    co<U[]>([]),
  );
}
function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

let soLanGoi = 0;
const chuyenSoCoDem = (vb: string): Option<number> => {
  soLanGoi = soLanGoi + 1;
  return chuyenSo(vb);
};

traverseReduce(["abc", "1", "2"], chuyenSoCoDem);
console.log(soLanGoi);
```

Dòng cuối in ra gì (phần tử LỖI nằm Ở ĐẦU mảng lần này, không phải
giữa)?

:::opt{correct}
`1`
:::

:::opt
`3` — vì `reduce` luôn duyệt qua HẾT mọi phần tử của mảng, nên
`chuyenSoCoDem` phải được gọi đủ ba lần bất kể phần tử nào lỗi
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng `reduce` (bản thân nó) duyệt qua HẾT
mọi CHỈ SỐ của mảng, không tự dừng sớm — quan sát đó đúng (bài học
chính của bài này).

Chỗ lệch: `reduce` duyệt hết CHỈ SỐ không có nghĩa là `f(x)` (bên
TRONG mỗi bước gộp) LUÔN được GỌI. Ngay bước ĐẦU TIÊN, `f("abc")`
chạy VÀ lỗi — `acc` sau bước đó là `khong()`. Ở hai bước SAU (`"1"`,
`"2"`), `chainOption(acc, ...)` thấy `acc.kind === "khong"` NGAY, trả
`khong()` mà KHÔNG hề gọi callback bên trong (nơi `f(x)` được gọi) —
`f` chỉ chạy ĐÚNG MỘT LẦN, cho phần tử lỗi đầu tiên.
::
:::

:::opt
Máy báo lỗi lúc chạy — phần tử lỗi Ở VỊ TRÍ ĐẦU khiến `acc` là
`khong()` ngay từ bước đầu, và các bước sau không xử lý được giá trị
`khong()` đó
::why
Gần đúng ở việc bạn nhận ra ĐÚNG `acc` trở thành `khong()` NGAY từ
bước đầu tiên — quan sát đó đúng.

Chỗ lệch: `chainOption` được THIẾT KẾ CHÍNH XÁC để xử lý `acc` là
`khong()` — nhánh `"khong"` của nó trả về `khong()` NGAY, không hề
gây lỗi hay ngoại lệ nào. Đây LÀ hành vi ĐÚNG, không phải một trường
hợp chưa xử lý.
::
:::
::::

::::code{#traverse_reduce}
Tự viết `traverseReduce<T, U>(ds: T[], f: (x: T) => Option<U>):
Option<U[]>` bằng `reduce` và `chainOption`.

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

function traverseReduce<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  return ds.reduce(
    (acc: Option<U[]>, x) =>
      chainOption(acc, (mang) =>
        chainOption(f(x), (gt) => ___),
      ),
    ___,
  );
}

function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

console.log(JSON.stringify(traverseReduce(["1", "2", "3"], chuyenSo)));
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

function traverseReduce<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  return ds.reduce(
    (acc: Option<U[]>, x) =>
      chainOption(acc, (mang) =>
        chainOption(f(x), (gt) => co([...mang, gt])),
      ),
    co<U[]>([]),
  );
}

function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

console.log(JSON.stringify(traverseReduce(["1", "2", "3"], chuyenSo)));
```

```typescript title=test
const a = traverseReduce(["1", "2", "3"], chuyenSo);
if (a.kind !== "co") throw new Error("mọi phần tử chuyển được phải ra co");
if (a.kind === "co" && JSON.stringify(a.giaTri) !== JSON.stringify([1, 2, 3])) throw new Error("kq phải là [1, 2, 3]");

const b = traverseReduce(["1", "abc", "3"], chuyenSo);
if (b.kind !== "khong") throw new Error("một phần tử lỗi phải khiến cả kết quả là khong");

let soLanGoi = 0;
const chuyenSoCoDem = (vb: string): Option<number> => {
  soLanGoi = soLanGoi + 1;
  return chuyenSo(vb);
};
traverseReduce(["1", "abc", "3"], chuyenSoCoDem);
if (soLanGoi !== 2) throw new Error("f không được gọi cho phần tử SAU phần tử lỗi, dù reduce vẫn duyệt hết chỉ số");

const c = traverseReduce<string, number>([], chuyenSo);
if (c.kind !== "co") throw new Error("mảng rỗng phải ra co([]), không phải khong");
if (c.kind === "co" && c.giaTri.length !== 0) throw new Error("mảng rỗng phải ra kq rỗng");
```

:::hints
- kind: attention
  body: "Chỗ trống ĐẦU (bên trong hai lớp chainOption): đã có mang (mảng cũ) và gt (giá trị mới) — tạo mảng MỚI [...mang, gt], bọc lại co(...). Chỗ trống SAU (giá trị khởi đầu của reduce): co<U[]>([]) — mảng rỗng, đã CÓ (nối thẳng gopTatCa's trung tính)."
- kind: strategy
  body: "chainOption(acc, mang => chainOption(f(x), gt => co([...mang, gt]))), co<U[]>([]) — hai lớp chainOption lồng nhau mỗi bước gộp, khởi đầu là co([])."
- kind: one-line
  body: "___ (chỗ 1) = co([...mang, gt])\n___ (chỗ 2) = co<U[]>([])"
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
`traverse` viết bằng `reduce` — CÙNG kết quả, CÙNG short-circuit,
khác chỗ short-circuit xảy ra (ở TỪNG bước gộp qua `chainOption`,
không phải ở bản thân vòng lặp).
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn có `traverse` — dùng nó cho một bài toán THẬT: parse một mảng
chuỗi thành mảng số, báo lỗi RÕ nếu một phần tử sai.
::::

::::checkpoint{mastery=0.8}
::::
