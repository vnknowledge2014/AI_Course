---
id: co-so-du-lieu.byteledger-capstone.chinh-sach-loi-toan-cum
title: "Chính sách lỗi cho toàn cụm"
summary: "ChinhSachLoi (q18 bài 8) giữ ĐÚNG MỘT tt cho CẢ cụm N=3 -- xuLyPrepareVoiChinhSachLoiDayDu quyết định MỖI backup có nhận Prepare không (coMatGoiTheoChinhSach) RỒI, nếu có, backup đó ghi xuống SimDisk riêng dưới MỘT lỗi đĩa vừa rút (apDungLoiDiaTheoChinhSach) -- cả hai quyết định cùng tiêu thụ CHUNG một tt. seed=3n: backup1(chiSo=1) MẤT Prepare hoàn toàn (kieuLoiDia=null), backup2(chiSo=2) nhận được và ghi sạch (kieuLoiDia=0) -- lặp lại y hệt qua nhiều lần chạy. seed=0n: CẢ hai backup đều nhận Prepare, backup1 ghi sạch (kieuLoiDia=0), backup2 dính lost fsync (kieuLoiDia=1)."
locale: vi
track: co-so-du-lieu
module: byteledger-capstone
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.chinh-sach-loi-toan-cum]
requires: [db.tiem-loi-dia-cho-tung-replica]
concepts: [db.chinh-sach-loi-toan-cum]
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
Mỗi replica giờ tự ghi log xuống đĩa RIÊNG (bài trước) — nhưng thử
nghiệm chỉ ĐỤNG tới MỘT đĩa, chưa hề chạm mạng. q18 bài 8 đã gộp lỗi
đĩa VÀ mạng dưới đúng MỘT seed. Áp dụng ĐIỀU đó cho CẢ cụm N=3.
::::

