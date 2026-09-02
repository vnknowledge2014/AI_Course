---
id: ky-nghe-phan-mem.van-hanh.boss-trung-tam-van-hanh
title: "BOSS — Trung tâm vận hành: CI/CD + Logging + Metrics + Alerting"
summary: "trungTamVanHanh(maNguon, logger, soLoi, soTong, nguong) — ghép TRỌN VẸN track: chayPipelineCI (cụm 1) QUYẾT ĐỊNH deploy được không; deploy thành công GHI log có cấu trúc (cụm 3); tinhTyLeLoi tính metrics (cụm 4); vongQuanSat (cụm 6) đọc metrics, tra runbook nếu cần. Pipeline hỏng CHẶN metrics/alerting hoàn toàn — không request nào được tính."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 24
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [vh.gate-boss-observability]
requires: [vh.observability-feedback-loop]
concepts: [vh.gate-boss-observability]
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
Sáu cụm, hai mươi ba bài — pipeline, deploy, logging, metrics,
tracing, alerting. Bài BOSS: ghép TẤT CẢ vào MỘT trung tâm vận
hành nhỏ.
::::

::::explain{#trung-tam-van-hanh}
`trungTamVanHanh` mô phỏng MỘT chu KỲ vận hành THẬT: pipeline CI
QUYẾT ĐỊNH deploy được KHÔNG (cụm 1), deploy thành CÔNG ghi log CÓ
cấu trúc (cụm 3), MỘT loạt request sau deploy sinh RA tỷ lệ lỗi
(cụm 4), VÀ vòng quan sát ĐỌC tỷ lệ ĐÓ, tra runbook NẾU cần (cụm 6):

```typescript title=readonly
type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };
function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) return { qua: false, buocHong: buoc.ten };
  }
  return { qua: true, buocHong: null };
}
type MaNguon = { coLoiKieu: boolean; testQua: boolean };

type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; [truong: string]: unknown };
type Logger = { ghi: (level: LogLevel, msg: string, extra?: Record<string, unknown>) => void; layNhatKy: () => NhatKy[] };
function taoNhatKy(): Logger {
  const cacLog: NhatKy[] = [];
  return { ghi: (level, msg, extra = {}) => { cacLog.push({ level, msg, ...extra }); }, layNhatKy: () => cacLog };
}

function tinhTyLeLoi(soLoi: number, soTong: number): number {
  if (soTong === 0) return 0;
  return soLoi / soTong;
}

type LoaiCanhBao = "ty_le_loi_cao" | "dependency_hong" | "do_tre_cao";
function layHanhDong(loai: LoaiCanhBao): string {
  switch (loai) {
    case "ty_le_loi_cao": return "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra";
    case "dependency_hong": return "Kiem tra dependency, xem xet failover hoac restart";
    case "do_tre_cao": return "Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau";
    default: { const _kiemTraDayDu: never = loai; return _kiemTraDayDu; }
  }
}
function vongQuanSat(tyLeLoi: number, nguong: number): string {
  if (tyLeLoi > nguong) return layHanhDong("ty_le_loi_cao");
  return "Binh thuong, khong can hanh dong";
}

type KetQuaTrungTam = { daDeploy: boolean; buocHong: string | null; hanhDong: string };

function trungTamVanHanh(maNguon: MaNguon, logger: Logger, soLoi: number, soTong: number, nguong: number): KetQuaTrungTam {
  const ketQuaPipeline = chayPipelineCI(
    maNguon,
    { ten: "typecheck", kiemTra: (mn: MaNguon) => !mn.coLoiKieu },
    { ten: "test", kiemTra: (mn: MaNguon) => mn.testQua },
  );

  if (!ketQuaPipeline.qua) {
    logger.ghi("error", "Pipeline that bai, khong deploy", { buocHong: ketQuaPipeline.buocHong });
    return { daDeploy: false, buocHong: ketQuaPipeline.buocHong, hanhDong: "Khong deploy -- pipeline chua qua" };
  }

  logger.ghi("info", "Deploy thanh cong");
  const tyLeLoi = tinhTyLeLoi(soLoi, soTong);
  const hanhDong = vongQuanSat(tyLeLoi, nguong);
  return { daDeploy: true, buocHong: null, hanhDong };
}

const logger1 = taoNhatKy();
console.log(trungTamVanHanh({ coLoiKieu: false, testQua: true }, logger1, 2, 100, 0.1));
```

```text title=readonly
{"daDeploy":true,"buocHong":null,"hanhDong":"Binh thuong, khong can hanh dong"}
```

Pipeline QUA (`coLoiKieu: false`, `testQua: true`) → deploy thành
CÔNG, log GHI LẠI, tỷ lệ lỗi (`2/100 = 0.02`) DƯỚI ngưỡng (`0.1`) →
"bình thường". SÁU cụm, MỘT chuỗi DUY NHẤT.
::::

::::example{#pipeline-hong-chan-toan-bo}
Pipeline HỎNG **CHẶN HOÀN TOÀN** metrics VÀ alerting — KHÔNG request
NÀO được TÍNH, BẤT KỂ `soLoi`/`soTong` LÀ bao NHIÊU:

```typescript title=readonly
type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };
function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) return { qua: false, buocHong: buoc.ten };
  }
  return { qua: true, buocHong: null };
}
type MaNguon = { coLoiKieu: boolean; testQua: boolean };
type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; [truong: string]: unknown };
type Logger = { ghi: (level: LogLevel, msg: string, extra?: Record<string, unknown>) => void; layNhatKy: () => NhatKy[] };
function taoNhatKy(): Logger {
  const cacLog: NhatKy[] = [];
  return { ghi: (level, msg, extra = {}) => { cacLog.push({ level, msg, ...extra }); }, layNhatKy: () => cacLog };
}
function tinhTyLeLoi(soLoi: number, soTong: number): number { if (soTong === 0) return 0; return soLoi / soTong; }
type LoaiCanhBao = "ty_le_loi_cao" | "dependency_hong" | "do_tre_cao";
function layHanhDong(loai: LoaiCanhBao): string {
  switch (loai) {
    case "ty_le_loi_cao": return "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra";
    case "dependency_hong": return "Kiem tra dependency, xem xet failover hoac restart";
    case "do_tre_cao": return "Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau";
    default: { const _kiemTraDayDu: never = loai; return _kiemTraDayDu; }
  }
}
function vongQuanSat(tyLeLoi: number, nguong: number): string {
  if (tyLeLoi > nguong) return layHanhDong("ty_le_loi_cao");
  return "Binh thuong, khong can hanh dong";
}
type KetQuaTrungTam = { daDeploy: boolean; buocHong: string | null; hanhDong: string };
function trungTamVanHanh(maNguon: MaNguon, logger: Logger, soLoi: number, soTong: number, nguong: number): KetQuaTrungTam {
  const ketQuaPipeline = chayPipelineCI(
    maNguon,
    { ten: "typecheck", kiemTra: (mn: MaNguon) => !mn.coLoiKieu },
    { ten: "test", kiemTra: (mn: MaNguon) => mn.testQua },
  );
  if (!ketQuaPipeline.qua) {
    logger.ghi("error", "Pipeline that bai, khong deploy", { buocHong: ketQuaPipeline.buocHong });
    return { daDeploy: false, buocHong: ketQuaPipeline.buocHong, hanhDong: "Khong deploy -- pipeline chua qua" };
  }
  logger.ghi("info", "Deploy thanh cong");
  const tyLeLoi = tinhTyLeLoi(soLoi, soTong);
  const hanhDong = vongQuanSat(tyLeLoi, nguong);
  return { daDeploy: true, buocHong: null, hanhDong };
}

const logger2 = taoNhatKy();
console.log(trungTamVanHanh({ coLoiKieu: true, testQua: true }, logger2, 50, 100, 0.1));
console.log(logger2.layNhatKy().length);
```

```text title=readonly
{"daDeploy":false,"buocHong":"typecheck","hanhDong":"Khong deploy -- pipeline chua qua"}
1
```

`soLoi: 50, soTong: 100` (tỷ lệ lỗi `0.5` — RẤT cao, LẼ ra PHẢI báo
động MẠNH) — NHƯNG `coLoiKieu: true` khiến `typecheck` HỎNG NGAY,
`trungTamVanHanh` `return` SỚM TRƯỚC khi TỚI dòng `tinhTyLeLoi`.
`logger2.layNhatKy().length` LÀ `1` (CHỈ dòng lỗi pipeline) — KHÔNG
CÓ dòng "Deploy thanh cong" NÀO, VÌ deploy CHƯA HỀ xảy ra.
::::

::::predict{#doan-pipeline-hong-metrics-khong-bao-gio-chay commitOnce}
```typescript
// (dinh nghia trungTamVanHanh, taoNhatKy, ... nhu tren)
const logger3 = taoNhatKy();
const ketQua3 = trungTamVanHanh({ coLoiKieu: true, testQua: true }, logger3, 50, 100, 0.1);
console.log(logger3.layNhatKy().length);
```

`soLoi: 50, soTong: 100` — tỷ lệ lỗi `0.5`, VƯỢT XA ngưỡng `0.1`.
NHƯNG `coLoiKieu: true` khiến pipeline HỎNG. Dòng cuối in ra gì?

:::opt{correct}
`1`
:::

:::opt
`2` — vì tỷ lệ lỗi `0.5` QUÁ cao, hệ thống NÊN ghi THÊM MỘT dòng log
CẢNH BÁO riêng VỀ tỷ lệ lỗi (bên CẠNH dòng log pipeline hỏng), NÊN
tổng CỘNG phải CÓ hai dòng log
::why
Gần đúng ở việc bạn nhận RA `0.5` LÀ MỘT tỷ lệ lỗi ĐÁNG báo động —
MỘT quan sát ĐÚNG về mức ĐỘ nghiêm trọng CỦA con số ĐÓ.

Chỗ lệch: NHÌN LẠI `trungTamVanHanh` — dòng `logger.ghi("error", ...)`
Ở nhánh `if (!ketQuaPipeline.qua)` LÀ dòng log DUY NHẤT ĐƯỢC ghi
TRƯỚC KHI HÀM `return` (thoát HÀM NGAY). CÁC dòng TÍNH `tyLeLoi`
VÀ `vongQuanSat` nằm **SAU** điểm `return` NÀY trong LUỒNG thực thi
— KHI pipeline hỏng, chương TRÌNH KHÔNG BAO GIỜ chạy tới ĐÓ, NÊN
KHÔNG có dòng log THỨ HAI nào được ghi, BẤT KỂ `soLoi`/`soTong` LÀ
GÌ. `logger3.layNhatKy().length` LÀ `1`.
::
:::

:::opt
Máy báo lỗi biên dịch — `trungTamVanHanh` gọi `tinhTyLeLoi(soLoi,
soTong)` Ở CUỐI hàm, NHƯNG nhánh `if` PHÍA TRÊN ĐÃ `return` SỚM,
TypeScript CẤM MỘT hàm CÓ code "KHÔNG BAO GIỜ chạy tới" (unreachable
code) SAU MỘT nhánh return CÓ điều kiện
::why
Gần đúng ở việc bạn để ý nhánh `if` PHÍA TRÊN CÓ `return` — MỘT
quan sát ĐÚNG về CẤU TRÚC điều khiển.

Chỗ lệch: TypeScript (VÀ JavaScript) HOÀN TOÀN cho PHÉP code SAU
MỘT `return` **CÓ ĐIỀU KIỆN** (BÊN TRONG `if`) — code ĐÓ CHỈ
"unreachable" khi NẰM SAU MỘT `return` VÔ ĐIỀU KIỆN (LUÔN chạy). Ở
ĐÂY, `return` NẰM TRONG `if (!ketQuaPipeline.qua)` — KHI ĐIỀU kiện
LÀ `false` (pipeline QUA), luồng thực thi TIẾP TỤC bình THƯỜNG
xuống CÁC dòng SAU. Biên dịch SẠCH.
::
:::
::::

::::code{#viet_trung_tam_van_hanh}
Hoàn thiện `trungTamVanHanh` — ghép pipeline (bước kiểm TRA), tỷ lệ
lỗi, VÀ vòng quan sát.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };
function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) return { qua: false, buocHong: buoc.ten };
  }
  return { qua: true, buocHong: null };
}
type MaNguon = { coLoiKieu: boolean; testQua: boolean };

