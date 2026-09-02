---
id: ky-nghe-phan-mem.bao-mat-ung-dung.bam-cham-vs-bam-nhanh
title: "Băm CHẬM vs băm NHANH — vì sao SHA-256 sai cho mật khẩu"
summary: "SHA-256/MD5 được thiết kế NHANH (tốt cho checksum) — SAI cho mật khẩu, vì tốc độ giúp brute-force hàng tỉ tổ hợp/giây. bcrypt/argon2 CỐ Ý chậm (cost factor). Cùng 1 tỉ tổ hợp: SHA-256 ~0.1 giây, bcrypt ~11.5 NGÀY — chênh lệch triệu lần từ một lựa chọn thiết kế."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [bmud.slow-hash-vs-fast]
requires: [bmud.salt-per-user]
concepts: [bmud.slow-hash-vs-fast]
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
SHA-256 băm CỰC NHANH — hàng tỉ lần mỗi giây trên GPU. Dùng nó (có
salt đầy đủ, bài trước) để băm mật khẩu — còn vấn đề gì?
::::

::::explain{#toc-do-la-vu-khi-2-luoi}
SHA-256/MD5 được THIẾT KẾ để **NHANH** — TỐT cho checksum file, chữ
ký số (tốc độ LÀ ưu điểm ở NHỮNG việc đó). **SAI HOÀN TOÀN** cho mật
khẩu: tốc độ NHANH giúp kẻ tấn công **brute-force** (thử TỪNG tổ hợp
mật khẩu) HÀNG TỈ lần MỖI GIÂY trên GPU hiện đại — CÓ salt (bài
trước) chỉ ngăn được tấn công HÀNG LOẠT (rainbow table), KHÔNG ngăn
được việc THỬ TỪNG salt MỘT, nhanh:

```typescript
function ucLuongThoiGianGiay(soToHop: number, hashMoiGiay: number): number {
  return soToHop / hashMoiGiay;
}

// SHA-256 trên GPU hiện đại: ~10 tỉ hash/giây
console.log(ucLuongThoiGianGiay(1_000_000_000, 10_000_000_000));
```

```text
0.1
```

THỬ **MỘT TỈ** tổ hợp mật khẩu bằng SHA-256 trên GPU chỉ mất **0.1
GIÂY** — với mật khẩu ngắn/phổ biến (không gian tổ hợp nhỏ), toàn bộ
"vũ trụ" khả năng bị quét sạch gần như TỨC THỜI.
::::

::::example{#bcrypt-co-y-cham}
`bcrypt`/`argon2` **CỐ Ý** chậm — có **cost factor** (số vòng lặp
CẤU HÌNH được, TĂNG theo thời gian khi phần cứng mạnh lên) khiến MỖI
lần băm tốn **HÀNG MILI GIÂY** thay vì phần triệu giây:

```typescript title=readonly
function ucLuongThoiGianGiay(soToHop: number, hashMoiGiay: number): number {
  return soToHop / hashMoiGiay;
}

// bcrypt (cost factor cao): ~1000 hash/giây -- CHẬM HƠN SHA-256 hàng chục triệu lần
const giayBcrypt = ucLuongThoiGianGiay(1_000_000_000, 1_000);
console.log(giayBcrypt);
console.log(giayBcrypt / 86400); // đổi giây sang ngày
```

```text title=readonly
1000000
11.574074074074074
```

CÙNG **một tỉ** tổ hợp: SHA-256 mất `0.1` giây, bcrypt mất
`1,000,000` giây (**~11.5 NGÀY**) — chênh lệch **HÀNG TRIỆU LẦN** đến
từ **ĐÚNG MỘT** lựa chọn thiết kế: cố ý làm hàm băm CHẬM. Chậm với
MỘT LẦN thử (người dùng đăng nhập, chờ vài chục mili giây — KHÔNG
NHẬN RA) là chấp nhận được; chậm nhân lên HÀNG TỈ LẦN (kẻ tấn công
brute-force) là RÀO CẢN THỰC SỰ.
::::

::::predict{#doan-tang-cost-factor commitOnce}
```typescript
function ucLuongThoiGianGiay(soToHop: number, hashMoiGiay: number): number {
  return soToHop / hashMoiGiay;
}

// Cost factor TĂNG GẤP ĐÔI (từ 1000 lên 500 hash/giây -- MỖI hash giờ tốn gấp đôi thời gian)
const truoc = ucLuongThoiGianGiay(1_000_000_000, 1_000);
const sau = ucLuongThoiGianGiay(1_000_000_000, 500);
console.log(sau / truoc);
```

Dòng cuối in ra gì?

:::opt{correct}
`2`
:::

:::opt
`0.5` — vì `hashMoiGiay` GIẢM (từ 1000 xuống 500), và các đại lượng
trong công thức đều "giảm cùng nhau" theo tỉ lệ đó
::why
Gần đúng ở việc bạn nhớ ĐÚNG `hashMoiGiay` GIẢM một nửa (từ `1000`
xuống `500`) — quan sát về SỰ THAY ĐỔI của tham số đó đúng.

Chỗ lệch: `ucLuongThoiGianGiay` là phép CHIA `soToHop / hashMoiGiay`
— khi MẪU SỐ (`hashMoiGiay`) GIẢM một nửa MÀ TỬ SỐ (`soToHop`) giữ
NGUYÊN, KẾT QUẢ phép chia **TĂNG GẤP ĐÔI** (chia cho số NHỎ HƠN ra
kết quả LỚN HƠN), KHÔNG giảm. `truoc = 1,000,000` (giây), `sau =
2,000,000` (giây) — `sau / truoc = 2`. Đây LÀ ĐÚNG Ý NGHĨA thực tế:
`hashMoiGiay` GIẢM nghĩa là băm CHẬM HƠN (mỗi giây làm được ÍT hash
hơn), nên THỜI GIAN cần để thử hết `soToHop` tổ hợp phải TĂNG lên.
::
:::

:::opt
Máy báo lỗi biên dịch — `sau / truoc` không hợp lệ vì cả hai biến đều
được TÍNH TỪ cùng một hàm `ucLuongThoiGianGiay`, TypeScript không cho
phép chia hai giá trị "cùng nguồn gốc" như vậy
::why
Gần đúng ở việc bạn để ý `truoc` VÀ `sau` đều đến TỪ CÙNG một hàm
`ucLuongThoiGianGiay` — một quan sát ĐÚNG về NGUỒN GỐC của hai giá
trị đó.

Chỗ lệch: KHÔNG có khái niệm "cùng nguồn gốc" nào ảnh hưởng tới phép
toán trong TypeScript — `truoc` VÀ `sau` đều CHỈ là `number` (kiểu
TRẢ VỀ của `ucLuongThoiGianGiay`), phép chia GIỮA HAI `number` LUÔN
hợp lệ, bất kể chúng được TÍNH TỪ ĐÂU. Biên dịch VÀ chạy hoàn toàn
bình thường.
::
:::
::::

::::code{#viet_uc_luong_thoi_gian}
Tự viết công thức ước lượng thời gian.

```typescript title=starter
function ucLuongThoiGianGiay(soToHop: number, hashMoiGiay: number): number {
  return ___;
}

console.log(ucLuongThoiGianGiay(1_000_000_000, 10_000_000_000));
console.log(ucLuongThoiGianGiay(1_000_000_000, 1_000));
```

```typescript title=solution
function ucLuongThoiGianGiay(soToHop: number, hashMoiGiay: number): number {
  return soToHop / hashMoiGiay;
}

console.log(ucLuongThoiGianGiay(1_000_000_000, 10_000_000_000));
console.log(ucLuongThoiGianGiay(1_000_000_000, 1_000));
```

```typescript title=test
if (ucLuongThoiGianGiay(1000, 10) !== 100) throw new Error("1000 tổ hợp / 10 mỗi giây phải ra 100 giây");
if (ucLuongThoiGianGiay(1_000_000_000, 10_000_000_000) !== 0.1) throw new Error("một tỉ tổ hợp / mười tỉ mỗi giây phải ra 0.1 giây");
if (ucLuongThoiGianGiay(1_000_000_000, 1_000) !== 1_000_000) throw new Error("một tỉ tổ hợp / một nghìn mỗi giây phải ra một triệu giây");
```

:::hints
- kind: attention
  body: "Thời gian cần = số tổ hợp CHIA cho số hash mỗi giây (đơn vị: giây)."
- kind: strategy
  body: "soToHop / hashMoiGiay — phép chia đơn giản, tử số là tổng công việc, mẫu số là tốc độ."
- kind: one-line
  body: "___ = soToHop / hashMoiGiay"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "0.1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Băm CHẬM có chủ đích là hàng rào chống brute-force — SHA-256/MD5
nhanh SAI mục đích cho mật khẩu. Bước tiếp theo: đăng nhập xong,
server cần "nhớ" ai vừa đăng nhập — bằng cách nào?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sau khi xác thực THÀNH CÔNG, server cần "nhớ" bạn đã đăng nhập cho
CÁC request TIẾP THEO — session (server tự lưu) và JWT (token TỰ
CHỨA thông tin) là hai cách khác nhau. JWT hoạt động thế nào?
::::

::::checkpoint{mastery=0.8}
::::
