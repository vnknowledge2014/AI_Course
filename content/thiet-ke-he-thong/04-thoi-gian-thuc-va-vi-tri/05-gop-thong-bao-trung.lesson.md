---
id: thiet-ke-he-thong.thoi-gian-thuc-va-vi-tri.gop-thong-bao-trung
title: "Gộp thông báo trùng: một tin, không phải mười"
summary: "xuLyGopThongBao gop nhieu su kien CUNG loai trong mot cua so thoi gian (do bang DongHoMoPhong) thanh MOT thong bao duy nhat -- 5 luot thich trong cua so 1000ms tra ve '5 nguoi da thich bai viet cua ban' thay vi 5 dong rieng. Goi TRUOC khi du cua so tra ve null (chua gop); goi lai NGAY sau khi vua gop (khong co su kien moi) cung tra ve null vi danh sach da duoc RESET ve rong."
locale: vi
track: thiet-ke-he-thong
module: thoi-gian-thuc-va-vi-tri
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.gop-thong-bao-trung]
requires: [sd.da-kenh-gui-thong-bao]
concepts: [sd.gop-thong-bao-trung]
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
Kênh gửi đã xong — thử tuần tự, dừng khi thắng. Nhưng CÓ một bài đăng nổi
tiếng, năm người thích nó chỉ trong vài giây. Gửi năm thông báo riêng LÀ
tra tấn hộp thư — cần GỘP chúng lại.
::::

