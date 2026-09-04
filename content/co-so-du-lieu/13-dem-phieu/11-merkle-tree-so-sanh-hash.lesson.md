---
id: co-so-du-lieu.dem-phieu.merkle-tree-so-sanh-hash
title: "Merkle tree — so khác biệt bằng hash"
summary: "xayCayMerkle chia khoá đã sắp xếp thành soNhom nhóm (theo bam(k) % soNhom), hash NỘI DUNG mỗi nhóm thành MỘT 'lá', rồi hash toàn bộ các lá thành MỘT 'gốc'. Hai bản sao khác nhau 3 khoá (trên 1000, chia 20 nhóm): hashGoc khác NGAY LẬP TỨC (một phép so sánh), và chỉ CẦN so 20 hash lá để biết ĐÚNG 3 nhóm nào lệch (nhóm 2, 6, 17) -- không cần chạm tới 997 khoá còn lại đang thực sự giống nhau."
locale: vi
track: co-so-du-lieu
module: dem-phieu
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.merkle-tree-so-sanh-hash]
requires: [db.so-sanh-toan-bo-la-qua-dat]
concepts: [db.merkle-tree-so-sanh-hash]
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
So từng khoá LÀ `O(n)` (bài trước) — đắt DÙ chỉ vài khoá lệch. Ý
tưởng: đóng GÓI từng NHÓM khoá thành MỘT con số duy nhất, so CON số
đó trước ĐÃ.
::::

