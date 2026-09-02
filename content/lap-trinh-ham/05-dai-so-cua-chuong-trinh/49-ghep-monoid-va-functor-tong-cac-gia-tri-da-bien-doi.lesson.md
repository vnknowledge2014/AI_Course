---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.ghep-monoid-va-functor-tong-cac-gia-tri-da-bien-doi
title: "Ghép Monoid + Functor: tổng các giá trị ĐÃ biến đổi"
summary: "tongSauKhiNhanDoi(ds: number[]): number — ds.map(x => x*2) (Functor, biến đổi TỪNG phần tử) RỒI gopTatCa(monoidCong, ...) (Monoid, cộng dồn) — hai khái niệm học RIÊNG, ghép lại thành MỘT hàm, một chuỗi hai bước."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 49
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [alg.combine-monoid-functor]
requires: [alg.six-concepts-one-family]
concepts: [alg.combine-monoid-functor]
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
Bài trước xếp SÁU khái niệm — Monoid, Functor, Applicative, Monad,
Traverse, Catamorphism — cạnh nhau thành MỘT họ hình dạng. Nhưng một
chương trình THẬT hiếm khi chỉ dùng ĐÚNG MỘT khái niệm. Bài này ghép
hai cái ĐẦU tiên lại: Monoid VÀ Functor, trong MỘT hàm.
::::

