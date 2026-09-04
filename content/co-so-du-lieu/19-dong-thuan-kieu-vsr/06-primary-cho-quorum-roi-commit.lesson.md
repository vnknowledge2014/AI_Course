---
id: co-so-du-lieu.dong-thuan-kieu-vsr.primary-cho-quorum-roi-commit
title: "Primary chờ quorum rồi commit"
summary: "nhanPrepareOk gộp phiếu vào Set theo opNumber (primary tự tính LÀ một phiếu khi Set vừa tạo) -- đủ nguongQuorum VÀ đúng thứ tự (opNumber===commitNumber+1) mới apDungButToanThuong (q16) THẬT vào cacTaiKhoan, rồi cập nhật commitNumber. N=5 (quorum=3): phiếu đầu tiên (primary+1 backup=2) CHƯA đủ -- trả false, TK1.creditsPosted vẫn 0; phiếu thứ hai (=3) đủ -- trả true, TK1.creditsPosted=500, heThongCanBang=true. Phiếu thứ ba (op đã commit) không được áp dụng lại."
locale: vi
track: co-so-du-lieu
module: dong-thuan-kieu-vsr
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.primary-cho-quorum-roi-commit]
requires: [db.backup-nhan-prepare-gui-prepare-ok]
concepts: [db.primary-cho-quorum-roi-commit]
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
PrepareOk đang BAY về primary (bài trước). Nhưng MỘT phiếu chưa đủ —
đây LÀ lúc quorum (bài 1) THẬT sự vào cuộc: primary phải ĐẾM.
::::

