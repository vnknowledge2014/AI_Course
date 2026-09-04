---
id: co-so-du-lieu.byteledger-capstone.crash-va-phuc-hoi-mot-replica
title: "Crash và phục hồi một replica"
summary: "phucHoiVaBatKip mô phỏng MỘT replica (r2) khởi động lại: tạo Replica MỚI hoàn toàn (mất sạch bộ nhớ), đọc log từ ĐĨA của chính nó (docLogTuDia, bài 3) rồi replay từng entry vào soCai -- sau đó dùng State Transfer (kiemTraCanDongBo/xuLyPhanHoiDongBo, q19 bài 13) hỏi primary CÒN SỐNG để bù đúng phần đĩa KHÔNG kịp ghi bền (một lỗi đĩa làm mất write của op3, bài 3's pattern). r0 (primary, không crash) có soDu TK1=700/TK2=300 sau 3 bút toán; đĩa của r2 chỉ còn 2/3 entry do crash; SAU phucHoiVaBatKip: r2Moi đạt opNumber=commitNumber=3, soDu TK1=700/TK2=300 -- hội tụ đúng với primary, KHÔNG cần view change (primary không hề chết)."
locale: vi
track: co-so-du-lieu
module: byteledger-capstone
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [db.crash-va-phuc-hoi-mot-replica]
requires: [db.chinh-sach-loi-toan-cum]
concepts: [db.crash-va-phuc-hoi-mot-replica]
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
Một seed điều khiển CẢ đĩa lẫn mạng (bài trước). Nhưng "tiêm lỗi" chỉ
LÀ nửa câu chuyện — điều THẬT sự quan trọng LÀ hệ thống làm GÌ SAU
đó. Nếu MỘT replica (không phải primary) thật sự "chết" và khởi động
lại, nó phục hồi bằng cách NÀO?
::::

::::explain{#phuc-hoi-tu-dia-va-state-transfer}
`phucHoiVaBatKip` mô phỏng ĐÚNG một replica khởi động LẠI: tạo
`Replica` HOÀN toàn mới (`taoReplica` — mất SẠCH bộ nhớ cũ), đọc log
TỪ đĩa của CHÍNH nó (`docLogTuDia`, bài 3) rồi REPLAY từng entry vào
`soCai` (giống tinh THẦN "số dư LÀ tổng đủ trừ", q16 bài 6). Nếu đĩa
CÒN thiếu (do một lỗi ĐĨA nào đó Ở bài trước), dùng ĐÚNG State
Transfer (`kiemTraCanDongBo`/`xuLyPhanHoiDongBo`, q19 bài 13) hỏi
primary CÒN sống để bù NỐT:

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

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }

interface HeThongSoCai {
  cacTaiKhoan: Map<number, TaiKhoan>;
  cacPendingDangCho: Map<number, ButToan>;
  dsIdDaXuLy: Set<number>;
}
function taoHeThongSoCai(cacId: number[]): HeThongSoCai {
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of cacId) cacTaiKhoan.set(id, taoTaiKhoan(id));
  return { cacTaiKhoan, cacPendingDangCho: new Map(), dsIdDaXuLy: new Set() };
}
function heThongCanBang(soCai: HeThongSoCai): boolean {
  let no = 0, co = 0;
  for (const [, tk] of soCai.cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function apDungQuaSoCai(soCai: HeThongSoCai, bt: ButToan): boolean {
  if (soCai.dsIdDaXuLy.has(bt.id)) return true;
  soCai.dsIdDaXuLy.add(bt.id);
  soCai.cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  soCai.cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
  return true;
}

interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
  soCai: HeThongSoCai;
}
function taoReplica(chiSo: number, tongSo: number, cacIdTaiKhoan: number[]): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0, soCai: taoHeThongSoCai(cacIdTaiKhoan) };
}
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }

interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  primary.opNumber += 1;
  const entry: LogEntry = { opNumber: primary.opNumber, bt };
  primary.log.push(entry);
  return entry;
}

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

