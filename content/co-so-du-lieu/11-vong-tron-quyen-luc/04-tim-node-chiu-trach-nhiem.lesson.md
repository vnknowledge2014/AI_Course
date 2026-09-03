---
id: co-so-du-lieu.vong-tron-quyen-luc.tim-node-chiu-trach-nhiem
title: "Tìm node chịu trách nhiệm"
summary: "timNodeChiuTrachNhiem duyệt vòng (đã sắp xếp theo vị trí), trả về node ĐẦU tiên có vị trí ≥ vị trí khoá. Nếu KHÔNG node nào thoả (khoá nằm SAU node cuối cùng trên vòng), quấn VỀ node đầu tiên trong mảng đã sắp xếp — khoá bam=950 (vượt qua alpha ở 785, node xa nhất) thuộc về delta (66), node ĐẦU tiên khi quấn vòng, không phải alpha."
locale: vi
track: co-so-du-lieu
module: vong-tron-quyen-luc
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.tim-node-chiu-trach-nhiem]
requires: [db.vong-tron-hash-y-tuong]
concepts: [db.tim-node-chiu-trach-nhiem]
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
Bốn node ĐÃ nằm trên vòng — `delta(66), gamma(210), beta(375),
alpha(785)`. Viết hàm THẬT tìm node chịu trách nhiệm CHO một khoá.
::::

::::explain{#tim-node}
`timNodeChiuTrachNhiem` duyệt vòng ĐÃ sắp xếp theo vị trí TĂNG dần,
trả VỀ node đầu tiên CÓ vị trí lớn hơn HOẶC bằng vị trí khoá:

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
    if (diem.viTri >= viTriKhoa) {
      return diem.ten;
    }
  }
  return vong[0]!.ten;
}

const vong = xayVong(["alpha", "beta", "gamma", "delta"]);
console.log(timNodeChiuTrachNhiem(vong, "an"));
console.log(timNodeChiuTrachNhiem(vong, "binh"));
console.log(timNodeChiuTrachNhiem(vong, "dung"));
```

```text title=readonly
alpha
gamma
beta
```

`bam("an")=708` — node ĐẦU tiên trên vòng CÓ vị trí `≥708` LÀ
`alpha(785)`. `bam("binh")=200` rơi TRƯỚC `gamma(210)`. `bam("dung")
=289` rơi TRƯỚC `beta(375)`. Vòng `for` dừng NGAY tại node ĐẦU tiên
thoả điều kiện — KHÔNG cần duyệt hết.
::::

::::example{#quan-vong}
Khoá `"nguoidung2"` có `bam = 950` — LỚN hơn vị trí CỦA `alpha`
(`785`), node XA nhất trên vòng. KHÔNG node nào trong vòng lặp
`for` thoả `diem.viTri >= 950` — vòng lặp chạy HẾT mà không `return`
gì, RƠI xuống dòng cuối:

```typescript title=readonly
console.log(timNodeChiuTrachNhiem(vong, "nguoidung2"));
```

```text title=readonly
delta
```

`vong[0]` (phần TỬ đầu tiên SAU khi đã sắp xếp TĂNG dần) LÀ `delta`
(`66`) — vị trí NHỎ nhất trên vòng. Đây chính LÀ "quấn vòng": khoá
nằm SAU node xa nhất (`alpha`) thuộc VỀ node GẦN vị trí `0` nhất
(`delta`), giống HỆT kim đồng hồ đi QUA `12 giờ` rồi quay LẠI `1
giờ`, không phải đi TIẾP mãi.
::::

::::predict{#doan-hai-node commitOnce}
Vòng CHỈ có ĐÚNG một node DUY nhất (`xayVong(["alpha"])`). Gọi
`timNodeChiuTrachNhiem` VỚI bất kỳ khoá nào — kết QUẢ luôn LÀ gì?

:::opt{correct}
LUÔN LÀ `"alpha"` — chỉ CÓ một node THÌ mọi khoá (dù nằm TRƯỚC hay
SAU vị trí của nó trên vòng) đều thuộc VỀ đúng node ĐÓ
:::

:::opt
Có THỂ báo lỗi — vòng `for` chỉ CÓ một phần tử, KHÔNG đủ để "quấn
vòng" đúng CÁCH
::why
Gần đúng ở việc bạn nghĩ TỚI trường hợp một-phần-tử NHƯ một ca "ĐẶC
biệt" cần xử lý riêng — MỘT trực giác thận trọng hợp lý KHI viết mã
xử lý vòng LẶP.

Chỗ lệch: LOGIC hiện tại KHÔNG cần biết vòng CÓ mấy phần tử — HOẶC
vòng `for` tìm được node THOẢ điều kiện (`diem.viTri >= viTriKhoa`)
NGAY trong LẦN lặp DUY nhất, HOẶC nó KHÔNG tìm thấy VÀ rơi xuống
`return vong[0]!.ten` — mà `vong[0]` VỚI một mảng MỘT phần tử chính
LÀ phần tử ĐÓ. Cả hai NHÁNH đều cho CÙNG kết quả: `"alpha"`. KHÔNG
CÓ lỗi, KHÔNG cần xử lý riêng.
::
:::
::::

::::code{#viet_tim_node_chiu_trach_nhiem}
Hoàn thiện `timNodeChiuTrachNhiem` — khi vòng `for` KHÔNG tìm thấy
node NÀO thoả (khoá nằm SAU node xa nhất), quấn VỀ node ĐẦU tiên
trên vòng đã sắp xếp.

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
    if (diem.viTri >= viTriKhoa) {
      return diem.ten;
    }
  }
  ___
}

const vong = xayVong(["alpha", "beta", "gamma", "delta"]);
console.log(timNodeChiuTrachNhiem(vong, "nguoidung2"));
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
    if (diem.viTri >= viTriKhoa) {
      return diem.ten;
    }
  }
  return vong[0]!.ten;
}

const vong = xayVong(["alpha", "beta", "gamma", "delta"]);
console.log(timNodeChiuTrachNhiem(vong, "nguoidung2"));
```

