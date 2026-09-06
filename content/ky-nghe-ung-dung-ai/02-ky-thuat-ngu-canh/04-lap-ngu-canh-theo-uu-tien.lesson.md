---
id: ky-nghe-ung-dung-ai.ky-thuat-ngu-canh.lap-ngu-canh-theo-uu-tien
title: "Lắp ngữ cảnh theo ưu tiên: ghim, không cắt mù theo tuổi"
summary: "catTheoUuTien(messages: ChatMessage[], nganSach: number, cacChiSoGhim: number[]): ChatMessage[] LUÔN giữ system (chỉ số 0 nếu có) VÀ các message tại cacChiSoGhim (chỉ số được \"ghim\"), rồi mới cắt bớt message CŨ NHẤT còn lại (không ghim, không phải system) cho tới khi vừa ngân sách. Trên CÙNG LICH_SU_HOI_THOAI + ngân sách 50 của bài 3, nhưng ghim chỉ số 1 (message chứa \"XY789\"): coConNhoSuKien SAU khi cắt bằng catTheoUuTien → true — khác hẳn kết quả false của catCuaSoTruot mù (bài 2) ở CÙNG ngân sách. Không ghim gì (cacChiSoGhim=[]) cho kết quả giống HỆT catCuaSoTruot; ghim SAI chỉ số (không phải message chứa sự kiện) thì recall vẫn mất — tham số cacChiSoGhim thật sự quyết định kết quả, không phải trang trí."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-ngu-canh
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.lap-ngu-canh-theo-uu-tien]
requires: [kna.context-rot-va-recall-trong-cua-so]
concepts: [kna.lap-ngu-canh-theo-uu-tien]
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
Context rot (bài trước) không phải LÀ một hệ quả BẮT BUỘC của việc cắt
bớt — nó LÀ hệ quả của việc cắt MÙ, chỉ dựa vào TUỔI của message. Nếu
hệ thống biết TRƯỚC message nào chứa thông tin quan trọng, nó có thể
"ghim" message đó — bảo vệ khỏi bị cắt, bất kể tuổi. Đây LÀ ý tưởng
đơn giản nhất để lắp ngữ cảnh THEO ƯU TIÊN thay vì mù theo tuổi.
::::

