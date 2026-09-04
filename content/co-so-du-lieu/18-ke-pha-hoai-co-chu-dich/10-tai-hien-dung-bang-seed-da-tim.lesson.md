---
id: co-so-du-lieu.ke-pha-hoai-co-chu-dich.tai-hien-dung-bang-seed-da-tim
title: "Tái hiện đúng bằng seed đã tìm"
summary: "taiHienNhieuLan chay chayKichBan(seed) N lan doc lap, tra ve mang N ket qua canBang. Voi seed=1n (bai truoc), goi 5 lan: TAT CA deu false, khong mot lan nao 'ngau nhien' cho true -- dung tinh than q17 bai 7-8. seed=0n, 5 lan: TAT CA deu true. Tai hien tuyet doi tro thanh mot thu KIEM TRA duoc bang code (mang ket qua dong nhat), khong phai chi quan sat bang mat mot lan roi tin."
locale: vi
track: co-so-du-lieu
module: ke-pha-hoai-co-chu-dich
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.tai-hien-dung-bang-seed-da-tim]
requires: [db.tim-mot-seed-lam-vo-bat-bien]
concepts: [db.tai-hien-dung-bang-seed-da-tim]
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
`seed=1n` làm vỡ bất biến — MỘT lần chạy. Nhưng một bug chỉ tái hiện
ĐÚNG một lần trong hàng nghìn lần chạy thì VÔ dụng để gỡ lỗi. Nó có
tái hiện được LẦN THỨ HAI không?
::::

