---
id: co-so-du-lieu.dem-phieu.node-im-lang-khi-ghi
title: "Node im lặng khi ghi — hinted handoff"
summary: "ghiVoiHint ghi trực tiếp vào owner CÒN sống; owner ĐANG sập được thay bằng một node CÒN sống nhưng KHÔNG phải owner, giữ tạm 'hint' (khoá, giá trị, VÀ tên chủ thật). Với RF=3, gamma sập: alpha/delta ghi trực tiếp, beta (không phải owner) giữ hint thay gamma -- soAck vẫn = 3. Nếu RF bằng đúng số node vật lý (mọi node ĐỀU là owner), một owner sập thì KHÔNG có node nào đủ điều kiện làm hint -- soAck giảm, không có cách 'mượn tạm'."
locale: vi
track: co-so-du-lieu
module: dem-phieu
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.node-im-lang-khi-ghi]
requires: [db.khi-cac-ban-sao-khong-dong-y]
concepts: [db.node-im-lang-khi-ghi]
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
`ghiTheoQuorum` (bài 3) đơn giản BỎ QUA owner đang sập — nếu đủ
`w` phiếu TỪ những owner còn LẠI, ghi vẫn "thành công". Nhưng owner
sập đó sẽ MÃI mãi thiếu dữ liệu, TRỪ khi có cách nào đó GIỮ lại
phần việc CỦA nó.
::::

