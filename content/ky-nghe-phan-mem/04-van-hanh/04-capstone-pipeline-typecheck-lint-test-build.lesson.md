---
id: ky-nghe-phan-mem.van-hanh.capstone-pipeline-typecheck-lint-test-build
title: "Capstone: Pipeline typecheck → lint → test → build"
summary: "Bài chốt cụm 1: mô phỏng đầy đủ bốn bước CI thật (typecheck, lint, test, build), mỗi bước là một hàm kiểm tra MỘT khía cạnh của một MaNguon giả lập, qua chayPipelineCI (bài 2). Kiểm CẢ trường hợp mọi bước qua LẪN từng trường hợp một bước cụ thể hỏng (buocHong đúng tên), và xác nhận build KHÔNG được gọi khi typecheck đã hỏng trước đó."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [vh.gate-boss-ci-pipeline]
requires: [vh.step-ordering-cheap-first]
concepts: [vh.gate-boss-ci-pipeline]
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
Bài chốt cụm 1. Ghép fail-fast (bài 2) + thứ tự rẻ-trước-đắt-sau (bài
3) thành MỘT pipeline CI thật: `typecheck → lint → test → build`.
::::

::::explain{#pipeline-bon-buoc-that}
MỖI bước LÀ một hàm kiểm tra MỘT khía cạnh của MỘT `MaNguon` giả
lập — GHÉP CẢ BỐN qua `chayPipelineCI` (bài 2), THEO ĐÚNG THỨ TỰ
rẻ-trước-đắt-sau (bài 3: `typecheck`/`lint` NHANH, `test`/`build`
CHẬM HƠN):

```typescript title=readonly
type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };
function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) return { qua: false, buocHong: buoc.ten };
  }
  return { qua: true, buocHong: null };
}

type MaNguon = { coLoiKieu: boolean; coLoiLint: boolean; testQua: boolean; buildQua: boolean };

const buocTypecheck: BuocKiemTra<MaNguon> = { ten: "typecheck", kiemTra: (mn) => !mn.coLoiKieu };
const buocLint: BuocKiemTra<MaNguon> = { ten: "lint", kiemTra: (mn) => !mn.coLoiLint };
const buocTest: BuocKiemTra<MaNguon> = { ten: "test", kiemTra: (mn) => mn.testQua };
const buocBuild: BuocKiemTra<MaNguon> = { ten: "build", kiemTra: (mn) => mn.buildQua };

function chayCiDayDu(mn: MaNguon) {
  return chayPipelineCI(mn, buocTypecheck, buocLint, buocTest, buocBuild);
}

const maNguonSach: MaNguon = { coLoiKieu: false, coLoiLint: false, testQua: true, buildQua: true };
console.log(chayCiDayDu(maNguonSach));

const maNguonLoiTest: MaNguon = { coLoiKieu: false, coLoiLint: false, testQua: false, buildQua: true };
console.log(chayCiDayDu(maNguonLoiTest));
```

```text title=readonly
{"qua":true,"buocHong":null}
{"qua":false,"buocHong":"test"}
```

`maNguonSach` (KHÔNG lỗi GÌ) qua HẾT BỐN bước. `maNguonLoiTest`
(`testQua: false`, MỌI thứ KHÁC ổn) — `typecheck` VÀ `lint` ĐỀU
qua, `test` HỎNG → dừng NGAY, `buocHong: "test"` — `build` KHÔNG
BAO GIỜ được KIỂM TỚI.
::::

::::example{#loi-o-giua-pipeline}
Lỗi Ở BẤT KỲ bước NÀO (KHÔNG CHỈ bước ĐẦU) ĐỀU được BÁO đúng TÊN:

```typescript title=readonly
type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };
function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) return { qua: false, buocHong: buoc.ten };
  }
  return { qua: true, buocHong: null };
}
type MaNguon = { coLoiKieu: boolean; coLoiLint: boolean; testQua: boolean; buildQua: boolean };
const buocTypecheck: BuocKiemTra<MaNguon> = { ten: "typecheck", kiemTra: (mn) => !mn.coLoiKieu };
const buocLint: BuocKiemTra<MaNguon> = { ten: "lint", kiemTra: (mn) => !mn.coLoiLint };
const buocTest: BuocKiemTra<MaNguon> = { ten: "test", kiemTra: (mn) => mn.testQua };
const buocBuild: BuocKiemTra<MaNguon> = { ten: "build", kiemTra: (mn) => mn.buildQua };
function chayCiDayDu(mn: MaNguon) {
  return chayPipelineCI(mn, buocTypecheck, buocLint, buocTest, buocBuild);
}

const maNguonLoiLintNhungBuildOk: MaNguon = { coLoiKieu: false, coLoiLint: true, testQua: true, buildQua: true };
console.log(chayCiDayDu(maNguonLoiLintNhungBuildOk));
```

```text title=readonly
{"qua":false,"buocHong":"lint"}
```

`buildQua: true` (bước CUỐI CÙNG, LẼ RA sẽ qua) **KHÔNG CỨU** được
pipeline — `lint` (bước THỨ HAI) ĐÃ hỏng TRƯỚC, `qua` LUÔN LÀ `false`
BẤT KỂ các bước SAU CÓ "tốt" tới đâu.
::::

::::predict{#doan-build-tot-khong-cuu-duoc-pipeline commitOnce}
```typescript
type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };
function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) return { qua: false, buocHong: buoc.ten };
  }
  return { qua: true, buocHong: null };
}
type MaNguon = { coLoiKieu: boolean; coLoiLint: boolean; testQua: boolean; buildQua: boolean };
const buocTypecheck: BuocKiemTra<MaNguon> = { ten: "typecheck", kiemTra: (mn) => !mn.coLoiKieu };
const buocLint: BuocKiemTra<MaNguon> = { ten: "lint", kiemTra: (mn) => !mn.coLoiLint };
const buocTest: BuocKiemTra<MaNguon> = { ten: "test", kiemTra: (mn) => mn.testQua };
const buocBuild: BuocKiemTra<MaNguon> = { ten: "build", kiemTra: (mn) => mn.buildQua };
function chayCiDayDu(mn: MaNguon) {
  return chayPipelineCI(mn, buocTypecheck, buocLint, buocTest, buocBuild);
}

const maNguon: MaNguon = { coLoiKieu: false, coLoiLint: false, testQua: false, buildQua: true };
console.log(chayCiDayDu(maNguon).qua);
```

`testQua: false` (lỗi Ở test), NHƯNG `buildQua: true` (build LẼ RA
sẽ qua NẾU được chạy TỚI). Dòng cuối in ra gì?

:::opt{correct}
`false`
:::

:::opt
`true` — vì `buildQua: true` LÀ giá trị CUỐI CÙNG trong OBJECT
`maNguon`, VÀ `chayCiDayDu` trả về `qua` DỰA TRÊN giá trị CỦA
TRƯỜNG **CUỐI CÙNG** này, KHÔNG PHẢI dựa trên TOÀN BỘ CHUỖI bước
::why
Gần đúng ở việc bạn để ý `buildQua: true` LÀ giá trị `true` — quan
sát ĐÓ về NỘI DUNG của trường ĐÚNG.

Chỗ lệch: `chayPipelineCI` KHÔNG "nhìn" MỘT trường CỤ THỂ NÀO của
`maNguon` — nó CHỈ gọi LẦN LƯỢT `kiemTra` của TỪNG bước ĐÃ được
TRUYỀN vào, THEO ĐÚNG thứ tự (`typecheck` → `lint` → `test` →
`build`), VÀ dừng NGAY khi GẶP bước ĐẦU TIÊN trả `false`. `test`
(bước THỨ BA) trả `false` (`testQua: false`) — pipeline DỪNG TẠI
ĐÓ, `qua` LÀ `false` — `buocBuild.kiemTra` (dù SẼ trả `true`) KHÔNG
BAO GIỜ được GỌI TỚI để "cứu" kết quả.
::
:::

:::opt
Máy báo lỗi biên dịch — `MaNguon` khai BỐN trường BOOLEAN, NHƯNG BA
trong SỐ ĐÓ (`coLoiKieu`, `coLoiLint`) MANG Ý NGHĨA "CÓ lỗi" trong
khi HAI trường KIA (`testQua`, `buildQua`) MANG Ý NGHĨA "CÓ qua",
TypeScript CẤM TRỘN hai QUY ƯỚC đặt tên NGƯỢC nghĩa trong CÙNG một
kiểu
::why
Gần đúng ở việc bạn để ý BỐN trường CÓ Ý NGHĨA "hướng" KHÁC NHAU
(`coLoi...` = "TRUE nghĩa LÀ XẤU", `...Qua` = "TRUE nghĩa LÀ TỐT")
— một quan sát TINH Ý về THIẾT KẾ đặt TÊN (ĐÂY THẬT SỰ LÀ một điểm
cần CẨN THẬN khi ĐỌC code — nhưng KHÔNG PHẢI lỗi biên dịch).

Chỗ lệch: TypeScript CHỈ quan tâm **KIỂU** (`boolean`) của MỖI
trường — HOÀN TOÀN KHÔNG có khái niệm "quy ước đặt tên NGƯỢC nghĩa
BỊ CẤM". Biên dịch SẠCH — nhưng ĐÂY LÀ lý do TỐT để LUÔN đọc KỸ Ý
NGHĨA từng trường (như `kiemTra: (mn) => !mn.coLoiKieu` — dấu `!`
CHÍNH LÀ chỗ "đảo hướng" quy ước NGƯỢC nghĩa NÀY).
::
:::
::::

::::code{#viet_capstone_ci_pipeline}
Hoàn thiện `buocLint`/`buocTest` (điều kiện qua) VÀ `chayCiDayDu`
(ghép cả BỐN bước qua `chayPipelineCI`).

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };
function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) return { qua: false, buocHong: buoc.ten };
  }
  return { qua: true, buocHong: null };
}

