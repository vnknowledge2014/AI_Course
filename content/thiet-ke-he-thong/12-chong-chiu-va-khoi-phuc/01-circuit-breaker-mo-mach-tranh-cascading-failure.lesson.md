---
id: thiet-ke-he-thong.chong-chiu-va-khoi-phuc.circuit-breaker-mo-mach-tranh-cascading-failure
title: "Circuit breaker: mở mạch khi downstream lỗi liên tiếp, tránh cascading failure"
summary: "MachDien{trangThai,soLoiLienTiep,nguongLoi,thoiDiemMoMach} bat dau dong (goi binh thuong). ghiNhanKetQua dem soLoiLienTiep, mo mach (tu choi NGAY, khong goi downstream) khi vuot nguongLoi -- 3 loi lien tiep (nguong 3) chuyen dong->mo, choPhepGoi tra ve false NGAY LAP TUC. capNhatTheoThoiGian mo mach sang nua_mo sau du thoi gian cho (DongHoMoPhong); nua_mo cho THU mot request -- thanh cong thi phuc hoi ve dong (soLoiLienTiep=0), that bai thi quay LAI mo ngay, khong can dem lai tu dau."
locale: vi
track: thiet-ke-he-thong
module: chong-chiu-va-khoi-phuc
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.circuit-breaker-mo-mach-tranh-cascading-failure]
requires: [sd.boss-quan-sat-va-canh-bao]
concepts: [sd.circuit-breaker-mo-mach-tranh-cascading-failure]
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
Chín bài trước (quest "Quan sát và cảnh báo") dạy cách BIẾT chuyện gì đang
xảy ra trong hệ thống của CHÍNH mình. Nhưng biết CHƯA đủ: khi một dịch vụ
hạ nguồn (downstream) mà hệ thống GỌI tới bắt đầu lỗi liên tục, cứ tiếp tục
gọi nó — rồi chờ timeout — không hề bảo vệ được AI cả. Nó chỉ khiến LỖI của
một dịch vụ lan RỘNG thành sập cả chuỗi gọi phía trên.
::::

