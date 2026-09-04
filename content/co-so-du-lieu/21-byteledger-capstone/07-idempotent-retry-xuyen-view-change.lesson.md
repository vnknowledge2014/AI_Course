---
id: co-so-du-lieu.byteledger-capstone.idempotent-retry-xuyen-view-change
title: "Idempotent retry xuyên suốt view change"
summary: "Client gửi bút toán id=2 (chuyển 300) tới primary CŨ (r0) -- r0 chết trước khi commit, view change đưa r1 lên làm primary MỚI. Client, KHÔNG biết gì về view change, gửi LẠI đúng id=2 tới primary MỚI -- nhanYeuCauTuClient tạo một log entry HOÀN TOÀN MỚI (opNumber=3, log.length=3), vì opNumber=2 (bản gốc) chưa hề commit nên KHÔNG bị chặn ở tầng submission. commitTatCaConTonDong đưa CẢ opNumber=2 lẫn 3 qua consensus theo đúng thứ tự -- commitNumber=3, dsIdDaXuLy.size=2 (id=1 và id=2, KHÔNG tính hai lần), soDu TK1=700/TK2=300: khoản 300 chỉ áp dụng ĐÚNG MỘT LẦN dù đi qua HAI opNumber khác nhau và HAI primary khác nhau."
locale: vi
track: co-so-du-lieu
module: byteledger-capstone
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [db.idempotent-retry-xuyen-view-change]
requires: [db.view-change-duoi-tai]
concepts: [db.idempotent-retry-xuyen-view-change]
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
Bài trước: một bút toán "đang bay" sống sót nguyên vẹn qua view
change, commit đúng một lần — nhưng CHÍNH nội bộ hệ thống tự lo liệu
điều đó. Câu hỏi khó HƠN: nếu CLIENT (không biết gì về view change,
CHỈ biết nó chưa nhận được xác nhận) tự Ý gửi LẠI, dsIdDaXuLy CÓ còn
bảo vệ đúng KHÔNG?
::::

