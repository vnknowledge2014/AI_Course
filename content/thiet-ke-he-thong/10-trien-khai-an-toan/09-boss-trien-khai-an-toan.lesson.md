---
id: thiet-ke-he-thong.trien-khai-an-toan.boss-trien-khai-an-toan
title: "BOSS — pipeline triển khai hoàn chỉnh"
summary: "chayPipelineTrienKhai(cacInstance, dh, trangThaiCanary, cacBuocDuLieu, nguong, buocTang) rap DUNG ba manh: loc instance readiness=true (bai 4, traffic CHI toi day), roi chay canary co auto-rollback tung buoc (bai 8) tren nhung instance DO. Vi du 3 instance (2 san sang, 1 con warm up) + 2 buoc du lieu AN toan -> soInstanceSanSang=2, tyLeCanaryCuoi=50; kich ban VO (buoc giua vuot nguong) -> DUNG NGAY, tyLeCanaryCuoi=0, daRollback=true. Feature flag (bai 3) cho MOT tinh nang con VAN hoat dong dung DOC LAP hoan toan, khong phu thuoc trang thai canary/rollback."
locale: vi
track: thiet-ke-he-thong
module: trien-khai-an-toan
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [sd.boss-trien-khai-an-toan]
requires: [sd.canary-ket-hop-rollback-tu-dong]
concepts: [sd.boss-trien-khai-an-toan]
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
Tám bài — ba chiến lược deploy, canary tăng dần, feature flag,
readiness probe, expand-contract, load test, rollback tự động, VÀ
canary kết hợp rollback. Giờ LÀ lúc ráp ba mảnh CỐT lõi vào MỘT pipeline
đầu-cuối: chỉ route traffic tới instance sẵn sàng, tăng canary có tự
động rollback trên đúng những instance đó, VÀ một feature flag riêng
vẫn vận hành độc lập suốt quá trình.
::::

