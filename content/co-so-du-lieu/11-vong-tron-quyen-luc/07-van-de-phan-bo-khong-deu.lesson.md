---
id: co-so-du-lieu.vong-tron-quyen-luc.van-de-phan-bo-khong-deu
title: "Vấn đề phân bố không đều — một điểm mỗi node chưa đủ"
summary: "demPhanBo đếm số khoá mỗi node nhận được. Với 100 khoá mẫu và bốn node (alpha/beta/gamma/delta), mỗi node ĐÚNG một điểm trên vòng — kết quả là {alpha:35, beta:16, gamma:13, delta:36}, chênh nhau tới 2.7 lần dù bam (bài 6) đã là một hàm băm TỐT. Nguyên nhân không phải hàm băm — mà là vị trí NGẪU NHIÊN của bốn điểm trên vòng chia nó thành bốn đoạn có độ DÀI rất khác nhau."
locale: vi
track: co-so-du-lieu
module: vong-tron-quyen-luc
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.van-de-phan-bo-khong-deu]
requires: [db.murmur3-rut-gon]
concepts: [db.van-de-phan-bo-khong-deu]
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
`bam` (bài 6) rải khoá đủ ĐỀU — nhưng "đủ đều" CHO khoá KHÔNG có
nghĩa LÀ "đủ đều" cho NODE. Đo THẬT xem CÁC node có nhận tải bằng
nhau không.
::::

