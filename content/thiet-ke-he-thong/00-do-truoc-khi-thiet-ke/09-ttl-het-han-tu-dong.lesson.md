---
id: thiet-ke-he-thong.do-truoc-khi-thiet-ke.ttl-het-han-tu-dong
title: "TTL: hết hạn tự động"
summary: "DongHoMoPhong (đồng hồ ảo tự viết, không gọi Date.now()) giữ thoiGianHienTai, tienThoiGian cộng dồn. layTuCacheTTL so sánh dh.thoiGianHienTai >= muc.hetHanLuc (không phải >) -- đúng thời điểm bằng hetHanLuc coi là ĐÃ hết hạn, không phải 'còn đúng khoảnh khắc cuối'. Mục TTL=1000ms đặt tại t=0: t=999 còn đọc được (42, cache còn giữ); t=1000 (đúng biên) đã hết hạn (undefined, cache đã tự xoá entry)."
locale: vi
track: thiet-ke-he-thong
module: do-truoc-khi-thiet-ke
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.ttl-het-han-tu-dong]
requires: [sd.lru-da-ai-ra-khi-day]
concepts: [sd.ttl-het-han-tu-dong]
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
LRU (bài trước) đá dữ liệu khi cache ĐẦY. Nhưng một giá trị vẫn CÓ
thể sai dù cache CÒN chỗ thoải mái — dữ liệu NGUỒN đã đổi từ LÂU,
cache chỉ chưa hề "biết".
::::

