---
id: lap-trinh-ham.adt-pattern-matching.viet-mot-adt-long-nhau
title: "Thiết kế một ADT lồng nhau + hàm xử lý"
summary: "Bài code có chấm điểm sống: viết một discriminated union hai biến thể, MỖI biến thể có field RIÊNG (không chỉ kind), rồi một hàm switch truy cập ĐÚNG field của TỪNG biến thể — TypeScript tự chặn nếu lẫn field biến thể này sang biến thể khác, nhờ narrowing."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 18
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.write-nested-adt]
requires: [ts.nested-adt]
concepts: [ts.write-nested-adt]
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
Bài trước: `DonHang` — một sum type hai biến thể, mỗi biến thể lại là một
product type riêng (field của nó, không phải field chung). Hôm nay tới
lượt bạn TỰ TAY dựng một ADT y hệt vậy, rồi viết hàm xử lý cho nó.
::::

::::explain{#thiet-ke-mot-adt-long-nhau}
Thiết kế một ADT lồng nhau đi theo hai bước: (1) liệt kê BIẾN THỂ (đây
là phần "sum" — MỘT trong nhiều khả năng), (2) với MỖI biến thể, liệt
kê field RIÊNG của nó (đây là phần "product" — nhiều field CÙNG LÚC,
chỉ nghiệp vụ biến thể đó cần, không phải mọi biến thể đều có).

```typescript
interface ThongBaoEmail {
  kind: "email";
  nguoiNhan: string;
  tieuDe: string;
}

interface ThongBaoSms {
  kind: "sms";
  soDienThoai: string;
  noiDungNgan: string;
}

type ThongBao = ThongBaoEmail | ThongBaoSms;

function moTa(t: ThongBao): string {
  switch (t.kind) {
    case "email":
      return "Email gửi " + t.nguoiNhan + ": " + t.tieuDe;
    case "sms":
      return "SMS tới " + t.soDienThoai + ": " + t.noiDungNgan;
  }
}

console.log(moTa({ kind: "email", nguoiNhan: "an@vidu.com", tieuDe: "Chào mừng" }));
console.log(moTa({ kind: "sms", soDienThoai: "0901234567", noiDungNgan: "Mã OTP: 1234" }));
```

```text
Email gửi an@vidu.com: Chào mừng
SMS tới 0901234567: Mã OTP: 1234
```

`ThongBaoEmail` có field `nguoiNhan`/`tieuDe`. `ThongBaoSms` KHÔNG có
hai field đó — thay vào đó có `soDienThoai`/`noiDungNgan`. Hai bộ field
này KHÔNG chồng lấn (ngoài `kind` chung cho cả hai). Trong `moTa`, mỗi
nhánh `case` chỉ chạm vào field CỦA CHÍNH biến thể nó xử lý: `case
"email"` đọc `nguoiNhan`/`tieuDe`, `case "sms"` đọc
`soDienThoai`/`noiDungNgan` — không nhánh nào đọc field của nhánh kia.

Đây CHÍNH LÀ "switch trên discriminant" đã học từ bài 4 — chỉ khác:
mỗi biến thể giờ mang NHIỀU field riêng (product), không chỉ một field
đơn lẻ như `Vuong.canh`/`Tron.banKinh` ở cụm 1. Nếu lỡ viết
`t.soDienThoai` bên trong `case "email"`, TypeScript CHẶN NGAY (mã lỗi
TS2339 — "Property 'soDienThoai' does not exist on type
'ThongBaoEmail'") — vì bên trong nhánh đó, so khớp `t.kind === "email"`
đã NARROW `t` xuống đúng `ThongBaoEmail`, kiểu không hề có field đó.
::::

::::example{#mang-thong-bao-tron-lan}
Một MẢNG trộn lẫn cả hai biến thể — hàm khác, đọc field khác
(`.length` của field riêng từng biến thể), nhưng nguyên tắc y hệt: mỗi
nhánh chỉ chạm field của biến thể nó xử lý.

```typescript title=readonly
interface ThongBaoEmail {
  kind: "email";
  nguoiNhan: string;
  tieuDe: string;
}

interface ThongBaoSms {
  kind: "sms";
  soDienThoai: string;
  noiDungNgan: string;
}

type ThongBao = ThongBaoEmail | ThongBaoSms;

function doDaiNoiDung(t: ThongBao): number {
  switch (t.kind) {
    case "email":
      return t.tieuDe.length;
    case "sms":
      return t.noiDungNgan.length;
  }
}

const ds: ThongBao[] = [
  { kind: "email", nguoiNhan: "an@vidu.com", tieuDe: "Chao mung" },
  { kind: "sms", soDienThoai: "0901234567", noiDungNgan: "OTP" },
];

for (const t of ds) {
  console.log(t.kind + ": " + doDaiNoiDung(t));
}
```

```text title=readonly
email: 9
sms: 3
```

`doDaiNoiDung` không đọc `nguoiNhan` hay `soDienThoai` — nó chỉ cần
ĐÚNG MỘT field mỗi biến thể (`tieuDe` hoặc `noiDungNgan`), và biết
chọn field nào nhờ `t.kind` đã narrow. Field còn lại của mỗi interface
(`nguoiNhan`, `soDienThoai`) tồn tại trong kiểu nhưng hàm này không cần
tới — một ADT lồng nhau không bắt buộc MỌI hàm phải dùng HẾT mọi field.
::::

::::predict{#doan-mo-ta-don-thu commitOnce}
```typescript
interface DonThuChoXuLy {
  kind: "choXuLy";
  ma: string;
  soLuong: number;
}

interface DonThuDaGiao {
  kind: "daGiao";
  ma: string;
  nguoiNhan: string;
  ngayGiao: string;
}

type DonThu = DonThuChoXuLy | DonThuDaGiao;

function moTaDon(d: DonThu): string {
  switch (d.kind) {
    case "choXuLy":
      return "Đơn " + d.ma + " cho " + d.soLuong + " món, đang chờ xử lý";
    case "daGiao":
      return "Đơn " + d.ma + " đã giao cho " + d.nguoiNhan + " ngày " + d.ngayGiao;
  }
}

console.log(moTaDon({ kind: "choXuLy", ma: "DH-9", soLuong: 3 }));
console.log(moTaDon({ kind: "daGiao", ma: "DH-2", nguoiNhan: "Bình", ngayGiao: "10/09" }));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`Đơn DH-9 cho 3 món, đang chờ xử lý` rồi `Đơn DH-2 đã giao cho Bình ngày 10/09`
:::

:::opt
Máy báo lỗi biên dịch ở nhánh `case "choXuLy"` — vì bên trong nhánh đó
`d` vẫn mang kiểu `DonThu` (union cả hai interface), không có field
`soLuong`
::why
Gần đúng ở việc bạn nhớ `d` được khai kiểu `DonThu` — một UNION của hai
interface, và đúng là kiểu UNION đó (nhìn tổng quát) không tự nhiên
đảm bảo có field `soLuong`.

Chỗ lệch: bên trong nhánh `case "choXuLy"`, phép so khớp `switch
(d.kind)` đã cho TypeScript đủ bằng chứng để NARROW `d` xuống đúng
`DonThuChoXuLy` — bên trong nhánh đó `d` KHÔNG còn là union nữa, field
`soLuong` (interface đó CÓ khai) truy cập được bình thường, không có
lỗi biên dịch nào.
::
:::

:::opt
Ném lỗi lúc CHẠY ở dòng gọi thứ nhất — vì `DonThuChoXuLy` không có
field `nguoiNhan`/`ngayGiao` mà `DonThuDaGiao` cần
::why
Gần đúng ở việc bạn để ý đúng một sự thật: `DonThuChoXuLy` THẬT SỰ
không khai field `nguoiNhan` hay `ngayGiao` — quan sát về cấu trúc đó
chính xác.

Chỗ lệch: nhánh `case "choXuLy"` (nhánh xử lý object thứ nhất) KHÔNG
ĐỌC field `nguoiNhan` hay `ngayGiao` nào cả — mỗi nhánh `switch` chỉ
truy cập field CỦA CHÍNH biến thể nó xử lý. Không có gì "thiếu" lúc
chạy, vì TypeScript đã CHẶN từ lúc BIÊN DỊCH mọi khả năng viết nhầm
field của biến thể này sang nhánh của biến thể kia.
::
:::
::::

::::code{#mo_ta_xe}
Viết `moTaXe(p)` cho đủ. `OTo` có field RIÊNG `soCho` (số chỗ ngồi) mà
`XeMay` KHÔNG có — trong nhánh `case "oTo"`, TypeScript đã tự thu hẹp
(narrow) `p` thành `OTo`, nên `p.soCho` truy cập được ngay tại đó.

```typescript title=starter
interface XeMay {
  kind: "xeMay";
  bienSo: string;
}

interface OTo {
  kind: "oTo";
  bienSo: string;
  soCho: number;
}

type PhuongTien = XeMay | OTo;

function moTaXe(p: PhuongTien): string {
  switch (p.kind) {
    case "xeMay":
      return "Xe máy biển số " + p.bienSo;
    case "oTo":
      return "Ô tô biển số " + p.bienSo + ", " + ___ + " chỗ";
  }
}

console.log(moTaXe({ kind: "xeMay", bienSo: "51A-123" }));
console.log(moTaXe({ kind: "oTo", bienSo: "51A-456", soCho: 4 }));
```

```typescript title=solution
interface XeMay {
  kind: "xeMay";
  bienSo: string;
}

interface OTo {
  kind: "oTo";
  bienSo: string;
  soCho: number;
}

type PhuongTien = XeMay | OTo;

function moTaXe(p: PhuongTien): string {
  switch (p.kind) {
    case "xeMay":
      return "Xe máy biển số " + p.bienSo;
    case "oTo":
      return "Ô tô biển số " + p.bienSo + ", " + p.soCho + " chỗ";
  }
}

console.log(moTaXe({ kind: "xeMay", bienSo: "51A-123" }));
console.log(moTaXe({ kind: "oTo", bienSo: "51A-456", soCho: 4 }));
```

```typescript title=test
const x1 = moTaXe({ kind: "xeMay", bienSo: "29B-777" });
if (x1 !== "Xe máy biển số 29B-777") throw new Error("moTaXe phải trả về đúng mô tả xe máy — đang là " + x1);

const x2 = moTaXe({ kind: "oTo", bienSo: "30A-999", soCho: 7 });
if (x2 !== "Ô tô biển số 30A-999, 7 chỗ") throw new Error("moTaXe phải trả về đúng mô tả ô tô, gồm cả số chỗ — đang là " + x2);

const x3 = moTaXe({ kind: "oTo", bienSo: "43C-111", soCho: 4 });
if (x3 !== "Ô tô biển số 43C-111, 4 chỗ") throw new Error("moTaXe với soCho khác cũng phải in đúng số chỗ đó — đang là " + x3);
```

:::hints
- kind: attention
  body: "Chỗ trống nằm trong nhánh case \"oTo\" — cần in ra field soCho, field RIÊNG của OTo mà XeMay không có."
- kind: strategy
  body: "OTo khai kind: \"oTo\"; bienSo: string; soCho: number — trong nhánh case \"oTo\", p đã được TypeScript tự narrow thành OTo, nên p.soCho hợp lệ. Viết đúng field đó vào chỗ trống."
- kind: one-line
  body: "Chỗ trống là: p.soCho"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Ô tô biển số 51A-456, 4 chỗ"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một discriminated union với field RIÊNG cho từng biến thể, một hàm
switch chỉ chạm ĐÚNG field mỗi nhánh — tự tay dựng được rồi, không chỉ
đọc ví dụ có sẵn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`ThongBao`, `DonThu`, `PhuongTien` — ba ADT lồng nhau khác nhau, cùng
một khuôn: liệt kê biến thể (sum), rồi liệt kê field riêng mỗi biến
thể (product). Nếu gặp một mô tả nghiệp vụ HOÀN TOÀN MỚI — chưa từng
thấy dạng nào ở trên — bạn có tự nhận ra được đâu là "sum" (biến thể
nào) và đâu là "product" (field nào thuộc biến thể nào) không, không
cần ai gợi ý sẵn cấu trúc?

Bài sau trả lời — không khái niệm mới, chỉ đo lại từ đầu.
::::

::::checkpoint{mastery=0.8}
::::
