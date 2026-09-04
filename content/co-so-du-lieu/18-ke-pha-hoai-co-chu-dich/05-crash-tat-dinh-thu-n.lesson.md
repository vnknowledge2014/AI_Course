---
id: co-so-du-lieu.ke-pha-hoai-co-chu-dich.crash-tat-dinh-thu-n
title: "Crash tất định thứ N"
summary: "datCrashSauThaoTacThuN(n) làm crash() TỰ ĐỘNG kích hoạt đúng SAU lần write() thứ n kể từ lúc gọi -- không phải một độ trễ thời gian thật. ghiTungCaiVaFsync ghi 5 sector, mỗi cái fsync() ngay sau khi ghi: crashSauThaoTacThuN(3) làm ĐÚNG sector thứ 3 (index 2) mất trắng, bốn sector còn lại (đã fsync trước/ghi lại sau khi bộ đếm crash đã dùng hết) vẫn nguyên -- xác nhận cho cả sáu giá trị n=1..6, kể cả n=6 (vượt tổng số write, không crash gì)."
locale: vi
track: co-so-du-lieu
module: ke-pha-hoai-co-chu-dich
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.crash-tat-dinh-thu-n]
requires: [db.misdirected-write-ghi-nham-cho]
concepts: [db.crash-tat-dinh-thu-n]
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
Ba lỗi đĩa (bài 2-4) đều cần GỌI `crash()` TAY để lộ ra. Nhưng crash
thật xảy ra Ở một THỜI ĐIỂM không biết trước — làm sao mô phỏng ĐÚNG
lúc "không biết trước NHƯNG tái lập được"?
::::

