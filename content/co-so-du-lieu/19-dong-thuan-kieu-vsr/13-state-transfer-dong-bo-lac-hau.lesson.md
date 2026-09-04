---
id: co-so-du-lieu.dong-thuan-kieu-vsr.state-transfer-dong-bo-lac-hau
title: "State Transfer — đồng bộ backup lạc hậu"
summary: "kiemTraCanDongBo(backup,opNumberMoiNhatDaBiet) so opNumber -- backup.opNumber < opNumberMoiNhatDaBiet thì lạc hậu, KHÔNG cần view change đầy đủ cho trường hợp này. xuLyPhanHoiDongBo chỉ thêm entry có opNumber LỚN HƠN backup.opNumber hiện tại (tránh trùng lặp), rồi cập nhật opNumber theo entry cuối. Backup có opNumber=1 nhận PhanHoiDongBo với 2 entry còn thiếu -> opNumber=3, log.length=3. Gửi lại một entry TRÙNG (opNumber=1) lẫn trong phản hồi: KHÔNG được thêm lại, log vẫn đúng 2 phần tử."
locale: vi
track: co-so-du-lieu
module: dong-thuan-kieu-vsr
order: 13
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.state-transfer-dong-bo-lac-hau]
requires: [db.start-view-hoan-tat-view-change]
concepts: [db.state-transfer-dong-bo-lac-hau]
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
StartView (bài trước) chữa MỌI lạc hậu — nhưng chỉ khi CẢ hệ thống
đang trải qua một cuộc VIEW change đầy đủ. Nếu primary VẪN sống,
nhưng CHỈ một backup bị mất mạng DÀI (lạc hậu vài entry), có CẦN cả
một cuộc bầu cử mới KHÔNG?
::::

::::explain{#dong-bo-khong-can-view-change}
`kiemTraCanDongBo(backup, opNumberMoiNhatDaBiet)` so SÁNH `opNumber`
đơn giản — `backup.opNumber < opNumberMoiNhatDaBiet` LÀ lạc hậu.
`xuLyPhanHoiDongBo` chỉ THÊM entry CÓ `opNumber` LỚN hơn `backup.
opNumber` hiện TẠI (tránh trùng LẶP nếu phản hồi vô tình LẶP lại một
entry đã CÓ), rồi cập nhật `opNumber` theo entry CUỐI. KHÔNG hề đụng
tới `status`/`viewNumber` — primary VẪN LÀ primary, KHÔNG có cuộc
bầu cử nào xảy RA:

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
function kiemTraCanDongBo(backup: Replica, opNumberMoiNhatDaBiet: number): boolean {
  return backup.opNumber < opNumberMoiNhatDaBiet;
}
function xuLyPhanHoiDongBo(backup: Replica, td: ThongDiep): void {
  const entryConThieu = td.log!;
  for (const entry of entryConThieu) if (entry.opNumber > backup.opNumber) backup.log.push(entry);
  backup.opNumber = opCuaLog(backup.log);
}

const backup2 = taoReplica(2, 3);
backup2.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 } });
backup2.opNumber = 1;
console.log("lac hau (biet toi op3)?", kiemTraCanDongBo(backup2, 3));
xuLyPhanHoiDongBo(backup2, { loai: 'PhanHoiDongBo', tu: 0, den: 2, viewNumber: 0, log: [
  { opNumber: 2, bt: { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 400 } },
  { opNumber: 3, bt: { id: 3, debitAccountId: 2, creditAccountId: 1, amount: 100 } },
] });
console.log("sau dong bo -- opNumber:", backup2.opNumber, "log.length:", backup2.log.length, "backup2.status:", backup2.status);
```

```text title=readonly
lac hau (biet toi op3)? true
sau dong bo -- opNumber: 3 log.length: 3 backup2.status: normal
```

`backup2` lạc HẬU (`opNumber=1` < `3`) — SAU `xuLyPhanHoiDongBo`,
`opNumber=3`, `log.length=3`, VÀ `status` VẪN LÀ `'normal'` suốt —
không hề CÓ view change NÀO xảy ra.
::::

::::example{#khac-view-change-o-diem-nao}
So VỚI bài 9-12 (view change): State Transfer LÀ một cuộc TRAO đổi
SONG phương, ĐƠN giản (một `YeuCauDongBo`, một `PhanHoiDongBo`),
KHÔNG cần quorum, KHÔNG đổi `viewNumber`, KHÔNG bầu primary mới —
CHỈ đơn thuần "SAO chép phần thiếu". Đây LÀ lý do MASTERPLAN §9.2 ghi
"ST" (State Transfer) TÁCH biệt hẳn khỏi "view change": chúng GIẢI
quyết hai VẤN đề khác nhau — view change xử LÝ "primary chết", state
transfer xử LÝ "một backup lỡ nhịp".
::::

::::predict{#doan-phan-hoi-co-entry-trung commitOnce}
`backup` ĐANG có `opNumber=1` (MỘT entry). Nó nhận `PhanHoiDongBo`
VỚI log gồm HAI entry: `opNumber=1` (TRÙNG, backup ĐÃ có SẴN) VÀ
`opNumber=2` (MỚI). SAU `xuLyPhanHoiDongBo`, `backup.log.length` LÀ
bao nhiêu?
:::opt{correct}
`2` — entry `opNumber=1` bị LOẠI (điều kiện `entry.opNumber >
backup.opNumber` LÀ `1 > 1 = false`, KHÔNG được đẩy VÀO); chỉ entry
`opNumber=2` (`2 > 1 = true`) được THÊM — tổng cộng `1` (đã có) `+
1` (mới) `= 2`, KHÔNG PHẢI `3`
:::
:::opt
`3` — phản hồi mang MẤY entry thì backup nhận ĐỦ mấy entry, cộng dồn
VÀO log đang có
::why
Trực giác NÀY bỏ QUA đúng điều kiện lọc `entry.opNumber >
backup.opNumber` — hàm KHÔNG "tin tưởng mù QUÁNG" nội dung phản hồi,
nó tự KIỂM tra từng entry TRƯỚC khi thêm.

Chỗ lệch: `for (const entry of entryConThieu) if (entry.opNumber >
backup.opNumber) backup.log.push(entry);` LỌC theo `backup.opNumber`
TẠI thời điểm SO sánh (KHÔNG đổi trong vòng lặp, vì `backup.opNumber`
chỉ cập nhật SAU vòng lặp) — bất kỳ entry NÀO có `opNumber` nhỏ hơn
HOẶC bằng giá trị BAN đầu ĐỀU bị bỏ qua, dù nó CÓ nằm trong phản hồi.
::
:::
::::

::::code{#viet_xu_ly_phan_hoi_dong_bo}
Hoàn thiện `xuLyPhanHoiDongBo` — chỉ thêm entry có `opNumber` lớn hơn
`backup.opNumber` hiện tại, rồi cập nhật `opNumber`.

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
function kiemTraCanDongBo(backup: Replica, opNumberMoiNhatDaBiet: number): boolean {
  return backup.opNumber < opNumberMoiNhatDaBiet;
}
function xuLyPhanHoiDongBo(backup: Replica, td: ThongDiep): void {
  const entryConThieu = td.log!;
  ___
  backup.opNumber = opCuaLog(backup.log);
}

const backup2 = taoReplica(2, 3);
backup2.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 100 } });
backup2.opNumber = 1;
xuLyPhanHoiDongBo(backup2, { loai: 'PhanHoiDongBo', tu: 0, den: 2, viewNumber: 0, log: [{ opNumber: 2, bt: { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 50 } }] });
console.log(backup2.opNumber);
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
function kiemTraCanDongBo(backup: Replica, opNumberMoiNhatDaBiet: number): boolean {
  return backup.opNumber < opNumberMoiNhatDaBiet;
}
function xuLyPhanHoiDongBo(backup: Replica, td: ThongDiep): void {
  const entryConThieu = td.log!;
  for (const entry of entryConThieu) if (entry.opNumber > backup.opNumber) backup.log.push(entry);
  backup.opNumber = opCuaLog(backup.log);
}

const backup2 = taoReplica(2, 3);
backup2.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 100 } });
backup2.opNumber = 1;
xuLyPhanHoiDongBo(backup2, { loai: 'PhanHoiDongBo', tu: 0, den: 2, viewNumber: 0, log: [{ opNumber: 2, bt: { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 50 } }] });
console.log(backup2.opNumber);
```

