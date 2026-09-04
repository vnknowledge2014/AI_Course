---
id: thiet-ke-he-thong.nhat-ky-bat-bien-va-fold.loi-quyet-dinh-thuan
title: "Lõi quyết định thuần: không I/O, không ngẫu nhiên, test không cần mock"
summary: "quyetDinh(trangThai, lenh, idSuKienMoi) la ham THUAN nhan trang thai HIEN TAI + mot lenh, tra ve DANH SACH su kien MOI hoac TU CHOI -- KHONG doc/ghi gi ben ngoai, khong random, khong Date.now(); idSuKienMoi PHAI duoc truyen VAO tu ben ngoai (khong tu sinh trong ham) vi tu sinh id (Math.random, counter) se pha vo tinh thuan; test bang du lieu THUAN, khong can mock database hay service nao."
locale: vi
track: thiet-ke-he-thong
module: nhat-ky-bat-bien-va-fold
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.fp.loi-quyet-dinh-thuan]
requires: [sd.fp.exactly-once-tu-at-least-once]
concepts: [sd.fp.loi-quyet-dinh-thuan]
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
Năm bài vừa qua đều nói về "sự kiện đã xảy ra" — nhưng chưa bài nào
hỏi: một sự kiện MỚI, trước khi được ghi, phải qua đâu để được CHẤP
NHẬN? Rút quá số dư có được không? Nạp âm tiền có hợp lệ không? Câu
trả lời cho những câu hỏi đó cần một chỗ đứng RIÊNG — tách hẳn khỏi
việc ghi hay việc fold.
::::

