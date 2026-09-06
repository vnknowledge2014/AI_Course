---
id: ky-nghe-ung-dung-ai.ky-thuat-ngu-canh.tom-tat-thay-vi-xoa
title: "Tóm tắt thay vì xoá: giữ một phần thông tin khi cắt"
summary: "Mở đầu q9.2b (Kỹ nghệ hoá ngữ cảnh) — q9.2a XOÁ HẲN phần bị cắt (catCuaSoTruot), mất trắng không dấu vết. tomTatDoan(messages: ChatMessage[]): ChatMessage LÀ hàm RULE-BASED (không gọi LLM thật): tạo MỘT message role \"assistant\", content nối 20 ký tự ĐẦU của mỗi message gốc bằng \" | \", tiền tố \"[TOM TAT] \". catVaTomTat(messages, nganSach): ChatMessage[] dùng LẠI đúng ranh giới cắt của catCuaSoTruot (bài 2, q9.2a) nhưng THAY phần bị cắt bằng KẾT QUẢ tomTatDoan thay vì xoá hẳn. Trên LICH_SU_HOI_THOAI + ngân sách 50: catCuaSoTruot còn 3 message/49 token (xoá trắng); catVaTomTat còn 4 message/68 token — NHIỀU hơn (68>49, tóm tắt không miễn phí) nhưng ÍT hơn hẳn không cắt gì (103), và giữ được cụm \"Ma don hang cua toi\" mà catCuaSoTruot đã xoá sạch."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-ngu-canh
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.tom-tat-thay-vi-xoa]
requires: [kna.boss-ngan-sach-token-va-cua-so-ngu-canh]
concepts: [kna.tom-tat-thay-vi-xoa]
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
q9.2a đóng Ở `6/6` với một sự thật đã đo được rõ ràng: `catCuaSoTruot`
XOÁ HẲN phần bị cắt — mất trắng, không còn dấu vết nào của nó trong cửa
sổ. Một hệ thống production thật hiếm khi chấp nhận mất trắng như vậy:
thay vì xoá, nó TÓM TẮT — giữ lại một bản GIST ngắn gọn, tốn ít token
hơn bản gốc nhưng vẫn còn HƠN KHÔNG CÓ GÌ. q9.2b — "Kỹ nghệ hoá ngữ
cảnh" — bắt đầu đúng chỗ q9.2a dừng: CÙNG bài toán cắt bớt, KHÁC chiến
lược xử lý phần bị cắt.
::::