type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; [truong: string]: unknown };
type Logger = { ghi: (level: LogLevel, msg: string, extra?: Record<string, unknown>) => void; layNhatKy: () => NhatKy[] };
function taoNhatKy(): Logger {
  const cacLog: NhatKy[] = [];
  return { ghi: (level, msg, extra = {}) => { cacLog.push({ level, msg, ...extra }); }, layNhatKy: () => cacLog };
}

function tinhTyLeLoi(soLoi: number, soTong: number): number {
  if (soTong === 0) return 0;
  return soLoi / soTong;
}

type LoaiCanhBao = "ty_le_loi_cao" | "dependency_hong" | "do_tre_cao";
function layHanhDong(loai: LoaiCanhBao): string {
  switch (loai) {
    case "ty_le_loi_cao": return "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra";
    case "dependency_hong": return "Kiem tra dependency, xem xet failover hoac restart";
    case "do_tre_cao": return "Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau";
    default: { const _kiemTraDayDu: never = loai; return _kiemTraDayDu; }
  }
}
function vongQuanSat(tyLeLoi: number, nguong: number): string {
  if (tyLeLoi > nguong) return layHanhDong("ty_le_loi_cao");
  return "Binh thuong, khong can hanh dong";
}

type KetQuaTrungTam = { daDeploy: boolean; buocHong: string | null; hanhDong: string };

