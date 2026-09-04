---
id: co-so-du-lieu.bia-mo-va-nguoi-don-rac.xoa-la-ghi-khong-phai-xoa
title: "Xoá là ghi, không phải xoá"
summary: "xoaTheoQuorum viết một 'bia mộ' (BanGhi{giaTri:null,thoiGian}) tới các owner còn sống, dùng lại đúng cơ chế ghiTheoQuorum của q13 — CHỈ khác giá trị ghi LÀ null. Nếu một owner sập LÚC xoá (gamma), nó KHÔNG hề thấy bia mộ — vẫn giữ nguyên giá trị THẬT cũ, y hệt cách nó bị bỏ lỡ một lần GHI bình thường ở q13 bài 7."
locale: vi
track: co-so-du-lieu
module: bia-mo-va-nguoi-don-rac
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.xoa-la-ghi-khong-phai-xoa]
requires: [db.merkle-tree-so-sanh-hash]
concepts: [db.xoa-la-ghi-khong-phai-xoa]
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
q13 ráp xong ghi/đọc/hint/repair/Merkle — một hệ phân tán ĐẦY đủ.
NHƯNG một thao tác chưa hề xuất hiện: XOÁ. Xoá một khoá TRÊN cả ba
owner cùng lúc — CÓ dễ như xoá một biến không?
::::

