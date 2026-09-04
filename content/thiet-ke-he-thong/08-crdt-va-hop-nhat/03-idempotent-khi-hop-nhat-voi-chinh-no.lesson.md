---
id: thiet-ke-he-thong.crdt-va-hop-nhat.idempotent-khi-hop-nhat-voi-chinh-no
title: "Idempotent khi hợp nhất: gộp một bản sao với chính nó, không đổi gì"
summary: "hopNhat(gc, gc) tra ve gia tri CAU TRUC GIONG HET gc goc -- khong can mot Set 'idDaApDung' rieng nhu apDungIdempotent cua quest truoc, tinh idempotent nay den MIEN PHI tu chinh Math.max (max(x,x) = x); hopNhatNhieuLan(banDau, cacBanSao) dung reduce de hop nhat voi MOT danh sach ban sao co the LAP (gossip gui trung) -- hop nhat 1 lan hay 3 lan CUNG mot ban sao deu ra dung 16."
locale: vi
track: thiet-ke-he-thong
module: crdt-va-hop-nhat
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.fp.idempotent-khi-hop-nhat-voi-chinh-no]
requires: [sd.fp.hop-nhat-giao-hoan-va-ket-hop]
concepts: [sd.fp.idempotent-khi-hop-nhat-voi-chinh-no]
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
Quest trước, `apDungIdempotent` phải TỰ xây một `Set<string>` riêng
(`idDaApDung`) để nhớ sự kiện nào đã áp dụng rồi, tránh cộng trùng.
Câu hỏi hôm nay: khi hai bản sao gặp lại NHAU (không phải một sự kiện
lặp, mà chính hai `GCounter` giống hệt nhau, đến từ gossip protocol
gửi trùng) — `hopNhat` có cần một cơ chế chống trùng RIÊNG như vậy
không, hay tính chất đó đã có sẵn?
::::

