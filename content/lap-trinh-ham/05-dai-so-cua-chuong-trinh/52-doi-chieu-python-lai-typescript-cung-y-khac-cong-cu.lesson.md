---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.doi-chieu-python-lai-typescript-cung-y-khac-cong-cu
title: "Đối chiếu: bạn đã THẤY những khuôn này ở Python (T4.1-T4.2), giờ có TÊN"
summary: "`reduce(f, xs, khoi_dau)` (T4.2, Python) và `gopTatCa(m, ds)` (track này) tính RA CÙNG MỘT SỐ trên CÙNG một mảng — không phải trùng hợp: f = ketHop, khoi_dau = rong. `compose`/`pipe` (T4.2) cũng vậy: bài 19 gọi đúng thao tác đó là 'luật Functor 2'."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 52
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [alg.recall-python-tracks]
requires: [alg.combine-traverse-catamorphism]
concepts: [alg.recall-python-tracks]
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
Bài trước ghép xong Traverse với Catamorphism — công cụ CUỐI trong
sáu khái niệm của track này. Trước khi đo tổng hợp (bài 53), một câu
hỏi ngược: những khuôn NÀY có thật sự MỚI, hay bạn đã DÙNG chúng từ
T4.1-T4.2, chỉ chưa có TÊN?
::::

::::explain{#reduce-la-goptatca}
T4.2 (Python) dạy `reduce(f, xs, khoi_dau)` — gộp một DÃY thành MỘT
giá trị. Track này (T4.5) dạy `gopTatCa(m, ds)` — CÙNG việc, dưới một
cái tên khác. Xem cả hai chạy TRÊN CÙNG một bài toán:

```typescript
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}

const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };

function tongDonHang(giaDs: number[]): number {
  return giaDs.reduce((acc, g) => acc + g, 0);
}

console.log(tongDonHang([10000, 20000, 5000]));
console.log(gopTatCa(monoidCong, [10000, 20000, 5000]));
console.log(tongDonHang([10000, 20000, 5000]) === gopTatCa(monoidCong, [10000, 20000, 5000]));
```

```text
35000
35000
true
```

`tongDonHang` viết lại đúng bài tập T4.2's `tong_don_hang` (bài 9,
Python: `reduce(lambda acc, g: acc + g, gia_ds, 0)`) bằng TypeScript —
`.reduce((acc, g) => acc + g, 0)`. `gopTatCa(monoidCong, ...)` (bài 4,
track này) gọi ĐÚNG `ds.reduce(m.ketHop, m.rong)` bên trong — với
`monoidCong.ketHop = (a, b) => a + b` và `monoidCong.rong = 0`, đó
CHÍNH LÀ `.reduce((acc, g) => acc + g, 0)`, từng tham số một.

Không phải trùng hợp: `f = ketHop`, `khoi_dau = rong`. Mọi lời gọi
`reduce(f, xs, khoi_dau)` bạn từng viết ở T4.2, NẾU `f` kết hợp được
đúng cách (bài 1-2, track này) và `khoi_dau` là giá trị trung tính
ĐÚNG của `f`, đã LÀ một lời gọi `gopTatCa` — chỉ chưa gọi tên `Monoid`
cho cặp `(f, khoi_dau)` đó. T4.2's bài 22 còn đi xa hơn: viết lại
`pipe` bằng `reduce(lambda acc, f: f(acc), ham, value)` — gộp một DÃY
HÀM, không phải dãy số — đúng bằng chứng `reduce` VỐN LÀ công cụ tổng
quát nhất để dùng một `Monoid`, không riêng gì phép cộng.
::::

::::example{#compose-pipe-la-luat-functor-hai}
Khuôn thứ hai: T4.2's `compose(f, g)`/`pipe(value, *ham)` cũng KHÔNG
phải khái niệm mới ở track này. Bài 19 (track này) đặt tên nó là
"luật Functor 2": gọi `map` nhiều lần TƯƠNG ĐƯƠNG gọi `map` một lần
với các hàm đã GHÉP lại trước. Xem lại, lần này với CẢ `compose` LẪN
`pipe`:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co": return co(f(o.giaTri));
    case "khong": return khong();
  }
}

function compose<A, B, C>(g: (b: B) => C, f: (a: A) => B): (a: A) => C {
  return (a) => g(f(a));
}

function pipe<T>(giaTri: T, ...ham: Array<(x: T) => T>): T {
  return ham.reduce((acc, f) => f(acc), giaTri);
}

const nhanDoi = (x: number) => x * 2;
const congMot = (x: number) => x + 1;

const haiLanMap = mapOption(mapOption(co(5), nhanDoi), congMot);
const motLanMapCompose = mapOption(co(5), compose(congMot, nhanDoi));
const motLanMapPipe = mapOption(co(5), (x) => pipe(x, nhanDoi, congMot));

