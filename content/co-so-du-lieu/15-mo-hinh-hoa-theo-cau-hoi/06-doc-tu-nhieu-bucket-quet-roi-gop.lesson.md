---
id: co-so-du-lieu.mo-hinh-hoa-theo-cau-hoi.doc-tu-nhieu-bucket-quet-roi-gop
title: "Đọc từ nhiều bucket — quét rồi gộp"
summary: "docTatCaTheoNgay hỏi ĐỦ cả soBucket khoá (ngayX#0 .. ngayX#(N-1)) rồi gộp kết quả lại — vì bucket hoá (bài 5) đã tản dữ liệu MỘT ngày ra N chỗ khác nhau, không còn CÁCH nào đọc bằng một lần tra cứu. Với soBucket=10, đọc 'mọi sự kiện ngày0' cần đúng 10 lần tra thay vì 1 -- nhưng vẫn trả về đủ 200 sự kiện, không thiếu không thừa."
locale: vi
track: co-so-du-lieu
module: mo-hinh-hoa-theo-cau-hoi
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.doc-tu-nhieu-bucket-quet-roi-gop]
requires: [db.bucket-hoa-partition-key]
concepts: [db.doc-tu-nhieu-bucket-quet-roi-gop]
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
Bucket hoá (bài trước) rải một ngày RA `10` partition khác nhau. Câu
hỏi "mọi sự kiện ngày0" giờ KHÔNG còn nằm Ở một chỗ — đọc thế NÀO?
::::

::::explain{#doc-tu-nhieu-bucket}
`docTatCaTheoNgay` phải hỏi ĐỦ cả `soBucket` khoá (`"ngay0#0"` tới
`"ngay0#9"`), RỒI gộp kết quả LẠI thành một mảng DUY nhất — không
còn LÀ một lần tra cứu, mà LÀ `soBucket` lần:

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
  for (let i = 0; i < soSuKien; i++) ds.push({ id: "sk" + i, ngay: "ngay" + (i % soNgay), userId: "user" + (i % soUser) });
  return ds;
}

function taoKhoaCoBucket(ngay: string, userId: string, soBucket: number): string {
  return ngay + "#" + (bam(userId) % soBucket);
}

function xayKhoTheoBucket(cacSuKien: SuKien[], soBucket: number): Map<string, SuKien[]> {
  const kho = new Map<string, SuKien[]>();
  for (const sk of cacSuKien) {
    const k = taoKhoaCoBucket(sk.ngay, sk.userId, soBucket);
    const ds = kho.get(k) ?? [];
    ds.push(sk);
    kho.set(k, ds);
  }
  return kho;
}

function cacKhoaBucketChoNgay(ngay: string, soBucket: number): string[] {
  const ds: string[] = [];
  for (let b = 0; b < soBucket; b++) ds.push(`${ngay}#${b}`);
  return ds;
}

function docTatCaTheoNgay(kho: Map<string, SuKien[]>, ngay: string, soBucket: number): SuKien[] {
  const ketQua: SuKien[] = [];
  for (const k of cacKhoaBucketChoNgay(ngay, soBucket)) {
    const ds = kho.get(k) ?? [];
    ketQua.push(...ds);
  }
  return ketQua;
}

