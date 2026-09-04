---
id: thiet-ke-he-thong.quan-sat-va-canh-bao.on-call-va-runbook
title: "On-call & runbook: ai được gọi, quy trình xử lý chuẩn hoá"
summary: "aiDangTruc(cacCaTruc, dh) tim CaTruc co dh.thoiGianHienTai NAM TRONG [gioBatDau, gioKetThuc) -- lich xoay vong 3 nguoi, moi ca 24 gio: gio 10 la An, gio 24 (dung ranh gioi, dung >=) chuyen sang Binh, gio 47 van la Binh, gio 48 sang Chi, gio 72 (het lich, chua xoay tiep) tra ve undefined. layRunbook(cacRunbook, tenSuCo) tim CHINH XAC runbook theo ten su co -- 'do-tre-cao' tra ve du 4 buoc xu ly chuan, su co CHUA co san ('mat-dien') tra ve undefined. Nguoi dang truc (du la ai) deu doc DUNG cung mot runbook, khong can doan."
locale: vi
track: thiet-ke-he-thong
module: quan-sat-va-canh-bao
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.on-call-va-runbook]
requires: [sd.alert-theo-trieu-chung-khong-theo-nguyen-nhan]
concepts: [sd.on-call-va-runbook]
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
Alert theo triệu chứng (bài trước) chỉ giải quyết CÂU hỏi "khi nào báo
động". Báo động ĐÓ phải tới được MỘT người cụ thể — không phải "cả đội",
vì "cả đội" nghĩa LÀ không ai chắc chắn mình PHẢI phản ứng. VÀ người ĐÓ cần
biết làm GÌ tiếp theo, không phải tự đoán giữa lúc đang có sự cố.
::::

::::explain{#lich-truc-xoay-vong}
`aiDangTruc` tìm ca trực mà thời điểm hiện tại của đồng hồ RƠI VÀO — dùng
`gioBatDau` (bao gồm, `>=`) VÀ `gioKetThuc` (không bao gồm, `<`), đúng kiểu
nửa-khoảng đã quen thuộc từ những bài trước. Lịch xoay vòng `3` người, mỗi
ca `24` giờ:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soGio: number): void { dh.thoiGianHienTai += soGio; }

interface CaTruc { tenNguoiTruc: string; gioBatDau: number; gioKetThuc: number; }
function aiDangTruc(cacCaTruc: CaTruc[], dh: DongHoMoPhong): string | undefined {
  const ca = cacCaTruc.find((c) => dh.thoiGianHienTai >= c.gioBatDau && dh.thoiGianHienTai < c.gioKetThuc);
  return ca?.tenNguoiTruc;
}

const cacCaTruc: CaTruc[] = [
  { tenNguoiTruc: "An", gioBatDau: 0, gioKetThuc: 24 },
  { tenNguoiTruc: "Binh", gioBatDau: 24, gioKetThuc: 48 },
  { tenNguoiTruc: "Chi", gioBatDau: 48, gioKetThuc: 72 },
];

