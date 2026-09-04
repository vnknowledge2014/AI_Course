---
id: co-so-du-lieu.byteledger-capstone.tiem-loi-dia-cho-tung-replica
title: "Tiêm lỗi đĩa cho từng replica"
summary: "ghiEntryXuongDia đóng gói MỘT LogEntry (20 byte: opNumber/id/debitAccountId/creditAccountId/amount, DataView -- q16) rồi write+fsync vào sector=opNumber-1 trên SimDisk (q18) RIÊNG của replica đó. docLogTuDia đọc lại tuần tự, DỪNG NGAY khi gặp sector rỗng (opNumber=0) -- một LỖ HỔNG, không phải cuối log THẬT. Ghi bình thường 2 entry: đọc lại đúng 2. Gọi datCrashSauThaoTacThuN(1) rồi ghi entry thứ 3 (op3): crash cắt ngang ĐÚNG lúc đó -- đọc lại VẪN chỉ 2 entry, op3 'biến mất' khỏi đĩa dù log trong bộ nhớ của replica đó đã có 3."
locale: vi
track: co-so-du-lieu
module: byteledger-capstone
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.tiem-loi-dia-cho-tung-replica]
requires: [db.but-toan-qua-vsr-idempotent]
concepts: [db.tiem-loi-dia-cho-tung-replica]
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
Bút toán ĐÃ đi đúng đường VSR, áp dụng idempotent (bài trước) — nhưng
MỌI thứ tới giờ chỉ sống trong bộ nhớ. q18 đã dạy MỘT `SimDisk` có
thể "nói dối" (lost fsync, torn write, crash tất định) — giờ MỖI
replica trong cụm cần GHI log của chính nó xuống một đĩa RIÊNG.
::::

::::explain{#ghi-log-xuong-dia}
`ghiEntryXuongDia` đóng gói MỘT `LogEntry` thành `20` byte (`opNumber`,
`id`, `debitAccountId`, `creditAccountId`, `amount` — CÙNG kỹ thuật
`DataView` q16 bài 4, chỉ đơn giản hoá bố cục) RỒI `write`+`fsync`
VÀO đúng sector `opNumber - 1` trên `SimDisk` (q18) — MỘT đĩa RIÊNG
CHO đúng replica đó. `docLogTuDia` đọc lại TUẦN tự, DỪNG ngay khi gặp
một sector "rỗng" (`opNumber=0`, tức LÀ chưa từng được ghi bền) —
MỘT lỗ HỔNG trên đĩa, không phải "cuối log thật":

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

interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }

