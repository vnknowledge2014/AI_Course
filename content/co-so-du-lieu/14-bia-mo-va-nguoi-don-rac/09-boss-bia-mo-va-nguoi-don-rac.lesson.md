---
id: co-so-du-lieu.bia-mo-va-nguoi-don-rac.boss-bia-mo-va-nguoi-don-rac
title: "BOSS — Bia mộ và người dọn rác"
summary: "heThongDonRac ráp TRỌN quest: xoaTheoQuorum ghi bia mộ trong lúc gamma sập (alpha+delta ack, W=2 đủ), gamma hồi phục rồi được docBanGhiVoiReadRepair CHỮA bằng bia mộ THẬT (không phải giá trị cũ), rồi nenTheoGCGrace chỉ purge SAU khi qua gcGraceMs trên CẢ BA node — kết quả: 3 bia mộ bị xoá sạch, KHÔNG còn zombie. Đối chứng: bỏ qua bước đọc-sửa, gamma vẫn giữ giá trị cũ 100 mãi mãi dù compaction đã chạy trên hai node kia."
locale: vi
track: co-so-du-lieu
module: bia-mo-va-nguoi-don-rac
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.so-sanh-ba-chien-luoc-nen]
concepts: [db.boss-q14]
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
Xoá LÀ ghi bia mộ (bài 1). Purge SỚM hồi sinh zombie (bài 2).
gc_grace LÀ hạn chờ AN toàn (bài 3). Bia mộ tích luỹ tới KHI compaction
dọn (bài 4-8). Ráp TẤT cả — một hệ dọn rác KHÔNG tạo zombie trông ra
SAO?
::::