const suKien = taoSuKien(1000, 5, 200);
const kho10 = xayKhoTheoBucket(suKien, 10);
console.log("so lan phai tra cuu:", cacKhoaBucketChoNgay("ngay0", 10).length);
console.log("tong so su kien tim thay:", docTatCaTheoNgay(kho10, "ngay0", 10).length);
```

```text title=readonly
so lan phai tra cuu: 10
tong so su kien tim thay: 200
```

VỚI `soBucket=10`, `docTatCaTheoNgay` phải tra cứu ĐỦ `10` khoá
(`"ngay0#0"` tới `"ngay0#9"`) — nhưng gộp LẠI vẫn ra đúng `200` sự
kiện, KHÔNG thiếu không thừa. Không bucket hoá (q11-q14, đọc THEO
đúng một partition key), đây LÀ một lần tra cứu; CÓ bucket hoá, đây
LÀ `soBucket` lần.
::::

::::example{#chi-phi-doc-ty-le-thuan-voi-soBucket}
Chi phí ĐỌC "mọi sự kiện ngày X" tỉ lệ THUẬN trực tiếp VỚI `soBucket`
— gấp đôi số bucket LÀ gấp đôi số lần tra cứu, KHÔNG phụ thuộc TỔNG
lượng dữ liệu. Đây chính LÀ cái GIÁ đối XỨNG với lợi ích Ở bài 5:
ghi cân bằng HƠN đổi lấy đọc "mọi sự kiện MỘT ngày" đắt HƠN.
::::

::::predict{#doan-doc-mot-nguoi-dung commitOnce}
Nếu câu hỏi đổi THÀNH "sự kiện của MỘT `userId` cụ thể, ngày X" (biết
TRƯỚC cả `userId` LẪN `ngay`) thay VÌ "mọi sự kiện ngày X" — số lần
cần tra cứu CÓ còn LÀ `soBucket=10` không?

:::opt{correct}
KHÔNG — chỉ CẦN đúng `1` lần: `userId` ĐÃ biết trước nghĩa LÀ tính
được CHÍNH XÁC `bam(userId) % soBucket`, tức LÀ biết ĐÚNG một khoá
bucket DUY nhất cần tra, không cần quét CẢ `10`
:::

:::opt
VẪN LÀ `10` — `docTatCaTheoNgay` luôn quét đủ MỌI bucket, bất kể có
biết `userId` trước hay không
::why
Trực giác NÀY đúng CHO chính hàm `docTatCaTheoNgay` (nó KHÔNG có
tham số `userId` nên KHÔNG THỂ tối ưu) — nhưng câu hỏi đang hỏi VỀ
một hàm KHÁC, biết trước `userId`.

Chỗ lệch: `taoKhoaCoBucket(ngay, userId, soBucket)` LÀ một hàm TẤT
định — nếu ĐÃ biết cả `ngay` LẪN `userId`, có THỂ tính TRỰC tiếp
đúng MỘT khoá bucket (y hệt cách viết bình thường tra MỘT khoá), thay
VÌ phải quét hết `soBucket` khoá NHƯ khi chỉ biết `ngay`. Biết CÀNG
nhiều phần của khoá, tra cứu CÀNG rẻ — đây chính LÀ lý do `userId`
(không phải một trường ngẫu nhiên NÀO khác) được chọn LÀM nguồn
băm cho hậu tố bucket Ở bài 5.
::
:::
::::

::::code{#viet_doc_tat_ca_theo_ngay}
Hoàn thiện `docTatCaTheoNgay` — với mỗi khoá bucket của ngày ĐÓ, nối
kết quả tra được VÀO mảng chung.

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

interface SuKien { id: string; ngay: string; userId: string; }

function taoSuKien(soSuKien: number, soNgay: number, soUser: number): SuKien[] {
  const ds: SuKien[] = [];
  for (let i = 0; i < soSuKien; i++) ds.push({ id: "sk" + i, ngay: "ngay" + (i % soNgay), userId: "user" + (i % soUser) });
  return ds;
}

function taoKhoaCoBucket(ngay: string, userId: string, soBucket: number): string {
  return ngay + "#" + (bam(userId) % soBucket);
}

function xayKhoTheoBucket(cacSuKien: SuKien[], soBucket: number): Map<string, SuKien[]> {
  const kho = new Map<string, SuKien[]>();
  for (const sk of cacSuKien) {
    const k = taoKhoaCoBucket(sk.ngay, sk.userId, soBucket);
    const ds = kho.get(k) ?? [];
    ds.push(sk);
    kho.set(k, ds);
  }
  return kho;
}

function cacKhoaBucketChoNgay(ngay: string, soBucket: number): string[] {
  const ds: string[] = [];
  for (let b = 0; b < soBucket; b++) ds.push(`${ngay}#${b}`);
  return ds;
}

function docTatCaTheoNgay(kho: Map<string, SuKien[]>, ngay: string, soBucket: number): SuKien[] {
  const ketQua: SuKien[] = [];
  for (const k of cacKhoaBucketChoNgay(ngay, soBucket)) {
    ___
  }
  return ketQua;
}

const suKien = taoSuKien(1000, 5, 200);
const kho10 = xayKhoTheoBucket(suKien, 10);
console.log(docTatCaTheoNgay(kho10, "ngay0", 10).length);
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

interface SuKien { id: string; ngay: string; userId: string; }

