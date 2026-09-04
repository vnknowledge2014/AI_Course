---
id: thiet-ke-he-thong.crdt-va-hop-nhat.bo-dem-luot-xem-phan-tan
title: "Bộ đếm lượt xem phân tán: nhiều edge server, hợp nhất định kỳ"
summary: "interface KhoBanSao { id: string; boDem: GCounter } -- moi edge server (hcm, hanoi, danang, cantho) tu dem doc lap; hopNhatNhieuBanSao(cacBoDem) = cacBoDem.reduce(hopNhatGCounter, counterRong()) hop nhat CA mang GCounter thanh MOT; hop nhat theo THU TU A [hcm,hanoi,danang,cantho] va THU TU B [cantho,hanoi,hcm,danang] deu ra dung tong 275 -- ke ca khi mot vai ban sao xuat hien HAI LAN trong danh sach (mo phong chu ky dong bo lai dinh ky), tong VAN la 275, khong dem trung."
locale: vi
track: thiet-ke-he-thong
module: crdt-va-hop-nhat
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.fp.bo-dem-luot-xem-phan-tan]
requires: [sd.fp.so-sanh-chinh-sach-hop-nhat]
concepts: [sd.fp.bo-dem-luot-xem-phan-tan]
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
Quest trước, bài "exactly-once" chứng minh: hệ thống giao sự kiện
LẶP bao nhiêu lần theo THỨ TỰ nào cũng không sao, miễn `apDungIdempotent`
lọc trùng đúng. Hôm nay, cùng ý tưởng đó — nhưng Ở tầng HỢP NHẤT ĐA
BẢN SAO thay vì tầng "một sự kiện tới nhiều lần": nhiều edge server
đếm lượt xem độc lập, hợp nhất định kỳ, không theo thứ tự cố định
nào. Kết quả cuối có còn đúng không?
::::

