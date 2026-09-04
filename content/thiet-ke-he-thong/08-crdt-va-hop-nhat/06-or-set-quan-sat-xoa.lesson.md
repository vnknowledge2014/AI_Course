---
id: thiet-ke-he-thong.crdt-va-hop-nhat.or-set-quan-sat-xoa
title: "OR-Set: mỗi lần thêm gắn một tag riêng, xoá chỉ đánh dấu tag đó"
summary: "interface ORSet { themVao: PhanTuOR[]; xoaDi: Set<string> } -- moi PhanTuOR co { giaTri, tag }, tag la CHUOI DUY NHAT (vi du taoTag('x',1) = 'x-1'), KHONG dung hash; xoaOR chi danh dau cac tag DANG HIEN DIEN vao xoaDi, khong xoa that khoi themVao (tombstone); hopNhat = gop themVao theo tag + hop xoaDi -- them 'an' roi xoa roi hop nhat voi ban sao chua biet xoa van hoi tu ve KHONG con 'an'; nhung THEM LAI bang TAG MOI (khac tag cu da bi xoa) thi hien dien tro lai binh thuong, day la chinh sach add-wins-qua-tag-moi duoc CHON tuong minh."
locale: vi
track: thiet-ke-he-thong
module: crdt-va-hop-nhat
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [sd.fp.or-set-quan-sat-xoa]
requires: [sd.fp.g-set-va-phep-hop]
concepts: [sd.fp.or-set-quan-sat-xoa]
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
`GSet` không xoá được — một khi thêm, Ở LẠI mãi mãi. Cách "sửa" đơn
giản nhất nghĩ tới: thêm một `Set` thứ hai để nhớ "đã xoá", rồi phần
tử hiện diện = có trong tập thêm VÀ không có trong tập xoá. Nhưng nếu
chỉ đánh dấu theo GIÁ TRỊ (ví dụ chuỗi `"an"`), một lần XOÁ "an" sẽ
chặn MỌI lần THÊM "an" sau đó mãi mãi — kể cả những lần thêm hoàn
toàn MỚI, xảy ra SAU khi xoá. Cần một cách phân biệt "lần thêm nào"
đang được nói tới.
::::

