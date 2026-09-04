---
id: thiet-ke-he-thong.quan-sat-va-canh-bao.boss-quan-sat-va-canh-bao
title: "BOSS — ráp toàn bộ observability stack cho một sự cố mô phỏng"
summary: "xuLySuCoDauCuoi rap DUNG sau khai niem theo THU TU: tinhPercentile (bai 3) tinh p99 tu histogram; p99 <= nguongSLOMs (bai 5) thi DUNG NGAY (khong co su co); vuot SLO thi kiem tra canhBaoTheoTrieuChung (bai 6) voi nguongDoTreCanhBaoMs RIENG (co the cao hon SLO -- vuot SLO nhung CHUA du de canh bao van DUNG som); alert kich hoat moi goi aiDangTruc (bai 7) tim nguoi truc, timDichVuChiemNhieuNhat (bai 4) tim dich vu cham tu trace, locLogTheoMucDoVaDichVu (bai 2) loc log loi CUA DUNG dich vu do (dong -- dung ten dich vu MOI tim duoc o buoc truoc), roi dung kiemTraDayDu (bai 8) kiem tra bien ban su co. Vi du day du: p99=800 vuot ca hai nguong (300 va 400) -> Binh dang truc, dich-vu-thanh-toan cham nhat (440ms), 1 log loi tim duoc, postmortem DAY DU."
locale: vi
track: thiet-ke-he-thong
module: quan-sat-va-canh-bao
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [sd.boss-quan-sat-va-canh-bao]
requires: [sd.blameless-postmortem]
concepts: [sd.boss-quan-sat-va-canh-bao]
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
Tám bài — ba trụ cột quan sát, log có cấu trúc, đúng loại metric, trace
xuyên dịch vụ, SLI/SLO/error budget, alert theo triệu chứng, on-call VÀ
runbook, blameless postmortem. Giờ LÀ lúc ráp TẤT cả thành một luồng xử lý
sự cố đầu-cuối: từ một con số metric bất thường, cho tới một biên bản sự
cố hoàn chỉnh.
::::

