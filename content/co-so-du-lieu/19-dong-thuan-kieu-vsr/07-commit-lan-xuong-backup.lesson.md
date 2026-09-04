---
id: co-so-du-lieu.dong-thuan-kieu-vsr.commit-lan-xuong-backup
title: "Commit lan xuống backup"
summary: "xuLyCommit áp dụng MỌI entry từ backup.commitNumber+1 tới td.commitNumber (tìm trong backup.log CỦA CHÍNH NÓ, đã có từ Prepare) vào cacTaiKhoan RIÊNG của backup, rồi cập nhật commitNumber -- primary gửi MỘT thông điệp Commit riêng (không piggyback) ngay sau khi tự commit, đơn giản hơn để kiểm chứng dù tốn thêm một round-trip mạng so với gộp vào Prepare kế tiếp. Backup nhận Commit(commitNumber=1): creditsPosted 0->1000. Nhận Commit(commitNumber=2) lần sau: KHÔNG áp dụng lại op1, chỉ thêm op2."
locale: vi
track: co-so-du-lieu
module: dong-thuan-kieu-vsr
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.commit-lan-xuong-backup]
requires: [db.primary-cho-quorum-roi-commit]
concepts: [db.commit-lan-xuong-backup]
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
Primary vừa commit THẬT (bài trước) — số dư ĐÃ đổi TRÊN primary. Nhưng
backup vẫn Ở trạng thái "đã ghi log, chưa áp dụng". Hai cách BÁO tin:
gộp VÀO Prepare kế tiếp (tiết kiệm), hay gửi RIÊNG (đơn giản). q19
chọn cách THỨ hai.
::::

