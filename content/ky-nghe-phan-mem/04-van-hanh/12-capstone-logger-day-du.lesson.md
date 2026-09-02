---
id: ky-nghe-phan-mem.van-hanh.capstone-logger-day-du
title: "Capstone: Logger đầy đủ — cấu trúc + level + correlation"
summary: "Bài chốt cụm 3: ghép taoNhatKy (bài 9) + locTheoLevel (bài 10) + locTheoRequestId (bài 11) — mô phỏng MỘT request thật ghi nhiều dòng log (bắt đầu, lỗi, cảnh báo) CÙNG requestId, xen lẫn với log của request khác. layLoiCuaRequest ghép cả hai bộ lọc — kiểm lọc ĐÚNG theo level VÀ theo request, không lẫn lộn giữa hai request."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [vh.gate-boss-structured-logging]
requires: [vh.request-id-correlation]
concepts: [vh.gate-boss-structured-logging]
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
Bài chốt cụm 3. Ghép nhà máy log (bài 9) + lọc theo level (bài 10) +
lọc theo request (bài 11) thành MỘT logger đầy đủ.
::::

::::explain{#ghep-tron-logger}
`layLoiCuaRequest` GHÉP CẢ HAI bộ lọc: TÌM log CỦA request TRƯỚC
(`locTheoRequestId`), RỒI lọc TIẾP chỉ giữ `"error"` (`locTheoLevel`)
— trả LẠI đúng "MỌI lỗi của MỘT request CỤ THỂ":

```typescript title=readonly
type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; requestId?: string; [truong: string]: unknown };

function taoNhatKy(): { ghi: (level: LogLevel, msg: string, extra?: Record<string, unknown>) => void; layNhatKy: () => NhatKy[] } {
  const cacLog: NhatKy[] = [];
  return {
    ghi: (level, msg, extra = {}) => { cacLog.push({ level, msg, ...extra }); },
    layNhatKy: () => cacLog,
  };
}

function locTheoLevel(cacLog: NhatKy[], cacLevelMuon: LogLevel[]): NhatKy[] {
  return cacLog.filter((log) => cacLevelMuon.includes(log.level));
}

function locTheoRequestId(cacLog: NhatKy[], id: string): NhatKy[] {
  return cacLog.filter((log) => log.requestId === id);
}

const logger = taoNhatKy();

logger.ghi("info", "bat dau", { requestId: "req-1" });
logger.ghi("info", "bat dau", { requestId: "req-2" });
logger.ghi("error", "loi thanh toan", { requestId: "req-1" });
logger.ghi("info", "hoan tat", { requestId: "req-2" });
logger.ghi("info", "hoan tat", { requestId: "req-1" });

const logReq1 = locTheoRequestId(logger.layNhatKy(), "req-1");
console.log(logReq1.length);

const loiReq1 = locTheoLevel(logReq1, ["error"]);
console.log(loiReq1.length);
console.log(loiReq1[0]?.msg);

const tatCaLoi = locTheoLevel(logger.layNhatKy(), ["error"]);
console.log(tatCaLoi.length);
```

```text title=readonly
3
1
loi thanh toan
1
```

`req-1` CÓ ba dòng log (bat dau, lỗi, hoàn tất) — lọc TIẾP theo
`error` CHỈ CÒN đúng MỘT. TOÀN BỘ hệ thống CŨNG chỉ CÓ đúng MỘT lỗi
(ĐÚNG dòng ĐÓ) — HAI cách lọc cho CÙNG kết quả VÌ `req-2` KHÔNG có
lỗi NÀO.
::::

::::example{#thu-tu-loc-khong-quan-trong}
Lọc THEO request TRƯỚC RỒI theo level, hay NGƯỢC LẠI — kết quả GIỐNG
HỆT NHAU (HAI bộ lọc GIAO HOÁN được):

```typescript title=readonly
type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; requestId?: string; [truong: string]: unknown };
function taoNhatKy(): { ghi: (level: LogLevel, msg: string, extra?: Record<string, unknown>) => void; layNhatKy: () => NhatKy[] } {
  const cacLog: NhatKy[] = [];
  return {
    ghi: (level, msg, extra = {}) => { cacLog.push({ level, msg, ...extra }); },
    layNhatKy: () => cacLog,
  };
}
function locTheoLevel(cacLog: NhatKy[], cacLevelMuon: LogLevel[]): NhatKy[] {
  return cacLog.filter((log) => cacLevelMuon.includes(log.level));
}
function locTheoRequestId(cacLog: NhatKy[], id: string): NhatKy[] {
  return cacLog.filter((log) => log.requestId === id);
}

const logger = taoNhatKy();
logger.ghi("info", "bat dau", { requestId: "req-1" });
logger.ghi("info", "bat dau", { requestId: "req-2" });
logger.ghi("error", "loi thanh toan", { requestId: "req-1" });
logger.ghi("info", "hoan tat", { requestId: "req-2" });
logger.ghi("info", "hoan tat", { requestId: "req-1" });

const loiReq1Nguoc = locTheoRequestId(locTheoLevel(logger.layNhatKy(), ["error"]), "req-1");
console.log(loiReq1Nguoc.length);
console.log(loiReq1Nguoc[0]?.msg);
```

```text title=readonly
1
loi thanh toan
```

Lọc `error` TRƯỚC (CẢ HỆ THỐNG chỉ CÒN MỘT dòng), RỒI lọc `req-1`
(dòng ĐÓ đúng LÀ thuộc `req-1`) — KẾT QUẢ giống HỆT cách lọc `req-1`
TRƯỚC. Filter TRÊN CÁC TRƯỜNG ĐỘC LẬP LUÔN giao HOÁN được.
::::

::::predict{#doan-canh-bao-khong-la-loi commitOnce}
```typescript
type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; requestId?: string; [truong: string]: unknown };
function taoNhatKy(): { ghi: (level: LogLevel, msg: string, extra?: Record<string, unknown>) => void; layNhatKy: () => NhatKy[] } {
  const cacLog: NhatKy[] = [];
  return {
    ghi: (level, msg, extra = {}) => { cacLog.push({ level, msg, ...extra }); },
    layNhatKy: () => cacLog,
  };
}
function locTheoLevel(cacLog: NhatKy[], cacLevelMuon: LogLevel[]): NhatKy[] {
  return cacLog.filter((log) => cacLevelMuon.includes(log.level));
}
function locTheoRequestId(cacLog: NhatKy[], id: string): NhatKy[] {
  return cacLog.filter((log) => log.requestId === id);
}

const logger = taoNhatKy();
logger.ghi("info", "start", { requestId: "X" });
logger.ghi("warn", "cham", { requestId: "X" });
logger.ghi("debug", "chi tiet noi bo", { requestId: "X" });
logger.ghi("error", "that bai", { requestId: "Y" });

const logX = locTheoRequestId(logger.layNhatKy(), "X");
const canhBaoVaLoiCuaX = locTheoLevel(logX, ["warn", "error"]);
console.log(canhBaoVaLoiCuaX.length);
```

Dòng cuối in ra gì?

:::opt{correct}
`1`
:::

:::opt
`2` — vì `canhBaoVaLoiCuaX` lọc theo HAI level (`"warn"` VÀ
`"error"`) CÙNG lúc, VÀ trong TOÀN BỘ log CỦA logger CÓ đúng HAI
dòng khớp HAI level ĐÓ (`"cham"` VÀ `"that bai"`) — bất KỂ dòng nào
thuộc request NÀO
::why
Gần đúng ở việc bạn ĐẾM ĐÚNG có HAI dòng log trên TOÀN BỘ hệ thống
khớp `"warn"`/`"error"` (`"cham"` VÀ `"that bai"`) — quan sát ĐÓ về
TOÀN BỘ dữ liệu chính xác.

Chỗ lệch: `canhBaoVaLoiCuaX` KHÔNG lọc TRÊN `logger.layNhatKy()`
(TOÀN BỘ log) — nó lọc TRÊN `logX` (BIẾN ĐÃ được TÍNH TRƯỚC ĐÓ bằng
`locTheoRequestId(..., "X")`, CHỈ CÒN BA dòng THUỘC `"X"`). Dòng
`"that bai"` (`level: "error"`) THUỘC `requestId: "Y"` — nó ĐÃ BỊ
LOẠI RA khỏi `logX` TỪ TRƯỚC, KHÔNG BAO GIỜ được đưa VÀO bước lọc
level SAU ĐÓ. CHỈ `"cham"` (`"warn"`, thuộc `"X"`) khớp — kết quả
LÀ `1`.
::
:::

:::opt
Máy báo lỗi biên dịch — `locTheoLevel(logX, ["warn", "error"])`
truyền HAI phần tử VÀO tham số `cacLevelMuon: LogLevel[]`, NHƯNG
`locTheoLevel` (bài 10) CHỈ được TEST với MỘT hoặc HAI phần tử
TRƯỚC ĐÓ theo THỨ TỰ KHÁC (`["warn", "error"]` so với `["error"]`),
TypeScript đòi THỨ TỰ phần tử phải NHẤT QUÁN GIỮA các lần gọi
::why
Gần đúng ở việc bạn để ý các lần gọi TRƯỚC dùng THỨ TỰ phần tử KHÁC
NHAU (`["warn", "error"]` Ở đây, `["error"]` Ở ví dụ TRƯỚC) — một
quan sát ĐÚNG về SỰ ĐA DẠNG lời gọi.

Chỗ lệch: `LogLevel[]` LÀ một **MẢNG** — TypeScript (VÀ
`.includes()` bên TRONG `locTheoLevel`) HOÀN TOÀN KHÔNG quan tâm
THỨ TỰ phần tử TRONG mảng, CHỈ quan tâm GIÁ TRỊ nào CÓ MẶT. Gọi
`locTheoLevel` VỚI BAO NHIÊU phần tử, THEO THỨ TỰ NÀO, Ở BAO NHIÊU
lần gọi KHÁC NHAU ĐỀU hợp lệ — biên dịch SẠCH.
::
:::
::::

::::code{#viet_capstone_logger}
Hoàn thiện `layLoiCuaRequest` — GHÉP `locTheoRequestId` VÀ
`locTheoLevel` (chỉ giữ `"error"`) thành MỘT hàm.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; requestId?: string; [truong: string]: unknown };

function taoNhatKy(): { ghi: (level: LogLevel, msg: string, extra?: Record<string, unknown>) => void; layNhatKy: () => NhatKy[] } {
  const cacLog: NhatKy[] = [];
  return {
    ghi: (level, msg, extra = {}) => { cacLog.push({ level, msg, ...extra }); },
    layNhatKy: () => cacLog,
  };
}

function locTheoLevel(cacLog: NhatKy[], cacLevelMuon: LogLevel[]): NhatKy[] {
  return cacLog.filter((log) => cacLevelMuon.includes(log.level));
}

function locTheoRequestId(cacLog: NhatKy[], id: string): NhatKy[] {
  return cacLog.filter((log) => log.requestId === id);
}

function layLoiCuaRequest(cacLog: NhatKy[], id: string): NhatKy[] {
  const logCuaRequest = ___;
  return ___;
}

const logger = taoNhatKy();
logger.ghi("info", "bat dau", { requestId: "req-1" });
logger.ghi("error", "loi", { requestId: "req-1" });
logger.ghi("error", "loi khac", { requestId: "req-2" });

const loiCuaReq1 = layLoiCuaRequest(logger.layNhatKy(), "req-1");
assertEqual(loiCuaReq1.length, 1, "chi lay loi cua req-1");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; requestId?: string; [truong: string]: unknown };

function taoNhatKy(): { ghi: (level: LogLevel, msg: string, extra?: Record<string, unknown>) => void; layNhatKy: () => NhatKy[] } {
  const cacLog: NhatKy[] = [];
  return {
    ghi: (level, msg, extra = {}) => { cacLog.push({ level, msg, ...extra }); },
    layNhatKy: () => cacLog,
  };
}

function locTheoLevel(cacLog: NhatKy[], cacLevelMuon: LogLevel[]): NhatKy[] {
  return cacLog.filter((log) => cacLevelMuon.includes(log.level));
}

function locTheoRequestId(cacLog: NhatKy[], id: string): NhatKy[] {
  return cacLog.filter((log) => log.requestId === id);
}

function layLoiCuaRequest(cacLog: NhatKy[], id: string): NhatKy[] {
  const logCuaRequest = locTheoRequestId(cacLog, id);
  return locTheoLevel(logCuaRequest, ["error"]);
}

const logger = taoNhatKy();
logger.ghi("info", "bat dau", { requestId: "req-1" });
logger.ghi("error", "loi", { requestId: "req-1" });
logger.ghi("error", "loi khac", { requestId: "req-2" });

const loiCuaReq1 = layLoiCuaRequest(logger.layNhatKy(), "req-1");
assertEqual(loiCuaReq1.length, 1, "chi lay loi cua req-1");
```

```typescript title=test
assertEqual(loiCuaReq1[0]!.msg, "loi", "dung noi dung loi");

logger.ghi("warn", "canh bao", { requestId: "req-1" });
const loiCuaReq1Lan2 = layLoiCuaRequest(logger.layNhatKy(), "req-1");
assertEqual(loiCuaReq1Lan2.length, 1, "canh bao khong tinh la loi");

const loiCuaReq3 = layLoiCuaRequest(logger.layNhatKy(), "req-khong-ton-tai");
assertEqual(loiCuaReq3.length, 0, "request khong ton tai -- khong co loi");

const loiCuaReq2 = layLoiCuaRequest(logger.layNhatKy(), "req-2");
assertEqual(loiCuaReq2.length, 1, "loi rieng cua req-2");
assertEqual(loiCuaReq2[0]!.msg, "loi khac", "dung noi dung loi cua req-2");
```

:::hints
- kind: attention
  body: "logCuaRequest: lọc theo requestId TRƯỚC. Kết quả cuối: lọc TIẾP logCuaRequest, chỉ giữ level \"error\"."
- kind: strategy
  body: 'locTheoRequestId(cacLog, id) : locTheoLevel(logCuaRequest, ["error"])'
- kind: one-line
  body: '___ (logCuaRequest) = locTheoRequestId(cacLog, id)\n___ (kết quả) = locTheoLevel(logCuaRequest, ["error"])'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "PASS"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cụm 3 hoàn tất: log có cấu trúc, lọc theo level, correlation theo
request, capstone ghép cả ba. Cụm tiếp theo: đo BAO NHIÊU, không
chỉ ghi lại chuyện gì đã xảy ra.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Log ghi lại TỪNG SỰ KIỆN riêng lẻ. Nếu MUỐN biết "hệ thống ĐANG có
BAO NHIÊU request MỖI giây" hay "tỉ lệ lỗi LÀ bao nhiêu PHẦN TRĂM" —
đếm TỪNG dòng log BẰNG TAY có ổn không?
::::

::::checkpoint{mastery=0.8}
::::