const dh = taoDongHoMoPhong();
tienThoiGian(dh, 10);
console.log("gio 10, dang truc:", aiDangTruc(cacCaTruc, dh));
tienThoiGian(dh, 14);
console.log("gio 24 (dung ranh gioi ca moi), dang truc:", aiDangTruc(cacCaTruc, dh));
tienThoiGian(dh, 23);
console.log("gio 47 (gan het ca Binh), dang truc:", aiDangTruc(cacCaTruc, dh));
tienThoiGian(dh, 1);
console.log("gio 48 (sang ca Chi), dang truc:", aiDangTruc(cacCaTruc, dh));
tienThoiGian(dh, 24);
console.log("gio 72 (het lich, chua xoay vong tiep), dang truc:", aiDangTruc(cacCaTruc, dh));
```

```text title=readonly
gio 10, dang truc: An
gio 24 (dung ranh gioi ca moi), dang truc: Binh
gio 47 (gan het ca Binh), dang truc: Binh
gio 48 (sang ca Chi), dang truc: Chi
gio 72 (het lich, chua xoay vong tiep), dang truc: undefined
```

Tại `gio=24`, đúng thời điểm CHUYỂN ca — `aiDangTruc` trả về `"Binh"` NGAY,
không còn LÀ `"An"`, vì `gioBatDau` dùng `>=`. Tại `gio=72`, KHÔNG có ca
nào bao PHỦ (lịch chỉ định nghĩa tới `72`, chưa xoay VÒNG lại) — hàm trả về
`undefined`, một tín hiệu THẲNG THẮN LÀ "lịch trực CHƯA được cấu hình xa hơn",
không phải LÀ lỗi ngầm.
::::

::::example{#runbook-doc-lap-nguoi-truc}
`layRunbook` tìm CHÍNH XÁC bộ bước xử lý chuẩn cho MỘT loại sự cố — VÀ bộ
bước ĐÓ không hề phụ thuộc AI đang trực. Người MỚI nhận ca lần đầu VÀ người
đã trực hàng chục lần đều đọc ĐÚNG cùng bốn bước, theo ĐÚNG cùng thứ tự:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
interface CaTruc { tenNguoiTruc: string; gioBatDau: number; gioKetThuc: number; }
function aiDangTruc(cacCaTruc: CaTruc[], dh: DongHoMoPhong): string | undefined {
  const ca = cacCaTruc.find((c) => dh.thoiGianHienTai >= c.gioBatDau && dh.thoiGianHienTai < c.gioKetThuc);
  return ca?.tenNguoiTruc;
}

interface Runbook { tenSuCo: string; cacBuoc: string[]; }
function layRunbook(cacRunbook: Runbook[], tenSuCo: string): Runbook | undefined {
  return cacRunbook.find((r) => r.tenSuCo === tenSuCo);
}

const cacCaTruc: CaTruc[] = [
  { tenNguoiTruc: "An", gioBatDau: 0, gioKetThuc: 24 },
  { tenNguoiTruc: "Binh", gioBatDau: 24, gioKetThuc: 48 },
  { tenNguoiTruc: "Chi", gioBatDau: 48, gioKetThuc: 72 },
];
const cacRunbook: Runbook[] = [
  {
    tenSuCo: "do-tre-cao",
    cacBuoc: [
      "Kiem tra dashboard p99 theo tung dich vu",
      "Tim dich vu chiem nhieu thoi gian nhat bang trace",
      "Kiem tra log loi cua dich vu do",
      "Neu can, rollback ban deploy gan nhat",
    ],
  },
  {
    tenSuCo: "ty-le-loi-cao",
    cacBuoc: [
      "Kiem tra alert theo trieu chung dang kich hoat",
      "Loc log theo mucDo=loi va dichVu nghi ngo",
      "Xac dinh co phai do mot deploy gan day khong",
      "Neu co, rollback ngay",
    ],
  },
];

const nguoiDangTruc = aiDangTruc(cacCaTruc, { thoiGianHienTai: 10 });
console.log("nguoi dang truc luc gio 10 (co the la nguoi MOI, chua tung gap su co nay):", nguoiDangTruc);
const runbookDoTre = layRunbook(cacRunbook, "do-tre-cao");
console.log("runbook cho 'do-tre-cao', ap dung DU cho ai dang truc:", JSON.stringify(runbookDoTre));
console.log("so buoc can lam:", runbookDoTre?.cacBuoc.length);
const runbookKhongTonTai = layRunbook(cacRunbook, "mat-dien");
console.log("runbook cho su co CHUA co san ('mat-dien'):", runbookKhongTonTai);
```