function trungTamVanHanh(maNguon: MaNguon, logger: Logger, soLoi: number, soTong: number, nguong: number): KetQuaTrungTam {
  const ketQuaPipeline = chayPipelineCI(
    maNguon,
    { ten: "typecheck", kiemTra: (mn: MaNguon) => ___ },
    { ten: "test", kiemTra: (mn: MaNguon) => ___ },
  );

  if (!ketQuaPipeline.qua) {
    logger.ghi("error", "Pipeline that bai, khong deploy", { buocHong: ketQuaPipeline.buocHong });
    return { daDeploy: false, buocHong: ketQuaPipeline.buocHong, hanhDong: "Khong deploy -- pipeline chua qua" };
  }

  logger.ghi("info", "Deploy thanh cong");
  const tyLeLoi = ___;
  const hanhDong = ___;
  return { daDeploy: true, buocHong: null, hanhDong };
}

const l1 = taoNhatKy();
const r1 = trungTamVanHanh({ coLoiKieu: true, testQua: true }, l1, 0, 0, 0.1);
assertEqual(r1.daDeploy, false, "pipeline fail typecheck -- khong deploy");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };
function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) return { qua: false, buocHong: buoc.ten };
  }
  return { qua: true, buocHong: null };
}
type MaNguon = { coLoiKieu: boolean; testQua: boolean };

