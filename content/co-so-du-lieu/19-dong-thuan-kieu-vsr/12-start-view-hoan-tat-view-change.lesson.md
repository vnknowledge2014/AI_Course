---
id: co-so-du-lieu.dong-thuan-kieu-vsr.start-view-hoan-tat-view-change
title: "StartView — hoàn tất View Change"
summary: "xuLyStartView thay THẾ HOÀN TOÀN log của replica bằng log hợp nhất (từ chonLogDayDuNhat), cập nhật opNumber theo entry cuối, viewNumber theo thông điệp, commitNumber dùng '??' để GIỮ NGUYÊN giá trị cũ nếu thông điệp không mang theo (không phải ghi đè undefined), rồi chuyển status='normal'. Replica đang 'view-change' opNumber=0 nhận StartView với log 3 entry, commitNumber=2 -- sau đó: status='normal', opNumber=3, commitNumber=2. Không có commitNumber trong thông điệp: commitNumber cũ (5) GIỮ NGUYÊN, không bị ghi đè."
locale: vi
track: co-so-du-lieu
module: dong-thuan-kieu-vsr
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.start-view-hoan-tat-view-change]
requires: [db.do-view-change-chon-log-day-du-nhat]
concepts: [db.start-view-hoan-tat-view-change]
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
Primary mới ĐÃ chọn được log đầy đủ nhất (bài trước). Bước CUỐI của
view change: LOAN báo log ĐÓ cho mọi replica — VÀ chính thức QUAY lại
trạng thái 'normal', sẵn sàng phục vụ TIẾP.
::::

::::explain{#xu-ly-start-view}
`xuLyStartView` thay THẾ hoàn toàn `log` của replica BẰNG log hợp
nhất (`td.log`, TỪ `chonLogDayDuNhat` bài 11), cập nhật `opNumber`
theo entry CUỐI, `viewNumber` theo thông ĐIỆP, `commitNumber` DÙNG
`??` để GIỮ nguyên giá trị CŨ nếu thông điệp KHÔNG mang theo (không
phải ghi đè `undefined`), rồi chuyển `status='normal'`:

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
interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'DoViewChange' | 'StartView' | 'YeuCauDongBo' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function opCuaLog(log: LogEntry[]): number {
  return log.length === 0 ? 0 : log[log.length - 1]!.opNumber;
}
function xuLyStartView(replica: Replica, td: ThongDiep): void {
  replica.log = td.log!;
  replica.opNumber = opCuaLog(replica.log);
  replica.viewNumber = td.viewNumber;
  replica.commitNumber = td.commitNumber ?? replica.commitNumber;
  replica.status = 'normal';
}

const backup2 = taoReplica(2, 3);
backup2.status = 'view-change'; backup2.viewNumber = 1;
const logHopNhat: LogEntry[] = [
  { opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 } },
  { opNumber: 2, bt: { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 400 } },
];
xuLyStartView(backup2, { loai: 'StartView', tu: 1, den: 2, viewNumber: 1, log: logHopNhat, commitNumber: 1 });
console.log("status:", backup2.status, "opNumber:", backup2.opNumber, "commitNumber:", backup2.commitNumber);
```

```text title=readonly
status: normal opNumber: 2 commitNumber: 1
```

`backup2` bắt đầu `status='view-change'`, `opNumber=0` — SAU
`xuLyStartView`: `status='normal'`, `opNumber=2` (theo log HỢP nhất
`2` entry), `commitNumber=1` (theo thông điệp). View change HOÀN
tất.
::::

::::example{#tai-sao-dung-nullish}
`replica.commitNumber = td.commitNumber ?? replica.commitNumber;`
dùng `??` (nullish coalescing) chứ KHÔNG phải `||` — vì `commitNumber
= 0` LÀ một giá trị HỢP lệ VÀ có Ý nghĩa ("chưa commit gì cả"), trong
khi `0 || replica.commitNumber` sẽ SAI lầm coi `0` LÀ "rỗng" VÀ ghi
đè bằng giá trị CŨ. Đây LÀ đúng LOẠI lỗi q11 (đã liệt kê trong quy
ước dự án) — `??` chỉ "bỏ qua" KHI giá trị THẬT sự LÀ `undefined`
hoặc `null`.
::::

::::predict{#doan-khong-co-commit-number commitOnce}
Một `StartView` gửi TỚI mà KHÔNG mang `commitNumber` (trường ĐÓ
`undefined` — chỉ view change VỀ mặt log, không kèm THÔNG tin commit
mới). Replica NHẬN đang có `commitNumber=5` TỪ trước. SAU
`xuLyStartView`, `commitNumber` LÀ bao nhiêu?
:::opt{correct}
`5` — `td.commitNumber ?? replica.commitNumber` với `td.commitNumber`
LÀ `undefined` sẽ LẤY nhánh BÊN phải (`replica.commitNumber`, giá
trị CŨ `5`) — `commitNumber` GIỮ nguyên, không bị mất đi
:::
:::opt
`0` — thiếu `commitNumber` trong thông điệp nghĩa LÀ "chưa commit gì
cả", nên PHẢI reset về `0`
::why
Trực giác NÀY nhầm "thông điệp không MANG trường này" VỚI "trường
này LÀ 0" — nhưng `??` phân BIỆT rõ hai điều đó: chỉ `undefined`/`null`
mới kích hoạt nhánh DỰ phòng, KHÔNG có nhánh nào tự Ý đặt về `0`.

Chỗ lệch: NẾU code dùng `td.commitNumber!` (ép KIỂU, bỏ qua kiểm tra)
thay VÌ `?? replica.commitNumber`, giá trị SẼ thành `undefined` thật
— VÀ gán `commitNumber = undefined` LÀ một lỗi kiểu rõ RÀNG (bị `tsc
--strict` chặn), không phải ÂM thầm thành `0`. Dùng `??` chính LÀ để
TRÁNH cả hai lỗi này CÙNG lúc: vừa giữ kiểu ĐÚNG, vừa giữ giá TRỊ cũ
khi không CÓ tin mới.
::
:::
::::

::::code{#viet_xu_ly_start_view}
Hoàn thiện `xuLyStartView` — thay log, cập nhật `opNumber`,
`viewNumber`, `commitNumber` (dùng `??`), VÀ chuyển `status`.

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
interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'DoViewChange' | 'StartView' | 'YeuCauDongBo' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function opCuaLog(log: LogEntry[]): number {
  return log.length === 0 ? 0 : log[log.length - 1]!.opNumber;
}
function xuLyStartView(replica: Replica, td: ThongDiep): void {
  replica.log = td.log!;
  replica.opNumber = opCuaLog(replica.log);
  replica.viewNumber = td.viewNumber;
  ___
}

const backup2 = taoReplica(2, 3);
xuLyStartView(backup2, { loai: 'StartView', tu: 1, den: 2, viewNumber: 1, log: [{ opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 100 } }], commitNumber: 1 });
console.log(backup2.status);
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
interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'DoViewChange' | 'StartView' | 'YeuCauDongBo' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function opCuaLog(log: LogEntry[]): number {
  return log.length === 0 ? 0 : log[log.length - 1]!.opNumber;
}
function xuLyStartView(replica: Replica, td: ThongDiep): void {
  replica.log = td.log!;
  replica.opNumber = opCuaLog(replica.log);
  replica.viewNumber = td.viewNumber;
  replica.commitNumber = td.commitNumber ?? replica.commitNumber;
  replica.status = 'normal';
}

const backup2 = taoReplica(2, 3);
xuLyStartView(backup2, { loai: 'StartView', tu: 1, den: 2, viewNumber: 1, log: [{ opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 100 } }], commitNumber: 1 });
console.log(backup2.status);
```

