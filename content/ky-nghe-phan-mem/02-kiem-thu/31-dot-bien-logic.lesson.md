---
id: ky-nghe-phan-mem.kiem-thu.dot-bien-logic
title: "Đột biến LOGIC — && thành ||, phủ định bị XOÁ"
summary: "Đổi toán tử logic (&&↔||) hoặc xoá một phủ định (!dieuKien → dieuKien) — giết được CHỈ khi có test cho TỪNG TỔ HỢP giá trị boolean của từng điều kiện con (đúng-đúng, đúng-sai, sai-đúng, sai-sai cho && / || hai điều kiện)."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 31
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.mutation-logical]
requires: [kt.mutation-arithmetic]
concepts: [kt.mutation-logical]
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
`duocPhepMua(coTien, duTuoi)` trả `coTien && duTuoi` — đổi `&&`
thành `||` — bộ test CHỈ thử (true,true)/(false,false) giết được không?
::::

::::explain{#dot-bien-logic-can-du-to-hop}
Đổi toán tử LOGIC (`&&`↔`||`) HOẶC XOÁ MỘT phủ định (`!dieuKien` →
`dieuKien`) — GIẾT được **CHỈ khi** có test CHO **TỪNG TỔ HỢP** giá
trị boolean CỦA từng điều kiện CON (đúng-đúng, đúng-sai, sai-đúng,
sai-sai CHO `&&`/`||` HAI điều kiện):

```typescript
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function duocPhepMua(coTien: boolean, duTuoi: boolean): boolean {
  return coTien && duTuoi;
}

// bo test CHI thu HAI to hop
assertEqual(duocPhepMua(true, true), true, "co tien va du tuoi -> duoc mua");
assertEqual(duocPhepMua(false, false), false, "khong tien va chua du tuoi -> khong duoc mua");
```

```text
[PASS] co tien va du tuoi -> duoc mua
[PASS] khong tien va chua du tuoi -> khong duoc mua
```

BỘ test NÀY CHỈ thử HAI trong **BỐN** tổ HỢP CÓ THỂ của HAI biến
boolean — BỎ SÓT `(true, false)` VÀ `(false, true)`.
::::

::::example{#dot-bien-song-sot-thieu-to-hop}
Đổi `&&` thành `||` RỒI chạy LẠI **ĐÚNG** hai TỔ HỢP TRÊN — VẪN
`[PASS]` (VÌ `true && true` = `true || true` = `true`, VÀ `false &&
false` = `false || false` = `false` — HAI tổ hợp NÀY KHÔNG PHÂN BIỆT
`&&` với `||`):

```typescript title=readonly
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

// DOT BIEN: && thanh ||
function duocPhepMuaDotBien(coTien: boolean, duTuoi: boolean): boolean {
  return coTien || duTuoi;
}

try {
  assertEqual(duocPhepMuaDotBien(true, true), true, "co tien va du tuoi -> duoc mua");
  assertEqual(duocPhepMuaDotBien(false, false), false, "khong tien va chua du tuoi -> khong duoc mua");
  console.log("DOT BIEN SONG SOT voi hai to hop cu");
} catch (e) {
  console.log("bi giet:", (e as Error).message);
}

// CHI khi THEM hai to hop CON LAI moi lo ra
try {
  assertEqual(duocPhepMuaDotBien(true, false), false, "co tien nhung CHUA du tuoi -> khong duoc mua");
} catch (e) {
  console.log("DOT BIEN BI GIET boi to hop (true,false):", (e as Error).message);
}
```

```text title=readonly
DOT BIEN SONG SOT voi hai to hop cu
DOT BIEN BI GIET boi to hop (true,false): [FAIL] co tien nhung CHUA du tuoi -> khong duoc mua: mong false, nhan true
```

`coTien || duTuoi` VỚI `(true, false)`: `true || false` = `true` —
NHƯNG NGHIỆP VỤ ĐÚNG (`coTien && duTuoi`) đòi HỎI `false` (CÓ tiền
NHƯNG **CHƯA** đủ tuổi VẪN KHÔNG được mua). CHỈ tổ HỢP `(true, false)`
(HOẶC `(false, true)`) MỚI phân BIỆT được `&&` với `||`.
::::

::::predict{#doan-phu-dinh-bi-xoa commitOnce}
```typescript
function coTheDangKy(daXacThuc: boolean): boolean {
  return !daXacThuc; // "chua xac thuc" -> DUOC dang ky moi
}

// DOT BIEN: phu dinh bi XOA
function coTheDangKyDotBien(daXacThuc: boolean): boolean {
  return daXacThuc;
}

console.log(coTheDangKy(true), coTheDangKyDotBien(true));
```

Dòng cuối in ra gì?

:::opt{correct}
`false true`
:::

:::opt
`false false` — vì `coTheDangKyDotBien` GẦN giống `coTheDangKy` (CHỈ
mất DẤU `!`), NÊN VỚI CÙNG input, HAI hàm VẪN có XU HƯỚNG cho ra kết
quả GẦN NHAU (CÙNG dấu HOẶC cùng loại)
::why
Gần đúng ở việc bạn nghĩ "MẤT một ký tự `!`" LÀ MỘT thay đổi NHỎ, NÊN
kết QUẢ CŨNG "gần giống" — MỘT trực giác về ĐỘ LỚN của THAY ĐỔI CÚ
PHÁP.

Chỗ lệch: PHỦ ĐỊNH (`!`) LÀ MỘT toán tử **ĐẢO NGƯỢC HOÀN TOÀN** giá
trị boolean — XOÁ nó KHÔNG PHẢI "thay đổi NHỎ" VỀ MẶT hành VI, MÀ LÀ
**ĐẢO NGƯỢC HOÀN TOÀN kết quả** (giống việc ĐẢO CHIỀU MỘT công tắc
— "nhỏ" VỀ mặt CÚ PHÁP, "TOÀN DIỆN" VỀ mặt Ý NGHĨA). `coTheDangKy
(true)`: `!true` = `false`. `coTheDangKyDotBien(true)`: TRẢ THẲNG
`true` (KHÔNG phủ định). HAI kết QUẢ **ĐỐI LẬP HOÀN TOÀN**: `false`
VÀ `true`.
::
:::

:::opt
Máy báo lỗi biên dịch — `coTheDangKyDotBien(daXacThuc: boolean):
boolean { return daXacThuc; }` không hợp lệ, vì HÀM khai kiểu trả VỀ
`boolean` NHƯNG THÂN hàm CHỈ trả LẠI THẲNG tham số ĐẦU VÀO, TypeScript
CẤM MỘT hàm "không LÀM GÌ" (identity function) khai kiểu trả VỀ RÕ RÀNG
::why
Gần đúng ở việc bạn để ý `coTheDangKyDotBien` "KHÔNG LÀM GÌ" VỚI
`daXacThuc` (CHỈ trả LẠI CHÍNH nó) — một quan sát ĐÚNG rằng THÂN hàm
NÀY rất ĐƠN GIẢN.

Chỗ lệch: TypeScript **HOÀN TOÀN CHO PHÉP** MỘT hàm "identity" (trả
LẠI CHÍNH tham số, KHÔNG biến ĐỔI GÌ) — ĐÂY LÀ MỘT MẪU HÀM HỢP LỆ VÀ
PHỔ BIẾN (ví dụ trong LẬP trình HÀM). Kiểu tham số (`boolean`) khớp
HOÀN TOÀN kiểu trả VỀ khai báo (`boolean`). Biên dịch sạch.
::
:::
::::

::::code{#viet_test_dot_bien_logic}
Cho `duocPhepMua` ĐÃ cài đặt sẵn. Tự viết THÊM assertion cho HAI tổ
hợp CÒN LẠI để giết đột biến `&&`↔`||`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function duocPhepMua(coTien: boolean, duTuoi: boolean): boolean {
  return coTien && duTuoi;
}

assertEqual(duocPhepMua(true, true), true, "co tien va du tuoi -> duoc mua");
assertEqual(duocPhepMua(false, false), false, "khong tien va chua du tuoi -> khong duoc mua");
assertEqual(duocPhepMua(true, false), ___, "co tien nhung CHUA du tuoi -> khong duoc mua");
assertEqual(duocPhepMua(false, true), ___, "du tuoi nhung KHONG co tien -> khong duoc mua");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function duocPhepMua(coTien: boolean, duTuoi: boolean): boolean {
  return coTien && duTuoi;
}

assertEqual(duocPhepMua(true, true), true, "co tien va du tuoi -> duoc mua");
assertEqual(duocPhepMua(false, false), false, "khong tien va chua du tuoi -> khong duoc mua");
assertEqual(duocPhepMua(true, false), false, "co tien nhung CHUA du tuoi -> khong duoc mua");
assertEqual(duocPhepMua(false, true), false, "du tuoi nhung KHONG co tien -> khong duoc mua");
```

```typescript title=test
// mo phong CHINH dot bien && thanh || -- neu blank sai, cac dong nay se PHAT HIEN
function duocPhepMuaDotBien(coTien: boolean, duTuoi: boolean): boolean {
  return coTien || duTuoi;
}

let gietToHop1 = false;
try {
  assertEqual(duocPhepMuaDotBien(true, false), false, "kiem dot bien || tren to hop (true,false)");
} catch {
  gietToHop1 = true;
}
if (!gietToHop1) throw new Error("bo test PHAI giet duoc dot bien && -> || (can to hop true,false)");

let gietToHop2 = false;
try {
  assertEqual(duocPhepMuaDotBien(false, true), false, "kiem dot bien || tren to hop (false,true)");
} catch {
  gietToHop2 = true;
}
if (!gietToHop2) throw new Error("bo test PHAI giet duoc dot bien && -> || (can to hop false,true)");

console.log("[PASS] bo test giet duoc dot bien logic voi ca bon to hop");
```

:::hints
- kind: attention
  body: "Với && (VÀ), CHỈ có (true, true) mới ra true — cả (true,false) LẪN (false,true) đều phải ra false. Đây là hai tổ hợp bộ test bài trước bỏ sót."
- kind: strategy
  body: 'false : false — cả hai tổ hợp còn lại đều phải trả về false với &&.'
- kind: one-line
  body: '___ (1) = false\n___ (2) = false'
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
Đột biến logic: cần đủ bốn tổ hợp mới giết được. Bước tiếp theo: đột
biến hoán đổi giá trị — field/hằng số cùng kiểu bị đổi chỗ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Một hàm ánh xạ hai chiều hoán đổi hai field cùng kiểu string (`ma`↔
`ten`) — test chỉ kiểm MỘT field có bắt được không?
::::

::::checkpoint{mastery=0.8}
::::
