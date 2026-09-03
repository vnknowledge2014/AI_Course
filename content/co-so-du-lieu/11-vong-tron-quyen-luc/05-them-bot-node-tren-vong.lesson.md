---
id: co-so-du-lieu.vong-tron-quyen-luc.them-bot-node-tren-vong
title: "Thêm/bớt node trên vòng — chỉ một phần khoá di chuyển"
summary: "demSoKhoaDoiVong đo số khoá đổi node khi thêm epsilon vào vòng bốn node (alpha/beta/gamma/delta). Với 12 khoá mẫu (giống bài 2), chỉ 3/12 (25%) đổi node — so với 9/12 (75%) của cách ngây thơ % soLuongNode. Và cả ba khoá đổi đều chuyển SANG epsilon, node mới — không có khoá nào 'lan' sang gamma hay delta, đúng lý thuyết: chỉ đoạn giữa node mới và node liền trước bị ảnh hưởng."
locale: vi
track: co-so-du-lieu
module: vong-tron-quyen-luc
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.them-bot-node-tren-vong]
requires: [db.tim-node-chiu-trach-nhiem]
concepts: [db.them-bot-node-tren-vong]
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
`% soLuongNode` (bài 2) khiến `75%` khoá đổi node CHỈ VÌ thêm một
máy. Vòng tròn (bài 3-4) CÓ làm tốt hơn không — đo THẬT xem sao.
::::

::::explain{#dem-doi-vong}
`demSoKhoaDoiVong` so sánh node CHỊU trách nhiệm CHO mỗi khoá GIỮA
hai vòng (TRƯỚC VÀ sau khi thêm node), đếm số LẦN kết quả khác NHAU
— CÙNG 12 khoá mẫu ĐÃ dùng Ở bài 2:

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

function demSoKhoaDoiVong(cacKhoa: string[], vongTruoc: DiemNode[], vongSau: DiemNode[]): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (timNodeChiuTrachNhiem(vongTruoc, khoa) !== timNodeChiuTrachNhiem(vongSau, khoa)) {
      dem = dem + 1;
    }
  }
  return dem;
}

const vongTruoc = xayVong(["alpha", "beta", "gamma", "delta"]);
const vongSau = xayVong(["alpha", "beta", "gamma", "delta", "epsilon"]);
const cacKhoa: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa.push("nguoidung" + i);
console.log(demSoKhoaDoiVong(cacKhoa, vongTruoc, vongSau));
```

```text title=readonly
3
```

`3` TRÊN `12` khoá — `25%` — đổi node khi THÊM `epsilon`, SO với
`75%` của `% soLuongNode` (bài 2) TRÊN CÙNG bộ khoá. Vòng tròn KHÔNG
xoá bỏ hoàn TOÀN việc di chuyển dữ liệu (thêm node LUÔN kéo THEO ít
nhất một VÀI khoá cần chuyển) — nhưng giảm nó XUỐNG còn MỘT phần
nhỏ, không phải gần hết.
::::

::::example{#chi-doan-giua-bi-anh-huong}
`epsilon` nằm Ở vị trí `486` trên vòng, XEN giữa `beta(375)` VÀ
`alpha(785)`. TRƯỚC khi thêm `epsilon`, MỌI khoá CÓ vị trí trong
đoạn `(375, 785]` đều thuộc VỀ `alpha`. SAU khi thêm, đoạn ĐÓ bị
CHIA đôi: `(375, 486]` giờ thuộc VỀ `epsilon`, chỉ CÒN `(486, 785]`
vẫn LÀ của `alpha`. Cả BA khoá đổi node (`bam` lần lượt LÀ `474,
441, 384` — đều rơi ĐÚNG trong đoạn `(375, 486]`) đều chuyển SANG
`epsilon` — KHÔNG khoá nào "lan" sang `gamma` hay `delta`, vì vị
trí CỦA hai node ĐÓ không hề thay ĐỔI, VÀ đoạn dữ liệu chúng phụ
trách cũng không hề bị ĐỤNG tới.
::::

::::predict{#doan-bot-node commitOnce}
Thay VÌ thêm `epsilon`, BỚT hẳn `beta` KHỎI vòng bốn node GỐC
(`alpha, beta, gamma, delta` → CHỈ còn `alpha, gamma, delta`). Những
khoá TRƯỚC đây thuộc VỀ `beta` (đoạn `(210, 375]`) giờ thuộc VỀ node
nào?

:::opt{correct}
`alpha` — node ĐẦU tiên gặp được ĐI theo chiều tăng dần TỪ đoạn
`(210, 375]` giờ LÀ `alpha(785)`, vì `beta` KHÔNG còn trên vòng
:::

:::opt
KHÔNG node nào — dữ liệu THUỘC về một node đã BỊ bớt biến MẤT khỏi
hệ thống, cần khôi phục TỪ bản sao Ở nơi khác trước khi CÓ thể tra
cứu lại
::why
Gần đúng ở việc bạn nghĩ TỚI khả năng MẤT dữ liệu khi một node RỜI
khỏi cụm — MỘT lo ngại THẬT sự trong hệ thống thật (cần bản sao —
q13's chủ đề RF/quorum, ngoài phạm vi bài NÀY).

Chỗ lệch: câu hỏi Ở đây CHỈ LÀ "vòng tròn HASH gán khoá vào NODE nào
sau khi bớt", KHÔNG phải "dữ liệu VẬT lý còn tồn tại HAY không".
`timNodeChiuTrachNhiem` chỉ đơn giản KHÔNG còn thấy `beta` trong
mảng `vong` nữa — vòng `for` tiếp TỤC đi tới node kế TIẾP THEO
chiều tăng dần, VÀ với đoạn `(210, 375]`, node ĐÓ chính LÀ `alpha`
(node gần NHẤT tính TỪ `375` trở đi trên vòng đã bớt `beta`).
::
:::
::::

::::code{#viet_dem_so_khoa_doi_vong}
Hoàn thiện `demSoKhoaDoiVong` — đếm số khoá CÓ node chịu trách nhiệm
TRƯỚC khác node SAU.

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

function demSoKhoaDoiVong(cacKhoa: string[], vongTruoc: DiemNode[], vongSau: DiemNode[]): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (___) {
      dem = dem + 1;
    }
  }
  return dem;
}

const vongTruoc = xayVong(["alpha", "beta", "gamma", "delta"]);
const vongSau = xayVong(["alpha", "beta", "gamma", "delta", "epsilon"]);
const cacKhoa: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa.push("nguoidung" + i);
console.log(demSoKhoaDoiVong(cacKhoa, vongTruoc, vongSau));
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

function demSoKhoaDoiVong(cacKhoa: string[], vongTruoc: DiemNode[], vongSau: DiemNode[]): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (timNodeChiuTrachNhiem(vongTruoc, khoa) !== timNodeChiuTrachNhiem(vongSau, khoa)) {
      dem = dem + 1;
    }
  }
  return dem;
}

const vongTruoc = xayVong(["alpha", "beta", "gamma", "delta"]);
const vongSau = xayVong(["alpha", "beta", "gamma", "delta", "epsilon"]);
const cacKhoa: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa.push("nguoidung" + i);
console.log(demSoKhoaDoiVong(cacKhoa, vongTruoc, vongSau));
```

