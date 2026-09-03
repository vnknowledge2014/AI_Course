---
id: co-so-du-lieu.vong-tron-quyen-luc.bam-roi-chia-cach-ngay-tho
title: "Băm rồi chia — cách ngây thơ"
summary: "chonNodeNgayTho(khoa, soLuongNode) = bam(khoa) % soLuongNode — cách ĐƠN giản nhất để rải dữ liệu ra nhiều máy (node) trong hệ phân tán. Với SỐ node CỐ định, cách này hoạt động HOÀN hảo: mỗi khoá LUÔN rơi vào ĐÚNG một node, tất định (deterministic), không cần một BẢNG tra cứu nào lưu trước."
locale: vi
track: co-so-du-lieu
module: vong-tron-quyen-luc
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [db.bam-roi-chia-ngay-tho]
requires: [db.doi-chieu-pipeline]
concepts: [db.bam-roi-chia-ngay-tho]
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
q10 chạy SurrealDB TRÊN một máy DUY nhất. Một hệ THẬT (ScyllaDB,
Cassandra) rải dữ liệu RA nhiều máy — MỘT khoá đi tới ĐÚNG máy nào?
::::

::::explain{#bam-roi-chia}
Cách ĐƠN giản nhất: băm khoá thành một SỐ, rồi lấy phần dư CHIA cho
số lượng node — `chonNodeNgayTho(khoa, soLuongNode) = bam(khoa) %
soLuongNode`:

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

console.log(chonNodeNgayTho("nguoidung0", 3));
console.log(chonNodeNgayTho("nguoidung1", 3));
console.log(chonNodeNgayTho("nguoidung2", 3));
console.log(chonNodeNgayTho("nguoidung3", 3));
```

```text title=readonly
1
0
2
0
```

`bam` biến MỘT chuỗi thành một số nguyên trong `[0, 999]` — KHÔNG
quan trọng công thức CHÍNH xác thế nào, điều kiện DUY nhất đòi hỏi
LÀ: CÙNG một chuỗi LUÔN cho CÙNG một số (tất định). `% soLuongNode`
rồi rút gọn số ĐÓ về đúng phạm vi `[0, soLuongNode)` — MỘT chỉ số
node hợp lệ.
::::

::::example{#tat-dinh-khong-can-bang-tra}
Với SỐ node CỐ định, `chonNodeNgayTho` KHÔNG cần lưu một bảng "khoá
NÀO ở node NÀO" Ở đâu cả — gọi LẠI hàm là đủ, LUÔN ra cùng kết quả:

```typescript title=readonly
console.log(chonNodeNgayTho("nguoidung0", 3));
console.log(chonNodeNgayTho("nguoidung0", 3));
```

```text title=readonly
1
1
```

Đây LÀ điểm MẠNH của "băm rồi chia": không CÓ trạng thái NÀO phải
đồng bộ GIỮA các máy — MỖI máy tự tính `bam(khoa) % soLuongNode` VÀ
biết NGAY khoá đó thuộc VỀ ai, miễn LÀ mọi máy đều biết `soLuongNode`
LÀ bao nhiêu.
::::

::::predict{#doan-doi-khoa commitOnce}
`chonNodeNgayTho("nguoidung1", 3)` VÀ `chonNodeNgayTho("nguoidung3",
3)` — HAI khoá KHÁC nhau. Chúng CÓ chắc chắn rơi vào HAI node khác
nhau không?

:::opt{correct}
KHÔNG chắc — hai khoá KHÁC nhau vẫn CÓ thể rơi vào CÙNG một node
(`% soLuongNode` chỉ CÓ `soLuongNode` giá trị khả dĩ)
:::

:::opt
CÓ, LUÔN khác nhau — `bam` LÀ một hàm băm TỐT, hai đầu VÀO khác nhau
PHẢI cho ra hai kết quả khác NHAU
::why
Gần đúng ở việc bạn tin TƯỞNG `bam` LÀ một hàm băm "TỐT" — MỘT hàm
băm tốt ĐÚNG là hiếm khi cho hai đầu VÀO khác nhau CÙNG một giá trị
băm (Ở phạm vi `[0,999]`).

Chỗ lệch: NGAY cả khi `bam` không hề đụng ĐỘ (hai giá trị băm khác
nhau), bước `% soLuongNode` SAU đó có thể VẪN khiến hai số dư TRÙNG
nhau — VÍ dụ `bam` cho `262` VÀ `474`, cả hai `% 3` đều LÀ `0`.
`soLuongNode` càng NHỎ, khả năng trùng CÀNG cao — đây LÀ điều
BÌNH thường, chính LÀ mục đích của phép chia LẤY dư: RÚT nhiều giá
trị băm VỀ đúng `soLuongNode` NHÓM.
::
:::
::::

::::code{#viet_chon_node_ngay_tho}
Hoàn thiện `chonNodeNgayTho` — băm khoá RỒI lấy phần DƯ chia cho
`soLuongNode`.

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
  return ___;
}

console.log(chonNodeNgayTho("nguoidung0", 3));
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

console.log(chonNodeNgayTho("nguoidung0", 3));
```

```typescript title=test
console.log(chonNodeNgayTho("nguoidung0", 3));
if (chonNodeNgayTho("nguoidung0", 3) !== 1) throw new Error("nguoidung0 voi 3 node phai ra node 1");
if (chonNodeNgayTho("nguoidung1", 3) !== 0) throw new Error("nguoidung1 voi 3 node phai ra node 0");
if (chonNodeNgayTho("nguoidung2", 3) !== 2) throw new Error("nguoidung2 voi 3 node phai ra node 2");

if (chonNodeNgayTho("bat_ky_khoa_nao", 1) !== 0) throw new Error("chi co 1 node thi MOI khoa phai vao node 0");

const lan1 = chonNodeNgayTho("nguoidung5", 7);
const lan2 = chonNodeNgayTho("nguoidung5", 7);
if (lan1 !== lan2) throw new Error("cung khoa, cung so node phai LUON ra cung mot ket qua (tat dinh)");
if (lan1 < 0 || lan1 >= 7) throw new Error("ket qua phai nam trong [0, soLuongNode)");
```

:::hints
- kind: attention
  body: "Bam khoa thanh so, roi lay phan du chia cho soLuongNode -- mot dong."
- kind: strategy
  body: "bam(khoa) % soLuongNode"
- kind: one-line
  body: "return bam(khoa) % soLuongNode;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Với số node CỐ định, "băm rồi chia" hoạt động HOÀN hảo. Nhưng số
node HIẾM khi cố định MÃI — thêm MỘT máy vào cụm thì SAO?
::::

::::reflect{#nghi-lai}
`chonNodeNgayTho` LÀ lựa chọn ĐƠN giản, tất định, KHÔNG cần trạng
thái chia SẺ — đúng những gì một hệ phân TÁN muốn Ở lớp định TUYẾN.
Cái GIÁ ẩn của sự đơn giản NÀY nằm Ở đâu, khi `soLuongNode` chính nó
THAY đổi (thêm HOẶC bớt một máy)?
::::

::::checkpoint{mastery=0.75}
::::
