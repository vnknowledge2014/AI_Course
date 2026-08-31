---
id: lap-trinh-ham.cong-typescript.boss-cong-typescript
title: "BOSS — Khép cổng TypeScript"
summary: "Ghép trọn cổng: kiểu tường minh, hàm có chữ ký đầy đủ, xử lý an toàn giá trị có thể vắng mặt, union type và interface — trong một chương trình duy nhất. Không dạy khái niệm mới, chỉ đòi ghép lại đúng những gì mười bảy bài trước đã dựng."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 18
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.gate-boss]
requires: [ts.interface]
concepts: [ts.gate-boss]
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
Mười bảy bài để tới đây. Giờ ghép hết lại: một chương trình duy nhất,
dùng đủ những gì cổng này dạy.
::::

::::explain{#bon-manh-ghep}
Suốt mười bảy bài, cổng này dựng đúng BỐN mảnh ghép, tất cả cùng phục
vụ một Ý duy nhất — kiểu có thể được KIỂM TRƯỚC KHI CHẠY:

1. **Kiểu là một lời hứa, kiểm trước khi chạy** — viết tường minh
   (`: number`) hay để TypeScript tự suy luận, lời hứa đó vẫn bị kiểm
   NGHIÊM NGẶT NHƯ NHAU, và một dòng sai kiểu khiến CẢ CHƯƠNG TRÌNH
   không được sinh ra, không riêng dòng đó (bài 1-5).
2. **Hàm có chữ ký hai chiều** — mỗi tham số một lời hứa, giá trị trả
   về một lời hứa khác; gọi sai kiểu, sai số lượng đối số đều bị chặn
   trước khi hàm kịp chạy (bài 6-10).
3. **Giá trị có thể vắng mặt phải kiểm trước khi dùng** — một phần tử
   mảng CÓ THỂ không tồn tại (`undefined`), một giá trị CỐ Ý không có
   gì (`null`) — TypeScript buộc kiểm tra trước, và trong nhánh đã
   kiểm, nó THU HẸP kiểu để cho dùng an toàn (bài 11-14).
4. **Một giá trị nhiều kiểu, một hình dạng có tên** — union type
   (`string | number`) đòi thu hẹp trước khi gọi phương thức riêng của
   một kiểu; interface đặt tên cho một hình dạng object, dùng lại nhiều
   nơi, thiếu một trường là bị từ chối ngay (bài 15-17).

Bốn mảnh đó là toàn bộ nguyên liệu để viết một chương trình TypeScript
nhỏ, an toàn kiểu từ đầu tới cuối. Bài này ghép chúng vào một chương
trình duy nhất.
::::

::::example{#chuong-trinh-ghep-du-bon-manh}
Một giỏ hàng nhỏ, dùng đủ bốn mảnh ghép:

```typescript title=readonly
interface SanPham {
  ten: string;
  gia: number;
}

function tinhTongGia(danhSach: SanPham[]): number {
  let tong = 0;
  for (const sp of danhSach) {
    tong += sp.gia;
  }
  return tong;
}

function laySanPhamDauTien(danhSach: SanPham[]): SanPham | undefined {
  return danhSach[0];
}

function moTaMa(ma: string | number): string {
  if (typeof ma === "string") {
    return "Mã chữ: " + ma;
  }
  return "Mã số: " + ma;
}

const gioHang: SanPham[] = [
  { ten: "Bánh", gia: 20000 },
  { ten: "Kẹo", gia: 15000 },
];

console.log("Tổng:", tinhTongGia(gioHang));

const dau = laySanPhamDauTien(gioHang);
if (dau !== undefined) {
  console.log("Đầu tiên:", dau.ten);
}

console.log(moTaMa("DH01"));
console.log(moTaMa(202));
```

```text title=readonly
Tổng: 35000
Đầu tiên: Bánh
Mã chữ: DH01
Mã số: 202
```

Đọc ra bốn mảnh: `interface SanPham` đặt tên cho hình dạng (mảnh 4).
`tinhTongGia` có chữ ký đầy đủ, tham số `SanPham[]`, trả về `number`
(mảnh 2). `laySanPhamDauTien` trả về `SanPham | undefined` — mảng
`danhSach` CÓ THỂ rỗng, nên phần tử đầu CÓ THỂ không tồn tại; kiểm
`dau !== undefined` trước khi đọc `dau.ten` (mảnh 3). `moTaMa` nhận
`string | number`, thu hẹp bằng `typeof` trước khi trả về chuỗi mô tả
khác nhau cho từng kiểu (mảnh 4). Và toàn bộ chương trình này chỉ chạy
được vì MỌI dòng đều giữ đúng lời hứa kiểu của nó (mảnh 1) — sai một
chỗ, không dòng nào được phép chạy.

Nếu bỏ bước kiểm `dau !== undefined` trong đúng chương trình trên, đọc
thẳng `dau.ten`:

```typescript title=readonly
interface SanPham {
  ten: string;
  gia: number;
}

function tinhTongGia(danhSach: SanPham[]): number {
  let tong = 0;
  for (const sp of danhSach) {
    tong += sp.gia;
  }
  return tong;
}

function laySanPhamDauTien(danhSach: SanPham[]): SanPham | undefined {
  return danhSach[0];
}

function moTaMa(ma: string | number): string {
  if (typeof ma === "string") {
    return "Mã chữ: " + ma;
  }
  return "Mã số: " + ma;
}

const gioHang: SanPham[] = [
  { ten: "Bánh", gia: 20000 },
  { ten: "Kẹo", gia: 15000 },
];

console.log("Tổng:", tinhTongGia(gioHang));

const dau = laySanPhamDauTien(gioHang);
console.log(dau.ten);

console.log(moTaMa("DH01"));
console.log(moTaMa(202));
```

```text title=readonly
(không biên dịch được)

TS18048 (dòng 33, cột 13): 'dau' is possibly 'undefined'.
```

Đúng mã lỗi của mảnh 3 — `dau` CÓ THỂ là `undefined`, và TypeScript từ
chối cho đọc `.ten` khi chưa kiểm.
::::

::::predict{#manh-nao-giai-thich commitOnce}
Byte viết đoạn mã sau, KHÔNG chạy thử — nó bị TypeScript từ chối:

```typescript
interface SanPham {
  ten: string;
  gia: number;
}

function laySanPhamDauTien(danhSach: SanPham[]): SanPham | undefined {
  return danhSach[0];
}

const gioHang: SanPham[] = [{ ten: "Bánh", gia: 20000 }];
const dau = laySanPhamDauTien(gioHang);
console.log(dau.ten);
```

**Trước khi đọc đáp án**, mảnh nào trong bốn mảnh của bài này giải
thích ĐÚNG NHẤT lý do bị từ chối?

:::opt{correct}
Mảnh 3 — `laySanPhamDauTien` khai trả về `SanPham | undefined`, nghĩa
là kết quả CÓ THỂ vắng mặt; đọc `dau.ten` mà chưa kiểm `dau !==
undefined` trước là dùng một giá trị có thể vắng mặt mà không kiểm tra
:::

:::opt
Mảnh 4 — `interface SanPham` định nghĩa thiếu trường nào đó, nên
`dau.ten` không tồn tại trên kiểu `SanPham`
::why
Gần đúng ở việc bạn nghi ngờ đúng "chỗ" — lỗi liên quan tới `dau.ten`.

Chỗ lệch: `interface SanPham` định nghĩa ĐỦ hai trường `ten` và `gia`,
và `.ten` HOÀN TOÀN tồn tại trên kiểu `SanPham`. Vấn đề không phải
`SanPham` thiếu gì — vấn đề là `dau` có kiểu `SanPham | undefined`,
không phải `SanPham` đơn thuần, nên trước khi đọc `.ten` phải loại bỏ
khả năng `undefined` trước (mảnh 3), không phải sửa interface.
::
:::

:::opt
Mảnh 2 — hàm `laySanPhamDauTien` khai chữ ký sai, vì một hàm nhận
`SanPham[]` không được phép trả về `SanPham | undefined`, chỉ được trả
đúng một kiểu duy nhất
::why
Gần đúng ở việc bạn để ý tới CHỮ KÝ hàm — đúng, chữ ký hàm là một khái
niệm quan trọng của cổng này (bài 6-10).

Chỗ lệch: một hàm hoàn toàn được phép khai kiểu trả về là một union,
như `SanPham | undefined` — đây chính là cách chính xác để nói "kết quả
CÓ THỂ vắng mặt". Chữ ký của `laySanPhamDauTien` không sai; lỗi nằm ở
nơi GỌI nó, đọc `.ten` mà chưa kiểm `undefined`.
::
:::

:::opt
Mảnh 1 — `dau` được khai bằng `const` mà không viết kiểu tường minh,
nên TypeScript không suy luận được kiểu cho nó, dẫn tới lỗi khi đọc
`.ten`
::why
Gần đúng ở việc bạn nhớ đúng: TypeScript SUY LUẬN kiểu khi không viết
tường minh (bài 4) — điều đó có thật.

Chỗ lệch: suy luận ở đây hoàn toàn THÀNH CÔNG — TypeScript suy ra đúng
kiểu `SanPham | undefined` cho `dau`, từ kiểu trả về đã khai của
`laySanPhamDauTien`. Vấn đề không phải suy luận thất bại, mà là kiểu
suy luận ĐÚNG ấy bao gồm khả năng `undefined`, và `.ten` bị đọc mà chưa
kiểm khả năng đó.
::
:::
::::

::::code{#giot-hang-day-du}
Một tập điểm 2D, có hình dạng riêng, và một hàm mô tả nhãn có thể là
chữ hoặc số. Điền chỗ trống cuối cùng để hoàn thành chương trình dùng
đủ bốn mảnh ghép của cổng này.

```typescript title=starter
interface Diem {
  x: number;
  y: number;
}

function tongX(danhSach: Diem[]): number {
  let tong = 0;
  for (const p of danhSach) {
    tong += p.x;
  }
  return tong;
}

function diemDauTien(danhSach: Diem[]): Diem | undefined {
  return danhSach[0];
}

function moTaNhan(nhan: string | number): string {
  if (typeof nhan === "string") {
    return "Nhãn chữ: " + nhan;
  }
  return "Nhãn số: " + nhan;
}

const cacDiem: Diem[] = [
  { x: 3, y: 4 },
  { x: 1, y: 1 },
];

console.log("Tổng x:", tongX(cacDiem));

const dau = diemDauTien(cacDiem);
if (dau !== undefined) {
  console.log("Điểm đầu:", dau.x, dau.y);
}

const ketQua = moTaNhan(___);        // gọi với nhãn số 7
console.log(ketQua);
```

```typescript title=solution
interface Diem {
  x: number;
  y: number;
}

function tongX(danhSach: Diem[]): number {
  let tong = 0;
  for (const p of danhSach) {
    tong += p.x;
  }
  return tong;
}

function diemDauTien(danhSach: Diem[]): Diem | undefined {
  return danhSach[0];
}

function moTaNhan(nhan: string | number): string {
  if (typeof nhan === "string") {
    return "Nhãn chữ: " + nhan;
  }
  return "Nhãn số: " + nhan;
}

const cacDiem: Diem[] = [
  { x: 3, y: 4 },
  { x: 1, y: 1 },
];

console.log("Tổng x:", tongX(cacDiem));

const dau = diemDauTien(cacDiem);
if (dau !== undefined) {
  console.log("Điểm đầu:", dau.x, dau.y);
}

const ketQua = moTaNhan(7);
console.log(ketQua);
```

```typescript title=test
if (tongX(cacDiem) !== 4) {
  throw new Error("tongX(cacDiem) phải là 4 (3 + 1) — đang là " + tongX(cacDiem));
}
if (ketQua !== "Nhãn số: 7") {
  throw new Error("ketQua phải là \"Nhãn số: 7\" (gọi moTaNhan với nhãn số 7) — đang là " + ketQua);
}
```

:::hints
- kind: attention
  body: 'moTaNhan nhận string | number. Đề bài đòi gọi nó với NHÃN SỐ 7 — chỗ trống là một number, không phải một chuỗi.'
- kind: strategy
  body: 'Bình luận cuối dòng ghi rõ "gọi với nhãn số 7" — moTaNhan(7) đi vào nhánh number của hàm, trả về "Nhãn số: 7". Điền đúng số 7, không có dấu ngoặc kép.'
- kind: one-line
  body: 'Chỗ trống là: 7'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "Nhãn số: 7"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn mảnh ghép, một chương trình. Kiểu tường minh, chữ ký hàm đầy đủ,
kiểm trước khi dùng giá trị có thể vắng mặt, union type và interface —
bạn vừa dùng cả bốn cùng lúc, không sai một dòng.
::::

::::reflect{#nghi-lai}
Một câu hỏi cuối, khép lại cả cổng này.

Mười tám bài, đi từ một dấu hai chấm nhỏ sau tên biến, tới một chương
trình mà MỌI lời hứa kiểu đều được kiểm TRƯỚC KHI CHẠY — không phải lúc
chạy tới dòng đó mới biết sai, như 452 bài Python trước đây vẫn luôn
vậy.

Track sau quay lại Python. Nhưng bạn mang theo một câu hỏi mới, một câu
hỏi Python không bao giờ tự trả lời được: kiểu của giá trị này — kiểu
NÓ THẬT SỰ ĐANG LÀ — có được kiểm trước khi chương trình chạy hay
không? Hay phải đợi tới đúng dòng đó chạy, mới biết?

Đó là câu hỏi cổng này để lại.
::::

::::checkpoint{mastery=0.85}
::::
