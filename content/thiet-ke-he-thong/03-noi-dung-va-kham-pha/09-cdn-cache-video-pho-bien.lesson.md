---
id: thiet-ke-he-thong.noi-dung-va-kham-pha.cdn-cache-video-pho-bien
title: "CDN cache video phổ biến, bỏ qua video ít xem"
summary: "phucVuVideo dung NGUONG_PHO_BIEN=100: video luotXem<100 LUON lay tu origin, khong bao gio vao CacheCDN (khai bao LRU DOC LAP, khong import tu T7.1) -- goi 3 lan lien tiep van 3 lan origin. Video luotXem>=100 dung cache-aside: lan dau miss (origin, roi tu cache), lan sau hit. Cache dung luong 2: video LRU (lau khong dung nhat) bi da khi day, y tuong giong LRU da hoc nhung khai bao lai tu dau."
locale: vi
track: thiet-ke-he-thong
module: noi-dung-va-kham-pha
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.cdn-cache-video-pho-bien]
requires: [sd.nhieu-do-phan-giai]
concepts: [sd.cdn-cache-video-pho-bien]
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
Video ĐÃ transcode xong, đủ độ phân giải để CLIENT chọn. Câu hỏi cuối:
phục vụ video TỪ đâu? Máy chủ gốc CÁCH xa hàng triệu người xem — CDN đặt
bản sao Ở gần họ hơn, NHƯNG không thể sao CHÉP mọi video ra khắp nơi.
::::

