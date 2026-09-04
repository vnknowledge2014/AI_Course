---
id: thiet-ke-he-thong.streaming-va-hieu-ung-o-bien.pipeline-thuan-dau-cuoi
title: "Pipeline thuần đầu-cuối: windowing → batching → backpressure, một chuỗi hàm thuần"
summary: "xuLyPipeline(cacSuKien, cauHinh) ghep gomTheoKhung (bai 1) + quyetDinhGomLo (bai 2) + quyetDinhNhanSuKien (bai 3) thanh MOT chuoi bien doi thuan, tra ve { cacKhung, cacLoCanGui, tinHieuBackpressure } -- KHONG mot buoc nao side-effect; toan bo pipeline logic test duoc voi du lieu bia, khong can mock I/O nao, noi lai tinh than 'test khong can mock' cua quest 'Nhat ky bat bien va fold' bai 6."
locale: vi
track: thiet-ke-he-thong
module: streaming-va-hieu-ung-o-bien
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [sd.fp.pipeline-thuan-dau-cuoi]
requires: [sd.fp.crdt-trong-khung-thoi-gian]
concepts: [sd.fp.pipeline-thuan-dau-cuoi]
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
Bảy bài — windowing thuần, batching thuần, backpressure thuần, vỏ mệnh
lệnh, effect Ở biên, watermark, CRDT trong khung. Từng mảnh đã đứng
vững RIÊNG LẺ. Giờ ráp BA lõi thuần (windowing, batching, backpressure)
thành MỘT chuỗi biến đổi — không thêm logic mới, chỉ nối những gì đã
có.
::::

