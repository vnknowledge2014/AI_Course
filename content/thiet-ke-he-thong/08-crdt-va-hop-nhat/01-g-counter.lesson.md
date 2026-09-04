---
id: thiet-ke-he-thong.crdt-va-hop-nhat.g-counter
title: "G-Counter: mỗi replica tự đếm, hợp nhất bằng MAX không phải cộng"
summary: "interface GCounter { theoReplica: Record<string, number> } -- tang(gc, replicaId, buoc) CHI cong vao o cua DUNG replica do; hopNhat(a,b) lay MAX tung replica, KHONG cong don -- gcA va gcB deu da tung dong bo voi replica 'c' (gia tri 5), hop nhat SAI kieu cong ra 15 (dem trung phan c), hop nhat DUNG bang max ra 10, dung bang 5 + 2 + 3."
locale: vi
track: thiet-ke-he-thong
module: crdt-va-hop-nhat
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.fp.g-counter]
requires: [sd.fp.boss-nhat-ky-bat-bien-va-fold]
concepts: [sd.fp.g-counter]
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
Quest trước kết ở `xuLyLenhDem` — một `kho` DUY NHẤT giữ nhật ký, mọi
lệnh đi qua ĐÚNG một nơi để quyết định đúng hay sai. Nhưng nếu hệ
thống có NHIỀU server, mỗi server nhận request riêng, không server nào
đợi server khác để trả lời — không hề có một `kho` chung nào cả, chỉ
có nhiều bản sao (replica) độc lập, thỉnh thoảng mới gặp nhau để hợp
nhất (merge) dữ liệu. Track mới hỏi: hợp nhất kiểu gì để KHÔNG cần một
trung tâm điều phối, mà vẫn ra kết quả đúng?
::::

::::explain{#g-counter-dem-doc-lap-hop-nhat-bang-max}
`GCounter` (grow-only counter) giữ một bộ đếm RIÊNG cho từng replica —
`theoReplica` ánh xạ `replicaId` tới số lần replica đó đã tăng. `tang`
chỉ cộng vào Ô CỦA ĐÚNG replica gọi nó, không đụng tới ô của replica
khác. `giaTri` là tổng tất cả các ô. Điểm mấu chốt nằm ở `hopNhat`: nó
lấy MAX từng ô, KHÔNG cộng dồn — vì hai bản sao có thể ĐÃ từng đồng bộ
với nhau trước đó, và cộng dồn sẽ đếm lại phần đã biết chung:

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
function hopNhatSaiKieuCong(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = hienTai + soDem;
  }
  return { theoReplica: ketQua };
}
function hopNhat(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}

let gcC = counterRong();
gcC = tang(gcC, "c", 5);

let gcA = hopNhat(counterRong(), gcC);
gcA = tang(gcA, "a", 2);

let gcB = hopNhat(counterRong(), gcC);
gcB = tang(gcB, "b", 3);

console.log("gcA (da biet ve c, tu tang them 2):", JSON.stringify(gcA));
console.log("gcB (da biet ve c, tu tang them 3):", JSON.stringify(gcB));

const sai = hopNhatSaiKieuCong(gcA, gcB);
console.log("hop nhat SAI (cong don theoReplica.c):", JSON.stringify(sai), "gia tri:", giaTri(sai));