type MaNguon = { coLoiKieu: boolean; coLoiLint: boolean; testQua: boolean; buildQua: boolean };

const buocTypecheck: BuocKiemTra<MaNguon> = { ten: "typecheck", kiemTra: (mn) => !mn.coLoiKieu };
const buocLint: BuocKiemTra<MaNguon> = { ten: "lint", kiemTra: (mn) => ___ };
const buocTest: BuocKiemTra<MaNguon> = { ten: "test", kiemTra: (mn) => ___ };
const buocBuild: BuocKiemTra<MaNguon> = { ten: "build", kiemTra: (mn) => mn.buildQua };

function chayCiDayDu(mn: MaNguon): { qua: boolean; buocHong: string | null } {
  return ___;
}

const maNguonSach: MaNguon = { coLoiKieu: false, coLoiLint: false, testQua: true, buildQua: true };
assertEqual(chayCiDayDu(maNguonSach).qua, true, "moi buoc qua het");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type BuocKiemTra<T> = { ten: string; kiemTra: (t: T) => boolean };
function chayPipelineCI<T>(giaTri: T, ...cacBuoc: BuocKiemTra<T>[]): { qua: boolean; buocHong: string | null } {
  for (const buoc of cacBuoc) {
    if (!buoc.kiemTra(giaTri)) return { qua: false, buocHong: buoc.ten };
  }
  return { qua: true, buocHong: null };
}

