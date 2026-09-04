---
id: thiet-ke-he-thong.url-rut-gon-va-thu-thap-web.het-han-va-thu-hoi
title: "Hết hạn và thu hồi: tái sử dụng mã ngắn"
summary: "traCuuMaRutGon phân biệt BA trạng thái: \"khong-ton-tai\" (chưa từng cấp), \"het-han\" (đã cấp nhưng dh.thoiGianHienTai >= hetHanLuc — quy ước >=, chạm đúng mốc coi là hết hạn), \"hop-le\" (còn dùng được). Mã hết hạn bị xoá khỏi anhXa VÀ đẩy vào poolHetHan để THU HỒI tái sử dụng qua layMaTuPoolHetHan — khác TTL cache thuần tuý (chỉ xoá), đây còn tái chế mã. Mã p1 TTL=1000 đặt tại t=0: tại t=1000 (đúng biên) chuyển 'het-han', bị xoá khỏi anhXa và đẩy vào pool; tra cứu lại LẦN NỮA trả về 'khong-ton-tai' (không còn là 'het-han')."
locale: vi
track: thiet-ke-he-thong
module: url-rut-gon-va-thu-thap-web
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.het-han-va-thu-hoi]
requires: [sd.alias-tuy-chinh-va-va-cham]
concepts: [sd.het-han-va-thu-hoi]
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
Bốn bài trước tạo ra mã NGẮN không trùng — nhưng CHƯA hề dọn dẹp. Một
kho chứa cứ nhận mã MỚI mãi mãi, không bao GIỜ trả lại chỗ, sẽ phình
to VÔ hạn. Mã ngắn cũng CẦN có hạn dùng.
::::

