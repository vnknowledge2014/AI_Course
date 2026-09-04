---
id: thiet-ke-he-thong.do-truoc-khi-thiet-ke.cache-ben-canh-doc-va-ghi
title: "Cache-aside: đọc và ghi"
summary: "docCacheAside tra cache TRƯỚC (hit -> trả ngay, không đụng nguồn); miss thì đọc nguồn chậm RỒI ghi vào cache. ghiCacheAside cập nhật nguồn RỒI xoá entry khỏi cache (invalidate, không update tại chỗ). Với nguồn {userA:100,userB:200}: đọc userA lần 1 (miss) -> nguồn.soLanDuocGoi=1; đọc userA lần 2 (hit) -> soLanDuocGoi VẪN=1; ghi userB=999 rồi đọc lại -> cache.has('userB')=false ngay sau ghi, giá trị đọc lại=999, soLanDuocGoi cuối=2."
locale: vi
track: thiet-ke-he-thong
module: do-truoc-khi-thiet-ke
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.cache-ben-canh-doc-va-ghi]
requires: [sd.can-bang-tai-it-ket-noi-nhat]
concepts: [sd.cache-ben-canh-doc-va-ghi]
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
Load balancer chọn được server (bài trước). Nhưng MỖI request tới
vẫn đụng thẳng NGUỒN dữ liệu — nếu nguồn ĐÓ chậm (một database THẬT),
`111` request/giây (bài 4) dồn hết VÀO nó thì sao?
::::