::::explain{#commit-rieng}
`xuLyCommit` áp dụng MỌI entry TỪ `backup.commitNumber + 1` tới
`td.commitNumber` (tìm TRONG `backup.log` của CHÍNH nó — đã CÓ sẵn
từ lúc xử lý Prepare, bài 5) vào `cacTaiKhoan` RIÊNG của backup, rồi
cập nhật `commitNumber`. Primary gửi MỘT thông điệp `Commit` RIÊNG
biệt (không piggyback trên Prepare kế TIẾP) NGAY sau khi tự nó
commit:

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
function xuLyCommit(cacTaiKhoan: Map<number, TaiKhoan>, backup: Replica, td: ThongDiep): void {
  const commitMoi = td.commitNumber!;
  for (let op = backup.commitNumber + 1; op <= commitMoi; op++) {
    const entry = backup.log.find((e) => e.opNumber === op)!;
    apDungButToanThuong(cacTaiKhoan, entry.bt);
  }
  backup.commitNumber = commitMoi;
}

const NGUON_NGOAI = 0, TK1 = 1;
const ck = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, TK1]) ck.set(id, taoTaiKhoan(id));
const backup1 = taoReplica(1, 3);
backup1.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 } });
backup1.opNumber = 1;
xuLyCommit(ck, backup1, { loai: 'Commit', tu: 0, den: 1, viewNumber: 0, commitNumber: 1 });
console.log("commitNumber:", backup1.commitNumber, "creditsPosted:", ck.get(TK1)!.creditsPosted);
```

```text title=readonly
commitNumber: 1 creditsPosted: 1000
```

Trade-off: PIGGYBACK (gộp `commitNumber` vào Prepare TIẾP theo) tiết
kiệm một round-trip nhưng LÀM backup "biết chậm" nếu không CÓ request
mới nào tới; gửi RIÊNG (bài NÀY) tốn thêm một thông điệp mỗi commit
nhưng ĐƠN giản hơn để đọc VÀ kiểm chứng — VSR thật DÙNG piggyback vì
tối ưu THÔNG lượng; q19 chọn RIÊNG vì mục tiêu Ở đây LÀ hiểu đúng cơ
chế, KHÔNG phải tối ưu mạng.
::::

::::example{#backup-tu-tinh-lai}
`xuLyCommit` KHÔNG hề nhận trực tiếp bút TOÁN từ `Commit` — nó chỉ
nhận MỘT con số (`commitNumber`) VÀ tự TÌM entry tương ứng TRONG
`backup.log` của CHÍNH mình (đã có SẴN từ Prepare, bài 5). Đây LÀ lý
DO `Commit` LÀ một thông điệp NHẸ — nó chỉ LÀ "tín hiệu", không mang
LẠI dữ liệu đã CÓ sẵn Ở nơi nhận.
::::

::::predict{#doan-hai-lan-commit commitOnce}
Gọi `xuLyCommit` với `commitNumber=1` (backup ÁP dụng `op1`), RỒI gọi
LẦN nữa với `commitNumber=2` (backup ĐÃ có `op1` VÀ `op2` trong log
từ TRƯỚC). Lần gọi THỨ hai có áp dụng LẠI `op1` không?
:::opt{correct}
KHÔNG — vòng `for` bắt đầu TỪ `backup.commitNumber + 1`, VÀ sau lần
gọi ĐẦU, `backup.commitNumber` ĐÃ LÀ `1`, nên lần THỨ hai chỉ lặp từ
`2` đến `2` — CHỈ áp dụng `op2`, `op1` không bị ĐỤNG tới lần nào nữa
:::
:::opt
CÓ — `xuLyCommit` luôn áp DỤNG lại TỪ đầu log mỗi lần được GỌI, để
đảm bảo KHÔNG bỏ sót
::why
Trực giác NÀY nhầm "an TOÀN" với "lặp lại TỪ đầu" — nhưng code THẬT
dùng ĐÚNG `backup.commitNumber` (trạng thái ĐÃ tiến tới) làm điểm
BẮT đầu, không phải hằng số `1`.

Chỗ lệch: `backup.commitNumber` được CẬP nhật Ở CUỐI mỗi lần gọi
(`backup.commitNumber = commitMoi;`), nên lần gọi SAU luôn "nhớ" đã
đi tới ĐÂU. Nếu áp dụng lại TỪ đầu mỗi lần, `op1` sẽ bị CỘNG dồn hai
lần — chính XÁC loại lỗi mà q16 bài 11 (idempotent) đã dạy cách
tránh, Ở đây tránh được BẰNG cách nhớ đúng ĐIỂM dừng thay vì một tập
`id` đã xử lý.
::
:::
::::

::::code{#viet_xu_ly_commit}
Hoàn thiện `xuLyCommit` — với MỖI `op` từ `backup.commitNumber + 1`
tới `commitMoi`, tìm entry TRONG log VÀ áp dụng.

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
function xuLyCommit(cacTaiKhoan: Map<number, TaiKhoan>, backup: Replica, td: ThongDiep): void {
  const commitMoi = td.commitNumber!;
  for (let op = backup.commitNumber + 1; op <= commitMoi; op++) {
    ___
  }
  backup.commitNumber = commitMoi;
}

const ck = new Map<number, TaiKhoan>();
ck.set(0, taoTaiKhoan(0)); ck.set(1, taoTaiKhoan(1));
const backup1 = taoReplica(1, 3);
backup1.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 } });
xuLyCommit(ck, backup1, { loai: 'Commit', tu: 0, den: 1, viewNumber: 0, commitNumber: 1 });
console.log(ck.get(1)!.creditsPosted);
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
function xuLyCommit(cacTaiKhoan: Map<number, TaiKhoan>, backup: Replica, td: ThongDiep): void {
  const commitMoi = td.commitNumber!;
  for (let op = backup.commitNumber + 1; op <= commitMoi; op++) {
    const entry = backup.log.find((e) => e.opNumber === op)!;
    apDungButToanThuong(cacTaiKhoan, entry.bt);
  }
  backup.commitNumber = commitMoi;
}

const ck = new Map<number, TaiKhoan>();
ck.set(0, taoTaiKhoan(0)); ck.set(1, taoTaiKhoan(1));
const backup1 = taoReplica(1, 3);
backup1.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 } });
xuLyCommit(ck, backup1, { loai: 'Commit', tu: 0, den: 1, viewNumber: 0, commitNumber: 1 });
console.log(ck.get(1)!.creditsPosted);
```

