---
id: ky-nghe-ung-dung-ai.ky-thuat-ngu-canh.cua-so-truot
title: "Cửa sổ trượt: cắt bớt message cũ nhất khi vượt ngân sách"
summary: "catCuaSoTruot(messages: ChatMessage[], nganSach: number): ChatMessage[] LUÔN giữ message ĐẦU TIÊN nếu role của nó là \"system\" (không bao giờ cắt system), rồi cắt bớt các message CŨ NHẤT còn lại theo ĐÚNG thứ tự trong mảng (không phải theo nội dung) cho tới khi tinhTongToken (tái dùng nguyên văn bài 1) không còn vượt ngân sách. Trên LICH_SU_HOI_THOAI (6 message, tổng 103 token, bài 1) với ngân sách 50: cắt CHÍNH XÁC 3 message giữa (index 1, 2, 3), giữ lại system + 2 message mới nhất, tổng còn lại 49 token — system LUÔN còn lại kể cả khi ngân sách cực nhỏ, và khi KHÔNG có message system ở đầu thì không message nào được ưu tiên, cắt từ chính đầu mảng."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-ngu-canh
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [kna.cua-so-truot]
requires: [kna.dem-token-gia-lap-va-vuot-ngan-sach]
concepts: [kna.cua-so-truot]
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
Bài trước đo được: `vuotNganSach` chỉ TRẢ LỜI có/không. Khi câu trả lời
LÀ "có" — CỬA SỔ PHẢI CO LẠI. Chiến lược đơn giản nhất, dùng khắp nơi
trong hệ thống thật: giữ những gì MỚI NHẤT, cắt những gì CŨ NHẤT. Đây
LÀ "cửa sổ trượt" (sliding window) — trượt về phía TRƯỚC theo thời
gian, bỏ lại phần ĐUÔI cũ phía sau.
::::

::::explain{#cua-so-truot-giu-system}
Một chi tiết không thể bỏ: message ĐẦU TIÊN thường LÀ chỉ thị hệ thống
(`role: "system"`) — quy tắc cho TOÀN BỘ phiên, không phải một lượt hội
thoại cụ thể. Cắt mất nó nghĩa LÀ model MẤT LUÔN chỉ thị nền tảng, tệ
hơn nhiều so với mất một message user/assistant cũ. `catCuaSoTruot`
tách riêng: nếu `messages[0]` LÀ `system`, GIỮ NÓ CỐ ĐỊNH; phần CÒN LẠI
mới LÀ đối tượng bị cắt, TỪ ĐẦU (cũ nhất) cho tới khi vừa ngân sách:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}

function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
}

function catCuaSoTruot(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (
    batDau < phanCoTheCat.length &&
    tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach
  ) {
    batDau += 1;
  }
  return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];
}
```

`messages[0]!` cần dấu `!` — `noUncheckedIndexedAccess` coi TRUY CẬP
QUA CHỈ SỐ luôn có thể LÀ `undefined`, kể cả sau khi đã kiểm
`messages.length === 0` Ở dòng trên (kiểm tra `.length` không làm
TypeScript thu hẹp kiểu của MỘT PHẦN TỬ truy cập qua chỉ số). `batDau`
tăng dần TỪNG MỘT — mỗi vòng lặp bỏ đi ĐÚNG một message cũ nhất còn lại
trong `phanCoTheCat`, cho tới khi tổng token (system + phần còn lại)
không còn vượt ngân sách, HOẶC hết sạch message để cắt.
::::

::::example{#do-tren-lich-su-cu-the}
Trên `LICH_SU_HOI_THOAI` (bài `1`, tổng `103` token) với ngân sách `50`:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}

function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
}

function catCuaSoTruot(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (
    batDau < phanCoTheCat.length &&
    tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach
  ) {
    batDau += 1;
  }
  return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];

const ketQuaCat = catCuaSoTruot(LICH_SU_HOI_THOAI, 50);
console.log("con lai:", ketQuaCat.length, "message");
console.log(ketQuaCat.map((m) => m.role));
console.log("tong token con lai:", tinhTongToken(ketQuaCat));
```

```text title=readonly
con lai: 3 message
["system","assistant","user"]
tong token con lai: 49
```

