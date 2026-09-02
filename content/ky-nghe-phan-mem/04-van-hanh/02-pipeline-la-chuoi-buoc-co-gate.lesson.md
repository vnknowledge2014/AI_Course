---
id: ky-nghe-phan-mem.van-hanh.pipeline-la-chuoi-buoc-co-gate
title: "Pipeline = chuỗi bước có GATE — fail-fast, dừng Ở bước đầu tiên hỏng"
summary: "taoPipeline (T5.4 bài 17) LUÔN chạy hết mọi bước — CI pipeline KHÁC: phải dừng NGAY ở bước ĐẦU TIÊN thất bại (\"fail-fast\"), KHÔNG lãng phí thời gian chạy test/build nếu typecheck đã hỏng. chayPipelineCI<T>(giaTri, ...cacBuoc): {qua, buocHong} lặp qua từng bước, TRẢ VỀ NGAY khi gặp bước kiemTra trả false."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [vh.pipeline-fail-fast]
requires: [vh.what-is-ci]
concepts: [vh.pipeline-fail-fast]
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
`taoPipeline` (đã học T5.4 bài 17) LUÔN chạy HẾT mọi bước. CI pipeline
CÓ nên LÀM y hệt — chạy hết build/test dù `typecheck` ĐÃ hỏng?
::::

::::explain{#fail-fast}
CI pipeline KHÁC `taoPipeline`: nó PHẢI dừng **NGAY** Ở bước ĐẦU
TIÊN thất bại ("fail-fast") — KHÔNG lãng phí thời gian chạy
`test`/`build` NẾU `typecheck` ĐÃ hỏng (code SAI KIỂU thì test/build
CHẮC CHẮN cũng hỏng THEO, chạy TIẾP CHỈ tốn thời gian):

```typescript title=readonly
type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };

function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) {
      return { qua: false, buocHong: buoc.ten };
    }
  }
  return { qua: true, buocHong: null };
}

type MaNguon = { coLoiKieu: boolean; testQua: boolean };

const ketQua = chayPipelineCI(
  { coLoiKieu: true, testQua: true },
  { ten: "typecheck", kiemTra: (mn: MaNguon) => !mn.coLoiKieu },
  { ten: "test", kiemTra: (mn: MaNguon) => mn.testQua },
);
console.log(ketQua);
```

```text title=readonly
{"qua":false,"buocHong":"typecheck"}
```

`chayPipelineCI` LẶP qua TỪNG bước — GẶP bước ĐẦU TIÊN mà
`kiemTra(giaTri)` trả `false`, nó **TRẢ VỀ NGAY** (`return` BÊN
TRONG vòng lặp), KHÔNG chạy TIẾP các bước SAU. `testQua: true`
(bước `test` LẼ RA sẽ QUA) HOÀN TOÀN KHÔNG được KIỂM TỚI — vì
`typecheck` ĐÃ hỏng TRƯỚC.
::::

::::example{#buoc-sau-khong-duoc-goi}
Bước SAU bước hỏng KHÔNG CHỈ "bị bỏ qua kết quả" — hàm `kiemTra` của
NÓ **HOÀN TOÀN KHÔNG được gọi**:

```typescript title=readonly
type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };
function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) return { qua: false, buocHong: buoc.ten };
  }
  return { qua: true, buocHong: null };
}

let soLanGoiBuoc3 = 0;
const ketQua = chayPipelineCI(
  0,
  { ten: "b1", kiemTra: () => true },
  { ten: "b2", kiemTra: () => false },
  { ten: "b3", kiemTra: () => { soLanGoiBuoc3++; return true; } },
);
console.log(ketQua.buocHong);
console.log(soLanGoiBuoc3);
```

```text title=readonly
b2
0
```

`b1` qua, `b2` HỎNG → `chayPipelineCI` TRẢ VỀ NGAY LẬP TỨC — `b3`'s
`kiemTra` (dù ĐÃ được viết SẴN, dù LẼ RA sẽ QUA) **CHƯA BAO GIỜ
được gọi** (`soLanGoiBuoc3` VẪN LÀ `0`). Đây LÀ điểm KHÁC BIỆT cốt
lõi với `taoPipeline` (T5.4 bài 17) — `taoPipeline` LUÔN chạy HẾT.
::::

::::predict{#doan-fail-fast-dung-som commitOnce}
```typescript
type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };
function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) return { qua: false, buocHong: buoc.ten };
  }
  return { qua: true, buocHong: null };
}

let soLanGoiC = 0;
let soLanGoiD = 0;
const ketQua = chayPipelineCI(
  1,
  { ten: "A", kiemTra: () => true },
  { ten: "B", kiemTra: () => true },
  { ten: "C", kiemTra: () => { soLanGoiC++; return false; } },
  { ten: "D", kiemTra: () => { soLanGoiD++; return true; } },
);
console.log(ketQua.buocHong);
console.log(soLanGoiC);
console.log(soLanGoiD);
```

Ba dòng cuối in ra gì?

:::opt{correct}
`C` rồi `1` rồi `0`
:::

:::opt
`C` rồi `1` rồi `1` — vì DÙ `C` hỏng, `chayPipelineCI` VẪN CẦN chạy
QUA HẾT các bước CÒN LẠI (bao gồm `D`) để BIẾT chắc CHẮN CÓ bước
nào KHÁC cũng hỏng hay không, TRƯỚC khi trả kết quả CUỐI CÙNG
::why
Gần đúng ở việc bạn nhớ ĐÚNG `C` LÀ bước ĐẦU TIÊN hỏng (`soLanGoiC`
ĐÚNG LÀ `1`, tức LÀ ĐÃ được gọi) — quan sát ĐÓ chính xác.

Chỗ lệch: `chayPipelineCI` **KHÔNG** cần "biết chắc CÒN bước nào
khác hỏng" — MỘT bước hỏng LÀ ĐỦ để BIẾT pipeline THẤT BẠI, KHÔNG
CẦN kiểm TIẾP. Dòng `if (!buoc.kiemTra(giaTri)) return {...}` BÊN
TRONG vòng lặp `for` THOÁT NGAY khỏi TOÀN BỘ hàm khi GẶP `C` — vòng
lặp **KHÔNG BAO GIỜ** tới lượt `D`. `soLanGoiD` GIỮ NGUYÊN `0`.
::
:::

:::opt
Máy báo lỗi biên dịch — `chayPipelineCI` khai `...cacBuoc:
BuocKiemTra<T>[]` (rest parameter), NHƯNG BỐN object được TRUYỀN
VÀO CÓ property `kiemTra` với THÂN hàm KHÁC ĐỘ DÀI (một dòng VS
nhiều dòng), TypeScript đòi MỌI phần tử rest parameter PHẢI có THÂN
hàm CÙNG cấu trúc
::why
Gần đúng ở việc bạn để ý BỐN object `{ten, kiemTra}` viết THÂN hàm
`kiemTra` KHÁC nhau (một số MỘT dòng, MỘT SỐ nhiều dòng CÓ khối
`{...}`) — một quan sát ĐÚNG về SỰ ĐA DẠNG cú pháp.

Chỗ lệch: TypeScript CHỈ quan tâm **KIỂU** của `kiemTra` (`(t: T) =>
boolean` — NHẬN MỘT `T`, TRẢ MỘT `boolean`) — HOÀN TOÀN KHÔNG quan
tâm THÂN hàm DÀI hay NGẮN, MỘT dòng hay NHIỀU dòng. MỌI hàm mũi tên
Ở ĐÂY ĐỀU khớp kiểu ĐÓ — biên dịch SẠCH.
::
:::
::::

::::code{#viet_chay_pipeline_ci}
Hoàn thiện `chayPipelineCI` — kiểm TỪNG bước, dừng NGAY VÀ trả về
TÊN bước hỏng khi GẶP bước ĐẦU TIÊN thất bại.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };

function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (___) {
      return ___;
    }
  }
  return { qua: true, buocHong: null };
}

const kq1 = chayPipelineCI(5, { ten: "duong", kiemTra: (n: number) => n > 0 });
assertEqual(kq1.qua, true, "mot buoc qua");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };

function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) {
      return { qua: false, buocHong: buoc.ten };
    }
  }
  return { qua: true, buocHong: null };
}

const kq1 = chayPipelineCI(5, { ten: "duong", kiemTra: (n: number) => n > 0 });
assertEqual(kq1.qua, true, "mot buoc qua");
```

```typescript title=test
const kq2 = chayPipelineCI(-5, { ten: "duong", kiemTra: (n: number) => n > 0 });
assertEqual(kq2.qua, false, "buoc khong qua");
assertEqual(kq2.buocHong, "duong", "buocHong dung ten");

let soLanGoiB2 = 0;
const kq3 = chayPipelineCI(
  0,
  { ten: "b1", kiemTra: () => false },
  { ten: "b2", kiemTra: () => { soLanGoiB2++; return true; } },
);
assertEqual(kq3.buocHong, "b1", "dung buoc dau tien hong");
assertEqual(soLanGoiB2, 0, "buoc sau KHONG duoc goi -- fail fast");

const kq4 = chayPipelineCI(10,
  { ten: "b1", kiemTra: () => true },
  { ten: "b2", kiemTra: () => true },
  { ten: "b3", kiemTra: () => true },
);
assertEqual(kq4.qua, true, "moi buoc qua het");
assertEqual(kq4.buocHong, null, "khong buoc nao hong");
```

:::hints
- kind: attention
  body: "Điều kiện: bước ĐÓ kiểm tra KHÔNG qua (phủ định kết quả kiemTra). Kết quả trả về: qua=false, buocHong=đúng TÊN của bước ĐANG xét."
- kind: strategy
  body: "!buoc.kiemTra(giaTri) : { qua: false, buocHong: buoc.ten } — phủ định điều kiện qua, và trả object có tên bước hỏng."
- kind: one-line
  body: '___ (điều kiện) = !buoc.kiemTra(giaTri)\n___ (trả về) = { qua: false, buocHong: buoc.ten }'
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
Fail-fast: dừng NGAY ở bước đầu hỏng, bước sau KHÔNG được gọi. Bài
tiếp theo: THỨ TỰ các bước có quan trọng không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`chayPipelineCI` chạy các bước THEO ĐÚNG THỨ TỰ được TRUYỀN vào. Nếu
đặt bước `build` (rất CHẬM) TRƯỚC bước `typecheck` (rất NHANH) —
liệu THỨ TỰ đó có ẢNH HƯỞNG gì tới trải nghiệm không?
::::

::::checkpoint{mastery=0.8}
::::
