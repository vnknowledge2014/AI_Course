---
id: ky-nghe-ung-dung-ai.ky-thuat-ngu-canh.dem-token-gia-lap-va-vuot-ngan-sach
title: "Đếm token giả lập & phát hiện vượt ngân sách"
summary: "Mở đầu T9.2 (Kỹ thuật Ngữ cảnh) — chuyển trục từ MỘT lượt prompt (T9.1) sang TOÀN BỘ cửa sổ token xuyên nhiều lượt. demTokenGiaLap(text: string): number = Math.ceil(text.length / 4), một xấp xỉ TẤT ĐỊNH (không phải tokenizer thật — quy tắc phổ biến ~4 ký tự/token). tinhTongToken(messages: ChatMessage[]): number cộng dồn token của MỌI message, không phân biệt role. vuotNganSach(messages, nganSach): boolean = tổng > nganSach (bằng nhau KHÔNG tính là vượt). Trên một lịch sử hội thoại cụ thể (6 message, tổng 103 token): vuot(90)=true, vuot(110)=false, vuot(103, đúng bằng tổng)=false, vuot(102)=true — ranh giới đo được chính xác từng token một."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-ngu-canh
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [kna.dem-token-gia-lap-va-vuot-ngan-sach]
requires: [kna.boss-ky-nghe-hoa-prompt]
concepts: [kna.dem-token-gia-lap-va-vuot-ngan-sach]
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
Track trước (T9.1) đo MỘT lượt: một prompt, một câu trả lời, một
`pass@1`. Track này — Kỹ thuật Ngữ cảnh — đo một thứ khác hẳn: TOÀN BỘ
cửa sổ hội thoại, xuyên SUỐT nhiều lượt, khi nó lớn dần và có thể vượt
quá giới hạn mà model cho phép. Trước khi cắt bớt bất cứ thứ gì, phải
đếm được: cửa sổ hiện tại nặng bao nhiêu?
::::