::::explain{#ba-trang-thai}
Mỗi mã rút GỌN gắn với một `hetHanLuc` (thời điểm ẢO hết hạn, dùng
`DongHoMoPhong` tự viết — không đụng `Date.now()`, giống các bài Realm
6 trước). Tra CỨU một mã có thể rơi vào đúng BA trạng thái: chưa từng
CẤP, đã hết hạn, hay còn hợp LỆ. Khi phát hiện hết hạn, mục bị xoá
khỏi bản đồ CHÍNH VÀ được đẩy VÀO một "pool" để THU hồi tái sử dụng:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface MucRutGon { urlGoc: string; hetHanLuc: number; }
interface KhoRutGon { anhXa: Map<string, MucRutGon>; poolHetHan: string[]; }
function taoKhoRutGon(): KhoRutGon { return { anhXa: new Map(), poolHetHan: [] }; }

function datMaRutGon(kho: KhoRutGon, dh: DongHoMoPhong, ma: string, urlGoc: string, ttlMs: number): void {
  kho.anhXa.set(ma, { urlGoc, hetHanLuc: dh.thoiGianHienTai + ttlMs });
}

type TrangThaiTraCuu = "hop-le" | "het-han" | "khong-ton-tai";

function traCuuMaRutGon(kho: KhoRutGon, dh: DongHoMoPhong, ma: string): { trangThai: TrangThaiTraCuu; urlGoc: string | undefined } {
  const muc = kho.anhXa.get(ma);
  if (muc === undefined) return { trangThai: "khong-ton-tai", urlGoc: undefined };
  if (dh.thoiGianHienTai >= muc.hetHanLuc) {
    kho.anhXa.delete(ma);
    kho.poolHetHan.push(ma);
    return { trangThai: "het-han", urlGoc: undefined };
  }
  return { trangThai: "hop-le", urlGoc: muc.urlGoc };
}

const dh = taoDongHoMoPhong();
const kho = taoKhoRutGon();
datMaRutGon(kho, dh, "a1", "http://vidu.com/mua-sam", 1000);

console.log("tra cuu ma chua tung cap:", JSON.stringify(traCuuMaRutGon(kho, dh, "z9")));
console.log("tra cuu 'a1' luc con han:", JSON.stringify(traCuuMaRutGon(kho, dh, "a1")));

tienThoiGian(dh, 1000);
console.log("tra cuu 'a1' dung luc het han (t=1000):", JSON.stringify(traCuuMaRutGon(kho, dh, "a1")));
console.log("pool ma het han:", kho.poolHetHan);
```

```text title=readonly
tra cuu ma chua tung cap: {"trangThai":"khong-ton-tai"}
tra cuu 'a1' luc con han: {"trangThai":"hop-le","urlGoc":"http://vidu.com/mua-sam"}
tra cuu 'a1' dung luc het han (t=1000): {"trangThai":"het-han"}
pool ma het han: [ 'a1' ]
```

Mã `"z9"` chưa từng CẤP nên rơi vào `"khong-ton-tai"` NGAY. Mã
`"a1"` (TTL `1000`ms, đặt tại `t=0`) còn HỢP lệ tại thời điểm ĐẶT,
nhưng đúng LÚC `thoiGianHienTai` chạm `1000` (dùng `>=`, giống quy
ước bài TTL Ở Realm 6), nó chuyển SANG `"het-han"` VÀ mã `"a1"` được
đẩy vào `poolHetHan` để tái SỬ dụng SAU này.
::::

::::example{#thu-hoi-tai-su-dung}
Mục ĐÍCH của pool: khi một mã HẾT hạn (không ai truy cập trong lúc
còn hạn), thay VÌ để nó "chết" vĩnh viễn, hệ thống có thể LẤY nó ra
VÀ cấp lại cho một URL HOÀN toàn mới — tiết kiệm không gian mã NGẮN:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface MucRutGon { urlGoc: string; hetHanLuc: number; }
interface KhoRutGon { anhXa: Map<string, MucRutGon>; poolHetHan: string[]; }
function taoKhoRutGon(): KhoRutGon { return { anhXa: new Map(), poolHetHan: [] }; }

function datMaRutGon(kho: KhoRutGon, dh: DongHoMoPhong, ma: string, urlGoc: string, ttlMs: number): void {
  kho.anhXa.set(ma, { urlGoc, hetHanLuc: dh.thoiGianHienTai + ttlMs });
}

type TrangThaiTraCuu = "hop-le" | "het-han" | "khong-ton-tai";

function traCuuMaRutGon(kho: KhoRutGon, dh: DongHoMoPhong, ma: string): { trangThai: TrangThaiTraCuu; urlGoc: string | undefined } {
  const muc = kho.anhXa.get(ma);
  if (muc === undefined) return { trangThai: "khong-ton-tai", urlGoc: undefined };
  if (dh.thoiGianHienTai >= muc.hetHanLuc) {
    kho.anhXa.delete(ma);
    kho.poolHetHan.push(ma);
    return { trangThai: "het-han", urlGoc: undefined };
  }
  return { trangThai: "hop-le", urlGoc: muc.urlGoc };
}

function layMaTuPoolHetHan(kho: KhoRutGon): string | undefined {
  return kho.poolHetHan.pop();
}

const dh = taoDongHoMoPhong();
const kho = taoKhoRutGon();
datMaRutGon(kho, dh, "m1", "http://vidu.com/km-thang-1", 500);
datMaRutGon(kho, dh, "m2", "http://vidu.com/km-thang-2", 500);

tienThoiGian(dh, 500);
console.log("m1 (dung bien het han):", JSON.stringify(traCuuMaRutGon(kho, dh, "m1")));
console.log("m2 (dung bien het han):", JSON.stringify(traCuuMaRutGon(kho, dh, "m2")));
console.log("pool sau khi ca hai het han:", kho.poolHetHan);

const maTaiSuDung = layMaTuPoolHetHan(kho);
console.log("ma lay ra de tai su dung:", maTaiSuDung);
if (maTaiSuDung !== undefined) {
  datMaRutGon(kho, dh, maTaiSuDung, "http://vidu.com/km-thang-3-moi", 500);
  console.log("tra cuu lai ma da tai su dung:", JSON.stringify(traCuuMaRutGon(kho, dh, maTaiSuDung)));
}
console.log("pool con lai:", kho.poolHetHan);
```

```text title=readonly
m1 (dung bien het han): {"trangThai":"het-han"}
m2 (dung bien het han): {"trangThai":"het-han"}
pool sau khi ca hai het han: [ 'm1', 'm2' ]
ma lay ra de tai su dung: m2
tra cuu lai ma da tai su dung: {"trangThai":"hop-le","urlGoc":"http://vidu.com/km-thang-3-moi"}
pool con lai: [ 'm1' ]
```

`layMaTuPoolHetHan` LẤY mã CUỐI trong pool (`"m2"`, dùng `pop()`).
Sau khi CẤP lại mã ĐÓ cho một URL mới BẰNG `datMaRutGon`, tra cứu LẠI
"m2" trả về `"hop-le"` VỚI đúng URL mới — mã CŨ đã "sống lại" hoàn
toàn, không hề mang theo dấu VẾT của URL cũ.
::::

::::predict{#doan-tra-cuu-sau-khi-thu-hoi commitOnce}
Mã `"p1"` đã hết HẠN, bị xoá khỏi `anhXa` VÀ được đẩy vào
`poolHetHan`. KHÔNG gọi `datMaRutGon` lại — tra cứu `"p1"` một lần
NỮA ngay sau đó trả về `trangThai` gì?

:::opt{correct}
`"khong-ton-tai"` — mục đã bị XOÁ khỏi `anhXa` ở lần tra cứu ĐẦU tiên
phát hiện hết hạn, lần tra cứu THỨ hai không còn gì để so hạn nữa
:::
:::opt
`"het-han"` — mã VẪN còn nằm trong `poolHetHan`, nên hệ thống VẪN
"nhớ" nó từng hết hạn VÀ báo lại đúng trạng thái đó
::why
Nhầm "còn TRONG `poolHetHan` để tái sử DỤNG" với "còn TRONG `anhXa` để
tra CỨU thời hạn" — đây LÀ hai cấu trúc dữ liệu hoàn toàn TÁCH biệt.

Chỗ lệch: `traCuuMaRutGon` chỉ đọc TỪ `kho.anhXa`, KHÔNG hề nhìn vào
`poolHetHan`. Dòng ĐẦU tiên `const muc = kho.anhXa.get(ma);` cho ra
`undefined` (vì mục đã bị `delete` Ở lần tra cứu trước), NÊN hàm
thoát NGAY với `"khong-ton-tai"` — `poolHetHan` chỉ LÀ nơi "chứa mã
chờ CẤP lại", không phải nơi để TRA cứu trạng thái.
::
:::
::::

::::code{#viet_tra_cuu_ma_rut_gon}
Hoàn thiện `traCuuMaRutGon` — khi phát hiện đã hết hạn, xoá mục khỏi
`anhXa` VÀ đẩy mã VÀO `poolHetHan` để có thể THU hồi sau này.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface MucRutGon { urlGoc: string; hetHanLuc: number; }
interface KhoRutGon { anhXa: Map<string, MucRutGon>; poolHetHan: string[]; }
function taoKhoRutGon(): KhoRutGon { return { anhXa: new Map(), poolHetHan: [] }; }

function datMaRutGon(kho: KhoRutGon, dh: DongHoMoPhong, ma: string, urlGoc: string, ttlMs: number): void {
  kho.anhXa.set(ma, { urlGoc, hetHanLuc: dh.thoiGianHienTai + ttlMs });
}

type TrangThaiTraCuu = "hop-le" | "het-han" | "khong-ton-tai";

function traCuuMaRutGon(kho: KhoRutGon, dh: DongHoMoPhong, ma: string): { trangThai: TrangThaiTraCuu; urlGoc: string | undefined } {
  const muc = kho.anhXa.get(ma);
  if (muc === undefined) return { trangThai: "khong-ton-tai", urlGoc: undefined };
  if (dh.thoiGianHienTai >= muc.hetHanLuc) {
    ___
    return { trangThai: "het-han", urlGoc: undefined };
  }
  return { trangThai: "hop-le", urlGoc: muc.urlGoc };
}

const dh = taoDongHoMoPhong();
const kho = taoKhoRutGon();
datMaRutGon(kho, dh, "x1", "http://vidu.com/a", 100);
tienThoiGian(dh, 100);
console.log(traCuuMaRutGon(kho, dh, "x1").trangThai, kho.poolHetHan.length);
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface MucRutGon { urlGoc: string; hetHanLuc: number; }
interface KhoRutGon { anhXa: Map<string, MucRutGon>; poolHetHan: string[]; }
function taoKhoRutGon(): KhoRutGon { return { anhXa: new Map(), poolHetHan: [] }; }

function datMaRutGon(kho: KhoRutGon, dh: DongHoMoPhong, ma: string, urlGoc: string, ttlMs: number): void {
  kho.anhXa.set(ma, { urlGoc, hetHanLuc: dh.thoiGianHienTai + ttlMs });
}

type TrangThaiTraCuu = "hop-le" | "het-han" | "khong-ton-tai";

function traCuuMaRutGon(kho: KhoRutGon, dh: DongHoMoPhong, ma: string): { trangThai: TrangThaiTraCuu; urlGoc: string | undefined } {
  const muc = kho.anhXa.get(ma);
  if (muc === undefined) return { trangThai: "khong-ton-tai", urlGoc: undefined };
  if (dh.thoiGianHienTai >= muc.hetHanLuc) {
    kho.anhXa.delete(ma);
    kho.poolHetHan.push(ma);
    return { trangThai: "het-han", urlGoc: undefined };
  }
  return { trangThai: "hop-le", urlGoc: muc.urlGoc };
}

const dh = taoDongHoMoPhong();
const kho = taoKhoRutGon();
datMaRutGon(kho, dh, "x1", "http://vidu.com/a", 100);
tienThoiGian(dh, 100);
console.log(traCuuMaRutGon(kho, dh, "x1").trangThai, kho.poolHetHan.length);
```

```typescript title=test
const dhT = taoDongHoMoPhong();
const khoT = taoKhoRutGon();

if (traCuuMaRutGon(khoT, dhT, "chua-cap").trangThai !== "khong-ton-tai") throw new Error("ma chua tung cap phai la 'khong-ton-tai'");
if (traCuuMaRutGon(khoT, dhT, "chua-cap").urlGoc !== undefined) throw new Error("ma chua tung cap khong duoc co urlGoc");

datMaRutGon(khoT, dhT, "p1", "http://vidu.com/khuyen-mai", 1000);
const trCon = traCuuMaRutGon(khoT, dhT, "p1");
if (trCon.trangThai !== "hop-le" || trCon.urlGoc !== "http://vidu.com/khuyen-mai") throw new Error("con han phai tra ve 'hop-le' voi dung urlGoc");
if (khoT.anhXa.size !== 1) throw new Error("muc con han khong duoc bi xoa khoi anhXa");

tienThoiGian(dhT, 1000);
const trHet = traCuuMaRutGon(khoT, dhT, "p1");
if (trHet.trangThai !== "het-han" || trHet.urlGoc !== undefined) throw new Error("dung bien het han (>=) phai tra ve 'het-han', urlGoc undefined");
if (khoT.anhXa.has("p1")) throw new Error("ma het han phai bi xoa khoi anhXa");
if (khoT.poolHetHan.length !== 1 || khoT.poolHetHan[0] !== "p1") throw new Error("ma het han phai duoc THU HOI vao poolHetHan de tai su dung");

const trSauKhiXoa = traCuuMaRutGon(khoT, dhT, "p1");
if (trSauKhiXoa.trangThai !== "khong-ton-tai") throw new Error("sau khi entry da bi xoa, tra cuu lai phai la 'khong-ton-tai' chu khong phai 'het-han' nua");
```

:::hints
- kind: attention
  body: "Cho trong nam ngay TRUOC dong return { trangThai: 'het-han', ... } -- can lam HAI viec: xoa muc khoi anhXa, VA day ma vao poolHetHan."
- kind: strategy
  body: "kho.anhXa.delete(ma); kho.poolHetHan.push(ma);"
- kind: one-line
  body: "kho.anhXa.delete(ma); kho.poolHetHan.push(ma);"
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "het-han"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Rút gọn URL đã đủ mảnh: mã hoá, giải mã, cấp phát, chống va chạm, hết
hạn VÀ thu hồi. Giờ chuyển SANG nửa còn lại của quest — một crawler
đi khám phá WEB, bắt đầu bằng thứ tự khám phá đúng ĐẮN.
::::

::::reflect{#nghi-lai}
`traCuuMaRutGon` gộp CHUNG một hành động đọc VỚI một hành động dọn
dẹp — phát hiện hết hạn VÀ xoá NGAY trong cùng một lần gọi, thay vì
cần một tiến trình dọn RÁC chạy riêng. Đưa mã hết hạn VÀO pool biến
"xoá" thành "TÁI chế": không gian mã NGẮN không hề bị lãng phí vĩnh
viễn.
::::

::::checkpoint{mastery=0.73}
::::
