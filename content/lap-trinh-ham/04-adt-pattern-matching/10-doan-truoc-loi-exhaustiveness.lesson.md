---
id: lap-trinh-ham.adt-pattern-matching.doan-truoc-loi-exhaustiveness
title: "Dự đoán: thêm biến thể có làm lộ lỗi cũ không?"
summary: "Một `switch` đã có `assertNever` xử lý ĐỦ hai biến thể — thêm MỘT biến thể thứ ba vào union, KHÔNG sửa `switch`. Dòng `chuaXuLy(...)` có báo lỗi biên dịch không? Bẫy: nghĩ chỉ báo lỗi khi CHẠY hàm với biến thể mới — sự thật: lỗi hiện NGAY lúc biên dịch, trước khi bất kỳ dòng nào chạy, đúng cơ chế bài trước."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [ts.predict-exhaustiveness]
requires: [ts.exhaustiveness-catches-new-variant]
concepts: [ts.predict-exhaustiveness]
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
Bài trước: thêm một biến thể mới vào union đã có `assertNever` bảo vệ
— TypeScript báo lỗi NGAY, không cần chạy chương trình. Hôm nay không
có khái niệm mới — chỉ kiểm bạn có THẬT hiểu đúng chỗ đó, hay chỉ nhớ
đúng VÍ DỤ cũ.
::::