::::explain{#cat-theo-uu-tien}
`catTheoUuTien` mở rộng đúng MỘT ý tưởng của `catCuaSoTruot` (bài `2`):
thay vì CHỈ có MỘT chỉ số được bảo vệ cố định (`system` Ở vị trí `0`),
giờ có một DANH SÁCH chỉ số được "ghim" — `cacChiSoGhim`. Message nào
nằm trong danh sách ghim (hoặc LÀ `system` Ở vị trí `0`) LUÔN được giữ;
phần CÒN LẠI mới bị cắt, VẪN theo đúng quy tắc "cũ nhất trước":

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

function catTheoUuTien(messages: ChatMessage[], nganSach: number, cacChiSoGhim: number[]): ChatMessage[] {
  const ghim = new Set<number>(cacChiSoGhim);
  if (messages.length > 0 && messages[0]!.role === "system") ghim.add(0);
  const chiSoCoTheCat = messages.map((_, i) => i).filter((i) => !ghim.has(i));
  let soLuongDaCat = 0;
  const layKetQua = (soCat: number): ChatMessage[] => {
    const biCat = new Set(chiSoCoTheCat.slice(0, soCat));
    return messages.filter((_, i) => !biCat.has(i));
  };
  while (soLuongDaCat < chiSoCoTheCat.length && tinhTongToken(layKetQua(soLuongDaCat)) > nganSach) {
    soLuongDaCat += 1;
  }
  return layKetQua(soLuongDaCat);
}
```

`chiSoCoTheCat` LÀ danh sách các CHỈ SỐ (không phải message) chưa bị
ghim, theo ĐÚNG thứ tự trong mảng gốc — "cũ nhất" trong danh sách này
LÀ chỉ số NHỎ NHẤT. `layKetQua(soCat)` trả về mảng SAU khi bỏ đi
`soCat` chỉ số ĐẦU TIÊN của `chiSoCoTheCat` — dùng `messages.filter`
(không phải nối mảng con) để kết quả LUÔN giữ ĐÚNG thứ tự thời gian
gốc, kể cả khi message được ghim nằm Ở GIỮA mảng.
::::

::::example{#do-tren-cung-kich-ban}
CÙNG `LICH_SU_HOI_THOAI` + ngân sách `50` của bài `3` — nhưng lần này
GHIM chỉ số `1` (message chứa `"XY789"`):

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

function catTheoUuTien(messages: ChatMessage[], nganSach: number, cacChiSoGhim: number[]): ChatMessage[] {
  const ghim = new Set<number>(cacChiSoGhim);
  if (messages.length > 0 && messages[0]!.role === "system") ghim.add(0);
  const chiSoCoTheCat = messages.map((_, i) => i).filter((i) => !ghim.has(i));
  let soLuongDaCat = 0;
  const layKetQua = (soCat: number): ChatMessage[] => {
    const biCat = new Set(chiSoCoTheCat.slice(0, soCat));
    return messages.filter((_, i) => !biCat.has(i));
  };
  while (soLuongDaCat < chiSoCoTheCat.length && tinhTongToken(layKetQua(soLuongDaCat)) > nganSach) {
    soLuongDaCat += 1;
  }
  return layKetQua(soLuongDaCat);
}

function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  return messages.some((tin) => tin.content.includes(cumTuCanTim));
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];

const ketQuaUuTien = catTheoUuTien(LICH_SU_HOI_THOAI, 50, [1]);
console.log(ketQuaUuTien.map((m) => m.role));
console.log("con nho su kien:", coConNhoSuKien(ketQuaUuTien, "XY789"));
console.log("tong token con lai:", tinhTongToken(ketQuaUuTien));
```

```text title=readonly
["system","user","user"]
con nho su kien: true
tong token con lai: 48
```

`3` message còn lại: `system`, message ghim (chứa `"XY789"`), VÀ
message mới nhất. `coConNhoSuKien` trả về `true` — KHÁC HẲN bài `3`,
nơi CÙNG ngân sách `50` nhưng dùng `catCuaSoTruot` (mù) cho ra `false`.
Chỉ MỘT thay đổi — ghim ĐÚNG chỉ số chứa sự kiện — đảo ngược hoàn toàn
kết quả recall, không cần tăng ngân sách.
::::

::::predict{#doan-khong-ghim-gi commitOnce}
Gọi `catTheoUuTien(LICH_SU_HOI_THOAI, 50, [])` — mảng ghim RỖNG, không
ghim chỉ số nào cả. So với `catCuaSoTruot(LICH_SU_HOI_THOAI, 50)` (bài
`2`), hai kết quả có GIỐNG NHAU không?

:::opt{correct}
CÓ, giống HỆT nhau — không ghim gì thì `ghim` chỉ chứa `{0}` (từ
`system`, y hệt `catCuaSoTruot`), `chiSoCoTheCat` LÀ TOÀN BỘ chỉ số
CÒN LẠI theo đúng thứ tự cũ→mới, và vòng lặp cắt dần TỪ ĐẦU danh sách
đó — chính XÁC LÀ hành vi "cắt cũ nhất trước, giữ system" của bài `2`
:::
:::opt
KHÁC nhau — `catTheoUuTien` có thêm tham số thứ ba nên LUÔN xử lý khác
`catCuaSoTruot`, dù tham số đó có rỗng hay không
::why
Nhầm "có THÊM một tham số" VỚI "LUÔN cho kết quả khác" — nhưng một
tham số RỖNG (`[]`) không hề thay đổi tập `ghim` so với khi không có
tham số đó — `new Set([])` LÀ một tập RỖNG, `ghim.add(0)` vẫn chạy y hệt.

Chỗ lệch: `catTheoUuTien` với `cacChiSoGhim = []` suy biến CHÍNH XÁC về
đúng logic của `catCuaSoTruot` — không phải một sự trùng hợp, mà LÀ
thiết kế: `catCuaSoTruot` LÀ một trường hợp ĐẶC BIỆT của
`catTheoUuTien` khi không ghim gì thêm.
::
:::
:::opt
Không so sánh được — hai hàm trả về kiểu dữ liệu khác nhau
::why
Gần đúng Ở việc bạn để Ý hai hàm có CHỮ KÝ (signature) khác nhau — quan
sát đó đúng (một hàm nhận `2` tham số, hàm kia nhận `3`).

Chỗ lệch: kiểu dữ liệu TRẢ VỀ của cả hai đều LÀ `ChatMessage[]` — hoàn
toàn so sánh được bằng `JSON.stringify` hoặc so từng phần tử, như mọi
bài trước đã làm.
::
:::
::::

::::code{#viet_cat_theo_uu_tien}
Hoàn thiện `catTheoUuTien` — phần ĐẦU: dựng tập `ghim` (từ
`cacChiSoGhim`, cộng thêm `0` nếu message đầu LÀ `system`), tính
`chiSoCoTheCat` (chỉ số chưa ghim), khởi tạo `soLuongDaCat = 0`. Phần
SAU (dùng `layKetQua` đã cho sẵn): vòng lặp tăng `soLuongDaCat` cho
tới khi vừa ngân sách, rồi trả kết quả.

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

function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  return messages.some((tin) => tin.content.includes(cumTuCanTim));
}

function catTheoUuTien(messages: ChatMessage[], nganSach: number, cacChiSoGhim: number[]): ChatMessage[] {
  ___
  const layKetQua = (soCat: number): ChatMessage[] => {
    const biCat = new Set(chiSoCoTheCat.slice(0, soCat));
    return messages.filter((_, i) => !biCat.has(i));
  };
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

const ketQuaUuTien = catTheoUuTien(LICH_SU_HOI_THOAI, 50, [1]);
console.log(ketQuaUuTien.length, tinhTongToken(ketQuaUuTien), coConNhoSuKien(ketQuaUuTien, "XY789"));
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

function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  return messages.some((tin) => tin.content.includes(cumTuCanTim));
}

function catTheoUuTien(messages: ChatMessage[], nganSach: number, cacChiSoGhim: number[]): ChatMessage[] {
  const ghim = new Set<number>(cacChiSoGhim);
  if (messages.length > 0 && messages[0]!.role === "system") ghim.add(0);
  const chiSoCoTheCat = messages.map((_, i) => i).filter((i) => !ghim.has(i));
  let soLuongDaCat = 0;
  const layKetQua = (soCat: number): ChatMessage[] => {
    const biCat = new Set(chiSoCoTheCat.slice(0, soCat));
    return messages.filter((_, i) => !biCat.has(i));
  };
  while (soLuongDaCat < chiSoCoTheCat.length && tinhTongToken(layKetQua(soLuongDaCat)) > nganSach) {
    soLuongDaCat += 1;
  }
  return layKetQua(soLuongDaCat);
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];

const ketQuaUuTien = catTheoUuTien(LICH_SU_HOI_THOAI, 50, [1]);
console.log(ketQuaUuTien.length, tinhTongToken(ketQuaUuTien), coConNhoSuKien(ketQuaUuTien, "XY789"));
```

```typescript title=test
if (ketQuaUuTien.length !== 3) throw new Error("phai con lai dung 3 message");
if (JSON.stringify(ketQuaUuTien.map((m) => m.content)) !== JSON.stringify([
  "Ban la tro ly cham soc khach hang, tra loi ngan gon.",
  "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang.",
  "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong.",
])) throw new Error("phai giu system + message ghim (chua XY789) + message moi nhat con vua ngan sach");
if (tinhTongToken(ketQuaUuTien) !== 48) throw new Error("tong token con lai phai la 48");
if (coConNhoSuKien(ketQuaUuTien, "XY789") !== true) throw new Error("ghim dung message chua su kien thi PHAI con nho, khac han sliding window mu");

const catCuaSoTruotLai = (messages: ChatMessage[], nganSach: number): ChatMessage[] => {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; }
  return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];
};
const khongGhim = catTheoUuTien(LICH_SU_HOI_THOAI, 50, []);
if (JSON.stringify(khongGhim.map((m) => m.content)) !== JSON.stringify(catCuaSoTruotLai(LICH_SU_HOI_THOAI, 50).map((m) => m.content))) {
  throw new Error("khong ghim gi thi phai giong het hanh vi sliding window mu");
}

