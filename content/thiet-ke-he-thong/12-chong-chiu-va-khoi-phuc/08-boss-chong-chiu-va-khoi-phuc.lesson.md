---
id: thiet-ke-he-thong.chong-chiu-va-khoi-phuc.boss-chong-chiu-va-khoi-phuc
title: "BOSS — ráp circuit breaker + retry + bulkhead + graceful degradation, và tổng kết Realm 7"
summary: "goiDichVuHaNguon rap DUNG bon co che theo THU TU: kiem tra choPhepGoi (bai 1) TRUOC -- mach mo thi DUNG NGAY, tra ve du_phong (bai 4), khong dung bulkhead, khong goi gi ca; mach dong/nua_mo thi xin tai nguyen qua xinTaiNguyen (bai 3) -- het cho thi tu_choi_tai_nguyen, KHONG anh huong mach; con neu duoc cap thi thu goi downstream (mo phong bang mang ketQuaCacLanGoi) toi soLanThuToiDa lan, MOI lan that bai deu cong don tinhThoiGianChoRetry (bai 2) vao tongThoiGianChoMs; sau CUNG luon tra lai bulkhead va ghiNhanKetQua (bai 1) DU thanh cong hay that bai. Vi du: 2 loi roi thanh cong lan 3 (seed=1, doTreCoBanMs=50) cho tongThoiGianChoMs=178, mach van dong; mach dang mo thi soLanThu=0 va bulkhead khong he bi dung toi."
locale: vi
track: thiet-ke-he-thong
module: chong-chiu-va-khoi-phuc
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [sd.boss-chong-chiu-va-khoi-phuc]
requires: [sd.disaster-recovery-active-passive-vs-active-active]
concepts: [sd.boss-chong-chiu-va-khoi-phuc]
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
Bảy bài — mạch ngắt ba trạng thái, retry với backoff VÀ jitter, bulkhead
cô lập theo nhóm, graceful degradation, rate limit bảo vệ downstream,
backup/restore có xác thực, VÀ đa khu vực. Giờ LÀ lúc ráp BỐN mảnh đầu
tiên (mạch ngắt, retry, bulkhead, dự phòng) thành MỘT pipeline gọi
downstream DUY nhất — VÀ đây LÀ bài CUỐI cùng của TOÀN bộ Realm 7.
::::

