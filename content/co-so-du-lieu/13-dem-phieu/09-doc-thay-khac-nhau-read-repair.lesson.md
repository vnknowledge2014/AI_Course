---
id: co-so-du-lieu.dem-phieu.doc-thay-khac-nhau-read-repair
title: "Đọc thấy khác nhau — read repair"
summary: "docVoiReadRepair thu thập BanGhi{giaTri,thoiGian} từ CÁC owner đang sống, dùng banMoiNhat (bài 6) chọn bản MỚI nhất, RỒI ghi đè lại NGAY những owner nào có thoiGian NHỎ hơn -- sửa lệch ngay trong lúc đọc, không cần chờ tiến trình nền riêng. Với alpha/gamma giữ bản cũ (thoiGian=1) và delta giữ bản mới (thoiGian=2), một lần đọc R=3 trả về giá trị mới VÀ tự sửa alpha/gamma. Hai bản HOÀ thoiGian nhưng khác giaTri (xung đột thật) thì KHÔNG bản nào bị coi là cũ -- không sửa gì, vì không có cách phân định 'đúng' nếu chỉ dựa vào thời gian."
locale: vi
track: co-so-du-lieu
module: dem-phieu
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.doc-thay-khac-nhau-read-repair]
requires: [db.phat-lai-hint-khi-node-hoi-phuc]
concepts: [db.doc-thay-khac-nhau-read-repair]
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
Hint (bài 7-8) sửa đúng LỖI "owner sập lúc ghi". Còn owner CHỈ đơn
giản chậm, KHÔNG hề sập — không AI gửi hint cho NÓ. Nó vẫn giữ bản
CŨ mãi mãi, TRỪ khi có ai đó phát hiện lúc ĐỌC.
::::

