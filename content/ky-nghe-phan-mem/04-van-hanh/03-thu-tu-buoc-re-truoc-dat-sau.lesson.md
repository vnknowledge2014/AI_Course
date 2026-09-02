---
id: ky-nghe-phan-mem.van-hanh.thu-tu-buoc-re-truoc-dat-sau
title: "Thứ tự bước: RẺ trước, ĐẮT sau — tối ưu thời gian phát hiện lỗi"
summary: "Vì fail-fast dừng NGAY khi gặp bước hỏng, THỨ TỰ các bước quyết định TỐC ĐỘ phát hiện lỗi — đặt bước RẺ (typecheck, vài giây) TRƯỚC bước ĐẮT (build, vài phút): nếu code có lỗi kiểu, pipeline dừng SAU vài giây, không lãng phí vài phút chờ build fail VÌ CÙNG lý do đó. Khi MỌI bước đều qua (happy path), thứ tự không ảnh hưởng số bước chạy."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [vh.step-ordering-cheap-first]
requires: [vh.pipeline-fail-fast]
concepts: [vh.step-ordering-cheap-first]
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
Fail-fast (bài 2) dừng NGAY ở bước đầu hỏng. Vậy ĐẶT bước NÀO trước
— `typecheck` (vài GIÂY) hay `build` (vài PHÚT) — có QUAN TRỌNG
không?
::::

