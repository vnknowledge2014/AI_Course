---
id: lap-trinh-ham.cong-typescript.tham-so-ham-cung-can-kieu
title: "Tham số hàm cũng cần kiểu"
summary: "function tinhTong(x: number) — tham số x giờ mang một lời hứa kiểu riêng, y hệt biến. Bỏ trống lời hứa đó, TypeScript không đoán bừa — nó báo TS7006, tham số ngầm mang kiểu any."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.param-type]
requires: [ts.predict-compile-error, core.function-parameter]
concepts: [ts.param-type]
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
Câu hỏi cuối bài trước: tham số hàm có cần một lời hứa kiểu, giống
hệt biến không? Có. Và bỏ trống lời hứa ấy không phải là "khỏi lo" —
TypeScript nhận ra ngay bạn bỏ trống.
::::

::::explain{#doc-tham-so-can-kieu}
R1.T1.3 dạy tham số bằng cách đặt cho nó một cái TÊN: hàm nhận `x`,
dùng `x` ở thân hàm. TypeScript giữ nguyên cái tên đó, chỉ thêm dấu
hai chấm ngay sau:

```typescript
function tinhTong(x: number) {
  return x + 1;
}
```

Giờ `x` không chỉ có tên — nó còn mang một LỜI HỨA, tách biệt với lời
hứa của bất kỳ biến nào khác trong hàm: mọi giá trị truyền vào `x` lúc
gọi hàm PHẢI là `number`.

Nếu bỏ trống dấu hai chấm — `function tinhTong(x) {` — TypeScript
KHÔNG tự suy luận như với biến (bài 4). Với biến, có một giá trị khởi
tạo ngay tại chỗ khai báo (`let x = 5`) để suy kiểu từ đó. Tham số thì
khác: nó chỉ nhận giá trị THẬT lúc hàm được GỌI, ở một chỗ khác, có
khi rất xa dòng định nghĩa — không có gì để suy luận ngay tại chỗ
khai báo cả. TypeScript đành gán cho `x` kiểu `any` — kiểu "chấp nhận
bất cứ thứ gì, tắt hết mọi kiểm tra". Nhưng ở chế độ nghiêm khắc
(`strict: true`, đã bật cho toàn bộ cổng này), một `any` NGẦM ĐỊNH như
vậy bị coi là lỗi, không phải một cái tắt lặng lẽ: mã lỗi TS7006.
::::

::::example{#doc-vi-du-tham-so}
Cùng một hàm, chỉ khác một dấu hai chấm.

```typescript
function tinhTong(x) {
  return x + 1;
}

console.log(tinhTong(4));
```

```text title=readonly
(không in ra gì cả)

TS7006 (dòng 1, cột 19): Parameter 'x' implicitly has an 'any' type.
```

Thêm đúng một dấu hai chấm và một kiểu:

```typescript
function tinhTong(x: number) {
  return x + 1;
}

console.log(tinhTong(4));
```

```text title=readonly
5
```

Đã chạy thật cả hai: thân hàm — `return x + 1;` — không đổi lấy một
chữ. Chỉ có lời hứa kiểu của tham số thay đổi, và đó là thứ quyết
định cả chương trình có được phép chạy hay không.
::::

::::predict{#chuoi-tham-so-thieu-kieu commitOnce}
Byte viết bốn hàm, KHÔNG chạy thử:

```typescript
function tinhTong(x: number) { return x + 1; }
function nhanDoi(x) { return x + 1; }
function congHai(x: number, y: number) { return x + y; }
function doDai(x: string) { return x.length; }
```

**Trước khi đọc đáp án**, hàm nào bị TypeScript từ chối, và vì sao?

:::opt{correct}
`nhanDoi` — tham số `x` không có dấu hai chấm kèm kiểu, và TypeScript
không có gì để suy luận kiểu từ đó (khác biến, tham số không có giá
trị khởi tạo ngay tại chỗ khai báo). `x` ngầm mang kiểu `any`, và ở
chế độ nghiêm khắc đó là lỗi: TS7006
:::

:::opt
`tinhTong` — tham số có kiểu (`x: number`) nhưng thân hàm
`return x + 1;` không khai kiểu trả về tường minh, nên bị từ chối
::why
Gần đúng ở việc bạn để ý thân hàm không viết `: number` sau dấu ngoặc
tham số.

Chỗ lệch: TypeScript vẫn tự SUY LUẬN kiểu trả về từ thân hàm, y hệt
cách nó suy luận kiểu biến ở bài 4 — không bắt buộc viết kiểu trả về
tường minh. Đã thử thật: `tinhTong` biên dịch sạch, không lỗi nào.
::
:::

:::opt
`congHai` — hai tham số cùng mang kiểu `number` không được phép,
TypeScript chỉ cho các tham số của một hàm mang kiểu khác nhau
::why
Gần đúng ở việc bạn nghi ngờ có thêm một ràng buộc khi hàm nhận nhiều
tham số.

Chỗ lệch: không có luật nào như vậy. Mỗi tham số hứa kiểu HOÀN TOÀN
ĐỘC LẬP với các tham số khác — hai tham số cùng là `number` không va
chạm gì cả. Đã thử thật: `congHai` biên dịch sạch.
::
:::

:::opt
`doDai` — `.length` không phải một thuộc tính có thật trên kiểu
`string`, phải viết một hàm riêng để đếm ký tự
::why
Gần đúng ở việc bạn thận trọng với những gì gắn theo một kiểu cụ thể —
đúng mối bận tâm sẽ quay lại rõ hơn ở union type (bài 16).

Chỗ lệch: `.length` LÀ một thuộc tính có thật trên mọi `string`, đo
đúng số ký tự — cả trong TypeScript lẫn JavaScript. Đã thử thật:
`doDai` biên dịch sạch, không lỗi nào.
::
:::
::::

::::code{#tinh-gia-ve}
Byte bán vé, mỗi vé 15.000đ. Hàm `tinhGia` đã có tham số mang đúng
kiểu — việc còn lại là tính tổng tiền bên trong thân hàm.

```typescript title=starter
function tinhGia(soLuong: number) {
  return ___;
}

console.log(tinhGia(3));
```

```typescript title=solution
function tinhGia(soLuong: number) {
  return soLuong * 15000;
}

console.log(tinhGia(3));
```

```typescript title=test
if (tinhGia(3) !== 45000) throw new Error("tinhGia(3) phải là 45000 — đang là " + tinhGia(3));
if (tinhGia(1) !== 15000) throw new Error("tinhGia(1) phải là 15000 — đang là " + tinhGia(1));
```

:::hints
- kind: attention
  body: soLuong đã hứa kiểu number rồi (bài học này) — không cần kiểm tra kiểu của nó nữa, chỉ cần tính đúng số tiền.
- kind: strategy
  body: 'Mỗi vé giá 15000. Tổng tiền = số lượng vé nhân với 15000 — dùng soLuong * 15000.'
- kind: one-line
  body: 'Chỗ trống là: soLuong * 15000'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "45000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tham số giờ có lời hứa kiểu, giống hệt biến. Quên viết nó — TypeScript
không đoán giùm, nó từ chối thẳng bằng TS7006.
::::

::::reflect{#nghi-lai}
Tham số giờ có lời hứa kiểu. Nhưng một hàm không chỉ NHẬN giá trị —
nó còn TRẢ VỀ một giá trị, ở dòng `return`.

Giá trị trả về có cần một lời hứa kiểu RIÊNG, tách biệt với lời hứa
của tham số, hay dùng chung luôn lời hứa đó? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
