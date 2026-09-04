---
id: co-so-du-lieu.bia-mo-va-nguoi-don-rac.bia-mo-hoi-sinh-du-lieu
title: "Bia mộ hồi sinh dữ liệu — zombie"
summary: "docBanGhiVoiReadRepair (mở rộng docVoiReadRepair q13 bài 9: MỌI owner còn sống đều tính là một phản hồi, kể cả khi chưa từng thấy khoá) chọn bản ghi mới nhất theo thoiGian. Nếu bia mộ ở alpha/delta bị xoá SẠCH (purge) quá sớm trong khi gamma (từng sập lúc xoá) vẫn giữ giá trị THẬT cũ, đọc lại thấy đúng giá trị cũ đó 'sống lại' — và read repair còn GHI nó ngược vào alpha/delta, biến zombie thành vĩnh viễn."
locale: vi
track: co-so-du-lieu
module: bia-mo-va-nguoi-don-rac
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.bia-mo-hoi-sinh-du-lieu]
requires: [db.xoa-la-ghi-khong-phai-xoa]
concepts: [db.bia-mo-hoi-sinh-du-lieu]
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
`gamma` sập lúc xoá (bài trước) — vẫn giữ giá trị THẬT cũ. Nếu bia mộ
Ở `alpha`/`delta` bị XOÁ SẠCH (dọn hẳn khỏi kho, không phải chỉ đánh
dấu) trước khi `gamma` kịp biết chuyện gì đã xảy ra — chuyện GÌ xảy
ra khi có người ĐỌC?
::::