::::explain{#mot-chinh-sach-ca-cum}
`ChinhSachLoi` (q18 bài 8) giữ ĐÚNG một `tt` — nhưng lần NÀY, `tt` đó
phục vụ CẢ cụm, không CHỈ một cặp đĩa/mạng đơn LẺ. `xuLyPrepareVoiChinhSachLoiDayDu`
quyết định MỖI backup CÓ nhận được Prepare không
(`coMatGoiTheoChinhSach`) RỒI, NẾU có, backup đó ghi xuống `SimDisk`
RIÊNG của nó dưới MỘT lỗi đĩa vừa rút (`apDungLoiDiaTheoChinhSach`) —
CẢ hai quyết định (mạng CHO backup NÀY, đĩa CHO backup ĐÓ) đều tiêu
thụ CHUNG một `tt`:

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
  read(sector: number): Uint8Array {
    const o = this.cache.get(sector) ?? this.platter.get(sector);
    return o ? o.slice() : new Uint8Array(this.kichThuocSector);
  }
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

interface KetQuaMotBackup { chiSo: number; prepareToiNoi: boolean; kieuLoiDia: number | null; }
function xuLyPrepareVoiChinhSachLoiDayDu(cs: ChinhSachLoi, chiSoBackup: number, disk: SimDisk, tyLeMatGoi: number): KetQuaMotBackup {
  const prepareToiNoi = !coMatGoiTheoChinhSach(cs, tyLeMatGoi);
  if (!prepareToiNoi) return { chiSo: chiSoBackup, prepareToiNoi: false, kieuLoiDia: null };
  const kieuLoiDia = apDungLoiDiaTheoChinhSach(cs, disk, 0);
  disk.write(0, new Uint8Array([9, 9, 9, 9]));
  disk.fsync();
  return { chiSo: chiSoBackup, prepareToiNoi: true, kieuLoiDia };
}

function chayChoCumN3(seed: bigint, tyLeMatGoi: number): KetQuaMotBackup[] {
  const cs = taoChinhSachLoi(seed);
  const ketQua: KetQuaMotBackup[] = [];
  for (const chiSoBackup of [1, 2]) {
    const disk = new SimDisk(4);
    ketQua.push(xuLyPrepareVoiChinhSachLoiDayDu(cs, chiSoBackup, disk, tyLeMatGoi));
  }
  return ketQua;
}

console.log("seed=3n:", JSON.stringify(chayChoCumN3(3n, 30)));
console.log("seed=3n (lap lai):", JSON.stringify(chayChoCumN3(3n, 30)));
console.log("seed=0n:", JSON.stringify(chayChoCumN3(0n, 30)));
```

```text title=readonly
seed=3n: [{"chiSo":1,"prepareToiNoi":false,"kieuLoiDia":null},{"chiSo":2,"prepareToiNoi":true,"kieuLoiDia":0}]
seed=3n (lap lai): [{"chiSo":1,"prepareToiNoi":false,"kieuLoiDia":null},{"chiSo":2,"prepareToiNoi":true,"kieuLoiDia":0}]
seed=0n: [{"chiSo":1,"prepareToiNoi":true,"kieuLoiDia":0},{"chiSo":2,"prepareToiNoi":true,"kieuLoiDia":1}]
```

`seed=3n`: `backup1` (`chiSo=1`) MẤT Prepare hoàn TOÀN — `coMatGoiTheoChinhSach`
tiêu thụ `tt` VÀ trả `true` (rớt), `kieuLoiDia` LÀ `null` (chưa từng
chạm đĩa, VÌ không có gì để GHI). `backup2` NHẬN được VÀ ghi sạch
(`kieuLoiDia=0`). Gọi LẠI `chayChoCumN3(3n, 30)` cho ĐÚNG lại kết quả
đó — MỘT `tt` phục vụ CẢ hai quyết định, cho CẢ hai backup, tái hiện
tuyệt đối. `seed=0n` kể một câu chuyện KHÁC hẳn: CẢ hai backup đều
nhận Prepare, nhưng `backup2` dính LỖI đĩa (`kieuLoiDia=1`, lost
fsync) NGAY trên chính lượt ghi CỦA nó.
::::

::::example{#mot-tt-hai-vai-tro}
Đây LÀ đúng bài học q18 bài 8 ("một chính sách, MỘT seed") ÁP dụng
CHO nhiều đối tượng thay VÌ một cặp đĩa/mạng đơn: `tt` được TIÊU thụ
theo đúng thứ TỰ vòng `for` — `backup1` LUÔN "hỏi" `tt` trước
`backup2` (y hệt cảnh báo q19 bài 4: đổi thứ tự mảng `[1,2]` thành
`[2,1]` sẽ đổi LUÔN ai nhận quyết định NÀO). MỘT con số DUY nhất giờ
quyết định TOÀN bộ lịch sử sự cố của CẢ cụm — không phải mỗi backup
một seed riêng.
::::

::::predict{#doan-ty-le-mat-100-phan-tram commitOnce}
Gọi `chayChoCumN3(seedBatKy, 100)` (tỉ lệ MẤT gói LÀ `100%`, MỌI
Prepare đều rớt). Với backup THỨ hai (`chiSo=2`), `kieuLoiDia` trong
kết QUẢ trả về LÀ giá trị NÀO?
:::opt{correct}
`null` — `prepareToiNoi` LÀ `false` (không backup NÀO nhận được gì Ở
tỉ lệ `100%`), hàm trả VỀ NGAY tại dòng `if (!prepareToiNoi) return
{...kieuLoiDia: null}` — KHÔNG BAO giờ chạm tới
`apDungLoiDiaTheoChinhSach`, VÌ không có GÌ để ghi xuống đĩa cả
:::
:::opt
`0`, `1`, hoặc `2` — dù mất Prepare, `SimDisk` của backup đó VẪN tồn
tại VÀ vẫn có thể "bị lỗi" một cách trừu tượng
::why
Trực giác NÀY tách RỜI "SimDisk tồn tại" VỚI "có thao TÁC ghi nào xảy
ra" — nhưng `apDungLoiDiaTheoChinhSach` chỉ CÓ Ý nghĩa NGAY TRƯỚC một
lần `write`/`fsync` THẬT SỰ, không phải một thuộc TÍNH cố hữu của
đĩa.

Chỗ lệch: hàm `if (!prepareToiNoi) return { ..., kieuLoiDia: null };`
THOÁT khỏi hàm NGAY khi Prepare không tới nơi — dòng gọi
`apDungLoiDiaTheoChinhSach` (VÀ MỌI thao tác ghi đĩa SAU đó) không
BAO giờ được thực thi. Một backup CHƯA từng nhận được gì thì KHÔNG
có "lỗi ghi" NÀO để nói tới — `null` phản ánh ĐÚNG "câu hỏi không áp
dụng được", KHÔNG phải "0 loại lỗi".
::
:::
::::

::::code{#viet_xu_ly_prepare_voi_chinh_sach}
Hoàn thiện `xuLyPrepareVoiChinhSachLoiDayDu` — SAU khi xác nhận
Prepare tới nơi, rút MỘT quyết định lỗi đĩa TỪ đúng `cs` (CÙNG `tt`
với quyết định mạng Ở trên) cho sector `0` của `disk`.

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
  read(sector: number): Uint8Array {
    const o = this.cache.get(sector) ?? this.platter.get(sector);
    return o ? o.slice() : new Uint8Array(this.kichThuocSector);
  }
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

interface KetQuaMotBackup { chiSo: number; prepareToiNoi: boolean; kieuLoiDia: number | null; }
function xuLyPrepareVoiChinhSachLoiDayDu(cs: ChinhSachLoi, chiSoBackup: number, disk: SimDisk, tyLeMatGoi: number): KetQuaMotBackup {
  const prepareToiNoi = !coMatGoiTheoChinhSach(cs, tyLeMatGoi);
  if (!prepareToiNoi) return { chiSo: chiSoBackup, prepareToiNoi: false, kieuLoiDia: null };
  ___
  disk.write(0, new Uint8Array([9, 9, 9, 9]));
  disk.fsync();
  return { chiSo: chiSoBackup, prepareToiNoi: true, kieuLoiDia };
}

function chayChoCumN3(seed: bigint, tyLeMatGoi: number): KetQuaMotBackup[] {
  const cs = taoChinhSachLoi(seed);
  const ketQua: KetQuaMotBackup[] = [];
  for (const chiSoBackup of [1, 2]) {
    const disk = new SimDisk(4);
    ketQua.push(xuLyPrepareVoiChinhSachLoiDayDu(cs, chiSoBackup, disk, tyLeMatGoi));
  }
  return ketQua;
}

console.log(JSON.stringify(chayChoCumN3(3n, 30)));
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
  read(sector: number): Uint8Array {
    const o = this.cache.get(sector) ?? this.platter.get(sector);
    return o ? o.slice() : new Uint8Array(this.kichThuocSector);
  }
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

interface KetQuaMotBackup { chiSo: number; prepareToiNoi: boolean; kieuLoiDia: number | null; }
function xuLyPrepareVoiChinhSachLoiDayDu(cs: ChinhSachLoi, chiSoBackup: number, disk: SimDisk, tyLeMatGoi: number): KetQuaMotBackup {
  const prepareToiNoi = !coMatGoiTheoChinhSach(cs, tyLeMatGoi);
  if (!prepareToiNoi) return { chiSo: chiSoBackup, prepareToiNoi: false, kieuLoiDia: null };
  const kieuLoiDia = apDungLoiDiaTheoChinhSach(cs, disk, 0);
  disk.write(0, new Uint8Array([9, 9, 9, 9]));
  disk.fsync();
  return { chiSo: chiSoBackup, prepareToiNoi: true, kieuLoiDia };
}

function chayChoCumN3(seed: bigint, tyLeMatGoi: number): KetQuaMotBackup[] {
  const cs = taoChinhSachLoi(seed);
  const ketQua: KetQuaMotBackup[] = [];
  for (const chiSoBackup of [1, 2]) {
    const disk = new SimDisk(4);
    ketQua.push(xuLyPrepareVoiChinhSachLoiDayDu(cs, chiSoBackup, disk, tyLeMatGoi));
  }
  return ketQua;
}

console.log(JSON.stringify(chayChoCumN3(3n, 30)));
```

```typescript title=test
const lan1 = chayChoCumN3(3n, 30);
const lan2 = chayChoCumN3(3n, 30);
if (JSON.stringify(lan1) !== JSON.stringify(lan2)) throw new Error("goi lai CUNG seed=3n phai cho DUNG cung ket qua (dia LAN mang) ca cum");
if (lan1.length !== 2) throw new Error("cum N=3 co dung 2 backup (chiSo 1 va 2)");
if (lan1[0]!.chiSo !== 1 || lan1[1]!.chiSo !== 2) throw new Error("thu tu ket qua phai theo dung thu tu backup [1,2]");
if (lan1[0]!.prepareToiNoi !== false) throw new Error("seed=3n, backup dau tien (chiSo=1) phai MAT Prepare");
if (lan1[0]!.kieuLoiDia !== null) throw new Error("backup KHONG nhan Prepare thi kieuLoiDia phai la null");
if (lan1[1]!.kieuLoiDia !== 0) throw new Error("seed=3n, backup thu hai (chiSo=2) phai co kieuLoiDia=0");

const seedKhac = chayChoCumN3(0n, 30);
if (JSON.stringify(seedKhac) === JSON.stringify(lan1)) throw new Error("seed KHAC (0n) phai cho ket qua KHAC seed=3n (it nhat mot truong khac)");
if (seedKhac[0]!.kieuLoiDia !== 0) throw new Error("seed=0n, backup dau tien (chiSo=1) phai co kieuLoiDia=0 (khong loi) -- gia tri THAT tu ChinhSachLoi, khong duoc hardcode");
if (seedKhac[1]!.kieuLoiDia !== 1) throw new Error("seed=0n, backup thu hai (chiSo=2) phai co kieuLoiDia=1 (lost fsync) -- neu kieuLoiDia luon la 0 (hardcode) day se sai");

const khongMatGoi = chayChoCumN3(5n, 0);
for (const r of khongMatGoi) {
  if (r.prepareToiNoi !== true) throw new Error("ty le mat 0% thi MOI backup phai nhan duoc Prepare");
  if (r.kieuLoiDia === null) throw new Error("da nhan Prepare thi PHAI co kieuLoiDia (0,1, hoac 2), khong duoc null");
  if (r.kieuLoiDia! < 0 || r.kieuLoiDia! > 2) throw new Error("kieuLoiDia phai nam trong [0,2]");
}

const matHetGoi = chayChoCumN3(5n, 100);
for (const r of matHetGoi) {
  if (r.prepareToiNoi !== false) throw new Error("ty le mat 100% thi KHONG backup nao duoc nhan Prepare");
  if (r.kieuLoiDia !== null) throw new Error("backup KHONG nhan duoc Prepare thi KHONG duoc co kieuLoiDia -- phai la null");
}

// torn write PHAI ap dung DUNG sector (0, khop voi disk.write(0,...) ben duoi) --
// khong chi kiem tra GIA TRI TRA VE (kieuLoiDia=2), ma phai kiem TRANG THAI THAT
// tren dia sau do (dung "sector" sai se lam corruption khong bao gio xay ra).
const csTorn = taoChinhSachLoi(31n);
const diskTorn = new SimDisk(4);
const rTorn = xuLyPrepareVoiChinhSachLoiDayDu(csTorn, 1, diskTorn, 30);
if (rTorn.kieuLoiDia !== 2) throw new Error("seed=31n phai cho kieuLoiDia=2 (torn write) -- sanity check gia tri that truoc khi kiem dia");
if (JSON.stringify([...diskTorn.read(0)]) !== '[9,9,0,0]') throw new Error("torn write phai AP DUNG DUNG sector 0 (khop voi disk.write(0,...)) -- doc lai sector 0 phai la [9,9,0,0] (2 byte moi + 2 byte cu tu torn write that), khong phai [9,9,9,9] (sach hoan toan, nghia la sector truyen vao apDungLoiDiaTheoChinhSach SAI)");
```

:::hints
- kind: attention
  body: "Goi apDungLoiDiaTheoChinhSach(cs, disk, 0) roi gan ket qua vao kieuLoiDia -- mot dong."
- kind: strategy
  body: "const kieuLoiDia = apDungLoiDiaTheoChinhSach(cs, disk, 0);"
- kind: one-line
  body: "const kieuLoiDia = apDungLoiDiaTheoChinhSach(cs, disk, 0);"
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
Một seed, một chính sách, toàn bộ lịch sử sự cố đĩa lẫn mạng của CẢ
cụm. Nhưng nếu một backup THẬT SỰ "chết" và khởi động lại — nó phục
hồi từ đâu?
::::

::::reflect{#nghi-lai}
`xuLyPrepareVoiChinhSachLoiDayDu` không thêm phép TOÁN mới — nó ráp
lại đúng `ChinhSachLoi` (q18 bài 8) VÀ `ghiEntryXuongDia`-style write
(bài 3) THÀNH một bước duy nhất, lặp qua NHIỀU replica thay vì MỘT
cặp đĩa/mạng đơn lẻ. Bài học lớn NHẤT: mở rộng phạm vi tiêm lỗi TỪ
"một máy" sang "cả cụm" không cần MỘT cơ chế mới — chỉ cần đúng MỘT
`tt`, dùng lại NHIỀU lần, theo đúng thứ tự.
::::

::::checkpoint{mastery=0.85}
::::
