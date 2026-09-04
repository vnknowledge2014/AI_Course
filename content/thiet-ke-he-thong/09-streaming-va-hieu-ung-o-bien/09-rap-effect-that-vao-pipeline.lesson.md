---
id: thiet-ke-he-thong.streaming-va-hieu-ung-o-bien.rap-effect-that-vao-pipeline
title: "Ráp effect thật vào cuối pipeline: gọi ghiRaNgoai đúng số lần, không hơn không kém"
summary: "chayPipelineThat(kho, cacSuKienMoi, cauHinh) la VO: goi xuLyPipeline (bai 8, THUAN) roi lap qua ketQua.cacLoCanGui de goi ghiRaNgoai (bai 5) DUNG mot lan cho MOI lo -- diem noi 'functional core, imperative shell' hoan chinh cho CA he thong streaming, cung tinh than quest 'Nhat ky bat bien va fold' bai 7 nhung o QUY MO nhieu buoc thay vi mot lenh don."
locale: vi
track: thiet-ke-he-thong
module: streaming-va-hieu-ung-o-bien
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [sd.fp.rap-effect-that-vao-pipeline]
requires: [sd.fp.pipeline-thuan-dau-cuoi]
concepts: [sd.fp.rap-effect-that-vao-pipeline]
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
`xuLyPipeline` (bài trước) tính ra `cacLoCanGui` — nhưng đó vẫn chỉ LÀ
dữ liệu, nằm yên trong bộ nhớ. Chưa có gì rời khỏi hệ thống. Bài học
cuối trước BOSS này ráp đúng MỘT vỏ mệnh lệnh lên trên pipeline thuần
đó — nơi DUY NHẤT trong toàn bộ quest gọi effect thật, VÀ gọi ĐÚNG số
lần cần thiết.
::::

