---
id: lap-trinh-ham.adt-pattern-matching.gop-thanh-mot-union-trang-thai-vo-nghia-khong-viet-duoc
title: "Gộp thành MỘT union — trạng thái vô nghĩa KHÔNG VIẾT RA ĐƯỢC nữa"
summary: "Thay `TrangThai` cờ rời rạc (bài trước) bằng `type TrangThai = { kind: \"dangTai\" } | { kind: \"loi\"; thongDiep: string } | { kind: \"thanhCong\"; duLieu: string }` — giờ 'vừa đang tải vừa có lỗi' KHÔNG CÒN VIẾT RA ĐƯỢC nữa: biến thể `dangTai` không có field `thongDiep`/`duLieu` để gán, lỗi bị đẩy từ 'phải NHỚ kiểm tra lúc chạy' sang 'không có KHÔNG GIAN kiểu nào để viết ra nó'."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 21
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.union-makes-illegal-states-unrepresentable]
requires: [ts.boolean-flags-allow-illegal-states]
concepts: [ts.union-makes-illegal-states-unrepresentable]
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
Bài trước: `TrangThai` với ba cờ rời rạc — biên dịch được cả tổ hợp
"vừa đang tải vừa có lỗi vừa có dữ liệu", dù vô nghĩa. Hôm nay: gộp lại
thành MỘT union, xem tổ hợp đó còn VIẾT RA ĐƯỢC không.
::::