::::explain{#hop-nhat-voi-chinh-no-khong-doi-gi}
Hợp nhất một `GCounter` VỚI CHÍNH nó — `hopNhat(gc, gc)` — phải trả về
một giá trị có CẤU TRÚC giống hệt `gc`. Không cần một tập `idDaApDung`
nào theo dõi "đã hợp nhất bản sao này chưa" — tính chất này đến MIỄN
PHÍ từ chính `Math.max`, vì `Math.max(x, x)` luôn LÀ `x`:

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

let gc = tang(counterRong(), "s1", 5);
gc = tang(gc, "s2", 3);
console.log("gc:", JSON.stringify(gc.theoReplica));
const tuHopNhat = hopNhat(gc, gc);
console.log("hopNhat(gc,gc):", JSON.stringify(tuHopNhat.theoReplica));
console.log("cau truc GIONG HET gc goc?", ganNhauCauTruc(gc, tuHopNhat));
console.log("giaTri khong doi?", giaTri(gc), giaTri(tuHopNhat));
```

```text title=readonly
gc: {"s1":5,"s2":3}
hopNhat(gc,gc): {"s1":5,"s2":3}
cau truc GIONG HET gc goc? true
giaTri khong doi? 8 8
```

Không hề có bước "kiểm tra xem hai tham số có phải CÙNG một object
không rồi bỏ qua" trong `hopNhat` — thân hàm chỉ đơn giản lấy MAX
từng ô như mọi lần khác. Với `hopNhat(gc, gc)`, mỗi ô đang so
`Math.max(x, x)`, và với BẤT KỲ số `x` nào, `Math.max(x, x)` LUÔN
bằng chính `x` — nên kết quả không thể khác `gc`. Không cần một Set
riêng theo dõi "đã thấy bản sao này chưa"; tính idempotent nằm sẵn
trong chính phép toán được chọn để hợp nhất.
::::

::::example{#gossip-gui-trung-hop-nhat-nhieu-lan-khong-sao}
Trong gossip protocol, cùng một trạng thái remote có thể tới một
replica qua NHIỀU đường khác nhau — `hopNhatNhieuLan` gộp một danh
sách bản sao (có thể trùng) vào một `GCounter` ban đầu bằng `reduce`:

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
function hopNhatNhieuLan(banDau: GCounter, cacBanSao: GCounter[]): GCounter {
  return cacBanSao.reduce(hopNhat, banDau);
}

const banDau = tang(counterRong(), "local", 10);
const gcRemote = tang(tang(counterRong(), "remote", 4), "other", 2);
const mot_lan = hopNhatNhieuLan(banDau, [gcRemote]);
const ba_lan = hopNhatNhieuLan(banDau, [gcRemote, gcRemote, gcRemote]);
console.log("hop nhat 1 lan:", JSON.stringify(mot_lan.theoReplica), "gia tri:", giaTri(mot_lan));
console.log("hop nhat 3 lan (gossip gui trung 3 lan):", JSON.stringify(ba_lan.theoReplica), "gia tri:", giaTri(ba_lan));
console.log("giong het nhau?", ganNhauCauTruc(mot_lan, ba_lan));
```

```text title=readonly
hop nhat 1 lan: {"local":10,"remote":4,"other":2} gia tri: 16
hop nhat 3 lan (gossip gui trung 3 lan): {"local":10,"remote":4,"other":2} gia tri: 16
giong het nhau? true
```

`gcRemote` được đưa vào danh sách BA lần — mô phỏng cùng một cập
nhật đến qua ba đường gossip khác nhau. Kết quả sau khi hợp nhất `3`
lần GIỐNG HỆT kết quả sau khi hợp nhất đúng `1` lần: cả hai đều dừng
Ở `16` (`10 + 4 + 2`). Không có bước lọc trùng RIÊNG nào trong
`hopNhatNhieuLan` — nó chỉ gọi `reduce` với `hopNhat`, và mỗi lần
`hopNhat` gặp lại đúng những giá trị đã biết, `Math.max` tự động
không cộng thêm gì.
::::

::::predict{#doan-hop-nhat-ba-lan-lien-tiep commitOnce}
Tiếp tục từ đoạn `explain`: `gc` có `giaTri(gc) = 8`. Gọi
`hopNhat(hopNhat(gc, gc), gc)` — hợp nhất `gc` VỚI CHÍNH nó, rồi hợp
nhất kết quả đó VỚI `gc` một lần NỮA — `giaTri` của kết quả cuối cùng
là bao nhiêu?

:::opt{correct}
Vẫn là `8` — mỗi lần `hopNhat` gặp lại `gc`, từng ô đều tính
`Math.max` giữa hai giá trị BẰNG NHAU, luôn trả về đúng giá trị đó;
hợp nhất một `GCounter` với chính nó bao nhiêu lần cũng không đổi
:::
:::opt
`24` — vì `hopNhat` được gọi tổng cộng HAI lần trong biểu thức, mỗi
lần hợp nhất phải cộng dồn thêm một lượt giá trị của `gc` (`8 + 8 +
8`)
::why
Nhầm "số lần gọi `hopNhat`" với "số lần giá trị được cộng thêm" —
nhưng `hopNhat` không hề cộng, nó lấy `Math.max` cho từng ô.

Chỗ lệch: dòng `ketQua[replicaId] = Math.max(hienTai, soDem);` là
phép toán DUY nhất thay đổi giá trị Ở mỗi ô. Với `hopNhat(gc, gc)`,
mọi ô đang so `Math.max` giữa hai số bằng nhau — kết quả luôn là
chính số đó, không tăng thêm. Gọi `hopNhat` thêm một lần nữa với
`gc` cũng vậy: `Math.max(8_da_co, 8_cua_gc)` vẫn là `8`. Số lần GỌI
hàm không liên quan gì tới việc giá trị có bị cộng dồn hay không —
điều đó phụ thuộc vào PHÉP TOÁN bên trong hàm.
::
:::
::::

::::code{#viet_hop_nhat_nhieu_lan}
Hoàn thiện `hopNhatNhieuLan` — dùng `Array.prototype.reduce` để hợp
nhất `banDau` lần lượt VỚI từng phần tử trong `cacBanSao` (theo đúng
thứ tự mảng), trả về kết quả cuối cùng.

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
function giaTri(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}

function hopNhatNhieuLan(banDau: GCounter, cacBanSao: GCounter[]): GCounter {
  ___
}

const banDauX: GCounter = { theoReplica: { x: 7 } };
const remoteX: GCounter = { theoReplica: { y: 9 } };
const ketQuaX = hopNhatNhieuLan(banDauX, [remoteX, remoteX]);
console.log(JSON.stringify(ketQuaX.theoReplica), giaTri(ketQuaX));
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
function giaTri(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}

function hopNhatNhieuLan(banDau: GCounter, cacBanSao: GCounter[]): GCounter {
  return cacBanSao.reduce(hopNhat, banDau);
}

const banDauX: GCounter = { theoReplica: { x: 7 } };
const remoteX: GCounter = { theoReplica: { y: 9 } };
const ketQuaX = hopNhatNhieuLan(banDauX, [remoteX, remoteX]);
console.log(JSON.stringify(ketQuaX.theoReplica), giaTri(ketQuaX));
```

```typescript title=test
const banDauT: GCounter = { theoReplica: { local: 100 } };
const remoteT: GCounter = { theoReplica: { r1: 5, r2: 2 } };

const ketQuaMotLan = hopNhatNhieuLan(banDauT, [remoteT]);
if (giaTri(ketQuaMotLan) !== 107) throw new Error("hop nhat 1 lan phai ra 100 + 5 + 2 = 107");

const ketQuaNamLan = hopNhatNhieuLan(banDauT, [remoteT, remoteT, remoteT, remoteT, remoteT]);
if (giaTri(ketQuaNamLan) !== 107) throw new Error("hop nhat CUNG mot ban sao 5 lan phai ra GIONG HET 1 lan (idempotent) -- van la 107");

const ketQuaRong = hopNhatNhieuLan(banDauT, []);
if (giaTri(ketQuaRong) !== 100) throw new Error("danh sach ban sao RONG phai giu nguyen banDau -- van la 100");

const truocBanDau = JSON.stringify(banDauT.theoReplica);
const truocRemote = JSON.stringify(remoteT.theoReplica);
hopNhatNhieuLan(banDauT, [remoteT, remoteT]);
if (JSON.stringify(banDauT.theoReplica) !== truocBanDau) throw new Error("KHONG duoc mutate banDau truyen vao");
if (JSON.stringify(remoteT.theoReplica) !== truocRemote) throw new Error("KHONG duoc mutate cac phan tu trong cacBanSao");
```

:::hints
- kind: attention
  body: "Dung cacBanSao.reduce(...) voi ham hopNhat da co san, gia tri khoi tao la banDau. Khong tu viet vong lap thu cong -- reduce voi hopNhat lam dung viec can lam."
- kind: strategy
  body: "return cacBanSao.reduce(hopNhat, banDau);"
- kind: one-line
  body: "return cacBanSao.reduce(hopNhat, banDau);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: '{"x":7,"y":9} 16'
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Gộp một bản sao với chính nó — hay với mười bản sao trùng lặp — không
làm sai lệch gì cả. Nhưng `GCounter` mới chỉ đếm LÊN. Nếu cần đếm
CẢ chiều xuống — huỷ một lượt thích, rút một mục hàng — `tang` với
bước ÂM có dùng được không?
::::

::::reflect{#nghi-lai}
Ở quest trước, `apDungIdempotent` phải TỰ xây một cơ chế chống trùng
— một `Set<string>` sống bên trong `TrangThai`, kiểm tra TRƯỚC mỗi
lần áp dụng. `hopNhat` của `GCounter` không cần cơ chế nào tương tự:
tính idempotent không phải một TÍNH NĂNG được THÊM vào, mà là một HỆ
QUẢ tự nhiên của việc chọn `Math.max` làm phép hợp nhất. Đây là một
bài học rộng hơn CRDT: đôi khi, thay vì viết thêm code để CHỐNG một
vấn đề, chọn đúng phép toán khiến vấn đề đó không thể xảy ra.
::::

::::checkpoint{mastery=0.75}
::::
