---
id: co-so-du-lieu.ke-pha-hoai-co-chu-dich.boss-ke-pha-hoai-co-chu-dich
title: "BOSS — Kẻ phá hoại có chủ đích"
summary: "timSeedLamVoBatBien(soLuongSeedThu) quet seed=0n,1n,2n,... tang dan, chay chayKichBan(seed) cho MOI seed, tra ve SEED DAU TIEN lam heThongCanBang=false, hoac null neu KHONG tim thay trong pham vi quet -- khong bao gio bia so. Quet 1 seed (chi 0n): null (0n an toan). Quet 2 seed (0n,1n): tra ve 1n -- VA chayKichBan(1n) lap lai 5 lan van luon false, chung minh tai hien tuyet doi. Day la 'mini-VOPR': tu dong tim + tu dong tai hien, khong can nguoi doan seed."
locale: vi
track: co-so-du-lieu
module: ke-pha-hoai-co-chu-dich
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.tai-hien-dung-bang-seed-da-tim]
concepts: [db.boss-q18]
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
Mười bài — năm lỗi (ba đĩa, một crash tất định, hai mạng gộp lại
thành một chính sách), tìm MỘT seed vỡ bất biến bằng tay (`1n`), tái
hiện được. Mảnh cuối: TỰ ĐỘNG tìm, không đoán trước.
::::