::::explain{#dem-phieu-roi-commit}
`nhanPrepareOk` gộp phiếu VÀO một `Set` THEO `opNumber` — Ngay khi
`Set` được TẠO, primary tự tính LÀ một PHIẾU của chính nó (nó ĐÃ ghi
log Ở bài 3). Đủ `nguongQuorum` (bài 1) VÀ đúng thứ tự
(`opNumber === commitNumber + 1` — KHÔNG commit vượt mặt một op còn
đang TREO) mới `apDungButToanThuong` (q16) THẬT vào `cacTaiKhoan`,
rồi cập nhật `commitNumber`:

```typescript title=readonly
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
}
function taoReplica(chiSo: number, tongSo: number): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };
}
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function heThongCanBang(cacTaiKhoan: Map<number, TaiKhoan>): boolean {
  let no = 0, co = 0;
  for (const [, tk] of cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}
interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'DoViewChange' | 'StartView' | 'YeuCauDongBo' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function nhanPrepareOk(
  primary: Replica, cacTaiKhoan: Map<number, TaiKhoan>,
  phieuTheoOp: Map<number, Set<number>>, td: ThongDiep,
): boolean {
  const opNumber = td.opNumber!;
  let phieu = phieuTheoOp.get(opNumber);
  if (!phieu) { phieu = new Set<number>([primary.chiSo]); phieuTheoOp.set(opNumber, phieu); }
  phieu.add(td.tu);
  if (phieu.size < nguongQuorum(primary.tongSo)) return false;
  if (opNumber !== primary.commitNumber + 1) return false;
  const entry = primary.log.find((e) => e.opNumber === opNumber)!;
  apDungButToanThuong(cacTaiKhoan, entry.bt);
  primary.commitNumber = opNumber;
  return true;
}

const NGUON_NGOAI = 0, TK1 = 1;
const ck = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, TK1]) ck.set(id, taoTaiKhoan(id));
const primary = taoReplica(0, 5); // N=5, quorum=3
primary.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 500 } });
primary.opNumber = 1;
const phieuTheoOp = new Map<number, Set<number>>();
console.log("phieu 1 (backup1):", nhanPrepareOk(primary, ck, phieuTheoOp, { loai: 'PrepareOk', tu: 1, den: 0, viewNumber: 0, opNumber: 1 }));
console.log("phieu 2 (backup2):", nhanPrepareOk(primary, ck, phieuTheoOp, { loai: 'PrepareOk', tu: 2, den: 0, viewNumber: 0, opNumber: 1 }));
console.log("TK1.creditsPosted:", ck.get(TK1)!.creditsPosted, "canBang:", heThongCanBang(ck));
```

```text title=readonly
phieu 1 (backup1): false
phieu 2 (backup2): true
TK1.creditsPosted: 500 canBang: true
```

`N=5` cần `3` phiếu (bài 1). PHIẾU đầu tiên (primary tự tính + backup1
= `2`) CHƯA đủ — trả `false`, `TK1.creditsPosted` VẪN `0`. Phiếu THỨ
hai (`= 3`) đủ quorum — trả `true`, bút toán ĐƯỢC áp dụng THẬT, hệ
thống cân BẰNG.
::::

::::example{#commit-khong-lap-lai}
Nếu backup3 gửi THÊM một `PrepareOk` cho op ĐÃ commit (`opNumber=1`
sau khi `commitNumber` ĐÃ LÀ `1`), điều KIỆN `opNumber !== primary.
commitNumber + 1` (`1 !== 2`) chặn NGAY — trả `false`, bút toán
KHÔNG bị áp dụng LẦN thứ hai. Đây LÀ lá chắn TỰ NHIÊN chống double-
apply, gần GIỐNG tinh thần `dsIdDaXuLy` (q16 bài 11), nhưng dựa TRÊN
thứ tự `opNumber` thay VÌ một tập `id` riêng.
::::

::::predict{#doan-op-vuot-mat commitOnce}
Nếu MỘT `PrepareOk` cho `opNumber=2` ĐẾN TRƯỚC khi `opNumber=1` đạt
quorum (`primary.commitNumber` VẪN LÀ `0`), VÀ `opNumber=2` tự nó
ĐẠT đủ phiếu quorum — `nhanPrepareOk` CÓ commit `opNumber=2` NGAY
không?
:::opt{correct}
KHÔNG — dù ĐỦ phiếu, điều kiện `opNumber !== primary.commitNumber +
1` (`2 !== 0+1=1`) chặn LẠI; VSR LUÔN commit THEO đúng thứ tự
`opNumber` liên tục, không cho PHÉP "nhảy cóc" dù op sau đã đủ phiếu
:::
:::opt
CÓ — MỘT khi đủ quorum, op ĐÓ nên commit NGAY để không lãng phí thời
gian chờ op TRƯỚC
::why
Trực giác NÀY tối ưu hoá TỐC độ nhưng phá vỡ đúng bất BIẾN cốt lõi
CỦA VSR: log PHẢI là một dãy LIÊN tục, không lỗ hổng.

Chỗ lệch: NẾU cho phép commit `opNumber=2` trước `opNumber=1`, một
replica ĐỌC lại log SẼ thấy `commitNumber=2` nhưng KHÔNG chắc `op1`
đã ĐƯỢC áp dụng — vi phạm đúng thứ tự GIAO dịch (giống bài 9, q18:
mất MỘT nửa giao dịch làm vỡ `heThongCanBang`). Kiểm tra thứ tự NÀY
chính LÀ điều ngăn tình huống đó xảy ra Ở TẦNG đồng thuận, trước cả
khi chạm tới TẦNG lưu trữ.
::
:::
::::

::::code{#viet_nhan_prepare_ok}
Hoàn thiện `nhanPrepareOk` — kiểm tra đủ quorum VÀ đúng thứ tự, rồi
áp dụng bút toán THẬT và cập nhật `commitNumber`.

```typescript title=starter
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
}
function taoReplica(chiSo: number, tongSo: number): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };
}
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}
interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'DoViewChange' | 'StartView' | 'YeuCauDongBo' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function nhanPrepareOk(
  primary: Replica, cacTaiKhoan: Map<number, TaiKhoan>,
  phieuTheoOp: Map<number, Set<number>>, td: ThongDiep,
): boolean {
  const opNumber = td.opNumber!;
  let phieu = phieuTheoOp.get(opNumber);
  if (!phieu) { phieu = new Set<number>([primary.chiSo]); phieuTheoOp.set(opNumber, phieu); }
  phieu.add(td.tu);
  if (phieu.size < nguongQuorum(primary.tongSo)) return false;
  if (opNumber !== primary.commitNumber + 1) return false;
  ___
}

const ck = new Map<number, TaiKhoan>();
ck.set(0, taoTaiKhoan(0)); ck.set(1, taoTaiKhoan(1));
const primary = taoReplica(0, 3);
primary.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 } });
primary.opNumber = 1;
console.log(nhanPrepareOk(primary, ck, new Map(), { loai: 'PrepareOk', tu: 1, den: 0, viewNumber: 0, opNumber: 1 }));
```

```typescript title=solution
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
}
function taoReplica(chiSo: number, tongSo: number): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };
}
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}
interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'DoViewChange' | 'StartView' | 'YeuCauDongBo' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function nhanPrepareOk(
  primary: Replica, cacTaiKhoan: Map<number, TaiKhoan>,
  phieuTheoOp: Map<number, Set<number>>, td: ThongDiep,
): boolean {
  const opNumber = td.opNumber!;
  let phieu = phieuTheoOp.get(opNumber);
  if (!phieu) { phieu = new Set<number>([primary.chiSo]); phieuTheoOp.set(opNumber, phieu); }
  phieu.add(td.tu);
  if (phieu.size < nguongQuorum(primary.tongSo)) return false;
  if (opNumber !== primary.commitNumber + 1) return false;
  const entry = primary.log.find((e) => e.opNumber === opNumber)!;
  apDungButToanThuong(cacTaiKhoan, entry.bt);
  primary.commitNumber = opNumber;
  return true;
}

const ck = new Map<number, TaiKhoan>();
ck.set(0, taoTaiKhoan(0)); ck.set(1, taoTaiKhoan(1));
const primary = taoReplica(0, 3);
primary.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 } });
primary.opNumber = 1;
console.log(nhanPrepareOk(primary, ck, new Map(), { loai: 'PrepareOk', tu: 1, den: 0, viewNumber: 0, opNumber: 1 }));
```

```typescript title=test
function heThongCanBangT(cacTaiKhoan: Map<number, TaiKhoan>): boolean {
  let no = 0, co = 0;
  for (const [, tk] of cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
const NGUON_NGOAI = 0, TK1 = 1;
const ck2 = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, TK1]) ck2.set(id, taoTaiKhoan(id));
const p6 = taoReplica(0, 5); // N=5, quorum=3
p6.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 500 } });
p6.opNumber = 1;
const pto = new Map<number, Set<number>>();
const r1 = nhanPrepareOk(p6, ck2, pto, { loai: 'PrepareOk', tu: 1, den: 0, viewNumber: 0, opNumber: 1 });
if (r1 !== false) throw new Error("N=5, quorum=3: primary(1 phieu tu) + 1 backup = 2 phieu, CHUA du quorum -- phai tra ve false");
if (ck2.get(TK1)!.creditsPosted !== 0) throw new Error("CHUA du quorum thi KHONG duoc ap dung but toan");
const r2 = nhanPrepareOk(p6, ck2, pto, { loai: 'PrepareOk', tu: 2, den: 0, viewNumber: 0, opNumber: 1 });
if (r2 !== true) throw new Error("du 3 phieu (primary+2 backup) thi PHAI tra ve true va commit");
if (p6.commitNumber !== 1) throw new Error("commitNumber phai duoc cap nhat thanh 1");
if (ck2.get(TK1)!.creditsPosted !== 500) throw new Error("but toan phai duoc AP DUNG vao cacTaiKhoan that");
if (!heThongCanBangT(ck2)) throw new Error("sau khi commit dung, he thong phai can bang");
const r3 = nhanPrepareOk(p6, ck2, pto, { loai: 'PrepareOk', tu: 3, den: 0, viewNumber: 0, opNumber: 1 });
if (r3 !== false) throw new Error("op da commit roi -- phieu them KHONG duoc tra ve true lan nua (tranh ap dung TRUNG LAP)");
if (ck2.get(TK1)!.creditsPosted !== 500) throw new Error("op da commit thi KHONG duoc ap dung LAI but toan lan nua");
```

:::hints
- kind: attention
  body: "Tim entry theo opNumber trong primary.log, ap dung but toan that, cap nhat commitNumber, tra ve true -- bon dong."
- kind: strategy
  body: "const entry = primary.log.find((e) => e.opNumber === opNumber)!; apDungButToanThuong(cacTaiKhoan, entry.bt); primary.commitNumber = opNumber; return true;"
- kind: one-line
  body: "const entry = primary.log.find((e) => e.opNumber === opNumber)!; apDungButToanThuong(cacTaiKhoan, entry.bt); primary.commitNumber = opNumber; return true;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Primary vừa commit — nhưng CHỈ NÓ biết. Backup vẫn còn giữ bút toán
Ở trạng thái "đã ghi, chưa áp dụng". Ai báo cho HỌ?
::::

::::reflect{#nghi-lai}
`nhanPrepareOk` LÀ nơi "prepare_ok" gặp "commit" (MASTERPLAN §9.2):
đếm phiếu (bài 1), tôn trọng thứ tự (bài 3), RỒI mới chạm TỚI
`apDungButToanThuong` THẬT (q16). Đây LÀ lần ĐẦU tiên trong q19 một
bút toán THỰC SỰ ảnh hưởng số dư — mọi bài TRƯỚC chỉ chuẩn bị cho
khoảnh khắc NÀY.
::::

::::checkpoint{mastery=0.9}
::::