::::explain{#nguong-pho-bien}
Chỉ video PHỔ biến (đủ lượt xem) mới ĐÁNG được cache Ở CDN edge — video
ÍT người xem thì lấy TỪ nguồn gốc mỗi lần vẫn RẺ hơn tốn chỗ cache CHO
nó. `phucVuVideo` rẽ nhánh THEO `luotXem`: dưới ngưỡng LUÔN đi
`origin`; từ ngưỡng trở LÊN mới dùng cache-aside (bài `07-cache-ben-canh-doc-va-ghi`
Ở T7.1, ráp cùng LRU):

```typescript title=readonly
interface CacheCDN { dungLuongToiDa: number; duLieu: Map<string, string>; }
function taoCacheCDN(dungLuongToiDa: number): CacheCDN {
  return { dungLuongToiDa, duLieu: new Map() };
}
function truyCapCDN(cache: CacheCDN, idVideo: string): string | undefined {
  if (!cache.duLieu.has(idVideo)) return undefined;
  const noiDung = cache.duLieu.get(idVideo)!;
  cache.duLieu.delete(idVideo);
  cache.duLieu.set(idVideo, noiDung);
  return noiDung;
}
function themVaoCacheCDN(cache: CacheCDN, idVideo: string, noiDung: string): void {
  if (cache.duLieu.has(idVideo)) {
    cache.duLieu.delete(idVideo);
  } else if (cache.duLieu.size >= cache.dungLuongToiDa) {
    cache.duLieu.delete(cache.duLieu.keys().next().value as string);
  }
  cache.duLieu.set(idVideo, noiDung);
}

const NGUONG_PHO_BIEN = 100;

function layVideoOrigin(idVideo: string): string {
  return `noi-dung-cua-${idVideo}`;
}

interface KetQuaPhucVu { noiDung: string; nguon: "cache" | "origin"; }

function phucVuVideo(cache: CacheCDN, idVideo: string, luotXem: number): KetQuaPhucVu {
  if (luotXem < NGUONG_PHO_BIEN) {
    return { noiDung: layVideoOrigin(idVideo), nguon: "origin" };
  }
  const tuCache = truyCapCDN(cache, idVideo);
  if (tuCache !== undefined) return { noiDung: tuCache, nguon: "cache" };
  const noiDung = layVideoOrigin(idVideo);
  themVaoCacheCDN(cache, idVideo, noiDung);
  return { noiDung, nguon: "origin" };
}

const cache = taoCacheCDN(2);
console.log("video pho bien, lan dau (chua co trong cache):", JSON.stringify(phucVuVideo(cache, "v-hot", 500)));
console.log("video pho bien, lan hai (da nam san trong cache):", JSON.stringify(phucVuVideo(cache, "v-hot", 500)));
```

```text title=readonly
video pho bien, lan dau (chua co trong cache): {"noiDung":"noi-dung-cua-v-hot","nguon":"origin"}
video pho bien, lan hai (da nam san trong cache): {"noiDung":"noi-dung-cua-v-hot","nguon":"cache"}
```

`CacheCDN` dùng ĐÚNG ý tưởng LRU (Map giữ THỨ tự chèn, `truyCapCDN` đẩy
khoá vừa đọc VỀ cuối) — nhưng khai báo LẠI hoàn toàn tại đây, không hề
import TỪ bài LRU cũ. Lần đầu "v-hot" chưa CÓ trong cache NÊN `nguon`
LÀ `"origin"` (rồi tự cache LẠI); lần hai đã LÀ `"cache"`.
::::

::::example{#video-it-pho-bien-khong-bao-gio-vao-cache}
Video DƯỚI ngưỡng phổ biến hoàn toàn KHÔNG chạm tới nhánh cache — gọi
LẠI bao nhiêu lần cũng LUÔN LÀ `"origin"`, VÀ cache CDN chưa bao giờ giữ
một BẢN nào của nó:

```typescript title=readonly
interface CacheCDN { dungLuongToiDa: number; duLieu: Map<string, string>; }
function taoCacheCDN(dungLuongToiDa: number): CacheCDN {
  return { dungLuongToiDa, duLieu: new Map() };
}
function truyCapCDN(cache: CacheCDN, idVideo: string): string | undefined {
  if (!cache.duLieu.has(idVideo)) return undefined;
  const noiDung = cache.duLieu.get(idVideo)!;
  cache.duLieu.delete(idVideo);
  cache.duLieu.set(idVideo, noiDung);
  return noiDung;
}
function themVaoCacheCDN(cache: CacheCDN, idVideo: string, noiDung: string): void {
  if (cache.duLieu.has(idVideo)) {
    cache.duLieu.delete(idVideo);
  } else if (cache.duLieu.size >= cache.dungLuongToiDa) {
    cache.duLieu.delete(cache.duLieu.keys().next().value as string);
  }
  cache.duLieu.set(idVideo, noiDung);
}

const NGUONG_PHO_BIEN = 100;

function layVideoOrigin(idVideo: string): string {
  return `noi-dung-cua-${idVideo}`;
}

interface KetQuaPhucVu { noiDung: string; nguon: "cache" | "origin"; }

function phucVuVideo(cache: CacheCDN, idVideo: string, luotXem: number): KetQuaPhucVu {
  if (luotXem < NGUONG_PHO_BIEN) {
    return { noiDung: layVideoOrigin(idVideo), nguon: "origin" };
  }
  const tuCache = truyCapCDN(cache, idVideo);
  if (tuCache !== undefined) return { noiDung: tuCache, nguon: "cache" };
  const noiDung = layVideoOrigin(idVideo);
  themVaoCacheCDN(cache, idVideo, noiDung);
  return { noiDung, nguon: "origin" };
}

const cache = taoCacheCDN(2);
console.log("video it pho bien, lan 1:", JSON.stringify(phucVuVideo(cache, "v-nguoi", 10)));
console.log("video it pho bien, lan 2:", JSON.stringify(phucVuVideo(cache, "v-nguoi", 10)));
console.log("video it pho bien, lan 3:", JSON.stringify(phucVuVideo(cache, "v-nguoi", 10)));
console.log("cache co chua v-nguoi khong (phai luon la false):", cache.duLieu.has("v-nguoi"));
```

```text title=readonly
video it pho bien, lan 1: {"noiDung":"noi-dung-cua-v-nguoi","nguon":"origin"}
video it pho bien, lan 2: {"noiDung":"noi-dung-cua-v-nguoi","nguon":"origin"}
video it pho bien, lan 3: {"noiDung":"noi-dung-cua-v-nguoi","nguon":"origin"}
cache co chua v-nguoi khong (phai luon la false): false
```

Dù gọi `phucVuVideo` cho `"v-nguoi"` (chỉ `10` lượt xem, DƯỚI ngưỡng
`100`) TỚI `3` lần, kết quả LUÔN LÀ `"origin"` — nhánh `luotXem <
NGUONG_PHO_BIEN` return NGAY, không hề chạm tới `themVaoCacheCDN`.
::::

::::predict{#doan-video-duoi-nguong-khong-tu-cache commitOnce}
Video `"v2"` có `luotXem = 10` (dưới `NGUONG_PHO_BIEN = 100`). Gọi
`phucVuVideo` cho `"v2"` HAI lần liên tiếp. Lần gọi THỨ hai có phải LÀ
một cache HIT không (`nguon === "cache"`)?

:::opt{correct}
KHÔNG — cả hai lần đều LÀ `"origin"`, vì `"v2"` chưa BAO giờ được đưa
vào cache (dưới ngưỡng phổ biến, nhánh cache HOÀN toàn bị bỏ qua ngay
từ đầu hàm)
:::
:::opt
CÓ — lần gọi ĐẦU tiên tự động đưa MỌI video được yêu cầu vào cache
(giống cache-aside thông thường), nên lần gọi SAU sẽ là cache hit
::why
Nhầm cache-aside THÔNG thường (mọi lần miss đều tự cache LẠI) với cách
`phucVuVideo` THẬT sự hoạt động — nhưng hàm này có một BƯỚC rẽ nhánh
TRƯỚC cả khi chạm tới logic cache-aside.

Chỗ lệch: `if (luotXem < NGUONG_PHO_BIEN) { return ...; }` LÀ dòng ĐẦU
tiên trong `phucVuVideo` — với `luotXem = 10 < 100`, nhánh NÀY return
NGAY, hoàn toàn KHÔNG chạm tới `truyCapCDN` hay `themVaoCacheCDN`. Logic
cache-aside (đọc-cache-nếu-miss-thì-nạp) chỉ NẰM Ở nhánh CÒN lại, dành
riêng CHO video đủ phổ biến.
::
:::
::::

::::code{#viet_phuc_vu_video}
Hoàn thiện nhánh VIDEO phổ biến trong `phucVuVideo` — cache-aside: thử
đọc TỪ cache trước, nếu miss thì lấy TỪ origin rồi nạp VÀO cache.

```typescript title=starter
interface CacheCDN { dungLuongToiDa: number; duLieu: Map<string, string>; }
function taoCacheCDN(dungLuongToiDa: number): CacheCDN {
  return { dungLuongToiDa, duLieu: new Map() };
}
function truyCapCDN(cache: CacheCDN, idVideo: string): string | undefined {
  if (!cache.duLieu.has(idVideo)) return undefined;
  const noiDung = cache.duLieu.get(idVideo)!;
  cache.duLieu.delete(idVideo);
  cache.duLieu.set(idVideo, noiDung);
  return noiDung;
}
function themVaoCacheCDN(cache: CacheCDN, idVideo: string, noiDung: string): void {
  if (cache.duLieu.has(idVideo)) {
    cache.duLieu.delete(idVideo);
  } else if (cache.duLieu.size >= cache.dungLuongToiDa) {
    cache.duLieu.delete(cache.duLieu.keys().next().value as string);
  }
  cache.duLieu.set(idVideo, noiDung);
}

const NGUONG_PHO_BIEN = 100;

function layVideoOrigin(idVideo: string): string {
  return `noi-dung-cua-${idVideo}`;
}

interface KetQuaPhucVu { noiDung: string; nguon: "cache" | "origin"; }

function phucVuVideo(cache: CacheCDN, idVideo: string, luotXem: number): KetQuaPhucVu {
  if (luotXem < NGUONG_PHO_BIEN) {
    return { noiDung: layVideoOrigin(idVideo), nguon: "origin" };
  }
  ___
}

const cache = taoCacheCDN(2);
console.log(JSON.stringify(phucVuVideo(cache, "v1", 500)));
console.log(JSON.stringify(phucVuVideo(cache, "v1", 500)));
```

```typescript title=solution
interface CacheCDN { dungLuongToiDa: number; duLieu: Map<string, string>; }
function taoCacheCDN(dungLuongToiDa: number): CacheCDN {
  return { dungLuongToiDa, duLieu: new Map() };
}
function truyCapCDN(cache: CacheCDN, idVideo: string): string | undefined {
  if (!cache.duLieu.has(idVideo)) return undefined;
  const noiDung = cache.duLieu.get(idVideo)!;
  cache.duLieu.delete(idVideo);
  cache.duLieu.set(idVideo, noiDung);
  return noiDung;
}
function themVaoCacheCDN(cache: CacheCDN, idVideo: string, noiDung: string): void {
  if (cache.duLieu.has(idVideo)) {
    cache.duLieu.delete(idVideo);
  } else if (cache.duLieu.size >= cache.dungLuongToiDa) {
    cache.duLieu.delete(cache.duLieu.keys().next().value as string);
  }
  cache.duLieu.set(idVideo, noiDung);
}

const NGUONG_PHO_BIEN = 100;

function layVideoOrigin(idVideo: string): string {
  return `noi-dung-cua-${idVideo}`;
}

interface KetQuaPhucVu { noiDung: string; nguon: "cache" | "origin"; }

function phucVuVideo(cache: CacheCDN, idVideo: string, luotXem: number): KetQuaPhucVu {
  if (luotXem < NGUONG_PHO_BIEN) {
    return { noiDung: layVideoOrigin(idVideo), nguon: "origin" };
  }
  const tuCache = truyCapCDN(cache, idVideo);
  if (tuCache !== undefined) return { noiDung: tuCache, nguon: "cache" };
  const noiDung = layVideoOrigin(idVideo);
  themVaoCacheCDN(cache, idVideo, noiDung);
  return { noiDung, nguon: "origin" };
}

const cache = taoCacheCDN(2);
console.log(JSON.stringify(phucVuVideo(cache, "v1", 500)));
console.log(JSON.stringify(phucVuVideo(cache, "v1", 500)));
```

```typescript title=test
const cacheT = taoCacheCDN(2);
const kq1 = phucVuVideo(cacheT, "v-hot", 500);
if (kq1.nguon !== "origin") throw new Error("lan dau tien, video pho bien PHAI lay tu origin");
if (cacheT.duLieu.size !== 1) throw new Error("sau lan dau, video pho bien PHAI duoc dua vao cache");

const kq2 = phucVuVideo(cacheT, "v-hot", 500);
if (kq2.nguon !== "cache") throw new Error("lan hai, video pho bien PHAI la cache hit");
if (kq2.noiDung !== kq1.noiDung) throw new Error("noi dung tra ve tu cache phai giong noi dung goc");

const cache2 = taoCacheCDN(2);
phucVuVideo(cache2, "v-nguoi", 10);
phucVuVideo(cache2, "v-nguoi", 10);
const kq3 = phucVuVideo(cache2, "v-nguoi", 10);
if (kq3.nguon !== "origin") throw new Error("video duoi nguong PHAI luon la origin, ke ca lan goi thu 3");
if (cache2.duLieu.has("v-nguoi")) throw new Error("video duoi nguong KHONG duoc dua vao cache CDN");
if (cache2.duLieu.size !== 0) throw new Error("cache phai VAN rong sau nhieu lan phuc vu video duoi nguong");

const cache3 = taoCacheCDN(2);
const kqDungNguong = phucVuVideo(cache3, "v-dung-nguong", NGUONG_PHO_BIEN);
if (kqDungNguong.nguon !== "origin") throw new Error("lan dau van la origin (chua co trong cache)");
if (!cache3.duLieu.has("v-dung-nguong")) throw new Error("luotXem DUNG BANG nguong (100) PHAI duoc coi la pho bien va duoc cache");

const cache4 = taoCacheCDN(2);
phucVuVideo(cache4, "va", 200);
phucVuVideo(cache4, "vb", 200);
phucVuVideo(cache4, "va", 200);
phucVuVideo(cache4, "vc", 200);
if (cache4.duLieu.has("vb")) throw new Error("vb (lau khong dung nhat) phai bi day ra khoi cache CDN khi day");
if (!cache4.duLieu.has("va") || !cache4.duLieu.has("vc")) throw new Error("va va vc phai con trong cache sau khi day vb");
if (cache4.duLieu.size !== 2) throw new Error("kich thuoc cache khong duoc vuot qua dungLuongToiDa");
```

:::hints
- kind: attention
  body: "Nhanh video pho bien la cache-aside dung chuan: thu doc tu cache truoc (truyCapCDN); neu co (khac undefined) tra ve ngay voi nguon 'cache'; neu khong, lay tu origin roi NAP vao cache truoc khi tra ve."
- kind: strategy
  body: "const tuCache = truyCapCDN(cache, idVideo); if (tuCache !== undefined) return {noiDung: tuCache, nguon: 'cache'}; roi lay noiDung tu layVideoOrigin, goi themVaoCacheCDN, tra ve voi nguon 'origin'."
- kind: one-line
  body: "const tuCache = truyCapCDN(cache, idVideo); if (tuCache !== undefined) return { noiDung: tuCache, nguon: \"cache\" }; const noiDung = layVideoOrigin(idVideo); themVaoCacheCDN(cache, idVideo, noiDung); return { noiDung, nguon: \"origin\" };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "origin"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
News Feed, Search Autocomplete, YouTube — ba mảng đều xong. Giờ ráp CẢ
ba vào một luồng đăng nội dung DUY nhất.
::::

::::reflect{#nghi-lai}
`phucVuVideo` ráp HAI ý tưởng đã học Ở hai nơi khác NHAU: ngưỡng quyết
định (giống hybrid Ở bài 3 — CHỌN chiến lược theo một con số ĐO được) VÀ
cache-aside kèm LRU (T7.1). Khai báo LẠI `CacheCDN` từ đầu, thay VÌ
import, giữ bài học NÀY độc lập — nhưng Ý tưởng bên dưới THÌ y hệt.
::::

::::checkpoint{mastery=0.81}
::::