::::explain{#ghi-voi-hint}
`ghiVoiHint` VẪN ghi trực tiếp VÀO owner còn sống — nhưng VỚI owner
ĐANG sập, nó tìm một node CÒN sống KHÔNG PHẢI owner (`thayThe`), gửi
CHO node đó một "hint": bản GHI kèm TÊN chủ thật (`chuThat`) — hint
sẽ được TRẢ lại đúng chủ khi chủ hồi phục (bài SAU):

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

interface Hint { khoa: string; giaTri: number; chuThat: string; }

function taoHopHint(tenCacNode: string[]): Map<string, Hint[]> {
  const hop = new Map<string, Hint[]>();
  for (const ten of tenCacNode) hop.set(ten, []);
  return hop;
}

function ghiVoiHint(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, hopHint: Map<string, Hint[]>, cacNodeSong: Set<string>, khoa: string, giaTri: number, rf: number): number {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  let soAck = 0;
  for (const chuThat of dsSoHuu) {
    if (cacNodeSong.has(chuThat)) {
      khoCacNode.get(chuThat)!.set(khoa, giaTri);
      soAck++;
      continue;
    }
    const thayThe = [...cacNodeSong].find((n) => !dsSoHuu.includes(n));
    if (thayThe === undefined) continue;
    hopHint.get(thayThe)!.push({ khoa, giaTri, chuThat });
    soAck++;
  }
  return soAck;
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
console.log("owners khoa1:", laySachSoHuu(vong, "khoa1", 3));

const kho = taoKhoChoMoiNode(tenCacNode);
const hop = taoHopHint(tenCacNode);
console.log("soAck:", ghiVoiHint(vong, kho, hop, new Set(["alpha", "beta", "delta"]), "khoa1", 100, 3));
console.log("gamma kho that (sap, khong co gi):", String(kho.get("gamma")!.get("khoa1")));
console.log("beta giu hint:", hop.get("beta"));
```

```text title=readonly
owners khoa1: [ 'alpha', 'gamma', 'delta' ]
soAck: 3
gamma kho that (sap, khong co gi): undefined
beta giu hint: [ { khoa: 'khoa1', giaTri: 100, chuThat: 'gamma' } ]
```

`gamma` (owner) đang SẬP — `beta` (KHÔNG phải owner, nhưng ĐANG
sống) nhận thay MỘT "hint" ghi RÕ `chuThat: "gamma"`. `soAck` vẫn
đếm ĐỦ `3`, dù `gamma` chưa hề nhận được GÌ trực tiếp — hint LÀ lời
hứa "TA đang giữ hộ, sẽ trả LẠI khi ngươi tỉnh".
::::

::::example{#nhieu-hint-cung-mot-node}
Nếu CẢ `gamma` LẪN `delta` cùng sập, `beta` LÀ node còn sống DUY
NHẤT không phải owner — nó nhận HAI hint (một CHO `gamma`, một CHO
`delta`), `soAck` vẫn LÀ `3`. Một node CÓ thể giữ hint thay cho
NHIỀU owner khác nhau CÙNG lúc — hộp hint LÀ một MẢNG, không giới
hạn số lượng.
::::

::::predict{#doan-rf-bang-so-node commitOnce}
`RF=4` (bằng ĐÚNG số node vật lý — MỌI node đều LÀ owner của
`"khoa1"`). `beta` sập. `ghiVoiHint` CÓ tìm được node thay THẾ cho
`beta` không?

:::opt{correct}
KHÔNG — mọi node CÒN sống (`alpha, gamma, delta`) ĐỀU đã LÀ owner
(vì RF=4=số node), NÊN không CÒN node nào "ngoài cuộc" để làm hint;
`soAck` giảm CÒN `3`, KHÔNG đủ `4`
:::

:::opt
CÓ — hệ thống LUÔN tìm được cách "MƯỢN tạm" một node BẤT kỳ, kể cả
khi node đó CŨNG là owner
::why
Trực giác NÀY hợp LÝ nếu hint được PHÉP gửi tới bất KỲ node sống nào
— nhưng `ghiVoiHint` cố Ý CHỈ chọn `thayThe` LÀ node KHÔNG nằm trong
`dsSoHuu`, để tránh MỘT node vừa LÀ owner thật của khoá KHÁC vừa
phải gánh hint — giữ vai TRÒ rõ ràng.

Chỗ lệch: dòng `[...cacNodeSong].find((n) => !dsSoHuu.includes(n))`
CHỈ tìm node KHÔNG có trong preference list. Khi RF bằng đúng số
node vật LÝ, `dsSoHuu` chứa TẤT cả node đang có — mọi `n` trong
`cacNodeSong` ĐỀU nằm trong `dsSoHuu`, nên `find` LUÔN trả về
`undefined`. `beta` sập trong trường hợp NÀY đơn giản LÀ mất một
phiếu, không CÓ hint nào bù lại.
::
:::
::::

::::code{#viet_ghi_voi_hint}
Hoàn thiện `ghiVoiHint` — KHI tìm được node thay THẾ, giữ hint CHO
owner thật.

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

interface Hint { khoa: string; giaTri: number; chuThat: string; }

function taoHopHint(tenCacNode: string[]): Map<string, Hint[]> {
  const hop = new Map<string, Hint[]>();
  for (const ten of tenCacNode) hop.set(ten, []);
  return hop;
}

function ghiVoiHint(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, hopHint: Map<string, Hint[]>, cacNodeSong: Set<string>, khoa: string, giaTri: number, rf: number): number {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  let soAck = 0;
  for (const chuThat of dsSoHuu) {
    if (cacNodeSong.has(chuThat)) {
      khoCacNode.get(chuThat)!.set(khoa, giaTri);
      soAck++;
      continue;
    }
    const thayThe = [...cacNodeSong].find((n) => !dsSoHuu.includes(n));
    if (thayThe === undefined) continue;
    ___
    soAck++;
  }
  return soAck;
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
const hop = taoHopHint(tenCacNode);
console.log(ghiVoiHint(vong, kho, hop, new Set(["alpha", "beta", "delta"]), "khoa1", 100, 3));
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

interface Hint { khoa: string; giaTri: number; chuThat: string; }

function taoHopHint(tenCacNode: string[]): Map<string, Hint[]> {
  const hop = new Map<string, Hint[]>();
  for (const ten of tenCacNode) hop.set(ten, []);
  return hop;
}

function ghiVoiHint(vong: DiemNode[], khoCacNode: Map<string, Map<string, number>>, hopHint: Map<string, Hint[]>, cacNodeSong: Set<string>, khoa: string, giaTri: number, rf: number): number {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  let soAck = 0;
  for (const chuThat of dsSoHuu) {
    if (cacNodeSong.has(chuThat)) {
      khoCacNode.get(chuThat)!.set(khoa, giaTri);
      soAck++;
      continue;
    }
    const thayThe = [...cacNodeSong].find((n) => !dsSoHuu.includes(n));
    if (thayThe === undefined) continue;
    hopHint.get(thayThe)!.push({ khoa, giaTri, chuThat });
    soAck++;
  }
  return soAck;
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
const hop = taoHopHint(tenCacNode);
console.log(ghiVoiHint(vong, kho, hop, new Set(["alpha", "beta", "delta"]), "khoa1", 100, 3));
```

```typescript title=test
const kho2 = taoKhoChoMoiNode(tenCacNode);
const hop2 = taoHopHint(tenCacNode);
const soAck2 = ghiVoiHint(vong, kho2, hop2, new Set(["alpha", "beta", "delta"]), "khoa1", 100, 3);
if (soAck2 !== 3) throw new Error("gamma sap nhung co beta thay the -- soAck van phai la 3");
if (kho2.get("alpha")!.get("khoa1") !== 100) throw new Error("alpha (owner con song) phai duoc ghi truc tiep");
if (kho2.get("gamma")!.get("khoa1") !== undefined) throw new Error("gamma (sap) khong duoc ghi truc tiep");
const hintBeta = hop2.get("beta")!;
if (hintBeta.length !== 1) throw new Error("beta phai nhan dung 1 hint");
if (hintBeta[0]!.chuThat !== "gamma" || hintBeta[0]!.giaTri !== 100) throw new Error("hint phai ghi dung chu that la gamma, gia tri 100");

const kho3 = taoKhoChoMoiNode(tenCacNode);
const hop3 = taoHopHint(tenCacNode);
const soAck3 = ghiVoiHint(vong, kho3, hop3, new Set(["alpha", "beta"]), "khoa1", 200, 3);
if (soAck3 !== 3) throw new Error("gamma+delta cung sap, beta thay the cho ca hai -- soAck van la 3");
if (hop3.get("beta")!.length !== 2) throw new Error("beta phai gom du 2 hint (thay cho gamma va delta)");

const kho4 = taoKhoChoMoiNode(tenCacNode);
const hop4 = taoHopHint(tenCacNode);
const soAck4 = ghiVoiHint(vong, kho4, hop4, new Set(["alpha", "gamma", "delta"]), "khoa1", 999, 4);
if (soAck4 !== 3) throw new Error("RF=4 (moi node deu la owner), beta sap thi KHONG CO node thay the -- soAck phai la 3, khong phai 4");
for (const [, ds] of hop4) if (ds.length !== 0) throw new Error("khong co node thay the thi khong hop hint nao duoc dien");
```

:::hints
- kind: attention
  body: "Da tim duoc node thay the -- them mot hint vao hop hint cua node do, mot dong."
- kind: strategy
  body: "hopHint.get(thayThe)!.push({ khoa, giaTri, chuThat });"
- kind: one-line
  body: "hopHint.get(thayThe)!.push({ khoa, giaTri, chuThat });"
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
Hint đang chờ Ở `beta` — khi `gamma` sống LẠI, làm sao trả hint VỀ
đúng chủ?
::::

::::reflect{#nghi-lai}
`ghiVoiHint` không "sửa" quorum — `soAck` vẫn đếm y HỆT cách bài 3
đã LÀM. Điều thay đổi LÀ: thay VÌ bỏ qua owner sập HOÀN toàn, hệ
thống TÌM một nơi tạm để KHÔNG mất dữ liệu. Đây LÀ cái GIÁ đánh đổi:
thêm PHỨC tạp (hộp hint, TÊN chủ thật) để đổi LẤY một đảm bảo mạnh
hơn — owner sập sẽ KHÔNG "bỏ lỡ" hoàn TOÀN lần ghi đó, MIỄN là có
node nào ĐÓ chịu giữ hộ.
::::

::::checkpoint{mastery=0.85}
::::