type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; [truong: string]: unknown };
type Logger = { ghi: (level: LogLevel, msg: string, extra?: Record<string, unknown>) => void; layNhatKy: () => NhatKy[] };
function taoNhatKy(): Logger {
  const cacLog: NhatKy[] = [];
  return { ghi: (level, msg, extra = {}) => { cacLog.push({ level, msg, ...extra }); }, layNhatKy: () => cacLog };
}

function tinhTyLeLoi(soLoi: number, soTong: number): number {
  if (soTong === 0) return 0;
  return soLoi / soTong;
}

type LoaiCanhBao = "ty_le_loi_cao" | "dependency_hong" | "do_tre_cao";
function layHanhDong(loai: LoaiCanhBao): string {
  switch (loai) {
    case "ty_le_loi_cao": return "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra";
    case "dependency_hong": return "Kiem tra dependency, xem xet failover hoac restart";
    case "do_tre_cao": return "Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau";
    default: { const _kiemTraDayDu: never = loai; return _kiemTraDayDu; }
  }
}
function vongQuanSat(tyLeLoi: number, nguong: number): string {
  if (tyLeLoi > nguong) return layHanhDong("ty_le_loi_cao");
  return "Binh thuong, khong can hanh dong";
}

type KetQuaTrungTam = { daDeploy: boolean; buocHong: string | null; hanhDong: string };

function trungTamVanHanh(maNguon: MaNguon, logger: Logger, soLoi: number, soTong: number, nguong: number): KetQuaTrungTam {
  const ketQuaPipeline = chayPipelineCI(
    maNguon,
    { ten: "typecheck", kiemTra: (mn: MaNguon) => !mn.coLoiKieu },
    { ten: "test", kiemTra: (mn: MaNguon) => mn.testQua },
  );

  if (!ketQuaPipeline.qua) {
    logger.ghi("error", "Pipeline that bai, khong deploy", { buocHong: ketQuaPipeline.buocHong });
    return { daDeploy: false, buocHong: ketQuaPipeline.buocHong, hanhDong: "Khong deploy -- pipeline chua qua" };
  }

  logger.ghi("info", "Deploy thanh cong");
  const tyLeLoi = tinhTyLeLoi(soLoi, soTong);
  const hanhDong = vongQuanSat(tyLeLoi, nguong);
  return { daDeploy: true, buocHong: null, hanhDong };
}