::::explain{#gop-theo-cua-so-thoi-gian}
Thay vì gửi thông báo NGAY mỗi khi có sự kiện, hệ thống GHI NHẬN sự kiện
vào một "bộ gom" trước — RỒI định kỳ (mỗi khi đủ một cửa sổ thời gian,
đo bằng đồng hồ mô phỏng) mới XỬ lý, gộp tất cả sự kiện đã tích luỹ thành
đúng MỘT thông báo:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface BoGomThongBao {
  cacNguoiThich: string[];
  thoiDiemSuKienDauTien: number | null;
}
function taoBoGomThongBao(): BoGomThongBao {
  return { cacNguoiThich: [], thoiDiemSuKienDauTien: null };
}

function ghiNhanLuotThich(bg: BoGomThongBao, dh: DongHoMoPhong, nguoiThich: string): void {
  if (bg.thoiDiemSuKienDauTien === null) bg.thoiDiemSuKienDauTien = dh.thoiGianHienTai;
  bg.cacNguoiThich.push(nguoiThich);
}

function xuLyGopThongBao(bg: BoGomThongBao, dh: DongHoMoPhong, cuaSoMs: number): string | null {
  if (bg.thoiDiemSuKienDauTien === null) return null;
  if (dh.thoiGianHienTai - bg.thoiDiemSuKienDauTien < cuaSoMs) return null;
  const soLuong = bg.cacNguoiThich.length;
  const noiDung = soLuong === 1
    ? `${bg.cacNguoiThich[0]!} da thich bai viet cua ban`
    : `${soLuong} nguoi da thich bai viet cua ban`;
  bg.cacNguoiThich = [];
  bg.thoiDiemSuKienDauTien = null;
  return noiDung;
}

const CUA_SO_MS = 1000;
const dh = taoDongHoMoPhong();
const bg = taoBoGomThongBao();

ghiNhanLuotThich(bg, dh, "an");
ghiNhanLuotThich(bg, dh, "binh");
ghiNhanLuotThich(bg, dh, "chi");
ghiNhanLuotThich(bg, dh, "dung");
ghiNhanLuotThich(bg, dh, "em");

tienThoiGian(dh, 500);
console.log("t=500 (chua du cua so 1000), xu ly gop:", xuLyGopThongBao(bg, dh, CUA_SO_MS));

tienThoiGian(dh, 500);
console.log("t=1000 (dung cua so), xu ly gop:", xuLyGopThongBao(bg, dh, CUA_SO_MS));
```

```text title=readonly
t=500 (chua du cua so 1000), xu ly gop: null
t=1000 (dung cua so), xu ly gop: 5 nguoi da thich bai viet cua ban
```

Năm lượt thích được `ghiNhanLuotThich` TÍCH luỹ vào `cacNguoiThich` —
chưa hề gửi gì. Gọi `xuLyGopThongBao` LÚC `t=500` (chưa đủ `cuaSoMs`) trả
về `null`, chưa xử LÝ gì cả. Chỉ khi đồng hồ đạt `t=1000` — đúng
`CUA_SO_MS` kể từ sự kiện ĐẦU tiên — hàm mới gộp cả năm lượt thích thành
đúng MỘT dòng thông báo duy nhất.
::::

::::example{#mot-nguoi-dung-ten-rieng}
Khi CHỈ có đúng MỘT sự kiện trong cửa sổ, thông báo dùng TÊN riêng của
người đó thay vì con số — VÀ cửa sổ MỚI (sau khi gộp xong) tích luỹ ĐỘC
LẬP với cửa sổ trước:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface BoGomThongBao {
  cacNguoiThich: string[];
  thoiDiemSuKienDauTien: number | null;
}
function taoBoGomThongBao(): BoGomThongBao {
  return { cacNguoiThich: [], thoiDiemSuKienDauTien: null };
}

function ghiNhanLuotThich(bg: BoGomThongBao, dh: DongHoMoPhong, nguoiThich: string): void {
  if (bg.thoiDiemSuKienDauTien === null) bg.thoiDiemSuKienDauTien = dh.thoiGianHienTai;
  bg.cacNguoiThich.push(nguoiThich);
}

function xuLyGopThongBao(bg: BoGomThongBao, dh: DongHoMoPhong, cuaSoMs: number): string | null {
  if (bg.thoiDiemSuKienDauTien === null) return null;
  if (dh.thoiGianHienTai - bg.thoiDiemSuKienDauTien < cuaSoMs) return null;
  const soLuong = bg.cacNguoiThich.length;
  const noiDung = soLuong === 1
    ? `${bg.cacNguoiThich[0]!} da thich bai viet cua ban`
    : `${soLuong} nguoi da thich bai viet cua ban`;
  bg.cacNguoiThich = [];
  bg.thoiDiemSuKienDauTien = null;
  return noiDung;
}

const CUA_SO_MS = 1000;
// tai lap dung trang thai: bo gom moi, dong ho o t=0
const dh = taoDongHoMoPhong();
const bg = taoBoGomThongBao();

ghiNhanLuotThich(bg, dh, "hoang"); // CHI mot nguoi thich trong cua so nay
tienThoiGian(dh, 1000);
console.log("chi 1 nguoi thich -> thong bao:", xuLyGopThongBao(bg, dh, CUA_SO_MS));

// cua so MOI bat dau: ghi nhan tiep 2 luot thich khac sau khi cua so truoc da xu ly xong
ghiNhanLuotThich(bg, dh, "khanh");
ghiNhanLuotThich(bg, dh, "lan");
tienThoiGian(dh, 1000);
console.log("cua so moi, 2 nguoi thich -> thong bao:", xuLyGopThongBao(bg, dh, CUA_SO_MS));
```

```text title=readonly
chi 1 nguoi thich -> thong bao: hoang da thich bai viet cua ban
cua so moi, 2 nguoi thich -> thong bao: 2 nguoi da thich bai viet cua ban
```

Với `soLuong === 1`, thông báo dùng TÊN thẳng (`"hoang da thich..."`)
thay vì con số vô nghĩa `"1 nguoi da thich..."`. SAU khi cửa sổ đầu xử lý
xong, `cacNguoiThich` VÀ `thoiDiemSuKienDauTien` đều bị RESET — cửa sổ
kế tiếp bắt đầu ĐẾM lại từ đầu, không hề mang theo dữ liệu cũ.
::::

::::predict{#doan-cua-so-ngan-hai-nguoi commitOnce}
`cuaSoMs = 500`. Tại `t=0`, `"toan"` VÀ `"vinh"` cùng thích một bài viết
(hai lượt thích, cùng nằm trong cửa sổ). Đồng hồ tiến tới `t=500`. Gọi
`xuLyGopThongBao` — nội dung thông báo trả về LÀ gì?

:::opt{correct}
`"2 nguoi da thich bai viet cua ban"` — `cacNguoiThich` có đúng hai phần
tử NÊN `soLuong === 2`, rơi vào nhánh dùng CON SỐ chứ không phải nhánh
dùng tên riêng (nhánh đó chỉ áp dụng khi `soLuong === 1`)
:::
:::opt
`"toan va vinh da thich bai viet cua ban"` — liệt kê ĐỦ tên của cả hai
người, để người nhận biết chính xác AI đã thích bài viết của mình
::why
Nhầm "gộp thông báo" VỚI "liệt kê đầy đủ danh sách tên" — nhưng
`xuLyGopThongBao` chỉ có ĐÚNG hai nhánh nội dung: một TÊN riêng (khi
`soLuong === 1`) hoặc một CON SỐ (mọi trường hợp còn lại).

Chỗ lệch: biểu thức điều kiện `soLuong === 1 ? ... : \`${soLuong} nguoi
da thich...\`` không hề có nhánh THỨ ba để nối tên nhiều người — với
`soLuong = 2`, nó rơi thẳng vào nhánh `else`, in ra con số `2`, không in
tên `"toan"` hay `"vinh"` ở bất kỳ đâu trong chuỗi kết quả.
::
:::
::::

::::code{#viet_xu_ly_gop_thong_bao}
Hoàn thiện `xuLyGopThongBao` — sau khi đã qua được hai điều kiện dừng
sớm (chưa có sự kiện, chưa đủ cửa sổ), tính `noiDung`: dùng TÊN riêng nếu
chỉ có một người, dùng CON SỐ nếu nhiều hơn.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface BoGomThongBao {
  cacNguoiThich: string[];
  thoiDiemSuKienDauTien: number | null;
}
function taoBoGomThongBao(): BoGomThongBao {
  return { cacNguoiThich: [], thoiDiemSuKienDauTien: null };
}

function ghiNhanLuotThich(bg: BoGomThongBao, dh: DongHoMoPhong, nguoiThich: string): void {
  if (bg.thoiDiemSuKienDauTien === null) bg.thoiDiemSuKienDauTien = dh.thoiGianHienTai;
  bg.cacNguoiThich.push(nguoiThich);
}

function xuLyGopThongBao(bg: BoGomThongBao, dh: DongHoMoPhong, cuaSoMs: number): string | null {
  if (bg.thoiDiemSuKienDauTien === null) return null;
  if (dh.thoiGianHienTai - bg.thoiDiemSuKienDauTien < cuaSoMs) return null;
  const soLuong = bg.cacNguoiThich.length;
  ___
  bg.cacNguoiThich = [];
  bg.thoiDiemSuKienDauTien = null;
  return noiDung;
}

const dhX = taoDongHoMoPhong();
const bgX = taoBoGomThongBao();
ghiNhanLuotThich(bgX, dhX, "p1");
ghiNhanLuotThich(bgX, dhX, "p2");
tienThoiGian(dhX, 500);
console.log(xuLyGopThongBao(bgX, dhX, 500));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface BoGomThongBao {
  cacNguoiThich: string[];
  thoiDiemSuKienDauTien: number | null;
}
function taoBoGomThongBao(): BoGomThongBao {
  return { cacNguoiThich: [], thoiDiemSuKienDauTien: null };
}

function ghiNhanLuotThich(bg: BoGomThongBao, dh: DongHoMoPhong, nguoiThich: string): void {
  if (bg.thoiDiemSuKienDauTien === null) bg.thoiDiemSuKienDauTien = dh.thoiGianHienTai;
  bg.cacNguoiThich.push(nguoiThich);
}

function xuLyGopThongBao(bg: BoGomThongBao, dh: DongHoMoPhong, cuaSoMs: number): string | null {
  if (bg.thoiDiemSuKienDauTien === null) return null;
  if (dh.thoiGianHienTai - bg.thoiDiemSuKienDauTien < cuaSoMs) return null;
  const soLuong = bg.cacNguoiThich.length;
  const noiDung = soLuong === 1
    ? `${bg.cacNguoiThich[0]!} da thich bai viet cua ban`
    : `${soLuong} nguoi da thich bai viet cua ban`;
  bg.cacNguoiThich = [];
  bg.thoiDiemSuKienDauTien = null;
  return noiDung;
}

const dhX = taoDongHoMoPhong();
const bgX = taoBoGomThongBao();
ghiNhanLuotThich(bgX, dhX, "p1");
ghiNhanLuotThich(bgX, dhX, "p2");
tienThoiGian(dhX, 500);
console.log(xuLyGopThongBao(bgX, dhX, 500));
```

```typescript title=test
const dh = taoDongHoMoPhong();
const bg = taoBoGomThongBao();
const CUA_SO = 500;

if (xuLyGopThongBao(bg, dh, CUA_SO) !== null) throw new Error("chua co su kien nao thi phai tra ve null");

ghiNhanLuotThich(bg, dh, "an");
tienThoiGian(dh, 499);
if (xuLyGopThongBao(bg, dh, CUA_SO) !== null) throw new Error("t=499 < cua so 500, chua duoc gop");

tienThoiGian(dh, 1); // t=500, dung cua so
const kq1 = xuLyGopThongBao(bg, dh, CUA_SO);
if (kq1 !== "an da thich bai viet cua ban") throw new Error("dung 1 nguoi thich phai dung ten rieng, khong dung so luong");

ghiNhanLuotThich(bg, dh, "binh");
ghiNhanLuotThich(bg, dh, "chi");
ghiNhanLuotThich(bg, dh, "dung");
tienThoiGian(dh, CUA_SO);
const kq2 = xuLyGopThongBao(bg, dh, CUA_SO);
if (kq2 !== "3 nguoi da thich bai viet cua ban") throw new Error("3 nguoi thich phai gop thanh MOT thong bao ghi so luong");

if (bg.cacNguoiThich.length !== 0) throw new Error("sau khi xu ly gop, danh sach phai duoc RESET ve rong");
if (bg.thoiDiemSuKienDauTien !== null) throw new Error("sau khi xu ly gop, thoiDiemSuKienDauTien phai RESET ve null");

if (xuLyGopThongBao(bg, dh, CUA_SO) !== null) throw new Error("goi lai ngay sau khi da gop (khong co su kien moi) phai tra ve null");
```

:::hints
- kind: attention
  body: "Dong bi thieu phai KHAI BAO noiDung: dung toan tu ba ngoi (? :) de chon giua 'ten rieng' (khi soLuong === 1) va 'con so' (moi truong hop con lai)."
- kind: strategy
  body: "soLuong === 1 ? mot chuoi dung bg.cacNguoiThich[0] : mot chuoi dung soLuong. Nho dung ! sau bg.cacNguoiThich[0] vi noUncheckedIndexedAccess coi no la string | undefined."
- kind: one-line
  body: "const noiDung = soLuong === 1 ? `${bg.cacNguoiThich[0]!} da thich bai viet cua ban` : `${soLuong} nguoi da thich bai viet cua ban`;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "2 nguoi"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Notification xong: đa kênh, gộp theo cửa sổ. Sang Proximity — nơi câu hỏi
đổi hẳn: không phải "ai online", mà LÀ "ai đang Ở GẦN đây".
::::

::::reflect{#nghi-lai}
`xuLyGopThongBao` tách RIÊNG hai việc: ghi nhận (rẻ, chạy MỖI sự kiện) và
gộp-gửi (đắt hơn, chỉ chạy khi ĐỦ cửa sổ). `DongHoMoPhong` cho phép kiểm
chứng đúng NHỊP đó mà không cần đợi đồng hồ thật trôi qua từng mili giây.
::::

::::checkpoint{mastery=0.74}
::::