::::explain{#or-set-tag-rieng-cho-tung-lan-them}
`ORSet` gắn mỗi lần thêm một `tag` DUY NHẤT (`taoTag(replicaId,
soThuTu)` chỉ nối chuỗi, không cần hash) — `themVao` là danh sách các
`{ giaTri, tag }`, `xoaDi` là tập các tag đã bị xoá. Một phần tử HIỆN
DIỆN khi tag của nó có trong `themVao` VÀ KHÔNG có trong `xoaDi`.
`xoaOR` không xoá thật khỏi `themVao` — nó chỉ thêm các tag ĐANG hiện
diện của giá trị đó vào `xoaDi` (tombstone — bia mộ đánh dấu, không
xoá xác):

```typescript title=readonly
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
function giaTriHienDien(os: ORSet): string {
  const tapHop = new Set(phanTuHienDien(os).map((pt) => pt.giaTri));
  return Array.from(tapHop).sort().join(",");
}
function xoaOR(os: ORSet, giaTri: string): ORSet {
  const tagCanXoa = phanTuHienDien(os).filter((pt) => pt.giaTri === giaTri).map((pt) => pt.tag);
  return { themVao: os.themVao, xoaDi: new Set([...os.xoaDi, ...tagCanXoa]) };
}
function hopNhat(a: ORSet, b: ORSet): ORSet {
  const theoTag = new Map<string, PhanTuOR>();
  for (const pt of a.themVao) theoTag.set(pt.tag, pt);
  for (const pt of b.themVao) theoTag.set(pt.tag, pt);
  return { themVao: Array.from(theoTag.values()), xoaDi: new Set([...a.xoaDi, ...b.xoaDi]) };
}

let x = orSetRong();
x = themOR(x, "an", taoTag("x", 1));
let y = hopNhat(orSetRong(), x);
y = xoaOR(y, "an");
console.log("x hien dien:", giaTriHienDien(x) || "(rong)");
console.log("y hien dien SAU xoa:", giaTriHienDien(y) || "(rong)");
const merged = hopNhat(x, y);
console.log("hop nhat x (chua biet xoa) va y (da xoa):", giaTriHienDien(merged) || "(rong)");
```

```text title=readonly
x hien dien: an
y hien dien SAU xoa: (rong)
hop nhat x (chua biet xoa) va y (da xoa): (rong)
```

`x` thêm `"an"` với tag `"x-1"`. `y` đồng bộ với `x` (biết `"an"` Ở
tag `"x-1"`), rồi TỰ xoá `"an"` — thao tác này đánh dấu tag `"x-1"`
vào `y.xoaDi`, KHÔNG xoá phần tử khỏi `y.themVao`. Hợp nhất `x` (chưa
biết chuyện xoá) VỚI `y` (đã xoá): `themVao` gộp lại vẫn có
`{an, x-1}`, nhưng `xoaDi` gộp lại giờ có `"x-1"` — nên
`phanTuHienDien` lọc nó ra, kết quả hội tụ đúng về KHÔNG còn `"an"`,
dù `x` (một bên của phép hợp nhất) chưa từng tự xoá gì cả.
::::

::::example{#them-lai-bang-tag-moi-khong-bi-chan}
Chính sách được CHỌN Ở đây: MỖI lần thêm sinh một tag HOÀN TOÀN mới.
Vì vậy, một lần xoá (đánh dấu tag CŨ) không bao giờ chặn một lần thêm
MỚI SAU đó — chúng nói về hai tag khác nhau hoàn toàn:

```typescript title=readonly
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
function giaTriHienDien(os: ORSet): string {
  const tapHop = new Set(phanTuHienDien(os).map((pt) => pt.giaTri));
  return Array.from(tapHop).sort().join(",");
}
function xoaOR(os: ORSet, giaTri: string): ORSet {
  const tagCanXoa = phanTuHienDien(os).filter((pt) => pt.giaTri === giaTri).map((pt) => pt.tag);
  return { themVao: os.themVao, xoaDi: new Set([...os.xoaDi, ...tagCanXoa]) };
}
function hopNhat(a: ORSet, b: ORSet): ORSet {
  const theoTag = new Map<string, PhanTuOR>();
  for (const pt of a.themVao) theoTag.set(pt.tag, pt);
  for (const pt of b.themVao) theoTag.set(pt.tag, pt);
  return { themVao: Array.from(theoTag.values()), xoaDi: new Set([...a.xoaDi, ...b.xoaDi]) };
}

let a2 = orSetRong();
a2 = themOR(a2, "binh", taoTag("a", 1));
let b2 = hopNhat(orSetRong(), a2);
b2 = xoaOR(b2, "binh");
a2 = themOR(a2, "binh", taoTag("a", 2));
const hopNhatCuoi = hopNhat(a2, b2);
console.log("a2 (da them lai bang tag a-2, chua biet b2 xoa):", giaTriHienDien(a2) || "(rong)");
console.log("b2 (da xoa bang tag a-1):", giaTriHienDien(b2) || "(rong)");
console.log("hop nhat CUOI:", giaTriHienDien(hopNhatCuoi) || "(rong)");
console.log("hop nhat CUOI (thu tu nguoc):", giaTriHienDien(hopNhat(b2, a2)) || "(rong)");
```

```text title=readonly
a2 (da them lai bang tag a-2, chua biet b2 xoa): binh
b2 (da xoa bang tag a-1): (rong)
hop nhat CUOI: binh
hop nhat CUOI (thu tu nguoc): binh
```

`a2` thêm `"binh"` (tag `"a-1"`), đồng bộ sang `b2`, rồi `b2` xoá
`"binh"` (đánh dấu `"a-1"`). Trong lúc đó, `a2` — CHƯA nghe tin xoá —
tự thêm LẠI `"binh"` bằng một tag hoàn toàn MỚI: `"a-2"`. Hợp nhất
`a2` VÀ `b2`: `themVao` có CẢ `{binh, a-1}` lẫn `{binh, a-2}`,
`xoaDi` chỉ có `"a-1"`. Lọc theo `xoaDi`: tag `"a-1"` bị loại, tag
`"a-2"` VẪN hiện diện — `"binh"` xuất hiện trở lại, đúng như chính
sách đã chọn: xoá một tag CŨ không ảnh hưởng một lần thêm hoàn toàn
MỚI. Hợp nhất theo thứ tự ngược lại cho CÙNG kết quả.
::::

::::predict{#doan-them-lai-bang-tag-cu-da-bi-xoa commitOnce}
Nếu, thay vì tạo tag MỚI, một replica "thêm lại" bằng CÁCH DÙNG LẠI
đúng tag CŨ đã bị xoá — ví dụ: thêm `"chi"` bằng tag `"p-1"`, xoá
`"chi"` (đánh dấu `"p-1"`), rồi gọi `themOR(p, "chi", taoTag("p",
1))` một lần NỮA (`taoTag("p", 1)` luôn trả về ĐÚNG chuỗi `"p-1"`,
không đổi) — `giaTriHienDien(p)` sau đó là gì?

:::opt{correct}
Vẫn RỖNG — tag `"p-1"` đã nằm trong `xoaDi` từ trước, VÀ `xoaOR`
không bao giờ xoá tag khỏi `xoaDi`; thêm lại một `PhanTuOR` khác
mang ĐÚNG tag đó vào `themVao` không thay đổi việc tag `"p-1"` vẫn bị
lọc bởi `phanTuHienDien`
:::
:::opt
`"chi"` hiện diện trở lại — vì `themOR` vừa được gọi thêm một lần
NỮA, thêm lại nghĩa là phục hồi phần tử đó
::why
Nhầm "gọi `themOR`" với "phục hồi hiện diện" — nhưng `phanTuHienDien`
lọc theo TAG, không theo "đã gọi `themOR` gần đây hay chưa".

Chỗ lệch: `taoTag("p", 1)` LUÔN trả về đúng chuỗi `"p-1"` — dùng lại
đúng tham số cũ tạo ra đúng tag CŨ, không phải tag mới. Dòng
`phanTuHienDien` lọc `os.themVao.filter((pt) => !os.xoaDi.has(pt.tag))`
— với BẤT KỲ `PhanTuOR` nào mang tag `"p-1"`, kể cả một entry MỚI vừa
thêm, `os.xoaDi.has("p-1")` vẫn LÀ `true` (vì `xoaOR` chỉ THÊM vào
`xoaDi`, không bao giờ gỡ ra), nên entry đó vẫn bị lọc. Đây chính là
lý do bài học nhấn mạnh: mỗi lần thêm PHẢI dùng một tag MỚI — dùng
lại tag cũ không "phục hồi" được gì.
::
:::
::::

::::code{#viet_hop_nhat_orset}
Hoàn thiện `hopNhat` cho `ORSet` — gộp `themVao` của `a` VÀ `b` theo
`tag` (mỗi tag chỉ giữ MỘT bản ghi, dùng `Map<string, PhanTuOR>`), VÀ
hợp `xoaDi` của `a` VÀ `b` thành một `Set` MỚI.

```typescript title=starter
interface PhanTuOR { giaTri: string; tag: string; }
interface ORSet { themVao: PhanTuOR[]; xoaDi: Set<string>; }
function phanTuHienDien(os: ORSet): PhanTuOR[] {
  return os.themVao.filter((pt) => !os.xoaDi.has(pt.tag));
}
function giaTriHienDien(os: ORSet): string {
  const tapHop = new Set(phanTuHienDien(os).map((pt) => pt.giaTri));
  return Array.from(tapHop).sort().join(",");
}

function hopNhat(a: ORSet, b: ORSet): ORSet {
  ___
}

const m1: ORSet = { themVao: [{ giaTri: "one", tag: "m1-1" }], xoaDi: new Set() };
const m2: ORSet = { themVao: [{ giaTri: "two", tag: "m2-1" }], xoaDi: new Set(["m2-1"]) };
const ketQua = hopNhat(m1, m2);
console.log(giaTriHienDien(ketQua) || "(rong)");
```

```typescript title=solution
interface PhanTuOR { giaTri: string; tag: string; }
interface ORSet { themVao: PhanTuOR[]; xoaDi: Set<string>; }
function phanTuHienDien(os: ORSet): PhanTuOR[] {
  return os.themVao.filter((pt) => !os.xoaDi.has(pt.tag));
}
function giaTriHienDien(os: ORSet): string {
  const tapHop = new Set(phanTuHienDien(os).map((pt) => pt.giaTri));
  return Array.from(tapHop).sort().join(",");
}

function hopNhat(a: ORSet, b: ORSet): ORSet {
  const theoTag = new Map<string, PhanTuOR>();
  for (const pt of a.themVao) theoTag.set(pt.tag, pt);
  for (const pt of b.themVao) theoTag.set(pt.tag, pt);
  return { themVao: Array.from(theoTag.values()), xoaDi: new Set([...a.xoaDi, ...b.xoaDi]) };
}

const m1: ORSet = { themVao: [{ giaTri: "one", tag: "m1-1" }], xoaDi: new Set() };
const m2: ORSet = { themVao: [{ giaTri: "two", tag: "m2-1" }], xoaDi: new Set(["m2-1"]) };
const ketQua = hopNhat(m1, m2);
console.log(giaTriHienDien(ketQua) || "(rong)");
```

```typescript title=test
const tOs1: ORSet = { themVao: [{ giaTri: "one", tag: "m1-1" }], xoaDi: new Set() };
const tOs2: ORSet = { themVao: [{ giaTri: "two", tag: "m2-1" }], xoaDi: new Set(["m2-1"]) };
const tKetQua = hopNhat(tOs1, tOs2);
if (giaTriHienDien(tKetQua) !== "one") throw new Error("chi 'one' phai hien dien -- 'two' da bi xoa (tag m2-1 nam trong xoaDi)");

const tKetQuaNguoc = hopNhat(tOs2, tOs1);
if (giaTriHienDien(tKetQuaNguoc) !== giaTriHienDien(tKetQua)) throw new Error("hopNhat phai giao hoan -- ca hai thu tu phai ra CUNG ket qua hien dien");

if (tKetQua.themVao.length !== 2) throw new Error("themVao phai GIU CA HAI entry (kem tag m1-1 va m2-1), du 'two' bi loc khoi hien dien");
if (!tKetQua.xoaDi.has("m2-1")) throw new Error("xoaDi phai chua m2-1 sau khi hop nhat");

const truoc1 = JSON.stringify({ tv: tOs1.themVao, xd: Array.from(tOs1.xoaDi) });
const truoc2 = JSON.stringify({ tv: tOs2.themVao, xd: Array.from(tOs2.xoaDi) });
hopNhat(tOs1, tOs2);
if (JSON.stringify({ tv: tOs1.themVao, xd: Array.from(tOs1.xoaDi) }) !== truoc1) throw new Error("KHONG duoc mutate a truyen vao");
if (JSON.stringify({ tv: tOs2.themVao, xd: Array.from(tOs2.xoaDi) }) !== truoc2) throw new Error("KHONG duoc mutate b truyen vao");
```

:::hints
- kind: attention
  body: "Dung mot Map<string, PhanTuOR> de gop themVao theo tag: duyet a.themVao roi b.themVao, set(pt.tag, pt) cho tung phan tu (tag trung se tu dong ghi de bang gia tri giong het, khong sao). xoaDi la new Set([...a.xoaDi, ...b.xoaDi])."
- kind: strategy
  body: "const theoTag = new Map<string, PhanTuOR>(); for (const pt of a.themVao) theoTag.set(pt.tag, pt); for (const pt of b.themVao) theoTag.set(pt.tag, pt); return { themVao: Array.from(theoTag.values()), xoaDi: new Set([...a.xoaDi, ...b.xoaDi]) };"
- kind: one-line
  body: "const theoTag = new Map<string, PhanTuOR>(); for (const pt of a.themVao) theoTag.set(pt.tag, pt); for (const pt of b.themVao) theoTag.set(pt.tag, pt); return { themVao: Array.from(theoTag.values()), xoaDi: new Set([...a.xoaDi, ...b.xoaDi]) };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "one"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Thêm lẫn xoá, hội tụ đúng — nhờ mỗi lần thêm mang một tag riêng, VÀ
xoá chỉ đánh dấu chứ không xoá thật. Nhưng cả bốn CRDT vừa học đều
hợp nhất bằng cách GIỮ LẠI mọi phiên bản (max, union, tombstone).
Nếu chỉ cần MỘT giá trị duy nhất — tên hiển thị, trạng thái online —
và chấp nhận bản MỚI NHẤT thắng, có cách nào đơn giản hơn không?
::::

::::reflect{#nghi-lai}
`ORSet` giải quyết đúng vấn đề `GSet` để ngỏ, nhưng phải trả giá bằng
độ phức tạp: mỗi phần tử giờ mang thêm một `tag`, VÀ dữ liệu "đã xoá"
không bao giờ thật sự biến mất khỏi `themVao` — nó chỉ bị ẩn đi bằng
`xoaDi`. Đây là một đánh đổi CÓ CHỦ ĐÍCH, không phải sơ suất: để hợp
nhất đúng trong một hệ phân tán, đôi khi phải GIỮ nhiều thông tin
hơn một cấu trúc dữ liệu tương đương chạy trên một máy — chính vì
không có ai đứng ra làm trọng tài duy nhất quyết định thứ tự sự kiện.
::::

::::checkpoint{mastery=0.79}
::::