```typescript title=test
const NGUON_NGOAI = 0, TK1 = 1;
const ck2 = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, TK1]) ck2.set(id, taoTaiKhoan(id));
const b2 = taoReplica(2, 3);
b2.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 } });
b2.log.push({ opNumber: 2, bt: { id: 2, debitAccountId: TK1, creditAccountId: NGUON_NGOAI, amount: 300 } });
b2.opNumber = 2;
xuLyCommit(ck2, b2, { loai: 'Commit', tu: 0, den: 2, viewNumber: 0, commitNumber: 2 });
if (b2.commitNumber !== 2) throw new Error("commitNumber phai duoc cap nhat DUNG BANG commitNumber trong thong diep");
if (ck2.get(TK1)!.creditsPosted !== 1000) throw new Error("op1 phai duoc ap dung");
if (ck2.get(TK1)!.debitsPosted !== 300) throw new Error("op2 phai duoc ap dung");
if (ck2.get(NGUON_NGOAI)!.debitsPosted !== 1000 || ck2.get(NGUON_NGOAI)!.creditsPosted !== 300) throw new Error("CA HAI but toan phai duoc ap dung THEO DUNG THU TU (khong duoc bo sot op nao)");
const ck3 = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, TK1]) ck3.set(id, taoTaiKhoan(id));
const b3 = taoReplica(1, 3);
b3.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 } });
b3.log.push({ opNumber: 2, bt: { id: 2, debitAccountId: TK1, creditAccountId: NGUON_NGOAI, amount: 300 } });
b3.opNumber = 2;
xuLyCommit(ck3, b3, { loai: 'Commit', tu: 0, den: 1, viewNumber: 0, commitNumber: 1 });
if (ck3.get(TK1)!.creditsPosted !== 1000) throw new Error("commit lan 1 (den op1) phai ap dung dung op1");
xuLyCommit(ck3, b3, { loai: 'Commit', tu: 0, den: 1, viewNumber: 0, commitNumber: 2 });
if (ck3.get(TK1)!.creditsPosted !== 1000) throw new Error("commit lan 2 KHONG duoc ap dung LAI op1 (creditsPosted phai VAN la 1000, khong phai 2000)");
if (ck3.get(TK1)!.debitsPosted !== 300) throw new Error("commit lan 2 phai ap dung op2 MOI (debitsPosted=300)");
```

:::hints
- kind: attention
  body: "Tim entry theo opNumber trong backup.log, ap dung but toan -- hai dong trong vong for."
- kind: strategy
  body: "const entry = backup.log.find((e) => e.opNumber === op)!; apDungButToanThuong(cacTaiKhoan, entry.bt);"
- kind: one-line
  body: "const entry = backup.log.find((e) => e.opNumber === op)!; apDungButToanThuong(cacTaiKhoan, entry.bt);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Prepare/PrepareOk/Commit hoàn CHỈNH cho đường ĐI thuận buồm xuôi gió.
Nhưng mạng KHÔNG luôn thuận buồm — q18 đã dạy Điều đó.
::::

::::reflect{#nghi-lai}
`xuLyCommit` khép LẠI "prepare/prepare_ok/commit" (MASTERPLAN §9.2):
primary QUYẾT định (bài 6), backup THI hành (bài NÀY) — dựa TRÊN
đúng một con SỐ (`commitNumber`), không cần gửi LẠI dữ liệu đã CÓ.
Bảy bài đầu ĐÃ xây xong đường Đi "khoẻ mạnh" — sáu bài TIẾP theo sẽ
hỏi: nếu MẠNG không hợp tác, hoặc PRIMARY biến mất, thì SAO?
::::

::::checkpoint{mastery=0.85}
::::