::::explain{#ba-trang-thai-mach}
`MachDien` mô phỏng một cầu dao điện: `trangThai` LÀ `"dong"` (đóng, gọi
downstream BÌNH thường), `"mo"` (mở, TỪ CHỐI ngay không hề gọi downstream),
hoặc `"nua_mo"` (nửa mở, cho THỬ một request). `ghiNhanKetQua` đếm
`soLoiLienTiep` — vượt `nguongLoi` LÀ mở mạch NGAY:

```typescript title=readonly
type TrangThaiMach = "dong" | "mo" | "nua_mo";
interface MachDien { trangThai: TrangThaiMach; soLoiLienTiep: number; nguongLoi: number; }
function taoMachDien(nguongLoi: number): MachDien {
  return { trangThai: "dong", soLoiLienTiep: 0, nguongLoi };
}
function choPhepGoi(mach: MachDien): boolean {
  return mach.trangThai !== "mo";
}
function ghiNhanKetQua(mach: MachDien, thanhCong: boolean): void {
  if (thanhCong) {
    mach.soLoiLienTiep = 0;
    mach.trangThai = "dong";
    return;
  }
  mach.soLoiLienTiep += 1;
  if (mach.trangThai === "nua_mo") {
    mach.trangThai = "mo";
    return;
  }
  if (mach.soLoiLienTiep >= mach.nguongLoi) {
    mach.trangThai = "mo";
  }
}

const mach = taoMachDien(3);
console.log("trang thai ban dau:", mach.trangThai, "-- cho phep goi:", choPhepGoi(mach));

ghiNhanKetQua(mach, false);
console.log("sau loi 1:", mach.trangThai, mach.soLoiLienTiep, "-- cho phep goi:", choPhepGoi(mach));
ghiNhanKetQua(mach, false);
console.log("sau loi 2:", mach.trangThai, mach.soLoiLienTiep, "-- cho phep goi:", choPhepGoi(mach));
ghiNhanKetQua(mach, false);
console.log("sau loi 3 (vuot nguong 3):", mach.trangThai, mach.soLoiLienTiep, "-- cho phep goi:", choPhepGoi(mach));
```

```text title=readonly
trang thai ban dau: dong -- cho phep goi: true
sau loi 1: dong 1 -- cho phep goi: true
sau loi 2: dong 2 -- cho phep goi: true
sau loi 3 (vuot nguong 3): mo 3 -- cho phep goi: false
```

Ba lỗi LIÊN tiếp (đúng bằng `nguongLoi=3`) chuyển `trangThai` từ `"dong"`
sang `"mo"` — VÀ ngay lập tức, `choPhepGoi` trả về `false`. Đây LÀ điểm mấu
chốt: mạch mở KHÔNG hề gọi downstream rồi chờ nó timeout mới từ chối — nó
từ chối NGAY tại chỗ, tiết kiệm TOÀN bộ thời gian chờ vô ích đó cho MỌI
request tới sau, cho tới khi mạch có lý do để thử LẠI.
::::

::::example{#nua-mo-va-phuc-hoi}
Mạch mở mãi mãi thì cũng vô dụng — hệ thống cần biết downstream ĐÃ hồi phục
CHƯA. `capNhatTheoThoiGian` chuyển mạch từ `"mo"` sang `"nua_mo"` sau khi
ĐỦ thời gian chờ trôi qua (dùng `DongHoMoPhong` quen thuộc); `"nua_mo"` cho
THỬ đúng một request — thất bại thì quay LẠI `"mo"` ngay, thành công thì
phục hồi HOÀN toàn về `"dong"`:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiMach = "dong" | "mo" | "nua_mo";
interface MachDien { trangThai: TrangThaiMach; soLoiLienTiep: number; nguongLoi: number; thoiDiemMoMach: number; }
function taoMachDien(nguongLoi: number): MachDien {
  return { trangThai: "dong", soLoiLienTiep: 0, nguongLoi, thoiDiemMoMach: 0 };
}
function choPhepGoi(mach: MachDien): boolean {
  return mach.trangThai !== "mo";
}
function ghiNhanKetQua(mach: MachDien, thanhCong: boolean, dh: DongHoMoPhong): void {
  if (thanhCong) {
    mach.soLoiLienTiep = 0;
    mach.trangThai = "dong";
    return;
  }
  mach.soLoiLienTiep += 1;
  if (mach.trangThai === "nua_mo") {
    mach.trangThai = "mo";
    mach.thoiDiemMoMach = dh.thoiGianHienTai;
    return;
  }
  if (mach.soLoiLienTiep >= mach.nguongLoi) {
    mach.trangThai = "mo";
    mach.thoiDiemMoMach = dh.thoiGianHienTai;
  }
}
function capNhatTheoThoiGian(mach: MachDien, dh: DongHoMoPhong, thoiGianChoMs: number): void {
  if (mach.trangThai === "mo" && dh.thoiGianHienTai - mach.thoiDiemMoMach >= thoiGianChoMs) {
    mach.trangThai = "nua_mo";
  }
}

const dh = taoDongHoMoPhong();
const mach = taoMachDien(2);
ghiNhanKetQua(mach, false, dh);
ghiNhanKetQua(mach, false, dh);
console.log("sau 2 loi lien tiep (nguong 2):", mach.trangThai, "-- cho phep goi:", choPhepGoi(mach));

tienThoiGian(dh, 3000);
capNhatTheoThoiGian(mach, dh, 5000);
console.log("sau 3000ms (chua du 5000ms cho), van con:", mach.trangThai);

tienThoiGian(dh, 2000);
capNhatTheoThoiGian(mach, dh, 5000);
console.log("sau tong 5000ms (du thoi gian cho), chuyen sang:", mach.trangThai, "-- cho phep goi THU:", choPhepGoi(mach));

// thu mot request trong nua_mo, THAT BAI -- quay lai mo ngay
ghiNhanKetQua(mach, false, dh);
console.log("thu that bai trong nua_mo, quay lai:", mach.trangThai);

tienThoiGian(dh, 5000);
capNhatTheoThoiGian(mach, dh, 5000);
console.log("cho du 5000ms lan nua, lai sang:", mach.trangThai);

// lan nay THU THANH CONG -- phuc hoi hoan toan
ghiNhanKetQua(mach, true, dh);
console.log("thu thanh cong trong nua_mo, phuc hoi:", mach.trangThai, "so loi lien tiep:", mach.soLoiLienTiep);
```

```text title=readonly
sau 2 loi lien tiep (nguong 2): mo -- cho phep goi: false
sau 3000ms (chua du 5000ms cho), van con: mo
sau tong 5000ms (du thoi gian cho), chuyen sang: nua_mo -- cho phep goi THU: true
thu that bai trong nua_mo, quay lai: mo
cho du 5000ms lan nua, lai sang: nua_mo
thu thanh cong trong nua_mo, phuc hoi: dong so loi lien tiep: 0
```

Chú Ý: THỬ thất bại trong `"nua_mo"` không hề CẦN đếm lại từ đầu tới
`nguongLoi` — nó quay LẠI `"mo"` NGAY sau đúng MỘT lần thất bại, vì mạch
vừa cho một cơ hội, VÀ cơ hội ĐÓ đã bị bỏ lỡ. Ngược lại, một lần thử THÀNH
CÔNG LÀ đủ để tin tưởng downstream đã ổn, phục hồi HOÀN toàn về `"dong"`.
::::

::::predict{#doan-reset-sau-thanh-cong commitOnce}
Mạch đang Ở `"dong"`, `nguongLoi=3`, ĐÃ có `soLoiLienTiep=2` (hai lỗi liên
tiếp trước đó). Gọi `ghiNhanKetQua(mach, true, dh)` (MỘT lần thành công),
RỒI gọi `ghiNhanKetQua(mach, false, dh)` (MỘT lỗi tiếp theo). Sau CẢ hai
lần gọi đó, `soLoiLienTiep` LÀ bao nhiêu, VÀ mạch có mở không?

:::opt{correct}
`soLoiLienTiep=1`, mạch VẪN `"dong"` — lần thành công RESET
`soLoiLienTiep` VỀ `0` hoàn toàn; lỗi tiếp theo bắt đầu đếm LẠI từ `0`,
CHƯA đủ chạm `nguongLoi=3`
:::
:::opt
`soLoiLienTiep=3`, mạch chuyển sang `"mo"` — hai lỗi TRƯỚC đó vẫn còn được
tính, cộng thêm lỗi MỚI LÀ đủ ba, đúng NGƯỠNG mở mạch
::why
Nhầm "một lần thành công XEN giữa" VỚI "không hề ảnh hưởng gì tới bộ đếm"
— nhưng nhánh `if (thanhCong)` trong `ghiNhanKetQua` RESET hẳn
`soLoiLienTiep` VỀ `0`, không hề giữ lại giá trị CŨ.

Chỗ lệch: dòng `if (thanhCong) { mach.soLoiLienTiep = 0; ... return; }`
chạy TRƯỚC tiên khi `thanhCong=true`, VÀ nó gán THẲNG `soLoiLienTiep = 0`
— không cộng, không trừ, chỉ đơn giản LÀ đặt lại. Lần gọi `false` NGAY sau
đó bắt đầu đếm TỪ `0`, cho `soLoiLienTiep=1` — còn RẤT xa `nguongLoi=3`.
::
:::
::::

::::code{#viet_ghi_nhan_ket_qua}
Hoàn thiện `ghiNhanKetQua` cho nhánh THẤT bại (`thanhCong=false`, phần
`thanhCong=true` đã có sẵn): tăng `soLoiLienTiep`; nếu ĐANG Ở `"nua_mo"`,
quay LẠI `"mo"` ngay (ghi lại `thoiDiemMoMach`) VÀ dừng; ngược lại, nếu
`soLoiLienTiep` đã VƯỢT (hoặc bằng) `nguongLoi`, mở mạch (cũng ghi lại
`thoiDiemMoMach`).

```typescript title=starter
type TrangThaiMach = "dong" | "mo" | "nua_mo";
interface MachDien { trangThai: TrangThaiMach; soLoiLienTiep: number; nguongLoi: number; thoiDiemMoMach: number; }
function taoMachDien(nguongLoi: number): MachDien {
  return { trangThai: "dong", soLoiLienTiep: 0, nguongLoi, thoiDiemMoMach: 0 };
}
function choPhepGoi(mach: MachDien): boolean {
  return mach.trangThai !== "mo";
}
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }

function ghiNhanKetQua(mach: MachDien, thanhCong: boolean, dh: DongHoMoPhong): void {
  if (thanhCong) {
    mach.soLoiLienTiep = 0;
    mach.trangThai = "dong";
    return;
  }
  ___
}

const dh = taoDongHoMoPhong();
const mach = taoMachDien(2);
console.log(choPhepGoi(mach));
ghiNhanKetQua(mach, false, dh);
ghiNhanKetQua(mach, false, dh);
console.log(mach.trangThai, choPhepGoi(mach));
```

```typescript title=solution
type TrangThaiMach = "dong" | "mo" | "nua_mo";
interface MachDien { trangThai: TrangThaiMach; soLoiLienTiep: number; nguongLoi: number; thoiDiemMoMach: number; }
function taoMachDien(nguongLoi: number): MachDien {
  return { trangThai: "dong", soLoiLienTiep: 0, nguongLoi, thoiDiemMoMach: 0 };
}
function choPhepGoi(mach: MachDien): boolean {
  return mach.trangThai !== "mo";
}
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }

function ghiNhanKetQua(mach: MachDien, thanhCong: boolean, dh: DongHoMoPhong): void {
  if (thanhCong) {
    mach.soLoiLienTiep = 0;
    mach.trangThai = "dong";
    return;
  }
  mach.soLoiLienTiep += 1;
  if (mach.trangThai === "nua_mo") {
    mach.trangThai = "mo";
    mach.thoiDiemMoMach = dh.thoiGianHienTai;
    return;
  }
  if (mach.soLoiLienTiep >= mach.nguongLoi) {
    mach.trangThai = "mo";
    mach.thoiDiemMoMach = dh.thoiGianHienTai;
  }
}

const dh = taoDongHoMoPhong();
const mach = taoMachDien(2);
console.log(choPhepGoi(mach));
ghiNhanKetQua(mach, false, dh);
ghiNhanKetQua(mach, false, dh);
console.log(mach.trangThai, choPhepGoi(mach));
```

```typescript title=test
function layTrangThai(m: MachDien): TrangThaiMach { return m.trangThai; }

const dhT = taoDongHoMoPhong();
const machT = taoMachDien(3);

ghiNhanKetQua(machT, false, dhT);
ghiNhanKetQua(machT, false, dhT);
const trangThaiSauHaiLoiT = layTrangThai(machT);
if (trangThaiSauHaiLoiT !== "dong") throw new Error("2 loi, chua toi nguong 3, van phai la dong");
if (choPhepGoi(machT) !== true) throw new Error("dang dong thi phai cho phep goi");

