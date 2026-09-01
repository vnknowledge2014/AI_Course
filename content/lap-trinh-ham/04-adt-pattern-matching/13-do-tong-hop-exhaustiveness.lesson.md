---
id: lap-trinh-ham.adt-pattern-matching.do-tong-hop-exhaustiveness
title: "Đo tổng hợp: exhaustiveness checking"
summary: "Bài chốt cụm 2: cho một union BỐN biến thể, tự viết hàm switch đầy đủ có assertNever — rồi xem thêm biến thể thứ năm làm lộ lỗi biên dịch ở ĐÚNG chỗ nào nếu quên cập nhật. Không khái niệm mới, đo khả năng tự dựng kỷ luật exhaustiveness từ đầu."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 13
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.review-exhaustiveness]
requires: [ts.write-exhaustive-match]
concepts: [ts.review-exhaustiveness]
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
Bài trước, bạn tự tay viết một hàm `switch` đầy đủ, có `chuaXuLy` đứng
gác ở nhánh `default` — cho một union BA biến thể. Hôm nay: BỐN biến
thể, và thêm một câu hỏi: nếu SAU NÀY ai đó mở rộng union mà QUÊN cập
nhật `switch`, chuyện gì xảy ra?
::::

::::explain{#switch-day-du-bon-bien-the}
```typescript
interface ChoXuLy {
  kind: "choXuLy";
  ma: string;
}

interface DangGiao {
  kind: "dangGiao";
  ma: string;
  maVanDon: string;
}

interface DaGiao {
  kind: "daGiao";
  ma: string;
  ngayGiao: string;
}

interface DaHuy {
  kind: "daHuy";
  ma: string;
  lyDoHuy: string;
}

type TrangThaiDonHang = ChoXuLy | DangGiao | DaGiao | DaHuy;

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý biến thể: " + JSON.stringify(x));
}

function moTa(d: TrangThaiDonHang): string {
  switch (d.kind) {
    case "choXuLy":
      return "đơn " + d.ma + " đang chờ xử lý";
    case "dangGiao":
      return "đơn " + d.ma + " đang giao, vận đơn " + d.maVanDon;
    case "daGiao":
      return "đơn " + d.ma + " đã giao ngày " + d.ngayGiao;
    case "daHuy":
      return "đơn " + d.ma + " đã huỷ, lý do: " + d.lyDoHuy;
    default:
      return chuaXuLy(d);
  }
}

console.log(moTa({ kind: "choXuLy", ma: "DH-1" }));
console.log(moTa({ kind: "dangGiao", ma: "DH-2", maVanDon: "VD-99" }));
console.log(moTa({ kind: "daGiao", ma: "DH-3", ngayGiao: "2026-08-30" }));
console.log(moTa({ kind: "daHuy", ma: "DH-4", lyDoHuy: "hết hàng" }));
```

```text
đơn DH-1 đang chờ xử lý
đơn DH-2 đang giao, vận đơn VD-99
đơn DH-3 đã giao ngày 2026-08-30
đơn DH-4 đã huỷ, lý do: hết hàng
```

`TrangThaiDonHang` có BỐN biến thể — nhiều hơn ví dụ ba biến thể quen
thuộc từ các bài trước, nhưng cách viết KHÔNG đổi. MỖI `case` narrow
`d` xuống đúng interface tương ứng (bài 3–4 đã học), `default` gọi
`chuaXuLy(d)` (bài 8 đã học). Sau khi bốn `case` đã xử lý HẾT bốn biến
thể, TypeScript thu hẹp kiểu của `d` tại nhánh `default` xuống còn
`never` — kiểu "không còn khả năng nào" — khớp ĐÚNG tham số mà
`chuaXuLy` đòi hỏi, nên `chuaXuLy(d)` biên dịch được.

Không có khái niệm MỚI ở đây. Bài này chỉ SCALE UP đúng kỷ luật đã học
(bài 8 dựng `assertNever`, bài 12 tự viết một hàm đầy đủ) lên một union
LỚN hơn — kiểm xem bạn có tự dựng lại được TOÀN BỘ khuôn mẫu đó từ đầu,
không cần nhìn mẫu, hay không.
::::

::::example{#them-bien-the-loi-hien-ngay}
Đây mới là điều `assertNever` THỰC SỰ mang lại. Giả sử SAU NÀY, nghiệp
vụ thêm một trạng thái MỚI — `HoanTien` (đơn đã hoàn tiền). Thêm
interface đó vào union `TrangThaiDonHang`, nhưng KHÔNG sửa `moTa` —
vẫn giữ nguyên bốn `case` cũ:

```typescript title=readonly
interface ChoXuLy {
  kind: "choXuLy";
  ma: string;
}

interface DangGiao {
  kind: "dangGiao";
  ma: string;
  maVanDon: string;
}

interface DaGiao {
  kind: "daGiao";
  ma: string;
  ngayGiao: string;
}

interface DaHuy {
  kind: "daHuy";
  ma: string;
  lyDoHuy: string;
}

interface HoanTien {
  kind: "hoanTien";
  ma: string;
  soTienHoan: number;
}

type TrangThaiDonHang = ChoXuLy | DangGiao | DaGiao | DaHuy | HoanTien;

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý biến thể: " + JSON.stringify(x));
}

function moTa(d: TrangThaiDonHang): string {
  switch (d.kind) {
    case "choXuLy":
      return "đơn " + d.ma + " đang chờ xử lý";
    case "dangGiao":
      return "đơn " + d.ma + " đang giao, vận đơn " + d.maVanDon;
    case "daGiao":
      return "đơn " + d.ma + " đã giao ngày " + d.ngayGiao;
    case "daHuy":
      return "đơn " + d.ma + " đã huỷ, lý do: " + d.lyDoHuy;
    default:
      return chuaXuLy(d);
  }
}

console.log(moTa({ kind: "hoanTien", ma: "DH-5", soTienHoan: 100000 }));
```

Chương trình này KHÔNG CÒN biên dịch được nữa. TypeScript báo đúng mã
`TS2345`: `Argument of type 'HoanTien' is not assignable to parameter
of type 'never'.` — NGAY tại dòng `return chuaXuLy(d);` bên trong
`moTa`. KHÔNG phải ở dòng khai báo `type TrangThaiDonHang` (thêm một
biến thể vào union không bao giờ tự nó là lỗi), cũng KHÔNG phải ở dòng
`console.log` cuối cùng.

Lý do: sau bốn `case` cũ, `d` tại nhánh `default` giờ bị thu hẹp xuống
đúng phần CÒN LẠI của union — mà phần còn lại đó bây giờ là `HoanTien`,
không còn là `never` nữa. Gọi `chuaXuLy(d)` với một đối số kiểu
`HoanTien` VI PHẠM chữ ký `never` mà hàm đòi hỏi, TypeScript CHẶN NGAY
lúc biên dịch — không cần chạy chương trình, không cần ai ĐỌC KỸ để
nhận ra thiếu sót. Đây CHÍNH LÀ "bài kiểm tra sống" của kỷ luật
exhaustiveness: bạn không tự canh bằng mắt xem mình đã sửa hết
`switch` chưa — trình biên dịch TỰ LÀM việc đó, mỗi lần build lại.
::::

::::predict{#doan-thieu-case-co-assertnever commitOnce}
```typescript
interface TienMat {
  kind: "tienMat";
}

interface TheTinDung {
  kind: "theTinDung";
  soThe: string;
}

interface ChuyenKhoan {
  kind: "chuyenKhoan";
  soTaiKhoan: string;
}

interface ViDienTu {
  kind: "viDienTu";
  maVi: string;
}

type PhuongThucThanhToan = TienMat | TheTinDung | ChuyenKhoan | ViDienTu;

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function phiXuLy(p: PhuongThucThanhToan): number {
  switch (p.kind) {
    case "tienMat":
      return 0;
    case "theTinDung":
      return 5000;
    case "chuyenKhoan":
      return 2000;
    default:
      return chuaXuLy(p);
  }
}

console.log(phiXuLy({ kind: "viDienTu", maVi: "VI-1" }));
```

Dòng cuối in ra gì?

:::opt{correct}
Máy báo lỗi biên dịch tại dòng `return chuaXuLy(p);` — sau ba `case`
đã xử lý, `p` chỉ còn kiểu `ViDienTu`, không khớp tham số `never` mà
`chuaXuLy` đòi
:::

:::opt
Biên dịch bình thường, in ra `undefined` — vì `switch` không bắt buộc
phải liệt kê hết mọi case của union
::why
Gần đúng ở việc bạn nhớ ĐÚNG: một `switch` KHÔNG có `default` gọi
`chuaXuLy` (bài 7 đã học) biên dịch được dù thiếu `case`, và trả về
`undefined` lúc chạy — quan sát đó ĐÚNG cho trường hợp đó.

Chỗ lệch: `switch` NÀY CÓ `default: return chuaXuLy(p);`. Sau ba
`case` tường minh, TypeScript thu hẹp `p` còn kiểu `ViDienTu` (không
phải `never`, vì union giờ có BỐN biến thể mà `switch` chỉ xử lý BA) —
gọi `chuaXuLy(p)` với đối số kiểu `ViDienTu` VI PHẠM chữ ký đòi
`never`, TS2345 chặn NGAY lúc biên dịch, không có dòng nào được chạy
để trả `undefined`.
::
:::

:::opt
Biên dịch lỗi — nhưng lỗi hiện ở dòng `type PhuongThucThanhToan = ...
| ViDienTu;`, báo rằng biến thể `ViDienTu` chưa được dùng ở đâu
::why
Gần đúng ở việc bạn đoán ĐÚNG là CÓ lỗi biên dịch — quan sát đó đúng.

Chỗ lệch: TypeScript KHÔNG kiểm "biến thể đã được dùng chưa" ngay tại
dòng khai báo union — bản thân dòng `type PhuongThucThanhToan = ...`
biên dịch HOÀN TOÀN bình thường (thêm một biến thể vào union không bao
giờ tự nó là lỗi, đúng như bài vừa thấy ở trên với `HoanTien`). Lỗi
thật nằm ở dòng `return chuaXuLy(p);` bên trong `phiXuLy` — nơi `p`
sau ba `case` tường minh bị thu hẹp còn `ViDienTu`, không khớp tham số
`never` mà `chuaXuLy` đòi.
::
:::
::::

::::code{#mo_ta_yeu_cau}
Cho sẵn union `TrangThaiYeuCau` BỐN biến thể (`moi` / `dangXuLy` /
`daGiaiQuyet` / `daTuChoi`) và hàm `chuaXuLy` — hoàn thiện `moTaYeuCau`
để nhánh `default` gọi ĐÚNG hàm đó với giá trị đã bị narrow.

```typescript title=starter
interface Moi {
  kind: "moi";
  ma: string;
}

interface DangXuLy {
  kind: "dangXuLy";
  ma: string;
  nguoiPhuTrach: string;
}

interface DaGiaiQuyet {
  kind: "daGiaiQuyet";
  ma: string;
  ghiChu: string;
}

interface DaTuChoi {
  kind: "daTuChoi";
  ma: string;
  lyDo: string;
}

type TrangThaiYeuCau = Moi | DangXuLy | DaGiaiQuyet | DaTuChoi;

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý biến thể: " + JSON.stringify(x));
}

function moTaYeuCau(y: TrangThaiYeuCau): string {
  switch (y.kind) {
    case "moi":
      return "yêu cầu " + y.ma + " mới tạo";
    case "dangXuLy":
      return "yêu cầu " + y.ma + " đang xử lý bởi " + y.nguoiPhuTrach;
    case "daGiaiQuyet":
      return "yêu cầu " + y.ma + " đã giải quyết: " + y.ghiChu;
    case "daTuChoi":
      return "yêu cầu " + y.ma + " đã từ chối, lý do: " + y.lyDo;
    default:
      return ___(y);
  }
}

console.log(moTaYeuCau({ kind: "moi", ma: "YC-1" }));
```

```typescript title=solution
interface Moi {
  kind: "moi";
  ma: string;
}

interface DangXuLy {
  kind: "dangXuLy";
  ma: string;
  nguoiPhuTrach: string;
}

interface DaGiaiQuyet {
  kind: "daGiaiQuyet";
  ma: string;
  ghiChu: string;
}

interface DaTuChoi {
  kind: "daTuChoi";
  ma: string;
  lyDo: string;
}

type TrangThaiYeuCau = Moi | DangXuLy | DaGiaiQuyet | DaTuChoi;

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý biến thể: " + JSON.stringify(x));
}

function moTaYeuCau(y: TrangThaiYeuCau): string {
  switch (y.kind) {
    case "moi":
      return "yêu cầu " + y.ma + " mới tạo";
    case "dangXuLy":
      return "yêu cầu " + y.ma + " đang xử lý bởi " + y.nguoiPhuTrach;
    case "daGiaiQuyet":
      return "yêu cầu " + y.ma + " đã giải quyết: " + y.ghiChu;
    case "daTuChoi":
      return "yêu cầu " + y.ma + " đã từ chối, lý do: " + y.lyDo;
    default:
      return chuaXuLy(y);
  }
}

console.log(moTaYeuCau({ kind: "moi", ma: "YC-1" }));
```

```typescript title=test
const r1 = moTaYeuCau({ kind: "moi", ma: "YC-1" });
if (r1 !== "yêu cầu YC-1 mới tạo") throw new Error("case moi sai — đang là " + r1);

const r2 = moTaYeuCau({ kind: "dangXuLy", ma: "YC-2", nguoiPhuTrach: "An" });
if (r2 !== "yêu cầu YC-2 đang xử lý bởi An") throw new Error("case dangXuLy sai — đang là " + r2);

const r3 = moTaYeuCau({ kind: "daGiaiQuyet", ma: "YC-3", ghiChu: "đã sửa xong" });
if (r3 !== "yêu cầu YC-3 đã giải quyết: đã sửa xong") throw new Error("case daGiaiQuyet sai — đang là " + r3);

const r4 = moTaYeuCau({ kind: "daTuChoi", ma: "YC-4", lyDo: "trùng yêu cầu khác" });
if (r4 !== "yêu cầu YC-4 đã từ chối, lý do: trùng yêu cầu khác") throw new Error("case daTuChoi sai — đang là " + r4);
```

:::hints
- kind: attention
  body: "Chỗ trống là TÊN HÀM được gọi ở nhánh default — không phải một giá trị mới, chỉ là gọi lại đúng hàm chuaXuLy đã định nghĩa ngay phía trên."
- kind: strategy
  body: "chuaXuLy(x: never): never đã có sẵn. Sau khi switch đã xử lý đủ cả bốn case (moi/dangXuLy/daGiaiQuyet/daTuChoi), TypeScript tự thu hẹp y ở nhánh default xuống còn never — khớp đúng tham số chuaXuLy đòi hỏi."
- kind: one-line
  body: "Chỗ trống là: chuaXuLy"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "yêu cầu YC-1 mới tạo"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn biến thể, một `switch` đầy đủ, một `chuaXuLy` đứng gác — và bạn
vừa thấy CHÍNH XÁC tại sao nó đáng công: thêm biến thể thứ năm, lỗi
hiện ra NGAY, đúng chỗ, trước khi chương trình chạy dòng nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cụm này khép lại: discriminant field, narrow bằng `if`/`switch`,
`assertNever` bắt lỗi lúc biên dịch. Nhưng có một câu hỏi ĐỊNH NGHĨA
chưa trả lời — `interface NguoiDung { ten: string; tuoi: number }` và
`type Hinh = Vuong | Tron` đều được gọi là "ADT" (algebraic data
type), nhưng chúng KHÁC NHAU ở CHỖ NÀO? Một cái đòi CẢ HAI field cùng
lúc, cái kia chỉ cho ĐÚNG MỘT biến thể tại một thời điểm — hai khái
niệm đó có TÊN RIÊNG không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
