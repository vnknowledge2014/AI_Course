---
id: co-so-du-lieu.mo-hinh-hoa-theo-cau-hoi.bucket-hoa-partition-key
title: "Bucket hoá partition key"
summary: "taoKhoaCoBucket nối THÊM một hậu tố nhân tạo (bam(userId) % soBucket) vào partition key gốc (ngay) — biến MỘT giá trị cardinality thấp thành nhiều 'bucket' phân biệt. Với soBucket=10, tải lớn nhất mỗi partition giảm từ 200 (không bucket) xuống còn 55 — không đều tuyệt đối (vì bam(userId) không chia đều hoàn hảo), nhưng không còn một partition ôm hết 200 sự kiện của một ngày."
locale: vi
track: co-so-du-lieu
module: mo-hinh-hoa-theo-cau-hoi
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.bucket-hoa-partition-key]
requires: [db.partition-key-sai-hot-partition-ra-doi]
concepts: [db.bucket-hoa-partition-key]
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
`ngay` chỉ CÓ `5` giá trị — không THỂ ép nó có thêm giá trị THẬT.
Nhưng câu hỏi vẫn cần LÀ "theo ngày". Có cách nào GIỮ nguyên câu hỏi
mà VẪN rải đều dữ liệu?
::::

::::explain{#bucket-hoa}
`taoKhoaCoBucket` nối THÊM một hậu tố NHÂN TẠO vào partition key gốc
— `bam(userId) % soBucket` — biến MỘT giá trị `ngay` thành `soBucket`
khoá PHÂN biệt (`"ngay0#0"`, `"ngay0#1"`, ..., `"ngay0#9"`):

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

interface SuKien { id: string; ngay: string; userId: string; }

function taoSuKien(soSuKien: number, soNgay: number, soUser: number): SuKien[] {
  const ds: SuKien[] = [];
  for (let i = 0; i < soSuKien; i++) {
    ds.push({ id: "sk" + i, ngay: "ngay" + (i % soNgay), userId: "user" + (i % soUser) });
  }
  return ds;
}

function demTheoKhoaPhanVung(cacKhoa: string[]): Map<string, number> {
  const dem = new Map<string, number>();
  for (const k of cacKhoa) dem.set(k, (dem.get(k) ?? 0) + 1);
  return dem;
}

function taoKhoaCoBucket(ngay: string, userId: string, soBucket: number): string {
  return ngay + "#" + (bam(userId) % soBucket);
}

const suKien = taoSuKien(1000, 5, 200);
const khongBucket = demTheoKhoaPhanVung(suKien.map((sk) => sk.ngay));
console.log("khong bucket -- tai lon nhat:", Math.max(...khongBucket.values()));

const coBucket = demTheoKhoaPhanVung(suKien.map((sk) => taoKhoaCoBucket(sk.ngay, sk.userId, 10)));
console.log("co bucket (soBucket=10) -- so partition co du lieu:", coBucket.size);
console.log("co bucket (soBucket=10) -- tai lon nhat:", Math.max(...coBucket.values()));
```

```text title=readonly
khong bucket -- tai lon nhat: 200
co bucket (soBucket=10) -- so partition co du lieu: 49
co bucket (soBucket=10) -- tai lon nhat: 55
```

KHÔNG bucket: tải lớn NHẤT LÀ `200` (đúng bằng cả một ngày dồn VÀO
một partition). VỚI `soBucket=10`, mỗi ngày giờ ĐƯỢC rải ra tối đa
`10` bucket khác nhau — tổng cộng `49` partition CÓ dữ liệu (gần
với `5 × 10 = 50` khả dĩ, một VÀI bucket trùng nhau do `bam` không
hoàn hảo tuyệt đối) — tải lớn NHẤT giảm CÒN `55`, không phải `200`.
::::

::::example{#khong-deu-tuyet-doi}
`55` KHÔNG bằng `1000 / 49 ≈ 20.4` (trung bình LÝ thuyết) — `bam`
KHÔNG chia đều HOÀN hảo, một VÀI bucket nhận NHIỀU hơn bucket khác
(y hệt Ở q11 bài 7: `1` điểm/node vẫn lệch `2.77` lần dù hàm băm
"tốt"). Bucket hoá KHÔNG hứa hẹn "cân bằng tuyệt đối" — nó hứa hẹn
"không CÒN một partition ôm HẾT nguyên một ngày". Từ `200` xuống
`55` LÀ một cải thiện THẬT, dù chưa hoàn hảo.
::::

::::predict{#doan-soBucket-1 commitOnce}
Gọi `taoKhoaCoBucket` VỚI `soBucket=1` (chỉ MỘT bucket) cho MỌI sự
kiện. Kết quả sau ĐÓ, VỚI `demTheoKhoaPhanVung`, tải lớn NHẤT CÓ khác
gì so VỚI "không bucket" (bài 4, `200`) không?

:::opt{correct}
KHÔNG khác — `bam(userId) % 1` LUÔN LÀ `0` (bất kỳ số nguyên chia
lấy dư cho `1` đều LÀ `0`), nên MỌI khoá trở THÀNH `"ngayX#0"` — y
hệt CHỈ dùng `ngay` một mình, tải lớn nhất VẪN LÀ `200`
:::

:::opt
CÓ khác, dù chỉ chút ÍT — thêm hậu tố (dù `soBucket=1`) VẪN LÀ một
thao tác THAY đổi khoá, nên phân bố cũng đổi THEO
::why
Trực giác NÀY nhầm "khoá CÓ thêm ký tự" VỚI "khoá phân biệt HƠN" —
hai điều KHÁC nhau.

Chỗ lệch: `bam(userId) % 1` LUÔN cho kết quả `0` cho MỌI `userId`
(phép chia lấy dư CHO `1` không BAO giờ khác `0`) — nên `taoKhoaCoBucket`
LUÔN trả về `ngay + "#0"`, một hậu tố CỐ định, không hề PHÂN biệt gì
thêm. `soBucket=1` chính LÀ "không bucket hoá", chỉ khoác thêm một
CHUỖI vô nghĩa.
::
:::
::::

::::code{#viet_tao_khoa_co_bucket}
Hoàn thiện `taoKhoaCoBucket` — nối `ngay`, dấu `#`, VÀ số bucket
(`bam(userId) % soBucket`) thành một khoá MỚI.

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

function taoKhoaCoBucket(ngay: string, userId: string, soBucket: number): string {
  return ___;
}

console.log(taoKhoaCoBucket("ngay0", "user0", 10));
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

function taoKhoaCoBucket(ngay: string, userId: string, soBucket: number): string {
  return ngay + "#" + (bam(userId) % soBucket);
}

console.log(taoKhoaCoBucket("ngay0", "user0", 10));
```

```typescript title=test
interface SuKien { id: string; ngay: string; userId: string; }
function taoSuKien(soSuKien: number, soNgay: number, soUser: number): SuKien[] {
  const ds: SuKien[] = [];
  for (let i = 0; i < soSuKien; i++) ds.push({ id: "sk" + i, ngay: "ngay" + (i % soNgay), userId: "user" + (i % soUser) });
  return ds;
}
function demTheoKhoaPhanVung(cacKhoa: string[]): Map<string, number> {
  const dem = new Map<string, number>();
  for (const k of cacKhoa) dem.set(k, (dem.get(k) ?? 0) + 1);
  return dem;
}

const k1 = taoKhoaCoBucket("ngay0", "user0", 10);
const k2 = taoKhoaCoBucket("ngay0", "user0", 10);
if (k1 !== k2) throw new Error("cung ngay, cung userId, cung soBucket -- phai cho ket qua giong het nhau");
if (!k1.startsWith("ngay0#")) throw new Error("khoa phai bat dau bang 'ngay0#'");

const suKien = taoSuKien(1000, 5, 200);
const coBucket10 = demTheoKhoaPhanVung(suKien.map((sk) => taoKhoaCoBucket(sk.ngay, sk.userId, 10)));
if (coBucket10.size !== 49) throw new Error("soBucket=10 phai cho dung 49 partition co du lieu");
if (Math.max(...coBucket10.values()) !== 55) throw new Error("soBucket=10 -- tai lon nhat phai la 55");

const coBucket1 = demTheoKhoaPhanVung(suKien.map((sk) => taoKhoaCoBucket(sk.ngay, sk.userId, 1)));
if (coBucket1.size !== 5) throw new Error("soBucket=1 phai cho dung 5 partition co du lieu (giong het khong bucket)");
if (Math.max(...coBucket1.values()) !== 200) throw new Error("soBucket=1 -- tai lon nhat phai van la 200, y het khong bucket hoa");

let tong = 0;
for (const [, dem] of coBucket10) tong += dem;
if (tong !== 1000) throw new Error("tong tat ca bucket cong lai phai bang dung 1000, khong mat du lieu");
```

:::hints
- kind: attention
  body: "Noi ngay, dau '#', va bam(userId) % soBucket thanh mot chuoi -- mot bieu thuc."
- kind: strategy
  body: "return ngay + \"#\" + (bam(userId) % soBucket);"
- kind: one-line
  body: "return ngay + \"#\" + (bam(userId) % soBucket);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "ngay0#"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bucket hoá cứu được GHI — mỗi partition nhận Ít hơn. Nhưng đọc "tất
cả sự kiện ngày X" giờ phải LÀM sao, khi dữ liệu tản RA nhiều bucket?
::::

::::reflect{#nghi-lai}
`taoKhoaCoBucket` không đổi CÂU hỏi ("theo ngày") — nó đổi CÁCH lưu
trữ để trả LỜI câu hỏi đó mà không tạo hot partition. Đây LÀ đánh
đổi ghi‑so‑với‑đọc quen THUỘC (y hệt q14's STCS/LCS/TWCS): ghi giờ
cân BẰNG hơn, nhưng đọc "MỌI sự kiện ngày X" không CÒN LÀ một lần
tra cứu ĐƠN giản — dữ liệu đã bị TẢN ra `soBucket` chỗ khác nhau.
::::

::::checkpoint{mastery=0.85}
::::
