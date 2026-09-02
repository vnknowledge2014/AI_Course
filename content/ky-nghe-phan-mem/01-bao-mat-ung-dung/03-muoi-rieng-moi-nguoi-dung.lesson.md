---
id: ky-nghe-phan-mem.bao-mat-ung-dung.muoi-rieng-moi-nguoi-dung
title: "Salt — muối riêng mỗi người dùng, phá rainbow table"
summary: "Rainbow table = bảng tra sẵn hash→mật khẩu, chỉ hiệu quả khi KHÔNG có salt. Salt (chuỗi ngẫu nhiên, RIÊNG mỗi user, lưu CẠNH hash) khiến CÙNG mật khẩu ra HAI hash khác nhau ở hai user khác nhau. Salt KHÔNG cần bí mật — chỉ cần DUY NHẤT."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [bmud.salt-per-user]
requires: [bmud.password-hashing-why]
concepts: [bmud.salt-per-user]
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
Hai người dùng CÙNG chọn mật khẩu `"123456"` (rất phổ biến). Băm
BẰNG NHAU — vấn đề gì?
::::

::::explain{#rainbow-table-va-salt}
**Rainbow table**: bảng tra SẴN, TÍNH TRƯỚC MỘT LẦN, ánh xạ hash→mật
khẩu PHỔ BIẾN — dùng LẠI được cho **MỌI** nạn nhân, MIỄN LÀ hash
KHÔNG có salt (cùng mật khẩu LUÔN ra cùng hash, tra MỘT LẦN, áp dụng
CHO MỌI hash TRÙNG). **Salt** (chuỗi NGẪU NHIÊN, **RIÊNG MỖI USER**,
lưu **CẠNH** hash trong database) phá vỡ điều đó:

```typescript
function bam(matKhau: string, muoi: string): string {
  return `HASH(${muoi}:${matKhau})`;
}

// Hai user CÙNG mật khẩu, salt RIÊNG mỗi người
const hashAn = bam("matkhau123", "muoi-cua-An");
const hashBinh = bam("matkhau123", "muoi-cua-Binh");
console.log(hashAn === hashBinh);
```

```text
false
```

CÙNG mật khẩu (`"matkhau123"`) nhưng salt KHÁC NHAU → hash KHÁC
NHAU. Kẻ tấn công KHÔNG THỂ dùng MỘT rainbow table CHO CẢ HAI user —
phải tính RIÊNG rainbow table CHO TỪNG salt (mỗi salt LÀ một "bảng
tra" RIÊNG, không dùng lại được), khiến việc bẻ khoá HÀNG LOẠT trở
nên VÔ NGHĨA về mặt chi phí.
::::

::::example{#xac-minh-bang-cach-bam-lai}
Xác minh mật khẩu KHÔNG BAO GIỜ "giải băm" — CHỈ **băm LẠI** với
salt ĐÃ LƯU, rồi **so sánh** hai hash:

```typescript title=readonly
function bam(matKhau: string, muoi: string): string {
  return `HASH(${muoi}:${matKhau})`;
}
function xacMinh(matKhauNhap: string, muoiDaLuu: string, hashDaLuu: string): boolean {
  return bam(matKhauNhap, muoiDaLuu) === hashDaLuu;
}

type HangNguoiDung = { muoi: string; hash: string };
const hangAn: HangNguoiDung = { muoi: "muoi-cua-An", hash: bam("matkhau123", "muoi-cua-An") };
console.log(JSON.stringify(hangAn));
console.log(xacMinh("matkhau123", hangAn.muoi, hangAn.hash));
```

```text title=readonly
{"muoi":"muoi-cua-An","hash":"HASH(muoi-cua-An:matkhau123)"}
true
```

`hangAn.muoi` (salt) **HIỆN RÕ** trong "database", KHÔNG mã hoá,
KHÔNG che giấu — salt **KHÔNG CẦN bí mật**, nó chỉ cần **DUY NHẤT**
(mỗi user một salt KHÁC NHAU). Điều này KHÁC HẲN với mật khẩu (PHẢI
bí mật) — nhiệm vụ CỦA salt CHỈ là làm cho hash CÙNG một mật khẩu
KHÁC NHAU giữa các user, không phải để "giấu" điều gì.
::::

::::predict{#doan-salt-khong-can-bi-mat commitOnce}
```typescript
function bam(matKhau: string, muoi: string): string {
  return `HASH(${muoi}:${matKhau})`;
}
function xacMinh(matKhauNhap: string, muoiDaLuu: string, hashDaLuu: string): boolean {
  return bam(matKhauNhap, muoiDaLuu) === hashDaLuu;
}

// Kẻ tấn công XEM ĐƯỢC database, biết CẢ salt LẪN hash của An
const muoiBiLo = "muoi-cua-An";
const hashBiLo = bam("matkhau123", muoiBiLo);

// Kẻ tấn công THỬ một mật khẩu SAI, dùng ĐÚNG salt đã lộ
console.log(xacMinh("mat-khau-doan-bua", muoiBiLo, hashBiLo));
```

Dòng cuối in ra gì?

:::opt{correct}
`false`
:::

:::opt
`true` — vì kẻ tấn công ĐÃ có CẢ salt LẪN hash (đủ mọi thứ cần), nên
`xacMinh` PHẢI chấp nhận BẤT KỲ mật khẩu nào được thử với salt ĐÚNG
::why
Gần đúng ở việc bạn nhớ ĐÚNG kẻ tấn công CÓ đủ `muoiBiLo` VÀ
`hashBiLo` — một quan sát THẬT về tình huống rò rỉ database (đây
CHÍNH LÀ lý do salt không cần giữ bí mật — nó CÓ THỂ lộ mà vẫn AN
TOÀN nếu mật khẩu đủ mạnh).

Chỗ lệch: `xacMinh` KHÔNG "chấp nhận vì đã biết salt/hash" — nó
**TÍNH LẠI** `bam("mat-khau-doan-bua", "muoi-cua-An")` (băm mật khẩu
kẻ tấn công VỪA THỬ, với salt ĐÚNG) rồi SO SÁNH kết quả ĐÓ với
`hashBiLo`. Vì `"mat-khau-doan-bua"` KHÁC `"matkhau123"` (mật khẩu
GỐC ĐÃ dùng để tạo `hashBiLo`), hai chuỗi băm ra KHÁC NHAU — `===`
cho `false`. Biết salt CHỈ giúp kẻ tấn công TÍNH ĐÚNG công thức băm
— KHÔNG giúp họ ĐOÁN ra mật khẩu, đó vẫn phải THỬ TỪNG khả năng.
::
:::

:::opt
Máy báo lỗi biên dịch — dùng LẠI biến `muoiBiLo` (đã khai ở dòng
trước) làm tham số cho `xacMinh` không hợp lệ, vì salt "bị lộ" không
còn cùng KIỂU với salt gốc trong hệ thống
::why
Gần đúng ở việc bạn nghĩ tới việc "salt bị lộ" LÀ một trạng thái ĐẶC
BIỆT — một trực giác NGHIỆP VỤ hợp lý (dữ liệu "đã lộ" nghe NGUY HIỂM
hơn dữ liệu bình thường).

Chỗ lệch: KHÔNG có khái niệm "kiểu dữ liệu bị lộ" trong TypeScript —
`muoiBiLo` chỉ là MỘT biến kiểu `string` bình thường, giống HỆT mọi
salt khác về mặt KIỂU. Việc dữ liệu "lộ" hay "an toàn" là một khái
niệm NGỮ NGHĨA (con người hiểu), không phải một RÀNG BUỘC kiểu mà
compiler kiểm tra được. Biên dịch sạch.
::
:::
::::

::::code{#viet_bam_va_xacminh}
Tự viết `bam` và `xacMinh`.

```typescript title=starter
function bam(matKhau: string, muoi: string): string {
  return ___;
}
function xacMinh(matKhauNhap: string, muoiDaLuu: string, hashDaLuu: string): boolean {
  return ___;
}

const hash = bam("matkhau123", "muoi-A");
console.log(xacMinh("matkhau123", "muoi-A", hash));
```

```typescript title=solution
function bam(matKhau: string, muoi: string): string {
  return `HASH(${muoi}:${matKhau})`;
}
function xacMinh(matKhauNhap: string, muoiDaLuu: string, hashDaLuu: string): boolean {
  return bam(matKhauNhap, muoiDaLuu) === hashDaLuu;
}

const hash = bam("matkhau123", "muoi-A");
console.log(xacMinh("matkhau123", "muoi-A", hash));
```

```typescript title=test
const hashAnTest = bam("matkhau123", "muoi-An");
const hashBinhTest = bam("matkhau123", "muoi-Binh");
if (hashAnTest === hashBinhTest) throw new Error("cùng mật khẩu nhưng salt khác nhau phải ra hash KHÁC NHAU");

if (!xacMinh("matkhau123", "muoi-An", hashAnTest)) throw new Error("mật khẩu ĐÚNG với salt ĐÚNG phải xác minh thành công");
if (xacMinh("mat-khau-sai", "muoi-An", hashAnTest)) throw new Error("mật khẩu SAI phải xác minh thất bại");
if (xacMinh("matkhau123", "muoi-Binh", hashAnTest)) throw new Error("salt SAI (không khớp lúc tạo hash) phải xác minh thất bại");

if (bam("matkhau123", "muoi-An") !== bam("matkhau123", "muoi-An")) throw new Error("cùng đúng hai tham số phải luôn ra cùng kết quả (hàm thuần)");
```

:::hints
- kind: attention
  body: "bam: ghép muoi và matKhau vào một chuỗi (template string). xacMinh: băm LẠI matKhauNhap với muoiDaLuu, so sánh kết quả với hashDaLuu bằng ===."
- kind: strategy
  body: 'HASH(${muoi}:${matKhau}) : bam(matKhauNhap, muoiDaLuu) === hashDaLuu — bam ghép chuỗi, xacMinh so sánh kết quả băm lại.'
- kind: one-line
  body: "___ (bam) = `HASH(${muoi}:${matKhau})`\n___ (xacMinh) = bam(matKhauNhap, muoiDaLuu) === hashDaLuu"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Salt: riêng mỗi user, phá rainbow table, không cần bí mật — chỉ cần
duy nhất. Bước tiếp theo: salt xong rồi, còn TỐC ĐỘ băm thì sao?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

SHA-256 băm CỰC NHANH (hàng tỉ lần/giây trên GPU). Dùng SHA-256 (có
salt đầy đủ) để băm mật khẩu — còn vấn đề gì không?
::::

::::checkpoint{mastery=0.8}
::::
