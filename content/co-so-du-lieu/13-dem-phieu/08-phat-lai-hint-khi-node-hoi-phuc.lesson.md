---
id: co-so-du-lieu.dem-phieu.phat-lai-hint-khi-node-hoi-phuc
title: "Phát lại hint khi node hồi phục"
summary: "phatLaiHint duyệt TOÀN BỘ hộp hint, ghi mọi hint có chuThat KHỚP tên node vừa hồi phục VÀO kho thật của chính nó, xoá hint đó khỏi hộp giữ tạm -- hint KHÔNG khớp được giữ nguyên. Sau khi gamma hồi phục và phát lại, kho thật của gamma có đúng giá trị đã ghi qua hint (200), và hộp hint của beta chỉ còn hint dành cho delta -- gọi phát lại lần hai khi hết hint trả về 0, không lỗi."
locale: vi
track: co-so-du-lieu
module: dem-phieu
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.phat-lai-hint-khi-node-hoi-phuc]
requires: [db.node-im-lang-khi-ghi]
concepts: [db.phat-lai-hint-khi-node-hoi-phuc]
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
`beta` đang giữ hộ hint CHO `gamma` (bài trước). `gamma` sống LẠI —
hint đó phải được TRẢ về đúng chủ, KHÔNG thể nằm mãi Ở `beta`.
::::