::::explain{#map-roi-gop}
```typescript
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}
function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}
const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };

function tongBinhPhuong(ds: number[]): number {
  const daBinhPhuong = ds.map((x) => x * x);
  return gopTatCa(monoidCong, daBinhPhuong);
}

console.log(tongBinhPhuong([1, 2, 3]));
console.log(tongBinhPhuong([]));
```

```text
14
0
```

Hai dòng, HAI khái niệm khác nhau, chạy NỐI TIẾP. `ds.map((x) => x * x)`
là Functor — biến đổi TỪNG phần tử BÊN TRONG mảng, giữ nguyên "vỏ" (vẫn
là một mảng, cùng số lượng phần tử). `gopTatCa(monoidCong, daBinhPhuong)`
là Monoid — gộp CẢ mảng đã biến đổi đó thành MỘT giá trị duy nhất, dùng
`ketHop` (cộng) và `rong` (`0`, dùng khi mảng rỗng).

`tongBinhPhuong([1, 2, 3])`: map chạy trước, ra `[1, 4, 9]` (bình
phương của `1`, `2`, `3` theo thứ tự), rồi `gopTatCa` cộng dồn:
`0 + 1 + 4 + 9 = 14`. `tongBinhPhuong([])`: map trên mảng rỗng ra mảng
rỗng, `gopTatCa` trên mảng rỗng trả thẳng `rong = 0` — đúng khuôn "mảng
rỗng vẫn ra đúng giá trị trung tính" đã đo ở bài 5, không cần xử lý
riêng cho trường hợp này.

Không có khái niệm MỚI ở đây — `map` (Functor, cụm 3) và `gopTatCa`
(Monoid, cụm 1) mỗi cái đã học RIÊNG. Cái mới là NỐI chúng lại: kết quả
của bước map (một mảng ĐÃ biến đổi) trở thành ĐẦU VÀO của bước gộp.
::::

::::example{#doi-buoc-bien-doi}
Khuôn "map RỒI gopTatCa" không chỉ dùng cho phép bình phương — ĐỔI
bước map sang một phép biến đổi HOÀN TOÀN khác, khuôn vẫn giữ nguyên:

```typescript title=readonly
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}
function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}
const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };

const danhSachTen = ["An", "Binh", "Chi"];
const tongDoDai = gopTatCa(monoidCong, danhSachTen.map((ten) => ten.length));
console.log(tongDoDai);
```

```text title=readonly
9
```

Lần này bước map không tính bình phương — nó biến đổi `string` thành
`number` (`ten.length`, độ dài chuỗi). Bước gộp VẪN dùng ĐÚNG
`monoidCong` như trước, không đổi gì. Hai bước tách bạch: map lo việc
"biến MỖI phần tử thành cái gì", `gopTatCa` lo việc "gộp CẢ dãy kết quả
đó lại thế nào" — đổi bước nào cũng không đụng tới bước kia.
::::

::::predict{#doan-map-roi-gop commitOnce}
```typescript
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}
function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}
const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };

console.log(gopTatCa(monoidCong, [4, 5, 6].map((x) => x * 2)));
console.log(gopTatCa(monoidCong, ([] as number[]).map((x) => x * 2)));
```

Hai dòng in ra gì?

:::opt{correct}
`30` rồi `0`
:::

:::opt
`960` rồi `0` — vì bước map đã "nhân đôi" nên phép gộp CŨNG phải là
phép NHÂN, không phải cộng
::why
Gần đúng ở việc bạn nhận ra bước map (`x => x * 2`) THẬT SỰ có phép
NHÂN — quan sát đó đúng, mỗi phần tử được nhân đôi trước khi gộp.

Chỗ lệch: phép nhân đó chỉ nằm TRONG hàm map (biến đổi TỪNG phần tử,
`[4,5,6]` thành `[8,10,12]`), không lan sang bước gộp. `monoidCong.ketHop
= (a, b) => a + b` LUÔN LÀ phép CỘNG (định nghĩa từ bài 3, không đổi ở
đây) — `gopTatCa` dùng ĐÚNG `ketHop` đó để gộp mảng đã nhân đôi, ra
`8 + 10 + 12 = 30`, không phải tích `8 × 10 × 12 = 960`.
::
:::

:::opt
`30` rồi máy báo lỗi — vì `([] as number[]).map(...)` gọi map trên một
mảng rỗng đã ép kiểu, TypeScript không cho phép map chạy khi không có
phần tử nào để suy ra kiểu trả về
::why
Gần đúng ở việc bạn tính đúng dòng ĐẦU (`30`).

Chỗ lệch: `[] as number[]` đã ÉP KIỂU rõ ràng — TypeScript biết đây là
`number[]` rỗng, không cần suy luận gì thêm. `.map` trên mảng rỗng chạy
bình thường, KHÔNG gọi hàm truyền vào lần nào, trả về một mảng rỗng
`number[]` khác. `gopTatCa(monoidCong, [])` sau đó dùng ĐÚNG `rong = 0`
làm kết quả — đây CHÍNH LÀ điều bài 5 đã đo: mảng rỗng vẫn ra đúng giá
trị trung tính, không có lỗi nào cả.
::
:::
::::

::::code{#tong_sau_khi_nhan_doi}
Viết `tongSauKhiNhanDoi(ds: number[]): number` — ghép ĐÚNG hai bước
vừa thấy: `.map()` nhân đôi TỪNG phần tử (Functor), rồi
`gopTatCa(monoidCong, ...)` cộng dồn kết quả đó lại (Monoid).

```typescript title=starter
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}
function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}
const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };

function tongSauKhiNhanDoi(ds: number[]): number {
  return ___;
}

console.log(tongSauKhiNhanDoi([1, 2, 3]));
```

```typescript title=solution
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}
function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}
const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };

function tongSauKhiNhanDoi(ds: number[]): number {
  return gopTatCa(monoidCong, ds.map((x) => x * 2));
}

console.log(tongSauKhiNhanDoi([1, 2, 3]));
```

```typescript title=test
if (tongSauKhiNhanDoi([1, 2, 3]) !== 12) throw new Error("tongSauKhiNhanDoi([1,2,3]) phải nhân đôi TỪNG phần tử ([2,4,6]) rồi cộng dồn, ra 12");
if (tongSauKhiNhanDoi([5]) !== 10) throw new Error("tongSauKhiNhanDoi([5]) phải ra 10 — 5 nhân đôi là 10, gộp một phần tử vẫn đúng");
if (tongSauKhiNhanDoi([]) !== 0) throw new Error("tongSauKhiNhanDoi([]) phải ra 0 — mảng rỗng dùng đúng giá trị trung tính rong=0 của monoidCong");
if (tongSauKhiNhanDoi([-3, 4]) !== 2) throw new Error("tongSauKhiNhanDoi([-3,4]) phải nhân đôi TỪNG phần tử ([-6,8]) rồi cộng dồn, ra 2");
```

:::hints
- kind: attention
  body: "Hai bước, ĐÚNG thứ tự: map (nhân đôi TỪNG phần tử) TRƯỚC, gopTatCa (cộng dồn cả mảng) SAU. Đừng gộp trước rồi nhân một số duy nhất — phải nhân đôi TỪNG phần tử, không phải nhân đôi tổng."
- kind: strategy
  body: "ds.map((x) => x * 2) cho ra mảng ĐÃ nhân đôi. Đưa thẳng mảng đó làm đối số thứ hai của gopTatCa(monoidCong, ...) — không cần biến trung gian, dù viết ra biến trung gian cũng được."
- kind: one-line
  body: "gopTatCa(monoidCong, ds.map((x) => x * 2))"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "12"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai khái niệm học RIÊNG — Functor và Monoid — vừa chạy NỐI TIẾP trong
MỘT hàm. Đây là điều "một họ hình dạng" (bài trước) thật sự có ích:
ghép được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tongSauKhiNhanDoi` ghép HAI bước ĐỘC LẬP với nhau — map không cần biết
gopTatCa sẽ làm gì, gopTatCa không cần biết dữ liệu đến từ đâu. Nhưng
không phải chương trình nào cũng ghép được kiểu "độc lập" như vậy. Nếu
một form vừa cần validate NHIỀU trường ĐỘC LẬP (Applicative), vừa cần
tính tiếp một bước PHỤ THUỘC vào kết quả CẢ HAI trường đó (Monad) —
ghép hai kiểu này có khác gì ghép map với gopTatCa không?

Bài sau ghép ĐÚNG hai kiểu đó lại trong một form.
::::

::::checkpoint{mastery=0.8}
::::