::::explain{#xay-cay-merkle}
`xayCayMerkle` chia khoá ĐÃ sắp xếp thành `soNhom` nhóm (dùng LẠI
`bam` để chọn NHÓM: `bam(k) % soNhom`), hash NỘI dung mỗi nhóm thành
MỘT "lá" (`hashLa`), rồi hash TOÀN bộ các lá thành MỘT "gốc"
(`hashGoc`). So SÁNH hai cây bắt đầu Ở GỐC:

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

function bamLa(kho: Map<string, number>, cacKhoaTrongNhom: string[]): number {
  let noiDung = "";
  for (const k of cacKhoaTrongNhom) noiDung += k + ":" + (kho.get(k) ?? "") + ";";
  return bam(noiDung);
}

interface CayMerkle { hashGoc: number; hashLa: number[]; nhomKhoa: string[][]; }

function xayCayMerkle(kho: Map<string, number>, soNhom: number): CayMerkle {
  const cacKhoa = [...kho.keys()].sort();
  const nhomKhoa: string[][] = [];
  for (let i = 0; i < soNhom; i++) nhomKhoa.push([]);
  for (const k of cacKhoa) {
    nhomKhoa[bam(k) % soNhom]!.push(k);
  }
  const hashLa = nhomKhoa.map((nhom) => bamLa(kho, nhom));
  const hashGoc = bam(hashLa.join(","));
  return { hashGoc, hashLa, nhomKhoa };
}

function timNhomKhacBiet(cayA: CayMerkle, cayB: CayMerkle): number[] {
  const ketQua: number[] = [];
  for (let i = 0; i < cayA.hashLa.length; i++) {
    if (cayA.hashLa[i] !== cayB.hashLa[i]) ketQua.push(i);
  }
  return ketQua;
}

const khoA = new Map<string, number>();
const khoB = new Map<string, number>();
for (let i = 0; i < 1000; i++) { khoA.set("khoa" + i, i); khoB.set("khoa" + i, i); }
khoB.set("khoa5", 9999);
khoB.set("khoa500", 8888);
khoB.delete("khoa999");

const cayA = xayCayMerkle(khoA, 20);
const cayB = xayCayMerkle(khoB, 20);
console.log("hashGoc khac nhau:", cayA.hashGoc !== cayB.hashGoc);
console.log("nhom lech:", timNhomKhacBiet(cayA, cayB));
```

```text title=readonly
hashGoc khac nhau: true
nhom lech: [ 2, 6, 17 ]
```

Một phép SO sánh (`hashGoc`) đủ để biết HAI bản sao khác NHAU. Sau
đó, chỉ CẦN so `20` hash LÁ (không phải `1000` khoá) để biết CHÍNH
xác `3` nhóm nào chứa khoá lệch (`2, 6, 17`) — mỗi nhóm trung bình
`1000/20 = 50` khoá, nghĩa LÀ chỉ cần tải lại khoảng `150` khoá
(TRONG ba nhóm đó) thay vì so sánh cả `1000`.
::::

::::example{#hai-tang-khong-can-thiet}
`timNhomKhacBiet` chỉ so `20` con số — KHÔNG hề đụng tới `nhomKhoa`
(danh sách khoá THẬT bên trong) trừ khi cần TÌM chính xác khoá nào
lệch TRONG một nhóm đã biết LÀ khác. Đây LÀ ý tưởng cốt lõi của cây
Merkle: một cây THẬT (nhiều tầng) có THỂ thu hẹp phạm vi tiếp, nhưng
ngay CẢ "cây một tầng" (gốc + lá phẳng) như bài NÀY cũng đã giảm chi
phí SO sánh từ `O(n)` khoá xuống `O(soNhom)` hash — VỚI `soNhom` LÀ
một con số CỐ định, nhỏ hơn `n` rất nhiều.
::::

::::predict{#doan-hai-kho-giong-het-merkle commitOnce}
Hai bản sao GIỐNG HỆT nhau hoàn TOÀN (`1000` khoá, `20` nhóm). So
với bài 10 (`soKhoaLechBangSoSanhTung` LUÔN duyệt đủ `1000` khoá dù
giống hệt), `xayCayMerkle` + so `hashGoc` CÓ rẻ hơn KHÔNG?

:::opt{correct}
CÓ — chỉ CẦN xây hai cây (mỗi cây vẫn phải duyệt `1000` khoá MỘT
lần để XÂY, không tránh được) rồi so ĐÚNG một cặp `hashGoc`; NHƯNG
nếu hai cây đã được xây SẴN từ trước (VÍ dụ lưu cache SAU mỗi lần
ghi), việc so sánh ĐỊNH kỳ sau đó chỉ LÀ một phép so sánh SỐ, không
cần xây lại
:::

:::opt
KHÔNG — xây cây VẪN phải duyệt hết `1000` khoá (Ở `xayCayMerkle`),
nên tổng chi phí không hề khác gì `soKhoaLechBangSoSanhTung`
::why
Đúng Ở MỘT nửa: XÂY cây lần đầu THẬT sự vẫn tốn `O(n)` (duyệt hết
khoá để chia NHÓM và hash). Đây LÀ quan sát chính XÁC.

Chỗ lệch: cái LỢI của Merkle tree không nằm Ở lần XÂY đầu tiên — mà
Ở việc `hashGoc`/`hashLa` CÓ thể được TÍNH một lần rồi lưu LẠI (VÍ
dụ cập nhật GIA tăng mỗi khi ghi), và những lần SO sánh định kỳ SAU
đó (anti-entropy chạy MỖI vài phút) chỉ cần so CÁC hash đã có SẴN —
không cần xây LẠI hay duyệt lại `1000` khoá MỖI lần so. So VỚI việc
GỌI `soKhoaLechBangSoSanhTung` mỗi lần (LUÔN `O(n)`), Merkle tree
biến phần lớn các lần SO sánh định kỳ thành gần NHƯ miễn phí.
::
:::
::::

::::code{#viet_xay_cay_merkle}
Hoàn thiện `xayCayMerkle` — VỚI mỗi khoá, xếp nó vào đúng nhóm theo
`bam(k) % soNhom`.

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

function bamLa(kho: Map<string, number>, cacKhoaTrongNhom: string[]): number {
  let noiDung = "";
  for (const k of cacKhoaTrongNhom) noiDung += k + ":" + (kho.get(k) ?? "") + ";";
  return bam(noiDung);
}

interface CayMerkle { hashGoc: number; hashLa: number[]; nhomKhoa: string[][]; }

function xayCayMerkle(kho: Map<string, number>, soNhom: number): CayMerkle {
  const cacKhoa = [...kho.keys()].sort();
  const nhomKhoa: string[][] = [];
  for (let i = 0; i < soNhom; i++) nhomKhoa.push([]);
  for (const k of cacKhoa) {
    ___
  }
  const hashLa = nhomKhoa.map((nhom) => bamLa(kho, nhom));
  const hashGoc = bam(hashLa.join(","));
  return { hashGoc, hashLa, nhomKhoa };
}

function timNhomKhacBiet(cayA: CayMerkle, cayB: CayMerkle): number[] {
  const ketQua: number[] = [];
  for (let i = 0; i < cayA.hashLa.length; i++) {
    if (cayA.hashLa[i] !== cayB.hashLa[i]) ketQua.push(i);
  }
  return ketQua;
}

const khoA = new Map<string, number>();
for (let i = 0; i < 1000; i++) khoA.set("khoa" + i, i);
console.log(xayCayMerkle(khoA, 20).hashGoc);
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

function bamLa(kho: Map<string, number>, cacKhoaTrongNhom: string[]): number {
  let noiDung = "";
  for (const k of cacKhoaTrongNhom) noiDung += k + ":" + (kho.get(k) ?? "") + ";";
  return bam(noiDung);
}

interface CayMerkle { hashGoc: number; hashLa: number[]; nhomKhoa: string[][]; }

function xayCayMerkle(kho: Map<string, number>, soNhom: number): CayMerkle {
  const cacKhoa = [...kho.keys()].sort();
  const nhomKhoa: string[][] = [];
  for (let i = 0; i < soNhom; i++) nhomKhoa.push([]);
  for (const k of cacKhoa) {
    nhomKhoa[bam(k) % soNhom]!.push(k);
  }
  const hashLa = nhomKhoa.map((nhom) => bamLa(kho, nhom));
  const hashGoc = bam(hashLa.join(","));
  return { hashGoc, hashLa, nhomKhoa };
}

function timNhomKhacBiet(cayA: CayMerkle, cayB: CayMerkle): number[] {
  const ketQua: number[] = [];
  for (let i = 0; i < cayA.hashLa.length; i++) {
    if (cayA.hashLa[i] !== cayB.hashLa[i]) ketQua.push(i);
  }
  return ketQua;
}

const khoA = new Map<string, number>();
for (let i = 0; i < 1000; i++) khoA.set("khoa" + i, i);
console.log(xayCayMerkle(khoA, 20).hashGoc);
```

```typescript title=test
const khoA2 = new Map<string, number>();
const khoB2 = new Map<string, number>();
for (let i = 0; i < 1000; i++) { khoA2.set("khoa" + i, i); khoB2.set("khoa" + i, i); }
khoB2.set("khoa5", 9999);
khoB2.set("khoa500", 8888);
khoB2.delete("khoa999");
const cayA = xayCayMerkle(khoA2, 20);
const cayB = xayCayMerkle(khoB2, 20);

if (cayA.hashGoc === cayB.hashGoc) throw new Error("hai kho khac nhau (3 khoa lech) phai co hashGoc khac nhau");
const nhomLech = timNhomKhacBiet(cayA, cayB);
if (nhomLech.length !== 3) throw new Error("phai co dung 3 nhom la lech (ung voi khoa5, khoa500, khoa999)");
if (JSON.stringify(nhomLech) !== JSON.stringify([2, 6, 17])) throw new Error("chi so 3 nhom lech phai la [2, 6, 17]");

let tongSoKhoaTrongNhom = 0;
for (const nhom of cayA.nhomKhoa) tongSoKhoaTrongNhom += nhom.length;
if (tongSoKhoaTrongNhom !== 1000) throw new Error("tong so khoa trong tat ca nhom phai bang dung 1000 (khong mat, khong trung khoa nao)");

const khoGiongHet = new Map(khoA2);
const cayGiongHet = xayCayMerkle(khoGiongHet, 20);
if (cayA.hashGoc !== cayGiongHet.hashGoc) throw new Error("hai kho giong het nhau phai co hashGoc BANG nhau");
if (timNhomKhacBiet(cayA, cayGiongHet).length !== 0) throw new Error("hai kho giong het nhau thi 0 nhom la nao lech");
```

:::hints
- kind: attention
  body: "Chon nhom cho k bang bam(k) % soNhom, roi day k vao dung nhom do -- mot dong."
- kind: strategy
  body: "nhomKhoa[bam(k) % soNhom]!.push(k);"
- kind: one-line
  body: "nhomKhoa[bam(k) % soNhom]!.push(k);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "632"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Preference list, quorum, hint, read repair, Merkle — mọi mảnh ghép
ĐÃ có. Ráp tất CẢ thành một hệ đếm phiếu HOÀN chỉnh trông ra SAO?
::::

::::reflect{#nghi-lai}
`xayCayMerkle` dùng LẠI đúng hàm `bam` đã xuất hiện xuyên suốt track
NÀY (q11-q13) — chỉ đổi CÁCH dùng: thay VÌ chọn NODE cho một khoá,
giờ dùng để chọn NHÓM cho khoá đó. Cây Merkle KHÔNG thay thế read
repair (bài 9) — hai cơ chế bổ SUNG cho nhau: read repair sửa NHANH
khi có AI đọc, Merkle quét ĐỊNH kỳ để bắt phần KHÔNG ai đọc tới.
::::

::::checkpoint{mastery=0.85}
::::
