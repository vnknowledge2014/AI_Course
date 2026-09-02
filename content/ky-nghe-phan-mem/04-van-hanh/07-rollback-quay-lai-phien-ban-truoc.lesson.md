---
id: ky-nghe-phan-mem.van-hanh.rollback-quay-lai-phien-ban-truoc
title: "Rollback — quay LẠI phiên bản TRƯỚC khi bản MỚI lỗi"
summary: "Deploy một phiên bản MỚI có thể lỗi (dù pipeline đã qua — test không bắt được mọi thứ). Rollback = quay VỀ phiên bản NGAY TRƯỚC đó — CHÍNH XÁC LÀ kỹ thuật undo đã học T5.4 bài 11 (lưu SNAPSHOT lịch sử deploy, không cần \"tính nghịch đảo\" của một lần deploy). lichSuDeploy: string[], rollback cắt phần tử cuối — tái sử dụng nguyên kỹ thuật, áp dụng vào bối cảnh mới."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [vh.rollback-previous-version]
requires: [vh.environment-staging-prod]
concepts: [vh.rollback-previous-version]
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
Deploy MỘT phiên bản MỚI (bài 6) — pipeline ĐÃ qua, NHƯNG production
VẪN CÓ lỗi (test KHÔNG bắt được MỌI thứ). Làm sao QUAY LẠI?
::::

