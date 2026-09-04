---
id: co-so-du-lieu.dong-thuan-kieu-vsr.client-gui-yeu-cau-toi-primary
title: "Client gửi yêu cầu tới primary"
summary: "nhanYeuCauTuClient(primary,bt) tăng primary.opNumber lên 1, tạo LogEntry{opNumber,bt} MỚI với opNumber ĐÓ (KHÔNG PHẢI bt.id), rồi đẩy vào primary.log -- ghi CỤC BỘ, chưa hề commit. Hai bút toán liên tiếp (id=999 rồi id=5, cố tình khác opNumber mong đợi) nhận đúng opNumber=1 rồi opNumber=2 -- primary.opNumber theo kịp, nhưng commitNumber vẫn đứng yên ở 0 sau cả hai lần gọi."
locale: vi
track: co-so-du-lieu
module: dong-thuan-kieu-vsr
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [db.client-gui-yeu-cau-toi-primary]
requires: [db.trang-thai-mot-replica]
concepts: [db.client-gui-yeu-cau-toi-primary]
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
`Replica` (bài trước) có chỗ để GIỮ bút toán (`log`) — nhưng CHƯA có
gì trong ĐÓ. Một client THẬT gửi một yêu cầu tới primary. Chuyện GÌ
xảy ra ĐẦU tiên?
::::

::::explain{#nhan-yeu-cau}
`nhanYeuCauTuClient(primary, bt)` tăng `primary.opNumber` lên `1`,
tạo một `LogEntry` MỚI với `opNumber` VỪA tăng (KHÔNG phải `bt.id` —
`opNumber` LÀ số THỨ tự CỦA PRIMARY, độc lập hoàn toàn với `id` bên
trong `ButToan`), rồi đẩy vào `primary.log`:

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

function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  primary.opNumber += 1;
  const entry: LogEntry = { opNumber: primary.opNumber, bt };
  primary.log.push(entry);
  return entry;
}

const primary = taoReplica(0, 3);
const e1 = nhanYeuCauTuClient(primary, { id: 999, debitAccountId: 0, creditAccountId: 1, amount: 1000 });
const e2 = nhanYeuCauTuClient(primary, { id: 5, debitAccountId: 1, creditAccountId: 2, amount: 400 });
console.log("e1.opNumber:", e1.opNumber, "e2.opNumber:", e2.opNumber);
console.log("primary.opNumber:", primary.opNumber, "primary.commitNumber:", primary.commitNumber);
```

```text title=readonly
e1.opNumber: 1 e2.opNumber: 2
primary.opNumber: 2 primary.commitNumber: 0
```

`bt.id` LẦN lượt LÀ `999` VÀ `5` — nhưng `opNumber` VẪN đúng `1` RỒI
`2`, hoàn TOÀN không liên quan `id` CỦA bút toán. `primary.opNumber`
theo KỊP (`2`) — nhưng `commitNumber` VẪN LÀ `0`: ghi vào `log` CỤC
bộ CHƯA phải LÀ commit.
::::

::::example{#cuc-bo-khac-commit}
Đây LÀ điểm phân BIỆT quan trọng NHẤT của cả VSR: `primary.log` chứa
những bút TOÁN primary ĐÃ "nhận VÀ đánh số", nhưng CHƯA chắc đã ĐƯỢC
đa số replica XÁC nhận. NẾU primary chết NGAY sau `nhanYeuCauTuClient`
(trước KHI kịp gửi Prepare — bài 4), bút TOÁN đó có THỂ biến mất
hoàn toàn MÀ không vi phạm gì — client CHƯA nhận được XÁC nhận
`commit` nên KHÔNG được coi LÀ đã thành công.
::::

::::predict{#doan-goi-lan-ba commitOnce}
SAU hai lời gọi Ở trên (`primary.opNumber = 2`), gọi THÊM
`nhanYeuCauTuClient` lần THỨ ba với MỘT `ButToan` bất kỳ. `opNumber`
của entry MỚI LÀ bao nhiêu?
:::opt{correct}
`3` — `primary.opNumber` LUÔN tăng thêm ĐÚNG `1` mỗi lần gọi, bất kể
nội dung `bt` LÀ gì; đây LÀ một bộ ĐẾM đơn thuần, không liên quan gì
tới GIÁ trị bên trong bút toán
:::
:::opt
Phụ thuộc `bt.id` của lần gọi thứ BA — nếu `bt.id` nhỏ hơn `2`,
`opNumber` có THỂ "lùi" LẠI
::why
Trực giác NÀY LẶP lại đúng nhầm LẪN mà bài NÀY vừa CHỈ ra: `opNumber`
KHÔNG hề đọc `bt.id`.

Chỗ lệch: dòng `primary.opNumber += 1;` LÀ phép CỘNG dựa HOÀN toàn
trên trạng thái NỘI bộ CỦA `primary` (giá trị TRƯỚC đó LÀ `2`) — tham
số `bt` chỉ được DÙNG để tạo `entry.bt`, không hề ẢNH hưởng tới phép
TÍNH `opNumber`.
::
:::
::::

::::code{#viet_nhan_yeu_cau}
Hoàn thiện `nhanYeuCauTuClient` — tăng `primary.opNumber`, tạo
`entry` với `opNumber` ĐÓ, đẩy vào `primary.log`, trả về `entry`.

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

function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  ___
}

const primary = taoReplica(0, 3);
console.log(nhanYeuCauTuClient(primary, { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 }).opNumber);
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

function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  primary.opNumber += 1;
  const entry: LogEntry = { opNumber: primary.opNumber, bt };
  primary.log.push(entry);
  return entry;
}

const primary = taoReplica(0, 3);
console.log(nhanYeuCauTuClient(primary, { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 }).opNumber);
```