const KICH_THUOC_BAN_GHI_DIA = 20;
function goiLogEntry(entry: LogEntry): Uint8Array {
  const buf = new ArrayBuffer(KICH_THUOC_BAN_GHI_DIA);
  const view = new DataView(buf);
  view.setUint32(0, entry.opNumber);
  view.setUint32(4, entry.bt.id);
  view.setUint32(8, entry.bt.debitAccountId);
  view.setUint32(12, entry.bt.creditAccountId);
  view.setUint32(16, entry.bt.amount);
  return new Uint8Array(buf);
}
function moGoiLogEntry(bytes: Uint8Array): LogEntry {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  return {
    opNumber: view.getUint32(0),
    bt: { id: view.getUint32(4), debitAccountId: view.getUint32(8), creditAccountId: view.getUint32(12), amount: view.getUint32(16) },
  };
}
function ghiEntryXuongDia(disk: SimDisk, entry: LogEntry): void {
  disk.write(entry.opNumber - 1, goiLogEntry(entry));
  disk.fsync();
}
function docLogTuDia(disk: SimDisk, soLuongOpToiDa: number): LogEntry[] {
  const ketQua: LogEntry[] = [];
  for (let sector = 0; sector < soLuongOpToiDa; sector++) {
    const entry = moGoiLogEntry(disk.read(sector));
    if (entry.opNumber === 0) break;
    ketQua.push(entry);
  }
  return ketQua;
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const diskR2 = new SimDisk(KICH_THUOC_BAN_GHI_DIA);
ghiEntryXuongDia(diskR2, { opNumber: 1, bt: { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 } });
ghiEntryXuongDia(diskR2, { opNumber: 2, bt: { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 } });
console.log("sau 2 lan ghi binh thuong:", docLogTuDia(diskR2, 5).length, "entry");

diskR2.datCrashSauThaoTacThuN(1);
ghiEntryXuongDia(diskR2, { opNumber: 3, bt: { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 } });
console.log("sau crash dung luc ghi op3:", docLogTuDia(diskR2, 5).length, "entry");
```

```text title=readonly
sau 2 lan ghi binh thuong: 2 entry
sau crash dung luc ghi op3: 2 entry
```

`ghiEntryXuongDia` ghi VÀ `fsync()` NGAY trong CÙNG một lời gọi (giống
q18 bài 5's `ghiTungCaiVaFsync`) — CHO nên `datCrashSauThaoTacThuN(1)`
cắt NGANG đúng lần `write()` KẾ tiếp (op3), TRƯỚC khi `fsync()` của
CHÍNH nó kịp chạy. `docLogTuDia` đọc LẠI: sector `0` (op1) VÀ sector
`1` (op2) đều CÒN nguyên — nhưng sector `2` (op3) TRỐNG (opNumber=0,
CHƯA từng bền) — vòng lặp DỪNG ngay TẠI đó, trả VỀ đúng `2` entry, dù
`op3` CÓ thể đã tồn tại TRONG bộ nhớ của replica đó.
::::

::::example{#log-nho-hon-bo-nho-la-binh-thuong}
Đây LÀ khác biệt CĂN bản GIỮA "log trong bộ nhớ" (`replica.log`, q19)
VÀ "log trên đĩa" (`docLogTuDia`): sau MỘT sự cố đĩa, hai thứ ĐÓ CÓ
thể LỆCH nhau — bộ nhớ "nhớ" nhiều HƠN những gì đã THẬT sự bền. Đây
KHÔNG phải lỗi thiết kế — LÀ ĐÚNG hành vi mong ĐỢI của MỘT WAL (R6-1):
nếu replica đó "chết" NGAY sau đó VÀ khởi động lại, nó CHỈ có thể tin
TƯỞNG những gì đọc LẠI được TỪ đĩa — phần "biết thêm" trong bộ nhớ đã
mất VĨNH viễn cùng RAM. Bài 5 sẽ dùng ĐÚNG khoảng cách NÀY để dạy
"phục hồi".
::::

::::predict{#doan-lo-hong-giua-log commitOnce}
Ghi ĐÚNG op1 (sector 0) VÀ op4 (sector 3) — NHƯNG bỏ QUA op2 VÀ op3
(chưa BAO giờ gọi `ghiEntryXuongDia` cho CHÚNG — mô phỏng "hai lần
ghi lỡ mất hoàn TOÀN", không PHẢI do crash). Gọi `docLogTuDia(disk,
5)`. Kết quả TRẢ về CÓ bao nhiêu entry?
:::opt{correct}
`1` — CHỈ sector `0` (op1) CÓ dữ liệu; sector `1` (đáng LẼ op2) trống
NGAY từ đầu (`opNumber=0`), vòng LẶP dừng Ở đó — op4 (sector `3`,
THẬT ra ĐÃ được ghi ĐÚNG) KHÔNG bao GIỜ được đọc tới, vì `docLogTuDia`
KHÔNG "nhảy qua" lỗ hổng để tìm tiếp
:::
:::opt
`2` — `docLogTuDia` bỏ QUA những sector trống VÀ đọc TIẾP, thu thập
được CẢ op1 lẫn op4 (hai entry THẬT sự tồn tại trên đĩa)
::why
Trực giác NÀY tưởng tượng MỘT vòng lặp "nhặt hết những GÌ có" — nhưng
`docLogTuDia` (đúng NHƯ code) `break` NGAY khi gặp sector trống, KHÔNG
`continue` để đọc tiếp.

Chỗ lệch: một LOG (theo đúng Ý nghĩa VSR, opNumber LIÊN tục) KHÔNG
được PHÉP có lỗ hổng Ở GIỮA — nếu op2 chưa bền, op4 (dù nằm SAU nó
VỀ mặt opNumber) CŨNG không thể coi LÀ "đã biết" một CÁCH an toàn,
VÌ không CÒN gì đảm bảo mọi op TRƯỚC nó ĐÃ áp dụng đúng thứ tự. Dừng
NGAY tại lỗ hổng đầu tiên LÀ hành vi AN toàn — đọc "nhảy cóc" mới LÀ
lỗi.
::
:::
::::

::::code{#viet_doc_log_tu_dia}
Hoàn thiện `docLogTuDia` — nếu sector đọc ĐƯỢC có `opNumber=0` (chưa
từng ghi bền), DỪNG vòng lặp NGAY, không đọc tiếp các sector sau.

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

interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }

const KICH_THUOC_BAN_GHI_DIA = 20;
function goiLogEntry(entry: LogEntry): Uint8Array {
  const buf = new ArrayBuffer(KICH_THUOC_BAN_GHI_DIA);
  const view = new DataView(buf);
  view.setUint32(0, entry.opNumber);
  view.setUint32(4, entry.bt.id);
  view.setUint32(8, entry.bt.debitAccountId);
  view.setUint32(12, entry.bt.creditAccountId);
  view.setUint32(16, entry.bt.amount);
  return new Uint8Array(buf);
}
function moGoiLogEntry(bytes: Uint8Array): LogEntry {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  return {
    opNumber: view.getUint32(0),
    bt: { id: view.getUint32(4), debitAccountId: view.getUint32(8), creditAccountId: view.getUint32(12), amount: view.getUint32(16) },
  };
}
function ghiEntryXuongDia(disk: SimDisk, entry: LogEntry): void {
  disk.write(entry.opNumber - 1, goiLogEntry(entry));
  disk.fsync();
}
function docLogTuDia(disk: SimDisk, soLuongOpToiDa: number): LogEntry[] {
  const ketQua: LogEntry[] = [];
  for (let sector = 0; sector < soLuongOpToiDa; sector++) {
    const entry = moGoiLogEntry(disk.read(sector));
    ___
    ketQua.push(entry);
  }
  return ketQua;
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const diskR2 = new SimDisk(KICH_THUOC_BAN_GHI_DIA);
ghiEntryXuongDia(diskR2, { opNumber: 1, bt: { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 } });
ghiEntryXuongDia(diskR2, { opNumber: 2, bt: { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 } });
diskR2.datCrashSauThaoTacThuN(1);
ghiEntryXuongDia(diskR2, { opNumber: 3, bt: { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 } });
console.log(docLogTuDia(diskR2, 5).length);
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

interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }

const KICH_THUOC_BAN_GHI_DIA = 20;
function goiLogEntry(entry: LogEntry): Uint8Array {
  const buf = new ArrayBuffer(KICH_THUOC_BAN_GHI_DIA);
  const view = new DataView(buf);
  view.setUint32(0, entry.opNumber);
  view.setUint32(4, entry.bt.id);
  view.setUint32(8, entry.bt.debitAccountId);
  view.setUint32(12, entry.bt.creditAccountId);
  view.setUint32(16, entry.bt.amount);
  return new Uint8Array(buf);
}
function moGoiLogEntry(bytes: Uint8Array): LogEntry {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  return {
    opNumber: view.getUint32(0),
    bt: { id: view.getUint32(4), debitAccountId: view.getUint32(8), creditAccountId: view.getUint32(12), amount: view.getUint32(16) },
  };
}
function ghiEntryXuongDia(disk: SimDisk, entry: LogEntry): void {
  disk.write(entry.opNumber - 1, goiLogEntry(entry));
  disk.fsync();
}
function docLogTuDia(disk: SimDisk, soLuongOpToiDa: number): LogEntry[] {
  const ketQua: LogEntry[] = [];
  for (let sector = 0; sector < soLuongOpToiDa; sector++) {
    const entry = moGoiLogEntry(disk.read(sector));
    if (entry.opNumber === 0) break;
    ketQua.push(entry);
  }
  return ketQua;
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const diskR2 = new SimDisk(KICH_THUOC_BAN_GHI_DIA);
ghiEntryXuongDia(diskR2, { opNumber: 1, bt: { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 } });
ghiEntryXuongDia(diskR2, { opNumber: 2, bt: { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 } });
diskR2.datCrashSauThaoTacThuN(1);
ghiEntryXuongDia(diskR2, { opNumber: 3, bt: { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 } });
console.log(docLogTuDia(diskR2, 5).length);
```

```typescript title=test
const NGUON_NGOAI2 = 0, TK1_2 = 1, TK2_2 = 2;
const disk2 = new SimDisk(KICH_THUOC_BAN_GHI_DIA);
if (docLogTuDia(disk2, 5).length !== 0) throw new Error("dia rong (chua ghi gi) phai doc lai duoc mang rong");

ghiEntryXuongDia(disk2, { opNumber: 1, bt: { id: 10, debitAccountId: NGUON_NGOAI2, creditAccountId: TK1_2, amount: 500 } });
ghiEntryXuongDia(disk2, { opNumber: 2, bt: { id: 11, debitAccountId: TK1_2, creditAccountId: TK2_2, amount: 200 } });
ghiEntryXuongDia(disk2, { opNumber: 3, bt: { id: 12, debitAccountId: TK2_2, creditAccountId: TK1_2, amount: 50 } });
const doc3 = docLogTuDia(disk2, 5);
if (doc3.length !== 3) throw new Error("ghi 3 lan binh thuong phai doc lai duoc dung 3 entry");
if (doc3[0]!.opNumber !== 1 || doc3[1]!.opNumber !== 2 || doc3[2]!.opNumber !== 3) throw new Error("thu tu doc lai phai dung opNumber tang dan 1,2,3");
if (doc3[1]!.bt.amount !== 200) throw new Error("entry thu hai phai co amount=200");
if (doc3[2]!.bt.debitAccountId !== TK2_2) throw new Error("entry thu ba phai co debitAccountId=TK2");

const disk3 = new SimDisk(KICH_THUOC_BAN_GHI_DIA);
ghiEntryXuongDia(disk3, { opNumber: 1, bt: { id: 20, debitAccountId: NGUON_NGOAI2, creditAccountId: TK1_2, amount: 1000 } });
ghiEntryXuongDia(disk3, { opNumber: 2, bt: { id: 21, debitAccountId: TK1_2, creditAccountId: TK2_2, amount: 400 } });
disk3.datCrashSauThaoTacThuN(1);
ghiEntryXuongDia(disk3, { opNumber: 3, bt: { id: 22, debitAccountId: TK2_2, creditAccountId: TK1_2, amount: 100 } });
const doc4 = docLogTuDia(disk3, 5);
if (doc4.length !== 2) throw new Error("crash dung luc ghi op3 phai lam doc lai CHI thay 2 entry (op3 mat trang), khong phai 3");
if (doc4[1]!.opNumber !== 2) throw new Error("entry cuoi doc duoc phai la op2, op3 khong ton tai tren dia");

if (docLogTuDia(disk3, 2).length !== 2) throw new Error("gioi han soLuongOpToiDa=2 phai chi doc toi da 2 sector, du dia co the co nhieu hon");

const disk4 = new SimDisk(KICH_THUOC_BAN_GHI_DIA);
ghiEntryXuongDia(disk4, { opNumber: 1, bt: { id: 30, debitAccountId: NGUON_NGOAI2, creditAccountId: TK1_2, amount: 100 } });
ghiEntryXuongDia(disk4, { opNumber: 2, bt: { id: 31, debitAccountId: TK1_2, creditAccountId: TK2_2, amount: 20 } });
ghiEntryXuongDia(disk4, { opNumber: 4, bt: { id: 32, debitAccountId: TK2_2, creditAccountId: TK1_2, amount: 5 } });
const doc5 = docLogTuDia(disk4, 5);
if (doc5.length !== 2) throw new Error("co MOT LO HONG o sector 2 (op3 chua bao gio ghi) -- phai DUNG lai tai do, khong duoc doc tiep sang op4 o sector 3");
```

:::hints
- kind: attention
  body: "Neu entry.opNumber === 0 (sector chua tung duoc ghi ben) thi break NGAY, dung doc tiep -- mot dong."
- kind: strategy
  body: "if (entry.opNumber === 0) break;"
- kind: one-line
  body: "if (entry.opNumber === 0) break;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một replica giờ ghi được log của CHÍNH nó xuống đĩa, VÀ đọc lại đúng
(kể cả khi thiếu). Nhưng đĩa VÀ mạng chưa hề gặp NHAU dưới cùng một
seed — bước tiếp theo.
::::

::::reflect{#nghi-lai}
`docLogTuDia` không phát MINH khái niệm mới — nó áp DỤNG lại đúng ý
tưởng "đọc tuần TỰ, dừng Ở lỗ hổng đầu tiên" TỪ q18 bài 9 (`sauCrash`)
CHO một cấu trúc dữ liệu CÓ ý nghĩa nghiệp vụ (`LogEntry`, không chỉ
byte thô). Bài học lớn NHẤT: "log trên đĩa" của một replica LÀ một
BẢN ghi độc lập, riêng của NÓ — và nó CÓ thể lạc hậu so VỚI bộ nhớ
của chính replica đó, chưa NÓI tới so với các replica KHÁC.
::::

::::checkpoint{mastery=0.85}
::::
