---
id: lap-trinh-ham.cong-typescript.goi-thieu-thua-doi-so-cung-bi-chan
title: "Gọi thiếu hoặc thừa đối số cũng bị chặn"
summary: "Không chỉ kiểu — SỐ LƯỢNG đối số cũng là một phần lời hứa. Gọi gapDoi() (thiếu) hay gapDoi(1, 2) (thừa) đều bị TSC từ chối trước khi chạy, mã lỗi TS2554."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.arg-count-check]
requires: [ts.arg-type-check]
concepts: [ts.arg-count-check]
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
Câu hỏi cuối bài trước: TypeScript có chặn được cả SỐ LƯỢNG đối số,
không chỉ kiểu của chúng? Có — và nó dùng một mã lỗi khác hẳn hai mã
đã gặp.
::::

::::explain{#doc-so-luong-doi-so}
Bài 8 dạy: đối số sai KIỂU bị chặn lúc gọi. Nhưng lời hứa của một tham
số còn có một phần nữa, dễ bị quên vì nó không nằm ở dấu hai chấm nào
cả — chính SỰ TỒN TẠI của tham số đó.

```typescript
function gapDoi(x: number): number {
  return x * 2;
}
```

Hàm `gapDoi` khai đúng MỘT tham số. Đó cũng là một lời hứa: mọi lời
gọi `gapDoi` phải đưa vào ĐÚNG MỘT đối số — không thiếu, không thừa.
Gọi `gapDoi()` (không đối số nào) hay `gapDoi(1, 2)` (hai đối số) đều
bị TypeScript từ chối, dùng một mã lỗi riêng: TS2554.

Python xử lý việc thiếu tham số hoàn toàn khác. `def gap_doi(x): ...`
rồi gọi `gap_doi()` cũng lỗi — nhưng lỗi đó (`TypeError: missing 1
required positional argument`) chỉ lộ ra LÚC DÒNG GỌI THỰC SỰ CHẠY,
có thể là rất lâu sau khi chương trình khởi động, tuỳ dòng đó nằm
trong nhánh nào. TypeScript đếm số đối số ngay LÚC BIÊN DỊCH, trước
khi có dòng nào chạy.
::::

::::example{#doc-vi-du-so-luong}
Thiếu đối số:

```typescript
function gapDoi(x: number): number {
  return x * 2;
}

console.log(gapDoi());
```

```text title=readonly
(không in ra gì cả)

TS2554 (dòng 5, cột 13): Expected 1 arguments, but got 0.
```

Thừa đối số:

```typescript
function gapDoi(x: number): number {
  return x * 2;
}

console.log(gapDoi(1, 2));
```

```text title=readonly
(không in ra gì cả)

TS2554 (dòng 5, cột 23): Expected 1 arguments, but got 2.
```

Đã chạy thật cả hai. Thông báo của TS2554 đọc trực tiếp được:
"Expected 1 arguments, but got 0" nghĩa là "cần 1 đối số, nhưng nhận
được 0" — TypeScript đếm và so sánh, không cần đọc thêm gì khác.
::::

::::predict{#chuoi-thieu-doi-so commitOnce}
Byte viết hai dòng gọi cùng một hàm, KHÔNG chạy thử:

```typescript
function congHaiSo(a: number, b: number): number {
  return a + b;
}

console.log(congHaiSo(3, 4));
console.log(congHaiSo(3));
```

**Trước khi đọc đáp án**, dòng nào bị TypeScript từ chối, và vì sao?

:::opt{correct}
Dòng cuối — `congHaiSo(3)` — hàm khai HAI tham số (`a`, `b`), nhưng
dòng này chỉ đưa vào một đối số. TypeScript không tự coi `b` là "để
trống cũng được" — thiếu một đối số là thiếu, dù kiểu của đối số đã
có (`3`) hoàn toàn đúng: TS2554
:::

:::opt
`congHaiSo(3, 4)` — hai đối số cùng nằm trên một dòng phải cách nhau
bằng dấu chấm phẩy `;`, không phải dấu phẩy `,`
::why
Gần đúng ở việc bạn để ý có một dấu phân cách giữa `3` và `4`, và nhớ
tới dấu `;` đã học ở bài 1 (cuối mỗi câu lệnh).

Chỗ lệch: dấu `;` đánh dấu HẾT một câu lệnh; dấu `,` phân cách các
PHẦN TỬ trong một danh sách (ở đây là danh sách đối số của một lời
gọi hàm) — hai vai trò khác nhau, không thay được cho nhau. Đã thử
thật: `congHaiSo(3, 4)` biên dịch sạch.
::
:::

:::opt
Cả hai dòng đều bị từ chối — hàm `congHaiSo` được gọi hai lần với số
đối số khác nhau, và TypeScript đòi mọi lời gọi một hàm phải giống hệt
nhau về số đối số
::why
Gần đúng ở việc bạn để ý hai lời gọi khác số đối số, và nghi ngờ sự
KHÔNG NHẤT QUÁN giữa chúng mới là vấn đề.

Chỗ lệch: TypeScript không so hai lời gọi VỚI NHAU — mỗi lời gọi được
so RIÊNG với chữ ký đã khai của hàm. `congHaiSo(3, 4)` có đúng 2 đối
số, khớp chữ ký, không sao cả — dù dòng sau nó sai. Đã thử thật: chỉ
đúng một dòng (`congHaiSo(3)`) bị từ chối.
::
:::

:::opt
Không dòng nào bị từ chối — tham số `b` không được dùng ở phép tính
nào phức tạp (chỉ `a + b`), nên TypeScript cho phép bỏ trống nó khi
gọi
::why
Gần đúng ở việc bạn nghĩ mức độ "quan trọng" của một tham số trong
thân hàm có thể ảnh hưởng tới việc nó có bắt buộc hay không.

Chỗ lệch: TypeScript không nhìn vào thân hàm để quyết định một tham số
có bắt buộc hay không — nó chỉ nhìn vào CHỮ KÝ đã khai. Một tham số
bắt buộc trừ khi được đánh dấu rõ là có thể vắng mặt (dấu `?`, cụm bài
sau sẽ gặp lại ý này với `undefined`). Đã thử thật: `congHaiSo(3)` bị
từ chối, đúng mã `TS2554`.
::
:::
::::

::::code{#tinh-tong-ba-so}
Hàm `tinhTongBaSo` hứa nhận đúng BA tham số. Byte gọi hàm ở dòng cuối
nhưng quên mất tham số thứ ba — dòng này đang bị TypeScript từ chối.
Sửa lại dòng gọi cho đủ ba đối số: `a = 2`, `b = 5`, `c = 7`.

```typescript title=starter
function tinhTongBaSo(a: number, b: number, c: number): number {
  return a + b + c;
}

console.log(tinhTongBaSo(2, 5));
```

```typescript title=solution
function tinhTongBaSo(a: number, b: number, c: number): number {
  return a + b + c;
}

console.log(tinhTongBaSo(2, 5, 7));
```

```typescript title=test
if (tinhTongBaSo(2, 5, 7) !== 14) throw new Error("tinhTongBaSo(2, 5, 7) phải là 14 — đang là " + tinhTongBaSo(2, 5, 7));
if (tinhTongBaSo(1, 1, 1) !== 3) throw new Error("tinhTongBaSo(1, 1, 1) phải là 3 — đang là " + tinhTongBaSo(1, 1, 1));
```

:::hints
- kind: attention
  body: tinhTongBaSo khai ba tham số (a, b, c), nhưng dòng gọi ở cuối chỉ đưa vào hai đối số — đó chính là lý do cả chương trình bị chặn, chưa chạy được dòng nào.
- kind: strategy
  body: 'Thêm đối số thứ ba vào lời gọi: tinhTongBaSo(2, 5, 7) — với a=2, b=5, c=7.'
- kind: one-line
  body: 'Sửa dòng gọi thành: console.log(tinhTongBaSo(2, 5, 7));'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "14"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Kiểu đúng thôi chưa đủ — SỐ LƯỢNG đối số cũng là một phần lời hứa, và
TypeScript đếm nó trước khi cho phép bất cứ dòng nào chạy.
::::

::::reflect{#nghi-lai}
Giờ bạn đã thấy đủ bốn mảnh của lời hứa quanh một hàm: kiểu tham số
(bài 6), kiểu trả về (bài 7), kiểu đối số lúc gọi (bài 8), số lượng
đối số lúc gọi (bài 9).

Nếu tự bạn viết một hàm dùng ĐỦ bốn mảnh đó — không phải chỉ đọc ví dụ
của Byte — bạn có tự đoán trước được TypeScript sẽ chặn ở đâu khi gọi
sai, trước cả khi bấm Chạy không? Bài sau để bạn tự thử.
::::

::::checkpoint{mastery=0.8}
::::
