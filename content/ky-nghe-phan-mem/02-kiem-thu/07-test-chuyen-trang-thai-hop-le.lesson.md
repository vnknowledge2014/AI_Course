---
id: ky-nghe-phan-mem.kiem-thu.test-chuyen-trang-thai-hop-le
title: "Test chuyển trạng thái HỢP LỆ — máy trạng thái là hàm thuần"
summary: "transition(trangThaiHienTai, suKien): KetQuaChuyen là MỘT hàm THUẦN — test GIỐNG HỆT mọi hàm thuần khác: input CỤ THỂ, output MONG ĐỢI, MỘT assertion mỗi cặp (trạng thái, sự kiện). MỖI đường chuyển HỢP LỆ trong sơ đồ trạng thái LÀ MỘT assertion riêng."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kt.test-valid-transitions]
requires: [kt.test-result-branches]
concepts: [kt.test-valid-transitions]
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
Một đơn hàng: nhập → chờ thanh toán → đã thanh toán → đã giao → đã
nhận. "Máy trạng thái" NÀY test có gì KHÁC một hàm bình thường?
::::

::::explain{#transition-la-ham-thuan}
`transition(trangThaiHienTai, suKien): KetQuaChuyen` LÀ MỘT **hàm
THUẦN** — test GIỐNG HỆT mọi hàm thuần khác đã test XUYÊN SUỐT track
NÀY: input CỤ THỂ, output MONG ĐỢI, MỘT assertion MỖI cặp (trạng
thái, sự kiện):

```typescript
type TrangThai = "nhap" | "cho_thanh_toan" | "da_thanh_toan" | "da_giao" | "da_nhan" | "da_huy";
type SuKien = "xac_nhan" | "thanh_toan" | "giao_hang" | "nhan_hang" | "huy";
type KetQuaChuyen = { tag: "ok"; trangThaiMoi: TrangThai } | { tag: "loi" };

function transition(trangThaiHienTai: TrangThai, suKien: SuKien): KetQuaChuyen {
  if (trangThaiHienTai === "nhap" && suKien === "xac_nhan") return { tag: "ok", trangThaiMoi: "cho_thanh_toan" };
  if (trangThaiHienTai === "cho_thanh_toan" && suKien === "thanh_toan") return { tag: "ok", trangThaiMoi: "da_thanh_toan" };
  if (trangThaiHienTai === "da_thanh_toan" && suKien === "giao_hang") return { tag: "ok", trangThaiMoi: "da_giao" };
  return { tag: "loi" };
}

function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

assertDeepEqual(transition("nhap", "xac_nhan"), { tag: "ok", trangThaiMoi: "cho_thanh_toan" }, "nhap + xac_nhan");
assertDeepEqual(transition("cho_thanh_toan", "thanh_toan"), { tag: "ok", trangThaiMoi: "da_thanh_toan" }, "cho_thanh_toan + thanh_toan");
```

```text
[PASS] nhap + xac_nhan
[PASS] cho_thanh_toan + thanh_toan
```

KHÔNG có gì "bí ẩn" — `transition` NHẬN hai giá trị, TRẢ VỀ một
object, KHÔNG side-effect, KHÔNG thời gian, KHÔNG ngẫu nhiên — CHÍNH
XÁC dạng hàm ĐÃ test từ bài 3. "Máy trạng thái" CHỈ LÀ MỘT CÁCH GỌI
hàm thuần có bảng ánh xạ (trạng thái, sự kiện) → trạng thái MỚI.
::::

::::example{#moi-duong-chuyen-mot-assertion-rieng}
"Sơ đồ trạng thái" (state diagram) của đơn hàng CÓ **NĂM** đường
chuyển HỢP LỆ. MỖI đường LÀ MỘT assertion RIÊNG — bỏ SÓT MỘT đường
nghĩa LÀ đường ĐÓ hoàn toàn KHÔNG được kiểm:

```typescript title=readonly
type TrangThai = "nhap" | "cho_thanh_toan" | "da_thanh_toan" | "da_giao" | "da_nhan" | "da_huy";
type SuKien = "xac_nhan" | "thanh_toan" | "giao_hang" | "nhan_hang" | "huy";
type KetQuaChuyen = { tag: "ok"; trangThaiMoi: TrangThai } | { tag: "loi" };

// BUG: nhánh "da_giao" + "nhan_hang" bị viết NHẦM thành "da_thanh_toan"
function transitionCoBug(trangThaiHienTai: TrangThai, suKien: SuKien): KetQuaChuyen {
  if (trangThaiHienTai === "nhap" && suKien === "xac_nhan") return { tag: "ok", trangThaiMoi: "cho_thanh_toan" };
  if (trangThaiHienTai === "cho_thanh_toan" && suKien === "thanh_toan") return { tag: "ok", trangThaiMoi: "da_thanh_toan" };
  if (trangThaiHienTai === "da_thanh_toan" && suKien === "giao_hang") return { tag: "ok", trangThaiMoi: "da_giao" };
  if (trangThaiHienTai === "da_giao" && suKien === "nhan_hang") return { tag: "ok", trangThaiMoi: "da_thanh_toan" }; // SAI
  return { tag: "loi" };
}

function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

// Nếu CHỈ test ba đường ĐẦU -- bug Ở đường thứ TƯ KHÔNG BỊ phát hiện
assertDeepEqual(transitionCoBug("nhap", "xac_nhan"), { tag: "ok", trangThaiMoi: "cho_thanh_toan" }, "duong 1");
assertDeepEqual(transitionCoBug("cho_thanh_toan", "thanh_toan"), { tag: "ok", trangThaiMoi: "da_thanh_toan" }, "duong 2");
assertDeepEqual(transitionCoBug("da_thanh_toan", "giao_hang"), { tag: "ok", trangThaiMoi: "da_giao" }, "duong 3");
// CHỈ khi test RIÊNG đường thứ TƯ mới lộ ra:
try {
  assertDeepEqual(transitionCoBug("da_giao", "nhan_hang"), { tag: "ok", trangThaiMoi: "da_nhan" }, "duong 4");
} catch (e) {
  console.log((e as Error).message);
}
```

```text title=readonly
[PASS] duong 1
[PASS] duong 2
[PASS] duong 3
[FAIL] duong 4: mong {"tag":"ok","trangThaiMoi":"da_nhan"}, nhan {"tag":"ok","trangThaiMoi":"da_thanh_toan"}
```

BA assertion ĐẦU ĐỀU `[PASS]` — bug Ở đường THỨ TƯ **VẪN ẨN** cho tới
khi CÓ assertion RIÊNG kiểm ĐÚNG đường ĐÓ. MỖI cặp (trạng thái, sự
kiện) LÀ MỘT tình huống ĐỘC LẬP, KHÔNG "suy ra" được từ CÁC tình
huống khác.
::::

::::predict{#doan-transition-tra-ve-object-gi commitOnce}
```typescript
type TrangThai = "nhap" | "cho_thanh_toan" | "da_thanh_toan" | "da_giao" | "da_nhan" | "da_huy";
type SuKien = "xac_nhan" | "thanh_toan" | "giao_hang" | "nhan_hang" | "huy";
type KetQuaChuyen = { tag: "ok"; trangThaiMoi: TrangThai } | { tag: "loi" };

function transition(trangThaiHienTai: TrangThai, suKien: SuKien): KetQuaChuyen {
  if (trangThaiHienTai === "nhap" && suKien === "huy") return { tag: "ok", trangThaiMoi: "da_huy" };
  if (trangThaiHienTai === "nhap" && suKien === "xac_nhan") return { tag: "ok", trangThaiMoi: "cho_thanh_toan" };
  return { tag: "loi" };
}

const ketQua = transition("nhap", "huy");
console.log(ketQua.tag === "ok" ? ketQua.trangThaiMoi : "khong co trang thai moi");
```

Dòng cuối in ra gì?

:::opt{correct}
`da_huy`
:::

:::opt
`khong co trang thai moi` — vì `transition` kiểm ĐIỀU KIỆN `"nhap" +
"xac_nhan"` TRƯỚC trong THÂN hàm, và `suKien` truyền vào LÀ `"huy"`
(KHÔNG PHẢI `"xac_nhan"`), nên hàm rơi VÀO nhánh `return { tag: "loi"
}` Ở CUỐI
::why
Gần đúng ở việc bạn đọc ĐÚNG rằng CÓ hai điều kiện `if` khác nhau
trong thân hàm — một quan sát ĐÚNG về CẤU TRÚC code.

Chỗ lệch: THỨ TỰ trong ĐOẠN code NÀY đặt điều kiện `"nhap" + "huy"`
**TRƯỚC** (dòng 1), điều kiện `"nhap" + "xac_nhan"` sau (dòng 2) —
KHÁC với đoạn code Ở phần "explain" TRƯỚC ĐÓ (chỉ CÓ nhánh
`"xac_nhan"`, không CÓ nhánh `"huy"`). Với `transition("nhap",
"huy")`: điều kiện DÒNG 1 (`trangThaiHienTai === "nhap" && suKien ===
"huy"`) khớp NGAY — hàm trả `{ tag: "ok", trangThaiMoi: "da_huy" }`
NGAY LẬP TỨC, KHÔNG chạy tới dòng 2. `ketQua.tag === "ok"` LÀ `true`,
in `ketQua.trangThaiMoi` = `"da_huy"`.
::
:::

:::opt
Máy báo lỗi biên dịch — biểu thức `ketQua.tag === "ok" ?
ketQua.trangThaiMoi : "..."` không hợp lệ, vì TypeScript KHÔNG THỂ
BIẾT tại nhánh `true` của toán tử ba ngôi, `ketQua` chắc chắn CÓ
field `trangThaiMoi`
::why
Gần đúng ở việc bạn nghĩ tới việc `ketQua` CÓ kiểu HỢP (union) —
`KetQuaChuyen` ĐÚNG LÀ union CỦA hai HÌNH DẠNG object khác nhau, một
quan sát ĐÚNG.

Chỗ lệch: TypeScript **CÓ THỂ** — đây CHÍNH LÀ "type narrowing" (thu
hẹp kiểu) ĐÃ dùng xuyên suốt track FP trước: kiểm tra `ketQua.tag ===
"ok"` LÀ một **type guard** trên field phân biệt (`tag`) của union —
TRONG nhánh `true` của toán tử ba ngôi, TypeScript TỰ ĐỘNG thu hẹp
`ketQua` VỀ ĐÚNG variant `{ tag: "ok"; trangThaiMoi: TrangThai }`,
CHO PHÉP truy cập `trangThaiMoi` AN TOÀN. Biên dịch sạch.
::
:::
::::

::::code{#viet_test_chuyen_hop_le}
Cho `transition` ĐÃ cài đặt sẵn (đầy đủ). Tự viết các assertion cho
BỐN đường chuyển hợp lệ trong CHUỖI thanh toán.

```typescript title=starter
type TrangThai = "nhap" | "cho_thanh_toan" | "da_thanh_toan" | "da_giao" | "da_nhan" | "da_huy";
type SuKien = "xac_nhan" | "thanh_toan" | "giao_hang" | "nhan_hang" | "huy";
type KetQuaChuyen = { tag: "ok"; trangThaiMoi: TrangThai } | { tag: "loi" };

function transition(trangThaiHienTai: TrangThai, suKien: SuKien): KetQuaChuyen {
  if (trangThaiHienTai === "nhap" && suKien === "xac_nhan") return { tag: "ok", trangThaiMoi: "cho_thanh_toan" };
  if (trangThaiHienTai === "nhap" && suKien === "huy") return { tag: "ok", trangThaiMoi: "da_huy" };
  if (trangThaiHienTai === "cho_thanh_toan" && suKien === "thanh_toan") return { tag: "ok", trangThaiMoi: "da_thanh_toan" };
  if (trangThaiHienTai === "cho_thanh_toan" && suKien === "huy") return { tag: "ok", trangThaiMoi: "da_huy" };
  if (trangThaiHienTai === "da_thanh_toan" && suKien === "giao_hang") return { tag: "ok", trangThaiMoi: "da_giao" };
  if (trangThaiHienTai === "da_giao" && suKien === "nhan_hang") return { tag: "ok", trangThaiMoi: "da_nhan" };
  return { tag: "loi" };
}

function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

assertDeepEqual(transition("nhap", "xac_nhan"), { tag: "ok", trangThaiMoi: ___ }, "nhap + xac_nhan");
assertDeepEqual(transition("cho_thanh_toan", "thanh_toan"), { tag: "ok", trangThaiMoi: ___ }, "cho_thanh_toan + thanh_toan");
assertDeepEqual(transition("da_thanh_toan", "giao_hang"), { tag: "ok", trangThaiMoi: ___ }, "da_thanh_toan + giao_hang");
assertDeepEqual(transition("da_giao", "nhan_hang"), { tag: "ok", trangThaiMoi: ___ }, "da_giao + nhan_hang");
```

```typescript title=solution
type TrangThai = "nhap" | "cho_thanh_toan" | "da_thanh_toan" | "da_giao" | "da_nhan" | "da_huy";
type SuKien = "xac_nhan" | "thanh_toan" | "giao_hang" | "nhan_hang" | "huy";
type KetQuaChuyen = { tag: "ok"; trangThaiMoi: TrangThai } | { tag: "loi" };

function transition(trangThaiHienTai: TrangThai, suKien: SuKien): KetQuaChuyen {
  if (trangThaiHienTai === "nhap" && suKien === "xac_nhan") return { tag: "ok", trangThaiMoi: "cho_thanh_toan" };
  if (trangThaiHienTai === "nhap" && suKien === "huy") return { tag: "ok", trangThaiMoi: "da_huy" };
  if (trangThaiHienTai === "cho_thanh_toan" && suKien === "thanh_toan") return { tag: "ok", trangThaiMoi: "da_thanh_toan" };
  if (trangThaiHienTai === "cho_thanh_toan" && suKien === "huy") return { tag: "ok", trangThaiMoi: "da_huy" };
  if (trangThaiHienTai === "da_thanh_toan" && suKien === "giao_hang") return { tag: "ok", trangThaiMoi: "da_giao" };
  if (trangThaiHienTai === "da_giao" && suKien === "nhan_hang") return { tag: "ok", trangThaiMoi: "da_nhan" };
  return { tag: "loi" };
}

function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

assertDeepEqual(transition("nhap", "xac_nhan"), { tag: "ok", trangThaiMoi: "cho_thanh_toan" }, "nhap + xac_nhan");
assertDeepEqual(transition("cho_thanh_toan", "thanh_toan"), { tag: "ok", trangThaiMoi: "da_thanh_toan" }, "cho_thanh_toan + thanh_toan");
assertDeepEqual(transition("da_thanh_toan", "giao_hang"), { tag: "ok", trangThaiMoi: "da_giao" }, "da_thanh_toan + giao_hang");
assertDeepEqual(transition("da_giao", "nhan_hang"), { tag: "ok", trangThaiMoi: "da_nhan" }, "da_giao + nhan_hang");
```

```typescript title=test
assertDeepEqual(transition("nhap", "huy"), { tag: "ok", trangThaiMoi: "da_huy" }, "nhap + huy phai ra da_huy, KHONG PHAI mot trang thai khac");
assertDeepEqual(transition("cho_thanh_toan", "huy"), { tag: "ok", trangThaiMoi: "da_huy" }, "cho_thanh_toan + huy phai ra da_huy");
assertDeepEqual(transition("nhap", "giao_hang"), { tag: "loi" }, "nhap khong the nhay thang toi giao_hang, phai bi tu choi");
```

:::hints
- kind: attention
  body: "Mỗi blank là trạng thái ĐÍCH của đúng đường chuyển đang được test — nhìn tên biến trạng thái hiện tại VÀ tên sự kiện trong lời gọi transition() để xác định trạng thái tiếp theo trong chuỗi thanh toán."
- kind: strategy
  body: 'Bốn blank theo đúng chuỗi thanh toán: cho_thanh_toan → da_thanh_toan → da_giao → da_nhan.'
- kind: one-line
  body: '___ (1) = "cho_thanh_toan"\n___ (2) = "da_thanh_toan"\n___ (3) = "da_giao"\n___ (4) = "da_nhan"'
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
Máy trạng thái = hàm thuần. Mỗi đường chuyển HỢP LỆ = một assertion
riêng. Bước tiếp theo: các đường KHÔNG hợp lệ cũng cần test.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đơn hàng KHÔNG THỂ nhảy thẳng từ "nhập" sang "đã giao" — bỏ qua các
bước trung gian. Trạng thái "đã hủy"/"đã nhận" là trạng thái CUỐI —
không còn chuyển đi đâu được nữa. Test các trường hợp NÀY như thế
nào?
::::

::::checkpoint{mastery=0.8}
::::
