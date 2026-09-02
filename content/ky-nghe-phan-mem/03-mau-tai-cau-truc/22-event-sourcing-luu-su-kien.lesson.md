---
id: ky-nghe-phan-mem.mau-tai-cau-truc.event-sourcing-luu-su-kien
title: "Event Sourcing — lưu SỰ KIỆN thay vì lưu TRẠNG THÁI"
summary: "Thay vì lưu \"số dư HIỆN TẠI\" (ghi ĐÈ mỗi lần đổi, MẤT lịch sử), Event Sourcing lưu TOÀN BỘ chuỗi sự kiện ĐÃ xảy ra — trạng thái HIỆN TẠI được TÍNH RA bằng reduce qua TOÀN BỘ chuỗi (giống sổ kế toán: KHÔNG xoá giao dịch cũ, CHỈ CỘNG THÊM giao dịch mới). tinhSoDu(cacSuKien): number — 'trạng thái' là KẾT QUẢ của một PHÉP TÍNH, không phải dữ liệu lưu trực tiếp."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 22
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.event-sourcing-basics]
requires: [mau.cqrs-read-write-split]
concepts: [mau.event-sourcing-basics]
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
Tài khoản ngân hàng: lưu "số dư = 500" (ghi ĐÈ mỗi lần đổi) HAY lưu
TOÀN BỘ lịch sử giao dịch (nạp/rút)? Sổ kế toán THẬT chọn cách nào?
::::

