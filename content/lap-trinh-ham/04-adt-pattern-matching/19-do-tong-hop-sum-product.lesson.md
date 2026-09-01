---
id: lap-trinh-ham.adt-pattern-matching.do-tong-hop-sum-product
title: "Đo tổng hợp: sum type & product type"
summary: "Bài chốt cụm 3, code có chấm điểm sống: ghép sum-of-products cho một tình huống MỚI (không lặp lại DonHang/Hinh đã dùng), viết hàm xử lý đủ mọi biến thể. Không khái niệm mới — đo khả năng tự thiết kế ADT từ một mô tả nghiệp vụ, không chỉ điền vào khung có sẵn."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 19
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.review-sum-product]
requires: [ts.write-nested-adt]
concepts: [ts.review-sum-product]
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
Bài trước bạn viết một ADT lồng nhau: hai biến thể, mỗi biến thể tự
mang field riêng, một hàm `switch` đọc đúng field. Hôm nay: y hệt kỹ
năng đó — nhưng cho một bài toán bạn CHƯA từng thấy trong track này.
::::

::::explain{#on-lai-sum-of-products}
```typescript
interface DungGio {
  kind: "dungGio";
  soHieu: string;
}

interface TreHoan {
  kind: "treHoan";
  soHieu: string;
  soPhutTre: number;
}

interface Huy {
  kind: "huy";
  soHieu: string;
  lyDoHuy: string;
}

type ChuyenBay = DungGio | TreHoan | Huy;

function thongBao(c: ChuyenBay): string {
  switch (c.kind) {
    case "dungGio":
      return "Chuyến " + c.soHieu + ": đúng giờ";
    case "treHoan":
      return "Chuyến " + c.soHieu + ": trễ " + c.soPhutTre + " phút";
    case "huy":
      return "Chuyến " + c.soHieu + ": đã huỷ — " + c.lyDoHuy;
  }
}

console.log(thongBao({ kind: "dungGio", soHieu: "VN204" }));
console.log(thongBao({ kind: "treHoan", soHieu: "VN205", soPhutTre: 45 }));
console.log(thongBao({ kind: "huy", soHieu: "VN206", lyDoHuy: "thời tiết xấu" }));
```

```text
Chuyến VN204: đúng giờ
Chuyến VN205: trễ 45 phút
Chuyến VN206: đã huỷ — thời tiết xấu
```

Bốn bài vừa qua ghép lại thành MỘT ở `ChuyenBay`: `interface` là kiểu
TÍCH (bài 14 — mỗi biến thể ĐÒI đủ field riêng CÙNG LÚC), union có
nhãn là kiểu TỔNG (bài 15 — MỘT `ChuyenBay` chỉ thuộc ĐÚNG một trong
ba biến thể), số trạng thái khả dĩ ĐẾM được bằng nhân và cộng (bài
16), và một sum type THẬT SỰ thường LỒNG một product type bên trong
TỪNG biến thể (bài 17 — `TreHoan` cần CẢ `soHieu` LẪN `soPhutTre` cùng
lúc, `Huy` cần CẢ `soHieu` LẪN `lyDoHuy`, không biến thể nào chỉ có
mỗi `kind`).

`thongBao` là một hàm xử lý ĐẦY ĐỦ (bài 18) — `switch` trên `kind`,
mỗi `case` đọc ĐÚNG field của biến thể đó, không nhánh nào bị bỏ sót.

Không có khái niệm MỚI nào ở đây. `ChuyenBay` không phải `DonHang`
hay `Hinh` đã dùng ở các bài trước — đây là bài ĐO: bạn có tự NGHĨ RA
được một ADT như thế này từ một mô tả nghiệp vụ, không chỉ đọc hiểu ví
dụ có sẵn, hay không?
::::

::::example{#mot-adt-nhieu-ham}
`ChuyenBay` không chỉ dùng được cho MỘT hàm — bất kỳ hàm nào cần xử lý
khác nhau theo biến thể đều dùng được CÙNG khuôn `switch (c.kind)`:

```typescript title=readonly
interface DungGio {
  kind: "dungGio";
  soHieu: string;
}

interface TreHoan {
  kind: "treHoan";
  soHieu: string;
  soPhutTre: number;
}

interface Huy {
  kind: "huy";
  soHieu: string;
  lyDoHuy: string;
}

type ChuyenBay = DungGio | TreHoan | Huy;

function canBaoDong(c: ChuyenBay): boolean {
  switch (c.kind) {
    case "dungGio":
      return false;
    case "treHoan":
      return c.soPhutTre >= 60;
    case "huy":
      return true;
  }
}

const dsChuyen: ChuyenBay[] = [
  { kind: "dungGio", soHieu: "VN100" },
  { kind: "treHoan", soHieu: "VN101", soPhutTre: 20 },
  { kind: "treHoan", soHieu: "VN102", soPhutTre: 90 },
  { kind: "huy", soHieu: "VN103", lyDoHuy: "kỹ thuật" },
];

for (const c of dsChuyen) {
  console.log(c.soHieu, canBaoDong(c));
}
```

```text title=readonly
VN100 false
VN101 false
VN102 true
VN103 true
```

`canBaoDong` trả `true`/`false` tuỳ biến thể: chuyến ĐÚNG GIỜ không
bao giờ cần cảnh báo (`false` cố định), chuyến HUỶ luôn cần cảnh báo
(`true` cố định), còn chuyến TRỄ HOÃN thì TUỲ — chỉ cảnh báo nếu trễ
từ 60 phút trở lên (`c.soPhutTre >= 60`, biến thể `VN101` trễ 20 phút
nên `false`, `VN102` trễ 90 phút nên `true`). Ba công thức khác hẳn
nhau, nhưng vẫn CÙNG một khuôn `switch` — một ADT phục vụ được NHIỀU
hàm, không riêng một hàm mô tả.
::::

::::predict{#doan-gia-cuoi-cung commitOnce}
```typescript
interface VeThuong {
  kind: "thuong";
  gia: number;
}

interface VeVip {
  kind: "vip";
  gia: number;
  phuThu: number;
}

interface VeSinhVien {
  kind: "sinhVien";
  gia: number;
  giamGiaPhanTram: number;
}

type Ve = VeThuong | VeVip | VeSinhVien;

function giaCuoiCung(v: Ve): number {
  switch (v.kind) {
    case "thuong":
      return v.gia;
    case "vip":
      return v.gia + v.phuThu;
    case "sinhVien":
      return v.gia * (1 - v.giamGiaPhanTram / 100);
  }
}

console.log(giaCuoiCung({ kind: "vip", gia: 100, phuThu: 50 }));
console.log(giaCuoiCung({ kind: "sinhVien", gia: 100, giamGiaPhanTram: 20 }));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`150` rồi `80`
:::

:::opt
`80` rồi `150`
::why
Gần đúng ở việc bạn tính đúng CẢ HAI con số 150 và 80 — case `"vip"`
cộng thêm phụ thu (100 + 50 = 150 đúng), case `"sinhVien"` nhân với tỉ
lệ còn lại sau giảm giá (100 * (1 - 20/100) = 80 đúng).

Chỗ lệch: thứ tự IN RA đi theo ĐÚNG thứ tự các dòng `console.log`
trong mã nguồn, không theo thứ tự khai báo biến thể trong `type Ve`.
Dòng ĐẦU TIÊN gọi `giaCuoiCung` với `kind: "vip"` nên in `150` TRƯỚC;
dòng THỨ HAI gọi với `kind: "sinhVien"` nên in `80` SAU, không phải
ngược lại.
::
:::

:::opt
`150` rồi `100`
::why
Gần đúng ở việc bạn tính đúng con số ĐẦU TIÊN — case `"vip"` trả về
`v.gia + v.phuThu`, với `gia: 100` và `phuThu: 50` thì 100 + 50 = 150
đúng.

Chỗ lệch: case `"sinhVien"` KHÔNG trả về `v.gia` giữ nguyên — nó NHÂN
với tỉ lệ còn lại sau giảm giá: `v.gia * (1 - v.giamGiaPhanTram / 100)`.
Với `gia: 100` và `giamGiaPhanTram: 20`, đó là 100 * (1 - 20/100) = 80,
không phải 100 — 100 chỉ đúng nếu QUÊN áp dụng phần trăm giảm giá.
::
:::
::::

::::code{#tom_tat_yeu_cau_ho_tro}
Viết hàm `tomTat(y)` xử lý discriminated union `YeuCauHoTro` — BA biến
thể (`moi`, `dangXuLy`, `daDong`) cho một tình huống nghiệp vụ MỚI
(yêu cầu hỗ trợ khách hàng), không phải `DonHang` hay `Hinh` đã gặp.
Nhánh `"dangXuLy"` còn thiếu MỘT field — đọc đúng field RIÊNG của biến
thể `YeuCauDangXuLy` để hoàn thành nó.

```typescript title=starter
interface YeuCauMoi {
  kind: "moi";
  maVe: string;
  noiDung: string;
}

interface YeuCauDangXuLy {
  kind: "dangXuLy";
  maVe: string;
  nhanVien: string;
}

interface YeuCauDaDong {
  kind: "daDong";
  maVe: string;
  ketQua: string;
}

type YeuCauHoTro = YeuCauMoi | YeuCauDangXuLy | YeuCauDaDong;

function tomTat(y: YeuCauHoTro): string {
  switch (y.kind) {
    case "moi":
      return "[" + y.maVe + "] mới: " + y.noiDung;
    case "dangXuLy":
      return "[" + y.maVe + "] đang xử lý bởi " + y.___;
    case "daDong":
      return "[" + y.maVe + "] đã đóng: " + y.ketQua;
  }
}

console.log(tomTat({ kind: "moi", maVe: "HT-1", noiDung: "không đăng nhập được" }));
console.log(tomTat({ kind: "dangXuLy", maVe: "HT-2", nhanVien: "Lan" }));
console.log(tomTat({ kind: "daDong", maVe: "HT-3", ketQua: "đã khắc phục" }));
```

```typescript title=solution
interface YeuCauMoi {
  kind: "moi";
  maVe: string;
  noiDung: string;
}

interface YeuCauDangXuLy {
  kind: "dangXuLy";
  maVe: string;
  nhanVien: string;
}

interface YeuCauDaDong {
  kind: "daDong";
  maVe: string;
  ketQua: string;
}

type YeuCauHoTro = YeuCauMoi | YeuCauDangXuLy | YeuCauDaDong;

function tomTat(y: YeuCauHoTro): string {
  switch (y.kind) {
    case "moi":
      return "[" + y.maVe + "] mới: " + y.noiDung;
    case "dangXuLy":
      return "[" + y.maVe + "] đang xử lý bởi " + y.nhanVien;
    case "daDong":
      return "[" + y.maVe + "] đã đóng: " + y.ketQua;
  }
}

console.log(tomTat({ kind: "moi", maVe: "HT-1", noiDung: "không đăng nhập được" }));
console.log(tomTat({ kind: "dangXuLy", maVe: "HT-2", nhanVien: "Lan" }));
console.log(tomTat({ kind: "daDong", maVe: "HT-3", ketQua: "đã khắc phục" }));
```

```typescript title=test
const r1 = tomTat({ kind: "moi", maVe: "HT-1", noiDung: "không đăng nhập được" });
if (r1 !== "[HT-1] mới: không đăng nhập được") throw new Error("tomTat cho YeuCauMoi sai — đang là " + r1);

const r2 = tomTat({ kind: "dangXuLy", maVe: "HT-2", nhanVien: "Lan" });
if (r2 !== "[HT-2] đang xử lý bởi Lan") throw new Error("tomTat cho YeuCauDangXuLy phải trả về \"[HT-2] đang xử lý bởi Lan\" — đang là " + r2);

const r3 = tomTat({ kind: "dangXuLy", maVe: "HT-9", nhanVien: "Minh" });
if (r3 !== "[HT-9] đang xử lý bởi Minh") throw new Error("tomTat cho YeuCauDangXuLy phải trả về \"[HT-9] đang xử lý bởi Minh\" — đang là " + r3);

const r4 = tomTat({ kind: "daDong", maVe: "HT-3", ketQua: "đã khắc phục" });
if (r4 !== "[HT-3] đã đóng: đã khắc phục") throw new Error("tomTat cho YeuCauDaDong sai — đang là " + r4);
```

:::hints
- kind: attention
  body: "Chỗ trống nằm trong nhánh case \"dangXuLy\" — interface YeuCauDangXuLy có field nào ngoài kind và maVe để đọc ra tên người phụ trách?"
- kind: strategy
  body: 'interface YeuCauDangXuLy khai kind: "dangXuLy", maVe: string, và nhanVien: string — bên trong nhánh case "dangXuLy", TypeScript đã NARROW y thành YeuCauDangXuLy, field còn lại để đọc là nhanVien.'
- kind: one-line
  body: "Chỗ trống là: nhanVien"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "[HT-2] đang xử lý bởi Lan"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sum type, product type, sum-of-products, hàm xử lý đủ biến thể — bạn
vừa tự ghép lại cả bốn trên một bài toán HOÀN TOÀN MỚI, không chỉ điền
vào khung có sẵn. Cụm 3 khép lại ở đây.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`YeuCauHoTro` bạn vừa dùng ĐÒI mỗi biến thể tự mang ĐÚNG field của nó
— không thiếu, không thừa, TypeScript tự chặn nếu sai. Nhưng nếu MỘT
mô tả nghiệp vụ được viết KHÔNG bằng union có nhãn, mà bằng vài cờ
boolean/optional RỜI RẠC gộp chung trong CÙNG một `interface` — trình
biên dịch có còn ngăn được tổ hợp VÔ NGHĨA (ví dụ "vừa đang xử lý VỪA
đã đóng" cùng lúc) không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