::::explain{#re-truoc-dat-sau}
Vì fail-fast dừng NGAY khi GẶP bước hỏng, **THỨ TỰ** các bước quyết
định TỐC ĐỘ phát hiện lỗi — đặt bước RẺ (`typecheck`, VÀI GIÂY)
TRƯỚC bước ĐẮT (`build`, VÀI PHÚT): NẾU code CÓ lỗi KIỂU, pipeline
dừng SAU vài GIÂY, KHÔNG lãng phí VÀI PHÚT chờ `build` fail VÌ
**CÙNG** lý do ĐÓ:

```typescript title=readonly
type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };
function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) return { qua: false, buocHong: buoc.ten };
  }
  return { qua: true, buocHong: null };
}

type MaNguon = { coLoiKieu: boolean };

function chayThuTu(thuTu: "re-truoc" | "dat-truoc"): number {
  let soLanChay = 0;
  const typecheckRe: BuocKiemTra<MaNguon> = {
    ten: "typecheck",
    kiemTra: (mn) => { soLanChay++; return !mn.coLoiKieu; },
  };
  const buildDat: BuocKiemTra<MaNguon> = {
    ten: "build",
    kiemTra: (mn) => { soLanChay++; return true; },
  };
  const cacBuoc = thuTu === "re-truoc" ? [typecheckRe, buildDat] : [buildDat, typecheckRe];
  chayPipelineCI({ coLoiKieu: true }, ...cacBuoc);
  return soLanChay;
}

console.log(chayThuTu("re-truoc"));
console.log(chayThuTu("dat-truoc"));
```

```text title=readonly
1
2
```

CÙNG một lỗi (`coLoiKieu: true`) — đặt `typecheck` (RẺ) TRƯỚC:
`build` (ĐẮT) KHÔNG BAO GIỜ chạy TỚI (chỉ `1` bước THẬT SỰ chạy).
Đặt `build` TRƯỚC: NÓ chạy XONG (VÔ ÍCH, VÌ pipeline VẪN THẤT BẠI
SAU ĐÓ Ở `typecheck`) — TỔNG `2` bước THẬT SỰ chạy, TỐN THỜI GIAN
HƠN cho CÙNG một kết luận.
::::

::::example{#thu-tu-khong-quan-trong-khi-deu-qua}
KHI MỌI bước ĐỀU qua ("happy path", KHÔNG có lỗi NÀO) — thứ tự
**KHÔNG** ảnh hưởng SỐ bước THẬT SỰ chạy, vì KHÔNG bước NÀO khiến
fail-fast kích HOẠT:

```typescript title=readonly
type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };
function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) return { qua: false, buocHong: buoc.ten };
  }
  return { qua: true, buocHong: null };
}
type MaNguon = { coLoiKieu: boolean };
function chayThuTuThanhCong(thuTu: "re-truoc" | "dat-truoc"): number {
  let soLanChay = 0;
  const typecheckRe: BuocKiemTra<MaNguon> = { ten: "typecheck", kiemTra: (mn) => { soLanChay++; return !mn.coLoiKieu; } };
  const buildDat: BuocKiemTra<MaNguon> = { ten: "build", kiemTra: (mn) => { soLanChay++; return true; } };
  const cacBuoc = thuTu === "re-truoc" ? [typecheckRe, buildDat] : [buildDat, typecheckRe];
  chayPipelineCI({ coLoiKieu: false }, ...cacBuoc);
  return soLanChay;
}
console.log(chayThuTuThanhCong("re-truoc"));
console.log(chayThuTuThanhCong("dat-truoc"));
```

```text title=readonly
2
2
```

CÙNG kết quả `2` DÙ đảo THỨ TỰ — vì `coLoiKieu: false`, CẢ HAI bước
ĐỀU qua, KHÔNG bước NÀO kích HOẠT fail-fast, pipeline LUÔN chạy HẾT.
Tối ưu "rẻ trước, đắt sau" CHỈ có ÍCH khi THỰC SỰ CÓ lỗi — trên
đường "MỌI thứ ĐỀU ổn", thứ tự VÔ NGHĨA.
::::

::::predict{#doan-dao-thu-tu-buoc-hong-len-dau commitOnce}
```typescript
type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };
function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) return { qua: false, buocHong: buoc.ten };
  }
  return { qua: true, buocHong: null };
}

let soLanChay = 0;
const b1: BuocKiemTra<number> = { ten: "b1", kiemTra: () => { soLanChay++; return true; } };
const b2: BuocKiemTra<number> = { ten: "b2", kiemTra: () => { soLanChay++; return false; } };
const b3: BuocKiemTra<number> = { ten: "b3", kiemTra: () => { soLanChay++; return true; } };

chayPipelineCI(0, b2, b1, b3);
console.log(soLanChay);
```

`b2` (bước LUÔN hỏng) được đặt LÊN ĐẦU TIÊN (thay VÌ Ở giữa). Dòng
cuối in ra gì?

:::opt{correct}
`1`
:::

:::opt
`3` — vì DÙ `b2` được đặt LÊN đầu, `chayPipelineCI` VẪN cần gọi
`kiemTra` của TẤT CẢ ba bước TRƯỚC khi quyết định pipeline CÓ qua
hay không — thứ tự CHỈ ảnh hưởng tới VIỆC nào CHẠY TRƯỚC, KHÔNG ảnh
hưởng SỐ LƯỢNG bước chạy
::why
Gần đúng ở việc bạn nhớ ĐÚNG `chayPipelineCI` NHẬN đủ BA bước làm
tham số — quan sát ĐÓ về SỐ LƯỢNG bước được TRUYỀN vào chính xác.

Chỗ lệch: fail-fast (bài 2) nghĩa LÀ dừng **NGAY** khi GẶP bước hỏng
ĐẦU TIÊN, BẤT KỂ nó Ở VỊ TRÍ nào trong danh sách. `b2` GIỜ Ở VỊ TRÍ
ĐẦU TIÊN — `chayPipelineCI` gọi `b2.kiemTra()` (`soLanChay` LÊN
`1`), THẤY `false`, **THOÁT NGAY**. `b1` VÀ `b3` KHÔNG BAO GIỜ được
gọi TỚI — `soLanChay` DỪNG Ở `1`. Đây CHÍNH LÀ lợi ích của việc đặt
bước "dễ hỏng" LÊN TRƯỚC: PHÁT HIỆN lỗi CÀNG SỚM, CÀNG ÍT bước lãng
phí.
::
:::

:::opt
Máy báo lỗi lúc chạy — đặt `b2` (bước SẼ hỏng) Ở VỊ TRÍ đầu TIÊN của
danh sách tham số KHIẾN `chayPipelineCI` KHÔNG XÁC ĐỊNH được thứ tự
ĐÚNG để chạy, ném lỗi "thứ tự KHÔNG hợp lệ"
::why
Gần đúng ở việc bạn để ý VỊ TRÍ của `b2` ĐÃ THAY ĐỔI SO với ví dụ
TRƯỚC (bài học TRƯỚC đặt `b2` Ở GIỮA) — một quan sát ĐÚNG về SỰ THAY
ĐỔI cấu trúc lời gọi.

Chỗ lệch: `chayPipelineCI` KHÔNG có khái niệm "thứ tự KHÔNG hợp lệ"
— NÓ ĐƠN GIẢN nhận MỘT mảng các bước THEO **BẤT KỲ** thứ tự nào
được TRUYỀN vào, VÀ chạy TUẦN TỰ ĐÚNG thứ tự ĐÓ (`for (const buoc of
cacBuoc)`). Đặt `b2` Ở ĐẦU HOÀN TOÀN hợp lệ — biên dịch VÀ chạy đều
SẠCH.
::
:::
::::

::::code{#viet_dem_buoc_da_chay}
Viết `demBuocDaChay` — đếm SỐ bước THẬT SỰ đã chạy TRƯỚC khi pipeline
dừng (dù dừng VÌ hỏng hay VÌ đã chạy HẾT).

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };

function demBuocDaChay<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): number {
  let dem = 0;
  for (const buoc of cacBuoc) {
    dem = ___;
    if (!buoc.kiemTra(giaTri)) {
      return dem;
    }
  }
  return dem;
}

const soLanA = demBuocDaChay(0, { ten: "a", kiemTra: () => false });
assertEqual(soLanA, 1, "mot buoc, hong ngay, dem 1");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };

function demBuocDaChay<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): number {
  let dem = 0;
  for (const buoc of cacBuoc) {
    dem = dem + 1;
    if (!buoc.kiemTra(giaTri)) {
      return dem;
    }
  }
  return dem;
}

