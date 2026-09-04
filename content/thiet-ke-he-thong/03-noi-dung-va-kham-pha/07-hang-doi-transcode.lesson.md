---
id: thiet-ke-he-thong.noi-dung-va-kham-pha.hang-doi-transcode
title: "Hàng đợi transcode: chưa xong ngay đâu"
summary: "tienTrinhXuLy mo phong 1 worker duy nhat: moi lan goi tien DUNG mot buoc (hoac nhan viec moi neu dang ranh, hoac dong viec neu da du THOI_GIAN_XU_LY_MS=300ms) -- khong ca hai trong CUNG mot lan goi vi nhanh nhan-viec nam TRUOC nhanh dong-viec. Voi 2 video: goi 1 lan tai t=300 chi dong v1 (v2 van cho_xu_ly); phai goi THEM 1 lan (van t=300) v2 moi chuyen dang_xu_ly."
locale: vi
track: thiet-ke-he-thong
module: noi-dung-va-kham-pha
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.hang-doi-transcode]
requires: [sd.cap-nhat-tan-suat-dinh-ky]
concepts: [sd.hang-doi-transcode]
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
Search Autocomplete xong. Mảng THỨ ba: một nền tảng video. Bấm "tải lên"
KHÔNG có nghĩa LÀ video sẵn sàng ngay — nó phải đi QUA một hàng đợi xử
lý (transcode) TRƯỚC đã.
::::

