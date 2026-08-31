---
id: lap-trinh-ham.cong-typescript.cung-mot-chuong-trinh-hai-ngon-ngu
title: "Cùng một chương trình, hai ngôn ngữ khác nhau"
summary: "452 bài trước dùng Python. Từ bài này, cùng một Ý — biến giữ giá trị, hàm là một khối lệnh gọi lại được — được gõ bằng cú pháp TypeScript: `let`/`const` thay gán trần, dấu `;` cuối dòng, dấu ngoặc nhọn thay thụt lề."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.syntax-bridge]
requires: [core.variable, core.function-def]
concepts: [ts.syntax-bridge]
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
Bốn trăm năm mươi hai bài, một ngôn ngữ. Hôm nay đổi ngôn ngữ — nhưng
không đổi cách bạn nghĩ. Byte dịch một chương trình Python nhỏ sang
TypeScript, ngay trước mắt bạn.
::::

::::explain{#mot-tap-tu-vung-moi}
Đây là track đầu tiên của khoá dùng **TypeScript** thay vì Python. Tin
tốt: bạn không học lại tư duy nào cả. Biến vẫn giữ giá trị. Hàm vẫn là
một khối lệnh có tên, gọi lại được nhiều lần. Rẽ nhánh, mảng — tất cả
những gì bạn đã biết vẫn còn nguyên.

Cái đổi chỉ là CÁCH GÕ. Bốn chỗ khác nhau, gặp ngay trong mọi chương
trình:

1. **Khai báo biến cần một từ khoá đứng trước tên** — `let ten = ...`
   hoặc `const ten = ...`, không gán trần như Python (`ten = ...`).
2. **Mỗi lệnh kết thúc bằng dấu chấm phẩy** `;` — không dùng dấu xuống
   dòng để đánh dấu hết lệnh như Python.
3. **Thân hàm và thân khối nằm trong dấu ngoặc nhọn** `{ }` — không
   dùng thụt lề để đánh dấu "cái gì thuộc về cái gì".
4. **Hàm khai bằng từ khoá `function`**, không phải `def` — và in ra
   màn hình bằng `console.log(...)`, không phải `print(...)`.

`let` với `const` khác nhau ở một điểm: `let` cho phép GÁN LẠI giá trị
sau này, `const` thì không — khoá giá trị lại ngay từ dòng khai báo.
Dùng cái nào, xem biến đó có đổi giá trị về sau hay không.
::::

::::example{#doi-mot-chuong-trinh-nho}
Cùng một câu chuyện — một khách tên cố định, một số điểm sẽ đổi — viết
bằng hai ngôn ngữ:

```python title=readonly
ten_khach = "Lan"
so_diem = 100

so_diem = so_diem + 20

def chao_khach():
    print("Khách:", ten_khach)
    print("Điểm:", so_diem)

chao_khach()
```

```typescript title=readonly
const ten_khach = "Lan";
let so_diem = 100;

so_diem = so_diem + 20;

function chao_khach() {
  console.log("Khách:", ten_khach);
  console.log("Điểm:", so_diem);
}

chao_khach();
```

```text title=readonly
Khách: Lan
Điểm: 120
```

Cùng một kết quả, ĐÚNG như nhau. `ten_khach` không đổi trong suốt
chương trình nên dùng `const`; `so_diem` bị cộng thêm ở dòng thứ ba nên
phải dùng `let` — `const` sẽ không cho phép dòng đó. Thân hàm
`chao_khach` chuyển từ thụt lề sang cặp `{ }`, và `print` chuyển thành
`console.log`. Từng dòng đổi cách gõ, không dòng nào đổi Ý.
::::

::::predict{#gan-lai-mot-bien commitOnce}
Đoạn Python này khai một biến rồi GÁN LẠI nó ngay dòng sau:

```python
so_du = 100
so_du = so_du + 50
```

Byte dịch nó sang TypeScript. **Trước khi đọc đáp án**, cách dịch nào
dưới đây đúng?

:::opt{correct}
`let so_du = 100;` rồi `so_du = so_du + 50;` — `let` cho phép gán lại
sau khi khai báo, đúng như Python cho phép gán lại `so_du` ở dòng sau
:::

:::opt
`const so_du = 100;` rồi `so_du = so_du + 50;` — `const` cũng là một
cách khai báo hợp lệ, dùng được y hệt `let`
::why
Gần đúng ở việc `const` đúng là một cách khai báo hợp lệ trong
TypeScript, giống `let` ở CÚ PHÁP.

Chỗ lệch: `const` khoá GIÁ TRỊ lại ngay từ dòng khai báo — không cho
gán lại lần nào nữa. Dòng thứ hai ở đây là một phép GÁN LẠI, và
`const` từ chối đúng phép gán đó — đã thử thật: TypeScript báo lỗi
"Cannot assign to 'so_du' because it is a constant." `let` mới là từ
khoá dành cho một biến còn đổi giá trị được.
::
:::

:::opt
`so_du = 100;` rồi `so_du = so_du + 50;` — không cần từ khoá nào cả,
viết y hệt cách Python gán trần
::why
Gần đúng ở việc Python đúng là viết y hệt vậy — một dòng gán trần,
không từ khoá nào đứng trước.

Chỗ lệch: TypeScript không có kiểu gán trần này cho một cái tên CHƯA
TỪNG khai báo. Thiếu cả `let` lẫn `const`, TypeScript coi `so_du` là
một cái tên chưa từng tồn tại — đã thử thật: lỗi "Cannot find name
'so_du'.", y hệt việc gõ sai chính tả một tên biến.
::
:::

:::opt
`let so_du = 100;` rồi `let so_du = so_du + 50;` — dùng `let` cho cả
hai dòng, để chắc chắn cú pháp luôn đúng
::why
Gần đúng ở việc cả hai dòng đều dùng đúng từ khoá `let`.

Chỗ lệch: `let` chỉ cho phép khai báo một tên MỘT LẦN trong cùng phạm
vi. Dòng thứ hai viết lại `let so_du` là khai báo LẦN THỨ HAI, không
phải gán lại. Muốn đổi giá trị của một biến đã có, dòng sau không viết
`let` nữa, chỉ viết `so_du = ...`. Đã thử thật: lỗi "Cannot redeclare
block-scoped variable 'so_du'."
::
:::
::::

::::code{#dich-don-hang}
Một đơn hàng nhỏ: tên khách cố định, điểm thưởng đổi sau một lần cộng
thêm. Viết hàm mô tả điểm, rồi gọi nó — điền đúng MỘT chỗ trống.

```typescript title=starter
const ten_khach = "Mai";
let so_diem = 200;

so_diem = so_diem + 30;

function mo_ta_diem() {
  return ten_khach + " có " + so_diem + " điểm";
}

let dong_mo_ta = ___;                // gọi mo_ta_diem, lưu kết quả

console.log(dong_mo_ta);
```

```typescript title=solution
const ten_khach = "Mai";
let so_diem = 200;

so_diem = so_diem + 30;

function mo_ta_diem() {
  return ten_khach + " có " + so_diem + " điểm";
}

let dong_mo_ta = mo_ta_diem();

console.log(dong_mo_ta);
```

```typescript title=test
if (dong_mo_ta !== "Mai có 230 điểm") {
  throw new Error(
    "dong_mo_ta phải là kết quả THẬT SỰ của việc gọi mo_ta_diem() — đang ra " + dong_mo_ta,
  );
}
```

:::hints
- kind: attention
  body: Một chỗ trống — gọi hàm mo_ta_diem() (không nhận tham số nào) và gán kết quả vào dong_mo_ta.
- kind: strategy
  body: 'mo_ta_diem không nhận tham số nào, chỉ đọc hai biến đã có sẵn ở ngoài. Gọi nó bằng mo_ta_diem() — có dấu ngoặc tròn rỗng — rồi gán thẳng kết quả trả về vào dong_mo_ta.'
- kind: one-line
  body: 'Chỗ trống là: mo_ta_diem()'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Mai có 230 điểm"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chương trình TypeScript đầu tiên của bạn vừa chạy — `const`, `let`,
dấu chấm phẩy, dấu ngoặc nhọn, đúng cả bốn chỗ khác. Ý vẫn là Ý cũ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại chương trình bạn vừa viết: không dòng nào có dấu hai chấm sau
tên biến. Bạn chỉ viết `let so_diem = 200`, y hệt cách gán một biến
trong Python, chỉ đổi thêm chữ `let` và dấu `;`.

Nhưng TypeScript nổi tiếng chính vì nó NÓI RÕ kiểu dữ liệu — thường
bằng một dấu hai chấm ngay sau tên biến, kiểu `ten: string`. Dấu hai
chấm đó dùng để làm gì, và khi nào cần viết nó ra? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