::::explain{#xoa-theo-quorum}
`xoaTheoQuorum` KHÔNG hề là một cơ chế mới — nó gọi thẳng
`ghiBanGhiTheoQuorum` (ĐÚNG cấu trúc `ghiTheoQuorum` của q13 bài 3),
chỉ khác Ở giá trị ghi: một "bia mộ" (`BanGhi{giaTri:null,thoiGian}`)
— thuật ngữ ĐÃ dùng Ở q04 bài 11 cho tombstone MỘT máy đơn, giờ áp
dụng LẠI cho MỖI owner trong preference list:

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

interface BanGhi { giaTri: number | null; thoiGian: number; }

function taoKhoChoMoiNode(tenCacNode: string[]): Map<string, Map<string, BanGhi>> {
  const kho = new Map<string, Map<string, BanGhi>>();
  for (const ten of tenCacNode) kho.set(ten, new Map<string, BanGhi>());
  return kho;
}

function ghiBanGhiTheoQuorum(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, cacNodeSong: Set<string>, khoa: string, banGhi: BanGhi, rf: number, w: number): boolean {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  let soAck = 0;
  for (const ten of dsSoHuu) {
    if (!cacNodeSong.has(ten)) continue;
    khoCacNode.get(ten)!.set(khoa, banGhi);
    soAck++;
  }
  return soAck >= w;
}

function xoaTheoQuorum(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, cacNodeSong: Set<string>, khoa: string, thoiGian: number, rf: number, w: number): boolean {
  return ghiBanGhiTheoQuorum(vong, khoCacNode, cacNodeSong, khoa, { giaTri: null, thoiGian }, rf, w);
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);

console.log("owners khoa1:", laySachSoHuu(vong, "khoa1", 3));
ghiBanGhiTheoQuorum(vong, kho, new Set(tenCacNode), "khoa1", { giaTri: 100, thoiGian: 1 }, 3, 3);

const xoaOk = xoaTheoQuorum(vong, kho, new Set(["alpha", "beta", "delta"]), "khoa1", 2, 3, 2);
console.log("xoa thanh cong:", xoaOk);
console.log("sau xoa -- alpha:", kho.get("alpha")!.get("khoa1"));
console.log("sau xoa -- gamma (sap luc xoa):", kho.get("gamma")!.get("khoa1"));
console.log("sau xoa -- delta:", kho.get("delta")!.get("khoa1"));
```

```text title=readonly
owners khoa1: [ 'alpha', 'gamma', 'delta' ]
xoa thanh cong: true
sau xoa -- alpha: { giaTri: null, thoiGian: 2 }
sau xoa -- gamma (sap luc xoa): { giaTri: 100, thoiGian: 1 }
sau xoa -- delta: { giaTri: null, thoiGian: 2 }
```

`"khoa1"` thuộc VỀ `[alpha, gamma, delta]` (RF=3, y hệt q13). `gamma`
sập LÚC lệnh xoá gửi đi — `alpha` VÀ `delta` ghi bia mộ THÀNH công
(`soAck=2 >= w=2`), NHƯNG `gamma` không hề THẤY lệnh xoá — nó VẪN giữ
nguyên giá trị THẬT cũ (`100`, `thoiGian:1`), y hệt cách một owner
sập BỎ LỠ một lần GHI bình thường Ở q13 bài 7.
::::

::::example{#xoa-khong-lam-giam-so-owner}
`xoaTheoQuorum` KHÔNG hề "xoá owner nào ra khỏi preference list" —
`laySachSoHuu` vẫn trả VỀ đúng `[alpha, gamma, delta]` y hệt TRƯỚC
khi xoá. Xoá LÀ một GIÁ TRỊ đặc biệt (`null`) ghi TỚI đúng những
owner ĐÓ, không phải một thao tác thay đổi AI sở hữu khoá nào.
::::

::::predict{#doan-gamma-hoi-phuc commitOnce}
`gamma` hồi phục NGAY sau đó (còn sống lại). CHƯA có ai đọc lại
`"khoa1"` — `gamma` trong kho THẬT của nó đang giữ giá trị GÌ?

:::opt{correct}
VẪN LÀ `{ giaTri: 100, thoiGian: 1 }` — hồi phục CHỈ có nghĩa LÀ
`gamma` lại nhận được kết NỐI, KHÔNG tự động đồng bộ LẠI dữ liệu nó
đã bỏ LỠ trong lúc sập
:::

:::opt
`{ giaTri: null, thoiGian: 2 }` — MỘT node hồi phục PHẢI tự động lấy
lại mọi thay đổi nó đã bỏ lỡ TRƯỚC khi được coi LÀ "sống" trở lại
::why
Trực giác NÀY hợp lý cho một số hệ thống có "catch-up" TỰ động khi
khởi động lại — nhưng KHÔNG hề có cơ chế NÀO như vậy trong
`xoaTheoQuorum`/`ghiBanGhiTheoQuorum`.

Chỗ lệch: "hồi phục" Ở đây CHỈ đơn giản LÀ `gamma` xuất hiện lại
trong `cacNodeSong` Ở lần GỌI tiếp theo — bản THÂN việc hồi phục
không hề GHI hay ĐỌC gì cả. Kho thật của `gamma` giữ nguyên NHỮNG gì
nó có TRƯỚC khi sập, cho tới khi có một THAO tác ghi/đọc THẬT chạm
tới nó (hint, q13 bài 7-8; hoặc read repair, q13 bài 9).
::
:::
::::

::::code{#viet_xoa_theo_quorum}
Hoàn thiện `xoaTheoQuorum` — gọi lại `ghiBanGhiTheoQuorum` với một
bia mộ (`giaTri: null`) mang đúng `thoiGian` được truyền vào.

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

interface BanGhi { giaTri: number | null; thoiGian: number; }

function taoKhoChoMoiNode(tenCacNode: string[]): Map<string, Map<string, BanGhi>> {
  const kho = new Map<string, Map<string, BanGhi>>();
  for (const ten of tenCacNode) kho.set(ten, new Map<string, BanGhi>());
  return kho;
}

function ghiBanGhiTheoQuorum(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, cacNodeSong: Set<string>, khoa: string, banGhi: BanGhi, rf: number, w: number): boolean {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  let soAck = 0;
  for (const ten of dsSoHuu) {
    if (!cacNodeSong.has(ten)) continue;
    khoCacNode.get(ten)!.set(khoa, banGhi);
    soAck++;
  }
  return soAck >= w;
}

function xoaTheoQuorum(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, cacNodeSong: Set<string>, khoa: string, thoiGian: number, rf: number, w: number): boolean {
  return ___;
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
ghiBanGhiTheoQuorum(vong, kho, new Set(tenCacNode), "khoa1", { giaTri: 100, thoiGian: 1 }, 3, 3);
console.log(xoaTheoQuorum(vong, kho, new Set(["alpha", "beta", "delta"]), "khoa1", 2, 3, 2));
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

interface BanGhi { giaTri: number | null; thoiGian: number; }

function taoKhoChoMoiNode(tenCacNode: string[]): Map<string, Map<string, BanGhi>> {
  const kho = new Map<string, Map<string, BanGhi>>();
  for (const ten of tenCacNode) kho.set(ten, new Map<string, BanGhi>());
  return kho;
}

function ghiBanGhiTheoQuorum(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, cacNodeSong: Set<string>, khoa: string, banGhi: BanGhi, rf: number, w: number): boolean {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  let soAck = 0;
  for (const ten of dsSoHuu) {
    if (!cacNodeSong.has(ten)) continue;
    khoCacNode.get(ten)!.set(khoa, banGhi);
    soAck++;
  }
  return soAck >= w;
}

function xoaTheoQuorum(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, cacNodeSong: Set<string>, khoa: string, thoiGian: number, rf: number, w: number): boolean {
  return ghiBanGhiTheoQuorum(vong, khoCacNode, cacNodeSong, khoa, { giaTri: null, thoiGian }, rf, w);
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
ghiBanGhiTheoQuorum(vong, kho, new Set(tenCacNode), "khoa1", { giaTri: 100, thoiGian: 1 }, 3, 3);
console.log(xoaTheoQuorum(vong, kho, new Set(["alpha", "beta", "delta"]), "khoa1", 2, 3, 2));
```

```typescript title=test
const kho2 = taoKhoChoMoiNode(tenCacNode);
ghiBanGhiTheoQuorum(vong, kho2, new Set(tenCacNode), "khoa1", { giaTri: 100, thoiGian: 1 }, 3, 3);
const ok2 = xoaTheoQuorum(vong, kho2, new Set(["alpha", "beta", "delta"]), "khoa1", 2, 3, 2);
if (ok2 !== true) throw new Error("gamma sap nhung alpha+delta van ack duoc, W=2 phai thanh cong");
if (kho2.get("alpha")!.get("khoa1")?.giaTri !== null) throw new Error("alpha phai co bia mo (giaTri=null) sau khi xoa");
if (kho2.get("alpha")!.get("khoa1")?.thoiGian !== 2) throw new Error("bia mo cua alpha phai mang dung thoiGian=2");
if (kho2.get("gamma")!.get("khoa1")?.giaTri !== 100) throw new Error("gamma sap luc xoa -- van phai giu gia tri CU 100, khong thay bia mo");
if (kho2.get("delta")!.get("khoa1")?.giaTri !== null) throw new Error("delta phai co bia mo sau khi xoa");

const kho3 = taoKhoChoMoiNode(tenCacNode);
ghiBanGhiTheoQuorum(vong, kho3, new Set(tenCacNode), "khoa1", { giaTri: 100, thoiGian: 1 }, 3, 3);
const ok3 = xoaTheoQuorum(vong, kho3, new Set(["alpha"]), "khoa1", 2, 3, 2);
if (ok3 !== false) throw new Error("chi 1/3 owner song (alpha), W=2 phai that bai");
```

:::hints
- kind: attention
  body: "Goi lai ghiBanGhiTheoQuorum, truyen { giaTri: null, thoiGian } lam ban ghi -- mot dong."
- kind: strategy
  body: "return ghiBanGhiTheoQuorum(vong, khoCacNode, cacNodeSong, khoa, { giaTri: null, thoiGian }, rf, w);"
- kind: one-line
  body: "return ghiBanGhiTheoQuorum(vong, khoCacNode, cacNodeSong, khoa, { giaTri: null, thoiGian }, rf, w);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`gamma` giữ giá trị CŨ, không hề biết mình đang giữ dữ liệu ĐÃ bị
xoá Ở nơi khác. Nếu SAU đó có ai đọc VÀ read repair chạy — chuyện GÌ
xảy ra?
::::

::::reflect{#nghi-lai}
`xoaTheoQuorum` không phải một cơ chế MỚI — nó LÀ `ghiTheoQuorum`
(q13) VIẾT một giá trị đặc biệt. Điều nàY có nghĩa MỌI thứ đã học VỀ
ghi (owner sập thì bỏ lỡ, hint có thể bù, W đủ thì vẫn "thành công")
ĐỀU áp dụng NGUYÊN cho xoá. NHƯNG có một điểm khác biệt SỐNG còn: một
lần ghi bị bỏ lỡ chỉ khiến DỮ liệu "cũ hơn một chút" — một lần XOÁ bị
bỏ lỡ khiến một owner giữ dữ liệu mà CẢ hệ thống coi LÀ "đã không còn
tồn tại". Điều gì xảy ra khi node ĐÓ (`gamma`) tham gia VÀO một lần
đọc?
::::

::::checkpoint{mastery=0.8}
::::
