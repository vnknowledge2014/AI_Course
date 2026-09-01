---
id: lap-trinh-ham.adt-pattern-matching.do-tong-hop-truoc-boss
title: "Đo tổng hợp trước BOSS"
summary: "Bài code có chấm điểm sống, ghép nhiều Ý đã học: một ADT đệ quy MỚI (danh sách lồng nhau, không lặp lại Expr) và một hàm xử lý đệ quy có assertNever bảo vệ — không giới thiệu khái niệm mới."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 35
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.adt-gate-review]
requires: [ts.extend-recursive-adt-exhaustive]
concepts: [ts.adt-gate-review]
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
Bài trước: thêm `tru` vào `Expr` (cây biểu thức đệ quy), rồi thấy
`assertNever` bắt lỗi ĐÚNG chỗ. Hôm nay: một hình dạng đệ quy KHÁC —
danh sách LỒNG NHAU, không phải cây biểu thức — cùng kỷ luật, không
khái niệm mới.
::::

::::explain{#adt-de-quy-danh-sach-long}
```typescript
type PhanTu =
  | { kind: "so"; giaTri: number }
  | { kind: "danhSach"; cac: PhanTu[] };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function tongPhanTu(p: PhanTu): number {
  switch (p.kind) {
    case "so":
      return p.giaTri;
    case "danhSach":
      return p.cac.reduce((tong, con) => tong + tongPhanTu(con), 0);
    default:
      return chuaXuLy(p);
  }
}

const vd: PhanTu = {
  kind: "danhSach",
  cac: [
    { kind: "so", giaTri: 1 },
    {
      kind: "danhSach",
      cac: [
        { kind: "so", giaTri: 2 },
        { kind: "so", giaTri: 3 },
      ],
    },
    { kind: "so", giaTri: 4 },
  ],
};

console.log(tongPhanTu(vd));
```

```text
10
```

`PhanTu` là một ADT đệ quy MỚI — KHÁC `Expr` (bài 32–34) ở CHỖ tự tham
chiếu: `Expr`'s `cong`/`nhan` có hai field đơn `trai`/`phai` kiểu
`Expr`, còn `PhanTu`'s biến thể `danhSach` có MỘT field `cac` kiểu
`PhanTu[]` — một MẢNG các `PhanTu` con, lồng sâu bao nhiêu tầng cũng
được. Nguyên lý tự tham chiếu là CÙNG một nguyên lý (bài 32 đã dạy),
chỉ khác HÌNH DẠNG field mang nó.

`tongPhanTu` theo ĐÚNG khuôn quen thuộc: `switch` trên `kind`, biến
thể `so` trả thẳng `giaTri`, biến thể `danhSach` GỌI LẠI chính
`tongPhanTu` (đệ quy) trên TỪNG phần tử con rồi cộng dồn bằng
`reduce`, và `default` gọi `chuaXuLy` — kiểu học ở bài 8 vẫn đứng gác
y nguyên. Không có khái niệm mới ở đây: đây là bài kiểm bạn có tự dựng
lại được TOÀN BỘ khuôn mẫu đó trên một hình dạng CHƯA từng thấy hay
không.
::::

::::example{#dem-la-cung-mot-khuon}
Một hàm KHÁC hoàn toàn về nghiệp vụ (đếm số lá `so`, không phải tính
tổng) trên CÙNG type `PhanTu` — nhưng CẤU TRÚC hàm không đổi một chút
nào:

```typescript title=readonly
type PhanTu =
  | { kind: "so"; giaTri: number }
  | { kind: "danhSach"; cac: PhanTu[] };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function demSoLa(p: PhanTu): number {
  switch (p.kind) {
    case "so":
      return 1;
    case "danhSach":
      return p.cac.reduce((tong, con) => tong + demSoLa(con), 0);
    default:
      return chuaXuLy(p);
  }
}

const vd2: PhanTu = {
  kind: "danhSach",
  cac: [
    { kind: "so", giaTri: 10 },
    {
      kind: "danhSach",
      cac: [
        { kind: "so", giaTri: 20 },
        { kind: "danhSach", cac: [{ kind: "so", giaTri: 30 }] },
      ],
    },
  ],
};

console.log(demSoLa(vd2));
```

```text title=readonly
3
```

`demSoLa` đếm ba giá trị `so` (`10`, `20`, `30`) dù chúng nằm ở BA tầng
lồng khác nhau. So với `tongPhanTu` ở trên: case `so` đổi từ
`return p.giaTri` thành `return 1`, case `danhSach` vẫn `reduce` gọi
lại chính hàm — CHỈ nghiệp vụ đổi, KHUÔN đệ quy (`switch` + gọi lại
chính hàm + `chuaXuLy` ở `default`) giữ nguyên. Đây LÀ điều đáng nhớ
nhất của cả track: một khi đã có khuôn, viết hàm đệ quy MỚI trên cùng
ADT chỉ còn là điền đúng nghiệp vụ vào đúng chỗ.
::::

::::predict{#doan-thieu-case-rong commitOnce}
```typescript
type PhanTu =
  | { kind: "so"; giaTri: number }
  | { kind: "danhSach"; cac: PhanTu[] }
  | { kind: "rong" };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function tongPhanTu(p: PhanTu): number {
  switch (p.kind) {
    case "so":
      return p.giaTri;
    case "danhSach":
      return p.cac.reduce((tong, con) => tong + tongPhanTu(con), 0);
    default:
      return chuaXuLy(p);
  }
}

console.log(tongPhanTu({ kind: "so", giaTri: 5 }));
```

Dòng cuối in ra gì?

:::opt{correct}
Máy báo lỗi biên dịch tại dòng `return chuaXuLy(p);` — sau hai `case`
đã xử lý, `p` chỉ còn kiểu `{ kind: "rong" }`, không khớp tham số
`never` mà `chuaXuLy` đòi
:::

:::opt
Biên dịch bình thường, in ra `5` — vì tham số truyền vào là `{ kind:
"so", giaTri: 5 }`, biến thể `rong` không hề được dùng tới trong lời
gọi này
::why
Gần đúng ở việc bạn nhận ra `rong` THẬT SỰ chưa được xử lý trong
`switch` — quan sát đó đúng: chỉ có `case "so"` và `case "danhSach"`,
không có `case "rong"`.

Chỗ lệch: lỗi không đợi tới LÚC CHẠY, và không phụ thuộc giá trị cụ
thể được truyền vào. Vì `switch` thiếu `case "rong"`, tại nhánh
`default`, kiểu của `p` KHÔNG được thu hẹp còn `never` — nó vẫn là
`{ kind: "rong" }`. Gọi `chuaXuLy(p)` với kiểu đó VI PHẠM chữ ký
`(x: never) => never` ngay lúc BIÊN DỊCH (mã TS2345), toàn bộ chương
trình không chạy được một dòng nào, kể cả dòng `console.log` cuối.
::
:::

:::opt
Biên dịch bình thường vì `switch` đã có nhánh `default` — có `default`
nghĩa là mọi giá trị còn lại đều được xử lý, không còn trường hợp nào
lọt qua
::why
Gần đúng ở việc bạn để ý `switch` CÓ nhánh `default` — quan sát đó
đúng, cú pháp hợp lệ, và `default` đúng là nhánh CHẠY khi không case
nào khớp.

Chỗ lệch: có `default` không tự động làm nhánh đó AN TOÀN VỀ KIỂU.
TypeScript vẫn kiểm kiểu THẬT của `p` tại `default` dựa trên những
`case` đã xử lý TRƯỚC đó — vì `rong` chưa được xử lý ở đâu cả, tại
`default` biến `p` vẫn có thể là `{ kind: "rong" }`, không thu hẹp về
`never`. `chuaXuLy` chỉ nhận `never`, nên gọi nó với `p` ở đây bị
TypeScript CHẶN NGAY lúc biên dịch, không đợi chạy.
::
:::
::::

::::code{#tim-so-lon-nhat}
Cho sẵn `PhanTu` (danh sách lồng nhau, cùng type đã dùng ở trên) và
hàm `chuaXuLy`. Viết `timSoLonNhat` — hàm ĐỆ QUY tìm giá trị `so` LỚN
NHẤT trong toàn bộ cấu trúc, dù lồng bao nhiêu tầng.

```typescript title=starter
type PhanTu =
  | { kind: "so"; giaTri: number }
  | { kind: "danhSach"; cac: PhanTu[] };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function timSoLonNhat(p: PhanTu): number {
  switch (p.kind) {
    case "so":
      return p.giaTri;
    case "danhSach":
      return p.cac.reduce((lon, con) => Math.max(lon, ___(con)), -Infinity);
    default:
      return chuaXuLy(p);
  }
}

console.log(timSoLonNhat({ kind: "so", giaTri: 42 }));
```

```typescript title=solution
type PhanTu =
  | { kind: "so"; giaTri: number }
  | { kind: "danhSach"; cac: PhanTu[] };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function timSoLonNhat(p: PhanTu): number {
  switch (p.kind) {
    case "so":
      return p.giaTri;
    case "danhSach":
      return p.cac.reduce((lon, con) => Math.max(lon, timSoLonNhat(con)), -Infinity);
    default:
      return chuaXuLy(p);
  }
}

console.log(timSoLonNhat({ kind: "so", giaTri: 42 }));
```

```typescript title=test
const t1: PhanTu = { kind: "so", giaTri: 5 };
if (timSoLonNhat(t1) !== 5) throw new Error("timSoLonNhat({kind:\"so\",giaTri:5}) phải trả 5 — đang là " + timSoLonNhat(t1));

const t2: PhanTu = {
  kind: "danhSach",
  cac: [
    { kind: "so", giaTri: 3 },
    { kind: "so", giaTri: 9 },
    { kind: "so", giaTri: 1 },
  ],
};
if (timSoLonNhat(t2) !== 9) throw new Error("timSoLonNhat(t2) phải trả 9 — đang là " + timSoLonNhat(t2));

const t3: PhanTu = {
  kind: "danhSach",
  cac: [
    { kind: "so", giaTri: 2 },
    {
      kind: "danhSach",
      cac: [
        { kind: "so", giaTri: 8 },
        { kind: "so", giaTri: 4 },
      ],
    },
    { kind: "so", giaTri: 1 },
  ],
};
if (timSoLonNhat(t3) !== 8) throw new Error("timSoLonNhat(t3) phải trả 8 — đang là " + timSoLonNhat(t3));
```

:::hints
- kind: attention
  body: "Chỗ trống là TÊN HÀM được gọi đệ quy bên trong nhánh danhSach — không phải một giá trị số, chỉ là gọi lại chính timSoLonNhat trên từng phần tử con."
- kind: strategy
  body: "Mỗi phần tử con trong p.cac có thể lại là một danhSach lồng sâu hơn — muốn biết giá trị lớn nhất bên trong NÓ, phải gọi lại đúng hàm đang định nghĩa (đệ quy), rồi lấy Math.max giữa các kết quả đó."
- kind: one-line
  body: "Chỗ trống là: timSoLonNhat"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "42"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một hình dạng đệ quy MỚI, y hệt kỷ luật cũ: `switch` trên `kind`, gọi
lại chính hàm ở nhánh đệ quy, `chuaXuLy` đứng gác ở `default`. Không
còn gì trong track này là bí ẩn — bước cuối: ghép TẤT CẢ lại.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Track này còn đúng MỘT bài — bài BOSS. Nó không dạy gì mới: chỉ hỏi
bạn viết một chương trình dùng ĐỦ mọi Ý đã học trong 35 bài vừa qua —
discriminant field, `switch` + `assertNever` exhaustiveness,
sum-of-products lồng nhau, thay cờ boolean rời rạc bằng union để
illegal states unrepresentable, branded type + smart constructor, và
một ADT đệ quy như vừa thấy hôm nay.

Sẵn sàng ghép tất cả chưa?
::::

::::checkpoint{mastery=0.8}
::::