const dung = hopNhat(gcA, gcB);
console.log("hop nhat DUNG (lay MAX theoReplica.c):", JSON.stringify(dung), "gia tri:", giaTri(dung));
```

```text title=readonly
gcA (da biet ve c, tu tang them 2): {"theoReplica":{"c":5,"a":2}}
gcB (da biet ve c, tu tang them 3): {"theoReplica":{"c":5,"b":3}}
hop nhat SAI (cong don theoReplica.c): {"theoReplica":{"c":10,"a":2,"b":3}} gia tri: 15
hop nhat DUNG (lay MAX theoReplica.c): {"theoReplica":{"c":5,"a":2,"b":3}} gia tri: 10
```

`gcA` VÀ `gcB` đều đã từng đồng bộ với replica `"c"` (cả hai đều biết
`c` đang Ở mức `5`) TRƯỚC khi mỗi bên tự tăng riêng. Nếu hợp nhất bằng
cách CỘNG (`hopNhatSaiKieuCong`), ô `c` bị cộng `5 + 5`, ra `10` —
SAI, vì `c` chỉ thực sự đóng góp `5` vào tổng, không phải `10`; kết
quả cuối `15` cao hơn sự thật. Hợp nhất bằng MAX (`hopNhat`) lấy
`Math.max(5, 5) = 5` cho ô `c`, giữ nguyên `a: 2` VÀ `b: 3` — tổng
đúng bằng `10`. Đây chính là lý do phải dùng MAX: mỗi ô ghi lại "mức
CAO NHẤT đã từng thấy" của một replica, không phải "cộng dồn mọi lần
nghe được về replica đó".
::::

::::example{#nhieu-replica-khong-kho-trung-tam}
Ba server độc lập đếm request của riêng mình — không server nào biết
tới sự tồn tại của hai server còn lại cho tới khi hợp nhất. `hopNhat`
gộp từng cặp một, không cần một `kho` trung tâm nào đứng ra điều phối
như `xuLyLenhDem` của quest trước:

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

let s1 = counterRong();
s1 = tang(s1, "server-1", 4);
s1 = tang(s1, "server-1", 6);

let s2 = counterRong();
s2 = tang(s2, "server-2", 7);

let s3 = counterRong();
s3 = tang(s3, "server-3", 2);
s3 = tang(s3, "server-3", 1);

const s12 = hopNhat(s1, s2);
const s123 = hopNhat(s12, s3);
console.log("s1 (server-1, tu dem, khong biet server khac):", JSON.stringify(s1));
console.log("s123 sau khi hop nhat CA BA (khong qua kho trung tam nao):", JSON.stringify(s123));
console.log("tong so luot dem toan he thong:", giaTri(s123));
```

```text title=readonly
s1 (server-1, tu dem, khong biet server khac): {"theoReplica":{"server-1":10}}
s123 sau khi hop nhat CA BA (khong qua kho trung tam nao): {"theoReplica":{"server-1":10,"server-2":7,"server-3":3}}
tong so luot dem toan he thong: 20
```

`server-1` tự tăng hai lần (`4` rồi `6`, cộng dồn Ở ĐÚNG ô của nó
thành `10`) mà không hề biết `server-2` hay `server-3` tồn tại. Hợp
nhất cặp `s1` với `s2`, rồi hợp nhất kết quả đó với `s3` — không có
bước nào cần một nơi giữ TOÀN bộ trạng thái trước khi tính; mỗi
`hopNhat` chỉ cần ĐÚNG HAI bản sao đang có trong tay. Tổng cuối
(`20`) đúng bằng `10 + 7 + 3`, khớp với việc cả ba server đã đếm.
::::

