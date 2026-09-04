---
id: co-so-du-lieu.dem-phieu.ghi-can-bao-nhieu-phieu-w
title: "Ghi cần bao nhiêu phiếu — W"
summary: "ghiTheoQuorum ghi vào TỪNG node trong preference list ĐANG sống, đếm số 'phiếu' (soAck) nhận được, coi LÀ THÀNH công khi soAck >= w. Với RF=3, W=2: một node sập (2/3 owner còn sống) vẫn thành công (2>=2); hai node sập (1/3 owner còn sống) thất bại (1<2). Một node KHOẺ mạnh nhưng KHÔNG PHẢI owner của khoá không hề tính vào phiếu — chỉ owner mới có quyền bầu."
locale: vi
track: co-so-du-lieu
module: dem-phieu
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.ghi-can-bao-nhieu-phieu-w]
requires: [db.rf-nhieu-hon-mot-preference-list]
concepts: [db.ghi-can-bao-nhieu-phieu-w]
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
Ba node giữ một khoá (RF=3, bài trước). Ghi một giá trị mới — cần
CẢ ba xác nhận, hay ÍT hơn cũng được TÍNH là "ghi xong"?
::::

::::explain{#ghi-theo-quorum}
`ghiTheoQuorum` gửi lệnh ghi TỚI từng node trong preference list ĐANG
sống, đếm SỐ node xác nhận (`soAck`) — coi LÀ thành công khi
`soAck >= w`. `w` (write quorum) LÀ số "phiếu" tối thiểu cần CÓ để
một lần ghi được TÍNH là hợp lệ:

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

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
console.log("owners khoa1:", laySachSoHuu(vong, "khoa1", 3));

const kho = taoKhoChoMoiNode(tenCacNode);
console.log("tat ca song, W=2:", ghiTheoQuorum(vong, kho, new Set(tenCacNode), "khoa1", 100, 3, 2));
console.log("gamma sap, W=2:", ghiTheoQuorum(vong, kho, new Set(["alpha", "beta", "delta"]), "khoa1", 100, 3, 2));
```

```text title=readonly
owners khoa1: [ 'alpha', 'gamma', 'delta' ]
tat ca song, W=2: true
gamma sap, W=2: true
```

`"khoa1"` thuộc VỀ `[alpha, gamma, delta]`. `gamma` sập vẫn CÒN
`alpha` VÀ `delta` xác nhận — `soAck=2`, ĐÚNG bằng `w=2` — ghi vẫn
THÀNH công. Chỉ khi CẢ `gamma` LẪN `delta` cùng sập (`soAck=1 < 2`)
thì ghi mới THẤT bại.
::::

::::example{#node-khoe-nhung-khong-phieu}
`beta` LUÔN khoẻ mạnh trong toàn bộ ví DỤ trên — nhưng `beta` KHÔNG
nằm trong preference list của `"khoa1"` (`[alpha, gamma, delta]`),
NÊN nó không hề được TÍNH vào `soAck`, dù `cacNodeSong` CÓ chứa nó
hay không. Quorum CHỈ đếm phiếu từ đúng NHỮNG node CÓ quyền giữ bản
sao của khoá ĐANG ghi — một node khoẻ mạnh nhưng "ngoài cuộc" không
giúp ích GÌ, giống HỆT bài 1 (một node khoẻ khác không cứu được khoá
thuộc VỀ node đã sập).
::::

::::predict{#doan-tat-ca-owner-sap commitOnce}
`RF=3`, `W=2`. CẢ ba owner của `"khoa1"` (`alpha, gamma, delta`) đều
sập — CHỈ `beta` còn sống (không phải owner). `ghiTheoQuorum` trả về
gì?

:::opt{correct}
`false` — `soAck=0` (không owner NÀO xác nhận được, vì cả ba đều
sập), `0 < 2`; `beta` sống KHÔNG đổi được gì vì nó không phải owner
:::

:::opt
`true` — VẪN còn MỘT node sống (`beta`) trong TOÀN cụm, hệ thống có
THỂ ghi tạm VÀO đó rồi chuyển GIAO sau
::why
Trực giác NÀY không sai VỀ mặt kỹ thuật phân TÁN nói chung — CÓ
những kỹ thuật thật (hinted handoff, sẽ học Ở bài 7) cho phép "ghi
tạm vào node KHÁC rồi chuyển giao SAU". Nhưng đó LÀ một cơ CHẾ RIÊNG,
CHƯA có mặt trong `ghiTheoQuorum` của bài NÀY.

Chỗ lệch: `ghiTheoQuorum` (đúng NHƯ code Ở trên) chỉ duyệt
`dsSoHuu` (preference list) — nó KHÔNG hề biết tới sự tồn tại CỦA
`beta` khi xử lý `"khoa1"`, DÙ `beta` có mặt trong `cacNodeSong`. Ba
owner cùng sập nghĩa LÀ `soAck` giữ nguyên `0` — `ghiTheoQuorum` trả
về `false`, KHÔNG có ngoại lệ nào.
::
:::
::::

::::code{#viet_ghi_theo_quorum}
Hoàn thiện `ghiTheoQuorum` — VỚI mỗi node sở hữu CÒN sống, ghi giá
trị THẬT vào kho của nó.

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
    ___
    soAck++;
  }
  return soAck >= w;
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
console.log(ghiTheoQuorum(vong, kho, new Set(["alpha", "beta", "delta"]), "khoa1", 100, 3, 2));
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

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const vong = xayVongVnode(tenCacNode, 20);
const kho = taoKhoChoMoiNode(tenCacNode);
console.log(ghiTheoQuorum(vong, kho, new Set(["alpha", "beta", "delta"]), "khoa1", 100, 3, 2));
```

```typescript title=test
let khoA = taoKhoChoMoiNode(tenCacNode);
if (ghiTheoQuorum(vong, khoA, new Set(tenCacNode), "khoa1", 100, 3, 2) !== true) throw new Error("tat ca node song, W=2 <= 3 owner song thi phai thanh cong");
if (khoA.get("alpha")!.get("khoa1") !== 100) throw new Error("alpha (owner) phai duoc ghi that");

let khoB = taoKhoChoMoiNode(tenCacNode);
if (ghiTheoQuorum(vong, khoB, new Set(["alpha", "beta", "delta"]), "khoa1", 100, 3, 2) !== true) throw new Error("gamma sap nhung 2/3 owner con lai (alpha,delta) van >= W=2 nen phai thanh cong");

let khoC = taoKhoChoMoiNode(tenCacNode);
if (ghiTheoQuorum(vong, khoC, new Set(["alpha", "beta"]), "khoa1", 100, 3, 2) !== false) throw new Error("gamma va delta cung sap, chi con 1 owner (alpha) < W=2 nen phai that bai");

let khoD = taoKhoChoMoiNode(tenCacNode);
if (ghiTheoQuorum(vong, khoD, new Set(["beta"]), "khoa1", 100, 3, 2) !== false) throw new Error("beta song nhung KHONG phai owner cua khoa1 -- khong tinh vao quorum, van phai that bai");
```

:::hints
- kind: attention
  body: "Khoa nay CON SONG (da qua continue) -- ghi that vao kho cua node, mot dong."
- kind: strategy
  body: "khoCacNode.get(ten)!.set(khoa, giaTri);"
- kind: one-line
  body: "khoCacNode.get(ten)!.set(khoa, giaTri);"
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
Ghi thành công khi ĐỦ phiếu — vậy ĐỌC cần bao nhiêu phiếu để chắc
chắn thấy đúng dữ liệu ĐÃ ghi?
::::

::::reflect{#nghi-lai}
`ghiTheoQuorum` không đòi hỏi TẤT cả `rf` owner phải sống — chỉ CẦN
`w` trong SỐ đó. Đây LÀ điểm khác biệt CĂN bản so VỚI RF=1 (bài 1):
thay VÌ "một node sập LÀ mất trắng", giờ HỆ thống chấp nhận một VÀI
owner sập MÀ vẫn ghi được, MIỄN là đủ phiếu tối THIỂU. Cái GIÁ:
những owner sập ĐÓ giờ giữ dữ liệu CŨ (hoặc trống) — câu hỏi tiếp
theo LÀ đọc lại có ĐẢM bảo thấy đúng giá trị MỚI hay không.
::::

::::checkpoint{mastery=0.8}
::::
