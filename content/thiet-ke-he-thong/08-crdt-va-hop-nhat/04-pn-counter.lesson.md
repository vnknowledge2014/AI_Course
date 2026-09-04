---
id: thiet-ke-he-thong.crdt-va-hop-nhat.pn-counter
title: "PN-Counter: tách tăng và giảm thành hai G-Counter riêng"
summary: "interface PNCounter { tang: GCounter; giam: GCounter } -- giaTri(pn) = giaTriGCounter(pn.tang) - giaTriGCounter(pn.giam); hopNhat(a,b) hop nhat RIENG tang voi tang, giam voi giam -- goi tangGCounter voi buoc AM (-2) truc tiep tren MOT GCounter pha vo hopNhat bang MAX: gcA giam con 3 nhung hop nhat voi gcB (dong bo LUC con 5) lai ra 5, mat sach phan giam; tach thanh PNCounter rieng tang/giam giu dung gia tri 3 sau hop nhat."
locale: vi
track: thiet-ke-he-thong
module: crdt-va-hop-nhat
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.fp.pn-counter]
requires: [sd.fp.idempotent-khi-hop-nhat-voi-chinh-no]
concepts: [sd.fp.pn-counter]
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
`GCounter` chỉ tăng — grow-only, ngay trong tên gọi. Nhưng nhiều thứ
cần đếm CẢ hai chiều: huỷ một lượt thích, trả lại một món hàng vào
kho. Cách nhanh nhất nghĩ tới: gọi `tang` với một bước ÂM. Trước khi
làm vậy, hãy xem điều gì xảy ra khi một bản sao ĐÃ giảm rồi mới hợp
nhất với một bản sao KHÁC chưa hề biết chuyện đó.
::::

