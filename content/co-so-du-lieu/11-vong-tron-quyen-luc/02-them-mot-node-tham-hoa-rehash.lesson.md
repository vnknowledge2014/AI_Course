---
id: co-so-du-lieu.vong-tron-quyen-luc.them-mot-node-tham-hoa-rehash
title: "Thêm một node — thảm hoạ rehash"
summary: "demSoKhoaDoiNode đếm số khoá đổi node khi soLuongNode thay đổi. Với 12 khoá mẫu, đi từ 3 lên 4 node khiến 9/12 khoá (75%) đổi node — dù chỉ THÊM đúng một máy. Nguyên nhân: bam(khoa) % soLuongNode phụ thuộc TOÀN BỘ vào giá trị soLuongNode, đổi mẫu số gần như luôn đổi phần dư."
locale: vi
track: co-so-du-lieu
module: vong-tron-quyen-luc
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.tham-hoa-rehash]
requires: [db.bam-roi-chia-ngay-tho]
concepts: [db.tham-hoa-rehash]
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
Cụm CÓ 3 máy, giờ THÊM một máy THỨ tư — chỉ MỘT thay đổi nhỏ. Bao
nhiêu khoá phải DI chuyển sang máy khác?
::::

::::explain{#dem-so-khoa-doi}
`demSoKhoaDoiNode` so sánh node CỦA mỗi khoá TRƯỚC VÀ sau khi
`soLuongNode` đổi, đếm số LẦN kết quả khác NHAU:

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

function chonNodeNgayTho(khoa: string, soLuongNode: number): number {
  return bam(khoa) % soLuongNode;
}

function demSoKhoaDoiNode(cacKhoa: string[], soNodeTruoc: number, soNodeSau: number): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (chonNodeNgayTho(khoa, soNodeTruoc) !== chonNodeNgayTho(khoa, soNodeSau)) {
      dem = dem + 1;
    }
  }
  return dem;
}

const cacKhoa: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa.push("nguoidung" + i);
console.log(demSoKhoaDoiNode(cacKhoa, 3, 4));
```

```text title=readonly
9
```

`9` TRÊN `12` khoá — `75%` — đổi NODE, chỉ VÌ số lượng node đi TỪ
`3` lên `4`. KHÔNG máy nào BỊ hỏng, KHÔNG khoá nào thay đổi GIÁ trị
— nhưng gần như MỌI khoá phải DI chuyển dữ liệu SANG một máy khác.
::::

::::example{#vi-sao-doi-nhieu-vay}
`chonNodeNgayTho(khoa, soLuongNode) = bam(khoa) % soLuongNode` —
KẾT quả phụ thuộc TOÀN bộ vào `soLuongNode`, KHÔNG chỉ một PHẦN nhỏ
của nó. Đổi `soLuongNode` TỪ `3` sang `4` nghĩa LÀ hầu hết phép chia
LẤY dư trước đó KHÔNG còn Ý nghĩa gì nữa — `bam("nguoidung3")=474`;
`474 % 3 = 0`, NHƯNG `474 % 4 = 2`: HAI phép chia lấy dư TRÊN CÙNG
một số bị chia hiếm khi cho CÙNG kết quả. Đây LÀ vấn đề CỐT lõi của
"băm rồi chia": KHÔNG có cách nào THÊM/bớt một node MÀ chỉ ảnh hưởng
tới một PHẦN nhỏ dữ liệu.
::::

::::predict{#doan-bot-node commitOnce}
Cụm CÓ `4` node, BỚT xuống còn `3` (một MÁY hỏng, phải rút KHỎI
cụm). SO với việc THÊM node (`3→4`, vừa thấy Ở trên), việc BỚT node
(`4→3`) CÓ khiến ÍT khoá di chuyển hơn KHÔNG?

:::opt{correct}
KHÔNG — `demSoKhoaDoiNode(cacKhoa, 4, 3)` VẪN đổi phần lớn khoá,
cùng LÝ do: `% 4` VÀ `% 3` LÀ hai phép chia gần như KHÔNG liên quan
:::

:::opt
CÓ, ÍT hơn — bớt node LÀ thao tác "NGƯỢC" của thêm node, nên số khoá
di CHUYỂN cũng phải Ở mức tương TỰ nhưng ngược HƯỚNG, tức LÀ nhỏ hơn
::why
Gần đúng ở việc bạn NGHĨ "ngược thao tác" nên "ngược MỨC độ ảnh
hưởng" — MỘT trực giác dễ hiểu nếu coi thêm/bớt LÀ hai phép toán đối
xứng NHAU.

Chỗ lệch: MỨC độ ảnh hưởng của `% soLuongNode` KHÔNG phụ thuộc "thêm
hay bớt", nó phụ thuộc CHÊNH lệch GIỮA hai phép chia LẤY dư — VÀ hai
phép CHIA khác nhau (`% 4` so VỚI `% 3`) LUÔN cho kết quả gần như
KHÔNG liên quan tới nhau, bất kể ĐI từ số NÀO sang số NÀO. Bớt node
`4→3` gây XÁO trộn tương ĐƯƠNG thêm node `3→4` — KHÔNG hề nhẹ hơn.
::
:::
::::

::::code{#viet_dem_so_khoa_doi_node}
Hoàn thiện `demSoKhoaDoiNode` — đếm số khoá CÓ node TRƯỚC khác node
SAU.

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

function chonNodeNgayTho(khoa: string, soLuongNode: number): number {
  return bam(khoa) % soLuongNode;
}

function demSoKhoaDoiNode(cacKhoa: string[], soNodeTruoc: number, soNodeSau: number): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (___) {
      dem = dem + 1;
    }
  }
  return dem;
}

const cacKhoa: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa.push("nguoidung" + i);
console.log(demSoKhoaDoiNode(cacKhoa, 3, 4));
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

function chonNodeNgayTho(khoa: string, soLuongNode: number): number {
  return bam(khoa) % soLuongNode;
}

function demSoKhoaDoiNode(cacKhoa: string[], soNodeTruoc: number, soNodeSau: number): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (chonNodeNgayTho(khoa, soNodeTruoc) !== chonNodeNgayTho(khoa, soNodeSau)) {
      dem = dem + 1;
    }
  }
  return dem;
}

const cacKhoa: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa.push("nguoidung" + i);
console.log(demSoKhoaDoiNode(cacKhoa, 3, 4));
```

