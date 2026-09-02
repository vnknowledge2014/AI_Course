---
id: ky-nghe-phan-mem.van-hanh.counter-dem-su-kien
title: "Counter — đếm sự kiện (số request, số lỗi)"
summary: "Metric ĐƠN GIẢN NHẤT: Counter — MỘT con số CHỈ TĂNG (không bao giờ giảm), đếm \"bao nhiêu lần X xảy ra\" (request, lỗi, đăng ký mới). taoCounter(ten): {ten, tang, giaTri} — closure GIỮ số đếm nội bộ, TÁI DÙNG kỹ thuật taoBoDem T5.4 bài 3, MỖI counter ĐỘC LẬP."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 13
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [vh.metrics-counter]
requires: [vh.gate-boss-structured-logging]
concepts: [vh.metrics-counter]
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
Cụm mới. Log (cụm 3) ghi TỪNG sự kiện RIÊNG LẺ. "Hệ thống ĐANG có
BAO NHIÊU request?" — đếm TỪNG dòng log BẰNG TAY?
::::

::::explain{#counter-la-gi}
Metric ĐƠN GIẢN NHẤT: **Counter** — MỘT con số CHỈ TĂNG (KHÔNG BAO
GIỜ giảm), đếm "BAO NHIÊU LẦN X xảy ra" (request, lỗi, đăng ký MỚI):

```typescript title=readonly
function taoCounter(ten: string): { ten: string; tang: () => void; giaTri: () => number } {
  let dem = 0;
  return {
    ten,
    tang: () => { dem += 1; },
    giaTri: () => dem,
  };
}

const requestCounter = taoCounter("http_requests_total");
const errorCounter = taoCounter("http_errors_total");

for (let i = 0; i < 100; i++) requestCounter.tang();
for (let i = 0; i < 3; i++) errorCounter.tang();

console.log(requestCounter.giaTri());
console.log(errorCounter.giaTri());
```

```text title=readonly
100
3
```

`taoCounter` LÀ MỘT nhà máy (TÁI DÙNG kỹ thuật `taoBoDem` T5.4 bài
3) — closure GIỮ số đếm `dem` NỘI BỘ, lộ RA `tang` (tăng LÊN MỘT)
VÀ `giaTri` (đọc số HIỆN TẠI). `requestCounter`/`errorCounter` LÀ
HAI counter HOÀN TOÀN ĐỘC LẬP.
::::

::::example{#moi-counter-doc-lap}
GỌI `taoCounter` NHIỀU lần TẠO NHIỀU counter ĐỘC LẬP — TĂNG counter
NÀY KHÔNG ẢNH HƯỞNG counter KHÁC:

```typescript title=readonly
function taoCounter(ten: string): { ten: string; tang: () => void; giaTri: () => number } {
  let dem = 0;
  return { ten, tang: () => { dem += 1; }, giaTri: () => dem };
}
const c1 = taoCounter("a");
const c2 = taoCounter("b");
c1.tang();
c1.tang();
console.log(c1.ten, c1.giaTri());
console.log(c2.ten, c2.giaTri());
```

```text title=readonly
a 2
b 0
```

`c1.tang()` gọi HAI lần — `c1.giaTri()` LÀ `2`. `c2` KHÔNG hề bị
CHẠM tới — `c2.giaTri()` VẪN LÀ `0` (giá trị KHỞI TẠO).
::::

::::predict{#doan-tang-co-dieu-kien commitOnce}
```typescript
function taoCounter(ten: string): { ten: string; tang: () => void; giaTri: () => number } {
  let dem = 0;
  return { ten, tang: () => { dem += 1; }, giaTri: () => dem };
}
const c = taoCounter("test");
const gia = [true, false, true, true, false];
for (const daXayRa of gia) {
  if (daXayRa) c.tang();
}
console.log(c.giaTri());
```

`gia` CÓ NĂM phần tử, BA trong SỐ ĐÓ LÀ `true`. Dòng cuối in ra gì?

:::opt{correct}
`3`
:::

:::opt
`5` — vì vòng lặp `for` CHẠY QUA đủ NĂM phần tử của `gia`, VÀ MỖI
lần VÒNG LẶP chạy (BẤT KỂ `daXayRa` LÀ `true` hay `false`) ĐỀU tính
LÀ MỘT "lượt xử lý" nên counter TĂNG THEO SỐ LƯỢT, KHÔNG PHẢI theo
SỐ `true`
::why
Gần đúng ở việc bạn nhớ ĐÚNG vòng lặp `for` chạy ĐỦ NĂM lượt (MỘT
lượt CHO MỖI phần tử của `gia`) — quan sát ĐÓ về SỐ LƯỢT lặp chính
xác.

Chỗ lệch: `c.tang()` CHỈ được gọi **BÊN TRONG** khối `if (daXayRa)`
— nó CHỈ THỰC SỰ chạy khi `daXayRa` LÀ `true`. Với `false`, khối
`if` KHÔNG chạy vào, `c.tang()` KHÔNG được GỌI Ở lượt ĐÓ. `gia` có
ĐÚNG BA giá trị `true` (VỊ trí 1, 3, 4) — `c.tang()` CHỈ thực sự
CHẠY ba LẦN, `c.giaTri()` LÀ `3`.
::
:::

:::opt
Máy báo lỗi biên dịch — `taoCounter` khai kiểu trả về CÓ property
`tang: () => void`, NHƯNG lời gọi `c.tang()` NẰM BÊN TRONG một khối
`if` (điều kiện), TypeScript CẤM gọi MỘT phương thức trả VỀ `void`
BÊN TRONG khối điều kiện
::why
Gần đúng ở việc bạn để ý `c.tang()` NẰM BÊN TRONG khối `if
(daXayRa)` — một quan sát ĐÚNG về VỊ TRÍ lời gọi TRONG code.

Chỗ lệch: TypeScript HOÀN TOÀN KHÔNG có quy tắc "CẤM gọi hàm trả
`void` TRONG điều kiện" — GỌI MỘT hàm (BẤT KỂ kiểu trả về LÀ gì) BÊN
TRONG `if`, `for`, `while`, HAY BẤT KỲ khối NÀO ĐỀU hợp lệ. Biên
dịch SẠCH.
::
:::
::::

::::code{#viet_tao_counter}
Hoàn thiện `taoCounter` — `tang` tăng bộ đếm NỘI BỘ LÊN MỘT, `giaTri`
đọc giá trị HIỆN TẠI.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function taoCounter(ten: string): { ten: string; tang: () => void; giaTri: () => number } {
  let dem = 0;
  return {
    ten,
    tang: () => { ___; },
    giaTri: () => ___,
  };
}

const c = taoCounter("test");
assertEqual(c.giaTri(), 0, "counter bat dau tu 0");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function taoCounter(ten: string): { ten: string; tang: () => void; giaTri: () => number } {
  let dem = 0;
  return {
    ten,
    tang: () => { dem += 1; },
    giaTri: () => dem,
  };
}

const c = taoCounter("test");
assertEqual(c.giaTri(), 0, "counter bat dau tu 0");
```

```typescript title=test
c.tang();
assertEqual(c.giaTri(), 1, "tang mot lan");
c.tang();
c.tang();
assertEqual(c.giaTri(), 3, "tang ba lan tong cong");
assertEqual(c.ten, "test", "ten duoc luu dung");

const c2 = taoCounter("khac");
c2.tang();
assertEqual(c2.giaTri(), 1, "counter khac doc lap");
assertEqual(c.giaTri(), 3, "counter dau khong bi anh huong boi counter hai");
```

:::hints
- kind: attention
  body: "tang: tăng dem lên 1 (dùng +=). giaTri: trả VỀ giá trị dem hiện tại."
- kind: strategy
  body: "dem += 1 : dem"
- kind: one-line
  body: '___ (tang) = dem += 1\n___ (giaTri) = dem'
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
Counter = số chỉ tăng, đếm sự kiện. Bài tiếp theo: từ HAI counter,
tính ra một CON SỐ có Ý NGHĨA hơn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`requestCounter` VÀ `errorCounter` (ví dụ ĐẦU bài) LÀ HAI con số
RIÊNG LẺ. "Hệ thống ĐANG lỗi NHIỀU hay ÍT" — CẦN kết hợp HAI con số
ĐÓ như thế NÀO?
::::

::::checkpoint{mastery=0.8}
::::
