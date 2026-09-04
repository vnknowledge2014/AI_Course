---
id: co-so-du-lieu.ke-pha-hoai-co-chu-dich.chon-loi-bang-seed-khong-phai-nguoi
title: "Chọn lỗi bằng seed, không phải người"
summary: "chonKieuLoi dùng soNguyenTrongKhoang(tt,0,3) để chọn GIỮA ba khả năng: 0=không lỗi, 1=lost fsync (R6-1 q02), 2=torn write (R6-1 q02) — rồi apDungLoiTheoSeed áp lỗi đó vào SimDisk. seed=0n chọn torn write (đọc sau crash: [9,9,0,0]); seed=2n chọn không lỗi ([9,9,9,9]); seed=4n chọn lost fsync ([0,0,0,0]) -- mỗi seed LUÔN chọn lại đúng khả năng đó, xác nhận bằng cách chạy lại."
locale: vi
track: co-so-du-lieu
module: ke-pha-hoai-co-chu-dich
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.chon-loi-bang-seed-khong-phai-nguoi]
requires: [db.loi-co-chu-dich-khong-phai-ngau-nhien]
concepts: [db.chon-loi-bang-seed-khong-phai-nguoi]
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
R6-1 (q02 "Khi điện mất") dạy hai lỗi đĩa: lost fsync, torn write —
NGƯỜI viết bài chọn TAY khi nào kích hoạt lỗi nào. Nếu để `chonMucTieu`
(bài trước) TỰ chọn giữa chúng thì sao?
::::

::::explain{#chon_loi_bang_seed}
`chonKieuLoi` trả về `0`, `1`, hay `2` — MỘT trong ba khả năng, chọn
bằng chính `tt`. `apDungLoiTheoSeed` áp đúng khả năng ĐÓ lên `SimDisk`
(copy nguyên ngữ nghĩa `boQuaFsyncKeTiep`/`danhDauTornGhi` từ R6-1 q02):

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
  crash(): void { this.cache.clear(); }
  boQuaFsyncKeTiep(): void { this.matFsyncKeTiep = true; }
  danhDauTornGhi(sector: number, soByteThanhCong: number): void { this.tornSector.set(sector, soByteThanhCong); }
}

function chonKieuLoi(tt: TrangThai): number {
  return soNguyenTrongKhoang(tt, 0, 3); // 0=khong loi, 1=lost fsync, 2=torn write
}
function apDungLoiTheoSeed(disk: SimDisk, tt: TrangThai, sector: number): number {
  const kieu = chonKieuLoi(tt);
  if (kieu === 1) disk.boQuaFsyncKeTiep();
  else if (kieu === 2) disk.danhDauTornGhi(sector, Math.floor(disk.kichThuocSector / 2));
  return kieu;
}

function chayThu(seed: bigint): { kieu: number; sauCrash: number[] } {
  const disk = new SimDisk(4);
  const tt = gieoHat(seed);
  disk.write(0, new Uint8Array([9, 9, 9, 9]));
  const kieu = apDungLoiTheoSeed(disk, tt, 0);
  disk.fsync();
  disk.crash();
  return { kieu, sauCrash: [...disk.read(0)] };
}

