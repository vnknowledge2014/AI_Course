---
id: co-so-du-lieu.thac-du-lieu.tim-trong-memtable
title: Tìm trong memtable
summary: "timMemtable dùng ĐÚNG vòng nhị phân dò vị trí của chenMemtable (bài trước) — nhưng dừng SỚM và trả về giá trị NGAY khi khớp khoá, thay vì chèn. Vì memtable LUÔN đã sắp xếp, tìm kiếm không cần quét tuyến tính."
locale: vi
track: co-so-du-lieu
module: thac-du-lieu
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.memtable-binary-search]
requires: [db.skip-list-idea]
concepts: [db.memtable-binary-search]
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
Memtable LUÔN giữ thứ tự (bài `memtable-luon-giu-thu-tu`) — vậy tìm
MỘT khoá trong đó có cần quét TỪNG phần tử một không?
::::

::::explain{#tim-memtable}
`timMemtable` dùng LẠI đúng vòng nhị phân của `chenMemtable`, NHƯNG
dừng SỚM VÀ trả về giá trị NGAY khi khớp khoá — không cần chèn gì
cả:

```typescript title=readonly
interface Memtable {
  khoa: string[];
  giaTri: (string | null)[];
}

function taoMemtable(): Memtable {
  return { khoa: [], giaTri: [] };
}

function chenMemtable(bang: Memtable, khoa: string, giaTri: string | null): void {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const khoaGiua = bang.khoa[mid];
    if (khoaGiua !== undefined && khoaGiua < khoa) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  if (bang.khoa[lo] === khoa) { bang.giaTri[lo] = giaTri; } else { bang.khoa.splice(lo, 0, khoa); bang.giaTri.splice(lo, 0, giaTri); }
}

function timMemtable(bang: Memtable, khoa: string): string | null | undefined {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const khoaGiua = bang.khoa[mid];
    if (khoaGiua === khoa) return bang.giaTri[mid];
    if (khoaGiua !== undefined && khoaGiua < khoa) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  return undefined;
}

const m = taoMemtable();
chenMemtable(m, "cam", "5000");
chenMemtable(m, "buoi", "8000");
chenMemtable(m, "tao", "12000");
console.log(timMemtable(m, "buoi"));
```

```text title=readonly
8000
```

`timMemtable(m, "buoi")` dò NHỊ phân TRÊN mảng `[buoi, cam, tao]` đã
sắp xếp — khớp `buoi` ngay khi vòng LẶP chạm tới nó, trả về
`giaTri[mid]` LUÔN, không đi tiếp thêm bước NÀO.
::::

::::example{#khong-tim-thay}
Tìm một khoá KHÔNG có trong memtable — vòng lặp CHẠY hết mà KHÔNG
khớp, trả về `undefined`:

```typescript title=readonly
console.log(String(timMemtable(m, "xoai")));
```

```text title=readonly
undefined
```

`"xoai"` không nằm TRONG `[buoi, cam, tao]` — vòng `while` thu hẹp
`[lo, hi)` dần tới KHI `lo === hi`, KHÔNG khoá nào khớp trong quá
trình đó, RỒI rơi ra ngoài vòng lặp, trả VỀ `undefined`. Bọc bằng
`String(...)` trước khi `console.log` — nếu KHÔNG, `undefined` in
ra thành DÒNG trống, không phải chữ "undefined".
::::

::::predict{#doan-tim-khoa-dau commitOnce}
Memtable đang có ba khoá (`buoi, cam, tao`). Byte tìm khoá ĐẦU tiên
theo bảng chữ cái:

```typescript
console.log(timMemtable(m, "buoi"), timMemtable(m, "cam"));
```

Dòng cuối in ra gì?

:::opt{correct}
`8000 5000`
:::

:::opt
`undefined undefined` — vì `timMemtable` chỉ hoạt động ĐÚNG khi
khoá cần tìm nằm Ở GIỮA mảng, còn khoá Ở hai ĐẦU thì vòng nhị phân
"trượt" qua mất
::why
Gần đúng ở việc bạn nghĩ TỚI khả năng tìm nhị phân "bỏ SÓT" các vị
trí biên — MỘT lo lắng hợp lý nếu cận `lo`/`hi` viết SAI.

Chỗ lệch: `bang.khoa[lo] === khoa` (bài `chenMemtable`) và
`khoaGiua === khoa` (Ở đây) đều là phép SO sánh ĐÚNG cho MỌI vị trí,
kể cả đầu VÀ cuối mảng — tìm nhị phân đã CHẠY thật, khớp `buoi`
(đầu) VÀ `cam` (giữa) đều đúng, không hề "trượt" qua vị trí nào.
::
:::

:::opt
`8000 8000` — vì `timMemtable` LUÔN trả về giá trị của phần tử ĐẦU
tiên tìm thấy trong vòng lặp, bất kể khoá cần TÌM là gì
::why
Gần đúng ở việc bạn nghĩ TỚI một hành vi "dừng SỚM" của vòng lặp —
ĐÚNG là `timMemtable` dừng NGAY khi khớp, một quan sát chính xác.

Chỗ lệch: nó dừng khi khớp ĐÚNG khoá đang TÌM (`khoaGiua === khoa`),
không phải LUÔN dừng Ở lần lặp đầu tiên — `timMemtable(m, "cam")`
dò TỚI đúng vị trí của `"cam"` (không phải `"buoi"`), trả về giá
trị RIÊNG của `"cam"` (`5000`), khác VỚI giá trị của `"buoi"`.
::
:::
::::

::::code{#viet_tim_memtable}
Hoàn thiện `timMemtable` — khi khớp đúng khoá, trả VỀ giá trị NGAY.

```typescript title=starter
interface Memtable {
  khoa: string[];
  giaTri: (string | null)[];
}

function taoMemtable(): Memtable {
  return { khoa: [], giaTri: [] };
}

function chenMemtable(bang: Memtable, khoa: string, giaTri: string | null): void {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const khoaGiua = bang.khoa[mid];
    if (khoaGiua !== undefined && khoaGiua < khoa) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  if (bang.khoa[lo] === khoa) { bang.giaTri[lo] = giaTri; } else { bang.khoa.splice(lo, 0, khoa); bang.giaTri.splice(lo, 0, giaTri); }
}

function timMemtable(bang: Memtable, khoa: string): string | null | undefined {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const khoaGiua = bang.khoa[mid];
    ___
    if (khoaGiua !== undefined && khoaGiua < khoa) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  return undefined;
}

const m = taoMemtable();
chenMemtable(m, "cam", "5000");
chenMemtable(m, "buoi", "8000");
chenMemtable(m, "tao", "12000");
console.log(timMemtable(m, "buoi"), String(timMemtable(m, "xoai")));
```

```typescript title=solution
interface Memtable {
  khoa: string[];
  giaTri: (string | null)[];
}

function taoMemtable(): Memtable {
  return { khoa: [], giaTri: [] };
}

function chenMemtable(bang: Memtable, khoa: string, giaTri: string | null): void {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const khoaGiua = bang.khoa[mid];
    if (khoaGiua !== undefined && khoaGiua < khoa) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  if (bang.khoa[lo] === khoa) { bang.giaTri[lo] = giaTri; } else { bang.khoa.splice(lo, 0, khoa); bang.giaTri.splice(lo, 0, giaTri); }
}

function timMemtable(bang: Memtable, khoa: string): string | null | undefined {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const khoaGiua = bang.khoa[mid];
    if (khoaGiua === khoa) return bang.giaTri[mid];
    if (khoaGiua !== undefined && khoaGiua < khoa) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  return undefined;
}

const m = taoMemtable();
chenMemtable(m, "cam", "5000");
chenMemtable(m, "buoi", "8000");
chenMemtable(m, "tao", "12000");
console.log(timMemtable(m, "buoi"), String(timMemtable(m, "xoai")));
```

```typescript title=test
const m2 = taoMemtable();
chenMemtable(m2, "cam", "5000");
chenMemtable(m2, "buoi", "8000");
chenMemtable(m2, "tao", "12000");
if (timMemtable(m2, "buoi") !== "8000") throw new Error("tim khoa dau, phai ra 8000");
if (timMemtable(m2, "cam") !== "5000") throw new Error("tim khoa giua, phai ra 5000");
if (timMemtable(m2, "tao") !== "12000") throw new Error("tim khoa cuoi, phai ra 12000");
if (timMemtable(m2, "xoai") !== undefined) throw new Error("khoa khong ton tai phai ra undefined");
if (timMemtable(taoMemtable(), "gi-cung-duoc") !== undefined) throw new Error("memtable rong phai ra undefined");
```

:::hints
- kind: attention
  body: "Neu khoaGiua === khoa, tra ve bang.giaTri[mid] NGAY -- mot dong, dat truoc nhanh cap nhat lo/hi."
- kind: strategy
  body: "if (khoaGiua === khoa) return bang.giaTri[mid];"
- kind: one-line
  body: "if (khoaGiua === khoa) return bang.giaTri[mid];"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "8000 undefined"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tìm trong memtable NHANH — nhị phân, không quét tuyến tính. Nhưng
memtable chỉ SỐNG trong RAM — khi nó ĐẦY, dữ liệu phải lên đĩa
bằng cách NÀO?
::::

::::reflect{#nghi-lai}
`timMemtable` tái dùng ĐÚNG vòng nhị phân của `chenMemtable`, chỉ
thêm MỘT điều kiện dừng sớm — vì memtable LUÔN đã sắp xếp, tìm kiếm
không bao giờ cần quét TUYẾN tính. Nhưng memtable chỉ tồn tại TRONG
RAM — mất điện LÀ mất hết. Khi nó đầy (bài `vi-sao-ghi-tai-cho-ton`
đã đặt câu hỏi này), dữ liệu phải được GHI xuống đĩa — dưới dạng
gì?
::::

::::checkpoint{mastery=0.8}
::::
