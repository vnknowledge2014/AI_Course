---
id: lap-trinh-ham.cong-typescript.dung-union-phai-kiem-tra-kieu-nao-truoc
title: "Dùng union type phải KIỂM TRA kiểu nào trước khi dùng phương thức riêng"
summary: "`function f(x: string | number) { return x.toUpperCase(); }` bị TSC từ chối (TS2339) — `.toUpperCase()` chỉ tồn tại trên `string`, không tồn tại trên `number`. Phải kiểm `typeof x === 'string'` trước — đúng kiểu thu hẹp bài 12 đã học, áp lên union."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 16
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.union-narrowing]
requires: [ts.union-type]
concepts: [ts.union-narrowing]
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
Bạn vừa gán được cả `string` lẫn `number` vào một biến union. Giờ thử
GỌI một phương thức thay vì gán — xem TypeScript nói gì.
::::

::::explain{#khong-duoc-tu-do-goi}
`string` có `.toUpperCase()`. `number` thì không — nó có `.toFixed()`
thay vào đó. Hai kiểu khác nhau, hai tập phương thức khác nhau hoàn
toàn.

Vậy một cái tên khai `string | number` được GỌI những phương thức nào?
Câu trả lời của TypeScript rất nghiêm: chỉ những phương thức tồn tại
trên **CẢ HAI** kiểu trong union mới được gọi tự do. `.toUpperCase()`
chỉ tồn tại trên `string`, không tồn tại trên `number` — nên gọi nó
trên một giá trị `string | number` bị từ chối, dù lúc chạy giá trị đó
CÓ THỂ đang thực sự là một chuỗi.

Lý do TypeScript nghiêm khắc vậy: nó không biết trước lúc CHẠY giá trị
sẽ là kiểu nào — nó chỉ có thể đảm bảo an toàn cho những gì ĐÚNG VỚI CẢ
HAI khả năng. Muốn gọi phương thức riêng của một kiểu, phải THU HẸP
(narrow) trước — bài 12 đã dạy đúng kỹ thuật này cho `undefined`, giờ
áp dụng lại cho union: dùng `typeof x === "string"` để báo cho
TypeScript biết, TRONG NHÁNH NÀY, `x` chắc chắn là `string`.
::::

::::example{#toan-goi-truoc-sau-khi-kiem}
Gọi `.toUpperCase()` trực tiếp trên tham số `string | number`, không
kiểm gì trước:

```typescript title=readonly
function moTa(x: string | number): string {
  return x.toUpperCase();
}
```

```text title=readonly
(không biên dịch được)

TS2339 (dòng 2, cột 12): Property 'toUpperCase' does not exist on type 'string | number'.   Property 'toUpperCase' does not exist on type 'number'.
```

TypeScript từ chối ngay — thông điệp nói rõ: `toUpperCase` không tồn
tại trên `number`, vế còn lại của union. Thêm `typeof x === "string"`
để THU HẸP trước khi gọi:

```typescript title=readonly
function moTa(x: string | number): string {
  if (typeof x === "string") {
    return x.toUpperCase();
  }
  return x.toString();
}

console.log(moTa("cam"));
console.log(moTa(7));
```

```text title=readonly
CAM
7
```

Trong nhánh `if (typeof x === "string")`, TypeScript tự biết `x` CHẮC
CHẮN là `string` — `.toUpperCase()` được gọi tự do, không lỗi nào cả.
Ở nhánh còn lại (sau `if`, không cần `else`), TypeScript tự suy ra phần
còn lại của union đã bị loại — `x` chỉ còn có thể là `number`, nên
`.toString()` (tồn tại trên cả hai kiểu) gọi được, và `.toFixed()` cũng
sẽ gọi được ở đúng nhánh này nếu cần.
::::

::::predict{#doan-dong-nao-bi-tu-choi commitOnce}
Byte viết một hàm, KHÔNG chạy thử:

```typescript
function xuLy(x: string | number): void {
  console.log(x.toString());
  console.log(x.toFixed(2));
  if (typeof x === "number") {
    console.log(x.toFixed(2));
  }
}
```

**Trước khi đọc đáp án**, dòng nào bị TypeScript từ chối, và vì sao?

:::opt{correct}
Dòng 3 (`x.toFixed(2)` NGOÀI nhánh `if`) — `.toFixed()` chỉ tồn tại
trên `number`, không tồn tại trên `string`, còn tại dòng 3, TypeScript
chưa thu hẹp `x` — nó vẫn là `string | number`. Dòng 2 (`.toString()`)
và dòng 5 (`.toFixed()` BÊN TRONG nhánh `if`, sau khi đã thu hẹp) đều
hợp lệ
:::

:::opt
Dòng 2 (`x.toString()`) — phương thức `.toString()` không tồn tại trên
kiểu union `string | number`, chỉ tồn tại trên từng kiểu riêng lẻ
::why
Gần đúng ở việc bạn nghĩ union type giới hạn phương thức gọi được —
đúng, đó chính là bài học của bài này.

Chỗ lệch: `.toString()` là một trong số ít phương thức tồn tại trên
CẢ HAI kiểu `string` VÀ `number` — mọi giá trị JavaScript đều có
`.toString()`. Gọi nó trên `string | number` không cần thu hẹp gì cả,
đã thử thật, dòng 2 không bị từ chối.
::
:::

:::opt
Dòng 5 (`x.toFixed(2)` BÊN TRONG nhánh `if`) — thu hẹp bằng `typeof`
chỉ có tác dụng trong bài kiểm tra `undefined` (bài 12), không áp dụng
được cho union type nhiều kiểu dữ liệu khác nhau như `string | number`
::why
Gần đúng ở việc bạn nhớ đúng nơi kỹ thuật `typeof` từng xuất hiện lần
đầu — bài 12, kiểm tra `undefined`.

Chỗ lệch: thu hẹp bằng `typeof` không chỉ dành riêng cho
`undefined`/`null` — nó hoạt động cho MỌI union type, kể cả
`string | number`. Trong nhánh `if (typeof x === "number")`, TypeScript
biết chắc `x` là `number`, nên `.toFixed()` gọi được, đã thử thật,
dòng 5 không bị từ chối.
::
:::

:::opt
Không dòng nào bị từ chối — tham số `x` đã khai `string | number`
tường minh ngay từ chữ ký hàm, nên mọi phương thức của cả hai kiểu đều
gọi được ở bất cứ đâu trong thân hàm
::why
Gần đúng ở việc bạn nhớ đúng: `x` ĐÃ khai kiểu tường minh ngay từ chữ
ký hàm (bài 6) — điều đó có thật.

Chỗ lệch: khai kiểu tường minh KHÔNG đồng nghĩa với việc mọi phương
thức của MỌI kiểu trong union đều gọi tự do — chỉ những phương thức
CHUNG cho cả hai kiểu mới được, trừ khi đã thu hẹp. Dòng 3 gọi
`.toFixed()` (chỉ có ở `number`) mà chưa thu hẹp — bị từ chối thật,
đúng mã lỗi `TS2339`.
::
:::
::::

::::code{#do-dai-hoac-gia-tri}
Một hàm nhận `x: string | number`: nếu là chuỗi, trả về ĐỘ DÀI của nó;
nếu là số, trả về CHÍNH nó. Điền chỗ trống để thu hẹp kiểu đúng cách
trước khi gọi `.length`.

```typescript title=starter
function doDaiHoacGiaTri(x: string | number): number {
  if (typeof x === ___) {
    return x.length;
  }
  return x;
}

console.log("cam:", doDaiHoacGiaTri("cam"));
console.log("9:", doDaiHoacGiaTri(9));
```

```typescript title=solution
function doDaiHoacGiaTri(x: string | number): number {
  if (typeof x === "string") {
    return x.length;
  }
  return x;
}

console.log("cam:", doDaiHoacGiaTri("cam"));
console.log("9:", doDaiHoacGiaTri(9));
```

```typescript title=test
if (doDaiHoacGiaTri("cam") !== 3) {
  throw new Error("doDaiHoacGiaTri(\"cam\") phải là 3 (độ dài chuỗi \"cam\") — đang là " + doDaiHoacGiaTri("cam"));
}
if (doDaiHoacGiaTri(9) !== 9) {
  throw new Error("doDaiHoacGiaTri(9) phải là 9 (chính giá trị số) — đang là " + doDaiHoacGiaTri(9));
}
```

:::hints
- kind: attention
  body: '.length chỉ tồn tại trên string, không tồn tại trên number — muốn gọi được nó, nhánh if phải thu hẹp x xuống đúng string trước, bằng typeof x === (một chuỗi tên kiểu).'
- kind: strategy
  body: 'typeof x trả về một chuỗi tên kiểu, ví dụ "string" hoặc "number". So sánh typeof x === "string" để TypeScript biết trong nhánh if, x chắc chắn là string, và .length gọi được an toàn.'
- kind: one-line
  body: 'Chỗ trống là: "string"'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "cam: 3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`typeof x === "string"` không chỉ là một điều kiện `if` bình thường —
nó khiến TypeScript THU HẸP kiểu của `x`, và mở khoá đúng những phương
thức thuộc về kiểu đã thu hẹp.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Union type giúp một GIÁ TRỊ ĐƠN LẺ có thể là một trong nhiều kiểu. Nhưng
nhiều bài toán không chỉ cần một giá trị đơn — chúng cần một khối dữ
liệu PHỨC HỢP, nhiều trường cùng lúc, như một người có cả tên lẫn tuổi.
Có cách nào ĐẶT TÊN cho một HÌNH DẠNG object như vậy, để dùng lại nhiều
nơi trong chương trình, thay vì viết lại từng trường mỗi lần không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