type MaNguon = { coLoiKieu: boolean; coLoiLint: boolean; testQua: boolean; buildQua: boolean };

const buocTypecheck: BuocKiemTra<MaNguon> = { ten: "typecheck", kiemTra: (mn) => !mn.coLoiKieu };
const buocLint: BuocKiemTra<MaNguon> = { ten: "lint", kiemTra: (mn) => !mn.coLoiLint };
const buocTest: BuocKiemTra<MaNguon> = { ten: "test", kiemTra: (mn) => mn.testQua };
const buocBuild: BuocKiemTra<MaNguon> = { ten: "build", kiemTra: (mn) => mn.buildQua };

function chayCiDayDu(mn: MaNguon): { qua: boolean; buocHong: string | null } {
  return chayPipelineCI(mn, buocTypecheck, buocLint, buocTest, buocBuild);
}

const maNguonSach: MaNguon = { coLoiKieu: false, coLoiLint: false, testQua: true, buildQua: true };
assertEqual(chayCiDayDu(maNguonSach).qua, true, "moi buoc qua het");
```

```typescript title=test
const maNguonLoiKieu: MaNguon = { coLoiKieu: true, coLoiLint: false, testQua: true, buildQua: true };
assertEqual(chayCiDayDu(maNguonLoiKieu).buocHong, "typecheck", "loi kieu bi bat dau tien");

const maNguonLoiLint: MaNguon = { coLoiKieu: false, coLoiLint: true, testQua: true, buildQua: true };
assertEqual(chayCiDayDu(maNguonLoiLint).buocHong, "lint", "loi lint bi bat");

const maNguonLoiTest: MaNguon = { coLoiKieu: false, coLoiLint: false, testQua: false, buildQua: true };
assertEqual(chayCiDayDu(maNguonLoiTest).buocHong, "test", "loi test bi bat");

const maNguonLoiBuild: MaNguon = { coLoiKieu: false, coLoiLint: false, testQua: true, buildQua: false };
assertEqual(chayCiDayDu(maNguonLoiBuild).buocHong, "build", "loi build bi bat");

let soLanGoiBuild = 0;
const buocBuildDem: BuocKiemTra<MaNguon> = { ten: "build", kiemTra: (mn) => { soLanGoiBuild++; return mn.buildQua; } };
chayPipelineCI({ coLoiKieu: true, coLoiLint: false, testQua: true, buildQua: true }, buocTypecheck, buocLint, buocTest, buocBuildDem);
assertEqual(soLanGoiBuild, 0, "build khong duoc goi khi typecheck da hong");
```

:::hints
- kind: attention
  body: "buocLint: qua khi KHÔNG coLoiLint (phủ định). buocTest: qua khi testQua đúng LÀ true (không phủ định — TRỰC TIẾP). chayCiDayDu: gọi chayPipelineCI với mn và ĐỦ bốn bước theo đúng thứ tự."
- kind: strategy
  body: "!mn.coLoiLint : mn.testQua : chayPipelineCI(mn, buocTypecheck, buocLint, buocTest, buocBuild)"
- kind: one-line
  body: '___ (buocLint) = !mn.coLoiLint\n___ (buocTest) = mn.testQua\n___ (chayCiDayDu) = chayPipelineCI(mn, buocTypecheck, buocLint, buocTest, buocBuild)'
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
Cụm 1 hoàn tất: CI là gì, fail-fast, thứ tự rẻ-trước-đắt-sau, capstone
pipeline bốn bước THẬT. Cụm tiếp theo: khi pipeline QUA, có deploy
NGAY không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`chayCiDayDu` chỉ TRẢ VỀ `{qua, buocHong}` — nó KHÔNG tự Ý deploy gì
cả. Deploy THẬT cần THÊM điều kiện GÌ, NGOÀI việc pipeline `qua`?
::::

::::checkpoint{mastery=0.8}
::::