::::explain{#rap-pipeline-chiu-loi}
`goiDichVuHaNguon` đi qua ĐÚNG các trạm THEO thứ tự: kiểm tra `choPhepGoi`
(bài 1) trước TIÊN — mạch mở thì dừng NGAY, dùng dự phòng; mạch cho phép
thì xin tài nguyên qua bulkhead (bài 3); rồi mới thử gọi downstream, retry
VỚI backoff (bài 2) nếu lỗi tạm thời; CUỐI cùng luôn trả lại bulkhead VÀ
cập nhật mạch (bài 1) bằng kết quả CUỐI cùng:

```typescript title=readonly
type TrangThaiMach = "dong" | "mo" | "nua_mo";
interface MachDien { trangThai: TrangThaiMach; soLoiLienTiep: number; nguongLoi: number; thoiDiemMoMach: number; }
function taoMachDien(nguongLoi: number): MachDien { return { trangThai: "dong", soLoiLienTiep: 0, nguongLoi, thoiDiemMoMach: 0 }; }
function choPhepGoi(mach: MachDien): boolean { return mach.trangThai !== "mo"; }
function ghiNhanKetQua(mach: MachDien, thanhCong: boolean, dh: DongHoMoPhong): void {
  if (thanhCong) { mach.soLoiLienTiep = 0; mach.trangThai = "dong"; return; }
  mach.soLoiLienTiep += 1;
  if (mach.trangThai === "nua_mo") { mach.trangThai = "mo"; mach.thoiDiemMoMach = dh.thoiGianHienTai; return; }
  if (mach.soLoiLienTiep >= mach.nguongLoi) { mach.trangThai = "mo"; mach.thoiDiemMoMach = dh.thoiGianHienTai; }
}

interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }

interface TrangThaiNhom { dangSuDung: number; toiDa: number; }
interface BoDemTaiNguyen { theoNhom: Map<string, TrangThaiNhom>; }
function taoBoDemTaiNguyen(cauHinh: Record<string, number>): BoDemTaiNguyen {
  const theoNhom = new Map<string, TrangThaiNhom>();
  for (const [nhom, toiDa] of Object.entries(cauHinh)) theoNhom.set(nhom, { dangSuDung: 0, toiDa });
  return { theoNhom };
}
function xinTaiNguyen(bo: BoDemTaiNguyen, nhom: string): boolean {
  const trangThai = bo.theoNhom.get(nhom);
  if (trangThai === undefined) return false;
  if (trangThai.dangSuDung >= trangThai.toiDa) return false;
  trangThai.dangSuDung += 1;
  return true;
}
function traTaiNguyen(bo: BoDemTaiNguyen, nhom: string): void {
  const trangThai = bo.theoNhom.get(nhom);
  if (trangThai === undefined) return;
  trangThai.dangSuDung = Math.max(0, trangThai.dangSuDung - 1);
}

function taoSoNguSeed(seed: number): number { return ((seed * 9301 + 49297) % 233280) / 233280; }
function tinhThoiGianChoRetry(lanThu: number, doTreCoBanMs: number, seed: number): number {
  const doTreCoSo = doTreCoBanMs * Math.pow(2, lanThu);
  const heSoNgauNhien = taoSoNguSeed(seed);
  const doTreJitterMs = heSoNgauNhien * doTreCoBanMs;
  return Math.round(doTreCoSo + doTreJitterMs);
}

type LoaiKetQuaGoi = "thanh_cong" | "du_phong" | "tu_choi_tai_nguyen" | "that_bai_sau_retry";
interface KetQuaGoiHaNguon { loai: LoaiKetQuaGoi; danhSach: string[]; soLanThu: number; tongThoiGianChoMs: number; }

function goiDichVuHaNguon(
  mach: MachDien, bo: BoDemTaiNguyen, nhom: string, dh: DongHoMoPhong,
  ketQuaCacLanGoi: boolean[], soLanThuToiDa: number, doTreCoBanMs: number, seed: number,
  dsCaNhanHoa: string[], dsPhoBien: string[]
): KetQuaGoiHaNguon {
  if (!choPhepGoi(mach)) {
    return { loai: "du_phong", danhSach: dsPhoBien, soLanThu: 0, tongThoiGianChoMs: 0 };
  }
  const duocCap = xinTaiNguyen(bo, nhom);
  if (!duocCap) {
    return { loai: "tu_choi_tai_nguyen", danhSach: dsPhoBien, soLanThu: 0, tongThoiGianChoMs: 0 };
  }

  let tongThoiGianChoMs = 0;
  let thanhCong = false;
  let soLanThu = 0;
  for (let i = 0; i < soLanThuToiDa; i++) {
    soLanThu += 1;
    const ketQua = ketQuaCacLanGoi[i] ?? false;
    if (ketQua) { thanhCong = true; break; }
    tongThoiGianChoMs += tinhThoiGianChoRetry(i, doTreCoBanMs, seed + i);
  }

  traTaiNguyen(bo, nhom);
  ghiNhanKetQua(mach, thanhCong, dh);

  return {
    loai: thanhCong ? "thanh_cong" : "that_bai_sau_retry",
    danhSach: thanhCong ? dsCaNhanHoa : dsPhoBien,
    soLanThu, tongThoiGianChoMs,
  };
}

const dsCaNhanHoa = ["giay-chay-bo-X", "ao-thun-Y"];
const dsPhoBien = ["ban-chay-1", "ban-chay-2"];

// KICH BAN A: mach dong, bulkhead con cho, THANH CONG ngay lan dau
const dhA = taoDongHoMoPhong();
const machA = taoMachDien(3);
const boA = taoBoDemTaiNguyen({ thuong: 2 });
const kqA = goiDichVuHaNguon(machA, boA, "thuong", dhA, [true], 3, 50, 1, dsCaNhanHoa, dsPhoBien);
console.log("A) thanh cong ngay lan 1:", JSON.stringify(kqA), "-- mach:", machA.trangThai, "-- bulkhead con dung:", boA.theoNhom.get("thuong")?.dangSuDung);

// KICH BAN B: 2 loi TAM THOI roi thanh cong lan 3 -- retry cuu duoc request
const dhB = taoDongHoMoPhong();
const machB = taoMachDien(3);
const boB = taoBoDemTaiNguyen({ thuong: 2 });
const kqB = goiDichVuHaNguon(machB, boB, "thuong", dhB, [false, false, true], 3, 50, 1, dsCaNhanHoa, dsPhoBien);
console.log("B) 2 loi roi thanh cong lan 3:", JSON.stringify(kqB), "-- mach:", machB.trangThai);
```

```text title=readonly
A) thanh cong ngay lan 1: {"loai":"thanh_cong","danhSach":["giay-chay-bo-X","ao-thun-Y"],"soLanThu":1,"tongThoiGianChoMs":0} -- mach: dong -- bulkhead con dung: 0
B) 2 loi roi thanh cong lan 3: {"loai":"thanh_cong","danhSach":["giay-chay-bo-X","ao-thun-Y"],"soLanThu":3,"tongThoiGianChoMs":178} -- mach: dong
```

Kịch bản A: thành công NGAY lần đầu — `soLanThu=1`, `tongThoiGianChoMs=0`
(không hề CẦN backoff), VÀ bulkhead được TRẢ lại NGAY sau đó (`dangSuDung`
VỀ `0`). Kịch bản B: hai lần lỗi TẠM thời được retry CỨU lại — `soLanThu=3`
(đúng số lần THẬT sự thử), `tongThoiGianChoMs=178` (tổng hai khoảng backoff
Ở `lanThu=0` VÀ `lanThu=1`, VỚI jitter) — VÀ vì lần CUỐI cùng thành công,
`ghiNhanKetQua(mach, true, ...)` giữ mạch Ở `"dong"`.
::::

::::example{#dung-som-o-mach-va-bulkhead}
Pipeline KHÔNG hề đi hết mọi bước trong MỌI trường hợp — nó dừng SỚM
ngay khi trạm đầu (mạch) hoặc trạm hai (bulkhead) đã đủ căn cứ để quyết
định, KHÔNG chạm gì tới các bước SAU:

```typescript title=readonly
type TrangThaiMach = "dong" | "mo" | "nua_mo";
interface MachDien { trangThai: TrangThaiMach; soLoiLienTiep: number; nguongLoi: number; thoiDiemMoMach: number; }
function taoMachDien(nguongLoi: number): MachDien { return { trangThai: "dong", soLoiLienTiep: 0, nguongLoi, thoiDiemMoMach: 0 }; }
function choPhepGoi(mach: MachDien): boolean { return mach.trangThai !== "mo"; }
function ghiNhanKetQua(mach: MachDien, thanhCong: boolean, dh: DongHoMoPhong): void {
  if (thanhCong) { mach.soLoiLienTiep = 0; mach.trangThai = "dong"; return; }
  mach.soLoiLienTiep += 1;
  if (mach.trangThai === "nua_mo") { mach.trangThai = "mo"; mach.thoiDiemMoMach = dh.thoiGianHienTai; return; }
  if (mach.soLoiLienTiep >= mach.nguongLoi) { mach.trangThai = "mo"; mach.thoiDiemMoMach = dh.thoiGianHienTai; }
}
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
interface TrangThaiNhom { dangSuDung: number; toiDa: number; }
interface BoDemTaiNguyen { theoNhom: Map<string, TrangThaiNhom>; }
function taoBoDemTaiNguyen(cauHinh: Record<string, number>): BoDemTaiNguyen {
  const theoNhom = new Map<string, TrangThaiNhom>();
  for (const [nhom, toiDa] of Object.entries(cauHinh)) theoNhom.set(nhom, { dangSuDung: 0, toiDa });
  return { theoNhom };
}
function xinTaiNguyen(bo: BoDemTaiNguyen, nhom: string): boolean {
  const trangThai = bo.theoNhom.get(nhom);
  if (trangThai === undefined) return false;
  if (trangThai.dangSuDung >= trangThai.toiDa) return false;
  trangThai.dangSuDung += 1;
  return true;
}
function traTaiNguyen(bo: BoDemTaiNguyen, nhom: string): void {
  const trangThai = bo.theoNhom.get(nhom);
  if (trangThai === undefined) return;
  trangThai.dangSuDung = Math.max(0, trangThai.dangSuDung - 1);
}
function taoSoNguSeed(seed: number): number { return ((seed * 9301 + 49297) % 233280) / 233280; }
function tinhThoiGianChoRetry(lanThu: number, doTreCoBanMs: number, seed: number): number {
  const doTreCoSo = doTreCoBanMs * Math.pow(2, lanThu);
  const heSoNgauNhien = taoSoNguSeed(seed);
  const doTreJitterMs = heSoNgauNhien * doTreCoBanMs;
  return Math.round(doTreCoSo + doTreJitterMs);
}
type LoaiKetQuaGoi = "thanh_cong" | "du_phong" | "tu_choi_tai_nguyen" | "that_bai_sau_retry";
interface KetQuaGoiHaNguon { loai: LoaiKetQuaGoi; danhSach: string[]; soLanThu: number; tongThoiGianChoMs: number; }
function goiDichVuHaNguon(
  mach: MachDien, bo: BoDemTaiNguyen, nhom: string, dh: DongHoMoPhong,
  ketQuaCacLanGoi: boolean[], soLanThuToiDa: number, doTreCoBanMs: number, seed: number,
  dsCaNhanHoa: string[], dsPhoBien: string[]
): KetQuaGoiHaNguon {
  if (!choPhepGoi(mach)) {
    return { loai: "du_phong", danhSach: dsPhoBien, soLanThu: 0, tongThoiGianChoMs: 0 };
  }
  const duocCap = xinTaiNguyen(bo, nhom);
  if (!duocCap) {
    return { loai: "tu_choi_tai_nguyen", danhSach: dsPhoBien, soLanThu: 0, tongThoiGianChoMs: 0 };
  }
  let tongThoiGianChoMs = 0;
  let thanhCong = false;
  let soLanThu = 0;
  for (let i = 0; i < soLanThuToiDa; i++) {
    soLanThu += 1;
    const ketQua = ketQuaCacLanGoi[i] ?? false;
    if (ketQua) { thanhCong = true; break; }
    tongThoiGianChoMs += tinhThoiGianChoRetry(i, doTreCoBanMs, seed + i);
  }
  traTaiNguyen(bo, nhom);
  ghiNhanKetQua(mach, thanhCong, dh);
  return {
    loai: thanhCong ? "thanh_cong" : "that_bai_sau_retry",
    danhSach: thanhCong ? dsCaNhanHoa : dsPhoBien,
    soLanThu, tongThoiGianChoMs,
  };
}

const dsCaNhanHoa = ["giay-chay-bo-X", "ao-thun-Y"];
const dsPhoBien = ["ban-chay-1", "ban-chay-2"];

// KICH BAN C: mach DANG MO -- du phong NGAY, khong dung bulkhead, khong thu goi
const dhC = taoDongHoMoPhong();
const machC = taoMachDien(3);
machC.trangThai = "mo";
const boC = taoBoDemTaiNguyen({ thuong: 2 });
const kqC = goiDichVuHaNguon(machC, boC, "thuong", dhC, [true], 3, 50, 1, dsCaNhanHoa, dsPhoBien);
console.log("C) mach dang mo, du phong ngay:", JSON.stringify(kqC), "-- bulkhead KHONG bi dung:", boC.theoNhom.get("thuong")?.dangSuDung);

// KICH BAN D: bulkhead HET cho (nhom da day) -- tu choi tai nguyen, mach KHONG bi anh huong
const dhD = taoDongHoMoPhong();
const machD = taoMachDien(3);
const boD = taoBoDemTaiNguyen({ thuong: 1 });
xinTaiNguyen(boD, "thuong"); // chiem het cho duy nhat truoc
const kqD = goiDichVuHaNguon(machD, boD, "thuong", dhD, [true], 3, 50, 1, dsCaNhanHoa, dsPhoBien);
console.log("D) bulkhead het cho:", JSON.stringify(kqD), "-- mach KHONG doi:", machD.trangThai, machD.soLoiLienTiep);
```

```text title=readonly
C) mach dang mo, du phong ngay: {"loai":"du_phong","danhSach":["ban-chay-1","ban-chay-2"],"soLanThu":0,"tongThoiGianChoMs":0} -- bulkhead KHONG bi dung: 0
D) bulkhead het cho: {"loai":"tu_choi_tai_nguyen","danhSach":["ban-chay-1","ban-chay-2"],"soLanThu":0,"tongThoiGianChoMs":0} -- mach KHONG doi: dong 0
```

Kịch bản C: mạch Ở `"mo"` khiến `goiDichVuHaNguon` DỪNG Ở trạm ĐẦU tiên —
`soLanThu=0` (KHÔNG hề thử gọi), bulkhead HOÀN toàn không bị đụng tới.
Kịch bản D: bulkhead hết CHỖ khiến pipeline dừng Ở trạm THỨ hai — nhưng
đây LÀ một vấn đề TÀI nguyên NỘI bộ, không phải LỖI của downstream, nên
mạch giữ NGUYÊN `soLoiLienTiep=0`, KHÔNG hề bị tính LÀ một lần thất bại.
::::

::::predict{#doan-nua-mo-co-dung-bulkhead-khong commitOnce}
Mạch đang Ở `"nua_mo"` (không phải `"mo"`, cũng chưa hoàn toàn `"dong"`).
Gọi `goiDichVuHaNguon` với `ketQuaCacLanGoi=[true]` (lần thử ĐẦU tiên
thành công NGAY). Kết quả `loai` LÀ gì, VÀ pipeline có đi qua bulkhead
không?

:::opt{correct}
`loai="thanh_cong"`, CÓ đi qua bulkhead — `choPhepGoi` trả về `true` cho
CẢ `"dong"` LẪN `"nua_mo"` (chỉ `"mo"` mới bị chặn), nên `"nua_mo"` được
đối XỬ y hệt `"dong"` trong toàn BỘ pipeline này
:::
:::opt
`loai="du_phong"`, KHÔNG đi qua bulkhead — `"nua_mo"` nghĩa LÀ hệ thống
còn nghi NGỜ downstream, nên NÊN dùng phương án dự phòng thay VÌ mạo
hiểm thử THẬT
::why
Nhầm "chưa hoàn toàn tin tưởng" VỚI "phải tránh HOÀN toàn" — nhưng điều
kiện DUY nhất `goiDichVuHaNguon` kiểm tra Ở trạm ĐẦU LÀ `!choPhepGoi(mach)`,
VÀ hàm ĐÓ (từ bài 1) chỉ trả `false` khi `trangThai === "mo"`.

Chỗ lệch: `choPhepGoi` viết `return mach.trangThai !== "mo";` — VỚI
`trangThai = "nua_mo"`, kết quả LÀ `true`, nên `if (!choPhepGoi(mach))`
KHÔNG kích hoạt. Pipeline đi TIẾP tới `xinTaiNguyen` như BÌNH thường —
CHÍNH đây LÀ Ý nghĩa của "nửa mở": cho THỬ một lần THẬT qua TOÀN bộ con
đường bình thường, không phải một nhánh RIÊNG biệt né tránh downstream.
::
:::
::::

::::code{#viet_goi_dich_vu_ha_nguon}
Hoàn thiện phần CÒN lại của `goiDichVuHaNguon` — SAU khi đã xin được tài
nguyên (`duocCap` LÀ `true`): thử lần LƯỢT các lần gọi trong `ketQuaCacLanGoi`
(tối đa `soLanThuToiDa` lần), CỘNG dồn thời gian chờ retry (`tinhThoiGianChoRetry`)
mỗi khi MỘT lần thất bại, dừng NGAY khi thành CÔNG; SAU đó luôn trả lại
bulkhead (`traTaiNguyen`) VÀ ghi nhận kết quả CUỐI cùng vào mạch
(`ghiNhanKetQua`), RỒI trả về `KetQuaGoiHaNguon` tổng hợp.

```typescript title=starter
type TrangThaiMach = "dong" | "mo" | "nua_mo";
interface MachDien { trangThai: TrangThaiMach; soLoiLienTiep: number; nguongLoi: number; thoiDiemMoMach: number; }
function taoMachDien(nguongLoi: number): MachDien { return { trangThai: "dong", soLoiLienTiep: 0, nguongLoi, thoiDiemMoMach: 0 }; }
function choPhepGoi(mach: MachDien): boolean { return mach.trangThai !== "mo"; }
function ghiNhanKetQua(mach: MachDien, thanhCong: boolean, dh: DongHoMoPhong): void {
  if (thanhCong) { mach.soLoiLienTiep = 0; mach.trangThai = "dong"; return; }
  mach.soLoiLienTiep += 1;
  if (mach.trangThai === "nua_mo") { mach.trangThai = "mo"; mach.thoiDiemMoMach = dh.thoiGianHienTai; return; }
  if (mach.soLoiLienTiep >= mach.nguongLoi) { mach.trangThai = "mo"; mach.thoiDiemMoMach = dh.thoiGianHienTai; }
}

interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }

interface TrangThaiNhom { dangSuDung: number; toiDa: number; }
interface BoDemTaiNguyen { theoNhom: Map<string, TrangThaiNhom>; }
function taoBoDemTaiNguyen(cauHinh: Record<string, number>): BoDemTaiNguyen {
  const theoNhom = new Map<string, TrangThaiNhom>();
  for (const [nhom, toiDa] of Object.entries(cauHinh)) theoNhom.set(nhom, { dangSuDung: 0, toiDa });
  return { theoNhom };
}
function xinTaiNguyen(bo: BoDemTaiNguyen, nhom: string): boolean {
  const trangThai = bo.theoNhom.get(nhom);
  if (trangThai === undefined) return false;
  if (trangThai.dangSuDung >= trangThai.toiDa) return false;
  trangThai.dangSuDung += 1;
  return true;
}
function traTaiNguyen(bo: BoDemTaiNguyen, nhom: string): void {
  const trangThai = bo.theoNhom.get(nhom);
  if (trangThai === undefined) return;
  trangThai.dangSuDung = Math.max(0, trangThai.dangSuDung - 1);
}

function taoSoNguSeed(seed: number): number { return ((seed * 9301 + 49297) % 233280) / 233280; }
function tinhThoiGianChoRetry(lanThu: number, doTreCoBanMs: number, seed: number): number {
  const doTreCoSo = doTreCoBanMs * Math.pow(2, lanThu);
  const heSoNgauNhien = taoSoNguSeed(seed);
  const doTreJitterMs = heSoNgauNhien * doTreCoBanMs;
  return Math.round(doTreCoSo + doTreJitterMs);
}

type LoaiKetQuaGoi = "thanh_cong" | "du_phong" | "tu_choi_tai_nguyen" | "that_bai_sau_retry";
interface KetQuaGoiHaNguon { loai: LoaiKetQuaGoi; danhSach: string[]; soLanThu: number; tongThoiGianChoMs: number; }

function goiDichVuHaNguon(
  mach: MachDien, bo: BoDemTaiNguyen, nhom: string, dh: DongHoMoPhong,
  ketQuaCacLanGoi: boolean[], soLanThuToiDa: number, doTreCoBanMs: number, seed: number,
  dsCaNhanHoa: string[], dsPhoBien: string[]
): KetQuaGoiHaNguon {
  if (!choPhepGoi(mach)) {
    return { loai: "du_phong", danhSach: dsPhoBien, soLanThu: 0, tongThoiGianChoMs: 0 };
  }
  const duocCap = xinTaiNguyen(bo, nhom);
  if (!duocCap) {
    return { loai: "tu_choi_tai_nguyen", danhSach: dsPhoBien, soLanThu: 0, tongThoiGianChoMs: 0 };
  }

  ___
}

const dhX = taoDongHoMoPhong();
const machX = taoMachDien(3);
const boX = taoBoDemTaiNguyen({ thuong: 2 });
const ketQuaX = goiDichVuHaNguon(machX, boX, "thuong", dhX, [false, true], 3, 50, 1, ["a"], ["b"]);
console.log(ketQuaX.loai, ketQuaX.soLanThu, boX.theoNhom.get("thuong")?.dangSuDung, machX.trangThai);
```

```typescript title=solution
type TrangThaiMach = "dong" | "mo" | "nua_mo";
interface MachDien { trangThai: TrangThaiMach; soLoiLienTiep: number; nguongLoi: number; thoiDiemMoMach: number; }
function taoMachDien(nguongLoi: number): MachDien { return { trangThai: "dong", soLoiLienTiep: 0, nguongLoi, thoiDiemMoMach: 0 }; }
function choPhepGoi(mach: MachDien): boolean { return mach.trangThai !== "mo"; }
function ghiNhanKetQua(mach: MachDien, thanhCong: boolean, dh: DongHoMoPhong): void {
  if (thanhCong) { mach.soLoiLienTiep = 0; mach.trangThai = "dong"; return; }
  mach.soLoiLienTiep += 1;
  if (mach.trangThai === "nua_mo") { mach.trangThai = "mo"; mach.thoiDiemMoMach = dh.thoiGianHienTai; return; }
  if (mach.soLoiLienTiep >= mach.nguongLoi) { mach.trangThai = "mo"; mach.thoiDiemMoMach = dh.thoiGianHienTai; }
}

interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }

interface TrangThaiNhom { dangSuDung: number; toiDa: number; }
interface BoDemTaiNguyen { theoNhom: Map<string, TrangThaiNhom>; }
function taoBoDemTaiNguyen(cauHinh: Record<string, number>): BoDemTaiNguyen {
  const theoNhom = new Map<string, TrangThaiNhom>();
  for (const [nhom, toiDa] of Object.entries(cauHinh)) theoNhom.set(nhom, { dangSuDung: 0, toiDa });
  return { theoNhom };
}
function xinTaiNguyen(bo: BoDemTaiNguyen, nhom: string): boolean {
  const trangThai = bo.theoNhom.get(nhom);
  if (trangThai === undefined) return false;
  if (trangThai.dangSuDung >= trangThai.toiDa) return false;
  trangThai.dangSuDung += 1;
  return true;
}
function traTaiNguyen(bo: BoDemTaiNguyen, nhom: string): void {
  const trangThai = bo.theoNhom.get(nhom);
  if (trangThai === undefined) return;
  trangThai.dangSuDung = Math.max(0, trangThai.dangSuDung - 1);
}

function taoSoNguSeed(seed: number): number { return ((seed * 9301 + 49297) % 233280) / 233280; }
function tinhThoiGianChoRetry(lanThu: number, doTreCoBanMs: number, seed: number): number {
  const doTreCoSo = doTreCoBanMs * Math.pow(2, lanThu);
  const heSoNgauNhien = taoSoNguSeed(seed);
  const doTreJitterMs = heSoNgauNhien * doTreCoBanMs;
  return Math.round(doTreCoSo + doTreJitterMs);
}

type LoaiKetQuaGoi = "thanh_cong" | "du_phong" | "tu_choi_tai_nguyen" | "that_bai_sau_retry";
interface KetQuaGoiHaNguon { loai: LoaiKetQuaGoi; danhSach: string[]; soLanThu: number; tongThoiGianChoMs: number; }

function goiDichVuHaNguon(
  mach: MachDien, bo: BoDemTaiNguyen, nhom: string, dh: DongHoMoPhong,
  ketQuaCacLanGoi: boolean[], soLanThuToiDa: number, doTreCoBanMs: number, seed: number,
  dsCaNhanHoa: string[], dsPhoBien: string[]
): KetQuaGoiHaNguon {
  if (!choPhepGoi(mach)) {
    return { loai: "du_phong", danhSach: dsPhoBien, soLanThu: 0, tongThoiGianChoMs: 0 };
  }
  const duocCap = xinTaiNguyen(bo, nhom);
  if (!duocCap) {
    return { loai: "tu_choi_tai_nguyen", danhSach: dsPhoBien, soLanThu: 0, tongThoiGianChoMs: 0 };
  }

  let tongThoiGianChoMs = 0;
  let thanhCong = false;
  let soLanThu = 0;
  for (let i = 0; i < soLanThuToiDa; i++) {
    soLanThu += 1;
    const ketQua = ketQuaCacLanGoi[i] ?? false;
    if (ketQua) { thanhCong = true; break; }
    tongThoiGianChoMs += tinhThoiGianChoRetry(i, doTreCoBanMs, seed + i);
  }

  traTaiNguyen(bo, nhom);
  ghiNhanKetQua(mach, thanhCong, dh);

  return {
    loai: thanhCong ? "thanh_cong" : "that_bai_sau_retry",
    danhSach: thanhCong ? dsCaNhanHoa : dsPhoBien,
    soLanThu, tongThoiGianChoMs,
  };
}

const dhX = taoDongHoMoPhong();
const machX = taoMachDien(3);
const boX = taoBoDemTaiNguyen({ thuong: 2 });
const ketQuaX = goiDichVuHaNguon(machX, boX, "thuong", dhX, [false, true], 3, 50, 1, ["a"], ["b"]);
console.log(ketQuaX.loai, ketQuaX.soLanThu, boX.theoNhom.get("thuong")?.dangSuDung, machX.trangThai);
```

```typescript title=test
const dsCaNhanHoaT = ["giay-chay-bo-X", "ao-thun-Y"];
const dsPhoBienT = ["ban-chay-1", "ban-chay-2"];

const dh1T = taoDongHoMoPhong();
const mach1T = taoMachDien(3);
const bo1T = taoBoDemTaiNguyen({ thuong: 2 });
const kq1T = goiDichVuHaNguon(mach1T, bo1T, "thuong", dh1T, [true], 3, 50, 1, dsCaNhanHoaT, dsPhoBienT);
if (kq1T.loai !== "thanh_cong") throw new Error("thanh cong ngay lan dau phai co loai=thanh_cong");
if (kq1T.soLanThu !== 1) throw new Error("thanh cong lan dau chi tinh 1 lan thu");
if (JSON.stringify(kq1T.danhSach) !== JSON.stringify(dsCaNhanHoaT)) throw new Error("thanh cong phai tra ve danh sach CA NHAN HOA");
const soDungSauKB1T = bo1T.theoNhom.get("thuong")?.dangSuDung;
if (soDungSauKB1T !== 0) throw new Error("sau khi xong, bulkhead phai duoc TRA lai (ve 0)");

const dh2T = taoDongHoMoPhong();
const mach2T = taoMachDien(3);
const bo2T = taoBoDemTaiNguyen({ thuong: 2 });
const kq2T = goiDichVuHaNguon(mach2T, bo2T, "thuong", dh2T, [false, false, true], 3, 50, 1, dsCaNhanHoaT, dsPhoBienT);
if (kq2T.loai !== "thanh_cong") throw new Error("thanh cong o lan thu 3 van phai la thanh_cong");
if (kq2T.soLanThu !== 3) throw new Error("phai tinh DU 3 lan thu");
if (kq2T.tongThoiGianChoMs !== 178) throw new Error("tong thoi gian cho (2 lan backoff, lanThu=0 va 1, seed=1 va 2) phai la 178ms");
const trangThaiMach2T = mach2T.trangThai;
if (trangThaiMach2T !== "dong") throw new Error("thanh cong CUOI CUNG phai giu mach o trang thai dong");

const dh3T = taoDongHoMoPhong();
const mach3T = taoMachDien(3);
mach3T.trangThai = "mo";
const bo3T = taoBoDemTaiNguyen({ thuong: 2 });
const kq3T = goiDichVuHaNguon(mach3T, bo3T, "thuong", dh3T, [true], 3, 50, 1, dsCaNhanHoaT, dsPhoBienT);
if (kq3T.loai !== "du_phong") throw new Error("mach dang mo phai tra ve du_phong NGAY");
if (kq3T.soLanThu !== 0) throw new Error("mach mo thi KHONG duoc thu goi downstream lan nao (soLanThu=0)");
const soDungSauKB3T = bo3T.theoNhom.get("thuong")?.dangSuDung;
if (soDungSauKB3T !== 0) throw new Error("mach mo thi bulkhead KHONG duoc dung toi");

const dh4T = taoDongHoMoPhong();
const mach4T = taoMachDien(3);
const bo4T = taoBoDemTaiNguyen({ thuong: 1 });
xinTaiNguyen(bo4T, "thuong");
const kq4T = goiDichVuHaNguon(mach4T, bo4T, "thuong", dh4T, [true], 3, 50, 1, dsCaNhanHoaT, dsPhoBienT);
if (kq4T.loai !== "tu_choi_tai_nguyen") throw new Error("bulkhead het cho phai tra ve tu_choi_tai_nguyen");
const soLoiMach4T = mach4T.soLoiLienTiep;
if (soLoiMach4T !== 0) throw new Error("tu choi vi bulkhead KHONG phai loi cua downstream -- mach KHONG duoc ghi nhan loi");

const dh5T = taoDongHoMoPhong();
const mach5T = taoMachDien(3);
const bo5T = taoBoDemTaiNguyen({ thuong: 2 });
const kq5T = goiDichVuHaNguon(mach5T, bo5T, "thuong", dh5T, [false, false, false], 3, 50, 1, dsCaNhanHoaT, dsPhoBienT);
if (kq5T.loai !== "that_bai_sau_retry") throw new Error("het sach retry van loi phai la that_bai_sau_retry");
if (JSON.stringify(kq5T.danhSach) !== JSON.stringify(dsPhoBienT)) throw new Error("that bai thi phai tra ve danh sach PHO BIEN (du phong)");
const soLoiMach5T = mach5T.soLoiLienTiep;
if (soLoiMach5T !== 1) throw new Error("mot LUOT goi (du co retry ben trong) chi tinh la MOT lan that bai doi voi circuit breaker");
const soDungSauKB5T = bo5T.theoNhom.get("thuong")?.dangSuDung;
if (soDungSauKB5T !== 0) throw new Error("du thanh cong hay that bai, bulkhead cung phai duoc TRA lai");
```

:::hints
- kind: attention
  body: "Sau khi da xin duoc tai nguyen: khoi tao tongThoiGianChoMs=0, thanhCong=false, soLanThu=0; vong lap toi soLanThuToiDa lan, moi vong tang soLanThu, lay ketQua = ketQuaCacLanGoi[i] ?? false, neu true thi thanhCong=true va break, neu false thi cong tinhThoiGianChoRetry(i, doTreCoBanMs, seed+i) vao tongThoiGianChoMs. Sau vong lap: goi traTaiNguyen VA ghiNhanKetQua, roi tra ve object KetQuaGoiHaNguon voi loai/danhSach phu thuoc thanhCong."
- kind: strategy
  body: "let tongThoiGianChoMs = 0; let thanhCong = false; let soLanThu = 0; for (let i = 0; i < soLanThuToiDa; i++) { soLanThu += 1; const ketQua = ketQuaCacLanGoi[i] ?? false; if (ketQua) { thanhCong = true; break; } tongThoiGianChoMs += tinhThoiGianChoRetry(i, doTreCoBanMs, seed + i); } traTaiNguyen(bo, nhom); ghiNhanKetQua(mach, thanhCong, dh); return { loai: thanhCong ? 'thanh_cong' : 'that_bai_sau_retry', danhSach: thanhCong ? dsCaNhanHoa : dsPhoBien, soLanThu, tongThoiGianChoMs };"
- kind: one-line
  body: "Sao chep dung logic o phan Strategy -- vong lap retry cong don tongThoiGianChoMs, sau do traTaiNguyen + ghiNhanKetQua roi tra ve ket qua tong hop."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "thanh_cong 2 0 dong"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn cơ chế — mạch ngắt, retry, bulkhead, dự phòng — giờ hoạt động NHƯ
một hệ thống DUY nhất: từ chối khi cần, thử lại có chừng mực, cô lập tài
nguyên, VÀ luôn để lại một câu trả lời cho người dùng. Realm 7 "Thiết kế
hệ thống" ĐÃ hoàn tất — 120/120.
::::

::::reflect{#nghi-lai}
`goiDichVuHaNguon` không hề PHÁT minh thêm khái niệm MỚI nào — nó gọi
ĐÚNG bốn hàm đã xây riêng lẻ trong bảy bài trước, VÀ để đầu RA của một
trạm chảy THẲNG vào quyết định của trạm kế tiếp: `choPhepGoi` quyết định
có ĐI tiếp hay dừng Ở dự phòng; `xinTaiNguyen` quyết định có đủ CHỖ để
thử; `tinhThoiGianChoRetry` chỉ được gọi khi THẬT sự cần thử lại; VÀ
`ghiNhanKetQua` luôn nhận đúng MỘT kết quả CUỐI cùng, dù bên trong đã thử
bao nhiêu lần.

Nhìn RỘNG hơn, đây LÀ một lời kết cho CẢ Realm 7. T7.1 bắt đầu bằng những
con số THÔ — độ trễ, QPS, chín số chín, rate limit BẢO vệ chính hệ thống
khỏi client. T7.2 đặt chúng vào những HỆ thống thực chiến — rút gọn URL,
newsfeed, vị trí thời gian thực, giao dịch tiền, hạ tầng dữ liệu quy mô
lớn. T7.3 REFRAME lại chính những Ý tưởng đó qua lăng kính hàm: log bất
biến LÀ fold Ở quy mô hệ thống, CRDT LÀ monoid giao hoán, streaming LÀ một
pipeline các phép biến đổi thuần xen giữa các hiệu ứng Ở biên. VÀ T7.4
mang tất cả TRỞ lại mặt đất vận hành — launch AN toàn, quan sát hệ thống
đang chạy, VÀ, Ở chính quest này, chống chịu khi mọi thứ KHÔNG như dự
tính. Không MỘT bài nào trong số ĐÓ đứng một MÌNH: circuit breaker Ở bài
1 quay LẠI làm nền cho graceful degradation Ở bài 4 VÀ cho chính pipeline
này; rate limit bảo vệ downstream Ở bài 5 LÀ chiếc gương lật ngược của
rate limit bảo vệ CHÍNH hệ thống Ở T7.1. Thiết kế hệ thống, xét cho CÙNG,
không phải LÀ một danh sách thuật toán rời rạc — nó LÀ việc biết đúng cơ
chế NÀO cần ráp VỚI cơ chế nào, VÀ theo THỨ tự nào, để một hệ thống vẫn
đứng vững khi một PHẦN của nó không đứng vững nữa.
::::

::::checkpoint{mastery=0.88}
::::
