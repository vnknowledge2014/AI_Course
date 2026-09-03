---
id: co-so-du-lieu.thac-du-lieu.doc-moi-nhat-truoc
title: Đọc — mới nhất trước
summary: "timTrongSSTables quét danh sách SSTable TỪ MỚI nhất tới CŨ nhất (chỉ số cao xuống thấp) — khớp khoá Ở SSTable nào trước, trả về NGAY, không quét tiếp. Vì SSTable mới hơn LUÔN được tạo SAU, giá trị của nó (nếu có) LÀ giá trị đúng đắn nhất."
locale: vi
track: co-so-du-lieu
module: thac-du-lieu
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.read-newest-sstable-first]
requires: [db.flush-to-sstable]
concepts: [db.read-newest-sstable-first]
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
Mỗi lần memtable đầy sinh RA một SSTable mới (bài TRƯỚC) — CÙNG một
khoá có THỂ xuất hiện Ở nhiều SSTable, với giá trị KHÁC nhau. Đọc
lúc đó, tin bản NÀO?
::::

::::explain{#doc-moi-nhat-truoc}
`timTrongSSTables` quét danh sách SSTable TỪ MỚI nhất (chỉ số CAO
nhất) tới CŨ nhất — khớp Ở đâu TRƯỚC, trả về NGAY:

```typescript title=readonly
interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

function timTrongMotSSTable(bang: SSTable, khoa: string): string | null | undefined {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const k = bang.khoa[mid];
    if (k === khoa) return bang.giaTri[mid];
    if (k !== undefined && k < khoa) lo = mid + 1; else hi = mid;
  }
  return undefined;
}

function timTrongSSTables(danhSach: SSTable[], khoa: string): string | null | undefined {
  for (let i = danhSach.length - 1; i >= 0; i--) {
    const bang = danhSach[i];
    if (bang === undefined) continue;
    const kq = timTrongMotSSTable(bang, khoa);
    if (kq !== undefined) return kq;
  }
  return undefined;
}

const ss1: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const ss2: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
console.log(timTrongSSTables([ss1, ss2], "cam"));
```

```text title=readonly
5500
```

`ss1` (flush TRƯỚC, chỉ số `0`) có `cam=5000`. `ss2` (flush SAU,
chỉ số `1`, MỚI hơn) có `cam=5500`. Vòng `for` bắt đầu TỪ chỉ số
`danhSach.length - 1 = 1` (`ss2`) TRƯỚC — khớp `cam` NGAY, trả về
`5500`, KHÔNG bao giờ đi tới `ss1`.
::::

::::example{#chi-co-o-ban-cu}
Một khoá CHỈ tồn tại Ở SSTable CŨ — vẫn tìm RA, sau khi quét QUA
SSTable mới không khớp:

```typescript title=readonly
console.log(timTrongSSTables([ss1, ss2], "tao"));
```

```text title=readonly
12000
```

`"tao"` KHÔNG có trong `ss2` (mới) — `timTrongMotSSTable(ss2,
"tao")` trả VỀ `undefined`, vòng `for` tiếp TỤC sang `ss1` (cũ hơn),
khớp NGAY, trả về `12000`.
::::

::::predict{#doan-khoa-khong-o-dau-ca commitOnce}
Byte tìm một khoá KHÔNG hề tồn tại Ở BẤT KỲ SSTable nào:

```typescript
console.log(timTrongSSTables([ss1, ss2], "xoai"));
```

Dòng cuối in ra gì?

:::opt{correct}
`undefined`
:::

:::opt
`0` — vì khi không tìm thấy Ở ĐÂU cả, hàm trả về giá trị "trống"
mặc định cho kiểu `string`, giống `""` hay `0`
::why
Gần đúng ở việc bạn nghĩ TỚI một giá trị "MẶC định" hợp lý cho
trường hợp không tìm thấy — MỘT thói quen từ những ngôn NGỮ có giá
trị mặc định NGẦM cho từng kiểu.

Chỗ lệch: `timTrongSSTables` khai kiểu trả VỀ `string | null |
undefined` — khi vòng `for` chạy HẾT mà không khớp gì, dòng cuối
CÙNG `return undefined;` chạy, KHÔNG có nhánh nào trả về `0` HAY
`""`. TypeScript cũng sẽ TỪ chối nếu bạn thử gán `0` VÀO một chỗ
khai kiểu `string | null | undefined`.
::
:::

:::opt
Máy báo lỗi — vì `danhSach[i]` CÓ thể là `undefined` (do
`noUncheckedIndexedAccess`), gọi `timTrongMotSSTable` TRÊN đó sẽ
làm chương trình CRASH
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng đọc chỉ số mảng LUÔN có kiểu
`T | undefined` — một quan sát chính XÁC về TypeScript nghiêm ngặt.

Chỗ lệch: `timTrongSSTables` đã XỬ lý trường hợp đó tường minh —
dòng `if (bang === undefined) continue;` BỎ qua NGAY nếu phần tử
trống, không bao giờ gọi hàm TRÊN một `undefined`. Với `[ss1, ss2]`
(không có lỗ hổng NÀO trong mảng), nhánh đó KHÔNG bao giờ chạy tới,
nhưng nó vẫn tồn TẠI để TypeScript chấp nhận đoạn code.
::
:::
::::

::::code{#viet_tim_trong_sstables}
Hoàn thiện `timTrongSSTables` — quét TỪ SSTable mới nhất, trả về
NGAY khi khớp.

```typescript title=starter
interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

function timTrongMotSSTable(bang: SSTable, khoa: string): string | null | undefined {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const k = bang.khoa[mid];
    if (k === khoa) return bang.giaTri[mid];
    if (k !== undefined && k < khoa) lo = mid + 1; else hi = mid;
  }
  return undefined;
}

function timTrongSSTables(danhSach: SSTable[], khoa: string): string | null | undefined {
  for (let i = danhSach.length - 1; i >= 0; i--) {
    const bang = danhSach[i];
    if (bang === undefined) continue;
    ___
  }
  return undefined;
}

const ss1: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const ss2: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
console.log(timTrongSSTables([ss1, ss2], "cam"), timTrongSSTables([ss1, ss2], "tao"));
```

```typescript title=solution
interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

function timTrongMotSSTable(bang: SSTable, khoa: string): string | null | undefined {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const k = bang.khoa[mid];
    if (k === khoa) return bang.giaTri[mid];
    if (k !== undefined && k < khoa) lo = mid + 1; else hi = mid;
  }
  return undefined;
}

function timTrongSSTables(danhSach: SSTable[], khoa: string): string | null | undefined {
  for (let i = danhSach.length - 1; i >= 0; i--) {
    const bang = danhSach[i];
    if (bang === undefined) continue;
    const kq = timTrongMotSSTable(bang, khoa); if (kq !== undefined) return kq;
  }
  return undefined;
}

const ss1: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const ss2: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
console.log(timTrongSSTables([ss1, ss2], "cam"), timTrongSSTables([ss1, ss2], "tao"));
```

```typescript title=test
const ssA: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const ssB: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
if (timTrongSSTables([ssA, ssB], "cam") !== "5500") throw new Error("SSTable moi hon phai thang khi khoa trung");
if (timTrongSSTables([ssA, ssB], "tao") !== "12000") throw new Error("khoa chi o ban cu van phai tim ra");
if (timTrongSSTables([ssA, ssB], "dua") !== "3000") throw new Error("khoa chi o ban moi phai tim ra");
if (timTrongSSTables([ssA, ssB], "xoai") !== undefined) throw new Error("khoa khong ton tai o dau phai ra undefined");
if (timTrongSSTables([], "gi-cung-duoc") !== undefined) throw new Error("danh sach rong phai ra undefined");
```

:::hints
- kind: attention
  body: "Goi timTrongMotSSTable(bang, khoa), luu ket qua, neu ket qua khac undefined thi return NGAY -- mot dong."
- kind: strategy
  body: "const kq = timTrongMotSSTable(bang, khoa); if (kq !== undefined) return kq;"
- kind: one-line
  body: "const kq = timTrongMotSSTable(bang, khoa); if (kq !== undefined) return kq;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "5500 12000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đọc mới nhất TRƯỚC — đúng bản ghi, dù có nhiều SSTable trùng khoá.
Nhưng quét TỪNG SSTable bằng nhị phân vẫn phải chạm VÀO toàn bộ nội
dung nó — có cách nào NHẢY nhanh hơn không?
::::

::::reflect{#nghi-lai}
`timTrongSSTables` quét TỪ SSTable mới nhất — khớp Ở đâu trước, trả
về NGAY, vì SSTable mới hơn LUÔN phản ánh trạng thái ĐÚNG đắn hơn.
Nhưng mỗi SSTable ĐƯỢC quét bằng tìm nhị phân TRÊN toàn bộ nội dung
của nó — nếu SSTable RẤT lớn, load hết vào RAM chỉ để tìm MỘT khoá
LÀ lãng phí. Có cách nào nhảy tới đúng VÙNG mà không cần load HẾT
không?
::::

::::checkpoint{mastery=0.8}
::::
