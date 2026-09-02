---
id: ky-nghe-phan-mem.ddd.event-storming
title: "Event Storming — khám phá domain cùng cả đội"
summary: "Event Storming: kỹ thuật workshop (Alberto Brandolini) — cả team khám phá domain bằng sticky-note. Năm thành phần & ánh xạ sang TypeScript: Events→DU (bài 5), Commands→DU khác (đứng TRƯỚC event), Aggregates→bounded context, Policies→event-handler function."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.event-storming]
requires: [ddd.domain-event]
concepts: [ddd.event-storming]
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
Bài chốt cụm 1. Câu hỏi cuối bài 5: làm sao BIẾT hết sự kiện nào tồn
tại trong một domain — TRƯỚC khi viết code?
::::

::::explain{#event-storming}
**Event Storming**: kỹ thuật workshop (Alberto Brandolini sáng tạo) —
thay vì dev TỰ ĐOÁN domain, CẢ ĐỘI (dev + Product Owner + domain
expert) cùng khám phá bằng sticky-note dán trên tường, theo dòng thời
gian TRÁI→PHẢI. Không cần code, không cần công cụ kỹ thuật — chỉ cần
giấy note và một bức tường trống.

**Năm thành phần**, mỗi màu một vai trò, VÀ cách ánh xạ sang những gì
bạn đã học:

- 🟧 **Events** (quá khứ, "đã xảy ra") → DU variant (bài 5's
  `SuKienDonHang`).
- 🟦 **Commands** (mệnh lệnh, đứng TRƯỚC event, CÓ THỂ bị từ chối) → DU
  variant KHÁC — MỚI, chưa gặp:

```typescript
// Command: MỆNH LỆNH, đứng TRƯỚC event, có thể bị TỪ CHỐI
type LenhDonHang =
  | { tag: "DatDonHang"; maKhachHang: string; tongTien: number }
  | { tag: "HuyDonHang"; maDonHang: string };

// Event: đã xảy ra, KHÔNG thể từ chối
type SuKienDonHang =
  | { tag: "DaDatDonHang"; maDonHang: string; tongTien: number }
  | { tag: "DaHuyDonHang"; maDonHang: string };
```

Để ý sự KHÁC NHAU: `LenhDonHang` dùng ĐỘNG TỪ MỆNH LỆNH (`"DatDonHang"`
— "Đặt đơn hàng!", một YÊU CẦU CÓ THỂ bị từ chối, ví dụ hết hàng);
`SuKienDonHang` dùng THÌ QUÁ KHỨ (`"DaDatDonHang"` — "Đã đặt đơn hàng",
CHUYỆN đã xảy ra rồi, không thể huỷ ngược). MỘT `LenhDonHang` THÀNH
CÔNG mới sinh ra MỘT `SuKienDonHang` tương ứng.

- 🟨 **Aggregates** (nhóm command+event LIÊN QUAN với nhau) → ứng viên
  cho module/bounded context (bài 3).
- 🟪 **Policies** ("NẾU sự kiện X xảy ra THÌ tự động phát lệnh Y") →
  event-handler function (bài 5's `BoXuLySuKien`, nhưng TRẢ VỀ command
  thay vì chỉ `void`).
- 🟩 **Read Models** (dashboard/báo cáo người dùng NHÌN thấy) — để dành,
  KHÔNG đào sâu trong track này.
::::

::::example{#policy-tu-dong-huy}
Một Policy CỤ THỂ: "nếu đơn hàng VỪA đặt có giá trị QUÁ LỚN, tự động
phát lệnh huỷ để kiểm tra gian lận":

```typescript title=readonly
type LenhDonHang =
  | { tag: "DatDonHang"; maKhachHang: string; tongTien: number }
  | { tag: "HuyDonHang"; maDonHang: string };
type SuKienDonHang =
  | { tag: "DaDatDonHang"; maDonHang: string; tongTien: number }
  | { tag: "DaHuyDonHang"; maDonHang: string };

function chinhSachSauKhiDat(sk: SuKienDonHang): LenhDonHang[] {
  switch (sk.tag) {
    case "DaDatDonHang":
      return sk.tongTien > 5000000 ? [{ tag: "HuyDonHang", maDonHang: sk.maDonHang }] : [];
    case "DaHuyDonHang":
      return [];
  }
}

console.log(chinhSachSauKhiDat({ tag: "DaDatDonHang", maDonHang: "DH-01", tongTien: 6000000 }));
console.log(chinhSachSauKhiDat({ tag: "DaDatDonHang", maDonHang: "DH-02", tongTien: 100000 }));
```

```text title=readonly
[{"tag":"HuyDonHang","maDonHang":"DH-01"}]
[]
```

`chinhSachSauKhiDat` LÀ một Policy — NHẬN một Event (đã xảy ra), TRẢ về
Command MỚI (yêu cầu HÀNH ĐỘNG tiếp theo). Đơn `DH-01` (6 triệu, quá
ngưỡng 5 triệu) tự động sinh lệnh `HuyDonHang` — CHÍNH LÀ khuôn
`BoXuLySuKien` (bài 5), chỉ khác: TRẢ VỀ command thay vì chỉ in log.
::::

::::predict{#doan-policy-duoi-nguong commitOnce}
```typescript
type LenhDonHang =
  | { tag: "DatDonHang"; maKhachHang: string; tongTien: number }
  | { tag: "HuyDonHang"; maDonHang: string };
type SuKienDonHang =
  | { tag: "DaDatDonHang"; maDonHang: string; tongTien: number }
  | { tag: "DaHuyDonHang"; maDonHang: string };
function chinhSachSauKhiDat(sk: SuKienDonHang): LenhDonHang[] {
  switch (sk.tag) {
    case "DaDatDonHang":
      return sk.tongTien > 5000000 ? [{ tag: "HuyDonHang", maDonHang: sk.maDonHang }] : [];
    case "DaHuyDonHang":
      return [];
  }
}

const ketQua = chinhSachSauKhiDat({ tag: "DaDatDonHang", maDonHang: "DH-09", tongTien: 5000000 });
console.log(ketQua.length);
```

`tongTien` ĐÚNG BẰNG `5000000` (ngưỡng, không vượt qua). Dòng cuối in
ra gì?

:::opt{correct}
`0`
:::

:::opt
`1` — vì `5000000` đạt NGƯỠNG kiểm tra gian lận, nên vẫn kích hoạt lệnh
huỷ như trường hợp vượt ngưỡng
::why
Gần đúng ở việc bạn để ý `5000000` LÀ con số NGƯỠNG được nhắc tới trong
policy — quan sát về việc con số này CÓ Ý NGHĨA đặc biệt đúng.

Chỗ lệch: điều kiện là `sk.tongTien > 5000000` — dùng `>` (LỚN HƠN
NGHIÊM NGẶT), KHÔNG phải `>=` (lớn hơn hoặc BẰNG). `5000000 > 5000000`
là `false` — đơn hàng ĐÚNG BẰNG ngưỡng KHÔNG kích hoạt policy, trả về
mảng RỖNG (`[]`, độ dài `0`).
::
:::

:::opt
Máy báo lỗi biên dịch — nhánh `case "DaDatDonHang"` dùng toán tử ba
ngôi (`? :`) thay vì `if`/`else`, không hợp lệ bên trong `switch`
::why
Gần đúng ở việc bạn để ý nhánh NÀY viết theo cấu trúc HƠI khác các
nhánh `switch` đơn giản khác (dùng `? :` thay vì if/else) — quan sát về
CÓ sự khác biệt cú pháp đó đúng.

Chỗ lệch: toán tử ba ngôi (`dieuKien ? A : B`) là một BIỂU THỨC hợp lệ
ở BẤT KỲ đâu một biểu thức được phép xuất hiện — kể cả ngay sau `return`
bên trong một nhánh `case`. Hoàn toàn hợp lệ, biên dịch và chạy bình
thường, không có ràng buộc nào cấm dùng `? :` trong `switch`.
::
:::
::::

::::code{#viet_chinh_sach}
Tự viết `chinhSachSauKhiDat(sk: SuKienDonHang): LenhDonHang[]`.

```typescript title=starter
type LenhDonHang =
  | { tag: "DatDonHang"; maKhachHang: string; tongTien: number }
  | { tag: "HuyDonHang"; maDonHang: string };
type SuKienDonHang =
  | { tag: "DaDatDonHang"; maDonHang: string; tongTien: number }
  | { tag: "DaHuyDonHang"; maDonHang: string };

function chinhSachSauKhiDat(sk: SuKienDonHang): LenhDonHang[] {
  switch (sk.tag) {
    case "DaDatDonHang":
      return sk.tongTien > 5000000 ? ___ : ___;
    case "DaHuyDonHang":
      return ___;
  }
}

console.log(chinhSachSauKhiDat({ tag: "DaDatDonHang", maDonHang: "DH-01", tongTien: 6000000 }));
```

```typescript title=solution
type LenhDonHang =
  | { tag: "DatDonHang"; maKhachHang: string; tongTien: number }
  | { tag: "HuyDonHang"; maDonHang: string };
type SuKienDonHang =
  | { tag: "DaDatDonHang"; maDonHang: string; tongTien: number }
  | { tag: "DaHuyDonHang"; maDonHang: string };

function chinhSachSauKhiDat(sk: SuKienDonHang): LenhDonHang[] {
  switch (sk.tag) {
    case "DaDatDonHang":
      return sk.tongTien > 5000000 ? [{ tag: "HuyDonHang", maDonHang: sk.maDonHang }] : [];
    case "DaHuyDonHang":
      return [];
  }
}

console.log(chinhSachSauKhiDat({ tag: "DaDatDonHang", maDonHang: "DH-01", tongTien: 6000000 }));
```

```typescript title=test
const a = chinhSachSauKhiDat({ tag: "DaDatDonHang", maDonHang: "DH-01", tongTien: 6000000 });
if (a.length !== 1) throw new Error("đơn hàng VƯỢT ngưỡng phải kích hoạt đúng một lệnh huỷ");
if (a[0]?.tag !== "HuyDonHang") throw new Error("lệnh kích hoạt phải là HuyDonHang");
if (a[0] && a[0].tag === "HuyDonHang" && a[0].maDonHang !== "DH-01") throw new Error("lệnh huỷ phải giữ đúng mã đơn hàng");

const b = chinhSachSauKhiDat({ tag: "DaDatDonHang", maDonHang: "DH-02", tongTien: 100000 });
if (b.length !== 0) throw new Error("đơn hàng DƯỚI ngưỡng không được kích hoạt lệnh nào");

const c = chinhSachSauKhiDat({ tag: "DaHuyDonHang", maDonHang: "DH-03" });
if (c.length !== 0) throw new Error("DaHuyDonHang không kích hoạt lệnh nào tiếp theo");

const d = chinhSachSauKhiDat({ tag: "DaDatDonHang", maDonHang: "DH-04", tongTien: 5000000 });
if (d.length !== 0) throw new Error("đơn hàng ĐÚNG BẰNG ngưỡng 5000000 KHÔNG được kích hoạt lệnh nào (ngưỡng dùng >, không phải >=)");

const e = chinhSachSauKhiDat({ tag: "DaDatDonHang", maDonHang: "DH-05", tongTien: 5000001 });
if (e.length !== 1) throw new Error("đơn hàng VƯỢT ngưỡng dù chỉ 1đ (5000001) VẪN phải kích hoạt lệnh huỷ");
```

:::hints
- kind: attention
  body: "Nhánh DaDatDonHang: NẾU tongTien VƯỢT ngưỡng, trả mảng có MỘT lệnh HuyDonHang (giữ đúng mã đơn hàng); NGƯỢC LẠI trả mảng RỖNG. Nhánh DaHuyDonHang: luôn trả mảng RỖNG (không kích hoạt gì thêm)."
- kind: strategy
  body: '[{ tag: "HuyDonHang", maDonHang: sk.maDonHang }] : [] — cho nhánh DaDatDonHang. [] — cho nhánh DaHuyDonHang.'
- kind: one-line
  body: 'return sk.tongTien > 5000000 ? [{ tag: "HuyDonHang", maDonHang: sk.maDonHang }] : [];\nreturn [];'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "HuyDonHang"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Event Storming: khám phá domain CÙNG cả đội, không cần code. Năm màu
sticky-note ánh xạ THẲNG sang những gì bạn đã học — Events, Commands,
Aggregates, Policies đều CÓ hình dạng TypeScript cụ thể.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cụm 1 xong — bạn có TỪ VỰNG domain và các "vật" cơ bản (Entity, Value
Object, Event). Nhưng những "vật" đó SỐNG trong một ỨNG DỤNG THẬT —
làm sao TỔ CHỨC code để domain KHÔNG bị trộn lẫn với chi tiết kỹ thuật
(database, API)?
::::

::::checkpoint{mastery=0.8}
::::