::::explain{#rap_pipeline}
`chayPipelineTrienKhai` đi qua ĐÚNG hai trạm, theo thứ TỰ: trạm MỘT —
lọc instance có readiness `true` (bài 4), CHỈ những instance ĐÓ mới
được coi LÀ đang phục vụ traffic. Trạm HAI — chạy canary từng bước có
auto-rollback (bài 8) TRÊN nền instance đã lọc: tăng dần nếu tỷ lệ lỗi
dưới ngưỡng, LÙI về `0` VÀ dừng NGAY nếu vượt Ở bất kỳ bước nào:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface Instance { id: string; conSong: boolean; thoiDiemBatDau: number; thoiGianWarmUpMs: number; }
function kiemTraReadiness(instance: Instance, dh: DongHoMoPhong): boolean {
  if (!instance.conSong) return false;
  return dh.thoiGianHienTai - instance.thoiDiemBatDau >= instance.thoiGianWarmUpMs;
}
function locInstanceSanSang(cacInstance: Instance[], dh: DongHoMoPhong): Instance[] {
  return cacInstance.filter((i) => kiemTraReadiness(i, dh));
}

interface FeatureFlag { ten: string; batToanBo: boolean; danhSachUserBat: Set<string>; tyLePhanTramBat: number; }
function hashOnDinh(chuoi: string): number {
  let tong = 0;
  for (let i = 0; i < chuoi.length; i++) tong += chuoi.charCodeAt(i);
  return tong % 100;
}
function kiemTraBat(flag: FeatureFlag, userId: string): boolean {
  if (flag.batToanBo) return true;
  if (flag.danhSachUserBat.has(userId)) return true;
  return hashOnDinh(userId) < flag.tyLePhanTramBat;
}

interface TrangThaiCanaryRollback { tyLeHienTai: number; daRollback: boolean; }
type KetQuaBuoc = "da_tang" | "rollback_ve_0";
function xuLyMotBuocCanary(
  trangThai: TrangThaiCanaryRollback,
  soLoiQuanSat: number,
  soRequestQuanSat: number,
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaBuoc {
  const tyLeLoi = soRequestQuanSat === 0 ? 0 : soLoiQuanSat / soRequestQuanSat;
  if (tyLeLoi > nguongTyLeLoi) {
    trangThai.tyLeHienTai = 0;
    trangThai.daRollback = true;
    return "rollback_ve_0";
  }
  trangThai.tyLeHienTai = Math.min(100, trangThai.tyLeHienTai + buocTang);
  return "da_tang";
}

interface DuLieuBuoc { soLoi: number; soRequest: number; }
interface KetQuaPipeline {
  soInstanceSanSang: number;
  cacKetQuaBuoc: KetQuaBuoc[];
  tyLeCanaryCuoi: number;
  daRollback: boolean;
}

function chayPipelineTrienKhai(
  cacInstance: Instance[],
  dh: DongHoMoPhong,
  trangThaiCanary: TrangThaiCanaryRollback,
  cacBuocDuLieu: DuLieuBuoc[],
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaPipeline {
  const sanSang = locInstanceSanSang(cacInstance, dh);
  const cacKetQuaBuoc: KetQuaBuoc[] = [];
  for (const buoc of cacBuocDuLieu) {
    const kq = xuLyMotBuocCanary(trangThaiCanary, buoc.soLoi, buoc.soRequest, nguongTyLeLoi, buocTang);
    cacKetQuaBuoc.push(kq);
    if (kq === "rollback_ve_0") break;
  }
  return {
    soInstanceSanSang: sanSang.length,
    cacKetQuaBuoc,
    tyLeCanaryCuoi: trangThaiCanary.tyLeHienTai,
    daRollback: trangThaiCanary.daRollback,
  };
}

const dh = taoDongHoMoPhong();
const cacInstance: Instance[] = [
  { id: "i-1", conSong: true, thoiDiemBatDau: -5000, thoiGianWarmUpMs: 3000 },
  { id: "i-2", conSong: true, thoiDiemBatDau: -5000, thoiGianWarmUpMs: 3000 },
  { id: "i-3", conSong: true, thoiDiemBatDau: 0, thoiGianWarmUpMs: 3000 },
];
const canary = { tyLeHienTai: 0, daRollback: false };
const cacBuoc: DuLieuBuoc[] = [
  { soLoi: 2, soRequest: 1000 },
  { soLoi: 3, soRequest: 1000 },
];

const ketQua = chayPipelineTrienKhai(cacInstance, dh, canary, cacBuoc, 0.05, 25);
console.log("so instance SAN SANG luc bat dau (i-3 con warm up):", ketQua.soInstanceSanSang);
console.log("cac ket qua tung buoc canary:", JSON.stringify(ketQua.cacKetQuaBuoc));
console.log("ty le canary cuoi:", ketQua.tyLeCanaryCuoi);
console.log("da rollback:", ketQua.daRollback);
```

```text title=readonly
so instance SAN SANG luc bat dau (i-3 con warm up): 2
cac ket qua tung buoc canary: ["da_tang","da_tang"]
ty le canary cuoi: 50
da rollback: false
```

`i-3` vừa khởi động (`thoiDiemBatDau=0`, đồng hồ vẫn Ở `0`) nên CHƯA
sẵn sàng — chỉ `i-1` VÀ `i-2` được tính LÀ `soInstanceSanSang=2`. Cả
hai bước canary đều dưới ngưỡng lỗi, tăng đều `25` mỗi bước, kết thúc
Ở `50` — pipeline chạy TRỌN vẹn, không có gì bất thường.
::::

::::example{#pipeline-khi-vo-va-flag-doc-lap}
Khi một bước canary vượt ngưỡng, pipeline lùi VỀ `0` VÀ dừng — giống
hệt bài 8. Nhưng một feature flag cho tính năng CON, không liên quan
gì tới quy trình canary, vẫn tiếp tục hoạt động BÌNH thường, cho DÙ
canary vừa bị rollback:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }

interface Instance { id: string; conSong: boolean; thoiDiemBatDau: number; thoiGianWarmUpMs: number; }
function kiemTraReadiness(instance: Instance, dh: DongHoMoPhong): boolean {
  if (!instance.conSong) return false;
  return dh.thoiGianHienTai - instance.thoiDiemBatDau >= instance.thoiGianWarmUpMs;
}
function locInstanceSanSang(cacInstance: Instance[], dh: DongHoMoPhong): Instance[] {
  return cacInstance.filter((i) => kiemTraReadiness(i, dh));
}

interface FeatureFlag { ten: string; batToanBo: boolean; danhSachUserBat: Set<string>; tyLePhanTramBat: number; }
function hashOnDinh(chuoi: string): number {
  let tong = 0;
  for (let i = 0; i < chuoi.length; i++) tong += chuoi.charCodeAt(i);
  return tong % 100;
}
function kiemTraBat(flag: FeatureFlag, userId: string): boolean {
  if (flag.batToanBo) return true;
  if (flag.danhSachUserBat.has(userId)) return true;
  return hashOnDinh(userId) < flag.tyLePhanTramBat;
}

interface TrangThaiCanaryRollback { tyLeHienTai: number; daRollback: boolean; }
type KetQuaBuoc = "da_tang" | "rollback_ve_0";
function xuLyMotBuocCanary(
  trangThai: TrangThaiCanaryRollback,
  soLoiQuanSat: number,
  soRequestQuanSat: number,
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaBuoc {
  const tyLeLoi = soRequestQuanSat === 0 ? 0 : soLoiQuanSat / soRequestQuanSat;
  if (tyLeLoi > nguongTyLeLoi) {
    trangThai.tyLeHienTai = 0;
    trangThai.daRollback = true;
    return "rollback_ve_0";
  }
  trangThai.tyLeHienTai = Math.min(100, trangThai.tyLeHienTai + buocTang);
  return "da_tang";
}

interface DuLieuBuoc { soLoi: number; soRequest: number; }
interface KetQuaPipeline {
  soInstanceSanSang: number;
  cacKetQuaBuoc: KetQuaBuoc[];
  tyLeCanaryCuoi: number;
  daRollback: boolean;
}

function chayPipelineTrienKhai(
  cacInstance: Instance[],
  dh: DongHoMoPhong,
  trangThaiCanary: TrangThaiCanaryRollback,
  cacBuocDuLieu: DuLieuBuoc[],
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaPipeline {
  const sanSang = locInstanceSanSang(cacInstance, dh);
  const cacKetQuaBuoc: KetQuaBuoc[] = [];
  for (const buoc of cacBuocDuLieu) {
    const kq = xuLyMotBuocCanary(trangThaiCanary, buoc.soLoi, buoc.soRequest, nguongTyLeLoi, buocTang);
    cacKetQuaBuoc.push(kq);
    if (kq === "rollback_ve_0") break;
  }
  return {
    soInstanceSanSang: sanSang.length,
    cacKetQuaBuoc,
    tyLeCanaryCuoi: trangThaiCanary.tyLeHienTai,
    daRollback: trangThaiCanary.daRollback,
  };
}

// tat ca instance da SAN SANG tu truoc (khong ai con warm up)
const dh = taoDongHoMoPhong();
const cacInstance: Instance[] = [
  { id: "i-1", conSong: true, thoiDiemBatDau: -10000, thoiGianWarmUpMs: 3000 },
  { id: "i-2", conSong: true, thoiDiemBatDau: -10000, thoiGianWarmUpMs: 3000 },
];
const canary: TrangThaiCanaryRollback = { tyLeHienTai: 0, daRollback: false };
const cacBuocVo: DuLieuBuoc[] = [
  { soLoi: 1, soRequest: 1000 },
  { soLoi: 90, soRequest: 1000 },
  { soLoi: 1, soRequest: 1000 },
];

const ketQuaVo = chayPipelineTrienKhai(cacInstance, dh, canary, cacBuocVo, 0.05, 20);
console.log("so instance san sang:", ketQuaVo.soInstanceSanSang);
console.log("cac ket qua canary (DUNG o buoc 2, KHONG chay buoc 3):", JSON.stringify(ketQuaVo.cacKetQuaBuoc));
console.log("ty le canary cuoi (rollback VE 0):", ketQuaVo.tyLeCanaryCuoi);
console.log("da rollback:", ketQuaVo.daRollback);

// feature flag cho mot tinh nang CON, doc lap hoan toan voi canary/rollback ben tren
const flagPhu: FeatureFlag = { ten: "thong-bao-moi", batToanBo: false, danhSachUserBat: new Set(), tyLePhanTramBat: 10 };
console.log("feature flag rieng, user 'an' (hash 7, duoi 10%), du canary DA rollback:", kiemTraBat(flagPhu, "an"));
console.log("feature flag rieng, user 'binh' (hash 17, tren 10%):", kiemTraBat(flagPhu, "binh"));
```

```text title=readonly
so instance san sang: 2
cac ket qua canary (DUNG o buoc 2, KHONG chay buoc 3): ["da_tang","rollback_ve_0"]
ty le canary cuoi (rollback VE 0): 0
da rollback: true
feature flag rieng, user 'an' (hash 7, duoi 10%), du canary DA rollback: true
feature flag rieng, user 'binh' (hash 17, tren 10%): false
```

Bước hai (`90` lỗi trên `1000`, tỷ lệ `9%`) vượt ngưỡng `5%` — canary
LÙI về `0` VÀ dừng, bước ba KHÔNG bao giờ chạy tới. Nhưng
`kiemTraBat(flagPhu, "an")` vẫn trả về `true`, y hệt như nếu canary
CHƯA từng rollback — feature flag không hề đọc `trangThaiCanary`, VÀ
`trangThaiCanary` cũng không hề đọc `flagPhu`. Hai trục kiểm soát
NẰM cạnh nhau, không phụ thuộc LẪN nhau.
::::

::::predict{#doan-tich-hop-boss commitOnce}
Ba instance: `i-1` (còn sống, đã qua warm-up từ lâu), `i-2` (đã CHẾT,
`conSong=false`), `i-3` (còn sống, VỪA khởi động, đồng hồ đang Ở `0`
đúng lúc `i-3` bắt đầu). Chạy pipeline với MỘT bước dữ liệu duy nhất:
`5` lỗi trên `100` request, ngưỡng `0.05` (tức tỷ lệ lỗi ĐÚNG BẰNG
ngưỡng). `soInstanceSanSang` VÀ `cacKetQuaBuoc` LÀ gì?

:::opt{correct}
`soInstanceSanSang = 1`, `cacKetQuaBuoc = ["da_tang"]` — chỉ `i-1` qua
được readiness (`i-2` chết nên loại NGAY, `i-3` vừa khởi động nên
chưa đủ warm-up); tỷ lệ lỗi `5/100 = 0.05` bằng đúng ngưỡng, điều kiện
`>` cho `false` nên bước NÀY vẫn LÀ `"da_tang"`, không rollback
:::
:::opt
`soInstanceSanSang = 1`, `cacKetQuaBuoc = ["rollback_ve_0"]` — tỷ lệ
lỗi chạm đúng ngưỡng LÀ đủ nguy hiểm để pipeline coi LÀ vượt ngưỡng VÀ
rollback ngay Ở bước duy nhất này
::why
Nhầm "readiness đúng" VỚI "canary cũng phải rollback theo cùng cách
suy luận chặt chẽ" — nhưng hai điều kiện `>=` (readiness, bài 4) VÀ
`>` (rollback canary, bài 7-8) LÀ HAI phép so sánh KHÁC nhau, được
thiết kế có chủ đích khác nhau, không thể suy luận chéo từ cái này
sang cái kia.

Chỗ lệch: readiness đúng LÀ `1` (chỉ `i-1`) — điều đó không sai. Nhưng
canary dùng điều kiện `tyLeLoi > nguongTyLeLoi`; với `0.05 > 0.05`
LÀ `false`, bước NÀY KHÔNG kích hoạt rollback, VÀ trả về `"da_tang"`.
Việc readiness dùng `>=` không hề kéo theo canary cũng phải dùng
`>=` — hai hàm hoàn toàn độc lập, MỖI hàm giữ điều kiện đã được
thiết kế riêng từ bài 4 VÀ bài 7-8.
::
:::
::::

::::code{#viet_chay_pipeline_trien_khai}
Hoàn thiện `chayPipelineTrienKhai` — lọc instance sẵn sàng (đã có
sẵn), RỒI với mỗi bước dữ liệu: gọi `xuLyMotBuocCanary`, đẩy kết quả
vào mảng, dừng vòng lặp NGAY nếu vừa rollback. Trả về kết quả tổng hợp.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface Instance { id: string; conSong: boolean; thoiDiemBatDau: number; thoiGianWarmUpMs: number; }
function kiemTraReadiness(instance: Instance, dh: DongHoMoPhong): boolean {
  if (!instance.conSong) return false;
  return dh.thoiGianHienTai - instance.thoiDiemBatDau >= instance.thoiGianWarmUpMs;
}
function locInstanceSanSang(cacInstance: Instance[], dh: DongHoMoPhong): Instance[] {
  return cacInstance.filter((i) => kiemTraReadiness(i, dh));
}

interface FeatureFlag { ten: string; batToanBo: boolean; danhSachUserBat: Set<string>; tyLePhanTramBat: number; }
function hashOnDinh(chuoi: string): number {
  let tong = 0;
  for (let i = 0; i < chuoi.length; i++) tong += chuoi.charCodeAt(i);
  return tong % 100;
}
function kiemTraBat(flag: FeatureFlag, userId: string): boolean {
  if (flag.batToanBo) return true;
  if (flag.danhSachUserBat.has(userId)) return true;
  return hashOnDinh(userId) < flag.tyLePhanTramBat;
}

interface TrangThaiCanaryRollback { tyLeHienTai: number; daRollback: boolean; }
type KetQuaBuoc = "da_tang" | "rollback_ve_0";
function xuLyMotBuocCanary(
  trangThai: TrangThaiCanaryRollback,
  soLoiQuanSat: number,
  soRequestQuanSat: number,
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaBuoc {
  const tyLeLoi = soRequestQuanSat === 0 ? 0 : soLoiQuanSat / soRequestQuanSat;
  if (tyLeLoi > nguongTyLeLoi) {
    trangThai.tyLeHienTai = 0;
    trangThai.daRollback = true;
    return "rollback_ve_0";
  }
  trangThai.tyLeHienTai = Math.min(100, trangThai.tyLeHienTai + buocTang);
  return "da_tang";
}

interface DuLieuBuoc { soLoi: number; soRequest: number; }
interface KetQuaPipeline {
  soInstanceSanSang: number;
  cacKetQuaBuoc: KetQuaBuoc[];
  tyLeCanaryCuoi: number;
  daRollback: boolean;
}

function chayPipelineTrienKhai(
  cacInstance: Instance[],
  dh: DongHoMoPhong,
  trangThaiCanary: TrangThaiCanaryRollback,
  cacBuocDuLieu: DuLieuBuoc[],
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaPipeline {
  const sanSang = locInstanceSanSang(cacInstance, dh);
  const cacKetQuaBuoc: KetQuaBuoc[] = [];
  ___
  return {
    soInstanceSanSang: sanSang.length,
    cacKetQuaBuoc,
    tyLeCanaryCuoi: trangThaiCanary.tyLeHienTai,
    daRollback: trangThaiCanary.daRollback,
  };
}

const dhX = taoDongHoMoPhong();
const cacInstanceX: Instance[] = [{ id: "x-1", conSong: true, thoiDiemBatDau: -1000, thoiGianWarmUpMs: 1000 }];
const canaryX: TrangThaiCanaryRollback = { tyLeHienTai: 0, daRollback: false };
const ketQuaX = chayPipelineTrienKhai(cacInstanceX, dhX, canaryX, [{ soLoi: 1, soRequest: 100 }], 0.05, 30);
console.log(ketQuaX.soInstanceSanSang, ketQuaX.tyLeCanaryCuoi);
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface Instance { id: string; conSong: boolean; thoiDiemBatDau: number; thoiGianWarmUpMs: number; }
function kiemTraReadiness(instance: Instance, dh: DongHoMoPhong): boolean {
  if (!instance.conSong) return false;
  return dh.thoiGianHienTai - instance.thoiDiemBatDau >= instance.thoiGianWarmUpMs;
}
function locInstanceSanSang(cacInstance: Instance[], dh: DongHoMoPhong): Instance[] {
  return cacInstance.filter((i) => kiemTraReadiness(i, dh));
}

interface FeatureFlag { ten: string; batToanBo: boolean; danhSachUserBat: Set<string>; tyLePhanTramBat: number; }
function hashOnDinh(chuoi: string): number {
  let tong = 0;
  for (let i = 0; i < chuoi.length; i++) tong += chuoi.charCodeAt(i);
  return tong % 100;
}
function kiemTraBat(flag: FeatureFlag, userId: string): boolean {
  if (flag.batToanBo) return true;
  if (flag.danhSachUserBat.has(userId)) return true;
  return hashOnDinh(userId) < flag.tyLePhanTramBat;
}

interface TrangThaiCanaryRollback { tyLeHienTai: number; daRollback: boolean; }
type KetQuaBuoc = "da_tang" | "rollback_ve_0";
function xuLyMotBuocCanary(
  trangThai: TrangThaiCanaryRollback,
  soLoiQuanSat: number,
  soRequestQuanSat: number,
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaBuoc {
  const tyLeLoi = soRequestQuanSat === 0 ? 0 : soLoiQuanSat / soRequestQuanSat;
  if (tyLeLoi > nguongTyLeLoi) {
    trangThai.tyLeHienTai = 0;
    trangThai.daRollback = true;
    return "rollback_ve_0";
  }
  trangThai.tyLeHienTai = Math.min(100, trangThai.tyLeHienTai + buocTang);
  return "da_tang";
}

interface DuLieuBuoc { soLoi: number; soRequest: number; }
interface KetQuaPipeline {
  soInstanceSanSang: number;
  cacKetQuaBuoc: KetQuaBuoc[];
  tyLeCanaryCuoi: number;
  daRollback: boolean;
}

function chayPipelineTrienKhai(
  cacInstance: Instance[],
  dh: DongHoMoPhong,
  trangThaiCanary: TrangThaiCanaryRollback,
  cacBuocDuLieu: DuLieuBuoc[],
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaPipeline {
  const sanSang = locInstanceSanSang(cacInstance, dh);
  const cacKetQuaBuoc: KetQuaBuoc[] = [];
  for (const buoc of cacBuocDuLieu) {
    const kq = xuLyMotBuocCanary(trangThaiCanary, buoc.soLoi, buoc.soRequest, nguongTyLeLoi, buocTang);
    cacKetQuaBuoc.push(kq);
    if (kq === "rollback_ve_0") break;
  }
  return {
    soInstanceSanSang: sanSang.length,
    cacKetQuaBuoc,
    tyLeCanaryCuoi: trangThaiCanary.tyLeHienTai,
    daRollback: trangThaiCanary.daRollback,
  };
}

const dhX = taoDongHoMoPhong();
const cacInstanceX: Instance[] = [{ id: "x-1", conSong: true, thoiDiemBatDau: -1000, thoiGianWarmUpMs: 1000 }];
const canaryX: TrangThaiCanaryRollback = { tyLeHienTai: 0, daRollback: false };
const ketQuaX = chayPipelineTrienKhai(cacInstanceX, dhX, canaryX, [{ soLoi: 1, soRequest: 100 }], 0.05, 30);
console.log(ketQuaX.soInstanceSanSang, ketQuaX.tyLeCanaryCuoi);
```

```typescript title=test
const dhT1 = taoDongHoMoPhong();
const cacInstanceT1: Instance[] = [
  { id: "a", conSong: true, thoiDiemBatDau: -10000, thoiGianWarmUpMs: 2000 },
  { id: "b", conSong: true, thoiDiemBatDau: 0, thoiGianWarmUpMs: 5000 },
  { id: "c", conSong: false, thoiDiemBatDau: -10000, thoiGianWarmUpMs: 2000 },
];
const canaryT1: TrangThaiCanaryRollback = { tyLeHienTai: 0, daRollback: false };
const ketQuaT1 = chayPipelineTrienKhai(
  cacInstanceT1,
  dhT1,
  canaryT1,
  [{ soLoi: 1, soRequest: 1000 }, { soLoi: 2, soRequest: 1000 }],
  0.05,
  20
);
const sanSangDem1 = ketQuaT1.soInstanceSanSang;
if (sanSangDem1 !== 1) throw new Error("chi instance 'a' san sang (b con warm up, c da chet), phai la 1");
const soBuocChay1 = ketQuaT1.cacKetQuaBuoc.length;
if (soBuocChay1 !== 2) throw new Error("khong vo thi phai chay HET ca 2 buoc du lieu");
const tyLeCuoi1 = ketQuaT1.tyLeCanaryCuoi;
if (tyLeCuoi1 !== 40) throw new Error("tang 20 moi buoc, sau 2 buoc thanh cong phai la 40");
const rollback1 = ketQuaT1.daRollback;
if (rollback1 !== false) throw new Error("khong vo thi daRollback phai la false");

const dhT2 = taoDongHoMoPhong();
tienThoiGian(dhT2, 5000);
const cacInstanceT2: Instance[] = [
  { id: "d", conSong: true, thoiDiemBatDau: 0, thoiGianWarmUpMs: 1000 },
  { id: "e", conSong: true, thoiDiemBatDau: 0, thoiGianWarmUpMs: 1000 },
];
const canaryT2: TrangThaiCanaryRollback = { tyLeHienTai: 0, daRollback: false };
const ketQuaT2 = chayPipelineTrienKhai(
  cacInstanceT2,
  dhT2,
  canaryT2,
  [
    { soLoi: 1, soRequest: 1000 },
    { soLoi: 60, soRequest: 1000 },
    { soLoi: 1, soRequest: 1000 },
  ],
  0.05,
  20
);
const sanSangDem2 = ketQuaT2.soInstanceSanSang;
if (sanSangDem2 !== 2) throw new Error("ca hai instance da qua thoi gian warm up (t=5000 >= 1000), phai san sang het");
const soBuocChay2 = ketQuaT2.cacKetQuaBuoc.length;
if (soBuocChay2 !== 2) throw new Error("phai DUNG NGAY o buoc vo (buoc 2), KHONG chay buoc 3 (chi con 2 phan tu)");
const tyLeCuoi2 = ketQuaT2.tyLeCanaryCuoi;
if (tyLeCuoi2 !== 0) throw new Error("sau khi vo, ty le canary phai VE DUNG 0");
const rollback2 = ketQuaT2.daRollback;
if (rollback2 !== true) throw new Error("sau khi vo, daRollback phai la true");

const flagPhuT: FeatureFlag = { ten: "t-flag", batToanBo: false, danhSachUserBat: new Set(["vip"]), tyLePhanTramBat: 10 };
if (kiemTraBat(flagPhuT, "vip") !== true) throw new Error("user trong danh sach rieng phai duoc bat, khong lien quan gi toi trang thai canary");
if (kiemTraBat(flagPhuT, "an") !== true) throw new Error("hash cua 'an' la 7, duoi 10%, phai duoc bat");
if (kiemTraBat(flagPhuT, "binh") !== false) throw new Error("hash cua 'binh' la 17, tren 10%, phai bi tat");
```

:::hints
- kind: attention
  body: "Sau khi da co sanSang, dung MOT vong for qua cacBuocDuLieu: goi xuLyMotBuocCanary(trangThaiCanary, buoc.soLoi, buoc.soRequest, nguongTyLeLoi, buocTang), day ket qua vao cacKetQuaBuoc, break neu ket qua la rollback_ve_0. Doan return da co san, khong can sua."
- kind: strategy
  body: "for (const buoc of cacBuocDuLieu) { const kq = xuLyMotBuocCanary(trangThaiCanary, buoc.soLoi, buoc.soRequest, nguongTyLeLoi, buocTang); cacKetQuaBuoc.push(kq); if (kq === 'rollback_ve_0') break; }"
- kind: one-line
  body: "for (const buoc of cacBuocDuLieu) { const kq = xuLyMotBuocCanary(trangThaiCanary, buoc.soLoi, buoc.soRequest, nguongTyLeLoi, buocTang); cacKetQuaBuoc.push(kq); if (kq === 'rollback_ve_0') break; }"
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "1 30"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Readiness lọc TRAFFIC, canary tăng dần có tự bảo vệ, feature flag kiểm
soát một trục HOÀN toàn riêng — ba mảnh RÁP đúng thứ tự thành MỘT quy
trình triển khai không có khoảng trống nào để một thay đổi tồi lan
rộng trước khi bị chặn lại. Track "Triển khai an toàn" đã xong — VÀ
với đó, câu hỏi "làm sao ĐƯA thay đổi vào production" đã có câu trả
lời BẰNG số liệu, không chỉ bằng mô tả.
::::

::::reflect{#nghi-lai}
`chayPipelineTrienKhai` không hề PHÁT minh thêm khái niệm nào MỚI —
nó gọi ĐÚNG những hàm đã xây RIÊNG lẻ Ở tám bài trước, theo đúng thứ
tự: lọc SẴN sàng trước (bài 4), rồi mới chạy canary có auto-rollback
(bài 8, chính nó LÀ sự kết hợp của bài 2 VÀ bài 7). Feature flag (bài
3) đứng CẠNH, không xen VÀO luồng chính — VÀ chính sự tách BIỆT đó mới
LÀ điểm mấu chốt: một pipeline triển khai an toàn không PHẢI LÀ một
khối logic khổng lồ, mà LÀ nhiều quyết định NHỎ, mỗi quyết định chỉ
nhìn vào đúng phần dữ liệu nó CẦN, ráp lại theo một thứ tự có chủ đích.
::::

::::checkpoint{mastery=0.86}
::::
