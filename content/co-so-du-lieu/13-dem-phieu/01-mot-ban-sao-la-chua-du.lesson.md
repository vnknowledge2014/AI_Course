---
id: co-so-du-lieu.dem-phieu.mot-ban-sao-la-chua-du
title: "Một bản sao là chưa đủ"
summary: "docRf1 chỉ đọc được một khoá khi CHÍNH node sở hữu nó (RF=1, qua timNodeChiuTrachNhiem) còn sống — dù BA node khác trong cụm vẫn khoẻ mạnh. Khoá 'khoa1' thuộc về alpha; alpha sập thì 'khoa1' KHÔNG đọc được, bất kể beta/gamma/delta còn sống. Trên 100 khoá đã đo ở q11 bài 8, alpha sở hữu đúng 29 khoá (29%) — nghĩa là 29% dữ liệu 'biến mất' chỉ vì MỘT node trong bốn sập."
locale: vi
track: co-so-du-lieu
module: dem-phieu
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.mot-ban-sao-la-chua-du]
requires: [db.do-ty-le-di-chuyen-vnode-loi]
concepts: [db.mot-ban-sao-la-chua-du]
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
q12 giải xong share-nothing — mỗi khoá thuộc về đúng MỘT lõi/node,
không tranh chấp. NHƯNG "đúng một" cũng có nghĩa LÀ: node đó sập thì
sao?
::::