::::explain{#crash-tat-dinh-thu-n}
`datCrashSauThaoTacThuN(n)` đặt lịch: ĐÚNG sau lần `write()` thứ `n`
KỂ TỪ lúc gọi hàm này, `crash()` TỰ ĐỘNG kích hoạt — không cần gọi tay.
Khác `Math.random()`/một độ trễ thời gian thật: thời điểm crash do
đúng MỘT con số quyết định (tính được từ seed, q17):

```typescript title=readonly
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

function ghiTungCaiVaFsync(disk: SimDisk, cacDuLieu: Uint8Array[]): void {
  for (let i = 0; i < cacDuLieu.length; i++) {
    disk.write(i, cacDuLieu[i]!);
    disk.fsync();
  }
}

function chayThu(n: number): number[] {
  const disk = new SimDisk(2);
  const cacDuLieu = [1, 2, 3, 4, 5].map((v) => new Uint8Array([v, v]));
  disk.datCrashSauThaoTacThuN(n);
  ghiTungCaiVaFsync(disk, cacDuLieu);
  return [0, 1, 2, 3, 4].map((i) => disk.read(i)[0]!);
}

for (const n of [1, 2, 3, 6]) console.log(`n=${n}:`, chayThu(n));
```

```text title=readonly
n=1: [ 0, 2, 3, 4, 5 ]
n=2: [ 1, 0, 3, 4, 5 ]
n=3: [ 1, 2, 0, 4, 5 ]
n=6: [ 1, 2, 3, 4, 5 ]
```

`ghiTungCaiVaFsync` ghi RỒI `fsync()` NGAY từng sector một — bình
thường mỗi sector sẽ BỀN ngay sau lượt của nó. `crashSauThaoTacThuN(n)`
làm ĐÚNG lần `write()` thứ `n` bị "cắt ngang": `write()` vẫn đưa dữ
liệu vào cache, NHƯNG `crash()` tự động chạy NGAY sau đó — TRƯỚC khi
`fsync()` của chính lượt đó kịp chạy — nên dữ liệu Ở VỊ TRÍ thứ `n-1`
(0-indexed) mất trắng, còn TẤT cả vị trí khác (đã fsync TRƯỚC, hoặc ghi
lại SAU khi bộ đếm đã dùng hết — đây LÀ bẫy một lần) vẫn nguyên.
`n=6` vượt quá tổng số `write()` (chỉ có 5) — không lần nào chạm mốc,
không crash gì cả.
::::

::::example{#khac-mot-do-tre-that}
Nếu dùng `setTimeout(..., Math.random() * 1000)` để crash "ngẫu nhiên"
Ở đâu đó giữa chừng, CHẠY LẠI script hai lần sẽ crash Ở hai lần `write()`
khác nhau — bug (nếu có) "biến mất" hoặc "xuất hiện" tuỳ may rủi. Với
`datCrashSauThaoTacThuN(n)`, `n` LÀ một tham số TƯỜNG minh: cùng `n`
LUÔN cắt ngang ĐÚNG cùng một lần `write()`, mọi lần chạy.
::::

::::predict{#doan-hai-lan-dat-lich commitOnce}
Trên MỘT `disk` (không tạo lại), gọi `disk.datCrashSauThaoTacThuN(2)`,
rồi `disk.write(0, ...)`, `disk.write(1, ...)` (write thứ hai TỰ kích
crash). SAU đó gọi THÊM `disk.datCrashSauThaoTacThuN(1)` rồi
`disk.write(2, ...)`. Lần `write(2, ...)` này CÓ tự kích crash không?
:::opt{correct}
CÓ — `datCrashSauThaoTacThuN` lần gọi THỨ HAI đặt lại mốc mới dựa trên
`demLanGhi` HIỆN TẠI (lúc đó LÀ `2`, cộng thêm `1` = mốc `3`) — VÀ
`write(2, ...)` chính LÀ lần `write()` thứ `3`, khớp mốc mới, kích
crash lần NỮA
:::
:::opt
Không — bộ đếm chỉ kích hoạt được ĐÚNG một lần trong TOÀN bộ vòng đời
`disk`, gọi `datCrashSauThaoTacThuN` lần hai không có tác DỤNG gì nữa
::why
Trực giác NÀY nhầm "one-shot per instance" VỚI "one-shot per LẦN đặt
lịch" — nhưng `crashSauLanGhiThu` chỉ bị đưa VỀ `null` SAU khi nó kích
hoạt, KHÔNG phải sau khi được TẠO ra.

Chỗ lệch: gọi `datCrashSauThaoTacThuN` LẠI đơn giản GÁN một mốc MỚI
(`demLanGhi + n`, tính TỪ số lần ghi HIỆN TẠI, không phải từ 0) — hoàn
toàn ĐỘC lập với việc mốc trước đó đã kích hoạt hay CHƯA.
::
:::
::::

::::code{#viet_ghi_tung_cai_va_fsync}
Hoàn thiện `ghiTungCaiVaFsync` — với MỖI phần tử, `write(i, cacDuLieu[i])`
rồi `fsync()` NGAY.

```typescript title=starter
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

function ghiTungCaiVaFsync(disk: SimDisk, cacDuLieu: Uint8Array[]): void {
  for (let i = 0; i < cacDuLieu.length; i++) {
    ___
  }
}

const disk = new SimDisk(2);
const cacDuLieu = [1, 2, 3, 4, 5].map((v) => new Uint8Array([v, v]));
disk.datCrashSauThaoTacThuN(3);
ghiTungCaiVaFsync(disk, cacDuLieu);
console.log([0, 1, 2, 3, 4].map((i) => disk.read(i)[0]!));
```

```typescript title=solution
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

function ghiTungCaiVaFsync(disk: SimDisk, cacDuLieu: Uint8Array[]): void {
  for (let i = 0; i < cacDuLieu.length; i++) {
    disk.write(i, cacDuLieu[i]!);
    disk.fsync();
  }
}

const disk = new SimDisk(2);
const cacDuLieu = [1, 2, 3, 4, 5].map((v) => new Uint8Array([v, v]));
disk.datCrashSauThaoTacThuN(3);
ghiTungCaiVaFsync(disk, cacDuLieu);
console.log([0, 1, 2, 3, 4].map((i) => disk.read(i)[0]!));
```

```typescript title=test
function chayThuT(n: number): number[] {
  const disk = new SimDisk(2);
  const cacDuLieu = [1, 2, 3, 4, 5].map((v) => new Uint8Array([v, v]));
  disk.datCrashSauThaoTacThuN(n);
  ghiTungCaiVaFsync(disk, cacDuLieu);
  return [0, 1, 2, 3, 4].map((i) => disk.read(i)[0]!);
}

const ketQuaMongDoi: Record<number, number[]> = {
  1: [0, 2, 3, 4, 5],
  2: [1, 0, 3, 4, 5],
  3: [1, 2, 0, 4, 5],
  4: [1, 2, 3, 0, 5],
  5: [1, 2, 3, 4, 0],
  6: [1, 2, 3, 4, 5],
};
for (const n of [1, 2, 3, 4, 5, 6]) {
  const ket = chayThuT(n);
  if (JSON.stringify(ket) !== JSON.stringify(ketQuaMongDoi[n])) {
    throw new Error(`n=${n} phai cho [${ketQuaMongDoi[n]}], nhan duoc [${ket}]`);
  }
}

const diskDem = new SimDisk(2);
if (diskDem.read(0)[0] !== 0) throw new Error("sector chua ghi phai la 0");
diskDem.datCrashSauThaoTacThuN(0);
diskDem.write(0, new Uint8Array([9, 9]));
if (diskDem.read(0)[0] !== 0) throw new Error("n=0 phai crash NGAY lan write dau tien, mat trang");
```

:::hints
- kind: attention
  body: "Voi moi i: disk.write(i, cacDuLieu[i]!), roi disk.fsync() -- hai dong trong vong lap."
- kind: strategy
  body: "disk.write(i, cacDuLieu[i]!); disk.fsync();"
- kind: one-line
  body: "disk.write(i, cacDuLieu[i]!); disk.fsync();"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1,2,0,4,5"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba lỗi đĩa VÀ một cách kích hoạt crash tất định — xong nửa đầu q18.
Nửa sau: mạng cũng có lỗi, VÀ cũng cần seed quyết định.
::::

::::reflect{#nghi-lai}
`datCrashSauThaoTacThuN` không giới thiệu khái niệm MỚI — nó LÀ phiên
bản "đếm thao tác" của cùng ý tưởng xuyên suốt q18: một CON SỐ tường
minh (`n`, tính TỪ một seed) quyết định thời điểm sự cố, thay VÌ một
độ trễ thời gian thật hay `Math.random()`. Đây LÀ mảnh ghép cuối cùng
Ở phía ĐĨA — bài 9-11 sẽ dùng CHÍNH cơ chế này để tìm một seed làm vỡ
bất biến của một hệ thống thật.
::::

::::checkpoint{mastery=0.85}
::::
