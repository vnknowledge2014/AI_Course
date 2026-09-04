---
id: co-so-du-lieu.dong-thuan-kieu-vsr.do-view-change-chon-log-day-du-nhat
title: "DoViewChange — chọn log đầy đủ nhất"
summary: "chonLogDayDuNhat(cacLog) duyệt tuyến tính, chọn log có opCuaLog (opNumber của entry CUỐI, hoặc 0 nếu rỗng) LỚN NHẤT, dùng '>' nghiêm ngặt nên khi HOÀ opNumber, log đứng Ở VỊ TRÍ ĐẦU TIÊN trong mảng thắng -- tie-break tường minh, giống hệt tinh thần thuTuChen (q17 bài 6). Ba log [ngắn(1), đầy đủ(2), rỗng(0)]: chọn đúng log đầy đủ (opNumber=2). Hai log CÙNG opNumber=2 nhưng nội dung entry cuối khác nhau: log đứng ĐẦU mảng luôn thắng, đảo thứ tự mảng thì log MỚI đứng đầu thắng."
locale: vi
track: co-so-du-lieu
module: dong-thuan-kieu-vsr
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.do-view-change-chon-log-day-du-nhat]
requires: [db.bat-dau-view-change]
concepts: [db.do-view-change-chon-log-day-du-nhat]
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
Đủ quorum StartViewChange (bài trước). Giờ MỖI replica gửi LOG của
chính NÓ (kèm `opNumber` cao NHẤT) tới replica sẽ LÀ primary mới
(`viewNumber % N`, bài 1). Nhưng các LOG có thể KHÁC nhau — ai đầy đủ
NHẤT thắng?
::::

