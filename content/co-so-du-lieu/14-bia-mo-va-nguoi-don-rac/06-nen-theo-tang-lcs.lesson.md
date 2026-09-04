---
id: co-so-du-lieu.bia-mo-va-nguoi-don-rac.nen-theo-tang-lcs
title: "Nén theo tầng — LCS"
summary: "demSoBangCanDocLCS đếm số SSTable phải đọc để tìm một khoá, cộng dồn qua các tầng. Với 21 SSTable tổ chức thành 3 tầng KHÔNG chồng lấn phạm vi khoá trong mỗi tầng (Leveled Compaction), chỉ cần đọc đúng 3 bảng (1 mỗi tầng) để tìm bất kỳ khoá nào -- so với 21/21 nếu CÙNG 21 bảng đó có phạm vi trùng lặp hoàn toàn (như STCS gộp theo kích cỡ, không theo khoá)."
locale: vi
track: co-so-du-lieu
module: bia-mo-va-nguoi-don-rac
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.nen-theo-tang-lcs]
requires: [db.nen-chon-theo-kich-co-stcs]
concepts: [db.nen-theo-tang-lcs]
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
STCS (bài trước) gộp theo KÍCH cỡ — hai bảng CÙNG kích cỡ VẪN có THỂ
chứa khoá TRÙNG phạm vi. Đọc MỘT khoá phải kiểm tra BAO nhiêu bảng?
::::

