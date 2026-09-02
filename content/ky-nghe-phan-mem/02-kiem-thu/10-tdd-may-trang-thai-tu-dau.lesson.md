---
id: ky-nghe-phan-mem.kiem-thu.tdd-may-trang-thai-tu-dau
title: "Capstone: TDD một máy trạng thái MỚI từ đặc tả THUẦN VĂN BẢN"
summary: "Bài chốt cụm 2: chỉ với đặc tả thuần văn bản (đèn giao thông: do → xanh → vang → do, mọi chuyển khác bị từ chối), viết assertion (Đỏ) TRƯỚC, rồi chuyenDen(trangThaiHienTai): Result<MauDen,'khong_hop_le'> (Xanh) — ghép trọn chu trình TDD (bài 1-9) vào MỘT bài toán MỚI."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kt.tdd-new-state-machine]
requires: [kt.test-full-workflow]
concepts: [kt.tdd-new-state-machine]
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
Bài chốt cụm 2. Đặc tả THUẦN VĂN BẢN — đèn giao thông: đỏ → xanh →
vàng → đỏ, MỌI chuyển khác bị từ chối. Ghép TRỌN chu trình đã học?
::::

::::explain{#dac-ta-thuan-van-ban}
Đặc tả (KHÔNG có code, KHÔNG có sơ đồ — CHỈ VĂN BẢN): "Đèn giao thông
có BA màu: đỏ (`do`), xanh (`xanh`), vàng (`vang`). Đèn LUÔN chuyển
theo CHU KỲ CỐ ĐỊNH: đỏ → xanh → vàng → đỏ → xanh → ... (lặp lại VÔ
HẠN, KHÔNG có sự kiện bên ngoài nào tác động — MỖI lần 'chuyển' luôn
đưa đèn tới màu TIẾP THEO trong chu kỳ)."

Bước **Đỏ**: viết assertion CHO hành vi MONG MUỐN **TRƯỚC**, trên MỘT
hàm CHƯA cài đặt:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

type MauDen = "do" | "xanh" | "vang";
type ChuyenDenLoi = "khong_hop_le";

// ĐỎ: chuyenDen CHƯA cài đặt
function chuyenDen(trangThaiHienTai: MauDen): Result<MauDen, ChuyenDenLoi> {
  throw new Error("chua cai dat");
}

try {
  assertDeepEqual(chuyenDen("do"), ok("xanh"), "do chuyen sang xanh");
} catch (e) {
  console.log("DO:", (e as Error).message);
}
```

```text
DO: chua cai dat
```

BA assertion (đỏ→xanh, xanh→vàng, vàng→đỏ) CÓ THỂ viết **TRỌN VẸN**
TỪ đặc tả VĂN BẢN, TRƯỚC KHI có DÒNG code THẬT nào cài đặt
`chuyenDen`. Đây LÀ TOÀN BỘ nội dung của "Đỏ": biến đặc tả THÀNH
assertion CỤ THỂ.
::::

::::example{#xanh-chuyenden-du-phong}
Bước **Xanh**: cài đặt TỐI THIỂU — `switch` trên BA giá trị hợp lệ,
KÈM một nhánh `default` **DỰ PHÒNG** (giá trị BẤT THƯỜNG lọt vào lúc
CHẠY, ví dụ dữ liệu ĐỌC từ bên NGOÀI KHÔNG qua kiểm kiểu):

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

type MauDen = "do" | "xanh" | "vang";
type ChuyenDenLoi = "khong_hop_le";

function chuyenDen(trangThaiHienTai: MauDen): Result<MauDen, ChuyenDenLoi> {
  switch (trangThaiHienTai) {
    case "do": return ok("xanh");
    case "xanh": return ok("vang");
    case "vang": return ok("do");
    default: return loi("khong_hop_le");
  }
}

assertDeepEqual(chuyenDen("do"), ok("xanh"), "do chuyen sang xanh");
assertDeepEqual(chuyenDen("xanh"), ok("vang"), "xanh chuyen sang vang");
assertDeepEqual(chuyenDen("vang"), ok("do"), "vang chuyen ve do, dong chu ky");

// gia tri bat thuong lot vao luc chay -- van phai bi tu choi
assertDeepEqual(chuyenDen("khong_ro" as MauDen), loi("khong_hop_le"), "gia tri la KHONG nam trong ba mau phai bi tu choi");
```

```text title=readonly
[PASS] do chuyen sang xanh
[PASS] xanh chuyen sang vang
[PASS] vang chuyen ve do, dong chu ky
[PASS] gia tri la KHONG nam trong ba mau phai bi tu choi
```

BA assertion "Đỏ" (Ở phần explain) GIỜ ĐỀU `[PASS]`, KHÔNG SỬA MỘT
assertion nào — CHỈ THÊM code CÀI ĐẶT. Nhánh `default` (dự phòng)
TƯỞNG như "không bao giờ chạy tới" (kiểu `MauDen` CHỈ có ba giá trị)
— NHƯNG dữ liệu THẬT (từ mạng, file, người dùng) KHÔNG PHẢI LUÔN đi
QUA kiểm kiểu TypeScript, nên vẫn PHẢI test RIÊNG nhánh NÀY (bài 8's
bài học: KHÔNG giả định "nhánh khó xảy ra thì không cần test").
::::

::::predict{#doan-ghep-hai-lan-chuyen commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type MauDen = "do" | "xanh" | "vang";
type ChuyenDenLoi = "khong_hop_le";

function chuyenDen(trangThaiHienTai: MauDen): Result<MauDen, ChuyenDenLoi> {
  switch (trangThaiHienTai) {
    case "do": return ok("xanh");
    case "xanh": return ok("vang");
    case "vang": return ok("do");
    default: return loi("khong_hop_le");
  }
}

const b1 = chuyenDen("do");
if (b1.kind !== "ok") throw new Error("khong the xay ra");
const b2 = chuyenDen(b1.giaTri);
console.log(b2.kind === "ok" ? b2.giaTri : "loi");
```

Dòng cuối in ra gì?

:::opt{correct}
`vang`
:::

:::opt
`do` — vì `chuyenDen` áp DỤNG **HAI LẦN** liên tiếp LÊN CÙNG giá trị
gốc (`"do"`), giống như GỌI `chuyenDen("do")` rồi `chuyenDen("do")`
LẦN NỮA — kết quả LẶP LẠI GIÁ TRỊ TRUNG GIAN ĐẦU (`"xanh"`), KHÔNG
TIẾN thêm bước NÀO trong chu kỳ
::why
Gần đúng ở việc bạn để ý CÓ **HAI** lời gọi `chuyenDen` trong đoạn
code — một quan sát ĐÚNG về SỐ LƯỢNG lời gọi.

Chỗ lệch: lời gọi THỨ HAI (`chuyenDen(b1.giaTri)`) KHÔNG gọi LẠI TRÊN
`"do"` — nó gọi TRÊN `b1.giaTri`, LÀ giá trị **KẾT QUẢ** của lời gọi
ĐẦU (`chuyenDen("do")` trả `ok("xanh")`, nên `b1.giaTri = "xanh"`).
Lời gọi THỨ HAI THỰC SỰ LÀ `chuyenDen("xanh")` — TIẾP TỤC chu kỳ, KHÔNG
LẶP LẠI đầu vào gốc. `chuyenDen("xanh")` trả `ok("vang")`, `b2.giaTri
= "vang"`.
::
:::

:::opt
Máy báo lỗi biên dịch — dòng `const b2 = chuyenDen(b1.giaTri);`
không hợp lệ, vì SAU `if (b1.kind !== "ok") throw ...`, TypeScript
VẪN coi `b1` LÀ kiểu `Result<MauDen, ChuyenDenLoi>` ĐẦY ĐỦ (union),
KHÔNG thu hẹp được, nên `b1.giaTri` không tồn tại trên MỌI nhánh
::why
Gần đúng ở việc bạn nhớ `b1` CÓ kiểu `Result<MauDen, ChuyenDenLoi>`
— MỘT UNION của hai HÌNH DẠNG object — một quan sát ĐÚNG về kiểu KHAI
BÁO của `b1`.

Chỗ lệch: `if (b1.kind !== "ok") throw ...` LÀ MỘT type guard TRÊN
field phân biệt `kind` — dòng NÀY **NÉM lỗi VÀ DỪNG** thực thi NẾU
`kind` KHÔNG PHẢI `"ok"`, nên MỌI dòng code SAU ĐÓ chỉ CÒN chạy khi
`b1.kind === "ok"` ĐÃ đúng — TypeScript **THU HẸP** `b1` VỀ ĐÚNG
variant `{ kind: "ok"; giaTri: MauDen }` từ dòng ĐÓ trở đi (giống HỆT
kỹ thuật "throw để thu hẹp" ĐÃ dùng xuyên suốt track FP TRƯỚC).
`b1.giaTri` truy cập AN TOÀN, kiểu `MauDen`. Biên dịch sạch.
::
:::
::::

::::code{#viet_chuyenden}
Cho đặc tả (Ở phần explain). Tự viết PHẦN CÒN THIẾU của `chuyenDen`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

type MauDen = "do" | "xanh" | "vang";
type ChuyenDenLoi = "khong_hop_le";

function chuyenDen(trangThaiHienTai: MauDen): Result<MauDen, ChuyenDenLoi> {
  switch (trangThaiHienTai) {
    case "do": return ok(___);
    case "xanh": return ok(___);
    case "vang": return ok(___);
    default: return loi("khong_hop_le");
  }
}

assertDeepEqual(chuyenDen("do"), ok("xanh"), "do chuyen sang xanh");
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

type MauDen = "do" | "xanh" | "vang";
type ChuyenDenLoi = "khong_hop_le";

function chuyenDen(trangThaiHienTai: MauDen): Result<MauDen, ChuyenDenLoi> {
  switch (trangThaiHienTai) {
    case "do": return ok("xanh");
    case "xanh": return ok("vang");
    case "vang": return ok("do");
    default: return loi("khong_hop_le");
  }
}

assertDeepEqual(chuyenDen("do"), ok("xanh"), "do chuyen sang xanh");
```

```typescript title=test
assertDeepEqual(chuyenDen("xanh"), ok("vang"), "xanh chuyen sang vang");
assertDeepEqual(chuyenDen("vang"), ok("do"), "vang chuyen ve do, dong chu ky");
assertDeepEqual(chuyenDen("khong_ro" as MauDen), loi("khong_hop_le"), "gia tri KHONG nam trong ba mau phai bi tu choi");

// chay tron mot chu ky ba buoc, phai quay lai "do"
let mau: MauDen = "do";
for (let i = 0; i < 3; i++) {
  const kq = chuyenDen(mau);
  if (kq.kind === "ok") mau = kq.giaTri;
}
assertDeepEqual(mau, "do", "sau dung ba buoc phai quay ve do, dung ba KHONG duoc dung hai hay bon");
```

:::hints
- kind: attention
  body: "Ba blank là MÀU TIẾP THEO trong chu kỳ đỏ → xanh → vàng → đỏ, tương ứng với case đang xét. Nhánh default (dự phòng) đã viết sẵn."
- kind: strategy
  body: 'case "do" → "xanh" ; case "xanh" → "vang" ; case "vang" → "do" — mỗi case trỏ tới MÀU KẾ TIẾP trong chu kỳ ba bước.'
- kind: one-line
  body: '___ (case "do") = "xanh"\n___ (case "xanh") = "vang"\n___ (case "vang") = "do"'
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
Cụm 2 hoàn tất: Đỏ→Xanh, đường hợp lệ, đường bị từ chối, cả luồng —
ghép trọn vào MỘT máy trạng thái MỚI, chỉ từ đặc tả văn bản. Cụm tiếp
theo: FP DI — fake là object thuần, không cần thư viện mock.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Một hàm nghiệp vụ THẬT (ví dụ `dangKyNguoiDung`) cần "gọi database" —
KHÔNG có database THẬT nào trong sandbox. Test hàm ĐÓ như thế nào,
KHÔNG dùng thư viện mock?
::::

::::checkpoint{mastery=0.8}
::::
