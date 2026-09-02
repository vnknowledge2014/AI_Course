---
id: ky-nghe-phan-mem.mau-tai-cau-truc.capstone-refactor-pipeline-request
title: "Capstone: Refactor pipeline xử lý request — lồng nhau → compose"
summary: "Bài chốt cụm 5: xuLyYeuCau lồng nhau sâu (if validate → if auth → if token, \"pyramid of doom\") tái cấu trúc thành taoPipeline(buocValidate, buocXacThuc, buocXuLy) — mỗi bước TÁCH RIÊNG, tự kiểm \"đã có lỗi trước đó chưa\" để bỏ qua. Kết quả CUỐI CÙNG giống HỆT bản gốc lồng nhau, chỉ đọc DỄ HƠN nhiều."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 20
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.gate-boss-middleware-refactor]
requires: [mau.refactor-conditional-to-pattern-match]
concepts: [mau.gate-boss-middleware-refactor]
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
Bài chốt cụm 5. Một hàm xử lý request LỒNG NHAU BA tầng if — validate
→ auth → xử lý. Ghép taoPipeline (bài 17) + Extract Function (bài
18) để "tháo" nó ra.
::::

::::explain{#pyramid-of-doom-den-pipeline}
Bản LỒNG NHAU ("pyramid of doom" — CÀNG lồng SÂU CÀNG khó đọc):

```typescript
// ❌ LỒNG NHAU BA TẦNG
// function xuLyYeuCauV1(yc: YeuCau): KetQua {
//   if (yc.duLieu.trim().length > 0) {
//     if (yc.token !== null) {
//       if (yc.token === "hop-le") {
//         return { thanhCong: true, thongDiep: `xu ly: ${yc.duLieu.trim()}` };
//       } else { return { thanhCong: false, thongDiep: "token khong hop le" }; }
//     } else { return { thanhCong: false, thongDiep: "thieu token" }; }
//   } else { return { thanhCong: false, thongDiep: "du lieu rong" }; }
// }
```

Bản PIPELINE: MỖI tầng if TRỞ THÀNH MỘT hàm RIÊNG (Extract Function),
GHÉP qua `taoPipeline` (bài 17) — MỖI bước TỰ kiểm "ĐÃ có lỗi TRƯỚC
ĐÓ chưa" ĐỂ bỏ qua (KHÔNG cần lồng if):

```typescript title=readonly
function taoPipeline<T>(...cacBuoc: Array<(t: T) => T>): (t: T) => T {
  return (t: T) => cacBuoc.reduce((giaTri, buoc) => buoc(giaTri), t);
}

type YeuCau = { token: string | null; duLieu: string };
type KetQua = { thanhCong: boolean; thongDiep: string };
type TrangThaiXuLy = { yc: YeuCau; ketQua: KetQua | null };

function buocValidate(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (tt.ketQua !== null) return tt;
  if (tt.yc.duLieu.trim().length === 0) {
    return { ...tt, ketQua: { thanhCong: false, thongDiep: "du lieu rong" } };
  }
  return tt;
}

function buocXacThuc(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (tt.ketQua !== null) return tt;
  if (tt.yc.token === null) {
    return { ...tt, ketQua: { thanhCong: false, thongDiep: "thieu token" } };
  }
  if (tt.yc.token !== "hop-le") {
    return { ...tt, ketQua: { thanhCong: false, thongDiep: "token khong hop le" } };
  }
  return tt;
}

function buocXuLy(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (tt.ketQua !== null) return tt;
  return { ...tt, ketQua: { thanhCong: true, thongDiep: `xu ly: ${tt.yc.duLieu.trim()}` } };
}

function xuLyYeuCau(yc: YeuCau): KetQua {
  const pipeline = taoPipeline(buocValidate, buocXacThuc, buocXuLy);
  return pipeline({ yc, ketQua: null }).ketQua!;
}

console.log(xuLyYeuCau({ token: "hop-le", duLieu: "  xin chao  " }));
console.log(xuLyYeuCau({ token: null, duLieu: "xin chao" }));
```

```text title=readonly
{"thanhCong":true,"thongDiep":"xu ly: xin chao"}
{"thanhCong":false,"thongDiep":"thieu token"}
```

KẾT QUẢ CUỐI CÙNG giống HỆT bản lồng nhau — NHƯNG giờ MỖI bước ĐỌC
được RIÊNG, TEST được RIÊNG, VÀ thêm bước MỚI CHỈ LÀ thêm MỘT tham
số vào `taoPipeline(...)`, KHÔNG cần LỒNG THÊM một tầng `if` nào.
::::

::::example{#loi-dau-tien-duoc-giu}
Guard `if (tt.ketQua !== null) return tt;` Ở ĐẦU MỖI bước LÀ chìa
khoá: MỘT khi lỗi ĐÃ được ghi nhận, các bước SAU **BỎ QUA HOÀN TOÀN**
— lỗi **ĐẦU TIÊN** tìm thấy LUÔN được GIỮ, KHÔNG BAO GIỜ bị GHI ĐÈ:

```typescript title=readonly
function taoPipeline<T>(...cacBuoc: Array<(t: T) => T>): (t: T) => T {
  return (t: T) => cacBuoc.reduce((giaTri, buoc) => buoc(giaTri), t);
}
type YeuCau = { token: string | null; duLieu: string };
type KetQua = { thanhCong: boolean; thongDiep: string };
type TrangThaiXuLy = { yc: YeuCau; ketQua: KetQua | null };
function buocValidate(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (tt.ketQua !== null) return tt;
  if (tt.yc.duLieu.trim().length === 0) {
    return { ...tt, ketQua: { thanhCong: false, thongDiep: "du lieu rong" } };
  }
  return tt;
}
function buocXacThuc(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (tt.ketQua !== null) return tt;
  if (tt.yc.token === null) {
    return { ...tt, ketQua: { thanhCong: false, thongDiep: "thieu token" } };
  }
  return tt;
}
function buocXuLy(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (tt.ketQua !== null) return tt;
  return { ...tt, ketQua: { thanhCong: true, thongDiep: `xu ly: ${tt.yc.duLieu.trim()}` } };
}
function xuLyYeuCau(yc: YeuCau): KetQua {
  const pipeline = taoPipeline(buocValidate, buocXacThuc, buocXuLy);
  return pipeline({ yc, ketQua: null }).ketQua!;
}

// CẢ HAI đều SAI: duLieu RỖNG *VÀ* token thiếu
console.log(xuLyYeuCau({ token: null, duLieu: "" }));
```

```text title=readonly
{"thanhCong":false,"thongDiep":"du lieu rong"}
```

Dù `token` CŨNG `null` (LẼ RA `buocXacThuc` sẽ báo "thieu token"),
`buocValidate` CHẠY TRƯỚC, phát hiện `duLieu` RỖNG, GHI `ketQua`
NGAY — `buocXacThuc` SAU ĐÓ THẤY `tt.ketQua !== null`, TRẢ VỀ
NGUYÊN VẸN (KHÔNG chạm gì cả). Thông điệp GIỮ NGUYÊN `"du lieu rong"`.
::::

::::predict{#doan-loi-dau-tien-thang commitOnce}
```typescript
function taoPipeline<T>(...cacBuoc: Array<(t: T) => T>): (t: T) => T {
  return (t: T) => cacBuoc.reduce((giaTri, buoc) => buoc(giaTri), t);
}
type YeuCau = { token: string | null; duLieu: string };
type KetQua = { thanhCong: boolean; thongDiep: string };
type TrangThaiXuLy = { yc: YeuCau; ketQua: KetQua | null };
function buocValidate(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (tt.ketQua !== null) return tt;
  if (tt.yc.duLieu.trim().length === 0) {
    return { ...tt, ketQua: { thanhCong: false, thongDiep: "du lieu rong" } };
  }
  return tt;
}
function buocXacThuc(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (tt.ketQua !== null) return tt;
  if (tt.yc.token !== "hop-le") {
    return { ...tt, ketQua: { thanhCong: false, thongDiep: "token khong hop le" } };
  }
  return tt;
}
function buocXuLy(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (tt.ketQua !== null) return tt;
  return { ...tt, ketQua: { thanhCong: true, thongDiep: `xu ly: ${tt.yc.duLieu.trim()}` } };
}
function xuLyYeuCau(yc: YeuCau): KetQua {
  const pipeline = taoPipeline(buocValidate, buocXacThuc, buocXuLy);
  return pipeline({ yc, ketQua: null }).ketQua!;
}

console.log(xuLyYeuCau({ token: "sai-token", duLieu: "co du lieu" }).thongDiep);
```

Dòng cuối in ra gì? (`duLieu` HỢP LỆ, `token` KHÔNG khớp `"hop-le"`.)

:::opt{correct}
`token khong hop le`
:::

:::opt
`xu ly: co du lieu` — vì `buocXacThuc` CHỈ kiểm token SAI cú pháp
(khác hẳn `null`), VÀ `"sai-token"` LÀ một CHUỖI hợp lệ (KHÔNG PHẢI
`null`) nên bước NÀY coi như XÁC THỰC thành công, để `buocXuLy` chạy
BÌNH THƯỜNG
::why
Gần đúng ở việc bạn để ý `"sai-token"` LÀ một CHUỖI (KHÔNG PHẢI
`null`) — quan sát ĐÓ về KIỂU dữ liệu đúng.

Chỗ lệch: `buocXacThuc` kiểm tra `tt.yc.token !== "hop-le"` — SO
SÁNH VỚI GIÁ TRỊ CHUỖI CỤ THỂ (`"hop-le"`), KHÔNG PHẢI CHỈ kiểm
`null`/không-null. `"sai-token"` **KHÁC** `"hop-le"` — điều kiện
`!==` LÀ `true`, hàm TRẢ VỀ lỗi `"token khong hop le"` NGAY, `ketQua`
được GHI, `buocXuLy` SAU ĐÓ thấy `ketQua !== null` VÀ bỏ qua.
::
:::

:::opt
Máy báo lỗi biên dịch — `TrangThaiXuLy` khai `ketQua: KetQua | null`,
NHƯNG `buocXacThuc` gán `ketQua` MỘT object literal (KHÔNG PHẢI
`null`), TypeScript CẤM gán giá trị KHÁC `null` cho một thuộc tính
khai kiểu UNION VỚI `null`
::why
Gần đúng ở việc bạn để ý `ketQua` khai kiểu UNION VỚI `null` (`KetQua
| null`) — một quan sát ĐÚNG về KHAI BÁO kiểu.

Chỗ lệch: UNION `KetQua | null` nghĩa LÀ "giá trị NÀY CÓ THỂ LÀ MỘT
trong HAI kiểu" — gán MỘT giá trị khớp `KetQua` (object literal
`{thanhCong, thongDiep}`) HOÀN TOÀN hợp lệ (ĐÓ CHÍNH LÀ MỘT trong hai
khả năng CỦA union). CHỈ gán một giá trị KHÔNG khớp CẢ HAI (ví dụ một
`number`) mới bị chặn. Biên dịch SẠCH.
::
:::
::::

::::code{#viet_capstone_pipeline_refactor}
Hoàn thiện `buocXacThuc` VÀ `buocXuLy` — MỖI bước tự kiểm "đã có lỗi
chưa" TRƯỚC khi xử lý.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function taoPipeline<T>(...cacBuoc: Array<(t: T) => T>): (t: T) => T {
  return (t: T) => cacBuoc.reduce((giaTri, buoc) => buoc(giaTri), t);
}

type YeuCau = { token: string | null; duLieu: string };
type KetQua = { thanhCong: boolean; thongDiep: string };
type TrangThaiXuLy = { yc: YeuCau; ketQua: KetQua | null };

function buocValidate(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (tt.ketQua !== null) return tt;
  if (tt.yc.duLieu.trim().length === 0) {
    return { ...tt, ketQua: { thanhCong: false, thongDiep: "du lieu rong" } };
  }
  return tt;
}

function buocXacThuc(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (___) return tt;
  if (tt.yc.token === null) {
    return { ...tt, ketQua: { thanhCong: false, thongDiep: "thieu token" } };
  }
  if (tt.yc.token !== "hop-le") {
    return { ...tt, ketQua: { thanhCong: false, thongDiep: "token khong hop le" } };
  }
  return tt;
}

function buocXuLy(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (___) return tt;
  return { ...tt, ketQua: { thanhCong: true, thongDiep: `xu ly: ${tt.yc.duLieu.trim()}` } };
}

function xuLyYeuCau(yc: YeuCau): KetQua {
  const pipeline = taoPipeline(buocValidate, buocXacThuc, buocXuLy);
  return pipeline({ yc, ketQua: null }).ketQua!;
}

assertEqual(xuLyYeuCau({ token: "hop-le", duLieu: "  xin chao  " }).thanhCong, true, "yeu cau hop le thanh cong");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function taoPipeline<T>(...cacBuoc: Array<(t: T) => T>): (t: T) => T {
  return (t: T) => cacBuoc.reduce((giaTri, buoc) => buoc(giaTri), t);
}

type YeuCau = { token: string | null; duLieu: string };
type KetQua = { thanhCong: boolean; thongDiep: string };
type TrangThaiXuLy = { yc: YeuCau; ketQua: KetQua | null };

function buocValidate(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (tt.ketQua !== null) return tt;
  if (tt.yc.duLieu.trim().length === 0) {
    return { ...tt, ketQua: { thanhCong: false, thongDiep: "du lieu rong" } };
  }
  return tt;
}

function buocXacThuc(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (tt.ketQua !== null) return tt;
  if (tt.yc.token === null) {
    return { ...tt, ketQua: { thanhCong: false, thongDiep: "thieu token" } };
  }
  if (tt.yc.token !== "hop-le") {
    return { ...tt, ketQua: { thanhCong: false, thongDiep: "token khong hop le" } };
  }
  return tt;
}

function buocXuLy(tt: TrangThaiXuLy): TrangThaiXuLy {
  if (tt.ketQua !== null) return tt;
  return { ...tt, ketQua: { thanhCong: true, thongDiep: `xu ly: ${tt.yc.duLieu.trim()}` } };
}

function xuLyYeuCau(yc: YeuCau): KetQua {
  const pipeline = taoPipeline(buocValidate, buocXacThuc, buocXuLy);
  return pipeline({ yc, ketQua: null }).ketQua!;
}

assertEqual(xuLyYeuCau({ token: "hop-le", duLieu: "  xin chao  " }).thanhCong, true, "yeu cau hop le thanh cong");
```

```typescript title=test
assertEqual(xuLyYeuCau({ token: "hop-le", duLieu: "  xin chao  " }).thongDiep, "xu ly: xin chao", "thong diep dung");
assertEqual(xuLyYeuCau({ token: null, duLieu: "xin chao" }).thongDiep, "thieu token", "thieu token bi bat");
assertEqual(xuLyYeuCau({ token: "sai", duLieu: "xin chao" }).thongDiep, "token khong hop le", "token sai bi bat");
assertEqual(xuLyYeuCau({ token: null, duLieu: "" }).thongDiep, "du lieu rong", "loi dau tien duoc giu, khong bi ghi de boi loi token");
```

:::hints
- kind: attention
  body: "Cả hai blank giống hệt guard đã dùng ở buocValidate: nếu tt.ketQua đã KHÁC null (đã có lỗi từ bước trước), trả về tt nguyên vẹn, không xử lý thêm."
- kind: strategy
  body: "tt.ketQua !== null — điều kiện guard giống nhau ở CẢ BA bước, kiểm tra xem đã có kết quả (lỗi hoặc thành công) từ bước trước chưa."
- kind: one-line
  body: '___ (cả hai chỗ) = tt.ketQua !== null'
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
Cụm 5 hoàn tất: Middleware=pipeline, Extract Function, Replace
Conditional, capstone tháo pyramid-of-doom. Cụm cuối: CQRS & Event
Sourcing.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`TrangThaiXuLy` gộp CẢ dữ liệu ĐỌC (`yc`) VÀ kết quả GHI (`ketQua`)
vào MỘT type. Nếu TÁCH RIÊNG "đọc" (query) VÀ "ghi" (command) thành
HAI model HOÀN TOÀN khác nhau — đó CHÍNH LÀ ý tưởng của track sắp
tới. Bạn đoán được TÊN của nó chưa?
::::

::::checkpoint{mastery=0.8}
::::
