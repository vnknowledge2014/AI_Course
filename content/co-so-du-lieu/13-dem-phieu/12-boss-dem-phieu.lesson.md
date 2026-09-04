---
id: co-so-du-lieu.dem-phieu.boss-dem-phieu
title: "BOSS — Đếm phiếu"
summary: "heThongDemPhieu ráp TRỌN quest: ghiVoiHint ghi 'khoa1' trong lúc gamma sập (2 owner truc tiếp + 1 qua hint, W=2 vẫn thành công), phatLaiHint đưa gamma bắt kịp khi hồi phục (đúng 1 hint), docVoiReadRepair đọc lại xác nhận đúng giá trị 100. Ngay CẢ khi đọc SỚM hơn -- lúc gamma VẪN còn sập -- R=2 vẫn đủ nhờ hai owner đã ghi trực tiếp (alpha, delta), đúng đảm bảo W+R>N (bài 5) phát huy tác dụng THẬT trong lúc một node đang chết."
locale: vi
track: co-so-du-lieu
module: dem-phieu
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.merkle-tree-so-sanh-hash]
concepts: [db.boss-q13]
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
Preference list (bài 2), quorum W/R (bài 3-5), hint (bài 7-8), read
repair (bài 9), Merkle (bài 10-11). Ráp TẤT cả thành một hệ đếm
phiếu hoàn chỉnh — trông ra SAO?
::::