```typescript title=test
if (kiemTraCanDongBo(taoReplica(1, 3), 0) !== false) throw new Error("replica moi (opNumber=0), khong ai biet gi hon (0) -- KHONG lac hau, phai la false");
const b3 = taoReplica(1, 3);
b3.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 100 } });
b3.opNumber = 1;
if (kiemTraCanDongBo(b3, 1) !== false) throw new Error("opNumber=1 == 1 (da du) -- phai la false");
if (kiemTraCanDongBo(b3, 2) !== true) throw new Error("opNumber=1 < 2 -- lac hau, phai la true");
const b4 = taoReplica(2, 3);
b4.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 100 } });
b4.opNumber = 1;
xuLyPhanHoiDongBo(b4, { loai: 'PhanHoiDongBo', tu: 0, den: 2, viewNumber: 0, log: [
  { opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 100 } },
  { opNumber: 2, bt: { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 50 } },
] });
if (b4.log.length !== 2) throw new Error("entry TRUNG (opNumber <= da co) KHONG duoc them lai -- log phai co dung 2 phan tu, khong phai 3");
if (b4.opNumber !== 2) throw new Error("opNumber phai cap nhat theo entry CAO NHAT trong log sau khi dong bo (2)");
function layStatus(r: Replica): Replica['status'] { return r.status; }
if (layStatus(b4) !== 'normal') throw new Error("dong bo KHONG duoc doi status -- van phai la 'normal' (khong can view change)");
```

:::hints
- kind: attention
  body: "Voi moi entry trong entryConThieu, neu opNumber cua no LON HON backup.opNumber hien tai thi day vao backup.log -- mot dong (vong for + if)."
- kind: strategy
  body: "for (const entry of entryConThieu) if (entry.opNumber > backup.opNumber) backup.log.push(entry);"
- kind: one-line
  body: "for (const entry of entryConThieu) if (entry.opNumber > backup.opNumber) backup.log.push(entry);"
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
Mười ba bài — prepare/prepare_ok/commit, phát hiện lỗi, view change,
state transfer. Giờ ráp TẤT CẢ lại, tiêm lỗi ngẫu nhiên xuyên suốt,
và xác nhận mọi replica hội tụ đúng.
::::

::::reflect{#nghi-lai}
`xuLyPhanHoiDongBo` khép LẠI ba trụ cột CỦA q19 (MASTERPLAN §9.2):
prepare/prepare_ok/commit (bài 3-8), view change (bài 9-12), VÀ giờ
LÀ State Transfer — con đường THỨ ba, NHẸ hơn, để chữa lạc hậu KHÔNG
cần cả một cuộc bầu cử. Ba cơ chế NÀY, kết hợp VỚI hạ tầng tất định
(q17) VÀ tiêm lỗi (q18), LÀ đủ để BOSS (bài 14) mô phỏng một hệ VSR
hoàn CHỈNH sống sót qua sự cố thật.
::::

::::checkpoint{mastery=0.9}
::::