::::explain{#nhieu-edge-server-hop-nhat-dinh-ky}
`KhoBanSao` đại diện một edge server — mỗi server GIỮ một `GCounter`
riêng, tự đếm lượt xem đến VỚI nó, không đợi server khác.
`hopNhatNhieuBanSao` gộp một MẢNG các `GCounter` (từ nhiều server)
thành một, bằng `reduce`. Hợp nhất theo hai thứ tự HOÀN TOÀN khác
nhau phải ra CÙNG một tổng:

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
interface KhoBanSao { id: string; boDem: GCounter; }
function taoKhoBanSao(id: string): KhoBanSao { return { id, boDem: counterRong() }; }
function ghiNhanLuotXem(kho: KhoBanSao, soLuot: number): KhoBanSao {
  return { id: kho.id, boDem: tangGCounter(kho.boDem, kho.id, soLuot) };
}
function hopNhatNhieuBanSao(cacBoDem: GCounter[]): GCounter {
  return cacBoDem.reduce(hopNhatGCounter, counterRong());
}

let hcm = taoKhoBanSao("hcm");
hcm = ghiNhanLuotXem(hcm, 50);
hcm = ghiNhanLuotXem(hcm, 70);
let hanoi = taoKhoBanSao("hanoi");
hanoi = ghiNhanLuotXem(hanoi, 80);
let danang = taoKhoBanSao("danang");
danang = ghiNhanLuotXem(danang, 45);
let cantho = taoKhoBanSao("cantho");
cantho = ghiNhanLuotXem(cantho, 30);

console.log("hcm.boDem:", JSON.stringify(hcm.boDem.theoReplica));
const thuTuA = hopNhatNhieuBanSao([hcm.boDem, hanoi.boDem, danang.boDem, cantho.boDem]);
console.log("hop nhat THEO THU TU A [hcm,hanoi,danang,cantho]:", JSON.stringify(thuTuA.theoReplica));
console.log("tong luot xem (thu tu A):", giaTriGCounter(thuTuA));

const thuTuB = hopNhatNhieuBanSao([cantho.boDem, hanoi.boDem, hcm.boDem, danang.boDem]);
console.log("hop nhat THEO THU TU B [cantho,hanoi,hcm,danang]:", JSON.stringify(thuTuB.theoReplica));
console.log("tong luot xem (thu tu B):", giaTriGCounter(thuTuB));
console.log("hai thu tu cho CUNG tong?", giaTriGCounter(thuTuA) === giaTriGCounter(thuTuB));
```

```text title=readonly
hcm.boDem: {"hcm":120}
hop nhat THEO THU TU A [hcm,hanoi,danang,cantho]: {"hcm":120,"hanoi":80,"danang":45,"cantho":30}
tong luot xem (thu tu A): 275
hop nhat THEO THU TU B [cantho,hanoi,hcm,danang]: {"cantho":30,"hanoi":80,"hcm":120,"danang":45}
tong luot xem (thu tu B): 275
hai thu tu cho CUNG tong? true
```

`hcm` tự đếm hai đợt (`50` rồi `70`, cộng dồn Ở đúng ô `"hcm"` thành
`120`) mà không cần biết ba server còn lại. Hợp nhất theo thứ tự A
(`hcm` trước) VÀ thứ tự B (`cantho` trước, hoàn toàn đảo lộn) đều ra
đúng CÙNG tổng: `120 + 80 + 45 + 30 = 275`. `theoReplica` của hai kết
quả có thứ tự KEY khác nhau trong JSON (do `reduce` spread key theo
thứ tự khác) — nhưng `giaTriGCounter` (tổng) không quan tâm thứ tự
key, chỉ cộng GIÁ TRỊ.
::::

::::example{#hop-nhat-dinh-ky-ban-sao-lap-khong-sao}
Trong thực tế, "hợp nhất định kỳ" nghĩa là CÙNG một edge server có
thể bị đưa vào danh sách hợp nhất NHIỀU lần qua các chu kỳ khác
nhau (đồng bộ lại sau một khoảng trễ mạng). Nhờ tính idempotent (bài
3), điều đó không làm sai kết quả:

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
interface KhoBanSao { id: string; boDem: GCounter; }
function taoKhoBanSao(id: string): KhoBanSao { return { id, boDem: counterRong() }; }
function ghiNhanLuotXem(kho: KhoBanSao, soLuot: number): KhoBanSao {
  return { id: kho.id, boDem: tangGCounter(kho.boDem, kho.id, soLuot) };
}
function hopNhatNhieuBanSao(cacBoDem: GCounter[]): GCounter {
  return cacBoDem.reduce(hopNhatGCounter, counterRong());
}

let hcm = taoKhoBanSao("hcm");
hcm = ghiNhanLuotXem(hcm, 50);
hcm = ghiNhanLuotXem(hcm, 70);
let hanoi = taoKhoBanSao("hanoi");
hanoi = ghiNhanLuotXem(hanoi, 80);
let danang = taoKhoBanSao("danang");
danang = ghiNhanLuotXem(danang, 45);
let cantho = taoKhoBanSao("cantho");
cantho = ghiNhanLuotXem(cantho, 30);

const thuTuC = hopNhatNhieuBanSao([hcm.boDem, hanoi.boDem, danang.boDem, cantho.boDem, hcm.boDem, danang.boDem]);
console.log("hop nhat CO LAP (hcm va danang xuat hien 2 lan, mo phong chu ky dong bo lai):", JSON.stringify(thuTuC.theoReplica));
console.log("tong luot xem (co lap):", giaTriGCounter(thuTuC));
```

```text title=readonly
hop nhat CO LAP (hcm va danang xuat hien 2 lan, mo phong chu ky dong bo lai): {"hcm":120,"hanoi":80,"danang":45,"cantho":30}
tong luot xem (co lap): 275
```

Danh sách hợp nhất Ở đây có `6` phần tử (`hcm` VÀ `danang` mỗi cái
xuất hiện `2` lần), nhưng tổng vẫn dừng đúng Ở `275` — GIỐNG HỆT khi
hợp nhất đúng `4` bản sao phân biệt (đoạn `explain`). Không cần lọc
trùng RIÊNG trước khi gọi `hopNhatNhieuBanSao`: mỗi lần `hopNhatGCounter`
gặp lại một ô đã biết, `Math.max` tự động không cộng thêm.
::::

::::predict{#doan-hop-nhat-danh-sach-mot-phan-tu-va-rong commitOnce}
Tiếp tục từ đoạn `explain`: `hopNhatNhieuBanSao([hcm.boDem])` (danh
sách CHỈ có đúng một phần tử) trả về `GCounter` có `giaTriGCounter`
là bao nhiêu? Còn `hopNhatNhieuBanSao([])` (danh sách RỖNG) thì sao?

:::opt{correct}
`120` cho danh sách chỉ có `hcm.boDem`, VÀ `0` cho danh sách rỗng —
`reduce` bắt đầu từ `counterRong()` (giá trị `0`) rồi hợp nhất LẦN
LƯỢT từng phần tử; với đúng một phần tử, kết quả CHÍNH LÀ phần tử đó
hợp nhất với rỗng (không đổi giá trị); với danh sách rỗng, `reduce`
không hề chạy vòng lặp nào, trả về NGUYÊN giá trị khởi tạo
:::
:::opt
Cả hai trường hợp đều gây LỖI runtime — `hopNhatNhieuBanSao` cần Ít
NHẤT hai phần tử để có gì đó "hợp nhất VỚI nhau"
::why
Nhầm "hợp nhất" với một phép toán chỉ có nghĩa giữa HAI giá trị trở
lên — nhưng `reduce` với giá trị khởi tạo tường minh
(`counterRong()`) hoạt động bình thường VỚI mọi độ dài mảng, kể cả
`0` hay `1` phần tử.

Chỗ lệch: `cacBoDem.reduce(hopNhatGCounter, counterRong())` LUÔN có
một điểm bắt đầu (`counterRong()`), nên `reduce` không cần Ít NHẤT
hai phần tử để chạy — với mảng rỗng, nó bỏ qua thân vòng lặp hoàn
toàn và trả về `counterRong()` (giá trị `0`); với đúng một phần tử,
nó gọi `hopNhatGCounter(counterRong(), phanTuDuyNhat)` đúng MỘT lần,
cho kết quả giống hệt chính phần tử đó (`counterRong()` là phần tử
trung tính của phép hợp nhất).
::
:::
::::

::::code{#viet_hop_nhat_nhieu_ban_sao}
Hoàn thiện `hopNhatNhieuBanSao` — dùng `reduce` để hợp nhất TOÀN bộ
các `GCounter` trong `cacBoDem` thành một, bắt đầu từ `counterRong()`.

```typescript title=starter
interface GCounter { theoReplica: Record<string, number>; }
function counterRong(): GCounter { return { theoReplica: {} }; }
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

function hopNhatNhieuBanSao(cacBoDem: GCounter[]): GCounter {
  ___
}

const repX: GCounter = { theoReplica: { repX: 12 } };
const repY: GCounter = { theoReplica: { repY: 8 } };
const tongHop = hopNhatNhieuBanSao([repX, repY, repX]);
console.log(giaTriGCounter(tongHop));
```

```typescript title=solution
interface GCounter { theoReplica: Record<string, number>; }
function counterRong(): GCounter { return { theoReplica: {} }; }
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

function hopNhatNhieuBanSao(cacBoDem: GCounter[]): GCounter {
  return cacBoDem.reduce(hopNhatGCounter, counterRong());
}

const repX: GCounter = { theoReplica: { repX: 12 } };
const repY: GCounter = { theoReplica: { repY: 8 } };
const tongHop = hopNhatNhieuBanSao([repX, repY, repX]);
console.log(giaTriGCounter(tongHop));
```

```typescript title=test
const s1: GCounter = { theoReplica: { s1: 40 } };
const s2: GCounter = { theoReplica: { s2: 25 } };
const s3: GCounter = { theoReplica: { s3: 10 } };

const kqThuTu1 = hopNhatNhieuBanSao([s1, s2, s3]);
const kqThuTu2 = hopNhatNhieuBanSao([s3, s1, s2]);
if (giaTriGCounter(kqThuTu1) !== 75) throw new Error("tong phai la 40 + 25 + 10 = 75");
if (giaTriGCounter(kqThuTu1) !== giaTriGCounter(kqThuTu2)) throw new Error("hop nhat theo hai thu tu KHAC nhau phai ra CUNG mot tong");

const kqCoLap = hopNhatNhieuBanSao([s1, s2, s3, s1, s2]);
if (giaTriGCounter(kqCoLap) !== 75) throw new Error("ban sao xuat hien LAP trong danh sach khong duoc lam tong tang len -- van phai la 75");

const kqRong = hopNhatNhieuBanSao([]);
if (giaTriGCounter(kqRong) !== 0) throw new Error("danh sach RONG phai tra ve gia tri 0 (counterRong)");

const truoc1 = JSON.stringify(s1.theoReplica);
const truoc2 = JSON.stringify(s2.theoReplica);
hopNhatNhieuBanSao([s1, s2]);
if (JSON.stringify(s1.theoReplica) !== truoc1) throw new Error("KHONG duoc mutate cac phan tu trong cacBoDem");
if (JSON.stringify(s2.theoReplica) !== truoc2) throw new Error("KHONG duoc mutate cac phan tu trong cacBoDem");
```

:::hints
- kind: attention
  body: "Dung cacBoDem.reduce(...) voi ham hopNhatGCounter da co san, gia tri khoi tao la counterRong(). Giong het hopNhatNhieuLan cua bai 3, chi doi ten tham so."
- kind: strategy
  body: "return cacBoDem.reduce(hopNhatGCounter, counterRong());"
- kind: one-line
  body: "return cacBoDem.reduce(hopNhatGCounter, counterRong());"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "20"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhiều edge server, hợp nhất theo thứ tự bất kỳ, số lần bất kỳ — tổng
luôn đúng. Ba CRDT (`GCounter`, `OR-Set`, `LWW-Register`) giờ đã có
đủ để ráp một dịch vụ phân tán nhỏ, hoàn chỉnh — không chỉ một bộ đếm
đơn lẻ.
::::

::::reflect{#nghi-lai}
Bài "exactly-once" (quest trước) chứng minh: một luồng sự kiện giao
LẶP, qua một phép áp dụng idempotent, cho hiệu ứng như giao đúng một
lần. Bài này chứng minh phiên bản tương ứng Ở TẦNG hợp nhất đa bản
sao: một danh sách các `GCounter` — bất kể thứ tự, bất kể số lần một
bản sao xuất hiện lại — qua `hopNhatNhieuBanSao`, luôn hội tụ về
đúng MỘT tổng. Hai chứng minh nhìn có vẻ khác nhau (một Ở tầng sự
kiện, một Ở tầng bản sao) nhưng dựa trên đúng MỘT nguyên lý: chọn một
phép kết hợp giao hoán, kết hợp, VÀ idempotent, rồi để nó tự lo phần
còn lại.
::::

::::checkpoint{mastery=0.84}
::::
