---
id: thiet-ke-he-thong.url-rut-gon-va-thu-thap-web.boss-url-rut-gon-va-thu-thap-web
title: "BOSS — Crawler đi khám phá, rút gọn cấp mã"
summary: "chayCrawlVaRutGon(urlGoc, doSauToiDa) ráp BFS (hàng đợi FIFO, đơn giản hoá từ bài 6-8: bỏ giới hạn domain và chuẩn hoá URL, giữ độ sâu + dedup bằng Set) VỚI bộ rút gọn (base62 + counter, bài 1-3) — MỖI URL vừa được lấy ra khỏi hàng đợi lập tức nhận một mã ngắn (thứ tự BFS = thứ tự cấp mã), rồi mới xét khám phá con của nó NẾU còn trong giới hạn độ sâu. Trên cây 7 trang (goc→a,b→c,d,e→f theo tầng), doSauToiDa=2 crawl đúng 6 trang (goc:0,a:1,b:2,c:3,d:4,e:5) — trang-f (tầng 3) hoàn toàn KHÔNG xuất hiện trong Map kết quả; doSauToiDa=3 mới bao gồm đủ 7 trang."
locale: vi
track: thiet-ke-he-thong
module: url-rut-gon-va-thu-thap-web
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [sd.boss-url-rut-gon-va-thu-thap-web]
requires: [sd.nhan-nha-theo-domain]
concepts: [sd.boss-url-rut-gon-va-thu-thap-web]
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
Chín bài — mã hoá VÀ giải mã base62, bộ đếm, alias, hết hạn (nửa
RÚT gọn URL); hàng đợi BFS, chuẩn hoá, chống bẫy, nhã nhặn theo
domain (nửa CRAWLER). Giờ ráp CẢ hai nửa VÀO một hệ thống DUY nhất.
::::

