---
id: co-so-du-lieu.thac-du-lieu.write-amplification
title: Write amplification — đo chi phí nén
summary: "demByte đếm TỔNG số ký tự (khoá + giá trị) trong một SSTable — bia mộ tính là 0. Cộng byte của MỌI SSTable đã flush, chia CHO byte còn lại sau khi nén, ra 'write amplification' — cái giá phải trả để đổi lấy ghi tuần tự nhanh."
locale: vi
track: co-so-du-lieu
module: thac-du-lieu
order: 13
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.write-amplification]
requires: [db.compaction-merge]
concepts: [db.write-amplification]
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
Nén (bài trước) ghi LẠI toàn bộ dữ liệu sống thêm MỘT lần — chi phí
đó lớn CỠ nào so với kích thước THẬT của dữ liệu?
::::

::::explain{#dem-byte-va-ti-le}
`demByte` đếm TỔNG số ký tự (khoá + giá trị) trong MỘT SSTable —
coi bia mộ (`null`) LÀ độ dài `0`. Cộng byte của MỌI SSTable đã
flush, chia CHO byte còn lại SAU khi nén, ra "write amplification":

```typescript title=readonly
interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
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

const ss1: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const ss2: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
const ss3: SSTable = { khoa: ["buoi", "le"], giaTri: [null, "4000"] };
const gop = gopSSTable([ss1, ss2, ss3]);

const tongGhi = demByte(ss1) + demByte(ss2) + demByte(ss3);
const byteLogic = demByte(gop);
console.log(demByte(ss1), demByte(ss2), demByte(ss3));
console.log(tongGhi, byteLogic);
```

```text title=readonly
23 14 10
47 28
```

Ba lần flush ghi TỔNG cộng `23 + 14 + 10 = 47` byte XUỐNG đĩa. Sau
khi nén, dữ liệu SỐNG thật sự chỉ CÒN `28` byte (bia mộ VÀ bản ghi
cũ bị GHI đè đã biến mất). `47` byte đã GHI, `28` byte CÒN lại —
mỗi byte dữ liệu THẬT đã tốn HƠN một lần ghi.
::::

::::example{#ti-le-khuech-dai}
Tính tỉ lệ — bao nhiêu LẦN mỗi byte dữ liệu thật đã bị ghi XUỐNG
đĩa, tính CẢ những lần ghi đè VÀ bia mộ:

```typescript title=readonly
console.log(Math.round((tongGhi / byteLogic) * 100) / 100);
```

```text title=readonly
1.68
```

`47 / 28 ≈ 1.68` — MỖI byte dữ liệu thật đã tốn TRUNG bình `1.68`
byte ghi thật sự XUỐNG đĩa (đếm cả `cam` bị ghi ĐÈ một lần, `buoi`
bị ghi rồi XOÁ). Đây LÀ "write amplification": chi phí PHỤ trội mà
LSM trả để đổi lấy ghi tuần tự NHANH lúc đầu (bài `vi-sao-ghi-tai-
cho-ton`).
::::

::::predict{#doan-byte-mot-sstable-don commitOnce}
Byte tạo MỘT SSTable chỉ có đúng một khoá: `{khoa: ["an"], giaTri:
["1"]}`.

```typescript
const donGian: SSTable = { khoa: ["an"], giaTri: ["1"] };
console.log(demByte(donGian));
```

Dòng cuối in ra gì?

:::opt{correct}
`3`
:::

:::opt
`2` — vì `demByte` chỉ đếm ĐỘ dài của khoá (`"an"` dài `2`), giá
trị không được TÍNH vào tổng
::why
Gần đúng ở việc bạn tính đúng ĐỘ dài của `"an"` — `2` ký tự, một
phép đếm chính xác.

Chỗ lệch: `demByte` cộng CẢ khoá LẪN giá trị — `(k?.length ?? 0) +
(v?.length ?? 0)` — `"an"` dài `2`, `"1"` dài `1`, tổng LÀ `2 + 1 =
3`. Giá trị hoàn toàn CÓ được tính, không bị bỏ qua.
::
:::

:::opt
`1` — vì `demByte` chỉ đếm SỐ khoá trong SSTable, không quan tâm
độ dài từng chuỗi
::why
Gần đúng ở việc bạn nghĩ TỚI một cách đếm "đơn giản hoá" — đếm số
lượng bản GHI, một cách đo hợp lý nhưng KHÁC với thứ hàm này làm.

Chỗ lệch: `demByte` cộng dồn `.length` của TỪNG chuỗi (khoá VÀ giá
trị), không phải đếm SỐ lượng phần tử — với một SSTable CÓ `100`
khoá ngắn, `demByte` sẽ ra một con số LỚN hơn `100` nhiều, không
phải `100`.
::
:::
::::

::::code{#viet_dem_byte}
Hoàn thiện `demByte` — cộng dồn độ dài khoá VÀ giá trị cho mỗi
bản ghi (bia mộ tính LÀ `0`).

```typescript title=starter
interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

function demByte(bang: SSTable): number {
  let tong = 0;
  for (let i = 0; i < bang.khoa.length; i++) {
    const k = bang.khoa[i];
    const v = bang.giaTri[i];
    ___
  }
  return tong;
}

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

const ss1: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const ss2: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
const ss3: SSTable = { khoa: ["buoi", "le"], giaTri: [null, "4000"] };
const gop = gopSSTable([ss1, ss2, ss3]);

const tongGhi = demByte(ss1) + demByte(ss2) + demByte(ss3);
const byteLogic = demByte(gop);
console.log(demByte(ss1), demByte(ss2), demByte(ss3), tongGhi, byteLogic, Math.round((tongGhi / byteLogic) * 100) / 100);
```

```typescript title=solution
interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
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

const ss1: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const ss2: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
const ss3: SSTable = { khoa: ["buoi", "le"], giaTri: [null, "4000"] };
const gop = gopSSTable([ss1, ss2, ss3]);

const tongGhi = demByte(ss1) + demByte(ss2) + demByte(ss3);
const byteLogic = demByte(gop);
console.log(demByte(ss1), demByte(ss2), demByte(ss3), tongGhi, byteLogic, Math.round((tongGhi / byteLogic) * 100) / 100);
```

```typescript title=test
const t: SSTable = { khoa: ["an"], giaTri: ["1"] };
if (demByte(t) !== 3) throw new Error("an (2 ky tu) + 1 (1 ky tu) phai ra 3");

const tRong: SSTable = { khoa: [], giaTri: [] };
if (demByte(tRong) !== 0) throw new Error("SSTable rong phai dem ra 0 byte");

const tBiaMo: SSTable = { khoa: ["x"], giaTri: [null] };
if (demByte(tBiaMo) !== 1) throw new Error("bia mo tinh do dai 0 -- chi con do dai khoa 'x' (1 ky tu)");

const ssA: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
if (demByte(ssA) !== 23) throw new Error("buoi(4)+8000(4)+cam(3)+5000(4)+tao(3)+12000(5) = 23");
```

:::hints
- kind: attention
  body: "Blank la dong cong don vao tong, ben trong vong for. Dung optional chaining (?.) va nullish coalescing (??) vi k, v co the undefined/null."
- kind: strategy
  body: "tong += (k?.length ?? 0) + (v?.length ?? 0);"
- kind: one-line
  body: "tong += (k?.length ?? 0) + (v?.length ?? 0);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "23 14 10 47 28 1.68"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Write amplification đo được — mỗi byte thật đã tốn `1.68` byte ghi
thật. Giờ ghép TẤT cả q04 lại: memtable, flush, bloom, compaction,
đo được không?
::::

::::reflect{#nghi-lai}
Write amplification LÀ cái GIÁ phải trả để đổi lấy ghi tuần TỰ
nhanh — mỗi lần ghi ĐÈ hay xoá một khoá đã tồn tại, byte CŨ vẫn
từng bị ghi thật, chỉ MẤT đi khi nén. B+Tree (q03) trả giá NGƯỢC
lại: ghi tại chỗ chậm HƠN, nhưng KHÔNG có write amplification —
mỗi byte chỉ ghi ĐÚNG một lần. Đây LÀ đánh đổi trung tâm của LSM.
Memtable, flush, đọc mới nhất trước, chỉ mục thưa, bloom filter,
tombstone, compaction, write amplification — tám mảnh RỜI. Ghép
TẤT cả thành một câu chuyện DUY nhất trông ra sao?
::::

::::checkpoint{mastery=0.8}
::::