::::explain{#tai-hien-nhieu-lan}
`taiHienNhieuLan` chạy `chayKichBan` (bài trước) VỚI cùng một seed,
NHIỀU lần độc lập (mỗi lần tạo `disk`/`cacTaiKhoan` HOÀN toàn mới) —
rồi trả về TOÀN bộ dãy kết quả để kiểm tra chúng có khớp NHAU không:

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
  private demLanGhi = 0;
  private crashSauLanGhiThu: number | null = null;
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  read(sector: number): Uint8Array {
    const o = this.cache.get(sector) ?? this.platter.get(sector);
    return o ? o.slice() : new Uint8Array(this.kichThuocSector);
  }
  write(sector: number, data: Uint8Array): void {
    this.cache.set(sector, data.slice());
    this.demLanGhi++;
    if (this.crashSauLanGhiThu !== null && this.demLanGhi >= this.crashSauLanGhiThu) {
      this.crashSauLanGhiThu = null;
      this.crash();
    }
  }
  fsync(): void {
    for (const [sector, data] of this.cache) this.platter.set(sector, data);
    this.cache.clear();
  }
  crash(): void { this.cache.clear(); }
  datCrashSauThaoTacThuN(n: number): void { this.crashSauLanGhiThu = this.demLanGhi + n; }
}

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function heThongCanBang(cacTaiKhoan: Map<number, TaiKhoan>): boolean {
  let no = 0, co = 0;
  for (const [, tk] of cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function taiKhoanThanhBytes(tk: TaiKhoan): Uint8Array {
  const buf = new Uint8Array(8);
  const dv = new DataView(buf.buffer);
  dv.setUint32(0, tk.debitsPosted);
  dv.setUint32(4, tk.creditsPosted);
  return buf;
}
function bytesThanhTaiKhoan(id: number, bytes: Uint8Array): TaiKhoan {
  const dv = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  return { id, debitsPosted: dv.getUint32(0), creditsPosted: dv.getUint32(4) };
}
function apDungVaLuu(disk: SimDisk, cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
  disk.write(debit.id, taiKhoanThanhBytes(debit));
  disk.fsync();
  disk.write(credit.id, taiKhoanThanhBytes(credit));
  disk.fsync();
}
function docLaiTuDia(disk: SimDisk, cacId: number[]): Map<number, TaiKhoan> {
  const m = new Map<number, TaiKhoan>();
  for (const id of cacId) m.set(id, bytesThanhTaiKhoan(id, disk.read(id)));
  return m;
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
function chayKichBan(seed: bigint): boolean {
  const tt = gieoHat(seed);
  const n = soNguyenTrongKhoang(tt, 1, 7);
  const disk = new SimDisk(8);
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of [NGUON_NGOAI, TK1, TK2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
  disk.datCrashSauThaoTacThuN(n);
  apDungVaLuu(disk, cacTaiKhoan, { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
  apDungVaLuu(disk, cacTaiKhoan, { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 });
  apDungVaLuu(disk, cacTaiKhoan, { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 });
  const sau = docLaiTuDia(disk, [NGUON_NGOAI, TK1, TK2]);
  return heThongCanBang(sau);
}

function taiHienNhieuLan(seed: bigint, soLan: number): boolean[] {
  const ketQua: boolean[] = [];
  for (let i = 0; i < soLan; i++) ketQua.push(chayKichBan(seed));
  return ketQua;
}

console.log("seed=1n, 5 lan:", taiHienNhieuLan(1n, 5));
console.log("seed=0n, 5 lan:", taiHienNhieuLan(0n, 5));
```

```text title=readonly
seed=1n, 5 lan: [ false, false, false, false, false ]
seed=0n, 5 lan: [ true, true, true, true, true ]
```

Không có LẦN nào lệch. `seed=1n` LUÔN làm vỡ bất biến, `seed=0n` LUÔN
giữ nó — dù chạy `5` hay `5000` lần. Đây chính LÀ điều làm mô phỏng
tất định HỮU dụng cho việc gỡ lỗi: một bug tìm được TỪ seed nào đó
KHÔNG "biến mất" khi thử lại — nó CHỜ đúng Ở đó, mọi lần.
::::

::::example{#khac-mot-flaky-test}
So với một "flaky test" (test thỉnh THOẢNG fail vì đụng `Date.now()`
hay điều kiện đua tranh THẬT) — nơi việc CHẠY LẠI có thể "che" bug đi
— `chayKichBan(1n)` là NGƯỢC lại hoàn toàn: chạy lại càng NHIỀU càng
CHẮC chắn bug LÀ có thật, không phải nhiễu.
::::

::::predict{#doan-doi-lai-nguon-ngoai commitOnce}
Nếu ĐỔI thứ tự tạo tài khoản trong `chayKichBan` — TỪ `[NGUON_NGOAI,
TK1, TK2]` thành `[TK2, TK1, NGUON_NGOAI]` (chỉ đổi THỨ TỰ khởi tạo
`Map`, không đổi ID hay logic NÀO khác) — `taiHienNhieuLan(1n, 5)` còn
cho `[false,false,false,false,false]` không?
:::opt{correct}
CÓ, vẫn y HỆT — `Map` không quan tâm thứ tự CHÈN để tra cứu bằng
`.get(id)`, và `heThongCanBang` cộng dồn TRÊN toàn bộ Map bất kể thứ
tự duyệt — không CÓ bước nào trong toàn bộ kịch bản phụ thuộc thứ tự
khởi tạo
:::
:::opt
Không chắc — thứ tự trong `Map` CÓ thể ảnh hưởng thứ tự `write()` được
gọi, làm lệch số đếm `demLanGhi`
::why
Trực giác NÀY nhầm "thứ tự KHỞI TẠO một Map" VỚI "thứ tự các lệnh
GỌI SAU đó" — nhưng KHÔNG có vòng lặp NÀO Ở đây duyệt qua `cacTaiKhoan`
để quyết định thứ tự ghi; MỌI lệnh `write()` xuất phát từ CHÍNH các
lời gọi `apDungVaLuu` viết SẴN theo đúng thứ tự cố định trong code.

Chỗ lệch: `demLanGhi` chỉ tăng khi `disk.write()` được GỌI — và các
lời gọi ĐÓ nằm trong thân `apDungVaLuu`, hoàn toàn ĐỘC lập với thứ tự
`cacTaiKhoan.set(...)` lúc khởi tạo. Đổi thứ tự khởi tạo Map không đổi
GÌ về thứ tự thực thi thực TẾ của chương trình.
::
:::
::::

::::code{#viet_tai_hien_nhieu_lan}
Hoàn thiện `taiHienNhieuLan` — gọi `chayKichBan(seed)` đúng `soLan`
lần, đẩy TỪNG kết quả vào mảng trả về.

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
  private demLanGhi = 0;
  private crashSauLanGhiThu: number | null = null;
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  read(sector: number): Uint8Array {
    const o = this.cache.get(sector) ?? this.platter.get(sector);
    return o ? o.slice() : new Uint8Array(this.kichThuocSector);
  }
  write(sector: number, data: Uint8Array): void {
    this.cache.set(sector, data.slice());
    this.demLanGhi++;
    if (this.crashSauLanGhiThu !== null && this.demLanGhi >= this.crashSauLanGhiThu) {
      this.crashSauLanGhiThu = null;
      this.crash();
    }
  }
  fsync(): void {
    for (const [sector, data] of this.cache) this.platter.set(sector, data);
    this.cache.clear();
  }
  crash(): void { this.cache.clear(); }
  datCrashSauThaoTacThuN(n: number): void { this.crashSauLanGhiThu = this.demLanGhi + n; }
}

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function heThongCanBang(cacTaiKhoan: Map<number, TaiKhoan>): boolean {
  let no = 0, co = 0;
  for (const [, tk] of cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function taiKhoanThanhBytes(tk: TaiKhoan): Uint8Array {
  const buf = new Uint8Array(8);
  const dv = new DataView(buf.buffer);
  dv.setUint32(0, tk.debitsPosted);
  dv.setUint32(4, tk.creditsPosted);
  return buf;
}
function bytesThanhTaiKhoan(id: number, bytes: Uint8Array): TaiKhoan {
  const dv = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  return { id, debitsPosted: dv.getUint32(0), creditsPosted: dv.getUint32(4) };
}
function apDungVaLuu(disk: SimDisk, cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
  disk.write(debit.id, taiKhoanThanhBytes(debit));
  disk.fsync();
  disk.write(credit.id, taiKhoanThanhBytes(credit));
  disk.fsync();
}
function docLaiTuDia(disk: SimDisk, cacId: number[]): Map<number, TaiKhoan> {
  const m = new Map<number, TaiKhoan>();
  for (const id of cacId) m.set(id, bytesThanhTaiKhoan(id, disk.read(id)));
  return m;
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
function chayKichBan(seed: bigint): boolean {
  const tt = gieoHat(seed);
  const n = soNguyenTrongKhoang(tt, 1, 7);
  const disk = new SimDisk(8);
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of [NGUON_NGOAI, TK1, TK2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
  disk.datCrashSauThaoTacThuN(n);
  apDungVaLuu(disk, cacTaiKhoan, { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
  apDungVaLuu(disk, cacTaiKhoan, { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 });
  apDungVaLuu(disk, cacTaiKhoan, { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 });
  const sau = docLaiTuDia(disk, [NGUON_NGOAI, TK1, TK2]);
  return heThongCanBang(sau);
}

function taiHienNhieuLan(seed: bigint, soLan: number): boolean[] {
  const ketQua: boolean[] = [];
  ___
  return ketQua;
}

console.log(taiHienNhieuLan(1n, 5));
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
  private demLanGhi = 0;
  private crashSauLanGhiThu: number | null = null;
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  read(sector: number): Uint8Array {
    const o = this.cache.get(sector) ?? this.platter.get(sector);
    return o ? o.slice() : new Uint8Array(this.kichThuocSector);
  }
  write(sector: number, data: Uint8Array): void {
    this.cache.set(sector, data.slice());
    this.demLanGhi++;
    if (this.crashSauLanGhiThu !== null && this.demLanGhi >= this.crashSauLanGhiThu) {
      this.crashSauLanGhiThu = null;
      this.crash();
    }
  }
  fsync(): void {
    for (const [sector, data] of this.cache) this.platter.set(sector, data);
    this.cache.clear();
  }
  crash(): void { this.cache.clear(); }
  datCrashSauThaoTacThuN(n: number): void { this.crashSauLanGhiThu = this.demLanGhi + n; }
}

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function heThongCanBang(cacTaiKhoan: Map<number, TaiKhoan>): boolean {
  let no = 0, co = 0;
  for (const [, tk] of cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function taiKhoanThanhBytes(tk: TaiKhoan): Uint8Array {
  const buf = new Uint8Array(8);
  const dv = new DataView(buf.buffer);
  dv.setUint32(0, tk.debitsPosted);
  dv.setUint32(4, tk.creditsPosted);
  return buf;
}
function bytesThanhTaiKhoan(id: number, bytes: Uint8Array): TaiKhoan {
  const dv = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  return { id, debitsPosted: dv.getUint32(0), creditsPosted: dv.getUint32(4) };
}
function apDungVaLuu(disk: SimDisk, cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
  disk.write(debit.id, taiKhoanThanhBytes(debit));
  disk.fsync();
  disk.write(credit.id, taiKhoanThanhBytes(credit));
  disk.fsync();
}
function docLaiTuDia(disk: SimDisk, cacId: number[]): Map<number, TaiKhoan> {
  const m = new Map<number, TaiKhoan>();
  for (const id of cacId) m.set(id, bytesThanhTaiKhoan(id, disk.read(id)));
  return m;
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
function chayKichBan(seed: bigint): boolean {
  const tt = gieoHat(seed);
  const n = soNguyenTrongKhoang(tt, 1, 7);
  const disk = new SimDisk(8);
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of [NGUON_NGOAI, TK1, TK2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
  disk.datCrashSauThaoTacThuN(n);
  apDungVaLuu(disk, cacTaiKhoan, { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
  apDungVaLuu(disk, cacTaiKhoan, { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 });
  apDungVaLuu(disk, cacTaiKhoan, { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 });
  const sau = docLaiTuDia(disk, [NGUON_NGOAI, TK1, TK2]);
  return heThongCanBang(sau);
}

function taiHienNhieuLan(seed: bigint, soLan: number): boolean[] {
  const ketQua: boolean[] = [];
  for (let i = 0; i < soLan; i++) ketQua.push(chayKichBan(seed));
  return ketQua;
}

console.log(taiHienNhieuLan(1n, 5));
```

```typescript title=test
const ket1 = taiHienNhieuLan(1n, 5);
if (ket1.length !== 5) throw new Error("taiHienNhieuLan(seed, 5) phai tra ve mang dung 5 phan tu");
if (ket1.some((b) => b !== false)) throw new Error("seed=1n, MOI lan trong 5 lan deu phai la false (vo bat bien)");

const ket0 = taiHienNhieuLan(0n, 5);
if (ket0.some((b) => b !== true)) throw new Error("seed=0n, MOI lan trong 5 lan deu phai la true (giu bat bien)");

if (taiHienNhieuLan(1n, 0).length !== 0) throw new Error("soLan=0 phai tra ve mang rong");

const ket1LanNua = taiHienNhieuLan(1n, 5);
if (JSON.stringify(ket1LanNua) !== JSON.stringify(ket1)) throw new Error("goi lai taiHienNhieuLan(1n,5) TOAN BO phai cho DUNG cung mang ket qua");

const ket1Muoi = taiHienNhieuLan(1n, 10);
if (ket1Muoi.length !== 10) throw new Error("soLan=10 phai tra ve dung 10 phan tu");
if (ket1Muoi.some((b) => b !== false)) throw new Error("seed=1n, ca 10 lan deu phai la false");
```

:::hints
- kind: attention
  body: "Vong lap soLan lan, moi lan push(chayKichBan(seed)) -- mot dong trong vong lap for."
- kind: strategy
  body: "for (let i = 0; i < soLan; i++) ketQua.push(chayKichBan(seed));"
- kind: one-line
  body: "for (let i = 0; i < soLan; i++) ketQua.push(chayKichBan(seed));"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một bug tìm được, tái hiện được tuyệt đối. Còn thiếu MỘT mảnh: TỰ ĐỘNG
tìm seed đó, thay VÌ đã biết trước LÀ `1n`. BOSS q18.
::::

::::reflect{#nghi-lai}
`taiHienNhieuLan` không thêm khái niệm MỚI — nó LÀ phép kiểm chứng
trực tiếp cho lời hứa "chạy lại đúng tuyệt đối" (q17 bài 7) áp DỤNG
lên một bug THẬT vừa tìm được (bài 9), không còn LÀ lý thuyết. Bài
học lớn nhất xuyên suốt cả q17 lẫn q18: một hệ thống mô phỏng chỉ THẬT
SỰ hữu ích để săn bug khi "tái hiện được" không phải LÀ một lời hứa
suông mà LÀ một thứ có thể GỌI HÀM để kiểm tra.
::::

::::checkpoint{mastery=0.85}
::::
