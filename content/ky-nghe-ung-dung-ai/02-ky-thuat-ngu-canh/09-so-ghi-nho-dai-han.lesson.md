---
id: ky-nghe-ung-dung-ai.ky-thuat-ngu-canh.so-ghi-nho-dai-han
title: "Sổ ghi nhớ dài hạn: trích xuất MỘT LẦN, tách biệt khỏi cửa sổ"
summary: "interface MucGhiNho { cumTu: string; noiDung: string }. trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] quét MỘT message DUY NHẤT (không phải toàn bộ lịch sử) — nếu content chứa MỘT cụm trong cacTuKhoaCanTrich, tạo MỘT MucGhiNho (message chứa NHIỀU cụm khớp tạo NHIỀU mục). capNhatSoGhiNho(soGhiNhoHienTai, tinMoi, cacTuKhoaCanTrich): MucGhiNho[] gọi trichXuatGhiNho CHỈ trên tinMoi rồi nối thêm vào soGhiNhoHienTai đã có — KHÔNG quét lại lịch sử cũ. Xử lý tuần tự 6 message của LICH_SU_HOI_THOAI với 3 từ khoá (\"XY789\", \"don hang\", \"30 ngay\"): sổ ghi nhớ cuối cùng có ĐÚNG 4 mục — message chứa CẢ \"XY789\" VÀ \"don hang\" tạo RA HAI mục riêng, không gộp lại thành một."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-ngu-canh
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.so-ghi-nho-dai-han]
requires: [kna.ngan-sach-theo-khu-vuc]
concepts: [kna.so-ghi-nho-dai-han]
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
q9.2a bài `6` (BOSS) tìm chỉ số chứa một sự kiện bằng cách QUÉT LẠI
TOÀN BỘ lịch sử Ở MỖI lượt (`timChiSoChuaCumTu`) — hoạt động đúng,
nhưng cái GIÁ phải trả LÀ quét lại từ đầu mỗi lần. Một kiến trúc gần
production hơn (gần vector memory/fact store thật) làm khác: TRÍCH XUẤT
MỘT LẦN, ngay khi message MỚI vừa tới, rồi CỘNG DỒN vào một SỔ GHI NHỚ
riêng — không bao giờ cần quét lại những gì đã xử lý xong.
::::

::::explain{#trich-xuat-mot-message}
`MucGhiNho` LÀ một mục ghi nhớ: `cumTu` (cụm từ khớp được) VÀ `noiDung`
(nội dung message gốc chứa cụm đó). `trichXuatGhiNho` CHỈ nhận MỘT
`tin: ChatMessage` — không nhận một mảng, không biết gì về lịch sử —
VÀ với MỖI cụm trong `cacTuKhoaCanTrich` CÓ MẶT trong `tin.content`, nó
tạo MỘT mục:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}

function trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  const ketQua: MucGhiNho[] = [];
  for (const tuKhoa of cacTuKhoaCanTrich) {
    if (tin.content.includes(tuKhoa)) {
      ketQua.push({ cumTu: tuKhoa, noiDung: tin.content });
    }
  }
  return ketQua;
}

console.log(JSON.stringify(trichXuatGhiNho(
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  ["XY789", "don hang", "30 ngay"],
)));
```

```text title=readonly
[{"cumTu":"XY789","noiDung":"Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang."},{"cumTu":"don hang","noiDung":"Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang."}]
```

MỘT message VỚI HAI cụm từ khớp (`"XY789"` VÀ `"don hang"` — cả hai
đều LÀ chuỗi con của `content` này) tạo ra HAI `MucGhiNho`, không phải
MỘT — mỗi cụm khớp LÀ một mục RIÊNG, dù chúng cùng trỏ về CÙNG một
`noiDung`. `capNhatSoGhiNho` gọi `trichXuatGhiNho` CHỈ trên message
MỚI, rồi nối thêm vào sổ đã có — KHÔNG bao giờ quét lại các message CŨ
đã xử lý xong Ở những lượt trước:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}

function trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  const ketQua: MucGhiNho[] = [];
  for (const tuKhoa of cacTuKhoaCanTrich) {
    if (tin.content.includes(tuKhoa)) {
      ketQua.push({ cumTu: tuKhoa, noiDung: tin.content });
    }
  }
  return ketQua;
}

function capNhatSoGhiNho(
  soGhiNhoHienTai: MucGhiNho[],
  tinMoi: ChatMessage,
  cacTuKhoaCanTrich: string[],
): MucGhiNho[] {
  return [...soGhiNhoHienTai, ...trichXuatGhiNho(tinMoi, cacTuKhoaCanTrich)];
}
```
::::

