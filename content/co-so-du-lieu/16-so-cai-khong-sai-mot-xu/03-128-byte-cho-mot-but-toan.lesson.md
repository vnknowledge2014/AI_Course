---
id: co-so-du-lieu.so-cai-khong-sai-mot-xu.128-byte-cho-mot-but-toan
title: "128 byte cho một bút toán"
summary: "TigerBeetle THẬT không lưu Transfer dưới dạng object linh hoạt như ButToan (bài 1-2) — mỗi bút toán chiếm ĐÚNG một kích cỡ CỐ định (128 byte). tinhKichThuocConLai cộng dồn kích cỡ 7 trường đã định nghĩa (id, debitAccountId, creditAccountId, amount, timestamp, pendingId, flags = 36 byte), còn lại 92 byte dự trữ cho tương lai — không phải lãng phí, mà là 'chỗ trống có chủ đích' để thêm trường mới sau này KHÔNG phải đổi kích cỡ bản ghi."
locale: vi
track: co-so-du-lieu
module: so-cai-khong-sai-mot-xu
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.128-byte-cho-mot-but-toan]
requires: [db.bat-bien-tong-no-bang-tong-co]
concepts: [db.128-byte-cho-mot-but-toan]
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
Bất biến ĐÃ chứng minh được (bài trước) — nhưng `ButToan` Ở đây LÀ
một object TypeScript linh hoạt. TigerBeetle THẬT lưu bút toán khác
HẲN — dưới dạng gì, VÀ tại sao KHÔNG dùng JSON?
::::

