---
id: ky-nghe-phan-mem.bao-mat-ung-dung.vi-sao-khong-luu-mat-khau-tho
title: "Vì sao không lưu mật khẩu thô — mã hoá 2 chiều vs băm 1 chiều"
summary: "Mã hoá (encryption, HAI CHIỀU, giải mã được) SAI cho mật khẩu — LinkedIn 2012 (SHA-1 không salt) và Adobe 2013 (mã hoá đối xứng, cùng khoá cho mọi user) là hai ca thật. Băm (hashing, MỘT CHIỀU) là giải pháp ĐÚNG: cùng input luôn cùng output, nhưng không suy ngược được input từ output."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [bmud.password-hashing-why]
requires: [bmud.authn-vs-authz]
concepts: [bmud.password-hashing-why]
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
Xác thực cần so sánh mật khẩu người dùng nhập với mật khẩu ĐÃ lưu.
Lưu mật khẩu THẬT (đọc được) trong database — nguy hiểm cỡ nào?
::::

::::explain{#hai-ca-that}
Hai câu chuyện THẬT: **LinkedIn 2012** — mật khẩu băm bằng SHA-1
**KHÔNG salt**, 6.5 triệu mật khẩu bị BẺ trong VÀI NGÀY (SHA-1 nhanh,
brute-force dễ). **Adobe 2013** — TỆ HƠN: dùng **mã hoá ĐỐI XỨNG**
(encryption, HAI CHIỀU, giải mã được) với **CÙNG MỘT khoá** cho MỌI
user — hai user CÙNG mật khẩu ra **CÙNG ciphertext**, kẻ tấn công
NHÓM được các tài khoản DÙNG CHUNG mật khẩu mà KHÔNG CẦN giải mã gì
cả:

```typescript
function maHoaSai(matKhau: string): string {
  return `ENC(${matKhau})`; // hai chiều -- CÓ THỂ giải mã lại
}
console.log(maHoaSai("123456") === maHoaSai("123456"));
```

```text
true
```

Hai user CÙNG mật khẩu `"123456"` (rất phổ biến) ra **CÙNG** kết quả
mã hoá — kẻ tấn công CHỈ CẦN so sánh ciphertext GIỐNG NHAU để biết
NHIỀU tài khoản dùng CHUNG mật khẩu phổ biến, KHÔNG CẦN giải mã bất
kỳ cái nào. Vấn đề GỐC: **mã hoá là HAI CHIỀU** — bản chất được thiết
kế để GIẢI MÃ LẠI (dùng cho dữ liệu CẦN đọc lại sau, ví dụ số thẻ tín
dụng) — **SAI HOÀN TOÀN** cho mật khẩu, vì mật khẩu KHÔNG BAO GIỜ cần
đọc lại "nguyên văn", CHỈ cần SO SÁNH.
::::

::::example{#bam-mot-chieu}
Giải pháp ĐÚNG: **băm** (hashing) — **MỘT CHIỀU**, KHÔNG có hàm
"giải băm" nào tồn tại. Cùng input LUÔN cùng output (verify được),
nhưng KHÔNG suy ngược được input TỪ output:

```typescript title=readonly
function maHoaSai(matKhau: string): string {
  return `ENC(${matKhau})`;
}
function giaiMaSai(cipherText: string): string {
  // Mã hoá LUÔN có hàm giải mã tương ứng -- đây CHÍNH LÀ vấn đề
  return cipherText.slice(4, -1);
}

const matKhauGoc = "matKhauBiMat123";
const daMaHoa = maHoaSai(matKhauGoc);
console.log(daMaHoa);
console.log(giaiMaSai(daMaHoa) === matKhauGoc);

function bam(matKhau: string, muoi: string): string {
  return `HASH(${muoi}:${matKhau})`;
}
console.log(bam("123456", "salt-A") === bam("123456", "salt-A")); // cùng input -- cùng output
```

```text title=readonly
ENC(matKhauBiMat123)
true
true
```

`giaiMaSai(daMaHoa) === matKhauGoc` là `true` — mã hoá LUÔN đi kèm
MỘT hàm giải mã tương ứng (bản chất thiết kế của nó), NẾU kẻ tấn công
lấy được KHOÁ (hoặc thuật toán yếu), mật khẩu GỐC lộ NGUYÊN VĂN. `bam`
KHÔNG có hàm ngược — KHÔNG tồn tại `giaiBam(hash): string` nào có thể
trả lại `matKhau` gốc CHỈ TỪ kết quả `bam(...)` — đây LÀ tính chất
**MỘT CHIỀU** cần cho mật khẩu: server CHỈ cần BĂM LẠI mật khẩu người
dùng NHẬP LÚC đăng nhập, SO SÁNH với hash ĐÃ lưu — KHÔNG BAO GIỜ cần
"đọc lại" mật khẩu gốc.
::::

::::predict{#doan-bam-lai-de-xac-minh commitOnce}
```typescript
function bam(matKhau: string, muoi: string): string {
  return `HASH(${muoi}:${matKhau})`;
}

// LÚC ĐĂNG KÝ: lưu hash NÀY vào database
const hashDaLuu = bam("mkhau2024", "salt-cua-An");

// LÚC ĐĂNG NHẬP: An nhập LẠI đúng mật khẩu, server băm LẠI với CÙNG salt đã lưu
const hashLucDangNhap = bam("mkhau2024", "salt-cua-An");
console.log(hashDaLuu === hashLucDangNhap);
```

Dòng cuối in ra gì?

:::opt{correct}
`true`
:::

:::opt
`false` — vì `bam` được GỌI HAI LẦN Ở HAI THỜI ĐIỂM KHÁC NHAU (lúc
đăng ký, lúc đăng nhập), và hàm băm LUÔN sinh ra kết quả NGẪU NHIÊN
mỗi lần gọi, giống như UUID hay timestamp
::why
Gần đúng ở việc bạn để ý `bam` được GỌI HAI LẦN, Ở HAI "THỜI ĐIỂM"
khác nhau trong câu chuyện (đăng ký, đăng nhập) — một quan sát ĐÚNG
về NGỮ CẢNH sử dụng.

Chỗ lệch: hàm băm (kể cả hàm THẬT như bcrypt/argon2, VÀ hàm giả lập
`bam` ở đây) là **HÀM THUẦN** — CÙNG một CẶP tham số (`matKhau`,
`muoi`) LUÔN cho ra CÙNG một kết quả, KHÔNG có yếu tố ngẫu nhiên NÀO
được thêm vào MỖI LẦN gọi (khác UUID/timestamp, vốn CỐ Ý ngẫu nhiên/
thay đổi theo thời gian). Ở đây `matKhau = "mkhau2024"` VÀ `muoi =
"salt-cua-An"` GIỐNG HỆT nhau ở CẢ HAI lần gọi — kết quả PHẢI giống
hệt nhau. Đây CHÍNH LÀ lý do việc XÁC MINH mật khẩu hoạt động được:
băm LẠI với ĐÚNG salt đã lưu, SO SÁNH hai hash.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `bam` với CÙNG hai đối số ("mkhau2024",
"salt-cua-An") HAI LẦN không hợp lệ, TypeScript coi đó là lời gọi
TRÙNG LẶP vô nghĩa
::why
Gần đúng ở việc bạn để ý CÓ hai lời gọi `bam(...)` GIỐNG HỆT NHAU về
đối số — một quan sát ĐÚNG về mặt VĂN BẢN code.

Chỗ lệch: TypeScript (và JavaScript) KHÔNG có khái niệm "gọi hàm
trùng lặp là lỗi" — một hàm THUẦN được thiết kế để gọi NHIỀU LẦN với
CÙNG đối số VÀ nhận CÙNG kết quả (đây LÀ định nghĩa của "thuần", đã
học từ T4.1). Biên dịch VÀ chạy hoàn toàn bình thường, không có ràng
buộc nào về việc gọi trùng.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Băm MỘT CHIỀU, không giải ngược được — mã hoá HAI CHIỀU, SAI cho mật
khẩu. Bước tiếp theo: cùng mật khẩu, khác user — hash có nên giống
nhau không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nếu hai người dùng CÙNG chọn mật khẩu `"123456"`, và cả hai đều dùng
`bam(matKhau, mot-salt-co-dinh-chung)` — hash của họ có GIỐNG NHAU
không? Vấn đề gì xảy ra nếu GIỐNG?
::::

::::checkpoint{mastery=0.8}
::::
