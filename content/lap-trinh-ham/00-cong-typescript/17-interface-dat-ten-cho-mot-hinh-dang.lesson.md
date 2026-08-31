---
id: lap-trinh-ham.cong-typescript.interface-dat-ten-cho-mot-hinh-dang
title: "Interface: đặt tên cho một HÌNH DẠNG dữ liệu"
summary: "`interface Nguoi { ten: string; tuoi: number }` — đặt tên cho một HÌNH DẠNG object, để dùng lại nhiều nơi thay vì viết lại từng trường mỗi lần. Thiếu một trường khi tạo giá trị kiểu này bị TSC từ chối (đã đo thật: TS2741)."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 17
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.interface]
requires: [ts.union-narrowing]
concepts: [ts.interface]
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
Union type đặt tên cho một GIÁ TRỊ có thể là một trong nhiều kiểu. Hôm
nay: đặt tên cho một object có NHIỀU TRƯỜNG cùng lúc.
::::

::::explain{#dat-ten-cho-hinh-dang}
Một object như `{ ten: "Byte", tuoi: 5 }` có một HÌNH DẠNG: nó có đúng
hai trường, `ten` (kiểu `string`) và `tuoi` (kiểu `number`). Nếu
chương trình cần TẠO NHIỀU object cùng hình dạng này — nhiều người,
mỗi người có tên và tuổi — viết đi viết lại `{ ten: string; tuoi:
number }` mỗi nơi rất dễ gõ lệch một trường mà không ai để ý.

**Interface** giải quyết đúng việc đó: đặt một CÁI TÊN cho một hình
dạng, khai một lần, dùng lại ở mọi nơi cần:

```
interface Nguoi {
  ten: string;
  tuoi: number;
}
```

Từ giờ, `Nguoi` không phải một GIÁ TRỊ — nó là một KIỂU, y hệt `string`
hay `number`, chỉ khác là kiểu này TỰ ĐỊNH NGHĨA bằng cách liệt kê các
trường. Một biến khai `: Nguoi` phải là một object có ĐỦ hai trường
`ten` và `tuoi`, đúng kiểu từng trường đã liệt kê — thiếu một trường,
sai kiểu một trường, đều bị TypeScript từ chối, cùng cơ chế lời hứa đã
gặp suốt track.
::::

::::example{#du-truong-thieu-truong}
Một interface `Nguoi`, và một object khớp đủ:

```typescript title=readonly
interface Nguoi {
  ten: string;
  tuoi: number;
}

const a: Nguoi = { ten: "Byte", tuoi: 5 };
console.log(a.ten, a.tuoi);
```

```text title=readonly
Byte 5
```

Đủ hai trường, đúng kiểu từng trường — TypeScript im lặng cho qua. Còn
nếu THIẾU một trường:

```typescript title=readonly
interface Nguoi {
  ten: string;
  tuoi: number;
}

const b: Nguoi = { ten: "Rin" };
```

```text title=readonly
(không biên dịch được)

TS2741 (dòng 6, cột 7): Property 'tuoi' is missing in type '{ ten: string; }' but required in type 'Nguoi'.
```

Thông điệp nói thẳng: `tuoi` bị thiếu, nhưng `Nguoi` đòi có trường đó.
Không cần bạn tự dò xem thiếu gì — TypeScript chỉ đích danh tên trường,
và tên interface đã định nghĩa nó.
::::

::::predict{#dong-nao-bi-tu-choi commitOnce}
Byte viết một interface và ba giá trị, KHÔNG chạy thử:

```typescript
interface SanPham {
  ten: string;
  gia: number;
}

const a: SanPham = { ten: "Bánh", gia: 20000 };
const b: SanPham = { ten: "Kẹo" };
const c: SanPham = { ten: "Nước", gia: 10000 };
```

**Trước khi đọc đáp án**, dòng nào bị TypeScript từ chối, và vì sao?

:::opt{correct}
Dòng khai `b` — object `{ ten: "Kẹo" }` thiếu trường `gia`, mà
`SanPham` đòi có cả `ten` VÀ `gia`. `a` và `c` đều đủ cả hai trường,
đúng kiểu từng trường, nên hợp lệ
:::

:::opt
Dòng khai `a` — `SanPham` được định nghĩa trước cả ba biến, nên biến
ĐẦU TIÊN dùng interface này (`a`) luôn bị TypeScript kiểm nghiêm ngặt
hơn hai biến sau
::why
Gần đúng ở việc bạn nghĩ tới THỨ TỰ khai báo có thể ảnh hưởng.

Chỗ lệch: TypeScript kiểm MỌI biến khai kiểu `SanPham` bằng đúng một
luật như nhau — không có biến nào bị kiểm "nghiêm hơn" chỉ vì đứng
trước. `a` có đủ `ten` và `gia`, đúng kiểu cả hai — hợp lệ, đã thử
thật.
::
:::

:::opt
Dòng khai `c` — `gia: 10000` là một số nguyên, còn interface đòi kiểu
`number`, mà TypeScript coi số nguyên và số thực là hai kiểu khác nhau
::why
Gần đúng ở việc bạn nhớ một số ngôn ngữ khác PHÂN BIỆT số nguyên và số
thực (như Python có `int` và `float`).

Chỗ lệch: TypeScript chỉ có DUY NHẤT một kiểu số, `number` — không
phân biệt nguyên hay thực (bài 4 đã chỉ ra điều này). `10000` khớp
hoàn toàn kiểu `number` mà `gia` yêu cầu — dòng khai `c` hợp lệ, đã
thử thật.
::
:::

:::opt
Không dòng nào bị từ chối — interface chỉ là một GỢI Ý cho người đọc mã
nguồn, không phải một luật thật sự được TypeScript kiểm khi biên dịch
::why
Gần đúng ở việc interface đúng là giúp NGƯỜI ĐỌC hiểu hình dạng dữ liệu
nhanh hơn — đó là một lợi ích có thật.

Chỗ lệch: lợi ích đó không thay thế việc KIỂM THẬT. Thiếu một trường
khi tạo giá trị kiểu interface bị trình biên dịch từ chối, đã đo bằng
đúng mã lỗi `TS2741` — dòng khai `b` (thiếu `gia`) bị chặn thật.
::
:::
::::

::::code{#don-hang-du-truong}
Một interface `DonHang` với hai trường: mã đơn (`string`) và số lượng
(`number`). Điền chỗ trống để tạo một đơn hàng ĐỦ cả hai trường.

```typescript title=starter
interface DonHang {
  ma: string;
  soLuong: number;
}

const don: DonHang = { ma: "DH01", soLuong: ___ };
console.log("Mã:", don.ma, "- Số lượng:", don.soLuong);
```

```typescript title=solution
interface DonHang {
  ma: string;
  soLuong: number;
}

const don: DonHang = { ma: "DH01", soLuong: 3 };
console.log("Mã:", don.ma, "- Số lượng:", don.soLuong);
```

```typescript title=test
if (don.soLuong !== 3) {
  throw new Error("don.soLuong phải là 3 — đang là " + don.soLuong);
}
```

:::hints
- kind: attention
  body: 'soLuong khai kiểu number trong interface DonHang — chỗ trống phải là một con số, cụ thể là 3 (đề bài đòi số lượng 3).'
- kind: strategy
  body: 'DonHang đòi đủ hai trường ma và soLuong, đúng kiểu string và number. ma đã có "DH01", chỗ trống là giá trị của soLuong — điền số 3.'
- kind: one-line
  body: 'Chỗ trống là: 3'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Mã: DH01 - Số lượng: 3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một cái tên, cho một hình dạng dữ liệu — dùng lại bao nhiêu nơi cũng
được, và thiếu một trường là TypeScript biết ngay, chỉ đích danh trường
nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn có đủ mọi mảnh ghép của cổng này: kiểu tường minh và suy luận
(bài 1-5), một hàm với chữ ký đầy đủ cả tham số lẫn giá trị trả về (bài
6-10), xử lý an toàn một giá trị có thể vắng mặt (bài 11-14), và giờ là
một giá trị có thể nhiều kiểu, cùng một hình dạng dữ liệu có tên riêng
(bài 15-17).

Ghép TẤT CẢ những mảnh đó vào MỘT chương trình duy nhất trông ra sao?
Bài sau không dạy gì mới — chỉ đòi bạn dùng đúng những gì đã học.
::::

::::checkpoint{mastery=0.8}
::::
