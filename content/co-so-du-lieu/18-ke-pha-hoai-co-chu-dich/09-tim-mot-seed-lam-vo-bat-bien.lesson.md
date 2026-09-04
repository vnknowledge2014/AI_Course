---
id: co-so-du-lieu.ke-pha-hoai-co-chu-dich.tim-mot-seed-lam-vo-bat-bien
title: "Tìm một seed làm vỡ bất biến"
summary: "apDungVaLuu ghi MOI but toan xuong SimDisk theo kieu KHONG NGUYEN TU: ghi sector no, fsync(), ghi sector co, fsync() -- HAI fsync tach roi, khong phai mot giao dich. Chay 3 but toan (nap 1000, chuyen 400, chuyen 100) tren MOT tai khoan NGUON_NGOAI/tk1/tk2 THAT (heThongCanBang tu q16), dat crash tat dinh bang soNguyenTrongKhoang(tt,1,7). seed=0n (n=3): canBang=true SAU khi doc lai tu dia. seed=1n (n=6): canBang=FALSE -- but toan cuoi mat mot nua, tong no != tong co. Seed nay KHONG doan, DA chay that bang node de tim."
locale: vi
track: co-so-du-lieu
module: ke-pha-hoai-co-chu-dich
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [db.tim-mot-seed-lam-vo-bat-bien]
requires: [db.chinh-sach-tiem-loi]
concepts: [db.tim-mot-seed-lam-vo-bat-bien]
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
Tám bài — ba lỗi đĩa, một crash tất định, hai lỗi mạng, một chính
sách gộp tất cả dưới một seed. Giờ dùng chúng để làm việc THẬT: làm
vỡ bất biến `heThongCanBang` (q16) — không phải trên giấy, TRÊN đĩa.
::::

