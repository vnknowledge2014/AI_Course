---
id: thiet-ke-he-thong.chong-chiu-va-khoi-phuc.rate-limit-bao-ve-downstream
title: "Rate limiting bảo vệ downstream: giới hạn tốc độ GỌI RA, không phải tốc độ nhận vào"
summary: "GioiHanGoiRa tai su dung Y tuong token bucket (nhu quest dinh-danh-va-toc-do) nhung DAO NGUOC huong: KHONG gioi han client goi DEN he thong cua chinh minh, ma gioi han CHINH he thong goi RA mot downstream cu the (vi du mot third-party API co han muc rieng). xinPhepGoiRa tra ve { choPhep, choDenMs } -- choDenMs la SO MILI-GIAY con phai cho truoc khi co du token, tinh tu soTokenThieu = 1 - soTokenHienTai chia tocDoNapMoiGiay. Bucket 2 token, nap 1/giay: 2 goi dau duoc ngay (choDenMs=0), goi 3 bi tu choi CHO 1000ms; sau 500ms van thieu (cho THEM 500ms), sau tong 1000ms moi du 1 token."
locale: vi
track: thiet-ke-he-thong
module: chong-chiu-va-khoi-phuc
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.rate-limit-bao-ve-downstream]
requires: [sd.graceful-degradation-giam-chat-luong-co-kiem-soat]
concepts: [sd.rate-limit-bao-ve-downstream]
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
Quest "định danh và tốc độ" (đầu Realm 7) đã dạy token bucket ĐỂ giới hạn
tốc độ CLIENT gửi request TỚI hệ thống của chính mình — bảo vệ CHÍNH nó
khỏi bị quá tải. Câu hỏi Ở bài NÀY đi NGƯỢC hướng hoàn toàn: hệ thống của
mình gọi RA một dịch vụ khác (ví dụ một third-party API có hạn mức request
riêng) — LÀM sao để KHÔNG làm dịch vụ ĐÓ quá tải?
::::