::::explain{#luu-su-kien-khong-luu-trang-thai}
Thay vì lưu "số dư HIỆN TẠI" (ghi ĐÈ MỖI lần đổi, MẤT lịch sử), Event
Sourcing lưu **TOÀN BỘ chuỗi sự kiện ĐÃ xảy ra** — GIỐNG chuẩn giao
sổ kế toán: KHÔNG xoá giao dịch CŨ, CHỈ **CỘNG THÊM** giao dịch MỚI.
Trạng thái HIỆN TẠI được **TÍNH RA** bằng cách `reduce` qua TOÀN BỘ
chuỗi:

```typescript title=readonly
type SuKien = { tag: "daNap"; soTien: number } | { tag: "daRut"; soTien: number };

function tinhSoDu(cacSuKien: SuKien[]): number {
  return cacSuKien.reduce((soDu, sk) => {
    switch (sk.tag) {
      case "daNap": return soDu + sk.soTien;
      case "daRut": return soDu - sk.soTien;
    }
  }, 0);
}

const cacSuKien: SuKien[] = [
  { tag: "daNap", soTien: 300 },
  { tag: "daRut", soTien: 100 },
  { tag: "daNap", soTien: 50 },
];
console.log(tinhSoDu(cacSuKien));
```

```text title=readonly
250
```

KHÔNG CÓ biến "số dư" nào được LƯU TRỰC TIẾP — `cacSuKien` LÀ nguồn
SỰ THẬT DUY NHẤT (single source of truth), `tinhSoDu` **TÍNH LẠI**
`250` (`300 - 100 + 50`) TỪ ĐẦU MỖI khi được gọi. "Trạng thái" KHÔNG
PHẢI dữ liệu LƯU TRỰC TIẾP — nó LÀ KẾT QUẢ của MỘT PHÉP TÍNH.
::::

::::example{#truy-van-thoi-diem-bat-ky}
Vì sự kiện LÀ nguồn SỰ THẬT, `tinhSoDu` áp dụng được cho **BẤT KỲ**
ĐOẠN ĐẦU nào của chuỗi — TRUY VẤN được số dư tại BẤT KỲ THỜI ĐIỂM
nào TRONG QUÁ KHỨ, KHÔNG CẦN lưu "snapshot" nào cả:

```typescript title=readonly
type SuKien = { tag: "daNap"; soTien: number } | { tag: "daRut"; soTien: number };
function tinhSoDu(cacSuKien: SuKien[]): number {
  return cacSuKien.reduce((soDu, sk) => {
    switch (sk.tag) {
      case "daNap": return soDu + sk.soTien;
      case "daRut": return soDu - sk.soTien;
    }
  }, 0);
}
const cacSuKien: SuKien[] = [
  { tag: "daNap", soTien: 300 },
  { tag: "daRut", soTien: 100 },
  { tag: "daNap", soTien: 50 },
];
console.log(tinhSoDu(cacSuKien.slice(0, 1)));
console.log(tinhSoDu(cacSuKien.slice(0, 2)));
console.log(tinhSoDu(cacSuKien));
```

```text title=readonly
300
200
250
```

`slice(0, 1)`: số dư NGAY SAU sự kiện ĐẦU TIÊN (`300`). `slice(0, 2)`:
số dư SAU HAI sự kiện ĐẦU (`300 - 100 = 200`). Đây LÀ "TRUY VẤN THEO
THỜI GIAN" (temporal query) — MỘT khả năng KHÔNG THỂ CÓ nếu CHỈ lưu
"số dư HIỆN TẠI" (ghi ĐÈ mất LỊCH SỬ).
::::

::::predict{#doan-du-am-giua-chung commitOnce}
```typescript
type SuKien = { tag: "daNap"; soTien: number } | { tag: "daRut"; soTien: number };
function tinhSoDu(cacSuKien: SuKien[]): number {
  return cacSuKien.reduce((soDu, sk) => {
    switch (sk.tag) {
      case "daNap": return soDu + sk.soTien;
      case "daRut": return soDu - sk.soTien;
    }
  }, 0);
}
const cacSuKien: SuKien[] = [
  { tag: "daNap", soTien: 100 },
  { tag: "daRut", soTien: 250 },
  { tag: "daNap", soTien: 500 },
];
console.log(tinhSoDu(cacSuKien));
```

Số dư SAU sự kiện THỨ HAI LÀ `100 - 250 = -150` (ÂM). Dòng cuối in
ra gì?

:::opt{correct}
`350`
:::

:::opt
Máy báo lỗi lúc chạy — số dư ÂM (`-150`) SAU sự kiện thứ hai LÀ MỘT
trạng thái KHÔNG hợp lệ cho tài khoản ngân hàng, `reduce` NÉM lỗi
NGAY khi phát hiện GIÁ TRỊ tích luỹ ÂM
::why
Gần đúng ở việc bạn tính ĐÚNG số dư TRUNG GIAN SAU sự kiện thứ hai LÀ
`-150` — phép TRỪ đúng.

Chỗ lệch: `tinhSoDu` (VÀ `reduce` NÓI CHUNG) **KHÔNG kiểm tra** giá
trị tích luỹ có ÂM hay không — nó ĐƠN GIẢN áp dụng phép tính CHO TỪNG
sự kiện, BẤT KỂ kết quả TRUNG GIAN LÀ GÌ. `-150` CHỈ LÀ một con số
TRUNG GIAN, KHÔNG gây lỗi gì — sự kiện THỨ BA (`daNap 500`) CHẠY TIẾP
BÌNH THƯỜNG, CỘNG THÊM vào số dư ÂM đó: `350 = 500 - 150`. (Việc "CẤM
số dư ÂM" — nếu CẦN — LÀ
việc của WRITE model lúc VALIDATE lệnh rút TRƯỚC khi sinh sự kiện,
KHÔNG PHẢI việc của `tinhSoDu`.)
::
:::

:::opt
`350` nhưng CHỈ khi CHẠY trên trình duyệt web — trên môi trường KHÁC
(như Node.js), `reduce` xử lý số ÂM TRUNG GIAN KHÁC ĐI, cho kết quả
KHÁC
::why
Gần đúng ở việc bạn nghĩ TỚI khả năng HÀNH VI JavaScript có THỂ khác
nhau GIỮA các môi trường — một mối lo hợp lý TRONG một số trường hợp
đặc thù (ví dụ: định dạng ngày tháng, timezone).

Chỗ lệch: PHÉP CỘNG/TRỪ số học (`+`, `-`) trên kiểu `number` LÀ HÀNH
VI **CHUẨN ECMAScript**, THỰC THI GIỐNG HỆT nhau TRÊN MỌI môi trường
JavaScript tuân chuẩn (trình duyệt, Node.js, Deno, ...) — KHÔNG có sự
khác biệt nào Ở ĐÂY. Kết quả `350` LÀ CỐ ĐỊNH, KHÔNG phụ thuộc môi
trường chạy.
::
:::
::::

::::code{#viet_tinh_so_du}
Hoàn thiện `tinhSoDu` — `reduce` qua chuỗi sự kiện, cộng khi nạp, trừ
khi rút.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type SuKien = { tag: "daNap"; soTien: number } | { tag: "daRut"; soTien: number };

function tinhSoDu(cacSuKien: SuKien[]): number {
  return cacSuKien.reduce((soDu, sk) => {
    switch (sk.tag) {
      case "daNap": return ___;
      case "daRut": return ___;
    }
  }, 0);
}

assertEqual(tinhSoDu([{ tag: "daNap", soTien: 100 }]), 100, "mot lan nap");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type SuKien = { tag: "daNap"; soTien: number } | { tag: "daRut"; soTien: number };

function tinhSoDu(cacSuKien: SuKien[]): number {
  return cacSuKien.reduce((soDu, sk) => {
    switch (sk.tag) {
      case "daNap": return soDu + sk.soTien;
      case "daRut": return soDu - sk.soTien;
    }
  }, 0);
}

assertEqual(tinhSoDu([{ tag: "daNap", soTien: 100 }]), 100, "mot lan nap");
```

```typescript title=test
assertEqual(tinhSoDu([{ tag: "daRut", soTien: 30 }]), -30, "mot lan rut tu 0 -- am");
assertEqual(tinhSoDu([
  { tag: "daNap", soTien: 300 },
  { tag: "daRut", soTien: 100 },
  { tag: "daNap", soTien: 50 },
]), 250, "nhieu su kien lien tiep");
assertEqual(tinhSoDu([]), 0, "khong su kien nao -- so du 0");
```

:::hints
- kind: attention
  body: "daNap: CỘNG sk.soTien vào soDu. daRut: TRỪ sk.soTien khỏi soDu."
- kind: strategy
  body: "soDu + sk.soTien : soDu - sk.soTien — cộng khi nạp, trừ khi rút."
- kind: one-line
  body: '___ (daNap) = soDu + sk.soTien\n___ (daRut) = soDu - sk.soTien'
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
Event Sourcing: lưu sự kiện, TÍNH trạng thái bằng reduce. Bài tiếp
theo: MỘT chuỗi sự kiện, NHIỀU cách "chiếu" thành view khác nhau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tinhSoDu` LÀ MỘT phép "chiếu" (projection) TỪ sự kiện SANG một con
số. Nếu CẦN NHIỀU phép chiếu KHÁC NHAU TỪ CÙNG một chuỗi sự kiện
(vừa số dư, vừa LỊCH SỬ giao dịch dạng bảng) — có cần lưu THÊM dữ
liệu nào KHÔNG?
::::

::::checkpoint{mastery=0.8}
::::