::::explain{#khong-nguyen-tu}
`apDungVaLuu` áp một `ButToan` (LẤY nguyên `TaiKhoan`/`heThongCanBang`
từ q16) RỒI ghi CẢ hai tài khoản (nợ và có) xuống `SimDisk` — nhưng
theo kiểu KHÔNG NGUYÊN TỬ: ghi sector nợ, `fsync()`, ghi sector có,
`fsync()` — HAI lần `fsync()` tách rời, không phải một giao dịch duy
nhất. Nếu `datCrashSauThaoTacThuN` cắt ngang ĐÚNG giữa hai `fsync()`
đó, một nửa giao dịch bền, nửa kia MẤT:

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

for (const seed of [0n, 1n]) console.log(`seed=${seed}n: canBang=${chayKichBan(seed)}`);
```

```text title=readonly
seed=0n: canBang=true
seed=1n: canBang=false
```

`seed=1n` chọn `n=6` — crash cắt ngang ĐÚNG giữa hai `fsync()` cuối
CÙNG của bút toán 3 (chuyển `100` từ `tk2` về `tk1`): sector `tk2`
(nợ) đã BỀN từ TRƯỚC (`fsync` của chính bút toán đó), nhưng sector
`tk1` (có, `+100`) chưa kịp `fsync()` khi crash xảy ra — MẤT trắng.
Đọc lại TỪ đĩa (`docLaiTuDia`, giả lập "khởi động lại"): `tk1` thiếu
mất `100` phần CÓ, `heThongCanBang` (q16) trả về `false`. `seed=0n`
chọn `n=3` — cắt ngang một VỊ trí "vô hại" (bút toán CHƯA từng fsync
lần NÀO ở đó, TOÀN bộ giao dịch coi như chưa xảy ra) — vẫn cân bằng.
::::

::::example{#khong-phai-moi-seed-deu-vo}
Không PHẢI seed nào cũng làm vỡ bất biến — `seed=0n` AN TOÀN (`n=3`
rơi vào một điểm mà việc mất một write KHÔNG để lại hậu quả, vì bút
toán đó CHƯA từng ghi bền phần NÀO). Đây chính LÀ lý do cần một cách
QUÉT nhiều seed thay vì đoán TAY — bài BOSS (bài 11) sẽ làm ĐÚNG việc
đó một cách tự động. `heThongCanBang` (q16 bài 2) LÀ đúng bất biến cốt
lõi mà `heThongSoCai` (BOSS q16) cũng kiểm tra SAU mọi cơ chế nạp/gửi/
pending/idempotent — Ở ĐÂY, chỉ cần LÀM vỡ đúng phần cốt lõi ĐÓ bằng
một sự cố ĐĨA THẬT là đủ chứng minh: một hệ thống ĐÚNG về logic (như
`heThongSoCai` đã chứng minh Ở q16) vẫn có thể vỡ bất biến nếu tầng
LƯU TRỮ bên dưới không đảm bảo tính nguyên tử.
::::

::::predict{#doan-tai-sao-mat-mot-nua commitOnce}
`apDungVaLuu` gọi `fsync()` NGAY sau MỖI lần `write()` (không gộp cả
hai write thành MỘT fsync). Nếu đổi thành gộp — ghi CẢ hai sector rồi
mới gọi `fsync()` MỘT lần DUY NHẤT — `seed=1n` (VẪN chọn `n=6`) còn
làm vỡ bất biến không?
:::opt{correct}
KHÔNG — gộp thành một `fsync()` DUY NHẤT nghĩa LÀ cả hai sector cùng
BỀN hoặc cùng MẤT theo đúng MỘT lần flush; nếu crash xảy ra TRƯỚC
`fsync()` đó, CẢ hai write bị mất (giao dịch coi như chưa xảy ra —
vẫn cân bằng); không CÒN "nửa bền, nửa mất" được nữa
:::
:::opt
CÓ, vẫn vỡ y hệt — gộp `fsync()` chỉ LÀ chi tiết cách viết code, không
ảnh hưởng gì bản chất bug
::why
Trực giác NÀY đánh giá THẤP đúng NGUYÊN NHÂN gốc của bug: KHÔNG phải
"có ghi xuống đĩa" mà LÀ "hai ghi liên quan tới NHAU được commit TÁCH
RỜI nhau".

Chỗ lệch: bug NÀY LÀ một ví dụ kinh điển của "non-atomic multi-sector
write" — chính XÁC vấn đề mà WAL (write-ahead log, R6-1) VÀ transaction
tồn tại để giải quyết: nhóm NHIỀU thay đổi liên quan thành MỘT đơn vị
bền/không-bền duy nhất. Gộp `fsync()` LÀ một bước hướng TỚI atomicity
thật — nó không "sửa" hoàn toàn (còn phụ thuộc cách WAL ghi), nhưng nó
loại bỏ đúng LỚP bug NÀY.
::
:::
::::

::::code{#viet_ap_dung_va_luu}
Hoàn thiện `apDungVaLuu` — cập nhật CẢ hai tài khoản trong bộ nhớ, RỒI
ghi + fsync sector nợ, RỒI ghi + fsync sector có (từng cái MỘT, không
gộp — đây chính LÀ chỗ hở khiến bất biến có thể vỡ).

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
  ___
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

console.log(chayKichBan(1n));
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

console.log(chayKichBan(1n));
```

```typescript title=test
if (chayKichBan(1n) !== false) throw new Error("seed=1n PHAI lam vo bat bien (canBang=false)");
if (chayKichBan(0n) !== true) throw new Error("seed=0n KHONG duoc lam vo bat bien (canBang=true)");

const laiLan1 = chayKichBan(1n);
const laiLan2 = chayKichBan(1n);
if (laiLan1 !== false || laiLan2 !== false) throw new Error("goi lai seed=1n NHIEU lan phai LUON cho canBang=false");

const diskKiemTra = new SimDisk(8);
const cacTKKiemTra = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, TK1, TK2]) cacTKKiemTra.set(id, taoTaiKhoan(id));
apDungVaLuu(diskKiemTra, cacTKKiemTra, { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
const tk1SauGhi = bytesThanhTaiKhoan(TK1, diskKiemTra.read(TK1));
if (tk1SauGhi.creditsPosted !== 1000) throw new Error("KHONG crash: apDungVaLuu phai ghi BEN ca hai sector, tk1.creditsPosted phai la 1000 tren dia");
const nguonSauGhi = bytesThanhTaiKhoan(NGUON_NGOAI, diskKiemTra.read(NGUON_NGOAI));
if (nguonSauGhi.debitsPosted !== 1000) throw new Error("KHONG crash: sector NGUON_NGOAI phai co debitsPosted=1000 tren dia");

if (cacTKKiemTra.get(TK1)!.creditsPosted !== 1000) throw new Error("trang thai TRONG BO NHO (khong qua doc dia) cung phai dung 1000");
```

:::hints
- kind: attention
  body: "Ghi + fsync sector no, ROI ghi + fsync sector co -- BON dong, tach roi tung cai mot."
- kind: strategy
  body: "disk.write(debit.id, taiKhoanThanhBytes(debit)); disk.fsync(); disk.write(credit.id, taiKhoanThanhBytes(credit)); disk.fsync();"
- kind: one-line
  body: "disk.write(debit.id, taiKhoanThanhBytes(debit)); disk.fsync(); disk.write(credit.id, taiKhoanThanhBytes(credit)); disk.fsync();"
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
Đã tìm ra một seed thật làm vỡ bất biến — không phải giả định, KIỂM
CHỨNG được bằng cách chạy. Bài tiếp theo: chứng minh nó tái hiện lại
được TUYỆT ĐỐI, y hệt q17 bài 7-8.
::::

::::reflect{#nghi-lai}
`apDungVaLuu` không giới thiệu khái niệm MỚI — nó ráp lại `SimDisk`
(bài 2-5), `heThongCanBang` (q16), VÀ `datCrashSauThaoTacThuN` (bài 5)
thành MỘT kịch bản có thể THẤT BẠI thật. Bài học lớn nhất: một hệ
thống có thể ĐÚNG trong MỌI test bình thường (không crash) nhưng vẫn
VỠ bất biến khi một sự cố xảy ra ĐÚNG vị trí — và chỉ mô phỏng tất
định mới cho PHÉP tìm ra vị trí đó một cách CÓ hệ thống, thay vì chờ
may rủi trong sản xuất.
::::

::::checkpoint{mastery=0.9}
::::