::::explain{#doc-voi-read-repair}
`docVoiReadRepair` thu thập `BanGhi{giaTri, thoiGian}` từ CÁC owner
đang sống, dùng `banMoiNhat` (bài 6) chọn bản MỚI nhất — RỒI ghi đè
NGAY những owner nào đang giữ bản CŨ hơn (`thoiGian` nhỏ hơn). Sửa
xảy ra NGAY trong lúc đọc, không cần một tiến TRÌNH nền riêng:

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

interface BanGhi { giaTri: number; thoiGian: number; }

function banMoiNhat(cacBanGhi: BanGhi[]): BanGhi {
  let ketQua = cacBanGhi[0]!;
  for (const bg of cacBanGhi) if (bg.thoiGian > ketQua.thoiGian) ketQua = bg;
  return ketQua;
}

function taoKhoChoMoiNode(tenCacNode: string[]): Map<string, Map<string, BanGhi>> {
  const kho = new Map<string, Map<string, BanGhi>>();
  for (const ten of tenCacNode) kho.set(ten, new Map<string, BanGhi>());
  return kho;
}

function docVoiReadRepair(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, cacNodeSong: Set<string>, khoa: string, rf: number, r: number): number | undefined {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  const phanHoi: { ten: string; banGhi: BanGhi }[] = [];
  for (const ten of dsSoHuu) {
    if (!cacNodeSong.has(ten)) continue;
    const bg = khoCacNode.get(ten)!.get(khoa);
    if (bg !== undefined) phanHoi.push({ ten, banGhi: bg });
  }
  if (phanHoi.length < r) return undefined;
  const moiNhat = banMoiNhat(phanHoi.map((p) => p.banGhi));
  for (const p of phanHoi) {
    if (p.banGhi.thoiGian < moiNhat.thoiGian) khoCacNode.get(p.ten)!.set(khoa, moiNhat);
  }
  return moiNhat.giaTri;
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
kho.get("alpha")!.set("khoa1", { giaTri: 100, thoiGian: 1 });
kho.get("gamma")!.set("khoa1", { giaTri: 100, thoiGian: 1 });
kho.get("delta")!.set("khoa1", { giaTri: 999, thoiGian: 2 }); // delta co ban MOI hon

console.log("doc R=3:", docVoiReadRepair(vong, kho, new Set(tenCacNode), "khoa1", 3, 3));
console.log("sau doc -- alpha:", kho.get("alpha")!.get("khoa1"));
console.log("sau doc -- gamma:", kho.get("gamma")!.get("khoa1"));
```

```text title=readonly
doc R=3: 999
sau doc -- alpha: { giaTri: 999, thoiGian: 2 }
sau doc -- gamma: { giaTri: 999, thoiGian: 2 }
```

`alpha` VÀ `gamma` giữ bản `thoiGian=1` (cũ HƠN); `delta` giữ bản
`thoiGian=2` (mới nhất). Đọc trả VỀ đúng `999` (giá TRỊ mới) — VÀ
NGAY lập tức ghi đè `alpha`, `gamma` VỀ bản mới. Lần đọc TIẾP theo,
dù `delta` sập, `alpha` HOẶC `gamma` một mình cũng ĐÃ đủ trả lời
đúng — read repair KHÔNG chỉ trả lời đúng LẦN này, mà còn "chữa"
LUÔN cho những lần SAU.
::::

::::example{#hoa-thoi-gian-khong-sua}
Nếu BA bản ghi hoà `thoiGian` NHƯNG khác `giaTri` (xung đột THẬT —
BA lần ghi gần như ĐỒNG thời tới BA owner khác nhau, chưa kịp thấy
NHAU) — `banMoiNhat` chọn bản GẶP đầu tiên (bài 6), nhưng điều kiện
`p.banGhi.thoiGian < moiNhat.thoiGian` KHÔNG đúng VỚI bất kỳ bản nào
(TẤT cả cùng `thoiGian`), NÊN `docVoiReadRepair` KHÔNG sửa owner
NÀO — mỗi owner vẫn giữ NGUYÊN giá trị của chính NÓ. Đây LÀ lựa chọn
cố Ý: chỉ timestamp KHÔNG đủ để phân định "đúng" khi thật SỰ hoà,
nên hệ thống thà KHÔNG sửa còn hơn sửa SAI.
::::

::::predict{#doan-doc-lai-sau-khi-sua commitOnce}
NGAY sau lần đọc Ở ví DỤ readonly (`alpha`, `gamma` đã được sửa VỀ
`999`), đọc LẠI `"khoa1"` một lần NỮA với `R=2`, lúc NÀY `delta` đã
sập (chỉ `alpha`, `beta`, `gamma` còn sống). Kết quả LÀ gì?

:::opt{correct}
Vẫn LÀ `999` — `alpha` VÀ `gamma` đã được read repair sửa Ở LẦN đọc
trước, giờ TỰ chúng cũng giữ đúng bản MỚI nhất, không cần `delta`
nữa
:::

:::opt
`undefined` — `delta` (node giữ bản GỐC mới nhất) sập thì KHÔNG ai
còn biết giá trị đúng LÀ gì
::why
Trực giác NÀY đúng NẾU chưa từng có lần đọc NÀO trước đó sửa lại
`alpha`/`gamma` — khi ĐÓ `delta` thật sự LÀ nguồn DUY nhất giữ bản
mới.

Chỗ lệch: câu hỏi ĐÃ nêu rõ đây LÀ lần đọc THỨ HAI, SAU khi read
repair Ở lần đọc ĐẦU đã ghi đè `alpha` VÀ `gamma` về đúng
`{giaTri:999, thoiGian:2}`. Read repair KHÔNG chỉ trả lời đúng một
lần rồi THÔI — nó thực SỰ cập nhật kho THẬT của owner, nên những lần
đọc SAU không còn phụ thuộc và `delta` nữa.
::
:::
::::

::::code{#viet_doc_voi_read_repair}
Hoàn thiện `docVoiReadRepair` — VỚI mỗi phản hồi CÓ `thoiGian` nhỏ
hơn bản mới NHẤT, ghi đè lại kho của NÓ.

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

interface BanGhi { giaTri: number; thoiGian: number; }

function banMoiNhat(cacBanGhi: BanGhi[]): BanGhi {
  let ketQua = cacBanGhi[0]!;
  for (const bg of cacBanGhi) if (bg.thoiGian > ketQua.thoiGian) ketQua = bg;
  return ketQua;
}

function taoKhoChoMoiNode(tenCacNode: string[]): Map<string, Map<string, BanGhi>> {
  const kho = new Map<string, Map<string, BanGhi>>();
  for (const ten of tenCacNode) kho.set(ten, new Map<string, BanGhi>());
  return kho;
}

function docVoiReadRepair(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, cacNodeSong: Set<string>, khoa: string, rf: number, r: number): number | undefined {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  const phanHoi: { ten: string; banGhi: BanGhi }[] = [];
  for (const ten of dsSoHuu) {
    if (!cacNodeSong.has(ten)) continue;
    const bg = khoCacNode.get(ten)!.get(khoa);
    if (bg !== undefined) phanHoi.push({ ten, banGhi: bg });
  }
  if (phanHoi.length < r) return undefined;
  const moiNhat = banMoiNhat(phanHoi.map((p) => p.banGhi));
  for (const p of phanHoi) {
    ___
  }
  return moiNhat.giaTri;
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
kho.get("alpha")!.set("khoa1", { giaTri: 100, thoiGian: 1 });
kho.get("gamma")!.set("khoa1", { giaTri: 100, thoiGian: 1 });
kho.get("delta")!.set("khoa1", { giaTri: 999, thoiGian: 2 });
console.log(docVoiReadRepair(vong, kho, new Set(tenCacNode), "khoa1", 3, 3));
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

interface BanGhi { giaTri: number; thoiGian: number; }

function banMoiNhat(cacBanGhi: BanGhi[]): BanGhi {
  let ketQua = cacBanGhi[0]!;
  for (const bg of cacBanGhi) if (bg.thoiGian > ketQua.thoiGian) ketQua = bg;
  return ketQua;
}

function taoKhoChoMoiNode(tenCacNode: string[]): Map<string, Map<string, BanGhi>> {
  const kho = new Map<string, Map<string, BanGhi>>();
  for (const ten of tenCacNode) kho.set(ten, new Map<string, BanGhi>());
  return kho;
}

function docVoiReadRepair(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, cacNodeSong: Set<string>, khoa: string, rf: number, r: number): number | undefined {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  const phanHoi: { ten: string; banGhi: BanGhi }[] = [];
  for (const ten of dsSoHuu) {
    if (!cacNodeSong.has(ten)) continue;
    const bg = khoCacNode.get(ten)!.get(khoa);
    if (bg !== undefined) phanHoi.push({ ten, banGhi: bg });
  }
  if (phanHoi.length < r) return undefined;
  const moiNhat = banMoiNhat(phanHoi.map((p) => p.banGhi));
  for (const p of phanHoi) {
    if (p.banGhi.thoiGian < moiNhat.thoiGian) khoCacNode.get(p.ten)!.set(khoa, moiNhat);
  }
  return moiNhat.giaTri;
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
kho.get("alpha")!.set("khoa1", { giaTri: 100, thoiGian: 1 });
kho.get("gamma")!.set("khoa1", { giaTri: 100, thoiGian: 1 });
kho.get("delta")!.set("khoa1", { giaTri: 999, thoiGian: 2 });
console.log(docVoiReadRepair(vong, kho, new Set(tenCacNode), "khoa1", 3, 3));
```

```typescript title=test
const kho2 = taoKhoChoMoiNode(tenCacNode);
kho2.get("alpha")!.set("khoa1", { giaTri: 100, thoiGian: 1 });
kho2.get("gamma")!.set("khoa1", { giaTri: 100, thoiGian: 1 });
kho2.get("delta")!.set("khoa1", { giaTri: 999, thoiGian: 2 });

const gtDoc = docVoiReadRepair(vong, kho2, new Set(tenCacNode), "khoa1", 3, 3);
if (gtDoc !== 999) throw new Error("doc phai tra ve gia tri MOI nhat (999), khong phai gia tri da so (100)");
if (kho2.get("alpha")!.get("khoa1")!.giaTri !== 999) throw new Error("read repair phai sua alpha ve ban ghi moi nhat (999)");
if (kho2.get("alpha")!.get("khoa1")!.thoiGian !== 2) throw new Error("read repair phai cap nhat CA thoiGian cua alpha thanh 2");
if (kho2.get("gamma")!.get("khoa1")!.giaTri !== 999) throw new Error("read repair phai sua gamma ve ban ghi moi nhat (999)");
if (kho2.get("delta")!.get("khoa1")!.thoiGian !== 2) throw new Error("delta (da co ban moi nhat tu truoc) khong duoc doi");

const kho3 = taoKhoChoMoiNode(tenCacNode);
kho3.get("alpha")!.set("khoa1", { giaTri: 50, thoiGian: 5 });
kho3.get("gamma")!.set("khoa1", { giaTri: 50, thoiGian: 5 });
kho3.get("delta")!.set("khoa1", { giaTri: 50, thoiGian: 5 });
const gtDongBo = docVoiReadRepair(vong, kho3, new Set(tenCacNode), "khoa1", 3, 3);
if (gtDongBo !== 50) throw new Error("da dong bo thi doc phai tra ve 50");

const kho4 = taoKhoChoMoiNode(tenCacNode);
kho4.get("alpha")!.set("khoa1", { giaTri: 100, thoiGian: 1 });
if (docVoiReadRepair(vong, kho4, new Set(["alpha", "beta"]), "khoa1", 3, 2) !== undefined) throw new Error("chi 1/2 owner con song va co du lieu, R=2 phai that bai");

const kho5 = taoKhoChoMoiNode(tenCacNode);
kho5.get("alpha")!.set("khoa1", { giaTri: 100, thoiGian: 5 });
kho5.get("gamma")!.set("khoa1", { giaTri: 200, thoiGian: 5 });
kho5.get("delta")!.set("khoa1", { giaTri: 300, thoiGian: 5 });
docVoiReadRepair(vong, kho5, new Set(tenCacNode), "khoa1", 3, 3);
if (kho5.get("gamma")!.get("khoa1")!.giaTri !== 200) throw new Error("hoa thoiGian: gamma KHONG duoc coi la cu, gia tri phai giu nguyen 200");
if (kho5.get("delta")!.get("khoa1")!.giaTri !== 300) throw new Error("hoa thoiGian: delta KHONG duoc coi la cu, gia tri phai giu nguyen 300");
```

:::hints
- kind: attention
  body: "Chi ghi de khi thoiGian cua phan hoi NHO HON thoiGian cua ban moi nhat -- mot dong."
- kind: strategy
  body: "if (p.banGhi.thoiGian < moiNhat.thoiGian) khoCacNode.get(p.ten)!.set(khoa, moiNhat);"
- kind: one-line
  body: "if (p.banGhi.thoiGian < moiNhat.thoiGian) khoCacNode.get(p.ten)!.set(khoa, moiNhat);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "999"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Read repair chỉ sửa những khoá CÓ AI đọc lại. Một khoá KHÔNG bao giờ
được đọc thì SAO — nó lệch mãi MÀ không ai hay biết?
::::

::::reflect{#nghi-lai}
`docVoiReadRepair` LÀ read repair THẬT — không phải một khái niệm
trừu TƯỢNG, mà đúng BỐN dòng: thu thập, chọn bản MỚI nhất bằng
`banMoiNhat` (bài 6), so SÁNH, ghi đè. NÓ giải quyết đúng loại lỗi
"chậm cập NHẬT" mà hint (bài 7-8) không CHẠM tới. Nhưng CẢ hai cơ chế
NÀY đều CÓ một điểm mù CHUNG: chúng chỉ hoạt động khi CÓ một thao tác
(ghi hoặc đọc) THỰC sự đi qua khoá LỆCH đó. Một khoá không ai đụng
tới thì KHÔNG bao giờ được sửa — kể cả bằng hai cơ CHẾ này.
::::

::::checkpoint{mastery=0.85}
::::
