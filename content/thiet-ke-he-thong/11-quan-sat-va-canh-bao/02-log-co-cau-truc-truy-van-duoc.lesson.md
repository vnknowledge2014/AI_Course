---
id: thiet-ke-he-thong.quan-sat-va-canh-bao.log-co-cau-truc-truy-van-duoc
title: "Structured logging: log có cấu trúc để QUERY được, không phải text tự do"
summary: "timTheoChuoiTuDo(cacDong, cacTuKhoa) doan tim BANG substring tren text tu do -- vua DUONG DUONG (khop nham dong 'don-hang bi tre do thanh-toan loi' khi tim 'ERROR'+'thanh-toan', ra 2 ket qua thay vi 1) vua THIEU SOT (dinh dang 'service:x' khac 'dich-vu=x' bi bo lot). locLogTheoMucDoVaDichVu(cacLog, mucDo, dichVu) loc CHINH XAC tren field co dinh { thoiDiem, mucDo, dichVu, thongDiep, metadata } -- ca hai dieu kien PHAI dung (AND), khong quan tam thongDiep viet the nao, luon ra DUNG so luong."
locale: vi
track: thiet-ke-he-thong
module: quan-sat-va-canh-bao
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.log-co-cau-truc-truy-van-duoc]
requires: [sd.ba-tru-cot-quan-sat]
concepts: [sd.log-co-cau-truc-truy-van-duoc]
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
Bài trước dùng `timLogTheoRequestId` để lọc log — nhưng lọc theo MỘT trường
`requestId` chính xác chỉ hoạt động NẾU log thật sự CÓ trường đó, chứ không
phải LÀ một dòng chữ tự do. Phần lớn log THẬT sự viết ra đời lại LÀ chuỗi
text ghép SẴN — VÀ text tự do không thể lọc CHÍNH XÁC được.
::::

