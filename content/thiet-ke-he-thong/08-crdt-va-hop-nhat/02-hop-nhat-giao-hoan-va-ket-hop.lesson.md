---
id: thiet-ke-he-thong.crdt-va-hop-nhat.hop-nhat-giao-hoan-va-ket-hop
title: "Hợp nhất giao hoán và kết hợp: kiểm bằng test, không chỉ tin"
summary: "ganNhauCauTruc(a,b) sap xep key roi so JSON.stringify -- kiemTraGiaoHoanVaKetHop(a,b,c) tra ve { giaoHoan, ketHop } bang cach so hopNhat(a,b) voi hopNhat(b,a), va hopNhat(hopNhat(a,b),c) voi hopNhat(a,hopNhat(b,c)) -- ca hai deu true tren so lieu that, KHONG chi tin loi noi; day chinh la luat KET HOP cua Monoid<T> hoc o R4 T4.5, cong them mot luat THU BA ma monoid thuong khong doi hoi: GIAO HOAN, can rieng cho CRDT vi mang khong dam bao thu tu giao."
locale: vi
track: thiet-ke-he-thong
module: crdt-va-hop-nhat
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.fp.hop-nhat-giao-hoan-va-ket-hop]
requires: [sd.fp.g-counter]
concepts: [sd.fp.hop-nhat-giao-hoan-va-ket-hop]
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
Bài trước dùng MAX thay vì cộng, và số liệu ra đúng. Nhưng "đúng cho
MỘT ví dụ" khác hẳn "đúng cho MỌI thứ tự nhận". Trên mạng thật, hai
bản sao có thể nhận đúng CẶP cập nhật đó theo HAI thứ tự khác nhau —
nếu `hopNhat` không giao hoán (thứ tự merge quan trọng), hai bản sao
sẽ hội tụ về HAI trạng thái khác nhau. Đừng tin lời hứa đó — viết test
kiểm nó bằng số thật.
::::