::::explain{#dem-token-gia-lap}
Model thật không đọc ký tự — nó đọc TOKEN, và một token thật KHÔNG
bằng một ký tự (tuỳ ngôn ngữ, tuỳ tokenizer, một từ có thể LÀ một
token hoặc vài mảnh token). Quest này KHÔNG gọi tokenizer thật, KHÔNG
gọi model thật, KHÔNG có mạng — `demTokenGiaLap` LÀ một xấp xỉ TẤT ĐỊNH
theo quy tắc phổ biến trong ngành: trung bình khoảng **4 ký tự cho một
token**. Đây LÀ một ước lượng thô, KHÔNG phải con số một tokenizer BPE
thật sẽ trả về — nhưng nó đủ để dạy đúng VẤN ĐỀ (ngân sách hữu hạn, cửa
sổ có thể tràn) mà không cần hạ tầng tokenizer thật:

```typescript title=readonly
function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}

console.log(demTokenGiaLap(""), demTokenGiaLap("A"), demTokenGiaLap("Xin chao"), demTokenGiaLap("Xin chao ban"));
```

```text title=readonly
0 1 2 3
```

Chuỗi rỗng LÀ `0` token. MỘT ký tự LÀ `1` token — `Math.ceil` LUÔN làm
tròn LÊN, một phần token cũng tính LÀ một token trọn vẹn (không có
"nửa token"). `"Xin chao"` (8 ký tự) chia hết cho 4, ra ĐÚNG `2`.
`"Xin chao ban"` (12 ký tự) ra ĐÚNG `3`. `tinhTongToken` cộng dồn token
của MỌI message trong một cửa sổ — KHÔNG phân biệt `role`: một message
`system` dài cũng ngốn ngân sách y hệt một message `user`:

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

function vuotNganSach(messages: ChatMessage[], nganSach: number): boolean {
  return tinhTongToken(messages) > nganSach;
}
```

`vuotNganSach` so sánh bằng `>` — TUYỆT ĐỐI, không phải `>=`. Một cửa
sổ có tổng token ĐÚNG BẰNG ngân sách LÀ vừa đủ, không phải vượt.
::::

::::example{#do-tren-lich-su-cu-the}
Một lịch sử hội thoại cụ thể — sáu message, độ dài khác nhau, một
message chứa mã đơn hàng `"XY789"` (sẽ quay lại Ở bài `3`) — đo tổng
token VÀ kiểm `vuotNganSach` Ở nhiều mức ngân sách khác nhau:

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

function vuotNganSach(messages: ChatMessage[], nganSach: number): boolean {
  return tinhTongToken(messages) > nganSach;
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];

console.log("tong token:", tinhTongToken(LICH_SU_HOI_THOAI));
console.log("vuot 90:", vuotNganSach(LICH_SU_HOI_THOAI, 90));
console.log("vuot 110:", vuotNganSach(LICH_SU_HOI_THOAI, 110));
console.log("vuot dung bang tong (103):", vuotNganSach(LICH_SU_HOI_THOAI, 103));
```

```text title=readonly
tong token: 103
vuot 90: true
vuot 110: false
vuot dung bang tong (103): false
```

Tổng token CỦA CẢ SÁU message LÀ `103`. Ngân sách `90` (nhỏ hơn tổng)
LÀM cửa sổ VƯỢT — `true`. Ngân sách `110` (lớn hơn tổng) KHÔNG vượt —
`false`. VÀ ngân sách ĐÚNG BẰNG tổng thật (`103`) CŨNG KHÔNG vượt —
`false` — vì `vuotNganSach` dùng `>`, không phải `>=`. KHÔNG PHẢI mọi
ngân sách đều vượt, và KHÔNG PHẢI mọi ngân sách đều an toàn — ranh giới
nằm CHÍNH XÁC giữa `102` và `103` cho lịch sử cụ thể này.
::::

::::predict{#doan-nganh-sach-dung-bang-tong commitOnce}
Vẫn LÀ `LICH_SU_HOI_THOAI` (tổng `103` token). Gọi
`vuotNganSach(LICH_SU_HOI_THOAI, 102)` — ngân sách NHỎ HƠN tổng đúng
MỘT token. Kết quả LÀ gì?

:::opt{correct}
`true` — `tinhTongToken` trả về `103`, VÀ `103 > 102` LÀ `true`; chỉ
lệch MỘT token cũng đủ để `vuotNganSach` trả về `true`, không cần lệch
nhiều
:::
:::opt
`false` — chênh lệch chỉ MỘT token LÀ quá nhỏ để tính LÀ "vượt", cửa sổ
gần như vừa đủ nên vẫn nên coi LÀ an toàn
::why
Nhầm "gần đủ" (một nhận xét về MỨC ĐỘ) VỚI cách `vuotNganSach` THẬT SỰ
quyết định — nhưng hàm này KHÔNG có khái niệm "gần đủ", nó chỉ có ĐÚNG
MỘT phép so sánh: `tinhTongToken(messages) > nganSach`.

Chỗ lệch: `103 > 102` LÀ `true` bằng đúng phép toán JavaScript thông
thường, không có ngoại lệ nào cho chênh lệch nhỏ. Một hệ thống thật
CŨNG hoạt động y hệt vậy — cửa sổ vượt quá giới hạn dù chỉ MỘT token
vẫn LÀ vượt, và phải xử lý (cắt bớt, bài sau) giống hệt như vượt nhiều.
::
:::
:::opt
Hàm ném lỗi, vì `102` không chia hết cho `4` như các ngân sách khác đã
dùng
::why
Gần đúng Ở việc bạn để Ý `102` LÀ một con số "lẻ" so với `90`/`110` —
quan sát đó đúng nhưng không liên quan.

Chỗ lệch: `vuotNganSach` không hề kiểm tra `nganSach` có chia hết cho
`4` hay không — `nganSach` LÀ một `number` bất kỳ do người gọi truyền
vào, không bị ràng buộc gì thêm. Hàm chỉ so sánh, không bao giờ `throw`.
::
:::
::::

::::code{#viet_dem_token_va_tong}
Hoàn thiện `demTokenGiaLap` — trả về `Math.ceil(text.length / 4)`.
Hoàn thiện `tinhTongToken` — cộng dồn token của MỌI message bằng
`reduce`, tái dùng `demTokenGiaLap`.

```typescript title=starter
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function demTokenGiaLap(text: string): number {
  ___
}

function tinhTongToken(messages: ChatMessage[]): number {
  ___
}

function vuotNganSach(messages: ChatMessage[], nganSach: number): boolean {
  return tinhTongToken(messages) > nganSach;
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];

console.log(tinhTongToken(LICH_SU_HOI_THOAI), vuotNganSach(LICH_SU_HOI_THOAI, 90), vuotNganSach(LICH_SU_HOI_THOAI, 110));
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

function vuotNganSach(messages: ChatMessage[], nganSach: number): boolean {
  return tinhTongToken(messages) > nganSach;
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];

console.log(tinhTongToken(LICH_SU_HOI_THOAI), vuotNganSach(LICH_SU_HOI_THOAI, 90), vuotNganSach(LICH_SU_HOI_THOAI, 110));
```

```typescript title=test
if (demTokenGiaLap("") !== 0) throw new Error("chuoi rong phai la 0 token");
if (demTokenGiaLap("A") !== 1) throw new Error("1 ky tu van lam tron len thanh 1 token (Math.ceil)");
if (demTokenGiaLap("Xin chao") !== 2) throw new Error("8 ky tu phai la 2 token (8/4=2)");
if (demTokenGiaLap("Xin chao ban") !== 3) throw new Error("12 ky tu phai la 3 token (12/4=3)");

if (tinhTongToken(LICH_SU_HOI_THOAI) !== 103) throw new Error("tong token ca 6 message phai la 103");
if (tinhTongToken([]) !== 0) throw new Error("mang rong phai co tong token la 0");

if (vuotNganSach(LICH_SU_HOI_THOAI, 90) !== true) throw new Error("ngan sach 90 nho hon tong 103, phai vuot");
if (vuotNganSach(LICH_SU_HOI_THOAI, 110) !== false) throw new Error("ngan sach 110 lon hon tong 103, khong vuot");
if (vuotNganSach(LICH_SU_HOI_THOAI, 103) !== false) throw new Error("ngan sach BANG tong thi KHONG duoc tinh la vuot");
if (vuotNganSach(LICH_SU_HOI_THOAI, 102) !== true) throw new Error("ngan sach it hon tong dung 1 token van phai vuot");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (demTokenGiaLap): tra ve Math.ceil(text.length / 4) -- lam tron LEN, khong lam tron xuong. Cho hai (tinhTongToken): dung messages.reduce, moi buoc cong don demTokenGiaLap(tin.content) vao tong, gia tri khoi dau la 0."
- kind: strategy
  body: "Cho dau: return Math.ceil(text.length / 4); Cho hai: return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);"
- kind: one-line
  body: "Sao chep dung hai dong o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "103 true false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`103` token, và ranh giới vượt ngân sách đo được chính xác tới TỪNG
token một (`102` vượt, `103` thì không). Nhưng biết CÓ vượt ngân sách
mới LÀ nửa đầu câu chuyện — nửa sau LÀ: khi vượt, phải cắt bớt CÁI GÌ?
Bài sau bắt đầu với chiến lược đơn giản nhất: cắt CŨ NHẤT trước.
::::

::::reflect{#nghi-lai}
`demTokenGiaLap` không hề giả vờ LÀ một tokenizer thật — nó LÀ một xấp
xỉ tất định, khai báo THẲNG THẮN quy tắc của nó (`length / 4`, làm tròn
lên). Điều quan trọng hơn con số CHÍNH XÁC LÀ khái niệm: một cửa sổ hội
thoại có NGÂN SÁCH hữu hạn, tính bằng TOKEN chứ không phải bằng SỐ
MESSAGE hay ký tự, và ngân sách đó có thể bị vượt Ở một điểm CỤ THỂ,
đo được bằng số — không phải một cảm giác "cửa sổ có vẻ dài". Toàn bộ
track này, từ đây tới BOSS, xây trên đúng hai hàm `tinhTongToken` và
`vuotNganSach` vừa viết.
::::

::::checkpoint{mastery=0.75}
::::