Từ `6` message xuống còn `3`: `system` (giữ CỐ ĐỊNH), rồi HAI message
MỚI NHẤT (`assistant` Ở index `4`, `user` Ở index `5`). BA message GIỮA
(index `1`, `2`, `3` — bao gồm CHÍNH message chứa mã đơn hàng
`"XY789"`) bị cắt hết, vì chúng LÀ những message CŨ NHẤT trong phần
"có thể cắt". Tổng token còn lại (`49`) vừa dưới ngân sách `50` — cắt
ĐÚNG số message cần, không cắt thừa.
::::

::::predict{#doan-khong-co-system commitOnce}
Gọi `catCuaSoTruot` trên một lịch sử KHÔNG có `system` Ở đầu (message
đầu tiên LÀ `role: "user"`), với ngân sách RẤT NHỎ (chỉ đủ giữ được
đúng MỘT message ngắn nhất). Message ĐẦU TIÊN (dù dài, dù chứa thông
tin quan trọng) có được ưu tiên giữ lại không?

:::opt{correct}
KHÔNG — `laHeThong` LÀ `false` (message đầu không phải `system`), nên
`phanGiuCoDinh` LÀ mảng RỖNG; TOÀN BỘ message (kể cả message đầu tiên)
đều nằm trong `phanCoTheCat` VÀ có thể bị cắt như bất kỳ message nào
khác — chỉ CÓ MỘT trường hợp được bảo vệ: `role === "system"` Ở đúng
vị trí đầu
:::
:::opt
CÓ — message đầu tiên LUÔN LÀ ngữ cảnh nền tảng của cuộc hội thoại, nên
`catCuaSoTruot` PHẢI có logic bảo vệ nó bất kể `role` LÀ gì
::why
Nhầm "trực giác về ngữ cảnh nền tảng" (một quan điểm THIẾT KẾ hợp lý)
VỚI những gì `catCuaSoTruot` THẬT SỰ kiểm tra — hàm chỉ có ĐÚNG một
điều kiện bảo vệ: `messages[0]!.role === "system"`, không có điều kiện
nào khác dựa trên VỊ TRÍ đơn thuần.

Chỗ lệch: nếu message đầu tiên LÀ `role: "user"`, điều kiện đó LÀ
`false`, `phanGiuCoDinh` RỖNG, và vòng `while` có thể cắt tới message
CUỐI CÙNG (kể cả message đầu) miễn còn vượt ngân sách.
::
:::
:::opt
Hàm ném lỗi, vì thiếu `system` LÀM `catCuaSoTruot` không biết bắt đầu
cắt TỪ ĐÂU
::why
Gần đúng Ở việc bạn nhận ra THIẾU `system` LÀ một trường hợp khác biệt
— quan sát đó đúng.

Chỗ lệch: `catCuaSoTruot` xử lý trường hợp này bằng RẼ NHÁNH (biểu thức
ba ngôi `laHeThong ? ... : ...`), không phải bằng `throw`.
`phanCoTheCat = messages.slice()` (TOÀN BỘ mảng) khi không có `system`
— hàm vẫn chạy bình thường, chỉ khác Ở chỗ không còn gì được ưu tiên.
::
:::
::::

::::code{#viet_cat_cua_so_truot}
Hoàn thiện `catCuaSoTruot` — phần ĐẦU: xác định `laHeThong`,
`phanGiuCoDinh`, `phanCoTheCat`. Phần SAU (dùng `layKetQua` đã cho
sẵn): vòng lặp tăng `soLuongDaCat`... À, quest này dùng biến `batDau`
(không phải `soLuongDaCat`) — cắt dần TỪ ĐẦU `phanCoTheCat` cho tới khi
vừa ngân sách, rồi trả về kết quả.

```typescript title=starter
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}

function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
}

function catCuaSoTruot(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  ___
  let batDau = 0;
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

const ketQuaCat = catCuaSoTruot(LICH_SU_HOI_THOAI, 50);
console.log(ketQuaCat.length, tinhTongToken(ketQuaCat), ketQuaCat[0]!.role);
```

```typescript title=solution
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}

function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
}

function catCuaSoTruot(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (
    batDau < phanCoTheCat.length &&
    tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach
  ) {
    batDau += 1;
  }
  return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];

const ketQuaCat = catCuaSoTruot(LICH_SU_HOI_THOAI, 50);
console.log(ketQuaCat.length, tinhTongToken(ketQuaCat), ketQuaCat[0]!.role);
```

```typescript title=test
if (ketQuaCat.length !== 3) throw new Error("phai con lai dung 3 message");
if (JSON.stringify(ketQuaCat.map((m) => m.content)) !== JSON.stringify([
  "Ban la tro ly cham soc khach hang, tra loi ngan gon.",
  "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang.",
  "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong.",
])) throw new Error("phai cat DUNG 3 message giua (index 1,2,3), giu system + 2 message moi nhat");
if (ketQuaCat[0]!.role !== "system") throw new Error("system phai luon con lai dau tien");
if (tinhTongToken(ketQuaCat) !== 49) throw new Error("tong token con lai phai la 49");

const khongCoHeThong: ChatMessage[] = [
  { role: "user", content: "Cau hoi rat dai can nhieu token de vuot qua ngan sach nho nay xem sao" },
  { role: "assistant", content: "Cau tra loi ngan" },
  { role: "user", content: "OK cam on ban nhe" },
];
const ketQuaKhongHeThong = catCuaSoTruot(khongCoHeThong, 8);
if (ketQuaKhongHeThong.length !== 1 || ketQuaKhongHeThong[0]!.content !== "OK cam on ban nhe") {
  throw new Error("khong co system thi cat tu dau, chi con lai message moi nhat vua ngan sach");
}

const chiHeThong = catCuaSoTruot(LICH_SU_HOI_THOAI, 5);
if (chiHeThong.length !== 1 || chiHeThong[0]!.role !== "system") throw new Error("ngan sach cuc nho van phai giu lai system, du no vuot ngan sach mot minh");

const khongCanCat = catCuaSoTruot(LICH_SU_HOI_THOAI, 1000);
if (khongCanCat.length !== 6) throw new Error("ngan sach du lon thi khong cat gi ca");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau: ba dong xac dinh laHeThong (messages[0]!.role === 'system'), phanGiuCoDinh (mang [messages[0]!] neu laHeThong, nguoc lai mang rong), phanCoTheCat (messages.slice(1) neu laHeThong, nguoc lai messages.slice()). Cho hai: mot while lap tang batDau tung mot, dieu kien dung la batDau con nho hon do dai phanCoTheCat VA tong token cua phanGiuCoDinh cong phanCoTheCat.slice(batDau) van con vuot nganSach; sau vong lap, return phanGiuCoDinh noi voi phanCoTheCat.slice(batDau)."
- kind: strategy
  body: "Cho dau: const laHeThong = messages[0]!.role === \"system\"; const phanGiuCoDinh = laHeThong ? [messages[0]!] : []; const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice(); Cho hai: while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; } return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung, DUNG THU TU."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "3 49 system"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`6` message xuống còn `3`, `system` luôn còn nguyên. Nhưng nhìn kỹ MỘT
trong ba message vừa bị cắt: nó chứa mã đơn hàng `"XY789"` — thông tin
mà một lượt SAU trong cuộc hội thoại có thể vẫn cần tới. Cắt CŨ NHẤT
không phân biệt "cũ nhưng QUAN TRỌNG" với "cũ và VÔ HẠI" — bài sau đặt
tên cho vấn đề này: context rot.
::::

::::reflect{#nghi-lai}
`catCuaSoTruot` KHÔNG hề "thông minh" — nó chỉ biết MỘT quy tắc: mới
hơn thì giữ, cũ hơn thì cắt, và một ngoại lệ DUY NHẤT (`system` Ở đúng
vị trí đầu). Đây LÀ chiến lược ĐƠN GIẢN NHẤT có thể viết cho bài toán
"cửa sổ vượt ngân sách" — và chính vì đơn giản, nó KHÔNG phân biệt được
"cũ nhưng vẫn cần" với "cũ và không còn cần nữa". Hai hàm của bài này —
`catCuaSoTruot` VÀ `tinhTongToken` tái dùng từ bài `1` — sẽ LÀ nền cho
mọi so sánh Ở các bài sau.
::::

::::checkpoint{mastery=0.78}
::::