ghiNhanKetQua(machT, false, dhT);
const trangThaiSauBaLoiT = layTrangThai(machT);
if (trangThaiSauBaLoiT !== "mo") throw new Error("du 3 loi lien tiep (nguong 3) phai mo mach");
if (choPhepGoi(machT) !== false) throw new Error("mach mo phai TU CHOI goi ngay, khong cho qua");

const machT2 = taoMachDien(2);
ghiNhanKetQua(machT2, false, dhT);
ghiNhanKetQua(machT2, true, dhT);
const soLoiSauThanhCongT = machT2.soLoiLienTiep;
if (soLoiSauThanhCongT !== 0) throw new Error("mot lan thanh cong phai RESET so loi lien tiep ve 0");
ghiNhanKetQua(machT2, false, dhT);
const trangThaiSauResetT = layTrangThai(machT2);
if (trangThaiSauResetT !== "dong") throw new Error("sau khi reset, 1 loi don le chua du de mo mach (nguong 2)");

const machT3 = taoMachDien(1);
machT3.trangThai = "nua_mo";
ghiNhanKetQua(machT3, false, dhT);
const trangThaiSauThuThatBaiT = layTrangThai(machT3);
if (trangThaiSauThuThatBaiT !== "mo") throw new Error("thu that bai trong nua_mo phai quay lai mo NGAY, khong can dem them nguong");