::::explain{#doc-toi-da-lcs}
"Leveled Compaction Strategy" (LCS) tổ chức SSTable thành các TẦNG —
TRONG mỗi tầng (từ tầng `1` trở lên), các bảng được xây để CÓ phạm vi
khoá KHÔNG chồng lấn nhau (mỗi khoá chỉ THUỘC đúng MỘT bảng trong
tầng đó). `demSoBangCanDocLCS` đếm số bảng THẬT sự phải đọc để tìm
một khoá — cộng dồn QUA các tầng:

```typescript title=readonly
interface PhamVi { min: number; max: number; }

function chiaThanhCacManh(soManh: number, tongKichCo: number): PhamVi[] {
  const ketQua: PhamVi[] = [];
  const kichCoManh = tongKichCo / soManh;
  for (let i = 0; i < soManh; i++) ketQua.push({ min: i * kichCoManh, max: (i + 1) * kichCoManh - 1 });
  return ketQua;
}

function soManhChuaKhoa(cacManh: PhamVi[], khoa: number): number {
  return cacManh.filter((m) => khoa >= m.min && khoa <= m.max).length;
}

function demSoBangCanDocLCS(cacTang: PhamVi[][], khoa: number): number {
  let dem = 0;
  for (const tang of cacTang) dem += soManhChuaKhoa(tang, khoa);
  return dem;
}

const TONG = 400;
const tang0 = chiaThanhCacManh(1, TONG);   // tang 0: 1 bang, chua flush/gop -- phu toan bo
const tang1 = chiaThanhCacManh(4, TONG);   // tang 1: 4 manh KHONG chong lan
const tang2 = chiaThanhCacManh(16, TONG);  // tang 2: 16 manh KHONG chong lan
const cacTang = [tang0, tang1, tang2];

console.log("tong so SSTable:", tang0.length + tang1.length + tang2.length);
console.log("so bang can doc de tim khoa=250:", demSoBangCanDocLCS(cacTang, 250));
console.log("so bang can doc de tim khoa=5:", demSoBangCanDocLCS(cacTang, 5));
```

```text title=readonly
tong so SSTable: 21
so bang can doc de tim khoa=250: 3
so bang can doc de tim khoa=5: 3
```

`21` SSTable TỔNG cộng (`1+4+16`), NHƯNG BẤT kỳ khoá NÀO cũng chỉ CẦN
đọc ĐÚNG `3` bảng — một Ở MỖI tầng (`soManhChuaKhoa` LUÔN trả về `0`
hoặc `1` cho MỘT tầng, VÌ các mảnh trong TẦNG đó không chồng LẤN).
Đây LÀ lý DO gọi LÀ "leveled" — mỗi TẦNG là một PHÂN hoạch đầy đủ,
KHÔNG trùng lặp, của TOÀN bộ không gian khoá.
::::

::::example{#so-sanh-neu-khong-phan-hoach}
NẾU cùng `21` bảng đó được tổ chức KIỂU STCS (gộp theo kích cỡ,
KHÔNG theo phạm vi khoá — MỖI bảng CÓ thể chứa BẤT kỳ khoá nào, phạm
vi TRÙNG lặp hoàn toàn), thì MỌI khoá Ở trường hợp XẤU nhất phải kiểm
tra CẢ `21` bảng — không CÓ cách nào loại trừ trước bảng NÀO chỉ dựa
VÀO phạm vi. `demSoBangCanDocLCS([manhTrungLap0, manhTrungLap1,
manhTrungLap2], khoa)` (mỗi manh phủ `[0, 399]` HẾT) cho ĐÚNG `21` —
gấp `7` lần so VỚI LCS trên CÙNG một khối lượng dữ liệu.
::::

::::predict{#doan-them-tang commitOnce}
Thêm MỘT tầng thứ TƯ (`tang3`, `64` mảnh KHÔNG chồng lấn, `TONG=400`)
VÀO `cacTang`. Số bảng cần đọc CHO một khoá BẤT kỳ giờ LÀ bao nhiêu?

:::opt{correct}
`4` — MỖI tầng luôn đóng góp ĐÚNG `1` bảng (vì không chồng LẤN
TRONG một tầng), thêm một tầng LÀ cộng thêm ĐÚNG `1`
:::

:::opt
`64` — tầng MỚI có `64` mảnh, và VÌ mảnh nhỏ hơn NÊN khả năng khoá
rơi VÀO nhiều mảnh cao HƠN
::why
Trực giác NÀY nhầm "số MẢNH trong một tầng" VỚI "số mảnh CHỨA một
khoá cụ thể" — hai con SỐ hoàn toàn khác nhau.

Chỗ lệch: DÙ tầng `3` có `64` mảnh, chúng VẪN không chồng lấn NHAU
(`chiaThanhCacManh` luôn CHIA đều, `min`/`max` LIÊN tiếp không trùng)
— một khoá CHỈ rơi và ĐÚNG một trong `64` mảnh đó. Số mảnh CÀNG nhiều
trong MỘT tầng không hề làm TĂNG số bảng cần đọc CHO tầng đó — luôn
LÀ `0` hoặc `1`.
::
:::
::::

::::code{#viet_dem_so_bang_can_doc_lcs}
Hoàn thiện `soManhChuaKhoa` — đếm số mảnh CÓ phạm vi chứa khoá.

```typescript title=starter
interface PhamVi { min: number; max: number; }

function chiaThanhCacManh(soManh: number, tongKichCo: number): PhamVi[] {
  const ketQua: PhamVi[] = [];
  const kichCoManh = tongKichCo / soManh;
  for (let i = 0; i < soManh; i++) ketQua.push({ min: i * kichCoManh, max: (i + 1) * kichCoManh - 1 });
  return ketQua;
}

function soManhChuaKhoa(cacManh: PhamVi[], khoa: number): number {
  return cacManh.filter((m) => ___).length;
}

function demSoBangCanDocLCS(cacTang: PhamVi[][], khoa: number): number {
  let dem = 0;
  for (const tang of cacTang) dem += soManhChuaKhoa(tang, khoa);
  return dem;
}

const tang0 = chiaThanhCacManh(1, 400);
const tang1 = chiaThanhCacManh(4, 400);
const tang2 = chiaThanhCacManh(16, 400);
console.log(demSoBangCanDocLCS([tang0, tang1, tang2], 250));
```

```typescript title=solution
interface PhamVi { min: number; max: number; }

function chiaThanhCacManh(soManh: number, tongKichCo: number): PhamVi[] {
  const ketQua: PhamVi[] = [];
  const kichCoManh = tongKichCo / soManh;
  for (let i = 0; i < soManh; i++) ketQua.push({ min: i * kichCoManh, max: (i + 1) * kichCoManh - 1 });
  return ketQua;
}

function soManhChuaKhoa(cacManh: PhamVi[], khoa: number): number {
  return cacManh.filter((m) => khoa >= m.min && khoa <= m.max).length;
}

function demSoBangCanDocLCS(cacTang: PhamVi[][], khoa: number): number {
  let dem = 0;
  for (const tang of cacTang) dem += soManhChuaKhoa(tang, khoa);
  return dem;
}

const tang0 = chiaThanhCacManh(1, 400);
const tang1 = chiaThanhCacManh(4, 400);
const tang2 = chiaThanhCacManh(16, 400);
console.log(demSoBangCanDocLCS([tang0, tang1, tang2], 250));
```

```typescript title=test
const t0 = chiaThanhCacManh(1, 400);
const t1 = chiaThanhCacManh(4, 400);
const t2 = chiaThanhCacManh(16, 400);
const cacTang = [t0, t1, t2];

if (demSoBangCanDocLCS(cacTang, 250) !== 3) throw new Error("moi tang chi dong gop dung 1 bang -- 3 tang phai la 3");
if (demSoBangCanDocLCS(cacTang, 5) !== 3) throw new Error("khoa nam o dau khoang cung phai la 3, khong phu thuoc vi tri khoa");
if (demSoBangCanDocLCS(cacTang, 399) !== 3) throw new Error("khoa o cuoi khoang cung phai la 3");

if (soManhChuaKhoa(t1, 100) !== 1) throw new Error("mot khoa chi thuoc dung 1 manh trong mot tang khong chong lan");

const t3 = chiaThanhCacManh(64, 400);
if (demSoBangCanDocLCS([t0, t1, t2, t3], 250) !== 4) throw new Error("them 1 tang thi cong dung them 1, bat ke tang do co bao nhieu manh");

const manhTrungLap = [{ min: 0, max: 399 }, { min: 0, max: 399 }, { min: 0, max: 399 }];
if (soManhChuaKhoa(manhTrungLap, 250) !== 3) throw new Error("neu pham vi TRUNG LAP het (khong phan hoach), phai kiem tra CA 3");
```

:::hints
- kind: attention
  body: "Loc cac manh ma khoa nam trong khoang [min, max] cua no -- mot bieu thuc so sanh."
- kind: strategy
  body: "return cacManh.filter((m) => khoa >= m.min && khoa <= m.max).length;"
- kind: one-line
  body: "return cacManh.filter((m) => khoa >= m.min && khoa <= m.max).length;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
LCS bảo đảm đọc THẤP, ổn định (bằng số TẦNG, không phụ thuộc tổng số
bảng) — nhưng để GIỮ mỗi tầng không chồng LẤN, nó phải viết LẠI dữ
liệu nhiều LẦN hơn STCS. Còn dữ liệu THEO thời gian (time-series)
thì sao — có chiến lược NÀO tận dụng được TÍNH chất đó?
::::

::::reflect{#nghi-lai}
LCS đổi chiến LƯỢC gộp: thay VÌ "cùng kích cỡ" (STCS), nó ĐÒI hỏi
"cùng tầng thì KHÔNG chồng lấn phạm vi khoá" — cái GIÁ LÀ mỗi lần một
bảng MỚI lọt vào một tầng đã CÓ, tầng đó phải được TỔ chức LẠI (viết
lại NHIỀU bảng) để giữ tính chất "không chồng lấn". Đổi LẠI, đọc trở
nên cực kỳ DỰ đoán được: LUÔN đúng bằng số tầng, DÙ tổng dữ liệu lớn
tới đâu. STCS (bài 5) VÀ LCS (bài NÀY) LÀ hai điểm khác nhau trên
CÙNG một trục đánh đổi ghi‑so‑với‑đọc.
::::

::::checkpoint{mastery=0.85}
::::