```typescript title=test
const cacKhoa2: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa2.push("nguoidung" + i);
console.log(demSoKhoaDoiNode(cacKhoa2, 3, 4));
if (demSoKhoaDoiNode(cacKhoa2, 3, 4) !== 9) throw new Error("3 len 4 node phai co dung 9/12 khoa doi node");
if (demSoKhoaDoiNode(cacKhoa2, 3, 3) !== 0) throw new Error("so node KHONG doi thi khong khoa nao duoc doi node");
if (demSoKhoaDoiNode([], 3, 4) !== 0) throw new Error("danh sach khoa rong thi dem phai la 0");
```

:::hints
- kind: attention
  body: "Dieu kien dem: node CUA khoa Ở soNodeTruoc KHAC node CUA no Ở soNodeSau -- mot dong."
- kind: strategy
  body: "chonNodeNgayTho(khoa, soNodeTruoc) !== chonNodeNgayTho(khoa, soNodeSau)"
- kind: one-line
  body: "if (chonNodeNgayTho(khoa, soNodeTruoc) !== chonNodeNgayTho(khoa, soNodeSau)) {"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "9"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
75% khoá di chuyển chỉ vì THÊM một máy — không chấp nhận được cho một
hệ THẬT. Cách khác — đặt CẢ node lẫn khoá lên một VÒNG tròn — trông
ra sao?
::::

::::reflect{#nghi-lai}
"Băm RỒI chia" (bài trước) đơn giản NHƯNG dễ vỡ đúng Ở chỗ hệ phân
tán CẦN nhất: khả năng THÊM/bớt máy MÀ không phải di chuyển LẠI gần
như toàn bộ dữ liệu. Vấn đề KHÔNG nằm Ở `bam` (hàm băm VẪN tất định,
VẪN rải đều) — nó nằm Ở CÁCH DÙNG kết quả băm: `% soLuongNode` LÀM
cho TOÀN bộ ánh xạ phụ thuộc và MỘT con số CÓ thể đổi bất kỳ LÚC nào.
Cần một cách ánh XẠ khoá→node ÍT nhạy CẢM hơn VỚI thay đổi CỦA
`soLuongNode`.
::::

::::checkpoint{mastery=0.8}
::::