```typescript title=test
const vongTruoc2 = xayVong(["alpha", "beta", "gamma", "delta"]);
const vongSau2 = xayVong(["alpha", "beta", "gamma", "delta", "epsilon"]);
const cacKhoa2: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa2.push("nguoidung" + i);
console.log(demSoKhoaDoiVong(cacKhoa2, vongTruoc2, vongSau2));
if (demSoKhoaDoiVong(cacKhoa2, vongTruoc2, vongSau2) !== 3) throw new Error("them epsilon phai khien dung 3/12 khoa doi node");
if (demSoKhoaDoiVong(cacKhoa2, vongTruoc2, vongTruoc2) !== 0) throw new Error("cung mot vong thi khong khoa nao duoc doi");
if (demSoKhoaDoiVong([], vongTruoc2, vongSau2) !== 0) throw new Error("danh sach khoa rong thi dem phai la 0");
```

:::hints
- kind: attention
  body: "Dieu kien dem: node cua khoa Ở vongTruoc KHAC node cua no Ở vongSau -- mot dong."
- kind: strategy
  body: "timNodeChiuTrachNhiem(vongTruoc, khoa) !== timNodeChiuTrachNhiem(vongSau, khoa)"
- kind: one-line
  body: "if (timNodeChiuTrachNhiem(vongTruoc, khoa) !== timNodeChiuTrachNhiem(vongSau, khoa)) {"
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
25% thay vì 75% — vòng tròn RÕ ràng tốt hơn hẳn. Nhưng "hàm băm"
dùng xuyên suốt tới giờ THỰC ra làm gì bên TRONG — và VÌ sao chất
lượng của nó lại QUAN trọng?
::::

::::reflect{#nghi-lai}
Consistent hashing KHÔNG hứa "KHÔNG khoá nào di chuyển" — nó hứa
"CHỈ những khoá NẰM đúng đoạn bị ảnh hưởng mới di chuyển", VÀ đoạn
đó THƯỜNG chỉ LÀ một phần NHỎ của toàn bộ vòng (Ở đây: đoạn GIỮA
node mới VÀ node liền trước nó). Đây LÀ khác biệt CĂN bản so VỚI
`% soLuongNode`, nơi MỘT thay đổi nhỏ (thêm MỘT node) làm xáo trộn
GẦN như toàn bộ ánh xạ. Toàn BỘ lý luận này đứng TRÊN một giả định
chưa hề kiểm CHỨNG: `bam` LÀ một hàm băm "TỐT" — rải giá trị đều
TRÊN vòng. Nếu KHÔNG, chuyện GÌ xảy ra?
::::

::::checkpoint{mastery=0.85}
::::
