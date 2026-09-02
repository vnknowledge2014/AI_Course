---
id: ky-nghe-phan-mem.mau-tai-cau-truc.middleware-la-composition-pipeline
title: "Middleware = Composition Pipeline — Express middleware THỰC CHẤT là gì"
summary: "\"Middleware\" (Express và tương tự) THỰC CHẤT LÀ composition hàm: MỖI middleware LÀ MỘT hàm nhận state, trả state đã biến đổi. Chạy chuỗi middleware = pipe(mw1, mw2, mw3). taoPipeline<T>(...cacBuoc): (t:T)=>T tự viết bằng reduce — mỗi \"middleware\" là một phần tử mảng, chạy TUẦN TỰ theo thứ tự truyền vào."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 17
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.middleware-as-pipeline]
requires: [mau.gate-boss-visitor-decorator]
concepts: [mau.middleware-as-pipeline]
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
Cụm mới. Express xử lý MỘT request qua NHIỀU "middleware" liên tiếp
(validate → auth → log). "Middleware" THỰC CHẤT LÀ gì?
::::

::::explain{#middleware-la-composition}
"Middleware" THỰC CHẤT LÀ **composition hàm**: MỖI middleware LÀ MỘT
hàm NHẬN state (request), TRẢ VỀ state ĐÃ biến đổi — chạy CHUỖI
middleware CHÍNH LÀ `pipe(mw1, mw2, mw3)` (đã học T4.2). TỰ VIẾT
`taoPipeline<T>(...cacBuoc)` bằng `reduce` (KHÔNG dùng Express THẬT):

```typescript title=readonly
function taoPipeline<T>(...cacBuoc: Array<(t: T) => T>): (t: T) => T {
  return (t: T) => cacBuoc.reduce((giaTri, buoc) => buoc(giaTri), t);
}

type YeuCau = { duLieu: string; daXacThuc: boolean; nhatKy: string[] };

const chuanHoaDuLieu = (yc: YeuCau): YeuCau => ({ ...yc, duLieu: yc.duLieu.trim().toLowerCase() });
const xacThuc = (yc: YeuCau): YeuCau => ({ ...yc, daXacThuc: true });
const themNhatKy = (buoc: string) => (yc: YeuCau): YeuCau => ({ ...yc, nhatKy: [...yc.nhatKy, buoc] });

const pipeline = taoPipeline(chuanHoaDuLieu, xacThuc, themNhatKy("hoan tat"));

const ketQua = pipeline({ duLieu: "  Xin Chao  ", daXacThuc: false, nhatKy: [] });
console.log(ketQua.duLieu);
console.log(ketQua.daXacThuc);
console.log(ketQua.nhatKy);
```

```text title=readonly
xin chao
true
["hoan tat"]
```

MỖI "middleware" (`chuanHoaDuLieu`, `xacThuc`, `themNhatKy("hoan tat")`)
LÀ MỘT phần tử trong mảng `cacBuoc` — `reduce` chạy TUẦN TỰ, TRUYỀN
KẾT QUẢ của bước TRƯỚC LÀM đầu vào bước SAU. KHÔNG framework, KHÔNG
`app.use(...)` — CHỈ MỘT hàm `reduce` quen thuộc.
::::

::::example{#thu-tu-quyet-dinh-ket-qua}
GIỐNG `apDungLenh`/`reduce` (bài 10), **THỨ TỰ** truyền middleware
QUYẾT ĐỊNH kết quả — ĐỔI thứ tự, ĐỔI kết quả:

```typescript title=readonly
function taoPipeline<T>(...cacBuoc: Array<(t: T) => T>): (t: T) => T {
  return (t: T) => cacBuoc.reduce((giaTri, buoc) => buoc(giaTri), t);
}

const congNam = (n: number) => n + 5;
const nhanHai = (n: number) => n * 2;

const p1 = taoPipeline(congNam, nhanHai);
const p2 = taoPipeline(nhanHai, congNam);

console.log(p1(10));
console.log(p2(10));
```

```text title=readonly
30
25
```

`p1`: `(10 + 5) * 2 = 30`. `p2`: `(10 * 2) + 5 = 25`. CÙNG HAI bước,
CÙNG đầu vào (`10`) — CHỈ đổi THỨ TỰ truyền cho `taoPipeline` ĐÃ đổi
HẲN kết quả. Đây LÀ lý do THỨ TỰ middleware (validate TRƯỚC hay auth
TRƯỚC?) LUÔN quan trọng trong THỰC TẾ.
::::

::::predict{#doan-doi-thu-tu-buoc commitOnce}
```typescript
function taoPipeline<T>(...cacBuoc: Array<(t: T) => T>): (t: T) => T {
  return (t: T) => cacBuoc.reduce((giaTri, buoc) => buoc(giaTri), t);
}

const tru3 = (n: number) => n - 3;
const nhanMuoi = (n: number) => n * 10;

const p = taoPipeline(tru3, nhanMuoi, tru3);
console.log(p(20));
```

Dòng cuối in ra gì?

:::opt{correct}
`167`
:::

:::opt
`170` — vì `tru3` xuất hiện HAI LẦN trong pipeline, VÀ (do CÙNG một
hàm) `reduce` chỉ ÁP DỤNG nó **MỘT LẦN DUY NHẤT** (bỏ qua lần lặp
lại), coi như pipeline THỰC TẾ chỉ có `[tru3, nhanMuoi]`
::why
Gần đúng ở việc bạn để ý `tru3` XUẤT HIỆN HAI LẦN trong `cacBuoc` —
một quan sát ĐÚNG về DANH SÁCH truyền vào.

Chỗ lệch: `reduce` KHÔNG "khử trùng lặp" các PHẦN TỬ THAM CHIẾU TỚI
CÙNG một hàm — nó CHẠY QUA **TỪNG VỊ TRÍ** trong mảng, BẤT KỂ giá trị
Ở đó có "TRÙNG" với vị trí KHÁC hay không. `p(20)`: `tru3` (`20-3=17`)
→ `nhanMuoi` (`17*10=170`) → `tru3` **LẦN THỨ HAI** (`170-3=167`) —
`tru3` CHẠY ĐỦ HAI LẦN, ĐÚNG theo SỐ LẦN nó xuất hiện trong mảng.
::
:::

:::opt
Máy báo lỗi biên dịch — `taoPipeline` nhận tham số `...cacBuoc` (rest
parameter), TypeScript CẤM truyền CÙNG một GIÁ TRỊ hàm (`tru3`) NHIỀU
HƠN MỘT LẦN qua rest parameter
::why
Gần đúng ở việc bạn để ý `tru3` được TRUYỀN hai LẦN Ở lời gọi
`taoPipeline(tru3, nhanMuoi, tru3)` — một quan sát ĐÚNG về CÚ PHÁP
gọi hàm.

Chỗ lệch: rest parameter (`...cacBuoc: Array<(t:T)=>T>`) CHỈ đòi MỖI
phần tử khớp KIỂU (`(t:T)=>T`) — HOÀN TOÀN KHÔNG có quy tắc "cấm
trùng giá trị". Truyền CÙNG một hàm NHIỀU lần LÀ hợp lệ (VÀ ĐÔI KHI
CÓ ÍCH, như minh hoạ Ở TRÊN: áp dụng MỘT phép biến đổi HAI LẦN Ở hai
GIAI ĐOẠN khác nhau). Biên dịch SẠCH.
::
:::
::::

::::code{#viet_pipeline}
Hoàn thiện `taoPipeline` — chạy TUẦN TỰ MỌI bước, TRUYỀN kết quả bước
TRƯỚC làm đầu vào bước SAU.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function taoPipeline<T>(...cacBuoc: Array<(t: T) => T>): (t: T) => T {
  return (t: T) => ___;
}

const congNam = (n: number) => n + 5;
const nhanHai = (n: number) => n * 2;
const pipeline = taoPipeline(congNam, nhanHai);
assertEqual(pipeline(10), 30, "cong 5 roi nhan 2");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function taoPipeline<T>(...cacBuoc: Array<(t: T) => T>): (t: T) => T {
  return (t: T) => cacBuoc.reduce((giaTri, buoc) => buoc(giaTri), t);
}

const congNam = (n: number) => n + 5;
const nhanHai = (n: number) => n * 2;
const pipeline = taoPipeline(congNam, nhanHai);
assertEqual(pipeline(10), 30, "cong 5 roi nhan 2");
```

```typescript title=test
const pipelineNguoc = taoPipeline(nhanHai, congNam);
assertEqual(pipelineNguoc(10), 25, "nhan 2 roi cong 5 -- thu tu nguoc lai");
const pipelineBaBuoc = taoPipeline(congNam, nhanHai, congNam);
assertEqual(pipelineBaBuoc(1), 17, "ba buoc: (1+5)*2+5 = 17");
const pipelineRong = taoPipeline<number>();
assertEqual(pipelineRong(99), 99, "pipeline rong tra ve nguyen gia tri dau vao");
```

:::hints
- kind: attention
  body: "Dùng cacBuoc.reduce — giá trị TÍCH LŨY bắt đầu từ t, mỗi bước gọi hàm buoc với giá trị hiện tại."
- kind: strategy
  body: "cacBuoc.reduce((giaTri, buoc) => buoc(giaTri), t) — reduce với giá trị khởi tạo t, mỗi bước ÁP DỤNG hàm lên giá trị tích luỹ."
- kind: one-line
  body: '___ = cacBuoc.reduce((giaTri, buoc) => buoc(giaTri), t)'
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
Middleware = pipeline hàm, chạy TUẦN TỰ theo thứ tự truyền vào. Bài
tiếp theo: một kỹ thuật tái cấu trúc PHỔ BIẾN — cắt hàm dài.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`taoPipeline` GHÉP nhiều hàm NHỎ thành MỘT chuỗi. Nếu MỘT hàm ĐƠN LẺ
đã DÀI (làm nhiều việc TRỘN LẪN trong MỘT khối), bước ĐẦU TIÊN để
"chia" nó thành nhiều bước NHỎ (giống các middleware Ở TRÊN) LÀ gì?
::::

::::checkpoint{mastery=0.8}
::::
