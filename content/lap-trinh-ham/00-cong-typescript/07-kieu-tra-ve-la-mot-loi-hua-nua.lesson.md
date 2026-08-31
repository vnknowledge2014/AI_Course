---
id: lap-trinh-ham.cong-typescript.kieu-tra-ve-la-mot-loi-hua-nua
title: "Kiểu trả về là một lời hứa nữa"
summary: "Dấu hai chấm SAU dấu ngoặc tham số (`: number`) hứa về giá trị hàm TRẢ VỀ — một lời hứa khác, tách biệt với lời hứa về tham số. Một hàm giờ có hai lời hứa, không phải một."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.return-type]
requires: [ts.param-type]
concepts: [ts.return-type]
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
Câu hỏi cuối bài trước: giá trị trả về có cần một lời hứa kiểu riêng
không? Có — và nó nằm ở một chỗ khác hẳn lời hứa của tham số.
::::

::::explain{#doc-kieu-tra-ve}
Bài trước, dấu hai chấm đứng NGAY SAU một cái tên tham số:
`function tinhGia(soLuong: number)`. Đó là lời hứa về CÁI ĐI VÀO hàm.

Còn một chỗ khác cho dấu hai chấm: NGAY SAU dấu ngoặc đóng của danh
sách tham số, trước dấu ngoặc nhọn mở thân hàm:

```typescript
function gapDoi(x: number): number {
  return x * 2;
}
```

Dấu hai chấm này hứa về CÁI ĐI RA — giá trị mà `return` sẽ trả về.
Đây là một lời hứa HOÀN TOÀN TÁCH BIỆT với lời hứa của tham số: một
hàm có thể nhận vào một kiểu và trả về một kiểu khác hẳn (ví dụ nhận
`number`, trả về `string`) — hai lời hứa không bị buộc phải giống
nhau.

Và giống mọi lời hứa kiểu đã gặp từ bài 2, lời hứa này bị KIỂM TRƯỚC
KHI CHẠY (bài 3): nếu thân hàm `return` một giá trị KHÔNG khớp kiểu đã
khai sau dấu ngoặc, TypeScript từ chối biên dịch — không đợi tới lúc
hàm thực sự chạy.
::::

::::example{#doc-vi-du-kieu-tra-ve}
Một hàm hứa trả về `string`, nhưng thân hàm trả một `number`:

```typescript
function chuyenThanhChuoi(x: number): string {
  return x;
}

console.log(chuyenThanhChuoi(5));
```

```text title=readonly
(không in ra gì cả)

TS2322 (dòng 2, cột 3): Type 'number' is not assignable to type
'string'.
```

Sửa thân hàm cho khớp lời hứa — dùng `String(x)` để thật sự đổi `x`
(vẫn là `number`) thành một `string`:

```typescript
function chuyenThanhChuoi(x: number): string {
  return String(x);
}

console.log(chuyenThanhChuoi(5));
```

```text title=readonly
5
```

Đã chạy thật cả hai. Chú ý: mã lỗi TS2322 này là MÃ Y HỆT bài 3 từng
gặp khi gán sai kiểu cho một biến (`let n: number = "ba"`). Không phải
trùng hợp — `return` bên trong một hàm có kiểu trả về tường minh cũng
là một phép GÁN, chỉ khác chỗ gán: gán giá trị đó cho "lời hứa trả
về" của hàm, thay vì gán cho một biến.
::::

::::predict{#chuoi-kieu-tra-ve-sai commitOnce}
Byte viết bốn hàm, KHÔNG chạy thử:

```typescript
function congMot(x: number): number { return x + 1; }
function chuyenChuoi(x: number): string { return x + 1; }
function laDuong(x: number): boolean { return x > 0; }
function thanhChuoi(x: number): string { return x.toString(); }
```

**Trước khi đọc đáp án**, hàm nào bị TypeScript từ chối, và vì sao?

:::opt{correct}
`chuyenChuoi` — hàm hứa trả về `string` (`: string` sau dấu ngoặc),
nhưng thân hàm `return x + 1;` tính ra một `number` (`x` là `number`,
cộng `1` vẫn là `number`) — giá trị trả về không khớp lời hứa: TS2322
:::

:::opt
`congMot` — tham số `x` và kiểu trả về cùng là `number`, TypeScript
không cho phép tham số và kiểu trả về trùng nhau
::why
Gần đúng ở việc bạn để ý cả hai lời hứa của `congMot` cùng là
`number`, và nghi ngờ đó là vấn đề.

Chỗ lệch: hai lời hứa của một hàm — tham số và trả về — HOÀN TOÀN ĐỘC
LẬP với nhau. Trùng kiểu không vi phạm gì cả, rất nhiều hàm hợp lệ có
cả hai cùng là `number` (ví dụ chính `gapDoi` ở phần giải thích). Đã
thử thật: `congMot` biên dịch sạch.
::
:::

:::opt
`laDuong` — thân hàm `return x > 0;` tạo ra một phép so sánh, mà phép
so sánh không được phép dùng làm giá trị `return`
::why
Gần đúng ở việc bạn thận trọng với biểu thức so sánh (`x > 0`) xuất
hiện ngay sau từ khoá `return`.

Chỗ lệch: `x > 0` là một biểu thức hợp lệ, và giá trị của nó là một
`boolean` (`true` hoặc `false`) — khớp đúng lời hứa `: boolean` của
`laDuong`. Đã thử thật: `laDuong` biên dịch sạch, không lỗi nào.
::
:::

:::opt
`thanhChuoi` — gọi `.toString()` ngay trên tham số `x` (kiểu `number`)
là không hợp lệ, phải dùng hàm `String(x)` như ví dụ ở trên
::why
Gần đúng ở việc bạn nhớ đúng ví dụ vừa xem dùng `String(x)`.

Chỗ lệch: `.toString()` là một phương thức có thật trên mọi `number`,
cho kết quả `string` y hệt `String(x)` — chỉ là cách viết khác của
cùng một việc. Đã thử thật: `thanhChuoi` biên dịch sạch, không lỗi.
::
:::
::::

::::code{#tinh-dien-tich}
Hàm `tinhDienTich` đã hứa cả hai chiều: nhận vào hai `number`, trả về
một `number`. Việc còn lại là tính đúng diện tích ở thân hàm.

```typescript title=starter
function tinhDienTich(dai: number, rong: number): number {
  return ___;
}

console.log(tinhDienTich(4, 5));
```

```typescript title=solution
function tinhDienTich(dai: number, rong: number): number {
  return dai * rong;
}

console.log(tinhDienTich(4, 5));
```

```typescript title=test
if (tinhDienTich(4, 5) !== 20) throw new Error("tinhDienTich(4, 5) phải là 20 — đang là " + tinhDienTich(4, 5));
if (tinhDienTich(3, 3) !== 9) throw new Error("tinhDienTich(3, 3) phải là 9 — đang là " + tinhDienTich(3, 3));
```

:::hints
- kind: attention
  body: dai và rong đều đã hứa kiểu number, kiểu trả về cũng đã hứa number — chỉ còn thiếu công thức tính diện tích hình chữ nhật.
- kind: strategy
  body: 'Diện tích hình chữ nhật bằng chiều dài nhân chiều rộng — dùng dai * rong.'
- kind: one-line
  body: 'Chỗ trống là: dai * rong'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "20"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai lời hứa, một hàm: tham số hứa cái đi vào, kiểu trả về hứa cái đi
ra. Cả hai đều bị kiểm TRƯỚC KHI CHẠY, và cả hai đều độc lập với nhau.
::::

::::reflect{#nghi-lai}
Tới giờ, mọi lời hứa bạn thấy được KIỂM đều nằm ở phía ĐỊNH NGHĨA
hàm — kiểu tham số, kiểu trả về. Nhưng một hàm không chỉ được định
nghĩa một lần — nó còn được GỌI, có khi ở rất nhiều chỗ khác nhau
trong chương trình.

Phía GỌI hàm có bị kiểm theo đúng lời hứa đó không, hay lời hứa chỉ có
tác dụng ngay tại chỗ định nghĩa? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
