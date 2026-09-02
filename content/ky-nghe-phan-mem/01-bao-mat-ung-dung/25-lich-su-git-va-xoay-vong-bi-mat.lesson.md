---
id: ky-nghe-phan-mem.bao-mat-ung-dung.lich-su-git-va-xoay-vong-bi-mat
title: "Lịch sử Git không xoá được — bí mật lộ phải XOAY VÒNG, không phải XOÁ"
summary: "Commit một bí mật rồi commit \"xoá\" nó Ở LẦN SAU KHÔNG làm nó biến mất — lịch sử git mặc định BẤT BIẾN, ai clone repo VẪN thấy bí mật ở commit cũ. Giải pháp DUY NHẤT đáng tin: XOAY VÒNG (tạo khoá mới, vô hiệu khoá cũ), không phải cố dọn lịch sử."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 25
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [bmud.secrets-git-rotate]
requires: [bmud.secrets-env-fail-fast]
concepts: [bmud.secrets-git-rotate]
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
Bạn LỠ commit một khoá API vào Git, rồi commit "xoá" nó Ở LẦN SAU.
Khoá đó CÒN AN TOÀN không?
::::

::::explain{#lich-su-bat-bien}
Câu chuyện THẬT (bài THUẦN TƯỜNG THUẬT — Git THẬT KHÔNG chạy được
trong sandbox, minh hoạ bằng DỮ LIỆU mô tả lịch sử): commit MỘT bí
mật, rồi commit **"XOÁ"** nó Ở LẦN SAU **KHÔNG** làm nó BIẾN MẤT:

```typescript
type LichSuCommit = { maCommit: string; noiDung: string };

const lichSu: LichSuCommit[] = [
  { maCommit: "c1", noiDung: 'const apiKey = "sk_live_ABC123";' },
  { maCommit: "c2", noiDung: "const apiKey = process.env.API_KEY;" },
];

function timBiMatTrongLichSu(lichSuTruyen: LichSuCommit[]): string[] {
  return lichSuTruyen.filter((c) => c.noiDung.includes("sk_live_")).map((c) => c.maCommit);
}

console.log(timBiMatTrongLichSu(lichSu));
```

```text
["c1"]
```

`c2` "xoá" khoá API (chuyển sang đọc từ `process.env`, ĐÚNG kỹ thuật
bài 24) — NHƯNG `c1` **VẪN NẰM NGUYÊN** trong `lichSu` (mảng, giống
lịch sử Git THẬT — **MẶC ĐỊNH BẤT BIẾN**, các commit CŨ không "biến
mất" khi có commit MỚI). AI đó `git clone` repo, `git log`, HOẶC CHỈ
CẦN `git checkout c1` — **VẪN THẤY** khoá API đó, NGUYÊN VĂN.
::::

::::example{#twitch-2021-va-xoay-vong}
Ca THẬT: rò rỉ **Twitch 2021** (toàn bộ mã nguồn nội bộ bị công khai)
sống sót lâu vì bí mật **CHƯA TỪNG** nằm trong CODE (nằm Ở hạ tầng CI
KHÁC) — MINH HOẠ **NGƯỢC LẠI**: NẾU bí mật ĐÃ TỪNG vào Git, giải pháp
DUY NHẤT ĐÁNG TIN là **XOAY VÒNG** (tạo khoá MỚI, VÔ HIỆU khoá CŨ ở
NHÀ CUNG CẤP DỊCH VỤ — ví dụ Stripe, AWS), **KHÔNG PHẢI** cố "dọn"
lịch sử:

```typescript title=readonly
type LichSuCommit = { maCommit: string; noiDung: string };
const lichSu: LichSuCommit[] = [
  { maCommit: "c1", noiDung: 'const apiKey = "sk_live_ABC123";' },
  { maCommit: "c2", noiDung: "const apiKey = process.env.API_KEY;" },
];
function timBiMatTrongLichSu(lichSuTruyen: LichSuCommit[]): string[] {
  return lichSuTruyen.filter((c) => c.noiDung.includes("sk_live_")).map((c) => c.maCommit);
}

// Dù đã "xoá" ở c2, khoá VẪN đọc được ở c1 -- công cụ như BFG Repo-Cleaner/git filter-branch
// CHỈ giảm rủi ro (dọn history CŨ), KHÔNG đảm bảo TUYỆT ĐỐI nếu repo ĐÃ TỪNG push public
// (ai đó CÓ THỂ đã clone/fork/cache TRƯỚC khi dọn)
const conLo = timBiMatTrongLichSu(lichSu).length > 0;
console.log(conLo ? "PHẢI xoay vòng khoá này ở Stripe -- không thể tin lịch sử đã 'sạch'" : "an toàn");
```

```text title=readonly
PHẢI xoay vòng khoá này ở Stripe -- không thể tin lịch sử đã 'sạch'
```

"Dọn" lịch sử (BFG Repo-Cleaner, `git filter-branch`) CHỈ **GIẢM RỦI
RO** — KHÔNG đảm bảo TUYỆT ĐỐI nếu repo **ĐÃ TỪNG** `push` PUBLIC (ai
đó CÓ THỂ đã `clone`/fork/cache TRƯỚC khi dọn — bản sao ĐÓ vẫn giữ
NGUYÊN lịch sử CŨ, hoàn toàn NGOÀI TẦM KIỂM SOÁT của repo GỐC). Khoá
đã lộ PHẢI được coi LÀ **XÂM PHẠM VĨNH VIỄN** — hành động ĐÚNG DUY
NHẤT LÀ xoay vòng NÓ tại nhà cung cấp (thu hồi khoá CŨ, phát khoá
MỚI), KHÔNG PHỤ THUỘC vào việc "dọn sạch" lịch sử có THÀNH CÔNG hay
không.
::::

::::predict{#doan-quiz-dung-sai commitOnce}
```typescript
type LichSuCommit = { maCommit: string; noiDung: string };
const lichSu: LichSuCommit[] = [
  { maCommit: "c1", noiDung: 'const dbPassword = "matkhau123";' },
  { maCommit: "c2", noiDung: "// đã xoá mật khẩu hardcode, dùng process.env" },
];
function timBiMatTrongLichSu(lichSuTruyen: LichSuCommit[]): string[] {
  return lichSuTruyen.filter((c) => c.noiDung.includes("matkhau123")).map((c) => c.maCommit);
}

// "Đã xoay vòng mật khẩu ở database THẬT (đổi sang mật khẩu MỚI, vô hiệu mật khẩu CŨ)"
// -- nhưng KHÔNG động gì tới lịch sử Git
const daXoayVong = true;
const conThayTrongLichSu = timBiMatTrongLichSu(lichSu).length > 0;
console.log(daXoayVong && conThayTrongLichSu ? "an_toan_du_lich_su_con_luu" : "van_nguy_hiem");
```

Xoay vòng RỒI (mật khẩu THẬT đã đổi), NHƯNG lịch sử Git VẪN còn lưu
mật khẩu CŨ. Dòng cuối in ra gì?

:::opt{correct}
`an_toan_du_lich_su_con_luu`
:::

:::opt
`van_nguy_hiem` — vì lịch sử Git VẪN chứa mật khẩu, nên MIỄN LÀ mật
khẩu ĐÓ còn "đọc được" ở đâu đó, hệ thống VẪN bị coi LÀ nguy hiểm,
BẤT KỂ đã xoay vòng hay chưa
::why
Gần đúng ở việc bạn nhớ ĐÚNG `timBiMatTrongLichSu(lichSu)` VẪN tìm
thấy `c1` (mật khẩu CŨ vẫn LƯU trong lịch sử) — quan sát ĐÓ đúng, và
đúng LÀ MỘT sự thật ĐÁNG LO nếu ĐỨNG RIÊNG.

Chỗ lệch: `conThayTrongLichSu` là `true` — NHƯNG biểu thức KIỂM TRA
là `daXoayVong && conThayTrongLichSu`, VÀ khi `daXoayVong` cũng LÀ
`true`, kết quả CỦA `&&` là `true` — dòng `console.log` in
`"an_toan_du_lich_su_con_luu"` (nhánh ĐẦU của toán tử `? :`). Đây LÀ
ĐÚNG bài học cốt lõi: mật khẩu **CŨ** (`"matkhau123"`) VẪN nằm TRONG
lịch sử, NHƯNG NÓ **KHÔNG CÒN LÀ mật khẩu THẬT** của database nữa
(ĐÃ xoay vòng sang mật khẩu MỚI) — kẻ tấn công ĐỌC được mật khẩu CŨ
từ lịch sử Git thì CŨNG KHÔNG dùng được gì (nó đã VÔ HIỆU). Xoay
vòng LÀM CHO việc "lịch sử còn lưu bí mật CŨ" TRỞ NÊN VÔ HẠI.
::
:::

:::opt
Máy báo lỗi biên dịch — biến `daXoayVong` khai `const` GIÁ TRỊ CỐ
ĐỊNH `true`, TypeScript không cho phép DÙNG nó trong biểu thức `&&`
kết hợp với MỘT giá trị được TÍNH TOÁN động (`conThayTrongLichSu`)
::why
Gần đúng ở việc bạn để ý `daXoayVong` là `const` với giá trị CỐ ĐỊNH,
CÒN `conThayTrongLichSu` được TÍNH TOÁN (gọi hàm) — một quan sát ĐÚNG
về NGUỒN GỐC khác nhau của hai biến.

Chỗ lệch: TypeScript KHÔNG có ràng buộc nào về việc "biến CỐ ĐỊNH"
không được KẾT HỢP với "biến TÍNH TOÁN" trong biểu thức — MỌI `boolean`
(bất kể ĐẾN từ đâu: literal, kết quả hàm, so sánh...) đều dùng được
TRONG `&&`/`||` như NHAU. Biên dịch sạch.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lịch sử Git bất biến — bí mật lộ phải XOAY VÒNG, không phải cố xoá.
Bước cuối track: ghép TẤT CẢ kỹ thuật runtime thành MỘT pipeline
middleware.
::::

::::reflect{#nghi-lai}
Bài BOSS cuối track. Ghép rate limiting, security headers, giới hạn
kích thước, kiểm Content-Type thành MỘT pipeline middleware — mỗi
middleware CHỈ chịu trách nhiệm MỘT mối lo, ghép được theo THỨ TỰ bất
kỳ. Trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