::::explain{#rollback-la-undo}
Rollback = quay VỀ phiên bản **NGAY TRƯỚC ĐÓ** — CHÍNH XÁC LÀ kỹ
thuật `undo` ĐÃ học T5.4 bài 11: lưu **SNAPSHOT** lịch sử deploy,
KHÔNG cần "tính NGHỊCH ĐẢO" của MỘT lần deploy (KHÔNG "gỡ" phiên
bản MỚI — CHỈ đơn giản CHUYỂN sang phiên bản CŨ):

```typescript title=readonly
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

let lichSu: readonly string[] = ["v1.0.0"];
lichSu = deploy(lichSu, "v1.1.0");
lichSu = deploy(lichSu, "v1.2.0");
console.log(phienBanHienTai(lichSu));
lichSu = rollback(lichSu);
console.log(phienBanHienTai(lichSu));
```

```text title=readonly
v1.2.0
v1.1.0
```

`deploy` ĐẨY THÊM phiên bản MỚI vào CUỐI `lichSu` (bất biến, giống
`apDungLenhCoLichSu` T5.4 bài 11). `rollback` CẮT phần tử CUỐI —
`v1.0.0` VÀ `v1.1.0` VẪN CÒN NGUYÊN trong `lichSu`, KHÔNG bị XOÁ,
CHỈ "con trỏ hiện tại" LÙI lại.
::::

::::example{#khong-rollback-qua-phien-ban-dau}
GIỐNG `undo` (T5.4 bài 11), `rollback` KHÔNG THỂ đi QUA phiên bản
**ĐẦU TIÊN** — luôn PHẢI CÓ ít NHẤT một phiên bản đang chạy:

```typescript title=readonly
function rollback(lichSu: readonly string[]): readonly string[] {
  if (lichSu.length <= 1) return lichSu;
  return lichSu.slice(0, lichSu.length - 1);
}
function phienBanHienTai(lichSu: readonly string[]): string {
  return lichSu[lichSu.length - 1] ?? "chua-co-gi";
}

let lichSu: readonly string[] = ["v1.0.0"];
lichSu = rollback(lichSu);
console.log(phienBanHienTai(lichSu));
console.log(lichSu.length);
```

```text title=readonly
v1.0.0
1
```

CHỈ CÓ ĐÚNG MỘT phiên bản (`v1.0.0`, CHƯA từng deploy thêm GÌ) —
`rollback` KHÔNG làm gì (`lichSu.length <= 1` chặn LẠI), `lichSu`
GIỮ NGUYÊN. Production KHÔNG BAO GIỜ được PHÉP "KHÔNG có phiên bản
nào đang chạy".
::::

::::predict{#doan-hai-lan-rollback-lien-tiep commitOnce}
```typescript
function rollback(lichSu: readonly string[]): readonly string[] {
  if (lichSu.length <= 1) return lichSu;
  return lichSu.slice(0, lichSu.length - 1);
}
function phienBanHienTai(lichSu: readonly string[]): string {
  return lichSu[lichSu.length - 1] ?? "chua-co-gi";
}
let lichSu: readonly string[] = ["v1", "v2", "v3", "v4"];
lichSu = rollback(lichSu);
lichSu = rollback(lichSu);
console.log(phienBanHienTai(lichSu));
```

`lichSu` BẮT ĐẦU với BỐN phiên bản, ĐANG Ở `"v4"`. `rollback` được
gọi HAI LẦN LIÊN TIẾP. Dòng cuối in ra gì?

:::opt{correct}
`v2`
:::

:::opt
`v3` — vì `rollback` CHỈ được gọi "về mặt Ý ĐỊNH" một LẦN DUY NHẤT
(gọi HAI lần LIÊN TIẾP TRÊN CÙNG một biến `lichSu` chỉ TÍNH LÀ MỘT
thao tác rollback THẬT SỰ, KHÔNG PHẢI hai bước lùi RIÊNG BIỆT)
::why
Gần đúng ở việc bạn nhớ ĐÚNG `"v3"` LÀ phiên bản NGAY TRƯỚC `"v4"` —
quan sát ĐÓ về THỨ TỰ CÁC phiên bản CHÍNH XÁC.

Chỗ lệch: MỖI lời gọi `rollback(lichSu)` LÀ MỘT thao tác **ĐỘC LẬP**
— KHÔNG có khái niệm "gọi liên tiếp CHỈ tính MỘT lần". Lần GỌI ĐẦU:
`lichSu` TỪ `["v1","v2","v3","v4"]` CẮT còn `["v1","v2","v3"]`
(hiện TẠI `"v3"`). Lần GỌI THỨ HAI (TRÊN `lichSu` MỚI ĐÓ): CẮT TIẾP
còn `["v1","v2"]` (hiện TẠI `"v2"`). HAI lần gọi = LÙI **HAI** bước,
KHÔNG PHẢI MỘT.
::
:::

:::opt
Máy báo lỗi lúc chạy — gọi `rollback(lichSu)` HAI LẦN LIÊN TIẾP TRÊN
CÙNG một biến (`lichSu = rollback(lichSu); lichSu = rollback(lichSu);`)
KHIẾN TypeScript phát hiện GÁN LẠI biến `readonly string[]` HAI LẦN
LÀ KHÔNG hợp lệ
::why
Gần đúng ở việc bạn để ý `lichSu` được GÁN LẠI (`=`) HAI LẦN LIÊN
TIẾP — một quan sát ĐÚNG về CẤU TRÚC code.

Chỗ lệch: `readonly string[]` (đã học T5.4 bài 11) CHỈ chặn việc
**MUTATE** MẢNG (như `.push()`) — HOÀN TOÀN KHÔNG chặn việc GÁN LẠI
BIẾN `lichSu` (được khai bằng `let`, KHÔNG PHẢI `const`) BẰNG một
GIÁ TRỊ MẢNG MỚI. Gán lại `let` BAO NHIÊU LẦN TUỲ Ý ĐỀU hợp lệ —
biên dịch VÀ chạy đều SẠCH.
::
:::
::::

::::code{#viet_deploy_rollback}
Hoàn thiện `deploy` (thêm phiên bản MỚI vào lịch sử) VÀ `rollback`
(cắt phiên bản CUỐI, GIỮ NGUYÊN nếu chỉ còn MỘT).

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function deploy(lichSu: readonly string[], phienBan: string): readonly string[] {
  return ___;
}

function rollback(lichSu: readonly string[]): readonly string[] {
  if (lichSu.length <= 1) return lichSu;
  return ___;
}

function phienBanHienTai(lichSu: readonly string[]): string {
  return lichSu[lichSu.length - 1] ?? "chua-co-gi";
}

let lichSu1: readonly string[] = ["v1.0.0"];
lichSu1 = deploy(lichSu1, "v1.1.0");
assertEqual(phienBanHienTai(lichSu1), "v1.1.0", "deploy them phien ban moi");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
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

let lichSu1: readonly string[] = ["v1.0.0"];
lichSu1 = deploy(lichSu1, "v1.1.0");
assertEqual(phienBanHienTai(lichSu1), "v1.1.0", "deploy them phien ban moi");
```

```typescript title=test
lichSu1 = rollback(lichSu1);
assertEqual(phienBanHienTai(lichSu1), "v1.0.0", "rollback ve phien ban truoc");
lichSu1 = rollback(lichSu1);
assertEqual(phienBanHienTai(lichSu1), "v1.0.0", "rollback tren phien ban dau -- khong doi");
assertEqual(lichSu1.length, 1, "khong the rollback qua phien ban dau tien");

let lichSu2: readonly string[] = ["a"];
lichSu2 = deploy(lichSu2, "b");
lichSu2 = deploy(lichSu2, "c");
lichSu2 = deploy(lichSu2, "d");
assertEqual(lichSu2.length, 4, "bon phien ban sau ba lan deploy");
lichSu2 = rollback(lichSu2);
lichSu2 = rollback(lichSu2);
assertEqual(phienBanHienTai(lichSu2), "b", "hai lan rollback tu d ve b");
```

:::hints
- kind: attention
  body: "deploy: THÊM phienBan vào CUỐI lichSu (mảng mới, giữ nguyên các mục cũ). rollback: CẮT mục cuối, giữ lại phần còn lại."
- kind: strategy
  body: "[...lichSu, phienBan] : lichSu.slice(0, lichSu.length - 1) — mảng mới nối thêm phần tử, và mảng mới bỏ phần tử cuối."
- kind: one-line
  body: '___ (deploy) = [...lichSu, phienBan]\n___ (rollback) = lichSu.slice(0, lichSu.length - 1)'
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
Rollback = undo, áp dụng vào lịch sử deploy. Bài chốt cụm: ghép gate
+ môi trường + rollback thành MỘT hệ hoàn chỉnh.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`xetDuyetDeploy` (bài 5), `xacDinhMoiTruong` (bài 6), VÀ
`deploy`/`rollback` (bài NÀY) hiện LÀ BA mảnh RIÊNG. Ghép CẢ BA vào
MỘT quy trình DUY NHẤT trông như THẾ NÀO?
::::

::::checkpoint{mastery=0.8}
::::
