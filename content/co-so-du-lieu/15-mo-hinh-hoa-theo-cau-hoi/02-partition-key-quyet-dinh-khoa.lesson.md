---
id: co-so-du-lieu.mo-hinh-hoa-theo-cau-hoi.partition-key-quyet-dinh-khoa
title: "Partition key quyết định khoá"
summary: "chonPartitionChoKhoa(khoa, soPartition) = bam(khoa) % soPartition — cùng công thức chonNodeNgayTho của q11, đổi tên: 'partition key' là phần khoá dùng để BĂM và định vị một hàng. Hai hàng CÙNG partition key ('phong5') LUÔN rơi vào ĐÚNG một partition, mọi lần gọi — đây chính là điều làm bảng riêng theo phòng (bài 1) hoạt động: mọi tin nhắn của một phòng co cụm lại một chỗ, không tản mát."
locale: vi
track: co-so-du-lieu
module: mo-hinh-hoa-theo-cau-hoi
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.partition-key-quyet-dinh-khoa]
requires: [db.mot-bang-cho-moi-cau-hoi]
concepts: [db.partition-key-quyet-dinh-khoa]
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
`xayBangTheoPhong` (bài trước) dùng `Map` — một máy DUY nhất tra được
NGAY. Trên MỘT hệ phân tán thật (nhiều máy), "phòng nào NẰM ở đâu"
được quyết định BỞI phần nào của khoá?
::::

::::explain{#partition-key}
`chonPartitionChoKhoa` — ĐÚNG công thức `chonNodeNgayTho` (q11 bài 1),
chỉ đổi TÊN: "partition key" LÀ phần dữ liệu dùng để BĂM và định vị
một hàng và tổ chức "phân vùng" (partition) NÀO nó thuộc về:

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

function chonPartitionChoKhoa(khoa: string, soPartition: number): number {
  return bam(khoa) % soPartition;
}

console.log("phong5, 8 partition:", chonPartitionChoKhoa("phong5", 8));
console.log("phong5 lan nua (giong het):", chonPartitionChoKhoa("phong5", 8));
console.log("phong6, 8 partition:", chonPartitionChoKhoa("phong6", 8));
```

```text title=readonly
phong5, 8 partition: 2
phong5 lan nua (giong het): 2
phong6, 8 partition: 3
```

`"phong5"` LUÔN rơi vào partition `2` — GỌI bao nhiêu lần, kết quả
VẪN `2` (tất định, y hệt hashing Ở q11). `"phong6"` (khác khoá) rơi
vào partition `3` — KHÁC hẳn. Đây LÀ lý do bảng theo phòng (bài 1)
hoạt động: MỌI tin nhắn CÓ cùng `phongId` sẽ LUÔN được định vị VÀO
đúng CÙNG một partition, không hề tản mát.
::::

::::example{#khoa-day-du-vs-partition-key}
Trong THỰC tế, một hàng thường CÓ khoá đầy đủ (VÍ dụ `phongId +
thoiGian + id`) — nhưng CHỈ phần "partition key" (`phongId`) được
đưa VÀO `bam` để định vị partition. Phần CÒN lại của khoá (`thoiGian`,
`id`) dùng để phân biệt CÁC hàng BÊN TRONG cùng một partition — đó LÀ
chủ đề của bài kế tiếp.
::::

::::predict{#doan-doi-so-partition commitOnce}
CÙNG khoá `"phong5"`, nhưng gọi VỚI `soPartition=16` (thay vì `8`) —
`chonPartitionChoKhoa("phong5", 16)` CÓ chắc chắn cho kết quả `2`
(giống LÚC `soPartition=8`) không?

:::opt{correct}
KHÔNG chắc — `bam("phong5") % 16` VÀ `bam("phong5") % 8` LÀ hai phép
chia lấy dư KHÁC nhau, không có LÝ do gì đảm bảo kết quả TRÙNG nhau
:::

:::opt
CÓ — `phong5` LUÔN đi VỀ partition `2` bất kể `soPartition` LÀ bao
nhiêu, vì `bam` LÀ một hàm tất định
::why
Đúng Ở việc `bam("phong5")` LUÔN trả về CÙNG một số NGUYÊN cố định
(tất định) — nhưng phép CHIA lấy dư cho `16` VÀ cho `8` áp DỤNG lên
CÙNG số nguyên đó thường cho ra kết quả KHÁC nhau.

Chỗ lệch: y hệt bài học Ở q12 bài 7 (đổi `soLoi` xáo trộn gần hết
routing) — đổi `soPartition` LÀ đổi phép chia, VÀ kết quả `%`
thường đổi THEO, không có gì đảm bảo giữ nguyên `2`.
::
:::
::::

::::code{#viet_chon_partition}
Hoàn thiện `chonPartitionChoKhoa` — băm khoá RỒI lấy phần dư CHIA cho
số partition.

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

function chonPartitionChoKhoa(khoa: string, soPartition: number): number {
  return ___;
}

console.log(chonPartitionChoKhoa("phong5", 8));
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

function chonPartitionChoKhoa(khoa: string, soPartition: number): number {
  return bam(khoa) % soPartition;
}

console.log(chonPartitionChoKhoa("phong5", 8));
```

```typescript title=test
const p1 = chonPartitionChoKhoa("phong5", 8);
const p2 = chonPartitionChoKhoa("phong5", 8);
if (p1 !== p2) throw new Error("cung khoa, cung soPartition -- phai cho ket qua giong het nhau moi lan goi");
if (p1 !== 2) throw new Error("chonPartitionChoKhoa('phong5', 8) phai la 2");
if (chonPartitionChoKhoa("phong6", 8) !== 3) throw new Error("chonPartitionChoKhoa('phong6', 8) phai la 3");

for (let sp = 1; sp <= 20; sp++) {
  const p = chonPartitionChoKhoa("phong5", sp);
  if (p < 0 || p >= sp) throw new Error(`ket qua phai nam trong [0, soPartition) -- sai voi soPartition=${sp}`);
}

if (chonPartitionChoKhoa("bat-ky-khoa-nao", 1) !== 0) throw new Error("chi 1 partition thi moi khoa phai ve dung partition 0");
```

:::hints
- kind: attention
  body: "Bam khoa roi lay phan du chia cho soPartition -- mot dong."
- kind: strategy
  body: "return bam(khoa) % soPartition;"
- kind: one-line
  body: "return bam(khoa) % soPartition;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Partition key định vị partition NÀO — nhưng bên TRONG một partition,
nhiều hàng vẫn cần một THỨ tự. Thứ tự đó tới TỪ đâu?
::::

::::reflect{#nghi-lai}
`chonPartitionChoKhoa` không giới THIỆU một thuật toán mới — nó LÀ
`chonNodeNgayTho` (q11) VÀ `bam(k) % soNhom` (q13 bài 11's Merkle
tree) áp DỤNG lại LẦN thứ ba, chỉ khác Ở TÊN gọi cho phù hợp NGỮ
cảnh mới. "Partition key" LÀ khái niệm — hashing modulo LÀ kỹ thuật
đã học xuyên suốt track NÀY để CÀI đặt nó. Nhưng MỘT phòng thường có
NHIỀU tin nhắn, tất cả CÙNG rơi vào MỘT partition — chúng cần được
sắp xếp bên TRONG partition đó thế nào?
::::

::::checkpoint{mastery=0.8}
::::

