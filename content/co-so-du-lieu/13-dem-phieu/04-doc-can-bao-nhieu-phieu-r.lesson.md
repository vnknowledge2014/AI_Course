---
id: co-so-du-lieu.dem-phieu.doc-can-bao-nhieu-phieu-r
title: "Đọc cần bao nhiêu phiếu — R"
summary: "docTheoQuorum thu thập giá trị từ CÁC owner đang sống trong preference list, coi LÀ THÀNH công khi số phản hồi >= r; nếu KHÔNG đủ, trả về undefined dù dữ liệu VẪN tồn tại Ở đâu đó. Với RF=3, R=2: gamma sập vẫn đọc được (alpha+delta = 2 phản hồi); gamma VÀ delta cùng sập thì đọc thất bại dù alpha vẫn giữ đúng giá trị — vì CHỈ một phản hồi (1<2), không đủ quorum."
locale: vi
track: co-so-du-lieu
module: dem-phieu
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.doc-can-bao-nhieu-phieu-r]
requires: [db.ghi-can-bao-nhieu-phieu-w]
concepts: [db.doc-can-bao-nhieu-phieu-r]
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
Ghi xong VỚI `W=2` phiếu (bài trước) — MỘT owner (`gamma`) CÓ thể
vẫn chưa nhận được giá trị mới. Đọc LẠI thế NÀO để không "trượt"
đúng CHỖ chưa cập nhật?
::::