function taoSuKien(soSuKien: number, soNgay: number, soUser: number): SuKien[] {
  const ds: SuKien[] = [];
  for (let i = 0; i < soSuKien; i++) ds.push({ id: "sk" + i, ngay: "ngay" + (i % soNgay), userId: "user" + (i % soUser) });
  return ds;
}

function taoKhoaCoBucket(ngay: string, userId: string, soBucket: number): string {
  return ngay + "#" + (bam(userId) % soBucket);
}

function xayKhoTheoBucket(cacSuKien: SuKien[], soBucket: number): Map<string, SuKien[]> {
  const kho = new Map<string, SuKien[]>();
  for (const sk of cacSuKien) {
    const k = taoKhoaCoBucket(sk.ngay, sk.userId, soBucket);
    const ds = kho.get(k) ?? [];
    ds.push(sk);
    kho.set(k, ds);
  }
  return kho;
}

function cacKhoaBucketChoNgay(ngay: string, soBucket: number): string[] {
  const ds: string[] = [];
  for (let b = 0; b < soBucket; b++) ds.push(`${ngay}#${b}`);
  return ds;
}

function docTatCaTheoNgay(kho: Map<string, SuKien[]>, ngay: string, soBucket: number): SuKien[] {
  const ketQua: SuKien[] = [];
  for (const k of cacKhoaBucketChoNgay(ngay, soBucket)) {
    const ds = kho.get(k) ?? [];
    ketQua.push(...ds);
  }
  return ketQua;
}

const suKien = taoSuKien(1000, 5, 200);
const kho10 = xayKhoTheoBucket(suKien, 10);
console.log(docTatCaTheoNgay(kho10, "ngay0", 10).length);
```

```typescript title=test
const suKien2 = taoSuKien(1000, 5, 200);

const kho10T = xayKhoTheoBucket(suKien2, 10);
if (docTatCaTheoNgay(kho10T, "ngay0", 10).length !== 200) throw new Error("soBucket=10, ngay0 phai tim dung 200 su kien");
if (docTatCaTheoNgay(kho10T, "ngay4", 10).length !== 200) throw new Error("soBucket=10, ngay4 phai tim dung 200 su kien");

const kho1T = xayKhoTheoBucket(suKien2, 1);
if (docTatCaTheoNgay(kho1T, "ngay0", 1).length !== 200) throw new Error("soBucket=1 (khong bucket hoa) van phai tim dung 200 su kien");

for (const sk of docTatCaTheoNgay(kho10T, "ngay0", 10)) {
  if (sk.ngay !== "ngay0") throw new Error("moi su kien tra ve phai thuc su co ngay=ngay0, khong lan tu bucket khac");
}

if (docTatCaTheoNgay(kho10T, "ngay-khong-ton-tai", 10).length !== 0) throw new Error("ngay khong ton tai thi phai tra ve mang rong, khong loi");

const khoRong = xayKhoTheoBucket([], 10);
if (docTatCaTheoNgay(khoRong, "ngay0", 10).length !== 0) throw new Error("kho rong thi phai tra ve mang rong");
```

:::hints
- kind: attention
  body: "Tra cuu bucket k trong kho (mang rong neu chua co), noi tat ca vao ketQua -- mot dong."
- kind: strategy
  body: "const ds = kho.get(k) ?? []; ketQua.push(...ds);"
- kind: one-line
  body: "const ds = kho.get(k) ?? []; ketQua.push(...ds);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "200"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`soBucket` càng LỚN, ghi CÀNG cân bằng nhưng đọc CÀNG đắt. Bao nhiêu
bucket LÀ đủ?
::::

::::reflect{#nghi-lai}
`docTatCaTheoNgay` LÀ "quét rồi gộp" (scatter-gather) — một mẫu hình
ĐÃ xuất hiện dưới TÊN khác xuyên suốt track NÀY: đọc quorum (q13
bài 4) hỏi NHIỀU owner rồi chọn; anti-entropy (q13 bài 10-11) so
sánh NHIỀU bản sao rồi hợp NHẤT. Ở đây, "nhiều nguồn" LÀ nhiều
bucket của CÙNG một ngày. Cái GIÁ luôn giống nhau: hỏi nhiều nơi LÀ
chậm hơn hỏi một nơi — câu hỏi còn LẠI LÀ đánh đổi bao nhiêu LÀ vừa.
::::

::::checkpoint{mastery=0.85}
::::