```typescript title=test
function layStatus(r: Replica): Replica['status'] { return r.status; }
function layOpNumber(r: Replica): number { return r.opNumber; }
function layViewNumber(r: Replica): number { return r.viewNumber; }
function layCommitNumber(r: Replica): number { return r.commitNumber; }
function layLogLength(r: Replica): number { return r.log.length; }

const b3 = taoReplica(0, 3);
b3.status = 'view-change'; b3.viewNumber = 2;
const logMoi: LogEntry[] = [
  { opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 500 } },
  { opNumber: 2, bt: { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 200 } },
  { opNumber: 3, bt: { id: 3, debitAccountId: 2, creditAccountId: 1, amount: 50 } },
];
xuLyStartView(b3, { loai: 'StartView', tu: 1, den: 0, viewNumber: 2, log: logMoi, commitNumber: 2 });
if (layStatus(b3) !== 'normal') throw new Error("sau StartView, status phai chuyen ve 'normal'");
if (layOpNumber(b3) !== 3) throw new Error("opNumber phai la opNumber cua entry CUOI trong log moi (3)");
if (layViewNumber(b3) !== 2) throw new Error("viewNumber phai duoc cap nhat theo thong diep");
if (layCommitNumber(b3) !== 2) throw new Error("commitNumber phai duoc cap nhat theo thong diep");
if (layLogLength(b3) !== 3) throw new Error("log phai duoc THAY THE hoan toan bang log moi (3 phan tu)");
const b4 = taoReplica(1, 3);
b4.commitNumber = 5;
xuLyStartView(b4, { loai: 'StartView', tu: 0, den: 1, viewNumber: 3, log: [] });
if (layCommitNumber(b4) !== 5) throw new Error("neu thong diep KHONG mang commitNumber, phai GIU NGUYEN gia tri cu (5), khong duoc ghi de thanh undefined/0");
```

:::hints
- kind: attention
  body: "Cap nhat commitNumber bang '??' (giu gia tri cu neu thong diep khong mang theo), roi chuyen status='normal' -- hai dong."
- kind: strategy
  body: "replica.commitNumber = td.commitNumber ?? replica.commitNumber; replica.status = 'normal';"
- kind: one-line
  body: "replica.commitNumber = td.commitNumber ?? replica.commitNumber; replica.status = 'normal';"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "normal"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
View change hoàn tất. Nhưng NẾU một backup lạc hậu QUÁ xa (nhiều
entry, không phải chỉ VÀI) — StartView có phải LÀ cách DUY nhất để
cứu nó?
::::

::::reflect{#nghi-lai}
`xuLyStartView` khép LẠI "view change" (MASTERPLAN §9.2) — bài 9 phát
hiện, bài 10 gom PHIẾU, bài 11 chọn log, bài NÀY loan báo. Cả bốn bài
ráp lại thành ĐÚNG một chu trình: primary chết KHÔNG làm hệ thống
dừng vĩnh viễn, chỉ TẠM dừng cho tới khi đa số ĐỒNG thuận Ở view
tiếp theo.
::::

::::checkpoint{mastery=0.85}
::::
