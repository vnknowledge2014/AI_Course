---
id: thiet-ke-he-thong.do-truoc-khi-thiet-ke.boss-do-truoc-khi-thiet-ke
title: "BOSS — ráp một hệ thống nhỏ"
summary: "xuLyYeuCau ráp trọn bộ: ganYeuCau (bài 6, least-connections+health check) chọn server, layTuCache/datVaoCache (bài 8+9, LRU+TTL) phục vụ hoặc nạp qua docTuNguon (bài 7). Kịch bản 4 server (3 sống s1/s2/s3, 1 chết s4), cache dung lượng 3, TTL=1000ms, nguồn 5 khoá: 7 request đầu + 1 request sau khi đồng hồ tiến 1100ms (buộc hết hạn) -- 's4' KHÔNG XUẤT HIỆN trong 8 server đã dùng (s1,s2,s3,s1,s2,s3,s1,s2). soHit=3, soMiss=5, tỉ lệ hit=38%, soLanDuocGoi nguồn=5 (khớp đúng soMiss), phân bố kết nối cuối s1=3/s2=3/s3=2/s4=0."
locale: vi
track: thiet-ke-he-thong
module: do-truoc-khi-thiet-ke
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [sd.boss-do-truoc-khi-thiet-ke]
requires: [sd.ttl-het-han-tu-dong]
concepts: [sd.boss-do-truoc-khi-thiet-ke]
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
Chín bài — ước lượng (bài 1-4), cân bằng tải (bài 5-6), cache-aside +
LRU + TTL (bài 7-9). Giờ ráp CẢ chín mảnh VÀO một dịch vụ nhỏ DUY
nhất, VÀ đo xem nó THẬT sự hoạt động đúng không.
::::