const l1 = taoNhatKy();
const r1 = trungTamVanHanh({ coLoiKieu: true, testQua: true }, l1, 0, 0, 0.1);
assertEqual(r1.daDeploy, false, "pipeline fail typecheck -- khong deploy");
```

```typescript title=test
assertEqual(r1.buocHong, "typecheck", "buoc hong dung la typecheck");
assertEqual(r1.hanhDong, "Khong deploy -- pipeline chua qua", "hanh dong khi khong deploy");
assertEqual(l1.layNhatKy().length, 1, "chi mot dong log khi pipeline fail");
assertEqual(l1.layNhatKy()[0]!.level, "error", "log level error khi pipeline fail");

const l2 = taoNhatKy();
const r2 = trungTamVanHanh({ coLoiKieu: false, testQua: false }, l2, 0, 0, 0.1);
assertEqual(r2.buocHong, "test", "buoc hong dung la test");

const l3 = taoNhatKy();
const r3 = trungTamVanHanh({ coLoiKieu: false, testQua: true }, l3, 20, 100, 0.1);
assertEqual(r3.daDeploy, true, "pipeline qua -- deploy thanh cong");
assertEqual(r3.hanhDong, "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra", "ty le loi cao -- co hanh dong");
assertEqual(l3.layNhatKy().length, 1, "chi mot dong log deploy thanh cong");
assertEqual(l3.layNhatKy()[0]!.level, "info", "log level info khi deploy thanh cong");

const l4 = taoNhatKy();
const r4 = trungTamVanHanh({ coLoiKieu: false, testQua: true }, l4, 2, 100, 0.1);
assertEqual(r4.hanhDong, "Binh thuong, khong can hanh dong", "ty le loi thap -- binh thuong");
```

:::hints
- kind: attention
  body: "Buoc typecheck: !mn.coLoiKieu. Buoc test: mn.testQua. tyLeLoi goi tinhTyLeLoi(soLoi, soTong). hanhDong goi vongQuanSat(tyLeLoi, nguong)."
- kind: strategy
  body: "!mn.coLoiKieu : mn.testQua : tinhTyLeLoi(soLoi, soTong) : vongQuanSat(tyLeLoi, nguong)"
- kind: one-line
  body: '___ (typecheck) = !mn.coLoiKieu\n___ (test) = mn.testQua\n___ (tyLeLoi) = tinhTyLeLoi(soLoi, soTong)\n___ (hanhDong) = vongQuanSat(tyLeLoi, nguong)'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "PASS"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Trung tâm vận hành hoàn chỉnh — pipeline, deploy, logging, metrics,
alerting, tất cả nối liền thành một chuỗi. Track "Vận hành" khép lại
ở đây.
::::

::::reflect{#tong-ket-track}
Track "Vận hành" (CI/CD & Observability) khép lại Ở đây. Nhìn LẠI
sáu cụm: `chayPipelineCI` fail-fast (bước RẺ trước, ĐẮT sau) —
`xetDuyetDeploy`/`xacDinhMoiTruong`/rollback (deploy CÓ điều kiện,
quay VỀ phiên bản trước KHI cần) — `taoNhatKy`/`locTheoLevel`/
`locTheoRequestId` (log CÓ cấu trúc, lọc ĐƯỢC, theo dõi ĐƯỢC MỘT
request) — `taoCounter`/`tinhTyLeLoi`/`tinhPercentile` (đếm, tỷ lệ,
phân vị — TỰ TAY, KHÔNG thư viện) — `Span`/`timBuocChamNhat`/
`kiemTraSucKhoe` (bước NÀO chậm, dependency NÀO hỏng) —
`tinhErrorBudget`/`layHanhDong`/`vongQuanSat` (cam kết, hành động,
vòng phản HỒI). HAI mươi BỐN bài, MỘT bức TRANH: một hệ THỐNG
KHÔNG chỉ CHẠY được, mà còn CHO BIẾT nó ĐANG chạy NHƯ thế nào, VÀ
khi SAI thì PHẢI làm GÌ. Realm 5 (Kỹ nghệ phần mềm) gần NHƯ trọn
vẹn — chỉ CÒN Git (T5.1), hoãn VÌ cần hạ tầng mô phỏng RIÊNG.
::::

::::checkpoint{mastery=0.85}
::::