::::explain{#rap-pipeline-quan-sat}
`xuLySuCoDauCuoi` đi qua ĐÚNG bốn trạm, DỪNG NGAY tại trạm nào chưa đủ căn
cứ đi tiếp: tính `p99` (bài 3) — dưới ngưỡng SLO (bài 5) thì DỪNG, chưa có
gì đáng lo; vượt SLO thì kiểm tra `canhBaoTheoTrieuChung` (bài 6) VỚI một
ngưỡng cảnh báo RIÊNG — nếu chưa đủ để cảnh báo thì CŨNG dừng; chỉ khi alert
THẬT sự kích hoạt, pipeline mới tìm người trực (bài 7), tìm dịch vụ chậm
qua trace (bài 4), lọc log lỗi CỦA đúng dịch vụ đó (bài 2), VÀ dựng biên
bản sự cố để kiểm tra tính đầy đủ (bài 8):

```typescript title=readonly
interface Histogram { ten: string; cacGiaTri: number[]; }
function taoHistogram(ten: string): Histogram { return { ten, cacGiaTri: [] }; }
function ghiHistogram(h: Histogram, giaTri: number): void { h.cacGiaTri.push(giaTri); }
function tinhPercentile(h: Histogram, phanTram: number): number | undefined {
  if (h.cacGiaTri.length === 0) return undefined;
  const sapXep = [...h.cacGiaTri].sort((a, b) => a - b);
  const idx = Math.ceil((phanTram / 100) * sapXep.length) - 1;
  const idxAnToan = Math.max(0, Math.min(sapXep.length - 1, idx));
  return sapXep[idxAnToan];
}

interface TrangThaiHeThong { cpuPhanTram: number; tyLeLoiNguoiDungThay: number; doTreP99Ms: number; }
function canhBaoTheoTrieuChung(ts: TrangThaiHeThong, nguongTyLeLoi: number, nguongDoTreMs: number): boolean {
  return ts.tyLeLoiNguoiDungThay > nguongTyLeLoi || ts.doTreP99Ms > nguongDoTreMs;
}

interface DongHoMoPhong { thoiGianHienTai: number; }
interface CaTruc { tenNguoiTruc: string; gioBatDau: number; gioKetThuc: number; }
function aiDangTruc(cacCaTruc: CaTruc[], dh: DongHoMoPhong): string | undefined {
  const ca = cacCaTruc.find((c) => dh.thoiGianHienTai >= c.gioBatDau && dh.thoiGianHienTai < c.gioKetThuc);
  return ca?.tenNguoiTruc;
}

interface Span {
  traceId: string; spanId: string; parentSpanId: string | undefined;
  tenDichVu: string; thoiGianBatDauMs: number; thoiGianKetThucMs: number;
}
function timDichVuChiemNhieuNhat(cacSpan: Span[]): { tenDichVu: string; thoiGianMs: number } | undefined {
  const conSpan = cacSpan.filter((s) => s.parentSpanId !== undefined);
  if (conSpan.length === 0) return undefined;
  let max = conSpan[0]!;
  for (const s of conSpan) {
    if (s.thoiGianKetThucMs - s.thoiGianBatDauMs > max.thoiGianKetThucMs - max.thoiGianBatDauMs) max = s;
  }
  return { tenDichVu: max.tenDichVu, thoiGianMs: max.thoiGianKetThucMs - max.thoiGianBatDauMs };
}

interface BanGhiLogCoCauTruc { thoiDiem: number; mucDo: string; dichVu: string; thongDiep: string; metadata: Record<string, string>; }
function locLogTheoMucDoVaDichVu(cacLog: BanGhiLogCoCauTruc[], mucDo: string, dichVu: string): BanGhiLogCoCauTruc[] {
  return cacLog.filter((l) => l.mucDo === mucDo && l.dichVu === dichVu);
}

interface MocThoiGian { thoiDiem: number; moTa: string; }
interface HanhDongTiepTheo { moTa: string; nguoiPhuTrach: string; trangThai: string; }
interface BienBanSuCo {
  tenSuCo: string; timeline: MocThoiGian[];
  nguyenNhanHeThong: string | undefined; hanhDongTiepTheo: HanhDongTiepTheo[];
}
interface KetQuaKiemTra { dayDu: boolean; thieu: string[]; }
function kiemTraDayDu(bienBan: BienBanSuCo): KetQuaKiemTra {
  const thieu: string[] = [];
  if (bienBan.timeline.length === 0) thieu.push("timeline");
  if (bienBan.nguyenNhanHeThong === undefined || bienBan.nguyenNhanHeThong.trim() === "") thieu.push("nguyenNhanHeThong");
  if (bienBan.hanhDongTiepTheo.length === 0) thieu.push("hanhDongTiepTheo");
  return { dayDu: thieu.length === 0, thieu };
}

interface KetQuaXuLySuCo {
  p99: number | undefined; vuotSLO: boolean; alertKichHoat: boolean;
  nguoiTruc: string | undefined; dichVuCham: { tenDichVu: string; thoiGianMs: number } | undefined;
  soLogLoiTimDuoc: number; postmortemDayDu: boolean; postmortemThieu: string[];
}
function xuLySuCoDauCuoi(
  histDoTre: Histogram, nguongSLOMs: number, nguongDoTreCanhBaoMs: number, tyLeLoiNguoiDungThay: number,
  cacCaTruc: CaTruc[], dh: DongHoMoPhong, cacSpan: Span[], cacLog: BanGhiLogCoCauTruc[], hanhDongTiepTheo: HanhDongTiepTheo[]
): KetQuaXuLySuCo {
  const ketQuaRong: KetQuaXuLySuCo = {
    p99: undefined, vuotSLO: false, alertKichHoat: false, nguoiTruc: undefined,
    dichVuCham: undefined, soLogLoiTimDuoc: 0, postmortemDayDu: false, postmortemThieu: [],
  };
  const p99 = tinhPercentile(histDoTre, 99);
  if (p99 === undefined || p99 <= nguongSLOMs) return { ...ketQuaRong, p99 };

  const ts: TrangThaiHeThong = { cpuPhanTram: 0, tyLeLoiNguoiDungThay, doTreP99Ms: p99 };
  const alertKichHoat = canhBaoTheoTrieuChung(ts, 1, nguongDoTreCanhBaoMs);
  if (!alertKichHoat) return { ...ketQuaRong, p99, vuotSLO: true };

  const nguoiTruc = aiDangTruc(cacCaTruc, dh);
  const dichVuCham = timDichVuChiemNhieuNhat(cacSpan);
  const logLoi = dichVuCham === undefined ? [] : locLogTheoMucDoVaDichVu(cacLog, "loi", dichVuCham.tenDichVu);
  const bienBan: BienBanSuCo = {
    tenSuCo: "do-tre-cao",
    timeline: [
      { thoiDiem: dh.thoiGianHienTai, moTa: "Alert theo trieu chung kich hoat, p99 vuot SLO" },
      { thoiDiem: dh.thoiGianHienTai, moTa: `Nguoi truc (${nguoiTruc ?? "chua xac dinh"}) dung trace tim ra dich vu cham` },
    ],
    nguyenNhanHeThong:
      dichVuCham === undefined ? undefined
        : `He thong khong canh bao rieng cho ${dichVuCham.tenDichVu} khi thoi gian xu ly vuot ${dichVuCham.thoiGianMs}ms`,
    hanhDongTiepTheo,
  };
  const ketQuaKiemTra = kiemTraDayDu(bienBan);
  return {
    p99, vuotSLO: true, alertKichHoat: true, nguoiTruc, dichVuCham,
    soLogLoiTimDuoc: logLoi.length, postmortemDayDu: ketQuaKiemTra.dayDu, postmortemThieu: ketQuaKiemTra.thieu,
  };
}

const histDoTre: Histogram = taoHistogram("do-tre-request");
for (const v of [120, 130, 140, 150, 800]) ghiHistogram(histDoTre, v);
const cacCaTruc: CaTruc[] = [
  { tenNguoiTruc: "An", gioBatDau: 0, gioKetThuc: 24 },
  { tenNguoiTruc: "Binh", gioBatDau: 24, gioKetThuc: 48 },
  { tenNguoiTruc: "Chi", gioBatDau: 48, gioKetThuc: 72 },
];
const dh: DongHoMoPhong = { thoiGianHienTai: 30 };
const cacSpan: Span[] = [
  { traceId: "trace-77", spanId: "s1", parentSpanId: undefined, tenDichVu: "api-gateway", thoiGianBatDauMs: 0, thoiGianKetThucMs: 500 },
  { traceId: "trace-77", spanId: "s2", parentSpanId: "s1", tenDichVu: "dich-vu-thanh-toan", thoiGianBatDauMs: 20, thoiGianKetThucMs: 460 },
  { traceId: "trace-77", spanId: "s3", parentSpanId: "s2", tenDichVu: "dich-vu-ngan-hang", thoiGianBatDauMs: 60, thoiGianKetThucMs: 420 },
];
const cacLog: BanGhiLogCoCauTruc[] = [
  { thoiDiem: 19, mucDo: "loi", dichVu: "dich-vu-thanh-toan", thongDiep: "Ket noi bi loi", metadata: { maLoi: "E502" } },
  { thoiDiem: 20, mucDo: "thong-tin", dichVu: "dich-vu-thanh-toan", thongDiep: "Da xu ly xong", metadata: {} },
  { thoiDiem: 21, mucDo: "loi", dichVu: "dich-vu-ngan-hang", thongDiep: "timeout", metadata: {} },
];
const hanhDongTiepTheo: HanhDongTiepTheo[] = [
  { moTa: "Them alert rieng cho dich-vu-thanh-toan khi vuot 300ms", nguoiPhuTrach: "An", trangThai: "dang-lam" },
];

const ketQua = xuLySuCoDauCuoi(histDoTre, 300, 400, 0.5, cacCaTruc, dh, cacSpan, cacLog, hanhDongTiepTheo);
console.log("ket qua xu ly su co (day du chuoi):", JSON.stringify(ketQua));
```

```text title=readonly
ket qua xu ly su co (day du chuoi): {"p99":800,"vuotSLO":true,"alertKichHoat":true,"nguoiTruc":"Binh","dichVuCham":{"tenDichVu":"dich-vu-thanh-toan","thoiGianMs":440},"soLogLoiTimDuoc":1,"postmortemDayDu":true,"postmortemThieu":[]}
```

`p99=800` vượt CẢ ngưỡng SLO (`300`) LẪN ngưỡng cảnh báo (`400`) — pipeline
đi HẾT bốn trạm: `"Binh"` LÀ người trực Ở `gio=30`, `"dich-vu-thanh-toan"`
LÀ dịch vụ chiếm nhiều thời gian nhất (`440ms`, tìm được từ trace), ĐÚNG
`1` log lỗi khớp dịch vụ ĐÓ (tham số `dichVu` truyền vào `locLogTheoMucDoVaDichVu`
LÀ `dichVuCham.tenDichVu` — kết quả của TRẠM trước, không phải một chuỗi
gõ tay), VÀ biên bản sự cố ĐẦY đủ cả ba phần bắt buộc.
::::

::::example{#dung-som-o-tung-tram}
Không phải mọi lần gọi đều đi hết bốn trạm — `xuLySuCoDauCuoi` DỪNG NGAY
tại trạm ĐẦU nếu `p99` chưa vượt SLO, VÀ dừng Ở trạm THỨ hai nếu đã vượt SLO
nhưng CHƯA đủ để kích hoạt cảnh báo (một VÙNG ĐỆM có chủ đích giữa hai
ngưỡng):

```typescript title=readonly
interface Histogram { ten: string; cacGiaTri: number[]; }
function taoHistogram(ten: string): Histogram { return { ten, cacGiaTri: [] }; }
function ghiHistogram(h: Histogram, giaTri: number): void { h.cacGiaTri.push(giaTri); }
function tinhPercentile(h: Histogram, phanTram: number): number | undefined {
  if (h.cacGiaTri.length === 0) return undefined;
  const sapXep = [...h.cacGiaTri].sort((a, b) => a - b);
  const idx = Math.ceil((phanTram / 100) * sapXep.length) - 1;
  const idxAnToan = Math.max(0, Math.min(sapXep.length - 1, idx));
  return sapXep[idxAnToan];
}

interface TrangThaiHeThong { cpuPhanTram: number; tyLeLoiNguoiDungThay: number; doTreP99Ms: number; }
function canhBaoTheoTrieuChung(ts: TrangThaiHeThong, nguongTyLeLoi: number, nguongDoTreMs: number): boolean {
  return ts.tyLeLoiNguoiDungThay > nguongTyLeLoi || ts.doTreP99Ms > nguongDoTreMs;
}

interface DongHoMoPhong { thoiGianHienTai: number; }
interface CaTruc { tenNguoiTruc: string; gioBatDau: number; gioKetThuc: number; }
function aiDangTruc(cacCaTruc: CaTruc[], dh: DongHoMoPhong): string | undefined {
  const ca = cacCaTruc.find((c) => dh.thoiGianHienTai >= c.gioBatDau && dh.thoiGianHienTai < c.gioKetThuc);
  return ca?.tenNguoiTruc;
}

interface Span {
  traceId: string; spanId: string; parentSpanId: string | undefined;
  tenDichVu: string; thoiGianBatDauMs: number; thoiGianKetThucMs: number;
}
function timDichVuChiemNhieuNhat(cacSpan: Span[]): { tenDichVu: string; thoiGianMs: number } | undefined {
  const conSpan = cacSpan.filter((s) => s.parentSpanId !== undefined);
  if (conSpan.length === 0) return undefined;
  let max = conSpan[0]!;
  for (const s of conSpan) {
    if (s.thoiGianKetThucMs - s.thoiGianBatDauMs > max.thoiGianKetThucMs - max.thoiGianBatDauMs) max = s;
  }
  return { tenDichVu: max.tenDichVu, thoiGianMs: max.thoiGianKetThucMs - max.thoiGianBatDauMs };
}

interface BanGhiLogCoCauTruc { thoiDiem: number; mucDo: string; dichVu: string; thongDiep: string; metadata: Record<string, string>; }
function locLogTheoMucDoVaDichVu(cacLog: BanGhiLogCoCauTruc[], mucDo: string, dichVu: string): BanGhiLogCoCauTruc[] {
  return cacLog.filter((l) => l.mucDo === mucDo && l.dichVu === dichVu);
}

interface MocThoiGian { thoiDiem: number; moTa: string; }
interface HanhDongTiepTheo { moTa: string; nguoiPhuTrach: string; trangThai: string; }
interface BienBanSuCo {
  tenSuCo: string; timeline: MocThoiGian[];
  nguyenNhanHeThong: string | undefined; hanhDongTiepTheo: HanhDongTiepTheo[];
}
interface KetQuaKiemTra { dayDu: boolean; thieu: string[]; }
function kiemTraDayDu(bienBan: BienBanSuCo): KetQuaKiemTra {
  const thieu: string[] = [];
  if (bienBan.timeline.length === 0) thieu.push("timeline");
  if (bienBan.nguyenNhanHeThong === undefined || bienBan.nguyenNhanHeThong.trim() === "") thieu.push("nguyenNhanHeThong");
  if (bienBan.hanhDongTiepTheo.length === 0) thieu.push("hanhDongTiepTheo");
  return { dayDu: thieu.length === 0, thieu };
}

interface KetQuaXuLySuCo {
  p99: number | undefined; vuotSLO: boolean; alertKichHoat: boolean;
  nguoiTruc: string | undefined; dichVuCham: { tenDichVu: string; thoiGianMs: number } | undefined;
  soLogLoiTimDuoc: number; postmortemDayDu: boolean; postmortemThieu: string[];
}
function xuLySuCoDauCuoi(
  histDoTre: Histogram, nguongSLOMs: number, nguongDoTreCanhBaoMs: number, tyLeLoiNguoiDungThay: number,
  cacCaTruc: CaTruc[], dh: DongHoMoPhong, cacSpan: Span[], cacLog: BanGhiLogCoCauTruc[], hanhDongTiepTheo: HanhDongTiepTheo[]
): KetQuaXuLySuCo {
  const ketQuaRong: KetQuaXuLySuCo = {
    p99: undefined, vuotSLO: false, alertKichHoat: false, nguoiTruc: undefined,
    dichVuCham: undefined, soLogLoiTimDuoc: 0, postmortemDayDu: false, postmortemThieu: [],
  };
  const p99 = tinhPercentile(histDoTre, 99);
  if (p99 === undefined || p99 <= nguongSLOMs) return { ...ketQuaRong, p99 };

  const ts: TrangThaiHeThong = { cpuPhanTram: 0, tyLeLoiNguoiDungThay, doTreP99Ms: p99 };
  const alertKichHoat = canhBaoTheoTrieuChung(ts, 1, nguongDoTreCanhBaoMs);
  if (!alertKichHoat) return { ...ketQuaRong, p99, vuotSLO: true };

  const nguoiTruc = aiDangTruc(cacCaTruc, dh);
  const dichVuCham = timDichVuChiemNhieuNhat(cacSpan);
  const logLoi = dichVuCham === undefined ? [] : locLogTheoMucDoVaDichVu(cacLog, "loi", dichVuCham.tenDichVu);
  const bienBan: BienBanSuCo = {
    tenSuCo: "do-tre-cao",
    timeline: [{ thoiDiem: dh.thoiGianHienTai, moTa: "canh bao kich hoat" }],
    nguyenNhanHeThong: dichVuCham === undefined ? undefined : `${dichVuCham.tenDichVu} cham ${dichVuCham.thoiGianMs}ms`,
    hanhDongTiepTheo,
  };
  const ketQuaKiemTra = kiemTraDayDu(bienBan);
  return {
    p99, vuotSLO: true, alertKichHoat: true, nguoiTruc, dichVuCham,
    soLogLoiTimDuoc: logLoi.length, postmortemDayDu: ketQuaKiemTra.dayDu, postmortemThieu: ketQuaKiemTra.thieu,
  };
}

const cacCaTruc: CaTruc[] = [{ tenNguoiTruc: "An", gioBatDau: 0, gioKetThuc: 24 }];
const dh: DongHoMoPhong = { thoiGianHienTai: 10 };
const cacSpan: Span[] = [];
const cacLog: BanGhiLogCoCauTruc[] = [];
const hanhDongTiepTheo: HanhDongTiepTheo[] = [];

// truong hop 1: p99 duoi SLO -- KHONG co su co, dung o TRAM DAU
const histBinhThuong: Histogram = taoHistogram("do-tre-binh-thuong");
for (const v of [100, 110, 120, 130, 140]) ghiHistogram(histBinhThuong, v);
const ketQuaBinhThuong = xuLySuCoDauCuoi(histBinhThuong, 300, 400, 0.1, cacCaTruc, dh, cacSpan, cacLog, hanhDongTiepTheo);
console.log("p99 duoi SLO, dung o TRAM DAU:", JSON.stringify(ketQuaBinhThuong));

// truong hop 2: VUOT SLO nhung CHUA du de canh bao -- dung o TRAM HAI
const histVungDem: Histogram = taoHistogram("do-tre-vung-dem");
ghiHistogram(histVungDem, 350);
const ketQuaVungDem = xuLySuCoDauCuoi(histVungDem, 300, 400, 0.1, cacCaTruc, dh, cacSpan, cacLog, hanhDongTiepTheo);
console.log("vuot SLO (350>300) nhung chua toi nguong canh bao (350<=400), dung o TRAM HAI:", JSON.stringify(ketQuaVungDem));
```

```text title=readonly
p99 duoi SLO, dung o TRAM DAU: {"p99":140,"vuotSLO":false,"alertKichHoat":false,"soLogLoiTimDuoc":0,"postmortemDayDu":false,"postmortemThieu":[]}
vuot SLO (350>300) nhung chua toi nguong canh bao (350<=400), dung o TRAM HAI: {"p99":350,"vuotSLO":true,"alertKichHoat":false,"soLogLoiTimDuoc":0,"postmortemDayDu":false,"postmortemThieu":[]}
```

Cả hai trường hợp đều KHÔNG hề gọi tới `aiDangTruc`, `timDichVuChiemNhieuNhat`,
hay `kiemTraDayDu` — `nguoiTruc` VÀ `dichVuCham` vắng MẶT hoàn toàn trong
JSON (giá trị `undefined` bị `JSON.stringify` bỏ QUA), giống hệt cách bài 4
đã cho thấy. Một VÙNG ĐỆM CÓ CHỦ ĐÍCH giữa ngưỡng SLO (`300`) VÀ ngưỡng cảnh
báo (`400`) nghĩa LÀ: đã VI PHẠM mục tiêu "đủ tốt", nhưng CHƯA đủ nghiêm
trọng để đánh thức người trực — một quyết định vận hành THẬT sự, không
phải LÀ một lỗ hổng logic.
::::

::::predict{#doan-p99-dung-bang-nguong-canh-bao commitOnce}
`p99` tính RA đúng bằng `400`, VÀ `nguongSLOMs=300` (nên `vuotSLO=true`),
`nguongDoTreCanhBaoMs=400` (CHẠM đúng, không vượt), tỷ lệ lỗi người dùng
thấp. `alertKichHoat` LÀ gì, VÀ pipeline có đi tới trạm tìm người trực
không?

:::opt{correct}
`alertKichHoat=false`, KHÔNG đi tới trạm tìm người trực — `canhBaoTheoTrieuChung`
dùng `>` (nghiêm ngặt) cho vế độ trễ; `400 > 400` LÀ `false`, VÀ tỷ lệ lỗi
cũng dưới ngưỡng, nên `alertKichHoat` LÀ `false`, pipeline DỪNG Ở trạm hai
:::
:::opt
`alertKichHoat=true`, ĐI tới trạm tìm người trực — vì `p99` ĐÃ vượt SLO
(`400 > 300`) từ trước, nên chắc chắn cũng đủ nghiêm trọng để kích hoạt
cảnh báo, không cần kiểm tra riêng vế độ trễ trong `canhBaoTheoTrieuChung`
::why
Nhầm "đã vượt SLO" VỚI "tự động đủ để kích hoạt cảnh báo" — nhưng
`canhBaoTheoTrieuChung` dùng một ngưỡng RIÊNG (`nguongDoTreCanhBaoMs`),
KHÁC với ngưỡng SLO, VÀ hai điều kiện được kiểm tra hoàn toàn độc lập.

Chỗ lệch: `ts.doTreP99Ms > nguongDoTreMs` với `doTreP99Ms=400` VÀ
`nguongDoTreMs=400` (tham số `nguongDoTreCanhBaoMs` truyền VÀO) cho
`400 > 400`, LÀ `false`. Vế tỷ lệ lỗi cũng `false`. `alertKichHoat` LÀ
`false`, VÀ nhánh `if (!alertKichHoat)` trả về SỚM — pipeline không bao
giờ chạm tới `aiDangTruc`.
::
:::
::::

::::code{#viet_xu_ly_su_co_dau_cuoi}
Hoàn thiện phần CÒN lại của `xuLySuCoDauCuoi` — SAU khi đã xác nhận alert
kích hoạt: tìm người trực (`aiDangTruc`), tìm dịch vụ chậm nhất từ trace
(`timDichVuChiemNhieuNhat`), lọc log lỗi CỦA đúng dịch vụ đó (dùng
`dichVuCham.tenDichVu` LÀM tham số, không phải một chuỗi cố định), dựng
`BienBanSuCo`, kiểm tra tính đầy đủ (`kiemTraDayDu`), RỒI trả về kết quả
tổng hợp.

```typescript title=starter
interface Histogram { ten: string; cacGiaTri: number[]; }
function taoHistogram(ten: string): Histogram { return { ten, cacGiaTri: [] }; }
function ghiHistogram(h: Histogram, giaTri: number): void { h.cacGiaTri.push(giaTri); }
function tinhPercentile(h: Histogram, phanTram: number): number | undefined {
  if (h.cacGiaTri.length === 0) return undefined;
  const sapXep = [...h.cacGiaTri].sort((a, b) => a - b);
  const idx = Math.ceil((phanTram / 100) * sapXep.length) - 1;
  const idxAnToan = Math.max(0, Math.min(sapXep.length - 1, idx));
  return sapXep[idxAnToan];
}
interface TrangThaiHeThong { cpuPhanTram: number; tyLeLoiNguoiDungThay: number; doTreP99Ms: number; }
function canhBaoTheoTrieuChung(ts: TrangThaiHeThong, nguongTyLeLoi: number, nguongDoTreMs: number): boolean {
  return ts.tyLeLoiNguoiDungThay > nguongTyLeLoi || ts.doTreP99Ms > nguongDoTreMs;
}
interface DongHoMoPhong { thoiGianHienTai: number; }
interface CaTruc { tenNguoiTruc: string; gioBatDau: number; gioKetThuc: number; }
function aiDangTruc(cacCaTruc: CaTruc[], dh: DongHoMoPhong): string | undefined {
  const ca = cacCaTruc.find((c) => dh.thoiGianHienTai >= c.gioBatDau && dh.thoiGianHienTai < c.gioKetThuc);
  return ca?.tenNguoiTruc;
}
interface Span {
  traceId: string; spanId: string; parentSpanId: string | undefined;
  tenDichVu: string; thoiGianBatDauMs: number; thoiGianKetThucMs: number;
}
function timDichVuChiemNhieuNhat(cacSpan: Span[]): { tenDichVu: string; thoiGianMs: number } | undefined {
  const conSpan = cacSpan.filter((s) => s.parentSpanId !== undefined);
  if (conSpan.length === 0) return undefined;
  let max = conSpan[0]!;
  for (const s of conSpan) {
    if (s.thoiGianKetThucMs - s.thoiGianBatDauMs > max.thoiGianKetThucMs - max.thoiGianBatDauMs) max = s;
  }
  return { tenDichVu: max.tenDichVu, thoiGianMs: max.thoiGianKetThucMs - max.thoiGianBatDauMs };
}
interface BanGhiLogCoCauTruc { thoiDiem: number; mucDo: string; dichVu: string; thongDiep: string; metadata: Record<string, string>; }
function locLogTheoMucDoVaDichVu(cacLog: BanGhiLogCoCauTruc[], mucDo: string, dichVu: string): BanGhiLogCoCauTruc[] {
  return cacLog.filter((l) => l.mucDo === mucDo && l.dichVu === dichVu);
}
interface MocThoiGian { thoiDiem: number; moTa: string; }
interface HanhDongTiepTheo { moTa: string; nguoiPhuTrach: string; trangThai: string; }
interface BienBanSuCo {
  tenSuCo: string; timeline: MocThoiGian[];
  nguyenNhanHeThong: string | undefined; hanhDongTiepTheo: HanhDongTiepTheo[];
}
interface KetQuaKiemTra { dayDu: boolean; thieu: string[]; }
function kiemTraDayDu(bienBan: BienBanSuCo): KetQuaKiemTra {
  const thieu: string[] = [];
  if (bienBan.timeline.length === 0) thieu.push("timeline");
  if (bienBan.nguyenNhanHeThong === undefined || bienBan.nguyenNhanHeThong.trim() === "") thieu.push("nguyenNhanHeThong");
  if (bienBan.hanhDongTiepTheo.length === 0) thieu.push("hanhDongTiepTheo");
  return { dayDu: thieu.length === 0, thieu };
}

interface KetQuaXuLySuCo {
  p99: number | undefined; vuotSLO: boolean; alertKichHoat: boolean;
  nguoiTruc: string | undefined; dichVuCham: { tenDichVu: string; thoiGianMs: number } | undefined;
  soLogLoiTimDuoc: number; postmortemDayDu: boolean; postmortemThieu: string[];
}
function xuLySuCoDauCuoi(
  histDoTre: Histogram, nguongSLOMs: number, nguongDoTreCanhBaoMs: number, tyLeLoiNguoiDungThay: number,
  cacCaTruc: CaTruc[], dh: DongHoMoPhong, cacSpan: Span[], cacLog: BanGhiLogCoCauTruc[], hanhDongTiepTheo: HanhDongTiepTheo[]
): KetQuaXuLySuCo {
  const ketQuaRong: KetQuaXuLySuCo = {
    p99: undefined, vuotSLO: false, alertKichHoat: false, nguoiTruc: undefined,
    dichVuCham: undefined, soLogLoiTimDuoc: 0, postmortemDayDu: false, postmortemThieu: [],
  };
  const p99 = tinhPercentile(histDoTre, 99);
  if (p99 === undefined || p99 <= nguongSLOMs) return { ...ketQuaRong, p99 };

  const ts: TrangThaiHeThong = { cpuPhanTram: 0, tyLeLoiNguoiDungThay, doTreP99Ms: p99 };
  const alertKichHoat = canhBaoTheoTrieuChung(ts, 1, nguongDoTreCanhBaoMs);
  if (!alertKichHoat) return { ...ketQuaRong, p99, vuotSLO: true };

  ___
}

const histDoTreX: Histogram = taoHistogram("x");
ghiHistogram(histDoTreX, 500);
const cacCaTrucX: CaTruc[] = [{ tenNguoiTruc: "Duy", gioBatDau: 0, gioKetThuc: 100 }];
const dhX: DongHoMoPhong = { thoiGianHienTai: 10 };
const cacSpanX: Span[] = [
  { traceId: "tx", spanId: "r", parentSpanId: undefined, tenDichVu: "root", thoiGianBatDauMs: 0, thoiGianKetThucMs: 200 },
  { traceId: "tx", spanId: "c", parentSpanId: "r", tenDichVu: "dich-vu-x", thoiGianBatDauMs: 10, thoiGianKetThucMs: 190 },
];
const cacLogX: BanGhiLogCoCauTruc[] = [
  { thoiDiem: 5, mucDo: "loi", dichVu: "dich-vu-x", thongDiep: "loi ket noi", metadata: {} },
];
const hanhDongTiepTheoX: HanhDongTiepTheo[] = [{ moTa: "dieu tra dich-vu-x", nguoiPhuTrach: "Duy", trangThai: "dang-lam" }];

const ketQuaX = xuLySuCoDauCuoi(histDoTreX, 300, 350, 0.1, cacCaTrucX, dhX, cacSpanX, cacLogX, hanhDongTiepTheoX);
console.log(ketQuaX.nguoiTruc, ketQuaX.dichVuCham?.tenDichVu, ketQuaX.soLogLoiTimDuoc, ketQuaX.postmortemDayDu);
```

```typescript title=solution
interface Histogram { ten: string; cacGiaTri: number[]; }
function taoHistogram(ten: string): Histogram { return { ten, cacGiaTri: [] }; }
function ghiHistogram(h: Histogram, giaTri: number): void { h.cacGiaTri.push(giaTri); }
function tinhPercentile(h: Histogram, phanTram: number): number | undefined {
  if (h.cacGiaTri.length === 0) return undefined;
  const sapXep = [...h.cacGiaTri].sort((a, b) => a - b);
  const idx = Math.ceil((phanTram / 100) * sapXep.length) - 1;
  const idxAnToan = Math.max(0, Math.min(sapXep.length - 1, idx));
  return sapXep[idxAnToan];
}
interface TrangThaiHeThong { cpuPhanTram: number; tyLeLoiNguoiDungThay: number; doTreP99Ms: number; }
function canhBaoTheoTrieuChung(ts: TrangThaiHeThong, nguongTyLeLoi: number, nguongDoTreMs: number): boolean {
  return ts.tyLeLoiNguoiDungThay > nguongTyLeLoi || ts.doTreP99Ms > nguongDoTreMs;
}
interface DongHoMoPhong { thoiGianHienTai: number; }
interface CaTruc { tenNguoiTruc: string; gioBatDau: number; gioKetThuc: number; }
function aiDangTruc(cacCaTruc: CaTruc[], dh: DongHoMoPhong): string | undefined {
  const ca = cacCaTruc.find((c) => dh.thoiGianHienTai >= c.gioBatDau && dh.thoiGianHienTai < c.gioKetThuc);
  return ca?.tenNguoiTruc;
}
interface Span {
  traceId: string; spanId: string; parentSpanId: string | undefined;
  tenDichVu: string; thoiGianBatDauMs: number; thoiGianKetThucMs: number;
}
function timDichVuChiemNhieuNhat(cacSpan: Span[]): { tenDichVu: string; thoiGianMs: number } | undefined {
  const conSpan = cacSpan.filter((s) => s.parentSpanId !== undefined);
  if (conSpan.length === 0) return undefined;
  let max = conSpan[0]!;
  for (const s of conSpan) {
    if (s.thoiGianKetThucMs - s.thoiGianBatDauMs > max.thoiGianKetThucMs - max.thoiGianBatDauMs) max = s;
  }
  return { tenDichVu: max.tenDichVu, thoiGianMs: max.thoiGianKetThucMs - max.thoiGianBatDauMs };
}
interface BanGhiLogCoCauTruc { thoiDiem: number; mucDo: string; dichVu: string; thongDiep: string; metadata: Record<string, string>; }
function locLogTheoMucDoVaDichVu(cacLog: BanGhiLogCoCauTruc[], mucDo: string, dichVu: string): BanGhiLogCoCauTruc[] {
  return cacLog.filter((l) => l.mucDo === mucDo && l.dichVu === dichVu);
}
interface MocThoiGian { thoiDiem: number; moTa: string; }
interface HanhDongTiepTheo { moTa: string; nguoiPhuTrach: string; trangThai: string; }
interface BienBanSuCo {
  tenSuCo: string; timeline: MocThoiGian[];
  nguyenNhanHeThong: string | undefined; hanhDongTiepTheo: HanhDongTiepTheo[];
}
interface KetQuaKiemTra { dayDu: boolean; thieu: string[]; }
function kiemTraDayDu(bienBan: BienBanSuCo): KetQuaKiemTra {
  const thieu: string[] = [];
  if (bienBan.timeline.length === 0) thieu.push("timeline");
  if (bienBan.nguyenNhanHeThong === undefined || bienBan.nguyenNhanHeThong.trim() === "") thieu.push("nguyenNhanHeThong");
  if (bienBan.hanhDongTiepTheo.length === 0) thieu.push("hanhDongTiepTheo");
  return { dayDu: thieu.length === 0, thieu };
}

interface KetQuaXuLySuCo {
  p99: number | undefined; vuotSLO: boolean; alertKichHoat: boolean;
  nguoiTruc: string | undefined; dichVuCham: { tenDichVu: string; thoiGianMs: number } | undefined;
  soLogLoiTimDuoc: number; postmortemDayDu: boolean; postmortemThieu: string[];
}
function xuLySuCoDauCuoi(
  histDoTre: Histogram, nguongSLOMs: number, nguongDoTreCanhBaoMs: number, tyLeLoiNguoiDungThay: number,
  cacCaTruc: CaTruc[], dh: DongHoMoPhong, cacSpan: Span[], cacLog: BanGhiLogCoCauTruc[], hanhDongTiepTheo: HanhDongTiepTheo[]
): KetQuaXuLySuCo {
  const ketQuaRong: KetQuaXuLySuCo = {
    p99: undefined, vuotSLO: false, alertKichHoat: false, nguoiTruc: undefined,
    dichVuCham: undefined, soLogLoiTimDuoc: 0, postmortemDayDu: false, postmortemThieu: [],
  };
  const p99 = tinhPercentile(histDoTre, 99);
  if (p99 === undefined || p99 <= nguongSLOMs) return { ...ketQuaRong, p99 };

  const ts: TrangThaiHeThong = { cpuPhanTram: 0, tyLeLoiNguoiDungThay, doTreP99Ms: p99 };
  const alertKichHoat = canhBaoTheoTrieuChung(ts, 1, nguongDoTreCanhBaoMs);
  if (!alertKichHoat) return { ...ketQuaRong, p99, vuotSLO: true };

  const nguoiTruc = aiDangTruc(cacCaTruc, dh);
  const dichVuCham = timDichVuChiemNhieuNhat(cacSpan);
  const logLoi = dichVuCham === undefined ? [] : locLogTheoMucDoVaDichVu(cacLog, "loi", dichVuCham.tenDichVu);
  const bienBan: BienBanSuCo = {
    tenSuCo: "do-tre-cao",
    timeline: [
      { thoiDiem: dh.thoiGianHienTai, moTa: "Alert theo trieu chung kich hoat, p99 vuot SLO" },
      { thoiDiem: dh.thoiGianHienTai, moTa: `Nguoi truc (${nguoiTruc ?? "chua xac dinh"}) dung trace tim ra dich vu cham` },
    ],
    nguyenNhanHeThong:
      dichVuCham === undefined ? undefined
        : `He thong khong canh bao rieng cho ${dichVuCham.tenDichVu} khi thoi gian xu ly vuot ${dichVuCham.thoiGianMs}ms`,
    hanhDongTiepTheo,
  };
  const ketQuaKiemTra = kiemTraDayDu(bienBan);
  return {
    p99, vuotSLO: true, alertKichHoat: true, nguoiTruc, dichVuCham,
    soLogLoiTimDuoc: logLoi.length, postmortemDayDu: ketQuaKiemTra.dayDu, postmortemThieu: ketQuaKiemTra.thieu,
  };
}

const histDoTreX: Histogram = taoHistogram("x");
ghiHistogram(histDoTreX, 500);
const cacCaTrucX: CaTruc[] = [{ tenNguoiTruc: "Duy", gioBatDau: 0, gioKetThuc: 100 }];
const dhX: DongHoMoPhong = { thoiGianHienTai: 10 };
const cacSpanX: Span[] = [
  { traceId: "tx", spanId: "r", parentSpanId: undefined, tenDichVu: "root", thoiGianBatDauMs: 0, thoiGianKetThucMs: 200 },
  { traceId: "tx", spanId: "c", parentSpanId: "r", tenDichVu: "dich-vu-x", thoiGianBatDauMs: 10, thoiGianKetThucMs: 190 },
];
const cacLogX: BanGhiLogCoCauTruc[] = [
  { thoiDiem: 5, mucDo: "loi", dichVu: "dich-vu-x", thongDiep: "loi ket noi", metadata: {} },
];
const hanhDongTiepTheoX: HanhDongTiepTheo[] = [{ moTa: "dieu tra dich-vu-x", nguoiPhuTrach: "Duy", trangThai: "dang-lam" }];

const ketQuaX = xuLySuCoDauCuoi(histDoTreX, 300, 350, 0.1, cacCaTrucX, dhX, cacSpanX, cacLogX, hanhDongTiepTheoX);
console.log(ketQuaX.nguoiTruc, ketQuaX.dichVuCham?.tenDichVu, ketQuaX.soLogLoiTimDuoc, ketQuaX.postmortemDayDu);
```

```typescript title=test
const cacCaTrucT: CaTruc[] = [
  { tenNguoiTruc: "An", gioBatDau: 0, gioKetThuc: 24 },
  { tenNguoiTruc: "Binh", gioBatDau: 24, gioKetThuc: 48 },
];
const dhT: DongHoMoPhong = { thoiGianHienTai: 30 };
const cacSpanT: Span[] = [
  { traceId: "t9", spanId: "r", parentSpanId: undefined, tenDichVu: "root", thoiGianBatDauMs: 0, thoiGianKetThucMs: 500 },
  { traceId: "t9", spanId: "a", parentSpanId: "r", tenDichVu: "dich-vu-thanh-toan", thoiGianBatDauMs: 20, thoiGianKetThucMs: 460 },
  { traceId: "t9", spanId: "b", parentSpanId: "a", tenDichVu: "dich-vu-ngan-hang", thoiGianBatDauMs: 60, thoiGianKetThucMs: 420 },
];
const cacLogT: BanGhiLogCoCauTruc[] = [
  { thoiDiem: 19, mucDo: "loi", dichVu: "dich-vu-thanh-toan", thongDiep: "loi", metadata: {} },
  { thoiDiem: 21, mucDo: "loi", dichVu: "dich-vu-ngan-hang", thongDiep: "timeout", metadata: {} },
];
const hanhDongT: HanhDongTiepTheo[] = [{ moTa: "sua", nguoiPhuTrach: "An", trangThai: "dang-lam" }];

const histDayDuT: Histogram = taoHistogram("day-du");
for (const v of [120, 130, 140, 150, 800]) ghiHistogram(histDayDuT, v);
const ketQuaDayDuT = xuLySuCoDauCuoi(histDayDuT, 300, 400, 0.5, cacCaTrucT, dhT, cacSpanT, cacLogT, hanhDongT);
if (ketQuaDayDuT.nguoiTruc !== "Binh") throw new Error("gio 30 phai la Binh dang truc");
if (ketQuaDayDuT.dichVuCham?.tenDichVu !== "dich-vu-thanh-toan") throw new Error("dich vu cham nhat phai la dich-vu-thanh-toan (440ms)");
if (ketQuaDayDuT.soLogLoiTimDuoc !== 1) throw new Error("phai tim DUNG 1 log loi cua dich-vu-thanh-toan");
if (ketQuaDayDuT.postmortemDayDu !== true) throw new Error("postmortem phai DAY DU khi co hanh dong tiep theo");

const histKhongVoT: Histogram = taoHistogram("khong-vo");
for (const v of [100, 110, 120]) ghiHistogram(histKhongVoT, v);
const ketQuaKhongVoT = xuLySuCoDauCuoi(histKhongVoT, 300, 400, 0.1, cacCaTrucT, dhT, cacSpanT, cacLogT, hanhDongT);
if (ketQuaKhongVoT.vuotSLO !== false) throw new Error("p99 duoi SLO thi vuotSLO phai la false");
if (ketQuaKhongVoT.nguoiTruc !== undefined) throw new Error("khong vuot SLO thi khong duoc tim nguoi truc");

const histThieuHanhDongT: Histogram = taoHistogram("thieu-hanh-dong");
for (const v of [120, 130, 140, 150, 800]) ghiHistogram(histThieuHanhDongT, v);
const ketQuaThieuHanhDongT = xuLySuCoDauCuoi(histThieuHanhDongT, 300, 400, 0.5, cacCaTrucT, dhT, cacSpanT, cacLogT, []);
if (ketQuaThieuHanhDongT.postmortemDayDu !== false) throw new Error("khong co hanh dong tiep theo nao thi postmortem phai CHUA DAY DU");
if (!ketQuaThieuHanhDongT.postmortemThieu.includes("hanhDongTiepTheo")) throw new Error("thieu phai chua 'hanhDongTiepTheo'");
```

:::hints
- kind: attention
  body: "Sau khi alertKichHoat da xac nhan true, can: tim nguoiTruc bang aiDangTruc(cacCaTruc, dh); tim dichVuCham bang timDichVuChiemNhieuNhat(cacSpan); tinh logLoi (mang rong neu dichVuCham la undefined, nguoc lai locLogTheoMucDoVaDichVu(cacLog, 'loi', dichVuCham.tenDichVu)); dung bienBan de goi kiemTraDayDu; roi tra ve object KetQuaXuLySuCo day du 8 field."
- kind: strategy
  body: "const nguoiTruc = aiDangTruc(cacCaTruc, dh); const dichVuCham = timDichVuChiemNhieuNhat(cacSpan); const logLoi = dichVuCham === undefined ? [] : locLogTheoMucDoVaDichVu(cacLog, 'loi', dichVuCham.tenDichVu); const bienBan: BienBanSuCo = { tenSuCo: 'do-tre-cao', timeline: [{ thoiDiem: dh.thoiGianHienTai, moTa: 'canh bao kich hoat' }], nguyenNhanHeThong: dichVuCham === undefined ? undefined : `${dichVuCham.tenDichVu} cham ${dichVuCham.thoiGianMs}ms`, hanhDongTiepTheo }; const ketQuaKiemTra = kiemTraDayDu(bienBan); return { p99, vuotSLO: true, alertKichHoat: true, nguoiTruc, dichVuCham, soLogLoiTimDuoc: logLoi.length, postmortemDayDu: ketQuaKiemTra.dayDu, postmortemThieu: ketQuaKiemTra.thieu };"
- kind: one-line
  body: "Sao chep dung logic o phan Strategy -- tim nguoiTruc, dichVuCham, loc logLoi THEO dichVuCham.tenDichVu, dung bienBan, kiemTraDayDu, roi tra ve object ket qua."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "Duy dich-vu-x 1 true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Từ MỘT con số histogram bất thường, TỚI một biên bản sự cố đầy đủ — không
có khoảng trống nào để một sự cố thật trôi qua mà không ai biết, VÀ không
có cảnh báo giả nào đánh thức người trực vô ích. Quest "Quan sát và cảnh
báo" đã xong — hệ thống giờ biết TỰ kể lại chuyện gì đã xảy ra với chính
nó, theo ĐÚNG thứ tự, bằng số liệu THẬT.
::::

::::reflect{#nghi-lai}
`xuLySuCoDauCuoi` không hề PHÁT minh thêm khái niệm nào MỚI — nó gọi ĐÚNG
tám hàm đã xây riêng lẻ Ở tám bài trước, theo ĐÚNG thứ tự nhân QUẢ: một
con số (bài 3) VƯỢT một mục tiêu (bài 5), kích hoạt một điều kiện (bài 6),
xác định một người (bài 7), người ĐÓ dùng một công cụ (bài 4) để tìm một
CHI tiết (bài 2), VÀ kết thúc bằng một tài liệu có thể kiểm tra được
(bài 8). Điểm mấu chốt không nằm Ở BẤT KỲ một hàm riêng lẻ nào — nó nằm Ở
việc ĐẦU RA của trạm này trở thành ĐẦU VÀO của trạm kế tiếp
(`dichVuCham.tenDichVu` chảy thẳng VÀO `locLogTheoMucDoVaDichVu`), VÀ
pipeline sẵn sàng DỪNG SỚM Ở bất kỳ trạm nào chưa đủ căn cứ, thay vì chạy
hết cho có.
::::

::::checkpoint{mastery=0.86}
::::