::::example{#tich-luy-qua-nhieu-luot}
Xử lý TUẦN TỰ cả sáu message của `LICH_SU_HOI_THOAI` (bài `1`, q9.2a),
MỖI lần gọi `capNhatSoGhiNho` với CHỈ message đó — không truyền lại
toàn bộ lịch sử:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}

function trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  const ketQua: MucGhiNho[] = [];
  for (const tuKhoa of cacTuKhoaCanTrich) {
    if (tin.content.includes(tuKhoa)) {
      ketQua.push({ cumTu: tuKhoa, noiDung: tin.content });
    }
  }
  return ketQua;
}

function capNhatSoGhiNho(
  soGhiNhoHienTai: MucGhiNho[],
  tinMoi: ChatMessage,
  cacTuKhoaCanTrich: string[],
): MucGhiNho[] {
  return [...soGhiNhoHienTai, ...trichXuatGhiNho(tinMoi, cacTuKhoaCanTrich)];
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];
const TU_KHOA_CAN_TRICH = ["XY789", "don hang", "30 ngay"];

let soGhiNho: MucGhiNho[] = [];
for (const tin of LICH_SU_HOI_THOAI) {
  soGhiNho = capNhatSoGhiNho(soGhiNho, tin, TU_KHOA_CAN_TRICH);
}
console.log("so luong muc:", soGhiNho.length);
console.log(soGhiNho.map((m) => m.cumTu));
```

```text title=readonly
so luong muc: 4
["XY789","don hang","don hang","30 ngay"]
```

Bốn mục, từ SÁU message: message Ở index `1` một mình sinh ra HAI mục
(`"XY789"` VÀ `"don hang"`, vì nó chứa CẢ HAI); message Ở index `2`
sinh MỘT mục nữa (`"don hang"`, lặp lại — mỗi lần khớp LÀ một mục MỚI,
không kiểm trùng); message Ở index `4` sinh MỘT mục (`"30 ngay"`). Ba
message còn lại (index `0`, `3`, `5`) không chứa cụm nào, không sinh
mục nào. Mỗi lượt gọi `capNhatSoGhiNho` CHỈ xử lý ĐÚNG message vừa tới
— không bao giờ quét lại năm message trước đó.
::::

::::predict{#doan-mot-message-hai-cum commitOnce}
Message Ở `index 1` (`"Ma don hang cua toi la XY789..."`) chứa CẢ HAI
cụm `"XY789"` VÀ `"don hang"`. Gọi `trichXuatGhiNho` trên ĐÚNG message
đó với `cacTuKhoaCanTrich = ["XY789", "don hang"]` — kết quả trả về
BAO NHIÊU `MucGhiNho`?

:::opt{correct}
`2` — `trichXuatGhiNho` lặp qua TỪNG cụm trong `cacTuKhoaCanTrich`, VÀ
với MỖI cụm CÓ MẶT trong `content`, nó `push` MỘT `MucGhiNho` RIÊNG;
message chứa CẢ HAI cụm tạo ra HAI mục, không gộp lại thành một, dù cả
hai đều trỏ về CÙNG một `noiDung`
:::
:::opt
`1` — dù chứa hai cụm khác nhau, chúng đều tới TỪ CÙNG một message nên
chỉ cần MỘT mục ghi nhớ đại diện LÀ đủ
::why
Nhầm "một message chỉ nên sinh một mục ghi nhớ" (một lựa chọn thiết kế
CÓ THỂ hợp lý Ở một hệ thống khác) VỚI cách `trichXuatGhiNho` THẬT SỰ
hoạt động — hàm này lặp theo TỪNG CỤM TỪ cần trích (`for (const tuKhoa
of cacTuKhoaCanTrich)`), không lặp theo TỪNG MESSAGE, nên số mục sinh
ra bằng số cụm TÌM THẤY, không giới hạn MỘT mục mỗi message.

Chỗ lệch: vòng lặp `push` MỘT `MucGhiNho` MỖI khi `tin.content.includes(tuKhoa)`
LÀ `true` — không có bước nào gộp lại hay kiểm trùng theo message.
::
:::
:::opt
`0` — `trichXuatGhiNho` chỉ trích được cụm từ Ở message ĐẦU TIÊN của
một cuộc hội thoại, không áp dụng cho message Ở giữa
::why
Gần đúng Ở việc bạn nghĩ tới VỊ TRÍ của message trong một cuộc hội
thoại — nhưng `trichXuatGhiNho` nhận đúng MỘT `tin: ChatMessage` LÀM
tham số, hoàn toàn không biết (VÀ không cần biết) nó nằm Ở vị trí nào
trong một lịch sử lớn hơn.

Chỗ lệch: hàm chỉ đọc `tin.content` của THAM SỐ được truyền vào — bất
kỳ message nào, Ở bất kỳ vị trí nào, đều được xử lý giống hệt nhau.
::
:::
::::

::::code{#viet_trich_xuat_va_cap_nhat_ghi_nho}
Hoàn thiện `trichXuatGhiNho` — lặp qua `cacTuKhoaCanTrich`, VỚI mỗi cụm
`tin.content` CÓ CHỨA, `push` một `MucGhiNho`. Hoàn thiện
`capNhatSoGhiNho` — nối `soGhiNhoHienTai` VỚI kết quả `trichXuatGhiNho`
áp lên `tinMoi` (KHÔNG quét lại lịch sử cũ).

```typescript title=starter
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}

function trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  ___
}

function capNhatSoGhiNho(
  soGhiNhoHienTai: MucGhiNho[],
  tinMoi: ChatMessage,
  cacTuKhoaCanTrich: string[],
): MucGhiNho[] {
  ___
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];
const TU_KHOA_CAN_TRICH = ["XY789", "don hang", "30 ngay"];

let soGhiNho: MucGhiNho[] = [];
for (const tin of LICH_SU_HOI_THOAI) {
  soGhiNho = capNhatSoGhiNho(soGhiNho, tin, TU_KHOA_CAN_TRICH);
}
console.log(soGhiNho.length, JSON.stringify(soGhiNho.map((m) => m.cumTu)));
```

```typescript title=solution
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}

function trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  const ketQua: MucGhiNho[] = [];
  for (const tuKhoa of cacTuKhoaCanTrich) {
    if (tin.content.includes(tuKhoa)) {
      ketQua.push({ cumTu: tuKhoa, noiDung: tin.content });
    }
  }
  return ketQua;
}

function capNhatSoGhiNho(
  soGhiNhoHienTai: MucGhiNho[],
  tinMoi: ChatMessage,
  cacTuKhoaCanTrich: string[],
): MucGhiNho[] {
  return [...soGhiNhoHienTai, ...trichXuatGhiNho(tinMoi, cacTuKhoaCanTrich)];
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];
const TU_KHOA_CAN_TRICH = ["XY789", "don hang", "30 ngay"];

let soGhiNho: MucGhiNho[] = [];
for (const tin of LICH_SU_HOI_THOAI) {
  soGhiNho = capNhatSoGhiNho(soGhiNho, tin, TU_KHOA_CAN_TRICH);
}
console.log(soGhiNho.length, JSON.stringify(soGhiNho.map((m) => m.cumTu)));
```

```typescript title=test
if (soGhiNho.length !== 4) throw new Error("sau khi xu ly het 6 message, so ghi nho phai co dung 4 muc");
if (JSON.stringify(soGhiNho.map((m) => m.cumTu)) !== JSON.stringify(["XY789", "don hang", "don hang", "30 ngay"])) {
  throw new Error("thu tu cum tu phai la XY789, don hang, don hang, 30 ngay -- dung thu tu xuat hien trong lich su");
}
if (soGhiNho[0]!.noiDung !== "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang.") {
  throw new Error("muc dau tien phai luu noi dung goc cua message chua XY789");
}

