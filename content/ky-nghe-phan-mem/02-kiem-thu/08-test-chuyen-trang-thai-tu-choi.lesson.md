---
id: ky-nghe-phan-mem.kiem-thu.test-chuyen-trang-thai-tu-choi
title: "Test chuyển trạng thái BỊ TỪ CHỐI — bao gồm trạng thái CUỐI"
summary: "QUAN TRỌNG NGANG bài trước: test các chuyển đổi KHÔNG hợp lệ PHẢI bị TỪ CHỐI — không thể nhảy tắt qua bước trung gian. Trạng thái CUỐI (đã hủy, đã nhận) PHẢI từ chối MỌI sự kiện — test RIÊNG điều đó, không giả định 'không test lỗi thì chắc ổn'."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kt.test-invalid-transitions]
requires: [kt.test-valid-transitions]
concepts: [kt.test-invalid-transitions]
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
Bài trước test NĂM đường chuyển HỢP LỆ. Nếu KHÔNG test đường KHÔNG
hợp lệ — "không test lỗi thì chắc ổn"?
::::

::::explain{#tu-choi-quan-trong-ngang-hop-le}
Test các chuyển đổi **KHÔNG hợp lệ** PHẢI bị **TỪ CHỐI** (`tag:
"loi"`) QUAN TRỌNG **NGANG** test đường hợp lệ (bài 7) — KHÔNG PHẢI
"phụ". Đơn hàng `"nhap"` **KHÔNG THỂ** nhảy THẲNG sang `"da_giao"`
(bỏ qua các bước TRUNG GIAN: xác nhận, thanh toán):

```typescript
type TrangThai = "nhap" | "cho_thanh_toan" | "da_thanh_toan" | "da_giao" | "da_nhan" | "da_huy";
type SuKien = "xac_nhan" | "thanh_toan" | "giao_hang" | "nhan_hang" | "huy";
type KetQuaChuyen = { tag: "ok"; trangThaiMoi: TrangThai } | { tag: "loi" };

function transition(trangThaiHienTai: TrangThai, suKien: SuKien): KetQuaChuyen {
  if (trangThaiHienTai === "nhap" && suKien === "xac_nhan") return { tag: "ok", trangThaiMoi: "cho_thanh_toan" };
  if (trangThaiHienTai === "cho_thanh_toan" && suKien === "thanh_toan") return { tag: "ok", trangThaiMoi: "da_thanh_toan" };
  return { tag: "loi" };
}

function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

assertDeepEqual(transition("nhap", "giao_hang"), { tag: "loi" }, "nhap khong the nhay thang toi giao_hang");
assertDeepEqual(transition("nhap", "nhan_hang"), { tag: "loi" }, "nhap khong the nhay thang toi nhan_hang");
```

```text
[PASS] nhap khong the nhay thang toi giao_hang
[PASS] nhap khong the nhay thang toi nhan_hang
```

Nếu `transition` **NHẦM LẪN** cho phép `"nhap"` nhảy thẳng tới
`"da_giao"` (một BUG THẬT — bỏ qua bước xác nhận VÀ thanh toán), CHỈ
CÓ assertion NÀY (kiểm KẾT QUẢ **PHẢI LÀ** `{ tag: "loi" }`) MỚI bắt
được. KHÔNG test đường "hợp lệ" nào bắt được bug NÀY — chúng CHỈ kiểm
đường ĐÚNG có hoạt động, KHÔNG kiểm đường SAI có bị chặn.
::::

::::example{#trang-thai-cuoi-tu-choi-moi-su-kien}
Trạng thái **CUỐI** (`"da_huy"`, `"da_nhan"`) PHẢI từ chối **MỌI** sự
kiện — KHÔNG CHỈ một vài sự kiện "hợp lý":

```typescript title=readonly
type TrangThai = "nhap" | "cho_thanh_toan" | "da_thanh_toan" | "da_giao" | "da_nhan" | "da_huy";
type SuKien = "xac_nhan" | "thanh_toan" | "giao_hang" | "nhan_hang" | "huy";
type KetQuaChuyen = { tag: "ok"; trangThaiMoi: TrangThai } | { tag: "loi" };

// BUG: quên chặn "da_huy" + "thanh_toan" -- vô tình cho phép "thanh toán" đơn ĐÃ HỦY
function transitionCoBug(trangThaiHienTai: TrangThai, suKien: SuKien): KetQuaChuyen {
  if (trangThaiHienTai === "da_huy" && suKien === "thanh_toan") return { tag: "ok", trangThaiMoi: "da_thanh_toan" }; // SAI
  return { tag: "loi" };
}

function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

try {
  assertDeepEqual(transitionCoBug("da_huy", "thanh_toan"), { tag: "loi" }, "da_huy phai tu choi thanh_toan");
} catch (e) {
  console.log((e as Error).message);
}
```

```text title=readonly
[FAIL] da_huy phai tu choi thanh_toan: mong {"tag":"loi"}, nhan {"tag":"ok","trangThaiMoi":"da_thanh_toan"}
```

Nếu KHÔNG có test NÀY, một đơn hàng **ĐÃ HỦY** vẫn CÓ THỂ bị "thanh
toán" — một BUG NGHIỆP VỤ nghiêm trọng, ẩn HOÀN TOÀN nếu chỉ test
đường HỢP LỆ. Trạng thái cuối CẦN test RIÊNG cho **TỪNG** sự kiện có
thể xảy ra, KHÔNG chỉ một sự kiện "tiêu biểu".
::::

::::predict{#doan-trang-thai-cuoi-nhan-su-kien-khac commitOnce}
```typescript
type TrangThai = "nhap" | "cho_thanh_toan" | "da_thanh_toan" | "da_giao" | "da_nhan" | "da_huy";
type SuKien = "xac_nhan" | "thanh_toan" | "giao_hang" | "nhan_hang" | "huy";
type KetQuaChuyen = { tag: "ok"; trangThaiMoi: TrangThai } | { tag: "loi" };

function transition(trangThaiHienTai: TrangThai, suKien: SuKien): KetQuaChuyen {
  if (trangThaiHienTai === "da_giao" && suKien === "nhan_hang") return { tag: "ok", trangThaiMoi: "da_nhan" };
  return { tag: "loi" };
}

const ketQua = transition("da_nhan", "xac_nhan");
console.log(ketQua.tag);
```

Dòng cuối in ra gì?

:::opt{correct}
`loi`
:::

:::opt
`ok` — vì `"da_nhan"` LÀ trạng thái CUỐI CÙNG trong quy trình đơn
hàng (đơn ĐÃ hoàn tất), và CHUYỂN từ trạng thái CUỐI vẫn ĐƯỢC coi LÀ
"hợp lệ theo mặc định" khi KHÔNG khớp điều kiện NÀO — hàm sẽ hiểu ĐÓ
LÀ "không có gì cần làm", trả `ok`
::why
Gần đúng ở việc bạn nghĩ ĐÚNG `"da_nhan"` LÀ trạng thái CUỐI, "đơn
hàng ĐÃ hoàn tất" — một quan sát ĐÚNG về Ý NGHĨA nghiệp vụ.

Chỗ lệch: "trạng thái cuối" KHÔNG có nghĩa "mặc định coi LÀ hợp lệ"
— NGƯỢC LẠI, đó LÀ lý do CHÍNH XÁC tại sao PHẢI **TỪ CHỐI**. Nhìn vào
THÂN hàm: CHỈ CÓ MỘT điều kiện `if` (khớp `"da_giao"` + `"nhan_hang"`)
— `transition("da_nhan", "xac_nhan")` KHÔNG khớp điều kiện ĐÓ (trạng
thái hiện tại LÀ `"da_nhan"`, không PHẢI `"da_giao"`), nên rơi thẳng
VÀO `return { tag: "loi" }` Ở CUỐI. `ketQua.tag` LÀ `"loi"`.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `transition("da_nhan", "xac_nhan")`
không hợp lệ vì `"da_nhan"` là trạng thái CUỐI, TypeScript CẤM gọi
`transition` với trạng thái ĐÃ ở dạng cuối làm tham số ĐẦU
::why
Gần đúng ở việc bạn nhớ `"da_nhan"` LÀ MỘT trong SÁU giá trị hợp lệ
của kiểu `TrangThai` — một quan sát ĐÚNG rằng kiểu NÀY CÓ khái niệm
"trạng thái cuối".

Chỗ lệch: TypeScript **KHÔNG** phân biệt "trạng thái thường" VÀ
"trạng thái cuối" Ở TẦNG KIỂU — `TrangThai` CHỈ LÀ MỘT union PHẲNG
của sáu chuỗi, KHÔNG có RÀNG BUỘC nào cấm gọi `transition` với BẤT KỲ
giá trị `TrangThai` nào LÀM tham số ĐẦU. "Trạng thái cuối không còn
chuyển đi đâu được" LÀ MỘT quy tắc NGHIỆP VỤ, thực thi BÊN TRONG THÂN
hàm (như phân tích Ở phương án ĐÚNG) — KHÔNG PHẢI RÀNG BUỘC kiểu.
Biên dịch sạch.
::
:::
::::

::::code{#viet_test_chuyen_tu_choi}
Cho `transition` ĐÃ cài đặt sẵn (đầy đủ). Tự viết các assertion cho
chuyển đổi KHÔNG hợp lệ, gồm CẢ trạng thái cuối.

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

assertDeepEqual(transition("nhap", "giao_hang"), { tag: ___ }, "nhap khong the nhay thang toi giao_hang");
assertDeepEqual(transition("da_nhan", "xac_nhan"), { tag: ___ }, "da_nhan (trang thai cuoi) tu choi xac_nhan");
assertDeepEqual(transition("da_huy", "thanh_toan"), { tag: ___ }, "da_huy (trang thai cuoi) tu choi thanh_toan");
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

assertDeepEqual(transition("nhap", "giao_hang"), { tag: "loi" }, "nhap khong the nhay thang toi giao_hang");
assertDeepEqual(transition("da_nhan", "xac_nhan"), { tag: "loi" }, "da_nhan (trang thai cuoi) tu choi xac_nhan");
assertDeepEqual(transition("da_huy", "thanh_toan"), { tag: "loi" }, "da_huy (trang thai cuoi) tu choi thanh_toan");
```

```typescript title=test
assertDeepEqual(transition("nhap", "nhan_hang"), { tag: "loi" }, "nhap khong the nhay thang toi nhan_hang");
assertDeepEqual(transition("da_nhan", "huy"), { tag: "loi" }, "da_nhan phai tu choi CA su kien huy, khong chi xac_nhan");
assertDeepEqual(transition("da_huy", "xac_nhan"), { tag: "loi" }, "da_huy phai tu choi CA su kien xac_nhan, khong chi thanh_toan");
assertDeepEqual(transition("da_thanh_toan", "nhan_hang"), { tag: "loi" }, "da_thanh_toan khong the nhay thang toi nhan_hang, bo qua giao_hang");
```

:::hints
- kind: attention
  body: "Cả ba lời gọi transition() ở trên đều dùng cặp (trạng thái, sự kiện) KHÔNG khớp bất kỳ nhánh if nào trong thân hàm — kết quả luôn rơi vào nhánh cuối cùng."
- kind: strategy
  body: 'Cả ba blank đều CÙNG một giá trị — "loi" — vì KetQuaChuyen chỉ có hai khả năng cho field tag: "ok" hoặc "loi", và ở đây cả ba trường hợp đều bị từ chối.'
- kind: one-line
  body: '___ (cả ba) = "loi"'
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
Chuyển đổi KHÔNG hợp lệ phải bị từ chối — trạng thái cuối từ chối MỌI
sự kiện. Bước tiếp theo: test một CHUỖI sự kiện, không chỉ từng bước.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đã test TỪNG bước riêng (hợp lệ VÀ bị từ chối). Nếu phát MỘT **chuỗi**
sự kiện liên tiếp (xác nhận → thanh toán → giao hàng → nhận hàng) —
test toàn bộ luồng như thế nào?
::::

::::checkpoint{mastery=0.8}
::::