::::explain{#doc-cache-aside}
"Cache-aside" LÀ mẫu hình phổ biến nhất: ứng dụng tự QUẢN lý cache,
KHÔNG để database biết GÌ về nó. Đọc: tra cache TRƯỚC — cache HIT thì
trả về NGAY, không đụng nguồn; cache MISS thì đọc nguồn (chậm), RỒI
ghi kết quả VÀO cache cho lần sau:

```typescript title=readonly
interface NguonCham { duLieu: Map<string, number>; soLanDuocGoi: number; }
function taoNguonCham(duLieuBanDau: [string, number][]): NguonCham {
  return { duLieu: new Map(duLieuBanDau), soLanDuocGoi: 0 };
}
function docTuNguon(nguon: NguonCham, khoa: string): number | undefined {
  nguon.soLanDuocGoi += 1;
  return nguon.duLieu.get(khoa);
}

interface BoNhoDem { duLieu: Map<string, number>; }
function taoBoNhoDem(): BoNhoDem { return { duLieu: new Map() }; }

function docCacheAside(cache: BoNhoDem, nguon: NguonCham, khoa: string): number | undefined {
  if (cache.duLieu.has(khoa)) return cache.duLieu.get(khoa);
  const giaTri = docTuNguon(nguon, khoa);
  if (giaTri !== undefined) cache.duLieu.set(khoa, giaTri);
  return giaTri;
}

const nguon = taoNguonCham([["userA", 100], ["userB", 200]]);
const cache = taoBoNhoDem();

console.log("doc userA lan 1 (mo dau, chua co trong cache):", docCacheAside(cache, nguon, "userA"));
console.log("soLanDuocGoi sau lan 1:", nguon.soLanDuocGoi);
console.log("doc userA lan 2 (phai HIT):", docCacheAside(cache, nguon, "userA"));
console.log("soLanDuocGoi sau lan 2 (khong doi vi HIT):", nguon.soLanDuocGoi);
```

```text title=readonly
doc userA lan 1 (mo dau, chua co trong cache): 100
soLanDuocGoi sau lan 1: 1
doc userA lan 2 (phai HIT): 100
soLanDuocGoi sau lan 2 (khong doi vi HIT): 1
```

Lần ĐỌC đầu ("miss") gọi `docTuNguon` — `soLanDuocGoi` tăng LÊN `1`.
Lần đọc THỨ hai, `cache.duLieu.has("userA")` đã LÀ `true` (được ghi Ở
lần TRƯỚC), NÊN trả về NGAY từ cache — `docTuNguon` KHÔNG được gọi
thêm, `soLanDuocGoi` giữ NGUYÊN `1`.
::::

::::example{#ghi-cache-aside}
Ghi: quy tắc LÀ cập nhật NGUỒN trước, RỒI xoá entry cũ khỏi cache
(chứ KHÔNG cập nhật cache TẠI chỗ) — lần đọc SAU sẽ tự miss VÀ nạp
LẠI giá trị mới nhất:

```typescript title=readonly
interface NguonCham { duLieu: Map<string, number>; soLanDuocGoi: number; }
function taoNguonCham(duLieuBanDau: [string, number][]): NguonCham {
  return { duLieu: new Map(duLieuBanDau), soLanDuocGoi: 0 };
}
function docTuNguon(nguon: NguonCham, khoa: string): number | undefined {
  nguon.soLanDuocGoi += 1;
  return nguon.duLieu.get(khoa);
}
interface BoNhoDem { duLieu: Map<string, number>; }
function taoBoNhoDem(): BoNhoDem { return { duLieu: new Map() }; }
function docCacheAside(cache: BoNhoDem, nguon: NguonCham, khoa: string): number | undefined {
  if (cache.duLieu.has(khoa)) return cache.duLieu.get(khoa);
  const giaTri = docTuNguon(nguon, khoa);
  if (giaTri !== undefined) cache.duLieu.set(khoa, giaTri);
  return giaTri;
}
function ghiVaoNguon(nguon: NguonCham, khoa: string, giaTri: number): void {
  nguon.duLieu.set(khoa, giaTri);
}
function ghiCacheAside(cache: BoNhoDem, nguon: NguonCham, khoa: string, giaTri: number): void {
  ghiVaoNguon(nguon, khoa, giaTri);
  cache.duLieu.delete(khoa);
}

// tai lap dung trang thai da co o khoi truoc: da doc userA hai lan (mot mo dau, mot HIT)
const nguon = taoNguonCham([["userA", 100], ["userB", 200]]);
const cache = taoBoNhoDem();
docCacheAside(cache, nguon, "userA");
docCacheAside(cache, nguon, "userA");

ghiCacheAside(cache, nguon, "userB", 999);
console.log("cache co con giu userB sau ghi khong:", cache.duLieu.has("userB"));
console.log("doc userB sau ghi:", docCacheAside(cache, nguon, "userB"));
console.log("soLanDuocGoi cuoi cung:", nguon.soLanDuocGoi);
```

```text title=readonly
cache co con giu userB sau ghi khong: false
doc userB sau ghi: 999
soLanDuocGoi cuoi cung: 2
```

`ghiCacheAside` KHÔNG hề chạm tới GIÁ trị `999` trong cache — nó XOÁ
entry `userB` khỏi cache (nếu CÓ), để lần đọc TIẾP theo phải đi qua
`docCacheAside`, miss, VÀ nạp lại đúng giá trị MỚI (`999`) từ nguồn.
`soLanDuocGoi` tăng lên `2` — đúng MỘT lần cho lần đọc SAU khi ghi.
::::

::::predict{#doan-doc-sau-ghi commitOnce}
Ngay SAU `ghiCacheAside(cache, nguon, "userB", 999)`, gọi
`docCacheAside(cache, nguon, "userB")`. Giá trị trả VỀ đến TỪ đâu?

:::opt{correct}
Từ NGUỒN (`docTuNguon` được gọi) — `ghiCacheAside` đã XOÁ `userB`
khỏi cache, nên lần đọc kế tiếp chắc chắn MISS
:::
:::opt
Từ cache — cache-aside LUÔN ưu tiên trả VỀ giá trị trong cache nếu
khoá ĐÓ từng tồn tại Ở đó trước ĐÂY
::why
Nhầm "cache LÀ nguồn ưu tiên tuyệt đối" VỚI cơ chế THẬT của
cache-aside: cache chỉ hợp LỆ khi entry CÒN tồn tại VÀ chưa bị vô
hiệu hoá.

Chỗ lệch: `ghiCacheAside` gọi `cache.duLieu.delete(khoa)` NGAY sau
khi ghi nguồn — entry `userB` KHÔNG còn trong `cache.duLieu` nữa.
`docCacheAside` kiểm tra `cache.duLieu.has(khoa)` ĐẦU tiên, thấy
`false`, NÊN đi thẳng vào nhánh `docTuNguon` — giá trị trả về LUÔN
LÀ giá trị MỚI nhất vừa ghi, không phải một giá trị cache CŨ.
::
:::
::::

::::code{#viet_doc_cache_aside}
Hoàn thiện `docCacheAside` — SAU khi xác nhận cache miss, ghi giá
trị vừa đọc được TỪ nguồn (nếu KHÔNG phải `undefined`) VÀO cache
trước khi trả về.

```typescript title=starter
interface NguonCham { duLieu: Map<string, number>; soLanDuocGoi: number; }
function taoNguonCham(duLieuBanDau: [string, number][]): NguonCham {
  return { duLieu: new Map(duLieuBanDau), soLanDuocGoi: 0 };
}
function docTuNguon(nguon: NguonCham, khoa: string): number | undefined {
  nguon.soLanDuocGoi += 1;
  return nguon.duLieu.get(khoa);
}
function ghiVaoNguon(nguon: NguonCham, khoa: string, giaTri: number): void {
  nguon.duLieu.set(khoa, giaTri);
}

interface BoNhoDem { duLieu: Map<string, number>; }
function taoBoNhoDem(): BoNhoDem { return { duLieu: new Map() }; }

function docCacheAside(cache: BoNhoDem, nguon: NguonCham, khoa: string): number | undefined {
  if (cache.duLieu.has(khoa)) return cache.duLieu.get(khoa);
  const giaTri = docTuNguon(nguon, khoa);
  ___
  return giaTri;
}

function ghiCacheAside(cache: BoNhoDem, nguon: NguonCham, khoa: string, giaTri: number): void {
  ghiVaoNguon(nguon, khoa, giaTri);
  cache.duLieu.delete(khoa);
}

const nguon = taoNguonCham([["userA", 100]]);
const cache = taoBoNhoDem();
console.log(docCacheAside(cache, nguon, "userA"));
```

```typescript title=solution
interface NguonCham { duLieu: Map<string, number>; soLanDuocGoi: number; }
function taoNguonCham(duLieuBanDau: [string, number][]): NguonCham {
  return { duLieu: new Map(duLieuBanDau), soLanDuocGoi: 0 };
}
function docTuNguon(nguon: NguonCham, khoa: string): number | undefined {
  nguon.soLanDuocGoi += 1;
  return nguon.duLieu.get(khoa);
}
function ghiVaoNguon(nguon: NguonCham, khoa: string, giaTri: number): void {
  nguon.duLieu.set(khoa, giaTri);
}

interface BoNhoDem { duLieu: Map<string, number>; }
function taoBoNhoDem(): BoNhoDem { return { duLieu: new Map() }; }

function docCacheAside(cache: BoNhoDem, nguon: NguonCham, khoa: string): number | undefined {
  if (cache.duLieu.has(khoa)) return cache.duLieu.get(khoa);
  const giaTri = docTuNguon(nguon, khoa);
  if (giaTri !== undefined) cache.duLieu.set(khoa, giaTri);
  return giaTri;
}

function ghiCacheAside(cache: BoNhoDem, nguon: NguonCham, khoa: string, giaTri: number): void {
  ghiVaoNguon(nguon, khoa, giaTri);
  cache.duLieu.delete(khoa);
}

const nguon = taoNguonCham([["userA", 100]]);
const cache = taoBoNhoDem();
console.log(docCacheAside(cache, nguon, "userA"));
```

```typescript title=test
function laySoLanDuocGoi(n: NguonCham): number { return n.soLanDuocGoi; }

const nguonT = taoNguonCham([["userA", 100], ["userB", 200]]);
const cacheT = taoBoNhoDem();

if (docCacheAside(cacheT, nguonT, "userA") !== 100) throw new Error("doc lan dau (miss) phai tra ve gia tri that tu nguon");
if (laySoLanDuocGoi(nguonT) !== 1) throw new Error("doc lan dau phai goi nguon dung 1 lan");
if (docCacheAside(cacheT, nguonT, "userA") !== 100) throw new Error("doc lan hai phai van ra 100");
if (laySoLanDuocGoi(nguonT) !== 1) throw new Error("doc lan hai (HIT) khong duoc goi them nguon -- soLanDuocGoi phai giu nguyen 1");
if (!cacheT.duLieu.has("userA")) throw new Error("sau khi doc mot lan (miss), cache PHAI da co userA");

ghiCacheAside(cacheT, nguonT, "userB", 999);
if (cacheT.duLieu.has("userB")) throw new Error("sau ghi, cache KHONG duoc con giu userB (phai bi xoa/invalidate)");
if (docCacheAside(cacheT, nguonT, "userB") !== 999) throw new Error("doc userB sau ghi phai ra gia tri MOI (999)");
if (laySoLanDuocGoi(nguonT) !== 2) throw new Error("tong soLanDuocGoi phai la 2 (1 lan cho userA, 1 lan cho userB sau ghi)");

const nguonRong = taoNguonCham([]);
const cacheRong = taoBoNhoDem();
if (docCacheAside(cacheRong, nguonRong, "khong-ton-tai") !== undefined) throw new Error("khoa khong ton tai trong nguon phai tra ve undefined");
if (cacheRong.duLieu.has("khong-ton-tai")) throw new Error("khoa khong ton tai KHONG duoc ghi vao cache");
```

:::hints
- kind: attention
  body: "Neu giaTri khac undefined thi ghi vao cache.duLieu truoc khi tra ve -- mot dong."
- kind: strategy
  body: "if (giaTri !== undefined) cache.duLieu.set(khoa, giaTri);"
- kind: one-line
  body: "if (giaTri !== undefined) cache.duLieu.set(khoa, giaTri);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "100"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cache đã bớt tải cho nguồn. Nhưng cache VÔ hạn dung lượng chính LÀ
một cách xả tràn bộ nhớ khác — cache CŨNG cần giới hạn.
::::

::::reflect{#nghi-lai}
`docCacheAside` VÀ `ghiCacheAside` chỉ khác NHAU đúng một hướng
LUỒNG dữ liệu — đọc: cache TRƯỚC, nguồn SAU (nếu miss); ghi: nguồn
TRƯỚC, cache SAU (xoá, không cập nhật). Đảo NGƯỢC thứ tự Ở BẤT kỳ
đâu (cập nhật cache trước khi ghi nguồn xong, VÍ dụ) LÀ nguồn gốc
của rất NHIỀU bug "cache stale" trong hệ thống THẬT.
::::

::::checkpoint{mastery=0.8}
::::