::::explain{#rap_he_thong}
Một request đi qua ĐÚNG hai trạm: TRẠM một — `ganYeuCau` (bài 6) chọn
MỘT server còn sống, ít kết nối NHẤT. Trạm hai — cache (LRU+TTL, bài
8-9) phục vụ NẾU còn hạn VÀ còn trong bộ nhớ, HOẶC đọc từ nguồn chậm
(bài 7) rồi nạp LẠI cache nếu miss. `xuLyYeuCau` LÀ hàm ráp CẢ hai
trạm, VÀ đếm số lần HIT/MISS:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ServerInfo { ten: string; soKetNoi: number; conSong: boolean; }
function chonServerItKetNoiNhat(cacServer: ServerInfo[]): string | undefined {
  let ketQua: ServerInfo | undefined;
  for (const s of cacServer) {
    if (!s.conSong) continue;
    if (!ketQua || s.soKetNoi < ketQua.soKetNoi) ketQua = s;
  }
  return ketQua?.ten;
}
function ganYeuCau(cacServer: ServerInfo[]): string | undefined {
  const ten = chonServerItKetNoiNhat(cacServer);
  if (ten === undefined) return undefined;
  cacServer.find((s) => s.ten === ten)!.soKetNoi += 1;
  return ten;
}

interface MucCache { giaTri: number; hetHanLuc: number; }
interface HeThongCache { dungLuongToiDa: number; duLieu: Map<string, MucCache>; }
function taoHeThongCache(dungLuongToiDa: number): HeThongCache {
  return { dungLuongToiDa, duLieu: new Map() };
}
function layTuCache(cache: HeThongCache, dh: DongHoMoPhong, khoa: string): number | undefined {
  const muc = cache.duLieu.get(khoa);
  if (muc === undefined) return undefined;
  if (dh.thoiGianHienTai >= muc.hetHanLuc) { cache.duLieu.delete(khoa); return undefined; }
  cache.duLieu.delete(khoa);
  cache.duLieu.set(khoa, muc);
  return muc.giaTri;
}
function datVaoCache(cache: HeThongCache, dh: DongHoMoPhong, khoa: string, giaTri: number, ttlMs: number): void {
  if (cache.duLieu.has(khoa)) {
    cache.duLieu.delete(khoa);
  } else if (cache.duLieu.size >= cache.dungLuongToiDa) {
    cache.duLieu.delete(cache.duLieu.keys().next().value as string);
  }
  cache.duLieu.set(khoa, { giaTri, hetHanLuc: dh.thoiGianHienTai + ttlMs });
}

interface NguonCham { duLieu: Map<string, number>; soLanDuocGoi: number; }
function taoNguonCham(duLieuBanDau: [string, number][]): NguonCham {
  return { duLieu: new Map(duLieuBanDau), soLanDuocGoi: 0 };
}
function docTuNguon(nguon: NguonCham, khoa: string): number | undefined {
  nguon.soLanDuocGoi += 1;
  return nguon.duLieu.get(khoa);
}

interface ThongKeHeThong { soHit: number; soMiss: number; }

function xuLyYeuCau(cacServer: ServerInfo[], cache: HeThongCache, nguon: NguonCham, dh: DongHoMoPhong, khoa: string, ttlMs: number, thongKe: ThongKeHeThong): { server: string | undefined; giaTri: number | undefined } {
  const server = ganYeuCau(cacServer);
  if (server === undefined) return { server: undefined, giaTri: undefined };
  const trongCache = layTuCache(cache, dh, khoa);
  if (trongCache !== undefined) {
    thongKe.soHit++;
    return { server, giaTri: trongCache };
  }
  thongKe.soMiss++;
  const giaTriNguon = docTuNguon(nguon, khoa);
  if (giaTriNguon !== undefined) datVaoCache(cache, dh, khoa, giaTriNguon, ttlMs);
  return { server, giaTri: giaTriNguon };
}
```

`xuLyYeuCau` KHÔNG hề biết chi tiết BÊN trong load balancer HAY
cache — nó chỉ GỌI đúng thứ tự: chọn server TRƯỚC (nếu không CÓ
server nào sống, dừng NGAY), rồi tra cache, rồi RƠI xuống nguồn nếu
miss.
::::

::::example{#kich-ban_mo_phong}
Bốn server (`s1, s2, s3` sống; `s4` CHẾT), cache dung lượng `3`, TTL
`1000ms`, nguồn CÓ năm khoá (`k1..k5`). Bảy request ĐẦU (một VÀI khoá
lặp lại để tạo HIT), rồi tiến đồng HỒ `1100ms` VÀ gửi thêm một request
`k1` (buộc hết HẠN):

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ServerInfo { ten: string; soKetNoi: number; conSong: boolean; }
function chonServerItKetNoiNhat(cacServer: ServerInfo[]): string | undefined {
  let ketQua: ServerInfo | undefined;
  for (const s of cacServer) {
    if (!s.conSong) continue;
    if (!ketQua || s.soKetNoi < ketQua.soKetNoi) ketQua = s;
  }
  return ketQua?.ten;
}
function ganYeuCau(cacServer: ServerInfo[]): string | undefined {
  const ten = chonServerItKetNoiNhat(cacServer);
  if (ten === undefined) return undefined;
  cacServer.find((s) => s.ten === ten)!.soKetNoi += 1;
  return ten;
}

interface MucCache { giaTri: number; hetHanLuc: number; }
interface HeThongCache { dungLuongToiDa: number; duLieu: Map<string, MucCache>; }
function taoHeThongCache(dungLuongToiDa: number): HeThongCache {
  return { dungLuongToiDa, duLieu: new Map() };
}
function layTuCache(cache: HeThongCache, dh: DongHoMoPhong, khoa: string): number | undefined {
  const muc = cache.duLieu.get(khoa);
  if (muc === undefined) return undefined;
  if (dh.thoiGianHienTai >= muc.hetHanLuc) { cache.duLieu.delete(khoa); return undefined; }
  cache.duLieu.delete(khoa);
  cache.duLieu.set(khoa, muc);
  return muc.giaTri;
}
function datVaoCache(cache: HeThongCache, dh: DongHoMoPhong, khoa: string, giaTri: number, ttlMs: number): void {
  if (cache.duLieu.has(khoa)) {
    cache.duLieu.delete(khoa);
  } else if (cache.duLieu.size >= cache.dungLuongToiDa) {
    cache.duLieu.delete(cache.duLieu.keys().next().value as string);
  }
  cache.duLieu.set(khoa, { giaTri, hetHanLuc: dh.thoiGianHienTai + ttlMs });
}

interface NguonCham { duLieu: Map<string, number>; soLanDuocGoi: number; }
function taoNguonCham(duLieuBanDau: [string, number][]): NguonCham {
  return { duLieu: new Map(duLieuBanDau), soLanDuocGoi: 0 };
}
function docTuNguon(nguon: NguonCham, khoa: string): number | undefined {
  nguon.soLanDuocGoi += 1;
  return nguon.duLieu.get(khoa);
}

interface ThongKeHeThong { soHit: number; soMiss: number; }

function xuLyYeuCau(cacServer: ServerInfo[], cache: HeThongCache, nguon: NguonCham, dh: DongHoMoPhong, khoa: string, ttlMs: number, thongKe: ThongKeHeThong): { server: string | undefined; giaTri: number | undefined } {
  const server = ganYeuCau(cacServer);
  if (server === undefined) return { server: undefined, giaTri: undefined };
  const trongCache = layTuCache(cache, dh, khoa);
  if (trongCache !== undefined) {
    thongKe.soHit++;
    return { server, giaTri: trongCache };
  }
  thongKe.soMiss++;
  const giaTriNguon = docTuNguon(nguon, khoa);
  if (giaTriNguon !== undefined) datVaoCache(cache, dh, khoa, giaTriNguon, ttlMs);
  return { server, giaTri: giaTriNguon };
}

const cacServer: ServerInfo[] = [
  { ten: "s1", soKetNoi: 0, conSong: true },
  { ten: "s2", soKetNoi: 0, conSong: true },
  { ten: "s3", soKetNoi: 0, conSong: true },
  { ten: "s4", soKetNoi: 0, conSong: false },
];
const cache = taoHeThongCache(3);
const nguon = taoNguonCham([["k1", 10], ["k2", 20], ["k3", 30], ["k4", 40], ["k5", 50]]);
const dh = taoDongHoMoPhong();
const thongKe: ThongKeHeThong = { soHit: 0, soMiss: 0 };
const TTL = 1000;

const cacYeuCau = ["k1", "k2", "k1", "k3", "k4", "k1", "k3"];
const cacServerDaDung: (string | undefined)[] = [];
for (const khoa of cacYeuCau) {
  cacServerDaDung.push(xuLyYeuCau(cacServer, cache, nguon, dh, khoa, TTL, thongKe).server);
}
tienThoiGian(dh, 1100);
cacServerDaDung.push(xuLyYeuCau(cacServer, cache, nguon, dh, "k1", TTL, thongKe).server);

console.log("server da dung:", cacServerDaDung.join(","));
console.log("co roi vao server chet (s4) khong:", cacServerDaDung.includes("s4"));
console.log("soHit:", thongKe.soHit, "soMiss:", thongKe.soMiss);
console.log("soLanDuocGoi nguon:", nguon.soLanDuocGoi);
```

```text title=readonly
server da dung: s1,s2,s3,s1,s2,s3,s1,s2
co roi vao server chet (s4) khong: false
soHit: 3 soMiss: 5
soLanDuocGoi nguon: 5
```

Tám request, KHÔNG một request nào rơi VÀO `s4` — `chonServerItKetNoiNhat`
(bài 6) LOẠI nó khỏi MỌI lượt chọn TỪ đầu tới cuối, dù `s4` LUÔN có
`soKetNoi=0` (Ít nhất). `soHit=3` (VÀ `soMiss=5`) — VÀ `soLanDuocGoi`
CỦA nguồn khớp CHÍNH XÁC `5`, đúng bằng `soMiss`: mỗi miss gọi nguồn
ĐÚNG một lần, không thừa không thiếu. Tỉ lệ hit `3/8 = 37.5%`, làm
tròn `38%`.
::::

::::predict{#doan_server_chet_giua_chung commitOnce}
Giả sử `s2` bị đánh dấu `conSong = false` NGAY GIỮA mô phỏng (SAU vài
request ĐẦU), rồi các request CÒN lại tiếp tục gửi qua `xuLyYeuCau`.
Các request SAU thời điểm ĐÓ có thể vẫn rơi VÀO `s2` không?

:::opt{correct}
KHÔNG — MỖI lần `xuLyYeuCau` gọi LẠI `ganYeuCau` → `chonServerItKetNoiNhat`,
VÀ hàm này luôn đọc `conSong` HIỆN tại của từng server tại đúng THỜI
điểm gọi, không nhớ trạng thái CŨ
:::
:::opt
CÓ thể — MỘT khi `s2` đã được chọn ổn định ở các request TRƯỚC, hệ
thống có xu hướng tiếp tục "dính" VÀO nó
::why
Nhầm "được chọn nhiều LẦN trước đó" VỚI "có trạng thái ghi NHỚ ưu
tiên" — nhưng `chonServerItKetNoiNhat` hoàn toàn KHÔNG lưu lịch sử
lựa chọn.

Chỗ lệch: `chonServerItKetNoiNhat` duyệt LẠI TOÀN bộ `cacServer` VÀ
kiểm tra `s.conSong` TỪ ĐẦU, MỖI lần được gọi — không hề CÓ bộ nhớ
"đã từng chọn AI trước đây". Ngay khi `s2.conSong` chuyển THÀNH
`false`, LẦN gọi kế tiếp ĐÃ bỏ qua nó ngay LẬP tức, y hệt CÁCH `s4`
bị bỏ qua từ ĐẦU.
::
:::
::::

::::code{#viet_xu_ly_yeu_cau}
Hoàn thiện `xuLyYeuCau` — KHI cache HIT, phải đếm VÀO `thongKe.soHit`
trước khi trả VỀ kết quả.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ServerInfo { ten: string; soKetNoi: number; conSong: boolean; }
function chonServerItKetNoiNhat(cacServer: ServerInfo[]): string | undefined {
  let ketQua: ServerInfo | undefined;
  for (const s of cacServer) {
    if (!s.conSong) continue;
    if (!ketQua || s.soKetNoi < ketQua.soKetNoi) ketQua = s;
  }
  return ketQua?.ten;
}
function ganYeuCau(cacServer: ServerInfo[]): string | undefined {
  const ten = chonServerItKetNoiNhat(cacServer);
  if (ten === undefined) return undefined;
  cacServer.find((s) => s.ten === ten)!.soKetNoi += 1;
  return ten;
}

interface MucCache { giaTri: number; hetHanLuc: number; }
interface HeThongCache { dungLuongToiDa: number; duLieu: Map<string, MucCache>; }
function taoHeThongCache(dungLuongToiDa: number): HeThongCache {
  return { dungLuongToiDa, duLieu: new Map() };
}
function layTuCache(cache: HeThongCache, dh: DongHoMoPhong, khoa: string): number | undefined {
  const muc = cache.duLieu.get(khoa);
  if (muc === undefined) return undefined;
  if (dh.thoiGianHienTai >= muc.hetHanLuc) { cache.duLieu.delete(khoa); return undefined; }
  cache.duLieu.delete(khoa);
  cache.duLieu.set(khoa, muc);
  return muc.giaTri;
}
function datVaoCache(cache: HeThongCache, dh: DongHoMoPhong, khoa: string, giaTri: number, ttlMs: number): void {
  if (cache.duLieu.has(khoa)) {
    cache.duLieu.delete(khoa);
  } else if (cache.duLieu.size >= cache.dungLuongToiDa) {
    cache.duLieu.delete(cache.duLieu.keys().next().value as string);
  }
  cache.duLieu.set(khoa, { giaTri, hetHanLuc: dh.thoiGianHienTai + ttlMs });
}

interface NguonCham { duLieu: Map<string, number>; soLanDuocGoi: number; }
function taoNguonCham(duLieuBanDau: [string, number][]): NguonCham {
  return { duLieu: new Map(duLieuBanDau), soLanDuocGoi: 0 };
}
function docTuNguon(nguon: NguonCham, khoa: string): number | undefined {
  nguon.soLanDuocGoi += 1;
  return nguon.duLieu.get(khoa);
}

interface ThongKeHeThong { soHit: number; soMiss: number; }

function xuLyYeuCau(cacServer: ServerInfo[], cache: HeThongCache, nguon: NguonCham, dh: DongHoMoPhong, khoa: string, ttlMs: number, thongKe: ThongKeHeThong): { server: string | undefined; giaTri: number | undefined } {
  const server = ganYeuCau(cacServer);
  if (server === undefined) return { server: undefined, giaTri: undefined };
  const trongCache = layTuCache(cache, dh, khoa);
  if (trongCache !== undefined) {
    ___
    return { server, giaTri: trongCache };
  }
  thongKe.soMiss++;
  const giaTriNguon = docTuNguon(nguon, khoa);
  if (giaTriNguon !== undefined) datVaoCache(cache, dh, khoa, giaTriNguon, ttlMs);
  return { server, giaTri: giaTriNguon };
}

const cacServer: ServerInfo[] = [{ ten: "s1", soKetNoi: 0, conSong: true }];
const cache = taoHeThongCache(3);
const nguon = taoNguonCham([["k1", 10]]);
const dh = taoDongHoMoPhong();
const thongKe: ThongKeHeThong = { soHit: 0, soMiss: 0 };
xuLyYeuCau(cacServer, cache, nguon, dh, "k1", 1000, thongKe);
xuLyYeuCau(cacServer, cache, nguon, dh, "k1", 1000, thongKe);
console.log(thongKe.soHit);
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ServerInfo { ten: string; soKetNoi: number; conSong: boolean; }
function chonServerItKetNoiNhat(cacServer: ServerInfo[]): string | undefined {
  let ketQua: ServerInfo | undefined;
  for (const s of cacServer) {
    if (!s.conSong) continue;
    if (!ketQua || s.soKetNoi < ketQua.soKetNoi) ketQua = s;
  }
  return ketQua?.ten;
}
function ganYeuCau(cacServer: ServerInfo[]): string | undefined {
  const ten = chonServerItKetNoiNhat(cacServer);
  if (ten === undefined) return undefined;
  cacServer.find((s) => s.ten === ten)!.soKetNoi += 1;
  return ten;
}

interface MucCache { giaTri: number; hetHanLuc: number; }
interface HeThongCache { dungLuongToiDa: number; duLieu: Map<string, MucCache>; }
function taoHeThongCache(dungLuongToiDa: number): HeThongCache {
  return { dungLuongToiDa, duLieu: new Map() };
}
function layTuCache(cache: HeThongCache, dh: DongHoMoPhong, khoa: string): number | undefined {
  const muc = cache.duLieu.get(khoa);
  if (muc === undefined) return undefined;
  if (dh.thoiGianHienTai >= muc.hetHanLuc) { cache.duLieu.delete(khoa); return undefined; }
  cache.duLieu.delete(khoa);
  cache.duLieu.set(khoa, muc);
  return muc.giaTri;
}
function datVaoCache(cache: HeThongCache, dh: DongHoMoPhong, khoa: string, giaTri: number, ttlMs: number): void {
  if (cache.duLieu.has(khoa)) {
    cache.duLieu.delete(khoa);
  } else if (cache.duLieu.size >= cache.dungLuongToiDa) {
    cache.duLieu.delete(cache.duLieu.keys().next().value as string);
  }
  cache.duLieu.set(khoa, { giaTri, hetHanLuc: dh.thoiGianHienTai + ttlMs });
}

interface NguonCham { duLieu: Map<string, number>; soLanDuocGoi: number; }
function taoNguonCham(duLieuBanDau: [string, number][]): NguonCham {
  return { duLieu: new Map(duLieuBanDau), soLanDuocGoi: 0 };
}
function docTuNguon(nguon: NguonCham, khoa: string): number | undefined {
  nguon.soLanDuocGoi += 1;
  return nguon.duLieu.get(khoa);
}

interface ThongKeHeThong { soHit: number; soMiss: number; }

function xuLyYeuCau(cacServer: ServerInfo[], cache: HeThongCache, nguon: NguonCham, dh: DongHoMoPhong, khoa: string, ttlMs: number, thongKe: ThongKeHeThong): { server: string | undefined; giaTri: number | undefined } {
  const server = ganYeuCau(cacServer);
  if (server === undefined) return { server: undefined, giaTri: undefined };
  const trongCache = layTuCache(cache, dh, khoa);
  if (trongCache !== undefined) {
    thongKe.soHit++;
    return { server, giaTri: trongCache };
  }
  thongKe.soMiss++;
  const giaTriNguon = docTuNguon(nguon, khoa);
  if (giaTriNguon !== undefined) datVaoCache(cache, dh, khoa, giaTriNguon, ttlMs);
  return { server, giaTri: giaTriNguon };
}

const cacServer: ServerInfo[] = [{ ten: "s1", soKetNoi: 0, conSong: true }];
const cache = taoHeThongCache(3);
const nguon = taoNguonCham([["k1", 10]]);
const dh = taoDongHoMoPhong();
const thongKe: ThongKeHeThong = { soHit: 0, soMiss: 0 };
xuLyYeuCau(cacServer, cache, nguon, dh, "k1", 1000, thongKe);
xuLyYeuCau(cacServer, cache, nguon, dh, "k1", 1000, thongKe);
console.log(thongKe.soHit);
```

```typescript title=test
function laySoHit(t: ThongKeHeThong): number { return t.soHit; }
function laySoMiss(t: ThongKeHeThong): number { return t.soMiss; }
function laySoLanDuocGoiBoss(n: NguonCham): number { return n.soLanDuocGoi; }
function laySoKetNoiBoss(s: ServerInfo): number { return s.soKetNoi; }

const cacServerT: ServerInfo[] = [
  { ten: "s1", soKetNoi: 0, conSong: true },
  { ten: "s2", soKetNoi: 0, conSong: true },
  { ten: "s3", soKetNoi: 0, conSong: true },
  { ten: "s4", soKetNoi: 0, conSong: false },
];
const cacheT = taoHeThongCache(3);
const nguonT = taoNguonCham([["k1", 10], ["k2", 20], ["k3", 30], ["k4", 40], ["k5", 50]]);
const dhT = taoDongHoMoPhong();
const thongKeT: ThongKeHeThong = { soHit: 0, soMiss: 0 };
const TTLT = 1000;

const cacYeuCauT = ["k1", "k2", "k1", "k3", "k4", "k1", "k3"];
const cacServerDaDungT: (string | undefined)[] = [];
for (const khoa of cacYeuCauT) {
  cacServerDaDungT.push(xuLyYeuCau(cacServerT, cacheT, nguonT, dhT, khoa, TTLT, thongKeT).server);
}
tienThoiGian(dhT, 1100);
cacServerDaDungT.push(xuLyYeuCau(cacServerT, cacheT, nguonT, dhT, "k1", TTLT, thongKeT).server);

if (cacServerDaDungT.join(",") !== "s1,s2,s3,s1,s2,s3,s1,s2") throw new Error("chuoi server da dung phai dung y het s1,s2,s3,s1,s2,s3,s1,s2");
if (cacServerDaDungT.includes("s4")) throw new Error("s4 (server chet) khong duoc xuat hien trong bat ky request nao");
if (laySoHit(thongKeT) !== 3) throw new Error("soHit phai la 3");
if (laySoMiss(thongKeT) !== 5) throw new Error("soMiss phai la 5");
if (laySoLanDuocGoiBoss(nguonT) !== 5) throw new Error("soLanDuocGoi nguon phai khop dung soMiss (5)");

const s1T = cacServerT.find((s) => s.ten === "s1")!;
const s2T = cacServerT.find((s) => s.ten === "s2")!;
const s3T = cacServerT.find((s) => s.ten === "s3")!;
const s4T = cacServerT.find((s) => s.ten === "s4")!;
if (laySoKetNoiBoss(s1T) !== 3) throw new Error("s1.soKetNoi cuoi phai la 3");
if (laySoKetNoiBoss(s2T) !== 3) throw new Error("s2.soKetNoi cuoi phai la 3");
if (laySoKetNoiBoss(s3T) !== 2) throw new Error("s3.soKetNoi cuoi phai la 2");
if (laySoKetNoiBoss(s4T) !== 0) throw new Error("s4 (chet) khong duoc nhan request nao, soKetNoi phai la 0");
```

:::hints
- kind: attention
  body: "Khi trongCache khac undefined (HIT), tang thongKe.soHit truoc khi tra ve -- mot dong."
- kind: strategy
  body: "thongKe.soHit++;"
- kind: one-line
  body: "thongKe.soHit++;"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Server sống được chọn đúng, cache-aside/LRU/TTL phối hợp trơn tru,
không request nào lạc vào server chết. Realm 7 đã bắt đầu — tiếp
theo LÀ định danh phân tán VÀ giới hạn tốc độ.
::::

::::reflect{#nghi-lai}
`xuLyYeuCau` không PHÁT minh gì mới — nó chỉ GỌI đúng thứ tự bốn khối
đã xây RIÊNG lẻ (`ganYeuCau`, `layTuCache`, `docTuNguon`,
`datVaoCache`). Đây chính LÀ bài học lớn nhất CỦA "thiết kế hệ
thống": phần khó thường KHÔNG nằm Ở từng thành phần (mỗi cái đã ĐỦ
đơn giản để viết trong MỘT bài `7-15` phút), mà Ở việc GHÉP chúng
ĐÚNG thứ tự, đúng ranh giới trách nhiệm.
::::

::::checkpoint{mastery=0.85}
::::