::::explain{#doc-voi-read-repair-va-bia-mo}
`docBanGhiVoiReadRepair` mở RỘNG `docVoiReadRepair` (q13 bài 9) theo
MỘT điểm quan trọng: Ở q13, chỉ owner CÓ dữ liệu mới tính LÀ "phản
hồi". Ở đây, MỌI owner còn sống ĐỀU tính LÀ một phản hồi — kể cả khi
nó CHƯA từng thấy khoá NÀY — vì phân biệt "chưa từng thấy" VÀ "bia mộ
đã bị xoá sạch" chính LÀ trọng tâm của vấn đề NÀY:

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

function xoaBiaMoQuaSom(khoCacNode: Map<string, Map<string, BanGhi>>, ten: string, khoa: string): void {
  khoCacNode.get(ten)!.delete(khoa);
}

function banMoiNhatBanGhi(cacBanGhi: BanGhi[]): BanGhi {
  let ketQua = cacBanGhi[0]!;
  for (const bg of cacBanGhi) if (bg.thoiGian > ketQua.thoiGian) ketQua = bg;
  return ketQua;
}

function docBanGhiVoiReadRepair(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, cacNodeSong: Set<string>, khoa: string, rf: number, r: number): BanGhi | undefined {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  const conSong = dsSoHuu.filter((ten) => cacNodeSong.has(ten));
  if (conSong.length < r) return undefined;
  const coDuLieu: BanGhi[] = [];
  for (const ten of conSong) {
    const bg = khoCacNode.get(ten)!.get(khoa);
    if (bg !== undefined) coDuLieu.push(bg);
  }
  if (coDuLieu.length === 0) return undefined;
  const moiNhat = banMoiNhatBanGhi(coDuLieu);
  for (const ten of conSong) {
    const bg = khoCacNode.get(ten)!.get(khoa);
    if (bg === undefined || bg.thoiGian < moiNhat.thoiGian) khoCacNode.get(ten)!.set(khoa, moiNhat);
  }
  return moiNhat;
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);

ghiBanGhiTheoQuorum(vong, kho, new Set(tenCacNode), "khoa1", { giaTri: 100, thoiGian: 1 }, 3, 3);
xoaTheoQuorum(vong, kho, new Set(["alpha", "beta", "delta"]), "khoa1", 2, 3, 2); // gamma sap luc xoa

// purge QUA SOM tren alpha va delta (gamma van con gia tri cu 100, t=1)
xoaBiaMoQuaSom(kho, "alpha", "khoa1");
xoaBiaMoQuaSom(kho, "delta", "khoa1");
console.log("truoc doc -- alpha:", String(kho.get("alpha")!.get("khoa1")));
console.log("truoc doc -- gamma:", kho.get("gamma")!.get("khoa1"));
console.log("truoc doc -- delta:", String(kho.get("delta")!.get("khoa1")));

const ketQuaDoc = docBanGhiVoiReadRepair(vong, kho, new Set(tenCacNode), "khoa1", 3, 3);
console.log("doc lai (R=3):", ketQuaDoc);
console.log("sau doc -- alpha:", kho.get("alpha")!.get("khoa1"));
console.log("sau doc -- delta:", kho.get("delta")!.get("khoa1"));
```

```text title=readonly
truoc doc -- alpha: undefined
truoc doc -- gamma: { giaTri: 100, thoiGian: 1 }
truoc doc -- delta: undefined
doc lai (R=3): { giaTri: 100, thoiGian: 1 }
sau doc -- alpha: { giaTri: 100, thoiGian: 1 }
sau doc -- delta: { giaTri: 100, thoiGian: 1 }
```

Sau khi purge, `alpha` VÀ `delta` KHÔNG còn dấu vết GÌ về `"khoa1"` —
với `docBanGhiVoiReadRepair`, chúng vẫn LÀ hai phản hồi HỢP lệ (còn
sống, được hỏi), nhưng KHÔNG có dữ liệu. `gamma` LÀ owner DUY nhất
CÓ dữ liệu — giá trị `100` "thắng" (bản ghi mới NHẤT trong SỐ những
bản ghi TỒN tại) VÀ read repair GHI nó ngược vào `alpha`, `delta` —
dữ liệu ĐÃ xoá vừa SỐNG lại, VÀ giờ VĨNH viễn (ba owner đều đồng ý).
::::

::::example{#khong-phai-loi-cua-read-repair}
Read repair KHÔNG hề sai — nó LÀM đúng chức năng của mình: hoà giải
sự KHÁC biệt giữa các owner. Vấn đề nằm Ở BƯỚC purge — `xoaBiaMoQuaSom`
huỷ THÔNG tin "khoá này đã bị xoá" TRƯỚC khi mọi owner (bao gồm
`gamma`, từng sập) kịp NHÌN thấy nó. Một khi bia mộ biến mất, MỌI cơ
chế hoà giải (read repair, anti-entropy) không CÒN cách nào phân biệt
"chưa từng CÓ" với "đã từng có RỒI bị xoá SẠCH" — cả hai đều trông
GIỐNG hệt nhau: KHÔNG có gì Ở đó.
::::

::::predict{#doan-khong-purge commitOnce}
CÙNG kịch bản (`gamma` sập lúc xoá), NHƯNG lần NÀY KHÔNG gọi
`xoaBiaMoQuaSom` — bia mộ Ở `alpha`/`delta` VẪN còn nguyên. Đọc LẠI
`"khoa1"` VỚI `R=3` — kết QUẢ LÀ gì?

:::opt{correct}
`{ giaTri: null, thoiGian: 2 }` — bia mộ (t=2) MỚI hơn giá trị CŨ của
gamma (t=1), NÊN `banMoiNhatBanGhi` chọn ĐÚNG bia mộ; read repair còn
GHI bia mộ đó VÀO gamma, "chữa" nó thay VÌ hồi sinh zombie
:::

:::opt
VẪN LÀ `{ giaTri: 100, thoiGian: 1 }` — gamma LÀ owner DUY nhất từng
sập, hệ thống ưu tiên node "ổn định hơn"
::why
Không CÓ khái niệm "node ổn định hơn" Ở ĐÂU trong
`docBanGhiVoiReadRepair` — nó CHỈ so sánh `thoiGian`, không hề biết
(hay quan TÂM) owner nào từng sập.

Chỗ lệch: KHI bia mộ CÒN nguyên Ở `alpha`/`delta`, `coDuLieu` chứa BA
bản ghi: `{100,t1}` (gamma), `{null,t2}` (alpha), `{null,t2}` (delta)
— `banMoiNhatBanGhi` chọn `thoiGian` LỚN nhất LÀ `2`, tức LÀ bia mộ.
Chỉ khi bia mộ đã bị PURGE mất (như readonly Ở trên) thì `100` mới
"thắng", VÌ lúc đó nó LÀ bản ghi DUY nhất còn tồn tại.
::
:::
::::

::::code{#viet_doc_ban_ghi_voi_read_repair}
Hoàn thiện `docBanGhiVoiReadRepair` — VỚI mỗi owner còn sống mà bản
ghi CỦA nó thiếu hoặc CŨ hơn bản mới nhất, ghi đè LẠI bằng bản mới
nhất.

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

function banMoiNhatBanGhi(cacBanGhi: BanGhi[]): BanGhi {
  let ketQua = cacBanGhi[0]!;
  for (const bg of cacBanGhi) if (bg.thoiGian > ketQua.thoiGian) ketQua = bg;
  return ketQua;
}

function docBanGhiVoiReadRepair(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, cacNodeSong: Set<string>, khoa: string, rf: number, r: number): BanGhi | undefined {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  const conSong = dsSoHuu.filter((ten) => cacNodeSong.has(ten));
  if (conSong.length < r) return undefined;
  const coDuLieu: BanGhi[] = [];
  for (const ten of conSong) {
    const bg = khoCacNode.get(ten)!.get(khoa);
    if (bg !== undefined) coDuLieu.push(bg);
  }
  if (coDuLieu.length === 0) return undefined;
  const moiNhat = banMoiNhatBanGhi(coDuLieu);
  for (const ten of conSong) {
    const bg = khoCacNode.get(ten)!.get(khoa);
    ___
  }
  return moiNhat;
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
ghiBanGhiTheoQuorum(vong, kho, new Set(tenCacNode), "khoa1", { giaTri: 100, thoiGian: 1 }, 3, 3);
console.log(docBanGhiVoiReadRepair(vong, kho, new Set(tenCacNode), "khoa1", 3, 3));
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

function banMoiNhatBanGhi(cacBanGhi: BanGhi[]): BanGhi {
  let ketQua = cacBanGhi[0]!;
  for (const bg of cacBanGhi) if (bg.thoiGian > ketQua.thoiGian) ketQua = bg;
  return ketQua;
}

function docBanGhiVoiReadRepair(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, cacNodeSong: Set<string>, khoa: string, rf: number, r: number): BanGhi | undefined {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  const conSong = dsSoHuu.filter((ten) => cacNodeSong.has(ten));
  if (conSong.length < r) return undefined;
  const coDuLieu: BanGhi[] = [];
  for (const ten of conSong) {
    const bg = khoCacNode.get(ten)!.get(khoa);
    if (bg !== undefined) coDuLieu.push(bg);
  }
  if (coDuLieu.length === 0) return undefined;
  const moiNhat = banMoiNhatBanGhi(coDuLieu);
  for (const ten of conSong) {
    const bg = khoCacNode.get(ten)!.get(khoa);
    if (bg === undefined || bg.thoiGian < moiNhat.thoiGian) khoCacNode.get(ten)!.set(khoa, moiNhat);
  }
  return moiNhat;
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
ghiBanGhiTheoQuorum(vong, kho, new Set(tenCacNode), "khoa1", { giaTri: 100, thoiGian: 1 }, 3, 3);
console.log(docBanGhiVoiReadRepair(vong, kho, new Set(tenCacNode), "khoa1", 3, 3));
```

```typescript title=test
function xoaBiaMoQuaSomTest(khoCacNode: Map<string, Map<string, BanGhi>>, ten: string, khoa: string): void {
  khoCacNode.get(ten)!.delete(khoa);
}

const kho2 = taoKhoChoMoiNode(tenCacNode);
ghiBanGhiTheoQuorum(vong, kho2, new Set(tenCacNode), "khoa1", { giaTri: 100, thoiGian: 1 }, 3, 3);
if (docBanGhiVoiReadRepair(vong, kho2, new Set(tenCacNode), "khoa1", 3, 3)?.giaTri !== 100) throw new Error("chua xoa gi -- doc lai phai ra dung 100");

const kho3 = taoKhoChoMoiNode(tenCacNode);
ghiBanGhiTheoQuorum(vong, kho3, new Set(tenCacNode), "khoa1", { giaTri: 100, thoiGian: 1 }, 3, 3);
ghiBanGhiTheoQuorum(vong, kho3, new Set(["alpha", "beta", "delta"]), "khoa1", { giaTri: null, thoiGian: 2 }, 3, 2);
const ketQuaConBiaMo = docBanGhiVoiReadRepair(vong, kho3, new Set(tenCacNode), "khoa1", 3, 3);
if (ketQuaConBiaMo?.giaTri !== null) throw new Error("bia mo con nguyen -- doc lai phai ra bia mo (null), khong phai gia tri cu");
if (kho3.get("gamma")!.get("khoa1")?.giaTri !== null) throw new Error("read repair phai chua gamma bang bia mo, khong con gia tri cu 100");

const kho4 = taoKhoChoMoiNode(tenCacNode);
ghiBanGhiTheoQuorum(vong, kho4, new Set(tenCacNode), "khoa1", { giaTri: 100, thoiGian: 1 }, 3, 3);
ghiBanGhiTheoQuorum(vong, kho4, new Set(["alpha", "beta", "delta"]), "khoa1", { giaTri: null, thoiGian: 2 }, 3, 2);
xoaBiaMoQuaSomTest(kho4, "alpha", "khoa1");
xoaBiaMoQuaSomTest(kho4, "delta", "khoa1");
const ketQuaZombie = docBanGhiVoiReadRepair(vong, kho4, new Set(tenCacNode), "khoa1", 3, 3);
if (ketQuaZombie?.giaTri !== 100) throw new Error("sau khi purge bia mo o alpha/delta, doc lai phai 'song lai' gia tri cu 100 (zombie)");
if (kho4.get("alpha")!.get("khoa1")?.giaTri !== 100) throw new Error("read repair phai ghi nguoc zombie vao alpha");

// hoa thoiGian: alpha va gamma cung thoiGian=5, gia tri khac nhau -- ban GAP DAU TIEN (alpha, dung thu tu preference list) phai thang
const kho5 = taoKhoChoMoiNode(tenCacNode);
kho5.get("alpha")!.set("khoa1", { giaTri: 100, thoiGian: 5 });
kho5.get("gamma")!.set("khoa1", { giaTri: 200, thoiGian: 5 });
const ketQuaHoa = docBanGhiVoiReadRepair(vong, kho5, new Set(tenCacNode), "khoa1", 3, 3);
if (ketQuaHoa?.giaTri !== 100) throw new Error("hoa thoiGian: ban GAP DAU TIEN trong preference list (alpha, gia tri 100) phai thang, khong phai gamma (200)");
if (kho5.get("gamma")!.get("khoa1")?.giaTri !== 200) throw new Error("hoa thoiGian: gamma KHONG duoc bi ghi de -- ban ghi cua no van phai giu nguyen gia tri 200, thoiGian 5 (khong thua ke, khong bang nhau)");
```

:::hints
- kind: attention
  body: "Neu bg thieu (undefined) HOAC cu hon moiNhat thi ghi lai bang moiNhat -- mot dong."
- kind: strategy
  body: "if (bg === undefined || bg.thoiGian < moiNhat.thoiGian) khoCacNode.get(ten)!.set(khoa, moiNhat);"
- kind: one-line
  body: "if (bg === undefined || bg.thoiGian < moiNhat.thoiGian) khoCacNode.get(ten)!.set(khoa, moiNhat);"
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
Purge quá SỚM biến dữ liệu đã xoá thành zombie SỐNG lại vĩnh viễn.
Vậy purge PHẢI chờ tới KHI nào mới an toàn?
::::

::::reflect{#nghi-lai}
Vấn đề Ở bài NÀY không nằm Ở `docBanGhiVoiReadRepair` — nó LÀM đúng
việc CỦA mình (hoà giải theo `thoiGian`). Vấn đề nằm Ở THỜI ĐIỂM
`xoaBiaMoQuaSom` được GỌI: nó xoá bằng CHỨNG duy nhất ("khoá này đã
bị xoá") TRƯỚC khi mọi owner (kể cả `gamma`, từng sập) kịp nhìn THẤY
nó. Câu hỏi tự nhiên: BAO lâu LÀ đủ để CHẮC chắn mọi owner đã kịp
biết?
::::

::::checkpoint{mastery=0.85}
::::