const ghimChiSo3 = catTheoUuTien(LICH_SU_HOI_THOAI, 50, [3]);
if (JSON.stringify(ghimChiSo3.map((m) => m.content)) === JSON.stringify(ketQuaUuTien.map((m) => m.content))) {
  throw new Error("ghim chi so KHAC nhau phai cho ket qua KHAC nhau -- tham so cacChiSoGhim phai that su rang buoc");
}
if (coConNhoSuKien(ghimChiSo3, "XY789") !== false) throw new Error("ghim chi so 3 (khong phai 1) thi KHONG con bao ve duoc message chua XY789");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau: bon dong -- const ghim = new Set<number>(cacChiSoGhim); if (messages.length > 0 && messages[0]!.role === 'system') ghim.add(0); const chiSoCoTheCat = messages.map((_, i) => i).filter((i) => !ghim.has(i)); let soLuongDaCat = 0; Cho hai: mot while lap tang soLuongDaCat tung mot, dieu kien dung la soLuongDaCat con nho hon do dai chiSoCoTheCat VA tinhTongToken(layKetQua(soLuongDaCat)) van con vuot nganSach; sau vong lap, return layKetQua(soLuongDaCat)."
- kind: strategy
  body: "Cho dau: const ghim = new Set<number>(cacChiSoGhim); if (messages.length > 0 && messages[0]!.role === \"system\") ghim.add(0); const chiSoCoTheCat = messages.map((_, i) => i).filter((i) => !ghim.has(i)); let soLuongDaCat = 0; Cho hai: while (soLuongDaCat < chiSoCoTheCat.length && tinhTongToken(layKetQua(soLuongDaCat)) > nganSach) { soLuongDaCat += 1; } return layKetQua(soLuongDaCat);"
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
  expect: "3 48 true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
