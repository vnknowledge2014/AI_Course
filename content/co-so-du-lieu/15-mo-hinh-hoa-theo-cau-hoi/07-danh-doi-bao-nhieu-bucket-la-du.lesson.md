---
id: co-so-du-lieu.mo-hinh-hoa-theo-cau-hoi.danh-doi-bao-nhieu-bucket-la-du
title: "Đánh đổi — bao nhiêu bucket là đủ?"
summary: "soSanhSoBucket đo tải lớn nhất mỗi partition (ghi) VÀ số lần tra cứu (đọc) cho cùng một khối lượng dữ liệu, với soBucket=1,4,16. N=1: tải 200, đọc 1 lần. N=4: tải 75, đọc 4 lần. N=16: tải 30, đọc 16 lần. Tải giảm dần khi N tăng, chi phí đọc tăng đúng theo N -- không có N 'tốt nhất' tuyệt đối, giống hệt tinh thần STCS/LCS/TWCS (q14 bài 8)."
locale: vi
track: co-so-du-lieu
module: mo-hinh-hoa-theo-cau-hoi
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.danh-doi-bao-nhieu-bucket-la-du]
requires: [db.doc-tu-nhieu-bucket-quet-roi-gop]
concepts: [db.danh-doi-bao-nhieu-bucket-la-du]
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
`soBucket=10` (bài 5-6) LÀ một con số CHỌN sẵn — nhưng KHÔNG có lý do
gì nói `10` LÀ đúng. Đo NHIỀU giá trị `soBucket` trên CÙNG dữ liệu để
thấy RÕ đánh đổi.
::::