const machT4 = taoMachDien(1);
machT4.trangThai = "nua_mo";
ghiNhanKetQua(machT4, true, dhT);
const trangThaiSauThuThanhCongT = layTrangThai(machT4);
if (trangThaiSauThuThanhCongT !== "dong") throw new Error("thu thanh cong trong nua_mo phai phuc hoi ve dong");
```

:::hints
- kind: attention
  body: "Nhanh that bai (sau khi da xu ly xong nhanh thanh cong): tang soLoiLienTiep len 1; neu DANG Ở nua_mo thi mo mach NGAY (ghi lai thoiDiemMoMach) roi return; nguoc lai, neu soLoiLienTiep >= nguongLoi thi cung mo mach."
- kind: strategy
  body: "mach.soLoiLienTiep += 1; if (mach.trangThai === 'nua_mo') { mach.trangThai = 'mo'; mach.thoiDiemMoMach = dh.thoiGianHienTai; return; } if (mach.soLoiLienTiep >= mach.nguongLoi) { mach.trangThai = 'mo'; mach.thoiDiemMoMach = dh.thoiGianHienTai; }"
- kind: one-line
  body: "mach.soLoiLienTiep += 1; if (mach.trangThai === 'nua_mo') { mach.trangThai = 'mo'; mach.thoiDiemMoMach = dh.thoiGianHienTai; return; } if (mach.soLoiLienTiep >= mach.nguongLoi) { mach.trangThai = 'mo'; mach.thoiDiemMoMach = dh.thoiGianHienTai; }"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "mo false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mạch mở đúng lúc, từ chối tức thời, VÀ biết thử lại sau khi đủ thời gian
chờ. Nhưng "thử lại" chỉ LÀ một request DUY nhất — nếu chính request đó
gặp lỗi TẠM thời (một cú chớp mạng, không phải downstream thật sự chết),
liệu có nên bỏ CUỘC ngay, hay thử lại vài lần nữa trước khi kết luận?
::::

::::reflect{#nghi-lai}
`ghiNhanKetQua` không hề đối xứng: MỞ mạch cần TỚI `nguongLoi` lỗi liên
tiếp (một ngưỡng có chủ đích, tránh phản ứng thái quá với MỘT lỗi đơn lẻ),
nhưng thoát KHỎI `"nua_mo"` bằng thất bại chỉ cần ĐÚNG một lần — vì đây LÀ
cơ hội mạch tự nguyện trao, không phải TRẠNG thái bình thường. Điểm quan
trọng nhất không nằm Ở logic đếm — nó nằm Ở `choPhepGoi`: khi mạch `"mo"`,
hệ thống KHÔNG hề chạm tới downstream, không hề tốn một mili-giây timeout
nào. Từ chối NGAY LÀ hành động bảo vệ, không phải LÀ một thất bại.
::::

::::checkpoint{mastery=0.73}
::::