function xuLyPrepareVaGhiDia(backup: Replica, disk: SimDisk, td: ThongDiep): ThongDiep {
  const entry: LogEntry = { opNumber: td.opNumber!, bt: td.bt! };
  backup.log.push(entry);
  backup.opNumber = td.opNumber!;
  ghiEntryXuongDia(disk, entry);
  return { loai: 'PrepareOk', tu: backup.chiSo, den: td.tu, viewNumber: backup.viewNumber, opNumber: td.opNumber! };
}
function nhanPrepareOk(primary: Replica, phieuTheoOp: Map<number, Set<number>>, td: ThongDiep): boolean {
  const opNumber = td.opNumber!;
  let phieu = phieuTheoOp.get(opNumber);
  if (!phieu) { phieu = new Set<number>([primary.chiSo]); phieuTheoOp.set(opNumber, phieu); }
  phieu.add(td.tu);
  if (phieu.size < nguongQuorum(primary.tongSo)) return false;
  if (opNumber !== primary.commitNumber + 1) return false;
  const entry = primary.log.find((e) => e.opNumber === opNumber)!;
  apDungQuaSoCai(primary.soCai, entry.bt);
  primary.commitNumber = opNumber;
  return true;
}
function xuLyCommit(backup: Replica, td: ThongDiep): void {
  const commitMoi = td.commitNumber!;
  for (let op = backup.commitNumber + 1; op <= commitMoi; op++) {
    const entry = backup.log.find((e) => e.opNumber === op)!;
    apDungQuaSoCai(backup.soCai, entry.bt);
  }
  backup.commitNumber = commitMoi;
}
function kiemTraCanDongBo(backup: Replica, opNumberMoiNhatDaBiet: number): boolean {
  return backup.opNumber < opNumberMoiNhatDaBiet;
}
function xuLyPhanHoiDongBo(backup: Replica, td: ThongDiep): void {
  const entryConThieu = td.log!;
  for (const entry of entryConThieu) if (entry.opNumber > backup.opNumber) backup.log.push(entry);
  backup.opNumber = backup.log.length === 0 ? 0 : backup.log[backup.log.length - 1]!.opNumber;
}

