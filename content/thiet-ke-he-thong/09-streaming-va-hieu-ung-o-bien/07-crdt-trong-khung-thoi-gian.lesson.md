---
id: thiet-ke-he-thong.streaming-va-hieu-ung-o-bien.crdt-trong-khung-thoi-gian
title: "Kết hợp CRDT vào windowing: mỗi khung là một GCounter độc lập"
summary: "gomTheoKhungCRDT(cacSuKien, kichThuocKhungMs) gom moi khung thanh mot GCounter (theo NGUON, khong phai mot tong don thuan) -- hopNhatKhungGCounter hop nhat hai Map khung tu hai server DOC LAP bang DUNG hopNhatGCounter (quest 'CRDT va hop nhat' bai 1) cho tung khung trung ten; hai nguon gom CUNG mot khung hop nhat ra dung tong, khong dem trung, hop nhat LAP lai van ra dung gia tri (nho MAX)."
locale: vi
track: thiet-ke-he-thong
module: streaming-va-hieu-ung-o-bien
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.fp.crdt-trong-khung-thoi-gian]
requires: [sd.fp.windowing-voi-watermark]
concepts: [sd.fp.crdt-trong-khung-thoi-gian]
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
`gomTheoKhung` (bài 1) giả định một luồng sự kiện DUY nhất, xử lý tuần
tự. Nhưng một hệ thống streaming THẬT thường có NHIỀU edge server, mỗi
server tự gom khung CỦA RIÊNG nó, không đợi server khác. Khi hai server
độc lập cùng có dữ liệu cho khung `0`, hợp nhất chúng thế nào để không
đếm trùng? Quest "CRDT và hợp nhất" đã trả lời câu hỏi này — cho một bộ
đếm ĐƠN. Giờ áp dụng ĐÚNG lời giải đó vào TỪNG khung thời gian.
::::

