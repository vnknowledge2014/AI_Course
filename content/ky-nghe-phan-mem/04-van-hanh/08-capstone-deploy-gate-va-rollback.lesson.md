---
id: ky-nghe-phan-mem.van-hanh.capstone-deploy-gate-va-rollback
title: "Capstone: Hệ deploy có GATE + Rollback"
summary: "Bài chốt cụm 2: ghép xacDinhMoiTruong (bài 6, quyết định đích) + deploy/rollback (bài 7, lịch sử bất biến) thành thuDeploy — TỪ CHỐI khi pipeline hỏng, ghi VÀO đúng lịch sử (staging hoặc production) khi qua, và rollback đưa lịch sử về phiên bản trước. Kiểm ĐỦ ba tình huống: từ chối, deploy thành công đúng môi trường, rollback."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [vh.gate-boss-deploy-rollback]
requires: [vh.rollback-previous-version]
concepts: [vh.gate-boss-deploy-rollback]
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
Bài chốt cụm 2. Ghép quyết định môi trường (bài 6) + lịch sử/rollback
(bài 7) thành MỘT quy trình: `thuDeploy`.
::::

::::explain{#thu-deploy-ghep-tron}
`thuDeploy` NHẬN kết quả pipeline (cụm 1), tên NHÁNH, VÀ phiên bản —
TỪ CHỐI NGAY nếu pipeline HỎNG (KHÔNG cần biết môi trường LÀ gì),
NGƯỢC LẠI dùng `xacDinhMoiTruong` (bài 6) để CHỌN ĐÚNG lịch sử ghi
VÀO (`deploy`, bài 7):

```typescript title=readonly
type KetQuaPipeline = { qua: boolean; buocHong: string | null };
type MoiTruong = "staging" | "production" | "khong_deploy";

function xacDinhMoiTruong(tenNhanh: string): MoiTruong {
  switch (tenNhanh) {
    case "develop": return "staging";
    case "main": return "production";
    default: return "khong_deploy";
  }
}

function deploy(lichSu: readonly string[], phienBan: string): readonly string[] {
  return [...lichSu, phienBan];
}
function rollback(lichSu: readonly string[]): readonly string[] {
  if (lichSu.length <= 1) return lichSu;
  return lichSu.slice(0, lichSu.length - 1);
}
function phienBanHienTai(lichSu: readonly string[]): string {
  return lichSu[lichSu.length - 1] ?? "chua-co-gi";
}

type HeThongDeploy = { lichSuStaging: readonly string[]; lichSuProduction: readonly string[] };

function thuDeploy(he: HeThongDeploy, kq: KetQuaPipeline, tenNhanh: string, phienBan: string): HeThongDeploy {
  if (!kq.qua) return he;
  const moiTruong = xacDinhMoiTruong(tenNhanh);
  if (moiTruong === "staging") {
    return { ...he, lichSuStaging: deploy(he.lichSuStaging, phienBan) };
  }
  if (moiTruong === "production") {
    return { ...he, lichSuProduction: deploy(he.lichSuProduction, phienBan) };
  }
  return he;
}

let he: HeThongDeploy = { lichSuStaging: ["v0"], lichSuProduction: ["v0"] };

he = thuDeploy(he, { qua: true, buocHong: null }, "develop", "v1-staging");
console.log(phienBanHienTai(he.lichSuStaging));
console.log(phienBanHienTai(he.lichSuProduction));

he = thuDeploy(he, { qua: false, buocHong: "test" }, "main", "v1-prod-loi");
console.log(phienBanHienTai(he.lichSuProduction));

he = thuDeploy(he, { qua: true, buocHong: null }, "main", "v1-prod");
console.log(phienBanHienTai(he.lichSuProduction));

he = { ...he, lichSuProduction: rollback(he.lichSuProduction) };
console.log(phienBanHienTai(he.lichSuProduction));
```

```text title=readonly
v1-staging
v0
v0
v1-prod
v0
```

Deploy `develop` (pipeline QUA) → CHỈ `lichSuStaging` đổi, `lichSuProduction`
GIỮ NGUYÊN. Deploy `main` (pipeline HỎNG) → `lichSuProduction` VẪN
KHÔNG đổi (`v0`). Deploy `main` (pipeline QUA) → `lichSuProduction`
LÊN `v1-prod`. Rollback → VỀ `v0`.
::::

::::example{#nhanh-tinh-nang-khong-cham-gi}
MỘT nhánh TÍNH NĂNG — DÙ pipeline QUA HOÀN TOÀN — KHÔNG CHẠM tới
BẤT KỲ lịch sử NÀO (KHÔNG staging, KHÔNG production):

```typescript title=readonly
type KetQuaPipeline = { qua: boolean; buocHong: string | null };
type MoiTruong = "staging" | "production" | "khong_deploy";
function xacDinhMoiTruong(tenNhanh: string): MoiTruong {
  switch (tenNhanh) {
    case "develop": return "staging";
    case "main": return "production";
    default: return "khong_deploy";
  }
}
function deploy(lichSu: readonly string[], phienBan: string): readonly string[] {
  return [...lichSu, phienBan];
}
type HeThongDeploy = { lichSuStaging: readonly string[]; lichSuProduction: readonly string[] };
function thuDeploy(he: HeThongDeploy, kq: KetQuaPipeline, tenNhanh: string, phienBan: string): HeThongDeploy {
  if (!kq.qua) return he;
  const moiTruong = xacDinhMoiTruong(tenNhanh);
  if (moiTruong === "staging") return { ...he, lichSuStaging: deploy(he.lichSuStaging, phienBan) };
  if (moiTruong === "production") return { ...he, lichSuProduction: deploy(he.lichSuProduction, phienBan) };
  return he;
}

let he: HeThongDeploy = { lichSuStaging: ["v0"], lichSuProduction: ["v0"] };
he = thuDeploy(he, { qua: true, buocHong: null }, "feature/thu-nghiem", "v-nguy-hiem");
console.log(he.lichSuStaging.length);
console.log(he.lichSuProduction.length);
```

```text title=readonly
1
1
```

`xacDinhMoiTruong("feature/thu-nghiem")` trả `"khong_deploy"` — CẢ
HAI nhánh `if` (`staging`/`production`) ĐỀU KHÔNG khớp, `thuDeploy`
rơi TỚI `return he;` CUỐI CÙNG — KHÔNG có gì thay ĐỔI.
::::

::::predict{#doan-staging-va-production-doc-lap commitOnce}
```typescript
type KetQuaPipeline = { qua: boolean; buocHong: string | null };
type MoiTruong = "staging" | "production" | "khong_deploy";
function xacDinhMoiTruong(tenNhanh: string): MoiTruong {
  switch (tenNhanh) {
    case "develop": return "staging";
    case "main": return "production";
    default: return "khong_deploy";
  }
}
function deploy(lichSu: readonly string[], phienBan: string): readonly string[] {
  return [...lichSu, phienBan];
}
type HeThongDeploy = { lichSuStaging: readonly string[]; lichSuProduction: readonly string[] };
function thuDeploy(he: HeThongDeploy, kq: KetQuaPipeline, tenNhanh: string, phienBan: string): HeThongDeploy {
  if (!kq.qua) return he;
  const moiTruong = xacDinhMoiTruong(tenNhanh);
  if (moiTruong === "staging") return { ...he, lichSuStaging: deploy(he.lichSuStaging, phienBan) };
  if (moiTruong === "production") return { ...he, lichSuProduction: deploy(he.lichSuProduction, phienBan) };
  return he;
}

let he: HeThongDeploy = { lichSuStaging: ["s0"], lichSuProduction: ["p0"] };
he = thuDeploy(he, { qua: true, buocHong: null }, "develop", "s1");
he = thuDeploy(he, { qua: true, buocHong: null }, "develop", "s2");
he = thuDeploy(he, { qua: false, buocHong: "build" }, "main", "p1-loi");
console.log(he.lichSuStaging.length);
console.log(he.lichSuProduction.length);
```

Hai dòng cuối in ra gì?

:::opt{correct}
`3` rồi `1`
:::

:::opt
`3` rồi `2` — vì `thuDeploy` được gọi TỔNG CỘNG BA lần, VÀ MỖI lần
gọi (BẤT KỂ pipeline qua hay HỎNG) ĐỀU ĐẨY THÊM một mục vào ĐÚNG
lịch sử tương ứng VỚI nhánh — chỉ NỘI DUNG mục ĐÓ khác nhau
(`"p1-loi"` VẪN được GHI, dù LÀ bản LỖI)
::why
Gần đúng ở việc bạn nhớ ĐÚNG `thuDeploy` được GỌI ba lần, VÀ lần THỨ
BA nhắm TỚI `lichSuProduction` (nhánh `"main"`) — quan sát ĐÓ về SỐ
LẦN gọi VÀ đích NHẮM đúng.

Chỗ lệch: dòng ĐẦU TIÊN CỦA `thuDeploy` (`if (!kq.qua) return he;`)
CHẶN **TRƯỚC KHI** chạm tới BẤT KỲ logic ghi lịch sử NÀO — lần gọi
THỨ BA có `kq.qua === false`, hàm TRẢ VỀ `he` **NGUYÊN VẸN** NGAY
LẬP TỨC, KHÔNG BAO GIỜ chạy tới dòng `deploy(he.lichSuProduction,
phienBan)`. `"p1-loi"` KHÔNG BAO GIỜ được GHI VÀO bất kỳ đâu —
`lichSuProduction` GIỮ NGUYÊN `["p0"]`, độ dài `1`.
::
:::

:::opt
Máy báo lỗi biên dịch — `he.lichSuStaging`/`he.lichSuProduction`
khai kiểu `readonly string[]`, NHƯNG `thuDeploy` TRẢ VỀ MỘT
`HeThongDeploy` **MỚI** (spread `{ ...he, ... }`) MỖI LẦN gọi,
TypeScript CẤM tạo NHIỀU bản sao của MỘT kiểu chứa TRƯỜNG `readonly`
::why
Gần đúng ở việc bạn để ý `thuDeploy` LUÔN tạo MỘT object `HeThongDeploy`
**MỚI** MỖI lần gọi (`{ ...he, lichSuStaging: ... }`) — một quan sát
ĐÚNG về CƠ CHẾ spread.

Chỗ lệch: TypeScript KHÔNG có quy tắc "CẤM tạo NHIỀU bản sao" — tạo
BAO NHIÊU object MỚI TUỲ Ý, MỖI cái ĐỘC LẬP HOÀN TOÀN, LÀ cách LÀM
VIỆC BÌNH THƯỜNG với dữ liệu BẤT BIẾN (`readonly`) — ĐÂY CHÍNH LÀ lý
do dùng `readonly`: AN TOÀN khi TẠO NHIỀU bản sao, VÌ KHÔNG bản sao
NÀO ảnh hưởng bản KHÁC. Biên dịch SẠCH.
::
:::
::::

::::code{#viet_capstone_deploy_rollback}
Hoàn thiện `thuDeploy` — từ chối NGAY khi pipeline hỏng, GHI vào
đúng lịch sử (staging HOẶC production) khi qua.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type KetQuaPipeline = { qua: boolean; buocHong: string | null };
type MoiTruong = "staging" | "production" | "khong_deploy";

function xacDinhMoiTruong(tenNhanh: string): MoiTruong {
  switch (tenNhanh) {
    case "develop": return "staging";
    case "main": return "production";
    default: return "khong_deploy";
  }
}

function deploy(lichSu: readonly string[], phienBan: string): readonly string[] {
  return [...lichSu, phienBan];
}
function rollback(lichSu: readonly string[]): readonly string[] {
  if (lichSu.length <= 1) return lichSu;
  return lichSu.slice(0, lichSu.length - 1);
}
function phienBanHienTai(lichSu: readonly string[]): string {
  return lichSu[lichSu.length - 1] ?? "chua-co-gi";
}

type HeThongDeploy = { lichSuStaging: readonly string[]; lichSuProduction: readonly string[] };

function thuDeploy(he: HeThongDeploy, kq: KetQuaPipeline, tenNhanh: string, phienBan: string): HeThongDeploy {
  if (___) return he;
  const moiTruong = xacDinhMoiTruong(tenNhanh);
  if (moiTruong === "staging") {
    return { ...he, lichSuStaging: deploy(he.lichSuStaging, phienBan) };
  }
  if (moiTruong === "production") {
    return ___;
  }
  return he;
}

let he1: HeThongDeploy = { lichSuStaging: ["s0"], lichSuProduction: ["p0"] };
he1 = thuDeploy(he1, { qua: true, buocHong: null }, "develop", "s1");
assertEqual(phienBanHienTai(he1.lichSuStaging), "s1", "deploy len staging thanh cong");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type KetQuaPipeline = { qua: boolean; buocHong: string | null };
type MoiTruong = "staging" | "production" | "khong_deploy";

function xacDinhMoiTruong(tenNhanh: string): MoiTruong {
  switch (tenNhanh) {
    case "develop": return "staging";
    case "main": return "production";
    default: return "khong_deploy";
  }
}

function deploy(lichSu: readonly string[], phienBan: string): readonly string[] {
  return [...lichSu, phienBan];
}
function rollback(lichSu: readonly string[]): readonly string[] {
  if (lichSu.length <= 1) return lichSu;
  return lichSu.slice(0, lichSu.length - 1);
}
function phienBanHienTai(lichSu: readonly string[]): string {
  return lichSu[lichSu.length - 1] ?? "chua-co-gi";
}

type HeThongDeploy = { lichSuStaging: readonly string[]; lichSuProduction: readonly string[] };

function thuDeploy(he: HeThongDeploy, kq: KetQuaPipeline, tenNhanh: string, phienBan: string): HeThongDeploy {
  if (!kq.qua) return he;
  const moiTruong = xacDinhMoiTruong(tenNhanh);
  if (moiTruong === "staging") {
    return { ...he, lichSuStaging: deploy(he.lichSuStaging, phienBan) };
  }
  if (moiTruong === "production") {
    return { ...he, lichSuProduction: deploy(he.lichSuProduction, phienBan) };
  }
  return he;
}

let he1: HeThongDeploy = { lichSuStaging: ["s0"], lichSuProduction: ["p0"] };
he1 = thuDeploy(he1, { qua: true, buocHong: null }, "develop", "s1");
assertEqual(phienBanHienTai(he1.lichSuStaging), "s1", "deploy len staging thanh cong");
```

```typescript title=test
he1 = thuDeploy(he1, { qua: false, buocHong: "test" }, "main", "p1-loi");
assertEqual(phienBanHienTai(he1.lichSuProduction), "p0", "pipeline hong -- production khong doi");

he1 = thuDeploy(he1, { qua: true, buocHong: null }, "main", "p1");
assertEqual(phienBanHienTai(he1.lichSuProduction), "p1", "deploy len production thanh cong");

he1 = thuDeploy(he1, { qua: true, buocHong: null }, "feature/x", "nguy-hiem");
assertEqual(phienBanHienTai(he1.lichSuStaging), "s1", "nhanh tinh nang khong lam gi voi staging");
assertEqual(phienBanHienTai(he1.lichSuProduction), "p1", "nhanh tinh nang khong lam gi voi production");

he1 = { ...he1, lichSuProduction: rollback(he1.lichSuProduction) };
assertEqual(phienBanHienTai(he1.lichSuProduction), "p0", "rollback production ve phien ban truoc");
```

:::hints
- kind: attention
  body: "Điều kiện đầu: pipeline KHÔNG qua thì thoát ngay. Nhánh production: giống hệt nhánh staging phía trên, chỉ đổi tên trường và hàm gọi."
- kind: strategy
  body: "!kq.qua : { ...he, lichSuProduction: deploy(he.lichSuProduction, phienBan) }"
- kind: one-line
  body: '___ (điều kiện) = !kq.qua\n___ (production) = { ...he, lichSuProduction: deploy(he.lichSuProduction, phienBan) }'
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
Cụm 2 hoàn tất: deploy có điều kiện, môi trường, rollback, capstone
hệ hoàn chỉnh. Cụm tiếp theo: production cần LOG có cấu trúc, không
phải console.log.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`thuDeploy` hiện KHÔNG ghi lại GÌ về VIỆC nó ĐÃ từ chối hay chấp
nhận — KHÔNG có "dấu vết". Nếu MUỐN biết SAU NÀY "lúc 10 giờ sáng,
deploy nào đã bị TỪ CHỐI VÀ TẠI SAO" — cần THÊM cái gì?
::::

::::checkpoint{mastery=0.8}
::::
