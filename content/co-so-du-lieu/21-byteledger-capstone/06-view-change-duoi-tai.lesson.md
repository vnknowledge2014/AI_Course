---
id: co-so-du-lieu.byteledger-capstone.view-change-duoi-tai
title: "View change dưới tải"
summary: "Op2 (chuyển 300) gửi Prepare CHỈ tới r1 (r2 không bao giờ nhận) rồi r0 'chết' TRƯỚC khi kịp xử lý PrepareOk của r1 -- op2 CHƯA commit (r0.commitNumber=1, chỉ op1). View change lên view=1: chonLogDayDuNhat chọn log của r1 (2 entry, đầy đủ hơn r2's 1 entry) -- op2 SỐNG SÓT qua view change (vẫn trong log hợp nhất) nhưng commitNumber vẫn là 1 (KHÔNG tự động commit qua view change). reDriveEntryChuaCommit tái đấu op2 qua primary MỚI (r1, view 1): đạt quorum (2, N=3), commit THẬT -- soDu TK1=700, TK2=300. Gọi lại reDriveEntryChuaCommit LẦN NỮA (mô phỏng PrepareOk trễ) không đổi gì thêm -- soDu vẫn 700, không double-apply."
locale: vi
track: co-so-du-lieu
module: byteledger-capstone
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [db.view-change-duoi-tai]
requires: [db.crash-va-phuc-hoi-mot-replica]
concepts: [db.view-change-duoi-tai]
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
Bài trước: MỘT backup (không phải primary) chết rồi hồi phục — êm ả,
primary không hề hay biết. Câu hỏi khó HƠN nhiều: nếu CHÍNH primary
chết TRONG lúc một bút toán đang "bay giữa đường" — CHƯA ai xác nhận
xong — bút toán đó có SỐNG SÓT không?
::::

