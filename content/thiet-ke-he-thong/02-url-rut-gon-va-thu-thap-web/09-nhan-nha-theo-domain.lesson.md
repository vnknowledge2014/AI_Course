---
id: thiet-ke-he-thong.url-rut-gon-va-thu-thap-web.nhan-nha-theo-domain
title: "Nhã nhặn theo domain: mỗi máy chủ một bucket riêng"
summary: "xinPhepTaiDomain tái khai báo ĐẦY ĐỦ, độc lập token bucket (ThungTokenTG + napTheoThoiGian + tieuThuTokenTG, y hệt cơ chế đã dạy ở quest dinh-danh-va-toc-do) nhưng đặt trong Map<string, ThungTokenTG> — MỖI domain có bucket RIÊNG, tạo lười (lazy) ở lần request ĐẦU tiên của domain đó. bao.vn (bucket 2 token, nạp 1/giây) dùng hết 2 token ở request 3 bị từ chối, nhưng cho.vn (bucket riêng, CHƯA từng bị chạm) vẫn cho qua BÌNH THƯỜNG ngay lập tức — domain A hết token không ảnh hưởng domain B."
locale: vi
track: thiet-ke-he-thong
module: url-rut-gon-va-thu-thap-web
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.nhan-nha-theo-domain]
requires: [sd.gioi-han-do-sau-va-bay-crawl]
concepts: [sd.nhan-nha-theo-domain]
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
Bẫy crawl (bài trước) đã bị chặn Ở mức TOÀN cục. Nhưng còn một vấn đề
KHÁC: ngay cả khi không CÓ bẫy, một crawler chạy đủ NHANH vẫn có thể
dội hàng NGÀN request vào một máy chủ NHỎ chỉ trong vài giây.
::::