::::explain{#tom-tat-doan}
`tomTatDoan` KHÔNG gọi model thật, KHÔNG có mạng — nó LÀ một hàm
RULE-BASED tất định: với một danh sách message (phần SẼ bị cắt), lấy
**20 ký tự ĐẦU** của MỖI message gốc, nối lại bằng `" | "`, rồi bọc
trong MỘT message DUY NHẤT, `role: "assistant"`:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function tomTatDoan(messages: ChatMessage[]): ChatMessage {
  const noiDung = messages.map((tin) => tin.content.slice(0, 20)).join(" | ");
  return { role: "assistant", content: `[TOM TAT] ${noiDung}` };
}

console.log(tomTatDoan([
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
]));
```

```text title=readonly
{"role":"assistant","content":"[TOM TAT] Ma don hang cua toi  | Toi da ghi nhan ma d"}
```

`slice(0, 20)` LUÔN cắt ĐÚNG `20` ký tự đầu (kể cả khi message gốc dài
hơn nhiều) — kể cả dấu cách CUỐI của một slice cũng được giữ NGUYÊN
(message đầu kết thúc bằng `"...cua toi "` — có khoảng trắng — nên nối
với `" | "` ra HAI dấu cách liền nhau, không phải lỗi, chỉ LÀ hệ quả
tất định của việc cắt đúng vị trí ký tự thứ `20`). `tomTatDoan` KHÔNG
đọc `role` của message gốc, KHÔNG hiểu Ý NGHĨA nội dung — nó chỉ LÀ một
phép cắt chuỗi VÀ nối chuỗi, hoàn toàn tất định, dễ kiểm.
::::

::::example{#cat-va-tom-tat-so-voi-xoa-han}
`catVaTomTat` tái dùng NGUYÊN VĂN đúng vòng lặp tìm ranh giới cắt của
`catCuaSoTruot` (bài `2`, q9.2a) — CÙNG một `batDau` được tính giống
hệt — nhưng thay vì trả về PHẦN CÒN LẠI mà bỏ hẳn phần bị cắt, nó chèn
KẾT QUẢ của `tomTatDoan` áp lên đúng phần bị cắt đó, ngay VỊ TRÍ phần
đó từng đứng:

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

function tomTatDoan(messages: ChatMessage[]): ChatMessage {
  const noiDung = messages.map((tin) => tin.content.slice(0, 20)).join(" | ");
  return { role: "assistant", content: `[TOM TAT] ${noiDung}` };
}

function catVaTomTat(messages: ChatMessage[], nganSach: number): ChatMessage[] {
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
  const phanBiCat = phanCoTheCat.slice(0, batDau);
  const phanConLai = phanCoTheCat.slice(batDau);
  if (phanBiCat.length === 0) return [...phanGiuCoDinh, ...phanConLai];
  return [...phanGiuCoDinh, tomTatDoan(phanBiCat), ...phanConLai];
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];

const xoaHan = catCuaSoTruot(LICH_SU_HOI_THOAI, 50);
const tomTatKetQua = catVaTomTat(LICH_SU_HOI_THOAI, 50);
console.log("xoa han:", xoaHan.length, "message,", tinhTongToken(xoaHan), "token");
console.log("tom tat:", tomTatKetQua.length, "message,", tinhTongToken(tomTatKetQua), "token");
console.log(tomTatKetQua.map((m) => m.role));
console.log("con dau vet sau tom tat:", tomTatKetQua.some((m) => m.content.includes("Ma don hang cua toi")));
console.log("con dau vet sau xoa han:", xoaHan.some((m) => m.content.includes("Ma don hang cua toi")));
```

```text title=readonly
xoa han: 3 message, 49 token
tom tat: 4 message, 68 token
["system","assistant","assistant","user"]
con dau vet sau tom tat: true
con dau vet sau xoa han: false
```

`68 > 49` — tóm tắt tốn NHIỀU token hơn xoá hẳn, vì nó THÊM một message
mới (chứ không chỉ bớt đi) — không có "tóm tắt miễn phí". Nhưng
`68 < 103` (tổng gốc, KHÔNG cắt gì) — vẫn LÀ một khoản tiết kiệm thật.
Đổi lại khoản chênh lệch `19` token đó (`68 - 49`): cụm `"Ma don hang
cua toi"` — một phần của message đã bị cắt — CÒN sống sót trong bản tóm
tắt, trong khi `catCuaSoTruot` xoá sạch không còn dấu vết nào.
::::

::::predict{#doan-token-nhieu-hay-it commitOnce}
So `tinhTongToken(catVaTomTat(LICH_SU_HOI_THOAI, 50))` (`68`) VỚI
`tinhTongToken(catCuaSoTruot(LICH_SU_HOI_THOAI, 50))` (`49`, bài `2`
q9.2a) — cái nào LỚN hơn, VÀ vì sao?

:::opt{correct}
`catVaTomTat` LỚN hơn (`68 > 49`) — nó giữ lại MỘT message tóm tắt (nội
dung khác rỗng, `content.length > 0`) THAY VÌ xoá hẳn phần bị cắt;
message tóm tắt đó VẪN được `demTokenGiaLap` đếm như bất kỳ message nào
khác, không có ngoại lệ "token miễn phí" cho tóm tắt
:::
:::opt
Bằng nhau (`68` cũng như `49`) — tóm tắt CHỈ LÀ một nhãn mô tả, không
tính vào `tinhTongToken`
::why
Nhầm "tóm tắt LÀ một khái niệm trừu tượng, không phải dữ liệu thật" VỚI
cách nó THẬT SỰ được biểu diễn trong `catVaTomTat` — nhưng `tomTatDoan`
trả về một `ChatMessage` BÌNH THƯỜNG, `content` của nó VẪN đi qua
`demTokenGiaLap` y hệt content của bất kỳ message nào khác.

Chỗ lệch: `68` VÀ `49` khác nhau ĐÚNG BẰNG số token của message tóm tắt
mới được chèn thêm — không có bước nào trong `catVaTomTat` khấu trừ
token của nó.
::
:::
:::opt
`catVaTomTat` NHỎ hơn — tóm tắt cắt MỌI message xuống còn `20` ký tự
nên tổng luôn nhỏ hơn xoá hẳn
::why
Gần đúng Ở việc bạn để Ý MỖI message gốc bị RÚT NGẮN rất nhiều (`20`
ký tự) — quan sát đó đúng, nhưng chỉ đúng cho phần message BỊ CẮT.

Chỗ lệch: `catVaTomTat` KHÔNG xoá các message CÒN LẠI (`phanConLai`,
chưa từng bị cắt) — nó chỉ THAY phần bị cắt bằng MỘT message tóm tắt
MỚI, cộng thêm vào những message vẫn giữ nguyên; kết quả LÀ `catVaTomTat`
có NHIỀU message hơn (`4` so với `3`), không ít hơn — VÀ tổng token vì
vậy cũng nhiều hơn, không ít hơn.
::
:::
::::

::::code{#viet_tom_tat_va_cat_va_tom_tat}
Hoàn thiện `tomTatDoan` — nối `20` ký tự đầu của mỗi message bằng
`" | "`, bọc trong message `role: "assistant"`, tiền tố `"[TOM TAT] "`.
Hoàn thiện phần CUỐI của `catVaTomTat` (dùng `batDau` đã tính sẵn Ở
trên) — tách `phanBiCat`/`phanConLai`, TRẢ VỀ nguyên trạng nếu không có
gì bị cắt, ngược lại chèn `tomTatDoan(phanBiCat)` vào đúng vị trí.

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

function tomTatDoan(messages: ChatMessage[]): ChatMessage {
  ___
}

function catVaTomTat(messages: ChatMessage[], nganSach: number): ChatMessage[] {
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

const ketQuaTomTat = catVaTomTat(LICH_SU_HOI_THOAI, 50);
console.log(ketQuaTomTat.length, tinhTongToken(ketQuaTomTat), ketQuaTomTat[0]!.role);
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

function tomTatDoan(messages: ChatMessage[]): ChatMessage {
  const noiDung = messages.map((tin) => tin.content.slice(0, 20)).join(" | ");
  return { role: "assistant", content: `[TOM TAT] ${noiDung}` };
}

function catVaTomTat(messages: ChatMessage[], nganSach: number): ChatMessage[] {
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
  const phanBiCat = phanCoTheCat.slice(0, batDau);
  const phanConLai = phanCoTheCat.slice(batDau);
  if (phanBiCat.length === 0) return [...phanGiuCoDinh, ...phanConLai];
  return [...phanGiuCoDinh, tomTatDoan(phanBiCat), ...phanConLai];
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];

const ketQuaTomTat = catVaTomTat(LICH_SU_HOI_THOAI, 50);
console.log(ketQuaTomTat.length, tinhTongToken(ketQuaTomTat), ketQuaTomTat[0]!.role);
```

```typescript title=test
if (ketQuaTomTat.length !== 4) throw new Error("phai con lai dung 4 message (system + 1 tom tat + 2 message goc con lai)");
if (JSON.stringify(ketQuaTomTat.map((m) => m.role)) !== JSON.stringify(["system", "assistant", "assistant", "user"])) {
  throw new Error("thu tu role phai la system, tom tat (assistant), roi hai message goc con lai");
}
if (ketQuaTomTat[1]!.content !== "[TOM TAT] Ma don hang cua toi  | Toi da ghi nhan ma d | Trong luc cho, toi m") {
  throw new Error("noi dung tom tat phai la 20 ky tu dau cua moi message bi cat, noi bang ' | ', tien to '[TOM TAT] '");
}
if (tinhTongToken(ketQuaTomTat) !== 68) throw new Error("tong token sau catVaTomTat phai la 68");

const catCuaSoTruotLai = (messages: ChatMessage[], nganSach: number): ChatMessage[] => {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; }
  return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];
};
const xoaHanKetQua = catCuaSoTruotLai(LICH_SU_HOI_THOAI, 50);
if (tinhTongToken(xoaHanKetQua) !== 49) throw new Error("doi chieu: xoa han (khong tom tat) phai con dung 49 token");
if (tinhTongToken(ketQuaTomTat) <= tinhTongToken(xoaHanKetQua)) {
  throw new Error("catVaTomTat PHAI ton nhieu token hon catCuaSoTruot -- tom tat khong mien phi");
}
if (tinhTongToken(ketQuaTomTat) >= tinhTongToken(LICH_SU_HOI_THOAI)) {
  throw new Error("catVaTomTat van phai it token hon KHONG cat gi ca (103)");
}

if (!ketQuaTomTat.some((m) => m.content.includes("Ma don hang cua toi"))) {
  throw new Error("tom tat phai giu lai dau vet cua message da bi cat");
}
if (xoaHanKetQua.some((m) => m.content.includes("Ma don hang cua toi"))) {
  throw new Error("xoa han thi KHONG duoc con dau vet nao cua message da bi cat");
}

const khongCanCat = catVaTomTat(LICH_SU_HOI_THOAI, 1000);
if (khongCanCat.length !== 6 || tinhTongToken(khongCanCat) !== 103) {
  throw new Error("ngan sach du lon thi KHONG duoc goi tomTatDoan, phai giu nguyen tat ca 6 message goc");
}
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (tomTatDoan): noiDung = messages.map lay 20 ky tu dau moi tin.content bang slice(0, 20), noi bang join(\" | \"); return mot ChatMessage { role: \"assistant\", content: mot chuoi bat dau bang \"[TOM TAT] \" roi noiDung }. Cho hai (cuoi catVaTomTat, dung bien batDau da tinh san): tach phanBiCat = phanCoTheCat.slice(0, batDau), phanConLai = phanCoTheCat.slice(batDau); neu phanBiCat rong thi return phanGiuCoDinh noi phanConLai (khong tom tat gi); nguoc lai return phanGiuCoDinh, tomTatDoan(phanBiCat), roi phanConLai."
- kind: strategy
  body: "Cho dau: const noiDung = messages.map((tin) => tin.content.slice(0, 20)).join(\" | \"); return { role: \"assistant\", content: `[TOM TAT] ${noiDung}` }; Cho hai: const phanBiCat = phanCoTheCat.slice(0, batDau); const phanConLai = phanCoTheCat.slice(batDau); if (phanBiCat.length === 0) return [...phanGiuCoDinh, ...phanConLai]; return [...phanGiuCoDinh, tomTatDoan(phanBiCat), ...phanConLai];"
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
  expect: "4 68 system"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`68` so với `49` — tóm tắt tốn token thật, nhưng đổi lại giữ được một
dấu vết mà xoá hẳn không giữ được gì cả. Nhưng cho tới giờ, quest này
vẫn dùng MỘT ngân sách DUY NHẤT cho toàn bộ cửa sổ. Một hệ thống thật
thường tách RIÊNG ngân sách theo TỪNG khu vực chức năng — chỉ thị hệ
thống, các bản tóm tắt, VÀ các message gần đây — bài sau làm đúng việc
đó.
::::

::::reflect{#nghi-lai}
`tomTatDoan` không "hiểu" nội dung theo bất kỳ nghĩa nào — nó chỉ LÀ
một phép cắt chuỗi tất định (`20` ký tự đầu mỗi message, nối bằng
`" | "`). Sức mạnh của bài này không nằm Ở ĐỘ THÔNG MINH của bản tóm
tắt — mà Ở việc THAY ĐỔI CÁCH XỬ LÝ phần bị cắt: `catCuaSoTruot` (bài
`2`, q9.2a) coi phần bị cắt LÀ RÁC, bỏ đi hoàn toàn; `catVaTomTat` coi
nó LÀ THÔNG TIN CÓ GIÁ TRỊ THẤP HƠN nhưng KHÔNG BẰNG KHÔNG — đáng để giữ
lại một bản nén, dù phải trả thêm một ít token. Đây LÀ đúng đánh đổi mà
một hệ thống production PHẢI cân nhắc: xoá HAY nén, không phải xoá LÀ
LỰA CHỌN DUY NHẤT khi ngân sách hữu hạn.
::::

::::checkpoint{mastery=0.78}
::::
