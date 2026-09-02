---
id: ky-nghe-phan-mem.van-hanh.console-log-khong-du-cho-production
title: "console.log KHÔNG đủ cho production — log CÓ CẤU TRÚC"
summary: "console.log(\"loi roi\") vô dụng khi cần tìm MỘT sự kiện cụ thể giữa hàng triệu dòng log production — cần STRUCTURED LOGGING: MỖI dòng log là một OBJECT có trường rõ ràng (level, msg, các field ngữ cảnh), KHÔNG PHẢI một chuỗi tự do. taoNhatKy(): {ghi, layNhatKy} — nhà máy đóng gói mảng log nội bộ, giống taoEventBus T5.4 bài 6."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [vh.structured-logging-intro]
requires: [vh.gate-boss-deploy-rollback]
concepts: [vh.structured-logging-intro]
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
Cụm mới. Production ghi HÀNG TRIỆU dòng log MỖI ngày. `console.log("có
lỗi rồi")` — TÌM lại đúng SỰ KIỆN đó SAU NÀY thế nào?
::::

::::explain{#structured-logging}
`console.log("loi roi")` VÔ DỤNG khi CẦN tìm MỘT sự kiện CỤ THỂ giữa
HÀNG TRIỆU dòng log — Production CẦN **structured logging**: MỖI
dòng log LÀ một OBJECT có TRƯỜNG rõ ràng (`level`, `msg`, các field
NGỮ CẢNH), KHÔNG PHẢI một CHUỖI tự do:

```typescript title=readonly
type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; [truong: string]: unknown };

function taoNhatKy(): { ghi: (level: LogLevel, msg: string, extra?: Record<string, unknown>) => void; layNhatKy: () => NhatKy[] } {
  const cacLog: NhatKy[] = [];
  return {
    ghi: (level, msg, extra = {}) => {
      cacLog.push({ level, msg, ...extra });
    },
    layNhatKy: () => cacLog,
  };
}

const logger = taoNhatKy();
logger.ghi("info", "Don hang da tao", { donHangId: "D1", soTien: 300 });
logger.ghi("error", "Thanh toan that bai", { donHangId: "D1", lyDo: "the bi tu choi" });

console.log(logger.layNhatKy().length);
console.log(logger.layNhatKy()[0]);
```

```text title=readonly
2
{"level":"info","msg":"Don hang da tao","donHangId":"D1","soTien":300}
```

`taoNhatKy` LÀ MỘT nhà máy (GIỐNG `taoEventBus` T5.4 bài 6) — đóng
gói mảng `cacLog` NỘI BỘ, lộ RA đúng HAI thao tác: `ghi` (thêm dòng
log MỚI) VÀ `layNhatKy` (đọc TOÀN BỘ). MỖI dòng log LÀ MỘT object,
KHÔNG PHẢI CHUỖI văn bản tự do.
::::

::::example{#tim-lai-theo-truong}
LỢI ÍCH THẬT: TÌM LẠI log THEO MỘT TRƯỜNG CỤ THỂ (như
`console.log` KHÔNG BAO GIỜ làm được):

```typescript title=readonly
type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; [truong: string]: unknown };
function taoNhatKy(): { ghi: (level: LogLevel, msg: string, extra?: Record<string, unknown>) => void; layNhatKy: () => NhatKy[] } {
  const cacLog: NhatKy[] = [];
  return {
    ghi: (level, msg, extra = {}) => { cacLog.push({ level, msg, ...extra }); },
    layNhatKy: () => cacLog,
  };
}

const logger = taoNhatKy();
logger.ghi("info", "Don hang tao", { donHangId: "A" });
logger.ghi("info", "Don hang tao", { donHangId: "B" });
logger.ghi("error", "Loi xu ly", { donHangId: "A" });

const logCuaA = logger.layNhatKy().filter((log) => log.donHangId === "A");
console.log(logCuaA.length);
```

```text title=readonly
2
```

`filter` TÌM ĐÚNG HAI dòng log CÓ `donHangId === "A"` — LÀM ĐƯỢC vì
`donHangId` LÀ MỘT TRƯỜNG THẬT của OBJECT, KHÔNG PHẢI ký tự CHÔN
TRONG một chuỗi văn bản.
::::

::::predict{#doan-hai-logger-doc-lap commitOnce}
```typescript
type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; [truong: string]: unknown };
function taoNhatKy(): { ghi: (level: LogLevel, msg: string, extra?: Record<string, unknown>) => void; layNhatKy: () => NhatKy[] } {
  const cacLog: NhatKy[] = [];
  return {
    ghi: (level, msg, extra = {}) => { cacLog.push({ level, msg, ...extra }); },
    layNhatKy: () => cacLog,
  };
}

const logger1 = taoNhatKy();
const logger2 = taoNhatKy();

logger1.ghi("info", "log 1");
logger1.ghi("info", "log 2");
logger2.ghi("warn", "log rieng");

console.log(logger1.layNhatKy().length);
console.log(logger2.layNhatKy().length);
```

Hai dòng cuối in ra gì?

:::opt{correct}
`2` rồi `1`
:::

:::opt
`3` rồi `3` — vì `taoNhatKy` LÀ một hàm DUY NHẤT (được ĐỊNH NGHĨA
MỘT lần), NÊN MỌI lời gọi `taoNhatKy()` ĐỀU TRẢ VỀ CÙNG một mảng
`cacLog` DÙNG CHUNG — TẤT CẢ log (`logger1` VÀ `logger2`) ĐỀU đổ
vào MỘT nơi
::why
Gần đúng ở việc bạn nhớ ĐÚNG `taoNhatKy` chỉ được ĐỊNH NGHĨA MỘT
lần trong code — quan sát ĐÓ về SỐ LẦN khai báo hàm chính xác.

Chỗ lệch: MỖI LẦN **GỌI** `taoNhatKy()` (KHÔNG PHẢI mỗi lần ĐỊNH
NGHĨA) LÀ một lần CHẠY TOÀN BỘ thân hàm TỪ ĐẦU — dòng `const cacLog:
NhatKy[] = [];` TẠO một mảng **MỚI HOÀN TOÀN** MỖI lần gọi (GIỐNG
`taoEventBus` T5.4 bài 6, `taoBoDem` T5.4 bài 3). `logger1` VÀ
`logger2` LÀ HAI object ĐỘC LẬP, MỖI cái đóng gói `cacLog` RIÊNG CỦA
NÓ — `logger1.ghi` KHÔNG BAO GIỜ chạm tới `cacLog` của `logger2`.
::
:::

:::opt
Máy báo lỗi biên dịch — `taoNhatKy()` được gọi HAI LẦN (tạo
`logger1` VÀ `logger2`), NHƯNG kiểu trả về của `taoNhatKy` chứa MỘT
HÀM (`ghi`), TypeScript CẤM tạo HAI GIÁ TRỊ CHỨA hàm TỪ CÙNG một
định nghĩa nhà máy
::why
Gần đúng ở việc bạn để ý kiểu trả VỀ của `taoNhatKy` CHỨA property
`ghi` LÀ MỘT HÀM — một quan sát ĐÚNG về HÌNH DẠNG kiểu trả về.

Chỗ lệch: HOÀN TOÀN KHÔNG có quy tắc "CẤM gọi MỘT nhà máy NHIỀU
lần" — NGƯỢC LẠI, đây CHÍNH LÀ LÝ DO các hàm nhà máy TỒN TẠI: gọi
NHIỀU lần để tạo NHIỀU thực thể ĐỘC LẬP. Biên dịch SẠCH.
::
:::
::::

::::code{#viet_tao_nhat_ky}
Hoàn thiện `ghi` — đẩy MỘT dòng log MỚI (kết hợp `level`, `msg`, VÀ
CÁC trường `extra`) vào mảng NỘI BỘ.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; [truong: string]: unknown };

function taoNhatKy(): { ghi: (level: LogLevel, msg: string, extra?: Record<string, unknown>) => void; layNhatKy: () => NhatKy[] } {
  const cacLog: NhatKy[] = [];
  return {
    ghi: (level, msg, extra = {}) => {
      ___;
    },
    layNhatKy: () => cacLog,
  };
}

const logger = taoNhatKy();
logger.ghi("info", "test");
assertEqual(logger.layNhatKy().length, 1, "mot log duoc ghi");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; [truong: string]: unknown };

function taoNhatKy(): { ghi: (level: LogLevel, msg: string, extra?: Record<string, unknown>) => void; layNhatKy: () => NhatKy[] } {
  const cacLog: NhatKy[] = [];
  return {
    ghi: (level, msg, extra = {}) => {
      cacLog.push({ level, msg, ...extra });
    },
    layNhatKy: () => cacLog,
  };
}

const logger = taoNhatKy();
logger.ghi("info", "test");
assertEqual(logger.layNhatKy().length, 1, "mot log duoc ghi");
```

```typescript title=test
assertEqual(logger.layNhatKy()[0]!.level, "info", "level dung");
assertEqual(logger.layNhatKy()[0]!.msg, "test", "msg dung");

const logger2 = taoNhatKy();
logger2.ghi("error", "loi", { maLoi: "E1" });
assertEqual(logger2.layNhatKy()[0]!.maLoi, "E1", "extra field duoc luu");
assertEqual(logger.layNhatKy().length, 1, "logger dau khong bi anh huong boi logger hai");
```

:::hints
- kind: attention
  body: "Đẩy vào cacLog một object gộp level, msg, VÀ trải TẤT CẢ trường của extra vào cùng cấp."
- kind: strategy
  body: "cacLog.push({ level, msg, ...extra })"
- kind: one-line
  body: '___ = cacLog.push({ level, msg, ...extra })'
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
Log có cấu trúc = object, tìm được theo trường. Bài tiếp theo: MỌI
log CÙNG mức độ QUAN TRỌNG như nhau không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`taoNhatKy` ghi MỌI dòng log — `info`, `warn`, VÀ `error` — VÀO
CÙNG một mảng. Production THƯỜNG CHỈ muốn XEM `warn`/`error` (ẨN
`debug`/`info` vì QUÁ NHIỀU). Làm SAO lọc RA?
::::

::::checkpoint{mastery=0.8}
::::