::::explain{#nhac-lai-kiem-tinh-luc-bien-dich}
```typescript
interface TheTinDung {
  kind: "the";
  soThe: string;
}

interface TienMat {
  kind: "tienMat";
  soTien: number;
}

type PhuongThuc = TheTinDung | TienMat;

function chuaXuLy(x: never): never {
  throw new Error("Chưa xử lý biến thể: " + JSON.stringify(x));
}

function moTaPhi(pt: PhuongThuc): string {
  switch (pt.kind) {
    case "the":
      return "phí thẻ cho số " + pt.soThe;
    case "tienMat":
      return "phí tiền mặt " + pt.soTien;
    default:
      return chuaXuLy(pt);
  }
}

console.log(moTaPhi({ kind: "the", soThe: "9999" }));
console.log(moTaPhi({ kind: "tienMat", soTien: 50000 }));
```

```text
phí thẻ cho số 9999
phí tiền mặt 50000
```

Mẫu đã học ở bài trước: `switch` trên `pt.kind` xử lý ĐỦ hai biến thể
(`the`, `tienMat`), nhánh `default` gọi `chuaXuLy(pt)` — cả hai dòng
`console.log` chạy đúng, không lỗi gì.

Điều bài trước ĐÃ ĐO: khi TypeScript kiểm `switch` này, nó KHÔNG đợi
chương trình CHẠY để biết còn sót biến thể nào — nó đọc TOÀN BỘ khai
báo `type PhuongThuc` VÀ toàn bộ thân hàm `moTaPhi` CÙNG LÚC, ngay lúc
biên dịch. Ở mỗi `case`, kiểu của `pt` bị THU HẸP dần (`case "the"`
loại `TheTinDung`, `case "tienMat"` loại tiếp `TienMat`) — tới nhánh
`default`, nếu KHÔNG còn biến thể nào chưa bị loại, kiểu của `pt` còn
lại đúng là `never`, và `chuaXuLy(pt)` biên dịch được.

Vậy nếu SAU ĐÓ có ai mở rộng `type PhuongThuc` — thêm một biến thể MỚI
mà KHÔNG sửa `switch` — điều gì xảy ra với dòng `chuaXuLy(pt)`? Đây
KHÔNG phải một câu hỏi runtime ("chạy thử xem có crash không") — nó là
một câu hỏi trả lời được chỉ bằng cách ĐỌC kiểu.
::::

::::example{#them-bien-the-khong-sua-switch}
Thêm `ViDienTu` vào `PhuongThuc` — `switch` trong `moTaPhi` giữ
NGUYÊN, không đụng gì:

```typescript title=readonly
interface TheTinDung {
  kind: "the";
  soThe: string;
}

interface TienMat {
  kind: "tienMat";
  soTien: number;
}

interface ViDienTu {
  kind: "viDienTu";
  email: string;
}

type PhuongThuc = TheTinDung | TienMat | ViDienTu;

function chuaXuLy(x: never): never {
  throw new Error("Chưa xử lý biến thể: " + JSON.stringify(x));
}

function moTaPhi(pt: PhuongThuc): string {
  switch (pt.kind) {
    case "the":
      return "phí thẻ cho số " + pt.soThe;
    case "tienMat":
      return "phí tiền mặt " + pt.soTien;
    default:
      return chuaXuLy(pt);
  }
}

console.log(moTaPhi({ kind: "the", soThe: "9999" }));
```

```text title=readonly
(không in ra gì cả)

TS2345 (dòng 29, cột 23): Argument of type 'ViDienTu' is not
assignable to parameter of type 'never'.
```

Để ý: `console.log(moTaPhi({ kind: "the", soThe: "9999" }))` ở dòng
cuối dùng một `TheTinDung` HOÀN TOÀN hợp lệ — không đụng gì tới
`ViDienTu`. Nhưng KHÔNG dòng nào được in ra. Đúng điều T4.0a's
`ts.compile-time-check` đã dạy: một chương trình có LỖI KIỂU ở BẤT KỲ
ĐÂU thì KHÔNG dòng nào được phép chạy, kể cả những dòng hoàn toàn đúng
— TypeScript từ chối sinh mã JavaScript cho CẢ file, không phải chỉ
riêng dòng lỗi.

Lỗi `TS2345` xảy ra vì: sau khi `case "the"` và `case "tienMat"` đã
loại hai biến thể, kiểu của `pt` tại `default` KHÔNG còn là `never`
nữa — nó là `ViDienTu` (biến thể còn sót lại, chưa `case` nào xử lý).
Truyền một `ViDienTu` vào `chuaXuLy` (đòi `never`) vi phạm chữ ký hàm —
bị chặn NGAY lúc biên dịch, không cần ai gọi `moTaPhi` với một
`ViDienTu` thật.
::::

::::predict{#doan-truoc-loi-bien-dich commitOnce}
```typescript
interface Khach {
  kind: "khach";
}

interface ThanhVien {
  kind: "thanhVien";
  diem: number;
}

interface QuanTri {
  kind: "quanTri";
  capDo: number;
}

type NguoiDung = Khach | ThanhVien | QuanTri;

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function chao(nd: NguoiDung): string {
  switch (nd.kind) {
    case "khach":
      return "Chào khách";
    case "thanhVien":
      return "Chào thành viên, bạn có " + nd.diem + " điểm";
    default:
      return chuaXuLy(nd);
  }
}

console.log(chao({ kind: "khach" }));
```

Biến thể `QuanTri` VỪA được thêm vào union `NguoiDung` — nhưng
`switch` trong `chao` VẪN CHỈ xử lý hai `case` cũ (`khach`,
`thanhVien`), không đổi gì thêm.

Việc thêm `QuanTri` như trên có làm dòng `chuaXuLy(nd)` báo lỗi biên
dịch không?

:::opt{correct}
Có — TypeScript báo lỗi NGAY lúc biên dịch (mã `TS2345`: `'QuanTri' is
not assignable to parameter of type 'never'`), không cần chạy
`chao(...)` với một `QuanTri` thật để thấy lỗi này
:::

:::opt
Không — mã vẫn biên dịch bình thường, TypeScript chỉ báo lỗi LÚC CHẠY
nếu có ai đó thực sự gọi `chao({ kind: "quanTri", capDo: 1 })`
::why
Gần đúng ở việc bạn nhận ra CÓ vấn đề thật liên quan tới `QuanTri`
chưa được `switch` xử lý — quan sát đó đúng.

Chỗ lệch: việc kiểm tra ở đây là TĨNH (static) — TypeScript phân tích
TOÀN BỘ thân hàm `chao` ngay lúc biên dịch, xem `nd` tại nhánh
`default` còn CÓ THỂ là những kiểu nào (ở đây là `QuanTri`, vì hai
`case` trước chỉ loại được `khach`/`thanhVien`). Việc chương trình CÓ
ĐƯỢC GỌI hay không, gọi với giá trị nào, không liên quan gì tới việc
trình biên dịch có chấp nhận đoạn mã hay không — lỗi hiện ra dù dòng
`console.log(chao({ kind: "khach" }))` chỉ dùng một `Khach` hợp lệ,
không đụng tới `QuanTri` ở đâu cả.
::
:::

:::opt
Không — `never` là kiểu đặc biệt nghĩa là "chấp nhận MỌI kiểu", nên
`chuaXuLy(nd)` luôn nhận được bất kỳ giá trị nào truyền vào, kể cả
`QuanTri`
::why
Gần đúng ở việc bạn nhớ đúng `chuaXuLy` khai tham số kiểu `never` —
quan sát về CHỮ KÝ hàm đúng.

Chỗ lệch: `never` mang nghĩa NGƯỢC LẠI hoàn toàn — nó là kiểu "KHÔNG
CÓ giá trị nào thuộc về nó", hẹp nhất có thể (kiểu "chấp nhận mọi thứ"
là `any`, một kiểu khác hẳn). Chính vì `never` nghiêm ngặt tới mức
không nhận gì, nên khi `switch` còn SÓT một biến thể (ở đây `QuanTri`)
tại nhánh `default`, kiểu của `nd` ở đó KHÔNG thu hẹp được về `never`
— TypeScript thấy `QuanTri` không khớp `never`, báo `TS2345` ngay.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không phải nhớ đúng DÒNG MÃ nào — mà nhớ đúng THỜI ĐIỂM: lỗi hiện ra
LÚC BIÊN DỊCH, trước khi bất kỳ dòng nào chạy. Cơ chế này áp dụng cho
MỌI union có `assertNever` bảo vệ, không riêng gì `Hinh` hay
`PhuongThuc` — thêm biến thể, quên cập nhật `switch`, trình biên dịch
tự lộ ra chỗ thiếu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cơ chế này chỉ có tác dụng NẾU nhánh `default` gọi ĐÚNG `chuaXuLy(...)`.
Nếu `default` làm việc KHÁC — ví dụ trả về một giá trị mặc định nào đó
thay vì gọi `chuaXuLy` — thêm biến thể mới có CÒN báo lỗi biên dịch
không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