```typescript title=test
const p2 = taoReplica(0, 3);
const e1 = nhanYeuCauTuClient(p2, { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 });
if (e1.opNumber !== 1) throw new Error("bat toan dau tien phai co opNumber=1");
const e2 = nhanYeuCauTuClient(p2, { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 400 });
if (e2.opNumber !== 2) throw new Error("bat toan thu hai phai co opNumber=2");
if (p2.opNumber !== 2) throw new Error("primary.opNumber phai theo kip opNumber cao nhat");
if (p2.log.length !== 2) throw new Error("moi yeu cau phai duoc GHI vao log (khong chi tra ve entry)");
if (p2.commitNumber !== 0) throw new Error("ghi log CUC BO chua phai la commit -- commitNumber phai VAN la 0");
if (p2.log[0]!.opNumber !== 1 || p2.log[1]!.opNumber !== 2) throw new Error("log phai giu dung thu tu opNumber");
const p3 = taoReplica(0, 3);
const eA = nhanYeuCauTuClient(p3, { id: 999, debitAccountId: 0, creditAccountId: 1, amount: 50 });
if (eA.opNumber !== 1) throw new Error("opNumber phai la SO THU TU cua primary (bat dau tu 1), KHONG PHAI id cua ButToan (id=999 o day)");
const eB = nhanYeuCauTuClient(p3, { id: 5, debitAccountId: 1, creditAccountId: 2, amount: 20 });
if (eB.opNumber !== 2) throw new Error("opNumber thu hai phai la 2, KHONG PHAI id=5 cua ButToan");
```

:::hints
- kind: attention
  body: "Tang primary.opNumber, tao entry voi opNumber do, day vao primary.log, tra ve entry -- bon dong."
- kind: strategy
  body: "primary.opNumber += 1; const entry: LogEntry = { opNumber: primary.opNumber, bt }; primary.log.push(entry); return entry;"
- kind: one-line
  body: "primary.opNumber += 1; const entry: LogEntry = { opNumber: primary.opNumber, bt }; primary.log.push(entry); return entry;"
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
Primary đã ghi bút toán CỦA riêng nó. Nhưng MỘT mình primary biết
KHÔNG đủ — nó phải BÁO cho các backup. Đó LÀ Prepare.
::::

::::reflect{#nghi-lai}
`nhanYeuCauTuClient` LÀ bước ĐẦU tiên của "prepare/prepare_ok/commit"
(MASTERPLAN §9.2): primary GHI trước, RỒI mới lan truyền. `opNumber`
LÀ "số THỨ tự tuyệt đối" mà TOÀN bộ hệ thống sẽ dùng để SO sánh "ai
đầy đủ HƠN" (bài 11) VÀ "ai lạc HẬU" (bài 13) — một con số ĐƠN giản,
nhưng LÀ trục thời gian CỦA cả giao thức.
::::

::::checkpoint{mastery=0.8}
::::