for (const seed of [0n, 2n, 4n]) console.log(`seed=${seed}n:`, JSON.stringify(chayThu(seed)));
```

```text title=readonly
seed=0n: {"kieu":2,"sauCrash":[9,9,0,0]}
seed=2n: {"kieu":0,"sauCrash":[9,9,9,9]}
seed=4n: {"kieu":1,"sauCrash":[0,0,0,0]}
```

Ba seed, ba kết quả HOÀN toàn khác nhau — không phải VÌ ba lỗi khác
nhau "ngẫu nhiên xảy ra", mà VÌ `chonKieuLoi` đọc ba giá trị KHÁC nhau
từ ba `tt` khác nhau. `seed=4n` chọn `kieu=1` (lost fsync): `fsync()`
nói dối, `crash()` xoá sạch cache CHƯA từng xuống platter → đọc lại
toàn số `0`. `seed=0n` chọn `kieu=2` (torn write, 2/4 byte): `fsync()`
chỉ ghi được `9,9`, hai byte sau giữ `0` cũ. `seed=2n` chọn `kieu=0`
(không lỗi): ghi bền HOÀN toàn, `crash()` không mất gì.
::::

::::example{#tai-lap-dung-loi-cu}
Nếu một bug chỉ tái hiện Ở seed=`4n` (lost fsync), ghi lại đúng con số
`4n` LÀ đủ để tái hiện — không cần nhớ "hôm đó lỗi loại gì". Đây LÀ lý
do R6-3 gọi đây là "kẻ phá hoại CÓ CHỦ ĐÍCH": tuy KHÔNG biết trước loại
lỗi nào sẽ xảy ra khi CHỌN seed, nhưng MỘT khi đã chọn, hành vi hoàn
toàn xác định VÀ tái lập được.
::::

::::predict{#doan-goi-lai-cung-seed commitOnce}
Gọi `chayThu(0n)` HAI lần độc lập (tạo `disk` VÀ `tt` mới mỗi lần, y
hệt hàm ĐANG viết). Hai kết quả trả về CÓ giống hệt nhau không?
:::opt{correct}
CÓ — `gieoHat(0n)` luôn cho ĐÚNG cùng trạng thái ban đầu, VÀ mọi bước
sau đó (`chonKieuLoi`, ghi, fsync, crash) đều tất định, KHÔNG hề đụng
`Math.random()`/`Date.now()`
:::
:::opt
Không chắc — `SimDisk` LÀ một class CÓ trạng thái nội bộ (Map), hai
instance riêng biệt có THỂ ứng xử khác nhau
::why
Trực giác NÀY đúng RẰNG mỗi `chayThu` tạo MỘT `disk` MỚI (một `Map`
rỗng khác) — nhưng "khác instance" KHÔNG có nghĩa LÀ "khác hành vi":
cả hai instance đều bắt đầu Ở đúng CÙNG trạng thái rỗng.

Chỗ lệch: hành vi của `SimDisk` hoàn toàn do CHUỖI lệnh gọi lên nó
quyết định (`write`, `fsync`, `crash`), KHÔNG có trạng thái ẩn nào
khác biệt giữa hai instance mới tạo. VÀ chuỗi lệnh đó (bao gồm CẢ
`kieu` mà `chonKieuLoi` chọn) hoàn toàn do `tt` quyết định — cùng seed
thì cùng `tt` ban đầu thì cùng chuỗi lệnh thì cùng kết quả.
::
:::
::::

::::code{#viet_ap_dung_loi_theo_seed}
Hoàn thiện `apDungLoiTheoSeed` — nếu `kieu === 2`, đánh dấu torn write
Ở `sector` với `soByteThanhCong = kichThuocSector / 2` (làm tròn xuống).

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
  crash(): void { this.cache.clear(); }
  boQuaFsyncKeTiep(): void { this.matFsyncKeTiep = true; }
  danhDauTornGhi(sector: number, soByteThanhCong: number): void { this.tornSector.set(sector, soByteThanhCong); }
}

function chonKieuLoi(tt: TrangThai): number {
  return soNguyenTrongKhoang(tt, 0, 3);
}
function apDungLoiTheoSeed(disk: SimDisk, tt: TrangThai, sector: number): number {
  const kieu = chonKieuLoi(tt);
  if (kieu === 1) disk.boQuaFsyncKeTiep();
  else if (kieu === 2) { ___ }
  return kieu;
}

const disk = new SimDisk(4);
const tt = gieoHat(0n);
disk.write(0, new Uint8Array([9, 9, 9, 9]));
apDungLoiTheoSeed(disk, tt, 0);
disk.fsync();
disk.crash();
console.log([...disk.read(0)]);
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
  crash(): void { this.cache.clear(); }
  boQuaFsyncKeTiep(): void { this.matFsyncKeTiep = true; }
  danhDauTornGhi(sector: number, soByteThanhCong: number): void { this.tornSector.set(sector, soByteThanhCong); }
}

function chonKieuLoi(tt: TrangThai): number {
  return soNguyenTrongKhoang(tt, 0, 3);
}
function apDungLoiTheoSeed(disk: SimDisk, tt: TrangThai, sector: number): number {
  const kieu = chonKieuLoi(tt);
  if (kieu === 1) disk.boQuaFsyncKeTiep();
  else if (kieu === 2) disk.danhDauTornGhi(sector, Math.floor(disk.kichThuocSector / 2));
  return kieu;
}

const disk = new SimDisk(4);
const tt = gieoHat(0n);
disk.write(0, new Uint8Array([9, 9, 9, 9]));
apDungLoiTheoSeed(disk, tt, 0);
disk.fsync();
disk.crash();
console.log([...disk.read(0)]);
```

