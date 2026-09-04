---
id: thiet-ke-he-thong.crdt-va-hop-nhat.boss-crdt-va-hop-nhat
title: "BOSS — dịch vụ reaction phân tán: PN-Counter + OR-Set + LWW-Register"
summary: "interface TrangThaiReaction { soLuotLike: PNCounter; nguoiDaReact: ORSet; tieuDe: LWWRegister } -- hopNhat rap DUNG ba manh: hopNhatPN cho so luot like, hopNhatOR cho danh sach nguoi da react, hopNhatLWW cho tieu de hien thi cuoi cung; ba replica r1 (like 3, react an+xoa binh, tieuDe t=10), r2 (like 2, react chi, tieuDe t=5), r3 (giam like 1, react dung, tieuDe t=10 hoa voi r1) hop nhat theo BA thu tu khac nhau deu hoi tu ve DUNG {like:4, react:'an,chi,dung', tieuDe:'Bai viet ve CRDT (chinh thuc)'}."
locale: vi
track: thiet-ke-he-thong
module: crdt-va-hop-nhat
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [sd.fp.boss-crdt-va-hop-nhat]
requires: [sd.fp.bo-dem-luot-xem-phan-tan]
concepts: [sd.fp.boss-crdt-va-hop-nhat]
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
Chín bài — G-Counter, hai luật monoid đo được bằng test, idempotent
miễn phí từ `Math.max`, PN-Counter tách tăng/giảm, G-Set, OR-Set với
tag riêng cho từng lần thêm, LWW-Register với tie-break, ba chính
sách so kề nhau, bộ đếm phân tán qua nhiều edge server. Giờ ráp cả
ba loại CRDT khác nhau — đếm, tập hợp, giá trị đơn — vào MỘT dịch vụ
nhỏ: hệ thống "reaction" cho một bài viết, chạy trên nhiều replica.
::::

