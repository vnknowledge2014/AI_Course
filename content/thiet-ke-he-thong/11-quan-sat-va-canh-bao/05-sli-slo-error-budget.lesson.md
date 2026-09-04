---
id: thiet-ke-he-thong.quan-sat-va-canh-bao.sli-slo-error-budget
title: "SLI/SLO/error budget: định nghĩa \"đủ tốt\" bằng số, và ngân sách rủi ro được phép tiêu"
summary: "tinhSoLuongChoPhepHong(cauHinh) tinh errorBudgetPhanTram = 100 - sloPhanTram (lam tron 3 chu so de tranh loi so thuc), nhan voi tongDonViChuKy -- SLO 99.9% tren 1 trieu request/chu ky cho 1000 request duoc phep hong; CUNG cong thuc, doi donVi sang 'phut' tren chu ky 43200 phut (30 ngay) cho 43 phut duoc phep down. tinhTinhTrangBudget(cauHinh, soDonViDaDung) tra ve phanTramDaTieu = min(100, daDung/choPhep*100) VA phanTramConLai = 100-phanTramDaTieu -- da dung 250/1000 la 25%/75%; SLO=100% (khong cho loi nao) lam soDonViChoPhep=0, budget bi coi la DA TIEU HET (100%) NGAY CA KHI chua co loi nao xay ra."
locale: vi
track: thiet-ke-he-thong
module: quan-sat-va-canh-bao
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [sd.sli-slo-error-budget]
requires: [sd.distributed-tracing-theo-trace-id]
concepts: [sd.sli-slo-error-budget]
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
Bài trước tìm ra dịch vụ nào chiếm nhiều thời gian nhất. Nhưng "chiếm nhiều
thời gian" chỉ đáng lo NẾU nó khiến hệ thống không còn "đủ tốt" nữa — VÀ
"đủ tốt" không phải LÀ một cảm giác. Nó LÀ một con số, được thoả thuận
TRƯỚC, VÀ có một khoản "được PHÉP tệ" đi kèm — không tệ hơn khoản ĐÓ.
::::

