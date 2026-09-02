---
id: ky-nghe-phan-mem.van-hanh.log-level-loc-theo-muc-do
title: "Log level — info/warn/error/debug, LỌC theo mức độ nghiêm trọng"
summary: "MỖI dòng log CÓ một level (\"info\"|\"warn\"|\"error\"|\"debug\", Discriminated Union hẹp) — cho PHÉP LỌC (production thường CHỈ hiện warn+error, ẩn debug vì quá NHIỀU). locTheoLevel(cacLog, cacLevelMuon): NhatKy[] — filter giữ lại đúng NHỮNG dòng có level nằm TRONG danh sách mong muốn, giữ NGUYÊN thứ tự gốc."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [vh.log-levels-filtering]
requires: [vh.structured-logging-intro]
concepts: [vh.log-levels-filtering]
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
`taoNhatKy` (bài 9) ghi MỌI dòng log VÀO chung MỘT mảng. Production
CHỈ muốn XEM `warn`/`error` — lọc RA cách nào?
::::

::::explain{#log-level-va-loc}
MỖI dòng log CÓ một `level` (`"info" | "warn" | "error" | "debug"`,
Discriminated Union HẸP đã học T4.3) — cho PHÉP **LỌC** (production
THƯỜNG CHỈ hiện `warn`+`error`, ẨN `debug` VÌ quá NHIỀU):

```typescript title=readonly
type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; [truong: string]: unknown };

function locTheoLevel(cacLog: NhatKy[], cacLevelMuon: LogLevel[]): NhatKy[] {
  return cacLog.filter((log) => cacLevelMuon.includes(log.level));
}

const cacLog: NhatKy[] = [
  { level: "info", msg: "a" },
  { level: "warn", msg: "b" },
  { level: "error", msg: "c" },
  { level: "debug", msg: "d" },
  { level: "info", msg: "e" },
];

const chiLoiVaCanhBao = locTheoLevel(cacLog, ["warn", "error"]);
console.log(chiLoiVaCanhBao.length);
console.log(chiLoiVaCanhBao.map((l) => l.msg));
```

```text title=readonly
2
["b","c"]
```

`locTheoLevel` GIỮ LẠI đúng NHỮNG dòng log có `level` NẰM TRONG danh
sách `cacLevelMuon` (`.includes`) — NĂM dòng CÓ SẴN, CHỈ HAI dòng
(`"warn"`, `"error"`) khớp.
::::

::::example{#hai-truong-hop-bien}
DANH SÁCH `cacLevelMuon` RỖNG → KHÔNG dòng NÀO khớp; DANH SÁCH ĐỦ
**BỐN** level → MỌI dòng ĐỀU khớp:

```typescript title=readonly
type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; [truong: string]: unknown };
function locTheoLevel(cacLog: NhatKy[], cacLevelMuon: LogLevel[]): NhatKy[] {
  return cacLog.filter((log) => cacLevelMuon.includes(log.level));
}
const cacLog: NhatKy[] = [
  { level: "info", msg: "a" },
  { level: "warn", msg: "b" },
  { level: "error", msg: "c" },
  { level: "debug", msg: "d" },
  { level: "info", msg: "e" },
];
console.log(locTheoLevel(cacLog, []).length);
console.log(locTheoLevel(cacLog, ["info", "warn", "error", "debug"]).length);
```

```text title=readonly
0
5
```

`[]` (KHÔNG level NÀO được YÊU CẦU) → `.includes` LUÔN trả `false`
CHO MỌI dòng → mảng RỖNG. Đủ BỐN level → MỌI dòng ĐỀU khớp → GIỮ
NGUYÊN VẸN cả năm dòng.
::::

::::predict{#doan-thu-tu-giu-nguyen commitOnce}
```typescript
type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; [truong: string]: unknown };
function locTheoLevel(cacLog: NhatKy[], cacLevelMuon: LogLevel[]): NhatKy[] {
  return cacLog.filter((log) => cacLevelMuon.includes(log.level));
}
const cacLog: NhatKy[] = [
  { level: "error", msg: "loi-C" },
  { level: "info", msg: "thong-tin-1" },
  { level: "error", msg: "loi-A" },
  { level: "warn", msg: "canh-bao-1" },
  { level: "error", msg: "loi-B" },
];
const chiLoi = locTheoLevel(cacLog, ["error"]);
console.log(chiLoi.map((l) => l.msg));
```

Dòng cuối in ra gì?

:::opt{correct}
`["loi-C","loi-A","loi-B"]`
:::

:::opt
`["loi-A","loi-B","loi-C"]` — vì `filter` SẮP XẾP LẠI kết quả THEO
THỨ TỰ BẢNG CHỮ CÁI (alphabet) của trường `msg`, giống CÁCH một
CÔNG CỤ TÌM KIẾM thường SẮP XẾP kết quả TÌM ĐƯỢC
::why
Gần đúng ở việc bạn nghĩ TỚI "sắp xếp kết quả TÌM KIẾM" — MỘT liên
tưởng hợp lý KHI nghĩ về "lọc VÀ TÌM" nói CHUNG (nhiều công cụ THẬT
SỰ có sắp xếp).

Chỗ lệch: `Array.prototype.filter` (VÀ `locTheoLevel` dùng nó) HOÀN
TOÀN KHÔNG sắp xếp GÌ cả — nó CHỈ GIỮ LẠI những phần tử THOẢ điều
kiện, **THEO ĐÚNG THỨ TỰ chúng XUẤT HIỆN Ở mảng GỐC** (KHÔNG chạm
tới `msg` để so sánh CHỮ CÁI). Ba dòng `"error"` xuất hiện Ở mảng
GỐC theo thứ tự `loi-C → loi-A → loi-B` — `filter` GIỮ NGUYÊN đúng
thứ tự ĐÓ.
::
:::

:::opt
Máy báo lỗi biên dịch — `cacLog` chứa BA dòng `level: "error"` (LẶP
LẠI CÙNG một giá trị `level`), TypeScript CẤM một MẢNG chứa NHIỀU
phần tử có CÙNG giá trị Ở MỘT trường CỤ THỂ
::why
Gần đúng ở việc bạn để ý CÓ BA dòng CÙNG `level: "error"` trong
`cacLog` — một quan sát ĐÚNG về NỘI DUNG mảng.

Chỗ lệch: TypeScript HOÀN TOÀN KHÔNG có khái niệm "CẤM giá trị TRÙNG
LẶP Ở một TRƯỜNG" cho MẢNG object thường — MỘT mảng `NhatKy[]` CHO
PHÉP BAO NHIÊU phần tử TUỲ Ý, CÓ trường GIỐNG nhau HAY khác nhau
ĐỀU được. Biên dịch SẠCH.
::
:::
::::

::::code{#viet_loc_theo_level}
Hoàn thiện `locTheoLevel` — giữ LẠI đúng những dòng log có `level`
NẰM TRONG danh sách MONG MUỐN.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; [truong: string]: unknown };

function locTheoLevel(cacLog: NhatKy[], cacLevelMuon: LogLevel[]): NhatKy[] {
  return ___;
}

const cacLog1: NhatKy[] = [
  { level: "info", msg: "a" },
  { level: "error", msg: "b" },
];
assertEqual(locTheoLevel(cacLog1, ["error"]).length, 1, "chi lay error");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; [truong: string]: unknown };

function locTheoLevel(cacLog: NhatKy[], cacLevelMuon: LogLevel[]): NhatKy[] {
  return cacLog.filter((log) => cacLevelMuon.includes(log.level));
}

const cacLog1: NhatKy[] = [
  { level: "info", msg: "a" },
  { level: "error", msg: "b" },
];
assertEqual(locTheoLevel(cacLog1, ["error"]).length, 1, "chi lay error");
```

```typescript title=test
assertEqual(locTheoLevel(cacLog1, ["error"])[0]!.msg, "b", "dung dong log");
assertEqual(locTheoLevel(cacLog1, ["info", "error"]).length, 2, "lay ca hai level");
assertEqual(locTheoLevel(cacLog1, []).length, 0, "khong level nao -- rong");
assertEqual(locTheoLevel(cacLog1, ["debug"]).length, 0, "level khong ton tai trong log -- rong");
```

:::hints
- kind: attention
  body: "Dùng cacLog.filter, giữ lại log MÀ level của nó NẰM TRONG cacLevelMuon (dùng .includes)."
- kind: strategy
  body: "cacLog.filter((log) => cacLevelMuon.includes(log.level))"
- kind: one-line
  body: '___ = cacLog.filter((log) => cacLevelMuon.includes(log.level))'
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
Lọc theo level = filter + includes, giữ nguyên thứ tự. Bài tiếp
theo: theo dõi MỘT request qua NHIỀU dòng log riêng lẻ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`locTheoLevel` lọc theo MỨC ĐỘ — NHƯNG nếu MUỐN tìm TẤT CẢ dòng log
LIÊN QUAN tới MỘT request CỤ THỂ (bất kể level), CẦN MỘT trường
NHẬN DIỆN nào?
::::

::::checkpoint{mastery=0.8}
::::