::::explain{#gioi-han-goi-ra}
`GioiHanGoiRa` tái dùng CHÍNH cơ chế token bucket (`napTheoThoiGian`) đã
học — nhưng lần NÀY, mỗi token đại diện cho "một request ĐƯỢC phép gửi
TỚI downstream", không phải "một request được phép NHẬN vào". `xinPhepGoiRa`
trả về `choPhep` VÀ, khi bị từ chối, `choDenMs` — số mili-giây CẦN đợi
trước khi có đủ token:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface GioiHanGoiRa { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function taoGioiHanGoiRa(soTokenToiDa: number, tocDoNapMoiGiay: number, dh: DongHoMoPhong): GioiHanGoiRa {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa, tocDoNapMoiGiay, thoiDiemNapCuoi: dh.thoiGianHienTai };
}
function napTheoThoiGian(gh: GioiHanGoiRa, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - gh.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * gh.tocDoNapMoiGiay;
  gh.soTokenHienTai = Math.min(gh.soTokenToiDa, gh.soTokenHienTai + soTokenNap);
  gh.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
interface KetQuaXinPhep { choPhep: boolean; choDenMs: number; }
function xinPhepGoiRa(gh: GioiHanGoiRa, dh: DongHoMoPhong): KetQuaXinPhep {
  napTheoThoiGian(gh, dh);
  if (gh.soTokenHienTai >= 1) {
    gh.soTokenHienTai -= 1;
    return { choPhep: true, choDenMs: 0 };
  }
  const soTokenThieu = 1 - gh.soTokenHienTai;
  const soMsCanCho = (soTokenThieu / gh.tocDoNapMoiGiay) * 1000;
  return { choPhep: false, choDenMs: Math.ceil(soMsCanCho) };
}

const dh = taoDongHoMoPhong();
const gh = taoGioiHanGoiRa(2, 1, dh); // gia lap han muc cua mot third-party API: toi da 2 request, nap 1/giay
console.log("goi 1:", JSON.stringify(xinPhepGoiRa(gh, dh)));
console.log("goi 2:", JSON.stringify(xinPhepGoiRa(gh, dh)));
console.log("goi 3 (het token):", JSON.stringify(xinPhepGoiRa(gh, dh)));
```

```text title=readonly
goi 1: {"choPhep":true,"choDenMs":0}
goi 2: {"choPhep":true,"choDenMs":0}
goi 3 (het token): {"choPhep":false,"choDenMs":1000}
```

Hai lần gọi ĐẦU tiêu thụ hết token — GIỐNG hệt token bucket Ở quest trước.
Điểm MỚI LÀ `choDenMs`: khi bị từ chối, `xinPhepGoiRa` KHÔNG chỉ nói
"không", nó còn tính CHÍNH XÁC còn phải đợi bao lâu (`1000`ms, vì thiếu
đúng `1` token, nạp `1`/giây). Vì CHÍNH hệ thống LÀ bên đang gọi ra, nó CÓ
thể chọn ĐỢI đúng khoảng đó rồi gọi TIẾP, thay vì chỉ đơn giản từ chối
như khi bảo vệ chính mình khỏi client lạ.
::::

::::example{#nap-tung-phan-va-doi-them}
`choDenMs` không hề LÀ một hằng số cố định — nó tính LẠI mỗi lần, dựa
trên số token CÒN thiếu tại đúng thời điểm gọi:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface GioiHanGoiRa { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function taoGioiHanGoiRa(soTokenToiDa: number, tocDoNapMoiGiay: number, dh: DongHoMoPhong): GioiHanGoiRa {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa, tocDoNapMoiGiay, thoiDiemNapCuoi: dh.thoiGianHienTai };
}
function napTheoThoiGian(gh: GioiHanGoiRa, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - gh.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * gh.tocDoNapMoiGiay;
  gh.soTokenHienTai = Math.min(gh.soTokenToiDa, gh.soTokenHienTai + soTokenNap);
  gh.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
interface KetQuaXinPhep { choPhep: boolean; choDenMs: number; }
function xinPhepGoiRa(gh: GioiHanGoiRa, dh: DongHoMoPhong): KetQuaXinPhep {
  napTheoThoiGian(gh, dh);
  if (gh.soTokenHienTai >= 1) {
    gh.soTokenHienTai -= 1;
    return { choPhep: true, choDenMs: 0 };
  }
  const soTokenThieu = 1 - gh.soTokenHienTai;
  const soMsCanCho = (soTokenThieu / gh.tocDoNapMoiGiay) * 1000;
  return { choPhep: false, choDenMs: Math.ceil(soMsCanCho) };
}

const dh = taoDongHoMoPhong();
const gh = taoGioiHanGoiRa(2, 1, dh);
xinPhepGoiRa(gh, dh);
xinPhepGoiRa(gh, dh);
xinPhepGoiRa(gh, dh); // het token o day, choDenMs se la 1000

tienThoiGian(dh, 500);
console.log("sau 500ms (nap duoc 0.5 token, chua du 1):", JSON.stringify(xinPhepGoiRa(gh, dh)));

tienThoiGian(dh, 500);
console.log("sau tong 1000ms tu luc het token, du 1 token:", JSON.stringify(xinPhepGoiRa(gh, dh)));
```

```text title=readonly
sau 500ms (nap duoc 0.5 token, chua du 1): {"choPhep":false,"choDenMs":500}
sau tong 1000ms tu luc het token, du 1 token: {"choPhep":true,"choDenMs":0}
```

Sau `500`ms, bucket mới nạp được `0,5` token — CÒN thiếu đúng `0,5`, nên
`choDenMs` giảm XUỐNG còn `500` (không phải vẫn LÀ `1000`). `choDenMs`
LUÔN phản ánh khoảng cách THẬT còn lại tính từ thời điểm gọi, chứ không
LÀ một con số đợi cố định tính TỪ lúc hết token.
::::

::::predict{#doan-doi-du-1-token-hay-day-bucket commitOnce}
Một `GioiHanGoiRa` có `soTokenToiDa=3`, `tocDoNapMoiGiay=1`. Sau khi dùng
HẾT sạch cả `3` token, gọi `xinPhepGoiRa` NGAY (không có thời gian trôi
qua Ở giữa). `choDenMs` trả về LÀ bao nhiêu?

:::opt{correct}
`1000` — chỉ CẦN đợi đủ CHO `1` token MỚI (thiếu đúng `1`, nạp `1`/giây),
KHÔNG cần đợi tới khi bucket ĐẦY lại hoàn toàn `3`
:::
:::opt
`3000` — phải đợi ĐỦ thời gian để nạp LẠI toàn bộ `3` token (bucket vừa
bị dùng HẾT sạch), tức `soTokenToiDa / tocDoNapMoiGiay * 1000`
::why
Nhầm "vừa dùng hết TOÀN bộ bucket" VỚI "phải chờ bucket ĐẦY lại toàn bộ"
— nhưng `xinPhepGoiRa` chỉ cần ĐỦ MỘT token để cho qua LẦN gọi tiếp theo,
không cần đợi hết cả `soTokenToiDa`.

Chỗ lệch: dòng `const soTokenThieu = 1 - gh.soTokenHienTai;` LUÔN tính so
với ngưỡng `1` (đủ cho ĐÚNG một lần gọi kế tiếp), không phải so với
`soTokenToiDa`. Với `soTokenHienTai=0`, `soTokenThieu = 1`, VÀ
`soMsCanCho = (1 / 1) * 1000 = 1000`. Hai token CÒN lại (Ở mức tối đa `3`)
sẽ tiếp tục nạp DẦN sau đó, nhưng KHÔNG hề LÀ điều kiện để cho phép lần
gọi tiếp theo.
::
:::
::::

::::code{#viet_xin_phep_goi_ra}
Hoàn thiện `xinPhepGoiRa` cho nhánh KHÔNG đủ token (`gh.soTokenHienTai <
1`, phần TRƯỚC đó đã có sẵn) — tính số token CÒN thiếu (`1 -
soTokenHienTai`), suy ra số mili-giây cần đợi (chia CHO `tocDoNapMoiGiay`,
nhân `1000`, làm tròn LÊN bằng `Math.ceil`), RỒI trả về `{ choPhep: false,
choDenMs }`.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface GioiHanGoiRa { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function taoGioiHanGoiRa(soTokenToiDa: number, tocDoNapMoiGiay: number, dh: DongHoMoPhong): GioiHanGoiRa {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa, tocDoNapMoiGiay, thoiDiemNapCuoi: dh.thoiGianHienTai };
}
function napTheoThoiGian(gh: GioiHanGoiRa, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - gh.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * gh.tocDoNapMoiGiay;
  gh.soTokenHienTai = Math.min(gh.soTokenToiDa, gh.soTokenHienTai + soTokenNap);
  gh.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
interface KetQuaXinPhep { choPhep: boolean; choDenMs: number; }

function xinPhepGoiRa(gh: GioiHanGoiRa, dh: DongHoMoPhong): KetQuaXinPhep {
  napTheoThoiGian(gh, dh);
  if (gh.soTokenHienTai >= 1) {
    gh.soTokenHienTai -= 1;
    return { choPhep: true, choDenMs: 0 };
  }
  ___
}

const dhX = taoDongHoMoPhong();
const ghX = taoGioiHanGoiRa(1, 1, dhX);
xinPhepGoiRa(ghX, dhX);
console.log(JSON.stringify(xinPhepGoiRa(ghX, dhX)));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface GioiHanGoiRa { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function taoGioiHanGoiRa(soTokenToiDa: number, tocDoNapMoiGiay: number, dh: DongHoMoPhong): GioiHanGoiRa {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa, tocDoNapMoiGiay, thoiDiemNapCuoi: dh.thoiGianHienTai };
}
function napTheoThoiGian(gh: GioiHanGoiRa, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - gh.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * gh.tocDoNapMoiGiay;
  gh.soTokenHienTai = Math.min(gh.soTokenToiDa, gh.soTokenHienTai + soTokenNap);
  gh.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
interface KetQuaXinPhep { choPhep: boolean; choDenMs: number; }

function xinPhepGoiRa(gh: GioiHanGoiRa, dh: DongHoMoPhong): KetQuaXinPhep {
  napTheoThoiGian(gh, dh);
  if (gh.soTokenHienTai >= 1) {
    gh.soTokenHienTai -= 1;
    return { choPhep: true, choDenMs: 0 };
  }
  const soTokenThieu = 1 - gh.soTokenHienTai;
  const soMsCanCho = (soTokenThieu / gh.tocDoNapMoiGiay) * 1000;
  return { choPhep: false, choDenMs: Math.ceil(soMsCanCho) };
}

const dhX = taoDongHoMoPhong();
const ghX = taoGioiHanGoiRa(1, 1, dhX);
xinPhepGoiRa(ghX, dhX);
console.log(JSON.stringify(xinPhepGoiRa(ghX, dhX)));
```

```typescript title=test
const dhT = taoDongHoMoPhong();
const ghT = taoGioiHanGoiRa(2, 1, dhT);

const kq1T = xinPhepGoiRa(ghT, dhT);
if (kq1T.choPhep !== true || kq1T.choDenMs !== 0) throw new Error("con token thi phai cho phep ngay, choDenMs=0");
const kq2T = xinPhepGoiRa(ghT, dhT);
if (kq2T.choPhep !== true) throw new Error("token thu hai van con, phai cho phep");
const kq3T = xinPhepGoiRa(ghT, dhT);
if (kq3T.choPhep !== false) throw new Error("het token (2/2 da dung) phai TU CHOI");
if (kq3T.choDenMs !== 1000) throw new Error("het sach token (0 con lai), toc do 1/giay, phai cho DUNG 1000ms");

tienThoiGian(dhT, 500);
const kq4T = xinPhepGoiRa(ghT, dhT);
if (kq4T.choPhep !== false) throw new Error("moi nap duoc 0.5 token (500ms x 1/giay), chua du 1, van phai tu choi");
if (kq4T.choDenMs !== 500) throw new Error("con thieu 0.5 token, toc do 1/giay, phai cho THEM 500ms");

tienThoiGian(dhT, 500);
const kq5T = xinPhepGoiRa(ghT, dhT);
if (kq5T.choPhep !== true) throw new Error("du tong 1000ms tu luc het token, phai nap du 1 token va cho phep");
if (kq5T.choDenMs !== 0) throw new Error("khi cho phep, choDenMs phai la 0");
```

:::hints
- kind: attention
  body: "Ba buoc: tinh soTokenThieu = 1 - gh.soTokenHienTai; tinh soMsCanCho = (soTokenThieu / gh.tocDoNapMoiGiay) * 1000; tra ve { choPhep: false, choDenMs: Math.ceil(soMsCanCho) }."
- kind: strategy
  body: "const soTokenThieu = 1 - gh.soTokenHienTai; const soMsCanCho = (soTokenThieu / gh.tocDoNapMoiGiay) * 1000; return { choPhep: false, choDenMs: Math.ceil(soMsCanCho) };"
- kind: one-line
  body: "const soTokenThieu = 1 - gh.soTokenHienTai; const soMsCanCho = (soTokenThieu / gh.tocDoNapMoiGiay) * 1000; return { choPhep: false, choDenMs: Math.ceil(soMsCanCho) };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "choPhep\":false,\"choDenMs\":1000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hệ thống giờ biết TỰ kìm tốc độ gọi ra, VÀ biết CHÍNH XÁC còn phải đợi
bao lâu — thay vì cứ gọi rồi hứng chịu lỗi từ downstream. Nhưng rate
limit chỉ ngăn được lỗi DO chính mình gây ra. Khi hạ tầng mất SẠCH dữ
liệu (không phải lỗi tạm thời), câu hỏi hoàn toàn khác: có khôi phục lại
được không, VÀ mất bao nhiêu?
::::

::::reflect{#nghi-lai}
`xinPhepGoiRa` VÀ token bucket bảo vệ CHÍNH hệ thống (quest trước) dùng
CHUNG một cơ chế toán học — `napTheoThoiGian` không hề đổi MỘT dòng nào.
Điều thay đổi HOÀN toàn LÀ VAI trò của mỗi bên trong quan hệ: khi bảo vệ
chính mình, hệ thống LÀ nạn nhân tiềm năng NÊN từ chối là đủ; khi bảo vệ
downstream, hệ thống LÀ bên có khả năng gây hại NÊN nó cần biết ĐỢI bao
lâu để tự giác quay lại đúng lúc, thay vì dội tiếp một request khác NGAY
LẬP TỨC. CÙNG một công thức, nhưng đặt sai HƯỚNG thì bảo vệ SAI đối
tượng.
::::

::::checkpoint{mastery=0.81}
::::
