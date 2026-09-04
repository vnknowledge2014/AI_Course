---
id: thiet-ke-he-thong.quan-sat-va-canh-bao.blameless-postmortem
title: "Blameless postmortem: không đổ lỗi, action item theo dõi được"
summary: "kiemTraDayDu(bienBan) kiem tra BA phan bat buoc cua BienBanSuCo: timeline (it nhat mot moc), nguyenNhanHeThong (KHONG undefined VA khong rong sau trim -- mo ta CHINH HE THONG thieu gi, khong phai 'ai lam sai'), hanhDongTiepTheo (it nhat mot HanhDongTiepTheo co nguoiPhuTrach VA trangThai) -- postmortem thieu nguyenNhanHeThong VA hanhDongTiepTheo tra ve dayDu=false, thieu=['nguyenNhanHeThong','hanhDongTiepTheo']. demHanhDongTheoTrangThai loc theo trangThai cu the, cho phep theo doi tien do tung action item rieng le."
locale: vi
track: thiet-ke-he-thong
module: quan-sat-va-canh-bao
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [sd.blameless-postmortem]
requires: [sd.on-call-va-runbook]
concepts: [sd.blameless-postmortem]
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
Runbook (bài trước) giúp xử lý sự cố TRONG lúc nó xảy ra. Sau khi hệ thống
đã ổn định trở LẠI, một câu hỏi khác xuất hiện: hệ thống đã học được gì để
LẦN sau không lặp LẠI chuyện tương tự? Câu trả lời KHÔNG phải LÀ "ai đã làm
sai" — đó LÀ câu hỏi sai NGAY từ đầu.
::::