::::explain{#quyet-dinh-thuan-khong-io}
`quyetDinh` nhận trạng thái HIỆN tại VÀ một lệnh (`Lenh`), trả về
`KetQuaQuyetDinh` — hoặc chấp nhận kèm danh sách sự kiện MỚI, hoặc từ
chối kèm lý do. Hàm không hề đọc file, gọi mạng, in ra màn hình, hay
sinh số ngẫu nhiên — `idSuKienMoi` được truyền VÀO như một tham số,
không tự sinh bên trong (tự sinh — bằng `Math.random()` hay bộ đếm
toàn cục — sẽ phá vỡ tính xác định của hàm):

```typescript title=readonly
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }

type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }

type Lenh =
  | { loai: "nap"; soTien: number }
  | { loai: "rut"; soTien: number };

type KetQuaQuyetDinh =
  | { ketQua: "chap_nhan"; suKienMoi: SuKien[] }
  | { ketQua: "tu_choi"; lyDo: string };

function quyetDinh(trangThai: TrangThai, lenh: Lenh, idSuKienMoi: string): KetQuaQuyetDinh {
  if (lenh.loai === "nap") {
    return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_nap", soTien: lenh.soTien }] };
  }
  if (trangThai.soDu < lenh.soTien) {
    return { ketQua: "tu_choi", lyDo: "khong_du_so_du" };
  }
  return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_tru", soTien: lenh.soTien }] };
}

const trangThaiHienTai: TrangThai = { soDu: 50000 };

const ketQua1 = quyetDinh(trangThaiHienTai, { loai: "rut", soTien: 20000 }, "ev-1");
console.log("rut 20000 tu so du 50000:", JSON.stringify(ketQua1));

const ketQua2 = quyetDinh(trangThaiHienTai, { loai: "rut", soTien: 20000 }, "ev-1");
console.log("goi LAI voi het cac tham so giong het:", JSON.stringify(ketQua2));
console.log("hai lan goi cho ra CUNG mot ket qua (JSON giong het)?", JSON.stringify(ketQua1) === JSON.stringify(ketQua2));

const ketQuaTuChoi = quyetDinh(trangThaiHienTai, { loai: "rut", soTien: 999999 }, "ev-2");
console.log("rut 999999 (vuot so du):", JSON.stringify(ketQuaTuChoi));
```

```text title=readonly
rut 20000 tu so du 50000: {"ketQua":"chap_nhan","suKienMoi":[{"id":"ev-1","loai":"da_tru","soTien":20000}]}
goi LAI voi het cac tham so giong het: {"ketQua":"chap_nhan","suKienMoi":[{"id":"ev-1","loai":"da_tru","soTien":20000}]}
hai lan goi cho ra CUNG mot ket qua (JSON giong het)? true
rut 999999 (vuot so du): {"ketQua":"tu_choi","lyDo":"khong_du_so_du"}
```

Gọi `quyetDinh` HAI lần với ĐÚNG cùng ba tham số (`trangThaiHienTai`,
lệnh, `"ev-1"`) cho ra JSON GIỐNG HỆT nhau — đây là dấu hiệu của một
hàm thuần: cùng đầu vào, luôn cùng đầu ra. `quyetDinh` không hề ghi gì
vào `trangThaiHienTai`, không hề gọi `Math.random()` để tự đặt `id` —
`"ev-1"` là chuỗi ĐƯỢC TRUYỀN vào, không phải tự hàm sinh ra.
::::

::::example{#test-khong-can-mock}
Vì `quyetDinh` chỉ nhận VÀ trả về dữ liệu thuần, kiểm tra nó không
cần mở database, không cần mock một service nào — chỉ cần gọi hàm với
dữ liệu bịa ra:

```typescript title=readonly
interface TrangThai { soDu: number; }
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
type Lenh =
  | { loai: "nap"; soTien: number }
  | { loai: "rut"; soTien: number };
type KetQuaQuyetDinh =
  | { ketQua: "chap_nhan"; suKienMoi: SuKien[] }
  | { ketQua: "tu_choi"; lyDo: string };
function quyetDinh(trangThai: TrangThai, lenh: Lenh, idSuKienMoi: string): KetQuaQuyetDinh {
  if (lenh.loai === "nap") {
    return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_nap", soTien: lenh.soTien }] };
  }
  if (trangThai.soDu < lenh.soTien) {
    return { ketQua: "tu_choi", lyDo: "khong_du_so_du" };
  }
  return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_tru", soTien: lenh.soTien }] };
}

const kichBan: Array<{ ten: string; trangThai: TrangThai; lenh: Lenh; idMoi: string }> = [
  { ten: "nap luon duoc chap nhan", trangThai: { soDu: 0 }, lenh: { loai: "nap", soTien: 1000 }, idMoi: "t1" },
  { ten: "rut dung bang so du", trangThai: { soDu: 500 }, lenh: { loai: "rut", soTien: 500 }, idMoi: "t2" },
  { ten: "rut vuot so du bi tu choi", trangThai: { soDu: 100 }, lenh: { loai: "rut", soTien: 200 }, idMoi: "t3" },
];

for (const kb of kichBan) {
  const kq = quyetDinh(kb.trangThai, kb.lenh, kb.idMoi);
  console.log(kb.ten + ":", kq.ketQua);
}
console.log("KHONG mo mot database nao, KHONG mock mot service nao - chi goi ham voi du lieu THUAN");
```

```text title=readonly
nap luon duoc chap nhan: chap_nhan
rut dung bang so du: chap_nhan
rut vuot so du bi tu choi: tu_choi
KHONG mo mot database nao, KHONG mock mot service nao - chi goi ham voi du lieu THUAN
```

Ba kịch bản, ba object dữ liệu bịa ra tại chỗ, ba lời gọi hàm — không
có bước "khởi tạo môi trường test" nào cả. Đáng chú ý: "rút đúng bằng
số dư" (`soDu: 500`, rút `500`) được CHẤP nhận — `trangThai.soDu <
lenh.soTien` là `500 < 500`, tức `false`, nên không rơi vào nhánh từ
chối. Rút hết sạch số dư là hợp lệ; chỉ rút VƯỢT mới bị từ chối.
::::

::::predict{#doan-tu-sinh-id-ben-trong commitOnce}
Giả sử `quyetDinh` được viết LẠI để tự sinh `id` bên trong bằng
`crypto.randomUUID()`, thay vì nhận `idSuKienMoi` làm tham số. Gọi
hàm phiên bản MỚI này hai lần, với ĐÚNG cùng `trangThai` VÀ cùng
`lenh` — điều gì xảy ra?

:::opt{correct}
`quyetDinh` không còn THUẦN nữa — hai lần gọi với cùng đầu vào sẽ cho
ra hai sự kiện với `id` NGẪU NHIÊN khác nhau mỗi lần, phá vỡ tính xác
định (cùng đầu vào không còn đảm bảo cùng đầu ra)
:::
:::opt
Không sao cả — `id` chỉ là metadata đi kèm, không ảnh hưởng tới việc
hàm chấp nhận hay từ chối lệnh, nên tính thuần của phần LOGIC quyết
định vẫn được giữ nguyên
::why
Nhầm "tính thuần của MỘT NHÁNH quyết định" với "tính thuần của TOÀN
BỘ hàm" — nhưng `id` là một phần của GIÁ TRỊ trả về (nằm trong
`suKienMoi`), không phải một chi tiết đứng ngoài kết quả.

Chỗ lệch: định nghĩa hàm thuần là "cùng đầu vào luôn cho cùng đầu
ra" — so sánh bằng `JSON.stringify` hay bằng cấu trúc dữ liệu, không
chỉ so sánh "có chấp nhận hay không". Nếu `id` sinh ngẫu nhiên bên
trong, `JSON.stringify(ketQua1) === JSON.stringify(ketQua2)` sẽ trả
về `false` dù `trangThai` VÀ `lenh` giống hệt nhau — đúng điều mà bài
học Ở trên vừa chứng minh LÀ `true`. Đây chính LÀ lý do `idSuKienMoi`
phải LÀ tham số: việc sinh id (cần nguồn ngẫu nhiên hoặc bộ đếm toàn
cục) là việc của VỎ mệnh lệnh, không phải của lõi quyết định.
::
:::
::::

::::code{#viet_quyet_dinh}
Hoàn thiện `quyetDinh` — nếu `lenh.loai` là `"nap"`, LUÔN chấp nhận,
tạo một sự kiện `"da_nap"`. Nếu là `"rut"`, từ chối (VỚI lý do
`"khong_du_so_du"`) khi `trangThai.soDu < lenh.soTien`; ngược lại chấp
nhận, tạo một sự kiện `"da_tru"`. Sự kiện tạo ra PHẢI dùng đúng
`idSuKienMoi` được truyền vào.

```typescript title=starter
interface TrangThai { soDu: number; }
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
type Lenh =
  | { loai: "nap"; soTien: number }
  | { loai: "rut"; soTien: number };
type KetQuaQuyetDinh =
  | { ketQua: "chap_nhan"; suKienMoi: SuKien[] }
  | { ketQua: "tu_choi"; lyDo: string };

function quyetDinh(trangThai: TrangThai, lenh: Lenh, idSuKienMoi: string): KetQuaQuyetDinh {
  ___
}

const soDuBanDau: TrangThai = { soDu: 800 };
const kqDemo = quyetDinh(soDuBanDau, { loai: "rut", soTien: 300 }, "demo-1");
console.log(kqDemo.ketQua);
```

```typescript title=solution
interface TrangThai { soDu: number; }
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
type Lenh =
  | { loai: "nap"; soTien: number }
  | { loai: "rut"; soTien: number };
type KetQuaQuyetDinh =
  | { ketQua: "chap_nhan"; suKienMoi: SuKien[] }
  | { ketQua: "tu_choi"; lyDo: string };

function quyetDinh(trangThai: TrangThai, lenh: Lenh, idSuKienMoi: string): KetQuaQuyetDinh {
  if (lenh.loai === "nap") {
    return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_nap", soTien: lenh.soTien }] };
  }
  if (trangThai.soDu < lenh.soTien) {
    return { ketQua: "tu_choi", lyDo: "khong_du_so_du" };
  }
  return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_tru", soTien: lenh.soTien }] };
}

const soDuBanDau: TrangThai = { soDu: 800 };
const kqDemo = quyetDinh(soDuBanDau, { loai: "rut", soTien: 300 }, "demo-1");
console.log(kqDemo.ketQua);
```

```typescript title=test
const t1 = quyetDinh({ soDu: 0 }, { loai: "nap", soTien: 1000 }, "a1");
if (t1.ketQua !== "chap_nhan") throw new Error("nap tien LUON phai duoc chap nhan");
if (t1.ketQua === "chap_nhan") {
  if (t1.suKienMoi.length !== 1) throw new Error("nap tien phai tao DUNG 1 su kien moi");
  const sk = t1.suKienMoi[0];
  if (sk === undefined || sk.loai !== "da_nap" || sk.soTien !== 1000) throw new Error("su kien tao ra phai la da_nap voi dung soTien");
  if (sk.id !== "a1") throw new Error("id cua su kien phai DUNG idSuKienMoi duoc truyen vao");
}

const t2 = quyetDinh({ soDu: 500 }, { loai: "rut", soTien: 500 }, "a2");
if (t2.ketQua !== "chap_nhan") throw new Error("rut DUNG BANG so du phai duoc chap nhan (khong duoc coi la thieu)");

const t3 = quyetDinh({ soDu: 100 }, { loai: "rut", soTien: 200 }, "a3");
if (t3.ketQua !== "tu_choi") throw new Error("rut VUOT so du phai bi TU CHOI");

const trangThaiFix: TrangThai = { soDu: 300 };
const r1 = quyetDinh(trangThaiFix, { loai: "rut", soTien: 100 }, "same-id");
const r2 = quyetDinh(trangThaiFix, { loai: "rut", soTien: 100 }, "same-id");
if (JSON.stringify(r1) !== JSON.stringify(r2)) throw new Error("quyetDinh phai THUAN -- cung dau vao phai cho CUNG ket qua moi lan goi");
if (trangThaiFix.soDu !== 300) throw new Error("quyetDinh KHONG duoc sua doi tham so trangThai truyen vao");
```

:::hints
- kind: attention
  body: "Neu lenh.loai === 'nap', LUON tra ve chap_nhan voi 1 su kien da_nap. Neu la 'rut', kiem tra trangThai.soDu < lenh.soTien truoc -- neu dung thi tu_choi; nguoc lai chap_nhan voi 1 su kien da_tru. Nho dung DUNG idSuKienMoi lam id cua su kien, khong tu bia id khac."
- kind: strategy
  body: "if (lenh.loai === 'nap') { return { ketQua: 'chap_nhan', suKienMoi: [{ id: idSuKienMoi, loai: 'da_nap', soTien: lenh.soTien }] }; } if (trangThai.soDu < lenh.soTien) { return { ketQua: 'tu_choi', lyDo: 'khong_du_so_du' }; } return { ketQua: 'chap_nhan', suKienMoi: [{ id: idSuKienMoi, loai: 'da_tru', soTien: lenh.soTien }] };"
- kind: one-line
  body: "if (lenh.loai === \"nap\") { return { ketQua: \"chap_nhan\", suKienMoi: [{ id: idSuKienMoi, loai: \"da_nap\", soTien: lenh.soTien }] }; } if (trangThai.soDu < lenh.soTien) { return { ketQua: \"tu_choi\", lyDo: \"khong_du_so_du\" }; } return { ketQua: \"chap_nhan\", suKienMoi: [{ id: idSuKienMoi, loai: \"da_tru\", soTien: lenh.soTien }] };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "chap_nhan"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lõi quyết định thuần — không I/O, xác định, test không cần mock. Nhưng
ai gọi nó? Ai gán `id` cho sự kiện MỚI? Ai thật sự GHI sự kiện đó vào
nhật ký? Đến lượt lớp bao QUANH lõi này lên tiếng.
::::

::::reflect{#nghi-lai}
`quyetDinh` chỉ trả lời một câu hỏi duy nhất — "với trạng thái này VÀ
lệnh này, điều gì NÊN xảy ra?" — và trả lời nó mà không cần biết trạng
thái đó tới từ đâu, hay kết quả sẽ đi về đâu sau đó. Đây chính LÀ Ý
nghĩa của "lõi hàm" (functional core): một khối logic nghiệp vụ hoàn
toàn tách rời khỏi mọi hạ tầng — có thể copy nguyên hàm này sang một
dự án khác, chạy trên trình duyệt, trên server, hay trong một bài kiểm
tra, mà không cần sửa MỘT dòng nào.
::::

::::checkpoint{mastery=0.76}
::::