::::explain{#mini-vopr}
`timSeedLamVoBatBien` LÀ một "mini-VOPR" (TigerBeetle gọi công cụ thật
của họ LÀ VOPR — "Viewstamped Operation Replicator", một bộ quét seed
tự động): quét `seed=0n, 1n, 2n, ...` tăng DẦN, chạy `chayKichBan(seed)`
(bài 9) cho MỖI seed, DỪNG lại VÀ trả về seed ĐẦU TIÊN làm bất biến
vỡ — hoặc `null` nếu quét HẾT phạm vi mà KHÔNG tìm thấy gì:

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

function timSeedLamVoBatBien(soLuongSeedThu: number): bigint | null {
  for (let i = 0; i < soLuongSeedThu; i++) {
    const seed = BigInt(i);
    if (!chayKichBan(seed)) return seed;
  }
  return null;
}

console.log("quet 1 seed:", String(timSeedLamVoBatBien(1)));
console.log("quet 2 seed:", String(timSeedLamVoBatBien(2)));
console.log("quet 0 seed:", String(timSeedLamVoBatBien(0)));
```

```text title=readonly
quet 1 seed: null
quet 2 seed: 1
quet 0 seed: null
```

`timSeedLamVoBatBien(1)` chỉ thử `seed=0n` — AN toàn, trả `null`
TRUNG thực (KHÔNG bịa ra một seed không hề phá được gì).
`timSeedLamVoBatBien(2)` thử THÊM `seed=1n` — tìm thấy NGAY, trả về
`1n` (in RA thành `"1"` — `String(mộtBigInt)` bỏ hậu tố `n`, đó CHỈ
LÀ cú pháp literal, không phải một phần giá trị). `timSeedLamVoBatBien
(0)` — phạm vi RỖNG, không thử seed nào, `null`. Ba lời gọi, BA kết
quả khác nhau, không hề đoán MÒ — MỖI kết quả LÀ hệ quả trực tiếp của
việc CHẠY THẬT `chayKichBan` trên đúng những seed nằm trong phạm vi
quét. (Bọc `String(...)` quanh kết quả LÀ bắt buộc Ở đây — `console.log`
một `bigint` trần in ĐÚNG trên máy phát triển, nhưng bộ chấm bài thật
dùng `JSON.stringify` cho mọi giá trị không-phải-chuỗi, và
`JSON.stringify` NÉM lỗi ngay khi gặp `bigint`.)
::::

::::example{#day-la-vopr-that}
TigerBeetle's VOPR THẬT quét HÀNG triệu seed mỗi ngày trên CI, mỗi
seed LÀ một vũ trụ tất định với hàng NGÀN sự kiện (đĩa, mạng, tiến
trình) — khi tìm được MỘT seed làm vỡ bất biến, đội ngũ CHỈ cần đúng
con số đó để tái hiện bug TRÊN máy của họ, byte-for-byte, không cần
mô tả "làm sao để tái tạo lỗi" bằng lời. `timSeedLamVoBatBien` LÀ
đúng nguyên lý ĐÓ, thu nhỏ về một kịch bản BA bút toán.
::::

::::predict{#doan-neu-khong-tim-thay commitOnce}
Giả sử (giả ĐỊNH, không cần chạy thử) MỌI seed từ `0n` đến `100n` đều
AN toàn (không seed nào làm vỡ bất biến — kịch bản khác VỚI kịch bản
Ở TRÊN). `timSeedLamVoBatBien(101)` trong trường hợp ĐÓ trả về GÌ?
:::opt{correct}
`null` — vòng lặp thử HẾT `soLuongSeedThu=101` seed (từ `0` tới `100`)
mà KHÔNG seed nào làm `chayKichBan` trả `false`, nên rơi RA khỏi vòng
lặp và chạm `return null;`
:::
:::opt
Ném lỗi — một hàm "tìm" thất bại thì PHẢI báo lỗi, không được trả về
im lặng
::why
Trực giác NÀY hợp lý cho NHIỀU tình huống khác — nhưng SAI CHO chính
hàm NÀY, vì kiểu trả về đã khai báo RÕ LÀ `bigint | null`: `null` LÀ
một kết quả HỢP LỆ, có Ý nghĩa riêng ("đã thử hết phạm vi, không thấy
gì"), không phải MỘT trường hợp lỗi.

Chỗ lệch: "không tìm thấy" và "lỗi" LÀ hai điều khác nhau. Một VOPR
thật CŨNG vậy: quét xong `N` seed mà không thấy bug KHÔNG có nghĩa LÀ
chương trình đã "hỏng" — nó chỉ có nghĩa LÀ cần quét NHIỀU seed hơn,
hoặc hệ thống THẬT sự không có bug loại NÀY (Ở PHẠM VI đã thử).
::
:::
::::

::::code{#viet_tim_seed_lam_vo_bat_bien}
Hoàn thiện `timSeedLamVoBatBien` — với MỖI `i` từ `0` tới
`soLuongSeedThu - 1`, chạy `chayKichBan(BigInt(i))`; NẾU trả `false`
(vỡ bất biến), trả về NGAY seed đó.

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

function timSeedLamVoBatBien(soLuongSeedThu: number): bigint | null {
  for (let i = 0; i < soLuongSeedThu; i++) {
    ___
  }
  return null;
}

console.log(String(timSeedLamVoBatBien(5)));
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

function timSeedLamVoBatBien(soLuongSeedThu: number): bigint | null {
  for (let i = 0; i < soLuongSeedThu; i++) {
    const seed = BigInt(i);
    if (!chayKichBan(seed)) return seed;
  }
  return null;
}

console.log(String(timSeedLamVoBatBien(5)));
```

```typescript title=test
if (timSeedLamVoBatBien(1) !== null) throw new Error("quet chi seed=0n (an toan) phai tra ve null");
if (timSeedLamVoBatBien(0) !== null) throw new Error("quet 0 seed (pham vi rong) phai tra ve null");

const tim = timSeedLamVoBatBien(2);
if (tim !== 1n) throw new Error("quet seed=0n VA 1n phai tim thay DUNG 1n (seed dau tien lam vo bat bien)");

const timLai = timSeedLamVoBatBien(2);
if (timLai !== tim) throw new Error("goi lai timSeedLamVoBatBien(2) phai tra ve DUNG cung ket qua");

const timNhieuHon = timSeedLamVoBatBien(10);
if (timNhieuHon !== 1n) throw new Error("quet nhieu hon (10 seed) van phai tra ve seed DAU TIEN tim thay, 1n, khong phai seed nao khac trong khoang");

for (let i = 0; i < 5; i++) {
  if (tim !== null && chayKichBan(tim) !== false) throw new Error("seed tim duoc phai tai hien vo bat bien o CA 5 lan chay lai doc lap");
}
```

:::hints
- kind: attention
  body: "Voi moi i: seed=BigInt(i); neu !chayKichBan(seed) thi return seed ngay -- hai dong."
- kind: strategy
  body: "const seed = BigInt(i); if (!chayKichBan(seed)) return seed;"
- kind: one-line
  body: "const seed = BigInt(i); if (!chayKichBan(seed)) return seed;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
q18 khép lại: năm lỗi có chủ đích, một chính sách gộp dưới một seed,
một mini-VOPR tự tìm VÀ tự tái hiện bug. q19 (Đồng thuận kiểu VSR) sẽ
cần đúng những công cụ này để tiêm lỗi mạng GIỮA nhiều bản sao.
::::

::::reflect{#nghi-lai}
`timSeedLamVoBatBien` không giới thiệu khái niệm MỚI nào — nó LÀ một
vòng lặp `for` bọc quanh `chayKichBan` (bài 9), chỉ khác Ở chỗ: KHÔNG
còn người phải NÓI "thử seed 1n xem sao" — chính CHƯƠNG TRÌNH tự tìm.
Bài học lớn nhất của q18: một kẻ phá hoại "có chủ đích" không có nghĩa
LÀ biết trước NÓ sẽ phá cái gì — mà LÀ mọi hành vi của nó (kể cả quá
trình TÌM ra lỗi) đều tất định, kiểm chứng được, VÀ tái hiện tuyệt
đối. Đó chính LÀ nền móng q19 (VSR) sẽ đứng LÊN: đồng thuận giữa nhiều
bản sao, dưới lỗi mạng, tất cả VẪN phải chạy lại đúng tuyệt đối khi
cần gỡ lỗi.
::::

::::checkpoint{mastery=0.95}
::::