::::explain{#retry-tao-op-so-moi}
Client gửi bút toán `id=2` (chuyển `300`) TỚI primary CŨ (`r0`) — `r0`
chết TRƯỚC khi commit, view change đưa `r1` lên LÀM primary MỚI.
Client, KHÔNG hề biết GÌ về view change, gửi LẠI đúng `id=2` TỚI
primary MỚI — `nhanYeuCauTuClient` (bài 2) tạo MỘT log entry HOÀN
toàn MỚI (`opNumber=3`), VÌ `opNumber=2` (bản gốc) CHƯA hề commit nên
KHÔNG hề bị chặn Ở tầng SUBMISSION (chỉ `dsIdDaXuLy` Ở tầng APPLY mới
biết `id=2` đã từng xuất hiện):

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
  replica.log = [...td.log!]; // ban sao rieng -- moi replica giu bo nho doc lap (bai 1)
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

// gui HET cac entry CHUA commit, theo DUNG thu tu opNumber
function commitTatCaConTonDong(primary: Replica, cacBackupConSong: Replica[]): void {
  const conTonDong = primary.log.filter((e) => e.opNumber > primary.commitNumber).sort((a, b) => a.opNumber - b.opNumber);
  for (const entry of conTonDong) {
    reDriveEntryChuaCommit(primary, cacBackupConSong, entry);
  }
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const r0 = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
const r1 = taoReplica(1, 3, [NGUON_NGOAI, TK1, TK2]);
const r2 = taoReplica(2, 3, [NGUON_NGOAI, TK1, TK2]);

// op1: commit binh thuong
{
  const entry = nhanYeuCauTuClient(r0, { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
  const phieu = new Map<number, Set<number>>();
  for (const b of [r1, r2]) {
    const phanHoi = xuLyPrepare(b, { loai: 'Prepare', tu: r0.chiSo, den: b.chiSo, viewNumber: r0.viewNumber, opNumber: entry.opNumber, bt: entry.bt });
    nhanPrepareOk(r0, phieu, phanHoi);
  }
  for (const b of [r1, r2]) xuLyCommit(b, { loai: 'Commit', tu: r0.chiSo, den: b.chiSo, viewNumber: r0.viewNumber, commitNumber: r0.commitNumber });
}

// op2 (id=2, chuyen 300): Prepare CHI toi r1 -- r0 "chet" TRUOC khi commit
const btGoc: ButToan = { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 300 };
const entry2 = nhanYeuCauTuClient(r0, btGoc);
xuLyPrepare(r1, { loai: 'Prepare', tu: r0.chiSo, den: r1.chiSo, viewNumber: r0.viewNumber, opNumber: entry2.opNumber, bt: entry2.bt });

// view change len view=1
phatHienPrimaryChet(r1, 1);
phatHienPrimaryChet(r2, 1);
const conSong = [r1, r2];
const logDayDuNhat = chonLogDayDuNhat(conSong.map((r) => r.log));
const commitNumberMoi = Math.max(...conSong.map((r) => r.commitNumber));
const primaryMoi = conSong.find((r) => r.chiSo === 1 % 3)!;
for (const r of conSong) xuLyStartView(r, { loai: 'StartView', tu: primaryMoi.chiSo, den: r.chiSo, viewNumber: 1, log: logDayDuNhat, commitNumber: commitNumberMoi });
const backupConLai = conSong.filter((r) => r.chiSo !== primaryMoi.chiSo);

// client, KHONG biet gi ve view change, GUI LAI dung id=2 toi primary MOI
const entryGuiLai = nhanYeuCauTuClient(primaryMoi, btGoc);
console.log("entryGuiLai.opNumber:", entryGuiLai.opNumber, "(entry CU op2 van la opNumber=2)");

commitTatCaConTonDong(primaryMoi, backupConLai);

console.log("soDu TK1:", soDuSoSach(primaryMoi.soCai.cacTaiKhoan.get(TK1)!), "soDu TK2:", soDuSoSach(primaryMoi.soCai.cacTaiKhoan.get(TK2)!));
console.log("commitNumber:", primaryMoi.commitNumber, "dsIdDaXuLy.size:", primaryMoi.soCai.dsIdDaXuLy.size);
console.log("hoi tu:", xacNhanHoiTu(primaryMoi.soCai, backupConLai[0]!.soCai));
```

```text title=readonly
entryGuiLai.opNumber: 3 (entry CU op2 van la opNumber=2)
soDu TK1: 700 soDu TK2: 300
commitNumber: 3 dsIdDaXuLy.size: 2
hoi tu: true
```

Gửi LẠI `id=2` TỚI primary MỚI TẠO đúng entry `opNumber=3` — primary
`log` giờ CÓ `2` entry MANG cùng `bt.id` (`opNumber=2` VÀ `opNumber=3`,
CẢ hai đều LÀ "chuyển 300"). `commitTatCaConTonDong` đưa CẢ hai qua
consensus THEO đúng thứ tự (`opNumber=2` TRƯỚC, `3` SAU — bắt buộc BỞI
`nhanPrepareOk`'s `opNumber !== commitNumber+1`, q19 bài 6): `commitNumber`
đạt `3`, NHƯNG `dsIdDaXuLy.size` CHỈ LÀ `2` (`id=1` VÀ `id=2` — KHÔNG
tính hai lần) — `soDu TK1=700`, khoản `300` CHỈ áp dụng ĐÚNG một lần,
DÙ nó đi qua HAI `opNumber` khác nhau VÀ hai PRIMARY khác nhau (`r0`
RỒI `r1`).
::::

::::example{#hai-tang-bao-ve-doc-lap}
Đây LÀ minh chứng RÕ nhất cho lý do BÀI 2 đặt kiểm tra `dsIdDaXuLy`
Ở TẦNG apply (`apDungQuaSoCai`), KHÔNG phải Ở tầng SUBMISSION
(`nhanYeuCauTuClient`): nếu kiểm TRA "id đã gửi CHƯA" TẠI lúc client
gửi, hệ thống PHẢI biết `id=2` đã "TỪNG" xuất hiện — nhưng op2 (bản
GỐC) CHƯA commit khi client gửi LẠI, nên `dsIdDaXuLy` (chỉ cập nhật
LÚC commit) CHƯA hề chứa `id=2` LÚC đó. Kiểm tra Ở TẦNG apply LÀ lớp
bảo vệ DUY nhất hoạt động ĐÚNG trong MỌI tình huống — kể cả khi op1
gốc CHƯA từng commit, kể cả khi nó đi qua HAI primary khác nhau.
::::

::::predict{#doan-neu-op-goc-da-commit commitOnce}
GIẢ sử (khác kịch bản Ở TRÊN) op2 gốc ĐàCOMMIT thành công TRƯỚC khi
`r0` chết (client CHỈ đơn giản không NHẬN được xác nhận VÌ mạng lỗi
MỘT chiều). Client vẫn GỬI lại `id=2` — `nhanYeuCauTuClient` VẪN tạo
một `opNumber` MỚI (y hệt trên). `commitTatCaConTonDong` VẪN chạy.
`soDu TK1` CÓ đổi thêm KHÔNG?
:::opt{correct}
KHÔNG — `dsIdDaXuLy` ĐÃ chứa `id=2` (TỪ lần commit ĐẦU tiên,
`đã thành công`), NÊN `apDungQuaSoCai` cho entry MỚI (opNumber KHÁC)
chỉ trả `true` NGAY, không cộng thêm GÌ — `soDu TK1` vẫn `700`
:::
:::opt
CÓ — MỘT khi op ĐÃ commit lần đầu, gửi lại sẽ bị hệ THỐNG từ chối
(`nhanYeuCauTuClient` trả về lỗi), KHÔNG BAO giờ tạo opNumber mới
::why
Trực giác NÀY tưởng tượng MỘT lớp chặn Ở TẦNG submission — nhưng
`nhanYeuCauTuClient` (bài 2, q19) KHÔNG hề kiểm tra `dsIdDaXuLy`, nó
LUÔN tạo entry MỚI, bất kể `bt.id` đã từng xuất hiện hay CHƯA.

Chỗ lệch: dù op2 GỐC đã commit hay CHƯA, `nhanYeuCauTuClient` VẪN tạo
MỘT opNumber mới CHO client resend — sự khác biệt DUY nhất nằm Ở
`apDungQuaSoCai` LÚC entry MỚI đó tới lượt commit: NẾU `id` đã có
trong `dsIdDaXuLy` (dù ĐàCOMMIT thật hay chỉ MỚI logged nhưng chưa
commit), nó LUÔN LÀ no-op AN toàn. Đây chính LÀ Ý nghĩa "idempotent":
KHÔNG quan trọng nó ĐÃ xảy ra hay CHƯA khi bạn gửi lại — kết quả CUỐI
cùng luôn giống hệt.
::
:::
::::

::::code{#viet_commit_tat_ca_con_ton_dong}
Hoàn thiện `commitTatCaConTonDong` — VỚI mỗi entry CÒN tồn đọng (theo
đúng thứ tự `opNumber` tăng dần), gọi `reDriveEntryChuaCommit` để đưa
NÓ qua consensus.

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
  replica.log = [...td.log!];
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

function commitTatCaConTonDong(primary: Replica, cacBackupConSong: Replica[]): void {
  const conTonDong = primary.log.filter((e) => e.opNumber > primary.commitNumber).sort((a, b) => a.opNumber - b.opNumber);
  for (const entry of conTonDong) {
    ___
  }
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
const btGoc: ButToan = { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 300 };
const entry2 = nhanYeuCauTuClient(r0, btGoc);
xuLyPrepare(r1, { loai: 'Prepare', tu: r0.chiSo, den: r1.chiSo, viewNumber: r0.viewNumber, opNumber: entry2.opNumber, bt: entry2.bt });

phatHienPrimaryChet(r1, 1);
phatHienPrimaryChet(r2, 1);
const conSong = [r1, r2];
const logDayDuNhat = chonLogDayDuNhat(conSong.map((r) => r.log));
const commitNumberMoi = Math.max(...conSong.map((r) => r.commitNumber));
const primaryMoi = conSong.find((r) => r.chiSo === 1 % 3)!;
for (const r of conSong) xuLyStartView(r, { loai: 'StartView', tu: primaryMoi.chiSo, den: r.chiSo, viewNumber: 1, log: logDayDuNhat, commitNumber: commitNumberMoi });
const backupConLai = conSong.filter((r) => r.chiSo !== primaryMoi.chiSo);

const entryGuiLai = nhanYeuCauTuClient(primaryMoi, btGoc);
commitTatCaConTonDong(primaryMoi, backupConLai);
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
  replica.log = [...td.log!];
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

function commitTatCaConTonDong(primary: Replica, cacBackupConSong: Replica[]): void {
  const conTonDong = primary.log.filter((e) => e.opNumber > primary.commitNumber).sort((a, b) => a.opNumber - b.opNumber);
  for (const entry of conTonDong) {
    reDriveEntryChuaCommit(primary, cacBackupConSong, entry);
  }
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
const btGoc: ButToan = { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 300 };
const entry2 = nhanYeuCauTuClient(r0, btGoc);
xuLyPrepare(r1, { loai: 'Prepare', tu: r0.chiSo, den: r1.chiSo, viewNumber: r0.viewNumber, opNumber: entry2.opNumber, bt: entry2.bt });

phatHienPrimaryChet(r1, 1);
phatHienPrimaryChet(r2, 1);
const conSong = [r1, r2];
const logDayDuNhat = chonLogDayDuNhat(conSong.map((r) => r.log));
const commitNumberMoi = Math.max(...conSong.map((r) => r.commitNumber));
const primaryMoi = conSong.find((r) => r.chiSo === 1 % 3)!;
for (const r of conSong) xuLyStartView(r, { loai: 'StartView', tu: primaryMoi.chiSo, den: r.chiSo, viewNumber: 1, log: logDayDuNhat, commitNumber: commitNumberMoi });
const backupConLai = conSong.filter((r) => r.chiSo !== primaryMoi.chiSo);

const entryGuiLai = nhanYeuCauTuClient(primaryMoi, btGoc);
commitTatCaConTonDong(primaryMoi, backupConLai);
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
const bGoc: ButToan = { id: 2, debitAccountId: T1, creditAccountId: T2, amount: 300 };
const e2 = nhanYeuCauTuClient(p0, bGoc);
xuLyPrepare(p1, { loai: 'Prepare', tu: p0.chiSo, den: p1.chiSo, viewNumber: p0.viewNumber, opNumber: e2.opNumber, bt: e2.bt });

phatHienPrimaryChet(p1, 1);
phatHienPrimaryChet(p2, 1);
const song = [p1, p2];
const logDDN = chonLogDayDuNhat(song.map((r) => r.log));
const cnMoi = Math.max(...song.map((r) => r.commitNumber));
const pMoi = song.find((r) => r.chiSo === 1 % 3)!;
for (const r of song) xuLyStartView(r, { loai: 'StartView', tu: pMoi.chiSo, den: r.chiSo, viewNumber: 1, log: logDDN, commitNumber: cnMoi });
const bkConLai = song.filter((r) => r.chiSo !== pMoi.chiSo);

const eGuiLai = nhanYeuCauTuClient(pMoi, bGoc);
if (eGuiLai.opNumber !== 3) throw new Error("gui lai id=2 toi primary MOI phai tao mot log entry MOI (opNumber=3)");
const logLenSauResend = pMoi.log.length;
if (logLenSauResend !== 3) throw new Error("pMoi.log phai co du 3 entry sau khi client gui lai");

commitTatCaConTonDong(pMoi, bkConLai);

const soDuT1 = soDuSoSach(pMoi.soCai.cacTaiKhoan.get(T1)!);
const soDuT2 = soDuSoSach(pMoi.soCai.cacTaiKhoan.get(T2)!);
if (soDuT1 !== 700) throw new Error("soDu T1 phai la 700 -- chuyen 300 CHI ap dung MOT LAN du co HAI opNumber cung id=2");
if (soDuT2 !== 300) throw new Error("soDu T2 phai la 300");
if (pMoi.commitNumber !== 3) throw new Error("CA hai opNumber (2 va 3) phai duoc commit (dong y ve thu tu), commitNumber phai la 3");
if (pMoi.soCai.dsIdDaXuLy.size !== 2) throw new Error("dsIdDaXuLy phai co dung 2 id (1 va 2) -- id=2 chi tinh MOT LAN du xuat hien o CA hai opNumber");
if (!heThongCanBang(pMoi.soCai)) throw new Error("he thong phai can bang");
if (!xacNhanHoiTu(pMoi.soCai, bkConLai[0]!.soCai)) throw new Error("primary moi va backup con lai phai hoi tu");
```

:::hints
- kind: attention
  body: "Voi moi entry con ton dong, goi reDriveEntryChuaCommit(primary, cacBackupConSong, entry) -- mot dong trong vong for."
- kind: strategy
  body: "reDriveEntryChuaCommit(primary, cacBackupConSong, entry);"
- kind: one-line
  body: "reDriveEntryChuaCommit(primary, cacBackupConSong, entry);"
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
Idempotent xuyên suốt mất mạng VÀ view change, đã chứng minh. Bây
giờ: quét TỰ ĐỘNG hàng loạt seed trên TOÀN bộ ByteLedger (đĩa+mạng+VSR
cùng lúc) — có seed nào làm vỡ được KHÔNG?
::::

::::reflect{#nghi-lai}
`commitTatCaConTonDong` không giới thiệu khái niệm MỚI — nó LẶP đúng
`reDriveEntryChuaCommit` (bài 6) theo thứ tự `opNumber`, cho PHÉP xử
lý MỘT hàng đợi các entry tồn đọng thay VÌ chỉ một. Bài học lớn NHẤT
của cả bài 6 lẫn bài 7: `dsIdDaXuLy` (q16 bài 11, nối VÀO VSR Ở bài
2) LÀ đảm bảo DUY nhất không hề quan tâm "bút toán đó đã đi qua BAO
nhiêu opNumber, BAO nhiêu primary, hay MỘT view change có xảy ra hay
không" — nó chỉ hỏi đúng MỘT câu: "`id` này đã áp dụng CHƯA?"
::::

::::checkpoint{mastery=0.92}
::::