::::explain{#doc-rf1}
`docRf1` dùng `timNodeChiuTrachNhiem` (q11 bài 4) để tìm node sở hữu
MỘT khoá — CHÍNH là RF=1 (replication factor 1: mỗi khoá chỉ có ĐÚNG
một bản sao, nằm trên đúng một node). Đọc chỉ thành công KHI node
sở hữu ĐÓ còn sống:

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

function xayVongVnode(tenCacNode: string[], soVnodeMoiNode: number): DiemNode[] {
  const vong: DiemNode[] = [];
  for (const ten of tenCacNode) {
    for (let i = 0; i < soVnodeMoiNode; i++) {
      vong.push({ viTri: bam(`${ten}#${i}`), ten });
    }
  }
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

function taoKhoChoMoiNode(tenCacNode: string[]): Map<string, Map<string, number>> {
  const kho = new Map<string, Map<string, number>>();
  for (const ten of tenCacNode) kho.set(ten, new Map<string, number>());
  return kho;
}

function ghiRf1(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, khoa: string, giaTri: number): void {
  const ten = timNodeChiuTrachNhiem(vong, khoa);
  khoCacNode.get(ten)!.set(khoa, giaTri);
}

function docRf1(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, cacNodeSong: Set<string>, khoa: string): number | undefined {
  const ten = timNodeChiuTrachNhiem(vong, khoa);
  if (!cacNodeSong.has(ten)) return undefined;
  return khoCacNode.get(ten)!.get(khoa);
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const khoCacNode = taoKhoChoMoiNode(tenCacNode);

ghiRf1(vong, khoCacNode, "khoa1", 100);
console.log("chu so huu:", timNodeChiuTrachNhiem(vong, "khoa1"));
console.log("doc khi moi node con song:", docRf1(vong, khoCacNode, new Set(tenCacNode), "khoa1"));
console.log("doc khi alpha sap:", String(docRf1(vong, khoCacNode, new Set(["beta", "gamma", "delta"]), "khoa1")));
```

```text title=readonly
chu so huu: alpha
doc khi moi node con song: 100
doc khi alpha sap: undefined
```

`"khoa1"` thuộc về `alpha`. Khi `alpha` còn sống, đọc trả VỀ đúng
`100` đã ghi. Khi `alpha` sập — dù `beta`, `gamma`, `delta` VẪN khoẻ
mạnh cả BA — kết quả LÀ `undefined`. `docRf1` chỉ kiểm tra ĐÚNG một
node DUY nhất (node sở hữu), KHÔNG hề hỏi các node khác — vì chúng
không hề giữ bản sao NÀO của `"khoa1"` cả.
::::

::::example{#khong-lien-quan-so-luong-song}
Đây KHÔNG phải lỗi thiết kế của `docRf1` — đây LÀ hệ quả TRỰC tiếp
của RF=1: mỗi khoá CHỈ tồn tại ở đúng MỘT nơi. Trên `100` khoá
(`"nguoidung0"`..`"nguoidung99"`) đã đo Ở q11 bài 8, phân bố sở hữu
LÀ `{alpha: 29, beta: 25, gamma: 15, delta: 31}`. Nếu `alpha` sập,
KHÔNG phải một vài khoá lẻ tẻ mất — mà đúng `29` TRÊN `100` khoá
(`29%`) hoàn toàn KHÔNG đọc được, cho tới khi `alpha` sống lại. Ba
node còn lại "khoẻ mạnh" không giúp ích GÌ cho `29` khoá đó.
::::

::::predict{#doan-node-khac-song commitOnce}
`"khoa2"` thuộc về `beta` (khác `alpha`). Nếu `beta` sập nhưng
`alpha`, `gamma`, `delta` đều sống — `docRf1` đọc `"khoa1"` (thuộc
`alpha`) CÓ bị ảnh hưởng KHÔNG?

:::opt{correct}
KHÔNG — `"khoa1"` đọc bình thường, vì node sở hữu CỦA nó (`alpha`)
vẫn sống; `docRf1` chỉ quan tâm TỚI node sở hữu của khoá đang được
đọc, không quan tâm node NÀO khác sập
:::

:::opt
CÓ, chậm hoặc lỗi — một node BẤT kỳ sập trong cụm sẽ ảnh hưởng TỚI
mọi thao tác đọc, không riêng khoá NÀO
::why
Gần đúng ở trực giác "một node sập LÀ tin xấu cho cả cụm" — ĐÚNG Ở
mức vận hành THẬT (giám sát, cảnh báo), nhưng KHÔNG đúng Ở mức RF=1
CỤ thể như bài này.

Chỗ lệch: `docRf1` (VÀ RF=1 nói CHUNG) không có khái niệm "cụm khoẻ
hay không khoẻ" — nó CHỈ hỏi đúng MỘT câu: "node sở hữu khoá NÀY có
sống không?". `"khoa1"` VÀ `"khoa2"` có node sở hữu KHÁC nhau
(`alpha` VÀ `beta`), nên số phận của chúng hoàn toàn ĐỘC lập — `beta`
sập chỉ ảnh hưởng những khoá THUỘC về `beta`, không lan sang khoá
thuộc node khác.
::
:::
::::

::::code{#viet_doc_rf1}
Hoàn thiện `docRf1` — nếu node sở hữu khoá KHÔNG còn sống, trả về
`undefined`; nếu còn sống, đọc giá trị THẬT từ kho của nó.

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

function xayVongVnode(tenCacNode: string[], soVnodeMoiNode: number): DiemNode[] {
  const vong: DiemNode[] = [];
  for (const ten of tenCacNode) {
    for (let i = 0; i < soVnodeMoiNode; i++) {
      vong.push({ viTri: bam(`${ten}#${i}`), ten });
    }
  }
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

function taoKhoChoMoiNode(tenCacNode: string[]): Map<string, Map<string, number>> {
  const kho = new Map<string, Map<string, number>>();
  for (const ten of tenCacNode) kho.set(ten, new Map<string, number>());
  return kho;
}

function ghiRf1(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, khoa: string, giaTri: number): void {
  const ten = timNodeChiuTrachNhiem(vong, khoa);
  khoCacNode.get(ten)!.set(khoa, giaTri);
}

function docRf1(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, cacNodeSong: Set<string>, khoa: string): number | undefined {
  const ten = timNodeChiuTrachNhiem(vong, khoa);
  if (___) return undefined;
  return khoCacNode.get(ten)!.get(khoa);
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const khoCacNode = taoKhoChoMoiNode(tenCacNode);
ghiRf1(vong, khoCacNode, "khoa1", 100);
console.log(String(docRf1(vong, khoCacNode, new Set(["beta", "gamma", "delta"]), "khoa1")));
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

function xayVongVnode(tenCacNode: string[], soVnodeMoiNode: number): DiemNode[] {
  const vong: DiemNode[] = [];
  for (const ten of tenCacNode) {
    for (let i = 0; i < soVnodeMoiNode; i++) {
      vong.push({ viTri: bam(`${ten}#${i}`), ten });
    }
  }
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

function taoKhoChoMoiNode(tenCacNode: string[]): Map<string, Map<string, number>> {
  const kho = new Map<string, Map<string, number>>();
  for (const ten of tenCacNode) kho.set(ten, new Map<string, number>());
  return kho;
}

function ghiRf1(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, khoa: string, giaTri: number): void {
  const ten = timNodeChiuTrachNhiem(vong, khoa);
  khoCacNode.get(ten)!.set(khoa, giaTri);
}

function docRf1(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, cacNodeSong: Set<string>, khoa: string): number | undefined {
  const ten = timNodeChiuTrachNhiem(vong, khoa);
  if (!cacNodeSong.has(ten)) return undefined;
  return khoCacNode.get(ten)!.get(khoa);
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const khoCacNode = taoKhoChoMoiNode(tenCacNode);
ghiRf1(vong, khoCacNode, "khoa1", 100);
console.log(String(docRf1(vong, khoCacNode, new Set(["beta", "gamma", "delta"]), "khoa1")));
```

```typescript title=test
console.log("chu khoa1:", timNodeChiuTrachNhiem(vong, "khoa1"));
console.log("chu khoa2:", timNodeChiuTrachNhiem(vong, "khoa2"));

if (docRf1(vong, khoCacNode, new Set(tenCacNode), "khoa1") !== 100) throw new Error("moi node con song thi doc duoc gia tri that");
if (docRf1(vong, khoCacNode, new Set(["beta", "gamma", "delta"]), "khoa1") !== undefined) throw new Error("chu so huu (alpha) sap thi khong doc duoc, du 3 node khac van song");

ghiRf1(vong, khoCacNode, "khoa2", 200);
if (docRf1(vong, khoCacNode, new Set(tenCacNode), "khoa2") !== 200) throw new Error("khoa2 phai doc duoc khi moi node con song");
if (docRf1(vong, khoCacNode, new Set(["alpha", "gamma", "delta"]), "khoa2") !== undefined) throw new Error("chu so huu cua khoa2 (beta) sap thi khong doc duoc, du alpha/gamma/delta van song");
if (docRf1(vong, khoCacNode, new Set(["alpha", "gamma", "delta"]), "khoa1") !== 100) throw new Error("khoa1 van doc duoc khi chu cua no (alpha) con song, du beta sap");
```

:::hints
- kind: attention
  body: "Kiem tra: node so huu (ten) co nam trong tap cacNodeSong khong -- mot dong."
- kind: strategy
  body: "if (!cacNodeSong.has(ten)) return undefined;"
- kind: one-line
  body: "if (!cacNodeSong.has(ten)) return undefined;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "undefined"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
RF=1 dễ hiểu nhưng dễ vỡ — một node sập LÀ mất trắng phần dữ liệu nó
giữ. Nếu MỖI khoá có NHIỀU hơn một bản sao thì sao?
::::

::::reflect{#nghi-lai}
`docRf1` chỉ khác `timNodeChiuTrachNhiem` (q11 bài 4) đúng MỘT điều
kiện: hỏi node sở hữu CÓ sống không TRƯỚC khi đọc. Đơn giản đến mức
DỄ bỏ qua — nhưng CHÍNH sự đơn giản này LÀ vấn đề: hệ thống không có
cách NÀO "chịu đựng" một node sập, vì KHÔNG hề có bản sao dự phòng
nào ở nơi khác. Câu hỏi tự nhiên tiếp theo: nếu MỘT khoá có NHIỀU
bản sao (không chỉ một), cần lưu Ở BAO NHIÊU node?
::::

::::checkpoint{mastery=0.8}
::::