::::explain{#moi-khung-la-mot-gcounter}
`gomTheoKhungCRDT` giống `gomTheoKhung` (bài 1) Ở cấu trúc — vẫn
`reduce` qua sự kiện, vẫn tạo `Map` MỚI mỗi bước — nhưng mỗi khung
không giữ một con số `tong` đơn thuần, mà giữ một `GCounter` (đúng
`interface` VÀ hàm từ `01-g-counter` của quest "CRDT và hợp nhất"),
đếm riêng theo `sk.nguon`. `hopNhatKhungGCounter` hợp nhất HAI `Map`
khung — của hai server ĐỘC LẬP — bằng cách gọi ĐÚNG `hopNhatGCounter`
cho từng khung trùng chỉ số:

```typescript title=readonly
interface GCounter { theoReplica: Record<string, number>; }
function counterRong(): GCounter { return { theoReplica: {} }; }
function tangGCounter(gc: GCounter, replicaId: string, buoc: number): GCounter {
  const hienTai = gc.theoReplica[replicaId] ?? 0;
  return { theoReplica: { ...gc.theoReplica, [replicaId]: hienTai + buoc } };
}
function giaTriGCounter(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
function hopNhatGCounter(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}

interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KhungGCounter { chiSoKhung: number; boDem: GCounter; }
function chiSoKhung(thoiDiem: number, kichThuocKhungMs: number): number {
  return Math.floor(thoiDiem / kichThuocKhungMs);
}

function gomTheoKhungCRDT(cacSuKien: SuKien[], kichThuocKhungMs: number): Map<number, KhungGCounter> {
  return cacSuKien.reduce((khungHienCo, sk) => {
    const idx = chiSoKhung(sk.thoiDiem, kichThuocKhungMs);
    const khungCu = khungHienCo.get(idx) ?? { chiSoKhung: idx, boDem: counterRong() };
    const khungMoi: KhungGCounter = { chiSoKhung: idx, boDem: tangGCounter(khungCu.boDem, sk.nguon, sk.soLuong) };
    const banSaoMoi = new Map(khungHienCo);
    banSaoMoi.set(idx, khungMoi);
    return banSaoMoi;
  }, new Map<number, KhungGCounter>());
}

function hopNhatKhungGCounter(a: Map<number, KhungGCounter>, b: Map<number, KhungGCounter>): Map<number, KhungGCounter> {
  const ketQua = new Map(a);
  for (const [idx, khungB] of b) {
    const khungA = ketQua.get(idx);
    if (khungA === undefined) {
      ketQua.set(idx, khungB);
    } else {
      ketQua.set(idx, { chiSoKhung: idx, boDem: hopNhatGCounter(khungA.boDem, khungB.boDem) });
    }
  }
  return ketQua;
}

const suKien = (id: string, nguon: string, thoiDiem: number, soLuong: number): SuKien => ({ id, nguon, thoiDiem, soLuong });

// server "edge-1" tu gom cac su kien CUA CHINH NO
const suKienEdge1: SuKien[] = [
  suKien("a1", "edge-1", 1000, 5),
  suKien("a2", "edge-1", 2000, 3),
  suKien("a3", "edge-1", 12000, 10),
];
const khungEdge1 = gomTheoKhungCRDT(suKienEdge1, 10000);

// server "edge-2" doc lap, KHONG biet edge-1 ton tai
const suKienEdge2: SuKien[] = [
  suKien("b1", "edge-2", 3000, 7),
  suKien("b2", "edge-2", 15000, 20),
];
const khungEdge2 = gomTheoKhungCRDT(suKienEdge2, 10000);

console.log("khung 0 cua edge-1:", JSON.stringify(khungEdge1.get(0)?.boDem.theoReplica));
console.log("khung 0 cua edge-2:", JSON.stringify(khungEdge2.get(0)?.boDem.theoReplica));

const hopNhat = hopNhatKhungGCounter(khungEdge1, khungEdge2);
console.log("khung 0 SAU hop nhat:", JSON.stringify(hopNhat.get(0)?.boDem.theoReplica));
console.log("gia tri khung 0 (tong luot xem):", giaTriGCounter(hopNhat.get(0)!.boDem));
console.log("khung 1 SAU hop nhat:", JSON.stringify(hopNhat.get(1)?.boDem.theoReplica));
console.log("gia tri khung 1:", giaTriGCounter(hopNhat.get(1)!.boDem));

console.log("--- hop nhat LAI mot lan nua (mo phong dong bo lap) khong doi gia tri ---");
const hopNhatLai = hopNhatKhungGCounter(hopNhat, khungEdge1);
console.log("gia tri khung 0 sau khi hop nhat LAP:", giaTriGCounter(hopNhatLai.get(0)!.boDem));
```

```text title=readonly
khung 0 cua edge-1: {"edge-1":8}
khung 0 cua edge-2: {"edge-2":7}
khung 0 SAU hop nhat: {"edge-1":8,"edge-2":7}
gia tri khung 0 (tong luot xem): 15
khung 1 SAU hop nhat: {"edge-1":10,"edge-2":20}
gia tri khung 1: 30
--- hop nhat LAI mot lan nua (mo phong dong bo lap) khong doi gia tri ---
gia tri khung 0 sau khi hop nhat LAP: 15
```

`edge-1` VÀ `edge-2` không hề biết nhau — mỗi bên gom khung `0`
BẰNG một `GCounter` chỉ có Ô của CHÍNH nó. Hợp nhất bằng
`hopNhatGCounter` KHÔNG cộng hai object lại, mà lấy `theoReplica`
CỦA CẢ hai (`{"edge-1":8, "edge-2":7}`) — tổng đúng LÀ `15`, không
đếm trùng phần nào. Hợp nhất KẾT QUẢ đó với `khungEdge1` một LẦN nữa
(mô phỏng một chu kỳ đồng bộ định kỳ lặp lại) vẫn cho ĐÚNG `15` — nhờ
`Math.max` bên trong `hopNhatGCounter`, hợp nhất lặp lại KHÔNG làm
tăng giá trị.
::::

::::example{#thu-tu-hop-nhat-khong-quan-trong}
Vì `hopNhatGCounter` là giao hoán VÀ kết hợp (đã chứng minh Ở quest
"CRDT và hợp nhất"), `hopNhatKhungGCounter` thừa hưởng ĐÚNG hai tính
chất đó — hợp nhất ba nguồn theo BẤT KỲ thứ tự nào đều ra CÙNG một
tổng:

```typescript title=readonly
interface GCounter { theoReplica: Record<string, number>; }
function counterRong(): GCounter { return { theoReplica: {} }; }
function tangGCounter(gc: GCounter, replicaId: string, buoc: number): GCounter {
  const hienTai = gc.theoReplica[replicaId] ?? 0;
  return { theoReplica: { ...gc.theoReplica, [replicaId]: hienTai + buoc } };
}
function giaTriGCounter(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
function hopNhatGCounter(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KhungGCounter { chiSoKhung: number; boDem: GCounter; }
function chiSoKhung(thoiDiem: number, kichThuocKhungMs: number): number {
  return Math.floor(thoiDiem / kichThuocKhungMs);
}
function gomTheoKhungCRDT(cacSuKien: SuKien[], kichThuocKhungMs: number): Map<number, KhungGCounter> {
  return cacSuKien.reduce((khungHienCo, sk) => {
    const idx = chiSoKhung(sk.thoiDiem, kichThuocKhungMs);
    const khungCu = khungHienCo.get(idx) ?? { chiSoKhung: idx, boDem: counterRong() };
    const khungMoi: KhungGCounter = { chiSoKhung: idx, boDem: tangGCounter(khungCu.boDem, sk.nguon, sk.soLuong) };
    const banSaoMoi = new Map(khungHienCo);
    banSaoMoi.set(idx, khungMoi);
    return banSaoMoi;
  }, new Map<number, KhungGCounter>());
}
function hopNhatKhungGCounter(a: Map<number, KhungGCounter>, b: Map<number, KhungGCounter>): Map<number, KhungGCounter> {
  const ketQua = new Map(a);
  for (const [idx, khungB] of b) {
    const khungA = ketQua.get(idx);
    if (khungA === undefined) {
      ketQua.set(idx, khungB);
    } else {
      ketQua.set(idx, { chiSoKhung: idx, boDem: hopNhatGCounter(khungA.boDem, khungB.boDem) });
    }
  }
  return ketQua;
}
function tongTatCaKhung(m: Map<number, KhungGCounter>): number {
  let tong = 0;
  for (const k of m.values()) tong += giaTriGCounter(k.boDem);
  return tong;
}

const suKien = (id: string, nguon: string, thoiDiem: number, soLuong: number): SuKien => ({ id, nguon, thoiDiem, soLuong });

const khungA = gomTheoKhungCRDT([suKien("a1", "s1", 500, 4), suKien("a2", "s1", 11000, 6)], 10000);
const khungB = gomTheoKhungCRDT([suKien("b1", "s2", 800, 9), suKien("b2", "s2", 12000, 1)], 10000);
const khungC = gomTheoKhungCRDT([suKien("c1", "s3", 1500, 2)], 10000);

const thuTu1 = hopNhatKhungGCounter(hopNhatKhungGCounter(khungA, khungB), khungC);
const thuTu2 = hopNhatKhungGCounter(khungC, hopNhatKhungGCounter(khungB, khungA));
console.log("tong tat ca khung (thu tu 1: (A hop B) hop C):", tongTatCaKhung(thuTu1));
console.log("tong tat ca khung (thu tu 2: C hop (B hop A)):", tongTatCaKhung(thuTu2));
console.log("hai thu tu hop nhat cho CUNG tong?", tongTatCaKhung(thuTu1) === tongTatCaKhung(thuTu2));

console.log("khungA truoc hop nhat co bi doi khong?", JSON.stringify(Array.from(khungA.entries()).map(([i, k]) => [i, k.boDem.theoReplica])));
```

```text title=readonly
tong tat ca khung (thu tu 1: (A hop B) hop C): 22
tong tat ca khung (thu tu 2: C hop (B hop A)): 22
hai thu tu hop nhat cho CUNG tong? true
khungA truoc hop nhat co bi doi khong? [[0,{"s1":4}],[1,{"s1":6}]]
```

`4 + 9 + 2 + 6 + 1 = 22`, khớp Ở CẢ hai thứ tự nhóm hợp nhất hoàn toàn
khác nhau. `khungA` sau khi tham gia hai lời gọi `hopNhatKhungGCounter`
vẫn giữ nguyên nội dung — `hopNhatKhungGCounter` không mutate bất kỳ
tham số nào, giống hệt kỷ luật của `hopNhatGCounter` gốc.
::::

::::predict{#doan-hop-nhat-voi-chinh-no commitOnce}
`khungA` là kết quả `gomTheoKhungCRDT` từ một sự kiện DUY nhất
(`soLuong: 4`, nguồn `"s1"`, khung `0`). Gọi
`hopNhatKhungGCounter(khungA, khungA)` (hợp nhất `khungA` VỚI CHÍNH
NÓ). `giaTriGCounter` của khung `0` trong kết quả LÀ bao nhiêu?

:::opt{correct}
Vẫn là `4` — `hopNhatGCounter` bên trong dùng `Math.max` cho từng ô
`replicaId`; hợp nhất `khungA` với chính nó nghĩa là so `Math.max(4,
4)`, kết quả không đổi, KHÔNG cộng dồn thành `8`
:::
:::opt
`8` — vì hợp nhất LÀ một dạng gộp dữ liệu, VÀ dữ liệu từ CẢ hai phía
(dù là cùng một nguồn) đều phải được TÍNH vào tổng
::why
Nhầm "hợp nhất" với "cộng dồn" — đây chính LÀ SAI LẦM mà `GCounter`
được thiết kế để tránh (đã học Ở bài 1 của quest "CRDT và hợp nhất"):
nếu hai bản sao ĐÃ từng biết về nhau (Ở đây LÀ CÙNG một object), cộng
dồn sẽ đếm lại phần trùng.

Chỗ lệch: `hopNhatGCounter` dùng `ketQua[replicaId] =
Math.max(hienTai, soDem)`, không phải `+`. Với `khungA` hợp nhất
VỚI CHÍNH NÓ, cả hai bên đều có `theoReplica: {"s1": 4}` — vòng lặp
tính `Math.max(4, 4) = 4`. Đây chính LÀ tính chất IDEMPOTENT của
`GCounter`: hợp nhất một giá trị với chính nó (hay hợp nhất LẶP LẠI
nhiều lần, như trong `hopNhatLai` Ở đoạn `explain`) không bao giờ làm
tổng tăng thêm.
::
:::
::::

::::code{#viet_hop_nhat_khung_gcounter}
Hoàn thiện `hopNhatKhungGCounter` — sao chép `a` vào một `Map` MỚI
(`ketQua`). Với mỗi cặp `[idx, khungB]` trong `b`: nếu `ketQua` CHƯA
có khung Ở `idx` đó, đặt thẳng `khungB` vào; nếu ĐÃ có (`khungA`), đặt
một `KhungGCounter` MỚI với `boDem` LÀ `hopNhatGCounter(khungA.boDem,
khungB.boDem)`. Trả về `ketQua`.

```typescript title=starter
interface GCounter { theoReplica: Record<string, number>; }
function counterRong(): GCounter { return { theoReplica: {} }; }
function tangGCounter(gc: GCounter, replicaId: string, buoc: number): GCounter {
  const hienTai = gc.theoReplica[replicaId] ?? 0;
  return { theoReplica: { ...gc.theoReplica, [replicaId]: hienTai + buoc } };
}
function giaTriGCounter(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
function hopNhatGCounter(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KhungGCounter { chiSoKhung: number; boDem: GCounter; }
function chiSoKhung(thoiDiem: number, kichThuocKhungMs: number): number {
  return Math.floor(thoiDiem / kichThuocKhungMs);
}
function gomTheoKhungCRDT(cacSuKien: SuKien[], kichThuocKhungMs: number): Map<number, KhungGCounter> {
  return cacSuKien.reduce((khungHienCo, sk) => {
    const idx = chiSoKhung(sk.thoiDiem, kichThuocKhungMs);
    const khungCu = khungHienCo.get(idx) ?? { chiSoKhung: idx, boDem: counterRong() };
    const khungMoi: KhungGCounter = { chiSoKhung: idx, boDem: tangGCounter(khungCu.boDem, sk.nguon, sk.soLuong) };
    const banSaoMoi = new Map(khungHienCo);
    banSaoMoi.set(idx, khungMoi);
    return banSaoMoi;
  }, new Map<number, KhungGCounter>());
}

function hopNhatKhungGCounter(a: Map<number, KhungGCounter>, b: Map<number, KhungGCounter>): Map<number, KhungGCounter> {
  ___
}

const khungX = gomTheoKhungCRDT([{ id: "x1", nguon: "s1", thoiDiem: 100, soLuong: 3 }], 1000);
const khungY = gomTheoKhungCRDT([{ id: "y1", nguon: "s2", thoiDiem: 200, soLuong: 5 }], 1000);
const hopX = hopNhatKhungGCounter(khungX, khungY);
console.log(giaTriGCounter(hopX.get(0)!.boDem), hopX.size);
```

```typescript title=solution
interface GCounter { theoReplica: Record<string, number>; }
function counterRong(): GCounter { return { theoReplica: {} }; }
function tangGCounter(gc: GCounter, replicaId: string, buoc: number): GCounter {
  const hienTai = gc.theoReplica[replicaId] ?? 0;
  return { theoReplica: { ...gc.theoReplica, [replicaId]: hienTai + buoc } };
}
function giaTriGCounter(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
function hopNhatGCounter(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KhungGCounter { chiSoKhung: number; boDem: GCounter; }
function chiSoKhung(thoiDiem: number, kichThuocKhungMs: number): number {
  return Math.floor(thoiDiem / kichThuocKhungMs);
}
function gomTheoKhungCRDT(cacSuKien: SuKien[], kichThuocKhungMs: number): Map<number, KhungGCounter> {
  return cacSuKien.reduce((khungHienCo, sk) => {
    const idx = chiSoKhung(sk.thoiDiem, kichThuocKhungMs);
    const khungCu = khungHienCo.get(idx) ?? { chiSoKhung: idx, boDem: counterRong() };
    const khungMoi: KhungGCounter = { chiSoKhung: idx, boDem: tangGCounter(khungCu.boDem, sk.nguon, sk.soLuong) };
    const banSaoMoi = new Map(khungHienCo);
    banSaoMoi.set(idx, khungMoi);
    return banSaoMoi;
  }, new Map<number, KhungGCounter>());
}

function hopNhatKhungGCounter(a: Map<number, KhungGCounter>, b: Map<number, KhungGCounter>): Map<number, KhungGCounter> {
  const ketQua = new Map(a);
  for (const [idx, khungB] of b) {
    const khungA = ketQua.get(idx);
    if (khungA === undefined) {
      ketQua.set(idx, khungB);
    } else {
      ketQua.set(idx, { chiSoKhung: idx, boDem: hopNhatGCounter(khungA.boDem, khungB.boDem) });
    }
  }
  return ketQua;
}

const khungX = gomTheoKhungCRDT([{ id: "x1", nguon: "s1", thoiDiem: 100, soLuong: 3 }], 1000);
const khungY = gomTheoKhungCRDT([{ id: "y1", nguon: "s2", thoiDiem: 200, soLuong: 5 }], 1000);
const hopX = hopNhatKhungGCounter(khungX, khungY);
console.log(giaTriGCounter(hopX.get(0)!.boDem), hopX.size);
```

```typescript title=test
const tKhungA = gomTheoKhungCRDT([{ id: "a1", nguon: "sa", thoiDiem: 100, soLuong: 10 }], 1000);
const tKhungB = gomTheoKhungCRDT([{ id: "b1", nguon: "sb", thoiDiem: 200, soLuong: 4 }, { id: "b2", nguon: "sb", thoiDiem: 1500, soLuong: 7 }], 1000);

const tHop = hopNhatKhungGCounter(tKhungA, tKhungB);
const tGiaTriKhung0 = giaTriGCounter(tHop.get(0)!.boDem);
if (tGiaTriKhung0 !== 14) throw new Error("khung 0 phai la 10 (sa) + 4 (sb) = 14");
const tGiaTriKhung1 = giaTriGCounter(tHop.get(1)!.boDem);
if (tGiaTriKhung1 !== 7) throw new Error("khung 1 chi co dong gop cua sb (7)");

const tHopNguocLai = hopNhatKhungGCounter(tKhungB, tKhungA);
if (giaTriGCounter(tHopNguocLai.get(0)!.boDem) !== tGiaTriKhung0) throw new Error("hop nhat phai GIAO HOAN -- thu tu khong duoc lam thay doi ket qua");

const tHopVoiChinhNo = hopNhatKhungGCounter(tHop, tKhungA);
if (giaTriGCounter(tHopVoiChinhNo.get(0)!.boDem) !== tGiaTriKhung0) throw new Error("hop nhat LAP LAI voi mot nguon da co KHONG duoc lam tang gia tri (idempotent qua MAX)");

const tChuoiTruocA = JSON.stringify(Array.from(tKhungA.entries()).map(([i, k]) => [i, k.boDem.theoReplica]));
hopNhatKhungGCounter(tKhungA, tKhungB);
const tChuoiSauA = JSON.stringify(Array.from(tKhungA.entries()).map(([i, k]) => [i, k.boDem.theoReplica]));
if (tChuoiTruocA !== tChuoiSauA) throw new Error("hopNhatKhungGCounter KHONG duoc mutate tham so a truyen vao");
```

:::hints
- kind: attention
  body: "const ketQua = new Map(a). Duyet for (const [idx, khungB] of b): neu ketQua.get(idx) la undefined thi ketQua.set(idx, khungB); nguoc lai ketQua.set(idx, { chiSoKhung: idx, boDem: hopNhatGCounter(khungA.boDem, khungB.boDem) }) voi khungA la ket qua cua ketQua.get(idx). Tra ve ketQua."
- kind: strategy
  body: "const ketQua = new Map(a); for (const [idx, khungB] of b) { const khungA = ketQua.get(idx); if (khungA === undefined) { ketQua.set(idx, khungB); } else { ketQua.set(idx, { chiSoKhung: idx, boDem: hopNhatGCounter(khungA.boDem, khungB.boDem) }); } } return ketQua;"
- kind: one-line
  body: "const ketQua = new Map(a); for (const [idx, khungB] of b) { const khungA = ketQua.get(idx); if (khungA === undefined) { ketQua.set(idx, khungB); } else { ketQua.set(idx, { chiSoKhung: idx, boDem: hopNhatGCounter(khungA.boDem, khungB.boDem) }); } } return ketQua;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "8 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Windowing giờ hợp nhất được qua nhiều nguồn, không đếm trùng, hội tụ
bất kể thứ tự — đúng tinh thần CRDT áp dụng vào từng khung thời gian.
Nhưng windowing, batching, backpressure vẫn đang là ba mảnh RIÊNG LẺ —
đã đến lúc ráp chúng thành MỘT pipeline.
::::

::::reflect{#nghi-lai}
`gomTheoKhungCRDT` VÀ `hopNhatKhungGCounter` không phát minh phép toán
MỚI nào — chúng chỉ ĐẶT một `GCounter` đã kiểm chứng đầy đủ (giao
hoán, kết hợp, idempotent — quest "CRDT và hợp nhất") vào TRONG một
cấu trúc `Map` theo khung đã kiểm chứng riêng (bài 1 của quest này).
Khi hai khối đã đúng được LẮP vào nhau mà không sửa logic bên trong
của khối nào, khối GHÉP thừa hưởng tính đúng đắn của cả hai — đây
chính LÀ giá trị của việc tách bạch từng mảnh thuần trước khi ráp
chúng lại.
::::

::::checkpoint{mastery=0.83}
::::
