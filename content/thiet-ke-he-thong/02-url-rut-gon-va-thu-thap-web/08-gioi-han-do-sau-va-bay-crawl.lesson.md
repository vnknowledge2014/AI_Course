---
id: thiet-ke-he-thong.url-rut-gon-va-thu-thap-web.gioi-han-do-sau-va-bay-crawl
title: "Giới hạn độ sâu và bẫy crawl"
summary: "coDuocKhamPha(tt, url, doSau, doSauToiDa) từ chối theo HAI điều kiện độc lập: doSau > doSauToiDa (vượt độ sâu -- dùng >, KHÁC quy ước >= của TTL bài 5: chạm đúng giới hạn VẪN được phép) VÀ soLuongTheoDomain.get(domain) >= SO_URL_TOI_DA_MOI_DOMAIN (bẫy crawl -- một domain sinh URL vô hạn như /dem/1,/dem/2,... bị chặn cứng sau N URL dù độ sâu không hề tăng). layDomain lowercase host nên 'B.VN' và 'b.vn' luôn đếm chung MỘT domain. Mô phỏng: domain bay.vn với gioi han 5, thử khám phá 100 URL liên tiếp thực tế chỉ khám phá được đúng 5 trước khi bị chặn vĩnh viễn."
locale: vi
track: thiet-ke-he-thong
module: url-rut-gon-va-thu-thap-web
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.gioi-han-do-sau-va-bay-crawl]
requires: [sd.chuan-hoa-va-khu-trung]
concepts: [sd.gioi-han-do-sau-va-bay-crawl]
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
Chuẩn hoá VÀ khử trùng (bài trước) chặn được URL "khác chuỗi CÙNG
đích". Nhưng có một mối nguy KHÁC: những trang hoàn toàn HỢP lệ,
khác NHAU thật sự, nhưng sinh RA vô hạn — một cái BẪY.
::::

