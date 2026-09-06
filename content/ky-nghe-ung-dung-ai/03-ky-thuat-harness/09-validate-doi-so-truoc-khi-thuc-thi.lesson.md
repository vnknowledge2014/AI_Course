---
id: ky-nghe-ung-dung-ai.ky-thuat-harness.validate-doi-so-truoc-khi-thuc-thi
title: "Validate đối số tool-call TRƯỚC khi thực thi"
summary: "validateDoiSo(doiSoTho: unknown): KetQuaValidate la discriminated union BA bien the -- {loai:'hop_le';doiSo} | {loai:'thieu_khoa';khoaThieu} | {loai:'sai_kieu';khoa;kieuThuc} -- kiem TUAN TU: khong phai object -> thieu_khoa; thieu khoa maDonHang -> thieu_khoa; co khoa nhung SAI kieu (khong phai string) -> sai_kieu (kem ten khoa VA kieu THAT SU gap phai); dung ca hai -> hop_le kem doiSo DA duoc thu hep kieu. goiToolCoValidate(doiSoTho) LA hang rao harness: goi validateDoiSo TRUOC, chi khi loai==='hop_le' moi goi thucThiTraCuuDonHang (tool THAT) -- moi truong hop khong hop le deu bi CHAN, demThucThi.soLan (bo dem toan cuc) KHONG tang len du goi goiToolCoValidate bao nhieu lan voi doi so sai. Khac q9.1a bai 5 (validate output MODEL tra ve, phan loai nhiem vu): bai nay validate DAU VAO cua chinh mot loi goi tool, dat truoc buoc thuc thi trong pipeline harness."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-harness
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.validate-doi-so-truoc-khi-thuc-thi]
requires: [kna.circuit-breaker-giua-nhieu-yeu-cau]
concepts: [kna.validate-doi-so-truoc-khi-thuc-thi]
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
Timeout (bài `7`) VÀ circuit breaker (bài `8`) đều giả định MỘT điều:
đối số gửi cho tool ĐÃ đúng hình dạng, VÀ vấn đề DUY NHẤT LÀ tool có
trả lời kịp hay không. Nhưng có một lớp lỗi XẢY RA SỚM HƠN NHIỀU: model
tự soạn ra đối số cho lời gọi tool, VÀ nó có thể quên một khoá, hoặc
gửi sai kiểu (một chuỗi số thay vì số THẬT). Bài này chặn lớp lỗi đó
TRƯỚC KHI bất kỳ tool THẬT nào — kể cả một tool hoàn hảo — kịp chạm
vào đối số sai.
::::