::::explain{#so-sanh-lab}
`soSanhSoBucket` chạy CÙNG `1000` sự kiện QUA ba giá trị `soBucket`
khác nhau, đo hai chỉ số: `taiLonNhat` (ghi — partition NẶNG nhất
phải gánh bao nhiêu) VÀ `soLanTraCuu` (đọc — cần hỏi bao nhiêu bucket
để trả lời "mọi sự kiện MỘT ngày"):

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
function demTheoKhoaPhanVung(cacKhoa: string[]): Map<string, number> {
  const dem = new Map<string, number>();
  for (const k of cacKhoa) dem.set(k, (dem.get(k) ?? 0) + 1);
  return dem;
}

interface KetQuaBucket { soBucket: number; taiLonNhat: number; soLanTraCuu: number; }

function soSanhSoBucket(cacSuKien: SuKien[], soBucket: number): KetQuaBucket {
  const theoKhoa = demTheoKhoaPhanVung(cacSuKien.map((sk) => taoKhoaCoBucket(sk.ngay, sk.userId, soBucket)));
  return { soBucket, taiLonNhat: Math.max(...theoKhoa.values()), soLanTraCuu: soBucket };
}

const duLieu = taoSuKien(1000, 5, 200);
for (const n of [1, 4, 16]) {
  console.log(JSON.stringify(soSanhSoBucket(duLieu, n)));
}
```

```text title=readonly
{"soBucket":1,"taiLonNhat":200,"soLanTraCuu":1}
{"soBucket":4,"taiLonNhat":75,"soLanTraCuu":4}
{"soBucket":16,"taiLonNhat":30,"soLanTraCuu":16}
```

`soBucket=1`: tải NẶNG nhất `200`, chỉ `1` lần tra CỨU. `soBucket=4`:
tải giảm CÒN `75`, đọc phải TRẢ giá `4` lần. `soBucket=16`: tải giảm
tiếp CÒN `30`, nhưng đọc GIỜ tốn `16` lần. Ghi CÀNG cân bằng, đọc
CÀNG đắt — HAI cột số đi ngược CHIỀU nhau theo đúng `soBucket`.
::::

::::example{#khong-co-n-tot-nhat}
KHÔNG CÓ giá trị `soBucket` "đúng" chung CHO mọi hệ thống — y hệt
STCS/LCS/TWCS (q14 bài 8) không CÓ chiến lược nén "tốt nhất". Hệ
THỐNG ghi nhiều, hiếm khi đọc "cả một ngày" NÊN chấp nhận `soBucket`
lớn (ưu tiên GHI cân bằng). Hệ thống đọc "cả ngày" THƯỜNG xuyên nên
giữ `soBucket` nhỏ (chấp nhận tải LỆCH hơn, đổi lấy đọc rẻ). Lựa
chọn ĐÚNG phụ thuộc workload THẬT, không phải một con SỐ vạn năng.
::::

::::predict{#doan-soBucket-tang-gap-doi commitOnce}
So SÁNH `soSanhSoBucket(duLieu, 4)` (`taiLonNhat: 75`) VÀ
`soSanhSoBucket(duLieu, 16)` (`taiLonNhat: 30`) — tăng `soBucket` LÊN
gấp `4` lần (TỪ `4` lên `16`) có làm `soLanTraCuu` tăng ĐÚNG gấp `4`
lần theo không?

:::opt{correct}
CÓ — `soLanTraCuu` LUÔN bằng CHÍNH `soBucket` (`4` rồi `16`), nên
tăng gấp `4` lần Ở đầu VÀO nghĩa LÀ tăng đúng gấp `4` lần Ở đầu ra —
quan hệ NÀY LÀ tuyến tính TUYỆT đối, không xấp xỉ
:::

:::opt
Không hẳn — `soLanTraCuu` phụ THUỘC cả dữ liệu THẬT (số bucket THỰC
sự có ít nhất một sự kiện), không CHỈ phụ thuộc `soBucket` khai báo
::why
Trực giác NÀY hợp lý CHO `taiLonNhat` (con SỐ ĐÓ phụ thuộc dữ liệu
thật, KHÔNG tuyến tính hoàn hảo) — nhưng KHÔNG đúng CHO `soLanTraCuu`.

Chỗ lệch: `docTatCaTheoNgay` (bài 6) VÀ `soSanhSoBucket` Ở ĐÂY đều
định NGHĨA `soLanTraCuu = soBucket` — một hàm tra `soBucket` khoá đã
BIẾT trước (`"ngay0#0"` tới `"ngay0#(N-1)"`), KHÔNG hề phụ thuộc bucket
nào THỰC sự có dữ liệu hay không (bucket rỗng VẪN bị tra, chỉ trả về
mảng rỗng). Đây LÀ con số DUY nhất trong bài NÀY hoàn TOÀN tất định,
không cần đo — CHỈ CẦN đọc `soBucket` LÀ biết ngay `soLanTraCuu`.
::
:::
::::

::::code{#viet_so_sanh_so_bucket}
Hoàn thiện `soSanhSoBucket` — tính `taiLonNhat` LÀ giá trị lớn NHẤT
trong bảng đếm theo khoá CÓ bucket.

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
function demTheoKhoaPhanVung(cacKhoa: string[]): Map<string, number> {
  const dem = new Map<string, number>();
  for (const k of cacKhoa) dem.set(k, (dem.get(k) ?? 0) + 1);
  return dem;
}

interface KetQuaBucket { soBucket: number; taiLonNhat: number; soLanTraCuu: number; }

function soSanhSoBucket(cacSuKien: SuKien[], soBucket: number): KetQuaBucket {
  const theoKhoa = demTheoKhoaPhanVung(cacSuKien.map((sk) => taoKhoaCoBucket(sk.ngay, sk.userId, soBucket)));
  return { soBucket, taiLonNhat: ___, soLanTraCuu: soBucket };
}

const duLieuMau = taoSuKien(1000, 5, 200);
console.log(soSanhSoBucket(duLieuMau, 4).taiLonNhat);
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
function demTheoKhoaPhanVung(cacKhoa: string[]): Map<string, number> {
  const dem = new Map<string, number>();
  for (const k of cacKhoa) dem.set(k, (dem.get(k) ?? 0) + 1);
  return dem;
}

interface KetQuaBucket { soBucket: number; taiLonNhat: number; soLanTraCuu: number; }

function soSanhSoBucket(cacSuKien: SuKien[], soBucket: number): KetQuaBucket {
  const theoKhoa = demTheoKhoaPhanVung(cacSuKien.map((sk) => taoKhoaCoBucket(sk.ngay, sk.userId, soBucket)));
  return { soBucket, taiLonNhat: Math.max(...theoKhoa.values()), soLanTraCuu: soBucket };
}

const duLieuMau = taoSuKien(1000, 5, 200);
console.log(soSanhSoBucket(duLieuMau, 4).taiLonNhat);
```

```typescript title=test
const duLieuKt = taoSuKien(1000, 5, 200);

const kq1 = soSanhSoBucket(duLieuKt, 1);
if (kq1.taiLonNhat !== 200) throw new Error("soBucket=1 -- taiLonNhat phai la 200");
if (kq1.soLanTraCuu !== 1) throw new Error("soBucket=1 -- soLanTraCuu phai la 1");

const kq4 = soSanhSoBucket(duLieuKt, 4);
if (kq4.taiLonNhat !== 75) throw new Error("soBucket=4 -- taiLonNhat phai la 75");
if (kq4.soLanTraCuu !== 4) throw new Error("soBucket=4 -- soLanTraCuu phai la 4");

const kq16 = soSanhSoBucket(duLieuKt, 16);
if (kq16.taiLonNhat !== 30) throw new Error("soBucket=16 -- taiLonNhat phai la 30");
if (kq16.soLanTraCuu !== 16) throw new Error("soBucket=16 -- soLanTraCuu phai la 16");

if (!(kq16.taiLonNhat < kq4.taiLonNhat && kq4.taiLonNhat < kq1.taiLonNhat)) {
  throw new Error("soBucket cang lon thi taiLonNhat cang giam -- 16 < 4 < 1 phai dung thu tu");
}
if (!(kq1.soLanTraCuu < kq4.soLanTraCuu && kq4.soLanTraCuu < kq16.soLanTraCuu)) {
  throw new Error("soBucket cang lon thi soLanTraCuu cang tang -- 1 < 4 < 16 phai dung thu tu");
}
```

:::hints
- kind: attention
  body: "taiLonNhat la gia tri LON NHAT trong Map theoKhoa -- mot bieu thuc."
- kind: strategy
  body: "taiLonNhat: Math.max(...theoKhoa.values()),"
- kind: one-line
  body: "taiLonNhat: Math.max(...theoKhoa.values()),"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "75"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Partition key, clustering column, bucket hoá — mọi mảnh ghép ĐÃ có.
Ráp tất cả VÀO một thiết kế THẬT cho một câu hỏi CỤ thể trông ra SAO?
::::

::::reflect{#nghi-lai}
`soSanhSoBucket` không đo một con SỐ "đúng" — nó phơi BÀY một trục
đánh đổi để NGƯỜI thiết kế tự CHỌN, đúng tinh thần "chậm mà CHẮC" đã
xuyên suốt khoá HỌC này: đếm THẬT thay vì đoán. Bài BOSS tiếp theo
sẽ đặt CÂU hỏi ngược lại: cho một câu hỏi thường GẶP cụ thể, thiết
kế NÀO (partition key + clustering column + bucket) trả LỜI nó tốt
nhất?
::::

::::checkpoint{mastery=0.85}
::::
