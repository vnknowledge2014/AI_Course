---
id: co-so-du-lieu.thac-du-lieu.bloom-loc-truoc-khi-quet
title: Bloom lọc trước khi quét
summary: "timVoiBloom đi qua các SSTable từ MỚI xuống CŨ — nhưng trước khi quét nhị phân một SSTable nào, nó hỏi bloom TƯƠNG ỨNG trước. Bloom báo 'chắc chắn không' thì bỏ qua hẳn, không tốn công quét — soLanQuet đếm đúng số SSTable THẬT SỰ bị chạm tới."
locale: vi
track: co-so-du-lieu
module: thac-du-lieu
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [db.bloom-skips-sstable-scan]
requires: [db.bloom-filter-implementation]
concepts: [db.bloom-skips-sstable-scan]
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
Bloom filter TRẢ lời có/không (bài trước) — nhưng câu trả lời đó
dùng để LÀM gì, khi thật sự tìm một khoá qua NHIỀU SSTable?
::::

::::explain{#loc-truoc-khi-quet}
`timVoiBloom` đi qua các SSTable TỪ mới xuống cũ (như bài
`doc-moi-nhat-truoc`) — nhưng TRƯỚC khi quét nhị phân một SSTable
nào, nó hỏi bloom TƯƠNG ỨNG trước. Bloom báo "chắc chắn không" thì
BỎ QUA hẳn, không tốn công quét:

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

interface KetQuaTim {
  giaTri: string | null | undefined;
  soLanQuet: number;
}

function timVoiBloom(danhSachSSTable: SSTable[], danhSachBloom: BoLocBloom[], khoa: string): KetQuaTim {
  let soLanQuet = 0;
  for (let i = danhSachSSTable.length - 1; i >= 0; i--) {
    const bang = danhSachSSTable[i];
    const bo = danhSachBloom[i];
    if (bang === undefined || bo === undefined) continue;
    if (!coTheCoBloom(bo, khoa)) continue;
    soLanQuet++;
    const kq = timTrongMotSSTable(bang, khoa);
    if (kq !== undefined) return { giaTri: kq, soLanQuet };
  }
  return { giaTri: undefined, soLanQuet };
}

const ssCu: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const ssMoi: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
const boCu = taoBoLocBloom(8, 2);
themBloom(boCu, "buoi");
themBloom(boCu, "cam");
themBloom(boCu, "tao");
const boMoi = taoBoLocBloom(8, 2);
themBloom(boMoi, "cam");
themBloom(boMoi, "dua");

const kq1 = timVoiBloom([ssCu, ssMoi], [boCu, boMoi], "tao");
console.log(kq1.giaTri, kq1.soLanQuet);
```

```text title=readonly
12000 1
```

`"tao"` KHÔNG hề nằm trong `ssMoi` — bloom `boMoi` báo "chắc chắn
không" NGAY (`coTheCoBloom` trả `false`), nhánh `continue` chạy,
`ssMoi` KHÔNG hề bị quét nhị phân. Chỉ `ssCu` được quét THẬT
(`soLanQuet=1`), tìm thấy `"tao"` VỚI giá trị `"12000"`.
::::

::::example{#tim-thay-ngay-o-sstable-moi}
Một khoá NẰM ở SSTable mới NHẤT — dừng ngay, KHÔNG cần chạm SSTable
cũ hơn:

```typescript title=readonly
const kq2 = timVoiBloom([ssCu, ssMoi], [boCu, boMoi], "cam");
console.log(kq2.giaTri, kq2.soLanQuet);
```

```text title=readonly
5500 1
```

`bloom boMoi` báo "có thể có" cho `"cam"` (đúng — đã thêm), quét
`ssMoi` tìm thấy NGAY (`"5500"`, giá trị MỚI hơn `"5000"` trong
`ssCu`) — hàm `return` LUÔN, `ssCu` không hề bị chạm tới, dù nó
CŨNG chứa `"cam"`.
::::

::::predict{#doan-so-lan-quet-khoa-khong-ton-tai commitOnce}
Byte tìm một khoá KHÔNG hề tồn tại ở BẤT KỲ SSTable nào, VÀ tình
cờ bị bloom của CẢ hai SSTable từ chối ngay:

```typescript
const kq3 = timVoiBloom([ssCu, ssMoi], [boCu, boMoi], "nho");
console.log(kq3.giaTri, kq3.soLanQuet);
```

Dòng cuối in ra gì? (Gợi ý: `"nho"` không khớp bit nào TRONG cả hai
bloom.)

:::opt{correct}
`undefined 0`
:::

:::opt
`undefined 2` — vì dù bloom từ chối, hàm VẪN phải quét CẢ hai
SSTable một lần để chắc chắn "nho" thật sự không tồn tại
::why
Gần đúng ở việc bạn nghĩ TỚI một bước "xác nhận LẦN cuối" hợp lý —
kiểm tra kỹ TRƯỚC khi kết luận "không có".

Chỗ lệch: bloom filter ĐƯỢC thiết kế để KHÔNG BAO GIỜ báo sai kiểu
"không có" khi THẬT có (không false negative) — nghĩa LÀ khi nó nói
"chắc chắn không", điều đó CHẮC chắn đúng, không cần xác nhận LẠI
bằng cách quét. `soLanQuet` chỉ tăng khi THẬT SỰ gọi
`timTrongMotSSTable` — nếu bloom từ chối cả hai, biến ĐÓ không hề
tăng, giữ nguyên `0`.
::
:::

:::opt
Máy báo lỗi — vì `timVoiBloom` không có nhánh xử LÝ trường hợp
"không SSTable nào được quét CẢ"
::why
Gần đúng ở việc bạn nghĩ TỚI trường hợp biên "không quét gì cả" —
một mối lo hợp lý về code không đầy đủ.

Chỗ lệch: vòng `for` chạy HẾT (dù mọi `continue` chạy liên TỤC),
rồi rơi ra ngoài vòng lặp TỰ nhiên, chạm dòng `return { giaTri:
undefined, soLanQuet };` — trả về BÌNH thường, không hề `throw`.
::
:::
::::

::::code{#viet_tim_voi_bloom}
Hoàn thiện `timVoiBloom` — TRƯỚC khi quét một SSTable, hỏi bloom
tương ứng, bỏ qua NẾU nó báo "chắc chắn không".

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

interface KetQuaTim {
  giaTri: string | null | undefined;
  soLanQuet: number;
}

function timVoiBloom(danhSachSSTable: SSTable[], danhSachBloom: BoLocBloom[], khoa: string): KetQuaTim {
  let soLanQuet = 0;
  for (let i = danhSachSSTable.length - 1; i >= 0; i--) {
    const bang = danhSachSSTable[i];
    const bo = danhSachBloom[i];
    if (bang === undefined || bo === undefined) continue;
    ___
    soLanQuet++;
    const kq = timTrongMotSSTable(bang, khoa);
    if (kq !== undefined) return { giaTri: kq, soLanQuet };
  }
  return { giaTri: undefined, soLanQuet };
}

const ssCu: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const ssMoi: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
const boCu = taoBoLocBloom(8, 2);
themBloom(boCu, "buoi");
themBloom(boCu, "cam");
themBloom(boCu, "tao");
const boMoi = taoBoLocBloom(8, 2);
themBloom(boMoi, "cam");
themBloom(boMoi, "dua");

const kq1 = timVoiBloom([ssCu, ssMoi], [boCu, boMoi], "tao");
console.log(kq1.giaTri, kq1.soLanQuet);
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

interface KetQuaTim {
  giaTri: string | null | undefined;
  soLanQuet: number;
}

function timVoiBloom(danhSachSSTable: SSTable[], danhSachBloom: BoLocBloom[], khoa: string): KetQuaTim {
  let soLanQuet = 0;
  for (let i = danhSachSSTable.length - 1; i >= 0; i--) {
    const bang = danhSachSSTable[i];
    const bo = danhSachBloom[i];
    if (bang === undefined || bo === undefined) continue;
    if (!coTheCoBloom(bo, khoa)) continue;
    soLanQuet++;
    const kq = timTrongMotSSTable(bang, khoa);
    if (kq !== undefined) return { giaTri: kq, soLanQuet };
  }
  return { giaTri: undefined, soLanQuet };
}

const ssCu: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const ssMoi: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
const boCu = taoBoLocBloom(8, 2);
themBloom(boCu, "buoi");
themBloom(boCu, "cam");
themBloom(boCu, "tao");
const boMoi = taoBoLocBloom(8, 2);
themBloom(boMoi, "cam");
themBloom(boMoi, "dua");

const kq1 = timVoiBloom([ssCu, ssMoi], [boCu, boMoi], "tao");
console.log(kq1.giaTri, kq1.soLanQuet);
```

```typescript title=test
const ssCu2: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const ssMoi2: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
const boCu2 = taoBoLocBloom(8, 2);
themBloom(boCu2, "buoi");
themBloom(boCu2, "cam");
themBloom(boCu2, "tao");
const boMoi2 = taoBoLocBloom(8, 2);
themBloom(boMoi2, "cam");
themBloom(boMoi2, "dua");

const rTao = timVoiBloom([ssCu2, ssMoi2], [boCu2, boMoi2], "tao");
if (rTao.giaTri !== "12000") throw new Error("tao chi nam o ssCu, phai tim thay 12000");
if (rTao.soLanQuet !== 1) throw new Error("bloom cua ssMoi phai bao khong co tao -- chi quet ssCu, soLanQuet phai la 1");

const rCam = timVoiBloom([ssCu2, ssMoi2], [boCu2, boMoi2], "cam");
if (rCam.giaTri !== "5500") throw new Error("cam nam o ca hai, phai tra ve gia tri MOI nhat tu ssMoi");
if (rCam.soLanQuet !== 1) throw new Error("tim thay ngay o ssMoi -- khong duoc cham toi ssCu");

const rNho = timVoiBloom([ssCu2, ssMoi2], [boCu2, boMoi2], "nho");
if (rNho.giaTri !== undefined) throw new Error("nho khong ton tai o dau ca");
if (rNho.soLanQuet !== 0) throw new Error("ca hai bloom phai tu choi nho ngay -- khong duoc quet SSTable nao");
```

:::hints
- kind: attention
  body: "Blank thay the cho buoc hoi bloom TRUOC khi tang soLanQuet. Neu coTheCoBloom(bo, khoa) la false, phai continue (bo qua SSTable nay)."
- kind: strategy
  body: "if (!coTheCoBloom(bo, khoa)) continue;"
- kind: one-line
  body: "if (!coTheCoBloom(bo, khoa)) continue;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "12000 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bloom lọc trước, quét THẬT chỉ khi cần — số SSTable chạm tới GIẢM
hẳn. Nhưng LSM còn thiếu một mảnh: xoá một khoá diễn RA thế nào?
::::

::::reflect{#nghi-lai}
`timVoiBloom` hỏi bloom TRƯỚC mỗi lần định quét một SSTable — bloom
KHÔNG BAO GIỜ khiến bỏ SÓT một khoá thật sự tồn tại (không false
negative), nên bỏ qua theo lời NÓ luôn an toàn. Số SSTable thật sự
bị quét (`soLanQuet`) giảm ĐÁNG kể khi có NHIỀU SSTable cũ không
liên quan. Đọc VÀ ghi giờ đã đủ mảnh — nhưng CẬP nhật một giá trị
thì dễ (ghi bản mới), còn XOÁ một khoá thì sao — dữ liệu đã nằm
TRÊN nhiều SSTable bất biến, không thể sửa TẠI chỗ?
::::

::::checkpoint{mastery=0.8}
::::
