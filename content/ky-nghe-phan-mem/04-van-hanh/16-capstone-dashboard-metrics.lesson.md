---
id: ky-nghe-phan-mem.van-hanh.capstone-dashboard-metrics
title: "Capstone cụm — ghép Counter + tỷ lệ lỗi + percentile thành Dashboard"
summary: "taoDashboard(nguongLoi): {ghiRequest, baoCao} — ghi MỘT request (thời gian + có lỗi hay không) đồng thời cập nhật CẢ BA công cụ (taoCounter bài 13, tinhTyLeLoi bài 14, tinhPercentile bài 15); baoCao() TỔNG HỢP thành MỘT bức tranh: tổng request, tỷ lệ lỗi, p50/p90, có vượt ngưỡng."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 16
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [vh.gate-boss-metrics]
requires: [vh.latency-percentile]
concepts: [vh.gate-boss-metrics]
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
Ba công cụ đứng RIÊNG (bài 13-15). Bài chốt cụm: gộp CẢ BA thành
MỘT dashboard, gọi ĐÚNG một lần cho MỖI request.
::::

::::explain{#dashboard-hop-nhat}
`taoDashboard(nguongLoi)` ghi MỘT request VÀO **CẢ BA** công cụ CÙNG
lúc — `baoCao()` đọc RA một bức TRANH tổng hợp:

```typescript title=readonly
function taoCounter(ten: string): { ten: string; tang: () => void; giaTri: () => number } {
  let dem = 0;
  return { ten, tang: () => { dem += 1; }, giaTri: () => dem };
}
function tinhTyLeLoi(soLoi: number, soTong: number): number {
  if (soTong === 0) return 0;
  return soLoi / soTong;
}
function tinhPercentile(dsThoiGian: number[], p: number): number {
  const daSapXep = [...dsThoiGian].sort((a, b) => a - b);
  const n = daSapXep.length;
  const chiSo = Math.min(n - 1, Math.max(0, Math.ceil((p / 100) * n) - 1));
  return daSapXep[chiSo]!;
}

type BaoCao = { tongRequest: number; tyLeLoi: number; p50: number; p90: number; vuotNguong: boolean };

function taoDashboard(nguongLoi: number): {
  ghiRequest: (thoiGianMs: number, coLoi: boolean) => void;
  baoCao: () => BaoCao;
} {
  const requestCounter = taoCounter("requests");
  const errorCounter = taoCounter("errors");
  const dsThoiGian: number[] = [];
  return {
    ghiRequest: (thoiGianMs, coLoi) => {
      requestCounter.tang();
      if (coLoi) errorCounter.tang();
      dsThoiGian.push(thoiGianMs);
    },
    baoCao: () => {
      const tongRequest = requestCounter.giaTri();
      const tyLeLoi = tinhTyLeLoi(errorCounter.giaTri(), tongRequest);
      return {
        tongRequest,
        tyLeLoi,
        p50: dsThoiGian.length === 0 ? 0 : tinhPercentile(dsThoiGian, 50),
        p90: dsThoiGian.length === 0 ? 0 : tinhPercentile(dsThoiGian, 90),
        vuotNguong: tyLeLoi > nguongLoi,
      };
    },
  };
}

const dash = taoDashboard(0.1);
const times = [200, 50, 900, 120, 300, 80, 450, 100, 600, 150];
const loi = [false, false, true, false, false, false, true, false, false, false];
for (let i = 0; i < times.length; i++) dash.ghiRequest(times[i]!, loi[i]!);
console.log(dash.baoCao());
```

```text title=readonly
{"tongRequest":10,"tyLeLoi":0.2,"p50":150,"p90":600,"vuotNguong":true}
```

MỖI lần `ghiRequest` gọi, `requestCounter` tăng LUÔN, `errorCounter`
CHỈ tăng khi `coLoi` LÀ `true`, `dsThoiGian` LUÔN nhận thêm phần TỬ
(BẤT KỂ có lỗi hay KHÔNG). `baoCao()` KHÔNG lưu trạng thái RIÊNG —
nó ĐỌC LẠI từ ba công cụ MỖI lần gọi.
::::

::::example{#dashboard-rong}
Dashboard MỚI tạo (CHƯA ghi request NÀO) trả VỀ báo cáo AN TOÀN —
KHÔNG `NaN`, KHÔNG lỗi truy cập phần TỬ rỗng:

```typescript title=readonly
function taoCounter(ten: string): { ten: string; tang: () => void; giaTri: () => number } {
  let dem = 0;
  return { ten, tang: () => { dem += 1; }, giaTri: () => dem };
}
function tinhTyLeLoi(soLoi: number, soTong: number): number {
  if (soTong === 0) return 0;
  return soLoi / soTong;
}
function tinhPercentile(dsThoiGian: number[], p: number): number {
  const daSapXep = [...dsThoiGian].sort((a, b) => a - b);
  const n = daSapXep.length;
  const chiSo = Math.min(n - 1, Math.max(0, Math.ceil((p / 100) * n) - 1));
  return daSapXep[chiSo]!;
}
type BaoCao = { tongRequest: number; tyLeLoi: number; p50: number; p90: number; vuotNguong: boolean };
function taoDashboard(nguongLoi: number): {
  ghiRequest: (thoiGianMs: number, coLoi: boolean) => void;
  baoCao: () => BaoCao;
} {
  const requestCounter = taoCounter("requests");
  const errorCounter = taoCounter("errors");
  const dsThoiGian: number[] = [];
  return {
    ghiRequest: (thoiGianMs, coLoi) => {
      requestCounter.tang();
      if (coLoi) errorCounter.tang();
      dsThoiGian.push(thoiGianMs);
    },
    baoCao: () => {
      const tongRequest = requestCounter.giaTri();
      const tyLeLoi = tinhTyLeLoi(errorCounter.giaTri(), tongRequest);
      return {
        tongRequest,
        tyLeLoi,
        p50: dsThoiGian.length === 0 ? 0 : tinhPercentile(dsThoiGian, 50),
        p90: dsThoiGian.length === 0 ? 0 : tinhPercentile(dsThoiGian, 90),
        vuotNguong: tyLeLoi > nguongLoi,
      };
    },
  };
}
console.log(taoDashboard(0.1).baoCao());
```

```text title=readonly
{"tongRequest":0,"tyLeLoi":0,"p50":0,"p90":0,"vuotNguong":false}
```

`tinhTyLeLoi` (guard bài 14) NGĂN `NaN` khi `tongRequest` LÀ `0`.
Riêng `tinhPercentile` (bài 15) KHÔNG TỰ guard mảng rỗng — `taoDashboard`
PHẢI kiểm TRA `dsThoiGian.length === 0` TRƯỚC KHI gọi nó, NẾU không
`daSapXep[-1]` sẽ trả VỀ `undefined`.
::::

::::predict{#doan-request-loi-co-tinh-vao-latency commitOnce}
```typescript
const d = taoDashboard(1);
d.ghiRequest(100, false);
d.ghiRequest(100, false);
d.ghiRequest(100, false);
d.ghiRequest(100, false);
d.ghiRequest(900, true);
console.log(d.baoCao().p90);
```

BỐN request THÀNH CÔNG mất `100`ms, MỘT request **LỖI** (`coLoi:
true`) mất `900`ms. Dòng cuối in ra gì?

:::opt{correct}
`900`
:::

:::opt
`100` — vì request LỖI (`coLoi: true`) KHÔNG PHẢI trải nghiệm THÀNH
CÔNG của người DÙNG, NÊN `ghiRequest` chỉ đưa thời GIAN của request
ĐÓ vào `dsThoiGian` khi `coLoi` LÀ `false`, percentile CHỈ phản ánh
BỐN request thành CÔNG (toàn `100`ms)
::why
Gần đúng ở việc bạn nghĩ tới sự KHÁC BIỆT hợp LÝ giữa "trải nghiệm
THÀNH CÔNG" VÀ "request lỗi" — latency THƯỜNG được diễn giải NHƯ
tốc độ phục vụ NGƯỜI DÙNG, NÊN tách RIÊNG nghe CÓ lý.

Chỗ lệch: NHÌN lại `ghiRequest` (phần explain) — dòng
`dsThoiGian.push(thoiGianMs)` nằm **NGOÀI** khối `if (coLoi)`, chạy
**VÔ ĐIỀU KIỆN** cho MỌI request, THÀNH công LẪN lỗi. Đây LÀ lựa
CHỌN THIẾT KẾ có CHỦ Ý: request lỗi (VÍ DỤ timeout SAU 900ms rồi
mới trả lỗi) VẪN chiếm THỜI GIAN xử lý THẬT — bỏ SÓT nó khỏi
percentile SẼ khiến p90 trông "đẹp" hơn THỰC TẾ, che giấu ĐÚNG loại
vấn đề mà dashboard cần LỘ ra. p90 CỦA năm giá trị `[100,100,100,
100,900]` LÀ giá trị LỚN NHẤT: `900`.
::
:::

:::opt
Máy báo lỗi biên dịch — `taoDashboard(1)` truyền `nguongLoi = 1`
(100%), NHƯNG kiểu `number` cho tham số NÀY chỉ CHẤP NHẬN giá trị
TRONG khoảng `[0, 1)`, TypeScript CẤM giá trị `1` chẵn
::why
Gần đúng ở việc bạn để ý `nguongLoi = 1` LÀ giá trị BIÊN (100%,
"không BAO GIỜ vượt ngưỡng") — một quan sát HỢP LÝ về Ý NGHĨA con
số NÀY.

Chỗ lệch: `nguongLoi: number` LÀ kiểu `number` THÔNG THƯỜNG, KHÔNG
CÓ ràng buộc khoảng giá TRỊ nào ở MỨC kiểu (TypeScript KHÔNG CÓ
"khoảng số" NHƯ VẬY) — TRUYỀN `1`, `0`, `2.5`, HAY BẤT KỲ số NÀO
ĐỀU biên dịch SẠCH.
::
:::
::::

::::code{#viet_tao_dashboard}
Hoàn thiện `taoDashboard` — MỖI request cập nhật CẢ BA công cụ,
`baoCao` tổng hợp LẠI.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function taoCounter(ten: string): { ten: string; tang: () => void; giaTri: () => number } {
  let dem = 0;
  return { ten, tang: () => { dem += 1; }, giaTri: () => dem };
}
function tinhTyLeLoi(soLoi: number, soTong: number): number {
  if (soTong === 0) return 0;
  return soLoi / soTong;
}
function tinhPercentile(dsThoiGian: number[], p: number): number {
  const daSapXep = [...dsThoiGian].sort((a, b) => a - b);
  const n = daSapXep.length;
  const chiSo = Math.min(n - 1, Math.max(0, Math.ceil((p / 100) * n) - 1));
  return daSapXep[chiSo]!;
}

type BaoCao = { tongRequest: number; tyLeLoi: number; p50: number; p90: number; vuotNguong: boolean };

function taoDashboard(nguongLoi: number): {
  ghiRequest: (thoiGianMs: number, coLoi: boolean) => void;
  baoCao: () => BaoCao;
} {
  const requestCounter = taoCounter("requests");
  const errorCounter = taoCounter("errors");
  const dsThoiGian: number[] = [];
  return {
    ghiRequest: (thoiGianMs, coLoi) => {
      requestCounter.tang();
      if (coLoi) ___;
      dsThoiGian.push(thoiGianMs);
    },
    baoCao: () => {
      const tongRequest = requestCounter.giaTri();
      const tyLeLoi = tinhTyLeLoi(___, tongRequest);
      return {
        tongRequest,
        tyLeLoi,
        p50: dsThoiGian.length === 0 ? 0 : tinhPercentile(dsThoiGian, 50),
        p90: dsThoiGian.length === 0 ? 0 : tinhPercentile(dsThoiGian, 90),
        vuotNguong: ___,
      };
    },
  };
}

const dash1 = taoDashboard(0.1);
dash1.ghiRequest(100, false);
assertEqual(dash1.baoCao().tongRequest, 1, "ghi mot request");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function taoCounter(ten: string): { ten: string; tang: () => void; giaTri: () => number } {
  let dem = 0;
  return { ten, tang: () => { dem += 1; }, giaTri: () => dem };
}
function tinhTyLeLoi(soLoi: number, soTong: number): number {
  if (soTong === 0) return 0;
  return soLoi / soTong;
}
function tinhPercentile(dsThoiGian: number[], p: number): number {
  const daSapXep = [...dsThoiGian].sort((a, b) => a - b);
  const n = daSapXep.length;
  const chiSo = Math.min(n - 1, Math.max(0, Math.ceil((p / 100) * n) - 1));
  return daSapXep[chiSo]!;
}

type BaoCao = { tongRequest: number; tyLeLoi: number; p50: number; p90: number; vuotNguong: boolean };

function taoDashboard(nguongLoi: number): {
  ghiRequest: (thoiGianMs: number, coLoi: boolean) => void;
  baoCao: () => BaoCao;
} {
  const requestCounter = taoCounter("requests");
  const errorCounter = taoCounter("errors");
  const dsThoiGian: number[] = [];
  return {
    ghiRequest: (thoiGianMs, coLoi) => {
      requestCounter.tang();
      if (coLoi) errorCounter.tang();
      dsThoiGian.push(thoiGianMs);
    },
    baoCao: () => {
      const tongRequest = requestCounter.giaTri();
      const tyLeLoi = tinhTyLeLoi(errorCounter.giaTri(), tongRequest);
      return {
        tongRequest,
        tyLeLoi,
        p50: dsThoiGian.length === 0 ? 0 : tinhPercentile(dsThoiGian, 50),
        p90: dsThoiGian.length === 0 ? 0 : tinhPercentile(dsThoiGian, 90),
        vuotNguong: tyLeLoi > nguongLoi,
      };
    },
  };
}

const dash1 = taoDashboard(0.1);
dash1.ghiRequest(100, false);
assertEqual(dash1.baoCao().tongRequest, 1, "ghi mot request");
```

```typescript title=test
const dash = taoDashboard(0.1);
const times = [200, 50, 900, 120, 300, 80, 450, 100, 600, 150];
const loi = [false, false, true, false, false, false, true, false, false, false];
for (let i = 0; i < times.length; i++) dash.ghiRequest(times[i]!, loi[i]!);
assertEqual(dash.baoCao().tongRequest, 10, "tong request dung");
assertEqual(dash.baoCao().tyLeLoi, 0.2, "ty le loi dung");
assertEqual(dash.baoCao().p50, 150, "p50 dung");
assertEqual(dash.baoCao().p90, 600, "p90 dung");
assertEqual(dash.baoCao().vuotNguong, true, "vuot nguong 10%");

const dashRong = taoDashboard(0.1);
assertEqual(dashRong.baoCao().tongRequest, 0, "dashboard rong -- tong request 0");
assertEqual(dashRong.baoCao().tyLeLoi, 0, "dashboard rong -- ty le loi 0");
assertEqual(dashRong.baoCao().p50, 0, "dashboard rong -- p50 0");
assertEqual(dashRong.baoCao().vuotNguong, false, "dashboard rong -- khong vuot nguong");

const dash2 = taoDashboard(0.5);
dash2.ghiRequest(10, false);
assertEqual(dash2.baoCao().tongRequest, 1, "dashboard doc lap voi dash");
assertEqual(dash.baoCao().tongRequest, 10, "dash khong bi anh huong boi dash2");
```

:::hints
- kind: attention
  body: "ghiRequest: coLoi thi tang errorCounter. baoCao: tyLeLoi doc errorCounter.giaTri(). vuotNguong so sanh tyLeLoi > nguongLoi."
- kind: strategy
  body: "errorCounter.tang() : errorCounter.giaTri() : tyLeLoi > nguongLoi"
- kind: one-line
  body: '___ (tang loi) = errorCounter.tang()\n___ (doc loi) = errorCounter.giaTri()\n___ (vuotNguong) = tyLeLoi > nguongLoi'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "PASS"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn bài, một dashboard hoàn chỉnh: đếm, tỷ lệ, phân vị — tất cả tự
tay dựng, không thư viện. Cụm tiếp theo: theo dõi MỘT request đi
QUA nhiều bước xử lý (tracing).
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`baoCao()` cho biết HỆ THỐNG khoẻ hay KHÔNG bằng SỐ LIỆU tổng hợp.
NHƯNG khi p90 CAO bất thường, "BƯỚC NÀO trong request LÀM CHẬM" —
số liệu tổng hợp CÓ trả lời được KHÔNG?
::::

::::checkpoint{mastery=0.8}
::::