::::explain{#ghep-ba-loi-thanh-mot-chuoi}
`xuLyPipeline` nhận một mảng sự kiện VÀ một `CauHinhPipeline` (gộp bốn
tham số cấu hình đã xuất hiện riêng lẻ Ở các bài trước), rồi CHẠY ba
bước: `gomTheoKhung` (bài 1) tính `cacKhung`; `reduce` qua
`quyetDinhGomLo` (bài 2) tích luỹ `cacLoCanGui`; `quyetDinhNhanSuKien`
(bài 3) tính `tinHieuBackpressure` từ tổng số sự kiện. Không bước nào
gọi `console.log`, `ghiRaNgoai`, hay bất kỳ I/O nào:

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

const suKien = (id: string, thoiDiem: number, soLuong: number): SuKien => ({ id, nguon: "edge-1", thoiDiem, soLuong });
const cacSuKien: SuKien[] = [
  suKien("e1", 0, 5), suKien("e2", 1000, 5), suKien("e3", 2000, 5),
  suKien("e4", 11000, 5), suKien("e5", 12000, 5), suKien("e6", 21000, 5),
];

const cauHinh: CauHinhPipeline = { kichThuocKhungMs: 10000, kichThuocLo: 3, thoiGianChoMs: 100000, gioiHanBuffer: 100 };
const ketQua = xuLyPipeline(cacSuKien, cauHinh);

console.log("so khung:", ketQua.cacKhung.size);
console.log("khung 0:", JSON.stringify(ketQua.cacKhung.get(0)));
console.log("khung 1:", JSON.stringify(ketQua.cacKhung.get(1)));
console.log("khung 2:", JSON.stringify(ketQua.cacKhung.get(2)));
console.log("so lo can gui:", ketQua.cacLoCanGui.length);
console.log("lo dau tien:", JSON.stringify(ketQua.cacLoCanGui[0]?.map((sk) => sk.id)));
console.log("tin hieu backpressure (6 su kien / gioi han 100):", ketQua.tinHieuBackpressure);
console.log("cacSuKien goc van con", cacSuKien.length, "phan tu");
```

```text title=readonly
so khung: 3
khung 0: {"chiSoKhung":0,"tong":15,"soSuKien":3}
khung 1: {"chiSoKhung":1,"tong":10,"soSuKien":2}
khung 2: {"chiSoKhung":2,"tong":5,"soSuKien":1}
so lo can gui: 2
lo dau tien: ["e1","e2","e3"]
tin hieu backpressure (6 su kien / gioi han 100): nhan
cacSuKien goc van con 6 phan tu
```

`xuLyPipeline` chạy CẢ ba lõi trên ĐÚNG cùng `cacSuKien` — nhưng mỗi
lõi đọc `cacSuKien` theo CÁCH riêng của nó (`gomTheoKhung` nhìn
`thoiDiem`, `quyetDinhGomLo` nhìn thứ tự VÀ kích thước, `quyetDinhNhanSuKien`
chỉ nhìn TỔNG số lượng). Ba kết quả (`cacKhung`, `cacLoCanGui`,
`tinHieuBackpressure`) độc lập với nhau — không kết quả nào ảnh hưởng
tới cách tính của kết quả khác.
::::

::::example{#test-khong-can-mock-o-quy-mo-pipeline}
Vì `xuLyPipeline` chỉ nhận VÀ trả về dữ liệu thuần, kiểm tra TOÀN BỘ
pipeline không cần mở kết nối mạng hay mock một service nào — chỉ cần
dữ liệu bịa VÀ các cấu hình khác nhau:

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

// khong mo mot ket noi mang nao, khong mock mot service nao -- chi du lieu BIA
const duLieuBia: SuKien[] = [
  { id: "z1", nguon: "test", thoiDiem: 0, soLuong: 1 },
  { id: "z2", nguon: "test", thoiDiem: 500, soLuong: 1 },
  { id: "z3", nguon: "test", thoiDiem: 900, soLuong: 1 },
  { id: "z4", nguon: "test", thoiDiem: 950, soLuong: 1 },
  { id: "z5", nguon: "test", thoiDiem: 980, soLuong: 1 },
];

const kqBufferNho = xuLyPipeline(duLieuBia, { kichThuocKhungMs: 1000, kichThuocLo: 10, thoiGianChoMs: 100000, gioiHanBuffer: 5 });
console.log("gioiHanBuffer=5, 5 su kien -> tin hieu:", kqBufferNho.tinHieuBackpressure);

const kqBufferLon = xuLyPipeline(duLieuBia, { kichThuocKhungMs: 1000, kichThuocLo: 10, thoiGianChoMs: 100000, gioiHanBuffer: 500 });
console.log("gioiHanBuffer=500, 5 su kien -> tin hieu:", kqBufferLon.tinHieuBackpressure);

const lanGoi1 = xuLyPipeline(duLieuBia, { kichThuocKhungMs: 1000, kichThuocLo: 10, thoiGianChoMs: 100000, gioiHanBuffer: 5 });
const lanGoi2 = xuLyPipeline(duLieuBia, { kichThuocKhungMs: 1000, kichThuocLo: 10, thoiGianChoMs: 100000, gioiHanBuffer: 5 });
console.log("hai lan goi cung du lieu, cung tin hieu?", lanGoi1.tinHieuBackpressure === lanGoi2.tinHieuBackpressure);
console.log("hai lan goi cung so khung?", lanGoi1.cacKhung.size === lanGoi2.cacKhung.size);
```

```text title=readonly
gioiHanBuffer=5, 5 su kien -> tin hieu: chan_lai
gioiHanBuffer=500, 5 su kien -> tin hieu: nhan
hai lan goi cung du lieu, cung tin hieu? true
hai lan goi cung so khung? true
```

Không một bước nào Ở trên mở kết nối, chờ thời gian thực, hay cần dọn
dẹp sau khi test — mọi thứ chỉ LÀ lời gọi hàm trên dữ liệu bịa. Đổi
`gioiHanBuffer` từ `5` xuống `500` đổi HẲN tín hiệu (`chan_lai` →
`nhan`) trong khi `cacKhung` VÀ `cacLoCanGui` không hề bị ảnh hưởng —
ba trục cấu hình (`kichThuocKhungMs`, `kichThuocLo`/`thoiGianChoMs`,
`gioiHanBuffer`) hoàn toàn ĐỘC LẬP.
::::

::::predict{#doan-doi-kich-thuoc-lo-khong-anh-huong-khung commitOnce}
Tiếp tục với `duLieuBia` (5 sự kiện, `kichThuocKhungMs: 1000`). Gọi
`xuLyPipeline` hai lần: lần MỘT với `kichThuocLo: 3`, lần HAI với
`kichThuocLo: 9999` (không bao giờ đủ để gom lô), CẢ hai đều giữ
nguyên `kichThuocKhungMs: 1000` VÀ `gioiHanBuffer: 500`. `cacKhung.size`
Ở HAI kết quả này khác nhau HAY giống nhau?

:::opt{correct}
GIỐNG nhau — `cacKhung` được tính bằng `gomTheoKhung(cacSuKien,
cauHinh.kichThuocKhungMs)`, hoàn toàn KHÔNG phụ thuộc vào
`cauHinh.kichThuocLo`; đổi `kichThuocLo` chỉ làm `cacLoCanGui` đổi
(`1` lô so với `0` lô), không đụng tới `cacKhung`
:::
:::opt
KHÁC nhau — nếu không có lô nào được gom đủ (`kichThuocLo: 9999`),
pipeline coi như "chưa xử lý xong" VÀ không tính `cacKhung` cho lượt
đó, nên `cacKhung.size` sẽ LÀ `0`
::why
Nhầm "ba bước trong `xuLyPipeline`" LÀ một chuỗi TUẦN TỰ phụ thuộc lẫn
nhau (bước sau chỉ chạy nếu bước trước "thành công") — nhưng
`xuLyPipeline` gọi `gomTheoKhung` VÀ khối `reduce` tính `cacLoCanGui`
HOÀN TOÀN độc lập, trên CÙNG `cacSuKien` gốc, không truyền kết quả của
bước này làm đầu vào cho bước kia.

Chỗ lệch: `const cacKhung = gomTheoKhung(cacSuKien,
cauHinh.kichThuocKhungMs);` được tính trước, dùng ĐÚNG `cacSuKien` gốc
— dòng này không hề đọc `cauHinh.kichThuocLo`. Dù `kichThuocLo` lớn
tới đâu (khiến `cacLoCanGui` rỗng), `cacKhung` vẫn được tính đầy đủ
Y HỆT, vì nó là một PHÉP BIẾN ĐỔI riêng biệt trên dữ liệu đầu vào, không
phải một bước phụ thuộc kết quả của bước gom lô.
::
:::
::::

::::code{#viet_xu_ly_pipeline}
Hoàn thiện `xuLyPipeline` — tính `cacKhung` bằng `gomTheoKhung(cacSuKien,
cauHinh.kichThuocKhungMs)`. Tính `cacLoCanGui` bằng cách `reduce` qua
`cacSuKien`, gọi `quyetDinhGomLo` với `cauHinh.kichThuocLo` VÀ
`cauHinh.thoiGianChoMs`, tích luỹ mọi `loMoi` khác `undefined` vào một
mảng. Tính `tinHieuBackpressure` bằng `quyetDinhNhanSuKien(cacSuKien.length,
cauHinh.gioiHanBuffer)`. Trả về cả ba trong một object.

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
  ___
}

const dsX: SuKien[] = [
  { id: "x1", nguon: "e", thoiDiem: 0, soLuong: 2 },
  { id: "x2", nguon: "e", thoiDiem: 100, soLuong: 2 },
];
const kqX = xuLyPipeline(dsX, { kichThuocKhungMs: 1000, kichThuocLo: 2, thoiGianChoMs: 5000, gioiHanBuffer: 10 });
console.log(kqX.cacKhung.size, kqX.cacLoCanGui.length, kqX.tinHieuBackpressure);
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

const dsX: SuKien[] = [
  { id: "x1", nguon: "e", thoiDiem: 0, soLuong: 2 },
  { id: "x2", nguon: "e", thoiDiem: 100, soLuong: 2 },
];
const kqX = xuLyPipeline(dsX, { kichThuocKhungMs: 1000, kichThuocLo: 2, thoiGianChoMs: 5000, gioiHanBuffer: 10 });
console.log(kqX.cacKhung.size, kqX.cacLoCanGui.length, kqX.tinHieuBackpressure);
```

```typescript title=test
const tDs: SuKien[] = [
  { id: "t1", nguon: "e", thoiDiem: 0, soLuong: 3 },
  { id: "t2", nguon: "e", thoiDiem: 500, soLuong: 3 },
  { id: "t3", nguon: "e", thoiDiem: 1200, soLuong: 3 },
];
const tCauHinh: CauHinhPipeline = { kichThuocKhungMs: 1000, kichThuocLo: 2, thoiGianChoMs: 100000, gioiHanBuffer: 10 };
const tKq = xuLyPipeline(tDs, tCauHinh);

const tSoKhung = tKq.cacKhung.size;
if (tSoKhung !== 2) throw new Error("3 su kien (0,500 o khung 0; 1200 o khung 1) phai gom thanh 2 khung");
const tKhung0 = tKq.cacKhung.get(0);
if (tKhung0 === undefined || tKhung0.tong !== 6) throw new Error("khung 0 phai co tong = 3+3 = 6");

const tSoLo = tKq.cacLoCanGui.length;
if (tSoLo !== 1) throw new Error("kichThuocLo=2, 3 su kien phai gom duoc 1 lo (2 su kien dau)");
const tLoDauDoDai = tKq.cacLoCanGui[0]?.length;
if (tLoDauDoDai !== 2) throw new Error("lo dau tien phai co 2 su kien");

if (tKq.tinHieuBackpressure !== "nhan") throw new Error("3 su kien / gioiHanBuffer=10 (30%) phai la nhan");

const tCauHinhChanLai: CauHinhPipeline = { kichThuocKhungMs: 1000, kichThuocLo: 2, thoiGianChoMs: 100000, gioiHanBuffer: 3 };
const tKqChanLai = xuLyPipeline(tDs, tCauHinhChanLai);
if (tKqChanLai.tinHieuBackpressure !== "chan_lai") throw new Error("3 su kien / gioiHanBuffer=3 (100%) phai la chan_lai");

const tChuoiTruoc = JSON.stringify(tDs);
xuLyPipeline(tDs, tCauHinh);
const tChuoiSau = JSON.stringify(tDs);
if (tChuoiTruoc !== tChuoiSau) throw new Error("xuLyPipeline KHONG duoc mutate cacSuKien truyen vao");

const tKqLan1 = xuLyPipeline(tDs, tCauHinh);
const tKqLan2 = xuLyPipeline(tDs, tCauHinh);
if (tKqLan1.tinHieuBackpressure !== tKqLan2.tinHieuBackpressure) throw new Error("xuLyPipeline phai THUAN -- cung dau vao phai cho cung ket qua");
if (tKqLan1.cacLoCanGui.length !== tKqLan2.cacLoCanGui.length) throw new Error("xuLyPipeline phai THUAN -- so lo phai giong nhau giua hai lan goi");
```

:::hints
- kind: attention
  body: "Ba buoc DOC LAP: (1) cacKhung = gomTheoKhung(cacSuKien, cauHinh.kichThuocKhungMs); (2) dung cacSuKien.reduce voi quyetDinhGomLo(tich.buffer, sk, cauHinh.kichThuocLo, cauHinh.thoiGianChoMs) de tich luy cacLoCanGui, bat dau tu { buffer: [], cacLoCanGui: [] }; (3) tinHieuBackpressure = quyetDinhNhanSuKien(cacSuKien.length, cauHinh.gioiHanBuffer). Tra ve ca ba trong mot object."
- kind: strategy
  body: "const cacKhung = gomTheoKhung(cacSuKien, cauHinh.kichThuocKhungMs); const { cacLoCanGui } = cacSuKien.reduce((tich, sk) => { const kq = quyetDinhGomLo(tich.buffer, sk, cauHinh.kichThuocLo, cauHinh.thoiGianChoMs); if (kq.loMoi !== undefined) { return { buffer: kq.bufferMoi, cacLoCanGui: [...tich.cacLoCanGui, kq.loMoi] }; } return { buffer: kq.bufferMoi, cacLoCanGui: tich.cacLoCanGui }; }, { buffer: [] as SuKien[], cacLoCanGui: [] as SuKien[][] }); const tinHieuBackpressure = quyetDinhNhanSuKien(cacSuKien.length, cauHinh.gioiHanBuffer); return { cacKhung, cacLoCanGui, tinHieuBackpressure };"
- kind: one-line
  body: "const cacKhung = gomTheoKhung(cacSuKien, cauHinh.kichThuocKhungMs); const { cacLoCanGui } = cacSuKien.reduce((tich, sk) => { const kq = quyetDinhGomLo(tich.buffer, sk, cauHinh.kichThuocLo, cauHinh.thoiGianChoMs); return kq.loMoi !== undefined ? { buffer: kq.bufferMoi, cacLoCanGui: [...tich.cacLoCanGui, kq.loMoi] } : { buffer: kq.bufferMoi, cacLoCanGui: tich.cacLoCanGui }; }, { buffer: [] as SuKien[], cacLoCanGui: [] as SuKien[][] }); return { cacKhung, cacLoCanGui, tinHieuBackpressure: quyetDinhNhanSuKien(cacSuKien.length, cauHinh.gioiHanBuffer) };"
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "1 1 nhan"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba lõi ghép thành một chuỗi biến đổi thuần — kiểm tra được toàn bộ
logic bằng dữ liệu bịa, không cần I/O nào. Nhưng đây vẫn chỉ LÀ tính
toán trên giấy — chưa có gì THẬT SỰ rời khỏi hệ thống.
::::

::::reflect{#nghi-lai}
`xuLyPipeline` không hề viết lại `gomTheoKhung`, `quyetDinhGomLo`, hay
`quyetDinhNhanSuKien` — nó chỉ GỌI cả ba, trên cùng một đầu vào, VÀ
gói ba kết quả độc lập vào một object. Đây chính LÀ giá trị của việc
xây từng lõi thuần TRƯỚC khi ráp: khi ráp xong, không có "tương tác
ẩn" nào giữa ba phần cần lo lắng — mỗi phần vẫn hoạt động ĐÚNG như khi
được test riêng lẻ, vì không phần nào có trạng thái ẩn để phần khác vô
tình chạm vào.
::::

::::checkpoint{mastery=0.85}
::::
