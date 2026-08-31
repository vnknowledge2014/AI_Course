---
id: lap-trinh-ham.cong-typescript.khong-can-viet-kieu-may-tu-suy-luan
title: "Không cần viết kiểu — máy tự suy luận được"
summary: "`let x = 5` (không có `: number`) — TypeScript vẫn TỰ SUY RA kiểu `number` từ giá trị khởi tạo, và một dòng gán sai kiểu sau đó vẫn bị chặn y hệt như khai tường minh. Suy luận không lỏng hơn — chỉ đỡ phải gõ."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.type-inference]
requires: [ts.compile-time-check]
concepts: [ts.type-inference]
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
Nếu bạn lười, không viết dấu hai chấm nào — TypeScript có bó tay
không? Không. Nó tự nhìn ra. Byte cho xem cách.
::::

::::explain{#suy-luan-khong-phai-long-tay}
Viết `: number`, `: string` mọi lần là hơi phiền — và hoá ra thường
KHÔNG CẦN THIẾT. Cho TypeScript một giá trị khởi tạo ngay lúc khai báo,
nó tự NHÌN vào giá trị đó và SUY RA kiểu, không cần bạn viết ra:

```
let x = 5;
```

Không dấu hai chấm nào ở đây. Nhưng TypeScript vẫn "ghim" kiểu `number`
cho `x`, y hệt như bạn đã viết `let x: number = 5;` tường minh — chỉ là
nó tự viết dòng đó THAY BẠN, ở một chỗ bạn không nhìn thấy.

Điểm quan trọng nhất, và dễ hiểu lầm nhất: **lời hứa suy luận ra vẫn là
một lời hứa THẬT**, kiểm nghiêm ngặt y hệt lời hứa viết tường minh. Nó
KHÔNG lỏng hơn. `x = "chuỗi"` ở một dòng sau vẫn bị TypeScript từ chối,
dù chưa từng có dấu hai chấm nào xuất hiện trong cả chương trình. Suy
luận chỉ tiết kiệm việc GÕ — không tiết kiệm việc KIỂM.

Suy luận chỉ hoạt động khi có GIÁ TRỊ KHỞI TẠO ngay lúc khai báo. Viết
`let d;` (không giá trị, không kiểu) thì TypeScript không có gì để
nhìn vào — trường hợp đó gần như không bị kiểm gì cả, khác hẳn ba biến
có giá trị khởi tạo.
::::

::::example{#suy-luan-van-chan-that}
Hai đoạn, cùng biến `x`, không dòng nào viết `: number`:

```typescript title=readonly
let x = 5;
x = 10;
console.log(x);
```

```text title=readonly
10
```

Gán lại bằng một số khác — vẫn đúng kiểu đã suy luận, TypeScript im
lặng cho qua.

```typescript title=readonly
let x = 5;
x = "chuoi";
console.log(x);
```

```text title=readonly
(không in ra gì cả)

TS2322 (dòng 2, cột 1): Type 'string' is not assignable to type 'number'.
```

Cùng một biến `x`, không một dấu hai chấm nào trong cả đoạn — nhưng
dòng gán chuỗi vẫn bị từ chối bằng ĐÚNG mã lỗi `TS2322`, ĐÚNG thông
điệp bạn đã gặp ở những bài khai kiểu tường minh. Suy luận không phải
một chế độ dễ dãi hơn.
::::

::::predict{#suy-luan-cho-ca-bon commitOnce}
Byte viết bốn dòng, không dòng nào có dấu hai chấm:

```typescript
let a = 5;
let b = "meo";
let c = true;
let d;
```

Sau đó Byte thử gán lại: `a = "năm";`. **Trước khi đọc đáp án**, dòng
gán lại này có bị TypeScript từ chối không, dù `a` chưa từng được khai
kiểu tường minh?

:::opt{correct}
Có, bị từ chối — TypeScript đã tự suy ra `a` là kiểu `number` ngay từ
`let a = 5`, và gán một chuỗi cho `a` sau đó phá đúng lời hứa ấy, y hệt
như khai `let a: number = 5` tường minh
:::

:::opt
Không — suy luận kiểu chỉ là một GỢI Ý cho trình soạn thảo, không phải
một luật thật sự được kiểm khi biên dịch
::why
Gần đúng ở việc trình soạn thảo (như VS Code) đúng là dùng suy luận
kiểu để GỢI Ý lúc bạn gõ — điều đó có thật.

Chỗ lệch: gợi ý đó không phải trang trí. Nó bắt nguồn từ MỘT lời hứa
THẬT được trình biên dịch KIỂM — TypeScript từ chối biên dịch dòng gán
lại này y hệt trường hợp khai tường minh, đã đo thật bằng đúng mã lỗi
`TS2322`.
::
:::

:::opt
Có, nhưng chỉ vì `5` là một số nguyên — nếu khai `let a = 5.0` thì
TypeScript sẽ không suy luận được kiểu, và dòng gán lại sẽ được chấp
nhận
::why
Gần đúng ở việc bạn để ý TypeScript CÓ khái niệm phân biệt các loại số
khác nhau ở NGÔN NGỮ khác (như int/float trong Python).

Chỗ lệch: TypeScript chỉ có DUY NHẤT một kiểu số, gọi là `number` —
không phân biệt số nguyên hay số thực. Suy luận từ `5` hay `5.0` đều
ra đúng một kiểu `number`, không ảnh hưởng gì tới việc dòng gán lại có
bị chặn hay không.
::
:::

:::opt
Không — vì `d` (không có giá trị khởi tạo) mới là biến duy nhất KHÔNG
suy luận được kiểu, còn `a`, `b`, `c` suy luận ra nhưng không áp dụng
cho việc gán lại sau này
::why
Gần đúng ở việc bạn để ý đúng: `d` là một trường hợp đặc biệt — không
có giá trị khởi tạo, TypeScript không suy ra được kiểu cụ thể nào cho
nó.

Chỗ lệch: điều đó không liên quan gì tới `a`, `b`, `c`. Ba biến này ĐỀU
có giá trị khởi tạo, TypeScript suy luận được kiểu cho cả ba, và lời
hứa suy luận đó ÁP DỤNG đầy đủ cho mọi lần gán lại sau này — không phải
một ngoại lệ chỉ dành cho khai báo tường minh.
::
:::
::::

::::code{#tinh-tien-khong-can-khai-kieu}
Tính tổng tiền một đơn hàng — không viết một dấu hai chấm nào cả. Điền
hai giá trị, để TypeScript tự suy luận kiểu `number` cho cả hai.

```typescript title=starter
let gia = ___;                       // 15000 đồng một sản phẩm
let so_luong = ___;                  // mua 3 sản phẩm

let tong = gia * so_luong;

console.log("Tổng:", tong);
```

```typescript title=solution
let gia = 15000;
let so_luong = 3;

let tong = gia * so_luong;

console.log("Tổng:", tong);
```

```typescript title=test
if (tong !== 45000) {
  throw new Error("tong phải là 45000 (15000 * 3) — đang là " + tong);
}
```

:::hints
- kind: attention
  body: Hai chỗ trống, không viết kiểu nào — chỉ điền giá trị. TypeScript tự suy luận kiểu number từ chính giá trị đó, và phép nhân gia * so_luong đòi cả hai đều là number.
- kind: strategy
  body: 'gia là 15000 đồng một sản phẩm, so_luong là 3 sản phẩm. Điền đúng hai con số này — không viết dấu hai chấm, không viết chuỗi.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là: 15000 và 3'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Tổng: 45000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không viết một chữ `number` nào — nhưng TypeScript vẫn biết chính xác,
và vẫn giữ đúng lời hứa nó tự đặt ra.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn biết ba điều ghép lại: một lời hứa kiểu (bài 2), bị kiểm TRƯỚC
KHI CHẠY chứ không phải giữa chừng (bài 3), và lời hứa đó vẫn có hiệu
lực dù không viết ra tường minh (bài này, suy luận).

Ghép cả ba lại: cho một đoạn mã Byte CHƯA TỪNG chạy thử, dựa vào lời
hứa và suy luận, bạn có ĐOÁN ĐƯỢC dòng nào sẽ bị từ chối — KHÔNG CẦN
bấm nút Chạy — không? Bài sau thử thách đúng điều đó.
::::

::::checkpoint{mastery=0.8}
::::
