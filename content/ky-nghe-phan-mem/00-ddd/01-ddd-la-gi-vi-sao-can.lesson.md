---
id: ky-nghe-phan-mem.ddd.ddd-la-gi-vi-sao-can
title: "DDD là gì — code nói ngôn ngữ nghiệp vụ, không phải ngôn ngữ máy"
summary: "DDD (Domain-Driven Design): viết code sao cho nó PHẢN ÁNH đúng cách domain expert nói về nghiệp vụ. `data.s`/`data.p` (vô nghĩa) → `TrangThaiDonHang`/`apDungGiamGiaXacNhan` (đọc hiểu ngay). Ba trụ cột: Ubiquitous Language, Bounded Context, Domain Model."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [ddd.what-and-why]
requires: [ts.adt-gate-boss]
concepts: [ddd.what-and-why]
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
Track mới — Software Engineering. Bắt đầu bằng một câu hỏi: code của bạn
NÓI ngôn ngữ gì?
::::

::::explain{#ddd-la-gi}
`data.s = 1; data.p = data.p * 0.9;` — KHÔNG ai đọc dòng này mà hiểu ngay
nó làm gì. `s` là gì? `p` là gì? Tại sao nhân `0.9`?

```typescript
type DonHangCu = { s: number; p: number };
function xuLyCu(dh: DonHangCu): DonHangCu {
  if (dh.s === 1) {
    return { s: dh.s, p: dh.p * 0.9 };
  }
  return dh;
}
```

So với:

```typescript
type TrangThaiDonHang = "nhap" | "da_xac_nhan" | "da_giao";
type DonHang = { trangThai: TrangThaiDonHang; tongTien: number };

function apDungGiamGiaXacNhan(donHang: DonHang): DonHang {
  if (donHang.trangThai === "da_xac_nhan") {
    return { ...donHang, tongTien: donHang.tongTien * 0.9 };
  }
  return donHang;
}
```

HAI phiên bản làm ĐÚNG MỘT VIỆC (giảm 10% khi đơn hàng đã xác nhận) — chỉ
khác TÊN. Nhưng phiên bản sau đọc lên là HIỂU ngay, không cần tra cứu
`s`/`p` nghĩa là gì. Đây là ý CỐT LÕI của **DDD (Domain-Driven Design)**:
viết code sao cho nó PHẢN ÁNH đúng cách domain expert (người hiểu nghiệp
vụ, không nhất thiết biết lập trình) NÓI về nghiệp vụ — không phải ngôn
ngữ máy tính.

DDD có BA trụ cột (mỗi trụ cột một bài riêng trong track này, ở đây chỉ
giới thiệu TÊN):

- **Ubiquitous Language** — dev và business dùng CHUNG một từ vựng, thể
  hiện ngay trong tên type/field.
- **Bounded Context** — ranh giới rõ ràng, nơi một từ có MỘT nghĩa nhất
  quán (một hệ thống lớn có NHIỀU ranh giới như vậy).
- **Domain Model** — code = business logic, ánh xạ trực tiếp sang Types
  + hàm thuần trong TypeScript (đúng phong cách track này theo đuổi
  xuyên suốt Realm 4: pure function, immutable data).
::::

::::example{#discriminated-union-la-tu-vung}
Ubiquitous Language không chỉ là ĐẶT TÊN — nó còn thể hiện qua CÁCH mã
hoá một khái niệm nghiệp vụ. `GiamGia` (khuyến mãi) trong đời thực có
NHIỀU hình thức khác nhau — mã hoá bằng discriminated union (đã học ở
T4.3) thay vì một con số mập mờ:

```typescript
type GiamGia =
  | { tag: "phan_tram"; phanTram: number }
  | { tag: "tien_co_dinh"; soTien: number };

function apDungGiamGia(gg: GiamGia, gia: number): number {
  switch (gg.tag) {
    case "phan_tram": return gia * (1 - gg.phanTram / 100);
    case "tien_co_dinh": return Math.max(0, gia - gg.soTien);
  }
}

console.log(apDungGiamGia({ tag: "phan_tram", phanTram: 10 }, 100000));
console.log(apDungGiamGia({ tag: "tien_co_dinh", soTien: 20000 }, 100000));
```

```text
90000
80000
```

`GiamGia` KHÔNG phải một con số hay một cờ boolean mập mờ — nó là MỘT
type nói ĐÚNG những gì domain expert nghĩ khi nói "khuyến mãi": HOẶC
giảm theo phần trăm, HOẶC giảm một số tiền cố định. Đây LÀ Ubiquitous
Language THỂ HIỆN qua kiểu dữ liệu, không chỉ qua tên biến.
::::

::::predict{#doan-donhang-chua-xac-nhan commitOnce}
```typescript
type TrangThaiDonHang = "nhap" | "da_xac_nhan" | "da_giao";
type DonHang = { trangThai: TrangThaiDonHang; tongTien: number };

function apDungGiamGiaXacNhan(donHang: DonHang): DonHang {
  if (donHang.trangThai === "da_xac_nhan") {
    return { ...donHang, tongTien: donHang.tongTien * 0.9 };
  }
  return donHang;
}

const donHangMoi: DonHang = { trangThai: "nhap", tongTien: 200000 };
const ketQua = apDungGiamGiaXacNhan(donHangMoi);
console.log(ketQua.tongTien);
```

`donHangMoi` có `trangThai: "nhap"` (CHƯA xác nhận). Dòng cuối in ra gì?

:::opt{correct}
`200000`
:::

:::opt
`180000` — vì mọi đơn hàng đi qua `apDungGiamGiaXacNhan` đều được giảm
10%, bất kể trạng thái
::why
Gần đúng ở việc bạn nhớ ĐÚNG công thức giảm giá (`* 0.9` = giảm 10%) —
phép tính đó đúng NẾU điều kiện thoả.

Chỗ lệch: `apDungGiamGiaXacNhan` CHỈ giảm giá khi `donHang.trangThai ===
"da_xac_nhan"` — đây LÀ điểm CHÍNH của tên hàm (`...XacNhan`, không phải
`...MoiDonHang`). `donHangMoi.trangThai` là `"nhap"`, KHÔNG khớp điều
kiện, hàm trả về `donHang` NGUYÊN VẸN (nhánh `return donHang;`) — `
tongTien` giữ nguyên `200000`.
::
:::

:::opt
Máy báo lỗi biên dịch — `trangThai: "nhap"` không đủ thông tin để
TypeScript biết có nên giảm giá hay không
::why
Gần đúng ở việc bạn để ý `TrangThaiDonHang` có NHIỀU giá trị khả dĩ
(`"nhap"`/`"da_xac_nhan"`/`"da_giao"`) — quan sát về việc CÓ nhiều biến
thể đó đúng.

Chỗ lệch: `"nhap"` là MỘT giá trị HOÀN TOÀN HỢP LỆ của union
`TrangThaiDonHang` — không có gì "thiếu thông tin" ở đây. Hàm
`apDungGiamGiaXacNhan` xử lý CẢ BA trường hợp bằng `if`/`return` — khi
điều kiện KHÔNG khớp, nó có một nhánh RÕ RÀNG (`return donHang`) để xử
lý, không cần TypeScript can thiệp gì thêm. Biên dịch và chạy hoàn toàn
bình thường.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
DDD bắt đầu từ một câu hỏi đơn giản: code của bạn có NÓI được ngôn ngữ
mà domain expert hiểu không? Ba trụ cột — Ubiquitous Language, Bounded
Context, Domain Model — là cách trả lời câu đó một cách có hệ thống.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

"Tên đúng" là bước đầu. Nhưng làm sao BIẾT một tên có ĐÚNG hay không —
ai là người quyết định?
::::

::::checkpoint{mastery=0.8}
::::