::::explain{#op-dang-bay-khi-primary-chet}
Op2 (chuyển `300`) gửi Prepare CHỈ tới `r1` (`r2` không BAO giờ nhận
— mô phỏng mất gói CÓ chủ đích, không phải seed) — `r0` "chết" TRƯỚC
khi kịp xử lý `PrepareOk` phản HỒI từ `r1`. Op2 CHƯA commit
(`r0.commitNumber=1`, CHỈ op1). View change (view=1): `chonLogDayDuNhat`
(q19 bài 11) chọn log của `r1` (`2` entry — đầy đủ HƠN `r2`'s `1`
entry). `reDriveEntryChuaCommit` tái ĐẤU op2 qua primary MỚI:

```typescript title=readonly
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
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'StartView';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  primary.opNumber += 1;
  const entry: LogEntry = { opNumber: primary.opNumber, bt };
  primary.log.push(entry);
  return entry;
}
function xuLyPrepare(backup: Replica, td: ThongDiep): ThongDiep {
  const entry: LogEntry = { opNumber: td.opNumber!, bt: td.bt! };
  backup.log.push(entry);
  backup.opNumber = td.opNumber!;
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
function phatHienPrimaryChet(replica: Replica, viewMoi: number): void {
  replica.status = 'view-change';
  replica.viewNumber = viewMoi;
}
function opCuaLog(log: LogEntry[]): number { return log.length === 0 ? 0 : log[log.length - 1]!.opNumber; }
function chonLogDayDuNhat(cacLog: LogEntry[][]): LogEntry[] {
  let logTotNhat = cacLog[0]!;
  for (let i = 1; i < cacLog.length; i++) {
    if (opCuaLog(cacLog[i]!) > opCuaLog(logTotNhat)) logTotNhat = cacLog[i]!;
  }
  return logTotNhat;
}
function xuLyStartView(replica: Replica, td: ThongDiep): void {
  replica.log = [...td.log!]; // ban sao rieng -- moi replica giu bo nho doc lap
  replica.opNumber = opCuaLog(replica.log);
  replica.viewNumber = td.viewNumber;
  replica.commitNumber = td.commitNumber ?? replica.commitNumber;
  replica.status = 'normal';
}

// tai-dau MOT entry CHUA COMMIT sau view change (khong tao opNumber moi -- dung lai entry san co)
function reDriveEntryChuaCommit(primary: Replica, cacBackupConSong: Replica[], entry: LogEntry): boolean {
  const phieuTheoOp = new Map<number, Set<number>>();
  for (const backup of cacBackupConSong) {
    if (backup.opNumber < entry.opNumber) {
      backup.log.push(entry);
      backup.opNumber = entry.opNumber;
    }
    nhanPrepareOk(primary, phieuTheoOp, { loai: 'PrepareOk', tu: backup.chiSo, den: primary.chiSo, viewNumber: backup.viewNumber, opNumber: entry.opNumber });
  }
  if (primary.commitNumber === entry.opNumber) {
    for (const backup of cacBackupConSong) {
      xuLyCommit(backup, { loai: 'Commit', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
    }
  }
  return primary.commitNumber === entry.opNumber;
}
function xacNhanHoiTu(a: HeThongSoCai, b: HeThongSoCai): boolean {
  return JSON.stringify([...a.cacTaiKhoan.entries()].sort()) === JSON.stringify([...b.cacTaiKhoan.entries()].sort());
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const r0 = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
const r1 = taoReplica(1, 3, [NGUON_NGOAI, TK1, TK2]);
const r2 = taoReplica(2, 3, [NGUON_NGOAI, TK1, TK2]);

// op1: qua het, commit binh thuong
{
  const entry = nhanYeuCauTuClient(r0, { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
  const phieu = new Map<number, Set<number>>();
  for (const b of [r1, r2]) {
    const phanHoi = xuLyPrepare(b, { loai: 'Prepare', tu: r0.chiSo, den: b.chiSo, viewNumber: r0.viewNumber, opNumber: entry.opNumber, bt: entry.bt });
    nhanPrepareOk(r0, phieu, phanHoi);
  }
  for (const b of [r1, r2]) xuLyCommit(b, { loai: 'Commit', tu: r0.chiSo, den: b.chiSo, viewNumber: r0.viewNumber, commitNumber: r0.commitNumber });
}

// op2: Prepare CHI toi r1 -- r0 "chet" TRUOC khi kip xu ly PrepareOk cua r1
const entry2 = nhanYeuCauTuClient(r0, { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 300 });
xuLyPrepare(r1, { loai: 'Prepare', tu: r0.chiSo, den: r1.chiSo, viewNumber: r0.viewNumber, opNumber: entry2.opNumber, bt: entry2.bt });
console.log("truoc view change -- r0.commitNumber:", r0.commitNumber, "r1.log.length:", r1.log.length, "r2.log.length:", r2.log.length);

// view change len view=1
phatHienPrimaryChet(r1, 1);
phatHienPrimaryChet(r2, 1);
const conSong = [r1, r2];
const logDayDuNhat = chonLogDayDuNhat(conSong.map((r) => r.log));
const commitNumberMoi = Math.max(...conSong.map((r) => r.commitNumber));
const primaryMoi = conSong.find((r) => r.chiSo === 1 % 3)!;
for (const r of conSong) xuLyStartView(r, { loai: 'StartView', tu: primaryMoi.chiSo, den: r.chiSo, viewNumber: 1, log: logDayDuNhat, commitNumber: commitNumberMoi });
console.log("sau StartView -- primaryMoi.chiSo:", primaryMoi.chiSo, "commitNumber:", primaryMoi.commitNumber, "op2 con trong log:", primaryMoi.log.some((e) => e.opNumber === 2));

// op2 tai-dau qua primary MOI
const backupConLai = conSong.filter((r) => r.chiSo !== primaryMoi.chiSo);
const entryOp2 = primaryMoi.log.find((e) => e.opNumber === 2)!;
const daCommitOp2 = reDriveEntryChuaCommit(primaryMoi, backupConLai, entryOp2);
console.log("op2 commit sau khi re-drive:", daCommitOp2, "soDu TK1:", soDuSoSach(primaryMoi.soCai.cacTaiKhoan.get(TK1)!));
```

```text title=readonly
truoc view change -- r0.commitNumber: 1 r1.log.length: 2 r2.log.length: 1
sau StartView -- primaryMoi.chiSo: 1 commitNumber: 1 op2 con trong log: true
op2 commit sau khi re-drive: true soDu TK1: 700
```

`r0.commitNumber=1` — CHỈ op1 commit TRƯỚC crash. `r1` (đã nhận
Prepare) có `2` entry, `r2` (chưa từng nhận) chỉ `1`. SAU `StartView`,
primary MỚI (`chiSo=1`, VÌ `1 % 3 = 1`) có `commitNumber` VẪN LÀ `1`
— view change KHÔNG tự Ý commit BẤT cứ gì — nhưng op2 VẪN CÒN trong
log (`true`), KHÔNG hề "biến mất". `reDriveEntryChuaCommit` tái đấu
op2 (đạt quorum VỚI đúng MỘT backup còn LẠI + tự-vote CỦA primary
MỚI = `2`, ĐỦ `nguongQuorum(3)=2`) — commit THẬT, `soDu TK1` giảm
ĐÚNG `300`, THÀNH `700`.
::::

::::example{#khong-mat-khong-nhan-doi}
Đây LÀ đúng hai đảm bảo bài NÀY chứng minh: (1) "KHÔNG mất" — op2
sống SÓT nguyên vẹn qua view change VÌ `chonLogDayDuNhat` LUÔN chọn
log ĐẦY đủ nhất (q19 bài 11); (2) "KHÔNG double-apply" — `apDungQuaSoCai`
(bài 2) chỉ áp DỤNG op2 đúng MỘT lần, DÙ nó tồn tại TRONG log của cả
hai replica TRƯỚC lẫn SAU view change, VÀ dù `reDriveEntryChuaCommit`
CÓ thể bị gọi LẶP lại (giả lập `PrepareOk` đến trễ) — `dsIdDaXuLy`
chặn NGAY hiệu ứng thừa, `opNumber !== commitNumber+1` (q19 bài 6)
chặn NGAY việc commit lặp.
::::

::::predict{#doan-goi-lai-re-drive commitOnce}
SAU khi op2 ĐÃ commit thành công (`soDu TK1=700`), gọi LẠI
`reDriveEntryChuaCommit(primaryMoi, backupConLai, entryOp2)` MỘT lần
NỮA (mô phỏng `PrepareOk` cũ đến TRỄ, tới sau khi mọi thứ ĐÃ xong).
`soDu TK1` SAU lần gọi đó LÀ bao nhiêu?
:::opt{correct}
`700` — VẪN giữ nguyên; `nhanPrepareOk` kiểm tra `opNumber !==
primary.commitNumber + 1` (`2 !== 2+1=3`), từ CHỐI ngay, KHÔNG áp
dụng LẠI bút toán
:::
:::opt
`400` — trừ THÊM `300` nữa, VÌ backup gửi lại phiếu VẪN được tính LÀ
"một xác nhận MỚI"
::why
Trực giác NÀY tưởng tượng MỖI `PrepareOk` LÀ một "sự kiện MỚI" độc
lập — nhưng `nhanPrepareOk` (q19 bài 6, tái sử DỤNG nguyên xi Ở
`reDriveEntryChuaCommit`) kiểm TRA thứ tự TRƯỚC khi áp dụng BẤT cứ
gì.

Chỗ lệch: `primary.commitNumber` ĐÃ LÀ `2` (op2 commit RỒI) — điều
kiện `opNumber !== primary.commitNumber + 1` (`2 !== 3`) chặn NGAY,
hàm trả VỀ `false` mà không hề chạm TỚI `apDungQuaSoCai`. Đây chính
LÀ lớp bảo vệ q19 bài 6 đã xây — Ở ĐÂY nó hoạt động XUYÊN suốt một
cuộc view change, không CHỈ trong đường THUẬN buồm.
::
:::
::::

::::code{#viet_re_drive_entry}
Hoàn thiện `reDriveEntryChuaCommit` — VỚI mỗi backup, SAU khi đảm bảo
nó có entry TRONG log, gọi `nhanPrepareOk` để gộp phiếu của backup
đó VÀO primary.

```typescript title=starter
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
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'StartView';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  primary.opNumber += 1;
  const entry: LogEntry = { opNumber: primary.opNumber, bt };
  primary.log.push(entry);
  return entry;
}
function xuLyPrepare(backup: Replica, td: ThongDiep): ThongDiep {
  const entry: LogEntry = { opNumber: td.opNumber!, bt: td.bt! };
  backup.log.push(entry);
  backup.opNumber = td.opNumber!;
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
function phatHienPrimaryChet(replica: Replica, viewMoi: number): void {
  replica.status = 'view-change';
  replica.viewNumber = viewMoi;
}
function opCuaLog(log: LogEntry[]): number { return log.length === 0 ? 0 : log[log.length - 1]!.opNumber; }
function chonLogDayDuNhat(cacLog: LogEntry[][]): LogEntry[] {
  let logTotNhat = cacLog[0]!;
  for (let i = 1; i < cacLog.length; i++) {
    if (opCuaLog(cacLog[i]!) > opCuaLog(logTotNhat)) logTotNhat = cacLog[i]!;
  }
  return logTotNhat;
}
function xuLyStartView(replica: Replica, td: ThongDiep): void {
  replica.log = [...td.log!]; // ban sao rieng -- moi replica giu bo nho doc lap
  replica.opNumber = opCuaLog(replica.log);
  replica.viewNumber = td.viewNumber;
  replica.commitNumber = td.commitNumber ?? replica.commitNumber;
  replica.status = 'normal';
}

function reDriveEntryChuaCommit(primary: Replica, cacBackupConSong: Replica[], entry: LogEntry): boolean {
  const phieuTheoOp = new Map<number, Set<number>>();
  for (const backup of cacBackupConSong) {
    if (backup.opNumber < entry.opNumber) {
      backup.log.push(entry);
      backup.opNumber = entry.opNumber;
    }
    ___
  }
  if (primary.commitNumber === entry.opNumber) {
    for (const backup of cacBackupConSong) {
      xuLyCommit(backup, { loai: 'Commit', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
    }
  }
  return primary.commitNumber === entry.opNumber;
}
function xacNhanHoiTu(a: HeThongSoCai, b: HeThongSoCai): boolean {
  return JSON.stringify([...a.cacTaiKhoan.entries()].sort()) === JSON.stringify([...b.cacTaiKhoan.entries()].sort());
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const r0 = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
const r1 = taoReplica(1, 3, [NGUON_NGOAI, TK1, TK2]);
const r2 = taoReplica(2, 3, [NGUON_NGOAI, TK1, TK2]);
{
  const entry = nhanYeuCauTuClient(r0, { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
  const phieu = new Map<number, Set<number>>();
  for (const b of [r1, r2]) {
    const phanHoi = xuLyPrepare(b, { loai: 'Prepare', tu: r0.chiSo, den: b.chiSo, viewNumber: r0.viewNumber, opNumber: entry.opNumber, bt: entry.bt });
    nhanPrepareOk(r0, phieu, phanHoi);
  }
  for (const b of [r1, r2]) xuLyCommit(b, { loai: 'Commit', tu: r0.chiSo, den: b.chiSo, viewNumber: r0.viewNumber, commitNumber: r0.commitNumber });
}
const entry2 = nhanYeuCauTuClient(r0, { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 300 });
xuLyPrepare(r1, { loai: 'Prepare', tu: r0.chiSo, den: r1.chiSo, viewNumber: r0.viewNumber, opNumber: entry2.opNumber, bt: entry2.bt });

phatHienPrimaryChet(r1, 1);
phatHienPrimaryChet(r2, 1);
const conSong = [r1, r2];
const logDayDuNhat = chonLogDayDuNhat(conSong.map((r) => r.log));
const commitNumberMoi = Math.max(...conSong.map((r) => r.commitNumber));
const primaryMoi = conSong.find((r) => r.chiSo === 1 % 3)!;
for (const r of conSong) xuLyStartView(r, { loai: 'StartView', tu: primaryMoi.chiSo, den: r.chiSo, viewNumber: 1, log: logDayDuNhat, commitNumber: commitNumberMoi });

const backupConLai = conSong.filter((r) => r.chiSo !== primaryMoi.chiSo);
const entryOp2 = primaryMoi.log.find((e) => e.opNumber === 2)!;
reDriveEntryChuaCommit(primaryMoi, backupConLai, entryOp2);
console.log(soDuSoSach(primaryMoi.soCai.cacTaiKhoan.get(TK1)!));
```

```typescript title=solution
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
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'StartView';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  primary.opNumber += 1;
  const entry: LogEntry = { opNumber: primary.opNumber, bt };
  primary.log.push(entry);
  return entry;
}
function xuLyPrepare(backup: Replica, td: ThongDiep): ThongDiep {
  const entry: LogEntry = { opNumber: td.opNumber!, bt: td.bt! };
  backup.log.push(entry);
  backup.opNumber = td.opNumber!;
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
function phatHienPrimaryChet(replica: Replica, viewMoi: number): void {
  replica.status = 'view-change';
  replica.viewNumber = viewMoi;
}
function opCuaLog(log: LogEntry[]): number { return log.length === 0 ? 0 : log[log.length - 1]!.opNumber; }
function chonLogDayDuNhat(cacLog: LogEntry[][]): LogEntry[] {
  let logTotNhat = cacLog[0]!;
  for (let i = 1; i < cacLog.length; i++) {
    if (opCuaLog(cacLog[i]!) > opCuaLog(logTotNhat)) logTotNhat = cacLog[i]!;
  }
  return logTotNhat;
}
function xuLyStartView(replica: Replica, td: ThongDiep): void {
  replica.log = [...td.log!]; // ban sao rieng -- moi replica giu bo nho doc lap
  replica.opNumber = opCuaLog(replica.log);
  replica.viewNumber = td.viewNumber;
  replica.commitNumber = td.commitNumber ?? replica.commitNumber;
  replica.status = 'normal';
}

function reDriveEntryChuaCommit(primary: Replica, cacBackupConSong: Replica[], entry: LogEntry): boolean {
  const phieuTheoOp = new Map<number, Set<number>>();
  for (const backup of cacBackupConSong) {
    if (backup.opNumber < entry.opNumber) {
      backup.log.push(entry);
      backup.opNumber = entry.opNumber;
    }
    nhanPrepareOk(primary, phieuTheoOp, { loai: 'PrepareOk', tu: backup.chiSo, den: primary.chiSo, viewNumber: backup.viewNumber, opNumber: entry.opNumber });
  }
  if (primary.commitNumber === entry.opNumber) {
    for (const backup of cacBackupConSong) {
      xuLyCommit(backup, { loai: 'Commit', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
    }
  }
  return primary.commitNumber === entry.opNumber;
}
function xacNhanHoiTu(a: HeThongSoCai, b: HeThongSoCai): boolean {
  return JSON.stringify([...a.cacTaiKhoan.entries()].sort()) === JSON.stringify([...b.cacTaiKhoan.entries()].sort());
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const r0 = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
const r1 = taoReplica(1, 3, [NGUON_NGOAI, TK1, TK2]);
const r2 = taoReplica(2, 3, [NGUON_NGOAI, TK1, TK2]);
{
  const entry = nhanYeuCauTuClient(r0, { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
  const phieu = new Map<number, Set<number>>();
  for (const b of [r1, r2]) {
    const phanHoi = xuLyPrepare(b, { loai: 'Prepare', tu: r0.chiSo, den: b.chiSo, viewNumber: r0.viewNumber, opNumber: entry.opNumber, bt: entry.bt });
    nhanPrepareOk(r0, phieu, phanHoi);
  }
  for (const b of [r1, r2]) xuLyCommit(b, { loai: 'Commit', tu: r0.chiSo, den: b.chiSo, viewNumber: r0.viewNumber, commitNumber: r0.commitNumber });
}
const entry2 = nhanYeuCauTuClient(r0, { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 300 });
xuLyPrepare(r1, { loai: 'Prepare', tu: r0.chiSo, den: r1.chiSo, viewNumber: r0.viewNumber, opNumber: entry2.opNumber, bt: entry2.bt });

phatHienPrimaryChet(r1, 1);
phatHienPrimaryChet(r2, 1);
const conSong = [r1, r2];
const logDayDuNhat = chonLogDayDuNhat(conSong.map((r) => r.log));
const commitNumberMoi = Math.max(...conSong.map((r) => r.commitNumber));
const primaryMoi = conSong.find((r) => r.chiSo === 1 % 3)!;
for (const r of conSong) xuLyStartView(r, { loai: 'StartView', tu: primaryMoi.chiSo, den: r.chiSo, viewNumber: 1, log: logDayDuNhat, commitNumber: commitNumberMoi });

const backupConLai = conSong.filter((r) => r.chiSo !== primaryMoi.chiSo);
const entryOp2 = primaryMoi.log.find((e) => e.opNumber === 2)!;
reDriveEntryChuaCommit(primaryMoi, backupConLai, entryOp2);
console.log(soDuSoSach(primaryMoi.soCai.cacTaiKhoan.get(TK1)!));
```

```typescript title=test
const NN2 = 0, T1 = 1, T2 = 2;
const p0 = taoReplica(0, 3, [NN2, T1, T2]);
const p1 = taoReplica(1, 3, [NN2, T1, T2]);
const p2 = taoReplica(2, 3, [NN2, T1, T2]);
{
  const entry = nhanYeuCauTuClient(p0, { id: 1, debitAccountId: NN2, creditAccountId: T1, amount: 1000 });
  const phieu = new Map<number, Set<number>>();
  for (const b of [p1, p2]) {
    const phanHoi = xuLyPrepare(b, { loai: 'Prepare', tu: p0.chiSo, den: b.chiSo, viewNumber: p0.viewNumber, opNumber: entry.opNumber, bt: entry.bt });
    nhanPrepareOk(p0, phieu, phanHoi);
  }
  for (const b of [p1, p2]) xuLyCommit(b, { loai: 'Commit', tu: p0.chiSo, den: b.chiSo, viewNumber: p0.viewNumber, commitNumber: p0.commitNumber });
}
const e2 = nhanYeuCauTuClient(p0, { id: 2, debitAccountId: T1, creditAccountId: T2, amount: 300 });
xuLyPrepare(p1, { loai: 'Prepare', tu: p0.chiSo, den: p1.chiSo, viewNumber: p0.viewNumber, opNumber: e2.opNumber, bt: e2.bt });

const p0CommitTruocVC = p0.commitNumber;
const p1LogLenTruocVC = p1.log.length;
const p2LogLenTruocVC = p2.log.length;
if (p0CommitTruocVC !== 1) throw new Error("truoc view change, commitNumber phai la 1 (chi op1)");
if (p1LogLenTruocVC !== 2) throw new Error("p1 phai co 2 entry (op1, op2)");
if (p2LogLenTruocVC !== 1) throw new Error("p2 chua tung nhan op2, chi co 1 entry");

phatHienPrimaryChet(p1, 1);
phatHienPrimaryChet(p2, 1);
const song = [p1, p2];
const logDDN = chonLogDayDuNhat(song.map((r) => r.log));
const cnMoi = Math.max(...song.map((r) => r.commitNumber));
const pMoi = song.find((r) => r.chiSo === 1 % 3)!;
for (const r of song) xuLyStartView(r, { loai: 'StartView', tu: pMoi.chiSo, den: r.chiSo, viewNumber: 1, log: logDDN, commitNumber: cnMoi });

const logLenSauSV = pMoi.log.length;
const co2SauSV = pMoi.log.some((e) => e.opNumber === 2);
const cnSauSV = pMoi.commitNumber;
if (logLenSauSV !== 2) throw new Error("sau StartView, log hop nhat phai co 2 entry");
if (!co2SauSV) throw new Error("op2 khong duoc bien mat qua view change");
if (cnSauSV !== 1) throw new Error("sau StartView, commitNumber phai VAN la 1 (view change khong tu dong commit)");

const bkConLai = song.filter((r) => r.chiSo !== pMoi.chiSo);
const eOp2 = pMoi.log.find((e) => e.opNumber === 2)!;
const daCommit = reDriveEntryChuaCommit(pMoi, bkConLai, eOp2);
if (daCommit !== true) throw new Error("re-drive op2 phai THANH CONG (dat quorum voi 1 backup con lai + tu-vote primary = 2 = quorum(3))");
if (soDuSoSach(pMoi.soCai.cacTaiKhoan.get(T1)!) !== 700) throw new Error("soDu T1 phai la 700 (1000-300)");
if (soDuSoSach(pMoi.soCai.cacTaiKhoan.get(T2)!) !== 300) throw new Error("soDu T2 phai la 300");
if (!heThongCanBang(pMoi.soCai)) throw new Error("he thong phai can bang sau khi op2 commit");
if (!xacNhanHoiTu(pMoi.soCai, bkConLai[0]!.soCai)) throw new Error("primary moi va backup con lai phai hoi tu");

reDriveEntryChuaCommit(pMoi, bkConLai, eOp2);
if (soDuSoSach(pMoi.soCai.cacTaiKhoan.get(T1)!) !== 700) throw new Error("goi lai re-drive LAN THU HAI (PrepareOk tre) KHONG duoc doi soDu them");
```

:::hints
- kind: attention
  body: "Goi nhanPrepareOk de gop phieu cua backup vao primary: tu=backup.chiSo, den=primary.chiSo, viewNumber=backup.viewNumber, opNumber=entry.opNumber -- mot loi goi."
- kind: strategy
  body: "nhanPrepareOk(primary, phieuTheoOp, { loai: 'PrepareOk', tu: backup.chiSo, den: primary.chiSo, viewNumber: backup.viewNumber, opNumber: entry.opNumber });"
- kind: one-line
  body: "nhanPrepareOk(primary, phieuTheoOp, { loai: 'PrepareOk', tu: backup.chiSo, den: primary.chiSo, viewNumber: backup.viewNumber, opNumber: entry.opNumber });"
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
Một bút toán đang bay sống sót nguyên vẹn qua view change, commit
đúng một lần. Nhưng nếu chính CLIENT (không biết view change đã xảy
ra) gửi lại bút toán ĐÓ — dsIdDaXuLy có còn bảo vệ đúng không?
::::

::::reflect{#nghi-lai}
`reDriveEntryChuaCommit` không giới thiệu khái niệm MỚI — nó gọi LẠI
đúng `nhanPrepareOk` (bài 2) VÀ `xuLyCommit` (bài 2), chỉ khác NGỮ
cảnh: entry ĐÃ tồn tại sẵn trong log (từ TRƯỚC view change), không
cần `nhanYeuCauTuClient` tạo MỚI. Bài học lớn nhất: "sống SÓT qua
view change" VÀ "commit đúng MỘT lần" LÀ hai bảo đảm TÁCH biệt —
`chonLogDayDuNhat` (q19 bài 11) lo cái ĐẦU, `apDungQuaSoCai`'s
`dsIdDaXuLy` (bài 2) VÀ `opNumber` (q19 bài 6) LO cái sau — VÀ không
cái nào phụ THUỘC cái kia để đúng.
::::

::::checkpoint{mastery=0.9}
::::