CÙNG ngân sách `50`, chỉ KHÁC Ở việc ghim đúng chỉ số chứa sự kiện —
`coConNhoSuKien` từ `false` (bài `3`) bật lên `true`. Cắt theo ưu tiên,
không cắt mù theo tuổi, LÀ cách sửa CHÍNH XÁC context rot. Nhưng cho
tới giờ, quest này vẫn nghĩ về "ghép các message" một cách thủ công —
bài sau đặt tên đúng cho phép TOÁN đó, qua lăng kính FP: Monoid.
::::

::::reflect{#nghi-lai}
`catTheoUuTien` không phát minh một cơ chế mới — nó CHỈ tổng quát hoá
đúng MỘT ý tưởng đã có Ở `catCuaSoTruot` (bài `2`): "một tập chỉ số
được bảo vệ khỏi bị cắt". Bài `2` cứng hoá tập đó thành `{0 nếu là
system}`; bài này biến nó thành MỘT THAM SỐ (`cacChiSoGhim`) — và chính
sự tổng quát hoá đơn giản đó đủ để sửa context rot MÀ KHÔNG cần tăng
ngân sách, không cần đổi chiến lược cắt "cũ nhất trước". Điều quyết
định KHÔNG phải LÀ thuật toán cắt — mà LÀ hệ thống có BIẾT trước message
nào đáng ghim hay không.
::::

::::checkpoint{mastery=0.83}
::::