const soLanA = demBuocDaChay(0, { ten: "a", kiemTra: () => false });
assertEqual(soLanA, 1, "mot buoc, hong ngay, dem 1");
```

```typescript title=test
const soLanB = demBuocDaChay(
  0,
  { ten: "a", kiemTra: () => true },
  { ten: "b", kiemTra: () => false },
  { ten: "c", kiemTra: () => true },
);
assertEqual(soLanB, 2, "hong o buoc thu hai, dem 2");

const soLanC = demBuocDaChay(
  0,
  { ten: "a", kiemTra: () => true },
  { ten: "b", kiemTra: () => true },
);
assertEqual(soLanC, 2, "moi buoc qua het, dem het so buoc");

const soLanRong = demBuocDaChay(0);
assertEqual(soLanRong, 0, "khong buoc nao -- dem 0");
```

:::hints
- kind: attention
  body: "dem: tăng lên 1 MỖI lần vào vòng lặp (TRƯỚC khi kiểm tra bước đó qua hay không)."
- kind: strategy
  body: "dem + 1 — tăng bộ đếm đúng một đơn vị mỗi lượt lặp."
- kind: one-line
  body: '___ = dem + 1'
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
Rẻ trước, đắt sau — chỉ có ích KHI thật sự có lỗi. Bài chốt cụm: ghép
fail-fast + thứ tự vào MỘT pipeline bốn bước THẬT.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`chayPipelineCI` (bài 2) NHẬN các bước RIÊNG LẺ. CI thật có BỐN bước
CỐ ĐỊNH: `typecheck` → `lint` → `test` → `build`. Bạn hình dung được
CÁCH ghép BỐN bước ĐÓ thành MỘT pipeline HOÀN CHỈNH chưa?
::::

::::checkpoint{mastery=0.8}
::::