::::explain{#vo-goi-effect-dung-so-lan}
`chayPipelineThat` là vỏ: nó gọi `xuLyPipeline` (lõi thuần, bài 8) để
lấy `ketQua`, rồi lặp qua `ketQua.cacLoCanGui` — với MỖI lô, gọi
`ghiRaNgoai` (bài 5) đúng MỘT lần. Không có lô nào bị bỏ sót, không
có lô nào bị ghi hai lần, VÀ không có effect nào xảy ra TRƯỚC khi lõi
đã quyết định xong toàn bộ:

```typescript title=readonly
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KhungGom { chiSoKhung: number; tong: number; soSuKien: number; }
function chiSoKhung(thoiDiem: number, kichThuocKhungMs: number): number {
  return Math.floor(thoiDiem / kichThuocKhungMs);
}
function themVaoKhung(cacKhung: Map<number, KhungGom>, sk: SuKien, kichThuocKhungMs: number): Map<number, KhungGom> {
  const idx = chiSoKhung(sk.thoiDiem, kichThuocKhungMs);
  const khungCu = cacKhung.get(idx) ?? { chiSoKhung: idx, tong: 0, soSuKien: 0 };
  const khungMoi: KhungGom = { chiSoKhung: idx, tong: khungCu.tong + sk.soLuong, soSuKien: khungCu.soSuKien + 1 };
  const banSaoMoi = new Map(cacKhung);
  banSaoMoi.set(idx, khungMoi);
  return banSaoMoi;
}
function gomTheoKhung(cacSuKien: SuKien[], kichThuocKhungMs: number): Map<number, KhungGom> {
  return cacSuKien.reduce((khung, sk) => themVaoKhung(khung, sk, kichThuocKhungMs), new Map<number, KhungGom>());
}
interface KetQuaGomLo { loMoi: SuKien[] | undefined; bufferMoi: SuKien[]; }
function quyetDinhGomLo(buffer: SuKien[], suKienMoi: SuKien, kichThuocLo: number, thoiGianChoMs: number): KetQuaGomLo {
  const bufferSau = [...buffer, suKienMoi];
  const dauBuffer = bufferSau[0];
  const daDuKichThuoc = bufferSau.length >= kichThuocLo;
  const daHetThoiGianCho = dauBuffer !== undefined && suKienMoi.thoiDiem - dauBuffer.thoiDiem >= thoiGianChoMs;
  if (daDuKichThuoc || daHetThoiGianCho) {
    return { loMoi: bufferSau, bufferMoi: [] };
  }
  return { loMoi: undefined, bufferMoi: bufferSau };
}
type TinHieuBackpressure = "nhan" | "tu_choi_tam" | "chan_lai";
function quyetDinhNhanSuKien(kichThuocBufferHienTai: number, gioiHanBuffer: number): TinHieuBackpressure {
  if (kichThuocBufferHienTai >= gioiHanBuffer) return "chan_lai";
  const tiLe = kichThuocBufferHienTai / gioiHanBuffer;
  if (tiLe >= 0.8) return "tu_choi_tam";
  return "nhan";
}
interface CauHinhPipeline { kichThuocKhungMs: number; kichThuocLo: number; thoiGianChoMs: number; gioiHanBuffer: number; }
interface KetQuaPipeline {
  cacKhung: Map<number, KhungGom>;
  cacLoCanGui: SuKien[][];
  tinHieuBackpressure: TinHieuBackpressure;
}
function xuLyPipeline(cacSuKien: SuKien[], cauHinh: CauHinhPipeline): KetQuaPipeline {
  const cacKhung = gomTheoKhung(cacSuKien, cauHinh.kichThuocKhungMs);
  const { cacLoCanGui } = cacSuKien.reduce(
    (tich, sk) => {
      const kq = quyetDinhGomLo(tich.buffer, sk, cauHinh.kichThuocLo, cauHinh.thoiGianChoMs);
      if (kq.loMoi !== undefined) {
        return { buffer: kq.bufferMoi, cacLoCanGui: [...tich.cacLoCanGui, kq.loMoi] };
      }
      return { buffer: kq.bufferMoi, cacLoCanGui: tich.cacLoCanGui };
    },
    { buffer: [] as SuKien[], cacLoCanGui: [] as SuKien[][] }
  );
  const tinHieuBackpressure = quyetDinhNhanSuKien(cacSuKien.length, cauHinh.gioiHanBuffer);
  return { cacKhung, cacLoCanGui, tinHieuBackpressure };
}

interface NhatKyGhiRaNgoai { cacLoDaGhi: SuKien[][]; }
function taoNhatKyGhiRaNgoai(): NhatKyGhiRaNgoai { return { cacLoDaGhi: [] }; }
function ghiRaNgoai(nhatKy: NhatKyGhiRaNgoai, lo: SuKien[]): void {
  nhatKy.cacLoDaGhi.push(lo);
}

interface KhoPipeline { nhatKyGhi: NhatKyGhiRaNgoai; }
function taoKhoPipeline(): KhoPipeline { return { nhatKyGhi: taoNhatKyGhiRaNgoai() }; }

function chayPipelineThat(kho: KhoPipeline, cacSuKienMoi: SuKien[], cauHinh: CauHinhPipeline): KetQuaPipeline {
  const ketQua = xuLyPipeline(cacSuKienMoi, cauHinh);
  for (const lo of ketQua.cacLoCanGui) {
    ghiRaNgoai(kho.nhatKyGhi, lo);
  }
  return ketQua;
}

const suKien = (id: string, thoiDiem: number, soLuong: number): SuKien => ({ id, nguon: "edge-1", thoiDiem, soLuong });
const cacSuKienMoi: SuKien[] = [
  suKien("e1", 0, 1), suKien("e2", 100, 1), suKien("e3", 200, 1),
  suKien("e4", 300, 1), suKien("e5", 400, 1), suKien("e6", 500, 1),
  suKien("e7", 600, 1),
];

const kho = taoKhoPipeline();
const cauHinh: CauHinhPipeline = { kichThuocKhungMs: 10000, kichThuocLo: 3, thoiGianChoMs: 100000, gioiHanBuffer: 1000 };
const ketQua = chayPipelineThat(kho, cacSuKienMoi, cauHinh);

console.log("so lo LOI da quyet dinh (ketQua.cacLoCanGui.length):", ketQua.cacLoCanGui.length);
console.log("so lo VO da GHI RA NGOAI that su (kho.nhatKyGhi.cacLoDaGhi.length):", kho.nhatKyGhi.cacLoDaGhi.length);
console.log("hai con so nay co KHOP nhau khong?", ketQua.cacLoCanGui.length === kho.nhatKyGhi.cacLoDaGhi.length);
console.log("noi dung lo dau tien da ghi:", JSON.stringify(kho.nhatKyGhi.cacLoDaGhi[0]?.map((sk) => sk.id)));
console.log("noi dung lo thu hai da ghi:", JSON.stringify(kho.nhatKyGhi.cacLoDaGhi[1]?.map((sk) => sk.id)));
```

```text title=readonly
so lo LOI da quyet dinh (ketQua.cacLoCanGui.length): 2
so lo VO da GHI RA NGOAI that su (kho.nhatKyGhi.cacLoDaGhi.length): 2
hai con so nay co KHOP nhau khong? true
noi dung lo dau tien da ghi: ["e1","e2","e3"]
noi dung lo thu hai da ghi: ["e4","e5","e6"]
```

Bảy sự kiện, `kichThuocLo: 3` — lõi quyết định ĐÚNG `2` lô (`e1..e3`
VÀ `e4..e6`), để lại `e7` trong buffer (chưa đủ để gom). Vỏ
`chayPipelineThat` ghi RA NGOÀI ĐÚNG `2` lần — không nhiều hơn (không
tự "ghi thêm" lô chưa đủ), không ít hơn (không "bỏ sót" lô đã sẵn
sàng). Đây chính LÀ điểm nối hoàn chỉnh giữa lõi thuần VÀ effect Ở
biên cho CẢ pipeline nhiều bước, không chỉ một hàm đơn.
::::

::::example{#nhieu-dot-goi-lien-tiep}
`chayPipelineThat` xử lý MỖI lần gọi như một đợt sự kiện ĐỘC LẬP — nó
không tự動 duy trì buffer xuyên suốt nhiều lần gọi (đó là một giới hạn
CHỦ ĐỘNG của thiết kế Ở bài học này, không phải một lỗi). Gọi nhiều
lần liên tiếp cho thấy số lô tích luỹ đúng theo TỪNG đợt:

```typescript title=readonly
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KhungGom { chiSoKhung: number; tong: number; soSuKien: number; }
function chiSoKhung(thoiDiem: number, kichThuocKhungMs: number): number {
  return Math.floor(thoiDiem / kichThuocKhungMs);
}
function themVaoKhung(cacKhung: Map<number, KhungGom>, sk: SuKien, kichThuocKhungMs: number): Map<number, KhungGom> {
  const idx = chiSoKhung(sk.thoiDiem, kichThuocKhungMs);
  const khungCu = cacKhung.get(idx) ?? { chiSoKhung: idx, tong: 0, soSuKien: 0 };
  const khungMoi: KhungGom = { chiSoKhung: idx, tong: khungCu.tong + sk.soLuong, soSuKien: khungCu.soSuKien + 1 };
  const banSaoMoi = new Map(cacKhung);
  banSaoMoi.set(idx, khungMoi);
  return banSaoMoi;
}
function gomTheoKhung(cacSuKien: SuKien[], kichThuocKhungMs: number): Map<number, KhungGom> {
  return cacSuKien.reduce((khung, sk) => themVaoKhung(khung, sk, kichThuocKhungMs), new Map<number, KhungGom>());
}
interface KetQuaGomLo { loMoi: SuKien[] | undefined; bufferMoi: SuKien[]; }
function quyetDinhGomLo(buffer: SuKien[], suKienMoi: SuKien, kichThuocLo: number, thoiGianChoMs: number): KetQuaGomLo {
  const bufferSau = [...buffer, suKienMoi];
  const dauBuffer = bufferSau[0];
  const daDuKichThuoc = bufferSau.length >= kichThuocLo;
  const daHetThoiGianCho = dauBuffer !== undefined && suKienMoi.thoiDiem - dauBuffer.thoiDiem >= thoiGianChoMs;
  if (daDuKichThuoc || daHetThoiGianCho) {
    return { loMoi: bufferSau, bufferMoi: [] };
  }
  return { loMoi: undefined, bufferMoi: bufferSau };
}
type TinHieuBackpressure = "nhan" | "tu_choi_tam" | "chan_lai";
function quyetDinhNhanSuKien(kichThuocBufferHienTai: number, gioiHanBuffer: number): TinHieuBackpressure {
  if (kichThuocBufferHienTai >= gioiHanBuffer) return "chan_lai";
  const tiLe = kichThuocBufferHienTai / gioiHanBuffer;
  if (tiLe >= 0.8) return "tu_choi_tam";
  return "nhan";
}
interface CauHinhPipeline { kichThuocKhungMs: number; kichThuocLo: number; thoiGianChoMs: number; gioiHanBuffer: number; }
interface KetQuaPipeline {
  cacKhung: Map<number, KhungGom>;
  cacLoCanGui: SuKien[][];
  tinHieuBackpressure: TinHieuBackpressure;
}
function xuLyPipeline(cacSuKien: SuKien[], cauHinh: CauHinhPipeline): KetQuaPipeline {
  const cacKhung = gomTheoKhung(cacSuKien, cauHinh.kichThuocKhungMs);
  const { cacLoCanGui } = cacSuKien.reduce(
    (tich, sk) => {
      const kq = quyetDinhGomLo(tich.buffer, sk, cauHinh.kichThuocLo, cauHinh.thoiGianChoMs);
      if (kq.loMoi !== undefined) {
        return { buffer: kq.bufferMoi, cacLoCanGui: [...tich.cacLoCanGui, kq.loMoi] };
      }
      return { buffer: kq.bufferMoi, cacLoCanGui: tich.cacLoCanGui };
    },
    { buffer: [] as SuKien[], cacLoCanGui: [] as SuKien[][] }
  );
  const tinHieuBackpressure = quyetDinhNhanSuKien(cacSuKien.length, cauHinh.gioiHanBuffer);
  return { cacKhung, cacLoCanGui, tinHieuBackpressure };
}
interface NhatKyGhiRaNgoai { cacLoDaGhi: SuKien[][]; }
function taoNhatKyGhiRaNgoai(): NhatKyGhiRaNgoai { return { cacLoDaGhi: [] }; }
function ghiRaNgoai(nhatKy: NhatKyGhiRaNgoai, lo: SuKien[]): void {
  nhatKy.cacLoDaGhi.push(lo);
}
interface KhoPipeline { nhatKyGhi: NhatKyGhiRaNgoai; }
function taoKhoPipeline(): KhoPipeline { return { nhatKyGhi: taoNhatKyGhiRaNgoai() }; }
function chayPipelineThat(kho: KhoPipeline, cacSuKienMoi: SuKien[], cauHinh: CauHinhPipeline): KetQuaPipeline {
  const ketQua = xuLyPipeline(cacSuKienMoi, cauHinh);
  for (const lo of ketQua.cacLoCanGui) {
    ghiRaNgoai(kho.nhatKyGhi, lo);
  }
  return ketQua;
}

const suKien = (id: string, thoiDiem: number, soLuong: number): SuKien => ({ id, nguon: "edge-1", thoiDiem, soLuong });
const cauHinh: CauHinhPipeline = { kichThuocKhungMs: 10000, kichThuocLo: 2, thoiGianChoMs: 100000, gioiHanBuffer: 1000 };

// mo phong nhieu DOT su kien den theo thoi gian -- moi dot goi chayPipelineThat RIENG
const kho = taoKhoPipeline();
chayPipelineThat(kho, [suKien("a1", 0, 1), suKien("a2", 100, 1)], cauHinh);
console.log("sau dot 1 (2 su kien, kichThuocLo=2): da ghi", kho.nhatKyGhi.cacLoDaGhi.length, "lo");

chayPipelineThat(kho, [suKien("b1", 200, 1)], cauHinh);
console.log("sau dot 2 (1 su kien, khong du gom): da ghi", kho.nhatKyGhi.cacLoDaGhi.length, "lo (khong doi)");

chayPipelineThat(kho, [suKien("c1", 300, 1), suKien("c2", 400, 1), suKien("c3", 500, 1)], cauHinh);
console.log("sau dot 3 (3 su kien, kichThuocLo=2 -> 1 lo): da ghi", kho.nhatKyGhi.cacLoDaGhi.length, "lo");

console.log("--- goi voi mang RONG (khong su kien nao) ---");
const khoRong = taoKhoPipeline();
const ketQuaRong = chayPipelineThat(khoRong, [], cauHinh);
console.log("ketQua.cacLoCanGui.length:", ketQuaRong.cacLoCanGui.length);
console.log("khoRong.nhatKyGhi.cacLoDaGhi.length:", khoRong.nhatKyGhi.cacLoDaGhi.length);
```

```text title=readonly
sau dot 1 (2 su kien, kichThuocLo=2): da ghi 1 lo
sau dot 2 (1 su kien, khong du gom): da ghi 1 lo (khong doi)
sau dot 3 (3 su kien, kichThuocLo=2 -> 1 lo): da ghi 2 lo
--- goi voi mang RONG (khong su kien nao) ---
ketQua.cacLoCanGui.length: 0
khoRong.nhatKyGhi.cacLoDaGhi.length: 0
```

Đợt `2` chỉ có MỘT sự kiện (`b1`) — không đủ `kichThuocLo: 2` — nên
KHÔNG có lô nào được ghi thêm, VÀ vì `chayPipelineThat` không giữ
buffer giữa các lần gọi, `b1` không hề được cộng dồn với đợt kế tiếp.
Đợt `3` xử lý ĐÚNG BA sự kiện của riêng nó, tạo `1` lô mới. Gọi với
mảng RỖNG cho ra `0` lô Ở CẢ hai phía — không có lô "giả" nào được ghi
ra chỉ vì hàm được gọi.
::::

::::predict{#doan-mang-rong-khong-goi-effect commitOnce}
Gọi `chayPipelineThat(taoKhoPipeline(), [], cauHinh)` — với
`cacSuKienMoi` LÀ một mảng RỖNG. `ghiRaNgoai` được gọi bao nhiêu lần
trong lệnh gọi ĐÓ?

:::opt{correct}
`0` lần — `xuLyPipeline([], cauHinh)` cho `cacLoCanGui` LÀ một mảng
rỗng (không sự kiện nào để gom), VÀ vòng `for (const lo of
ketQua.cacLoCanGui)` không chạy thân vòng lặp lần nào khi mảng rỗng
:::
:::opt
`1` lần — với một lô RỖNG (`[]`), để báo hiệu cho hệ thống bên ngoài
biết "đợt này không có dữ liệu"
::why
Nhầm "không có dữ liệu" LÀ một trường hợp CẦN báo hiệu, với hành vi
THẬT của vòng lặp `for...of` trên một mảng rỗng — nhưng `chayPipelineThat`
không hề có logic đặc biệt nào cho trường hợp `cacLoCanGui` rỗng, nó
chỉ đơn giản lặp qua PHẦN TỬ đã có.

Chỗ lệch: `xuLyPipeline` không bao giờ tạo ra một "lô rỗng" — mọi
phần tử trong `cacLoCanGui` đều LÀ kết quả của `quyetDinhGomLo` khi ĐÃ
đủ điều kiện (kích thước hoặc thời gian chờ), VÀ với `cacSuKien` rỗng,
`reduce` không hề chạy qua phần tử nào để tạo ra lô. Vòng
`for (const lo of ketQua.cacLoCanGui)` khi `cacLoCanGui.length === 0`
đơn giản là không thực thi thân vòng lặp — không có "lần gọi rỗng" nào
cả. Đây chính LÀ điều bài học này muốn khẳng định: số lần gọi
`ghiRaNgoai` LUÔN khớp CHÍNH XÁC với số phần tử trong `cacLoCanGui`,
kể cả khi con số đó LÀ `0`.
::
:::
::::

::::code{#viet_chay_pipeline_that}
Hoàn thiện `chayPipelineThat` — gọi `xuLyPipeline(cacSuKienMoi,
cauHinh)` để lấy `ketQua`. Lặp qua `ketQua.cacLoCanGui`, gọi
`ghiRaNgoai(kho.nhatKyGhi, lo)` cho MỖI `lo`. Trả về `ketQua`.

```typescript title=starter
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KhungGom { chiSoKhung: number; tong: number; soSuKien: number; }
function chiSoKhung(thoiDiem: number, kichThuocKhungMs: number): number {
  return Math.floor(thoiDiem / kichThuocKhungMs);
}
function themVaoKhung(cacKhung: Map<number, KhungGom>, sk: SuKien, kichThuocKhungMs: number): Map<number, KhungGom> {
  const idx = chiSoKhung(sk.thoiDiem, kichThuocKhungMs);
  const khungCu = cacKhung.get(idx) ?? { chiSoKhung: idx, tong: 0, soSuKien: 0 };
  const khungMoi: KhungGom = { chiSoKhung: idx, tong: khungCu.tong + sk.soLuong, soSuKien: khungCu.soSuKien + 1 };
  const banSaoMoi = new Map(cacKhung);
  banSaoMoi.set(idx, khungMoi);
  return banSaoMoi;
}
function gomTheoKhung(cacSuKien: SuKien[], kichThuocKhungMs: number): Map<number, KhungGom> {
  return cacSuKien.reduce((khung, sk) => themVaoKhung(khung, sk, kichThuocKhungMs), new Map<number, KhungGom>());
}
interface KetQuaGomLo { loMoi: SuKien[] | undefined; bufferMoi: SuKien[]; }
function quyetDinhGomLo(buffer: SuKien[], suKienMoi: SuKien, kichThuocLo: number, thoiGianChoMs: number): KetQuaGomLo {
  const bufferSau = [...buffer, suKienMoi];
  const dauBuffer = bufferSau[0];
  const daDuKichThuoc = bufferSau.length >= kichThuocLo;
  const daHetThoiGianCho = dauBuffer !== undefined && suKienMoi.thoiDiem - dauBuffer.thoiDiem >= thoiGianChoMs;
  if (daDuKichThuoc || daHetThoiGianCho) {
    return { loMoi: bufferSau, bufferMoi: [] };
  }
  return { loMoi: undefined, bufferMoi: bufferSau };
}
type TinHieuBackpressure = "nhan" | "tu_choi_tam" | "chan_lai";
function quyetDinhNhanSuKien(kichThuocBufferHienTai: number, gioiHanBuffer: number): TinHieuBackpressure {
  if (kichThuocBufferHienTai >= gioiHanBuffer) return "chan_lai";
  const tiLe = kichThuocBufferHienTai / gioiHanBuffer;
  if (tiLe >= 0.8) return "tu_choi_tam";
  return "nhan";
}
interface CauHinhPipeline { kichThuocKhungMs: number; kichThuocLo: number; thoiGianChoMs: number; gioiHanBuffer: number; }
interface KetQuaPipeline {
  cacKhung: Map<number, KhungGom>;
  cacLoCanGui: SuKien[][];
  tinHieuBackpressure: TinHieuBackpressure;
}
function xuLyPipeline(cacSuKien: SuKien[], cauHinh: CauHinhPipeline): KetQuaPipeline {
  const cacKhung = gomTheoKhung(cacSuKien, cauHinh.kichThuocKhungMs);
  const { cacLoCanGui } = cacSuKien.reduce(
    (tich, sk) => {
      const kq = quyetDinhGomLo(tich.buffer, sk, cauHinh.kichThuocLo, cauHinh.thoiGianChoMs);
      if (kq.loMoi !== undefined) {
        return { buffer: kq.bufferMoi, cacLoCanGui: [...tich.cacLoCanGui, kq.loMoi] };
      }
      return { buffer: kq.bufferMoi, cacLoCanGui: tich.cacLoCanGui };
    },
    { buffer: [] as SuKien[], cacLoCanGui: [] as SuKien[][] }
  );
  const tinHieuBackpressure = quyetDinhNhanSuKien(cacSuKien.length, cauHinh.gioiHanBuffer);
  return { cacKhung, cacLoCanGui, tinHieuBackpressure };
}
interface NhatKyGhiRaNgoai { cacLoDaGhi: SuKien[][]; }
function taoNhatKyGhiRaNgoai(): NhatKyGhiRaNgoai { return { cacLoDaGhi: [] }; }
function ghiRaNgoai(nhatKy: NhatKyGhiRaNgoai, lo: SuKien[]): void {
  nhatKy.cacLoDaGhi.push(lo);
}
interface KhoPipeline { nhatKyGhi: NhatKyGhiRaNgoai; }
function taoKhoPipeline(): KhoPipeline { return { nhatKyGhi: taoNhatKyGhiRaNgoai() }; }

function chayPipelineThat(kho: KhoPipeline, cacSuKienMoi: SuKien[], cauHinh: CauHinhPipeline): KetQuaPipeline {
  ___
}

const khoX = taoKhoPipeline();
const dsX: SuKien[] = [
  { id: "x1", nguon: "e", thoiDiem: 0, soLuong: 1 },
  { id: "x2", nguon: "e", thoiDiem: 100, soLuong: 1 },
];
chayPipelineThat(khoX, dsX, { kichThuocKhungMs: 1000, kichThuocLo: 2, thoiGianChoMs: 5000, gioiHanBuffer: 10 });
console.log(khoX.nhatKyGhi.cacLoDaGhi.length);
```

```typescript title=solution
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KhungGom { chiSoKhung: number; tong: number; soSuKien: number; }
function chiSoKhung(thoiDiem: number, kichThuocKhungMs: number): number {
  return Math.floor(thoiDiem / kichThuocKhungMs);
}
function themVaoKhung(cacKhung: Map<number, KhungGom>, sk: SuKien, kichThuocKhungMs: number): Map<number, KhungGom> {
  const idx = chiSoKhung(sk.thoiDiem, kichThuocKhungMs);
  const khungCu = cacKhung.get(idx) ?? { chiSoKhung: idx, tong: 0, soSuKien: 0 };
  const khungMoi: KhungGom = { chiSoKhung: idx, tong: khungCu.tong + sk.soLuong, soSuKien: khungCu.soSuKien + 1 };
  const banSaoMoi = new Map(cacKhung);
  banSaoMoi.set(idx, khungMoi);
  return banSaoMoi;
}
function gomTheoKhung(cacSuKien: SuKien[], kichThuocKhungMs: number): Map<number, KhungGom> {
  return cacSuKien.reduce((khung, sk) => themVaoKhung(khung, sk, kichThuocKhungMs), new Map<number, KhungGom>());
}
interface KetQuaGomLo { loMoi: SuKien[] | undefined; bufferMoi: SuKien[]; }
function quyetDinhGomLo(buffer: SuKien[], suKienMoi: SuKien, kichThuocLo: number, thoiGianChoMs: number): KetQuaGomLo {
  const bufferSau = [...buffer, suKienMoi];
  const dauBuffer = bufferSau[0];
  const daDuKichThuoc = bufferSau.length >= kichThuocLo;
  const daHetThoiGianCho = dauBuffer !== undefined && suKienMoi.thoiDiem - dauBuffer.thoiDiem >= thoiGianChoMs;
  if (daDuKichThuoc || daHetThoiGianCho) {
    return { loMoi: bufferSau, bufferMoi: [] };
  }
  return { loMoi: undefined, bufferMoi: bufferSau };
}
type TinHieuBackpressure = "nhan" | "tu_choi_tam" | "chan_lai";
function quyetDinhNhanSuKien(kichThuocBufferHienTai: number, gioiHanBuffer: number): TinHieuBackpressure {
  if (kichThuocBufferHienTai >= gioiHanBuffer) return "chan_lai";
  const tiLe = kichThuocBufferHienTai / gioiHanBuffer;
  if (tiLe >= 0.8) return "tu_choi_tam";
  return "nhan";
}
interface CauHinhPipeline { kichThuocKhungMs: number; kichThuocLo: number; thoiGianChoMs: number; gioiHanBuffer: number; }
interface KetQuaPipeline {
  cacKhung: Map<number, KhungGom>;
  cacLoCanGui: SuKien[][];
  tinHieuBackpressure: TinHieuBackpressure;
}
function xuLyPipeline(cacSuKien: SuKien[], cauHinh: CauHinhPipeline): KetQuaPipeline {
  const cacKhung = gomTheoKhung(cacSuKien, cauHinh.kichThuocKhungMs);
  const { cacLoCanGui } = cacSuKien.reduce(
    (tich, sk) => {
      const kq = quyetDinhGomLo(tich.buffer, sk, cauHinh.kichThuocLo, cauHinh.thoiGianChoMs);
      if (kq.loMoi !== undefined) {
        return { buffer: kq.bufferMoi, cacLoCanGui: [...tich.cacLoCanGui, kq.loMoi] };
      }
      return { buffer: kq.bufferMoi, cacLoCanGui: tich.cacLoCanGui };
    },
    { buffer: [] as SuKien[], cacLoCanGui: [] as SuKien[][] }
  );
  const tinHieuBackpressure = quyetDinhNhanSuKien(cacSuKien.length, cauHinh.gioiHanBuffer);
  return { cacKhung, cacLoCanGui, tinHieuBackpressure };
}
interface NhatKyGhiRaNgoai { cacLoDaGhi: SuKien[][]; }
function taoNhatKyGhiRaNgoai(): NhatKyGhiRaNgoai { return { cacLoDaGhi: [] }; }
function ghiRaNgoai(nhatKy: NhatKyGhiRaNgoai, lo: SuKien[]): void {
  nhatKy.cacLoDaGhi.push(lo);
}
interface KhoPipeline { nhatKyGhi: NhatKyGhiRaNgoai; }
function taoKhoPipeline(): KhoPipeline { return { nhatKyGhi: taoNhatKyGhiRaNgoai() }; }

function chayPipelineThat(kho: KhoPipeline, cacSuKienMoi: SuKien[], cauHinh: CauHinhPipeline): KetQuaPipeline {
  const ketQua = xuLyPipeline(cacSuKienMoi, cauHinh);
  for (const lo of ketQua.cacLoCanGui) {
    ghiRaNgoai(kho.nhatKyGhi, lo);
  }
  return ketQua;
}

const khoX = taoKhoPipeline();
const dsX: SuKien[] = [
  { id: "x1", nguon: "e", thoiDiem: 0, soLuong: 1 },
  { id: "x2", nguon: "e", thoiDiem: 100, soLuong: 1 },
];
chayPipelineThat(khoX, dsX, { kichThuocKhungMs: 1000, kichThuocLo: 2, thoiGianChoMs: 5000, gioiHanBuffer: 10 });
console.log(khoX.nhatKyGhi.cacLoDaGhi.length);
```

```typescript title=test
const tKho = taoKhoPipeline();
const tCauHinh: CauHinhPipeline = { kichThuocKhungMs: 1000, kichThuocLo: 2, thoiGianChoMs: 100000, gioiHanBuffer: 100 };
const tDs: SuKien[] = [
  { id: "t1", nguon: "e", thoiDiem: 0, soLuong: 1 },
  { id: "t2", nguon: "e", thoiDiem: 100, soLuong: 1 },
  { id: "t3", nguon: "e", thoiDiem: 200, soLuong: 1 },
  { id: "t4", nguon: "e", thoiDiem: 300, soLuong: 1 },
  { id: "t5", nguon: "e", thoiDiem: 400, soLuong: 1 },
];
const tKetQua = chayPipelineThat(tKho, tDs, tCauHinh);
const tSoLoQuyetDinh = tKetQua.cacLoCanGui.length;
const tSoLoDaGhi = tKho.nhatKyGhi.cacLoDaGhi.length;
if (tSoLoQuyetDinh !== 2) throw new Error("5 su kien, kichThuocLo=2, phai quyet dinh 2 lo (con 1 su kien le trong buffer)");
if (tSoLoDaGhi !== tSoLoQuyetDinh) throw new Error("so lan ghiRaNgoai duoc goi PHAI KHOP CHINH XAC voi so lo can gui");

const tKhoRong = taoKhoPipeline();
const tKetQuaRong = chayPipelineThat(tKhoRong, [], tCauHinh);
if (tKetQuaRong.cacLoCanGui.length !== 0) throw new Error("mang rong khong the tao lo nao");
const tSoLoRong = tKhoRong.nhatKyGhi.cacLoDaGhi.length;
if (tSoLoRong !== 0) throw new Error("mang su kien rong KHONG duoc goi ghiRaNgoai lan nao");

const tKho2 = taoKhoPipeline();
chayPipelineThat(tKho2, [{ id: "u1", nguon: "e", thoiDiem: 0, soLuong: 1 }], tCauHinh);
const tSoLoSauDot1 = tKho2.nhatKyGhi.cacLoDaGhi.length;
if (tSoLoSauDot1 !== 0) throw new Error("1 su kien don le (chua du kichThuocLo=2) khong duoc tao lo nao");

const tChuoiTruoc = JSON.stringify(tDs);
chayPipelineThat(taoKhoPipeline(), tDs, tCauHinh);
const tChuoiSau = JSON.stringify(tDs);
if (tChuoiTruoc !== tChuoiSau) throw new Error("chayPipelineThat KHONG duoc mutate cacSuKienMoi truyen vao");
```

:::hints
- kind: attention
  body: "Goi ketQua = xuLyPipeline(cacSuKienMoi, cauHinh). Dung for (const lo of ketQua.cacLoCanGui) { ghiRaNgoai(kho.nhatKyGhi, lo); }. Sau vong lap, return ketQua."
- kind: strategy
  body: "const ketQua = xuLyPipeline(cacSuKienMoi, cauHinh); for (const lo of ketQua.cacLoCanGui) { ghiRaNgoai(kho.nhatKyGhi, lo); } return ketQua;"
- kind: one-line
  body: "const ketQua = xuLyPipeline(cacSuKienMoi, cauHinh); for (const lo of ketQua.cacLoCanGui) { ghiRaNgoai(kho.nhatKyGhi, lo); } return ketQua;"
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lõi thuần (windowing, batching, backpressure) VÀ vỏ mệnh lệnh (effect
đúng số lần) đã ráp thành một pipeline hoàn chỉnh. Chín bài đã dựng đủ
mọi mảnh của quest này. BOSS giờ sẽ ráp NÓ cùng với hai quest TRƯỚC —
nhật ký bất biến VÀ CRDT — thành một luồng ingest hoàn chỉnh.
::::

::::reflect{#nghi-lai}
`chayPipelineThat` là điểm nối DUY NHẤT giữa "mọi thứ đã tính xong"
(lõi, bài 8) VÀ "mọi thứ THẬT SỰ xảy ra" (effect, bài 5) — nó không hề
thêm logic quyết định nào của riêng nó, chỉ dịch một danh sách kết quả
thuần thành một chuỗi lời gọi effect, ĐÚNG một lần cho mỗi phần tử.
Đây chính là hình dạng đầy đủ của "functional core, imperative shell"
Ở QUY MÔ một pipeline nhiều bước: toàn bộ trí tuệ (windowing, gom lô,
tín hiệu backpressure) nằm Ở lõi TEST được không cần mock; vỏ chỉ có
đúng một việc — biến kết quả đó thành hành động, không hơn không kém.
::::

::::checkpoint{mastery=0.87}
::::