::::explain{#sli-slo-error-budget}
`SLI` (Service Level Indicator) LÀ con số ĐO được — ví dụ tỷ lệ request
thành công. `SLO` (Service Level Objective) LÀ mục tiêu thoả thuận cho SLI
đó — ví dụ `99.9%`. `errorBudget` LÀ phần CÒN lại được PHÉP tệ:
`100% - SLO`. `tinhSoLuongChoPhepHong` chuyển phần trăm ĐÓ thành một con số
CỤ THỂ — số request, hoặc số phút — được phép hỏng trong một chu kỳ:

```typescript title=readonly
interface CauHinhSLO { sloPhanTram: number; tongDonViChuKy: number; donVi: string; }
function tinhSoLuongChoPhepHong(cauHinh: CauHinhSLO): number {
  const errorBudgetPhanTram = Math.round((100 - cauHinh.sloPhanTram) * 1000) / 1000;
  return Math.floor((errorBudgetPhanTram / 100) * cauHinh.tongDonViChuKy);
}

const sloRequest: CauHinhSLO = { sloPhanTram: 99.9, tongDonViChuKy: 1000000, donVi: "request" };
console.log("SLO 99.9%, 1 trieu request/chu ky -- so request duoc phep hong:", tinhSoLuongChoPhepHong(sloRequest));

const sloUptime: CauHinhSLO = { sloPhanTram: 99.9, tongDonViChuKy: 43200, donVi: "phut" };
console.log("SLO 99.9%, chu ky 30 ngay (43200 phut) -- so phut duoc phep down:", tinhSoLuongChoPhepHong(sloUptime));
```

```text title=readonly
SLO 99.9%, 1 trieu request/chu ky -- so request duoc phep hong: 1000
SLO 99.9%, chu ky 30 ngay (43200 phut) -- so phut duoc phep down: 43
```

CÙNG một công thức — `100% - SLO`, rồi nhân với tổng đơn vị trong chu kỳ —
áp dụng cho HAI loại đơn vị khác hẳn nhau: `1000` request được phép hỏng
trên một triệu request, HOẶC `43` phút được phép ngừng hoạt động trên một
chu kỳ `30` ngày. `errorBudget` không LÀ một khái niệm trừu tượng — nó LÀ
một con số cụ thể, đo được, VÀ có thể TIÊU dần.
::::

::::example{#da-tieu-va-con-lai}
`tinhTinhTrangBudget` so sánh số đơn vị THẬT sự đã hỏng với số được phép,
RỒI trả về phần trăm ĐÃ tiêu VÀ phần trăm CÒN lại. Một trường hợp đặc biệt:
nếu `SLO=100%` (không được phép hỏng dù chỉ MỘT lần), `soDonViChoPhep` LÀ
`0` — VÀ budget bị coi LÀ đã tiêu hết NGAY LẬP TỨC, dù CHƯA có lỗi nào xảy
ra:

```typescript title=readonly
interface CauHinhSLO { sloPhanTram: number; tongDonViChuKy: number; donVi: string; }
function tinhSoLuongChoPhepHong(cauHinh: CauHinhSLO): number {
  const errorBudgetPhanTram = Math.round((100 - cauHinh.sloPhanTram) * 1000) / 1000;
  return Math.floor((errorBudgetPhanTram / 100) * cauHinh.tongDonViChuKy);
}

interface TinhTrangBudget { phanTramDaTieu: number; phanTramConLai: number; soDonViChoPhep: number; soDonViDaDung: number; }
function tinhTinhTrangBudget(cauHinh: CauHinhSLO, soDonViDaDung: number): TinhTrangBudget {
  const soDonViChoPhep = tinhSoLuongChoPhepHong(cauHinh);
  const phanTramDaTieu = soDonViChoPhep === 0 ? 100 : Math.min(100, (soDonViDaDung / soDonViChoPhep) * 100);
  const phanTramConLai = 100 - phanTramDaTieu;
  return { phanTramDaTieu, phanTramConLai, soDonViChoPhep, soDonViDaDung };
}

const sloRequest: CauHinhSLO = { sloPhanTram: 99.9, tongDonViChuKy: 1000000, donVi: "request" };
console.log("da dung 250/1000 request loi:", JSON.stringify(tinhTinhTrangBudget(sloRequest, 250)));
console.log("da dung 1200/1000 (VUOT budget, van CHAN o 100%):", JSON.stringify(tinhTinhTrangBudget(sloRequest, 1200)));

// truong hop dac biet: SLO=100% nghia la KHONG duoc phep hong du CHI mot lan
const sloTuyetDoi: CauHinhSLO = { sloPhanTram: 100, tongDonViChuKy: 1000000, donVi: "request" };
console.log("SLO=100%, so request duoc phep hong:", tinhSoLuongChoPhepHong(sloTuyetDoi));
console.log("da dung 0 loi, nhung budget cho phep = 0:", JSON.stringify(tinhTinhTrangBudget(sloTuyetDoi, 0)));
```

```text title=readonly
da dung 250/1000 request loi: {"phanTramDaTieu":25,"phanTramConLai":75,"soDonViChoPhep":1000,"soDonViDaDung":250}
da dung 1200/1000 (VUOT budget, van CHAN o 100%): {"phanTramDaTieu":100,"phanTramConLai":0,"soDonViChoPhep":1000,"soDonViDaDung":1200}
SLO=100%, so request duoc phep hong: 0
da dung 0 loi, nhung budget cho phep = 0: {"phanTramDaTieu":100,"phanTramConLai":0,"soDonViChoPhep":0,"soDonViDaDung":0}
```

`250` lỗi trên `1000` được phép LÀ `25%` đã tiêu, còn NGUYÊN `75%` để "chi
tiêu" cho rủi ro — deploy nhanh hơn, thử nghiệm nhiều hơn. `1200` lỗi VƯỢT
quá `1000` được phép, nhưng `phanTramDaTieu` vẫn CHẶN Ở đúng `100`, không
vượt lên `120`. VÀ với `SLO=100%`, `soDonViChoPhep` LÀ `0` — nhánh đặc biệt
trong `tinhTinhTrangBudget` coi budget LÀ đã tiêu hết NGAY, vì chia cho `0`
không có Ý nghĩa, VÀ về mặt vận hành, "không được phép hỏng lần nào" nghĩa
LÀ không CÒN dư địa nào để chấp nhận rủi ro, kể cả khi chưa có lỗi thật.
::::

::::predict{#doan-slo-tuyet-doi commitOnce}
Một cấu hình có `sloPhanTram=99` (khác `100`), `tongDonViChuKy=50`. Gọi
`tinhSoLuongChoPhepHong` — kết quả LÀ bao nhiêu, VÀ TẠI SAO?

:::opt{correct}
`0` — `errorBudgetPhanTram = 100 - 99 = 1`; `(1 / 100) * 50 = 0.5`;
`Math.floor(0.5) = 0` — dù VỀ mặt tỷ lệ CÓ error budget (`1%`), số đơn vị
THẬT sự trong một chu kỳ CHỈ `50` LÀ quá ÍT để làm tròn ra được dù chỉ MỘT
đơn vị hỏng
:::
:::opt
`1` — error budget LÀ `1%`, VÀ với BẤT KỲ tổng số đơn vị nào lớn hơn `0`,
luôn nên làm TRÒN LÊN Ít nhất một đơn vị được phép hỏng, để không quá khắt
khe
::why
Nhầm "nên làm tròn LÊN để bớt khắt khe" VỚI cách `Math.floor` THẬT sự hoạt
động trong code — `tinhSoLuongChoPhepHong` dùng `Math.floor`, LÀM TRÒN
XUỐNG, không làm tròn lên.

Chỗ lệch: `errorBudgetPhanTram = 1`, `(1 / 100) * 50 = 0.5`. `Math.floor(0.5)`
LÀ `0`, không phải `1`. Khi tổng số đơn vị trong chu kỳ quá NHỎ so với tỷ
lệ error budget, kết quả LÀM TRÒN XUỐNG có thể ra ĐÚNG `0` — nghĩa LÀ về
mặt SỐ HỌC, hệ thống ĐÓ không có dư địa lỗi nào cả, dù SLO không phải LÀ
`100%`.
::
:::
::::

::::code{#viet_tinh_tinh_trang_budget}
Hoàn thiện `tinhTinhTrangBudget` — tính `soDonViChoPhep` (hàm ĐÃ có sẵn),
tính `phanTramDaTieu` (nếu `soDonViChoPhep=0` thì LÀ `100`, ngược lại LÀ
`soDonViDaDung / soDonViChoPhep * 100` NHƯNG chặn tối đa Ở `100` bằng
`Math.min`), rồi tính `phanTramConLai = 100 - phanTramDaTieu`.

```typescript title=starter
interface CauHinhSLO { sloPhanTram: number; tongDonViChuKy: number; donVi: string; }
function tinhSoLuongChoPhepHong(cauHinh: CauHinhSLO): number {
  const errorBudgetPhanTram = Math.round((100 - cauHinh.sloPhanTram) * 1000) / 1000;
  return Math.floor((errorBudgetPhanTram / 100) * cauHinh.tongDonViChuKy);
}

interface TinhTrangBudget { phanTramDaTieu: number; phanTramConLai: number; soDonViChoPhep: number; soDonViDaDung: number; }
function tinhTinhTrangBudget(cauHinh: CauHinhSLO, soDonViDaDung: number): TinhTrangBudget {
  ___
}

const cauHinhX: CauHinhSLO = { sloPhanTram: 99.5, tongDonViChuKy: 2000, donVi: "request" };
const ketQuaX = tinhTinhTrangBudget(cauHinhX, 5);
console.log(ketQuaX.soDonViChoPhep, ketQuaX.phanTramDaTieu, ketQuaX.phanTramConLai);
```

```typescript title=solution
interface CauHinhSLO { sloPhanTram: number; tongDonViChuKy: number; donVi: string; }
function tinhSoLuongChoPhepHong(cauHinh: CauHinhSLO): number {
  const errorBudgetPhanTram = Math.round((100 - cauHinh.sloPhanTram) * 1000) / 1000;
  return Math.floor((errorBudgetPhanTram / 100) * cauHinh.tongDonViChuKy);
}

interface TinhTrangBudget { phanTramDaTieu: number; phanTramConLai: number; soDonViChoPhep: number; soDonViDaDung: number; }
function tinhTinhTrangBudget(cauHinh: CauHinhSLO, soDonViDaDung: number): TinhTrangBudget {
  const soDonViChoPhep = tinhSoLuongChoPhepHong(cauHinh);
  const phanTramDaTieu = soDonViChoPhep === 0 ? 100 : Math.min(100, (soDonViDaDung / soDonViChoPhep) * 100);
  const phanTramConLai = 100 - phanTramDaTieu;
  return { phanTramDaTieu, phanTramConLai, soDonViChoPhep, soDonViDaDung };
}

const cauHinhX: CauHinhSLO = { sloPhanTram: 99.5, tongDonViChuKy: 2000, donVi: "request" };
const ketQuaX = tinhTinhTrangBudget(cauHinhX, 5);
console.log(ketQuaX.soDonViChoPhep, ketQuaX.phanTramDaTieu, ketQuaX.phanTramConLai);
```

```typescript title=test
const cauHinhT: CauHinhSLO = { sloPhanTram: 99.9, tongDonViChuKy: 1000000, donVi: "request" };
const chuaTieuT = tinhTinhTrangBudget(cauHinhT, 0);
if (chuaTieuT.phanTramDaTieu !== 0) throw new Error("chua co loi nao thi phanTramDaTieu phai la 0");
if (chuaTieuT.phanTramConLai !== 100) throw new Error("chua co loi nao thi phanTramConLai phai la 100");

const nuaBudgetT = tinhTinhTrangBudget(cauHinhT, 500);
if (nuaBudgetT.phanTramDaTieu !== 50) throw new Error("500/1000 phai la 50% da tieu");
if (nuaBudgetT.phanTramConLai !== 50) throw new Error("500/1000 phai con lai 50%");

const dungHetT = tinhTinhTrangBudget(cauHinhT, 1000);
if (dungHetT.phanTramDaTieu !== 100) throw new Error("dung DUNG het budget (1000/1000) phai la 100% da tieu");
if (dungHetT.phanTramConLai !== 0) throw new Error("dung DUNG het budget thi phanTramConLai phai la 0");

const vuotT = tinhTinhTrangBudget(cauHinhT, 5000);
if (vuotT.phanTramDaTieu !== 100) throw new Error("vuot qua budget van phai CHAN o 100%, khong duoc vuot len 500%");
if (vuotT.phanTramConLai !== 0) throw new Error("vuot qua budget thi phanTramConLai phai la 0, khong duoc am");

const sloTuyetDoiT: CauHinhSLO = { sloPhanTram: 100, tongDonViChuKy: 1000000, donVi: "request" };
const tuyetDoiT = tinhTinhTrangBudget(sloTuyetDoiT, 0);
if (tuyetDoiT.soDonViChoPhep !== 0) throw new Error("SLO=100% thi soDonViChoPhep phai la 0");
if (tuyetDoiT.phanTramDaTieu !== 100) throw new Error("SLO=100% (khong co budget) thi phanTramDaTieu phai la 100 NGAY CA KHI soDonViDaDung=0");
```

:::hints
- kind: attention
  body: "Ba buoc: const soDonViChoPhep = tinhSoLuongChoPhepHong(cauHinh); tinh phanTramDaTieu (neu soDonViChoPhep===0 thi 100, nguoc lai Math.min(100, soDonViDaDung/soDonViChoPhep*100)); tinh phanTramConLai = 100 - phanTramDaTieu; roi tra ve object du bon field."
- kind: strategy
  body: "const soDonViChoPhep = tinhSoLuongChoPhepHong(cauHinh); const phanTramDaTieu = soDonViChoPhep === 0 ? 100 : Math.min(100, (soDonViDaDung / soDonViChoPhep) * 100); const phanTramConLai = 100 - phanTramDaTieu; return { phanTramDaTieu, phanTramConLai, soDonViChoPhep, soDonViDaDung };"
- kind: one-line
  body: "const soDonViChoPhep = tinhSoLuongChoPhepHong(cauHinh); const phanTramDaTieu = soDonViChoPhep === 0 ? 100 : Math.min(100, (soDonViDaDung / soDonViChoPhep) * 100); const phanTramConLai = 100 - phanTramDaTieu; return { phanTramDaTieu, phanTramConLai, soDonViChoPhep, soDonViDaDung };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "10 50 50"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
"Đủ tốt" giờ LÀ một con số, VÀ ngân sách rủi ro có thể theo dõi được từng
phần trăm. Nhưng đo error budget chỉ có Ý nghĩa NẾU hệ thống báo động ĐÚNG
lúc nó bị tiêu quá nhanh — VÀ không phải mọi cách báo động đều đáng tin.
::::

::::reflect{#nghi-lai}
`tinhSoLuongChoPhepHong` VÀ `tinhTinhTrangBudget` không hề PHÁN xét SLO nào
LÀ "đúng" — chúng chỉ dịch một tỷ lệ phần trăm thoả thuận TRƯỚC thành những
con số VẬN hành được: số request, số phút, số phần trăm ĐÃ dùng. Trường hợp
`SLO=100%` cho thấy một điều tinh tế: khi KHÔNG chấp nhận rủi ro nào,
budget luôn Ở trạng thái "đã hết" — không phải VÌ hệ thống đang lỗi, mà VÌ
định nghĩa "đủ tốt" đó không hề chừa chỗ cho bất kỳ sai số nào ngay từ đầu.
::::

::::checkpoint{mastery=0.78}
::::