```typescript title=test
const vong2 = xayVong(["alpha", "beta", "gamma", "delta"]);
console.log(timNodeChiuTrachNhiem(vong2, "nguoidung2"));
if (timNodeChiuTrachNhiem(vong2, "nguoidung2") !== "delta") throw new Error("khoa vuot qua node xa nhat phai quan ve node dau tien (delta)");
if (timNodeChiuTrachNhiem(vong2, "an") !== "alpha") throw new Error("an phai thuoc ve alpha");
if (timNodeChiuTrachNhiem(vong2, "binh") !== "gamma") throw new Error("binh phai thuoc ve gamma");
if (timNodeChiuTrachNhiem(vong2, "dung") !== "beta") throw new Error("dung phai thuoc ve beta");

const vongMotNode = xayVong(["alpha"]);
if (timNodeChiuTrachNhiem(vongMotNode, "bat_ky_khoa_nao") !== "alpha") throw new Error("chi co 1 node thi MOI khoa phai thuoc ve node do");
```

:::hints
- kind: attention
  body: "Khong tim thay node nao thoa dieu kien trong vong lap -- quan VE node dau tien tren vong da sap xep, mot dong."
- kind: strategy
  body: "return vong[0]!.ten;"
- kind: one-line
  body: "return vong[0]!.ten;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "delta"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tìm node chịu trách nhiệm — xong. Thêm/bớt một node trên VÒNG này —
bao nhiêu khoá thực sự BỊ ảnh hưởng?
::::

::::reflect{#nghi-lai}
`timNodeChiuTrachNhiem` LÀ trái tim CỦA consistent hashing — MỘT
vòng lặp tuyến TÍNH, một điều KIỆN so sánh, VÀ một trường hợp
"quấn VÒNG" xử lý bằng ĐÚNG một dòng (`vong[0]`). Ngắn GỌN hơn nhiều
so VỚI vẻ phức tạp CỦA cái tên "consistent hashing" — cái GIÁ trả
cho sự đơn GIẢN này LÀ gì, khi node THỰC sự thêm/bớt?
::::

::::checkpoint{mastery=0.8}
::::
