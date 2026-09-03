---
id: co-so-du-lieu.thac-du-lieu.boss-thac-du-lieu
title: "BOSS — Thác dữ liệu"
summary: "Ghép TRỌN q04: hai đợt ghi qua memtable, hai lần flush thành SSTable, bloom filter lọc trước khi quét, đọc mới nhất trước (kể cả khi đó LÀ một bia mộ), compaction gộp sạch, đo write amplification — VÀ docLSM đọc memtable HIỆN tại trước tiên, chỉ rơi xuống SSTable khi cần."
locale: vi
track: co-so-du-lieu
module: thac-du-lieu
order: 14
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.write-amplification]
concepts: [db.boss-q04]
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
Memtable, flush, đọc mới nhất trước, chỉ mục thưa, bloom filter,
tombstone, compaction, write amplification — tám mảnh. Ghép TẤT cả
vào một hệ thống LSM hoàn chỉnh trông ra sao?
::::

::::explain{#toan-bo-pipeline}
Hai đợt ghi, hai lần flush, tìm qua bloom filter, đọc ĐÚNG bản mới
nhất kể cả khi ĐÓ là một bia mộ:

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

function xoaMemtable(bang: Memtable, khoa: string): void {
  chenMemtable(bang, khoa, null);
}

interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

function flush(bang: Memtable): SSTable {
  return { khoa: [...bang.khoa], giaTri: [...bang.giaTri] };
}

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

const m1 = taoMemtable();
chenMemtable(m1, "buoi", "8000");
chenMemtable(m1, "cam", "5000");
chenMemtable(m1, "tao", "12000");
const ssOld = flush(m1);

const m2 = taoMemtable();
chenMemtable(m2, "cam", "5500");
chenMemtable(m2, "dua", "3000");
xoaMemtable(m2, "buoi");
const ssNew = flush(m2);

const boOld = taoBoLocBloom(8, 2);
themBloom(boOld, "buoi");
themBloom(boOld, "cam");
themBloom(boOld, "tao");
const boNew = taoBoLocBloom(8, 2);
themBloom(boNew, "buoi");
themBloom(boNew, "cam");
themBloom(boNew, "dua");

const rCam = timVoiBloom([ssOld, ssNew], [boOld, boNew], "cam");
const rBuoi = timVoiBloom([ssOld, ssNew], [boOld, boNew], "buoi");
console.log(rCam.giaTri, rCam.soLanQuet);
console.log(rBuoi.giaTri, rBuoi.soLanQuet);
```

```text title=readonly
5500 1
null 1
```

`"cam"` tìm thấy NGAY Ở `ssNew` (`5500`, giá trị MỚI nhất). `"buoi"`
tìm thấy Ở `ssNew` — nhưng đó LÀ một bia mộ (`null`), VẪN được trả
về ĐÚNG như vậy, dừng NGAY — không hề lan xuống `ssOld` (nơi `buoi`
còn giá trị THẬT `8000`, đã lỗi thời). Bia mộ CHE khuất mọi bản ghi
cũ hơn của cùng khoá đó.
::::

::::example{#nen-va-do-khuech-dai}
Nén CẢ hai SSTable, đo write amplification:

```typescript title=readonly
function gopSSTable(danhSach: SSTable[]): SSTable {
  const banDo = new Map<string, string | null>();
  for (const bang of danhSach) {
    for (let i = 0; i < bang.khoa.length; i++) {
      const k = bang.khoa[i];
      const v = bang.giaTri[i];
      if (k !== undefined) banDo.set(k, v === undefined ? null : v);
    }
  }
  const khoaSapXep = [...banDo.keys()].sort();
  const khoa: string[] = [];
  const giaTri: (string | null)[] = [];
  for (const k of khoaSapXep) {
    const v = banDo.get(k);
    if (v !== null && v !== undefined) { khoa.push(k); giaTri.push(v); }
  }
  return { khoa, giaTri };
}

function demByte(bang: SSTable): number {
  let tong = 0;
  for (let i = 0; i < bang.khoa.length; i++) {
    const k = bang.khoa[i];
    const v = bang.giaTri[i];
    tong += (k?.length ?? 0) + (v?.length ?? 0);
  }
  return tong;
}

const gop = gopSSTable([ssOld, ssNew]);
console.log(gop.khoa.join(","), gop.giaTri.join(","));
console.log(Math.round(((demByte(ssOld) + demByte(ssNew)) / demByte(gop)) * 100) / 100);
```

```text title=readonly
cam,dua,tao 5500,3000,12000
1.86
```

`buoi` biến MẤT hoàn toàn (bia mộ bị loại khi nén TOÀN bộ). Mỗi
byte dữ liệu SỐNG đã tốn TRUNG bình `1.86` byte ghi thật xuống đĩa
— cái GIÁ của việc đổi ghi tuần TỰ nhanh lấy công VIỆC dọn dẹp Ở
nền.
::::

::::predict{#doan-doc-lsm-bia-mo commitOnce}
Một memtable `m3` HIỆN tại (chưa flush) chỉ có ĐÚNG một khoá:
`chenMemtable(m3, "tao", "99999")`. Byte đọc `"buoi"` — khoá KHÔNG
nằm trong `m3`:

```typescript
const m3 = taoMemtable();
chenMemtable(m3, "tao", "99999");
console.log(docLSM(m3, [ssOld, ssNew], [boOld, boNew], "buoi"));
```

(`docLSM` kiểm tra memtable TRƯỚC, rồi mới rơi xuống các SSTable
nếu KHÔNG tìm thấy Ở đó.) Dòng cuối in ra gì?

:::opt{correct}
`null`
:::

:::opt
`undefined` — vì `m3` không hề chứa `"buoi"`, nên `docLSM` phải
kết luận NGAY là khoá đó không tồn tại Ở đâu cả
::why
Gần đúng ở việc bạn quan sát ĐÚNG: `m3` THẬT sự không chứa
`"buoi"` — một quan sát chính xác về bước ĐẦU tiên.

Chỗ lệch: `m3` không có `"buoi"` chỉ có NGHĨA là bước kiểm tra
memtable trả về `undefined`, KHIẾN `docLSM` đi TIẾP xuống
`timVoiBloom` trên các SSTable — nơi `"buoi"` được tìm thấy Ở
`ssNew` DƯỚI dạng bia mộ (`null`). Không tìm Ở memtable KHÔNG có
nghĩa LÀ không tồn tại Ở đâu cả — nó chỉ có nghĩa LÀ "hãy tìm tiếp
Ở nơi khác".
::
:::

:::opt
`8000` — vì `"buoi"` vẫn còn giá trị thật Ở `ssOld`, VÀ đó LÀ nơi
`"buoi"` được ghi lần đầu, nên LÀ giá trị "gốc" đúng nhất
::why
Gần đúng ở việc bạn nhớ ĐÚNG `"buoi"=8000` từng tồn tại thật Ở
`ssOld` — một sự kiện có thật.

Chỗ lệch: `timVoiBloom` quét TỪ SSTable mới NHẤT trở VỀ cũ, dừng
NGAY khi tìm thấy bản ghi ĐẦU tiên (dù đó LÀ giá trị thật hay bia
mộ) — `ssNew` (mới hơn `ssOld`) CÓ bia mộ cho `"buoi"`, được tìm
thấy TRƯỚC, hàm trả VỀ ngay, KHÔNG bao giờ chạm tới `ssOld`.
::
:::
::::

::::code{#viet_doc_lsm}
Hoàn thiện `docLSM` — đọc Ở memtable TRƯỚC (dữ liệu MỚI nhất, chưa
flush), chỉ rơi xuống các SSTable khi memtable KHÔNG có khoá đó.

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

function xoaMemtable(bang: Memtable, khoa: string): void {
  chenMemtable(bang, khoa, null);
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

interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

function flush(bang: Memtable): SSTable {
  return { khoa: [...bang.khoa], giaTri: [...bang.giaTri] };
}

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

function docLSM(memtableHienTai: Memtable, danhSachSSTable: SSTable[], danhSachBloom: BoLocBloom[], khoa: string): string | null | undefined {
  const tuMemtable = timMemtable(memtableHienTai, khoa);
  ___
  const ketQuaSSTable = timVoiBloom(danhSachSSTable, danhSachBloom, khoa);
  return ketQuaSSTable.giaTri;
}

const m1 = taoMemtable();
chenMemtable(m1, "buoi", "8000");
chenMemtable(m1, "cam", "5000");
chenMemtable(m1, "tao", "12000");
const ssOld = flush(m1);

const m2 = taoMemtable();
chenMemtable(m2, "cam", "5500");
chenMemtable(m2, "dua", "3000");
xoaMemtable(m2, "buoi");
const ssNew = flush(m2);

const boOld = taoBoLocBloom(8, 2);
themBloom(boOld, "buoi");
themBloom(boOld, "cam");
themBloom(boOld, "tao");
const boNew = taoBoLocBloom(8, 2);
themBloom(boNew, "buoi");
themBloom(boNew, "cam");
themBloom(boNew, "dua");

const m3 = taoMemtable();
chenMemtable(m3, "tao", "99999");
console.log(docLSM(m3, [ssOld, ssNew], [boOld, boNew], "tao"), docLSM(m3, [ssOld, ssNew], [boOld, boNew], "cam"));
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

function xoaMemtable(bang: Memtable, khoa: string): void {
  chenMemtable(bang, khoa, null);
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

interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

function flush(bang: Memtable): SSTable {
  return { khoa: [...bang.khoa], giaTri: [...bang.giaTri] };
}

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

function docLSM(memtableHienTai: Memtable, danhSachSSTable: SSTable[], danhSachBloom: BoLocBloom[], khoa: string): string | null | undefined {
  const tuMemtable = timMemtable(memtableHienTai, khoa);
  if (tuMemtable !== undefined) return tuMemtable;
  const ketQuaSSTable = timVoiBloom(danhSachSSTable, danhSachBloom, khoa);
  return ketQuaSSTable.giaTri;
}

const m1 = taoMemtable();
chenMemtable(m1, "buoi", "8000");
chenMemtable(m1, "cam", "5000");
chenMemtable(m1, "tao", "12000");
const ssOld = flush(m1);

const m2 = taoMemtable();
chenMemtable(m2, "cam", "5500");
chenMemtable(m2, "dua", "3000");
xoaMemtable(m2, "buoi");
const ssNew = flush(m2);

const boOld = taoBoLocBloom(8, 2);
themBloom(boOld, "buoi");
themBloom(boOld, "cam");
themBloom(boOld, "tao");
const boNew = taoBoLocBloom(8, 2);
themBloom(boNew, "buoi");
themBloom(boNew, "cam");
themBloom(boNew, "dua");

const m3 = taoMemtable();
chenMemtable(m3, "tao", "99999");
console.log(docLSM(m3, [ssOld, ssNew], [boOld, boNew], "tao"), docLSM(m3, [ssOld, ssNew], [boOld, boNew], "cam"));
```

```typescript title=test
const m1t = taoMemtable();
chenMemtable(m1t, "buoi", "8000");
chenMemtable(m1t, "cam", "5000");
chenMemtable(m1t, "tao", "12000");
const ssOldT = flush(m1t);

const m2t = taoMemtable();
chenMemtable(m2t, "cam", "5500");
chenMemtable(m2t, "dua", "3000");
xoaMemtable(m2t, "buoi");
const ssNewT = flush(m2t);

const boOldT = taoBoLocBloom(8, 2);
themBloom(boOldT, "buoi");
themBloom(boOldT, "cam");
themBloom(boOldT, "tao");
const boNewT = taoBoLocBloom(8, 2);
themBloom(boNewT, "buoi");
themBloom(boNewT, "cam");
themBloom(boNewT, "dua");

const m3t = taoMemtable();
chenMemtable(m3t, "tao", "99999");

if (docLSM(m3t, [ssOldT, ssNewT], [boOldT, boNewT], "tao") !== "99999") throw new Error("tao dang co ban cap nhat MOI trong memtable -- phai doc tu memtable, khong phai SSTable cu");
if (docLSM(m3t, [ssOldT, ssNewT], [boOldT, boNewT], "cam") !== "5500") throw new Error("cam khong o memtable m3t -- phai roi xuong SSTable, tim thay o ssNewT");
if (docLSM(m3t, [ssOldT, ssNewT], [boOldT, boNewT], "buoi") !== null) throw new Error("buoi khong o memtable -- roi xuong SSTable, tim thay bia mo (null) o ssNewT");
if (docLSM(m3t, [ssOldT, ssNewT], [boOldT, boNewT], "xoai") !== undefined) throw new Error("xoai khong o dau ca -- phai la undefined");

const m4t = taoMemtable();
chenMemtable(m4t, "cam", "TUOI-NHAT");
if (docLSM(m4t, [ssOldT, ssNewT], [boOldT, boNewT], "cam") !== "TUOI-NHAT") throw new Error("memtable LUON uu tien hon moi SSTable, du SSTable cung co khoa do");
```

:::hints
- kind: attention
  body: "Blank ngay sau khi tinh tuMemtable. Neu tuMemtable khac undefined (tim thay o memtable, ke ca gia tri null la bia mo), tra ve NGAY, khong roi xuong SSTable."
- kind: strategy
  body: "if (tuMemtable !== undefined) return tuMemtable;"
- kind: one-line
  body: "if (tuMemtable !== undefined) return tuMemtable;"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "99999 5500"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một hệ thống LSM hoàn chỉnh — ghi nhanh vào bộ nhớ, đọc ĐÚNG qua
mọi tầng, dọn dẹp Ở nền. q04 hoàn TẤT.
::::

::::reflect{#nghi-lai}
q04 xây một LSM THẬT: ghi trước vào memtable (tránh chạm đĩa MỖI
lần), flush thành SSTable bất biến khi đầy, đọc mới NHẤT trước qua
nhiều SSTable, chỉ mục thưa VÀ bloom filter để tránh quét thừa,
tombstone để xoá MÀ không sửa tại chỗ, compaction để dọn SẠCH bia
mộ VÀ đo write amplification — cái GIÁ phải trả để đổi lấy ghi
nhanh. So SÁNH trực tiếp với q03: B+Tree ghi CHẬM hơn (chạm đĩa
ngay) nhưng KHÔNG khuếch đại; LSM ghi NHANH hơn nhưng phải trả GIÁ
khi nén. Cả hai đều LÀ cách tổ chức một kho lưu trữ — không CÓ
"đúng" tuyệt đối, chỉ CÓ đánh đổi phù hợp VỚI tải công việc. q05
"Bốn chữ cái ACID" chuyển sang một câu hỏi KHÁC hẳn: khi NHIỀU
giao dịch cùng đọc/ghi MỘT kho lưu trữ (dù LÀ B+Tree hay LSM) đồng
thời, làm sao đảm bảo chúng không GIẪM lên nhau?
::::

::::checkpoint{mastery=0.85}
::::