::::explain{#ba_bien_the_cua_validate}
`validateDoiSo` nhận MỘT đối số CHƯA được tin tưởng (`unknown` — kiểu
TypeScript nói "có thể LÀ bất cứ thứ gì"), VÀ trả về MỘT trong BA biến
thể: `"hop_le"` (đối số ĐÚNG hình dạng, kèm `doiSo` ĐÃ được thu hẹp
kiểu an toàn), `"thieu_khoa"` (thiếu khoá bắt buộc), hoặc `"sai_kieu"`
(CÓ khoá đó, nhưng giá trị SAI kiểu — kèm TÊN khoá VÀ kiểu THẬT SỰ gặp
phải, không phải một thông báo lỗi chung chung):

```typescript title=readonly
interface DoiSoTraCuuDonHang {
  maDonHang: string;
}

type KetQuaValidate =
  | { loai: "hop_le"; doiSo: DoiSoTraCuuDonHang }
  | { loai: "thieu_khoa"; khoaThieu: string[] }
  | { loai: "sai_kieu"; khoa: string; kieuThuc: string };

function validateDoiSo(doiSoTho: unknown): KetQuaValidate {
  if (typeof doiSoTho !== "object" || doiSoTho === null) {
    return { loai: "thieu_khoa", khoaThieu: ["maDonHang"] };
  }
  const ghi = doiSoTho as Record<string, unknown>;
  if (!("maDonHang" in ghi)) {
    return { loai: "thieu_khoa", khoaThieu: ["maDonHang"] };
  }
  if (typeof ghi.maDonHang !== "string") {
    return { loai: "sai_kieu", khoa: "maDonHang", kieuThuc: typeof ghi.maDonHang };
  }
  return { loai: "hop_le", doiSo: { maDonHang: ghi.maDonHang } };
}

console.log(JSON.stringify(validateDoiSo({ maDonHang: "XY789" })));
console.log(JSON.stringify(validateDoiSo({ ma: "XY789" })));
console.log(JSON.stringify(validateDoiSo({ maDonHang: 789 })));
```

```text title=readonly
{"loai":"hop_le","doiSo":{"maDonHang":"XY789"}}
{"loai":"thieu_khoa","khoaThieu":["maDonHang"]}
{"loai":"sai_kieu","khoa":"maDonHang","kieuThuc":"number"}
```

Ba đối số TRÔNG gần giống nhau (đều LÀ object có MỘT khoá) nhưng cho
BA kết quả khác hẳn NHAU Ở CẤU TRÚC: `{ maDonHang: "XY789" }` đúng cả
tên khoá LẪN kiểu; `{ ma: "XY789" }` sai TÊN khoá (khoá `maDonHang`
không tồn tại); `{ maDonHang: 789 }` đúng TÊN khoá nhưng giá trị LÀ
`number`, không phải `string` — VÀ `kieuThuc: "number"` ghi lại CHÍNH
XÁC kiểu THẬT gặp phải, không chỉ nói "sai kiểu".
::::

::::example{#hang_rao_truoc_khi_thuc_thi}
`goiToolCoValidate` LÀ hàng rao harness: gọi `validateDoiSo` TRƯỚC,
CHỈ khi kết quả LÀ `"hop_le"` mới gọi `thucThiTraCuuDonHang` — tool
THẬT. `demThucThi.soLan` đếm số lần tool THẬT SỰ được gọi, để chứng
minh mọi đối số KHÔNG hợp lệ đều bị chặn Ở BƯỚC VALIDATE, không hề
chạm tới tool:

```typescript title=readonly
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

type KetQuaGoiTool = KetQua<string, string>;

interface DoiSoTraCuuDonHang {
  maDonHang: string;
}

type KetQuaValidate =
  | { loai: "hop_le"; doiSo: DoiSoTraCuuDonHang }
  | { loai: "thieu_khoa"; khoaThieu: string[] }
  | { loai: "sai_kieu"; khoa: string; kieuThuc: string };

function validateDoiSo(doiSoTho: unknown): KetQuaValidate {
  if (typeof doiSoTho !== "object" || doiSoTho === null) {
    return { loai: "thieu_khoa", khoaThieu: ["maDonHang"] };
  }
  const ghi = doiSoTho as Record<string, unknown>;
  if (!("maDonHang" in ghi)) {
    return { loai: "thieu_khoa", khoaThieu: ["maDonHang"] };
  }
  if (typeof ghi.maDonHang !== "string") {
    return { loai: "sai_kieu", khoa: "maDonHang", kieuThuc: typeof ghi.maDonHang };
  }
  return { loai: "hop_le", doiSo: { maDonHang: ghi.maDonHang } };
}

const demThucThi = { soLan: 0 };

function thucThiTraCuuDonHang(doiSo: DoiSoTraCuuDonHang): KetQuaGoiTool {
  demThucThi.soLan++;
  return { thanhCong: true, giaTri: `don_hang_${doiSo.maDonHang}` };
}

function goiToolCoValidate(doiSoTho: unknown): KetQuaGoiTool {
  const kqValidate = validateDoiSo(doiSoTho);
  if (kqValidate.loai !== "hop_le") {
    return { thanhCong: false, loi: `loi_validate_${kqValidate.loai}` };
  }
  return thucThiTraCuuDonHang(kqValidate.doiSo);
}

console.log(JSON.stringify(goiToolCoValidate({ maDonHang: "XY789" })));
console.log(JSON.stringify(goiToolCoValidate({ ma: "XY789" })));
console.log(JSON.stringify(goiToolCoValidate({ maDonHang: 789 })));
console.log("so lan THUC THI that:", demThucThi.soLan);
```

```text title=readonly
{"thanhCong":true,"giaTri":"don_hang_XY789"}
{"thanhCong":false,"loi":"loi_validate_thieu_khoa"}
{"thanhCong":false,"loi":"loi_validate_sai_kieu"}
so lan THUC THI that: 1
```

BA lần gọi `goiToolCoValidate`, nhưng `demThucThi.soLan` CHỈ LÀ `1` —
KHÔNG phải `3`. Hai lần gọi VỚI đối số sai (thiếu khoá, sai kiểu) đều
dừng lại NGAY Ở `validateDoiSo`, KHÔNG bao giờ chạm tới
`thucThiTraCuuDonHang`. Đây LÀ điểm khác biệt cốt lõi VỚI q9.1a bài
`5` (`tool-schema-la-domain-modeling`): Ở đó, discriminated union
validate OUTPUT mà MODEL đã trả về (một kết quả phân loại); Ở đây, nó
validate ĐẦU VÀO của một lời gọi tool, VÀ đặt phép validate đó LÀM
CỔNG GÁC đứng TRƯỚC bước thực thi trong CHÍNH pipeline harness.
::::

::::predict{#doan-doi-so-sai-kieu commitOnce}
Gọi `goiToolCoValidate({ maDonHang: 789 })` — đúng TÊN khoá, nhưng giá
trị LÀ `number` (`789`), không phải `string`. `thucThiTraCuuDonHang`
(tool THẬT) có được gọi không?

:::opt{correct}
Không — `validateDoiSo` trả về `{ loai: "sai_kieu", ... }` (vì
`typeof ghi.maDonHang !== "string"` LÀ `true` khi giá trị LÀ `789`);
`kqValidate.loai !== "hop_le"` đúng, `goiToolCoValidate` trả lỗi NGAY,
`thucThiTraCuuDonHang` không bao giờ được gọi
:::
:::opt
Có — `789` LÀ một giá trị hợp lệ VỀ Ý NGHĨA (một mã đơn hàng dạng số),
nên `validateDoiSo` sẽ TỰ ép kiểu nó thành chuỗi trước khi thực thi
::why
Nhầm rằng TypeScript/JavaScript tự động ép kiểu ngầm định giữa
`number` VÀ `string` khi kiểm tra — nhưng `typeof ghi.maDonHang` LUÔN
trả về đúng kiểu RUNTIME thật của giá trị, không hề quy đổi.

Chỗ lệch: `typeof 789` LÀ `"number"`, không phải `"string"` — điều
kiện `!== "string"` đúng NGAY LẬP TỨC, VÀ `validateDoiSo` return
`sai_kieu` mà không hề thử ép kiểu gì cả.
::
:::
:::opt
Không xác định được — phụ thuộc thứ tự nhánh `if` nào được viết trước
trong `validateDoiSo`
::why
Nhầm rằng có sự MƠ HỒ về thứ tự kiểm tra — nhưng `validateDoiSo` kiểm
TUẦN TỰ, RÕ RÀNG: trước tiên LÀ object hợp lệ hay không, rồi khoá
`maDonHang` có TỒN TẠI không, rồi MỚI tới kiểu của giá trị đó. Với
`{ maDonHang: 789 }`, hai bước đầu đều qua (LÀ object, CÓ khoá), nên
CHỈ bước kiểm kiểu (bước thứ ba) LÀ bước quyết định, không có nhánh
nào chạy song song hay không rõ ràng.
::
:::
::::

::::code{#viet_validate_doi_so}
Hoàn thiện `validateDoiSo` — kiểm TUẦN TỰ: KHÔNG phải object (hoặc LÀ
`null`) thì `"thieu_khoa"`; LÀ object nhưng KHÔNG có khoá `maDonHang`
thì `"thieu_khoa"`; CÓ khoá nhưng giá trị KHÔNG phải `string` thì
`"sai_kieu"` (kèm `khoa` VÀ `kieuThuc` LÀ kết quả của `typeof`); còn
lại LÀ `"hop_le"` (kèm `doiSo` đã thu hẹp kiểu). Hoàn thiện
`goiToolCoValidate` — gọi `validateDoiSo`; NẾU KHÔNG `"hop_le"`, trả về
LỖI NGAY (không gọi `thucThiTraCuuDonHang`); NẾU `"hop_le"`, gọi
`thucThiTraCuuDonHang` VỚI `doiSo` đã validate.

```typescript title=starter
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

type KetQuaGoiTool = KetQua<string, string>;

interface DoiSoTraCuuDonHang {
  maDonHang: string;
}

type KetQuaValidate =
  | { loai: "hop_le"; doiSo: DoiSoTraCuuDonHang }
  | { loai: "thieu_khoa"; khoaThieu: string[] }
  | { loai: "sai_kieu"; khoa: string; kieuThuc: string };

const demThucThi = { soLan: 0 };

function thucThiTraCuuDonHang(doiSo: DoiSoTraCuuDonHang): KetQuaGoiTool {
  demThucThi.soLan++;
  return { thanhCong: true, giaTri: `don_hang_${doiSo.maDonHang}` };
}

function validateDoiSo(doiSoTho: unknown): KetQuaValidate {
  ___
}

function goiToolCoValidate(doiSoTho: unknown): KetQuaGoiTool {
  ___
}

const ketQuaHopLe = goiToolCoValidate({ maDonHang: "XY789" });
console.log(JSON.stringify(ketQuaHopLe), demThucThi.soLan);
```

```typescript title=solution
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

type KetQuaGoiTool = KetQua<string, string>;

interface DoiSoTraCuuDonHang {
  maDonHang: string;
}

type KetQuaValidate =
  | { loai: "hop_le"; doiSo: DoiSoTraCuuDonHang }
  | { loai: "thieu_khoa"; khoaThieu: string[] }
  | { loai: "sai_kieu"; khoa: string; kieuThuc: string };

const demThucThi = { soLan: 0 };

function thucThiTraCuuDonHang(doiSo: DoiSoTraCuuDonHang): KetQuaGoiTool {
  demThucThi.soLan++;
  return { thanhCong: true, giaTri: `don_hang_${doiSo.maDonHang}` };
}

function validateDoiSo(doiSoTho: unknown): KetQuaValidate {
  if (typeof doiSoTho !== "object" || doiSoTho === null) {
    return { loai: "thieu_khoa", khoaThieu: ["maDonHang"] };
  }
  const ghi = doiSoTho as Record<string, unknown>;
  if (!("maDonHang" in ghi)) {
    return { loai: "thieu_khoa", khoaThieu: ["maDonHang"] };
  }
  if (typeof ghi.maDonHang !== "string") {
    return { loai: "sai_kieu", khoa: "maDonHang", kieuThuc: typeof ghi.maDonHang };
  }
  return { loai: "hop_le", doiSo: { maDonHang: ghi.maDonHang } };
}

function goiToolCoValidate(doiSoTho: unknown): KetQuaGoiTool {
  const kqValidate = validateDoiSo(doiSoTho);
  if (kqValidate.loai !== "hop_le") {
    return { thanhCong: false, loi: `loi_validate_${kqValidate.loai}` };
  }
  return thucThiTraCuuDonHang(kqValidate.doiSo);
}

const ketQuaHopLe = goiToolCoValidate({ maDonHang: "XY789" });
console.log(JSON.stringify(ketQuaHopLe), demThucThi.soLan);
```

```typescript title=test
if (!ketQuaHopLe.thanhCong || ketQuaHopLe.giaTri !== "don_hang_XY789") {
  throw new Error("doi so hop le phai THUC THI thanh cong, dung gia tri don_hang_XY789");
}
if (demThucThi.soLan !== 1) throw new Error("doi so hop le phai THUC THI dung 1 lan");

const kqThieuKhoa = goiToolCoValidate({ ma: "XY789" });
if (kqThieuKhoa.thanhCong !== false || kqThieuKhoa.loi !== "loi_validate_thieu_khoa") {
  throw new Error("doi so thieu khoa maDonHang phai bi CHAN o buoc validate, loi la loi_validate_thieu_khoa");
}
if (demThucThi.soLan !== 1) throw new Error("doi so THIEU KHOA khong duoc goi thucThiTraCuuDonHang -- validate phai chan TRUOC KHI thuc thi, demThucThi.soLan phai VAN la 1");

const kqSaiKieu = goiToolCoValidate({ maDonHang: 789 });
if (kqSaiKieu.thanhCong !== false || kqSaiKieu.loi !== "loi_validate_sai_kieu") {
  throw new Error("maDonHang la number thay vi string phai bi CHAN, loi la loi_validate_sai_kieu");
}
if (demThucThi.soLan !== 1) throw new Error("doi so SAI KIEU khong duoc goi thucThiTraCuuDonHang -- demThucThi.soLan phai VAN la 1");

const kqNull = validateDoiSo(null);
if (kqNull.loai !== "thieu_khoa") throw new Error("validateDoiSo(null) phai tra ve thieu_khoa (khong duoc nem loi runtime khi ep kieu)");

const kqHopLeTrucTiep = validateDoiSo({ maDonHang: "ABC" });
if (kqHopLeTrucTiep.loai !== "hop_le") throw new Error("validateDoiSo voi doi so dung hinh dang phai tra ve hop_le");
if (kqHopLeTrucTiep.loai === "hop_le" && kqHopLeTrucTiep.doiSo.maDonHang !== "ABC") {
  throw new Error("truong hop hop_le phai giu dung gia tri maDonHang da xac nhan");
}

const kqSaiKieuTrucTiep = validateDoiSo({ maDonHang: 42 });
if (kqSaiKieuTrucTiep.loai !== "sai_kieu") throw new Error("maDonHang la number phai tra ve sai_kieu");
if (kqSaiKieuTrucTiep.loai === "sai_kieu" && kqSaiKieuTrucTiep.kieuThuc !== "number") {
  throw new Error("kieuThuc phai ghi dung kieu THAT SU cua gia tri sai (number)");
}
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (validateDoiSo): ba buoc if lien tiep, moi buoc return NGAY khi phat hien sai -- (1) khong phai object hoac la null -> thieu_khoa; (2) la object nhung khong co khoa maDonHang -> thieu_khoa; (3) co khoa nhung typeof khac 'string' -> sai_kieu; con lai (het ca ba buoc) -> hop_le. Cho hai (goiToolCoValidate): goi validateDoiSo luu vao mot bien; neu loai KHAC 'hop_le' thi return loi ngay (khong goi thucThiTraCuuDonHang); nguoc lai return thucThiTraCuuDonHang voi doiSo da validate."
- kind: strategy
  body: "Cho dau: if (typeof doiSoTho !== \"object\" || doiSoTho === null) { return { loai: \"thieu_khoa\", khoaThieu: [\"maDonHang\"] }; } const ghi = doiSoTho as Record<string, unknown>; if (!(\"maDonHang\" in ghi)) { return { loai: \"thieu_khoa\", khoaThieu: [\"maDonHang\"] }; } if (typeof ghi.maDonHang !== \"string\") { return { loai: \"sai_kieu\", khoa: \"maDonHang\", kieuThuc: typeof ghi.maDonHang }; } return { loai: \"hop_le\", doiSo: { maDonHang: ghi.maDonHang } }; Cho hai: const kqValidate = validateDoiSo(doiSoTho); if (kqValidate.loai !== \"hop_le\") { return { thanhCong: false, loi: `loi_validate_${kqValidate.loai}` }; } return thucThiTraCuuDonHang(kqValidate.doiSo);"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung, GIU NGUYEN thu tu ba buoc kiem trong validateDoiSo."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "{\"thanhCong\":true,\"giaTri\":\"don_hang_XY789\"} 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba lần gọi, một lần chạm tool THẬT — hai lần còn lại bị chặn Ở đúng
CỔNG validate, TRƯỚC KHI bất kỳ tác dụng phụ nào (ghi log, gọi mạng,
đổi dữ liệu) kịp xảy ra. Nhưng bài này CHỈ xử lý được MỘT tool duy
nhất. Một harness thật thường phải chọn GIỮA nhiều tool khác nhau tuỳ
loại tác vụ — bài sau xử lý đúng câu hỏi đó: định tuyến.
::::

::::reflect{#nghi-lai}
Validate đối số TRƯỚC khi thực thi không phải LÀ một bước "cẩn thận
thêm cho chắc" — nó LÀ một RANH GIỚI kiến trúc: MỌI hàm sau ranh giới
đó (`thucThiTraCuuDonHang`) được viết VỚI GIẢ ĐỊNH đối số đã ĐÚNG hình
dạng (`DoiSoTraCuuDonHang`, không phải `unknown`), VÀ không cần tự
kiểm tra lại từ đầu. Discriminated union `KetQuaValidate` LÀ thứ LÀM
cho ranh giới đó tường minh: TypeScript buộc phải thu hẹp
(narrow) VỀ đúng `"hop_le"` trước khi được PHÉP đọc `doiSo` — không có
cách nào "quên kiểm" mà vẫn qua được compiler. Khác VỚI q9.1a bài `5`
(validate OUTPUT của model để hiểu đúng Ý ĐỊNH), bài này validate ĐẦU
VÀO của một hành động SẮP xảy ra — VÀ phần thưởng của việc validate
đúng chỗ LÀ mọi lớp phòng thủ khác (timeout, circuit breaker, retry)
chỉ còn phải lo về tool CÓ CHẠY ĐÚNG hay không, không còn phải lo về
việc đối số gửi vào nó có HỢP LỆ hay không.
::::

::::checkpoint{mastery=0.86}
::::