::::explain{#128-byte-co-dinh}
TigerBeetle lưu MỖI Transfer dưới dạng một BẢN ghi NHỊ phân kích
thước CỐ định: đúng `128` byte, KHÔNG hơn KHÔNG kém — bất kể bút toán
đó "đơn giản" hay "phức tạp". `tinhKichThuocConLai` cộng dồn kích cỡ
các trường ĐÃ định nghĩa, TRỪ khỏi tổng `128`:

```typescript title=readonly
interface TruongDuLieu { ten: string; kichThuoc: number; }

const KICH_THUOC_BAN_GHI = 128;

const CAC_TRUONG: TruongDuLieu[] = [
  { ten: "id", kichThuoc: 4 },
  { ten: "debitAccountId", kichThuoc: 4 },
  { ten: "creditAccountId", kichThuoc: 4 },
  { ten: "amount", kichThuoc: 8 },
  { ten: "timestamp", kichThuoc: 8 },
  { ten: "pendingId", kichThuoc: 4 },
  { ten: "flags", kichThuoc: 4 },
];

function tinhKichThuocConLai(cacTruong: TruongDuLieu[], tongKichThuoc: number): number {
  let daDung = 0;
  for (const t of cacTruong) daDung += t.kichThuoc;
  return tongKichThuoc - daDung;
}

console.log("tong kich thuoc cac truong da dinh nghia:", CAC_TRUONG.reduce((s, t) => s + t.kichThuoc, 0));
console.log("con lai (du tru cho tuong lai):", tinhKichThuocConLai(CAC_TRUONG, KICH_THUOC_BAN_GHI));
```

```text title=readonly
tong kich thuoc cac truong da dinh nghia: 36
con lai (du tru cho tuong lai): 92
```

Bảy trường ĐÃ định nghĩa (`id`, `debitAccountId`, `creditAccountId`,
`amount`, `timestamp`, `pendingId`, `flags`) chiếm đúng `36` byte —
NHƯNG bản ghi VẪN chiếm ĐỦ `128` byte, để LẠI `92` byte "trống". Đây
KHÔNG phải lãng phí — LÀ chỗ dự trữ CÓ chủ đích để thêm trường MỚI
sau NÀY (VÍ dụ mã tiền tệ, ID người dùng) mà KHÔNG cần đổi kích cỡ
bản ghi, phá VỠ mọi offset ĐÃ tính từ trước.
::::

::::example{#tai-sao-co-dinh}
Kích cỡ CỐ định (thay VÌ mỗi bút toán một độ dài KHÁC nhau, tuỳ
"thông tin đính kèm") đổi lấy MỘT lợi ích LỚN: bản ghi thứ `N` LUÔN
nằm Ở đúng offset `N × 128` byte TRONG file — tìm được NGAY, không
cần đọc/parse những bản ghi ĐỨNG trước nó để biết chúng dài bao
nhiêu. Cái GIÁ: `92` byte "chưa dùng" TRÊN mỗi bản ghi — với hàng
triệu bút toán, đó LÀ không gian đĩa THẬT bị chiếm — một đánh đổi
CÓ ý thức, không phải sơ suất.
::::

::::predict{#doan-them-truong-moi commitOnce}
Thêm MỘT trường mới `maTienTe` (mã tiền tệ, `4` byte) VÀO
`CAC_TRUONG`. `tinhKichThuocConLai(CAC_TRUONG, 128)` (SAU khi thêm)
LÀ bao nhiêu?
:::opt{correct}
`88` — `7` trường CŨ (`36` byte) CỘNG `4` byte MỚI = `40` byte đã
dùng, `128 - 40 = 88` byte CÒN lại
:::
:::opt
VẪN LÀ `92` — thêm MỘT trường không ẢNH hưởng "chỗ trống dự trữ",
VÌ `92` byte đó LÀ một VÙNG cố định, tách RIÊNG khỏi các trường ĐÃ
đặt tên
::why
Trực giác NÀY nhầm "khoảng TRỐNG dự trữ" VỚI một VÙNG bộ nhớ CỐ định
tách BIỆT — thực ra `92` chỉ LÀ KẾT quả của một phép TRỪ, không phải
một vùng CÓ tên riêng.

Chỗ lệch: `tinhKichThuocConLai` tính LẠI TỪ ĐẦU mỗi lần gọi — CỘNG
dồn TẤT cả `kichThuoc` trong `cacTruong` HIỆN tại RỒI trừ khỏi
`tongKichThuoc`. Thêm MỘT trường `4` byte làm tổng ĐàDÙNG tăng từ
`36` lên `40`, nên PHẦN còn lại GIẢM từ `92` xuống ĐÚNG `88` — "chỗ
trống dự trữ" chỉ LÀ SỐ byte CHƯA được đặt tên, co LẠI mỗi khi có
trường MỚI.
::
:::
::::

::::code{#viet_tinh_kich_thuoc_con_lai}
Hoàn thiện `tinhKichThuocConLai` — cộng dồn `kichThuoc` của TỪNG
trường, TRỪ khỏi `tongKichThuoc`.

```typescript title=starter
interface TruongDuLieu { ten: string; kichThuoc: number; }

function tinhKichThuocConLai(cacTruong: TruongDuLieu[], tongKichThuoc: number): number {
  let daDung = 0;
  for (const t of cacTruong) {
    ___
  }
  return tongKichThuoc - daDung;
}

const CAC_TRUONG: TruongDuLieu[] = [
  { ten: "id", kichThuoc: 4 },
  { ten: "debitAccountId", kichThuoc: 4 },
  { ten: "creditAccountId", kichThuoc: 4 },
  { ten: "amount", kichThuoc: 8 },
  { ten: "timestamp", kichThuoc: 8 },
  { ten: "pendingId", kichThuoc: 4 },
  { ten: "flags", kichThuoc: 4 },
];
console.log(tinhKichThuocConLai(CAC_TRUONG, 128));
```

```typescript title=solution
interface TruongDuLieu { ten: string; kichThuoc: number; }

function tinhKichThuocConLai(cacTruong: TruongDuLieu[], tongKichThuoc: number): number {
  let daDung = 0;
  for (const t of cacTruong) {
    daDung += t.kichThuoc;
  }
  return tongKichThuoc - daDung;
}

const CAC_TRUONG: TruongDuLieu[] = [
  { ten: "id", kichThuoc: 4 },
  { ten: "debitAccountId", kichThuoc: 4 },
  { ten: "creditAccountId", kichThuoc: 4 },
  { ten: "amount", kichThuoc: 8 },
  { ten: "timestamp", kichThuoc: 8 },
  { ten: "pendingId", kichThuoc: 4 },
  { ten: "flags", kichThuoc: 4 },
];
console.log(tinhKichThuocConLai(CAC_TRUONG, 128));
```

```typescript title=test
if (tinhKichThuocConLai(CAC_TRUONG, 128) !== 92) throw new Error("7 truong dinh nghia san (36 byte) tren 128 byte phai con lai 92");

const themMaTienTe = [...CAC_TRUONG, { ten: "maTienTe", kichThuoc: 4 }];
if (tinhKichThuocConLai(themMaTienTe, 128) !== 88) throw new Error("them 1 truong 4 byte phai lam con lai giam tu 92 xuong 88");

if (tinhKichThuocConLai([], 128) !== 128) throw new Error("khong truong nao thi con lai phai bang dung tong kich thuoc ban dau");

const vuaKhopHet: TruongDuLieu[] = [{ ten: "x", kichThuoc: 128 }];
if (tinhKichThuocConLai(vuaKhopHet, 128) !== 0) throw new Error("mot truong dung bang tong kich thuoc thi con lai phai la 0");

const vuotQua: TruongDuLieu[] = [{ ten: "x", kichThuoc: 200 }];
if (tinhKichThuocConLai(vuotQua, 128) !== -72) throw new Error("truong vuot qua tong kich thuoc phai cho ket qua AM (128-200=-72), khong duoc chan o 0");
```

:::hints
- kind: attention
  body: "Cong don kichThuoc cua tung truong vao bien daDung -- mot dong."
- kind: strategy
  body: "daDung += t.kichThuoc;"
- kind: one-line
  body: "daDung += t.kichThuoc;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "92"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Layout ĐÃ có trên GIẤY — nhưng LÀM sao THẬT sự đóng gói bảy trường
đó VÀO đúng 128 byte nhị phân, KHÔNG phải chỉ liệt kê tên VÀ kích cỡ?
::::

::::reflect{#nghi-lai}
`tinhKichThuocConLai` chưa hề chạm tới BYTE thật nào — nó CHỈ CỘNG
trừ số nguyên TRÊN giấy. Nhưng bài NÀY đặt đúng NỀN tảng: MỘT bút
toán TigerBeetle không phải "một object CÓ mấy trường tuỳ Ý" — nó LÀ
MỘT khối `128` byte CỐ định, VÀ mỗi trường chiếm đúng một VỊ trí đã
định trước. Bước tiếp theo: THẬT sự ghi các trường ĐÓ vào đúng vị trí
NHỊ phân, dùng `DataView`.
::::

::::checkpoint{mastery=0.8}
::::