::::explain{#doc-theo-quorum}
`docTheoQuorum` thu thập giá TRỊ từ các owner ĐANG sống trong
preference list — coi LÀ đủ tin cậy khi số phản HỒI `>= r` (read
quorum):

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

function laySachSoHuu(vong: DiemNode[], khoa: string, rf: number): string[] {
  const viTriKhoa = bam(khoa);
  let batDau = vong.findIndex((d) => d.viTri >= viTriKhoa);
  if (batDau === -1) batDau = 0;
  const ketQua: string[] = [];
  for (let i = 0; i < vong.length && ketQua.length < rf; i++) {
    const diem = vong[(batDau + i) % vong.length]!;
    if (!ketQua.includes(diem.ten)) ketQua.push(diem.ten);
  }
  return ketQua;
}

function taoKhoChoMoiNode(tenCacNode: string[]): Map<string, Map<string, number>> {
  const kho = new Map<string, Map<string, number>>();
  for (const ten of tenCacNode) kho.set(ten, new Map<string, number>());
  return kho;
}

function ghiTheoQuorum(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, cacNodeSong: Set<string>, khoa: string, giaTri: number, rf: number, w: number): boolean {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  let soAck = 0;
  for (const ten of dsSoHuu) {
    if (!cacNodeSong.has(ten)) continue;
    khoCacNode.get(ten)!.set(khoa, giaTri);
    soAck++;
  }
  return soAck >= w;
}

function docTheoQuorum(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, cacNodeSong: Set<string>, khoa: string, rf: number, r: number): number | undefined {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  const cacGiaTri: number[] = [];
  for (const ten of dsSoHuu) {
    if (!cacNodeSong.has(ten)) continue;
    const gt = khoCacNode.get(ten)!.get(khoa);
    if (gt !== undefined) cacGiaTri.push(gt);
  }
  if (cacGiaTri.length < r) return undefined;
  return cacGiaTri[0];
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
ghiTheoQuorum(vong, kho, new Set(tenCacNode), "khoa1", 100, 3, 3);

console.log("gamma sap, R=2:", docTheoQuorum(vong, kho, new Set(["alpha", "beta", "delta"]), "khoa1", 3, 2));
console.log("gamma+delta sap, R=2:", String(docTheoQuorum(vong, kho, new Set(["alpha", "beta"]), "khoa1", 3, 2)));
```

```text title=readonly
gamma sap, R=2: 100
gamma+delta sap, R=2: undefined
```

`gamma` sập nhưng `alpha` VÀ `delta` (hai owner còn LẠI) vẫn phản
hồi — đủ `r=2`. Khi CẢ `gamma` lẫn `delta` cùng sập, chỉ CÒN `alpha`
phản hồi (`1 < 2`) — `docTheoQuorum` trả về `undefined`, DÙ `alpha`
VẪN giữ đúng giá trị `100` trong kho CỦA nó.
::::

::::example{#khong-du-quorum-khong-co-nghia-mat-du-lieu}
`undefined` Ở TRÊN KHÔNG có nghĩa dữ liệu ĐÃ mất — `alpha` vẫn giữ
`100` NGUYÊN vẹn. `docTheoQuorum` chỉ TỪ chối trả lời KHI không đủ
`r` phản hồi ĐỂ tin tưởng — đây LÀ lựa chọn AN toàn (thà báo "không
đọc được" còn HƠN liều đưa RA một giá trị dựa trên QUÁ ít bằng
chứng), không phải một LỖI thật sự của hệ thống.
::::

::::predict{#doan-w3-r1 commitOnce}
`RF=3`, ghi VỚI `W=3` (CẢ ba owner đều phải xác nhận MỚI tính LÀ
thành công). Sau đó đọc VỚI `R=1` (chỉ cần MỘT owner phản hồi). Nếu
ghi TRƯỚC đó thành công, đọc SAU luôn thấy đúng giá trị MỚI — ĐÚNG
hay sai?

:::opt{correct}
Đúng — `W=3` bảo đảm CẢ ba owner đều CÓ giá trị mới TRƯỚC khi ghi
được TÍNH là xong, nên VỚI `R=1`, owner NÀO trả lời cũng cho đúng
giá trị đó
:::

:::opt
Sai — LUÔN có khả năng owner được chọn đọc CHƯA kịp cập nhật, bất kể
`W` lớn cỡ NÀO
::why
Trực giác NÀY đúng Ở BỐI cảnh TỔNG quát hơn (VÍ dụ ghi VỚI `W` nhỏ
rồi đọc SAU) — cẩn trọng LÀ hợp lý khi KHÔNG biết chắc `W` đã dùng.

Chỗ lệch: câu hỏi ĐÃ cho biết `W=3` — nghĩa LÀ CẢ ba owner PHẢI xác
nhận trước khi `ghiTheoQuorum` trả VỀ `true`. Nếu `W=3` thành công,
theo ĐỊNH nghĩa, không CÒN owner nào "chưa kịp cập NHẬT" — CẢ ba đều
đã ghi XONG. Đọc VỚI `R=1` sau đó, DÙ chỉ hỏi một owner BẤT kỳ, owner
đó chắc chắn có giá trị mới, VÌ không owner nào bị bỏ SÓT khi
`W=3`.
::
:::
::::

::::code{#viet_doc_theo_quorum}
Hoàn thiện `docTheoQuorum` — VỚI mỗi owner còn sống VÀ có giá trị,
thêm giá trị đó vào danh sách phản hồi.

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

function laySachSoHuu(vong: DiemNode[], khoa: string, rf: number): string[] {
  const viTriKhoa = bam(khoa);
  let batDau = vong.findIndex((d) => d.viTri >= viTriKhoa);
  if (batDau === -1) batDau = 0;
  const ketQua: string[] = [];
  for (let i = 0; i < vong.length && ketQua.length < rf; i++) {
    const diem = vong[(batDau + i) % vong.length]!;
    if (!ketQua.includes(diem.ten)) ketQua.push(diem.ten);
  }
  return ketQua;
}

function taoKhoChoMoiNode(tenCacNode: string[]): Map<string, Map<string, number>> {
  const kho = new Map<string, Map<string, number>>();
  for (const ten of tenCacNode) kho.set(ten, new Map<string, number>());
  return kho;
}

function ghiTheoQuorum(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, cacNodeSong: Set<string>, khoa: string, giaTri: number, rf: number, w: number): boolean {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  let soAck = 0;
  for (const ten of dsSoHuu) {
    if (!cacNodeSong.has(ten)) continue;
    khoCacNode.get(ten)!.set(khoa, giaTri);
    soAck++;
  }
  return soAck >= w;
}

function docTheoQuorum(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, cacNodeSong: Set<string>, khoa: string, rf: number, r: number): number | undefined {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  const cacGiaTri: number[] = [];
  for (const ten of dsSoHuu) {
    if (!cacNodeSong.has(ten)) continue;
    const gt = khoCacNode.get(ten)!.get(khoa);
    ___
  }
  if (cacGiaTri.length < r) return undefined;
  return cacGiaTri[0];
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
ghiTheoQuorum(vong, kho, new Set(tenCacNode), "khoa1", 100, 3, 3);
console.log(docTheoQuorum(vong, kho, new Set(["alpha", "beta", "delta"]), "khoa1", 3, 2));
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

function laySachSoHuu(vong: DiemNode[], khoa: string, rf: number): string[] {
  const viTriKhoa = bam(khoa);
  let batDau = vong.findIndex((d) => d.viTri >= viTriKhoa);
  if (batDau === -1) batDau = 0;
  const ketQua: string[] = [];
  for (let i = 0; i < vong.length && ketQua.length < rf; i++) {
    const diem = vong[(batDau + i) % vong.length]!;
    if (!ketQua.includes(diem.ten)) ketQua.push(diem.ten);
  }
  return ketQua;
}

function taoKhoChoMoiNode(tenCacNode: string[]): Map<string, Map<string, number>> {
  const kho = new Map<string, Map<string, number>>();
  for (const ten of tenCacNode) kho.set(ten, new Map<string, number>());
  return kho;
}

function ghiTheoQuorum(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, cacNodeSong: Set<string>, khoa: string, giaTri: number, rf: number, w: number): boolean {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  let soAck = 0;
  for (const ten of dsSoHuu) {
    if (!cacNodeSong.has(ten)) continue;
    khoCacNode.get(ten)!.set(khoa, giaTri);
    soAck++;
  }
  return soAck >= w;
}

function docTheoQuorum(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, cacNodeSong: Set<string>, khoa: string, rf: number, r: number): number | undefined {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  const cacGiaTri: number[] = [];
  for (const ten of dsSoHuu) {
    if (!cacNodeSong.has(ten)) continue;
    const gt = khoCacNode.get(ten)!.get(khoa);
    if (gt !== undefined) cacGiaTri.push(gt);
  }
  if (cacGiaTri.length < r) return undefined;
  return cacGiaTri[0];
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
ghiTheoQuorum(vong, kho, new Set(tenCacNode), "khoa1", 100, 3, 3);
console.log(docTheoQuorum(vong, kho, new Set(["alpha", "beta", "delta"]), "khoa1", 3, 2));
```

```typescript title=test
const kho2 = taoKhoChoMoiNode(tenCacNode);
ghiTheoQuorum(vong, kho2, new Set(tenCacNode), "khoa1", 100, 3, 3);
if (docTheoQuorum(vong, kho2, new Set(tenCacNode), "khoa1", 3, 2) !== 100) throw new Error("tat ca song, R=2 phai doc duoc gia tri 100");
if (docTheoQuorum(vong, kho2, new Set(["alpha", "beta", "delta"]), "khoa1", 3, 2) !== 100) throw new Error("gamma sap, 2 owner con lai (alpha,delta) van du R=2");
if (docTheoQuorum(vong, kho2, new Set(["alpha", "beta"]), "khoa1", 3, 2) !== undefined) throw new Error("gamma+delta sap, chi con 1 owner (alpha) < R=2 phai that bai");
if (docTheoQuorum(vong, kho2, new Set(["beta"]), "khoa1", 3, 1) !== undefined) throw new Error("beta song nhung khong phai owner, khong tinh vao quorum doc");

const kho3 = taoKhoChoMoiNode(tenCacNode);
if (docTheoQuorum(vong, kho3, new Set(tenCacNode), "khoa1", 3, 1) !== undefined) throw new Error("chua ghi gi thi doc R=1 cung phai undefined vi khong co gia tri nao ton tai");
```

:::hints
- kind: attention
  body: "Neu gia tri doc duoc KHONG phai undefined thi them vao danh sach phan hoi -- mot dong."
- kind: strategy
  body: "if (gt !== undefined) cacGiaTri.push(gt);"
- kind: one-line
  body: "if (gt !== undefined) cacGiaTri.push(gt);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "100"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`W` bảo vệ lúc GHI, `R` bảo vệ lúc ĐỌC — nhưng CÓ một mối LIÊN hệ
giữa hai con số NÀY quyết định liệu đọc CÓ đảm bảo thấy đúng giá trị
mới hay KHÔNG. Mối liên hệ đó LÀ gì?
::::

::::reflect{#nghi-lai}
`docTheoQuorum` giống HỆT cấu trúc `ghiTheoQuorum` — cùng duyệt
preference list, cùng đếm phản HỒI, cùng so SÁNH với một ngưỡng.
Nhưng CÂU hỏi thật sự quan trọng KHÔNG nằm Ở TỪNG hàm riêng lẻ: nếu
`W` VÀ `R` được chọn ĐỘC lập, LÀM sao biết chắc tập owner ĐÃ ghi
VÀ tập owner ĐÃ đọc luôn CÓ ít nhất một điểm CHUNG?
::::

::::checkpoint{mastery=0.8}
::::