::::explain{#giao-hoan-hopnhat-ab-bang-hopnhat-ba}
Ở R4 T4.5, `Monoid<T>` chỉ cần đúng HAI luật: `ketHop` phải KẾT HỢP
(nhóm thế nào cũng ra cùng kết quả) và có `rong` là phần tử TRUNG
TÍNH. Một monoid bất kỳ — ví dụ phép trừ — KHÔNG cần giao hoán:
`ketHop(a, b)` có thể khác `ketHop(b, a)`, chỉ có cách NHÓM là không
quan trọng. Nhưng `hopNhat` của một CRDT cần THÊM một luật thứ ba mà
monoid tổng quát không đòi: GIAO HOÁN (commutative) — vì hai bản sao
trên mạng có thể nhận CÙNG một cặp cập nhật theo HAI thứ tự khác
nhau, và cả hai thứ tự đó PHẢI hội tụ về đúng MỘT trạng thái.
`ganNhauCauTruc` so sánh hai `GCounter` bằng cách sắp xếp key trước
khi so — kiểm CẤU TRÚC, không chỉ tổng số:

```typescript title=readonly
interface GCounter { theoReplica: Record<string, number>; }
function counterRong(): GCounter { return { theoReplica: {} }; }
function tang(gc: GCounter, replicaId: string, buoc: number): GCounter {
  const hienTai = gc.theoReplica[replicaId] ?? 0;
  return { theoReplica: { ...gc.theoReplica, [replicaId]: hienTai + buoc } };
}
function giaTri(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
function hopNhat(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
function ganNhauCauTruc(a: GCounter, b: GCounter): boolean {
  const chuanHoa = (gc: GCounter): string =>
    JSON.stringify(Object.keys(gc.theoReplica).sort().map((k) => [k, gc.theoReplica[k] ?? 0]));
  return chuanHoa(a) === chuanHoa(b);
}

const gcX = tang(counterRong(), "s1", 5);
const gcY = tang(counterRong(), "s2", 3);
console.log("hopNhat(gcX,gcY):", JSON.stringify(hopNhat(gcX, gcY).theoReplica));
console.log("hopNhat(gcY,gcX):", JSON.stringify(hopNhat(gcY, gcX).theoReplica));
console.log("cau truc GIONG HET nhau (giao hoan)?", ganNhauCauTruc(hopNhat(gcX, gcY), hopNhat(gcY, gcX)));
console.log("giaTri hai chieu:", giaTri(hopNhat(gcX, gcY)), giaTri(hopNhat(gcY, gcX)));
```

```text title=readonly
hopNhat(gcX,gcY): {"s1":5,"s2":3}
hopNhat(gcY,gcX): {"s2":3,"s1":5}
cau truc GIONG HET nhau (giao hoan)? true
giaTri hai chieu: 8 8
```

`hopNhat(gcX,gcY)` VÀ `hopNhat(gcY,gcX)` in ra HAI chuỗi JSON khác
nhau — `{"s1":5,"s2":3}` so với `{"s2":3,"s1":5}` — vì thứ tự CHÈN
key vào object khác nhau tuỳ bên nào được spread trước. Nhưng
`ganNhauCauTruc` vẫn trả về `true`: nó SẮP XẾP key trước khi so, nên
thứ tự chèn không ảnh hưởng tới kết quả so sánh. Giao hoán không phải
chuyện "hai object trông giống hệt nhau trong bộ nhớ" — nó là chuyện
"hai object đại diện cho CÙNG một tập giá trị".
::::

::::example{#ket-hop-nhom-the-nao-cung-ra-cung-ket-qua}
`ketHop` (bài toán R4) cần cách NHÓM ba giá trị không ảnh hưởng kết
quả. Với `hopNhat`, thêm một `GCounter` thứ ba (`gcZ`) rồi so sánh hai
cách nhóm — nhóm từ trái trước, hay nhóm từ phải trước:

```typescript title=readonly
interface GCounter { theoReplica: Record<string, number>; }
function counterRong(): GCounter { return { theoReplica: {} }; }
function tang(gc: GCounter, replicaId: string, buoc: number): GCounter {
  const hienTai = gc.theoReplica[replicaId] ?? 0;
  return { theoReplica: { ...gc.theoReplica, [replicaId]: hienTai + buoc } };
}
function giaTri(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
function hopNhat(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
function ganNhauCauTruc(a: GCounter, b: GCounter): boolean {
  const chuanHoa = (gc: GCounter): string =>
    JSON.stringify(Object.keys(gc.theoReplica).sort().map((k) => [k, gc.theoReplica[k] ?? 0]));
  return chuanHoa(a) === chuanHoa(b);
}

const gcX = tang(counterRong(), "s1", 5);
const gcY = tang(counterRong(), "s2", 3);
const gcZ = tang(tang(counterRong(), "s1", 1), "s3", 2);

const veTrai = hopNhat(hopNhat(gcX, gcY), gcZ);
const vePhai = hopNhat(gcX, hopNhat(gcY, gcZ));
console.log("hopNhat(hopNhat(gcX,gcY),gcZ):", JSON.stringify(veTrai.theoReplica));
console.log("hopNhat(gcX,hopNhat(gcY,gcZ)):", JSON.stringify(vePhai.theoReplica));
console.log("cau truc GIONG HET nhau (ket hop)?", ganNhauCauTruc(veTrai, vePhai));
console.log("giaTri hai cach nhom:", giaTri(veTrai), giaTri(vePhai));
```

```text title=readonly
hopNhat(hopNhat(gcX,gcY),gcZ): {"s1":5,"s2":3,"s3":2}
hopNhat(gcX,hopNhat(gcY,gcZ)): {"s1":5,"s2":3,"s3":2}
cau truc GIONG HET nhau (ket hop)? true
giaTri hai cach nhom: 10 10
```

`gcZ` có ô `"s1": 1` — TRÙNG với `gcX` (ô `"s1": 5`). Dù nhóm
`(gcX hợp gcY) hợp gcZ` hay `gcX hợp (gcY hợp gcZ)`, ô `"s1"` cuối
cùng đều dừng Ở `Math.max(5, 1) = 5` (không phải `5 + 1 = 6`), và
tổng cuối đều là `10`. `Monoid` (R4) đã hứa "nhóm thế nào cũng ra
cùng kết quả" — Ở đây lời hứa đó được đo bằng số thật, không phải chỉ
được viết trong prose.
::::

::::predict{#doan-json-stringify-khong-tu-sap-xep commitOnce}
Ở đoạn `explain`, `hopNhat(gcX,gcY).theoReplica` in ra
`{"s1":5,"s2":3}` còn `hopNhat(gcY,gcX).theoReplica` in ra
`{"s2":3,"s1":5}` — hai chuỗi JSON KHÁC NHAU (khác thứ tự key), nhưng
`ganNhauCauTruc` vẫn trả về `true`. Vì sao hai chuỗi lại KHÁC nhau
như vậy?

:::opt{correct}
Vì `JSON.stringify` giữ đúng thứ tự CHÈN key vào object (`hopNhat`
sao chép `a.theoReplica` trước rồi mới thêm key mới của `b`) — `gcX`
được sao chép trước Ở `hopNhat(gcX,gcY)` nên `"s1"` đứng đầu, còn
`gcY` được sao chép trước Ở `hopNhat(gcY,gcX)` nên `"s2"` đứng đầu;
`ganNhauCauTruc` phải tự SẮP XẾP key bằng `.sort()` để bỏ qua khác
biệt này
:::
:::opt
Vì `JSON.stringify` tự động sắp xếp key theo bảng chữ cái trước khi
in ra chuỗi, và `ganNhauCauTruc` chỉ đang kiểm tra lại điều đó thêm
một lần nữa cho chắc
::why
Kết luận sai về HÀNH VI của `JSON.stringify` — quan sát Ở trên chính
là bằng chứng ngược lại: nếu `JSON.stringify` tự sắp xếp theo bảng
chữ cái, cả hai chuỗi Ở trên đều phải Ở dạng `{"s1":...,"s2":...}`
giống hệt nhau, không thể có chuỗi nào bắt đầu bằng `"s2"` trước
`"s1"`.

Chỗ lệch: `JSON.stringify` trên một object thường giữ NGUYÊN thứ tự
các thuộc tính được thêm vào (insertion order), không sắp xếp gì cả.
Dòng `const ketQua: Record<string, number> = { ...a.theoReplica };`
trong `hopNhat` sao chép key của tham số ĐẦU tiên trước — đổi vị trí
`a` và `b` thì đổi luôn key nào đứng trước trong object kết quả.
Chính vì `JSON.stringify` KHÔNG tự sắp xếp, `ganNhauCauTruc` mới cần
gọi `.sort()` một cách tường minh trước khi so sánh.
::
:::
::::

::::code{#viet_kiem_tra_giao_hoan_va_ket_hop}
Hoàn thiện `kiemTraGiaoHoanVaKetHop` — trả về một object với `giaoHoan`
LÀ kết quả so sánh CẤU TRÚC giữa `hopNhat(a, b)` và `hopNhat(b, a)`
(dùng `ganNhauCauTruc`), VÀ `ketHop` LÀ kết quả so sánh CẤU TRÚC giữa
`hopNhat(hopNhat(a, b), c)` và `hopNhat(a, hopNhat(b, c))`.

```typescript title=starter
interface GCounter { theoReplica: Record<string, number>; }
function hopNhat(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
function ganNhauCauTruc(a: GCounter, b: GCounter): boolean {
  const chuanHoa = (gc: GCounter): string =>
    JSON.stringify(Object.keys(gc.theoReplica).sort().map((k) => [k, gc.theoReplica[k] ?? 0]));
  return chuanHoa(a) === chuanHoa(b);
}

function kiemTraGiaoHoanVaKetHop(a: GCounter, b: GCounter, c: GCounter): { giaoHoan: boolean; ketHop: boolean } {
  ___
}

const t1: GCounter = { theoReplica: { p: 4 } };
const t2: GCounter = { theoReplica: { q: 9 } };
const t3: GCounter = { theoReplica: { p: 1, r: 6 } };
const kq = kiemTraGiaoHoanVaKetHop(t1, t2, t3);
console.log(kq.giaoHoan, kq.ketHop);
```

```typescript title=solution
interface GCounter { theoReplica: Record<string, number>; }
function hopNhat(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
function ganNhauCauTruc(a: GCounter, b: GCounter): boolean {
  const chuanHoa = (gc: GCounter): string =>
    JSON.stringify(Object.keys(gc.theoReplica).sort().map((k) => [k, gc.theoReplica[k] ?? 0]));
  return chuanHoa(a) === chuanHoa(b);
}

function kiemTraGiaoHoanVaKetHop(a: GCounter, b: GCounter, c: GCounter): { giaoHoan: boolean; ketHop: boolean } {
  const giaoHoan = ganNhauCauTruc(hopNhat(a, b), hopNhat(b, a));
  const ketHop = ganNhauCauTruc(hopNhat(hopNhat(a, b), c), hopNhat(a, hopNhat(b, c)));
  return { giaoHoan, ketHop };
}

const t1: GCounter = { theoReplica: { p: 4 } };
const t2: GCounter = { theoReplica: { q: 9 } };
const t3: GCounter = { theoReplica: { p: 1, r: 6 } };
const kq = kiemTraGiaoHoanVaKetHop(t1, t2, t3);
console.log(kq.giaoHoan, kq.ketHop);
```

```typescript title=test
const a1: GCounter = { theoReplica: { x: 2, y: 5 } };
const b1: GCounter = { theoReplica: { y: 8, z: 1 } };
const c1: GCounter = { theoReplica: { x: 9 } };
const ketQua = kiemTraGiaoHoanVaKetHop(a1, b1, c1);
if (ketQua.giaoHoan !== true) throw new Error("hopNhat cua GCounter PHAI giao hoan -- giaoHoan phai la true");
if (ketQua.ketHop !== true) throw new Error("hopNhat cua GCounter PHAI ket hop -- ketHop phai la true");

const truoc1 = JSON.stringify(a1.theoReplica);
const truoc2 = JSON.stringify(b1.theoReplica);
const truoc3 = JSON.stringify(c1.theoReplica);
kiemTraGiaoHoanVaKetHop(a1, b1, c1);
if (JSON.stringify(a1.theoReplica) !== truoc1) throw new Error("KHONG duoc mutate a truyen vao");
if (JSON.stringify(b1.theoReplica) !== truoc2) throw new Error("KHONG duoc mutate b truyen vao");
if (JSON.stringify(c1.theoReplica) !== truoc3) throw new Error("KHONG duoc mutate c truyen vao");

const rong: GCounter = { theoReplica: {} };
const ketQuaRong = kiemTraGiaoHoanVaKetHop(rong, rong, rong);
if (ketQuaRong.giaoHoan !== true || ketQuaRong.ketHop !== true) throw new Error("ba GCounter rong cung phai giao hoan va ket hop");
```

:::hints
- kind: attention
  body: "giaoHoan la ganNhauCauTruc(hopNhat(a,b), hopNhat(b,a)) -- doi CHIEU tham so cua hopNhat. ketHop la ganNhauCauTruc(hopNhat(hopNhat(a,b),c), hopNhat(a,hopNhat(b,c))) -- doi CACH NHOM ba gia tri, khong doi thu tu."
- kind: strategy
  body: "const giaoHoan = ganNhauCauTruc(hopNhat(a, b), hopNhat(b, a)); const ketHop = ganNhauCauTruc(hopNhat(hopNhat(a, b), c), hopNhat(a, hopNhat(b, c))); return { giaoHoan, ketHop };"
- kind: one-line
  body: "const giaoHoan = ganNhauCauTruc(hopNhat(a, b), hopNhat(b, a)); const ketHop = ganNhauCauTruc(hopNhat(hopNhat(a, b), c), hopNhat(a, hopNhat(b, c))); return { giaoHoan, ketHop };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "true true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Giao hoán VÀ kết hợp — không còn là lời hứa, mà là hai con số `true`
đo được. Nhưng cả hai luật đó mới nói "kết quả đúng bất kể THỨ TỰ".
Còn một tình huống khác: điều gì xảy ra nếu CÙNG một cập nhật tới
HAI LẦN — không phải hai cập nhật khác nhau theo thứ tự khác, mà là
một bản sao CHÍNH NÓ, lặp lại?
::::

::::reflect{#nghi-lai}
`hopNhat` Ở đây CHÍNH LÀ `ketHop` của một `Monoid<GCounter>` — chỉ
đổi tên cho rõ nghĩa trong ngữ cảnh CRDT. Nhưng monoid tổng quát
(R4) không đòi giao hoán; `hopNhat` của một CRDT thì BẮT BUỘC, vì
nguồn phân kỳ ở đây không phải "thứ tự các bước trong MỘT tiến trình"
(luôn cố định) mà là "thứ tự các bản sao GẶP nhau qua mạng" (không
ai đảm bảo trước). `kiemTraGiaoHoanVaKetHop` không chứng minh được
điều đó ĐÚNG cho MỌI `GCounter` có thể có — nó chỉ kiểm được từng bộ
ba cụ thể. Nhưng đo được trên số liệu thật vẫn đáng tin hơn nhiều so
với chỉ tin vào lời mô tả suông.
::::

::::checkpoint{mastery=0.73}
::::