::::explain{#boss-that}
`heThongDemPhieu` xây vòng vnode, ghi `"khoa1"` TRONG lúc `gamma`
sập (dùng `ghiVoiHint`, bài 7), rồi `gamma` hồi phục (`phatLaiHint`,
bài 8), CUỐI cùng đọc lại VỚI quorum + read repair (`docVoiReadRepair`,
bài 9):

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

interface Hint { khoa: string; banGhi: BanGhi; chuThat: string; }

function taoHopHint(tenCacNode: string[]): Map<string, Hint[]> {
  const hop = new Map<string, Hint[]>();
  for (const ten of tenCacNode) hop.set(ten, []);
  return hop;
}

function ghiVoiHint(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, hopHint: Map<string, Hint[]>, cacNodeSong: Set<string>, khoa: string, banGhi: BanGhi, rf: number): number {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  let soAck = 0;
  for (const chuThat of dsSoHuu) {
    if (cacNodeSong.has(chuThat)) {
      khoCacNode.get(chuThat)!.set(khoa, banGhi);
      soAck++;
      continue;
    }
    const thayThe = [...cacNodeSong].find((n) => !dsSoHuu.includes(n));
    if (thayThe === undefined) continue;
    hopHint.get(thayThe)!.push({ khoa, banGhi, chuThat });
    soAck++;
  }
  return soAck;
}

function phatLaiHint(khoCacNode: Map<string, Map<string, BanGhi>>, hopHint: Map<string, Hint[]>, tenNodeHoiPhuc: string): number {
  let soHintDaPhat = 0;
  for (const [tenGiu, dsHint] of hopHint) {
    const conLai: Hint[] = [];
    for (const hint of dsHint) {
      if (hint.chuThat === tenNodeHoiPhuc) {
        khoCacNode.get(tenNodeHoiPhuc)!.set(hint.khoa, hint.banGhi);
        soHintDaPhat++;
      } else {
        conLai.push(hint);
      }
    }
    hopHint.set(tenGiu, conLai);
  }
  return soHintDaPhat;
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

function heThongDemPhieu(tenCacNode: string[], soVnodeMoiNode: number, rf: number, w: number, r: number): { ghiThanhCong: boolean; soHintPhat: number; docDuoc: number | undefined } {
  const vong = xayVongVnode(tenCacNode, soVnodeMoiNode);
  const khoCacNode = taoKhoChoMoiNode(tenCacNode);
  const hopHint = taoHopHint(tenCacNode);

  const soAckGhi = ghiVoiHint(vong, khoCacNode, hopHint, new Set(["alpha", "beta", "delta"]), "khoa1", { giaTri: 100, thoiGian: 1 }, rf);
  const ghiThanhCong = soAckGhi >= w;

  const soHintPhat = phatLaiHint(khoCacNode, hopHint, "gamma");

  const docDuoc = docVoiReadRepair(vong, khoCacNode, new Set(tenCacNode), "khoa1", rf, r);

  return { ghiThanhCong, soHintPhat, docDuoc };
}

console.log(heThongDemPhieu(["alpha", "beta", "gamma", "delta"], 20, 3, 2, 2));
```

```text title=readonly
{ ghiThanhCong: true, soHintPhat: 1, docDuoc: 100 }
```

`"khoa1"` thuộc VỀ `[alpha, gamma, delta]` (RF=3). `gamma` sập LÚC
ghi — `alpha`, `delta` xác nhận trực TIẾP, `beta` giữ hint THAY
`gamma`; `soAckGhi=3 >= w=2` NÊN `ghiThanhCong=true`. `gamma` hồi
phục, NHẬN đúng `1` hint. Đọc lại VỚI `r=2`, cả `3` owner ĐỀU đồng ý
`100` — `docDuoc=100`.
::::

::::example{#doc-som-van-dung}
Điều THÚ vị hơn: nếu đọc `"khoa1"` NGAY sau bước ghi — TRƯỚC khi
`gamma` hồi phục, TRƯỚC khi gọi `phatLaiHint` — VỚI `r=2`, kết quả
VẪN LÀ `100`. `gamma` vẫn ĐANG sập, nhưng `alpha` VÀ `delta` (hai
owner còn LẠI, đã ghi trực tiếp) đủ để thoả `r=2`. Đây chính LÀ đảm
bảo `W+R>N` (bài 5) hoạt động THẬT: `W=2, R=2, N=3` — `W+R=4>3` —
tập ghi VÀ tập đọc LUÔN giao nhau Ở ít nhất một owner, DÙ một owner
khác đang sập. Hint (bài 7-8) không phải điều KIỆN để đọc đúng NGAY
bây giờ — nó chỉ LÀ cách để owner sập bắt kịp KHI nó hồi phục.
::::

::::predict{#doan-rf-nho commitOnce}
Gọi `heThongDemPhieu(["alpha","beta","gamma","delta"], 20, 2, 1, 1)`
— RF=2 (thay VÌ 3), W=1, R=1 (thay VÌ 2). Hàm CÓ chạy được KHÔNG lỗi
VÀ trả về đúng `docDuoc: 100` không?

:::opt{correct}
Có — MỌI hàm trong `heThongDemPhieu` nhận `rf`, `w`, `r` LÀM tham
số, không hard-code `3`/`2` Ở đâu CẢ; RF=2 chỉ đổi preference list
CÒN LẠI hai owner, nhưng logic ghi/hint/đọc vận hành y HỆT
:::

:::opt
Lỗi runtime — TOÀN bộ hệ thống được thiết kế CỐ định cho đúng
RF=3,W=2,R=2 như VÍ dụ readonly
::why
Số liệu `3, 2, 2` chỉ LÀ đối SỐ truyền và trong VÍ dụ readonly — CÁC
hàm bên trong (`laySachSoHuu`, `ghiVoiHint`, `docVoiReadRepair`) đều
nhận `rf`/`w`/`r` LÀM tham số tường minh, không có con SỐ nào bị
"đóng cứng" bên trong THÂN hàm.

Chỗ lệch: KHÔNG có dòng code nào giả định `rf===3` hay
`w===2` — mọi phép SO sánh (`ketQua.length < rf`, `soAck >= w`,
`phanHoi.length < r`) đều dùng ĐÚNG tham số được truyền VÀO. Với
RF=2, preference list của `"khoa1"` rút GỌN còn `[alpha, gamma]`;
`gamma` vẫn sập LÚC ghi nên vẫn cần hint (qua `beta`); `W=1` chỉ cần
MỘT trong hai owner xác nhận (đã CÓ `alpha`), `R=1` chỉ cần MỘT
phản hồi lúc đọc — kết quả cuối VẪN đúng `100`, không lỗi.
::
:::
::::

::::code{#viet_boss_dem_phieu}
Hoàn thiện `heThongDemPhieu` — bước ĐỌC cuối cùng dùng
`docVoiReadRepair` VỚI toàn bộ node đang sống.

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

interface Hint { khoa: string; banGhi: BanGhi; chuThat: string; }

function taoHopHint(tenCacNode: string[]): Map<string, Hint[]> {
  const hop = new Map<string, Hint[]>();
  for (const ten of tenCacNode) hop.set(ten, []);
  return hop;
}

function ghiVoiHint(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, hopHint: Map<string, Hint[]>, cacNodeSong: Set<string>, khoa: string, banGhi: BanGhi, rf: number): number {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  let soAck = 0;
  for (const chuThat of dsSoHuu) {
    if (cacNodeSong.has(chuThat)) {
      khoCacNode.get(chuThat)!.set(khoa, banGhi);
      soAck++;
      continue;
    }
    const thayThe = [...cacNodeSong].find((n) => !dsSoHuu.includes(n));
    if (thayThe === undefined) continue;
    hopHint.get(thayThe)!.push({ khoa, banGhi, chuThat });
    soAck++;
  }
  return soAck;
}

function phatLaiHint(khoCacNode: Map<string, Map<string, BanGhi>>, hopHint: Map<string, Hint[]>, tenNodeHoiPhuc: string): number {
  let soHintDaPhat = 0;
  for (const [tenGiu, dsHint] of hopHint) {
    const conLai: Hint[] = [];
    for (const hint of dsHint) {
      if (hint.chuThat === tenNodeHoiPhuc) {
        khoCacNode.get(tenNodeHoiPhuc)!.set(hint.khoa, hint.banGhi);
        soHintDaPhat++;
      } else {
        conLai.push(hint);
      }
    }
    hopHint.set(tenGiu, conLai);
  }
  return soHintDaPhat;
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

function heThongDemPhieu(tenCacNode: string[], soVnodeMoiNode: number, rf: number, w: number, r: number): { ghiThanhCong: boolean; soHintPhat: number; docDuoc: number | undefined } {
  const vong = xayVongVnode(tenCacNode, soVnodeMoiNode);
  const khoCacNode = taoKhoChoMoiNode(tenCacNode);
  const hopHint = taoHopHint(tenCacNode);

  const soAckGhi = ghiVoiHint(vong, khoCacNode, hopHint, new Set(["alpha", "beta", "delta"]), "khoa1", { giaTri: 100, thoiGian: 1 }, rf);
  const ghiThanhCong = soAckGhi >= w;

  const soHintPhat = phatLaiHint(khoCacNode, hopHint, "gamma");

  const docDuoc = ___;

  return { ghiThanhCong, soHintPhat, docDuoc };
}

console.log(heThongDemPhieu(["alpha", "beta", "gamma", "delta"], 20, 3, 2, 2));
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

interface Hint { khoa: string; banGhi: BanGhi; chuThat: string; }

function taoHopHint(tenCacNode: string[]): Map<string, Hint[]> {
  const hop = new Map<string, Hint[]>();
  for (const ten of tenCacNode) hop.set(ten, []);
  return hop;
}

function ghiVoiHint(vong: DiemNode[], khoCacNode: Map<string, Map<string, BanGhi>>, hopHint: Map<string, Hint[]>, cacNodeSong: Set<string>, khoa: string, banGhi: BanGhi, rf: number): number {
  const dsSoHuu = laySachSoHuu(vong, khoa, rf);
  let soAck = 0;
  for (const chuThat of dsSoHuu) {
    if (cacNodeSong.has(chuThat)) {
      khoCacNode.get(chuThat)!.set(khoa, banGhi);
      soAck++;
      continue;
    }
    const thayThe = [...cacNodeSong].find((n) => !dsSoHuu.includes(n));
    if (thayThe === undefined) continue;
    hopHint.get(thayThe)!.push({ khoa, banGhi, chuThat });
    soAck++;
  }
  return soAck;
}

function phatLaiHint(khoCacNode: Map<string, Map<string, BanGhi>>, hopHint: Map<string, Hint[]>, tenNodeHoiPhuc: string): number {
  let soHintDaPhat = 0;
  for (const [tenGiu, dsHint] of hopHint) {
    const conLai: Hint[] = [];
    for (const hint of dsHint) {
      if (hint.chuThat === tenNodeHoiPhuc) {
        khoCacNode.get(tenNodeHoiPhuc)!.set(hint.khoa, hint.banGhi);
        soHintDaPhat++;
      } else {
        conLai.push(hint);
      }
    }
    hopHint.set(tenGiu, conLai);
  }
  return soHintDaPhat;
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

function heThongDemPhieu(tenCacNode: string[], soVnodeMoiNode: number, rf: number, w: number, r: number): { ghiThanhCong: boolean; soHintPhat: number; docDuoc: number | undefined } {
  const vong = xayVongVnode(tenCacNode, soVnodeMoiNode);
  const khoCacNode = taoKhoChoMoiNode(tenCacNode);
  const hopHint = taoHopHint(tenCacNode);

  const soAckGhi = ghiVoiHint(vong, khoCacNode, hopHint, new Set(["alpha", "beta", "delta"]), "khoa1", { giaTri: 100, thoiGian: 1 }, rf);
  const ghiThanhCong = soAckGhi >= w;

  const soHintPhat = phatLaiHint(khoCacNode, hopHint, "gamma");

  const docDuoc = docVoiReadRepair(vong, khoCacNode, new Set(tenCacNode), "khoa1", rf, r);

  return { ghiThanhCong, soHintPhat, docDuoc };
}

console.log(heThongDemPhieu(["alpha", "beta", "gamma", "delta"], 20, 3, 2, 2));
```

```typescript title=test
const tenCacNode = ["alpha", "beta", "gamma", "delta"];
const kq = heThongDemPhieu(tenCacNode, 20, 3, 2, 2);
if (kq.ghiThanhCong !== true) throw new Error("gamma sap nhung 3/3 owner van 'ack' duoc (2 truc tiep + 1 qua hint), W=2 phai thanh cong");
if (kq.soHintPhat !== 1) throw new Error("gamma hoi phuc phai phat dung 1 hint (cho khoa1)");
if (kq.docDuoc !== 100) throw new Error("doc lai sau khi gamma da bat kip phai thay dung gia tri 100");

const kq2 = heThongDemPhieu(tenCacNode, 20, 3, 3, 3);
if (kq2.docDuoc !== 100) throw new Error("voi rf=w=r=3, ket qua cuoi van phai la 100");

const kq3 = heThongDemPhieu(tenCacNode, 20, 2, 1, 1);
if (kq3.docDuoc !== 100) throw new Error("voi rf=2,w=1,r=1, ket qua cuoi van phai la 100 (khong loi runtime)");
```

:::hints
- kind: attention
  body: "Buoc doc cuoi cung dung docVoiReadRepair, voi toan bo tenCacNode dang song, rf va r duoc truyen vao -- mot dong."
- kind: strategy
  body: "docVoiReadRepair(vong, khoCacNode, new Set(tenCacNode), \"khoa1\", rf, r)"
- kind: one-line
  body: "const docDuoc = docVoiReadRepair(vong, khoCacNode, new Set(tenCacNode), \"khoa1\", rf, r);"
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
Preference list, quorum, hint, read repair, Merkle — BỐN cơ chế bổ
sung nhau đã ráp thành một hệ chịu lỗi THẬT. q13 "Đếm phiếu" khép
lại — dữ liệu bị XOÁ thì sao, một hệ phân tán "quên" một khoá bằng
cách NÀO?
::::

::::reflect{#nghi-lai}
`heThongDemPhieu` không giới thiệu MỘT khái niệm mới nào — nó CHỈ
xếp đúng THỨ tự những gì đã học: xây vòng (q11), ghi VỚI hint khi
owner sập (bài 7), phát lại hint KHI owner hồi phục (bài 8), đọc VỚI
quorum + read repair (bài 4, 9). Điều LÀM nên một hệ "đếm phiếu"
THẬT không phải MỘT thuật toán đơn lẻ, MÀ là cách những cơ chế NHỎ
này khớp VÀO nhau: quorum (bài 3-5) đảm BẢO luôn có ít nhất một
điểm CHUNG, hint (bài 7-8) giữ lại phần việc CỦA owner tạm sập,
read repair (bài 9) sửa NGAY khi phát hiện lệch LÚC đọc, và Merkle
(bài 10-11) quét ĐỊNH kỳ để bắt phần CÒN sót — không cơ chế NÀO một
mình LÀ đủ, nhưng CẢ bốn cùng nhau tạo THÀNH một hệ chịu lỗi THẬT sự.
::::

::::checkpoint{mastery=0.9}
::::