::::explain{#giam-truc-tiep-pha-vo-hop-nhat-bang-max}
`tangGCounter(gc, replicaId, buoc)` cộng `buoc` vào ô hiện tại — không
gì ngăn `buoc` là số ÂM. Nhưng `hopNhatGCounter` lấy MAX từng ô, dựa
trên giả định NGẦM rằng giá trị mỗi ô chỉ TĂNG theo thời gian
(monotonic). Gọi `tangGCounter(gc, "a", -2)` phá vỡ giả định đó — và
khi hợp nhất với một bản sao đã đồng bộ TRƯỚC lúc giảm, phần giảm
biến mất:

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

let gcA = counterRong();
gcA = tangGCounter(gcA, "a", 5);
let gcB = hopNhatGCounter(counterRong(), gcA);
gcA = tangGCounter(gcA, "a", -2);
console.log("gcA sau khi goi tang(-2) de 'giam':", JSON.stringify(gcA.theoReplica));
console.log("gcB (da dong bo TRUOC khi gcA giam):", JSON.stringify(gcB.theoReplica));
const hopNhatSai = hopNhatGCounter(gcA, gcB);
console.log("hop nhat gcA (da giam) voi gcB (chua biet giam):", JSON.stringify(hopNhatSai.theoReplica));
console.log("gia tri SAU hop nhat (MAX thang, phan giam bien mat):", giaTriGCounter(hopNhatSai));

interface PNCounter { tang: GCounter; giam: GCounter; }
function pnCounterRong(): PNCounter { return { tang: counterRong(), giam: counterRong() }; }
function pnTang(pn: PNCounter, replicaId: string, buoc: number): PNCounter {
  return { tang: tangGCounter(pn.tang, replicaId, buoc), giam: pn.giam };
}
function pnGiam(pn: PNCounter, replicaId: string, buoc: number): PNCounter {
  return { tang: pn.tang, giam: tangGCounter(pn.giam, replicaId, buoc) };
}
function giaTri(pn: PNCounter): number {
  return giaTriGCounter(pn.tang) - giaTriGCounter(pn.giam);
}
function hopNhat(a: PNCounter, b: PNCounter): PNCounter {
  return { tang: hopNhatGCounter(a.tang, b.tang), giam: hopNhatGCounter(a.giam, b.giam) };
}

let pnA = pnCounterRong();
pnA = pnTang(pnA, "a", 5);
let pnB = hopNhat(pnCounterRong(), pnA);
pnA = pnGiam(pnA, "a", 2);
console.log("pnA sau khi giam 2 DUNG PNCounter:", JSON.stringify(pnA));
console.log("gia tri pnA:", giaTri(pnA));
const hopNhatDung = hopNhat(pnA, pnB);
console.log("hop nhat pnA (da giam) voi pnB (chua biet giam):", JSON.stringify(hopNhatDung));
console.log("gia tri SAU hop nhat DUNG:", giaTri(hopNhatDung));
```

```text title=readonly
gcA sau khi goi tang(-2) de 'giam': {"a":3}
gcB (da dong bo TRUOC khi gcA giam): {"a":5}
hop nhat gcA (da giam) voi gcB (chua biet giam): {"a":5}
gia tri SAU hop nhat (MAX thang, phan giam bien mat): 5
pnA sau khi giam 2 DUNG PNCounter: {"tang":{"theoReplica":{"a":5}},"giam":{"theoReplica":{"a":2}}}
gia tri pnA: 3
hop nhat pnA (da giam) voi pnB (chua biet giam): {"tang":{"theoReplica":{"a":5}},"giam":{"theoReplica":{"a":2}}}
gia tri SAU hop nhat DUNG: 3
```

`gcA` tăng `5` rồi "giảm" bằng `tang(-2)`, còn lại `3` Ở ô `"a"`.
Nhưng `gcB` — đồng bộ TỪ TRƯỚC lúc `gcA` giảm — vẫn nhớ `"a"` LÀ `5`.
Hợp nhất `gcA` với `gcB`: `Math.max(3, 5) = 5` — phần giảm bị XOÁ
sạch, không còn dấu vết. `PNCounter` tách riêng `tang` và `giam`
thành HAI `GCounter` — mỗi cái RIÊNG vẫn chỉ tăng, không bao giờ vi
phạm giả định monotonic. `pnGiam` ghi vào `pn.giam`, KHÔNG đụng tới
`pn.tang`. Hợp nhất `pnA` (đã giảm) với `pnB` (chưa biết): `tang`
lấy `Math.max(5, 5) = 5`, `giam` lấy `Math.max(2, 0) = 2`, giá trị
cuối đúng bằng `5 - 2 = 3` — không mất gì.
::::

::::example{#pn-counter-hai-replica-doc-lap}
Hai replica hoạt động độc lập — một bên vừa tăng vừa giảm, một bên
chỉ tăng — hợp nhất vẫn ra đúng kết quả, vì `tang` và `giam` mỗi bên
luôn hợp nhất ĐÚNG với phần tương ứng của bên kia:

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
function giaTri(pn: PNCounter): number {
  return giaTriGCounter(pn.tang) - giaTriGCounter(pn.giam);
}
function hopNhat(a: PNCounter, b: PNCounter): PNCounter {
  return { tang: hopNhatGCounter(a.tang, b.tang), giam: hopNhatGCounter(a.giam, b.giam) };
}

let pnX = pnCounterRong();
pnX = pnTang(pnX, "x", 10);
pnX = pnGiam(pnX, "x", 3);
let pnY = pnCounterRong();
pnY = pnTang(pnY, "y", 4);
const pnMerged = hopNhat(pnX, pnY);
console.log("pnMerged:", JSON.stringify(pnMerged));
console.log("gia tri:", giaTri(pnMerged));
```

```text title=readonly
pnMerged: {"tang":{"theoReplica":{"x":10,"y":4}},"giam":{"theoReplica":{"x":3}}}
gia tri: 11
```

`pnX` tự tăng `10` rồi giảm `3` — cả hai đều ghi vào ô `"x"` của
đúng phần `tang` hoặc `giam` tương ứng. `pnY` chỉ tăng `4` Ở ô `"y"`.
Hợp nhất: `tang` gộp cả hai ô (`x: 10, y: 4`), `giam` chỉ có ô `"x":
3` (vì `pnY` chưa từng giảm). Giá trị cuối `11` đúng bằng
`(10 + 4) - 3`.
::::

::::predict{#doan-pn-counter-tang-them-sau-khi-giam commitOnce}
`pnP1` được tạo bằng cách: tăng `8` rồi giảm `3`, cùng `replicaId`
`"p"` (`pnGiam(pnTang(pnCounterRong(), "p", 8), "p", 3)`), nên
`giaTri(pnP1) = 5`. Gọi thêm `pnP2 = pnTang(pnP1, "p", 1)` (tăng
thêm `1`, vẫn `replicaId` `"p"`) — `giaTri(pnP2)` là bao nhiêu?

:::opt{correct}
`6` — `pnTang` chỉ cộng thêm vào `pn.tang` (Ở ô `"p"`, từ `8` lên
`9`), KHÔNG đụng tới `pn.giam` (vẫn giữ `3`); giá trị mới là
`9 - 3 = 6`
:::
:::opt
`9` — vì lệnh `pnTang` MỚI NHẤT ghi đè lên toàn bộ lịch sử giảm
trước đó, tính lại từ `8 + 1 = 9` như thể phần giảm chưa từng xảy ra
::why
Nhầm `pnTang` với một phép GHI ĐÈ toàn bộ — nhưng thân hàm
`pnTang` chỉ trả về `{ tang: tangGCounter(pn.tang, replicaId,
buoc), giam: pn.giam }`, giữ NGUYÊN `pn.giam` Ở object mới.

Chỗ lệch: `pnTang` VÀ `pnGiam` là hai hàm HOÀN TOÀN tách biệt, mỗi
hàm chỉ chạm vào ĐÚNG một phần của `PNCounter` (`tang` hoặc `giam`),
không phần nào ảnh hưởng phần kia. `pnP1.giam` Ở ô `"p"` đã LÀ `3`
từ trước — gọi `pnTang` sau đó chỉ cộng thêm vào `pn.tang` (`8` lên
`9`), `pn.giam` không hề bị đọc hay ghi lại. `giaTri(pnP2) = 9 - 3 =
6`, không phải `9`.
::
:::
::::

::::code{#viet_hop_nhat_pn}
Hoàn thiện `hopNhat` cho `PNCounter` — hợp nhất RIÊNG phần `tang` của
`a` với phần `tang` của `b` (dùng `hopNhatGCounter` đã có), VÀ hợp
nhất RIÊNG phần `giam` của `a` với phần `giam` của `b`, rồi gộp cả
hai vào một `PNCounter` mới.

```typescript title=starter
interface GCounter { theoReplica: Record<string, number>; }
function hopNhatGCounter(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
function giaTriGCounter(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
interface PNCounter { tang: GCounter; giam: GCounter; }
function giaTri(pn: PNCounter): number {
  return giaTriGCounter(pn.tang) - giaTriGCounter(pn.giam);
}

function hopNhat(a: PNCounter, b: PNCounter): PNCounter {
  ___
}

const pnT1: PNCounter = { tang: { theoReplica: { t1: 6 } }, giam: { theoReplica: {} } };
const pnT2: PNCounter = { tang: { theoReplica: {} }, giam: { theoReplica: { t2: 2 } } };
const ketQuaT = hopNhat(pnT1, pnT2);
console.log(giaTri(ketQuaT), giaTriGCounter(ketQuaT.tang), giaTriGCounter(ketQuaT.giam));
```

```typescript title=solution
interface GCounter { theoReplica: Record<string, number>; }
function hopNhatGCounter(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
function giaTriGCounter(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
interface PNCounter { tang: GCounter; giam: GCounter; }
function giaTri(pn: PNCounter): number {
  return giaTriGCounter(pn.tang) - giaTriGCounter(pn.giam);
}

function hopNhat(a: PNCounter, b: PNCounter): PNCounter {
  return { tang: hopNhatGCounter(a.tang, b.tang), giam: hopNhatGCounter(a.giam, b.giam) };
}

const pnT1: PNCounter = { tang: { theoReplica: { t1: 6 } }, giam: { theoReplica: {} } };
const pnT2: PNCounter = { tang: { theoReplica: {} }, giam: { theoReplica: { t2: 2 } } };
const ketQuaT = hopNhat(pnT1, pnT2);
console.log(giaTri(ketQuaT), giaTriGCounter(ketQuaT.tang), giaTriGCounter(ketQuaT.giam));
```

```typescript title=test
const a1: PNCounter = { tang: { theoReplica: { r1: 10 } }, giam: { theoReplica: { r1: 4 } } };
const b1: PNCounter = { tang: { theoReplica: { r2: 3 } }, giam: { theoReplica: {} } };
const kq1 = hopNhat(a1, b1);
if (giaTri(kq1) !== 9) throw new Error("gia tri phai la (10 + 3) - 4 = 9");
if (giaTriGCounter(kq1.tang) !== 13) throw new Error("phan tang phai hop nhat DUNG: 10 + 3 = 13");
if (giaTriGCounter(kq1.giam) !== 4) throw new Error("phan giam phai giu nguyen cua a (b khong co giam): 4");

const truoc1 = JSON.stringify(a1);
const truoc2 = JSON.stringify(b1);
hopNhat(a1, b1);
if (JSON.stringify(a1) !== truoc1) throw new Error("KHONG duoc mutate a truyen vao");
if (JSON.stringify(b1) !== truoc2) throw new Error("KHONG duoc mutate b truyen vao");

const rong: PNCounter = { tang: { theoReplica: {} }, giam: { theoReplica: {} } };
const kqRong = hopNhat(rong, a1);
if (giaTri(kqRong) !== giaTri(a1)) throw new Error("hop nhat voi PNCounter rong phai giu nguyen gia tri kia");
```

:::hints
- kind: attention
  body: "Goi hopNhatGCounter HAI lan rieng biet: mot lan cho a.tang voi b.tang, mot lan cho a.giam voi b.giam. Tra ve mot PNCounter moi voi hai ket qua do -- KHONG duoc tron lan tang voi giam."
- kind: strategy
  body: "return { tang: hopNhatGCounter(a.tang, b.tang), giam: hopNhatGCounter(a.giam, b.giam) };"
- kind: one-line
  body: "return { tang: hopNhatGCounter(a.tang, b.tang), giam: hopNhatGCounter(a.giam, b.giam) };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "4 6 2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tách tăng và giảm thành hai G-Counter riêng — giờ đếm được cả hai
chiều, vẫn hội tụ đúng. Nhưng cả G-Counter lẫn PN-Counter chỉ xử lý
SỐ. Nếu cần hợp nhất một TẬP HỢP — danh sách người dùng, danh sách
thẻ gắn — thì cần một cấu trúc khác hẳn.
::::

::::reflect{#nghi-lai}
`PNCounter` không hề "sửa" `GCounter` để nó chấp nhận số âm — nó
dùng LẠI nguyên vẹn `GCounter` (không đổi một dòng nào bên trong),
chỉ thêm một tầng cấu trúc bên ngoài: hai bản `GCounter` riêng, mỗi
bản vẫn tuân đúng luật grow-only của chính nó. Đây là một khuôn sẽ
lặp lại xuyên suốt quest này: khi một cấu trúc dữ liệu không hợp nhất
đúng cho một loại thao tác nào đó, câu trả lời thường không phải "sửa
phép hợp nhất cho phức tạp hơn", mà là "tách thao tác đó ra một cấu
trúc con riêng, vẫn giữ phép hợp nhất đơn giản".
::::

::::checkpoint{mastery=0.76}
::::