::::explain{#ba-phan-bat-buoc}
`kiemTraDayDu` kiểm tra một `BienBanSuCo` có đủ BA phần bắt buộc: `timeline`
(ít nhất một mốc thời gian), `nguyenNhanHeThong` (root cause — PHẢI mô tả
CHÍNH hệ thống thiếu gì, không phải tên một người), VÀ `hanhDongTiepTheo`
(ít nhất một action item, mỗi cái có người phụ trách RIÊNG VÀ trạng thái):

```typescript title=readonly
interface MocThoiGian { thoiDiem: number; moTa: string; }
interface HanhDongTiepTheo { moTa: string; nguoiPhuTrach: string; trangThai: string; }
interface BienBanSuCo {
  tenSuCo: string;
  timeline: MocThoiGian[];
  nguyenNhanHeThong: string | undefined;
  hanhDongTiepTheo: HanhDongTiepTheo[];
}

interface KetQuaKiemTra { dayDu: boolean; thieu: string[]; }
function kiemTraDayDu(bienBan: BienBanSuCo): KetQuaKiemTra {
  const thieu: string[] = [];
  if (bienBan.timeline.length === 0) thieu.push("timeline");
  if (bienBan.nguyenNhanHeThong === undefined || bienBan.nguyenNhanHeThong.trim() === "") thieu.push("nguyenNhanHeThong");
  if (bienBan.hanhDongTiepTheo.length === 0) thieu.push("hanhDongTiepTheo");
  return { dayDu: thieu.length === 0, thieu };
}

// nguyenNhanHeThong mo ta HE THONG thieu gi -- khong phai "ky su X quen lam gi do"
const bienBanDayDu: BienBanSuCo = {
  tenSuCo: "do-tre-cao-15-thang-3",
  timeline: [
    { thoiDiem: 0, moTa: "Alert theo trieu chung kich hoat (p99 vuot nguong)" },
    { thoiDiem: 5, moTa: "Nguoi truc xac nhan qua trace: dich-vu-thanh-toan cham" },
    { thoiDiem: 20, moTa: "Rollback ban deploy gan nhat, do tre tro ve binh thuong" },
  ],
  nguyenNhanHeThong:
    "He thong khong co canh bao truoc khi deploy neu do tre p99 tang qua 20% so voi baseline, nen ban deploy loi duoc dua vao production ma khong bi chan lai",
  hanhDongTiepTheo: [
    { moTa: "Them buoc kiem tra do tre p99 vao pipeline truoc khi tang canary", nguoiPhuTrach: "An", trangThai: "dang-lam" },
    { moTa: "Viet alert rieng cho do lech p99 so voi baseline", nguoiPhuTrach: "Binh", trangThai: "chua-bat-dau" },
  ],
};
console.log("postmortem DAY DU:", JSON.stringify(kiemTraDayDu(bienBanDayDu)));

const bienBanThieu: BienBanSuCo = {
  tenSuCo: "loi-thanh-toan-2-thang-4",
  timeline: [{ thoiDiem: 0, moTa: "Nguoi dung bao loi thanh toan" }],
  nguyenNhanHeThong: undefined,
  hanhDongTiepTheo: [],
};
console.log("postmortem THIEU:", JSON.stringify(kiemTraDayDu(bienBanThieu)));
```

```text title=readonly
postmortem DAY DU: {"dayDu":true,"thieu":[]}
postmortem THIEU: {"dayDu":false,"thieu":["nguyenNhanHeThong","hanhDongTiepTheo"]}
```

`bienBanDayDu.nguyenNhanHeThong` mô tả một KHOẢNG TRỐNG của hệ thống —
"không có cảnh báo trước khi deploy" — chứ KHÔNG nói "kỹ sư nào đã bấm nút
deploy". `bienBanThieu` thiếu ĐÚNG hai phần: không có root cause VÀ không
có action item nào — `kiemTraDayDu` liệt kê CHÍNH XÁC những gì còn thiếu,
không chỉ nói chung chung "chưa xong".
::::

::::example{#theo-doi-hanh-dong}
Mỗi `HanhDongTiepTheo` mang một `nguoiPhuTrach` RIÊNG VÀ một `trangThai` —
`demHanhDongTheoTrangThai` lọc theo ĐÚNG trạng thái, cho phép theo dõi tiến
độ từng phần một, thay vì chỉ biết "postmortem có action item hay không":

```typescript title=readonly
interface HanhDongTiepTheo { moTa: string; nguoiPhuTrach: string; trangThai: string; }
function demHanhDongTheoTrangThai(cacHanhDong: HanhDongTiepTheo[], trangThai: string): number {
  return cacHanhDong.filter((h) => h.trangThai === trangThai).length;
}

const cacHanhDong: HanhDongTiepTheo[] = [
  { moTa: "Them buoc kiem tra do tre p99 vao pipeline truoc khi tang canary", nguoiPhuTrach: "An", trangThai: "dang-lam" },
  { moTa: "Viet alert rieng cho do lech p99 so voi baseline", nguoiPhuTrach: "Binh", trangThai: "chua-bat-dau" },
];
console.log("so hanh dong dang-lam:", demHanhDongTheoTrangThai(cacHanhDong, "dang-lam"));
console.log("so hanh dong chua-bat-dau:", demHanhDongTheoTrangThai(cacHanhDong, "chua-bat-dau"));
console.log("so hanh dong hoan-thanh:", demHanhDongTheoTrangThai(cacHanhDong, "hoan-thanh"));
```

```text title=readonly
so hanh dong dang-lam: 1
so hanh dong chua-bat-dau: 1
so hanh dong hoan-thanh: 0
```

`demHanhDongTheoTrangThai(cacHanhDong, "hoan-thanh")` trả về `0` — KHÔNG
phải LÀ lỗi, mà LÀ sự thật: chưa hành động nào được đánh dấu xong. Đây
CHÍNH LÀ giá trị của việc gắn `trangThai` VÀO từng action item riêng —
biết ĐƯỢC bao nhiêu việc còn tồn ĐỌNG, không chỉ biết "có kế hoạch" chung
chung.
::::

::::predict{#doan-chuoi-rong commitOnce}
Một `BienBanSuCo` có `nguyenNhanHeThong` LÀ chuỗi RỖNG `""` (đã được GÁN
giá trị, không phải `undefined`). Gọi `kiemTraDayDu` — `"nguyenNhanHeThong"`
có xuất hiện trong `thieu` không?

:::opt{correct}
Có — điều kiện LÀ `bienBan.nguyenNhanHeThong === undefined ||
bienBan.nguyenNhanHeThong.trim() === ""`; chuỗi rỗng KHÔNG phải
`undefined`, nhưng vế THỨ HAI (`trim() === ""`) vẫn LÀ `true`, nên toàn bộ
điều kiện LÀ `true` VÀ `"nguyenNhanHeThong"` vẫn bị thêm VÀO `thieu`
:::
:::opt
Không — chuỗi rỗng LÀ một giá trị `string` hợp lệ VỀ mặt kiểu dữ liệu
(khác `undefined`), nên nó ĐƯỢC coi LÀ "đã điền", dù nội dung không có gì
::why
Nhầm "đúng kiểu dữ liệu" VỚI "có nội dung Ý nghĩa" — nhưng `kiemTraDayDu`
kiểm tra CẢ HAI khả năng bằng một phép `||`, không chỉ kiểm tra kiểu.

Chỗ lệch: điều kiện có HAI vế nối bằng `||`. Vế đầu (`=== undefined`) LÀ
`false` với chuỗi rỗng. NHƯNG vế sau — `.trim() === ""` — LÀ `true`, VÌ
chuỗi rỗng sau khi `trim()` VẪN LÀ rỗng. `false || true` cho `true`, nên
nhánh `thieu.push("nguyenNhanHeThong")` VẪN chạy.
::
:::
::::

::::code{#viet_kiem_tra_day_du}
Hoàn thiện `kiemTraDayDu` — kiểm tra ĐỦ ba phần: `timeline` không rỗng,
`nguyenNhanHeThong` có nội dung THẬT sự (không `undefined`, không chỉ toàn
khoảng trắng), VÀ `hanhDongTiepTheo` không rỗng. Với MỖI phần thiếu, thêm
đúng TÊN phần đó VÀO mảng `thieu`.

```typescript title=starter
interface MocThoiGian { thoiDiem: number; moTa: string; }
interface HanhDongTiepTheo { moTa: string; nguoiPhuTrach: string; trangThai: string; }
interface BienBanSuCo {
  tenSuCo: string;
  timeline: MocThoiGian[];
  nguyenNhanHeThong: string | undefined;
  hanhDongTiepTheo: HanhDongTiepTheo[];
}
interface KetQuaKiemTra { dayDu: boolean; thieu: string[]; }
function kiemTraDayDu(bienBan: BienBanSuCo): KetQuaKiemTra {
  ___
}

const bienBanX: BienBanSuCo = {
  tenSuCo: "x",
  timeline: [{ thoiDiem: 0, moTa: "bat dau" }],
  nguyenNhanHeThong: "he thong thieu kiem tra Y",
  hanhDongTiepTheo: [],
};
const ketQuaX = kiemTraDayDu(bienBanX);
console.log(ketQuaX.dayDu, JSON.stringify(ketQuaX.thieu));
```

```typescript title=solution
interface MocThoiGian { thoiDiem: number; moTa: string; }
interface HanhDongTiepTheo { moTa: string; nguoiPhuTrach: string; trangThai: string; }
interface BienBanSuCo {
  tenSuCo: string;
  timeline: MocThoiGian[];
  nguyenNhanHeThong: string | undefined;
  hanhDongTiepTheo: HanhDongTiepTheo[];
}
interface KetQuaKiemTra { dayDu: boolean; thieu: string[]; }
function kiemTraDayDu(bienBan: BienBanSuCo): KetQuaKiemTra {
  const thieu: string[] = [];
  if (bienBan.timeline.length === 0) thieu.push("timeline");
  if (bienBan.nguyenNhanHeThong === undefined || bienBan.nguyenNhanHeThong.trim() === "") thieu.push("nguyenNhanHeThong");
  if (bienBan.hanhDongTiepTheo.length === 0) thieu.push("hanhDongTiepTheo");
  return { dayDu: thieu.length === 0, thieu };
}

const bienBanX: BienBanSuCo = {
  tenSuCo: "x",
  timeline: [{ thoiDiem: 0, moTa: "bat dau" }],
  nguyenNhanHeThong: "he thong thieu kiem tra Y",
  hanhDongTiepTheo: [],
};
const ketQuaX = kiemTraDayDu(bienBanX);
console.log(ketQuaX.dayDu, JSON.stringify(ketQuaX.thieu));
```

```typescript title=test
const dayDuT: BienBanSuCo = {
  tenSuCo: "t1",
  timeline: [{ thoiDiem: 0, moTa: "bat dau" }],
  nguyenNhanHeThong: "he thong thieu X",
  hanhDongTiepTheo: [{ moTa: "sua X", nguoiPhuTrach: "An", trangThai: "dang-lam" }],
};
const ketQuaDayDuT = kiemTraDayDu(dayDuT);
if (ketQuaDayDuT.dayDu !== true) throw new Error("du ca ba phan thi dayDu phai la true");
if (ketQuaDayDuT.thieu.length !== 0) throw new Error("du ca ba phan thi thieu phai la mang rong");

const rongHetT: BienBanSuCo = { tenSuCo: "t2", timeline: [], nguyenNhanHeThong: undefined, hanhDongTiepTheo: [] };
const ketQuaRongHetT = kiemTraDayDu(rongHetT);
if (ketQuaRongHetT.dayDu !== false) throw new Error("thieu ca ba phan thi dayDu phai la false");
if (ketQuaRongHetT.thieu.length !== 3) throw new Error("thieu ca ba phan thi thieu phai co dung 3 phan tu");

const chuoiTrangT: BienBanSuCo = {
  tenSuCo: "t3",
  timeline: [{ thoiDiem: 0, moTa: "bat dau" }],
  nguyenNhanHeThong: "   ",
  hanhDongTiepTheo: [{ moTa: "sua", nguoiPhuTrach: "An", trangThai: "dang-lam" }],
};
const ketQuaChuoiTrangT = kiemTraDayDu(chuoiTrangT);
if (ketQuaChuoiTrangT.dayDu !== false) throw new Error("nguyenNhanHeThong chi toan khoang trang phai bi tinh la THIEU");
if (!ketQuaChuoiTrangT.thieu.includes("nguyenNhanHeThong")) throw new Error("thieu phai chua 'nguyenNhanHeThong' khi chuoi chi toan khoang trang");

const thieuHanhDongT: BienBanSuCo = {
  tenSuCo: "t4",
  timeline: [{ thoiDiem: 0, moTa: "bat dau" }],
  nguyenNhanHeThong: "he thong thieu X",
  hanhDongTiepTheo: [],
};
const ketQuaThieuHanhDongT = kiemTraDayDu(thieuHanhDongT);
if (ketQuaThieuHanhDongT.thieu.length !== 1) throw new Error("chi thieu hanhDongTiepTheo thi thieu phai co dung 1 phan tu");
if (ketQuaThieuHanhDongT.thieu[0] !== "hanhDongTiepTheo") throw new Error("phan tu do phai la 'hanhDongTiepTheo'");
```

:::hints
- kind: attention
  body: "Tao mang thieu rong. Neu bienBan.timeline.length === 0 thi push 'timeline'. Neu bienBan.nguyenNhanHeThong === undefined HOAC .trim() === '' thi push 'nguyenNhanHeThong'. Neu bienBan.hanhDongTiepTheo.length === 0 thi push 'hanhDongTiepTheo'. Tra ve { dayDu: thieu.length === 0, thieu }."
- kind: strategy
  body: "const thieu: string[] = []; if (bienBan.timeline.length === 0) thieu.push('timeline'); if (bienBan.nguyenNhanHeThong === undefined || bienBan.nguyenNhanHeThong.trim() === '') thieu.push('nguyenNhanHeThong'); if (bienBan.hanhDongTiepTheo.length === 0) thieu.push('hanhDongTiepTheo'); return { dayDu: thieu.length === 0, thieu };"
- kind: one-line
  body: "const thieu: string[] = []; if (bienBan.timeline.length === 0) thieu.push('timeline'); if (bienBan.nguyenNhanHeThong === undefined || bienBan.nguyenNhanHeThong.trim() === '') thieu.push('nguyenNhanHeThong'); if (bienBan.hanhDongTiepTheo.length === 0) thieu.push('hanhDongTiepTheo'); return { dayDu: thieu.length === 0, thieu };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "false [\"hanhDongTiepTheo\"]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đầy đủ ba phần, kiểm tra được bằng CODE thay vì bằng cảm giác "có vẻ ổn
rồi". Tám bài đã ráp đủ mảnh: ba trụ cột quan sát, log có cấu trúc, đúng
loại metric, trace xuyên dịch vụ, SLI/SLO/error budget, alert đúng triệu
chứng, on-call VÀ runbook, VÀ giờ LÀ postmortem không đổ lỗi. Bài CUỐI ráp
TẤT cả thành một luồng xử lý sự cố đầu-cuối.
::::

::::reflect{#nghi-lai}
`kiemTraDayDu` không hề đọc được nội DUNG có "đổ lỗi" hay không — nó chỉ
kiểm tra CẤU trúc có đủ hay thiếu. Tính "blameless" không nằm Ở một điều
kiện `if` nào cả — nó nằm Ở CÁCH người viết diễn đạt `nguyenNhanHeThong`:
mô tả một khoảng trống của HỆ THỐNG (như `bienBanDayDu` đã làm) thay vì chỉ
tên một người. Code chỉ đảm bảo được phần CÓ THỂ đo: đủ timeline, đủ root
cause, đủ action item theo dõi được — phần còn lại LÀ kỷ luật của người
viết, không phải LÀ thứ một hàm `boolean` có thể ép buộc.
::::

::::checkpoint{mastery=0.84}
::::