function chayMotVongVaGhiDia(primary: Replica, backups: { r: Replica; disk: SimDisk }[], bt: ButToan): void {
  const entry = nhanYeuCauTuClient(primary, bt);
  const phieuTheoOp = new Map<number, Set<number>>();
  for (const b of backups) {
    const phanHoi = xuLyPrepareVaGhiDia(b.r, b.disk, { loai: 'Prepare', tu: primary.chiSo, den: b.r.chiSo, viewNumber: primary.viewNumber, opNumber: entry.opNumber, bt: entry.bt });
    nhanPrepareOk(primary, phieuTheoOp, phanHoi);
  }
  for (const b of backups) {
    xuLyCommit(b.r, { loai: 'Commit', tu: primary.chiSo, den: b.r.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
  }
}

function phucHoiVaBatKip(diskCuaNo: SimDisk, chiSo: number, tongSo: number, cacIdTaiKhoan: number[], primary: Replica): Replica {
  const replicaMoi = taoReplica(chiSo, tongSo, cacIdTaiKhoan);
  const logTuDia = docLogTuDia(diskCuaNo, primary.opNumber);
  for (const entry of logTuDia) {
    replicaMoi.log.push(entry);
    replicaMoi.opNumber = entry.opNumber;
    apDungQuaSoCai(replicaMoi.soCai, entry.bt);
  }
  replicaMoi.commitNumber = replicaMoi.opNumber;

  if (kiemTraCanDongBo(replicaMoi, primary.opNumber)) {
    const conThieu = primary.log.filter((e) => e.opNumber > replicaMoi.opNumber);
    xuLyPhanHoiDongBo(replicaMoi, { loai: 'PhanHoiDongBo', tu: primary.chiSo, den: replicaMoi.chiSo, viewNumber: primary.viewNumber, log: conThieu });
  }
  for (let op = replicaMoi.commitNumber + 1; op <= primary.commitNumber; op++) {
    const e = replicaMoi.log.find((x) => x.opNumber === op)!;
    apDungQuaSoCai(replicaMoi.soCai, e.bt);
  }
  replicaMoi.commitNumber = primary.commitNumber;
  return replicaMoi;
}

function xacNhanHoiTu(a: HeThongSoCai, b: HeThongSoCai): boolean {
  return JSON.stringify([...a.cacTaiKhoan.entries()].sort()) === JSON.stringify([...b.cacTaiKhoan.entries()].sort());
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const r0 = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
const r1 = taoReplica(1, 3, [NGUON_NGOAI, TK1, TK2]);
const r2 = taoReplica(2, 3, [NGUON_NGOAI, TK1, TK2]);
const diskR1 = new SimDisk(KICH_THUOC_BAN_GHI_DIA);
const diskR2 = new SimDisk(KICH_THUOC_BAN_GHI_DIA);

chayMotVongVaGhiDia(r0, [{ r: r1, disk: diskR1 }, { r: r2, disk: diskR2 }], { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
chayMotVongVaGhiDia(r0, [{ r: r1, disk: diskR1 }, { r: r2, disk: diskR2 }], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 });

// dia cua r2 bi crash DUNG luc ghi op3 -- op3 van vao bo nho (r2.log/soCai) nhung KHONG ben tren dia
diskR2.datCrashSauThaoTacThuN(1);
chayMotVongVaGhiDia(r0, [{ r: r1, disk: diskR1 }, { r: r2, disk: diskR2 }], { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 });

console.log("primary -- soDu TK1:", soDuSoSach(r0.soCai.cacTaiKhoan.get(TK1)!), "soDu TK2:", soDuSoSach(r0.soCai.cacTaiKhoan.get(TK2)!));
console.log("dia r2 doc lai duoc:", docLogTuDia(diskR2, 5).length, "entry (log bo nho cu co", r2.log.length, ")");

// r2 "chet" va khoi dong lai -- MAT toan bo bo nho, CHI con diskR2
const r2Moi = phucHoiVaBatKip(diskR2, 2, 3, [NGUON_NGOAI, TK1, TK2], r0);
console.log("r2Moi sau phuc hoi -- opNumber:", r2Moi.opNumber, "commitNumber:", r2Moi.commitNumber);
console.log("r2Moi -- soDu TK1:", soDuSoSach(r2Moi.soCai.cacTaiKhoan.get(TK1)!), "soDu TK2:", soDuSoSach(r2Moi.soCai.cacTaiKhoan.get(TK2)!));
console.log("hoi tu voi primary:", xacNhanHoiTu(r2Moi.soCai, r0.soCai));
```

```text title=readonly
primary -- soDu TK1: 700 soDu TK2: 300
dia r2 doc lai duoc: 2 entry (log bo nho cu co 3 )
r2Moi sau phuc hoi -- opNumber: 3 commitNumber: 3
r2Moi -- soDu TK1: 700 soDu TK2: 300
hoi tu voi primary: true
```

Ba bút toán qua `r0` (primary, KHÔNG hề crash) cho `soDu TK1=700`,
`TK2=300`. Đĩa của `r2` LẼ ra CÓ `3` entry — nhưng `datCrashSauThaoTacThuN(1)`
cắt NGANG đúng lần ghi CUỐI (op3): đĩa CHỈ còn `2`. `r2` (TRƯỚC khi
"chết") có `log.length=3` TRONG bộ nhớ — nhưng bộ nhớ ĐÓ SẼ mất SẠCH
khi restart. `phucHoiVaBatKip` đọc LẠI đúng `2` entry TỪ đĩa, replay
vào `soCai` (`opNumber=2`), RỒI phát hiện `kiemTraCanDongBo` báo LẠC
hậu (`2 < 3`) — State Transfer (q19 bài 13) bù ĐÚNG entry còn thiếu
(`op3`) TỪ `primary.log`. `r2Moi` cuối CÙNG đạt `opNumber=commitNumber=3`,
hội tụ ĐÚNG với primary — KHÔNG hề có view change NÀO (`primary` VẪN
LÀ primary suốt).
::::

::::example{#khac-han-view-change}
Đây LÀ tình huống HOÀN toàn khác `view change` (bài 6-7 sắp tới):
`primary` KHÔNG hề chết, KHÔNG hề có cuộc BẦU cử nào — chỉ MỘT backup
tự khởi động lại, tự đọc đĩa CỦA chính nó, RỒI hỏi thêm đúng phần
thiếu. Đây chính LÀ lý do MASTERPLAN §9.2 (q19) tách "State Transfer"
RA khỏi "view change": chúng chữa hai LOẠI sự cố khác hẳn nhau — MỘT
cái LÀ "lãnh đạo biến mất", cái KIA LÀ "một thành VIÊN lỡ nhịp".
::::

::::predict{#doan-dia-khong-loi-gi commitOnce}
Gọi `phucHoiVaBatKip(diskR1, 1, 3, [...], r0)` — `diskR1` CHƯA từng
gặp lỗi NÀO (đủ `3` entry). `kiemTraCanDongBo` (bên TRONG hàm) trả về
GÌ ngay SAU bước đọc đĩa?
:::opt{correct}
`false` — SAU khi đọc `3` entry TỪ `diskR1`, `replicaMoi.opNumber`
ĐÃ LÀ `3`, BẰNG đúng `primary.opNumber` (`3`) — điều kiện
`backup.opNumber < opNumberMoiNhatDaBiet` (`3 < 3`) SAI, KHÔNG cần
State Transfer THÊM gì cả
:::
:::opt
`true` — MỌI replica MỚI khởi tạo (từ `taoReplica`) đều bắt đầu Ở
`opNumber=0`, nên LUÔN cần đồng bộ, bất kể đĩa ĐẦY đủ ra sao
::why
Trực giác NÀY dừng lại Ở dòng ĐẦU của `phucHoiVaBatKip`
(`taoReplica`, `opNumber=0`) — nhưng bỏ QUA vòng lặp NGAY SAU đó, vốn
CẬP nhật `replicaMoi.opNumber` theo TỪNG entry đọc được TỪ đĩa.

Chỗ lệch: `kiemTraCanDongBo` được GỌI SAU khi vòng lặp đọc đĩa ĐÃ
chạy xong — LÚC đó `replicaMoi.opNumber` không CÒN LÀ `0` nữa, mà LÀ
`opNumber` của entry CUỐI đọc được TỪ đĩa. Với `diskR1` (đủ CẢ `3`
entry), `opNumber` ĐÃ bắt kịp primary TRƯỚC cả khi State Transfer
được xét TỚI.
::
:::
::::

::::code{#viet_phuc_hoi_va_bat_kip}
Hoàn thiện `phucHoiVaBatKip` — VỚI mỗi `op` còn CHƯA commit (từ
`replicaMoi.commitNumber + 1` tới `primary.commitNumber`), tìm entry
tương ứng TRONG `replicaMoi.log` (đã đủ SAU State Transfer) và áp
dụng THẬT vào `replicaMoi.soCai`.

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

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }

interface HeThongSoCai {
  cacTaiKhoan: Map<number, TaiKhoan>;
  cacPendingDangCho: Map<number, ButToan>;
  dsIdDaXuLy: Set<number>;
}
function taoHeThongSoCai(cacId: number[]): HeThongSoCai {
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of cacId) cacTaiKhoan.set(id, taoTaiKhoan(id));
  return { cacTaiKhoan, cacPendingDangCho: new Map(), dsIdDaXuLy: new Set() };
}
function heThongCanBang(soCai: HeThongSoCai): boolean {
  let no = 0, co = 0;
  for (const [, tk] of soCai.cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function apDungQuaSoCai(soCai: HeThongSoCai, bt: ButToan): boolean {
  if (soCai.dsIdDaXuLy.has(bt.id)) return true;
  soCai.dsIdDaXuLy.add(bt.id);
  soCai.cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  soCai.cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
  return true;
}

interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
  soCai: HeThongSoCai;
}
function taoReplica(chiSo: number, tongSo: number, cacIdTaiKhoan: number[]): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0, soCai: taoHeThongSoCai(cacIdTaiKhoan) };
}
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }

interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  primary.opNumber += 1;
  const entry: LogEntry = { opNumber: primary.opNumber, bt };
  primary.log.push(entry);
  return entry;
}

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

function xuLyPrepareVaGhiDia(backup: Replica, disk: SimDisk, td: ThongDiep): ThongDiep {
  const entry: LogEntry = { opNumber: td.opNumber!, bt: td.bt! };
  backup.log.push(entry);
  backup.opNumber = td.opNumber!;
  ghiEntryXuongDia(disk, entry);
  return { loai: 'PrepareOk', tu: backup.chiSo, den: td.tu, viewNumber: backup.viewNumber, opNumber: td.opNumber! };
}
function nhanPrepareOk(primary: Replica, phieuTheoOp: Map<number, Set<number>>, td: ThongDiep): boolean {
  const opNumber = td.opNumber!;
  let phieu = phieuTheoOp.get(opNumber);
  if (!phieu) { phieu = new Set<number>([primary.chiSo]); phieuTheoOp.set(opNumber, phieu); }
  phieu.add(td.tu);
  if (phieu.size < nguongQuorum(primary.tongSo)) return false;
  if (opNumber !== primary.commitNumber + 1) return false;
  const entry = primary.log.find((e) => e.opNumber === opNumber)!;
  apDungQuaSoCai(primary.soCai, entry.bt);
  primary.commitNumber = opNumber;
  return true;
}
function xuLyCommit(backup: Replica, td: ThongDiep): void {
  const commitMoi = td.commitNumber!;
  for (let op = backup.commitNumber + 1; op <= commitMoi; op++) {
    const entry = backup.log.find((e) => e.opNumber === op)!;
    apDungQuaSoCai(backup.soCai, entry.bt);
  }
  backup.commitNumber = commitMoi;
}
function kiemTraCanDongBo(backup: Replica, opNumberMoiNhatDaBiet: number): boolean {
  return backup.opNumber < opNumberMoiNhatDaBiet;
}
function xuLyPhanHoiDongBo(backup: Replica, td: ThongDiep): void {
  const entryConThieu = td.log!;
  for (const entry of entryConThieu) if (entry.opNumber > backup.opNumber) backup.log.push(entry);
  backup.opNumber = backup.log.length === 0 ? 0 : backup.log[backup.log.length - 1]!.opNumber;
}

function chayMotVongVaGhiDia(primary: Replica, backups: { r: Replica; disk: SimDisk }[], bt: ButToan): void {
  const entry = nhanYeuCauTuClient(primary, bt);
  const phieuTheoOp = new Map<number, Set<number>>();
  for (const b of backups) {
    const phanHoi = xuLyPrepareVaGhiDia(b.r, b.disk, { loai: 'Prepare', tu: primary.chiSo, den: b.r.chiSo, viewNumber: primary.viewNumber, opNumber: entry.opNumber, bt: entry.bt });
    nhanPrepareOk(primary, phieuTheoOp, phanHoi);
  }
  for (const b of backups) {
    xuLyCommit(b.r, { loai: 'Commit', tu: primary.chiSo, den: b.r.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
  }
}

function phucHoiVaBatKip(diskCuaNo: SimDisk, chiSo: number, tongSo: number, cacIdTaiKhoan: number[], primary: Replica): Replica {
  const replicaMoi = taoReplica(chiSo, tongSo, cacIdTaiKhoan);
  const logTuDia = docLogTuDia(diskCuaNo, primary.opNumber);
  for (const entry of logTuDia) {
    replicaMoi.log.push(entry);
    replicaMoi.opNumber = entry.opNumber;
    apDungQuaSoCai(replicaMoi.soCai, entry.bt);
  }
  replicaMoi.commitNumber = replicaMoi.opNumber;

  if (kiemTraCanDongBo(replicaMoi, primary.opNumber)) {
    const conThieu = primary.log.filter((e) => e.opNumber > replicaMoi.opNumber);
    xuLyPhanHoiDongBo(replicaMoi, { loai: 'PhanHoiDongBo', tu: primary.chiSo, den: replicaMoi.chiSo, viewNumber: primary.viewNumber, log: conThieu });
  }
  for (let op = replicaMoi.commitNumber + 1; op <= primary.commitNumber; op++) {
    ___
  }
  replicaMoi.commitNumber = primary.commitNumber;
  return replicaMoi;
}

function xacNhanHoiTu(a: HeThongSoCai, b: HeThongSoCai): boolean {
  return JSON.stringify([...a.cacTaiKhoan.entries()].sort()) === JSON.stringify([...b.cacTaiKhoan.entries()].sort());
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const r0 = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
const r1 = taoReplica(1, 3, [NGUON_NGOAI, TK1, TK2]);
const r2 = taoReplica(2, 3, [NGUON_NGOAI, TK1, TK2]);
const diskR1 = new SimDisk(KICH_THUOC_BAN_GHI_DIA);
const diskR2 = new SimDisk(KICH_THUOC_BAN_GHI_DIA);
chayMotVongVaGhiDia(r0, [{ r: r1, disk: diskR1 }, { r: r2, disk: diskR2 }], { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
chayMotVongVaGhiDia(r0, [{ r: r1, disk: diskR1 }, { r: r2, disk: diskR2 }], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 });
diskR2.datCrashSauThaoTacThuN(1);
chayMotVongVaGhiDia(r0, [{ r: r1, disk: diskR1 }, { r: r2, disk: diskR2 }], { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 });

const r2Moi = phucHoiVaBatKip(diskR2, 2, 3, [NGUON_NGOAI, TK1, TK2], r0);
console.log(soDuSoSach(r2Moi.soCai.cacTaiKhoan.get(TK1)!));
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

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }

interface HeThongSoCai {
  cacTaiKhoan: Map<number, TaiKhoan>;
  cacPendingDangCho: Map<number, ButToan>;
  dsIdDaXuLy: Set<number>;
}
function taoHeThongSoCai(cacId: number[]): HeThongSoCai {
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of cacId) cacTaiKhoan.set(id, taoTaiKhoan(id));
  return { cacTaiKhoan, cacPendingDangCho: new Map(), dsIdDaXuLy: new Set() };
}
function heThongCanBang(soCai: HeThongSoCai): boolean {
  let no = 0, co = 0;
  for (const [, tk] of soCai.cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function apDungQuaSoCai(soCai: HeThongSoCai, bt: ButToan): boolean {
  if (soCai.dsIdDaXuLy.has(bt.id)) return true;
  soCai.dsIdDaXuLy.add(bt.id);
  soCai.cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  soCai.cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
  return true;
}

interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
  soCai: HeThongSoCai;
}
function taoReplica(chiSo: number, tongSo: number, cacIdTaiKhoan: number[]): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0, soCai: taoHeThongSoCai(cacIdTaiKhoan) };
}
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }

interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  primary.opNumber += 1;
  const entry: LogEntry = { opNumber: primary.opNumber, bt };
  primary.log.push(entry);
  return entry;
}

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

function xuLyPrepareVaGhiDia(backup: Replica, disk: SimDisk, td: ThongDiep): ThongDiep {
  const entry: LogEntry = { opNumber: td.opNumber!, bt: td.bt! };
  backup.log.push(entry);
  backup.opNumber = td.opNumber!;
  ghiEntryXuongDia(disk, entry);
  return { loai: 'PrepareOk', tu: backup.chiSo, den: td.tu, viewNumber: backup.viewNumber, opNumber: td.opNumber! };
}
function nhanPrepareOk(primary: Replica, phieuTheoOp: Map<number, Set<number>>, td: ThongDiep): boolean {
  const opNumber = td.opNumber!;
  let phieu = phieuTheoOp.get(opNumber);
  if (!phieu) { phieu = new Set<number>([primary.chiSo]); phieuTheoOp.set(opNumber, phieu); }
  phieu.add(td.tu);
  if (phieu.size < nguongQuorum(primary.tongSo)) return false;
  if (opNumber !== primary.commitNumber + 1) return false;
  const entry = primary.log.find((e) => e.opNumber === opNumber)!;
  apDungQuaSoCai(primary.soCai, entry.bt);
  primary.commitNumber = opNumber;
  return true;
}
function xuLyCommit(backup: Replica, td: ThongDiep): void {
  const commitMoi = td.commitNumber!;
  for (let op = backup.commitNumber + 1; op <= commitMoi; op++) {
    const entry = backup.log.find((e) => e.opNumber === op)!;
    apDungQuaSoCai(backup.soCai, entry.bt);
  }
  backup.commitNumber = commitMoi;
}
function kiemTraCanDongBo(backup: Replica, opNumberMoiNhatDaBiet: number): boolean {
  return backup.opNumber < opNumberMoiNhatDaBiet;
}
function xuLyPhanHoiDongBo(backup: Replica, td: ThongDiep): void {
  const entryConThieu = td.log!;
  for (const entry of entryConThieu) if (entry.opNumber > backup.opNumber) backup.log.push(entry);
  backup.opNumber = backup.log.length === 0 ? 0 : backup.log[backup.log.length - 1]!.opNumber;
}

function chayMotVongVaGhiDia(primary: Replica, backups: { r: Replica; disk: SimDisk }[], bt: ButToan): void {
  const entry = nhanYeuCauTuClient(primary, bt);
  const phieuTheoOp = new Map<number, Set<number>>();
  for (const b of backups) {
    const phanHoi = xuLyPrepareVaGhiDia(b.r, b.disk, { loai: 'Prepare', tu: primary.chiSo, den: b.r.chiSo, viewNumber: primary.viewNumber, opNumber: entry.opNumber, bt: entry.bt });
    nhanPrepareOk(primary, phieuTheoOp, phanHoi);
  }
  for (const b of backups) {
    xuLyCommit(b.r, { loai: 'Commit', tu: primary.chiSo, den: b.r.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
  }
}

function phucHoiVaBatKip(diskCuaNo: SimDisk, chiSo: number, tongSo: number, cacIdTaiKhoan: number[], primary: Replica): Replica {
  const replicaMoi = taoReplica(chiSo, tongSo, cacIdTaiKhoan);
  const logTuDia = docLogTuDia(diskCuaNo, primary.opNumber);
  for (const entry of logTuDia) {
    replicaMoi.log.push(entry);
    replicaMoi.opNumber = entry.opNumber;
    apDungQuaSoCai(replicaMoi.soCai, entry.bt);
  }
  replicaMoi.commitNumber = replicaMoi.opNumber;

  if (kiemTraCanDongBo(replicaMoi, primary.opNumber)) {
    const conThieu = primary.log.filter((e) => e.opNumber > replicaMoi.opNumber);
    xuLyPhanHoiDongBo(replicaMoi, { loai: 'PhanHoiDongBo', tu: primary.chiSo, den: replicaMoi.chiSo, viewNumber: primary.viewNumber, log: conThieu });
  }
  for (let op = replicaMoi.commitNumber + 1; op <= primary.commitNumber; op++) {
    const e = replicaMoi.log.find((x) => x.opNumber === op)!;
    apDungQuaSoCai(replicaMoi.soCai, e.bt);
  }
  replicaMoi.commitNumber = primary.commitNumber;
  return replicaMoi;
}

function xacNhanHoiTu(a: HeThongSoCai, b: HeThongSoCai): boolean {
  return JSON.stringify([...a.cacTaiKhoan.entries()].sort()) === JSON.stringify([...b.cacTaiKhoan.entries()].sort());
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const r0 = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
const r1 = taoReplica(1, 3, [NGUON_NGOAI, TK1, TK2]);
const r2 = taoReplica(2, 3, [NGUON_NGOAI, TK1, TK2]);
const diskR1 = new SimDisk(KICH_THUOC_BAN_GHI_DIA);
const diskR2 = new SimDisk(KICH_THUOC_BAN_GHI_DIA);
chayMotVongVaGhiDia(r0, [{ r: r1, disk: diskR1 }, { r: r2, disk: diskR2 }], { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
chayMotVongVaGhiDia(r0, [{ r: r1, disk: diskR1 }, { r: r2, disk: diskR2 }], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 });
diskR2.datCrashSauThaoTacThuN(1);
chayMotVongVaGhiDia(r0, [{ r: r1, disk: diskR1 }, { r: r2, disk: diskR2 }], { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 });

const r2Moi = phucHoiVaBatKip(diskR2, 2, 3, [NGUON_NGOAI, TK1, TK2], r0);
console.log(soDuSoSach(r2Moi.soCai.cacTaiKhoan.get(TK1)!));
```

```typescript title=test
if (soDuSoSach(r0.soCai.cacTaiKhoan.get(TK1)!) !== 700) throw new Error("primary phai co soDu TK1 = 700");
if (soDuSoSach(r0.soCai.cacTaiKhoan.get(TK2)!) !== 300) throw new Error("primary phai co soDu TK2 = 300");
if (docLogTuDia(diskR2, 5).length !== 2) throw new Error("dia cua r2 phai CHI con 2 entry (op3 mat do crash)");
if (r2.log.length !== 3) throw new Error("bo nho cua r2 (TRUOC khi 'chet') phai co du 3 entry");

if (r2Moi.opNumber !== 3) throw new Error("sau phuc hoi (doc dia + state transfer), opNumber phai la 3 (bat kip primary)");
if (r2Moi.commitNumber !== 3) throw new Error("sau phuc hoi, commitNumber phai la 3");
if (r2Moi.log.length !== 3) throw new Error("sau phuc hoi, log phai co du 3 entry");
if (soDuSoSach(r2Moi.soCai.cacTaiKhoan.get(TK1)!) !== 700) throw new Error("sau phuc hoi, soDu TK1 phai la 700 (giong primary)");
if (soDuSoSach(r2Moi.soCai.cacTaiKhoan.get(TK2)!) !== 300) throw new Error("sau phuc hoi, soDu TK2 phai la 300");
if (!xacNhanHoiTu(r2Moi.soCai, r0.soCai)) throw new Error("sau phuc hoi, r2Moi phai HOI TU voi primary");
if (!heThongCanBang(r2Moi.soCai)) throw new Error("sau phuc hoi, r2Moi phai can bang");

const r1Moi = phucHoiVaBatKip(diskR1, 1, 3, [NGUON_NGOAI, TK1, TK2], r0);
if (r1Moi.opNumber !== 3 || r1Moi.commitNumber !== 3) throw new Error("r1 (dia khong loi) phuc hoi van phai dat opNumber=commitNumber=3");
if (!xacNhanHoiTu(r1Moi.soCai, r0.soCai)) throw new Error("r1Moi phai hoi tu voi primary");
```

:::hints
- kind: attention
  body: "Tim entry theo opNumber trong replicaMoi.log, ap dung THAT vao replicaMoi.soCai -- hai dong trong vong for."
- kind: strategy
  body: "const e = replicaMoi.log.find((x) => x.opNumber === op)!; apDungQuaSoCai(replicaMoi.soCai, e.bt);"
- kind: one-line
  body: "const e = replicaMoi.log.find((x) => x.opNumber === op)!; apDungQuaSoCai(replicaMoi.soCai, e.bt);"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "700"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một replica sống lại đúng, hội tụ đúng, primary chưa hề hay biết gì.
Nhưng nếu chính PRIMARY chết — giữa lúc nhiều bút toán đang bay?
::::

::::reflect{#nghi-lai}
`phucHoiVaBatKip` không giới thiệu khái niệm MỚI — nó XẾP đúng thứ tự
BA thứ đã học: đọc đĩa (bài 3), replay VÀO sổ cái (tinh thần q16 bài
6), VÀ State Transfer (q19 bài 13) cho ĐÚNG phần đĩa không kịp ghi.
Bài học lớn NHẤT: "phục hồi" KHÔNG phải một phép MÀU — nó LÀ phép
CỘNG của những gì ĐÃ bền (đĩa) VÀ những gì một PEER còn sống có thể
xác NHẬN (mạng), không hơn không kém.
::::

::::checkpoint{mastery=0.9}
::::