::::explain{#chon-log-day-du}
`chonLogDayDuNhat(cacLog)` duyệt TUYẾN tính, chọn log CÓ `opCuaLog`
(`opNumber` của entry CUỐI, hoặc `0` nếu rỗng) LỚN NHẤT — dùng `>`
NGHIÊM ngặt nên khi HOÀ `opNumber`, log ĐỨNG Ở vị trí ĐẦU tiên trong
mảng THẮNG (tie-break tường MINH, y hệt tinh thần `thuTuChen` q17 bài
6 — không phụ thuộc thuật toán TÌM max cụ thể):

```typescript title=readonly
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }

function opCuaLog(log: LogEntry[]): number {
  return log.length === 0 ? 0 : log[log.length - 1]!.opNumber;
}
function chonLogDayDuNhat(cacLog: LogEntry[][]): LogEntry[] {
  let logTotNhat = cacLog[0]!;
  for (let i = 1; i < cacLog.length; i++) {
    if (opCuaLog(cacLog[i]!) > opCuaLog(logTotNhat)) logTotNhat = cacLog[i]!;
  }
  return logTotNhat;
}

const bt = (id: number, amount: number): ButToan => ({ id, debitAccountId: 0, creditAccountId: 1, amount });
const logNgan: LogEntry[] = [{ opNumber: 1, bt: bt(1, 100) }];
const logDayDu: LogEntry[] = [{ opNumber: 1, bt: bt(1, 100) }, { opNumber: 2, bt: bt(2, 200) }];
const logRong: LogEntry[] = [];
const ketQua = chonLogDayDuNhat([logNgan, logDayDu, logRong]);
console.log("opNumber cao nhat duoc chon:", opCuaLog(ketQua), "so entry:", ketQua.length);
```

```text title=readonly
opNumber cao nhat duoc chon: 2 so entry: 2
```

Ba log — `1` entry, `2` entry, `0` entry — `chonLogDayDuNhat` chọn
ĐÚNG log `2` entry (`opNumber` CAO nhất). ĐÂY LÀ log sẽ trở thành bản
CHÍNH thức cho view MỚI (bài 12 dùng nó CHO `StartView`).
::::

::::example{#vi-sao-can-tie-break}
Nếu HAI log cùng ĐẠT `opNumber=2` nhưng NỘI dung entry cuối KHÁC
nhau (KHÔNG nên xảy ra Ở một hệ thống ĐÚNG, nhưng code PHẢI xử lý
được mọi ĐẦU vào) — luật `>` NGHIÊM ngặt đảm bảo LUÔN chọn log Ở VỊ
trí đầu tiên trong DANH sách nhận được, một cách NHẤT quán VÀ tái
lập được, thay VÌ "ngẫu nhiên" theo THỨ tự bộ nhớ hay thuật toán sắp
XẾP nội bộ — y hệt bài học của q17 bài 6.
::::

::::predict{#doan-doi-thu-tu-mang commitOnce}
Hai log `logA` VÀ `logB` cùng CÓ `opNumber=2` (nội DUNG entry cuối
khác nhau). Gọi `chonLogDayDuNhat([logA, logB])` được `logA`. Gọi
`chonLogDayDuNhat([logB, logA])` (ĐẢO thứ tự MẢNG đầu vào) được LOG
nào?
:::opt{correct}
`logB` — luật tie-break LÀ "log đứng ĐẦU mảng thắng", KHÔNG phải "logA
luôn thắng"; đảo thứ TỰ mảng đầu vào thì log ĐỨNG đầu MỚI (giờ LÀ
`logB`) sẽ thắng
:::
:::opt
Vẫn LÀ `logA` — MỘT khi đã "thắng" một LẦN Ở cùng `opNumber`, `logA`
LUÔN được ưu tiên bất kể thứ tự truyền VÀO
::why
Trực giác NÀY gán cho `logA` một "đặc QUYỀN" không hề tồn tại trong
CODE — hàm KHÔNG hề biết hay nhớ "logA" LÀ tên biến GÌ, nó chỉ thấy
các phần TỬ mảng theo VỊ trí.

Chỗ lệch: `chonLogDayDuNhat` khởi tạo `logTotNhat = cacLog[0]!` —
PHẦN tử Ở CHỈ số `0` của MẢNG truyền vào, bất kể biến ĐÓ tên gì Ở nơi
gọi. Đảo thứ tự MẢNG nghĩa LÀ đổi LUÔN "ai đứng Ở vị trí `0`" — VÀ
tie-break đi theo ĐÚNG vị trí đó.
::
:::
::::

::::code{#viet_chon_log_day_du}
Hoàn thiện `chonLogDayDuNhat` — so sánh `opCuaLog` của TỪNG log, cập
nhật `logTotNhat` khi tìm THẤY một log đầy đủ HƠN.

```typescript title=starter
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }

function opCuaLog(log: LogEntry[]): number {
  return log.length === 0 ? 0 : log[log.length - 1]!.opNumber;
}
function chonLogDayDuNhat(cacLog: LogEntry[][]): LogEntry[] {
  let logTotNhat = cacLog[0]!;
  for (let i = 1; i < cacLog.length; i++) {
    ___
  }
  return logTotNhat;
}

const bt = (id: number, amount: number): ButToan => ({ id, debitAccountId: 0, creditAccountId: 1, amount });
console.log(chonLogDayDuNhat([[{ opNumber: 1, bt: bt(1, 1) }], [{ opNumber: 1, bt: bt(1, 1) }, { opNumber: 2, bt: bt(2, 2) }]]).length);
```

```typescript title=solution
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }

function opCuaLog(log: LogEntry[]): number {
  return log.length === 0 ? 0 : log[log.length - 1]!.opNumber;
}
function chonLogDayDuNhat(cacLog: LogEntry[][]): LogEntry[] {
  let logTotNhat = cacLog[0]!;
  for (let i = 1; i < cacLog.length; i++) {
    if (opCuaLog(cacLog[i]!) > opCuaLog(logTotNhat)) logTotNhat = cacLog[i]!;
  }
  return logTotNhat;
}

const bt = (id: number, amount: number): ButToan => ({ id, debitAccountId: 0, creditAccountId: 1, amount });
console.log(chonLogDayDuNhat([[{ opNumber: 1, bt: bt(1, 1) }], [{ opNumber: 1, bt: bt(1, 1) }, { opNumber: 2, bt: bt(2, 2) }]]).length);
```

```typescript title=test
const bt2 = (id: number, amount: number): ButToan => ({ id, debitAccountId: 0, creditAccountId: 1, amount });
const logNgan: LogEntry[] = [{ opNumber: 1, bt: bt2(1, 100) }];
const logDayDu: LogEntry[] = [{ opNumber: 1, bt: bt2(1, 100) }, { opNumber: 2, bt: bt2(2, 200) }];
const logRong: LogEntry[] = [];
const r1 = chonLogDayDuNhat([logNgan, logDayDu, logRong]);
if (r1 !== logDayDu) throw new Error("phai chon dung THAM CHIEU log day du nhat (opNumber cao nhat)");
if (opCuaLog(r1) !== 2) throw new Error("log duoc chon phai co opNumber cao nhat la 2");
const r2 = chonLogDayDuNhat([logNgan]);
if (r2 !== logNgan) throw new Error("chi co 1 log thi phai chon dung no");
const logA: LogEntry[] = [{ opNumber: 1, bt: bt2(1, 100) }, { opNumber: 2, bt: bt2(11, 111) }];
const logB: LogEntry[] = [{ opNumber: 1, bt: bt2(1, 100) }, { opNumber: 2, bt: bt2(22, 222) }];
const r3 = chonLogDayDuNhat([logA, logB]);
if (r3 !== logA) throw new Error("hoa opNumber -- phai chon log DUNG VI TRI DAU TIEN trong danh sach (logA), khong phai logB");
const r4 = chonLogDayDuNhat([logB, logA]);
if (r4 !== logB) throw new Error("hoa opNumber -- doi thu tu danh sach thi log DUNG DAU (gio la logB) phai thang");
```

:::hints
- kind: attention
  body: "Neu opCuaLog cua log thu i lon hon opCuaLog cua logTotNhat thi cap nhat logTotNhat = cacLog[i]! -- mot dong."
- kind: strategy
  body: "if (opCuaLog(cacLog[i]!) > opCuaLog(logTotNhat)) logTotNhat = cacLog[i]!;"
- kind: one-line
  body: "if (opCuaLog(cacLog[i]!) > opCuaLog(logTotNhat)) logTotNhat = cacLog[i]!;"
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
Log đầy đủ nhất đã được chọn. Giờ primary mới phải LOAN báo nó cho
mọi replica còn sống — VÀ tuyên bố view mới chính thức bắt đầu.
::::

::::reflect{#nghi-lai}
`chonLogDayDuNhat` LÀ trái tim của "DoViewChange" (MASTERPLAN §9.2):
không đoán MÒ ai đúng, chỉ đơn giản CHỌN log NHIỀU bằng chứng NHẤT
(`opNumber` cao nhất) — VÀ khi hoà, dùng đúng LUẬT "ai đứng trước
thắng" như q17 bài 6 đã dạy cho sự kiện. Một khối XÂY nhỏ, tái sử
dụng Ý tưởng lớn.
::::

::::checkpoint{mastery=0.85}
::::
