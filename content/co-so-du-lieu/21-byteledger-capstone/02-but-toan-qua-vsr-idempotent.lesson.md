---
id: co-so-du-lieu.byteledger-capstone.but-toan-qua-vsr-idempotent
title: "Bút toán đi qua VSR, áp dụng idempotent"
summary: "nhanPrepareOk (q19) giờ áp dụng bút toán bằng apDungQuaSoCai (q16) thay vì apDungButToanThuong thô -- kiểm tra dsIdDaXuLy TRƯỚC khi ghi. N=3, hai bút toán (nạp 1000, chuyển 400) qua đủ prepare/prepare_ok/commit: soDu TK1=600, TK2=400, commitNumber=2. Gửi LẠI đúng bút toán id=2 (giả lập client retry) -- primary tạo MỘT log entry MỚI (opNumber=3, commitNumber=3) và VẪN commit nó (đồng thuận về thứ tự không đổi), nhưng soDu KHÔNG đổi thêm (soDu TK1=600, TK2=400) và dsIdDaXuLy.size vẫn là 2 -- opNumber bảo vệ THỨ TỰ, dsIdDaXuLy bảo vệ HIỆU ỨNG, hai lớp độc lập nhau."
locale: vi
track: co-so-du-lieu
module: byteledger-capstone
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.but-toan-qua-vsr-idempotent]
requires: [db.kien-truc-byteledger]
concepts: [db.but-toan-qua-vsr-idempotent]
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
`Replica` giờ mang một sổ cái ĐẦY đủ (bài trước) — nhưng bài trước áp
dụng bút toán TRỰC TIẾP, không qua đồng thuận. q19 (bài 3-7) VỐN áp
dụng bằng `apDungButToanThuong` thô, KHÔNG hề kiểm tra trùng lặp. Nối
lại đúng chỗ ĐÓ.
::::