console.log(JSON.stringify(haiLanMap));
console.log(JSON.stringify(motLanMapCompose));
console.log(JSON.stringify(motLanMapPipe));
```

```text title=readonly
{"kind":"co","giaTri":11}
{"kind":"co","giaTri":11}
{"kind":"co","giaTri":11}
```

`compose(g, f)` ở đây CHÍNH LÀ `compose` T4.2 bài 19 dạy — `(a) =>
g(f(a))`, chạy `f` TRƯỚC, `g` SAU, đọc phải-sang-trái. `pipe` cũng
CHÍNH LÀ `pipe` T4.2 bài 21-22 dạy — áp dụng lần lượt từng hàm, đọc
trái-sang-phải, viết bằng `reduce` bên trong — đúng khuôn "`reduce`
gộp một dãy HÀM" vừa thấy ở trên. Ba cách viết — gọi `mapOption` hai
lần, gọi một lần với `compose`, gọi một lần với `pipe` — ra CÙNG một
kết quả.

T4.2 KHÔNG gọi đây là "luật Functor" — nó chỉ dạy "ghép hai hàm thành
một". Track này (bài 19) đặt tên chính thức cho đúng thao tác đó, và
CHỨNG MINH nó tương thích với `map` — không phải một sự thật MỚI, mà
một sự thật ĐÃ DÙNG, giờ mới có phát biểu rõ ràng.
::::

::::predict{#doan-monoid-nhan-sai-khoi-dau commitOnce}
```typescript
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

const monoidNhan: Monoid<number> = { ketHop: (a, b) => a * b, rong: 1 };

const dungRong = [2, 3, 4].reduce(monoidNhan.ketHop, monoidNhan.rong);
const saiKhoiDau = [2, 3, 4].reduce(monoidNhan.ketHop, 0);

console.log(dungRong);
console.log(saiKhoiDau);
```

Hai dòng cuối in ra gì?

:::opt{correct}
`24` rồi `0`
:::

:::opt
`24` rồi `24` — vì `reduce` với CÙNG `ketHop` (`(a, b) => a * b`) trên
CÙNG mảng `[2, 3, 4]` luôn ra cùng một tích, bất kể `khoi_dau` truyền
vào là gì
::why
Gần đúng ở việc bạn nhận ra ĐÚNG rằng `ketHop` và mảng đầu vào của cả
hai lời gọi giống hệt nhau (đều là `(a, b) => a * b` trên `[2, 3,
4]`) — quan sát đó đúng.

Chỗ lệch: `khoi_dau` không phải chi tiết bỏ qua được (T4.2's bài 9 đã
đo y hệt điều này với phép cộng) — nó là điểm XUẤT PHÁT của phép tích
luỹ, nhân dồn CÙNG mọi phần tử. `reduce(ketHop, 0)` tính
`0 * 2 * 3 * 4 = 0`, không phải `24` — dùng đúng `ketHop` nhưng SAI
`khoi_dau` (`0` không phải `rong: 1` của `monoidNhan`) làm hỏng kết
quả. Đây đúng là lý do `rong` tồn tại trong `Monoid<T>`: không phải
trường "phòng hờ", mà là giá trị DUY NHẤT khiến `reduce` trở thành
`gopTatCa` thật.
::
:::

:::opt
Máy báo lỗi biên dịch — `reduce()` chỉ chấp nhận `khoi_dau` TRÙNG với
trường `rong` của một `Monoid<T>` đã khai báo, `0` không khớp `rong:
1` của `monoidNhan`
::why
Gần đúng ở việc bạn nghĩ tới mối LIÊN HỆ giữa `khoi_dau` và `rong` —
mối liên hệ đó có thật về mặt Ý NGHĨA (`khoi_dau` PHẢI LÀ `rong` để
kết quả đúng thuật toán "gộp bằng Monoid"), y hệt điều dòng đầu vừa
làm đúng.

Chỗ lệch: TypeScript không kiểm ràng buộc NGỮ NGHĨA đó — `Array<T>
.reduce` chấp nhận BẤT KỲ giá trị nào cùng kiểu `T` làm `khoi_dau`,
không cần khớp trường `rong` của `interface Monoid<T>` nào cả (dòng
`saiKhoiDau` gọi `reduce` trực tiếp, không hề nhắc tới
`monoidNhan.rong`). Biên dịch và chạy sạch — chỉ sai về kết quả số
học, không phải lỗi cú pháp hay kiểu dữ liệu.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không có công cụ MỚI hôm nay — chỉ có TÊN, cho những gì bạn đã viết
đúng suốt T4.1-T4.2.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sáu khái niệm — Monoid, Functor, Applicative, Monad, Traverse,
Catamorphism — hoá ra không phải sáu Ý MỚI cần nhớ tách biệt. Chúng là
TÊN cho những khuôn đã tự tay viết, đọc, gỡ lỗi suốt bốn track trước —
giờ mới có một CHỮ chung để gọi đúng, và một CÁCH KIỂM (luật identity,
luật composition...) để biết một đoạn mã có đúng khuôn đó hay không.

Bài sau đo lại toàn bộ — trước khi bước vào BOSS, một chương trình nhỏ
phải dùng ÍT NHẤT bốn trong sáu cái tên đó CÙNG LÚC. Bạn đã có đủ vật
liệu chưa?
::::

::::checkpoint{mastery=0.8}
::::
