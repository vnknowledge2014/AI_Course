---
id: co-so-du-lieu.ke-pha-hoai-co-chu-dich.chinh-sach-tiem-loi
title: "Chính sách tiêm lỗi"
summary: "ChinhSachLoi goi DUY NHAT mot tt -- apDungLoiDiaTheoChinhSach (bai 2) VA coMatGoiTheoChinhSach (bai 6) deu tieu thu TU CHINH tt do, khong phai hai tt rieng. chayMotBuoc goi ca hai theo THU TU cho MOI 'buoc' (mot thao tac dia + mot goi mang). seed=2n chay 4 buoc cho DUNG chuoi ket qua ca dia LAN mang, lap lai KHOP tuyet doi -- MOT con so quyet dinh TOAN BO lich su co ca hai loai su co, khong phai hai con so rieng."
locale: vi
track: co-so-du-lieu
module: ke-pha-hoai-co-chu-dich
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [db.chinh-sach-tiem-loi]
requires: [db.mang-lam-tre-va-dao-thu-tu]
concepts: [db.chinh-sach-tiem-loi]
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
Năm lỗi đĩa/crash (bài 2-5) VÀ hai lỗi mạng (bài 6-7) — mỗi bài dùng
MỘT `tt` riêng. Một hệ thống thật có CẢ đĩa lẫn mạng cùng lúc — chúng
có cần seed riêng không?
::::

