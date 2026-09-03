---
id: co-so-du-lieu.thac-du-lieu.chi-muc-thua-cho-sstable
title: Chỉ mục thưa cho SSTable
summary: "xayChiMucThua giữ lại MỘT khoá trong mỗi buoc khoá của SSTable (thưa dần), thay vì load MỌI khoá vào RAM. Tìm kiếm nhảy tới đúng VÙNG qua chỉ mục thưa này, rồi quét tuyến tính NHỎ trong vùng đó — không cần biết trước MỌI khoá để tìm đúng một khoá."
locale: vi
track: co-so-du-lieu
module: thac-du-lieu
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.sparse-index]
requires: [db.read-newest-sstable-first]
concepts: [db.sparse-index]
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
Tìm nhị phân (bài TRƯỚC) trên MỘT SSTable vẫn cần TOÀN bộ khoá nằm
sẵn trong RAM. Nếu SSTable quá LỚN để load hết, làm sao vẫn NHẢY
nhanh tới đúng vùng?
::::

::::explain{#chi-muc-thua}
`xayChiMucThua` giữ lại MỘT khoá trong mỗi `buoc` khoá của SSTable
— một chỉ mục THƯA, nhỏ hơn hẳn toàn bộ dữ liệu, đủ để nhảy TỚI
đúng vùng:

```typescript title=readonly
interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

interface MucThua {
  khoa: string;
  viTri: number;
}

function xayChiMucThua(bang: SSTable, buoc: number): MucThua[] {
  const kq: MucThua[] = [];
  for (let i = 0; i < bang.khoa.length; i += buoc) {
    const k = bang.khoa[i];
    if (k !== undefined) kq.push({ khoa: k, viTri: i });
  }
  return kq;
}

const ssBig: SSTable = { khoa: ["an", "bo", "ca", "da", "ga", "ha", "in", "ka"], giaTri: ["1", "2", "3", "4", "5", "6", "7", "8"] };
const chiMuc = xayChiMucThua(ssBig, 3);
console.log(JSON.stringify(chiMuc));
```

```text title=readonly
[{"khoa":"an","viTri":0},{"khoa":"da","viTri":3},{"khoa":"in","viTri":6}]
```

`ssBig` có `8` khoá — chỉ mục THƯA (`buoc=3`) chỉ giữ `3` mục: khoá
Ở vị trí `0`, `3`, VÀ `6`. KHÔNG cần giữ CẢ `8` khoá trong RAM để
biết vùng NÀO chứa khoá cần tìm.
::::

::::example{#quet-trong-vung}
Dùng chỉ mục thưa để NHẢY tới đúng vùng, rồi quét tuyến tính NHỎ
trong vùng đó:

```typescript title=readonly
function timViTriBatDau(chiMuc: MucThua[], khoa: string): number {
  let viTri = 0;
  for (const muc of chiMuc) {
    if (muc.khoa <= khoa) viTri = muc.viTri; else break;
  }
  return viTri;
}

function timBangChiMucThua(bang: SSTable, chiMuc: MucThua[], khoa: string): string | null | undefined {
  const batDau = timViTriBatDau(chiMuc, khoa);
  for (let i = batDau; i < bang.khoa.length; i++) {
    const k = bang.khoa[i];
    if (k === khoa) return bang.giaTri[i];
    if (k !== undefined && k > khoa) break;
  }
  return undefined;
}

console.log(timBangChiMucThua(ssBig, chiMuc, "ha"));
```

```text title=readonly
6
```

Tìm `"ha"`: `timViTriBatDau` quét chỉ mục THƯA (chỉ `3` mục, KHÔNG
phải `8`) — `"an" <= "ha"` đúng (`viTri=0`), `"da" <= "ha"` đúng
(`viTri=3`), `"in" <= "ha"`? So sánh CHỮ cái đầu: `i` đứng SAU `h`
trong bảng chữ cái, nên `"in"` LỚN hơn `"ha"` — điều kiện SAI, vòng
lặp DỪNG lại, `batDau=3` (mục `"da"` LÀ mục gần nhất không vượt
quá). Quét tuyến TÍNH từ vị trí `3` (`da`, `ga`, `ha`, …) khớp
`"ha"` Ở vị trí `5`, trả về `giaTri[5]="6"`.
::::

::::predict{#doan-tim-khong-ton-tai commitOnce}
Byte tìm khoá `"xx"` — LỚN hơn MỌI khoá trong `ssBig`:

```typescript
console.log(timBangChiMucThua(ssBig, chiMuc, "xx"));
```

Dòng cuối in ra gì?

:::opt{correct}
`undefined`
:::

:::opt
`8` — vì khoá LỚN nhất trong mảng luôn ĐỨNG cuối, tìm một khoá
LỚN hơn tất cả sẽ tự động khớp phần TỬ cuối cùng đó
::why
Gần đúng ở việc bạn nghĩ TỚI vị trí "cuối mảng" như một điểm ĐẶC
biệt — một trực giác dễ hiểu KHI khoá cần tìm rất LỚN.

Chỗ lệch: `timBangChiMucThua` SO sánh CHÍNH XÁC (`k === khoa`), nó
KHÔNG "làm tròn" tới phần tử gần nhất — quét tuyến tính TỪ vị trí
`batDau` chạy tới CUỐI mảng KHÔNG khớp `"xx"` bất KỲ đâu (khoá lớn
nhất thật LÀ `"ka"`, khác `"xx"`), rơi RA khỏi vòng lặp, trả về
`undefined`.
::
:::

:::opt
Máy báo lỗi — vì `timViTriBatDau` không tìm được mục chỉ mục NÀO
thoả `muc.khoa <= "xx"`, khiến `viTri` không được gán giá trị nào
::why
Gần đúng ở việc bạn để Ý: MỌI mục trong `chiMuc` (`an`, `da`, `in`)
đều nhỏ hơn `"xx"` — đúng, cả BA mục đều thoả điều kiện.

Chỗ lệch: chính VÌ cả ba mục ĐỀU thoả `muc.khoa <= "xx"`, `viTri`
được CẬP nhật liên tục TỚI mục CUỐI (`in`, viTri=6) — biến `viTri`
khởi tạo SẴN bằng `0` VÀ luôn có giá trị hợp lệ, không có nhánh nào
để KHÔNG gán được, không lỗi gì.
::
:::
::::

::::code{#viet_xay_chi_muc_thua}
Hoàn thiện `xayChiMucThua` — mỗi `buoc` khoá, giữ lại ĐÚNG một mục
trong chỉ mục.

```typescript title=starter
interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

interface MucThua {
  khoa: string;
  viTri: number;
}

function xayChiMucThua(bang: SSTable, buoc: number): MucThua[] {
  const kq: MucThua[] = [];
  for (let i = 0; i < bang.khoa.length; i += buoc) {
    const k = bang.khoa[i];
    ___
  }
  return kq;
}

const ssBig: SSTable = { khoa: ["an", "bo", "ca", "da", "ga", "ha", "in", "ka"], giaTri: ["1", "2", "3", "4", "5", "6", "7", "8"] };
console.log(JSON.stringify(xayChiMucThua(ssBig, 3)));
```

```typescript title=solution
interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

interface MucThua {
  khoa: string;
  viTri: number;
}

function xayChiMucThua(bang: SSTable, buoc: number): MucThua[] {
  const kq: MucThua[] = [];
  for (let i = 0; i < bang.khoa.length; i += buoc) {
    const k = bang.khoa[i];
    if (k !== undefined) kq.push({ khoa: k, viTri: i });
  }
  return kq;
}

const ssBig: SSTable = { khoa: ["an", "bo", "ca", "da", "ga", "ha", "in", "ka"], giaTri: ["1", "2", "3", "4", "5", "6", "7", "8"] };
console.log(JSON.stringify(xayChiMucThua(ssBig, 3)));
```

```typescript title=test
const ssTest: SSTable = { khoa: ["an", "bo", "ca", "da", "ga", "ha", "in", "ka"], giaTri: ["1", "2", "3", "4", "5", "6", "7", "8"] };
const cm = xayChiMucThua(ssTest, 3);
if (cm.length !== 3) throw new Error("buoc=3 tren 8 khoa phai cho dung 3 muc chi muc");
if (JSON.stringify(cm) !== JSON.stringify([{ khoa: "an", viTri: 0 }, { khoa: "da", viTri: 3 }, { khoa: "in", viTri: 6 }])) throw new Error("chi muc phai giu dung khoa va vi tri o cac buoc 0, 3, 6");
const cm2 = xayChiMucThua(ssTest, 1);
if (cm2.length !== 8) throw new Error("buoc=1 phai giu MOI khoa, khong bo sot");
const cm3 = xayChiMucThua({ khoa: [], giaTri: [] }, 3);
if (cm3.length !== 0) throw new Error("SSTable rong phai cho chi muc rong");
```

:::hints
- kind: attention
  body: "Neu k khac undefined, push { khoa: k, viTri: i } vao kq -- mot dong."
- kind: strategy
  body: "if (k !== undefined) kq.push({ khoa: k, viTri: i });"
- kind: one-line
  body: "if (k !== undefined) kq.push({ khoa: k, viTri: i });"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "[{\"khoa\":\"an\",\"viTri\":0},{\"khoa\":\"da\",\"viTri\":3},{\"khoa\":\"in\",\"viTri\":6}]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chỉ mục thưa — nhảy tới đúng VÙNG mà không cần giữ MỌI khoá trong
RAM. Nhưng NẾU khoá chắc chắn không có Ở một SSTable NÀO đó, có
cách nào biết TRƯỚC mà KHÔNG cần quét gì cả?
::::

::::reflect{#nghi-lai}
`xayChiMucThua` đánh đổi ĐỘ chính xác lấy KÍCH thước — chỉ giữ MỘT
khoá trong mỗi `buoc`, đủ để nhảy TỚI đúng vùng RỒI quét tuyến tính
NHỎ trong đó, không cần TOÀN bộ SSTable nằm sẵn trong RAM. NHƯNG
nếu một khoá KHÔNG hề tồn tại trong SSTable, chỉ mục thưa VẪN buộc
phải quét MỘT vùng (dù nhỏ) rồi mới biết chắc "không có". Có cách
nào biết TRƯỚC — gần như NGAY lập tức, không cần quét gì — rằng một
khoá CHẮC chắn không nằm trong một SSTable?
::::

::::checkpoint{mastery=0.8}
::::
