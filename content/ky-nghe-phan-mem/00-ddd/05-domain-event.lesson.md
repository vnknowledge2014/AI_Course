---
id: ky-nghe-phan-mem.ddd.domain-event
title: "Domain Event — sự kiện nghiệp vụ, luôn ở thì QUÁ KHỨ"
summary: "Domain Event đặt tên bằng thuật ngữ domain expert quan tâm, LUÔN thì quá khứ. type SuKienDonHang = {tag:\"DaDatDonHang\";...} | {tag:\"DaNhanThanhToan\";...} | ... Event chain: một hàm thuần nhận MỘT event, trả về MẢNG event kế tiếp."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ddd.domain-event]
requires: [ddd.entity-vs-value-object]
concepts: [ddd.domain-event]
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
Entity, Value Object — hai loại "vật" trong domain. Hôm nay: một loại
KHÁC hẳn — không phải vật, mà là CHUYỆN ĐÃ XẢY RA.
::::

::::explain{#domain-event}
**Domain Event**: một sự kiện nghiệp vụ, đặt tên bằng thuật ngữ domain
expert QUAN TÂM, LUÔN ở THÌ QUÁ KHỨ (đã xảy ra, KHÔNG thể đổi được nữa)
— KHÔNG dùng ngôn ngữ kỹ thuật/CRUD (`"DaChenBangDuLieu"` KHÔNG phải
domain event, dù về mặt kỹ thuật đúng là có một dòng vừa được chèn):

```typescript
type SuKienDonHang =
  | { tag: "DaDatDonHang"; maDonHang: string; tongTien: number }
  | { tag: "DaNhanThanhToan"; maDonHang: string }
  | { tag: "DaGiaoHang"; maDonHang: string; maVanDon: string };

type BoXuLySuKien<E> = (event: E) => void;

const inSuKien: BoXuLySuKien<SuKienDonHang> = (sk) => {
  console.log(`Xử lý sự kiện: ${sk.tag}`);
};

inSuKien({ tag: "DaDatDonHang", maDonHang: "DH-01", tongTien: 100000 });
```

```text
Xử lý sự kiện: DaDatDonHang
```

Để ý TÊN mỗi variant: `"DaDatDonHang"` (ĐÃ đặt đơn hàng), `"DaNhanThanhToan"`
(ĐÃ nhận thanh toán), `"DaGiaoHang"` (ĐÃ giao hàng) — CẢ BA đều bắt đầu
bằng "Đã", THÌ QUÁ KHỨ. Domain expert nghĩ về nghiệp vụ theo DÒNG SỰ
KIỆN đã xảy ra — Domain Event mã hoá đúng cách nghĩ đó, không phải cách
database lưu trữ.
::::

::::example{#event-chain}
Một event có thể KÍCH HOẠT các event TIẾP THEO — mã hoá bằng một hàm
THUẦN nhận MỘT event, trả về MẢNG event kế tiếp:

```typescript title=readonly
type SuKienDonHang =
  | { tag: "DaDatDonHang"; maDonHang: string; tongTien: number }
  | { tag: "DaNhanThanhToan"; maDonHang: string }
  | { tag: "DaGiaoHang"; maDonHang: string; maVanDon: string };

function luongDonHang(sk: SuKienDonHang): SuKienDonHang[] {
  switch (sk.tag) {
    case "DaDatDonHang": return [{ tag: "DaNhanThanhToan", maDonHang: sk.maDonHang }];
    case "DaNhanThanhToan": return [{ tag: "DaGiaoHang", maDonHang: sk.maDonHang, maVanDon: "VD-01" }];
    case "DaGiaoHang": return [];
  }
}

console.log(luongDonHang({ tag: "DaDatDonHang", maDonHang: "DH-01", tongTien: 100000 }));
console.log(luongDonHang({ tag: "DaGiaoHang", maDonHang: "DH-01", maVanDon: "VD-01" }));
```

```text title=readonly
[{"tag":"DaNhanThanhToan","maDonHang":"DH-01"}]
[]
```

`DaDatDonHang` KÍCH HOẠT `DaNhanThanhToan` (mảng MỘT phần tử). `DaGiaoHang`
là TERMINAL — không có event nào tiếp theo (mảng RỖNG). Đây LÀ khuôn
"đệ quy trên một chuỗi trạng thái" — nếu gọi `luongDonHang` LẶP LẠI trên
KẾT QUẢ của chính nó, ta có toàn bộ hành trình một đơn hàng đi qua.
::::

::::predict{#doan-luong-tu-thanh-toan commitOnce}
```typescript
type SuKienDonHang =
  | { tag: "DaDatDonHang"; maDonHang: string; tongTien: number }
  | { tag: "DaNhanThanhToan"; maDonHang: string }
  | { tag: "DaGiaoHang"; maDonHang: string; maVanDon: string };

function luongDonHang(sk: SuKienDonHang): SuKienDonHang[] {
  switch (sk.tag) {
    case "DaDatDonHang": return [{ tag: "DaNhanThanhToan", maDonHang: sk.maDonHang }];
    case "DaNhanThanhToan": return [{ tag: "DaGiaoHang", maDonHang: sk.maDonHang, maVanDon: "VD-01" }];
    case "DaGiaoHang": return [];
  }
}

const ketQua = luongDonHang({ tag: "DaNhanThanhToan", maDonHang: "DH-05" });
console.log(ketQua.length);
console.log(ketQua[0]?.tag);
```

Hai dòng cuối in ra gì?

:::opt{correct}
`1` rồi `DaGiaoHang`
:::

:::opt
`3` rồi `DaDatDonHang` — vì `luongDonHang` trả về TOÀN BỘ hành trình
còn lại của đơn hàng, không chỉ bước KẾ TIẾP
::why
Gần đúng ở việc bạn nghĩ tới Ý "toàn bộ hành trình còn lại" — một trực
giác hợp lý cho khái niệm "luồng sự kiện".

Chỗ lệch: `luongDonHang` (đúng như tên gọi VÀ chữ ký `SuKienDonHang =>
SuKienDonHang[]`) chỉ trả về các event KẾ TIẾP TRỰC TIẾP từ event ĐẦU
VÀO — MỘT bước, không tự đệ quy đi xa hơn. `sk.tag === "DaNhanThanhToan"`
khớp nhánh `case "DaNhanThanhToan"`, trả về MẢNG MỘT phần tử
(`DaGiaoHang`) — muốn đi xa hơn phải GỌI LẠI `luongDonHang` trên chính
kết quả đó (không phải việc hàm này tự làm).
::
:::

:::opt
Máy báo lỗi biên dịch — `ketQua[0]?.tag` không hợp lệ vì `ketQua` có thể
rỗng, TypeScript không cho phép truy cập phần tử của mảng có thể rỗng
::why
Gần đúng ở việc bạn để ý `ketQua` CÓ THỂ là mảng rỗng (đúng với
`DaGiaoHang`, dù không phải trường hợp này) — quan sát về khả năng rỗng
đó đúng NÓI CHUNG.

Chỗ lệch: `ketQua[0]?.tag` dùng optional chaining (`?.`) CHÍNH XÁC để
xử lý trường hợp `ketQua[0]` là `undefined` (mảng rỗng) MỘT CÁCH AN
TOÀN — không lỗi biên dịch, không lỗi runtime, chỉ trả `undefined` NẾU
mảng rỗng. Ở đây `ketQua` có MỘT phần tử, `ketQua[0]` tồn tại,
`.tag` đọc bình thường.
::
:::
::::

::::code{#viet_luong_don_hang}
Tự viết `luongDonHang(sk: SuKienDonHang): SuKienDonHang[]`.

```typescript title=starter
type SuKienDonHang =
  | { tag: "DaDatDonHang"; maDonHang: string; tongTien: number }
  | { tag: "DaNhanThanhToan"; maDonHang: string }
  | { tag: "DaGiaoHang"; maDonHang: string; maVanDon: string };

function luongDonHang(sk: SuKienDonHang): SuKienDonHang[] {
  switch (sk.tag) {
    case "DaDatDonHang": return ___;
    case "DaNhanThanhToan": return ___;
    case "DaGiaoHang": return ___;
  }
}

console.log(luongDonHang({ tag: "DaDatDonHang", maDonHang: "DH-01", tongTien: 100000 }));
```

```typescript title=solution
type SuKienDonHang =
  | { tag: "DaDatDonHang"; maDonHang: string; tongTien: number }
  | { tag: "DaNhanThanhToan"; maDonHang: string }
  | { tag: "DaGiaoHang"; maDonHang: string; maVanDon: string };

function luongDonHang(sk: SuKienDonHang): SuKienDonHang[] {
  switch (sk.tag) {
    case "DaDatDonHang": return [{ tag: "DaNhanThanhToan", maDonHang: sk.maDonHang }];
    case "DaNhanThanhToan": return [{ tag: "DaGiaoHang", maDonHang: sk.maDonHang, maVanDon: "VD-01" }];
    case "DaGiaoHang": return [];
  }
}

console.log(luongDonHang({ tag: "DaDatDonHang", maDonHang: "DH-01", tongTien: 100000 }));
```

```typescript title=test
const a = luongDonHang({ tag: "DaDatDonHang", maDonHang: "DH-01", tongTien: 100000 });
if (a.length !== 1) throw new Error("DaDatDonHang phải kích hoạt đúng một event tiếp theo");
if (a[0]?.tag !== "DaNhanThanhToan") throw new Error("DaDatDonHang phải dẫn tới DaNhanThanhToan");

const b = luongDonHang({ tag: "DaNhanThanhToan", maDonHang: "DH-02" });
if (b.length !== 1) throw new Error("DaNhanThanhToan phải kích hoạt đúng một event tiếp theo");
if (b[0]?.tag !== "DaGiaoHang") throw new Error("DaNhanThanhToan phải dẫn tới DaGiaoHang");
if (b[0] && b[0].tag === "DaGiaoHang" && b[0].maDonHang !== "DH-02") throw new Error("event kế tiếp phải giữ đúng mã đơn hàng gốc");
if (b[0] && b[0].tag === "DaGiaoHang" && b[0].maVanDon !== "VD-01") throw new Error("event DaGiaoHang phải mang đúng mã vận đơn \"VD-01\"");

const c = luongDonHang({ tag: "DaGiaoHang", maDonHang: "DH-03", maVanDon: "VD-09" });
if (c.length !== 0) throw new Error("DaGiaoHang là terminal, không kích hoạt event nào tiếp theo");
```

:::hints
- kind: attention
  body: "Mỗi nhánh trả về MỘT MẢNG (kể cả khi chỉ có một event, phải bọc trong []). Nhánh DaGiaoHang (terminal) trả mảng RỖNG."
- kind: strategy
  body: '[{ tag: "DaNhanThanhToan", maDonHang: sk.maDonHang }] — cho DaDatDonHang. [{ tag: "DaGiaoHang", maDonHang: sk.maDonHang, maVanDon: "VD-01" }] — cho DaNhanThanhToan. [] — cho DaGiaoHang.'
- kind: one-line
  body: 'return [{ tag: "DaNhanThanhToan", maDonHang: sk.maDonHang }];\nreturn [{ tag: "DaGiaoHang", maDonHang: sk.maDonHang, maVanDon: "VD-01" }];\nreturn [];'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "DaNhanThanhToan"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Domain Event: chuyện ĐÃ XẢY RA, luôn thì quá khứ. Event chain: một hàm
thuần nối các sự kiện thành một dòng chảy nghiệp vụ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Làm sao BIẾT hết những sự kiện nào tồn tại trong một domain — TRƯỚC
khi viết một dòng code nào?
::::

::::checkpoint{mastery=0.8}
::::