::::explain{#mot-seed-ca-hai}
`ChinhSachLoi` chỉ giữ ĐÚNG một `tt` duy nhất. `apDungLoiDiaTheoChinhSach`
(gọi lại logic bài 2) VÀ `coMatGoiTheoChinhSach` (gọi lại logic bài 6)
đều tiêu thụ TỪ CHÍNH `tt` đó — không phải hai trạng thái tách biệt.
`chayMotBuoc` gọi CẢ hai theo đúng THỨ TỰ cho mỗi "bước" mô phỏng:

```typescript title=readonly
const MASK64 = (1n << 64n) - 1n;
interface TrangThai { s0: bigint; s1: bigint; }
function gieoHat(seed: bigint): TrangThai {
  let s0 = seed & MASK64;
  if (s0 === 0n) s0 = 0x9e3779b97f4a7c15n;
  let s1 = (seed * 6364136223846793005n + 1442695040888963407n) & MASK64;
  if (s1 === 0n) s1 = 0xbf58476d1ce4e5b9n;
  return { s0, s1 };
}
function soTiepTheo(tt: TrangThai): bigint {
  let s1 = tt.s0;
  const s0 = tt.s1;
  const ketQua = (s1 + s0) & MASK64;
  tt.s0 = s0;
  s1 ^= (s1 << 23n) & MASK64;
  s1 ^= s1 >> 17n;
  s1 ^= s0 ^ (s0 >> 26n);
  tt.s1 = s1 & MASK64;
  return ketQua;
}
function soNguyenTrongKhoang(tt: TrangThai, min: number, max: number): number {
  return min + Number(soTiepTheo(tt) % BigInt(max - min));
}

class SimDisk {
  readonly kichThuocSector: number;
  private readonly platter = new Map<number, Uint8Array>();
  private readonly cache = new Map<number, Uint8Array>();
  private matFsyncKeTiep = false;
  private readonly tornSector = new Map<number, number>();
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  write(sector: number, data: Uint8Array): void { this.cache.set(sector, data.slice()); }
  fsync(): void {
    if (this.matFsyncKeTiep) { this.matFsyncKeTiep = false; return; }
    for (const [sector, data] of this.cache) {
      let ghiDuoc = data;
      const n = this.tornSector.get(sector);
      if (n !== undefined) {
        this.tornSector.delete(sector);
        const cu = this.platter.get(sector) ?? new Uint8Array(this.kichThuocSector);
        const ket = new Uint8Array(this.kichThuocSector);
        ket.set(data.slice(0, n), 0);
        ket.set(cu.slice(n), n);
        ghiDuoc = ket;
      }
      this.platter.set(sector, ghiDuoc);
    }
    this.cache.clear();
  }
  boQuaFsyncKeTiep(): void { this.matFsyncKeTiep = true; }
  danhDauTornGhi(sector: number, n: number): void { this.tornSector.set(sector, n); }
}

interface ChinhSachLoi { tt: TrangThai; }
function taoChinhSachLoi(seed: bigint): ChinhSachLoi { return { tt: gieoHat(seed) }; }
function apDungLoiDiaTheoChinhSach(cs: ChinhSachLoi, disk: SimDisk, sector: number): number {
  const kieu = soNguyenTrongKhoang(cs.tt, 0, 3); // 0=khong loi,1=lost fsync,2=torn write
  if (kieu === 1) disk.boQuaFsyncKeTiep();
  else if (kieu === 2) disk.danhDauTornGhi(sector, Math.floor(disk.kichThuocSector / 2));
  return kieu;
}
function coMatGoiTheoChinhSach(cs: ChinhSachLoi, tyLeMatPhanTram: number): boolean {
  return soNguyenTrongKhoang(cs.tt, 0, 100) < tyLeMatPhanTram;
}
function chayMotBuoc(cs: ChinhSachLoi, disk: SimDisk, sector: number, tyLeMatGoi: number): { kieuLoiDia: number; matGoi: boolean } {
  const kieuLoiDia = apDungLoiDiaTheoChinhSach(cs, disk, sector);
  const matGoi = coMatGoiTheoChinhSach(cs, tyLeMatGoi);
  return { kieuLoiDia, matGoi };
}

function chayKichBan(seed: bigint): { kieuLoiDia: number; matGoi: boolean }[] {
  const cs = taoChinhSachLoi(seed);
  const disk = new SimDisk(4);
  const ketQua = [];
  for (let i = 0; i < 4; i++) ketQua.push(chayMotBuoc(cs, disk, i % 2, 30));
  return ketQua;
}

console.log("seed=2n:", JSON.stringify(chayKichBan(2n)));
console.log("seed=2n lap lai:", JSON.stringify(chayKichBan(2n)));
```

```text title=readonly
seed=2n: [{"kieuLoiDia":0,"matGoi":false},{"kieuLoiDia":2,"matGoi":true},{"kieuLoiDia":0,"matGoi":false},{"kieuLoiDia":2,"matGoi":false}]
seed=2n lap lai: [{"kieuLoiDia":0,"matGoi":false},{"kieuLoiDia":2,"matGoi":true},{"kieuLoiDia":0,"matGoi":false},{"kieuLoiDia":2,"matGoi":false}]
```

Bốn "bước", mỗi bước MỘT quyết định đĩa VÀ một quyết định mạng — TÁM
quyết định tổng CỘNG, tất cả rút RA từ đúng MỘT `tt` ban đầu (`gieoHat
(2n)`). Không phải "seed đĩa" VÀ "seed mạng" tách biệt — CHỈ một con
số duy nhất quyết định TOÀN bộ lịch sử sự cố, thuộc CẢ hai lớp hệ
thống. Gọi lại `chayKichBan(2n)` cho ĐÚNG lại từng chữ số Ở TRÊN.
::::

::::example{#vi-sao-mot-seed-duy-nhat}
Nếu đĩa VÀ mạng dùng seed RIÊNG, "tái hiện một bug" nghĩa LÀ phải ghi
lại HAI con số VÀ đảm bảo chúng phối hợp đúng CÁCH mỗi lần chạy lại.
Dồn về MỘT `tt` xoá bỏ hoàn toàn rủi ro đó: một con số, một lịch SỬ sự
cố đầy đủ, tái hiện tuyệt đối — đúng tinh THẦN "vũ trụ tất định" (q17)
áp dụng cho CẢ một hệ thống nhiều thành phần.
::::

::::predict{#doan-doi-thu-tu-goi commitOnce}
Nếu `chayMotBuoc` đổi THỨ TỰ — gọi `coMatGoiTheoChinhSach` TRƯỚC,
`apDungLoiDiaTheoChinhSach` SAU — với CÙNG seed=2n, kết quả bốn bước
CÓ còn giống hệt Ở TRÊN không?
:::opt{correct}
KHÔNG chắc giống — `tt` bị tiêu thụ theo ĐÚNG thứ tự gọi, đổi thứ tự
gọi làm HAI hàm đọc từ hai "chỗ" khác nhau trong chuỗi số sinh RA, kết
quả CÓ thể khác hoàn toàn dù CÙNG seed
:::
:::opt
CÓ, vẫn giống hệt — cả hai hàm ĐỘC lập với nhau, thứ tự gọi không ảnh
hưởng gì tới kết QUẢ cuối
::why
Trực giác NÀY nhầm "hai hàm không gọi LẪN nhau" VỚI "hai hàm không
chia sẻ TRẠNG THÁI" — nhưng CẢ hai cùng đọc VÀ ghi đè lên đúng MỘT
`cs.tt`.

Chỗ lệch: MỖI lần `soNguyenTrongKhoang` được gọi (bên trong CẢ hai
hàm) đều tiêu thụ "số tiếp theo" trong CHUỖI sinh từ `tt`, và làm `tt`
tiến thêm MỘT bước. Đổi thứ tự gọi hai hàm đổi LUÔN thứ tự tiêu thụ
chuỗi số ĐÓ — y hệt đổi thứ tự đọc một cuốn sách sẽ đổi câu chuyện bạn
hiểu, dù VẪN là đúng những trang sách đó.
::
:::
::::

::::code{#viet_chay_mot_buoc}
Hoàn thiện `chayMotBuoc` — gọi `apDungLoiDiaTheoChinhSach` TRƯỚC lấy
`kieuLoiDia`, RỒI `coMatGoiTheoChinhSach` lấy `matGoi`, trả về CẢ hai.

```typescript title=starter
const MASK64 = (1n << 64n) - 1n;
interface TrangThai { s0: bigint; s1: bigint; }
function gieoHat(seed: bigint): TrangThai {
  let s0 = seed & MASK64;
  if (s0 === 0n) s0 = 0x9e3779b97f4a7c15n;
  let s1 = (seed * 6364136223846793005n + 1442695040888963407n) & MASK64;
  if (s1 === 0n) s1 = 0xbf58476d1ce4e5b9n;
  return { s0, s1 };
}
function soTiepTheo(tt: TrangThai): bigint {
  let s1 = tt.s0;
  const s0 = tt.s1;
  const ketQua = (s1 + s0) & MASK64;
  tt.s0 = s0;
  s1 ^= (s1 << 23n) & MASK64;
  s1 ^= s1 >> 17n;
  s1 ^= s0 ^ (s0 >> 26n);
  tt.s1 = s1 & MASK64;
  return ketQua;
}
function soNguyenTrongKhoang(tt: TrangThai, min: number, max: number): number {
  return min + Number(soTiepTheo(tt) % BigInt(max - min));
}

class SimDisk {
  readonly kichThuocSector: number;
  private readonly platter = new Map<number, Uint8Array>();
  private readonly cache = new Map<number, Uint8Array>();
  private matFsyncKeTiep = false;
  private readonly tornSector = new Map<number, number>();
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  write(sector: number, data: Uint8Array): void { this.cache.set(sector, data.slice()); }
  fsync(): void {
    if (this.matFsyncKeTiep) { this.matFsyncKeTiep = false; return; }
    for (const [sector, data] of this.cache) {
      let ghiDuoc = data;
      const n = this.tornSector.get(sector);
      if (n !== undefined) {
        this.tornSector.delete(sector);
        const cu = this.platter.get(sector) ?? new Uint8Array(this.kichThuocSector);
        const ket = new Uint8Array(this.kichThuocSector);
        ket.set(data.slice(0, n), 0);
        ket.set(cu.slice(n), n);
        ghiDuoc = ket;
      }
      this.platter.set(sector, ghiDuoc);
    }
    this.cache.clear();
  }
  boQuaFsyncKeTiep(): void { this.matFsyncKeTiep = true; }
  danhDauTornGhi(sector: number, n: number): void { this.tornSector.set(sector, n); }
}

interface ChinhSachLoi { tt: TrangThai; }
function taoChinhSachLoi(seed: bigint): ChinhSachLoi { return { tt: gieoHat(seed) }; }
function apDungLoiDiaTheoChinhSach(cs: ChinhSachLoi, disk: SimDisk, sector: number): number {
  const kieu = soNguyenTrongKhoang(cs.tt, 0, 3);
  if (kieu === 1) disk.boQuaFsyncKeTiep();
  else if (kieu === 2) disk.danhDauTornGhi(sector, Math.floor(disk.kichThuocSector / 2));
  return kieu;
}
function coMatGoiTheoChinhSach(cs: ChinhSachLoi, tyLeMatPhanTram: number): boolean {
  return soNguyenTrongKhoang(cs.tt, 0, 100) < tyLeMatPhanTram;
}
function chayMotBuoc(cs: ChinhSachLoi, disk: SimDisk, sector: number, tyLeMatGoi: number): { kieuLoiDia: number; matGoi: boolean } {
  ___
}

console.log(JSON.stringify(chayMotBuoc(taoChinhSachLoi(2n), new SimDisk(4), 0, 30)));
```

```typescript title=solution
const MASK64 = (1n << 64n) - 1n;
interface TrangThai { s0: bigint; s1: bigint; }
function gieoHat(seed: bigint): TrangThai {
  let s0 = seed & MASK64;
  if (s0 === 0n) s0 = 0x9e3779b97f4a7c15n;
  let s1 = (seed * 6364136223846793005n + 1442695040888963407n) & MASK64;
  if (s1 === 0n) s1 = 0xbf58476d1ce4e5b9n;
  return { s0, s1 };
}
function soTiepTheo(tt: TrangThai): bigint {
  let s1 = tt.s0;
  const s0 = tt.s1;
  const ketQua = (s1 + s0) & MASK64;
  tt.s0 = s0;
  s1 ^= (s1 << 23n) & MASK64;
  s1 ^= s1 >> 17n;
  s1 ^= s0 ^ (s0 >> 26n);
  tt.s1 = s1 & MASK64;
  return ketQua;
}
function soNguyenTrongKhoang(tt: TrangThai, min: number, max: number): number {
  return min + Number(soTiepTheo(tt) % BigInt(max - min));
}

class SimDisk {
  readonly kichThuocSector: number;
  private readonly platter = new Map<number, Uint8Array>();
  private readonly cache = new Map<number, Uint8Array>();
  private matFsyncKeTiep = false;
  private readonly tornSector = new Map<number, number>();
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  write(sector: number, data: Uint8Array): void { this.cache.set(sector, data.slice()); }
  fsync(): void {
    if (this.matFsyncKeTiep) { this.matFsyncKeTiep = false; return; }
    for (const [sector, data] of this.cache) {
      let ghiDuoc = data;
      const n = this.tornSector.get(sector);
      if (n !== undefined) {
        this.tornSector.delete(sector);
        const cu = this.platter.get(sector) ?? new Uint8Array(this.kichThuocSector);
        const ket = new Uint8Array(this.kichThuocSector);
        ket.set(data.slice(0, n), 0);
        ket.set(cu.slice(n), n);
        ghiDuoc = ket;
      }
      this.platter.set(sector, ghiDuoc);
    }
    this.cache.clear();
  }
  boQuaFsyncKeTiep(): void { this.matFsyncKeTiep = true; }
  danhDauTornGhi(sector: number, n: number): void { this.tornSector.set(sector, n); }
}

interface ChinhSachLoi { tt: TrangThai; }
function taoChinhSachLoi(seed: bigint): ChinhSachLoi { return { tt: gieoHat(seed) }; }
function apDungLoiDiaTheoChinhSach(cs: ChinhSachLoi, disk: SimDisk, sector: number): number {
  const kieu = soNguyenTrongKhoang(cs.tt, 0, 3);
  if (kieu === 1) disk.boQuaFsyncKeTiep();
  else if (kieu === 2) disk.danhDauTornGhi(sector, Math.floor(disk.kichThuocSector / 2));
  return kieu;
}
function coMatGoiTheoChinhSach(cs: ChinhSachLoi, tyLeMatPhanTram: number): boolean {
  return soNguyenTrongKhoang(cs.tt, 0, 100) < tyLeMatPhanTram;
}
function chayMotBuoc(cs: ChinhSachLoi, disk: SimDisk, sector: number, tyLeMatGoi: number): { kieuLoiDia: number; matGoi: boolean } {
  const kieuLoiDia = apDungLoiDiaTheoChinhSach(cs, disk, sector);
  const matGoi = coMatGoiTheoChinhSach(cs, tyLeMatGoi);
  return { kieuLoiDia, matGoi };
}

console.log(JSON.stringify(chayMotBuoc(taoChinhSachLoi(2n), new SimDisk(4), 0, 30)));
```

```typescript title=test
function chayKichBanT(seed: bigint): { kieuLoiDia: number; matGoi: boolean }[] {
  const cs = taoChinhSachLoi(seed);
  const disk = new SimDisk(4);
  const ketQua = [];
  for (let i = 0; i < 4; i++) ketQua.push(chayMotBuoc(cs, disk, i % 2, 30));
  return ketQua;
}

const KQ_MONG_DOI = '[{"kieuLoiDia":0,"matGoi":false},{"kieuLoiDia":2,"matGoi":true},{"kieuLoiDia":0,"matGoi":false},{"kieuLoiDia":2,"matGoi":false}]';
const kq2 = chayKichBanT(2n);
if (JSON.stringify(kq2) !== KQ_MONG_DOI) throw new Error("seed=2n, 4 buoc phai cho DUNG " + KQ_MONG_DOI);

const kq2Lai = chayKichBanT(2n);
if (JSON.stringify(kq2Lai) !== JSON.stringify(kq2)) throw new Error("goi lai CUNG seed=2n phai cho DUNG cung ket qua (dia LAN mang)");

const kq3 = chayKichBanT(3n);
if (JSON.stringify(kq3) === JSON.stringify(kq2)) throw new Error("seed KHAC (3n) phai cho ket qua KHAC");

const mot = chayMotBuoc(taoChinhSachLoi(2n), new SimDisk(4), 0, 30);
if (typeof mot.matGoi !== "boolean") throw new Error("matGoi phai la boolean");
if (typeof mot.kieuLoiDia !== "number") throw new Error("kieuLoiDia phai la number");
if (JSON.stringify(mot) !== '{"kieuLoiDia":0,"matGoi":false}') throw new Error("buoc dau tien seed=2n phai la {kieuLoiDia:0,matGoi:false}");
```

:::hints
- kind: attention
  body: "Goi apDungLoiDiaTheoChinhSach TRUOC lay kieuLoiDia, roi coMatGoiTheoChinhSach lay matGoi, tra ve { kieuLoiDia, matGoi } -- ba dong."
- kind: strategy
  body: "const kieuLoiDia = apDungLoiDiaTheoChinhSach(cs, disk, sector); const matGoi = coMatGoiTheoChinhSach(cs, tyLeMatGoi); return { kieuLoiDia, matGoi };"
- kind: one-line
  body: "const kieuLoiDia = apDungLoiDiaTheoChinhSach(cs, disk, sector); const matGoi = coMatGoiTheoChinhSach(cs, tyLeMatGoi); return { kieuLoiDia, matGoi };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "kieuLoiDia"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một seed, một chính sách, toàn bộ lịch sử sự cố cả đĩa lẫn mạng. Giờ
dùng chính nó để làm điều thật sự: TÌM một seed làm sập một hệ thống
có thật.
::::

::::reflect{#nghi-lai}
`ChinhSachLoi` không thêm phép toán MỚI — nó chỉ đóng gói lại đúng một
`tt` DUY NHẤT thay vì để mỗi lớp hệ thống (đĩa, mạng) tự quản một
trạng thái riêng. Bài học lớn nhất q18 tới giờ: một khi ĐÃ có một
nguồn tất định DUY NHẤT, việc thêm bao nhiêu LOẠI sự cố (đĩa, mạng,
sau này LÀ tiến trình — q19) không hề làm mất tính "chạy lại đúng
tuyệt đối" — nó chỉ LÀ đọc thêm vài số nữa từ CÙNG một chuỗi.
::::

::::checkpoint{mastery=0.9}
::::