::::explain{#boss-that}
`heThongDonRac` ghi giá trị BAN đầu, xoá TRONG lúc `gamma` sập (dùng
`xoaTheoQuorum`, bài 1), để `gamma` hồi PHỤC rồi đọc LẠI VỚI read
repair (`docBanGhiVoiReadRepair`, bài 2 — CHỮA `gamma` bằng bia mộ
THẬT chứ không phải zombie, vì bia mộ VẪN còn nguyên Ở `alpha`/`delta`
lúc đọc), CUỐI cùng chạy compaction tôn trọng gc_grace
(`nenTheoGCGrace`, dùng `coTheXoaBiaMo` bài 3) TRÊN từng node:

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

function coTheXoaBiaMo(banGhi: BanGhi, thoiGianHienTai: number, gcGraceMs: number): boolean {
  return banGhi.giaTri === null && (thoiGianHienTai - banGhi.thoiGian) >= gcGraceMs;
}

function nenTheoGCGrace(kho: Map<string, BanGhi>, thoiGianHienTai: number, gcGraceMs: number): number {
  let daXoa = 0;
  for (const [k, bg] of [...kho.entries()]) {
    if (coTheXoaBiaMo(bg, thoiGianHienTai, gcGraceMs)) { kho.delete(k); daXoa++; }
  }
  return daXoa;
}

function heThongDonRac(tenCacNode: string[], soVnodeMoiNode: number, rf: number, w: number, gcGraceMs: number) {
  const vong = xayVongVnode(tenCacNode, soVnodeMoiNode);
  const khoCacNode = taoKhoChoMoiNode(tenCacNode);
  const khoa = "khoa1";

  ghiBanGhiTheoQuorum(vong, khoCacNode, new Set(tenCacNode), khoa, { giaTri: 100, thoiGian: 1 }, rf, rf);
  const xoaOk = xoaTheoQuorum(vong, khoCacNode, new Set(["alpha", "beta", "delta"]), khoa, 2, rf, w);

  const docSauHoiPhuc = docBanGhiVoiReadRepair(vong, khoCacNode, new Set(tenCacNode), khoa, rf, rf);

  const thoiGianNen = 2 + gcGraceMs + 1;
  let tongSoDaXoa = 0;
  for (const ten of tenCacNode) tongSoDaXoa += nenTheoGCGrace(khoCacNode.get(ten)!, thoiGianNen, gcGraceMs);

  const conBiaMo = tenCacNode.some((ten) => khoCacNode.get(ten)!.has(khoa));
  const coZombie = tenCacNode.some((ten) => khoCacNode.get(ten)!.get(khoa)?.giaTri === 100);

  return { xoaOk, docSauHoiPhuc, tongSoDaXoa, conBiaMo, coZombie };
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
console.log(heThongDonRac(tenCacNode, 20, 3, 2, 1000));
```

```text title=readonly
{
  xoaOk: true,
  docSauHoiPhuc: { giaTri: null, thoiGian: 2 },
  tongSoDaXoa: 3,
  conBiaMo: false,
  coZombie: false
}
```

Toàn bộ CÂU chuyện: `gamma` sập LÚC xoá — vẫn ACK đủ `W=2` từ
`alpha`/`delta`. `gamma` hồi PHỤC, đọc LẠI (read repair) chữa NÓ
đúng — BẰNG bia mộ (`docSauHoiPhuc.giaTri: null`), KHÔNG phải zombie
`100`. Sau khi qua `gcGraceMs`, compaction xoá SẠCH CẢ ba bia mộ
(`tongSoDaXoa: 3`) — `conBiaMo: false` (dọn xong), `coZombie: false`
(không hồi SINH). Thứ TỰ đúng — không zombie.
::::

::::example{#thu-tu-sai-thi-sao}
NẾU bỏ QUA bước đọc-sửa (không hề gọi `docBanGhiVoiReadRepair`
TRƯỚC khi nén), `gamma` VẪN giữ giá trị CŨ `100` mãi MÃI — compaction
CHỈ xoá được bia mộ Ở `alpha`/`delta` (`tongSoDaXoa: 2`, không phải
`3`, VÌ `gamma` không hề có bia mộ NÀO để xoá), còn `gamma` tiếp tục
"nói dối" cho BẤT kỳ ai đọc trực tiếp NÓ. `coZombie` sẽ LÀ `true` nếu
kiểm tra ngay lúc NÀY — thứ TỰ (repair TRƯỚC purge) LÀ điều kiện SỐNG
còn, không phải chi tiết vặt.
::::

::::predict{#doan-gc-grace-qua-ngan commitOnce}
CÙNG kịch bản, NHƯNG `gcGraceMs=1` (CỰC ngắn — compaction chạy gần
NHƯ ngay lập tức Ở `thoiGianNen = 2+1+1 = 4`), VÀ `gamma` chỉ hồi
phục Ở `thoiGian=100` (rất LÂU sau đó, KHÔNG kịp trước lúc nén).
`coZombie` cuối CÙNG LÀ gì?

:::opt{correct}
`true` — compaction Ở `alpha`/`delta` chạy TRƯỚC khi `gamma` từng có
cơ hội được CHỮA, nên KHÔNG owner nào còn giữ bằng chứng "đã xoá" Ở
thời ĐIỂM đọc sau đó — y hệt lỗi purge quá SỚM Ở bài 2
:::

:::opt
`false` — `heThongDonRac` LUÔN gọi `docBanGhiVoiReadRepair` TRƯỚC
`nenTheoGCGrace` trong CHÍNH code của nó, NÊN thứ tự đúng đã được
đảm bảo SẴN
::why
Đúng LÀ code của `heThongDonRac` GỌI read repair trước nén — nhưng
CÂU hỏi ĐÃ đổi kịch bản: `gamma` hồi PHỤC Ở `thoiGian=100`, SAU thời
điểm compaction (`thoiGianNen=4`) diễn RA trong hàm.

Chỗ lệch: THỨ tự gọi HÀM trong code KHÔNG đảm bảo thứ tự SỰ kiện
THẬT — `docBanGhiVoiReadRepair` (gọi Ở giữa hàm) chỉ CHỮA được owner
ĐANG sống TẠI thời điểm đó; `gamma` (hồi phục MUỘN Ở t=100) KHÔNG hề
có mặt trong `cacNodeSong` LÚC hàm gọi read repair — bia mộ Ở
`alpha`/`delta` đã bị `nenTheoGCGrace` xoá TRƯỚC khi `gamma` kịp
tham gia bất KỲ lần đọc nào.
::
:::
::::

::::code{#viet_he_thong_don_rac}
Hoàn thiện `heThongDonRac` — bước NÉN cuối cùng gọi `nenTheoGCGrace`
CHO từng node, cộng dồn tổng số bia mộ đã xoá.

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
  return ghiBanGhiTheoQuorum(vong, khoCacNode, cacNodeSong, khoa, { giaTri: null, thoiGian }, rf, w);
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

function coTheXoaBiaMo(banGhi: BanGhi, thoiGianHienTai: number, gcGraceMs: number): boolean {
  return banGhi.giaTri === null && (thoiGianHienTai - banGhi.thoiGian) >= gcGraceMs;
}

function nenTheoGCGrace(kho: Map<string, BanGhi>, thoiGianHienTai: number, gcGraceMs: number): number {
  let daXoa = 0;
  for (const [k, bg] of [...kho.entries()]) {
    if (coTheXoaBiaMo(bg, thoiGianHienTai, gcGraceMs)) { kho.delete(k); daXoa++; }
  }
  return daXoa;
}

function heThongDonRac(tenCacNode: string[], soVnodeMoiNode: number, rf: number, w: number, gcGraceMs: number) {
  const vong = xayVongVnode(tenCacNode, soVnodeMoiNode);
  const khoCacNode = taoKhoChoMoiNode(tenCacNode);
  const khoa = "khoa1";

  ghiBanGhiTheoQuorum(vong, khoCacNode, new Set(tenCacNode), khoa, { giaTri: 100, thoiGian: 1 }, rf, rf);
  const xoaOk = xoaTheoQuorum(vong, khoCacNode, new Set(["alpha", "beta", "delta"]), khoa, 2, rf, w);

  const docSauHoiPhuc = docBanGhiVoiReadRepair(vong, khoCacNode, new Set(tenCacNode), khoa, rf, rf);

  const thoiGianNen = 2 + gcGraceMs + 1;
  let tongSoDaXoa = 0;
  for (const ten of tenCacNode) {
    ___
  }

  const conBiaMo = tenCacNode.some((ten) => khoCacNode.get(ten)!.has(khoa));
  const coZombie = tenCacNode.some((ten) => khoCacNode.get(ten)!.get(khoa)?.giaTri === 100);

  return { xoaOk, docSauHoiPhuc, tongSoDaXoa, conBiaMo, coZombie };
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
console.log(heThongDonRac(tenCacNode, 20, 3, 2, 1000).tongSoDaXoa);
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

function coTheXoaBiaMo(banGhi: BanGhi, thoiGianHienTai: number, gcGraceMs: number): boolean {
  return banGhi.giaTri === null && (thoiGianHienTai - banGhi.thoiGian) >= gcGraceMs;
}

function nenTheoGCGrace(kho: Map<string, BanGhi>, thoiGianHienTai: number, gcGraceMs: number): number {
  let daXoa = 0;
  for (const [k, bg] of [...kho.entries()]) {
    if (coTheXoaBiaMo(bg, thoiGianHienTai, gcGraceMs)) { kho.delete(k); daXoa++; }
  }
  return daXoa;
}

function heThongDonRac(tenCacNode: string[], soVnodeMoiNode: number, rf: number, w: number, gcGraceMs: number) {
  const vong = xayVongVnode(tenCacNode, soVnodeMoiNode);
  const khoCacNode = taoKhoChoMoiNode(tenCacNode);
  const khoa = "khoa1";

  ghiBanGhiTheoQuorum(vong, khoCacNode, new Set(tenCacNode), khoa, { giaTri: 100, thoiGian: 1 }, rf, rf);
  const xoaOk = xoaTheoQuorum(vong, khoCacNode, new Set(["alpha", "beta", "delta"]), khoa, 2, rf, w);

  const docSauHoiPhuc = docBanGhiVoiReadRepair(vong, khoCacNode, new Set(tenCacNode), khoa, rf, rf);

  const thoiGianNen = 2 + gcGraceMs + 1;
  let tongSoDaXoa = 0;
  for (const ten of tenCacNode) {
    tongSoDaXoa += nenTheoGCGrace(khoCacNode.get(ten)!, thoiGianNen, gcGraceMs);
  }

  const conBiaMo = tenCacNode.some((ten) => khoCacNode.get(ten)!.has(khoa));
  const coZombie = tenCacNode.some((ten) => khoCacNode.get(ten)!.get(khoa)?.giaTri === 100);

  return { xoaOk, docSauHoiPhuc, tongSoDaXoa, conBiaMo, coZombie };
}

const tenCacNode = ["alpha", "beta", "gamma", "delta"];
console.log(heThongDonRac(tenCacNode, 20, 3, 2, 1000).tongSoDaXoa);
```

```typescript title=test
const kq = heThongDonRac(tenCacNode, 20, 3, 2, 1000);
if (kq.xoaOk !== true) throw new Error("gamma sap nhung alpha+delta ack du, W=2 phai thanh cong");
if (kq.docSauHoiPhuc?.giaTri !== null) throw new Error("doc lai sau khi gamma song lai phai thay bia mo (null), khong phai gia tri cu");
if (kq.tongSoDaXoa !== 3) throw new Error("sau khi qua gc_grace, ca 3 owner (da duoc chua) phai bi xoa bia mo -- tong la 3");
if (kq.conBiaMo !== false) throw new Error("sau nen, khong con owner nao giu bia mo nua");
if (kq.coZombie !== false) throw new Error("thu tu dung (repair truoc, nen sau) khong duoc tao zombie");

const kqRfKhac = heThongDonRac(tenCacNode, 20, 2, 1, 500);
if (kqRfKhac.coZombie !== false) throw new Error("voi rf/w/gcGrace khac, van khong duoc co zombie neu thu tu dung");

const kqGcZero = heThongDonRac(tenCacNode, 20, 3, 2, 0);
if (kqGcZero.tongSoDaXoa !== 3) throw new Error("gcGraceMs=0 (khong an han) van phai xoa dung ca 3 bia mo -- moi node PHAI dung DUNG gcGraceMs lam nguong, khong duoc lay nham mot bien khac");
```

:::hints
- kind: attention
  body: "Cong don ket qua nenTheoGCGrace tren kho cua tung node vao tongSoDaXoa -- mot dong."
- kind: strategy
  body: "tongSoDaXoa += nenTheoGCGrace(khoCacNode.get(ten)!, thoiGianNen, gcGraceMs);"
- kind: one-line
  body: "tongSoDaXoa += nenTheoGCGrace(khoCacNode.get(ten)!, thoiGianNen, gcGraceMs);"
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
Preference list, quorum, hint, read repair, Merkle (q13) — VÀ giờ
bia mộ, gc_grace, ba chiến lược nén (q14) — MỘT hệ phân TÁN dọn rác
đúng THỨ tự, không zombie. q14 "Bia mộ và người dọn rác" khép LẠI —
còn MỘT câu hỏi cuối CỦA R6-2: mô hình hoá dữ liệu phân tán THEO
đúng câu hỏi sẽ hỏi, không phải theo THÓI quen từ CSDL quan hệ.
::::

::::reflect{#nghi-lai}
`heThongDonRac` không giới THIỆU một khái niệm MỚI nào — nó XẾP đúng
THỨ tự những gì đã học: xoá LÀ ghi bia mộ (bài 1), read repair CHỮA
owner lỡ hẹn TRƯỚC khi bia mộ biến mất (bài 2, đối lập VỚI purge quá
sớm), gc_grace LÀ hạn chờ AN toàn (bài 3), VÀ compaction (bài 5-8)
LÀ nơi bia mộ THẬT sự bị dọn. Bài học LỚN nhất của quest NÀY: THỨ tự
sự kiện QUAN trọng hơn từng thao TÁC riêng lẻ — cùng những hàm ĐÓ,
đảo ngược THỨ tự (nén TRƯỚC khi mọi owner kịp được chữa) biến MỘT hệ
đúng đắn thành một hệ HỒI sinh dữ liệu đã xoá.
::::

::::checkpoint{mastery=0.9}
::::