const haiCum = trichXuatGhiNho(
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  ["XY789", "don hang"],
);
if (haiCum.length !== 2) throw new Error("mot message chua CA HAI cum tu phai sinh DUNG hai MucGhiNho, khong gop lai");
if (haiCum[0]!.cumTu !== "XY789" || haiCum[1]!.cumTu !== "don hang") {
  throw new Error("thu tu hai muc phai dung thu tu trong cacTuKhoaCanTrich");
}

if (trichXuatGhiNho({ role: "user", content: "khong lien quan gi ca" }, ["XY789"]).length !== 0) {
  throw new Error("message khong chua cum nao thi phai tra ve mang rong");
}

const soRong: MucGhiNho[] = [];
const sauMotLanCapNhat = capNhatSoGhiNho(soRong, LICH_SU_HOI_THOAI[1]!, TU_KHOA_CAN_TRICH);
if (sauMotLanCapNhat.length !== 2) throw new Error("capNhatSoGhiNho tren so RONG + 1 message 2-cum phai cho ra dung 2 muc");
if (soRong.length !== 0) throw new Error("capNhatSoGhiNho KHONG duoc sua doi soGhiNhoHienTai goc (phai tao mang moi)");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (trichXuatGhiNho): tao ketQua: MucGhiNho[] = []; for (const tuKhoa of cacTuKhoaCanTrich) { if (tin.content.includes(tuKhoa)) ketQua.push({ cumTu: tuKhoa, noiDung: tin.content }); } roi return ketQua. Cho hai (capNhatSoGhiNho): return [...soGhiNhoHienTai, ...trichXuatGhiNho(tinMoi, cacTuKhoaCanTrich)] -- CHI goi tren tinMoi, KHONG quet lai soGhiNhoHienTai."
- kind: strategy
  body: "Cho dau: const ketQua: MucGhiNho[] = []; for (const tuKhoa of cacTuKhoaCanTrich) { if (tin.content.includes(tuKhoa)) { ketQua.push({ cumTu: tuKhoa, noiDung: tin.content }); } } return ketQua; Cho hai: return [...soGhiNhoHienTai, ...trichXuatGhiNho(tinMoi, cacTuKhoaCanTrich)];"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "4 [\"XY789\",\"don hang\",\"don hang\",\"30 ngay\"]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn mục, tích luỹ qua sáu lượt gọi TÁCH BIỆT — mỗi lượt chỉ xử lý ĐÚNG
message vừa tới, không quét lại gì cả. Nhưng "sổ ghi nhớ có đúng các
mục quan trọng" mới chỉ LÀ MỘT NỬA câu hỏi — nửa còn lại: nó có
THẬT SỰ giữ được đủ THÔNG TIN người dùng CẦN hay không? Bài sau đo điều
đó bằng một con số LIÊN TỤC, không phải một boolean nhị phân.
::::

::::reflect{#nghi-lai}
`trichXuatGhiNho` chỉ xử lý MỘT message — đây LÀ điểm khác biệt cốt lõi
so với `coConNhoSuKien` (bài `3`, q9.2a) hay `timChiSoChuaCumTu` (bài
`6`, q9.2a), cả hai đều quét TOÀN BỘ một mảng MỖI lần gọi.
`capNhatSoGhiNho` bù lại bằng cách TÍCH LUỸ: gọi nó LẶP ĐI LẶP LẠI, mỗi
lần với đúng MỘT message mới, LÀ đủ để xây một sổ ghi nhớ đầy đủ theo
thời gian — không cần bao giờ nhìn lại quá khứ. Đây LÀ nguyên lý gần
với một fact store/vector memory THẬT: chi phí trích xuất trả MỘT LẦN
Ở đúng thời điểm message xuất hiện, không phải trả ĐI TRẢ LẠI mỗi khi
cần tra cứu.
::::

::::checkpoint{mastery=0.8}
::::