::::explain{#bucket-rieng-cho-domain}
Quest TRƯỚC đã dạy token bucket (nạp token theo THỜI gian, tiêu thụ
mỗi request MỘT token). Ý tưởng ĐÓ áp dụng LẠI Ở đây — nhưng thay vì
MỘT bucket chung cho toàn hệ thống, mỗi DOMAIN cần bucket RIÊNG của
nó, giữ trong một `Map`. `xinPhepTaiDomain` tạo bucket MỚI (đầy) cho
domain LẦN đầu gặp, rồi tái sử DỤNG nó cho các lần SAU:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungTokenTG { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function taoThungTokenTG(soTokenToiDa: number, tocDoNapMoiGiay: number, dh: DongHoMoPhong): ThungTokenTG {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa, tocDoNapMoiGiay, thoiDiemNapCuoi: dh.thoiGianHienTai };
}
function napTheoThoiGian(thung: ThungTokenTG, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * thung.tocDoNapMoiGiay;
  thung.soTokenHienTai = Math.min(thung.soTokenToiDa, thung.soTokenHienTai + soTokenNap);
  thung.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
function tieuThuTokenTG(thung: ThungTokenTG, dh: DongHoMoPhong): boolean {
  napTheoThoiGian(thung, dh);
  if (thung.soTokenHienTai < 1) return false;
  thung.soTokenHienTai -= 1;
  return true;
}

interface BoGioiHanTheoDomain { thungTheoDomain: Map<string, ThungTokenTG>; }
function taoBoGioiHanTheoDomain(): BoGioiHanTheoDomain { return { thungTheoDomain: new Map() }; }

function xinPhepTaiDomain(bo: BoGioiHanTheoDomain, dh: DongHoMoPhong, domain: string, soTokenToiDa: number, tocDoNapMoiGiay: number): boolean {
  let thung = bo.thungTheoDomain.get(domain);
  if (thung === undefined) {
    thung = taoThungTokenTG(soTokenToiDa, tocDoNapMoiGiay, dh);
    bo.thungTheoDomain.set(domain, thung);
  }
  return tieuThuTokenTG(thung, dh);
}

const dh = taoDongHoMoPhong();
const bo = taoBoGioiHanTheoDomain();

console.log("bao.vn request 1:", xinPhepTaiDomain(bo, dh, "bao.vn", 2, 1));
console.log("bao.vn request 2:", xinPhepTaiDomain(bo, dh, "bao.vn", 2, 1));
console.log("bao.vn request 3:", xinPhepTaiDomain(bo, dh, "bao.vn", 2, 1));
console.log("so domain dang duoc theo doi:", bo.thungTheoDomain.size);
```

```text title=readonly
bao.vn request 1: true
bao.vn request 2: true
bao.vn request 3: false
so domain dang duoc theo doi: 1
```

Bucket của `bao.vn` (đầy `2` token LÚC tạo) cho QUA hai request đầu,
từ chối request THỨ ba — y hệt token bucket đơn LẺ đã học Ở quest
trước. Điểm MỚI LÀ: nó nằm trong `thungTheoDomain.get(domain)`, được
TẠO đúng một lần cho MỖI domain riêng biệt.
::::

::::example{#doc-lap-giua-cac-domain}
Điểm mấu CHỐT của "nhã nhặn theo domain": một domain BỊ dồn dập tới
mức hết TOKEN hoàn toàn KHÔNG ảnh hưởng tới khả năng request các
domain KHÁC — mỗi bucket sống trong THẾ giới riêng của nó:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungTokenTG { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function taoThungTokenTG(soTokenToiDa: number, tocDoNapMoiGiay: number, dh: DongHoMoPhong): ThungTokenTG {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa, tocDoNapMoiGiay, thoiDiemNapCuoi: dh.thoiGianHienTai };
}
function napTheoThoiGian(thung: ThungTokenTG, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * thung.tocDoNapMoiGiay;
  thung.soTokenHienTai = Math.min(thung.soTokenToiDa, thung.soTokenHienTai + soTokenNap);
  thung.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
function tieuThuTokenTG(thung: ThungTokenTG, dh: DongHoMoPhong): boolean {
  napTheoThoiGian(thung, dh);
  if (thung.soTokenHienTai < 1) return false;
  thung.soTokenHienTai -= 1;
  return true;
}

interface BoGioiHanTheoDomain { thungTheoDomain: Map<string, ThungTokenTG>; }
function taoBoGioiHanTheoDomain(): BoGioiHanTheoDomain { return { thungTheoDomain: new Map() }; }

function xinPhepTaiDomain(bo: BoGioiHanTheoDomain, dh: DongHoMoPhong, domain: string, soTokenToiDa: number, tocDoNapMoiGiay: number): boolean {
  let thung = bo.thungTheoDomain.get(domain);
  if (thung === undefined) {
    thung = taoThungTokenTG(soTokenToiDa, tocDoNapMoiGiay, dh);
    bo.thungTheoDomain.set(domain, thung);
  }
  return tieuThuTokenTG(thung, dh);
}

// bao.vn bi khai thac het token, nhung cho.vn hoan toan khong bi anh huong --
// moi domain co bucket RIENG, khong he chia se voi nhau.
const dh = taoDongHoMoPhong();
const bo = taoBoGioiHanTheoDomain();

const ketQuaBao: boolean[] = [];
for (let i = 0; i < 3; i++) ketQuaBao.push(xinPhepTaiDomain(bo, dh, "bao.vn", 2, 1));
console.log("bao.vn (bucket 2 token):", ketQuaBao.join(","));

console.log("cho.vn (bucket rieng, chua bi cham toi):", xinPhepTaiDomain(bo, dh, "cho.vn", 2, 1));
console.log("cho.vn request 2:", xinPhepTaiDomain(bo, dh, "cho.vn", 2, 1));

tienThoiGian(dh, 1000);
console.log("sau 1000ms, bao.vn nap lai 1 token, request moi:", xinPhepTaiDomain(bo, dh, "bao.vn", 2, 1));
console.log("tong so domain dang theo doi:", bo.thungTheoDomain.size);
```

```text title=readonly
bao.vn (bucket 2 token): true,true,false
cho.vn (bucket rieng, chua bi cham toi): true
cho.vn request 2: true
sau 1000ms, bao.vn nap lai 1 token, request moi: true
tong so domain dang theo doi: 2
```

`bao.vn` bị TỪ chối Ở request thứ ba (hết TOKEN) — nhưng NGAY sau
đó, `cho.vn` vẫn được CHO qua bình thường Ở CẢ hai request, không hề
"lây" trạng thái hết token của `bao.vn`. Sau `1000`ms, `bao.vn` nạp
lại đúng `1` token VÀ được PHÉP tiếp — hoàn toàn độc lập VỚI những gì
xảy ra Ở `cho.vn`.
::::

::::predict{#doan-bucket-moi-day-hay-rong commitOnce}
`xinPhepTaiDomain` được gọi LẦN đầu tiên cho một domain HOÀN toàn
mới, `soTokenToiDa=3`. Bucket của domain ĐÓ được tạo VỚI
`soTokenHienTai` bằng bao NHIÊU — đầy hay rỗng — TRƯỚC khi request
đầu tiên ĐƯỢC xử lý?

:::opt{correct}
Đầy (`3`) — `taoThungTokenTG` khởi tạo `soTokenHienTai: soTokenToiDa`,
nên request đầu TIÊN của một domain mới luôn có cơ HỘI được chấp
nhận NGAY
:::
:::opt
Rỗng (`0`) — domain mới phải "chờ" nạp DẦN token trước khi request
đầu tiên có thể được CHẤP nhận
::why
Nhầm "bucket MỚI tạo" VỚI "bucket đã bị tiêu thụ HẾT" — nhưng
`taoThungTokenTG` không hề khởi tạo VỀ `0`.

Chỗ lệch: dòng `return { soTokenHienTai: soTokenToiDa, soTokenToiDa,
... };` trong `taoThungTokenTG` gán `soTokenHienTai` BẰNG đúng
`soTokenToiDa` — nghĩa LÀ bucket luôn bắt đầu ĐẦY. Trong
`xinPhepTaiDomain`, nhánh `if (thung === undefined)` chỉ chạy đúng
MỘT lần cho mỗi domain (lần gặp ĐẦU tiên), VÀ nó luôn tạo một bucket
đầy TRƯỚC khi `tieuThuTokenTG` được gọi.
::
:::
::::

::::code{#viet_xin_phep_tai_domain}
Hoàn thiện `xinPhepTaiDomain` — khi domain CHƯA có bucket (nhánh
`undefined`), tạo một bucket MỚI VÀ lưu nó vào `Map` trước khi dùng.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungTokenTG { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function taoThungTokenTG(soTokenToiDa: number, tocDoNapMoiGiay: number, dh: DongHoMoPhong): ThungTokenTG {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa, tocDoNapMoiGiay, thoiDiemNapCuoi: dh.thoiGianHienTai };
}
function napTheoThoiGian(thung: ThungTokenTG, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * thung.tocDoNapMoiGiay;
  thung.soTokenHienTai = Math.min(thung.soTokenToiDa, thung.soTokenHienTai + soTokenNap);
  thung.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
function tieuThuTokenTG(thung: ThungTokenTG, dh: DongHoMoPhong): boolean {
  napTheoThoiGian(thung, dh);
  if (thung.soTokenHienTai < 1) return false;
  thung.soTokenHienTai -= 1;
  return true;
}

interface BoGioiHanTheoDomain { thungTheoDomain: Map<string, ThungTokenTG>; }
function taoBoGioiHanTheoDomain(): BoGioiHanTheoDomain { return { thungTheoDomain: new Map() }; }

function xinPhepTaiDomain(bo: BoGioiHanTheoDomain, dh: DongHoMoPhong, domain: string, soTokenToiDa: number, tocDoNapMoiGiay: number): boolean {
  let thung = bo.thungTheoDomain.get(domain);
  if (thung === undefined) {
    ___
  }
  return tieuThuTokenTG(thung, dh);
}

const dh = taoDongHoMoPhong();
const bo = taoBoGioiHanTheoDomain();
console.log(xinPhepTaiDomain(bo, dh, "a.vn", 1, 1), xinPhepTaiDomain(bo, dh, "a.vn", 1, 1));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungTokenTG { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function taoThungTokenTG(soTokenToiDa: number, tocDoNapMoiGiay: number, dh: DongHoMoPhong): ThungTokenTG {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa, tocDoNapMoiGiay, thoiDiemNapCuoi: dh.thoiGianHienTai };
}
function napTheoThoiGian(thung: ThungTokenTG, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * thung.tocDoNapMoiGiay;
  thung.soTokenHienTai = Math.min(thung.soTokenToiDa, thung.soTokenHienTai + soTokenNap);
  thung.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
function tieuThuTokenTG(thung: ThungTokenTG, dh: DongHoMoPhong): boolean {
  napTheoThoiGian(thung, dh);
  if (thung.soTokenHienTai < 1) return false;
  thung.soTokenHienTai -= 1;
  return true;
}

interface BoGioiHanTheoDomain { thungTheoDomain: Map<string, ThungTokenTG>; }
function taoBoGioiHanTheoDomain(): BoGioiHanTheoDomain { return { thungTheoDomain: new Map() }; }

function xinPhepTaiDomain(bo: BoGioiHanTheoDomain, dh: DongHoMoPhong, domain: string, soTokenToiDa: number, tocDoNapMoiGiay: number): boolean {
  let thung = bo.thungTheoDomain.get(domain);
  if (thung === undefined) {
    thung = taoThungTokenTG(soTokenToiDa, tocDoNapMoiGiay, dh);
    bo.thungTheoDomain.set(domain, thung);
  }
  return tieuThuTokenTG(thung, dh);
}

const dh = taoDongHoMoPhong();
const bo = taoBoGioiHanTheoDomain();
console.log(xinPhepTaiDomain(bo, dh, "a.vn", 1, 1), xinPhepTaiDomain(bo, dh, "a.vn", 1, 1));
```

```typescript title=test
const dhT = taoDongHoMoPhong();
const boT = taoBoGioiHanTheoDomain();

if (xinPhepTaiDomain(boT, dhT, "a.vn", 2, 1) !== true) throw new Error("request dau tien cua domain moi phai duoc CHO PHEP");
if (xinPhepTaiDomain(boT, dhT, "a.vn", 2, 1) !== true) throw new Error("request thu 2 (con token) phai duoc CHO PHEP");
if (xinPhepTaiDomain(boT, dhT, "a.vn", 2, 1) !== false) throw new Error("request thu 3 (het token) phai bi TU CHOI");

if (xinPhepTaiDomain(boT, dhT, "b.vn", 2, 1) !== true) throw new Error("domain KHAC (b.vn) phai co bucket RIENG, khong bi anh huong boi a.vn da het token");
if (boT.thungTheoDomain.size !== 2) throw new Error("phai co dung 2 bucket rieng biet, mot cho a.vn mot cho b.vn");

tienThoiGian(dhT, 1000);
if (xinPhepTaiDomain(boT, dhT, "a.vn", 2, 1) !== true) throw new Error("sau 1000ms (nap lai 1 token/giay), a.vn phai duoc CHO PHEP lai");
```

:::hints
- kind: attention
  body: "Nhanh nay chi chay khi domain CHUA co bucket -- can tao bucket MOI (dung ham co san) VA luu no vao Map truoc khi dung."
- kind: strategy
  body: "thung = taoThungTokenTG(soTokenToiDa, tocDoNapMoiGiay, dh); bo.thungTheoDomain.set(domain, thung);"
- kind: one-line
  body: "thung = taoThungTokenTG(soTokenToiDa, tocDoNapMoiGiay, dh); bo.thungTheoDomain.set(domain, thung);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "true false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chín mảnh đã xong: hai chiều mã hoá, bộ đếm, alias, hết hạn, hàng đợi
BFS, chuẩn hoá, chống bẫy, VÀ nhã nhặn theo domain. Giờ LÀ lúc ráp
TẤT cả thành một hệ thống DUY nhất.
::::

::::reflect{#nghi-lai}
`xinPhepTaiDomain` không hề PHÁT minh lại token bucket — nó tái sử
DỤNG nguyên VẸN cơ chế đã học Ở quest trước, chỉ THÊM đúng một lớp:
một `Map` ánh XẠ mỗi khoá (domain) TỚI trạng thái RIÊNG của chính nó.
Đây LÀ một khuôn mẫu lặp lại rất NHIỀU trong hệ thống thật — "N bản
sao độc lập của MỘT cơ chế đã biết", không phải một thuật toán MỚI.
::::

::::checkpoint{mastery=0.81}
::::
