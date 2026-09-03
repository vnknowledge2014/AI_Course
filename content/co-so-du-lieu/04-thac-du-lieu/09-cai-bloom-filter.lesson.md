---
id: co-so-du-lieu.thac-du-lieu.cai-bloom-filter
title: Cài bloom filter — thêm và kiểm tra
summary: "themBloom bật LÊN mọi bit mà bamDon chỉ tới (một khoá, nhiều hàm băm qua seed). coTheCoBloom kiểm TRA đúng những bit đó — nếu bất kỳ bit nào CÒN tắt, khoá chắc chắn CHƯA từng được thêm."
locale: vi
track: co-so-du-lieu
module: thac-du-lieu
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.bloom-filter-implementation]
requires: [db.bloom-filter-idea]
concepts: [db.bloom-filter-implementation]
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
Bài trước giới thiệu Ý tưởng — CHẮC chắn không, có THỂ có. Cài đặt
THẬT của "thêm" và "kiểm tra" trông NHƯ thế nào?
::::

::::explain{#them-va-kiem-tra}
`themBloom` bật LÊN mọi bit mà `bamDon` chỉ tới (MỘT khoá, NHIỀU hàm
băm khác nhau qua `seed`). `coTheCoBloom` kiểm TRA — nếu BẤT KỲ bit
nào trong số đó CÒN tắt, khoá chắc chắn CHƯA từng được thêm:

```typescript title=readonly
function bamDon(s: string, seed: number, kichThuoc: number): number {
  let h = seed;
  for (let i = 0; i < s.length; i++) {
    h = (h * 31 + s.charCodeAt(i)) >>> 0;
  }
  return h % kichThuoc;
}

interface BoLocBloom {
  bit: boolean[];
  kichThuoc: number;
  soHam: number;
}

function taoBoLocBloom(kichThuoc: number, soHam: number): BoLocBloom {
  return { bit: new Array(kichThuoc).fill(false), kichThuoc, soHam };
}

function themBloom(bo: BoLocBloom, khoa: string): void {
  for (let seed = 0; seed < bo.soHam; seed++) {
    const viTri = bamDon(khoa, seed, bo.kichThuoc);
    bo.bit[viTri] = true;
  }
}

function coTheCoBloom(bo: BoLocBloom, khoa: string): boolean {
  for (let seed = 0; seed < bo.soHam; seed++) {
    const viTri = bamDon(khoa, seed, bo.kichThuoc);
    if (bo.bit[viTri] !== true) return false;
  }
  return true;
}

const bo = taoBoLocBloom(8, 2);
themBloom(bo, "cam");
themBloom(bo, "buoi");
console.log(coTheCoBloom(bo, "cam"), coTheCoBloom(bo, "buoi"));
```

```text title=readonly
true true
```

`themBloom(bo, "cam")` bật hai bit (`seed=0` và `seed=1` của `bamDon`
cho "cam"). `themBloom(bo, "buoi")` bật thêm hai bit KHÁC (có thể
trùng một phần). `coTheCoBloom` kiểm TRA đúng những bit ĐÓ — cả hai
khoá VỪA thêm đều trả về `true`.
::::

::::example{#am-tinh-that}
Một khoá CHƯA từng thêm, hai vị trí băm của nó ĐỀU chưa từng được bật:

```typescript title=readonly
console.log(coTheCoBloom(bo, "tao"));
```

```text title=readonly
false
```

`"tao"` chưa hề gọi `themBloom` — MỘT trong hai vị trí băm của nó
(có thể CẢ hai) còn TẮT, `coTheCoBloom` trả về `false` ngay LẬP tức
tại vòng lặp đầu tiên gặp bit tắt. Đây LÀ âm tính THẬT — bloom nói
đúng "chắc chắn không có".
::::

::::predict{#doan-dua-chua-them commitOnce}
`bo` hiện chỉ chứa "cam" VÀ "buoi" (từ ví dụ đầu). Byte kiểm tra một
khoá HOÀN toàn khác, chưa hề động tới:

```typescript
console.log(coTheCoBloom(bo, "dua"));
```

Dòng cuối in ra gì?

:::opt{correct}
`false`
:::

:::opt
`true` — vì bloom filter LUÔN trả về `true` cho MỌI khoá chưa kiểm
tra LẦN nào, chỉ trả `false` sau khi đã bị TỪ chối một lần
::why
Gần đúng ở việc bạn nghĩ TỚI một trạng thái "chưa biết" nào đó cho
khoá hoàn toàn MỚI — một trực giác hợp lý nếu nghĩ bloom LÀ một bộ
nhớ đệm ghi nhớ LỊCH sử truy vấn.

Chỗ lệch: `coTheCoBloom` KHÔNG hề nhớ những lần kiểm tra TRƯỚC —
mỗi lần gọi TÍNH lại từ đầu, chỉ dựa VÀO trạng thái các bit hiện
tại. Hai vị trí băm của `"dua"` — MỘT trong số đó vẫn CÒN tắt (chưa
bao giờ được `themBloom` bật lên bởi `"cam"` hay `"buoi"`) — nên
hàm trả về `false` ngay khi gặp bit tắt ĐẦU tiên.
::
:::

:::opt
Máy báo lỗi — vì `"dua"` chưa từng được `themBloom` thêm vào, kiểm
tra một khoá LẠ như vậy LÀ thao tác không hợp lệ
::why
Gần đúng ở việc bạn để Ý `"dua"` THẬT sự chưa từng xuất hiện trong
`bo` — một quan sát đúng.

Chỗ lệch: `coTheCoBloom` được THIẾT kế để kiểm tra BẤT KỲ chuỗi
nào, kể cả chuỗi CHƯA từng thêm — đó chính LÀ mục đích của nó (trả
lời "có THỂ có" hay "chắc chắn không"). Không có `throw` nào cả,
hàm luôn trả về MỘT giá trị `boolean`.
::
:::
::::

::::code{#viet_them_bloom}
Hoàn thiện `themBloom` — bật đúng bit mà `bamDon` chỉ tới, cho MỖI
hàm băm (`seed`).

```typescript title=starter
function bamDon(s: string, seed: number, kichThuoc: number): number {
  let h = seed;
  for (let i = 0; i < s.length; i++) {
    h = (h * 31 + s.charCodeAt(i)) >>> 0;
  }
  return h % kichThuoc;
}

interface BoLocBloom {
  bit: boolean[];
  kichThuoc: number;
  soHam: number;
}

function taoBoLocBloom(kichThuoc: number, soHam: number): BoLocBloom {
  return { bit: new Array(kichThuoc).fill(false), kichThuoc, soHam };
}

function themBloom(bo: BoLocBloom, khoa: string): void {
  for (let seed = 0; seed < bo.soHam; seed++) {
    const viTri = bamDon(khoa, seed, bo.kichThuoc);
    ___
  }
}

function coTheCoBloom(bo: BoLocBloom, khoa: string): boolean {
  for (let seed = 0; seed < bo.soHam; seed++) {
    const viTri = bamDon(khoa, seed, bo.kichThuoc);
    if (bo.bit[viTri] !== true) return false;
  }
  return true;
}

const bo = taoBoLocBloom(8, 2);
themBloom(bo, "cam");
themBloom(bo, "buoi");
console.log(coTheCoBloom(bo, "cam"), coTheCoBloom(bo, "buoi"), coTheCoBloom(bo, "tao"));
```

```typescript title=solution
function bamDon(s: string, seed: number, kichThuoc: number): number {
  let h = seed;
  for (let i = 0; i < s.length; i++) {
    h = (h * 31 + s.charCodeAt(i)) >>> 0;
  }
  return h % kichThuoc;
}

interface BoLocBloom {
  bit: boolean[];
  kichThuoc: number;
  soHam: number;
}

function taoBoLocBloom(kichThuoc: number, soHam: number): BoLocBloom {
  return { bit: new Array(kichThuoc).fill(false), kichThuoc, soHam };
}

function themBloom(bo: BoLocBloom, khoa: string): void {
  for (let seed = 0; seed < bo.soHam; seed++) {
    const viTri = bamDon(khoa, seed, bo.kichThuoc);
    bo.bit[viTri] = true;
  }
}

function coTheCoBloom(bo: BoLocBloom, khoa: string): boolean {
  for (let seed = 0; seed < bo.soHam; seed++) {
    const viTri = bamDon(khoa, seed, bo.kichThuoc);
    if (bo.bit[viTri] !== true) return false;
  }
  return true;
}

const bo = taoBoLocBloom(8, 2);
themBloom(bo, "cam");
themBloom(bo, "buoi");
console.log(coTheCoBloom(bo, "cam"), coTheCoBloom(bo, "buoi"), coTheCoBloom(bo, "tao"));
```

```typescript title=test
const bo2 = taoBoLocBloom(8, 2);
if (coTheCoBloom(bo2, "cam") !== false) throw new Error("bo rong -- chua them gi thi khong the co gi");
themBloom(bo2, "cam");
if (coTheCoBloom(bo2, "cam") !== true) throw new Error("da them cam -- phai bao co the co");
themBloom(bo2, "buoi");
if (coTheCoBloom(bo2, "buoi") !== true) throw new Error("da them buoi -- phai bao co the co");
if (coTheCoBloom(bo2, "cam") !== true) throw new Error("them buoi khong duoc lam mat cam da them truoc do");
if (coTheCoBloom(bo2, "tao") !== false) throw new Error("tao chua tung them -- it nhat mot bit phai con tat");
if (bo2.bit.filter((b) => b).length === 0) throw new Error("phai co it nhat mot bit duoc bat len sau khi them");
```

:::hints
- kind: attention
  body: "Blank nam trong themBloom, dong bat bit tai viTri da tinh san. Gan true vao dung phan tu mang bo.bit."
- kind: strategy
  body: "bo.bit[viTri] = true;"
- kind: one-line
  body: "bo.bit[viTri] = true;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "true true false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Thêm và kiểm tra hoạt động ĐÚNG. Nhưng bloom filter một mình chưa
ích gì — nó cần đứng TRƯỚC một SSTable, để làm gì?
::::

::::reflect{#nghi-lai}
`themBloom` bật bit TẠI mọi vị trí mà các hàm băm chỉ tới — mỗi khoá
CHIẾM một vài bit, có thể CHIA sẻ bit với khoá khác (đó chính LÀ
nguồn gốc dương tính giả). `coTheCoBloom` chỉ cần MỘT bit tắt LÀ đủ
để khẳng định CHẮC chắn "không có". Một bloom filter đứng MỘT mình
chỉ trả lời có/không — giá trị THẬT của nó là đứng TRƯỚC một
SSTable, quyết định CÓ đáng quét hay không. Ghép chúng LẠI thế nào?
::::

::::checkpoint{mastery=0.8}
::::