::::explain{#dong-ho-mo-phong}
TTL (time-to-live) gắn MỖI mục cache MỘT hạn dùng — quá hạn thì tự
coi LÀ không còn giá trị, dù chưa BỊ đá bởi LRU. Đo thời gian CẦN
một cái đồng HỒ — nhưng `Date.now()` gắn chặt VỚI đồng hồ THẬT của
máy đang chạy, KHÔNG tái lập được (chạy lại SẼ ra giờ khác). Ta tự
viết một đồng hồ ẢO: chỉ một trường số, tiến LÊN khi được YÊU cầu,
không hề đụng TỚI hệ điều hành:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

const dh = taoDongHoMoPhong();
console.log("luc bat dau:", dh.thoiGianHienTai);
tienThoiGian(dh, 999);
console.log("sau khi tien 999ms:", dh.thoiGianHienTai);
```

```text title=readonly
luc bat dau: 0
sau khi tien 999ms: 999
```

`DongHoMoPhong` KHÔNG hề gọi `Date.now()` hay `performance.now()` —
"thời gian" Ở ĐÂY hoàn toàn LÀ một con số DO chương trình TỰ cộng
dồn. Mô phỏng `TTL` hàng giờ CHỈ cần gọi `tienThoiGian` đủ số LẦN,
chạy XONG trong tích tắc, KHÔNG cần chờ thời gian THẬT trôi qua.
::::

::::example{#het-han-dung-bien}
Mỗi mục cache lưu MỘT `hetHanLuc` (thời điểm ẢO hết hạn, tính BẰNG
`thoiGianHienTai` LÚC đặt cộng `ttlMs`). `layTuCacheTTL` kiểm tra
`thoiGianHienTai >= hetHanLuc` — dùng `>=`, KHÔNG phải `>`:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface MucCache { giaTri: number; hetHanLuc: number; }
interface CacheTTL { duLieu: Map<string, MucCache>; }
function taoCacheTTL(): CacheTTL { return { duLieu: new Map() }; }

function datVaoCacheTTL(cache: CacheTTL, dh: DongHoMoPhong, khoa: string, giaTri: number, ttlMs: number): void {
  cache.duLieu.set(khoa, { giaTri, hetHanLuc: dh.thoiGianHienTai + ttlMs });
}

function layTuCacheTTL(cache: CacheTTL, dh: DongHoMoPhong, khoa: string): number | undefined {
  const muc = cache.duLieu.get(khoa);
  if (muc === undefined) return undefined;
  if (dh.thoiGianHienTai >= muc.hetHanLuc) { cache.duLieu.delete(khoa); return undefined; }
  return muc.giaTri;
}

const dh2 = taoDongHoMoPhong();
const cache = taoCacheTTL();
datVaoCacheTTL(cache, dh2, "phien1", 42, 1000);
tienThoiGian(dh2, 999);

console.log("t=999, con han:", layTuCacheTTL(cache, dh2, "phien1"));
console.log("cache co con giu phien1 khong (t=999):", cache.duLieu.has("phien1"));

tienThoiGian(dh2, 1);
console.log("t=1000 (dung hetHanLuc), da het han:", String(layTuCacheTTL(cache, dh2, "phien1")));
console.log("cache co con giu phien1 khong (t=1000):", cache.duLieu.has("phien1"));
```

```text title=readonly
t=999, con han: 42
cache co con giu phien1 khong (t=999): true
t=1000 (dung hetHanLuc), da het han: undefined
cache co con giu phien1 khong (t=1000): false
```

Mục `"phien1"` đặt tại `t=0` VỚI `ttlMs=1000` → `hetHanLuc=1000`. Ở
`t=999` (còn CÁCH `hetHanLuc` đúng `1ms`), đọc VẪN ra `42`. NGAY khi
`t` chạm ĐÚNG `1000` (không CẦN vượt QUA), `layTuCacheTTL` coi LÀ đã
hết hạn — trả `undefined` VÀ TỰ xoá entry khỏi `cache.duLieu` (không
chỉ trả undefined MÀ còn dọn RÁC luôn).
::::

::::predict{#doan-dung-bien-het-han commitOnce}
Một mục có `hetHanLuc = 1000`. Tại ĐÚNG `thoiGianHienTai = 1000`
(không phải `999`, không phải `1001`) — mục ĐÓ còn đọc được không?

:::opt{correct}
KHÔNG — đã hết hạn; điều kiện dùng `>=`, NÊN `thoiGianHienTai` bằng
ĐÚNG `hetHanLuc` cũng bị coi LÀ hết hạn, không cần VƯỢT qua
:::
:::opt
CÓ — `1000` chưa VƯỢT quá `1000`, chỉ khi lớn hơn THẬT sự (`1001` trở
LÊN) mới tính LÀ hết hạn
::why
Nhầm "chạm đúng NGƯỠNG" VỚI "chưa vượt ngưỡng" — trực giác off-by-one
kinh điển.

Chỗ lệch: `layTuCacheTTL` viết `if (dh.thoiGianHienTai >= muc.hetHanLuc)`
— dùng `>=`, KHÔNG phải `>`. Tại `thoiGianHienTai === hetHanLuc`,
điều kiện NÀY đã đúng (`true`), NÊN mục bị coi LÀ hết hạn NGAY tại
đúng ranh giới, không CÓ "khoảnh khắc cuối cùng" nào còn dùng được.
::
:::
::::

::::code{#viet_lay_tu_cache_ttl}
Hoàn thiện `layTuCacheTTL` — điều kiện kiểm tra HẾT hạn (thời gian
hiện tại đã BẰNG hoặc vượt quá `hetHanLuc`).

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface MucCache { giaTri: number; hetHanLuc: number; }
interface CacheTTL { duLieu: Map<string, MucCache>; }
function taoCacheTTL(): CacheTTL { return { duLieu: new Map() }; }

function datVaoCacheTTL(cache: CacheTTL, dh: DongHoMoPhong, khoa: string, giaTri: number, ttlMs: number): void {
  cache.duLieu.set(khoa, { giaTri, hetHanLuc: dh.thoiGianHienTai + ttlMs });
}

function layTuCacheTTL(cache: CacheTTL, dh: DongHoMoPhong, khoa: string): number | undefined {
  const muc = cache.duLieu.get(khoa);
  if (muc === undefined) return undefined;
  if (___) { cache.duLieu.delete(khoa); return undefined; }
  return muc.giaTri;
}

const dh = taoDongHoMoPhong();
const cache = taoCacheTTL();
datVaoCacheTTL(cache, dh, "phien1", 42, 1000);
console.log(layTuCacheTTL(cache, dh, "phien1"));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface MucCache { giaTri: number; hetHanLuc: number; }
interface CacheTTL { duLieu: Map<string, MucCache>; }
function taoCacheTTL(): CacheTTL { return { duLieu: new Map() }; }

function datVaoCacheTTL(cache: CacheTTL, dh: DongHoMoPhong, khoa: string, giaTri: number, ttlMs: number): void {
  cache.duLieu.set(khoa, { giaTri, hetHanLuc: dh.thoiGianHienTai + ttlMs });
}

function layTuCacheTTL(cache: CacheTTL, dh: DongHoMoPhong, khoa: string): number | undefined {
  const muc = cache.duLieu.get(khoa);
  if (muc === undefined) return undefined;
  if (dh.thoiGianHienTai >= muc.hetHanLuc) { cache.duLieu.delete(khoa); return undefined; }
  return muc.giaTri;
}

const dh = taoDongHoMoPhong();
const cache = taoCacheTTL();
datVaoCacheTTL(cache, dh, "phien1", 42, 1000);
console.log(layTuCacheTTL(cache, dh, "phien1"));
```

```typescript title=test
function layKichThuocTTL(c: CacheTTL): number { return c.duLieu.size; }

const dhT = taoDongHoMoPhong();
const cacheT = taoCacheTTL();
datVaoCacheTTL(cacheT, dhT, "phien1", 42, 1000);

tienThoiGian(dhT, 999);
if (layTuCacheTTL(cacheT, dhT, "phien1") !== 42) throw new Error("t=999 (con han) phai doc duoc gia tri 42");
if (!cacheT.duLieu.has("phien1")) throw new Error("t=999, cache PHAI con giu phien1");

tienThoiGian(dhT, 1);
if (layTuCacheTTL(cacheT, dhT, "phien1") !== undefined) throw new Error("t=1000 (dung bang hetHanLuc) phai da het han, tra ve undefined");
if (cacheT.duLieu.has("phien1")) throw new Error("sau khi het han, cache PHAI tu xoa entry phien1");
if (layKichThuocTTL(cacheT) !== 0) throw new Error("cache phai rong sau khi entry duy nhat het han");

const dh2 = taoDongHoMoPhong();
const cache2 = taoCacheTTL();
datVaoCacheTTL(cache2, dh2, "phien2", 7, 500);
tienThoiGian(dh2, 500);
if (layTuCacheTTL(cache2, dh2, "phien2") !== undefined) throw new Error("tien dung bang ttl (500) cung phai duoc coi la het han");

if (layTuCacheTTL(cacheT, dhT, "khong-ton-tai") !== undefined) throw new Error("khoa khong ton tai phai tra ve undefined, khong duoc nem loi");
```

:::hints
- kind: attention
  body: "So sanh dh.thoiGianHienTai voi muc.hetHanLuc bang >= (khong phai >) -- mot dong."
- kind: strategy
  body: "dh.thoiGianHienTai >= muc.hetHanLuc"
- kind: one-line
  body: "if (dh.thoiGianHienTai >= muc.hetHanLuc) { cache.duLieu.delete(khoa); return undefined; }"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "42"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Load balancer, cache-aside, LRU, TTL — bốn mảnh ghép RIÊNG lẻ đã
xong. Giờ LÀ lúc ráp TẤT cả VÀO một hệ thống DUY nhất.
::::

::::reflect{#nghi-lai}
`layTuCacheTTL` chỉ khác một hàm đọc cache THƯỜNG đúng MỘT điều kiện
— nhưng chọn `>=` thay VÌ `>` LÀ quyết định có Ý nghĩa THẬT: nó định
nghĩa CHÍNH XÁC khoảnh khắc một hợp đồng "dữ liệu này VALID trong
đúng `ttlMs` mili-giây" kết thúc. `DongHoMoPhong` giúp KIỂM chứng
quyết định ĐÓ mà không cần chờ thời GIAN thật trôi qua.
::::

::::checkpoint{mastery=0.8}
::::