::::explain{#commit-qua-so-cai-day-du}
`nhanPrepareOk` (q19 bài 6) VỐN gọi `apDungButToanThuong` — CỘNG
thẳng vào `Map<TaiKhoan>`, không kiểm tra GÌ. Giờ nó gọi
`apDungQuaSoCai` (q16 BOSS's `guiButToanAnToan`, RÚT gọn phần kiểm
tra số dư để tập trung ĐÚNG vào idempotent) — kiểm tra `dsIdDaXuLy`
TRƯỚC khi ghi:

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
// Nang cap so voi bai 1: kiem tra dsIdDaXuLy TRUOC khi ghi -- idempotent (q16 bai 11).
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
  loai: 'Prepare' | 'PrepareOk' | 'Commit';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number;
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

function chayMotVongDayDu(primary: Replica, backups: Replica[], bt: ButToan): void {
  const entry = nhanYeuCauTuClient(primary, bt);
  const phieuTheoOp = new Map<number, Set<number>>();
  for (const backup of backups) {
    const phanHoi = xuLyPrepare(backup, { loai: 'Prepare', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, opNumber: entry.opNumber, bt: entry.bt });
    nhanPrepareOk(primary, phieuTheoOp, phanHoi);
  }
  for (const backup of backups) {
    xuLyCommit(backup, { loai: 'Commit', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
  }
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const r0 = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
const r1 = taoReplica(1, 3, [NGUON_NGOAI, TK1, TK2]);
const r2 = taoReplica(2, 3, [NGUON_NGOAI, TK1, TK2]);

chayMotVongDayDu(r0, [r1, r2], { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
chayMotVongDayDu(r0, [r1, r2], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 });
console.log("sau op1+op2 -- soDu TK1:", soDuSoSach(r0.soCai.cacTaiKhoan.get(TK1)!), "soDu TK2:", soDuSoSach(r0.soCai.cacTaiKhoan.get(TK2)!), "commitNumber:", r0.commitNumber);

// client gui LAI dung id=2 (khong nhan duoc phan hoi) -- primary tao log entry MOI (opNumber=3)
chayMotVongDayDu(r0, [r1, r2], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 });
console.log("sau khi gui LAI id=2 -- soDu TK1:", soDuSoSach(r0.soCai.cacTaiKhoan.get(TK1)!), "soDu TK2:", soDuSoSach(r0.soCai.cacTaiKhoan.get(TK2)!));
console.log("commitNumber:", r0.commitNumber, "opNumber:", r0.opNumber, "dsIdDaXuLy.size:", r0.soCai.dsIdDaXuLy.size);
```

```text title=readonly
sau op1+op2 -- soDu TK1: 600 soDu TK2: 400 commitNumber: 2
sau khi gui LAI id=2 -- soDu TK1: 600 soDu TK2: 400
commitNumber: 3 opNumber: 3 dsIdDaXuLy.size: 2
```

Gửi LẠI đúng `id=2` TẠO một log entry MỚI (`opNumber=3` — `nhanYeuCauTuClient`
KHÔNG hề biết gì về `dsIdDaXuLy`, nó chỉ tăng bộ đếm) — VÀ entry ĐÓ
VẪN đi trót lọt qua prepare/prepare_ok, commit THÀNH `3`. Nhưng
`apDungQuaSoCai` phát hiện `id=2` ĐÃ có trong `dsIdDaXuLy`, KHÔNG cộng
thêm GÌ — `soDu` giữ NGUYÊN `600`/`400`. Hai lớp bảo VỆ hoàn toàn độc
lập: `opNumber` (VSR) đảm bảo MỌI replica đồng Ý về THỨ TỰ các thao
tác; `dsIdDaXuLy` (sổ cái) đảm bảo HIỆU ỨNG của một thao TÁC chỉ xảy
ra đúng MỘT lần, dù nó được ĐỒNG thuận bao nhiêu LẦN.
::::

::::example{#hai-lop-doc-lap}
Đây LÀ điểm KHÁC hẳn q19 (bài 6): ở ĐÓ, "commit HAI lần cùng op" bị
chặn Ở TẦNG `opNumber` (`opNumber !== commitNumber+1`) — nhưng CHẶN
đó chỉ bảo vệ được MỘT `opNumber` cụ THỂ khỏi bị áp dụng LẶP, KHÔNG
bảo vệ được trường hợp CÙNG một `bt.id` xuất hiện Ở HAI `opNumber`
khác nhau (chính XÁC điều xảy ra Ở readonly TRÊN: `id=2` Ở CẢ
`opNumber=2` LẪN `opNumber=3`). Đó LÀ Lý do sổ cái ĐẦY đủ CẦN
`dsIdDaXuLy` riêng — một lớp bảo vệ Ở TẦNG "Ý nghĩa nghiệp vụ", không
phải TẦNG "thứ tự log".
::::

::::predict{#doan-hai-id-khac-nhau commitOnce}
Thay VÌ gửi LẠI `id=2`, gửi HAI bút toán MỚI HOÀN toàn khác `id`
(`id=10` VÀ `id=11`, CÙNG `amount=50`, CÙNG cặp tài khoản) — qua
`chayMotVongDayDu` HAI lần liên TIẾP. `soDu` tổng cộng thay đổi BAO
nhiêu?
:::opt{correct}
`100` (`50+50`) — hai `id` khác nhau LÀ hai bút toán THẬT sự riêng
biệt trong `dsIdDaXuLy`, KHÔNG hề bị coi LÀ trùng lặp — cả hai đều
được áp dụng THẬT
:::
:::opt
`50` — hệ thống "gộp" các bút toán CÓ cùng cặp tài khoản VÀ cùng
`amount` lại, coi LÀ một giao dịch lặp
::why
Trực giác NÀY nhầm "idempotent theo `id`" VỚI "gộp theo NỘI dung
giống nhau" — giống HỆT nhầm lẫn q16 bài 11 đã CHỈ ra.

Chỗ lệch: `apDungQuaSoCai` chỉ SO sánh `soCai.dsIdDaXuLy.has(bt.id)`
— KHÔNG hề nhìn tới `amount`/`debitAccountId`/`creditAccountId`. Hai
`id` phân biệt (`10`, `11`), DÙ giống hệt nhau Ở MỌI trường khác, vẫn
LÀ hai bút toán riêng biệt — cả hai đều CHƯA có trong `dsIdDaXuLy`,
cả hai đều được commit VÀ áp dụng THẬT.
::
:::
::::

::::code{#viet_ap_dung_qua_so_cai}
Hoàn thiện `apDungQuaSoCai` — nếu `bt.id` ĐÃ có trong `dsIdDaXuLy`,
trả VỀ `true` NGAY (thành công, KHÔNG ghi gì thêm) trước khi chạm
tới BẤT kỳ dòng ghi nào.

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
  ___
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
  loai: 'Prepare' | 'PrepareOk' | 'Commit';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number;
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

function chayMotVongDayDu(primary: Replica, backups: Replica[], bt: ButToan): void {
  const entry = nhanYeuCauTuClient(primary, bt);
  const phieuTheoOp = new Map<number, Set<number>>();
  for (const backup of backups) {
    const phanHoi = xuLyPrepare(backup, { loai: 'Prepare', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, opNumber: entry.opNumber, bt: entry.bt });
    nhanPrepareOk(primary, phieuTheoOp, phanHoi);
  }
  for (const backup of backups) {
    xuLyCommit(backup, { loai: 'Commit', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
  }
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const r0 = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
const r1 = taoReplica(1, 3, [NGUON_NGOAI, TK1, TK2]);
const r2 = taoReplica(2, 3, [NGUON_NGOAI, TK1, TK2]);
chayMotVongDayDu(r0, [r1, r2], { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
chayMotVongDayDu(r0, [r1, r2], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 });
chayMotVongDayDu(r0, [r1, r2], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 });
console.log(soDuSoSach(r0.soCai.cacTaiKhoan.get(TK1)!));
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
  loai: 'Prepare' | 'PrepareOk' | 'Commit';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number;
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

function chayMotVongDayDu(primary: Replica, backups: Replica[], bt: ButToan): void {
  const entry = nhanYeuCauTuClient(primary, bt);
  const phieuTheoOp = new Map<number, Set<number>>();
  for (const backup of backups) {
    const phanHoi = xuLyPrepare(backup, { loai: 'Prepare', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, opNumber: entry.opNumber, bt: entry.bt });
    nhanPrepareOk(primary, phieuTheoOp, phanHoi);
  }
  for (const backup of backups) {
    xuLyCommit(backup, { loai: 'Commit', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
  }
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const r0 = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
const r1 = taoReplica(1, 3, [NGUON_NGOAI, TK1, TK2]);
const r2 = taoReplica(2, 3, [NGUON_NGOAI, TK1, TK2]);
chayMotVongDayDu(r0, [r1, r2], { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
chayMotVongDayDu(r0, [r1, r2], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 });
chayMotVongDayDu(r0, [r1, r2], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 });
console.log(soDuSoSach(r0.soCai.cacTaiKhoan.get(TK1)!));
```

```typescript title=test
function layCommitNumber(r: Replica): number { return r.commitNumber; }
function layOpNumber(r: Replica): number { return r.opNumber; }

const rp = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
const rb1 = taoReplica(1, 3, [NGUON_NGOAI, TK1, TK2]);
const rb2 = taoReplica(2, 3, [NGUON_NGOAI, TK1, TK2]);
chayMotVongDayDu(rp, [rb1, rb2], { id: 10, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 500 });
if (soDuSoSach(rp.soCai.cacTaiKhoan.get(TK1)!) !== 500) throw new Error("sau 1 vong day du, soDu TK1 phai la 500");
if (layCommitNumber(rp) !== 1) throw new Error("commitNumber phai la 1 sau 1 op");
if (soDuSoSach(rb1.soCai.cacTaiKhoan.get(TK1)!) !== 500) throw new Error("backup rb1 phai cung co soDu 500 (xuLyCommit da ap dung)");
if (soDuSoSach(rb2.soCai.cacTaiKhoan.get(TK1)!) !== 500) throw new Error("backup rb2 phai cung co soDu 500");

chayMotVongDayDu(rp, [rb1, rb2], { id: 10, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 500 });
if (layOpNumber(rp) !== 2) throw new Error("gui lai van tao mot opNumber MOI (2), khong tai su dung opNumber cu");
if (layCommitNumber(rp) !== 2) throw new Error("op moi (dung id cu) van duoc COMMIT (dong y ve thu tu), chi khong duoc AP DUNG THEM");
if (soDuSoSach(rp.soCai.cacTaiKhoan.get(TK1)!) !== 500) throw new Error("soDu KHONG duoc doi them -- id=10 da xu ly roi, apDungQuaSoCai phai la no-op lan hai");
if (rp.soCai.dsIdDaXuLy.size !== 1) throw new Error("dsIdDaXuLy chi co dung 1 id (10), du da commit 2 op");
if (!heThongCanBang(rp.soCai)) throw new Error("he thong van phai can bang sau ca hai lan");

const rp3 = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
const rb3a = taoReplica(1, 3, [NGUON_NGOAI, TK1, TK2]);
const rb3b = taoReplica(2, 3, [NGUON_NGOAI, TK1, TK2]);
chayMotVongDayDu(rp3, [rb3a, rb3b], { id: 20, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 100 });
chayMotVongDayDu(rp3, [rb3a, rb3b], { id: 21, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 50 });
if (soDuSoSach(rp3.soCai.cacTaiKhoan.get(TK1)!) !== 150) throw new Error("hai id KHAC nhau (20,21) phai duoc AP DUNG CA HAI, tong 150");

const soCaiDon = taoHeThongSoCai([NGUON_NGOAI, TK1]);
const lanDau = apDungQuaSoCai(soCaiDon, { id: 99, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 10 });
if (lanDau !== true) throw new Error("apDungQuaSoCai lan dau (id moi) phai tra ve true");
const lanHai = apDungQuaSoCai(soCaiDon, { id: 99, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 10 });
if (lanHai !== true) throw new Error("apDungQuaSoCai lan hai (id DA xu ly) VAN phai tra ve true (idempotent = thanh cong, khong phai loi)");
if (soDuSoSach(soCaiDon.cacTaiKhoan.get(TK1)!) !== 10) throw new Error("goi apDungQuaSoCai HAI lan cung id chi duoc cong DUNG MOT LAN, soDu phai la 10 khong phai 20");
if (soCaiDon.dsIdDaXuLy.size !== 1) throw new Error("dsIdDaXuLy phai co dung 1 phan tu (99) sau hai lan goi cung id");
```

:::hints
- kind: attention
  body: "Neu bt.id da co trong soCai.dsIdDaXuLy thi tra ve true NGAY, truoc khi dong nao khac chay -- mot dong."
- kind: strategy
  body: "if (soCai.dsIdDaXuLy.has(bt.id)) return true;"
- kind: one-line
  body: "if (soCai.dsIdDaXuLy.has(bt.id)) return true;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "600"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bút toán đi qua VSR đúng cách, idempotent xuyên suốt. Nhưng "đường
mạng" tới giờ hoàn hảo — VÀ mỗi replica chưa hề chạm tới đĩa nào cả.
::::

::::reflect{#nghi-lai}
`apDungQuaSoCai` chỉ THÊM đúng MỘT điều kiện VÀO trước
`apDungButToanThuong` (q19) — nhưng điều kiện ĐÓ LÀ ranh giới GIỮA
một VSR "chỉ đồng thuận về thứ tự" (q19 thuần) VÀ một VSR "đồng thuận
về thứ tự VÀ đảm bảo hiệu ứng đúng MỘT lần" (ByteLedger). `opNumber`
VÀ `dsIdDaXuLy` LÀ hai lớp bảo VỆ ĐỘC lập — bài SAU (bài 7) sẽ đẩy
lớp NÀY tới giới hạn: retry SAU cả một cuộc view change.
::::

::::checkpoint{mastery=0.85}
::::
