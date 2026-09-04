---
id: thiet-ke-he-thong.crdt-va-hop-nhat.g-set-va-phep-hop
title: "G-Set: hợp nhất một tập hợp bằng phép hợp (union)"
summary: "interface GSet { phanTu: Set<string> } -- them(gs,x) tra ve GSet MOI voi x them vao; hopNhat(a,b) = union hai Set -- tu nhien giao hoan, ket hop, idempotent, KHONG can chung minh rieng nhu GCounter vi hop cua hai tap hop von da co san ba tinh chat do; nhung GSet chi THEM, khong co 'xoa' -- van de mo cho bai sau."
locale: vi
track: thiet-ke-he-thong
module: crdt-va-hop-nhat
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.fp.g-set-va-phep-hop]
requires: [sd.fp.pn-counter]
concepts: [sd.fp.g-set-va-phep-hop]
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
`GCounter` VÀ `PNCounter` hợp nhất SỐ. Nhưng nhiều dữ liệu phân tán
không phải số — danh sách người đã "thích" một bài viết, tập nhãn gắn
vào một item. Hợp nhất một TẬP HỢP khác hẳn hợp nhất một con số: cần
một phép toán vừa giao hoán, vừa kết hợp, vừa idempotent — VÀ hoá ra,
một phép toán như vậy đã tồn tại từ lâu, không cần phát minh gì mới.
::::