::::explain{#phat-lai-hint}
`phatLaiHint` duyệt TOÀN bộ hộp hint (Ở MỌI node), tìm những hint CÓ
`chuThat` khớp ĐÚNG tên node vừa hồi PHỤC — ghi chúng VÀO kho thật
của chính node ĐÓ, rồi xoá khỏi hộp giữ TẠM. Hint KHÔNG khớp được
giữ nguyên:

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

function phatLaiHint(khoCacNode: Map<string, Map<string, number>>, hopHint: Map<string, Hint[]>, tenNodeHoiPhuc: string): number {
  let soHintDaPhat = 0;
  for (const [tenGiu, dsHint] of hopHint) {
    const conLai: Hint[] = [];
    for (const hint of dsHint) {
      if (hint.chuThat === tenNodeHoiPhuc) {
        khoCacNode.get(tenNodeHoiPhuc)!.set(hint.khoa, hint.giaTri);
        soHintDaPhat++;
      } else {
        conLai.push(hint);
      }
    }
    hopHint.set(tenGiu, conLai);
  }
  return soHintDaPhat;
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
const hop = taoHopHint(tenCacNode);
ghiVoiHint(vong, kho, hop, new Set(["alpha", "beta"]), "khoa1", 200, 3); // gamma+delta sap

console.log("truoc phat lai -- gamma kho:", String(kho.get("gamma")!.get("khoa1")));
console.log("gamma hoi phuc -- so hint phat:", phatLaiHint(kho, hop, "gamma"));
console.log("sau phat lai -- gamma kho:", kho.get("gamma")!.get("khoa1"));
console.log("beta con lai:", hop.get("beta"));
```

```text title=readonly
truoc phat lai -- gamma kho: undefined
gamma hoi phuc -- so hint phat: 1
sau phat lai -- gamma kho: 200
beta con lai: [ { khoa: 'khoa1', giaTri: 200, chuThat: 'delta' } ]
```

`gamma` hồi PHỤC nhận đúng `1` hint (dành riêng cho NÓ) — kho THẬT
của `gamma` giờ CÓ `200`. Hint dành CHO `delta` (`delta` vẫn ĐANG
sập) KHÔNG bị đụng TỚI — vẫn nằm nguyên Ở `beta`, chờ đúng LƯỢT của
nó.
::::

::::example{#phat-lai-het-hint}
Nếu gọi `phatLaiHint(kho, hop, "delta")` NGAY sau đó, `delta` sẽ
nhận NỐT hint còn LẠI — hộp hint của `beta` LÚC này rỗng HOÀN toàn.
Gọi `phatLaiHint` một LẦN nữa (KHÔNG còn hint nào) trả VỀ `0`, KHÔNG
LỖI — hàm được thiết kế AN toàn khi gọi LẶP lại, giống HỆT tinh thần
idempotent ĐÃ gặp Ở nhiều nơi trong track NÀY.
::::

::::predict{#doan-node-chua-tung-sap commitOnce}
Gọi `phatLaiHint(kho, hop, "alpha")` — TRONG khi `alpha` CHƯA từng
sập lần NÀO (mọi ghi TRƯỚC đó đều tới thẳng `alpha`, không QUA hint).
Kết quả TRẢ về LÀ gì?

:::opt{correct}
`0` — KHÔNG hộp hint nào (Ở BẤT kỳ node nào) chứa MỘT hint có
`chuThat === "alpha"`, vì `alpha` chưa BAO giờ cần ai giữ hộ; hàm
chạy an toàn, không LỖI, chỉ đơn giản KHÔNG tìm thấy gì để phát
:::

:::opt
Lỗi runtime — hàm giả định LUÔN có Ít nhất một hint để phát MỖI khi
được gọi
::why
Không CÓ dòng code NÀO trong `phatLaiHint` giả định "PHẢI có hint" —
KHÔNG có `!` non-null assertion HAY truy cập nào có thể NÉM lỗi khi
danh sách hint rỗng.

Chỗ lệch: vòng lặp `for (const hint of dsHint)` đơn giản KHÔNG chạy
LẦN nào nếu `dsHint` rỗng (HOẶC không chứa hint khớp TÊN) — `conLai`
giữ nguyên NHỮNG gì đã có, `soHintDaPhat` không tăng. Hàm trả VỀ `0`
một CÁCH bình thường, đúng NHƯ mọi lần gọi "không tìm thấy GÌ" khác
trong track NÀY (VÍ dụ `docRf1` trả `undefined` khi owner sập —
không lỗi, chỉ LÀ một kết quả hợp LỆ).
::
:::
::::

::::code{#viet_phat_lai_hint}
Hoàn thiện `phatLaiHint` — VỚI mỗi hint khớp ĐÚNG tên node hồi phục,
ghi giá TRỊ thật vào kho của NÓ.

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
    hopHint.get(thayThe)!.push({ khoa, giaTri, chuThat });
    soAck++;
  }
  return soAck;
}

function phatLaiHint(khoCacNode: Map<string, Map<string, number>>, hopHint: Map<string, Hint[]>, tenNodeHoiPhuc: string): number {
  let soHintDaPhat = 0;
  for (const [tenGiu, dsHint] of hopHint) {
    const conLai: Hint[] = [];
    for (const hint of dsHint) {
      if (hint.chuThat === tenNodeHoiPhuc) {
        ___
        soHintDaPhat++;
      } else {
        conLai.push(hint);
      }
    }
    hopHint.set(tenGiu, conLai);
  }
  return soHintDaPhat;
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
const hop = taoHopHint(tenCacNode);
ghiVoiHint(vong, kho, hop, new Set(["alpha", "beta"]), "khoa1", 200, 3);
console.log(phatLaiHint(kho, hop, "gamma"));
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

function phatLaiHint(khoCacNode: Map<string, Map<string, number>>, hopHint: Map<string, Hint[]>, tenNodeHoiPhuc: string): number {
  let soHintDaPhat = 0;
  for (const [tenGiu, dsHint] of hopHint) {
    const conLai: Hint[] = [];
    for (const hint of dsHint) {
      if (hint.chuThat === tenNodeHoiPhuc) {
        khoCacNode.get(tenNodeHoiPhuc)!.set(hint.khoa, hint.giaTri);
        soHintDaPhat++;
      } else {
        conLai.push(hint);
      }
    }
    hopHint.set(tenGiu, conLai);
  }
  return soHintDaPhat;
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
const hop = taoHopHint(tenCacNode);
ghiVoiHint(vong, kho, hop, new Set(["alpha", "beta"]), "khoa1", 200, 3);
console.log(phatLaiHint(kho, hop, "gamma"));
```

```typescript title=test
const kho2 = taoKhoChoMoiNode(tenCacNode);
const hop2 = taoHopHint(tenCacNode);
ghiVoiHint(vong, kho2, hop2, new Set(["alpha", "beta"]), "khoa1", 200, 3);
if (kho2.get("gamma")!.get("khoa1") !== undefined) throw new Error("truoc khi phat lai, gamma chua co gi trong kho that");

const soPhat1 = phatLaiHint(kho2, hop2, "gamma");
if (soPhat1 !== 1) throw new Error("gamma hoi phuc phai phat dung 1 hint");
if (kho2.get("gamma")!.get("khoa1") !== 200) throw new Error("sau khi phat lai, gamma phai co dung gia tri 200 trong kho that");
if (hop2.get("beta")!.length !== 1) throw new Error("hop hint cua beta phai con lai dung 1 hint (cho delta), da xoa hint gamma");
if (hop2.get("beta")![0]!.chuThat !== "delta") throw new Error("hint con lai phai la cua delta");

const soPhat2 = phatLaiHint(kho2, hop2, "delta");
if (soPhat2 !== 1) throw new Error("delta hoi phuc phai phat dung 1 hint");
if (kho2.get("delta")!.get("khoa1") !== 200) throw new Error("sau khi phat lai, delta phai co dung gia tri 200");
if (hop2.get("beta")!.length !== 0) throw new Error("het hint thi hop hint cua beta phai rong");

const soPhat3 = phatLaiHint(kho2, hop2, "delta");
if (soPhat3 !== 0) throw new Error("phat lai lan hai khi khong con hint nao phai tra ve 0");
```

:::hints
- kind: attention
  body: "Hint nay khop dung ten node hoi phuc -- ghi gia tri that vao kho cua CHINH no, mot dong."
- kind: strategy
  body: "khoCacNode.get(tenNodeHoiPhuc)!.set(hint.khoa, hint.giaTri);"
- kind: one-line
  body: "khoCacNode.get(tenNodeHoiPhuc)!.set(hint.khoa, hint.giaTri);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hint đã trả ĐÚNG chủ — nhưng NẾU không ai từng ghi hint (owner CHỈ
đơn giản chậm cập nhật, không hề sập), làm sao PHÁT hiện sự khác
biệt LÚC đọc?
::::

::::reflect{#nghi-lai}
`phatLaiHint` đóng lại đúng VÒNG lặp mà `ghiVoiHint` (bài 7) mở RA:
ghi tạm Ở node khác, rồi TRẢ về đúng chủ khi có THỂ. Cặp đôi NÀY giải
quyết đúng một LOẠI lỗi — node sập ngay LÚC ghi. Nhưng còn một LOẠI
khác: node KHÔNG hề sập, chỉ đơn giản CHƯA kịp nhận một lần ghi (VÍ
dụ do độ trễ mạng) — hint không giúp gì Ở đây, VÌ chẳng ai biết cần
gửi hint CHO nó.
::::

::::checkpoint{mastery=0.85}
::::