::::explain{#rap_ba_crdt_thanh_mot_trang_thai}
`TrangThaiReaction` gộp ba CRDT: `soLuotLike` (đếm lượt thích, có thể
huỷ — `PNCounter`), `nguoiDaReact` (ai đã react — `ORSet`), `tieuDe`
(tên hiển thị mới nhất — `LWWRegister`). `hopNhat` của toàn bộ trạng
thái chỉ đơn giản gọi ĐÚNG phép hợp nhất riêng của TỪNG phần — không
có logic nào MỚI phát sinh khi ba CRDT được gộp chung:

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
interface PNCounter { tang: GCounter; giam: GCounter; }
function pnCounterRong(): PNCounter { return { tang: counterRong(), giam: counterRong() }; }
function pnTang(pn: PNCounter, replicaId: string, buoc: number): PNCounter {
  return { tang: tangGCounter(pn.tang, replicaId, buoc), giam: pn.giam };
}
function pnGiam(pn: PNCounter, replicaId: string, buoc: number): PNCounter {
  return { tang: pn.tang, giam: tangGCounter(pn.giam, replicaId, buoc) };
}
function giaTriPN(pn: PNCounter): number {
  return giaTriGCounter(pn.tang) - giaTriGCounter(pn.giam);
}
function hopNhatPN(a: PNCounter, b: PNCounter): PNCounter {
  return { tang: hopNhatGCounter(a.tang, b.tang), giam: hopNhatGCounter(a.giam, b.giam) };
}
interface PhanTuOR { giaTri: string; tag: string; }
interface ORSet { themVao: PhanTuOR[]; xoaDi: Set<string>; }
function orSetRong(): ORSet { return { themVao: [], xoaDi: new Set() }; }
function taoTag(replicaId: string, soThuTu: number): string { return replicaId + "-" + soThuTu; }
function themOR(os: ORSet, giaTri: string, tag: string): ORSet {
  return { themVao: [...os.themVao, { giaTri, tag }], xoaDi: os.xoaDi };
}
function phanTuHienDien(os: ORSet): PhanTuOR[] {
  return os.themVao.filter((pt) => !os.xoaDi.has(pt.tag));
}
function giaTriHienDienOR(os: ORSet): string {
  const tapHop = new Set(phanTuHienDien(os).map((pt) => pt.giaTri));
  return Array.from(tapHop).sort().join(",");
}
function xoaOR(os: ORSet, giaTri: string): ORSet {
  const tagCanXoa = phanTuHienDien(os).filter((pt) => pt.giaTri === giaTri).map((pt) => pt.tag);
  return { themVao: os.themVao, xoaDi: new Set([...os.xoaDi, ...tagCanXoa]) };
}
function hopNhatOR(a: ORSet, b: ORSet): ORSet {
  const theoTag = new Map<string, PhanTuOR>();
  for (const pt of a.themVao) theoTag.set(pt.tag, pt);
  for (const pt of b.themVao) theoTag.set(pt.tag, pt);
  return { themVao: Array.from(theoTag.values()), xoaDi: new Set([...a.xoaDi, ...b.xoaDi]) };
}
interface LWWRegister { giaTri: string; thoiDiem: number; replicaId: string; }
function ghiLWW(giaTriMoi: string, thoiDiem: number, replicaId: string): LWWRegister {
  return { giaTri: giaTriMoi, thoiDiem, replicaId };
}
function hopNhatLWW(a: LWWRegister, b: LWWRegister): LWWRegister {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a : b;
  return a.replicaId > b.replicaId ? a : b;
}

interface TrangThaiReaction { soLuotLike: PNCounter; nguoiDaReact: ORSet; tieuDe: LWWRegister; }
function trangThaiRong(): TrangThaiReaction {
  return { soLuotLike: pnCounterRong(), nguoiDaReact: orSetRong(), tieuDe: ghiLWW("(chua co tieu de)", 0, "") };
}
function hopNhat(a: TrangThaiReaction, b: TrangThaiReaction): TrangThaiReaction {
  return {
    soLuotLike: hopNhatPN(a.soLuotLike, b.soLuotLike),
    nguoiDaReact: hopNhatOR(a.nguoiDaReact, b.nguoiDaReact),
    tieuDe: hopNhatLWW(a.tieuDe, b.tieuDe),
  };
}
function tomTat(t: TrangThaiReaction): string {
  return JSON.stringify({ like: giaTriPN(t.soLuotLike), react: giaTriHienDienOR(t.nguoiDaReact), tieuDe: t.tieuDe.giaTri });
}

let r1 = trangThaiRong();
r1.soLuotLike = pnTang(r1.soLuotLike, "r1", 3);
r1.nguoiDaReact = themOR(r1.nguoiDaReact, "an", taoTag("r1", 1));
r1.nguoiDaReact = themOR(r1.nguoiDaReact, "binh", taoTag("r1", 2));
r1.tieuDe = ghiLWW("Bai viet ve CRDT", 10, "r1");
r1.nguoiDaReact = xoaOR(r1.nguoiDaReact, "binh");

let r2 = trangThaiRong();
r2.soLuotLike = pnTang(r2.soLuotLike, "r2", 2);
r2.nguoiDaReact = themOR(r2.nguoiDaReact, "chi", taoTag("r2", 1));
r2.tieuDe = ghiLWW("Bai viet ve CRDT (ban nhap)", 5, "r2");

let r3 = trangThaiRong();
r3.soLuotLike = pnGiam(r3.soLuotLike, "r3", 1);
r3.nguoiDaReact = themOR(r3.nguoiDaReact, "dung", taoTag("r3", 1));
r3.tieuDe = ghiLWW("Bai viet ve CRDT (chinh thuc)", 10, "r3");

console.log("r1:", tomTat(r1));
console.log("r2:", tomTat(r2));
console.log("r3:", tomTat(r3));

const cach1 = hopNhat(hopNhat(r1, r2), r3);
const cach2 = hopNhat(hopNhat(r2, r3), r1);
const cach3 = hopNhat(r3, hopNhat(r1, r2));
console.log("cach1 hopNhat(hopNhat(r1,r2),r3):", tomTat(cach1));
console.log("cach2 hopNhat(hopNhat(r2,r3),r1):", tomTat(cach2));
console.log("cach3 hopNhat(r3,hopNhat(r1,r2)):", tomTat(cach3));
console.log("ca ba GIONG HET nhau?", tomTat(cach1) === tomTat(cach2) && tomTat(cach2) === tomTat(cach3));
```

```text title=readonly
r1: {"like":3,"react":"an","tieuDe":"Bai viet ve CRDT"}
r2: {"like":2,"react":"chi","tieuDe":"Bai viet ve CRDT (ban nhap)"}
r3: {"like":-1,"react":"dung","tieuDe":"Bai viet ve CRDT (chinh thuc)"}
cach1 hopNhat(hopNhat(r1,r2),r3): {"like":4,"react":"an,chi,dung","tieuDe":"Bai viet ve CRDT (chinh thuc)"}
cach2 hopNhat(hopNhat(r2,r3),r1): {"like":4,"react":"an,chi,dung","tieuDe":"Bai viet ve CRDT (chinh thuc)"}
cach3 hopNhat(r3,hopNhat(r1,r2)): {"like":4,"react":"an,chi,dung","tieuDe":"Bai viet ve CRDT (chinh thuc)"}
ca ba GIONG HET nhau? true
```

`r1` thêm "an" VÀ "binh" rồi tự xoá "binh" — chỉ còn "an" hiện diện
Ở `r1`. `r2` chỉ có "chi". `r3` GIẢM `1` lượt like (không hề tăng
trước đó — giá trị riêng của `r3` là `-1`, hoàn toàn hợp lệ CỤC BỘ,
vì nó chỉ phản ánh phần `r3` đã đóng góp). Ba cách nhóm phép hợp
nhất — `(r1 hợp r2) hợp r3`, `(r2 hợp r3) hợp r1`, `r3 hợp (r1 hợp
r2)` — đều hội tụ về ĐÚNG MỘT kết quả: `like: 4` (`3 + 2 - 1`),
`react: "an,chi,dung"` (không có "binh", đã bị xoá VÀ hội tụ đúng
qua `hopNhatOR`), `tieuDe` là bản của `r3` (`thoiDiem: 10`, hoà với
`r1` cũng `thoiDiem: 10`, tie-break `"r3" > "r1"` theo thứ tự ký tự).
::::

::::example{#tung_phan_doc_lap_cho_dung_ket_qua}
Vì `hopNhat` của `TrangThaiReaction` không làm gì ngoài gọi ba phép
hợp nhất con, gọi TRỰC TIẾP từng phép hợp nhất riêng (không qua
`TrangThaiReaction`) phải cho ra ĐÚNG những con số đã thấy Ở trên:

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
interface PNCounter { tang: GCounter; giam: GCounter; }
function pnCounterRong(): PNCounter { return { tang: counterRong(), giam: counterRong() }; }
function pnTang(pn: PNCounter, replicaId: string, buoc: number): PNCounter {
  return { tang: tangGCounter(pn.tang, replicaId, buoc), giam: pn.giam };
}
function pnGiam(pn: PNCounter, replicaId: string, buoc: number): PNCounter {
  return { tang: pn.tang, giam: tangGCounter(pn.giam, replicaId, buoc) };
}
function giaTriPN(pn: PNCounter): number {
  return giaTriGCounter(pn.tang) - giaTriGCounter(pn.giam);
}
function hopNhatPN(a: PNCounter, b: PNCounter): PNCounter {
  return { tang: hopNhatGCounter(a.tang, b.tang), giam: hopNhatGCounter(a.giam, b.giam) };
}
interface PhanTuOR { giaTri: string; tag: string; }
interface ORSet { themVao: PhanTuOR[]; xoaDi: Set<string>; }
function orSetRong(): ORSet { return { themVao: [], xoaDi: new Set() }; }
function taoTag(replicaId: string, soThuTu: number): string { return replicaId + "-" + soThuTu; }
function themOR(os: ORSet, giaTri: string, tag: string): ORSet {
  return { themVao: [...os.themVao, { giaTri, tag }], xoaDi: os.xoaDi };
}
function phanTuHienDien(os: ORSet): PhanTuOR[] {
  return os.themVao.filter((pt) => !os.xoaDi.has(pt.tag));
}
function giaTriHienDienOR(os: ORSet): string {
  const tapHop = new Set(phanTuHienDien(os).map((pt) => pt.giaTri));
  return Array.from(tapHop).sort().join(",");
}
function xoaOR(os: ORSet, giaTri: string): ORSet {
  const tagCanXoa = phanTuHienDien(os).filter((pt) => pt.giaTri === giaTri).map((pt) => pt.tag);
  return { themVao: os.themVao, xoaDi: new Set([...os.xoaDi, ...tagCanXoa]) };
}
function hopNhatOR(a: ORSet, b: ORSet): ORSet {
  const theoTag = new Map<string, PhanTuOR>();
  for (const pt of a.themVao) theoTag.set(pt.tag, pt);
  for (const pt of b.themVao) theoTag.set(pt.tag, pt);
  return { themVao: Array.from(theoTag.values()), xoaDi: new Set([...a.xoaDi, ...b.xoaDi]) };
}
interface LWWRegister { giaTri: string; thoiDiem: number; replicaId: string; }
function ghiLWW(giaTriMoi: string, thoiDiem: number, replicaId: string): LWWRegister {
  return { giaTri: giaTriMoi, thoiDiem, replicaId };
}
function hopNhatLWW(a: LWWRegister, b: LWWRegister): LWWRegister {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a : b;
  return a.replicaId > b.replicaId ? a : b;
}

let r1 = trangThaiRong_choViDu();
function trangThaiRong_choViDu(): { soLuotLike: PNCounter; nguoiDaReact: ORSet; tieuDe: LWWRegister } {
  return { soLuotLike: pnCounterRong(), nguoiDaReact: orSetRong(), tieuDe: ghiLWW("(chua co tieu de)", 0, "") };
}
r1.soLuotLike = pnTang(r1.soLuotLike, "r1", 3);
r1.nguoiDaReact = themOR(r1.nguoiDaReact, "an", taoTag("r1", 1));
r1.nguoiDaReact = themOR(r1.nguoiDaReact, "binh", taoTag("r1", 2));
r1.tieuDe = ghiLWW("Bai viet ve CRDT", 10, "r1");
r1.nguoiDaReact = xoaOR(r1.nguoiDaReact, "binh");

let r2 = trangThaiRong_choViDu();
r2.soLuotLike = pnTang(r2.soLuotLike, "r2", 2);
r2.nguoiDaReact = themOR(r2.nguoiDaReact, "chi", taoTag("r2", 1));
r2.tieuDe = ghiLWW("Bai viet ve CRDT (ban nhap)", 5, "r2");

let r3 = trangThaiRong_choViDu();
r3.soLuotLike = pnGiam(r3.soLuotLike, "r3", 1);
r3.nguoiDaReact = themOR(r3.nguoiDaReact, "dung", taoTag("r3", 1));
r3.tieuDe = ghiLWW("Bai viet ve CRDT (chinh thuc)", 10, "r3");

console.log("giaTriPN rieng (khong qua trangThaiReaction):", giaTriPN(hopNhatPN(r1.soLuotLike, hopNhatPN(r2.soLuotLike, r3.soLuotLike))));
console.log("react hien dien rieng:", giaTriHienDienOR(hopNhatOR(r1.nguoiDaReact, hopNhatOR(r2.nguoiDaReact, r3.nguoiDaReact))));
console.log("tieuDe rieng:", JSON.stringify(hopNhatLWW(r1.tieuDe, hopNhatLWW(r2.tieuDe, r3.tieuDe))));
```

```text title=readonly
giaTriPN rieng (khong qua trangThaiReaction): 4
react hien dien rieng: an,chi,dung
tieuDe rieng: {"giaTri":"Bai viet ve CRDT (chinh thuc)","thoiDiem":10,"replicaId":"r3"}
```

Gọi `hopNhatPN`, `hopNhatOR`, `hopNhatLWW` trực tiếp trên đúng ba
cặp trường tương ứng của `r1`, `r2`, `r3` — không thông qua hàm
`hopNhat` của `TrangThaiReaction` — cho ra ĐÚNG các con số đã thấy Ở
`cach1`/`cach2`/`cach3`: `like` là `4`, `react` là `"an,chi,dung"`,
`tieuDe` là bản của `"r3"`. `hopNhat` của toàn bộ trạng thái không hề
thêm hay bớt logic nào — nó chỉ điều phối tới ba phép hợp nhất con
đã được kiểm chứng riêng Ở các bài trước.
::::

::::predict{#doan-tieude-hoa-thoi-diem commitOnce}
Ở `cach1`, `tieuDe` cuối cùng là bản của `"r3"` (`thoiDiem: 10`).
Nhưng `r1.tieuDe` CŨNG có `thoiDiem: 10`, CÙNG giá trị với `r3.tieuDe`.
Tại sao `"r3"` thắng, không phải `"r1"`?

:::opt{correct}
Vì hai `thoiDiem` BẰNG nhau (`10 === 10`), `hopNhatLWW` rơi xuống
nhánh tie-break: so `replicaId` theo thứ tự ký tự, `"r3"` LỚN hơn
`"r1"` (ký tự thứ hai: `"3"` đứng SAU `"1"`), nên `r3.tieuDe` thắng
:::
:::opt
Vì `r3` được xử lý sau CÙNG trong `cach1` (`hopNhat(hopNhat(r1,r2),r3)`
gọi `r3` Ở lượt cuối), VÀ tham số bên PHẢI luôn thắng khi có nhiều
hơn hai `TrangThaiReaction` được gộp liên tiếp
::why
Nhầm "thứ tự GỌI hàm" với "luật SO SÁNH bên trong hàm" — nhưng
`ca ba GIONG HET nhau?` Ở đoạn `explain` đã trả về `true`: `cach2`
(`hopNhat(hopNhat(r2,r3),r1)`, `r1` mới là tham số CUỐI) VÀ `cach3`
(`hopNhat(r3,hopNhat(r1,r2))`, `r3` là tham số ĐẦU) đều ra ĐÚNG kết
quả giống `cach1` — nếu "bên phải luôn thắng" là đúng, ba cách nhóm
đó phải cho ba `tieuDe` KHÁC nhau.

Chỗ lệch: `hopNhatLWW` không hề quan tâm THỨ TỰ tham số theo nghĩa
"vị trí trong lệnh gọi" — nó chỉ so `thoiDiem`, rồi (khi hoà) so
`replicaId`. Cả ba cách nhóm đều đi tới đúng CẶP `r1.tieuDe` VÀ
`r3.tieuDe` cần so sánh (cùng `thoiDiem: 10`), VÀ `"r3" > "r1"` luôn
đúng bất kể `hopNhatLWW` được gọi theo hướng nào. Đây chính là lý do
bài 7 nhấn mạnh tie-break phải DỰA vào nội dung (ở đây là `replicaId`),
không phải dựa vào vị trí tham số.
::
:::
::::

::::code{#viet_hop_nhat_trang_thai}
Hoàn thiện `hopNhat` cho `TrangThaiReaction` — hợp nhất RIÊNG từng
trường: `soLuotLike` bằng `hopNhatPN`, `nguoiDaReact` bằng
`hopNhatOR`, `tieuDe` bằng `hopNhatLWW`.

```typescript title=starter
interface GCounter { theoReplica: Record<string, number>; }
interface PNCounter { tang: GCounter; giam: GCounter; }
function hopNhatGCounter(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
function hopNhatPN(a: PNCounter, b: PNCounter): PNCounter {
  return { tang: hopNhatGCounter(a.tang, b.tang), giam: hopNhatGCounter(a.giam, b.giam) };
}
function giaTriGCounter(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
function giaTriPN(pn: PNCounter): number {
  return giaTriGCounter(pn.tang) - giaTriGCounter(pn.giam);
}
interface PhanTuOR { giaTri: string; tag: string; }
interface ORSet { themVao: PhanTuOR[]; xoaDi: Set<string>; }
function hopNhatOR(a: ORSet, b: ORSet): ORSet {
  const theoTag = new Map<string, PhanTuOR>();
  for (const pt of a.themVao) theoTag.set(pt.tag, pt);
  for (const pt of b.themVao) theoTag.set(pt.tag, pt);
  return { themVao: Array.from(theoTag.values()), xoaDi: new Set([...a.xoaDi, ...b.xoaDi]) };
}
function giaTriHienDienOR(os: ORSet): string {
  const tapHop = new Set(os.themVao.filter((pt) => !os.xoaDi.has(pt.tag)).map((pt) => pt.giaTri));
  return Array.from(tapHop).sort().join(",");
}
interface LWWRegister { giaTri: string; thoiDiem: number; replicaId: string; }
function hopNhatLWW(a: LWWRegister, b: LWWRegister): LWWRegister {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a : b;
  return a.replicaId > b.replicaId ? a : b;
}
interface TrangThaiReaction { soLuotLike: PNCounter; nguoiDaReact: ORSet; tieuDe: LWWRegister; }

function hopNhat(a: TrangThaiReaction, b: TrangThaiReaction): TrangThaiReaction {
  ___
}

const sA: TrangThaiReaction = {
  soLuotLike: { tang: { theoReplica: { sa: 4 } }, giam: { theoReplica: {} } },
  nguoiDaReact: { themVao: [{ giaTri: "x", tag: "sa-1" }], xoaDi: new Set() },
  tieuDe: { giaTri: "Tieu de A", thoiDiem: 3, replicaId: "sa" },
};
const sB: TrangThaiReaction = {
  soLuotLike: { tang: { theoReplica: { sb: 1 } }, giam: { theoReplica: {} } },
  nguoiDaReact: { themVao: [{ giaTri: "y", tag: "sb-1" }], xoaDi: new Set() },
  tieuDe: { giaTri: "Tieu de B", thoiDiem: 7, replicaId: "sb" },
};
const ketQuaCuoi = hopNhat(sA, sB);
console.log(giaTriPN(ketQuaCuoi.soLuotLike), giaTriHienDienOR(ketQuaCuoi.nguoiDaReact), ketQuaCuoi.tieuDe.giaTri);
```

```typescript title=solution
interface GCounter { theoReplica: Record<string, number>; }
interface PNCounter { tang: GCounter; giam: GCounter; }
function hopNhatGCounter(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
function hopNhatPN(a: PNCounter, b: PNCounter): PNCounter {
  return { tang: hopNhatGCounter(a.tang, b.tang), giam: hopNhatGCounter(a.giam, b.giam) };
}
function giaTriGCounter(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
function giaTriPN(pn: PNCounter): number {
  return giaTriGCounter(pn.tang) - giaTriGCounter(pn.giam);
}
interface PhanTuOR { giaTri: string; tag: string; }
interface ORSet { themVao: PhanTuOR[]; xoaDi: Set<string>; }
function hopNhatOR(a: ORSet, b: ORSet): ORSet {
  const theoTag = new Map<string, PhanTuOR>();
  for (const pt of a.themVao) theoTag.set(pt.tag, pt);
  for (const pt of b.themVao) theoTag.set(pt.tag, pt);
  return { themVao: Array.from(theoTag.values()), xoaDi: new Set([...a.xoaDi, ...b.xoaDi]) };
}
function giaTriHienDienOR(os: ORSet): string {
  const tapHop = new Set(os.themVao.filter((pt) => !os.xoaDi.has(pt.tag)).map((pt) => pt.giaTri));
  return Array.from(tapHop).sort().join(",");
}
interface LWWRegister { giaTri: string; thoiDiem: number; replicaId: string; }
function hopNhatLWW(a: LWWRegister, b: LWWRegister): LWWRegister {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a : b;
  return a.replicaId > b.replicaId ? a : b;
}
interface TrangThaiReaction { soLuotLike: PNCounter; nguoiDaReact: ORSet; tieuDe: LWWRegister; }

function hopNhat(a: TrangThaiReaction, b: TrangThaiReaction): TrangThaiReaction {
  return {
    soLuotLike: hopNhatPN(a.soLuotLike, b.soLuotLike),
    nguoiDaReact: hopNhatOR(a.nguoiDaReact, b.nguoiDaReact),
    tieuDe: hopNhatLWW(a.tieuDe, b.tieuDe),
  };
}

const sA: TrangThaiReaction = {
  soLuotLike: { tang: { theoReplica: { sa: 4 } }, giam: { theoReplica: {} } },
  nguoiDaReact: { themVao: [{ giaTri: "x", tag: "sa-1" }], xoaDi: new Set() },
  tieuDe: { giaTri: "Tieu de A", thoiDiem: 3, replicaId: "sa" },
};
const sB: TrangThaiReaction = {
  soLuotLike: { tang: { theoReplica: { sb: 1 } }, giam: { theoReplica: {} } },
  nguoiDaReact: { themVao: [{ giaTri: "y", tag: "sb-1" }], xoaDi: new Set() },
  tieuDe: { giaTri: "Tieu de B", thoiDiem: 7, replicaId: "sb" },
};
const ketQuaCuoi = hopNhat(sA, sB);
console.log(giaTriPN(ketQuaCuoi.soLuotLike), giaTriHienDienOR(ketQuaCuoi.nguoiDaReact), ketQuaCuoi.tieuDe.giaTri);
```

```typescript title=test
const cX: TrangThaiReaction = {
  soLuotLike: { tang: { theoReplica: { x: 10 } }, giam: { theoReplica: { x: 2 } } },
  nguoiDaReact: { themVao: [{ giaTri: "p", tag: "x-1" }, { giaTri: "q", tag: "x-2" }], xoaDi: new Set(["x-2"]) },
  tieuDe: { giaTri: "tieu de X", thoiDiem: 4, replicaId: "x" },
};
const cY: TrangThaiReaction = {
  soLuotLike: { tang: { theoReplica: { y: 3 } }, giam: { theoReplica: {} } },
  nguoiDaReact: { themVao: [{ giaTri: "r", tag: "y-1" }], xoaDi: new Set() },
  tieuDe: { giaTri: "tieu de Y", thoiDiem: 9, replicaId: "y" },
};
const kqXY = hopNhat(cX, cY);
if (giaTriPN(kqXY.soLuotLike) !== 11) throw new Error("like phai la (10 + 3) - 2 = 11");
if (giaTriHienDienOR(kqXY.nguoiDaReact) !== "p,r") throw new Error("react hien dien phai la p,r -- q da bi xoa");
if (kqXY.tieuDe.giaTri !== "tieu de Y") throw new Error("tieuDe phai la ban co thoiDiem lon hon (9 > 4)");

const kqYX = hopNhat(cY, cX);
if (giaTriPN(kqYX.soLuotLike) !== giaTriPN(kqXY.soLuotLike)) throw new Error("hopNhat(cY,cX) phai giao hoan voi hopNhat(cX,cY) -- like phai giong nhau");
if (giaTriHienDienOR(kqYX.nguoiDaReact) !== giaTriHienDienOR(kqXY.nguoiDaReact)) throw new Error("hopNhat(cY,cX) phai giao hoan -- react phai giong nhau");
if (kqYX.tieuDe.giaTri !== kqXY.tieuDe.giaTri) throw new Error("hopNhat(cY,cX) phai giao hoan -- tieuDe phai giong nhau");

const truocX = JSON.stringify({ soLuotLike: cX.soLuotLike, nguoiDaReact: { tv: cX.nguoiDaReact.themVao, xd: Array.from(cX.nguoiDaReact.xoaDi) }, tieuDe: cX.tieuDe });
hopNhat(cX, cY);
const sauX = JSON.stringify({ soLuotLike: cX.soLuotLike, nguoiDaReact: { tv: cX.nguoiDaReact.themVao, xd: Array.from(cX.nguoiDaReact.xoaDi) }, tieuDe: cX.tieuDe });
if (truocX !== sauX) throw new Error("hopNhat KHONG duoc mutate a truyen vao");
```

:::hints
- kind: attention
  body: "Tra ve mot TrangThaiReaction MOI voi ba truong, moi truong goi DUNG ham hop nhat rieng cua no: soLuotLike dung hopNhatPN, nguoiDaReact dung hopNhatOR, tieuDe dung hopNhatLWW. Khong tron lan cac ham voi nhau."
- kind: strategy
  body: "return { soLuotLike: hopNhatPN(a.soLuotLike, b.soLuotLike), nguoiDaReact: hopNhatOR(a.nguoiDaReact, b.nguoiDaReact), tieuDe: hopNhatLWW(a.tieuDe, b.tieuDe) };"
- kind: one-line
  body: "return { soLuotLike: hopNhatPN(a.soLuotLike, b.soLuotLike), nguoiDaReact: hopNhatOR(a.nguoiDaReact, b.nguoiDaReact), tieuDe: hopNhatLWW(a.tieuDe, b.tieuDe) };"
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "5 x,y Tieu de B"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba CRDT hoàn toàn khác nhau — đếm, tập hợp, giá trị đơn — ráp vào
MỘT trạng thái, hợp nhất theo BẤT KỲ thứ tự hay cặp nào cũng hội tụ
về đúng MỘT kết quả. Quest "CRDT và hợp nhất" khép lại — nền đã dọn
xong cho quest tiếp theo: streaming và hiệu ứng Ở biên, nơi lõi vẫn
thuần nhưng dữ liệu không còn đứng yên chờ hợp nhất, mà chảy liên
tục.
::::

::::reflect{#nghi-lai}
`hopNhat` của `TrangThaiReaction` không phát minh gì mới — nó ráp
đúng ba phép hợp nhất đã xây VÀ kiểm chứng riêng Ở các bài trước,
mỗi phép áp dụng cho ĐÚNG một trường. Đây chính là điều một `Monoid`
tổng hợp (composite) làm được mà một hệ OOP mutation khó đạt tới
cùng mức độ tin cậy: khi TỪNG phần đã được chứng minh giao hoán, kết
hợp, idempotent một cách ĐỘC LẬP, phần TỔNG HỢP của chúng — hợp nhất
từng trường riêng biệt — tự động thừa hưởng cả ba tính chất đó, không
cần chứng minh lại từ đầu cho toàn bộ `TrangThaiReaction`. Ba thứ tự
hợp nhất khác nhau Ở đầu bài chỉ là MỘT phép thử — nhưng phép thử đó
đứng vững chính vì nền tảng bên dưới đã được đo, không chỉ được tin.
::::

::::checkpoint{mastery=0.87}
::::