::::explain{#rap_crawler_va_rut_gon}
Một crawler ĐƠN giản hoá (BFS + giới hạn độ sâu + khử trùng bằng
`Set`, gộp lại TỪ bài 6-8) khám phá URL theo TỪNG tầng. MỖI URL vừa
được LẤY ra khỏi hàng đợi — TRƯỚC khi xét có khám phá tiếp hay không
— lập tức được đưa QUA bộ rút gọn (base62 + bộ đếm, bài 1-3) để nhận
một mã NGẮN. Kết quả LÀ một `Map` từ URL gốc TỚI mã ngắn tương ứng:

```typescript title=readonly
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
function maHoaBase62(n: number): string {
  if (n === 0) return "0";
  let ketQua = "";
  let con = n;
  while (con > 0) {
    ketQua = BANG_BASE62[con % 62]! + ketQua;
    con = Math.floor(con / 62);
  }
  return ketQua;
}

interface BoDemToanCuc { giaTriHienTai: number; }
function taoBoDemToanCuc(): BoDemToanCuc { return { giaTriHienTai: 0 }; }
function laMaNganTiepTheo(bo: BoDemToanCuc): string {
  const ma = maHoaBase62(bo.giaTriHienTai);
  bo.giaTriHienTai += 1;
  return ma;
}

interface HangDoiBFS { danhSach: { url: string; doSau: number }[]; }
function taoHangDoiBFS(): HangDoiBFS { return { danhSach: [] }; }
function themVaoHangDoi(hd: HangDoiBFS, url: string, doSau: number): void { hd.danhSach.push({ url, doSau }); }
function layTuHangDoi(hd: HangDoiBFS): { url: string; doSau: number } | undefined { return hd.danhSach.shift(); }

function khamPha(url: string): string[] {
  const DO_THI: Record<string, string[]> = {
    "trang-goc": ["trang-a", "trang-b"],
    "trang-a": ["trang-c", "trang-d"],
    "trang-b": ["trang-e"],
    "trang-c": ["trang-f"],
    "trang-d": [],
    "trang-e": [],
    "trang-f": [],
  };
  return DO_THI[url] ?? [];
}

function chayCrawlVaRutGon(urlGoc: string, doSauToiDa: number): Map<string, string> {
  const hd = taoHangDoiBFS();
  const boDem = taoBoDemToanCuc();
  const ketQua = new Map<string, string>();
  const daTham = new Set<string>();

  themVaoHangDoi(hd, urlGoc, 0);
  daTham.add(urlGoc);

  let hienTai = layTuHangDoi(hd);
  while (hienTai !== undefined) {
    ketQua.set(hienTai.url, laMaNganTiepTheo(boDem));
    if (hienTai.doSau < doSauToiDa) {
      for (const con of khamPha(hienTai.url)) {
        if (!daTham.has(con)) {
          daTham.add(con);
          themVaoHangDoi(hd, con, hienTai.doSau + 1);
        }
      }
    }
    hienTai = layTuHangDoi(hd);
  }
  return ketQua;
}

const ketQua = chayCrawlVaRutGon("trang-goc", 2);
console.log("so URL da crawl VA rut gon:", ketQua.size);
for (const [url, maNgan] of ketQua) console.log(`  ${url} -> ${maNgan}`);
```

```text title=readonly
so URL da crawl VA rut gon: 6
  trang-goc -> 0
  trang-a -> 1
  trang-b -> 2
  trang-c -> 3
  trang-d -> 4
  trang-e -> 5
```

`chayCrawlVaRutGon` KHÔNG hề biết chi tiết bên TRONG hàng đợi HAY bộ
đếm — nó chỉ gọi ĐÚNG thứ tự: lấy một URL RA, cấp mã NGAY, rồi mới
xét khám phá TIẾP. Mã ngắn được cấp đúng theo thứ tự KHÁM phá BFS:
`trang-goc` (mã `"0"`, khám phá ĐẦU tiên) tới `trang-e` (mã `"5"`,
khám phá SAU cùng trong giới hạn độ sâu `2`).
::::

::::example{#do-sau-quyet-dinh-pham-vi}
`trang-f` LÀ con của `trang-c`, nằm Ở độ sâu `3`. Tăng dần
`doSauToiDa` từ `0` TỚI `3` cho thấy RÕ: phạm vi crawl (VÀ vì thế,
số URL được rút GỌN) hoàn toàn do giới hạn độ sâu quyết ĐỊNH:

```typescript title=readonly
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
function maHoaBase62(n: number): string {
  if (n === 0) return "0";
  let ketQua = "";
  let con = n;
  while (con > 0) {
    ketQua = BANG_BASE62[con % 62]! + ketQua;
    con = Math.floor(con / 62);
  }
  return ketQua;
}

interface BoDemToanCuc { giaTriHienTai: number; }
function taoBoDemToanCuc(): BoDemToanCuc { return { giaTriHienTai: 0 }; }
function laMaNganTiepTheo(bo: BoDemToanCuc): string {
  const ma = maHoaBase62(bo.giaTriHienTai);
  bo.giaTriHienTai += 1;
  return ma;
}

interface HangDoiBFS { danhSach: { url: string; doSau: number }[]; }
function taoHangDoiBFS(): HangDoiBFS { return { danhSach: [] }; }
function themVaoHangDoi(hd: HangDoiBFS, url: string, doSau: number): void { hd.danhSach.push({ url, doSau }); }
function layTuHangDoi(hd: HangDoiBFS): { url: string; doSau: number } | undefined { return hd.danhSach.shift(); }

function khamPha(url: string): string[] {
  const DO_THI: Record<string, string[]> = {
    "trang-goc": ["trang-a", "trang-b"],
    "trang-a": ["trang-c", "trang-d"],
    "trang-b": ["trang-e"],
    "trang-c": ["trang-f"],
    "trang-d": [],
    "trang-e": [],
    "trang-f": [],
  };
  return DO_THI[url] ?? [];
}

function chayCrawlVaRutGon(urlGoc: string, doSauToiDa: number): Map<string, string> {
  const hd = taoHangDoiBFS();
  const boDem = taoBoDemToanCuc();
  const ketQua = new Map<string, string>();
  const daTham = new Set<string>();

  themVaoHangDoi(hd, urlGoc, 0);
  daTham.add(urlGoc);

  let hienTai = layTuHangDoi(hd);
  while (hienTai !== undefined) {
    ketQua.set(hienTai.url, laMaNganTiepTheo(boDem));
    if (hienTai.doSau < doSauToiDa) {
      for (const con of khamPha(hienTai.url)) {
        if (!daTham.has(con)) {
          daTham.add(con);
          themVaoHangDoi(hd, con, hienTai.doSau + 1);
        }
      }
    }
    hienTai = layTuHangDoi(hd);
  }
  return ketQua;
}

// tang doSauToiDa tu 0 den 3: cang sau, cang nhieu URL duoc crawl VA rut gon
for (const doSau of [0, 1, 2, 3]) {
  const kq = chayCrawlVaRutGon("trang-goc", doSau);
  console.log(`doSauToiDa=${doSau}: ${kq.size} URL -- [${[...kq.keys()].join(",")}]`);
}

console.log('co "trang-f" trong ket qua khi doSauToiDa=2:', chayCrawlVaRutGon("trang-goc", 2).has("trang-f"));
console.log('co "trang-f" trong ket qua khi doSauToiDa=3:', chayCrawlVaRutGon("trang-goc", 3).has("trang-f"));
```

```text title=readonly
doSauToiDa=0: 1 URL -- [trang-goc]
doSauToiDa=1: 3 URL -- [trang-goc,trang-a,trang-b]
doSauToiDa=2: 6 URL -- [trang-goc,trang-a,trang-b,trang-c,trang-d,trang-e]
doSauToiDa=3: 7 URL -- [trang-goc,trang-a,trang-b,trang-c,trang-d,trang-e,trang-f]
co "trang-f" trong ket qua khi doSauToiDa=2: false
co "trang-f" trong ket qua khi doSauToiDa=3: true
```

`doSauToiDa=0` chỉ CRAWL đúng gốc (`1` URL). Mỗi lần TĂNG giới hạn
thêm `1`, thêm ĐÚNG một tầng của cây được ĐƯA vào — VÀ `trang-f`
(tầng `3`) chỉ xuất hiện KHI `doSauToiDa` đạt tới `3`, không SỚM hơn.
::::

::::predict{#doan_trang_f_vang_mat commitOnce}
Crawl với `doSauToiDa=2`, bắt đầu TỪ `"trang-goc"`. `"trang-f"` LÀ
con của `"trang-c"` (độ sâu `3`). `Map` kết QUẢ trả về CÓ chứa khoá
`"trang-f"` không?

:::opt{correct}
KHÔNG — `"trang-c"` Ở đúng độ sâu `2` (bằng `doSauToiDa`), điều kiện
`hienTai.doSau < doSauToiDa` sai (`2 < 2` LÀ `false`) nên nhánh khám
phá con của `"trang-c"` không CHẠY, `"trang-f"` không bao giờ được
đưa vào hàng đợi
:::
:::opt
CÓ, nhưng ánh xạ tới `undefined` — vì nó đã vượt QUÁ độ sâu cho phép,
hệ thống vẫn "ghi nhận" sự tồn tại của nó nhưng không cấp mã
::why
Nhầm "một khoá tồn tại trong `Map` VỚI giá trị `undefined`" VỚI "một
khoá hoàn toàn KHÔNG tồn tại trong `Map`" — nhưng `ketQua.set` chỉ
được GỌI cho những URL thực sự BỊ lấy ra khỏi hàng đợi.

Chỗ lệch: `"trang-f"` chỉ được ĐƯA vào hàng đợi Ở nhánh `for (const
con of khamPha(hienTai.url))`, mà nhánh đó nằm TRONG điều kiện
`hienTai.doSau < doSauToiDa`. Với `hienTai.url === "trang-c"` VÀ
`hienTai.doSau === 2 === doSauToiDa`, điều kiện SAI, nên
`themVaoHangDoi` không hề được gọi cho `"trang-f"` — nó không BAO
giờ được `layTuHangDoi` lấy ra, VÀ `ketQua.set("trang-f", ...)`
không bao giờ CHẠY. `Map.get("trang-f")` trả về `undefined` (khoá
không tồn tại), khác hẳn "khoá tồn tại VỚI giá trị `undefined`" —
kiểm bằng `.has()` sẽ cho `false`.
::
:::
::::

::::code{#viet_chay_crawl_va_rut_gon}
Hoàn thiện `chayCrawlVaRutGon` — ngay khi lấy một URL RA khỏi hàng
đợi, cấp cho nó một mã NGẮN mới (dùng bộ đếm) VÀ lưu vào kết quả.

```typescript title=starter
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
function maHoaBase62(n: number): string {
  if (n === 0) return "0";
  let ketQua = "";
  let con = n;
  while (con > 0) {
    ketQua = BANG_BASE62[con % 62]! + ketQua;
    con = Math.floor(con / 62);
  }
  return ketQua;
}

interface BoDemToanCuc { giaTriHienTai: number; }
function taoBoDemToanCuc(): BoDemToanCuc { return { giaTriHienTai: 0 }; }
function laMaNganTiepTheo(bo: BoDemToanCuc): string {
  const ma = maHoaBase62(bo.giaTriHienTai);
  bo.giaTriHienTai += 1;
  return ma;
}

interface HangDoiBFS { danhSach: { url: string; doSau: number }[]; }
function taoHangDoiBFS(): HangDoiBFS { return { danhSach: [] }; }
function themVaoHangDoi(hd: HangDoiBFS, url: string, doSau: number): void { hd.danhSach.push({ url, doSau }); }
function layTuHangDoi(hd: HangDoiBFS): { url: string; doSau: number } | undefined { return hd.danhSach.shift(); }

function khamPha(url: string): string[] {
  const DO_THI: Record<string, string[]> = {
    "trang-goc": ["trang-a", "trang-b"],
    "trang-a": ["trang-c", "trang-d"],
    "trang-b": ["trang-e"],
    "trang-c": ["trang-f"],
    "trang-d": [],
    "trang-e": [],
    "trang-f": [],
  };
  return DO_THI[url] ?? [];
}

function chayCrawlVaRutGon(urlGoc: string, doSauToiDa: number): Map<string, string> {
  const hd = taoHangDoiBFS();
  const boDem = taoBoDemToanCuc();
  const ketQua = new Map<string, string>();
  const daTham = new Set<string>();

  themVaoHangDoi(hd, urlGoc, 0);
  daTham.add(urlGoc);

  let hienTai = layTuHangDoi(hd);
  while (hienTai !== undefined) {
    ketQua.set(hienTai.url, ___);
    if (hienTai.doSau < doSauToiDa) {
      for (const con of khamPha(hienTai.url)) {
        if (!daTham.has(con)) {
          daTham.add(con);
          themVaoHangDoi(hd, con, hienTai.doSau + 1);
        }
      }
    }
    hienTai = layTuHangDoi(hd);
  }
  return ketQua;
}

const ketQua = chayCrawlVaRutGon("trang-goc", 1);
console.log(ketQua.size, [...ketQua.entries()]);
```

```typescript title=solution
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
function maHoaBase62(n: number): string {
  if (n === 0) return "0";
  let ketQua = "";
  let con = n;
  while (con > 0) {
    ketQua = BANG_BASE62[con % 62]! + ketQua;
    con = Math.floor(con / 62);
  }
  return ketQua;
}

interface BoDemToanCuc { giaTriHienTai: number; }
function taoBoDemToanCuc(): BoDemToanCuc { return { giaTriHienTai: 0 }; }
function laMaNganTiepTheo(bo: BoDemToanCuc): string {
  const ma = maHoaBase62(bo.giaTriHienTai);
  bo.giaTriHienTai += 1;
  return ma;
}

interface HangDoiBFS { danhSach: { url: string; doSau: number }[]; }
function taoHangDoiBFS(): HangDoiBFS { return { danhSach: [] }; }
function themVaoHangDoi(hd: HangDoiBFS, url: string, doSau: number): void { hd.danhSach.push({ url, doSau }); }
function layTuHangDoi(hd: HangDoiBFS): { url: string; doSau: number } | undefined { return hd.danhSach.shift(); }

function khamPha(url: string): string[] {
  const DO_THI: Record<string, string[]> = {
    "trang-goc": ["trang-a", "trang-b"],
    "trang-a": ["trang-c", "trang-d"],
    "trang-b": ["trang-e"],
    "trang-c": ["trang-f"],
    "trang-d": [],
    "trang-e": [],
    "trang-f": [],
  };
  return DO_THI[url] ?? [];
}

function chayCrawlVaRutGon(urlGoc: string, doSauToiDa: number): Map<string, string> {
  const hd = taoHangDoiBFS();
  const boDem = taoBoDemToanCuc();
  const ketQua = new Map<string, string>();
  const daTham = new Set<string>();

  themVaoHangDoi(hd, urlGoc, 0);
  daTham.add(urlGoc);

  let hienTai = layTuHangDoi(hd);
  while (hienTai !== undefined) {
    ketQua.set(hienTai.url, laMaNganTiepTheo(boDem));
    if (hienTai.doSau < doSauToiDa) {
      for (const con of khamPha(hienTai.url)) {
        if (!daTham.has(con)) {
          daTham.add(con);
          themVaoHangDoi(hd, con, hienTai.doSau + 1);
        }
      }
    }
    hienTai = layTuHangDoi(hd);
  }
  return ketQua;
}

const ketQua = chayCrawlVaRutGon("trang-goc", 1);
console.log(ketQua.size, [...ketQua.entries()]);
```

```typescript title=test
const kqSau0 = chayCrawlVaRutGon("trang-goc", 0);
if (kqSau0.size !== 1) throw new Error("doSauToiDa=0 chi duoc crawl DUY NHAT trang-goc");
if (kqSau0.get("trang-goc") !== "0") throw new Error("trang-goc phai nhan ma ngan '0' (gia tri counter dau tien)");

const kqSau2 = chayCrawlVaRutGon("trang-goc", 2);
if (kqSau2.size !== 6) throw new Error("doSauToiDa=2 phai crawl dung 6 trang (goc,a,b,c,d,e), KHONG bao gom trang-f (do sau 3)");
if (kqSau2.has("trang-f")) throw new Error("trang-f o do sau 3 KHONG duoc xuat hien khi doSauToiDa=2");

const maTatCa = [...kqSau2.values()];
if (new Set(maTatCa).size !== maTatCa.length) throw new Error("moi URL phai nhan mot ma ngan DUY NHAT, khong trung lap");
if (kqSau2.get("trang-goc") !== "0") throw new Error("trang-goc (kham pha dau tien theo BFS) phai co ma '0'");
if (kqSau2.get("trang-a") !== "1") throw new Error("trang-a (kham pha thu 2 theo BFS) phai co ma '1'");
if (kqSau2.get("trang-b") !== "2") throw new Error("trang-b (kham pha thu 3 theo BFS) phai co ma '2'");

const kqSau3 = chayCrawlVaRutGon("trang-goc", 3);
if (!kqSau3.has("trang-f")) throw new Error("doSauToiDa=3 PHAI bao gom trang-f");
if (kqSau3.size !== 7) throw new Error("doSauToiDa=3 phai crawl dung 7 trang");
```

:::hints
- kind: attention
  body: "Ngay khi lay hienTai ra khoi hang doi, no can nhan MOT ma ngan moi tu bo dem -- dung ham da xay o bai 3."
- kind: strategy
  body: "laMaNganTiepTheo(boDem) sinh mot ma moi VA tu dong tang bo dem; dat no lam gia tri cho hienTai.url trong ketQua."
- kind: one-line
  body: "ketQua.set(hienTai.url, laMaNganTiepTheo(boDem));"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "trang-goc"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Crawler khám phá đúng THỨ tự, bộ rút gọn cấp mã KHÔNG trùng, giới hạn
độ sâu quyết định PHẠM vi — cả ba ráp KHỚP thành một hệ thống DUY
nhất. Quest "Nền tảng phân TÁN" đã xong.
::::

::::reflect{#nghi-lai}
`chayCrawlVaRutGon` không hề PHÁT minh gì mới — nó chỉ GỌI đúng thứ
tự các mảnh ĐÃ xây riêng lẻ trong chín bài trước: LẤY một URL, CẤP mã
ngay, RỒI mới xét khám phá tiếp. Đây LÀ bài học lặp lại xuyên suốt
Realm 7: một hệ thống LỚN thường không CẦN thuật toán mới, chỉ CẦN
ráp ĐÚNG thứ tự VÀ đúng ranh giới trách nhiệm các mảnh ĐÃ đúng sẵn.
::::

::::checkpoint{mastery=0.85}
::::