::::explain{#dem-phan-bo}
`demPhanBo` gọi `timNodeChiuTrachNhiem` (bài 4) CHO từng khoá, đếm
SỐ khoá mỗi node nhận ĐƯỢC:

```typescript title=readonly
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}

interface DiemNode { viTri: number; ten: string; }

function xayVong(tenCacNode: string[]): DiemNode[] {
  const vong: DiemNode[] = tenCacNode.map((ten) => ({ viTri: bam(ten), ten }));
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

function timNodeChiuTrachNhiem(vong: DiemNode[], khoa: string): string {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.ten;
  }
  return vong[0]!.ten;
}

function demPhanBo(vong: DiemNode[], cacKhoa: string[]): Record<string, number> {
  const dem: Record<string, number> = {};
  for (const khoa of cacKhoa) {
    const ten = timNodeChiuTrachNhiem(vong, khoa);
    dem[ten] = (dem[ten] ?? 0) + 1;
  }
  return dem;
}

const vong = xayVong(["alpha", "beta", "gamma", "delta"]);
const cacKhoa: string[] = [];
for (let i = 0; i < 100; i++) cacKhoa.push("nguoidung" + i);
console.log(demPhanBo(vong, cacKhoa));
```

```text title=readonly
{ alpha: 35, beta: 16, gamma: 13, delta: 36 }
```

TRÊN `100` khoá, `delta` VÀ `alpha` nhận LẦN lượt `36` VÀ `35` khoá
— trong khi `gamma` chỉ nhận `13`. `delta` xử LÝ nhiều hơn GẤP `2.7`
lần `gamma`, dù `bam` (bài 6) LÀ một hàm băm TỐT, rải khoá đủ đều
TRÊN vòng.
::::

::::example{#khong-phai-loi-ham-bam}
Vấn đề KHÔNG nằm Ở `bam` — nó nằm Ở VỊ trí của BỐN node TRÊN vòng.
`bam("alpha")=785`, `bam("beta")=375`, `bam("gamma")=210`,
`bam("delta")=66` — bốn vị trí NÀY chia vòng `[0,999]` (`1000` đơn
vị) thành BỐN đoạn CÓ độ dài rất khác NHAU: đoạn của `alpha` (TỪ
`beta` tới chính nó) dài `785-375=410` đơn vị, trong khi đoạn của
`gamma` (từ `beta` LÙI về, tức TỪ `delta` tới `gamma`) chỉ dài
`210-66=144` đơn vị. VỊ trí của node hoàn TOÀN phụ thuộc VÀO `bam
(tên node)` — VÀ với CHỈ bốn tên, không CÓ gì đảm bảo bốn vị TRÍ đó
cách đều nhau TRÊN một vòng `1000` đơn vị.
::::

::::predict{#doan-them-node-thu-nam commitOnce}
THÊM `epsilon` (`bam("epsilon")=486`) VÀO bốn node TRÊN — vòng giờ
CÓ năm điểm. Việc thêm MỘT node NGẪU nhiên nữa CÓ chắc chắn làm phân
bố tải ĐỀU hơn không?

:::opt{correct}
KHÔNG chắc — vị trí CỦA `epsilon` (`486`) chỉ CHIA nhỏ đoạn `alpha`
đang giữ (TỪ `beta` Ở `375` tới `alpha` Ở `785`), KHÔNG động tới
đoạn NHỎ của `gamma` — mất CÂN bằng vẫn CÒN đó, chỉ chuyển bớt một
PHẦN tải từ `alpha` sang `epsilon`
:::

:::opt
CÓ, LUÔN đều hơn — CÀNG nhiều node trên vòng THÌ phân bố CÀNG tiệm
cận đều, bất kể vị trí TỪNG node LÀ bao nhiêu
::why
Gần đúng ở việc bạn nghĩ TỚI "nhiều điểm HƠN → đều hơn" như một xu
HƯỚNG chung — Ở quy MÔ rất lớn (HÀNG nghìn điểm), điều NÀY đúng theo
nghĩa THỐNG kê.

Chỗ lệch: VỚI một vài node cụ THỂ, vị trí của node MỚI hoàn toàn
NGẪU nhiên (phụ thuộc TÊN của nó qua `bam`) — nó CÓ thể rơi ĐÚNG vào
đoạn ĐÃ nhỏ (giúp CHIA nhỏ thêm, càng LỆCH), hoặc đoạn đang LỚN
(giúp cân bằng hơn MỘT chút, như CA `epsilon` Ở trên — chia bớt tải
của `alpha`). KHÔNG có gì đảm bảo HƯỚNG nào xảy ra — thêm ĐÚNG một
node không phải một CÁCH đáng tin CẬY để sửa mất cân bằng.
::
:::
::::

::::code{#viet_dem_phan_bo}
Hoàn thiện `demPhanBo` — VỚI mỗi khoá, tìm node chịu trách NHIỆM rồi
tăng bộ ĐẾM của node đó.

```typescript title=starter
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}

interface DiemNode { viTri: number; ten: string; }

function xayVong(tenCacNode: string[]): DiemNode[] {
  const vong: DiemNode[] = tenCacNode.map((ten) => ({ viTri: bam(ten), ten }));
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

function timNodeChiuTrachNhiem(vong: DiemNode[], khoa: string): string {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.ten;
  }
  return vong[0]!.ten;
}

function demPhanBo(vong: DiemNode[], cacKhoa: string[]): Record<string, number> {
  const dem: Record<string, number> = {};
  for (const khoa of cacKhoa) {
    const ten = timNodeChiuTrachNhiem(vong, khoa);
    ___
  }
  return dem;
}

const vong = xayVong(["alpha", "beta", "gamma", "delta"]);
const cacKhoa: string[] = [];
for (let i = 0; i < 100; i++) cacKhoa.push("nguoidung" + i);
console.log(demPhanBo(vong, cacKhoa));
```

```typescript title=solution
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}

interface DiemNode { viTri: number; ten: string; }

function xayVong(tenCacNode: string[]): DiemNode[] {
  const vong: DiemNode[] = tenCacNode.map((ten) => ({ viTri: bam(ten), ten }));
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

function timNodeChiuTrachNhiem(vong: DiemNode[], khoa: string): string {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.ten;
  }
  return vong[0]!.ten;
}

function demPhanBo(vong: DiemNode[], cacKhoa: string[]): Record<string, number> {
  const dem: Record<string, number> = {};
  for (const khoa of cacKhoa) {
    const ten = timNodeChiuTrachNhiem(vong, khoa);
    dem[ten] = (dem[ten] ?? 0) + 1;
  }
  return dem;
}

const vong = xayVong(["alpha", "beta", "gamma", "delta"]);
const cacKhoa: string[] = [];
for (let i = 0; i < 100; i++) cacKhoa.push("nguoidung" + i);
console.log(demPhanBo(vong, cacKhoa));
```

```typescript title=test
const vong2 = xayVong(["alpha", "beta", "gamma", "delta"]);
const cacKhoa2: string[] = [];
for (let i = 0; i < 100; i++) cacKhoa2.push("nguoidung" + i);
const ketQua = demPhanBo(vong2, cacKhoa2);
console.log(ketQua);
if (ketQua["alpha"] !== 35) throw new Error("alpha phai nhan dung 35 khoa");
if (ketQua["beta"] !== 16) throw new Error("beta phai nhan dung 16 khoa");
if (ketQua["gamma"] !== 13) throw new Error("gamma phai nhan dung 13 khoa");
if (ketQua["delta"] !== 36) throw new Error("delta phai nhan dung 36 khoa");
const tong = (ketQua["alpha"] ?? 0) + (ketQua["beta"] ?? 0) + (ketQua["gamma"] ?? 0) + (ketQua["delta"] ?? 0);
if (tong !== 100) throw new Error("tong so khoa duoc dem phai dung bang 100 (khong khoa nao bi dem sot hay dem lap)");
```

:::hints
- kind: attention
  body: "Tang bo dem CUA node vua tim duoc len 1 (dung ?? 0 neu chua co) -- mot dong."
- kind: strategy
  body: "dem[ten] = (dem[ten] ?? 0) + 1;"
- kind: one-line
  body: "dem[ten] = (dem[ten] ?? 0) + 1;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "alpha"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một điểm mỗi node CHƯA đủ để cân bằng tải. Cho MỖI node NHIỀU điểm
trên vòng — trông ra sao?
::::

::::reflect{#nghi-lai}
`demPhanBo` cho THẤY một sự THẬT quan trọng: "hàm băm TỐT" (bài 6)
VÀ "phân bố tải ĐỀU" (bài NÀY) LÀ hai câu hỏi khác NHAU. Hàm băm
tốt đảm bảo KHOÁ rải đều TRÊN vòng — nhưng NẾU chỉ CÓ vài điểm ĐẠI
diện cho node (Ở đây: mỗi node ĐÚNG một điểm), phân bố tải VẪN phụ
thuộc VÀO việc vài điểm ĐÓ tình cờ rơi Ở đâu, VÀ với số điểm ÍT thì
"tình CỜ" một mình LÀ đủ để tạo ra CHÊNH lệch lớn. Cách sửa TRỰC
tiếp: cho mỗi node KHÔNG chỉ một, mà NHIỀU điểm đại diện.
::::

::::checkpoint{mastery=0.8}
::::