::::predict{#doan-hop-nhat-hai-replica-trung-ten commitOnce}
Giả sử do một lỗi cấu hình, hai server dùng CHUNG một `replicaId`:
`gcp1 = { theoReplica: { s1: 5 } }` VÀ `gcp2 = { theoReplica: { s1: 3,
s2: 4 } }` (cả hai đều có ô `"s1"`, nhưng với giá trị KHÁC nhau).
`giaTri(hopNhat(gcp1, gcp2))` là bao nhiêu?

:::opt{correct}
`9` — `hopNhat` lấy `Math.max(5, 3) = 5` cho ô `"s1"`, cộng với `4`
của ô `"s2"`, tổng đúng bằng `5 + 4 = 9`; giá trị `3` của `gcp2` Ở ô
`"s1"` bị THAY bằng `5` (lớn hơn), không được cộng thêm vào
:::
:::opt
`12` — vì tổng số phải phản ánh ĐẦY ĐỦ những gì cả hai bên đã đếm:
`5` (của `gcp1`) cộng `3` (của `gcp2` Ở `"s1"`) cộng `4` (của `gcp2`
Ở `"s2"`)
::why
Nhầm `hopNhat` với một phép CỘNG tổng quát — nhưng dòng
`ketQua[replicaId] = Math.max(hienTai, soDem);` trong thân hàm dùng
`Math.max`, không phải `+`, cho MỌI ô trùng tên, không riêng gì
trường hợp "hai bên đã từng đồng bộ".

Chỗ lệch: `hopNhat` không phân biệt được "ô `s1` trùng vì hai bên
từng đồng bộ" với "ô `s1` trùng vì cấu hình sai" — nó LUÔN lấy MAX cho
mọi ô trùng tên, không có ngoại lệ. Với ô `"s1"`, `Math.max(5, 3)`
cho `5`, không phải `5 + 3 = 8`. Ô `"s2"` không trùng nên giữ nguyên
`4`. Tổng đúng là `5 + 4 = 9`.
::
:::
::::

::::code{#viet_hop_nhat}
Hoàn thiện `hopNhat` — với mỗi replica trong `b.theoReplica`, lấy giá
trị LỚN HƠN giữa ô tương ứng của `a` (mặc định `0` nếu chưa có) và
giá trị của `b`, rồi gộp vào một `GCounter` MỚI. KHÔNG được sửa `a`
hay `b` truyền vào.

```typescript title=starter
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
  ___
}

const nkX: GCounter = { theoReplica: { rep1: 3 } };
const nkY: GCounter = { theoReplica: { rep1: 5, rep2: 2 } };
const nkZ = hopNhat(nkX, nkY);
console.log(JSON.stringify(nkZ.theoReplica), giaTri(nkZ));
```

```typescript title=solution
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

const nkX: GCounter = { theoReplica: { rep1: 3 } };
const nkY: GCounter = { theoReplica: { rep1: 5, rep2: 2 } };
const nkZ = hopNhat(nkX, nkY);
console.log(JSON.stringify(nkZ.theoReplica), giaTri(nkZ));
```

```typescript title=test
const a1: GCounter = { theoReplica: { r1: 3, r2: 7 } };
const b1: GCounter = { theoReplica: { r2: 5, r3: 2 } };
const kq1 = hopNhat(a1, b1);
if (kq1.theoReplica["r1"] !== 3) throw new Error("o CHI co o a phai giu nguyen gia tri cua a");
if (kq1.theoReplica["r2"] !== 7) throw new Error("o trung phai lay MAX(7,5) = 7, khong phai cong don");
if (kq1.theoReplica["r3"] !== 2) throw new Error("o CHI co o b phai lay gia tri cua b");
if (giaTri(kq1) !== 12) throw new Error("tong phai la 3 + 7 + 2 = 12");

const a1Sau = JSON.stringify(a1.theoReplica);
const b1Sau = JSON.stringify(b1.theoReplica);
if (a1Sau !== '{"r1":3,"r2":7}') throw new Error("hopNhat KHONG duoc mutate a truyen vao");
if (b1Sau !== '{"r2":5,"r3":2}') throw new Error("hopNhat KHONG duoc mutate b truyen vao");

const rong: GCounter = { theoReplica: {} };
const kqRong = hopNhat(rong, a1);
if (giaTri(kqRong) !== giaTri(a1)) throw new Error("hop nhat voi GCounter rong phai giu nguyen gia tri kia (phan tu trung tinh)");
```

:::hints
- kind: attention
  body: "Sao chep theoReplica cua a truoc ({ ...a.theoReplica }). Sau do voi TUNG cap [replicaId, soDem] cua b.theoReplica (dung Object.entries), lay Math.max giua gia tri hien co trong ketQua (mac dinh 0 neu chua co) va soDem cua b, roi gan lai vao ketQua[replicaId]."
- kind: strategy
  body: "const ketQua: Record<string, number> = { ...a.theoReplica }; for (const [replicaId, soDem] of Object.entries(b.theoReplica)) { const hienTai = ketQua[replicaId] ?? 0; ketQua[replicaId] = Math.max(hienTai, soDem); } return { theoReplica: ketQua };"
- kind: one-line
  body: "const ketQua: Record<string, number> = { ...a.theoReplica }; for (const [replicaId, soDem] of Object.entries(b.theoReplica)) { const hienTai = ketQua[replicaId] ?? 0; ketQua[replicaId] = Math.max(hienTai, soDem); } return { theoReplica: ketQua };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: '{"rep1":5,"rep2":2} 7'
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không cần một trung tâm nào ra lệnh — mỗi replica tự đếm, hợp nhất
bằng MAX là đủ để hội tụ đúng. Nhưng "hội tụ đúng bất kể thứ tự nhận"
mới chỉ là LỜI HỨA — đã ai kiểm chứng nó bằng số thật chưa?
::::

::::reflect{#nghi-lai}
`hopNhat` không hề biết "hai replica này đã từng gặp nhau chưa" —
nó chỉ áp dụng ĐÚNG một luật, MAX từng ô, cho MỌI cặp bản sao, bất kể
lịch sử đồng bộ giữa chúng ra sao. Đây chính là điểm khác biệt cốt
lõi với `xuLyLenhDem` của quest trước: Ở đó, đúng đắn đến từ việc có
đúng MỘT nơi giữ trạng thái; Ở đây, đúng đắn đến từ việc phép hợp
nhất được chọn ĐÚNG — không phụ thuộc router nào biết trước ai đã nói
chuyện với ai.
::::

::::checkpoint{mastery=0.72}
::::