::::explain{#text-tu-do-vs-co-cau-truc}
`timTheoChuoiTuDo` mô phỏng cách tìm log PHỔ biến nhất: tìm những dòng có
chứa TẤT cả từ khoá, bằng `includes`. Nhưng `includes` không phân biệt được
"từ khoá xuất hiện Ở ĐÚNG trường" với "từ khoá tình cờ xuất hiện Ở đâu đó
trong dòng":

```typescript title=readonly
const cacDongTuDo: string[] = [
  "10:00:00 INFO dich-vu=thanh-toan msg=Da xu ly xong",
  "10:00:01 ERROR dich-vu=thanh-toan msg=Ket noi bi loi",
  "10:00:02 ERROR dich-vu=don-hang msg=thanh-toan bi tre do don hang loi",
];
function timTheoChuoiTuDo(cacDong: string[], cacTuKhoa: string[]): string[] {
  return cacDong.filter((d) => cacTuKhoa.every((tk) => d.includes(tk)));
}

interface BanGhiLogCoCauTruc {
  thoiDiem: number;
  mucDo: string;
  dichVu: string;
  thongDiep: string;
  metadata: Record<string, string>;
}
function locLogTheoMucDoVaDichVu(cacLog: BanGhiLogCoCauTruc[], mucDo: string, dichVu: string): BanGhiLogCoCauTruc[] {
  return cacLog.filter((l) => l.mucDo === mucDo && l.dichVu === dichVu);
}

const cacLogCoCauTruc: BanGhiLogCoCauTruc[] = [
  { thoiDiem: 0, mucDo: "thong-tin", dichVu: "thanh-toan", thongDiep: "Da xu ly xong", metadata: {} },
  { thoiDiem: 1, mucDo: "loi", dichVu: "thanh-toan", thongDiep: "Ket noi bi loi", metadata: { maLoi: "E502" } },
  { thoiDiem: 2, mucDo: "loi", dichVu: "don-hang", thongDiep: "thanh-toan bi tre do don hang loi", metadata: { maLoi: "E100" } },
];

console.log("tim dong chua ca 'ERROR' va 'thanh-toan' (doan bang substring):", JSON.stringify(timTheoChuoiTuDo(cacDongTuDo, ["ERROR", "thanh-toan"])));
console.log("so dong tim duoc (SAI, dong 3 la loi cua don-hang khong phai thanh-toan):", timTheoChuoiTuDo(cacDongTuDo, ["ERROR", "thanh-toan"]).length);
console.log("loc CHINH XAC mucDo=loi, dichVu=thanh-toan:", JSON.stringify(locLogTheoMucDoVaDichVu(cacLogCoCauTruc, "loi", "thanh-toan")));
console.log("so log DUNG tim duoc:", locLogTheoMucDoVaDichVu(cacLogCoCauTruc, "loi", "thanh-toan").length);
```

```text title=readonly
tim dong chua ca 'ERROR' va 'thanh-toan' (doan bang substring): ["10:00:01 ERROR dich-vu=thanh-toan msg=Ket noi bi loi","10:00:02 ERROR dich-vu=don-hang msg=thanh-toan bi tre do don hang loi"]
so dong tim duoc (SAI, dong 3 la loi cua don-hang khong phai thanh-toan): 2
loc CHINH XAC mucDo=loi, dichVu=thanh-toan: [{"thoiDiem":1,"mucDo":"loi","dichVu":"thanh-toan","thongDiep":"Ket noi bi loi","metadata":{"maLoi":"E502"}}]
so log DUNG tim duoc: 1
```

`timTheoChuoiTuDo` trả về `2` dòng — nhưng dòng THỨ BA thật ra LÀ lỗi của
`don-hang`, chỉ TÌNH CỜ nhắc tới chữ `"thanh-toan"` trong phần mô tả. Substring
không phân biệt được "đây LÀ dịch vụ gây lỗi" với "chữ này CHỈ xuất hiện
trong câu". `locLogTheoMucDoVaDichVu` không hề mắc lỗi đó — nó so field
`dichVu` với ĐÚNG chuỗi `"thanh-toan"`, VÀ chỉ tìm thấy `1` kết quả đúng.
::::

::::example{#dinh-dang-khong-dong-nhat}
Vấn đề còn LỚN hơn thế: hai đội khác nhau có thể ghi log CÙNG một loại sự
kiện theo hai ĐỊNH DẠNG khác nhau — `timTheoChuoiTuDo` tìm theo một khuôn
mẫu cố định sẽ BỎ SÓT những dòng viết khác khuôn mẫu đó, dù chúng mang Ý
nghĩa GIỐNG hệt nhau:

```typescript title=readonly
function timTheoChuoiTuDo(cacDong: string[], cacTuKhoa: string[]): string[] {
  return cacDong.filter((d) => cacTuKhoa.every((tk) => d.includes(tk)));
}

interface BanGhiLogCoCauTruc {
  thoiDiem: number;
  mucDo: string;
  dichVu: string;
  thongDiep: string;
  metadata: Record<string, string>;
}
function locLogTheoMucDoVaDichVu(cacLog: BanGhiLogCoCauTruc[], mucDo: string, dichVu: string): BanGhiLogCoCauTruc[] {
  return cacLog.filter((l) => l.mucDo === mucDo && l.dichVu === dichVu);
}

// hai dong TA BIET deu la loi cua dich vu thanh-toan, nhung dinh dang KHONG dong nhat
const cacDongKhongDongNhat: string[] = [
  "10:05:00 ERROR service:thanh-toan reason:ket-noi-bi-tu-choi",
  "10:05:01 ERROR dich-vu=thanh-toan msg=timeout",
];
console.log("tim theo tu khoa 'dich-vu=thanh-toan' (mot dinh dang co dinh):", JSON.stringify(timTheoChuoiTuDo(cacDongKhongDongNhat, ["dich-vu=thanh-toan"])));
console.log("so dong tim duoc (THIEU dong 1, dinh dang khac):", timTheoChuoiTuDo(cacDongKhongDongNhat, ["dich-vu=thanh-toan"]).length);

const cacLogKhongDongNhat: BanGhiLogCoCauTruc[] = [
  { thoiDiem: 0, mucDo: "loi", dichVu: "thanh-toan", thongDiep: "ket noi bi tu choi", metadata: {} },
  { thoiDiem: 1, mucDo: "loi", dichVu: "thanh-toan", thongDiep: "timeout", metadata: {} },
];
console.log("loc theo field dichVu (khong quan tam thongDiep viet the nao):", JSON.stringify(locLogTheoMucDoVaDichVu(cacLogKhongDongNhat, "loi", "thanh-toan")));
console.log("so log DU tim duoc:", locLogTheoMucDoVaDichVu(cacLogKhongDongNhat, "loi", "thanh-toan").length);
```

```text title=readonly
tim theo tu khoa 'dich-vu=thanh-toan' (mot dinh dang co dinh): ["10:05:01 ERROR dich-vu=thanh-toan msg=timeout"]
so dong tim duoc (THIEU dong 1, dinh dang khac): 1
loc theo field dichVu (khong quan tam thongDiep viet the nao): [{"thoiDiem":0,"mucDo":"loi","dichVu":"thanh-toan","thongDiep":"ket noi bi tu choi","metadata":{}},{"thoiDiem":1,"mucDo":"loi","dichVu":"thanh-toan","thongDiep":"timeout","metadata":{}}]
so log DU tim duoc: 2
```

Cả hai dòng free-text đều nói VỀ cùng một sự thật — dịch vụ `thanh-toan` gặp
lỗi — nhưng `timTheoChuoiTuDo` chỉ tìm thấy `1`, bỏ SÓT dòng viết theo khuôn
`service:` thay vì `dich-vu=`. `locLogTheoMucDoVaDichVu` tìm ĐỦ cả `2`, vì
nó so field `dichVu` — một GIÁ TRỊ cố định, tách RIÊNG khỏi phần mô tả tự do
`thongDiep` — không quan tâm ai viết câu mô tả đó ra sao.
::::

::::predict{#doan-and-hai-dieu-kien commitOnce}
Một log có `mucDo="loi"` (đúng mức cần tìm) NHƯNG `dichVu="kho-hang"` (khác
dịch vụ đang tìm, ví dụ tìm `"thanh-toan"`). Gọi `locLogTheoMucDoVaDichVu`
với `mucDo="loi"`, `dichVu="thanh-toan"` — log NÀY có nằm trong kết quả trả
về không?

:::opt{correct}
Không — điều kiện lọc LÀ `l.mucDo === mucDo && l.dichVu === dichVu`, dùng
`&&`; CHỈ khớp `mucDo` thôi CHƯA đủ, `dichVu` cũng phải khớp ĐÚNG mới được
giữ lại
:::
:::opt
Có — vì `mucDo="loi"` LÀ điều kiện quan trọng nhất (log lỗi luôn đáng chú
Ý), nên chỉ cần khớp mức độ LÀ đủ để xuất hiện trong kết quả
::why
Nhầm "điều kiện quan trọng hơn" VỚI "điều kiện DUY NHẤT cần khớp" — nhưng
`locLogTheoMucDoVaDichVu` yêu cầu CẢ HAI điều kiện đúng, nối bằng `&&`, hàm
không hề coi `mucDo` LÀ đủ một mình.

Chỗ lệch: dòng `cacLog.filter((l) => l.mucDo === mucDo && l.dichVu ===
dichVu)` LÀ một phép AND. Với `l.dichVu = "kho-hang"` khác tham số truyền
vào `"thanh-toan"`, vế thứ hai LÀ `false` — toàn bộ biểu thức LÀ `false`,
NGAY cả khi `mucDo` khớp. Log ĐÓ bị loại khỏi kết quả, không được giữ lại.
::
:::
::::

::::code{#viet_loc_log_theo_muc_do_va_dich_vu}
Hoàn thiện `locLogTheoMucDoVaDichVu` — giữ lại CHỈ những log có `mucDo` VÀ
`dichVu` khớp ĐÚNG (cả hai điều kiện) với tham số truyền vào.

```typescript title=starter
interface BanGhiLogCoCauTruc {
  thoiDiem: number;
  mucDo: string;
  dichVu: string;
  thongDiep: string;
  metadata: Record<string, string>;
}
function locLogTheoMucDoVaDichVu(cacLog: BanGhiLogCoCauTruc[], mucDo: string, dichVu: string): BanGhiLogCoCauTruc[] {
  ___
}

const cacLogX: BanGhiLogCoCauTruc[] = [
  { thoiDiem: 0, mucDo: "loi", dichVu: "thanh-toan", thongDiep: "timeout", metadata: {} },
  { thoiDiem: 1, mucDo: "thong-tin", dichVu: "thanh-toan", thongDiep: "ok", metadata: {} },
];
const ketQuaX = locLogTheoMucDoVaDichVu(cacLogX, "loi", "thanh-toan");
console.log(ketQuaX.length, JSON.stringify(ketQuaX));
```

```typescript title=solution
interface BanGhiLogCoCauTruc {
  thoiDiem: number;
  mucDo: string;
  dichVu: string;
  thongDiep: string;
  metadata: Record<string, string>;
}
function locLogTheoMucDoVaDichVu(cacLog: BanGhiLogCoCauTruc[], mucDo: string, dichVu: string): BanGhiLogCoCauTruc[] {
  return cacLog.filter((l) => l.mucDo === mucDo && l.dichVu === dichVu);
}

const cacLogX: BanGhiLogCoCauTruc[] = [
  { thoiDiem: 0, mucDo: "loi", dichVu: "thanh-toan", thongDiep: "timeout", metadata: {} },
  { thoiDiem: 1, mucDo: "thong-tin", dichVu: "thanh-toan", thongDiep: "ok", metadata: {} },
];
const ketQuaX = locLogTheoMucDoVaDichVu(cacLogX, "loi", "thanh-toan");
console.log(ketQuaX.length, JSON.stringify(ketQuaX));
```

```typescript title=test
const rongT = locLogTheoMucDoVaDichVu([], "loi", "thanh-toan");
if (rongT.length !== 0) throw new Error("danh sach rong phai tra ve mang rong");

const cacLogT: BanGhiLogCoCauTruc[] = [
  { thoiDiem: 0, mucDo: "loi", dichVu: "thanh-toan", thongDiep: "timeout", metadata: {} },
  { thoiDiem: 1, mucDo: "loi", dichVu: "kho-hang", thongDiep: "het hang", metadata: {} },
  { thoiDiem: 2, mucDo: "thong-tin", dichVu: "thanh-toan", thongDiep: "ok", metadata: {} },
  { thoiDiem: 3, mucDo: "loi", dichVu: "thanh-toan", thongDiep: "the bi tu choi", metadata: { maLoi: "E1" } },
];

const ketQuaT = locLogTheoMucDoVaDichVu(cacLogT, "loi", "thanh-toan");
if (ketQuaT.length !== 2) throw new Error("phai co DUNG 2 log khop CA HAI dieu kien mucDo=loi va dichVu=thanh-toan");
const idDauT = ketQuaT[0];
if (idDauT === undefined || idDauT.thoiDiem !== 0) throw new Error("phan tu dau tien phai giu THU TU ban dau, thoiDiem=0");
const idSauT = ketQuaT[1];
if (idSauT === undefined || idSauT.thoiDiem !== 3) throw new Error("phan tu thu hai phai la thoiDiem=3");

const khoHangT = locLogTheoMucDoVaDichVu(cacLogT, "loi", "kho-hang");
if (khoHangT.length !== 1) throw new Error("mucDo=loi, dichVu=kho-hang phai tra ve DUNG 1 log");

const khongCoT = locLogTheoMucDoVaDichVu(cacLogT, "canh-bao", "thanh-toan");
if (khongCoT.length !== 0) throw new Error("mucDo khong ton tai trong du lieu phai tra ve mang rong, khong duoc khop nham voi mucDo khac");
```

:::hints
- kind: attention
  body: "Dung filter tren cacLog, giu lai nhung l co CA l.mucDo === mucDo VA l.dichVu === dichVu (noi bang &&)."
- kind: strategy
  body: "return cacLog.filter((l) => l.mucDo === mucDo && l.dichVu === dichVu);"
- kind: one-line
  body: "return cacLog.filter((l) => l.mucDo === mucDo && l.dichVu === dichVu);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "1 [{\"thoiDiem\":0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Field cố định, khớp CHÍNH XÁC — không còn đoán bằng substring. Nhưng log
có cấu trúc chỉ trả lời "chuyện gì xảy ra Ở một sự kiện". Muốn biết một con
số như "độ trễ trung bình" hay "có BAO NHIÊU request đang mở", cần một loại
dữ liệu khác hẳn — VÀ không phải loại metric nào cũng đo đúng cùng một cách.
::::

::::reflect{#nghi-lai}
`timTheoChuoiTuDo` không hề LÀ một cách tiếp cận tệ một cách ngẫu nhiên —
nó thất bại theo ĐÚNG hai hướng đối lập: khớp NHẦM khi từ khoá tình cờ xuất
hiện sai chỗ (dòng `don-hang` bị tính LÀ lỗi của `thanh-toan`), VÀ bỏ SÓT
khi cùng một sự thật được viết theo khuôn khác (`service:` thay vì
`dich-vu=`). `locLogTheoMucDoVaDichVu` không tránh được cả hai lỗi ĐÓ bằng
một mẹo thông minh nào — nó tránh được vì `mucDo` VÀ `dichVu` LÀ những FIELD
tách riêng, có kiểu dữ liệu cố định, không LẪN vào phần mô tả tự do.
::::

::::checkpoint{mastery=0.74}
::::