::::explain{#hang-doi-transcode}
Video mới nộp mang trạng thái `"cho_xu_ly"`. Một worker (Ở đây LÀ đúng
MỘT, đơn giản hoá) lấy video RA khỏi hàng đợi, chuyển nó sang
`"dang_xu_ly"`, VÀ chỉ sau `THOI_GIAN_XU_LY_MS` mô phỏng mới đánh dấu
`"hoan_tat"`. Mỗi lần GỌI `tienTrinhXuLy` chỉ tiến ĐÚNG một bước — HOẶC
nhận việc MỚI (nếu đang rảnh), HOẶC đóng việc ĐANG chạy (nếu đã đủ giờ),
không LÀM cả hai TRONG cùng một lần gọi:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiVideo = "cho_xu_ly" | "dang_xu_ly" | "hoan_tat";

interface HeThongTranscode {
  hangDoi: string[];
  trangThai: Map<string, TrangThaiVideo>;
  dangXuLy: string | undefined;
  thoiDiemXongDuKien: number;
}
function taoHeThongTranscode(): HeThongTranscode {
  return { hangDoi: [], trangThai: new Map(), dangXuLy: undefined, thoiDiemXongDuKien: 0 };
}

const THOI_GIAN_XU_LY_MS = 300;

function nopVideoMoi(ht: HeThongTranscode, idVideo: string): void {
  ht.hangDoi.push(idVideo);
  ht.trangThai.set(idVideo, "cho_xu_ly");
}

function tienTrinhXuLy(ht: HeThongTranscode, dh: DongHoMoPhong): void {
  if (ht.dangXuLy === undefined && ht.hangDoi.length > 0) {
    const idVideo = ht.hangDoi.shift()!;
    ht.dangXuLy = idVideo;
    ht.trangThai.set(idVideo, "dang_xu_ly");
    ht.thoiDiemXongDuKien = dh.thoiGianHienTai + THOI_GIAN_XU_LY_MS;
    return;
  }
  if (ht.dangXuLy !== undefined && dh.thoiGianHienTai >= ht.thoiDiemXongDuKien) {
    ht.trangThai.set(ht.dangXuLy, "hoan_tat");
    ht.dangXuLy = undefined;
  }
}

const dh = taoDongHoMoPhong();
const ht = taoHeThongTranscode();
nopVideoMoi(ht, "v1");
console.log("ngay sau khi nop:", ht.trangThai.get("v1"));
tienTrinhXuLy(ht, dh);
console.log("goi tienTrinhXuLy lan 1 (worker ranh, nhan viec ngay):", ht.trangThai.get("v1"));
tienThoiGian(dh, 299);
tienTrinhXuLy(ht, dh);
console.log("t=299 (chua du 300ms):", ht.trangThai.get("v1"));
tienThoiGian(dh, 1);
tienTrinhXuLy(ht, dh);
console.log("t=300 (dung thoi gian xu ly):", ht.trangThai.get("v1"));
```

```text title=readonly
ngay sau khi nop: cho_xu_ly
goi tienTrinhXuLy lan 1 (worker ranh, nhan viec ngay): dang_xu_ly
t=299 (chua du 300ms): dang_xu_ly
t=300 (dung thoi gian xu ly): hoan_tat
```

Video đi ĐÚNG qua ba trạng thái theo THỨ tự: `cho_xu_ly` → `dang_xu_ly`
→ `hoan_tat`. Tại `t=299` (còn CÁCH `THOI_GIAN_XU_LY_MS` đúng `1ms`) nó
VẪN `dang_xu_ly` — chỉ đúng `t=300` mới `hoan_tat`.
::::

::::example{#mot-worker-duy-nhat}
Với `2` video CÙNG nộp, chỉ MỘT video được nhận xử LÝ tại một thời
điểm. Đóng video ĐANG chạy VÀ nhận video kế tiếp LÀ hai bước RIÊNG —
cần gọi `tienTrinhXuLy` HAI lần liên tiếp (dù đồng hồ KHÔNG hề tiến
thêm) để cả hai việc CÙNG xảy ra:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiVideo = "cho_xu_ly" | "dang_xu_ly" | "hoan_tat";

interface HeThongTranscode {
  hangDoi: string[];
  trangThai: Map<string, TrangThaiVideo>;
  dangXuLy: string | undefined;
  thoiDiemXongDuKien: number;
}
function taoHeThongTranscode(): HeThongTranscode {
  return { hangDoi: [], trangThai: new Map(), dangXuLy: undefined, thoiDiemXongDuKien: 0 };
}

const THOI_GIAN_XU_LY_MS = 300;

function nopVideoMoi(ht: HeThongTranscode, idVideo: string): void {
  ht.hangDoi.push(idVideo);
  ht.trangThai.set(idVideo, "cho_xu_ly");
}

function tienTrinhXuLy(ht: HeThongTranscode, dh: DongHoMoPhong): void {
  if (ht.dangXuLy === undefined && ht.hangDoi.length > 0) {
    const idVideo = ht.hangDoi.shift()!;
    ht.dangXuLy = idVideo;
    ht.trangThai.set(idVideo, "dang_xu_ly");
    ht.thoiDiemXongDuKien = dh.thoiGianHienTai + THOI_GIAN_XU_LY_MS;
    return;
  }
  if (ht.dangXuLy !== undefined && dh.thoiGianHienTai >= ht.thoiDiemXongDuKien) {
    ht.trangThai.set(ht.dangXuLy, "hoan_tat");
    ht.dangXuLy = undefined;
  }
}

const dh = taoDongHoMoPhong();
const ht = taoHeThongTranscode();
nopVideoMoi(ht, "v1");
nopVideoMoi(ht, "v2");
tienTrinhXuLy(ht, dh);
console.log("ngay sau khi nop ca 2 (1 lan goi):", "v1=" + ht.trangThai.get("v1"), "v2=" + ht.trangThai.get("v2"));

tienThoiGian(dh, 300);
tienTrinhXuLy(ht, dh);
console.log("t=300, goi 1 lan (chi dong v1, v2 CHUA duoc nhan):", "v1=" + ht.trangThai.get("v1"), "v2=" + ht.trangThai.get("v2"));

tienTrinhXuLy(ht, dh);
console.log("goi THEM 1 lan nua (van t=300, worker gio moi nhan v2):", "v1=" + ht.trangThai.get("v1"), "v2=" + ht.trangThai.get("v2"));
```

```text title=readonly
ngay sau khi nop ca 2 (1 lan goi): v1=dang_xu_ly v2=cho_xu_ly
t=300, goi 1 lan (chi dong v1, v2 CHUA duoc nhan): v1=hoan_tat v2=cho_xu_ly
goi THEM 1 lan nua (van t=300, worker gio moi nhan v2): v1=hoan_tat v2=dang_xu_ly
```

Tại `t=300`, LẦN gọi đầu tiên đi VÀO nhánh "đóng việc" (vì `dangXuLy`
đang LÀ `v1`, không phải `undefined`) — nhánh "nhận việc MỚI" đã bị bỏ
qua Ở ngay ĐẦU hàm, TRƯỚC khi `v1` kịp được đóng. `v2` chỉ được nhận Ở
lần GỌI thứ hai, khi `dangXuLy` đã LÀ `undefined`.
::::

::::predict{#doan-mot-lan-goi-mot-buoc commitOnce}
Hai video `v1`, `v2` đã nộp; `v1` đang `dang_xu_ly`, `v2` đang
`cho_xu_ly`. Đồng hồ vừa chạm ĐÚNG thời điểm `v1` xử lý xong. Gọi
`tienTrinhXuLy` ĐÚNG một lần. Ngay SAU lần gọi đó, `v2` đã chuyển sang
`dang_xu_ly` chưa?

:::opt{correct}
CHƯA — lần gọi đó CHỈ đủ để đóng `v1` (chuyển `hoan_tat`) VÀ giải phóng
worker; việc NHẬN `v2` chỉ xảy ra Ở lần gọi TIẾP theo, vì nhánh "nhận
việc mới" đã được kiểm tra (VÀ bỏ qua, do `dangXuLy` LÚC đó còn LÀ
`v1`) trước cả nhánh "đóng việc"
:::
:::opt
RỒI — worker vừa RẢNH ra ngay trong lần gọi ĐÓ, nên hàm sẽ tự ĐỘNG nhận
việc tiếp theo NGAY trong cùng một lần gọi
::why
Nhầm "worker đã rảnh SAU khi hàm chạy xong" với "worker rảnh NGAY từ
lúc BẮT đầu lần gọi đó" — nhưng thứ tự HAI nhánh `if` bên trong
`tienTrinhXuLy` không cho PHÉP điều đó xảy ra trong CÙNG một lần gọi.

Chỗ lệch: nhánh "nhận việc mới" (`if (ht.dangXuLy === undefined && ...)`)
nằm Ở TRÊN nhánh "đóng việc" trong THÂN hàm. Tại thời điểm lần gọi NÀY
bắt đầu, `ht.dangXuLy` VẪN còn LÀ `"v1"` (chưa hề đóng), NÊN nhánh nhận
việc bị bỏ qua NGAY — hàm rơi xuống nhánh đóng việc, đặt `v1` thành
`hoan_tat` VÀ `dangXuLy = undefined`, RỒI hàm kết thúc. `v2` phải chờ
một lần GỌI hoàn toàn mới để được nhận.
::
:::
::::

::::code{#viet_tien_trinh_xu_ly}
Hoàn thiện `tienTrinhXuLy` — nhánh THỨ hai: nếu worker đang xử LÝ một
video VÀ đã đủ thời gian, đánh dấu video đó `"hoan_tat"` VÀ giải phóng
worker.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiVideo = "cho_xu_ly" | "dang_xu_ly" | "hoan_tat";

interface HeThongTranscode {
  hangDoi: string[];
  trangThai: Map<string, TrangThaiVideo>;
  dangXuLy: string | undefined;
  thoiDiemXongDuKien: number;
}
function taoHeThongTranscode(): HeThongTranscode {
  return { hangDoi: [], trangThai: new Map(), dangXuLy: undefined, thoiDiemXongDuKien: 0 };
}

const THOI_GIAN_XU_LY_MS = 300;

function nopVideoMoi(ht: HeThongTranscode, idVideo: string): void {
  ht.hangDoi.push(idVideo);
  ht.trangThai.set(idVideo, "cho_xu_ly");
}

function tienTrinhXuLy(ht: HeThongTranscode, dh: DongHoMoPhong): void {
  if (ht.dangXuLy === undefined && ht.hangDoi.length > 0) {
    const idVideo = ht.hangDoi.shift()!;
    ht.dangXuLy = idVideo;
    ht.trangThai.set(idVideo, "dang_xu_ly");
    ht.thoiDiemXongDuKien = dh.thoiGianHienTai + THOI_GIAN_XU_LY_MS;
    return;
  }
  ___
}

const dh = taoDongHoMoPhong();
const ht = taoHeThongTranscode();
nopVideoMoi(ht, "v1");
tienTrinhXuLy(ht, dh);
tienThoiGian(dh, 300);
tienTrinhXuLy(ht, dh);
console.log(ht.trangThai.get("v1"));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiVideo = "cho_xu_ly" | "dang_xu_ly" | "hoan_tat";

interface HeThongTranscode {
  hangDoi: string[];
  trangThai: Map<string, TrangThaiVideo>;
  dangXuLy: string | undefined;
  thoiDiemXongDuKien: number;
}
function taoHeThongTranscode(): HeThongTranscode {
  return { hangDoi: [], trangThai: new Map(), dangXuLy: undefined, thoiDiemXongDuKien: 0 };
}

const THOI_GIAN_XU_LY_MS = 300;

function nopVideoMoi(ht: HeThongTranscode, idVideo: string): void {
  ht.hangDoi.push(idVideo);
  ht.trangThai.set(idVideo, "cho_xu_ly");
}

function tienTrinhXuLy(ht: HeThongTranscode, dh: DongHoMoPhong): void {
  if (ht.dangXuLy === undefined && ht.hangDoi.length > 0) {
    const idVideo = ht.hangDoi.shift()!;
    ht.dangXuLy = idVideo;
    ht.trangThai.set(idVideo, "dang_xu_ly");
    ht.thoiDiemXongDuKien = dh.thoiGianHienTai + THOI_GIAN_XU_LY_MS;
    return;
  }
  if (ht.dangXuLy !== undefined && dh.thoiGianHienTai >= ht.thoiDiemXongDuKien) {
    ht.trangThai.set(ht.dangXuLy, "hoan_tat");
    ht.dangXuLy = undefined;
  }
}

const dh = taoDongHoMoPhong();
const ht = taoHeThongTranscode();
nopVideoMoi(ht, "v1");
tienTrinhXuLy(ht, dh);
tienThoiGian(dh, 300);
tienTrinhXuLy(ht, dh);
console.log(ht.trangThai.get("v1"));
```

```typescript title=test
const dhT = taoDongHoMoPhong();
const htT = taoHeThongTranscode();
nopVideoMoi(htT, "v1");
if (htT.trangThai.get("v1") !== "cho_xu_ly") throw new Error("video vua nop phai o trang thai cho_xu_ly");

tienTrinhXuLy(htT, dhT);
if (htT.trangThai.get("v1") !== "dang_xu_ly") throw new Error("worker ranh, goi 1 lan phai nhan viec ngay");
if (htT.dangXuLy !== "v1") throw new Error("dangXuLy phai la v1");

tienThoiGian(dhT, 299);
tienTrinhXuLy(htT, dhT);
if (htT.trangThai.get("v1") !== "dang_xu_ly") throw new Error("t=299 (< 300ms xu ly) van phai dang_xu_ly, CHUA hoan_tat");

tienThoiGian(dhT, 1);
tienTrinhXuLy(htT, dhT);
if (htT.trangThai.get("v1") !== "hoan_tat") throw new Error("t=300 (dung THOI_GIAN_XU_LY_MS) phai hoan_tat");
if (htT.dangXuLy !== undefined) throw new Error("sau khi hoan tat, worker phai duoc giai phong (dangXuLy = undefined)");

const dh2 = taoDongHoMoPhong();
const ht2 = taoHeThongTranscode();
nopVideoMoi(ht2, "v1");
nopVideoMoi(ht2, "v2");
tienTrinhXuLy(ht2, dh2);
if (ht2.trangThai.get("v2") !== "cho_xu_ly") throw new Error("v2 phai VAN cho_xu_ly khi v1 dang duoc xu ly (chi 1 worker)");

tienThoiGian(dh2, 300);
tienTrinhXuLy(ht2, dh2);
if (ht2.trangThai.get("v1") !== "hoan_tat") throw new Error("v1 phai hoan_tat sau 300ms");
if (ht2.trangThai.get("v2") !== "cho_xu_ly") throw new Error("NGAY sau 1 lan goi tai t=300, v2 phai VAN cho_xu_ly (moi mot buoc/mot lan goi)");

tienTrinhXuLy(ht2, dh2);
if (ht2.trangThai.get("v2") !== "dang_xu_ly") throw new Error("goi THEM 1 lan nua (van t=300), worker vua ranh phai nhan v2");

tienThoiGian(dh2, 300);
tienTrinhXuLy(ht2, dh2);
if (ht2.trangThai.get("v2") !== "hoan_tat") throw new Error("v2 phai hoan_tat sau khi worker xu ly du 300ms");
```

:::hints
- kind: attention
  body: "Nhanh thu hai chi chay KHI dangXuLy dang co video (khac undefined) VA da toi/vuot qua thoiDiemXongDuKien -- luc do moi danh dau hoan_tat VA giai phong worker."
- kind: strategy
  body: "Dieu kien: ht.dangXuLy !== undefined VA dh.thoiGianHienTai >= ht.thoiDiemXongDuKien. Neu dung: set trangThai cua video do thanh hoan_tat, roi dat dangXuLy = undefined."
- kind: one-line
  body: "if (ht.dangXuLy !== undefined && dh.thoiGianHienTai >= ht.thoiDiemXongDuKien) { ht.trangThai.set(ht.dangXuLy, \"hoan_tat\"); ht.dangXuLy = undefined; }"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "hoan_tat"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Video giờ đi đúng qua hàng đợi transcode. Nhưng "xong" không hề LÀ MỘT
file — nó phải sinh RA nhiều bản độ phân giải KHÁC nhau.
::::

::::reflect{#nghi-lai}
`tienTrinhXuLy` chỉ CÓ hai nhánh `if` — nhưng thứ TỰ của chúng quyết
định hành vi: đặt nhánh "nhận việc" TRƯỚC nhánh "đóng việc" nghĩa LÀ
worker không bao GIỜ nhận VÀ đóng trong CÙNG một lần gọi. Đây LÀ một
lựa chọn thiết KẾ có chủ đích, không phải NGẪU nhiên — nó giữ MỖI lần
gọi làm đúng MỘT việc, dễ suy LUẬN hơn hẳn.
::::

::::checkpoint{mastery=0.77}
::::