```text title=readonly
nguoi dang truc luc gio 10 (co the la nguoi MOI, chua tung gap su co nay): An
runbook cho 'do-tre-cao', ap dung DU cho ai dang truc: {"tenSuCo":"do-tre-cao","cacBuoc":["Kiem tra dashboard p99 theo tung dich vu","Tim dich vu chiem nhieu thoi gian nhat bang trace","Kiem tra log loi cua dich vu do","Neu can, rollback ban deploy gan nhat"]}
so buoc can lam: 4
runbook cho su co CHUA co san ('mat-dien'): undefined
```

`layRunbook` không hề biết `"An"` LÀ ai, có kinh nghiệm gì, hay đây LÀ lần
đầu trực — nó chỉ tìm ĐÚNG bản ghi khớp `tenSuCo`, VÀ trả về ĐỦ cả `4`
bước. `"mat-dien"` chưa CÓ runbook nào khớp — kết quả `undefined` LÀ tín
hiệu THẲNG thắn: đội ngũ CẦN viết thêm một runbook mới, không phải LÀ lỗi
im lặng khiến người trực phải tự bịa CÁCH xử lý.
::::

::::predict{#doan-bien-ca-dau commitOnce}
Đồng hồ ĐANG Ở đúng `thoiGianHienTai=0` — thời điểm BẮT ĐẦU của ca đầu
tiên (`"An"`, `gioBatDau=0`). `aiDangTruc` tại thời điểm NÀY trả về gì?

:::opt{correct}
`"An"` — điều kiện dùng `>=` cho mốc bắt đầu; `0 >= 0` LÀ `true`, VÀ `0 <
24` cũng LÀ `true`, nên ca của `"An"` ĐÃ tính LÀ đang diễn ra NGAY tại thời
điểm bắt đầu, không cần chờ thêm
:::
:::opt
`undefined` — tại ĐÚNG thời điểm chuyển giao, chưa CÓ đủ căn cứ để xác
định ai đang trực, nên hệ thống nên coi LÀ "chưa xác định" để an toàn
::why
Nhầm "thời điểm bắt đầu LÀ mơ hồ, cần thêm dữ liệu" VỚI cách điều kiện
THẬT sự viết trong `aiDangTruc` — hàm dùng nửa-khoảng `[gioBatDau,
gioKetThuc)`, mốc bắt đầu LUÔN được TÍNH LÀ đã Ở trong ca.

Chỗ lệch: điều kiện LÀ `dh.thoiGianHienTai >= c.gioBatDau && dh.thoiGianHienTai
< c.gioKetThuc`. Với `thoiGianHienTai=0` VÀ ca của `"An"` có `gioBatDau=0`,
vế đầu LÀ `0 >= 0`, cho `true` NGAY LẬP TỨC — không có khoảng "chưa xác
định" nào Ở giữa hai ca.
::
:::
::::

::::code{#viet_ai_dang_truc}
Hoàn thiện `aiDangTruc` — tìm ca trực mà `dh.thoiGianHienTai` nằm trong
nửa-khoảng `[gioBatDau, gioKetThuc)`, RỒI trả về `tenNguoiTruc` của ca đó
(hoặc `undefined` nếu không ca nào khớp).

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
interface CaTruc { tenNguoiTruc: string; gioBatDau: number; gioKetThuc: number; }
function aiDangTruc(cacCaTruc: CaTruc[], dh: DongHoMoPhong): string | undefined {
  ___
}

const cacCaTrucX: CaTruc[] = [
  { tenNguoiTruc: "Dung", gioBatDau: 0, gioKetThuc: 12 },
  { tenNguoiTruc: "Em", gioBatDau: 12, gioKetThuc: 24 },
];
console.log(aiDangTruc(cacCaTrucX, { thoiGianHienTai: 15 }));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
interface CaTruc { tenNguoiTruc: string; gioBatDau: number; gioKetThuc: number; }
function aiDangTruc(cacCaTruc: CaTruc[], dh: DongHoMoPhong): string | undefined {
  const ca = cacCaTruc.find((c) => dh.thoiGianHienTai >= c.gioBatDau && dh.thoiGianHienTai < c.gioKetThuc);
  return ca?.tenNguoiTruc;
}

const cacCaTrucX: CaTruc[] = [
  { tenNguoiTruc: "Dung", gioBatDau: 0, gioKetThuc: 12 },
  { tenNguoiTruc: "Em", gioBatDau: 12, gioKetThuc: 24 },
];
console.log(aiDangTruc(cacCaTrucX, { thoiGianHienTai: 15 }));
```

```typescript title=test
const cacCaTrucT: CaTruc[] = [
  { tenNguoiTruc: "Gia", gioBatDau: 0, gioKetThuc: 24 },
  { tenNguoiTruc: "Hoa", gioBatDau: 24, gioKetThuc: 48 },
];

const dauCaT = aiDangTruc(cacCaTrucT, { thoiGianHienTai: 0 });
if (dauCaT !== "Gia") throw new Error("dung mocbat dau (0), phai la nguoi cua ca dau tien (>=), khong phai undefined");

const giuaCaT = aiDangTruc(cacCaTrucT, { thoiGianHienTai: 23 });
if (giuaCaT !== "Gia") throw new Error("gio 23 van con trong ca dau tien (0-24)");

const chuyenCaT = aiDangTruc(cacCaTrucT, { thoiGianHienTai: 24 });
if (chuyenCaT !== "Hoa") throw new Error("dung ranh gioi 24, phai CHUYEN sang ca thu hai (>=), khong con la Gia");

const ngoaiLichT = aiDangTruc(cacCaTrucT, { thoiGianHienTai: 100 });
if (ngoaiLichT !== undefined) throw new Error("thoi diem ngoai moi ca da dinh nghia phai tra ve undefined");

const rongT = aiDangTruc([], { thoiGianHienTai: 5 });
if (rongT !== undefined) throw new Error("danh sach ca truc rong phai tra ve undefined");
```

:::hints
- kind: attention
  body: "Dung .find tren cacCaTruc, tim c co dh.thoiGianHienTai >= c.gioBatDau VA dh.thoiGianHienTai < c.gioKetThuc. Tra ve ca?.tenNguoiTruc (optional chaining, tu dong ra undefined neu khong tim thay)."
- kind: strategy
  body: "const ca = cacCaTruc.find((c) => dh.thoiGianHienTai >= c.gioBatDau && dh.thoiGianHienTai < c.gioKetThuc); return ca?.tenNguoiTruc;"
- kind: one-line
  body: "const ca = cacCaTruc.find((c) => dh.thoiGianHienTai >= c.gioBatDau && dh.thoiGianHienTai < c.gioKetThuc); return ca?.tenNguoiTruc;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "Em"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đúng người, đúng lúc, đúng quy trình — VÀ không ai phải tự bịa CÁCH xử lý
giữa lúc đang có sự cố. Nhưng runbook chỉ giải quyết TRONG lúc sự cố đang
diễn ra. Sau khi đã xử lý xong, câu hỏi CÒN lại: hệ thống ĐÃ học được gì,
VÀ ai chịu trách nhiệm đảm bảo bài học ĐÓ được áp dụng?
::::

::::reflect{#nghi-lai}
`aiDangTruc` VÀ `layRunbook` giải quyết hai vấn đề TÁCH biệt nhưng bổ sung
cho nhau: một hàm trả lời "AI", hàm kia trả lời "LÀM gì" — VÀ cả hai đều
không phụ thuộc vào TRÍ nhớ hay kinh nghiệm cá nhân của bất kỳ ai. Lịch
trực loại bỏ sự mơ hồ VỀ trách nhiệm ("ai đó sẽ xử lý" trở thành "chính xác
người này đang trực"); runbook loại bỏ sự mơ hồ VỀ hành động ("thử vài
thứ xem sao" trở thành "bốn bước theo đúng thứ tự"). Cả hai đều LÀ dữ liệu
có cấu trúc, tra cứu được — không phải LÀ kiến thức truyền miệng.
::::

::::checkpoint{mastery=0.81}
::::