::::explain{#g-set-hop-nhat-bang-union}
`GSet` (grow-only set) giữ một `Set<string>` — `them` trả về một
`GSet` MỚI với phần tử vừa thêm. `hopNhat` chỉ đơn giản lấy PHÉP HỢP
(union) của hai `Set`:

```typescript title=readonly
interface GSet { phanTu: Set<string>; }
function gSetRong(): GSet { return { phanTu: new Set<string>() }; }
function them(gs: GSet, x: string): GSet {
  return { phanTu: new Set(gs.phanTu).add(x) };
}
function hopNhat(a: GSet, b: GSet): GSet {
  return { phanTu: new Set([...a.phanTu, ...b.phanTu]) };
}
function dsSapXep(gs: GSet): string {
  return Array.from(gs.phanTu).sort().join(",");
}

let x = gSetRong();
x = them(x, "an");
x = them(x, "binh");
let y = gSetRong();
y = them(y, "chi");
const merged = hopNhat(x, y);
console.log("x:", dsSapXep(x));
console.log("y:", dsSapXep(y));
console.log("hopNhat(x,y):", dsSapXep(merged));
```

```text title=readonly
x: an,binh
y: chi
hopNhat(x,y): an,binh,chi
```

`x` (từ replica biết "an" và "binh" đã tương tác) hợp nhất với `y`
(chỉ biết "chi") ra đúng CẢ ba tên. `them` không hề sửa `gs.phanTu`
truyền vào — nó tạo một `Set` MỚI (`new Set(gs.phanTu)`) rồi mới
`.add(x)` vào bản sao đó. `hopNhat` cũng vậy: `new Set([...a.phanTu,
...b.phanTu])` luôn tạo một `Set` HOÀN TOÀN mới, không đụng tới `a`
hay `b`.
::::

::::example{#union-tu-nhien-giao-hoan-ket-hop-idempotent}
Không cần viết một `ganNhauCauTruc` riêng để kiểm ba luật như đã làm
với `GCounter` — phép hợp của tập hợp vốn dĩ ĐÃ giao hoán, kết hợp,
VÀ idempotent, đúng theo định nghĩa toán học của chính phép hợp:

```typescript title=readonly
interface GSet { phanTu: Set<string>; }
function gSetRong(): GSet { return { phanTu: new Set<string>() }; }
function them(gs: GSet, x: string): GSet {
  return { phanTu: new Set(gs.phanTu).add(x) };
}
function hopNhat(a: GSet, b: GSet): GSet {
  return { phanTu: new Set([...a.phanTu, ...b.phanTu]) };
}
function dsSapXep(gs: GSet): string {
  return Array.from(gs.phanTu).sort().join(",");
}

let x = gSetRong();
x = them(x, "an");
x = them(x, "binh");
let y = gSetRong();
y = them(y, "chi");
const merged = hopNhat(x, y);

const mergedNguoc = hopNhat(y, x);
console.log("hopNhat(y,x):", dsSapXep(mergedNguoc));
console.log("giong hopNhat(x,y)?", dsSapXep(merged) === dsSapXep(mergedNguoc));
const tuHopNhat = hopNhat(merged, merged);
console.log("hopNhat(merged,merged):", dsSapXep(tuHopNhat));
console.log("idempotent?", dsSapXep(merged) === dsSapXep(tuHopNhat));
```

```text title=readonly
hopNhat(y,x): an,binh,chi
giong hopNhat(x,y)? true
hopNhat(merged,merged): an,binh,chi
idempotent? true
```

Cả `giao hoán` lẫn `idempotent` đều ra `true`, đúng như GCounter Ở
hai bài trước — nhưng lần này KHÔNG có ô nào để lấy `Math.max`, chỉ
có `Set` gộp lại. `GSet` giải quyết TỐT bài toán "thêm vào một tập
hợp phân tán". Nhưng nó chỉ có `them`, hoàn toàn KHÔNG có `xoa` — một
khi đã thêm, một phần tử Ở LẠI mãi mãi trong mọi bản sao.
::::

::::predict{#doan-goi-them-khong-gan-lai commitOnce}
Tiếp tục từ đoạn `explain`: `x` có `phanTu` chứa đúng `"an"` VÀ
`"binh"`. Gọi `them(x, "dung")` một lần NỮA (không gán kết quả trả về
vào biến nào) — sau lệnh gọi đó, `x` chứa những gì?

:::opt{correct}
Vẫn chỉ `"an"` VÀ `"binh"` — `them` trả về một `GSet` MỚI
(`new Set(gs.phanTu).add(x)` tạo bản sao TRƯỚC khi thêm), nó không hề
sửa `gs.phanTu` của tham số truyền vào; kết quả trả về bị bỏ qua nên
không ai nhìn thấy `"dung"` cả
:::
:::opt
`x` giờ có thêm `"dung"`, vì `.add()` LÀ một phương thức mutate trực
tiếp Ở trên `Set`, và `them` gọi đúng phương thức đó
::why
Đúng một nửa: `Set.prototype.add` THẬT SỰ mutate `Set` mà nó được gọi
trên đó — nhưng câu hỏi là `.add()` được gọi trên `Set` NÀO.

Chỗ lệch: dòng `return new Set(gs.phanTu).add(x);` trước tiên tạo
MỘT `Set` HOÀN TOÀN MỚI bằng `new Set(gs.phanTu)` (sao chép phần tử
từ `gs.phanTu`, không giữ tham chiếu tới nó), RỒI mới gọi `.add(x)`
Ở TRÊN bản sao mới đó — không phải Ở trên `gs.phanTu` gốc. `.add()`
có mutate thật, nhưng nó mutate cái `Set` MỚI vừa tạo, không phải
`Set` bên trong `x` truyền vào.
::
:::
::::

::::code{#viet_hop_nhat_gset}
Hoàn thiện `hopNhat` cho `GSet` — trả về một `GSet` MỚI chứa PHÉP HỢP
(union) của `a.phanTu` VÀ `b.phanTu`, không sửa `a` hay `b` truyền
vào.

```typescript title=starter
interface GSet { phanTu: Set<string>; }
function gSetRong(): GSet { return { phanTu: new Set<string>() }; }
function them(gs: GSet, x: string): GSet {
  return { phanTu: new Set(gs.phanTu).add(x) };
}

function hopNhat(a: GSet, b: GSet): GSet {
  ___
}

let gA = gSetRong();
gA = them(gA, "p");
let gB = gSetRong();
gB = them(gB, "q");
gB = them(gB, "r");
const ketQua = hopNhat(gA, gB);
console.log(Array.from(ketQua.phanTu).sort().join(","));
```

```typescript title=solution
interface GSet { phanTu: Set<string>; }
function gSetRong(): GSet { return { phanTu: new Set<string>() }; }
function them(gs: GSet, x: string): GSet {
  return { phanTu: new Set(gs.phanTu).add(x) };
}

function hopNhat(a: GSet, b: GSet): GSet {
  return { phanTu: new Set([...a.phanTu, ...b.phanTu]) };
}

let gA = gSetRong();
gA = them(gA, "p");
let gB = gSetRong();
gB = them(gB, "q");
gB = them(gB, "r");
const ketQua = hopNhat(gA, gB);
console.log(Array.from(ketQua.phanTu).sort().join(","));
```

```typescript title=test
const a1: GSet = { phanTu: new Set(["m", "n"]) };
const b1: GSet = { phanTu: new Set(["n", "o"]) };
const kq1 = hopNhat(a1, b1);
const dsKq1 = Array.from(kq1.phanTu).sort().join(",");
if (dsKq1 !== "m,n,o") throw new Error("hop nhat phai la UNION: m,n,o (khong lap n)");
if (kq1.phanTu.size !== 3) throw new Error("phan tu TRUNG (n) chi duoc tinh MOT lan trong union");

const truoc1 = Array.from(a1.phanTu).sort().join(",");
const truoc2 = Array.from(b1.phanTu).sort().join(",");
hopNhat(a1, b1);
if (Array.from(a1.phanTu).sort().join(",") !== truoc1) throw new Error("KHONG duoc mutate a truyen vao");
if (Array.from(b1.phanTu).sort().join(",") !== truoc2) throw new Error("KHONG duoc mutate b truyen vao");

const rong: GSet = { phanTu: new Set() };
const kqRong = hopNhat(rong, a1);
if (Array.from(kqRong.phanTu).sort().join(",") !== "m,n") throw new Error("hop nhat voi GSet rong phai giu nguyen gia tri kia");

const kqTuHopNhat = hopNhat(kq1, kq1);
if (Array.from(kqTuHopNhat.phanTu).sort().join(",") !== dsKq1) throw new Error("hop nhat mot GSet voi CHINH NO phai giu nguyen (idempotent)");
```

:::hints
- kind: attention
  body: "Dung spread de gop hai Set lai roi bo vao mot Set moi: new Set([...a.phanTu, ...b.phanTu]). KHONG duoc goi .add() truc tiep tren a.phanTu hay b.phanTu."
- kind: strategy
  body: "return { phanTu: new Set([...a.phanTu, ...b.phanTu]) };"
- kind: one-line
  body: "return { phanTu: new Set([...a.phanTu, ...b.phanTu]) };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "p,q,r"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Thêm vào một tập hợp phân tán — xong, không cần chứng minh gì thêm,
phép hợp tự lo hết. Nhưng "không xoá được" là một giới hạn thật: nếu
ai đó huỷ lượt thích, `GSet` không có cách nào phản ánh điều đó. Cần
gì để một tập hợp phân tán hỗ trợ CẢ xoá?
::::

::::reflect{#nghi-lai}
`GSet` không cần một `ganNhauCauTruc` riêng, không cần chứng minh ba
luật bằng tay như `GCounter` — phép hợp của tập hợp đã MANG SẴN cả
ba tính chất đó, đúng từ định nghĩa toán học của nó. Nhưng "dễ hợp
nhất" VÀ "đủ biểu diễn" là hai chuyện khác nhau: `GSet` hợp nhất cực
kỳ đơn giản, ĐỔI LẠI nó chỉ biểu diễn được đúng một loại thay đổi —
thêm vào, không bao giờ bớt ra.
::::

::::checkpoint{mastery=0.78}
::::