::::explain{#gioi-han-do-sau}
Một crawler cần HAI lớp phòng thủ ĐỘC lập. Lớp MỘT: giới hạn ĐỘ sâu —
không đi XA quá `doSauToiDa` bước tính TỪ URL gốc. Lớp HAI: giới hạn
số URL tối ĐA được khám phá TỪ một domain — chặn "bẫy crawl" (trang
tự sinh liên KẾT vô hạn Ở cùng một chiều SÂU, như
`/dem/1, /dem/2, /dem/3, ...`), thứ mà giới hạn độ SÂU một mình không
chặn được VÌ độ sâu của chuỗi ĐÓ không hề tăng:

```typescript title=readonly
function layDomain(url: string): string {
  const khop = /^[a-zA-Z][a-zA-Z0-9+.-]*:\/\/([^/]+)/.exec(url);
  return (khop?.[1] ?? url).toLowerCase();
}

interface TrangThaiCrawl { soLuongTheoDomain: Map<string, number>; }
function taoTrangThaiCrawl(): TrangThaiCrawl { return { soLuongTheoDomain: new Map() }; }

const SO_URL_TOI_DA_MOI_DOMAIN = 4;

function coDuocKhamPha(tt: TrangThaiCrawl, url: string, doSau: number, doSauToiDa: number): boolean {
  if (doSau > doSauToiDa) return false;
  const domain = layDomain(url);
  const soHienTai = tt.soLuongTheoDomain.get(domain) ?? 0;
  return soHienTai < SO_URL_TOI_DA_MOI_DOMAIN;
}

function ghiNhanKhamPha(tt: TrangThaiCrawl, url: string): void {
  const domain = layDomain(url);
  tt.soLuongTheoDomain.set(domain, (tt.soLuongTheoDomain.get(domain) ?? 0) + 1);
}

const tt = taoTrangThaiCrawl();
console.log("do sau 2, gioi han 3 -- duoc kham pha:", coDuocKhamPha(tt, "http://bao.vn/a", 2, 3));
console.log("do sau 4, gioi han 3 -- duoc kham pha:", coDuocKhamPha(tt, "http://bao.vn/b", 4, 3));

for (let i = 0; i < 4; i++) ghiNhanKhamPha(tt, `http://bao.vn/trang-${i}`);
console.log("da kham pha 4 URL cua bao.vn, gioi han la 4 -- con duoc kham pha them:", coDuocKhamPha(tt, "http://bao.vn/them", 0, 10));
```

```text title=readonly
do sau 2, gioi han 3 -- duoc kham pha: true
do sau 4, gioi han 3 -- duoc kham pha: false
da kham pha 4 URL cua bao.vn, gioi han la 4 -- con duoc kham pha them: false
```

Độ sâu `2` (chưa VƯỢT giới hạn `3`) được PHÉP; độ sâu `4` (vượt QUA
`3`) bị chặn. Sau khi `bao.vn` đã "dùng HẾT" đúng `4` URL (bằng
`SO_URL_TOI_DA_MOI_DOMAIN`), URL thứ NĂM của cùng domain đó bị chặn
— dù độ sâu của nó hoàn toàn BÌNH thường.
::::

::::example{#bay-crawl-thuc-te}
Một máy chủ ĐỘC hại (hoặc chỉ đơn giản LÀ cấu hình sai) có thể sinh
RA một chuỗi URL VÔ hạn — mỗi trang lại trỏ TỚI trang số tiếp theo,
cùng MỘT domain, cùng một độ SÂU. Không có giới hạn theo domain, một
crawler sẽ cắm ĐẦU khám phá MÃI MÃI:

```typescript title=readonly
function layDomain(url: string): string {
  const khop = /^[a-zA-Z][a-zA-Z0-9+.-]*:\/\/([^/]+)/.exec(url);
  return (khop?.[1] ?? url).toLowerCase();
}

interface TrangThaiCrawl { soLuongTheoDomain: Map<string, number>; }
function taoTrangThaiCrawl(): TrangThaiCrawl { return { soLuongTheoDomain: new Map() }; }

const SO_URL_TOI_DA_MOI_DOMAIN = 5;

function coDuocKhamPha(tt: TrangThaiCrawl, url: string, doSau: number, doSauToiDa: number): boolean {
  if (doSau > doSauToiDa) return false;
  const domain = layDomain(url);
  const soHienTai = tt.soLuongTheoDomain.get(domain) ?? 0;
  return soHienTai < SO_URL_TOI_DA_MOI_DOMAIN;
}

function ghiNhanKhamPha(tt: TrangThaiCrawl, url: string): void {
  const domain = layDomain(url);
  tt.soLuongTheoDomain.set(domain, (tt.soLuongTheoDomain.get(domain) ?? 0) + 1);
}

// bay crawl: mot may chu sinh URL VO HAN, moi trang tro toi trang so tiep
// theo -- vd http://bay.vn/dem/1, http://bay.vn/dem/2, http://bay.vn/dem/3, ...
// khong co gioi han so luong moi domain, crawler se cham vo tan trang nay
// MAI MAI (do sau khong tang, vi day la mot chuoi ngang, khong phai cay).
const tt = taoTrangThaiCrawl();
let soDaKhamPha = 0;
let i = 1;
while (soDaKhamPha < 100) {
  const url = `http://bay.vn/dem/${i}`;
  if (!coDuocKhamPha(tt, url, 0, 10)) break;
  ghiNhanKhamPha(tt, url);
  soDaKhamPha++;
  i++;
}
console.log("so URL cua bay.vn da kham pha truoc khi bi CHAN:", soDaKhamPha);
console.log("gioi han moi domain la:", SO_URL_TOI_DA_MOI_DOMAIN);
console.log(`URL thu ${i} co duoc kham pha khong:`, coDuocKhamPha(tt, `http://bay.vn/dem/${i}`, 0, 10));
```

```text title=readonly
so URL cua bay.vn da kham pha truoc khi bi CHAN: 5
gioi han moi domain la: 5
URL thu 6 co duoc kham pha khong: false
```

Dù vòng lặp SẴN sàng thử tới `100` URL VÀ giới hạn độ sâu (`10`)
hoàn toàn không CHẶN chuỗi này (độ sâu luôn LÀ `0`), giới hạn THEO
domain đã dừng crawler LẠI đúng tại `5` URL — CHÍNH xác bằng
`SO_URL_TOI_DA_MOI_DOMAIN`.
::::

::::predict{#doan-hai-dieu-kien-doc-lap commitOnce}
Giới hạn độ sâu LÀ `2`. Một URL Ở đúng độ sâu `2` (KHÔNG phải `3`,
không phải `1`) — VÀ domain của nó CHƯA đạt giới hạn số lượng. Nó
CÓ được `coDuocKhamPha` cho phép không?

:::opt{correct}
CÓ — điều kiện từ chối LÀ `doSau > doSauToiDa` (chỉ từ chối khi VƯỢT
quá), nên độ sâu ĐÚNG bằng giới hạn vẫn được phép, ngược VỚI quy ước
`>=` của TTL (bài 5) nơi CHẠM đúng mốc coi LÀ đã hết hạn
:::
:::opt
KHÔNG — giống hệt quy tắc TTL Ở bài 5 (`>=` khiến chạm đúng mốc LÀ
hết hạn), chạm đúng giới hạn độ sâu cũng phải bị coi LÀ đã "hết
hạn mức", nên bị từ chối
::why
Nhầm "cùng LÀ một ranh giới bằng số" VỚI "mọi ranh giới PHẢI dùng
chung một quy ước" — nhưng MỖI bài chọn quy ước ranh giới theo đúng Ý
nghĩa của chính NÓ, không phải một luật chung cho tất cả.

Chỗ lệch: `coDuocKhamPha` viết `if (doSau > doSauToiDa) return
false;` — dùng `>`, KHÔNG phải `>=`. `doSauToiDa` mang Ý nghĩa "được
phép ĐI tới độ sâu này", nên chạm đúng nó VẪN hợp lệ; chỉ khi VƯỢT
qua (`doSau` lớn HƠN) mới bị chặn. Đây LÀ lựa chọn ngược hẳn VỚI
`layTuCacheTTL` (bài TTL trước, dùng `>=`) — vì Ở đó, `hetHanLuc` LÀ
"thời điểm dữ liệu KHÔNG còn hiệu lực", nên chạm đúng mốc đã LÀ hết
hạn.
::
:::
::::

::::code{#viet_co_duoc_kham_pha}
Hoàn thiện `coDuocKhamPha` — sau khi đã loại trường hợp VƯỢT độ sâu
(dòng TRÊN) và đếm số URL hiện tại của domain, so sánh VỚI giới hạn
để quyết định CHO phép hay từ chối.

```typescript title=starter
function layDomain(url: string): string {
  const khop = /^[a-zA-Z][a-zA-Z0-9+.-]*:\/\/([^/]+)/.exec(url);
  return (khop?.[1] ?? url).toLowerCase();
}

interface TrangThaiCrawl { soLuongTheoDomain: Map<string, number>; }
function taoTrangThaiCrawl(): TrangThaiCrawl { return { soLuongTheoDomain: new Map() }; }

const SO_URL_TOI_DA_MOI_DOMAIN = 3;

function coDuocKhamPha(tt: TrangThaiCrawl, url: string, doSau: number, doSauToiDa: number): boolean {
  if (doSau > doSauToiDa) return false;
  const domain = layDomain(url);
  const soHienTai = tt.soLuongTheoDomain.get(domain) ?? 0;
  ___
}

function ghiNhanKhamPha(tt: TrangThaiCrawl, url: string): void {
  const domain = layDomain(url);
  tt.soLuongTheoDomain.set(domain, (tt.soLuongTheoDomain.get(domain) ?? 0) + 1);
}

const tt = taoTrangThaiCrawl();
console.log(coDuocKhamPha(tt, "http://a.vn/x", 1, 2));
```

```typescript title=solution
function layDomain(url: string): string {
  const khop = /^[a-zA-Z][a-zA-Z0-9+.-]*:\/\/([^/]+)/.exec(url);
  return (khop?.[1] ?? url).toLowerCase();
}

interface TrangThaiCrawl { soLuongTheoDomain: Map<string, number>; }
function taoTrangThaiCrawl(): TrangThaiCrawl { return { soLuongTheoDomain: new Map() }; }

const SO_URL_TOI_DA_MOI_DOMAIN = 3;

function coDuocKhamPha(tt: TrangThaiCrawl, url: string, doSau: number, doSauToiDa: number): boolean {
  if (doSau > doSauToiDa) return false;
  const domain = layDomain(url);
  const soHienTai = tt.soLuongTheoDomain.get(domain) ?? 0;
  return soHienTai < SO_URL_TOI_DA_MOI_DOMAIN;
}

function ghiNhanKhamPha(tt: TrangThaiCrawl, url: string): void {
  const domain = layDomain(url);
  tt.soLuongTheoDomain.set(domain, (tt.soLuongTheoDomain.get(domain) ?? 0) + 1);
}

const tt = taoTrangThaiCrawl();
console.log(coDuocKhamPha(tt, "http://a.vn/x", 1, 2));
```

```typescript title=test
const ttT = taoTrangThaiCrawl();

if (coDuocKhamPha(ttT, "http://a.vn/x", 3, 2) !== false) throw new Error("do sau 3 > gioi han 2 phai bi TU CHOI");
if (coDuocKhamPha(ttT, "http://a.vn/x", 2, 2) !== true) throw new Error("do sau 2 = gioi han 2 (chua VUOT qua) phai duoc CHO PHEP");
if (coDuocKhamPha(ttT, "http://a.vn/x", 0, 2) !== true) throw new Error("do sau 0, chua ghi nhan URL nao cua a.vn, phai duoc CHO PHEP");

for (let i = 0; i < 3; i++) ghiNhanKhamPha(ttT, `http://a.vn/trang-${i}`);
if (coDuocKhamPha(ttT, "http://a.vn/trang-3", 0, 10) !== false) throw new Error("da dat 3/3 URL cua a.vn (bay crawl), URL thu 4 phai bi TU CHOI");
if (coDuocKhamPha(ttT, "http://b.vn/trang-0", 0, 10) !== true) throw new Error("domain KHAC (b.vn) khong bi anh huong boi gioi han cua a.vn");

ghiNhanKhamPha(ttT, "http://B.VN/y");
ghiNhanKhamPha(ttT, "http://b.vn/z");
ghiNhanKhamPha(ttT, "http://B.vn/w");
if (coDuocKhamPha(ttT, "http://b.vn/them-nua", 0, 10) !== false) throw new Error("layDomain phai lowercase host: 'B.VN', 'b.vn', 'B.vn' phai duoc dem la CUNG mot domain (da dat 3/3)");
```

:::hints
- kind: attention
  body: "Con thieu dung mot cau lenh so sanh soHienTai voi SO_URL_TOI_DA_MOI_DOMAIN, roi tra ve ket qua so sanh do."
- kind: strategy
  body: "return soHienTai < SO_URL_TOI_DA_MOI_DOMAIN;"
- kind: one-line
  body: "return soHienTai < SO_URL_TOI_DA_MOI_DOMAIN;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bẫy crawl đã bị chặn. Còn một điều nữa: một crawler LỊCH sự không dồn
DẬP request vào một máy chủ — nó cần "nhường nhịn" theo TỪNG domain.
::::

::::reflect{#nghi-lai}
`coDuocKhamPha` gộp HAI điều kiện độc lập trong CÙNG một hàm — độ sâu
VÀ số lượng theo domain — nhưng chúng bảo vệ khỏi HAI mối nguy khác
hẳn nhau: độ sâu chặn việc đi QUÁ xa khỏi gốc, còn giới hạn domain
chặn việc bị "giữ chân" mãi Ở một chỗ. Một crawler thật CẦN cả hai,
vì bẫy crawl không hề LÀM tăng độ sâu.
::::

::::checkpoint{mastery=0.79}
::::