```typescript title=test
function chayThuT(seed: bigint): { kieu: number; sauCrash: number[] } {
  const disk = new SimDisk(4);
  const tt = gieoHat(seed);
  disk.write(0, new Uint8Array([9, 9, 9, 9]));
  const kieu = apDungLoiTheoSeed(disk, tt, 0);
  disk.fsync();
  disk.crash();
  return { kieu, sauCrash: [...disk.read(0)] };
}

const r0 = chayThuT(0n);
if (r0.kieu !== 2) throw new Error("seed=0n phai chon kieu=2 (torn write)");
if (JSON.stringify(r0.sauCrash) !== "[9,9,0,0]") throw new Error("seed=0n sau crash phai la [9,9,0,0] (2 byte dau moi, 2 byte sau van 0 cu)");

const r2 = chayThuT(2n);
if (r2.kieu !== 0) throw new Error("seed=2n phai chon kieu=0 (khong loi)");
if (JSON.stringify(r2.sauCrash) !== "[9,9,9,9]") throw new Error("seed=2n khong loi thi sau crash phai giu nguyen [9,9,9,9]");

const r4 = chayThuT(4n);
if (r4.kieu !== 1) throw new Error("seed=4n phai chon kieu=1 (lost fsync)");
if (JSON.stringify(r4.sauCrash) !== "[0,0,0,0]") throw new Error("seed=4n lost fsync thi sau crash phai mat sach, [0,0,0,0]");

const r0Lai = chayThuT(0n);
if (JSON.stringify(r0Lai) !== JSON.stringify(r0)) throw new Error("goi lai CUNG seed=0n phai cho DUNG cung ket qua (kieu VA sauCrash)");
```

:::hints
- kind: attention
  body: "kieu===2: goi disk.danhDauTornGhi(sector, Math.floor(disk.kichThuocSector / 2)) -- mot dong."
- kind: strategy
  body: "disk.danhDauTornGhi(sector, Math.floor(disk.kichThuocSector / 2));"
- kind: one-line
  body: "disk.danhDauTornGhi(sector, Math.floor(disk.kichThuocSector / 2));"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "9,9,0,0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Seed giờ chọn được GIỮA hai lỗi cũ (R6-1). Nhưng R6-1 chưa có: lỗi mà
`fsync()` KHÔNG cứu được — đọc SAI dù chưa hề crash. Bài tiếp theo.
::::

::::reflect{#nghi-lai}
`apDungLoiTheoSeed` không phát minh lỗi MỚI nào — nó chỉ chuyển quyền
"chọn lỗi nào" từ NGƯỜI viết bài (R6-1, gọi tay `danhDauTornGhi`) sang
MỘT con số (`tt`, R6-3). Đây LÀ bước đầu tiên hướng tới việc quét HÀNG
trăm seed tự động (BOSS q18) thay VÌ người phải nghĩ ra từng kịch bản
lỗi một.
::::

::::checkpoint{mastery=0.85}
::::
