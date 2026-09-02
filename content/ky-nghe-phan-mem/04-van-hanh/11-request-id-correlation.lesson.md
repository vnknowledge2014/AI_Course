---
id: ky-nghe-phan-mem.van-hanh.request-id-correlation
title: "Request ID correlation — theo dõi MỘT request qua NHIỀU dòng log"
summary: "MỖI request VÀO hệ thống ĐƯỢC gán MỘT requestId (được TRUYỀN VÀO — không dùng crypto.randomUUID(), quy ước injected-id của dự án) — MỌI dòng log LIÊN QUAN request đó MANG THEO CÙNG requestId. locTheoRequestId(cacLog, id): NhatKy[] — TÌM lại TOÀN BỘ \"câu chuyện\" của MỘT request cụ thể giữa hàng ngàn dòng log KHÁC, giống cách grep requestId hoạt động trong thực tế."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [vh.request-id-correlation]
requires: [vh.log-levels-filtering]
concepts: [vh.request-id-correlation]
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
`locTheoLevel` (bài 10) lọc theo MỨC ĐỘ. Production xử lý HÀNG NGHÌN
request CÙNG lúc, log XEN LẪN vào nhau — làm SAO tìm ĐÚNG log của
MỘT request?
::::

::::explain{#request-id-correlation}
MỖI request VÀO hệ thống ĐƯỢC gán MỘT `requestId` (ĐƯỢC TRUYỀN VÀO —
KHÔNG dùng `crypto.randomUUID()`, quy ước injected-id CỦA dự án) —
MỌI dòng log LIÊN QUAN request ĐÓ MANG THEO **CÙNG** `requestId`:

```typescript title=readonly
type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; requestId?: string; [truong: string]: unknown };

function locTheoRequestId(cacLog: NhatKy[], id: string): NhatKy[] {
  return cacLog.filter((log) => log.requestId === id);
}

const cacLog: NhatKy[] = [
  { level: "info", msg: "bat dau xu ly", requestId: "req-A" },
  { level: "info", msg: "bat dau xu ly", requestId: "req-B" },
  { level: "info", msg: "truy van du lieu", requestId: "req-A" },
  { level: "error", msg: "loi ket noi", requestId: "req-B" },
  { level: "info", msg: "hoan tat", requestId: "req-A" },
];

const cauChuyenA = locTheoRequestId(cacLog, "req-A");
console.log(cauChuyenA.map((l) => l.msg));
```

```text title=readonly
["bat dau xu ly","truy van du lieu","hoan tat"]
```

Log CỦA `"req-A"` VÀ `"req-B"` XEN LẪN VÀO nhau — `locTheoRequestId`
TÌM lại ĐÚNG **BA** dòng thuộc VỀ `"req-A"`, THEO ĐÚNG THỨ TỰ chúng
xảy RA, BỎ QUA HOÀN TOÀN log của `"req-B"` — TÁI HIỆN "câu chuyện"
CỦA MỘT request, GIỐNG cách `grep requestId` hoạt động trong THỰC
TẾ (đã nhắc Ở nguồn chương 43).
::::

::::example{#request-id-khong-ton-tai}
TRUY VẤN MỘT `requestId` KHÔNG hề tồn tại TRẢ VỀ mảng RỖNG (KHÔNG
lỗi gì) — GIỐNG `filter` mọi trường hợp KHÁC:

```typescript title=readonly
type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; requestId?: string; [truong: string]: unknown };
function locTheoRequestId(cacLog: NhatKy[], id: string): NhatKy[] {
  return cacLog.filter((log) => log.requestId === id);
}
const cacLog: NhatKy[] = [
  { level: "info", msg: "khoi dong he thong" },
  { level: "info", msg: "xu ly yeu cau", requestId: "req-X" },
];
console.log(locTheoRequestId(cacLog, "req-khong-ton-tai").length);
```

```text title=readonly
0
```

Dòng ĐẦU (`"khoi dong he thong"`) KHÔNG CÓ `requestId` NÀO CẢ (sự
kiện HỆ THỐNG, KHÔNG gắn VỚI request cụ thể) — HOÀN TOÀN bình
thường, `NhatKy`'s `requestId?` khai kiểu **TUỲ CHỌN**.
::::

::::predict{#doan-chuoi-undefined-khac-gia-tri-undefined commitOnce}
```typescript
type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; requestId?: string; [truong: string]: unknown };
function locTheoRequestId(cacLog: NhatKy[], id: string): NhatKy[] {
  return cacLog.filter((log) => log.requestId === id);
}
const cacLog: NhatKy[] = [
  { level: "info", msg: "khoi dong he thong" },
  { level: "info", msg: "xu ly yeu cau", requestId: "req-X" },
];
console.log(locTheoRequestId(cacLog, "undefined").length);
```

Dòng ĐẦU của `cacLog` KHÔNG CÓ trường `requestId` (giá trị THẬT SỰ
LÀ `undefined`). Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
`1` — vì dòng ĐẦU CÓ `requestId` LÀ `undefined`, VÀ CHUỖI `"undefined"`
CHÍNH LÀ CÁCH JavaScript "viết ra" giá trị `undefined` — SO SÁNH
`log.requestId === "undefined"` sẽ ĐÚNG cho dòng ĐÓ
::why
Gần đúng ở việc bạn nhớ ĐÚNG dòng ĐẦU thật sự KHÔNG CÓ `requestId`
(giá trị `undefined`) — quan sát ĐÓ về NỘI DUNG dữ liệu chính xác.

Chỗ lệch: `undefined` (GIÁ TRỊ ĐẶC BIỆT, kiểu `undefined`) VÀ
`"undefined"` (MỘT CHUỖI GỒM chín ký TỰ) LÀ **HAI THỨ HOÀN TOÀN
KHÁC NHAU** trong JavaScript/TypeScript — `===` so sánh **CẢ kiểu
LẪN giá trị**, `undefined === "undefined"` LUÔN LÀ `false`. Việc
"in RA màn hình" (như trong THÔNG BÁO lỗi) CÓ THỂ HIỂN THỊ chữ
`undefined`, NHƯNG ĐÓ CHỈ LÀ CÁCH TRÌNH BÀY, KHÔNG PHẢI GIÁ TRỊ chuỗi
thật. KHÔNG dòng NÀO khớp — kết quả LÀ `0`.
::
:::

:::opt
Máy báo lỗi biên dịch — `NhatKy` khai `requestId?: string` (TUỲ
CHỌN), NHƯNG dòng ĐẦU của `cacLog` KHÔNG cung cấp trường ĐÓ, trong
khi `locTheoRequestId` yêu CẦU tham số `id: string` (BẮT BUỘC),
TypeScript CẤM SO SÁNH một trường TUỲ CHỌN VỚI một tham số BẮT BUỘC
::why
Gần đúng ở việc bạn để ý `requestId?` (TUỲ CHỌN) VÀ `id: string`
(BẮT BUỘC) khai kiểu KHÁC NHAU VỀ TÍNH BẮT BUỘC — một quan sát ĐÚNG
về KHAI BÁO kiểu.

Chỗ lệch: `log.requestId` có kiểu `string | undefined` (VÌ khai
TUỲ CHỌN) — SO SÁNH `string | undefined === string` HOÀN TOÀN hợp
lệ trong TypeScript (`===` chấp NHẬN so sánh GIỮA các kiểu CÓ GIAO
NHAU). Biên dịch SẠCH — kết quả LÚC CHẠY đơn giản LÀ `false` cho
dòng THIẾU `requestId`.
::
:::
::::

::::code{#viet_loc_theo_request_id}
Hoàn thiện `locTheoRequestId` — giữ LẠI đúng những dòng log CÓ
`requestId` KHỚP.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; requestId?: string; [truong: string]: unknown };

function locTheoRequestId(cacLog: NhatKy[], id: string): NhatKy[] {
  return ___;
}

const cacLog1: NhatKy[] = [
  { level: "info", msg: "a", requestId: "r1" },
  { level: "info", msg: "b", requestId: "r2" },
];
assertEqual(locTheoRequestId(cacLog1, "r1").length, 1, "chi lay dung request");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LogLevel = "info" | "warn" | "error" | "debug";
type NhatKy = { level: LogLevel; msg: string; requestId?: string; [truong: string]: unknown };

function locTheoRequestId(cacLog: NhatKy[], id: string): NhatKy[] {
  return cacLog.filter((log) => log.requestId === id);
}

const cacLog1: NhatKy[] = [
  { level: "info", msg: "a", requestId: "r1" },
  { level: "info", msg: "b", requestId: "r2" },
];
assertEqual(locTheoRequestId(cacLog1, "r1").length, 1, "chi lay dung request");
```

```typescript title=test
assertEqual(locTheoRequestId(cacLog1, "r1")[0]!.msg, "a", "dung dong log");
assertEqual(locTheoRequestId(cacLog1, "r-khong-ton-tai").length, 0, "khong tim thay request");

const cacLog2: NhatKy[] = [
  { level: "info", msg: "bat dau", requestId: "req-A" },
  { level: "info", msg: "bat dau", requestId: "req-B" },
  { level: "info", msg: "tiep tuc", requestId: "req-A" },
  { level: "error", msg: "loi", requestId: "req-B" },
  { level: "info", msg: "xong", requestId: "req-A" },
];
assertEqual(locTheoRequestId(cacLog2, "req-A").length, 3, "gom du ba dong cua req-A");
assertEqual(locTheoRequestId(cacLog2, "req-B").length, 2, "gom du hai dong cua req-B");
```

:::hints
- kind: attention
  body: "Dùng cacLog.filter, giữ lại log MÀ requestId của nó ĐÚNG BẰNG id được truyền vào."
- kind: strategy
  body: "cacLog.filter((log) => log.requestId === id)"
- kind: one-line
  body: '___ = cacLog.filter((log) => log.requestId === id)'
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
Request ID = câu chuyện của MỘT request giữa hàng nghìn log XEN LẪN.
Bài chốt cụm: ghép cấu trúc + level + correlation thành MỘT logger
đầy đủ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`taoNhatKy` (bài 9), `locTheoLevel` (bài 10), VÀ `locTheoRequestId`
(bài NÀY) hiện LÀ BA mảnh RIÊNG. Ghép CẢ BA thành MỘT logger DUY
NHẤT trông như thế NÀO?
::::

::::checkpoint{mastery=0.8}
::::