::::explain{#gop-thanh-mot-union}
Nhắc lại `TrangThai` bài trước — MỘT `interface` duy nhất, ba field
ĐỘC LẬP với nhau:

```typescript
interface TrangThai {
  dangTai: boolean;
  loi: string | null;
  duLieu: string | null;
}

const voNghia: TrangThai = {
  dangTai: true,
  loi: "mất kết nối",
  duLieu: "42 bản ghi",
};

console.log(voNghia);
```

```text
{"dangTai":true,"loi":"mất kết nối","duLieu":"42 bản ghi"}
```

Đã ĐO THẬT ở bài trước: TypeScript biên dịch object này BÌNH THƯỜNG,
dù về nghiệp vụ nó VÔ NGHĨA — không thể VỪA đang tải VỪA có lỗi VỪA có
dữ liệu cùng lúc. Không có gì trong CẤU TRÚC `interface` này ngăn việc
gán CẢ BA field cùng lúc, vì mỗi field độc lập với hai field còn lại.

Gộp lại thành MỘT `type` union, BA biến thể, MỖI biến thể là một HÌNH
DẠNG RIÊNG:

```typescript
type TrangThai =
  | { kind: "dangTai" }
  | { kind: "loi"; thongDiep: string }
  | { kind: "thanhCong"; duLieu: string };

function hienThi(t: TrangThai): string {
  switch (t.kind) {
    case "dangTai":
      return "đang tải...";
    case "loi":
      return "lỗi: " + t.thongDiep;
    case "thanhCong":
      return "xong: " + t.duLieu;
  }
}

console.log(hienThi({ kind: "dangTai" }));
console.log(hienThi({ kind: "loi", thongDiep: "mất kết nối" }));
console.log(hienThi({ kind: "thanhCong", duLieu: "42 bản ghi" }));
```

```text
đang tải...
lỗi: mất kết nối
xong: 42 bản ghi
```

Biến thể `{ kind: "dangTai" }` CHỈ có đúng MỘT field — `kind`. KHÔNG có
field `thongDiep`, KHÔNG có field `duLieu`. Biến thể `{ kind: "loi";
thongDiep: string }` có `thongDiep` nhưng KHÔNG có `duLieu`. Ba hình
dạng TÁCH BIỆT — mỗi hình dạng chỉ mang ĐÚNG những field nó cần, không
hình dạng nào "thừa" field của hình dạng khác.
::::

::::example{#khong-con-viet-ra-duoc}
Thử viết lại đúng tổ hợp "vừa đang tải vừa có lỗi" mà kiểu CŨ chấp
nhận — kiểu MỚI thì sao?

```typescript title=readonly
type TrangThai =
  | { kind: "dangTai" }
  | { kind: "loi"; thongDiep: string }
  | { kind: "thanhCong"; duLieu: string };

const boiRoi: TrangThai = {
  kind: "dangTai",
  thongDiep: "mất kết nối",
};

console.log(boiRoi);
```

```text title=readonly
(không biên dịch được)

TS2353 (dòng 8, cột 3): Object literal may only specify known
properties, and 'thongDiep' does not exist in type '{ kind: "dangTai"; }'.
```

Đã ĐO THẬT: TypeScript CHẶN NGAY LÚC BIÊN DỊCH, không dòng nào chạy.
Không phải vì `"mất kết nối"` sai GIÁ TRỊ — mà vì biến thể `{ kind:
"dangTai" }` KHÔNG CÓ chỗ nào trong HÌNH DẠNG của nó để field
`thongDiep` tồn tại. Đây chính là "making illegal states
unrepresentable": lỗi không còn là thứ bạn phải NHỚ kiểm tra lúc chạy
(như `if (dangTai && loi !== null)` ở bài trước) — nó là thứ KHÔNG CÓ
KHÔNG GIAN kiểu nào để viết ra, TypeScript từ chối trước khi chương
trình từng tồn tại ở dạng chạy được.

Bỏ field thừa, chỉ giữ đúng field mà biến thể `dangTai` khai:

```typescript title=readonly
type TrangThai =
  | { kind: "dangTai" }
  | { kind: "loi"; thongDiep: string }
  | { kind: "thanhCong"; duLieu: string };

const dangTai: TrangThai = { kind: "dangTai" };

console.log(dangTai);
```

```text title=readonly
{"kind":"dangTai"}
```

Biên dịch sạch — object khớp CHÍNH XÁC một trong ba hình dạng mà
`TrangThai` cho phép, không thừa, không thiếu field nào.
::::

::::predict{#doan-gan-nham-field commitOnce}
```typescript
type TrangThai =
  | { kind: "dangTai" }
  | { kind: "loi"; thongDiep: string }
  | { kind: "thanhCong"; duLieu: string };

const b1: TrangThai = { kind: "loi", thongDiep: "hết pin" };
const b2: TrangThai = { kind: "dangTai", loi: null };

console.log(b1);
console.log(b2);
```

Đoạn này biên dịch được không?

:::opt{correct}
Không — bị chặn NGAY LÚC BIÊN DỊCH tại dòng khai `b2` (TS2353): field
`loi` không tồn tại trong kiểu `{ kind: "dangTai" }`
:::

:::opt
Biên dịch được, in cả `b1` lẫn `b2` bình thường — vì `loi: null` là
cách viết QUEN THUỘC từ `TrangThai` cũ (bài trước), TypeScript vẫn cho
phép field đó xuất hiện trên MỌI biến thể
::why
Gần đúng ở việc bạn nhớ ĐÚNG: field `loi` từng tồn tại thật, trong
`interface TrangThai` CŨ (bài trước) — lúc đó gán `loi: null` cho MỌI
đối tượng `TrangThai` là hợp lệ, vì cả ba field cùng thuộc một
interface DUY NHẤT.

Chỗ lệch: `TrangThai` bây giờ không còn là MỘT interface — nó là UNION
của BA hình dạng RIÊNG. Biến thể `{ kind: "dangTai" }` CHỈ khai đúng
field `kind`, không hề có field `loi`. Gán trực tiếp một object literal
thừa field cho biến khai kiểu union bị TypeScript kiểm "excess
property" và CHẶN — đã đo thật: TS2353 tại dòng khai `b2`.
::
:::

:::opt
Biên dịch được, nhưng lúc CHẠY `b2.loi` sẽ là `undefined` — TypeScript
âm thầm bỏ field lạ không khớp biến thể nào, chỉ giữ lại field khớp
`kind`
::why
Gần đúng ở việc bạn nhận ra `loi` là field KHÔNG khớp với biến thể
`dangTai` — quan sát đó đúng, field này thật sự "lạc chỗ".

Chỗ lệch: TypeScript KHÔNG âm thầm bỏ field lạ khi gán TRỰC TIẾP một
object literal — nó CHẶN LUÔN việc biên dịch (TS2353), không sinh ra
dòng JavaScript nào cả. Không có "lúc chạy" nào để `b2.loi` mang giá
trị `undefined` — chương trình chưa từng tồn tại ở dạng chạy được.
::
:::
::::

::::code{#tao_thanh_cong}
Viết `taoThanhCong(duLieu)` — trả về một giá trị đúng hình dạng biến
thể `"thanhCong"` của `TrangThai`: `kind` đúng nhãn `"thanhCong"`,
`duLieu` đúng giá trị truyền vào.

```typescript title=starter
type TrangThai =
  | { kind: "dangTai" }
  | { kind: "loi"; thongDiep: string }
  | { kind: "thanhCong"; duLieu: string };

function taoThanhCong(duLieu: string): { kind: "thanhCong"; duLieu: string } {
  return { kind: ___, duLieu: duLieu };
}

console.log(taoThanhCong("42 bản ghi").kind);
```

```typescript title=solution
type TrangThai =
  | { kind: "dangTai" }
  | { kind: "loi"; thongDiep: string }
  | { kind: "thanhCong"; duLieu: string };

function taoThanhCong(duLieu: string): { kind: "thanhCong"; duLieu: string } {
  return { kind: "thanhCong", duLieu: duLieu };
}

console.log(taoThanhCong("42 bản ghi").kind);
```

```typescript title=test
const t1 = taoThanhCong("42 bản ghi");
if (t1.kind !== "thanhCong") throw new Error("taoThanhCong phải trả về kind là \"thanhCong\" — đang là " + t1.kind);
if (t1.duLieu !== "42 bản ghi") throw new Error("taoThanhCong phải có duLieu đúng giá trị truyền vào — đang là " + t1.duLieu);
const t2 = taoThanhCong("100 bản ghi");
if (t2.duLieu !== "100 bản ghi") throw new Error("taoThanhCong(\"100 bản ghi\") phải có duLieu là \"100 bản ghi\" — đang là " + t2.duLieu);
if (t2.kind !== "thanhCong") throw new Error("taoThanhCong phải có kind là \"thanhCong\" — đang là " + t2.kind);
```

:::hints
- kind: attention
  body: "Chỗ trống là giá trị của field kind — biến thể \"thanhCong\" chỉ chấp nhận đúng chuỗi \"thanhCong\" (kiểu literal), không phải string chung chung."
- kind: strategy
  body: 'Chữ ký hàm đã khai rõ kiểu trả về là { kind: "thanhCong"; duLieu: string } — object trả về phải khớp ĐÚNG hình dạng đó, viết "thanhCong" trong dấu ngoặc kép.'
- kind: one-line
  body: 'Chỗ trống là: "thanhCong"'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "thanhCong"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không phải bạn giỏi hơn, cũng không phải bạn CẨN THẬN hơn — chỉ là tổ
hợp vô nghĩa giờ không còn KHÔNG GIAN kiểu nào để tồn tại. Đó là
"making illegal states unrepresentable".
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy `TrangThai` union CHẶN được tổ hợp vô nghĩa ngay lúc khai
một GIÁ TRỊ đơn lẻ. Nhưng bài trước, vấn đề thật không chỉ nằm ở việc
khai một giá trị — nó nằm ở một HÀM xử lý `TrangThai`, phải viết `if
(dangTai) ... else if (loi !== null) ... else if (duLieu !== null) ...
else ...` dài dòng, dễ quên nhánh.

Với `TrangThai` union mới, hàm xử lý đó sẽ viết KHÁC thế nào? Bài sau
đối chiếu TRỰC TIẾP hai cách viết, trên CÙNG một bài toán.
::::

::::checkpoint{mastery=0.8}
::::
